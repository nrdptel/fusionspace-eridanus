//! Predictions merged before their errors: the public real flights that join the seven of
//! [`crate::real_flight`], each flown from its record alone ([M2.3d3][m2-3d3], decisions
//! [ADR-328][adr-328] §1a and [ADR-334][adr-334]).
//!
//! A flight here has a design, a motor, a mass, a site, a rail and its day's weather, each from a
//! named record, and a *rule* for the apogee its log will be read at. It has no measured apogee:
//! nothing in this module parses a flight log, and neither [`PredictionRow`] nor
//! [`PredictionsReport`] has a field for a measured height or an error. The log's bytes are
//! hashed, so the report pins the file the rule will later be applied to, and never read as
//! numbers. The comparison is a later pull request's ([M2.3d2][m2-3d2]), which must print each
//! prediction here beside its error and may not change one without saying so.
//!
//! What is flown is what [`crate::real_flight`] flies for the seven: the design under
//! `validation/designs/` built from the example's inputs, on the example's own thrust file with
//! its burn time, from the example's rail and site, in the ERA5 air of the launch hour, on HPR
//! Sim's own aerodynamics. The three EuRoC 2022 flights' weather file is netCDF-4, which
//! [`hpr_io::netcdf`] refuses; `validation/oracles/netcdf/euroc_2022.py` converts it as the guide
//! says and checks the result, and [`CONVERSION_RECORD`] holds both files' SHA-256. The converted
//! file is flown only if its SHA-256 is the committed one.
//!
//! The predicted apogee is read as the log's altimeter would read it ([`Barometer`]: the standard
//! atmosphere's altitude of the day's pressure at the apogee, less the pad's), because that is
//! what it will be compared with; the height climbed is kept beside it.
//!
//! [m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3
//! [m2-3d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d2
//! [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
//! [adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md

use std::fmt::Write as _;
use std::path::Path;

use hpr_design::Rocket;
use hpr_sim::{EventKind, FlightMetrics, FlightSettings, Simulation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::real_flight::{
    Altimeter, Barometer, FileRead, Heights, MASS_FIXTURE, ROCKETPY, RealFlightError,
    example_thrust_of, fixture_case_of, fmt_number, launch_day_at, parse_thrust, rail_from,
    read_bytes, rocket_with_curve, sha256_hex,
};

/// The predictions, as data.
pub const PREDICTIONS_JSON: &str = "validation/reports/public-predictions.json";

/// The predictions, as a page.
pub const PREDICTIONS_MD: &str = "validation/reports/public-predictions.md";

/// The record of the EuRoC 2022 weather file's conversion: the source's and the converted file's
/// path and SHA-256, written by `validation/oracles/netcdf/euroc_2022.py`.
pub const CONVERSION_RECORD: &str = "validation/fixtures/weather/euroc-2022-conversion.json";

/// How long a prediction flies at most, s. Every flight here reaches its apogee in under a
/// minute; with no parachute in the design it then falls to the ground and stops.
const MAX_TIME_S: f64 = 600.0;

/// What "traced" means in [`PredictedFlight::untraced`], as the page says it.
pub const TRACED_RULE: &str = "An input is traced when the flight's own record states it: here \
    RocketPy's example notebook, which holds the team's inputs. What no RocketPy example records, \
    the surface finish and the fin edges, is a placeholder in every design built from one and \
    doesn't count against the tag. A flight is tagged not traced when its record leaves out, or \
    contradicts, something that moves the apogee.";

/// The apogee a flight's log will be read at: the rule, never the value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference {
    /// The log, under `refs/rocketpy/data/rockets/`. Its bytes are hashed and never parsed here.
    pub log: &'static str,
    /// The rule, in words: which column, read how far.
    pub rule: &'static str,
    /// What the log's height is taken to be, and so how the prediction is read.
    pub altimeter: Altimeter,
}

/// A second prediction shown beside a flight's counted one and never counted: the design flown
/// as committed, on its own bundled catalog curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogCurve {
    /// The motor, as the catalog names it.
    pub motor: &'static str,
    /// Why it is shown, and why it is not the counted one.
    pub why: &'static str,
}

