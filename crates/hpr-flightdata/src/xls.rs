//! Excel's older binary format, `.xls`, as a bare worksheet stream: the records of one sheet, written
//! on their own rather than inside the compound file Excel 97 and later save, as some loggers'
//! programs write their exports.
//!
//! Like an `.xlsx` workbook ([`workbook`](crate::workbook)), the file isn't a layout of its own.
//! [`sheets`] writes the sheet's cells out as comma-separated text, one line a row, and [`read`]
//! gives the text to [`import::read`], which recognizes it as it would the file the sheet came
//! from.
//!
//! **The records.** A BIFF stream is a run of records, each a 16-bit identifier, a 16-bit length
//! and that many bytes, all little-endian. A worksheet's stream opens with a BOF record and closes
//! with an EOF record (*OpenOffice.org's Documentation of the Microsoft Excel File Format*, Daniel
//! Rentz, revision 1.42, 2008, §2.1, §4.1.1, §5.8 and §5.37; Microsoft's *[MS-XLS]* describes only
//! BIFF8). The reader takes the stream when its first record is a BOF of any version (identifier
//! `0009`, `0209`, `0409` or `0809`) for a worksheet (type `0010`), and reads these cell records,
//! BIFF2's:
//!
//! | Record | Identifier | The cell | Written as |
//! |---|---|---|---|
//! | NUMBER (§5.71) | `0003` | a 64-bit IEEE 754 number | its shortest decimal that reads back as the same double, such as `0.005` |
//! | INTEGER (§5.60) | `0002` | an unsigned 16-bit integer | its digits |
//! | LABEL (§5.63) | `0004` | text, 8-bit, its length in one byte | its text |
//! | BOOLERR (§5.10) | `0005` | a truth value, or an error such as `#DIV/0!` | `TRUE` or `FALSE`; an error is a gap |
//! | BLANK (§5.7) | `0001` | a cell formatted but empty | nothing |
//!
//! Every other record, formats, fonts, column widths and the sheet's settings, is passed over. A
//! record of a later version that holds a value (NUMBER `0203`, LABEL `0204`, RK, MULRK, LABELSST,
//! RSTRING, BOOLERR `0205`) or a formula is refused by name rather than passed over, so no value is
//! left out unsaid.
//!
//! **What a cell becomes.** A cell is written as the sheet stores it. Its format isn't read: each
//! BIFF2 cell names one in its attributes (§2.5.13), and a number under one other than the first,
//! `General`, is written as the number stored, with a note counting them, as such a number may be
//! a date the sheet would show as one. Text is taken only in plain ASCII: a byte beyond it is in a
//! code page the reader doesn't read, and the sheet is refused. A cell's text that holds a comma or
//! a double quote is quoted, its quotes doubled; a line break inside it becomes a space, as an
//! `.xlsx` sheet's does.
//!
//! **Rows and columns.** Cells are placed by their row and column. Every row is written as wide as
//! the sheet's last column that holds a value; a row with no value is left out. A sheet with no
//! value gives no text, and [`read`] says it has no data. A worksheet's stream names no sheet, so
//! the sheet's name is empty.
//!
//! **Bounds.** The cells must come in order, row by row and, within a row, column by column, as
//! Excel writes them, so that a cell can't be written twice; a column past `IV`, the 256th and last
//! of a BIFF2 to BIFF5 sheet, is refused, as is a record longer than 8,224 bytes (BIFF8's limit,
//! §2.1.2), one that runs past the end, a stream with no EOF, or bytes after it, where a second
//! stream would be passed over. The text written is held to [`MAX_TEXT_BYTES`], each line counted
//! before it is kept. A workbook of several sheets, as Excel 5 and later write inside a compound
//! file, isn't read yet.
//!
//! ```
//! use hpr_flightdata::xls;
//!
//! // An invented worksheet stream: BOF, a header of two labels, two rows of numbers, EOF.
//! fn record(id: u16, data: &[u8]) -> Vec<u8> {
//!     let mut bytes = id.to_le_bytes().to_vec();
//!     bytes.extend((data.len() as u16).to_le_bytes());
//!     bytes.extend(data);
//!     bytes
//! }
//! fn cell(row: u16, column: u16) -> Vec<u8> {
//!     let mut bytes = row.to_le_bytes().to_vec();
//!     bytes.extend(column.to_le_bytes());
//!     bytes.extend([0, 0, 0]); // the cell's attributes: format 0, General
//!     bytes
//! }
//! let mut stream = record(0x0009, &[0, 0, 0x10, 0]);
//! for (column, name) in ["Time (s)", "Altitude (m)"].iter().enumerate() {
//!     let mut data = cell(0, column as u16);
//!     data.push(name.len() as u8);
//!     data.extend(name.as_bytes());
//!     stream.extend(record(0x0004, &data));
//! }
//! for (row, values) in [[0.0, 0.0], [0.05, 1.5]].iter().enumerate() {
//!     for (column, value) in values.iter().enumerate() {
//!         let mut data = cell(row as u16 + 1, column as u16);
//!         data.extend(value.to_le_bytes());
//!         stream.extend(record(0x0003, &data));
//!     }
//! }
//! stream.extend(record(0x000A, &[]));
//!
//! let sheets = xls::sheets(&stream)?;
//! assert_eq!(sheets[0].text, "Time (s),Altitude (m)\n0,0\n0.05,1.5\n");
//! let log = xls::read(&stream)?.remove(0).log?;
//! assert_eq!(log.altitude_m, [0.0, 1.5]);
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use crate::error::LogError;
use crate::import;
use crate::workbook::{Sheet, SheetLog, escaped, shortened};

