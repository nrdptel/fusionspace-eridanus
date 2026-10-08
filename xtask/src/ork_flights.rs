//! `cargo xtask ork-flights [--check]`: flies hpr on every configuration of OpenRocket's flight
//! record that hpr flies, in the recorded conditions, and reports the apogee, the largest speed and
//! the stability margin at rod clearance against OpenRocket's (M2.2d2, ADR-069).
//!
//! The record is `validation/fixtures/ork/openrocket-flights.json` (M2.2d1, ADR-068): OpenRocket
//! 24.12's calm-air flight of every motor configuration of its 17 examples and the seven Loft
//! demos. Each metric is taken by the [`definition`] hpr holds for OpenRocket 24.12 and compared
//! with [`compare`], so an aborted reference is withheld and a missing value is never scored:
//!
//! - **apogee**: the largest height of hpr's center of mass above the site, which is where hpr's
//!   apogee event puts it;
//! - **largest speed**: the largest speed of the center of mass, sampled at every step's end and
//!   at three points inside it from the dense output;
//! - **margin at rod clearance**: at the recorded rod-clearance step's time and Mach number, hpr's
//!   center of pressure at zero angle of attack less its center of mass at that time, over its
//!   reference diameter.
//!
//! The flight compared for the climb flies no recovery device, so a reference whose parachute
//! opened before its apogee is marked, with how long before, and is summarised apart. Each
//! configuration is then flown again with the file's recovery, as `hpr::ork::recovery` maps it,
//! and its descent compared with OpenRocket's ([`descent_flight`], ADR-153). A configuration with a powered separation ([`ork::Staging`], M1.9c, ADR-076)
//! flies it: the sustainer's apogee and largest speed are compared with OpenRocket's, whose
//! record holds the flight of the branch that keeps the nose. hpr's descent of a separated body
//! needs a device on each (ADR-074), so the sustainer tumbles from its apogee and the booster from
//! the separation; neither changes the climb that is compared. A lone separation with nothing
//! ahead of it left to burn (ADR-165) is the descent's: the climb flies whole ([`climbing`]); the
//! descent flies it, comparing the apogee and descent of the part that keeps the nose; and
//! [`coasting`] measures what that part would reach coasting from the split with no drag, the
//! flight `hpr sim` refuses. The configurations the record holds that hpr does not fly are listed with
//! the importer's reason.
//!
//! The motor curves come from OpenRocket's own database by digest (ADR-067), whose record lives
//! under the gitignored `corpus-out/`, and the examples from the pinned jar: so the flights run
//! only where both are fetched. What they write, [`REPORT_JSON`] and [`REPORT_MD`], is committed,
//! and a test holds its reference values to the record and its outcomes to [`compare`] in CI.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::FRAC_PI_2;
use std::fs;
use std::path::Path;

use hpr_aero::{DragTable, Flow};
use hpr_core::DVec3;
use hpr_core::geodesy::Geodetic;
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use hpr_io::ork;
use hpr_sim::recovery::{Device, DeviceDrag, Trigger};
use hpr_sim::{
    Environment, EventKind, FlightSettings, FlightStep, Observer, Phase, Rail, SimError, Simulation,
};
use hpr_validate::flight_metrics::{
    FlightMetric, MetricOutcome, OPENROCKET_MEASURED, ReferenceReading, Tool, compare, definition,
};
use serde_json::{Value, json};

pub const USAGE: &str = "\
  ork-flights [--check | --corpus [RECORD] | --library [--check]]
                           Fly hpr on each configuration of OpenRocket's flight record it
                           flies, and write validation/reports/openrocket-flights.{md,json}.
                           Needs the pinned jar and corpus-out/openrocket-motors.json. --check
                           compares with the committed report instead of writing it.
                           --corpus prints only counts of OpenRocket's flights of the
                           private library, from corpus-out/openrocket-flights.json or
                           another record flights.py wrote. --library flies the
                           private library's configurations from that record and
                           writes validation/reports/openrocket-library-flights.{md,json}
                           under anonymised ids (with --check, compares instead).";

/// OpenRocket's flights (M2.2d1).
pub(crate) const RECORD: &str = "validation/fixtures/ork/openrocket-flights.json";

/// The report, as data.
pub(crate) const REPORT_JSON: &str = "validation/reports/openrocket-flights.json";

/// The report, as a page.
pub(crate) const REPORT_MD: &str = "validation/reports/openrocket-flights.md";

/// The jar the examples are read from.
pub(crate) const JAR: &str = "refs/openrocket/OpenRocket-24.12.jar";

/// The pod probes (M1.13c2), which `validation/oracles/openrocket/pod_probes.py` writes.
pub(crate) const POD_PROBES: &str = "validation/fixtures/ork/pod-flights/";

/// The tilted-rod probes (M2.2e5), which `validation/oracles/openrocket/rod_probes.py` writes.
pub(crate) const ROD_PROBES: &str = "validation/fixtures/ork/rod-flights/";

/// The metrics compared, with their names in the report.
pub(crate) const METRICS: [(FlightMetric, &str); 3] = [
    (FlightMetric::Apogee, "apogee_m"),
    (FlightMetric::MaxSpeed, "max_speed_m_s"),
    (
        FlightMetric::RodClearanceStability,
        "rod_clearance_margin_cal",
    ),
];

/// An apogee difference above this, in per cent of OpenRocket's, needs a written cause (M2.2's
/// parent *done when*).
const APOGEE_CAUSE_PERCENT: f64 = 5.0;

/// How far `--check` lets a regenerated number move, relative.
pub(crate) const CHECK_RELATIVE: f64 = 1e-9;

pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        [flag] if flag == "--corpus" => return crate::ork_corpus_flights::run(None),
        [flag] if flag == "--library" => return crate::ork_library_flights::run(false),
        [flag, check] if flag == "--library" && check == "--check" => {
            return crate::ork_library_flights::run(true);
        }
        [flag, record] if flag == "--corpus" => {
            return crate::ork_corpus_flights::run(Some(record));
        }
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::ork::root()?;
    let record = read_json(&root.join(RECORD))?;
    let report = fly_all(&root, &record)?;
    let page = page(&report);
    if check {
        let committed = read_json(&root.join(REPORT_JSON))?;
        let mut apart = Vec::new();
        same(&committed, &report, "", &mut apart);
        let committed_page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("{REPORT_MD}: {error}"))?;
        if committed_page.replace("\r\n", "\n") != page {
            apart.push(format!("{REPORT_MD} differs from what the flights write"));
        }
        if !apart.is_empty() {
            return Err(format!(
                "the committed report is stale; run `cargo xtask ork-flights`:\n  {}",
                apart.join("\n  ")
            ));
        }
        println!("ork-flights: the committed report matches the flights");
    } else {
        let text = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
        fs::write(root.join(REPORT_JSON), text)
            .map_err(|error| format!("{REPORT_JSON}: {error}"))?;
        fs::write(root.join(REPORT_MD), &page).map_err(|error| format!("{REPORT_MD}: {error}"))?;
        println!("ork-flights: wrote {REPORT_MD} and {REPORT_JSON}");
    }
    print!("{}", summary_lines(&report));
    Ok(())
}

pub(crate) fn read_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
}

/// Flies every configuration of the record that hpr flies, and builds the report.
fn fly_all(root: &Path, record: &Value) -> Result<Value, String> {
    let jar = root.join(JAR);
    if !jar.is_file() {
        return Err(format!(
            "{JAR} is missing: fetch it with `cargo xtask refs fetch`"
        ));
    }
    let examples = crate::ork::examples_in_jar(&jar)?;
    let supply = crate::ork_supply::Supply::load(root, true)?;
    if !supply.is_present() {
        return Err(format!(
            "{} is missing: run validation/oracles/openrocket/motor_database.py (ADR-067)",
            crate::ork_supply::RECORD
        ));
    }
    if let Some(failure) = supply.failure() {
        return Err(format!("the supplied curves fail their checks: {failure}"));
    }
    let designs = record["designs"]
        .as_array()
        .ok_or_else(|| format!("{RECORD} has no `designs` list"))?;
    let text = fs::read_to_string(root.join(PUBLIC_DRAG_CURVES))
        .map_err(|error| format!("{PUBLIC_DRAG_CURVES}: {error}"))?;
    let drag_curves: Value =
        serde_json::from_str(&text).map_err(|error| format!("{PUBLIC_DRAG_CURVES}: {error}"))?;
    let public = Mode::Public {
        drag_curves: &drag_curves,
    };
    let mut flights = Vec::new();
    let mut not_flown = Vec::new();
    let mut probes = Vec::new();
    for entry in designs {
        let file = entry["file"].as_str().ok_or("a design without a file")?;
        let probe = file.starts_with(POD_PROBES) || file.starts_with(ROD_PROBES);
        let Some(recorded) = entry["flights"].as_array() else {
            if probe {
                return Err(format!("OpenRocket flew nothing of the probe {file}"));
            }
            continue;
        };
        let bytes = match file.split_once('!') {
            Some((_, inside)) => examples
                .iter()
                .find(|(name, _)| name.ends_with(&format!("!{inside}")))
                .map(|(_, bytes)| bytes.clone())
                .ok_or_else(|| format!("{inside} is not in {JAR}"))?,
            None => fs::read(root.join(file)).map_err(|error| format!("{file}: {error}"))?,
        };
        let digest = crate::ork_supply::sha256(&bytes);
        if Some(digest.as_str()) != entry["sha256"].as_str() {
            return Err(format!(
                "{file} is not the file the record flew (SHA-256 differs)"
            ));
        }
        let name = design_name(file);
        let label =
            |_: usize, flight: &Value| flight["name"].as_str().unwrap_or_default().to_owned();
        let flown = fly_design(file, &name, &bytes, recorded, &supply, label, &public)?;
        if probe {
            // A probe is not a design: it is listed apart, out of the designs' statistics, and
            // it exists to be flown.
            if let Some(refused) = flown.not_flown.first() {
                return Err(format!("the probe {name} is not flown: {}", refused["why"]));
            }
            if flown.flights.is_empty() {
                return Err(format!("the probe {name} has no flight"));
            }
            probes.extend(flown.flights);
            continue;
        }
        flights.extend(flown.flights);
        not_flown.extend(flown.not_flown);
    }
    if !probes.is_empty() && !probes.iter().any(|probe| probe["design"] == WITHOUT_PODS) {
        return Err(format!(
            "the probes have no {WITHOUT_PODS} to measure the pods and the rods against"
        ));
    }
    let summary = summarise(&flights, &not_flown);
    Ok(json!({
        "generated_by": "cargo xtask ork-flights",
        "record": RECORD,
        "reference": { "tool": "OpenRocket", "version": OPENROCKET_MEASURED },
        "summary": summary,
        "flights": flights,
        "not_flown": not_flown,
        "probes": probes,
    }))
}

/// The probe of the pods' airframe alone, which the others' pods are measured against.
pub(crate) const WITHOUT_PODS: &str = "pods-none";

/// What a probe's pods or rod change, against [`WITHOUT_PODS`]: the apogee in per cent and the
/// margin at rod clearance in calibres, each as `[openrocket, hpr]`.
pub(crate) fn probe_change(probe: &Value, without: &Value) -> Option<[[f64; 2]; 2]> {
    let number =
        |flight: &Value, metric: &str, code: &str| flight["metrics"][metric][code].as_f64();
    let mut change = [[0.0; 2]; 2];
    for (at, code) in ["openrocket", "hpr"].into_iter().enumerate() {
        let apogee = number(probe, "apogee_m", code)? / number(without, "apogee_m", code)?;
        change[0][at] = 100.0 * (apogee - 1.0);
        change[1][at] = number(probe, "rod_clearance_margin_cal", code)?
            - number(without, "rod_clearance_margin_cal", code)?;
    }
    Some(change)
}

/// What hpr made of one design's recorded flights.
pub(crate) struct Flown {
    /// The flights hpr flew, each compared with OpenRocket's.
    pub flights: Vec<Value>,
    /// The configurations OpenRocket flew that hpr did not, with why.
    pub not_flown: Vec<Value>,
}

/// Flies every powered configuration of `recorded`, OpenRocket's flights of the design `file`
/// whose bytes are `bytes`, that hpr flies, under the name `name`. `label` names a configuration
/// in the report from its place among the design's recorded configurations (from 1) and its
/// record.
///
/// A configuration is flown only when OpenRocket is shown to fly the curves hpr is given: each
/// motor's curve is the design's own embedded one or one supplied for its digest, and the motor
/// record ([`crate::ork_supply::RECORD`]) finds OpenRocket placing those digests in that
/// configuration. A curve from hpr's bundled catalog is found by name, which can name another
/// curve than the digest OpenRocket loads, and OpenRocket's loader takes another curve without
/// saying so in the flight record when a digest is not in its database.
///
/// A record that lacks what a comparison needs is an error. So is a flight hpr fails, unless
/// flying the library ([`Mode::Library`]), when the configuration is listed as [`FLIGHT_FAILED`]
/// and the detail, which can name a part, is printed on this machine only.
pub(crate) fn fly_design(
    file: &str,
    name: &str,
    bytes: &[u8],
    recorded: &[Value],
    supply: &crate::ork_supply::Supply,
    label: impl Fn(usize, &Value) -> String,
    mode: &Mode,
) -> Result<Flown, String> {
    let list_failures = matches!(mode, Mode::Library { .. });
    let read = ork::read(bytes).map_err(|error| format!("{name}: {error}"))?;
    let design = ork::design_with(&read.value, supply.curves()).value;
    let sha = crate::ork_supply::sha256(bytes);
    let curves = match mode {
        Mode::Library { drag_curves } | Mode::Public { drag_curves } => drag_curves["designs"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|entry| entry["sha256"].as_str() == Some(sha.as_str())),
    };
    let overrides = drag_overrides(&read.value.document.root);
    let powered: Vec<(usize, &Value)> = recorded
        .iter()
        .enumerate()
        .filter(|(_, f)| f["has_motors"] == true)
        .map(|(index, f)| (index + 1, f))
        .collect();
    let mut out = Flown {
        flights: Vec::new(),
        not_flown: Vec::new(),
    };
    for &(place, flight) in &powered {
        let id = flight["configuration"]
            .as_str()
            .ok_or("a flight without a configuration")?;
        let motors = label(place, flight);
        let not_flown = |why: &str| {
            json!({
                "file": file,
                "design": name,
                "configuration": id,
                "motors": motors,
                "aborted": flight["aborted"],
                "why": why,
            })
        };
        if flight["refused"].is_string() {
            out.not_flown.push(not_flown(REFUSED));
            continue;
        }
        // OpenRocket gives a configuration whose id is not a UUID a new one, so a design's
        // only configuration is matched to the record's only powered flight.
        let configurations = &design.motors.configurations;
        let (matched, renamed) = match configurations
            .iter()
            .find(|c| c.id.eq_ignore_ascii_case(id))
        {
            Some(configuration) => (Some(configuration), false),
            None if configurations.len() == 1 && powered.len() == 1 => {
                (configurations.first(), true)
            }
            None => (None, false),
        };
        let flown = matched.and_then(|matched| {
            design
                .rocket
                .configurations
                .iter()
                .find(|c| c.id == matched.id)
        });
        let unconfirmed = match matched.filter(|_| flown.is_some()) {
            Some(matched) => match unflown_conditions(&flight["conditions"])
                .map_err(|error| format!("{motors}: {error}"))?
            {
                Some(why) => Some(why),
                None => unconfirmed_curve(
                    &curve_sources(matched),
                    supply
                        .placed(&sha, id)
                        .map_err(|error| format!("{name}: {error}"))?,
                ),
            },
            None => None,
        };
        let Some(configuration) = flown.filter(|_| unconfirmed.is_none()) else {
            let why = match (flown, unconfirmed) {
                (Some(_), Some(why)) => why.to_owned(),
                _ => matched.and_then(|c| c.left_out.as_ref()).map_or_else(
                    || NO_SUCH_CONFIGURATION.to_owned(),
                    |out| {
                        let prefix = if renamed { RENAMED } else { "" };
                        format!("{prefix}{}", crate::ork_motors::not_flown(out.why))
                    },
                ),
            };
            out.not_flown.push(not_flown(&why));
            continue;
        };
        recorded_enough(flight).map_err(|error| format!("{motors}: {error}"))?;
        let stagings: Vec<ork::Staging> = matched
            .map(|matched| matched.stagings().cloned().collect())
            .unwrap_or_default();
        let flew_with =
            |rocket: &hpr_design::Rocket, drag: Drag, descent: Option<&ork::Recovery>| {
                let flown = (name, rocket, &configuration.id[..], &stagings[..]);
                fly(flown, drag, descent, &motors, flight).map_err(|error| {
                    if list_failures {
                        eprintln!("{motors}: {FLIGHT_FAILED}: {error}");
                    }
                    error
                })
            };
        let flew_as = |rocket: &hpr_design::Rocket, drag: Drag| flew_with(rocket, drag, None);
        // The flight compared is flown a second time with the file's recovery (ADR-153).
        let mut entry = match flew_with(&design.rocket, Drag::Own, Some(&design.recovery)) {
            Ok(entry) => entry,
            Err(_) if list_failures => {
                out.not_flown.push(not_flown(FLIGHT_FAILED));
                continue;
            }
            Err(error) => return Err(error),
        };
        entry["file"] = json!(file);
        if !overrides.is_empty() {
            entry["drag_overrides_not_applied"] =
                json!(overrides.iter().map(|o| &o.name).collect::<Vec<_>>());
        }
        if let Some(sized) =
            causes_removed(flight, &entry, &overrides).map_err(|e| format!("{motors}: {e}"))?
        {
            entry["apogee_with_the_causes_removed"] = sized;
        }
        if drag_probed(&entry) {
            // An apogee off by more than the bar with no named cause, or with an early parachute
            // that, held, still leaves more than the bar: hpr flies it again on OpenRocket's own
            // drag, where the library's record has it (ADR-097). Within the bar, the drag is the
            // first one's named cause, and sizes what the second one's parachute leaves. Flown
            // also with only OpenRocket's base drag under power, which says how much of it that
            // one rule is.
            let table = curves
                .map(|curves| openrocket_drag(curves, id, renamed))
                .transpose()
                .map_err(|error| format!("{motors}: {error}"))?
                .flatten()
                .filter(|_| stagings.is_empty());
            let probes = [
                (WHOLE_BASE_PROBE, Some(Drag::WholeBase)),
                (OPENROCKET_DRAG_PROBE, table.map(Drag::OpenRocket)),
            ];
            // A probe hpr fails to fly is marked so, and the flight it probes stays compared: it
            // then has no named cause, which the bar's test reports.
            for (key, drag) in probes {
                let Some(drag) = drag else { continue };
                entry[key] = match flew_as(&design.rocket, drag) {
                    Ok(probe) => json!({
                        "apogee_m": probe["metrics"]["apogee_m"],
                        "max_speed_m_s": probe["metrics"]["max_speed_m_s"],
                    }),
                    Err(_) if list_failures => json!({ "failed": true }),
                    Err(error) => return Err(error),
                };
            }
            if let Some(sized) = on_openrocket_s_drag_held(&entry) {
                entry["apogee_with_the_causes_removed"][OPENROCKET_DRAG_PROBE] = sized;
            }
        }
        out.flights.push(entry);
    }
    Ok(out)
}

/// Checks the record holds everything [`fly`] reads from it, so that an error from [`fly`] is
/// hpr's, not the record's.
fn recorded_enough(flight: &Value) -> Result<(), String> {
    let number = |value: &Value, what: &str| {
        value
            .as_f64()
            .ok_or_else(|| format!("the record has no {what}"))
    };
    let conditions = &flight["conditions"];
    Geodetic::from_degrees(
        number(&conditions["launch_latitude_deg"], "latitude")?,
        number(&conditions["launch_longitude_deg"], "longitude")?,
        number(&conditions["launch_altitude_m"], "launch altitude")?,
    )
    .map_err(|error| error.to_string())?;
    number(&conditions["rod_length_m"], "rod length")?;
    number(&conditions["rod_angle_rad"], "rod angle")?;
    number(&conditions["rod_direction_rad"], "rod direction")?;
    number(&flight["rod_clearance"]["time_s"], "rod-clearance time")?;
    number(
        &flight["rod_clearance"]["mach"],
        "rod-clearance Mach number",
    )?;
    early_chute(flight)?;
    Ok(())
}

/// Why a configuration is not flown: OpenRocket refused to fly it.
pub(crate) const REFUSED: &str = "OpenRocket refused to fly it";

/// Why a configuration is not flown: hpr's flight of it failed.
pub(crate) const FLIGHT_FAILED: &str = "hpr's flight of it failed";

/// Why a configuration is not flown: the importer builds none of the recorded id.
pub(crate) const NO_SUCH_CONFIGURATION: &str = "the importer builds no configuration of that id";

/// Put before the importer's reason when OpenRocket gave the design's only configuration a new id.
pub(crate) const RENAMED: &str =
    "the design's only configuration, which OpenRocket gave a new id: ";

/// Why a configuration is not flown: a launch rod hpr's rail does not take.
pub(crate) const ROD_NOT_TAKEN: &str =
    "a launch rod tilted outside 0 up to 90 degrees from the vertical";

/// Why a configuration is not flown: wind.
pub(crate) const WIND: &str = "wind";

/// Why a configuration is not flown: an atmosphere other than the standard one.
pub(crate) const NOT_STANDARD_AIR: &str = "an atmosphere other than the standard one";

/// Why a configuration is not flown: a curve from hpr's bundled catalog, found by name.
pub(crate) const CURVE_BY_NAME: &str = "a curve found by name, which OpenRocket may not fly";

/// Why a configuration is not flown: the motor record does not find OpenRocket placing its curves.
pub(crate) const NOT_PLACED: &str = "a curve OpenRocket is not shown to place";

/// Why the recorded launch `conditions` are not ones this report flies: it flies a rod tilted
/// from 0 up to 90 degrees from the vertical ([`rod_rail`]) in calm standard air only, as [`fly`]
/// requires. A condition the record does not state is an error, not a reason.
pub(crate) fn unflown_conditions(conditions: &Value) -> Result<Option<&'static str>, String> {
    let number = |key: &str| {
        conditions[key]
            .as_f64()
            .ok_or_else(|| format!("the record states no `{key}`"))
    };
    let (rod, wind, turbulence) = (
        number("rod_angle_rad")?,
        number("wind_average_m_s")?,
        number("wind_turbulence")?,
    );
    let (model, standard) = (
        conditions["wind_model"]
            .as_str()
            .ok_or("the record states no `wind_model`")?,
        conditions["isa_atmosphere"]
            .as_bool()
            .ok_or("the record states no `isa_atmosphere`")?,
    );
    Ok(if !(0.0..FRAC_PI_2).contains(&rod) {
        Some(ROD_NOT_TAKEN)
    } else if wind != 0.0 || turbulence != 0.0 || model != "AVERAGE" {
        // `flights.py` calms the average wind model only.
        Some(WIND)
    } else if !standard {
        Some(NOT_STANDARD_AIR)
    } else {
        None
    })
}

/// The rail that OpenRocket's recorded launch rod stands for (M2.2e5, issue #173).
///
/// OpenRocket's `rod_angle_rad` is the rod's angle from the vertical and `rod_direction_rad` the
/// compass bearing it leans toward: `conditions.py` measured that a rocket from a rod tilted
/// toward 0 lands north of the pad and one tilted toward π/2 lands east
/// (`validation/fixtures/ork/openrocket-conditions.json`). hpr's rail takes the same bearing,
/// clockwise from true north, and its angle above the horizon, `π/2` less OpenRocket's. The rail
/// is frictionless and the rocket unrolled on it, as the vertical rod was flown before.
///
/// # Errors
///
/// When the record states no rod length, angle or direction, or the rail refuses them.
pub(crate) fn rod_rail(conditions: &Value) -> Result<Rail, String> {
    let number = |key: &str| {
        conditions[key]
            .as_f64()
            .ok_or_else(|| format!("the record states no `{key}`"))
    };
    let rail = Rail {
        azimuth_rad: number("rod_direction_rad")?,
        elevation_rad: FRAC_PI_2 - number("rod_angle_rad")?,
        ..Rail::vertical(number("rod_length_m")?)
    };
    rail.validate().map_err(|error| error.to_string())?;
    Ok(rail)
}

/// Where the curve of one motor hpr flies came from, as [`unconfirmed_curve`] needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CurveSource<'a> {
    /// The design's own embedded curve, or one supplied for the motor's digest: the digest.
    Digest(&'a str),
    /// hpr's bundled catalog, found by name.
    Catalog,
    /// Anything else, such as a curve with no digest.
    Other,
}

/// The curve sources of the motors of `configuration`.
fn curve_sources(configuration: &ork::MotorConfiguration) -> Vec<CurveSource<'_>> {
    configuration
        .motors
        .iter()
        .map(|motor| match (&motor.curve, &motor.digest) {
            (ork::Curve::Embedded { .. } | ork::Curve::Supplied { .. }, Some(digest)) => {
                CurveSource::Digest(digest)
            }
            (ork::Curve::Catalog { .. }, _) => CurveSource::Catalog,
            _ => CurveSource::Other,
        })
        .collect()
}

