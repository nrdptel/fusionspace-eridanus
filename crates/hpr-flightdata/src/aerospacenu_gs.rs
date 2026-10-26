//! AerospaceNU's ground station's text: the club's flight computer board's radio downlink, one
//! packet a line, as the ground station parsed it.
//!
//! AerospaceNU, Northeastern University's rocketry club, flies a flight computer board (the FCB)
//! that sends its readings to a ground station by radio, and publishes the ground station's text
//! beside its flights' logs. No published document describes the packets or their units, so
//! every unit a field's name doesn't state comes from two checks on the file's own values that
//! don't depend on each other ([ADR-258][adr-258] §2, the rule that a unit needs two checks),
//! and a unit no two checks settle is kept as written.
//! The club's per-flight summaries aren't read and aren't a check: they exist for few flights
//! and needn't match the packets the ground station received.
//!
//! **Layout.** A `RUN START` line with the ground station's local date and time when it started,
//! then one line a packet or message, each opening with the ground station's time of day when it
//! received it (invented values; the fields shortened):
//!
//! ```text
//! RUN START 2025-01-01 10:00:00
//!
//! 10:00:01.250000: Switching radio to band 1
//! 10:00:01.512345: Position Data {'radio_id': 0, 'rssi': '-50 db', 'crc': 'Good', 'time_stamp_ms': 61000, 'fcb_altitude': 0.25, 'v_speed': 0.0, …}
//! 10:00:01.733210: Alt Info & Cfg {'crc': 'Good', 'time_stamp_ms': 61200, 'barometer_pressure': 0.9677…, …}
//! ```
//!
//! A packet is its name, then its fields as a Python dictionary literal: text in quotes, numbers,
//! `True`, `False`, `None`, `nan`, `inf` and lists. The reader parses that literal itself, bounded:
//! a line over [`MAX_LINE_BYTES`] bytes, lists or dictionaries nested more than 8 deep or holding
//! more than 1,024 items, a key that isn't text or is given twice, make the line unreadable, and it
//! is skipped and counted in a note. So is a line whose literal doesn't parse, one that isn't a
//! packet or a message, and a second `RUN START`. A message, a line with a time and no dictionary,
//! such as `Switching radio to band 1` or the ground station's `Invalid message type …` and
//! `Packet too short …`, is skipped and counted.
//!
//! **Corrupt packets.** Every packet from the board carries `crc`, the ground station's verdict on
//! its radio checksum. A packet whose `crc` isn't `'Good'` is left out, counted in a note: its bytes
//! failed the check, and in the sample files such packets hold a callsign with stray characters,
//! clocks billions of milliseconds off and readings of 10³⁰. Values in a `'Good'` packet are kept as
//! written but for the checks below.
//!
//! **Channels.** Every numeric field of every packet is a channel named `<packet>.<field>`, as
//! `Position Data.fcb_altitude`, each on its packets' own times; an element of a list is
//! `<packet>.<field>[i]`, and `True` and `False` read 1 and 0. A field holding text isn't a channel
//! but for `rssi` (`'-50 db'`, read as its number), `Hdop` (a number in quotes) and `gps_time`
//! (`'YYYY-MM-DD HH:MM:SS'`, read as its time of day); the others, such as `callsign`, `crc` and
//! `gps_string`, the raw NMEA sentence of `GPS Passthrough`, are left out, named in a note. `None`,
//! `nan` and `inf` are gaps, counted. A field the reader doesn't know is kept as written,
//! [`Quantity::Unrecognized`], named in a note for its packet. At most [`MAX_CHANNELS`] channels are
//! kept; values past them, and fields with names over 64 characters, are counted in a note.
//!
//! | Packet | Field | Read as | Unit, and from where |
//! |---|---|---|---|
//! | every board packet | `time_stamp_ms` | the clock (below) | ms, from its name and the ground station's clock |
//! | every board packet | `radio_id`, `lqi`, `software_version`, `board_serial_number`, `rssi` | [`Quantity::Number`] | as written (`rssi` in dB, as its text says) |
//! | `Position Data` | `fcb_altitude` | [`Quantity::AltitudeAboveGround`]: the primary track | m, from the barometer and the coast (below) |
//! | `Position Data` | `v_speed` | [`Quantity::Speed`], up positive: [`FlightLog::vertical_speed_m_s`] | m/s, from the barometer and the coast (below) |
//! | `Position Data` | `fcb_latitude`, `fcb_longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`] | degrees, from their range and their fractions (below) |
//! | `Position Data` | `gps_time` | [`Quantity::TimeOfDay`] | the time of day it writes, in whatever time zone |
//! | `Position Data` | `gps_sats`, `fcb_state_number`, `ble_client` | [`Quantity::Number`] | as written; no source gives the state codes' meanings |
//! | `Position Data` | `temperature`, `fcb_battery_voltage`, `gps_alt`, `ground_speed`, `course_over_ground` | [`Quantity::Unrecognized`] | kept as written (below) |
//! | `Orientation` | `accel_x`, `accel_y`, `accel_z` | [`Quantity::Acceleration`] | m/s², from the pad's gravity and the barometer (below) |
//! | `Orientation` | `rot_vel_x`, `rot_vel_y`, `rot_vel_z` | [`Quantity::AngularRate`] | rad/s, from the board's turning and its tilt (below) |
//! | `Orientation` | `angle_vertical` | [`Quantity::Angle`] | degrees, from its range and the accelerometer (below) |
//! | `Orientation` | `orientation_quaternion[i]`, `fcb_state_number` | [`Quantity::Number`] | a quaternion's parts have no unit |
//! | `Orientation` | `roll`, `pitch`, `yaw`, `orientation_rpy[i]`, `magnetic_field_*` | [`Quantity::Unrecognized`] | kept as written (below) |
//! | `Alt Info & Cfg` | `barometer_pressure`, `barometer_2_pressure` | [`Quantity::Pressure`] | atm, from their size and their step (below) |
//! | `Alt Info & Cfg` | `press_ref` | [`Quantity::Pressure`] | atm, from its size and the barometer's (below) |
//! | `Alt Info & Cfg` | `ground_elev`, `ground_temp`, `pitot_ducer_press` | [`Quantity::Unrecognized`] | kept as written (below) |
//! | `Pyro Info` | `pyro_continuity[i]`, `pyro_status[i]`, `flash_usage` | [`Quantity::Number`] | as written |
//! | `Ground Station GPS`, `Groundstation Status` | `gs_baro` | [`Quantity::Pressure`] | atm, from its size and its step (below) |
//! | `Ground Station GPS`, `Groundstation Status` | `gs_lat`, `gs_lon` | [`Quantity::Latitude`], [`Quantity::Longitude`] | degrees, as the board's position |
//! | `Ground Station GPS`, `Groundstation Status` | `gs_alt`, `gs_temp` | [`Quantity::Unrecognized`] | kept as written (below) |
//! | `GPS Passthrough` | `Latitude`, `Longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`] | degrees, as the board's position |
//! | `GPS Passthrough` | `Satellites`, `Hdop` | [`Quantity::Number`] | as written |
//! | `GPS Passthrough` | `GPS Alt` | [`Quantity::Unrecognized`] | kept as written (below) |
//!
//! The line cutters' packets (`Line Cutter Data`, `Line Cutter Vars`) and `CLI Data` hold fields no
//! source describes: each is kept as written. The board and ground station packets have no
//! events: no source gives the state codes' or pyro flags' meanings.
//!
//! **The clock.** A board packet's `time_stamp_ms` is the board's clock, in milliseconds by its
//! name. The ground station's time of day on each line is a second clock, of known unit, that the
//! board's is checked against: the gap between them, the time the ground station received a packet
//! less the time the board stamped it, holds steady to within the radio's latency, under a second
//! in the sample files, and steps where the board restarted and its clock began again. The reader
//! cuts the packets, in the file's order, into stretches where that gap stays within 2 s of the
//! stretch's first; over the stretch spanning most of the ground station's clock, at least 20 s,
//! the board's clock must advance 0.8 to 1.25 s for each second of the ground station's, where a
//! clock in seconds, centiseconds or microseconds would be 10 or more times off (in the sample
//! files, within 0.4% of 1). Then each packet's time is the board's stamp placed on the ground
//! station's clock by its stretch's median gap, from the `RUN START` line's time: the board's clock
//! for the steps between samples, the ground station's to put stretches after a restart in order.
//! Otherwise every packet takes the time the ground station received it, the board's clocks are
//! kept as written as channels, and a note says why. A packet without a board stamp, as the ground
//! station's own `Ground Station GPS`, takes the time it was received. A line's time of day before
//! the line above's by more than 12 hours is taken to be the next day.
//!
//! **Units decided by two checks.** A unit a field's name doesn't state is read only where two
//! checks that don't depend on each other agree, each note saying which and with what evidence;
//! otherwise the field is kept as written, [`Quantity::Unrecognized`], with a note:
//!
//! - *The pressures* (`barometer_pressure`, `barometer_2_pressure`, `gs_baro`) are read in
//!   atmospheres, × 101,325 Pa, by their size and their step. Size: the first value lies within
//!   0.4935 to 1.1, a pad's pressure (50,000 to 110,000 Pa, the bounds [`crate::rrc3`] argues
//!   for) in atmospheres or in bars, and in no other unit a barometer is written in (pascals,
//!   hectopascals, kilopascals, psi or inches of mercury are 10 or more times off). Step: the board
//!   reads whole pascals and divides by 101,325, so each value times 101,325 lies within the
//!   value's own rounding of a whole number: half a unit in the last place of the 32-bit float the
//!   board sends (2⁻²⁴ near 1, so 0.003 to 0.006 Pa), or of the 64-bit float where the value isn't a
//!   32-bit one, as `gs_baro`'s aren't. Atmospheres are taken where at least 99% of at least 10
//!   distinct non-zero values pass at 101,325 and at most 10% pass at 100,000, the step bars
//!   would leave; a value off any such step passes either by chance about once in a hundred. In
//!   the sample files every value passes at 101,325 and at most 1.6% at 100,000. The pad's
//!   pressure altitude against the GPS isn't a check: the weather moves it as far as atmospheres
//!   and bars are apart, 110 m near sea level.
//! - *`press_ref`*, the board's reference pressure, is read in atmospheres where its size passes
//!   as above and its first value lies within 0.5% of `barometer_pressure`'s in the same packet,
//!   read in atmospheres: atmospheres and bars are 1.3% apart, and the two are read at one place
//!   and time, so no weather comes between them (within 0.08% in the sample files). It is an
//!   average, not one reading, so it has no step to check.
//! - *`fcb_altitude`* is read in meters by the barometer and the coast. The barometer: from
//!   `barometer_pressure`'s first value, on the pad, to its lowest, the climb in the U.S. Standard
//!   Atmosphere, 1976 ([`hpr_atmos::ussa76`]), against `fcb_altitude`'s between the `Position
//!   Data` packets nearest those two, within 0.5 s, must lie within 25%; feet, 3.28 times off,
//!   can't, and a climb beyond 25% refuses the file ([`LogError::Unit`]). The coast: an upward
//!   coast slows at standard gravity and more, as drag adds to it, so from the later sample of the
//!   column's fastest step (of those at most 1 s long), the end of the boost, to its highest
//!   value, which leaves out a parachute opened at apogee, the least-squares parabola's
//!   deceleration must lie within a factor of 1.8 of 9.806 65 m/s² either way, at least 5 samples
//!   across at least 1 s; read in meters, a column in feet would slow 3.28 g or more (1.8² =
//!   3.24). A parachute opened before apogee, or a coast too short to fit, fails the check and
//!   keeps the column; it can't make feet pass. The barometer checks the
//!   column against the pressure and the coast against gravity and the clock, so neither depends
//!   on the other. A climb under 10 m, no pressure read in atmospheres, or a coast too short or
//!   outside the band keeps `fcb_altitude` as written, the record's altitude track empty. In the
//!   sample files the column climbs 1.006 to 1.090 times the barometer and its coast slows 1.08
//!   to 1.28 g.
//! - *`v_speed`* is read in meters per second by the barometer and the coast, neither of which
//!   looks at `fcb_altitude`, whose unit is as unknown. The barometer: over its ascent, from its
//!   lowest pressure back to where the pressure last began to fall, the climb in the standard
//!   atmosphere must lie within 25% of the speed's integral over the same times, at least 10 m,
//!   the speed taken as a straight line between samples, across gaps in the reception too, as a
//!   coast's speed falls nearly in a line (the longest step bridged is named in the note; 7.1 s
//!   in one sample file). The coast: from its highest value to its first at or below 0 after it,
//!   the least-squares line's deceleration must lie within a factor of 1.8 of 9.806 65 m/s², at
//!   least 5 samples across at least 1 s. In the sample files the climb is 0.909 to 1.004 times
//!   the integral and the coast slows 1.00 to 1.47 g. Otherwise it is kept as written.
//! - *The positions* (`fcb_latitude` and `fcb_longitude`, `gs_lat` and `gs_lon`, `Latitude` and
//!   `Longitude`), each pair decided together as one fix, are read in degrees by their range and
//!   their fractions. A position of exactly (0, 0) is the receiver's mark for no fix, a gap, as
//!   [`crate::altos`] reads AltOS's, counted in a note. Range: every fix lies within ±90° and
//!   ±180°, and most fixes lie beyond ±π/2 or ±π, which radians can't, nor 10⁻⁷ degrees or NMEA's
//!   `ddmm.mmmm` written as one number, which lie far beyond ±180. Fractions: in most fixes the
//!   latitude's or the longitude's part after the whole degrees is 0.6 or more, which degrees and
//!   minutes run together, `dd.mmmm`, can't be. Most, so that one stray fix decides nothing (in
//!   the sample files every fix passes both). A pair with no fix, or failing either check, is kept
//!   as written.
//! - *The accelerometer* (`accel_x`, `accel_y`, `accel_z`) is read in meters per second squared
//!   by the pad's gravity and the barometer. The pad's gravity, as [`crate::sillygoose`] checks
//!   it: the time the three axes' magnitude lies within 10% of 9.806 65, each sample counting
//!   until the next but at most 1 s, is at least 1 s and twice the time it lies within 10% of 1.
//!   The barometer, as [`crate::aerospacenu_fcb`] checks the board's own CSV: from the launch,
//!   the first sample of at least 3 times the median magnitude of the 10 before it (at least 3),
//!   the next two at least 2 times, the specific force along those samples' median direction,
//!   less 9.806 65, is integrated read in m/s², by trapezoids over steps of at most 0.5 s, to a
//!   speed and a climb until the speed falls to 0; at each pressure the integrated climb passes
//!   10 m by, within 0.5 s, the barometer's climb from its last pressure at or before the launch,
//!   in the standard atmosphere, over the integrated climb; the median of at least 3 must lie
//!   within a factor of 1.8 of 1. Read in g the speed falls to 0 at once; in feet per second
//!   squared the pad reads 3.28 g. In the sample files 3 of the 4 accelerometers pass both, the
//!   median 0.997 to 1.090, and one reads 0 throughout.
//! - *`angle_vertical`* is read in degrees by its range and the accelerometer: every value lies
//!   within 0 to 180° and one beyond π, as no angle in radians from vertical can; and at the
//!   samples where the accelerometer, read in m/s², reads within 5% of 1 g, at least 10, the median
//!   difference between the column and the angle of the accelerometer's reading from the board's z
//!   axis is at most 2° read in degrees and more than 2° read in radians (in the sample files,
//!   0.11° to 0.70° in degrees, 34° to 416° in radians).
//! - *The gyroscope* (`rot_vel_x`, `rot_vel_y`, `rot_vel_z`) is read in radians per second by the
//!   board's turning and its tilt. Turning, as [`crate::sillygoose`] checks it: over stretches of
//!   10 steps at most 0.3 s apart with the accelerometer within 15% of 1 g, the angle its reading
//!   turns through and the gyroscope's turn across it, |ω × â| dt; radians per second are borne
//!   out where the gyroscope's turn is within a quarter to 4 times the accelerometer's, given at
//!   least 3 stretches and 0.2 rad (0.78 to 1.23 in the three sample files whose accelerometer
//!   reads; in degrees per second it would be 57 times less). Tilt: the angle between the board's z axis and vertical can change no
//!   faster than the board turns across that axis, √(ω_x² + ω_y²). Over windows of at most 2 s, of
//!   steps at most 0.5 s, in which `angle_vertical`, read in degrees, moves at least 10°, at least
//!   3 of them, the tilt's change must be at most twice the gyroscope's turn: read in degrees per
//!   second the gyroscope turns 7.6 to 9.3 times too little for the tilt in the sample files, so
//!   degrees per second are refuted, and in radians per second the tilt's change is 0.13 to 0.16
//!   of the turn. A board spinning about its axis turns more than it tilts, so this check refutes
//!   too slow a unit and never too fast a one: degrees per second can't be read.
//!
//! Kept as written, each with a note: the temperatures (`temperature`, `gs_temp`, `ground_temp`),
//! whose names state no unit and which read as a bay's or the day's in degrees Celsius or
//! Fahrenheit alike; `fcb_battery_voltage`, `ground_speed`, `course_over_ground`, `ground_elev`
//! and `pitot_ducer_press`, whose names state no unit that anything in the file checks (in the
//! sample files `pitot_ducer_press` follows `gps_alt` where it isn't 0); the GPS altitudes
//! (`gps_alt`, `gs_alt`, `GPS Alt`), whose rise against the barometer's would tell meters from feet
//! but which no second check independent of the barometer reaches; `roll`, `pitch`, `yaw` and
//! `orientation_rpy`, which lie within ±π as radians would, but which nothing else in the file
//! checks; and the magnetometer, for which the record has no quantity.
//!
//! **The primary track.** The record's altitude is `fcb_altitude`, the board's estimate above its
//! zero, the pad (within 4 m of 0 on the pad in the sample files), where it is read in meters; its
//! vertical speed is `v_speed`. The track takes the `Position Data` samples in time order, the
//! first at each time; a note counts those put in order. The record states no temperature or
//! battery: those fields' units aren't settled.
//!
//! **Values.** A value too large to hold once converted to its unit, such as a pressure of 10³⁰⁸
//! atmospheres, refuses the file, naming its line ([`LogError::Syntax`]), so that every channel holds
//! values in SI or gaps.
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md

use std::collections::{BTreeSet, HashMap, HashSet};
use std::f64::consts::PI;

use hpr_atmos::ussa76::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, Notes, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "AerospaceNU ground station text";

/// The logger, as the record names it.
const LOGGER: &str = "AerospaceNU flight computer board, by its ground station";

/// The first line's opening.
const RUN_START: &str = "RUN START";

/// The barometric altitude's step, m: the board's barometers read whole pascals (every pressure in
/// the sample files is a whole number of pascals divided by 101,325, module docs), and
/// `fcb_altitude`, the board's filtered estimate sent as a 32-bit float, has no step of its own,
/// so the pascal's, `1 / (ρ₀ g₀)` with the International Standard Atmosphere's ρ₀ = 1.225 kg/m³ (ISO
/// 2533:1975) and g₀ = 9.806 65 m/s², 0.083 m at sea level, is its resolution near the ground.
pub const ALTITUDE_STEP_M: f64 = 1.0 / (1.225 * 9.806_65);

/// The longest line read, bytes: the ground station's longest packet in the sample files is under
/// a kilobyte.
pub const MAX_LINE_BYTES: usize = 65_536;

/// The most lists or dictionaries one inside another in a packet.
const MAX_DEPTH: usize = 8;

/// The most items one list or dictionary may hold.
const MAX_ITEMS: usize = 1_024;

/// The most channels a record keeps: the sample files have under 150.
pub const MAX_CHANNELS: usize = 1_024;

/// The longest packet or field name read, characters.
const MAX_NAME_CHARS: usize = 64;

