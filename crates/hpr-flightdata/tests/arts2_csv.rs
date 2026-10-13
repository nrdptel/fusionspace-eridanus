//! Ozark Aerospace ARTS2's interpreted data, read through the importer from invented files and
//! pinned whole by snapshots (M7.1c3). The files hold made-up flights written every 0.5 s, not
//! the logger's 50 Hz, so that the snapshots stay short: `Vel` and `iAlt` the trapezoid integrals
//! of the written `Acc` and `Vel` at that step, as the software works them out, and `pAlt` a
//! little under the climb, then a steady descent, in steps of an invented barometer count. A
//! metric file, LF line ends, 2 s of boost at 75 m/s² to about 910 m, its last row's `Vel`
//! written as 0; and an English one, CR LF, `Acc` in g, 2.5 s at 6 g to about 3,200 ft, an empty
//! `Acc` cell and its last two rows' `Vel` written as 0.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// Standard gravity, m/s².
const G0: f64 = 9.806_65;

/// Meters in a foot.
const FOOT_M: f64 = 0.3048;

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

fn highest(values: &[f64]) -> f64 {
    values
        .iter()
        .filter(|value| value.is_finite())
        .fold(f64::NEG_INFINITY, |top, value| top.max(*value))
}

/// The metric file: every column in SI as written; the barometric altitude the primary track.
/// Reading it in feet would make its apogee 3.28 times lower: the asserts on the values fail.
#[test]
fn a_metric_file_reads_in_meters() {
    let log = import::read(include_bytes!("samples/arts2-metric.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Arts2Csv);
    assert_eq!(log.logger, "Ozark Aerospace ARTS2");
    assert_eq!(log.time_s.len(), 61);
    assert_eq!((log.time_s[0], log.time_s[60]), (0.0, 30.0));
    let quantities: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        quantities,
        [
            ("Acc", Quantity::Acceleration),
            ("Vel", Quantity::Speed),
            ("iAlt", Quantity::InertialAltitude),
            ("pAlt", Quantity::AltitudeAboveGround),
        ]
    );
    // The highest `pAlt` written, 912.00, is the track's highest, in meters.
    assert_eq!(highest(&log.altitude_m), 912.0);
    assert_eq!(log.altitude_m, channel(&log, "pAlt").unwrap().values);
    // The fourth row: 75.00 m/s², 93.75 m/s, 60.94 m.
    assert_eq!(channel(&log, "Acc").unwrap().values[3], 75.0);
    assert_eq!(channel(&log, "Vel").unwrap().values[3], 93.75);
    assert_eq!(channel(&log, "iAlt").unwrap().values[3], 60.94);
    // The last row's `Vel`, written 0.00 after −14.80, is a gap.
    let vel = &channel(&log, "Vel").unwrap().values;
    assert!(vel[60].is_nan() && vel[59] == -14.8);
    assert!(log.vertical_speed_m_s.is_none());
    assert!(log.temperature_k.is_none() && log.battery_v.is_none());
    note(&log, "`Vel` read in meters per second, from the apogee: ");
    note(&log, "`pAlt` read in meters, from `Vel`: ");
    note(
        &log,
        "`Acc` read in meters per second squared, from `Vel`: ",
    );
    note(&log, "`iAlt` read in meters, from `Vel`: ");
    note(&log, "the last 1 rows, from line 62, write `Vel` as 0");
    assert_eq!(log.notes.len(), 5, "{:#?}", log.notes);
    insta::assert_json_snapshot!(log);
}

/// The English file: feet, feet per second and `Acc` in standard gravities, each from the file's
/// evidence. Reading its feet as meters, or its g as m/s², fails the asserts on the values.
#[test]
fn an_english_file_reads_in_feet_and_g() {
    let log = import::read(include_bytes!("samples/arts2-english.csv")).unwrap();
    assert_eq!(log.format, LogFormat::Arts2Csv);
    assert_eq!(log.time_s.len(), 65);
    // The highest `pAlt` written, 3196.80 ft.
    let top = highest(&log.altitude_m);
    assert!((top - 3_196.8 * FOOT_M).abs() < 1e-9, "{top}");
    let acc = &channel(&log, "Acc").unwrap().values;
    assert!((acc[3] - 6.0 * G0).abs() < 1e-12);
    // The empty `Acc` cell on the 41st row.
    assert!(acc[40].is_nan());
    let vel = &channel(&log, "Vel").unwrap().values;
    assert!((vel[3] - 241.31 * FOOT_M).abs() < 1e-9);
    assert!(vel[63].is_nan() && vel[64].is_nan() && vel[62].is_finite());
    let inertial = channel(&log, "iAlt").unwrap();
    assert_eq!(inertial.quantity, Quantity::InertialAltitude);
    assert!((inertial.values[3] - 156.85 * FOOT_M).abs() < 1e-9);
    let vel_note = note(&log, "`Vel` read in feet per second, from the apogee: ");
    assert!(
        vel_note.contains("of standard gravity's 32.174 ft/s²"),
        "{vel_note}"
    );
    note(&log, "`pAlt` read in feet, from `Vel`: ");
    let acc_note = note(
        &log,
        "`Acc` read in standard gravities, 9.806 65 m/s², from `Vel`: ",
    );
    assert!(acc_note.contains("rises 32.174 times"), "{acc_note}");
    note(&log, "`iAlt` read in feet, from `Vel`: ");
    note(&log, "the last 2 rows, from line 65, write `Vel` as 0");
    note(
        &log,
        "`Acc` has 1 cells that are empty or aren't a number, the first on line 42: gaps",
    );
    assert_eq!(log.notes.len(), 6, "{:#?}", log.notes);
    insta::assert_json_snapshot!(log);
}

/// The two files are told apart from every other layout the importer reads, and the ARTS2's raw
/// data isn't taken for its interpreted data.
#[test]
fn only_the_interpreted_data_is_taken_for_it() {
    for bytes in [
        &include_bytes!("samples/arts2-metric.csv")[..],
        &include_bytes!("samples/arts2-english.csv")[..],
    ] {
        assert_eq!(import::detect(bytes), Some(LogFormat::Arts2Csv));
    }
    assert_ne!(
        import::detect(b"MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n0.000,512,4,777,64\n"),
        Some(LogFormat::Arts2Csv)
    );
}
