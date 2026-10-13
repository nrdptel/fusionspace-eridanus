//! AerospaceNU's flight computer board's CSV, read through the importer from invented files and
//! pinned by snapshots (M7.1c4). The files hold one made-up flight in three of the board's
//! layouts: 10 rows on a pad 250 m up, a 0.4 s boost, a coast to 195 m and a descent at 20 m/s
//! with the board swinging about its x axis at 1 rad/s; the pressure is the standard
//! atmosphere's for the altitude, rounded to whole pascals and written in atmospheres as the
//! club's tools write it. `aerospacenu-fcb-board.csv` is the 87-column layout with two working
//! IMUs and a GPS in degrees and minutes; `aerospacenu-fcb-seconds.csv` the oldest, 84-column
//! one, its clock in `timestamp_s` in seconds, its second IMU and first barometer reading 0;
//! `aerospacenu-fcb-fcbv2.csv` the 73-column FCBV2 layout, its scaled accelerometer 1.5 times
//! off, its GPS in degrees with companions converted as if it were degrees and minutes. Each unit
//! decision is pinned both ways by editing a sample's column in the test.

use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const G: f64 = 9.806_65;
const BOARD: &str = include_str!("samples/aerospacenu-fcb-board.csv");
const SECONDS: &str = include_str!("samples/aerospacenu-fcb-seconds.csv");
const FCBV2: &str = include_str!("samples/aerospacenu-fcb-fcbv2.csv");

// The helpers that unwrap are marked `#[cfg(test)]`, so that clippy.toml's leave to unwrap and
// panic in tests covers them as it covers the tests.
#[cfg(test)]
fn read(text: &str) -> FlightLog {
    import::read(text.as_bytes()).unwrap()
}

#[cfg(test)]
fn refusal(text: &str) -> LogError {
    import::read(text.as_bytes()).unwrap_err()
}

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

#[cfg(test)]
fn quantity(log: &FlightLog, column: &str) -> Quantity {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no channel {column}"))
        .quantity
}

fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a [f64] {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .map_or(&[], |c| c.values.as_slice())
}

/// The text with each cell of the named columns rewritten from its row's index and its text.
#[cfg(test)]
fn edit(text: &str, columns: &[&str], cell: impl Fn(usize, &str) -> String) -> String {
    let mut lines = text.lines();
    let header = lines.next().unwrap();
    let at: Vec<usize> = columns
        .iter()
        .map(|name| header.split(',').position(|n| n == *name).unwrap())
        .collect();
    let mut out = vec![header.to_owned()];
    for (row, line) in lines.enumerate() {
        let mut cells: Vec<String> = line.split(',').map(str::to_owned).collect();
        for &i in &at {
            cells[i] = cell(row, &cells[i]);
        }
        out.push(cells.join(","));
    }
    out.join("\n") + "\n"
}

/// Each cell times a factor, written as Rust writes the nearest `f64`.
#[cfg(test)]
fn scaled(factor: f64) -> impl Fn(usize, &str) -> String {
    move |_, cell| format!("{:?}", cell.parse::<f64>().unwrap() * factor)
}

const PRESSURES: [&str; 3] = ["baro1_pres", "baro2_pres", "baro_pres_avg"];

fn triple(stem: &str, tail: &str) -> [String; 3] {
    ["x", "y", "z"].map(|axis| format!("{stem}_{axis}{tail}"))
}

