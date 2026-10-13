//! AerospaceNU's flight computer board's CSV: the club's table of a flight, one row per packet
//! the board logged, under a header whose first name is empty.
//!
//! The flight computer board (FCB) is a flight computer of Northeastern University's rocketry
//! club, AerospaceNU, with two STMicroelectronics LSM9DS1 inertial units (each an accelerometer,
//! a gyroscope and a magnetometer), a high-g accelerometer, one or two barometers and a GPS. The
//! club publishes its flights' logs, each board's binary log beside a CSV its tools made from it
//! (AerospaceNU, *rocket-flight-data*, `README.md`, read 2026-10-12). This reader reads that CSV.
//! No published document describes the columns or their units, so each unit here comes from the
//! column's name or from two checks the reader runs on the file's values that agree, as below;
//! where they don't, the column is kept as written. 20 such files are among the project's sample
//! logs, under eight headers of 73 to 87 columns: the oldest open with `timestamp_s, timestamp_ms`,
//! the later ones with `packetType`, then one clock column or both; the 73-column layout, the
//! board's second version (FCBV2), has one IMU and one barometer and adds servo columns. An
//! invented row of the 87-column layout, cut short:
//!
//! ```text
//! ,packetType,timestamp_ms,imu1_accel_x,imu1_accel_y,imu1_accel_z,imu1_gyro_x,…
//! 1000,0,600000,0,0,1366,0,0,0,-310,1220,1405,2,2,1368,…
//! ```
//!
//! **Recognition.** The first line that isn't blank is a header whose first name is empty, the
//! club's row-number column, and which names `packetType`, `timestamp_ms` or `timestamp_s`.
//! Names and cells are split at commas.
//!
//! **Columns.** Each name the reader knows, what it is read as, and where its unit comes from;
//! every column but the clock is kept as a channel on the clock, under the file's name. A column
//! the reader doesn't know is kept as written, [`Quantity::Unrecognized`], with a note (the first
//! 20 such notes kept, the rest counted); a header naming a column twice, with an empty name past
//! the first, or with more than [`MAX_COLUMNS`] names is refused:
//!
//! | Column | Quantity | Unit, and from where |
//! |---|---|---|
//! | the first, unnamed | [`Quantity::Number`] | the club's row number, as written |
//! | `timestamp_ms`, `timestamp_s` | the clock | milliseconds or seconds, from its spacing and the climb (below), whatever its name says; the other, if it holds only 0, is kept as written |
//! | `baro1_pres`, `baro2_pres`, `baro_pres_avg` | [`Quantity::Pressure`] | atmospheres, × 101,325 Pa, from the size and the step (below) |
//! | `imu1_accel_*_real`, `imu2_accel_*_real` | [`Quantity::Acceleration`] | m/s² or standard gravities, from the pad's gravity and the sensor's scale (below) |
//! | `imu_accel_*_avg` | [`Quantity::Acceleration`] | the IMUs' unit, from the pad's gravity and the mean (below) |
//! | `high_g_accel_*_real` | [`Quantity::Acceleration`] | as the IMUs' scaled columns (below); kept as written in every sample file |
//! | `acc_x`, `acc_y`, `acc_z` | [`Quantity::Acceleration`] | m/s² or standard gravities, from the pad's gravity and the climb (below): the board's own estimate, in axes the file doesn't state |
//! | `imu1_gyro_*_real`, `imu2_gyro_*_real` | [`Quantity::AngularRate`] | radians or degrees per second, from the board's turning and the sensor's scale (below) |
//! | `imu_gyro_*_avg` | [`Quantity::AngularRate`] | the IMUs' unit, from the turning and the mean (below) |
//! | `pos_z` | [`Quantity::AltitudeAboveGround`]: the primary track | meters or feet, from the pressure's climb and the accelerometer's (below) |
//! | `baro_height`, `baroalt_agl`, `baro1_alt` | [`Quantity::AltitudeAboveGround`] | as `pos_z` |
//! | `gps_alt` | [`Quantity::GnssAltitude`] | as `pos_z`, at the GPS's fresh fixes (below) |
//! | `vel_z` | [`Quantity::Speed`], up positive: [`FlightLog::vertical_speed_m_s`] | m/s or ft/s, from `pos_z` and the accelerometer (below) |
//! | `gps_lat`, `gps_long` | [`Quantity::Latitude`], [`Quantity::Longitude`] | NMEA 0183's degrees and minutes, from the values and their companions (below) |
//! | `gps_lat_mod`, `gps_long_mod` | [`Quantity::Latitude`], [`Quantity::Longitude`] | degrees, where they are `gps_lat` and `gps_long` converted from degrees and minutes (below) |
//! | `dynamixel_degrees`, `dynamixel_degrees_1` | [`Quantity::Angle`] | degrees, from their names: a servo's angle |
//! | `packetType`, `state`, `pyro_cont`, `pyro_status`, `trigger_status`, `gps_fix_quality`, `gps_sats_tracked`, `gps_hdop`, `q_x` … `q_w` | [`Quantity::Number`] | codes, flags, counts, a dilution of precision and an attitude quaternion's parts, as written: no published source gives the codes' meanings |
//! | `imu1_accel_*`, `imu1_gyro_*`, `imu1_mag_*`, the same of `imu2`, `high_g_accel_*` | [`Quantity::Unrecognized`] | a sensor's raw counts, kept as written: the `_real` columns hold them scaled |
//! | `imu1_mag_*_real`, `imu2_mag_*_real`, `imu_mag_*_avg` | [`Quantity::Unrecognized`] | kept as written: a magnetic field, a quantity HPR Sim doesn't hold |
//! | `baro1_temp`, `baro2_temp`, `baro_temp_avg` | [`Quantity::Unrecognized`] | kept as written: the names state no unit, and degrees Celsius and Fahrenheit both put the values within a barometer's working −40 to 85 °C; no other column holds a temperature to tell them apart |
//! | `battery_voltage`, `battery_voltage_1`, `battery_voltage_2`, `dynamixel_length`, `dynamixel_length_1` | [`Quantity::Unrecognized`] | kept as written: the names state no unit and nothing in the file checks one |
//! | `heading`, `vtg`, `pos_x`, `pos_y`, `vel_x`, `vel_y` | [`Quantity::Unrecognized`] | kept as written: the names state no unit, and in the sample files they hold only 0 but for one file's `vel_x` |
//!
//! Each unit a check decides gets a note saying on what evidence, and each column kept as written
//! a note saying why. Every decision rests on two checks that must agree, neither of which
//! compares the column with another of the same unknown unit, nor can be met by the weather or
//! the flight in the wrong unit; a check without enough data to tell decides nothing. The checks,
//! with what they found in the 20 sample files:
//!
//! - *The pressures' size and step.* A pad lies 0.5 to 1.1 from sea level's pressure in
//!   atmospheres or bars, and far from it in pascals, hectopascals, kilopascals, psi or inches or
//!   millimeters of mercury: the first check is the median of the first 10 rows' values. The
//!   board's barometers read whole pascals, and the club divides them by 101,325; so the second
//!   check is that each cell lies within a millionth of a step of a whole number of steps of
//!   1/101,325, or of 1/202,650 for an average of two barometers, for every cell written to
//!   enough decimals that its printing, at most half a unit in its last place, takes under half
//!   that tolerance (at least 10 cells). In bars, a whole number of pascals over 100,000, the cells
//!   miss the steps by up to half a step. A pressure on both grids or on neither is kept as
//!   written. In the sample files 1,517 to 21,291 cells test per column and every barometer that
//!   reads is in atmospheres; `baro_pres_avg` takes half steps where it averages two. One
//!   barometer reads 0 throughout and is kept as written.
//! - *The clock's spacing and the climb.* The board logs a row every 15 or 16 ms in flight and
//!   about every 100 ms on the pad; the first check is that the rows' median step, in the unit,
//!   lies within 5 to 500 ms. The second integrates the first IMU's raw accelerometer (else the
//!   second's), in its pad reading's 1 g so that its counts' unit divides out, along the pad
//!   reading's direction less gravity, from the launch (the first row of 3 g, the next two at 2
//!   g) to the pressure's apogee in the U.S. Standard Atmosphere, 1976 ([`hpr_atmos::ussa76`]);
//!   the median, over the rows past 30 m, of the pressure's climb over the integrated one must lie
//!   within a factor of 10 of 1. Seconds read as milliseconds, or the reverse, put it 10⁶ off.
//!   The unit both checks bear out is taken; with none, or two, the file is refused
//!   ([`LogError::Unit`]): nothing in it has a time. Either clock column can hold either unit: in
//!   six sample files `timestamp_s` holds milliseconds, in two seconds, and in the other twelve
//!   `timestamp_ms` holds milliseconds; the ratio is 0.57 to 1.15 in milliseconds, and 0.83 and
//!   3.26 in the two in seconds.
//! - *The pad's gravity and the sensor's scale.* A board at rest reads 1 g: the magnitude of the
//!   three axes' pad readings within 10% of 9.806 65 in m/s² or of 1 in standard gravities is the
//!   first check. The second: each scaled column is its raw counts times one factor (within a
//!   millionth), and that factor is, in the unit, one of the LSM9DS1's accelerometer
//!   sensitivities, 0.061, 0.122, 0.244 or 0.732 mg a count (*LSM9DS1*, STMicroelectronics,
//!   DocID025715 Rev 3, 2015, Table 3, `LA_So`). The reader never rescales by a factor it
//!   infers. In the sample files the factor is 0.732 mg in m/s² throughout, and the IMUs read
//!   9.11 to 9.97 at rest, but for the FCBV2's, which reads 14.7 (1.5 g: its counts are another
//!   part's), four whose second IMU reads 0.27 to 0.54, and one second IMU that reads 0; these
//!   are kept as written. The high-g accelerometer's factor, 0.029 42, is none of the LSM9DS1's,
//!   so it is kept as written in every file.
//! - *The IMUs' average.* `imu_accel_*_avg` must read 1 g at rest in the unit of the IMUs' scaled
//!   columns, all of which must be read in it, and be their mean at every row, within a
//!   millionth. Where an IMU is kept as written, so is the average; in the sample files, where
//!   one IMU reads near 0, the average reads half of 1 g.
//! - *The board's estimate `acc_*`.* Its pad reading must be 1 g within 10%, and the pressure's
//!   climb over the one it integrates to, as the clock's check but read in the unit, must lie
//!   within a factor of 1.8 of 1 in that unit and in no other; read in the wrong one, the
//!   specific force is 9.8 times too large or too small against the gravity taken off it. In the
//!   sample files it reads 9.11 to 9.97 at rest and the ratio is 0.74 to 1.10, in m/s².
//! - *The board's turning and the gyroscope's scale.* As [`crate::sillygoose`] makes the turning
//!   check: over stretches of 10 steps at most 0.05 s apart with the raw accelerometer within 15%
//!   of 1 g, the angle its direction turns against the gyroscope's turn across that direction,
//!   |ω × â| dt; the unit, radians or degrees per second, whose turn lies within a quarter to 4
//!   times the accelerometer's, from at least 3 stretches and 0.2 rad. The second check: the
//!   scaled columns are their raw counts times one factor, one of the LSM9DS1's gyroscope
//!   sensitivities, 8.75, 17.5 or 70 m°/s a count (Table 3, `G_So`), in the unit. In the sample
//!   files the factor is 17.5 or 70 m°/s in rad/s, and the turning ratio 0.54 to 2.34 where it is
//!   read; one file's is 6.6 and 8.4, and two have no stretches to check, so those are kept as
//!   written. `imu_gyro_*_avg` must bear the IMUs' unit out by the turning and be their mean.
//! - *The climbs: `pos_z`, `baro_height`, `baroalt_agl`, `baro1_alt`, `gps_alt`.* First, the
//!   column's climb from its pad reading to its highest (its 5-row running median) over the
//!   pressure's, which must be at least 30 m, must lie within 25% of 1 in meters or in feet;
//!   `gps_alt`'s is the least-squares slope, through the pad, of its rise on the pressure's at
//!   the rows up to the apogee where it changes, a fresh fix, from at least 3. Second, its climb
//!   over the one the accelerometer integrates to, as above, with the first of `acc_*`, the IMUs'
//!   scaled columns and their average read in a unit, must lie within a factor of 1.8 of 1:
//!   1.8² = 3.24 is under the 3.28 between meters and feet. The pressure and the accelerometer
//!   are decided by their own checks above, so neither shares the column's unknown unit. In the
//!   sample files `pos_z` is read in meters in every file, its gain 0.90 to 1.21 and its ratio 0.82
//!   to 1.11; `gps_alt` in 12, gains 0.92 to 1.10, and kept as written in 8, where it reads 0,
//!   has too few fresh fixes, or climbs far short of the pressure or the accelerometer.
//! - *`vel_z`.* First, over the steps at most 0.05 s long, the least-squares slope of `pos_z`'s
//!   change, in meters, on `vel_z` times the step must lie within 25% of 1 in m/s or of 0.3048 in
//!   ft/s, from at least 10 m of movement. Second, the median, over the rows past 10 m/s, of it
//!   over the speed the accelerometer integrates to must lie within a factor of 1.8. In the sample
//!   files the slope is 0.92 to 1.00 and the ratio 0.80 to 1.24 in m/s; one file's `vel_z` reads 0
//!   throughout and one's slope is 0.59, kept as written.
//! - *The GPS position.* NMEA 0183's degrees and minutes write a latitude of 42.5° as 4230.0. The
//!   first check: some latitude passes 90 or longitude 180, which no degrees do, and every value's
//!   last two digits before the point, its minutes, are under 60. The second: `gps_lat_mod` and
//!   `gps_long_mod` hold every value converted from degrees and minutes to degrees, within a
//!   millionth. Then the values are read in degrees and minutes and their companions in degrees;
//!   otherwise all four are kept as written. 15 sample files pass both. In three later ones a
//!   longitude's "minutes" pass 60, so the values aren't degrees and minutes, and the companions
//!   hold them converted as if they were: the values read like degrees, but no second check bears
//!   that out. In one the GPS reads 0 throughout, and in one a longitude's "minutes" pass 60 and
//!   the companions copy the values, which checks no unit.
//!
//! **The primary track.** The record's altitude is `pos_z`, the board's filtered altitude above
//! the pad, where it is read in a unit; otherwise the track is empty and a note says so, as no
//! other column is the board's altitude above the pad in a decided unit. The vertical speed is
//! `vel_z` where it is read. The track takes the rows in time order, the first at each time,
//! leaving out a row without a time; a note counts those put in order or left out, and the
//! channels keep every row in the file's order. Where `pos_z` reads more than 100 m off zero at
//! the track's first row, a note says so. No temperature or battery voltage is the record's, as
//! none is read in a unit. **Events:** none. `state` is the board's state code, a
//! [`Quantity::Number`]; its codes' meanings aren't published, so no event is drawn from it.
//!
//! **Cells.** An empty cell, or one that isn't a number, is a gap (`NaN`), counted in a note. A
//! value too large to hold once converted to its unit, such as an acceleration of 1.7 × 10³⁰⁸ in
//! standard gravities or of 1.7 × 10³⁰⁸ atmospheres, is refused, naming its line. A row with
//! fewer cells than the header names has the rest as gaps, counted in a note; a row with more is
//! refused, naming its line, as is a file whose rows hold under a quarter of the cells the header
//! names (past the first 65,536), so that a short file can't make the reader store far more gaps
//! than it writes cells. Lines may end in CR, LF or both, blank lines are skipped, and a
//! byte-order mark is dropped.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use hpr_atmos::ussa76::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, Notes, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "AerospaceNU flight computer CSV";

