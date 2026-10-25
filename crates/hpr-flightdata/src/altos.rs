//! AltOS's CSV flight logs, as AltosUI exports a TeleMetrum's, TeleMega's, EasyMega's or
//! EasyMini's flight.
//!
//! The file is text: perhaps a few comment lines starting with `#`, then one header line naming
//! the columns, then one row per sample, the cells separated by a comma and spaces:
//!
//! ```text
//! #version,serial,flight,call,time,rssi,lqi,state,state_name,acceleration,pressure,altitude,height,speed,temperature,…
//! 6, 1234, 3,N0CALL,    -1.00, -80,  40, 2,     pad,    0.10,  98000.00,  250.00,    0.00,    0.00, 21.0,…
//! ```
//!
//! The columns differ by device and by AltosUI's version: the header says which there are, and
//! this reader maps them by name. `call` is there when the flight came in by radio, and `rssi`
//! and `lqi` when it was received as telemetry; a GPS-equipped unit adds `latitude`, `longitude`,
//! a second `altitude`, the time of day, the distance and bearing to the pad and the receiver's
//! satellite columns `sat01`…`sat32`; a TeleMega or EasyMega adds its inertial and magnetic
//! sensors. One of the files read has no `pressure` or barometric `altitude` column at all.
//!
//! **Units.** The AltOS manual says the file holds "all of the sensor values … converted to
//! standard units, with the barometric data reported in both pressure, altitude and height above
//! pad units", and that its header and comment lines start with `#` (Altus Metrum, *AltOS*
//! manual, §12.5.1 "Comma Separated Value Format"). It names no column's unit; these are each
//! one's reading, and where it comes from:
//!
//! | columns | read as | from |
//! |---|---|---|
//! | `time` | the clock, s, from the logger's zero at boost: negative on the pad | Debrief's reader |
//! | `height` | [`Quantity::AltitudeAboveGround`], m above the pad | Debrief's reader; the manual's "height above pad" |
//! | `altitude` (the first) | [`Quantity::AltitudeMsl`], m, the barometric altitude | the manual; Debrief's reader |
//! | `altitude` after `longitude` | [`Quantity::GnssAltitude`], m, the GPS receiver's | Debrief's reader, which finds it by its place |
//! | `pressure` | [`Quantity::Pressure`], Pa; hPa if the column never reaches 1,200 (below) | the files read: about 98,000 on a pad |
//! | `acceleration`, `accel_x`/`_y`/`_z` | [`Quantity::Acceleration`], m/s² | Debrief's reader and its proof from the data (below) |
//! | `speed`, `accel_speed`, `baro_speed` | [`Quantity::Speed`], m/s, up positive | Debrief's reader |
//! | `temperature` | [`Quantity::Temperature`], °C in the file, K here | Debrief's reader |
//! | `battery_voltage`, `drogue_voltage`, `main_voltage`, `pyro`, `igniter_*` | [`Quantity::Voltage`], V | Debrief's reader (battery); the manual's §13.9 states the igniter and battery readings in volts |
//! | `latitude`, `longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`], degrees | Debrief's reader |
//! | `tilt`, `pad_az`, `pad_el` | [`Quantity::Angle`], degrees | the manual (its pyro condition's "angle from vertical", in degrees; §13.10's bearing "in degrees"); the files read stay within 0–180 and 0–360 |
//! | `pad_dist`, `pad_range` | [`Quantity::Distance`], m | only the manual's "standard units": no document states their unit |
//! | `version`, `serial`, `flight`, `rssi`, `lqi`, `state`, `connected`, `locked`, `nsat`, `year`…`second`, `pdop`, `hdop`, `vdop`, `sat01`…`sat32` | [`Quantity::Number`], as written | counts, codes, flags, signal strengths and dilutions of precision |
//! | `gyro_roll`/`_pitch`/`_yaw`, `mag_x`/`_y`/`_z` | [`Quantity::Unrecognized`], as written | the record has no quantity for a rotation rate or a magnetic field, and no document states their unit |
//!
//! Every other column is kept as written, [`Quantity::Unrecognized`], with a note. `call` (the
//! radio call sign) and `state_name` (the flight computer's state in words) are text, not
//! channels: each state the file changes to is an [`Event`] instead, at the time of the row that
//! changes it, the first row's state included. The numeric `state` column is kept as a channel.
//!
//! **The primary track.** [`FlightLog::altitude_m`] is `height`; the vertical speed is
//! `baro_speed`, the barometer's, and absent without it: `accel_speed` integrates the
//! accelerometer and `speed` is the flight computer's own estimate, which leans on the
//! accelerometer on the way up, so both stay channels only (Debrief's reader took `accel_speed`
//! first). The temperature is `temperature` and the battery `battery_voltage`. `time` is
//! required. A header with no `height` gives the record no primary track, only its channels,
//! with a note.
//!
//! **Quirks, and what this reader does about them.**
//!
//! - *Repeated times.* AltosUI writes rows that share a time with the row before, hundreds in a
//!   flight; in every one of the files read, such rows also repeat the height. The primary track
//!   must have strictly increasing times, so it keeps the first row at each time, and the rows it
//!   leaves out are counted in a note. Rows out of time order (none in the files read) are put in
//!   order on the primary track, again with a note. This is what Debrief does (its
//!   `lib/flight/build.ts`: sort by time, keep the first row at each time). The channels keep
//!   every row, in the file's order, each on its own time.
//! - *Acceleration net of gravity.* AltOS writes `acceleration` along the rocket's axis with
//!   gravity taken out: it reads about 0 at rest, not 1 g, while the TeleMega's body-axis
//!   `accel_x` on the same row reads about 9.8 m/s² (Debrief's reader, from the data). The
//!   channel is kept as written, and a note says so. The pad's reading of 1 g along the axis is
//!   what comes out, so the channel is the rocket's own acceleration only while it points
//!   straight up; tilted, it is off by the difference between g and g's share along the axis.
//! - *GPS before a fix.* AltOS writes a position of exactly (0, 0) before its receiver has a fix
//!   (Debrief's reader). A row with latitude and longitude both 0 has its latitude, longitude and
//!   GPS altitude read as gaps, counted in a note. None of the files read has such a row; the
//!   rule is Debrief's, kept so that a track can't run to the equator.
//! - *Pressure in hectopascals.* Debrief's own sample file writes pressure in hPa (about 983 on
//!   its pad), where AltosUI's exports write Pa. No launch site's pressure is below 10 kPa nor
//!   above 120 kPa, and the two units are a hundredfold apart, so the column's highest value
//!   settles it: from 10,000 to 120,000, Pa; from 100 to 1,200, hPa, converted with a note;
//!   anything else is kept as written, [`Quantity::Unrecognized`], with a note.
//! - *Gaps.* AltosUI writes `NaN` where it has no value (a TeleMega's `tilt` before it has an
//!   attitude). A cell that is empty or not a finite number is a gap, `NaN`, counted per column
//!   in a note.
//! - *Short and long rows.* A row with fewer cells than the header (a file cut off mid-row) has
//!   the rest as gaps, with a note; a row with more cells is refused, naming its line, unless the
//!   extra cells are empty (a trailing comma). A row whose time isn't a number can't be placed on
//!   the clock and is left out, with a note.
//!
//! **The states.** AltosUI's exports number the flight's states and name them on each row: 2
//! `pad`, 3 `boost`, 4 `fast`, 5 `coast`, 6 `drogue`, 7 `main` and 8 `landed`. A row whose
//! `state` and `state_name` are one of these numbers named as another, or a name numbered as
//! another, is counted in a note, the first such line quoted, and the events take the names as
//! written: Debrief's own sample export names state 1 `pad`. (A table cut from an export is
//! stricter, below.)
//!
//! **A spreadsheet's re-save.** A flyer may open an export in a spreadsheet and save it again. The
//! reader reads three things a spreadsheet leaves:
//!
//! - *A unit after a name.* A header may write a unit after a column's name in parentheses, `height
//!   (m)`. The column is read by its name. Where the unit is the one this reader reads the column
//!   in (the table above: `s`; `m`; `m/s^2`, `m/s²` or `m/s/s`; `m/s`; `V`; `degrees`, `deg` or
//!   `°`; `C` or `°C`; `Pa`), it is read as AltOS's, the columns named in one note. Any other
//!   unit, such as `altitude (ft)` or `accel_x (m/s)`, keeps the column as written,
//!   [`Quantity::Unrecognized`], with a note: the reader can't tell how its values were made. A
//!   pressure written `Pa` whose values settle as hPa is kept as written too. A unit after `time`
//!   other than `s` is refused, as the clock can't be placed.
//! - *Formulas.* A cell may hold the spreadsheet's formula, such as `=3.28084*J2`, not a value.
//!   The formulas aren't worked out. A column of formulas alone isn't a channel; a column with
//!   some has those cells as gaps. Either way the column's formulas are kept as text in a note,
//!   run by run: a run is rows on consecutive lines whose formula is the same once each reference
//!   to the cell's own row is written `{row}`, so `=3.28084*J2` on line 2 and `=3.28084*J3` on
//!   line 3 are one run, `` `=3.28084*J{row}` on lines 2–3 ``, and each cell the note gives can
//!   be rebuilt from it. A reference is one to three capital letters, an optional `$` and the
//!   row's number; a CSV a spreadsheet saves writes its row *n* on line *n*. A note gives at most 8 runs a
//!   column, counting the cells past them, and quotes at most 200 characters of a formula; at
//!   most 64 columns' formulas are noted, one more note counting the rest.
//! - *A deleted column.* `height` may be gone, as above.
//!
//! **A table cut from an export.** A file may hold AltOS's columns alone: a header of AltOS's own
//! names with no `#version,serial,flight` before them and no unit after them, naming `time`,
//! `state` and `state_name`, such as `time,state,state_name,acceleration,pressure,altitude,height`.
//! Such names alone don't show the file is AltOS's, so its states must: of its first 100 rows, at
//! least one must give a `state` and a `state_name` that are one of AltOS's pairs, and none may
//! give a pair that contradicts them ([`recognizes_table`]); a row further on that contradicts
//! them is refused at its line. Each row is then read as an export's, with a note that the header
//! was found this way.
//!
//! **What the file says about itself.** The serial number and flight number come from the comment
//! lines `# serial …` and `# flight …` when the file has them, and from the first row's `serial`
//! and `flight` cells otherwise; `# product …` names the device, as in `AltOS TeleMetrum`. The
//! file states no firmware version (its `version` column is the data's, kept as a channel), no
//! apogee and no sample count.
//!
//! **Where this comes from.** Ported from Debrief's reader (`lib/parsers/altusmetrum.ts`, MIT, the
//! project owner's own; see `THIRD-PARTY-NOTICES.md`), which was written from exported files, and
//! the AltOS manual's description of the export. No AltOS or AltosUI source code was read. Where
//! this reader departs from Debrief's: it keeps every column as a channel where Debrief keeps a
//! chosen few; it keeps the file's clock, where Debrief moves the zero to the first row; it reads a
//! hectopascal pressure as such; it refuses a long row; it requires the header first (after
//! comment lines), where Debrief searches the first 60 lines for it. The units after names, the
//! formulas, the record with no `height` and the table cut from an export are this reader's own,
//! written from the collection's files ([ADR-274][adr-274]). AltOS's radio telemetry is other
//! files: its log of packets, `.telem` ([`crate::altos_telem`]), and the packets decoded to a
//! table, `serial,tick,ptype,…` ([`crate::altos_telem_csv`]).
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::finite;

