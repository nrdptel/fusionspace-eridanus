//! `cargo xtask real-flights` also writes each public flight's residuals against its simulation
//! (M7.3a, ADR-314): `validation/reports/flight-residuals.{json,md}`.
//!
//! Each flight is flown as `real-flights` flies it, from the example's thrust file and rail, in
//! its ERA5 air and wind and on hpr's own drag, here down to the ground. Its log and the flight go
//! to `hpr_forensics::residuals`. The report keeps each flight's summaries and readings, not its
//! samples: `hpr compare` prints those for a log of your own. The logs are under the gitignored
//! `refs/`, so CI holds only the page to the committed data (below).

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::timeline::{Cell, What};
use hpr_flightdata::{custom, import, readings};
use hpr_forensics::residuals::{self, Comparison, HeightKind, Phase, Row, Speed, Summary, Track};
use hpr_sim::{Device, DeviceDrag, FlightSettings, Simulation, Trigger};
use hpr_validate::real_flight::{
    Altimeter, FLIGHTS, FileRead, ROCKETPY, RealFlight, example_thrust, launch_day, parse_thrust,
    rail_of, read_bytes, rocket_on_curve, sha256_hex,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The report's data.
pub const REPORT_JSON: &str = "validation/reports/flight-residuals.json";
/// The report's page.
pub const REPORT_MD: &str = "validation/reports/flight-residuals.md";
/// The column mappings RocketPy's logs are read by.
const MAPPINGS: &str = "validation/fixtures/logs/rocketpy-mappings";
/// How far two runs' numbers may differ for `--check`: only a float's print and read.
const REPRODUCES_WITHIN: f64 = 1e-6;

/// One flight's residuals, without its samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightResiduals {
    /// The flight's id, as in `real-flights`.
    pub id: String,
    /// Its title.
    pub title: String,
    /// The altimeter's kind, as in `real-flights`.
    pub altimeter: String,
    /// The files read, with their hashes.
    pub files: Vec<FileRead>,
    /// The log's samples read.
    pub log_rows: usize,
    /// What the log's height was taken to stand for.
    pub height_read_as: HeightKind,
    /// How well the clocks are lined up, s.
    pub clock_bound_s: f64,
    /// Whether the log reads a landing: otherwise the residuals stop at its end.
    pub reaches_landing: bool,
    /// Whether the example gives parachutes to fly its descent with: without them only the
    /// ascent is compared.
    pub descent_flown: bool,
    /// The log's samples after the simulated flight had ended.
    pub past_simulation: usize,
    /// The height's residuals by phase, m.
    pub height_m: Vec<Summary>,
    /// The speed's by phase, m/s, if the smoother reads the log.
    pub speed_m_s: Option<Vec<Summary>>,
    /// Why the smoother doesn't read it.
    pub speed_withheld: Option<String>,
    /// Each reading beside the simulation's.
    pub rows: Vec<Row>,
}

/// The report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// The flights.
    pub flights: Vec<FlightResiduals>,
}