/// The most names of text fields a note lists.
const MAX_LISTED: usize = 12;

/// Pascals in an atmosphere.
const ATM_PA: f64 = 101_325.0;

/// Pascals in a bar.
const BAR_PA: f64 = 100_000.0;

/// A pressure near the ground, Pa, the bounds [`crate::rrc3`] argues for a pad's.
const PAD_PA: (f64, f64) = (50_000.0, 110_000.0);

/// The share of a pressure's values that must lie on the 1/101,325 step for atmospheres.
const STEP_HOLDS: f64 = 0.99;

/// The most share of a pressure's values that may lie on the 1/100,000 step for atmospheres.
const STEP_FAILS: f64 = 0.10;

/// The fewest distinct non-zero values the step check decides on.
const STEP_LEAST_VALUES: usize = 10;

/// How far, as a share, `press_ref` may be from the barometer's reading in the same packet.
const REFERENCE_SHARE: f64 = 0.005;

/// How far, s, a packet's gap from the ground station's clock may be from its stretch's first.
const STRETCH_GAP_S: f64 = 2.0;

/// The least span, s of the ground station's clock, of the stretch the board's clock is checked on.
const CLOCK_SPAN_S: f64 = 20.0;

/// The rates of the board's clock against the ground station's that bear out milliseconds.
const CLOCK_RATE: (f64, f64) = (0.8, 1.25);

/// How far apart in time, s, a `Position Data` sample may be from a pressure's to stand beside it.
const NEAREST_S: f64 = 0.5;

/// The least climb, m, from which the pressure checks the altitude's unit.
const CHECK_CLIMB_M: f64 = 10.0;

/// How far, as a share, the altitude's climb may be from the pressure's, and the pressure's climb
/// from the speed's integral.
const MARGIN: f64 = 0.25;

/// How many times off, either way, a coast's deceleration or the accelerometer's climb may be
/// and bear meters out: 1.8² = 3.24 is under the 3.28 between feet and meters.
const GRAVITY_FACTOR: f64 = 1.8;

/// The longest step, s, between two `Position Data` samples that the search for the coast's
/// start takes: the board sends one about every 0.2 s; a longer step is a gap in the reception.
const COAST_STEP_S: f64 = 1.0;

/// The fewest samples a coast's fit takes.
const COAST_LEAST_SAMPLES: usize = 5;

/// The least time, s, a coast's fit spans.
const COAST_LEAST_S: f64 = 1.0;

/// The launch: the first accelerometer sample of at least this many times the pad's reading…
const LAUNCH_FORCE: f64 = 3.0;

/// …the next two of at least this many.
const LAUNCH_HELD: f64 = 2.0;

/// The most samples before a launch that its pad reading is the median of.
const PAD_SAMPLES: usize = 10;

/// The fewest samples before a launch that its pad reading is taken from.
const PAD_LEAST_SAMPLES: usize = 3;

/// The longest step, s, the accelerometer's integration bridges: the board sends about 5
/// samples a second.
const INTEGRATE_STEP_S: f64 = 0.5;

/// The fewest pressures the accelerometer's climb is weighed at.
const INTEGRATED_LEAST: usize = 3;

/// How far, as a share, the accelerometer's magnitude may lie from 1 g to count as reading it.
const GRAVITY_MARGIN: f64 = 0.1;

/// The least time, s, the file must read 1 g in a unit for that unit to be taken.
const GRAVITY_LEAST_S: f64 = 1.0;

/// How many times longer the taken unit's 1 g must last than the other's.
const GRAVITY_LEAD: f64 = 2.0;

/// The most time, s, one accelerometer sample counts for: the board sends about 5 a second.
const GRAVITY_STEP_MAX_S: f64 = 1.0;

/// How far, as a share, the accelerometer may read from 1 g at the samples `angle_vertical` is
/// checked on.
const TILT_GRAVITY_MARGIN: f64 = 0.05;

/// How far, degrees, `angle_vertical` may be from the accelerometer's tilt, as a median.
const TILT_AGREE_DEG: f64 = 2.0;

/// The fewest samples the tilt is compared on.
const TILT_LEAST_SAMPLES: usize = 10;

/// The longest step, s, of the turning check.
const TURN_STEP_S: f64 = 0.3;

/// The steps a stretch of the turning check spans.
const TURN_STEPS: usize = 10;

/// How far, as a share, the accelerometer may read from 1 g in a stretch of the turning check.
const TURN_GRAVITY_MARGIN: f64 = 0.15;

/// The least the accelerometer's direction must turn, rad, for the turning check to decide.
const TURN_LEAST_RAD: f64 = 0.2;

/// The fewest stretches for the turning check to decide.
const TURN_LEAST_STRETCHES: usize = 3;

/// How many times off, either way, the gyroscope's turn may be from the accelerometer's.
const TURN_FACTOR: f64 = 4.0;

/// The longest window, s, of the tilt check.
const TILT_WINDOW_S: f64 = 2.0;

/// The longest step, s, in a window of the tilt check.
const TILT_STEP_S: f64 = 0.5;

/// The least change, rad, of `angle_vertical` over a window for it to count: 10°.
const TILT_LEAST_RAD: f64 = 10.0 * PI / 180.0;

/// The fewest windows for the tilt check to decide.
const TILT_LEAST_WINDOWS: usize = 3;

/// How many times the gyroscope's turn the tilt's change may be in a unit not refuted: the
/// samples' 0.2 s steps blur both sides.
const TILT_FACTOR: f64 = 2.0;

/// Whether the text is the ground station's: its first line that isn't blank opens `RUN START`.
pub fn recognizes(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with(RUN_START))
}

// ---------------------------------------------------------------------------------------------
// The packets' Python literals.
// ---------------------------------------------------------------------------------------------

/// A Python literal, as the ground station writes a packet's fields.
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(f64),
    Bool(bool),
    Text(String),
    /// `None`.
    Nothing,
    List(Vec<Value>),
    Dict(Vec<(String, Value)>),
}

/// Parses a Python literal (module docs), the whole of `text` but spaces around it.
#[cfg(test)]
fn parse_literal(text: &str) -> Result<Value, String> {
    parse_from(text, 0)
}

/// Parses a Python literal, the whole of `line` from byte `start` but spaces around it; an error
/// names the place it found, as a character position in `line` counted from 1.
fn parse_from(line: &str, start: usize) -> Result<Value, String> {
    let mut parser = Parser {
        text: line,
        at: start,
    };
    let value = parser.value(0)?;
    parser.skip_space();
    if parser.at < line.len() {
        return Err(format!(
            "text after the literal, at character {} of the line",
            parser.place(parser.at)
        ));
    }
    Ok(value)
}

/// A walk through a line's bytes; structure is ASCII, so every place it stops is a character
/// boundary.
struct Parser<'a> {
    text: &'a str,
    at: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.text.as_bytes().get(self.at).copied()
    }

    /// The character position in the line, from 1, of the byte at `at`.
    fn place(&self, at: usize) -> usize {
        self.text
            .get(..at)
            .map_or(at, |before| before.chars().count())
            + 1
    }

    fn skip_space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t')) {
            self.at += 1;
        }
    }

    fn value(&mut self, depth: usize) -> Result<Value, String> {
        self.skip_space();
        match self.peek() {
            None => Err("the line ends inside the packet".to_owned()),
            Some(b'{') => self.dict(depth + 1),
            Some(b'[') => self.list(depth + 1, b']'),
            Some(b'(') => self.list(depth + 1, b')'),
            Some(quote @ (b'\'' | b'"')) => self.string(quote).map(Value::Text),
            Some(b'-' | b'+' | b'.' | b'0'..=b'9') => self.number(),
            Some(byte) if byte.is_ascii_alphabetic() => self.word(),
            Some(_) => Err(format!(
                "an unexpected character at character {} of the line",
                self.place(self.at)
            )),
        }
    }

    /// The opening of a list or dictionary, refused past [`MAX_DEPTH`].
    fn open(&mut self, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH {
            return Err(format!(
                "lists or dictionaries nested more than {MAX_DEPTH} deep"
            ));
        }
        self.at += 1;
        Ok(())
    }

    /// After an item: a comma, or the closing byte, left for the caller.
    fn after_item(&mut self, close: u8) -> Result<(), String> {
        self.skip_space();
        match self.peek() {
            Some(b',') => {
                self.at += 1;
                Ok(())
            }
            Some(byte) if byte == close => Ok(()),
            _ => Err(format!(
                "no comma or `{}` after an item, at character {} of the line",
                close as char,
                self.place(self.at)
            )),
        }
    }

    fn dict(&mut self, depth: usize) -> Result<Value, String> {
        self.open(depth)?;
        let mut entries: Vec<(String, Value)> = Vec::new();
        let mut keys: HashSet<String> = HashSet::new();
        loop {
            self.skip_space();
            if self.peek() == Some(b'}') {
                self.at += 1;
                return Ok(Value::Dict(entries));
            }
            if entries.len() >= MAX_ITEMS {
                return Err(format!("a dictionary of more than {MAX_ITEMS} items"));
            }
            let Value::Text(key) = self.value(depth)? else {
                return Err("a key that isn't text".to_owned());
            };
            if !keys.insert(key.clone()) {
                return Err(format!("the key {:?} given twice", short(&key)));
            }
            self.skip_space();
            if self.peek() != Some(b':') {
                return Err(format!(
                    "no `:` after a key, at character {} of the line",
                    self.place(self.at)
                ));
            }
            self.at += 1;
            let value = self.value(depth)?;
            entries.push((key, value));
            self.after_item(b'}')?;
        }
    }

    fn list(&mut self, depth: usize, close: u8) -> Result<Value, String> {
        self.open(depth)?;
        let mut items = Vec::new();
        loop {
            self.skip_space();
            if self.peek() == Some(close) {
                self.at += 1;
                return Ok(Value::List(items));
            }
            if items.len() >= MAX_ITEMS {
                return Err(format!("a list of more than {MAX_ITEMS} items"));
            }
            items.push(self.value(depth)?);
            self.after_item(close)?;
        }
    }

    /// A word: `True`, `False`, `None`, `nan` or `inf`.
    fn word(&mut self) -> Result<Value, String> {
        let start = self.at;
        while self
            .peek()
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            self.at += 1;
        }
        match &self.text[start..self.at] {
            "True" => Ok(Value::Bool(true)),
            "False" => Ok(Value::Bool(false)),
            "None" => Ok(Value::Nothing),
            "nan" => Ok(Value::Number(f64::NAN)),
            "inf" => Ok(Value::Number(f64::INFINITY)),
            word => Err(format!("the word {:?}, no Python literal", short(word))),
        }
    }

    /// A number: an optional sign, then `inf`, `nan`, or digits with a point and an exponent as
    /// Python writes them.
    fn number(&mut self) -> Result<Value, String> {
        let start = self.at;
        let negative = self.peek() == Some(b'-');
        if matches!(self.peek(), Some(b'-' | b'+')) {
            self.at += 1;
        }
        if self.peek().is_some_and(|byte| byte.is_ascii_alphabetic()) {
            return match self.word()? {
                Value::Number(value) if negative => Ok(Value::Number(-value)),
                Value::Number(value) => Ok(Value::Number(value)),
                _ => Err("a sign before a word that isn't a number".to_owned()),
            };
        }
        let mut digits = self.digits();
        if self.peek() == Some(b'.') {
            self.at += 1;
            digits += self.digits();
        }
        if digits == 0 {
            return Err(format!(
                "a number without digits, at character {} of the line",
                self.place(start)
            ));
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.at += 1;
            if matches!(self.peek(), Some(b'-' | b'+')) {
                self.at += 1;
            }
            if self.digits() == 0 {
                return Err(format!(
                    "an exponent without digits, at character {} of the line",
                    self.place(start)
                ));
            }
        }
        if self
            .peek()
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.')
        {
            return Err(format!(
                "a number Python wouldn't write, at character {} of the line",
                self.place(start)
            ));
        }
        self.text[start..self.at]
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| {
                format!(
                    "a number that doesn't parse, at character {} of the line",
                    self.place(start)
                )
            })
    }

    fn digits(&mut self) -> usize {
        let start = self.at;
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.at += 1;
        }
        self.at - start
    }

    /// Text in quotes, with Python's escapes.
    fn string(&mut self, quote: u8) -> Result<String, String> {
        self.at += 1;
        let mut out = String::new();
        loop {
            let Some(c) = self.text[self.at..].chars().next() else {
                return Err("a text in quotes runs to the line's end".to_owned());
            };
            self.at += c.len_utf8();
            if c == quote as char {
                return Ok(out);
            }
            if c != '\\' {
                out.push(c);
                continue;
            }
            let Some(escaped) = self.text[self.at..].chars().next() else {
                return Err("a text in quotes runs to the line's end".to_owned());
            };
            self.at += escaped.len_utf8();
            match escaped {
                '\\' | '\'' | '"' => out.push(escaped),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'a' => out.push('\u{7}'),
                'b' => out.push('\u{8}'),
                'f' => out.push('\u{c}'),
                'v' => out.push('\u{b}'),
                'x' => out.push(self.code(2, 16)?),
                'u' => out.push(self.code(4, 16)?),
                'U' => out.push(self.code(8, 16)?),
                '0'..='7' => {
                    self.at -= 1;
                    out.push(self.code_up_to(3, 8));
                }
                other => {
                    out.push('\\');
                    out.push(other);
                }
            }
        }
    }

    /// A character from exactly `count` digits in `radix`; one no character is, the replacement.
    fn code(&mut self, count: usize, radix: u32) -> Result<char, String> {
        let digits = self.text.get(self.at..self.at + count).unwrap_or("");
        if digits.len() != count || !digits.chars().all(|c| c.is_digit(radix)) {
            return Err(format!(
                "an escape without its {count} digits, at character {} of the line",
                self.place(self.at)
            ));
        }
        self.at += count;
        Ok(u32::from_str_radix(digits, radix)
            .ok()
            .and_then(char::from_u32)
            .unwrap_or(char::REPLACEMENT_CHARACTER))
    }

    /// A character from up to `count` digits in `radix`, at least one.
    fn code_up_to(&mut self, count: usize, radix: u32) -> char {
        let start = self.at;
        while self.at - start < count
            && self
                .peek()
                .is_some_and(|byte| (byte as char).is_digit(radix))
        {
            self.at += 1;
        }
        u32::from_str_radix(&self.text[start..self.at], radix)
            .ok()
            .and_then(char::from_u32)
            .unwrap_or(char::REPLACEMENT_CHARACTER)
    }
}

/// A time of day `HH:MM:SS` or `HH:MM:SS.ffffff`, s after midnight.
fn time_of_day(text: &str) -> Option<f64> {
    let mut parts = text.split(':');
    let (hours, minutes, seconds) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let whole = |part: &str, below: u32| {
        (part.len() == 2 && part.bytes().all(|b| b.is_ascii_digit()))
            .then(|| part.parse::<u32>().ok())
            .flatten()
            .filter(|value| *value < below)
    };
    let (seconds_whole, fraction) = seconds.split_once('.').unwrap_or((seconds, ""));
    if !fraction.bytes().all(|b| b.is_ascii_digit()) || fraction.len() > 9 {
        return None;
    }
    let seconds = f64::from(whole(seconds_whole, 60)?)
        + if fraction.is_empty() {
            0.0
        } else {
            format!("0.{fraction}").parse::<f64>().ok()?
        };
    Some(f64::from(whole(hours, 24)?) * 3_600.0 + f64::from(whole(minutes, 60)?) * 60.0 + seconds)
}

// ---------------------------------------------------------------------------------------------
// What each field is.
// ---------------------------------------------------------------------------------------------

const POSITION: &str = "Position Data";
const ORIENTATION: &str = "Orientation";
const ALT_INFO: &str = "Alt Info & Cfg";
const GROUND_STATION: [&str; 2] = ["Ground Station GPS", "Groundstation Status"];
const PASSTHROUGH: &str = "GPS Passthrough";

/// Why a field the reader knows is kept as written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kept {
    Temperature,
    /// A name that states no unit and nothing in the file checks.
    Unstated,
    GnssAltitude,
    Euler,
    Magnetic,
}

/// What a channel is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// A board packet's `time_stamp_ms`: a channel only where the clock isn't read.
    Clock,
    Number,
    TimeOfDay,
    Barometer,
    Reference,
    Altitude,
    VerticalSpeed,
    Latitude,
    Longitude,
    Accel(usize),
    Gyro(usize),
    Tilt,
    Kept(Kept),
    Unknown,
}

/// What a field of a packet is; `listed` for an element of a list.
fn role_of(packet: &str, field: &str, listed: bool) -> Role {
    let ground_station = GROUND_STATION.contains(&packet);
    match field {
        "time_stamp_ms" if !listed => Role::Clock,
        "radio_id"
        | "lqi"
        | "software_version"
        | "board_serial_number"
        | "rssi"
        | "flash_usage"
        | "gps_sats"
        | "ble_client"
        | "fcb_state_number"
            if !listed =>
        {
            Role::Number
        }
        "pyro_continuity" | "pyro_status" | "orientation_quaternion" if listed => Role::Number,
        "orientation_rpy" if listed && packet == ORIENTATION => Role::Kept(Kept::Euler),
        _ if listed => Role::Unknown,
        "gps_time" if packet == POSITION => Role::TimeOfDay,
        "fcb_altitude" if packet == POSITION => Role::Altitude,
        "v_speed" if packet == POSITION => Role::VerticalSpeed,
        "fcb_latitude" if packet == POSITION => Role::Latitude,
        "fcb_longitude" if packet == POSITION => Role::Longitude,
        "temperature" if packet == POSITION => Role::Kept(Kept::Temperature),
        "gps_alt" if packet == POSITION => Role::Kept(Kept::GnssAltitude),
        "fcb_battery_voltage" | "ground_speed" | "course_over_ground" if packet == POSITION => {
            Role::Kept(Kept::Unstated)
        }
        "accel_x" if packet == ORIENTATION => Role::Accel(0),
        "accel_y" if packet == ORIENTATION => Role::Accel(1),
        "accel_z" if packet == ORIENTATION => Role::Accel(2),
        "rot_vel_x" if packet == ORIENTATION => Role::Gyro(0),
        "rot_vel_y" if packet == ORIENTATION => Role::Gyro(1),
        "rot_vel_z" if packet == ORIENTATION => Role::Gyro(2),
        "angle_vertical" if packet == ORIENTATION => Role::Tilt,
        "roll" | "pitch" | "yaw" if packet == ORIENTATION => Role::Kept(Kept::Euler),
        "magnetic_field_x" | "magnetic_field_y" | "magnetic_field_z" if packet == ORIENTATION => {
            Role::Kept(Kept::Magnetic)
        }
        "barometer_pressure" | "barometer_2_pressure" if packet == ALT_INFO => Role::Barometer,
        "press_ref" if packet == ALT_INFO => Role::Reference,
        "ground_temp" if packet == ALT_INFO => Role::Kept(Kept::Temperature),
        "ground_elev" | "pitot_ducer_press" if packet == ALT_INFO => Role::Kept(Kept::Unstated),
        "gs_baro" if ground_station => Role::Barometer,
        "gs_lat" if ground_station => Role::Latitude,
        "gs_lon" if ground_station => Role::Longitude,
        "gs_temp" if ground_station => Role::Kept(Kept::Temperature),
        "gs_alt" if ground_station => Role::Kept(Kept::GnssAltitude),
        "Latitude" if packet == PASSTHROUGH => Role::Latitude,
        "Longitude" if packet == PASSTHROUGH => Role::Longitude,
        "Satellites" | "Hdop" if packet == PASSTHROUGH => Role::Number,
        "GPS Alt" if packet == PASSTHROUGH => Role::Kept(Kept::GnssAltitude),
        _ => Role::Unknown,
    }
}

