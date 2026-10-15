//! Binary records read by a layout the caller states: a team's flight computer that writes each
//! sample as a fixed run of bytes, such as a `u32` clock in milliseconds then 24 `f32` readings,
//! with nothing in the file to say so.
//!
//! No maker documents these files, and their bytes carry no names: a record of a clock and two
//! readings looks like any other 12 bytes. So the reader takes the layout from the caller, each
//! field's name and type in the record's order ([`RecordLayout`]), and never guesses one. The
//! names do the work a table's header does: the records are written out as a table, the names its
//! header and one row a record, and the table is read by [`generic::read`], which reads a column
//! in a unit only where its name states one, such as `timestamp__ms` or `Time (ms)`, and keeps
//! every other as written. A layout whose names state no clock is refused; [`table`] gives the
//! text, for a caller who would rather read it by a column mapping ([`generic::read_with`]).
//!
//! **Records.** The file is records end to end, from its first byte, each the layout's fields with
//! no gap between them, in little-endian byte order unless the layout says big. A field is an
//! unsigned or signed integer of 8, 16, 32 or 64 bits, or an IEEE 754 binary32 or binary64 float
//! ([`FieldKind`]). A float that isn't finite (a NaN or an infinity) is written as an empty cell,
//! which the table's reader keeps as a gap, and counted in a note.
//!
//! **Erased flash.** A logger that saves to flash memory leaves the memory it never wrote erased,
//! every byte `0xFF`, and a dump of the memory may run past its last record into it. A run of
//! records each wholly `0xFF` at the file's end is that memory, not samples: it is dropped, and a
//! note counts it. So are bytes past the last whole record if every one is `0xFF`. Any other bytes
//! past the last whole record are refused, as a layout that doesn't fit the file. An erased record
//! with a written one after it is read as written: its unsigned integers at their largest, its
//! signed ones −1, its floats NaN. So a file with a float clock is refused at such a record, its
//! clock no number; with an integer clock, the record's time and values stay in the log, and a
//! column whose values it pushes past what its unit allows is kept as written.
//!
//! **Bounds.** A layout has from 2 to [`MAX_FIELDS`] fields, each named with 1 to
//! [`MAX_NAME_CHARS`] characters, no two alike, none a number or holding a separator (a comma, a
//! semicolon or `|`), a quotation mark, a control character or spaces at its ends, and the first
//! not opening with `#`, `sep=` or a byte order mark. The table is refused once its text would pass [`MAX_TEXT_BYTES`].
//!
//! How far to trust it: the reader is exact for the bytes and types the caller states; whether the
//! layout is the logger's is the caller's statement. The project's sample files are each held,
//! field by field, to the table the logger's team decoded from the same bytes (`cargo xtask
//! logs`, [ADR-271][adr-271]).
//!
//! [adr-271]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md
//!
//! ```
//! use hpr_flightdata::records::{self, Field, FieldKind, RecordLayout};
//!
//! // Two invented records: a clock in milliseconds and a pressure in kilopascals.
//! let mut bytes = Vec::new();
//! for (ms, kpa) in [(1000_u32, 101.3_f32), (1010, 101.2)] {
//!     bytes.extend(ms.to_le_bytes());
//!     bytes.extend(kpa.to_le_bytes());
//! }
//! let layout = RecordLayout::new(vec![
//!     Field::new("time__ms", FieldKind::U32),
//!     Field::new("pressure__kpa", FieldKind::F32),
//! ]);
//! let log = records::read(&bytes, &layout)?;
//! assert_eq!(log.channels[0].time_s, [1.0, 1.01]);
//! assert_eq!(log.channels[0].values[0], f64::from(101.3_f32) * 1000.0); // pascals
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::error::LogError;
use crate::generic;
use crate::log::{FlightLog, LogFormat};
use crate::table::short;

/// The most fields a layout may have: as many columns as a table may.
pub const MAX_FIELDS: usize = generic::MAX_COLUMNS;