/// Why OpenRocket is not shown to fly the curves hpr flies, given each motor's [`CurveSource`] and
/// the digests the motor record finds OpenRocket placing in that configuration (`None` when it
/// finds no such configuration): nothing when every curve has a digest and OpenRocket places
/// exactly those digests, as many times each.
pub(crate) fn unconfirmed_curve(
    curves: &[CurveSource<'_>],
    placed: Option<&[String]>,
) -> Option<&'static str> {
    if curves.contains(&CurveSource::Catalog) {
        return Some(CURVE_BY_NAME);
    }
    let Some(placed) = placed else {
        return Some(NOT_PLACED);
    };
    let mut left: Vec<&str> = placed.iter().map(String::as_str).collect();
    for curve in curves {
        let CurveSource::Digest(digest) = curve else {
            return Some(NOT_PLACED);
        };
        match left.iter().position(|d| d == digest) {
            Some(at) => {
                left.swap_remove(at);
            }
            None => return Some(NOT_PLACED),
        }
    }
    if left.is_empty() {
        None
    } else {
        Some(NOT_PLACED)
    }
}

/// A design's name in the report: an example's file name, or a demo's.
fn design_name(file: &str) -> String {
    let base = file.rsplit(['/', '!']).next().unwrap_or(file);
    base.strip_suffix(".ork").unwrap_or(base).to_owned()
}

/// A drag coefficient the design (not one of its stored simulations) states that hpr does not
/// apply: one on the rocket itself (ADR-167).
#[derive(Clone)]
pub(crate) struct DragOverride {
    /// The id of the element that states it.
    pub id: String,
    /// Its name.
    pub name: String,
}

/// The drag coefficients the design states (`<overridecd>`) that hpr does not apply: the
/// rocket's own, which `hpr-design` has no place for. A part's and a stage's fly (ADR-167).
pub(crate) fn drag_overrides(root: &ork::Element) -> Vec<DragOverride> {
    root.children_named("rocket")
        .filter(|rocket| rocket.child("overridecd").is_some())
        .map(|rocket| {
            let text = |tag: &str| {
                rocket
                    .child(tag)
                    .map(|e| e.text().trim().to_owned())
                    .unwrap_or_default()
            };
            DragOverride {
                id: text("id"),
                name: text("name"),
            }
        })
        .collect()
}

/// The center of mass's height and place at the start, and its largest speed up to apogee, from
/// the dense output. The flight it watches flies no recovery, so its fall is unbraked and is left
/// out: the reference's peak speed is on the way up, and a free fall could outrun it.
///
/// The largest speed is taken over every sample up to the highest one (the apogee itself is the
/// flight's apogee event). An
/// earlier rule stopped at the first sample whose vertical speed was not positive after one that
/// was, and a rocket held on the pad can show a vertical speed of a few µm/s either way (the
/// held state in Earth's frame): on one private flight it stopped before liftoff, and the
/// largest speed read 100% low.
#[derive(Default)]
struct Peaks {
    start_height_m: Option<f64>,
    start_enu_m: Option<DVec3>,
    /// The largest speed up to the highest sample so far, m/s.
    max_speed_m_s: Option<f64>,
    /// The largest speed so far, m/s.
    running_speed_m_s: f64,
    highest_m: Option<f64>,
    /// The least static margin in free flight up to the highest sample so far, in the rocket's
    /// weakest plane ([`hpr_sim::metrics::weakest_margin`]), calibres; samples with no margin
    /// left out.
    least_margin_cal: Option<f64>,
    /// The least static margin in free flight so far, calibres.
    running_margin_cal: Option<f64>,
    /// When the angle of attack first passed 90° in free flight, the air arriving from behind the
    /// rocket's beam, up to the highest sample so far, s.
    turns_over_s: Option<f64>,
    /// When the angle of attack first passed 90° in free flight so far, s.
    running_turn_s: Option<f64>,
    /// A time to sample, s: OpenRocket's last row of a flight it aborted (ADR-172).
    at_s: Option<f64>,
    /// The height above the start and the speed at `at_s`, m and m/s.
    at: Option<(f64, f64)>,
}

impl Observer for Peaks {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let (start, end) = (step.start_s(), step.end_s());
        if self.start_height_m.is_none() {
            let sample = step.sample(start)?;
            self.start_height_m = Some(sample.height_above_ground_m);
            self.start_enu_m = Some(sample.cg_enu_m);
        }
        if let (Some(at_s), Some(start_m)) = (self.at_s, self.start_height_m)
            && start < at_s
            && at_s <= end
        {
            let sample = step.sample(at_s)?;
            self.at = Some((
                sample.height_above_ground_m - start_m,
                sample.cg_velocity_enu_m_s.length(),
            ));
        }
        for k in 1..=4 {
            let t = if k == 4 {
                end
            } else {
                start + (end - start) * f64::from(k) / 4.0
            };
            let sample = step.sample(t)?;
            self.running_speed_m_s = self
                .running_speed_m_s
                .max(sample.cg_velocity_enu_m_s.length());
            if step.phase() == Phase::Free
                && let Some(cal) = step.stability(t)?.static_margin.margin_cal
            {
                self.running_margin_cal = Some(self.running_margin_cal.map_or(cal, |m| m.min(cal)));
            }
            if step.phase() == Phase::Free
                && self.running_turn_s.is_none()
                && sample.angle_of_attack_rad > std::f64::consts::FRAC_PI_2
            {
                self.running_turn_s = Some(t);
            }
            if self
                .highest_m
                .is_none_or(|highest| sample.height_above_ground_m > highest)
            {
                self.highest_m = Some(sample.height_above_ground_m);
                self.max_speed_m_s = Some(self.running_speed_m_s);
                self.least_margin_cal = self.running_margin_cal;
                self.turns_over_s = self.running_turn_s;
            }
        }
        Ok(())
    }
}

/// `simulation` with `stagings`' separations, each dropped part tumbling from its split and the
/// sustainer from its apogee: hpr's descent of a separated body needs a device on each (ADR-074).
pub(crate) fn staged(
    simulation: Simulation,
    stagings: &[ork::Staging],
) -> Result<Simulation, SimError> {
    let assembly = simulation.assembly();
    let separations = hpr::ork::separations(stagings, assembly)?;
    let stage_count = assembly.layout.stages.len();
    let stages = |body: usize| {
        hpr_sim::Separation::stages_of_body(&separations, body, stage_count).ok_or(
            SimError::Domain {
                what: "stage boundary of a separation (there is no stage aft of it, or it is \
                       not forward of the one before)",
                value: body as f64,
            },
        )
    };
    let sustainer = DeviceDrag::tumbling_stages(assembly, stages(0)?)?;
    let mut devices = vec![Device::new(
        "the sustainer, tumbling",
        sustainer,
        Trigger::Apogee,
    )];
    for body in 1..=separations.len() {
        let drag = DeviceDrag::tumbling_stages(assembly, stages(body)?)?;
        let name = if body == 1 {
            "the booster, tumbling".to_owned()
        } else {
            format!("part {body}, tumbling")
        };
        devices.push(Device::new(name, drag, Trigger::Time { time_s: 0.0 }).on_body(body));
    }
    simulation
        .with_recovery(devices)?
        .with_separations(separations)
}

/// The separations of `stagings` the climb flies: all of them, but for a lone separation with
/// nothing ahead of it left to burn (ADR-165), which belongs to the descent: with no device of its
/// own open, the part that keeps the nose would coast from it with no drag, so the climb compared
/// flies whole, as OpenRocket's undeployed flight would were the payload's airframe kept on.
pub(crate) fn climbing<'a>(
    stagings: &'a [ork::Staging],
    assembly: &hpr_design::Assembly,
) -> Result<&'a [ork::Staging], SimError> {
    let separations = hpr::ork::separations(stagings, assembly)?;
    Ok(match (separations.as_slice(), stagings) {
        ([separation], [staging]) if !separation.powered_at(assembly, staging.time_s) => &[],
        _ => stagings,
    })
}

/// How [`fly_design`] flies: the public designs, where a flight hpr fails stops the report, or the
/// private library, where it is listed, with OpenRocket's drag curves ([`DRAG_CURVES`]) to size a
/// cause in the drag.
pub(crate) enum Mode<'a> {
    /// The public designs, with the record `drag_curves.py` wrote of the jar's examples
    /// ([`PUBLIC_DRAG_CURVES`]).
    Public {
        /// That record.
        drag_curves: &'a Value,
    },
    /// The private library, with the record `drag_curves.py` wrote of it.
    Library {
        /// That record.
        drag_curves: &'a Value,
    },
}

/// Where `drag_curves.py` writes OpenRocket's drag along its flights of the private library.
pub(crate) const DRAG_CURVES: &str = "corpus-out/openrocket-drag-curves.json";

/// OpenRocket's drag along its flights of the jar's examples (M2.2e9), which `drag_curves.py`
/// writes from the examples unpacked outside `refs/`, where `cargo xtask ork` would count them:
///
/// ```text
/// mkdir -p target/openrocket-examples && cd target/openrocket-examples &&
///   unzip -o -j ../../refs/openrocket/OpenRocket-24.12.jar 'datafiles/examples/*.ork' && cd -
/// refs/venv/bin/python validation/oracles/openrocket/drag_curves.py \
///   validation/fixtures/ork/openrocket-drag-curves.json target/openrocket-examples
/// ```
///
/// The examples are public, so the record is committed.
pub(crate) const PUBLIC_DRAG_CURVES: &str = "validation/fixtures/ork/openrocket-drag-curves.json";

/// The drag hpr flies a configuration on.
enum Drag {
    /// Its own buildup.
    Own,
    /// Its own buildup with the base's whole drag kept while a motor burns, as OpenRocket's
    /// ([`Simulation::with_full_base_drag_under_power`]).
    WholeBase,
    /// OpenRocket's drag coefficient along its own flight ([`openrocket_drag`]).
    OpenRocket(DragTable),
}

/// OpenRocket's drag along its flight of `configuration`, from the design's entry in
/// [`DRAG_CURVES`], as a table hpr can fly: power on while a motor burns, power off after, each
/// linear in Mach number and held at its ends, on OpenRocket's reference diameter. `None` when the
/// record has no such flight, or OpenRocket refused or aborted it, or it has more than one branch
/// (a separation: the curves would join two shapes), or a curve has fewer than two points. A
/// configuration OpenRocket `renamed` (its id is not a UUID, so each opening gives it a new one)
/// is the record's only flight, as [`fly_design`] matches it.
fn openrocket_drag(
    design: &Value,
    configuration: &str,
    renamed: bool,
) -> Result<Option<DragTable>, String> {
    let flights: Vec<&Value> = design["flights"].as_array().into_iter().flatten().collect();
    let found = flights.iter().find(|flight| {
        flight["configuration"]
            .as_str()
            .is_some_and(|id| id.eq_ignore_ascii_case(configuration))
    });
    let only = if renamed && flights.len() == 1 {
        flights.first()
    } else {
        None
    };
    let Some(flight) = found.or(only).copied() else {
        return Ok(None);
    };
    if !flight["refused"].is_null() || flight["aborted"] != false || flight["branches"] != 1 {
        return Ok(None);
    }
    let curve = |key: &str| -> Result<Option<Table1D>, String> {
        let points = flight[key]
            .as_array()
            .ok_or_else(|| format!("{DRAG_CURVES} has no {key} curve"))?;
        if points.len() < 2 {
            return Ok(None);
        }
        let mut xs = Vec::with_capacity(points.len());
        let mut ys = Vec::with_capacity(points.len());
        for point in points {
            let (Some(mach), Some(cd)) = (point[0].as_f64(), point[1].as_f64()) else {
                return Err(format!(
                    "{DRAG_CURVES} has a {key} point that is not two numbers"
                ));
            };
            xs.push(mach);
            ys.push(cd);
        }
        Table1D::new(xs, ys, Interpolation::Linear, Extrapolation::Clamp)
            .map(Some)
            .map_err(|error| format!("{DRAG_CURVES}'s {key} curve: {error}"))
    };
    let (Some(off), Some(on)) = (curve("power_off")?, curve("power_on")?) else {
        return Ok(None);
    };
    let reference = flight["reference_length_m"]
        .as_f64()
        .ok_or_else(|| format!("{DRAG_CURVES} has no reference length"))?;
    Ok(Some(
        DragTable::new(off, Some(on))
            .with_reference_diameter_m(reference)
            .map_err(|error| format!("{DRAG_CURVES}: {error}"))?,
    ))
}

/// Flies one configuration (the design's name, its rocket, the configuration's id and its
/// stagings) in the recorded conditions, with its powered separations if it has any, on `drag`,
/// and compares it with the record.
fn fly(
    (design, rocket, configuration, stagings): (&str, &hpr_design::Rocket, &str, &[ork::Staging]),
    drag: Drag,
    descent: Option<&ork::Recovery>,
    motors: &str,
    recorded: &Value,
) -> Result<Value, String> {
    let at = format!("{design} {motors}");
    let number = |value: &Value, what: &str| {
        value
            .as_f64()
            .ok_or_else(|| format!("{at}: the record has no {what}"))
    };
    let conditions = &recorded["conditions"];
    let clearance = &recorded["rod_clearance"];
    let aborted = recorded["aborted"] == true;
    let site = Geodetic::from_degrees(
        number(&conditions["launch_latitude_deg"], "latitude")?,
        number(&conditions["launch_longitude_deg"], "longitude")?,
        number(&conditions["launch_altitude_m"], "launch altitude")?,
    )
    .map_err(|error| format!("{at}: {error}"))?;
    if number(&conditions["wind_average_m_s"], "wind")? != 0.0
        || conditions["isa_atmosphere"] != true
    {
        return Err(format!("{at}: only calm standard air is flown here"));
    }
    let rail = rod_rail(conditions).map_err(|error| format!("{at}: {error}"))?;
    let rod_length_m = rail.length_m;
    let environment = Environment::standard(site).map_err(|error| format!("{at}: {error}"))?;
    // OpenRocket flies a design whatever hpr's checks find in it, so hpr does too, and the report
    // lists what they found.
    let findings = hpr_design::checks::check(rocket).map_err(|error| format!("{at}: {error}"))?;
    let design_errors: Vec<_> = findings
        .iter()
        .filter(|finding| finding.severity() == hpr_design::checks::Severity::Error)
        .collect();
    let settings = FlightSettings {
        accept_design_errors: true,
        ..FlightSettings::default()
    };
    let descended = descent.map(|recovery| {
        let flown = Simulation::new(rocket, configuration, environment.clone(), rail, settings);
        flown.map_or_else(
            |error| Err(format!("{at}: {error}")),
            |simulation| {
                Ok(descent_flight(
                    simulation, recovery, rocket, stagings, recorded,
                ))
            },
        )
    });
    let simulation = Simulation::new(rocket, configuration, environment, rail, settings)
        .map_err(|error| format!("{at}: {error}"))?;
    let all_stagings = stagings;
    let stagings =
        climbing(stagings, simulation.assembly()).map_err(|error| format!("{at}: {error}"))?;
    // A lone split with nothing ahead of it left to burn: what the part that keeps the nose
    // would reach coasting from it with no drag, the flight ADR-165 refuses, against OpenRocket's
    // flight of the same with no device deployed, where the part flies on its own airframe.
    let coast = if stagings.len() == all_stagings.len() {
        None
    } else {
        let coasted = Simulation::new(
            rocket,
            configuration,
            simulation.environment().clone(),
            rail,
            settings,
        )
        .map_err(|error| format!("{at}: {error}"))?;
        Some(coasting(coasted, all_stagings, recorded).map_err(|error| format!("{at}: {error}"))?)
    };
    let simulation = if stagings.is_empty() {
        simulation
    } else {
        staged(simulation, stagings).map_err(|error| format!("{at}: {error}"))?
    };
    let simulation = match drag {
        Drag::Own => simulation,
        Drag::WholeBase => simulation.with_full_base_drag_under_power(),
        Drag::OpenRocket(table) => simulation.with_drag_table(table),
    };
    // OpenRocket's last row of a flight it aborted, where the two are compared (ADR-172).
    let last_row_s = aborted
        .then(|| number(&recorded["series"]["last_time_s"], "last row's time"))
        .transpose()?;
    let mut peaks = Peaks {
        at_s: last_row_s,
        ..Peaks::default()
    };
    let result = simulation
        .run(&mut peaks)
        .map_err(|error| format!("{at}: {error}"))?;
    // OpenRocket's altitude is 0 at launch; hpr's center of mass starts above the ground (the
    // rocket stands on the rail's foot), so hpr's apogee is counted from where it starts.
    let apogee_m = result
        .event(EventKind::Apogee)
        .zip(peaks.start_height_m)
        .map(|(event, start)| event.sample.height_above_ground_m - start);
    // Where the rocket is at apogee, east and north of where it started, which OpenRocket's
    // position at its highest row is (its position is 0 at launch): what says a tilted rod sent it
    // the same way in both codes.
    let apogee_moved_m = result
        .event(EventKind::Apogee)
        .zip(peaks.start_enu_m)
        .map(|(event, start)| event.sample.cg_enu_m - start);

    // The margin at the recorded rod-clearance step: hpr's mass at its time, and its center of
    // pressure at its Mach number with the air along the axis.
    let assembly = simulation.assembly();
    let clearance_time_s = number(&clearance["time_s"], "rod-clearance time")?;
    let clearance_mach = number(&clearance["mach"], "rod-clearance Mach number")?;
    // Before any separation, so a motor waiting on one counts as unlit.
    let lit = assembly.ignition_times_s(|_| None);
    let apogee_s = result
        .event(EventKind::Apogee)
        .map(|apogee| apogee.sample.time_s)
        .ok_or_else(|| format!("{at}: hpr's flight has no apogee"))?;
    let burns: Vec<_> = assembly
        .motors
        .iter()
        .map(|motor| (motor.mounted.motor.burnout_time_s(), motor.fails))
        .collect();
    spent_by_apogee(&lit, &burns, apogee_s).map_err(|error| format!("{at}: {error}"))?;
    let mass = assembly.mass_properties_lit(clearance_time_s, &lit);
    let cg_m = -mass.cg_m.z;
    let cp_m = simulation
        .aero()
        .normal_force(&Flow::axial(clearance_mach))
        .map_err(|error| format!("{at}: {error}"))?
        .cp_station_m;
    let reference_m = assembly.layout.reference_diameter_m;
    let margin_cal = cp_m.map(|cp| (cp - cg_m) / reference_m);
    // OpenRocket's rocket can reach the rod-clearance row at an angle of attack, more the more
    // its rod is tilted (M2.2e5), where hpr's margin is taken at none: hpr's center of pressure
    // at OpenRocket's angle sizes what that difference moves.
    let clearance_alpha_rad = clearance["angle_of_attack_rad"]
        .as_f64()
        .filter(|alpha| alpha.is_finite());
    // A diagnostic, so a flow the model refuses leaves it out rather than failing the flight.
    let cp_at_openrocket_alpha_m = clearance_alpha_rad.and_then(|alpha| {
        simulation
            .aero()
            .normal_force(&Flow::new(clearance_mach, alpha, 0.0))
            .ok()
            .and_then(|force| force.cp_station_m)
    });

    let summary = &recorded["summary"];
    let reference_value = |key: &str| {
        if aborted {
            ReferenceReading::Aborted
        } else {
            // OpenRocket's `NaN` is written as JSON null.
            ReferenceReading::Complete(summary[key].as_f64().or(Some(f64::NAN)))
        }
    };
    let references = [
        reference_value("max_altitude_m"),
        reference_value("max_velocity_m_s"),
        if aborted {
            ReferenceReading::Aborted
        } else {
            ReferenceReading::Complete(clearance["stability_cal"].as_f64().or(Some(f64::NAN)))
        },
    ];
    let measured = [apogee_m, peaks.max_speed_m_s, margin_cal];
    let tool = openrocket();
    let mut metrics = serde_json::Map::new();
    for (((metric, key), reference), measured) in METRICS.iter().zip(references).zip(measured) {
        let outcome = compare(&tool, *metric, reference, measured);
        metrics.insert(
            (*key).to_owned(),
            metric_entry(reference, measured, &outcome),
        );
    }

    let deployed_before_apogee_s =
        early_chute(recorded).map_err(|error| format!("{at}: {error}"))?;
    let events = recorded["events"]
        .as_array()
        .ok_or_else(|| format!("{at}: the record has no events"))?;
    // Each tool's separations, in the order they came.
    let openrocket_separations_s: Vec<f64> = events
        .iter()
        .filter(|event| event["type"] == "STAGE_SEPARATION")
        .filter_map(|event| event["time_s"].as_f64())
        .collect();
    let hpr_separations_s: Vec<f64> = result
        .events
        .iter()
        .filter(|event| event.kind == EventKind::Separation)
        .map(|event| event.sample.time_s)
        .collect();
    // Each tool parts the stack as often as the file says, or the times below pair up wrongly.
    if !stagings.is_empty()
        && (openrocket_separations_s.len() != stagings.len()
            || hpr_separations_s.len() != stagings.len())
    {
        return Err(format!(
            "{at}: the file has {} separations under power, OpenRocket recorded {} and hpr flew {}",
            stagings.len(),
            openrocket_separations_s.len(),
            hpr_separations_s.len()
        ));
    }
    // A staged or clustered flight says so, for M1.9c's tolerance (ADR-076): each separation, in
    // the order it fires.
    let separated = (!stagings.is_empty()).then(|| {
        stagings
            .iter()
            .enumerate()
            .map(|(index, staging)| {
                // What the part it drops still had to burn, which its flight as a point leaves
                // out (ADR-172).
                let left_n_s = result
                    .bodies
                    .iter()
                    .find(|body| body.body == index + 1)
                    .map(|body| body.impulse_left_n_s);
                let mut entry = json!({
                    "after_stage": staging.after_stage,
                    "separation_s": {
                        "openrocket": openrocket_separations_s.get(index),
                        "hpr": hpr_separations_s.get(index),
                    },
                });
                if let Some(left_n_s) = left_n_s.filter(|&left_n_s| left_n_s > 0.0) {
                    entry["dropped_impulse_left_n_s"] = json!(left_n_s);
                }
                entry
            })
            .collect::<Vec<_>>()
    });
    // The motors in a mount that places more than one, a motor in each tube of a cluster.
    let clusters: BTreeSet<&str> = assembly
        .motors
        .iter()
        .filter(|motor| motor.tube > 0)
        .map(|motor| motor.mounted.mount.as_str())
        .collect();
    let clustered_motors = assembly
        .motors
        .iter()
        .filter(|motor| clusters.contains(motor.mounted.mount.as_str()))
        .count();
    let mut flown = json!({
        "design": design,
        "configuration": recorded["configuration"],
        "motors": motors,
        "aborted": aborted,
        "rod_length_m": rod_length_m,
        "termination": result.termination,
        "design_errors_accepted": design_errors,
        "deployed_before_apogee_s": deployed_before_apogee_s,
        "max_mach_openrocket": recorded["summary"]["max_mach"],
        "metrics": metrics,
        "at_rod_clearance": {
            "time_s": clearance_time_s,
            "mach": clearance_mach,
            "openrocket": {
                "mass_kg": clearance["mass_kg"],
                "cg_from_nose_m": clearance["cg_from_nose_m"],
                "cp_from_nose_m": clearance["cp_from_nose_m"],
                "reference_length_m": clearance["reference_length_m"],
                "angle_of_attack_rad": clearance["angle_of_attack_rad"],
            },
            "hpr": {
                "mass_kg": mass.mass_kg,
                "cg_from_nose_m": cg_m,
                "cp_from_nose_m": cp_m,
                "reference_length_m": reference_m,
                "cp_from_nose_m_at_openrocket_angle_of_attack": cp_at_openrocket_alpha_m,
            },
        },
        "launch_mass_kg": {
            "openrocket": recorded["series"]["launch_mass_kg"],
            "hpr": assembly.mass_properties_lit(0.0, &lit).mass_kg,
        },
        "rod": {
            "angle_from_vertical_rad": conditions["rod_angle_rad"],
            "direction_rad": conditions["rod_direction_rad"],
        },
        "apogee_position_m": {
            "openrocket": {
                "east": recorded["series"]["east_at_max_altitude_m"],
                "north": recorded["series"]["north_at_max_altitude_m"],
            },
            "hpr": {
                "east": apogee_moved_m.map(|moved| moved.x),
                "north": apogee_moved_m.map(|moved| moved.y),
            },
        },
    });
    if let Some(separated) = separated {
        flown["staging"] = json!(separated);
    }
    if let Some(coast) = coast {
        flown["unpowered_separation"] = coast;
    }
    if clustered_motors > 0 {
        flown["clustered_motors"] = json!(clustered_motors);
    }
    // OpenRocket's own flight tumbling before its apogee leaves rigid-body flight there: its
    // apogee is its tumble model's. The flight says so, with hpr's least margin to apogee in the
    // rocket's weakest plane and when hpr's angle of attack first passed 90° before its apogee,
    // for [`TURNS_OVER`] (ADR-168).
    let event_s = |kind: &str| {
        events
            .iter()
            .filter(|event| event["type"] == kind)
            .filter_map(|event| event["time_s"].as_f64())
            .reduce(f64::min)
    };
    if let (Some(tumble_s), Some(apogee_s)) = (event_s("TUMBLE"), event_s("APOGEE"))
        && tumble_s < apogee_s
    {
        flown["openrocket_tumbles_s"] = json!(tumble_s);
        flown["hpr_least_margin_cal"] = json!(peaks.least_margin_cal);
        flown["hpr_turns_over_s"] = json!(peaks.turns_over_s);
    }
    if let Some(last_row_s) = last_row_s {
        flown["before_abort"] = before_abort(recorded, &result, &peaks, last_row_s)
            .map_err(|error| format!("{at}: {error}"))?;
    }
    if let Some(descended) = descended {
        flown["descent"] = descended?;
    }
    Ok(flown)
}

