//! PerfectFlite's `.pf2` flight logs, as its software writes a Pnut's, a StratoLogger's or a
//! StratoLoggerCF's flights, the CSV and text exports it writes of them ([`read_export`]), and
//! the older `.pfa` of an Alt15K/WD's flights ([`read_pfa`]). Every
//! `.pf2` and `.pfa` among the project's sample logs reads, of all four models (`cargo xtask
//! logs`).
//!
//! A `.pf2` is text: the logger's name on the first line, a preamble of `Key: value` lines, a
//! `Data:` line naming the columns, then one row per sample, comma-separated, at about 20 Hz:
//!
//! ```text
//! PerfectFlite Pnut
//! Firmware: 1.0
//! Apogee: 1280' AGL
//! Ground Elevation: 600' MSL
//! NumSamps: 3
//! Flight Number: 2
//! Data: (Time, Altitude, Velocity, Temperature (F), Voltage)
//! 0.00, 0, 0, 70.00, 4.20
//! 0.05, 0, 0
//! 0.10, 1, 8, 70.00, 4.20
//! ```
//!
//! Times are seconds, altitudes feet above the logger's reading on the pad, speeds feet per
//! second, temperatures degrees Fahrenheit, voltages volts. A row may leave its last columns out
//! (the temperature and voltage are logged less often); a column left out is a gap, `NaN`. The
//! velocity is the logger's own, worked out from its barometric altitude: a PerfectFlite has no
//! accelerometer.
//!
//! **Where this comes from.** PerfectFlite publishes no specification of the format. This reader
//! follows the one in Debrief (`lib/parsers/perfectflite.ts`, MIT, the project owner's own; see
//! `THIRD-PARTY-NOTICES.md`), which was written from exported files and cites no document; the
//! units above are its reading, borne out by the Pnut fixture Debrief ships, whose preamble states
//! its apogee with a foot mark. Where this reader departs from Debrief's: it takes the column order
//! from the `Data:` line when there is one, where Debrief assumes it; and it refuses what Debrief
//! passes over, naming the line: an empty or non-numeric cell, a time that doesn't increase, and a
//! line that isn't a row once the rows have begun. A stated apogee or elevation in a unit other
//! than feet is refused rather than guessed at. A byte-order mark is dropped, and lines may end
//! in CR, LF or both.
//!
//! # The older `.pfa`
//!
//! A `.pfa` is what PerfectFlite's DataCap program (AltCapWD 1.10 on the Mac) saves of an
//! Alt15K/WD's flight. It is text, its lines ending in CR, or CR LF: the logger's name, four
//! `Key: value` lines, a blank line, a `**!!Data:` line, then one altitude a line, zero-padded to
//! five digits, and nothing else. An invented flight's first lines:
//!
//! ```text
//! PerfectFlite Alt15K/WD
//! Ground: 1150
//! Apogee: 274
//! NumSamps: 24
//! Comments:
//!
//! **!!Data:
//! 00000
//! 00002
//! ```
//!
//! [`import::read`](crate::import::read) tells it from a `.pf2` by the `**!!` line: both open
//! with a line naming PerfectFlite.
//!
//! **Units and clock.** The file states neither. The samples are feet above the logger's reading on
//! the pad, 0.1 s apart, so the record's time is the sample's place in the file over 10, from the
//! first sample:
//!
//! - *Alt15K/WD User's Manual* (PerfectFlite; archived 2013), p. 2: waiting on the pad, the logger
//!   samples "every 100 milliseconds"; once a sample is 80 ft above the ground reading, it saves
//!   "the previous 8 altitude samples" and adds one "every 100 milliseconds" for the rest of the
//!   flight. It beeps the apogee in feet (p. 6).
//! - The *A15K Data Sheet* (archived 2011) and the revision 2 specifications page (archived
//!   2023): "10 samples per second"; the latter lets the launch-detect height be set from 40 to
//!   250 ft, and keeps 2,304 samples a flight.
//! - DataCap's own help: the samples are collected "at the rate of 10 samples per second"; the
//!   `Apogee:` line is "the highest altitude encountered during the flight"; `NumSamps:` is "the
//!   number of data points".
//!
//! The project's 25 sample `.pfa`s bear these out: in every one `NumSamps:` counts the samples and
//! `Apogee:` is their highest, and each first reaches 40 or 80 ft at its eighth sample. Each record
//! says where its units came from in a note, `` `Data` read in feet, 0.1 s apart, from the
//! manual ``, which `cargo xtask logs` counts.
//!
//! **Time zero isn't liftoff.** DataCap's help: "Sampling begins at a point 0.7 seconds before the
//! launch is detected", so the logger saw the launch at 0.7 s, liftoff came before that, and for a
//! rocket slow off the pad before time zero too (the help: the data "may begin at an altitude
//! greater than 0"). A note says so.
//!
//! **`Ground:` isn't the pad's elevation.** DataCap's help gives it as the site's elevation "when
//! the ambient barometric pressure is 29.92" Hg at sea level": a pressure altitude, about 100 ft
//! off the true elevation for every 0.1 inHg the day's sea-level pressure differs, "reported for
//! interest only". It is kept in a note, never as
//! [`Stated::ground_elevation_msl_m`](crate::log::Stated::ground_elevation_msl_m).
//!
//! **What else is checked.** A `NumSamps:` that differs from the samples read is noted, as is an
//! `Apogee:` that isn't their highest. DataCap's help says a `NumSamps` of 2,200 (revision 2) or
//! 2,400 (the original) likely means the logger lost power in flight, its memory then holding
//! older flights after this one's: such a file reads, with a note. The comment, every line from
//! `Comments:` to `**!!Data:`, is kept short in a note. A sample line that isn't a whole number of
//! up to six digits, an empty line among the samples (it would move every time after it), a
//! `**!!` line other than `**!!Data:`, or a `Ground:`, `Apogee:` or `NumSamps:` stated twice is
//! refused, naming the line. Nothing is sized from `NumSamps:`.

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::finite;

/// The format's name in errors.
const FORMAT: &str = ".pf2";

/// Meters in a foot, exactly (the international foot).
pub const FOOT_M: f64 = 0.3048;

/// The columns a `.pf2` has when it has no `Data:` line: the order Debrief reads and the Pnut
/// fixture's `Data:` line states.
const DEFAULT_COLUMNS: [(Column, &str); 5] = [
    (Column::Time, "Time"),
    (Column::Altitude, "Altitude"),
    (Column::Velocity, "Velocity"),
    (
        Column::Temperature(TemperatureUnit::Fahrenheit),
        "Temperature (F)",
    ),
    (Column::Voltage, "Voltage"),
];

/// A column of the data rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Column {
    Time,
    Altitude,
    Velocity,
    Temperature(TemperatureUnit),
    Voltage,
    /// A column the reader doesn't know, kept as written with a note.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

impl TemperatureUnit {
    fn kelvin(self, value: f64) -> f64 {
        match self {
            Self::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
            Self::Celsius => value + 273.15,
        }
    }
}