/// The 87-column board: the pressures in atmospheres by size and step, the clock in milliseconds
/// by spacing and climb, both IMUs, their average and the estimate in m/s², the gyroscopes in
/// rad/s, `pos_z`, `vel_z` and `gps_alt` in meters, the position in degrees and minutes; the
/// track is `pos_z`.
#[test]
fn the_boards_csv_reads_each_decided_column_in_si() {
    let log = read(BOARD);
    assert_eq!(log.format, LogFormat::AerospaceNuFcbCsv);
    assert_eq!(log.logger, "AerospaceNU flight computer board");
    assert_eq!(log.channels.len(), 86, "every column but the clock");
    assert_eq!(log.time_s.len(), 73);
    assert_eq!(log.time_s[0], 600.0);
    assert_eq!(log.altitude_m[0], 0.0);
    let top = log.altitude_m.iter().copied().fold(0.0, f64::max);
    assert!((top - 195.48).abs() < 0.01, "{top}");
    assert!(log.vertical_speed_m_s.is_some());
    assert!(log.events.is_empty());
    assert!(log.temperature_k.is_none() && log.battery_v.is_none());
    for (column, expected) in [
        ("", Quantity::Number),
        ("packetType", Quantity::Number),
        ("state", Quantity::Number),
        ("baro1_pres", Quantity::Pressure),
        ("baro_pres_avg", Quantity::Pressure),
        ("imu1_accel_z", Quantity::Unrecognized),
        ("imu1_accel_z_real", Quantity::Acceleration),
        ("imu2_accel_z_real", Quantity::Acceleration),
        ("imu_accel_z_avg", Quantity::Acceleration),
        ("acc_x", Quantity::Acceleration),
        ("high_g_accel_z_real", Quantity::Unrecognized),
        ("imu1_gyro_x_real", Quantity::AngularRate),
        ("imu_gyro_x_avg", Quantity::AngularRate),
        ("imu1_mag_x_real", Quantity::Unrecognized),
        ("pos_z", Quantity::AltitudeAboveGround),
        ("vel_z", Quantity::Speed),
        ("gps_alt", Quantity::GnssAltitude),
        ("gps_lat", Quantity::Latitude),
        ("gps_long_mod", Quantity::Longitude),
        ("baro1_temp", Quantity::Unrecognized),
        ("battery_voltage", Quantity::Unrecognized),
        ("heading", Quantity::Unrecognized),
    ] {
        assert_eq!(quantity(&log, column), expected, "{column}");
    }
    // The pad's pressure in pascals, a whole number; 1 g in m/s²; 1 rad/s in °/s; the pad's
    // position, 12.5° N 34.75° W, in degrees.
    assert_eq!(
        channel(&log, "baro1_pres")[0].round(),
        channel(&log, "baro1_pres")[0]
    );
    assert!((channel(&log, "imu1_accel_z_real")[0] - G).abs() < 0.01);
    let turning = channel(&log, "imu1_gyro_x_real")
        .iter()
        .copied()
        .fold(0.0, f64::max);
    assert!((turning - 57.3).abs() < 0.02, "{turning}");
    assert!((channel(&log, "gps_lat")[0] - 12.5).abs() < 1e-9);
    assert!((channel(&log, "gps_long")[0] + 34.75).abs() < 1e-9);
    assert!((channel(&log, "gps_lat_mod")[0] - 12.5).abs() < 1e-9);
    for start in [
        "`baro1_pres` read in atmospheres, from its size and its step: ",
        "`baro_pres_avg` read in atmospheres, from its size and its step: ",
        "`timestamp_ms` read in milliseconds, from its spacing and the climb: ",
        "`imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` read in meters per \
         second squared, from the pad's gravity and the sensor's scale: ",
        "`imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` read in meters per second \
         squared, from the pad's gravity and the IMUs: ",
        "`acc_x`, `acc_y` and `acc_z` read in meters per second squared, from the pad's gravity \
         and the climb: ",
        "`imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` read in radians per \
         second, from the board's turning and the sensor's scale: ",
        "`imu_gyro_x_avg`, `imu_gyro_y_avg` and `imu_gyro_z_avg` read in radians per second, \
         from the board's turning and the IMUs: ",
        "`pos_z` read in meters, from the pressure's climb and the accelerometer's: ",
        "`gps_alt` read in meters, from the pressure's climb and the accelerometer's: ",
        "`vel_z` read in meters per second, from `pos_z` and the accelerometer: ",
        "`gps_lat` and `gps_long` read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their \
         ranges and their minutes: ",
        "`gps_lat_mod` and `gps_long_mod` read in degrees, from `gps_lat` and `gps_long`: ",
        "`high_g_accel_x_real`, `high_g_accel_y_real` and `high_g_accel_z_real` kept as written: ",
        "`baro1_temp` kept as written: ",
    ] {
        note(&log, start);
    }
    assert!(log.notes.iter().all(|n| !n.contains("assum")));
    insta::assert_json_snapshot!(condensed(&log));
}

/// The oldest layout: `timestamp_s` in seconds, `timestamp_ms` all 0; a barometer and an IMU that
/// read 0 kept as written, and the average that takes the dead IMU in kept with them.
#[test]
fn the_oldest_layout_reads_its_clock_in_seconds_and_keeps_dead_sensors() {
    let log = read(SECONDS);
    assert_eq!(log.format, LogFormat::AerospaceNuFcbCsv);
    assert_eq!(log.time_s[0], 600.0);
    assert_eq!(log.time_s.len(), 73);
    note(
        &log,
        "`timestamp_s` read in seconds, from its spacing and the climb: ",
    );
    note(
        &log,
        "`timestamp_ms` kept as written: every value is 0; the other clock column",
    );
    note(
        &log,
        "`baro1_pres` kept as written: every value is 0, a barometer that isn't working",
    );
    note(
        &log,
        "`baro_pres_avg` read in atmospheres, from its size and its step: ",
    );
    note(
        &log,
        "`imu2_accel_x_real`, `imu2_accel_y_real` and `imu2_accel_z_real` kept as written: every \
         value is 0, a sensor that isn't working",
    );
    assert!(
        note(
            &log,
            "`imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` kept as written: "
        )
        .contains("aren't all read in one unit")
    );
    note(
        &log,
        "`imu_gyro_x_avg`, `imu_gyro_y_avg` and `imu_gyro_z_avg` kept as written: ",
    );
    // The GPS altitude holds one fix throughout: no climb to check.
    assert!(note(&log, "`gps_alt` kept as written: ").contains("none, too little to tell"));
    assert_eq!(quantity(&log, "imu_accel_z_avg"), Quantity::Unrecognized);
    assert_eq!(quantity(&log, "pos_z"), Quantity::AltitudeAboveGround);
    insta::assert_json_snapshot!(condensed(&log));
}

