//! Silicdyne's Fluctus flight computer's logs, as its Fluctus Control Center (FCC) exports a
//! flight: a `sep=;` line, then a semicolon-separated table with each column's unit in its name.
//!
//! The Fluctus records its black box at each tick of its flight software, 50 times a second, from
//! a second before the launch it detects (*Fluctus – Full Documentation*, v1.7b, Silicdyne, 2025,
//! <https://silicdyne.net/resources/docs/fluctus_fulldoc_v1_7b_EN.pdf>, read 2026-10-13, p26); the
//! FCC's BlackBox Viewer exports it to a `.csv` file "which can be opened in any spreadsheet
//! program, such as Excel" (p58). The first line is `sep=;`, the hint that tells a spreadsheet the
//! separator; the next names the columns, a unit in brackets after most; each row below is one
//! tick:
//!
//! ```text
//! sep=;
//! time (ms);deltaTime (ms);status;baro-altitude (m);dedrck-v-speed (m/s);angle (deg);…;gpsSats
//! 61500;20;3;0;0;3,5;0;0,0;9,8;0;0,0;21;8400;1;1;0;0;0;0;12,345678;-23,456789;150;1;8
//! ```
//!
//! **Recognition.** Within the file's first 16 lines, the first that isn't blank is either `sep=`
//! and one separator character (`;`, `,`, a tab or `|`), followed by the header on the next line
//! that isn't blank, or the header itself, then separated by semicolons. The header's first name
//! is `time`, and it names `deltaTime` and `baro-altitude` too: those are the Fluctus's own names
//! (manual p27), and no other layout among the project's sample logs has them. Names are compared
//! with their bracketed unit taken off and without regard to case. An Eggtimer's or an RRC3's
//! semicolon-separated file has neither name.
//!
//! **Numbers.** The FCC's export has a *Decimal point delimiter* setting, as spreadsheets in
//! different regions read a `.csv` file differently (p58): the sample logs write `12,5` for 12.5. When the separator isn't a comma, a comma in a cell is read as the decimal mark, and a
//! point as well; a cell with both, or with two commas, isn't a number (`table::number`). The
//! files bear this out: their latitudes are written `12,345678`, six digits after the comma,
//! which as a thousands separator would put the logger millions of degrees from the equator, and
//! their clock, past 1,000 ms on every row, is written with no thousands separator, so `1,234` is
//! 1.234, never 1,234. When `sep=,` names the comma, numbers take a point only.
//!
//! **Columns.** A name is split into the name and the unit in the brackets at its end, as
//! `baro-altitude (m)`. The name says what the column holds and the unit how to read it; the
//! units are those of the header, read without regard to case or spaces:
//!
//! | Column | Read as | From |
//! |---|---|---|
//! | `time` | the clock, the logger's own: its zero isn't stated, and the rows start about a second before the launch | the header (`ms`); manual p27 ("Frame time") |
//! | `deltaTime` | not a channel: checked against the clock's step, and a note counts the rows where it differs | the header (`ms`); manual p27 ("Time since last tick") |
//! | `status` | [`Quantity::Number`], and an event at each change, named as the manual names the code (below) | manual p23 |
//! | `baro-altitude` | [`Quantity::AltitudeAboveGround`]: the primary track | the header (`m`); manual p27 ("Baro altitude from origin") |
//! | `dedrck-v-speed` | [`Quantity::Speed`], up positive: the vertical speed by dead reckoning, the accelerometers' and gyroscope's integral | the header (`m/s`); manual p23, p27 |
//! | `angle` | [`Quantity::Angle`]: the tilt from vertical on the rail and in the descent, from the launch axis in the climb | the header (`deg`); manual p24 |
//! | `roll-rate` | [`Quantity::AngularRate`] | the header (`deg/s`); manual p27 |
//! | `vert-accel` | [`Quantity::Acceleration`], vertical | the header (`m/s2`); manual p27 |
//! | `accel` | [`Quantity::Acceleration`]: the magnitude of the acceleration the accelerometers feel, 1 g at rest | the header (`m/s2`); manual p24 |
//! | `dedrck-alti` | [`Quantity::InertialAltitude`] | the header (`m`); manual p27 |
//! | `baro-speed` | [`Quantity::Speed`], up positive: the primary track's vertical speed | the header (`m/s`); manual p23 ("barometric altitude derivative") |
//! | `amb-temp` | [`Quantity::Temperature`]: the board's sensor, which reads the bay | the header (`deg c`); manual p27 |
//! | `batt-voltage` | [`Quantity::Voltage`]: the battery's | the header (`mV`); manual p27 |
//! | `P1-state`, `P2-state`, `P3-state` | [`Quantity::Number`], and an event when an output fires (below) | manual p23 |
//! | `analog1`, `analog2` | [`Quantity::Voltage`] | the header (`mV`); manual p27 |
//! | `inFreefall` | [`Quantity::Number`], 1 or 0 | manual p27 ("Rocket in free fall (bool)") |
//! | `gpsLat`, `gpsLng` | [`Quantity::Latitude`], [`Quantity::Longitude`], when the values bear out degrees (below) | manual p38; a note |
//! | `gpsAltMSL` | [`Quantity::GnssAltitude`] in meters, when the barometric altitude bears it out (below); otherwise kept as written | manual p27 ("GPS altitude (MSL)"); a note |
//! | `gpsState` | [`Quantity::Number`] as written: the manual gives no codes; Silicdyne's changelog says that from firmware 2.2 it holds the satellite count, the fix status before it moved elsewhere (<https://silicdyne.net/changelog-fluctus/>, update 2.2, read 2026-10-13) | |
//! | `gpsSats` | [`Quantity::Number`]: the satellites the GPS uses | manual p27 |
//!
//! The units the header may state: `ms`, `s` and `us` for a time; `m` and `ft`; `m/s`, `ft/s`
//! and `km/h`; `m/s2` (or `m/s²`, `m/s^2`) and `ft/s2`; `deg` (or `°`) and `rad`; `deg/s` and
//! `rad/s`; `deg c` and `deg f` (or `°c`, `°f`) and `k`; `mV` and `V`; `Pa`, `hPa`, `mbar` and
//! `kPa`. A column the reader knows whose unit is another, or is of the wrong kind, such as an
//! angle in `m/s`, is kept as written, [`Quantity::Unrecognized`], with a note; so is one the
//! manual gives a unit and the header writes none for. A column the reader doesn't know is read
//! by its unit where the unit can mean one quantity only, a speed, an acceleration, an angle, a
//! rate of turn, a temperature, a voltage or a pressure; a length, a time, no unit or one the
//! reader doesn't know keeps it as written, with a note. The clock must be in a unit of time and
//! the barometric altitude in one of length: a file without either is refused.
//!
//! *The altitude's zero.* The manual gives `baro-altitude` "from origin" (p27) and doesn't say
//! where that is; as of 2026-10-13, in all 6 Fluctus files among the project's sample logs, it
//! reads 0 on the first rows, on the pad before launch, so the logger zeroes it on the pad. It is
//! written in whole meters there, though the manual gives the barometer a resolution of 0.1 m
//! (p18), so the format's resolution is 1 m
//! ([`LogFormat::altitude_resolution_m`](crate::log::LogFormat::altitude_resolution_m)).
//!
//! *The position in degrees.* No unit is in the header for `gpsLat` and `gpsLng`; the manual's
//! picture of the FCC's *Locate* tab shows the position in decimal degrees to six places (p38), as
//! the file writes it. Each is read in degrees if it holds at least one reading and every reading
//! lies within ±90° for the latitude or ±180° for the longitude; otherwise it is kept as written,
//! with a note.
//!
//! *The GPS altitude in meters.* No unit is in the header for `gpsAltMSL`, nor in the manual (p27),
//! and Silicdyne's changelog says only that the firmware logs it from update 1.6. It is read in
//! meters only if the barometric altitude bears it out, at the GPS's fresh fixes: the rows where
//! the GPS altitude differs from its last reading before, the first included. A reading that
//! repeats the last, as the receiver holds its last fix while it has none, and a gap are left out.
//! Through those fixes a straight line is drawn by least squares, the GPS altitude against the
//! barometric; in meters it rises 1 for each meter, in feet 3.28. The GPS altitude is read in
//! meters where all four hold:
//!
//! - there are at least 10 fresh fixes, more than the two that fix a line exactly, so that the
//!   scatter about it is measured too;
//! - they span at least 120 m of the barometric altitude. The manual's ±3 m for the GPS (p18) is
//!   its horizontal position; a GPS receiver's vertical error is larger, the U.S. government's
//!   standard for GPS giving 13 m against 8 m horizontal, 95% of the time, 1.6 times (*GPS
//!   Standard Positioning Service Performance Standard*, 5th ed., 2020, Table 3.8-3, p58). Allowing
//!   twice the horizontal, 6 m, two readings that far off in opposite directions at the span's ends
//!   tilt the slope by 12 m over the span: over 120 m, the 10% margin below;
//! - they scatter about the line, RMS, by at most 10% of that span. A receiver that lags far
//!   behind the flight traces a loop rather than a line, and the slope of a loop doesn't tell the
//!   unit: a feet GPS lagging by a 10 s filter can come out near 1 (a test pins one at 3 times the
//!   allowed scatter);
//! - the slope lies within 10% of 1, far from the 3.28 of feet.
//!
//! Otherwise it is kept as written, [`Quantity::Unrecognized`], with a note saying which failed.
//! A GPS that loses its fix in the climb and regains it low in the descent, the fixes spanning
//! little of the flight, is kept: its highest reading's rise above the pad can match the apogee in
//! feet by chance, but the span or the slope gives it away. As of 2026-10-13, none of the 6 sample
//! files passes: in each, the GPS altitude lags the barometric one by hundreds of meters in the
//! descent, and every slope is below 0.75.
//!
//! **Events.** Each change of `status` is an event named as the manual names the code it changes
//! to (p23): 0 `IDLE`, 1 `ARMED`, 2 `COUNTDOWN ENGAGED`, 3 `WAITING FOR LAUNCH`, 4 `ASCENT` (the
//! launch detected), 5 `DESCENT` (the apogee detected), 6 `TOUCHDOWN`; another code is named
//! `status` and its number. A pyro output's state is 0 for no continuity, 1 for continuity and 10
//! for fired (p23): each change to 10 is an event, the column's name and `ENABLED`, as `P1-state
//! ENABLED`. A row whose cell is a gap changes nothing.
//!
//! **Placeholders.** A cell of 2,147,483,647, the largest signed 32-bit number, or of −2,147,483,648,
//! the smallest, isn't a reading: as of 2026-10-13, in one of the 6 sample files, both of the
//! dead-reckoning columns hold 2,147,483,647 from a few rows after the launch to the end, where
//! their readings would run to many kilometers a second. Each such cell is a gap, counted in a note.
//!
//! **Rows.** Every row must have as many cells as the header has names; a row of another width, a
//! clock cell that isn't a number, or a time that doesn't come after the row above's is refused,
//! naming its line. Any other cell that is empty or isn't a number, such as `nan`, is a gap,
//! counted per column in a note. A reading its unit's conversion carries past the largest number
//! an `f64` holds, as `1e308` in `deg f` or `1.7e308` in `g`, is refused, naming its line
//! ([ADR-258][adr-258] §3). Blank lines are skipped; lines may end in CR, LF or both, and a
//! byte-order mark is dropped. A note counts the steps of the clock longer than its commonest. A
//! header of more than [`MAX_COLUMNS`] names, or naming a column twice, is refused.
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md

