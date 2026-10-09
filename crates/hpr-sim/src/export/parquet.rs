//! A recording as an Apache Parquet file, written by hand from the format's specification: no
//! dependency, and it builds for wasm32 like the rest of the core.
//!
//! Source: the Apache Parquet format specification, `parquet-format` at commit `bf09939`: its
//! `README.md` (file layout, data pages) and `src/main/thrift/parquet.thrift` (the footer's
//! structures), with the footer in the Apache Thrift compact protocol
//! (`doc/specs/thrift-compact-protocol.md` in `apache/thrift`).
//!
//! The file is the simplest Parquet a reader must accept:
//!
//! - one `REQUIRED` `DOUBLE` column per recorded column, under the recorder's column name;
//! - one row group holding every row, or none when nothing was recorded;
//! - version 1 data pages of at most [`PARQUET_PAGE_ROWS`] values each, `PLAIN` encoded (each
//!   value is its 8 little-endian IEEE 754 bytes) and uncompressed. A required column that isn't
//!   nested has no repetition or definition levels, so a page is its values and nothing else.
//!
//! It carries no statistics, dictionary, index or compression. Its footer's key-value metadata
//! names the program that wrote it, as every file HPR Sim writes does ([`hpr_core::tool`]): `tool`
//! (`FusionSpace HPR`), `tool_version` and `designation` (`FS-ACHERNAR · SW · TOOL 001`), then
//! any pairs a caller adds ([`parquet_with`]), such as the motor catalog's date.

use crate::error::SimError;
use crate::recorder::Recorder;

/// The most values in one Parquet data page: 1024 doubles, the 8 KiB page the specification
/// recommends (`README.md`, "Configurations").
pub const PARQUET_PAGE_ROWS: usize = 1024;

/// The file's first and last four bytes.
const MAGIC: &[u8; 4] = b"PAR1";

// Codes from `parquet.thrift`.
const TYPE_DOUBLE: i32 = 5;
const REPETITION_REQUIRED: i32 = 0;
const ENCODING_PLAIN: i32 = 0;
const ENCODING_RLE: i32 = 3;
const CODEC_UNCOMPRESSED: i32 = 0;
const PAGE_DATA: i32 = 0;

// Compact-protocol type codes.
const T_I16: u8 = 4;
const T_I32: u8 = 5;
const T_I64: u8 = 6;
const T_BINARY: u8 = 8;
const T_LIST: u8 = 9;
const T_STRUCT: u8 = 12;

/// A Thrift compact-protocol encoder for the few types the footer needs.
struct Compact {
    out: Vec<u8>,
    /// The last field id written in each open struct, innermost last.
    last_field: Vec<i16>,
}

impl Compact {
    fn new() -> Self {
        Self {
            out: Vec::new(),
            last_field: vec![0],
        }
    }

    /// An unsigned LEB128 varint.
    fn varint(&mut self, mut value: u64) {
        while value >= 0x80 {
            // Truncation keeps the low seven bits, which the mask selects anyway.
            self.out.push((value as u8 & 0x7f) | 0x80);
            value >>= 7;
        }
        self.out.push(value as u8);
    }

    /// A signed integer, zigzag mapped (0, −1, 1, −2 … to 0, 1, 2, 3 …) then as a varint.
    fn int(&mut self, value: i64) {
        self.varint(((value << 1) ^ (value >> 63)) as u64);
    }

    /// A field header: the id as a delta from the last one in this struct when it is 1 to 15,
    /// else the type byte followed by the id itself.
    fn field(&mut self, id: i16, kind: u8) {
        let last = self.last_field.last().copied().unwrap_or(0);
        let delta = id - last;
        if (1..=15).contains(&delta) {
            self.out.push(((delta as u8) << 4) | kind);
        } else {
            self.out.push(kind);
            self.int(i64::from(id));
        }
        if let Some(slot) = self.last_field.last_mut() {
            *slot = id;
        }
    }

    fn i16(&mut self, id: i16, value: i16) {
        self.field(id, T_I16);
        self.int(i64::from(value));
    }

    fn i32(&mut self, id: i16, value: i32) {
        self.field(id, T_I32);
        self.int(i64::from(value));
    }

    fn i64(&mut self, id: i16, value: i64) {
        self.field(id, T_I64);
        self.int(value);
    }

    fn bytes(&mut self, value: &[u8]) {
        self.varint(value.len() as u64);
        self.out.extend_from_slice(value);
    }

