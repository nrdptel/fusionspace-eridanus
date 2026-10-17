//! Rocketronics' AltiMAX altimeters: a flight's `.ALS` file, the samples as the altimeter stores
//! them, before the PC software works heights and a speed out of them.
//!
//! An AltiMAX with a memory card writes each flight to it as an `.als` file, which the maker's PC
//! software reads directly (*AltiMAX G2 Handbuch*, Louis Schreyer, 2010, p. 12). No manual
//! describes the bytes, so the layout here was worked out from the one sample file, decoded
//! against the software's Excel export of the same flight ([`altimax`](crate::altimax)): the file
//! holds one record for each of the export's rows, and every value it stores is the export's.
//!
//! **Layout.** Little-endian throughout: a header of 32 bytes, then one record of 22 bytes a
//! sample, then a trailer.
//!
//! | Bytes | Field | Read as |
//! |---|---|---|
//! | header 4–7 | u32 | the zero pressure, Pa: every height in the export is the standard atmosphere's height of a pressure less that of this one |
//! | header 8–11 | u32 | the apogee's pressure, Pa: the least filtered pressure, to the pascal |
//! | header 26–27 | u16 | the index of the last record: the records are one more |
//! | record 0 | u8 | the record's kind: bit 0 always set, bit 1 never; bit 2 set when the record was stored 25 ms after the one before, clear when 5 ms; bits 3–7 the voltage channel the record's count is, 1 to 5 in turn |
//! | record 1 | u8 | the event's code, 0 for none |
//! | record 2–5 | f32 | the filtered pressure, Pa |
//! | record 6–9 | u32 | the raw pressure, Pa |
//! | record 10–11 | i16 | the acceleration, in counts of 0.01 g along an axis down the rocket |
//! | record 12–13 | u16 | the voltage channel's count |
//! | record 14–21 | 8 bytes | zero in every sample record |
//!
//! The header's other bytes, and the trailer, aren't read: in the sample, the trailer is 2,616
//! bytes, nearly all zero.
//!
//! **The clock** isn't stored: each record is 5 ms or 25 ms after the one before, as its kind's
//! bit 2 says, and the first is at 5 ms, as in the export. The maker's table gives storage rates of
//! 200 and 40 samples a second (*AltiMAX G4 Manual*, Rocketronics, 2021, p. 10); its text says the
//! rate drops to 50 a second 500 ms after the peak (p. 8), but the sample's slower records are 25
//! ms apart, the table's 40 a second. The clock is the running sum of the steps, as the export's
//! `TIME`.
//!
//! **Channels**, each on that clock, named as the export names the column each becomes:
//!
//! | Channel | Quantity | Unit |
//! |---|---|---|
//! | `PRESS FILTER`, `PRESS RAW` | [`Quantity::Pressure`] | Pa, as stored; the export's header says hPa, but its values are these pascals |
//! | `ACCEL` | [`Quantity::Acceleration`] | m/s²: each count 0.0981 m/s², 0.01 g with g taken as 9.81 m/s², the export's own factor; the axis points down the rocket, so the pad reads about −1 g |
//! | `EVENT` | [`Quantity::Number`] | the event's code, as written |
//! | `VOLT. CHANNEL` | [`Quantity::Number`] | the record's voltage channel, 1 to 5 |
//! | `VOLTAGE count (channel N)` | [`Quantity::Unrecognized`], as written | each channel's count, on the records that hold it |
//!
//! The counts are kept as written. The export turns them into volts by one factor for channels 1
//! and 5 and another for 2 to 4, but no document states either, and the software lists five
//! voltages without saying which channel is which (*AltiMAX G4 Manual*, p. 42).
//!
//! **Events.** A record whose event code isn't 0 is an event, named as the export names its code:
//!
//! | Code | Name | Code | Name |
//! |---:|---|---:|---|
//! | 3 | `BARO-LIFTOFF` | 14 | `LAND` |
//! | 5 | `MACHDELAY` | 20, 21 | `PYRO1 ON`, `PYRO1 OFF` |
//! | 6, 7, 8 | `TIMER1 END`, `TIMER2 END`, `TIMER3 END` | 22, 23 | `PYRO2 ON`, `PYRO2 OFF` |
//! | 9 | `APOGEE PRESSURE` | 28, 29, 30 | `TIMER1 START`, `TIMER2 START`, `TIMER3 START` |
//! | 10, 11 | `HEIGHT ASCEND`, `HEIGHT DESCEND` | | |
//!
//! Any other code, which the sample doesn't show, is named `event N`.
//!
//! **Stated.** The record count, and the zero and apogee pressures as figures, `zero pressure`
//! and `apogee pressure`.
//!
//! **No primary track.** The file holds no height and no clock, so [`FlightLog::time_s`] and
//! [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`, and every value is a
//! channel. The export's heights are the software's: the standard atmosphere's height of each
//! pressure, less that of the zero pressure.
//!
//! **Recognition.** [`recognizes`] takes a file whose header states at least one record and
//! whose first records, up to 1,000, are each a kind with bit 0 set and bit 1 clear, a voltage
//! channel one past the record before's (1 after 5), and eight zero bytes at their end.
//!
//! **Refused**, naming the byte: a file shorter than its header and one record; a record whose
//! kind or zero bytes aren't as above; a raw pressure on the first record that isn't a pad's,
//! 50,000 to 110,000 Pa; a filtered pressure that isn't finite; and records running on past the
//! header's count, as they would in a flight of more than 65,536 records, whose count 16 bits
//! can't hold. A file that ends before its header's count is read to its last whole record, with
//! a note.
//!
//! ```
//! use hpr_flightdata::altimax_als;
//!
//! // An invented file: two records, the second an event, then no trailer.
//! let mut bytes = vec![0_u8; 32];
//! bytes[4..8].copy_from_slice(&98_000_u32.to_le_bytes());
//! bytes[26..28].copy_from_slice(&1_u16.to_le_bytes());
//! for (channel, event, pressure) in [(1_u8, 0_u8, 98_000.0_f32), (2, 3, 97_990.0)] {
//!     bytes.push(channel << 3 | 1);
//!     bytes.push(event);
//!     bytes.extend(pressure.to_le_bytes());
//!     bytes.extend((pressure as u32).to_le_bytes());
//!     bytes.extend((-100_i16).to_le_bytes());
//!     bytes.extend(512_u16.to_le_bytes());
//!     bytes.extend([0; 8]);
//! }
//! let log = altimax_als::read(&bytes)?;
//! assert_eq!(log.events[0].name, "BARO-LIFTOFF");
//! assert_eq!(log.events[0].time_s, 0.01);
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use crate::error::LogError;
use crate::log::{Channel, Event, Figure, FlightLog, LogFormat, Quantity, Stated};