// ---------------------------------------------------------------------------------------------
// Reading.
// ---------------------------------------------------------------------------------------------

/// A packet's place on the clocks.
#[derive(Debug, Clone, Copy)]
struct Slot {
    line: usize,
    /// The ground station's time, s after the `RUN START` line's.
    received_s: f64,
    /// The board's stamp, ms, if the packet has one.
    board_ms: Option<f64>,
    /// The record's time, s, set once the clock is decided.
    time_s: f64,
}

/// A channel as it is read.
struct Build {
    column: String,
    packet: String,
    role: Role,
    quantity: Quantity,
    scale: f64,
    /// Each sample's packet, in the file's order.
    slots: Vec<usize>,
    /// The values as written, `NaN` for a gap.
    values: Vec<f64>,
}

impl Build {
    fn quoted(&self) -> String {
        format!("`{}`", short(&self.column))
    }

    /// Keeps the channel as written.
    fn keep(&mut self) {
        self.quantity = Quantity::Unrecognized;
        self.scale = 1.0;
    }

    /// The lowest and highest of the values, if any is a number.
    fn range(&self) -> Option<(f64, f64)> {
        let mut finite = self.values.iter().copied().filter(|v| v.is_finite());
        let first = finite.next()?;
        Some(finite.fold((first, first), |(lo, hi), v| (lo.min(v), hi.max(v))))
    }
}

/// What the reader counts as it goes.
#[derive(Default)]
struct Counts {
    corrupt: usize,
    first_corrupt: Option<usize>,
    messages: usize,
    first_message: Option<(usize, String)>,
    unparsed: usize,
    first_unparsed: Option<(usize, String)>,
    other: usize,
    first_other: Option<usize>,
    long: usize,
    first_long: Option<usize>,
    runs: usize,
    first_run: Option<usize>,
    gaps: usize,
    first_gap: Option<usize>,
    left_out: usize,
    first_left_out: Option<usize>,
    text_fields: BTreeSet<String>,
    nested_fields: BTreeSet<String>,
}

/// The reader's state.
#[derive(Default)]
struct Reading {
    slots: Vec<Slot>,
    channels: Vec<Build>,
    by_name: HashMap<String, usize>,
    counts: Counts,
}

impl Reading {
    /// Adds a value to the channel `packet.field`, made if new.
    fn push(&mut self, packet: &str, field: &str, listed: bool, value: f64, slot: usize) {
        let line = self.slots[slot].line;
        let column = format!("{packet}.{field}");
        let index = if let Some(&index) = self.by_name.get(&column) {
            index
        } else {
            if self.channels.len() >= MAX_CHANNELS {
                self.counts.left_out += 1;
                self.counts.first_left_out.get_or_insert(line);
                return;
            }
            // A list's item is `key[i]`, and takes its role by `key`; any other name is the
            // field's own, a `[` in it as well.
            let base = if listed {
                field.rsplit_once('[').map_or(field, |(key, _)| key)
            } else {
                field
            };
            let role = role_of(packet, base, listed);
            let quantity = match role {
                Role::Number => Quantity::Number,
                Role::TimeOfDay => Quantity::TimeOfDay,
                // Until a check decides it.
                _ => Quantity::Unrecognized,
            };
            self.channels.push(Build {
                column: column.clone(),
                packet: packet.to_owned(),
                role,
                quantity,
                scale: 1.0,
                slots: Vec::new(),
                values: Vec::new(),
            });
            self.by_name.insert(column, self.channels.len() - 1);
            self.channels.len() - 1
        };
        let value = if value.is_finite() {
            value
        } else {
            self.counts.gaps += 1;
            self.counts.first_gap.get_or_insert(line);
            f64::NAN
        };
        self.channels[index].slots.push(slot);
        self.channels[index].values.push(value);
    }

    /// Adds a field's values.
    fn field(&mut self, packet: &str, key: &str, value: &Value, slot: usize) {
        if key.chars().count() > MAX_NAME_CHARS {
            self.counts.left_out += 1;
            let line = self.slots[slot].line;
            self.counts.first_left_out.get_or_insert(line);
            return;
        }
        match value {
            Value::Number(number) => self.push(packet, key, false, *number, slot),
            Value::Bool(flag) => self.push(packet, key, false, f64::from(u8::from(*flag)), slot),
            Value::Nothing => self.push(packet, key, false, f64::NAN, slot),
            Value::Text(text) => {
                let number = match key {
                    "rssi" | "Hdop" => text.split_whitespace().next().and_then(|n| n.parse().ok()),
                    "gps_time" => text.split_once(' ').and_then(|(_, time)| time_of_day(time)),
                    _ => {
                        if self.counts.text_fields.len() < MAX_LISTED {
                            self.counts.text_fields.insert(key.to_owned());
                        }
                        return;
                    }
                };
                self.push(packet, key, false, number.unwrap_or(f64::NAN), slot);
            }
            Value::List(items) => {
                let flat = items
                    .iter()
                    .all(|item| matches!(item, Value::Number(_) | Value::Bool(_) | Value::Nothing));
                if !flat {
                    if self.counts.nested_fields.len() < MAX_LISTED {
                        self.counts.nested_fields.insert(key.to_owned());
                    }
                    return;
                }
                for (index, item) in items.iter().enumerate() {
                    let value = match item {
                        Value::Number(number) => *number,
                        Value::Bool(flag) => f64::from(u8::from(*flag)),
                        _ => f64::NAN,
                    };
                    self.push(packet, &format!("{key}[{index}]"), true, value, slot);
                }
            }
            Value::Dict(_) => {
                if self.counts.nested_fields.len() < MAX_LISTED {
                    self.counts.nested_fields.insert(key.to_owned());
                }
            }
        }
    }

    /// The channel with a role in a packet, if the file has it.
    fn find(&self, packet: &str, role: Role) -> Option<usize> {
        self.channels
            .iter()
            .position(|c| c.packet == packet && c.role == role)
    }

    /// The channel named `column`, if the file has it.
    fn named(&self, column: &str) -> Option<usize> {
        self.by_name.get(column).copied()
    }

    /// The samples two channels share, by packet: each packet and the two values.
    fn paired(&self, a: usize, b: usize) -> Vec<(usize, f64, f64)> {
        let (a, b) = (&self.channels[a], &self.channels[b]);
        let (mut i, mut j) = (0, 0);
        let mut out = Vec::new();
        while i < a.slots.len() && j < b.slots.len() {
            match a.slots[i].cmp(&b.slots[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    out.push((a.slots[i], a.values[i], b.values[j]));
                    i += 1;
                    j += 1;
                }
            }
        }
        out
    }

    /// The samples three channels share, by packet.
    fn tripled(&self, at: [usize; 3]) -> Vec<(usize, [f64; 3])> {
        let first = self.paired(at[0], at[1]);
        let third = &self.channels[at[2]];
        let mut out = Vec::new();
        let mut k = 0;
        for (slot, x, y) in first {
            while k < third.slots.len() && third.slots[k] < slot {
                k += 1;
            }
            if k < third.slots.len() && third.slots[k] == slot {
                out.push((slot, [x, y, third.values[k]]));
            }
        }
        out
    }

    fn time(&self, slot: usize) -> f64 {
        self.slots[slot].time_s
    }
}

/// Reads the ground station's text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't a `RUN START` line;
/// [`LogError::Syntax`] if that line gives no time of day, or a value is too large to hold once
/// converted to its unit; [`LogError::NoData`] if no packet is read; [`LogError::Unit`] if
/// `fcb_altitude`'s climb is more than 25% off the barometer's (module docs).
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text);
    let mut start = None;
    for (number, line) in lines.by_ref() {
        let line = line.trim();
        if !line.is_empty() {
            start = Some((number, line));
            break;
        }
    }
    let Some((start_line, first)) = start else {
        return Err(not_this("the file is empty".to_owned()));
    };
    let Some(rest) = first.strip_prefix(RUN_START) else {
        return Err(not_this(format!(
            "its first line, {:?}, isn't the ground station's `RUN START` line",
            short(first)
        )));
    };
    let Some(start_s) = rest.split_whitespace().nth(1).and_then(time_of_day) else {
        return Err(syntax(
            start_line,
            format!(
                "the `RUN START` line, {:?}, gives no date and time of day as `YYYY-MM-DD \
                 HH:MM:SS`",
                short(first)
            ),
        ));
    };

    let mut reading = Reading::default();
    let mut previous_s = start_s;
    let mut day_s = 0.0;
    for (number, line) in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() > MAX_LINE_BYTES {
            reading.counts.long += 1;
            reading.counts.first_long.get_or_insert(number);
            continue;
        }
        if line.starts_with(RUN_START) {
            reading.counts.runs += 1;
            reading.counts.first_run.get_or_insert(number);
            continue;
        }
        let Some((clock, rest)) = line
            .split_once(": ")
            .and_then(|(clock, rest)| Some((time_of_day(clock)?, rest)))
        else {
            reading.counts.other += 1;
            reading.counts.first_other.get_or_insert(number);
            continue;
        };
        // A time of day more than 12 hours before the last is the next day's.
        if clock + day_s < previous_s - 43_200.0 {
            day_s += 86_400.0;
        }
        previous_s = clock + day_s;
        let received_s = previous_s - start_s;
        let Some((name, body)) = rest
            .split_once(" {")
            .map(|(name, body)| (name.trim(), body))
        else {
            reading.counts.messages += 1;
            reading
                .counts
                .first_message
                .get_or_insert_with(|| (number, short(rest)));
            continue;
        };
        let parsed = if name.is_empty() || name.chars().count() > MAX_NAME_CHARS {
            Err("no packet name, or one longer than 64 characters".to_owned())
        } else {
            // `body` follows the line's ` {`, so the literal opens one byte before it.
            parse_from(line, line.len() - body.len() - 1)
        };
        let fields = match parsed {
            Ok(Value::Dict(fields)) => fields,
            // A literal opening with `{` is a dictionary or an error; this arm is never taken.
            Ok(_) => Vec::new(),
            Err(why) => {
                reading.counts.unparsed += 1;
                reading.counts.first_unparsed.get_or_insert((number, why));
                continue;
            }
        };
        let crc = fields.iter().find(|(key, _)| key == "crc");
        if crc.is_some_and(|(_, value)| *value != Value::Text("Good".to_owned())) {
            reading.counts.corrupt += 1;
            reading.counts.first_corrupt.get_or_insert(number);
            continue;
        }
        let board_ms = fields.iter().find_map(|(key, value)| match value {
            Value::Number(ms) if key == "time_stamp_ms" && ms.is_finite() => Some(*ms),
            _ => None,
        });
        let slot = reading.slots.len();
        reading.slots.push(Slot {
            line: number,
            received_s,
            board_ms,
            time_s: received_s,
        });
        for (key, value) in &fields {
            if key != "crc" {
                reading.field(name, key, value, slot);
            }
        }
    }
    if reading.slots.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes: Vec<String> = Vec::new();
    let clock_read = place_on_clock(&mut reading.slots, &mut notes);
    decide_pressures(&mut reading, &mut notes);
    decide_reference(&mut reading, &mut notes);
    decide_altitude(&mut reading, start_line, &mut notes)?;
    decide_speed(&mut reading, &mut notes);
    let no_fix = decide_positions(&mut reading, &mut notes);
    let one_g = decide_accelerometer(&mut reading, &mut notes);
    let tilt = decide_tilt(&mut reading, one_g, &mut notes);
    decide_gyroscope(&mut reading, one_g, tilt, &mut notes);
    notes_on_kept(&mut reading, &mut notes);

    // Every channel, in its unit; the board's clocks only where they aren't the clock.
    let mut channels = Vec::with_capacity(reading.channels.len());
    let mut index_of: HashMap<usize, usize> = HashMap::new();
    for (at, build) in reading.channels.iter().enumerate() {
        if build.role == Role::Clock && clock_read {
            continue;
        }
        let mut values = Vec::with_capacity(build.values.len());
        for (value, slot) in build.values.iter().zip(&build.slots) {
            values.push(table::finite(
                value * build.scale,
                FORMAT,
                reading.slots[*slot].line,
                &build.column,
            )?);
        }
        index_of.insert(at, channels.len());
        channels.push(Channel {
            column: build.column.clone(),
            quantity: build.quantity,
            time_s: build.slots.iter().map(|slot| reading.time(*slot)).collect(),
            values,
        });
    }

    // The primary track: `fcb_altitude` in time order, the first at each time.
    let primary = reading
        .find(POSITION, Role::Altitude)
        .filter(|at| reading.channels[*at].quantity == Quantity::AltitudeAboveGround);
    let speed = reading
        .find(POSITION, Role::VerticalSpeed)
        .filter(|at| reading.channels[*at].quantity == Quantity::Speed);
    let (mut time_s, mut altitude_m, mut vertical) = (Vec::new(), Vec::new(), Vec::new());
    // Only the board's clocks are left out of `channels`, so both are there when read.
    let output = |at: usize| index_of.get(&at).map(|&k| &channels[k]);
    if let Some((primary, channel)) = primary.and_then(|at| Some((at, output(at)?))) {
        let build = &reading.channels[primary];
        let mut order: Vec<usize> = (0..build.slots.len())
            .filter(|&i| channel.time_s[i].is_finite())
            .collect();
        let backwards = order
            .windows(2)
            .filter(|pair| channel.time_s[pair[1]] < channel.time_s[pair[0]])
            .count();
        order.sort_by(|&a, &b| channel.time_s[a].total_cmp(&channel.time_s[b]));
        let before = order.len();
        order.dedup_by(|later, earlier| channel.time_s[*later] == channel.time_s[*earlier]);
        let repeats = before - order.len();
        if backwards > 0 || repeats > 0 {
            notes.push(format!(
                "`{POSITION}` goes back in time {backwards} times and repeats a time {repeats} \
                 times. The record's track takes its samples in time order, the first at each \
                 time. The channels keep them all in the file's order"
            ));
        }
        let speed_at = |slot: usize| -> f64 {
            let Some((build, written)) =
                speed.and_then(|at| Some((&reading.channels[at], output(at)?)))
            else {
                return f64::NAN;
            };
            build
                .slots
                .binary_search(&slot)
                .map_or(f64::NAN, |k| written.values[k])
        };
        for i in order {
            time_s.push(channel.time_s[i]);
            altitude_m.push(channel.values[i]);
            vertical.push(speed_at(build.slots[i]));
        }
    } else {
        notes.push(format!(
            "the record has no altitude track: `{POSITION}.fcb_altitude` isn't read in meters \
             (above), and the channels hold everything the file does"
        ));
    }

    let mut line_notes = Notes::default();
    line_notes_into(&reading.counts, no_fix, &mut line_notes);
    notes.extend(line_notes.into_vec());
    let vertical_speed_m_s = speed.filter(|_| primary.is_some()).map(|_| vertical);

    Ok(FlightLog {
        format: LogFormat::AerospaceNuGroundStationText,
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

// ---------------------------------------------------------------------------------------------
// The clock.
// ---------------------------------------------------------------------------------------------

/// Places every packet on the record's clock (module docs); whether the board's clock is read.
fn place_on_clock(slots: &mut [Slot], notes: &mut Vec<String>) -> bool {
    let stretches = stretches_of(slots);
    let span = |stretch: &[usize]| {
        let received = stretch.iter().map(|&at| slots[at].received_s);
        let (lo, hi) = received.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
            (lo.min(t), hi.max(t))
        });
        hi - lo
    };
    let Some((longest, received)) = longest_stretch(&stretches, span) else {
        return true;
    };
    let longest = &stretches[longest];
    let board = {
        let ms = longest.iter().filter_map(|&at| slots[at].board_ms);
        let (lo, hi) = ms.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
            (lo.min(t), hi.max(t))
        });
        (hi - lo) / 1_000.0
    };
    let rate = board / received;
    let evidence = format!(
        "over the longest stretch of packets, {received:.1} s of the ground station's clock, \
         the board's advances {board:.1} s read in milliseconds, {rate:.4} for each second. \
         The stamps in it keep within {STRETCH_GAP_S} s of one gap from the ground station's \
         clock. The band is 0.8 to 1.25 over at least 20 s"
    );
    if !(received >= CLOCK_SPAN_S && (CLOCK_RATE.0..=CLOCK_RATE.1).contains(&rate)) {
        notes.push(format!(
            "`time_stamp_ms` kept as written, unchecked against the ground station's clock: \
             {evidence}. Every packet takes the time the ground station received it"
        ));
        return false;
    }
    notes.push(format!(
        "`time_stamp_ms` read in milliseconds, from its name and the ground station's clock: \
         {evidence}"
    ));
    for stretch in &stretches {
        let mut gaps: Vec<f64> = stretch
            .iter()
            .filter_map(|&at| Some(slots[at].received_s - slots[at].board_ms? / 1_000.0))
            .collect();
        gaps.sort_by(f64::total_cmp);
        // Every packet in a stretch has a board stamp, so `gaps` is never empty.
        let Some(&median) = gaps.get(gaps.len() / 2) else {
            continue;
        };
        for &at in stretch {
            if let Some(ms) = slots[at].board_ms {
                slots[at].time_s = ms / 1_000.0 + median;
            }
        }
    }
    if stretches.len() > 1 {
        notes.push(format!(
            "the board's clock runs in {} stretches, each a steady gap from the ground station's \
             clock, as after a restart of the board. Each is placed on the ground station's \
             clock by its own median gap",
            stretches.len()
        ));
    }
    true
}

/// The stretches (module docs): each the packets with a board stamp, in the file's order, whose
/// gap from the ground station's clock stays within [`STRETCH_GAP_S`] of its first's.
fn stretches_of(slots: &[Slot]) -> Vec<Vec<usize>> {
    let mut stretches: Vec<Vec<usize>> = Vec::new();
    let mut first_gap = f64::NAN;
    for (at, slot) in slots.iter().enumerate() {
        let Some(ms) = slot.board_ms else { continue };
        let gap = slot.received_s - ms / 1_000.0;
        match stretches.last_mut() {
            Some(stretch) if (gap - first_gap).abs() <= STRETCH_GAP_S => stretch.push(at),
            _ => {
                stretches.push(vec![at]);
                first_gap = gap;
            }
        }
    }
    stretches
}

/// The stretch spanning the most of the ground station's clock, the last of the longest if
/// several, and its span: each stretch's span is taken once, so the search is linear in the
/// packets however the stretches fall.
fn longest_stretch(
    stretches: &[Vec<usize>],
    mut span: impl FnMut(&[usize]) -> f64,
) -> Option<(usize, f64)> {
    let spans: Vec<f64> = stretches.iter().map(|stretch| span(stretch)).collect();
    spans
        .iter()
        .copied()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(&b.1))
}

// ---------------------------------------------------------------------------------------------
// The units.
// ---------------------------------------------------------------------------------------------

/// Half a unit in the last place of a value as sent: of a 32-bit float if it is one, else of a
/// 64-bit float.
fn half_ulp(value: f64) -> f64 {
    let magnitude = value.abs();
    #[allow(
        clippy::cast_possible_truncation,
        reason = "testing whether it is a 32-bit float"
    )]
    let single = magnitude as f32;
    if f64::from(single) == magnitude && single.is_finite() {
        let next = f32::from_bits(single.to_bits() + 1);
        0.5 * (f64::from(next) - magnitude)
    } else {
        0.5 * (f64::from_bits(magnitude.to_bits() + 1) - magnitude)
    }
}

/// Whether `value × per_unit` lies within the value's own rounding of a whole number of pascals.
fn on_step(value: f64, per_unit: f64) -> bool {
    let pascals = value * per_unit;
    let product_rounding = f64::from_bits(pascals.abs().to_bits() + 1) - pascals.abs();
    pascals.is_finite()
        && (pascals - pascals.round()).abs() <= per_unit * half_ulp(value) + product_rounding
}

