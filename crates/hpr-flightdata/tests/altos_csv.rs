//! AltOS's CSV, read from invented files in the variants AltosUI writes and pinned whole by
//! snapshots (M7.1a, ADR-251): a flight received by radio with GPS, one with no barometric
//! pressure or altitude, and one with comment lines and its pressure in hPa. A gap (`NaN`) is
//! `null` in a snapshot.

use hpr_flightdata::altos;
use hpr_flightdata::log::{LogFormat, Quantity};

/// The states the invented flights go through, in order.
const STATES: [&str; 7] = ["pad", "boost", "fast", "coast", "drogue", "main", "landed"];

#[test]
fn a_telemetry_flight_with_gps() {
    let log = altos::read(include_str!("samples/altos-telemetrum-gps.csv")).unwrap();
    assert_eq!(log.format, LogFormat::AltosCsv);
    assert_eq!(log.logger, "AltOS");
    assert_eq!(log.serial_number.as_deref(), Some("1234"));
    assert_eq!(log.flight_number, Some(7));
    // 69 columns: the clock, `call` and `state_name` aren't channels.
    assert_eq!(log.channels.len(), 69 - 3);
    let altitudes: Vec<Quantity> = log
        .channels
        .iter()
        .filter(|c| c.column == "altitude")
        .map(|c| c.quantity)
        .collect();
    assert_eq!(altitudes, [Quantity::AltitudeMsl, Quantity::GnssAltitude]);
    // 31 rows, one repeating the time before it: every row in the channels, 30 on the track.
    assert!(log.channels.iter().all(|c| c.values.len() == 31));
    assert_eq!(log.time_s.len(), 30);
    // The first three rows have no GPS fix.
    let gnss = log.channels_of(Quantity::GnssAltitude).next().unwrap();
    assert!(gnss.values[..3].iter().all(|v| v.is_nan()) && gnss.values[3].is_finite());
    let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, STATES);
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_flight_without_pressure() {
    let log = altos::read(include_str!("samples/altos-easymega-no-pressure.csv")).unwrap();
    assert_eq!(log.channels_of(Quantity::Pressure).count(), 0);
    assert_eq!(log.channels_of(Quantity::AltitudeMsl).count(), 0);
    assert!(log.temperature_k.is_none());
    // The last row is cut off after its main voltage: its battery is a gap.
    let battery = log.battery_v.as_ref().unwrap();
    assert!(battery[..29].iter().all(|v| v.is_finite()) && battery[29].is_nan());
    assert!(
        log.notes
            .iter()
            .any(|n| n.starts_with("line 31 has 10 cells")),
        "{:?}",
        log.notes
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn comment_lines_and_pressure_in_hectopascals() {
    let log = altos::read(include_str!("samples/altos-comments-hpa.csv")).unwrap();
    assert_eq!(log.logger, "AltOS TeleMetrum");
    assert_eq!(log.serial_number.as_deref(), Some("2001"));
    let pressure = log.channels_of(Quantity::Pressure).next().unwrap();
    assert!(
        (pressure.values[0] - 98_000.0).abs() < 0.5,
        "{}",
        pressure.values[0]
    );
    // `accel_speed` comes first for the vertical speed, as in Debrief's reader.
    let speed = log.vertical_speed_m_s.as_ref().unwrap();
    let accel_speed = log
        .channels
        .iter()
        .find(|c| c.column == "accel_speed")
        .unwrap();
    assert_eq!(speed, &accel_speed.values);
    insta::assert_json_snapshot!(log);
}
