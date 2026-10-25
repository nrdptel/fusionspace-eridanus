//! Entacore AIM XTRA's own files: a flight as the AIM software saves it to an `.xtra` file, and
//! the flight computer's memory as the software downloads it, raw, before any conversion.
//!
//! The AIM software saves a flight in its own `.xtra` file and exports it to CSV
//! ([`entacore`](crate::entacore)); the manual names both but describes neither layout (*AIM
//! XTRA/BASE v4.x.x Manual*, Entacore, 2 February 2025, p38). The layouts here were worked out from
//! one flight's three files among the sample logs: a raw dump of the computer's memory, the
//! `.xtra` the software made of it, and an earlier copy of that `.xtra`, cut short. The raw dump's
//! last recording reads, by the rules below, to the `.xtra`'s values bit for bit, and the short
//! copy's bytes are the `.xtra`'s first ones. So every rule here is checked against the
//! maker's own conversion, on one flight.
//!
//! **What the records hold.** One record a sample of one group of readings, each group at its own
//! rate (the manual gives the barometer every 20 ms, the accelerometers and gyroscope every 10 ms,
//! the magnetometer every 100 ms, the temperature every 500 ms, and the GPS's position every
//! 200 ms and its time every 500 ms, p2, each written at a multiple of its period the owner sets
//! for each phase of the flight, p21). Each
//! group is a kind of record, and each of its fields a channel, on the clock of that kind's
//! records. The names are the CSV export's, where its column is the same sensor:
//!
//! | Kind: `.xtra` class, raw type | Channels | Quantity, unit | Raw value |
//! |---|---|---|---|
//! | 2, `0x02` | `acceleration` | [`Quantity::Acceleration`], g × 9.806 65 m/s², the CSV's | i16 / 256 |
//! | 3, `0x0b` | `lat. X accel.`, `lat. Y accel.` | the same | i16 / 256 each |
//! | 15, `0x0c` | `Gyro X`, `Gyro Y`, `Gyro Z` | [`Quantity::AngularRate`], °/s (p24) | i16 × 0.07 each |
//! | 4, `0x03` | `pressure` | [`Quantity::Pressure`], Pa | u24 |
//! | 5, `0x04`; 6, `0x05` | `battery 1 voltage`, `battery 2 voltage` | [`Quantity::Voltage`], V | (u16 ÷ 16) × (3.3 × 133 / 33) / 4096, the factor in single precision |
//! | 7, `0x06` | `temperature` | [`Quantity::Temperature`], °C + 273.15 K | i16 × 0.01 |
//! | 17–20, `0x07`–`0x0a` | `A Voltage` … `D Voltage` | [`Quantity::Voltage`], V | u16 × 3.3 / 16,384 |
//! | 16, `0x0d` | `Mag X`, `Mag Y`, `Mag Z` | [`Quantity::Unrecognized`], as written | i16 × 10⁻⁶ each |
//! | 10, `0x0e` | `lat`, `long`, `GPS MSL`, `SVs`, `GPS flag` | degrees, degrees, [`Quantity::GnssAltitude`] m, [`Quantity::Number`], as written | i32 × f32(10⁻⁷), i32 × f32(10⁻⁷), i32 mm as an f32 × f32(0.001), a byte's bits 0–4, its bit 5 |
//! | 14, `0x12` | `GPS Time of Week`, `GPS week`, `GPS time flag 1`, `GPS time flag 2`, `GPS leap seconds` | as written (ms, as the CSV's), [`Quantity::Number`], as written, as written, [`Quantity::Number`] | u32, u16, a byte's bit 7, its bit 6, its bits 0–5 |
//! | 13, `0x10` | `state`, `A on` … `D on`, `A cont` … `D cont` | [`Quantity::Number`] | a u16's top 4 bits (0, 1, 3, 5 read as 0, 1, 2, 3), bits 8–11, bits 7 to 4 |
//! | 21, `0x15` | `quaternion 1` … `quaternion 4` | [`Quantity::Unrecognized`], as written | i16 / 32,768 each |
//!
//! The raw values are big-endian, and are converted as the software converts them, in 32-bit
//! floats, so that the two readers' channels are equal value for value. The accelerations are
//! recorded in g, as the CSV's are, and read in m/s²; `lat. X accel.` reads 16 g at −4,096, the
//! 16 g its sensor measures (p2).
//! Which battery is the computer's and which the ejection charges' isn't known: both read 6.6 V in
//! the sample. The magnetometer's unit isn't settled, as for the CSV. The quaternion is kept as
//! written. The `on` and `cont` names, and the letters, are inferred from the sample: the CSV
//! names four outputs' firing and continuity flags so, and in the sample C and D fire and lose
//! continuity when the recovery charges do; A and B never change. What `state`'s numbers mean
//! isn't stated either: in the sample it turns 1 about a second into the boost, 2 later in the
//! climb and 3 at apogee.
//!
//! **Events.** Each change of `state` is an event named `state N`, and each rise of an `on` flag
//! from 0 to 1 an event named by its channel, at the record's time.
//!
//! **No primary track.** Neither file holds an altitude above the pad, so
//! [`FlightLog::time_s`] and [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`,
//! and every value is a channel.
//!
//! **The `.xtra` file** is a Boost serialization binary archive: a signature,
//! `serialization::archive`, the archive's version (16), the sizes of an int, a long, a float and
//! a double (4, 4, 4, 8), the top object's header, and the record count as a u64. Each record is
//! then its class as an i16 (followed, the first time the class appears, by a byte 1 and the
//! class's version as a u32), its object id (twice its index), on the first record only a byte 1
//! and a u32 1, its base object's id (one more), the base's fields (an f32 of 3.3, a byte 0, and
//! the record's time in seconds as an f32), then the kind's fields, little-endian. A class whose
//! version isn't the sample's is refused, as its fields may differ. A file that ends inside a
//! record is read to its last whole one, with a note; bytes after the stated count, 114 in the
//! sample, aren't read.
//!
//! **The raw dump** ([`read_dump`]) is the computer's memory: recordings back to back, then
//! erased memory, `0xFF`, to its end. A recording opens with type `0x13` and 101 bytes this
//! reader doesn't read. A byte from `0x80` to `0xFE` adds its low 7 bits to the clock, in the
//! computer's ticks; any other byte is a record's type, its fields following. The clock is read as
//! the ticks in milliseconds from the recording's start, as the software reads them. Two things
//! the dump and the `.xtra` differ in, both the software's: it sums the `.xtra`'s times in 32-bit
//! floats, a tick at a time, so they fall behind the ticks as the sum grows: in the sample, by
//! 2.40 s by the end of the recording, and by 0.9 % of the climb's time; and it leaves the dump's
//! last record out. Neither file's clock is GPS time: over the sample's flight the computer's tick
//! is 1.009 ms of the GPS's own clock, 1.021 on the pad, so the dump's times run 0.9 % short in
//! flight and 2.1 % on the pad; the `.xtra`'s, behind the ticks as well, run about 1.8 % short
//! over the climb (0.9910 ÷ 1.0087 = 0.982).
//!
//! **Held to the sample.** Each byte whose meaning the sample doesn't show is held to the one
//! value it has in every sample record, and a file where it differs is refused, naming the byte:
//! the `.xtra`'s base fields, 3.3 and 0, the output lines' two flag bytes, 0, and the status's
//! third set of four bytes, 0; the dump's battery bits 0–3, the output lines' bits 14–15, the
//! GPS byte's bits 6–7, the status's bits 0–3 and a state number other than the four.
//! [`recognizes_xtra`] and [`recognizes_dump`] don't read these, so a file that differs there is
//! refused by name, not left unrecognized. A type or class the sample doesn't hold is refused
//! too, as its length is unknown; among a dump's first 1,000 records, or as an `.xtra`'s first
//! record, it leaves the file unrecognized instead.
//!
//! ```
//! use hpr_flightdata::entacore_raw;
//!
//! // An invented raw dump: a recording's header, 10 ticks, a pressure of 97,700 Pa (0x017DA4),
//! // 20 ticks, another, then erased memory.
//! let mut bytes = vec![0x13];
//! bytes.extend([0; 101]);
//! bytes.extend([0x8A, 0x03, 0x01, 0x7D, 0xA4, 0x94, 0x03, 0x01, 0x7D, 0x90]);
//! bytes.extend([0xFF; 16]);
//! let recordings = entacore_raw::read_dump(&bytes)?;
//! let pressure = &recordings[0].channels[0];
//! assert_eq!(pressure.column, "pressure");
//! assert_eq!(pressure.time_s, [0.01, 0.03]);
//! assert_eq!(pressure.values, [97_700.0, 97_680.0]);
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use std::collections::BTreeMap;

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};

