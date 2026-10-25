//! An Eggtimer's flight data, read through the importer from invented files and pinned whole by
//! snapshots (M7.1b). The files hold made-up numbers in three of the download's layouts: the TRS's
//! eleven columns with the time in seconds, the same with `Filt_Alt` and the time in milliseconds,
//! and the Proton's 22 columns with its accelerometer.

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

fn events(log: &hpr_flightdata::log::FlightLog) -> Vec<(f64, &str)> {
    log.events
        .iter()
        .map(|event| (event.time_s, event.name.as_str()))
        .collect()
}

/// The TRS's layout in seconds, each row followed by a blank line (CR CR LF), with a `Main`
/// marker held from its event to the end of the file: one event for it.
#[test]
fn the_layout_in_seconds_reads_its_markers_as_events() {
    let log = import::read(include_bytes!("samples/eggtimer-trs-seconds.csv")).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerCsv);
    assert_eq!(log.logger, "Eggtimer");
    assert_eq!(log.time_s.len(), 43);
    assert_eq!(log.time_s[..3], [0.0, 0.05, 0.1]);
    assert_eq!(log.channels.len(), 10);
    // The highest sample, 673 ft at 6.5 s.
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert!((peak - 673.0 * FOOT_M).abs() < 1e-9, "{peak}");
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    assert!((speed[1..].iter().copied().fold(f64::MIN, f64::max) - 220.0 * FOOT_M).abs() < 1e-9);
    assert_eq!(
        events(&log),
        [
            (0.65, "LDA"),
            (4.0, "LowV"),
            (6.5, "Apogee"),
            (7.5, "N-O"),
            (7.5, "Drogue"),
            (10.5, "Main"),
        ]
    );
    let main = log.channels.iter().find(|c| c.column == "Main").unwrap();
    assert_eq!(main.quantity, Quantity::Unrecognized);
    assert_eq!(main.values.iter().filter(|&&v| v == 441.0).count(), 4);
    assert!(log.notes[0].starts_with("`T` read in seconds, from the step: "));
    insta::assert_json_snapshot!(log);
}

/// The `Filt_Alt` layout with its clock in milliseconds and its speeds in feet per second, CR LF
/// line ends and a blank line at the end.
#[test]
fn the_layout_in_milliseconds_reads_its_clock_in_seconds() {
    let log = import::read(include_bytes!("samples/eggtimer-filt-alt-milliseconds.csv")).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerCsv);
    assert_eq!(log.time_s.len(), 26);
    assert_eq!(log.time_s[..3], [0.0, 0.05, 0.1]);
    assert_eq!(log.time_s.last(), Some(&5.3));
    assert!(
        log.notes[0].starts_with(
            "`T` read in milliseconds, from the step: the median step between rows is 50."
        ),
        "{}",
        log.notes[0]
    );
    // Every raw speed is its altitude's change over the step in seconds.
    assert!(
        log.notes[2].contains("holds in 25 of 25 rows after the first"),
        "{}",
        log.notes[2]
    );
    assert_eq!(log.channels[2].column, "Filt_Alt");
    assert_eq!(log.channels[2].quantity, Quantity::AltitudeAboveGround);
    assert_eq!(
        events(&log),
        [
            (0.6, "LDA"),
            (3.3, "LowV"),
            (3.8, "Apogee"),
            (4.8, "N-O"),
            (4.8, "Drogue"),
            (4.8, "Main"),
        ]
    );
    insta::assert_json_snapshot!(log);
}

/// The Proton's layout from its pre-launch buffer at negative time, CR line ends, ending in rows of
/// empty cells: the accelerometer in SI, its velocity and distance as channels only, `MaxV` and
/// `MaxA` holding a speed and an acceleration as written.
#[test]
fn the_accelerometer_layout_keeps_every_column() {
    let log = import::read(include_bytes!("samples/eggtimer-proton-accelerometer.csv")).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerCsv);
    assert_eq!(log.time_s.len(), 40);
    assert_eq!(log.time_s[0], -0.2);
    let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
    assert_eq!(
        quantities[..12],
        [
            Quantity::AltitudeAboveGround,
            Quantity::Speed,
            Quantity::Acceleration,
            Quantity::Acceleration,
            Quantity::Speed,
            Quantity::InertialAltitude,
            Quantity::AltitudeAboveGround,
            Quantity::Speed,
            Quantity::Acceleration,
            Quantity::Acceleration,
            Quantity::Speed,
            Quantity::InertialAltitude,
        ]
    );
    assert!(
        quantities[12..]
            .iter()
            .all(|q| *q == Quantity::Unrecognized)
    );
    // 8.3 g and 267.75 ft/s² are the same acceleration within the cells' rounding.
    let (g, ft_s2) = (&log.channels[2], &log.channels[3]);
    assert!((g.values[5] - 8.3 * 9.806_65).abs() < 1e-9);
    assert!((ft_s2.values[5] - 267.75 * FOOT_M).abs() < 1e-9);
    assert!((g.values[5] - ft_s2.values[5]).abs() < 0.05 * 9.806_65);
    // The vertical speed is the barometer's `Veloc`, not the accelerometer's `VAccel`.
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    assert_eq!(speed, log.channels[1].values.as_slice());
    let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "MaxA", "LDA", "MaxV", "BO#1", "LowV", "Apogee", "N-O", "CH1", "CH2"
        ]
    );
    assert!(
        log.notes
            .iter()
            .any(|note| note == "5 rows hold only empty cells, and were skipped"),
        "{:?}",
        log.notes
    );
    insta::assert_json_snapshot!(log);
}
