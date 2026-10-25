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
//! or `mode`, each change of its word is an [`Event`] at the row's time, up to
//! [`MAX_EVENTS_PER_COLUMN`] a column, the rest counted in a note; otherwise it is left out, with
//! a note.
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
//! A mapping can state it ([`Mapping::primary`]): that column, an
//! [`Quantity::AltitudeAboveGround`], becomes the primary track on the clock, and stays a channel
//! too. Its clock must go forward, as the track's must; a mapping is refused if it doesn't, or if
//! the column holds no altitude it can read.
//!
//! **A column mapping.** [`mapping`] returns what the reader takes from the header, as a
//! [`Mapping`] a person or a program can change; [`read_with`] reads a file by a mapping, with the
//! same checks. A mapping can name the columns of a file with no header row, state a unit the
//! header leaves out, or keep a column as written; a unit it states must measure what its
//! quantity does, or the mapping is refused ([`LogError::Mapping`]).
//!
//! A mapping can also say what a header can't, about how the logger writes its rows:
//!
//! - **A clock that turns over** ([`Mapping::clock_wrap`]): the count a logger's counter runs over
//!   at, in the clock's unit, such as 2³² for a 32-bit count of microseconds. Each value is taken
//!   modulo it, as a logger may write it past the turn (one team's board prints its count 9 × 2³²
//!   high in the upper half of each turn, for a reason its files don't state), and each step
//!   back of more than half a turn is a turn: from there on the turn is added, so the clock runs
//!   on. A restart that steps back so far is read as a turn too. A note counts the values written
//!   past the turn and the turns.
//! - **Rows written as lists** ([`RowForm::List`]): `[1, 2.5, True]`, as a program printing its
//!   lists writes them. The brackets are dropped; a row without them is refused on its line.
//! - **Stamped rows** ([`RowForm::Stamped`]): a date and time, then `:` and the cells, as
//!   `2022-10-13 14:05:09.125: 1005.71` or `2022-10-13 14:05:09.125: (0.1, -0.2, 9.8)`. The stamp
//!   is column 0, the clock, in seconds from 1970-01-01 00:00 in the stamp's own zone, which such
//!   a file doesn't state: only the differences of its times are durations. The cells are split at
//!   the separator, a tuple's parentheses dropped. A line with no stamp is a mark, such as a
//!   payload's `=== MOTION DETECTED ===`: an event named by its words, without the `=`, `-` and
//!   `*` around them, at the next row's time, up to [`MAX_MARKS`], the rest counted in a note. A
//!   line shaped as a stamp, `dddd-dd-dd dd:dd:dd`, with no valid date and time in it is refused,
//!   a leap second's `23:59:60` among them. As an `f64`, a time since 1970 resolves to about
//!   0.25 µs, so a finer fraction is rounded.
//! - **Columns left out** ([`ColumnMapping::skip`]): cells beside a table that aren't part of it.
//!   They count in a row's width, but make no channel; a note says how many are left out.
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
//! A mapping is JSON a person can write. Every column has an entry in `columns`, a column left
//! out too; stamped rows need the stamp's entry first, the clock, in `s` and with no turn; and a
//! turn is in the clock's own unit, 4294967296 (2³²) for a 32-bit count of microseconds:
//!
//! ```
//! use hpr_flightdata::generic::{self, Mapping};
//!
//! // An invented payload's text: a stamp and a pressure in hectopascals, a note beside it, and a
//! // line marking motion, an event at the next row's time.
//! let text = "2024-06-01 14:05:09.0: 1005.5, ok\n\
//!             === MOTION DETECTED ===\n\
//!             2024-06-01 14:05:09.5: 1005.25, ok\n";
//! let mapping: Mapping = serde_json::from_str(
//!     r#"{"header_line": null, "separator": ",", "decimal_comma": false, "clock": 0,
//!         "rows": "stamped",
//!         "columns": [
//!             {"name": "stamp", "quantity": null, "unit": "s"},
//!             {"name": "pressure", "quantity": "pressure", "unit": "hPa"},
//!             {"name": "note", "quantity": null, "unit": null, "skip": true}]}"#,
//! )?;
//! let log = generic::read_with(text, &mapping)?;
//! assert_eq!(log.channels.len(), 1); // the note is left out
//! assert_eq!(log.channels[0].values, [100_550.0, 100_525.0]);
//! assert_eq!(log.channels[0].time_s[1] - log.channels[0].time_s[0], 0.5);
//! assert_eq!(log.events[0].name, "MOTION DETECTED");
//!
//! // An invented 32-bit count of microseconds, in rows written as lists, turning over once.
//! let text = "[4294967000, 10.0]\n[4294967200, 10.5]\n[104, 11.0]\n";
//! let mapping: Mapping = serde_json::from_str(
//!     r#"{"header_line": null, "separator": ",", "decimal_comma": false, "clock": 0,
//!         "clock_wrap": 4294967296, "rows": "list",
//!         "columns": [
//!             {"name": "clock", "quantity": null, "unit": "us"},
//!             {"name": "altitude", "quantity": "altitude_above_ground", "unit": "m"}]}"#,
//! )?;
//! let log = generic::read_with(text, &mapping)?;
//! let t = &log.channels[0].time_s;
//! assert!((t[2] - t[1] - 200e-6).abs() < 1e-9); // 104 µs is 200 µs past 4294967200 µs
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md