/// Flies `flight` to the ground and sets its log against it.
fn fly(root: &Path, flight: &RealFlight) -> Result<FlightResiduals, String> {
    let failed = |error: &dyn std::fmt::Display| format!("{}: {error}", flight.id);
    let mut files = Vec::new();
    let mut read = |relative: String, what: &'static str| -> Result<Vec<u8>, String> {
        let bytes = read_bytes(root, &relative, what).map_err(|error| failed(&error))?;
        files.push(FileRead {
            sha256: sha256_hex(&bytes),
            path: relative,
        });
        Ok(bytes)
    };
    let log_bytes = read(
        format!("{ROCKETPY}/data/rockets/{}", flight.log.file),
        "reading a flight log",
    )?;
    let log = flight_log(root, flight, &log_bytes).map_err(|error| failed(&error))?;
    let read_log = readings::read(&log);

    let (thrust_path, burn, reshape) =
        example_thrust(root, flight).map_err(|error| failed(&error))?;
    let thrust_bytes = read(thrust_path.clone(), "reading a thrust file")?;
    let (curve, _) = parse_thrust(
        flight.id,
        &thrust_path,
        &String::from_utf8_lossy(&thrust_bytes),
        burn,
        reshape,
    )
    .map_err(|error| failed(&error))?;
    let design_path = format!("validation/designs/{}.json", flight.design);
    let design_bytes = read(design_path.clone(), "reading a design")?;
    let design: Value = serde_json::from_slice(&design_bytes).map_err(|error| failed(&error))?;
    let rocket = rocket_on_curve(flight, &design, &design_path, &thrust_path, &curve)
        .map_err(|error| failed(&error))?;
    let weather_path = format!("{ROCKETPY}/data/weather/{}", flight.weather);
    let weather_bytes = read(weather_path.clone(), "reading an ERA5 file")?;
    let (environment, _) =
        launch_day(flight, &weather_path, &weather_bytes).map_err(|error| failed(&error))?;

    let simulation = Simulation::new(
        &rocket,
        "example",
        environment.clone(),
        rail_of(flight),
        FlightSettings::default(),
    )
    .map_err(|error| failed(&error))?
    .with_recovery(devices(flight.id))
    .map_err(|error| failed(&error))?;
    let mut track = Track::new();
    simulation.run(&mut track).map_err(|error| failed(&error))?;
    let simulated = track
        .simulated(&environment)
        .map_err(|error| failed(&error))?;
    let height_read_as = match flight.log.altimeter {
        Altimeter::Height(_) => HeightKind::Geometric,
        Altimeter::Barometric(_) | Altimeter::AssumedBarometric(_) => HeightKind::Barometric,
    };
    let comparison = residuals::compare(&log, &read_log, &simulated, Some(height_read_as))
        .map_err(|error| failed(&error))?;
    Ok(row(flight, files, log.time_s.len(), comparison))
}

/// The example's parachutes, as its notebook adds them: a name, its `C_d S` in m², its trigger
/// height (`None` at apogee) and its lag in s. Notebooks state these, the designs don't, as
/// `real-flights` flies only up to apogee. `cavour_flight_sim.ipynb` adds none, and says so; Lince's
/// main body is taken alone, without the payload the notebook splits off under its own canopy.
type Chute = (&'static str, f64, Option<f64>, f64);
const CHUTES: [(&str, &[Chute]); 7] = [
    // `bella_lui_flight_sim.ipynb`: `CdS_drogue` π/4, `lag_rec` 1.
    (
        "bella-lui",
        &[("Drogue", std::f64::consts::FRAC_PI_4, None, 1.0)],
    ),
    // `validation/fixtures/recovery/rocketpy-descent.json`, from `ndrt_2020_flight_sim.ipynb`.
    (
        "ndrt-2020",
        &[
            ("Drogue", 0.437_795_261_940_237_93, None, 1.0),
            ("Main", 16.052_492_937_808_73, Some(167.64), 1.0),
        ],
    ),
    // `prometheus_2022_flight_sim.ipynb`: 1.6π(0.3048)² and 2.2π(0.9144)², no lag.
    (
        "prometheus-2022",
        &[
            ("Drogue", 0.466_981_612_736_253_94, None, 0.0),
            ("Main", 5.778_897_457_611_142_5, Some(457.2), 0.0),
        ],
    ),
    // `juno3_flight_sim.ipynb`: one canopy.
    ("juno-iii", &[("Drogue", 0.885, None, 0.5)]),
    ("cavour", &[]),
    // `genesis_flight_sim.ipynb`.
    (
        "genesis",
        &[
            ("Drogue", 0.285_005_285_533_666, None, 1.0),
            ("Main", 1.1, Some(870.0), 1.0),
        ],
    ),
    // `lince_flight_sim.ipynb`: `LincePostDeploy`'s.
    ("lince", &[("Main", 3.9, None, 0.0)]),
];

/// The readings past apogee, which mean nothing against a flight flown without its parachutes.
const PAST_APOGEE: [What; 4] = [
    What::FlightTime,
    What::DrogueRate,
    What::MainOpening,
    What::MainRate,
];

/// `id`'s chutes.
fn chutes(id: &str) -> &'static [Chute] {
    CHUTES
        .iter()
        .find(|(flight, _)| *flight == id)
        .map_or(&[], |(_, chutes)| chutes)
}

/// `id`'s recovery, each canopy released by the next.
fn devices(id: &str) -> Vec<Device> {
    let chutes = chutes(id);
    chutes
        .iter()
        .enumerate()
        .map(|(index, &(name, cd_s_m2, height, lag_s))| {
            let trigger = match height {
                Some(height_above_ground_m) => Trigger::Altitude {
                    height_above_ground_m,
                },
                None => Trigger::Apogee,
            };
            let device =
                Device::new(name, DeviceDrag::DragArea { cd_s_m2 }, trigger).with_lag_s(lag_s);
            if index + 1 < chutes.len() {
                device.with_release_by(index + 1)
            } else {
                device
            }
        })
        .collect()
}

