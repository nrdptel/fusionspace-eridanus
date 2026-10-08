//! `cargo xtask fixture-flights [conditions | --check]`: flies the private collection's tier-A
//! flights from a `.ork` in their day's weather, in hpr and in OpenRocket, and compares each with
//! its logged apogee (M2.3c1, ADR-184).
//!
//! The collection, `hpr-sim-fixtures` (ADR-151), pairs each tier-A flight's design as flown with
//! the barometric log of the same flight, its exact commercial motor, its date and site, and the
//! ERA5 reanalysis of its day. Its license allows only aggregate statistics to be published, so
//! everything per flight stays on this machine:
//!
//! - the manifest, [`MANIFEST`] under the gitignored `corpus/`, reads each flight's free-text row
//!   into numbers: which `.ork` and configuration, when and where, the rail, the ERA5 file, the
//!   logged apogee and what read it, an early deployment, and why a flight is left out of the
//!   errors, if it is;
//! - `validation/oracles/netcdf/fixture_era5.py` converts each ERA5 file to the classic format
//!   `hpr_io::netcdf` reads, under the gitignored `corpus-out/`;
//! - `conditions` writes [`CONDITIONS`]: each flight's configuration, as hpr chooses it, and its
//!   weather as OpenRocket takes it, the pad's temperature and pressure (its atmosphere is the
//!   standard one through them) and the wind every [`WIND_STEP_M`] up (its multi-level wind);
//! - `validation/oracles/openrocket/fixture_flights.py` flies those in OpenRocket 24.12 with
//!   nothing deployed and writes [`RECORD`];
//! - this flies hpr in the whole ERA5 profile, and again in OpenRocket's air (the standard
//!   atmosphere anchored at the pad, [`Ussa76::anchored`], in the same wind), with nothing
//!   deployed; turns each logged barometric apogee into the height climbed in the day's air
//!   ([`Barometer::height_m`]); and writes only aggregates, [`REPORT_JSON`] and [`REPORT_MD`].
//!
//! A flight either simulator does not fly is counted under its cause, and every cause names the
//! open issue that tracks it ([`ISSUES`]); a cause with none stops the run. `--check` writes
//! nothing and fails unless the committed report reproduces. CI, without `refs/`, holds the
//! committed report to itself ([`Report::check_consistent`]).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_atmos::{Atmosphere, LayeredWind, SoundingProfile, Ussa76, Wind, WindInterpolation};
use hpr_core::earth::Earth;
use hpr_core::geodesy::Geodetic;
use hpr_io::era5::{Era5Profile, Era5Request, UtcTime, direction_from_rad};
use hpr_io::netcdf::NetCdf;
use hpr_io::ork;
use hpr_sim::{
    Environment, EventKind, FlightSettings, FlightStep, Observer, Rail, SimError, Simulation,
};
use hpr_validate::real_flight::Barometer;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const USAGE: &str = "  fixture-flights [conditions | --check]
                           Fly the private collection's tier-A .ork flights in their ERA5
                           weather, in hpr and OpenRocket, against their logged apogees;
                           needs refs/hpr-sim-fixtures and the manifest under corpus/.
                           `conditions` writes what the OpenRocket oracle flies; then
                           writes validation/reports/fixture-flights.{md,json}, aggregates
                           only; --check writes nothing and fails unless they reproduce.";

/// The hand-read manifest of the flights, one entry each (private, gitignored).
pub(crate) const MANIFEST: &str = "corpus/hpr-sim-fixtures/manifest.json";

/// `fixture_era5.py`'s index of the converted ERA5 files.
const ERA5_INDEX: &str = "corpus-out/hpr-sim-fixtures/era5/index.json";

/// What the OpenRocket oracle flies: each flight's configuration and weather.
pub(crate) const CONDITIONS: &str = "corpus-out/hpr-sim-fixtures/conditions.json";

/// OpenRocket's flights, as `fixture_flights.py` writes them.
pub(crate) const RECORD: &str = "corpus-out/hpr-sim-fixtures/openrocket-flights.json";

/// The oracle, whose current text the record must have been written by.
const ORACLE: &str = "validation/oracles/openrocket/fixture_flights.py";

/// The committed report: aggregates only.
pub(crate) const REPORT_JSON: &str = "validation/reports/fixture-flights.json";

/// The committed report's page.
pub(crate) const REPORT_MD: &str = "validation/reports/fixture-flights.md";

/// The collection's tier-A flights, and those whose design is a `.ork` (its `flights.csv` at the
/// pinned commit, ADR-151): the manifest must hold exactly that many.
pub(crate) const TIER_A: usize = 89;
pub(crate) const TIER_A_ORK: usize = 83;

/// The rail both simulators fly where neither the flight nor its design's simulation states one:
/// 6 ft, m. Its length moves an apogee little; how many flights fly it is counted.
pub(crate) const DEFAULT_RAIL_M: f64 = 1.8288;

/// The spacing of the wind levels OpenRocket is given, from the pad up to [`WIND_TOP_M`], m:
/// close enough that how either code interpolates between them moves nothing measurable.
pub(crate) const WIND_STEP_M: f64 = 50.0;

/// The highest wind level OpenRocket is given, above the pad, m.
pub(crate) const WIND_TOP_M: f64 = 25_000.0;

/// The bounds of the error histogram's bins, per cent: below the first, between each pair, and
/// above the last.
pub(crate) const BINS_PERCENT: [f64; 13] = [
    -30.0, -25.0, -20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0,
];

/// The reasons a flight is left out of the errors, as the manifest classes them.
pub(crate) const EXCLUSION_CLASSES: [&str; 5] = [
    "a motor anomaly",
    "active control in the climb",
    "a log that lost its apogee",
    "a design that is not the flown rocket",
    "another reason",
];

/// The open issue that tracks each cause of a flight not flown, by who did not fly it and why.
pub(crate) const ISSUES: &[(&str, &str, u64)] = &[
    (HPR, TAB_PAST_ROOT, 357),
    (HPR, REPEATED_POINT, 358),
    (HPR, NEGATIVE_SPAN, 359),
    (HPR, UNREAD_FINISH, 360),
    (HPR, OVERRIDE_FLAGS, 361),
    (HPR, OFF_AXIS_TUBE, 181),
    (HPR, NO_CURVE, 362),
    (OPENROCKET, OPENROCKET_NO_MOTOR, 362),
    (COLLECTION, NO_CONFIGURATION, 363),
    (COLLECTION, NO_ERA5, 364),
];

/// hpr's causes, each named from the words of the refusal it stands for ([`HPR_CAUSES`]).
pub(crate) const TAB_PAST_ROOT: &str = "a fin tab ending past the root chord";
pub(crate) const REPEATED_POINT: &str = "a freeform fin outline read as crossing itself";
pub(crate) const NEGATIVE_SPAN: &str = "a part of negative length (rail buttons spaced forward)";
pub(crate) const UNREAD_FINISH: &str = "an airframe not read as written: an unread surface finish";
pub(crate) const OVERRIDE_FLAGS: &str =
    "an airframe not read as written: mass and CG overrides that differ on the parts inside";
