//! Entacore AIM exports, as the AIM XTRA and AIM BASE software writes them, read through the
//! importer from invented files and pinned whole by snapshots (M7.1c2). The files hold made-up
//! flights whose pressure is the standard atmosphere's for their altitude, each group of channels
//! on its own clock: the flight computer's own log, its GPS group the last and running out early,
//! one empty cell too wide; the ground station's telemetry, its GPS group the first, so that every
//! group after it would be read one cell off without the repair; and a file in feet and °F
//! without a `GPS Time of Week`.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// Each channel's name, quantity and sample count.
fn layout(log: &FlightLog) -> Vec<(&str, Quantity, usize)> {
    log.channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity, c.time_s.len()))
        .collect()
}

/// The flight computer's log: twelve groups at their own rates, every column kept, the GPS
/// group's extra cell dropped from the 24 rows after it runs out; CR LF line ends.
#[test]
fn the_flight_computers_log_reads_every_group_on_its_clock() {
    let log = import::read(include_bytes!("samples/entacore-xtra.csv")).unwrap();
    assert_eq!(log.format, LogFormat::EntacoreAimCsv);
    assert_eq!(log.logger, "Entacore AIM");
    assert_eq!(
        layout(&log),
        [
            ("Gyro X", Quantity::AngularRate, 30),
            ("Gyro Y", Quantity::AngularRate, 30),
            ("Gyro Z", Quantity::AngularRate, 30),
            ("pressure", Quantity::Pressure, 15),
            ("Pressure MSL", Quantity::AltitudeMsl, 15),
            ("Pressure AGL", Quantity::AltitudeAboveGround, 15),
            ("acceleration", Quantity::Acceleration, 30),
            ("lat. X accel.", Quantity::Acceleration, 30),
            ("lat. Y accel.", Quantity::Acceleration, 30),
            ("Mag X", Quantity::Unrecognized, 7),
            ("Mag Y", Quantity::Unrecognized, 7),
            ("Mag Z", Quantity::Unrecognized, 7),
            ("eject. bat. voltage", Quantity::Voltage, 7),
            ("comp. bat. voltage", Quantity::Voltage, 7),
            ("launch detect", Quantity::Number, 15),
            ("apogee detect", Quantity::Number, 15),
            ("A cont.", Quantity::Number, 15),
            ("B cont.", Quantity::Number, 15),
            ("C cont.", Quantity::Number, 15),
            ("D cont.", Quantity::Number, 15),
            ("A on.", Quantity::Number, 15),
            ("B on", Quantity::Number, 15),
            ("C on", Quantity::Number, 15),
            ("D on", Quantity::Number, 15),
            ("A Voltage", Quantity::Voltage, 15),
            ("temperature", Quantity::Temperature, 4),
            ("GPS Time of Week", Quantity::Unrecognized, 4),
            ("lat", Quantity::Latitude, 6),
            ("long", Quantity::Longitude, 6),
            ("GPS MSL", Quantity::Unrecognized, 6),
            ("SVs", Quantity::Number, 6),
        ]
    );
    // The track is `Pressure AGL`'s, on the pressure's clock, from -1.99 s by 1 s.
    assert_eq!(log.time_s.len(), 15);
    assert!((log.time_s[0] + 1.99).abs() < 1e-12);
    assert_eq!(log.altitude_m[0], 0.0);
    let apogee = log.altitude_m.iter().copied().fold(0.0, f64::max);
    assert!(apogee > 500.0 && apogee < 560.0, "{apogee}");
    // 1.00391 g on the pad; 22.5 °C.
    assert!((log.channels[6].values[0] - 1.00391 * 9.806_65).abs() < 1e-9);
    assert!((log.channels[25].values[0] - 295.65).abs() < 1e-9);
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(
        events,
        [
            (1.13, "launch detect"),
            (10.13, "apogee detect"),
            (10.13, "A on.")
        ]
    );
    note(&log, "`time` read in seconds, from `GPS Time of Week`: ");
    note(&log, "`pressure` read in pascals, from the values: ");
    note(
        &log,
        "`Pressure MSL` and `Pressure AGL` read in meters, from the pressure: ",
    );
    // A site 250 m up is too low for the factor between feet and meters to show against the
    // barometric altitude, so `GPS MSL` is kept as written.
    assert!(
        note(
            &log,
            "`GPS MSL` kept as written, in a unit `Pressure MSL` doesn't settle: "
        )
        .contains("doesn't tell the units apart")
    );
    note(
        &log,
        "`temperature` read in degrees Celsius, from the heights' unit and the manual, p33: ",
    );
    note(
        &log,
        "`acceleration`, `lat. X accel.` and `lat. Y accel.` read in g, from the manual: ",
    );
    note(&log, "`Mag X`, `Mag Y` and `Mag Z` kept as written, ");
    note(
        &log,
        "`GPS Time of Week` kept as written, in milliseconds, from the values: ",
    );
    note(&log, "`A Voltage` read in volts, from the values: ");
    note(&log, "`SVs` kept as written, a count, from the manual: ");
    assert!(log.notes.iter().all(|n| !n.contains("assumed")));
    note(
        &log,
        "24 rows, the first on line 8, write the run-out GPS group",
    );
    assert!(log.notes.iter().all(|n| !n.contains("RSSI")));
    insta::assert_json_snapshot!(log);
}