const XTRA_FORMAT: &str = "Entacore AIM .xtra";
const DUMP_FORMAT: &str = "Entacore AIM raw dump";

/// The `.xtra`'s first 36 bytes: the signature's length and text, the archive's version, the
/// sizes of an int, a long, a float and a double, and 1, which tells the byte order.
const XTRA_SIGNATURE: &[u8; 36] =
    b"\x16\0\0\0serialization::archive\x10\0\x04\x04\x04\x08\x01\0\0\0";

/// The 10 bytes after the signature, the top object's header, as the sample writes them.
const XTRA_TOP: [u8; 10] = [0, 3, 0, 0, 0, 0, 0, 0, 0, 0];

/// Where the record count and the first record are.
const XTRA_COUNT_AT: usize = 46;
const XTRA_RECORDS_AT: usize = 54;

/// The base's first field, as every sample record holds it.
const XTRA_BASE_FIRST: f32 = 3.3;

/// A recording's opening type in a raw dump, and the bytes that follow it.
const DUMP_RECORDING: u8 = 0x13;
const DUMP_RECORDING_BYTES: usize = 101;

/// Erased memory.
const ERASED: u8 = 0xFF;

/// The records [`recognizes_dump`] reads.
const RECOGNIZED_RECORDS: usize = 1_000;