pub(crate) const OFF_AXIS_TUBE: &str =
    "an airframe not read as written: a part inside an off-axis inner tube";
pub(crate) const NO_CURVE: &str = "a motor whose digest OpenRocket 24.12's database lacks";

/// OpenRocket's cause for a configuration it reads with no motor in it.
pub(crate) const OPENROCKET_NO_MOTOR: &str = "OpenRocket loads no motor into that configuration";

/// The words of an hpr refusal or failure, and the cause each names: the first that the detail
/// holds. A refusal none names is counted under its generic kind, which has no issue, so the run
/// stops until it is named and tracked. The first reason a refusal gives can hide a second.
pub(crate) const HPR_CAUSES: [(&str, &str); 7] = [
    ("a fin tab must lie along the root chord", TAB_PAST_ROOT),
    ("the freeform fin outline crosses itself", REPEATED_POINT),
    ("attached part length is outside its domain", NEGATIVE_SPAN),
    (
        "is not a surface finish this reader has a roughness for",
        UNREAD_FINISH,
    ),
    (
        "the mass override covers the parts inside this one",
        OVERRIDE_FLAGS,
    ),
    ("off the body's axis", OFF_AXIS_TUBE),
    ("no thrust curve for", NO_CURVE),
];

/// The cause `detail` names, or `generic` when none does.
fn named_cause(detail: &str, generic: &str) -> String {
    HPR_CAUSES
        .iter()
        .find(|(words, _)| detail.contains(words))
        .map_or(generic, |(_, cause)| cause)
        .to_owned()
}

/// Who stopped a flight short: hpr, OpenRocket, or the collection's data before either.
const HPR: &str = "hpr";
const OPENROCKET: &str = "OpenRocket";
const COLLECTION: &str = "the collection";

/// The collection's causes: a design with no configuration holding the flown motor, and a flight
/// whose day has no ERA5 file in the collection.
pub(crate) const NO_CONFIGURATION: &str = "the design holds no configuration with the flown motor";
pub(crate) const NO_ERA5: &str = "the collection has no ERA5 file of the flight's day";
/// A manifest naming a configuration id its design lacks: the manifest's error, with no issue,
/// so it stops the run.
pub(crate) const NO_CONFIGURATION_ID: &str = "the manifest's configuration id is not in the design";

/// The cause for a flight hpr's simulation fails.
pub(crate) const FLIGHT_FAILED: &str = "hpr's flight of it failed";

/// What a logged apogee is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LoggedKind {
    /// An altitude the altimeter computed from the pressure it measured.
    Barometric,
    /// A filtered estimate from a barometer and an accelerometer: read as barometric.
    Fused,
    /// A satellite height above the pad, taken as it is.
    Gps,
}

/// The motor flown, by its designation (the manufacturer's name is read and ignored).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MotorName {
    pub designation: String,
}

/// One flight of the manifest. The curation's own notes are read and ignored.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Entry {
    pub flight_id: String,
    pub design: String,
    pub configuration_id: Option<String>,
    pub motor: MotorName,
    pub launch_utc: String,
    pub launch_time_assumed: bool,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub rail_length_m: Option<f64>,
    pub rail_from_vertical_deg: Option<f64>,
    pub rail_heading_deg: Option<f64>,
    pub era5: Option<String>,
    pub logged_apogee_m: f64,
    pub logged_kind: LoggedKind,
    pub early_deployment: Option<bool>,
    pub excluded: Option<String>,
    pub excluded_class: Option<String>,
}