/// One flight predicted before its log is compared.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictedFlight {
    /// The flight's public id, the one the real-flights report will list it under.
    pub id: &'static str,
    /// The rocket, the team and the event, in words.
    pub title: &'static str,
    /// The design flown, by file stem under `validation/designs/`, which is also its case name in
    /// [`MASS_FIXTURE`].
    pub design: &'static str,
    /// Where the site, date, rail and geometry come from, in RocketPy 1.13.0's notebook.
    pub source: &'static str,
    /// The motor, as its record names it.
    pub motor: &'static str,
    /// Where the thrust file and its burn time come from.
    pub motor_source: &'static str,
    /// Where the mass comes from.
    pub mass_source: &'static str,
    /// The site: latitude and longitude, degrees, and elevation above sea level, m.
    pub site: (f64, f64, f64),
    /// The launch hour the weather is read at, UTC: year, month, day, hour.
    pub utc: (i32, u32, u32, u32),
    /// The rail: length, m; inclination from the horizontal and heading from north, degrees.
    pub rail: (f64, f64, f64),
    /// The apogee rule.
    pub reference: Reference,
    /// What moves the apogee and no record states, or a record contradicts; empty when every
    /// input is traced ([`TRACED_RULE`]).
    pub untraced: &'static [&'static str],
    /// The catalog curve's prediction, where the counted curve is not the catalog's.
    pub catalog_curve: Option<CatalogCurve>,
    /// What else a reader should know before the comparison; empty when nothing.
    pub note: &'static str,
}

/// The flights, in the order the page lists them: EuRoC 2022's three.
///
/// Each site, date and rail is the example notebook's (`docs/examples/<name>_flight_sim.ipynb` in
/// RocketPy 1.13.0). The notebooks give local time with `timezone="Portugal"`: Western European
/// Summer Time, UTC+1, until 30 October 2022, so 16 h is 15:00Z and 14 h is 13:00Z.
///
/// Every rule and tag here was written before any of the three was flown, and each log was looked
/// at only for its columns and for a transient near its peak (none: each peak is the whole log's
/// highest reading).
pub const NEW_FLIGHTS: [PredictedFlight; 3] = [
    PredictedFlight {
        id: "astra",
        title: "Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350)",
        design: "rocketpy-astra",
        source: "astra_flight_sim.ipynb:123-131 (site, date), :247-280 (rocket, rail buttons, \
                 motor position, nose, fins), :316-318 (rail)",
        motor: "Cesaroni 4263L1350-P",
        motor_source: "astra_flight_sim.ipynb:155-157: RocketPy's copy of the certified curve, \
                       cut at the notebook's burn time",
        mass_source: "astra_flight_sim.ipynb:249-251 (mass, inertia and center without the motor), \
                      :158-162 (the grain, whose density and size give the propellant), :168-170 \
                      (no motor dry mass or inertia)",
        site: (39.3897, -8.288_963_9, 160.0),
        utc: (2022, 10, 13, 15),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "astra/flight_data.csv",
            rule: "the highest reading of the log's `alt` column, in meters, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter's flight (astra_flight_sim.ipynb:383-385), \
                 one row a second beside a latitude and a longitude, and records no more: \
                 barometric assumed, as hobby altimeters are",
            ),
        },
        untraced: &[],
        catalog_curve: None,
        note: "",
    },
    PredictedFlight {
        id: "andromeda",
        title: "Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve)",
        design: "rocketpy-andromeda",
        source: "andromeda_flight_sim.ipynb:123-131 (site, date), :244-277 (rocket, rail buttons, \
                 motor position, nose, fins), :388-390 (rail)",
        motor: "the team's thrust file, flown in the notebook under the name of a Cesaroni M2020",
        motor_source: "andromeda_flight_sim.ipynb:153 and :162-165: \"obtained by the team, after \
                       testing\", cut at the notebook's burn time; the catalog file is commented \
                       out at :163",
        mass_source: "andromeda_flight_sim.ipynb:246-248 (mass, inertia and center without the \
                      motor), :166-170 (the grain), :176-178 (no motor dry mass or inertia)",
        site: (39.388_692, -8.287_814, 160.0),
        utc: (2022, 10, 14, 13),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "andromeda/flight_data.csv",
            rule: "the highest reading of the log's `alt(m)` column, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter (andromeda_flight_sim.ipynb:459-461), \
                 logged in feet with a vertical speed, and records no more: barometric assumed, as \
                 hobby altimeters are",
            ),
        },
        untraced: &[
            "the flight carried the team's air brakes, and no record says when or how far they \
             opened: none is flown here, so any opening leaves this prediction high",
            "the thrust file holds more impulse than the Cesaroni M2020 reload it is named for is \
             certified at, so the notebook does not settle which motor the logged flight burned",
        ],
        catalog_curve: Some(CatalogCurve {
            motor: "Cesaroni 8429M2020-P",
            why: "the reload the notebook names, on its certified curve as bundled, whole. Shown \
                  beside the counted prediction and never counted: every RocketPy flight here \
                  is counted on its example's own thrust file",
        }),
        note: "both tags are the research note's findings (docs/research/andromeda-impulse.md), \
               written down before this flight was flown",
    },
    PredictedFlight {
        id: "erebus-11",
        title: "Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K)",
        design: "rocketpy-erebus-11",
        source: "erebus_flight_sim.ipynb:123-131 (site, date), :251-284 (rocket, rail buttons, \
                 motor position, nose, fins, tail), :320-322 (rail)",
        motor: "a Cesaroni Pro54 K-class motor, as the notebook says; the team's thrust file",
        motor_source: "erebus_flight_sim.ipynb:153 and :162-165, cut at the notebook's burn time",
        mass_source: "erebus_flight_sim.ipynb:253-255 (mass, inertia and center without the \
                      motor), :166-172 (the grain), :163 and :173-174 (no motor dry mass or \
                      inertia)",
        site: (39.3897, -8.288_963_9, 160.0),
        utc: (2022, 10, 15, 15),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "erebus11/flight_data_filtered.csv",
            rule: "the highest reading of the log's `alt` column, in meters, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter (erebus_flight_sim.ipynb:390-392), a file \
                 named filtered, and records no more: barometric assumed, as hobby altimeters are",
            ),
        },
        untraced: &[],
        catalog_curve: None,
        note: "",
    },
];