/// The most text taken from one stream, 256 MiB: [`workbook`](crate::workbook)'s bound.
pub const MAX_TEXT_BYTES: usize = crate::workbook::MAX_TEXT_BYTES;

/// The longest record's data, BIFF8's limit (§2.1.2; BIFF2 to BIFF5's is 2,080 bytes).
const MAX_RECORD_BYTES: usize = 8_224;

/// Columns `A` to `IV`, a BIFF2 to BIFF5 sheet's.
const MAX_COLUMNS: u16 = 256;

/// The BOF record's identifiers, BIFF2's to BIFF8's (§5.8).
const BOF: [u16; 4] = [0x0009, 0x0209, 0x0409, 0x0809];

/// A BOF's type for a worksheet (§5.8).
const WORKSHEET: u16 = 0x0010;

const EOF: u16 = 0x000A;
const BLANK: u16 = 0x0001;
const INTEGER: u16 = 0x0002;
const NUMBER: u16 = 0x0003;
const LABEL: u16 = 0x0004;
const BOOLERR: u16 = 0x0005;

/// Records of later versions that hold a cell's value, refused by name (§5.50, §5.65, §5.71,
/// §5.63, §5.10, §5.87, §5.68, §5.89, §5.103).
const NOT_READ: [(u16, &str); 13] = [
    (0x0006, "FORMULA"),
    (0x0206, "FORMULA"),
    (0x0406, "FORMULA"),
    (0x0007, "STRING"),
    (0x0207, "STRING"),
    (0x0203, "NUMBER (BIFF3 and later)"),
    (0x0204, "LABEL (BIFF3 and later)"),
    (0x0205, "BOOLERR (BIFF3 and later)"),
    (0x027E, "RK"),
    (0x00BD, "MULRK"),
    (0x00FD, "LABELSST"),
    (0x00D6, "RSTRING"),
    (0x04BC, "SHRFMLA"),
];

const FORMAT: &str = "Excel `.xls` worksheet";