/// Flies, or writes OpenRocket's conditions.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    let mut conditions_only = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            "conditions" => conditions_only = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let entries = manifest(&root)?;
    let index = era5_index(&root)?;
    let supply = crate::ork_supply::Supply::load(&root, true)?;
    if !supply.is_present() {
        return Err(format!(
            "{} is missing; run validation/oracles/openrocket/motor_database.py",
            crate::ork_supply::RECORD
        ));
    }
    if conditions_only {
        return write_conditions(&root, &entries, &index, &supply);
    }
    let report = fly_all(&root, &entries, &index, &supply)?;
    let markdown = report.to_markdown();
    report.check_consistent(&markdown)?;
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: Report = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        if committed != report {
            return Err(format!(
                "{REPORT_JSON} does not reproduce; run `cargo xtask fixture-flights`"
            ));
        }
        let page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        committed.check_consistent(&page)?;
        println!("fixture-flights: the committed report reproduces");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, markdown)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("fixture-flights: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}

/// Reads the manifest and checks it holds each tier-A `.ork` flight once, with numbers that can
/// be flown and an exclusion class from the list.
fn manifest(root: &Path) -> Result<Vec<Entry>, String> {
    let text = fs::read_to_string(root.join(MANIFEST))
        .map_err(|error| format!("reading {MANIFEST}: {error}"))?;
    let entries: Vec<Entry> =
        serde_json::from_str(&text).map_err(|error| format!("reading {MANIFEST}: {error}"))?;
    if entries.len() != TIER_A_ORK {
        return Err(format!(
            "{MANIFEST} holds {} flights, not the collection's {TIER_A_ORK}",
            entries.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for entry in &entries {
        let at = &entry.flight_id;
        if !seen.insert(at.clone()) {
            return Err(format!("{MANIFEST}: {at} is listed twice"));
        }
        let finite = [
            entry.latitude_deg,
            entry.longitude_deg,
            entry.elevation_m,
            entry.logged_apogee_m,
        ];
        if finite.iter().any(|value| !value.is_finite()) || entry.logged_apogee_m <= 0.0 {
            return Err(format!("{MANIFEST}: {at} has a number that is not one"));
        }
        if entry.excluded.is_some() != entry.excluded_class.is_some()
            || entry
                .excluded_class
                .as_deref()
                .is_some_and(|class| !EXCLUSION_CLASSES.contains(&class))
        {
            return Err(format!(
                "{MANIFEST}: {at} needs an exclusion class from the list with its reason"
            ));
        }
        if !entry.design.to_ascii_lowercase().ends_with(".ork") {
            return Err(format!("{MANIFEST}: {at}'s design is not a .ork"));
        }
        // The id must be one of the collection's flights, and the design that flight's.
        let folder = format!("refs/hpr-sim-fixtures/flights/{at}/");
        if !root.join(&folder).is_dir() || !entry.design.starts_with(&folder) {
            return Err(format!(
                "{MANIFEST}: {at} is not a flight of the collection, or its design is another's"
            ));
        }
    }
    Ok(entries)
}

/// `fixture_era5.py`'s converted file for each source file.
fn era5_index(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let index = crate::ork_flights::read_json(&root.join(ERA5_INDEX))
        .map_err(|error| format!("{error}; run validation/oracles/netcdf/fixture_era5.py"))?;
    index["files"]
        .as_array()
        .ok_or_else(|| format!("{ERA5_INDEX} has no files"))?
        .iter()
        .map(
            |file| match (file["source"].as_str(), file["converted"].as_str()) {
                (Some(source), Some(converted)) => Ok((source.to_owned(), converted.to_owned())),
                _ => Err(format!("{ERA5_INDEX} has a file without its paths")),
            },
        )
        .collect()
}

/// A flight's weather: the ERA5 profile over its site at its hour, and its wind.
struct Weather {
    sounding: SoundingProfile,
    wind: LayeredWind,
}

fn weather(
    root: &Path,
    entry: &Entry,
    index: &BTreeMap<String, String>,
) -> Result<Weather, String> {
    let at = &entry.flight_id;
    let era5 = entry
        .era5
        .as_deref()
        .ok_or_else(|| format!("{at}: {NO_ERA5}"))?;
    let converted = index
        .get(era5)
        .ok_or_else(|| format!("{at}: {era5} was not converted"))?;
    let bytes = fs::read(root.join(converted)).map_err(|error| format!("{converted}: {error}"))?;
    let file = NetCdf::parse(&bytes).map_err(|error| format!("{converted}: {error}"))?;
    let profile = Era5Profile::read(
        &file,
        Era5Request {
            latitude_deg: entry.latitude_deg,
            longitude_deg: entry.longitude_deg,
            time: launch_time(entry)?,
        },
    )
    .map_err(|error| format!("{at}: {error}"))?;
    let sounding = profile
        .sounding(WindInterpolation::Components)
        .map_err(|error| format!("{at}: {error}"))?;
    let wind = sounding
        .wind()
        .ok_or_else(|| format!("{at}: the ERA5 profile gives no wind"))?
        .clone();
    Ok(Weather { sounding, wind })
}

/// The launch time, from the manifest's `YYYY-MM-DDTHH:MM:00Z`.
fn launch_time(entry: &Entry) -> Result<UtcTime, String> {
    let text = &entry.launch_utc;
    let bad = || {
        format!(
            "{}: launch time `{text}` is not YYYY-MM-DDTHH:MM:00Z",
            entry.flight_id
        )
    };
    let part = |range: std::ops::Range<usize>| -> Result<u32, String> {
        text.get(range).and_then(|s| s.parse().ok()).ok_or_else(bad)
    };
    if text.len() != 20 || !text.ends_with(":00Z") || text.as_bytes()[10] != b'T' {
        return Err(bad());
    }
    let year = i32::try_from(part(0..4)?).map_err(|_| bad())?;
    UtcTime::from_civil(
        year,
        part(5..7)?,
        part(8..10)?,
        part(11..13)?,
        part(14..16)?,
        0.0,
    )
    .map_err(|error| format!("{}: {error}", entry.flight_id))
}

/// The rail as the manifest gives it, or [`DEFAULT_RAIL_M`] long and upright.
fn rail(entry: &Entry) -> Rail {
    let from_vertical_deg = entry.rail_from_vertical_deg.unwrap_or(0.0);
    Rail {
        length_m: entry.rail_length_m.unwrap_or(DEFAULT_RAIL_M),
        azimuth_rad: entry.rail_heading_deg.unwrap_or(0.0).to_radians(),
        elevation_rad: (90.0 - from_vertical_deg).to_radians(),
        roll_rad: 0.0,
        friction_coefficient: 0.0,
    }
}

/// The design as hpr reads it, with OpenRocket's curves for the digests it records.
fn design(
    root: &Path,
    entry: &Entry,
    supply: &crate::ork_supply::Supply,
) -> Result<ork::Design, String> {
    let bytes = fs::read(root.join(&entry.design))
        .map_err(|error| format!("{}: {error}", entry.flight_id))?;
    let read = ork::read(&bytes).map_err(|error| format!("{}: {error}", entry.flight_id))?;
    Ok(ork::design_with(&read.value, supply.curves()).value)
}

/// A designation reduced to what names a motor: lower case, without the manufacturer, a note
/// in brackets, the total impulse before the class letter, a delay, spaces or dashes, so
/// `Maker 1000K1000-Xtra-14A (a note)` and `K1000XTRA` match.
fn designation_key(designation: &str) -> String {
    let lower = designation.to_ascii_lowercase();
    // The first word shaped like a motor (digits, one class letter, digits), outside brackets
    // first; a note's words, or the manufacturer's, are not the motor's name.
    let outside = lower.split('(').next().unwrap_or("");
    let motor_like = |word: &&str| {
        let name = word.trim_start_matches(|c: char| c.is_ascii_digit());
        let mut chars = name.chars();
        chars.next().is_some_and(|c| ('a'..='o').contains(&c))
            && chars.next().is_some_and(|c| c.is_ascii_digit())
    };
    let word = outside
        .split_whitespace()
        .find(motor_like)
        .or_else(|| {
            lower
                .split(|c: char| c.is_whitespace() || "()\"'".contains(c))
                .find(motor_like)
        })
        .unwrap_or(outside)
        .trim_start_matches(|c: char| c.is_ascii_digit());
    // A delay after the designation, `-14A`, `-P` or `-M`, is not the motor's name.
    let mut parts: Vec<&str> = word.split('-').collect();
    while parts.len() > 1
        && parts.last().is_some_and(|part| {
            part.starts_with(|c: char| c.is_ascii_digit()) || ["s", "m", "l", "p"].contains(part)
        })
    {
        parts.pop();
    }
    parts
        .concat()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

/// The configuration the flight flew: the manifest's, by id, or else the only one holding a
/// motor of the flown designation (the first, if several do).
fn configuration<'a>(
    design: &'a ork::Design,
    entry: &Entry,
) -> Result<&'a ork::MotorConfiguration, &'static str> {
    let configurations = &design.motors.configurations;
    if let Some(id) = &entry.configuration_id {
        return configurations
            .iter()
            .find(|c| c.id.eq_ignore_ascii_case(id))
            .ok_or(NO_CONFIGURATION_ID);
    }
    let flown = designation_key(&entry.motor.designation);
    configurations
        .iter()
        .find(|c| {
            c.motors
                .iter()
                .any(|motor| designation_key(&motor.designation) == flown)
        })
        .ok_or(NO_CONFIGURATION)
}

/// The configuration a flight flies, or the collection's cause for flying it in neither code.
fn collection_cause<'a>(
    design: &'a ork::Design,
    entry: &Entry,
) -> Result<&'a ork::MotorConfiguration, &'static str> {
    if entry.era5.is_none() {
        return Err(NO_ERA5);
    }
    configuration(design, entry)
}

/// The weather OpenRocket is given: the pad's temperature and pressure, and the wind every
/// [`WIND_STEP_M`] above the pad, blowing from a direction clockwise from north.
fn openrocket_weather(entry: &Entry, weather: &Weather) -> Result<Value, String> {
    let at = &entry.flight_id;
    let pad = weather
        .sounding
        .air(entry.elevation_m)
        .map_err(|error| format!("{at}: {error}"))?
        .air;
    let mut levels = Vec::new();
    let mut height_m = 0.0;
    while height_m <= WIND_TOP_M {
        let velocity = weather
            .wind
            .wind(entry.elevation_m + height_m)
            .map_err(|error| format!("{at}: {error}"))?
            .velocity_enu_m_s;
        let speed = velocity.x.hypot(velocity.y);
        levels.push(json!([
            height_m,
            speed,
            direction_from_rad(velocity.x, velocity.y)
        ]));
        height_m += WIND_STEP_M;
    }
    Ok(json!({
        "pad_temperature_k": pad.temperature_k,
        "pad_pressure_pa": pad.pressure_pa,
        "wind_agl_m_speed_m_s_from_rad": levels,
    }))
}

/// The text of [`CONDITIONS`], built from the manifest and the ERA5 files as they are now.
fn conditions(
    root: &Path,
    entries: &[Entry],
    index: &BTreeMap<String, String>,
    supply: &crate::ork_supply::Supply,
) -> Result<String, String> {
    let mut flights = Vec::new();
    for entry in entries {
        let design = design(root, entry, supply)?;
        let chosen = match collection_cause(&design, entry) {
            Ok(chosen) => chosen,
            Err(NO_CONFIGURATION_ID) => {
                return Err(format!("{}: {NO_CONFIGURATION_ID}", entry.flight_id));
            }
            Err(_) => continue,
        };
        let weather = weather(root, entry, index)?;
        let rail = rail(entry);
        flights.push(json!({
            "flight_id": entry.flight_id,
            "design": entry.design,
            "configuration": chosen.id,
            "designation": entry.motor.designation,
            "launch_altitude_m": entry.elevation_m,
            "launch_latitude_deg": entry.latitude_deg,
            "launch_longitude_deg": entry.longitude_deg,
            "rod_length_m": rail.length_m,
            "rod_angle_rad": entry.rail_from_vertical_deg.unwrap_or(0.0).to_radians(),
            "rod_direction_rad": rail.azimuth_rad,
            "weather": openrocket_weather(entry, &weather)?,
        }));
    }
    let text = serde_json::to_string_pretty(&json!({ "flights": flights }))
        .map_err(|error| error.to_string())?;
    Ok(text + "\n")
}

/// Writes [`CONDITIONS`].
fn write_conditions(
    root: &Path,
    entries: &[Entry],
    index: &BTreeMap<String, String>,
    supply: &crate::ork_supply::Supply,
) -> Result<(), String> {
    let text = conditions(root, entries, index, supply)?;
    let path = root.join(CONDITIONS);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{CONDITIONS}: {error}"))?;
    }
    fs::write(&path, text).map_err(|error| format!("writing {CONDITIONS}: {error}"))?;
    println!("fixture-flights: wrote {CONDITIONS}");
    Ok(())
}

/// The highest point of the center of mass above where it started.
#[derive(Default)]
struct Climb {
    start_m: Option<f64>,
}

impl Observer for Climb {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        if self.start_m.is_none() {
            self.start_m = Some(step.sample(step.start_s())?.height_above_ground_m);
        }
        Ok(())
    }
}

/// hpr's apogee of `entry` in `environment`, nothing deployed: the center of mass's height at
/// apogee above where it started, as OpenRocket's altitude is 0 at launch.
fn hpr_apogee(
    design: &ork::Design,
    configuration: &ork::MotorConfiguration,
    environment: Environment,
    rail: Rail,
) -> Result<f64, String> {
    let settings = FlightSettings {
        accept_design_errors: true,
        ..FlightSettings::default()
    };
    let simulation = Simulation::new(
        &design.rocket,
        &configuration.id,
        environment,
        rail,
        settings,
    )
    .map_err(|error| error.to_string())?;
    let stagings: Vec<ork::Staging> = configuration.stagings().cloned().collect();
    let stagings = crate::ork_flights::climbing(&stagings, simulation.assembly())
        .map_err(|error| error.to_string())?;
    let simulation = if stagings.is_empty() {
        simulation
    } else {
        crate::ork_flights::staged(simulation, stagings).map_err(|error| error.to_string())?
    };
    let mut climb = Climb::default();
    let result = simulation
        .run(&mut climb)
        .map_err(|error| error.to_string())?;
    let apogee = result
        .event(EventKind::Apogee)
        .ok_or("hpr's flight has no apogee")?;
    let start = climb.start_m.ok_or("hpr's flight has no steps")?;
    Ok(apogee.sample.height_above_ground_m - start)
}

/// The record's flights by id, after checking it was written by OpenRocket 24.12, by the
/// current oracle and the scripts it imports, from `conditions`: the conditions as the manifest
/// and the ERA5 files give them now, not as [`CONDITIONS`] last wrote them.
fn openrocket_record(root: &Path, conditions: &str) -> Result<BTreeMap<String, Value>, String> {
    let record = crate::ork_flights::read_json(&root.join(RECORD))
        .map_err(|error| format!("{error}; run {ORACLE}"))?;
    let stale = |what: &str| {
        format!(
            "{RECORD} was written from other {what}; run `cargo xtask fixture-flights conditions` \
             and {ORACLE} again"
        )
    };
    let inputs = record["inputs_sha256"]
        .as_object()
        .filter(|inputs| inputs.contains_key(ORACLE))
        .ok_or_else(|| format!("{RECORD} has no inputs_sha256 for {ORACLE}"))?;
    for (path, recorded) in inputs {
        let bytes = fs::read(root.join(path)).map_err(|error| format!("{path}: {error}"))?;
        if recorded.as_str() != Some(crate::ork_supply::sha256(&bytes).as_str()) {
            return Err(stale(&format!("scripts ({path} differs)")));
        }
    }
    if record["conditions_sha256"].as_str()
        != Some(crate::ork_supply::sha256(conditions.as_bytes()).as_str())
    {
        return Err(stale("conditions"));
    }
    if !record["openrocket"]
        .as_str()
        .is_some_and(|v| v.starts_with("24.12"))
    {
        return Err(format!("{RECORD} is not OpenRocket 24.12's"));
    }
    Ok(record["flights"]
        .as_array()
        .ok_or_else(|| format!("{RECORD} has no flights"))?
        .iter()
        .filter_map(|flight| Some((flight["flight_id"].as_str()?.to_owned(), flight.clone())))
        .collect())
}

/// One flight's numbers, kept on this machine.
struct Compared {
    hpr_percent: f64,
    openrocket_percent: f64,
    hpr_in_openrocket_air_percent: f64,
    hpr_over_openrocket_percent: f64,
    early: bool,
}

fn percent(simulated: f64, reference: f64) -> f64 {
    100.0 * (simulated - reference) / reference
}

/// Flies every flight and builds the report.
fn fly_all(
    root: &Path,
    entries: &[Entry],
    index: &BTreeMap<String, String>,
    supply: &crate::ork_supply::Supply,
) -> Result<Report, String> {
    let record = openrocket_record(root, &conditions(root, entries, index, supply)?)?;
    let mut causes: BTreeMap<(&'static str, String), usize> = BTreeMap::new();
    let mut excluded: BTreeMap<String, usize> = BTreeMap::new();
    let mut compared = Vec::new();
    let mut counts = Counts {
        tier_a: TIER_A,
        from_ork: entries.len(),
        ..Counts::default()
    };
    let mut logged = Logged::default();
    let mut ratios = Vec::new();
    for entry in entries {
        let at = &entry.flight_id;
        counts.assumed_times += usize::from(entry.launch_time_assumed);
        counts.default_rails += usize::from(entry.rail_length_m.is_none());
        let design = design(root, entry, supply)?;
        let chosen = match collection_cause(&design, entry) {
            Ok(chosen) => chosen,
            Err(cause) => {
                *causes.entry((COLLECTION, cause.to_owned())).or_default() += 1;
                continue;
            }
        };
        let weather = weather(root, entry, index)?;
        let site =
            Geodetic::from_degrees(entry.latitude_deg, entry.longitude_deg, entry.elevation_m)
                .map_err(|error| format!("{at}: {error}"))?;
        let earth = Earth::wgs84(site).map_err(|error| format!("{at}: {error}"))?;
        let rail = rail(entry);

        // hpr, in the whole profile and in OpenRocket's air.
        let hpr = match &chosen.left_out {
            Some(out) => Err(named_cause(
                &out.message,
                crate::ork_motors::not_flown(out.why),
            )),
            None => {
                let pad = weather
                    .sounding
                    .air(entry.elevation_m)
                    .map_err(|error| format!("{at}: {error}"))?
                    .air;
                let anchored =
                    Ussa76::anchored(entry.elevation_m, pad.temperature_k, pad.pressure_pa)
                        .map_err(|error| format!("{at}: {error}"))?;
                let day = Environment::new(earth, weather.sounding.clone(), weather.wind.clone());
                let standard = Environment::new(earth, anchored, weather.wind.clone());
                hpr_apogee(&design, chosen, day, rail)
                    .and_then(|day| Ok((day, hpr_apogee(&design, chosen, standard, rail)?)))
                    .map_err(|error| {
                        let cause = named_cause(&error, FLIGHT_FAILED);
                        if cause == FLIGHT_FAILED {
                            eprintln!("{at}: {FLIGHT_FAILED}: {error}");
                        }
                        cause
                    })
            }
        };
        let openrocket = record
            .get(at)
            .ok_or_else(|| format!("{RECORD} has no flight {at}; run {ORACLE} again"))?;
        let openrocket = match (
            openrocket["refused"].as_str(),
            openrocket["apogee_m"].as_f64(),
        ) {
            (Some(why), _) => Err(why.to_owned()),
            (None, Some(apogee)) if apogee.is_finite() => Ok(apogee),
            _ => {
                return Err(format!(
                    "{RECORD}: {at} has neither an apogee nor a refusal"
                ));
            }
        };
        counts.flown_by_hpr += usize::from(hpr.is_ok());
        counts.flown_by_openrocket += usize::from(openrocket.is_ok());
        if let Err(cause) = &hpr {
            *causes.entry((HPR, cause.clone())).or_default() += 1;
        }
        if let Err(cause) = &openrocket {
            *causes.entry((OPENROCKET, cause.clone())).or_default() += 1;
        }
        let (Ok((hpr_m, hpr_standard_m)), Ok(openrocket_m)) = (hpr, openrocket) else {
            continue;
        };
        counts.flown_by_both += 1;
        if let Some(class) = &entry.excluded_class {
            *excluded.entry(class.clone()).or_default() += 1;
            continue;
        }

        // The logged apogee as a height climbed in the day's air.
        let logged_m = match entry.logged_kind {
            LoggedKind::Gps => {
                logged.gps += 1;
                entry.logged_apogee_m
            }
            kind => {
                if kind == LoggedKind::Fused {
                    logged.fused += 1;
                } else {
                    logged.barometric += 1;
                }
                let barometer = Barometer::on_pad(&weather.sounding, entry.elevation_m, 0.0)
                    .map_err(|error| format!("{at}: {error}"))?;
                let height = barometer
                    .height_m(entry.logged_apogee_m)
                    .map_err(|error| format!("{at}: {error}"))?;
                ratios.push(height / entry.logged_apogee_m);
                height
            }
        };
        let early = entry.early_deployment == Some(true);
        counts.early_deployments += usize::from(early);
        counts.early_unknown += usize::from(entry.early_deployment.is_none());
        let row = Compared {
            hpr_percent: percent(hpr_m, logged_m),
            openrocket_percent: percent(openrocket_m, logged_m),
            hpr_in_openrocket_air_percent: percent(hpr_standard_m, logged_m),
            hpr_over_openrocket_percent: percent(hpr_m, openrocket_m),
            early,
        };
        println!(
            "fixture-flights: {at:<48} hpr {:+7.2}%  OpenRocket {:+7.2}%  hpr in its air {:+7.2}%{}",
            row.hpr_percent,
            row.openrocket_percent,
            row.hpr_in_openrocket_air_percent,
            if early { "  early deployment" } else { "" }
        );
        compared.push(row);
    }
    counts.excluded = excluded.values().sum();
    counts.compared = compared.len();
    logged.height_over_reading = Spread::of(&ratios);

    let mut not_flown = Vec::new();
    let mut missing = Vec::new();
    for ((who, cause), flights) in causes {
        let issue = ISSUES
            .iter()
            .find(|(w, c, _)| *w == who && *c == cause)
            .map(|(_, _, issue)| *issue);
        if issue.is_none() {
            missing.push(format!("{who}: {cause} ({flights})"));
        }
        not_flown.push(Cause {
            who: who.to_owned(),
            cause,
            flights,
            issue,
        });
    }
    if !missing.is_empty() {
        return Err(format!(
            "these causes have no issue in fixture_flights::ISSUES: {}",
            missing.join("; ")
        ));
    }
    let set = |name: &str, rows: Vec<&Compared>| ErrorSet {
        name: name.to_owned(),
        flights: rows.len(),
        hpr: Stats::of(rows.iter().map(|r| r.hpr_percent)),
        openrocket: Stats::of(rows.iter().map(|r| r.openrocket_percent)),
        hpr_in_openrocket_air: Stats::of(rows.iter().map(|r| r.hpr_in_openrocket_air_percent)),
        hpr_over_openrocket: Stats::of(rows.iter().map(|r| r.hpr_over_openrocket_percent)),
    };
    let sets = vec![
        set(EVERY_FLIGHT, compared.iter().collect()),
        set(
            WITHOUT_EARLY,
            compared.iter().filter(|row| !row.early).collect(),
        ),
    ];
    Ok(Report {
        milestone: "M2.3c1".to_owned(),
        decision: "ADR-184".to_owned(),
        counts,
        not_flown,
        excluded: excluded
            .into_iter()
            .map(|(reason, flights)| Tally { reason, flights })
            .collect(),
        logged,
        sets,
    })
}

/// The two sets of flights the errors are taken over.
pub(crate) const EVERY_FLIGHT: &str = "every flight compared";
pub(crate) const WITHOUT_EARLY: &str = "without the early deployments";

/// The committed report: aggregates only, no flight named.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Report {
    pub milestone: String,
    pub decision: String,
    pub counts: Counts,
    /// Each cause of a flight not flown, by who did not fly it, with its issue.
    pub not_flown: Vec<Cause>,
    /// The flights both flew but whose log is no fair test, by why.
    pub excluded: Vec<Tally>,
    pub logged: Logged,
    pub sets: Vec<ErrorSet>,
}

/// How many flights went where.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Counts {
    pub tier_a: usize,
    pub from_ork: usize,
    pub flown_by_hpr: usize,
    pub flown_by_openrocket: usize,
    pub flown_by_both: usize,
    pub excluded: usize,
    pub compared: usize,
    /// Of those compared, the flights whose log or notes show a deployment before apogee.
    pub early_deployments: usize,
    /// Of those compared, the flights where that is not known.
    pub early_unknown: usize,
    /// Of all, the flights whose launch hour is not known (flown at local noon).
    pub assumed_times: usize,
    /// Of all, the flights flown on [`DEFAULT_RAIL_M`].
    pub default_rails: usize,
}