/// A flight OpenRocket aborted, against hpr's flight of it up to the abort (ADR-172): OpenRocket's
/// cause, and each tool's height above its start and speed at OpenRocket's first separation and
/// at its last row, `last_row_s`. OpenRocket's height at its last row is its largest, which the
/// record holds, so this needs that row to be the highest. At the separation, OpenRocket's
/// state is its row just after it and hpr's its separation event's sample, the whole stack's
/// center of mass at the split (the sustainer's sits about 0.1 m higher on *Pods--powered*).
/// Heights are counted from where each tool's flight starts, as the apogee's are.
fn before_abort(
    recorded: &Value,
    result: &hpr_sim::FlightResult,
    peaks: &Peaks,
    last_row_s: f64,
) -> Result<Value, String> {
    let series = &recorded["series"];
    if series["time_of_max_altitude_s"].as_f64() != Some(last_row_s) {
        return Err("OpenRocket's last row before the abort is not its highest".to_owned());
    }
    let events = recorded["events"]
        .as_array()
        .ok_or("the record has no events")?;
    let abort = events
        .iter()
        .find(|event| event["type"] == "SIM_ABORT")
        .ok_or("the record is aborted with no abort event")?;
    let separation = events
        .iter()
        .find(|event| event["type"] == "STAGE_SEPARATION");
    let start_m = peaks
        .start_height_m
        .ok_or("hpr's flight recorded no step")?;
    let hpr_separation = result.event(EventKind::Separation).map(|event| {
        (
            event.sample.time_s,
            event.sample.height_above_ground_m - start_m,
            event.sample.cg_velocity_enu_m_s.length(),
        )
    });
    let (height_m, speed_m_s) = peaks
        .at
        .ok_or("hpr's flight ended before OpenRocket's last row")?;
    let after = |event: &Value, key: &str| event["after"][key].as_f64();
    let compared = [
        relative(series["max_altitude_m"].as_f64(), Some(height_m)),
        relative(series["last_total_velocity_m_s"].as_f64(), Some(speed_m_s)),
        relative(
            separation.and_then(|event| after(event, "altitude_m")),
            hpr_separation.map(|s| s.1),
        ),
        relative(
            separation.and_then(|event| after(event, "total_velocity_m_s")),
            hpr_separation.map(|s| s.2),
        ),
    ];
    // Each within ABORT_PERCENT of OpenRocket's, and the splits at one time.
    let met = compared.iter().all(|entry| {
        entry["relative_percent"]
            .as_f64()
            .is_some_and(|percent| percent.abs() <= ABORT_PERCENT)
    }) && separation.and_then(|event| event["time_s"].as_f64())
        == hpr_separation.map(|s| s.0);
    let [
        height_m,
        speed_m_s,
        separation_height_m,
        separation_speed_m_s,
    ] = compared;
    Ok(json!({
        "cause": abort["cause"],
        "time_s": last_row_s,
        "height_m": height_m,
        "speed_m_s": speed_m_s,
        "separation": {
            "time_s": {
                "openrocket": separation.map(|event| &event["time_s"]),
                "hpr": hpr_separation.map(|s| s.0),
            },
            "height_m": separation_height_m,
            "speed_m_s": separation_speed_m_s,
        },
        "hpr_turns_over_s": peaks.running_turn_s,
        "met": met,
    }))
}

/// The flights whose separation drops motors still burning, at their stage's first burnout
/// (ADR-172), with the impulse they had left, which the dropped part's flight as a point leaves
/// out. Empty when there are none.
fn dropped_burning_lines(report: &Value) -> String {
    let empty = Vec::new();
    let mut lines = String::new();
    for flight in report["flights"].as_array().unwrap_or(&empty) {
        for (index, split) in flight["staging"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .enumerate()
        {
            if split["dropped_impulse_left_n_s"].is_number() {
                lines.push_str(&format!(
                    "- {} {}: part {} drops at {} s with {} N·s of its motors' impulse left\n",
                    flight["design"].as_str().unwrap_or_default(),
                    flight["motors"].as_str().unwrap_or_default(),
                    index + 1,
                    fixed(&split["separation_s"]["hpr"], 3),
                    fixed(&split["dropped_impulse_left_n_s"], 3),
                ));
            }
        }
    }
    if lines.is_empty() {
        return lines;
    }
    format!(
        "\n## Parts dropped still burning\n\nA stage whose motors sit in several mounts \
         separates at its first burnout, as OpenRocket's does, and drops its other motors still \
         burning (decision [ADR-172][adr-172]). The part flies as a point with no thrust, so its \
         flight leaves out what they had left; the sustainer's flight, compared above, is not \
         touched by it.\n\n{lines}"
    )
}

/// How near hpr's height and speed must come to OpenRocket's, at its separation and at its last
/// row, in a flight OpenRocket aborted, in per cent of OpenRocket's (ADR-172): the band M2.2 gives
/// an apogee before it needs a written cause.
pub(crate) const ABORT_PERCENT: f64 = APOGEE_CAUSE_PERCENT;

/// The flights OpenRocket aborted, against hpr's flight of them up to the abort
/// ([`before_abort`]). Empty when there are none.
fn aborted_table(report: &Value) -> String {
    let empty = Vec::new();
    let flights: Vec<&Value> = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|flight| flight["before_abort"].is_object())
        .collect();
    if flights.is_empty() {
        return String::new();
    }
    let mut out = format!(
        "\n## Flights OpenRocket aborted\n\nOpenRocket stops a flight it can't carry on, such as \
         a stage that tumbles while its motor still burns, and its record then has no apogee, so \
         the first table withholds the comparison. Up to the abort the record is a flight like \
         any other: hpr's height above its start and its speed are compared at OpenRocket's \
         separation (OpenRocket's row just after it, and hpr's sample at its separation event, \
         the whole stack's center of mass at the split) and at OpenRocket's last row, and are met \
         within {ABORT_PERCENT}% \
         of OpenRocket's with the separation at the same time (decision [ADR-172][adr-172]). \
         *Turns over*: when hpr's angle of attack first passes 90°.\n\n\
         [adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md\n\n\
         | design | motors | OR's cause | split, OR / hpr (s) | height at split, OR / hpr (m) | Δ \
         | speed at split, OR / hpr (m/s) | Δ | last row (s) | height, OR / hpr (m) | Δ \
         | speed, OR / hpr (m/s) | Δ | hpr turns over (s) | met |\n\
         |---|---|---|---|---|---:|---|---:|---:|---|---:|---|---:|---:|---|\n",
    );
    for flight in flights {
        let abort = &flight["before_abort"];
        let split = &abort["separation"];
        let pair = |entry: &Value, digits: usize| {
            format!(
                "{} / {}",
                fixed(&entry["openrocket"], digits),
                fixed(&entry["hpr"], digits)
            )
        };
        let delta = |entry: &Value| format!("{}%", signed(&entry["relative_percent"], 2));
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            abort["cause"].as_str().unwrap_or_default(),
            pair(&split["time_s"], 2),
            pair(&split["height_m"], 2),
            delta(&split["height_m"]),
            pair(&split["speed_m_s"], 2),
            delta(&split["speed_m_s"]),
            fixed(&abort["time_s"], 2),
            pair(&abort["height_m"], 1),
            delta(&abort["height_m"]),
            pair(&abort["speed_m_s"], 1),
            delta(&abort["speed_m_s"]),
            fixed(&abort["hpr_turns_over_s"], 2),
            if abort["met"] == true {
                "met"
            } else {
                "missed"
            },
        ));
    }
    out
}

/// What the part that keeps the nose reaches after a lone separation with nothing ahead of it left
/// to burn, coasting from it with no drag, as `simulation` flies it with [`staged`]'s devices (the
/// part tumbles from its own apogee): the flight `hpr::ork::tumbling` refuses (ADR-165), measured
/// against OpenRocket's flight of the configuration with every device set never to open, the
/// record's `undeployed`, whose part flies on its own airframe. Each apogee is counted from where
/// its tool's flight starts, as the climb's is; also each tool's separation time, and whether
/// OpenRocket's came before its own apogee.
fn coasting(
    simulation: Simulation,
    stagings: &[ork::Staging],
    recorded: &Value,
) -> Result<Value, String> {
    let mut start = Peaks::default();
    let result = staged(simulation, stagings)
        .and_then(|simulation| simulation.run(&mut start))
        .map_err(|error| error.to_string())?;
    let apogee = nose_events(&result)
        .into_iter()
        .find(|event| event.kind == EventKind::Apogee)
        .zip(start.start_height_m)
        .map(|(event, start)| event.height_m - start);
    let separation_s = result
        .event(EventKind::Separation)
        .map(|event| event.sample.time_s);
    let empty = Vec::new();
    let events = recorded["events"].as_array().unwrap_or(&empty);
    let first = |kind: &str| {
        events
            .iter()
            .find(|event| event["type"] == kind)
            .and_then(|event| event["time_s"].as_f64())
    };
    let openrocket_separation_s = first("STAGE_SEPARATION");
    Ok(json!({
        "separation_s": { "openrocket": openrocket_separation_s, "hpr": separation_s },
        "before_apogee": openrocket_separation_s
            .zip(first("APOGEE"))
            .map(|(separation, apogee)| separation < apogee),
        "coast_apogee_m": relative(recorded["undeployed"]["max_altitude_m"].as_f64(), apogee),
    }))
}

/// An event of the flight that keeps the nose: a kind, a time, s, a height above the ground, m,
/// a speed over the ground, m/s, and a vertical velocity, m/s, up.
struct NoseEvent {
    kind: EventKind,
    time_s: f64,
    height_m: f64,
    speed_m_s: f64,
    vertical_m_s: f64,
}

/// The events of the flight of the part that keeps the nose, in order: the stack's, then, after a
/// lone separation with nothing left to burn ([`hpr_sim::Termination::Separated`]), those of body
/// 0, which flies on as a point. OpenRocket's record holds that part's branch.
fn nose_events(result: &hpr_sim::FlightResult) -> Vec<NoseEvent> {
    let mut events: Vec<NoseEvent> = result
        .events
        .iter()
        .map(|event| NoseEvent {
            kind: event.kind,
            time_s: event.sample.time_s,
            height_m: event.sample.height_above_ground_m,
            speed_m_s: event.sample.cg_velocity_enu_m_s.length(),
            vertical_m_s: event.sample.cg_velocity_enu_m_s.z,
        })
        .collect();
    if result.termination == hpr_sim::Termination::Separated {
        let nose = result.bodies.iter().filter(|body| body.body == 0);
        events.extend(nose.flat_map(|body| &body.events).map(|event| NoseEvent {
            kind: event.kind,
            time_s: event.sample.time_s,
            height_m: event.sample.height_above_ground_m,
            speed_m_s: event.sample.cg_velocity_enu_m_s.length(),
            vertical_m_s: event.sample.cg_velocity_enu_m_s.z,
        }));
    }
    events
}

/// The largest difference, s, a deployment may have from OpenRocket's and meet ADR-153's target,
/// unless [`DEPLOYMENT_TARGET_FRACTION`] of its time is larger.
pub(crate) const DEPLOYMENT_TARGET_S: f64 = 0.1;

/// The share of OpenRocket's deployment time a deployment may differ by and meet ADR-153's
/// target, where it is larger than [`DEPLOYMENT_TARGET_S`].
pub(crate) const DEPLOYMENT_TARGET_FRACTION: f64 = 0.02;

/// ADR-153's target for the landing speed and the flight time: within this many per cent of
/// OpenRocket's.
pub(crate) const DESCENT_TARGET_PERCENT: f64 = 5.0;

/// Why a flight's descent is not compared: OpenRocket's or hpr's flight separates.
pub(crate) const DESCENT_SEPARATES: &str = "the flight separates";
/// The same: OpenRocket's flight is aborted or never lands.
pub(crate) const DESCENT_NOTHING_RECORDED: &str = "OpenRocket's flight is aborted or does not land";
/// The same: neither OpenRocket's record nor hpr's mapping deploys a device.
pub(crate) const DESCENT_NO_DEVICE: &str = "neither tool deploys a device";
/// The same: hpr's flight with its devices failed, which the public report counts as an error.
pub(crate) const DESCENT_FAILED: &str = "hpr's flight with its devices failed";

/// Every reason a descent is not compared, in words that name no part: the private library's
/// report carries them. The last six are [`refused_why`]'s.
pub(crate) const DESCENT_NOT_COMPARED: [&str; 10] = [
    DESCENT_SEPARATES,
    DESCENT_NOTHING_RECORDED,
    DESCENT_NO_DEVICE,
    DESCENT_FAILED,
    "a device inside a part hpr does not read",
    "a device that opens at a separation",
    "a deployment event hpr does not know",
    "a device the assembled rocket does not hold",
    "a device's number outside its domain",
    "a part's own numbers hpr refuses",
];

/// Why `hpr::ork::recovery` refused a configuration, from [`DESCENT_NOT_COMPARED`].
fn refused_why(refused: &hpr::ork::RecoveryRefused) -> &'static str {
    use hpr::ork::RecoveryRefused as R;
    DESCENT_NOT_COMPARED[match refused {
        R::Unread { .. } => 4,
        R::AtSeparation { .. } => 5,
        R::UnknownEvent { .. } => 6,
        R::NoPart { .. } => 7,
        R::Domain { .. } => 8,
        _ => 9,
    }]
}

/// Where hpr's center of mass comes back down through the height it started at, OpenRocket's
/// zero (its altitude is 0 at launch): the time and the speed there, found on the dense output by
/// bisection. hpr's own ground hit is later by the fall from the center of mass's height on the
/// rail.
#[derive(Default)]
struct Touchdown {
    start_height_m: Option<f64>,
    /// Whether it has been a meter above its start, so the pad's settling is not a landing.
    risen: bool,
    at: Option<(f64, f64)>,
}

impl Observer for Touchdown {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        if self.at.is_some() {
            return Ok(());
        }
        let (start, end) = (step.start_s(), step.end_s());
        let start_height_m = match self.start_height_m {
            Some(height) => height,
            None => *self
                .start_height_m
                .insert(step.sample(start)?.height_above_ground_m),
        };
        let height_at_end = step.sample(end)?.height_above_ground_m;
        if !self.risen {
            self.risen = height_at_end > start_height_m + 1.0;
            return Ok(());
        }
        if height_at_end > start_height_m {
            return Ok(());
        }
        // The step before ended above the start, so the crossing is in this one.
        let (mut above, mut below) = (start, end);
        for _ in 0..64 {
            let middle = 0.5 * (above + below);
            if step.sample(middle)?.height_above_ground_m > start_height_m {
                above = middle;
            } else {
                below = middle;
            }
        }
        let sample = step.sample(below)?;
        self.at = Some((below, sample.cg_velocity_enu_m_s.length()));
        Ok(())
    }
}

/// How much of a deployment's difference from OpenRocket's, s, comes from the apogee and from
/// where OpenRocket's step put an altitude's deployment, for a device fired by `trigger`; `None`
/// for a trigger timed from a motor or the launch, or where a number is missing.
///
/// - **Apogee:** hpr's apogee time less OpenRocket's apogee event, which can come a step after its
///   highest row.
/// - **Altitude:** the time each tool takes to fall from its apogee to its deployment, at
///   OpenRocket's mean speed over that fall. hpr's apogee is `apogee_m` above where it started,
///   `start_m` above the ground, and it deploys at the trigger's height above the ground (the
///   caller passes where hpr opened the device, below the set height by any delay); OpenRocket
///   falls from its highest row's height, at its time, to its deployment's row,
///   `openrocket_height_m`, which its step puts below the set height.
fn expected_difference_s(
    trigger: Trigger,
    (openrocket_apogee_event_s, openrocket_top_s, openrocket_top_m): (
        Option<f64>,
        Option<f64>,
        Option<f64>,
    ),
    (apogee_s, apogee_m, start_m): (Option<f64>, Option<f64>, Option<f64>),
    (openrocket_s, openrocket_height_m): (f64, Option<f64>),
) -> Option<f64> {
    match trigger {
        Trigger::Apogee => Some(apogee_s? - openrocket_apogee_event_s?),
        Trigger::Altitude {
            height_above_ground_m,
        } => {
            let openrocket_fall_m = openrocket_top_m? - openrocket_height_m?;
            let speed = openrocket_fall_m / (openrocket_s - openrocket_top_s?);
            let hpr_fall_m = apogee_m? + start_m? - height_above_ground_m;
            let expected = apogee_s? - openrocket_top_s? + (hpr_fall_m - openrocket_fall_m) / speed;
            expected.is_finite().then_some(expected)
        }
        _ => None,
    }
}

/// `entry`'s comparison `{openrocket, hpr, relative_percent}`.
fn relative(openrocket: Option<f64>, hpr: Option<f64>) -> Value {
    let percent = openrocket
        .zip(hpr)
        .filter(|(reference, _)| *reference != 0.0)
        .map(|(reference, value)| 100.0 * (value - reference) / reference);
    json!({ "openrocket": openrocket, "hpr": hpr, "relative_percent": percent })
}

/// The configuration `simulation` flies, flown with the `.ork` file's `recovery` mapped by
/// `hpr::ork::recovery`, against OpenRocket's `recorded` flight (ADR-153): each deployment's
/// time in order, the apogee, and the speed and time where the center of mass comes back through
/// its starting height ([`Touchdown`]). A flight it does not compare says why in `not_flown`.
fn descent_flight(
    simulation: Simulation,
    recovery: &ork::Recovery,
    rocket: &hpr_design::Rocket,
    stagings: &[ork::Staging],
    recorded: &Value,
) -> Value {
    let not_flown = |why: &str| json!({ "not_flown": why });
    let empty = Vec::new();
    let events = recorded["events"].as_array().unwrap_or(&empty);
    let times = |kind: &str| -> Vec<f64> {
        events
            .iter()
            .filter(|event| event["type"] == kind)
            .filter_map(|event| event["time_s"].as_f64())
            .collect()
    };
    // A separation hpr flies is one of the file's: powered, or a lone one with nothing left to
    // burn (ADR-165); any other OpenRocket flew isn't hpr's.
    if stagings.is_empty() && !times("STAGE_SEPARATION").is_empty() {
        return not_flown(DESCENT_SEPARATES);
    }
    let summary = &recorded["summary"];
    let openrocket_deployments = times("RECOVERY_DEVICE_DEPLOYMENT");
    if recorded["aborted"] == true || times("GROUND_HIT").is_empty() {
        return not_flown(DESCENT_NOTHING_RECORDED);
    }
    // With separations, each device on its part and the tumbles `hpr sim` adds (M4.5g1 to g3);
    // the record and the touchdown are those of the part that keeps the nose.
    let assembly = simulation.assembly();
    let separations = match hpr::ork::separations(stagings, assembly) {
        Ok(separations) => separations,
        Err(error) => {
            eprintln!("{DESCENT_FAILED}: {error}");
            return not_flown(DESCENT_FAILED);
        }
    };
    let recovered = if separations.is_empty() {
        hpr::ork::recovery(recovery, rocket, assembly)
    } else {
        hpr::ork::separated_recovery(recovery, rocket, assembly, &separations)
    };
    let recovered = match recovered {
        Ok(recovered) => recovered,
        Err(refused) => return not_flown(refused_why(&refused)),
    };
    // Where neither tool flies a device there is no descent to compare; where one does and the
    // other doesn't, the counts differ and the flight misses.
    if recovered.devices.is_empty() && openrocket_deployments.is_empty() {
        return not_flown(DESCENT_NO_DEVICE);
    }
    let triggers: Vec<Trigger> = recovered.devices.iter().map(|d| d.trigger).collect();
    let devices = triggers.len();
    let mut touchdown = Touchdown::default();
    let result = match simulation
        .with_recovery(recovered.devices)
        .and_then(|simulation| simulation.with_separations(separations))
        .and_then(|simulation| simulation.run(&mut touchdown))
    {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{DESCENT_FAILED}: {error}");
            return not_flown(DESCENT_FAILED);
        }
    };
    let start_m = touchdown.start_height_m;
    // The part that keeps the nose: after a split with nothing left to burn it flies on as body 0,
    // whose flight has events only (ADR-165).
    let nose = nose_events(&result);
    let apogee = nose.iter().find(|event| event.kind == EventKind::Apogee);
    let apogee_m = apogee
        .zip(start_m)
        .map(|(event, start)| event.height_m - start);
    let apogee_s = apogee.map(|event| event.time_s);
    // The speed at the last deployment, which OpenRocket's summary interpolates at its event.
    let last_deployment_speed = nose
        .iter()
        .filter(|event| matches!(event.kind, EventKind::Deployment(_)))
        .max_by(|a, b| a.time_s.total_cmp(&b.time_s))
        .map(|event| event.speed_m_s);
    // Each deployment's time, the trigger that fired it and the height above the ground it opened
    // at, in time order.
    let mut hpr_deployments: Vec<(f64, Trigger, f64)> = nose
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::Deployment(index) => {
                Some((event.time_s, *triggers.get(index)?, event.height_m))
            }
            _ => None,
        })
        .collect();
    // Body 0's landing, back through the height it started at: its flight has no dense output, so
    // the crossing is its ground hit less that height over its descent rate, which a canopy holds
    // steady over the last fraction of a meter.
    if touchdown.at.is_none()
        && let Some((ground, start)) = nose
            .iter()
            .rev()
            .find(|event| event.kind == EventKind::GroundHit)
            .zip(start_m)
        && result.termination == hpr_sim::Termination::Separated
        && ground.vertical_m_s < 0.0
    {
        touchdown.at = Some((
            ground.time_s + start / ground.vertical_m_s,
            ground.speed_m_s,
        ));
    }
    hpr_deployments.sort_by(|a, b| a.0.total_cmp(&b.0));
    let deployed_at = |kind: &str, time: f64| {
        events
            .iter()
            .find(|event| event["type"] == kind && event["time_s"].as_f64() == Some(time))
    };
    let openrocket_apogee = (
        times("APOGEE").first().copied(),
        recorded["series"]["time_of_max_altitude_s"].as_f64(),
        summary["max_altitude_m"].as_f64(),
    );
    let deployments: Vec<Value> = openrocket_deployments
        .iter()
        .zip(&hpr_deployments)
        .map(|(&openrocket, &(hpr, trigger, hpr_height_m))| {
            let difference = hpr - openrocket;
            let allowed = DEPLOYMENT_TARGET_S.max(DEPLOYMENT_TARGET_FRACTION * openrocket);
            let openrocket_height_m = deployed_at("RECOVERY_DEVICE_DEPLOYMENT", openrocket)
                .and_then(|event| event["after"]["altitude_m"].as_f64());
            // A height's deployment is sized from where hpr opened it, which a delay puts below
            // the set height.
            let sizing = match trigger {
                Trigger::Altitude { .. } => Trigger::Altitude {
                    height_above_ground_m: hpr_height_m,
                },
                other => other,
            };
            let expected = expected_difference_s(
                sizing,
                openrocket_apogee,
                (apogee_s, apogee_m, start_m),
                (openrocket, openrocket_height_m),
            );
            let residual = expected.map(|expected| difference - expected);
            json!({
                "openrocket_s": openrocket,
                "hpr_s": hpr,
                "difference_s": difference,
                "within_target": difference.abs() <= allowed,
                "trigger": match trigger {
                    Trigger::Apogee => "apogee",
                    Trigger::Altitude { .. } => "altitude",
                    Trigger::MotorDelay { .. } => "ejection",
                    _ => "time",
                },
                "openrocket_height_m": openrocket_height_m,
                "hpr_height_m": hpr_height_m,
                "set_height_m": match trigger {
                    Trigger::Altitude {
                        height_above_ground_m,
                    } => Some(height_above_ground_m),
                    _ => None,
                },
                "expected_difference_s": expected,
                "residual_s": residual,
                "residual_within_target": residual.map(|residual| residual.abs() <= allowed),
            })
        })
        .collect();
    let landing_speed = relative(
        summary["ground_hit_velocity_m_s"].as_f64(),
        touchdown.at.map(|(_, speed)| speed),
    );
    let flight_time = relative(
        summary["flight_time_s"].as_f64(),
        touchdown.at.map(|(time, _)| time),
    );
    let within = |entry: &Value| {
        entry["relative_percent"]
            .as_f64()
            .is_some_and(|percent| percent.abs() <= DESCENT_TARGET_PERCENT)
    };
    let counted = openrocket_deployments.len() == hpr_deployments.len();
    let meets_targets = counted
        && deployments.iter().all(|d| d["within_target"] == true)
        && within(&landing_speed)
        && within(&flight_time);
    // The same, each deployment that misses held instead by what is left once the apogee and
    // OpenRocket's step are taken out of it.
    let meets_targets_once_sized = counted
        && deployments
            .iter()
            .all(|d| d["within_target"] == true || d["residual_within_target"] == true)
        && within(&landing_speed)
        && within(&flight_time);
    let apogee_above = |trigger: &Trigger| match (trigger, apogee_m.zip(start_m)) {
        (
            Trigger::Altitude {
                height_above_ground_m,
            },
            Some((apogee, start)),
        ) => *height_above_ground_m > apogee + start,
        _ => false,
    };
    json!({
        "devices": devices,
        "not_deployed": recovered.not_deployed.len(),
        "deployment_count": {
            "openrocket": openrocket_deployments.len(),
            "hpr": hpr_deployments.len(),
        },
        "deployments": deployments,
        "deploy_height_above_apogee": triggers.iter().any(apogee_above),
        "apogee_m": relative(summary["max_altitude_m"].as_f64(), apogee_m),
        "apogee_s": { "openrocket": openrocket_apogee.1, "hpr": apogee_s },
        "start_height_m": start_m,
        "last_deployment_speed_m_s": relative(
            summary["deployment_velocity_m_s"].as_f64(),
            last_deployment_speed,
        ),
        "landing_speed_m_s": landing_speed,
        "flight_time_s": flight_time,
        "meets_targets": meets_targets,
        "meets_targets_once_sized": meets_targets_once_sized,
    })
}