/// The catalog curve's prediction, as the report keeps it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogCurveRow {
    /// The motor, as the catalog names it.
    pub motor: String,
    /// Why it is shown and not counted.
    pub why: String,
    /// The impulse flown, N s.
    pub total_impulse_ns: f64,
    /// Its predicted apogee as the log's altimeter would read it, m. Never counted.
    pub predicted_apogee_m: f64,
    /// Its predicted apogee as a height above the pad, m.
    pub predicted_height_apogee_m: f64,
}

/// One flight's prediction, as the report keeps it. It has no field for a measured apogee or an
/// error, on purpose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionRow {
    /// The flight's public id.
    pub id: String,
    /// The rocket, team and event.
    pub title: String,
    /// The design flown.
    pub design: String,
    /// Where the site, date, rail and geometry come from.
    pub source: String,
    /// The motor, as its record names it.
    pub motor: String,
    /// Where the thrust file and its burn time come from.
    pub motor_source: String,
    /// The thrust file flown, from the repository root.
    pub thrust_file: String,
    /// The burn time the example cuts it at, s.
    pub burn_time_s: Option<f64>,
    /// The impulse flown, N s.
    pub total_impulse_ns: f64,
    /// Where the mass comes from.
    pub mass_source: String,
    /// The mass the record gives without the motor, kg.
    pub mass_without_motor_kg: f64,
    /// The mass flown at liftoff, kg: the above and the grain.
    pub liftoff_mass_kg: f64,
    /// The site's latitude, degrees north.
    pub latitude_deg: f64,
    /// The site's longitude, degrees east.
    pub longitude_deg: f64,
    /// The site's elevation above sea level, m.
    pub elevation_m: f64,
    /// The rail's length, m.
    pub rail_length_m: f64,
    /// The rail's inclination from the horizontal, degrees.
    pub rail_inclination_deg: f64,
    /// The rail's heading from north, degrees.
    pub rail_heading_deg: f64,
    /// The hour the weather is read at.
    pub weather_time_utc: String,
    /// The log the rule will be applied to, from the repository root.
    pub reference_log: String,
    /// That log's SHA-256: its bytes hashed, never parsed.
    pub reference_log_sha256: String,
    /// The apogee rule.
    pub reference_rule: String,
    /// What the log's height is taken to be.
    pub altimeter: String,
    /// Where that comes from.
    pub altimeter_evidence: String,
    /// Whether every input is traced to the record.
    pub all_inputs_traced: bool,
    /// What isn't, in words.
    pub untraced: Vec<String>,
    /// The predicted apogee as the log's altimeter would read it, m: the counted prediction.
    pub predicted_apogee_m: f64,
    /// The predicted apogee as a height above the pad, m.
    pub predicted_height_apogee_m: f64,
    /// The predicted time from ignition to apogee, s.
    pub predicted_time_to_apogee_s: f64,
    /// The flight's largest Mach number.
    pub hpr_max_mach: f64,
    /// The catalog curve's prediction, shown beside and never counted.
    pub catalog_curve: Option<CatalogCurveRow>,
    /// Every file read, with its SHA-256; the log's is its bytes' alone.
    pub files: Vec<FileRead>,
    /// The SHA-256 of the flight's entry in [`NEW_FLIGHTS`].
    pub inputs_sha256: String,
    /// What else a reader should know.
    pub note: String,
}