/// The FCBV2: its scaled accelerometer reads 1.5 g at rest, so it and its average are kept as
/// written while the estimate `acc_*` is read; `baroalt_agl` read in meters; the GPS read in
/// degrees by its ranges, and its companions, which hold it converted as if it were degrees and
/// minutes, kept as written.
#[test]
fn the_fcbv2_keeps_a_scaled_accelerometer_that_reads_one_and_a_half_g() {
    let log = read(FCBV2);
    assert_eq!(log.channels.len(), 72);
    assert!(
        note(
            &log,
            "`imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` kept as written: "
        )
        .contains("at rest they read 14.702, 1 g within 10% in neither m/s² nor g")
    );
    note(
        &log,
        "`imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` kept as written: ",
    );
    note(
        &log,
        "`acc_x`, `acc_y` and `acc_z` read in meters per second squared, ",
    );
    note(
        &log,
        "`baroalt_agl` read in meters, from the pressure's climb and the accelerometer's: ",
    );
    note(
        &log,
        "`imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` read in radians ",
    );
    note(
        &log,
        "`gps_lat` and `gps_long` read in degrees, from the latitudes' range and the longitudes': ",
    );
    assert!(
        note(&log, "`gps_lat_mod` and `gps_long_mod` kept as written: ")
            .contains("converted as if they were degrees and minutes, which they aren't")
    );
    assert_eq!(quantity(&log, "dynamixel_degrees"), Quantity::Angle);
    assert_eq!(quantity(&log, "dynamixel_length"), Quantity::Unrecognized);
    assert_eq!(quantity(&log, "gps_lat"), Quantity::Latitude);
    assert_eq!(quantity(&log, "gps_lat_mod"), Quantity::Unrecognized);
    assert_eq!(channel(&log, "gps_long")[0], -34.75);
    insta::assert_json_snapshot!(condensed(&log));
}

/// The importer tells the layout from the others by its header alone, a byte-order mark or not.
#[test]
fn the_importer_recognizes_each_sample() {
    for text in [BOARD, SECONDS, FCBV2, &format!("\u{feff}{BOARD}")] {
        assert_eq!(
            import::detect(text.as_bytes()),
            Some(LogFormat::AerospaceNuFcbCsv)
        );
    }
    assert_eq!(read(&format!("\u{feff}{BOARD}")).time_s.len(), 73);
}

/// A pressure in bars, a whole number of pascals over 100,000, has atmospheres' size but not
/// their step: kept as written. So is one in pascals, on the steps but far from 1, and one
/// written too short to test its step.
#[test]
fn a_pressure_is_read_in_atmospheres_only_on_both_its_size_and_its_step() {
    let bars = read(&edit(BOARD, &["baro1_pres"], scaled(101_325.0 / 100_000.0)));
    assert!(
        note(
            &bars,
            "`baro1_pres` kept as written: its size and its step don't both say "
        )
        .contains("its values lie on bars' steps of 1/100,000, not on atmospheres'")
    );
    assert_eq!(quantity(&bars, "baro1_pres"), Quantity::Unrecognized);
    note(&bars, "`baro2_pres` read in atmospheres, ");

    let pascals = read(&edit(BOARD, &["baro1_pres"], scaled(101_325.0)));
    assert!(
        note(&pascals, "`baro1_pres` kept as written: ")
            .contains("outside the 0.5 to 1.1 of atmospheres or bars")
    );

    let short = read(&edit(BOARD, &["baro1_pres"], |_, cell| {
        format!("{:.5}", cell.parse::<f64>().unwrap())
    }));
    assert!(
        note(&short, "`baro1_pres` kept as written: ")
            .contains("of its cells written precisely enough to test (0)")
    );
}

/// The sample's barometers read 3 Pa apart, so their average lies on half steps: still a whole
/// number of half pascals over 101,325, as no value in bars is.
#[test]
fn an_average_of_two_barometers_lies_on_half_steps() {
    let log = read(BOARD);
    assert!(
        note(&log, "`baro_pres_avg` read in atmospheres, ")
            .contains("a whole or half number of pascals over 101,325")
    );
    assert!(
        note(&log, "`baro1_pres` read in atmospheres, ")
            .contains("a whole number of pascals over 101,325 within")
    );
}