/// The logger's name: the files don't name the board or its version.
const LOGGER: &str = "AerospaceNU flight computer board";

/// The barometric altitude's step, m ([`crate::log::LogFormat::altitude_resolution_m`]): the
/// track is the board's altitude estimate `pos_z`, written to 15 or more digits, but the
/// barometer it rests on reads whole pascals (its pressure is a whole number of pascals over
/// 101,325, below), and one pascal is `1 / (ρ₀ g₀)` = 0.083 m of altitude near sea level, with the
/// International Standard Atmosphere's ρ₀ = 1.225 kg/m³ (ISO 2533:1975) and g₀ = 9.806 65 m/s².
pub const ALTITUDE_STEP_M: f64 = 1.0 / (1.225 * STANDARD_GRAVITY_MPS2);

/// The most names a header may have: the board's files have 73 to 87.
pub const MAX_COLUMNS: usize = 256;

/// The rows at the file's start that are the pad: the median of each column over them is its
/// reading at rest. Each of the 20 sample files holds at least 32 rows before the launch.
const PAD_ROWS: usize = 10;

/// Pascals in a standard atmosphere and in a bar.
const ATM_PA: f64 = 101_325.0;
const BAR_PA: f64 = 100_000.0;

/// The pressures a pad can read in atmospheres or bars: 0.5 to 1.1, a pad from 5,500 m up to below
/// sea level in a high. Pascals, hectopascals, kilopascals, psi and inches or millimeters of
/// mercury put a pad far from 1.
const PAD_PRESSURE: (f64, f64) = (0.5, 1.1);

/// How far, in steps, a cell may lie from a whole number of a pressure's steps.
const STEP_TOLERANCE: f64 = 1e-6;

/// The fewest cells that must test the step for it to decide.
const LEAST_STEP_CELLS: usize = 10;

/// The board's logging, as a step between rows, s: 2 to 200 rows a second. The board logs a row
/// every 15 or 16 ms in flight and every 100 ms or so on the pad.
const ROW_STEP_S: (f64, f64) = (0.005, 0.5);

/// How many times off the climb check may find the clock's climb and still bear its unit out:
/// milliseconds and seconds put it a million times apart.
const CLOCK_FACTOR: f64 = 10.0;

/// The specific force, in its pad reading, from which a row is the launch: 3 g, held for the next
/// two rows at 2 g.
const LAUNCH_G: f64 = 3.0;
const LAUNCH_HELD_G: f64 = 2.0;

/// The least climb, m, integrated from the accelerometer, at which a row counts in a climb ratio,
/// and the least climb of the pressure for a gain to be computed.
const LEAST_CLIMB_M: f64 = 30.0;

/// The least speed, m/s, integrated from the accelerometer, at which a row counts in a speed ratio.
const LEAST_SPEED_M_S: f64 = 10.0;

/// The fewest rows a ratio's median takes.
const LEAST_ROWS: usize = 5;

/// How many times off a length, a speed or an acceleration may be from the accelerometer's climb
/// and still bear its unit out: 1.8² = 3.24 is under the 3.28 between meters and feet, so at most
/// one of the two is within it.
const LENGTH_FACTOR: f64 = 1.8;

/// How far, as a share, a length's or a speed's gain may lie from 1 against a column of known
/// unit.
const GAIN_MARGIN: f64 = 0.25;

/// How far, as a share, a reading at rest may be from 1 g.
const GRAVITY_MARGIN: f64 = 0.1;

/// The relative difference within which two computations of the same value agree: the files'
/// values are the club's computations in `f64` or `f32`, a `f32` within 1.2 × 10⁻⁷.
const SAME: f64 = 1e-6;

/// LSM9DS1's accelerometer's sensitivities, g a count, at ±2, ±4, ±8 and ±16 g (*LSM9DS1*,
/// STMicroelectronics, DocID025715 Rev 3, 2015, Table 3, `LA_So`).
const ACCEL_SCALES_G: [f64; 4] = [0.061e-3, 0.122e-3, 0.244e-3, 0.732e-3];

/// LSM9DS1's gyroscope's sensitivities, °/s a count, at ±245, ±500 and ±2,000 °/s (Table 3,
/// `G_So`).
const GYRO_SCALES_DPS: [f64; 3] = [8.75e-3, 17.5e-3, 70e-3];

/// The longest step, s, of the turning and speed checks: the board's flight logging is 15 or 16
/// ms.
const FAST_STEP_S: f64 = 0.05;

/// The turning check, as SillyGoose's reader makes it ([`crate::sillygoose`]): stretches of so
/// many steps, the accelerometer within this share of 1 g, at least so many stretches and so much
/// turning, and a unit taken if the gyroscope's turn is within this factor of the
/// accelerometer's.
const TURN_STEPS: usize = 10;
const TURN_GRAVITY_MARGIN: f64 = 0.15;
const TURN_LEAST_STRETCHES: usize = 3;
const TURN_LEAST_RAD: f64 = 0.2;
const TURN_FACTOR: f64 = 4.0;

/// The least movement, m, the speed integrates over the fast steps for the speed's check against
/// `pos_z` to decide.
const SPEED_LEAST_M: f64 = 10.0;

/// How far, m, the track may read from zero at its first sample before a note says the record
/// doesn't start at the board's zero.
const GROUND_OFF_M: f64 = 100.0;

/// A foot, m.
const FOOT_M: f64 = 0.3048;

/// What a column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// The unnamed first column: the row's number in the club's table.
    Index,
    /// `timestamp_ms` or `timestamp_s`.
    Clock,
    /// A sensor's raw counts, such as `imu1_accel_x` or `high_g_accel_z`.
    Raw,
    /// An IMU's accelerometer, scaled: `imu1_accel_x_real`; the IMU and the axis.
    AccelReal(u8, usize),
    /// An IMU's gyroscope, scaled.
    GyroReal(u8, usize),
    /// The IMUs' accelerometers averaged: `imu_accel_x_avg`.
    AccelAverage(usize),
    /// The IMUs' gyroscopes averaged.
    GyroAverage(usize),
    /// A magnetometer, scaled or averaged.
    Magnetic,
    /// The high-g accelerometer, scaled: `high_g_accel_x_real`.
    HighG(usize),
    /// `baro1_pres`, `baro2_pres` or `baro_pres_avg`.
    Pressure,
    /// `baro1_temp`, `baro2_temp` or `baro_temp_avg`.
    BaroTemperature,
    /// `baro_height`, `baroalt_agl` or `baro1_alt`: the board's altitude from its barometer.
    BaroAltitude,
    Latitude,
    Longitude,
    /// `gps_lat_mod`: the club's conversion of `gps_lat`.
    LatitudeMod,
    LongitudeMod,
    GpsAltitude,
    Battery,
    /// `dynamixel_degrees`: a servo's angle, its unit in its name.
    ServoAngle,
    /// A column no check settles: `heading`, `vtg`, `pos_x`, `pos_y`, `vel_x`, `vel_y`,
    /// `dynamixel_length`.
    Unsettled,
    /// `pos_z`: the board's altitude estimate.
    Height,
    /// `vel_z`: the board's vertical speed estimate.
    Climb,
    /// `acc_x`, `acc_y`, `acc_z`: the board's acceleration estimate.
    Estimate(usize),
    /// A code, flag, count or quaternion part, as written.
    Number,
    Unknown,
}

/// The axis a name's part names.
fn axis_of(part: &str) -> Option<usize> {
    ["x", "y", "z"].iter().position(|axis| *axis == part)
}