/// Whether `bytes` open as a worksheet's BIFF stream: a BOF record of any version, whole, whose
/// type is a sheet, a chart or a macro sheet, or a workbook's globals (§5.8). [`sheets`] reads only
/// a worksheet's.
pub fn recognizes(bytes: &[u8]) -> bool {
    // The record's identifier and length, then the BOF's version and type.
    let Some(header) = bytes.get(..8) else {
        return false;
    };
    let id = u16::from_le_bytes([header[0], header[1]]);
    let length = usize::from(u16::from_le_bytes([header[2], header[3]]));
    let kind = u16::from_le_bytes([header[6], header[7]]);
    BOF.contains(&id)
        && (4..=20).contains(&length)
        && bytes.len() >= 4 + length
        && [0x0005, WORKSHEET, 0x0020, 0x0040].contains(&kind)
}

/// The sheet of a bare worksheet stream, read as a log by [`import::read`], as
/// [`workbook::read`](crate::workbook::read) reads an `.xlsx` workbook's.
///
/// # Errors
///
/// As [`sheets`]; a sheet that doesn't read is not an error, but its [`SheetLog::log`].
pub fn read(bytes: &[u8]) -> Result<Vec<SheetLog>, LogError> {
    Ok(sheets(bytes)?
        .into_iter()
        .map(|sheet| {
            let log = if sheet.text.is_empty() {
                Err(LogError::NoData { format: FORMAT })
            } else {
                import::read(sheet.text.as_bytes()).map(|mut log| {
                    let mut notes =
                        vec!["read from an Excel `.xls` worksheet, its cells as stored".to_owned()];
                    notes.extend(sheet.notes);
                    notes.append(&mut log.notes);
                    log.notes = notes;
                    log
                })
            };
            SheetLog {
                name: sheet.name,
                log,
            }
        })
        .collect())
}

/// The sheet of a bare worksheet stream as text; [`sheets_within`] with [`MAX_TEXT_BYTES`].
///
/// # Errors
///
/// As [`sheets_within`].
pub fn sheets(bytes: &[u8]) -> Result<Vec<Sheet>, LogError> {
    sheets_within(bytes, MAX_TEXT_BYTES)
}

/// The sheet of a bare worksheet stream as text, keeping at most `max_text_bytes` of it: one
/// sheet, as the stream holds one.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when the bytes don't open with a BOF record ([`recognizes`]);
/// [`LogError::Workbook`] when the stream is a workbook's or a chart's rather than a worksheet's,
/// a record runs past the end or past 8,224 bytes, a cell record is shorter than its fields, a
/// record holds a value the reader doesn't read (a formula, or a later version's cell), cells
/// come out of order or past column `IV`, a number isn't finite, a text cell holds a byte beyond
/// ASCII, the stream has no EOF or holds bytes after it, or the text would pass
/// `max_text_bytes`.
pub fn sheets_within(bytes: &[u8], max_text_bytes: usize) -> Result<Vec<Sheet>, LogError> {
    if !recognizes(bytes) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "its bytes don't open with a BIFF stream's BOF record".to_owned(),
        });
    }
    // Invariant: `recognizes` read the BOF's type, its data's bytes 2 and 3.
    let kind = u16::from_le_bytes([bytes[6], bytes[7]]);
    if kind != WORKSHEET {
        return Err(refused(format!(
            "it is a BIFF stream of type {kind:04X}h, a workbook's, a chart's or a macro sheet's: \
             only a worksheet's stream, type 0010h, is read"
        )));
    }
    // Two walks: the first checks the whole stream and finds how wide the rows are, the second
    // writes them, so that no cell is held twice in memory.
    let mut width = 0u16;
    let mut counts = Counts::default();
    walk(bytes, |cell| {
        if cell.value.is_some() {
            width = width.max(cell.column + 1);
        }
        match cell.value {
            Some(Value::Number(_)) if cell.format != 0 => counts.formatted += 1,
            None if cell.error => counts.errors += 1,
            _ => {}
        }
        Ok(())
    })?;
    let mut text = String::new();
    let mut row: Vec<Option<String>> = vec![None; usize::from(width)];
    let mut current: Option<u16> = None;
    let flush = |row: &mut Vec<Option<String>>, text: &mut String| -> Result<(), LogError> {
        if row.iter().all(Option::is_none) {
            return Ok(());
        }
        let line = row
            .iter_mut()
            .map(|cell| cell.take().unwrap_or_default())
            .collect::<Vec<_>>()
            .join(",");
        if text.len() + line.len() + 1 > max_text_bytes {
            return Err(refused(format!(
                "its text would pass {max_text_bytes} bytes, the most taken from one sheet"
            )));
        }
        text.push_str(&line);
        text.push('\n');
        Ok(())
    };
    walk(bytes, |cell| {
        if current != Some(cell.row) {
            flush(&mut row, &mut text)?;
            current = Some(cell.row);
        }
        if let Some(value) = cell.value {
            row[usize::from(cell.column)] = Some(value.text());
        }
        Ok(())
    })?;
    flush(&mut row, &mut text)?;
    let mut notes = Vec::new();
    if counts.formatted > 0 {
        notes.push(format!(
            "{} numbers under a number format other than `General` were kept as stored: the \
             reader doesn't read a BIFF2 sheet's formats, so a date stays its serial number",
            counts.formatted
        ));
    }
    if counts.errors > 0 {
        notes.push(format!(
            "{} cells holding an error value, such as `#DIV/0!`, are gaps",
            counts.errors
        ));
    }
    Ok(vec![Sheet {
        name: String::new(),
        text,
        notes,
    }])
}