/// A cause of flights not flown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Cause {
    pub who: String,
    pub cause: String,
    pub flights: usize,
    pub issue: Option<u64>,
}

/// A count under a reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Tally {
    pub reason: String,
    pub flights: usize,
}

/// What the compared logs are, and how far the day's air moves a barometric reading.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Logged {
    pub barometric: usize,
    pub fused: usize,
    pub gps: usize,
    /// The height climbed over the reading, for the barometric and fused logs.
    pub height_over_reading: Spread,
}

/// The mean, least and most of some ratios.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Spread {
    pub mean: f64,
    pub least: f64,
    pub most: f64,
}

impl Spread {
    fn of(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self::default();
        }
        let sum = values.iter().fold(0.0, |sum, value| sum + value);
        Self {
            mean: round(sum / values.len() as f64, 4),
            least: round(values.iter().copied().fold(f64::INFINITY, f64::min), 4),
            most: round(values.iter().copied().fold(f64::NEG_INFINITY, f64::max), 4),
        }
    }
}

/// One simulator's errors over a set of flights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ErrorSet {
    pub name: String,
    pub flights: usize,
    pub hpr: Stats,
    pub openrocket: Stats,
    /// hpr in OpenRocket's air: the standard atmosphere through the pad's temperature and
    /// pressure, in the same wind; what of the gap is the atmosphere.
    pub hpr_in_openrocket_air: Stats,
    /// hpr's apogee against OpenRocket's, in their own airs.
    pub hpr_over_openrocket: Stats,
}