/// What a column is, by its name.
fn role_of(name: &str) -> Role {
    match name {
        "timestamp_ms" | "timestamp_s" => return Role::Clock,
        "packetType" | "state" | "pyro_cont" | "pyro_status" | "trigger_status"
        | "gps_fix_quality" | "gps_sats_tracked" | "gps_hdop" | "q_x" | "q_y" | "q_z" | "q_w" => {
            return Role::Number;
        }
        "baro1_pres" | "baro2_pres" | "baro_pres_avg" => return Role::Pressure,
        "baro1_temp" | "baro2_temp" | "baro_temp_avg" => return Role::BaroTemperature,
        "baro_height" | "baroalt_agl" | "baro1_alt" => return Role::BaroAltitude,
        "gps_lat" => return Role::Latitude,
        "gps_long" => return Role::Longitude,
        "gps_lat_mod" => return Role::LatitudeMod,
        "gps_long_mod" => return Role::LongitudeMod,
        "gps_alt" => return Role::GpsAltitude,
        "battery_voltage" | "battery_voltage_1" | "battery_voltage_2" => return Role::Battery,
        "dynamixel_degrees" | "dynamixel_degrees_1" => return Role::ServoAngle,
        "dynamixel_length" | "dynamixel_length_1" | "heading" | "vtg" | "pos_x" | "pos_y"
        | "vel_x" | "vel_y" => return Role::Unsettled,
        "pos_z" => return Role::Height,
        "vel_z" => return Role::Climb,
        _ => {}
    }
    let parts: Vec<&str> = name.split('_').collect();
    match parts.as_slice() {
        ["acc", axis] => axis_of(axis).map_or(Role::Unknown, Role::Estimate),
        [imu, "accel" | "gyro" | "mag", axis] if matches!(*imu, "imu1" | "imu2") => {
            axis_of(axis).map_or(Role::Unknown, |_| Role::Raw)
        }
        [imu, sensor, axis, "real"] if matches!(*imu, "imu1" | "imu2") => {
            let number = if *imu == "imu1" { 1 } else { 2 };
            match (*sensor, axis_of(axis)) {
                ("accel", Some(axis)) => Role::AccelReal(number, axis),
                ("gyro", Some(axis)) => Role::GyroReal(number, axis),
                ("mag", Some(_)) => Role::Magnetic,
                _ => Role::Unknown,
            }
        }
        ["imu", sensor, axis, "avg"] => match (*sensor, axis_of(axis)) {
            ("accel", Some(axis)) => Role::AccelAverage(axis),
            ("gyro", Some(axis)) => Role::GyroAverage(axis),
            ("mag", Some(_)) => Role::Magnetic,
            _ => Role::Unknown,
        },
        ["high", "g", "accel", axis] => axis_of(axis).map_or(Role::Unknown, |_| Role::Raw),
        ["high", "g", "accel", axis, "real"] => axis_of(axis).map_or(Role::Unknown, Role::HighG),
        _ => Role::Unknown,
    }
}

/// How a column's values become its quantity's unit.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Conversion {
    /// Times a factor.
    Scale(f64),
    /// NMEA's degrees and minutes, `ddmm.mmmm`, to degrees.
    DegreesMinutes,
}

impl Conversion {
    fn apply(self, value: f64) -> f64 {
        match self {
            Self::Scale(scale) => value * scale,
            Self::DegreesMinutes => degrees_minutes(value),
        }
    }
}

/// NMEA 0183's `ddmm.mmmm`, degrees times 100 plus minutes, in degrees, its sign kept.
fn degrees_minutes(value: f64) -> f64 {
    let magnitude = value.abs();
    let degrees = (magnitude / 100.0).floor();
    (degrees + (magnitude - 100.0 * degrees) / 60.0).copysign(value)
}

/// One data column.
struct Column {
    name: String,
    role: Role,
    /// The values as written, `NaN` for a gap.
    values: Vec<f64>,
    /// For a pressure, each cell's last written decimal place: the digits after its point, less
    /// its exponent; `None` for a gap.
    places: Vec<Option<i32>>,
    /// What the column is read as: [`Quantity::Unrecognized`], kept as written, until a check
    /// decides its unit.
    quantity: Quantity,
    conversion: Conversion,
}

impl Column {
    fn new(name: &str, role: Role) -> Self {
        let quantity = match role {
            Role::Index | Role::Number => Quantity::Number,
            Role::ServoAngle => Quantity::Angle,
            _ => Quantity::Unrecognized,
        };
        Self {
            name: name.to_owned(),
            role,
            values: Vec::new(),
            places: Vec::new(),
            quantity,
            conversion: Conversion::Scale(1.0),
        }
    }

    /// Reads the column as a quantity, its values times a factor.
    fn read_as(&mut self, quantity: Quantity, scale: f64) {
        self.quantity = quantity;
        self.conversion = Conversion::Scale(scale);
    }

    /// The column's name, cut short, in backticks.
    fn quoted(&self) -> String {
        format!("`{}`", short(&self.name))
    }

    /// Whether no value is a number other than 0.
    fn all_zero(&self) -> bool {
        self.values.iter().all(|v| !v.is_finite() || *v == 0.0)
    }

    /// The median of its first [`PAD_ROWS`] numbers: its reading on the pad.
    fn pad(&self) -> Option<f64> {
        median(self.values.iter().take(PAD_ROWS).copied())
    }
}

/// The median of the numbers among some values; `None` if none is.
fn median(values: impl Iterator<Item = f64>) -> Option<f64> {
    let mut finite: Vec<f64> = values.filter(|v| v.is_finite()).collect();
    if finite.is_empty() {
        return None;
    }
    finite.sort_by(f64::total_cmp);
    Some(finite[finite.len() / 2])
}

/// A cell's last written decimal place: the digits after its point, less its exponent.
fn decimal_places(cell: &str) -> i32 {
    let cell = cell.trim();
    let (mantissa, exponent) = match cell.find(['e', 'E']) {
        Some(at) => (&cell[..at], cell[at + 1..].parse::<i32>().unwrap_or(0)),
        None => (cell, 0),
    };
    let after_point = mantissa
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    i32::try_from(after_point)
        .unwrap_or(i32::MAX)
        .saturating_sub(exponent)
}

