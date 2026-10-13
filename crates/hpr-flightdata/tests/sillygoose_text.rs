//! SillyGoose logs, as AerospaceNU's board writes them, read through the importer from invented
//! files and pinned whole by snapshots (M7.1c3). The files hold made-up flights whose pressure is
//! the standard atmosphere's for their altitude above a pad 300 m up: the board's tab-separated
//! log in m/s² and rad/s, its board turning at 1 rad/s under a parachute; the same layout
//! space-separated, in g and °/s, with a temperature below zero that no kelvin can be; and the
//! layout with a `CONFIG` line and 28 columns, its temperature never changing.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const G: f64 = 9.806_65;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// Each channel's name and quantity.
fn layout(log: &FlightLog) -> Vec<(&str, Quantity)> {
    log.channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect()
}

/// The values of a column's channel; none if the log has no such channel.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a [f64] {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .map_or(&[], |c| c.values.as_slice())
}

/// The board's log: every column in its unit, the gyroscope's radians per second decided by the
/// board's turning, the fired flags' rises as events.
#[test]
fn the_boards_log_reads_every_column_in_si() {
    let log = import::read(include_bytes!("samples/sillygoose-board.txt")).unwrap();
    assert_eq!(log.format, LogFormat::SillyGooseText);
    assert_eq!(log.logger, "AerospaceNU SillyGoose");
    assert_eq!(
        layout(&log),
        [
            ("pressurePa", Quantity::Pressure),
            ("tempK", Quantity::Temperature),
            ("accelX", Quantity::Acceleration),
            ("accelY", Quantity::Acceleration),
            ("accelZ", Quantity::Acceleration),
            ("gyroX", Quantity::AngularRate),
            ("gyroY", Quantity::AngularRate),
            ("gyroZ", Quantity::AngularRate),
            ("imuTemp", Quantity::Temperature),
            ("battV", Quantity::Voltage),
            ("altitudeM", Quantity::AltitudeAboveGround),
            ("velocityMS", Quantity::Speed),
            ("accelerationMSS", Quantity::Acceleration),
            ("unfiltAlt", Quantity::AltitudeAboveGround),
            ("flightState", Quantity::Number),
            ("drogueCont", Quantity::Number),
            ("drogueFired", Quantity::Number),
            ("mainCont", Quantity::Number),
            ("mainFired", Quantity::Number),
        ]
    );
    // 48 samples from 1 s; the track is `altitudeM`, 0 on the pad and 300 m at the top.
    assert_eq!(log.time_s.len(), 48);
    assert_eq!(log.time_s[0], 1.0);
    assert_eq!(log.altitude_m[0], 0.0);
    let top = log.altitude_m.iter().copied().fold(0.0, f64::max);
    assert_eq!(top, 300.0);
    assert!(log.vertical_speed_m_s.is_some());
    assert_eq!(log.temperature_k.as_ref().unwrap()[0], 290.15);
    assert_eq!(log.battery_v.as_ref().unwrap()[0], 4.05);
    // 1 rad/s about x under the parachute: 57.3 °/s.
    let turning = channel(&log, "gyroX").iter().copied().fold(0.0, f64::max);
    assert!(
        (turning - 180.0 / std::f64::consts::PI).abs() < 1e-9,
        "{turning}"
    );
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(events, [(15.6, "drogueFired"), (46.2, "mainFired")]);
    note(
        &log,
        "`accelX`, `accelY` and `accelZ` read in meters per second squared, from the pad's \
         gravity: ",
    );
    note(
        &log,
        "`gyroX`, `gyroY` and `gyroZ` read in radians per second, from the board's turning: ",
    );
    note(&log, "`imuTemp` read in kelvin, from the values: ");
    note(&log, "`unfiltAlt` read in meters, from the pressure: ");
    assert!(log.notes.iter().all(|n| !n.contains("assum")));
    insta::assert_json_snapshot!(log);
}

