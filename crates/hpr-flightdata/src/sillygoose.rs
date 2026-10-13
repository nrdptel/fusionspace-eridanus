//! AerospaceNU's SillyGoose flight computer's logs, as the board writes them: text under a
//! `timestampMs` header, one row per sample.
//!
//! The SillyGoose is a flight computer of Northeastern University's rocketry club, AerospaceNU,
//! that logs its barometer, its accelerometer and gyroscope, its own altitude and vertical-speed
//! estimates and its pyro channels' state. The club publishes its flights' logs with a README
//! that names each file after its flight and tags it, among others, `pre reboot` and `post
//! reboot` where the board restarted in flight and its log is in two files, and `doctored` where
//! the file was altered after it was offloaded (AerospaceNU, *rocket-flight-data*, `README.md`,
//! read 2026-10-12). No published document describes the columns, so each unit here comes from
//! the column's name or from a check the reader runs on the file's values, as below. Three
//! layouts are among the project's sample logs, all read alike:
//!
//! - *The board's log*: tab-separated, 20 columns, about 100 samples a second in flight and one
//!   every 5 s on the pad (here with the tabs shown as spaces):
//!
//!   ```text
//!   timestampMs  pressurePa  tempK  accelX  accelY  accelZ  gyroX  gyroY  gyroZ  imuTemp  …
//!   1000  97716.500000  290.15  0.012000  -0.020000  9.810000  0.001064  …
//!   ```
//!
//! - *With its settings*: a `CONFIG` line of `KEY=VALUE` settings, then the same header with 8
//!   more columns, `tiltMagnitudeDeg`, `angularVelRadS_x` … `_z` and `quaternion_a` … `_d`. The one
//!   such sample file looks like replayed output, not a recording: after a few samples on the pad
//!   it jumps into the boost, it ends still climbing, and its temperature holds one value
//!   throughout (a note says so; below).
//! - *Space-separated*: the 20 columns' header with single spaces between the names, at 50
//!   samples a second, whose values don't all agree with the units the names state; the checks
//!   below find which do.
//!
//! **Recognition.** The first line that isn't blank, or the one after it if that first line opens
//! with `CONFIG`, is a header whose first name is `timestampMs` and which names `pressurePa` and
//! `altitudeM`, compared without regard to case. A header with a tab has its names and its rows'
//! cells split at tabs; one without, at runs of spaces. The club's ground station's conversion
//! of a downlink, a `CONFIG` line and then comma-separated rows with no header, is not this
//! layout and isn't read.
//!
//! **Columns.** Each name the reader knows, what it is read as, and where its unit comes from;
//! the column keeps the file's name. A column the reader doesn't know is kept as written,
//! [`Quantity::Unrecognized`], with a note; a header naming a column twice, or with an empty
//! name, is refused:
//!
//! | Column | Quantity | Unit, and from where |
//! |---|---|---|
//! | `timestampMs` | the clock | milliseconds, from its name; the board's own zero, before its first sample |
//! | `pressurePa` | [`Quantity::Pressure`] | Pa, from its name, if its first value is a pressure near the ground in pascals |
//! | `tempK` | [`Quantity::Temperature`]: [`FlightLog::temperature_k`] | K, from its name, if every value is one an electronics bay can have in kelvin |
//! | `accelX`, `accelY`, `accelZ` | [`Quantity::Acceleration`] | m/s² or standard gravities, from the pad's gravity (below) |
//! | `gyroX`, `gyroY`, `gyroZ` | [`Quantity::AngularRate`] | radians or degrees per second, from the board's turning (below) |
//! | `imuTemp` | [`Quantity::Temperature`] | K, from the values (below) |
//! | `battV` | [`Quantity::Voltage`]: [`FlightLog::battery_v`] | V, from its name, if every value lies within 0 to 20 V |
//! | `altitudeM` | [`Quantity::AltitudeAboveGround`]: the primary track | m, from its name, checked against the pressure (below) |
//! | `velocityMS` | [`Quantity::Speed`], up positive: [`FlightLog::vertical_speed_m_s`] | m/s, from its name, checked against the altitude (below) |
//! | `accelerationMSS` | [`Quantity::Acceleration`] | m/s², from its name: the board's own estimate, along an axis the file doesn't state |
//! | `unfiltAlt` | [`Quantity::AltitudeAboveGround`] | m, from the pressure (below); the zero is the board's, and not stated |
//! | `flightState` | [`Quantity::Number`] | the board's state code, as written: no published source gives the codes' meanings |
//! | `drogueCont`, `mainCont` | [`Quantity::Number`] | 0 or 1, as written: the pyro channels' continuity, by their names |
//! | `drogueFired`, `mainFired` | [`Quantity::Number`] | 0 or 1, as written; each rise from 0 to 1 is an event (below) |
//! | `tiltMagnitudeDeg` | [`Quantity::Angle`] | degrees, from its name, if every value lies within 0 to 180° |
//! | `angularVelRadS_x`, `_y`, `_z` | [`Quantity::AngularRate`] | radians per second, from their names, if no value is beyond what a gyroscope measures (below) |
//! | `quaternion_a` … `_d` | [`Quantity::Number`] | the parts of an attitude quaternion, which have no unit |
//!
//! A column whose values refute the unit its name states is kept as written,
//! [`Quantity::Unrecognized`], with a note saying why; a column whose unit the values decide gets
//! a note saying on what evidence. The checks:
//!
//! - *The pressure* is read in pascals if its first value lies within 50,000 to 110,000 Pa (the
//!   bounds [`crate::rrc3`] argues for a pad's pressure). Every sample file's does.
//! - *The temperatures.* An electronics bay's temperature lies within −40 to 60 °C, 233.15 to
//!   333.15 K. `tempK` is read in kelvin, as its name says, if every value lies within that range;
//!   `imuTemp`'s name states no unit, and it is read in kelvin on the same evidence: in degrees
//!   Celsius or Fahrenheit the same values would be hotter than any bay survives. Otherwise each is
//!   kept as written. In the space-separated sample `tempK` reads below zero, which no temperature
//!   in kelvin can, and nothing in the file says in which unit it is; there and in the file with
//!   its settings `imuTemp` is 0 at every sample, a placeholder rather than a reading.
//! - *The pad's gravity.* A board at rest, or hanging under a parachute, reads 1 g: 9.806 65 in
//!   m/s², 1 in standard gravities. The reader adds up the time the three axes' magnitude lies
//!   within 10% of each of the two, every sample counting until the next. The unit whose 1 g
//!   lasts at least 1 s and at least twice as long as the other's is taken; otherwise the axes
//!   are kept as written. In the board's sample logs the magnitude reads 9.8 for minutes on the
//!   pad and under the parachutes, against at most 1 s near 1 m/s² in the coast; in the
//!   space-separated one it reads 1 for 46 s and never 9.8, so its axes are read in g, × 9.806 65.
//! - *The board's turning.* With the board at about 1 g (within 15%), the direction the
//!   accelerometer reads is gravity's, so as the board turns, that direction turns by as much as
//!   the gyroscope's rates across it say. Over stretches of 10 steps at most 0.05 s apart, the
//!   reader adds up the angle between the accelerometer's direction at each stretch's ends and the
//!   gyroscope's turn across that direction, |ω × â| dt, with ω read in radians per second. The
//!   unit, radians or degrees per second, in which the gyroscope's turn lies within a quarter to 4
//!   times the accelerometer's is taken, given at least 3 stretches and 0.2 rad of turning; as the
//!   units are 57.3 times apart, at most one can be. Otherwise the gyroscope is kept as written.
//!   Swinging under a parachute turns the accelerometer's direction without turning the board, so
//!   the ratio often runs below 1: in radians per second it is 0.40 to 1.6 in the 11 of the board's
//!   own logs with stretches to check, and in degrees per second 1.3 in the other two, whose rates
//!   reach about 1,000; the 42-sample log, all on the pad, has none, and its gyroscope is kept as
//!   written.
//! - *`angularVelRadS_*`* are read in radians per second, as their names say, if no value's
//!   magnitude passes 349 rad/s, the 20,000 °/s full scale of Analog Devices' ADXRS649, a
//!   gyroscope made for very high rates (*ADXRS649* data sheet). In the file with its settings they
//!   pass it, so they are kept as written.
//! - *`altitudeM`* is checked against the pressure: from the first sample to the one of lowest
//!   pressure, the climb in the U.S. Standard Atmosphere, 1976 ([`hpr_atmos::ussa76`]), against
//!   the column's. Within 25%, meters are borne out; beyond, the file is refused
//!   ([`LogError::Unit`]): the primary track has no unit to be read in. Under 10 m of climb, or
//!   without a pressure in pascals, the check can't be made and a note says so. `unfiltAlt`,
//!   whose name states no unit, is read in meters on the same check, and otherwise kept as
//!   written.
//! - *`velocityMS`* is checked against `altitudeM`: over the steps at most 0.05 s long, the least
//!   squares slope of the altitude's change on the speed times the step must lie within 25% of 1.
//!   This ties the clock, the speed and the altitude together: a speed in feet per second, or a
//!   clock in seconds, would be 3.28 or 1,000 times off. Under 10 m of movement the check can't be
//!   made and a note says so; a slope beyond is kept as written. In the sample files it is 0.86 to
//!   1.08.
//!
//! **The primary track.** The record's altitude is `altitudeM`, the board's own altitude above its
//! zero, the pad: in the sample files that start on the pad it reads within 2.1 m of zero at the
//! first sample, but for the replayed file's 5 m. It is a filtered estimate, smoother than the
//! pressure; `unfiltAlt`, the board's altitude before filtering, is on a zero the file doesn't
//! state (in the board's logs
//! within 150 m of the standard atmosphere's altitude of the pressure, in the space-separated file
//! the same as `altitudeM`'s), so it isn't the track. The vertical speed is `velocityMS`, the
//! temperature `tempK` and the battery `battV`, where they are read in their units. The track
//! takes the samples in time order, the first at each time, leaving out any sample without a time;
//! a note counts those put in order or left out, and the channels keep every one in the file's
//! order. Where the altitude reads more than 100 m off zero at the track's first sample, as in the
//! file after a restart in flight, a note says so: each of the two files is its own record.
//!
//! **Events.** Each time `drogueFired` or `mainFired` goes from 0 to 1 is an [`Event`] named by its
//! column, at the sample's time, in time order: the names say the board fired that channel. A flag
//! already 1 at its first sample is no event. `flightState` changes are no events: the codes'
//! meanings aren't published.
//!
//! **The settings.** In a `CONFIG` line, `BOARD_NAME` names the logger and `FIRMWARE_VERSION` the
//! firmware; the other settings are left out, counted in a note, as no source gives their units.
//! Without a `BOARD_NAME` the logger is `AerospaceNU SillyGoose`.
//!
//! **Cells.** An empty cell, or one that isn't a number, is a gap (`NaN`), counted in a note. A row
//! with fewer cells than the header names has the rest as gaps, counted in a note; a row with more
//! is refused, naming its line, as is a file whose rows hold under a quarter of the cells the header
//! names (past the first 65,536), so that a short file can't make the reader store far more gaps
//! than it writes cells. Lines may end in CR, LF or both, blank lines are skipped, and a byte-order
//! mark is dropped.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use hpr_atmos::ussa76::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{Notes, number, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "SillyGoose text";

