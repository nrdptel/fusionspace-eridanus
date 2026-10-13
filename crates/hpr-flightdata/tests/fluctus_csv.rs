//! Silicdyne Fluctus exports, as its Fluctus Control Center writes them, read through the importer
//! from invented files and pinned whole by snapshots (M7.1c3). The files hold made-up flights, a
//! row every 500 ms rather than the logger's 20 ms to keep them short: a flight in the full
//! layout whose GPS altitude follows the barometric one in meters but for two rows where it holds
//! its last fix, with a `nan` cell and a missing row; a flight whose dead reckoning stops, written
//! as the largest 32-bit number, and whose GPS altitude stays near the pad, with CR LF line ends
//! and a byte-order mark; a flight whose GPS altitude, in feet, regains its fix low in the descent;
//! and a header stating other units than the logger's own, with points for decimals.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// Each channel's name and quantity.
fn layout(log: &FlightLog) -> Vec<(&str, Quantity)> {
    log.channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect()
}

/// Each event's time and name.
fn events(log: &FlightLog) -> Vec<(f64, &str)> {
    log.events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect()
}

/// The full layout: every column but the clock and `deltaTime` is a channel, the position in
/// degrees and the GPS altitude in meters, the status changes and the fired outputs events.
#[test]
fn a_flight_reads_every_column_and_its_events() {
    let log = import::read(include_bytes!("samples/fluctus-flight.csv")).unwrap();
    assert_eq!(log.format, LogFormat::FluctusCsv);
    assert_eq!(log.logger, "Silicdyne Fluctus");
    assert_eq!(
        layout(&log),
        [
            ("status", Quantity::Number),
            ("baro-altitude (m)", Quantity::AltitudeAboveGround),
            ("dedrck-v-speed (m/s)", Quantity::Speed),
            ("angle (deg)", Quantity::Angle),
            ("roll-rate (deg/s)", Quantity::AngularRate),
            ("vert-accel (m/s2)", Quantity::Acceleration),
            ("accel (m/s2)", Quantity::Acceleration),
            ("dedrck-alti (m)", Quantity::InertialAltitude),
            ("baro-speed (m/s)", Quantity::Speed),
            ("amb-temp (deg c)", Quantity::Temperature),
            ("batt-voltage (mV)", Quantity::Voltage),
            ("P1-state", Quantity::Number),
            ("P2-state", Quantity::Number),
            ("P3-state", Quantity::Number),
            ("analog1 (mV)", Quantity::Voltage),
            ("analog2 (mV)", Quantity::Voltage),
            ("inFreefall", Quantity::Number),
            ("gpsLat", Quantity::Latitude),
            ("gpsLng", Quantity::Longitude),
            ("gpsAltMSL", Quantity::GnssAltitude),
            ("gpsState", Quantity::Number),
            ("gpsSats", Quantity::Number),
        ]
    );
    // The logger's own clock, from 61 s, by 0.5 s but once by 1 s.
    assert_eq!(log.time_s.len(), 26);
    assert_eq!((log.time_s[0], log.time_s[25]), (61.0, 74.0));
    // The barometric altitude from 0 on the pad, whole meters.
    assert_eq!(log.altitude_m[0], 0.0);
    let apogee = log.altitude_m.iter().copied().fold(0.0, f64::max);
    assert_eq!(apogee, 321.0);
    // 8.4 V, 21 °C, a decimal comma's 12.345678° N.
    assert_eq!(log.battery_v.as_ref().map(|v| v[0]), Some(8.4));
    assert_eq!(log.temperature_k.as_ref().map(|t| t[0]), Some(294.15));
    assert_eq!(log.channels[17].values[0], 12.345678);
    assert_eq!(log.vertical_speed_m_s.as_ref().map(|v| v[2]), Some(80.0));
    // The `nan` cell is a gap.
    assert!(log.channels[3].values[6].is_nan());
    assert_eq!(
        events(&log),
        [
            (62.0, "ASCENT"),
            (65.5, "DESCENT"),
            (65.5, "P1-state ENABLED"),
            (71.0, "P2-state ENABLED"),
            (74.0, "TOUCHDOWN"),
        ]
    );
    assert!(
        note(
            &log,
            "`gpsAltMSL` read in meters, from the barometric altitude: "
        )
        .contains(
            "at its 24 fresh fixes it rises 1.00 for each meter of the barometric altitude, \
             within 10% of 1, where in feet it would be 3.28, over 321 m of the barometric \
             altitude, and scatters 2 about that line"
        )
    );
    note(
        &log,
        "`gpsLat` read in degrees, from the manual and the values: ",
    );
    note(
        &log,
        "`gpsLng` read in degrees, from the manual and the values: ",
    );
    note(
        &log,
        "the column `angle (deg)` holds 1 cell that is empty or not a number, such as \"nan\"",
    );
    note(
        &log,
        "the clock steps by 500 ms on most rows, and by more once: the longest, 1000 ms, before \
         line 15",
    );
    assert_eq!(log.notes.len(), 5, "{:?}", log.notes);
    insta::assert_json_snapshot!(log);
}