    fn string(&mut self, id: i16, value: &str) {
        self.field(id, T_BINARY);
        self.bytes(value.as_bytes());
    }

    /// A list header: the size in the high four bits when it is under 15, else `1111` there and
    /// the size as a varint after.
    fn list(&mut self, id: i16, element: u8, len: usize) {
        self.field(id, T_LIST);
        if len < 15 {
            self.out.push(((len as u8) << 4) | element);
        } else {
            self.out.push(0xf0 | element);
            self.varint(len as u64);
        }
    }

    /// Opens a struct that is a list element (no field header).
    fn open(&mut self) {
        self.last_field.push(0);
    }

    /// Opens a struct that is a field.
    fn open_field(&mut self, id: i16) {
        self.field(id, T_STRUCT);
        self.open();
    }

    /// Closes the innermost struct with the stop byte.
    fn close(&mut self) {
        self.out.push(0);
        self.last_field.pop();
    }
}

/// A count as the `i32` or `i64` a Parquet field holds.
fn count<T: TryFrom<usize>>(value: usize) -> Result<T, SimError> {
    T::try_from(value).map_err(|_| SimError::Unsupported {
        what: "a recording too large for a Parquet file's counts",
    })
}

/// The footer's key-value metadata, in the order written and before any a caller adds
/// ([`parquet_with`]): the program that wrote the file, its version and its designation in the
/// FusionSpace product system.
const KEY_VALUE: [(&str, &str); 3] = [
    ("tool", hpr_core::tool::NAME),
    ("tool_version", hpr_core::tool::VERSION),
    ("designation", hpr_core::tool::DESIGNATION),
];

/// The footer's `created_by`, in the form Parquet's own writers give it, `<program> version
/// <version>`: [`hpr_core::tool::NAME`] and [`hpr_core::tool::VERSION`], which a test holds it to.
const CREATED_BY: &str = concat!("FusionSpace HPR version ", env!("CARGO_PKG_VERSION"));

/// Where one column's pages sit in the file.
struct Chunk {
    offset: usize,
    size: usize,
}

/// The recorded rows as an Apache Parquet file: one `DOUBLE` column per recorded column, named as
/// in [`Recorder::columns`], one row group, `PLAIN` encoding, no compression. Every value reads
/// back as the exact `f64` recorded. The footer's key-value metadata names the program that wrote
/// the file: `tool` = `FusionSpace HPR`, `tool_version` = its version and `designation` =
/// `FS-ACHERNAR · SW · TOOL 001`.
///
/// Needs the `parquet` feature. Returns the file's bytes; the caller writes them.
///
/// # Errors
///
/// [`SimError::Domain`] for a value that isn't finite, as the text exports refuse one;
/// [`SimError::Unsupported`] for a recorder with no columns, or with more rows than a Parquet
/// count holds.
pub fn parquet(recorder: &Recorder) -> Result<Vec<u8>, SimError> {
    parquet_with(recorder, &[])
}

