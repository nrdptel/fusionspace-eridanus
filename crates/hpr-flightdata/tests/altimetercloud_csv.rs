//! Rocketry Ltd's logs as AltimeterCloud hands them back, read through the importer from invented
//! files and pinned whole by snapshots (M7.1c2). The files hold a made-up flight, 2 s of boost to
//! 100 m/s, a coast to about 610 m and a 6 m/s descent to a landing at 114.219 s: a Nano V1's log
//! with its summary, its pressure and raw altitude, CR LF line ends; the same, cut short at 52 s,
//! with an adjusted log, an apogee time the altitude doesn't bear out, a landing time with no
//! landing height, a column no source names, a byte-order mark and LF; and a Mercury's, in lower
//! case, its header ending in a comma, with its centi-degree temperature and its sparse prediction
//! and air-brake columns.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

/// Standard gravity, m/s²: a thousandth of it is one `mG`.
const G0: f64 = 9.806_65;

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

/// The Nano V1's log: the summary's apogee, serial, firmware and events; every column a channel
/// on its own rows; the accelerometer's pad zeros and the closing zero temperature as gaps.
#[test]
fn a_nano_log_keeps_its_summary_and_every_column() {
    let log = import::read(include_bytes!("samples/altimetercloud-nano.csv")).unwrap();
    assert_eq!(log.format, LogFormat::AltimeterCloudCsv);
    assert_eq!(log.logger, "Rocketry Ltd Nano V1");
    assert_eq!(log.serial_number.as_deref(), Some("0000-0000-00a1"));
    assert_eq!(log.firmware.as_deref(), Some("9.99"));
    assert_eq!(log.stated.apogee_m, Some(609.81));
    assert_eq!(log.time_s.len(), 24);
    assert_eq!(log.time_s[0], -0.4);
    assert_eq!(log.time_s[23], 120.0);
    let quantities: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        quantities,
        [
            ("Altitude(m)", Quantity::AltitudeAboveGround),
            ("Velocity(m/s)", Quantity::Speed),
            ("acceleration_x(mG)", Quantity::Acceleration),
            ("acceleration_y(mG)", Quantity::Acceleration),
            ("acceleration_z(mG)", Quantity::Acceleration),
            ("acceleration_total(mG)", Quantity::Acceleration),
            ("gyro_x", Quantity::AngularRate),
            ("gyro_y", Quantity::AngularRate),
            ("gyro_z", Quantity::AngularRate),
            ("Board temp(C)", Quantity::Temperature),
            ("pitch", Quantity::Angle),
            ("roll", Quantity::Angle),
            ("yaw", Quantity::Angle),
            ("tilt", Quantity::Angle),
            ("Pressure(hPa)", Quantity::Pressure),
            ("Raw altitude(m)", Quantity::AltitudeAboveGround),
        ]
    );
    // The fourth row: 1003 mG on z, 3.1 mG in all with 1 g taken out; 1000 hPa.
    let z = channel(&log, "acceleration_z(mG)").unwrap();
    assert!(z.values[..3].iter().all(|v| v.is_nan()));
    assert!((z.values[3] - 1003.0 * G0 / 1000.0).abs() < 1e-12);
    assert!(
        (channel(&log, "acceleration_total(mG)").unwrap().values[3] - 3.1 * G0 / 1000.0).abs()
            < 1e-12
    );
    assert!((channel(&log, "Pressure(hPa)").unwrap().values[0] - 100_000.0).abs() < 1e-6);
    // The temperature on every 4th row, its own clock; the last, 0.00 after 21.80, a gap.
    let temperature = channel(&log, "Board temp(C)").unwrap();
    assert_eq!(temperature.time_s, [-0.4, -0.05, 1.5, 12.0, 24.0, 44.0]);
    assert!((temperature.values[0] - 294.15).abs() < 1e-9);
    assert!(temperature.values[5].is_nan());
    let track = log.temperature_k.as_deref().unwrap();
    assert_eq!(track.len(), 24);
    assert!(track[1].is_nan() && track[20].is_nan());
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(
        events,
        [
            (2.0, "Burnout"),
            (12.198, "Apogee"),
            (12.5, "Ejection"),
            (114.219, "Landing")
        ]
    );
    note(&log, "the logger is taken as a Rocketry Ltd Nano V1: ");
    note(
        &log,
        "the first 3 rows, before lift-off, to line 13, write 0 on all three",
    );
    note(
        &log,
        "the last 1 readings of `Board temp(C)`, from line 31,",
    );
    note(
        &log,
        "`yaw` reaches ±327.67° or beyond on 1 rows, the first on line 23",
    );
    note(&log, "`Apogee time` read in seconds, from the altitude: ");
    assert!(
        note(
            &log,
            "`Landing time` read in seconds, from the altitude: at 114.219 s, "
        )
        .contains("the altitude is -0.400 m")
    );
    note(
        &log,
        "the summary's other 15 entries, such as `Landing meters`",
    );
    insta::assert_json_snapshot!(log);
}

