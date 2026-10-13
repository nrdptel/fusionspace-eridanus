//! A phone app's `launch` CSV, read through the importer from invented files and pinned by
//! snapshots (M7.1c4). The newer layout's file holds a made-up flight, five rows a second: a pad
//! 100 m above sea level, 1 s of boost at 15 g net, a drag-free coast to 1,176.8 m at 16 s and a
//! 15 m/s descent, its gyroscope stopping at 1,000 °/s in the boost, its heading crossing north,
//! CR LF line ends. The older layout's file holds 1.15 s on the pad at 20 Hz, its barometer
//! writing 0 on every row. In both, the accelerometer's axes point along gravity's direction for
//! the phone's pitch and roll, and the magnetometer reads an invented field of 22 µT north and
//! 42 µT down turned by the whole attitude, so that the yaw is the azimuth of the phone's y axis.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};
use serde::Serialize;

/// Standard gravity, m/s².
const G0: f64 = 9.806_65;

/// The newer flight's file.
const NEWER: &[u8] = include_bytes!("samples/phone-launch-newer.csv");
/// The older file.
const OLDER: &[u8] = include_bytes!("samples/phone-launch-older.csv");

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel of a column.
fn channel<'a>(log: &'a FlightLog, column: &str) -> Option<&'a hpr_flightdata::log::Channel> {
    log.channels.iter().find(|c| c.column == column)
}

/// One channel in brief: too many rows to snapshot whole, its first, largest and last values.
#[derive(Serialize)]
struct Brief<'a> {
    column: &'a str,
    quantity: Quantity,
    rows: usize,
    first: f64,
    largest: f64,
    last: f64,
}

/// The record in brief: everything but the rows, which [`Brief`] sums up.
#[derive(Serialize)]
struct Summary<'a> {
    format: LogFormat,
    logger: &'a str,
    track_rows: usize,
    track_top_m: f64,
    channels: Vec<Brief<'a>>,
    notes: &'a [String],
}

fn summary(log: &FlightLog) -> Summary<'_> {
    Summary {
        format: log.format,
        logger: &log.logger,
        track_rows: log.time_s.len(),
        track_top_m: log
            .altitude_m
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max),
        channels: log
            .channels
            .iter()
            .map(|c| Brief {
                column: &c.column,
                quantity: c.quantity,
                rows: c.values.len(),
                first: c.values[0],
                largest: c.values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                last: c.values[c.values.len() - 1],
            })
            .collect(),
        notes: &log.notes,
    }
}

/// The newer flight: each unit two checks settle read in SI, the barometer's altitude and speed
/// as the primary track, the GPS altitude and the temperature kept as written.
#[test]
fn a_newer_flight_reads_the_units_two_checks_settle() {
    let log = import::read(NEWER).unwrap();
    assert_eq!(log.format, LogFormat::PhoneLaunchCsv);
    assert_eq!(log.logger, "phone app");
    assert_eq!(log.time_s.len(), 111);
    assert_eq!(log.time_s[0], -2.0);
    assert_eq!(log.channels.len(), 68);
    assert_eq!(channel(&log, "GTotal").unwrap().values[0], G0);
    assert_eq!(
        channel(&log, "AltitudeMSLGPS").unwrap().quantity,
        Quantity::Unrecognized
    );
    assert_eq!(
        channel(&log, "Temperature").unwrap().quantity,
        Quantity::Unrecognized
    );
    assert!(
        note(
            &log,
            "`AltitudeRelativeBarometer` read in meters, from the pressure and the coast"
        )
        .contains("(2) in the coast")
    );
    assert!(note(&log, "`Timestamp` read in milliseconds since 1970 UTC").ends_with("UTC"));
    insta::assert_json_snapshot!(summary(&log));
}

/// The older file: no barometer reading, so nothing that needs it is read and the primary track
/// is empty; the attitude, heading, position and time stamp settle on their own checks.
#[test]
fn an_older_file_keeps_what_it_cant_check() {
    let log = import::read(OLDER).unwrap();
    assert_eq!(log.format, LogFormat::PhoneLaunchCsv);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert!(log.vertical_speed_m_s.is_none());
    assert_eq!(
        channel(&log, "Gx").unwrap().quantity,
        Quantity::Unrecognized
    );
    assert_eq!(channel(&log, "Pitch").unwrap().quantity, Quantity::Angle);
    assert!(note(&log, "the primary track is empty").contains("barometric altitude"));
    insta::assert_json_snapshot!(log);
}