use std::collections::{HashMap, HashSet};

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, Notes, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "Silicdyne Fluctus CSV";

/// The step between two altitudes the format can tell apart near the ground, m
/// ([`crate::log::LogFormat::altitude_resolution_m`]): `baro-altitude` is written in whole meters
/// in every one of the 6 Fluctus files among the project's sample logs as of 2026-10-13, where it
/// steps by 1 m near the pad, though the manual gives the barometer a resolution of 0.1 m (p18).
pub(crate) const ALTITUDE_STEP_M: f64 = 1.0;

/// The most names a header may have: the Fluctus writes 24 (manual p27).
pub const MAX_COLUMNS: usize = 256;

/// How many lines from the file's start the `sep=` line and the header may lie within.
const HEAD_LINES: usize = 16;

/// How far the slope of the GPS altitude against the barometric altitude, at the GPS's fresh
/// fixes, may lie from 1 for the GPS altitude to be read in meters; in feet it would be 3.28
/// (module docs, *The GPS altitude in meters*).
const GPS_SLOPE_MARGIN: f64 = 0.1;

/// The least span of the barometric altitude, m, across the GPS's fresh fixes for the slope to be
/// measured: two readings each 6 m off, the vertical error allowed for, in opposite directions at
/// the span's ends tilt the slope by 12 m over it, which over 120 m is the 10% margin (module
/// docs, *The GPS altitude in meters*).
const GPS_SPAN_FLOOR_M: f64 = 120.0;

/// The fewest fresh fixes the slope is measured from: more than the two that fix a line exactly,
/// so that the scatter about it is measured too.
const GPS_MIN_FIXES: usize = 10;

/// The most the fresh fixes may scatter about their line, RMS, as a share of the barometric
/// altitude's span across them: a receiver that lags far behind the flight traces a loop, not a
/// line, and the slope of a loop doesn't tell the unit (module docs, *The GPS altitude in
/// meters*).
const GPS_SCATTER_SHARE: f64 = 0.1;

/// How far, s, the `deltaTime` may lie from the clock's step for the two to agree: a microsecond,
/// far below the millisecond both are written in.
const STEP_TOLERANCE_S: f64 = 1e-6;

/// The manual's names for the flight sequencer's codes, 0 to 6 (p23).
const STATUS_NAMES: [&str; 7] = [
    "IDLE",
    "ARMED",
    "COUNTDOWN ENGAGED",
    "WAITING FOR LAUNCH",
    "ASCENT",
    "DESCENT",
    "TOUCHDOWN",
];

/// A pyro output's state when it fires (manual p23).
const PYRO_FIRED: f64 = 10.0;

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
    Pressure,
}

/// A unit the header may state: what it measures, and the value in SI (or degrees) as
/// `written × multiply / divide + add`. Dividing keeps a thousandth exact in the record, as
/// 1143.492 s for 1143492 ms.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Unit {
    dimension: Dimension,
    multiply: f64,
    divide: f64,
    add: f64,
}

impl Unit {
    const fn new(dimension: Dimension, multiply: f64, divide: f64, add: f64) -> Self {
        Self {
            dimension,
            multiply,
            divide,
            add,
        }
    }

    /// The value in SI (or degrees); infinite for a value too large for an `f64` once
    /// converted, as `1e308` in `deg f`, where the conversion multiplies by 5 before dividing by 9.
    fn convert(self, written: f64) -> f64 {
        written * self.multiply / self.divide + self.add
    }

    /// The value in SI (or degrees), a value the conversion makes infinite refused on its line
    /// as a value too large for its unit ([ADR-258][adr-258] §3).
    ///
    /// [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
    fn convert_on(self, written: f64, line: usize, column: &str) -> Result<f64, LogError> {
        table::finite(self.convert(written), FORMAT, line, column)
    }
}

/// One foot, m.
const FOOT_M: f64 = 0.3048;

/// Standard gravity, m/s² (CGPM 1901).
const G0_M_S2: f64 = 9.806_65;

/// A radian, degrees.
const RADIAN_DEG: f64 = 180.0 / std::f64::consts::PI;

/// The units the reader knows, in lower case with spaces taken out (module docs, *Columns*).
const UNITS: [(&str, Unit); 33] = [
    ("ms", Unit::new(Dimension::Time, 1.0, 1000.0, 0.0)),
    ("s", Unit::new(Dimension::Time, 1.0, 1.0, 0.0)),
    ("us", Unit::new(Dimension::Time, 1.0, 1e6, 0.0)),
    ("µs", Unit::new(Dimension::Time, 1.0, 1e6, 0.0)),
    ("m", Unit::new(Dimension::Length, 1.0, 1.0, 0.0)),
    ("ft", Unit::new(Dimension::Length, FOOT_M, 1.0, 0.0)),
    ("m/s", Unit::new(Dimension::Speed, 1.0, 1.0, 0.0)),
    ("ft/s", Unit::new(Dimension::Speed, FOOT_M, 1.0, 0.0)),
    ("km/h", Unit::new(Dimension::Speed, 1.0, 3.6, 0.0)),
    ("m/s2", Unit::new(Dimension::Acceleration, 1.0, 1.0, 0.0)),
    ("m/s²", Unit::new(Dimension::Acceleration, 1.0, 1.0, 0.0)),
    ("m/s^2", Unit::new(Dimension::Acceleration, 1.0, 1.0, 0.0)),
    (
        "ft/s2",
        Unit::new(Dimension::Acceleration, FOOT_M, 1.0, 0.0),
    ),
    (
        "ft/s²",
        Unit::new(Dimension::Acceleration, FOOT_M, 1.0, 0.0),
    ),
    ("deg", Unit::new(Dimension::Angle, 1.0, 1.0, 0.0)),
    ("°", Unit::new(Dimension::Angle, 1.0, 1.0, 0.0)),
    ("rad", Unit::new(Dimension::Angle, RADIAN_DEG, 1.0, 0.0)),
    ("deg/s", Unit::new(Dimension::AngularRate, 1.0, 1.0, 0.0)),
    ("°/s", Unit::new(Dimension::AngularRate, 1.0, 1.0, 0.0)),
    (
        "rad/s",
        Unit::new(Dimension::AngularRate, RADIAN_DEG, 1.0, 0.0),
    ),
    ("degc", Unit::new(Dimension::Temperature, 1.0, 1.0, 273.15)),
    ("°c", Unit::new(Dimension::Temperature, 1.0, 1.0, 273.15)),
    (
        "degf",
        Unit::new(Dimension::Temperature, 5.0, 9.0, 273.15 - 32.0 * 5.0 / 9.0),
    ),
    (
        "°f",
        Unit::new(Dimension::Temperature, 5.0, 9.0, 273.15 - 32.0 * 5.0 / 9.0),
    ),
    ("k", Unit::new(Dimension::Temperature, 1.0, 1.0, 0.0)),
    ("mv", Unit::new(Dimension::Voltage, 1.0, 1000.0, 0.0)),
    ("v", Unit::new(Dimension::Voltage, 1.0, 1.0, 0.0)),
    ("pa", Unit::new(Dimension::Pressure, 1.0, 1.0, 0.0)),
    ("hpa", Unit::new(Dimension::Pressure, 100.0, 1.0, 0.0)),
    ("mbar", Unit::new(Dimension::Pressure, 100.0, 1.0, 0.0)),
    ("kpa", Unit::new(Dimension::Pressure, 1000.0, 1.0, 0.0)),
    ("g", Unit::new(Dimension::Acceleration, G0_M_S2, 1.0, 0.0)),
    ("μs", Unit::new(Dimension::Time, 1.0, 1e6, 0.0)),
];

/// A unit as the header writes it, if the reader knows it.
fn unit_of(written: &str) -> Option<Unit> {
    let key: String = written
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    UNITS
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, unit)| *unit)
}

/// What a known column is to the record beside its channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    None,
    Altitude,
    VerticalSpeed,
    Temperature,
    Battery,
}

/// What a header column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// `time`: the clock.
    Clock,
    /// `deltaTime`: the clock's step, checked.
    Step,
    /// `status`: the flight sequencer's code.
    Status,
    /// A pyro output's state.
    Pyro,
    /// A count, flag or code without a unit.
    Number,
    /// A reading whose unit the header states.
    Reading(Quantity, Dimension, Role),
    Latitude,
    Longitude,
    GpsAltitude,
    /// A column the reader doesn't know: read by its unit, if that can mean one quantity only.
    Unknown,
}