/// The space-separated file: the accelerometer decided in g and the gyroscope in °/s, so both
/// read as the tab-separated board's do in SI; its `tempK` below zero and its `imuTemp` of zeros
/// kept as written.
#[test]
fn the_space_separated_file_is_read_by_its_values_not_its_names() {
    let log = import::read(include_bytes!("samples/sillygoose-spaces.txt")).unwrap();
    assert_eq!(log.format, LogFormat::SillyGooseText);
    assert_eq!(log.time_s.len(), 67);
    // At rest the z axis reads 1 g, here 9.806 65 m/s², not 1 m/s².
    assert!((channel(&log, "accelZ")[0] - G).abs() < 1e-9);
    let turning = channel(&log, "gyroX").iter().copied().fold(0.0, f64::max);
    assert!((turning - 57.3).abs() < 0.01, "{turning}");
    let quantity = |column: &str| {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap()
            .quantity
    };
    assert_eq!(quantity("tempK"), Quantity::Unrecognized);
    assert_eq!(quantity("imuTemp"), Quantity::Unrecognized);
    assert_eq!(quantity("gyroX"), Quantity::AngularRate);
    assert!(log.temperature_k.is_none());
    assert!(
        note(&log, "`tempK` kept as written: its name says kelvin, but ")
            .contains("below 0 no temperature in kelvin can be")
    );
    note(&log, "`imuTemp` kept as written: ");
    note(
        &log,
        "`accelX`, `accelY` and `accelZ` read in standard gravities, × 9.806 65 m/s², from the \
         pad's gravity: ",
    );
    note(
        &log,
        "`gyroX`, `gyroY` and `gyroZ` read in degrees per second, from the board's turning: ",
    );
    assert!(log.notes.iter().all(|n| !n.contains("assum")));
    insta::assert_json_snapshot!(log);
}

/// The layout with a `CONFIG` line: the board's name and firmware from it, the 8 more columns
/// read, the gyroscope kept as written for want of turning to decide it, and a temperature that
/// never changes noted; CR LF line ends.
#[test]
fn the_layout_with_its_settings_reads_its_extra_columns() {
    let log = import::read(include_bytes!("samples/sillygoose-config.txt")).unwrap();
    assert_eq!(log.format, LogFormat::SillyGooseText);
    assert_eq!(log.logger, "TestGoose");
    assert_eq!(log.firmware.as_deref(), Some("0.9"));
    assert_eq!(log.channels.len(), 27);
    assert_eq!(
        layout(&log)[19..],
        [
            ("tiltMagnitudeDeg", Quantity::Angle),
            ("angularVelRadS_x", Quantity::AngularRate),
            ("angularVelRadS_y", Quantity::AngularRate),
            ("angularVelRadS_z", Quantity::AngularRate),
            ("quaternion_a", Quantity::Number),
            ("quaternion_b", Quantity::Number),
            ("quaternion_c", Quantity::Number),
            ("quaternion_d", Quantity::Number),
        ]
    );
    // 0.22 rad/s at the last sample, in °/s.
    let rate = channel(&log, "angularVelRadS_x")[11];
    assert!(
        (rate - 0.22 * 180.0 / std::f64::consts::PI).abs() < 1e-9,
        "{rate}"
    );
    assert_eq!(layout(&log)[5].1, Quantity::Unrecognized);
    note(&log, "`gyroX`, `gyroY` and `gyroZ` kept as written: ");
    note(
        &log,
        "`velocityMS` read in meters per second, from its name: ",
    );
    note(
        &log,
        "the `CONFIG` line's 2 other settings, such as `DROGUE_DELAY`, ",
    );
    assert!(
        note(&log, "`tempK` reads 300 at every one of its 12 samples").contains("may be a replay")
    );
    assert!(log.events.is_empty());
    insta::assert_json_snapshot!(log);
}

/// The importer tells this layout from the others by its header alone.
#[test]
fn the_importer_recognizes_each_sample() {
    for bytes in [
        &include_bytes!("samples/sillygoose-board.txt")[..],
        include_bytes!("samples/sillygoose-spaces.txt"),
        include_bytes!("samples/sillygoose-config.txt"),
    ] {
        assert_eq!(import::detect(bytes), Some(LogFormat::SillyGooseText));
    }
}
