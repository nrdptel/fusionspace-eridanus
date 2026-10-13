//! Rocketry Ltd's flight logs as CSV, as a flight page on altimetercloud.com hands them back: a
//! Nano V1's, or a Mercury's.
//!
//! Rocketry Ltd's altimeters, the Nano V1, the Mercury V1 and Mercury Plus, and the Jupiter, upload
//! their flights to AltimeterCloud, which shows each on a flight page and gives back its log as a
//! CSV file. Every one has a Bosch BMP581 barometer, and the Nano V1 and Mercury V1 a ±32 g
//! accelerometer and a ±2,000 °/s gyroscope (*Altimeters*,
//! <https://www.altimetercloud.com/altimeters/>, read 2026-10-13). Two layouts are in the
//! project's sample logs. A Nano V1's opens with a flight summary of keys and values, then a blank
//! line, then its header; its columns are those of the Nano V1 manual's page *CSV Flight log
//! format* (<https://www.altimetercloud.com/support/nano-v1/flight-log-format/>, read 2026-10-13):
//!
//! ```text
//! Apogee meters,120.50,,Landing meters,-0.40,Burnout time (ms),900,
//! Apogee time,5.120,,Landing time,30.000,Burnout velocity (m/s),40.000,
//! …
//!
//! Time(ms),Altitude(m),Velocity(m/s),acceleration_x(mG),…,Board temp(C),pitch,roll,yaw,tilt
//! -2000,0.010,0.00,0,0,0,-1000.0,0,0,0,,0.00,0.00,0.00,0.00,
//! ```
//!
//! A Mercury's opens straight on its header, in lower case, with the channels the Mercury V1
//! manual says it logs, "temperature, pressure, air density, predicted apogee and air brake
//! output" beside the Nano's (*Mercury V1 manual*,
//! <https://www.altimetercloud.com/support/mercuryv1/>, read 2026-10-13):
//!
//! ```text
//! time(ms),altitude(m),velocity_pressure(m/s),acceleration_x(mG),…,bmp_temp(x100),pitch,roll,
//! yaw,tilt,mt1_temp,apogee_prediction,air_brake_percent,target_alt,air_density,
//! ```
//!
//! The logger is named from the header: a Rocketry Ltd Mercury if it has a column only the
//! Mercury's layout has (`velocity_pressure(m/s)`, `bmp_temp(x100)`, `mt1_temp`,
//! `apogee_prediction`, `air_brake_percent`, `target_alt` or `air_density`), else a Rocketry Ltd
//! Nano V1, whose manual describes the other layout; a note on the record says which.
//!
//! **Recognition.** Within the file's first 65 lines, a header whose first name is `Time(ms)`, in
//! any case, and which names `acceleration_total(mG)` and `tilt` too: following Debrief's reader
//! (`lib/parsers/altimeterCloud.ts`, MIT, the project owner's own; see `THIRD-PARTY-NOTICES.md`),
//! those two beside a millisecond clock are this logger's own names, and no other layout among
//! the project's sample logs has them. The lines above the header are the summary.
//!
//! **Columns.** Every column is a channel; a column the reader doesn't know is kept as written,
//! [`Quantity::Unrecognized`], with a note. Each column holds only the rows that write it: the Nano
//! writes its board temperature on every 50th row and leaves it blank between (Nano manual), and
//! the Mercury's slower columns are as sparse, so a channel's clock is the times of its own rows.
//! The primary track's temperature is on the track's clock, a gap where the row has none.
//!
//! | Column | Read as | From |
//! |---|---|---|
//! | `Time(ms)`, `time(ms)` | the clock: milliseconds from lift-off, before it negative | the header; Nano manual ("time from lift-off, with lift-off as zero") |
//! | `Altitude(m)`, `altitude(m)` | [`Quantity::AltitudeAboveGround`], m: the primary track | the header; Nano manual: the height above the pad after the logger's TrueFuse and TruePath processing |
//! | `Velocity(m/s)`, `velocity_pressure(m/s)` | [`Quantity::Speed`], m/s, up positive: the primary track's vertical speed | the header; Nano manual: the vertical speed of the processed altitude; the Mercury's, by its name, the barometer's |
//! | `acceleration_x(mG)`, `_y`, `_z` | [`Quantity::Acceleration`], thousandths of standard gravity, 9.806 65 m/s² | the header; Nano manual: along each of the board's axes |
//! | `acceleration_total(mG)` | [`Quantity::Acceleration`], as the axes, but the acceleration's magnitude less 1 g, −1 g in free fall, along no axis; the record has no quantity for that, and a note says what it is | the header; Nano manual ("with the 1 g of gravity taken out, so it sits near zero at rest") |
//! | `gyro_x`, `gyro_y`, `gyro_z` | [`Quantity::AngularRate`], °/s | the Nano manual; a note says so |
//! | `Board temp(C)` | [`Quantity::Temperature`], °C | the header; Nano manual |
//! | `bmp_temp(x100)` | [`Quantity::Temperature`], hundredths of a °C, when the values bear it out (below) | the name; Debrief's reader; a note says how |
//! | `pitch`, `roll`, `yaw`, `tilt` | [`Quantity::Angle`], degrees | the Nano manual (attitude angles from their zero on the pad, and the tilt from vertical); a note says so |
//! | `Pressure(hPa)` | [`Quantity::Pressure`], hPa × 100 Pa | the header; Nano manual |
//! | `Raw altitude(m)` | [`Quantity::AltitudeAboveGround`], m: the barometer's altitude before processing | the header; Nano manual |
//! | `air_brake_percent` | [`Quantity::Number`], the percentage as written | the name; Mercury manual ("air brake output"); a note says so |
//! | `mt1_temp`, `apogee_prediction`, `target_alt`, `air_density` | [`Quantity::Unrecognized`], as written | no source states their units; a prediction and a setting aren't readings, and the record has no quantity for a density; a note each |
//!
//! The Mercury's manual describes none of its columns, so where they share the Nano's names, the
//! Nano's manual is taken for them too, and the notes say so.
//!
//! *The centi-degree temperature.* `bmp_temp(x100)` holds the BMP581's temperature times 100, as
//! its name says and Debrief's reader takes it: 2150 for 21.50 °C, a whole number in every one of
//! the 4 Mercury files among the project's sample logs as of 2026-10-13. It is read so if every
//! reading is a whole number, every one divided by 100 lies within the −40 to +85 °C the BMP581
//! measures, and at least one as written lies beyond it, so that the column can't be in degrees
//! (Bosch Sensortec, *BMP581 data sheet*, BST-BMP581-DS004-13, revision 1.13, April 2025, table 2,
//! p. 10: the temperature measurement range, −40 to 85 °C). Otherwise it is kept as written,
//! [`Quantity::Unrecognized`], with a note.
//!
//! **Placeholders.** Two kinds of cell, as of 2026-10-13 in every one of the 11 files among the
//! project's sample logs, hold a number that isn't a reading; each is a gap, counted in a note:
//!
//! - *The accelerometer before it reads.* The first rows, all before lift-off, write 0 on all three
//!   axes (and −1000 mG for the total, a magnitude of zero less 1 g), where an accelerometer at
//!   rest reads 1 g. On the leading rows with a negative time whose three axes are all exactly 0,
//!   the axes and the total are gaps. A row at or after lift-off is kept as written: in free fall
//!   an accelerometer does read about zero.
//! - *The temperature after it stops.* The last readings of the board temperature are exactly 0,
//!   where the reading before was 27 to 44 °C. A run of exactly 0 that ends the column is a gap if
//!   the reading before it lies more than 5 °C from 0: a board can't cool that far between two
//!   readings and then hold at exactly 0.00 °C. On a day near freezing the zeros are kept.
//!
//! The yaw is kept as written, but a note counts the rows where it reaches ±327.67°: as of
//! 2026-10-13, in no sample log does the yaw pass ±327.68°, ±32,768 hundredths of a degree, the
//! range of a signed 16-bit number, though in all 11 it comes within 0.2° of it and jumps by more
//! than 300° between two rows 22 to 121 times, so a yaw beyond that range isn't written as it
//! was: it is clipped, or wrapped to the other end.
//!
//! **The summary.** The lines above the header are keys each followed by its value, groups of them
//! parted by an empty cell, as `Apogee meters,120.50,,Landing meters,-0.40,…`; a key with no value
//! is followed by an empty cell. The reader keeps:
//!
//! - `Apogee meters` as the stated apogee ([`Stated::apogee_m`]), m, as the key says;
//! - `Serial number` and `Firmware`;
//! - `Burnout time (ms)` and `Ejection time (ms)` as events, `Burnout` and `Ejection`;
//! - `Apogee time` as an event, `Apogee`, in seconds if the altitude bears it out: if the highest
//!   altitude's time lies within 0.5 s of it read in seconds. The key states no unit; as of
//!   2026-10-13, in the 7 files among the project's sample logs with a summary, it lies within
//!   0.01 s of the highest altitude's time read in seconds, and a thousand times off read in
//!   milliseconds;
//! - `Landing time` as an event, `Landing`, in seconds if the altitude bears it out: if, read in
//!   seconds, it comes after the highest altitude and the altitude there (on the line between the
//!   rows either side, if no row is at that time) lies within 0.01 m of the summary's `Landing
//!   meters`, and read in milliseconds it doesn't do both. The Nano manual's page *Flight info
//!   page* (<https://www.altimetercloud.com/support/nano-v1/flight-info-page/>, read 2026-10-13)
//!   gives the landing as "when it landed, and the height it came to rest at", but no unit for
//!   either. Both are written to the millimeter, and the margin is ten times that; as of
//!   2026-10-13, in the 7 files among the project's sample logs with a summary, the altitude at the
//!   landing time read in seconds lies within 0.001 m of the landing height, and read in
//!   milliseconds, on the climb, more than a meter from it. Without a landing height to check, the
//!   landing time is left out.
//!
//!   A note says how each time was read, or why it was left out;
//! - `Adjusted log`: a value other than `no` gets a note, as AltimeterCloud can hand back a log
//!   recalculated after the flight with another pressure or temperature (Nano manual: the key
//!   "reads 'no' for an original flight").
//!
//! The events are in time order, as a summary has no order of its own. The other entries, such as
//! the settings the logger flew with, are counted in a note and left out.
//!
//! **Rows.** Every data row ends in a comma (Nano manual), so a row holds one more cell than the
//! header names, an empty one; the Mercury's header ends in a comma too, its last name empty, and
//! that name is dropped. A row as wide as the header, or one wider with the last cell empty, is
//! read cell for cell; a row of another width, a cell that isn't a number, a row with no time or a
//! time that doesn't increase is refused, naming its line. Blank lines are skipped. Lines may end
//! in CR, LF or both, and a byte-order mark is dropped. As every row must be as wide as the
//! header, a short file can't make the reader store more cells than it writes.
//!
//! **Where this comes from.** The recognition and the centi-degree temperature follow Debrief's
//! reader; the columns' units, the clock's zero and the placeholders' rules are the Nano manual's
//! and the files'; the summary's times are read as the altitude bears them out. Where this reader departs from Debrief's: it keeps every column, where Debrief
//! leaves the gyroscope, the attitude angles and the Mercury's prediction and air-brake columns out;
//! it keeps the summary's apogee, serial number and events; and it refuses what Debrief passes over.