/// The names the reader knows, in lower case, and what each holds (module docs, *Columns*).
const KNOWN: [(&str, Kind); 23] = [
    ("time", Kind::Clock),
    ("deltatime", Kind::Step),
    ("status", Kind::Status),
    (
        "baro-altitude",
        Kind::Reading(
            Quantity::AltitudeAboveGround,
            Dimension::Length,
            Role::Altitude,
        ),
    ),
    (
        "dedrck-v-speed",
        Kind::Reading(Quantity::Speed, Dimension::Speed, Role::None),
    ),
    (
        "angle",
        Kind::Reading(Quantity::Angle, Dimension::Angle, Role::None),
    ),
    (
        "roll-rate",
        Kind::Reading(Quantity::AngularRate, Dimension::AngularRate, Role::None),
    ),
    (
        "vert-accel",
        Kind::Reading(Quantity::Acceleration, Dimension::Acceleration, Role::None),
    ),
    (
        "accel",
        Kind::Reading(Quantity::Acceleration, Dimension::Acceleration, Role::None),
    ),
    (
        "dedrck-alti",
        Kind::Reading(Quantity::InertialAltitude, Dimension::Length, Role::None),
    ),
    (
        "baro-speed",
        Kind::Reading(Quantity::Speed, Dimension::Speed, Role::VerticalSpeed),
    ),
    (
        "amb-temp",
        Kind::Reading(
            Quantity::Temperature,
            Dimension::Temperature,
            Role::Temperature,
        ),
    ),
    (
        "batt-voltage",
        Kind::Reading(Quantity::Voltage, Dimension::Voltage, Role::Battery),
    ),
    ("p1-state", Kind::Pyro),
    ("p2-state", Kind::Pyro),
    ("p3-state", Kind::Pyro),
    (
        "analog1",
        Kind::Reading(Quantity::Voltage, Dimension::Voltage, Role::None),
    ),
    (
        "analog2",
        Kind::Reading(Quantity::Voltage, Dimension::Voltage, Role::None),
    ),
    ("infreefall", Kind::Number),
    ("gpslat", Kind::Latitude),
    ("gpslng", Kind::Longitude),
    ("gpsstate", Kind::Number),
    ("gpssats", Kind::Number),
];

/// What a column named so holds: `gpsAltMSL`, or the manual's `gpsAlt`, is the GPS altitude.
fn kind_of(name: &str) -> Kind {
    if name.eq_ignore_ascii_case("gpsaltmsl") || name.eq_ignore_ascii_case("gpsalt") {
        return Kind::GpsAltitude;
    }
    KNOWN
        .iter()
        .find(|(known, _)| name.eq_ignore_ascii_case(known))
        .map_or(Kind::Unknown, |(_, kind)| *kind)
}

/// A header cell's name and the unit in the brackets at its end, both trimmed: `baro-altitude
/// (m)` is `baro-altitude` and `m`.
fn split_name(cell: &str) -> (&str, Option<&str>) {
    let cell = cell.trim();
    if let Some(inner) = cell.strip_suffix(')')
        && let Some(open) = inner.rfind('(')
    {
        let name = inner[..open].trim();
        if !name.is_empty() {
            return (name, Some(inner[open + 1..].trim()));
        }
    }
    (cell, None)
}

/// The header: its line number, its text and the separator.
struct Header<'a> {
    line: usize,
    text: &'a str,
    separator: char,
}

/// The header, if the file is in this layout (module docs, *Recognition*).
fn find_header(text: &str) -> Option<Header<'_>> {
    let mut lines = numbered_lines(text)
        .take(HEAD_LINES)
        .filter(|(_, line)| !line.trim().is_empty());
    let (first_number, first) = lines.next()?;
    let first = first.trim_start_matches([' ', '\t']);
    let declared = first
        .get(..4)
        .filter(|prefix| prefix.eq_ignore_ascii_case("sep="))
        .and_then(|_| first.get(4..));
    let header = match declared {
        Some(rest) => {
            // One character, and nothing after it but spaces.
            let mut chars = rest.trim_end_matches(' ').chars();
            let separator = chars.next()?;
            if chars.next().is_some() || !matches!(separator, ';' | ',' | '\t' | '|') {
                return None;
            }
            let (line, text) = lines.next()?;
            Header {
                line,
                text,
                separator,
            }
        }
        None => Header {
            line: first_number,
            text: first,
            separator: ';',
        },
    };
    is_header(header.text, header.separator).then_some(header)
}

/// Whether a line is this layout's header: `time` first, and `deltaTime` and `baro-altitude`
/// among the names. The cells are looked at one by one, never collected, so a long line costs no
/// memory.
fn is_header(line: &str, separator: char) -> bool {
    let mut names = line.split(separator).map(|cell| split_name(cell).0);
    if !names
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case("time"))
    {
        return false;
    }
    let (mut step, mut altitude) = (false, false);
    for name in names {
        step |= name.eq_ignore_ascii_case("deltatime");
        altitude |= name.eq_ignore_ascii_case("baro-altitude");
        if step && altitude {
            return true;
        }
    }
    false
}

/// Whether the text is in this reader's layout, judged from its first lines, never the file's
/// name.
pub fn recognizes(text: &str) -> bool {
    find_header(text).is_some()
}

/// A header column.
struct Column {
    /// The name as written, unit and all, as the channel keeps it.
    written: String,
    kind: Kind,
    /// The unit the header states, if the reader knows it.
    unit: Option<Unit>,
    /// The unit as the header writes it, if it writes one.
    unit_written: Option<String>,
    /// Cells that are empty or not numbers, and the first of them.
    gaps: usize,
    gap_example: Option<String>,
    /// Cells holding a 32-bit placeholder, and the first one's line.
    placeholders: usize,
    placeholder_line: usize,
}

/// The header's columns (module docs, *Columns*).
fn columns(header: &Header<'_>) -> Result<Vec<Column>, LogError> {
    let mut columns: Vec<Column> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (index, cell) in header.text.split(header.separator).enumerate() {
        if index >= MAX_COLUMNS {
            return Err(syntax(
                header.line,
                format!("the header names more than {MAX_COLUMNS} columns"),
            ));
        }
        let written = cell.trim();
        let (name, unit_written) = split_name(written);
        if written.is_empty() {
            return Err(syntax(
                header.line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(syntax(
                header.line,
                format!("the header names {:?} twice", short(name)),
            ));
        }
        columns.push(Column {
            written: written.to_owned(),
            kind: kind_of(name),
            unit: unit_written.and_then(unit_of),
            unit_written: unit_written.map(str::to_owned),
            gaps: 0,
            gap_example: None,
            placeholders: 0,
            placeholder_line: 0,
        });
    }
    // The header was recognized by its first name, `time`, and its `baro-altitude`.
    let unit_error = |what: &str, wanted: &str| LogError::Unit {
        format: FORMAT,
        line: header.line,
        message: format!("{what} states no unit of {wanted} this reader knows"),
    };
    if columns
        .first()
        .and_then(|clock| clock.unit)
        .is_none_or(|unit| unit.dimension != Dimension::Time)
    {
        return Err(unit_error("the clock, `time`,", "time"));
    }
    let altitude_in_length = columns.iter().any(|column| {
        matches!(column.kind, Kind::Reading(_, _, Role::Altitude))
            && column
                .unit
                .is_some_and(|unit| unit.dimension == Dimension::Length)
    });
    if !altitude_in_length {
        return Err(unit_error("`baro-altitude`", "length"));
    }
    Ok(columns)
}

/// Whether a value is one of the 32-bit placeholders (module docs, *Placeholders*).
fn is_placeholder(value: f64) -> bool {
    value == f64::from(i32::MAX) || value == f64::from(i32::MIN)
}

/// The rows read: the clock as written, and each column's values as written, gaps `NaN`.
struct Rows {
    lines: Vec<usize>,
    clock: Vec<f64>,
    values: Vec<Vec<f64>>,
}

/// Reads the data rows (module docs, *Rows*).
fn read_rows<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
    columns: &mut [Column],
    separator: char,
) -> Result<Rows, LogError> {
    let width = columns.len();
    let decimal_comma = separator != ',';
    let mut rows = Rows {
        lines: Vec::new(),
        clock: Vec::new(),
        values: columns.iter().map(|_| Vec::new()).collect(),
    };
    for (number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        // One more than the header names, so that a wider row is told apart.
        let cells: Vec<&str> = line
            .split(separator)
            .map(str::trim)
            .take(width + 1)
            .collect();
        if cells.len() != width {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells, where the header names {width}",
                    if cells.len() > width {
                        format!("more than {width}")
                    } else {
                        cells.len().to_string()
                    }
                ),
            ));
        }
        let clock_cell = cells.first().copied().unwrap_or_default();
        let Some(time) = table::number(clock_cell, decimal_comma) else {
            return Err(syntax(
                number,
                format!("the clock's cell {:?} isn't a number", short(clock_cell)),
            ));
        };
        if let Some(&last) = rows.clock.last()
            && time <= last
        {
            return Err(syntax(
                number,
                format!("the time {time} doesn't come after the one above it, {last}"),
            ));
        }
        for (index, (column, cell)) in columns.iter_mut().zip(&cells).enumerate().skip(1) {
            let value = match table::number(cell, decimal_comma) {
                Some(value) if is_placeholder(value) => {
                    if column.placeholders == 0 {
                        column.placeholder_line = number;
                    }
                    column.placeholders += 1;
                    f64::NAN
                }
                Some(value) => value,
                None => {
                    column.gaps += 1;
                    if column.gap_example.is_none() {
                        column.gap_example = Some(short(cell));
                    }
                    f64::NAN
                }
            };
            if let Some(store) = rows.values.get_mut(index) {
                store.push(value);
            }
        }
        rows.lines.push(number);
        rows.clock.push(time);
    }
    Ok(rows)
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// Reads a log in this layout.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the file's first lines aren't this layout's (module docs,
/// *Recognition*); [`LogError::Unit`] if the clock isn't in a unit of time or `baro-altitude` in
/// one of length; [`LogError::Syntax`] for a header of more than [`MAX_COLUMNS`] names, naming a
/// column twice or leaving a name empty, a row of the wrong width, a clock cell that isn't a
/// number or doesn't increase, or a reading its unit's conversion makes infinite, naming the
/// line; [`LogError::NoData`] if no row follows the header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let Some(header) = find_header(text) else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "within its first {HEAD_LINES} lines there is no Fluctus header, a `sep=` line \
                 if any, then names starting `time` and naming `deltaTime` and `baro-altitude`"
            ),
        });
    };
    let mut columns = columns(&header)?;
    let rows = read_rows(
        numbered_lines(text).skip_while(|(number, _)| *number <= header.line),
        &mut columns,
        header.separator,
    )?;
    if rows.clock.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    build(&columns, &rows)
}

/// What the reader does with a column, decided from its kind, its unit and, for the position and
/// the GPS altitude, its values.
#[derive(Debug, Clone, Copy)]
enum Reading {
    /// A channel of this quantity, converted so.
    Channel(Quantity, Unit),
    /// Kept as written, [`Quantity::Unrecognized`]; `true` once a note on the evidence says why.
    Kept(bool),
    /// `deltaTime`, checked against the clock.
    Step(Unit),
}

