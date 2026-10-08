//! Which of ThrustCurve.org's data files holds the curve OpenRocket flies, for each motor of
//! OpenRocket's examples (M4.5g2, ADR-161): the table `hpr sim` reads to fetch OpenRocket's own
//! curve for a `.ork` motor's digest, where ThrustCurve holds several files of the motor.
//!
//! For each motor a configuration of the pinned jar's examples places, the command reads
//! OpenRocket's curve for its digest from OpenRocket's database (`corpus-out/openrocket-motors.json`,
//! ADR-067), finds the motor on ThrustCurve as `hpr sim` does (by manufacturer and designation,
//! [`hpr::hpr_net::on_demand`]), and compares the curve of every file ThrustCurve holds of it with
//! OpenRocket's: the largest difference in thrust at any sample time of either curve, over
//! OpenRocket's peak thrust. A file matches when that is under [`MATCH`]; of the matches, the one
//! with the least difference is written, and of files equal in thrust, the one whose masses (the
//! propellant's and the dry motor's, as hpr builds the motor from the file) are nearest
//! OpenRocket's. A motor with no match is listed with why.
//!
//! [`TABLE`] holds ids, names and that difference only, no curve: ThrustCurve states no license
//! for its records and OpenRocket's database states no terms (ADR-145), and the examples are
//! public, so no private design's motor is named.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use hpr::hpr_net::on_demand::{self, FindError, Wanted};
use hpr::hpr_net::thrustcurve::Format;
use hpr::hpr_net::{Cache, Client, Http, Mode};
use hpr_io::ork;
use hpr_motor::{SolidMotor, ThrustCurve};
use serde_json::{Value, json};

/// The table, as `hpr sim` reads it.
pub(crate) const TABLE: &str = "crates/hpr-cli/data/openrocket-curves.json";

/// The jar whose examples' motors are matched.
const JAR: &str = "refs/openrocket/OpenRocket-24.12.jar";

/// A file matches OpenRocket's curve when no thrust differs by more than this share of
/// OpenRocket's peak thrust. On 2026-10-04 every file was within 4 parts in a million of
/// OpenRocket's curve or 2.7% of the peak or more from it (the table lists each motor's
/// differences), so the bound falls between with room on both sides.
const MATCH: f64 = 0.01;

pub const USAGE: &str = "\
  ork-curves [--offline]   Match each motor of OpenRocket's examples to the ThrustCurve.org
                           file holding OpenRocket's curve, and write
                           crates/hpr-cli/data/openrocket-curves.json, the table hpr sim
                           fetches OpenRocket's curve by. Needs the pinned jar,
                           corpus-out/openrocket-motors.json and the network, or with
                           --offline the hpr cache alone.";