use std::collections::HashSet;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, Notes, finite, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "AltimeterCloud CSV";

/// The most lines, the summary's and blank ones, above the header.
const MAX_SUMMARY_LINES: usize = 64;

/// Standard gravity, m/s² (CGPM 1901): a thousandth of it is one `mG`.
const G0_M_S2: f64 = 9.806_65;

/// The temperatures the BMP581 measures, °C (module docs).
const BMP581_RANGE_C: (f64, f64) = (-40.0, 85.0);

/// How far, °C, from 0 the reading before a closing run of zeros must lie for the zeros to be
/// gaps (module docs).
const TEMPERATURE_JUMP_C: f64 = 5.0;

/// How far, s, the stated apogee time may lie from the highest altitude's for it to be read in
/// seconds (module docs).
const APOGEE_TIME_MARGIN_S: f64 = 0.5;

/// How far, m, the altitude at the stated landing time may lie from the stated landing height
/// for the time to be read so (module docs): ten times the millimeter both are written to.
const LANDING_HEIGHT_MARGIN_M: f64 = 0.01;

/// The yaw, degrees, from which a note counts the rows as at the end of its range (module docs).
const YAW_END_DEG: f64 = 327.67;

/// What a header column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Time,
    Altitude,
    Velocity,
    /// One of the accelerometer's three axes.
    Axis,
    /// The acceleration's magnitude less 1 g.
    Total,
    Gyro,
    BoardTemperature,
    CentiTemperature,
    /// Pitch, roll or tilt.
    Angle,
    Yaw,
    Pressure,
    RawAltitude,
    AirBrake,
    /// A column the reader knows but keeps as written; the words say why.
    Kept(&'static str),
    Unknown,
}

/// The names the reader knows, in lower case, and what each holds.
const KNOWN: [(&str, Kind); 23] = [
    ("time(ms)", Kind::Time),
    ("altitude(m)", Kind::Altitude),
    ("velocity(m/s)", Kind::Velocity),
    ("velocity_pressure(m/s)", Kind::Velocity),
    ("acceleration_x(mg)", Kind::Axis),
    ("acceleration_y(mg)", Kind::Axis),
    ("acceleration_z(mg)", Kind::Axis),
    ("acceleration_total(mg)", Kind::Total),
    ("gyro_x", Kind::Gyro),
    ("gyro_y", Kind::Gyro),
    ("gyro_z", Kind::Gyro),
    ("board temp(c)", Kind::BoardTemperature),
    ("bmp_temp(x100)", Kind::CentiTemperature),
    ("pitch", Kind::Angle),
    ("roll", Kind::Angle),
    ("yaw", Kind::Yaw),
    ("tilt", Kind::Angle),
    ("pressure(hpa)", Kind::Pressure),
    ("raw altitude(m)", Kind::RawAltitude),
    ("air_brake_percent", Kind::AirBrake),
    (
        "mt1_temp",
        Kind::Kept("by its name a temperature, but no source states its unit or what it measures"),
    ),
    (
        "apogee_prediction",
        Kind::Kept(
            "the Mercury's predicted apogee (Mercury V1 manual), a prediction rather than a \
             reading, in a unit no source states",
        ),
    ),
    (
        "target_alt",
        Kind::Kept(
            "by its name the altitude the air brakes aim for, a setting rather than a reading, in \
             a unit no source states",
        ),
    ),
];

/// The names only a Mercury's layout has, in lower case (module docs).
const MERCURY_ONLY: [&str; 7] = [
    "velocity_pressure(m/s)",
    "bmp_temp(x100)",
    "mt1_temp",
    "apogee_prediction",
    "air_brake_percent",
    "target_alt",
    "air_density",
];

/// What a column named so holds.
fn kind_of(name: &str) -> Kind {
    if name.eq_ignore_ascii_case("air_density") {
        return Kind::Kept(
            "the Mercury logs the air's density (Mercury V1 manual), in a unit no source states, \
             and the record has no quantity for a density",
        );
    }
    KNOWN
        .iter()
        .find(|(known, _)| name.eq_ignore_ascii_case(known))
        .map_or(Kind::Unknown, |(_, kind)| *kind)
}

/// A header column: what it holds and its name as written.
struct Column {
    kind: Kind,
    written: String,
}

/// Whether a line is this layout's header (module docs, *Recognition*). The cells are looked at
/// one by one, never collected, so a long line costs no memory.
fn is_header(line: &str) -> bool {
    let mut cells = line.split(',').map(str::trim);
    if !cells
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case("time(ms)"))
    {
        return false;
    }
    let (mut total, mut tilt) = (false, false);
    for cell in cells {
        total |= cell.eq_ignore_ascii_case("acceleration_total(mg)");
        tilt |= cell.eq_ignore_ascii_case("tilt");
        if total && tilt {
            return true;
        }
    }
    false
}

/// Lines of the file, each with its number from 1.
type Lines<'a> = Vec<(usize, &'a str)>;

/// The header's line number and text, and the lines above it, if one is within the first
/// [`MAX_SUMMARY_LINES`] + 1 lines.
fn find_header(text: &str) -> Option<(usize, &str, Lines<'_>)> {
    let mut above = Vec::new();
    for (number, line) in numbered_lines(text).take(MAX_SUMMARY_LINES + 1) {
        if is_header(line) {
            return Some((number, line, above));
        }
        above.push((number, line));
    }
    None
}

/// Whether the text is in this reader's layout, judged from its header row's names, never the
/// file's name.
pub(crate) fn recognizes(text: &str) -> bool {
    find_header(text).is_some()
}