/// No conversion.
const AS_WRITTEN: Unit = Unit::new(Dimension::Length, 1.0, 1.0, 0.0);

/// The record from the rows read; a reading its unit's conversion makes infinite is refused on
/// its line.
fn build(columns: &[Column], rows: &Rows) -> Result<FlightLog, LogError> {
    let clock = columns.first();
    let clock_name = clock.map_or("time", |clock| clock.written.as_str());
    let clock_unit = clock.and_then(|clock| clock.unit).unwrap_or(AS_WRITTEN);
    let time_s = convert_column(clock_unit, &rows.clock, &rows.lines, clock_name)?;
    let mut log = FlightLog {
        format: LogFormat::FluctusCsv,
        logger: "Silicdyne Fluctus".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::new(),
        events: Vec::new(),
        notes: Vec::new(),
    };
    // The notes on columns kept as written, at most one a column, and few.
    let mut kept = Notes::default();
    // The barometric altitude, m, which the GPS altitude is measured against.
    let altitude_m: Vec<f64> = match columns
        .iter()
        .zip(&rows.values)
        .find(|(column, _)| matches!(column.kind, Kind::Reading(_, _, Role::Altitude)))
    {
        Some((column, values)) => convert_column(
            column.unit.unwrap_or(AS_WRITTEN),
            values,
            &rows.lines,
            &column.written,
        )?,
        None => Vec::new(),
    };
    let mut events: Vec<(usize, Event)> = Vec::new();

    for (index, column) in columns.iter().enumerate().skip(1) {
        let Some(values) = rows.values.get(index) else {
            continue;
        };
        let name = short(&column.written);
        let reading = reading_of(column, values, &altitude_m, &mut log.notes, &mut kept);
        let (quantity, unit) = match reading {
            Reading::Step(unit) => {
                step_check(column, unit, values, &time_s, rows, &mut log.notes);
                continue;
            }
            Reading::Channel(quantity, unit) => (quantity, unit),
            Reading::Kept(_) => (Quantity::Unrecognized, AS_WRITTEN),
        };
        match column.kind {
            Kind::Status => events.extend(changes(values, |code| Some(status_name(code)), &time_s)),
            Kind::Pyro => events.extend(changes(
                values,
                |state| {
                    (state == PYRO_FIRED)
                        .then(|| format!("{} ENABLED", split_name(&column.written).0))
                },
                &time_s,
            )),
            _ => {}
        }
        if column.gaps > 0 {
            log.notes.push(format!(
                "the column `{name}` holds {} that {} empty or not a number, such as {:?}: they \
                 are gaps",
                count(column.gaps, "cell"),
                if column.gaps == 1 { "is" } else { "are" },
                column.gap_example.as_deref().unwrap_or_default()
            ));
        }
        if column.placeholders > 0 {
            log.notes.push(format!(
                "the column `{name}` writes 2147483647 or -2147483648, the ends of a signed 32-bit \
                 number, on {} from line {}: no reading, so they are gaps",
                count(column.placeholders, "row"),
                column.placeholder_line
            ));
        }
        let converted = convert_column(unit, values, &rows.lines, &column.written)?;
        if let Kind::Reading(_, _, role) = column.kind
            && quantity != Quantity::Unrecognized
        {
            match role {
                Role::Altitude if log.altitude_m.is_empty() => log.altitude_m = converted.clone(),
                Role::VerticalSpeed if log.vertical_speed_m_s.is_none() => {
                    log.vertical_speed_m_s = Some(converted.clone());
                }
                Role::Temperature if log.temperature_k.is_none() => {
                    log.temperature_k = Some(converted.clone());
                }
                Role::Battery if log.battery_v.is_none() => {
                    log.battery_v = Some(converted.clone());
                }
                _ => {}
            }
        }
        log.channels.push(Channel {
            column: column.written.clone(),
            quantity,
            time_s: time_s.clone(),
            values: converted,
        });
    }
    // In time order; a status change before a pyro event on the same row, as the file's columns.
    events.sort_by_key(|(row, _)| *row);
    log.events = events.into_iter().map(|(_, event)| event).collect();
    clock_steps(&time_s, &rows.lines, &mut log.notes);
    log.notes.extend(kept.into_vec());
    log.time_s = time_s;
    Ok(log)
}

/// A column's values converted from its unit, each with its row's line: a value the conversion
/// makes infinite, as `1.7e308` in `hPa` or `g`, is refused on its line ([ADR-258][adr-258] §3).
///
/// [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
fn convert_column(
    unit: Unit,
    values: &[f64],
    lines: &[usize],
    column: &str,
) -> Result<Vec<f64>, LogError> {
    values
        .iter()
        .zip(lines)
        .map(|(value, line)| unit.convert_on(*value, *line, column))
        .collect()
}

/// The note on a column kept as written, saying why.
fn kept_note(column: &Column) -> String {
    let name = short(&column.written);
    let why = match (&column.unit_written, column.unit, column.kind) {
        (None, _, Kind::Unknown) => {
            "this reader doesn't know the column, and the header states no unit".to_owned()
        }
        (None, _, _) => "the header states no unit for it".to_owned(),
        (Some(unit), None, _) => {
            format!("its unit, {:?}, isn't one this reader knows", short(unit))
        }
        (Some(_), Some(unit), Kind::Unknown) => format!(
            "this reader doesn't know the column, and its unit, a {}, could be more than one \
             quantity",
            dimension_word(unit.dimension)
        ),
        (Some(_), Some(unit), _) => format!(
            "its unit is a {}, not what the column holds",
            dimension_word(unit.dimension)
        ),
    };
    format!("`{name}` kept as written: {why}")
}

fn dimension_word(dimension: Dimension) -> &'static str {
    match dimension {
        Dimension::Time => "time",
        Dimension::Length => "length",
        Dimension::Speed => "speed",
        Dimension::Acceleration => "acceleration",
        Dimension::Angle => "angle",
        Dimension::AngularRate => "rate of turn",
        Dimension::Temperature => "temperature",
        Dimension::Voltage => "voltage",
        Dimension::Pressure => "pressure",
    }
}

/// How a column is read (module docs, *Columns*).
fn reading_of(
    column: &Column,
    values: &[f64],
    altitude_m: &[f64],
    notes: &mut Vec<String>,
    kept: &mut Notes,
) -> Reading {
    let reading = decided(column, values, altitude_m, notes);
    if let Reading::Kept(false) = reading {
        kept.push(kept_note(column));
    }
    reading
}

/// How a column is read, and the notes on the decisions made from evidence; a column kept as
/// written for its unit alone is left for [`reading_of`] to note.
fn decided(
    column: &Column,
    values: &[f64],
    altitude_m: &[f64],
    notes: &mut Vec<String>,
) -> Reading {
    let stated = |dimension: Dimension| column.unit.filter(|unit| unit.dimension == dimension);
    let kept = Reading::Kept(false);
    match column.kind {
        Kind::Clock => kept,
        Kind::Step => stated(Dimension::Time).map_or(kept, Reading::Step),
        Kind::Status | Kind::Pyro | Kind::Number => {
            if column.unit_written.is_some() {
                kept
            } else {
                Reading::Channel(Quantity::Number, AS_WRITTEN)
            }
        }
        Kind::Reading(quantity, dimension, _) => {
            stated(dimension).map_or(kept, |unit| Reading::Channel(quantity, unit))
        }
        Kind::Latitude | Kind::Longitude => {
            let (quantity, limit) = if column.kind == Kind::Latitude {
                (Quantity::Latitude, 90.0)
            } else {
                (Quantity::Longitude, 180.0)
            };
            if column.unit_written.is_some() {
                return stated(Dimension::Angle)
                    .map_or(kept, |unit| Reading::Channel(quantity, unit));
            }
            match degrees(values, limit) {
                Degrees::NoReading => {
                    notes.push(format!(
                        "`{}` kept as written, in a unit no reading bears out: the header states \
                         none, and the column holds no reading to check against ±{limit}°",
                        short(&column.written)
                    ));
                    Reading::Kept(true)
                }
                Degrees::Within => {
                    notes.push(format!(
                        "`{}` read in degrees, from the manual and the values. The header states \
                         no unit. The manual's picture of the Locate tab shows the position in \
                         decimal degrees (p38). Every reading lies within ±{limit}°",
                        short(&column.written)
                    ));
                    Reading::Channel(quantity, AS_WRITTEN)
                }
                Degrees::Beyond(beyond) => {
                    notes.push(format!(
                        "`{}` kept as written, in a unit the values don't bear out. The header \
                         states none. A reading of {beyond} lies beyond ±{limit}°, so it \
                         isn't in degrees",
                        short(&column.written)
                    ));
                    Reading::Kept(true)
                }
            }
        }
        Kind::GpsAltitude => {
            if column.unit_written.is_some() {
                let reading = stated(Dimension::Length)
                    .map_or(kept, |unit| Reading::Channel(Quantity::GnssAltitude, unit));
                return reading;
            }
            match gps_check(values, altitude_m) {
                GpsCheck::Meters(fit) => {
                    notes.push(format!(
                        "`{}` read in meters, from the barometric altitude. The header and the \
                         manual state no unit. At its {} fresh fixes it rises {:.2} for each meter \
                         of the barometric altitude, within {:.0}% of 1. In feet it would be \
                         3.28. They span {} m of the barometric altitude and \
                         scatter {} about that line",
                        short(&column.written),
                        fit.fixes,
                        fit.slope,
                        GPS_SLOPE_MARGIN * 100.0,
                        rounded(fit.span_m),
                        rounded(fit.scatter)
                    ));
                    Reading::Channel(Quantity::GnssAltitude, AS_WRITTEN)
                }
                GpsCheck::Undecided(why) => {
                    notes.push(format!(
                        "`{}` kept as written, in a unit no source states: {why}",
                        short(&column.written)
                    ));
                    Reading::Kept(true)
                }
            }
        }
        Kind::Unknown => match column.unit {
            Some(unit) => {
                let quantity = match unit.dimension {
                    Dimension::Speed => Quantity::Speed,
                    Dimension::Acceleration => Quantity::Acceleration,
                    Dimension::Angle => Quantity::Angle,
                    Dimension::AngularRate => Quantity::AngularRate,
                    Dimension::Temperature => Quantity::Temperature,
                    Dimension::Voltage => Quantity::Voltage,
                    Dimension::Pressure => Quantity::Pressure,
                    Dimension::Time | Dimension::Length => return kept,
                };
                notes.push(format!(
                    "the column `{}` isn't one this reader knows: it is read as a {}, as its unit \
                     says",
                    short(&column.written),
                    dimension_word(unit.dimension)
                ));
                Reading::Channel(quantity, unit)
            }
            None => kept,
        },
    }
}