/// The most characters in a field's name.
pub const MAX_NAME_CHARS: usize = 256;

/// The most bytes of text the records may be written out as: a bound on the memory a file can make
/// the reader take, whatever its layout (a binary64 can write over 300 characters).
pub const MAX_TEXT_BYTES: usize = 256 * 1024 * 1024;

/// The format's name, as its errors give it.
const FORMAT: &str = "binary records";

/// The type of one field of a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FieldKind {
    /// An unsigned 8-bit integer.
    U8,
    /// A signed 8-bit integer, two's complement.
    I8,
    /// An unsigned 16-bit integer.
    U16,
    /// A signed 16-bit integer, two's complement.
    I16,
    /// An unsigned 32-bit integer.
    U32,
    /// A signed 32-bit integer, two's complement.
    I32,
    /// An unsigned 64-bit integer.
    U64,
    /// A signed 64-bit integer, two's complement.
    I64,
    /// An IEEE 754 binary32 float.
    F32,
    /// An IEEE 754 binary64 float.
    F64,
}

impl FieldKind {
    /// The field's size, in bytes.
    pub fn bytes(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
        }
    }
}

/// The order of a field's bytes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    /// The least significant byte first, as ARM and x86 processors store a number.
    #[default]
    Little,
    /// The most significant byte first.
    Big,
}

/// One field of a record: its name, as a table's header would write it, and its type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// The field's name, its unit in it where it has one, such as `pressure__kpa` or `Time (ms)`
    /// ([`generic`]'s *Units in a name*).
    pub name: String,
    /// The field's type.
    pub kind: FieldKind,
}

impl Field {
    /// A field of the name and type.
    pub fn new(name: impl Into<String>, kind: FieldKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
}

/// How each record of a file is laid out: its fields, in order, with no gap between them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordLayout {
    /// The fields, in the record's order.
    pub fields: Vec<Field>,
    /// The order of each field's bytes; little-endian if not stated.
    #[serde(default)]
    pub byte_order: ByteOrder,
}

impl RecordLayout {
    /// A little-endian layout of the fields.
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields,
            byte_order: ByteOrder::Little,
        }
    }

    /// A record's size, in bytes: its fields' sizes summed.
    pub fn record_bytes(&self) -> usize {
        self.fields.iter().map(|field| field.kind.bytes()).sum()
    }
}

/// A file's records written out as a table, by [`table`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Table {
    /// The table: a header row of the fields' names, then one row a record, comma-separated,
    /// each line ended by LF.
    pub text: String,
    /// The records written.
    pub records: usize,
    /// A record's size, bytes: record `n`, from 1, starts at byte `(n − 1) × record_bytes`.
    pub record_bytes: usize,
    /// What the reader noticed: erased memory dropped, values that aren't finite.
    pub notes: Vec<String>,
}

fn layout_error(message: String) -> LogError {
    LogError::Layout { message }
}

/// Refuses a layout outside the bounds (module docs, *Bounds*).
fn check(layout: &RecordLayout) -> Result<(), LogError> {
    let count = layout.fields.len();
    // A table's header names two columns at least: a clock and something on it.
    if !(2..=MAX_FIELDS).contains(&count) {
        return Err(layout_error(format!(
            "a layout has from 2 to {MAX_FIELDS} fields; this one has {count}"
        )));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, field) in layout.fields.iter().enumerate() {
        let name = &field.name;
        let chars = name.chars().count();
        let why = if chars == 0 {
            Some("has no name".to_owned())
        } else if chars > MAX_NAME_CHARS {
            Some(format!(
                "has a name of more than {MAX_NAME_CHARS} characters"
            ))
        } else if name.contains([',', ';', '|', '"']) || name.chars().any(char::is_control) {
            // The table reader would take `;` or `|` for the separator.
            Some(format!(
                "is named {}, holding a separator (a comma, a semicolon or `|`), a quotation \
                 mark or a control character",
                quoted(name)
            ))
        } else if index == 0
            && (name.starts_with(['#', '\u{FEFF}'])
                || name.to_ascii_lowercase().starts_with("sep="))
        {
            // The table reader skips a `#` comment, takes `sep=` for the separator's line and
            // drops a byte order mark opening the text.
            Some(format!(
                "is named {}, which a table's first line can't open with",
                quoted(name)
            ))
        } else if name.parse::<f64>().is_ok_and(f64::is_finite) {
            // A header holds no number, so a table's reader doesn't take a row for its header.
            Some(format!("is named {}, a number", quoted(name)))
        } else if name.trim() != name {
            Some(format!(
                "is named {}, with spaces at its ends",
                quoted(name)
            ))
        } else if !seen.insert(name.as_str()) {
            Some(format!("is named {}, as an earlier field is", quoted(name)))
        } else {
            None
        };
        if let Some(why) = why {
            return Err(layout_error(format!(
                "field {} of the layout {why}",
                index + 1
            )));
        }
    }
    Ok(())
}