/// The whole log, as `hpr compare` reads it: by its format's own reader, or by the mapping
/// `analyze_writes_a_ghost_of_each_of_rocketpys_logs` reads it by.
fn flight_log(root: &Path, flight: &RealFlight, bytes: &[u8]) -> Result<FlightLog, String> {
    let stem = flight.log.file.trim_end_matches(".csv").replace('/', "-");
    let mapping = root.join(format!("{MAPPINGS}/{stem}.json"));
    if !mapping.exists() {
        return import::read_one(bytes, None, None).map_err(|error| error.to_string());
    }
    let text = fs::read_to_string(&mapping).map_err(|error| error.to_string())?;
    let mapping = custom::mapping_from_json(&text).map_err(|error| error.to_string())?;
    custom::read_by_mapping(bytes, &mapping, None).map_err(|error| error.to_string())
}

/// The report's row for a comparison.
fn row(
    flight: &RealFlight,
    files: Vec<FileRead>,
    log_rows: usize,
    comparison: Comparison,
) -> FlightResiduals {
    let descent_flown = !chutes(flight.id).is_empty();
    let ascent = |phases: Vec<Summary>| -> Vec<Summary> {
        phases
            .into_iter()
            .filter(|summary| descent_flown || summary.phase != Phase::Descent)
            .collect()
    };
    let (speed_m_s, speed_withheld) = match comparison.speed_m_s {
        Speed::Read(speed) => (Some(ascent(speed.phases)), None),
        Speed::Withheld { detail } => (None, Some(detail)),
    };
    FlightResiduals {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        altimeter: flight.log.altimeter.kind().to_owned(),
        files,
        log_rows,
        height_read_as: comparison.height_read_as,
        clock_bound_s: comparison.clock.bound_s,
        reaches_landing: comparison.reaches_landing,
        descent_flown,
        past_simulation: if descent_flown {
            comparison.height_m.past_simulation
        } else {
            0
        },
        height_m: ascent(comparison.height_m.phases),
        speed_m_s,
        speed_withheld,
        rows: comparison
            .rows
            .into_iter()
            .filter(|row| descent_flown || !PAST_APOGEE.contains(&row.what))
            .collect(),
    }
}

/// A phase's name on the page.
fn phase_name(phase: Phase) -> &'static str {
    match phase {
        Phase::Boost => "boost",
        Phase::Coast => "coast",
        Phase::Ascent => "ascent",
        Phase::Descent => "descent",
        Phase::Flight => "flight",
    }
}

/// A reading's name and unit on the page.
fn what_name(what: What) -> (&'static str, &'static str) {
    match what {
        What::ApogeeHeight => ("apogee", "m"),
        What::ApogeeTime => ("time to apogee", "s"),
        What::SmoothedApogee => ("smoothed apogee", "m"),
        What::TopSpeed => ("top vertical speed", "m/s"),
        What::TopAcceleration => ("top acceleration", "m/s²"),
        What::Burnout => ("burnout", "s"),
        What::FlightTime => ("flight time", "s"),
        What::DrogueRate => ("descent under the drogue", "m/s"),
        What::MainOpening => ("main's opening", "m"),
        What::MainRate => ("descent under the main", "m/s"),
    }
}

/// A cell on the page; the log's carries its 90% interval.
fn cell_text(cell: &Cell) -> String {
    match cell {
        Cell::Read(given) if given.is_floor => format!("at least {:.2}", given.value),
        Cell::Read(given) => format!("{:.2}", given.value),
        Cell::Withheld { .. } => "withheld".to_owned(),
    }
}

/// A logged cell's 90% interval on the page.
fn interval_text(cell: &Cell) -> String {
    match cell {
        Cell::Read(given) => given.interval.map_or("none stated".to_owned(), |interval| {
            format!("{:.2} to {:.2}", interval.low, interval.high)
        }),
        Cell::Withheld { .. } => String::new(),
    }
}

/// A logged cell's provenance on the page.
fn provenance_text(cell: &Cell) -> String {
    match cell {
        Cell::Read(given) => given.provenance.map_or(String::new(), |provenance| {
            serde_json::to_value(provenance)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default()
        }),
        Cell::Withheld { .. } => String::new(),
    }
}