const FORMAT: &str = "AltiMAX .ALS";

/// The header's length, bytes.
const HEADER_BYTES: usize = 32;

/// A record's length, bytes.
const RECORD_BYTES: usize = 22;

/// Where the header holds the zero pressure, the apogee's pressure and the last record's index.
const ZERO_PRESSURE_AT: usize = 4;
const APOGEE_PRESSURE_AT: usize = 8;
const LAST_INDEX_AT: usize = 26;

/// A record's step from the one before, s: 200 samples a second, or 40 where the kind's bit 2 is
/// set (*AltiMAX G4 Manual*, p. 10).
const FAST_STEP_S: f64 = 0.005;
const SLOW_STEP_S: f64 = 0.025;

/// The kind's bits: bit 0 always set, bit 1 never, bit 2 the slower step.
const KIND_SET: u8 = 0b001;
const KIND_CLEAR: u8 = 0b010;
const KIND_SLOW: u8 = 0b100;

/// The voltage channels, 1 to 5 in turn.
const VOLT_CHANNELS: u8 = 5;

/// The export's factor for an acceleration count, m/s²: 0.01 g with g taken as 9.81 m/s².
const ACCEL_M_S2_PER_COUNT: f64 = 0.0981;

/// A pad's pressure, Pa: 500 to 1,100 hPa, the band the other readers hold a pressure to.
const PAD_PA: std::ops::RangeInclusive<f64> = 50_000.0..=110_000.0;

/// The records [`recognizes`] reads.
const RECOGNIZED_RECORDS: usize = 1_000;

/// Each event code the sample export names, and its name there.
const EVENT_NAMES: [(u8, &str); 16] = [
    (3, "BARO-LIFTOFF"),
    (5, "MACHDELAY"),
    (6, "TIMER1 END"),
    (7, "TIMER2 END"),
    (8, "TIMER3 END"),
    (9, "APOGEE PRESSURE"),
    (10, "HEIGHT ASCEND"),
    (11, "HEIGHT DESCEND"),
    (14, "LAND"),
    (20, "PYRO1 ON"),
    (21, "PYRO1 OFF"),
    (22, "PYRO2 ON"),
    (23, "PYRO2 OFF"),
    (28, "TIMER1 START"),
    (29, "TIMER2 START"),
    (30, "TIMER3 START"),
];