/// A count and its noun, plural but for one: `1 row`, `2 rows`.
fn count(number: usize, noun: &str) -> String {
    if number == 1 {
        format!("1 {noun}")
    } else {
        format!("{number} {noun}s")
    }
}

/// A number as a note writes it: to the meter, as the file writes its altitudes.
fn rounded(value: f64) -> String {
    format!("{value:.0}")
}

/// What the position's check found (module docs, *The position in degrees*).
#[derive(Debug, PartialEq)]
enum Degrees {
    /// At least one reading, and every one within ±`limit`.
    Within,
    /// The first reading beyond ±`limit`.
    Beyond(f64),
    /// No reading at all: every cell a gap.
    NoReading,
}

/// Whether every reading, gaps aside, lies within ±`limit`; a column of gaps alone bears out
/// nothing.
fn degrees(values: &[f64], limit: f64) -> Degrees {
    if !values.iter().any(|value| value.is_finite()) {
        return Degrees::NoReading;
    }
    match values.iter().find(|value| value.abs() > limit) {
        Some(beyond) => Degrees::Beyond(*beyond),
        None => Degrees::Within,
    }
}

/// The GPS altitude's straight line against the barometric altitude, by least squares at the
/// GPS's fresh fixes.
#[derive(Debug, Clone, Copy, PartialEq)]
struct GpsFit {
    /// The fresh fixes the line is drawn through.
    fixes: usize,
    /// The barometric altitude's span across them, m.
    span_m: f64,
    /// The GPS altitude's change for each meter of the barometric altitude's.
    slope: f64,
    /// The fixes' RMS distance from the line, in the GPS altitude's own unit, on `fixes − 2`
    /// degrees of freedom.
    scatter: f64,
}

/// What the GPS altitude's check found (module docs, *The GPS altitude in meters*).
#[derive(Debug, PartialEq)]
enum GpsCheck {
    Meters(GpsFit),
    /// Why the check can't decide.
    Undecided(String),
}

/// The (barometric altitude, GPS altitude) pairs at the GPS's fresh fixes: the rows where the GPS
/// altitude differs from its last reading before, the first reading included. A reading that
/// repeats the last, as the receiver holds its last fix while it has none, or a gap, is left out;
/// so is a row whose barometric altitude is a gap.
fn fresh_fixes(gps: &[f64], altitude_m: &[f64]) -> Vec<(f64, f64)> {
    let mut fixes = Vec::new();
    let mut last: Option<f64> = None;
    for (&reading, &altitude) in gps.iter().zip(altitude_m) {
        if !reading.is_finite() {
            continue;
        }
        if last != Some(reading) && altitude.is_finite() {
            fixes.push((altitude, reading));
        }
        last = Some(reading);
    }
    fixes
}

/// The least-squares line through the fixes, about their means; `None` for fewer than three
/// fixes or none apart in the barometric altitude.
fn gps_fit(fixes: &[(f64, f64)]) -> Option<GpsFit> {
    if fixes.len() < 3 {
        return None;
    }
    let n = fixes.len() as f64;
    let mean_x = fixes.iter().map(|(x, _)| x).sum::<f64>() / n;
    let mean_y = fixes.iter().map(|(_, y)| y).sum::<f64>() / n;
    let sxx: f64 = fixes.iter().map(|(x, _)| (x - mean_x).powi(2)).sum();
    let sxy: f64 = fixes.iter().map(|(x, y)| (x - mean_x) * (y - mean_y)).sum();
    let apart = sxx > 0.0;
    if !apart {
        return None;
    }
    let slope = sxy / sxx;
    let residuals: f64 = fixes
        .iter()
        .map(|(x, y)| (y - mean_y - slope * (x - mean_x)).powi(2))
        .sum();
    let (low, high) = fixes
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), (x, _)| {
            (low.min(*x), high.max(*x))
        });
    Some(GpsFit {
        fixes: fixes.len(),
        span_m: high - low,
        slope,
        scatter: (residuals / (n - 2.0)).sqrt(),
    })
}

/// Measures the GPS altitude against the barometric altitude, m, at the GPS's fresh fixes: read
/// in meters only where the fixes are enough, span enough of the flight, lie along a line and
/// that line's slope is within [`GPS_SLOPE_MARGIN`] of 1. Every comparison fails closed, so a NaN
/// decides nothing.
fn gps_check(gps: &[f64], altitude_m: &[f64]) -> GpsCheck {
    let fixes = fresh_fixes(gps, altitude_m);
    if fixes.is_empty() {
        return GpsCheck::Undecided(
            "the file has no GPS altitude or no barometric altitude to measure it against"
                .to_owned(),
        );
    }
    if fixes.len() < GPS_MIN_FIXES {
        return GpsCheck::Undecided(format!(
            "it changes on {} where the barometric altitude is read. That is fewer \
             than the {GPS_MIN_FIXES} fresh fixes the check needs to measure a line and the \
             scatter about it",
            count(fixes.len(), "row")
        ));
    }
    let span = |fit: Option<GpsFit>| fit.map_or(0.0, |fit| fit.span_m);
    let fit = gps_fit(&fixes);
    let fit = match fit {
        Some(fit) if fit.span_m >= GPS_SPAN_FLOOR_M => fit,
        _ => {
            return GpsCheck::Undecided(format!(
                "its {} fresh fixes span {} m of the barometric altitude, less than the {} m the \
                 check needs to tell meters from feet",
                fixes.len(),
                rounded(span(fit)),
                rounded(GPS_SPAN_FLOOR_M)
            ));
        }
    };
    let on_a_line = fit.scatter <= GPS_SCATTER_SHARE * fit.span_m;
    if !on_a_line {
        return GpsCheck::Undecided(format!(
            "its {} fresh fixes scatter {} about their line, more than {:.0}% of the {} m of \
             the barometric altitude they span. They trace a loop, as a receiver lagging far \
             behind the flight does. The slope of a loop doesn't tell the unit",
            fit.fixes,
            rounded(fit.scatter),
            GPS_SCATTER_SHARE * 100.0,
            rounded(fit.span_m)
        ));
    }
    let near_1 = (fit.slope - 1.0).abs() <= GPS_SLOPE_MARGIN;
    if !near_1 {
        return GpsCheck::Undecided(format!(
            "at its {} fresh fixes it rises {:.2} for each meter of the barometric altitude, \
             over {} m of it. That is not within {:.0}% of 1 as in meters. In feet it would be \
             3.28",
            fit.fixes,
            fit.slope,
            rounded(fit.span_m),
            GPS_SLOPE_MARGIN * 100.0
        ));
    }
    GpsCheck::Meters(fit)
}

/// The manual's name for a flight sequencer's code (p23), or `status` and the number.
fn status_name(code: f64) -> String {
    let named = (code.fract() == 0.0 && (0.0..=6.0).contains(&code))
        .then(|| STATUS_NAMES.get(code as usize))
        .flatten();
    named.map_or_else(|| format!("status {code}"), |name| (*name).to_owned())
}

/// An event at each row where the values change to one that `name` names, with the row's index;
/// a gap changes nothing.
fn changes(
    values: &[f64],
    name: impl Fn(f64) -> Option<String>,
    time_s: &[f64],
) -> Vec<(usize, Event)> {
    let mut events = Vec::new();
    let mut last: Option<f64> = None;
    for (row, (value, time)) in values.iter().zip(time_s).enumerate() {
        if value.is_nan() {
            continue;
        }
        if let Some(before) = last
            && before != *value
            && let Some(name) = name(*value)
        {
            events.push((
                row,
                Event {
                    time_s: *time,
                    name,
                },
            ));
        }
        last = Some(*value);
    }
    events
}

/// Checks `deltaTime` against the clock's step; a note counts the rows where they differ.
fn step_check(
    column: &Column,
    unit: Unit,
    values: &[f64],
    time_s: &[f64],
    rows: &Rows,
    notes: &mut Vec<String>,
) {
    let mut differ = 0;
    let mut first_line = 0;
    for (row, value) in values.iter().enumerate().skip(1) {
        let (Some(now), Some(before)) = (time_s.get(row), time_s.get(row - 1)) else {
            continue;
        };
        if value.is_nan() {
            continue;
        }
        if (unit.convert(*value) - (now - before)).abs() > STEP_TOLERANCE_S {
            if differ == 0 {
                first_line = rows.lines.get(row).copied().unwrap_or_default();
            }
            differ += 1;
        }
    }
    if differ > 0 {
        notes.push(format!(
            "`{}` differs from the clock's step on {}, the first on line {first_line}: \
             the clock is kept, and `{0}` isn't",
            short(&column.written),
            count(differ, "row")
        ));
    }
    if column.gaps > 0 {
        notes.push(format!(
            "the column `{}` holds {} that {} empty or not a number: it is only checked against \
             the clock, so nothing is lost",
            short(&column.written),
            count(column.gaps, "cell"),
            if column.gaps == 1 { "is" } else { "are" }
        ));
    }
}