/// Reads a `.pf2` flight log from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line doesn't name PerfectFlite;
/// [`LogError::Syntax`] for a row that isn't numbers, a time that doesn't increase, a
/// temperature too large to convert to kelvin, or a `Data:` line without a time or an altitude
/// column; [`LogError::Unit`] for a stated height that isn't
/// in feet; [`LogError::NoData`] if there are no rows.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()));
    let logger = loop {
        match lines.next() {
            Some((_, "")) => {}
            Some((_, line)) if line.to_ascii_lowercase().contains("perfectflite") => {
                break line.to_owned();
            }
            Some((_, line)) => {
                return Err(LogError::NotThisFormat {
                    format: FORMAT,
                    message: format!(
                        "its first line, {:?}, doesn't name PerfectFlite",
                        short(line)
                    ),
                });
            }
            None => {
                return Err(LogError::NotThisFormat {
                    format: FORMAT,
                    message: "the file is empty".to_owned(),
                });
            }
        }
    };

    let mut log = FlightLog {
        format: LogFormat::PerfectFlitePf2,
        logger,
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
    let mut notes = Notes::default();
    let mut columns: Option<Vec<(Column, String)>> = None;
    let mut rows: Vec<(usize, Vec<f64>)> = Vec::new();

    for (number, line) in lines {
        if line.is_empty() {
            continue;
        }
        if is_data_row(line) {
            rows.push((number, data_row(line, number)?));
            continue;
        }
        if !rows.is_empty() {
            return Err(syntax(
                number,
                format!(
                    "{:?} after the data rows began isn't a row of numbers",
                    short(line)
                ),
            ));
        }
        let Some((key, value)) = line.split_once(':') else {
            notes.push(format!(
                "line {number}, {:?}, isn't a `Key: value` line and was skipped",
                short(line)
            ));
            continue;
        };
        let value = value.trim();
        match key.trim().to_ascii_lowercase().as_str() {
            "data" => columns = Some(data_columns(value, number, &mut notes)?),
            "apogee" => log.stated.apogee_m = stated_feet(value, number, "apogee", &mut notes)?,
            "ground elevation" => {
                log.stated.ground_elevation_msl_m =
                    stated_feet(value, number, "ground elevation", &mut notes)?;
            }
            "numsamps" => log.stated.samples = value.parse().ok(),
            "flight number" => log.flight_number = value.parse().ok(),
            "serial number" if !value.is_empty() => log.serial_number = Some(value.to_owned()),
            "firmware" if !value.is_empty() => log.firmware = Some(value.to_owned()),
            // The software version, comments and anything else describe the file, not the flight.
            _ => {}
        }
    }

    if rows.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    let columns = columns.unwrap_or_else(|| {
        log.notes.push(
            "the file has no `Data:` line; its columns were read as time, altitude, velocity, \
             temperature (°F) and voltage"
                .to_owned(),
        );
        DEFAULT_COLUMNS
            .iter()
            .map(|(column, name)| (*column, (*name).to_owned()))
            .collect()
    });
    fill(&mut log, FORMAT, &columns, &rows)?;
    if let Some(stated) = log.stated.samples
        && stated != log.time_s.len()
    {
        log.notes.push(format!(
            "the file states {stated} samples and holds {}",
            log.time_s.len()
        ));
    }
    // The file-wide notes above come first, then those about single lines.
    log.notes.extend(notes.kept);
    if notes.left_out > 0 {
        log.notes.push(format!(
            "and {} more notes about single lines, left out so that a broken file's list stays \
             short",
            notes.left_out
        ));
    }
    Ok(log)
}

/// The CSV export's name in errors.
const EXPORT_CSV: &str = "PerfectFlite CSV";
/// The text export's name in errors.
const EXPORT_TEXT: &str = "PerfectFlite text";

/// The column names a PerfectFlite export writes, as the project's 17 sample exports with a
/// header row write them (one names its temperature `Temp`, without the dot). A header row with
/// any other name, or the same name in another case, is another logger's or a person's, such as
/// the RRC3's `Time,Altitude,Pressure,…` or a bare `time,altitude`, whose units nothing states.
const EXPORT_COLUMNS: [&str; 9] = [
    "Time",
    "Altitude",
    "Velocity",
    "Temperature (F)",
    "Temperature (C)",
    "Temp.",
    "Temp",
    "Voltage",
    "Acceleration (calculated)",
];

/// The separator of a PerfectFlite export's header row: `Time`, `Altitude` and at least one more
/// of the names an export writes ([`EXPORT_COLUMNS`]) and no other, by commas or tabs; `None` if
/// the line isn't one.
pub(crate) fn export_separator(header: &str) -> Option<char> {
    [',', '\t'].into_iter().find(|separator| {
        let names: Vec<&str> = header.split(*separator).map(str::trim).collect();
        names.len() >= 3
            && names[0] == "Time"
            && names[1] == "Altitude"
            && names.iter().all(|name| EXPORT_COLUMNS.contains(name))
    })
}

/// The columns a tab-separated export with no header row holds, in order: the header row the
/// text exports with one write.
const HEADERLESS_COLUMNS: &str = "Time\tAltitude\tTemp.\tVoltage";

/// Whether a line is the first row of a tab-separated export written with no header row: four
/// cells, each a number, the third a temperature followed by its unit's letter, `F` or `C` in
/// either case, as
/// the text export with a header row writes its `Temp.` column (`58.7F`). No other layout HPR Sim
/// knows writes a unit's letter after a number in a row of numbers.
pub(crate) fn is_headerless_row(line: &str) -> bool {
    let cells: Vec<&str> = line.split('\t').map(str::trim).collect();
    let plain = |cell: &str| cell.parse::<f64>().is_ok_and(f64::is_finite);
    let [time, altitude, temperature, voltage] = cells[..] else {
        return false;
    };
    plain(time)
        && plain(altitude)
        && plain(voltage)
        && temperature_suffix(temperature).is_some()
        && plain(&temperature[..temperature.len() - 1])
}