/// Without a pressure in a unit the clock has no climb to check: refused.
#[test]
fn a_clock_without_a_pressure_to_check_is_refused() {
    let text = edit(BOARD, &PRESSURES, scaled(1.01325));
    let LogError::Unit { line, message, .. } = refusal(&text) else {
        panic!("not a unit refusal")
    };
    assert_eq!(line, 1);
    assert!(
        message.starts_with(
            "the clock `timestamp_ms` is in no unit both its spacing and the climb bear out: "
        ),
        "{message}"
    );
    assert!(
        message.contains("read in milliseconds, none, too little to tell"),
        "{message}"
    );
}

/// A clock whose spacing says milliseconds but whose climb is 16 times the pressure's: refused,
/// as the spacing alone can't decide.
#[test]
fn a_clock_whose_climb_disagrees_with_its_spacing_is_refused() {
    let text = edit(BOARD, &["timestamp_ms"], |_, cell| {
        format!("{}", cell.parse::<f64>().unwrap() * 0.25)
    });
    let LogError::Unit { message, .. } = refusal(&text) else {
        panic!("not a unit refusal")
    };
    assert!(
        message.starts_with(
            "the clock `timestamp_ms` is in no unit both its spacing and the climb bear out: "
        ),
        "{message}"
    );
    assert!(
        message.contains("its rows' median step is 6.25 as written"),
        "{message}"
    );
}

/// A clock whose climb says milliseconds but whose rows' median step is 1 ms, under any the board
/// logs: refused, as the climb alone can't decide.
#[test]
fn a_clock_whose_spacing_disagrees_with_its_climb_is_refused() {
    let mut lines: Vec<String> = BOARD.lines().map(str::to_owned).collect();
    let pad = lines[1].clone();
    let extra: Vec<String> = (0..80)
        .map(|k| pad.replacen(",600000,", &format!(",{},", 599_920 + k), 1))
        .collect();
    lines.splice(1..1, extra);
    let LogError::Unit { message, .. } = refusal(&(lines.join("\n") + "\n")) else {
        panic!("not a unit refusal")
    };
    assert!(
        message.contains("its rows' median step is 1 as written"),
        "{message}"
    );
    assert!(message.contains("read in milliseconds, 1.05"), "{message}");
}

/// The clock is chosen by which column holds times: none, or both, is refused.
#[test]
fn the_clock_column_is_the_one_that_holds_times() {
    let none = edit(SECONDS, &["timestamp_s"], |_, _| "0".to_owned());
    let LogError::Syntax { line, message, .. } = refusal(&none) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(line, 1);
    assert_eq!(
        message,
        "no clock column, `timestamp_ms` or `timestamp_s`, holds times"
    );

    let both = edit(SECONDS, &["timestamp_ms"], |row, _| {
        format!("{}", 1000 + row)
    });
    let LogError::Syntax { message, .. } = refusal(&both) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(
        message,
        "`timestamp_s` and `timestamp_ms` both hold times: the reader can't tell which is the \
         board's clock"
    );
}

/// The estimate in standard gravities reads 1 at rest and climbs 9.8 times short in m/s²: read
/// in g. A value that overflows once converted is refused, naming its line.
#[test]
fn an_estimate_in_g_is_read_in_g_and_an_overflow_is_refused_by_line() {
    let acc = ["acc_x", "acc_y", "acc_z"];
    let in_g = edit(BOARD, &acc, scaled(1.0 / G));
    let log = read(&in_g);
    note(
        &log,
        "`acc_x`, `acc_y` and `acc_z` read in standard gravities, × 9.806 65 m/s², from the pad's \
         gravity and the climb: ",
    );
    // Back in m/s², as the board wrote them.
    let written = channel(&read(BOARD), "acc_x")[0];
    assert!((channel(&log, "acc_x")[0] - written).abs() < 1e-9 * written);
    assert!((written - G).abs() < 0.01, "{written}");

    // Row 60 is in the descent, past the climb the checks integrate; its line is 62.
    let overflow = edit(&in_g, &["acc_x"], |row, cell| {
        if row == 60 {
            "1.7e308".to_owned()
        } else {
            cell.to_owned()
        }
    });
    let LogError::Syntax { line, message, .. } = refusal(&overflow) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(line, 62);
    assert_eq!(
        message,
        "the column acc_x holds a value too large for its unit's conversion to SI"
    );
    // In m/s² the same cell is no conversion's: kept.
    let kept = edit(BOARD, &["acc_x"], |row, cell| {
        if row == 60 {
            "1.7e308".to_owned()
        } else {
            cell.to_owned()
        }
    });
    assert_eq!(channel(&read(&kept), "acc_x")[60], 1.7e308);
}