/// A table of phases, each line's cells `{:+.1}`.
fn phase_table(page: &mut String, unit: &str, phases: &[Summary]) {
    let _ = writeln!(
        page,
        "| phase | from (s) | to (s) | samples | mean ({unit}) | RMS ({unit}) | largest ({unit}) | at (s) |\n|---|---:|---:|---:|---:|---:|---:|---:|"
    );
    for summary in phases {
        let _ = writeln!(
            page,
            "| {} | {:.2} | {:.2} | {} | {:+.1} | {:.1} | {:+.1} | {:.2} |",
            phase_name(summary.phase),
            summary.start_s,
            summary.end_s,
            summary.samples,
            summary.mean,
            summary.rms,
            summary.largest,
            summary.largest_at_s
        );
    }
}

impl Report {
    /// The page, from the data alone.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut page = String::from(
            "# hpr's residuals against the logs of real flights\n\n\
             Written by `cargo xtask real-flights` ([M7.3a][m7-3a], decision [ADR-314][adr-314]). \
             [How to read them][site].\n\n\
             - **What is flown:** the 7 flights of the [real-flights report][real], each flown \
             to the ground in its ERA5 air and wind, on hpr's own drag, under the parachutes \
             its example's notebook gives it.\n\
             - **A residual** is the logged value less the simulated one. Positive: the log read \
             higher, or faster upward, than the simulation.\n\
             - **Heights** are the log's own samples, above its pad, unsmoothed. The simulation's \
             are read as the log's altimeter would read them. **Speeds** are the smoother's, \
             taken from the log's heights, against the rate at which the same altimeter's reading \
             would change.\n\
             - **The clocks** are lined up by what one reader calls liftoff on each.\n\
             - A residual is one flight against one simulation. It doesn't say which is wrong.\n\n\
             [m7-3a]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3a\n\
             [adr-314]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0314-m7-3-in-five-residuals-first.md\n\
             [site]: https://hpr.fusionspace.co/comparing-a-flight.html\n\
             [real]: real-flights.md\n",
        );
        for flight in &self.flights {
            let _ = writeln!(page, "\n## {}\n", flight.title);
            let _ = writeln!(
                page,
                "Altimeter: {}, read as {}. {} log samples; clocks lined up within {:.2} s; {}{}\n",
                flight.altimeter,
                match flight.height_read_as {
                    HeightKind::Barometric => "a barometer",
                    HeightKind::Geometric => "a height climbed",
                },
                flight.log_rows,
                flight.clock_bound_s,
                if !flight.descent_flown {
                    "the example gives no parachutes, so only the ascent is compared"
                } else if flight.reaches_landing {
                    "the log reads a landing"
                } else {
                    "the log ends before a landing, so the residuals stop at its end"
                },
                match flight.past_simulation {
                    0 => ".".to_owned(),
                    count => format!("; {count} samples after the simulated flight had ended."),
                }
            );
            page.push_str("Height:\n\n");
            phase_table(&mut page, "m", &flight.height_m);
            page.push('\n');
            match (&flight.speed_m_s, &flight.speed_withheld) {
                (Some(phases), _) => {
                    page.push_str("Speed:\n\n");
                    phase_table(&mut page, "m/s", phases);
                }
                (None, Some(detail)) => {
                    let _ = writeln!(page, "Speed: withheld. {detail}");
                }
                (None, None) => {}
            }
            page.push_str(
                "\nReadings, the simulation read as the log is:\n\n\
                 | reading | logged | its 90% interval | provenance | simulated | logged less simulated |\n\
                 |---|---:|---:|---|---:|---:|\n",
            );
            for row in &flight.rows {
                let (name, unit) = what_name(row.what);
                let _ = writeln!(
                    page,
                    "| {name} ({unit}) | {} | {} | {} | {} | {} |",
                    cell_text(&row.logged),
                    interval_text(&row.logged),
                    provenance_text(&row.logged),
                    cell_text(&row.simulated),
                    row.logged_minus_simulated
                        .map_or(String::new(), |difference| format!("{difference:+.2}"))
                );
            }
        }
        page
    }
}