/// The logger's name where the file states none.
const LOGGER: &str = "AerospaceNU SillyGoose";

/// The clock's name, the header's first.
const CLOCK: &str = "timestampMs";

/// The names a header must hold besides the clock to be recognized.
const MARKS: [&str; 2] = ["pressurePa", "altitudeM"];

/// The opening word of the settings line.
const CONFIG: &str = "CONFIG";

/// The step between two altitudes the format can tell apart near the ground, m
/// ([`crate::log::LogFormat::altitude_resolution_m`]): the board writes `altitudeM` to the
/// centimeter, and it steps by that, 0.01 m, near the ground in every one of the 12 board's logs
/// among the project's sample logs (2026-10-13), as in the file with its settings. The
/// space-separated file writes four decimals but steps by at least 0.08 m, its pressure being in
/// whole pascals; the step the board's own layout writes is taken, so that a tolerance built on it
/// stays tight.
pub(crate) const ALTITUDE_STEP_M: f64 = 0.01;

/// A pressure near the ground in pascals, the bounds [`crate::rrc3`] argues for a pad's.
const PAD_PA: (f64, f64) = (50_000.0, 110_000.0);

/// An electronics bay's temperatures, K: −40 to 60 °C, a winter launch to a bay in the desert sun.
const BAY_K: (f64, f64) = (233.15, 333.15);

/// The most a battery may read, V, to be read in volts: a board's battery is a few volts, and
/// millivolts would be thousands.
const VOLTS_MAX: f64 = 20.0;

/// How far, as a share, the accelerometer's magnitude may lie from 1 g to count as reading it.
const GRAVITY_MARGIN: f64 = 0.1;

/// The least time, s, the file must read 1 g in a unit for that unit to be taken.
const GRAVITY_LEAST_S: f64 = 1.0;

/// How many times longer the taken unit's 1 g must last than the other's.
const GRAVITY_LEAD: f64 = 2.0;

/// The longest step, s, of the board's fast logging that the turning and speed checks use: 10 ms
/// or 20 ms in the sample files, where the pad's are 5 s.
const FAST_STEP_S: f64 = 0.05;

/// The steps a stretch of the turning check spans.
const TURN_STEPS: usize = 10;

/// How far, as a share, the accelerometer may read from 1 g in a stretch of the turning check.
const TURN_GRAVITY_MARGIN: f64 = 0.15;

/// The least the accelerometer's direction must turn, rad, over the stretches for the check to
/// decide.
const TURN_LEAST_RAD: f64 = 0.2;

/// The fewest stretches for the turning check to decide.
const TURN_LEAST_STRETCHES: usize = 3;

/// How many times off, either way, the gyroscope's turn may be from the accelerometer's for a
/// unit to be taken: degrees and radians are 57.3 times apart, so at most one unit is within it.
const TURN_FACTOR: f64 = 4.0;

/// The fastest rate a gyroscope measures, rad/s: 20,000 °/s, the full scale of Analog Devices'
/// ADXRS649 (*ADXRS649* data sheet), a gyroscope made for very high rates.
const RATE_MAX_RAD_S: f64 = 20_000.0 * PI / 180.0;

/// The least climb, m, from which the pressure checks an altitude's unit: twice the 5 m of noise
/// and drift a board's altitude has on the pad, and enough for meters and feet, 3.28 times apart,
/// to be told apart within [`CLIMB_MARGIN`].
const CHECK_CLIMB_M: f64 = 10.0;

/// How far, as a share, an altitude's climb may be from the pressure's for meters to be borne
/// out.
const CLIMB_MARGIN: f64 = 0.25;

/// How far the speed check's slope may lie from 1.
const SPEED_MARGIN: f64 = 0.25;

/// The least movement, m, the speed integrates over the fast steps for the speed check to decide.
const SPEED_LEAST_M: f64 = 10.0;

/// How far, m, the altitude may read from zero at the track's first sample before a note says the
/// record doesn't start at the board's zero.
const GROUND_OFF_M: f64 = 100.0;

/// The least climb, m, over which a temperature that never changes gets a note.
const STILL_CLIMB_M: f64 = 100.0;

/// The most events a log keeps; past it, a note counts the rest.
const MAX_EVENTS: usize = 1_000;

/// What a column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Pressure,
    /// `tempK`.
    Temperature,
    /// One axis of the accelerometer: 0 for x, 1 for y, 2 for z.
    Accel(usize),
    /// One axis of the gyroscope.
    Gyro(usize),
    /// `imuTemp`.
    ImuTemperature,
    Battery,
    /// `altitudeM`.
    Altitude,
    /// `velocityMS`.
    Speed,
    /// `accelerationMSS`, the board's estimate.
    Estimate,
    /// `unfiltAlt`.
    Unfiltered,
    /// `flightState`.
    State,
    /// A pyro channel's continuity.
    Continuity,
    /// A pyro channel's fired flag: its rises are events.
    Fired,
    Tilt,
    AngularVelocity,
    Quaternion,
    Unknown,
}

/// The names the reader knows, but the clock, and what each is.
const KNOWN: [(&str, Role); 27] = [
    ("pressurePa", Role::Pressure),
    ("tempK", Role::Temperature),
    ("accelX", Role::Accel(0)),
    ("accelY", Role::Accel(1)),
    ("accelZ", Role::Accel(2)),
    ("gyroX", Role::Gyro(0)),
    ("gyroY", Role::Gyro(1)),
    ("gyroZ", Role::Gyro(2)),
    ("imuTemp", Role::ImuTemperature),
    ("battV", Role::Battery),
    ("altitudeM", Role::Altitude),
    ("velocityMS", Role::Speed),
    ("accelerationMSS", Role::Estimate),
    ("unfiltAlt", Role::Unfiltered),
    ("flightState", Role::State),
    ("drogueCont", Role::Continuity),
    ("mainCont", Role::Continuity),
    ("drogueFired", Role::Fired),
    ("mainFired", Role::Fired),
    ("tiltMagnitudeDeg", Role::Tilt),
    ("angularVelRadS_x", Role::AngularVelocity),
    ("angularVelRadS_y", Role::AngularVelocity),
    ("angularVelRadS_z", Role::AngularVelocity),
    ("quaternion_a", Role::Quaternion),
    ("quaternion_b", Role::Quaternion),
    ("quaternion_c", Role::Quaternion),
    ("quaternion_d", Role::Quaternion),
];

/// What a column is, by its name, compared without regard to case.
fn role_of(name: &str) -> Role {
    KNOWN
        .iter()
        .find(|(known, _)| known.eq_ignore_ascii_case(name))
        .map_or(Role::Unknown, |(_, role)| *role)
}

/// A line's cells: split at tabs, trimmed, if `tabs`; else at runs of spaces.
fn cells(line: &str, tabs: bool) -> Box<dyn Iterator<Item = &str> + '_> {
    if tabs {
        Box::new(line.split('\t').map(str::trim))
    } else {
        Box::new(line.split_whitespace())
    }
}

/// Whether a line, trimmed, is a header of this layout: `timestampMs` first, and [`MARKS`]. The
/// walk stops once both are seen, and copies nothing.
fn is_header(line: &str) -> bool {
    let mut names = cells(line, line.contains('\t'));
    if !names
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case(CLOCK))
    {
        return false;
    }
    let mut seen = [false; MARKS.len()];
    for name in names {
        for (mark, seen) in MARKS.iter().zip(&mut seen) {
            *seen |= name.eq_ignore_ascii_case(mark);
        }
        if seen.iter().all(|seen| *seen) {
            return true;
        }
    }
    false
}

/// Whether a line, trimmed, is the settings line: `CONFIG`, then its settings.
fn is_config(line: &str) -> bool {
    cells(line, line.contains('\t')).next() == Some(CONFIG)
}

/// Whether the text is this layout: its first line that isn't blank is a header (module docs), or
/// a `CONFIG` line with a header after it.
pub fn recognizes(text: &str) -> bool {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|line| !line.is_empty());
    match lines.next() {
        Some(first) if is_config(first) => lines.next().is_some_and(is_header),
        Some(first) => is_header(first),
        None => false,
    }
}