/// The statistics of some errors, per cent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Stats {
    pub n: usize,
    pub mean_percent: f64,
    pub median_percent: f64,
    pub mean_absolute_percent: f64,
    pub rms_percent: f64,
    pub within_5_percent: usize,
    pub within_10_percent: usize,
    /// Counts in the bins [`BINS_PERCENT`] bounds: below the first bound, between each pair, and
    /// at or above the last.
    pub histogram: Vec<usize>,
}

/// Rounds to `decimals` places, for a report that reads the same on every run.
fn round(value: f64, decimals: i32) -> f64 {
    let scale = 10f64.powi(decimals);
    (value * scale).round() / scale
}

impl Stats {
    pub(crate) fn of(errors: impl Iterator<Item = f64>) -> Self {
        let mut sorted: Vec<f64> = errors.collect();
        sorted.sort_by(f64::total_cmp);
        let n = sorted.len();
        let mean = |values: &mut dyn Iterator<Item = f64>| {
            if n == 0 {
                0.0
            } else {
                values.fold(0.0, |sum, value| sum + value) / n as f64
            }
        };
        let median = match n {
            0 => 0.0,
            _ if n % 2 == 1 => sorted[n / 2],
            _ => 0.5 * (sorted[n / 2 - 1] + sorted[n / 2]),
        };
        let mut histogram = vec![0; BINS_PERCENT.len() + 1];
        for &error in &sorted {
            let bin = BINS_PERCENT.iter().filter(|&&bound| error >= bound).count();
            histogram[bin] += 1;
        }
        Self {
            n,
            mean_percent: round(mean(&mut sorted.iter().copied()), 2),
            median_percent: round(median, 2),
            mean_absolute_percent: round(mean(&mut sorted.iter().map(|e| e.abs())), 2),
            rms_percent: round(mean(&mut sorted.iter().map(|e| e * e)).sqrt(), 2),
            // Strictly within, as the bins are closed below: +5% is in [+5, +10), not the middle.
            within_5_percent: sorted.iter().filter(|e| e.abs() < 5.0).count(),
            within_10_percent: sorted.iter().filter(|e| e.abs() < 10.0).count(),
            histogram,
        }
    }

