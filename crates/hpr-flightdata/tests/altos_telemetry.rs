//! AltOS radio telemetry, as AltosUI saves a TeleDongle's packets to a `.telem` file, read through
//! the importer from an invented file and pinned whole by a snapshot (M7.1c9): CR LF line ends,
//! the configuration line the docs decode,
//! an earlier power-on on the pad, then the flight's, its tick started again; a packet that failed
//! the radio's CRC, a GPS packet before a fix and one after, whose tick is behind the sensor
//! packet before it, and a TeleMega packet the reader doesn't decode.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel of a column.
fn channel<'a>(log: &'a FlightLog, column: &str) -> Option<&'a Channel> {
    log.channels.iter().find(|c| c.column == column)
}

/// The flight's power-on, the height on the sensor packets' clock, the states as events, the
/// GPS on its own clock, and every leaving-out counted.
#[test]
fn the_telemetry_keeps_the_power_on_that_flies() {
    let bytes = include_bytes!("samples/altos-telemetry.telem");
    // The configuration line the flight-log layouts page and the module's docs decode.
    let text = String::from_utf8_lossy(bytes);
    assert!(text.contains(
        "TELEM 22d2040001040b0300011b0000fa0000084e3043414c4c0000312e392e32320000f4a8c1\r\n"
    ));
    assert_eq!(import::detect(bytes), Some(LogFormat::AltosTelemetry));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::AltosTelemetry);
    assert_eq!(log.logger, "AltOS TeleMetrum v2 or newer");
    assert_eq!(log.serial_number.as_deref(), Some("1234"));
    assert_eq!(log.flight_number, Some(3));
    assert_eq!(log.firmware.as_deref(), Some("1.9.22"));
    // The flight's power-on from tick 300: the earlier one's two packets are left out.
    assert_eq!(log.time_s, [3.1, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7]);
    assert_eq!(
        log.altitude_m,
        [0.0, 0.0, 4.0, 25.0, 70.0, 110.0, 118.0, 90.0]
    );
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(
        events,
        [
            (3.1, "pad"),
            (4.2, "boost"),
            (4.4, "fast"),
            (4.5, "coast"),
            (4.6, "drogue"),
            (4.7, "main")
        ]
    );
    let pressure = channel(&log, "pressure").unwrap();
    assert_eq!(pressure.quantity, Quantity::Pressure);
    assert_eq!(pressure.values[0], 98_000.0);
    assert_eq!(channel(&log, "speed").unwrap().values[3], 30.0);
    assert_eq!(channel(&log, "acceleration").unwrap().values[2], 100.0);
    // The GPS: the first packet before a fix, its position a gap; the second at tick 405, behind
    // the sensor packet at 410 sent before it.
    let latitude = channel(&log, "latitude").unwrap();
    assert_eq!(latitude.time_s, [3.12, 4.05]);
    assert!(latitude.values[0].is_nan() && latitude.values[1] == 45.0);
    assert_eq!(channel(&log, "longitude").unwrap().values[1], -122.0);
    assert_eq!(channel(&log, "gps_altitude").unwrap().values[1], 1002.0);
    assert_eq!(channel(&log, "nsat").unwrap().values, [2.0, 7.0]);
    assert_eq!(channel(&log, "sat_channels").unwrap().values, [3.0]);
    assert_eq!(channel(&log, "svid_02").unwrap().values, [12.0]);
    assert_eq!(channel(&log, "ground_pressure").unwrap().values, [98_000.0]);
    note(
        &log,
        "17 packets heard: 1 failed the radio's CRC and are left out",
    );
    note(
        &log,
        "the board's tick starts again 1 times, a power-on each: the record holds power-on 2 of 2 \
         (the one that leaves the pad), and 2 packets of the others are left out",
    );
    note(
        &log,
        "`pres` read in pascals, not the document's tenths of a pascal: ",
    );
    note(&log, "1 GPS packets with the flags' valid bit clear");
    note(&log, "1 packets of type 0x08 (TeleMega sensors) not read");
    insta::assert_json_snapshot!(log);
}