fn quoted(name: &str) -> String {
    format!("`{}`", short(name))
}

/// One field's value, from its bytes, written as the table's cell; `None` for a float that isn't
/// finite.
fn cell(kind: FieldKind, bytes: &[u8], order: ByteOrder, out: &mut String) -> bool {
    macro_rules! read {
        ($ty:ty) => {{
            let mut word = [0u8; std::mem::size_of::<$ty>()];
            word.copy_from_slice(bytes);
            match order {
                ByteOrder::Little => <$ty>::from_le_bytes(word),
                ByteOrder::Big => <$ty>::from_be_bytes(word),
            }
        }};
    }
    // Writing to a String can't fail.
    let float = |value: f64, out: &mut String| {
        if value.is_finite() {
            let _ = write!(out, "{value}");
            true
        } else {
            false
        }
    };
    match kind {
        FieldKind::U8 => {
            let _ = write!(out, "{}", read!(u8));
        }
        FieldKind::I8 => {
            let _ = write!(out, "{}", read!(i8));
        }
        FieldKind::U16 => {
            let _ = write!(out, "{}", read!(u16));
        }
        FieldKind::I16 => {
            let _ = write!(out, "{}", read!(i16));
        }
        FieldKind::U32 => {
            let _ = write!(out, "{}", read!(u32));
        }
        FieldKind::I32 => {
            let _ = write!(out, "{}", read!(i32));
        }
        FieldKind::U64 => {
            let _ = write!(out, "{}", read!(u64));
        }
        FieldKind::I64 => {
            let _ = write!(out, "{}", read!(i64));
        }
        // A binary32 widens to binary64 exactly, and `{}` writes the shortest decimal that reads
        // back to the same binary64, so the table's number is the field's value.
        FieldKind::F32 => return float(f64::from(read!(f32)), out),
        FieldKind::F64 => return float(read!(f64), out),
    }
    true
}

/// The file's records written out as a table (module docs); [`table_within`] with
/// [`MAX_TEXT_BYTES`].
///
/// # Errors
///
/// As [`table_within`].
pub fn table(bytes: &[u8], layout: &RecordLayout) -> Result<Table, LogError> {
    table_within(bytes, layout, MAX_TEXT_BYTES)
}