/// The names of the columns at some places, as a sentence.
fn listed(columns: &[Column], at: &[usize]) -> String {
    let quoted: Vec<String> = at.iter().map(|&i| columns[i].quoted()).collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Whether a ratio lies within a factor of 1, either way; `NaN` doesn't.
fn within_factor(ratio: f64, factor: f64) -> bool {
    ratio >= 1.0 / factor && ratio <= factor
}

/// Whether a gain lies within a share of 1; `NaN` doesn't.
fn within_margin(gain: f64, margin: f64) -> bool {
    (gain - 1.0).abs() <= margin
}

/// Whether the text is the board's CSV: a header row whose first cell is empty and that names
/// `packetType` or a `timestamp_ms` or `timestamp_s` column.
pub fn recognizes(text: &str) -> bool {
    let first = text
        .trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("");
    first.starts_with(',')
        && first
            .split(',')
            .any(|name| matches!(name.trim(), "packetType" | "timestamp_ms" | "timestamp_s"))
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

fn not_this(message: String) -> LogError {
    LogError::NotThisFormat {
        format: FORMAT,
        message,
    }
}

/// The file's rows, read.
struct Table {
    header_line: usize,
    columns: Vec<Column>,
    /// Each row's line in the file.
    lines: Vec<usize>,
    gaps: usize,
    first_gap: Option<usize>,
    short_rows: usize,
    first_short: Option<usize>,
}

/// Reads the header and the rows.
fn read_table(text: &str) -> Result<Table, LogError> {
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(not_this("the file is empty".to_owned()));
    };
    let header = header.trim();
    if !recognizes(header) {
        return Err(not_this(format!(
            "its header, {:?}, isn't the board's: an empty first name, then names such as \
             `packetType` and `timestamp_ms`",
            short(header)
        )));
    }
    let mut columns: Vec<Column> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if index >= MAX_COLUMNS {
            return Err(syntax(
                header_line,
                format!("the header names more than {MAX_COLUMNS} columns"),
            ));
        }
        if index > 0 && name.is_empty() {
            return Err(syntax(
                header_line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if !seen.insert(name) {
            return Err(syntax(
                header_line,
                format!("the header names {:?} twice", short(name)),
            ));
        }
        let role = if index == 0 {
            Role::Index
        } else {
            role_of(name)
        };
        columns.push(Column::new(name, role));
    }
    let width = columns.len();

    let mut table = Table {
        header_line,
        columns,
        lines: Vec::new(),
        gaps: 0,
        first_gap: None,
        short_rows: 0,
        first_short: None,
    };
    let mut cells_written = 0usize;
    for (line_number, line) in lines {
        // One more than the header names, so that a wider row is told apart without collecting
        // the whole of a long line.
        let row: Vec<&str> = line.trim_end().split(',').take(width + 1).collect();
        if row.len() > width {
            return Err(syntax(
                line_number,
                format!("the row has more than {width} cells, where the header names {width}"),
            ));
        }
        if row.len() < width {
            table.short_rows += 1;
            table.first_short.get_or_insert(line_number);
        }
        cells_written = cells_written.saturating_add(row.len());
        table.lines.push(line_number);
        for (index, column) in table.columns.iter_mut().enumerate() {
            let cell = row.get(index);
            let value = match cell {
                Some(cell) => number(cell, false).unwrap_or_else(|| {
                    table.gaps += 1;
                    table.first_gap.get_or_insert(line_number);
                    f64::NAN
                }),
                None => f64::NAN,
            };
            column.values.push(value);
            if column.role == Role::Pressure {
                column.places.push(
                    cell.filter(|_| value.is_finite())
                        .map(|c| decimal_places(c)),
                );
            }
        }
        if crate::log::too_sparse(width, table.lines.len(), cells_written) {
            return Err(syntax(
                line_number,
                format!(
                    "{} rows hold {cells_written} of the {} cells the header's {width} columns \
                     name: too few for the header to be this file's",
                    table.lines.len(),
                    table.lines.len().saturating_mul(width)
                ),
            ));
        }
    }
    Ok(table)
}

/// Reads the board's CSV.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't the board's header;
/// [`LogError::Syntax`] for a header naming more than [`MAX_COLUMNS`] columns, a column with no
/// name but the first, a name given twice, a row with more cells than the header names, a file
/// whose rows hold too few of the cells its header names, two clock columns that both hold times,
/// or a value too large to hold once converted to its unit; [`LogError::NoData`] if no row has a
/// time; [`LogError::Unit`] if the clock's unit isn't decided (module docs).
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut table = read_table(text)?;
    let header_line = table.header_line;
    let mut decisions: Vec<String> = Vec::new();

    // The clock: whichever of `timestamp_ms` and `timestamp_s` holds times.
    let clocks: Vec<usize> = (0..table.columns.len())
        .filter(|&i| table.columns[i].role == Role::Clock)
        .collect();
    let ticking: Vec<usize> = clocks
        .iter()
        .copied()
        .filter(|&i| !table.columns[i].all_zero())
        .collect();
    let clock = match ticking.as_slice() {
        [one] => *one,
        [] => {
            return Err(syntax(
                header_line,
                "no clock column, `timestamp_ms` or `timestamp_s`, holds times".to_owned(),
            ));
        }
        _ => {
            return Err(syntax(
                header_line,
                format!(
                    "{} both hold times: the reader can't tell which is the board's clock",
                    listed(&table.columns, &ticking)
                ),
            ));
        }
    };

    let pressure = decide_pressures(&mut table.columns, &mut decisions);
    let pressure_m = pressure_altitudes(&table.columns, pressure);
    let raw = raw_accelerometer(&table.columns);
    let (time_s, clock_note) = decide_clock(&table.columns, clock, &pressure_m, raw, header_line)?;
    decisions.push(clock_note);
    let flight = Flight::new(
        time_s,
        pressure_m,
        raw.map(|(axes, pad)| (axes, pad, &table.columns)),
    );

    let columns = &mut table.columns;
    decide_imu_accelerometers(columns, &mut decisions);
    decide_estimate(columns, &flight, &mut decisions);
    decide_gyroscopes(columns, &flight, &mut decisions);
    let reference = reference_accelerometer(columns);
    decide_lengths(columns, &flight, reference, &mut decisions);
    decide_climb(columns, &flight, reference, &mut decisions);
    decide_position(columns, &mut decisions);
    keep_the_rest(columns, clock, &mut decisions);

    build(table, clock, flight, decisions)
}

/// What the flight's checks share: the decided clock, the pressure's altitude and the launch.
struct Flight {
    /// Each row's time, s.
    time_s: Vec<f64>,
    /// The pressure altitude's 5-row running median, against a barometer's spikes.
    smooth_m: Vec<f64>,
    /// The pressure altitude on the pad, m.
    pad_m: Option<f64>,
    /// The row of the highest smoothed pressure altitude.
    apogee: Option<usize>,
    /// The launch's row, and the raw accelerometer's direction on the pad.
    launch: Option<(usize, [usize; 3], [f64; 3])>,
}

impl Flight {
    fn new(
        time_s: Vec<f64>,
        pressure_m: Vec<f64>,
        raw: Option<([usize; 3], [f64; 3], &Vec<Column>)>,
    ) -> Self {
        let smooth_m = running_median(&pressure_m);
        let pad_m = median(pressure_m.iter().take(PAD_ROWS).copied());
        let apogee = highest(&smooth_m);
        let launch = raw.and_then(|(axes, pad, columns)| {
            launch_row(columns, axes, pad).map(|row| (row, axes, pad))
        });
        Self {
            time_s,
            smooth_m,
            pad_m,
            apogee,
            launch,
        }
    }

    /// The pressure's climb above the pad at a row, smoothed.
    fn climb_m(&self, row: usize) -> f64 {
        self.pad_m.map_or(f64::NAN, |pad| self.smooth_m[row] - pad)
    }
}

/// A 5-row running median, `NaN` where the window holds no number.
fn running_median(values: &[f64]) -> Vec<f64> {
    (0..values.len())
        .map(|row| {
            let from = row.saturating_sub(2);
            let to = (row + 3).min(values.len());
            median(values[from..to].iter().copied()).unwrap_or(f64::NAN)
        })
        .collect()
}

/// The row of the highest number, the first if several.
fn highest(values: &[f64]) -> Option<usize> {
    let mut best: Option<usize> = None;
    for (row, value) in values.iter().enumerate() {
        if value.is_finite() && best.is_none_or(|b| *value > values[b]) {
            best = Some(row);
        }
    }
    best
}

/// Decides each pressure's unit (module docs); the column the altitude checks use, if any is read:
/// `baro_pres_avg`, else `baro1_pres`, else `baro2_pres`.
fn decide_pressures(columns: &mut [Column], decisions: &mut Vec<String>) -> Option<usize> {
    for column in columns.iter_mut().filter(|c| c.role == Role::Pressure) {
        decisions.push(decide_pressure(column));
    }
    ["baro_pres_avg", "baro1_pres", "baro2_pres"]
        .iter()
        .find_map(|name| {
            columns
                .iter()
                .position(|c| c.name == *name && c.quantity == Quantity::Pressure)
        })
}

/// Whether every cell of a column precise enough to test lies on a step of `1 / (m × per)`, for
/// m = 1 or 2 (half steps, an average of two): the m, if so, and how many cells were tested at it.
fn on_step(column: &Column, per: f64) -> (Option<u32>, usize) {
    let mut most_tested = 0;
    for m in [1u32, 2] {
        let steps = f64::from(m) * per;
        let mut tested = 0usize;
        let mut on = true;
        for (value, place) in column.values.iter().zip(&column.places) {
            let Some(place) = place else { continue };
            // A cell written too short to test: its printing alone could take half the tolerance.
            if *value == 0.0 || steps * 0.5 * 10f64.powi(-*place) > STEP_TOLERANCE / 2.0 {
                continue;
            }
            tested += 1;
            let in_steps = value * steps;
            let off = (in_steps - in_steps.round()).abs();
            // Written to fail closed: a value that overflows, its distance `NaN`, is off.
            if off.is_nan() || off > STEP_TOLERANCE {
                on = false;
                break;
            }
        }
        if on && tested >= LEAST_STEP_CELLS {
            return (Some(m), tested);
        }
        most_tested = most_tested.max(tested);
    }
    (None, most_tested)
}

/// Decides one pressure's unit: atmospheres if its size and its step both say so (module docs).
fn decide_pressure(column: &mut Column) -> String {
    let name = column.quoted();
    if column.all_zero() {
        return format!("{name} kept as written: every value is 0, a barometer that isn't working");
    }
    let Some(pad) = column.pad() else {
        return format!("{name} kept as written: no value on the pad");
    };
    let size = (PAD_PRESSURE.0..=PAD_PRESSURE.1).contains(&pad);
    let (atm, tested) = on_step(column, ATM_PA);
    let (bar, _) = on_step(column, BAR_PA);
    match (size, atm, bar) {
        (true, Some(m), None) => {
            let step = if m == 1 {
                "a whole number of pascals over 101,325"
            } else {
                "a whole or half number of pascals over 101,325, as an average of two whole ones \
                 is"
            };
            column.read_as(Quantity::Pressure, ATM_PA);
            format!(
                "{name} read in atmospheres, from its size and its step: on the pad it reads \
                 {pad}, within the 0.5 to 1.1 that only atmospheres and bars of the units a \
                 barometer is written in put a pad, and each of its {tested} cells written \
                 precisely enough to test is {step} within a millionth of a step, as a barometer's \
                 pascals written in atmospheres are and in bars, over 100,000, aren't"
            )
        }
        _ => {
            column.quantity = Quantity::Unrecognized;
            let size_says = if size {
                format!("on the pad it reads {pad}, near 1 as atmospheres or bars put a pad")
            } else {
                format!("on the pad it reads {pad}, outside the 0.5 to 1.1 of atmospheres or bars")
            };
            let step_says = match (atm, bar) {
                (Some(_), Some(_)) => {
                    "its values lie on the steps of both atmospheres and bars".to_owned()
                }
                (None, Some(_)) => "its values lie on bars' steps of 1/100,000, not on \
                                    atmospheres' of a pascal over 101,325"
                    .to_owned(),
                (None, None) => format!(
                    "of its cells written precisely enough to test ({tested}), not all lie on \
                     a whole number of pascals over 101,325 or over 100,000, or too few do to \
                     tell"
                ),
                (Some(_), None) => "its values lie on atmospheres' steps".to_owned(),
            };
            format!(
                "{name} kept as written: its size and its step don't both say atmospheres: \
                 {size_says}; {step_says}"
            )
        }
    }
}

/// Each row's pressure altitude in the U.S. Standard Atmosphere, 1976, m, from the pressure the
/// checks use; `NaN` where it has none.
fn pressure_altitudes(columns: &[Column], pressure: Option<usize>) -> Vec<f64> {
    let rows = columns.first().map_or(0, |c| c.values.len());
    let Some(at) = pressure else {
        return vec![f64::NAN; rows];
    };
    let atmosphere = Ussa76::standard();
    let column = &columns[at];
    column
        .values
        .iter()
        .map(|value| {
            let pa = column.conversion.apply(*value);
            if pa.is_finite() && pa > 0.0 {
                atmosphere.pressure_altitude_m(pa).unwrap_or(f64::NAN)
            } else {
                f64::NAN
            }
        })
        .collect()
}

/// The three columns of a sensor, if the file has all three, found by a role per axis.
fn triple(columns: &[Column], role: impl Fn(usize) -> Role) -> Option<[usize; 3]> {
    let find = |axis| columns.iter().position(|c| c.role == role(axis));
    Some([find(0)?, find(1)?, find(2)?])
}

/// The three columns with these names, if the file has all three.
fn named(columns: &[Column], names: [String; 3]) -> Option<[usize; 3]> {
    let find = |name: &String| columns.iter().position(|c| &c.name == name);
    Some([find(&names[0])?, find(&names[1])?, find(&names[2])?])
}

/// A sensor's values at a row, as written.
fn vector(columns: &[Column], axes: [usize; 3], row: usize) -> [f64; 3] {
    axes.map(|at| columns[at].values[row])
}

/// A sensor's reading on the pad: each axis's median over [`PAD_ROWS`].
fn pad_vector(columns: &[Column], axes: [usize; 3]) -> Option<[f64; 3]> {
    Some([
        columns[axes[0]].pad()?,
        columns[axes[1]].pad()?,
        columns[axes[2]].pad()?,
    ])
}

/// The raw accelerometer the launch and the clock's check use, `imu1`'s or else `imu2`'s, and its
/// reading on the pad: in counts, its own unit, which the checks divide out.
fn raw_accelerometer(columns: &[Column]) -> Option<([usize; 3], [f64; 3])> {
    ["imu1", "imu2"].iter().find_map(|imu| {
        let axes = named(columns, ["x", "y", "z"].map(|a| format!("{imu}_accel_{a}")))?;
        let pad = pad_vector(columns, axes)?;
        (magnitude(pad) > 0.0).then_some((axes, pad))
    })
}

/// The launch's row: the first whose specific force, in its pad reading, is at least
/// [`LAUNCH_G`], the next two rows' at least [`LAUNCH_HELD_G`].
fn launch_row(columns: &[Column], axes: [usize; 3], pad: [f64; 3]) -> Option<usize> {
    let one_g = magnitude(pad);
    let rows = columns[axes[0]].values.len();
    let force = |row: usize| magnitude(vector(columns, axes, row)) / one_g;
    (0..rows).find(|&row| {
        force(row) >= LAUNCH_G && (row + 1..(row + 3).min(rows)).all(|r| force(r) >= LAUNCH_HELD_G)
    })
}

/// How an accelerometer's values become m/s².
#[derive(Debug, Clone, Copy)]
enum ToMs2 {
    /// Times a factor.
    Scale(f64),
    /// In its pad reading, 1 g: unit-free.
    PadIsOneG,
}

/// One row of an integrated climb: the row, the speed, m/s, and the climb, m.
type Integrated = (usize, f64, f64);

/// The climb an accelerometer integrates to (module docs): from the launch's row, its specific
/// force along its pad reading's direction, less gravity, integrated once to a speed and again to
/// a climb, row by row on `times` times `scale` seconds, until the pressure's apogee or until the
/// speed falls back to 0.
fn integrate(
    columns: &[Column],
    axes: [usize; 3],
    to_ms2: ToMs2,
    times: &[f64],
    scale: f64,
    launch: usize,
    stop: usize,
) -> Vec<Integrated> {
    let Some(pad) = pad_vector(columns, axes) else {
        return Vec::new();
    };
    let one_g = magnitude(pad);
    if one_g == 0.0 {
        return Vec::new();
    }
    let up = pad.map(|v| v / one_g);
    let factor = match to_ms2 {
        ToMs2::Scale(factor) => factor,
        ToMs2::PadIsOneG => STANDARD_GRAVITY_MPS2 / one_g,
    };
    let mut rows = Vec::new();
    let (mut speed, mut climb) = (0.0, 0.0);
    for row in launch..stop.min(times.len().saturating_sub(1)) {
        let step = (times[row + 1] - times[row]) * scale;
        let force = dot(vector(columns, axes, row), up) * factor;
        if !(step > 0.0 && step.is_finite() && force.is_finite()) {
            continue;
        }
        speed += (force - STANDARD_GRAVITY_MPS2) * step;
        climb += speed * step;
        if row > launch && speed <= 0.0 {
            break;
        }
        rows.push((row + 1, speed, climb));
    }
    rows
}

/// The median, over the integrated rows whose climb reaches [`LEAST_CLIMB_M`], of a quantity at the
/// row over the climb; `None` with under [`LEAST_ROWS`] such rows.
fn climb_ratio(integrated: &[Integrated], of: impl Fn(usize) -> f64) -> Option<f64> {
    let ratios = integrated
        .iter()
        .filter(|(_, _, climb)| *climb >= LEAST_CLIMB_M)
        .map(|&(row, _, climb)| of(row) / climb);
    let ratios: Vec<f64> = ratios.filter(|r| r.is_finite()).collect();
    (ratios.len() >= LEAST_ROWS)
        .then(|| median(ratios.into_iter()))
        .flatten()
}

/// The same over the rows whose speed reaches [`LEAST_SPEED_M_S`], of a quantity over the speed.
fn speed_ratio(integrated: &[Integrated], of: impl Fn(usize) -> f64) -> Option<f64> {
    let ratios = integrated
        .iter()
        .filter(|(_, speed, _)| *speed >= LEAST_SPEED_M_S)
        .map(|&(row, speed, _)| of(row) / speed);
    let ratios: Vec<f64> = ratios.filter(|r| r.is_finite()).collect();
    (ratios.len() >= LEAST_ROWS)
        .then(|| median(ratios.into_iter()))
        .flatten()
}

/// Decides the clock's unit, milliseconds or seconds, from its spacing and the climb (module
/// docs): each row's time, s, and the note; refused if the two don't agree on one unit.
fn decide_clock(
    columns: &[Column],
    clock: usize,
    pressure_m: &[f64],
    raw: Option<([usize; 3], [f64; 3])>,
    header_line: usize,
) -> Result<(Vec<f64>, String), LogError> {
    let column = &columns[clock];
    let times = &column.values;
    let step = median(
        times
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .filter(|step| *step > 0.0),
    );
    let units = [("milliseconds", 1e-3), ("seconds", 1.0)];
    let spaced = |scale: f64| {
        step.is_some_and(|step| (ROW_STEP_S.0..=ROW_STEP_S.1).contains(&(step * scale)))
    };
    let smooth = running_median(pressure_m);
    let pad_m = median(pressure_m.iter().take(PAD_ROWS).copied());
    let launch = raw.and_then(|(axes, pad)| launch_row(columns, axes, pad));
    let apogee = highest(&smooth);
    let climbed = |scale: f64| -> Option<f64> {
        let ((axes, _), launch, apogee, pad_m) = (raw?, launch?, apogee?, pad_m?);
        let integrated = integrate(
            columns,
            axes,
            ToMs2::PadIsOneG,
            times,
            scale,
            launch,
            apogee,
        );
        climb_ratio(&integrated, |row| smooth[row] - pad_m)
    };
    let found: Vec<(f64, Option<f64>)> = units.iter().map(|(_, s)| (*s, climbed(*s))).collect();
    let name = column.quoted();
    let step_text = step.map_or_else(|| "no step".to_owned(), |s| format!("{s}"));
    let climb_text = |ratio: Option<f64>| {
        ratio.map_or_else(
            || "none, too little to tell".to_owned(),
            |r| format!("{r:.3}"),
        )
    };
    let evidence = format!(
        "its rows' median step is {step_text} as written, and the climb the pressure reads from \
         the launch, over the one the raw accelerometer integrates to in its pad reading's 1 g, \
         is, read in milliseconds, {}, and read in seconds, {}; a row every 5 to 500 ms, as the \
         board logs, and a ratio within a factor of {CLOCK_FACTOR} of 1 are taken",
        climb_text(found[0].1),
        climb_text(found[1].1)
    );
    let agreed: Vec<usize> = (0..units.len())
        .filter(|&i| {
            spaced(units[i].1) && found[i].1.is_some_and(|r| within_factor(r, CLOCK_FACTOR))
        })
        .collect();
    match agreed.as_slice() {
        [i] => {
            let (unit, scale) = units[*i];
            let time_s = if scale == 1.0 {
                times.clone()
            } else {
                times.iter().map(|t| t / 1_000.0).collect()
            };
            Ok((
                time_s,
                format!("{name} read in {unit}, from its spacing and the climb: {evidence}"),
            ))
        }
        _ => Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "the clock {name} is in no unit both its spacing and the climb bear out: \
                 {evidence}"
            ),
        }),
    }
}

