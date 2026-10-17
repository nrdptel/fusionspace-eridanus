//! A flyer's own logger, read by a column mapping or a record layout stated in a JSON file: what
//! `hpr analyze` and the Python package take ([ADR-276][adr-276];
//! [M7.1d2, your own logger from the command line and Python][m7-1d2]).
//!
//! [`generic::read_with`], [`records::read`] and [`capture::read`] read a log by a mapping or a
//! layout a program builds. This module reads the mapping or layout from the JSON file a person
//! writes, and the log's bytes by it, so that the command line, Python and a Rust program read
//! one file into one record:
//!
//! - **A mapping file** is a [`Mapping`] as JSON: how a table's columns are read, and which one is
//!   the primary track, if one is ([`Mapping::primary`]). [`header_mapping`] gives the one a
//!   table's header states, to edit. Its schema is
//!   [`schema/cli/analyze-mapping.schema.json`][mapping-schema].
//! - **A layout file** is a [`Layout`]: each field of a record, its name stating its unit as a
//!   table's header does, the byte order, and for a serial console's capture, the block the
//!   records are in. Its schema is [`schema/cli/analyze-layout.schema.json`][layout-schema].
//!
//! **Refusals.** A file is refused naming the field it fails by its path in the JSON, such as
//! `columns[2].unit`, with its line and column, and a key the schema doesn't name is refused
//! too, so that a misspelled one isn't silently left out. A file is at most [`MAX_FILE_BYTES`].
//! What the JSON can't check, such as a unit that doesn't measure its column's quantity, the
//! reader refuses when it reads the log, naming the column.
//!
//! **A workbook.** A mapping reads a table from an `.xlsx` workbook's sheet, its cells as the
//! workbook stores them ([`workbook::sheets`]), when the caller names the sheet, from 0. A
//! workbook read with no sheet named is refused, its sheets listed, rather than one picked.
//!
//! [adr-276]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0276-the-2026-10-16-own-loggers-and-gpx-in-the-analyzer.md
//! [m7-1d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-1d2
//! [mapping-schema]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-mapping.schema.json
//! [layout-schema]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-layout.schema.json

use std::borrow::Cow;

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::capture;
use crate::error::LogError;
use crate::generic::{self, Mapping};
use crate::import;
use crate::log::FlightLog;
use crate::records::{self, ByteOrder, Field, RecordLayout};
use crate::workbook;

/// The most bytes a mapping or a layout file may hold, 1 MiB: [`generic::MAX_COLUMNS`] columns
/// or fields, each with a name of hundreds of characters, take a few tens of kilobytes.
pub const MAX_FILE_BYTES: usize = 1 << 20;

/// A record layout as its file states it: the fields of a record, their byte order, and for a
/// serial console's capture, the block the records are in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    /// Each field of a record, in the record's order. A field's name states its unit as a
    /// table's header does, such as `Time (ms)` or `pressure__pa`: the records are written out
    /// as a table under those names and read by what they state ([`records::read`]), so one
    /// field must be a clock in a unit of time.
    pub fields: Vec<Field>,
    /// The order of each field's bytes: little-endian unless the layout says otherwise.
    #[serde(default)]
    pub byte_order: ByteOrder,
    /// For a serial console's capture of the records as base64, the name of the block they are
    /// in, as the capture's start marker names it ([`capture`]); `None` for a file of the
    /// records' bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
}

impl Layout {
    /// The fields and byte order, as [`records::read`] and [`capture::read`] take them.
    pub fn record_layout(&self) -> RecordLayout {
        RecordLayout {
            fields: self.fields.clone(),
            byte_order: self.byte_order,
        }
    }
}

/// Reads a mapping file's JSON.
///
/// # Errors
///
/// [`LogError::Mapping`] for text of more than [`MAX_FILE_BYTES`], or JSON that isn't a mapping:
/// a key missing or not the schema's, or a value of the wrong kind, naming its path and line.
pub fn mapping_from_json(text: &str) -> Result<Mapping, LogError> {
    from_json(text, "mapping").map_err(|message| LogError::Mapping { message })
}