/// Reads one of the exports PerfectFlite's software writes from a flight: comma-separated, with a
/// header row such as `Time,Altitude,Velocity,Temperature (F),Voltage`, or tab-separated, with a
/// header row such as `Time`, `Altitude`, `Temp.`, `Voltage` and temperatures written with their unit,
/// such as `58.7F`.
///
/// A tab-separated export may have no header row: its rows are then the four cells the text
/// export writes under `Time`, `Altitude`, `Temp.`, `Voltage`, and a row is recognized by its
/// temperature's unit letter, as no other layout HPR Sim knows writes one. The record notes that
/// the columns were taken so. Four of the project's sample logs are written this way; `cargo
/// xtask logs` checks their altitudes' unit as it checks the other exports'.
///
/// The units are a `.pf2`'s: seconds, feet, feet per second, and the temperature's stated unit.
/// The header doesn't state the altitude's. `cargo xtask logs` bears out feet on the project's
/// sample exports: most peak at exactly the height of the `.pf2` saved beside them, and the rest
/// climb, read in feet, as long as those do against a fall in vacuum from the peak (its report,
/// `validation/reports/log-import.md`, gives the counts and ratios). An export doesn't name the
/// logger's model, so the logger reads `PerfectFlite`. A column the reader doesn't know, such as
/// `Acceleration (calculated)`, is kept as written.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't an export's header row;
/// [`LogError::Syntax`] for a cell that isn't a number, a temperature in a unit other than its
/// column's or too large to convert to kelvin, a row with more cells than the header, or a time
/// that doesn't increase;
/// [`LogError::NoData`] if there are no rows.
pub fn read_export(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()))
        .filter(|(_, line)| !line.is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: EXPORT_CSV,
            message: "the file is empty".to_owned(),
        });
    };
    // A headerless export's first line is its first row, read under the columns it implies.
    let headerless = export_separator(header).is_none() && is_headerless_row(header);
    let first_row = headerless.then_some((header_line, header));
    let (header_line, header) = if headerless {
        (0, HEADERLESS_COLUMNS)
    } else {
        (header_line, header)
    };
    let Some(separator) = export_separator(header) else {
        return Err(LogError::NotThisFormat {
            format: EXPORT_CSV,
            message: format!(
                "its first line, {:?}, isn't a header row of `Time` then `Altitude`, nor a row \
                 of a tab-separated export without one",
                short(header)
            ),
        });
    };
    let (format, name) = if separator == ',' {
        (LogFormat::PerfectFliteCsv, EXPORT_CSV)
    } else {
        (LogFormat::PerfectFliteText, EXPORT_TEXT)
    };
    let rows: Vec<(usize, Vec<&str>)> = first_row
        .into_iter()
        .chain(lines)
        .map(|(number, line)| (number, line.split(separator).map(str::trim).collect()))
        .collect();
    if rows.is_empty() {
        return Err(LogError::NoData { format: name });
    }
    let mut notes = Notes::default();
    // A temperature column whose name states no unit takes the one its first cell is written in.
    let names: Vec<&str> = header.split(separator).collect();
    let cell_unit = names
        .iter()
        .position(|written| written.trim().to_ascii_lowercase().starts_with("temp"))
        .and_then(|index| rows.iter().find_map(|(_, cells)| cells.get(index)))
        .and_then(|cell| temperature_suffix(cell));
    let columns = named_columns(names.into_iter(), cell_unit, &mut notes);
    require_time_and_altitude(&columns, name, header_line, "header row", header)?;

    let mut values = Vec::with_capacity(rows.len());
    for (number, cells) in &rows {
        let row = cells
            .iter()
            .zip(&columns)
            .map(|(cell, (column, _))| export_cell(cell, *column, name, *number))
            .collect::<Result<Vec<f64>, LogError>>()?;
        if cells.len() > columns.len() {
            return Err(LogError::Syntax {
                format: name,
                line: *number,
                message: format!(
                    "the row has {} values and the columns are {}",
                    cells.len(),
                    columns.len()
                ),
            });
        }
        values.push((*number, row));
    }

    let mut log = FlightLog {
        format,
        logger: "PerfectFlite".to_owned(),
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
        notes: headerless
            .then(|| {
                "the file has no header row; its four columns are taken as a text export's, \
                 `Time`, `Altitude`, `Temp.`, `Voltage`, by the unit letter of its temperatures"
                    .to_owned()
            })
            .into_iter()
            .collect(),
    };
    fill(&mut log, name, &columns, &values)?;
    log.notes.extend(notes.kept);
    if notes.left_out > 0 {
        log.notes.push(format!(
            "and {} more notes about single lines, left out so that a broken file's list stays \
             short",
            notes.left_out
        ));
    }
    Ok(log)
}

/// The unit a temperature cell is written in, such as `58.7F`; `None` for a bare number.
fn temperature_suffix(cell: &str) -> Option<TemperatureUnit> {
    match cell.trim().chars().last()? {
        'F' | 'f' => Some(TemperatureUnit::Fahrenheit),
        'C' | 'c' => Some(TemperatureUnit::Celsius),
        _ => None,
    }
}

/// An export's cell as a number in the file's unit: a temperature may carry its column's unit
/// letter, and no other.
fn export_cell(
    cell: &str,
    column: Column,
    format: &'static str,
    line: usize,
) -> Result<f64, LogError> {
    let digits = match (column, temperature_suffix(cell)) {
        (_, None) => cell,
        (Column::Temperature(unit), Some(written)) if unit == written => &cell[..cell.len() - 1],
        _ => {
            return Err(LogError::Syntax {
                format,
                line,
                message: format!("{:?} carries a unit its column doesn't state", short(cell)),
            });
        }
    };
    digits
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| LogError::Syntax {
            format,
            line,
            message: format!("{:?} isn't a finite number", short(cell)),
        })
}

/// The most notes about single lines a log keeps; past it, one more note counts the rest.
const MAX_NOTES: usize = 20;

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

/// Whether a line is a data row: it starts with a number and has a comma, as Debrief tells them.
fn is_data_row(line: &str) -> bool {
    let first = line.trim_start_matches(['-', '+']);
    first.starts_with(|c: char| c.is_ascii_digit() || c == '.') && line.contains(',')
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// A data row's cells as numbers.
fn data_row(line: &str, number: usize) -> Result<Vec<f64>, LogError> {
    line.split(',')
        .map(|cell| {
            let cell = cell.trim();
            cell.parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
                .ok_or_else(|| syntax(number, format!("{:?} isn't a finite number", short(cell))))
        })
        .collect()
}

/// The columns a `Data:` line names, such as `(Time, Altitude, Velocity, Temperature (F), Voltage)`,
/// each with its name as written.
fn data_columns(
    value: &str,
    number: usize,
    notes: &mut Notes,
) -> Result<Vec<(Column, String)>, LogError> {
    let inner = value.strip_prefix('(').unwrap_or(value);
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    let columns = named_columns(inner.split(','), None, notes);
    require_time_and_altitude(&columns, FORMAT, number, "`Data:` line", value)?;
    Ok(columns)
}

/// Each column's meaning from its name, with the name as written. A temperature column whose name
/// states no unit, such as the text export's `Temp.`, takes `cell_unit`, the unit its cells
/// carry; without one it is kept as written.
fn named_columns<'a>(
    names: impl Iterator<Item = &'a str>,
    cell_unit: Option<TemperatureUnit>,
    notes: &mut Notes,
) -> Vec<(Column, String)> {
    names
        .map(|written| {
            let name = written.trim().to_ascii_lowercase();
            let column = if name.starts_with("time") {
                Column::Time
            } else if name.starts_with("altitude") {
                Column::Altitude
            } else if name.starts_with("velocity") {
                Column::Velocity
            } else if name.starts_with("temp") && name.contains("(f)") {
                Column::Temperature(TemperatureUnit::Fahrenheit)
            } else if name.starts_with("temp") && name.contains("(c)") {
                Column::Temperature(TemperatureUnit::Celsius)
            } else if let (true, Some(unit)) = (name.starts_with("temp"), cell_unit) {
                Column::Temperature(unit)
            } else if name.starts_with("voltage") {
                Column::Voltage
            } else {
                Column::Unknown
            };
            if column == Column::Unknown {
                notes.push(format!(
                    "the column {:?} isn't one this reader knows, and was kept as written",
                    short(&name)
                ));
            }
            (column, written.trim().to_owned())
        })
        .collect()
}