/// An accelerometer's unit: m/s² or standard gravities, and the factor to m/s².
const ACCEL_UNITS: [(&str, f64); 2] = [
    ("meters per second squared", 1.0),
    ("standard gravities, × 9.806 65 m/s²", STANDARD_GRAVITY_MPS2),
];

/// Which of [`ACCEL_UNITS`] reads 1 g at rest: the magnitude of a sensor's pad reading within
/// [`GRAVITY_MARGIN`] of 9.806 65 or of 1.
fn gravity_unit(columns: &[Column], axes: [usize; 3]) -> (Option<usize>, Option<f64>) {
    let Some(pad) = pad_vector(columns, axes) else {
        return (None, None);
    };
    let reading = magnitude(pad);
    let unit = ACCEL_UNITS.iter().position(|(_, scale)| {
        within_margin(reading * scale / STANDARD_GRAVITY_MPS2, GRAVITY_MARGIN)
    });
    (unit, Some(reading))
}

/// Whether every number of a column is its raw count's times one factor, within [`SAME`]: `None`
/// if not, `Some(None)` if every count is 0 and so is every number (any factor fits), and
/// `Some(Some(factor))` otherwise.
fn factor_of(column: &Column, raw: &Column) -> Option<Option<f64>> {
    let mut found: Option<f64> = None;
    for (value, count) in column.values.iter().zip(&raw.values) {
        if !(value.is_finite() && count.is_finite()) {
            continue;
        }
        if *count == 0.0 {
            if *value != 0.0 {
                return None;
            }
            continue;
        }
        let ratio = value / count;
        match found {
            None => found = Some(ratio),
            Some(f) if ((ratio - f) / f).abs() <= SAME => {}
            Some(_) => return None,
        }
    }
    Some(found)
}

/// The one factor a sensor's three scaled columns are their raw counts times (module docs): every
/// axis must fit it, and at least one must have a count that isn't 0 to set it.
fn common_factor(columns: &[Column], axes: [usize; 3]) -> Option<f64> {
    let mut common: Option<f64> = None;
    for at in axes {
        let raw = raw_of(columns, at)?;
        let Some(factor) = factor_of(&columns[at], &columns[raw])? else {
            continue;
        };
        match common {
            None => common = Some(factor),
            Some(f) if ((factor - f) / f).abs() <= SAME => {}
            Some(_) => return None,
        }
    }
    common
}

/// The raw column of a scaled one: its name without `_real`.
fn raw_of(columns: &[Column], at: usize) -> Option<usize> {
    let raw = columns[at].name.strip_suffix("_real")?;
    columns.iter().position(|c| c.name == raw)
}

/// Whether a scale is one of a list's within [`SAME`].
fn listed_scale(scale: f64, list: &[f64]) -> Option<f64> {
    list.iter()
        .copied()
        .find(|known| ((scale - known) / known).abs() <= SAME)
}

/// Decides each IMU's and the high-g accelerometer's scaled columns, then the IMUs' average
/// (module docs).
fn decide_imu_accelerometers(columns: &mut [Column], decisions: &mut Vec<String>) {
    let mut decided: Vec<(u8, Option<usize>)> = Vec::new();
    for imu in [1u8, 2] {
        if let Some(axes) = triple(columns, |a| Role::AccelReal(imu, a)) {
            let unit = decide_scaled_accelerometer(columns, axes, decisions);
            decided.push((imu, unit));
        }
    }
    if let Some(axes) = triple(columns, Role::HighG) {
        decide_scaled_accelerometer(columns, axes, decisions);
    }
    let Some(axes) = triple(columns, Role::AccelAverage) else {
        return;
    };
    let names = listed(columns, &axes);
    let (unit, reading) = gravity_unit(columns, axes);
    let inputs = decided.iter().map(|(_, unit)| *unit).collect::<Vec<_>>();
    let same_unit = inputs
        .first()
        .copied()
        .flatten()
        .filter(|u| inputs.iter().all(|i| *i == Some(*u)));
    let mean = same_unit.is_some()
        && (0..3).all(|axis| is_mean(columns, axes[axis], |imu| Role::AccelReal(imu, axis)));
    let reading = reading.map_or_else(|| "nothing".to_owned(), |r| format!("{r:.3}"));
    match (unit, same_unit, mean) {
        (Some(unit), Some(input), true) if unit == input => {
            for at in axes {
                columns[at].read_as(Quantity::Acceleration, ACCEL_UNITS[unit].1);
            }
            decisions.push(format!(
                "{names} read in {}, from the pad's gravity and the IMUs: at rest they read \
                 {reading}, 1 g in that unit within 10%, and each is the mean of the IMUs' scaled \
                 columns, read in it",
                ACCEL_UNITS[unit].0
            ));
        }
        _ => {
            let why = if same_unit.is_none() {
                "the IMUs' scaled columns they average aren't all read in one unit, so neither is \
                 their mean"
            } else if !mean {
                "they aren't the mean of the IMUs' scaled columns"
            } else {
                "they don't read 1 g at rest in the IMUs' unit"
            };
            decisions.push(format!(
                "{names} kept as written: the pad's gravity and the IMUs don't both bear one unit \
                 out: {why}; at rest they read {reading}"
            ));
        }
    }
}

/// Whether a column is the mean of the IMUs' columns of a role, every row within [`SAME`].
fn is_mean(columns: &[Column], at: usize, role: impl Fn(u8) -> Role) -> bool {
    let inputs: Vec<usize> = [1u8, 2]
        .iter()
        .filter_map(|imu| columns.iter().position(|c| c.role == role(*imu)))
        .collect();
    if inputs.is_empty() {
        return false;
    }
    let count = inputs.len() as f64;
    let mut any = false;
    for row in 0..columns[at].values.len() {
        let value = columns[at].values[row];
        let mean = inputs.iter().map(|&i| columns[i].values[row]).sum::<f64>() / count;
        if !(value.is_finite() && mean.is_finite()) {
            continue;
        }
        any = true;
        let off = (value - mean).abs();
        if off.is_nan() || off > SAME * value.abs().max(1.0) {
            return false;
        }
    }
    any
}

/// Decides a scaled accelerometer's three columns by the pad's gravity and the sensor's scale
/// (module docs); the unit, if decided.
fn decide_scaled_accelerometer(
    columns: &mut [Column],
    axes: [usize; 3],
    decisions: &mut Vec<String>,
) -> Option<usize> {
    let names = listed(columns, &axes);
    if axes.iter().all(|&at| columns[at].all_zero()) {
        decisions.push(format!(
            "{names} kept as written: every value is 0, a sensor that isn't working"
        ));
        return None;
    }
    let (gravity, reading) = gravity_unit(columns, axes);
    let common = common_factor(columns, axes);
    let scale_unit = common.and_then(|factor| {
        ACCEL_UNITS.iter().position(|(_, to_ms2)| {
            listed_scale(factor * to_ms2 / STANDARD_GRAVITY_MPS2, &ACCEL_SCALES_G).is_some()
        })
    });
    let reading = reading.map_or_else(|| "nothing".to_owned(), |r| format!("{r:.3}"));
    let factor = common.map_or_else(
        || "they aren't their raw counts times one factor".to_owned(),
        |f| format!("each is its raw count times {f:.7}"),
    );
    match (gravity, scale_unit) {
        (Some(g), Some(s)) if g == s => {
            for at in axes {
                columns[at].read_as(Quantity::Acceleration, ACCEL_UNITS[g].1);
            }
            decisions.push(format!(
                "{names} read in {}, from the pad's gravity and the sensor's scale: at rest they \
                 read {reading}, 1 g in that unit within 10%, and {factor}, in that unit one of \
                 the counts of STMicroelectronics' LSM9DS1 (0.061, 0.122, 0.244 or 0.732 mg)",
                ACCEL_UNITS[g].0
            ));
            Some(g)
        }
        _ => {
            let scale_says = match scale_unit {
                Some(s) => format!(
                    "{factor}, one of the LSM9DS1's counts read in {}",
                    ACCEL_UNITS[s].0
                ),
                None => format!("{factor}, none of the LSM9DS1's counts in either unit"),
            };
            decisions.push(format!(
                "{names} kept as written: the pad's gravity and the sensor's scale don't both \
                 bear one unit out: at rest they read {reading}, 1 g within 10% in {}; \
                 {scale_says}",
                gravity.map_or("neither m/s² nor g", |g| if g == 0 {
                    "m/s²"
                } else {
                    "g"
                })
            ));
            None
        }
    }
}

/// Decides the board's estimate `acc_*` by the pad's gravity and the climb (module docs).
fn decide_estimate(columns: &mut [Column], flight: &Flight, decisions: &mut Vec<String>) {
    let Some(axes) = triple(columns, Role::Estimate) else {
        return;
    };
    let names = listed(columns, &axes);
    let (gravity, reading) = gravity_unit(columns, axes);
    // The climb integrated in each unit: gravity is taken off in m/s², so a unit's climb isn't
    // another's times a factor.
    let ratios: Vec<Option<f64>> = ACCEL_UNITS
        .iter()
        .map(|(_, scale)| match (flight.launch, flight.apogee) {
            (Some((launch, _, _)), Some(apogee)) => {
                let integrated = integrate(
                    columns,
                    axes,
                    ToMs2::Scale(*scale),
                    &flight.time_s,
                    1.0,
                    launch,
                    apogee,
                );
                climb_ratio(&integrated, |row| flight.climb_m(row))
            }
            _ => None,
        })
        .collect();
    let climbed = (0..ACCEL_UNITS.len())
        .filter(|&u| ratios[u].is_some_and(|r| within_factor(r, LENGTH_FACTOR)))
        .collect::<Vec<_>>();
    let climbed = match climbed.as_slice() {
        [one] => Some(*one),
        _ => None,
    };
    let reading = reading.map_or_else(|| "nothing".to_owned(), |r| format!("{r:.3}"));
    let text = |r: Option<f64>| {
        r.map_or_else(
            || "none, too little to tell".to_owned(),
            |r| format!("{r:.3}"),
        )
    };
    let evidence = format!(
        "at rest they read {reading}, and the pressure's climb from the launch over the one they \
         integrate to is, read in m/s², {}, and read in g, {}; 1 g within 10% and a ratio within \
         a factor of {LENGTH_FACTOR} of 1 are taken",
        text(ratios[0]),
        text(ratios[1])
    );
    match (gravity, climbed) {
        (Some(g), Some(c)) if g == c => {
            for at in axes {
                columns[at].read_as(Quantity::Acceleration, ACCEL_UNITS[g].1);
            }
            decisions.push(format!(
                "{names} read in {}, from the pad's gravity and the climb: {evidence}",
                ACCEL_UNITS[g].0
            ));
        }
        _ => decisions.push(format!(
            "{names} kept as written: the pad's gravity and the climb don't both bear one unit out: \
             {evidence}"
        )),
    }
}