/// Reads a layout file's JSON.
///
/// # Errors
///
/// [`LogError::Layout`] as [`mapping_from_json`] says for a mapping.
pub fn layout_from_json(text: &str) -> Result<Layout, LogError> {
    from_json(text, "layout").map_err(|message| LogError::Layout { message })
}

/// A mapping's or a layout's file as text: UTF-8, a byte-order mark dropped, or UTF-16 with its
/// byte-order mark, as Windows PowerShell 5.1 writes what `>` sends to a file; `None` for any
/// other bytes.
#[must_use]
pub fn json_text(bytes: &[u8]) -> Option<String> {
    let utf16 = |rest: &[u8], unit: fn([u8; 2]) -> u16| {
        let (pairs, odd) = rest.as_chunks::<2>();
        if !odd.is_empty() {
            return None;
        }
        char::decode_utf16(pairs.iter().map(|pair| unit(*pair)))
            .collect::<Result<String, _>>()
            .ok()
    };
    match bytes {
        [0xEF, 0xBB, 0xBF, rest @ ..] => std::str::from_utf8(rest).ok().map(str::to_owned),
        [0xFF, 0xFE, rest @ ..] => utf16(rest, u16::from_le_bytes),
        [0xFE, 0xFF, rest @ ..] => utf16(rest, u16::from_be_bytes),
        _ => std::str::from_utf8(bytes).ok().map(str::to_owned),
    }
}

/// The JSON read as a `T`, or why not: the path of the field it fails, then serde's words, which
/// end in the line and column.
fn from_json<T: DeserializeOwned>(text: &str, what: &str) -> Result<T, String> {
    if text.len() > MAX_FILE_BYTES {
        return Err(format!(
            "a {what} file is at most {MAX_FILE_BYTES} bytes; this one is {}",
            text.len()
        ));
    }
    let mut json = serde_json::Deserializer::from_str(text);
    let value = serde_path_to_error::deserialize(&mut json).map_err(|error| {
        let words = error.inner().to_string();
        let mut segments: Vec<_> = error.path().iter().collect();
        // The path of a key the schema doesn't name ends at the key, which serde's words name.
        if words.starts_with("unknown field") {
            segments.pop();
        }
        // serde's words quote the value refused, which can be as long as the file.
        let words = cut(&words, MAX_WORDS_CHARS);
        match path(&segments) {
            path if path.is_empty() => words,
            path => format!("`{path}`: {words}"),
        }
    })?;
    json.end().map_err(|error| error.to_string())?;
    Ok(value)
}

/// The most characters of serde's words a refusal quotes: room for the value refused and the
/// list of every value a field takes (a quantity's 20).
const MAX_WORDS_CHARS: usize = 1000;

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

/// A field's path in the JSON, as `columns[2].unit`; empty for the document itself.
fn path(segments: &[&serde_path_to_error::Segment]) -> String {
    use serde_path_to_error::Segment;
    let mut text = String::new();
    for segment in segments {
        let key = match segment {
            Segment::Seq { index } => {
                text.push_str(&format!("[{index}]"));
                continue;
            }
            Segment::Map { key } => key.as_str(),
            Segment::Enum { variant } => variant.as_str(),
            Segment::Unknown => "?",
        };
        if !text.is_empty() {
            text.push('.');
        }
        text.push_str(key);
    }
    text
}

/// What a table's header states, as a mapping to edit, with a note on each column it leaves
/// kept as written, saying why ([`generic::header_reading`]). `sheet` names an `.xlsx`
/// workbook's sheet, from 0; `None` for a text file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] for a table whose header doesn't name a clock with a unit of time;
/// [`LogError::Mapping`] for a workbook with no sheet named, or a sheet it doesn't have, or a sheet
/// named for a file that isn't a workbook.
pub fn header_mapping(
    bytes: &[u8],
    sheet: Option<usize>,
) -> Result<(Mapping, Vec<String>), LogError> {
    generic::header_reading(&table_text(bytes, sheet)?)
}