/// A summary entry: its line, its key and its value, both trimmed.
struct Entry<'a> {
    line: usize,
    key: &'a str,
    value: &'a str,
}

/// The summary's entries: on each line, a key and the cell after it, an empty cell between
/// groups skipped where a key would be (module docs, *The summary*). An iterator, made afresh for
/// each look-up, so that a long summary line costs no memory.
fn summary_entries<'a>(lines: &'a [(usize, &'a str)]) -> impl Iterator<Item = Entry<'a>> + 'a {
    lines.iter().flat_map(|&(line, text)| {
        let mut cells = text.split(',').map(str::trim);
        std::iter::from_fn(move || {
            loop {
                let key = cells.next()?;
                if !key.is_empty() {
                    return Some(Entry {
                        line,
                        key,
                        value: cells.next().unwrap_or_default(),
                    });
                }
            }
        })
    })
}

/// The rows read: each row's line and time as written, ms, and each column's cells, `NaN` where
/// the row leaves the cell empty.
struct Rows {
    lines: Vec<usize>,
    time_ms: Vec<f64>,
    /// Per header column; empty for the time column.
    values: Vec<Vec<f64>>,
}

/// Reads a Rocketry Ltd log as AltimeterCloud's CSV writes it, from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if no line within the first 65 is this layout's header;
/// [`LogError::Syntax`] for a header naming a column twice or leaving a name empty, a row of the
/// wrong width, a cell that isn't a number or is too large to convert to SI, or a time that is
/// missing or doesn't increase, naming the line; [`LogError::NoData`] if no row follows the
/// header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let Some((header_line, header, above)) = find_header(text) else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "none of its first {} lines is an AltimeterCloud header: `Time(ms)` first, and \
                 `acceleration_total(mG)` and `tilt` among the names",
                MAX_SUMMARY_LINES + 1
            ),
        });
    };
    let columns = columns(header, header_line)?;
    let rows = read_rows(
        numbered_lines(text).skip_while(|(number, _)| *number <= header_line),
        &columns,
    )?;
    if rows.time_ms.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    build(&columns, rows, &above)
}

/// The header's columns; a name given twice, or left empty, is refused. A last empty name, the
/// line's closing comma, is dropped. The names are read one by one, so a header refused for an
/// empty name costs no memory for the names after it.
fn columns(header: &str, line: usize) -> Result<Vec<Column>, LogError> {
    let mut columns: Vec<Column> = Vec::new();
    // The names met, in lower case: a set, so that a header of many names isn't checked in
    // quadratic time.
    let mut seen: HashSet<String> = HashSet::new();
    let mut cells = header.split(',').map(str::trim).enumerate().peekable();
    while let Some((index, cell)) = cells.next() {
        if cell.is_empty() {
            if index > 0 && cells.peek().is_none() {
                break;
            }
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if !seen.insert(cell.to_ascii_lowercase()) {
            return Err(syntax(
                line,
                format!("the header names {:?} twice", short(cell)),
            ));
        }
        columns.push(Column {
            kind: kind_of(cell),
            written: cell.to_owned(),
        });
    }
    if !columns.iter().any(|column| column.kind == Kind::Altitude) {
        return Err(syntax(
            line,
            "the header names no altitude, `Altitude(m)`".to_owned(),
        ));
    }
    Ok(columns)
}

/// Reads the data rows (module docs, *Rows*).
fn read_rows<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
    columns: &[Column],
) -> Result<Rows, LogError> {
    let width = columns.len();
    let mut rows = Rows {
        lines: Vec::new(),
        time_ms: Vec::new(),
        values: columns.iter().map(|_| Vec::new()).collect(),
    };
    for (number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        // One more than a closing comma allows, so a wider row is still told apart.
        let cells: Vec<&str> = line.split(',').map(str::trim).take(width + 2).collect();
        let closing = cells.len() == width + 1 && cells.last().is_some_and(|c| c.is_empty());
        if cells.len() != width && !closing {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells: the header names {width}, and a row may add one empty \
                     cell after them, its closing comma",
                    if cells.len() > width + 1 {
                        format!("more than {}", width + 1)
                    } else {
                        cells.len().to_string()
                    }
                ),
            ));
        }
        let time_ms = match cells.first() {
            Some(cell) if !cell.is_empty() => number_in(cell, number, &columns[0].written)?,
            _ => return Err(syntax(number, "the row has no time".to_owned())),
        };
        if let Some(&last) = rows.time_ms.last()
            && time_ms <= last
        {
            return Err(syntax(
                number,
                format!("the time {time_ms} ms doesn't come after the one above it, {last} ms"),
            ));
        }
        for (index, (column, cell)) in columns.iter().zip(&cells).enumerate().skip(1) {
            let value = if cell.is_empty() {
                f64::NAN
            } else {
                number_in(cell, number, &column.written)?
            };
            if let Some(store) = rows.values.get_mut(index) {
                store.push(value);
            }
        }
        rows.lines.push(number);
        rows.time_ms.push(time_ms);
    }
    Ok(rows)
}