/// What the turning check found.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Turning {
    stretches: usize,
    /// The accelerometer's direction's turn, rad.
    accelerometer_rad: f64,
    /// The gyroscope's turn across that direction, its rates read in rad/s, rad.
    gyroscope_rad: f64,
}

/// The turning check (module docs), with the raw accelerometer's direction.
fn turning(columns: &[Column], flight: &Flight, gyro: [usize; 3]) -> Turning {
    let mut found = Turning {
        stretches: 0,
        accelerometer_rad: 0.0,
        gyroscope_rad: 0.0,
    };
    let Some((_, accel, pad)) = flight.launch else {
        return found;
    };
    let one_g = magnitude(pad);
    let times = &flight.time_s;
    let near_g = |row: usize| {
        let reading = magnitude(vector(columns, accel, row)) / one_g;
        reading.is_finite() && (reading - 1.0).abs() <= TURN_GRAVITY_MARGIN
    };
    let fast = |row: usize| {
        let step = times[row + 1] - times[row];
        step > 0.0
            && step <= FAST_STEP_S
            && vector(columns, gyro, row).iter().all(|v| v.is_finite())
    };
    let direction = |row: usize| {
        let a = vector(columns, accel, row);
        let m = magnitude(a);
        a.map(|v| v / m)
    };
    let mut start = 0;
    while start + TURN_STEPS < times.len() {
        let end = start + TURN_STEPS;
        if !((start..end).all(fast) && (start..=end).all(near_g)) {
            start += 1;
            continue;
        }
        let cosine = dot(direction(start), direction(end)).clamp(-1.0, 1.0);
        let mut gyroscope = 0.0;
        for row in start..end {
            let w = vector(columns, gyro, row);
            let a = direction(row);
            let across = [
                w[1] * a[2] - w[2] * a[1],
                w[2] * a[0] - w[0] * a[2],
                w[0] * a[1] - w[1] * a[0],
            ];
            gyroscope += magnitude(across) * (times[row + 1] - times[row]);
        }
        found.stretches += 1;
        found.accelerometer_rad += cosine.acos();
        found.gyroscope_rad += gyroscope;
        start = end;
    }
    found
}

/// A gyroscope's unit: rad/s or °/s, and the factor to °/s.
const GYRO_UNITS: [(&str, f64); 2] = [
    ("radians per second", 180.0 / PI),
    ("degrees per second", 1.0),
];

/// Which of [`GYRO_UNITS`] the turning check bears out, and its evidence.
fn turning_unit(columns: &[Column], flight: &Flight, gyro: [usize; 3]) -> (Option<usize>, String) {
    let found = turning(columns, flight, gyro);
    let ratio = found.gyroscope_rad / found.accelerometer_rad;
    let enough =
        found.stretches >= TURN_LEAST_STRETCHES && found.accelerometer_rad >= TURN_LEAST_RAD;
    let unit = GYRO_UNITS
        .iter()
        .position(|(_, to_dps)| enough && within_factor(ratio * to_dps * PI / 180.0, TURN_FACTOR));
    let evidence = format!(
        "over {} stretches of {TURN_STEPS} steps at most {FAST_STEP_S} s apart with the \
         accelerometer within 15% of 1 g, its direction turns {:.3} rad, and the rates turn the \
         board across it {:.3} rad read in radians per second, {:.3} rad read in degrees; the \
         unit whose turn is within a quarter to 4 times the accelerometer's is taken, from 3 \
         stretches and 0.2 rad",
        found.stretches,
        found.accelerometer_rad,
        found.gyroscope_rad,
        found.gyroscope_rad * PI / 180.0
    );
    (unit, evidence)
}

/// Decides each IMU's scaled gyroscope by the board's turning and the sensor's scale, then the
/// IMUs' average by the turning and the mean (module docs).
fn decide_gyroscopes(columns: &mut [Column], flight: &Flight, decisions: &mut Vec<String>) {
    let mut inputs: Vec<Option<usize>> = Vec::new();
    for imu in [1u8, 2] {
        let Some(axes) = triple(columns, |a| Role::GyroReal(imu, a)) else {
            continue;
        };
        let names = listed(columns, &axes);
        if axes.iter().all(|&at| columns[at].all_zero()) {
            decisions.push(format!(
                "{names} kept as written: every value is 0, a sensor that isn't working"
            ));
            inputs.push(None);
            continue;
        }
        let (turned, evidence) = turning_unit(columns, flight, axes);
        let common = common_factor(columns, axes);
        let scaled = common.and_then(|factor| {
            GYRO_UNITS
                .iter()
                .position(|(_, to_dps)| listed_scale(factor * to_dps, &GYRO_SCALES_DPS).is_some())
        });
        let factor = common.map_or_else(
            || "they aren't their raw counts times one factor".to_owned(),
            |f| format!("each is its raw count times {f:.7}"),
        );
        match (turned, scaled) {
            (Some(t), Some(s)) if t == s => {
                for at in axes {
                    columns[at].read_as(Quantity::AngularRate, GYRO_UNITS[t].1);
                }
                decisions.push(format!(
                    "{names} read in {}, from the board's turning and the sensor's scale: \
                     {evidence}; and {factor}, in that unit one of the counts of \
                     STMicroelectronics' LSM9DS1 (8.75, 17.5 or 70 m°/s)",
                    GYRO_UNITS[t].0
                ));
                inputs.push(Some(t));
            }
            _ => {
                let scale_says = match scaled {
                    Some(s) => format!(
                        "{factor}, one of the LSM9DS1's counts read in {}",
                        GYRO_UNITS[s].0
                    ),
                    None => format!("{factor}, none of the LSM9DS1's counts in either unit"),
                };
                decisions.push(format!(
                    "{names} kept as written: the board's turning and the sensor's scale don't \
                     agree on a unit: {evidence}; {scale_says}"
                ));
                inputs.push(None);
            }
        }
    }
    let Some(axes) = triple(columns, Role::GyroAverage) else {
        return;
    };
    let names = listed(columns, &axes);
    let same_unit = inputs
        .first()
        .copied()
        .flatten()
        .filter(|u| inputs.iter().all(|i| *i == Some(*u)));
    let mean = same_unit.is_some()
        && (0..3).all(|axis| is_mean(columns, axes[axis], |imu| Role::GyroReal(imu, axis)));
    let (turned, evidence) = turning_unit(columns, flight, axes);
    match (turned, same_unit, mean) {
        (Some(t), Some(input), true) if t == input => {
            for at in axes {
                columns[at].read_as(Quantity::AngularRate, GYRO_UNITS[t].1);
            }
            decisions.push(format!(
                "{names} read in {}, from the board's turning and the IMUs: {evidence}; and each \
                 is the mean of the IMUs' scaled columns, read in it",
                GYRO_UNITS[t].0
            ));
        }
        _ => {
            let why = if same_unit.is_none() {
                "the IMUs' scaled columns they average aren't all read in one unit, so neither is \
                 their mean"
            } else if !mean {
                "they aren't the mean of the IMUs' scaled columns"
            } else {
                "the turning doesn't bear out the IMUs' unit"
            };
            decisions.push(format!(
                "{names} kept as written: the board's turning and the IMUs don't both bear one \
                 unit out: {why}; {evidence}"
            ));
        }
    }
}

/// The accelerometer the lengths' and the speed's checks integrate, read in a unit: `acc_*`, else
/// an IMU's scaled columns, else their average; with its factor to m/s².
fn reference_accelerometer(columns: &[Column]) -> Option<([usize; 3], f64)> {
    let roles: [&dyn Fn(usize) -> Role; 4] = [
        &Role::Estimate,
        &|a| Role::AccelReal(1, a),
        &|a| Role::AccelReal(2, a),
        &Role::AccelAverage,
    ];
    roles.iter().find_map(|role| {
        let axes = triple(columns, role)?;
        let column = &columns[axes[0]];
        match (column.quantity, column.conversion) {
            (Quantity::Acceleration, Conversion::Scale(scale)) => Some((axes, scale)),
            _ => None,
        }
    })
}

/// A length's units: meters and feet, and the factor to meters.
const LENGTH_UNITS: [(&str, f64); 2] = [("meters", 1.0), ("feet", FOOT_M)];

/// The accelerometer's integrated rows, if the flight and the accelerometer allow.
fn integrated(
    columns: &[Column],
    flight: &Flight,
    reference: Option<([usize; 3], f64)>,
) -> Vec<Integrated> {
    match (reference, flight.launch, flight.apogee) {
        (Some((axes, scale)), Some((launch, _, _)), Some(apogee)) => integrate(
            columns,
            axes,
            ToMs2::Scale(scale),
            &flight.time_s,
            1.0,
            launch,
            apogee,
        ),
        _ => Vec::new(),
    }
}

/// Decides `pos_z`'s, the barometric altitudes' and the GPS altitude's units (module docs).
fn decide_lengths(
    columns: &mut [Column],
    flight: &Flight,
    reference: Option<([usize; 3], f64)>,
    decisions: &mut Vec<String>,
) {
    let rows = integrated(columns, flight, reference);
    let pressure_climb = flight.apogee.map(|row| flight.climb_m(row));
    for column in columns.iter_mut() {
        let role = column.role;
        if !matches!(role, Role::Height | Role::BaroAltitude | Role::GpsAltitude) {
            continue;
        }
        let name = column.quoted();
        if column.all_zero() {
            decisions.push(format!(
                "{name} kept as written: every value is 0, no reading"
            ));
            continue;
        }
        let Some(pad) = column.pad() else {
            decisions.push(format!("{name} kept as written: no value on the pad"));
            continue;
        };
        // Check 1: the gain against the pressure's climb.
        let (gain, gain_how) = if role == Role::GpsAltitude {
            (
                fresh_fix_gain(column, flight, pad),
                "at the GPS's fresh fixes up to the apogee",
            )
        } else {
            let smooth = running_median(&column.values);
            let top = highest(&smooth).map(|row| smooth[row] - pad);
            let gain = match (top, pressure_climb) {
                (Some(top), Some(climb)) if climb >= LEAST_CLIMB_M => Some(top / climb),
                _ => None,
            };
            (gain, "from the pad to its highest")
        };
        // Check 2: the climb against the accelerometer's.
        let ratio = climb_ratio(&rows, |row| column.values[row] - pad);
        let by_gain = LENGTH_UNITS
            .iter()
            .position(|(_, s)| gain.is_some_and(|g| within_margin(g * s, GAIN_MARGIN)));
        let by_climb = LENGTH_UNITS
            .iter()
            .position(|(_, s)| ratio.is_some_and(|r| within_factor(r * s, LENGTH_FACTOR)));
        let text = |x: Option<f64>| {
            x.map_or_else(
                || "none, too little to tell".to_owned(),
                |x| format!("{x:.3}"),
            )
        };
        let evidence = format!(
            "its climb {gain_how} over the pressure's in the standard atmosphere is {} as written, \
             and its climb from the launch over the one the accelerometer integrates to is {}; a \
             gain within 25% of 1 and a ratio within a factor of {LENGTH_FACTOR} are taken",
            text(gain),
            text(ratio)
        );
        let quantity = match role {
            Role::GpsAltitude => Quantity::GnssAltitude,
            _ => Quantity::AltitudeAboveGround,
        };
        match (by_gain, by_climb) {
            (Some(g), Some(c)) if g == c => {
                column.read_as(quantity, LENGTH_UNITS[g].1);
                decisions.push(format!(
                    "{name} read in {}, from the pressure's climb and the accelerometer's: \
                     {evidence}",
                    LENGTH_UNITS[g].0
                ));
            }
            _ => decisions.push(format!(
                "{name} kept as written: the pressure's climb and the accelerometer's don't \
                 agree on a unit: {evidence}"
            )),
        }
    }
}