/// A pressure of 1.7 × 10³⁰⁸ atmospheres is too coarse for the step check to test, so the column
/// is still read in atmospheres; in pascals the cell overflows, and the file is refused by its
/// line.
#[test]
fn a_pressure_that_overflows_in_pascals_is_refused_by_line() {
    let text = edit(BOARD, &["baro1_pres"], |row, cell| {
        if row == 60 {
            "1.7e308".to_owned()
        } else {
            cell.to_owned()
        }
    });
    let LogError::Syntax { line, message, .. } = refusal(&text) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(line, 62);
    assert_eq!(
        message,
        "the column baro1_pres holds a value too large for its unit's conversion to SI"
    );
}

/// The estimate at twice its size reads 2 g at rest: neither unit, kept.
#[test]
fn an_estimate_off_both_units_is_kept_as_written() {
    let log = read(&edit(BOARD, &["acc_x", "acc_y", "acc_z"], scaled(2.0)));
    assert!(
        note(
            &log,
            "`acc_x`, `acc_y` and `acc_z` kept as written: the pad's gravity and the climb "
        )
        .contains("at rest they read 19.6")
    );
}

/// An IMU's scaled columns in g read 1 at rest and are the counts times 0.732 mg: read in g. At
/// 1.5 times a listed count they are kept, as the FCBV2's are.
#[test]
fn a_scaled_accelerometer_is_read_on_its_gravity_and_its_count() {
    let columns = triple("imu1_accel", "_real");
    let columns: Vec<&str> = columns.iter().map(String::as_str).collect();
    let log = read(&edit(BOARD, &columns, scaled(1.0 / G)));
    note(
        &log,
        "`imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` read in standard \
         gravities, × 9.806 65 m/s², from the pad's gravity and the sensor's scale: ",
    );
    assert!((channel(&log, "imu1_accel_z_real")[0] - G).abs() < 0.01);
    // The average still holds m/s², while the IMUs are in two units: kept.
    note(
        &log,
        "`imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` kept as written: ",
    );

    // One cell off its count's factor: no one factor, kept.
    let off = read(&edit(BOARD, &["imu1_accel_z_real"], |row, cell| {
        let value: f64 = cell.parse().unwrap();
        format!("{:?}", if row == 40 { value * 1.01 } else { value })
    }));
    assert!(
        note(
            &off,
            "`imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` kept as "
        )
        .contains("they aren't their raw counts times one factor")
    );
}

/// The gyroscope in °/s turns the board 57 times too far read in rad/s, and its count is 17.5
/// m°/s: read in °/s. At 1.5 times its count it still turns the board in rad/s, but its count is
/// none of the LSM9DS1's: kept.
#[test]
fn a_gyroscope_is_read_on_its_turning_and_its_count() {
    let columns = triple("imu1_gyro", "_real");
    let columns: Vec<&str> = columns.iter().map(String::as_str).collect();
    let degrees = read(&edit(BOARD, &columns, scaled(180.0 / std::f64::consts::PI)));
    note(
        &degrees,
        "`imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` read in degrees per \
         second, from the board's turning and the sensor's scale: ",
    );
    let turning = channel(&degrees, "imu1_gyro_x_real")
        .iter()
        .copied()
        .fold(0.0, f64::max);
    assert!((turning - 57.3).abs() < 0.02, "{turning}");

    let off = read(&edit(BOARD, &columns, scaled(1.5)));
    assert!(
        note(
            &off,
            "`imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` kept as "
        )
        .contains("none of the LSM9DS1's counts in either unit")
    );
}

/// `pos_z` in feet climbs 3.28 times the pressure's: read in feet, its track in meters; `vel_z`
/// in ft/s read in ft/s. At twice its climb `pos_z` is in neither, kept, and the track is empty.
#[test]
fn the_altitude_and_the_speed_are_read_in_meters_or_feet() {
    let feet = read(&edit(BOARD, &["pos_z", "vel_z"], scaled(1.0 / 0.3048)));
    note(
        &feet,
        "`pos_z` read in feet, from the pressure's climb and the accelerometer's: ",
    );
    note(
        &feet,
        "`vel_z` read in feet per second, from `pos_z` and the accelerometer: ",
    );
    let top = feet.altitude_m.iter().copied().fold(0.0, f64::max);
    assert!((top - 195.48).abs() < 0.01, "{top}");

    let doubled = read(&edit(BOARD, &["pos_z"], scaled(2.0)));
    note(
        &doubled,
        "`pos_z` kept as written: the pressure's climb and the accelerometer's ",
    );
    note(&doubled, "the record's primary track is empty: ");
    assert!(doubled.time_s.is_empty() && doubled.vertical_speed_m_s.is_none());
    // `vel_z` has no `pos_z` in meters to check against.
    assert!(note(&doubled, "`vel_z` kept as written: ").contains("in meters is none"));
}

