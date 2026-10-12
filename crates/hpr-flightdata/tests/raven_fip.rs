//! The Featherweight Raven's FIP CSV export, read through the importer from invented files and
//! pinned whole by snapshots (M7.1a, ADR-251). The files hold made-up numbers in the export's two
//! main layouts: with a status column after each channel, where a row is one cell shorter for each
//! channel that has stopped, and without, where every row keeps the header's width.

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

/// Six channels with their status columns, ending after 8, 16 and 32 rows, so the later rows
/// leave out the stopped channels' status cells and shift the rest left; CR LF line ends.
#[test]
fn the_export_with_status_columns_reads_every_channel_on_its_clock() {
    let log = import::read(include_bytes!("samples/raven-fip-status.csv")).unwrap();
    assert_eq!(log.format, LogFormat::RavenFipCsv);
    let lengths: Vec<(&str, usize)> = log
        .channels
        .iter()
        .map(|channel| (channel.column.as_str(), channel.time_s.len()))
        .collect();
    assert_eq!(
        lengths,
        [
            ("Axial Accel (Gs)", 32),
            ("bILBA", 32),
            ("Baro (Atm)", 16),
            ("bILBA", 16),
            ("Temperature (F)", 8),
            ("bILBA", 8),
            ("Volts Battery (V)", 8),
            ("bILBA", 8),
            ("[Altitude (Baro-Ft-AGL)]", 16),
            ("bILBA", 16),
            ("[Velocity (Accel-MPH)]", 32),
            ("bILBA", 32),
        ]
    );
    // The last channel's samples are read from the right cells after the others stop: its clock
    // runs on to the file's end, a quarter second a row.
    let speed = &log.channels[10];
    assert_eq!(speed.time_s.last(), Some(&7.75));
    assert!(speed.values[0].is_nan(), "`-1.#QNAN` is a gap");
    // The primary track is the barometric altitude, 299.7 ft at its highest sample.
    assert_eq!(log.time_s.len(), 16);
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert!((peak - 299.7 * 0.3048).abs() < 1e-9, "{peak}");
    // The temperature and battery are on clocks of their own: channels only.
    assert!(log.temperature_k.is_none() && log.battery_v.is_none());
    assert!(log.vertical_speed_m_s.is_none() && log.events.is_empty());
    insta::assert_json_snapshot!(log);
}

/// Eight channels without status columns: a stopped channel's two cells stay empty in place.
#[test]
fn the_export_without_status_columns_keeps_its_rows_wide() {
    let log = import::read(include_bytes!("samples/raven-fip-plain.csv")).unwrap();
    assert_eq!(log.format, LogFormat::RavenFipCsv);
    let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
    assert_eq!(
        quantities,
        [
            Quantity::Acceleration,
            Quantity::Pressure,
            Quantity::Current,
            Quantity::Acceleration,
            Quantity::Voltage,
            Quantity::InertialAltitude,
            Quantity::AltitudeAboveGround,
            Quantity::AltitudeMsl,
        ]
    );
    let lengths: Vec<usize> = log.channels.iter().map(|c| c.time_s.len()).collect();
    assert_eq!(lengths, [30, 15, 8, 15, 8, 30, 15, 15]);
    // Above sea level and above the pad differ by the pad's height all the way through.
    let (agl, asl) = (&log.channels[6], &log.channels[7]);
    assert_eq!(agl.time_s, asl.time_s);
    for (above_pad, above_sea) in agl.values.iter().zip(&asl.values) {
        assert!((above_sea - above_pad - 812.0 * 0.3048).abs() < 1e-9);
    }
    assert!(log.notes.is_empty(), "{:?}", log.notes);
    insta::assert_json_snapshot!(log);
}