/// Whether two JSON documents agree, numbers within [`REPRODUCES_WITHIN`].
fn agrees(path: &str, a: &Value, b: &Value) -> Result<(), String> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (
                x.as_f64().unwrap_or(f64::NAN),
                y.as_f64().unwrap_or(f64::NAN),
            );
            if (x - y).abs() <= REPRODUCES_WITHIN * x.abs().max(1.0) {
                Ok(())
            } else {
                Err(format!("{path}: committed {x}, flown {y}"))
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .try_for_each(|(i, (x, y))| agrees(&format!("{path}/{i}"), x, y)),
        (Value::Object(x), Value::Object(y)) if x.len() == y.len() => {
            x.iter().try_for_each(|(key, x)| match y.get(key) {
                Some(y) => agrees(&format!("{path}/{key}"), x, y),
                None => Err(format!("{path}/{key}: not flown")),
            })
        }
        _ if a == b => Ok(()),
        _ => Err(format!("{path}: committed {a}, flown {b}")),
    }
}

/// Flies the flights and writes or checks the report.
pub fn run(root: &Path, check: bool) -> Result<(), String> {
    let flights = FLIGHTS
        .iter()
        .map(|flight| fly(root, flight))
        .collect::<Result<Vec<_>, _>>()?;
    let report = Report { flights };
    for flight in &report.flights {
        let (rms, samples) = flight
            .height_m
            .iter()
            .fold((0.0_f64, 0), |(rms, samples), summary| {
                (rms.max(summary.rms), samples + summary.samples)
            });
        println!(
            "flight-residuals: {:<16} {samples:>5} samples, largest phase RMS {rms:>7.1} m",
            flight.id
        );
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: Value = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let flown: Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
        agrees("", &committed, &flown)?;
        println!("flight-residuals: the committed report reproduces");
        return Ok(());
    }
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, report.to_markdown())] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("flight-residuals: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn committed() -> (Report, String) {
        let root = crate::designs::root().unwrap();
        let json = fs::read_to_string(root.join(REPORT_JSON)).unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        (serde_json::from_str(&json).unwrap(), page)
    }

    /// CI has no `refs/`: it holds the committed page to the committed data.
    #[test]
    fn the_committed_page_is_its_data() {
        let (report, page) = committed();
        assert_eq!(report.to_markdown(), page);
    }

    /// Every flight in the report, each with residuals, its logged readings keeping a provenance.
    #[test]
    fn the_report_holds_all_seven_flights() {
        let (report, _) = committed();
        assert_eq!(report.flights.len(), 7);
        for flight in &report.flights {
            assert!(
                flight.height_m.iter().map(|s| s.samples).sum::<usize>() > 0,
                "{}",
                flight.id
            );
            for row in &flight.rows {
                if let Cell::Read(logged) = &row.logged {
                    assert!(logged.provenance.is_some(), "{}: {row:?}", flight.id);
                }
                if let Cell::Read(simulated) = &row.simulated {
                    assert_eq!(simulated.provenance, None, "{}: {row:?}", flight.id);
                }
            }
        }
    }

    /// The simulated apogee read through the log's reader agrees with the one `real-flights` takes
    /// straight from the flight's event, an independent path through the same flight.
    #[test]
    fn the_simulated_apogee_is_real_flights_own() {
        let (report, _) = committed();
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(hpr_validate::real_flight::REPORT_JSON)).unwrap();
        let real: Value = serde_json::from_str(&text).unwrap();
        let rows = real["flights"].as_array().unwrap();
        for flight in &report.flights {
            let row = rows.iter().find(|row| row["id"] == *flight.id).unwrap();
            let event = row["hpr_apogee_m"].as_f64().unwrap();
            let apogee = flight
                .rows
                .iter()
                .find(|row| row.what == What::ApogeeHeight)
                .unwrap();
            let Cell::Read(simulated) = &apogee.simulated else {
                panic!("{}: {apogee:?}", flight.id);
            };
            assert!(
                (simulated.value - event).abs() < 0.5,
                "{}: {} against {event}",
                flight.id,
                simulated.value
            );
        }
    }

    /// `--check` fails on a changed number: a report that can't fail checks nothing.
    #[test]
    fn a_changed_number_fails_the_check() {
        let a = serde_json::json!({"x": [1.0, {"y": 2.0}]});
        let b = serde_json::json!({"x": [1.0, {"y": 2.01}]});
        assert!(agrees("", &a, &a).is_ok());
        assert!(agrees("", &a, &b).unwrap_err().contains("/x/1/y"));
    }
}
