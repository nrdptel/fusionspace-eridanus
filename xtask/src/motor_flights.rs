//! Every bundled motor listed by `hpr motors list` and flown by `hpr sim --motor`, offline (M5.7b2,
//! ADR-302 §6).
//!
//! The command line runs in-process, offline, with an empty weather cache. `hpr motors list
//! --json` must list each solid motor of the snapshot, by a selector that names it alone. Each
//! motor with a curve file then flies in a probe rocket built for its case size: a light
//! airframe a fifth wider than the motor, a motor tube as long as the longest motor of that
//! size, three fins. A flight counts as flown when it leaves the rail and reaches an apogee with
//! the bundled curve of that very motor, and when it warns of a default file that fails ADR-005's
//! 1% check and only then. The probe is no one's rocket: the check is that each motor flies,
//! not how high.
//!
//! The expected counts come from the raw snapshot under `refs/` where it is there: every solid
//! motor of its search answer, and every one with a RASP or RockSim file in its download answers
//! that isn't left out. Without it they come from the committed records and file list, which the
//! raw comparison holds equal to it.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use hpr::rocket::{Fins, MotorTube, Nose, Tube, material};
use hpr::{FinPlanform, Motor, NoseShape, Rocket};
use hpr_motor::Catalog;
use serde_json::Value;

/// How much wider than the motor the probe's bore is, in meters: within a real case's slack, so
/// the design checks take every motor of that size.
const BORE_SLACK_M: f64 = 0.0002;

/// What the snapshot says should be listed and flown.
pub struct Expected {
    /// Every solid motor's id.
    pub listed: BTreeSet<String>,
    /// Every solid motor with a file that is bundled.
    pub flown: BTreeSet<String>,
    /// Where the counts came from.
    pub source: &'static str,
}

/// Lists and flies every motor, failing on the first count or flight that is wrong.
pub fn check(catalog: &Catalog, expected: &Expected) -> Result<(), String> {
    let scratch = std::env::temp_dir().join(format!("hpr-motor-flights-{}", std::process::id()));
    let cache = scratch.join("cache");
    fs::create_dir_all(&cache).map_err(|error| format!("{}: {error}", cache.display()))?;
    let result = list_and_fly(catalog, expected, &scratch, &cache);
    let _ = fs::remove_dir_all(&scratch);
    result
}