/// How long before the apogee of the same flight with nothing deployed OpenRocket's first
/// parachute opened, when it opened before the flight's own apogee: the early parachute moves that
/// apogee, not the undeployed one.
fn early_chute(recorded: &Value) -> Result<Option<f64>, String> {
    let empty = Vec::new();
    let events = recorded["events"].as_array().unwrap_or(&empty);
    let first = |kind: &str| {
        events
            .iter()
            .find(|event| event["type"] == kind)
            .and_then(|event| event["time_s"].as_f64())
    };
    match (first("RECOVERY_DEVICE_DEPLOYMENT"), first("APOGEE")) {
        (Some(deployed), Some(apogee)) if deployed < apogee => {
            recorded["undeployed"]["apogee_time_s"]
                .as_f64()
                .map(|undeployed| Some(undeployed - deployed))
                .ok_or_else(|| "the record has no apogee without deployment".to_owned())
        }
        _ => Ok(None),
    }
}

/// OpenRocket's apogee with a flight's named causes taken out of its own flight, and hpr's apogee
/// against it (M2.2e4, decision ADR-073), or `None` for an aborted flight, a flight of hpr's with
/// no apogee (its outcome is scored already), or one with neither cause. `overrides` are the
/// drag coefficients the design states that hpr does not apply: the rocket's own (ADR-167).
///
/// - `parachutes_held`: the record's `undeployed` flight, the same one with nothing deployed. A
///   parachute that opens before apogee lowers the apogee, and hpr flies none from a `.ork`.
/// - `drag_overrides_cleared_too`, where the rocket states a drag coefficient: its
///   `undeployed_without_drag_overrides` flight, with nothing deployed and every stated coefficient
///   cleared. That is OpenRocket flying what hpr flies only when the rocket's is the only one
///   stated, since hpr applies a part's and a stage's: the record must then name the same elements
///   as cleared as hpr reads (one with no id by count only), or the report stops.
///
/// Each is OpenRocket's apogee and hpr's difference in per cent of it, or why there is none (the
/// oracle failed, or OpenRocket refused or aborted the flight). The flight with all the causes
/// taken out is the second where there is one, else the first. `remaining_percent` is hpr's
/// difference from it, what the named causes leave; `within_5_percent` holds when there is one
/// and it is within 5%. A flight still more than 5% off after an early parachute is held is flown
/// again by hpr on OpenRocket's drag, which [`on_openrocket_s_drag_held`] adds here once flown.
fn causes_removed(
    recorded: &Value,
    entry: &Value,
    overrides: &[DragOverride],
) -> Result<Option<Value>, String> {
    let early = !entry["deployed_before_apogee_s"].is_null();
    if entry["aborted"] == true || (!early && overrides.is_empty()) {
        return Ok(None);
    }
    let Some(hpr) = entry["metrics"]["apogee_m"]["hpr"].as_f64() else {
        return Ok(None);
    };
    // A part with no id in the file gets a new one in OpenRocket, so it is matched by count only.
    let ids: BTreeSet<String> = overrides
        .iter()
        .filter(|o| !o.id.is_empty())
        .map(|o| o.id.to_lowercase())
        .collect();
    let step = |key: &str, parts: Option<&str>, measured: f64| -> Result<Value, String> {
        let held = &recorded[key];
        if held.is_null() {
            return Err(format!("the record has no {key} flight"));
        }
        if let Some(error) = held["driver_error"].as_str() {
            return Ok(json!({ "why": format!("the oracle failed: {error}") }));
        }
        if let Some(verb) = parts {
            let changed: BTreeSet<String> = held[verb]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_lowercase)
                .collect();
            if changed.len() != overrides.len() || !ids.is_subset(&changed) {
                return Err(format!(
                    "OpenRocket's {key} flight {verb} {} parts, hpr reads {}, and {} of hpr's \
                     ids are not among them",
                    changed.len(),
                    overrides.len(),
                    ids.difference(&changed).count()
                ));
            }
        }
        if !held["refused"].is_null() {
            return Ok(json!({ "why": "OpenRocket refused it" }));
        }
        if held["aborted"] != false {
            return Ok(json!({ "why": "OpenRocket aborted it" }));
        }
        let openrocket = held["max_altitude_m"]
            .as_f64()
            .filter(|m| m.is_finite() && *m > 0.0)
            .ok_or_else(|| format!("the record's {key} flight has no apogee"))?;
        Ok(json!({
            "openrocket_m": openrocket,
            "relative_percent": 100.0 * (measured - openrocket) / openrocket,
        }))
    };
    let held = step("undeployed", None, hpr)?;
    let mut remaining = held["relative_percent"].as_f64();
    let mut sized = json!({ "parachutes_held": held });
    if !overrides.is_empty() {
        let too = step("undeployed_without_drag_overrides", Some("cleared"), hpr)?;
        remaining = too["relative_percent"].as_f64();
        sized["drag_overrides_cleared_too"] = too;
    }
    sized["remaining_percent"] = json!(remaining);
    sized["within_5_percent"] = json!(remaining.is_some_and(|p| p.abs() <= APOGEE_CAUSE_PERCENT));
    Ok(Some(sized))
}

/// The reference tool.
pub(crate) fn openrocket() -> Tool {
    Tool::OpenRocket {
        version: OPENROCKET_MEASURED.to_owned(),
    }
}

/// One metric's entry: both values, the outcome, and the difference.
pub(crate) fn metric_entry(
    reference: ReferenceReading,
    measured: Option<f64>,
    outcome: &MetricOutcome,
) -> Value {
    let reference = match reference {
        ReferenceReading::Complete(value) => value.filter(|v| v.is_finite()),
        _ => None,
    };
    let measured = measured.filter(|v| v.is_finite());
    let difference = outcome.difference();
    let relative_percent = match (difference, reference) {
        (Some(difference), Some(reference)) if reference != 0.0 => {
            Some(100.0 * difference / reference)
        }
        _ => None,
    };
    json!({
        "openrocket": reference,
        "hpr": measured,
        "outcome": outcome,
        "difference": difference,
        "relative_percent": relative_percent,
    })
}

/// The counts and the spread of each metric's differences.
pub(crate) fn summarise(flights: &[Value], not_flown: &[Value]) -> Value {
    let mut metrics = serde_json::Map::new();
    for (metric, key) in METRICS {
        let mut outcomes: BTreeMap<String, usize> = BTreeMap::new();
        let mut groups: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for flight in flights {
            let entry = &flight["metrics"][key];
            let outcome = entry["outcome"]["outcome"].as_str().unwrap_or("missing");
            *outcomes.entry(outcome.to_owned()).or_default() += 1;
            // The margin's difference is in calibres; the others' in per cent of OpenRocket's.
            let value = if metric == FlightMetric::RodClearanceStability {
                entry["difference"].as_f64()
            } else {
                entry["relative_percent"].as_f64()
            };
            if let Some(value) = value {
                groups.entry(cause(flight, metric)).or_default().push(value);
            }
        }
        let unit = if metric == FlightMetric::RodClearanceStability {
            "calibres"
        } else {
            "percent of OpenRocket's"
        };
        let groups: serde_json::Map<String, Value> = groups
            .iter()
            .map(|(cause, values)| ((*cause).to_owned(), spread(values)))
            .collect();
        metrics.insert(
            key.to_owned(),
            json!({
                "definition": definition(&openrocket(), metric),
                "outcomes": outcomes,
                "difference_unit": unit,
                "scored_by_cause": groups,
            }),
        );
    }
    let over = flights
        .iter()
        .filter(|f| {
            f["metrics"]["apogee_m"]["relative_percent"]
                .as_f64()
                .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT)
        })
        .count();
    json!({
        "flown": flights.len(),
        "not_flown": not_flown.len(),
        "apogee_over_5_percent": over,
        "metrics": metrics,
        "apogee_with_the_causes_removed": with_the_causes_removed(flights),
        "mass_and_cg": mass_and_cg(flights),
        "descent": descent_summary(&flights.iter().map(descent_row).collect::<Vec<_>>()),
    })
}

/// What the summary of the descents reads of a flight's `descent` block (ADR-153), the same
/// for the public flights and the private library's rows: whether it was compared (or why not),
/// whether its apogee has no named cause but an early parachute, which the descent flies (`in
/// scope`), the landing speed and flight time in per cent of OpenRocket's, and which targets it
/// meets. A flight with no block, a staged one, reads as not compared.
pub(crate) fn descent_row(flight: &Value) -> Value {
    let descent = &flight["descent"];
    if let Some(why) = descent["not_flown"].as_str() {
        return json!({ "outcome": why });
    }
    if descent.is_null() {
        return json!({ "outcome": DESCENT_SEPARATES });
    }
    let deployments = descent["deployments"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let counted = descent["deployment_count"]["openrocket"] == descent["deployment_count"]["hpr"];
    json!({
        "outcome": "compared",
        "in_scope": matches!(cause(flight, FlightMetric::Apogee), NO_NAMED_CAUSE | EARLY_CHUTE),
        "landing_speed_percent": descent["landing_speed_m_s"]["relative_percent"],
        "flight_time_percent": descent["flight_time_s"]["relative_percent"],
        "deployments_within": counted
            && deployments.iter().all(|d| d["within_target"] == true),
        "meets_targets": descent["meets_targets"],
        "meets_targets_once_sized": descent["meets_targets_once_sized"],
    })
}

/// The descents' summary from [`descent_row`]s: how many were compared and why the rest were
/// not; over the flights in scope and apart, the spread of the landing speed and the flight time
/// and how many meet ADR-153's targets, as written and once each deployment that misses is held
/// by its residual instead.
pub(crate) fn descent_summary(rows: &[Value]) -> Value {
    let mut not_compared: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: BTreeMap<&str, Vec<&Value>> = BTreeMap::new();
    for row in rows {
        match row["outcome"].as_str() {
            Some("compared") => groups
                .entry(if row["in_scope"] == true {
                    "in_scope"
                } else {
                    "apart"
                })
                .or_default()
                .push(row),
            other => {
                *not_compared
                    .entry(other.unwrap_or("missing").to_owned())
                    .or_default() += 1
            }
        }
    }
    let group = |rows: &[&Value]| {
        let values =
            |key: &str| -> Vec<f64> { rows.iter().filter_map(|row| row[key].as_f64()).collect() };
        let count = |key: &str| rows.iter().filter(|row| row[key] == true).count();
        json!({
            "flights": rows.len(),
            "landing_speed_percent": spread(&values("landing_speed_percent")),
            "flight_time_percent": spread(&values("flight_time_percent")),
            "deployments_within": count("deployments_within"),
            "meeting_targets": count("meets_targets"),
            "meeting_targets_once_sized": count("meets_targets_once_sized"),
        })
    };
    json!({
        "targets": {
            "deployment_s": DEPLOYMENT_TARGET_S,
            "deployment_fraction": DEPLOYMENT_TARGET_FRACTION,
            "landing_speed_and_flight_time_percent": DESCENT_TARGET_PERCENT,
        },
        "compared": groups.values().map(Vec::len).sum::<usize>(),
        "not_compared": not_compared,
        "in_scope": group(groups.get("in_scope").map_or(&[][..], Vec::as_slice)),
        "apart": group(groups.get("apart").map_or(&[][..], Vec::as_slice)),
    })
}

/// The descents section of the page: what it compares, the summary, and a row per flight compared
/// (ADR-153).
fn descents_table(report: &Value) -> String {
    let empty = Vec::new();
    let mut out = String::from(
        "\n## Descents\n\nEach configuration flown again with the file's parachutes and streamers, \
         as `hpr::ork::recovery` maps them and OpenRocket flies them (decision [ADR-153][adr-153]): \
         the drag of the open devices only, each opening at once. A flight hpr separates is \
         compared by the part that keeps the nose, the branch OpenRocket's record keeps; one \
         OpenRocket separates where hpr does not is not compared. Each deployment is matched to OpenRocket's in time order; the landing is where \
         hpr's center of mass comes back through the height it started at, OpenRocket's zero. The \
         targets, set before measuring: each deployment within 0.1 s of OpenRocket's or 2% of its \
         time, whichever is larger; the landing speed and the flight time within 5%. A deployment \
         timed from the apogee or a height carries the apogee's difference in it, and OpenRocket \
         opens a device set to a height at the end of its step, below that height: the *left* \
         column is the difference less those two, computed from the record and hpr's flight. \
         Times are seconds from launch, OpenRocket's then hpr's.\n\n\
         [adr-153]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0153-a-orks-recovery-flown-as-openrocket-flies-it.md\n\n",
    );
    out.push_str(&descent_lines(&report["summary"]["descent"]));
    out.push_str(
        "\n| design | motors | deployments, OR / hpr (s) | left (s) | landing speed OR (m/s) | \
         hpr | Δ | flight time OR (s) | hpr | Δ | targets |\n\
         |---|---|---|---|---:|---:|---:|---:|---:|---:|---|\n",
    );
    for flight in report["flights"].as_array().unwrap_or(&empty) {
        let descent = &flight["descent"];
        let Some(deployments) = descent["deployments"].as_array() else {
            continue;
        };
        let times: Vec<String> = deployments
            .iter()
            .map(|d| {
                format!(
                    "{} / {}{}",
                    fixed(&d["openrocket_s"], 2),
                    fixed(&d["hpr_s"], 2),
                    if d["within_target"] == true {
                        ""
                    } else {
                        " (misses)"
                    }
                )
            })
            .collect();
        let left: Vec<String> = deployments
            .iter()
            .map(|d| {
                if d["residual_s"].is_null() {
                    "–".to_owned()
                } else {
                    signed(&d["residual_s"], 2)
                }
            })
            .collect();
        let delta = |entry: &Value| format!("{}%", signed(&entry["relative_percent"], 2));
        let targets = if descent["meets_targets"] == true {
            "met"
        } else if descent["meets_targets_once_sized"] == true {
            "met once sized"
        } else {
            "missed"
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            times.join("; "),
            left.join("; "),
            fixed(&descent["landing_speed_m_s"]["openrocket"], 2),
            fixed(&descent["landing_speed_m_s"]["hpr"], 2),
            delta(&descent["landing_speed_m_s"]),
            fixed(&descent["flight_time_s"]["openrocket"], 1),
            fixed(&descent["flight_time_s"]["hpr"], 1),
            delta(&descent["flight_time_s"]),
            targets,
        ));
    }
    out
}

/// The configurations whose stack comes apart with nothing ahead of the split left to burn
/// (ADR-165): the apogee of the part that keeps the nose, flown with its devices, and what it would
/// reach coasting from the split with no drag ([`coasting`]). Empty when there are none.
fn unpowered_table(report: &Value) -> String {
    let empty = Vec::new();
    let flights: Vec<&Value> = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|flight| flight["unpowered_separation"].is_object())
        .collect();
    if flights.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "\n## Separations with nothing left to burn\n\nA configuration whose stack comes apart with \
         nothing ahead of the split left to burn, such as a payload dropped at its booster's \
         ejection charge, flies its climb whole in the first table. Flown again with its devices, \
         each part flies on from the split as a point with only its own devices' drag, as \
         OpenRocket flies a descent, and only when the part that keeps the nose has a device of \
         its own open by then (decision [ADR-165][adr-165]). OpenRocket's record holds that \
         part's branch, so its apogee is compared here, from where each tool's flight starts, \
         and its descent in the table above. *Coasting*: the same part flown on from the split \
         with no drag until its own apogee, the flight `hpr sim` refuses, against OpenRocket's \
         flight of the configuration with no device deployed, where the part flies on its own \
         airframe; shown where OpenRocket's split came before its apogee.\n\n\
         [adr-165]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md\n\n\
         | design | motors | split, OR / hpr (s) | before OR's apogee | apogee OR (m) | hpr | Δ \
         | coasting: OR, nothing deployed (m) | hpr, no drag | Δ |\n\
         |---|---|---|---|---:|---:|---:|---:|---:|---:|\n",
    );
    for flight in flights {
        let split = &flight["unpowered_separation"];
        let apogee = &flight["descent"]["apogee_m"];
        let before = split["before_apogee"] == true;
        let coast = &split["coast_apogee_m"];
        let (coast_or, coast_hpr, coast_delta) = if before {
            (
                fixed(&coast["openrocket"], 1),
                fixed(&coast["hpr"], 1),
                format!("{}%", signed(&coast["relative_percent"], 2)),
            )
        } else {
            ("–".to_owned(), "–".to_owned(), "–".to_owned())
        };
        out.push_str(&format!(
            "| {} | {} | {} / {} | {} | {} | {} | {}% | {} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            fixed(&split["separation_s"]["openrocket"], 2),
            fixed(&split["separation_s"]["hpr"], 2),
            if before { "yes" } else { "no" },
            fixed(&apogee["openrocket"], 1),
            fixed(&apogee["hpr"], 1),
            signed(&apogee["relative_percent"], 2),
            coast_or,
            coast_hpr,
            coast_delta,
        ));
    }
    out
}

/// The descents' summary as lines for a page (ADR-153).
pub(crate) fn descent_lines(summary: &Value) -> String {
    let spread_text = |entry: &Value| {
        if entry["count"].as_u64().unwrap_or(0) == 0 {
            "none".to_owned()
        } else {
            format!(
                "median {}%, mean absolute {}%, from {}% to {}%",
                signed(&entry["median"], 2),
                fixed(&entry["mean_absolute"], 2),
                signed(&entry["min"], 2),
                signed(&entry["max"], 2),
            )
        }
    };
    let mut out = String::new();
    let not_compared = summary["not_compared"].as_object();
    let reasons: Vec<String> = not_compared
        .into_iter()
        .flatten()
        .map(|(why, count)| format!("{why}: {count}"))
        .collect();
    out.push_str(&format!(
        "- descents compared: {}; not compared: {}\n",
        summary["compared"],
        if reasons.is_empty() {
            "none".to_owned()
        } else {
            reasons.join("; ")
        },
    ));
    for (key, words) in [
        (
            "in_scope",
            "with no named cause for the apogee but an early parachute",
        ),
        ("apart", "with a named cause for the apogee"),
    ] {
        let group = &summary[key];
        if group["flights"].as_u64().unwrap_or(0) == 0 {
            continue;
        }
        out.push_str(&format!(
            "- {} {words}: landing speed {}; flight time {}; every deployment within its target \
             in {}; meeting every target {}, and {} once each deployment that misses is held by \
             what is left of it\n",
            group["flights"],
            spread_text(&group["landing_speed_percent"]),
            spread_text(&group["flight_time_percent"]),
            group["deployments_within"],
            group["meeting_targets"],
            group["meeting_targets_once_sized"],
        ));
    }
    out
}

/// The spread of what the named causes leave of hpr's apogee difference, over the flights that
/// have one (M2.2e4); and, of the flights more than 5% from OpenRocket's, how many have their
/// causes sized (a difference from a flight without them), how many come within 5% of every
/// flight of OpenRocket's without them, and how many of the rest come within 5% of OpenRocket's
/// flight with nothing deployed when hpr flies OpenRocket's drag ([`on_openrocket_s_drag_held`]):
/// what their early parachute leaves is then in the drag coefficient (ADR-097, ADR-167).
fn with_the_causes_removed(flights: &[Value]) -> Value {
    let (mut remaining, mut over, mut sized, mut within, mut on_drag) = (Vec::new(), 0, 0, 0, 0);
    for flight in flights {
        let removed = &flight["apogee_with_the_causes_removed"];
        remaining.extend(removed["remaining_percent"].as_f64());
        if flight["metrics"]["apogee_m"]["relative_percent"]
            .as_f64()
            .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT)
        {
            over += 1;
            sized += usize::from(removed["remaining_percent"].is_f64());
            let after = removed["within_5_percent"] == true;
            within += usize::from(after);
            on_drag += usize::from(
                !after && drag_sizes(removed[OPENROCKET_DRAG_PROBE]["relative_percent"].as_f64()),
            );
        }
    }
    json!({
        "remaining_percent": spread(&remaining),
        "over_5_percent": over,
        "over_5_percent_sized": sized,
        "over_5_percent_within_5_percent_after": within,
        "over_5_percent_within_5_percent_on_openrocket_s_drag": on_drag,
    })
}

/// The spread of hpr's mass and center of mass against OpenRocket's (M2.2e1), over the flights
/// that were not aborted: the mass at launch and at the rod-clearance step in per cent of
/// OpenRocket's, and the center of mass at that step, hpr's less OpenRocket's distance from the
/// nose, in OpenRocket's calibres (so it moves the margin by as much, the other way).
fn mass_and_cg(flights: &[Value]) -> Value {
    let (mut launch, mut clearance, mut cg) = (Vec::new(), Vec::new(), Vec::new());
    for flight in flights.iter().filter(|f| f["aborted"] != true) {
        let [l, c, g] = mass_and_cg_differences(flight);
        launch.extend(l);
        clearance.extend(c);
        cg.extend(g);
    }
    json!({
        "launch_mass_percent": spread(&launch),
        "rod_clearance_mass_percent": spread(&clearance),
        "rod_clearance_cg_cal": spread(&cg),
    })
}

/// One flight's differences [`mass_and_cg`] spreads: its mass at launch and at the rod-clearance
/// step in per cent of OpenRocket's, and its center of mass at that step in OpenRocket's calibres.
pub(crate) fn mass_and_cg_differences(flight: &Value) -> [Option<f64>; 3] {
    let relative = |hpr: &Value, openrocket: &Value| {
        hpr.as_f64()
            .zip(openrocket.as_f64())
            .map(|(h, o)| 100.0 * (h - o) / o)
            .filter(|p| p.is_finite())
    };
    let at = &flight["at_rod_clearance"];
    let (hpr, openrocket) = (&at["hpr"], &at["openrocket"]);
    [
        relative(
            &flight["launch_mass_kg"]["hpr"],
            &flight["launch_mass_kg"]["openrocket"],
        ),
        relative(&hpr["mass_kg"], &openrocket["mass_kg"]),
        hpr["cg_from_nose_m"]
            .as_f64()
            .zip(openrocket["cg_from_nose_m"].as_f64())
            .zip(openrocket["reference_length_m"].as_f64())
            .map(|((h, o), reference)| (h - o) / reference)
            .filter(|cal| cal.is_finite()),
    ]
}

/// A drag coefficient stated on the rocket itself, which hpr does not apply: `hpr-design` has
/// no place for it. A part's and a stage's fly (ADR-167).
pub(crate) const DRAG_OVERRIDE: &str = "the rocket's own drag override not applied";

/// A reference parachute open before its apogee.
pub(crate) const EARLY_CHUTE: &str = "reference parachute open before apogee";

/// The rocket turns over before apogee: OpenRocket's own flight tumbles before its apogee, hpr's
/// least static margin to apogee, in the rocket's weakest plane, is negative, and hpr's angle of
/// attack passes 90° before its apogee (ADR-168). Both tools then fly a rocket that is unstable
/// in some plane, and where it turns over depends on what first tips it: OpenRocket's calm,
/// vertical flight keeps its angle of attack at zero until it tumbles, hpr's turns over as the
/// Earth's rotation tips it, so at another site it may not turn over at all.
pub(crate) const TURNS_OVER: &str = "the rocket turns over before apogee in both tools";

/// hpr's drag coefficient, not OpenRocket's: flown on OpenRocket's, hpr's apogee comes within 5%
/// of OpenRocket's (ADR-097).
pub(crate) const OWN_DRAG: &str = "hpr's own drag coefficient";

/// None of these.
pub(crate) const NO_NAMED_CAUSE: &str = "no named cause";

/// Where a flight holds hpr's flight again with only OpenRocket's base drag under power: its
/// apogee and largest speed against OpenRocket's.
pub(crate) const WHOLE_BASE_PROBE: &str = "with_the_whole_base_under_power";

/// Where a flight holds hpr's flight again on OpenRocket's drag coefficient along OpenRocket's
/// flight ([`DRAG_CURVES`]): its apogee and largest speed against OpenRocket's.
pub(crate) const OPENROCKET_DRAG_PROBE: &str = "on_openrocket_s_drag";

/// The named cause a flight's difference in `metric` is summarised under. A drag override moves
/// every metric but the margin; an early parachute only the apogee. A flight with both is put
/// under the drag override. A flight with neither whose apogee (or largest speed) hpr brings
/// within 5% of OpenRocket's by flying OpenRocket's drag has hpr's own drag as that metric's
/// cause. That says only that the gap is in the drag: which part of it, and whether hpr's or
/// OpenRocket's is right, needs its own written breakdown (ADR-097).
///
/// A flight with an early parachute names no drag cause, even where hpr flew it on OpenRocket's
/// drag ([`drag_probed`]): that flight sizes what the parachute leaves of the apogee, in
/// `apogee_with_the_causes_removed`, and a cause named for its largest speed would need a
/// written breakdown of its own (ADR-097).
pub(crate) fn cause(flight: &Value, metric: FlightMetric) -> &'static str {
    let early = !flight["deployed_before_apogee_s"].is_null();
    if metric == FlightMetric::RodClearanceStability {
        NO_NAMED_CAUSE
    } else if turns_over(flight) {
        TURNS_OVER
    } else if !flight["drag_overrides_not_applied"].is_null() {
        DRAG_OVERRIDE
    } else if metric == FlightMetric::Apogee && early {
        EARLY_CHUTE
    } else if !early
        && drag_sizes(
            flight[OPENROCKET_DRAG_PROBE][if metric == FlightMetric::Apogee {
                "apogee_m"
            } else {
                "max_speed_m_s"
            }]["relative_percent"]
                .as_f64(),
        )
    {
        OWN_DRAG
    } else {
        NO_NAMED_CAUSE
    }
}