/// What [`sheets_within`] counts for its notes.
#[derive(Default)]
struct Counts {
    formatted: usize,
    errors: usize,
}

/// A cell's value, as the stream stores it.
enum Value<'a> {
    Number(f64),
    Integer(u16),
    Text(&'a str),
    Bool(bool),
}

impl Value<'_> {
    /// The value as the sheet's text writes it.
    fn text(&self) -> String {
        match self {
            // Rust writes a double as the shortest decimal that reads back as the same double.
            Value::Number(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
            Value::Text(text) => escaped(text).into_owned(),
            Value::Bool(value) => if *value { "TRUE" } else { "FALSE" }.to_owned(),
        }
    }
}

/// A cell record, read: its place, its format's index and its value, `None` for a gap.
struct Cell<'a> {
    row: u16,
    column: u16,
    format: u8,
    value: Option<Value<'a>>,
    /// The cell holds an error value, written as a gap.
    error: bool,
}

/// Walks the worksheet's records from its BOF to its EOF, giving `each` every cell in order.
fn walk<'a>(
    bytes: &'a [u8],
    mut each: impl FnMut(Cell<'a>) -> Result<(), LogError>,
) -> Result<(), LogError> {
    let mut at = 0usize;
    let mut last: Option<(u16, u16)> = None;
    let mut first = true;
    loop {
        let Some(header) = bytes.get(at..at + 4) else {
            return Err(refused(format!(
                "it ends at byte {} with no EOF record, the end of a sheet's stream",
                bytes.len()
            )));
        };
        let id = u16::from_le_bytes([header[0], header[1]]);
        let length = usize::from(u16::from_le_bytes([header[2], header[3]]));
        if length > MAX_RECORD_BYTES {
            return Err(refused(format!(
                "the record {id:04X}h at byte {at} is {length} bytes long, past the 8,224 a \
                 record may hold"
            )));
        }
        let Some(data) = bytes.get(at + 4..at + 4 + length) else {
            return Err(refused(format!(
                "the record {id:04X}h at byte {at} runs past the end of the file"
            )));
        };
        at += 4 + length;
        if first {
            // Checked by `recognizes`: a worksheet's BOF.
            first = false;
            continue;
        }
        if BOF.contains(&id) {
            return Err(refused(format!(
                "a second BOF record, at byte {}, opens a stream inside the sheet's",
                at - 4 - length
            )));
        }
        if id == EOF {
            if at != bytes.len() {
                return Err(refused(format!(
                    "{} bytes follow the sheet's EOF record, where another stream could be",
                    bytes.len() - at
                )));
            }
            return Ok(());
        }
        if let Some((_, name)) = NOT_READ.iter().find(|(not_read, _)| *not_read == id) {
            return Err(refused(format!(
                "the record at byte {}, {name} ({id:04X}h), holds a value of a kind the reader \
                 doesn't read: it reads BIFF2's NUMBER, INTEGER, LABEL and BOOLERR",
                at - 4 - length
            )));
        }
        let Some(cell) = cell(id, data, at - 4 - length)? else {
            continue;
        };
        if cell.column >= MAX_COLUMNS {
            return Err(refused(format!(
                "a cell in row {} is in column {}, past `IV`, the last of a BIFF2 to BIFF5 sheet",
                u32::from(cell.row) + 1,
                u32::from(cell.column) + 1
            )));
        }
        if last.is_some_and(|last| (cell.row, cell.column) <= last) {
            return Err(refused(format!(
                "the cell in row {}, column {}, comes after a cell at or past it: cells must come \
                 in order, row by row",
                u32::from(cell.row) + 1,
                u32::from(cell.column) + 1
            )));
        }
        last = Some((cell.row, cell.column));
        each(cell)?;
    }
}

