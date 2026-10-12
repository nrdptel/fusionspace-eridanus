//! PerfectFlite's `.pf2` flight logs, as its software writes a Pnut's, a StratoLogger's or a
//! StratoLoggerCF's flights, and the CSV and text exports it writes of them ([`read_export`]).
//! Every `.pf2` among the project's sample logs reads, of all three models (`cargo xtask logs`).
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

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};

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
/// [`LogError::Syntax`] for a row that isn't numbers, a time that doesn't increase, or a `Data:`
/// line without a time or an altitude column; [`LogError::Unit`] for a stated height that isn't
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

/// The column names a PerfectFlite export writes, as the collection's 16 exports write them. A
/// header row with any other name, or the same name in another case, is another logger's or a
/// person's, such as the RRC3's `Time,Altitude,Pressure,…` or a bare `time,altitude`, whose
/// units nothing states.
const EXPORT_COLUMNS: [&str; 8] = [
    "Time",
    "Altitude",
    "Velocity",
    "Temperature (F)",
    "Temperature (C)",
    "Temp.",
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

/// Reads one of the exports PerfectFlite's software writes from a flight: comma-separated, with a
/// header row such as `Time,Altitude,Velocity,Temperature (F),Voltage`, or tab-separated, with a
/// header row such as `Time`, `Altitude`, `Temp.`, `Voltage` and temperatures written with their unit,
/// such as `58.7F`.
///
/// The units are a `.pf2`'s: seconds, feet, feet per second, and the temperature's stated unit.
/// The header doesn't state the altitude's, and two checks bear out feet on the collection's 16
/// exports: each of the 12 CSV files with its flight's `.pf2` beside it peaks at that file's
/// height, and every export's climb to apogee takes about as long as a fall from that many feet
/// would (in meters, the fall would take 1.8 times longer than the climb did, which no rocket
/// slowed by drag can match). An export doesn't name the logger's model, so the logger reads
/// `PerfectFlite`. A column the reader doesn't know, such as `Acceleration (calculated)`, is kept
/// as written.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't an export's header row;
/// [`LogError::Syntax`] for a cell that isn't a number, a temperature in a unit other than its
/// column's, a row with more cells than the header, or a time that doesn't increase;
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
    let Some(separator) = export_separator(header) else {
        return Err(LogError::NotThisFormat {
            format: EXPORT_CSV,
            message: format!(
                "its first line, {:?}, isn't a header row of `Time` then `Altitude`",
                short(header)
            ),
        });
    };
    let (format, name) = if separator == ',' {
        (LogFormat::PerfectFliteCsv, EXPORT_CSV)
    } else {
        (LogFormat::PerfectFliteText, EXPORT_TEXT)
    };
    let rows: Vec<(usize, Vec<&str>)> = lines
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
        notes: Vec::new(),
    };
    fill(&mut log, name, &columns, &values)?;
    log.notes.extend(notes.kept);
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
        for (index, (column, _)) in columns.iter().enumerate() {
            let cell = cells.get(index).copied();
            match column {
                Column::Time => time = cell,
                Column::Altitude => altitude = cell,
                _ => {}
            }
            values[index].push(cell.map_or(f64::NAN, |value| si(*column, value)));
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

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = "PerfectFlite Pnut\r\nFirmware: 1.0\r\nSerial Number: 17\r\n\
                        Apogee: 1280' AGL\r\nGround Elevation: 600' MSL\r\nNumSamps: 3\r\n\
                        Flight Number: 2\r\nComments: \r\n\r\n\
                        Data: (Time, Altitude, Velocity, Temperature (F), Voltage)\r\n";

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
