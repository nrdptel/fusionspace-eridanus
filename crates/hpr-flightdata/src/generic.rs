//! A table of samples from a logger no other reader knows, read by what its header states: one
//! header row naming the columns, the units in the names, then one row a sample. Or the same
//! table read by a column mapping the caller gives, for a file whose header states too little or
//! that has none.
//!
//! Teams' own flight computers, data-logging scripts and spreadsheet exports write such tables,
//! each in its own words, such as:
//!
//! ```text
//! Time (s),Acc (g),Alt AGL (ft),Temp (F)
//! time__us,baro__kpa,temp_c,acc_y__m_s2,gyro_y__rad_s
//! Time [s],Battery [V],Altitude [m],Accel X [m/s^2]
//! ```
//!
//! No maker documents these layouts: the reader takes what the header says and no more. It reads
//! a column in a unit only where the header states the unit, and keeps every other column as the
//! file writes it ([`Quantity::Unrecognized`]), with a note saying why. A stated unit is the
//! file's own statement, the first of the two checks [ADR-258][adr-258] asks of a unit; the
//! second is that the values bear it out (*The values' check*, below), and a column whose values
//! contradict its unit is kept as written too. Nothing is guessed from a column's values alone.
//!
//! **Recognition.** Tried after every other layout, so it never takes a file another reader
//! knows. Within the file's first 64 lines, past any blank lines, lines starting `#` and a
//! `sep=` line naming the separator, the first line is the header. Its separator is the one of
//! `,`, `;`, a tab and `|` that splits it into the most cells, at least two, unless a `sep=` line
//! named it; no cell of it may be a number. One column must be the clock: a name with a time
//! word in it (`time`, `t`, `ts`, `timestamp`, `uptime`) and a unit of time stated. The next line
//! that isn't blank must hold a number in the clock's cell.
//!
//! **Units in a name.** The unit is read from brackets at the name's end, `Alt AGL (ft)` or
//! `Altitude [m]`: their whole content, spaces taken out, as `[deg C]`, or else its first word,
//! the rest a qualifier, as `(m ASL)` or `(° dec)`. With no brackets, it is read from the name's
//! last one to four words joined by underscores, as `Time_s`, `temp_c`, `acc_y__m_s2` or
//! `pitch_rate_deg_per_s`; a single-letter unit counts there only if it is `s`, `m`, `g`,
//! `c`, `f` or `v`, so that a name such as `flag_a` isn't read as amperes. Units are compared
//! without regard to case. The units known, by what they measure ([`unit_spellings`] lists
//! every spelling):
//!
//! | Measures | Units |
//! |---|---|
//! | time | `s`, `ms`, `us` (`µs`), and their words |
//! | length | `m`, `ft` (`feet`), `km` |
//! | speed | `m/s` (`mps`, `m_s`, `meters_per_s`), `ft/s` (`fps`, `ft_per_s`), `km/h`, `mph`, `kn` |
//! | acceleration | `m/s2` (`m/s²`, `m/s^2`, `m_s2`, `mps2`), `ft/s2`, `g` (`gs`), `mg` (thousandths of a g) |
//! | angle | `deg` (`°`), `rad` |
//! | rate of turn | `deg/s` (`dps`, `deg_per_s`), `rad/s` (`rad_per_s`) |
//! | temperature | `degC` (`°C`, `C`, `celsius`), `degF` (`°F`, `F`, `fahrenheit`), `K` |
//! | voltage | `V`, `mV` |
//! | current | `A` (in brackets only), `mA` (`milliamps`) |
//! | pressure | `Pa`, `hPa`, `mbar`, `kPa`, `bar`, `atm`, `psi`, `inHg` |
//!
//! `g` is standard gravity, 9.806 65 m/s² (CGPM 1901); `mg` a thousandth of it, as accelerometer
//! logs write it (an accelerometer log never holds milligrams); `inHg` 3,386.389 Pa and `psi`
//! 6,894.757 Pa (NIST SP 811, 2008, appendix B.8); `atm` 101,325 Pa; `kn` 1,852 m an hour;
//! `mph` 0.447 04 m/s.
//!
//! **What a column is.** The unit says what the column measures. The name's words say what it
//! may be: an altitude (`alt…`, `…alt`, `altitude`, `height`, `elev`, `agl`, `msl`, `asl`), a
//! pressure (`press…`, `baro…`, `pression`), a temperature (`temp…`), a voltage (`volt…`,
//! `batt…`, `vbat…`), a current (`curr…`, `amp…`), an acceleration (`acc…`, `…acc`, `…accel…`),
//! a rate of turn (`gyro…`, `rate`), a speed (`vel…`, `speed…`, `vx`, `vy`, `vz`, `v`), an angle
//! (`lat`, `lon`, `lng`, `long`, `heading`, `angle`, `tilt`, `roll`, `pitch`, `yaw`, `course`,
//! `bearing`, `cog`) or a time (`time…`, `t`, `ts`, `uptime`). A column is read in its unit when
//! the name's words allow what the unit measures, or name nothing; it is kept as written when
//! they disagree, as `vz_filtered_ms` (a speed's name, a time's unit), or say a rate of turn over
//! an angle's unit, as `Roll rate (deg)`; when the name says an estimate's error or a difference
//! (`accuracy`, `error`, `err`, `std`, `stdev`, `stddev`, `sigma`, `var`, `variance`,
//! `uncertainty`, `diff`, `differential`, `dynamic`, `dyn`, `pitot`, `delta`), as `Alt error (m)`
//! or a pitot tube's `Diff press (Pa)`; when it has a length or a time and the name says no
//! altitude or time, as `z (m)`, which may be a height or a distance; and when its unit may stand
//! for one the reader doesn't know (`g`, `gs` and `mg`, also grams and gauss; `c`, `f`, `v`, `k`
//! and `a`) and the name doesn't say what it measures, as `Mag X (G)` or `pyro_c`. Then:
//!
//! | The unit measures | The channel's quantity |
//! |---|---|
//! | a length | [`Quantity::GnssAltitude`] if the name says `gps` or `gnss` and not `agl`; [`Quantity::AltitudeAboveGround`] or [`Quantity::AltitudeMsl`] if it says `baro` and `agl`, or `baro` and `msl` or `asl`; [`Quantity::InertialAltitude`] if it says `inertial`; otherwise [`Quantity::Altitude`], an altitude whose sensor or zero the file doesn't state |
//! | a time | [`Quantity::Duration`] |
//! | an angle | [`Quantity::Latitude`] or [`Quantity::Longitude`] if the name says so, otherwise [`Quantity::Angle`] |
//! | anything else | its own quantity: [`Quantity::Speed`], [`Quantity::Acceleration`], [`Quantity::AngularRate`], [`Quantity::Temperature`], [`Quantity::Voltage`], [`Quantity::Current`], [`Quantity::Pressure`] |
//!
//! A column with no unit is kept as written. A column with no number in it, such as a flight
//! phase written as words, is no channel: if its name says `state`, `phase`, `status`, `event`
//! or `mode`, each change of its word is an [`Event`] at the row's time; otherwise it is left
//! out, with a note.
//!
//! **The values' check.** A column read in a unit is kept as written instead if any of its
//! values, converted, lies outside what that quantity can be on a flight logger: a pressure below
//! 0 or above 110 kPa, a little over the highest air pressure measured at sea level, 108.38 kPa
//! (Agata, Siberia, 1968; WMO's archive of weather extremes); a temperature below −100 °C or
//! above 150 °C, past −40 °C to 125 °C, the automotive grade 1 range (AEC-Q100); a length beyond
//! 150 km; a speed beyond 3,000 m/s, more than twice Mach 3.5, the simulator's limit; an acceleration
//! beyond 1,000 g; a rate of turn beyond 50,000 °/s; a voltage or a current beyond 100; a
//! latitude beyond 90° or a longitude beyond 180°. Angles but those and times aren't checked.
//! The clock's unit is checked by its step: the median step forward, converted, must lie from
//! 0.1 ms to 60 s, from 10,000 samples a second to one a minute; a clock outside it, or that
//! never steps forward, as a file of one row, is refused. Milliseconds headed as seconds pass
//! when the logger writes more than about 17 rows a second, a step of 60 ms or less, and then
//! the flight's times come out a thousand times too long; so do microseconds headed as
//! milliseconds. The check can only keep a column as written; it never changes a unit.
//!
//! **Rows.** Lines end in CR, LF or both. A cell is trimmed of spaces, and of double quotes
//! around it (a quoted cell may hold the separator; `""` in it is one quote). The file's decimal
//! mark is settled once: a point, unless the separator isn't a comma, no number below the header
//! is written with a point, and one is written with a comma before other than three digits, as
//! `0,05`, which no thousands separator is. A cell in the other mark, such as `1,250` in a file
//! of points or in one whose commas all stand before three digits, isn't a number: a gap,
//! counted in a note, so a thousands separator is never read as a decimal mark. A row must have as many cells as the header names, and a number in the
//! clock's cell: a row that doesn't is refused on its line. Blank lines and lines starting `#`
//! are skipped. The clock may step back, as a logger's clock does when it restarts; a note counts
//! the steps back, and every channel keeps the rows in the file's order.
//!
//! **The record.** Every column but the clock is a channel on the clock, in the file's order; the
//! clock is the logger's own, its zero unstated, so times are kept as the file writes them,
//! converted to seconds. The record has no primary track ([`FlightLog::time_s`] and
//! [`FlightLog::altitude_m`] are empty): which column is a barometric altitude above the pad isn't
//! stated, so the readings that need one wait for [M7.2][roadmap] to take them from the channels.
//!
//! **A column mapping.** [`mapping`] returns what the reader takes from the header, as a
//! [`Mapping`] a person or a program can change; [`read_with`] reads a file by a mapping, with the
//! same checks. A mapping can name the columns of a file with no header row, state a unit the
//! header leaves out, or keep a column as written; a unit it states must measure what its
//! quantity does, or the mapping is refused ([`LogError::Mapping`]).
//!
//! ```
//! use hpr_flightdata::generic;
//! use hpr_flightdata::log::Quantity;
//!
//! // An invented table: seconds, an altitude above ground in feet, a speed with no unit.
//! let text = "Time (s),Alt AGL (ft),Velocity\n0.0,0,0\n0.5,100,190\n1.0,180,140\n";
//! let log = generic::read(text)?;
//! let altitude = &log.channels[0];
//! assert_eq!(altitude.quantity, Quantity::Altitude);
//! assert_eq!(altitude.values[1], 100.0 * 0.3048);
//! assert_eq!(log.channels[1].quantity, Quantity::Unrecognized); // no unit stated
//!
//! // The same file, its velocity in feet a second by the caller's word.
//! let mut mapping = generic::mapping(text)?;
//! mapping.columns[2].quantity = Some(Quantity::Speed);
//! mapping.columns[2].unit = Some("ft/s".to_owned());
//! let log = generic::read_with(text, &mapping)?;
//! assert_eq!(log.channels[1].values[1], 190.0 * 0.3048);
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "table read by its header";