/// The cell a BIFF2 cell record holds; `None` for any other record.
fn cell(id: u16, data: &[u8], at: usize) -> Result<Option<Cell<'_>>, LogError> {
    let fixed = match id {
        BLANK => 7,
        INTEGER => 9,
        NUMBER => 15,
        LABEL => 8,
        BOOLERR => 9,
        _ => return Ok(None),
    };
    if data.len() < fixed {
        return Err(refused(format!(
            "the cell record {id:04X}h at byte {at} holds {} bytes, fewer than its {fixed} fields",
            data.len()
        )));
    }
    let row = u16::from_le_bytes([data[0], data[1]]);
    let column = u16::from_le_bytes([data[2], data[3]]);
    // The cell's attributes: its format's index is the low 6 bits of the second byte (§2.5.13).
    let format = data[5] & 0x3F;
    let mut error = false;
    let value = match id {
        INTEGER => Some(Value::Integer(u16::from_le_bytes([data[7], data[8]]))),
        NUMBER => {
            let mut double = [0u8; 8];
            double.copy_from_slice(&data[7..15]);
            let value = f64::from_le_bytes(double);
            if !value.is_finite() {
                return Err(refused(format!(
                    "the number in row {}, column {}, isn't finite",
                    u32::from(row) + 1,
                    u32::from(column) + 1
                )));
            }
            Some(Value::Number(value))
        }
        LABEL => {
            let length = usize::from(data[7]);
            let Some(bytes) = data.get(8..8 + length) else {
                return Err(refused(format!(
                    "the text in row {}, column {}, runs past its record",
                    u32::from(row) + 1,
                    u32::from(column) + 1
                )));
            };
            if !bytes.is_ascii() {
                return Err(refused(format!(
                    "the text in row {}, column {}, {:?}, holds a byte beyond ASCII, in a code \
                     page the reader doesn't read",
                    u32::from(row) + 1,
                    u32::from(column) + 1,
                    shortened(&String::from_utf8_lossy(bytes))
                )));
            }
            // Invariant: ASCII, checked above, is UTF-8.
            let text = std::str::from_utf8(bytes).unwrap_or_default();
            (!text.is_empty()).then_some(Value::Text(text))
        }
        BOOLERR => {
            // The value, then whether it is an error's code rather than a truth value (§5.10).
            if data[8] == 0 {
                Some(Value::Bool(data[7] != 0))
            } else {
                error = true;
                None
            }
        }
        _ => None,
    };
    Ok(Some(Cell {
        row,
        column,
        format,
        value,
        error,
    }))
}