/// An adjusted log, its apogee time 8 s from the highest altitude's: the events from the summary
/// in milliseconds stay, the apogee's is left out, and the landing's, with no landing height to
/// check it against; a column no source names is kept as written.
#[test]
fn an_adjusted_nano_log_leaves_out_an_apogee_time_the_altitude_denies() {
    let log = import::read(include_bytes!("samples/altimetercloud-nano-adjusted.csv")).unwrap();
    assert_eq!(log.format, LogFormat::AltimeterCloudCsv);
    assert_eq!(log.logger, "Rocketry Ltd Nano V1");
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(events, [(2.0, "Burnout")]);
    assert!(note(&log, "the summary's `Apogee time`, 4, is left out. ").contains("12.2 s"));
    assert!(
        note(&log, "the summary's `Landing time`, 49.8, is left out: ")
            .contains("no landing height")
    );
    note(&log, "the summary's `Adjusted log` is \"yes\". ");
    let spare = channel(&log, "spare_channel").unwrap();
    assert_eq!(spare.quantity, Quantity::Unrecognized);
    assert_eq!(spare.values[2], 14.0);
    note(
        &log,
        "`spare_channel` kept as written, in a unit no source states: ",
    );
    insta::assert_json_snapshot!(log);
}

/// The Mercury's layout: its centi-degree temperature read in kelvin, its own columns kept as
/// written or as a percentage, and its header's closing comma no column.
#[test]
fn a_mercury_log_reads_its_centi_degree_temperature() {
    let log = import::read(include_bytes!("samples/altimetercloud-mercury.csv")).unwrap();
    assert_eq!(log.format, LogFormat::AltimeterCloudCsv);
    assert_eq!(log.logger, "Rocketry Ltd Mercury");
    assert_eq!(log.channels.len(), 19);
    assert_eq!(log.stated.apogee_m, None);
    assert!(log.events.is_empty());
    // 2150 is 21.50 °C, 294.65 K; the closing 0 after 2180 is a gap.
    let temperature = channel(&log, "bmp_temp(x100)").unwrap();
    assert_eq!(temperature.quantity, Quantity::Temperature);
    assert!((temperature.values[0] - 294.65).abs() < 1e-9);
    assert!(temperature.values.last().unwrap().is_nan());
    note(
        &log,
        "`bmp_temp(x100)` read in hundredths of a degree Celsius, from its name and the values: ",
    );
    note(
        &log,
        "the last 1 readings of `bmp_temp(x100)`, from line 20,",
    );
    assert_eq!(channel(&log, "mt1_temp").unwrap().values.len(), 0);
    let target = channel(&log, "target_alt").unwrap();
    assert_eq!(
        (target.time_s.as_slice(), target.values.as_slice()),
        ([1.0].as_slice(), [600.0].as_slice())
    );
    assert_eq!(
        channel(&log, "air_brake_percent").unwrap().quantity,
        Quantity::Number
    );
    assert_eq!(channel(&log, "apogee_prediction").unwrap().values.len(), 16);
    let speed = log.vertical_speed_m_s.as_deref().unwrap();
    assert_eq!(speed.len(), 20);
    note(
        &log,
        "the first 4 rows, before lift-off, to line 5, write 0 on all three",
    );
    insta::assert_json_snapshot!(log);
}