/// A column's cells, as numbers.
#[cfg(test)]
fn column(text: &str, name: &str) -> Vec<f64> {
    let mut lines = text.lines();
    let at = lines
        .next()
        .unwrap()
        .split(',')
        .position(|n| n == name)
        .unwrap();
    lines
        .map(|line| line.split(',').nth(at).unwrap().parse::<f64>().unwrap())
        .collect()
}

/// The board's `gps_alt` with a late fix: 0, the receiver's mark for no fix, until `pos_z` passes
/// 30 m, then the site's elevation plus the climb. A line with a free offset through the fixes
/// reads it in the unit it is written in at a site 400 or 1,500 m up, the zeros as gaps; through
/// a pad of 0 its climb would read the elevation too, and at 400 m up meters would read as feet.
#[test]
fn a_late_gps_fix_at_a_high_site_is_read_in_its_unit() {
    let climb = column(BOARD, "pos_z");
    let late = |elevation_m: f64, per_m: f64| {
        let climb = climb.clone();
        edit(BOARD, &["gps_alt"], move |row, _| {
            if climb[row] < 30.0 {
                "0.0".to_owned()
            } else {
                format!("{:?}", (elevation_m + climb[row]) * per_m)
            }
        })
    };
    let blank = climb.iter().filter(|z| **z < 30.0).count();
    for elevation_m in [400.0, 1_500.0] {
        for (unit, per_m) in [("meters", 1.0), ("feet", 1.0 / 0.3048)] {
            let log = read(&late(elevation_m, per_m));
            note(
                &log,
                &format!(
                    "`gps_alt` read in {unit}, from the pressure's climb and the accelerometer's: "
                ),
            );
            note(
                &log,
                &format!(
                    "{blank} rows of `gps_alt` hold 0, the receiver's mark for no fix: each is a \
                     gap"
                ),
            );
            assert_eq!(quantity(&log, "gps_alt"), Quantity::GnssAltitude);
            let gps = channel(&log, "gps_alt");
            assert_eq!(gps.iter().filter(|v| v.is_nan()).count(), blank);
            let top = gps
                .iter()
                .copied()
                .filter(|v| v.is_finite())
                .fold(0.0, f64::max);
            assert!(
                (top - (elevation_m + 195.48)).abs() < 0.01,
                "{elevation_m} {unit}: {top}"
            );
        }
    }
}

/// A value held before the first fix, 100 m where the fixes' line puts the pad at 262 m, isn't
/// the pad's altitude: the column is kept as written, though both checks read meters.
#[test]
fn a_gps_altitude_held_from_before_its_first_fix_is_kept() {
    let climb = column(BOARD, "pos_z");
    let held = edit(BOARD, &["gps_alt"], |row, cell| {
        if climb[row] < 30.0 {
            "100.0".to_owned()
        } else {
            cell.to_owned()
        }
    });
    let log = read(&held);
    let kept = note(&log, "`gps_alt` kept as written: ");
    assert!(
        kept.contains("its pad reading isn't where its climb starts") && kept.contains("reads 100"),
        "{kept}"
    );
    assert_eq!(quantity(&log, "gps_alt"), Quantity::Unrecognized);
}

/// Companions that copy the values check nothing: the position is read in degrees and minutes on
/// its own ranges and minutes, and the copies kept as written.
#[test]
fn companions_that_copy_the_values_are_kept() {
    let text = edit(BOARD, &["gps_lat_mod"], |_, _| "1250.0".to_owned());
    let text = edit(&text, &["gps_long_mod"], |_, _| "-3445.0".to_owned());
    let text = edit(&text, &["gps_lat"], |_, _| "1250.0".to_owned());
    let log = read(&text);
    note(
        &log,
        "`gps_lat` and `gps_long` read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their \
         ranges and their minutes: ",
    );
    assert!(
        note(&log, "`gps_lat_mod` and `gps_long_mod` kept as written: ")
            .contains("they aren't their companions read as decided")
    );
    assert!((channel(&log, "gps_lat")[0] - (12.0 + 50.0 / 60.0)).abs() < 1e-9);
    assert_eq!(quantity(&log, "gps_lat_mod"), Quantity::Unrecognized);
}