/// The format's name in errors.
const FORMAT: &str = "AltOS CSV";

/// The most notes about single lines a log keeps; past it, one more note counts the rest.
const MAX_NOTES: usize = 20;

/// The most characters of the file an error or note quotes.
const QUOTED_CHARS: usize = 80;

/// What a column of the file is to this reader.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    /// The `time` column: the clock.
    Clock,
    /// Text, not a channel: `call` and `state_name`.
    Text,
    /// A channel of this quantity, the file's values as SI.
    Value(Quantity),
    /// A temperature in °C, a channel in K.
    Celsius,
    /// A pressure, Pa or hPa as the column's values settle.
    Pressure,
}

/// A column's role from its name, and whether the reader knows the name. `after_longitude` says
/// whether a `longitude` column comes before it, which makes an `altitude` the GPS receiver's.
fn role(name: &str, after_longitude: bool) -> (Role, bool) {
    use Quantity as Q;
    let lower = name.to_ascii_lowercase();
    let value = |quantity| (Role::Value(quantity), true);
    match lower.as_str() {
        "time" => (Role::Clock, true),
        "call" | "state_name" => (Role::Text, true),
        "height" => value(Q::AltitudeAboveGround),
        "altitude" if after_longitude => value(Q::GnssAltitude),
        "altitude" => value(Q::AltitudeMsl),
        "pressure" => (Role::Pressure, true),
        "acceleration" | "accel_x" | "accel_y" | "accel_z" => value(Q::Acceleration),
        "speed" | "accel_speed" | "baro_speed" => value(Q::Speed),
        "temperature" => (Role::Celsius, true),
        "battery_voltage" | "drogue_voltage" | "main_voltage" | "pyro" => value(Q::Voltage),
        "latitude" => value(Q::Latitude),
        "longitude" => value(Q::Longitude),
        "tilt" | "pad_az" | "pad_el" => value(Q::Angle),
        "pad_dist" | "pad_range" => value(Q::Distance),
        "version" | "serial" | "flight" | "rssi" | "lqi" | "state" | "connected" | "locked"
        | "nsat" | "year" | "month" | "day" | "hour" | "minute" | "second" | "pdop" | "hdop"
        | "vdop" => value(Q::Number),
        "gyro_roll" | "gyro_pitch" | "gyro_yaw" | "mag_x" | "mag_y" | "mag_z" => {
            value(Q::Unrecognized)
        }
        _ if lower.starts_with("igniter_") => value(Q::Voltage),
        _ if is_satellite(&lower) => value(Q::Number),
        _ => (Role::Value(Q::Unrecognized), false),
    }
}

/// A header cell split into its name and the unit written after it in parentheses, as a
/// spreadsheet's user wrote `acceleration (m/s^2)`: the unit `None` for a name with none, such as
/// AltosUI writes.
fn split_unit(cell: &str) -> (&str, Option<&str>) {
    match cell
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("))
    {
        Some((name, unit)) if !name.trim().is_empty() => (name.trim(), Some(unit.trim())),
        _ => (cell, None),
    }
}

/// Whether a unit written after a column's name is the one this reader reads the column's role
/// in ([`split_unit`]). A pressure's `Pa` must also be the unit its values settle.
fn unit_agrees(role: Role, unit: &str) -> bool {
    use Quantity as Q;
    let units: &[&str] = match role {
        Role::Clock => &["s"],
        Role::Value(Q::AltitudeAboveGround | Q::AltitudeMsl | Q::GnssAltitude | Q::Distance) => {
            &["m"]
        }
        Role::Value(Q::Acceleration) => &["m/s^2", "m/s²", "m/s/s"],
        Role::Value(Q::Speed) => &["m/s"],
        Role::Value(Q::Voltage) => &["V"],
        Role::Value(Q::Latitude | Q::Longitude | Q::Angle) => &["degrees", "deg", "°"],
        Role::Celsius => &["C", "°C"],
        Role::Pressure => &["Pa"],
        _ => &[],
    };
    units.contains(&unit)
}

/// The unit a header writes after a column's name, against the one this reader reads it in.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Unit {
    /// None written, as AltosUI writes its header.
    Unwritten,
    /// The one this reader reads the column in: the column is read as AltOS's.
    Agrees,
    /// Another: the column is kept as written.
    Differs,
}

/// Whether a column is one of the receiver's satellite columns, `sat01` to `sat32`.
fn is_satellite(lower: &str) -> bool {
    lower
        .strip_prefix("sat")
        .is_some_and(|digits| digits.len() == 2 && digits.bytes().all(|b| b.is_ascii_digit()))
}

/// Notes about single lines as the reader meets them: the first [`MAX_NOTES`] kept, the rest
/// only counted, so a file of many broken lines can't fill memory with them.
#[derive(Default)]
struct Notes {
    kept: Vec<String>,
    left_out: usize,
}

impl Notes {
    fn push(&mut self, note: String) {
        if self.kept.len() < MAX_NOTES {
            self.kept.push(note);
        } else {
            self.left_out += 1;
        }
    }
}

/// What the comment lines before the header state.
#[derive(Default)]
struct Preamble {
    product: Option<String>,
    serial: Option<String>,
    flight: Option<String>,
}