/// Reads a table by a mapping, as [`generic::read_with`] does: from a text file, or from the
/// `.xlsx` workbook's sheet `sheet`, from 0.
///
/// # Errors
///
/// As [`header_mapping`] for the sheet; otherwise as [`generic::read_with`].
pub fn read_by_mapping(
    bytes: &[u8],
    mapping: &Mapping,
    sheet: Option<usize>,
) -> Result<FlightLog, LogError> {
    generic::read_with(&table_text(bytes, sheet)?, mapping)
}

/// Reads binary records by a layout ([`records::read`]), or a serial console's capture of them,
/// the block the layout names ([`capture::read`]).
///
/// # Errors
///
/// [`LogError::Layout`] for a capture read by a layout that names no block, or a block named for
/// a file that isn't a capture; otherwise as [`records::read`] or [`capture::read`].
pub fn read_by_layout(bytes: &[u8], layout: &Layout) -> Result<FlightLog, LogError> {
    let text = import::decode(bytes);
    let is_capture = capture::recognizes(&text);
    match (&layout.block, is_capture) {
        (Some(block), true) => capture::read(&text, block, &layout.record_layout()),
        (Some(block), false) => Err(LogError::Layout {
            message: format!(
                "the layout names the block {block:?} of a serial console's capture, and the \
                 file isn't one: for a file of the records' bytes, leave `block` out"
            ),
        }),
        (None, true) => {
            let names: Vec<String> = capture::blocks(&text)?
                .into_iter()
                .map(|block| format!("{:?}", block.name))
                .collect();
            Err(LogError::Layout {
                message: format!(
                    "the file is a serial console's capture of records as base64, its blocks {}: \
                     the layout must name the block its records are in, as `block`",
                    names.join(", ")
                ),
            })
        }
        (None, false) => records::read(bytes, &layout.record_layout()),
    }
}

