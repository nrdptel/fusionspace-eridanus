//! Missile Works' RTx/GPS tracker: the `.rtx` file its base station's software saves when it
//! downloads the tracker's fixes, and the CSV and text files the same software exports from it.
//!
//! The RTx, built around a u-blox 7 receiver, keeps its fixes at 1 Hz, "~68 minutes per flight",
//! and sends them to a base station "at 1-Hz intervals" (*RTx/GPS User Manual*, firmware 1.31,
//! Missile Works, 2016-11-26, pp2, 14); Missile Works' mDACS software saves a download "with the
//! .rtx file extension" and exports it "to Google Earth KML, Excel CSV, or a simple text format"
//! (*mDACS User Manual*, revision 1.71, May 2024, p16), the KML read by [`crate::kml`]. Neither
//! manual describes the files; what follows was learned from the 5 `.rtx` files among the
//! project's sample logs and the 13 exports beside them, with no other program's source read.
//!
//! **The `.rtx` file** is a .NET `BinaryFormatter` stream, the layout Microsoft publishes as
//! MS-NRBF (*[MS-NRBF]: .NET Remoting: Binary Format Data Structure*, revision 13.0, 2024-11-19),
//! of one `List<Int32>`: a serialization header (§2.6.1), the list's class record,
//! `SystemClassWithMembersAndTypes` (§2.3.2.3) with its three members `_items`, `_size` and
//! `_version`, a `MemberReference` (§2.5.3) to the items, the two counts, an
//! `ArraySinglePrimitive` (§2.4.3.3) of 32-bit integers, and `MessageEnd` (§2.6.3). Every sample
//! file holds the same 186 bytes up to `_size`, a list of 65,536 slots, and nothing after the end
//! record. The reader takes that shape and no other: a stream that differs is refused at the first
//! byte that does.
//!
//! **The words.** Each slot holds a 16-bit value, but not as written: the value times 387,420,489
//! (3¹⁸), modulo 10⁹. The reader multiplies by 513,180,409, that multiplier's inverse modulo 10⁹,
//! to read it back. Three things in the files show it: the slot that holds the time of day steps
//! by exactly 387,420,489, modulo 10⁹, from one record to the next a second later; every one of
//! the 264,336 slots in use in the 5 files lies from 0 to 10⁹ − 1 and reads back as a value below
//! 65,536, which a slot drawn at random from that range would do about once in 15,000; and the values read back
//! equal the exports' figures, field by field, on every row (`cargo xtask logs` compares them). A
//! slot outside 0 to 10⁹ − 1, or one that reads back past 65,535, is refused.
//!
//! **The records.** `_size` counts the slots in use, 16 a record, oldest first; the slots after
//! them are 0. The list's 65,536 slots hold 4,096 records, 68.3 minutes at 1 Hz, the manual's
//! flight. Each record is one fix, its 16 words, numbered from 0:
//!
//! | Words | Field | As read |
//! |---|---|---|
//! | 0, 1 | `UTC Time`, `HHMMSS` | word 1 × 65,536 + word 0 |
//! | 2 | `Latitude`'s degrees and whole minutes, `DDMM` | |
//! | 3, 4 | its minutes' five decimals, after a leading 1 | word 4 × 65,536 + word 3 − 100,000 |
//! | 5 | `Longitude`'s degrees and whole minutes, `DDDMM` | |
//! | 6, 7 | its minutes' five decimals, after a leading 1 | word 7 × 65,536 + word 6 − 100,000 |
//! | 8 | `Altitude`, m | |
//! | 9 | `Velocity` | kept as written |
//! | 10 | `VDOP` | kept as written |
//! | 11 | the hemispheres: `N` or `S` in its low byte, `E` or `W` in its high | |
//! | 12 | `NavStatus` in its low byte, `Sats` in its high | |
//! | 13 | `SOG` | kept as written |
//! | 14 | `COG`, degrees | |
//! | 15 | a word the exports leave out | kept as written |
//!
//! The leading 1 keeps a minute's decimals' leading zeros, as `193418` holds `.93418`.
//!
//! **Units and the second checks.** The altitude is in whole meters: the RTx's manual says its GPS
//! altitude "is always expressed as Mean Sea Level" (p13), the KML export beside each sample file
//! writes the word itself, as meters, on every point, and the CSV and text exports write the word ×
//! 3.280 84 ft rounded down on every row. A word can't fall below 0: in 4 of the 5 sample files,
//! from pads near sea level, it reads 0 for most of the fixes, as a receiver's altitude clamped at
//! 0 would, and a note counts those 0s. The survey (`cargo xtask logs`) holds the exports' feet and
//! meters to the words, and reports, without checking, the climb from the pad to the highest fix
//! against the barometric altimeter flown beside the tracker: 0.98 to 1.15 times it in 4 flights.
//! `COG` is a course in degrees, as a GPS receiver's course over ground is: it lies from 0 to 359
//! in the sample files, and in each the median difference either way from the bearing between the
//! fixes a second either side of it, while the tracker moves, is 1.7° to 6.9°. `Velocity` and `SOG`
//! are kept as written: `SOG` runs 3.5 to 4.0 times the speed between fixes in meters a second, the
//! median in each file, as kilometers an hour would, one of the two speed units the RTx's display
//! offers (p12), but no document says which unit the file holds, and feet a second would give 3.3;
//! `Velocity` reads, while the rocket climbs, far more than the climb rate in meters a second, and
//! while it falls, about the fall rate itself, a pattern no document explains. `VDOP`, `NavStatus`
//! and `Sats` are numbers, and word 15, which runs from 814 to 2,272 in the sample files, is kept
//! as written.
//!
//! **The clock** is the GPS's time of day, UTC, in whole seconds; the file states no date. The
//! log's zero is the first record's time; each time takes the day that puts it nearest the one
//! before it, so a time more than 12 hours before the one before is read as the next day's, and
//! one more than 12 hours after it, as a repeated pair written across midnight would be, as the
//! day before's. Records keep the file's order, so a time may go back: in 3 of the 5
//! sample files a pair of records repeats the pair before it a second back, as `t`, `t + 1`, `t`,
//! `t + 1`. A note counts how often times go back. A time that isn't a time of day, such as the
//! record in one sample file whose word 1 reads 65,282 where every other reads 2, as a garbled
//! radio packet's might, is a gap, counted in a note.
//!
//! **Positions.** A latitude's or longitude's minutes past 59, decimals that don't start with the
//! leading 1, a hemisphere byte other than its two letters, or a latitude past 90° or longitude
//! past 180° make that coordinate a gap, counted in a note; so do both, where both read exactly 0,
//! as a receiver writes before it has a fix. No sample file has either.
//!
//! **The exports.** The CSV opens `UTC Time,Latitude,Longitude,Altitude,Velocity,VDOP,NavStatus,
//! Sats,SOG,COG`; the text file, tab-separated, `UTC`, `Lat`, an empty name, `Lon`, an empty name,
//! `Alt`, `Vel`, `VDOP`, `Nav`, `Sats`, `SOG`, `COG`, its rows ten cells, as the CSV's. Both write
//! the time as `HHMMSS`, a position either as `DDMM.mmmmm` and its hemisphere's letter, such as
//! `3259.01234N`, or, in 5 of the 9 sample exports, in signed degrees cut toward zero at the
//! sixth decimal and a degree sign, such as `-106.975720°`, and the altitude in whole feet, read as 0.3048 m each. Each export
//! holds every record of its `.rtx` but the last, in the same order (`cargo xtask logs`). A row of
//! other than ten cells, or a cell that isn't its field's number, is refused, naming the line; a
//! time or a position out of range is a gap, as in the `.rtx`.
//!
//! **The record.** No barometer: the primary track is empty, with a note, and every field is a
//! channel on the clock above, as [`crate::fwgps`] reads a GPS tracker's. The logger is `Missile
//! Works RTx`; the files state no serial number, firmware, flight number or date.
//!
//! **Refused**, the `.rtx` naming the byte: a stream that isn't the list's shape above; a `_size`
//! that isn't a whole number of records or that passes the list's slots; a slot in use that isn't
//! a word, as above; a slot past `_size` that isn't 0; and a file with no record whose time is a
//! time of day.

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{Notes, short};

