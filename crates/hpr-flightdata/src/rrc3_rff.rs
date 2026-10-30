//! Missile Works' RRC3 raw flight files, `.rff`: what its mDACS software saves when it downloads a
//! flight from the altimeter, before any export.
//!
//! The RRC3 records its pressure at 20 Hz, and its temperature and battery voltage at 1 Hz (*RRC3+
//! / RRC3 User Manual* v1.71, p. 23). mDACS keeps the download as a .NET `BinaryFormatter` stream,
//! the layout Microsoft publishes as MS-NRBF (*[MS-NRBF]: .NET Remoting: Binary Format Data
//! Structure*, revision 13.0, 2024-11-19): a serialized `mDACS.AnalysisSupport+FlightRecord` whose
//! flight data is a `List<Int16>`, the altimeter's memory as it came off the board, one 16-bit word
//! a slot. The stream has no columns and no times:
//!
//! - a word below `0x4000` is a pressure, in tenths of a millibar (10 Pa): `0x4000` is 1,638.4
//!   mbar, above the 1,200 mbar at which the RRC3 itself reports a pressure sensor fault (p. 9),
//!   so no reading reaches it;
//! - a word at or above `0x4000` is an auxiliary word, written in pairs once a second,
//!   presumably the temperature and battery voltage the manual names, in counts: Debrief found
//!   mDACS's export's temperature and voltage tracking them, though not linearly. The file holds
//!   no calibration for them, and they are kept as written ([`Quantity::Unrecognized`]), in the
//!   order the file writes them. A few stand alone, not in a pair (in each sample file, three: one
//!   near apogee, one later, and a last `0xFFFF`); the first two fall where the manual's low-voltage
//!   samples taken as the drogue and the main fire would (p. 23), but that is a guess, and they
//!   are kept as written in a channel of their own. Three or more auxiliary words in a row, which
//!   no sample file holds, are read as pairs from the first, with an odd last one kept alone;
//! - the readings start at the first word, among the first eight, that a pressure on a
//!   launch pad can be, at least 500 mbar (5,570 m in the standard atmosphere, above
//!   any launch site): the words before it are kept, as written, in a note;
//! - a zero word ends the log: the list's slots after it are unwritten.
//!
//! **Times.** The stream has none, so the pressure's clock is the RRC3's 20 Hz, from the manual,
//! the `n`th reading at `n / 20` s, and the auxiliary pairs' times are the time of the reading
//! before them. Two checks stand behind it: the pairs, one a second, must count the same seconds as
//! the readings do at 20 a second, within 10% and two pairs, or the file is refused; and wherever
//! the project's sample logs hold mDACS's export of the same flight, its pressures match the raw
//! readings one for one, and its `Time` column is this 20 Hz clock as mDACS keeps it: 0.05 s
//! summed in single precision, so that it runs a hundredth of a second ahead late in a long log
//! (`cargo xtask logs` counts the pairs and compares them).
//!
//! **The record.** One [`Quantity::Pressure`] channel, `Pressure`, and the three auxiliary
//! channels. The primary track is empty, with a note: the file holds no altitude, and working one
//! out of the pressure is a reading, not this reader's to make (milestone [M7.2], the readings,
//! takes it), as mDACS's export's `Altitude` is mDACS's.
//!
//! **Refused**, naming the byte: a stream with no `List<Int16>` of the flight record's, or more
//! than one candidate for it; a list whose slots run past the file's end; readings that don't start
//! among the first eight words; a word other than a zero after the zero that ends the
//! log; fewer than two seconds of readings; and a clock check that fails.
//!
//! **Where this comes from.** The layout, the `0x4000` threshold, the 500 mbar floor, the
//! preamble's length and the clock check follow the reader in Debrief
//! (`lib/parsers/missileworksRff.ts`, MIT, the project owner's own; see `THIRD-PARTY-NOTICES.md`),
//! which was worked out from a raw file and mDACS's export of the same flight; the record types
//! and their fields are MS-NRBF's (§2.4.3.3 `ArraySinglePrimitive`, §2.5.3 `MemberReference`,
//! §2.6.1 `SerializationHeaderRecord`). Where this reader departs from Debrief's: it keeps the
//! auxiliary words, paired and single, and the preamble where Debrief drops them; it refuses a
//! stream with two candidate lists where Debrief takes the longer; and it refuses data after the
//! zero that ends the log where Debrief stops reading.
//!
//! [M7.2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};

/// The format's name in errors.
const FORMAT: &str = "RRC3 .rff";

/// The class mDACS serializes, written in ASCII in the stream's class record.
const CLASS_NAME: &[u8] = b"mDACS.AnalysisSupport+FlightRecord";

/// How far into the file the class name may first appear, bytes: the stream's header and the class
/// record come first.
const CLASS_WITHIN: usize = 4096;

/// MS-NRBF's `ArraySinglePrimitive` record type (§2.1.2.1, `RecordTypeEnumeration` 15).
const ARRAY_SINGLE_PRIMITIVE: u8 = 0x0f;