/// The weather file's conversion, as [`CONVERSION_RECORD`] holds it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeatherConversion {
    /// The record's path.
    pub record: String,
    /// The conversion, as code.
    pub conversion: String,
    /// The netCDF-4 file, from the repository root.
    pub source: String,
    /// Its SHA-256.
    pub source_sha256: String,
    /// The converted file flown, from the repository root.
    pub converted: String,
    /// Its SHA-256.
    pub converted_sha256: String,
}

/// The predictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionsReport {
    /// What wrote it.
    pub generator: String,
    /// What "traced" means.
    pub traced_rule: String,
    /// The EuRoC 2022 weather file's conversion.
    pub weather_conversion: WeatherConversion,
    /// The flights, in [`NEW_FLIGHTS`]' order.
    pub flights: Vec<PredictionRow>,
}

/// What one flight of a rocket gave.
struct Flown {
    apogee_m: f64,
    height_apogee_m: f64,
    time_to_apogee_s: f64,
    max_mach: f64,
    total_impulse_ns: f64,
    liftoff_mass_kg: f64,
}

/// Reads [`CONVERSION_RECORD`].
fn conversion(root: &Path) -> Result<WeatherConversion, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: "the EuRoC 2022 weather".to_owned(),
        what,
    };
    let bytes = read_bytes(root, CONVERSION_RECORD, "reading the conversion's record")?;
    let record: Value = serde_json::from_slice(&bytes)
        .map_err(|error| input(format!("{CONVERSION_RECORD}: {error}")))?;
    let text = |pointer: &str| -> Result<String, RealFlightError> {
        record
            .pointer(pointer)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| input(format!("{CONVERSION_RECORD} has no {pointer}")))
    };
    Ok(WeatherConversion {
        record: CONVERSION_RECORD.to_owned(),
        conversion: text("/conversion")?,
        source: text("/source/path")?,
        source_sha256: text("/source/sha256")?,
        converted: text("/converted/path")?,
        converted_sha256: text("/converted/sha256")?,
    })
}