/// The `.rtx` format's name in errors.
const RAW: &str = "RTx .rtx";

/// The CSV export's name in errors.
const CSV: &str = "RTx CSV";

/// The text export's name in errors.
const TEXT: &str = "RTx text";

/// The logger, as the record names it, and as [`crate::kml`] names the RTx's KML.
const LOGGER: &str = "Missile Works RTx";

/// MS-NRBF's `SerializationHeaderRecord` (§2.6.1): record type 0, `RootId` 1, `HeaderId` −1,
/// version 1.0.
const SERIALIZATION_HEADER: &[u8] = &[
    0x00, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

/// `SystemClassWithMembersAndTypes` (§2.3.2.3, record type 4) for object 1, then its name's
/// length, 126, as a one-byte `LengthPrefixedString` length (§2.1.1.6).
const CLASS_RECORD: &[u8] = &[0x04, 0x01, 0x00, 0x00, 0x00, 0x7e];

/// The list's class, as the stream names it.
const CLASS_NAME: &[u8] = b"System.Collections.Generic.List`1[[System.Int32, mscorlib, \
Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]";

/// The class's three members, their names, their `BinaryTypeEnumeration`s (§2.1.2.2: 7, a
/// primitive array, then two primitives) and their primitive types (§2.1.2.3: 8, `Int32`, each),
/// then a `MemberReference` (§2.5.3, record type 9) to object 2, the items.
const MEMBERS: &[u8] = b"\x03\x00\x00\x00\x06_items\x05_size\x08_version\x07\x00\x00\x08\x08\x08\
\x09\x02\x00\x00\x00";

/// `ArraySinglePrimitive` (§2.4.3.3, record type 15) for object 2.
const ARRAY_RECORD: &[u8] = &[0x0f, 0x02, 0x00, 0x00, 0x00];

/// The array's primitive type, `Int32` (§2.1.2.3).
const INT32: u8 = 8;

/// `MessageEnd` (§2.6.3, record type 11).
const MESSAGE_END: u8 = 0x0b;

/// The words in a record.
const RECORD_WORDS: usize = 16;

/// The modulus of the slots' mapping (module docs).
const MODULUS: u64 = 1_000_000_000;

/// What a word is multiplied by, modulo [`MODULUS`], to make its slot: 3¹⁸.
const MULTIPLIER: u64 = 387_420_489;

/// [`MULTIPLIER`]'s inverse modulo [`MODULUS`]: a slot times it, modulo [`MODULUS`], is its word.
const INVERSE: u64 = 513_180_409;

/// The minutes' decimals carry a leading 1, so a decimals word pair reads from this to twice it,
/// less one.
const DECIMALS_LEAD: u32 = 100_000;

/// A day, s.
const DAY_S: f64 = 86_400.0;

/// A time this far before the one before it is the next day's, and this far after it the day
/// before's, s.
const NEXT_DAY_S: f64 = DAY_S / 2.0;

/// A foot, m (international foot).
const FOOT_M: f64 = 0.3048;

/// The CSV export's header.
const CSV_HEADER: [&str; 10] = [
    "UTC Time",
    "Latitude",
    "Longitude",
    "Altitude",
    "Velocity",
    "VDOP",
    "NavStatus",
    "Sats",
    "SOG",
    "COG",
];

/// The text export's header, with its two empty names.
const TEXT_HEADER: [&str; 12] = [
    "UTC", "Lat", "", "Lon", "", "Alt", "Vel", "VDOP", "Nav", "Sats", "SOG", "COG",
];

/// The `.rtx`'s altitude step, m.
pub const ALTITUDE_STEP_M: f64 = 1.0;

/// The exports' altitude step, m.
pub const EXPORT_ALTITUDE_STEP_M: f64 = FOOT_M;

/// Whether the bytes open as an `.rtx` file: MS-NRBF's serialization header, then the class record
/// of a `List<Int32>` as the RTx's software writes it.
pub fn recognizes_raw(bytes: &[u8]) -> bool {
    bytes
        .strip_prefix(SERIALIZATION_HEADER)
        .and_then(|rest| rest.strip_prefix(CLASS_RECORD))
        .is_some_and(|rest| rest.starts_with(CLASS_NAME))
}

/// Which export a line is the header of, if either.
fn export_header(line: &str) -> Option<LogFormat> {
    if line.split(',').map(str::trim).eq(CSV_HEADER) {
        Some(LogFormat::RtxCsv)
    } else if line.split('\t').map(str::trim).eq(TEXT_HEADER) {
        Some(LogFormat::RtxText)
    } else {
        None
    }
}

/// Which RTx export a file's text is, if either: its first line that isn't blank is the CSV's or
/// the text file's header, the names as written.
pub fn export_format(text: &str) -> Option<LogFormat> {
    text.strip_prefix('\u{feff}')
        .unwrap_or(text)
        .split(['\r', '\n'])
        .find(|line| !line.trim().is_empty())
        .and_then(export_header)
}

/// A slot's word: the slot times [`INVERSE`] modulo [`MODULUS`]; `None` for a slot outside 0 to
/// 10⁹ − 1, or one that reads back past 65,535.
fn word(slot: i32) -> Option<u16> {
    let slot = u64::try_from(slot).ok().filter(|&s| s < MODULUS)?;
    u16::try_from(slot * INVERSE % MODULUS).ok()
}

/// A slot for a word, as the RTx's software writes it: the inverse of [`word`]. For tests.
#[doc(hidden)]
pub fn slot(word: u16) -> i32 {
    #[allow(
        clippy::cast_possible_truncation,
        reason = "a remainder modulo 10⁹ is within an `i32`"
    )]
    {
        (u64::from(word) * MULTIPLIER % MODULUS) as i32
    }
}