/// One corrupt fix in a file of degrees, a latitude of 3230.0 alone or with a longitude of
/// -10645.0, doesn't make the file degrees and minutes: most of its latitudes and longitudes lie
/// within ±90 and ±180, so the ranges bear no unit out and the position is kept.
#[test]
fn one_glitch_beyond_90_doesnt_make_degrees_degrees_and_minutes() {
    let at_row_40 = |value: &'static str| {
        move |row: usize, cell: &str| {
            if row == 40 {
                value.to_owned()
            } else {
                cell.to_owned()
            }
        }
    };
    let latitude = edit(FCBV2, &["gps_lat"], at_row_40("3230.0"));
    let both = edit(&latitude, &["gps_long"], at_row_40("-10645.0"));
    for (glitch, longitudes) in [(&latitude, 0), (&both, 1)] {
        let log = read(glitch);
        let kept = note(&log, "`gps_lat` and `gps_long` kept as written: ");
        assert!(
            kept.contains(&format!(
                "of its 73 fixes, 1 latitudes pass ±90 and {longitudes} longitudes ±180"
            )),
            "{kept}"
        );
        note(&log, "`gps_lat_mod` and `gps_long_mod` kept as written: ");
        assert_eq!(quantity(&log, "gps_lat"), Quantity::Unrecognized);
        assert_eq!(quantity(&log, "gps_long"), Quantity::Unrecognized);
    }

    // A file of degrees and minutes with one value whose minutes pass 60, or that converts past
    // 90°, is kept too.
    for (cell, says) in [
        (
            "1275.0",
            "some value's last two digits before the point are 60 or more",
        ),
        (
            "9230.0",
            "converted from degrees and minutes, a latitude passes ±90",
        ),
    ] {
        let text = edit(BOARD, &["gps_lat"], |row, old| {
            if row == 40 {
                cell.to_owned()
            } else {
                old.to_owned()
            }
        });
        let log = read(&text);
        let kept = note(&log, "`gps_lat` and `gps_long` kept as written: ");
        assert!(kept.contains(says), "{kept}");
        assert_eq!(quantity(&log, "gps_lat"), Quantity::Unrecognized);
        note(&log, "`gps_lat_mod` and `gps_long_mod` kept as written: ");
    }
}

/// Degrees and minutes within 1° of the equator lie within ±90: the latitudes' range then says
/// degrees and the longitudes' degrees and minutes, so the position is kept.
#[test]
fn a_position_whose_ranges_disagree_is_kept() {
    let text = edit(BOARD, &["gps_lat"], |_, _| "30.0".to_owned());
    let text = edit(&text, &["gps_lat_mod"], |_, _| "0.5".to_owned());
    let log = read(&text);
    assert!(
        note(&log, "`gps_lat` and `gps_long` kept as written: ")
            .contains("the latitudes' range and the longitudes' don't both bear one unit out")
    );
}

/// A position (0, 0) is the receiver's mark for no fix: it takes no part in the checks, and where
/// the position is read, it and its companions are gaps, counted in a note.
#[test]
fn a_position_of_zero_zero_is_a_gap() {
    let early = |row: usize, cell: &str| {
        if row < 5 {
            "0.0".to_owned()
        } else {
            cell.to_owned()
        }
    };
    let text = edit(
        BOARD,
        &["gps_lat", "gps_long", "gps_lat_mod", "gps_long_mod"],
        early,
    );
    let log = read(&text);
    note(
        &log,
        "`gps_lat` and `gps_long` read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their \
         ranges and their minutes: of its 68 fixes, ",
    );
    note(
        &log,
        "`gps_lat_mod` and `gps_long_mod` read in degrees, from `gps_lat` and `gps_long`: ",
    );
    note(
        &log,
        "5 rows of `gps_lat` and `gps_long` hold the position (0, 0), the receiver's mark for no \
         fix: each is a gap",
    );
    for column in ["gps_lat", "gps_long", "gps_lat_mod", "gps_long_mod"] {
        let values = channel(&log, column);
        assert!(values[..5].iter().all(|v| v.is_nan()), "{column}");
        assert!(values[5..].iter().all(|v| v.is_finite()), "{column}");
    }

    // Every position (0, 0): no fix, kept as written.
    let none = edit(
        BOARD,
        &["gps_lat", "gps_long", "gps_lat_mod", "gps_long_mod"],
        |_, _| "0.0".to_owned(),
    );
    let log = read(&none);
    assert!(
        note(&log, "`gps_lat` and `gps_long` kept as written: ").contains("no row holds a fix")
    );
    note(&log, "`gps_lat_mod` and `gps_long_mod` kept as written: ");
}