/// Predicts one flight: flies it from its record and reads its apogee as its log's altimeter
/// would. The log is hashed, never parsed.
///
/// # Errors
///
/// [`RealFlightError`] if a file is missing or doesn't read, the converted weather file is not
/// the committed conversion, or HPR Sim can't fly it.
pub fn predict(
    root: &Path,
    flight: &PredictedFlight,
    weather: &WeatherConversion,
) -> Result<PredictionRow, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: flight.id.to_owned(),
        what,
    };
    let sim = |source: hpr_sim::SimError| RealFlightError::Sim {
        flight: flight.id.to_owned(),
        source,
    };
    let mut files = Vec::new();
    let mut read = |relative: String, what: &'static str| -> Result<Vec<u8>, RealFlightError> {
        let bytes = read_bytes(root, &relative, what)?;
        files.push(FileRead {
            sha256: sha256_hex(&bytes),
            path: relative,
        });
        Ok(bytes)
    };

    // The design, with the example's own thrust and burn time from the fixture.
    read(MASS_FIXTURE.to_owned(), "reading the design fixture")?;
    let case = fixture_case_of(root, flight.id, flight.design)?;
    let mass_without_motor_kg = case["rocket"]["mass"]
        .as_f64()
        .ok_or_else(|| input(format!("{MASS_FIXTURE} gives {} no mass", flight.design)))?;
    let (thrust_path, burn, reshape) = example_thrust_of(root, flight.id, flight.design)?;
    if reshape.is_some() {
        return Err(input(
            "the example reshapes its thrust file, which no prediction here expects".to_owned(),
        ));
    }
    let thrust_bytes = read(thrust_path.clone(), "reading a thrust file")?;
    let (curve, _) = parse_thrust(
        flight.id,
        &thrust_path,
        &String::from_utf8_lossy(&thrust_bytes),
        burn,
        None,
    )?;
    let design_path = format!("validation/designs/{}.json", flight.design);
    let design_bytes = read(design_path.clone(), "reading a design")?;
    let design: Value = serde_json::from_slice(&design_bytes)
        .map_err(|error| input(format!("{design_path}: {error}")))?;
    let rocket = rocket_with_curve(flight.id, &design, &design_path, &thrust_path, &curve)?;

    // The weather: the converted file, only if it is the committed conversion.
    let weather_bytes = read(weather.converted.clone(), "reading the converted ERA5 file")?;
    if sha256_hex(&weather_bytes) != weather.converted_sha256 {
        return Err(input(format!(
            "{} is not the conversion {CONVERSION_RECORD} records: run \
             validation/oracles/netcdf/euroc_2022.py",
            weather.converted
        )));
    }
    let (environment, air) = launch_day_at(
        flight.id,
        flight.site,
        flight.utc,
        &weather.converted,
        &weather_bytes,
    )?;
    let (latitude_deg, longitude_deg, elevation_m) = flight.site;
    let rail = rail_from(flight.rail);
    let settings = FlightSettings {
        max_time_s: MAX_TIME_S,
        ..FlightSettings::default()
    };

    // The log: its bytes' SHA-256, and nothing else of it.
    let log_path = format!("{ROCKETPY}/data/rockets/{}", flight.reference.log);
    let log_sha256 = sha256_hex(&read(log_path.clone(), "hashing a flight log")?);

    let fly_once = |rocket: &Rocket| -> Result<Flown, RealFlightError> {
        let simulation =
            Simulation::new(rocket, "example", environment.clone(), rail, settings).map_err(sim)?;
        let total_impulse_ns = simulation
            .assembly()
            .motors
            .iter()
            .map(|motor| motor.mounted.motor.curve().total_impulse_ns())
            .sum();
        let liftoff_mass_kg = simulation.assembly().mass_properties(0.0).mass_kg;
        let mut heights = Heights {
            next: 0,
            rows: Vec::new(),
        };
        let mut metrics = FlightMetrics::new();
        let result = simulation
            .run(&mut (&mut heights, &mut metrics))
            .map_err(sim)?;
        let max_mach = metrics
            .summary(&result, &environment)
            .map_err(sim)?
            .max_mach
            .ok_or_else(|| input("the flight has no Mach number".to_owned()))?
            .value;
        let apogee = result
            .event(EventKind::Apogee)
            .ok_or_else(|| input("the flight has no apogee".to_owned()))?
            .sample;
        let start_m = heights
            .rows
            .first()
            .map(|row| row.1)
            .ok_or_else(|| input("the flight has no steps".to_owned()))?;
        // What the log's altimeter would read at the apogee, above its start.
        let atmos = |error: hpr_atmos::AtmosError| input(format!("{}: {error}", weather.converted));
        let apogee_m = match flight.reference.altimeter {
            Altimeter::Height(_) => apogee.height_above_ground_m - start_m,
            Altimeter::Barometric(_) | Altimeter::AssumedBarometric(_) => {
                Barometer::on_pad(&air, elevation_m, start_m)
                    .map_err(atmos)?
                    .reading_m(apogee.height_above_ground_m)
                    .map_err(atmos)?
            }
        };
        Ok(Flown {
            apogee_m,
            height_apogee_m: apogee.height_above_ground_m - start_m,
            time_to_apogee_s: apogee.time_s,
            max_mach,
            total_impulse_ns,
            liftoff_mass_kg,
        })
    };
    let counted = fly_once(&rocket)?;
    // The design as committed, on its own bundled catalog curve: shown, never counted.
    let catalog_curve = match flight.catalog_curve {
        Some(catalog) => {
            let as_committed: Rocket = serde_json::from_value(design.clone())
                .map_err(|error| input(format!("{design_path}: {error}")))?;
            let flown = fly_once(&as_committed)?;
            Some(CatalogCurveRow {
                motor: catalog.motor.to_owned(),
                why: catalog.why.to_owned(),
                total_impulse_ns: flown.total_impulse_ns,
                predicted_apogee_m: flown.apogee_m,
                predicted_height_apogee_m: flown.height_apogee_m,
            })
        }
        None => None,
    };
    let (year, month, day, hour) = flight.utc;
    let (rail_length_m, rail_inclination_deg, rail_heading_deg) = flight.rail;

    Ok(PredictionRow {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        design: flight.design.to_owned(),
        source: flight.source.to_owned(),
        motor: flight.motor.to_owned(),
        motor_source: flight.motor_source.to_owned(),
        thrust_file: thrust_path,
        burn_time_s: burn,
        total_impulse_ns: counted.total_impulse_ns,
        mass_source: flight.mass_source.to_owned(),
        mass_without_motor_kg,
        liftoff_mass_kg: counted.liftoff_mass_kg,
        latitude_deg,
        longitude_deg,
        elevation_m,
        rail_length_m,
        rail_inclination_deg,
        rail_heading_deg,
        weather_time_utc: format!("{year:04}-{month:02}-{day:02}T{hour:02}:00Z"),
        reference_log: log_path,
        reference_log_sha256: log_sha256,
        reference_rule: flight.reference.rule.to_owned(),
        altimeter: flight.reference.altimeter.kind().to_owned(),
        altimeter_evidence: flight.reference.altimeter.evidence().to_owned(),
        all_inputs_traced: flight.untraced.is_empty(),
        untraced: flight
            .untraced
            .iter()
            .map(|&what| what.to_owned())
            .collect(),
        predicted_apogee_m: counted.apogee_m,
        predicted_height_apogee_m: counted.height_apogee_m,
        predicted_time_to_apogee_s: counted.time_to_apogee_s,
        hpr_max_mach: counted.max_mach,
        catalog_curve,
        files,
        inputs_sha256: sha256_hex(format!("{flight:?}").as_bytes()),
        note: flight.note.to_owned(),
    })
}