/// The ground station's telemetry: the GPS group first, its extra cell dropped from the 10 rows
/// after it runs out, so the pressure and the groups after it are read under their names; LF line
/// ends.
#[test]
fn the_ground_stations_file_reads_past_its_run_out_gps_group() {
    let log = import::read(include_bytes!("samples/entacore-base.csv")).unwrap();
    assert_eq!(log.format, LogFormat::EntacoreAimCsv);
    let counts: Vec<(&str, usize)> = layout(&log)
        .into_iter()
        .map(|(name, _, count)| (name, count))
        .collect();
    assert_eq!(
        counts[0..4],
        [("lat", 4), ("long", 4), ("GPS MSL", 4), ("SVs", 4)]
    );
    assert_eq!(
        counts[4..8],
        [
            ("pressure", 11),
            ("Pressure MSL", 11),
            ("Pressure AGL", 11),
            ("RSSI", 14)
        ]
    );
    // Read shifted, the RSSI would hold the AGL's heights and the pressure a time.
    let rssi = &log.channels[7];
    assert!(
        rssi.values.iter().all(|v| (-110.0..-80.0).contains(v)),
        "{:?}",
        rssi.values
    );
    assert!(log.channels[4].values.iter().all(|p| *p > 90_000.0));
    assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
    note(
        &log,
        "10 rows, the first on line 6, write the run-out GPS group",
    );
    note(
        &log,
        "the file holds `RSSI`, the radio's strength at the AIM BASE",
    );
    note(
        &log,
        "`comp. bat. voltage` read in volts, from the values: ",
    );
    note(&log, "`RSSI` kept as written, in dBm, from the manual: ");
    assert!(log.notes.iter().all(|n| !n.contains("assumed")));
    insta::assert_json_snapshot!(log);
}

/// A file in imperial units: the altitudes decided in feet from the pressure, the temperature then
/// in °F, and the clock decided in seconds from its step, with no `GPS Time of Week`.
#[test]
fn a_file_in_feet_reads_the_same_heights() {
    let log = import::read(include_bytes!("samples/entacore-feet.csv")).unwrap();
    let meters = import::read(include_bytes!("samples/entacore-xtra.csv")).unwrap();
    assert_eq!(log.time_s.len(), 13);
    // The same invented flight, sampled at other times: the highest sample is within a meter of
    // the other file's apogee in meters, not 3.28 times it.
    let highest = log.altitude_m.iter().copied().fold(0.0, f64::max);
    let apogee = meters.altitude_m.iter().copied().fold(0.0, f64::max);
    assert!((highest - apogee).abs() < 1.0, "{highest} {apogee}");
    let msl = &log.channels[1];
    assert!(
        (msl.values[0] - 249.96).abs() < 0.01 * FOOT_M + 1e-3,
        "{}",
        msl.values[0]
    );
    note(
        &log,
        "`Pressure MSL` and `Pressure AGL` read in feet, from the pressure: ",
    );
    note(
        &log,
        "`temperature` read in degrees Fahrenheit, from the heights' unit and the manual, p33: ",
    );
    // 71.2 °F is 21.777… °C.
    assert!((log.channels[4].values[0] - (71.2 - 32.0) * 5.0 / 9.0 - 273.15).abs() < 1e-9);
    // No `GPS Time of Week`: the clock is decided from the step, 0.5 s.
    assert!(
        note(&log, "`time` read in seconds, from the step: ")
            .contains("the file has no `GPS Time of Week`")
    );
    assert!(log.notes.iter().all(|n| !n.contains("assumed")));
    insta::assert_json_snapshot!(log);
}
