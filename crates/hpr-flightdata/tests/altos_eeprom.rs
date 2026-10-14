//! AltOS raw flight downloads, as AltosUI saves a flight computer's memory to an `.eeprom` file,
//! read through the importer from two invented files and pinned whole by snapshots (M7.1c10).
//!
//! - `altos-eeprom-telemetrum-v2.eeprom`, the JSON form of a TeleMetrum v2's flight (log format
//!   7, the MS5607 data sheet's example constants): the pad's samples written after the flight
//!   record, the 16-bit tick turning over mid-climb, a GPS fix whose records carry its earlier
//!   tick, one before the receiver has a fix, and erased memory at the end; the climb runs a
//!   sample every 0.1 s to 1.1 s after boost, its pressures what the standard atmosphere gives
//!   the height its accelerations integrate to, so the reader's climb check passes.
//! - `altos-eeprom-telemetrum-v1.eeprom`, the text form of a TeleMetrum v1's flight from firmware
//!   before 1.0, which states no log format: the console's echo and `Syntax error` lines, a
//!   record written twice, another session's records more than 10 s later, and a last line cut
//!   short with no line break; its climb, too, runs to 1.1 s after boost.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The notes that open as given, one each.
#[cfg(test)]
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel of a column.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// The JSON form: tick order across the turn, the units, the fix, the erased memory.
#[test]
fn the_json_form_reads_in_tick_order_across_the_turn() {
    let bytes = include_bytes!("samples/altos-eeprom-telemetrum-v2.eeprom");
    assert_eq!(import::detect(bytes), Some(LogFormat::AltosEeprom));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.logger, "AltOS TeleMetrum-v2.0");
    assert_eq!(log.serial_number.as_deref(), Some("1234"));
    // The flight record's number, not the configuration's `stored_flight`, 9.
    assert_eq!(log.flight_number, Some(3));
    assert!(log.time_s.is_empty());
    let pressure = channel(&log, "pressure");
    assert_eq!(pressure.quantity, Quantity::Pressure);
    // The pad's three samples, then the climb, across the turn at 65,536.
    assert_eq!(
        pressure.time_s,
        [
            -0.03, -0.02, -0.01, 0.0, 0.01, 0.02, 0.05, 0.06, 0.08, 0.09, 0.1, 0.2, 0.3, 0.4, 0.5,
            0.6, 0.7, 0.8, 0.9, 1.0, 1.1
        ]
    );
    // The data sheet's example counts are 110,002 Pa at 20.00 °C.
    assert_eq!(pressure.values[..4], [110_002.0; 4]);
    assert!((channel(&log, "temperature").values[0] - 293.15).abs() < 1e-9);
    // 20 counts a g, 0 at the flight record's 1,000: 900 counts is 5 g.
    let acceleration = &channel(&log, "acceleration").values;
    assert!((acceleration[3] - 5.0 * 9.806_65).abs() < 1e-12);
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(events, [(0.01, "boost"), (1.11, "coast"), (1.12, "drogue")]);
    // The fix at tick 65,520, 0.10 s before boost; the next without one, a gap.
    let latitude = channel(&log, "latitude");
    assert_eq!(latitude.time_s, [-0.1, 0.11]);
    assert!(latitude.values[0] == 45.0 && latitude.values[1].is_nan());
    assert_eq!(channel(&log, "gps_altitude").values[0], 4464.0);
    assert_eq!(channel(&log, "svid").values, [3.0, 7.0, 12.0, 19.0, 24.0]);
    note(
        &log,
        "32 records of 16 bytes, log format 7, from the JSON form: every record's checksum \
         checked, all pass",
    );
    note(
        &log,
        "2 erased records (every byte 0xFF) after the last, skipped",
    );
    note(&log, "1 GPS position records without a valid fix");
    note(&log, "`pressure` and `acceleration` held to each other");
    insta::assert_json_snapshot!(log);
}

/// The text form of firmware before 1.0: format 1 from the product, the repeated record, the
/// other session and the cut line left out.
#[test]
fn the_text_form_of_old_firmware_reads_by_its_product() {
    let bytes = include_bytes!("samples/altos-eeprom-telemetrum-v1.eeprom");
    assert_eq!(import::detect(bytes), Some(LogFormat::AltosEeprom));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.logger, "AltOS TeleMetrum-v1.0");
    assert_eq!(log.firmware.as_deref(), Some("0.6+00+g0000000"));
    let pressure = channel(&log, "pressure");
    assert_eq!(
        pressure.time_s,
        [
            -0.02, -0.01, 0.0, 0.01, 0.02, 0.03, 0.04, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
            1.0, 1.1
        ]
    );
    // 22,000 counts: (22,000 / 32,752 + 0.095) / 0.009 kPa.
    assert!((pressure.values[0] - (22_000.0 / 32_752.0 + 0.095) / 0.009 * 1000.0).abs() < 1e-9);
    // 250 counts a g: 15,500 counts is 2 g.
    assert!((channel(&log, "acceleration").values[2] - 2.0 * 9.806_65).abs() < 1e-12);
    assert_eq!(channel(&log, "temp").quantity, Quantity::Unrecognized);
    assert_eq!(channel(&log, "latitude").values, [45.0]);
    assert_eq!(channel(&log, "longitude").values, [-122.0]);
    note(
        &log,
        "the configuration states no log format, as firmware before 1.0 didn't: format 1",
    );
    note(&log, "the file ends without a line break");
    note(
        &log,
        "2 records from the first more than 10 s from the latest tick on",
    );
    note(&log, "1 records repeated exactly");
    note(&log, "`pressure` and `acceleration` held to each other");
    let main = log
        .stated
        .figures
        .iter()
        .find(|f| f.name == "main_deploy")
        .unwrap();
    assert_eq!(main.quantity, Some(Quantity::AltitudeAboveGround));
    assert_eq!(main.values, [250.0]);
    insta::assert_json_snapshot!(log);
}