use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "table read by its header";

/// The most columns a header or a mapping may name: the widest table among the project's sample
/// logs names 99.
pub const MAX_COLUMNS: usize = 512;

/// The most events one column of words gives; past it, its changes are counted in a note, so a
/// hostile file whose word changes every row can't fill memory with events.
pub const MAX_EVENTS_PER_COLUMN: usize = 64;

/// The most marks a file of stamped rows gives as events ([`RowForm::Stamped`]); past it, they
/// are counted in a note.
pub const MAX_MARKS: usize = 64;

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
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
    /// The primary track's column, from 0: the barometric altitude above the pad, which becomes
    /// [`FlightLog::altitude_m`] on the clock, [`FlightLog::time_s`] (module docs, *The record*).
    /// Its quantity must be [`Quantity::AltitudeAboveGround`]. `None` for a table that has none,
    /// or doesn't say which; the header never says, so [`mapping`] returns `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<usize>,
    /// Every column, in the file's order.
    pub columns: Vec<ColumnMapping>,
    /// The count the clock turns over at, in its unit, for a logger's counter that runs over,
    /// such as 2³² microseconds; `None` for a clock that doesn't (module docs, *A column
    /// mapping*).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_wrap: Option<f64>,
    /// How each row is written: plain cells, unless the mapping says otherwise.
    #[serde(default, skip_serializing_if = "RowForm::is_cells")]
    pub rows: RowForm,
}

/// How a table's rows are written (module docs, *A column mapping*).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RowForm {
    /// Cells between separators.
    #[default]
    Cells,
    /// A list between `[` and `]`, as a program printing its lists writes it: `[1, 2.5, True]`.
    List,
    /// A date and time, `YYYY-MM-DD HH:MM:SS` with any fraction of a second, then `:` and the
    /// cells, between parentheses if they are a tuple, as `(x, y, z)`. The stamp is column 0, the
    /// clock, in seconds.
    Stamped,
}

impl RowForm {
    fn is_cells(&self) -> bool {
        *self == Self::Cells
    }
}

/// What one column holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
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
    /// Whether the mapping leaves the column out, as cells beside a table that aren't part of it:
    /// no channel is made of it, its cells only counted in the row's width.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub skip: bool,
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
                skip: false,
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
            skip: false,
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

/// A cell as a number in the file's decimal mark, `None` if it isn't one (module docs, *Rows*).
fn number(cell: &str, decimal_comma: bool) -> Option<f64> {
    table::number_in_mark(cell, decimal_comma)
}