/// Whether `flight` turns over before apogee in both tools ([`TURNS_OVER`]): OpenRocket's record
/// tumbles before its apogee, hpr's least margin to apogee is negative, and hpr's angle of attack
/// passes 90° before its apogee.
pub(crate) fn turns_over(flight: &Value) -> bool {
    flight["openrocket_tumbles_s"].is_number()
        && flight["hpr_least_margin_cal"]
            .as_f64()
            .is_some_and(|cal| cal < 0.0)
        && flight["hpr_turns_over_s"].is_number()
}

/// Whether hpr's apogee or largest speed on OpenRocket's drag, `percent` from OpenRocket's, is
/// within the bar that sizes a cause.
pub(crate) fn drag_sizes(percent: Option<f64>) -> bool {
    percent.is_some_and(|p| p.abs() <= APOGEE_CAUSE_PERCENT)
}

/// Whether hpr flies `entry` again on OpenRocket's drag and with the whole base under power
/// (ADR-097): its apogee is more than 5% from OpenRocket's in a flight not aborted, and either it
/// has no named cause, or its cause is an early parachute and OpenRocket's own flight with nothing
/// deployed is still more than 5% from hpr's (`within_5_percent` false). Both of hpr's flights
/// fly nothing deployed, so the second compares like with like: within 5% on OpenRocket's drag,
/// what the parachute leaves is in the drag coefficient. Before the parts' drag overrides flew
/// (ADR-167), no flight left that way had OpenRocket's drag to size it.
pub(crate) fn drag_probed(entry: &Value) -> bool {
    let off = entry["metrics"]["apogee_m"]["relative_percent"]
        .as_f64()
        .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT);
    off && entry["aborted"] != true
        && match cause(entry, FlightMetric::Apogee) {
            NO_NAMED_CAUSE => true,
            EARLY_CHUTE => entry["apogee_with_the_causes_removed"]["within_5_percent"] == false,
            _ => false,
        }
}

/// hpr's apogee on OpenRocket's drag against OpenRocket's apogee with nothing deployed, for a
/// flight [`drag_probed`] flew for its early parachute (ADR-097, ADR-167): `hpr_m`,
/// `openrocket_m` and hpr's difference in per cent of OpenRocket's, or why there is none. `None`
/// for any other flight.
pub(crate) fn on_openrocket_s_drag_held(entry: &Value) -> Option<Value> {
    if cause(entry, FlightMetric::Apogee) != EARLY_CHUTE || !drag_probed(entry) {
        return None;
    }
    let probe = &entry[OPENROCKET_DRAG_PROBE];
    let held = &entry["apogee_with_the_causes_removed"]["parachutes_held"];
    Some(if probe["failed"] == true {
        json!({ "why": "hpr failed to fly it" })
    } else if probe.is_null() {
        json!({ "why": "no curve of OpenRocket's to fly" })
    } else if let Some(percent) = probe_percent(entry, OPENROCKET_DRAG_PROBE, "apogee_m") {
        json!({
            "hpr_m": probe["apogee_m"]["hpr"],
            "openrocket_m": held["openrocket_m"],
            "relative_percent": percent,
        })
    } else {
        json!({ "why": "no apogee to compare" })
    })
}

/// A drag probe's (`probe`) difference in the metric `key` (`apogee_m` or `max_speed_m_s`), in
/// per cent of OpenRocket's. The probes fly nothing deployed, so for a flight with an early
/// parachute the apogee is taken against OpenRocket's flight with nothing deployed, like with
/// like; anything else against the flight compared.
pub(crate) fn probe_percent(flight: &Value, probe: &str, key: &str) -> Option<f64> {
    let probe = &flight[probe];
    if key == "apogee_m" && !flight["deployed_before_apogee_s"].is_null() {
        let held = &flight["apogee_with_the_causes_removed"]["parachutes_held"]["openrocket_m"];
        probe["apogee_m"]["hpr"]
            .as_f64()
            .zip(held.as_f64())
            .map(|(hpr, openrocket)| 100.0 * (hpr - openrocket) / openrocket)
            .filter(|p| p.is_finite())
    } else {
        probe[key]["relative_percent"].as_f64()
    }
}

/// How many, the median, the mean, and the smallest and largest of `values`.
pub(crate) fn spread(values: &[f64]) -> Value {
    if values.is_empty() {
        return json!({ "count": 0 });
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let median = if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    };
    let mean_abs = sorted.iter().map(|v| v.abs()).sum::<f64>() / n as f64;
    json!({
        "count": n,
        "median": median,
        "mean_absolute": mean_abs,
        "min": sorted[0],
        "max": sorted[n - 1],
    })
}

/// The report as a page.
pub(crate) fn page(report: &Value) -> String {
    let mut out = String::new();
    out.push_str("# hpr against OpenRocket 24.12's flights of the public designs\n\n");
    out.push_str(
        "Written by `cargo xtask ork-flights` ([M2.2d2][m2-2d2], hpr's flights against \
         OpenRocket's; decision [ADR-069][adr-069]) from OpenRocket 24.12's calm-air flights in \
         `validation/fixtures/ork/openrocket-flights.json` ([M2.2d1][m2-2d1]). OR is \
         OpenRocket. Each metric is taken by the definition hpr holds for OpenRocket 24.12: \
         the apogee is the highest point above the launch position; the largest speed is the \
         peak speed (hpr's up to its apogee, since the flight compared for the climb flies no \
         parachute: its descent is compared apart, under *Descents*); the margin, in \
         calibres, is OpenRocket's stability column at its rod-clearance step, which hpr takes \
         at that step's time and Mach number with the air along the axis. Differences (Δ) are \
         hpr less OpenRocket. Motors are OpenRocket's configuration names, one bracketed group \
         per stage. The explanation is on the [documentation site][site].\n\n\
         [m2-2d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2d2\n\
         [m2-2d1]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2d1\n\
         [m2-2e1]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e1\n\
         [adr-069]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-069-hprs-flights-of-the-public-designs-against-openrockets-2026-09-25\n\
         [adr-070]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-070-m22e-split-mass-and-centre-of-mass-first-then-the-corpus-2026-09-25\n\
         [site]: https://hpr.fusionspace.co/format/ork.html#the-simulators-flights-against-openrockets\n\n",
    );
    out.push_str(&summary_lines(report));
    out.push('\n');
    out.push_str(
        "| design | motors | apogee OR (m) | hpr (m) | Δ | max speed OR (m/s) | hpr (m/s) | Δ \
         | max Mach OR | margin OR (cal) | hpr (cal) | Δ (cal) |\n",
    );
    out.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    let empty = Vec::new();
    for flight in report["flights"].as_array().unwrap_or(&empty) {
        let m = &flight["metrics"];
        let early = flight["deployed_before_apogee_s"]
            .as_f64()
            .map_or(String::new(), |s| format!(" (chute {s:.2} s early)"))
            + &drag_probes(flight, "apogee_m");
        let speed_probe = drag_probes(flight, "max_speed_m_s");
        out.push_str(&format!(
            "| {} | {} | {} | {} | {}{} | {} | {} | {}{} | {} | {} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            fixed(&m["apogee_m"]["openrocket"], 1),
            fixed(&m["apogee_m"]["hpr"], 1),
            percent(&m["apogee_m"]),
            early,
            fixed(&m["max_speed_m_s"]["openrocket"], 2),
            fixed(&m["max_speed_m_s"]["hpr"], 2),
            percent(&m["max_speed_m_s"]),
            speed_probe,
            fixed(&flight["max_mach_openrocket"], 3),
            fixed(&m["rod_clearance_margin_cal"]["openrocket"], 3),
            fixed(&m["rod_clearance_margin_cal"]["hpr"], 3),
            signed(&m["rod_clearance_margin_cal"]["difference"], 4),
        ));
    }
    out.push_str(
        "\n*Chute s early*: OpenRocket's parachute opened that long before the apogee of the \
         same flight with nothing deployed, which the record also holds; the flight compared for \
         the climb flies no parachute.\n",
    );
    if report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .any(|f| f[OPENROCKET_DRAG_PROBE].is_object() || f[WHOLE_BASE_PROBE].is_object())
    {
        out.push_str(
            "\n*On OpenRocket's drag*: the same flight by hpr on OpenRocket's drag coefficient \
             along OpenRocket's own flight (power on while a motor burns, power off after). \
             *With the whole base under power*: hpr's own drag, but with OpenRocket's base drag \
             under power, the whole base's while a motor burns, where hpr takes the motor's \
             cross-section off it. hpr flies both for an apogee more than 5% off with no other \
             named cause. Within 5% on OpenRocket's drag, the metric's cause is *hpr's own drag \
             coefficient*: the net gap is in the drag, though parts of it could cancel, and it \
             does not say which drag is right, so each such flight has a written breakdown \
             ([ADR-097][adr-097]). hpr flies both too for an apogee more than 5% off whose early \
             parachute, held, still leaves more than 5% ([ADR-167][adr-167]); both fly nothing \
             deployed, so that flight's apogee brackets are against OpenRocket's flight with \
             nothing deployed, and they name no cause: the table of named causes below sizes \
             what the parachute leaves with them.\n\n\
             [adr-097]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-097-a-cause-in-the-drag-sized-by-hpr-flying-openrockets-drag-2026-09-28\n\
             [adr-167]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md\n",
        );
    }
    out.push_str(&causes_table(report));
    out.push_str(
        "\nAt the rod-clearance step, the parts of the margin (m from the nose tip, and kg):\n\n",
    );
    out.push_str(
        "| design | motors | CG OR | CG hpr | CP OR | CP hpr | reference OR | reference hpr \
         | mass OR | mass hpr |\n",
    );
    out.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for flight in report["flights"].as_array().unwrap_or(&empty) {
        let at = &flight["at_rod_clearance"];
        let (or, hpr) = (&at["openrocket"], &at["hpr"]);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            fixed(&or["cg_from_nose_m"], 4),
            fixed(&hpr["cg_from_nose_m"], 4),
            fixed(&or["cp_from_nose_m"], 4),
            fixed(&hpr["cp_from_nose_m"], 4),
            fixed(&or["reference_length_m"], 4),
            fixed(&hpr["reference_length_m"], 4),
            fixed(&or["mass_kg"], 4),
            fixed(&hpr["mass_kg"], 4),
        ));
    }
    let checked: Vec<&Value> = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|f| {
            f["design_errors_accepted"]
                .as_array()
                .is_some_and(|e| !e.is_empty())
        })
        .collect();
    if !checked.is_empty() {
        out.push_str(
            "\nWhat hpr's design checks object to, in flights flown anyway as OpenRocket flies \
             them:\n\n| design | motors | findings |\n|---|---|---|\n",
        );
        for flight in checked {
            out.push_str(&format!(
                "| {} | {} | `{}` |\n",
                flight["design"].as_str().unwrap_or_default(),
                flight["motors"].as_str().unwrap_or_default(),
                flight["design_errors_accepted"],
            ));
        }
    }
    out.push_str(&descents_table(report));
    out.push_str(&unpowered_table(report));
    out.push_str(&aborted_table(report));
    out.push_str(&dropped_burning_lines(report));
    let not_flown = report["not_flown"].as_array().unwrap_or(&empty);
    if !not_flown.is_empty() {
        out.push_str("\nConfigurations OpenRocket flew that hpr does not fly yet:\n\n");
        out.push_str("| design | motors | why |\n|---|---|---|\n");
        for entry in not_flown {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                entry["design"].as_str().unwrap_or_default(),
                entry["motors"].as_str().unwrap_or_default(),
                entry["why"].as_str().unwrap_or_default(),
            ));
        }
    }
    out.push_str(&probes_table(report));
    out.push_str(&rod_probes_table(report));
    out
}

/// The pod probes' table (M1.13c2): each probe against OpenRocket, and what its pods change against
/// the airframe alone in each code.
fn probes_table(report: &Value) -> String {
    let probes: Vec<&Value> = report["probes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|probe| {
            probe["file"]
                .as_str()
                .is_some_and(|file| file.starts_with(POD_PROBES))
        })
        .collect();
    if probes.is_empty() {
        return String::new();
    }
    let without = probes
        .iter()
        .find(|probe| probe["design"] == WITHOUT_PODS)
        .copied()
        .unwrap_or(&Value::Null);
    let mut out = String::from(
        "\n## Pod probes\n\nSmall designs written to test pods ([M1.13c2][m1-13c2]), not counted \
         among the designs above: one airframe on an AeroTech H128W, carrying pods of bodies, \
         fins, a tail cone, winglets or motors, and once with none (`pods-none`, first). *Pods' \
         change* is the apogee's and the margin's change from `pods-none` in each code. \
         `pods-motors-2` flies an H128W in each of its two pods and none in the airframe, with \
         0.35 kg of nose ballast, not 0.15, so its change is not its pods' alone.\n\n\
         [m1-13c2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-13c2\n\n\
         | probe | apogee OR (m) | hpr (m) | Δ | pods' change OR | hpr | max speed OR (m/s) \
         | hpr (m/s) | Δ | max Mach OR | margin OR (cal) | hpr (cal) | pods' change OR (cal) \
         | hpr (cal) |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    let first = probes
        .iter()
        .filter(|probe| probe["design"] == WITHOUT_PODS);
    let others = probes
        .iter()
        .filter(|probe| probe["design"] != WITHOUT_PODS);
    for probe in first.chain(others) {
        let m = &probe["metrics"];
        let change = probe_change(probe, without).map_or_else(
            || {
                [
                    "-".to_owned(),
                    "-".to_owned(),
                    "-".to_owned(),
                    "-".to_owned(),
                ]
            },
            |[apogee, margin]| {
                [
                    format!("{:+.2}%", apogee[0]),
                    format!("{:+.2}%", apogee[1]),
                    format!("{:+.4}", margin[0]),
                    format!("{:+.4}", margin[1]),
                ]
            },
        );
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            probe["design"].as_str().unwrap_or_default(),
            fixed(&m["apogee_m"]["openrocket"], 1),
            fixed(&m["apogee_m"]["hpr"], 1),
            percent(&m["apogee_m"]),
            change[0],
            change[1],
            fixed(&m["max_speed_m_s"]["openrocket"], 2),
            fixed(&m["max_speed_m_s"]["hpr"], 2),
            percent(&m["max_speed_m_s"]),
            fixed(&probe["max_mach_openrocket"], 3),
            fixed(&m["rod_clearance_margin_cal"]["openrocket"], 3),
            fixed(&m["rod_clearance_margin_cal"]["hpr"], 3),
            change[2],
            change[3],
        ));
    }
    out
}

/// hpr's margin at rod clearance with its center of pressure at OpenRocket's angle of attack
/// there, in calibres of hpr's reference diameter: `None` when either is missing.
pub(crate) fn margin_at_openrocket_alpha_cal(flight: &Value) -> Option<f64> {
    let hpr = &flight["at_rod_clearance"]["hpr"];
    let cp = hpr["cp_from_nose_m_at_openrocket_angle_of_attack"].as_f64()?;
    Some((cp - hpr["cg_from_nose_m"].as_f64()?) / hpr["reference_length_m"].as_f64()?)
}

/// `value` to one decimal, with no sign on a value that rounds to zero.
fn tenths(value: f64) -> String {
    // Adding zero turns the `-0.0` a small negative value rounds to into `0.0`.
    format!("{:.1}", (value * 10.0).round() / 10.0 + 0.0)
}

/// The compass bearing of a place `east` and `north` of the pad, in degrees from 0 up to 360.
pub(crate) fn bearing_deg(east: f64, north: f64) -> f64 {
    let bearing = east.atan2(north).to_degrees().rem_euclid(360.0);
    // `rem_euclid` rounds a bearing a hair below 0 up to 360 itself.
    if bearing >= 360.0 { 0.0 } else { bearing }
}

/// The tilted-rod probes' table (M2.2e5): the pod probes' airframe from a vertical rod
/// ([`WITHOUT_PODS`]) and from each tilted rod, against OpenRocket, with what the rod changes and
/// where each code's rocket is at apogee.
fn rod_probes_table(report: &Value) -> String {
    let all = report["probes"].as_array().into_iter().flatten();
    let without = all
        .clone()
        .find(|probe| probe["design"] == WITHOUT_PODS)
        .unwrap_or(&Value::Null);
    let rods: Vec<&Value> = all
        .filter(|probe| {
            probe["file"]
                .as_str()
                .is_some_and(|file| file.starts_with(ROD_PROBES))
        })
        .collect();
    // `fly_all` refuses probes without the control, so neither is ever missing here.
    if rods.is_empty() || without.is_null() {
        return String::new();
    }
    let mut out = String::from(
        "\n## Tilted-rod probes\n\nThe pod probes' airframe with no pods, launched from a 1 m \
         rod tilted from the vertical toward a compass bearing ([M2.2e5][m2-2e5]), in calm air, \
         and not counted among the designs above:\n\n\
         - `pods-none`, first, is the same airframe from OpenRocket's default vertical rod: the \
         control. Its bearing means nothing: from a vertical rod both codes' rockets drift about \
         0.4 m west, from Earth's rotation.\n\
         - *Rod's change* is the apogee's change from `pods-none` in each code.\n\
         - *At apogee* is where the rocket is then, meters east and north of where it started, \
         and *bearing* the direction of that place from the pad, clockwise from north: a rod \
         read the wrong way round would send hpr's rocket another way than OpenRocket's.\n\
         - *α OR* is the angle of attack OpenRocket's rocket has at its rod-clearance row, which \
         grows with the tilt, where hpr's margin is taken at none; *at OR's α* is hpr's margin \
         at OpenRocket's angle, which says how much of the margins' difference that accounts \
         for.\n\n\
         [m2-2e5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e5\n\n\
         | probe | rod | apogee OR (m) | hpr (m) | Δ | rod's change OR | hpr \
         | max speed OR (m/s) | hpr (m/s) | Δ | α OR (°) | margin OR (cal) | hpr (cal) \
         | at OR's α (cal) | at apogee OR (E, N m) | hpr (E, N m) | bearing OR | hpr |\n\
         |---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:\
         |---:|\n",
    );
    for probe in std::iter::once(without).chain(rods) {
        let m = &probe["metrics"];
        let rod = &probe["rod"];
        let angle = rod["angle_from_vertical_rad"].as_f64().unwrap_or(f64::NAN);
        let direction = rod["direction_rad"].as_f64().unwrap_or(f64::NAN);
        let rod = if angle == 0.0 {
            "vertical".to_owned()
        } else {
            let bearing = direction.to_degrees();
            let names = [
                "north",
                "north-east",
                "east",
                "south-east",
                "south",
                "south-west",
                "west",
                "north-west",
            ];
            // A whole eighth of the compass is named; 8 eighths is north again.
            let eighth = (bearing / 45.0).rem_euclid(8.0);
            let name = (0_u8..)
                .zip(names)
                .find(|(at, _)| {
                    let off = (eighth - f64::from(*at)).abs();
                    off < 1e-9 || (*at == 0 && (8.0 - eighth).abs() < 1e-9)
                })
                .map_or_else(String::new, |(_, name)| format!(" ({name})"));
            format!("{:.0}° toward {bearing:.0}°{name}", angle.to_degrees())
        };
        let change = |at: usize| {
            probe_change(probe, without).map_or_else(
                || "-".to_owned(),
                |[apogee, _]| format!("{:+.2}%", apogee[at]),
            )
        };
        let place = |code: &str| {
            let at = &probe["apogee_position_m"][code];
            match (at["east"].as_f64(), at["north"].as_f64()) {
                (Some(east), Some(north)) => (
                    format!("{}, {}", tenths(east), tenths(north)),
                    format!("{:.1}°", bearing_deg(east, north)),
                ),
                _ => ("-".to_owned(), "-".to_owned()),
            }
        };
        let (at_or, bearing_or) = place("openrocket");
        let (at_hpr, bearing_hpr) = place("hpr");
        let clearance = &probe["at_rod_clearance"];
        let alpha_deg = clearance["openrocket"]["angle_of_attack_rad"]
            .as_f64()
            .map(f64::to_degrees);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} \
             | {} |\n",
            probe["design"].as_str().unwrap_or_default(),
            rod,
            fixed(&m["apogee_m"]["openrocket"], 1),
            fixed(&m["apogee_m"]["hpr"], 1),
            percent(&m["apogee_m"]),
            change(0),
            change(1),
            fixed(&m["max_speed_m_s"]["openrocket"], 2),
            fixed(&m["max_speed_m_s"]["hpr"], 2),
            percent(&m["max_speed_m_s"]),
            fixed(&json!(alpha_deg), 3),
            fixed(&m["rod_clearance_margin_cal"]["openrocket"], 3),
            fixed(&m["rod_clearance_margin_cal"]["hpr"], 3),
            fixed(&json!(margin_at_openrocket_alpha_cal(probe)), 3),
            at_or,
            at_hpr,
            bearing_or,
            bearing_hpr,
        ));
    }
    out
}

/// The table of each named cause's size: every flight with one, and OpenRocket's own flight flown
/// again without it (M2.2e4, decision ADR-073).
fn causes_table(report: &Value) -> String {
    let empty = Vec::new();
    let sized: Vec<&Value> = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|f| f["apogee_with_the_causes_removed"].is_object())
        .collect();
    if sized.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "\nThe named causes, sized ([M2.2e4][m2-2e4], decision [ADR-073][adr-073]). OpenRocket \
         flew each flight with a named cause again without it. *Nothing deployed*: the same \
         flight with no parachute opening. *Drag settings cleared too*: also with every stated \
         drag coefficient cleared, so each part has the drag of its shape, for a design whose \
         rocket states its own coefficient, which hpr does not apply; a part's and a stage's \
         fly ([ADR-167][adr-167-sized]). Each Δ is hpr's apogee less that flight's, in per cent of it. \
         *Within 5% after*, for an apogee more than 5% off: whether OpenRocket's flight with all \
         its causes taken out is within 5% of hpr's. *hpr on OR's drag, nothing deployed*: where \
         an early parachute, held, still leaves more than 5%, hpr's apogee flown on OpenRocket's \
         drag coefficient along OpenRocket's own flight, nothing deployed in either, and its Δ \
         from OpenRocket's flight with nothing deployed; within 5%, what the parachute leaves is \
         in the drag coefficient, not which drag is right ([ADR-097][adr-097-sized]).\n\n\
         | design | motors | Δ apogee | chute early (s) | OR, nothing deployed (m) | Δ \
         | OR, drag settings cleared too (m) | Δ | within 5% after \
         | hpr on OR's drag, nothing deployed (m) | Δ |\n\
         |---|---|---:|---:|---:|---:|---:|---:|---|---:|---:|\n",
    );
    // A flight OpenRocket could not fly shows why instead of its apogee.
    let apogee = |step: &Value| {
        step["why"]
            .as_str()
            .map_or_else(|| fixed(&step["openrocket_m"], 1), str::to_owned)
    };
    let relative = |step: &Value| {
        step["relative_percent"]
            .as_f64()
            .map_or_else(|| "n/a".to_owned(), |p| format!("{p:+.2}%"))
    };
    for flight in sized {
        let removed = &flight["apogee_with_the_causes_removed"];
        let (held, too) = (
            &removed["parachutes_held"],
            &removed["drag_overrides_cleared_too"],
        );
        let on_drag = &removed[OPENROCKET_DRAG_PROBE];
        let over = flight["metrics"]["apogee_m"]["relative_percent"]
            .as_f64()
            .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT);
        let within = match (over, removed["within_5_percent"].as_bool()) {
            (true, Some(true)) => "yes",
            (true, _) => "no",
            (false, _) => "n/a",
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {within} | {} | {} |\n",
            flight["design"].as_str().unwrap_or_default(),
            flight["motors"].as_str().unwrap_or_default(),
            percent(&flight["metrics"]["apogee_m"]),
            fixed(&flight["deployed_before_apogee_s"], 2),
            apogee(held),
            relative(held),
            if too.is_null() {
                "n/a".to_owned()
            } else {
                apogee(too)
            },
            relative(too),
            on_drag["why"]
                .as_str()
                .map_or_else(|| fixed(&on_drag["hpr_m"], 1), str::to_owned),
            relative(on_drag),
        ));
    }
    out.push_str(
        "\n[m2-2e4]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e4\n\
         [adr-073]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-073-each-named-cause-sized-by-openrockets-own-flight-without-it-2026-09-25\n\
         [adr-097-sized]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-097-a-cause-in-the-drag-sized-by-hpr-flying-openrockets-drag-2026-09-28\n\
         [adr-167-sized]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md\n",
    );
    out
}

/// The brackets after a flight's apogee or largest-speed difference (`key`) giving hpr's drag
/// probes, where the flight holds them ([`probe_percent`]: a flight with an early parachute has
/// its apogee's against OpenRocket's flight with nothing deployed): a probe hpr failed to fly says
/// so, as the library's report does, and a flight probed with no drag curve of OpenRocket's to fly
/// says it has none.
fn drag_probes(flight: &Value, key: &str) -> String {
    let probed = flight[WHOLE_BASE_PROBE].is_object();
    [
        (OPENROCKET_DRAG_PROBE, "on OpenRocket's drag"),
        (WHOLE_BASE_PROBE, "with the whole base under power"),
    ]
    .iter()
    .map(|&(probe_key, label)| {
        let probe = &flight[probe_key];
        if probe["failed"] == true {
            format!(" (failed {label})")
        } else if probe.is_null() && probed {
            format!(" (no curve to fly {label})")
        } else {
            probe_percent(flight, probe_key, key)
                .map_or(String::new(), |p| format!(" ({p:+.2}% {label})"))
        }
    })
    .collect()
}

