//! Missile Works' RRC3 exports, as mDACS writes them, read through the importer from invented
//! files and pinned whole by snapshots (M7.1b). The files hold made-up flights whose pressure is
//! the standard atmosphere's for their altitude, so that each unit the header doesn't name is
//! decided from the file as it would be from a real one: the common export in feet and ft/s with
//! its short last rows; one with its velocity in mph; one naming its units in brackets, its last
//! rows padded and two rows with no time; and a European-locale export in meters and m/s. Then
//! invented 20 Hz flights whose velocity runs behind the altitude, delayed or smoothed, each of
//! which must have its velocity's unit decided right or left unknown, never decided wrong.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// The common export: seven columns, feet and ft/s decided from the file, the temperature kept as
/// written (its unit unstated, #852) and volts assumed,
/// the last three rows without their temperature and voltage; a byte-order mark and CR LF.
#[test]
fn the_common_export_decides_feet_and_feet_per_second() {
    let log = import::read(include_bytes!("samples/rrc3-feet.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Rrc3Csv);
    assert_eq!(log.logger, "Missile Works RRC3");
    let quantities: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        quantities,
        [
            ("Altitude", Quantity::AltitudeAboveGround),
            ("Pressure", Quantity::Pressure),
            ("Velocity", Quantity::Speed),
            ("Temperature", Quantity::Unrecognized),
            ("Voltages", Quantity::Voltage),
        ]
    );
    assert_eq!(log.time_s.len(), 64);
    // The second row: 16.45 ft, 983.00 hPa, 65.80 ft/s, 71.60.
    assert!((log.altitude_m[1] - 16.45 * FOOT_M).abs() < 1e-12);
    assert!((log.channels[1].values[1] - 98_300.0).abs() < 1e-9);
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    assert!((speed[1] - 65.80 * FOOT_M).abs() < 1e-12);
    // The temperature as written: 71.6, its unit unstated.
    let temperature = &log.channels[3].values;
    assert_eq!(temperature[1], 71.6);
    // The last three rows leave the temperature and voltage out: gaps, not the event cell.
    assert!(temperature[60].is_finite());
    assert!(temperature[61..].iter().all(|t| t.is_nan()));
    assert!(
        log.battery_v.as_deref().unwrap()[61..]
            .iter()
            .all(|v| v.is_nan())
    );
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(events, [(8.0, "Drogue"), (13.5, "Main")]);
    assert!(note(&log, "`Altitude` read in feet, from the pressure: ").contains("line 2 to line"));
    note(
        &log,
        "`Velocity` read in feet per second, from the altitude: ",
    );
    note(&log, "`Pressure` read in hectopascals, from the values: ");
    note(&log, "`Temperature` kept as written, its unit unknown: ");
    assert!(log.temperature_k.is_none());
    note(&log, "`Voltages` read in volts, assumed: ");
    note(&log, "3 rows, the first on line 63, leave out the 1 Hz");
    insta::assert_json_snapshot!(log);
}

/// A velocity in miles per hour beside an altitude in feet, told apart by the velocity's
/// integral; no temperature column; LF line ends and a pad 1,200 m up.
#[test]
fn a_velocity_in_miles_per_hour_is_told_from_the_altitude() {
    let log = import::read(include_bytes!("samples/rrc3-mph.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Rrc3Csv);
    note(
        &log,
        "`Velocity` read in miles per hour, from the altitude: ",
    );
    note(&log, "`Altitude` read in feet, from the pressure: ");
    assert!(log.temperature_k.is_none());
    assert_eq!(log.channels.len(), 4);
    // The fastest sample, read in mph, against the altitude's fastest climb between two rows.
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    let fastest = speed.iter().copied().fold(0.0, f64::max);
    let climb = log
        .altitude_m
        .windows(2)
        .zip(log.time_s.windows(2))
        .map(|(h, t)| (h[1] - h[0]) / (t[1] - t[0]))
        .fold(0.0, f64::max);
    assert!(
        (fastest / climb - 1.0).abs() < 0.2,
        "{fastest} m/s, {climb} m/s"
    );
    let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["Drogue", "Main", "Aux"]);
    note(
        &log,
        "2 rows, the first on line 52, leave out the 1 Hz `Voltages` cell",
    );
    insta::assert_json_snapshot!(log);
}

/// Units in brackets are taken as written, and checked against the pressure and the altitude:
/// meters, mbar, km/h, °C and volts, with no notes on units; the last rows padded to the
/// header's width, and two rows with no time skipped.
#[test]
fn units_in_brackets_are_taken_as_written() {
    let log = import::read(include_bytes!("samples/rrc3-bracketed.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Rrc3Csv);
    assert!(
        log.notes.iter().all(|n| !n.contains(" read in ")),
        "{:?}",
        log.notes
    );
    assert_eq!(log.channels[0].column, "Altitude [m]");
    assert_eq!(log.channels[2].quantity, Quantity::Speed);
    let temperature = log.temperature_k.as_deref().unwrap();
    assert!((temperature[0] - (24.5 + 273.15)).abs() < 1e-9);
    assert_eq!(
        log.notes,
        [
            "2 rows, the first on line 63, leave out the 1 Hz `Temperature [C]` and `Voltages [V]` \
             cells: they are gaps; 2 of them pad the row to the header's width with empty cells \
             after the others",
            "2 rows with no time and nothing but an altitude cell, the first on line 65, were \
             skipped",
        ]
    );
    insta::assert_json_snapshot!(log);
}

/// A European-locale export: semicolons, decimal commas, and meters and m/s decided from the
/// file; its temperature, which states no unit, kept as written beside an altitude in meters.
#[test]
fn a_european_export_reads_decimal_commas_and_decides_meters() {
    let log = import::read(include_bytes!("samples/rrc3-semicolon.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Rrc3Csv);
    note(&log, "`Altitude` read in meters, from the pressure: ");
    note(
        &log,
        "`Velocity` read in meters per second, from the altitude: ",
    );
    note(&log, "`Temperature` kept as written, its unit unknown: ");
    // The second row: 6,26 m and 25,04 m/s.
    assert!((log.altitude_m[1] - 6.26).abs() < 1e-12);
    assert!((log.vertical_speed_m_s.as_deref().unwrap()[1] - 25.04).abs() < 1e-12);
    assert!(log.temperature_k.is_none());
    let temperature = log.channels.iter().find(|c| c.column == "Temperature");
    assert_eq!(temperature.unwrap().values[0], 21.0);
    insta::assert_json_snapshot!(log);
}

/// The pressure, hPa, at a height above a sea-level pad, m, and back: the altimeter formula of
/// the standard atmosphere's troposphere.
fn pressure_hpa(height_m: f64) -> f64 {
    1_013.25 * (1.0 - height_m / 44_330.8).powf(1.0 / 0.190_263)
}

fn height_m(pressure_hpa: f64) -> f64 {
    44_330.8 * (1.0 - (pressure_hpa / 1_013.25).powf(0.190_263))
}

/// How an invented flight's velocity runs behind its altitude.
#[derive(Debug, Clone, Copy)]
enum Lag {
    /// Late by this many seconds, unchanged.
    Delay(f64),
    /// Smoothed by a first-order filter of this time constant, s.
    Smoothed(f64),
}

/// An invented flight at 20 Hz, as an RRC3 export with no units named: a second on the pad, a
/// boost at 80 m/s² and a coast at 12 m/s² to `top_m`, a fall at 9.8 m/s² to `descent_m_s`, that
/// speed to the ground and three seconds there. The pressure is the standard atmosphere's to
/// 0.1 hPa, as the RRC3 writes it, and the altitude is worked back from it, written in units of
/// `length_m`; the velocity is the climb rate run through `lag`, written in units of `speed_m_s`;
/// `keep` rows are written, all if `None`.
fn flight(
    top_m: f64,
    descent_m_s: f64,
    length_m: f64,
    speed_m_s: f64,
    lag: Lag,
    keep: Option<usize>,
) -> String {
    let (boost, coast, gravity) = (80.0, 12.0, 9.8);
    let boost_s = (top_m / (boost / 2.0 + boost * boost / (2.0 * coast))).sqrt();
    let burnout_m_s = boost * boost_s;
    let apogee_s = boost_s + burnout_m_s / coast;
    let track = |t: f64| -> (f64, f64) {
        let (height, rate) = if t < 0.0 {
            (0.0, 0.0)
        } else if t < boost_s {
            (boost * t * t / 2.0, boost * t)
        } else if t < apogee_s {
            let u = t - boost_s;
            (
                boost * boost_s * boost_s / 2.0 + burnout_m_s * u - coast * u * u / 2.0,
                burnout_m_s - coast * u,
            )
        } else {
            let u = t - apogee_s;
            let fall_s = descent_m_s / gravity;
            if u < fall_s {
                (top_m - gravity * u * u / 2.0, -gravity * u)
            } else {
                (
                    top_m - gravity * fall_s * fall_s / 2.0 - descent_m_s * (u - fall_s),
                    -descent_m_s,
                )
            }
        };
        if height <= 0.0 && t > 0.0 {
            (0.0, 0.0)
        } else {
            (height, rate)
        }
    };
    let mut text = "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages\n".to_owned();
    let pad_hpa = (pressure_hpa(0.0) * 10.0).round() / 10.0;
    let mut smoothed = 0.0;
    let mut landed_s = None;
    for row in 0_u32.. {
        let t = f64::from(row) * 0.05 - 1.0;
        let (height, rate) = track(t);
        if t > 1.0 && height == 0.0 {
            landed_s.get_or_insert(t);
        }
        if landed_s.is_some_and(|landed| t > landed + 3.0)
            || keep.is_some_and(|k| row as usize >= k)
        {
            break;
        }
        let velocity = match lag {
            Lag::Delay(delay_s) => track(t - delay_s).1,
            Lag::Smoothed(tau_s) => {
                smoothed += (rate - smoothed) * 0.05 / (tau_s + 0.05);
                smoothed
            }
        };
        let hpa = (pressure_hpa(height) * 10.0).round() / 10.0;
        let altitude = (height_m(hpa) - height_m(pad_hpa)) / length_m;
        text.push_str(&format!(
            "{:.2},{altitude:.2},{hpa:.1},{:.2},71.60, -,9.20\n",
            t + 1.0,
            velocity / speed_m_s
        ));
    }
    text
}

/// The unit the velocity was decided in, from its note; `None` if it was kept as written.
fn velocity_unit(log: &FlightLog) -> Option<String> {
    let decided: Vec<&String> = log
        .notes
        .iter()
        .filter(|n| n.starts_with("`Velocity` read in "))
        .collect();
    match decided.as_slice() {
        [note] => Some(
            note.trim_start_matches("`Velocity` read in ")
                .split(", from the altitude: ")
                .next()
                .unwrap_or_default()
                .to_owned(),
        ),
        _ => {
            note(log, "`Velocity` kept as written, its unit unknown: ");
            assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
            None
        }
    }
}

/// Flights in meters and km/h, and in feet and ft/s, 100 to 300 m up and coming down at 25 to
/// 40 m/s, their velocity one or two seconds late: each is decided right. ft/s and km/h are the
/// nearest two units, 9.7% apart, so these are the ones a lag could swap.
#[test]
fn a_late_velocity_is_decided_right() {
    for (length_m, speed_m_s, words) in [
        (1.0, 1.0 / 3.6, "kilometers per hour"),
        (FOOT_M, FOOT_M, "feet per second"),
    ] {
        for top_m in [100.0, 200.0, 300.0] {
            for descent_m_s in [25.0, 40.0] {
                for delay_s in [1.0, 2.0] {
                    let case = format!("{words}, {top_m} m, {descent_m_s} m/s, {delay_s} s late");
                    let text = flight(
                        top_m,
                        descent_m_s,
                        length_m,
                        speed_m_s,
                        Lag::Delay(delay_s),
                        None,
                    );
                    let log = import::read(text.as_bytes()).unwrap();
                    assert_eq!(log.format, LogFormat::Rrc3Csv);
                    assert_eq!(velocity_unit(&log).as_deref(), Some(words), "{case}");
                    note(&log, "`Time` read in seconds, from the step: ");
                    // The note gives the delay the fit found, to the 0.05 s step it searches.
                    let decided = note(&log, "`Velocity` read in ");
                    assert!(
                        decided.contains(&format!("as it was {delay_s:.2} s earlier")),
                        "{case}: {decided}"
                    );
                }
            }
        }
    }
}

/// The same flights with their velocity smoothed by a first-order filter of one or two seconds,
/// which rounds the apogee off the velocity's integral: each is decided right or kept as
/// written, never decided in the other unit.
#[test]
fn a_smoothed_velocity_is_never_decided_wrong() {
    let mut decided = 0;
    for (length_m, speed_m_s, words) in [
        (1.0, 1.0 / 3.6, "kilometers per hour"),
        (FOOT_M, FOOT_M, "feet per second"),
    ] {
        for top_m in [100.0, 200.0, 300.0] {
            for descent_m_s in [25.0, 40.0] {
                for tau_s in [1.0, 2.0] {
                    let text = flight(
                        top_m,
                        descent_m_s,
                        length_m,
                        speed_m_s,
                        Lag::Smoothed(tau_s),
                        None,
                    );
                    let log = import::read(text.as_bytes()).unwrap();
                    let unit = velocity_unit(&log);
                    assert!(
                        unit.as_deref().is_none_or(|unit| unit == words),
                        "{words}, {top_m} m, {descent_m_s} m/s, smoothed over {tau_s} s: {unit:?}"
                    );
                    decided += usize::from(unit.is_some());
                }
            }
        }
    }
    // Some are decided: the rule doesn't pass by leaving every one unknown.
    assert!(decided > 0);
}

/// A record that stops while the rocket still comes down fast, its velocity two seconds late:
/// the end could move the measure by the areas past what is allowed, so the velocity is kept as
/// written; and one that stops before coming halfway down.
#[test]
fn a_record_that_stops_early_leaves_the_velocity_unknown() {
    let full = flight(100.0, 40.0, 1.0, 1.0 / 3.6, Lag::Delay(2.0), None);
    let rows = full.lines().count() - 1;
    // The last 3 s on the ground and 0.75 s of the descent left out: about 30 m up.
    let early = flight(
        100.0,
        40.0,
        1.0,
        1.0 / 3.6,
        Lag::Delay(2.0),
        Some(rows - 75),
    );
    let log = import::read(early.as_bytes()).unwrap();
    assert_eq!(velocity_unit(&log), None);
    let kept = note(&log, "`Velocity` kept as written, its unit unknown: ");
    assert!(kept.contains("still moving"), "{kept}");
    // Stopped two seconds after apogee.
    let high = flight(300.0, 25.0, 1.0, 1.0 / 3.6, Lag::Delay(1.0), Some(220));
    let log = import::read(high.as_bytes()).unwrap();
    assert_eq!(velocity_unit(&log), None);
    let kept = note(&log, "`Velocity` kept as written, its unit unknown: ");
    assert!(kept.contains("of the way up the climb"), "{kept}");
}

/// The two public exports of #852, read where `refs/` holds them (they aren't committed): both
/// in feet, one's temperature °C by the other loggers on its flight (47.2 falling to 43.2), the
/// other's °F by the airport's air (69.8 to 70.52). The altitude's unit can't tell them apart, so
/// both are kept as written.
#[test]
fn two_public_exports_in_feet_keep_their_temperature_as_written() {
    let refs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../refs");
    let files = [
        (
            "research/public-logs-evidence-gaps/cache-b/volta/RRC3_FLIGHT_DATA_2025_IREC.csv",
            (47.2, 43.2),
        ),
        (
            "research/sl-weighed-mass-flights/github-usfsoar/RRC3_Flight_Data_2024-12-21.csv",
            (69.8, 70.52),
        ),
    ];
    for (file, (first, last)) in files {
        let Ok(bytes) = std::fs::read(refs.join(file)) else {
            println!("{file} isn't under refs/: not read");
            continue;
        };
        let log = import::read(&bytes).unwrap();
        assert_eq!(log.format, LogFormat::Rrc3Csv);
        note(&log, "`Altitude` read in feet, from the pressure: ");
        note(&log, "`Temperature` kept as written, its unit unknown: ");
        assert!(log.temperature_k.is_none(), "{file}");
        let temperature = log.channels.iter().find(|c| c.column == "Temperature");
        let temperature = temperature.unwrap();
        assert_eq!(temperature.quantity, Quantity::Unrecognized);
        let written: Vec<f64> = temperature
            .values
            .iter()
            .copied()
            .filter(|t| t.is_finite())
            .collect();
        assert_eq!((written[0], written[written.len() - 1]), (first, last));
    }
}