/// The GPS altitude's gain against the pressure's: at the rows up to the apogee where it differs
/// from the row before, a fresh fix, the least squares slope through the pad of its rise on the
/// pressure's; `None` under 3 fixes or a pressure climb under [`LEAST_CLIMB_M`] among them.
fn fresh_fix_gain(column: &Column, flight: &Flight, pad: f64) -> Option<f64> {
    let apogee = flight.apogee?;
    let (mut cross, mut square, mut fixes, mut top) = (0.0, 0.0, 0usize, 0.0f64);
    for row in 1..=apogee.min(column.values.len().saturating_sub(1)) {
        let (before, value) = (column.values[row - 1], column.values[row]);
        let climb = flight.climb_m(row);
        if value == before || !(value.is_finite() && climb.is_finite()) {
            continue;
        }
        cross += climb * (value - pad);
        square += climb * climb;
        fixes += 1;
        top = top.max(climb);
    }
    (fixes >= 3 && top >= LEAST_CLIMB_M).then_some(cross / square)
}

/// Decides `vel_z`'s unit by `pos_z`'s change and the accelerometer's speed (module docs).
fn decide_climb(
    columns: &mut [Column],
    flight: &Flight,
    reference: Option<([usize; 3], f64)>,
    decisions: &mut Vec<String>,
) {
    let Some(at) = columns.iter().position(|c| c.role == Role::Climb) else {
        return;
    };
    let name = columns[at].quoted();
    if columns[at].all_zero() {
        decisions.push(format!(
            "{name} kept as written: every value is 0, no reading"
        ));
        return;
    }
    // Check 1: against `pos_z`'s change, read in meters.
    let height = columns
        .iter()
        .position(|c| c.role == Role::Height && c.quantity == Quantity::AltitudeAboveGround);
    let slope = height.and_then(|h| {
        let Conversion::Scale(to_m) = columns[h].conversion else {
            return None;
        };
        let (v, z, t) = (&columns[at].values, &columns[h].values, &flight.time_s);
        let (mut cross, mut square, mut moved) = (0.0, 0.0, 0.0);
        for row in 0..t.len().saturating_sub(1) {
            let step = t[row + 1] - t[row];
            let traveled = 0.5 * (v[row] + v[row + 1]) * step;
            let risen = (z[row + 1] - z[row]) * to_m;
            if !(step > 0.0 && step <= FAST_STEP_S && traveled.is_finite() && risen.is_finite()) {
                continue;
            }
            cross += traveled * risen;
            square += traveled * traveled;
            moved += traveled.abs();
        }
        (moved >= SPEED_LEAST_M).then_some(cross / square)
    });
    // Check 2: against the accelerometer's speed.
    let rows = integrated(columns, flight, reference);
    let ratio = speed_ratio(&rows, |row| columns[at].values[row]);
    // The slope is of meters on the speed as written: in feet a second it is 0.3048.
    let by_slope = LENGTH_UNITS
        .iter()
        .position(|(_, s)| slope.is_some_and(|k| within_margin(k / s, GAIN_MARGIN)));
    let by_speed = LENGTH_UNITS
        .iter()
        .position(|(_, s)| ratio.is_some_and(|r| within_factor(r * s, LENGTH_FACTOR)));
    let text = |x: Option<f64>| {
        x.map_or_else(
            || "none, too little to tell".to_owned(),
            |x| format!("{x:.3}"),
        )
    };
    let evidence = format!(
        "over the steps at most {FAST_STEP_S} s long, `pos_z`'s change in meters is {} times what \
         it integrates to as written, and from the launch it is {} times the speed the \
         accelerometer integrates to; a slope within 25% of 1 and a ratio within a factor of \
         {LENGTH_FACTOR} are taken",
        text(slope),
        text(ratio)
    );
    match (by_slope, by_speed) {
        (Some(s), Some(r)) if s == r => {
            columns[at].read_as(Quantity::Speed, LENGTH_UNITS[s].1);
            let unit = if s == 0 {
                "meters per second"
            } else {
                "feet per second"
            };
            decisions.push(format!(
                "{name} read in {unit}, from `pos_z` and the accelerometer: {evidence}"
            ));
        }
        _ => decisions.push(format!(
            "{name} kept as written: `pos_z` and the accelerometer don't both bear one unit out: \
             {evidence}"
        )),
    }
}

/// Decides the GPS's position: NMEA's degrees and minutes if the values and their `_mod`
/// companions both say so (module docs).
fn decide_position(columns: &mut [Column], decisions: &mut Vec<String>) {
    let find = |role: Role| columns.iter().position(|c| c.role == role);
    let (Some(lat), Some(long)) = (find(Role::Latitude), find(Role::Longitude)) else {
        return;
    };
    let mods = (find(Role::LatitudeMod), find(Role::LongitudeMod));
    let mut at = vec![lat, long];
    at.extend(mods.0);
    at.extend(mods.1);
    let names = listed(columns, &[lat, long]);
    let all_names = listed(columns, &at);
    if columns[lat].all_zero() && columns[long].all_zero() {
        decisions.push(format!(
            "{all_names} kept as written: every value is 0, a receiver without a fix"
        ));
        return;
    }
    let numbers = |i: usize| columns[i].values.iter().copied().filter(|v| v.is_finite());
    // Check 1: the values. Degrees reach 90 and 180; degrees and minutes write minutes under 60.
    let beyond_degrees =
        numbers(lat).any(|v| v.abs() > 90.0) || numbers(long).any(|v| v.abs() > 180.0);
    let minutes_ok = numbers(lat)
        .chain(numbers(long))
        .all(|v| v.abs() % 100.0 < 60.0);
    // Check 2: the companions hold the values converted from degrees and minutes.
    let converted = |value: usize, companion: Option<usize>| {
        companion.is_some_and(|c| {
            let mut any = false;
            let all = columns[value]
                .values
                .iter()
                .zip(&columns[c].values)
                .filter(|(v, m)| v.is_finite() && m.is_finite())
                .all(|(v, m)| {
                    any = true;
                    let expected = degrees_minutes(*v);
                    (expected - m).abs() <= SAME * expected.abs().max(1.0)
                });
            any && all
        })
    };
    let companions = converted(lat, mods.0) && converted(long, mods.1);
    let values_say = if beyond_degrees && minutes_ok {
        "they pass 90 or 180, which no degrees do, and every one's last two digits before the \
         point, its minutes, are under 60"
    } else if !minutes_ok {
        "a value's last two digits before the point are 60 or more, which no minutes are, so they \
         aren't degrees and minutes"
    } else {
        "every value lies within ±90 and ±180, as degrees and degrees and minutes both can"
    };
    if beyond_degrees && minutes_ok && companions {
        columns[lat].quantity = Quantity::Latitude;
        columns[lat].conversion = Conversion::DegreesMinutes;
        columns[long].quantity = Quantity::Longitude;
        columns[long].conversion = Conversion::DegreesMinutes;
        decisions.push(format!(
            "{names} read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their values and \
             their companions: {values_say}, and `gps_lat_mod` and `gps_long_mod` hold them \
             converted from degrees and minutes to degrees"
        ));
        if let (Some(lat_mod), Some(long_mod)) = mods {
            columns[lat_mod].read_as(Quantity::Latitude, 1.0);
            columns[long_mod].read_as(Quantity::Longitude, 1.0);
            decisions.push(format!(
                "{} read in degrees, from {names}: each is its companion, read in degrees and \
                 minutes, converted to degrees",
                listed(columns, &[lat_mod, long_mod])
            ));
        }
    } else {
        let copied = |value: usize, companion: Option<usize>| {
            companion.is_some_and(|c| columns[value].values == columns[c].values)
        };
        let companions_say = if companions {
            "`gps_lat_mod` and `gps_long_mod` hold them converted as degrees and minutes are"
        } else if copied(lat, mods.0) && copied(long, mods.1) {
            "`gps_lat_mod` and `gps_long_mod` hold the same values, a copy that checks no unit"
        } else {
            "`gps_lat_mod` and `gps_long_mod` don't hold them converted from degrees and minutes"
        };
        decisions.push(format!(
            "{all_names} kept as written: the values and their companions don't both bear one unit out: \
             {values_say}; {companions_say}"
        ));
    }
}

/// Notes on the columns kept as written that no check decides: raw counts, magnetometers,
/// temperatures, batteries and the columns no check settles; and on the clock column that isn't
/// the clock, and those the reader doesn't know.
fn keep_the_rest(columns: &[Column], clock: usize, decisions: &mut Vec<String>) {
    // The raw counts and the magnetometers, a note a sensor.
    let mut sensors: Vec<(String, Vec<usize>)> = Vec::new();
    for (at, column) in columns.iter().enumerate() {
        if matches!(column.role, Role::Raw | Role::Magnetic) {
            // The name without its axis: `imu1_mag_x_real` and its kin are `imu1_mag_real`.
            let group: Vec<&str> = column
                .name
                .split('_')
                .filter(|part| axis_of(part).is_none())
                .collect();
            let group = group.join("_");
            match sensors.iter_mut().find(|(g, _)| *g == group) {
                Some((_, at_list)) => at_list.push(at),
                None => sensors.push((group, vec![at])),
            }
        }
    }
    for (_, at) in &sensors {
        let names = listed(columns, at);
        let why = if columns[at[0]].role == Role::Raw {
            "a sensor's raw counts, which the columns named with `_real` hold scaled"
        } else {
            "a magnetometer's reading, a quantity HPR Sim doesn't hold"
        };
        decisions.push(format!("{names} kept as written: {why}"));
    }
    let mut unknown = Notes::default();
    for (at, column) in columns.iter().enumerate() {
        let name = column.quoted();
        match column.role {
            Role::Clock if at != clock => decisions.push(format!(
                "{name} kept as written: {}",
                if column.all_zero() {
                    "every value is 0; the other clock column holds the times"
                } else {
                    "the other clock column holds the times"
                }
            )),
            Role::BaroTemperature => decisions.push(format!(
                "{name} kept as written: {}",
                temperature_why(column)
            )),
            Role::Battery => decisions.push(format!(
                "{name} kept as written: its name states no unit, and nothing in the file checks \
                 one"
            )),
            Role::Unsettled => decisions.push(format!(
                "{name} kept as written: {}",
                if column.all_zero() {
                    "every value is 0, and its name states no unit"
                } else {
                    "its name states no unit, and nothing in the file checks one"
                }
            )),
            Role::Unknown => unknown.push(format!(
                "the column {name} isn't one this reader knows; its values are kept as written"
            )),
            _ => {}
        }
    }
    decisions.extend(unknown.into_vec());
}

/// Why a barometer's temperature is kept as written: its name states no unit, and the values
/// can't tell degrees Celsius from Fahrenheit where both put them within a barometer's working
/// range, −40 to 85 °C (−40 to 185 °F).
fn temperature_why(column: &Column) -> String {
    if column.all_zero() {
        return "every value is 0, and its name states no unit".to_owned();
    }
    let numbers = column.values.iter().copied().filter(|v| v.is_finite());
    let (low, high) = numbers.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
        (lo.min(v), hi.max(v))
    });
    let celsius = low >= -40.0 && high <= 85.0;
    let fahrenheit = low >= -40.0 && high <= 185.0;
    if celsius && fahrenheit {
        format!(
            "its name states no unit, and its values, {low} to {high}, lie within a barometer's \
             working −40 to 85 °C read in degrees Celsius or in Fahrenheit; no other column holds \
             a temperature to tell the two apart"
        )
    } else {
        format!(
            "its name states no unit, and its values, {low} to {high}, lie outside a barometer's \
             working −40 to 85 °C in degrees Celsius or in Fahrenheit, with no other column \
             holding a temperature to check a unit by"
        )
    }
}