/// Whether the file's numbers are written with a decimal comma: only where the separator isn't a
/// comma, no number below the header is written with a point, and one is written with a comma
/// followed by other than three digits, which no thousands separator is (module docs, *Rows*).
fn decimal_comma(text: &str, header_line: usize, separator: char) -> bool {
    if separator == ',' {
        return false;
    }
    table::decimal_comma_shown(
        numbered_lines(text)
            .filter(|(n, l)| *n > header_line && !skipped(l))
            .flat_map(|(_, line)| cells(line, separator, MAX_COLUMNS + 1)),
    )
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
            primary: None,
            columns,
            clock_wrap: None,
            rows: RowForm::Cells,
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

/// [`mapping`], with a note on each column the header leaves kept as written, saying why, as
/// [`read`] gives them.
///
/// # Errors
///
/// As [`mapping`].
pub fn header_reading(text: &str) -> Result<(Mapping, Vec<String>), LogError> {
    find_header(text).ok_or_else(not_this_format)
}

fn not_this_format() -> LogError {
    LogError::NotThisFormat {
        format: FORMAT,
        message: format!(
            "within its first {HEAD_LINES} lines there is no header naming a clock with a unit of \
             time, followed by a row with a number there. For example, `Time (s)` or \
             `time__us` names such a clock"
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
/// isn't one of its columns, isn't in a unit of time or is left out, whose turn isn't a count
/// above 0, whose stamped rows' clock isn't column 0 in `s` with no turn, or a column whose unit
/// the reader doesn't know or doesn't measure what its quantity does; [`LogError::Syntax`] for a
/// row of other than the mapping's width, a row of lists not between `[` and `]`, a stamp with no
/// valid date and time (a leap second's `23:59:60` among them), a clock cell that isn't a number,
/// or a reading its unit's conversion makes infinite, naming the line; [`LogError::Unit`] for a
/// clock that never steps forward or whose median step, in its unit, lies outside 0.1 ms to
/// 60 s; [`LogError::NoData`] if no row follows the header.
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
    if clock.skip {
        return Err(mapping_error(format!(
            "the clock, {}, is a column the mapping leaves out",
            quoted(&clock.name)
        )));
    }
    if let Some(turn) = mapping.clock_wrap
        && !(turn.is_finite() && turn > 0.0)
    {
        return Err(mapping_error(format!(
            "the clock's turn, {turn}, isn't a count above 0"
        )));
    }
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
    if mapping.rows == RowForm::Stamped
        && (mapping.clock != 0 || clock_unit.convert(1.0) != 1.0 || mapping.clock_wrap.is_some())
    {
        return Err(mapping_error(
            "stamped rows' clock is their stamp: column 0, in `s`, with no turn".to_owned(),
        ));
    }
    if let Some(primary) = mapping.primary {
        let Some(column) = mapping.columns.get(primary) else {
            return Err(mapping_error(format!(
                "the primary track is column {primary}, of {width}"
            )));
        };
        let refused = |why: &str| {
            mapping_error(format!(
                "the primary track, {}, {why}",
                quoted(&column.name)
            ))
        };
        if primary == mapping.clock {
            return Err(refused("is the clock's column"));
        }
        if column.skip {
            return Err(refused("is a column the mapping leaves out"));
        }
        if column.quantity != Some(Quantity::AltitudeAboveGround) {
            let mapped = column
                .quantity
                .and_then(|q| serde_json::to_value(q).ok())
                .and_then(|v| v.as_str().map(|name| format!("mapped to `{name}`")))
                .unwrap_or_else(|| "kept as written".to_owned());
            return Err(refused(&format!(
                "is {mapped}: it must be `altitude_above_ground`, the barometric altitude above \
                 the pad"
            )));
        }
    }
    let mut units = Vec::with_capacity(width);
    for (index, column) in mapping.columns.iter().enumerate() {
        if index == mapping.clock || column.skip {
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
    /// and the word, the first [`MAX_EVENTS_PER_COLUMN`] of them.
    changes: Vec<Vec<(usize, String)>>,
    /// For each such column, its last word, and how many changes past the kept ones it makes.
    last_word: Vec<(Option<String>, usize)>,
    /// Stamped rows' marks: the index of the row each comes before and its words, the first
    /// [`MAX_MARKS`] of them; how many past those; and how many follow the last row.
    marks: Vec<(usize, String)>,
    marks_left_out: usize,
    marks_after_last: usize,
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
        last_word: vec![(None, 0); width],
        marks: Vec::new(),
        marks_left_out: 0,
        marks_after_last: 0,
    };
    // The marks since the last row, kept up to the cap and the rest counted.
    let mut pending_marks = Vec::new();
    let mut pending_past_cap = 0;
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
        let (stamp, body) = match mapping.rows {
            RowForm::Cells => (None, line),
            RowForm::List => {
                let Some(body) = line
                    .trim()
                    .strip_prefix('[')
                    .and_then(|b| b.strip_suffix(']'))
                else {
                    return Err(syntax(
                        line_number,
                        "the row isn't a list between `[` and `]`".to_owned(),
                    ));
                };
                (None, body)
            }
            RowForm::Stamped => match stamped(line) {
                Stamp::Row(seconds, body) => (Some(seconds), body),
                Stamp::None => {
                    if rows.marks.len() + pending_marks.len() < MAX_MARKS {
                        pending_marks.push(mark(line));
                    } else {
                        pending_past_cap += 1;
                    }
                    continue;
                }
                Stamp::Invalid => {
                    return Err(syntax(
                        line_number,
                        "the row's stamp isn't a date and time, `YYYY-MM-DD HH:MM:SS`, followed \
                         by `:`"
                            .to_owned(),
                    ));
                }
            },
        };
        let mut cells = cells(body, mapping.separator, width + 1);
        if stamp.is_some() {
            cells.insert(0, Cow::Borrowed(""));
        }
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
        let Some(time) = stamp.or_else(|| number(clock_cell, mapping.decimal_comma)) else {
            return Err(syntax(
                line_number,
                format!("the clock's cell {} isn't a number", quoted(clock_cell)),
            ));
        };
        let row = rows.clock.len();
        rows.marks
            .extend(pending_marks.drain(..).map(|words| (row, words)));
        rows.marks_left_out += std::mem::take(&mut pending_past_cap);
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
                    let (last, left_out) = &mut rows.last_word[index];
                    if last.as_ref() != Some(&word) {
                        let changes = &mut rows.changes[index];
                        if changes.len() < MAX_EVENTS_PER_COLUMN {
                            changes.push((row, word.clone()));
                        } else {
                            *left_out += 1;
                        }
                        *last = Some(word);
                    }
                }
            }
            rows.values[index].push(value.unwrap_or(f64::NAN));
        }
        rows.lines.push(line_number);
        rows.clock.push(time);
    }
    rows.marks_after_last = pending_marks.len() + pending_past_cap;
    Ok(rows)
}

