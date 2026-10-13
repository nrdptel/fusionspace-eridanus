//! Missile Works' RRC3 exports, as mDACS writes them, read through the importer from invented
//! files and pinned whole by snapshots (M7.1b). The files hold made-up flights whose pressure is
//! the standard atmosphere's for their altitude, so that each unit the header doesn't name is
//! decided from the file as it would be from a real one: the common export in feet and ft/s with
//! its short last rows; one with its velocity in mph; one naming its units in brackets, its last
//! rows padded and two rows with no time; and a European-locale export in meters and m/s.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// The common export: seven columns, feet and ft/s decided from the file, °F and volts assumed,
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
            ("Temperature", Quantity::Temperature),
            ("Voltages", Quantity::Voltage),
        ]
    );
    assert_eq!(log.time_s.len(), 64);
    // The second row: 16.45 ft, 983.00 hPa, 65.80 ft/s, 71.60 °F.
    assert!((log.altitude_m[1] - 16.45 * FOOT_M).abs() < 1e-12);
    assert!((log.channels[1].values[1] - 98_300.0).abs() < 1e-9);
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    assert!((speed[1] - 65.80 * FOOT_M).abs() < 1e-12);
    let temperature = log.temperature_k.as_deref().unwrap();
    assert!((temperature[1] - (71.6 - 32.0) * 5.0 / 9.0 - 273.15).abs() < 1e-9);
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
    note(&log, "`Temperature` read in degrees Fahrenheit, assumed: ");
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
/// file; its temperature assumed in °C beside an altitude in meters.
#[test]
fn a_european_export_reads_decimal_commas_and_decides_meters() {
    let log = import::read(include_bytes!("samples/rrc3-semicolon.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Rrc3Csv);
    note(&log, "`Altitude` read in meters, from the pressure: ");
    note(
        &log,
        "`Velocity` read in meters per second, from the altitude: ",
    );
    note(&log, "`Temperature` read in degrees Celsius, assumed: ");
    // The second row: 6,26 m and 25,04 m/s.
    assert!((log.altitude_m[1] - 6.26).abs() < 1e-12);
    assert!((log.vertical_speed_m_s.as_deref().unwrap()[1] - 25.04).abs() < 1e-12);
    assert!((log.temperature_k.as_deref().unwrap()[0] - 294.15).abs() < 1e-9);
    insta::assert_json_snapshot!(log);
}