/// Refuses a list of columns without exactly one time and one altitude.
fn require_time_and_altitude(
    columns: &[(Column, String)],
    format: &'static str,
    line: usize,
    what: &str,
    written: &str,
) -> Result<(), LogError> {
    for required in [Column::Time, Column::Altitude] {
        if columns
            .iter()
            .filter(|(column, _)| *column == required)
            .count()
            != 1
        {
            return Err(LogError::Syntax {
                format,
                line,
                message: format!(
                    "the {what} must name one {required:?} column: {:?}",
                    short(written)
                ),
            });
        }
    }
    Ok(())
}

/// A stated height, such as `1009' AGL`, in meters; `None`, with a note, for one that isn't a
/// number HPR Sim can read, such as `PWRLOSS`, or that names no unit, such as `1009 AGL`. A number in
/// a unit other than feet (`'`, `ft`, `feet`) is refused.
fn stated_feet(
    value: &str,
    number: usize,
    what: &str,
    notes: &mut Notes,
) -> Result<Option<f64>, LogError> {
    let end = value
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+')))
        .unwrap_or(value.len());
    let (digits, rest) = value.split_at(end);
    let Some(feet) = digits.parse::<f64>().ok().filter(|feet| feet.is_finite()) else {
        notes.push(format!(
            "the file states its {what} as {:?}, which isn't a height",
            short(value)
        ));
        return Ok(None);
    };
    let rest = rest.trim_start();
    let word: String = rest
        .chars()
        .take_while(char::is_ascii_alphabetic)
        .collect::<String>()
        .to_ascii_lowercase();
    if rest.starts_with('\'') || word == "ft" || word == "feet" {
        return Ok(Some(feet * FOOT_M));
    }
    // `AGL` and `MSL` say what the height is measured from, not its unit.
    if word.is_empty() || word == "agl" || word == "msl" {
        notes.push(format!(
            "the file states its {what} as {:?}, with no unit HPR Sim can read",
            short(value)
        ));
        return Ok(None);
    }
    Err(LogError::Unit {
        format: FORMAT,
        line: number,
        message: format!(
            "the {what} {:?} is in {:?}, not feet ('), the only unit this reader knows",
            short(value),
            short(&word)
        ),
    })
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

/// The most characters of the file an error or note quotes.
const QUOTED_CHARS: usize = 80;

/// Puts the rows' values into the log's channels, in SI.
fn fill(
    log: &mut FlightLog,
    format: &'static str,
    columns: &[(Column, String)],
    rows: &[(usize, Vec<f64>)],
) -> Result<(), LogError> {
    let syntax = |line: usize, message: String| LogError::Syntax {
        format,
        line,
        message,
    };
    let written: usize = rows.iter().map(|(_, cells)| cells.len()).sum();
    if crate::log::too_sparse(columns.len(), rows.len(), written) {
        return Err(syntax(
            rows.first().map_or(1, |(number, _)| *number),
            format!(
                "{} rows hold {written} of the {} cells the {} columns name: too few for the \
                 columns to be this file's",
                rows.len(),
                rows.len().saturating_mul(columns.len()),
                columns.len()
            ),
        ));
    }
    // Every column's values, the time's included, row by row; a cell a row leaves out is NaN.
    let mut values: Vec<Vec<f64>> = vec![Vec::with_capacity(rows.len()); columns.len()];
    for (number, cells) in rows {
        if cells.len() > columns.len() {
            return Err(syntax(
                *number,
                format!(
                    "the row has {} values and the columns are {}",
                    cells.len(),
                    columns.len()
                ),
            ));
        }
        let mut time = None;
        let mut altitude = None;
        for (index, (column, name)) in columns.iter().enumerate() {
            let cell = cells.get(index).copied();
            match column {
                Column::Time => time = cell,
                Column::Altitude => altitude = cell,
                _ => {}
            }
            // °F to kelvin multiplies by 5 before it divides by 9, so a temperature near the
            // largest `f64` is infinite in kelvin: refused on its row's line (ADR-258 §3).
            let value = match cell {
                Some(value) => finite(si(*column, value), format, *number, name)?,
                None => f64::NAN,
            };
            values[index].push(value);
        }
        let (Some(time), Some(_)) = (time, altitude) else {
            return Err(syntax(
                *number,
                "the row leaves out its time or its altitude".to_owned(),
            ));
        };
        if let Some(&last) = log.time_s.last()
            && time <= last
        {
            return Err(syntax(
                *number,
                format!("the time {time} s doesn't come after the row before's {last} s"),
            ));
        }
        log.time_s.push(time);
    }
    for ((column, name), values) in columns.iter().zip(values) {
        let quantity = match column {
            Column::Time => continue,
            Column::Altitude => {
                log.altitude_m.clone_from(&values);
                Quantity::AltitudeAboveGround
            }
            Column::Velocity => {
                log.vertical_speed_m_s = Some(values.clone());
                Quantity::Speed
            }
            Column::Temperature(_) => {
                log.temperature_k = Some(values.clone());
                Quantity::Temperature
            }
            Column::Voltage => {
                log.battery_v = Some(values.clone());
                Quantity::Voltage
            }
            Column::Unknown => Quantity::Unrecognized,
        };
        log.channels.push(Channel {
            column: name.clone(),
            quantity,
            time_s: log.time_s.clone(),
            values,
        });
    }
    Ok(())
}

/// A cell's value in SI: feet to meters, temperatures to kelvin, the rest as written.
fn si(column: Column, value: f64) -> f64 {
    match column {
        Column::Altitude | Column::Velocity => value * FOOT_M,
        Column::Temperature(unit) => unit.kelvin(value),
        Column::Time | Column::Voltage | Column::Unknown => value,
    }
}

/// The `.pfa`'s name in errors.
const PFA: &str = ".pfa";

/// An Alt15K/WD's samples a second, fixed: it logs "every 100 milliseconds" (*Alt15K/WD User's
/// Manual*, PerfectFlite, p. 2), "10 samples per second" (*A15K Data Sheet*; DataCap's help, as
/// AltCapWD 1.10 shows it). A `.pfa` doesn't state it.
const PFA_SAMPLES_PER_S: f64 = 10.0;

/// How long before the logger detected the launch its first sample is, s: "Sampling begins at a
/// point 0.7 seconds before the launch is detected" (DataCap's help), the 8 samples it keeps from
/// before the detection (*Alt15K/WD User's Manual*, p. 2) spanning 7 steps of 0.1 s.
const PFA_BEFORE_DETECTION_S: f64 = 0.7;

/// The sample counts DataCap's help gives for an Alt15K/WD that has likely lost power in flight,
/// its memory then holding older flights after this one's: 2,200 for the revision 2 logger and
/// 2,400 for the original.
const PFA_FULL_MEMORY_SAMPLES: [usize; 2] = [2_200, 2_400];

/// The most digits a `.pfa` sample may have: DataCap writes five, and an Alt15K/WD works to
/// 15,000 ft (*Alt15K/WD User's Manual*, p. 10); a sixth is let through, more is no height.
const PFA_SAMPLE_DIGITS: usize = 6;

/// The name the record gives the `.pfa`'s one channel, the `Data` its `**!!Data:` line names.
const PFA_CHANNEL: &str = "Data";

/// Reads a `.pfa` flight log, as PerfectFlite's DataCap saves an Alt15K/WD's flight, from its
/// text: one altitude a line after a `**!!Data:` line, each 0.1 s after the one before, in feet
/// above the pad. The layout and where its units come from are in the [module's
/// documentation](self).
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line doesn't name PerfectFlite;
/// [`LogError::Syntax`] for a sample line that isn't a whole number, an empty line among the
/// samples, a `**!!` line other than `**!!Data:`, or a `Ground:`, `Apogee:` or `NumSamps:` line
/// stated twice; [`LogError::NoData`] if there is no `**!!Data:` line or no sample after it.
pub fn read_pfa(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()));
    let logger = loop {
        match lines.next() {
            Some((_, "")) => {}
            Some((_, line)) if line.to_ascii_lowercase().contains("perfectflite") => {
                break line.to_owned();
            }
            Some((_, line)) => {
                return Err(LogError::NotThisFormat {
                    format: PFA,
                    message: format!(
                        "its first line, {:?}, doesn't name PerfectFlite",
                        short(line)
                    ),
                });
            }
            None => {
                return Err(LogError::NotThisFormat {
                    format: PFA,
                    message: "the file is empty".to_owned(),
                });
            }
        }
    };

    let mut notes = Notes::default();
    let mut ground_ft: Option<f64> = None;
    let mut apogee_ft: Option<f64> = None;
    let mut stated_samples: Option<usize> = None;
    // The keys stated so far, to refuse one stated twice.
    let mut seen: [Option<usize>; 3] = [None; 3];
    // The comment, from its `Comments:` line to the `**!!Data:` line, cut short as it grows.
    let mut comment: Option<String> = None;
    let mut data_line = false;
    for (number, line) in lines.by_ref() {
        if let Some(rest) = line.strip_prefix("**!!") {
            let rest = rest.trim();
            let named = rest
                .strip_prefix("Data:")
                .or_else(|| rest.strip_prefix("data:"));
            if named.is_none_or(|after| !after.trim().is_empty()) {
                return Err(pfa_syntax(
                    number,
                    format!(
                        "{:?} isn't the `**!!Data:` line the samples follow",
                        short(line)
                    ),
                ));
            }
            data_line = true;
            break;
        }
        if let Some(comment) = comment.as_mut() {
            // Every line after `Comments:` is the comment's, up to the samples.
            if !line.is_empty() && comment.chars().count() <= QUOTED_CHARS {
                if !comment.is_empty() {
                    comment.push(' ');
                }
                comment.push_str(&short(line));
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            notes.push(format!(
                "line {number}, {:?}, isn't a `Key: value` line and was skipped",
                short(line)
            ));
            continue;
        };
        let value = value.trim();
        let key = key.trim().to_ascii_lowercase();
        let slot = match key.as_str() {
            "ground" => 0,
            "apogee" => 1,
            "numsamps" => 2,
            "comments" => {
                comment = Some(short(value));
                continue;
            }
            _ => {
                notes.push(format!(
                    "line {number}, {:?}, isn't a line this reader knows and was skipped",
                    short(line)
                ));
                continue;
            }
        };
        if let Some(first) = seen[slot] {
            return Err(pfa_syntax(
                number,
                format!(
                    "{:?} states the {key} line {first} stated already",
                    short(line)
                ),
            ));
        }
        seen[slot] = Some(number);
        match slot {
            0 | 1 => {
                let feet = value.parse::<f64>().ok().filter(|feet| feet.is_finite());
                if feet.is_none() {
                    notes.push(format!(
                        "the file states its {key} as {:?}, which isn't a height in feet",
                        short(value)
                    ));
                }
                if slot == 0 {
                    ground_ft = feet;
                } else {
                    apogee_ft = feet;
                }
            }
            _ => {
                stated_samples = value.parse().ok();
                if stated_samples.is_none() {
                    notes.push(format!(
                        "the file states its sample count as {:?}, which isn't a count",
                        short(value)
                    ));
                }
            }
        }
    }
    if !data_line {
        return Err(LogError::NoData { format: PFA });
    }

    // One sample a line; an empty line is allowed only after the last.
    let mut feet: Vec<f64> = Vec::new();
    let mut empty_line: Option<usize> = None;
    for (number, line) in lines {
        if line.is_empty() {
            empty_line = empty_line.or(Some(number));
            continue;
        }
        if let Some(empty) = empty_line {
            return Err(pfa_syntax(
                empty,
                "an empty line among the samples: each line is the sample 0.1 s after the one \
                 before, so a line left out would move every time after it"
                    .to_owned(),
            ));
        }
        feet.push(pfa_sample(line).ok_or_else(|| {
            pfa_syntax(
                number,
                format!(
                    "{:?} isn't a sample: a whole number of feet, up to {PFA_SAMPLE_DIGITS} \
                     digits, alone on its line",
                    short(line)
                ),
            )
        })?);
    }
    if feet.is_empty() {
        return Err(LogError::NoData { format: PFA });
    }

    // The n-th sample's time is n / 10 s, divided rather than multiplied by 0.1 so that each time
    // is the double nearest its tenth.
    let time_s: Vec<f64> = (0..feet.len())
        .map(|index| index as f64 / PFA_SAMPLES_PER_S)
        .collect();
    let altitude_m: Vec<f64> = feet.iter().map(|feet| feet * FOOT_M).collect();

    let mut file_notes = vec![
        "`Data` read in feet, 0.1 s apart, from the manual: the file states neither; the \
         Alt15K/WD manual logs a sample every 100 ms (p. 2) and beeps its apogee in feet (p. 6), \
         and DataCap's help gives 10 samples a second and the `Apogee:` line as the highest \
         sample"
            .to_owned(),
        format!(
            "time zero is {PFA_BEFORE_DETECTION_S} s before the logger detected the launch, not \
             liftoff: DataCap's help starts the samples {PFA_BEFORE_DETECTION_S} s before the \
             detection, so liftoff came before {PFA_BEFORE_DETECTION_S} s, and before time zero \
             if the rocket was slow off the pad"
        ),
    ];
    if let Some(ground) = ground_ft {
        file_notes.push(format!(
            "the file's `Ground:`, {ground} ft, is the pad's pressure altitude, not its \
             elevation: DataCap's help gives it as the site's height were sea-level pressure \
             29.92 inHg, about 100 ft off for every 0.1 inHg the day's differs, and for interest \
             only; it isn't read as the ground elevation"
        ));
    }
    if let Some(full) = [stated_samples, Some(feet.len())]
        .into_iter()
        .flatten()
        .find(|count| PFA_FULL_MEMORY_SAMPLES.contains(count))
    {
        file_notes.push(format!(
            "the file holds {full} samples, the count DataCap's help gives for an Alt15K/WD that \
             likely lost power in flight (2200 for revision 2, 2400 for the original): the \
             samples after this flight's may be older flights'"
        ));
    }
    if let Some(stated) = stated_samples
        && stated != feet.len()
    {
        file_notes.push(format!(
            "the file states {stated} samples and holds {}",
            feet.len()
        ));
    }
    let peak_ft = feet.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if let Some(apogee) = apogee_ft
        && apogee != peak_ft
    {
        file_notes.push(format!(
            "the file states its apogee as {apogee} ft and its samples peak at {peak_ft} ft"
        ));
    }
    if let Some(comment) = comment.filter(|comment| !comment.is_empty()) {
        file_notes.push(format!("the file's comment: {:?}", short(&comment)));
    }
    // The file-wide notes above come first, then those about single lines.
    file_notes.extend(notes.kept);
    if notes.left_out > 0 {
        file_notes.push(format!(
            "and {} more notes about single lines, left out so that a broken file's list stays \
             short",
            notes.left_out
        ));
    }

    Ok(FlightLog {
        format: LogFormat::PerfectFlitePfa,
        logger,
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            apogee_m: apogee_ft.map(|feet| feet * FOOT_M),
            // `Ground:` is a pressure altitude, not the site's elevation: in a note, not here.
            ground_elevation_msl_m: None,
            samples: stated_samples,
            figures: Vec::new(),
        },
        channels: vec![Channel {
            column: PFA_CHANNEL.to_owned(),
            quantity: Quantity::AltitudeAboveGround,
            time_s: time_s.clone(),
            values: altitude_m.clone(),
        }],
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        events: Vec::new(),
        notes: file_notes,
    })
}

