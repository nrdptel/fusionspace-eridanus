//! AerospaceNU's ground station text, read through the importer from invented files and pinned by
//! snapshots (M7.1c4). The files hold made-up flights whose pressure is the standard
//! atmosphere's, in whole pascals, for their altitude above a pad 250 m up, sent as 32-bit
//! floats in atmospheres: a whole flight, its board swinging under a parachute, with two packets
//! that failed the radio's checksum and a `nan`; and a short record from a board with no working
//! accelerometer or GPS, whose clock restarts on the pad, with a packet this reader doesn't know.

use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

const FLIGHT: &[u8] = include_bytes!("samples/aerospacenu-gs-flight.txt");
const KEPT: &[u8] = include_bytes!("samples/aerospacenu-gs-kept.txt");

/// The note that opens as given, one only.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel named `column`, if the log has it.
fn channel<'a>(log: &'a FlightLog, column: &str) -> Option<&'a Channel> {
    log.channels.iter().find(|c| c.column == column)
}

/// A channel in short: its name, quantity, count, and first, lowest and highest values.
fn summary(channel: &Channel) -> serde_json::Value {
    let finite = channel.values.iter().copied().filter(|v| v.is_finite());
    let lowest = finite.clone().fold(f64::INFINITY, f64::min);
    let highest = finite.fold(f64::NEG_INFINITY, f64::max);
    serde_json::json!({
        "column": channel.column,
        "quantity": channel.quantity,
        "count": channel.values.len(),
        "first": [channel.time_s.first(), channel.values.first()],
        "range": [lowest, highest],
    })
}

/// The whole flight: every unit a name doesn't state read by its two checks, the track from
/// `fcb_altitude` on the board's clock, the corrupt packets left out.
#[test]
fn the_flight_reads_every_unit_by_its_two_checks() {
    let log = import::read(FLIGHT).unwrap();
    assert_eq!(log.format, LogFormat::AerospaceNuGroundStationText);
    assert_eq!(
        log.logger,
        "AerospaceNU flight computer board, by its ground station"
    );
    for (column, quantity) in [
        ("Alt Info & Cfg.barometer_pressure", Quantity::Pressure),
        ("Alt Info & Cfg.press_ref", Quantity::Pressure),
        ("Ground Station GPS.gs_baro", Quantity::Pressure),
        ("Position Data.fcb_altitude", Quantity::AltitudeAboveGround),
        ("Position Data.v_speed", Quantity::Speed),
        ("Position Data.fcb_latitude", Quantity::Latitude),
        ("Ground Station GPS.gs_lon", Quantity::Longitude),
        ("Orientation.accel_z", Quantity::Acceleration),
        ("Orientation.angle_vertical", Quantity::Angle),
        ("Orientation.rot_vel_y", Quantity::AngularRate),
        ("Position Data.gps_time", Quantity::TimeOfDay),
        ("Position Data.temperature", Quantity::Unrecognized),
        ("Position Data.gps_alt", Quantity::Unrecognized),
        ("Orientation.orientation_rpy[1]", Quantity::Unrecognized),
    ] {
        assert_eq!(
            channel(&log, column).map(|c| c.quantity),
            Some(quantity),
            "{column}"
        );
    }
    // The pad's pressure in pascals, whole as the board measured it.
    let pad = channel(&log, "Alt Info & Cfg.barometer_pressure").map_or(f64::NAN, |c| c.values[0]);
    assert!((pad - 98_358.0).abs() < 0.01, "{pad}");
    // The track: 40 samples, 0 on the pad and 178 m at the top; the corrupt packets' 1.7 × 10³⁸
    // is nowhere in it.
    assert_eq!(log.time_s.len(), 40);
    assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
    assert_eq!(log.altitude_m[0], 0.0);
    let top = log.altitude_m.iter().copied().fold(0.0, f64::max);
    assert!((top - 178.1).abs() < 0.1, "{top}");
    // The `nan` in a `v_speed` is a gap in the track's speed, its altitude kept.
    let speeds = log.vertical_speed_m_s.as_ref().unwrap();
    assert_eq!(speeds.iter().filter(|v| v.is_nan()).count(), 1);
    note(
        &log,
        "2 packets, the first on line 64, are left out: the ground station marks their `crc` \
         other than `'Good'`",
    );
    note(
        &log,
        "2 lines are the ground station's own messages, not packets, such as \"Switching radio to \
         band 2\" on line 2: skipped",
    );
    note(
        &log,
        "1 values, the first on line 106, are `None`, `nan`, `inf`",
    );
    assert!(log.notes.iter().all(|n| !n.contains("assum")));
    let channels: Vec<serde_json::Value> = log.channels.iter().map(summary).collect();
    insta::assert_json_snapshot!(serde_json::json!({
        "format": log.format,
        "track": {
            "count": log.time_s.len(),
            "first": [log.time_s[0], log.altitude_m[0], speeds[0]],
            "last": [log.time_s.last(), log.altitude_m.last(), speeds.last()],
        },
        "notes": log.notes,
        "channels": channels,
    }));
}

/// The short record: the clock read through the board's restart, the altitude read, and the
/// rest kept as written for want of a check (an accelerometer reading 0, no GPS fix, positions
/// once a second).
#[test]
fn the_short_record_keeps_what_it_cant_check() {
    let log = import::read(KEPT).unwrap();
    assert_eq!(log.format, LogFormat::AerospaceNuGroundStationText);
    note(
        &log,
        "the board's clock runs in 2 stretches, each a steady gap from the ground station's clock",
    );
    note(
        &log,
        "`Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` kept as written, \
         the pad's gravity not bearing out the club's meters per second squared: ",
    );
    note(
        &log,
        "`Position Data.v_speed` kept as written, the altitude not bearing out the club's meters \
         per second: ",
    );
    note(
        &log,
        "`Position Data.fcb_latitude` and `Position Data.fcb_longitude` kept as written, holding \
         no fix: ",
    );
    note(
        &log,
        "2 fields of the `Line Cutter Data` packets, such as `lc_state`, aren't ones this reader \
         knows",
    );
    assert_eq!(log.time_s.len(), 26);
    assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
    assert!(log.vertical_speed_m_s.is_none());
    insta::assert_json_snapshot!(log);
}

/// A value its conversion to SI makes infinite refuses the file by its line: a coast roll rate of
/// 1.7 × 10³⁰⁸ rad/s is past the largest `f64` in °/s.
#[test]
fn a_value_too_large_for_si_refuses_the_file() {
    let text = std::str::from_utf8(FLIGHT).unwrap();
    let line = text
        .lines()
        .position(|line| line.contains("'rot_vel_z': 0.8"))
        .unwrap();
    let changed = text.replacen("'rot_vel_z': 0.8", "'rot_vel_z': 1.7e308", 1);
    assert_eq!(
        import::read(changed.as_bytes()).unwrap_err(),
        LogError::Syntax {
            format: "AerospaceNU ground station text",
            line: line + 1,
            message: "the column Orientation.rot_vel_z holds a value too large for its unit's \
                      conversion to SI"
                .to_owned(),
        }
    );
}