/// The dead reckoning's placeholders are gaps; the GPS altitude, which doesn't follow the climb,
/// is kept as written; the logger never detects the apogee.
#[test]
fn a_flight_with_placeholders_and_a_lost_gps() {
    let log = import::read(include_bytes!("samples/fluctus-lost-gps.csv")).unwrap();
    assert_eq!(log.format, LogFormat::FluctusCsv);
    assert_eq!(log.time_s.len(), 13);
    let speed = &log.channels[2];
    assert_eq!(speed.column, "dedrck-v-speed (m/s)");
    assert_eq!(speed.values[2], 61.0);
    assert!(speed.values[3..].iter().all(|v| v.is_nan()));
    note(
        &log,
        "the column `dedrck-v-speed (m/s)` writes 2147483647 or -2147483648, the ends of a signed \
         32-bit number, on 10 rows from line 6: ",
    );
    note(&log, "the column `dedrck-alti (m)` writes 2147483647");
    let gps = log
        .channels
        .iter()
        .find(|c| c.column == "gpsAltMSL")
        .unwrap();
    assert_eq!(
        (gps.quantity, gps.values[0]),
        (Quantity::Unrecognized, 88.0)
    );
    assert!(
        note(
            &log,
            "`gpsAltMSL` kept as written, in a unit no source states: "
        )
        .contains("it changes on 2 rows where the barometric altitude is read, fewer than the 10")
    );
    assert_eq!(log.channels[17].values[0], -41.234567);
    assert_eq!(events(&log), [(246.02, "ASCENT")]);
    insta::assert_json_snapshot!(log);
}

/// A GPS altitude written in feet whose receiver loses its fix in the climb and regains it at
/// 150 m in the descent: its highest fixed reading rises 492 above the pad's, within 10% of the
/// 500 m apogee, so a check of that rise alone would read it in meters, 3.28 times too high. At
/// its fresh fixes it rises 3.28 for each meter of the barometric altitude: kept as written.
#[test]
fn a_feet_gps_altitude_that_regains_its_fix_late_is_kept() {
    let log = import::read(include_bytes!("samples/fluctus-late-fix-feet.csv")).unwrap();
    assert_eq!(log.format, LogFormat::FluctusCsv);
    let gps = log
        .channels
        .iter()
        .find(|c| c.column == "gpsAltMSL")
        .unwrap();
    assert_eq!(
        (gps.quantity, gps.values[0]),
        (Quantity::Unrecognized, 500.0)
    );
    assert!(
        note(
            &log,
            "`gpsAltMSL` kept as written, in a unit no source states: "
        )
        .contains("at its 13 fresh fixes it rises 3.28 for each meter of the barometric altitude")
    );
    insta::assert_json_snapshot!(log);
}

/// A header stating other units: each column read in its own, a column the reader doesn't know
/// read by a unit that means one quantity only, and one with no unit kept as written.
#[test]
fn a_header_in_other_units_is_read_in_them() {
    let log = import::read(include_bytes!("samples/fluctus-other-units.csv")).unwrap();
    assert_eq!(log.format, LogFormat::FluctusCsv);
    assert_eq!(
        layout(&log),
        [
            ("status", Quantity::Number),
            ("baro-altitude (ft)", Quantity::AltitudeAboveGround),
            ("baro-speed (ft/s)", Quantity::Speed),
            ("amb-temp (deg f)", Quantity::Temperature),
            ("batt-voltage (V)", Quantity::Voltage),
            ("gpsAltMSL (m)", Quantity::GnssAltitude),
            ("pitot (kPa)", Quantity::Pressure),
            ("mystery", Quantity::Unrecognized),
        ]
    );
    assert_eq!(log.time_s[0], 12.0);
    assert!((log.altitude_m[5] - 640.0 * FOOT_M).abs() < 1e-9);
    assert!(
        log.vertical_speed_m_s
            .as_ref()
            .is_some_and(|v| (v[3] - 270.0 * FOOT_M).abs() < 1e-9)
    );
    assert!(
        log.temperature_k
            .as_ref()
            .is_some_and(|t| (t[0] - 293.15).abs() < 1e-9)
    );
    assert_eq!(log.battery_v.as_ref().map(|v| v[0]), Some(8.41));
    assert!((log.channels[6].values[0] - 101_300.0).abs() < 1e-6);
    assert_eq!(events(&log), [(14.0, "ASCENT"), (18.0, "DESCENT")]);
    assert_eq!(
        log.notes,
        [
            "the column `pitot (kPa)` isn't one this reader knows: it is read as a pressure, as \
             its unit says",
            "`mystery` kept as written: this reader doesn't know the column, and the header \
             states no unit",
        ]
    );
    insta::assert_json_snapshot!(log);
}