/// The table's text: the file's, or a workbook's sheet's.
fn table_text(bytes: &[u8], sheet: Option<usize>) -> Result<Cow<'_, str>, LogError> {
    let refused = |message: String| LogError::Mapping { message };
    match sheet {
        Some(sheet) => {
            let sheets = workbook::sheets(bytes).map_err(|error| {
                refused(format!(
                    "a sheet is named, and the file isn't an `.xlsx` workbook that reads: {error}"
                ))
            })?;
            let count = sheets.len();
            sheets
                .into_iter()
                .nth(sheet)
                .map(|found| Cow::Owned(found.text))
                .ok_or_else(|| {
                    refused(format!(
                        "the workbook has no sheet {sheet}: {}",
                        match count {
                            0 => "it has none".to_owned(),
                            1 => "its one sheet is 0".to_owned(),
                            n => format!("its {n} sheets are 0 to {}", n - 1),
                        }
                    ))
                })
        }
        None => {
            if bytes.starts_with(b"PK\x03\x04")
                && let Ok(sheets) = workbook::sheets(bytes)
            {
                let names: Vec<String> = sheets
                    .iter()
                    .enumerate()
                    .map(|(index, found)| format!("{index} {:?}", found.name))
                    .collect();
                return Err(refused(format!(
                    "the file is an `.xlsx` workbook, its sheets {}: name the sheet the table is \
                     on, from 0",
                    if names.is_empty() {
                        "none".to_owned()
                    } else {
                        names.join(", ")
                    }
                )));
            }
            Ok(import::decode(bytes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::Quantity;

    /// An invented logger's table: a clock in milliseconds, a barometric altitude above the pad
    /// in feet, an acceleration in g.
    const TABLE: &str = "\
t (ms),alt (ft),accel (g)
0,0,1.0
50,1.5,6.2
100,6.0,6.1
150,13.4,0.4
";

    fn mapping(primary: &str) -> String {
        format!(
            r#"{{
  "header_line": 1,
  "separator": ",",
  "decimal_comma": false,
  "clock": 0,
  "primary": {primary},
  "columns": [
    {{"name": "t (ms)", "quantity": null, "unit": "ms"}},
    {{"name": "alt (ft)", "quantity": "altitude_above_ground", "unit": "ft"}},
    {{"name": "accel (g)", "quantity": "acceleration", "unit": "g"}}
  ]
}}"#
        )
    }

    fn message(error: LogError) -> String {
        match error {
            LogError::Mapping { message } | LogError::Layout { message } => message,
            other => panic!("not a mapping's or layout's refusal: {other}"),
        }
    }

    #[test]
    fn a_marked_primary_track_is_the_records_altitude_on_its_clock() {
        let mapping = mapping_from_json(&mapping("1")).unwrap();
        let log = read_by_mapping(TABLE.as_bytes(), &mapping, None).unwrap();
        assert_eq!(log.time_s, [0.0, 0.05, 0.1, 0.15]);
        let feet = [0.0, 1.5, 6.0, 13.4].map(|ft: f64| ft * 0.3048);
        assert_eq!(log.altitude_m, feet);
        // The track stays a channel too, so the record keeps every sample.
        let altitude: Vec<_> = log.channels_of(Quantity::AltitudeAboveGround).collect();
        assert_eq!(altitude.len(), 1);
        assert_eq!(altitude[0].values, feet);
        assert_eq!(log.channels.len(), 2);
    }

    #[test]
    fn a_mapping_that_marks_no_primary_track_leaves_it_empty() {
        for primary in ["null", "none"] {
            let text = if primary == "none" {
                mapping("null").replace("  \"primary\": null,\n", "")
            } else {
                mapping(primary)
            };
            let mapping = mapping_from_json(&text).unwrap();
            assert_eq!(mapping.primary, None);
            let log = read_by_mapping(TABLE.as_bytes(), &mapping, None).unwrap();
            assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
            assert_eq!(log.channels.len(), 2);
        }
    }

    #[test]
    fn a_primary_track_that_isnt_an_altitude_above_the_pad_is_refused() {
        let refused = |primary: &str, table: &str| {
            let mapping = mapping_from_json(&mapping(primary)).unwrap();
            message(read_by_mapping(table.as_bytes(), &mapping, None).unwrap_err())
        };
        assert_eq!(refused("3", TABLE), "the primary track is column 3, of 3");
        assert_eq!(
            refused("0", TABLE),
            "the primary track, \"t (ms)\", is the clock's column"
        );
        assert_eq!(
            refused("2", TABLE),
            "the primary track, \"accel (g)\", is mapped to `acceleration`: it must be \
             `altitude_above_ground`, the barometric altitude above the pad"
        );
        let skipped = mapping("1").replace(r#""unit": "ft"}"#, r#""unit": "ft", "skip": true}"#);
        let mapping_skipped = mapping_from_json(&skipped).unwrap();
        assert_eq!(
            message(read_by_mapping(TABLE.as_bytes(), &mapping_skipped, None).unwrap_err()),
            "the primary track, \"alt (ft)\", is a column the mapping leaves out"
        );
        let kept = mapping("1").replace(
            r#""quantity": "altitude_above_ground", "unit": "ft""#,
            r#""quantity": null, "unit": null"#,
        );
        let mapping_kept = mapping_from_json(&kept).unwrap();
        assert_eq!(
            message(read_by_mapping(TABLE.as_bytes(), &mapping_kept, None).unwrap_err()),
            "the primary track, \"alt (ft)\", is kept as written: it must be \
             `altitude_above_ground`, the barometric altitude above the pad"
        );
        // A height its unit makes impossible keeps the column as written, so no track.
        let impossible = TABLE.replace("100,6.0,6.1", "100,999999999,6.1");
        assert_eq!(
            refused("1", &impossible),
            "the primary track, \"alt (ft)\", holds values that contradict its unit, \"ft\""
        );
    }

    #[test]
    fn a_primary_track_needs_altitudes_on_a_clock_that_goes_forward() {
        let mapping = mapping_from_json(&mapping("1")).unwrap();
        let empty = "t (ms),alt (ft),accel (g)\n0,,1.0\n50,,6.2\n100,,6.1\n";
        assert_eq!(
            message(read_by_mapping(empty.as_bytes(), &mapping, None).unwrap_err()),
            "the primary track, \"alt (ft)\", holds no number"
        );
        // A clock that steps back is kept in the file's order as a channel's, but a track's
        // times must go forward; a repeated time doesn't either.
        for (back, line) in [("100,6.0", 4), ("50,6.0", 4)] {
            let table = TABLE.replace("150,13.4", back);
            let error = message(read_by_mapping(table.as_bytes(), &mapping, None).unwrap_err());
            assert!(
                error.starts_with(&format!(
                    "the primary track, \"alt (ft)\", needs a clock that goes forward, and the \
                     clock, \"t (ms)\", doesn't on line {}",
                    line + 1
                )),
                "{error}"
            );
        }
        // Without a primary track, the same clock reads, its step back noted.
        let unmarked = mapping_from_json(&super::tests::mapping("null")).unwrap();
        let table = TABLE.replace("150,13.4", "50,6.0");
        let log = read_by_mapping(table.as_bytes(), &unmarked, None).unwrap();
        assert!(log.notes.iter().any(|n| n.contains("steps back 1 times")));
    }

    #[test]
    fn a_mapping_file_is_refused_naming_the_field_it_fails() {
        let refused = |text: &str| message(mapping_from_json(text).unwrap_err());
        let misspelled = mapping("1").replace("\"separator\"", "\"seperator\"");
        assert!(
            refused(&misspelled).starts_with("unknown field `seperator`, expected one of"),
            "{}",
            refused(&misspelled)
        );
        let missing = mapping("1").replace("  \"clock\": 0,\n", "");
        assert!(refused(&missing).starts_with("missing field `clock`"));
        let wrong = mapping("1").replace(r#""unit": "ft""#, r#""unit": 3"#);
        assert!(
            refused(&wrong).starts_with("`columns[1].unit`: invalid type: integer `3`"),
            "{}",
            refused(&wrong)
        );
        assert!(
            refused(&wrong).ends_with("line 9 column 71"),
            "{}",
            refused(&wrong)
        );
        let column_key = mapping("1").replace(r#""unit": "ft""#, r#""unit": "ft", "units": "ft""#);
        assert!(refused(&column_key).starts_with("`columns[1]`: unknown field `units`"));
        let quantity = mapping("1").replace("acceleration", "accel");
        assert!(
            refused(&quantity).starts_with("`columns[2].quantity`: unknown variant `accel`"),
            "{}",
            refused(&quantity)
        );
        assert!(refused(&mapping("-1")).starts_with("`primary`: invalid value: integer `-1`"));
        assert!(refused(&format!("{} {{}}", mapping("1"))).starts_with("trailing characters"));
        let long = format!("{}{}", mapping("1"), " ".repeat(MAX_FILE_BYTES));
        assert!(refused(&long).starts_with("a mapping file is at most 1048576 bytes"));
        // A refused value as long as the file is quoted cut, not whole.
        let huge = mapping("1").replace("acceleration", &"x".repeat(100_000));
        let said = refused(&huge);
        assert!(
            said.starts_with("`columns[2].quantity`: unknown variant `xxx"),
            "{said}"
        );
        assert!(said.ends_with('…'), "{said}");
        assert_eq!(
            said.chars().count(),
            "`columns[2].quantity`: ".chars().count() + MAX_WORDS_CHARS
        );
    }

    #[test]
    fn a_files_text_is_utf8_or_utf16_with_its_mark() {
        let text = mapping("1");
        assert_eq!(json_text(text.as_bytes()), Some(text.clone()));
        let marked = [&[0xEF, 0xBB, 0xBF][..], text.as_bytes()].concat();
        assert_eq!(json_text(&marked), Some(text.clone()));
        let little: Vec<u8> = [0xFF, 0xFE]
            .into_iter()
            .chain(text.encode_utf16().flat_map(u16::to_le_bytes))
            .collect();
        assert_eq!(json_text(&little), Some(text.clone()));
        let big: Vec<u8> = [0xFE, 0xFF]
            .into_iter()
            .chain(text.encode_utf16().flat_map(u16::to_be_bytes))
            .collect();
        assert_eq!(json_text(&big), Some(text.clone()));
        // Read, each is the mapping the text states.
        assert_eq!(
            mapping_from_json(&json_text(&little).unwrap()).unwrap(),
            mapping_from_json(&text).unwrap()
        );
        assert_eq!(json_text(&little[..little.len() - 1]), None);
        assert_eq!(json_text(&[0xFF, 0xFE, 0x00, 0xD8]), None);
        assert_eq!(json_text(b"{\xff}"), None);
    }

    const LAYOUT: &str = r#"{
  "fields": [
    {"name": "time__ms", "kind": "u32"},
    {"name": "pressure__pa", "kind": "f32"}
  ]
}"#;

    /// Three records of `LAYOUT`, little-endian.
    fn records() -> Vec<u8> {
        let mut bytes = Vec::new();
        for (time, pressure) in [(0_u32, 101_325.0_f32), (20, 101_300.0), (40, 101_250.0)] {
            bytes.extend(time.to_le_bytes());
            bytes.extend(pressure.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn a_layout_file_reads_records_as_the_library_does() {
        let layout = layout_from_json(LAYOUT).unwrap();
        assert_eq!(layout.byte_order, ByteOrder::Little);
        assert_eq!(layout.block, None);
        let log = read_by_layout(&records(), &layout).unwrap();
        assert_eq!(
            log,
            records::read(&records(), &layout.record_layout()).unwrap()
        );
        assert_eq!(log.channels[0].values, [101_325.0, 101_300.0, 101_250.0]);
    }

    #[test]
    fn a_layout_reads_a_captures_block_it_names_and_refuses_one_it_doesnt() {
        let encoded = base64(&records());
        let capture = format!(
            "uart:~$ log dump\n----++++//[[( data start )]]\\\\++++----\n{encoded}\n\
             ----++++//[[( data end )]]\\\\++++----\nuart:~$\n"
        );
        assert!(capture::recognizes(&capture), "{capture}");
        let named = LAYOUT.replace("\n  ]\n}", "\n  ],\n  \"block\": \"data\"\n}");
        let layout = layout_from_json(&named).unwrap();
        let log = read_by_layout(capture.as_bytes(), &layout).unwrap();
        assert_eq!(
            log,
            capture::read(&capture, "data", &layout.record_layout()).unwrap()
        );
        let unnamed = layout_from_json(LAYOUT).unwrap();
        assert_eq!(
            message(read_by_layout(capture.as_bytes(), &unnamed).unwrap_err()),
            "the file is a serial console's capture of records as base64, its blocks \"data\": \
             the layout must name the block its records are in, as `block`"
        );
        assert!(
            message(read_by_layout(&records(), &layout).unwrap_err())
                .starts_with("the layout names the block \"data\" of a serial console's capture")
        );
    }

    #[test]
    fn a_layout_file_is_refused_naming_the_field_it_fails() {
        let refused = |text: &str| message(layout_from_json(text).unwrap_err());
        let kind = LAYOUT.replace("\"f32\"", "\"float\"");
        assert!(
            refused(&kind).starts_with("`fields[1].kind`: unknown variant `float`"),
            "{}",
            refused(&kind)
        );
        let key = LAYOUT.replace("\"fields\"", "\"field\"");
        assert!(refused(&key).starts_with("unknown field `field`"));
        let order = LAYOUT.replace("\n  ]\n}", "\n  ],\n  \"byte_order\": \"middle\"\n}");
        assert!(refused(&order).starts_with("`byte_order`: unknown variant `middle`"));
    }

    #[test]
    fn a_sheet_named_for_a_text_file_is_refused() {
        let mapping = mapping_from_json(&mapping("1")).unwrap();
        let error = message(read_by_mapping(TABLE.as_bytes(), &mapping, Some(0)).unwrap_err());
        assert!(
            error.starts_with("a sheet is named, and the file isn't an `.xlsx` workbook"),
            "{error}"
        );
    }

    /// Standard base64 with padding, as a console capture writes its records.
    fn base64(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut text = String::new();
        for chunk in bytes.chunks(3) {
            let word = chunk.iter().enumerate().fold(0_u32, |word, (i, byte)| {
                word | u32::from(*byte) << (16 - 8 * i)
            });
            for i in 0..4 {
                if i <= chunk.len() {
                    text.push(char::from(ALPHABET[(word >> (18 - 6 * i) & 63) as usize]));
                } else {
                    text.push('=');
                }
            }
        }
        text
    }
}