/// The file's records written out as a table of at most `max_text_bytes` (module docs).
///
/// # Errors
///
/// [`LogError::Layout`] for a layout outside the bounds, or a table whose text would pass
/// `max_text_bytes`; [`LogError::Binary`] for bytes past the last whole record that aren't all
/// `0xFF`, naming the first; [`LogError::NoData`] for a file with no record but erased memory.
pub fn table_within(
    bytes: &[u8],
    layout: &RecordLayout,
    max_text_bytes: usize,
) -> Result<Table, LogError> {
    check(layout)?;
    let size = layout.record_bytes();
    let whole = bytes.len() / size;
    let tail = &bytes[whole * size..];
    let mut notes = Vec::new();
    if !tail.is_empty() {
        if let Some(at) = tail.iter().position(|&byte| byte != 0xFF) {
            return Err(LogError::Binary {
                format: FORMAT,
                offset: whole * size + at,
                message: format!(
                    "the file's {} bytes aren't whole records of {size} bytes: {} are left past \
                     the last, and they aren't erased memory, all 0xFF",
                    bytes.len(),
                    tail.len()
                ),
            });
        }
        notes.push(format!(
            "{} bytes past the last whole record, each 0xFF, dropped as erased memory",
            tail.len()
        ));
    }
    // The records are walked, never collected: a list of them would take 16 bytes a record.
    let body = &bytes[..whole * size];
    let erased = body
        .rchunks_exact(size)
        .take_while(|record| record.iter().all(|&byte| byte == 0xFF))
        .count();
    if erased > 0 {
        notes.push(format!(
            "the last {erased} of {whole} records are erased memory, every byte 0xFF: dropped"
        ));
    }
    let written = whole - erased;
    if written == 0 {
        return Err(LogError::NoData { format: FORMAT });
    }
    // Each record writes a separator a field at least, so a table past the bound is refused
    // before any of it is written.
    let fewest = written.saturating_mul(layout.fields.len());
    if fewest > max_text_bytes {
        return Err(layout_error(format!(
            "the records, written out as a table, would pass {max_text_bytes} bytes: its \
             {written} records write {fewest} at least"
        )));
    }
    let names: Vec<&str> = layout.fields.iter().map(|f| f.name.as_str()).collect();
    let mut text = names.join(",");
    text.push('\n');
    let mut not_finite = 0usize;
    for record in body.chunks_exact(size).take(written) {
        let mut at = 0;
        for (index, field) in layout.fields.iter().enumerate() {
            if index > 0 {
                text.push(',');
            }
            let width = field.kind.bytes();
            if !cell(
                field.kind,
                &record[at..at + width],
                layout.byte_order,
                &mut text,
            ) {
                not_finite += 1;
            }
            at += width;
        }
        text.push('\n');
        if text.len() > max_text_bytes {
            return Err(layout_error(format!(
                "the records, written out as a table, would pass {max_text_bytes} bytes"
            )));
        }
    }
    if not_finite > 0 {
        notes.push(format!(
            "{not_finite} float values aren't finite, NaN or infinite: kept as gaps"
        ));
    }
    Ok(Table {
        text,
        records: written,
        record_bytes: size,
        notes,
    })
}

/// Reads a file of binary records by the layout the caller states (module docs), as
/// [`LogFormat::BinaryRecords`].
///
/// # Errors
///
/// As [`table`]; [`LogError::Layout`] if the layout's names state no clock with a unit of time,
/// such as `Time (ms)`; [`LogError::Binary`], naming the record and its offset, for a record the
/// table's reader refuses, such as a clock that isn't a number; otherwise as [`generic::read`]
/// refuses the table, such as a clock that never steps forward.
pub fn read(bytes: &[u8], layout: &RecordLayout) -> Result<FlightLog, LogError> {
    let table = table(bytes, layout)?;
    let mut log = read_table(&table)?;
    log.format = LogFormat::BinaryRecords;
    Ok(log)
}