/// The step check of a pressure: the shares of its distinct non-zero values on the 1/101,325 and
/// 1/100,000 steps, and how many there are.
fn steps(values: &[f64]) -> (f64, f64, usize) {
    let mut distinct: Vec<f64> = values
        .iter()
        .copied()
        .filter(|v| v.is_finite() && *v != 0.0)
        .collect();
    distinct.sort_by(f64::total_cmp);
    distinct.dedup();
    let count = distinct.len();
    #[allow(clippy::cast_precision_loss, reason = "counts of samples")]
    let share = |per_unit: f64| {
        distinct.iter().filter(|v| on_step(**v, per_unit)).count() as f64 / count.max(1) as f64
    };
    (share(ATM_PA), share(BAR_PA), count)
}

/// Whether a value is a pad's pressure in atmospheres or bars.
fn pad_size(value: f64) -> bool {
    (PAD_PA.0 / ATM_PA..=PAD_PA.1 / BAR_PA).contains(&value)
}

/// Decides the barometers' and the ground station's pressures (module docs).
fn decide_pressures(reading: &mut Reading, notes: &mut Vec<String>) {
    for build in reading
        .channels
        .iter_mut()
        .filter(|c| c.role == Role::Barometer)
    {
        let first = build
            .values
            .iter()
            .copied()
            .find(|v| v.is_finite() && *v != 0.0);
        let (atm, bar, count) = steps(&build.values);
        let size = first.map_or_else(
            || "it holds no number but 0".to_owned(),
            |first| format!("its first value is {first}"),
        );
        let evidence = format!(
            "{size}, a pad's pressure in atmospheres or bars if within 0.4935 to 1.1. A value \
             counts as a whole number within its own rounding as sent. Of its {count} distinct \
             non-zero values, {:.1}% are a whole number of pascals divided by 101,325. Of the same \
             values, {:.1}% are a whole number of pascals divided by 100,000",
            atm * 100.0,
            bar * 100.0
        );
        if first.is_some_and(pad_size)
            && count >= STEP_LEAST_VALUES
            && atm >= STEP_HOLDS
            && bar <= STEP_FAILS
        {
            notes.push(format!(
                "{} read in atmospheres, from its size and its step: {evidence}",
                build.quoted()
            ));
            build.quantity = Quantity::Pressure;
            build.scale = ATM_PA;
        } else {
            notes.push(format!(
                "{} kept as written, its size or its step not an atmosphere's: {evidence}. \
                 Atmospheres need 99% of at least {STEP_LEAST_VALUES} values on the first step \
                 and at most 10% on the second",
                build.quoted()
            ));
            build.keep();
        }
    }
}

/// Decides `press_ref` from its size and the barometer beside it (module docs).
fn decide_reference(reading: &mut Reading, notes: &mut Vec<String>) {
    let Some(reference) = reading.find(ALT_INFO, Role::Reference) else {
        return;
    };
    let barometer = reading
        .named(&format!("{ALT_INFO}.barometer_pressure"))
        .filter(|at| reading.channels[*at].quantity == Quantity::Pressure);
    let first = barometer.and_then(|barometer| {
        reading
            .paired(reference, barometer)
            .into_iter()
            .find(|(_, r, b)| r.is_finite() && b.is_finite() && *b != 0.0)
    });
    let quoted = reading.channels[reference].quoted();
    let decided = match first {
        Some((_, value, beside)) => {
            let off = (value / beside - 1.0).abs();
            let evidence = format!(
                "its first value, {value}, is {:.3}% from `barometer_pressure`'s in the same \
                 packet, {beside}, read in atmospheres; atmospheres and bars are 1.3% apart",
                off * 100.0
            );
            if pad_size(value) && off <= REFERENCE_SHARE {
                notes.push(format!(
                    "{quoted} read in atmospheres, from its size and the barometer's: {evidence}"
                ));
                true
            } else {
                notes.push(format!(
                    "{quoted} kept as written, its size or the barometer's not an atmosphere's: \
                     {evidence}; atmospheres need a pad's size and 0.5% at most"
                ));
                false
            }
        }
        None => {
            notes.push(format!(
                "{quoted} kept as written, with no barometer read in atmospheres beside it: its \
                 unit is checked against `barometer_pressure`'s in the same packet"
            ));
            false
        }
    };
    let build = &mut reading.channels[reference];
    if decided {
        build.quantity = Quantity::Pressure;
        build.scale = ATM_PA;
    } else {
        build.keep();
    }
}

/// The barometer's samples, Pa, each beside the `fcb_altitude` sample nearest it in time, within
/// [`NEAREST_S`]: pairs of pressure and altitude, in the barometer's order.
fn beside(reading: &Reading, barometer: usize, altitude: usize) -> Vec<(f64, f64)> {
    let build = &reading.channels[altitude];
    let mut heights: Vec<(f64, f64)> = build
        .slots
        .iter()
        .zip(&build.values)
        .filter(|(slot, value)| value.is_finite() && reading.time(**slot).is_finite())
        .map(|(slot, value)| (reading.time(*slot), *value))
        .collect();
    heights.sort_by(|a, b| a.0.total_cmp(&b.0));
    let build = &reading.channels[barometer];
    let mut pairs = Vec::new();
    for (slot, value) in build.slots.iter().zip(&build.values) {
        let time = reading.time(*slot);
        if !(value.is_finite() && *value > 0.0 && time.is_finite()) {
            continue;
        }
        let k = heights.partition_point(|(t, _)| *t < time);
        let near = [k.checked_sub(1), Some(k)]
            .into_iter()
            .flatten()
            .filter_map(|k| heights.get(k))
            .filter(|(t, _)| (t - time).abs() <= NEAREST_S)
            .min_by(|a, b| (a.0 - time).abs().total_cmp(&(b.0 - time).abs()));
        if let Some((_, height)) = near {
            pairs.push((value * ATM_PA, *height));
        }
    }
    pairs
}

/// The `barometer_pressure` channel, if read in atmospheres.
fn barometer(reading: &Reading) -> Option<usize> {
    reading
        .named(&format!("{ALT_INFO}.barometer_pressure"))
        .filter(|at| reading.channels[*at].quantity == Quantity::Pressure)
}

/// A channel's numbers on the record's clock, times its scale: `(time, value)` in time order.
fn series(reading: &Reading, at: usize) -> Vec<(f64, f64)> {
    let build = &reading.channels[at];
    let mut points: Vec<(f64, f64)> = build
        .slots
        .iter()
        .zip(&build.values)
        .map(|(slot, value)| (reading.time(*slot), value * build.scale))
        .filter(|(time, value)| time.is_finite() && value.is_finite())
        .collect();
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    points
}

/// The index of the highest value, the first if several.
fn highest(points: &[(f64, f64)]) -> Option<usize> {
    let mut best: Option<usize> = None;
    for (i, point) in points.iter().enumerate() {
        if best.is_none_or(|b| point.1 > points[b].1) {
            best = Some(i);
        }
    }
    best
}

/// Whether a deceleration, m/s², lies within [`GRAVITY_FACTOR`] of standard gravity either way.
fn near_gravity(deceleration: f64) -> bool {
    let share = deceleration / STANDARD_GRAVITY_MPS2;
    share.is_finite() && (1.0 / GRAVITY_FACTOR..=GRAVITY_FACTOR).contains(&share)
}

/// The second derivative of the least-squares parabola through `points`, `(t, z)`: twice its
/// `t²` coefficient, from the normal equations solved by Cramer's rule about the mean time.
fn curvature(points: &[(f64, f64)]) -> f64 {
    #[allow(clippy::cast_precision_loss, reason = "a count of samples")]
    let n = points.len() as f64;
    let mean = points.iter().map(|p| p.0).sum::<f64>() / n;
    let (mut s, mut y) = ([0.0; 5], [0.0; 3]);
    for &(t, z) in points {
        let x = t - mean;
        let mut power = 1.0;
        for (k, sum) in s.iter_mut().enumerate() {
            *sum += power;
            if let Some(moment) = y.get_mut(k) {
                *moment += z * power;
            }
            power *= x;
        }
    }
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let normal = [[s[0], s[1], s[2]], [s[1], s[2], s[3]], [s[2], s[3], s[4]]];
    let mut quadratic = normal;
    for (row, moment) in quadratic.iter_mut().zip(y) {
        row[2] = moment;
    }
    2.0 * det(quadratic) / det(normal)
}

/// The slope of the least-squares line through `points`, `(t, v)`.
fn slope(points: &[(f64, f64)]) -> f64 {
    #[allow(clippy::cast_precision_loss, reason = "a count of samples")]
    let n = points.len() as f64;
    let mean_t = points.iter().map(|p| p.0).sum::<f64>() / n;
    let mean_v = points.iter().map(|p| p.1).sum::<f64>() / n;
    let (mut cross, mut square) = (0.0, 0.0);
    for &(t, v) in points {
        cross += (t - mean_t) * (v - mean_v);
        square += (t - mean_t) * (t - mean_t);
    }
    cross / square
}

/// A coast's fit: its samples, the time it spans, s, and the deceleration it finds, m/s² read in
/// the candidate unit; `None` with under [`COAST_LEAST_SAMPLES`] samples or [`COAST_LEAST_S`].
#[derive(Debug, Clone, Copy, PartialEq)]
struct Coast {
    samples: usize,
    span_s: f64,
    deceleration: f64,
}

impl Coast {
    fn of(window: &[(f64, f64)], fit: fn(&[(f64, f64)]) -> f64) -> Option<Self> {
        let span_s = window.last()?.0 - window.first()?.0;
        (window.len() >= COAST_LEAST_SAMPLES && span_s >= COAST_LEAST_S).then(|| Self {
            samples: window.len(),
            span_s,
            deceleration: -fit(window),
        })
    }

    /// The evidence, `what` naming the fit.
    fn text(found: Option<Self>, what: &str, unit: &str) -> String {
        found.map_or_else(
            || {
                format!(
                    "The coast holds fewer than {COAST_LEAST_SAMPLES} samples or spans under \
                     {COAST_LEAST_S} s, too little to fit"
                )
            },
            |coast| {
                format!(
                    "Over the coast, {} samples across {:.1} s, {what} slows it {:.2} {unit}, \
                     {:.3} g read so. An upward coast slows at least 1 g. The band is a \
                     factor of {GRAVITY_FACTOR} either way, where feet would read 3.28 times",
                    coast.samples,
                    coast.span_s,
                    coast.deceleration,
                    coast.deceleration / STANDARD_GRAVITY_MPS2
                )
            },
        )
    }
}

/// The altitude's coast (module docs): from the later sample of its fastest step, of those at most
/// [`COAST_STEP_S`] long, to its highest value, the least-squares parabola's deceleration.
fn altitude_coast(points: &[(f64, f64)]) -> Option<Coast> {
    let top = highest(points)?;
    let mut fastest: Option<(f64, usize)> = None;
    for (i, pair) in points[..=top].windows(2).enumerate() {
        let step = pair[1].0 - pair[0].0;
        let rate = (pair[1].1 - pair[0].1) / step;
        if step > 0.0 && step <= COAST_STEP_S && fastest.is_none_or(|(best, _)| rate > best) {
            fastest = Some((rate, i + 1));
        }
    }
    let (_, start) = fastest?;
    Coast::of(&points[start..=top], curvature)
}

/// The speed's coast (module docs): from its highest value to its first at or below 0 after it,
/// the least-squares line's deceleration.
fn speed_coast(points: &[(f64, f64)]) -> Option<Coast> {
    let top = highest(points)?;
    let end = (top..points.len()).find(|&i| points[i].1 <= 0.0)?;
    Coast::of(&points[top..=end], slope)
}

/// Decides `fcb_altitude` by the barometer's climb and the coast (module docs), refusing the file
/// if the barometer refutes meters.
fn decide_altitude(
    reading: &mut Reading,
    start_line: usize,
    notes: &mut Vec<String>,
) -> Result<(), LogError> {
    let Some(altitude) = reading.find(POSITION, Role::Altitude) else {
        return Ok(());
    };
    let quoted = reading.channels[altitude].quoted();
    let Some(barometer) = barometer(reading) else {
        notes.push(format!(
            "{quoted} kept as written, with no barometer read in atmospheres: its climb is \
             checked against `barometer_pressure`'s"
        ));
        reading.channels[altitude].keep();
        return Ok(());
    };
    let pairs = beside(reading, barometer, altitude);
    let atmosphere = Ussa76::standard();
    let climbs = pairs.first().copied().and_then(|pad| {
        let top = pairs.iter().copied().min_by(|a, b| a.0.total_cmp(&b.0))?;
        let high = atmosphere.pressure_altitude_m(top.0).ok()?;
        let ground = atmosphere.pressure_altitude_m(pad.0).ok()?;
        Some((high - ground, top.1 - pad.1))
    });
    let (barometer_text, borne) = match climbs {
        None => (
            format!(
                "no `{POSITION}` sample lies within {NEAREST_S} s of a pressure of \
                 `barometer_pressure` that has an altitude in the standard atmosphere"
            ),
            false,
        ),
        Some((climb, _)) if climb < CHECK_CLIMB_M => (
            format!(
                "the barometer climbs {climb:.2} m in the standard atmosphere, under the \
                 {CHECK_CLIMB_M} m from which it checks a unit"
            ),
            false,
        ),
        Some((climb, column)) => {
            let evidence = format!(
                "from `barometer_pressure`'s first value to its lowest, each within {NEAREST_S} s \
                 of a sample of the column, the standard atmosphere climbs {climb:.1} m. The \
                 column's climb over the same span is {column:.1}, {:.3} times that. Meters need within 25%",
                column / climb
            );
            if (column / climb - 1.0).abs() > MARGIN {
                return Err(LogError::Unit {
                    format: FORMAT,
                    line: start_line,
                    message: format!("{quoted} isn't in meters, by the barometer: {evidence}"),
                });
            }
            (evidence, true)
        }
    };
    let coast = altitude_coast(&series(reading, altitude));
    let evidence = format!(
        "{barometer_text}. {}",
        Coast::text(coast, "the column's least-squares parabola", "m/s²")
    );
    if borne && coast.is_some_and(|coast| near_gravity(coast.deceleration)) {
        notes.push(format!(
            "{quoted} read in meters, from the barometer and the coast: {evidence}"
        ));
        reading.channels[altitude].quantity = Quantity::AltitudeAboveGround;
    } else {
        notes.push(format!(
            "{quoted} kept as written, the barometer and the coast not both bearing meters out: \
             {evidence}"
        ));
        reading.channels[altitude].keep();
    }
    Ok(())
}

/// The ascent the barometer reads: from the lowest pressure back to the last sample before the
/// pressure stops falling, its first and last times, s, and its climb in the standard atmosphere,
/// m.
fn ascent(points: &[(f64, f64)]) -> Option<(f64, f64, f64)> {
    let mut top: Option<usize> = None;
    for (i, point) in points.iter().enumerate() {
        if point.1 > 0.0 && top.is_none_or(|t| point.1 < points[t].1) {
            top = Some(i);
        }
    }
    let top = top?;
    let mut start = top;
    while start > 0 && points[start - 1].1 > points[start].1 {
        start -= 1;
    }
    let atmosphere = Ussa76::standard();
    let high = atmosphere.pressure_altitude_m(points[top].1).ok()?;
    let ground = atmosphere.pressure_altitude_m(points[start].1).ok()?;
    Some((points[start].0, points[top].0, high - ground))
}

/// A speed's integral, m, from `from` to `to`, s, the speed taken as a straight line between
/// samples, and the longest step it bridges; `None` unless samples lie on both sides.
fn integral(points: &[(f64, f64)], from: f64, to: f64) -> Option<(f64, f64)> {
    if points.first()?.0 > from || points.last()?.0 < to {
        return None;
    }
    let (mut sum, mut longest) = (0.0, 0.0_f64);
    for pair in points.windows(2) {
        let ((t0, v0), (t1, v1)) = (pair[0], pair[1]);
        let (lo, hi) = (t0.max(from), t1.min(to));
        if t1 <= t0 || hi <= lo {
            continue;
        }
        let at = |t: f64| v0 + (v1 - v0) * (t - t0) / (t1 - t0);
        sum += 0.5 * (at(lo) + at(hi)) * (hi - lo);
        longest = longest.max(t1 - t0);
    }
    Some((sum, longest))
}

/// Decides `v_speed` by the barometer's climb and the coast (module docs).
fn decide_speed(reading: &mut Reading, notes: &mut Vec<String>) {
    let Some(speed) = reading.find(POSITION, Role::VerticalSpeed) else {
        return;
    };
    let quoted = reading.channels[speed].quoted();
    let Some(barometer) = barometer(reading) else {
        notes.push(format!(
            "{quoted} kept as written, with no barometer read in atmospheres: its integral is \
             checked against `barometer_pressure`'s climb"
        ));
        reading.channels[speed].keep();
        return;
    };
    let points = series(reading, speed);
    let climb = ascent(&series(reading, barometer))
        .and_then(|(from, to, climb)| Some((from, to, climb, integral(&points, from, to)?)));
    let (barometer_text, borne) = match climb {
        None => (
            "the barometer reads no ascent the column's samples span".to_owned(),
            false,
        ),
        Some((from, to, climb, (moved, longest))) => {
            let ratio = climb / moved;
            (
                format!(
                    "over the barometer's ascent, the standard atmosphere climbs {climb:.1} m. The \
                     ascent is the {:.1} s from where the barometer's pressure last began to fall to its \
                     lowest. The column integrates to {moved:.1}, the speed taken as a straight \
                     line between samples up to {longest:.1} s apart. The climb is {ratio:.3} \
                     times that. Meters per second need at least {CHECK_CLIMB_M} m and within 25%",
                    to - from
                ),
                climb >= CHECK_CLIMB_M && (ratio - 1.0).abs() <= MARGIN,
            )
        }
    };
    let coast = speed_coast(&points);
    let evidence = format!(
        "{barometer_text}. {}",
        Coast::text(coast, "the column's least-squares line", "m/s²")
    );
    if borne && coast.is_some_and(|coast| near_gravity(coast.deceleration)) {
        notes.push(format!(
            "{quoted} read in meters per second, from the barometer and the coast: {evidence}"
        ));
        reading.channels[speed].quantity = Quantity::Speed;
    } else {
        notes.push(format!(
            "{quoted} kept as written, the barometer and the coast not both bearing meters per \
             second out: {evidence}"
        ));
        reading.channels[speed].keep();
    }
}