/// One record, as stored.
#[derive(Clone, Copy)]
struct Record {
    kind: u8,
    event: u8,
    filtered_pa: f32,
    raw_pa: u32,
    accel_counts: i16,
    volt_count: u16,
}

/// The record at `index`, if the file holds all its bytes.
fn record(bytes: &[u8], index: usize) -> Option<(Record, &[u8])> {
    let at = HEADER_BYTES.checked_add(index.checked_mul(RECORD_BYTES)?)?;
    let data: &[u8; RECORD_BYTES] = bytes
        .get(at..at.checked_add(RECORD_BYTES)?)?
        .try_into()
        .ok()?;
    let record = Record {
        kind: data[0],
        event: data[1],
        filtered_pa: f32::from_le_bytes([data[2], data[3], data[4], data[5]]),
        raw_pa: u32::from_le_bytes([data[6], data[7], data[8], data[9]]),
        accel_counts: i16::from_le_bytes([data[10], data[11]]),
        volt_count: u16::from_le_bytes([data[12], data[13]]),
    };
    Some((record, &data[14..]))
}

/// The header's u32 at `at`.
fn header_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// The records the header states: the last record's index, plus one.
fn stated_records(bytes: &[u8]) -> Option<usize> {
    let last = u16::from_le_bytes(
        bytes
            .get(LAST_INDEX_AT..LAST_INDEX_AT + 2)?
            .try_into()
            .ok()?,
    );
    Some(usize::from(last) + 1)
}

/// Why the record at `index` isn't one, given the voltage channel of the record before, if any.
fn fault(record: &Record, zeros: &[u8], before: Option<u8>) -> Option<String> {
    let channel = record.kind >> 3;
    if record.kind & KIND_SET == 0 || record.kind & KIND_CLEAR != 0 {
        return Some(format!(
            "its kind, {:#04x}, has bit 0 clear or bit 1 set, which no sample record has",
            record.kind
        ));
    }
    if !(1..=VOLT_CHANNELS).contains(&channel) {
        return Some(format!(
            "its kind, {:#04x}, names voltage channel {channel}, where the channels are 1 to \
             {VOLT_CHANNELS}",
            record.kind
        ));
    }
    if let Some(before) = before {
        let next = before % VOLT_CHANNELS + 1;
        if channel != next {
            return Some(format!(
                "its voltage channel is {channel}, where the channels take turns and {next} \
                 follows {before}"
            ));
        }
    }
    if zeros.iter().any(|&byte| byte != 0) {
        return Some(
            "its last 8 bytes aren't zero, as they are in every sample record: the reader \
             doesn't know what they hold"
                .to_owned(),
        );
    }
    None
}

/// Whether `bytes` are an AltiMAX `.ALS` file: a header stating at least one record, and the
/// first records, up to 1,000, each a record ([`fault`] finds nothing).
pub fn recognizes(bytes: &[u8]) -> bool {
    let Some(stated) = stated_records(bytes) else {
        return false;
    };
    let mut before = None;
    for index in 0..stated.min(RECOGNIZED_RECORDS) {
        let Some((record, zeros)) = record(bytes, index) else {
            return index > 0;
        };
        if fault(&record, zeros, before).is_some() {
            return false;
        }
        before = Some(record.kind >> 3);
    }
    true
}

/// The byte where record `index` starts.
fn offset(index: usize) -> usize {
    HEADER_BYTES + index * RECORD_BYTES
}

fn binary(offset: usize, message: String) -> LogError {
    LogError::Binary {
        format: FORMAT,
        offset,
        message,
    }
}

