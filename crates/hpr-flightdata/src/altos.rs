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
//! first). The temperature is `temperature` and the battery `battery_voltage`. Both `time` and
//! `height` are required.
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
//! comment lines), where Debrief searches the first 60 lines for it. AltOS's radio-telemetry log
//! (`tick`, `ptype`, …) is a different file and isn't read here.

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
/// (`version,serial,flight,…`, with or without a leading `#`); [`LogError::Syntax`] for a header
/// without a `time` or a `height` column, a row with more cells than the header names, or a
/// pressure in hPa too large to convert to pascals; [`LogError::NoData`] if no row has a time.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()));

    let mut preamble = Preamble::default();
    let (header_line, header) = loop {
        match lines.next() {
            Some((_, "")) => {}
            Some((number, line)) => {
                let bare = line.trim_start_matches('#').trim();
                if is_header(bare) {
                    break (number, bare.split(',').map(|cell| cell.trim().to_owned()));
                }
                if line.starts_with('#') {
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
    let names: Vec<String> = header.collect();
    let columns = header_columns(&names, header_line)?;

    let mut notes = Notes::default();
    let mut table = Table::new(&names, &columns);
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
    assemble(&names, &columns, table, &preamble, notes)
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

/// A column of the header: its role, and whether the reader knows its name.
#[derive(Debug, Clone, Copy)]
struct Column {
    role: Role,
    known: bool,
}

/// Each header cell's column; refuses a header without `time` or `height`.
fn header_columns(names: &[String], number: usize) -> Result<Vec<Column>, LogError> {
    let mut after_longitude = false;
    let mut altitudes = 0;
    let columns: Vec<Column> = names
        .iter()
        .map(|name| {
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
            Column { role, known }
        })
        .collect();
    for required in ["time", "height"] {
        if !names.iter().any(|name| name.eq_ignore_ascii_case(required)) {
            return Err(LogError::Syntax {
                format: FORMAT,
                line: number,
                message: format!("the header names no `{required}` column"),
            });
        }
    }
    Ok(columns)
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
    /// Each column's count of cells that were empty or not a finite number.
    gaps: Vec<usize>,
    /// Each kept row's `state_name`, if the file has the column.
    states: Vec<String>,
    /// The index of the `time` column.
    clock: usize,
    /// The index of the `state_name` column.
    state_name: Option<usize>,
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
    fn new(names: &[String], columns: &[Column]) -> Self {
        let find = |wanted: &str| {
            names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(wanted))
        };
        Self {
            time_s: Vec::new(),
            lines: Vec::new(),
            values: vec![Vec::new(); columns.len()],
            gaps: vec![0; columns.len()],
            states: Vec::new(),
            // `header_columns` has refused a header without a `time` column.
            clock: find("time").unwrap_or(0),
            state_name: find("state_name"),
            serial: find("serial"),
            flight: find("flight"),
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
            // A cell a short row leaves out is a gap its row's note counts, not the column's.
            let value = match cells.get(index) {
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
) -> Result<FlightLog, LogError> {
    let mut notes: Vec<String> = Vec::new();
    let mut column_notes = Notes::default();
    let n = table.time_s.len();
    let find = |wanted: &str| {
        names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(wanted))
    };

    // GPS positions of exactly (0, 0) are the receiver's before a fix.
    if let (Some(lat), Some(lon)) = (find("latitude"), find("longitude")) {
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
                 receiver has a fix): their latitude, longitude and GPS altitude read as gaps",
                count(blanked, "row")
            ));
        }
    }

    let mut channels = Vec::with_capacity(columns.len());
    let mut converted: Vec<Option<Vec<f64>>> = vec![None; columns.len()];
    for (index, column) in columns.iter().enumerate() {
        let name = &names[index];
        let mut values = std::mem::take(&mut table.values[index]);
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
    if find("acceleration").is_some() {
        notes.push(
            "AltOS writes `acceleration` with gravity taken out: it reads about 0 at rest, not \
             1 g; the channel is kept as written"
                .to_owned(),
        );
    }

    // The primary track: in time order, the first row at each time.
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
    if out_of_order > 0 {
        notes.push(format!(
            "{} coming before an earlier row's time: the primary track takes the rows in time \
             order; the channels keep the file's",
            count(out_of_order, "row")
        ));
    }
    if kept.len() < n {
        notes.push(format!(
            "{} repeating an earlier row's time: the primary track keeps the first row at each \
             time; the channels keep every row",
            count(n - kept.len(), "row")
        ));
    }
    let primary = |index: Option<usize>| {
        index
            .and_then(|index| converted[index].as_ref())
            .map(|values| kept.iter().map(|&row| values[row]).collect::<Vec<f64>>())
    };
    // The primary track is barometric: `accel_speed` integrates the accelerometer, and `speed`
    // is the flight computer's own estimate, which uses its accelerometer on the way up.
    let speed = find("baro_speed");

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
        // `header_columns` has refused a header without a `height` column.
        altitude_m: primary(find("height")).unwrap_or_default(),
        vertical_speed_m_s: primary(speed),
        temperature_k: primary(find("temperature")),
        battery_v: primary(find("battery_voltage")),
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
    if text.chars().count() <= QUOTED_CHARS {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(QUOTED_CHARS - 1).collect();
        cut.push('…');
        cut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = "#version,serial,flight,call,time,rssi,lqi,state,state_name,acceleration,\
                        pressure,altitude,height,baro_speed,temperature,battery_voltage\n";

    fn row(time: f64, state: &str, height: f64) -> String {
        format!(
            "6, 77, 4,N0CALL, {time:8.2}, -80, 40, 2, {state:>7}, 0.10, 98000.00, \
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
                "#version,serial,flight,time\n0,0,0,0\n",
                "line 1: the header names no `height`",
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