/// A cell as a number, or the error naming its line and column.
fn number_in(cell: &str, line: usize, column: &str) -> Result<f64, LogError> {
    table::number(cell, false).ok_or_else(|| {
        syntax(
            line,
            format!("{:?} under {:?} isn't a number", short(cell), short(column)),
        )
    })
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// The names of the columns of a kind, as a note lists them: `` `a`, `b` and `c` ``.
fn names_of(columns: &[Column], kinds: &[Kind]) -> Option<String> {
    let names: Vec<String> = columns
        .iter()
        .filter(|column| kinds.contains(&column.kind))
        .map(|column| format!("`{}`", short(&column.written)))
        .collect();
    match names.as_slice() {
        [] => None,
        [one] => Some(one.clone()),
        [rest @ .., last] => Some(format!("{} and {last}", rest.join(", "))),
    }
}

/// The record from the rows read: each column's unit, the placeholders, the summary and the
/// notes.
///
/// # Errors
///
/// [`LogError::Syntax`] for a value that its unit's conversion makes infinite.
fn build(
    columns: &[Column],
    rows: Rows,
    summary_lines: &[(usize, &str)],
) -> Result<FlightLog, LogError> {
    let mercury = columns.iter().any(|column| {
        MERCURY_ONLY
            .iter()
            .any(|name| column.written.eq_ignore_ascii_case(name))
    });
    let time_s: Vec<f64> = rows.time_ms.iter().map(|ms| ms / 1000.0).collect();
    let mut log = FlightLog {
        format: LogFormat::AltimeterCloudCsv,
        logger: if mercury {
            "Rocketry Ltd Mercury"
        } else {
            "Rocketry Ltd Nano V1"
        }
        .to_owned(),
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
    log.notes.push(if mercury {
        "the logger is taken as a Rocketry Ltd Mercury: its header has columns only the \
         Mercury's layout has, such as the predicted apogee and the air-brake output its manual \
         lists"
            .to_owned()
    } else {
        "the logger is taken as a Rocketry Ltd Nano V1: its header is the layout the Nano V1 \
         manual's page *CSV Flight log format* describes"
            .to_owned()
    });
    let mut line_notes = Notes::default();
    let manual = if mercury {
        "the Nano V1 manual, whose layout has the same columns"
    } else {
        "the Nano V1 manual"
    };
    if let Some(names) = names_of(columns, &[Kind::Gyro]) {
        log.notes.push(format!(
            "{names} read in degrees per second, from {manual}: its page *CSV Flight log format* \
             gives each as the rate of turn about one of the board's axes in °/s, as whole \
             numbers; the altimeter's gyroscope reads up to ±2,000 °/s"
        ));
    }
    if let Some(names) = names_of(columns, &[Kind::Angle, Kind::Yaw]) {
        log.notes.push(format!(
            "{names} read in degrees, from {manual}: its page *CSV Flight log format* gives the \
             pitch, roll and yaw as attitude angles from their zero on the pad and the tilt as the \
             angle from vertical, each in degrees"
        ));
    }
    if let Some(names) = names_of(columns, &[Kind::Total]) {
        log.notes.push(format!(
            "{names} read in thousandths of standard gravity, from its name and {manual}: its page \
             *CSV Flight log format* gives it as the acceleration's magnitude with the 1 g of \
             gravity taken out, so it sits near zero at rest; it is kept so, the magnitude less \
             1 g, not an acceleration along any axis, and reads −1 g in free fall"
        ));
    }

    let pad_rows = accelerometer_pad_rows(columns, &rows);
    let mut temperature: Option<Vec<f64>> = None;
    for (index, column) in columns.iter().enumerate() {
        let Some(values) = rows.values.get(index) else {
            continue;
        };
        let (quantity, scale): (Quantity, fn(f64) -> f64) = match column.kind {
            Kind::Time => continue,
            Kind::Altitude | Kind::RawAltitude => (Quantity::AltitudeAboveGround, |v| v),
            Kind::Velocity => (Quantity::Speed, |v| v),
            Kind::Axis | Kind::Total => (Quantity::Acceleration, |v| v * G0_M_S2 / 1000.0),
            Kind::Gyro => (Quantity::AngularRate, |v| v),
            Kind::Angle | Kind::Yaw => (Quantity::Angle, |v| v),
            Kind::Pressure => (Quantity::Pressure, |v| v * 100.0),
            Kind::BoardTemperature => (Quantity::Temperature, |v| v + 273.15),
            Kind::CentiTemperature => {
                if centi_degrees(values, &column.written, &mut log.notes) {
                    (Quantity::Temperature, |v| v / 100.0 + 273.15)
                } else {
                    (Quantity::Unrecognized, |v| v)
                }
            }
            Kind::AirBrake => {
                log.notes.push(format!(
                    "`{}` read in percent, from its name: the air-brake output the Mercury logs \
                     (Mercury V1 manual), kept as the percentage written",
                    short(&column.written)
                ));
                (Quantity::Number, |v| v)
            }
            Kind::Kept(why) => {
                log.notes.push(format!(
                    "`{}` kept as written: {why}",
                    short(&column.written)
                ));
                (Quantity::Unrecognized, |v| v)
            }
            Kind::Unknown => {
                line_notes.push(format!(
                    "`{}` kept as written, in a unit no source states: this reader doesn't know \
                     the column",
                    short(&column.written)
                ));
                (Quantity::Unrecognized, |v| v)
            }
        };
        // A finite cell that its unit's conversion makes infinite, such as `1.7e308` mG or hPa, is
        // refused on its row's line (ADR-258 §3).
        let mut converted: Vec<f64> = Vec::with_capacity(values.len());
        for (value, &line) in values.iter().zip(&rows.lines) {
            converted.push(finite(scale(*value), FORMAT, line, &column.written)?);
        }
        if matches!(column.kind, Kind::Axis | Kind::Total) {
            for value in converted.iter_mut().take(pad_rows) {
                *value = f64::NAN;
            }
        }
        if quantity == Quantity::Temperature {
            closing_zeros(&mut converted, &rows.lines, &column.written, &mut log.notes);
            temperature.get_or_insert_with(|| converted.clone());
        }
        if column.kind == Kind::Yaw {
            yaw_end(values, &rows.lines, &column.written, &mut log.notes);
        }
        // The channel holds the rows the file writes the column on (module docs, *Columns*).
        let (times, kept): (Vec<f64>, Vec<f64>) = values
            .iter()
            .zip(&converted)
            .zip(&time_s)
            .filter(|((written, _), _)| !written.is_nan())
            .map(|((_, value), time)| (*time, *value))
            .unzip();
        log.channels.push(Channel {
            column: column.written.clone(),
            quantity,
            time_s: times,
            values: kept,
        });
        if column.kind == Kind::Altitude && log.altitude_m.is_empty() {
            log.altitude_m = converted;
        } else if column.kind == Kind::Velocity && log.vertical_speed_m_s.is_none() {
            log.vertical_speed_m_s = Some(converted);
        }
    }
    if pad_rows > 0
        && let Some(names) = names_of(columns, &[Kind::Axis, Kind::Total])
    {
        log.notes.push(format!(
            "the first {pad_rows} rows, before lift-off, to line {}, write 0 on all three of the \
             accelerometer's axes, which reads 1 g at rest: it hasn't read yet, so {names} are \
             gaps on them",
            rows.lines.get(pad_rows - 1).copied().unwrap_or_default()
        ));
    }
    log.temperature_k = temperature;
    log.time_s = time_s;
    summary(&mut log, summary_lines);
    log.notes.extend(line_notes.into_vec());
    Ok(log)
}

/// How many leading rows are the accelerometer's placeholders: before lift-off, with all three
/// axes exactly 0 (module docs, *Placeholders*). Zero without all three axes.
fn accelerometer_pad_rows(columns: &[Column], rows: &Rows) -> usize {
    let axes: Vec<&Vec<f64>> = columns
        .iter()
        .zip(&rows.values)
        .filter(|(column, _)| column.kind == Kind::Axis)
        .map(|(_, values)| values)
        .collect();
    if axes.len() != 3 {
        return 0;
    }
    rows.time_ms
        .iter()
        .enumerate()
        .take_while(|(row, time)| {
            **time < 0.0 && axes.iter().all(|axis| axis.get(*row) == Some(&0.0))
        })
        .count()
}

/// Whether `bmp_temp(x100)` is in hundredths of a degree Celsius (module docs), with its note.
fn centi_degrees(values: &[f64], written: &str, notes: &mut Vec<String>) -> bool {
    let readings: Vec<f64> = values.iter().copied().filter(|v| !v.is_nan()).collect();
    let (low, high) = BMP581_RANGE_C;
    let within = |celsius: f64| (low..=high).contains(&celsius);
    let whole = readings.iter().all(|value| value.fract() == 0.0);
    let scaled = readings.iter().all(|value| within(value / 100.0));
    let beyond = readings.iter().filter(|value| !within(**value)).count();
    let span = span(&readings);
    if !readings.is_empty() && whole && scaled && beyond > 0 {
        notes.push(format!(
            "`{}` read in hundredths of a degree Celsius, from its name and the values: every \
             reading, {span} as written, is a whole number and lies within the {low} to {high} °C \
             the BMP581 measures when divided by 100, and {beyond} lie beyond it as written",
            short(written)
        ));
        true
    } else {
        let why = if readings.is_empty() {
            "it holds no reading".to_owned()
        } else if !whole {
            format!("its readings, {span}, aren't all whole numbers")
        } else if !scaled {
            format!(
                "its readings, {span}, don't all lie within the {low} to {high} °C the BMP581 \
                 measures when divided by 100"
            )
        } else {
            format!(
                "its readings, {span}, all lie within the {low} to {high} °C the BMP581 measures \
                 as written too, so they could be degrees"
            )
        };
        notes.push(format!(
            "`{}` kept as written, its unit unknown: {why}",
            short(written)
        ));
        false
    }
}

/// Turns a closing run of exactly 0 °C into gaps if the reading before it lies more than
/// [`TEMPERATURE_JUMP_C`] from 0 (module docs, *Placeholders*), with a note. `kelvin` holds the
/// column's values in K, `NaN` on rows without a reading.
fn closing_zeros(kelvin: &mut [f64], lines: &[usize], written: &str, notes: &mut Vec<String>) {
    const ZERO_C_K: f64 = 273.15;
    let readings: Vec<usize> = (0..kelvin.len())
        .filter(|&row| !kelvin[row].is_nan())
        .collect();
    let zeros = readings
        .iter()
        .rev()
        .take_while(|&&row| kelvin[row] == ZERO_C_K)
        .count();
    let Some(&before) = readings
        .len()
        .checked_sub(zeros + 1)
        .and_then(|at| readings.get(at))
    else {
        return;
    };
    let before_c = kelvin[before] - ZERO_C_K;
    if zeros == 0 || before_c.abs() <= TEMPERATURE_JUMP_C {
        return;
    }
    let first = readings[readings.len() - zeros];
    for &row in &readings[readings.len() - zeros..] {
        kelvin[row] = f64::NAN;
    }
    notes.push(format!(
        "the last {zeros} readings of `{}`, from line {}, are exactly 0 °C after a reading of \
         {before_c:.2} °C on line {}: a board can't cool so far between two readings and hold \
         there, so they aren't readings and are gaps",
        short(written),
        lines.get(first).copied().unwrap_or_default(),
        lines.get(before).copied().unwrap_or_default()
    ));
}

/// Notes the rows where the yaw reaches the end of its range (module docs, *Placeholders*).
fn yaw_end(values: &[f64], lines: &[usize], written: &str, notes: &mut Vec<String>) {
    let at_end = |value: &f64| value.abs() >= YAW_END_DEG;
    if let Some(first) = values.iter().position(at_end) {
        notes.push(format!(
            "`{}` reaches ±{YAW_END_DEG}° or beyond on {} rows, the first on line {}: in no \
             sample log does the yaw pass ±327.68°, ±32,768 hundredths of a degree, the range of \
             a signed 16-bit number, so a yaw beyond it isn't written as it was, but clipped or \
             wrapped to the other end; kept as written",
            short(written),
            values.iter().filter(|value| at_end(value)).count(),
            lines.get(first).copied().unwrap_or_default()
        ));
    }
}

/// The least and greatest of the values, as a note writes them.
fn span(values: &[f64]) -> String {
    let (low, high) = values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &value| {
            (low.min(value), high.max(value))
        });
    if low.is_finite() {
        format!("from {low} to {high}")
    } else {
        "none".to_owned()
    }
}

/// Keeps what the summary states (module docs, *The summary*): the apogee, the serial number, the
/// firmware and the events, with notes. `lines` are the lines above the header.
fn summary(log: &mut FlightLog, lines: &[(usize, &str)]) {
    const USED: [&str; 8] = [
        "Apogee meters",
        "Apogee time",
        "Landing time",
        "Burnout time (ms)",
        "Ejection time (ms)",
        "Serial number",
        "Firmware",
        "Adjusted log",
    ];
    let entry =
        |key: &str| summary_entries(lines).find(|entry| entry.key.eq_ignore_ascii_case(key));
    let mut notes = Vec::new();
    let number = |key: &str, notes: &mut Vec<String>| {
        let entry = entry(key)?;
        let value = table::number(entry.value, false);
        if value.is_none() {
            notes.push(format!(
                "the summary's `{key}` on line {}, {:?}, isn't a number: left out",
                entry.line,
                short(entry.value)
            ));
        }
        value
    };
    log.stated.apogee_m = number("Apogee meters", &mut notes);
    let text = |key: &str| {
        entry(key)
            .map(|entry| short(entry.value))
            .filter(|value| !value.is_empty())
    };
    log.serial_number = text("Serial number");
    log.firmware = text("Firmware");
    let mut events = Vec::new();
    for (key, name) in [
        ("Burnout time (ms)", "Burnout"),
        ("Ejection time (ms)", "Ejection"),
    ] {
        if let Some(ms) = number(key, &mut notes) {
            events.push(Event {
                time_s: ms / 1000.0,
                name: name.to_owned(),
            });
        }
    }
    // The highest altitude's time, the first if it is reached twice.
    let peak_s = log
        .altitude_m
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_nan())
        .fold(
            None,
            |best: Option<(usize, f64)>, (row, &value)| match best {
                Some((_, top)) if top >= value => best,
                _ => Some((row, value)),
            },
        )
        .and_then(|(row, _)| log.time_s.get(row).copied());
    if let Some(apogee_s) = number("Apogee time", &mut notes) {
        match peak_s {
            Some(peak_s) if (apogee_s - peak_s).abs() <= APOGEE_TIME_MARGIN_S => {
                notes.push(format!(
                    "`Apogee time` read in seconds, from the altitude: the highest altitude is at \
                     {peak_s} s, within {APOGEE_TIME_MARGIN_S} s of its {apogee_s}, where read in \
                     milliseconds it would be at the lift-off"
                ));
                events.push(Event {
                    time_s: apogee_s,
                    name: "Apogee".to_owned(),
                });
            }
            _ => notes.push(format!(
                "the summary's `Apogee time`, {apogee_s}, is left out: the key states no unit, and \
                 read in seconds it isn't within {APOGEE_TIME_MARGIN_S} s of the highest \
                 altitude's time, {}",
                peak_s.map_or("which the file doesn't give".to_owned(), |t| format!(
                    "{t} s"
                ))
            )),
        }
    }
    if let Some(landing) = number("Landing time", &mut notes) {
        let height_m = entry("Landing meters").and_then(|entry| table::number(entry.value, false));
        match landing_in_seconds(log, landing, height_m, peak_s) {
            Ok(note) => {
                notes.push(note);
                events.push(Event {
                    time_s: landing,
                    name: "Landing".to_owned(),
                });
            }
            Err(why) => notes.push(format!(
                "the summary's `Landing time`, {landing}, is left out: the key states no unit, \
                 and {why}"
            )),
        }
    }
    events.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));
    log.events = events;
    if let Some(adjusted) = entry("Adjusted log")
        && !adjusted.value.eq_ignore_ascii_case("no")
    {
        notes.push(format!(
            "the summary's `Adjusted log` is {:?}: AltimeterCloud can hand back a log \
             recalculated after the flight with another pressure or temperature (Nano V1 manual), \
             so this one's figures may not be the logger's own",
            short(adjusted.value)
        ));
    }
    let is_other = |entry: &Entry<'_>| !USED.iter().any(|key| entry.key.eq_ignore_ascii_case(key));
    if let Some(first) = summary_entries(lines).find(is_other) {
        notes.push(format!(
            "the summary's other {} entries, such as `{}`, are left out: the record has no place \
             for them",
            summary_entries(lines).filter(is_other).count(),
            short(first.key)
        ));
    }
    log.notes.extend(notes);
}