/// MS-NRBF's primitive type code for a 16-bit signed integer (§2.1.2.3, `PrimitiveTypeEnumeration`
/// 7).
const PRIMITIVE_INT16: u8 = 7;

/// MS-NRBF's `MemberReference` record type (§2.1.2.1, `RecordTypeEnumeration` 9).
const MEMBER_REFERENCE: u8 = 0x09;

/// The most slots a list may have, 2²⁴: at 20 words a second, over nine days of flight.
const MAX_SLOTS: i32 = 1 << 24;

/// The first word that isn't a pressure: `0x4000` tenths of a millibar is 1,638.4 mbar. A
/// threshold, not a bit test: a word with bit 15 set and bit 14 clear is no pressure either.
pub(crate) const AUXILIARY: u16 = 0x4000;

/// The least pressure a reading on a launch pad can be, tenths of a millibar: 500 mbar, 5,570 m in
/// the standard atmosphere, above any launch site.
pub(crate) const PAD_FLOOR: u16 = 5_000;

/// How many words may precede the first reading.
pub(crate) const MAX_PREAMBLE: usize = 8;

/// The RRC3's pressure rate, Hz (RRC3 manual p. 23).
const RATE_HZ: f64 = 20.0;

/// Pascals in a tenth of a millibar.
const COUNT_PA: f64 = 10.0;

/// Whether the bytes are an MS-NRBF stream holding mDACS's flight record: a serialization header
/// record (type 0) whose major and minor versions are 1 and 0, and the record's class name within
/// the first [`CLASS_WITHIN`] bytes.
pub(crate) fn recognizes(bytes: &[u8]) -> bool {
    bytes.len() >= 17
        && bytes[0] == 0
        && i32_at(bytes, 9) == Some(1)
        && i32_at(bytes, 13) == Some(0)
        && contains(&bytes[..bytes.len().min(CLASS_WITHIN)], CLASS_NAME)
}

/// A little-endian 32-bit integer at `at`, if the file is long enough.
fn i32_at(bytes: &[u8], at: usize) -> Option<i32> {
    let word = bytes.get(at..at.checked_add(4)?)?;
    Some(i32::from_le_bytes([word[0], word[1], word[2], word[3]]))
}

/// Whether `needle` occurs in `haystack`.
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

/// The flight data's list: where its first slot is, bytes, and how many slots are filled.
#[derive(Debug, Clone, Copy, PartialEq)]
struct List {
    at: usize,
    size: usize,
}

/// The flight record's `List<Int16>`: an `ArraySinglePrimitive` record of 16-bit integers whose
/// object id the list's `MemberReference` names just before it, the reference followed by the
/// list's size, which may be less than the array's length (the rest is spare capacity). Each
/// candidate is checked in constant work, so the scan is linear in the file's length.
fn find_list(bytes: &[u8]) -> Result<List, LogError> {
    let mut found: Option<(usize, List)> = None;
    for record in 13..bytes.len().saturating_sub(9) {
        if bytes[record] != ARRAY_SINGLE_PRIMITIVE || bytes[record + 9] != PRIMITIVE_INT16 {
            continue;
        }
        let (Some(id), Some(length)) = (i32_at(bytes, record + 1), i32_at(bytes, record + 5))
        else {
            continue;
        };
        if id <= 0 || length <= 0 || length > MAX_SLOTS {
            continue;
        }
        // The list's `_items` reference, then its `_size`, end 13 bytes before the array: a
        // reference record is a type byte and an id, and the size and `_version` follow it.
        let reference = record - 13;
        if bytes[reference] != MEMBER_REFERENCE || i32_at(bytes, reference + 1) != Some(id) {
            continue;
        }
        let Some(size) = i32_at(bytes, reference + 5) else {
            continue;
        };
        if size <= 0 || size > length {
            continue;
        }
        let (Ok(length), Ok(size)) = (usize::try_from(length), usize::try_from(size)) else {
            continue;
        };
        let at = record + 10;
        if at + 2 * length > bytes.len() {
            return Err(binary(
                record,
                format!(
                    "the flight data's list of {length} slots runs past the file's end, at {} \
                     bytes",
                    bytes.len()
                ),
            ));
        }
        if let Some((first, _)) = found {
            return Err(binary(
                record,
                format!(
                    "the stream holds two lists that could be the flight data's, at bytes \
                     {first} and {record}"
                ),
            ));
        }
        found = Some((record, List { at, size }));
    }
    found.map(|(_, list)| list).ok_or_else(|| {
        binary(
            0,
            "the stream holds no list of 16-bit words that its flight record names: the file may \
             be cut short"
                .to_owned(),
        )
    })
}