    /// What must hold of any statistics, whatever the errors were.
    fn consistent(&self, what: &str) -> Result<(), String> {
        let fail = |why: &str| Err(format!("{REPORT_JSON}: {what}: {why}"));
        if self.histogram.len() != BINS_PERCENT.len() + 1
            || self.histogram.iter().sum::<usize>() != self.n
        {
            return fail("the histogram does not count every flight");
        }
        if self.within_5_percent > self.within_10_percent || self.within_10_percent > self.n {
            return fail("the counts within 5% and 10% are out of order");
        }
        // Rounding to 0.01 can tie the three.
        if self.n > 0
            && (self.mean_absolute_percent + 0.01 < self.mean_percent.abs()
                || self.rms_percent + 0.01 < self.mean_absolute_percent)
        {
            return fail("the mean, mean absolute and RMS errors are out of order");
        }
        // The bins around zero hold at least the flights within 5%.
        let central: usize = self.histogram[BINS_PERCENT.len() / 2..BINS_PERCENT.len() / 2 + 2]
            .iter()
            .sum();
        if central < self.within_5_percent {
            return fail("the histogram's middle bins hold fewer than the flights within 5%");
        }
        Ok(())
    }
}

impl Report {
    /// Holds the report to itself: its counts to each other, each set's statistics to its count,
    /// and its page to its data.
    pub(crate) fn check_consistent(&self, page: &str) -> Result<(), String> {
        let c = &self.counts;
        let fail = |why: String| Err(format!("{REPORT_JSON}: {why}"));
        let not_flown = |who: &str| -> usize {
            self.not_flown
                .iter()
                .filter(|cause| cause.who == who)
                .map(|cause| cause.flights)
                .sum()
        };
        if c.from_ork != TIER_A_ORK || c.tier_a != TIER_A {
            return fail(format!(
                "{} of {} flights, not {TIER_A_ORK} of {TIER_A}",
                c.from_ork, c.tier_a
            ));
        }
        let unflyable = not_flown(COLLECTION);
        if c.flown_by_hpr + not_flown(HPR) + unflyable != c.from_ork
            || c.flown_by_openrocket + not_flown(OPENROCKET) + unflyable != c.from_ork
        {
            return fail("each flight is either flown or under a cause".to_owned());
        }
        if c.flown_by_both > c.flown_by_hpr.min(c.flown_by_openrocket)
            || c.flown_by_both + not_flown(HPR) + not_flown(OPENROCKET) + unflyable < c.from_ork
        {
            return fail("the flights flown by both don't follow from each's".to_owned());
        }
        if self.excluded.iter().map(|t| t.flights).sum::<usize>() != c.excluded
            || c.excluded + c.compared != c.flown_by_both
        {
            return fail("the compared and left-out flights don't add up".to_owned());
        }
        if self.not_flown.iter().any(|cause| {
            cause.issue.is_none() || ![HPR, OPENROCKET, COLLECTION].contains(&cause.who.as_str())
        }) {
            return fail("a cause names no issue".to_owned());
        }
        if self
            .excluded
            .iter()
            .any(|tally| !EXCLUSION_CLASSES.contains(&tally.reason.as_str()))
        {
            return fail("a reason for leaving a flight out is not on the list".to_owned());
        }
        let l = &self.logged;
        if l.barometric + l.fused + l.gps != c.compared {
            return fail("each compared log is barometric, fused or GPS".to_owned());
        }
        if c.early_deployments + c.early_unknown > c.compared {
            return fail("more early deployments than flights compared".to_owned());
        }
        let names: Vec<&str> = self.sets.iter().map(|s| s.name.as_str()).collect();
        if names != [EVERY_FLIGHT, WITHOUT_EARLY] {
            return fail(format!("the sets are {names:?}"));
        }
        for (set, flights) in self
            .sets
            .iter()
            .zip([c.compared, c.compared - c.early_deployments])
        {
            if set.flights != flights {
                return fail(format!(
                    "{} holds {} flights, not {flights}",
                    set.name, set.flights
                ));
            }
            for (what, stats) in [
                ("hpr", &set.hpr),
                ("OpenRocket", &set.openrocket),
                ("hpr in OpenRocket's air", &set.hpr_in_openrocket_air),
                ("hpr against OpenRocket", &set.hpr_over_openrocket),
            ] {
                if stats.n != flights {
                    return fail(format!("{}: {what} counts {} flights", set.name, stats.n));
                }
                stats.consistent(&format!("{}: {what}", set.name))?;
            }
        }
        if page != self.to_markdown() {
            return Err(format!(
                "{REPORT_MD} is not the page of {REPORT_JSON}; run `cargo xtask fixture-flights`"
            ));
        }
        Ok(())
    }