/// The table read by its header, the records' notes before the table's.
pub(crate) fn read_table(table: &Table) -> Result<FlightLog, LogError> {
    let mut log = generic::read(&table.text).map_err(|error| match error {
        LogError::NotThisFormat { .. } => layout_error(
            "the layout's names state no clock with a unit of time, such as `Time (ms)` or \
             `timestamp__us`: name the clock so, or read the records' table by a column mapping \
             (`records::table`, then `generic::read_with`)"
                .to_owned(),
        ),
        // The table's line `n` is record `n − 1`: name the record, not a line the caller never saw.
        LogError::Syntax { line, message, .. } if line >= 2 => LogError::Binary {
            format: FORMAT,
            offset: (line - 2).saturating_mul(table.record_bytes),
            message: format!("record {}: {message}", line - 1),
        },
        other => other,
    })?;
    let mut notes = vec![format!(
        "{} binary records read by the caller's layout, written out as a table and read by its \
         header",
        table.records
    )];
    notes.extend(table.notes.iter().cloned());
    notes.append(&mut log.notes);
    log.notes = notes;
    Ok(log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::Quantity;

    fn layout(fields: &[(&str, FieldKind)]) -> RecordLayout {
        RecordLayout::new(
            fields
                .iter()
                .map(|(name, kind)| Field::new(*name, *kind))
                .collect(),
        )
    }

    fn clock_and_pressure() -> RecordLayout {
        layout(&[
            ("time__ms", FieldKind::U32),
            ("pressure__kpa", FieldKind::F32),
        ])
    }

    fn records(rows: &[(u32, f32)]) -> Vec<u8> {
        rows.iter()
            .flat_map(|(ms, kpa)| ms.to_le_bytes().into_iter().chain(kpa.to_le_bytes()))
            .collect()
    }

    #[test]
    fn each_record_is_a_row_its_names_read_in_their_units() {
        let bytes = records(&[(1000, 101.3), (1010, 101.25), (1020, 101.2)]);
        let log = read(&bytes, &clock_and_pressure()).unwrap();
        assert_eq!(log.format, LogFormat::BinaryRecords);
        assert_eq!(log.channels[0].time_s, [1.0, 1.01, 1.02]);
        let pressure = &log.channels[0];
        assert_eq!(pressure.column, "pressure__kpa");
        assert_eq!(pressure.quantity, Quantity::Pressure);
        // The binary32 widened exactly, then kilopascals to pascals.
        assert_eq!(pressure.values[1], f64::from(101.25_f32) * 1000.0);
        assert!(
            log.notes[0].starts_with("3 binary records"),
            "{:?}",
            log.notes
        );
    }

    #[test]
    fn every_kind_is_read_at_its_width_in_either_byte_order() {
        let kinds = [
            (FieldKind::U8, vec![0xFE], "254"),
            (FieldKind::I8, vec![0xFE], "-2"),
            (FieldKind::U16, vec![0x01, 0x02], "513"),
            (FieldKind::I16, vec![0xFF, 0xFE], "-257"),
            (FieldKind::U32, vec![0x01, 0, 0, 0x80], "2147483649"),
            (FieldKind::I32, vec![0xFF, 0xFF, 0xFF, 0xFF], "-1"),
            (FieldKind::U64, vec![0xFF; 8], "18446744073709551615"),
            (
                FieldKind::I64,
                vec![0, 0, 0, 0, 0, 0, 0, 0x80],
                "-9223372036854775808",
            ),
            (
                FieldKind::F32,
                0.1_f32.to_le_bytes().to_vec(),
                "0.10000000149011612",
            ),
            (FieldKind::F64, 0.1_f64.to_le_bytes().to_vec(), "0.1"),
        ];
        for (kind, little, written) in kinds {
            assert_eq!(kind.bytes(), little.len(), "{kind:?}");
            let mut out = String::new();
            assert!(cell(kind, &little, ByteOrder::Little, &mut out));
            assert_eq!(out, written, "{kind:?} little-endian");
            let big: Vec<u8> = little.iter().rev().copied().collect();
            let mut out = String::new();
            assert!(cell(kind, &big, ByteOrder::Big, &mut out));
            assert_eq!(out, written, "{kind:?} big-endian");
        }
    }

    /// Values at the edges of their types survive the table exactly: the largest and smallest
    /// binary64, a subnormal of each width, negative zero, and integers at their ends.
    #[test]
    fn extreme_values_survive_the_table_exactly() {
        let layout = layout(&[
            ("time__ms", FieldKind::U32),
            ("a", FieldKind::F64),
            ("b", FieldKind::F32),
            ("c", FieldKind::I64),
            ("d", FieldKind::U64),
        ]);
        let rows: [(u32, f64, f32, i64, u64); 4] = [
            (0, f64::MAX, f32::MAX, i64::MIN, u64::MAX),
            (10, f64::from_bits(1), f32::from_bits(1), i64::MAX, 0),
            (20, -0.0, -0.0, -1, 1),
            (30, f64::MIN_POSITIVE, f32::MIN_POSITIVE, 0, 1 << 53),
        ];
        let mut bytes = Vec::new();
        for (t, a, b, c, d) in rows {
            bytes.extend(t.to_le_bytes());
            bytes.extend(a.to_le_bytes());
            bytes.extend(b.to_le_bytes());
            bytes.extend(c.to_le_bytes());
            bytes.extend(d.to_le_bytes());
        }
        let log = read(&bytes, &layout).unwrap();
        let column = |name: &str| {
            &log.channels
                .iter()
                .find(|channel| channel.column == name)
                .unwrap()
                .values
        };
        for (i, (_, a, b, _, _)) in rows.iter().enumerate() {
            assert_eq!(column("a")[i].to_bits(), a.to_bits(), "row {i}");
            assert_eq!(column("b")[i].to_bits(), f64::from(*b).to_bits(), "row {i}");
        }
        // A 64-bit integer is written in full; read as a number, it is the nearest binary64.
        let table = table(&bytes, &layout).unwrap();
        assert!(
            table
                .text
                .contains(",-9223372036854775808,18446744073709551615\n")
        );
        assert_eq!(column("d")[3], 9_007_199_254_740_992.0);
    }

    #[test]
    fn a_big_endian_layout_reads_its_records() {
        let mut bytes = Vec::new();
        for (ms, kpa) in [(1000_u32, 101.3_f32), (1010, 101.2)] {
            bytes.extend(ms.to_be_bytes());
            bytes.extend(kpa.to_be_bytes());
        }
        let mut layout = clock_and_pressure();
        layout.byte_order = ByteOrder::Big;
        assert_eq!(
            read(&bytes, &layout).unwrap().channels[0].time_s,
            [1.0, 1.01]
        );
        // Read little-endian, the same bytes are another clock.
        layout.byte_order = ByteOrder::Little;
        assert_ne!(
            read(&bytes, &layout).map(|log| log.channels[0].time_s.clone()),
            Ok(vec![1.0, 1.01])
        );
    }

    #[test]
    fn a_float_that_isnt_finite_is_a_gap_and_counted() {
        let bytes = records(&[(1000, 101.3), (1010, f32::NAN), (1020, f32::INFINITY)]);
        let log = read(&bytes, &clock_and_pressure()).unwrap();
        let values = &log.channels[0].values;
        assert!(values[1].is_nan() && values[2].is_nan(), "{values:?}");
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("2 float values aren't finite")),
            "{:?}",
            log.notes
        );
    }

    #[test]
    fn trailing_erased_records_and_bytes_are_dropped_and_counted() {
        let mut bytes = records(&[(1000, 101.3), (1010, 101.2)]);
        bytes.extend([0xFF; 8 * 3 + 5]);
        let table = table(&bytes, &clock_and_pressure()).unwrap();
        assert_eq!(table.records, 2);
        assert_eq!(table.text.lines().count(), 3);
        assert_eq!(
            table.notes,
            [
                "5 bytes past the last whole record, each 0xFF, dropped as erased memory",
                "the last 3 of 5 records are erased memory, every byte 0xFF: dropped",
            ]
        );
    }

    #[test]
    fn an_erased_record_before_a_written_one_is_read_as_written() {
        let mut bytes = records(&[(1000, 101.3), (1010, 101.3), (1020, 101.2)]);
        bytes.extend([0xFF; 8]);
        bytes.extend(records(&[(1040, 101.2), (1050, 101.1)]));
        let table = table(&bytes, &clock_and_pressure()).unwrap();
        assert_eq!(table.records, 6);
        assert!(table.text.contains("\n4294967295,\n"), "{}", table.text);
        // Its clock at its largest, its float a gap, as the logger's bytes say.
        let log = read(&bytes, &clock_and_pressure()).unwrap();
        let pressure = &log.channels[0];
        assert_eq!(pressure.time_s[3], 4_294_967.295);
        assert!(pressure.values[3].is_nan());
        assert_eq!(pressure.time_s[4], 1.04);
    }

    #[test]
    fn bytes_past_the_last_record_that_arent_erased_are_refused_at_the_first() {
        let mut bytes = records(&[(1000, 101.3), (1010, 101.2)]);
        bytes.extend([0xFF, 0xFF, 0x00]);
        match table(&bytes, &clock_and_pressure()) {
            Err(LogError::Binary {
                offset, message, ..
            }) => {
                assert_eq!(offset, 18);
                assert!(message.contains("3 are left past the last"), "{message}");
            }
            other => panic!("expected a binary refusal, got {other:?}"),
        }
    }

    #[test]
    fn a_file_of_only_erased_memory_has_no_data() {
        for bytes in [vec![], vec![0xFF; 16], vec![0xFF; 3]] {
            assert_eq!(
                table(&bytes, &clock_and_pressure()),
                Err(LogError::NoData { format: FORMAT }),
                "{} bytes",
                bytes.len()
            );
        }
    }

    #[test]
    fn a_layout_outside_the_bounds_is_refused_naming_why() {
        let many: Vec<(String, FieldKind)> = (0..=MAX_FIELDS)
            .map(|i| (format!("f{i}"), FieldKind::U8))
            .collect();
        let many = RecordLayout::new(
            many.into_iter()
                .map(|(name, kind)| Field::new(name, kind))
                .collect(),
        );
        let long = "x".repeat(MAX_NAME_CHARS + 1);
        // A clock, then a field of the name.
        let named = |name: &str| layout(&[("t (ms)", FieldKind::U8), (name, FieldKind::U8)]);
        let first = |name: &str| layout(&[(name, FieldKind::U8), ("t (ms)", FieldKind::U8)]);
        let cases = [
            (layout(&[]), "from 2 to 512 fields; this one has 0"),
            (layout(&[("t (ms)", FieldKind::U8)]), "this one has 1"),
            (many, "this one has 513"),
            (named(""), "field 2 of the layout has no name"),
            (named(&long), "more than 256 characters"),
            (
                named("a,b"),
                "holding a separator (a comma, a semicolon or `|`)",
            ),
            (named("a;b"), "holding a separator"),
            (named("a|b"), "holding a separator"),
            (named("a\"b"), "a quotation mark"),
            (named("a\nb"), "a control character"),
            (named("a\tb"), "a control character"),
            (named(" a"), "spaces at its ends"),
            (named("12"), "field 2 of the layout is named `12`, a number"),
            (named("1e3"), "a number"),
            (
                first("#t (ms)"),
                "field 1 of the layout is named `#t (ms)`, which a table's first line can't open with",
            ),
            (first("SEP=x"), "can't open with"),
            (first("\u{FEFF}t (ms)"), "can't open with"),
            (
                named("t (ms)"),
                "field 2 of the layout is named `t (ms)`, as an earlier",
            ),
        ];
        for (layout, why) in cases {
            match table(&[0; 64], &layout) {
                Err(LogError::Layout { message }) => {
                    assert!(message.contains(why), "{message} lacks {why}");
                }
                other => panic!("expected a layout refusal for {why}, got {other:?}"),
            }
        }
        // A name of exactly the most characters is a name; so is `#` past the first field, and a
        // word that only parses as a number that isn't finite.
        let most = "t".repeat(MAX_NAME_CHARS);
        for layout in [named(&most), named("#2"), named("nan"), named("inf")] {
            assert!(table(&[0; 4], &layout).is_ok(), "{layout:?}");
        }
    }

    #[test]
    fn names_that_state_no_clock_are_refused_pointing_to_a_mapping() {
        let bytes = records(&[(1000, 101.3), (1010, 101.2)]);
        let unnamed = layout(&[("col1", FieldKind::U32), ("col2", FieldKind::F32)]);
        match read(&bytes, &unnamed) {
            Err(LogError::Layout { message }) => {
                assert!(message.contains("generic::read_with"), "{message}");
            }
            other => panic!("expected a layout refusal, got {other:?}"),
        }
        // The same records, read by a mapping of their table.
        let table = table(&bytes, &unnamed).unwrap();
        let mut mapping = generic::mapping(&table.text.replace("col1", "time (ms)")).unwrap();
        mapping.columns[0].name = "col1".to_owned();
        let log = generic::read_with(&table.text, &mapping).unwrap();
        assert_eq!(log.channels[0].time_s, [1.0, 1.01]);
    }

    #[test]
    fn a_table_past_its_bound_is_refused() {
        let bytes = records(&[(1000, 101.3), (1010, 101.2)]);
        let text = table(&bytes, &clock_and_pressure()).unwrap().text;
        let exact = table_within(&bytes, &clock_and_pressure(), text.len()).unwrap();
        assert_eq!(exact.text, text);
        match table_within(&bytes, &clock_and_pressure(), text.len() - 1) {
            Err(LogError::Layout { message }) => {
                assert!(
                    message.contains(&format!("would pass {} bytes", text.len() - 1)),
                    "{message}"
                );
            }
            other => panic!("expected the bound, got {other:?}"),
        }
    }

    #[test]
    fn a_table_that_cant_fit_is_refused_before_it_is_written() {
        // Two fields a record write 2 bytes at least: 1,000 records can't fit in 1,999.
        let bytes = vec![0u8; 2 * 1000];
        let layout = layout(&[("t (ms)", FieldKind::U8), ("a", FieldKind::U8)]);
        match table_within(&bytes, &layout, 1999) {
            Err(LogError::Layout { message }) => {
                assert!(
                    message
                        .ends_with("would pass 1999 bytes: its 1000 records write 2000 at least")
                )
            }
            other => panic!("expected the bound, got {other:?}"),
        }
    }

    #[test]
    fn a_record_the_tables_reader_refuses_is_named_with_its_offset() {
        // A float clock: an erased record before written ones writes no clock, which the
        // table's reader refuses; the error names the record, from 1, and where it starts.
        let layout = layout(&[("time (s)", FieldKind::F32), ("alt (m)", FieldKind::F32)]);
        let mut bytes = Vec::new();
        for (t, alt) in [(0.0_f32, 1.0_f32), (0.1, 2.0)] {
            bytes.extend(t.to_le_bytes());
            bytes.extend(alt.to_le_bytes());
        }
        bytes.extend([0xFF; 8]);
        bytes.extend(0.3_f32.to_le_bytes());
        bytes.extend(3.0_f32.to_le_bytes());
        match read(&bytes, &layout) {
            Err(LogError::Binary {
                offset, message, ..
            }) => {
                assert_eq!(offset, 16);
                assert!(message.starts_with("record 3: "), "{message}");
            }
            other => panic!("expected the record named, got {other:?}"),
        }
    }

    #[test]
    fn a_layout_survives_its_json() {
        let mut layout = clock_and_pressure();
        layout.byte_order = ByteOrder::Big;
        let json = serde_json::to_string(&layout).unwrap();
        assert_eq!(
            json,
            r#"{"fields":[{"name":"time__ms","kind":"u32"},{"name":"pressure__kpa","kind":"f32"}],"byte_order":"big"}"#
        );
        assert_eq!(serde_json::from_str::<RecordLayout>(&json).unwrap(), layout);
        // The byte order defaults to little-endian.
        let little: RecordLayout =
            serde_json::from_str(r#"{"fields":[{"name":"t (s)","kind":"f64"}]}"#).unwrap();
        assert_eq!(little.byte_order, ByteOrder::Little);
    }
}