/// The most columns a header or a mapping may name: the widest table among the project's sample
/// logs names 99.
pub const MAX_COLUMNS: usize = 512;

/// How many lines from the file's start the header may lie within.
const HEAD_LINES: usize = 64;

/// The separators a header may use, in the order a tie is settled.
const SEPARATORS: [char; 4] = [',', ';', '\t', '|'];

/// Words that make a column something other than the quantity its unit measures: an estimate's
/// error, or a difference such as a pitot tube's pressure (module docs, *What a column is*).
const NOT_THE_QUANTITY: [&str; 16] = [
    "accuracy",
    "error",
    "err",
    "std",
    "stdev",
    "stddev",
    "sigma",
    "var",
    "variance",
    "uncertainty",
    "diff",
    "differential",
    "dynamic",
    "dyn",
    "pitot",
    "delta",
];

/// Units whose spelling also names a unit the reader doesn't know, read only under a name that
/// says what they measure: `g` and `mg` for grams or gauss, a single letter for many things
/// (module docs, *What a column is*).
const AMBIGUOUS_UNITS: [&str; 8] = ["g", "gs", "mg", "c", "f", "v", "k", "a"];

/// The single-letter units a name's underscored ending may state (module docs, *Units in a
/// name*).
const SINGLE_LETTER_SUFFIXES: [&str; 6] = ["s", "m", "g", "c", "f", "v"];

/// How a file's columns are read: the header's line, the separator, and what each column holds.
/// [`mapping`] returns the one the header states; a caller may change it and read the file by it
/// with [`read_with`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    /// The header row's line, from 1; `None` for a file with no header row, whose rows start at
    /// its first line that isn't blank or a `#` comment.
    pub header_line: Option<usize>,
    /// The character between cells.
    pub separator: char,
    /// Whether a comma in a cell is the decimal mark, as `1,5` for 1.5; only where the separator
    /// isn't a comma. Either way, a cell in the other mark is no number (module docs, *Rows*).
    pub decimal_comma: bool,
    /// The clock's column, from 0. Its [`ColumnMapping::unit`] must be a unit of time.
    pub clock: usize,
    /// Every column, in the file's order.
    pub columns: Vec<ColumnMapping>,
}

/// What one column holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnMapping {
    /// The column's name, as the header writes it, or as the caller names it.
    pub name: String,
    /// What the column measures; `None` to keep its numbers as written
    /// ([`Quantity::Unrecognized`]). Ignored for the clock.
    pub quantity: Option<Quantity>,
    /// The unit the values are written in, one of [`unit_spellings`], such as `ft` or
    /// `m/s^2`, in any case; `None` when [`ColumnMapping::quantity`] is `None` or
    /// [`Quantity::Number`].
    pub unit: Option<String>,
}

/// What a unit measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dimension {
    Time,
    Length,
    Speed,
    Acceleration,
    Angle,
    AngularRate,
    Temperature,
    Voltage,
    Current,
    Pressure,
}

impl Dimension {
    fn word(self) -> &'static str {
        match self {
            Self::Time => "a time",
            Self::Length => "a length",
            Self::Speed => "a speed",
            Self::Acceleration => "an acceleration",
            Self::Angle => "an angle",
            Self::AngularRate => "a rate of turn",
            Self::Temperature => "a temperature",
            Self::Voltage => "a voltage",
            Self::Current => "a current",
            Self::Pressure => "a pressure",
        }
    }

    /// The values a quantity of this dimension can take on a flight logger, in SI (or degrees),
    /// `None` where none is checked (module docs, *The values' check*).
    fn bounds(self) -> Option<(f64, f64)> {
        match self {
            Self::Pressure => Some((0.0, MAX_PRESSURE_PA)),
            Self::Temperature => Some((
                CELSIUS_ZERO_K + MIN_TEMPERATURE_C,
                CELSIUS_ZERO_K + MAX_TEMPERATURE_C,
            )),
            Self::Length => Some((-MAX_LENGTH_M, MAX_LENGTH_M)),
            Self::Speed => Some((-MAX_SPEED_M_S, MAX_SPEED_M_S)),
            Self::Acceleration => {
                Some((-MAX_ACCELERATION_G * G0_M_S2, MAX_ACCELERATION_G * G0_M_S2))
            }
            Self::AngularRate => Some((-MAX_RATE_DEG_S, MAX_RATE_DEG_S)),
            Self::Voltage | Self::Current => Some((-MAX_ELECTRIC, MAX_ELECTRIC)),
            Self::Time | Self::Angle => None,
        }
    }
}

/// The highest pressure a column may hold, Pa: a little over the highest sea-level pressure on
/// record, 108.38 kPa (Agata, Siberia, 1968, WMO's archive of weather and climate extremes).
const MAX_PRESSURE_PA: f64 = 110_000.0;
/// The coldest and hottest a logger's temperature may read, °C.
const MIN_TEMPERATURE_C: f64 = -100.0;
const MAX_TEMPERATURE_C: f64 = 150.0;
/// The largest length a column may hold, m.
const MAX_LENGTH_M: f64 = 150_000.0;
/// The largest speed, m/s.
const MAX_SPEED_M_S: f64 = 3_000.0;
/// The largest acceleration, in standard gravities.
const MAX_ACCELERATION_G: f64 = 1_000.0;
/// The largest rate of turn, °/s.
const MAX_RATE_DEG_S: f64 = 50_000.0;
/// The largest voltage, V, or current, A.
const MAX_ELECTRIC: f64 = 100.0;
/// The shortest and longest median step a clock may take, s.
const MIN_CLOCK_STEP_S: f64 = 1e-4;
const MAX_CLOCK_STEP_S: f64 = 60.0;

/// 0 °C, K.
const CELSIUS_ZERO_K: f64 = 273.15;
/// One foot, m.
const FOOT_M: f64 = 0.3048;
/// Standard gravity, m/s² (CGPM 1901).
const G0_M_S2: f64 = 9.806_65;
/// A radian, degrees.
const RADIAN_DEG: f64 = 180.0 / std::f64::consts::PI;

/// A unit: what it measures, and the value in SI (or degrees) as `written × multiply / divide +
/// add`. Dividing keeps a thousandth exact, as 1.5 s for 1500 ms.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Unit {
    dimension: Dimension,
    multiply: f64,
    divide: f64,
    add: f64,
}

impl Unit {
    const fn new(dimension: Dimension, multiply: f64, divide: f64) -> Self {
        Self {
            dimension,
            multiply,
            divide,
            add: 0.0,
        }
    }

    const fn offset(dimension: Dimension, multiply: f64, divide: f64, add: f64) -> Self {
        Self {
            dimension,
            multiply,
            divide,
            add,
        }
    }

    fn convert(self, written: f64) -> f64 {
        written * self.multiply / self.divide + self.add
    }
}

/// Degrees Fahrenheit: `(F − 32) × 5 / 9 + 273.15`.
const FAHRENHEIT: Unit = Unit::offset(
    Dimension::Temperature,
    5.0,
    9.0,
    CELSIUS_ZERO_K - 32.0 * 5.0 / 9.0,
);
const CELSIUS: Unit = Unit::offset(Dimension::Temperature, 1.0, 1.0, CELSIUS_ZERO_K);