/// What a line of stamped rows opens with (module docs, *A column mapping*).
enum Stamp<'a> {
    /// A stamp: its clock, in seconds from 1970-01-01 00:00 in the stamp's own zone, and the
    /// rest of the line after its `:`, the parentheses dropped if it is a tuple.
    Row(f64, &'a str),
    /// The shape of a stamp, `dddd-dd-dd dd:dd:dd`, with no valid date and time in it, or no `:`
    /// after it.
    Invalid,
    /// No stamp: a mark.
    None,
}

fn stamped(line: &str) -> Stamp<'_> {
    let line = line.trim_start();
    let bytes = line.as_bytes();
    let shape = b"dddd-dd-dd dd:dd:dd";
    let shaped = bytes.len() >= shape.len()
        && shape.iter().zip(bytes).all(|(want, got)| {
            if *want == b'd' {
                got.is_ascii_digit()
            } else {
                want == got
            }
        });
    if !shaped {
        return Stamp::None;
    }
    // Every byte of the shape is ASCII, so these slices are on character boundaries.
    let field = |from: usize, count: usize| -> i64 {
        line[from..from + count]
            .bytes()
            .fold(0, |n, b| n * 10 + i64::from(b - b'0'))
    };
    let (year, month, day) = (field(0, 4), field(5, 2), field(8, 2));
    let (hour, minute, second) = (field(11, 2), field(14, 2), field(17, 2));
    if !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return Stamp::Invalid;
    }
    let mut end = shape.len();
    let mut fraction = 0.0;
    if bytes.get(end) == Some(&b'.') {
        let count = bytes[end + 1..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        if count == 0 || count > 9 {
            return Stamp::Invalid;
        }
        fraction = field(end + 1, count) as f64 / 10f64.powi(count as i32);
        end += 1 + count;
    }
    let Some(rest) = line[end..].strip_prefix(':') else {
        return Stamp::Invalid;
    };
    let rest = rest.trim();
    let rest = rest
        .strip_prefix('(')
        .and_then(|r| r.strip_suffix(')'))
        .map_or(rest, str::trim);
    let whole = days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second;
    Stamp::Row(whole as f64 + fraction, rest)
}

/// A mark's words: the line trimmed of spaces and of the `=`, `-` and `*` runs around it.
fn mark(line: &str) -> String {
    short(
        line.trim()
            .trim_matches(|c: char| c == '=' || c == '-' || c == '*' || c.is_whitespace()),
    )
}