/// Predicts every flight in [`NEW_FLIGHTS`].
///
/// # Errors
///
/// The first flight's [`RealFlightError`].
pub fn run(root: &Path) -> Result<PredictionsReport, RealFlightError> {
    let weather_conversion = conversion(root)?;
    let flights = NEW_FLIGHTS
        .iter()
        .map(|flight| predict(root, flight, &weather_conversion))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PredictionsReport {
        generator: "cargo xtask public-predictions".to_owned(),
        traced_rule: TRACED_RULE.to_owned(),
        weather_conversion,
        flights,
    })
}

impl PredictionsReport {
    /// Whether the report holds together without the files it was flown from, as CI checks it:
    /// every prediction is a positive, finite height, each tag is its list's, and the page is this
    /// report's rendering.
    ///
    /// # Errors
    ///
    /// The first thing that doesn't hold, in words.
    pub fn check_consistent(&self, markdown: &str) -> Result<(), String> {
        for row in &self.flights {
            let mut heights = vec![
                ("predicted apogee", row.predicted_apogee_m),
                (
                    "predicted apogee as a height",
                    row.predicted_height_apogee_m,
                ),
                ("time to apogee", row.predicted_time_to_apogee_s),
                ("largest Mach number", row.hpr_max_mach),
                ("impulse", row.total_impulse_ns),
                ("liftoff mass", row.liftoff_mass_kg),
            ];
            if let Some(catalog) = &row.catalog_curve {
                heights.push(("catalog curve's apogee", catalog.predicted_apogee_m));
                heights.push((
                    "catalog curve's apogee as a height",
                    catalog.predicted_height_apogee_m,
                ));
                heights.push(("catalog curve's impulse", catalog.total_impulse_ns));
            }
            if let Some((what, value)) = heights
                .iter()
                .find(|(_, value)| !(value.is_finite() && *value > 0.0))
            {
                return Err(format!("{}: its {what} is {value}", row.id));
            }
            if row.all_inputs_traced != row.untraced.is_empty() {
                return Err(format!("{}: its traced tag is not its list's", row.id));
            }
            if row.liftoff_mass_kg <= row.mass_without_motor_kg {
                return Err(format!(
                    "{}: it lifts off no heavier than without its motor",
                    row.id
                ));
            }
            if row.reference_log_sha256.len() != 64 {
                return Err(format!("{}: its log has no SHA-256", row.id));
            }
        }
        if markdown != self.to_markdown() {
            return Err(format!(
                "{PREDICTIONS_MD} is not {PREDICTIONS_JSON}'s rendering: rerun `cargo xtask \
                 public-predictions`"
            ));
        }
        Ok(())
    }