/// The record from the decided columns: every channel, the primary track and the notes.
fn build(
    table: Table,
    clock: usize,
    flight: Flight,
    decisions: Vec<String>,
) -> Result<FlightLog, LogError> {
    let Table {
        header_line,
        columns,
        lines,
        gaps,
        first_gap,
        short_rows,
        first_short,
    } = table;
    let times = flight.time_s;
    if !times.iter().any(|t| t.is_finite()) {
        return Err(LogError::NoData { format: FORMAT });
    }
    // Every channel but the clock, in its unit; a value too large to hold once converted is
    // refused.
    let mut channels: Vec<Channel> = Vec::with_capacity(columns.len());
    let mut channel_of: Vec<Option<usize>> = Vec::with_capacity(columns.len());
    for (at, column) in columns.iter().enumerate() {
        if at == clock {
            channel_of.push(None);
            continue;
        }
        let mut values = Vec::with_capacity(column.values.len());
        for (value, line) in column.values.iter().zip(&lines) {
            values.push(table::finite(
                column.conversion.apply(*value),
                FORMAT,
                *line,
                &column.name,
            )?);
        }
        channel_of.push(Some(channels.len()));
        channels.push(Channel {
            column: column.name.clone(),
            quantity: column.quantity,
            time_s: times.clone(),
            values,
        });
    }

    // The primary track: `pos_z`, read in meters, its samples in time order, the first at each
    // time.
    let mut order: Vec<usize> = (0..times.len()).filter(|&i| times[i].is_finite()).collect();
    let untimed = times.len() - order.len();
    let backwards = order
        .windows(2)
        .filter(|pair| times[pair[1]] < times[pair[0]])
        .count();
    order.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
    let before = order.len();
    order.dedup_by(|later, earlier| times[*later] == times[*earlier]);
    let repeats = before - order.len();
    let track_of = |role: Role, quantity: Quantity| {
        columns
            .iter()
            .position(|c| c.role == role && c.quantity == quantity)
            .and_then(|at| channel_of[at])
            .map(|channel| {
                order
                    .iter()
                    .map(|&row| channels[channel].values[row])
                    .collect::<Vec<f64>>()
            })
    };
    let mut notes = decisions;
    let (time_s, altitude_m) = match track_of(Role::Height, Quantity::AltitudeAboveGround) {
        Some(altitude) => (order.iter().map(|&row| times[row]).collect(), altitude),
        None => {
            notes.push(
                "the record's primary track is empty: `pos_z`, the board's altitude, isn't read \
                 in a unit; every column is kept as a channel"
                    .to_owned(),
            );
            (Vec::new(), Vec::new())
        }
    };
    let vertical_speed_m_s = if time_s.is_empty() {
        None
    } else {
        track_of(Role::Climb, Quantity::Speed)
    };
    if backwards > 0 || repeats > 0 || untimed > 0 {
        notes.push(format!(
            "the clock goes back {backwards} times and repeats a time {repeats} times, and \
             {untimed} samples have no time: the record's track takes the samples in time order, \
             the first at each time, and leaves out those without one; the channels keep them all"
        ));
    }
    if let Some(ground) = altitude_m.iter().copied().find(|v| v.is_finite())
        && ground.abs() > GROUND_OFF_M
    {
        notes.push(format!(
            "`pos_z` reads {ground:.2} m at the track's first sample, not near zero: the record \
             starts away from the board's zero"
        ));
    }
    let mut line_notes = Notes::default();
    if gaps > 0 {
        line_notes.push(format!(
            "{gaps} cells, the first on line {}, are empty or not numbers: each is a gap",
            first_gap.unwrap_or(header_line)
        ));
    }
    if short_rows > 0 {
        line_notes.push(format!(
            "{short_rows} rows, the first on line {}, hold fewer cells than the header names: the \
             cells they leave out are gaps",
            first_short.unwrap_or(header_line)
        ));
    }
    notes.extend(line_notes.into_vec());

    Ok(FlightLog {
        format: LogFormat::AerospaceNuFcbCsv,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s,
        altitude_m,
        vertical_speed_m_s,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pressure column from cells as written.
    fn pressure(cells: &[String]) -> Column {
        let mut column = Column::new("baro1_pres", Role::Pressure);
        for cell in cells {
            let value: f64 = cell.parse().unwrap();
            column.values.push(value);
            column.places.push(Some(decimal_places(cell)));
        }
        column
    }

    /// Whole pascals from 97,000 up, times `1 / per`, written as Rust writes the nearest `f64`,
    /// as the club's tools write theirs: to 15 or more digits where the product isn't short.
    fn whole_pascals(per: f64, half: bool) -> Vec<String> {
        (0..40)
            .map(|k| {
                let pa = 97_000.0 + f64::from(k) * 7.0 + if half { 0.5 } else { 0.0 };
                format!("{:?}", pa * (1.0 / per))
            })
            .collect()
    }

    #[test]
    fn degrees_and_minutes_become_degrees_with_their_sign() {
        assert_eq!(degrees_minutes(4230.0), 42.5);
        assert_eq!(degrees_minutes(-7145.0), -71.75);
        assert_eq!(degrees_minutes(15.0), 0.25);
        assert_eq!(degrees_minutes(0.0), 0.0);
    }

    #[test]
    fn a_cells_last_decimal_place_counts_its_exponent() {
        assert_eq!(decimal_places("0.9707179866765359"), 16);
        assert_eq!(decimal_places("12"), 0);
        assert_eq!(decimal_places("1.5e-3"), 4);
        assert_eq!(decimal_places("1.25E2"), 0);
        assert_eq!(decimal_places(" 3.250 "), 3);
    }

    #[test]
    fn columns_take_their_roles_from_their_names() {
        assert_eq!(role_of("timestamp_s"), Role::Clock);
        assert_eq!(role_of("imu2_accel_y_real"), Role::AccelReal(2, 1));
        assert_eq!(role_of("imu1_gyro_z_real"), Role::GyroReal(1, 2));
        assert_eq!(role_of("imu_accel_x_avg"), Role::AccelAverage(0));
        assert_eq!(role_of("imu_mag_x_avg"), Role::Magnetic);
        assert_eq!(role_of("imu1_mag_y_real"), Role::Magnetic);
        assert_eq!(role_of("high_g_accel_z_real"), Role::HighG(2));
        assert_eq!(role_of("high_g_accel_z"), Role::Raw);
        assert_eq!(role_of("imu1_accel_w"), Role::Unknown);
        assert_eq!(role_of("imu3_accel_x_real"), Role::Unknown);
        assert_eq!(role_of("acc_z"), Role::Estimate(2));
        assert_eq!(role_of("state"), Role::Number);
        assert_eq!(role_of("baroalt_agl"), Role::BaroAltitude);
        assert_eq!(role_of("mystery"), Role::Unknown);
    }

    #[test]
    fn recognition_needs_the_empty_first_name_and_a_known_column() {
        assert!(recognizes(",packetType,timestamp_ms,imu1_accel_x\n"));
        assert!(recognizes("\u{feff}\n,timestamp_s,timestamp_ms\n"));
        assert!(!recognizes("packetType,timestamp_ms\n"));
        assert!(!recognizes(",time,altitude\n"));
    }

    /// Atmospheres' step holds whole pascals; bars' doesn't, nor the reverse.
    #[test]
    fn whole_pascals_lie_on_atmospheres_step_and_not_on_bars() {
        let atm = pressure(&whole_pascals(ATM_PA, false));
        assert_eq!(on_step(&atm, ATM_PA), (Some(1), 40));
        assert_eq!(on_step(&atm, BAR_PA).0, None);
        let bar = pressure(&whole_pascals(BAR_PA, false));
        assert_eq!(on_step(&bar, BAR_PA).0, Some(1));
        assert_eq!(on_step(&bar, ATM_PA).0, None);
        // An average of two whole readings: half steps.
        let half = pressure(&whole_pascals(ATM_PA, true));
        assert_eq!(on_step(&half, ATM_PA), (Some(2), 40));
    }

    /// The tolerance is a millionth of a step: a cell 10⁻⁵ of a pascal off is off, as a f64's
    /// printing, 10⁻¹¹ of one, never is.
    #[test]
    fn the_step_tolerance_is_a_millionth_of_a_step() {
        let mut cells = whole_pascals(ATM_PA, false);
        cells[7] = format!("{:?}", (97_049.0 + 1e-5) / ATM_PA);
        assert_eq!(on_step(&pressure(&cells), ATM_PA).0, None);
        cells[7] = format!("{:?}", (97_049.0 + 1e-7) / ATM_PA);
        assert_eq!(on_step(&pressure(&cells), ATM_PA).0, Some(1));
    }

    /// A cell too short to test is left out, as is one too large for its printing to place it on
    /// a step (the record refuses it once converted); under 10 tested cells decide nothing.
    #[test]
    fn short_cells_are_not_tested_and_overflow_fails_closed() {
        let short: Vec<String> = whole_pascals(ATM_PA, false)
            .iter()
            .map(|c| format!("{:.6}", c.parse::<f64>().unwrap()))
            .collect();
        assert_eq!(on_step(&pressure(&short), ATM_PA), (None, 0));
        let few = pressure(&whole_pascals(ATM_PA, false)[..9]);
        assert_eq!(on_step(&few, ATM_PA), (None, 9));
        let mut cells = whole_pascals(ATM_PA, false);
        let tested = on_step(&pressure(&cells), ATM_PA).1;
        cells[3] = "1.7e308".to_owned();
        assert_eq!(on_step(&pressure(&cells), ATM_PA), (Some(1), tested - 1));
    }

    #[test]
    fn a_pressure_is_read_in_atmospheres_on_its_size_and_its_step() {
        let mut atm = pressure(&whole_pascals(ATM_PA, false));
        let note = decide_pressure(&mut atm);
        assert!(note.starts_with("`baro1_pres` read in atmospheres, from its size and its step: "));
        assert_eq!(atm.quantity, Quantity::Pressure);
        assert_eq!(atm.conversion.apply(atm.values[0]), 97_000.0);

        let mut bar = pressure(&whole_pascals(BAR_PA, false));
        let note = decide_pressure(&mut bar);
        assert!(
            note.contains("its values lie on bars' steps of 1/100,000"),
            "{note}"
        );
        assert_eq!(bar.quantity, Quantity::Unrecognized);

        let pascals: Vec<String> = (0..40).map(|k| format!("{}.0", 97_000 + k)).collect();
        let mut pa = pressure(&pascals);
        let note = decide_pressure(&mut pa);
        assert!(
            note.contains("outside the 0.5 to 1.1 of atmospheres or bars"),
            "{note}"
        );
        assert_eq!(pa.quantity, Quantity::Unrecognized);
    }

    /// A scaled cell where its count is 0 must be 0; an axis whose counts are all 0 fits any
    /// factor.
    #[test]
    fn a_factor_holds_at_every_count() {
        let column = |name: &str, values: &[f64]| {
            let mut c = Column::new(name, role_of(name));
            c.values = values.to_vec();
            c
        };
        let raw = column("imu1_accel_x", &[0.0, 2.0, 4.0]);
        assert_eq!(
            factor_of(&column("imu1_accel_x_real", &[0.0, 1.0, 2.0]), &raw),
            Some(Some(0.5))
        );
        assert_eq!(
            factor_of(&column("imu1_accel_x_real", &[0.1, 1.0, 2.0]), &raw),
            None
        );
        let zeros = column("imu1_accel_y", &[0.0; 3]);
        assert_eq!(
            factor_of(&column("imu1_accel_y_real", &[0.0; 3]), &zeros),
            Some(None)
        );
    }

    #[test]
    fn nan_is_within_no_factor_or_margin() {
        assert!(!within_factor(f64::NAN, 10.0));
        assert!(!within_margin(f64::NAN, 0.25));
        assert!(within_factor(0.11, 10.0) && !within_factor(0.09, 10.0));
    }
}