/// One data column.
struct Column {
    name: String,
    role: Role,
    /// The values as written, `NaN` for a gap.
    values: Vec<f64>,
    /// What the column is read as.
    quantity: Quantity,
    /// The factor from the values as written to the quantity's unit.
    scale: f64,
}

impl Column {
    /// The column, read as its role's quantity in its name's unit until a check decides otherwise.
    fn new(name: &str) -> Self {
        let role = role_of(name);
        let (quantity, scale) = match role {
            Role::Pressure => (Quantity::Pressure, 1.0),
            Role::Temperature | Role::ImuTemperature => (Quantity::Temperature, 1.0),
            Role::Accel(_) | Role::Estimate => (Quantity::Acceleration, 1.0),
            Role::Gyro(_) => (Quantity::AngularRate, 1.0),
            Role::AngularVelocity => (Quantity::AngularRate, 180.0 / PI),
            Role::Battery => (Quantity::Voltage, 1.0),
            Role::Altitude | Role::Unfiltered => (Quantity::AltitudeAboveGround, 1.0),
            Role::Speed => (Quantity::Speed, 1.0),
            Role::Tilt => (Quantity::Angle, 1.0),
            Role::State | Role::Continuity | Role::Fired | Role::Quaternion => {
                (Quantity::Number, 1.0)
            }
            Role::Unknown => (Quantity::Unrecognized, 1.0),
        };
        Self {
            name: name.to_owned(),
            role,
            values: Vec::new(),
            quantity,
            scale,
        }
    }

    /// Keeps the column as written.
    fn keep(&mut self) {
        self.quantity = Quantity::Unrecognized;
        self.scale = 1.0;
    }

    /// The column's name, cut short, in backticks.
    fn quoted(&self) -> String {
        format!("`{}`", short(&self.name))
    }

    /// The lowest and highest of the values, if any is a number.
    fn range(&self) -> Option<(f64, f64)> {
        let mut finite = self.values.iter().copied().filter(|v| v.is_finite());
        let first = finite.next()?;
        Some(finite.fold((first, first), |(lo, hi), v| (lo.min(v), hi.max(v))))
    }
}

/// Reads a SillyGoose log from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank, or the one after a `CONFIG`
/// line, isn't this layout's header; [`LogError::Syntax`] for a header with an empty name or a
/// name given twice, a row with more cells than the header names, or a file whose rows hold too
/// few of the cells its header names; [`LogError::NoData`] if no row has a time;
/// [`LogError::Unit`] if `altitudeM`'s climb is more than 25% off the pressure's (module docs).
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some(first) = lines.next() else {
        return Err(not_this("the file is empty".to_owned()));
    };
    let (config, (header_line, header)) = if is_config(first.1.trim()) {
        let Some(next) = lines.next() else {
            return Err(not_this(
                "the file holds a `CONFIG` line and nothing after it".to_owned(),
            ));
        };
        (Some(first.1.trim()), next)
    } else {
        (None, first)
    };
    let header = header.trim();
    if !is_header(header) {
        return Err(not_this(format!(
            "its header, {:?}, isn't a SillyGoose log's: `timestampMs` first, then names such as \
             `pressurePa` and `altitudeM`",
            short(header)
        )));
    }
    let tabs = header.contains('\t');

    let mut columns: Vec<Column> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (index, name) in cells(header, tabs).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                header_line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(syntax(
                header_line,
                format!("the header names {:?} twice", short(name)),
            ));
        }
        if index > 0 {
            columns.push(Column::new(name));
        }
    }
    let width = columns.len() + 1;

    let mut times: Vec<f64> = Vec::new();
    let mut gaps = 0usize;
    let mut first_gap = None;
    let mut short_rows = 0usize;
    let mut first_short = None;
    let mut cells_written = 0usize;
    for (number_of_line, line) in lines {
        let row: Vec<&str> = cells(line.trim_end(), tabs).collect();
        if row.len() > width {
            return Err(syntax(
                number_of_line,
                format!(
                    "the row has {} cells where the header names {width}",
                    row.len()
                ),
            ));
        }
        if row.len() < width {
            short_rows += 1;
            first_short.get_or_insert(number_of_line);
        }
        cells_written = cells_written.saturating_add(row.len());
        let mut value_at = |index: usize| match row.get(index) {
            Some(cell) => number(cell, false).unwrap_or_else(|| {
                gaps += 1;
                first_gap.get_or_insert(number_of_line);
                f64::NAN
            }),
            None => f64::NAN,
        };
        times.push(value_at(0) / 1_000.0);
        for (index, column) in columns.iter_mut().enumerate() {
            column.values.push(value_at(index + 1));
        }
        if crate::log::too_sparse(width, times.len(), cells_written) {
            return Err(syntax(
                number_of_line,
                format!(
                    "{} rows hold {cells_written} of the {} cells the header's {width} columns \
                     name: too few for the header to be this file's",
                    times.len(),
                    times.len().saturating_mul(width)
                ),
            ));
        }
    }
    if !times.iter().any(|time| time.is_finite()) {
        return Err(LogError::NoData { format: FORMAT });
    }
    let Some(primary) = columns.iter().position(|c| c.role == Role::Altitude) else {
        // Invariant: the header was recognized, so it names `altitudeM`.
        return Err(syntax(
            header_line,
            "the file has no `altitudeM`".to_owned(),
        ));
    };

    let mut decisions: Vec<String> = Vec::new();
    let pressure_pa = decide_pressure(&mut columns, &mut decisions);
    decide_temperatures(&mut columns, &mut decisions);
    let gravity = decide_accelerometer(&mut columns, &times, &mut decisions);
    decide_gyroscope(&mut columns, &times, gravity, &mut decisions);
    decide_by_range(&mut columns, &mut decisions);
    decide_altitudes(&mut columns, pressure_pa, header_line, &mut decisions)?;
    decide_speed(&mut columns, &times, &mut decisions);
    for column in &columns {
        if column.role == Role::Unknown {
            decisions.push(format!(
                "the column {} isn't one this reader knows; its values are kept as written",
                column.quoted()
            ));
        }
    }

    let (logger, firmware, config_note) = settings(config);

    // Every channel, in its unit.
    let channels: Vec<Channel> = columns
        .iter()
        .map(|column| Channel {
            column: column.name.clone(),
            quantity: column.quantity,
            time_s: times.clone(),
            values: column.values.iter().map(|v| v * column.scale).collect(),
        })
        .collect();

    // The events: the fired flags' rises.
    let mut events = Vec::new();
    let mut events_left_out = 0usize;
    for column in columns.iter().filter(|c| c.role == Role::Fired) {
        let mut before: Option<f64> = None;
        for (value, time) in column.values.iter().zip(&times) {
            if !value.is_finite() {
                continue;
            }
            if before == Some(0.0) && *value == 1.0 && time.is_finite() {
                if events.len() < MAX_EVENTS {
                    events.push(Event {
                        time_s: *time,
                        name: column.name.clone(),
                    });
                } else {
                    events_left_out += 1;
                }
            }
            before = Some(*value);
        }
    }
    events.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));

    // The primary track: the samples in time order, the first at each time.
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
    let on_track = |index: usize| -> Vec<f64> {
        order
            .iter()
            .map(|&row| channels[index].values[row])
            .collect()
    };
    let time_s: Vec<f64> = order.iter().map(|&row| times[row]).collect();
    let altitude_m = on_track(primary);
    let track_of = |role: Role, quantity: Quantity| {
        columns
            .iter()
            .position(|c| c.role == role && c.quantity == quantity)
            .map(on_track)
    };
    let vertical_speed_m_s = track_of(Role::Speed, Quantity::Speed);
    let temperature_k = track_of(Role::Temperature, Quantity::Temperature);
    let battery_v = track_of(Role::Battery, Quantity::Voltage);

    let mut notes = decisions;
    notes.extend(config_note);
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
            "`{}` reads {ground:.2} m at the track's first sample, not near zero: the record \
             starts away from the board's zero, as a log after a restart in flight does",
            short(&columns[primary].name)
        ));
    }
    notes.extend(still_temperature(&columns, primary));
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
    if events_left_out > 0 {
        line_notes.push(format!(
            "and {events_left_out} more rises of the fired flags, left out of the events so that \
             the list stays short"
        ));
    }
    notes.extend(line_notes.into_vec());

    Ok(FlightLog {
        format: LogFormat::SillyGooseText,
        logger,
        serial_number: None,
        firmware,
        flight_number: None,
        stated: Stated::default(),
        time_s,
        altitude_m,
        vertical_speed_m_s,
        temperature_k,
        battery_v,
        channels,
        events,
        notes,
    })
}

/// The column with a role, if the file has it.
fn find(columns: &[Column], role: Role) -> Option<usize> {
    columns.iter().position(|c| c.role == role)
}

/// Reads the pressure in pascals if its first value is a pressure near the ground in them; whether
/// it is.
fn decide_pressure(columns: &mut [Column], decisions: &mut Vec<String>) -> bool {
    let Some(at) = find(columns, Role::Pressure) else {
        return false;
    };
    let column = &mut columns[at];
    match column.values.iter().copied().find(|v| v.is_finite()) {
        Some(first) if (PAD_PA.0..=PAD_PA.1).contains(&first) => true,
        first => {
            let why = first.map_or_else(
                || "it holds no number".to_owned(),
                |first| {
                    format!(
                        "its first value, {first}, is no pressure near the ground in them \
                         (50,000 to 110,000 Pa)"
                    )
                },
            );
            decisions.push(format!(
                "{} kept as written: its name says pascals, but {why}",
                column.quoted()
            ));
            column.keep();
            false
        }
    }
}