    /// Whether `other`, flown now, is this committed report: the same words, files and digests,
    /// and the same numbers but for the last digits platforms round differently.
    ///
    /// # Errors
    ///
    /// The first difference, in words.
    pub fn reproduces(&self, other: &PredictionsReport) -> Result<(), String> {
        if (&self.generator, &self.traced_rule, &self.weather_conversion)
            != (
                &other.generator,
                &other.traced_rule,
                &other.weather_conversion,
            )
        {
            return Err("the report's header differs".to_owned());
        }
        if self.flights.len() != other.flights.len() {
            return Err(format!(
                "{} flights committed, {} flown",
                self.flights.len(),
                other.flights.len()
            ));
        }
        let same = crate::report::same_but_for_platform_rounding;
        for (a, b) in self.flights.iter().zip(&other.flights) {
            // Every field that is not a flown number, compared whole.
            let words = |row: &PredictionRow| {
                let mut row = row.clone();
                for value in [
                    &mut row.total_impulse_ns,
                    &mut row.liftoff_mass_kg,
                    &mut row.predicted_apogee_m,
                    &mut row.predicted_height_apogee_m,
                    &mut row.predicted_time_to_apogee_s,
                    &mut row.hpr_max_mach,
                ] {
                    *value = 0.0;
                }
                if let Some(catalog) = &mut row.catalog_curve {
                    catalog.total_impulse_ns = 0.0;
                    catalog.predicted_apogee_m = 0.0;
                    catalog.predicted_height_apogee_m = 0.0;
                }
                row
            };
            if words(a) != words(b) {
                return Err(format!(
                    "{}: its inputs, sources or files differ from the committed prediction's",
                    a.id
                ));
            }
            let mut numbers = vec![
                ("impulse", a.total_impulse_ns, b.total_impulse_ns),
                ("liftoff mass", a.liftoff_mass_kg, b.liftoff_mass_kg),
                (
                    "predicted apogee",
                    a.predicted_apogee_m,
                    b.predicted_apogee_m,
                ),
                (
                    "predicted apogee as a height",
                    a.predicted_height_apogee_m,
                    b.predicted_height_apogee_m,
                ),
                (
                    "time to apogee",
                    a.predicted_time_to_apogee_s,
                    b.predicted_time_to_apogee_s,
                ),
                ("largest Mach number", a.hpr_max_mach, b.hpr_max_mach),
            ];
            if let (Some(x), Some(y)) = (&a.catalog_curve, &b.catalog_curve) {
                numbers.push((
                    "catalog curve's impulse",
                    x.total_impulse_ns,
                    y.total_impulse_ns,
                ));
                numbers.push((
                    "catalog curve's apogee",
                    x.predicted_apogee_m,
                    y.predicted_apogee_m,
                ));
                numbers.push((
                    "catalog curve's apogee as a height",
                    x.predicted_height_apogee_m,
                    y.predicted_height_apogee_m,
                ));
            }
            if let Some((what, x, y)) = numbers.iter().find(|(_, x, y)| !same(*x, *y)) {
                return Err(format!(
                    "{}: the {what} is {x} committed and {y} flown: a merged prediction is not \
                     changed, it is revised beside the old one (ADR-328 §1b)",
                    a.id
                ));
            }
        }
        Ok(())
    }