/// The summary, as lines for the terminal and the page.
pub(crate) fn summary_lines(report: &Value) -> String {
    let summary = &report["summary"];
    let metrics = &summary["metrics"];
    let line = |label: &str, spread: &Value, unit: &str, digits: usize| {
        format!(
            "- {label}: {} scored, median {}{unit}, mean absolute {}{unit}, from {}{unit} to {}{unit}\n",
            spread["count"],
            signed(&spread["median"], digits),
            fixed(&spread["mean_absolute"], digits),
            signed(&spread["min"], digits),
            signed(&spread["max"], digits),
        )
    };
    let mut out = format!(
        "- configurations flown: {} ({} the record holds are not flown by hpr); apogee more than \
         5% from OpenRocket's: {}\n",
        summary["flown"], summary["not_flown"], summary["apogee_over_5_percent"],
    );
    let empty = Vec::new();
    let fastest = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|f| f["max_mach_openrocket"].is_f64())
        .max_by(|a, b| {
            let mach = |f: &Value| f["max_mach_openrocket"].as_f64().unwrap_or(0.0);
            mach(a).total_cmp(&mach(b))
        });
    if let Some(fastest) = fastest {
        out.push_str(&format!(
            "- fastest: {} {}, OpenRocket's largest Mach number {}\n",
            fastest["design"].as_str().unwrap_or_default(),
            fastest["motors"].as_str().unwrap_or_default(),
            fixed(&fastest["max_mach_openrocket"], 3),
        ));
    }
    for (key, label, unit, digits) in [
        ("apogee_m", "apogee", "%", 2),
        ("max_speed_m_s", "largest speed", "%", 2),
        (
            "rod_clearance_margin_cal",
            "margin at rod clearance",
            " cal",
            4,
        ),
    ] {
        let empty = serde_json::Map::new();
        let groups = metrics[key]["scored_by_cause"]
            .as_object()
            .unwrap_or(&empty);
        for (cause, spread) in groups {
            out.push_str(&line(&format!("{label}, {cause}"), spread, unit, digits));
        }
    }
    let sized = &summary["apogee_with_the_causes_removed"];
    if sized.is_object() {
        out.push_str(&line(
            "apogee against OpenRocket's own flights with the named causes removed, over the \
             flights with a named cause, each counting its largest difference from them",
            &sized["remaining_percent"],
            "%",
            2,
        ));
        out.push_str(&format!(
            "- apogees more than 5% off: {}, {} with their named causes sized; within 5% of every \
             flight of OpenRocket's without the causes: {} of {}\n",
            sized["over_5_percent"],
            sized["over_5_percent_sized"],
            sized["over_5_percent_within_5_percent_after"],
            sized["over_5_percent"],
        ));
        let on_drag = &sized["over_5_percent_within_5_percent_on_openrocket_s_drag"];
        if on_drag.as_u64().is_some_and(|n| n > 0) {
            out.push_str(&format!(
                "- of those still more than 5% off with their causes removed, within 5% of \
                 OpenRocket's flight with nothing deployed when hpr flies OpenRocket's drag, and so \
                 put down to the drag coefficient: {on_drag}\n"
            ));
        }
    }
    let own_drag = report["flights"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|f| {
            f["metrics"]["apogee_m"]["relative_percent"]
                .as_f64()
                .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT)
                && cause(f, FlightMetric::Apogee) == OWN_DRAG
        })
        .count();
    if own_drag > 0 {
        out.push_str(&format!(
            "- apogees more than 5% off sized instead by hpr flying OpenRocket's drag, within 5% on \
             it, and so put down to hpr's own drag coefficient: {own_drag}\n"
        ));
    }
    let mass_and_cg = &summary["mass_and_cg"];
    out.push_str(
        "\nhpr's mass and center of mass less OpenRocket's ([M2.2e1][m2-2e1], decision \
         [ADR-070][adr-070]), over the flights not aborted: the masses in per cent of \
         OpenRocket's, the center of mass in OpenRocket's calibres, positive when hpr's is further \
         aft, which shortens the margin by as much.\n\n",
    );
    for (key, label, unit, digits) in [
        ("launch_mass_percent", "mass at launch", "%", 3),
        (
            "rod_clearance_mass_percent",
            "mass at rod clearance",
            "%",
            3,
        ),
        (
            "rod_clearance_cg_cal",
            "center of mass at rod clearance",
            " cal",
            4,
        ),
    ] {
        out.push_str(&line(label, &mass_and_cg[key], unit, digits).replacen(
            " scored,",
            " compared,",
            1,
        ));
    }
    out
}

pub(crate) fn fixed(value: &Value, digits: usize) -> String {
    value
        .as_f64()
        .map_or_else(|| "n/a".to_owned(), |v| format!("{v:.digits$}"))
}

pub(crate) fn signed(value: &Value, digits: usize) -> String {
    value
        .as_f64()
        .map_or_else(|| "n/a".to_owned(), |v| format!("{v:+.digits$}"))
}

fn percent(entry: &Value) -> String {
    match entry["relative_percent"].as_f64() {
        Some(p) => format!("{p:+.2}%"),
        None => entry["outcome"]["outcome"]
            .as_str()
            .unwrap_or("n/a")
            .to_owned(),
    }
}

/// Collects where `now` departs from `committed`: numbers beyond [`CHECK_RELATIVE`], anything
/// else that differs.
pub(crate) fn same(committed: &Value, now: &Value, at: &str, apart: &mut Vec<String>) {
    match (committed, now) {
        (Value::Number(a), Value::Number(b)) => {
            let (a, b) = (
                a.as_f64().unwrap_or(f64::NAN),
                b.as_f64().unwrap_or(f64::NAN),
            );
            if (a - b).abs() > CHECK_RELATIVE * a.abs().max(b.abs()) {
                apart.push(format!("{at}: {a} then, {b} now"));
            }
        }
        (Value::Object(a), Value::Object(b)) => {
            for key in a.keys().chain(b.keys().filter(|k| !a.contains_key(*k))) {
                same(
                    a.get(key).unwrap_or(&Value::Null),
                    b.get(key).unwrap_or(&Value::Null),
                    &format!("{at}/{key}"),
                    apart,
                );
            }
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (index, (a, b)) in a.iter().zip(b).enumerate() {
                same(a, b, &format!("{at}/{index}"), apart);
            }
        }
        (a, b) if a != b => apart.push(format!("{at}: {a} then, {b} now")),
        _ => {}
    }
}

