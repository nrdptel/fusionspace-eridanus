//! MARSA's flight downloads, read from an invented download and the same download as a
//! spreadsheet re-saves it, found by the importer and pinned whole by a snapshot (M7.1c21,
//! ADR-274).

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;
const G0: f64 = 9.806_65;

#[test]
fn a_download_reads_in_seconds_feet_and_g() {
    let log = import::read(include_bytes!("samples/marsa.dat")).unwrap();
    assert_eq!(log.format, LogFormat::Marsa);
    assert_eq!(log.logger, "MARSA");
    assert_eq!(log.firmware.as_deref(), Some("26"));
    // 34 samples ten a second, then 7 a second apart.
    assert_eq!(log.time_s.len(), 41);
    assert_eq!(log.time_s[..2], [-0.3, -0.2]);
    assert_eq!(log.time_s[40], 10.0);
    // The pad reads two counts below the logger's zero, as written.
    assert_eq!(log.altitude_m[0], -58.1 * FOOT_M);
    assert_eq!(log.altitude_m[40], 1172.5 * FOOT_M);
    let column = |name: &str| log.channels.iter().find(|c| c.column == name).unwrap();
    assert_eq!(column("altitude").quantity, Quantity::AltitudeAboveGround);
    let acceleration = column("acceleration");
    assert_eq!(acceleration.quantity, Quantity::Acceleration);
    assert_eq!(acceleration.values[4], 8.4 * G0);
    // Every setting is a figure, after its section's heading where it has one.
    let names: Vec<&str> = log.stated.figures.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "BARO_ZERO 0",
            "ACCEL_ZERO 0",
            "LAUNCH TIME 0",
            "FIRMWARE VER",
            "MARSA VERSION",
            "CHAN 1, CHAN APOGEE OPTION",
            "CHAN 1, CHAN DELAY",
            "CHAN 2, CHAN APOGEE OPTION",
            "CHAN 2, CHAN ALTITUDE SETTING",
        ]
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_spreadsheets_resave_reads_the_same() {
    let dat = import::read(include_bytes!("samples/marsa.dat")).unwrap();
    let csv = import::read(include_bytes!("samples/marsa-resaved.csv")).unwrap();
    assert_eq!(csv.format, LogFormat::Marsa);
    assert_eq!(csv.time_s, dat.time_s);
    assert_eq!(csv.altitude_m, dat.altitude_m);
    assert_eq!(csv.channels, dat.channels);
    assert_eq!(csv.stated, dat.stated);
}