/// A `.pfa` sample line's height, ft: a whole number of up to [`PFA_SAMPLE_DIGITS`] digits,
/// perhaps after a minus sign; `None` for anything else.
fn pfa_sample(line: &str) -> Option<f64> {
    let digits = line.strip_prefix('-').unwrap_or(line);
    if digits.is_empty()
        || digits.len() > PFA_SAMPLE_DIGITS
        || !digits.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    // At most six digits, so the value is exact as an `i32` and as an `f64`.
    line.parse::<i32>().ok().map(f64::from)
}

fn pfa_syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: PFA,
        line,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = "PerfectFlite Pnut\r\nFirmware: 1.0\r\nSerial Number: 17\r\n\
                        Apogee: 1280' AGL\r\nGround Elevation: 600' MSL\r\nNumSamps: 3\r\n\
                        Flight Number: 2\r\nComments: \r\n\r\n\
                        Data: (Time, Altitude, Velocity, Temperature (F), Voltage)\r\n";

    /// A temperature in °F near the largest `f64` is finite as written and infinite in kelvin, as
    /// the conversion multiplies by 5 before it divides by 9: refused on its line, in the `.pf2`
    /// and in both exports (ADR-258 §3). Feet only shrink, so no height or speed can overflow.
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let pf2 = format!("{HEAD}0.00, 0, 0, 32.00, 4.20\r\n0.05, 1, 10, -1.7e308, 4.2\r\n");
        assert_eq!(
            read(&pf2).unwrap_err().to_string(),
            ".pf2 line 12: the column Temperature (F) holds a value too large for its unit's \
             conversion to SI"
        );
        let csv = "Time,Altitude,Velocity,Temperature (F),Voltage\n0,0,0,70,9\n\
                   0.05,1,1,-1.7e308,9\n";
        assert_eq!(
            read_export(csv).unwrap_err().to_string(),
            "PerfectFlite CSV line 3: the column Temperature (F) holds a value too large for its \
             unit's conversion to SI"
        );
        let text = "Time\tAltitude\tTemp.\tVoltage\n0\t0\t70F\t9\n0.05\t1\t-1.7e308F\t9\n";
        assert_eq!(
            read_export(text).unwrap_err().to_string(),
            "PerfectFlite text line 3: the column Temp. holds a value too large for its unit's \
             conversion to SI"
        );
    }

    /// Every value in SI, a short row's missing cells gaps, and the preamble's statements read.
    #[test]
    fn reads_the_preamble_and_rows_in_si() {
        let text =
            format!("{HEAD}0.00, 0, 0, 32.00, 4.20\r\n0.05, -1, 10\r\n0.10, 10, 20, 212, 4.1\r\n");
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::PerfectFlitePf2);
        assert_eq!(log.logger, "PerfectFlite Pnut");
        assert_eq!(log.serial_number.as_deref(), Some("17"));
        assert_eq!(log.firmware.as_deref(), Some("1.0"));
        assert_eq!(log.flight_number, Some(2));
        assert_eq!(log.stated.apogee_m, Some(1280.0 * 0.3048));
        assert_eq!(log.stated.ground_elevation_msl_m, Some(600.0 * 0.3048));
        assert_eq!(log.stated.samples, Some(3));
        assert_eq!(log.time_s, [0.0, 0.05, 0.10]);
        assert_eq!(log.altitude_m, [0.0, -0.3048, 3.048]);
        assert_eq!(
            log.vertical_speed_m_s.as_deref(),
            Some(&[0.0, 3.048, 6.096][..])
        );
        let temperature = log.temperature_k.unwrap();
        assert_eq!(temperature[0], 273.15);
        assert!(temperature[1].is_nan());
        assert!((temperature[2] - 373.15).abs() < 1e-12);
        let battery = log.battery_v.unwrap();
        assert_eq!((battery[0], battery[2]), (4.2, 4.1));
        assert!(battery[1].is_nan());
        assert!(log.notes.is_empty(), "{:?}", log.notes);
    }

    /// The columns come from the `Data:` line, in its order; a column it doesn't know is noted
    /// and left out; without the line, Debrief's order is assumed and noted.
    #[test]
    fn columns_follow_the_data_line() {
        let text = "PerfectFlite StratoLogger\nData: (Altitude, Time, Pressure)\n100, 0.5, 9\n";
        let log = read(text).unwrap();
        assert_eq!((log.time_s[0], log.altitude_m[0]), (0.5, 30.48));
        assert!(log.vertical_speed_m_s.is_none() && log.temperature_k.is_none());
        assert!(log.notes[0].contains("\"pressure\""), "{:?}", log.notes);

        let log = read("PerfectFlite Pnut\n0.0, 5, 1, 50, 4\n").unwrap();
        assert_eq!(log.altitude_m, [5.0 * 0.3048]);
        assert!(log.notes[0].contains("no `Data:` line"), "{:?}", log.notes);
    }

    /// `PWRLOSS` is no height, and `1009 AGL` names no unit: both noted, not refused. A height in
    /// another unit is refused.
    #[test]
    fn stated_heights_are_feet_or_nothing() {
        let log = read("PerfectFlite Pnut\nApogee: PWRLOSS\n0, 0, 0\n").unwrap();
        assert_eq!(log.stated.apogee_m, None);
        assert!(
            log.notes.iter().any(|note| note.contains("PWRLOSS")),
            "{:?}",
            log.notes
        );
        let error = read("PerfectFlite Pnut\nApogee: 390 m AGL\n0, 0, 0\n").unwrap_err();
        assert!(
            matches!(&error, LogError::Unit { line: 2, message, .. } if message.contains("\"390 m AGL\" is in \"m\"")),
            "{error}"
        ); // `AGL` names the height's zero, not its unit: noted, and the flight still reads.
        let log = read("PerfectFlite Pnut\nApogee: 1009 AGL\n0, 0, 0\n").unwrap();
        assert_eq!(log.stated.apogee_m, None);
        assert!(
            log.notes.iter().any(|note| note.contains("no unit")),
            "{:?}",
            log.notes
        );
    }

    /// A header of many unreadable lines keeps a short list of notes, the file-wide one first,
    /// and counts the 980 it left out of the 1,000 skipped lines.
    #[test]
    fn notes_stay_few() {
        let text = format!("PerfectFlite Pnut\n{}0, 0, 0\n", "x\n".repeat(1000));
        let log = read(&text).unwrap();
        assert_eq!(log.notes.len(), MAX_NOTES + 2);
        assert!(log.notes[0].contains("no `Data:` line"), "{:?}", log.notes);
        assert!(
            log.notes[MAX_NOTES + 1].starts_with("and 980 more notes"),
            "{:?}",
            log.notes
        );
    }

    /// A file that isn't text is refused with a short quote of it, not the whole of it.
    #[test]
    fn a_long_line_is_quoted_short() {
        let junk = "\u{fffd}".repeat(100_000);
        let error = read(&junk).unwrap_err().to_string();
        assert!(error.len() < 400, "{} bytes", error.len());
        assert!(error.contains('…'), "{error}");
    }

    /// A byte-order mark is dropped; CR-only line ends read as lines; `ft` is feet; a height
    /// hpr can't read, such as one with a thousands comma, is noted and left out.
    #[test]
    fn marks_line_ends_and_odd_heights() {
        let log = read(
            "\u{feff}PerfectFlite Pnut\rApogee: 1,009' AGL\rGround Elevation: 600 ft\r0, 0, 0\r",
        )
        .unwrap();
        assert_eq!(log.logger, "PerfectFlite Pnut");
        assert_eq!(log.time_s.len(), 1);
        assert_eq!(log.stated.apogee_m, None);
        assert!(
            log.notes.iter().any(|note| note.contains("\"1,009' AGL\"")),
            "{:?}",
            log.notes
        );
        assert_eq!(log.stated.ground_elevation_msl_m, Some(600.0 * FOOT_M));
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let cases = [
            ("", "the file is empty"),
            ("Time, Altitude\n0, 0\n", "doesn't name PerfectFlite"),
            ("PerfectFlite Pnut\nApogee: 5'\n", "no data rows"),
            (
                "PerfectFlite Pnut\n0, 0, 0\n0, 1, 0\n",
                "line 3: the time 0 s doesn't come after",
            ),
            (
                "PerfectFlite Pnut\n0, 0, x\n",
                "line 2: \"x\" isn't a finite number",
            ),
            (
                "PerfectFlite Pnut\n0, 0, 0\nEnd\n",
                "line 3: \"End\" after the data rows",
            ),
            (
                "PerfectFlite Pnut\n0, 0, 0, 1, 2, 3\n",
                "line 2: the row has 6 values",
            ),
            ("PerfectFlite Pnut\n0\n", "no data rows"),
            (
                "PerfectFlite Pnut\nData: (Time, Velocity)\n0, 0\n",
                "line 2: the `Data:` line",
            ),
            (
                "PerfectFlite Pnut\nData: (Time, Altitude)\n0.5\n",
                "no data rows",
            ),
            (
                "PerfectFlite Pnut\nData: (Time, Altitude)\n0.5,\n",
                "line 3: \"\" isn't",
            ),
            (
                "PerfectFlite Pnut\n0, inf, 0\n",
                "line 2: \"inf\" isn't a finite number",
            ),
        ];
        for (text, expected) in cases {
            let error = read(text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
        // A short row that stops before the altitude's column leaves it out.
        let error =
            read("PerfectFlite Pnut\nData: (Time, Velocity, Altitude)\n0, 5\n").unwrap_err();
        assert!(
            error.to_string().contains("line 3: the row leaves out"),
            "{error}"
        );
    }

    /// A stated sample count the rows don't match is noted.
    #[test]
    fn a_sample_count_that_differs_is_noted() {
        let log = read("PerfectFlite Pnut\nNumSamps: 3\n0, 0, 0\n0.05, 0, 0\n").unwrap();
        assert_eq!(
            log.notes.last().unwrap(),
            "the file states 3 samples and holds 2"
        );
    }

    /// An invented `.pfa`'s preamble, CR-separated as DataCap writes most.
    const PFA_HEAD: &str =
        "PerfectFlite Alt15K/WD\rGround: 1150\rApogee: 30\rNumSamps: 4\rComments: \r\r**!!Data: \r";

    /// Each sample is 0.1 s after the one before, from 0, and feet above the pad in meters; the
    /// stated apogee and count read, and `Ground:` stays out of the ground elevation.
    #[test]
    fn a_pfa_reads_a_sample_every_tenth_of_a_second() {
        let log = read_pfa(&format!("{PFA_HEAD}00000\r00004\r00030\r00012\r")).unwrap();
        assert_eq!(log.format, LogFormat::PerfectFlitePfa);
        assert_eq!(log.logger, "PerfectFlite Alt15K/WD");
        assert_eq!(log.time_s, [0.0, 0.1, 0.2, 0.3]);
        assert_eq!(
            log.altitude_m,
            [0.0, 4.0 * FOOT_M, 30.0 * FOOT_M, 12.0 * FOOT_M]
        );
        assert_eq!(log.stated.apogee_m, Some(30.0 * FOOT_M));
        assert_eq!(log.stated.samples, Some(4));
        assert_eq!(log.stated.ground_elevation_msl_m, None);
        assert_eq!(log.channels.len(), 1);
        assert_eq!(log.channels[0].column, "Data");
        assert_eq!(log.channels[0].quantity, Quantity::AltitudeAboveGround);
        assert_eq!(log.channels[0].values, log.altitude_m);
        assert_eq!(log.channels[0].time_s, log.time_s);
        // The unit decision, the clock's zero and `Ground:`, and nothing more.
        assert_eq!(log.notes.len(), 3, "{:?}", log.notes);
        assert!(
            log.notes[0].starts_with("`Data` read in feet, 0.1 s apart, from the manual: "),
            "{:?}",
            log.notes
        );
        assert!(log.notes[1].starts_with("time zero is 0.7 s before the logger detected"));
        assert!(log.notes[2].starts_with("the file's `Ground:`, 1150 ft, is the pad's pressure"));
    }

    /// The thousandth sample's time is the double nearest 99.9 s, not 999 × 0.1's sum or product.
    #[test]
    fn a_pfa_time_is_the_nearest_double_to_its_tenth() {
        let samples = "00001\r".repeat(1000);
        let log = read_pfa(&format!("PerfectFlite Alt15K/WD\r**!!Data:\r{samples}")).unwrap();
        assert_eq!(log.time_s[999], 99.9);
        assert_eq!(log.time_s[3], 0.3);
    }

    /// The count DataCap's help names for a logger that lost power is noted, stated or read;
    /// another large count isn't.
    #[test]
    fn a_pfa_at_a_full_memory_count_is_noted() {
        for (count, noted) in [(2_200, true), (2_400, true), (2_304, false), (2_199, false)] {
            let samples = "00010\r".repeat(count);
            let text = format!("PerfectFlite Alt15K/WD\r**!!Data:\r{samples}");
            let log = read_pfa(&text).unwrap();
            let note = format!("the file holds {count} samples, the count DataCap's help gives");
            assert_eq!(
                log.notes.iter().any(|n| n.starts_with(&note)),
                noted,
                "{count}: {:?}",
                log.notes
            );
        }
        // Stated, though the file holds fewer: noted, beside the count that differs.
        let log =
            read_pfa("PerfectFlite Alt15K/WD\rNumSamps: 2400\r**!!Data:\r00001\r00002\r").unwrap();
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("the file holds 2400 samples"))
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n == "the file states 2400 samples and holds 2")
        );
    }

    /// A huge stated count costs nothing: nothing is sized from it.
    #[test]
    fn a_pfa_huge_count_allocates_nothing() {
        let log = read_pfa(&format!(
            "PerfectFlite Alt15K/WD\rNumSamps: {}\r**!!Data:\r00001\r",
            usize::MAX
        ))
        .unwrap();
        assert_eq!(log.stated.samples, Some(usize::MAX));
        assert_eq!(log.time_s.len(), 1);
    }

    /// An apogee that isn't the samples' highest, a value that isn't a height and a comment over
    /// several lines are noted; `Ground:` that isn't a height leaves no `Ground:` note.
    #[test]
    fn a_pfa_preamble_is_checked_against_its_samples() {
        let text = "PerfectFlite Alt15K/WD\rGround: n/a\rApogee: 31\rNumSamps: lots\r\
                    Comments: first line\rsecond: line\r\r**!!Data:\r00000\r00030\r";
        let log = read_pfa(text).unwrap();
        let notes = &log.notes;
        assert!(
            notes.contains(
                &"the file states its apogee as 31 ft and its samples peak at 30 ft".to_owned()
            ),
            "{notes:?}"
        );
        assert!(notes.contains(&"the file's comment: \"first line second: line\"".to_owned()));
        assert!(notes.iter().any(|n| n.contains("its ground as \"n/a\"")));
        assert!(notes.iter().any(|n| n.contains("sample count as \"lots\"")));
        assert!(!notes.iter().any(|n| n.contains("pressure altitude")));
        assert_eq!(log.stated.samples, None);
        // A comment of any length is quoted short.
        let long = format!(
            "PerfectFlite Alt15K/WD\rComments: {}\r**!!Data:\r00001\r",
            "word ".repeat(10_000)
        );
        let log = read_pfa(&long).unwrap();
        let comment = log
            .notes
            .iter()
            .find(|n| n.starts_with("the file's comment"))
            .unwrap();
        assert!(comment.chars().count() < 2 * QUOTED_CHARS, "{comment}");
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn a_malformed_pfa_is_refused_where_it_goes_wrong() {
        let cases = [
            ("", "the file is empty"),
            ("Ground: 5\r**!!Data:\r00001\r", "doesn't name PerfectFlite"),
            ("PerfectFlite Alt15K/WD\rApogee: 5\r", "no data rows"),
            ("PerfectFlite Alt15K/WD\r**!!Data:\r\r", "no data rows"),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r00001\r0001x\r",
                "line 4: \"0001x\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r00001\r12.5\r",
                "line 4: \"12.5\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r00001\r00001, 00002\r",
                "line 4: \"00001, 00002\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r1234567\r",
                "line 3: \"1234567\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r-\r",
                "line 3: \"-\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r00001\r\r00002\r",
                "line 4: an empty line among the samples",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data:\r00001\r**!!Data:\r00002\r",
                "line 4: \"**!!Data:\" isn't a sample",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Info:\r00001\r",
                "line 2: \"**!!Info:\" isn't the `**!!Data:` line",
            ),
            (
                "PerfectFlite Alt15K/WD\r**!!Data: 00001\r",
                "line 2: \"**!!Data: 00001\" isn't the `**!!Data:` line",
            ),
            (
                "PerfectFlite Alt15K/WD\rApogee: 5\rApogee: 6\r**!!Data:\r00001\r",
                "line 3: \"Apogee: 6\" states the apogee line 2 stated already",
            ),
        ];
        for (text, expected) in cases {
            let error = read_pfa(text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
            assert!(error.contains(".pfa"), "{text:?}: {error}");
        }
        // A negative sample, below the pad, reads; so do trailing empty lines.
        let log = read_pfa("PerfectFlite Alt15K/WD\r**!!Data:\r-0003\r00001\r\r\r").unwrap();
        assert_eq!(log.altitude_m, [-3.0 * FOOT_M, FOOT_M]);
    }

    /// Unknown preamble lines are noted, and only so many of them.
    #[test]
    fn a_pfa_keeps_its_notes_few() {
        let text = format!(
            "PerfectFlite Alt15K/WD\r{}**!!Data:\r00001\r",
            "Odd: line\r".repeat(500)
        );
        let log = read_pfa(&text).unwrap();
        assert_eq!(log.notes.len(), 2 + MAX_NOTES + 1, "{:?}", log.notes);
        assert!(log.notes.last().unwrap().starts_with("and 480 more notes"));
    }

    proptest::proptest! {
        /// Any text is read or refused as a `.pfa`, never a panic; what is read has times 0.1 s
        /// apart from 0 and as many altitudes.
        #[test]
        fn any_pfa_text_is_read_or_refused(
            text in "(PerfectFlite Alt15K/WD\r)?([-0-9 A-Za-z:*!]{0,12}\r){0,6}(\\*\\*!!Data:\r)?([-0-9 ]{0,7}\r){0,12}"
        ) {
            if let Ok(log) = read_pfa(&text) {
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for (index, time) in log.time_s.iter().enumerate() {
                    proptest::prop_assert_eq!(*time, index as f64 / 10.0);
                }
            }
        }
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has increasing finite times.
        #[test]
        fn any_text_is_read_or_refused(text in "(PerfectFlite Pnut\n)?([-0-9., A-Za-z:'()]{0,20}\n){0,12}") {
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
            }
        }
    }
}