/// Decides each pair of positions (module docs); the count of (0, 0) positions made gaps.
fn decide_positions(reading: &mut Reading, notes: &mut Vec<String>) -> usize {
    let mut no_fix = 0;
    let latitudes: Vec<usize> = (0..reading.channels.len())
        .filter(|&at| reading.channels[at].role == Role::Latitude)
        .collect();
    let mut paired_longitudes = Vec::new();
    for lat in latitudes {
        let packet = reading.channels[lat].packet.clone();
        let Some(lon) = reading.find(&packet, Role::Longitude) else {
            notes.push(format!(
                "{} kept as written, without its longitude: a position is decided as one fix",
                reading.channels[lat].quoted()
            ));
            reading.channels[lat].keep();
            continue;
        };
        paired_longitudes.push(lon);
        let names = format!(
            "{} and {}",
            reading.channels[lat].quoted(),
            reading.channels[lon].quoted()
        );
        let pairs = reading.paired(lat, lon);
        let fixes: Vec<(f64, f64)> = pairs
            .iter()
            .filter(|(_, a, b)| a.is_finite() && b.is_finite() && !(*a == 0.0 && *b == 0.0))
            .map(|(_, a, b)| (*a, *b))
            .collect();
        if fixes.is_empty() {
            notes.push(format!(
                "{names} kept as written, holding no fix: every position is (0, 0), the \
                 receiver's mark for none, or a gap"
            ));
            reading.channels[lat].keep();
            reading.channels[lon].keep();
            continue;
        }
        let within = fixes
            .iter()
            .all(|(a, b)| a.abs() <= 90.0 && b.abs() <= 180.0);
        let beyond_radians = fixes
            .iter()
            .filter(|(a, b)| a.abs() > PI / 2.0 || b.abs() > PI)
            .count();
        let fraction = |v: f64| v.abs().fract();
        let past_minutes = fixes
            .iter()
            .filter(|(a, b)| fraction(*a).max(fraction(*b)) >= 0.6)
            .count();
        // Most fixes, not one, so that a stray fix decides nothing.
        let most = |count: usize| 2 * count > fixes.len();
        let evidence = format!(
            "{} fixes, {} within ±90° and ±180°, {beyond_radians} beyond ±π/2 or ±π. \
             {past_minutes} of them have a part after the whole degrees of 0.6 or more, where \
             degrees and minutes run together stay under 0.6. Degrees need all within and most \
             of the fixes past both",
            fixes.len(),
            if within { "all" } else { "not all" },
        );
        if within && most(beyond_radians) && most(past_minutes) {
            notes.push(format!(
                "{names} read in degrees, from their range and their fractions: {evidence}"
            ));
            for (slot, a, b) in pairs {
                if a == 0.0 && b == 0.0 {
                    no_fix += 1;
                    for at in [lat, lon] {
                        let build = &mut reading.channels[at];
                        if let Ok(k) = build.slots.binary_search(&slot) {
                            build.values[k] = f64::NAN;
                        }
                    }
                }
            }
            reading.channels[lat].quantity = Quantity::Latitude;
            reading.channels[lon].quantity = Quantity::Longitude;
        } else {
            notes.push(format!(
                "{names} kept as written, their range or their fractions not degrees': \
                 {evidence}"
            ));
            reading.channels[lat].keep();
            reading.channels[lon].keep();
        }
    }
    for at in 0..reading.channels.len() {
        if reading.channels[at].role == Role::Longitude && !paired_longitudes.contains(&at) {
            notes.push(format!(
                "{} kept as written, without its latitude: a position is decided as one fix",
                reading.channels[at].quoted()
            ));
            reading.channels[at].keep();
        }
    }
    no_fix
}