fn list_and_fly(
    catalog: &Catalog,
    expected: &Expected,
    scratch: &Path,
    cache: &Path,
) -> Result<(), String> {
    let listed = hpr_json(&["motors", "list", "--json"], cache)?;
    let rows = listed["motors"]
        .as_array()
        .ok_or("`hpr motors list --json` printed no motors")?;
    let mut by_selector = BTreeMap::new();
    for (row, motor) in rows.iter().zip(&catalog.motors) {
        let selector = row["selector"].as_str().unwrap_or_default();
        if row["designation"].as_str() != Some(motor.designation.as_str())
            || catalog
                .find(selector)
                .map(|found| &found.motor_id)
                .collect::<Vec<_>>()
                != [&motor.motor_id]
        {
            return Err(format!(
                "`hpr motors list` lists {row} where the catalog has {} {}",
                motor.manufacturer_abbrev, motor.designation
            ));
        }
        if row["curve"].as_bool() != Some(motor.default_curve().is_some()) {
            return Err(format!("`hpr motors list` marks {selector}'s curve wrongly"));
        }
        by_selector.insert(motor.motor_id.clone(), selector.to_owned());
    }
    let listed_ids: BTreeSet<String> = by_selector.keys().cloned().collect();
    if rows.len() != catalog.motors.len() || listed_ids != expected.listed {
        return Err(format!(
            "`hpr motors list` lists {} motors, the {} {}",
            rows.len(),
            expected.listed.len(),
            expected.source
        ));
    }

    // The motors with a curve, by case diameter: one probe rocket for each size.
    let mut sizes: BTreeMap<u64, Vec<&hpr_motor::catalog::CatalogMotor>> = BTreeMap::new();
    for motor in catalog.motors.iter().filter(|m| m.default_curve().is_some()) {
        let micrometers = (motor.diameter_mm * 1e3).round();
        // A case size is a positive number of micrometers, well within a u64.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        sizes.entry(micrometers as u64).or_default().push(motor);
    }
    let flown_ids: BTreeSet<String> = sizes
        .values()
        .flatten()
        .map(|m| m.motor_id.clone())
        .collect();
    if flown_ids != expected.flown {
        return Err(format!(
            "{} motors have a bundled curve, the {} {}",
            flown_ids.len(),
            expected.flown.len(),
            expected.source
        ));
    }
    let mut runs = Vec::new();
    for (micrometers, motors) in &sizes {
        let design = probe(scratch, *micrometers, motors, &by_selector)?;
        for motor in motors {
            runs.push((design.clone(), *motor, by_selector[&motor.motor_id].clone()));
        }
    }
    let workers = thread::available_parallelism().map_or(4, usize::from);
    let chunk = runs.len().div_ceil(workers).max(1);
    let failures: Vec<String> = thread::scope(|scope| {
        let handles: Vec<_> = runs
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .filter_map(|(design, motor, selector)| {
                            flies(design, motor, selector, cache).err()
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_else(|_| vec!["a flight panicked".into()]))
            .collect()
    });
    if let Some(first) = failures.first() {
        return Err(format!(
            "{} of {} motors don't fly; the first: {first}",
            failures.len(),
            runs.len()
        ));
    }
    println!(
        "`hpr motors list` lists all {} motors, and `hpr sim --motor` flies each of the {} with a \
         curve, in {} probe rockets, offline; the counts are the {}",
        rows.len(),
        runs.len(),
        sizes.len(),
        expected.source
    );
    Ok(())
}

/// Writes the probe rocket for motors `micrometers` across, as a rocket's JSON, flying the first.
fn probe(
    scratch: &Path,
    micrometers: u64,
    motors: &[&hpr_motor::catalog::CatalogMotor],
    selectors: &BTreeMap<String, String>,
) -> Result<PathBuf, String> {
    let fail = |error: hpr::Error| format!("the {micrometers} µm probe: {error}");
    #[allow(clippy::cast_precision_loss)]
    let bore_m = micrometers as f64 * 1e-6 + BORE_SLACK_M;
    let longest_m = motors
        .iter()
        .map(|m| m.length_mm * 1e-3)
        .fold(0.0, f64::max);
    let wall_m = (0.02 * bore_m).max(0.0005);
    let diameter_m = 1.2 * bore_m + 2.0 * wall_m;
    let first = motors.first().ok_or("an empty size")?;
    let motor = Motor::from_catalog(&selectors[&first.motor_id]).map_err(fail)?;
    let paper = material("kraft_phenolic").map_err(fail)?;
    let mut rocket = Rocket::new(&format!("probe {micrometers} µm"), diameter_m).map_err(fail)?;
    rocket
        .add_nose(Nose::hollow(
            NoseShape::Ogive { radius_ratio: 1.0 },
            4.0 * diameter_m,
            wall_m,
            material("abs").map_err(fail)?,
        ))
        .map_err(fail)?
        .add_tube(Tube::new(longest_m + 4.0 * diameter_m, wall_m, paper.clone()))
        .map_err(fail)?
        .add_motor_tube(MotorTube::new(longest_m, bore_m, wall_m, paper))
        .map_err(fail)?
        .add_fins(Fins::new(
            3,
            FinPlanform::Trapezoidal {
                root_chord_m: 2.0 * diameter_m,
                tip_chord_m: diameter_m,
                span_m: 1.5 * diameter_m,
                sweep_m: diameter_m,
            },
            (0.05 * diameter_m).max(0.0008),
            material("birch_plywood").map_err(fail)?,
        ))
        .map_err(fail)?
        .set_motor(motor)
        .map_err(fail)?;
    let json = serde_json::to_string(rocket.design()).map_err(|error| error.to_string())?;
    let path = scratch.join(format!("probe-{micrometers}.json"));
    fs::write(&path, json).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(path)
}

/// Flies `motor` in `design` with `hpr sim --motor <selector>`.
fn flies(
    design: &Path,
    motor: &hpr_motor::catalog::CatalogMotor,
    selector: &str,
    cache: &Path,
) -> Result<(), String> {
    let path = design.to_string_lossy();
    let flight = hpr_json(&["sim", &path, "--motor", selector, "--json"], cache)
        .map_err(|error| format!("{selector}: {error}"))?;
    let summary = &flight["summary"];
    if summary["rail_exit_speed_m_s"].is_null() || summary["apogee"].is_null() {
        return Err(format!(
            "{selector} doesn't leave the rail and reach an apogee: {summary}"
        ));
    }
    let flown = &flight["motors"][0];
    if flown["designation"].as_str() != Some(motor.designation.as_str())
        || flown["source"]["kind"] != "catalog"
    {
        return Err(format!("{selector} flies {flown}"));
    }
    let fails = motor.default_curve().is_some_and(|curve| !curve.check.passes);
    let warned = flight["warnings"].as_array().is_some_and(|warnings| {
        warnings.iter().any(|warning| {
            warning.to_string().contains("1%")
                && warning.to_string().contains(motor.designation.as_str())
        })
    });
    if fails != warned {
        return Err(format!(
            "{selector}'s default file {} the 1% check, but the flight {} of it",
            if fails { "fails" } else { "passes" },
            if warned { "warns" } else { "doesn't warn" }
        ));
    }
    Ok(())
}

/// Runs `hpr <args>` in-process and offline, and reads the JSON it prints.
fn hpr_json(args: &[&str], cache: &Path) -> Result<Value, String> {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = hpr_cli::run_offline(
        std::iter::once("hpr").chain(args.iter().copied()),
        &mut out,
        &mut err,
        cache,
    );
    let value: Value = serde_json::from_slice(&out).map_err(|error| {
        format!(
            "`hpr {}` printed no JSON ({error}): {}",
            args.join(" "),
            String::from_utf8_lossy(&err)
        )
    })?;
    if exit != hpr_cli::Exit::Success || !value["error"].is_null() {
        return Err(format!("`hpr {}`: {}", args.join(" "), value["error"]));
    }
    Ok(value)
}