/// Whether every value of a column is a temperature an electronics bay can have in kelvin, and at
/// least one is a number.
fn bay_kelvin(column: &Column) -> bool {
    column
        .range()
        .is_some_and(|(lo, hi)| lo >= BAY_K.0 && hi <= BAY_K.1)
}

/// Why a column's values aren't a bay's temperatures in kelvin.
fn not_kelvin(column: &Column) -> String {
    match column.range() {
        None => "it holds no number".to_owned(),
        Some((lo, hi)) if lo == 0.0 && hi == 0.0 => {
            "every value is exactly 0, no temperature in kelvin, and a placeholder rather than a \
             reading in any unit"
                .to_owned()
        }
        Some((lo, hi)) => format!(
            "its values run from {lo} to {hi}, outside the 233.15 to 333.15 K (−40 to 60 °C) an \
             electronics bay can have{}",
            if lo < 0.0 {
                ", and below 0 no temperature in kelvin can be"
            } else {
                ""
            }
        ),
    }
}

/// Reads `tempK` in kelvin, as its name says, and `imuTemp`, by its values, if they are a bay's;
/// otherwise keeps each as written.
fn decide_temperatures(columns: &mut [Column], decisions: &mut Vec<String>) {
    for column in columns.iter_mut() {
        match column.role {
            Role::Temperature if !bay_kelvin(column) => {
                decisions.push(format!(
                    "{} kept as written: its name says kelvin, but {}; nothing in the file says \
                     which unit it is in",
                    column.quoted(),
                    not_kelvin(column)
                ));
                column.keep();
            }
            Role::ImuTemperature => {
                if let (true, Some((lo, hi))) = (bay_kelvin(column), column.range()) {
                    decisions.push(format!(
                        "{} read in kelvin, from the values: they run from {lo} to {hi}, within \
                         the 233.15 to 333.15 K an electronics bay can have, where in degrees \
                         Celsius or Fahrenheit they would be hotter than any bay survives",
                        column.quoted()
                    ));
                } else {
                    decisions.push(format!(
                        "{} kept as written: its name states no unit, and {}",
                        column.quoted(),
                        not_kelvin(column)
                    ));
                    column.keep();
                }
            }
            _ => {}
        }
    }
}

/// The three axes of a sensor, by role, if the file has all three.
fn axes(columns: &[Column], role: fn(usize) -> Role) -> Option<[usize; 3]> {
    Some([
        find(columns, role(0))?,
        find(columns, role(1))?,
        find(columns, role(2))?,
    ])
}