/// A value as a note writes it: as Rust writes it, or with an exponent if very small or large.
fn shown(value: f64) -> String {
    if value != 0.0 && !(1e-4..1e9).contains(&value.abs()) {
        format!("{value:e}")
    } else {
        format!("{value}")
    }
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// The three axes of a sensor, by role, if the file has all three.
fn axes(reading: &Reading, role: fn(usize) -> Role) -> Option<[usize; 3]> {
    Some([
        reading.find(ORIENTATION, role(0))?,
        reading.find(ORIENTATION, role(1))?,
        reading.find(ORIENTATION, role(2))?,
    ])
}

/// The names of some channels, as a sentence.
fn listed(reading: &Reading, at: &[usize]) -> String {
    let quoted: Vec<String> = at.iter().map(|&i| reading.channels[i].quoted()).collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// The median of some numbers, `None` if there are none.
fn median(mut values: Vec<f64>) -> Option<f64> {
    values.sort_by(f64::total_cmp);
    values.get(values.len() / 2).copied()
}

/// The climb check of the accelerometer (module docs): how many pressures it is weighed at, and
/// the median of the barometer's climb over the accelerometer's there; or why it can't be made.
fn accelerometer_climb(reading: &Reading, axes: [usize; 3]) -> Result<(usize, f64), String> {
    let Some(barometer) = barometer(reading) else {
        return Err("the file has no `barometer_pressure` read in atmospheres".to_owned());
    };
    let mut samples: Vec<(f64, [f64; 3])> = reading
        .tripled(axes)
        .into_iter()
        .map(|(slot, a)| (reading.time(slot), a))
        .filter(|(t, a)| t.is_finite() && a.iter().all(|v| v.is_finite()))
        .collect();
    samples.sort_by(|a, b| a.0.total_cmp(&b.0));
    let force = |i: usize| magnitude(samples[i].1);
    let launch = (0..samples.len().saturating_sub(2)).find_map(|i| {
        let before = &samples[i.saturating_sub(PAD_SAMPLES)..i];
        if before.len() < PAD_LEAST_SAMPLES {
            return None;
        }
        let one_g = median(before.iter().map(|s| magnitude(s.1)).collect())?;
        (one_g > 0.0
            && force(i) >= LAUNCH_FORCE * one_g
            && force(i + 1) >= LAUNCH_HELD * one_g
            && force(i + 2) >= LAUNCH_HELD * one_g)
            .then_some((i, before))
    });
    let Some((launch, before)) = launch else {
        return Err(format!(
            "no launch: no sample reads {LAUNCH_FORCE} times the median of the {PAD_SAMPLES} \
             before it and the next two {LAUNCH_HELD} times"
        ));
    };
    let pad: Vec<f64> = (0..3)
        .filter_map(|axis| median(before.iter().map(|s| s.1[axis]).collect()))
        .collect();
    let pad_g = (pad.iter().map(|v| v * v).sum::<f64>()).sqrt();
    if pad.len() != 3 || pad_g <= 0.0 {
        return Err("the pad's reading has no direction".to_owned());
    }
    let up: Vec<f64> = pad.iter().map(|v| v / pad_g).collect();
    let net = |i: usize| {
        let a = samples[i].1;
        a[0] * up[0] + a[1] * up[1] + a[2] * up[2] - STANDARD_GRAVITY_MPS2
    };
    let launched_s = samples[launch].0;
    let mut rows: Vec<(f64, f64)> = Vec::new();
    let (mut speed, mut climb) = (0.0, 0.0);
    for i in launch..samples.len() - 1 {
        let step = samples[i + 1].0 - samples[i].0;
        if !(step > 0.0 && step <= INTEGRATE_STEP_S) {
            break;
        }
        let next = speed + 0.5 * (net(i) + net(i + 1)) * step;
        climb += 0.5 * (speed + next) * step;
        speed = next;
        if speed.is_nan() || speed <= 0.0 {
            break;
        }
        rows.push((samples[i + 1].0, climb));
    }
    let pressures = series(reading, barometer);
    let atmosphere = Ussa76::standard();
    let Some(pad_m) = pressures
        .iter()
        .rev()
        .find(|(t, p)| *t <= launched_s && *p > 0.0)
        .and_then(|(_, p)| atmosphere.pressure_altitude_m(*p).ok())
    else {
        return Err("no pressure on the pad at or before the launch".to_owned());
    };
    let mut ratios = Vec::new();
    for &(t, p) in pressures.iter().filter(|(t, _)| *t > launched_s) {
        let k = rows.partition_point(|row| row.0 < t);
        let near = [k.checked_sub(1), Some(k)]
            .into_iter()
            .flatten()
            .filter_map(|k| rows.get(k))
            .filter(|row| (row.0 - t).abs() <= NEAREST_S)
            .min_by(|a, b| (a.0 - t).abs().total_cmp(&(b.0 - t).abs()));
        let (Some(&(_, integrated)), Ok(height)) = (near, atmosphere.pressure_altitude_m(p)) else {
            continue;
        };
        if integrated >= CHECK_CLIMB_M {
            ratios.push((height - pad_m) / integrated);
        }
    }
    let count = ratios.len();
    match median(ratios) {
        Some(ratio) if count >= INTEGRATED_LEAST => Ok((count, ratio)),
        _ => Err(format!(
            "from the launch, it integrates to {climb:.1} m before its speed falls to 0 or a \
             step passes {INTEGRATE_STEP_S} s. Past {CHECK_CLIMB_M} m of that climb there are \
             {count} pressures, under the {INTEGRATED_LEAST} needed"
        )),
    }
}

/// Decides the accelerometer from the pad's gravity and the barometer's climb (module docs):
/// 1 g in the written unit, if read in m/s².
fn decide_accelerometer(reading: &mut Reading, notes: &mut Vec<String>) -> Option<f64> {
    let axes = axes(reading, Role::Accel)?;
    let names = listed(reading, &axes);
    let samples = reading.tripled(axes);
    let (mut in_ms2, mut in_g) = (0.0, 0.0);
    for pair in samples.windows(2) {
        let step = reading.time(pair[1].0) - reading.time(pair[0].0);
        let reading_g = magnitude(pair[0].1);
        if !(step > 0.0 && step.is_finite() && reading_g.is_finite()) {
            continue;
        }
        let weight = step.min(GRAVITY_STEP_MAX_S);
        if (reading_g - STANDARD_GRAVITY_MPS2).abs() <= GRAVITY_MARGIN * STANDARD_GRAVITY_MPS2 {
            in_ms2 += weight;
        }
        if (reading_g - 1.0).abs() <= GRAVITY_MARGIN {
            in_g += weight;
        }
    }
    let pad = in_ms2 >= GRAVITY_LEAST_S && in_ms2 >= GRAVITY_LEAD * in_g;
    let climb = accelerometer_climb(reading, axes);
    let climbs = climb.as_ref().is_ok_and(|&(_, ratio)| {
        ratio.is_finite() && (1.0 / GRAVITY_FACTOR..=GRAVITY_FACTOR).contains(&ratio)
    });
    let climb_text = match &climb {
        Ok((count, ratio)) => format!(
            "From the launch, its specific force along the pad's reading, less 9.806 65, \
             integrates read in m/s² to a climb. At {count} pressures past {CHECK_CLIMB_M} m \
             of it, the barometer's climb in the standard atmosphere is {ratio:.3} times it, as \
             a median. The band is a factor of {GRAVITY_FACTOR} either way"
        ),
        Err(why) => format!("The climb check can't be made: {why}"),
    };
    let evidence = format!(
        "the three axes' magnitude reads within 10% of 9.806 65 for {in_ms2:.1} s of the file's \
         time. It reads within 10% of 1 for {in_g:.1} s. A board at rest or under a parachute \
         reads 1 g. Meters per second squared need at least 1 s and twice the other's. \
         {climb_text}"
    );
    let decided = pad && climbs;
    if decided {
        notes.push(format!(
            "{names} read in meters per second squared, from the pad's gravity and the \
             barometer: {evidence}"
        ));
    } else {
        notes.push(format!(
            "{names} kept as written, the pad's gravity and the barometer not both bearing \
             meters per second squared out: {evidence}"
        ));
    }
    for at in axes {
        if decided {
            reading.channels[at].quantity = Quantity::Acceleration;
        } else {
            reading.channels[at].keep();
        }
    }
    decided.then_some(STANDARD_GRAVITY_MPS2)
}

/// Decides `angle_vertical` from its range and the accelerometer (module docs); whether read in
/// degrees.
fn decide_tilt(reading: &mut Reading, one_g: Option<f64>, notes: &mut Vec<String>) -> bool {
    let Some(tilt) = reading.find(ORIENTATION, Role::Tilt) else {
        return false;
    };
    let quoted = reading.channels[tilt].quoted();
    let range = reading.channels[tilt].range();
    let in_range = range.is_some_and(|(lo, hi)| lo >= 0.0 && hi <= 180.0 && hi > PI);
    let range_text = range.map_or_else(
        || "it holds no number".to_owned(),
        |(lo, hi)| format!("its values run from {} to {}", shown(lo), shown(hi)),
    );
    let (Some(one_g), Some(accel)) = (one_g, axes(reading, Role::Accel)) else {
        notes.push(format!(
            "{quoted} kept as written, with no accelerometer read in m/s² to check it by: \
             {range_text}"
        ));
        reading.channels[tilt].keep();
        return false;
    };
    let accel = reading.tripled(accel);
    let column = &reading.channels[tilt];
    let (mut as_degrees, mut as_radians) = (Vec::new(), Vec::new());
    let mut k = 0;
    for (slot, a) in accel {
        while k < column.slots.len() && column.slots[k] < slot {
            k += 1;
        }
        if k >= column.slots.len() || column.slots[k] != slot {
            continue;
        }
        let m = magnitude(a);
        let value = column.values[k];
        if !(value.is_finite() && m.is_finite()) || (m - one_g).abs() > TILT_GRAVITY_MARGIN * one_g
        {
            continue;
        }
        let from_z = (a[2] / m).clamp(-1.0, 1.0).acos().to_degrees();
        as_degrees.push((value - from_z).abs());
        as_radians.push((value.to_degrees() - from_z).abs());
    }
    let median = |mut v: Vec<f64>| {
        v.sort_by(f64::total_cmp);
        v.get(v.len() / 2).copied().unwrap_or(f64::NAN)
    };
    let samples = as_degrees.len();
    let (degrees, radians) = (median(as_degrees), median(as_radians));
    let evidence = format!(
        "{range_text}. The accelerometer reads within 5% of 1 g at {samples} samples. At \
         these, the median difference from the angle of the accelerometer's reading from the z \
         axis is {degrees:.2}° read in degrees. Read in radians, it is {radians:.2}°"
    );
    if in_range
        && samples >= TILT_LEAST_SAMPLES
        && degrees <= TILT_AGREE_DEG
        && radians > TILT_AGREE_DEG
    {
        notes.push(format!(
            "{quoted} read in degrees, from its range and the accelerometer: {evidence}"
        ));
        reading.channels[tilt].quantity = Quantity::Angle;
        true
    } else {
        notes.push(format!(
            "{quoted} kept as written, its range or the accelerometer not degrees': {evidence}. \
             Degrees need 0 to 180 with a value past π, at least {TILT_LEAST_SAMPLES} samples \
             and 2° at most"
        ));
        reading.channels[tilt].keep();
        false
    }
}

/// What the turning check found.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Turning {
    stretches: usize,
    accelerometer_rad: f64,
    gyroscope_rad: f64,
}

/// The turning check (module docs), the gyroscope read in rad/s.
fn turning(reading: &Reading, samples: &[(usize, [f64; 3], [f64; 3])], one_g: f64) -> Turning {
    let mut found = Turning {
        stretches: 0,
        accelerometer_rad: 0.0,
        gyroscope_rad: 0.0,
    };
    let time = |i: usize| reading.time(samples[i].0);
    let near_g = |i: usize| {
        let m = magnitude(samples[i].1);
        m.is_finite() && (m - one_g).abs() <= TURN_GRAVITY_MARGIN * one_g
    };
    let fast = |i: usize| {
        let step = time(i + 1) - time(i);
        step > 0.0 && step <= TURN_STEP_S && samples[i].2.iter().all(|v| v.is_finite())
    };
    let direction = |i: usize| {
        let a = samples[i].1;
        let m = magnitude(a);
        a.map(|v| v / m)
    };
    let mut start = 0;
    while start + TURN_STEPS < samples.len() {
        let end = start + TURN_STEPS;
        if !((start..end).all(fast) && (start..=end).all(near_g)) {
            start += 1;
            continue;
        }
        let (from, to) = (direction(start), direction(end));
        let cosine = (from[0] * to[0] + from[1] * to[1] + from[2] * to[2]).clamp(-1.0, 1.0);
        let mut gyroscope = 0.0;
        for (i, sample) in samples.iter().enumerate().skip(start).take(TURN_STEPS) {
            let w = sample.2;
            let a = direction(i);
            let across = [
                w[1] * a[2] - w[2] * a[1],
                w[2] * a[0] - w[0] * a[2],
                w[0] * a[1] - w[1] * a[0],
            ];
            gyroscope += magnitude(across) * (time(i + 1) - time(i));
        }
        found.stretches += 1;
        found.accelerometer_rad += cosine.acos();
        found.gyroscope_rad += gyroscope;
        start = end;
    }
    found
}

/// The tilt check (module docs): the windows, the tilt's change over them, rad, and the
/// gyroscope's turn across the z axis, read in rad/s.
fn tilt_check(reading: &Reading, samples: &[(usize, f64, [f64; 3])]) -> (usize, f64, f64) {
    let time = |i: usize| reading.time(samples[i].0);
    let (mut windows, mut tilted, mut turned) = (0, 0.0, 0.0);
    let mut start = 0;
    while start + 1 < samples.len() {
        let mut end = start;
        while end + 1 < samples.len()
            && time(end + 1) - time(start) <= TILT_WINDOW_S
            && time(end + 1) - time(end) > 0.0
            && time(end + 1) - time(end) <= TILT_STEP_S
        {
            end += 1;
        }
        if end == start {
            start += 1;
            continue;
        }
        let change = (samples[end].1 - samples[start].1).to_radians().abs();
        if change >= TILT_LEAST_RAD {
            let turn: f64 = (start..end)
                .map(|i| {
                    let w = samples[i].2;
                    (w[0] * w[0] + w[1] * w[1]).sqrt() * (time(i + 1) - time(i))
                })
                .sum();
            if turn.is_finite() {
                windows += 1;
                tilted += change;
                turned += turn;
            }
        }
        start = end;
    }
    (windows, tilted, turned)
}

/// Decides the gyroscope from the board's turning and its tilt (module docs).
fn decide_gyroscope(
    reading: &mut Reading,
    one_g: Option<f64>,
    tilt_read: bool,
    notes: &mut Vec<String>,
) {
    let Some(gyro) = axes(reading, Role::Gyro) else {
        return;
    };
    let names = listed(reading, &gyro);
    let keep = |reading: &mut Reading| {
        for at in gyro {
            reading.channels[at].keep();
        }
    };
    let (Some(one_g), Some(accel)) = (one_g, axes(reading, Role::Accel)) else {
        notes.push(format!(
            "{names} kept as written, with no accelerometer read in m/s²: their unit is checked \
             against its gravity as the board turns"
        ));
        keep(reading);
        return;
    };
    let accel = reading.tripled(accel);
    let rates = reading.tripled(gyro);
    let mut samples = Vec::new();
    let mut k = 0;
    for (slot, a) in accel {
        while k < rates.len() && rates[k].0 < slot {
            k += 1;
        }
        if k < rates.len() && rates[k].0 == slot {
            samples.push((slot, a, rates[k].1));
        }
    }
    let found = turning(reading, &samples, one_g);
    let ratio = found.gyroscope_rad / found.accelerometer_rad;
    let degrees = PI / 180.0;
    let within = |ratio: f64| (1.0 / TURN_FACTOR..=TURN_FACTOR).contains(&ratio);
    let enough =
        found.stretches >= TURN_LEAST_STRETCHES && found.accelerometer_rad >= TURN_LEAST_RAD;
    let turning_text = format!(
        "over {} stretches of {TURN_STEPS} steps at most {TURN_STEP_S} s apart at about 1 g, the \
         accelerometer's reading turns {:.3} rad. Over those stretches the gyroscope turns the \
         board across it {:.3} rad read in radians per second",
        found.stretches, found.accelerometer_rad, found.gyroscope_rad
    );
    let tilt = if tilt_read {
        reading.find(ORIENTATION, Role::Tilt)
    } else {
        None
    };
    let (windows, tilted, turned) = tilt.map_or((0, 0.0, 0.0), |tilt| {
        let column = &reading.channels[tilt];
        let mut joined = Vec::new();
        let mut k = 0;
        for &(slot, _, w) in &samples {
            while k < column.slots.len() && column.slots[k] < slot {
                k += 1;
            }
            if k < column.slots.len() && column.slots[k] == slot && column.values[k].is_finite() {
                joined.push((slot, column.values[k], w));
            }
        }
        tilt_check(reading, &joined)
    });
    let tilt_text = format!(
        "Over {windows} windows of at most {TILT_WINDOW_S} s in which `angle_vertical` moves at \
         least 10°, it changes {tilted:.3} rad. In those windows the gyroscope turns the board across its z \
         axis {turned:.3} rad read in radians per second and {:.3} rad read in degrees",
        turned * degrees
    );
    let tilt_refutes = |turn: f64| tilted > TILT_FACTOR * turn;
    let tilt_decides =
        windows >= TILT_LEAST_WINDOWS && tilt_refutes(turned * degrees) && !tilt_refutes(turned);
    if enough && within(ratio) && tilt_decides {
        notes.push(format!(
            "{names} read in radians per second, from the board's turning and its tilt: \
             {turning_text}, within a quarter to 4 times. {tilt_text}. So degrees per second \
             turn too little for the tilt"
        ));
        for at in gyro {
            reading.channels[at].quantity = Quantity::AngularRate;
            reading.channels[at].scale = 180.0 / PI;
        }
    } else {
        notes.push(format!(
            "{names} kept as written, the board's turning and its tilt not both radians per \
             second's: {turning_text}. {tilt_text}. Radians per second need at least 3 \
             stretches, 0.2 rad and a quarter to 4 times. They also need 3 windows whose tilt \
             refutes degrees per second"
        ));
        keep(reading);
    }
}

/// Why a kept field is kept: the part before the colon, and after it.
fn kept_why(kind: Kept) -> (&'static str, &'static str) {
    match kind {
        Kept::Temperature => (
            "in degrees Celsius or Fahrenheit alike",
            "its name states no unit, and nothing in the file tells the two apart",
        ),
        Kept::Unstated => (
            "in a unit nothing in the file checks",
            "its name states no unit, and no check in the file reaches one",
        ),
        Kept::GnssAltitude => (
            "with no check of its unit but the barometer's",
            "a GPS altitude's rise against the barometer's tells meters from feet, but no second \
             check independent of the barometer is in the file",
        ),
        Kept::Euler => (
            "with no check of their unit but their range",
            "within ±π they read as radians would, but nothing else in the file checks them",
        ),
        Kept::Magnetic => (
            "with no quantity for a magnetic field",
            "the record has none, and their names state no unit",
        ),
    }
}

/// Notes on the fields kept as written by kind and on those the reader doesn't know; sets each
/// to [`Quantity::Unrecognized`].
fn notes_on_kept(reading: &mut Reading, notes: &mut Vec<String>) {
    // Each packet's Euler angles and magnetometer as one note, the others one each.
    let mut groups: Vec<(Kept, String, Vec<usize>)> = Vec::new();
    for (at, build) in reading.channels.iter().enumerate() {
        let Role::Kept(kind) = build.role else {
            continue;
        };
        let grouped = matches!(kind, Kept::Euler | Kept::Magnetic);
        match groups
            .iter_mut()
            .find(|(k, packet, _)| grouped && *k == kind && *packet == build.packet)
        {
            Some((_, _, members)) => members.push(at),
            None => groups.push((kind, build.packet.clone(), vec![at])),
        }
    }
    for (kind, _, members) in groups {
        let (decision, why) = kept_why(kind);
        let range = members
            .iter()
            .filter_map(|&at| reading.channels[at].range())
            .reduce(|(lo, hi), (l, h)| (lo.min(l), hi.max(h)));
        let whose = if members.len() > 1 { "their" } else { "its" };
        let range = range.map_or_else(
            || format!("{whose} values hold no number"),
            |(lo, hi)| format!("{whose} values run from {} to {}", shown(lo), shown(hi)),
        );
        notes.push(format!(
            "{} kept as written, {decision}: {why}; {range}",
            listed(reading, &members)
        ));
        for at in members {
            reading.channels[at].keep();
        }
    }
    let mut unknown: Vec<(String, Vec<usize>)> = Vec::new();
    for (at, build) in reading.channels.iter().enumerate() {
        if build.role != Role::Unknown {
            continue;
        }
        match unknown
            .iter_mut()
            .find(|(packet, _)| *packet == build.packet)
        {
            Some((_, members)) => members.push(at),
            None => unknown.push((build.packet.clone(), vec![at])),
        }
    }
    let mut unknown_notes = Notes::default();
    for (packet, members) in unknown {
        let first = &reading.channels[members[0]].column;
        let field = first.strip_prefix(&format!("{packet}.")).unwrap_or(first);
        unknown_notes.push(if members.len() == 1 {
            format!(
                "1 field of the `{}` packets, `{}`, isn't one this reader knows: its values are \
                 kept as written",
                short(&packet),
                short(field)
            )
        } else {
            format!(
                "{} fields of the `{}` packets, such as `{}`, aren't ones this reader knows: \
                 their values are kept as written",
                members.len(),
                short(&packet),
                short(field)
            )
        });
    }
    notes.extend(unknown_notes.into_vec());
}

/// A count of things and where the first is, as a note's subject: `1 line, on line 5,` or
/// `2 lines, the first on line 5,`.
fn counted(count: usize, one: &str, many: &str, line: usize) -> String {
    if count == 1 {
        format!("1 {one}, on line {line},")
    } else {
        format!("{count} {many}, the first on line {line},")
    }
}

/// The word for one thing, or for several.
fn agree<'a>(count: usize, one: &'a str, many: &'a str) -> &'a str {
    if count == 1 { one } else { many }
}

/// The notes about lines and values skipped.
fn line_notes_into(counts: &Counts, no_fix: usize, notes: &mut Notes) {
    if let Some(line) = counts.first_corrupt {
        let n = counts.corrupt;
        notes.push(format!(
            "{} {} left out: the ground station marks {} `crc` other than `'Good'`, {} bytes \
             having failed the radio's checksum",
            counted(n, "packet", "packets", line),
            agree(n, "is", "are"),
            agree(n, "its", "their"),
            agree(n, "its", "their"),
        ));
    }
    if let Some((line, text)) = &counts.first_message {
        notes.push(if counts.messages == 1 {
            format!(
                "1 line is the ground station's own message, not a packet, {text:?} on line \
                 {line}: skipped"
            )
        } else {
            format!(
                "{} lines are the ground station's own messages, not packets, such as {text:?} \
                 on line {line}: skipped",
                counts.messages
            )
        });
    }
    if let Some((line, why)) = &counts.first_unparsed {
        let n = counts.unparsed;
        notes.push(format!(
            "{} {} parse as a Python literal: {}; skipped",
            counted(n, "packet line", "packet lines", *line),
            agree(n, "doesn't", "don't"),
            short(why)
        ));
    }
    if let Some(line) = counts.first_other {
        let n = counts.other;
        notes.push(format!(
            "{} {} with no time of day `HH:MM:SS`: skipped",
            counted(n, "line", "lines", line),
            agree(n, "opens", "open")
        ));
    }
    if let Some(line) = counts.first_long {
        let n = counts.long;
        notes.push(format!(
            "{} {} longer than {MAX_LINE_BYTES} bytes: skipped",
            counted(n, "line", "lines", line),
            agree(n, "is", "are")
        ));
    }
    if let Some(line) = counts.first_run {
        let n = counts.runs;
        notes.push(format!(
            "{} {} skipped: the times follow the first",
            counted(n, "more `RUN START` line", "more `RUN START` lines", line),
            agree(n, "is", "are")
        ));
    }
    if let Some(line) = counts.first_gap {
        let n = counts.gaps;
        notes.push(format!(
            "{} {} `None`, `nan`, `inf` or text that isn't a number: {} a gap",
            counted(n, "value", "values", line),
            agree(n, "is", "are"),
            agree(n, "it is", "each is")
        ));
    }
    if no_fix > 0 {
        notes.push(format!(
            "{no_fix} {} of (0, 0), the receiver's mark for no fix, {}",
            agree(no_fix, "position", "positions"),
            agree(no_fix, "is a gap", "are gaps")
        ));
    }
    if let Some(line) = counts.first_left_out {
        let n = counts.left_out;
        notes.push(format!(
            "{} {} left out: {} longer than {MAX_NAME_CHARS} characters, or past the first \
             {MAX_CHANNELS} channels",
            counted(n, "value", "values", line),
            agree(n, "is", "are"),
            agree(n, "its field's name is", "their fields' names are")
        ));
    }
    let names = |set: &BTreeSet<String>| {
        set.iter()
            .map(|name| format!("`{}`", short(name)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    if !counts.text_fields.is_empty() {
        notes.push(format!(
            "the fields {} hold text, not numbers: they are left out of the channels; `crc` \
             decides which packets are read",
            names(&counts.text_fields)
        ));
    }
    if !counts.nested_fields.is_empty() {
        notes.push(format!(
            "the fields {} hold lists of lists or dictionaries: they are left out",
            names(&counts.nested_fields)
        ));
    }
}

fn not_this(message: String) -> LogError {
    LogError::NotThisFormat {
        format: FORMAT,
        message,
    }
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: f64 = STANDARD_GRAVITY_MPS2;

    /// How an invented flight is written: each quantity's value per SI unit as sent (per pascal
    /// for the pressures, per millisecond for the board's clock), and when its board restarts.
    #[derive(Debug, Clone, Copy)]
    struct Written {
        per_pa: f64,
        accel: f64,
        /// The accelerometer's readings off the pad, on top of `accel`.
        boost: f64,
        gyro: f64,
        altitude: f64,
        speed: f64,
        /// The altitude written as a straight line through the coast, its ends kept.
        straight_coast: bool,
        /// The speed written as its mean through the coast, its integral kept.
        steady_coast: bool,
        stamp: f64,
        latitude: f64,
        longitude: f64,
        restart_at_s: Option<f64>,
    }

    impl Default for Written {
        fn default() -> Self {
            Self {
                per_pa: 1.0 / ATM_PA,
                accel: 1.0,
                boost: 1.0,
                gyro: 1.0,
                altitude: 1.0,
                speed: 1.0,
                straight_coast: false,
                steady_coast: false,
                stamp: 1.0,
                latitude: 41.7102,
                longitude: -70.8301,
                restart_at_s: None,
            }
        }
    }

    /// The pad's 6 s, the boost's 2 s at 40 m/s², and the file's 30 s in quarters.
    const BOOST_S: f64 = 6.0;
    const BURNOUT_S: f64 = 8.0;
    const QUARTERS: u32 = 120;
    /// The coast's roll, rad/s, as the gyroscope's z axis reads it.
    const COAST_ROLL: f64 = 0.5;

    fn apogee_s() -> f64 {
        BURNOUT_S + 80.0 / G
    }

    /// The flight at a time: altitude, m, vertical speed, m/s, the swing under the parachute,
    /// rad, and its rate, rad/s.
    fn state(t: f64) -> (f64, f64, f64, f64) {
        if t < BOOST_S {
            (0.0, 0.0, 0.0, 0.0)
        } else if t < BURNOUT_S {
            let s = t - BOOST_S;
            (20.0 * s * s, 40.0 * s, 0.0, 0.0)
        } else if t < apogee_s() {
            let s = t - BURNOUT_S;
            (80.0 + 80.0 * s - 0.5 * G * s * s, 80.0 - G * s, 0.0, 0.0)
        } else {
            let s = t - apogee_s();
            let top = 80.0 + 80.0 * 80.0 / (2.0 * G);
            let swing = 0.45 * (0.9 * s).sin();
            (top - 15.0 * s, -15.0, swing, 0.45 * 0.9 * (0.9 * s).cos())
        }
    }

    /// The pressure, whole Pa, at an altitude above a pad 300 m up in the standard atmosphere's
    /// troposphere: the altimeter formula inverted.
    fn pascals(altitude_m: f64) -> f64 {
        (101_325.0 * (1.0 - (300.0 + altitude_m) / 44_330.8).powf(1.0 / 0.190_263)).round()
    }

    /// A value as a 32-bit float, as the board sends it.
    fn single(value: f64) -> f64 {
        #[allow(clippy::cast_possible_truncation, reason = "the board's 32-bit floats")]
        let single = value as f32;
        f64::from(single)
    }

    /// A value rounded to 4 decimals, so that the lines stay short.
    fn short4(value: f64) -> f64 {
        (value * 10_000.0).round() / 10_000.0
    }

    /// A clock reading `HH:MM:SS.ffffff`.
    fn clock(seconds: f64) -> String {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "a time of day in microseconds"
        )]
        let micros = (seconds * 1e6).round() as u64;
        let whole = micros / 1_000_000;
        format!(
            "{:02}:{:02}:{:02}.{:06}",
            (whole / 3_600) % 24,
            (whole / 60) % 60,
            whole % 60,
            micros % 1_000_000
        )
    }

    /// A number as Python writes it.
    fn py(value: f64) -> String {
        format!("{value:?}")
    }

    /// A packet line: the board's header fields if `stamp_ms`, then `fields`.
    fn packet(
        received_s: f64,
        name: &str,
        stamp_ms: Option<f64>,
        fields: &[(&str, String)],
    ) -> String {
        let mut all: Vec<String> = Vec::new();
        if let Some(ms) = stamp_ms {
            all.extend([
                "'radio_id': 1".to_owned(),
                "'radio_name': '915 MHz Radio'".to_owned(),
                "'rssi': '-60 db'".to_owned(),
                "'lqi': 3".to_owned(),
                "'crc': 'Good'".to_owned(),
                "'board_serial_number': 7".to_owned(),
                format!("'time_stamp_ms': {ms:.0}"),
                "'callsign': 'N0CALL'".to_owned(),
            ]);
        }
        all.extend(
            fields
                .iter()
                .map(|(key, value)| format!("'{key}': {value}")),
        );
        format!("{}: {name} {{{}}}\n", clock(received_s), all.join(", "))
    }

    /// An invented flight in the ground station's layout (module docs): 6 s on the pad, a 2 s
    /// boost, a coast to 406 m, and 14 s under a swinging parachute.
    fn flight(written: Written) -> String {
        let start_s = 15.0 * 3_600.0;
        let mut text = String::from("\n\nRUN START 2025-01-01 15:00:00\n\n");
        text.push_str(&format!(
            "{}: Switching radio to band 1\n",
            clock(start_s + 1.0)
        ));
        let pad_pa = pascals(0.0);
        for quarter in 0..=QUARTERS {
            let t = f64::from(quarter) / 4.0;
            let flying = t >= BOOST_S;
            let mut kinds = Vec::new();
            if flying || quarter % 4 == 0 {
                kinds.push(0); // Orientation
            }
            if (flying && quarter % 2 == 0) || quarter % 4 == 0 {
                kinds.push(1); // Position Data
            }
            if quarter % 4 == 0 {
                kinds.push(2); // Alt Info & Cfg
            }
            if quarter % 8 == 0 {
                kinds.extend([3, 4]); // Pyro Info, Ground Station GPS
            }
            let board_ms = match written.restart_at_s {
                Some(restart) if t >= restart => 2_000.0 + (t - restart) * 1_000.0,
                _ => 120_000.0 + t * 1_000.0,
            } * written.stamp;
            let (z, v, swing, swing_rate) = state(t);
            let descending = t >= apogee_s();
            let coasting = (BURNOUT_S..apogee_s()).contains(&t);
            let top = state(apogee_s()).0;
            let written_z = if written.straight_coast && coasting {
                80.0 + (top - 80.0) * (t - BURNOUT_S) / (apogee_s() - BURNOUT_S)
            } else {
                z
            };
            let written_v = if written.steady_coast && coasting {
                40.0
            } else {
                v
            } * written.speed;
            let accel_scale = written.accel * if flying { written.boost } else { 1.0 };
            let state_number = if descending { "3" } else { "2" }.to_owned();
            let (accel, gyro, tilt_deg) = if t < BOOST_S {
                let lean = 5.0_f64.to_radians();
                ([G * lean.sin(), 0.0, G * lean.cos()], [0.0; 3], 5.0)
            } else if t < BURNOUT_S {
                ([0.0, 0.0, 40.0 + G], [0.0; 3], 0.0)
            } else if !descending {
                ([0.0; 3], [0.0, 0.0, COAST_ROLL], 0.0)
            } else {
                (
                    [G * swing.sin(), 0.0, G * swing.cos()],
                    [0.0, swing_rate, 0.0],
                    swing.abs().to_degrees(),
                )
            };
            for (j, kind) in kinds.into_iter().enumerate() {
                #[allow(clippy::cast_precision_loss, reason = "a small count")]
                let received = start_s + 2.03 + t + 0.002 * j as f64;
                let f = |v: f64| py(short4(v));
                let line = match kind {
                    0 => packet(
                        received,
                        ORIENTATION,
                        Some(board_ms),
                        &[
                            ("fcb_state_number", state_number.clone()),
                            ("rot_vel_x", f(gyro[0] * written.gyro)),
                            ("rot_vel_y", f(gyro[1] * written.gyro)),
                            ("rot_vel_z", f(gyro[2] * written.gyro)),
                            ("accel_x", f(accel[0] * accel_scale)),
                            ("accel_y", f(accel[1] * accel_scale)),
                            ("accel_z", f(accel[2] * accel_scale)),
                            ("magnetic_field_x", f(0.21)),
                            ("magnetic_field_y", f(-0.13)),
                            ("magnetic_field_z", f(0.42)),
                            ("angle_vertical", py((tilt_deg * 10.0).round() / 10.0)),
                            ("roll", f(0.0)),
                            ("pitch", f(swing)),
                            ("yaw", f(0.1)),
                            (
                                "orientation_quaternion",
                                format!(
                                    "[{}, 0.0, {}, 0.0]",
                                    f((swing / 2.0).cos()),
                                    f((swing / 2.0).sin())
                                ),
                            ),
                            ("orientation_rpy", format!("[0.0, {}, 0.1]", f(swing))),
                        ],
                    ),
                    1 => packet(
                        received,
                        POSITION,
                        Some(board_ms),
                        &[
                            ("temperature", py(single(25.5))),
                            ("fcb_altitude", py(single(written_z * written.altitude))),
                            ("v_speed", py(single(written_v))),
                            ("fcb_latitude", py(written.latitude + 1e-4 * z / 400.0)),
                            ("fcb_longitude", py(written.longitude)),
                            ("gps_alt", py(single(((300.0 + z) * 10.0).round() / 10.0))),
                            ("fcb_battery_voltage", py(single(7.9))),
                            ("ground_speed", py(0.0)),
                            ("course_over_ground", py(0.0)),
                            ("gps_time", format!("'2025-01-01 20:00:{:02.0}'", t.floor())),
                            ("gps_sats", "9".to_owned()),
                            ("fcb_state_number", state_number.clone()),
                        ],
                    ),
                    2 => {
                        let p = pascals(z);
                        packet(
                            received,
                            ALT_INFO,
                            Some(board_ms),
                            &[
                                ("barometer_pressure", py(single(p * written.per_pa))),
                                (
                                    "barometer_2_pressure",
                                    py(single((p + 3.0) * written.per_pa)),
                                ),
                                ("press_ref", py(single((pad_pa + 0.4) * written.per_pa))),
                                ("ground_elev", py(0.0)),
                                ("ground_temp", py(20.0)),
                                ("pitot_ducer_press", py(0.0)),
                            ],
                        )
                    }
                    3 => packet(
                        received,
                        "Pyro Info",
                        Some(board_ms),
                        &[
                            ("pyro_continuity", "[True, True, False, False]".to_owned()),
                            ("flash_usage", "10".to_owned()),
                        ],
                    ),
                    _ => {
                        let fix = t > 0.0;
                        packet(
                            received,
                            GROUND_STATION[0],
                            None,
                            &[
                                ("gs_lat", py(if fix { 41.7101 } else { 0.0 })),
                                ("gs_lon", py(if fix { -70.8302 } else { 0.0 })),
                                ("gs_alt", py(300.0)),
                                ("gs_baro", py((pad_pa + 120.0 + t) * written.per_pa)),
                                ("gs_temp", py(30.25)),
                            ],
                        )
                    }
                };
                text.push_str(&line);
            }
        }
        text
    }

    /// The channel named `column`.
    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap_or_else(|| panic!("no {column}"))
    }

    /// The one note opening as given.
    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
        matching[0]
    }

    /// The flight's text with `from` changed to `to` on the first line holding both `packet` and
    /// `from`: the text and the line's number.
    fn changed(text: &str, packet: &str, from: &str, to: &str) -> (String, usize) {
        let target = text
            .lines()
            .position(|line| line.contains(packet) && line.contains(from))
            .unwrap_or_else(|| panic!("no {from}"));
        let mut out = String::new();
        for (k, line) in text.lines().enumerate() {
            if k == target {
                out.push_str(&line.replacen(from, to, 1));
            } else {
                out.push_str(line);
            }
            out.push('\n');
        }
        (out, target + 1)
    }

    #[test]
    fn python_literals_parse() {
        let value = parse_literal(
            r#"{'a': 1, "b": -2.5e-3, 'c': 'it\'s \\ \x41é\n', 'd': [True, False, None], 'e': (1, nan, -inf, +inf), 'f': {'g': .5,}, 'h': 1., 'i': +3, 'j': 1E+2}"#,
        )
        .unwrap();
        let Value::Dict(entries) = value else {
            panic!("{value:?}")
        };
        assert_eq!(entries[0], ("a".to_owned(), Value::Number(1.0)));
        assert_eq!(entries[1], ("b".to_owned(), Value::Number(-0.0025)));
        assert_eq!(
            entries[2],
            ("c".to_owned(), Value::Text("it's \\ Aé\n".to_owned()))
        );
        assert_eq!(
            entries[3].1,
            Value::List(vec![Value::Bool(true), Value::Bool(false), Value::Nothing])
        );
        let Value::List(numbers) = &entries[4].1 else {
            panic!("{:?}", entries[4])
        };
        assert!(matches!(numbers[1], Value::Number(n) if n.is_nan()));
        assert_eq!(numbers[2], Value::Number(f64::NEG_INFINITY));
        assert_eq!(numbers[3], Value::Number(f64::INFINITY));
        assert_eq!(
            entries[5].1,
            Value::Dict(vec![("g".to_owned(), Value::Number(0.5))])
        );
        assert_eq!(entries[6].1, Value::Number(1.0));
        assert_eq!(entries[7].1, Value::Number(3.0));
        assert_eq!(entries[8].1, Value::Number(100.0));
        assert_eq!(
            parse_literal(r"'\101\0\q'").unwrap(),
            Value::Text("A\0\\q".to_owned())
        );
        assert_eq!(parse_literal(" [] ").unwrap(), Value::List(Vec::new()));
        // As deep as allowed, and as many items.
        let deep = format!("{}1{}", "[".repeat(MAX_DEPTH), "]".repeat(MAX_DEPTH));
        assert!(parse_literal(&deep).is_ok());
        let wide = format!("[{}]", vec!["0"; MAX_ITEMS].join(","));
        assert!(parse_literal(&wide).is_ok());
    }

    #[test]
    fn malformed_literals_are_refused_by_why() {
        let deep = format!(
            "{}1{}",
            "[".repeat(MAX_DEPTH + 1),
            "]".repeat(MAX_DEPTH + 1)
        );
        let wide = format!("[{}]", vec!["0"; MAX_ITEMS + 1].join(","));
        let keys: Vec<String> = (0..=MAX_ITEMS).map(|k| format!("'k{k}': 0")).collect();
        let many_keys = format!("{{{}}}", keys.join(","));
        let cases = [
            ("{'a': 1", "no comma or `}` after an item"),
            ("{'a': ", "the line ends inside the packet"),
            ("{'a' 1}", "no `:` after a key"),
            ("{1: 2}", "a key that isn't text"),
            ("{'a': 1, 'a': 2}", "the key \"a\" given twice"),
            ("{'a': 1.2.3}", "a number Python wouldn't write"),
            ("{'a': 0x10}", "a number Python wouldn't write"),
            ("{'a': 1e}", "an exponent without digits"),
            ("{'a': -}", "a number without digits"),
            ("{'a': -True}", "a sign before a word that isn't a number"),
            ("{'a': foo}", "the word \"foo\", no Python literal"),
            ("{'a': 'open}", "a text in quotes runs to the line's end"),
            ("{'a': '\\x4'}", "an escape without its 2 digits"),
            ("{'a': 1} extra", "text after the literal"),
            ("{'a': 1 'b': 2}", "no comma or `}` after an item"),
            ("{'a': @}", "an unexpected character"),
            (
                deep.as_str(),
                "lists or dictionaries nested more than 8 deep",
            ),
            (wide.as_str(), "a list of more than 1024 items"),
            (many_keys.as_str(), "a dictionary of more than 1024 items"),
        ];
        for (text, why) in cases {
            let error = parse_literal(text).unwrap_err();
            assert!(error.starts_with(why), "{text:.40}: {error}");
        }
    }

    #[test]
    fn times_of_day_are_strict() {
        assert_eq!(
            time_of_day("14:45:09.743501"),
            Some(14.0 * 3600.0 + 45.0 * 60.0 + 9.743_501)
        );
        assert_eq!(time_of_day("00:00:00"), Some(0.0));
        for bad in [
            "24:00:00",
            "1:00:00",
            "12:60:00",
            "12:00:60",
            "12:00",
            "12:00:00:00",
            "aa:bb:cc",
            "12:00:0.5",
            "12:00:00.-5",
            "12:00:00.1234567890",
        ] {
            assert_eq!(time_of_day(bad), None, "{bad}");
        }
    }

    /// The step check, both ways: whole pascals over 101,325 sent as 32-bit floats lie on the
    /// first step and not the second; over 100,000, the other way round; whole multiples of 1/25,
    /// on both.
    #[test]
    fn the_step_check_tells_atmospheres_from_bars() {
        let pascals: Vec<f64> = (0..200).map(|k| 95_000.0 + 37.0 * f64::from(k)).collect();
        let atm: Vec<f64> = pascals.iter().map(|p| single(p / ATM_PA)).collect();
        let bar: Vec<f64> = pascals.iter().map(|p| single(p / BAR_PA)).collect();
        let (on_atm, on_bar, count) = steps(&atm);
        assert_eq!(count, 200);
        assert!((on_atm - 1.0).abs() < f64::EPSILON, "{on_atm}");
        assert!(on_bar < 0.05, "{on_bar}");
        let (on_atm, on_bar, _) = steps(&bar);
        assert!((on_bar - 1.0).abs() < f64::EPSILON, "{on_bar}");
        assert!(on_atm < 0.05, "{on_atm}");
        let both: Vec<f64> = (20..36).map(|m| f64::from(m) / 25.0).collect();
        let (on_atm, on_bar, _) = steps(&both);
        assert!(on_atm > 0.99 && on_bar > 0.99, "{on_atm} {on_bar}");
        // A 64-bit value, as the ground station's own barometer sends: its own rounding.
        assert!(on_step(103_587.0 / ATM_PA, ATM_PA));
        assert!(!on_step(103_587.01 / ATM_PA, ATM_PA));
        // Half a unit in the last place of a 32-bit float near 1: 2⁻²⁵ below 1, 2⁻²⁴ above.
        assert!((half_ulp(single(0.99)) - 2f64.powi(-25)).abs() < f64::EPSILON);
        assert!((half_ulp(single(1.01)) - 2f64.powi(-24)).abs() < f64::EPSILON);
        assert!((half_ulp(0.1) - 0.5 * (f64::from_bits(0.1_f64.to_bits() + 1) - 0.1)).abs() == 0.0);
    }

    /// The invented flight: every unit read by its two checks, the track from `fcb_altitude`, and
    /// the notes in the survey's shape.
    #[test]
    fn a_flight_reads_every_unit_by_two_checks() {
        let log = read(&flight(Written::default())).unwrap();
        assert_eq!(log.format, LogFormat::AerospaceNuGroundStationText);
        let read = [
            ("Alt Info & Cfg.barometer_pressure", Quantity::Pressure),
            ("Alt Info & Cfg.barometer_2_pressure", Quantity::Pressure),
            ("Alt Info & Cfg.press_ref", Quantity::Pressure),
            ("Ground Station GPS.gs_baro", Quantity::Pressure),
            ("Position Data.fcb_altitude", Quantity::AltitudeAboveGround),
            ("Position Data.v_speed", Quantity::Speed),
            ("Position Data.fcb_latitude", Quantity::Latitude),
            ("Position Data.fcb_longitude", Quantity::Longitude),
            ("Ground Station GPS.gs_lat", Quantity::Latitude),
            ("Orientation.accel_z", Quantity::Acceleration),
            ("Orientation.rot_vel_y", Quantity::AngularRate),
            ("Orientation.angle_vertical", Quantity::Angle),
            ("Position Data.gps_time", Quantity::TimeOfDay),
            ("Position Data.rssi", Quantity::Number),
            ("Pyro Info.pyro_continuity[0]", Quantity::Number),
            ("Orientation.orientation_quaternion[2]", Quantity::Number),
            ("Position Data.temperature", Quantity::Unrecognized),
            ("Position Data.gps_alt", Quantity::Unrecognized),
            ("Orientation.pitch", Quantity::Unrecognized),
            ("Orientation.orientation_rpy[1]", Quantity::Unrecognized),
            ("Orientation.magnetic_field_x", Quantity::Unrecognized),
        ];
        for (column, expected) in read {
            assert_eq!(channel(&log, column).quantity, expected, "{column}");
        }
        // The board's clocks aren't channels when they are the clock.
        assert!(
            log.channels
                .iter()
                .all(|c| !c.column.ends_with("time_stamp_ms"))
        );
        // In SI: the pad's pressure in pascals, the swing's rate in degrees per second.
        let pressure = channel(&log, "Alt Info & Cfg.barometer_pressure");
        assert!(
            (pressure.values[0] - pascals(0.0)).abs() < 0.01,
            "{}",
            pressure.values[0]
        );
        let swing = channel(&log, "Orientation.rot_vel_y");
        let fastest = swing.values.iter().copied().fold(0.0, f64::max);
        assert!((fastest - 0.405_f64.to_degrees()).abs() < 0.5, "{fastest}");
        // The track: `Position Data` in time order, from 0 on the pad to 406 m.
        assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
        assert!(log.altitude_m[0].abs() < f64::EPSILON);
        let top = log.altitude_m.iter().copied().fold(0.0, f64::max);
        assert!((top - 406.3).abs() < 0.5, "{top}");
        assert_eq!(
            log.vertical_speed_m_s.as_ref().map(Vec::len),
            Some(log.time_s.len())
        );
        // Placed on the ground station's clock: about 2.03 s after `RUN START`, on the pad.
        assert!((log.time_s[0] - 2.03).abs() < 0.05, "{}", log.time_s[0]);
        for start in [
            "`time_stamp_ms` read in milliseconds, from its name and the ground station's clock: ",
            "`Alt Info & Cfg.barometer_pressure` read in atmospheres, from its size and its step: ",
            "`Alt Info & Cfg.barometer_2_pressure` read in atmospheres, from its size and its \
             step: ",
            "`Ground Station GPS.gs_baro` read in atmospheres, from its size and its step: ",
            "`Alt Info & Cfg.press_ref` read in atmospheres, from its size and the barometer's: ",
            "`Position Data.fcb_altitude` read in meters, from the barometer and the coast: ",
            "`Position Data.v_speed` read in meters per second, from the barometer and the coast: ",
            "`Position Data.fcb_latitude` and `Position Data.fcb_longitude` read in degrees, from \
             their range and their fractions: ",
            "`Ground Station GPS.gs_lat` and `Ground Station GPS.gs_lon` read in degrees, from \
             their range and their fractions: ",
            "`Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` read in meters \
             per second squared, from the pad's gravity and the barometer: ",
            "`Orientation.angle_vertical` read in degrees, from its range and the accelerometer: ",
            "`Orientation.rot_vel_x`, `Orientation.rot_vel_y` and `Orientation.rot_vel_z` read in \
             radians per second, from the board's turning and its tilt: ",
            "`Position Data.temperature` kept as written, in degrees Celsius or Fahrenheit alike: ",
            "`Position Data.gps_alt` kept as written, with no check of its unit but the \
             barometer's: ",
            "`Orientation.roll`, `Orientation.pitch`, `Orientation.yaw`, \
             `Orientation.orientation_rpy[0]`, `Orientation.orientation_rpy[1]` and \
             `Orientation.orientation_rpy[2]` kept as written, with no check of their unit but \
             their range: ",
            "1 position of (0, 0), the receiver's mark for no fix, is a gap",
            "1 line is the ground station's own message, not a packet, \"Switching radio to band \
             1\" on line 5: skipped",
            "the fields `callsign`, `radio_name` hold text, not numbers",
        ] {
            note(&log, start);
        }
        assert!(
            log.notes.iter().all(|n| !n.contains("assum")),
            "{:#?}",
            log.notes
        );
        // The ground station's first position, (0, 0), is a gap.
        assert!(channel(&log, "Ground Station GPS.gs_lat").values[0].is_nan());
    }

    /// Pressures on the 1/100,000 step, as bars would be: kept as written, and the altitude with
    /// them, the record's track empty.
    #[test]
    fn pressures_in_bars_are_kept_and_the_altitude_with_them() {
        let log = read(&flight(Written {
            per_pa: 1.0 / BAR_PA,
            ..Written::default()
        }))
        .unwrap();
        for column in [
            "Alt Info & Cfg.barometer_pressure",
            "Alt Info & Cfg.press_ref",
            "Ground Station GPS.gs_baro",
            "Position Data.fcb_altitude",
            "Position Data.v_speed",
        ] {
            assert_eq!(
                channel(&log, column).quantity,
                Quantity::Unrecognized,
                "{column}"
            );
        }
        let kept = note(
            &log,
            "`Alt Info & Cfg.barometer_pressure` kept as written, its size or its step not an \
             atmosphere's: ",
        );
        assert!(
            kept.contains("100.0% are a whole number of pascals divided by 100,000"),
            "{kept}"
        );
        note(
            &log,
            "`Alt Info & Cfg.press_ref` kept as written, with no barometer read in atmospheres \
             beside it: ",
        );
        note(
            &log,
            "`Position Data.fcb_altitude` kept as written, with no barometer read in atmospheres: ",
        );
        note(
            &log,
            "`Position Data.v_speed` kept as written, with no barometer read in atmospheres: ",
        );
        note(
            &log,
            "`Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` kept as \
             written, the pad's gravity and the barometer not both bearing meters per second \
             squared out: ",
        );
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        assert!(log.vertical_speed_m_s.is_none());
        note(&log, "the record has no altitude track: ");
    }

    /// A pressure column read only when both checks hold: one on both steps (whole multiples of
    /// 1/25) or on neither (a half pascal off) is kept as written, and so is one on the first
    /// step alone at a thousand times a pad's size (thousandths of an atmosphere).
    #[test]
    fn pressures_on_both_steps_or_neither_are_kept() {
        let mut both = String::from("RUN START 2025-01-01 10:00:00\n");
        let mut neither = both.clone();
        let mut thousandths = both.clone();
        for m in 20..36 {
            thousandths.push_str(&format!(
                "10:00:{m:02}.0: Alt Info & Cfg {{'barometer_pressure': {}}}\n",
                py((95_000.0 + 101.0 * f64::from(m)) / 101.325)
            ));
            both.push_str(&format!(
                "10:00:{m:02}.0: Alt Info & Cfg {{'barometer_pressure': {}}}\n",
                py(f64::from(m) / 25.0)
            ));
            neither.push_str(&format!(
                "10:00:{m:02}.0: Alt Info & Cfg {{'barometer_pressure': {}}}\n",
                py(single((95_000.5 + 101.0 * f64::from(m)) / ATM_PA))
            ));
        }
        for (text, shares) in [
            (
                both,
                "100.0% are a whole number of pascals divided by 101,325. Of the same values, \
                    100.0% are a whole number of pascals divided by 100,000",
            ),
            (
                neither,
                "0.0% are a whole number of pascals divided by 101,325. Of the same values, \
                       0.0% are a whole number of pascals divided by 100,000",
            ),
            (
                thousandths,
                "100.0% are a whole number of pascals divided by 101,325",
            ),
        ] {
            let log = read(&text).unwrap();
            let kept = note(
                &log,
                "`Alt Info & Cfg.barometer_pressure` kept as written, its size or its step not an \
                 atmosphere's: ",
            );
            assert!(kept.contains(shares), "{kept}");
            assert!(kept.contains("within its own rounding as sent"), "{kept}");
            assert_eq!(
                channel(&log, "Alt Info & Cfg.barometer_pressure").quantity,
                Quantity::Unrecognized
            );
        }
    }

    /// An altitude in feet: the barometer refutes meters, and the file is refused.
    #[test]
    fn an_altitude_the_barometer_refutes_refuses_the_file() {
        let error = read(&flight(Written {
            altitude: 1.0 / 0.3048,
            ..Written::default()
        }))
        .unwrap_err();
        let LogError::Unit { line, message, .. } = &error else {
            panic!("{error}")
        };
        assert_eq!(*line, 3);
        assert!(
            message.starts_with("`Position Data.fcb_altitude` isn't in meters, by the barometer: ")
                && message.contains(" 3.2")
                && message.ends_with(" times that. Meters need within 25%"),
            "{message}"
        );
    }

    /// An accelerometer in g: the pad's gravity and the climb refute meters per second squared;
    /// it is kept, and the tilt and the gyroscope that are checked by it.
    #[test]
    fn an_accelerometer_in_g_is_kept_and_what_rests_on_it() {
        let log = read(&flight(Written {
            accel: 1.0 / G,
            ..Written::default()
        }))
        .unwrap();
        let kept = note(
            &log,
            "`Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` kept as \
             written, the pad's gravity and the barometer not both bearing meters per second \
             squared out: ",
        );
        assert!(
            kept.contains("within 10% of 9.806 65 for 0.0 s")
                && kept
                    .contains("Past 10 m of that climb there are 0 pressures, under the 3 needed"),
            "{kept}"
        );
        note(
            &log,
            "`Orientation.angle_vertical` kept as written, with no accelerometer read in m/s²",
        );
        note(
            &log,
            "`Orientation.rot_vel_x`, `Orientation.rot_vel_y` and `Orientation.rot_vel_z` kept as \
             written, with no accelerometer read in m/s²",
        );
        for column in [
            "Orientation.accel_z",
            "Orientation.angle_vertical",
            "Orientation.rot_vel_y",
        ] {
            assert_eq!(
                channel(&log, column).quantity,
                Quantity::Unrecognized,
                "{column}"
            );
        }
    }

    /// A gyroscope in degrees per second: the board turns 57 times more than its accelerometer
    /// shows, and the gyroscope is kept as written.
    #[test]
    fn a_gyroscope_in_degrees_per_second_is_kept() {
        let log = read(&flight(Written {
            gyro: 180.0 / PI,
            ..Written::default()
        }))
        .unwrap();
        note(
            &log,
            "`Orientation.rot_vel_x`, `Orientation.rot_vel_y` and `Orientation.rot_vel_z` kept as \
             written, the board's turning and its tilt not both radians per second's: ",
        );
        assert_eq!(
            channel(&log, "Orientation.rot_vel_y").quantity,
            Quantity::Unrecognized
        );
        // The tilt in degrees still reads: it rests on the accelerometer alone.
        assert_eq!(
            channel(&log, "Orientation.angle_vertical").quantity,
            Quantity::Angle
        );
    }

    /// An `angle_vertical` in radians: within ±π, so its range alone refutes degrees.
    #[test]
    fn a_tilt_in_radians_is_kept() {
        let text = flight(Written::default());
        let mut out = String::new();
        for line in text.lines() {
            let line = match line.split_once("'angle_vertical': ") {
                Some((before, after)) => {
                    let (value, rest) = after.split_once(',').unwrap();
                    let radians = value.parse::<f64>().unwrap().to_radians();
                    format!("{before}'angle_vertical': {},{rest}", py(short4(radians)))
                }
                None => line.to_owned(),
            };
            out.push_str(&line);
            out.push('\n');
        }
        let log = read(&out).unwrap();
        let kept = note(
            &log,
            "`Orientation.angle_vertical` kept as written, its range or the accelerometer not \
             degrees': ",
        );
        assert!(
            kept.contains("its values run from 0 to 0.4485")
                && kept.contains("Read in radians, it is 0.03°"),
            "{kept}"
        );
        // And with no tilt read, no tilt check: the gyroscope is kept too.
        assert_eq!(
            channel(&log, "Orientation.rot_vel_y").quantity,
            Quantity::Unrecognized
        );
    }

    /// The tilt check alone: a gyroscope turning as fast as the tilt changes is borne out in
    /// rad/s and refuted in °/s; too few windows decide nothing.
    #[test]
    fn the_tilt_check_refutes_too_slow_a_unit() {
        let mut reading = Reading::default();
        let mut samples = Vec::new();
        for k in 0..41 {
            let t = 0.25 * f64::from(k);
            reading.slots.push(Slot {
                line: 1,
                received_s: t,
                board_ms: None,
                time_s: t,
            });
            // Tilting back and forth, the gyroscope turning at the tilt's rate.
            let tilt = 40.0 * (0.5 * t).sin();
            let rate = (20.0 * (0.5 * t).cos()).to_radians();
            samples.push((reading.slots.len() - 1, tilt, [0.0, rate, 0.0]));
        }
        let (windows, tilted, turned) = tilt_check(&reading, &samples);
        assert!(windows >= TILT_LEAST_WINDOWS, "{windows}");
        assert!(tilted <= TILT_FACTOR * turned, "{tilted} {turned}");
        assert!(
            tilted > TILT_FACTOR * turned * PI / 180.0,
            "{tilted} {turned}"
        );
        let (windows, _, _) = tilt_check(&reading, &samples[..9]);
        assert!(windows < TILT_LEAST_WINDOWS, "{windows}");
    }

    /// A board clock in seconds, not milliseconds: it drifts from the ground station's a second
    /// each second, so no stretch spans 20 s, and every packet takes the time it was received.
    #[test]
    fn a_clock_in_other_units_is_kept_and_the_ground_stations_used() {
        let log = read(&flight(Written {
            stamp: 0.001,
            ..Written::default()
        }))
        .unwrap();
        note(
            &log,
            "`time_stamp_ms` kept as written, unchecked against the ground station's clock: ",
        );
        let stamps = channel(&log, "Position Data.time_stamp_ms");
        assert_eq!(stamps.quantity, Quantity::Unrecognized);
        assert!((stamps.values[0] - 120.0).abs() < f64::EPSILON);
        // On the ground station's clock: the pad's first position 2.032 s after `RUN START`.
        assert!((log.time_s[0] - 2.032).abs() < 1e-6, "{}", log.time_s[0]);
        assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
    }

    /// A board that restarts: its clock starts again, and each stretch is placed by its own gap,
    /// so the track runs on in time without a jump.
    #[test]
    fn a_restart_is_placed_by_its_own_gap() {
        let log = read(&flight(Written {
            restart_at_s: Some(3.0),
            ..Written::default()
        }))
        .unwrap();
        note(
            &log,
            "the board's clock runs in 2 stretches, each a steady gap from the ground station's \
             clock",
        );
        let steps: Vec<f64> = log
            .time_s
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        assert!(
            steps.iter().all(|step| (0.4..1.1).contains(step)),
            "{steps:?}"
        );
        assert!(!log.notes.iter().any(|n| n.contains("goes back in time")));
    }

    /// Skipped lines and left-out packets, each counted: a corrupt packet, a message, a line that
    /// doesn't parse, one with no time, one too long, a second `RUN START`, and a `nan`.
    #[test]
    fn skipped_lines_are_counted_by_kind() {
        let mut text = flight(Written::default());
        let mut line = text.lines().count();
        let mut next = |text: &mut String, added: &str| {
            text.push_str(added);
            text.push('\n');
            line += 1;
            line
        };
        let corrupt = next(
            &mut text,
            "15:00:40.000000: Position Data {'crc': 'Bad', 'time_stamp_ms': 9999999999, \
             'temperature': -3.1e11}",
        );
        next(
            &mut text,
            "15:00:40.100000: Invalid message type 26 Could not parse message",
        );
        let unparsed = next(
            &mut text,
            "15:00:40.200000: Pyro Info {'crc': 'Good', 'flash_usage': 'é', 'n': 1,,}",
        );
        let other = next(&mut text, "garbage without a time");
        let long = next(
            &mut text,
            &format!(
                "15:00:40.300000: X {{'a': '{}'}}",
                "x".repeat(MAX_LINE_BYTES)
            ),
        );
        let run = next(&mut text, "RUN START 2025-01-01 16:00:00");
        let gap = next(
            &mut text,
            "15:00:40.400000: CLI Data {'crc': 'Good', 'time_stamp_ms': 158370, 'cli_string': \
             'OK', 'x': nan}",
        );
        let log = read(&text).unwrap();
        note(
            &log,
            &format!(
                "1 packet, on line {corrupt}, is left out: the ground station marks its `crc` \
                 other than `'Good'`, its bytes"
            ),
        );
        note(&log, "2 lines are the ground station's own messages");
        // The place is a character of the line as written, its clock and name before it: the
        // second comma is its 71st character, its 72nd byte past the two-byte `é`.
        note(
            &log,
            &format!(
                "1 packet line, on line {unparsed}, doesn't parse as a Python literal: an \
                 unexpected character at character 71 of the line; skipped"
            ),
        );
        note(
            &log,
            &format!("1 line, on line {other}, opens with no time of day"),
        );
        note(
            &log,
            &format!("1 line, on line {long}, is longer than 65536 bytes"),
        );
        note(
            &log,
            &format!("1 more `RUN START` line, on line {run}, is skipped"),
        );
        note(
            &log,
            &format!(
                "1 value, on line {gap}, is `None`, `nan`, `inf` or text that isn't a number: it \
                 is a gap"
            ),
        );
        note(
            &log,
            "the fields `callsign`, `cli_string`, `radio_name` hold text",
        );
        note(
            &log,
            "1 field of the `CLI Data` packets, `x`, isn't one this reader knows: its values are \
             kept as written",
        );
        // The corrupt packet's temperature is nowhere in the record.
        let temperature = channel(&log, "Position Data.temperature");
        assert!(temperature.values.iter().all(|v| *v > 0.0));
        assert!(channel(&log, "CLI Data.x").values[0].is_nan());
    }

    /// A value its conversion makes infinite refuses the file by its line (ADR-258 §3): a roll
    /// rate of 1.7 × 10³⁰⁸ rad/s in the coast, where the checks don't look, is past the largest
    /// `f64` in °/s. Kept as written, the same value reads.
    #[test]
    fn a_value_too_large_for_its_conversion_refuses_the_file() {
        let (text, line) = changed(
            &flight(Written::default()),
            ORIENTATION,
            "'rot_vel_z': 0.5",
            "'rot_vel_z': 1.7e308",
        );
        assert_eq!(
            read(&text).unwrap_err(),
            LogError::Syntax {
                format: FORMAT,
                line,
                message: "the column Orientation.rot_vel_z holds a value too large for its unit's \
                          conversion to SI"
                    .to_owned(),
            }
        );
        let (text, _) = changed(
            &flight(Written {
                accel: 1.0 / G,
                ..Written::default()
            }),
            ORIENTATION,
            "'rot_vel_z': 0.5",
            "'rot_vel_z': 1.7e308",
        );
        let log = read(&text).unwrap();
        let kept = channel(&log, "Orientation.rot_vel_z");
        assert!(kept.values.contains(&1.7e308));
    }

    /// Positions whose fractions stay under 0.6, as degrees and minutes run together would: kept
    /// as written.
    #[test]
    fn positions_that_could_be_degrees_and_minutes_are_kept() {
        let log = read(&flight(Written {
            latitude: 41.3102,
            longitude: -70.4301,
            ..Written::default()
        }))
        .unwrap();
        note(
            &log,
            "`Position Data.fcb_latitude` and `Position Data.fcb_longitude` kept as written, their \
             range or their fractions not degrees': ",
        );
        assert_eq!(
            channel(&log, "Position Data.fcb_latitude").quantity,
            Quantity::Unrecognized
        );
    }

    /// An altitude whose barometer check holds but whose coast is a straight line: the coast
    /// refutes meters on its own, so the column is kept, and the record has no track.
    #[test]
    fn an_altitude_whose_coast_isnt_gravitys_is_kept() {
        let log = read(&flight(Written {
            straight_coast: true,
            ..Written::default()
        }))
        .unwrap();
        let kept = note(
            &log,
            "`Position Data.fcb_altitude` kept as written, the barometer and the coast not both \
             bearing meters out: ",
        );
        assert!(
            kept.contains("Meters need within 25%. Over the coast,"),
            "{kept}"
        );
        assert_eq!(
            channel(&log, "Position Data.fcb_altitude").quantity,
            Quantity::Unrecognized
        );
        assert!(log.time_s.is_empty());
        // The speed rests on the barometer and its own coast, not on the altitude.
        assert_eq!(
            channel(&log, "Position Data.v_speed").quantity,
            Quantity::Speed
        );
    }

    /// A speed in feet per second: the barometer's climb is 0.305 times its integral, and its
    /// coast slows 3.4 g; it is kept.
    #[test]
    fn a_speed_in_feet_per_second_is_kept() {
        let log = read(&flight(Written {
            speed: 1.0 / 0.3048,
            ..Written::default()
        }))
        .unwrap();
        let kept = note(
            &log,
            "`Position Data.v_speed` kept as written, the barometer and the coast not both bearing \
             meters per second out: ",
        );
        assert!(
            kept.contains("The climb is 0.30") && kept.contains(" 3.4"),
            "{kept}"
        );
        assert_eq!(
            channel(&log, "Position Data.v_speed").quantity,
            Quantity::Unrecognized
        );
        assert!(log.vertical_speed_m_s.is_none());
        assert_eq!(
            channel(&log, "Position Data.fcb_altitude").quantity,
            Quantity::AltitudeAboveGround
        );
    }

    /// A speed that integrates to the barometer's climb but holds steady through the coast: the
    /// coast refutes meters per second on its own.
    #[test]
    fn a_speed_steady_through_the_coast_is_kept() {
        let log = read(&flight(Written {
            steady_coast: true,
            ..Written::default()
        }))
        .unwrap();
        let kept = note(
            &log,
            "`Position Data.v_speed` kept as written, the barometer and the coast not both bearing \
             meters per second out: ",
        );
        let ratio = kept
            .split("The climb is ")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|r| r.parse::<f64>().ok())
            .unwrap_or(f64::NAN);
        assert!((ratio - 1.0).abs() <= MARGIN, "{kept}");
        assert_eq!(
            channel(&log, "Position Data.v_speed").quantity,
            Quantity::Unrecognized
        );
    }

    /// An accelerometer that reads 1 g at rest but three times its force in flight: the pad's
    /// gravity holds and the barometer's climb refutes it, so it is kept, and what rests on it.
    #[test]
    fn an_accelerometer_whose_climb_the_barometer_refutes_is_kept() {
        let log = read(&flight(Written {
            boost: 3.0,
            ..Written::default()
        }))
        .unwrap();
        let kept = note(
            &log,
            "`Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` kept as \
             written, the pad's gravity and the barometer not both bearing meters per second \
             squared out: ",
        );
        let ratio = kept
            .split("standard atmosphere is ")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|r| r.parse::<f64>().ok())
            .unwrap_or(f64::NAN);
        assert!(ratio < 1.0 / GRAVITY_FACTOR, "{kept}");
        assert!(kept.contains("within 10% of 9.806 65 for 6.0 s"), "{kept}");
        for column in ["Orientation.accel_z", "Orientation.angle_vertical"] {
            assert_eq!(
                channel(&log, column).quantity,
                Quantity::Unrecognized,
                "{column}"
            );
        }
    }

    /// The coasts' fits: a parabola slowing at 1 g reads 1 g in meters and 3.28 g in feet, a
    /// line the same, and a coast too short to fit decides nothing.
    #[test]
    fn the_coast_fits_tell_meters_from_feet() {
        let mut points: Vec<(f64, f64)> = (0..5).map(|k| (f64::from(k) * 0.2, 0.0)).collect();
        for k in 0..=40 {
            let t = 1.0 + 0.2 * f64::from(k);
            let s = t - 1.0;
            points.push((t, 300.0 * s + 60.0 * s - 0.5 * G * s * s));
        }
        let found = altitude_coast(&points).unwrap();
        assert!((found.deceleration - G).abs() < 1e-6, "{found:?}");
        assert!(near_gravity(found.deceleration));
        let feet: Vec<(f64, f64)> = points.iter().map(|&(t, z)| (t, z / 0.3048)).collect();
        let found = altitude_coast(&feet).unwrap();
        assert!(!near_gravity(found.deceleration), "{found:?}");
        let speeds: Vec<(f64, f64)> = (0..=30)
            .map(|k| f64::from(k) * 0.25)
            .map(|t| (t, 70.0 - G * t))
            .collect();
        let found = speed_coast(&speeds).unwrap();
        assert!((found.deceleration - G).abs() < 1e-9, "{found:?}");
        assert!(!near_gravity(found.deceleration / 0.3048));
        assert!(near_gravity(G / 1.79) && near_gravity(G * 1.79));
        assert!(!near_gravity(G / 1.81) && !near_gravity(G * 1.81) && !near_gravity(f64::NAN));
        // Four samples, or five within a second, are too few.
        assert_eq!(speed_coast(&speeds[26..]), None);
        let close: Vec<(f64, f64)> = (0..6)
            .map(|k| (0.1 * f64::from(k), 5.0 - f64::from(k)))
            .collect();
        assert_eq!(speed_coast(&close), None);
    }

    /// The clock's longest stretch, on a long steady stretch then one-packet stretches each a
    /// different gap: every stretch's span is taken once, so the work is one visit a packet.
    #[test]
    fn the_longest_stretch_is_found_in_linear_work() {
        let packets: u32 = 20_000;
        let mut slots: Vec<Slot> = (0..packets)
            .map(|k| {
                let received_s = 0.1 * f64::from(k);
                let board_ms = if k < packets / 2 {
                    received_s * 1_000.0
                } else if k % 2 == 0 {
                    0.0
                } else {
                    100_000_000.0
                };
                Slot {
                    line: 1,
                    received_s,
                    board_ms: Some(board_ms),
                    time_s: received_s,
                }
            })
            .collect();
        let stretches = stretches_of(&slots);
        assert_eq!(stretches.len(), 1 + usize::try_from(packets / 2).unwrap());
        let mut visits = 0;
        let longest = longest_stretch(&stretches, |stretch| {
            visits += stretch.len();
            #[allow(clippy::cast_precision_loss, reason = "a count of packets")]
            let span = stretch.len() as f64;
            span
        });
        assert_eq!(longest.map(|(at, _)| at), Some(0));
        assert_eq!(visits, usize::try_from(packets).unwrap());
        // And the whole check on that input reads the board's clock by its long stretch.
        let mut notes = Vec::new();
        assert!(place_on_clock(&mut slots, &mut notes));
        assert!(
            notes[0].contains("999.9 s of the ground station's clock"),
            "{notes:?}"
        );
    }

    /// A key holding a `[` that isn't a list's item takes no role by the name before it.
    #[test]
    fn a_bracket_in_a_key_isnt_a_list_item() {
        let mut text = String::from("RUN START 2025-01-01 10:00:00\n");
        for k in 0..3 {
            text.push_str(&format!(
                "10:00:0{k}.0: Position Data {{'crc': 'Good', 'fcb_altitude[1]': 5.0, \
                 'pyro_status': [1, 0]}}\n"
            ));
        }
        let log = read(&text).unwrap();
        note(
            &log,
            "1 field of the `Position Data` packets, `fcb_altitude[1]`, isn't one this reader \
             knows: ",
        );
        assert!(
            !log.notes
                .iter()
                .any(|n| n.starts_with("`Position Data.fcb_altitude")),
            "{:#?}",
            log.notes
        );
        assert_eq!(
            channel(&log, "Position Data.fcb_altitude[1]").quantity,
            Quantity::Unrecognized
        );
        // A list's items take their role by the list's name.
        assert_eq!(
            channel(&log, "Position Data.pyro_status[0]").quantity,
            Quantity::Number
        );
    }

    /// Positions in radians with one stray fix in degrees: one fix beyond ±π/2 decides nothing,
    /// so the pair is kept; most fixes must pass.
    #[test]
    fn one_stray_fix_doesnt_make_degrees() {
        let text = flight(Written {
            latitude: 0.728,
            longitude: -1.2362,
            ..Written::default()
        });
        let (text, _) = changed(
            &text,
            POSITION,
            "'fcb_latitude': 0.728,",
            "'fcb_latitude': 41.71,",
        );
        let log = read(&text).unwrap();
        let kept = note(
            &log,
            "`Position Data.fcb_latitude` and `Position Data.fcb_longitude` kept as written, their \
             range or their fractions not degrees': ",
        );
        assert!(kept.contains(", 1 beyond ±π/2 or ±π."), "{kept}");
    }

    #[test]
    fn files_that_arent_this_layout_or_hold_nothing_are_refused() {
        assert_eq!(
            read("\n\n").unwrap_err(),
            not_this("the file is empty".to_owned())
        );
        let error = read("time,alt\n").unwrap_err();
        assert!(
            matches!(&error, LogError::NotThisFormat { message, .. }
                if message.ends_with("isn't the ground station's `RUN START` line")),
            "{error}"
        );
        let error = read("RUN START sometime\n").unwrap_err();
        assert!(
            matches!(&error, LogError::Syntax { line: 1, message, .. }
                if message.starts_with("the `RUN START` line, ")),
            "{error}"
        );
        assert_eq!(
            read("RUN START 2025-01-01 10:00:00\n10:00:01.0: Switching radio to band 1\n")
                .unwrap_err(),
            LogError::NoData { format: FORMAT }
        );
        assert!(recognizes("\n  RUN START 2025-01-01 10:00:00\n"));
        assert!(!recognizes("timestampMs\tpressurePa\n"));
    }

    /// More fields than [`MAX_CHANNELS`]: the rest are counted, not kept.
    #[test]
    fn channels_are_bounded() {
        let mut text = String::from("RUN START 2025-01-01 10:00:00\n");
        for (second, packet) in ["A", "B"].into_iter().enumerate() {
            let fields: Vec<String> = (0..600).map(|k| format!("'f{k}': 1")).collect();
            text.push_str(&format!(
                "10:00:0{second}.0: {packet} {{{}}}\n",
                fields.join(", ")
            ));
        }
        let log = read(&text).unwrap();
        assert_eq!(log.channels.len(), MAX_CHANNELS);
        note(&log, "176 values, the first on line 3, are left out");
        note(&log, "600 fields of the `A` packets");
        note(&log, "424 fields of the `B` packets");
    }

    proptest::proptest! {
        /// Any text parses or is refused, never a panic.
        #[test]
        fn any_literal_parses_or_is_refused(text in "[{}\\[\\](),:'\"\\\\a-zA-Z0-9.+\\- ]{0,64}") {
            let _ = parse_literal(&text);
        }

        /// Any packets after a `RUN START` line are read or refused, never a panic; what is read
        /// has a strictly increasing track and as many times as values in every channel.
        #[test]
        fn any_text_is_read_or_refused(
            lines in proptest::collection::vec(
                "(1[0-2]:[0-5][0-9]:[0-5][0-9]\\.[0-9]{1,3}: )?(Position Data|Alt Info & Cfg|Orientation|X)? ?\\{?('[a-z_]{1,14}': ?[-0-9.e,'\\[\\]TrueFalsNoan]{0,10}, ?){0,4}\\}?",
                0..20,
            )
        ) {
            let text = format!("RUN START 2025-01-01 10:00:00\n{}", lines.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
            }
        }
    }
}