/// The altitude at a time, m: the row's at that time, or on a straight line between the rows
/// either side of it; `None` outside the log or beside a row without an altitude.
fn altitude_at(time_s: &[f64], altitude_m: &[f64], at_s: f64) -> Option<f64> {
    let after = time_s.partition_point(|&time| time < at_s);
    let (t1, a1) = (*time_s.get(after)?, *altitude_m.get(after)?);
    let altitude = if t1 == at_s {
        a1
    } else {
        let before = after.checked_sub(1)?;
        let (t0, a0) = (*time_s.get(before)?, *altitude_m.get(before)?);
        a0 + (at_s - t0) / (t1 - t0) * (a1 - a0)
    };
    altitude.is_finite().then_some(altitude)
}

/// Whether the summary's landing time, `landing` as written, is in seconds (module docs, *The
/// summary*): the note saying so, or why it is left out. It is if, read so, it comes after the
/// highest altitude and the altitude there is the stated landing height, `height_m`, within
/// [`LANDING_HEIGHT_MARGIN_M`], and read in milliseconds it doesn't do both.
fn landing_in_seconds(
    log: &FlightLog,
    landing: f64,
    height_m: Option<f64>,
    peak_s: Option<f64>,
) -> Result<String, String> {
    let Some(height_m) = height_m else {
        return Err(
            "the summary gives no landing height, `Landing meters`, to check it \
                    against"
                .to_owned(),
        );
    };
    let Some(peak_s) = peak_s else {
        return Err("the log gives no highest altitude to check it against".to_owned());
    };
    let at = |time_s: f64| altitude_at(&log.time_s, &log.altitude_m, time_s);
    let agrees = |time_s: f64| {
        time_s >= peak_s
            && at(time_s).is_some_and(|a| (a - height_m).abs() <= LANDING_HEIGHT_MARGIN_M)
    };
    // What the altitude says of a reading that doesn't agree, the time as written with its unit.
    let denies = |time_s: f64, written: &str| {
        if time_s < peak_s {
            format!("{written} comes before the highest altitude, at {peak_s} s")
        } else {
            match at(time_s) {
                Some(a) => format!(
                    "the altitude at {written} is {a:.3} m, not within \
                     {LANDING_HEIGHT_MARGIN_M} m of the stated landing height, {height_m} m"
                ),
                None => format!("the log gives no altitude at {written}"),
            }
        }
    };
    let ms_s = landing / 1000.0;
    match (agrees(landing), agrees(ms_s)) {
        (true, false) => Ok(format!(
            "`Landing time` read in seconds, from the altitude: at {landing} s, after the highest \
             altitude at {peak_s} s, the altitude is {:.3} m, within {LANDING_HEIGHT_MARGIN_M} m \
             of the summary's `Landing meters`, {height_m} m, the height the rocket came to rest \
             at (Nano V1 manual, *Flight info page*); read in milliseconds, {}",
            at(landing).unwrap_or(f64::NAN),
            denies(ms_s, &format!("{landing} ms"))
        )),
        (true, true) => Err(format!(
            "read in seconds or in milliseconds alike, the altitude there, after the highest, is \
             within {LANDING_HEIGHT_MARGIN_M} m of the stated landing height, {height_m} m"
        )),
        (false, _) => Err(format!(
            "read in seconds, {}",
            denies(landing, &format!("{landing} s"))
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NANO_HEADER: &str = "Time(ms),Altitude(m),Velocity(m/s),acceleration_x(mG),\
        acceleration_y(mG),acceleration_z(mG),acceleration_total(mG),gyro_x,gyro_y,gyro_z,\
        Board temp(C),pitch,roll,yaw,tilt";

    const MERCURY_HEADER: &str = "time(ms),altitude(m),velocity_pressure(m/s),acceleration_x(mG),\
        acceleration_y(mG),acceleration_z(mG),acceleration_total(mG),gyro_x,gyro_y,gyro_z,\
        bmp_temp(x100),pitch,roll,yaw,tilt,mt1_temp,apogee_prediction,air_brake_percent,\
        target_alt,air_density,";

    /// A Nano row: time, altitude, the three axes, the temperature cell and the yaw.
    fn nano_row(ms: i32, altitude: f64, axes: [i32; 3], temperature: &str, yaw: f64) -> String {
        let [x, y, z] = axes;
        let total = (f64::from(x * x + y * y + z * z)).sqrt() - 1000.0;
        format!(
            "{ms},{altitude:.3},0.00,{x},{y},{z},{total:.1},1,-2,3,{temperature},1.00,2.00,{yaw:.2},3.00,"
        )
    }

    fn nano(rows: &[String]) -> String {
        format!("{NANO_HEADER}\n{}\n", rows.join("\n"))
    }

    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
        matching[0]
    }

    fn has_note(log: &FlightLog, start: &str) -> bool {
        log.notes.iter().any(|n| n.starts_with(start))
    }

    #[test]
    fn the_header_is_recognized_by_its_own_names() {
        assert!(recognizes(&format!(
            "{NANO_HEADER}\n0,0,0,0,0,0,0,0,0,0,,0,0,0,0,\n"
        )));
        assert!(recognizes(&format!("{MERCURY_HEADER}\n")));
        // After a summary block, and in any case.
        let summary = "Apogee meters,1.0,,\n".repeat(20);
        assert!(recognizes(&format!(
            "{summary}\n{}\n",
            NANO_HEADER.to_ascii_uppercase()
        )));
        // Not without the signature, not without the clock first, not past 64 lines.
        assert!(!recognizes("Time(ms),Altitude(m),tilt\n0,0,0\n"));
        assert!(!recognizes("Time(ms),Altitude(m),acceleration_total(mG)\n"));
        assert!(!recognizes(
            "Altitude(m),Time(ms),acceleration_total(mG),tilt\n"
        ));
        assert!(!recognizes("Time (s),Alt (ft)\n0,0\n1,100\n"));
        let long = "x,1\n".repeat(MAX_SUMMARY_LINES + 1);
        assert!(!recognizes(&format!("{long}{NANO_HEADER}\n")));
        let just = "x,1\n".repeat(MAX_SUMMARY_LINES);
        assert!(recognizes(&format!("{just}{NANO_HEADER}\n")));
    }

    #[test]
    fn summary_entries_pair_keys_with_values_across_empty_cells() {
        let lines = [
            (
                1,
                "Apogee meters,12.5,,Landing meters,-0.1,Burnout time (ms),900,",
            ),
            (2, "Device tag,Unit A,,Competitor tag,,"),
            (3, "Firmware, 1.00,,Lockouts,0,"),
        ];
        let pairs: Vec<(usize, &str, &str)> = summary_entries(&lines)
            .map(|e| (e.line, e.key, e.value))
            .collect();
        assert_eq!(
            pairs,
            [
                (1, "Apogee meters", "12.5"),
                (1, "Landing meters", "-0.1"),
                (1, "Burnout time (ms)", "900"),
                (2, "Device tag", "Unit A"),
                (2, "Competitor tag", ""),
                (3, "Firmware", "1.00"),
                (3, "Lockouts", "0"),
            ]
        );
    }

    #[test]
    fn the_clock_is_milliseconds_and_every_column_is_a_channel() {
        let text = nano(&[
            nano_row(-40, 0.0, [10, 20, -990], "25.00", 0.0),
            nano_row(-20, 0.001, [10, 20, -990], "", 0.0),
            nano_row(0, 0.002, [500, 20, -3000], "", 0.0),
            nano_row(25, 0.5, [600, 20, -4000], "", 0.0),
        ]);
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::AltimeterCloudCsv);
        assert_eq!(log.logger, "Rocketry Ltd Nano V1");
        assert_eq!(log.time_s, [-0.04, -0.02, 0.0, 0.025]);
        assert_eq!(log.altitude_m, [0.0, 0.001, 0.002, 0.5]);
        assert_eq!(log.channels.len(), 14);
        // 1000 mG is standard gravity.
        let x = &log.channels[2];
        assert_eq!(x.column, "acceleration_x(mG)");
        assert!((x.values[0] - 10.0 * G0_M_S2 / 1000.0).abs() < 1e-12);
        // The temperature's channel holds only its row; the track has gaps.
        let temperature = log
            .channels
            .iter()
            .find(|c| c.quantity == Quantity::Temperature)
            .unwrap();
        assert_eq!(temperature.time_s, [-0.04]);
        assert!((temperature.values[0] - 298.15).abs() < 1e-9);
        let track = log.temperature_k.as_deref().unwrap();
        assert!((track[0] - 298.15).abs() < 1e-9);
        assert!(track[1..].iter().all(|t| t.is_nan()));
        note(
            &log,
            "`gyro_x`, `gyro_y` and `gyro_z` read in degrees per second, from the Nano V1 manual: ",
        );
        note(
            &log,
            "`pitch`, `roll`, `yaw` and `tilt` read in degrees, from the Nano V1 manual: ",
        );
        assert!(log.events.is_empty());
    }

    /// A value near the largest `f64` is finite as written and infinite once converted: in
    /// thousandths of g to m/s², in hPa to pascals. Each is refused on its line (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let text = nano(&[
            nano_row(0, 0.0, [10, 20, -990], "", 0.0),
            nano_row(25, 0.5, [600, 20, -4000], "", 0.0).replacen(",600,", ",1.7e308,", 1),
        ]);
        assert_eq!(
            read(&text).unwrap_err().to_string(),
            "AltimeterCloud CSV line 3: the column acceleration_x(mG) holds a value too large \
             for its unit's conversion to SI"
        );
        let text = format!(
            "{NANO_HEADER},pressure(hPa)\n{}1013.25\n{}-1.7e308\n{}1012.5\n",
            nano_row(0, 0.0, [10, 20, -990], "", 0.0),
            nano_row(25, 0.5, [600, 20, -4000], "", 0.0),
            nano_row(50, 1.0, [600, 20, -4000], "", 0.0),
        );
        assert_eq!(
            read(&text).unwrap_err().to_string(),
            "AltimeterCloud CSV line 3: the column pressure(hPa) holds a value too large for its \
             unit's conversion to SI"
        );
    }

    #[test]
    fn rows_of_another_width_or_out_of_order_are_refused_by_line() {
        let good = nano_row(0, 0.0, [1, 2, 3], "", 0.0);
        let short_row = "10,0.0,0.00";
        let error = read(&nano(&[good.clone(), short_row.to_owned()])).unwrap_err();
        assert!(matches!(error, LogError::Syntax { line: 3, .. }), "{error}");
        let wide = format!("{},9", nano_row(10, 0.0, [1, 2, 3], "", 0.0));
        let error = read(&nano(&[good.clone(), wide])).unwrap_err();
        assert!(matches!(error, LogError::Syntax { line: 3, .. }), "{error}");
        // A row without its closing comma is as wide as the header, and reads.
        let bare = nano_row(10, 0.0, [1, 2, 3], "", 0.0);
        assert!(
            read(&nano(&[
                good.clone(),
                bare.trim_end_matches(',').to_owned()
            ]))
            .is_ok()
        );
        let back = nano_row(0, 0.0, [1, 2, 3], "", 0.0);
        let error = read(&nano(&[good.clone(), back])).unwrap_err();
        assert!(error.to_string().contains("doesn't come after"), "{error}");
        let word = nano_row(10, 0.0, [1, 2, 3], "warm", 0.0);
        let error = read(&nano(&[good.clone(), word])).unwrap_err();
        assert!(error.to_string().contains("isn't a number"), "{error}");
        let no_time = format!(",{}", &nano_row(10, 0.0, [1, 2, 3], "", 0.0)[3..]);
        let error = read(&nano(&[good, no_time])).unwrap_err();
        assert!(error.to_string().contains("no time"), "{error}");
        assert!(matches!(
            read(&format!("{NANO_HEADER}\n\n")),
            Err(LogError::NoData { .. })
        ));
    }

    #[test]
    fn a_header_naming_a_column_twice_or_none_is_refused() {
        let twice = format!("{NANO_HEADER},TILT\n");
        let error = read(&twice).unwrap_err();
        assert!(error.to_string().contains("twice"), "{error}");
        let empty = NANO_HEADER.replace(",gyro_y,", ",,");
        let error = read(&format!("{empty}\n")).unwrap_err();
        assert!(error.to_string().contains("has no name"), "{error}");
        let no_altitude = NANO_HEADER.replace("Altitude(m)", "Height");
        let error = read(&format!("{no_altitude}\n")).unwrap_err();
        assert!(error.to_string().contains("no altitude"), "{error}");
    }

    /// The leading rows before lift-off whose three axes read exactly 0 are gaps; a row at or
    /// after lift-off with zeros is kept, as is a row before lift-off with a nonzero axis.
    #[test]
    fn the_accelerometers_zeros_on_the_pad_are_gaps_and_only_there() {
        let text = nano(&[
            nano_row(-60, 0.0, [0, 0, 0], "", 0.0),
            nano_row(-40, 0.0, [0, 0, 0], "", 0.0),
            nano_row(-20, 0.0, [0, 0, -1000], "", 0.0),
            nano_row(0, 0.0, [0, 0, 0], "", 0.0),
        ]);
        let log = read(&text).unwrap();
        let total = log
            .channels
            .iter()
            .find(|c| c.column == "acceleration_total(mG)")
            .unwrap();
        assert!(total.values[0].is_nan() && total.values[1].is_nan());
        assert!(total.values[2].abs() < 1e-12);
        assert!((total.values[3] + G0_M_S2).abs() < 1e-12);
        let gyro = log.channels.iter().find(|c| c.column == "gyro_x").unwrap();
        assert!(gyro.values.iter().all(|v| v.is_finite()));
        assert!(
            note(&log, "the first 2 rows, before lift-off, to line 3,")
                .contains("`acceleration_total(mG)` are gaps")
        );

        // Zeros only from lift-off on: nothing is a gap.
        let text = nano(&[
            nano_row(0, 0.0, [0, 0, 0], "", 0.0),
            nano_row(20, 0.0, [0, 0, 0], "", 0.0),
        ]);
        let log = read(&text).unwrap();
        assert!(
            log.channels
                .iter()
                .all(|c| c.values.iter().all(|v| !v.is_nan()))
        );
        assert!(!has_note(&log, "the first"));
    }

    /// A closing run of exact zeros after a warm reading is a gap; after a reading near 0 °C it
    /// is kept.
    #[test]
    fn closing_zero_temperatures_are_gaps_only_after_a_jump() {
        let rows = |before: &str| {
            nano(&[
                nano_row(0, 0.0, [1, 2, 3], "30.00", 0.0),
                nano_row(20, 0.0, [1, 2, 3], before, 0.0),
                nano_row(40, 0.0, [1, 2, 3], "0.00", 0.0),
                nano_row(60, 0.0, [1, 2, 3], "", 0.0),
                nano_row(80, 0.0, [1, 2, 3], "0.00", 0.0),
            ])
        };
        let log = read(&rows("29.50")).unwrap();
        let track = log.temperature_k.as_deref().unwrap();
        assert!((track[1] - 302.65).abs() < 1e-9);
        assert!(track[2].is_nan() && track[4].is_nan());
        assert!(
            note(&log, "the last 2 readings of `Board temp(C)`, from line 4,")
                .contains("29.50 °C on line 3")
        );

        let log = read(&rows("4.00")).unwrap();
        let track = log.temperature_k.as_deref().unwrap();
        assert!((track[2] - 273.15).abs() < 1e-9 && (track[4] - 273.15).abs() < 1e-9);
        assert!(!has_note(&log, "the last"));
    }

    fn mercury(temperatures: [&str; 3]) -> String {
        let rows: Vec<String> = temperatures
            .iter()
            .enumerate()
            .map(|(row, temperature)| {
                format!(
                    "{},0.000,0.00,1,2,3,-996.3,0,0,0,{temperature},0.00,0.00,0.00,0.00,,0.00,0,,12000,",
                    row * 20
                )
            })
            .collect();
        format!("{MERCURY_HEADER}\n{}\n", rows.join("\n"))
    }

    /// Hundredths of a degree when the values bear it out; kept as written when they could be
    /// degrees, aren't whole, or are too hot even divided by 100.
    #[test]
    fn the_centi_degree_temperature_is_read_only_when_the_values_say_so() {
        let log = read(&mercury(["2000", "", "2150"])).unwrap();
        assert_eq!(log.logger, "Rocketry Ltd Mercury");
        let channel = log
            .channels
            .iter()
            .find(|c| c.column == "bmp_temp(x100)")
            .unwrap();
        assert_eq!(channel.quantity, Quantity::Temperature);
        assert_eq!(channel.time_s, [0.0, 0.04]);
        assert!((channel.values[1] - 294.65).abs() < 1e-9);
        note(
            &log,
            "`bmp_temp(x100)` read in hundredths of a degree Celsius, from its name and the values: ",
        );
        assert!(log.temperature_k.is_some());

        for (cells, why) in [
            (["20", "21", "22"], "as written too"),
            (["2000.5", "2001", "2002"], "aren't all whole numbers"),
            (["9000", "9100", "9200"], "don't all lie within"),
        ] {
            let log = read(&mercury(cells)).unwrap();
            let channel = log
                .channels
                .iter()
                .find(|c| c.column == "bmp_temp(x100)")
                .unwrap();
            assert_eq!(channel.quantity, Quantity::Unrecognized, "{cells:?}");
            assert_eq!(channel.values[0], cells[0].parse::<f64>().unwrap());
            assert!(
                note(&log, "`bmp_temp(x100)` kept as written, its unit unknown: ").contains(why),
                "{cells:?}"
            );
            assert!(log.temperature_k.is_none());
        }
    }

    #[test]
    fn the_mercurys_own_columns_are_kept_as_written() {
        let log = read(&mercury(["2000", "2000", "2000"])).unwrap();
        // The header's closing comma isn't a column.
        assert_eq!(log.channels.len(), 19);
        let quantity = |name: &str| {
            log.channels
                .iter()
                .find(|c| c.column == name)
                .map(|c| (c.quantity, c.values.len()))
        };
        assert_eq!(quantity("mt1_temp"), Some((Quantity::Unrecognized, 0)));
        assert_eq!(
            quantity("apogee_prediction"),
            Some((Quantity::Unrecognized, 3))
        );
        assert_eq!(quantity("air_brake_percent"), Some((Quantity::Number, 3)));
        assert_eq!(quantity("target_alt"), Some((Quantity::Unrecognized, 0)));
        assert_eq!(quantity("air_density"), Some((Quantity::Unrecognized, 3)));
        assert_eq!(
            quantity("velocity_pressure(m/s)"),
            Some((Quantity::Speed, 3))
        );
        for name in ["mt1_temp", "apogee_prediction", "target_alt", "air_density"] {
            note(&log, &format!("`{name}` kept as written: "));
        }
        note(&log, "`air_brake_percent` read in percent, from its name: ");
        assert!(
            note(
                &log,
                "`gyro_x`, `gyro_y` and `gyro_z` read in degrees per second, from "
            )
            .contains("whose layout has the same columns")
        );
    }

    #[test]
    fn an_unknown_column_is_kept_and_its_notes_stay_few() {
        let names: Vec<String> = (0..30).map(|i| format!("extra_{i}")).collect();
        let header = format!("{NANO_HEADER},{}", names.join(","));
        let row = format!(
            "{}{}",
            nano_row(0, 0.0, [1, 2, 3], "", 0.0),
            vec!["7"; 30].join(",")
        );
        let log = read(&format!("{header}\n{row}\n")).unwrap();
        assert_eq!(log.channels.len(), 44);
        let extra = log.channels.iter().find(|c| c.column == "extra_0").unwrap();
        assert_eq!(extra.quantity, Quantity::Unrecognized);
        assert_eq!(extra.values, [7.0]);
        let kept = log
            .notes
            .iter()
            .filter(|n| n.contains("this reader doesn't know the column"))
            .count();
        assert_eq!(kept, 20);
        assert!(has_note(&log, "and 10 more notes"));
    }

    #[test]
    fn a_yaw_at_the_end_of_its_range_is_noted() {
        let text = nano(&[
            nano_row(0, 0.0, [1, 2, 3], "", 300.0),
            nano_row(20, 0.0, [1, 2, 3], "", -327.68),
        ]);
        let log = read(&text).unwrap();
        note(
            &log,
            "`yaw` reaches ±327.67° or beyond on 1 rows, the first on line 3",
        );
        let text = nano(&[nano_row(0, 0.0, [1, 2, 3], "", 327.66)]);
        assert!(!has_note(&read(&text).unwrap(), "`yaw` reaches"));
    }

    fn with_summary(summary: &str) -> String {
        format!(
            "{summary}\n\n{}",
            nano(&[
                nano_row(-20, 0.0, [1, 2, 3], "", 0.0),
                nano_row(0, 0.0, [1, 2, 3], "", 0.0),
                nano_row(5_000, 100.0, [1, 2, 3], "", 0.0),
                nano_row(5_100, 101.0, [1, 2, 3], "", 0.0),
                nano_row(5_200, 100.5, [1, 2, 3], "", 0.0),
                nano_row(30_000, 0.0, [1, 2, 3], "", 0.0),
            ])
        )
    }

    /// The apogee time and the landing time in seconds when the altitude bears them out; left
    /// out when it doesn't.
    #[test]
    fn the_summary_gives_the_apogee_the_serial_and_the_events() {
        let summary = "Apogee meters,101.00,,Landing meters,0.0,Burnout time (ms),1200,\n\
                       Apogee time,5.150,,Landing time,30.000,\n\
                       Serial number,0000-0000-0001,,Ejection time (ms),5300,\n\
                       Firmware, 1.00,,Adjusted log,no,\n\
                       Device tag,Unit A,,Competitor tag,,";
        let log = read(&with_summary(summary)).unwrap();
        assert_eq!(log.stated.apogee_m, Some(101.0));
        assert_eq!(log.serial_number.as_deref(), Some("0000-0000-0001"));
        assert_eq!(log.firmware.as_deref(), Some("1.00"));
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [
                (1.2, "Burnout"),
                (5.15, "Apogee"),
                (5.3, "Ejection"),
                (30.0, "Landing")
            ]
        );
        assert!(
            note(&log, "`Apogee time` read in seconds, from the altitude: ").contains("at 5.1 s")
        );
        assert!(
            note(
                &log,
                "`Landing time` read in seconds, from the altitude: at 30 s, "
            )
            .contains(
                "the altitude is 0.000 m, within 0.01 m of the summary's `Landing meters`, 0 m"
            )
        );
        assert!(
            note(
                &log,
                "the summary's other 3 entries, such as `Landing meters`"
            )
            .contains("left out")
        );
        assert!(!has_note(&log, "the summary's `Adjusted log`"));

        // An apogee time a thousand times off is left out, and a landing time with no landing
        // height to check it against.
        let summary = "Apogee time,5150,,Landing time,30000,\nAdjusted log,yes,";
        let log = read(&with_summary(summary)).unwrap();
        assert!(log.events.is_empty());
        assert!(note(&log, "the summary's `Apogee time`, 5150, is left out").contains("5.1 s"));
        assert!(
            note(&log, "the summary's `Landing time`, 30000, is left out: ")
                .contains("gives no landing height")
        );
        note(&log, "the summary's `Adjusted log` is \"yes\"");
        assert_eq!(log.stated.apogee_m, None);
        let log = read(&with_summary("Apogee meters,high,")).unwrap();
        note(
            &log,
            "the summary's `Apogee meters` on line 1, \"high\", isn't a number",
        );
    }

    /// The landing time in seconds only where the altitude, after the highest, is the stated
    /// landing height, and in milliseconds it isn't; between rows, on the line joining them.
    #[test]
    fn the_landing_time_is_read_in_seconds_only_where_the_altitude_agrees() {
        let landing = |height: &str, time: &str| {
            let summary = format!("Landing meters,{height},,Landing time,{time},");
            read(&with_summary(&summary)).unwrap()
        };
        let events = |log: &FlightLog| -> Vec<(f64, String)> {
            log.events
                .iter()
                .map(|e| (e.time_s, e.name.clone()))
                .collect()
        };
        // Between the rows at 5.2 s, 100.5 m, and 30 s, 0 m: 4.052 m at 29 s.
        let log = landing("4.05", "29.000");
        assert_eq!(events(&log), [(29.0, "Landing".to_owned())]);
        assert!(
            note(
                &log,
                "`Landing time` read in seconds, from the altitude: at 29 s, "
            )
            .contains("the altitude is 4.052 m")
        );
        // A landing height 2 cm off, or a time the log doesn't reach, or one before the highest
        // altitude, is left out.
        for (height, time, written, why) in [
            (
                "4.07",
                "29.000",
                "29",
                "the altitude at 29 s is 4.052 m, not within 0.01 m",
            ),
            ("0.0", "31.000", "31", "the log gives no altitude at 31 s"),
            (
                "50.0",
                "2.600",
                "2.6",
                "2.6 s comes before the highest altitude, at 5.1 s",
            ),
        ] {
            let log = landing(height, time);
            assert!(events(&log).is_empty(), "{time}");
            let start = format!("the summary's `Landing time`, {written}, is left out: ");
            assert!(note(&log, &start).contains(why), "{time}");
            assert!(!has_note(&log, "`Landing time` read in"), "{time}");
        }
        // A log whose altitude peaks at once, so that read in milliseconds too the landing time
        // comes after the highest altitude where the altitude is the landing height: left out.
        let text = format!(
            "Landing meters,0.0,,Landing time,20.000,\n\n{}",
            nano(&[
                nano_row(0, 5.0, [1, 2, 3], "", 0.0),
                nano_row(10, 0.0, [1, 2, 3], "", 0.0),
                nano_row(20_000, 0.0, [1, 2, 3], "", 0.0),
            ])
        );
        let log = read(&text).unwrap();
        assert!(log.events.is_empty());
        assert!(
            note(&log, "the summary's `Landing time`, 20, is left out: ")
                .contains("in seconds or in milliseconds alike")
        );
    }

    #[test]
    fn the_altitude_at_a_time_is_a_rows_or_on_the_line_between_two() {
        let time = [0.0, 1.0, 3.0, 4.0];
        let altitude = [0.0, 10.0, 30.0, f64::NAN];
        assert_eq!(altitude_at(&time, &altitude, 1.0), Some(10.0));
        assert_eq!(altitude_at(&time, &altitude, 2.5), Some(25.0));
        assert_eq!(altitude_at(&time, &altitude, 0.0), Some(0.0));
        assert_eq!(altitude_at(&time, &altitude, -0.5), None);
        assert_eq!(altitude_at(&time, &altitude, 3.5), None);
        assert_eq!(altitude_at(&time, &altitude, 5.0), None);
    }

    /// A long line of commas, above a header or as the whole file, is read without collecting
    /// its cells.
    #[test]
    fn a_long_line_of_commas_is_no_header_and_no_summary() {
        let commas = ",".repeat(1 << 20);
        assert!(!recognizes(&commas));
        assert!(!recognizes(&format!("Time(ms){commas}")));
        let log = read(&format!(
            "{commas}\n\n{}",
            nano(&[nano_row(0, 0.0, [1, 2, 3], "", 0.0)])
        ))
        .unwrap();
        assert!(!has_note(&log, "the summary's other"));
        // A header with an empty name in the middle is refused; a closing one is dropped.
        let error = read(&format!("{NANO_HEADER}{commas}x\n")).unwrap_err();
        assert!(error.to_string().contains("has no name"), "{error}");
    }

    /// The total acceleration's note says it is the magnitude less 1 g.
    #[test]
    fn the_total_acceleration_is_noted_as_the_magnitude_less_one_g() {
        let log = read(&nano(&[nano_row(0, 0.0, [1, 2, 3], "", 0.0)])).unwrap();
        assert!(
            note(
                &log,
                "`acceleration_total(mG)` read in thousandths of standard gravity, from its name \
                 and the Nano V1 manual: "
            )
            .contains("the magnitude less 1 g, not an acceleration along any axis")
        );
    }

    /// Every note on a unit opens as `cargo xtask logs` counts a unit decision: a column in
    /// backticks, then `read in` or `kept as written`, then a colon.
    #[test]
    fn unit_notes_open_as_the_survey_counts_them() {
        let log = read(&mercury(["2000", "2010", "2020"])).unwrap();
        for note in log.notes.iter().filter(|n| n.starts_with('`')) {
            let (decision, _) = note.split_once(": ").unwrap();
            assert!(
                decision.contains("` read in ") || decision.contains("` kept as written"),
                "{note}"
            );
        }
    }
}