    /// The report's page.
    pub(crate) fn to_markdown(&self) -> String {
        let c = &self.counts;
        let mut page = String::new();
        let _ = writeln!(
            page,
            "# Real flights of the private collection: logged apogees ({}, {})\n",
            self.milestone, self.decision
        );
        let _ = writeln!(
            page,
            "Written by `cargo xtask fixture-flights`; do not edit. The flights are the tier-A \
             flights of a private collection of designs paired with logs of the same flight \
             (ADR-151). Its license allows only aggregate statistics, so no flight is named or \
             numbered here.\n"
        );
        let _ = writeln!(page, "## Flights\n");
        let _ = writeln!(page, "| | flights |\n|---|---:|");
        for (what, n) in [
            ("tier A", c.tier_a),
            ("from a `.ork`", c.from_ork),
            ("flown by hpr", c.flown_by_hpr),
            ("flown by OpenRocket 24.12", c.flown_by_openrocket),
            ("flown by both", c.flown_by_both),
            ("left out of the errors (below)", c.excluded),
            ("compared with the log", c.compared),
            ("of those, with an early deployment", c.early_deployments),
            ("of those, deployment not known", c.early_unknown),
            (
                "of the 83, launch hour not known (local noon flown)",
                c.assumed_times,
            ),
            ("of the 83, rail not known (6 ft flown)", c.default_rails),
        ] {
            let _ = writeln!(page, "| {what} | {n} |");
        }
        if !self.not_flown.is_empty() {
            let _ = writeln!(page, "\n## Not flown\n");
            let _ = writeln!(page, "| by | cause | flights | issue |\n|---|---|---:|---|");
            for cause in &self.not_flown {
                let issue = cause
                    .issue
                    .map_or_else(|| "none".to_owned(), |n| format!("#{n}"));
                let _ = writeln!(
                    page,
                    "| {} | {} | {} | {issue} |",
                    cause.who, cause.cause, cause.flights
                );
            }
        }
        if !self.excluded.is_empty() {
            let _ = writeln!(page, "\n## Left out of the errors\n");
            let _ = writeln!(page, "| reason | flights |\n|---|---:|");
            for tally in &self.excluded {
                let _ = writeln!(page, "| {} | {} |", tally.reason, tally.flights);
            }
        }
        let l = &self.logged;
        let _ = writeln!(page, "\n## The logs\n");
        let _ = writeln!(
            page,
            "{} barometric, {} fused (barometer and accelerometer, read as barometric) and {} \
             satellite (GPS) apogees. Each barometric or fused reading is turned into the height \
             climbed in the day's ERA5 air: the height above the pad that reads it, on an \
             altimeter that converts pressure to the 1976 standard atmosphere's altitude. That \
             height over the reading: mean {:.4}, least {:.4}, most {:.4}.",
            l.barometric,
            l.fused,
            l.gps,
            l.height_over_reading.mean,
            l.height_over_reading.least,
            l.height_over_reading.most
        );
        let _ = writeln!(page, "\n## Apogee errors\n");
        let _ = writeln!(
            page,
            "Each simulator's apogee less the logged one, over the logged one, per cent. hpr \
             flies the whole ERA5 profile; OpenRocket the standard atmosphere through the pad's \
             temperature and pressure, in the same wind every {WIND_STEP_M} m; \"hpr in \
             OpenRocket's air\" is hpr there too; \"hpr against OpenRocket\" is hpr's apogee less \
             OpenRocket's, over OpenRocket's. Nothing deploys in either. The mean absolute \
             error averages the errors without their signs; \"within 5%\" and \"within 10%\" \
             count the errors strictly smaller either way. The histogram's bins are bounded at \
             {}%, each closed below.",
            BINS_PERCENT
                .iter()
                .map(|b| format!("{b:+}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        for set in &self.sets {
            let _ = writeln!(page, "\n### {} ({} flights)\n", set.name, set.flights);
            let _ = writeln!(
                page,
                "| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |\n\
                 |---|---:|---:|---:|---:|---:|---:|---|"
            );
            for (what, s) in [
                ("hpr", &set.hpr),
                ("OpenRocket", &set.openrocket),
                ("hpr in OpenRocket's air", &set.hpr_in_openrocket_air),
                ("hpr against OpenRocket", &set.hpr_over_openrocket),
            ] {
                let _ = writeln!(
                    page,
                    "| {what} | {:+.2}% | {:+.2}% | {:.2}% | {:.2}% | {} | {} | {} |",
                    s.mean_percent,
                    s.median_percent,
                    s.mean_absolute_percent,
                    s.rms_percent,
                    s.within_5_percent,
                    s.within_10_percent,
                    s.histogram
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
        let _ = writeln!(page, "\n## Sources\n");
        let _ = writeln!(
            page,
            "Flight data published by university rocketry teams, rocketry courses and hobbyists \
             on GitHub, team websites and The Rocketry Forum.\n"
        );
        let _ = writeln!(
            page,
            "Generated using Copernicus Climate Change Service information 2026. Contains \
             modified Copernicus Climate Change Service information 2026 (the Earth Data Hub \
             subsets). Neither the European Commission nor ECMWF is responsible for any use that \
             may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. \
             et al. (2020), *The ERA5 global reanalysis*, Q. J. R. Meteorol. Soc. 146, \
             1999–2049, <https://doi.org/10.1002/qj.3803>; ERA5 hourly data on single levels \
             (<https://doi.org/10.24381/cds.adbb2d47>) and on pressure levels \
             (<https://doi.org/10.24381/cds.bd0915c6>), Copernicus Climate Data Store."
        );
        page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed report holds to itself, here and in CI, where the flights can't be flown.
    #[test]
    fn the_committed_report_holds_to_itself() {
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(REPORT_JSON)).unwrap();
        let report: Report = serde_json::from_str(&text).unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        report.check_consistent(&page).unwrap();
    }

    /// The statistics of four errors worked by hand: mean (−12 + 3 + 4 + 25) / 4 = 5, median
    /// (3 + 4) / 2 = 3.5, mean absolute 11, RMS √((144 + 9 + 16 + 625) / 4) = √198.5; 3 and 4
    /// within both 5% and 10%, as 12 is past 10.
    #[test]
    fn statistics_of_four_errors() {
        let stats = Stats::of([4.0, -12.0, 25.0, 3.0].into_iter());
        assert_eq!(stats.n, 4);
        assert_eq!(stats.mean_percent, 5.0);
        assert_eq!(stats.median_percent, 3.5);
        assert_eq!(stats.mean_absolute_percent, 11.0);
        assert_eq!(stats.rms_percent, round(198.5f64.sqrt(), 2));
        assert_eq!((stats.within_5_percent, stats.within_10_percent), (2, 2));
        // −12 is in [−15, −10): bin 4; 3 and 4 in [0, 5): bin 7; 25 in [25, 30): bin 12.
        let mut expected = vec![0; 14];
        expected[4] = 1;
        expected[7] = 2;
        expected[12] = 1;
        assert_eq!(stats.histogram, expected);
        stats.consistent("four errors").unwrap();
        // An error of exactly 5% falls in the bin above the middle two, and is not within 5%.
        let edges = Stats::of([5.0, -5.0, 10.0].into_iter());
        assert_eq!((edges.within_5_percent, edges.within_10_percent), (0, 2));
        edges.consistent("errors on the bounds").unwrap();
        // A bound belongs to the bin above it; past the last, the last bin.
        assert_eq!(Stats::of([0.0].into_iter()).histogram[7], 1);
        assert_eq!(Stats::of([30.0].into_iter()).histogram[13], 1);
        assert_eq!(Stats::of([-31.0].into_iter()).histogram[0], 1);
    }

    /// A histogram that misses a flight, or counts out of order, is caught.
    #[test]
    fn inconsistent_statistics_are_caught() {
        let mut stats = Stats::of([1.0, 2.0, 30.0].into_iter());
        stats.histogram[13] = 0;
        assert!(stats.consistent("x").is_err());
        let mut stats = Stats::of([1.0, 2.0, 30.0].into_iter());
        stats.within_5_percent = 3;
        assert!(stats.consistent("x").is_err());
    }

    /// A refusal's words name its cause; words no cause names keep the generic kind, which no
    /// issue tracks; and every cause named has an issue.
    #[test]
    fn refusals_name_their_causes() {
        assert_eq!(
            named_cause(
                "p-3: inconsistent geometry: a fin tab must lie along the root chord (1 m)",
                FLIGHT_FAILED
            ),
            TAB_PAST_ROOT
        );
        assert_eq!(
            named_cause(
                "no thrust curve for X: no embedded curve",
                "a motor with no curve"
            ),
            NO_CURVE
        );
        assert_eq!(named_cause("something new", FLIGHT_FAILED), FLIGHT_FAILED);
        assert!(!ISSUES.iter().any(|(_, cause, _)| *cause == FLIGHT_FAILED));
        for (_, cause) in HPR_CAUSES {
            assert!(
                ISSUES.iter().any(|(who, c, _)| *who == HPR && *c == cause),
                "{cause}"
            );
        }
    }

    /// Designations match whatever their case, dashes and the manufacturer's prefix.
    #[test]
    fn designations_match_loosely() {
        assert_eq!(
            designation_key("Maker K1000-Xtra"),
            designation_key("K1000XTRA")
        );
        assert_eq!(designation_key("Maker M5000XY"), "m5000xy");
        assert_ne!(designation_key("J300Z"), designation_key("J400Z"));
        assert_eq!(designation_key("H100Z-M"), "h100z");
        assert_eq!(designation_key("J300Z-14A"), "j300z");
        // A total impulse before the class letter, and a note in brackets after it.
        assert_eq!(designation_key("Maker 1200K1000-Xtra-14A"), "k1000xtra");
        assert_eq!(designation_key("K1000Z (the K1000 Z page)"), "k1000z");
        assert_eq!(designation_key("K1000 Long Name (1200K1000-P)"), "k1000");
        assert_eq!(designation_key("a note (1200K1000Z-P)"), "k1000z");
    }
}