/// The recorded rows as [`parquet()`] writes them, with the pairs of `about`, in its order, after
/// the program's three in the footer's key-value metadata: such as the motor catalog's date and
/// how far to trust the recording, as [`super::json_with`] carries them beside its rows. Each is a
/// `KeyValue` of `parquet.thrift`, its key and its value UTF-8 strings. Keys other programs read
/// as their own, such as Apache Arrow's `ARROW:schema` or pandas' `pandas`, are the caller's to
/// avoid: a reader decodes them, and may refuse the file when they hold something else.
///
/// Needs the `parquet` feature. Returns the file's bytes; the caller writes them.
///
/// # Errors
///
/// As [`parquet()`]; also [`SimError::Unsupported`] for a key named `tool`, `tool_version` or
/// `designation`, or given twice: a key written twice, which readers that collect the pairs into
/// a map would keep only one of.
pub fn parquet_with(recorder: &Recorder, about: &[(&str, &str)]) -> Result<Vec<u8>, SimError> {
    for (n, (key, _)) in about.iter().enumerate() {
        if KEY_VALUE.iter().any(|(own, _)| own == key) || about[..n].iter().any(|(k, _)| k == key) {
            return Err(SimError::Unsupported {
                what: "a Parquet metadata key named tool, tool_version or designation, or given twice",
            });
        }
    }
    let columns = recorder.columns();
    if columns.is_empty() {
        return Err(SimError::Unsupported {
            what: "a Parquet file with no columns",
        });
    }
    let rows = recorder.rows();
    for row in rows {
        for &value in row {
            super::finite("recorded value", value)?;
        }
    }

    let mut out = MAGIC.to_vec();
    let mut chunks = Vec::with_capacity(columns.len());
    if !rows.is_empty() {
        for column in 0..columns.len() {
            let offset = out.len();
            for page in rows.chunks(PARQUET_PAGE_ROWS) {
                let size: i32 = count(page.len() * 8)?;
                let mut header = Compact::new();
                header.i32(1, PAGE_DATA);
                header.i32(2, size);
                header.i32(3, size);
                header.open_field(5);
                header.i32(1, count(page.len())?);
                header.i32(2, ENCODING_PLAIN);
                header.i32(3, ENCODING_RLE);
                header.i32(4, ENCODING_RLE);
                header.close();
                header.close();
                out.extend_from_slice(&header.out);
                for row in page {
                    out.extend_from_slice(&row[column].to_le_bytes());
                }
            }
            chunks.push(Chunk {
                offset,
                size: out.len() - offset,
            });
        }
    }

    let mut footer = Compact::new();
    footer.i32(1, 1);
    footer.list(2, T_STRUCT, columns.len() + 1);
    footer.open();
    footer.string(4, "schema");
    footer.i32(5, count(columns.len())?);
    footer.close();
    for name in &columns {
        footer.open();
        footer.i32(1, TYPE_DOUBLE);
        footer.i32(3, REPETITION_REQUIRED);
        footer.string(4, name);
        footer.close();
    }
    footer.i64(3, count(rows.len())?);
    footer.list(4, T_STRUCT, usize::from(!chunks.is_empty()));
    if !chunks.is_empty() {
        let total: i64 = count(chunks.iter().map(|c| c.size).sum())?;
        footer.open();
        footer.list(1, T_STRUCT, chunks.len());
        for (name, chunk) in columns.iter().zip(&chunks) {
            let size: i64 = count(chunk.size)?;
            footer.open();
            // Deprecated; 0 when no column metadata is written outside the footer.
            footer.i64(2, 0);
            footer.open_field(3);
            footer.i32(1, TYPE_DOUBLE);
            footer.list(2, T_I32, 1);
            footer.int(i64::from(ENCODING_PLAIN));
            footer.list(3, T_BINARY, 1);
            footer.bytes(name.as_bytes());
            footer.i32(4, CODEC_UNCOMPRESSED);
            footer.i64(5, count(rows.len())?);
            footer.i64(6, size);
            footer.i64(7, size);
            footer.i64(9, count(chunk.offset)?);
            footer.close();
            footer.close();
        }
        footer.i64(2, total);
        footer.i64(3, count(rows.len())?);
        footer.i64(5, count(MAGIC.len())?);
        footer.i64(6, total);
        footer.i16(7, 0);
        footer.close();
    }
    // `key_value_metadata` (field 5): a list of KeyValue structs, `1: required string key`,
    // `2: optional string value`.
    footer.list(5, T_STRUCT, KEY_VALUE.len() + about.len());
    for &(key, value) in KEY_VALUE.iter().chain(about) {
        footer.open();
        footer.string(1, key);
        footer.string(2, value);
        footer.close();
    }
    footer.string(6, CREATED_BY);
    footer.close();

    let length: u32 = count(footer.out.len())?;
    out.extend_from_slice(&footer.out);
    out.extend_from_slice(&length.to_le_bytes());
    out.extend_from_slice(MAGIC);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use parquet_reader::basic::{Encoding, PageType, Repetition, Type};
    use parquet_reader::file::reader::{FileReader, SerializedFileReader};
    use parquet_reader::record::RowAccessor;

    use super::*;
    use crate::flight::{FlightSettings, Simulation};
    use crate::rail::Rail;
    use crate::recorder::Channel;
    use hpr_atmos::ConstantWind;

    use crate::testing::{design, windy_environment};

    /// Reads `file` with Apache's Parquet implementation and returns its column names and rows.
    fn read(file: Vec<u8>) -> (SerializedFileReader<Bytes>, Vec<String>, Vec<Vec<f64>>) {
        let reader = SerializedFileReader::new(Bytes::from(file)).unwrap();
        let schema = reader.metadata().file_metadata().schema_descr_ptr();
        let names = schema
            .columns()
            .iter()
            .map(|c| c.name().to_owned())
            .collect();
        for column in schema.columns() {
            assert_eq!(column.physical_type(), Type::DOUBLE);
            assert_eq!(
                column.self_type().get_basic_info().repetition(),
                Repetition::REQUIRED
            );
        }
        let rows = reader
            .get_row_iter(None)
            .unwrap()
            .map(|row| {
                let row = row.unwrap();
                (0..row.len()).map(|i| row.get_double(i).unwrap()).collect()
            })
            .collect();
        (reader, names, rows)
    }

    #[test]
    fn apaches_reader_reads_back_every_recorded_value_exactly() {
        let (recorder, ..) = super::super::tests::flown();
        assert!(recorder.rows().len() > 20);
        let (reader, names, rows) = read(parquet(&recorder).unwrap());
        assert_eq!(names, recorder.columns());
        assert_eq!(rows, recorder.rows());
        let metadata = reader.metadata();
        assert_eq!(metadata.num_row_groups(), 1);
        assert_eq!(
            metadata.file_metadata().num_rows(),
            recorder.rows().len() as i64
        );
        assert_eq!(
            metadata.file_metadata().created_by(),
            Some(concat!(
                "FusionSpace HPR version ",
                env!("CARGO_PKG_VERSION")
            ))
        );
        assert_eq!(
            CREATED_BY,
            format!(
                "{} version {}",
                hpr_core::tool::NAME,
                hpr_core::tool::VERSION
            )
        );
    }

    /// Apache's reader finds the program's name, version and designation in the footer's
    /// key-value metadata, in a file with rows and in one without.
    #[test]
    fn the_key_value_metadata_names_the_program_version_and_designation() {
        let (recorder, ..) = super::super::tests::flown();
        let empty = Recorder::with_rows(vec![Channel::Time], Vec::new());
        for recorder in [recorder, empty] {
            let (reader, names, rows) = read(parquet(&recorder).unwrap());
            assert_eq!(names, recorder.columns());
            assert_eq!(rows, recorder.rows());
            let pairs: Vec<(String, Option<String>)> = reader
                .metadata()
                .file_metadata()
                .key_value_metadata()
                .unwrap()
                .iter()
                .map(|kv| (kv.key.clone(), kv.value.clone()))
                .collect();
            let pair = |key: &str, value: &str| (key.to_owned(), Some(value.to_owned()));
            assert_eq!(
                pairs,
                [
                    pair("tool", "FusionSpace HPR"),
                    pair("tool_version", env!("CARGO_PKG_VERSION")),
                    pair("designation", "FS-ACHERNAR · SW · TOOL 001"),
                ]
            );
        }
    }

    /// Apache's reader finds added pairs after the program's three, in the order given, with the
    /// recording unchanged; a key the program writes, or one given twice, is refused by name.
    #[test]
    fn added_pairs_follow_the_programs_and_a_repeated_key_is_refused() {
        let (recorder, ..) = super::super::tests::flown();
        let note = "How far to trust it. \u{b1}5 % \u{2014} a note past 127 bytes. ".repeat(4);
        let about = [
            ("catalog_as_of", "2026-01-02"),
            ("kind", "simulated"),
            ("trust", &note),
        ];
        let (reader, names, rows) = read(parquet_with(&recorder, &about).unwrap());
        assert_eq!(names, recorder.columns());
        assert_eq!(rows, recorder.rows());
        let pairs: Vec<(String, Option<String>)> = reader
            .metadata()
            .file_metadata()
            .key_value_metadata()
            .unwrap()
            .iter()
            .map(|kv| (kv.key.clone(), kv.value.clone()))
            .collect();
        let keys: Vec<&str> = pairs.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(
            keys,
            [
                "tool",
                "tool_version",
                "designation",
                "catalog_as_of",
                "kind",
                "trust"
            ]
        );
        assert_eq!(pairs[5].1.as_deref(), Some(note.as_str()));
        assert!(note.len() > 127);

        for about in [
            &[("tool", "x")][..],
            &[("tool_version", "x")],
            &[("designation", "x")],
            &[("trust", "a"), ("kind", "b"), ("trust", "c")],
        ] {
            assert!(
                matches!(
                    parquet_with(&recorder, about),
                    Err(SimError::Unsupported { what })
                        if what.starts_with("a Parquet metadata key named tool")
                ),
                "{about:?}"
            );
        }
    }

    /// Every channel (30 columns, past the compact protocol's short list header) at every step:
    /// enough rows for several pages per column.
    #[test]
    fn a_long_recording_of_every_channel_splits_into_pages() {
        let sim = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            windy_environment(ConstantWind::new(4.0, 0.0).unwrap()),
            Rail::vertical(3.0),
            FlightSettings::default(),
        )
        .unwrap();
        let mut recorder = Recorder::new(Channel::ALL.to_vec(), Some(0.01)).unwrap();
        sim.run(&mut recorder).unwrap();
        let n = recorder.rows().len();
        assert!(n > 2 * PARQUET_PAGE_ROWS, "{n} rows");
        assert!(recorder.columns().len() >= 15);

        let file = parquet(&recorder).unwrap();
        let (reader, names, rows) = read(file.clone());
        assert_eq!(names, recorder.columns());
        assert_eq!(rows, recorder.rows());
        // The chunks follow the magic number and each other, and the last ends where the footer
        // begins: before the footer, its 4-byte length and the closing magic number.
        let metadata = reader.metadata().row_group(0);
        let mut end = MAGIC.len() as i64;
        for chunk in metadata.columns() {
            assert_eq!(chunk.data_page_offset(), end);
            assert_eq!(chunk.compressed_size(), chunk.uncompressed_size());
            assert_eq!(chunk.num_values(), n as i64);
            end += chunk.compressed_size();
        }
        let length = u32::from_le_bytes(file[file.len() - 8..file.len() - 4].try_into().unwrap());
        assert_eq!(end, (file.len() - 8 - length as usize) as i64);
        assert_eq!(metadata.total_byte_size(), end - MAGIC.len() as i64);
        assert_eq!(metadata.compressed_size(), metadata.total_byte_size());
        assert_eq!(metadata.file_offset(), Some(MAGIC.len() as i64));
        assert_eq!(metadata.ordinal(), Some(0));

        let group = reader.get_row_group(0).unwrap();
        for column in 0..names.len() {
            let pages: Vec<_> = group
                .get_column_page_reader(column)
                .unwrap()
                .map(Result::unwrap)
                .collect();
            assert_eq!(pages.len(), n.div_ceil(PARQUET_PAGE_ROWS));
            let mut values = 0;
            for page in &pages {
                assert_eq!(page.page_type(), PageType::DATA_PAGE);
                assert_eq!(page.encoding(), Encoding::PLAIN);
                assert!(page.num_values() as usize <= PARQUET_PAGE_ROWS);
                values += page.num_values() as usize;
            }
            assert_eq!(values, n);
        }
    }

    #[test]
    fn an_empty_recording_is_a_file_with_no_row_groups() {
        let recorder = Recorder::with_rows(vec![Channel::Time, Channel::Mach], Vec::new());
        let (reader, names, rows) = read(parquet(&recorder).unwrap());
        assert_eq!(names, ["time_s", "mach"]);
        assert!(rows.is_empty());
        assert_eq!(reader.metadata().num_row_groups(), 0);
        assert_eq!(reader.metadata().file_metadata().num_rows(), 0);
    }

    #[test]
    fn extreme_values_keep_their_bits() {
        let values = [
            -0.0,
            f64::MIN_POSITIVE,
            5e-324,
            f64::MAX,
            f64::MIN,
            0.1 + 0.2,
        ];
        let recorder = Recorder::with_rows(
            vec![Channel::Time, Channel::Mach],
            values.iter().map(|&v| vec![v, -v]).collect(),
        );
        let (_, _, rows) = read(parquet(&recorder).unwrap());
        assert_eq!(rows.len(), values.len());
        for (row, &v) in rows.iter().zip(&values) {
            assert_eq!(row[0].to_bits(), v.to_bits());
            assert_eq!(row[1].to_bits(), (-v).to_bits());
        }
    }

    #[test]
    fn no_columns_or_a_value_that_is_not_finite_is_refused() {
        let empty = Recorder::with_rows(Vec::new(), vec![Vec::new()]);
        assert!(matches!(
            parquet(&empty),
            Err(SimError::Unsupported { what }) if what.contains("no columns")
        ));
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let recorder = Recorder::with_rows(vec![Channel::Time], vec![vec![0.0], vec![bad]]);
            assert!(matches!(
                parquet(&recorder),
                Err(SimError::Domain { what: "recorded value", value })
                    if value.to_bits() == bad.to_bits()
            ));
        }
    }

    /// A two-row file of one column, byte for byte, each byte worked out by hand from
    /// `parquet.thrift` and the compact protocol. It pins every field the reader ignores, such as
    /// the uncompressed sizes and the row group's offset.
    #[test]
    fn a_small_file_is_the_bytes_the_specification_gives() {
        let recorder = Recorder::with_rows(vec![Channel::Time], vec![vec![1.0], vec![2.0]]);
        // A list of 3 structs (0x3c), the program's pairs and no more.
        assert_eq!(parquet(&recorder).unwrap(), small_file(0x3c, &[]));
    }

    /// The small file of [`a_small_file_is_the_bytes_the_specification_gives`] with two pairs
    /// added, worked out by hand the same way: the list now of 5 structs (0x5c), and the added
    /// pairs after the program's three, in the order given. The note's 200 bytes take a two-byte
    /// varint length: 200 is 0b1_1001000, so 0xc8 (the low seven bits with the continuation bit),
    /// then 0x01.
    #[test]
    fn added_pairs_are_the_bytes_the_specification_gives() {
        let recorder = Recorder::with_rows(vec![Channel::Time], vec![vec![1.0], vec![2.0]]);
        let note = "n".repeat(200);
        let mut added: Vec<u8> = Vec::new();
        // "catalog_as_of" 13 bytes, "2026-01-02" 10, then a stop.
        added.extend([0x18, 0x0d]);
        added.extend(b"catalog_as_of");
        added.extend([0x18, 0x0a]);
        added.extend(b"2026-01-02");
        added.push(0x00);
        // "trust" 5 bytes, the note 200, then a stop.
        added.extend([0x18, 0x05]);
        added.extend(b"trust");
        added.extend([0x18, 0xc8, 0x01]);
        added.extend(note.as_bytes());
        added.push(0x00);
        assert_eq!(
            parquet_with(
                &recorder,
                &[("catalog_as_of", "2026-01-02"), ("trust", &note)]
            )
            .unwrap(),
            small_file(0x5c, &added)
        );
        assert_eq!(
            parquet_with(&recorder, &[]).unwrap(),
            parquet(&recorder).unwrap()
        );
    }

    /// The two-row file's bytes with the key-value list's header `list` and the bytes `added`
    /// after the program's three pairs.
    fn small_file(list: u8, added: &[u8]) -> Vec<u8> {
        let mut expected: Vec<u8> = b"PAR1".to_vec();
        // PageHeader, each field one on from the last (delta 1: 0x15 for an i32) but the fifth:
        // type DATA_PAGE (0), uncompressed and compressed sizes 16 (zigzag 32), DataPageHeader
        // (field 5, delta 2, a struct: 0x2c) of 2 values (zigzag 4), PLAIN (0), RLE levels
        // (zigzag 6), then two stops.
        expected.extend([
            0x15, 0x00, 0x15, 0x20, 0x15, 0x20, 0x2c, 0x15, 0x04, 0x15, 0x00, 0x15, 0x06, 0x15,
            0x06, 0x00, 0x00,
        ]);
        // The two values, little-endian: 1.0 and 2.0. The chunk is 17 + 16 = 33 bytes from 4.
        expected.extend(1.0_f64.to_le_bytes());
        expected.extend(2.0_f64.to_le_bytes());
        let footer_start = expected.len();
        // FileMetaData: version 1; schema, a list of 2 structs: the root ("schema", 6 bytes, 1
        // child) and `time_s` (DOUBLE = zigzag 10, REQUIRED at field 3 by delta 2); 2 rows.
        expected.extend([0x15, 0x02, 0x19, 0x2c, 0x48, 0x06]);
        expected.extend(b"schema");
        expected.extend([0x15, 0x02, 0x00, 0x15, 0x0a, 0x25, 0x00, 0x18, 0x06]);
        expected.extend(b"time_s");
        expected.extend([0x00, 0x16, 0x04]);
        // Row groups, a list of 1: its columns, a list of 1 ColumnChunk: file_offset 0 (field 2),
        // then ColumnMetaData (3): DOUBLE, encodings [PLAIN], path ["time_s"], UNCOMPRESSED, 2
        // values, sizes 33 and 33 (zigzag 66), data_page_offset 4 (field 9 by delta 2, zigzag 8).
        expected.extend([
            0x19, 0x1c, 0x19, 0x1c, 0x26, 0x00, 0x1c, 0x15, 0x0a, 0x19, 0x15,
        ]);
        expected.extend([0x00, 0x19, 0x18, 0x06]);
        expected.extend(b"time_s");
        expected.extend([
            0x15, 0x00, 0x16, 0x04, 0x16, 0x42, 0x16, 0x42, 0x26, 0x08, 0x00, 0x00,
        ]);
        // The row group: total_byte_size 33, 2 rows, file_offset 4 (field 5 by delta 2),
        // total_compressed_size 33, ordinal 0 (i16), stop.
        expected.extend([
            0x16, 0x42, 0x16, 0x04, 0x26, 0x08, 0x16, 0x42, 0x14, 0x00, 0x00,
        ]);
        // key_value_metadata (field 5 by delta 1, a list: 0x19), a list of `list` structs. Each
        // KeyValue is its key (field 1 by delta 1, a binary: 0x18) and value (field 2, 0x18 again),
        // each a varint byte length then the UTF-8 bytes, and a stop. The lengths: "tool" 4,
        // "FusionSpace HPR" 15, "tool_version" 12, "designation" 11, and "FS-ACHERNAR · SW · TOOL 001"
        // 29: 27 characters, of which the two middle dots (U+00B7) are two bytes each, 0xc2
        // 0xb7. The version's length is read off the version, which is under 128 bytes, so one
        // varint byte.
        let version = env!("CARGO_PKG_VERSION");
        expected.extend([0x19, list]);
        expected.extend([0x18, 0x04]);
        expected.extend(b"tool");
        expected.extend([0x18, 0x0f]);
        expected.extend(b"FusionSpace HPR");
        expected.push(0x00);
        expected.extend([0x18, 0x0c]);
        expected.extend(b"tool_version");
        expected.extend([0x18, version.len() as u8]);
        expected.extend(version.as_bytes());
        expected.push(0x00);
        expected.extend([0x18, 0x0b]);
        expected.extend(b"designation");
        expected.extend([0x18, 0x1d]);
        expected.extend(b"FS-ACHERNAR \xc2\xb7 SW \xc2\xb7 TOOL 001");
        expected.push(0x00);
        expected.extend(added);
        // created_by (field 6 by delta 1: 0x18), then the file's stop.
        let created_by = concat!("FusionSpace HPR version ", env!("CARGO_PKG_VERSION"));
        expected.extend([0x18, created_by.len() as u8]);
        expected.extend(created_by.as_bytes());
        expected.push(0x00);
        let footer_length = (expected.len() - footer_start) as u32;
        expected.extend(footer_length.to_le_bytes());
        expected.extend(b"PAR1");
        expected
    }

    /// The compact protocol's encodings against its specification's own examples.
    #[test]
    fn compact_protocol_matches_its_specification() {
        let mut c = Compact::new();
        // "50399 = 11000100 11011111" becomes 0xdf 0x89 0x03 as a varint.
        c.varint(50399);
        assert_eq!(c.out, [0xdf, 0x89, 0x03]);
        // ZigZag: 0, -1, 1, -2, 2147483647, -2147483648 to 0, 1, 2, 3, 4294967294, 4294967295.
        for (value, expected) in [
            (0, 0u64),
            (-1, 1),
            (1, 2),
            (-2, 3),
            (2_147_483_647, 4_294_967_294),
            (-2_147_483_648, 4_294_967_295),
        ] {
            let mut c = Compact::new();
            c.int(value);
            let mut v = Compact::new();
            v.varint(expected);
            assert_eq!(c.out, v.out, "{value}");
        }
        // A field header in short form (delta 1 to 15), then in long form (delta 16): type byte,
        // then the zigzag id.
        let mut c = Compact::new();
        c.i32(1, 7);
        c.i32(17, 7);
        assert_eq!(c.out, [0x15, 0x0e, 0x05, 0x22, 0x0e]);
        // Lists: size in the header's high bits up to 14, else 0xf and a varint size.
        let mut c = Compact::new();
        c.list(1, T_I32, 14);
        c.list(2, T_STRUCT, 15);
        assert_eq!(c.out, [0x19, 0xe5, 0x19, 0xfc, 0x0f]);
        // A struct's fields count from zero again, and the outer count resumes after it.
        let mut c = Compact::new();
        c.open_field(3);
        c.i16(1, -1);
        c.close();
        c.i64(4, 1);
        assert_eq!(c.out, [0x3c, 0x14, 0x01, 0x00, 0x16, 0x02]);
    }
}