/// A note counting the clock's steps longer than its commonest, if any.
fn clock_steps(time_s: &[f64], lines: &[usize], notes: &mut Vec<String>) {
    // Each step in whole microseconds, so that equal steps count together.
    let steps: Vec<i64> = time_s
        .windows(2)
        .map(|pair| ((pair[1] - pair[0]) * 1e6).round() as i64)
        .collect();
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for step in &steps {
        *counts.entry(*step).or_default() += 1;
    }
    let Some(commonest) = counts
        .iter()
        .max_by_key(|(step, count)| (**count, std::cmp::Reverse(**step)))
        .map(|(step, _)| *step)
    else {
        return;
    };
    let longer: Vec<(usize, i64)> = steps
        .iter()
        .enumerate()
        .filter(|(_, step)| **step > commonest)
        .map(|(index, step)| (index, *step))
        .collect();
    let Some(&(longest_at, longest)) = longer.iter().max_by_key(|(_, step)| *step) else {
        return;
    };
    let in_ms = |microseconds: i64| format!("{} ms", microseconds as f64 / 1000.0);
    notes.push(format!(
        "the clock steps by {} on most rows, and by more {}: the longest, {}, before line {}; \
         the logger wrote no row between",
        in_ms(commonest),
        if longer.len() == 1 {
            "once".to_owned()
        } else {
            format!("{} times", longer.len())
        },
        in_ms(longest),
        lines.get(longest_at + 1).copied().unwrap_or_default()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "time (ms);deltaTime (ms);status;baro-altitude (m);dedrck-v-speed \
                          (m/s);angle (deg);roll-rate (deg/s);vert-accel (m/s2);accel (m/s2);\
                          dedrck-alti (m);baro-speed (m/s);amb-temp (deg c);batt-voltage (mV);\
                          P1-state;P2-state;P3-state;analog1 (mV);analog2 (mV);inFreefall;gpsLat;\
                          gpsLng;gpsAltMSL;gpsState;gpsSats";

    /// A row of the full layout: the time, the status, the altitude and the GPS altitude as
    /// given, the rest invented constants.
    fn row(time_ms: u32, status: u8, altitude_m: i32, gps_m: i32) -> String {
        format!(
            "{time_ms};20;{status};{altitude_m};0;3,5;0;0,0;9,8;0;0,0;21;8400;1;1;0;0;0;0;\
             12,345678;-23,456789;{gps_m};1;8"
        )
    }

    /// A file of the full layout from (time, status, altitude, GPS altitude) rows.
    fn file(rows: &[(u32, u8, i32, i32)]) -> String {
        let mut text = format!("sep=;\n{HEADER}\n");
        for &(time, status, altitude, gps) in rows {
            text.push_str(&row(time, status, altitude, gps));
            text.push('\n');
        }
        text
    }

    fn notes_starting<'a>(log: &'a FlightLog, start: &str) -> Vec<&'a String> {
        log.notes.iter().filter(|n| n.starts_with(start)).collect()
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap_or_else(|| panic!("no {column}: {:?}", log.channels))
    }

    #[test]
    fn recognizes_the_sep_line_and_the_fluctus_names() {
        assert!(recognizes(&format!("sep=;\n{HEADER}\n")));
        assert!(recognizes(&format!(
            "\u{feff}\r\nSEP=;\r\n\r\n{HEADER}\r\n"
        )));
        // Without the hint, a semicolon-separated header of the Fluctus's names.
        assert!(recognizes(&format!("{HEADER}\n")));
        // The comma named as the separator.
        assert!(recognizes(&format!(
            "sep=,\n{}\n",
            HEADER.replace(';', ",")
        )));
        // Without `deltaTime`, without `baro-altitude`, or with another first name.
        assert!(!recognizes("sep=;\ntime (ms);status;baro-altitude (m)\n"));
        assert!(!recognizes(
            "sep=;\ntime (ms);deltaTime (ms);altitude (m)\n"
        ));
        assert!(!recognizes(
            "sep=;\nstatus;time (ms);deltaTime (ms);baro-altitude (m)\n"
        ));
        // A header split by another separator than the one the hint names.
        assert!(!recognizes(&format!("sep=,\n{HEADER}\n")));
        // A separator that could be part of a number, or more than one character.
        assert!(!recognizes("sep=.\ntime.deltaTime.baro-altitude\n"));
        assert!(!recognizes(&format!("sep=;;\n{HEADER}\n")));
        // The Eggtimer's and an RRC3's semicolon-separated files.
        assert!(!recognizes(
            "T;Alt;Veloc;FAlt;FVeloc;LDA;LowV;Apogee\n0;0;0;0;0;0;0;0\n"
        ));
        assert!(!recognizes(
            "Time;Altitude;Pressure;Velocity;Temperature;Events;Voltages\n0,00;0;1013,2;0\n"
        ));
        // The header too far down.
        let far = format!("{}sep=;\n{HEADER}\n", "x\n".repeat(HEAD_LINES));
        assert!(!recognizes(&far));
        assert!(!recognizes(""));
    }

    #[test]
    fn names_split_into_name_and_unit() {
        assert_eq!(
            split_name(" baro-altitude (m) "),
            ("baro-altitude", Some("m"))
        );
        assert_eq!(split_name("amb-temp (deg c)"), ("amb-temp", Some("deg c")));
        assert_eq!(split_name("gpsLat"), ("gpsLat", None));
        assert_eq!(split_name("(m)"), ("(m)", None));
        assert_eq!(split_name("odd (a(b)"), ("odd (a", Some("b")));
    }

    #[test]
    fn units_convert_to_si() {
        let cases = [
            ("ms", 1500.0, 1.5),
            ("s", 1.5, 1.5),
            ("us", 1.5e6, 1.5),
            ("m", 12.0, 12.0),
            ("ft", 100.0, 30.48),
            ("km/h", 36.0, 10.0),
            ("m/s2", 9.8, 9.8),
            ("M/S²", 9.8, 9.8),
            ("ft/s2", 10.0, 3.048),
            ("g", 1.0, 9.806_65),
            ("rad", std::f64::consts::PI, 180.0),
            ("deg c", 20.0, 293.15),
            ("°C", 20.0, 293.15),
            ("deg f", 212.0, 373.15),
            ("k", 300.0, 300.0),
            ("mV", 8400.0, 8.4),
            ("V", 8.4, 8.4),
            ("hPa", 1013.25, 101_325.0),
            ("kPa", 101.325, 101_325.0),
        ];
        for (written, value, si) in cases {
            let unit = unit_of(written).unwrap_or_else(|| panic!("{written}"));
            assert!((unit.convert(value) - si).abs() < 1e-9, "{written}");
        }
        // A thousandth stays exact: 1143.492, not 1143.4920000000002.
        assert_eq!(
            unit_of("ms").map(|u| u.convert(1_143_492.0)),
            Some(1143.492)
        );
        for unknown in ["furlongs", "c", "f", "", "mm"] {
            assert_eq!(unit_of(unknown), None, "{unknown}");
        }
    }

    #[test]
    fn decimal_commas_are_read_when_the_separator_isnt_a_comma() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);angle (deg)\n1000;20;0;1,234\n\
                    1020;20;1;1.5\n1040;20;2;1,000.5\n";
        let log = read(text).unwrap();
        let angle = channel(&log, "angle (deg)");
        assert_eq!(angle.values[..2], [1.234, 1.5]);
        assert!(angle.values[2].is_nan());
        // With the comma as the separator, a point only.
        let comma = "sep=,\ntime (ms),deltaTime (ms),baro-altitude (m),angle (deg)\n\
                     1000,20,0,1.25\n";
        let log = read(comma).unwrap();
        assert_eq!(channel(&log, "angle (deg)").values, [1.25]);
    }

    #[test]
    fn every_column_of_the_layout_is_read_as_its_table_says() {
        let log = read(&file(&[(1000, 3, 0, 150), (1020, 4, 1, 150)])).unwrap();
        let quantities: Vec<(&str, Quantity)> = log
            .channels
            .iter()
            .map(|c| (c.column.as_str(), c.quantity))
            .collect();
        assert_eq!(
            quantities,
            [
                ("status", Quantity::Number),
                ("baro-altitude (m)", Quantity::AltitudeAboveGround),
                ("dedrck-v-speed (m/s)", Quantity::Speed),
                ("angle (deg)", Quantity::Angle),
                ("roll-rate (deg/s)", Quantity::AngularRate),
                ("vert-accel (m/s2)", Quantity::Acceleration),
                ("accel (m/s2)", Quantity::Acceleration),
                ("dedrck-alti (m)", Quantity::InertialAltitude),
                ("baro-speed (m/s)", Quantity::Speed),
                ("amb-temp (deg c)", Quantity::Temperature),
                ("batt-voltage (mV)", Quantity::Voltage),
                ("P1-state", Quantity::Number),
                ("P2-state", Quantity::Number),
                ("P3-state", Quantity::Number),
                ("analog1 (mV)", Quantity::Voltage),
                ("analog2 (mV)", Quantity::Voltage),
                ("inFreefall", Quantity::Number),
                ("gpsLat", Quantity::Latitude),
                ("gpsLng", Quantity::Longitude),
                // A GPS altitude that never changes can't settle its unit.
                ("gpsAltMSL", Quantity::Unrecognized),
                ("gpsState", Quantity::Number),
                ("gpsSats", Quantity::Number),
            ]
        );
        assert_eq!(log.time_s, [1.0, 1.02]);
        assert_eq!(log.altitude_m, [0.0, 1.0]);
        assert_eq!(log.temperature_k, Some(vec![294.15, 294.15]));
        assert_eq!(log.battery_v, Some(vec![8.4, 8.4]));
        assert_eq!(log.vertical_speed_m_s, Some(vec![0.0, 0.0]));
        assert_eq!(channel(&log, "gpsLat").values, [12.345678, 12.345678]);
        assert_eq!(
            notes_starting(
                &log,
                "`gpsLat` read in degrees, from the manual and the values. "
            )
            .len(),
            1
        );
        assert_eq!(
            notes_starting(
                &log,
                "`gpsLng` read in degrees, from the manual and the values. "
            )
            .len(),
            1
        );
        assert_eq!(
            notes_starting(
                &log,
                "`gpsAltMSL` kept as written, in a unit no source states: it changes on 1 row \
                 where the barometric altitude is read. That is fewer than the 10 fresh fixes"
            )
            .len(),
            1,
            "{:?}",
            log.notes
        );
    }

    /// A flight of `rows` rows, a GPS fix on each, climbing to `apogee` m and back down in a
    /// straight line each way, the GPS altitude `150 + slope × altitude` and half a unit up and
    /// down on alternate rows, so that no two rows in a row repeat a reading.
    fn gps_line(slope: f64, apogee: f64, rows: usize) -> FlightLog {
        let mut text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);gpsAltMSL\n".to_owned();
        let last = (rows - 1) as f64;
        for row in 0..rows {
            let share = 1.0 - (2.0 * row as f64 / last - 1.0).abs();
            let altitude = apogee * share;
            let wobble = if row % 2 == 0 { 0.5 } else { -0.5 };
            let gps = 150.0 + slope * altitude + wobble;
            text.push_str(&format!(
                "{};1000;{altitude:.3};{gps:.3}\n",
                1000 * (row + 1)
            ));
        }
        read(&text).unwrap()
    }

    fn gps_quantity(log: &FlightLog) -> Quantity {
        channel(log, "gpsAltMSL").quantity
    }

    fn gps_note(log: &FlightLog) -> &String {
        let notes: Vec<&String> = log
            .notes
            .iter()
            .filter(|n| n.starts_with("`gpsAltMSL` "))
            .collect();
        assert_eq!(notes.len(), 1, "{:?}", log.notes);
        notes[0]
    }

    #[test]
    fn the_gps_altitude_is_read_in_meters_only_where_its_slope_is_near_1() {
        for (slope, meters) in [
            (1.0, true),
            (0.92, true),
            (0.88, false),
            (1.08, true),
            (1.12, false),
            // In feet, the GPS altitude rises 3.28 for each meter.
            (1.0 / FOOT_M, false),
            (0.5, false),
        ] {
            let log = gps_line(slope, 300.0, 21);
            assert_eq!(
                gps_quantity(&log) == Quantity::GnssAltitude,
                meters,
                "slope {slope}: {:?}",
                log.notes
            );
            let start = if meters {
                "`gpsAltMSL` read in meters, from the barometric altitude. "
            } else {
                "`gpsAltMSL` kept as written, in a unit no source states: at its 21 fresh fixes \
                 it rises "
            };
            assert!(gps_note(&log).starts_with(start), "{:?}", log.notes);
        }
        let log = gps_line(1.0, 300.0, 21);
        assert!(
            gps_note(&log).contains(
                "At its 21 fresh fixes it rises 1.00 for each meter of the barometric altitude, \
                 within 10% of 1. In feet it would be 3.28. They span 300 m"
            ),
            "{:?}",
            log.notes
        );
        // Read as written, in meters: 150.5 m on the pad.
        assert_eq!(channel(&log, "gpsAltMSL").values[0], 150.5);
    }

    #[test]
    fn the_gps_altitude_needs_120_m_of_span_and_10_fresh_fixes() {
        let at_floor = gps_line(1.0, 120.0, 21);
        assert_eq!(gps_quantity(&at_floor), Quantity::GnssAltitude);
        let below = gps_line(1.0, 119.0, 21);
        assert_eq!(gps_quantity(&below), Quantity::Unrecognized);
        assert!(
            gps_note(&below).contains(
                "its 21 fresh fixes span 119 m of the barometric altitude, less than the 120 m"
            ),
            "{:?}",
            below.notes
        );
        assert_eq!(
            gps_quantity(&gps_line(1.0, 300.0, 10)),
            Quantity::GnssAltitude
        );
        let few = gps_line(1.0, 300.0, 9);
        assert_eq!(gps_quantity(&few), Quantity::Unrecognized);
        assert!(
            gps_note(&few).contains("it changes on 9 rows where the barometric altitude is read"),
            "{:?}",
            few.notes
        );
    }

    /// The reviewed failure: a GPS altitude in feet that loses its fix in the climb and regains
    /// it low in the descent, its highest fixed reading 295 ft (90 m) above the pad's against a
    /// 300 m apogee. Its rise alone is within 10% of the apogee; its fresh fixes span 90 m, too
    /// little to tell the unit, and the slope over them is 3.28.
    #[test]
    fn a_feet_gps_altitude_regaining_its_fix_late_is_never_read_in_meters() {
        let mut altitude = vec![0.0, 0.0, 100.0, 200.0, 280.0, 300.0, 290.0, 250.0, 200.0];
        let mut gps = vec![
            500.0, 501.0, 501.0, 501.0, 501.0, 501.0, 501.0, 501.0, 501.0,
        ];
        for (row, height_m) in (0..=9).rev().map(|step| f64::from(step) * 10.0).enumerate() {
            altitude.push(height_m);
            gps.push((500.0 + height_m / FOOT_M + if row % 2 == 0 { 0.4 } else { -0.4 }).round());
        }
        let check = gps_check(&gps, &altitude);
        let GpsCheck::Undecided(why) = check else {
            panic!("read in meters: {check:?}");
        };
        assert!(why.contains("span 90 m"), "{why}");
        // Over a wider span, the slope settles it.
        let fixes = fresh_fixes(&gps, &altitude);
        let fit = gps_fit(&fixes).unwrap();
        assert!((fit.slope - 1.0 / FOOT_M).abs() < 0.05, "{fit:?}");
    }

    /// A receiver that holds its last fix while it has none writes the same reading on row after
    /// row: those rows, and gaps, aren't fixes. This GPS holds the pad's reading through the
    /// climb, then follows the descent in meters; drawn through every row, the line would bend
    /// toward the held reading.
    #[test]
    fn held_readings_and_gaps_are_not_fresh_fixes() {
        let mut altitude = Vec::new();
        let mut gps = Vec::new();
        for step in 0..=10 {
            altitude.push(f64::from(step) * 30.0);
            gps.push(if step == 5 { f64::NAN } else { 150.0 });
        }
        for step in (0..10).rev() {
            let height_m = f64::from(step) * 30.0 + 15.0;
            altitude.push(height_m);
            gps.push(150.0 + height_m + if step % 2 == 0 { 0.5 } else { -0.5 });
        }
        // A gap in the barometric altitude leaves its row out.
        altitude.push(f64::NAN);
        gps.push(100.0);
        let fixes = fresh_fixes(&gps, &altitude);
        assert_eq!(fixes.len(), 11, "{fixes:?}");
        assert!(
            matches!(gps_check(&gps, &altitude), GpsCheck::Meters(fit) if fit.fixes == 11),
            "{:?}",
            gps_check(&gps, &altitude)
        );
        // Every row, held readings included: the slope falls far from 1.
        let every: Vec<(f64, f64)> = altitude
            .iter()
            .zip(&gps)
            .filter(|(a, g)| a.is_finite() && g.is_finite())
            .map(|(a, g)| (*a, *g))
            .collect();
        let bent = gps_fit(&every).unwrap();
        assert!(bent.slope < 0.8, "{bent:?}");
    }

    /// A GPS altitude in feet that lags the flight by a first-order filter of 10 s: its slope
    /// against the barometric altitude comes out near 1, but its fixes trace a loop that scatters
    /// far more than 10% of the span about the line, so it is kept.
    #[test]
    fn a_lagging_gps_altitude_whose_slope_is_near_1_is_kept_by_its_scatter() {
        let (step_s, tau_s) = (0.02, 10.0);
        let (mut time_s, mut lagged) = (0.0_f64, 0.0_f64);
        let (mut altitude, mut gps) = (Vec::new(), Vec::new());
        let mut next_fix_s = 0.0;
        while time_s < 53.0 {
            let height_m = if time_s < 3.0 {
                300.0 * (std::f64::consts::FRAC_PI_2 * time_s / 3.0).sin()
            } else {
                (300.0 - 15.0 * (time_s - 3.0)).max(0.0)
            };
            lagged += (height_m / FOOT_M - lagged) * step_s / tau_s;
            if time_s >= next_fix_s {
                altitude.push(height_m);
                gps.push(lagged);
                next_fix_s += 1.0;
            }
            time_s += step_s;
        }
        let fit = gps_fit(&fresh_fixes(&gps, &altitude)).unwrap();
        assert!(
            (fit.slope - 1.0).abs() <= GPS_SLOPE_MARGIN,
            "the slope alone would pass: {fit:?}"
        );
        assert!(
            fit.scatter > 3.0 * GPS_SCATTER_SHARE * fit.span_m,
            "{fit:?}"
        );
        let check = gps_check(&gps, &altitude);
        let GpsCheck::Undecided(why) = check else {
            panic!("read in meters: {check:?}");
        };
        assert!(why.contains("They trace a loop"), "{why}");
    }

    #[test]
    fn no_gps_altitude_or_no_barometric_altitude_decides_nothing() {
        let nan = vec![f64::NAN; 20];
        let line: Vec<f64> = (0..20).map(|row| f64::from(row) * 20.0).collect();
        for (gps, altitude) in [(&nan, &line), (&line, &nan), (&nan, &nan)] {
            assert!(
                matches!(gps_check(gps, altitude), GpsCheck::Undecided(ref why)
                    if why.starts_with("the file has no GPS altitude")),
                "{:?}",
                gps_check(gps, altitude)
            );
        }
    }

    #[test]
    fn a_gps_altitude_with_a_stated_unit_is_read_in_it() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);gpsAltMSL (ft)\n\
                    1000;20;0;1000\n1020;20;5;1000\n";
        let log = read(text).unwrap();
        let gps = channel(&log, "gpsAltMSL (ft)");
        assert_eq!(gps.quantity, Quantity::GnssAltitude);
        assert!((gps.values[0] - 304.8).abs() < 1e-9);
        assert!(
            log.notes.iter().all(|n| !n.contains("gpsAltMSL")),
            "{:?}",
            log.notes
        );
    }

    #[test]
    fn a_position_beyond_its_range_is_kept() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);gpsLat;gpsLng\n\
                    1000;20;0;90;180\n1020;20;0;-90;-180\n";
        let log = read(text).unwrap();
        assert_eq!(channel(&log, "gpsLat").quantity, Quantity::Latitude);
        assert_eq!(channel(&log, "gpsLng").quantity, Quantity::Longitude);
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);gpsLat;gpsLng\n\
                    1000;20;0;90,5;180\n1020;20;0;0;-180,5\n";
        let log = read(text).unwrap();
        assert_eq!(channel(&log, "gpsLat").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "gpsLng").quantity, Quantity::Unrecognized);
        assert_eq!(
            notes_starting(
                &log,
                "`gpsLat` kept as written, in a unit the values don't bear out. "
            )
            .len(),
            1
        );
        assert!(
            notes_starting(
                &log,
                "`gpsLng` kept as written, in a unit the values don't bear out. "
            )[0]
            .contains("-180.5")
        );
    }

    #[test]
    fn units_of_the_wrong_kind_or_unknown_keep_a_column_as_written() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (ft);angle (m/s);roll-rate \
                    (furlongs);amb-temp;status (s);pitot (kPa);mystery (m);other\n\
                    1000;20;100;1;2;3;4;5;6;7\n";
        let log = read(text).unwrap();
        assert!((log.altitude_m[0] - 30.48).abs() < 1e-9);
        let kept = |column: &str, why: &str| {
            assert_eq!(
                channel(&log, column).quantity,
                Quantity::Unrecognized,
                "{column}"
            );
            let note = format!("`{column}` kept as written: {why}");
            assert!(log.notes.contains(&note), "{note}: {:?}", log.notes);
        };
        kept(
            "angle (m/s)",
            "its unit is a speed, not what the column holds",
        );
        kept(
            "roll-rate (furlongs)",
            "its unit, \"furlongs\", isn't one this reader knows",
        );
        kept("amb-temp", "the header states no unit for it");
        kept(
            "status (s)",
            "its unit is a time, not what the column holds",
        );
        kept(
            "mystery (m)",
            "this reader doesn't know the column, and its unit, a length, could be more than one \
             quantity",
        );
        kept(
            "other",
            "this reader doesn't know the column, and the header states no unit",
        );
        // A column the reader doesn't know, read by a unit that means one quantity only.
        let pitot = channel(&log, "pitot (kPa)");
        assert_eq!(
            (pitot.quantity, pitot.values[0]),
            (Quantity::Pressure, 5000.0)
        );
        // An unknown temperature column isn't the record's temperature.
        assert_eq!(log.temperature_k, None);
    }

    /// A position column of gaps alone bears out no unit: kept, with no claim about its readings.
    #[test]
    fn a_position_of_gaps_alone_is_kept() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);gpsLat;gpsLng\n\
                    1000;20;0;nan;1,5\n1020;20;0;;2,5\n";
        let log = read(text).unwrap();
        assert_eq!(channel(&log, "gpsLat").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "gpsLng").quantity, Quantity::Longitude);
        assert_eq!(
            notes_starting(
                &log,
                "`gpsLat` kept as written, in a unit no reading bears out: the header states \
                 none, and the column holds no reading"
            )
            .len(),
            1,
            "{:?}",
            log.notes
        );
        assert!(
            log.notes
                .iter()
                .all(|n| !n.contains("every reading lies within ±90°"))
        );
        assert_eq!(degrees(&[f64::NAN, f64::NAN], 90.0), Degrees::NoReading);
        assert_eq!(degrees(&[], 90.0), Degrees::NoReading);
        assert_eq!(degrees(&[f64::NAN, 45.0], 90.0), Degrees::Within);
        assert_eq!(degrees(&[f64::NAN, -91.0], 90.0), Degrees::Beyond(-91.0));
    }

    /// A reading a conversion carries past the largest `f64` is refused on its line, never an
    /// infinity nor a gap (ADR-258 §3): `deg f` multiplies by 5 before dividing by 9, `hPa`
    /// multiplies by 100 and `g` by 9.806 65.
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        for (unit, written) in [("deg f", 1e308), ("hPa", 1e307), ("kPa", -1e306)] {
            let converted = unit_of(unit).map(|u| u.convert(written));
            assert!(
                converted.is_some_and(f64::is_infinite),
                "{unit}: {converted:?}"
            );
        }
        let header = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);amb-temp (deg f);\
                      pitot (hPa);accel (g)\n";
        let cases = [
            ("1e308;1013;1", 3, "amb-temp (deg f)"),
            ("70;1.7e308;1", 3, "pitot (hPa)"),
            ("70;1013;-1.7e308", 3, "accel (g)"),
        ];
        for (cells, line, column) in cases {
            let text = format!("{header}1000;20;0;{cells}\n1020;20;0;70;1013;1\n");
            assert_eq!(
                read(&text).unwrap_err().to_string(),
                format!(
                    "Silicdyne Fluctus CSV line {line}: the column {column} holds a value too \
                     large for its unit's conversion to SI"
                )
            );
        }
        // On a later row, the refusal names that row's line.
        let text = format!("{header}1000;20;0;70;1013;1\n1020;20;0;70;1013;1.7e308\n");
        assert_eq!(
            read(&text).unwrap_err().to_string(),
            "Silicdyne Fluctus CSV line 4: the column accel (g) holds a value too large for its \
             unit's conversion to SI"
        );
    }

    #[test]
    fn a_clock_or_altitude_without_its_unit_is_refused() {
        for (text, what) in [
            (
                "sep=;\ntime;deltaTime (ms);baro-altitude (m)\n1;20;0\n",
                "`time`",
            ),
            (
                "sep=;\ntime (m);deltaTime (ms);baro-altitude (m)\n1;20;0\n",
                "`time`",
            ),
            (
                "sep=;\ntime (ms);deltaTime (ms);baro-altitude (s)\n1;20;0\n",
                "`baro-altitude`",
            ),
            (
                "sep=;\ntime (ms);deltaTime (ms);baro-altitude\n1;20;0\n",
                "`baro-altitude`",
            ),
        ] {
            match read(text) {
                Err(LogError::Unit {
                    line: 2, message, ..
                }) => {
                    assert!(message.contains(what), "{message}");
                }
                other => panic!("{text}: {other:?}"),
            }
        }
    }

    #[test]
    fn placeholders_and_cells_that_arent_numbers_are_gaps() {
        let text = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);dedrck-alti (m);angle \
                    (deg)\n1000;20;0;2147483647;nan\n1020;20;1;-2147483648;\n\
                    1040;20;2;2147483646;1,5\n";
        let log = read(text).unwrap();
        let inertial = channel(&log, "dedrck-alti (m)");
        assert!(inertial.values[0].is_nan() && inertial.values[1].is_nan());
        // One short of the placeholder is a reading.
        assert_eq!(inertial.values[2], 2_147_483_646.0);
        let angle = channel(&log, "angle (deg)");
        assert!(angle.values[0].is_nan() && angle.values[1].is_nan());
        assert_eq!(angle.values[2], 1.5);
        assert!(log.notes.contains(
            &"the column `dedrck-alti (m)` writes 2147483647 or -2147483648, the ends of a signed \
              32-bit number, on 2 rows from line 3: no reading, so they are gaps"
                .to_owned()
        ));
        assert!(
            log.notes.contains(
                &"the column `angle (deg)` holds 2 cells that are empty or not a number, such as \
              \"nan\": they are gaps"
                    .to_owned()
            )
        );
    }

    #[test]
    fn status_changes_and_fired_outputs_are_events() {
        let text = "sep=;\ntime (ms);deltaTime (ms);status;baro-altitude (m);P1-state;P2-state\n\
                    1000;20;3;0;1;1\n1020;20;4;5;1;0\n1040;20;;9;10;1\n1060;20;5;9;10;10\n\
                    1080;20;6;0;1;0\n1100;20;9;0;10;0\n";
        let log = read(text).unwrap();
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [
                (1.02, "ASCENT"),
                (1.04, "P1-state ENABLED"),
                (1.06, "DESCENT"),
                (1.06, "P2-state ENABLED"),
                (1.08, "TOUCHDOWN"),
                (1.1, "status 9"),
                (1.1, "P1-state ENABLED"),
            ]
        );
        let names: Vec<String> = (0..=6).map(|code| status_name(f64::from(code))).collect();
        assert_eq!(names, STATUS_NAMES.map(str::to_owned));
        assert_eq!(status_name(2.5), "status 2.5");
        assert_eq!(status_name(-1.0), "status -1");
    }

    #[test]
    fn delta_time_is_checked_against_the_clock() {
        let agreeing = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m)\n1000;20;0\n1020;20;0\n\
                        1088;68;0\n1108;20;0\n";
        let log = read(agreeing).unwrap();
        assert!(
            log.channels
                .iter()
                .all(|c| !c.column.starts_with("deltaTime"))
        );
        assert!(
            log.notes.iter().all(|n| !n.contains("differs")),
            "{:?}",
            log.notes
        );
        assert_eq!(
            log.notes,
            [
                "the clock steps by 20 ms on most rows, and by more once: the longest, 68 ms, before \
              line 5; the logger wrote no row between"
            ]
        );
        let differing = agreeing.replace("1088;68", "1088;20");
        let log = read(&differing).unwrap();
        assert!(log.notes.contains(
            &"`deltaTime (ms)` differs from the clock's step on 1 row, the first on line 5: the \
              clock is kept, and `deltaTime (ms)` isn't"
                .to_owned()
        ));
    }

    #[test]
    fn malformed_files_are_refused_naming_the_line() {
        let head = "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m)\n";
        for (rows, line) in [
            ("1000;20\n", 3),
            ("1000;20;0;1\n", 3),
            ("1000;20;0\n1000;20;0\n", 4),
            ("1000;20;0\n990;20;0\n", 4),
            ("1000;20;0\nnan;20;0\n", 4),
            ("1000;20;0\n;20;0\n", 4),
        ] {
            match read(&format!("{head}{rows}")) {
                Err(LogError::Syntax { line: found, .. }) => assert_eq!(found, line, "{rows}"),
                other => panic!("{rows}: {other:?}"),
            }
        }
        assert_eq!(read(head), Err(LogError::NoData { format: FORMAT }));
        assert!(matches!(
            read("sep=;\ntime (ms);deltaTime (ms);baro-altitude (m);deltatime\n"),
            Err(LogError::Syntax { line: 2, .. })
        ));
        assert!(matches!(
            read("sep=;\ntime (ms);deltaTime (ms);;baro-altitude (m)\n"),
            Err(LogError::Syntax { line: 2, .. })
        ));
        let wide = format!(
            "sep=;\ntime (ms);deltaTime (ms);baro-altitude (m){}\n",
            (0..MAX_COLUMNS)
                .map(|i| format!(";c{i}"))
                .collect::<String>()
        );
        assert!(matches!(read(&wide), Err(LogError::Syntax { line: 2, .. })));
        assert!(matches!(
            read("Time,Altitude\n0,0\n"),
            Err(LogError::NotThisFormat { .. })
        ));
    }

    #[test]
    fn crlf_blank_lines_and_a_byte_order_mark_are_read() {
        let text = "\u{feff}sep=;\r\ntime (ms);deltaTime (ms);baro-altitude (m)\r\n1000;20;0\r\n\
                    \r\n1020;20;3\r\n";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [1.0, 1.02]);
        assert_eq!(log.altitude_m, [0.0, 3.0]);
    }

    #[test]
    fn the_resolution_is_the_whole_meter() {
        assert_eq!(ALTITUDE_STEP_M, 1.0);
        assert_eq!(LogFormat::FluctusCsv.altitude_resolution_m(), 1.0);
    }
}