/// Reads an AltOS CSV flight log from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't a `#` comment isn't AltOS's header
/// (`version,serial,flight,…`, with or without a leading `#`) or a header of AltOS's own names
/// that names `time`, `state` and `state_name`; [`LogError::Syntax`] for a header without a
/// `time` column, a row with more cells than the header names, a row whose `state` and
/// `state_name` aren't the number and the name AltOS gives one state, a header-only table none of
/// whose rows gives a state AltOS's exports name, or a pressure in hPa too large to convert to
/// pascals; [`LogError::NoData`] if no row has a time.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()));

    let mut preamble = Preamble::default();
    let mut comments = false;
    let (header_line, header, bare_table) = loop {
        match lines.next() {
            Some((_, "")) => {}
            Some((number, line)) => {
                let bare = line.trim_start_matches('#').trim();
                if is_header(bare) {
                    break (number, bare, false);
                }
                if !comments && is_table_header(line) {
                    break (number, line, true);
                }
                if line.starts_with('#') {
                    comments = true;
                    preamble.read(bare);
                    continue;
                }
                return Err(LogError::NotThisFormat {
                    format: FORMAT,
                    message: format!(
                        "line {number}, {:?}, isn't AltOS's header, `version,serial,flight,…`",
                        short(line)
                    ),
                });
            }
            None => {
                return Err(LogError::NotThisFormat {
                    format: FORMAT,
                    message: "the file has no `version,serial,flight,…` header".to_owned(),
                });
            }
        }
    };
    let names: Vec<String> = header
        .split(',')
        .map(|cell| cell.trim().to_owned())
        .collect();
    let columns = header_columns(&names, header_line)?;

    let mut notes = Notes::default();
    let mut table = Table::new(&names, &columns, bare_table);
    for (number, line) in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            notes.push(format!(
                "line {number}, a `#` line among the rows, was skipped"
            ));
            continue;
        }
        table.row(&columns, line, number, &mut notes)?;
    }
    if table.time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    if bare_table && table.states_checked == 0 {
        return Err(LogError::Syntax {
            format: FORMAT,
            line: header_line,
            message: "a table of AltOS's columns with no `version,serial,flight` header is read \
                      by its states. No row's `state` and `state_name` are a state AltOS's \
                      exports name"
                .to_owned(),
        });
    }
    assemble(&names, &columns, table, &preamble, notes, bare_table)
}

/// Whether a line, its `#` taken off, is AltOS's header.
fn is_header(bare: &str) -> bool {
    let mut cells = bare.split(',').map(str::trim);
    ["version", "serial", "flight"].iter().all(|expected| {
        cells
            .next()
            .is_some_and(|cell| cell.eq_ignore_ascii_case(expected))
    })
}

/// Whether a line is a header of AltOS's own column names only, as AltosUI writes them, with no
/// `version,serial,flight` before them and no unit after them, naming `time`, `state` and
/// `state_name`: a table cut from an AltOS export.
fn is_table_header(line: &str) -> bool {
    let names: Vec<&str> = line.split(',').map(str::trim).collect();
    let mut after_longitude = false;
    let all_known = names.iter().all(|name| {
        let known = role(name, after_longitude).1;
        after_longitude |= name.eq_ignore_ascii_case("longitude");
        known
    });
    all_known
        && ["time", "state", "state_name"]
            .iter()
            .all(|wanted| names.iter().any(|name| name.eq_ignore_ascii_case(wanted)))
}

/// Whether a text is a table cut from an AltOS export, its header AltOS's own names with no
/// `version,serial,flight` before them, naming `time`, `state` and `state_name`, and AltOS's
/// states its proof: of its first 100 rows, at least one gives a `state` and a `state_name` that
/// AltOS's exports write for one state ([`crate::altos_telem`]), and none gives a pair they
/// contradict.
pub fn recognizes_table(text: &str) -> bool {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|line| !line.is_empty());
    let Some(header) = lines.next().filter(|line| is_table_header(line)) else {
        return false;
    };
    let names: Vec<&str> = header.split(',').map(str::trim).collect();
    let find = |wanted: &str| {
        names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(wanted))
    };
    let (Some(state), Some(state_name)) = (find("state"), find("state_name")) else {
        return false;
    };
    let mut checked = 0;
    for line in lines.take(100) {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if let (Some(number), Some(name)) = (cells.get(state), cells.get(state_name)) {
            match state_pair(number, name) {
                Ok(true) => checked += 1,
                Ok(false) => {}
                Err(_) => return false,
            }
        }
    }
    checked > 0
}

/// Whether a row's `state` and `state_name` are a pair AltOS's exports write ([`crate::altos_telem`]):
/// `Ok(true)` if they are, `Ok(false)` if neither is a state they name, so nothing is checked,
/// and why not if one is and the other isn't its partner.
fn state_pair(number: &str, name: &str) -> Result<bool, String> {
    use crate::altos_telem::STATES;
    // A number a spreadsheet rewrote, `3.0` for `3`, is the same state.
    let by_number = number
        .parse::<f64>()
        .ok()
        .and_then(|number| STATES.iter().find(|(known, _)| f64::from(*known) == number));
    let by_name = STATES
        .iter()
        .find(|(_, known)| known.eq_ignore_ascii_case(name));
    match (by_number, by_name) {
        (None, None) => Ok(false),
        (Some(a), Some(b)) if a == b => Ok(true),
        (Some((known, partner)), _) => Err(format!(
            "the row's state {known} is named {:?}, where AltOS names {known} `{partner}`",
            short(name)
        )),
        (None, Some((partner, known))) => Err(format!(
            "the row's state {:?} is named `{known}`, where AltOS numbers `{known}` {partner}",
            short(number)
        )),
    }
}

impl Preamble {
    /// Reads a comment line such as `product TeleMetrum`, keeping what names the logger and the
    /// flight; the rest (`call`, a title) is passed over.
    fn read(&mut self, bare: &str) {
        let Some((key, value)) = bare.split_once(char::is_whitespace) else {
            return;
        };
        let value = value.trim();
        if value.is_empty() {
            return;
        }
        let slot = match key.to_ascii_lowercase().as_str() {
            "product" => &mut self.product,
            "serial" => &mut self.serial,
            "flight" => &mut self.flight,
            _ => return,
        };
        slot.get_or_insert_with(|| short(value));
    }
}

/// A column of the header: its role, whether the reader knows its name, and the unit written
/// after the name.
#[derive(Debug, Clone, Copy)]
struct Column {
    role: Role,
    known: bool,
    unit: Unit,
}

/// The index of the first header cell whose name, less any unit after it, is the one wanted.
fn named(names: &[String], wanted: &str) -> Option<usize> {
    names
        .iter()
        .position(|name| split_unit(name).0.eq_ignore_ascii_case(wanted))
}

/// Each header cell's column; refuses a header without `time`.
fn header_columns(names: &[String], number: usize) -> Result<Vec<Column>, LogError> {
    let mut after_longitude = false;
    let mut altitudes = 0;
    let columns: Vec<Column> = names
        .iter()
        .map(|cell| {
            let (name, unit) = split_unit(cell);
            let lower = name.to_ascii_lowercase();
            // A second `altitude` is the GPS receiver's even if no `longitude` precedes it.
            let gnss = after_longitude || (lower == "altitude" && altitudes > 0);
            if lower == "altitude" {
                altitudes += 1;
            }
            if lower == "longitude" {
                after_longitude = true;
            }
            let (role, known) = role(name, gnss);
            match unit {
                None => Column {
                    role,
                    known,
                    unit: Unit::Unwritten,
                },
                Some(unit) if known && unit_agrees(role, unit) => Column {
                    role,
                    known,
                    unit: Unit::Agrees,
                },
                Some(_) => Column {
                    role: if known && role != Role::Text {
                        Role::Value(Quantity::Unrecognized)
                    } else {
                        role
                    },
                    known,
                    unit: Unit::Differs,
                },
            }
        })
        .collect();
    match named(names, "time") {
        Some(index) if columns[index].role == Role::Clock => Ok(columns),
        Some(_) => Err(LogError::Syntax {
            format: FORMAT,
            line: number,
            message: "the header writes a unit after `time` other than s".to_owned(),
        }),
        None => Err(LogError::Syntax {
            format: FORMAT,
            line: number,
            message: "the header names no `time` column".to_owned(),
        }),
    }
}

/// The most runs of formulas a column's note gives one by one ([`Formulas`]); past them the
/// rest are counted.
const MAX_FORMULA_RUNS: usize = 8;

/// A run of rows on consecutive lines whose cells in a column hold one formula, its cell
/// references to its own row written `{row}`: the formula, and the first and last lines of the
/// run.
#[derive(Debug, Clone)]
struct Run {
    formula: String,
    first: usize,
    last: usize,
}

/// The most characters of a formula a note quotes.
const FORMULA_CHARS: usize = 200;

/// The most columns of formulas the notes give, apart from the other notes' bound, so that a
/// note about a column or a line can't push a formula out; past them one note counts the rest.
const MAX_FORMULA_COLUMNS: usize = 64;

/// A column's formula cells, the runs of them kept as text.
#[derive(Debug, Clone, Default)]
struct Formulas {
    /// The cells that hold a formula.
    cells: usize,
    /// The first [`MAX_FORMULA_RUNS`] runs, in the file's order.
    runs: Vec<Run>,
    /// The cells of the runs past them.
    left_out: usize,
}