/// Reads an AltiMAX `.ALS` file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when the file is shorter than a header and one record;
/// [`LogError::Binary`], naming the byte, when a record's kind or zero bytes aren't as the
/// module describes, the first raw pressure isn't a pad's, a filtered pressure isn't finite, or a
/// record follows the header's count.
pub fn read(bytes: &[u8]) -> Result<FlightLog, LogError> {
    let Some(stated) = stated_records(bytes).filter(|_| bytes.len() >= offset(1)) else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "{} bytes, shorter than a header of {HEADER_BYTES} and a record of {RECORD_BYTES}",
                bytes.len()
            ),
        });
    };
    let mut records = Vec::new();
    let mut before = None;
    for index in 0..stated {
        let Some((record, zeros)) = record(bytes, index) else {
            break;
        };
        if let Some(message) = fault(&record, zeros, before) {
            return Err(binary(
                offset(index),
                format!("record {}: {message}", index + 1),
            ));
        }
        if !record.filtered_pa.is_finite() {
            return Err(binary(
                offset(index) + 2,
                format!("record {}: its filtered pressure isn't a number", index + 1),
            ));
        }
        if index == 0 && !PAD_PA.contains(&f64::from(record.raw_pa)) {
            return Err(binary(
                offset(index) + 6,
                format!(
                    "the first record's raw pressure, {} Pa, isn't a pad's, 50,000 to 110,000 Pa",
                    record.raw_pa
                ),
            ));
        }
        before = Some(record.kind >> 3);
        records.push(record);
    }
    // A record past the header's count: a flight longer than 16 bits can count.
    if let Some((next, zeros)) = record(bytes, records.len())
        && records.len() == stated
        && fault(&next, zeros, before).is_none()
    {
        return Err(binary(
            offset(stated),
            format!(
                "a record follows the {stated} the header states: a flight of more records than \
                 its 16-bit count holds, which no sample shows"
            ),
        ));
    }

    let mut time_s = Vec::with_capacity(records.len());
    let mut clock = 0.0;
    for record in &records {
        clock += if record.kind & KIND_SLOW == 0 {
            FAST_STEP_S
        } else {
            SLOW_STEP_S
        };
        time_s.push(clock);
    }
    let column = |values: Vec<f64>, name: &str, quantity| Channel {
        column: name.to_owned(),
        quantity,
        time_s: time_s.clone(),
        values,
    };
    let mut channels = vec![
        column(
            records.iter().map(|r| f64::from(r.filtered_pa)).collect(),
            "PRESS FILTER",
            Quantity::Pressure,
        ),
        column(
            records.iter().map(|r| f64::from(r.raw_pa)).collect(),
            "PRESS RAW",
            Quantity::Pressure,
        ),
        column(
            records
                .iter()
                .map(|r| f64::from(r.accel_counts) * ACCEL_M_S2_PER_COUNT)
                .collect(),
            "ACCEL",
            Quantity::Acceleration,
        ),
        column(
            records.iter().map(|r| f64::from(r.event)).collect(),
            "EVENT",
            Quantity::Number,
        ),
        column(
            records.iter().map(|r| f64::from(r.kind >> 3)).collect(),
            "VOLT. CHANNEL",
            Quantity::Number,
        ),
    ];
    for channel in 1..=VOLT_CHANNELS {
        let (times, values) = records
            .iter()
            .zip(&time_s)
            .filter(|(record, _)| record.kind >> 3 == channel)
            .map(|(record, time)| (*time, f64::from(record.volt_count)))
            .unzip();
        channels.push(Channel {
            column: format!("VOLTAGE count (channel {channel})"),
            quantity: Quantity::Unrecognized,
            time_s: times,
            values,
        });
    }
    let events = records
        .iter()
        .zip(&time_s)
        .filter(|(record, _)| record.event != 0)
        .map(|(record, time)| Event {
            time_s: *time,
            name: EVENT_NAMES
                .iter()
                .find(|(code, _)| *code == record.event)
                .map_or_else(
                    || format!("event {}", record.event),
                    |(_, name)| (*name).to_owned(),
                ),
        })
        .collect();

    let mut stated_figures = Stated {
        samples: Some(stated),
        ..Stated::default()
    };
    for (name, at) in [
        ("zero pressure", ZERO_PRESSURE_AT),
        ("apogee pressure", APOGEE_PRESSURE_AT),
    ] {
        if let Some(pa) = header_u32(bytes, at) {
            stated_figures.figures.push(Figure::new(
                name,
                format!("{pa} Pa"),
                Some(Quantity::Pressure),
                vec![f64::from(pa)],
            ));
        }
    }

    let end = offset(records.len());
    let mut notes = vec![format!(
        "the clock isn't stored: each record is {FAST_STEP_S} s after the one before, or \
         {SLOW_STEP_S} s where its kind's bit 2 is set, the 200 and 40 samples a second the \
         maker's manual gives"
    )];
    if records.len() < stated {
        notes.push(format!(
            "the header states {stated} records, but the file ends after {}: read those",
            records.len()
        ));
    } else if bytes.len() > end {
        notes.push(format!(
            "{} bytes after the records, a trailer, aren't read",
            bytes.len() - end
        ));
    }
    notes.push(
        "the voltage counts are kept as written: no document states their factor to volts"
            .to_owned(),
    );
    Ok(FlightLog {
        format: LogFormat::AltimaxAls,
        logger: "Rocketronics AltiMAX".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: stated_figures,
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events,
        notes,
    })
}