/// Checks that every motor that lights is spent by hpr's apogee at `apogee_s`, from each motor's
/// ignition time (`lit`) and its burn time and whether it fails (`burns`).
///
/// hpr-io leaves a separation at apogee or on the way down to the descent and flies the
/// configuration whole (ADR-076 §4), which holds only if every motor is spent by then. The check
/// is made of every flight, staged or not, since a motor still burning at apogee would also make
/// the compared climb something other than a climb. A motor that lights must have a time: a
/// `.ork` configuration never lights one at a separation.
fn spent_by_apogee(
    lit: &[Option<f64>],
    burns: &[(f64, bool)],
    apogee_s: f64,
) -> Result<(), String> {
    for (index, (lit_s, (burn_s, fails))) in lit.iter().zip(burns).enumerate() {
        match lit_s {
            Some(lit_s) if lit_s + burn_s > apogee_s => {
                return Err(format!(
                    "motor {index} burns until {} s, after hpr's apogee at {apogee_s} s",
                    lit_s + burn_s
                ));
            }
            Some(_) => {}
            None if *fails => {}
            None => {
                return Err(format!(
                    "motor {index} lights at no time known before the flight"
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    /// A flight turns over in both tools only with all three signs: OpenRocket's record tumbling
    /// before its apogee, hpr's least margin to apogee negative, and hpr's own angle of attack
    /// passing 90° before its apogee (ADR-168). Each one missing leaves the cause unnamed.
    #[test]
    fn a_flight_turns_over_only_when_both_tools_flights_do() {
        let both = json!({
            "openrocket_tumbles_s": 2.84, "hpr_least_margin_cal": -4.2, "hpr_turns_over_s": 1.9,
        });
        assert!(turns_over(&both));
        assert_eq!(cause(&both, FlightMetric::Apogee), TURNS_OVER);
        assert_eq!(
            cause(&both, FlightMetric::RodClearanceStability),
            NO_NAMED_CAUSE
        );
        for (key, value) in [
            ("openrocket_tumbles_s", Value::Null),
            ("hpr_least_margin_cal", json!(0.5)),
            ("hpr_least_margin_cal", json!(0.0)),
            ("hpr_least_margin_cal", Value::Null),
            ("hpr_turns_over_s", Value::Null),
        ] {
            let mut flight = both.clone();
            flight[key] = value.clone();
            assert!(!turns_over(&flight), "{key} {value}");
            assert_eq!(cause(&flight, FlightMetric::Apogee), NO_NAMED_CAUSE);
        }
    }

    #[test]
    fn a_flight_is_reported_only_with_every_lit_motor_spent_by_apogee() {
        let burns = [(3.0, false), (2.0, false), (1.0, true)];
        // Lit at launch and at 4 s: spent at 3 s and 6 s, before an apogee at 10 s; the failing
        // tube never lights.
        assert_eq!(
            super::spent_by_apogee(&[Some(0.0), Some(4.0), None], &burns, 10.0),
            Ok(())
        );
        // The second lit at 9 s burns to 11 s, past it.
        let late = super::spent_by_apogee(&[Some(0.0), Some(9.0), None], &burns, 10.0);
        assert!(late.is_err_and(|e| e.contains("motor 1 burns until 11 s")));
        // A motor that lights with no known time.
        let unknown = super::spent_by_apogee(&[Some(0.0), None, None], &burns, 10.0);
        assert!(unknown.is_err_and(|e| e.contains("motor 1 lights at no time known")));
    }

    use super::*;

    fn committed() -> (Value, Value) {
        let root = crate::ork::root().unwrap();
        (
            read_json(&root.join(RECORD)).unwrap(),
            read_json(&root.join(REPORT_JSON)).unwrap(),
        )
    }

    #[test]
    fn the_public_drag_curves_are_the_record_s_examples_by_the_current_scripts() {
        // `drag_curves.py` flew the jar's examples, unpacked, with the scripts as they are now:
        // each design is one the record flew, by digest, and the jar is the record's.
        let root = crate::ork::root().unwrap();
        let (record, report) = committed();
        let curves = read_json(&root.join(PUBLIC_DRAG_CURVES)).unwrap();
        assert_eq!(curves["jar_sha256"], record["jar_sha256"]);
        for (name, path) in [
            (
                "drag_curves.py",
                "validation/oracles/openrocket/drag_curves.py",
            ),
            ("flights.py", "validation/oracles/openrocket/flights.py"),
            ("events.py", "validation/oracles/openrocket/events.py"),
            (
                "geometry.py",
                "validation/oracles/rocketserializer/geometry.py",
            ),
        ] {
            let bytes = fs::read(root.join(path)).unwrap();
            assert_eq!(
                curves["inputs_sha256"][name].as_str(),
                Some(crate::ork_supply::sha256(&bytes).as_str()),
                "{PUBLIC_DRAG_CURVES} was not written by the current {path}"
            );
        }
        let examples: BTreeSet<&str> = record["designs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["file"].as_str().is_some_and(|f| f.contains(".jar!")))
            .filter_map(|entry| entry["sha256"].as_str())
            .collect();
        let designs = curves["designs"].as_array().unwrap();
        assert_eq!(designs.len(), 17);
        for design in designs {
            let sha = design["sha256"].as_str().unwrap();
            assert!(
                examples.contains(sha),
                "{} is not an example the record flew",
                design["file"]
            );
        }
        // The one public flight whose apogee needed them: the tube fin rocket, within the bar on
        // OpenRocket's drag (M2.2e9).
        let tubes = report["flights"]
            .as_array()
            .unwrap()
            .iter()
            .find(|flight| flight["design"] == "Tube fin rocket")
            .unwrap();
        assert_eq!(cause(tubes, FlightMetric::Apogee), OWN_DRAG);
        let on_theirs = tubes[OPENROCKET_DRAG_PROBE]["apogee_m"]["relative_percent"]
            .as_f64()
            .unwrap();
        assert!(on_theirs.abs() < 0.1, "{on_theirs}");
    }

    /// The record's flight of `file`'s `configuration`.
    fn recorded<'a>(record: &'a Value, file: &str, configuration: &str) -> &'a Value {
        record["designs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["file"] == file)
            .flat_map(|entry| entry["flights"].as_array().unwrap())
            .find(|flight| flight["configuration"] == configuration)
            .unwrap_or_else(|| panic!("{file} {configuration} is not in the record"))
    }

    #[test]
    fn every_motor_configuration_of_the_record_is_flown_or_named() {
        let (record, report) = committed();
        let mut listed: Vec<(String, String)> = report["flights"]
            .as_array()
            .unwrap()
            .iter()
            .chain(report["not_flown"].as_array().unwrap())
            .chain(report["probes"].as_array().unwrap())
            .map(|f| {
                (
                    f["file"].as_str().unwrap().to_owned(),
                    f["configuration"].as_str().unwrap().to_owned(),
                )
            })
            .collect();
        let mut expected: Vec<(String, String)> = record["designs"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|entry| {
                let name = entry["file"].as_str().unwrap().to_owned();
                entry["flights"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|f| f["has_motors"] == true)
                    .map(move |f| {
                        (
                            name.clone(),
                            f["configuration"].as_str().unwrap().to_owned(),
                        )
                    })
            })
            .collect();
        listed.sort();
        expected.sort();
        assert_eq!(listed, expected);
        // M2.2d2's scope, the 21 configurations in five of the jar's examples that hpr flies, and
        // M1.9c's 12: five clusters, five air starts beside a cluster and two two-stage flights.
        // M1.13c2's six pod probes and M2.2e5's four tilted-rod probes are listed apart. M2.2e9
        // added the tube fin rocket, M4.5g2 the three-stage example's three configurations,
        // M4.5g3 the six of the two payload examples, M4.5g4 the five of *Pods--airframes and
        // winglets*, M4.5i the three of *Pods--powered with recovery deployment* that
        // OpenRocket completes, M4.5m the two of *Parallel booster staging*, and M4.5n the one
        // of *Pods--powered* that OpenRocket aborts.
        assert_eq!(report["flights"].as_array().unwrap().len(), 54);
        assert_eq!(report["probes"].as_array().unwrap().len(), 10);
    }

    #[test]
    fn the_reference_values_are_the_records_and_the_outcomes_are_compares() {
        let (record, report) = committed();
        let tool = openrocket();
        let probes = report["probes"].as_array().unwrap();
        for flight in report["flights"].as_array().unwrap().iter().chain(probes) {
            let design = flight["file"].as_str().unwrap();
            let configuration = flight["configuration"].as_str().unwrap();
            let source = recorded(&record, design, configuration);
            assert_eq!(flight["design"], design_name(design).as_str());
            assert_eq!(flight["aborted"], source["aborted"]);
            assert_eq!(flight["motors"], source["name"]);
            // A flight OpenRocket aborted is held up to the abort (ADR-172): OpenRocket's side is
            // the record's, and the outcome is the percentages' against ABORT_PERCENT.
            assert_eq!(
                flight["before_abort"].is_object(),
                source["aborted"] == true,
                "{design} {configuration}"
            );
            if source["aborted"] == true {
                let held = &flight["before_abort"];
                let split = source["events"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|event| event["type"] == "STAGE_SEPARATION")
                    .unwrap();
                assert_eq!(held["time_s"], source["series"]["last_time_s"]);
                assert_eq!(
                    held["height_m"]["openrocket"],
                    source["series"]["max_altitude_m"]
                );
                assert_eq!(
                    held["speed_m_s"]["openrocket"],
                    source["series"]["last_total_velocity_m_s"]
                );
                assert_eq!(held["separation"]["time_s"]["openrocket"], split["time_s"]);
                assert_eq!(
                    held["separation"]["height_m"]["openrocket"],
                    split["after"]["altitude_m"]
                );
                assert_eq!(
                    held["separation"]["speed_m_s"]["openrocket"],
                    split["after"]["total_velocity_m_s"]
                );
                let entries = [
                    &held["height_m"],
                    &held["speed_m_s"],
                    &held["separation"]["height_m"],
                    &held["separation"]["speed_m_s"],
                ];
                let mut met = held["separation"]["time_s"]["openrocket"]
                    == held["separation"]["time_s"]["hpr"];
                for entry in entries {
                    let (reference, hpr) = (
                        entry["openrocket"].as_f64().unwrap(),
                        entry["hpr"].as_f64().unwrap(),
                    );
                    let percent = 100.0 * (hpr - reference) / reference;
                    assert!(
                        (entry["relative_percent"].as_f64().unwrap() - percent).abs() <= 1e-9,
                        "{design} {configuration}"
                    );
                    met &= percent.abs() <= ABORT_PERCENT;
                }
                assert_eq!(held["met"], json!(met), "{design} {configuration}");
                // M4.5n's *done when*: the one flight OpenRocket aborts is met up to its abort.
                assert!(met, "{design} {configuration}");
            }
            assert_eq!(flight["rod_length_m"], source["conditions"]["rod_length_m"]);
            assert_eq!(flight["max_mach_openrocket"], source["summary"]["max_mach"]);
            assert_eq!(
                flight["launch_mass_kg"]["openrocket"],
                source["series"]["launch_mass_kg"]
            );
            assert_eq!(
                flight["deployed_before_apogee_s"],
                json!(early_chute(source).unwrap()),
                "{design} {configuration}"
            );
            for key in [
                "mass_kg",
                "cg_from_nose_m",
                "cp_from_nose_m",
                "reference_length_m",
            ] {
                assert_eq!(
                    flight["at_rod_clearance"]["openrocket"][key], source["rod_clearance"][key],
                    "{design} {configuration} {key}"
                );
            }
            let keys = [
                &source["summary"]["max_altitude_m"],
                &source["summary"]["max_velocity_m_s"],
                &source["rod_clearance"]["stability_cal"],
            ];
            for ((metric, key), reference) in METRICS.iter().zip(keys) {
                let entry = &flight["metrics"][key];
                // An aborted reference is withheld (`metric_entry`).
                let written = if source["aborted"] == true {
                    &Value::Null
                } else {
                    reference
                };
                assert_eq!(
                    &entry["openrocket"], written,
                    "{design} {configuration} {key}"
                );
                let reading = if source["aborted"] == true {
                    ReferenceReading::Aborted
                } else {
                    ReferenceReading::Complete(reference.as_f64().or(Some(f64::NAN)))
                };
                let outcome = compare(&tool, *metric, reading, entry["hpr"].as_f64());
                assert_eq!(
                    metric_entry(reading, entry["hpr"].as_f64(), &outcome),
                    *entry,
                    "{design} {configuration} {key}"
                );
            }
            for key in ["time_s", "mach"] {
                assert_eq!(
                    flight["at_rod_clearance"][key], source["rod_clearance"][key],
                    "{design} {configuration} {key}"
                );
            }
        }
    }

    /// M4.5g3 (ADR-165): every configuration of OpenRocket's two payload examples flies, its
    /// stack coming apart with nothing left to burn, two of them before OpenRocket's apogee. The
    /// climb is flown whole; the part that keeps the nose has its apogee within 5% of OpenRocket's
    /// and its descent meets ADR-153's targets; and the coast with no drag, the flight refused, is
    /// held to the record's flight with nothing deployed. Each figure of OpenRocket's is the
    /// record's.
    #[test]
    fn a_payload_split_with_nothing_left_to_burn_is_held_to_the_record() {
        let (record, report) = committed();
        let (mut seen, mut before) = (0, 0);
        for flight in report["flights"].as_array().unwrap() {
            let split = &flight["unpowered_separation"];
            if !split.is_object() {
                continue;
            }
            seen += 1;
            let design = flight["file"].as_str().unwrap();
            let configuration = flight["configuration"].as_str().unwrap();
            let at = format!("{design} {configuration}");
            assert!(design.contains("payload"), "{at}");
            let source = recorded(&record, design, configuration);
            let first = |kind: &str| {
                source["events"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|event| event["type"] == kind)
                    .and_then(|event| event["time_s"].as_f64())
            };
            let separation_s = first("STAGE_SEPARATION").unwrap();
            assert_eq!(
                split["separation_s"]["openrocket"],
                json!(separation_s),
                "{at}"
            );
            let earlier = separation_s < first("APOGEE").unwrap();
            assert_eq!(split["before_apogee"], json!(earlier), "{at}");
            before += usize::from(earlier);
            let coast = &split["coast_apogee_m"];
            assert_eq!(
                coast["openrocket"], source["undeployed"]["max_altitude_m"],
                "{at}"
            );
            assert_eq!(
                *coast,
                relative(coast["openrocket"].as_f64(), coast["hpr"].as_f64()),
                "{at}"
            );
            // The climb is the whole stack's; the split is the descent's.
            assert!(flight["staging"].is_null(), "{at}");
            assert_eq!(flight["termination"], "ground_hit", "{at}");
            let descent = &flight["descent"];
            let apogee = descent["apogee_m"]["relative_percent"].as_f64().unwrap();
            assert!(apogee.abs() <= 5.0, "{at}: {apogee}");
            assert_eq!(descent["meets_targets"], true, "{at}");
        }
        assert_eq!((seen, before), (6, 2));
    }

    /// Each descent's OpenRocket values are the record's, and every comparison, residual and
    /// target outcome follows from its numbers (ADR-153); no public descent failed, and the only
    /// flights not compared are ones OpenRocket separates where hpr does not, and ones OpenRocket
    /// aborted (ADR-172).
    #[test]
    fn the_descents_are_the_records_and_their_outcomes_follow_from_them() {
        let (record, report) = committed();
        let mut compared = 0;
        for flight in report["flights"].as_array().unwrap() {
            let design = flight["file"].as_str().unwrap();
            let configuration = flight["configuration"].as_str().unwrap();
            let at = format!("{design} {configuration}");
            let source = recorded(&record, design, configuration);
            let descent = &flight["descent"];
            if let Some(why) = descent["not_flown"].as_str() {
                if source["aborted"] == true {
                    assert_eq!(why, DESCENT_NOTHING_RECORDED, "{at}");
                } else {
                    assert_eq!(why, DESCENT_SEPARATES, "{at}");
                }
                continue;
            }
            compared += 1;
            let summary = &source["summary"];
            let events: Vec<&Value> = source["events"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|event| event["type"] == "RECOVERY_DEVICE_DEPLOYMENT")
                .collect();
            let apogee_event = source["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|event| event["type"] == "APOGEE")
                .and_then(|event| event["time_s"].as_f64());
            for (key, reference) in [
                ("landing_speed_m_s", &summary["ground_hit_velocity_m_s"]),
                ("flight_time_s", &summary["flight_time_s"]),
                ("apogee_m", &summary["max_altitude_m"]),
                (
                    "last_deployment_speed_m_s",
                    &summary["deployment_velocity_m_s"],
                ),
            ] {
                let entry = &descent[key];
                assert_eq!(&entry["openrocket"], reference, "{at} {key}");
                assert_eq!(
                    *entry,
                    relative(entry["openrocket"].as_f64(), entry["hpr"].as_f64()),
                    "{at} {key}"
                );
            }
            assert_eq!(
                descent["apogee_s"]["openrocket"],
                source["series"]["time_of_max_altitude_s"]
            );
            let deployments = descent["deployments"].as_array().unwrap();
            assert_eq!(
                descent["deployment_count"]["openrocket"],
                json!(events.len())
            );
            let hpr_count = descent["deployment_count"]["hpr"].as_u64().unwrap() as usize;
            assert_eq!(deployments.len(), events.len().min(hpr_count), "{at}");
            let number = |value: &Value| value.as_f64();
            let hpr = (
                number(&descent["apogee_s"]["hpr"]),
                number(&descent["apogee_m"]["hpr"]),
                number(&descent["start_height_m"]),
            );
            let openrocket_apogee = (
                apogee_event,
                number(&descent["apogee_s"]["openrocket"]),
                number(&descent["apogee_m"]["openrocket"]),
            );
            let mut all_within = true;
            let mut all_sized = true;
            for (deployment, event) in deployments.iter().zip(&events) {
                let openrocket = deployment["openrocket_s"].as_f64().unwrap();
                assert_eq!(deployment["openrocket_s"], event["time_s"], "{at}");
                assert_eq!(
                    deployment["openrocket_height_m"], event["after"]["altitude_m"],
                    "{at}"
                );
                let difference = deployment["hpr_s"].as_f64().unwrap() - openrocket;
                assert_eq!(deployment["difference_s"], json!(difference), "{at}");
                let allowed = DEPLOYMENT_TARGET_S.max(DEPLOYMENT_TARGET_FRACTION * openrocket);
                let within = difference.abs() <= allowed;
                assert_eq!(deployment["within_target"], json!(within), "{at}");
                let trigger = match deployment["trigger"].as_str().unwrap() {
                    "apogee" => Trigger::Apogee,
                    "altitude" => Trigger::Altitude {
                        height_above_ground_m: deployment["hpr_height_m"].as_f64().unwrap(),
                    },
                    "ejection" => Trigger::MotorDelay { motor: 0 },
                    _ => Trigger::Time { time_s: 0.0 },
                };
                let expected = expected_difference_s(
                    trigger,
                    openrocket_apogee,
                    hpr,
                    (openrocket, number(&deployment["openrocket_height_m"])),
                );
                assert_eq!(deployment["expected_difference_s"], json!(expected), "{at}");
                let residual = expected.map(|expected| difference - expected);
                assert_eq!(deployment["residual_s"], json!(residual), "{at}");
                let sized = residual.map(|residual| residual.abs() <= allowed);
                assert_eq!(deployment["residual_within_target"], json!(sized), "{at}");
                all_within &= within;
                all_sized &= within || sized == Some(true);
            }
            let counted =
                descent["deployment_count"]["openrocket"] == descent["deployment_count"]["hpr"];
            let percent_within = |key: &str| {
                descent[key]["relative_percent"]
                    .as_f64()
                    .is_some_and(|p| p.abs() <= DESCENT_TARGET_PERCENT)
            };
            let rest =
                counted && percent_within("landing_speed_m_s") && percent_within("flight_time_s");
            assert_eq!(descent["meets_targets"], json!(rest && all_within), "{at}");
            assert_eq!(
                descent["meets_targets_once_sized"],
                json!(rest && all_sized),
                "{at}"
            );
        }
        assert!(compared > 0, "the report compares no descent");
    }

    /// The share of a deployment's difference the apogee and OpenRocket's step explain, by hand:
    /// none for a motor's charge or a time; the apogee's time for an apogee; for a height, the
    /// apogee's time plus the two falls' difference at OpenRocket's mean speed.
    #[test]
    fn a_deployments_expected_difference_is_the_apogee_and_openrocket_s_step() {
        let openrocket_apogee = (Some(10.05), Some(10.0), Some(600.0));
        let hpr = (Some(9.9), Some(590.0), Some(0.5));
        assert_eq!(
            expected_difference_s(
                Trigger::MotorDelay { motor: 0 },
                openrocket_apogee,
                hpr,
                (12.0, None)
            ),
            None
        );
        assert_eq!(
            expected_difference_s(
                Trigger::Time { time_s: 3.0 },
                openrocket_apogee,
                hpr,
                (3.0, None)
            ),
            None
        );
        // The apogee's own time, against OpenRocket's apogee event (a step after its top).
        let apogee = expected_difference_s(Trigger::Apogee, openrocket_apogee, hpr, (10.06, None));
        assert!((apogee.unwrap() - (9.9 - 10.05)).abs() < 1e-12);
        // OpenRocket falls 600 - 145 = 455 m in 20 s, 22.75 m/s; hpr 590 + 0.5 - 150 = 440.5 m.
        let height = expected_difference_s(
            Trigger::Altitude {
                height_above_ground_m: 150.0,
            },
            openrocket_apogee,
            hpr,
            (30.0, Some(145.0)),
        );
        let by_hand = (9.9 - 10.0) + (440.5 - 455.0) / 22.75;
        assert!((height.unwrap() - by_hand).abs() < 1e-12, "{height:?}");
        // A missing number gives none, not a made-up share.
        assert_eq!(
            expected_difference_s(
                Trigger::Altitude {
                    height_above_ground_m: 150.0
                },
                openrocket_apogee,
                hpr,
                (30.0, None)
            ),
            None
        );
    }

    /// The descents' summary sorts rows by scope, counts each target, and names why a flight
    /// was not compared.
    #[test]
    fn the_descents_summary_counts_by_scope_and_target() {
        let row = |in_scope: bool, speed: f64, within: bool, meets: bool, sized: bool| {
            json!({
                "outcome": "compared",
                "in_scope": in_scope,
                "landing_speed_percent": speed,
                "flight_time_percent": -speed,
                "deployments_within": within,
                "meets_targets": meets,
                "meets_targets_once_sized": sized,
            })
        };
        let rows = [
            row(true, 0.1, true, true, true),
            row(true, -0.3, false, false, true),
            row(false, 2.0, true, false, false),
            json!({ "outcome": DESCENT_SEPARATES }),
            json!({ "outcome": DESCENT_SEPARATES }),
        ];
        let summary = descent_summary(&rows);
        assert_eq!(summary["compared"], 3);
        assert_eq!(summary["not_compared"][DESCENT_SEPARATES], 2);
        let in_scope = &summary["in_scope"];
        assert_eq!(in_scope["flights"], 2);
        assert_eq!(in_scope["deployments_within"], 1);
        assert_eq!(in_scope["meeting_targets"], 1);
        assert_eq!(in_scope["meeting_targets_once_sized"], 2);
        assert_eq!(in_scope["landing_speed_percent"]["min"], -0.3);
        assert_eq!(summary["apart"]["flights"], 1);
        assert_eq!(summary["apart"]["meeting_targets"], 0);
    }

    #[test]
    fn the_summary_and_the_page_are_the_flights() {
        let root = crate::ork::root().unwrap();
        let (_, report) = committed();
        let summary = summarise(
            report["flights"].as_array().unwrap(),
            report["not_flown"].as_array().unwrap(),
        );
        assert_eq!(summary, report["summary"]);
        let page_now = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(page_now.replace("\r\n", "\n"), page(&report));
    }

    /// OpenRocket's drag curves become a table on its reference diameter, power on while burning;
    /// a flight that can't stand for one gives none, and a malformed record is an error.
    #[test]
    fn openrocket_s_drag_is_a_table_of_its_two_curves() {
        let flight = json!({
            "configuration": "ABC",
            "branches": 1,
            "aborted": false,
            "reference_length_m": 0.1,
            "power_on": [[0.1, 0.5], [0.5, 0.4], [1.5, 0.6]],
            "power_off": [[0.2, 0.7], [1.0, 0.9]],
        });
        let design = |flight: &Value| json!({ "flights": [flight] });
        let table = openrocket_drag(&design(&flight), "abc", false)
            .unwrap()
            .unwrap();
        assert_eq!(table.reference_diameter_m, Some(0.1));
        let at = |mach: f64, thrusting: bool| table.lookup(mach, thrusting).unwrap().value;
        assert!((at(1.0, true) - 0.5).abs() < 1e-15);
        assert!((at(0.6, false) - 0.8).abs() < 1e-15);
        // Held at the ends.
        assert_eq!(at(3.0, true), 0.6);
        assert_eq!(at(0.0, false), 0.7);
        assert!(
            openrocket_drag(&design(&flight), "other", false)
                .unwrap()
                .is_none()
        );
        // A renamed configuration is the record's only flight, and only then.
        assert!(
            openrocket_drag(&design(&flight), "other", true)
                .unwrap()
                .is_some()
        );
        let two = json!({ "flights": [flight.clone(), flight.clone()] });
        assert!(openrocket_drag(&two, "other", true).unwrap().is_none());
        for (key, value) in [
            ("branches", json!(2)),
            ("aborted", json!(true)),
            ("refused", json!("no")),
            ("power_off", json!([[0.2, 0.7]])),
        ] {
            let mut changed = flight.clone();
            changed[key] = value;
            assert!(
                openrocket_drag(&design(&changed), "ABC", false)
                    .unwrap()
                    .is_none(),
                "{key}"
            );
        }
        for (key, value) in [
            ("power_on", Value::Null),
            ("power_on", json!([[0.1, "x"], [0.5, 0.4]])),
            ("power_on", json!([[0.5, 0.5], [0.1, 0.4]])),
            ("reference_length_m", Value::Null),
        ] {
            let mut changed = flight.clone();
            changed[key] = value;
            assert!(
                openrocket_drag(&design(&changed), "ABC", false).is_err(),
                "{key}"
            );
        }
    }

    #[test]
    fn a_drag_probe_hpr_failed_or_had_no_curve_for_is_marked() {
        let flown = json!({ "apogee_m": { "relative_percent": 0.025 } });
        let whole = json!({ "apogee_m": { "relative_percent": 4.61 } });
        let both = json!({ OPENROCKET_DRAG_PROBE: flown, WHOLE_BASE_PROBE: whole });
        assert_eq!(
            drag_probes(&both, "apogee_m"),
            " (+0.03% on OpenRocket's drag) (+4.61% with the whole base under power)"
        );
        let failed = json!({ OPENROCKET_DRAG_PROBE: { "failed": true }, WHOLE_BASE_PROBE: whole });
        assert_eq!(
            drag_probes(&failed, "apogee_m"),
            " (failed on OpenRocket's drag) (+4.61% with the whole base under power)"
        );
        let no_curve = json!({ WHOLE_BASE_PROBE: { "failed": true } });
        assert_eq!(
            drag_probes(&no_curve, "apogee_m"),
            " (no curve to fly on OpenRocket's drag) (failed with the whole base under power)"
        );
        // A flight hpr did not probe has no brackets.
        assert_eq!(drag_probes(&json!({}), "apogee_m"), "");
    }

    /// Each flight with a metric put down to hpr's own drag, and the decision that writes its
    /// breakdown: flying on OpenRocket's drag says the gap is in the drag, not which drag is
    /// right, so each one needs its parts sized in writing (ADR-097), as the library's do.
    const DRAG_CAUSES_WRITTEN: [(&str, &str); 1] = [("Tube fin rocket [D12-7]", "ADR-099")];

    #[test]
    fn every_flight_put_down_to_the_drag_has_a_written_breakdown() {
        let (_, report) = committed();
        let caused: BTreeSet<String> = report["flights"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|flight| {
                [FlightMetric::Apogee, FlightMetric::MaxSpeed]
                    .into_iter()
                    .any(|metric| cause(flight, metric) == OWN_DRAG)
            })
            .map(|flight| {
                format!(
                    "{} {}",
                    flight["design"].as_str().unwrap(),
                    flight["motors"].as_str().unwrap()
                )
            })
            .collect();
        let written: BTreeSet<String> = DRAG_CAUSES_WRITTEN
            .iter()
            .map(|(flight, _)| (*flight).to_owned())
            .collect();
        assert_eq!(
            caused, written,
            "a flight put down to the drag needs a written breakdown"
        );
        let root = crate::ork::root().unwrap();
        for (flight, adr) in DRAG_CAUSES_WRITTEN {
            let record = crate::records::adr_text(&root, adr).unwrap();
            assert!(
                record.contains(&format!("`{flight}`'s breakdown")),
                "{adr} writes no breakdown of {flight}"
            );
        }
    }

    #[test]
    fn every_apogee_more_than_5_percent_off_has_a_named_cause() {
        // M2.2's parent asks a written cause for each. This report has three: a reference
        // parachute open before apogee, which hpr does not fly from a `.ork`; hpr's own drag,
        // sized on OpenRocket's drag curves of the jar's examples (M2.2e9); and a rocket that
        // turns over before apogee in both tools (M4.5i, ADR-168). A fourth, a drag coefficient
        // stated on the rocket itself, which hpr does not apply, is on no example; a part's, the
        // one some had, flies since M4.5h (ADR-167).
        let (_, report) = committed();
        for flight in report["flights"].as_array().unwrap() {
            let percent = flight["metrics"]["apogee_m"]["relative_percent"].as_f64();
            if percent.is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT) {
                assert!(
                    cause(flight, FlightMetric::Apogee) != NO_NAMED_CAUSE,
                    "{} {} is {percent:?}% off with no cause written",
                    flight["design"],
                    flight["motors"]
                );
            }
        }
    }

    /// M1.9c's bar (ADR-076 §1, set before these flights were measured): a two-stage design and a
    /// cluster design, each with every configuration hpr flies within 5% of OpenRocket's apogee
    /// (of its flight with nothing deployed, where its parachute opened before apogee) and of its
    /// largest speed. A staged flight also separates when OpenRocket's does.
    #[test]
    fn a_two_stage_and_a_cluster_design_are_within_5_percent_of_openrocket() {
        let (_, report) = committed();
        let flights = report["flights"].as_array().unwrap();
        let designs = |mark: &str| -> BTreeSet<&str> {
            flights
                .iter()
                .filter(|flight| !flight[mark].is_null())
                .map(|flight| flight["design"].as_str().unwrap())
                .collect()
        };
        let staged = designs("staging");
        let clustered = designs("clustered_motors");
        assert!(!staged.is_empty(), "no staged design is flown");
        assert!(!clustered.is_empty(), "no cluster design is flown");
        // A flight that turns over before apogee in both tools has no rigid-body reference to be
        // held to: OpenRocket's apogee is its tumble model's (ADR-168). One OpenRocket aborted
        // has no apogee at all, and is held up to the abort instead (ADR-172). Each is named
        // here, so that none joins unseen.
        let mut turned_over = Vec::new();
        let mut aborted = Vec::new();
        for flight in flights {
            let design = flight["design"].as_str().unwrap();
            if !staged.contains(design) && !clustered.contains(design) {
                continue;
            }
            let at = format!("{design} {}", flight["motors"]);

            let apogee_percent = if flight["deployed_before_apogee_s"].is_null() {
                &flight["metrics"]["apogee_m"]["relative_percent"]
            } else {
                &flight["apogee_with_the_causes_removed"]["parachutes_held"]["relative_percent"]
            };
            let speed_percent = &flight["metrics"]["max_speed_m_s"]["relative_percent"];
            if flight["aborted"] == true {
                assert_eq!(flight["before_abort"]["met"], true, "{at}");
                aborted.push(at.clone());
            } else if turns_over(flight) {
                turned_over.push(at.clone());
            } else {
                for (what, percent) in
                    [("apogee", apogee_percent), ("largest speed", speed_percent)]
                {
                    let percent = percent.as_f64().unwrap_or(f64::NAN);
                    assert!(percent.abs() <= 5.0, "{at}: {what} {percent}% off");
                }
            }
            let staging = &flight["staging"];
            assert!(
                staging.is_null() || staging.as_array().is_some_and(|list| !list.is_empty()),
                "{at}: staging is {staging}"
            );
            for separation in staging.as_array().into_iter().flatten() {
                let times = &separation["separation_s"];
                let (hpr, openrocket) = (
                    times["hpr"].as_f64().unwrap(),
                    times["openrocket"].as_f64().unwrap(),
                );
                assert!(
                    (hpr - openrocket).abs() < 1e-6,
                    "{at}: {hpr} s, {openrocket} s"
                );
            }
        }
        assert_eq!(
            turned_over,
            ["Pods--powered with recovery deployment \"[C6-7; B6-0]\""]
        );
        assert_eq!(
            aborted,
            ["Pods--powered with recovery deployment \"[C6-7; 2× A3-4, B6-0]\""]
        );
    }

    /// M1.13's second bar (M1.13c2), at ADR-076's per-case 5%: every pod probe, one airframe
    /// carrying pods of bodies, fins, a tail cone, winglets or motors, within 5% of OpenRocket's
    /// apogee and largest speed. So the pods are seen to count, not only the airframe, what each
    /// probe's pods change (its apogee and margin against the same airframe without pods) is held
    /// too: within 1 point of apogee and 0.005 calibres of OpenRocket's change, and each margin
    /// within 0.005 calibres of OpenRocket's. The bounds were set after the measurement (0.32
    /// points, 0.0004 and 0.0014 calibres at most), to catch a pod rule that breaks: a pod fin's
    /// `K_T(B)` taken on the airframe, by a hand estimate, moves the margin by some 0.04 calibres.
    /// The census leaves the probes out (ADR-093), so these bounds are what holds them in CI.
    #[test]
    fn pod_designs_are_within_5_percent_of_openrocket() {
        let (_, report) = committed();
        let probe = |entry: &&Value| {
            entry["file"]
                .as_str()
                .is_some_and(|file| file.starts_with(POD_PROBES))
        };
        for list in ["flights", "not_flown"] {
            let listed: Vec<_> = report[list]
                .as_array()
                .unwrap()
                .iter()
                .filter(probe)
                .collect();
            assert!(listed.is_empty(), "a probe among the designs: {listed:?}");
        }
        let probes: BTreeMap<&str, &Value> = report["probes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(probe)
            .map(|flight| (flight["design"].as_str().unwrap(), flight))
            .collect();
        let names: Vec<_> = probes.keys().copied().collect();
        assert_eq!(
            names,
            [
                "pods-bodies-3",
                "pods-fins-2",
                "pods-fins-tail-4",
                "pods-motors-2",
                "pods-none",
                "pods-winglets-2"
            ]
        );
        let without = probes[WITHOUT_PODS];
        for (design, flight) in &probes {
            assert!(flight["deployed_before_apogee_s"].is_null(), "{design}");
            for metric in ["apogee_m", "max_speed_m_s"] {
                let percent = flight["metrics"][metric]["relative_percent"]
                    .as_f64()
                    .unwrap_or(f64::NAN);
                assert!(percent.abs() <= 5.0, "{design}: {metric} {percent}% off");
            }
            let [[apogee_or, apogee_hpr], [margin_or, margin_hpr]] =
                probe_change(flight, without).unwrap();
            assert!(
                (apogee_hpr - apogee_or).abs() <= 1.0,
                "{design}: the pods change the apogee {apogee_hpr:+.2}% in hpr, {apogee_or:+.2}% \
                 in OpenRocket"
            );
            let margin = flight["metrics"]["rod_clearance_margin_cal"]["difference"]
                .as_f64()
                .unwrap_or(f64::NAN);
            assert!(
                margin.abs() <= 0.005,
                "{design}: margin {margin:+.4} cal off"
            );
            assert!(
                (margin_hpr - margin_or).abs() <= 0.005,
                "{design}: the pods change the margin {margin_hpr:+.4} cal in hpr, \
                 {margin_or:+.4} in OpenRocket"
            );
        }
    }

    /// The smaller turn between two compass bearings, in degrees.
    fn turn_deg(a: f64, b: f64) -> f64 {
        let turn = (a - b).rem_euclid(360.0);
        turn.min(360.0 - turn)
    }

    /// OpenRocket's rod leans the way `conditions.py` found its rocket lands (a compass bearing,
    /// the angle from the vertical), and [`rod_rail`] points hpr's rail the same way: its bearing
    /// is where OpenRocket's rocket landed from the same rod, and its angle above the horizon is
    /// 90 degrees less the rod's.
    #[test]
    fn the_rail_leans_where_openrockets_rocket_lands() {
        let root = crate::ork::root().unwrap();
        let measured =
            read_json(&root.join("validation/fixtures/ork/openrocket-conditions.json")).unwrap();
        let landings = measured["direction"].as_array().unwrap();
        assert_eq!(landings.len(), 2);
        for landing in landings {
            let direction_deg = landing["rod_direction_deg"].as_f64().unwrap();
            let rail = rod_rail(&json!({
                "rod_length_m": 1.0,
                "rod_angle_rad": 10f64.to_radians(),
                "rod_direction_rad": direction_deg.to_radians(),
            }))
            .unwrap();
            let up = rail.direction_enu();
            assert!((up.z - 10f64.to_radians().cos()).abs() < 1e-12, "{up}");
            let landed = bearing_deg(
                landing["landed_east_m"].as_f64().unwrap(),
                landing["landed_north_m"].as_f64().unwrap(),
            );
            let leans = bearing_deg(up.x, up.y);
            assert!(
                turn_deg(leans, landed) < 1e-9,
                "a rod toward {direction_deg}° leans to {leans}°, OpenRocket landed at {landed}°"
            );
        }
        // No length, angle or direction is an error; a rod the rail refuses too.
        let rod = json!({"rod_length_m": 1.0, "rod_angle_rad": 0.1, "rod_direction_rad": 0.0});
        for key in ["rod_length_m", "rod_angle_rad", "rod_direction_rad"] {
            let mut missing = rod.clone();
            missing[key] = Value::Null;
            assert!(rod_rail(&missing).is_err(), "{key}");
        }
        let mut flat = rod.clone();
        flat["rod_angle_rad"] = json!(FRAC_PI_2);
        assert!(rod_rail(&flat).is_err());
    }

    /// M2.2e5 (issue #173), on the tilted-rod probes: the pod probes' airframe from rods tilted 5,
    /// 10 and 20 degrees toward three bearings, which OpenRocket flew from each file's stored
    /// conditions. Each is within ADR-076's per-case 5% of OpenRocket's apogee and largest speed,
    /// and three things a rod read the wrong way would break are held: where the rocket is at
    /// apogee, within 0.1 degrees of OpenRocket's bearing from the pad and 1% of its distance;
    /// what the rod takes off the apogee against the vertical `pods-none`, within 0.5 points of
    /// OpenRocket's; and the margin at rod clearance at OpenRocket's angle of attack there,
    /// within 0.006 calibres of OpenRocket's. The bounds were set after the measurement (0.03
    /// degrees, 0.64%, 0.27 points and 0.0048 calibres at most); a bearing read anticlockwise
    /// turns the east probes 180 degrees, and an angle read from the horizon makes the vertical
    /// rod a horizontal rail, which hpr refuses. The bearing's 0.03 degrees is motion across the
    /// tilt (0.18 m across 370 m at 20 degrees): hpr's fits Earth's rotation, OpenRocket's goes
    /// the other way, cause unmeasured. The distance's 0.64% is partly that OpenRocket's position
    /// is its highest 0.05 s row and hpr's its apogee event: a longer flight would take more of
    /// both bounds.
    /// The census leaves the probes out (ADR-093), so these bounds are what holds them in CI.
    #[test]
    fn tilted_rods_fly_as_openrocket_flies_them() {
        let (_, report) = committed();
        let probes: BTreeMap<&str, &Value> = report["probes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|flight| (flight["design"].as_str().unwrap(), flight))
            .collect();
        let rods: Vec<(&str, &Value)> = probes
            .iter()
            .filter(|(_, flight)| {
                flight["file"]
                    .as_str()
                    .is_some_and(|file| file.starts_with(ROD_PROBES))
            })
            .map(|(design, flight)| (*design, *flight))
            .collect();
        let names: Vec<_> = rods.iter().map(|(design, _)| *design).collect();
        assert_eq!(
            names,
            [
                "rod-10-east",
                "rod-10-southwest",
                "rod-20-east",
                "rod-5-north"
            ]
        );
        let without = probes[WITHOUT_PODS];
        assert_eq!(without["rod"]["angle_from_vertical_rad"], json!(0.0));
        // From the vertical rod OpenRocket clears at next to no angle of attack (1.6e-7 rad), a
        // thousandth of the smallest tilt's.
        let vertical_alpha = without["at_rod_clearance"]["openrocket"]["angle_of_attack_rad"]
            .as_f64()
            .unwrap();
        assert!(vertical_alpha.abs() < 1e-6, "{vertical_alpha}");
        for (design, flight) in rods {
            let (angle, direction) = match design {
                "rod-5-north" => (5.0, 0.0),
                "rod-10-east" => (10.0, 90.0),
                "rod-10-southwest" => (10.0, 225.0),
                _ => (20.0, 90.0),
            };
            let rod = &flight["rod"];
            let read = |key: &str| rod[key].as_f64().unwrap().to_degrees();
            assert!(
                (read("angle_from_vertical_rad") - angle).abs() < 1e-9,
                "{design}"
            );
            assert!((read("direction_rad") - direction).abs() < 1e-9, "{design}");
            for metric in ["apogee_m", "max_speed_m_s"] {
                let percent = flight["metrics"][metric]["relative_percent"]
                    .as_f64()
                    .unwrap_or(f64::NAN);
                assert!(percent.abs() <= 5.0, "{design}: {metric} {percent}% off");
            }
            let place = |code: &str| {
                let at = &flight["apogee_position_m"][code];
                (at["east"].as_f64().unwrap(), at["north"].as_f64().unwrap())
            };
            let ((east_or, north_or), (east_hpr, north_hpr)) = (place("openrocket"), place("hpr"));
            let (bearing_or, bearing_hpr) = (
                bearing_deg(east_or, north_or),
                bearing_deg(east_hpr, north_hpr),
            );
            assert!(
                turn_deg(bearing_or, direction) < 1.0,
                "{design}: OpenRocket's rocket is at {bearing_or}° at apogee"
            );
            assert!(
                turn_deg(bearing_hpr, bearing_or) <= 0.1,
                "{design}: at apogee hpr's rocket is at {bearing_hpr}°, OpenRocket's at \
                 {bearing_or}°"
            );
            let distance = east_hpr.hypot(north_hpr) / east_or.hypot(north_or) - 1.0;
            assert!(
                distance.abs() <= 0.01,
                "{design}: hpr's rocket is {:+.2}% as far from the pad at apogee",
                100.0 * distance
            );
            let [[apogee_or, apogee_hpr], _] = probe_change(flight, without).unwrap();
            assert!(
                apogee_or < 0.0,
                "{design}: the rod adds {apogee_or}% in OpenRocket"
            );
            assert!(
                (apogee_hpr - apogee_or).abs() <= 0.5,
                "{design}: the rod changes the apogee {apogee_hpr:+.2}% in hpr, \
                 {apogee_or:+.2}% in OpenRocket"
            );
            let alpha = flight["at_rod_clearance"]["openrocket"]["angle_of_attack_rad"]
                .as_f64()
                .unwrap();
            assert!(
                alpha > 0.0,
                "{design}: OpenRocket clears the rod at α {alpha}"
            );
            let margin_or = flight["metrics"]["rod_clearance_margin_cal"]["openrocket"]
                .as_f64()
                .unwrap();
            let at_alpha = margin_at_openrocket_alpha_cal(flight).unwrap();
            assert!(
                (at_alpha - margin_or).abs() <= 0.006,
                "{design}: margin at OpenRocket's α {at_alpha:.4} cal in hpr, {margin_or:.4} in \
                 OpenRocket"
            );
        }
    }

    #[test]
    fn every_named_cause_is_sized_by_openrockets_own_flight_without_it() {
        // M2.2e4: each named cause is sized by OpenRocket's flight of the same configuration
        // without it, which the record holds, and nothing else.
        let (record, report) = committed();
        let (mut sized, mut over, mut within, mut over_all, mut drag_sized) = (0, 0, 0, 0, 0);
        for flight in report["flights"].as_array().unwrap() {
            over_all += usize::from(
                flight["metrics"]["apogee_m"]["relative_percent"]
                    .as_f64()
                    .is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT),
            );
            let overridden = !flight["drag_overrides_not_applied"].is_null();
            let named = !flight["deployed_before_apogee_s"].is_null() || overridden;
            let removed = &flight["apogee_with_the_causes_removed"];
            let at = format!("{} {}", flight["design"], flight["motors"]);
            if !named || flight["aborted"] == true {
                assert!(removed.is_null(), "{at} is sized with no cause named");
                continue;
            }
            sized += 1;
            let file = flight["file"].as_str().unwrap();
            let source = recorded(&record, file, flight["configuration"].as_str().unwrap());
            let hpr = flight["metrics"]["apogee_m"]["hpr"].as_f64().unwrap();
            let check = |step: &Value, key: &str, measured: f64| {
                let openrocket = source[key]["max_altitude_m"].as_f64().unwrap();
                assert_eq!(
                    step["openrocket_m"].as_f64(),
                    Some(openrocket),
                    "{at} {key}"
                );
                let percent = 100.0 * (measured - openrocket) / openrocket;
                let written = step["relative_percent"].as_f64().unwrap();
                assert!((written - percent).abs() < 1e-9, "{at} {key}");
                written
            };
            let mut last = check(&removed["parachutes_held"], "undeployed", hpr);
            let too = &removed["drag_overrides_cleared_too"];
            if overridden {
                last = check(too, "undeployed_without_drag_overrides", hpr);
            } else {
                assert!(too.is_null(), "{at}");
            }
            let agrees = last.abs() <= APOGEE_CAUSE_PERCENT;
            assert_eq!(removed["remaining_percent"].as_f64(), Some(last), "{at}");
            assert_eq!(removed["within_5_percent"].as_bool(), Some(agrees), "{at}");
            // hpr on OpenRocket's drag, nothing deployed, against OpenRocket's flight with
            // nothing deployed: only where an early parachute, held, leaves more than 5%.
            let on_drag = &removed[OPENROCKET_DRAG_PROBE];
            let percent = flight["metrics"]["apogee_m"]["relative_percent"].as_f64();
            let off = percent.is_some_and(|p| p.abs() > APOGEE_CAUSE_PERCENT);
            if off && !agrees && !overridden {
                let probe = flight[OPENROCKET_DRAG_PROBE]["apogee_m"]["hpr"]
                    .as_f64()
                    .unwrap();
                assert_eq!(on_drag["hpr_m"].as_f64(), Some(probe), "{at}");
                drag_sized += usize::from(drag_sizes(Some(check(on_drag, "undeployed", probe))));
            } else {
                assert!(on_drag.is_null(), "{at}");
            }
            if off {
                over += 1;
                within += usize::from(agrees);
            }
        }
        // M1.9c added the cluster's three flights with an early parachute, one of them over 5%.
        // M2.2e9's tube fin rocket is a seventh over 5%, whose cause, hpr's own drag, is sized on
        // OpenRocket's drag instead (the public drag curves' test). M4.5g2's three-stage example
        // adds two flights with an early parachute, both within 5%. M4.5g3's payload example adds
        // two more, the payload's parachute opening at a split before apogee: one over 5% on the
        // whole stack's climb, within 5% of OpenRocket's flight with nothing deployed. M4.5h flies
        // a part's drag override (ADR-167): the base drag hack's three flights, whose one part
        // OpenRocket is told has no drag, lose that cause. Its C11-5 flight, which had no other,
        // is no longer sized and comes within 5% (one fewer sized, over 5% and over 5% overall);
        // its D12-3 and E12-4 flights keep their early parachute, now 6.0% and 9.1% from
        // OpenRocket's flight with nothing deployed (two fewer within). Flown on OpenRocket's
        // drag, nothing deployed, both come within 5% of that flight: what the parachute leaves
        // is in the drag coefficient (`drag_sized`). M4.5i's *Pods--powered* `[C6-7; B6-0]` is an
        // eighth over 5% overall: its sustainer turns over before apogee in both tools
        // (ADR-168), a cause no flight of OpenRocket's without it can size.
        assert_eq!(
            (sized, over, within, over_all, drag_sized),
            (15, 6, 4, 8, 2)
        );
        let summary = &report["summary"]["apogee_with_the_causes_removed"];
        assert_eq!(summary["over_5_percent"], over_all);
        assert_eq!(summary["over_5_percent_sized"], over);
        assert_eq!(summary["over_5_percent_within_5_percent_after"], within);
        assert_eq!(
            summary["over_5_percent_within_5_percent_on_openrocket_s_drag"],
            drag_sized
        );
    }

    #[test]
    fn a_parachute_moves_openrockets_apogee_only_when_it_opens_before_it() {
        // The early-chute cause, both ways, over every flight of the record. Where the first
        // deployment is not before the apogee event, the flight with nothing deployed has the same
        // apogee to the bit: nothing else differs between the two runs. Where it is before, the
        // flight with nothing deployed climbs higher, or is within what the deployment event alone
        // can move the peak of rows 0.05 s apart (g dt²/8, 3 mm).
        let (record, _) = committed();
        let sampling_m = 9.81 * 0.05 * 0.05 / 8.0;
        let (mut lowered, mut within_sampling, mut late) = (0, 0, 0);
        for design in record["designs"].as_array().unwrap() {
            for flight in design["flights"].as_array().into_iter().flatten() {
                if flight["has_motors"] != true
                    || flight["aborted"] != false
                    || flight["undeployed"]["aborted"] != false
                {
                    continue;
                }
                let at = format!("{} {}", design["file"], flight["name"]);
                // The flight's apogee is its summary's, which is its altitude column's peak.
                let deployed = flight["summary"]["max_altitude_m"].as_f64().unwrap();
                assert_eq!(flight["series"]["max_altitude_m"].as_f64(), Some(deployed));
                let held = flight["undeployed"]["max_altitude_m"].as_f64().unwrap();
                if early_chute(flight).unwrap().is_some() {
                    if held - deployed > sampling_m {
                        lowered += 1;
                    } else {
                        assert!((held - deployed).abs() <= sampling_m, "{at}");
                        within_sampling += 1;
                    }
                } else {
                    late += 1;
                    assert_eq!(deployed, held, "{at}: a late parachute moved the apogee");
                }
            }
        }
        // The six pod probes (M1.13c2) and four rod probes (M2.2e5) carry no parachute, so they
        // are among the late.
        assert_eq!((lowered, within_sampling, late), (14, 1, 51));
    }

    #[test]
    fn a_cause_is_sized_against_the_flight_without_it() {
        let part = "a1b2";
        let recorded = json!({
            "undeployed": { "aborted": false, "max_altitude_m": 200.0 },
            "undeployed_without_drag_overrides": {
                "aborted": false, "refused": null, "cleared": [part], "max_altitude_m": 160.0,
            },
        });
        let overrides = [DragOverride {
            id: part.to_owned(),
            name: "a rocket".to_owned(),
        }];
        let entry = |early: Value, overridden: bool| {
            let mut entry = json!({
                "aborted": false,
                "deployed_before_apogee_s": early,
                "metrics": { "apogee_m": { "hpr": 168.0 } },
            });
            if overridden {
                entry["drag_overrides_not_applied"] = json!(["a rocket"]);
            }
            entry
        };
        let close = |value: &Value, expected: f64| {
            assert!(
                (value.as_f64().unwrap() - expected).abs() < 1e-12,
                "{value}"
            );
        };
        // An early parachute alone: held, 168 against 200, more than 5% off after.
        let sized = causes_removed(&recorded, &entry(json!(1.5), false), &[])
            .unwrap()
            .unwrap();
        close(&sized["parachutes_held"]["relative_percent"], -16.0);
        close(&sized["remaining_percent"], -16.0);
        assert_eq!(sized["within_5_percent"], false);
        assert!(sized["drag_overrides_cleared_too"].is_null());
        // A drag override with no early parachute: 168 against 160, within 5%.
        let sized = causes_removed(&recorded, &entry(Value::Null, true), &overrides)
            .unwrap()
            .unwrap();
        close(
            &sized["drag_overrides_cleared_too"]["relative_percent"],
            5.0,
        );
        close(&sized["remaining_percent"], 5.0);
        assert_eq!(sized["within_5_percent"], true);
        // With an early parachute too, the flight with both taken out is the one left: 168
        // against 160, not the 200 of the parachute's alone.
        let sized = causes_removed(&recorded, &entry(json!(1.5), true), &overrides)
            .unwrap()
            .unwrap();
        close(&sized["parachutes_held"]["relative_percent"], -16.0);
        close(&sized["remaining_percent"], 5.0);
        assert_eq!(sized["within_5_percent"], true);
        // A rocket with no id is matched by count, as OpenRocket gives it a new one.
        let mut unnamed = overrides.to_vec();
        unnamed[0].id = String::new();
        assert!(causes_removed(&recorded, &entry(Value::Null, true), &unnamed).is_ok());
        unnamed.push(unnamed[0].clone());
        assert!(causes_removed(&recorded, &entry(Value::Null, true), &unnamed).is_err());
        let mut upper = overrides.clone();
        upper[0].id = part.to_uppercase();
        assert!(causes_removed(&recorded, &entry(Value::Null, true), &upper).is_ok());
        // Neither cause, an aborted flight or hpr's flight with no apogee is not sized.
        let none = |entry: &Value| causes_removed(&recorded, entry, &[]).unwrap().is_none();
        assert!(none(&entry(Value::Null, false)));
        let mut aborted = entry(json!(1.5), false);
        aborted["aborted"] = json!(true);
        assert!(none(&aborted));
        let mut no_apogee = entry(json!(1.5), false);
        no_apogee["metrics"]["apogee_m"]["hpr"] = Value::Null;
        assert!(none(&no_apogee));
        // A flight OpenRocket aborted, refused or failed has a reason and no difference.
        for (key, value, why) in [
            ("aborted", json!(true), "OpenRocket aborted it"),
            ("refused", json!("no motor"), "OpenRocket refused it"),
            ("driver_error", json!("boom"), "the oracle failed: boom"),
        ] {
            let mut failed = recorded.clone();
            failed["undeployed_without_drag_overrides"][key] = value;
            let sized = causes_removed(&failed, &entry(Value::Null, true), &overrides)
                .unwrap()
                .unwrap();
            assert_eq!(sized["drag_overrides_cleared_too"]["why"], why);
            assert!(sized["remaining_percent"].is_null());
            assert_eq!(sized["within_5_percent"], false);
        }
        // OpenRocket must have changed exactly the parts hpr reads, and a flight must be there.
        let mut other = recorded.clone();
        other["undeployed_without_drag_overrides"]["cleared"] = json!(["c3d4"]);
        assert!(causes_removed(&other, &entry(Value::Null, true), &overrides).is_err());
        // A part OpenRocket cleared besides the rocket's: hpr applies the part's, so the cleared
        // flight is not what hpr flies, and the report stops.
        let mut other = recorded.clone();
        other["undeployed_without_drag_overrides"]["cleared"] = json!([part, "c3d4"]);
        assert!(causes_removed(&other, &entry(Value::Null, true), &overrides).is_err());
        let mut missing = recorded.clone();
        missing["undeployed"]["max_altitude_m"] = Value::Null;
        assert!(causes_removed(&missing, &entry(json!(1.5), false), &[]).is_err());
        let mut missing = recorded.clone();
        missing["undeployed_without_drag_overrides"] = Value::Null;
        assert!(causes_removed(&missing, &entry(Value::Null, true), &overrides).is_err());
    }

    #[test]
    fn the_causes_summary_counts_only_flights_with_a_difference() {
        let flight = |percent: f64, removed: Value| {
            json!({
                "metrics": { "apogee_m": { "relative_percent": percent } },
                "apogee_with_the_causes_removed": removed,
            })
        };
        let summary = with_the_causes_removed(&[
            // Over 5%, sized and brought within.
            flight(
                -16.0,
                json!({"remaining_percent": 1.0, "within_5_percent": true}),
            ),
            // Over 5%, sized and not.
            flight(
                12.0,
                json!({"remaining_percent": 7.0, "within_5_percent": false}),
            ),
            // Over 5%, but OpenRocket's flights without the cause have only reasons.
            flight(
                9.0,
                json!({"remaining_percent": null, "within_5_percent": false}),
            ),
            // Over 5% with no named cause.
            flight(-6.0, Value::Null),
            // Within 5%, sized.
            flight(
                -1.0,
                json!({"remaining_percent": -2.0, "within_5_percent": true}),
            ),
            // Over 5%, not within after, within 5% on OpenRocket's drag with nothing deployed;
            // then one not within on it, and one whose flight on it has only a reason.
            flight(
                13.0,
                json!({"remaining_percent": 6.0, "within_5_percent": false,
                       OPENROCKET_DRAG_PROBE: {"relative_percent": -5.0}}),
            ),
            flight(
                11.0,
                json!({"remaining_percent": 9.0, "within_5_percent": false,
                       OPENROCKET_DRAG_PROBE: {"relative_percent": 5.01}}),
            ),
            flight(
                10.0,
                json!({"remaining_percent": 8.0, "within_5_percent": false,
                       OPENROCKET_DRAG_PROBE: {"why": "hpr failed to fly it"}}),
            ),
            // Within 5% of OpenRocket's apogee: not counted, whatever its flight on the drag.
            flight(
                4.0,
                json!({"remaining_percent": 6.0, "within_5_percent": false,
                       OPENROCKET_DRAG_PROBE: {"relative_percent": 0.0}}),
            ),
        ]);
        assert_eq!(summary["over_5_percent"], 7);
        assert_eq!(summary["over_5_percent_sized"], 5);
        assert_eq!(summary["over_5_percent_within_5_percent_after"], 1);
        assert_eq!(
            summary["over_5_percent_within_5_percent_on_openrocket_s_drag"],
            1
        );
        assert_eq!(summary["remaining_percent"]["count"], 7);
        assert_eq!(summary["remaining_percent"]["max"], 9.0);
        // The summary says so, after the count of flights within 5% after, only when it is some.
        let lines = |on_drag: usize| {
            let mut sized = summary.clone();
            sized["over_5_percent_within_5_percent_on_openrocket_s_drag"] = json!(on_drag);
            summary_lines(&json!({ "summary": { "apogee_with_the_causes_removed": sized } }))
        };
        let line = "- of those still more than 5% off with their causes removed, within 5% of \
                    OpenRocket's flight with nothing deployed when hpr flies OpenRocket's drag, \
                    and so put down to the drag coefficient: 1\n";
        let text = lines(1);
        let after = text.find("- apogees more than 5% off: 7").unwrap();
        assert_eq!(text[after..].lines().nth(1), Some(line.trim_end()));
        assert!(!lines(0).contains("put down to the drag coefficient"));
    }

    /// The drag probes fly for an apogee more than 5% off with no named cause, or with an early
    /// parachute that, held, still leaves more than 5%; and the second sizes what it leaves on
    /// OpenRocket's flight with nothing deployed, like with like (ADR-097, ADR-167).
    #[test]
    fn an_early_parachute_s_leftover_gap_is_sized_on_openrocket_s_drag() {
        let flight = |percent: f64, early: Value, within: Value| {
            json!({
                "aborted": false,
                "deployed_before_apogee_s": early,
                "metrics": { "apogee_m": { "hpr": 225.0, "relative_percent": percent } },
                "apogee_with_the_causes_removed": {
                    "parachutes_held": { "openrocket_m": 200.0, "relative_percent": 12.5 },
                    "within_5_percent": within,
                },
            })
        };
        let early = flight(13.0, json!(1.8), json!(false));
        assert!(drag_probed(&early));
        // Not when the held flight is within 5%, nor the apogee itself, nor aborted.
        assert!(!drag_probed(&flight(13.0, json!(1.8), json!(true))));
        assert!(!drag_probed(&flight(4.9, json!(1.8), json!(false))));
        let mut aborted = early.clone();
        aborted["aborted"] = json!(true);
        assert!(!drag_probed(&aborted));
        // Not under a drag override on the rocket; still with no named cause at all.
        let mut overridden = early.clone();
        overridden["drag_overrides_not_applied"] = json!(["a rocket"]);
        assert!(!drag_probed(&overridden));
        let mut none = flight(-6.0, Value::Null, Value::Null);
        none["apogee_with_the_causes_removed"] = Value::Null;
        assert!(drag_probed(&none));
        assert!(on_openrocket_s_drag_held(&none).is_none());
        // Flown on OpenRocket's drag: 210 m against 200 m nothing deployed, not the 40% the
        // probe holds against the flight compared, and the parachute stays the apogee's cause,
        // with no drag cause for the largest speed either.
        let mut probed = early.clone();
        probed[OPENROCKET_DRAG_PROBE] = json!({
            "apogee_m": { "hpr": 210.0, "relative_percent": 40.0 },
            "max_speed_m_s": { "relative_percent": 1.0 },
        });
        let sized = on_openrocket_s_drag_held(&probed).unwrap();
        assert_eq!(sized["hpr_m"], 210.0);
        assert_eq!(sized["openrocket_m"], 200.0);
        assert!((sized["relative_percent"].as_f64().unwrap() - 5.0).abs() < 1e-12);
        assert_eq!(cause(&probed, FlightMetric::Apogee), EARLY_CHUTE);
        assert_eq!(cause(&probed, FlightMetric::MaxSpeed), NO_NAMED_CAUSE);
        assert_eq!(
            drag_probes(&probed, "apogee_m"),
            " (+5.00% on OpenRocket's drag)"
        );
        assert_eq!(
            drag_probes(&probed, "max_speed_m_s"),
            " (+1.00% on OpenRocket's drag)"
        );
        // A probe hpr failed, or had no curve for, gives a reason.
        let mut failed = early.clone();
        failed[OPENROCKET_DRAG_PROBE] = json!({ "failed": true });
        assert_eq!(
            on_openrocket_s_drag_held(&failed).unwrap()["why"],
            "hpr failed to fly it"
        );
        assert_eq!(
            on_openrocket_s_drag_held(&early).unwrap()["why"],
            "no curve of OpenRocket's to fly"
        );
        // And the causes table shows it, or the reason.
        let mut table = probed.clone();
        table["apogee_with_the_causes_removed"][OPENROCKET_DRAG_PROBE] = sized;
        table["design"] = json!("d");
        table["motors"] = json!("[m]");
        let text = causes_table(&json!({ "flights": [table] }));
        assert!(text.contains("| no | 210.0 | +5.00% |"), "{text}");
        let mut reason = probed.clone();
        reason["apogee_with_the_causes_removed"][OPENROCKET_DRAG_PROBE] =
            on_openrocket_s_drag_held(&failed).unwrap();
        let text = causes_table(&json!({ "flights": [reason] }));
        assert!(
            text.contains("| no | hpr failed to fly it | n/a |"),
            "{text}"
        );
    }

    #[test]
    fn mass_and_cg_are_hprs_less_openrockets_and_skip_an_aborted_flight() {
        let flight = |aborted: bool| {
            json!({
                "aborted": aborted,
                "launch_mass_kg": { "openrocket": 2.0, "hpr": 2.02 },
                "at_rod_clearance": {
                    "openrocket": { "mass_kg": 1.6, "cg_from_nose_m": 1.0, "reference_length_m": 0.1 },
                    "hpr": { "mass_kg": 1.64, "cg_from_nose_m": 1.03, "reference_length_m": 0.2 },
                },
            })
        };
        let spreads = mass_and_cg(&[flight(false), flight(true)]);
        let only = |key: &str| {
            let s = &spreads[key];
            assert_eq!(s["count"], 1, "{key}");
            s["median"].as_f64().unwrap()
        };
        assert!((only("launch_mass_percent") - 1.0).abs() < 1e-12);
        assert!((only("rod_clearance_mass_percent") - 2.5).abs() < 1e-12);
        assert!((only("rod_clearance_cg_cal") - 0.3).abs() < 1e-12);
    }

    #[test]
    fn a_spread_is_the_count_median_mean_size_and_ends() {
        assert_eq!(
            spread(&[1.0, -2.0, 3.0, -4.0]),
            json!({ "count": 4, "median": -0.5, "mean_absolute": 2.5, "min": -4.0, "max": 3.0 })
        );
        assert_eq!(spread(&[]), json!({ "count": 0 }));
    }

    #[test]
    fn the_check_finds_a_moved_number_a_missing_key_and_a_changed_list() {
        let committed = json!({ "a": 1.0, "b": [1, 2], "c": "x" });
        let mut apart = Vec::new();
        same(
            &committed,
            &json!({ "a": 1.0 + 1e-12, "b": [1, 2], "c": "x" }),
            "",
            &mut apart,
        );
        assert!(apart.is_empty(), "{apart:?}");
        same(
            &committed,
            &json!({ "a": 1.001, "b": [1], "d": "x" }),
            "",
            &mut apart,
        );
        assert_eq!(apart.len(), 4, "{apart:?}");
    }

    #[test]
    fn a_designs_name_is_its_file_name() {
        assert_eq!(
            design_name(
                "refs/openrocket/OpenRocket-24.12.jar!datafiles/examples/Chute release.ork"
            ),
            "Chute release"
        );
        assert_eq!(
            design_name("validation/fixtures/ork/loft-demo/demo-stable.ork"),
            "demo-stable"
        );
    }

    #[test]
    fn a_curve_is_confirmed_only_when_openrocket_places_exactly_it() {
        use CurveSource::{Catalog, Digest, Other};
        let placed = |digests: &[&str]| digests.iter().map(|d| (*d).to_owned()).collect::<Vec<_>>();
        let two = placed(&["a", "b"]);
        assert_eq!(
            unconfirmed_curve(&[Digest("a"), Digest("b")], Some(&two)),
            None
        );
        assert_eq!(
            unconfirmed_curve(&[Digest("b"), Digest("a")], Some(&two)),
            None
        );
        // One too few, one too many, or one other, either way round.
        assert_eq!(
            unconfirmed_curve(&[Digest("a")], Some(&two)),
            Some(NOT_PLACED)
        );
        let repeated = placed(&["a", "a"]);
        assert_eq!(
            unconfirmed_curve(&[Digest("a")], Some(&repeated)),
            Some(NOT_PLACED)
        );
        assert_eq!(
            unconfirmed_curve(&[Digest("a"), Digest("a")], Some(&placed(&["a"]))),
            Some(NOT_PLACED)
        );
        assert_eq!(
            unconfirmed_curve(&[Digest("a"), Digest("c")], Some(&two)),
            Some(NOT_PLACED)
        );
        // No such configuration in the motor record, or a curve without a digest.
        assert_eq!(unconfirmed_curve(&[Digest("a")], None), Some(NOT_PLACED));
        assert_eq!(
            unconfirmed_curve(&[Other], Some(&placed(&[]))),
            Some(NOT_PLACED)
        );
        // A catalog curve is found by name, whatever OpenRocket places.
        assert_eq!(
            unconfirmed_curve(&[Digest("a"), Catalog], Some(&two)),
            Some(CURVE_BY_NAME)
        );
        assert_eq!(unconfirmed_curve(&[Catalog], None), Some(CURVE_BY_NAME));
    }

    #[test]
    fn only_a_rod_the_rail_takes_in_calm_standard_air_is_flown() {
        let calm = json!({
            "rod_angle_rad": 0.0, "wind_average_m_s": 0.0, "wind_turbulence": 0.0,
            "wind_model": "AVERAGE", "isa_atmosphere": true,
        });
        assert_eq!(unflown_conditions(&calm), Ok(None));
        let with = |key: &str, value: Value| {
            let mut conditions = calm.clone();
            conditions[key] = value;
            unflown_conditions(&conditions)
        };
        // A tilted rod is flown (M2.2e5), up to but not at the horizontal; one tilted back past
        // the vertical is not.
        assert_eq!(with("rod_angle_rad", json!(0.05)), Ok(None));
        assert_eq!(with("rod_angle_rad", json!(1.5)), Ok(None));
        for refused in [-0.05, FRAC_PI_2, 2.0] {
            assert_eq!(
                with("rod_angle_rad", json!(refused)),
                Ok(Some(ROD_NOT_TAKEN)),
                "{refused}"
            );
        }
        assert_eq!(with("wind_average_m_s", json!(2.0)), Ok(Some(WIND)));
        assert_eq!(with("wind_turbulence", json!(0.1)), Ok(Some(WIND)));
        assert_eq!(with("wind_model", json!("MULTI_LEVEL")), Ok(Some(WIND)));
        assert_eq!(
            with("isa_atmosphere", json!(false)),
            Ok(Some(NOT_STANDARD_AIR))
        );
        // A condition the record does not state is an error, not a reason.
        assert!(with("rod_angle_rad", Value::Null).is_err());
        assert!(with("wind_model", Value::Null).is_err());
        assert!(with("isa_atmosphere", Value::Null).is_err());
    }

    #[test]
    fn only_the_rocket_s_own_drag_override_is_not_applied() {
        let text = |t: &str| json!({"kind": "text", "text": t});
        let element = |name: &str, children: Vec<Value>| json!({"kind": "element", "name": name, "attributes": [], "children": children});
        let part = |tag: &str, id: &str, cd: &str| {
            element(
                tag,
                vec![
                    element("id", vec![text(id)]),
                    element("name", vec![text(&format!("{tag} {id}"))]),
                    element("overridecd", vec![text(cd)]),
                ],
            )
        };
        let mut rocket = element(
            "rocket",
            vec![
                element("id", vec![text("r")]),
                element("overridecd", vec![text("0.5")]),
                element(
                    "subcomponents",
                    vec![element(
                        "stage",
                        vec![
                            element("overridecd", vec![text("0.0")]),
                            element(
                                "subcomponents",
                                vec![
                                    part("nosecone", "n", "0.0"),
                                    part("transition", "t", "0.35"),
                                ],
                            ),
                        ],
                    )],
                ),
            ],
        );
        rocket.as_object_mut().unwrap().remove("kind");
        let root = json!({"name": "openrocket", "attributes": [], "children": [
            {"kind": "element", "name": "rocket", "attributes": [],
             "children": rocket["children"]},
        ]});
        let root: ork::Element = serde_json::from_value(root).unwrap();
        let found = drag_overrides(&root);
        let ids: Vec<&str> = found.iter().map(|o| o.id.as_str()).collect();
        // A stage's and a part's fly; only the rocket's own is not applied.
        assert_eq!(ids, vec!["r"]);
    }
}