fn refused(message: String) -> LogError {
    LogError::Workbook { message }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: u16, data: &[u8]) -> Vec<u8> {
        let mut bytes = id.to_le_bytes().to_vec();
        bytes.extend(u16::try_from(data.len()).unwrap().to_le_bytes());
        bytes.extend(data);
        bytes
    }

    /// A cell record's place and attributes, `format` its format's index.
    fn place(row: u16, column: u16, format: u8) -> Vec<u8> {
        let mut bytes = row.to_le_bytes().to_vec();
        bytes.extend(column.to_le_bytes());
        bytes.extend([0, format, 0]);
        bytes
    }

    fn number(row: u16, column: u16, value: f64) -> Vec<u8> {
        let mut data = place(row, column, 0);
        data.extend(value.to_le_bytes());
        record(NUMBER, &data)
    }

    fn label(row: u16, column: u16, text: &[u8]) -> Vec<u8> {
        let mut data = place(row, column, 0);
        data.push(u8::try_from(text.len()).unwrap());
        data.extend(text);
        record(LABEL, &data)
    }

    /// A worksheet stream of `cells`, BOF and EOF around them: the BOF as the sample's writer has
    /// it, BIFF5's identifier with 6 bytes of data.
    fn stream(cells: &[Vec<u8>]) -> Vec<u8> {
        let mut bytes = record(0x0809, &[0, 0, 0x10, 0, 0xBB, 0x0D]);
        for cell in cells {
            bytes.extend(cell);
        }
        bytes.extend(record(EOF, &[]));
        bytes
    }

    fn message(result: Result<Vec<Sheet>, LogError>) -> String {
        match result {
            Err(LogError::Workbook { message }) => message,
            other => panic!("expected a workbook refusal, got {other:?}"),
        }
    }

    #[test]
    fn each_cell_record_is_written_as_stored() {
        let mut integer = place(1, 1, 0);
        integer.extend(7u16.to_le_bytes());
        let mut truth = place(1, 2, 0);
        truth.extend([1, 0]);
        let mut error = place(1, 3, 0);
        error.extend([0x07, 1]);
        let bytes = stream(&[
            label(0, 0, b"a, \"b\""),
            label(0, 1, b"two\r\nlines"),
            number(1, 0, 461.629_999_999_859),
            record(INTEGER, &integer),
            record(BOOLERR, &truth),
            record(BOOLERR, &error),
            record(BLANK, &place(1, 4, 0)),
            number(2, 0, -0.005),
        ]);
        let sheet = sheets(&bytes).unwrap().remove(0);
        assert_eq!(sheet.name, "");
        // The row is as wide as the last column holding a value, the truth value's: the error and
        // the blank cell widen nothing.
        assert_eq!(
            sheet.text,
            "\"a, \"\"b\"\"\",two lines,\n461.629999999859,7,TRUE\n-0.005,,\n"
        );
        assert_eq!(
            sheet.notes,
            ["1 cells holding an error value, such as `#DIV/0!`, are gaps"]
        );
    }

    #[test]
    fn rows_with_no_value_are_left_out_and_an_empty_sheet_has_no_data() {
        let bytes = stream(&[record(BLANK, &place(0, 0, 0)), number(3, 1, 2.5)]);
        assert_eq!(sheets(&bytes).unwrap()[0].text, ",2.5\n");
        let empty = stream(&[record(BLANK, &place(0, 0, 0)), label(1, 0, b"")]);
        assert_eq!(sheets(&empty).unwrap()[0].text, "");
        assert!(matches!(
            read(&empty).unwrap()[0].log,
            Err(LogError::NoData { .. })
        ));
    }

    #[test]
    fn a_number_under_a_format_is_kept_as_stored_with_a_note() {
        let mut dated = place(0, 0, 14);
        dated.extend(45_000.25f64.to_le_bytes());
        let sheet = sheets(&stream(&[record(NUMBER, &dated)]))
            .unwrap()
            .remove(0);
        assert_eq!(sheet.text, "45000.25\n");
        assert!(sheet.notes[0].starts_with("1 numbers under a number format other than"));
    }

    #[test]
    fn a_later_versions_cell_record_or_a_formula_is_refused_by_name() {
        for (id, name) in [
            (0x0203, "NUMBER (BIFF3"),
            (0x027E, "RK"),
            (0x0006, "FORMULA"),
        ] {
            let refusal = message(sheets(&stream(&[record(id, &[0; 14])])));
            assert!(refusal.contains(name), "{refusal}");
        }
    }

    #[test]
    fn cells_out_of_order_or_twice_are_refused() {
        for cells in [
            [number(1, 0, 1.0), number(0, 5, 2.0)],
            [number(1, 1, 1.0), number(1, 0, 2.0)],
            [number(1, 1, 1.0), number(1, 1, 2.0)],
        ] {
            let refusal = message(sheets(&stream(&cells)));
            assert!(refusal.contains("must come in order"), "{refusal}");
        }
    }

    #[test]
    fn the_streams_bounds_are_held() {
        // Column 256 is past `IV`; 255 is its last.
        assert!(sheets(&stream(&[number(0, 255, 1.0)])).is_ok());
        let refusal = message(sheets(&stream(&[number(0, 256, 1.0)])));
        assert!(refusal.contains("past `IV`"), "{refusal}");
        // No EOF, a record cut short, bytes after the EOF, a second BOF.
        let whole = stream(&[number(0, 0, 1.0)]);
        let refusal = message(sheets(&whole[..whole.len() - 4]));
        assert!(refusal.contains("no EOF"), "{refusal}");
        let refusal = message(sheets(&whole[..whole.len() - 6]));
        assert!(refusal.contains("runs past the end"), "{refusal}");
        let mut after = whole.clone();
        after.push(0);
        let refusal = message(sheets(&after));
        assert!(refusal.contains("1 bytes follow"), "{refusal}");
        let refusal = message(sheets(&stream(&[record(0x0009, &[0, 0, 0x10, 0])])));
        assert!(refusal.contains("a second BOF"), "{refusal}");
        // A record of 8,225 bytes.
        let refusal = message(sheets(&stream(&[record(0x00EF, &[0; 8_225])])));
        assert!(refusal.contains("past the 8,224"), "{refusal}");
        // A cell record shorter than its fields, and text running past its record.
        let refusal = message(sheets(&stream(&[record(NUMBER, &place(0, 0, 0))])));
        assert!(refusal.contains("fewer than its 15 fields"), "{refusal}");
        let mut long = place(0, 0, 0);
        long.extend([5, b'a']);
        let refusal = message(sheets(&stream(&[record(LABEL, &long)])));
        assert!(refusal.contains("runs past its record"), "{refusal}");
    }

    #[test]
    fn a_value_that_cant_be_written_faithfully_is_refused() {
        let refusal = message(sheets(&stream(&[number(0, 0, f64::NAN)])));
        assert!(refusal.contains("isn't finite"), "{refusal}");
        let refusal = message(sheets(&stream(&[label(0, 0, b"\xb0C")])));
        assert!(refusal.contains("beyond ASCII"), "{refusal}");
    }

    #[test]
    fn the_text_is_held_to_its_bound() {
        let bytes = stream(&[number(0, 0, 12.5), number(1, 0, 3.0)]);
        // "12.5\n3\n" is 7 bytes.
        assert_eq!(sheets_within(&bytes, 7).unwrap()[0].text, "12.5\n3\n");
        let refusal = message(sheets_within(&bytes, 6));
        assert!(refusal.contains("would pass 6 bytes"), "{refusal}");
    }

    #[test]
    fn only_a_worksheets_stream_is_read() {
        assert!(matches!(
            sheets(b"PK\x03\x04"),
            Err(LogError::NotThisFormat { .. })
        ));
        let mut globals = record(0x0809, &[0, 6, 0x05, 0, 0xBB, 0x0D, 0xCC, 0x07]);
        globals.extend(record(EOF, &[]));
        assert!(recognizes(&globals));
        let refusal = message(sheets(&globals));
        assert!(refusal.contains("type 0005h"), "{refusal}");
        // A compound file isn't a bare stream.
        assert!(!recognizes(&[
            0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1
        ]));
    }
}