/// An `HHMMSS` time as seconds after midnight; `None` if it isn't a time of day. A second of 60
/// is a leap second's.
fn time_of_day(hhmmss: u64) -> Option<f64> {
    let (hours, minutes, seconds) = (hhmmss / 10_000, hhmmss / 100 % 100, hhmmss % 100);
    (hours < 24 && minutes < 60 && seconds <= 60).then(|| {
        #[allow(
            clippy::cast_precision_loss,
            reason = "under 86,401 s, exact in an `f64`"
        )]
        {
            (hours * 3_600 + minutes * 60 + seconds) as f64
        }
    })
}

/// A coordinate from its `DDMM` or `DDDMM` word, its decimals word pair and its hemisphere's byte;
/// `None` if any part is out of range (module docs).
fn coordinate(
    degrees_minutes: u16,
    decimals: u32,
    hemisphere: u8,
    letters: [u8; 2],
    most: f64,
) -> Option<f64> {
    let decimals = decimals.checked_sub(DECIMALS_LEAD)?;
    if decimals >= DECIMALS_LEAD || degrees_minutes % 100 >= 60 {
        return None;
    }
    let minutes = f64::from(degrees_minutes % 100) + f64::from(decimals) / 1e5;
    let value = f64::from(degrees_minutes / 100) + minutes / 60.0;
    let sign = if hemisphere == letters[0] {
        1.0
    } else if hemisphere == letters[1] {
        -1.0
    } else {
        return None;
    };
    (value <= most).then_some(sign * value)
}