/// Runs `cargo xtask ork-curves`.
pub fn run(args: &[String]) -> Result<(), String> {
    let offline = match args {
        [] => false,
        [flag] if flag == "--offline" => true,
        _ => return Err(format!("ork-curves takes only --offline\n\n{USAGE}")),
    };
    let root = crate::ork::root()?;
    let table = match_all(&root, offline)?;
    let text = serde_json::to_string_pretty(&table).map_err(|error| error.to_string())? + "\n";
    let path = root.join(TABLE);
    fs::write(&path, text).map_err(|error| format!("{}: {error}", path.display()))?;
    let others: Vec<f64> = table["files"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|file| file["others_differences"].as_array().into_iter().flatten())
        .filter_map(Value::as_f64)
        .collect();
    let within = others.iter().filter(|&&d| d < MATCH).count();
    let by_mass = table["files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|file| file["decided_by_mass"] == true)
        .count();
    let nearest = others
        .iter()
        .copied()
        .filter(|&d| d >= MATCH)
        .fold(f64::INFINITY, f64::min);
    println!(
        "wrote {TABLE}: {} motors matched, {} not; of the matched motors' {} other files, {} \
         within {MATCH} (the largest {}), {} beyond it (the nearest {nearest}, {} of them 0.34 \
         or more); {by_mass} picked by mass among files alike in thrust",
        table["files"].as_array().map_or(0, Vec::len),
        table["unmatched"].as_array().map_or(0, Vec::len),
        others.len(),
        within,
        others
            .iter()
            .copied()
            .filter(|&d| d < MATCH)
            .fold(0.0, f64::max),
        others.len() - within,
        others.iter().filter(|&&d| d >= 0.34).count()
    );
    Ok(())
}

/// A motor the examples place: its maker and designation as the `.ork` records them.
struct Placed {
    manufacturer: String,
    designation: String,
}

fn match_all(root: &Path, offline: bool) -> Result<Value, String> {
    let jar = root.join(JAR);
    if !jar.is_file() {
        return Err(format!(
            "{JAR} is missing: fetch it with `cargo xtask refs fetch`"
        ));
    }
    let supply = crate::ork_supply::Supply::load(root, true)?;
    if !supply.is_present() {
        return Err(format!(
            "{} is missing: run validation/oracles/openrocket/motor_database.py (ADR-067)",
            crate::ork_supply::RECORD
        ));
    }
    let mut placed = BTreeMap::new();
    for (name, bytes) in crate::ork::examples_in_jar(&jar)? {
        let read = ork::read(&bytes).map_err(|error| format!("{name}: {error}"))?;
        let design = ork::design_with(&read.value, supply.curves()).value;
        for configuration in &design.motors.configurations {
            for motor in &configuration.motors {
                if let Some(digest) = &motor.digest {
                    placed.entry(digest.clone()).or_insert_with(|| Placed {
                        manufacturer: motor.manufacturer.clone(),
                        designation: motor.designation.clone(),
                    });
                }
            }
        }
    }
    let dir = Cache::platform_dir().ok_or("no cache folder: set HPR_CACHE_DIR")?;
    let mode = if offline { Mode::Offline } else { Mode::Online };
    let client = Client::new(Http::new(), Cache::new(dir), mode);
    let now_s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let mut files = Vec::new();
    let mut unmatched = Vec::new();
    for (digest, motor) in &placed {
        let named = json!({
            "digest": digest,
            "manufacturer": motor.manufacturer,
            "designation": motor.designation,
        });
        let unmatched_why = |why: String| {
            let mut entry = named.clone();
            entry["why"] = json!(why);
            entry
        };
        let Some(openrocket) = supply.curves().get(digest) else {
            unmatched.push(unmatched_why(
                "OpenRocket's database has no solid curve for the digest".to_owned(),
            ));
            continue;
        };
        let candidates = RefCell::new(Vec::new());
        let wanted = Wanted::by(&motor.manufacturer, &motor.designation);
        let found = on_demand::find_with(&client, &wanted, now_s, |file| {
            on_demand::reads_as_one_motor(file)?;
            let text = file.text().map_err(|error| error.to_string())?;
            let built = match file.format {
                Format::Rasp => hpr::Motor::from_eng(&text),
                _ => hpr::Motor::from_rse(&text),
            }
            .map_err(|error| error.to_string())?;
            candidates.borrow_mut().push((
                file.simfile_id.clone(),
                file.format,
                built.solid_motor().clone(),
            ));
            // Refused, so that every file is read.
            Err::<(), _>("compared".to_owned())
        });
        match found {
            Err(FindError::NoFile { .. }) => {}
            Err(error) => {
                unmatched.push(unmatched_why(error.to_string()));
                continue;
            }
            Ok(_) => return Err("every file is refused, so none can be found".to_owned()),
        }
        let mut compared: Vec<(String, Format, f64, f64)> = candidates
            .into_inner()
            .into_iter()
            .map(|(id, format, motor)| {
                let difference = largest_difference(openrocket.curve(), motor.curve());
                // Rounded, so a difference under a millionth decides nothing.
                let mass = mass_difference(openrocket, &motor);
                (id, format, rounded(difference), rounded(mass))
            })
            .collect();
        // Stable: of files alike in both, the order `hpr sim` would take them in.
        compared.sort_by(|a, b| a.2.total_cmp(&b.2).then(a.3.total_cmp(&b.3)));
        let others: Vec<f64> = compared.iter().skip(1).map(|c| c.2).collect();
        match compared.first() {
            Some((id, format, difference, mass)) if *difference < MATCH => {
                let mut entry = named.clone();
                entry["simfile_id"] = json!(id);
                entry["format"] = json!(format);
                entry["largest_difference"] = json!(difference);
                entry["mass_difference"] = json!(mass);
                // A file alike in thrust but not in mass was passed over: the mass decided.
                let decided_by_mass = compared
                    .get(1)
                    .is_some_and(|next| next.2 == *difference && next.3 > *mass);
                if decided_by_mass {
                    entry["decided_by_mass"] = json!(true);
                }
                entry["others_differences"] = json!(others);
                files.push(entry);
            }
            _ => unmatched.push(unmatched_why(format!(
                "no file of {} is within {MATCH} of OpenRocket's peak thrust: {}",
                compared.len(),
                compared
                    .iter()
                    .map(|c| format!("{} {:.3}", c.0, c.2))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))),
        }
    }
    Ok(json!({
        "about": "For each motor of OpenRocket 24.12's examples, by the digest a .ork records, the \
                  ThrustCurve.org data file holding the curve OpenRocket flies: written by \
                  `cargo xtask ork-curves` (ADR-161). largest_difference is the largest \
                  difference in thrust from OpenRocket's curve, over its peak thrust; \
                  mass_difference the larger difference of the propellant's and the dry motor's \
                  mass from OpenRocket's, over its loaded mass; others_differences the thrust \
                  difference for the motor's other files.",
        "openrocket": "24.12",
        "match_bound": MATCH,
        "files": files,
        "unmatched": unmatched,
    }))
}

/// The largest difference in thrust between two curves at any sample time of either, over the
/// first's peak thrust.
fn largest_difference(reference: &ThrustCurve, other: &ThrustCurve) -> f64 {
    let largest = reference
        .times_s()
        .iter()
        .chain(other.times_s())
        .map(|&t| (reference.thrust_n(t) - other.thrust_n(t)).abs())
        .fold(0.0, f64::max);
    largest / reference.peak_thrust_n()
}

/// The larger difference of the propellant's and the dry motor's mass between two motors, over
/// the first's loaded mass.
fn mass_difference(reference: &SolidMotor, other: &SolidMotor) -> f64 {
    let propellant =
        (reference.propellant_initial_mass_kg() - other.propellant_initial_mass_kg()).abs();
    let dry = (reference.dry().mass_kg - other.dry().mass_kg).abs();
    propellant.max(dry) / (reference.propellant_initial_mass_kg() + reference.dry().mass_kg)
}

/// Six decimals: the table's differences, short enough to read.
fn rounded(value: f64) -> f64 {
    (value * 1e6).round() / 1e6
}