/// Days in a month of the proleptic Gregorian calendar.
fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar (H. Hinnant,
/// *chrono-Compatible Low-Level Date Algorithms*, `days_from_civil`, public domain).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let shifted_month = (month + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// A count and its noun, `1 line` or `2 lines`.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// A clock that counts modulo `turn`, unwrapped in place: each value taken modulo it, and `turn`
/// added for each step back of more than half of it. How many values were written past the turn,
/// and how many turns there were.
fn unwrap_clock(clock: &mut [f64], turn: f64) -> (usize, usize) {
    let (mut past, mut turns) = (0, 0);
    let mut offset = 0.0;
    let mut previous: Option<f64> = None;
    for value in clock.iter_mut() {
        let counted = value.rem_euclid(turn);
        if counted != *value {
            past += 1;
        }
        if previous.is_some_and(|p| p - counted > turn / 2.0) {
            offset += turn;
            turns += 1;
        }
        previous = Some(counted);
        *value = counted + offset;
    }
    (past, turns)
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
    let mut rows = read_rows(text, mapping)?;
    if rows.clock.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    let turned = mapping
        .clock_wrap
        .map(|turn| (turn, unwrap_clock(&mut rows.clock, turn)));
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
    if let Some((turn, (past, turns))) = turned {
        notes.push(format!(
            "the clock, {}, counts modulo {turn}: {} written past it taken modulo it, and it \
             turns over {}, each turn added",
            quoted(clock_name),
            counted(past, "value", "values"),
            match turns {
                1 => "once".to_owned(),
                n => format!("{n} times"),
            }
        ));
    }
    let skipped_columns = mapping.columns.iter().filter(|c| c.skip).count();
    if skipped_columns > 0 {
        notes.push(format!(
            "the mapping leaves out {skipped_columns} of the {} columns",
            mapping.columns.len()
        ));
    }
    if !rows.marks.is_empty() || rows.marks_left_out > 0 {
        notes.push(format!(
            "{} with no stamp: each a mark, an event at the next row's time{}",
            counted(rows.marks.len() + rows.marks_left_out, "line", "lines"),
            if rows.marks_left_out > 0 {
                format!(", the first {MAX_MARKS} kept")
            } else {
                String::new()
            }
        ));
    }
    if rows.marks_after_last > 0 {
        notes.push(format!(
            "{} with no stamp after the last row: left out",
            counted(rows.marks_after_last, "line", "lines")
        ));
    }
    let back = rows.clock.windows(2).filter(|p| p[1] < p[0]).count();
    if back > 0 {
        notes.push(format!(
            "the clock, {}, steps back {back} times; the rows are kept in the file's order",
            quoted(clock_name)
        ));
    }
    let mut channels = Vec::new();
    let mut primary_channel = None;
    let mut changes: Vec<(usize, &str)> = Vec::new();
    for (index, column) in mapping.columns.iter().enumerate() {
        if index == mapping.clock || column.skip {
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
                let left_out = rows.last_word[index].1;
                if left_out > 0 {
                    notes.push(format!(
                        "the column {} changes its word {left_out} more times than the \
                         {MAX_EVENTS_PER_COLUMN} kept as events",
                        quoted(&column.name)
                    ));
                }
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
        if mapping.primary == Some(index) {
            primary_channel = Some(channels.len());
        }
        channels.push(Channel {
            column: column.name.clone(),
            quantity,
            time_s: time_s.clone(),
            values,
        });
    }
    let (track_time_s, altitude_m) = primary_track(
        mapping,
        primary_channel.and_then(|i| channels.get(i)),
        &time_s,
        &rows.lines,
    )?;
    changes.extend(rows.marks.iter().map(|(r, w)| (*r, w.as_str())));
    // In the file's order: by row, and a row's events in its columns' order, its marks after
    // them (a stable sort).
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
        time_s: track_time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events,
        notes,
    })
}