impl Formulas {
    /// Adds a formula cell on a line, the line being the cell's row in the spreadsheet, as a CSV
    /// a spreadsheet saves writes its row 1 on line 1.
    fn add(&mut self, cell: &str, line: usize) {
        self.cells += 1;
        let formula = cut(&own_row(cell, line), FORMULA_CHARS);
        if let Some(run) = self.runs.last_mut()
            && self.left_out == 0
            && run.formula == formula
            && line == run.last + 1
        {
            run.last = line;
        } else if self.runs.len() < MAX_FORMULA_RUNS && self.left_out == 0 {
            self.runs.push(Run {
                formula,
                first: line,
                last: line,
            });
        } else {
            self.left_out += 1;
        }
    }
}

/// A formula with each cell reference to its own row, such as `J2` in row 2's `=3.28*J2`, written
/// `J{row}`: a reference is one to three letters, an optional `$`, and the row's number, with no
/// letter or digit before it and none, nor a `(`, after it, so a function such as `LOG10(` isn't
/// one.
fn own_row(formula: &str, row: usize) -> String {
    let row = row.to_string();
    let bytes = formula.as_bytes();
    let mut out = String::with_capacity(formula.len());
    let mut at = 0;
    while at < bytes.len() {
        let letters = bytes[at..]
            .iter()
            .take_while(|b| b.is_ascii_uppercase())
            .count();
        let starts = at == 0 || !bytes[at - 1].is_ascii_alphanumeric();
        if letters == 0 || !starts {
            // Copies one character, whole.
            let next = formula[at..].chars().next().map_or(1, char::len_utf8);
            out.push_str(&formula[at..at + next]);
            at += next;
            continue;
        }
        let mut end = at + letters;
        let dollar = bytes.get(end) == Some(&b'$');
        if dollar {
            end += 1;
        }
        let digits = bytes[end..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        let after = bytes.get(end + digits);
        let ends = after.is_none_or(|b| !b.is_ascii_alphanumeric() && *b != b'(');
        if letters <= 3 && digits > 0 && ends && formula[end..end + digits] == row {
            out.push_str(&formula[at..end]);
            out.push_str("{row}");
        } else {
            out.push_str(&formula[at..end + digits]);
        }
        at = end + digits;
    }
    out
}

/// The rows read so far, column by column.
struct Table {
    /// Each kept row's time, in the file's order.
    time_s: Vec<f64>,
    /// Each kept row's line in the file.
    lines: Vec<usize>,
    /// Each column's values, row by row, as written (no unit converted yet); empty for the clock
    /// and text columns.
    values: Vec<Vec<f64>>,
    /// Each column's count of cells that were empty or not a finite number, formulas apart.
    gaps: Vec<usize>,
    /// Each column's formula cells.
    formulas: Vec<Formulas>,
    /// Each kept row's `state_name`, if the file has the column.
    states: Vec<String>,
    /// The index of the `time` column.
    clock: usize,
    /// The indices of the `state` and `state_name` columns.
    state: Option<usize>,
    state_name: Option<usize>,
    /// The rows whose `state` and `state_name` are a pair AltOS's exports write.
    states_checked: usize,
    /// Whether the header is a table's with no `version,serial,flight` ([`is_table_header`]),
    /// whose states are the proof that it is AltOS's: a row contradicting them is refused.
    bare: bool,
    /// Otherwise, the rows whose `state` and `state_name` contradict AltOS's pair, and the first
    /// of them, its line and why.
    states_differing: usize,
    first_differing: Option<String>,
    /// The indices of the `serial` and `flight` columns.
    serial: Option<usize>,
    flight: Option<usize>,
    /// The first kept row's `serial` and `flight` cells, as written.
    first_serial: Option<String>,
    first_flight: Option<String>,
    /// The rows read so far, and the cells they wrote, for [`crate::log::too_sparse`].
    rows: usize,
    cells_written: usize,
}

impl Table {
    fn new(names: &[String], columns: &[Column], bare: bool) -> Self {
        Self {
            time_s: Vec::new(),
            lines: Vec::new(),
            values: vec![Vec::new(); columns.len()],
            gaps: vec![0; columns.len()],
            formulas: vec![Formulas::default(); columns.len()],
            states: Vec::new(),
            // `header_columns` has refused a header without a `time` column.
            clock: named(names, "time").unwrap_or(0),
            state: named(names, "state"),
            state_name: named(names, "state_name"),
            states_checked: 0,
            bare,
            states_differing: 0,
            first_differing: None,
            serial: named(names, "serial"),
            flight: named(names, "flight"),
            first_serial: None,
            first_flight: None,
            rows: 0,
            cells_written: 0,
        }
    }

    /// Reads one data row.
    fn row(
        &mut self,
        columns: &[Column],
        line: &str,
        number: usize,
        notes: &mut Notes,
    ) -> Result<(), LogError> {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() > columns.len() && cells[columns.len()..].iter().any(|c| !c.is_empty()) {
            return Err(LogError::Syntax {
                format: FORMAT,
                line: number,
                message: format!(
                    "the row has {} cells and the header names {}",
                    cells.len(),
                    columns.len()
                ),
            });
        }
        let cell = |index: Option<usize>| index.and_then(|index| cells.get(index).copied());
        let Some(time) = cell(Some(self.clock)).and_then(number_in) else {
            notes.push(format!(
                "line {number} has no time and was left out: {:?}",
                short(line)
            ));
            return Ok(());
        };
        if let (Some(state), Some(name)) = (cell(self.state), cell(self.state_name)) {
            match state_pair(state, name) {
                Ok(checked) => self.states_checked += usize::from(checked),
                Err(message) if self.bare => {
                    return Err(LogError::Syntax {
                        format: FORMAT,
                        line: number,
                        message,
                    });
                }
                Err(message) => {
                    self.states_differing += 1;
                    self.first_differing
                        .get_or_insert_with(|| format!("line {number}: {message}"));
                }
            }
        }
        self.rows += 1;
        self.cells_written += cells.len().min(columns.len());
        if crate::log::too_sparse(columns.len(), self.rows, self.cells_written) {
            return Err(LogError::Syntax {
                format: FORMAT,
                line: number,
                message: format!(
                    "{} rows hold {} of the {} cells the header's {} columns name: too few for \
                     the header to be this file's",
                    self.rows,
                    self.cells_written,
                    self.rows.saturating_mul(columns.len()),
                    columns.len()
                ),
            });
        }
        if cells.len() < columns.len() {
            notes.push(format!(
                "line {number} has {} cells of the header's {}; the rest read as gaps",
                cells.len(),
                columns.len()
            ));
        }
        if self.time_s.is_empty() {
            self.first_serial = cell(self.serial).filter(|s| !s.is_empty()).map(short);
            self.first_flight = cell(self.flight).filter(|s| !s.is_empty()).map(short);
        }
        self.time_s.push(time);
        self.lines.push(number);
        for (index, column) in columns.iter().enumerate() {
            if matches!(column.role, Role::Clock | Role::Text) {
                continue;
            }
            // A cell a short row leaves out is a gap its row's note counts, not the column's; a
            // formula is text, kept in its column's note.
            let value = match cells.get(index) {
                Some(cell) if cell.starts_with('=') => {
                    self.formulas[index].add(cell, number);
                    f64::NAN
                }
                Some(cell) => number_in(cell).unwrap_or_else(|| {
                    self.gaps[index] += 1;
                    f64::NAN
                }),
                None => f64::NAN,
            };
            self.values[index].push(value);
        }
        if self.state_name.is_some() {
            self.states
                .push(cell(self.state_name).map(short).unwrap_or_default());
        }
        Ok(())
    }
}

/// A cell's value: a finite number, or `None`.
fn number_in(cell: &str) -> Option<f64> {
    cell.parse::<f64>().ok().filter(|value| value.is_finite())
}

/// The lowest of a pad's pressures, Pa, that this reader takes a column for: below any launch
/// site, and a hundred times the highest pressure in hPa below.
const PASCAL_LOW: f64 = 10_000.0;
/// The highest pressure, Pa, that this reader takes a column for: above any on Earth's surface.
const PASCAL_HIGH: f64 = 120_000.0;

/// How a pressure column's values read, from its highest: Pa (a factor of 1), hPa (100), or
/// neither (`None`).
fn pressure_factor(values: &[f64]) -> Option<f64> {
    let highest = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold(f64::NEG_INFINITY, f64::max);
    if (PASCAL_LOW..=PASCAL_HIGH).contains(&highest) {
        Some(1.0)
    } else if (PASCAL_LOW / 100.0..=PASCAL_HIGH / 100.0).contains(&highest) {
        Some(100.0)
    } else {
        None
    }
}

/// A column's formulas as a note gives them: each run's formula and its lines, then any cells
/// past the runs given.
fn formula_runs(formulas: &Formulas) -> String {
    let mut runs: Vec<String> = formulas
        .runs
        .iter()
        .map(|run| {
            let cells = run.last - run.first + 1;
            let lines = if cells == 1 {
                format!("line {}", run.first)
            } else {
                format!("lines {}–{}", run.first, run.last)
            };
            format!("`{}` on {lines}", run.formula)
        })
        .collect();
    if formulas.left_out > 0 {
        runs.push(format!(
            "{} more past them",
            count(formulas.left_out, "cell")
        ));
    }
    runs.join("; ")
}

/// Builds the log from the rows.
///
/// # Errors
///
/// [`LogError::Syntax`] for a pressure in hPa that the factor of 100 makes infinite.
fn assemble(
    names: &[String],
    columns: &[Column],
    mut table: Table,
    preamble: &Preamble,
    line_notes: Notes,
    bare_table: bool,
) -> Result<FlightLog, LogError> {
    let mut notes: Vec<String> = Vec::new();
    let mut column_notes = Notes::default();
    let n = table.time_s.len();
    // A column by its name, and its role: a unit written after the name that differs from this
    // reader's makes the column another's.
    let find = |wanted: &str, role: Role| {
        named(names, wanted).filter(|&index| columns[index].role == role)
    };
    if let Some(first) = &table.first_differing {
        notes.push(format!(
            "{} a `state` and a `state_name` that aren't the pair AltOS's exports write, the \
             first at {first}; the events take the names as written",
            match table.states_differing {
                1 => "1 row gives".to_owned(),
                n => format!("{n} rows give"),
            }
        ));
    }
    if bare_table {
        notes.push(format!(
            "the header names AltOS's columns with no `version,serial,flight` before them. It is \
             read as AltOS's, as {} give a `state` and a `state_name` that AltOS's exports \
             write for one state, and none contradicts them",
            count(table.states_checked, "row")
        ));
    }

    // GPS positions of exactly (0, 0) are the receiver's before a fix.
    if let (Some(lat), Some(lon)) = (
        find("latitude", Role::Value(Quantity::Latitude)),
        find("longitude", Role::Value(Quantity::Longitude)),
    ) {
        let gnss: Vec<usize> = (0..columns.len())
            .filter(|&index| columns[index].role == Role::Value(Quantity::GnssAltitude))
            .collect();
        let mut blanked = 0;
        for row in 0..n {
            if table.values[lat][row] == 0.0 && table.values[lon][row] == 0.0 {
                blanked += 1;
                for &index in gnss.iter().chain([&lat, &lon]) {
                    table.values[index][row] = f64::NAN;
                }
            }
        }
        if blanked > 0 {
            notes.push(format!(
                "{} placing the GPS at latitude 0, longitude 0 (AltOS's position before its \
                 receiver has a fix). Their latitude, longitude and GPS altitude read as gaps",
                count(blanked, "row")
            ));
        }
    }

    let mut channels = Vec::with_capacity(columns.len());
    let mut converted: Vec<Option<Vec<f64>>> = vec![None; columns.len()];
    let mut units_agreeing = Vec::new();
    let mut formula_notes: Vec<String> = Vec::new();
    let mut formula_columns_left_out = 0;
    let mut formula_note = |note: String| {
        if formula_notes.len() < MAX_FORMULA_COLUMNS {
            formula_notes.push(note);
        } else {
            formula_columns_left_out += 1;
        }
    };
    for (index, column) in columns.iter().enumerate() {
        let name = &names[index];
        let mut values = std::mem::take(&mut table.values[index]);
        let formulas = &table.formulas[index];
        if formulas.cells > 0 && formulas.cells == n {
            // Every cell a formula: text, not a channel.
            formula_note(format!(
                "the `{}` column's {} a spreadsheet's {}, not {}. The column is not a channel. \
                 Each formula is kept here as text, not worked out. `{{row}}` stands for each \
                 cell's own row: {}",
                short(name),
                match formulas.cells {
                    1 => "1 cell is".to_owned(),
                    cells => format!("{cells} cells are"),
                },
                if formulas.cells == 1 {
                    "formula"
                } else {
                    "formulas"
                },
                if formulas.cells == 1 {
                    "a value"
                } else {
                    "values"
                },
                formula_runs(formulas)
            ));
            continue;
        }
        if formulas.cells > 0 {
            formula_note(format!(
                "the `{}` column has {} holding a spreadsheet's formula, not a value. The cells \
                 read as gaps, and the formulas are kept here as text. `{{row}}` stands for \
                 each cell's own row: {}",
                short(name),
                count(formulas.cells, "cell"),
                formula_runs(formulas)
            ));
        }
        let quantity = match column.role {
            Role::Clock | Role::Text => continue,
            Role::Value(quantity) => quantity,
            Role::Celsius => {
                for value in &mut values {
                    *value += 273.15;
                }
                Quantity::Temperature
            }
            Role::Pressure => match pressure_factor(&values) {
                Some(factor) if factor != 1.0 && column.unit == Unit::Agrees => {
                    notes.push(format!(
                        "the `{}` column states Pa, but its highest value is a pad's pressure \
                         in hPa; it is kept as written",
                        short(name)
                    ));
                    Quantity::Unrecognized
                }
                Some(factor) => {
                    if factor != 1.0 {
                        // A finite cell that the factor makes infinite, such as `-1.7e308` hPa,
                        // is refused on its row's line (ADR-258 §3).
                        for (value, &line) in values.iter_mut().zip(&table.lines) {
                            *value = finite(*value * factor, FORMAT, line, name)?;
                        }
                        notes.push(format!(
                            "the `{}` column never reaches 1,200 and was read as hPa, not Pa",
                            short(name)
                        ));
                    }
                    Quantity::Pressure
                }
                None => {
                    notes.push(format!(
                        "the `{}` column's highest value isn't a pad's pressure in Pa or hPa; it \
                         is kept as written",
                        short(name)
                    ));
                    Quantity::Unrecognized
                }
            },
        };
        match column.unit {
            Unit::Unwritten => {}
            Unit::Agrees => units_agreeing.push(format!("`{}`", short(name))),
            Unit::Differs if column.known => column_notes.push(format!(
                "the header writes `{}`, a unit after AltOS's `{}` that isn't the one this \
                 reader reads it in; its values are kept as written",
                short(name),
                short(split_unit(name).0)
            )),
            Unit::Differs => {}
        }
        if !column.known {
            column_notes.push(format!(
                "the column `{}` isn't one this reader knows; its values are kept as written",
                short(name)
            ));
        }
        if table.gaps[index] > 0 {
            column_notes.push(format!(
                "the `{}` column has {} empty or not a number (AltosUI writes `NaN` where it \
                 has no value), read as gaps",
                short(name),
                count(table.gaps[index], "cell")
            ));
        }
        converted[index] = Some(values.clone());
        channels.push(Channel {
            column: name.clone(),
            quantity,
            time_s: table.time_s.clone(),
            values,
        });
    }
    if !units_agreeing.is_empty() {
        notes.push(format!(
            "the header writes a unit after AltOS's name of {}: each the unit this reader reads \
             the column in, so read as AltOS's",
            units_agreeing.join(", ")
        ));
    }
    if find("acceleration", Role::Value(Quantity::Acceleration)).is_some() {
        notes.push(
            "AltOS writes `acceleration` with gravity taken out: it reads about 0 at rest, not \
             1 g; the channel is kept as written"
                .to_owned(),
        );
    }

    // The primary track: in time order, the first row at each time; none without a `height`, or
    // with one of formulas alone, which isn't a channel.
    let named_height = find("height", Role::Value(Quantity::AltitudeAboveGround));
    let height = named_height.filter(|&index| converted[index].is_some());
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| table.time_s[a].total_cmp(&table.time_s[b]));
    let mut latest = f64::NEG_INFINITY;
    let out_of_order = table
        .time_s
        .iter()
        .filter(|&&time| {
            let early = time < latest;
            latest = latest.max(time);
            early
        })
        .count();
    let mut kept: Vec<usize> = Vec::with_capacity(n);
    for row in order {
        match kept.last() {
            Some(&last) if table.time_s[last] == table.time_s[row] => {}
            _ => kept.push(row),
        }
    }
    if height.is_none() {
        kept.clear();
        notes.push(if named_height.is_some() {
            "the `height` column, AltOS's height above the pad, holds a spreadsheet's formulas \
             alone: the record has no primary track, only its channels"
                .to_owned()
        } else {
            "the header names no `height`, AltOS's height above the pad: the record has no \
             primary track, only its channels"
                .to_owned()
        });
    } else {
        if out_of_order > 0 {
            notes.push(format!(
                "{} coming before an earlier row's time: the primary track takes the rows in \
                 time order; the channels keep the file's",
                count(out_of_order, "row")
            ));
        }
        if kept.len() < n {
            notes.push(format!(
                "{} repeating an earlier row's time: the primary track keeps the first row at \
                 each time; the channels keep every row",
                count(n - kept.len(), "row")
            ));
        }
    }
    let primary = |index: Option<usize>| {
        index
            .filter(|_| height.is_some())
            .and_then(|index| converted[index].as_ref())
            .map(|values| kept.iter().map(|&row| values[row]).collect::<Vec<f64>>())
    };
    // The primary track is barometric: `accel_speed` integrates the accelerometer, and `speed`
    // is the flight computer's own estimate, which uses its accelerometer on the way up.
    let speed = find("baro_speed", Role::Value(Quantity::Speed));

    let mut events: Vec<Event> = Vec::new();
    let mut state: Option<&str> = None;
    for (row, name) in table.states.iter().enumerate() {
        if !name.is_empty() && state != Some(name.as_str()) {
            events.push(Event {
                time_s: table.time_s[row],
                name: name.clone(),
            });
            state = Some(name.as_str());
        }
    }

    let serial_number = preamble
        .serial
        .clone()
        .or_else(|| table.first_serial.clone());
    if let (Some(stated), Some(written)) = (&preamble.serial, &table.first_serial)
        && stated != written
    {
        notes.push(format!(
            "the comment lines give the serial number as {stated:?} and the rows as {written:?}; \
             the comment lines' is kept"
        ));
    }
    let flight = preamble.flight.as_ref().or(table.first_flight.as_ref());
    let flight_number = flight.and_then(|flight| flight.parse().ok());
    if flight.is_some() && flight_number.is_none() {
        notes.push(format!(
            "the flight number {:?} isn't a whole number and was left out",
            flight.map(String::as_str).unwrap_or_default()
        ));
    }

    notes.extend(formula_notes);
    if formula_columns_left_out > 0 {
        notes.push(format!(
            "and {} holding formulas, past the {MAX_FORMULA_COLUMNS} whose formulas are given",
            count(formula_columns_left_out, "more column")
        ));
    }
    notes.extend(column_notes.kept);
    notes.extend(line_notes.kept);
    let left_out = column_notes.left_out + line_notes.left_out;
    if left_out > 0 {
        notes.push(format!(
            "and {left_out} more notes about single columns and lines, left out so that a \
             broken file's list stays short"
        ));
    }

    Ok(FlightLog {
        format: LogFormat::AltosCsv,
        logger: match &preamble.product {
            Some(product) => format!("AltOS {product}"),
            None => "AltOS".to_owned(),
        },
        serial_number,
        firmware: None,
        flight_number,
        stated: Stated::default(),
        time_s: kept.iter().map(|&row| table.time_s[row]).collect(),
        altitude_m: primary(height).unwrap_or_default(),
        vertical_speed_m_s: primary(speed),
        temperature_k: primary(find("temperature", Role::Celsius)),
        battery_v: primary(find("battery_voltage", Role::Value(Quantity::Voltage))),
        channels,
        events,
        notes,
    })
}