/// What a set of fixes' times and positions came to, for the notes.
#[derive(Default)]
struct Counts {
    bad_times: usize,
    bad_time_example: Option<u64>,
    bad_latitudes: usize,
    bad_longitudes: usize,
    no_fix: usize,
    next_days: usize,
    days_before: usize,
}

/// One fix's time and position, before the clock is made.
struct Fix {
    hhmmss: u64,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

/// The clock: each fix's time from the first that is a time of day, `NaN` for one that isn't; and
/// the zero's `HHMMSS`. Each time takes the day that puts it nearest the one before it: a time more
/// than 12 hours before the one before is the next day's, and one more than 12 hours after it the
/// day before's, as a repeated pair of records written across midnight is.
fn clock(fixes: &[Fix], counts: &mut Counts) -> Option<(Vec<f64>, u64)> {
    let mut zero: Option<(f64, u64)> = None;
    let mut day = 0.0;
    let mut last: Option<f64> = None;
    let mut time_s = Vec::with_capacity(fixes.len());
    for fix in fixes {
        let Some(of_day) = time_of_day(fix.hhmmss) else {
            counts.bad_times += 1;
            counts.bad_time_example.get_or_insert(fix.hhmmss);
            time_s.push(f64::NAN);
            continue;
        };
        if last.is_some_and(|last| of_day + day < last - NEXT_DAY_S) {
            day += DAY_S;
            counts.next_days += 1;
        } else if last.is_some_and(|last| of_day + day > last + NEXT_DAY_S) {
            day -= DAY_S;
            counts.days_before += 1;
        }
        let at = of_day + day;
        last = Some(at);
        let (zero_s, _) = *zero.get_or_insert((at, fix.hhmmss));
        time_s.push(at - zero_s);
    }
    zero.map(|(_, hhmmss)| (time_s, hhmmss))
}

/// The notes on the clock, the times and the positions.
fn clock_notes(time_s: &[f64], zero: u64, counts: &Counts, quoted: &str) -> Vec<String> {
    let mut notes = vec![format!(
        "the log's clock is the GPS's time of day, UTC, `{quoted}`, in whole seconds; its zero is \
         the first record's, {:02}:{:02}:{:02}; the file states no date",
        zero / 10_000,
        zero / 100 % 100,
        zero % 100
    )];
    let known: Vec<f64> = time_s.iter().copied().filter(|t| !t.is_nan()).collect();
    let backs = known.windows(2).filter(|pair| pair[1] < pair[0]).count();
    let largest = known
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .fold(0.0, f64::max);
    let mut sorted = known.clone();
    sorted.sort_by(f64::total_cmp);
    let repeats = sorted.windows(2).filter(|pair| pair[0] == pair[1]).count();
    if backs > 0 {
        notes.push(format!(
            "times go back {backs} times from one record to the next, by up to {largest:.0} s, \
             and {repeats} records repeat a time already written: every record is kept, in the \
             file's order"
        ));
    }
    if counts.next_days > 0 {
        notes.push(format!(
            "times fall more than 12 hours behind the one before {} times, each read as the next \
             day's",
            counts.next_days
        ));
    }
    if counts.days_before > 0 {
        notes.push(format!(
            "times run more than 12 hours ahead of the one before {} times, each read as the day \
             before's",
            counts.days_before
        ));
    }
    if counts.bad_times > 0 {
        notes.push(format!(
            "{} records hold a time that isn't a time of day, such as {}: their times are gaps",
            counts.bad_times,
            counts.bad_time_example.unwrap_or_default()
        ));
    }
    for (count, what) in [
        (counts.bad_latitudes, "latitudes"),
        (counts.bad_longitudes, "longitudes"),
    ] {
        if count > 0 {
            notes.push(format!(
                "{count} {what} are out of range (minutes past 59, decimals without their \
                 leading 1, a hemisphere byte other than its two letters, or beyond ±90° or \
                 ±180°): they are gaps"
            ));
        }
    }
    if counts.no_fix > 0 {
        notes.push(format!(
            "{} records have the latitude and longitude both 0, as a receiver writes before it \
             has a fix: their positions are gaps",
            counts.no_fix
        ));
    }
    notes
}

/// A position read: a gap for a coordinate out of range, both for (0, 0).
fn position(
    latitude: Option<f64>,
    longitude: Option<f64>,
    counts: &mut Counts,
) -> (Option<f64>, Option<f64>) {
    counts.bad_latitudes += usize::from(latitude.is_none());
    counts.bad_longitudes += usize::from(longitude.is_none());
    if latitude == Some(0.0) && longitude == Some(0.0) {
        counts.no_fix += 1;
        (None, None)
    } else {
        (latitude, longitude)
    }
}

/// The note that the log has no barometer.
const NO_BAROMETER: &str = "the log has no barometer, so the record's altitude track is empty: \
                            an RTx's file holds GPS fixes only, and every field is a channel";

/// A channel on the clock.
fn channel(column: &str, quantity: Quantity, time_s: &[f64], values: Vec<f64>) -> Channel {
    Channel {
        column: column.to_owned(),
        quantity,
        time_s: time_s.to_vec(),
        values,
    }
}

/// An error naming a byte of an `.rtx` file.
fn fault(offset: usize, message: impl Into<String>) -> LogError {
    LogError::Binary {
        format: RAW,
        offset,
        message: message.into(),
    }
}

/// A little-endian 32-bit integer at a byte; the caller has checked the length.
fn int_at(bytes: &[u8], at: usize) -> i32 {
    let mut four = [0; 4];
    four.copy_from_slice(&bytes[at..at + 4]);
    i32::from_le_bytes(four)
}

/// The bytes a stream must hold from a byte on, or the offset of the first that differs.
fn expect(bytes: &[u8], at: usize, wanted: &[u8], what: &str) -> Result<usize, LogError> {
    match wanted
        .iter()
        .enumerate()
        .find(|&(k, b)| bytes.get(at + k) != Some(b))
    {
        Some((k, _)) => Err(fault(
            at + k,
            format!("the stream isn't the RTx's list here: {what}"),
        )),
        None => Ok(at + wanted.len()),
    }
}

/// Reads an RTx `.rtx` file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the bytes don't open as an `.rtx`'s; [`LogError::Binary`] if
/// the stream isn't laid out as the module docs say, or a slot isn't a word; [`LogError::NoData`]
/// if the list holds no record, or no record whose time is a time of day.
pub fn read_raw(bytes: &[u8]) -> Result<FlightLog, LogError> {
    if !recognizes_raw(bytes) {
        return Err(LogError::NotThisFormat {
            format: RAW,
            message: "the file doesn't open as an MS-NRBF stream of the RTx's `List<Int32>`"
                .to_owned(),
        });
    }
    let mut at = expect(bytes, 0, SERIALIZATION_HEADER, "the serialization header")?;
    at = expect(bytes, at, CLASS_RECORD, "the list's class record")?;
    at = expect(bytes, at, CLASS_NAME, "the list's class name")?;
    at = expect(bytes, at, MEMBERS, "the list's members")?;
    if bytes.len() < at + 8 {
        return Err(fault(bytes.len(), "the file ends inside the list's counts"));
    }
    let size_at = at;
    let size = int_at(bytes, at);
    at = expect(bytes, at + 8, ARRAY_RECORD, "the items' array record")?;
    if bytes.len() < at + 5 {
        return Err(fault(
            bytes.len(),
            "the file ends inside the items' array record",
        ));
    }
    let length_at = at;
    let length = usize::try_from(int_at(bytes, at))
        .map_err(|_| fault(length_at, "the items' array has a negative length"))?;
    at = expect(bytes, at + 4, &[INT32], "the items' type, `Int32`")?;
    let data_at = at;
    let end = length
        .checked_mul(4)
        .and_then(|n| n.checked_add(data_at))
        .filter(|&end| end < bytes.len())
        .ok_or_else(|| {
            fault(
                length_at,
                format!("the items' {length} slots run past the file's end"),
            )
        })?;
    if bytes[end] != MESSAGE_END || bytes.len() != end + 1 {
        return Err(fault(
            end,
            "the stream doesn't end with `MessageEnd` straight after the items",
        ));
    }
    let size = usize::try_from(size)
        .ok()
        .filter(|&s| s % RECORD_WORDS == 0 && s <= length)
        .ok_or_else(|| {
            fault(
                size_at,
                format!(
                    "the list's `_size`, {size}, isn't a whole number of {RECORD_WORDS}-word \
                     records within its {length} slots"
                ),
            )
        })?;
    let slot_at = |k: usize| data_at + 4 * k;
    if let Some(k) = (size..length).find(|&k| int_at(bytes, slot_at(k)) != 0) {
        return Err(fault(
            slot_at(k),
            format!("slot {k}, past the list's `_size` of {size}, isn't 0"),
        ));
    }
    let mut records = Vec::with_capacity(size / RECORD_WORDS);
    for r in 0..size / RECORD_WORDS {
        let mut record = [0u16; RECORD_WORDS];
        for (w, value) in record.iter_mut().enumerate() {
            let k = r * RECORD_WORDS + w;
            let raw = int_at(bytes, slot_at(k));
            *value = word(raw).ok_or_else(|| {
                fault(
                    slot_at(k),
                    format!(
                        "slot {k}, {raw}, isn't a 16-bit word times {MULTIPLIER} modulo \
                         {MODULUS}"
                    ),
                )
            })?;
        }
        records.push(record);
    }
    if records.is_empty() {
        return Err(LogError::NoData { format: RAW });
    }

    let mut counts = Counts::default();
    let fixes: Vec<Fix> = records
        .iter()
        .map(|w| {
            let [north, east] = w[11].to_le_bytes();
            let pair = |lo: u16, hi: u16| u32::from(hi) << 16 | u32::from(lo);
            Fix {
                hhmmss: u64::from(pair(w[0], w[1])),
                latitude: coordinate(w[2], pair(w[3], w[4]), north, *b"NS", 90.0),
                longitude: coordinate(w[5], pair(w[6], w[7]), east, *b"EW", 180.0),
            }
        })
        .collect();
    let (time_s, zero) = clock(&fixes, &mut counts).ok_or(LogError::NoData { format: RAW })?;
    let (latitudes, longitudes): (Vec<f64>, Vec<f64>) = fixes
        .iter()
        .map(|fix| {
            let (lat, lon) = position(fix.latitude, fix.longitude, &mut counts);
            (lat.unwrap_or(f64::NAN), lon.unwrap_or(f64::NAN))
        })
        .unzip();
    let field = |w: usize| -> Vec<f64> { records.iter().map(|r| f64::from(r[w])).collect() };
    let byte = |w: usize, high: bool| -> Vec<f64> {
        records
            .iter()
            .map(|r| f64::from(if high { r[w] >> 8 } else { r[w] & 0xff }))
            .collect()
    };
    let channels = vec![
        channel("Latitude", Quantity::Latitude, &time_s, latitudes),
        channel("Longitude", Quantity::Longitude, &time_s, longitudes),
        channel("Altitude", Quantity::GnssAltitude, &time_s, field(8)),
        channel("Velocity", Quantity::Unrecognized, &time_s, field(9)),
        channel("VDOP", Quantity::Number, &time_s, field(10)),
        channel("NavStatus", Quantity::Number, &time_s, byte(12, false)),
        channel("Sats", Quantity::Number, &time_s, byte(12, true)),
        channel("SOG", Quantity::Unrecognized, &time_s, field(13)),
        channel("COG", Quantity::Angle, &time_s, field(14)),
        channel("word 15", Quantity::Unrecognized, &time_s, field(15)),
    ];
    let mut notes = vec![
        NO_BAROMETER.to_owned(),
        format!(
            "each slot is its 16-bit word times {MULTIPLIER} modulo {MODULUS}, read back by \
             multiplying by {INVERSE}; the list holds {} records of {RECORD_WORDS} words",
            records.len()
        ),
    ];
    notes.extend(clock_notes(&time_s, zero, &counts, "UTC Time"));
    let at_zero = records.iter().filter(|r| r[8] == 0).count();
    notes.extend(units_notes("Altitude", "Velocity", true, at_zero));
    notes.push(
        "`word 15` kept as written, a word the exports leave out: no document names it".to_owned(),
    );
    Ok(log(LogFormat::RtxRaw, records.len(), channels, notes))
}

/// The notes on the fields' units, by the file's column names: the altitude's, the velocity's
/// and the course's; `raw` for the `.rtx`, whose altitude is in meters, not an export's feet; and
/// the count of altitudes at 0, the least a word holds, each perhaps a lower one clamped.
fn units_notes(altitude: &str, velocity: &str, raw: bool, at_zero: usize) -> Vec<String> {
    let mut notes = vec![
        if raw {
            format!(
                "`{altitude}` read in meters, from the manual and the exports: whole meters, above \
                 mean sea level as the RTx's manual states its GPS altitude, as the KML export \
                 writes them and the CSV and text exports convert them to feet; a word can't fall \
                 below 0"
            )
        } else {
            format!(
                "`{altitude}` read in feet, from the `.rtx` files beside the sample exports: whole \
                 feet, the `.rtx`'s meters × 3.280 84 rounded down, read as 0.3048 m each"
            )
        },
        format!(
            "`{velocity}` and `SOG` kept as written, in units no document states: `SOG` runs \
             about 3.6 times the speed between fixes, as kilometers an hour would, and the \
             velocity far more than the climb rate while it climbs"
        ),
        "`COG` read in degrees, from the positions: the course over ground, within a few \
         degrees of the bearing between the fixes either side while the tracker moves"
            .to_owned(),
    ];
    if at_zero > 0 {
        notes.push(format!(
            "{at_zero} `{altitude}` values read exactly 0, the least the file can hold: each \
             stands for 0 m or lower, as an altitude clamped at 0 would"
        ));
    }
    notes
}

/// The record every RTx file reads into.
fn log(format: LogFormat, samples: usize, channels: Vec<Channel>, notes: Vec<String>) -> FlightLog {
    FlightLog {
        format,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            samples: Some(samples),
            ..Stated::default()
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    }
}

/// An export's coordinate cell: `DDMM.mmmmm` and its hemisphere's letter, or signed degrees and a
/// degree sign. `Err` for a cell that is neither; `Ok(None)` for one out of range.
fn coordinate_cell(cell: &str, letters: [u8; 2], most: f64) -> Result<Option<f64>, ()> {
    if let Some(degrees) = cell.strip_suffix('°') {
        let value: f64 = degrees.trim().parse().map_err(|_| ())?;
        if !value.is_finite() {
            return Err(());
        }
        return Ok((value.abs() <= most).then_some(value));
    }
    let hemisphere = *cell.as_bytes().last().ok_or(())?;
    if !letters.contains(&hemisphere) {
        return Err(());
    }
    let number = &cell[..cell.len() - 1];
    let (whole, decimals) = number.split_once('.').ok_or(())?;
    if whole.is_empty()
        || decimals.len() != 5
        || !whole
            .bytes()
            .chain(decimals.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err(());
    }
    // All digits, so the only failure is a number past a word's 65,535: out of range, a gap.
    let Ok(degrees_minutes) = whole.parse::<u16>() else {
        return Ok(None);
    };
    let decimals: u32 = decimals.parse().map_err(|_| ())?;
    Ok(coordinate(
        degrees_minutes,
        decimals + DECIMALS_LEAD,
        hemisphere,
        letters,
        most,
    ))
}

/// Reads an RTx CSV or text export.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't either export's header;
/// [`LogError::Syntax`] for a row of other than ten cells or a cell that isn't its field's number;
/// [`LogError::NoData`] if no row holds a time of day.
pub fn read_export(text: &str) -> Result<FlightLog, LogError> {
    let mut lines =
        crate::table::numbered_lines(text).skip_while(|(_, line)| line.trim().is_empty());
    let Some(format) = lines.next().and_then(|(_, line)| export_header(line)) else {
        return Err(LogError::NotThisFormat {
            format: CSV,
            message: "the first line that isn't blank isn't the RTx CSV's or text file's header"
                .to_owned(),
        });
    };
    let (name, separator, names) = match format {
        LogFormat::RtxCsv => (CSV, ',', CSV_HEADER),
        _ => (
            TEXT,
            '\t',
            [
                "UTC", "Lat", "Lon", "Alt", "Vel", "VDOP", "Nav", "Sats", "SOG", "COG",
            ],
        ),
    };
    let mut fixes = Vec::new();
    let mut rows: Vec<[f64; 7]> = Vec::new();
    for (line_number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let syntax = |message: String| LogError::Syntax {
            format: name,
            line: line_number,
            message,
        };
        let cells: Vec<&str> = line.split(separator).map(str::trim).collect();
        let [time, latitude, longitude, rest @ ..] = &cells[..] else {
            return Err(syntax(format!("{} cells, not 10", cells.len())));
        };
        if cells.len() != 10 {
            return Err(syntax(format!("{} cells, not 10", cells.len())));
        }
        let not_a =
            |k: usize, cell: &str| syntax(format!("`{}` {} isn't one", names[k], short(cell)));
        if time.is_empty() || time.len() > 10 || !time.bytes().all(|b| b.is_ascii_digit()) {
            return Err(not_a(0, time));
        }
        let hhmmss: u64 = time.parse().map_err(|_| not_a(0, time))?;
        let latitude = coordinate_cell(latitude, *b"NS", 90.0).map_err(|()| not_a(1, latitude))?;
        let longitude =
            coordinate_cell(longitude, *b"EW", 180.0).map_err(|()| not_a(2, longitude))?;
        let mut values = [0.0; 7];
        for (k, (value, cell)) in values.iter_mut().zip(rest).enumerate() {
            *value = crate::table::number(cell, false).ok_or_else(|| not_a(k + 3, cell))?;
        }
        fixes.push(Fix {
            hhmmss,
            latitude,
            longitude,
        });
        rows.push(values);
    }
    if fixes.is_empty() {
        return Err(LogError::NoData { format: name });
    }
    let mut counts = Counts::default();
    let (time_s, zero) = clock(&fixes, &mut counts).ok_or(LogError::NoData { format: name })?;
    let (latitudes, longitudes): (Vec<f64>, Vec<f64>) = fixes
        .iter()
        .map(|fix| {
            let (lat, lon) = position(fix.latitude, fix.longitude, &mut counts);
            (lat.unwrap_or(f64::NAN), lon.unwrap_or(f64::NAN))
        })
        .unzip();
    let column = |k: usize, scale: f64| -> Vec<f64> { rows.iter().map(|r| r[k] * scale).collect() };
    let channels = vec![
        channel(names[1], Quantity::Latitude, &time_s, latitudes),
        channel(names[2], Quantity::Longitude, &time_s, longitudes),
        channel(names[3], Quantity::GnssAltitude, &time_s, column(0, FOOT_M)),
        channel(names[4], Quantity::Unrecognized, &time_s, column(1, 1.0)),
        channel(names[5], Quantity::Number, &time_s, column(2, 1.0)),
        channel(names[6], Quantity::Number, &time_s, column(3, 1.0)),
        channel(names[7], Quantity::Number, &time_s, column(4, 1.0)),
        channel(names[8], Quantity::Unrecognized, &time_s, column(5, 1.0)),
        channel(names[9], Quantity::Angle, &time_s, column(6, 1.0)),
    ];
    let mut notes = Notes::default();
    notes.push(NO_BAROMETER.to_owned());
    for note in clock_notes(&time_s, zero, &counts, names[0]) {
        notes.push(note);
    }
    let at_zero = rows.iter().filter(|r| r[0] == 0.0).count();
    for note in units_notes(names[3], names[4], false, at_zero) {
        notes.push(note);
    }
    notes.push(
        "the software writes every record of its `.rtx` file but the last: the `.rtx` holds one \
         more fix"
            .to_owned(),
    );
    Ok(log(format, fixes.len(), channels, notes.into_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_inverse_undoes_the_multiplier() {
        assert_eq!(MULTIPLIER * INVERSE % MODULUS, 1);
        assert_eq!(MULTIPLIER, 3u64.pow(18));
        for value in [0u16, 1, 2, 255, 22_350, 65_535] {
            assert_eq!(word(slot(value)), Some(value));
        }
        // A slot one off a word's reads back far past 65,535.
        assert_eq!(word(slot(2) + 1), None);
        assert_eq!(word(-1), None);
        assert_eq!(word(1_000_000_000), None);
    }

    #[test]
    fn times_of_day_are_read_strictly() {
        assert_eq!(time_of_day(170_903), Some(61_743.0));
        assert_eq!(time_of_day(0), Some(0.0));
        assert_eq!(time_of_day(235_960), Some(86_400.0));
        assert_eq!(time_of_day(240_000), None);
        assert_eq!(time_of_day(126_000), None);
        assert_eq!(time_of_day(120_061), None);
        assert_eq!(time_of_day(4_278_331_365), None);
    }

    #[test]
    fn a_coordinate_takes_its_leading_one_and_hemisphere() {
        let north = coordinate(2755, 193_418, b'N', *b"NS", 90.0).unwrap_or(f64::NAN);
        assert!((north - (27.0 + 55.934_18 / 60.0)).abs() < 1e-12);
        let west = coordinate(8042, 100_589, b'W', *b"EW", 180.0).unwrap_or(f64::NAN);
        assert!((west + (80.0 + 42.005_89 / 60.0)).abs() < 1e-12);
        assert_eq!(coordinate(2760, 100_000, b'N', *b"NS", 90.0), None);
        assert_eq!(coordinate(2755, 99_999, b'N', *b"NS", 90.0), None);
        assert_eq!(coordinate(2755, 200_000, b'N', *b"NS", 90.0), None);
        assert_eq!(coordinate(2755, 100_000, b'X', *b"NS", 90.0), None);
        assert_eq!(coordinate(9100, 100_000, b'N', *b"NS", 90.0), None);
    }
}