/// What a field's raw value is read as.
#[derive(Clone, Copy)]
enum Unit {
    /// Its value times a factor, in the quantity's unit.
    Scale(f64),
    /// °C, read in K.
    Celsius,
}

/// One field of a kind: its channel's name, quantity and unit.
struct Field {
    name: &'static str,
    quantity: Quantity,
    unit: Unit,
}

const fn field(name: &'static str, quantity: Quantity, unit: Unit) -> Field {
    Field {
        name,
        quantity,
        unit,
    }
}

const G: Unit = Unit::Scale(STANDARD_GRAVITY_MPS2);
const AS_IS: Unit = Unit::Scale(1.0);

/// One kind of record: its class and the class's version in an `.xtra`, its type and length in a
/// raw dump, its length in an `.xtra`, and its fields.
struct Kind {
    class: i16,
    version: u32,
    raw_type: u8,
    raw_bytes: usize,
    xtra_bytes: usize,
    fields: &'static [Field],
}

const KINDS: [Kind; 16] = [
    Kind {
        class: 2,
        version: 3,
        raw_type: 0x02,
        raw_bytes: 2,
        xtra_bytes: 4,
        fields: &[field("acceleration", Quantity::Acceleration, G)],
    },
    Kind {
        class: 3,
        version: 3,
        raw_type: 0x0b,
        raw_bytes: 4,
        xtra_bytes: 8,
        fields: &[
            field("lat. X accel.", Quantity::Acceleration, G),
            field("lat. Y accel.", Quantity::Acceleration, G),
        ],
    },
    Kind {
        class: 15,
        version: 0,
        raw_type: 0x0c,
        raw_bytes: 6,
        xtra_bytes: 12,
        fields: &[
            field("Gyro X", Quantity::AngularRate, AS_IS),
            field("Gyro Y", Quantity::AngularRate, AS_IS),
            field("Gyro Z", Quantity::AngularRate, AS_IS),
        ],
    },
    Kind {
        class: 4,
        version: 1,
        raw_type: 0x03,
        raw_bytes: 3,
        xtra_bytes: 4,
        fields: &[field("pressure", Quantity::Pressure, AS_IS)],
    },
    Kind {
        class: 5,
        version: 1,
        raw_type: 0x04,
        raw_bytes: 2,
        xtra_bytes: 4,
        fields: &[field("battery 1 voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 6,
        version: 1,
        raw_type: 0x05,
        raw_bytes: 2,
        xtra_bytes: 4,
        fields: &[field("battery 2 voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 7,
        version: 1,
        raw_type: 0x06,
        raw_bytes: 2,
        xtra_bytes: 4,
        fields: &[field("temperature", Quantity::Temperature, Unit::Celsius)],
    },
    Kind {
        class: 17,
        version: 0,
        raw_type: 0x07,
        raw_bytes: 2,
        xtra_bytes: 6,
        fields: &[field("A Voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 18,
        version: 0,
        raw_type: 0x08,
        raw_bytes: 2,
        xtra_bytes: 6,
        fields: &[field("B Voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 19,
        version: 0,
        raw_type: 0x09,
        raw_bytes: 2,
        xtra_bytes: 6,
        fields: &[field("C Voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 20,
        version: 0,
        raw_type: 0x0a,
        raw_bytes: 2,
        xtra_bytes: 6,
        fields: &[field("D Voltage", Quantity::Voltage, AS_IS)],
    },
    Kind {
        class: 16,
        version: 0,
        raw_type: 0x0d,
        raw_bytes: 6,
        xtra_bytes: 12,
        fields: &[
            field("Mag X", Quantity::Unrecognized, AS_IS),
            field("Mag Y", Quantity::Unrecognized, AS_IS),
            field("Mag Z", Quantity::Unrecognized, AS_IS),
        ],
    },
    Kind {
        class: 10,
        version: 4,
        raw_type: 0x0e,
        raw_bytes: 13,
        xtra_bytes: 25,
        fields: &[
            field("lat", Quantity::Latitude, AS_IS),
            field("long", Quantity::Longitude, AS_IS),
            field("GPS MSL", Quantity::GnssAltitude, AS_IS),
            field("SVs", Quantity::Number, AS_IS),
            field("GPS flag", Quantity::Unrecognized, AS_IS),
        ],
    },
    Kind {
        class: 14,
        version: 2,
        raw_type: 0x12,
        raw_bytes: 7,
        xtra_bytes: 12,
        fields: &[
            field("GPS Time of Week", Quantity::Unrecognized, AS_IS),
            field("GPS week", Quantity::Number, AS_IS),
            field("GPS time flag 1", Quantity::Unrecognized, AS_IS),
            field("GPS time flag 2", Quantity::Unrecognized, AS_IS),
            field("GPS leap seconds", Quantity::Number, AS_IS),
        ],
    },
    Kind {
        class: 13,
        version: 1,
        raw_type: 0x10,
        raw_bytes: 2,
        xtra_bytes: 28,
        fields: &[
            field("state", Quantity::Number, AS_IS),
            field("A on", Quantity::Number, AS_IS),
            field("B on", Quantity::Number, AS_IS),
            field("C on", Quantity::Number, AS_IS),
            field("D on", Quantity::Number, AS_IS),
            field("A cont", Quantity::Number, AS_IS),
            field("B cont", Quantity::Number, AS_IS),
            field("C cont", Quantity::Number, AS_IS),
            field("D cont", Quantity::Number, AS_IS),
        ],
    },
    Kind {
        class: 21,
        version: 0,
        raw_type: 0x15,
        raw_bytes: 8,
        xtra_bytes: 16,
        fields: &[
            field("quaternion 1", Quantity::Unrecognized, AS_IS),
            field("quaternion 2", Quantity::Unrecognized, AS_IS),
            field("quaternion 3", Quantity::Unrecognized, AS_IS),
            field("quaternion 4", Quantity::Unrecognized, AS_IS),
        ],
    },
];

/// The status's state numbers, by its u16's top 4 bits, as the software converts them.
const STATES: [(u16, f64); 4] = [(0x0, 0.0), (0x1, 1.0), (0x3, 2.0), (0x5, 3.0)];

/// One sample: its kind, its time and its fields' raw values, as the software converts them.
struct Sample {
    kind: &'static Kind,
    time_s: f64,
    values: Vec<f64>,
}

/// Reads a little-endian value of `N` bytes at `at`.
fn le<const N: usize>(bytes: &[u8], at: usize) -> Option<[u8; N]> {
    bytes.get(at..at.checked_add(N)?)?.try_into().ok()
}

/// An `.xtra` kind's fields from its payload, or the byte in it, and why, where one isn't held
/// to the sample.
fn xtra_values(kind: &Kind, data: &[u8]) -> Result<Vec<f64>, (usize, String)> {
    let f64_at = |at: usize| {
        f64::from_le_bytes([
            data[at],
            data[at + 1],
            data[at + 2],
            data[at + 3],
            data[at + 4],
            data[at + 5],
            data[at + 6],
            data[at + 7],
        ])
    };
    let f32_at = |at: usize| {
        f64::from(f32::from_le_bytes([
            data[at],
            data[at + 1],
            data[at + 2],
            data[at + 3],
        ]))
    };
    let u32_at =
        |at: usize| u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]]);
    // Invariant: `data` is `kind.xtra_bytes` long, which each arm's offsets stay within.
    Ok(match kind.class {
        10 => vec![
            f64_at(0),
            f64_at(8),
            f32_at(16),
            f64::from(u32_at(20)),
            f64::from(data[24]),
        ],
        13 => {
            for (at, name) in [(4, "first"), (12, "second"), (20, "third")] {
                if u32_at(at) != 4 {
                    return Err((
                        at,
                        format!(
                            "the status's {name} set states {} bytes, where every sample \
                             record's holds 4",
                            u32_at(at)
                        ),
                    ));
                }
            }
            if let Some(at) = data[24..28].iter().position(|&b| b != 0) {
                return Err((
                    24 + at,
                    "the status's third set of four bytes isn't zero, as it is in every sample \
                     record"
                        .to_owned(),
                ));
            }
            let mut values = vec![f64::from(u32_at(0))];
            values.extend(
                data[8..12]
                    .iter()
                    .chain(&data[16..20])
                    .map(|&b| f64::from(b)),
            );
            values
        }
        14 => vec![
            f64::from(u32_at(0)),
            f64::from(u16::from_le_bytes([data[4], data[5]])),
            f64::from(data[6]),
            f64::from(data[7]),
            f64::from(u32_at(8)),
        ],
        17..=20 => {
            if let Some(at) = data[0..2].iter().position(|&b| b != 0) {
                return Err((
                    at,
                    "an output line's two flag bytes aren't zero, as they are in every sample \
                     record"
                        .to_owned(),
                ));
            }
            vec![f32_at(2)]
        }
        _ => (0..kind.xtra_bytes / 4).map(|k| f32_at(4 * k)).collect(),
    })
}

/// A raw kind's fields from its big-endian payload, converted as the software converts them, or
/// the byte in it, and why, where a bit isn't held to the sample.
fn raw_values(kind: &Kind, data: &[u8]) -> Result<Vec<f64>, (usize, String)> {
    // Invariant: `data` is `kind.raw_bytes` long, which each arm's offsets stay within.
    let i16_at = |k: usize| i16::from_be_bytes([data[2 * k], data[2 * k + 1]]);
    let u16_at = |k: usize| u16::from_be_bytes([data[2 * k], data[2 * k + 1]]);
    let times = |count: usize, factor: f32| -> Vec<f64> {
        (0..count)
            .map(|k| f64::from(f32::from(i16_at(k)) * factor))
            .collect()
    };
    Ok(match kind.raw_type {
        0x02 | 0x0b => (0..kind.raw_bytes / 2)
            .map(|k| f64::from(f32::from(i16_at(k)) / 256.0))
            .collect(),
        0x0c => times(3, 0.07),
        0x0d => times(3, 1e-6),
        0x06 => times(1, 0.01),
        0x15 => (0..4)
            .map(|k| f64::from(f32::from(i16_at(k)) / 32_768.0))
            .collect(),
        0x03 => vec![f64::from(u32::from_be_bytes([
            0, data[0], data[1], data[2],
        ]))],
        0x04 | 0x05 => {
            let raw = u16_at(0);
            // Bits 0–3 are in the second byte, big-endian.
            if raw & 0x000F != 0 {
                return Err((
                    1,
                    format!(
                        "a battery's bits 0–3 aren't zero ({raw:#06x}), as they are in every sample \
                     record"
                    ),
                ));
            }
            // As the software: the 12-bit count times 3.3 V × 133/33 taken in single precision
            // (f32(3.3) × f32(133/33) = 13.299 999 2, one ulp below f32(13.3)), over 4096,
            // rounded to an f32. With this factor, every battery value in the sample equals the
            // `.xtra`'s, as the survey's comparison shows.
            let volts =
                f64::from(raw >> 4) * f64::from(3.3_f32 * (133.0_f64 / 33.0) as f32) / 4_096.0;
            vec![f64::from(volts as f32)]
        }
        0x07..=0x0a => {
            let raw = u16_at(0);
            if raw & 0xC000 != 0 {
                return Err((
                    0,
                    format!(
                        "an output line's bits 14–15 aren't zero ({raw:#06x}), as they are in every \
                     sample record"
                    ),
                ));
            }
            vec![f64::from(f32::from(raw) * ((3.3_f64 / 16_384.0) as f32))]
        }
        0x0e => {
            let i32_at = |at: usize| {
                i32::from_be_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
            };
            let flags = data[12];
            if flags & 0xC0 != 0 {
                return Err((
                    12,
                    format!(
                        "the GPS byte's bits 6–7 aren't zero ({flags:#04x}), as they are in every \
                     sample record"
                    ),
                ));
            }
            let degrees = f64::from(1e-7_f32);
            // The altitude in mm, as an f32 times f32(0.001); `as` rounds to the nearest f32.
            let altitude_m = i32_at(8) as f32 * 0.001;
            vec![
                f64::from(i32_at(0)) * degrees,
                f64::from(i32_at(4)) * degrees,
                f64::from(altitude_m),
                f64::from(flags & 0x1F),
                f64::from(flags >> 5 & 1),
            ]
        }
        0x10 => {
            let raw = u16_at(0);
            let Some((_, state)) = STATES.iter().find(|(top, _)| *top == raw >> 12) else {
                return Err((
                    0,
                    format!("the status's top 4 bits ({raw:#06x}) are no state the sample holds"),
                ));
            };
            if raw & 0x000F != 0 {
                return Err((
                    1,
                    format!(
                        "the status's bits 0–3 aren't zero ({raw:#06x}), as they are in every sample \
                     record"
                    ),
                ));
            }
            let mut values = vec![*state];
            values.extend((0..4).map(|line| f64::from(raw >> (8 + line) & 1)));
            values.extend((0..4).map(|line| f64::from(raw >> (7 - line) & 1)));
            values
        }
        0x12 => {
            let flags = data[6];
            vec![
                f64::from(u32::from_be_bytes([data[0], data[1], data[2], data[3]])),
                f64::from(u16::from_be_bytes([data[4], data[5]])),
                f64::from(flags >> 7 & 1),
                f64::from(flags >> 6 & 1),
                f64::from(flags & 0x3F),
            ]
        }
        // Invariant: `KINDS` holds no other raw type.
        other => return Err((0, format!("type {other:#04x} has no conversion"))),
    })
}

/// The record built from samples: a channel for each field, in the order each kind first
/// appears, and the status's events.
fn build(format: LogFormat, samples: &[Sample], notes: Vec<String>) -> FlightLog {
    let mut order: Vec<i16> = Vec::new();
    let mut by_kind: BTreeMap<i16, Vec<&Sample>> = BTreeMap::new();
    for sample in samples {
        let class = sample.kind.class;
        if !by_kind.contains_key(&class) {
            order.push(class);
        }
        by_kind.entry(class).or_default().push(sample);
    }
    let mut channels = Vec::new();
    let mut events = Vec::new();
    for class in order {
        let kind_samples = &by_kind[&class];
        // Invariant: each class in `order` has a sample.
        let kind = kind_samples[0].kind;
        let time_s: Vec<f64> = kind_samples.iter().map(|s| s.time_s).collect();
        for (index, field) in kind.fields.iter().enumerate() {
            let values = kind_samples
                .iter()
                .map(|s| match field.unit {
                    Unit::Scale(factor) => s.values[index] * factor,
                    Unit::Celsius => s.values[index] + 273.15,
                })
                .collect();
            channels.push(Channel {
                column: field.name.to_owned(),
                quantity: field.quantity,
                time_s: time_s.clone(),
                values,
            });
        }
        if kind.raw_type == 0x10 {
            let mut before: Option<&Sample> = None;
            for sample in kind_samples {
                if let Some(before) = before {
                    if sample.values[0] != before.values[0] {
                        events.push(Event {
                            time_s: sample.time_s,
                            name: format!("state {}", sample.values[0]),
                        });
                    }
                    for line in 0..4 {
                        if sample.values[1 + line] == 1.0 && before.values[1 + line] == 0.0 {
                            events.push(Event {
                                time_s: sample.time_s,
                                name: kind.fields[1 + line].name.to_owned(),
                            });
                        }
                    }
                }
                before = Some(sample);
            }
        }
    }
    FlightLog {
        format,
        logger: "Entacore AIM XTRA".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            samples: Some(samples.len()),
            ..Stated::default()
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events,
        notes,
    }
}

/// Whether `bytes` are an AIM `.xtra` file: a Boost binary archive's signature, the sample's
/// top object header, and a first record whose class is an AIM kind's. The rest isn't read here:
/// [`read_xtra`] refuses a file whose records differ from the sample's, naming the byte.
pub fn recognizes_xtra(bytes: &[u8]) -> bool {
    let class = le::<2>(bytes, XTRA_RECORDS_AT).map(i16::from_le_bytes);
    bytes.starts_with(XTRA_SIGNATURE)
        && le::<10>(bytes, XTRA_SIGNATURE.len()) == Some(XTRA_TOP)
        && class.is_some_and(|class| KINDS.iter().any(|kind| kind.class == class))
}

fn xtra_error(offset: usize, message: String) -> LogError {
    LogError::Binary {
        format: XTRA_FORMAT,
        offset,
        message,
    }
}

/// The `.xtra`'s samples, and its notes.
fn xtra_samples(bytes: &[u8]) -> Result<(Vec<Sample>, Vec<String>), LogError> {
    if !bytes.starts_with(XTRA_SIGNATURE) {
        return Err(LogError::NotThisFormat {
            format: XTRA_FORMAT,
            message: "it doesn't open with a Boost binary archive's signature, \
                      `serialization::archive`, version 16, and its sizes"
                .to_owned(),
        });
    }
    let top = le::<10>(bytes, XTRA_SIGNATURE.len());
    if top != Some(XTRA_TOP) {
        return Err(xtra_error(
            XTRA_SIGNATURE.len(),
            "the top object's header isn't the sample's: a version this reader doesn't know"
                .to_owned(),
        ));
    }
    let count = le::<8>(bytes, XTRA_COUNT_AT)
        .map(u64::from_le_bytes)
        .ok_or_else(|| {
            xtra_error(
                XTRA_COUNT_AT,
                "the file ends before its record count".to_owned(),
            )
        })?;
    let mut samples = Vec::new();
    let mut notes = Vec::new();
    let mut seen: Vec<i16> = Vec::new();
    let mut at = XTRA_RECORDS_AT;
    let mut index: u64 = 0;
    while index < count {
        let start = at;
        let Some(sample) = xtra_record(bytes, &mut at, index, &mut seen)? else {
            notes.push(if bytes.len() == start {
                format!("the file ends after record {index} of the {count} it states: read those")
            } else {
                format!(
                    "the file ends inside record {} of the {count} it states, {} bytes into it: \
                     read the {index} before",
                    index + 1,
                    bytes.len() - start
                )
            });
            return Ok((samples, notes));
        };
        samples.push(sample);
        index += 1;
    }
    if bytes.len() > at {
        notes.push(format!(
            "{} bytes after the {count} records aren't read",
            bytes.len() - at
        ));
    }
    Ok((samples, notes))
}

/// The `n` bytes at `*at`, moving `*at` past them; `None` if the file ends first.
fn take<'a>(bytes: &'a [u8], at: &mut usize, n: usize) -> Option<&'a [u8]> {
    let data = bytes.get(*at..at.checked_add(n)?)?;
    *at += n;
    Some(data)
}

/// The record at `*at`, the `index`th, moving `*at` past it; `None` if the file ends inside it.
fn xtra_record(
    bytes: &[u8],
    at: &mut usize,
    index: u64,
    seen: &mut Vec<i16>,
) -> Result<Option<Sample>, LogError> {
    let record = index + 1;
    let start = *at;
    let Some(class) = take(bytes, at, 2).map(|b| i16::from_le_bytes([b[0], b[1]])) else {
        return Ok(None);
    };
    let Some(kind) = KINDS.iter().find(|kind| kind.class == class) else {
        return Err(xtra_error(
            start,
            format!("record {record}: class {class}, which no sample holds"),
        ));
    };
    if !seen.contains(&class) {
        let header = *at;
        let Some(info) = take(bytes, at, 5) else {
            return Ok(None);
        };
        let version = u32::from_le_bytes([info[1], info[2], info[3], info[4]]);
        if info[0] != 1 {
            return Err(xtra_error(
                header,
                format!(
                    "record {record}: class {class}'s header opens with {}, where every sample's \
                     opens with 1",
                    info[0]
                ),
            ));
        }
        if version != kind.version {
            return Err(xtra_error(
                header + 1,
                format!(
                    "record {record}: class {class}'s version is {version}, where the sample's \
                     is {}: its fields may differ",
                    kind.version
                ),
            ));
        }
        seen.push(class);
    }
    let id_at = *at;
    let Some(id) = take(bytes, at, 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])) else {
        return Ok(None);
    };
    if u64::from(id) != 2 * index {
        return Err(xtra_error(
            id_at,
            format!("record {record}: its object id is {id}, not {}", 2 * index),
        ));
    }
    if index == 0 {
        let header = *at;
        let Some(base) = take(bytes, at, 5) else {
            return Ok(None);
        };
        if base != [1, 1, 0, 0, 0] {
            return Err(xtra_error(
                header,
                "the first record's base header isn't the sample's".to_owned(),
            ));
        }
    }
    let base_at = *at;
    let Some(base) = take(bytes, at, 13) else {
        return Ok(None);
    };
    let base_id = u32::from_le_bytes([base[0], base[1], base[2], base[3]]);
    let first = f32::from_le_bytes([base[4], base[5], base[6], base[7]]);
    let time = f32::from_le_bytes([base[9], base[10], base[11], base[12]]);
    // The base's fields in turn: its id, the 3.3, the 0.
    let fault = if u64::from(base_id) != 2 * index + 1 {
        Some(0)
    } else if first != XTRA_BASE_FIRST {
        Some(4)
    } else if base[8] != 0 {
        Some(8)
    } else {
        None
    };
    if let Some(field) = fault {
        return Err(xtra_error(
            base_at + field,
            format!(
                "record {record}: its base isn't the sample's (an id one past the record's, \
                 {XTRA_BASE_FIRST} and 0)"
            ),
        ));
    }
    if !time.is_finite() {
        return Err(xtra_error(
            base_at + 9,
            format!("record {record}: its time isn't a number"),
        ));
    }
    let data_at = *at;
    let Some(data) = take(bytes, at, kind.xtra_bytes) else {
        return Ok(None);
    };
    let values = xtra_values(kind, data).map_err(|(field, message)| {
        xtra_error(data_at + field, format!("record {record}: {message}"))
    })?;
    Ok(Some(Sample {
        kind,
        time_s: f64::from(time),
        values,
    }))
}

/// Reads an AIM `.xtra` file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when it doesn't open with the archive's signature;
/// [`LogError::Binary`], naming the byte, when the top object's header, a class, a class's
/// version, an object id or a byte held to the sample isn't as the module describes;
/// [`LogError::NoData`] when it holds no whole record.
pub fn read_xtra(bytes: &[u8]) -> Result<FlightLog, LogError> {
    let (samples, mut notes) = xtra_samples(bytes)?;
    if samples.is_empty() {
        return Err(LogError::NoData {
            format: XTRA_FORMAT,
        });
    }
    held_pad_pressure(&samples, XTRA_FORMAT)?;
    notes.push(
        "the times are the software's, summed in 32-bit floats. So they drift from the \
         computer's own clock as they grow, behind it in the one sample. That clock isn't \
         the GPS's. On the sample's flight, a tick is 1.009 to 1.021 ms of the GPS's clock. \
         Over its climb these times run about 1.8 % short of the GPS's"
            .to_owned(),
    );
    Ok(build(LogFormat::EntacoreXtra, &samples, notes))
}

/// Refuses a recording whose first pressure isn't a pad's, 50,000 to 110,000 Pa: its unit would
/// be something else.
fn held_pad_pressure(samples: &[Sample], format: &'static str) -> Result<(), LogError> {
    match samples.iter().find(|s| s.kind.raw_type == 0x03) {
        Some(first) if !(50_000.0..=110_000.0).contains(&first.values[0]) => {
            Err(LogError::NotThisFormat {
                format,
                message: format!(
                    "its first pressure, {}, isn't a pad's in pascals, 50,000 to 110,000",
                    first.values[0]
                ),
            })
        }
        _ => Ok(()),
    }
}

fn dump_error(offset: usize, message: String) -> LogError {
    LogError::Binary {
        format: DUMP_FORMAT,
        offset,
        message,
    }
}

/// One recording of a raw dump: where it starts, the records walked in it, its samples.
struct Recording {
    start: usize,
    records: usize,
    samples: Vec<Sample>,
}

/// The raw dump's recordings, reading at most `limit` records in all. With `held` false, as
/// [`recognizes_dump`] walks it, a record whose bits aren't held to the sample is passed over,
/// not refused: only [`read_dump`] refuses it, naming the byte.
fn dump_recordings(bytes: &[u8], limit: usize, held: bool) -> Result<Vec<Recording>, LogError> {
    if bytes.first() != Some(&DUMP_RECORDING) {
        return Err(LogError::NotThisFormat {
            format: DUMP_FORMAT,
            message: "it doesn't open with a recording's type, 0x13".to_owned(),
        });
    }
    let mut recordings: Vec<Recording> = Vec::new();
    let mut ticks: u64 = 0;
    let mut read = 0;
    let mut at = 0;
    while at < bytes.len() && read < limit {
        let byte = bytes[at];
        if byte == ERASED {
            if let Some(stray) = bytes[at..].iter().position(|&b| b != ERASED) {
                return Err(dump_error(
                    at + stray,
                    format!(
                        "data after the erased memory that starts at byte {at}: the reader \
                         doesn't know a gap in the memory"
                    ),
                ));
            }
            break;
        }
        if byte & 0x80 != 0 {
            ticks += u64::from(byte & 0x7F);
            at += 1;
            continue;
        }
        if byte == DUMP_RECORDING {
            if bytes.len() - at < 1 + DUMP_RECORDING_BYTES {
                return Err(dump_error(
                    at,
                    "the dump ends inside a recording's opening bytes".to_owned(),
                ));
            }
            recordings.push(Recording {
                start: at,
                records: 0,
                samples: Vec::new(),
            });
            ticks = 0;
            at += 1 + DUMP_RECORDING_BYTES;
            continue;
        }
        let Some(kind) = KINDS.iter().find(|kind| kind.raw_type == byte) else {
            return Err(dump_error(
                at,
                format!("type {byte:#04x}, which no sample holds: its length isn't known"),
            ));
        };
        let Some(data) = bytes.get(at + 1..at + 1 + kind.raw_bytes) else {
            return Err(dump_error(
                at,
                format!("the dump ends inside a record of type {byte:#04x}"),
            ));
        };
        let values = match raw_values(kind, data) {
            Ok(values) => Some(values),
            Err((field, message)) if held => return Err(dump_error(at + 1 + field, message)),
            Err(_) => None,
        };
        // Invariant: the dump opens with a recording, so one is open.
        if let Some(recording) = recordings.last_mut() {
            recording.records += 1;
            if let Some(values) = values {
                recording.samples.push(Sample {
                    kind,
                    time_s: ticks as f64 / 1_000.0,
                    values,
                });
            }
        }
        read += 1;
        at += 1 + kind.raw_bytes;
    }
    Ok(recordings)
}

/// Whether `bytes` are an AIM raw dump: a recording's opening, then records of the AIM's types,
/// up to 1,000, or erased memory. The bits held to the sample aren't read here: [`read_dump`]
/// refuses a dump that differs there, naming the byte.
pub fn recognizes_dump(bytes: &[u8]) -> bool {
    bytes.len() > DUMP_RECORDING_BYTES
        && dump_recordings(bytes, RECOGNIZED_RECORDS, false)
            .is_ok_and(|recordings| recordings.iter().any(|r| r.records > 0))
}

/// Reads an AIM raw dump: each recording in it as a record of its own, in the dump's order. A
/// recording that holds no record, one opening straight after another, is a record with no
/// channel, so that the records keep the dump's count.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when it doesn't open with a recording's type, or a recording's
/// first pressure isn't a pad's; [`LogError::Binary`], naming the byte, when a type isn't the
/// sample's, a bit held to the sample is set, the dump ends inside a record, or data follows
/// erased memory; [`LogError::NoData`] when it holds no record.
pub fn read_dump(bytes: &[u8]) -> Result<Vec<FlightLog>, LogError> {
    let recordings = dump_recordings(bytes, usize::MAX, true)?;
    if recordings.iter().all(|r| r.samples.is_empty()) {
        return Err(LogError::NoData {
            format: DUMP_FORMAT,
        });
    }
    let count = recordings.len();
    recordings
        .iter()
        .enumerate()
        .map(|(index, recording)| {
            held_pad_pressure(&recording.samples, DUMP_FORMAT)?;
            let notes = vec![
                format!(
                    "recording {} of the dump's {count}, from byte {}; its {DUMP_RECORDING_BYTES} \
                     opening bytes aren't read",
                    index + 1,
                    recording.start
                ),
                "the clock is the computer's ticks, read as milliseconds from the recording's \
                 start, as the AIM software reads them. On the one sample flight, a tick is \
                 1.009 to 1.021 ms of the GPS's clock"
                    .to_owned(),
            ];
            Ok(build(LogFormat::EntacoreDump, &recording.samples, notes))
        })
        .collect()
}

/// Reads an AIM raw dump that holds one recording, as [`crate::import::read`] does.
///
/// # Errors
///
/// As [`read_dump`], and [`LogError::Binary`] when the dump holds more than one recording,
/// naming where the second starts: [`read_dump`] reads each.
pub(crate) fn read_dump_one(bytes: &[u8]) -> Result<FlightLog, LogError> {
    let mut recordings = read_dump(bytes)?;
    if recordings.len() > 1 {
        let second = dump_recordings(bytes, usize::MAX, true)?
            .get(1)
            .map_or(0, |r| r.start);
        return Err(dump_error(
            second,
            format!(
                "the dump holds {} recordings, the second from this byte: \
                 `entacore_raw::read_dump` reads each as a record of its own",
                recordings.len()
            ),
        ));
    }
    // Invariant: `read_dump` returns a recording or an error.
    recordings.pop().ok_or(LogError::NoData {
        format: DUMP_FORMAT,
    })
}