    /// The predictions as a page.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let weather = &self.weather_conversion;
        let _ = write!(
            out,
            "# Public flights predicted before they are compared\n\n\
             Written by `cargo xtask public-predictions` ([M2.3d3][m2-3d3], decisions \
             [ADR-328][adr-328] and [ADR-334][adr-334]). What it is for is on the \
             [documentation site][site].\n\n\
             - **What this is:** {} public real flights that join the seven of the \
             [real-flights report][real], each flown by HPR Sim from its record alone. This page \
             was merged before any of their logs was compared, so no number here could be tuned \
             to its own error.\n\
             - **What it holds:** each flight's inputs with their sources, the rule its log's \
             apogee will be read by, and HPR Sim's predicted apogee. It holds no measured apogee \
             and no error: those come in a later pull request, which must print each prediction \
             below beside its error.\n\
             - **How it is flown:** by the seven's method. The design is built from the example's \
             inputs and flown on the example's own thrust file, from its rail and site, in the \
             ERA5 air of its launch hour, on HPR Sim's own aerodynamics.\n\
             - **The predicted apogee** is read as the log's altimeter would read it: the standard \
             atmosphere's altitude of the day's pressure at the apogee, less the pad's. The height \
             climbed is beside it.\n\
             - **Traced:** {}\n\
             - **Files:** the examples' thrust files and the logs are read from the pinned \
             RocketPy checkout, and the weather file is converted from the checkout's; none is \
             committed, and each one's SHA-256 is in the JSON. A log's \
             bytes are hashed to pin the file its rule will be applied to; they are never read as \
             numbers here.\n\n\
             [m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3\n\
             [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md\n\
             [adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md\n\
             [site]: https://hpr.fusionspace.co/accuracy.html#predicted-before-compared\n\
             [real]: real-flights.md\n\n",
            self.flights.len(),
            self.traced_rule,
        );
        out.push_str(
            "## Predictions\n\n\
             | flight | impulse flown (N s) | liftoff mass (kg) | predicted apogee (m) | as a \
             height (m) | time to apogee (s) | largest Mach | every input traced |\n\
             |---|---:|---:|---:|---:|---:|---:|---|\n",
        );
        for row in &self.flights {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                row.title,
                fmt_number(row.total_impulse_ns, 1),
                fmt_number(row.liftoff_mass_kg, 3),
                fmt_number(row.predicted_apogee_m, 1),
                fmt_number(row.predicted_height_apogee_m, 1),
                fmt_number(row.predicted_time_to_apogee_s, 2),
                fmt_number(row.hpr_max_mach, 3),
                if row.all_inputs_traced { "yes" } else { "no" },
            );
        }
        let catalog: Vec<(&PredictionRow, &CatalogCurveRow)> = self
            .flights
            .iter()
            .filter_map(|row| row.catalog_curve.as_ref().map(|catalog| (row, catalog)))
            .collect();
        if !catalog.is_empty() {
            out.push_str(
                "\n## On the catalog curve, never counted\n\n\
                 The same design on the bundled certified curve of the motor its record names, \
                 where the counted flight is on another curve. It is here so the reader can see \
                 what the choice of curve moves; it enters no mean.\n\n\
                 | flight | catalog motor | impulse flown (N s) | predicted apogee (m) | as a \
                 height (m) |\n|---|---|---:|---:|---:|\n",
            );
            for (row, catalog) in &catalog {
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | {} | {} |",
                    row.title,
                    catalog.motor,
                    fmt_number(catalog.total_impulse_ns, 1),
                    fmt_number(catalog.predicted_apogee_m, 1),
                    fmt_number(catalog.predicted_height_apogee_m, 1),
                );
            }
            out.push('\n');
            for (row, catalog) in &catalog {
                let _ = writeln!(out, "- **{}:** {}.", row.title, catalog.why);
            }
        }
        out.push_str("\n## Each flight's inputs and its apogee rule\n\n");
        for row in &self.flights {
            let _ = writeln!(
                out,
                "- **{}** (`{}`, design `{}`):\n  \
                 - Design, site and rail: RocketPy 1.13.0's {}. The site is {}°, {}° at {} m; the \
                 rail is {} m long, {}° from the horizontal, heading {}°.\n  \
                 - Motor: {} ({}). Thrust file `{}`{}.\n  \
                 - Mass: {} kg without the motor, {} kg at liftoff ({}).\n  \
                 - Weather: ERA5 at {}, from the converted file below.\n  \
                 - Apogee rule: {}, in `{}`. Altimeter: {} ({}).\n  \
                 - Every input traced: {}{}",
                row.title,
                row.id,
                row.design,
                row.source,
                row.latitude_deg,
                row.longitude_deg,
                row.elevation_m,
                row.rail_length_m,
                row.rail_inclination_deg,
                row.rail_heading_deg,
                row.motor,
                row.motor_source,
                row.thrust_file,
                row.burn_time_s
                    .map_or_else(String::new, |burn| format!(", burn time {burn} s")),
                row.mass_without_motor_kg,
                fmt_number(row.liftoff_mass_kg, 3),
                row.mass_source,
                row.weather_time_utc,
                row.reference_rule,
                row.reference_log,
                row.altimeter,
                row.altimeter_evidence,
                if row.all_inputs_traced {
                    "yes.".to_owned()
                } else {
                    format!("no. Not traced: {}.", row.untraced.join("; "))
                },
                if row.note.is_empty() {
                    String::new()
                } else {
                    format!(" Note: {}.", row.note)
                },
            );
        }
        let _ = write!(
            out,
            "\n## The weather file\n\n\
             The three flights' ERA5 file is netCDF-4, which HPR Sim does not read. \
             `validation/oracles/netcdf/euroc_2022.py` converts it as the [guide][era5] says, \
             `{}`, and checks the result: the four fields hold the source's values bit for bit, \
             and RocketPy reads the same levels from both files at each flight's site and hour. \
             Its record is `{}`.\n\n\
             | file | SHA-256 |\n|---|---|\n\
             | `{}` (netCDF-4, as RocketPy ships it) | `{}` |\n\
             | `{}` (converted, flown) | `{}` |\n\n\
             [era5]: https://hpr.fusionspace.co/format/era5.html#converting-a-current-file\n",
            weather.conversion,
            weather.record,
            weather.source,
            weather.source_sha256,
            weather.converted,
            weather.converted_sha256,
        );
        out
    }
}