/// The primary track a mapping marks ([`Mapping::primary`]), its times and altitudes; both empty
/// for a mapping that marks none. `channel` is the column's channel, `None` if it made none;
/// `time_s` the clock, each row's from `lines`.
///
/// Refused if the column made no channel, as one that holds no number makes none, if its values
/// contradict its unit, so that it is kept as written, or if the clock doesn't go forward, as the
/// track's times must.
fn primary_track(
    mapping: &Mapping,
    channel: Option<&Channel>,
    time_s: &[f64],
    lines: &[usize],
) -> Result<(Vec<f64>, Vec<f64>), LogError> {
    let Some(primary) = mapping.primary else {
        return Ok((Vec::new(), Vec::new()));
    };
    let column = mapping.columns.get(primary);
    let name = quoted(column.map_or("", |c| c.name.as_str()));
    let channel = match channel {
        None => {
            return Err(mapping_error(format!(
                "the primary track, {name}, holds no number"
            )));
        }
        Some(channel) if channel.quantity != Quantity::AltitudeAboveGround => {
            return Err(mapping_error(format!(
                "the primary track, {name}, holds values that contradict its unit, {}",
                quoted(column.and_then(|c| c.unit.as_deref()).unwrap_or_default())
            )));
        }
        Some(channel) => channel,
    };
    if let Some(step) = time_s.windows(2).position(|pair| pair[1] <= pair[0]) {
        let clock = mapping
            .columns
            .get(mapping.clock)
            .map_or("", |c| c.name.as_str());
        return Err(mapping_error(format!(
            "the primary track, {name}, needs a clock that goes forward, and the clock, {}, \
             doesn't on line {}: {} s, then {} s",
            quoted(clock),
            lines.get(step + 1).copied().unwrap_or_default(),
            time_s[step],
            time_s[step + 1]
        )));
    }
    Ok((channel.time_s.clone(), channel.values.clone()))
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
                    skip: false,
                },
                ColumnMapping {
                    name: "pressure".to_owned(),
                    quantity: Some(Quantity::Pressure),
                    unit: Some("kPa".to_owned()),
                    skip: false,
                },
            ],
            primary: None,
            clock_wrap: None,
            rows: RowForm::Cells,
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
        // Past the cap, changes are counted, not kept.
        let rows: String = (0..200).map(|i| format!("{i},{}\n", i % 2)).collect();
        let log = read(&format!(
            "Time (s),Mode\n{}",
            rows.replace(",0\n", ",A\n").replace(",1\n", ",B\n")
        ))
        .expect("reads");
        assert_eq!(log.events.len(), MAX_EVENTS_PER_COLUMN);
        assert!(log.notes.iter().any(|n| n.contains("136 more times")));
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

    fn column(name: &str, quantity: Option<Quantity>, unit: Option<&str>) -> ColumnMapping {
        ColumnMapping {
            name: name.to_owned(),
            quantity,
            unit: unit.map(str::to_owned),
            skip: false,
        }
    }

    fn headerless(columns: Vec<ColumnMapping>) -> Mapping {
        Mapping {
            header_line: None,
            separator: ',',
            decimal_comma: false,
            clock: 0,
            columns,
            primary: None,
            clock_wrap: None,
            rows: RowForm::Cells,
        }
    }

    fn mapping_refusal(text: &str, mapping: &Mapping) -> String {
        match read_with(text, mapping) {
            Err(LogError::Mapping { message }) => message,
            other => panic!("expected a mapping refusal, got {other:?}"),
        }
    }

    /// A 32-bit count of microseconds, printed past its turn where it is past half of it (as
    /// 9 × 2³² more, as one team's board prints it), and turning over once: one clock that runs on.
    #[test]
    fn a_clock_that_turns_over_runs_on() {
        let turn = 4_294_967_296.0_f64;
        let counts = [
            turn - 40_000.0,
            turn - 20_000.0,
            0.0,
            20_000.0,
            turn / 2.0 + 10_000.0,
        ];
        let written = [
            counts[0] + 9.0 * turn,
            counts[1] + 9.0 * turn,
            counts[2],
            counts[3],
            counts[4] + 9.0 * turn,
        ];
        let text: String = written
            .iter()
            .enumerate()
            .map(|(k, w)| format!("{w},{}\n", 90_000 + k))
            .collect();
        let mut mapping = headerless(vec![
            column("clock", None, Some("us")),
            column("pressure", Some(Quantity::Pressure), Some("Pa")),
        ]);
        mapping.clock_wrap = Some(turn);
        let log = read_with(&text, &mapping).expect("reads");
        let times = &channel(&log, "pressure").time_s;
        let expected = [
            (turn - 40_000.0) / 1e6,
            (turn - 20_000.0) / 1e6,
            turn / 1e6,
            (turn + 20_000.0) / 1e6,
            (turn + turn / 2.0 + 10_000.0) / 1e6,
        ];
        assert_eq!(times.as_slice(), expected);
        assert!(log.notes.iter().any(|n| n.contains(
            "counts modulo 4294967296: 3 values written past it taken modulo it, and it turns over \
             once"
        )));
        // Without the turn the same file reads a clock that jumps back by days.
        mapping.clock_wrap = None;
        let log = read_with(&text, &mapping).expect("reads");
        assert!(log.notes.iter().any(|n| n.contains("steps back 1 times")));
        for bad in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            mapping.clock_wrap = Some(bad);
            assert!(mapping_refusal(&text, &mapping).contains("isn't a count above 0"));
        }
    }

    #[test]
    fn rows_written_as_lists_drop_their_brackets() {
        let mut mapping = headerless(vec![
            column("clock", None, Some("ms")),
            column("pressure", Some(Quantity::Pressure), Some("Pa")),
            column("fix", None, None),
        ]);
        mapping.rows = RowForm::List;
        let text = "[0, 101325, True]\n[500, 101300, True]\n";
        let log = read_with(text, &mapping).expect("reads");
        assert_eq!(channel(&log, "pressure").values, [101_325.0, 101_300.0]);
        assert_eq!(channel(&log, "pressure").time_s, [0.0, 0.5]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("\"fix\" holds text, not numbers, such as \"True\""))
        );
        for row in ["1000, 101200, True]", "[1000, 101200, True"] {
            match read_with(&format!("{text}{row}\n"), &mapping) {
                Err(LogError::Syntax { line, message, .. }) => {
                    assert_eq!(line, 3);
                    assert!(message.contains("isn't a list"), "{message}");
                }
                other => panic!("expected a refusal, got {other:?}"),
            }
        }
    }

    #[test]
    fn stamped_rows_take_their_stamp_as_the_clock() {
        let mut mapping = headerless(vec![
            column("stamp", None, Some("s")),
            column("x", Some(Quantity::Acceleration), Some("m/s^2")),
            column("y", Some(Quantity::Acceleration), Some("m/s^2")),
            column("z", Some(Quantity::Acceleration), Some("m/s^2")),
        ]);
        mapping.rows = RowForm::Stamped;
        let text = "2022-10-13 23:59:59.875: (0.5, -0.25, 9.75)\n\
                    ===== MOTION DETECTED =====\n\
                    2022-10-14 00:00:00.125000: (1, 2, 30)\n\
                    2022-10-14 00:00:01: (0, 0, 9.8)\n\
                    -- end --\n";
        let log = read_with(text, &mapping).expect("reads");
        // 2022-10-13 is day 19,278 from 1970-01-01.
        let midnight = 19_279.0 * 86_400.0;
        assert_eq!(
            channel(&log, "z").time_s,
            [midnight - 0.125, midnight + 0.125, midnight + 1.0]
        );
        assert_eq!(channel(&log, "x").values, [0.5, 1.0, 0.0]);
        assert_eq!(channel(&log, "z").values, [9.75, 30.0, 9.8]);
        assert_eq!(log.events.len(), 1);
        assert_eq!(log.events[0].name, "MOTION DETECTED");
        assert_eq!(log.events[0].time_s, midnight + 0.125);
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("1 line with no stamp: each a mark"))
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("1 line with no stamp after the last row"))
        );
        // A stamp's shape with no date or time in it, or no `:` after it, is refused on its line.
        for bad in [
            "2022-02-29 00:00:02: (0, 0, 9.8)",
            "2022-10-14 24:00:02: (0, 0, 9.8)",
            "2022-10-14 00:00:02. (0, 0, 9.8)",
            "2022-10-14 00:00:02 (0, 0, 9.8)",
            "2022-10-14 00:00:02.1234567890: (0, 0, 9.8)",
        ] {
            match read_with(&format!("{text}{bad}\n"), &mapping) {
                Err(LogError::Syntax { line, message, .. }) => {
                    assert_eq!(line, 6, "{bad}");
                    assert!(message.contains("stamp isn't a date"), "{message}");
                }
                other => panic!("{bad}: expected a refusal, got {other:?}"),
            }
        }
        // A leap day is a date; a stamp with a single value reads as one cell.
        let mut one = headerless(vec![
            column("stamp", None, Some("s")),
            column("pressure", Some(Quantity::Pressure), Some("hPa")),
        ]);
        one.rows = RowForm::Stamped;
        let log = read_with(
            "2024-02-29 12:00:00: 1005.5\n2024-02-29 12:00:00.5: 1005.25\n",
            &one,
        )
        .expect("reads");
        assert_eq!(channel(&log, "pressure").values, [100_550.0, 100_525.0]);
        // Past the cap, marks are counted, not kept, before a row and after the last.
        let many = format!(
            "{}2024-02-29 12:00:00: 1005.5\n{}2024-02-29 12:00:01: 1005.25\nend\nend\n",
            "mark\n".repeat(MAX_MARKS - 2),
            "mark\n".repeat(5)
        );
        let log = read_with(&many, &one).expect("reads");
        assert_eq!(log.events.len(), MAX_MARKS);
        assert!(log.notes.iter().any(|n| n.contains(&format!(
            "{} lines with no stamp: each a mark, an event at the next row's time, the first \
             {MAX_MARKS} kept",
            MAX_MARKS + 3
        ))));
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("2 lines with no stamp after the last row"))
        );
        let mut wrong = one.clone();
        wrong.columns[0].unit = Some("ms".to_owned());
        assert!(mapping_refusal("", &wrong).contains("column 0, in `s`"));
        let mut wrong = one;
        wrong.clock_wrap = Some(60.0);
        assert!(mapping_refusal("", &wrong).contains("with no turn"));
    }

    #[test]
    fn columns_left_out_make_no_channel() {
        let mut mapping = headerless(vec![
            column("time", None, Some("s")),
            column("altitude", Some(Quantity::AltitudeAboveGround), Some("ft")),
            column("beside 1", None, None),
            column("beside 2", Some(Quantity::Pressure), Some("ft")),
        ]);
        mapping.columns[2].skip = true;
        mapping.columns[3].skip = true;
        let text = "0,0,Name,12\n0.5,100,,\n1,180,,\n";
        let log = read_with(text, &mapping).expect("reads");
        assert_eq!(log.channels.len(), 1);
        assert_eq!(log.channels[0].column, "altitude");
        assert!(
            log.notes
                .iter()
                .any(|n| n == "the mapping leaves out 2 of the 4 columns")
        );
        assert!(!log.notes.iter().any(|n| n.contains("Name")));
        // A row still needs every cell, and the clock can't be left out.
        assert!(matches!(
            read_with("0,0,Name\n", &mapping),
            Err(LogError::Syntax { line: 1, .. })
        ));
        // A column left out isn't held to its unit: kept, a pressure in feet is refused.
        mapping.columns[3].skip = false;
        assert!(mapping_refusal(text, &mapping).contains("\"beside 2\""));
        mapping.columns[3].skip = true;
        mapping.columns[0].skip = true;
        assert!(mapping_refusal(text, &mapping).contains("the mapping leaves out"));
    }

    /// The new options default when a mapping's JSON leaves them out, and a mapping that doesn't
    /// use them writes the JSON it wrote before; set, they round-trip.
    #[test]
    fn a_mappings_options_default_and_round_trip() {
        let old = r#"{"header_line":null,"separator":",","decimal_comma":false,"clock":0,"columns":[{"name":"t","quantity":null,"unit":"s"}]}"#;
        let mapping: Mapping = serde_json::from_str(old).expect("parses");
        assert_eq!(mapping.clock_wrap, None);
        assert_eq!(mapping.primary, None);
        assert_eq!(mapping.rows, RowForm::Cells);
        assert!(!mapping.columns[0].skip);
        assert_eq!(serde_json::to_string(&mapping).expect("writes"), old);
        let new = r#"{"header_line":null,"separator":",","decimal_comma":false,"clock":0,"primary":1,"columns":[{"name":"t","quantity":null,"unit":"s"},{"name":"x","quantity":null,"unit":null,"skip":true}],"clock_wrap":4294967296.0,"rows":"stamped"}"#;
        let mapping: Mapping = serde_json::from_str(new).expect("parses");
        assert_eq!(mapping.clock_wrap, Some(4_294_967_296.0));
        assert_eq!(mapping.primary, Some(1));
        assert_eq!(mapping.rows, RowForm::Stamped);
        assert!(mapping.columns[1].skip);
        assert_eq!(serde_json::to_string(&mapping).expect("writes"), new);
        let list: Mapping = serde_json::from_str(&new.replace("stamped", "list")).expect("parses");
        assert_eq!(list.rows, RowForm::List);
    }
}
