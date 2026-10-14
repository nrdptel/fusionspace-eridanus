//! The Featherweight Blue Raven's CSV, as its phone app exports a flight, read through the importer
//! from invented files and pinned whole by snapshots (M7.1b, ADR-251). The files hold made-up
//! numbers in the app's two files: the low-rate file of 101 columns, in two of its header's
//! variants, and the high-rate file of 18, which holds no altitude.

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

/// Every column of a low-rate file but the clock, a channel in the file's order; the barometric
/// altitude the primary track; the flags' rises the events; LF line ends.
#[test]
fn a_low_rate_file_reads_every_column() {
    let log = import::read(include_bytes!("samples/blueraven-low-rate.csv")).unwrap();
    assert_eq!(log.format, LogFormat::BlueRavenCsv);
    assert_eq!(log.channels.len(), 100);
    assert!(log.channels.iter().any(|c| c.column == "Apo_FER_H  ex"));
    assert_eq!(log.time_s.len(), 9);
    assert_eq!(log.time_s.first(), Some(&-0.06));
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert!((peak - 4.5 * 0.3048).abs() < 1e-12, "{peak}");
    assert!(log.vertical_speed_m_s.is_none());
    assert_eq!(log.temperature_k.as_ref().map(Vec::len), Some(9));
    assert_eq!(log.battery_v.as_ref().map(Vec::len), Some(9));
    let events: Vec<(&str, f64)> = log
        .events
        .iter()
        .map(|event| (event.name.as_str(), event.time_s))
        .collect();
    assert_eq!(
        events,
        [
            ("Liftoff", 0.0),
            ("Burnout_Coast", 0.06),
            ("Apogee", 0.08),
            ("Apo_fired", 0.1)
        ]
    );
    // The rocket's register holds the flags as bits, `Liftoff` the lowest.
    let register = log
        .channels
        .iter()
        .find(|c| c.column == "Rocket_FER_Hex")
        .unwrap();
    assert_eq!(register.values[0], f64::from(1 << 9));
    assert_eq!(
        register.values[8],
        f64::from(1 + 2 + 4 + 8 + 16 + (1 << 9) + (1 << 10))
    );
    insta::assert_json_snapshot!(log);
}

/// The variant with the axial and lateral accelerations where the third and fourth outputs'
/// voltages are; its wall clock passes midnight, which a note reports; CR LF line ends.
#[test]
fn the_variant_with_axial_acceleration_reads_alike() {
    let log = import::read(include_bytes!("samples/blueraven-low-rate-axial.csv")).unwrap();
    assert_eq!(log.format, LogFormat::BlueRavenCsv);
    let axial = log
        .channels
        .iter()
        .find(|c| c.column == "Axial Accel (Gs)")
        .unwrap();
    assert_eq!(axial.quantity, Quantity::Acceleration);
    assert!((axial.values[2] - 8.42 * 9.806_65).abs() < 1e-9);
    assert!(log.channels.iter().all(|c| c.column != "3rd_Volts"));
    assert!(
        log.notes
            .iter()
            .any(|note| note.contains("goes back 1 times")),
        "{:?}",
        log.notes
    );
    insta::assert_json_snapshot!(log);
}

/// The high-rate file: an empty primary track, and the gyro, acceleration, quaternion, voltage
/// and current columns as channels, each kind's unit decided with a note but the current's, which
/// no source states.
#[test]
fn a_high_rate_file_with_no_date_reads_as_one_with_it() {
    let dated = import::read(include_bytes!("samples/blueraven-high-rate.csv")).unwrap();
    let bytes = include_bytes!("samples/blueraven-high-rate-undated.csv");
    assert_eq!(import::detect(bytes), Some(LogFormat::BlueRavenCsv));
    let log = import::read(bytes).unwrap();
    // The dated file's channels but `Year`, `Month`, `Day` and `Time`, the first four.
    assert_eq!(log.channels[..], dated.channels[4..]);
    let note = "the file holds no date or time of day";
    assert_eq!(
        log.notes.iter().filter(|n| n.contains(note)).count(),
        1,
        "{:?}",
        log.notes
    );
    assert_eq!(log.notes.len(), dated.notes.len() + 1);
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_high_rate_file_reads_with_an_empty_track() {
    let log = import::read(include_bytes!("samples/blueraven-high-rate.csv")).unwrap();
    assert_eq!(log.format, LogFormat::BlueRavenCsv);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
    assert_eq!(
        quantities,
        [
            Quantity::Number,
            Quantity::Number,
            Quantity::Number,
            Quantity::TimeOfDay,
            Quantity::Number,
            Quantity::AngularRate,
            Quantity::AngularRate,
            Quantity::AngularRate,
            Quantity::Acceleration,
            Quantity::Acceleration,
            Quantity::Acceleration,
            Quantity::Number,
            Quantity::Number,
            Quantity::Number,
            Quantity::Number,
            Quantity::Voltage,
            Quantity::Unrecognized,
        ]
    );
    assert!(log.channels.iter().all(|c| c.time_s.len() == 10));
    assert_eq!(log.notes.len(), 4, "{:?}", log.notes);
    insta::assert_json_snapshot!(log);
}