/// A count and its noun, such as `1 row` or `3 rows`.
fn count(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("1 {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

/// Text from the file as an error or note quotes it: cut to [`QUOTED_CHARS`] characters, so a
/// binary file's "first line" doesn't fill the message.
fn short(text: &str) -> String {
    cut(text, QUOTED_CHARS)
}

/// Text cut to a number of characters, its last a `…` where it is cut.
fn cut(text: &str, chars: usize) -> String {
    if text.chars().count() <= chars {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(chars - 1).collect();
        cut.push('…');
        cut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = "#version,serial,flight,call,time,rssi,lqi,state,state_name,acceleration,\
                        pressure,altitude,height,baro_speed,temperature,battery_voltage\n";

    /// A row, its state's number the one AltOS's exports give its name.
    fn row(time: f64, state: &str, height: f64) -> String {
        let number = crate::altos_telem::STATES
            .iter()
            .find(|(_, name)| *name == state)
            .map_or(2, |(number, _)| *number);
        format!(
            "6, 77, 4,N0CALL, {time:8.2}, -80, 40, {number}, {state:>7}, 0.10, 98000.00, \
             {:.2}, {height:.2}, 1.50, 20.0, 3.90\n",
            250.0 + height
        )
    }

    /// Every column a channel in the file's order, but the clock and the text; the primary
    /// track from `height`, `baro_speed`, `temperature` and `battery_voltage`, in SI.
    #[test]
    fn reads_columns_and_the_primary_track() {
        let text = format!("{HEAD}{}{}", row(-0.5, "pad", 0.0), row(0.0, "boost", 1.25));
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::AltosCsv);
        assert_eq!(log.logger, "AltOS");
        assert_eq!(log.serial_number.as_deref(), Some("77"));
        assert_eq!(log.flight_number, Some(4));
        assert_eq!(log.firmware, None);
        assert_eq!(log.time_s, [-0.5, 0.0]);
        assert_eq!(log.altitude_m, [0.0, 1.25]);
        assert_eq!(log.vertical_speed_m_s.as_deref(), Some(&[1.5, 1.5][..]));
        assert_eq!(log.temperature_k.as_deref(), Some(&[293.15, 293.15][..]));
        assert_eq!(log.battery_v.as_deref(), Some(&[3.9, 3.9][..]));
        let names: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
        assert_eq!(
            names,
            [
                "version",
                "serial",
                "flight",
                "rssi",
                "lqi",
                "state",
                "acceleration",
                "pressure",
                "altitude",
                "height",
                "baro_speed",
                "temperature",
                "battery_voltage"
            ]
        );
        let pressure = log.channels_of(Quantity::Pressure).next().unwrap();
        assert_eq!(pressure.values, [98000.0, 98000.0]);
        assert_eq!(log.channels_of(Quantity::AltitudeMsl).count(), 1);
        assert_eq!(
            log.events,
            [
                Event {
                    time_s: -0.5,
                    name: "pad".to_owned()
                },
                Event {
                    time_s: 0.0,
                    name: "boost".to_owned()
                }
            ]
        );
        assert_eq!(log.notes.len(), 1, "{:?}", log.notes);
        assert!(log.notes[0].contains("gravity taken out"));
    }

    /// The second `altitude`, after `longitude`, is the GPS receiver's; a (0, 0) position is
    /// no fix, and its row's latitude, longitude and GPS altitude are gaps.
    #[test]
    fn the_second_altitude_is_the_gps_and_zero_zero_is_no_fix() {
        let text = "#version,serial,flight,time,state_name,altitude,height,latitude,longitude,\
                    altitude,nsat,sat01,pad_dist,pad_az,tilt,gyro_roll\n\
                    6,1,1,0.0,pad,250,0,0,0,0,0,0,0,0,NaN,1\n\
                    6,1,1,0.5,pad,250,0,40.5,-105.25,251.5,7,31,3,90,2.5,1\n";
        let log = read(text).unwrap();
        let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
        use Quantity as Q;
        assert_eq!(
            quantities,
            [
                Q::Number,
                Q::Number,
                Q::Number,
                Q::AltitudeMsl,
                Q::AltitudeAboveGround,
                Q::Latitude,
                Q::Longitude,
                Q::GnssAltitude,
                Q::Number,
                Q::Number,
                Q::Distance,
                Q::Angle,
                Q::Angle,
                Q::Unrecognized
            ]
        );
        let gnss = log.channels_of(Q::GnssAltitude).next().unwrap();
        assert_eq!(gnss.column, "altitude");
        assert!(gnss.values[0].is_nan());
        assert_eq!(gnss.values[1], 251.5);
        let latitude = log.channels_of(Q::Latitude).next().unwrap();
        assert!(latitude.values[0].is_nan());
        assert_eq!(latitude.values[1], 40.5);
        // The barometric altitude on the same row is untouched.
        let baro = log.channels_of(Q::AltitudeMsl).next().unwrap();
        assert_eq!(baro.values, [250.0, 250.0]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("1 row placing the GPS")),
            "{:?}",
            log.notes
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("`tilt` column has 1 cell empty")),
            "{:?}",
            log.notes
        );
        // No speed, temperature or battery column: none on the primary track.
        assert!(log.vertical_speed_m_s.is_none() && log.temperature_k.is_none());
        assert!(log.battery_v.is_none());
    }

    /// Repeated times: the primary track keeps the first row at each, the channels every row.
    /// Rows out of order are put in order on the primary track, and both are noted.
    #[test]
    fn repeated_and_reordered_times() {
        let text = format!(
            "{HEAD}{}{}{}{}",
            row(0.0, "pad", 0.0),
            row(0.5, "boost", 5.0),
            row(0.5, "boost", 6.0),
            row(0.25, "boost", 2.0)
        );
        let log = read(&text).unwrap();
        assert_eq!(log.time_s, [0.0, 0.25, 0.5]);
        assert_eq!(log.altitude_m, [0.0, 2.0, 5.0]);
        let height = log
            .channels_of(Quantity::AltitudeAboveGround)
            .next()
            .unwrap();
        assert_eq!(height.time_s, [0.0, 0.5, 0.5, 0.25]);
        assert_eq!(height.values, [0.0, 5.0, 6.0, 2.0]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("1 row coming before")),
            "{:?}",
            log.notes
        );
        assert!(
            log.notes.iter().any(|n| n.starts_with("1 row repeating")),
            "{:?}",
            log.notes
        );
        // One event per change of state, in the file's order.
        let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["pad", "boost"]);
    }

    /// Comment lines before a header without `#`: product, serial and flight from them.
    #[test]
    fn comment_lines_name_the_logger() {
        let text = "\u{feff}# Altus Metrum flight log\r\n# product TeleMetrum\r\n# serial 2001\r\n\
                    # flight 3\r\n# call N0CALL\r\nversion,serial,flight,time,state_name,height\r\n\
                    5,2002,3,0.00,pad,0.0\r\n";
        let log = read(text).unwrap();
        assert_eq!(log.logger, "AltOS TeleMetrum");
        assert_eq!(log.serial_number.as_deref(), Some("2001"));
        assert_eq!(log.flight_number, Some(3));
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("\"2001\" and the rows as \"2002\"")),
            "{:?}",
            log.notes
        );
    }

    /// A pressure near the lowest `f64`, in a column read as hPa, is finite as written and
    /// infinite in pascals: refused on its line (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let head = "#version,serial,flight,time,state_name,pressure,height\n";
        let text = format!("{head}5,1,1,0,pad,983.25,0\n5,1,1,1,pad,-1.7e308,10\n");
        assert_eq!(
            read(&text).unwrap_err().to_string(),
            "AltOS CSV line 3: the column pressure holds a value too large for its unit's \
             conversion to SI"
        );
    }

    /// A pressure column in hPa is converted; one that is neither Pa nor hPa is kept as written.
    #[test]
    fn pressure_unit_from_its_highest_value() {
        let head = "#version,serial,flight,time,state_name,pressure,height\n";
        let log = read(&format!(
            "{head}5,1,1,0,pad,983.25,0\n5,1,1,1,pad,950.5,10\n"
        ))
        .unwrap();
        let pressure = log.channels_of(Quantity::Pressure).next().unwrap();
        assert_eq!(pressure.values, [98325.0, 95050.0]);
        assert!(
            log.notes.iter().any(|n| n.contains("read as hPa")),
            "{:?}",
            log.notes
        );

        let log = read(&format!("{head}5,1,1,0,pad,98.3,0\n")).unwrap();
        assert_eq!(log.channels_of(Quantity::Pressure).count(), 0);
        let kept = log
            .channels
            .iter()
            .find(|c| c.column == "pressure")
            .unwrap();
        assert_eq!(
            (kept.quantity, kept.values[0]),
            (Quantity::Unrecognized, 98.3)
        );

        // The boundaries: 1,200 hPa and 10,000 Pa are each still their unit.
        assert_eq!(pressure_factor(&[1200.0, f64::NAN]), Some(100.0));
        assert_eq!(pressure_factor(&[10_000.0]), Some(1.0));
        assert_eq!(pressure_factor(&[120_000.0]), Some(1.0));
        assert_eq!(pressure_factor(&[1200.5]), None);
        assert_eq!(pressure_factor(&[120_000.5]), None);
        assert_eq!(pressure_factor(&[f64::NAN]), None);
    }

    /// Short rows have gaps; a long row of empty cells is a trailing comma; a row with no time
    /// is left out; an unknown column is kept and noted.
    #[test]
    fn odd_rows_and_columns() {
        let text = "#version,serial,flight,time,state_name,height,widget\n\
                    6,1,1,0.0,pad,0,5,,\n\
                    6,1,1,,pad,0,5\n\
                    6,1,1,1.0,boost,3\n";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [0.0, 1.0]);
        let widget = log.channels.iter().find(|c| c.column == "widget").unwrap();
        assert_eq!(widget.quantity, Quantity::Unrecognized);
        assert_eq!(widget.values[0], 5.0);
        assert!(widget.values[1].is_nan());
        let expected = [
            "the column `widget` isn't one this reader knows",
            "line 3 has no time and was left out",
            "line 4 has 6 cells of the header's 7",
        ];
        for text in expected {
            assert!(
                log.notes.iter().any(|n| n.starts_with(text)),
                "{text}: {:?}",
                log.notes
            );
        }
        // The short row's missing cell is counted by its line's note, not as the column's gap.
        assert_eq!(log.notes.len(), expected.len(), "{:?}", log.notes);
    }

    /// A file of many broken rows keeps a short list of notes and counts the rest.
    #[test]
    fn notes_stay_few() {
        let text = format!(
            "#version,serial,flight,time,height\n{}0,1,1,0,0\n",
            "0,1,1,x,0\n".repeat(1000)
        );
        let log = read(&text).unwrap();
        assert_eq!(log.notes.len(), MAX_NOTES + 1);
        assert!(
            log.notes[MAX_NOTES].starts_with("and 980 more notes"),
            "{:?}",
            log.notes
        );
    }

    /// Each refusal says what is wrong, and where.
    #[test]
    fn malformed_files_are_refused() {
        let cases = [
            ("", "not read as AltOS CSV: the file has no"),
            ("# just a comment\n", "has no `version,serial,flight"),
            (
                "time,height\n0,0\n",
                "line 1, \"time,height\", isn't AltOS's header",
            ),
            ("PerfectFlite Pnut\n", "isn't AltOS's header"),
            (
                "#version,serial,flight,time (ms)\n0,0,0,0\n",
                "line 1: the header writes a unit after `time` other than s",
            ),
            (
                "#version,serial,flight,height\n0,0,0,0\n",
                "line 1: the header names no `time`",
            ),
            ("#version,serial,flight,time,height\n", "no data rows"),
            (
                "#version,serial,flight,time,height\n0,0,0,x,0\n",
                "no data rows",
            ),
            (
                "#version,serial,flight,time,height\n0,0,0,0,0,7\n",
                "line 2: the row has 6 cells and the header names 5",
            ),
        ];
        for (text, expected) in cases {
            let error = read(text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
        let junk = "\u{fffd}".repeat(100_000);
        let error = read(&junk).unwrap_err().to_string();
        assert!(error.len() < 400 && error.contains('…'), "{error}");
    }

    /// A header without `height` reads, with no primary track and every column a channel.
    #[test]
    fn no_height_no_primary_track() {
        let log = read("#version,serial,flight,time,altitude\n6,77,4,0.5,250\n").unwrap();
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        assert!(log.temperature_k.is_none() && log.battery_v.is_none());
        let altitude = log
            .channels
            .iter()
            .find(|c| c.column == "altitude")
            .unwrap();
        assert_eq!(
            (altitude.quantity, altitude.values.as_slice()),
            (Quantity::AltitudeMsl, &[250.0][..])
        );
        assert!(log.notes.iter().any(|n| n.contains("no primary track")));
    }

    /// A unit after AltOS's name that is this reader's reads the column as AltOS's; another
    /// keeps it as written; a pressure stated in Pa whose values are hPa is kept as written.
    #[test]
    fn units_after_the_names() {
        let text = "#version,serial,flight,time,height (m),acceleration (m/s^2),accel_x (m/s),\
                    pressure (Pa)\n6,77,4,0.5,10,1.5,9.8,980\n";
        let log = read(text).unwrap();
        let quantity = |column: &str| {
            log.channels
                .iter()
                .find(|c| c.column == column)
                .map(|c| c.quantity)
        };
        assert_eq!(log.altitude_m, [10.0]);
        assert_eq!(
            quantity("acceleration (m/s^2)"),
            Some(Quantity::Acceleration)
        );
        assert_eq!(quantity("accel_x (m/s)"), Some(Quantity::Unrecognized));
        assert_eq!(quantity("pressure (Pa)"), Some(Quantity::Unrecognized));
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("`accel_x (m/s)`, a unit"))
        );
        let feet = read("#version,serial,flight,time,height (ft)\n6,77,4,0.5,10\n").unwrap();
        assert!(
            feet.altitude_m.is_empty(),
            "a height in feet isn't AltOS's height"
        );
    }

    /// A `height` of formulas alone isn't a channel, so the record has no primary track, as with
    /// none: the clock and the track stay the same length.
    #[test]
    fn a_height_of_formulas_alone_is_no_primary_track() {
        let text = "#version,serial,flight,time,height,altitude\n\
                    6,77,4,0.5,=F2-F$2,250\n\
                    6,77,4,0.6,=F3-F$2,251\n";
        let log = read(text).unwrap();
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        assert!(log.channels.iter().all(|c| c.column != "height"));
        assert!(
            log.notes.iter().any(|n| n.starts_with(
                "the `height` column, AltOS's height above the pad, holds a spreadsheet's \
                 formulas alone"
            )),
            "{:#?}",
            log.notes
        );
    }

    /// One formula cell is written of in the singular.
    #[test]
    fn one_formula_cell_reads_in_the_singular() {
        let log = read("#version,serial,flight,time,height,x\n6,77,4,0.5,0,=E2*2\n").unwrap();
        assert!(
            log.notes.iter().any(|n| n.starts_with(
                "the `x` column's 1 cell is a spreadsheet's formula, not a value. The column"
            )),
            "{:#?}",
            log.notes
        );
    }

    /// The formula notes' bounds hold at their limits and count what is past them: at most
    /// [`MAX_FORMULA_RUNS`] runs a column and [`MAX_FORMULA_COLUMNS`] columns.
    #[test]
    fn the_formula_bounds_hold_at_their_limits() {
        let runs = |extra: usize| {
            let mut text = String::from("#version,serial,flight,time,height,x\n");
            for k in 0..MAX_FORMULA_RUNS + extra {
                text += &format!("6,77,4,{k}.0,{k},=E{}*{k}\n", k + 2);
            }
            let log = read(&text).unwrap();
            log.notes
                .into_iter()
                .find(|n| n.starts_with("the `x` column's"))
                .unwrap()
        };
        let at = runs(0);
        assert_eq!(at.matches("` on line ").count(), MAX_FORMULA_RUNS, "{at}");
        assert!(!at.contains("past them"), "{at}");
        let past = runs(2);
        assert_eq!(
            past.matches("` on line ").count(),
            MAX_FORMULA_RUNS,
            "{past}"
        );
        assert!(past.ends_with("; 2 cells more past them"), "{past}");

        let columns = |n: usize| {
            let names: Vec<String> = (0..n).map(|k| format!("x{k}")).collect();
            let cells: Vec<String> = (0..n).map(|k| format!("={k}")).collect();
            let text = format!(
                "#version,serial,flight,time,height,{}\n6,77,4,0.0,0,{}\n",
                names.join(","),
                cells.join(",")
            );
            read(&text).unwrap().notes
        };
        let given = |notes: &[String]| notes.iter().filter(|n| n.starts_with("the `x")).count();
        assert_eq!(given(&columns(MAX_FORMULA_COLUMNS)), MAX_FORMULA_COLUMNS);
        let past = columns(MAX_FORMULA_COLUMNS + 3);
        assert_eq!(given(&past), MAX_FORMULA_COLUMNS);
        assert!(
            past.iter()
                .any(|n| n.contains("past the 64 whose formulas are given")),
            "{past:#?}"
        );
    }

    /// A state a spreadsheet rewrote, `3.0`, is AltOS's `3`; and a table opening with a
    /// byte-order mark is still recognized.
    #[test]
    fn a_rewritten_state_and_a_byte_order_mark_still_read() {
        let text = "\u{feff}time,state,state_name,height\n0.0,3.0,boost,0\n0.1,5.0,coast,2\n";
        assert!(recognizes_table(text));
        let log = read(text).unwrap();
        let events: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(events, ["boost", "coast"]);
    }

    /// A formula is text: a column of them isn't a channel, and its note gives every formula,
    /// each cell's own row as `{row}`, run by run; a formula among numbers is a gap.
    #[test]
    fn formulas_are_kept_as_text() {
        let text = "#version,serial,flight,time,height,altitude (ft),(ft/s)\n\
                    6,77,4,0.0,0,=3.28084*E2,=3.28*E2\n\
                    6,77,4,0.5,10,=3.28*E3,1.5\n\
                    6,77,4,1.0,20,=3.28*E4,=3.28*E4\n\
                    6,77,4,1.5,30,=3.28*E5,=3.28*E5\n";
        let log = read(text).unwrap();
        assert_eq!(log.altitude_m, [0.0, 10.0, 20.0, 30.0]);
        assert!(log.channels.iter().all(|c| c.column != "altitude (ft)"));
        let note = |start: &str| {
            log.notes
                .iter()
                .find(|n| n.starts_with(start))
                .unwrap_or_else(|| panic!("{start}: {:#?}", log.notes))
        };
        let all = note("the `altitude (ft)` column's 4 cells are a spreadsheet's formulas");
        assert!(
            all.ends_with("`=3.28084*E{row}` on line 2; `=3.28*E{row}` on lines 3–5"),
            "{all}"
        );
        let mixed = note("the `(ft/s)` column has 3 cells holding");
        assert!(
            mixed.ends_with("`=3.28*E{row}` on line 2; `=3.28*E{row}` on lines 4–5"),
            "{mixed}"
        );
        let speed = log.channels.iter().find(|c| c.column == "(ft/s)").unwrap();
        assert_eq!(speed.values[1], 1.5);
        assert!(speed.values[0].is_nan() && speed.values[3].is_nan());
    }

    /// Only a reference to the cell's own row becomes `{row}`.
    #[test]
    fn own_row_rewrites_references_to_the_row_only() {
        assert_eq!(own_row("=3.28*J2", 2), "=3.28*J{row}");
        assert_eq!(own_row("=AL12-AL$2+J$12", 12), "=AL{row}-AL$2+J${row}");
        assert_eq!(own_row("=LOG10(J10)", 10), "=LOG10(J{row})");
        assert_eq!(own_row("=ABCD7+A7B", 7), "=ABCD7+A7B");
        assert_eq!(own_row("=J22*2", 2), "=J22*2");
        assert_eq!(own_row("=é+J3", 3), "=é+J{row}");
    }

    /// In a table with no `version` header, a row whose state number and name aren't AltOS's
    /// pair is refused at its line; a state AltOS's exports don't name passes unchecked.
    #[test]
    fn a_state_unlike_altoss_is_refused() {
        let head = "time,state,state_name,height\n-0.5,2,pad,0\n";
        let error = read(&format!("{head}0.0,3,ignition,0\n"))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(
                "line 3: the row's state 3 is named \"ignition\", where AltOS names 3 `boost`"
            ),
            "{error}"
        );
        let error = read(&format!("{head}0.0,4,boost,0\n"))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("line 3: the row's state 4 is named"),
            "{error}"
        );
        let error = read(&format!("{head}0.0,12,boost,0\n"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("where AltOS numbers `boost` 3"), "{error}");
        assert!(read(&format!("{head}0.0,9,invalid,0\n")).is_ok());
    }

    /// In an export with AltOS's `version,serial,flight` header, a state unlike AltOS's is
    /// counted in a note, its name kept for the events: one sample file numbers `pad` 1.
    #[test]
    fn an_exports_state_unlike_altoss_is_noted() {
        let text = format!(
            "{HEAD}{}{}",
            row(-0.5, "pad", 0.0).replacen(", 2,", ", 1,", 1),
            row(0.0, "boost", 1.25)
        );
        let log = read(&text).unwrap();
        let note = log
            .notes
            .iter()
            .find(|n| n.starts_with("1 row gives a `state`"))
            .unwrap_or_else(|| panic!("{:#?}", log.notes));
        assert!(
            note.contains("the first at line 2: the row's state \"1\" is named `pad`"),
            "{note}"
        );
        let events: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(events, ["pad", "boost"]);
    }

    /// A table cut from an export, AltOS's names with no `version` header, is read by its
    /// states; one with no state AltOS names, a unit after a name, a name AltOS doesn't write or
    /// comments before it, isn't.
    #[test]
    fn a_table_of_altoss_columns_reads_by_its_states() {
        let text = "time,state,state_name,acceleration,pressure,altitude,height,speed,temperature\n\
                    -0.10,3,boost,9.1,95300,512.7,5.2,15.8,23.7\n\
                    0.00,3,boost,10.0,95290,513.5,6.0,16.8,23.7\n\
                    0.10,5,coast,-9.8,95200,521.6,14.1,15.0,23.7\n";
        assert!(recognizes_table(text));
        assert_eq!(
            crate::import::detect(text.as_bytes()),
            Some(LogFormat::AltosCsv)
        );
        let log = read(text).unwrap();
        assert_eq!(log.altitude_m, [5.2, 6.0, 14.1]);
        assert_eq!(log.serial_number, None);
        let events: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(events, ["boost", "coast"]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("the header names AltOS's columns"))
        );
        let unknown = text
            .replace("3,boost", "1,idle")
            .replace("5,coast", "0,startup");
        assert!(!recognizes_table(&unknown));
        assert!(
            read(&unknown)
                .unwrap_err()
                .to_string()
                .contains("by its states")
        );
        assert!(!recognizes_table(&text.replace("speed,", "speed (m/s),")));
        assert!(!recognizes_table(&text.replace("speed,", "source,")));
        assert!(!recognizes_table(&text.replace("5,coast", "5,boost")));
        assert!(read(&format!("# product EasyMini\n{text}")).is_err());
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a strictly increasing
        /// primary track, and every channel as many times as values.
        #[test]
        fn any_text_is_read_or_refused(
            text in "(#version,serial,flight,time,height,state_name,altitude,longitude,altitude\n)?\
                     ([-0-9., NaN#a-z]{0,24}\n){0,12}"
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
