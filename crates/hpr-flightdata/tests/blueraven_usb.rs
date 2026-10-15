//! The Blue Raven's USB download, as a terminal program saves it, read through the importer from
//! invented files and pinned whole by snapshots (M7.1c17): a high-rate download with the two
//! values past the manual's ten that the sample logs' firmware writes, and a low-rate one opening
//! with a status line, holding a record that lost its CRC and ran into the next, and one whose CRC
//! no longer checks.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel of a column.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no `{column}`"))
}

/// The manual's ten values in its order and units, the two past them kept as written; every
/// channel on the records' clock, from the first record's time.
#[test]
fn the_high_rate_download_reads_by_the_manual() {
    let bytes = include_bytes!("samples/blueraven-usb-high-rate.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::BlueRavenUsb));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.logger, "Featherweight Blue Raven");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    let columns: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        columns,
        [
            ("Year", Quantity::Number),
            ("Month", Quantity::Number),
            ("Day", Quantity::Number),
            ("Time", Quantity::TimeOfDay),
            ("Sync", Quantity::Number),
            ("Gyro X", Quantity::AngularRate),
            ("Gyro Y", Quantity::AngularRate),
            ("Gyro Z", Quantity::AngularRate),
            ("Accel X", Quantity::Acceleration),
            ("Accel Y", Quantity::Acceleration),
            ("Accel Z", Quantity::Acceleration),
            ("Quaternion X", Quantity::Number),
            ("Quaternion Y", Quantity::Number),
            ("Quaternion Z", Quantity::Number),
            ("Quaternion magnitude", Quantity::Number),
            ("LOG_HIR 11", Quantity::Unrecognized),
            ("LOG_HIR 12", Quantity::Unrecognized),
        ]
    );
    let gyro = channel(&log, "Gyro X");
    assert_eq!(gyro.time_s, [0.0, 0.002, 0.004, 0.006]);
    // 250 hundredths of a degree a second.
    assert_eq!(gyro.values[2], 2.5);
    // 1,320 hundredths of g.
    assert_eq!(channel(&log, "Accel Z").values[3], 13.2 * 9.806_65);
    assert_eq!(channel(&log, "LOG_HIR 11").values, [0.0, 0.0, 3.0, 3.0]);
    note(
        &log,
        "the high-rate record holds 12 values where the manual lists 10: the manual's read in its \
         order, the other 2 kept as written",
    );
    let clock = note(
        &log,
        "a high-rate download: the clock counts from the first record's time",
    );
    assert!(clock.contains("2026 5 23 10:15:00.120"), "{clock}");
    insta::assert_json_snapshot!(log);
}

/// Only the records whose CRC checks: the one that ran into the next and the damaged one are
/// dropped and counted, as is the status line; `V:`'s five values kept as written.
#[test]
fn the_low_rate_download_keeps_only_records_whose_crc_checks() {
    let bytes = include_bytes!("samples/blueraven-usb-low-rate.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::BlueRavenUsb));
    let log = import::read(bytes).unwrap();
    let sync = channel(&log, "Sync");
    assert_eq!(sync.values, [40.0, 60.0, 100.0, 140.0, 160.0]);
    assert_eq!(sync.time_s, [0.0, 0.02, 0.06, 0.1, 0.12]);
    let pressure = channel(&log, "Bo: pressure");
    assert_eq!(pressure.quantity, Quantity::Pressure);
    assert_eq!(pressure.values[0], 49_655.0 / 50_000.0 * 101_325.0);
    assert_eq!(channel(&log, "V: 1").values[0], 4_105.0);
    assert_eq!(
        channel(&log, "FER: rocket").values,
        [1536.0, 1536.0, 1537.0, 1537.0, 1537.0]
    );
    note(&log, "1 piece that opens `@ LOG_` but closes with no CRC");
    note(&log, "1 piece whose CRC differs from its text, dropped");
    note(&log, "1 line holding text other than a record, skipped");
    note(
        &log,
        "`V:` holds 5 values where the manual lists 6: kept as written",
    );
    insta::assert_json_snapshot!(log);
}