/// Every spelling of a unit the reader knows, in lower case with spaces taken out (module docs,
/// *Units in a name*): what a [`ColumnMapping::unit`] may be.
pub fn unit_spellings() -> impl Iterator<Item = &'static str> {
    UNIT_TABLE.iter().map(|(name, _)| *name)
}

/// Every spelling of a unit the reader knows, and the unit.
const UNIT_TABLE: &[(&str, Unit)] = &[
    ("s", Unit::new(Dimension::Time, 1.0, 1.0)),
    ("sec", Unit::new(Dimension::Time, 1.0, 1.0)),
    ("secs", Unit::new(Dimension::Time, 1.0, 1.0)),
    ("second", Unit::new(Dimension::Time, 1.0, 1.0)),
    ("seconds", Unit::new(Dimension::Time, 1.0, 1.0)),
    ("ms", Unit::new(Dimension::Time, 1.0, 1e3)),
    ("msec", Unit::new(Dimension::Time, 1.0, 1e3)),
    ("millis", Unit::new(Dimension::Time, 1.0, 1e3)),
    ("millisecond", Unit::new(Dimension::Time, 1.0, 1e3)),
    ("milliseconds", Unit::new(Dimension::Time, 1.0, 1e3)),
    ("us", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("µs", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("μs", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("usec", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("microsecond", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("microseconds", Unit::new(Dimension::Time, 1.0, 1e6)),
    ("m", Unit::new(Dimension::Length, 1.0, 1.0)),
    ("meter", Unit::new(Dimension::Length, 1.0, 1.0)),
    ("meters", Unit::new(Dimension::Length, 1.0, 1.0)),
    ("ft", Unit::new(Dimension::Length, FOOT_M, 1.0)),
    ("feet", Unit::new(Dimension::Length, FOOT_M, 1.0)),
    ("foot", Unit::new(Dimension::Length, FOOT_M, 1.0)),
    ("km", Unit::new(Dimension::Length, 1e3, 1.0)),
    ("m/s", Unit::new(Dimension::Speed, 1.0, 1.0)),
    ("mps", Unit::new(Dimension::Speed, 1.0, 1.0)),
    ("m_s", Unit::new(Dimension::Speed, 1.0, 1.0)),
    ("ft/s", Unit::new(Dimension::Speed, FOOT_M, 1.0)),
    ("fps", Unit::new(Dimension::Speed, FOOT_M, 1.0)),
    ("ft_s", Unit::new(Dimension::Speed, FOOT_M, 1.0)),
    ("km/h", Unit::new(Dimension::Speed, 1.0, 3.6)),
    ("kmh", Unit::new(Dimension::Speed, 1.0, 3.6)),
    ("kph", Unit::new(Dimension::Speed, 1.0, 3.6)),
    ("m_per_s", Unit::new(Dimension::Speed, 1.0, 1.0)),
    ("meters_per_s", Unit::new(Dimension::Speed, 1.0, 1.0)),
    ("ft_per_s", Unit::new(Dimension::Speed, FOOT_M, 1.0)),
    ("feet_per_s", Unit::new(Dimension::Speed, FOOT_M, 1.0)),
    ("mph", Unit::new(Dimension::Speed, 0.447_04, 1.0)),
    ("kn", Unit::new(Dimension::Speed, 1_852.0, 3_600.0)),
    ("kt", Unit::new(Dimension::Speed, 1_852.0, 3_600.0)),
    ("knots", Unit::new(Dimension::Speed, 1_852.0, 3_600.0)),
    ("m/s2", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("m/s²", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("m/s^2", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("m/s/s", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("m_s2", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("mps2", Unit::new(Dimension::Acceleration, 1.0, 1.0)),
    ("ft/s2", Unit::new(Dimension::Acceleration, FOOT_M, 1.0)),
    ("ft/s²", Unit::new(Dimension::Acceleration, FOOT_M, 1.0)),
    ("ft/s^2", Unit::new(Dimension::Acceleration, FOOT_M, 1.0)),
    ("g", Unit::new(Dimension::Acceleration, G0_M_S2, 1.0)),
    ("gs", Unit::new(Dimension::Acceleration, G0_M_S2, 1.0)),
    ("mg", Unit::new(Dimension::Acceleration, G0_M_S2, 1e3)),
    ("milli-g", Unit::new(Dimension::Acceleration, G0_M_S2, 1e3)),
    ("deg", Unit::new(Dimension::Angle, 1.0, 1.0)),
    ("degs", Unit::new(Dimension::Angle, 1.0, 1.0)),
    ("degree", Unit::new(Dimension::Angle, 1.0, 1.0)),
    ("degrees", Unit::new(Dimension::Angle, 1.0, 1.0)),
    ("°", Unit::new(Dimension::Angle, 1.0, 1.0)),
    ("rad", Unit::new(Dimension::Angle, RADIAN_DEG, 1.0)),
    ("radians", Unit::new(Dimension::Angle, RADIAN_DEG, 1.0)),
    ("deg/s", Unit::new(Dimension::AngularRate, 1.0, 1.0)),
    ("°/s", Unit::new(Dimension::AngularRate, 1.0, 1.0)),
    ("dps", Unit::new(Dimension::AngularRate, 1.0, 1.0)),
    ("deg_per_s", Unit::new(Dimension::AngularRate, 1.0, 1.0)),
    ("degrees/s", Unit::new(Dimension::AngularRate, 1.0, 1.0)),
    ("rad/s", Unit::new(Dimension::AngularRate, RADIAN_DEG, 1.0)),
    ("rad_s", Unit::new(Dimension::AngularRate, RADIAN_DEG, 1.0)),
    (
        "rad_per_s",
        Unit::new(Dimension::AngularRate, RADIAN_DEG, 1.0),
    ),
    (
        "radians/s",
        Unit::new(Dimension::AngularRate, RADIAN_DEG, 1.0),
    ),
    ("degc", CELSIUS),
    ("°c", CELSIUS),
    ("c", CELSIUS),
    ("celsius", CELSIUS),
    ("degf", FAHRENHEIT),
    ("°f", FAHRENHEIT),
    ("f", FAHRENHEIT),
    ("fahrenheit", FAHRENHEIT),
    ("k", Unit::new(Dimension::Temperature, 1.0, 1.0)),
    ("kelvin", Unit::new(Dimension::Temperature, 1.0, 1.0)),
    ("v", Unit::new(Dimension::Voltage, 1.0, 1.0)),
    ("volt", Unit::new(Dimension::Voltage, 1.0, 1.0)),
    ("volts", Unit::new(Dimension::Voltage, 1.0, 1.0)),
    ("mv", Unit::new(Dimension::Voltage, 1.0, 1e3)),
    ("millivolts", Unit::new(Dimension::Voltage, 1.0, 1e3)),
    ("a", Unit::new(Dimension::Current, 1.0, 1.0)),
    ("amp", Unit::new(Dimension::Current, 1.0, 1.0)),
    ("amps", Unit::new(Dimension::Current, 1.0, 1.0)),
    ("ma", Unit::new(Dimension::Current, 1.0, 1e3)),
    ("milliamps", Unit::new(Dimension::Current, 1.0, 1e3)),
    ("pa", Unit::new(Dimension::Pressure, 1.0, 1.0)),
    ("pascal", Unit::new(Dimension::Pressure, 1.0, 1.0)),
    ("pascals", Unit::new(Dimension::Pressure, 1.0, 1.0)),
    ("hpa", Unit::new(Dimension::Pressure, 100.0, 1.0)),
    ("mbar", Unit::new(Dimension::Pressure, 100.0, 1.0)),
    ("millibar", Unit::new(Dimension::Pressure, 100.0, 1.0)),
    ("kpa", Unit::new(Dimension::Pressure, 1e3, 1.0)),
    ("bar", Unit::new(Dimension::Pressure, 1e5, 1.0)),
    ("atm", Unit::new(Dimension::Pressure, 101_325.0, 1.0)),
    ("psi", Unit::new(Dimension::Pressure, 6_894.757, 1.0)),
    ("inhg", Unit::new(Dimension::Pressure, 3_386.389, 1.0)),
];

/// A unit's key: lower case, spaces taken out.
fn key(written: &str) -> String {
    written
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// A unit as written, if the reader knows it.
fn unit_of(written: &str) -> Option<Unit> {
    let key = key(written);
    UNIT_TABLE
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, unit)| *unit)
}

/// A header name split into the name, with any qualifier from its brackets after it, and the
/// unit it states, as its key (module docs, *Units in a name*).
fn split_name(written: &str) -> (String, Option<String>) {
    let written = written.trim();
    if let Some(close) = written.chars().last().filter(|c| *c == ')' || *c == ']') {
        let open = if close == ')' { '(' } else { '[' };
        if let Some(at) = written.rfind(open) {
            let inner = &written[at + open.len_utf8()..written.len() - close.len_utf8()];
            let base = written[..at].trim_end_matches([' ', '_']);
            if unit_of(inner).is_some() {
                return (base.to_owned(), Some(key(inner)));
            }
            let mut words = inner.split_whitespace();
            if let Some(first) = words.next()
                && unit_of(first).is_some()
            {
                let qualifier: Vec<&str> = words.collect();
                let name = if qualifier.is_empty() {
                    base.to_owned()
                } else {
                    format!("{base} {}", qualifier.join(" "))
                };
                return (name, Some(key(first)));
            }
            return (written.to_owned(), None);
        }
    }
    let words: Vec<&str> = written.split('_').filter(|w| !w.is_empty()).collect();
    for count in (1..=4).rev() {
        if words.len() <= count {
            continue;
        }
        let ending = words[words.len() - count..].join("_");
        let ending_key = key(&ending);
        if ending_key.chars().count() == 1 && !SINGLE_LETTER_SUFFIXES.contains(&ending_key.as_str())
        {
            continue;
        }
        if unit_of(&ending).is_some() {
            return (words[..words.len() - count].join("_"), Some(ending_key));
        }
    }
    (written.to_owned(), None)
}

/// A name's words: lower case, split at anything but a letter or a digit.
fn words(name: &str) -> Vec<String> {
    name.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Whether a name's words name a time, as a clock's do.
fn is_time_name(words: &[String]) -> bool {
    words
        .iter()
        .any(|w| matches!(w.as_str(), "t" | "ts" | "uptime" | "timestamp") || w.starts_with("time"))
}

/// What a name's words say a column may measure (module docs, *What a column is*).
fn hints(words: &[String]) -> Vec<Dimension> {
    let mut hints = Vec::new();
    let mut hint = |dimension: Dimension, said: bool| {
        if said && !hints.contains(&dimension) {
            hints.push(dimension);
        }
    };
    let any = |test: &dyn Fn(&str) -> bool| words.iter().any(|w| test(w));
    hint(
        Dimension::Length,
        any(&|w| {
            w.starts_with("alt")
                || w.ends_with("alt")
                || matches!(w, "height" | "elev" | "elevation" | "agl" | "msl" | "asl")
        }),
    );
    hint(
        Dimension::Pressure,
        any(&|w| w.starts_with("press") || w.starts_with("baro") || w == "pression"),
    );
    hint(Dimension::Temperature, any(&|w| w.starts_with("temp")));
    hint(
        Dimension::Voltage,
        any(&|w| w.starts_with("volt") || w.starts_with("batt") || w.starts_with("vbat")),
    );
    hint(
        Dimension::Current,
        any(&|w| w.starts_with("curr") || w.starts_with("amp")),
    );
    hint(
        Dimension::Acceleration,
        any(&|w| w.starts_with("acc") || w.ends_with("acc") || w.contains("accel")),
    );
    hint(
        Dimension::AngularRate,
        any(&|w| w.starts_with("gyro") || w == "rate"),
    );
    hint(
        Dimension::Speed,
        any(&|w| {
            w.starts_with("vel") || w.starts_with("speed") || matches!(w, "vx" | "vy" | "vz" | "v")
        }),
    );
    hint(
        Dimension::Angle,
        any(&|w| {
            matches!(
                w,
                "lat"
                    | "latitude"
                    | "lon"
                    | "lng"
                    | "long"
                    | "longitude"
                    | "heading"
                    | "angle"
                    | "tilt"
                    | "roll"
                    | "pitch"
                    | "yaw"
                    | "course"
                    | "bearing"
                    | "cog"
            )
        }),
    );
    hint(Dimension::Time, is_time_name(words));
    hints
}

/// The quantity a column of a known dimension is, by its name's words (module docs, *What a
/// column is*).
fn quantity_of(dimension: Dimension, words: &[String]) -> Quantity {
    let says = |options: &[&str]| words.iter().any(|w| options.contains(&w.as_str()));
    let mentions = |part: &str| words.iter().any(|w| w.contains(part));
    match dimension {
        Dimension::Length => {
            let agl = says(&["agl"]);
            if (mentions("gps") || mentions("gnss")) && !agl {
                Quantity::GnssAltitude
            } else if mentions("baro") && agl {
                Quantity::AltitudeAboveGround
            } else if mentions("baro") && (says(&["msl", "asl"])) {
                Quantity::AltitudeMsl
            } else if mentions("inertial") {
                Quantity::InertialAltitude
            } else {
                Quantity::Altitude
            }
        }
        Dimension::Time => Quantity::Duration,
        Dimension::Angle => {
            if says(&["lat", "latitude"]) {
                Quantity::Latitude
            } else if says(&["lon", "lng", "long", "longitude"]) {
                Quantity::Longitude
            } else {
                Quantity::Angle
            }
        }
        Dimension::Speed => Quantity::Speed,
        Dimension::Acceleration => Quantity::Acceleration,
        Dimension::AngularRate => Quantity::AngularRate,
        Dimension::Temperature => Quantity::Temperature,
        Dimension::Voltage => Quantity::Voltage,
        Dimension::Current => Quantity::Current,
        Dimension::Pressure => Quantity::Pressure,
    }
}

/// What a quantity's unit measures; `None` for a number with no unit.
fn dimension_of(quantity: Quantity) -> Option<Dimension> {
    match quantity {
        Quantity::AltitudeAboveGround
        | Quantity::AltitudeMsl
        | Quantity::GnssAltitude
        | Quantity::InertialAltitude
        | Quantity::Altitude
        | Quantity::Distance => Some(Dimension::Length),
        Quantity::Pressure => Some(Dimension::Pressure),
        Quantity::Speed => Some(Dimension::Speed),
        Quantity::Acceleration => Some(Dimension::Acceleration),
        Quantity::Temperature => Some(Dimension::Temperature),
        Quantity::Voltage => Some(Dimension::Voltage),
        Quantity::Current => Some(Dimension::Current),
        Quantity::Latitude | Quantity::Longitude | Quantity::Angle => Some(Dimension::Angle),
        Quantity::AngularRate => Some(Dimension::AngularRate),
        Quantity::TimeOfDay | Quantity::Duration => Some(Dimension::Time),
        Quantity::Number | Quantity::Unrecognized => None,
    }
}

/// The values a quantity may hold: its dimension's, narrowed for a latitude and a longitude.
fn bounds_of(quantity: Quantity, dimension: Dimension) -> Option<(f64, f64)> {
    match quantity {
        Quantity::Latitude => Some((-90.0, 90.0)),
        Quantity::Longitude => Some((-180.0, 180.0)),
        _ => dimension.bounds(),
    }
}

/// A column as the header states it, and why a column is kept as written, if it is.
fn infer(written: &str) -> (ColumnMapping, Option<String>) {
    let (name, unit_key) = split_name(written);
    let words = words(&name);
    let kept = |why: String| {
        (
            ColumnMapping {
                name: written.to_owned(),
                quantity: None,
                unit: None,
            },
            Some(why),
        )
    };
    let Some((unit_key, unit)) = unit_key.and_then(|k| unit_of(&k).map(|u| (k, u))) else {
        return kept(format!(
            "the column {} states no unit the reader knows: kept as written",
            quoted(written)
        ));
    };
    if let Some(word) = words
        .iter()
        .find(|w| NOT_THE_QUANTITY.contains(&w.as_str()))
    {
        return kept(format!(
            "the column {}'s name says {word:?}, an estimate's error or a difference, not the \
             quantity itself: kept as written",
            quoted(written)
        ));
    }
    let hints = hints(&words);
    if hints.is_empty() && matches!(unit.dimension, Dimension::Length | Dimension::Time) {
        return kept(format!(
            "the column {} is {} whose name says no altitude or time: kept as written",
            quoted(written),
            unit.dimension.word()
        ));
    }
    if hints.is_empty() && AMBIGUOUS_UNITS.contains(&unit_key.as_str()) {
        return kept(format!(
            "the column {}'s unit, {unit_key}, may stand for something other than {}, and its \
             name doesn't say which: kept as written",
            quoted(written),
            unit.dimension.word()
        ));
    }
    if unit.dimension == Dimension::Angle && hints.contains(&Dimension::AngularRate) {
        return kept(format!(
            "the column {}'s name says a rate of turn and its unit, {unit_key}, an angle: kept as \
             written",
            quoted(written)
        ));
    }
    if !hints.is_empty() && !hints.contains(&unit.dimension) {
        let named: Vec<&str> = hints.iter().map(|d| d.word()).collect();
        return kept(format!(
            "the column {}'s name says {} and its unit, {unit_key}, {}: kept as written",
            quoted(written),
            named.join(" or "),
            unit.dimension.word()
        ));
    }
    (
        ColumnMapping {
            name: written.to_owned(),
            quantity: Some(quantity_of(unit.dimension, &words)),
            unit: Some(unit_key),
        },
        None,
    )
}

fn quoted(name: &str) -> String {
    format!("{:?}", short(name))
}

/// A line's cells, split at the separator outside double quotes, each trimmed and unquoted; at
/// most `limit` of them, so a hostile line of separators isn't split whole before it is refused.
fn cells(line: &str, separator: char, limit: usize) -> Vec<Cow<'_, str>> {
    if !line.contains('"') {
        return line
            .split(separator)
            .take(limit)
            .map(|c| Cow::Borrowed(c.trim()))
            .collect();
    }
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cell.push('"');
                    chars.next();
                } else {
                    quoted = false;
                }
            } else {
                cell.push(c);
            }
        } else if c == '"' && cell.trim().is_empty() {
            cell.clear();
            quoted = true;
        } else if c == separator {
            cells.push(Cow::Owned(cell.trim().to_owned()));
            if cells.len() == limit {
                return cells;
            }
            cell.clear();
        } else {
            cell.push(c);
        }
    }
    cells.push(Cow::Owned(cell.trim().to_owned()));
    cells
}

/// A cell as a number in the file's decimal mark, `None` if it isn't one. With a decimal comma, a
/// cell holding a point isn't a number; with a point, one holding a comma isn't: either is a
/// thousands separator or a mix the reader can't settle, never a decimal mark (module docs,
/// *Rows*).
fn number(cell: &str, decimal_comma: bool) -> Option<f64> {
    if decimal_comma && cell.contains('.') {
        return None;
    }
    table::number(cell, decimal_comma)
}

/// Whether the file's numbers are written with a decimal comma: only where the separator isn't a
/// comma, no number below the header is written with a point, and one is written with a comma
/// followed by other than three digits, which no thousands separator is (module docs, *Rows*).
fn decimal_comma(text: &str, header_line: usize, separator: char) -> bool {
    if separator == ',' {
        return false;
    }
    let mut shown = false;
    for (_, line) in numbered_lines(text).filter(|(n, l)| *n > header_line && !skipped(l)) {
        for cell in cells(line, separator, MAX_COLUMNS + 1) {
            if cell.contains('.') && table::number(&cell, false).is_some() {
                return false;
            }
            shown |= table::number(&cell, true).is_some()
                && cell.split_once(',').is_some_and(|(_, decimals)| {
                    decimals.len() != 3 || !decimals.bytes().all(|b| b.is_ascii_digit())
                });
        }
    }
    shown
}

/// Whether a line is skipped: blank, or a `#` comment.
fn skipped(line: &str) -> bool {
    let line = line.trim();
    line.is_empty() || line.starts_with('#')
}

/// The separator a `sep=` line names, if the line is one.
fn sep_line(line: &str) -> Option<char> {
    let rest = line.trim().strip_prefix("sep=")?;
    let mut chars = rest.chars();
    let separator = chars
        .next()
        .filter(|c| SEPARATORS.contains(c) || *c == ' ')?;
    chars.next().is_none().then_some(separator)
}

/// The header the reader finds, and the mapping it states, with a note on each column kept as
/// written; `None` if the file isn't in this layout (module docs, *Recognition*).
fn find_header(text: &str) -> Option<(Mapping, Vec<String>)> {
    let mut named: Option<char> = None;
    let mut lines = numbered_lines(text)
        .take(HEAD_LINES)
        .filter(|(_, l)| !skipped(l));
    let (line, header) = loop {
        let (number, line) = lines.next()?;
        if named.is_none()
            && let Some(separator) = sep_line(line)
        {
            named = Some(separator);
            continue;
        }
        break (number, line);
    };
    let separator = named.unwrap_or_else(|| {
        let mut best = (SEPARATORS[0], 0);
        for separator in SEPARATORS {
            let count = cells(header, separator, MAX_COLUMNS + 1).len();
            if count > best.1 {
                best = (separator, count);
            }
        }
        best.0
    });
    let names = cells(header, separator, MAX_COLUMNS + 1);
    if names.len() < 2 || names.len() > MAX_COLUMNS {
        return None;
    }
    if names
        .iter()
        .any(|name| table::number(name, separator != ',').is_some())
    {
        return None;
    }
    let mut columns = Vec::with_capacity(names.len());
    let mut notes = Vec::new();
    let mut clock = None;
    for name in &names {
        let (mapping, why) = infer(name);
        let is_clock = clock.is_none()
            && mapping
                .unit
                .as_deref()
                .and_then(unit_of)
                .map(|u| u.dimension)
                == Some(Dimension::Time)
            && is_time_name(&words(&split_name(name).0));
        if is_clock {
            clock = Some(columns.len());
        } else if let Some(why) = why {
            notes.push(why);
        }
        columns.push(mapping);
    }
    let clock = clock?;
    let (_, first) = numbered_lines(text)
        .skip_while(|(number, _)| *number <= line)
        .find(|(_, l)| !skipped(l))?;
    let decimal_comma = decimal_comma(text, line, separator);
    let first = cells(first, separator, MAX_COLUMNS + 1);
    number(first.get(clock)?, decimal_comma)?;
    if let Some(column) = columns.get_mut(clock) {
        column.quantity = None;
    }
    Some((
        Mapping {
            header_line: Some(line),
            separator,
            decimal_comma,
            clock,
            columns,
        },
        notes,
    ))
}

/// Whether the text is in this layout, judged from its first lines, never the file's name.
pub fn recognizes(text: &str) -> bool {
    find_header(text).is_some()
}

/// What the reader takes from the file's header: its line, the separator and each column's
/// quantity and unit, as [`read`] would read them.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the file isn't in this layout (module docs, *Recognition*).
pub fn mapping(text: &str) -> Result<Mapping, LogError> {
    find_header(text)
        .map(|(mapping, _)| mapping)
        .ok_or_else(not_this_format)
}

fn not_this_format() -> LogError {
    LogError::NotThisFormat {
        format: FORMAT,
        message: format!(
            "within its first {HEAD_LINES} lines there is no header naming a clock with a unit of \
             time, such as `Time (s)` or `time__us`, followed by a row with a number there"
        ),
    }
}

/// Reads a table by what its header states (module docs).
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the file isn't in this layout; otherwise as [`read_with`].
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let (mapping, notes) = find_header(text).ok_or_else(not_this_format)?;
    read_mapped(text, &mapping, notes)
}

/// Reads a table by the caller's column mapping, with the checks [`read`] makes.
///
/// # Errors
///
/// [`LogError::Mapping`] for a mapping of no column or more than [`MAX_COLUMNS`], whose clock
/// isn't one of its columns or isn't in a unit of time, or a column whose unit the reader doesn't
/// know or doesn't measure what its quantity does; [`LogError::Syntax`] for a row of other than
/// the mapping's width, a clock cell that isn't a number, or a reading its unit's conversion makes
/// infinite, naming the line; [`LogError::Unit`] for a clock that never steps forward or whose
/// median step, in its unit, lies outside 0.1 ms to 60 s; [`LogError::NoData`] if no row follows the header.
pub fn read_with(text: &str, mapping: &Mapping) -> Result<FlightLog, LogError> {
    read_mapped(text, mapping, Vec::new())
}

fn mapping_error(message: String) -> LogError {
    LogError::Mapping { message }
}

/// Each column's unit, checked against its quantity; `None` for one kept as written.
fn units(mapping: &Mapping) -> Result<(Unit, Vec<Option<Unit>>), LogError> {
    let width = mapping.columns.len();
    if width == 0 || width > MAX_COLUMNS {
        return Err(mapping_error(format!(
            "a mapping names from 1 to {MAX_COLUMNS} columns; this one names {width}"
        )));
    }
    if !SEPARATORS.contains(&mapping.separator) && mapping.separator != ' ' {
        return Err(mapping_error(format!(
            "the separator {:?} isn't one of `,`, `;`, a tab, `|` or a space",
            mapping.separator
        )));
    }
    if mapping.decimal_comma && mapping.separator == ',' {
        return Err(mapping_error(
            "a decimal comma needs a separator other than a comma".to_owned(),
        ));
    }
    let Some(clock) = mapping.columns.get(mapping.clock) else {
        return Err(mapping_error(format!(
            "the clock is column {}, of {width}",
            mapping.clock
        )));
    };
    let clock_unit = clock
        .unit
        .as_deref()
        .and_then(unit_of)
        .filter(|u| u.dimension == Dimension::Time)
        .ok_or_else(|| {
            mapping_error(format!(
                "the clock, {}, has no unit of time the reader knows",
                quoted(&clock.name)
            ))
        })?;
    let mut units = Vec::with_capacity(width);
    for (index, column) in mapping.columns.iter().enumerate() {
        if index == mapping.clock {
            units.push(None);
            continue;
        }
        let wanted = column.quantity.and_then(dimension_of);
        let unit = match (&column.unit, wanted) {
            (None, None) => None,
            (Some(written), Some(dimension)) => match unit_of(written) {
                Some(unit) if unit.dimension == dimension => Some(unit),
                Some(unit) => {
                    return Err(mapping_error(format!(
                        "the column {} is mapped to {} in {}, which measures {}",
                        quoted(&column.name),
                        dimension.word(),
                        quoted(written),
                        unit.dimension.word()
                    )));
                }
                None => {
                    return Err(mapping_error(format!(
                        "the column {}'s unit, {}, isn't one the reader knows",
                        quoted(&column.name),
                        quoted(written)
                    )));
                }
            },
            (None, Some(dimension)) => {
                return Err(mapping_error(format!(
                    "the column {} is mapped to {} but states no unit",
                    quoted(&column.name),
                    dimension.word()
                )));
            }
            (Some(written), None) => {
                return Err(mapping_error(format!(
                    "the column {} states a unit, {}, for a quantity that has none",
                    quoted(&column.name),
                    quoted(written)
                )));
            }
        };
        units.push(unit);
    }
    Ok((clock_unit, units))
}

/// The rows as written: each row's line, the clock and each column's cells, numbers or `NaN`.
struct Rows {
    lines: Vec<usize>,
    clock: Vec<f64>,
    values: Vec<Vec<f64>>,
    /// Each column's cells that aren't numbers: how many, and the first, as a note quotes it.
    text: Vec<(usize, Option<String>)>,
    /// For each column whose name says it holds events, its word at each change: the row's index
    /// and the word. Only changes are kept, so a column of words costs no more than its events.
    changes: Vec<Vec<(usize, String)>>,
}

fn read_rows(text: &str, mapping: &Mapping) -> Result<Rows, LogError> {
    let width = mapping.columns.len();
    let syntax = |line: usize, message: String| LogError::Syntax {
        format: FORMAT,
        line,
        message,
    };
    let mut rows = Rows {
        lines: Vec::new(),
        clock: Vec::new(),
        values: vec![Vec::new(); width],
        text: vec![(0, None); width],
        changes: vec![Vec::new(); width],
    };
    let event_columns: Vec<bool> = mapping
        .columns
        .iter()
        .map(|c| is_event_name(&c.name))
        .collect();
    let start = mapping.header_line.unwrap_or(0);
    let mut header_skipped = false;
    for (line_number, line) in numbered_lines(text) {
        if line_number <= start || skipped(line) {
            continue;
        }
        if !header_skipped && mapping.header_line.is_none() && sep_line(line).is_some() {
            header_skipped = true;
            continue;
        }
        header_skipped = true;
        let cells = cells(line, mapping.separator, width + 1);
        if cells.len() != width {
            return Err(syntax(
                line_number,
                format!(
                    "the row has {} cells, where the {} names {width}",
                    if cells.len() > width {
                        "more".to_owned()
                    } else {
                        cells.len().to_string()
                    },
                    if mapping.header_line.is_some() {
                        "header"
                    } else {
                        "mapping"
                    }
                ),
            ));
        }
        let clock_cell = cells.get(mapping.clock).map_or("", |c| c.as_ref());
        let Some(time) = number(clock_cell, mapping.decimal_comma) else {
            return Err(syntax(
                line_number,
                format!("the clock's cell {} isn't a number", quoted(clock_cell)),
            ));
        };
        let row = rows.clock.len();
        for (index, cell) in cells.iter().enumerate() {
            let value = number(cell, mapping.decimal_comma);
            if value.is_none() && !cell.is_empty() {
                let (count, first) = &mut rows.text[index];
                *count += 1;
                if first.is_none() {
                    *first = Some(short(cell));
                }
                if event_columns[index] {
                    let word = short(cell);
                    let changes = &mut rows.changes[index];
                    if changes.last().is_none_or(|(_, last)| *last != word) {
                        changes.push((row, word));
                    }
                }
            }
            rows.values[index].push(value.unwrap_or(f64::NAN));
        }
        rows.lines.push(line_number);
        rows.clock.push(time);
    }
    Ok(rows)
}

/// The median of the clock's steps forward, in its unit; `None` if it never steps forward.
fn median_step(clock: &[f64]) -> Option<f64> {
    let mut steps: Vec<f64> = clock
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|step| *step > 0.0)
        .collect();
    if steps.is_empty() {
        return None;
    }
    steps.sort_by(f64::total_cmp);
    steps.get(steps.len() / 2).copied()
}

/// Whether a column's words name what an event column holds: a word that is one of them, its
/// plural, or ends in one, as `flightstate`; not one that only starts with one, as `model`.
fn is_event_name(name: &str) -> bool {
    words(name).iter().any(|w| {
        ["state", "phase", "status", "event", "mode"]
            .iter()
            .any(|word| w.ends_with(word) || w.strip_suffix('s') == Some(word))
    })
}

fn read_mapped(
    text: &str,
    mapping: &Mapping,
    header_notes: Vec<String>,
) -> Result<FlightLog, LogError> {
    let (clock_unit, units) = units(mapping)?;
    let rows = read_rows(text, mapping)?;
    if rows.clock.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    let clock_name = mapping
        .columns
        .get(mapping.clock)
        .map_or("", |c| c.name.as_str());
    let first_line = rows.lines.first().copied().unwrap_or(0);
    let unit_line = mapping.header_line.unwrap_or(first_line);
    let Some(step) = median_step(&rows.clock) else {
        return Err(LogError::Unit {
            format: FORMAT,
            line: unit_line,
            message: format!(
                "the clock, {}, never steps forward, so its values can't bear out its unit",
                quoted(clock_name)
            ),
        });
    };
    let step_s = clock_unit.convert(step);
    if !(MIN_CLOCK_STEP_S..=MAX_CLOCK_STEP_S).contains(&step_s) {
        return Err(LogError::Unit {
            format: FORMAT,
            line: unit_line,
            message: format!(
                "the clock, {}, steps by {step} at the median, {step_s} s in its unit, outside \
                 {MIN_CLOCK_STEP_S} s to {MAX_CLOCK_STEP_S} s: its values contradict the unit",
                quoted(clock_name)
            ),
        });
    }
    let mut time_s = Vec::with_capacity(rows.clock.len());
    for (value, line) in rows.clock.iter().zip(&rows.lines) {
        time_s.push(table::finite(
            clock_unit.convert(*value),
            FORMAT,
            *line,
            clock_name,
        )?);
    }
    // One note at most for each column, and one for the clock: bounded by `MAX_COLUMNS`, so all
    // are kept, unlike notes about single lines.
    let mut notes = header_notes;
    let back = rows.clock.windows(2).filter(|p| p[1] < p[0]).count();
    if back > 0 {
        notes.push(format!(
            "the clock, {}, steps back {back} times; the rows are kept in the file's order",
            quoted(clock_name)
        ));
    }
    let mut channels = Vec::new();
    let mut changes: Vec<(usize, &str)> = Vec::new();
    for (index, column) in mapping.columns.iter().enumerate() {
        if index == mapping.clock {
            continue;
        }
        let values = &rows.values[index];
        let (text, first) = &rows.text[index];
        let first = quoted(first.as_deref().unwrap_or_default());
        if values.iter().all(|v| v.is_nan()) {
            if *text == 0 {
                notes.push(format!(
                    "the column {} holds no value: left out",
                    quoted(&column.name)
                ));
            } else if is_event_name(&column.name) {
                changes.extend(rows.changes[index].iter().map(|(r, w)| (*r, w.as_str())));
            } else {
                notes.push(format!(
                    "the column {} holds text, not numbers, such as {first}: left out",
                    quoted(&column.name)
                ));
            }
            continue;
        }
        if *text > 0 {
            notes.push(format!(
                "the column {} holds {text} cells that aren't numbers, such as {first}: gaps",
                quoted(&column.name)
            ));
        }
        let quantity = column.quantity.unwrap_or(Quantity::Unrecognized);
        let (quantity, values) = match units[index] {
            Some(unit) => {
                let mut converted = Vec::with_capacity(values.len());
                for (value, line) in values.iter().zip(&rows.lines) {
                    converted.push(table::finite(
                        unit.convert(*value),
                        FORMAT,
                        *line,
                        &column.name,
                    )?);
                }
                let outside = bounds_of(quantity, unit.dimension).and_then(|(low, high)| {
                    converted
                        .iter()
                        .zip(&rows.lines)
                        .zip(values)
                        .find(|((v, _), _)| **v < low || **v > high)
                        .map(|((_, line), written)| (*line, *written, low, high))
                });
                match outside {
                    Some((line, written, low, high)) => {
                        notes.push(format!(
                            "the column {}'s values contradict its unit, {}: {written} on line \
                             {line} lies outside {low} to {high} {} once converted; kept as \
                             written",
                            quoted(&column.name),
                            column.unit.as_deref().unwrap_or_default(),
                            quantity.unit()
                        ));
                        (Quantity::Unrecognized, values.clone())
                    }
                    None => (quantity, converted),
                }
            }
            None => (quantity, values.clone()),
        };
        channels.push(Channel {
            column: column.name.clone(),
            quantity,
            time_s: time_s.clone(),
            values,
        });
    }
    // In the file's order: by row, and a row's events in its columns' order (a stable sort).
    changes.sort_by_key(|(row, _)| *row);
    let events = changes
        .into_iter()
        .map(|(row, word)| Event {
            time_s: time_s[row],
            name: word.to_owned(),
        })
        .collect();
    Ok(FlightLog {
        format: LogFormat::GenericTable,
        logger: "unknown".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events,
        notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn channel<'a>(log: &'a FlightLog, name: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|c| c.column == name)
            .unwrap_or_else(|| panic!("no {name}"))
    }

    #[test]
    fn the_unit_table_and_its_names_agree() {
        assert_eq!(unit_spellings().count(), UNIT_TABLE.len());
        for (name, _) in UNIT_TABLE {
            assert_eq!(key(name), *name, "{name} isn't a key");
            assert_eq!(
                UNIT_TABLE.iter().filter(|(other, _)| other == name).count(),
                1,
                "{name} twice"
            );
        }
    }

    /// Each unit against a hand conversion.
    #[test]
    fn units_convert_by_hand() {
        let cases = [
            ("ms", 1500.0, 1.5),
            ("us", 2_500_000.0, 2.5),
            ("ft", 100.0, 30.48),
            ("km", 1.5, 1500.0),
            ("ft/s", 10.0, 3.048),
            ("km/h", 36.0, 10.0),
            ("mph", 100.0, 44.704),
            ("kn", 3600.0, 1852.0),
            ("g", 2.0, 19.6133),
            ("mg", 1000.0, 9.80665),
            ("rad", std::f64::consts::PI, 180.0),
            ("rad/s", 1.0, RADIAN_DEG),
            ("degc", 25.0, 298.15),
            ("degf", 212.0, 373.15),
            ("f", 32.0, 273.15),
            ("mv", 3700.0, 3.7),
            ("ma", 250.0, 0.25),
            ("hpa", 1013.25, 101_325.0),
            ("kpa", 101.325, 101_325.0),
            ("bar", 1.01325, 101_325.0),
            ("atm", 1.0, 101_325.0),
            ("psi", 14.695_95, 101_325.0),
            ("inhg", 29.92, 101_320.76),
        ];
        for (unit, written, expected) in cases {
            let converted = unit_of(unit)
                .unwrap_or_else(|| panic!("{unit}"))
                .convert(written);
            assert!(
                (converted - expected).abs() < 1e-2 * expected.abs().max(1.0) * 1e-3,
                "{unit}: {written} → {converted}, expected {expected}"
            );
        }
    }

    #[test]
    fn names_split_into_the_name_and_the_unit() {
        let cases: [(&str, &str, Option<&str>); 18] = [
            ("Time (s)", "Time", Some("s")),
            ("Alt AGL (ft)", "Alt AGL", Some("ft")),
            ("Altitude [m]", "Altitude", Some("m")),
            ("Temperature [deg C]", "Temperature", Some("degc")),
            ("Alt (m ASL)", "Alt ASL", Some("m")),
            ("Lon (° dec)", "Lon dec", Some("°")),
            ("Flight_Time_(s)", "Flight_Time", Some("s")),
            (
                "Baro_Altitude_AGL_(feet)",
                "Baro_Altitude_AGL",
                Some("feet"),
            ),
            ("time__us", "time", Some("us")),
            ("acc_y__m_s2", "acc_y", Some("m_s2")),
            ("pitch_rate_deg_per_s", "pitch_rate", Some("deg_per_s")),
            ("vel_north_meters_per_s", "vel_north", Some("meters_per_s")),
            ("Yacc_g", "Yacc", Some("g")),
            ("time(raw)", "time(raw)", None),
            ("flag_a", "flag_a", None),
            ("Load (grams)", "Load (grams)", None),
            ("Velocity_Up", "Velocity_Up", None),
            ("s", "s", None),
        ];
        for (written, name, unit) in cases {
            let (got_name, got_unit) = split_name(written);
            assert_eq!(
                (got_name.as_str(), got_unit.as_deref()),
                (name, unit),
                "{written}"
            );
        }
    }

    #[test]
    fn a_column_is_read_only_where_name_and_unit_agree() {
        let cases: [(&str, Option<Quantity>); 28] = [
            ("Alt AGL (ft)", Some(Quantity::Altitude)),
            (
                "Baro_Altitude_AGL_(feet)",
                Some(Quantity::AltitudeAboveGround),
            ),
            ("alt_gps_m", Some(Quantity::GnssAltitude)),
            ("Lat (° dec)", Some(Quantity::Latitude)),
            ("Lon (° dec)", Some(Quantity::Longitude)),
            ("Tilt (deg)", Some(Quantity::Angle)),
            ("baro__kpa", Some(Quantity::Pressure)),
            ("Baro (Pa)", Some(Quantity::Pressure)),
            ("speed_baro(m/s)", Some(Quantity::Speed)),
            ("gyro_y__rad_s", Some(Quantity::AngularRate)),
            ("servo_current_milliamps", Some(Quantity::Current)),
            ("Temp (F)", Some(Quantity::Temperature)),
            ("vz_filtered_ms", None),
            ("z (m)", None),
            ("sensor_read_us", None),
            ("battery_voltage", None),
            // A unit that may be another one, under a name that says nothing: gauss, grams,
            // flags. A unit that can be only one thing reads under any name but a length's.
            ("Fin Position (deg)", Some(Quantity::Angle)),
            ("Apo_Volts", Some(Quantity::Voltage)),
            ("Mag X (G)", None),
            ("Load (g)", None),
            ("kalman_v", None),
            ("pyro_c", None),
            // A rate of turn's name over an angle's unit.
            ("Roll rate (deg)", None),
            // An estimate's error, or a difference, not the quantity itself.
            ("Alt error (m)", None),
            ("Altitude accuracy (m)", None),
            ("Diff press (Pa)", None),
            ("Baro pitot (Pa)", None),
            ("Alt AGL std (ft)", None),
        ];
        for (written, quantity) in cases {
            assert_eq!(infer(written).0.quantity, quantity, "{written}");
        }
    }

    #[test]
    fn quoted_cells_hold_the_separator() {
        let got: Vec<String> = cells(r#" 1, "a,b" ,"say ""hi""","" "#, ',', usize::MAX)
            .into_iter()
            .map(Cow::into_owned)
            .collect();
        assert_eq!(got, ["1", "a,b", r#"say "hi""#, ""]);
    }

    const TABLE: &str = "# an invented flight\n\
        Time (ms);Alt AGL (ft);Pressure (hPa);Phase;Note;Batt (V)\n\
        0;0;1000,5;PAD;;7,9\n\
        50;10;999,0;PAD;x;n/a\n\
        100;30;997,5;BOOST;;7,8\n\
        150;60;995,0;COAST;;7,8\n";

    #[test]
    fn a_table_reads_by_its_header() {
        assert!(recognizes(TABLE));
        let log = read(TABLE).expect("reads");
        assert_eq!(log.format, LogFormat::GenericTable);
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        let altitude = channel(&log, "Alt AGL (ft)");
        assert_eq!(altitude.quantity, Quantity::Altitude);
        assert_eq!(altitude.time_s, [0.0, 0.05, 0.1, 0.15]);
        assert_eq!(altitude.values[3], 60.0 * FOOT_M);
        let pressure = channel(&log, "Pressure (hPa)");
        assert_eq!(pressure.values[0], 100_050.0);
        let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["PAD", "BOOST", "COAST"]);
        assert_eq!(log.events[1].time_s, 0.1);
        assert!(
            log.channels
                .iter()
                .all(|c| c.column != "Note" && c.column != "Phase")
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("\"Note\"") && n.contains("text"))
        );
        let battery = channel(&log, "Batt (V)");
        assert_eq!(battery.quantity, Quantity::Voltage);
        assert!(battery.values[1].is_nan());
        assert_eq!(battery.values[2], 7.8);
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("\"Batt (V)\"") && n.contains("1 cells"))
        );
    }

    #[test]
    fn values_that_contradict_the_unit_keep_the_column_as_written() {
        let text = "t (s),Pressure (Pa),Temp (C)\n0,101000,20\n1,150000,21\n";
        let log = read(text).expect("reads");
        let pressure = channel(&log, "Pressure (Pa)");
        assert_eq!(pressure.quantity, Quantity::Unrecognized);
        assert_eq!(pressure.values, [101_000.0, 150_000.0]);
        assert!(log.notes.iter().any(|n| n.contains("150000 on line 3")));
        assert_eq!(channel(&log, "Temp (C)").quantity, Quantity::Temperature);
        // A value inside the bounds passes.
        let text = "t (s),Pressure (Pa)\n0,101000\n1,109999\n";
        assert_eq!(
            channel(&read(text).expect("reads"), "Pressure (Pa)").quantity,
            Quantity::Pressure
        );
    }

    #[test]
    fn a_clock_whose_step_contradicts_its_unit_is_refused() {
        // Milliseconds headed as seconds at 50 Hz, a 20 s step, pass: the check can't tell them
        // from a slow logger (module docs, *The values' check*).
        let text = "Time (s),Alt (m)\n0,0\n20,1\n40,3\n";
        assert!(read(text).is_ok());
        let text = "Time (s),Alt (m)\n0,0\n61,1\n122,3\n";
        match read(text) {
            Err(LogError::Unit { message, .. }) => assert!(message.contains("61"), "{message}"),
            other => panic!("expected a unit refusal, got {other:?}"),
        }
        let text = "Time (ms),Alt (m)\n0,0\n0.05,1\n0.1,3\n";
        assert!(matches!(read(text), Err(LogError::Unit { .. })));
        // A clock that never steps forward, as a one-row file of a computer's settings.
        for text in [
            "Time (s),Alt (m)\n5,0\n",
            "Time (s),Alt (m)\n5,0\n5,1\n4,2\n",
        ] {
            match read(text) {
                Err(LogError::Unit { message, .. }) => {
                    assert!(message.contains("never steps forward"), "{message}");
                }
                other => panic!("expected a unit refusal, got {other:?}"),
            }
        }
    }

    #[test]
    fn rows_must_match_the_header() {
        let text = "Time (s),Alt (m)\n0,0\n1,1,1\n";
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => {
                assert_eq!(line, 3);
                assert!(message.contains("has more cells"), "{message}");
            }
            other => panic!("expected a syntax refusal, got {other:?}"),
        }
        let text = "Time (s),Alt (m)\n0,0\nend,1\n";
        assert!(matches!(read(text), Err(LogError::Syntax { line: 3, .. })));
        let text = "Time (s),Alt (m)\n0,0\n1,1e308\n";
        assert!(
            read(text).is_ok(),
            "a length that large passes the conversion"
        );
        let text = "Time (s),Alt (km)\n0,0\n1,1.7e308\n";
        assert!(matches!(read(text), Err(LogError::Syntax { line: 3, .. })));
    }

    #[test]
    fn what_isnt_this_layout_is_not_recognized() {
        for text in [
            "alt,t\n0,0\n",
            "time,altitude\n0,0\n",
            "Time (s)\n0\n",
            "1,2,3\n4,5,6\n",
            "Time (s),Alt (m)\nnot a number,0\n",
            "Time (s),Alt (m)\n",
            "Burnout time (ms),1500,Apogee meters,300\n",
        ] {
            assert!(!recognizes(text), "{text:?}");
            assert!(matches!(read(text), Err(LogError::NotThisFormat { .. })));
        }
    }

    #[test]
    fn a_mapping_reads_a_file_with_no_header() {
        let text = "0,100\n1,90\n2,80\n";
        let mapping = Mapping {
            header_line: None,
            separator: ',',
            decimal_comma: false,
            clock: 0,
            columns: vec![
                ColumnMapping {
                    name: "time".to_owned(),
                    quantity: None,
                    unit: Some("s".to_owned()),
                },
                ColumnMapping {
                    name: "pressure".to_owned(),
                    quantity: Some(Quantity::Pressure),
                    unit: Some("kPa".to_owned()),
                },
            ],
        };
        let log = read_with(text, &mapping).expect("reads");
        assert_eq!(
            channel(&log, "pressure").values,
            [100_000.0, 90_000.0, 80_000.0]
        );
        let mut wrong = mapping.clone();
        wrong.columns[1].unit = Some("ft".to_owned());
        match read_with(text, &wrong) {
            Err(LogError::Mapping { message }) => {
                assert!(message.contains("measures a length"), "{message}");
            }
            other => panic!("expected a mapping refusal, got {other:?}"),
        }
        let refusal = |wrong: &Mapping| match read_with(text, wrong) {
            Err(LogError::Mapping { message }) => message,
            other => panic!("expected a mapping refusal, got {other:?}"),
        };
        let mut wrong = mapping.clone();
        wrong.columns[0].unit = Some("m".to_owned());
        assert!(refusal(&wrong).contains("no unit of time"));
        let mut wrong = mapping.clone();
        wrong.clock = 2;
        assert!(refusal(&wrong).contains("the clock is column 2, of 2"));
        let mut wrong = mapping.clone();
        wrong.columns[1].unit = Some(format!("furlongs{}", " ".repeat(10_000)));
        let message = refusal(&wrong);
        assert!(message.contains("isn't one the reader knows") && message.len() < 200);
        let mut wrong = mapping.clone();
        wrong.separator = '.';
        assert!(refusal(&wrong).contains("isn't one of"));
        let mut wrong = mapping.clone();
        wrong.decimal_comma = true;
        assert!(refusal(&wrong).contains("a decimal comma needs"));
        // A headerless file may open with a `sep=` line, which isn't a row.
        let mut semicolons = mapping;
        semicolons.separator = ';';
        let log = read_with("sep=;\n0;100\n1;90\n", &semicolons).expect("reads");
        assert_eq!(channel(&log, "pressure").values, [100_000.0, 90_000.0]);
    }

    /// A thousands separator is never read as a decimal mark: the file's mark is settled once,
    /// and a cell in the other is a gap.
    #[test]
    fn a_thousands_separator_is_never_a_decimal_mark() {
        // Tabs, a point in the clock: `1,250` is no number.
        let text = "Time (s)\tAlt (ft)\n0.00\t0\n0.05\t1,250\n0.10\t980\n";
        let altitude = channel(&read(text).expect("reads"), "Alt (ft)")
            .values
            .clone();
        assert!(altitude[1].is_nan());
        assert_eq!(altitude[2], 980.0 * FOOT_M);
        // No point anywhere, and every comma before three digits: as likely thousands, so a gap.
        let text = "Time (ms);Alt (ft)\n0;0\n50;1,250\n100;2,500\n";
        let log = read(text).expect("reads");
        assert!(
            channel(&log, "Alt (ft)").values[1..]
                .iter()
                .all(|v| v.is_nan())
        );
        assert!(log.notes.iter().any(|n| n.contains("\"1,250\": gaps")));
        // A comma before other than three digits shows a decimal comma.
        let text = "Time (s);Alt (ft)\n0;0\n0,05;1250\n0,10;2,5\n";
        let mut by_header = mapping(text).expect("a header");
        assert!(by_header.decimal_comma);
        let altitude = channel(&read(text).expect("reads"), "Alt (ft)")
            .values
            .clone();
        assert_eq!(altitude[1..], [1250.0 * FOOT_M, 2.5 * FOOT_M]);
        // With a decimal comma, a point is no number: `1.250` is a gap, not 1.25.
        let text = "Time (s);Alt (ft)\n0;0\n0,05;1.250\n0,10;2,5\n";
        assert!(matches!(read(text), Err(LogError::Syntax { line: 3, .. })));
        by_header.header_line = Some(1);
        let altitude = channel(&read_with(text, &by_header).expect("reads"), "Alt (ft)")
            .values
            .clone();
        assert!(altitude[1].is_nan());
        assert_eq!(altitude[2], 2.5 * FOOT_M);
        // A comma-separated file reads points only; a quoted `"1,250"` is no number.
        let text = "Time (s),Alt (ft)\n0,0\n1,\"1,250\"\n";
        assert!(channel(&read(text).expect("reads"), "Alt (ft)").values[1].is_nan());
    }

    /// Every column's note is kept, however wide the table: the cap on notes about single lines
    /// doesn't apply to them.
    #[test]
    fn every_columns_note_is_kept() {
        let names: Vec<String> = (1..=30).map(|i| format!("c{i}")).collect();
        let text = format!(
            "Time (s),{},Pressure (Pa)\n0,{}101000\n1,{}150000\n",
            names.join(","),
            "1,".repeat(30),
            "2,".repeat(30)
        );
        let log = read(&text).expect("reads");
        for name in &names {
            assert!(
                log.notes
                    .iter()
                    .any(|n| n.contains(&format!("{name:?} states no unit"))),
                "{name}"
            );
        }
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("\"Pressure (Pa)\"'s values contradict"))
        );
        assert!(!log.notes.iter().any(|n| n.contains("more notes")));
    }

    /// Events from two columns come out in the file's order, by row.
    #[test]
    fn events_from_two_columns_are_in_the_files_order() {
        let text = "Time (s),State,Mode\n0,PAD,A\n1,BOOST,A\n2,BOOST,B\n3,BOOST,B\n";
        let log = read(text).expect("reads");
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [(0.0, "PAD"), (0.0, "A"), (1.0, "BOOST"), (2.0, "B")]
        );
        // A name that only starts with an event word holds no events: its text is left out.
        let text = "Time (s),Model\n0,EasyMini\n1,EasyMini\n";
        let log = read(text).expect("reads");
        assert!(log.events.is_empty());
        assert!(log.notes.iter().any(|n| n.contains("\"Model\" holds text")));
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has every channel as many
        /// times as values, and every value read in a unit finite, inside its quantity's bounds,
        /// or a gap.
        #[test]
        fn any_text_is_read_or_refused(
            text in "(sep=[;,]\n)?(Time \\(m?s\\)[,;\t](Alt|Temp|Press|v)[_ ]?\\(?(ft|m|C|kPa|m/s|g)?\\)?[,;\t]?[a-z\"]{0,4}\n)?\
                     ([-0-9.,;\t eE\"a-z]{0,24}\n){0,12}"
        ) {
            if let Ok(log) = read(&text) {
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                    if let Some((low, high)) = dimension_of(channel.quantity)
                        .and_then(|d| bounds_of(channel.quantity, d))
                    {
                        for value in &channel.values {
                            proptest::prop_assert!(
                                value.is_nan() || (low..=high).contains(value),
                                "{value} in {:?}", channel.quantity
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_mapping_round_trips_through_json() {
        let mapping = mapping(TABLE).expect("a header");
        let json = serde_json::to_string(&mapping).expect("writes");
        let back: Mapping = serde_json::from_str(&json).expect("reads");
        assert_eq!(back, mapping);
        let by_mapping = read_with(TABLE, &back).expect("reads");
        let by_header = read(TABLE).expect("reads");
        // Compared as JSON, where a gap is `null`: `NaN` isn't equal to itself.
        let json = |log: &FlightLog| serde_json::to_value(&log.channels).expect("writes");
        assert_eq!(json(&by_mapping), json(&by_header));
        assert_eq!(by_mapping.events, by_header.events);
    }
}