/// A file whose every latitude and longitude is 0 or more writes no hemisphere: the position and
/// its companions are kept as written, the note saying how the ranges would read them, in degrees
/// and minutes or in degrees; a (0, 0) row stays as written, not a gap.
#[test]
fn a_position_with_no_negative_value_shows_no_hemisphere_and_is_kept() {
    let east = edit(BOARD, &["gps_long"], |_, _| "3445.0".to_owned());
    let east = edit(&east, &["gps_long_mod"], |_, _| "34.75".to_owned());
    let east = edit(&east, &["gps_lat", "gps_long"], |row, cell| {
        if row == 0 { "0.0" } else { cell }.to_owned()
    });
    let degrees = edit(FCBV2, &["gps_long"], |_, _| "34.75".to_owned());
    for (text, would) in [
        (
            &east,
            "read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their ranges",
        ),
        (
            &degrees,
            "read in degrees, from the latitudes' range and the longitudes'",
        ),
    ] {
        let log = read(text);
        let kept = note(
            &log,
            "`gps_lat` and `gps_long` kept as written, with no hemisphere: every value in them is \
             0 or more and the file writes no hemisphere",
        );
        assert!(
            kept.contains(&format!("Unsigned, they would be {would}")),
            "{kept}"
        );
        note(
            &log,
            "`gps_lat_mod` and `gps_long_mod` kept as written, with no hemisphere: ",
        );
        assert!(
            !log.notes
                .iter()
                .any(|n| n.contains("the receiver's mark for no fix: each"))
        );
        for column in ["gps_lat", "gps_long", "gps_lat_mod", "gps_long_mod"] {
            assert_eq!(quantity(&log, column), Quantity::Unrecognized, "{column}");
        }
    }
    let log = read(&east);
    assert_eq!(channel(&log, "gps_lat")[0], 0.0);
    assert_eq!(channel(&log, "gps_long")[1], 3445.0);
}

/// A file that writes negative values shows its hemisphere, in either coordinate: south and
/// east, the latitudes negative and the longitudes positive, is read with each sign, as the
/// samples' west, the longitudes negative, is.
#[test]
fn a_position_with_negative_values_is_read_with_its_signs() {
    let south = edit(BOARD, &["gps_lat"], |_, _| "-1230.0".to_owned());
    let south = edit(&south, &["gps_lat_mod"], |_, _| "-12.5".to_owned());
    let south = edit(&south, &["gps_long"], |_, _| "3445.0".to_owned());
    let south = edit(&south, &["gps_long_mod"], |_, _| "34.75".to_owned());
    for (text, latitude, longitude) in [(south.as_str(), -12.5, 34.75), (BOARD, 12.5, -34.75)] {
        let log = read(text);
        note(
            &log,
            "`gps_lat` and `gps_long` read in NMEA 0183's degrees and minutes, ddmm.mmmm, from \
             their ranges and their minutes: ",
        );
        assert!(!log.notes.iter().any(|n| n.contains("no hemisphere")));
        assert_eq!(quantity(&log, "gps_long"), Quantity::Longitude);
        assert_eq!(quantity(&log, "gps_lat_mod"), Quantity::Latitude);
        assert!((channel(&log, "gps_lat")[0] - latitude).abs() < 1e-9);
        assert!((channel(&log, "gps_long")[0] - longitude).abs() < 1e-9);
        assert!((channel(&log, "gps_long_mod")[0] - longitude).abs() < 1e-9);
    }
}

/// A column the reader doesn't know is kept as written with a note; the header's errors are
/// refused by line.
#[test]
fn unknown_columns_are_kept_and_bad_headers_refused() {
    let mut lines: Vec<String> = BOARD.lines().map(|l| format!("{l},7")).collect();
    lines[0] = lines[0].replace(",7", ",mystery");
    let log = read(&(lines.join("\n") + "\n"));
    assert_eq!(quantity(&log, "mystery"), Quantity::Unrecognized);
    note(
        &log,
        "the column `mystery` isn't one this reader knows; its values are kept as written",
    );

    let twice = BOARD.replacen("pos_x", "pos_y", 1);
    let LogError::Syntax { line, message, .. } = refusal(&twice) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(
        (line, message.as_str()),
        (1, "the header names \"pos_y\" twice")
    );

    let wide = BOARD.replacen("\n1003,", "\n1003,9,", 1);
    let LogError::Syntax { line, message, .. } = refusal(&wide) else {
        panic!("not a syntax refusal")
    };
    assert_eq!(line, 5);
    assert_eq!(
        message,
        "the row has more than 87 cells, where the header names 87"
    );

    let unnamed = BOARD.replacen(",vtg,", ",,", 1);
    let LogError::Syntax { message, .. } = refusal(&unnamed) else {
        panic!("not a syntax refusal")
    };
    assert!(message.starts_with("the header's column ") && message.ends_with(" has no name"));
}

/// Each channel's name, quantity, length and first and last values, with the track and the
/// notes: a whole record's 86 channels of 73 values would make the snapshot hundreds of
/// kilobytes, and the tests above pin the values that the decisions turn.
#[cfg(test)]
fn condensed(log: &FlightLog) -> serde_json::Value {
    let mut value = serde_json::to_value(log).unwrap();
    let channels: Vec<serde_json::Value> = log
        .channels
        .iter()
        .map(|c| {
            serde_json::json!({
                "column": c.column,
                "quantity": c.quantity,
                "samples": c.values.len(),
                "first": c.values.first(),
                "last": c.values.last(),
            })
        })
        .collect();
    value["channels"] = serde_json::Value::Array(channels);
    value
}