/// A sample of three axes, as written.
fn vector(columns: &[Column], axes: [usize; 3], row: usize) -> [f64; 3] {
    axes.map(|at| columns[at].values[row])
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// The names of the columns at some places, as a sentence.
fn listed_at(columns: &[Column], at: &[usize]) -> String {
    let quoted: Vec<String> = at.iter().map(|&i| columns[i].quoted()).collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// Decides the accelerometer's unit from the pad's gravity (module docs): the value 1 g reads as,
/// 9.806 65 or 1, if decided.
fn decide_accelerometer(
    columns: &mut [Column],
    times: &[f64],
    decisions: &mut Vec<String>,
) -> Option<f64> {
    let axes = axes(columns, Role::Accel)?;
    let mut in_ms2 = 0.0;
    let mut in_g = 0.0;
    for row in 0..times.len().saturating_sub(1) {
        let step = times[row + 1] - times[row];
        let reading = magnitude(vector(columns, axes, row));
        if !(step > 0.0 && step.is_finite() && reading.is_finite()) {
            continue;
        }
        if (reading - STANDARD_GRAVITY_MPS2).abs() <= GRAVITY_MARGIN * STANDARD_GRAVITY_MPS2 {
            in_ms2 += step;
        }
        if (reading - 1.0).abs() <= GRAVITY_MARGIN {
            in_g += step;
        }
    }
    let names = listed_at(columns, &axes);
    let evidence = format!(
        "the three axes' magnitude reads within 10% of 9.806 65 for {in_ms2:.1} s of the file's \
         time and within 10% of 1 for {in_g:.1} s; a board at rest or under a parachute reads \
         1 g, and the unit whose 1 g lasts at least 1 s and twice as long as the other's is taken"
    );
    let decided = if in_ms2 >= GRAVITY_LEAST_S && in_ms2 >= GRAVITY_LEAD * in_g {
        decisions.push(format!(
            "{names} read in meters per second squared, from the pad's gravity: {evidence}"
        ));
        Some((STANDARD_GRAVITY_MPS2, 1.0))
    } else if in_g >= GRAVITY_LEAST_S && in_g >= GRAVITY_LEAD * in_ms2 {
        decisions.push(format!(
            "{names} read in standard gravities, × 9.806 65 m/s², from the pad's gravity: \
             {evidence}"
        ));
        Some((1.0, STANDARD_GRAVITY_MPS2))
    } else {
        decisions.push(format!(
            "{names} kept as written: the pad's gravity doesn't settle their unit: {evidence}"
        ));
        None
    };
    for at in axes {
        match decided {
            Some((_, scale)) => columns[at].scale = scale,
            None => columns[at].keep(),
        }
    }
    decided.map(|(one_g, _)| one_g)
}

/// What the turning check found.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Turning {
    /// The stretches it used.
    stretches: usize,
    /// The accelerometer's direction's turn over them, rad.
    accelerometer_rad: f64,
    /// The gyroscope's turn across that direction, its rates read in rad/s, rad.
    gyroscope_rad: f64,
}

/// The turning check (module docs): over stretches of [`TURN_STEPS`] fast steps where the
/// accelerometer reads about `one_g`, the accelerometer's direction's turn and the gyroscope's.
fn turning(
    columns: &[Column],
    times: &[f64],
    accel: [usize; 3],
    gyro: [usize; 3],
    one_g: f64,
) -> Turning {
    let mut found = Turning {
        stretches: 0,
        accelerometer_rad: 0.0,
        gyroscope_rad: 0.0,
    };
    let near_g = |row: usize| {
        let reading = magnitude(vector(columns, accel, row));
        reading.is_finite() && (reading - one_g).abs() <= TURN_GRAVITY_MARGIN * one_g
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
        let (from, to) = (direction(start), direction(end));
        let cosine = (from[0] * to[0] + from[1] * to[1] + from[2] * to[2]).clamp(-1.0, 1.0);
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

/// Decides the gyroscope's unit from the board's turning (module docs).
fn decide_gyroscope(
    columns: &mut [Column],
    times: &[f64],
    one_g: Option<f64>,
    decisions: &mut Vec<String>,
) {
    let Some(gyro) = axes(columns, Role::Gyro) else {
        return;
    };
    let names = listed_at(columns, &gyro);
    let accel = axes(columns, Role::Accel);
    let (Some(accel), Some(one_g)) = (accel, one_g) else {
        decisions.push(format!(
            "{names} kept as written: their unit is decided from the accelerometer's gravity as \
             the board turns, and the file has no accelerometer read in a known unit"
        ));
        for at in gyro {
            columns[at].keep();
        }
        return;
    };
    let found = turning(columns, times, accel, gyro, one_g);
    let ratio = found.gyroscope_rad / found.accelerometer_rad;
    let degrees = PI / 180.0;
    let evidence = format!(
        "over {} stretches of {TURN_STEPS} steps at most {FAST_STEP_S} s apart with the \
         accelerometer within 15% of 1 g, as at rest or under a parachute, its direction turns \
         {:.3} rad, and the gyroscope's rates turn the board across it {:.3} rad read in radians \
         per second, {:.3} rad read in degrees",
        found.stretches,
        found.accelerometer_rad,
        found.gyroscope_rad,
        found.gyroscope_rad * degrees
    );
    let within = |ratio: f64| (1.0 / TURN_FACTOR..=TURN_FACTOR).contains(&ratio);
    let enough =
        found.stretches >= TURN_LEAST_STRETCHES && found.accelerometer_rad >= TURN_LEAST_RAD;
    let scale = if enough && within(ratio) {
        decisions.push(format!(
            "{names} read in radians per second, from the board's turning: {evidence}; the unit \
             whose turn is within a quarter to 4 times the accelerometer's is taken"
        ));
        Some(180.0 / PI)
    } else if enough && within(ratio * degrees) {
        decisions.push(format!(
            "{names} read in degrees per second, from the board's turning: {evidence}; the unit \
             whose turn is within a quarter to 4 times the accelerometer's is taken"
        ));
        Some(1.0)
    } else {
        let why = if enough {
            "in neither unit is the gyroscope's turn within a quarter to 4 times the \
             accelerometer's"
        } else {
            "too little turning to tell, under 3 stretches or 0.2 rad"
        };
        decisions.push(format!(
            "{names} kept as written: the board's turning doesn't settle their unit, {why}: \
             {evidence}"
        ));
        None
    };
    for at in gyro {
        match scale {
            Some(scale) => columns[at].scale = scale,
            None => columns[at].keep(),
        }
    }
}

/// Checks the battery's, the tilt's and the angular velocities' values against their names' units,
/// keeping as written a column they refute.
fn decide_by_range(columns: &mut [Column], decisions: &mut Vec<String>) {
    let mut rates = Vec::new();
    let mut rates_beyond = None;
    for (at, column) in columns.iter_mut().enumerate() {
        let range = column.range();
        match column.role {
            Role::Battery => {
                if let Some((lo, hi)) = range.filter(|(lo, hi)| *lo < 0.0 || *hi > VOLTS_MAX) {
                    decisions.push(format!(
                        "{} kept as written: its name says volts, but its values run from {lo} \
                         to {hi}, outside the 0 to 20 V a board's battery reads",
                        column.quoted()
                    ));
                    column.keep();
                }
            }
            Role::Tilt => {
                if let Some((lo, hi)) = range.filter(|(lo, hi)| *lo < 0.0 || *hi > 180.0) {
                    decisions.push(format!(
                        "{} kept as written: its name says degrees, but its values run from {lo} \
                         to {hi}, where a tilt's size lies within 0 to 180°",
                        column.quoted()
                    ));
                    column.keep();
                }
            }
            Role::AngularVelocity => {
                rates.push(at);
                if let Some((lo, hi)) = range {
                    let largest = lo.abs().max(hi.abs());
                    if largest > RATE_MAX_RAD_S {
                        rates_beyond = Some(rates_beyond.map_or(largest, |l: f64| l.max(largest)));
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(largest) = rates_beyond {
        decisions.push(format!(
            "{} kept as written: their names say radians per second, but they reach {largest}, \
             beyond the 20,000 °/s (349 rad/s) full scale of Analog Devices' ADXRS649, a \
             gyroscope made for very high rates",
            listed_at(columns, &rates)
        ));
        for at in rates {
            columns[at].keep();
        }
    }
}

/// What the altitude check found for a column.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Climb {
    /// The pressure climbs too little, or has no unit, to check by; the climb, if any.
    Unchecked(Option<f64>),
    /// The pressure's climb in the standard atmosphere, m, and the column's, as written.
    Checked { pressure_m: f64, column: f64 },
}

/// The altitude check (module docs): from the first sample to the one of lowest pressure, the
/// pressure's climb in the standard atmosphere and the column's.
fn climb(columns: &[Column], pressure: Option<usize>, altitude: usize) -> Climb {
    let Some(pressure) = pressure else {
        return Climb::Unchecked(None);
    };
    let pairs = columns[pressure]
        .values
        .iter()
        .zip(&columns[altitude].values)
        .filter(|(p, a)| p.is_finite() && **p > 0.0 && a.is_finite());
    let mut first = None;
    let mut lowest: Option<(f64, f64)> = None;
    for (p, a) in pairs {
        first.get_or_insert((*p, *a));
        if lowest.is_none_or(|(low, _)| *p < low) {
            lowest = Some((*p, *a));
        }
    }
    let (Some(first), Some(lowest)) = (first, lowest) else {
        return Climb::Unchecked(None);
    };
    let atmosphere = Ussa76::standard();
    let Ok(pressure_m) = atmosphere
        .pressure_altitude_m(lowest.0)
        .and_then(|low| atmosphere.pressure_altitude_m(first.0).map(|pad| low - pad))
    else {
        return Climb::Unchecked(None);
    };
    if pressure_m < CHECK_CLIMB_M {
        return Climb::Unchecked(Some(pressure_m));
    }
    Climb::Checked {
        pressure_m,
        column: lowest.1 - first.1,
    }
}

/// Checks `altitudeM` against the pressure, refusing the file if its climb is off, and reads
/// `unfiltAlt` in meters if the pressure bears them out.
fn decide_altitudes(
    columns: &mut [Column],
    pressure_pa: bool,
    header_line: usize,
    decisions: &mut Vec<String>,
) -> Result<(), LogError> {
    let pressure = find(columns, Role::Pressure).filter(|_| pressure_pa);
    let within = |pressure_m: f64, column: f64| (column / pressure_m - 1.0).abs() <= CLIMB_MARGIN;
    let unchecked = |climb: Option<f64>| match climb {
        Some(climb) => format!(
            "the pressure climbs {climb:.2} m in the standard atmosphere, under the \
             {CHECK_CLIMB_M} m from which it checks an altitude's unit"
        ),
        None => "the file has no pressure in pascals to check it by".to_owned(),
    };
    if let Some(at) = find(columns, Role::Altitude) {
        match climb(columns, pressure, at) {
            Climb::Checked { pressure_m, column } if !within(pressure_m, column) => {
                return Err(LogError::Unit {
                    format: FORMAT,
                    line: header_line,
                    message: format!(
                        "{} climbs {column:.1} from the first sample to the one of lowest \
                         pressure, where the pressure's climb in the standard atmosphere is \
                         {pressure_m:.1} m: its name says meters, but they aren't within 25%",
                        columns[at].quoted()
                    ),
                });
            }
            Climb::Checked { .. } => {}
            Climb::Unchecked(climb) => decisions.push(format!(
                "{} read in meters, from its name: {}",
                columns[at].quoted(),
                unchecked(climb)
            )),
        }
    }
    if let Some(at) = find(columns, Role::Unfiltered) {
        match climb(columns, pressure, at) {
            Climb::Checked { pressure_m, column } if within(pressure_m, column) => {
                decisions.push(format!(
                    "{} read in meters, from the pressure: from the first sample to the one of \
                     lowest pressure it climbs {column:.1} where the standard atmosphere climbs \
                     {pressure_m:.1} m, within 25%",
                    columns[at].quoted()
                ));
            }
            Climb::Checked { pressure_m, column } => {
                decisions.push(format!(
                    "{} kept as written: from the first sample to the one of lowest pressure it \
                     climbs {column:.1} where the standard atmosphere climbs {pressure_m:.1} m, \
                     not within 25%",
                    columns[at].quoted()
                ));
                columns[at].keep();
            }
            Climb::Unchecked(climb) => {
                decisions.push(format!(
                    "{} kept as written: its name states no unit, and {}",
                    columns[at].quoted(),
                    unchecked(climb)
                ));
                columns[at].keep();
            }
        }
    }
    Ok(())
}

/// Checks `velocityMS` against `altitudeM` (module docs), keeping it as written if the slope is
/// off.
fn decide_speed(columns: &mut [Column], times: &[f64], decisions: &mut Vec<String>) {
    let (Some(speed), Some(altitude)) = (find(columns, Role::Speed), find(columns, Role::Altitude))
    else {
        return;
    };
    let v = &columns[speed].values;
    let z = &columns[altitude].values;
    let (mut cross, mut square, mut moved) = (0.0, 0.0, 0.0);
    for row in 0..times.len().saturating_sub(1) {
        let step = times[row + 1] - times[row];
        let travelled = 0.5 * (v[row] + v[row + 1]) * step;
        let risen = z[row + 1] - z[row];
        if !(step > 0.0 && step <= FAST_STEP_S && travelled.is_finite() && risen.is_finite()) {
            continue;
        }
        cross += travelled * risen;
        square += travelled * travelled;
        moved += travelled.abs();
    }
    let name = columns[speed].quoted();
    if moved < SPEED_LEAST_M {
        decisions.push(format!(
            "{name} read in meters per second, from its name: over the steps at most \
             {FAST_STEP_S} s long it moves {moved:.1} m, under the {SPEED_LEAST_M} m from which \
             the altitude checks it"
        ));
        return;
    }
    let slope = cross / square;
    if (slope - 1.0).abs() > SPEED_MARGIN {
        decisions.push(format!(
            "{name} kept as written: its name says meters per second, but over the steps at most \
             {FAST_STEP_S} s long the altitude changes {slope:.3} times what it integrates to, \
             not within 25% of once"
        ));
        columns[speed].keep();
    }
}

/// A note if `tempK`, read in kelvin, holds one value at every sample while the altitude climbs
/// more than [`STILL_CLIMB_M`].
fn still_temperature(columns: &[Column], primary: usize) -> Option<String> {
    let at = find(columns, Role::Temperature)?;
    let column = &columns[at];
    if column.quantity != Quantity::Temperature {
        return None;
    }
    let (lo, hi) = column.range()?;
    let samples = column.values.iter().filter(|v| v.is_finite()).count();
    let (low, high) = columns[primary].range()?;
    (lo == hi && samples >= 2 && high - low > STILL_CLIMB_M).then(|| {
        format!(
            "{} reads {lo} at every one of its {samples} samples while the altitude spans \
             {:.0} m: a board's sensor warms or cools in flight, so the file may be a replay of \
             recorded or made-up values rather than the board's own log",
            column.quoted(),
            high - low
        )
    })
}

/// The logger, the firmware and a note on the settings left out, from a `CONFIG` line.
fn settings(config: Option<&str>) -> (String, Option<String>, Option<String>) {
    let mut logger = LOGGER.to_owned();
    let mut firmware = None;
    let Some(config) = config else {
        return (logger, firmware, None);
    };
    let mut others = 0usize;
    let mut first_other = None;
    for setting in cells(config, config.contains('\t')).skip(1) {
        let Some((key, value)) = setting.split_once('=') else {
            others += 1;
            continue;
        };
        let value = value.trim();
        match key.trim() {
            "BOARD_NAME" if !value.is_empty() => logger = short(value),
            "FIRMWARE_VERSION" if !value.is_empty() => firmware = Some(short(value)),
            key => {
                others += 1;
                first_other.get_or_insert_with(|| short(key));
            }
        }
    }
    let note = (others > 0).then(|| {
        format!(
            "the `CONFIG` line's {others} other settings{} are left out: no source gives their \
             meanings or units",
            first_other.map_or_else(String::new, |key| format!(", such as `{key}`,"))
        )
    });
    (logger, firmware, note)
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
    const FOOT_M: f64 = 0.3048;

    const HEADER: &str = "timestampMs\tpressurePa\ttempK\taccelX\taccelY\taccelZ\tgyroX\tgyroY\t\
                          gyroZ\timuTemp\tbattV\taltitudeM\tvelocityMS\taccelerationMSS\t\
                          unfiltAlt\tflightState\tdrogueCont\tdrogueFired\tmainCont\tmainFired";

    /// The pressure, Pa, at an altitude above a pad 300 m up in the standard atmosphere's
    /// troposphere: the altimeter formula inverted.
    fn pressure_at(altitude_m: f64) -> f64 {
        101_325.0 * (1.0 - (300.0 + altitude_m) / 44_330.8).powf(1.0 / 0.190_263)
    }

    /// How an invented flight is written: each quantity's value per SI unit, and the bay's
    /// temperatures as written.
    #[derive(Debug, Clone, Copy)]
    struct Written {
        accel: f64,
        gyro: f64,
        altitude: f64,
        unfiltered: f64,
        speed: f64,
        pressure: f64,
        temp: f64,
        imu_temp: f64,
        battery: f64,
        turning: bool,
    }

    impl Default for Written {
        fn default() -> Self {
            Self {
                accel: 1.0,
                gyro: 1.0,
                altitude: 1.0,
                unfiltered: 1.0,
                speed: 1.0,
                pressure: 1.0,
                temp: 290.15,
                imu_temp: 293.15,
                battery: 4.0,
                turning: true,
            }
        }
    }

    /// One sample of the invented flight, in SI and rad/s.
    struct Sample {
        t_ms: f64,
        altitude_m: f64,
        speed_m_s: f64,
        accel: [f64; 3],
        gyro: [f64; 3],
        drogue: u8,
        main: u8,
    }

    /// An invented flight: 4 samples on the pad 5 s apart; a boost at 7 g, 20 samples 50 ms
    /// apart; 60 samples 20 ms apart under a parachute from 300 m, the board turning about x at
    /// 1 rad/s if `turning`, the drogue fired; 3 samples landed 5 s apart, the main fired.
    fn samples(turning: bool) -> Vec<Sample> {
        let mut out = Vec::new();
        for k in 0..4 {
            out.push(Sample {
                t_ms: 1_000.0 + 5_000.0 * f64::from(k),
                altitude_m: 0.0,
                speed_m_s: 0.0,
                accel: [0.0, 0.0, G],
                gyro: [0.0; 3],
                drogue: 0,
                main: 0,
            });
        }
        for k in 1..=20 {
            let tau = 0.05 * f64::from(k);
            out.push(Sample {
                t_ms: 16_000.0 + 50.0 * f64::from(k),
                altitude_m: 40.0 * tau * tau,
                speed_m_s: 80.0 * tau,
                accel: [0.0, 0.0, 7.0 * G],
                gyro: [0.0; 3],
                drogue: 0,
                main: 0,
            });
        }
        let rate = if turning { 1.0 } else { 0.0 };
        for k in 0..60 {
            let tau = 0.02 * f64::from(k);
            let angle = rate * tau;
            out.push(Sample {
                t_ms: 21_000.0 + 20.0 * f64::from(k),
                altitude_m: 300.0 - 15.0 * tau,
                speed_m_s: -15.0,
                accel: [0.0, G * angle.sin(), G * angle.cos()],
                gyro: [rate, 0.0, 0.0],
                drogue: 1,
                main: 0,
            });
        }
        for k in 0..3 {
            out.push(Sample {
                t_ms: 60_000.0 + 5_000.0 * f64::from(k),
                altitude_m: 0.4,
                speed_m_s: 0.0,
                accel: [0.0, G, 0.0],
                gyro: [0.0; 3],
                drogue: 1,
                main: 1,
            });
        }
        out
    }

    /// The invented flight as the board's tab-separated log, written as asked.
    fn text(w: Written) -> String {
        let mut text = format!("{HEADER}\n");
        for (k, s) in samples(w.turning).into_iter().enumerate() {
            let a = s.accel.map(|v| v * w.accel);
            let g = s.gyro.map(|v| v * w.gyro);
            text.push_str(&format!(
                "{}\t{:.4}\t{:.3}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{}\t{}\t{:.3}\t{:.3}\t\
                 0.00\t{:.3}\t0\t1\t{}\t1\t{}\n",
                s.t_ms,
                pressure_at(s.altitude_m) * w.pressure,
                w.temp + 0.001 * k as f64,
                a[0],
                a[1],
                a[2],
                g[0],
                g[1],
                g[2],
                w.imu_temp,
                w.battery,
                s.altitude_m * w.altitude,
                s.speed_m_s * w.speed,
                (300.0 + s.altitude_m) * w.unfiltered,
                s.drogue,
                s.main
            ));
        }
        text
    }

    /// The invented flight with its temperature, row by row, as given.
    fn text_with_temperature(temperature: impl Fn(usize) -> f64) -> String {
        let text = text(Written::default());
        let mut lines = text.lines();
        let mut out = format!("{}\n", lines.next().unwrap_or_default());
        for (k, line) in lines.enumerate() {
            let mut cells: Vec<String> = line.split('\t').map(str::to_owned).collect();
            cells[2] = format!("{}", temperature(k));
            out.push_str(&cells.join("\t"));
            out.push('\n');
        }
        out
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap_or_else(|| panic!("no {column}"))
    }

    fn values<'a>(log: &'a FlightLog, column: &str) -> &'a [f64] {
        &channel(log, column).values
    }

    fn quantity(log: &FlightLog, column: &str) -> Quantity {
        channel(log, column).quantity
    }

    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
        matching[0]
    }

    fn has_note(log: &FlightLog, start: &str) -> bool {
        log.notes.iter().any(|n| n.starts_with(start))
    }

    /// The survey's reading of a note (`xtask logs`' `unit_decision`): the text before the first
    /// `: ` of a note opening with a name in backticks that is `read in` or `kept as written`.
    fn counted(note: &str) -> Option<&str> {
        let (decision, _) = note.split_once(": ")?;
        (decision.starts_with('`')
            && (decision.contains("` read in ") || decision.contains("` kept as written")))
        .then_some(decision)
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-9 * b.abs().max(1.0)
    }

    #[test]
    fn recognizes_the_header_and_nothing_like_it() {
        assert!(recognizes(&format!("\u{feff}\r\n{HEADER}\r\n1\t2\n")));
        assert!(recognizes(&HEADER.replace('\t', " ")));
        assert!(recognizes("TIMESTAMPMS altitudeM PRESSUREPA\n"));
        assert!(recognizes(&format!("CONFIG\tBOARD_NAME=x\n\n{HEADER}\n")));
        for other in [
            "",
            "timestampMs\tpressurePa\n",
            "timestampMs,pressurePa,altitudeM\n",
            "time\tpressurePa\taltitudeM\n",
            "timestamp_ms,pressurePa,altitudeM\n",
            "CONFIG\tconverted_from=x\n6098809,0,302.7,-2.8\n",
            "CONFIG\tBOARD_NAME=x\n",
            "time,pressure,Pressure AGL,time,launch detect\n",
            "# MET Acc Vel iAlt pAlt\n",
        ] {
            assert!(!recognizes(other), "{other:?}");
        }
        // The header must come first, or straight after the settings line.
        assert!(!recognizes(&format!("a note\n{HEADER}\n")));
    }

    #[test]
    fn reads_an_invented_flight_in_si() {
        let log = read(&text(Written::default())).unwrap();
        assert_eq!(log.format, LogFormat::SillyGooseText);
        assert_eq!(log.logger, LOGGER);
        assert_eq!(log.time_s.len(), 87);
        assert_eq!(log.time_s[0], 1.0);
        assert_eq!(log.channels.len(), 19);
        assert!(log.channels.iter().all(|c| c.time_s.len() == 87));
        assert_eq!(quantity(&log, "pressurePa"), Quantity::Pressure);
        assert_eq!(quantity(&log, "accelZ"), Quantity::Acceleration);
        assert_eq!(quantity(&log, "gyroX"), Quantity::AngularRate);
        assert_eq!(quantity(&log, "flightState"), Quantity::Number);
        assert!(close(values(&log, "accelZ")[0], G));
        // 1 rad/s, in °/s.
        assert!(close(values(&log, "gyroX")[30], 180.0 / PI));
        let top = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
        assert_eq!(top, 300.0);
        assert_eq!(log.vertical_speed_m_s.as_ref().unwrap()[10], 28.0);
        assert_eq!(log.temperature_k.as_ref().unwrap()[0], 290.15);
        assert_eq!(log.battery_v.as_ref().unwrap()[0], 4.0);
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(events, [(21.0, "drogueFired"), (60.0, "mainFired")]);
        // Every unit decision is one the survey counts.
        for note in log.notes.iter().filter(|n| n.starts_with('`')) {
            assert!(counted(note).is_some(), "{note}");
        }
        assert!(log.notes.iter().all(|n| !n.contains("assum")));
    }

    /// The accelerometer's unit both ways: a file in g reads the same in SI as one in m/s², and
    /// neither is taken for the other. Flipping the rule would read the g file's pad as 1 m/s².
    #[test]
    fn the_accelerometer_s_unit_is_decided_by_the_pad_s_gravity_both_ways() {
        let in_ms2 = read(&text(Written::default())).unwrap();
        let in_g = read(&text(Written {
            accel: 1.0 / G,
            ..Written::default()
        }))
        .unwrap();
        note(
            &in_ms2,
            "`accelX`, `accelY` and `accelZ` read in meters per second squared, from the pad's \
             gravity: ",
        );
        note(
            &in_g,
            "`accelX`, `accelY` and `accelZ` read in standard gravities, × 9.806 65 m/s², from \
             the pad's gravity: ",
        );
        for (a, b) in values(&in_g, "accelZ")
            .iter()
            .zip(values(&in_ms2, "accelZ"))
        {
            assert!((a - b).abs() < 1e-5, "{a} {b}");
        }
        assert!(close(values(&in_g, "accelZ")[0], G));
        // The gyroscope's check runs on either.
        assert_eq!(quantity(&in_g, "gyroX"), Quantity::AngularRate);

        // A magnitude near neither 1 g settles nothing: the axes, and the gyroscope that needs
        // them, are kept as written.
        let neither = read(&text(Written {
            accel: 0.5,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&neither, "accelX"), Quantity::Unrecognized);
        assert!(close(values(&neither, "accelZ")[0], 0.5 * G));
        note(
            &neither,
            "`accelX`, `accelY` and `accelZ` kept as written: ",
        );
        assert!(
            note(&neither, "`gyroX`, `gyroY` and `gyroZ` kept as written: ")
                .contains("no accelerometer read in a known unit")
        );
    }

    /// The pad's gravity counts time, not samples: a fast run of 1 m/s² readings in a coast
    /// doesn't outweigh minutes of 9.8 at rest.
    #[test]
    fn the_pad_s_gravity_weighs_each_reading_by_its_time() {
        let mut text = format!("{HEADER}\n");
        for t in [0.0, 60_000.0, 120_000.0] {
            text.push_str(&format!(
                "{t}\t97772\t290\t0\t0\t{G}\t0\t0\t0\t293\t4\t0\t0\t0\t300\t0\t0\t0\t0\t0\n"
            ));
        }
        for k in 1..=300 {
            let t = 120_000.0 + 10.0 * f64::from(k);
            text.push_str(&format!(
                "{t}\t97772\t290\t0\t0\t1.0\t0\t0\t0\t293\t4\t0\t0\t0\t300\t0\t0\t0\t0\t0\n"
            ));
        }
        let log = read(&text).unwrap();
        assert!(
            note(
                &log,
                "`accelX`, `accelY` and `accelZ` read in meters per second squared"
            )
            .contains("within 10% of 9.806 65 for 120.0 s of the file's time and within 10% of 1 for 3.0 s"),
            "{:?}",
            log.notes
        );
    }

    /// The gyroscope's unit both ways, and kept where the turning doesn't settle it. Flipping the
    /// rule would read the radian file's 1 rad/s as 1 °/s.
    #[test]
    fn the_gyroscope_s_unit_is_decided_by_the_board_s_turning_both_ways() {
        let radians = read(&text(Written::default())).unwrap();
        let degrees = read(&text(Written {
            gyro: 180.0 / PI,
            ..Written::default()
        }))
        .unwrap();
        assert!(
            note(
                &radians,
                "`gyroX`, `gyroY` and `gyroZ` read in radians per second, from the board's \
                 turning: "
            )
            .contains("over 5 stretches")
        );
        note(
            &degrees,
            "`gyroX`, `gyroY` and `gyroZ` read in degrees per second, from the board's turning: ",
        );
        for (a, b) in values(&radians, "gyroX")
            .iter()
            .zip(values(&degrees, "gyroX"))
        {
            assert!((a - b).abs() < 1e-4, "{a} {b}");
        }
        assert!(close(values(&radians, "gyroX")[30], 180.0 / PI));

        // No turning: too little to tell.
        let still = read(&text(Written {
            turning: false,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&still, "gyroX"), Quantity::Unrecognized);
        assert!(
            note(&still, "`gyroX`, `gyroY` and `gyroZ` kept as written: ")
                .contains("too little turning to tell")
        );

        // Rates 10 times the turning's in rad/s, 0.17 times in °/s: in neither unit within a
        // factor of 4.
        let off = read(&text(Written {
            gyro: 10.0,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&off, "gyroX"), Quantity::Unrecognized);
        assert!(
            note(&off, "`gyroX`, `gyroY` and `gyroZ` kept as written: ")
                .contains("in neither unit")
        );
    }

    #[test]
    fn the_turning_check_measures_a_known_turn() {
        let log_text = text(Written::default());
        let mut lines = numbered_lines(&log_text);
        let _ = lines.next();
        let mut columns: Vec<Column> = HEADER.split('\t').skip(1).map(Column::new).collect();
        let mut times = Vec::new();
        for (_, line) in lines {
            let mut cells = line
                .split('\t')
                .map(|c| c.parse::<f64>().unwrap_or(f64::NAN));
            times.push(cells.next().unwrap_or(f64::NAN) / 1_000.0);
            for (column, value) in columns.iter_mut().zip(cells) {
                column.values.push(value);
            }
        }
        let accel = axes(&columns, Role::Accel).unwrap();
        let gyro = axes(&columns, Role::Gyro).unwrap();
        let found = turning(&columns, &times, accel, gyro, G);
        // 59 steps of 20 ms at 1 rad/s under the parachute: 5 whole stretches of 0.2 rad.
        assert_eq!(found.stretches, 5);
        assert!((found.accelerometer_rad - 1.0).abs() < 1e-5, "{found:?}");
        assert!((found.gyroscope_rad - 1.0).abs() < 1e-5, "{found:?}");
    }

    #[test]
    fn temperatures_are_kelvin_only_where_the_values_bear_it_out() {
        let log = read(&text(Written::default())).unwrap();
        assert_eq!(quantity(&log, "tempK"), Quantity::Temperature);
        assert_eq!(quantity(&log, "imuTemp"), Quantity::Temperature);
        note(&log, "`imuTemp` read in kelvin, from the values: ");
        assert!(!has_note(&log, "`tempK`"), "{:?}", log.notes);

        let refuted = read(&text(Written {
            temp: -9.0,
            imu_temp: 0.0,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&refuted, "tempK"), Quantity::Unrecognized);
        assert!(refuted.temperature_k.is_none());
        assert_eq!(values(&refuted, "tempK")[0], -9.0);
        assert!(
            note(
                &refuted,
                "`tempK` kept as written: its name says kelvin, but "
            )
            .contains("below 0 no temperature in kelvin can be")
        );
        assert!(
            note(
                &refuted,
                "`imuTemp` kept as written: its name states no unit, and "
            )
            .contains("every value is exactly 0")
        );

        // Degrees Celsius aren't kelvin, and nothing says they are Celsius.
        let celsius = read(&text(Written {
            temp: 20.0,
            imu_temp: 22.5,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&celsius, "tempK"), Quantity::Unrecognized);
        assert_eq!(quantity(&celsius, "imuTemp"), Quantity::Unrecognized);
        assert!(
            note(&celsius, "`tempK` kept as written: ")
                .contains("run from 20 to 20.086, outside the 233.15 to 333.15 K")
        );
    }

    /// `altitudeM` in feet is refused: the primary track would be 3.28 times off. `unfiltAlt` in
    /// feet is kept as written.
    #[test]
    fn altitudes_are_checked_against_the_pressure() {
        match read(&text(Written {
            altitude: 1.0 / FOOT_M,
            ..Written::default()
        })) {
            Err(LogError::Unit {
                line: 1, message, ..
            }) => {
                assert!(message.contains("`altitudeM` climbs 984.3"), "{message}");
                assert!(message.contains("aren't within 25%"), "{message}");
            }
            other => panic!("{other:?}"),
        }
        let unfiltered = read(&text(Written {
            unfiltered: 1.0 / FOOT_M,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&unfiltered, "unfiltAlt"), Quantity::Unrecognized);
        assert!(note(&unfiltered, "`unfiltAlt` kept as written: ").contains("climbs 984.3 where"));
        let meters = read(&text(Written::default())).unwrap();
        assert_eq!(
            quantity(&meters, "unfiltAlt"),
            Quantity::AltitudeAboveGround
        );
        note(&meters, "`unfiltAlt` read in meters, from the pressure: ");
        assert!(!has_note(&meters, "`altitudeM`"));
    }

    #[test]
    fn a_pressure_that_is_no_pad_s_in_pascals_is_kept_and_the_altitude_unchecked() {
        let hectopascals = read(&text(Written {
            pressure: 0.01,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(
            quantity(&hectopascals, "pressurePa"),
            Quantity::Unrecognized
        );
        assert!(
            note(
                &hectopascals,
                "`pressurePa` kept as written: its name says pascals, but "
            )
            .contains("(50,000 to 110,000 Pa)")
        );
        assert!(
            note(&hectopascals, "`altitudeM` read in meters, from its name: ")
                .contains("no pressure in pascals")
        );
        note(&hectopascals, "`unfiltAlt` kept as written: ");
    }

    /// A speed in feet per second is 3.28 times what the altitude climbs: kept as written, and
    /// no vertical speed. In m/s it is borne out without a note.
    #[test]
    fn the_speed_is_checked_against_the_altitude() {
        let log = read(&text(Written::default())).unwrap();
        assert_eq!(quantity(&log, "velocityMS"), Quantity::Speed);
        assert!(!has_note(&log, "`velocityMS`"));
        let feet = read(&text(Written {
            speed: 1.0 / FOOT_M,
            ..Written::default()
        }))
        .unwrap();
        assert_eq!(quantity(&feet, "velocityMS"), Quantity::Unrecognized);
        assert!(feet.vertical_speed_m_s.is_none());
        assert!(note(&feet, "`velocityMS` kept as written: ").contains("0.305 times"));
    }

    /// A log all on the pad: too little climb or movement to check the altitude's and the speed's
    /// units, so they are read from their names, and `unfiltAlt` is kept.
    #[test]
    fn a_log_on_the_pad_reads_its_units_from_their_names() {
        let mut text = format!("{HEADER}\n");
        for k in 0..6 {
            let t = 1_000 + 5_000 * k;
            text.push_str(&format!(
                "{t}\t97772.5\t290.2\t0.01\t0.02\t9.79\t0.001\t0\t0\t293\t4.1\t0.{k}\t0.0\t0\t\
                 300.{k}\t0\t0\t0\t0\t0\n"
            ));
        }
        let log = read(&text).unwrap();
        assert!(
            note(&log, "`altitudeM` read in meters, from its name: ")
                .contains("the pressure climbs 0.00 m")
        );
        note(
            &log,
            "`velocityMS` read in meters per second, from its name: ",
        );
        note(
            &log,
            "`unfiltAlt` kept as written: its name states no unit, and ",
        );
        note(&log, "`gyroX`, `gyroY` and `gyroZ` kept as written: ");
        assert!(log.events.is_empty());
    }

    #[test]
    fn the_settings_line_names_the_board_and_the_extra_columns_are_checked() {
        let header = format!(
            "{HEADER}\ttiltMagnitudeDeg\tangularVelRadS_x\tangularVelRadS_y\tangularVelRadS_z\t\
             quaternion_a\tquaternion_b\tquaternion_c\tquaternion_d"
        );
        let file = |config: &str, tilt: f64, rate: f64, battery: f64| {
            let mut text = format!("{config}\n{header}\n");
            for k in 0..3 {
                text.push_str(&format!(
                    "{}\t97772\t290\t0\t0\t9.8\t0\t0\t0\t293\t{battery}\t0\t0\t0\t300\t0\t0\t0\t\
                     0\t0\t{tilt}\t{rate}\t0\t0\t1\t0\t0\t0\n",
                    1_000 + 5_000 * k
                ));
            }
            text
        };
        let log = read(&file(
            "CONFIG\tBOARD_NAME=Bench\tFIRMWARE_VERSION=2.1\tA=1\tNOVALUE",
            12.5,
            3.0,
            4.0,
        ))
        .unwrap();
        assert_eq!(log.logger, "Bench");
        assert_eq!(log.firmware.as_deref(), Some("2.1"));
        note(
            &log,
            "the `CONFIG` line's 2 other settings, such as `A`, are left out: ",
        );
        assert_eq!(quantity(&log, "tiltMagnitudeDeg"), Quantity::Angle);
        assert_eq!(quantity(&log, "quaternion_a"), Quantity::Number);
        assert_eq!(quantity(&log, "angularVelRadS_x"), Quantity::AngularRate);
        assert!(close(values(&log, "angularVelRadS_x")[0], 3.0 * 180.0 / PI));

        // Past what a gyroscope measures, beyond a tilt's range, and beyond a battery's volts.
        let refuted = read(&file("CONFIG\tBOARD_NAME=", 190.0, 400.0, 3_700.0)).unwrap();
        assert_eq!(refuted.logger, LOGGER);
        assert!(refuted.firmware.is_none());
        for column in [
            "angularVelRadS_x",
            "angularVelRadS_y",
            "tiltMagnitudeDeg",
            "battV",
        ] {
            assert_eq!(
                quantity(&refuted, column),
                Quantity::Unrecognized,
                "{column}"
            );
        }
        assert!(refuted.battery_v.is_none());
        assert!(
            note(
                &refuted,
                "`angularVelRadS_x`, `angularVelRadS_y` and `angularVelRadS_z` kept as written: "
            )
            .contains("reach 400")
        );
        note(&refuted, "`tiltMagnitudeDeg` kept as written: ");
        note(&refuted, "`battV` kept as written: ");
        // A rate just inside 20,000 °/s is read.
        let inside = read(&file("CONFIG", 10.0, 349.0, 4.0)).unwrap();
        assert_eq!(quantity(&inside, "angularVelRadS_x"), Quantity::AngularRate);
    }

    #[test]
    fn malformed_files_are_refused_naming_the_line() {
        let syntax_line = |text: &str| match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("{other:?}"),
        };
        let wide = format!("{HEADER}\n{}\t1\n", ["0"; 20].join("\t"));
        let (line, message) = syntax_line(&wide);
        assert_eq!(line, 2);
        assert!(
            message.contains("21 cells where the header names 20"),
            "{message}"
        );
        let (line, message) = syntax_line("timestampMs pressurePa altitudeM altitudeM\n1 2 3 4\n");
        assert_eq!(line, 1);
        assert!(message.contains("names \"altitudeM\" twice"), "{message}");
        let (_, message) = syntax_line("timestampMs\tpressurePa\t\taltitudeM\n");
        assert!(message.contains("column 3 has no name"), "{message}");
        for text in ["", "\n\n", "CONFIG\tA=1\n", "hello\n"] {
            assert!(
                matches!(read(text), Err(LogError::NotThisFormat { .. })),
                "{text:?}"
            );
        }
        assert!(matches!(
            read("timestampMs pressurePa altitudeM\n"),
            Err(LogError::NoData { .. })
        ));
        assert!(matches!(
            read("timestampMs pressurePa altitudeM\nx 97000 0\n"),
            Err(LogError::NoData { .. })
        ));
        // A wide header over short rows can't make the reader store far more than the file holds.
        let names: Vec<String> = (0..2_000).map(|k| format!("c{k}")).collect();
        let mut sparse = format!("timestampMs\tpressurePa\taltitudeM\t{}\n", names.join("\t"));
        for k in 0..100 {
            sparse.push_str(&format!("{k}\n"));
        }
        let (_, message) = syntax_line(&sparse);
        assert!(message.contains("too few for the header"), "{message}");
    }

    #[test]
    fn empty_and_unreadable_cells_and_short_rows_are_gaps() {
        let text = "timestampMs pressurePa altitudeM extra\n\
                    1000 97772 0 1\n\
                    2000 x 1 2\n\
                    3000 97760 2\n";
        let log = read(text).unwrap();
        assert!(values(&log, "pressurePa")[1].is_nan());
        assert!(values(&log, "extra")[2].is_nan());
        assert_eq!(quantity(&log, "extra"), Quantity::Unrecognized);
        note(
            &log,
            "1 cells, the first on line 3, are empty or not numbers: ",
        );
        note(
            &log,
            "1 rows, the first on line 4, hold fewer cells than the header names: ",
        );
        note(&log, "the column `extra` isn't one this reader knows");
        let tabbed = read("timestampMs\tpressurePa\taltitudeM\n1000\t\t0\n").unwrap();
        assert!(tabbed.channels[0].values[0].is_nan());
        note(
            &tabbed,
            "1 cells, the first on line 2, are empty or not numbers: ",
        );
    }

    #[test]
    fn the_track_is_put_in_time_order_and_the_channels_kept_as_written() {
        let text = "timestampMs pressurePa altitudeM\n\
                    1000 97772 0\n\
                    3000 97749 2\n\
                    2000 97760 1\n\
                    3000 97668 9\n\
                    - 97714 5\n";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [1.0, 2.0, 3.0]);
        assert_eq!(log.altitude_m, [0.0, 1.0, 2.0]);
        assert_eq!(values(&log, "altitudeM"), [0.0, 2.0, 1.0, 9.0, 5.0]);
        note(
            &log,
            "the clock goes back 1 times and repeats a time 1 times, and 1 samples have no time",
        );
    }

    #[test]
    fn a_log_starting_aloft_and_a_temperature_that_never_changes_are_noted() {
        let text = "timestampMs pressurePa tempK altitudeM\n\
                    100 96000 290.15 150\n\
                    200 96010 290.15 149\n";
        let log = read(text).unwrap();
        note(
            &log,
            "`altitudeM` reads 150.00 m at the track's first sample, not near zero",
        );
        // 150 m to 149 m isn't a climb of over 100 m: no note on the temperature.
        assert!(!has_note(&log, "`tempK` reads"));

        let replay = read(&text_with_temperature(|_| 300.0)).unwrap();
        assert!(
            note(
                &replay,
                "`tempK` reads 300 at every one of its 87 samples while the altitude spans 300 m"
            )
            .contains("replay")
        );
        let recording = read(&text_with_temperature(|k| 300.0 + 0.01 * k as f64)).unwrap();
        assert!(!has_note(&recording, "`tempK` reads"));
    }

    #[test]
    fn a_fired_flag_s_rises_are_events_and_one_already_set_is_not() {
        let text = "timestampMs pressurePa altitudeM drogueFired mainFired drogueCont flightState\n\
                    1000 97772 0 0 1 1 0\n\
                    2000 97770 0 1 1 0 1\n\
                    3000 97770 0 - 0 1 2\n\
                    4000 97770 0 1 1 0 3\n\
                    5000 97770 0 0 0 1 4\n\
                    6000 97770 0 1 0 1 4\n";
        let log = read(text).unwrap();
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        // `mainFired` set at its first sample is no rise; it rises again at 4 s. A gap between
        // two 1s is no rise, nor are the continuity flag's and the state's changes.
        assert_eq!(
            events,
            [
                (2.0, "drogueFired"),
                (4.0, "mainFired"),
                (6.0, "drogueFired")
            ]
        );
        assert_eq!(quantity(&log, "drogueCont"), Quantity::Number);
        assert_eq!(quantity(&log, "flightState"), Quantity::Number);
    }

    #[test]
    fn listed_names_read_as_a_sentence() {
        let columns: Vec<Column> = ["a", "b", "c"].into_iter().map(Column::new).collect();
        assert_eq!(listed_at(&columns, &[0]), "`a`");
        assert_eq!(listed_at(&columns, &[0, 1]), "`a` and `b`");
        assert_eq!(listed_at(&columns, &[0, 1, 2]), "`a`, `b` and `c`");
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a strictly increasing
        /// primary track, and every channel as many times as values.
        #[test]
        fn any_text_is_read_or_refused(
            text in "(CONFIG\tBOARD_NAME=[a-z]{0,4}\n)?timestampMs[ \t]pressurePa[ \t]altitudeM\
                     ([ \t](accelX|accelY|accelZ|gyroX|gyroY|gyroZ|tempK|velocityMS|mainFired)){0,6}\n\
                     ([-0-9. \t]{0,30}\n){0,16}"
        ) {
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