/// Reads an RRC3 `.rff` file.
pub fn read(bytes: &[u8]) -> Result<FlightLog, LogError> {
    if !recognizes(bytes) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file isn't an MS-NRBF stream holding mDACS's flight record".to_owned(),
        });
    }
    let list = find_list(bytes)?;
    // Every index asked for is below `list.size`, at most the list's length, whose slots
    // `find_list` checked lie inside the file: the indexing can't fail.
    let word = |index: usize| {
        let at = list.at + 2 * index;
        u16::from_le_bytes([bytes[at], bytes[at + 1]])
    };
    let byte_of = |index: usize| list.at + 2 * index;

    let start = (0..list.size.min(MAX_PREAMBLE))
        .find(|&index| (PAD_FLOOR..AUXILIARY).contains(&word(index)))
        .ok_or_else(|| {
            binary(
                list.at,
                format!(
                    "none of the log's first {MAX_PREAMBLE} words is a pressure on a launch pad, \
                     at least 500 mbar. The reader doesn't guess where the readings start"
                ),
            )
        })?;

    let mut pressure = Vec::new();
    let mut auxiliary: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    let mut auxiliary_time = Vec::new();
    let (mut lone, mut lone_time) = (Vec::new(), Vec::new());
    let mut end = list.size;
    let mut index = start;
    while index < list.size {
        let value = word(index);
        if value == 0 {
            end = index;
            break;
        }
        if value < AUXILIARY {
            pressure.push(f64::from(value) * COUNT_PA);
            index += 1;
        } else if index + 1 < list.size && word(index + 1) >= AUXILIARY {
            // The time of the reading before the pair; the readings' clock starts at zero.
            auxiliary_time.push(pressure.len().saturating_sub(1) as f64 / RATE_HZ);
            auxiliary[0].push(f64::from(value));
            auxiliary[1].push(f64::from(word(index + 1)));
            index += 2;
        } else {
            lone_time.push(pressure.len().saturating_sub(1) as f64 / RATE_HZ);
            lone.push(f64::from(value));
            index += 1;
        }
    }
    if let Some(after) = (end..list.size).find(|&index| word(index) != 0) {
        return Err(binary(
            byte_of(after),
            format!(
                "the log ends with a zero word at slot {end}, but slot {after} after it holds \
                 {:#06x}",
                word(after)
            ),
        ));
    }
    if (pressure.len() as f64) < 2.0 * RATE_HZ {
        return Err(LogError::NoData { format: FORMAT });
    }
    let by_readings = pressure.len() as f64 / RATE_HZ;
    let by_pairs = auxiliary_time.len() as f64;
    if (by_pairs - by_readings).abs() > 0.1 * by_readings + 2.0 {
        return Err(binary(
            list.at,
            format!(
                "the log's {} once-a-second pairs and its {} readings at 20 a second, {by_readings} \
                 s, don't agree on how long it ran. The words aren't laid out as this reader takes \
                 them",
                auxiliary_time.len(),
                pressure.len()
            ),
        ));
    }

    let mut notes = vec![
        format!(
            "the file holds no times. The pressure is on the RRC3's 20 Hz clock (RRC3 manual p. \
             23), which its {} once-a-second pairs bear out to within {:.1} s",
            auxiliary_time.len(),
            (by_pairs - by_readings).abs()
        ),
        "the record's primary track is empty: the file holds pressures, not an altitude".to_owned(),
        "the once-a-second pairs are kept as written. They are presumably the temperature and \
         battery voltage the RRC3 records at 1 Hz (RRC3 manual p. 23). The file holds no \
         calibration for them"
            .to_owned(),
    ];
    if start > 0 {
        let preamble: Vec<String> = (0..start).map(|i| format!("{:#06x}", word(i))).collect();
        notes.push(format!(
            "the log opens with {start} words before its first reading, kept here as written: {}",
            preamble.join(", ")
        ));
    }
    if !lone.is_empty() {
        notes.push(format!(
            "{} auxiliary words stand alone, not in a pair; they are kept as written, as `single \
             auxiliary word`",
            lone.len()
        ));
    }
    if end < list.size {
        notes.push(format!(
            "the log ends with a zero word at slot {end} of the list's {}",
            list.size
        ));
    }

    let time_s: Vec<f64> = (0..pressure.len()).map(|n| n as f64 / RATE_HZ).collect();
    let [first, second] = auxiliary;
    Ok(FlightLog {
        format: LogFormat::Rrc3Rff,
        logger: "Missile Works RRC3".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: vec![
            Channel {
                column: "Pressure".to_owned(),
                quantity: Quantity::Pressure,
                time_s,
                values: pressure,
            },
            Channel {
                column: "auxiliary word 1".to_owned(),
                quantity: Quantity::Unrecognized,
                time_s: auxiliary_time.clone(),
                values: first,
            },
            Channel {
                column: "auxiliary word 2".to_owned(),
                quantity: Quantity::Unrecognized,
                time_s: auxiliary_time,
                values: second,
            },
            Channel {
                column: "single auxiliary word".to_owned(),
                quantity: Quantity::Unrecognized,
                time_s: lone_time,
                values: lone,
            },
        ],
        events: Vec::new(),
        notes,
    })
}

/// A refusal at a byte of the file.
fn binary(offset: usize, message: String) -> LogError {
    LogError::Binary {
        format: FORMAT,
        offset,
        message,
    }
}
