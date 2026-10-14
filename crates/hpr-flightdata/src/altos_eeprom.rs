//! AltOS raw flight downloads, as AltosUI saves a flight computer's memory to a `.eeprom` file:
//! the board's own log, record by record, in a JSON form or an older text form.
//!
//! An Altus Metrum flight computer logs its sensors 100 times a second in the climb and 10 times
//! a second in the descent, in records of 8 bytes (TeleMetrum v1), 16 (TeleMetrum v2 and newer,
//! EasyMini) or 32 (TeleMega, EasyMega) (*The Altus Metrum System*, Bdale Garbee and Keith
//! Packard, v1.9.22.2, 2025, Appendix F, Table 11, p. 77). AltosUI saves a flight it downloads as
//! a `.eeprom` file. Altus Metrum publishes no layout for the records, so this reader's layout was
//! worked out from the 14 sample files and the AltosUI exports (CSV) of the same flights beside 9
//! of them, and is held to those exports in the survey (`cargo xtask logs`); no AltOS or AltosUI
//! source code was read. The manual and *AltOS Telemetry* (v1.9.22.2) are published under CC
//! BY-SA 3.0; this reader restates facts from them, not their text.
//!
//! **Recognition.** The JSON form opens with a JSON object that names `log_format` and `product`;
//! the text form holds, within its first 1,000 lines, a `Config version:` line and a
//! `manufacturer altusmetrum.org` line, as the board prints its configuration. No other layout
//! HPR Sim reads does either.
//!
//! **The two forms.** The JSON form is the board's configuration as a JSON object, then the
//! memory's bytes, two hexadecimal digits each, whitespace between them. The text form is a serial
//! console's transcript: the configuration as the board prints it, then one record a line, a type
//! letter, the tick in hexadecimal and the record's other bytes, all but its checksum. In both, a
//! record is a type byte, a checksum byte, a 16-bit tick (low byte first) and the type's fields.
//!
//! - *Record size* follows the log format the configuration states (`log_format`, `log-format`):
//!   8 bytes for format 1; 16 for 6, 7 and 14; 32 for 15, 16 and 22. Any other number refuses the
//!   file, by number: read at a wrong size, misaligned bytes would make a plausible flight. A file
//!   that states no log format, as firmware before 1.0 didn't, is format 1 if its product is a
//!   TeleMetrum v1, as AltosUI does ("Those ancient versions didn't report the log format, so just
//!   use the product name instead", the manual's release notes for version 1.4, §H.43.2, p. 128);
//!   any other product refuses it. Format 1's text prints its four field bytes as two 16-bit words.
//! - *Checksum.* In the JSON form, every record's bytes sum, with 0x5A, to 0 modulo 256 (the seed
//!   the TeleDongle's line checksum uses, *AltOS Telemetry* §4, p. 10): every record of the seven
//!   JSON sample files passes, and fewer than 5% of chunks of a wrong size do. One that fails
//!   refuses the file ([`LogError::Binary`]). The text form prints no checksum.
//! - *Erased memory.* A record of all 0xFF bytes is erased flash, after the flight's last record,
//!   and is skipped and counted; a data record after one refuses the file, as the memory's order
//!   is then unknown.
//! - *A malformed line* in the text form's records refuses the file, but the last line of a file
//!   that ends without a line break, as a download cut short does, is left out with a note: it may
//!   be cut mid-value however well it reads.
//! - *The configuration* may be printed more than once in a transcript; the first copy is read,
//!   and a later one that differs in the log format, serial number, product, barometer constants
//!   or accelerometer calibration refuses the file. `2147483647` in the JSON means not set.
//!
//! **The clock.** The tick counts 100 a second (*AltOS Companion Port*, v1.9.22.2, Table 1;
//! Appendix F's 100 samples a second) and turns over every 655.36 s. Each record's tick goes on
//! the turn nearest the latest tick so far. The file's order isn't time order: the samples from
//! before boost, kept in a ring buffer, are written after the flight record, a state record ahead
//! of sensor records with earlier ticks, and a GPS record carries its fix's tick, up to 0.96 s
//! behind; so the records are put in tick order, the file's order kept among equal ticks. A record
//! more than 10 s from the latest tick, a silence no logging rate leaves, ends the flight: it and
//! every record after it are left out, counted in a note (one sample file ends with another
//! session's records, 332 s on). Records repeated exactly, the same bytes at the same tick, as
//! old firmware wrote some, are dropped and counted. Time zero is the flight record's tick, the
//! board's detection of boost, as in AltosUI's exports: the samples from the pad have negative
//! times. A file needs exactly one flight record (`F`), or it is refused.
//!
//! **What is read.** Each field is in SI where a cited conversion and a second check bear the
//! unit out, and kept as written ([`Quantity::Unrecognized`] for counts, [`Quantity::Number`] for
//! codes and flags) otherwise:
//!
//! | Board (format) | Field | Channel | As read |
//! |---|---|---|---|
//! | all with an MS5607 (6, 7, 14, 15, 16, 22) | the barometer's D1 and D2 | `pressure`, `temperature` | Pa and K, by the data sheet's compensation (below); `d1`, `d2` as written without its constants |
//! | TeleMetrum v1 (1) | pressure count | `pressure` | Pa, by the MP3H6115A's transfer function (below) |
//! | all but EasyMini | the high-g accelerometer's count | `acceleration` | m/s² along the board's axis, from the file's own calibration (below); `accel` as written without one |
//! | TeleMega, EasyMega (15, 16, 22) | pyro battery, igniters A–D, apogee and main senses | `pyro`, `igniter_a` … `igniter_d`, `drogue_voltage`, `main_voltage` | V, as *AltOS Telemetry* converts a Mega's: count / 4095 × 3.3 V × 127 / 27 (p. 3) |
//! | every board | battery and other senses | `v_batt`, `sense_a`, `sense_m` | as written: the factors that fit the exports have no source |
//! | TeleMetrum v1 (1) | the CC1111's die temperature | `temp` | as written: the data sheet's constants miss AltosUI's export by up to 0.10 K |
//! | EasyMega v2, TeleMega v6 (16, 22) | the IMU's nine words | `imu_accel_0` … `imu_mag_2` | as written: their scales fit the exports but have no source |
//! | every board | state | `state` | as written; each change an [`Event`] |
//! | boards with GPS | latitude, longitude | `latitude`, `longitude` | degrees, from ten-millionths, as *AltOS Telemetry*'s GPS packet (Table 15, pp. 6–7); a gap without a valid fix |
//! | | altitude | `gps_altitude` | m, as the GPS packet's; with the configuration's `altitude-32`, its high 16 bits too |
//! | | flags, time, date, dilutions, course, speed, climb, satellites | `gps_flags`, `nsat`, `hour` …, `year` …, `pdop` …, `course`, `ground_speed`, `climb_rate`, `svid`, `c_n` | as written: the date is two digits and needn't be the flight's |
//!
//! *Pressure and temperature, MS5607.* The data sheet's first-order compensation and, below
//! 20 °C, its second order (TE Connectivity, *MS5607-02BA03*, 03/2024, Figure 2, p. 8, and Figure
//! 3, p. 9), in integers, each division by 2ⁿ a floor (an arithmetic shift): truncating instead
//! misses the exports by 1 Pa at 6 of one cold flight's samples. Second check: the pad's pressure
//! (the median of the samples before boost, or the first sample's) must lie from 10 to 120 kPa and
//! within 1% of the flight record's ground pressure, or the file is refused: either failing means
//! misread bytes. A board without the constants, or whose JSON marks an MS5611, keeps `d1` and `d2`
//! as written.
//!
//! *Pressure, TeleMetrum v1.* The MP3H6115A's output is `Vout = Vs (0.009 P − 0.095)`, P in kPa
//! (Freescale, *MP3H6115A*, rev. 5.1, 2012, "Transfer Function", p. 5), read by the CC1111's
//! 12-bit converter, whose single-ended result, 0 to 2,047, reaches 2,047 at its reference and is
//! held in the top bits of its two result registers (TI, *CC1110Fx/CC1111Fx*, SWRS033H,
//! §12.10.2.6, p. 143, and `ADCH`/`ADCL`, p. 144): full scale is 2,047 × 16 = 32,752. So
//! `P = (count / 32,752 + 0.095) / 0.009` kPa, the converter's reference being the sensor's own
//! supply (the board's design, which no document states). Second check: the pad's pressure must
//! lie from 10 to 120 kPa, or the file is refused; the export of the same flight agrees within
//! half its 0.01 Pa print step (the survey's report).
//!
//! *Acceleration.* AltosUI calibrates the accelerometer with the board's antenna end up, then down
//! (the manual, §12.6.20, p. 42), giving its counts at +1 g and −1 g (`accel_cal_plus`,
//! `accel_cal_minus`, or the text form's `Accel cal +1g: P -1g: M`). The record's
//! `acceleration` is `(ground − count) / ((minus − plus) / 2) × 9.806 65` m/s², `ground` the
//! flight record's reading on the pad, 9.806 65 m/s² standard gravity (CGPM, 1901): 0 on the pad,
//! up the board's axis positive, gravity out. A board mounted antenna down keeps a flipped
//! calibration (the JSON's `_cooked` values) whose span is the same, so the result doesn't depend
//! on which pair is read. Without a calibration, with a span that isn't positive or without a
//! ground reading, the counts are kept as written, `accel`.
//!
//! *Not read.* The TeleMega v4's IMU (format 15): its one sample file's IMU never ran, so its
//! layout is unconfirmed. Any Mega whose flight record's IMU fields are all zero: its records'
//! IMU bytes are then stale buffer bytes, not readings. Bytes a record type leaves over from the
//! record before (the tails of state, sense, GPS and satellite records) are never read. A record
//! type the board's layout doesn't hold is counted in a note.
//!
//! *The flight's states.* AltosUI's exports write each state's number beside its name, and in
//! all 9 sample exports 2 is `pad`, 3 `boost`, 4 `fast`, 5 `coast`, 6 `drogue`, 7 `main`, 8
//! `landed`; the events take those names, another number `state N`.
//!
//! **What the file says about itself.** [`FlightLog::serial_number`] and
//! [`FlightLog::firmware`] are the configuration's; [`FlightLog::flight_number`] is the flight
//! record's, not the JSON's `stored_flight` or `flights`, which needn't name the flight the file
//! holds. [`Stated::figures`] holds the flight record's ground pressure (Pa) and ground
//! accelerometer reading, the calibration, the IMU's pad readings and the configuration's main
//! deployment height and apogee delay, as written, in SI where the text form names the unit.
//!
//! **The primary track.** The file holds no altitude, so [`FlightLog::time_s`] and
//! [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`; every field is a channel
//! on its records' clock, in tick order.
//!
//! An invented record, a TeleMetrum v2's (format 7) flight record in the JSON form: type `F`,
//! checksum, tick 0x0164 (356), flight 3, ground reading 25 counts, ground pressure 98,000 Pa, and
//! four unused bytes:
//!
//! ```text
//! 46 90 64 01 03 00 19 00 d0 7e 01 00 00 00 00 00
//! ```
//!
//! The same record in the text form, its checksum not printed:
//!
//! ```text
//! F 164 03 00 19 00 d0 7e 01 00 00 00 00 00
//! ```

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::error::LogError;
use crate::log::{Channel, Event, Figure, FlightLog, LogFormat, Quantity, Stated};

/// The format's name in errors.
const FORMAT: &str = "AltOS .eeprom";

/// The configuration's value for "not set".
const NOT_SET: i64 = 2_147_483_647;

/// What a JSON record's checksum adds to the sum of its bytes: 0x5A plus every byte, the checksum
/// byte among them, is 0 modulo 256.
const CHECKSUM_BASE: u8 = 0x5a;

/// The tick's turn: 2¹⁶ ticks.
const TURN: i64 = 1 << 16;

/// The ticks in a second: a time is the tick divided by it, correctly rounded.
const TICKS_PER_S: f64 = 100.0;

/// The farthest a record's tick may lie from the latest so far, ticks: 10 s. The slowest logging
/// rate is 10 a second (Appendix F) and a GPS record's tick lags at most 0.96 s in the samples.
const SILENCE_TICKS: i64 = 1_000;

/// Standard gravity, m/s² (CGPM, 1901).
const STANDARD_GRAVITY: f64 = 9.806_65;

/// The pressures a launch site's pad can have, Pa.
const PAD_PRESSURE_PA: std::ops::RangeInclusive<f64> = 10_000.0..=120_000.0;

/// How far the pad's pressure may lie from the flight record's ground pressure, as a share of it.
const GROUND_AGREES: f64 = 0.01;

/// The CC1111's full-scale count, its 12-bit result's 2,047 in the top bits of 16 (SWRS033H,
/// p. 143, p. 144).
const CC1111_FULL_SCALE: f64 = 32_752.0;

/// A Mega's sense divider and converter: 12 bits over 3.3 V, behind 100 kΩ over 27 kΩ (*AltOS
/// Telemetry*, p. 3).
const MEGA_SENSE_V: f64 = 3.3 / 4095.0 * (100.0 + 27.0) / 27.0;

/// The format's resolution ([`LogFormat::altitude_resolution_m`]): the file writes no altitude,
/// and its coarsest pressure is a TeleMetrum v1's, whose counts move by 4 in the sample files:
/// 4 / 32,752 / 0.009 kPa, 13.6 Pa, or 1.13 m near sea level (`Δh = Δp / (ρ₀ g₀)`). An MS5607
/// board's moves by a whole pascal.
pub const ALTITUDE_STEP_M: f64 =
    4.0 / CC1111_FULL_SCALE / 0.009 * 1000.0 / (1.225 * STANDARD_GRAVITY);

/// How many lines, from the first, the text form's marks are looked for in.
const HEAD_LINES: usize = 1_000;

/// The flight's states by number, as AltosUI's exports name them.
const STATES: [(u16, &str); 7] = [
    (2, "pad"),
    (3, "boost"),
    (4, "fast"),
    (5, "coast"),
    (6, "drogue"),
    (7, "main"),
    (8, "landed"),
];

/// The GPS flags' valid bit (*AltOS Telemetry*, Table 16, p. 7).
const GPS_VALID: u8 = 0x10;

/// A board's layout, by its log format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Board {
    /// Format 1: TeleMetrum v1, 8-byte records.
    TeleMetrumV1,
    /// Formats 6 and 14: EasyMini v1 and v2, 16-byte records.
    EasyMini,
    /// Format 7: TeleMetrum v2 and newer, 16-byte records.
    TeleMetrumV2,
    /// Formats 15, 16 and 22: TeleMega and EasyMega, 32-byte records.
    Mega,
}

/// The board and the record size of a log format; `None` for one the samples don't show.
fn layout(log_format: i64) -> Option<(Board, usize)> {
    match log_format {
        1 => Some((Board::TeleMetrumV1, 8)),
        6 | 14 => Some((Board::EasyMini, 16)),
        7 => Some((Board::TeleMetrumV2, 16)),
        15 | 16 | 22 => Some((Board::Mega, 32)),
        _ => None,
    }
}

/// Where in the file a record or a fault is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum At {
    /// A 1-based line of the text form, or of the JSON form's head.
    Line(usize),
    /// A 0-based byte of the JSON form's memory.
    Offset(usize),
}

fn refuse(at: At, message: String) -> LogError {
    match at {
        At::Line(line) => LogError::Syntax {
            format: FORMAT,
            line,
            message,
        },
        At::Offset(offset) => LogError::Binary {
            format: FORMAT,
            offset,
            message,
        },
    }
}

/// One record: its whole bytes, the checksum byte 0 in the text form.
#[derive(Debug, Clone)]
struct Record {
    at: At,
    bytes: Vec<u8>,
}

impl Record {
    fn kind(&self) -> u8 {
        self.bytes[0]
    }
    fn tick(&self) -> u16 {
        self.u16(2)
    }
    fn u8(&self, at: usize) -> f64 {
        f64::from(self.bytes[at])
    }
    fn u16(&self, at: usize) -> u16 {
        u16::from_le_bytes([self.bytes[at], self.bytes[at + 1]])
    }
    fn i16(&self, at: usize) -> i16 {
        i16::from_le_bytes([self.bytes[at], self.bytes[at + 1]])
    }
    fn u24(&self, at: usize) -> u32 {
        u32::from_le_bytes([self.bytes[at], self.bytes[at + 1], self.bytes[at + 2], 0])
    }
    fn u32(&self, at: usize) -> u32 {
        u32::from_le_bytes([
            self.bytes[at],
            self.bytes[at + 1],
            self.bytes[at + 2],
            self.bytes[at + 3],
        ])
    }
    fn i32(&self, at: usize) -> i32 {
        self.u32(at) as i32
    }
}

/// The barometer's constants C1 to C6 (*MS5607-02BA03*, p. 8).
type Prom = [i64; 6];

/// The barometer's constants' keys, C1 to C6, in the JSON's `ms5607` object and the text form's
/// `ms5607 KEY: N` lines, with the setting's name for a refusal.
const PROM_KEYS: [(&str, &str); 6] = [
    ("sens", "barometer's C1"),
    ("off", "barometer's C2"),
    ("tcs", "barometer's C3"),
    ("tco", "barometer's C4"),
    ("tref", "barometer's C5"),
    ("tempsens", "barometer's C6"),
];

/// The configuration, as read from either form.
#[derive(Debug, Default)]
struct Config {
    log_format: Option<i64>,
    product: Option<String>,
    serial: Option<i64>,
    version: Option<String>,
    prom: Option<Prom>,
    ms5611: bool,
    accel_plus: Option<i64>,
    accel_minus: Option<i64>,
    altitude_32: bool,
    main_deploy: Option<Figure>,
    apogee_delay: Option<Figure>,
}

/// Whether the text is an AltOS `.eeprom` file, in either form.
pub fn recognizes(text: &str) -> bool {
    let trimmed = text.trim_start();
    if trimmed.starts_with('{') {
        return json_head(text).is_some_and(|(head, _)| {
            head.contains_key("log_format") && head.get("product").is_some_and(Value::is_string)
        });
    }
    let (mut config, mut maker) = (false, false);
    for line in text.lines().take(HEAD_LINES).map(str::trim) {
        config |= line.starts_with("Config version:");
        maker |= value_of(line, "manufacturer") == Some("altusmetrum.org");
        if config && maker {
            return true;
        }
    }
    false
}

/// The JSON form's head, the object, and the byte where it ends; `None` if the text doesn't open
/// with a JSON object.
fn json_head(text: &str) -> Option<(Map<String, Value>, usize)> {
    let start = text.len() - text.trim_start().len();
    let mut stream =
        serde_json::Deserializer::from_str(&text[start..]).into_iter::<serde_json::Value>();
    match stream.next() {
        Some(Ok(Value::Object(head))) => Some((head, start + stream.byte_offset())),
        _ => None,
    }
}

/// A text-form configuration line's value after its key and whitespace, such as `product
/// TeleMetrum-v2.0`'s.
fn value_of<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    let value = rest.trim_start();
    (value.len() < rest.len() && !value.is_empty()).then_some(value)
}

/// Reads an AltOS `.eeprom` file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] for a file in neither form; [`LogError::Syntax`] for a log format
/// the reader doesn't know (named by number), a malformed line, a configuration whose copies
/// differ, a file with no flight record or two; [`LogError::Binary`] for a JSON record whose
/// checksum fails, a record after erased memory, or a pad pressure that isn't a launch site's.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "neither a JSON object naming `log_format` and `product` nor an AltOS \
                      board's printed configuration"
                .to_owned(),
        });
    }
    let raw = if text.trim_start().starts_with('{') {
        json_form(text)?
    } else {
        text_form(text)?
    };
    decode(raw)
}

/// What either form gives: the configuration, the records in the file's order, and the counts
/// the notes need.
#[derive(Debug)]
struct Raw {
    config: Config,
    board: Board,
    size: usize,
    log_format: i64,
    from_product: bool,
    json: bool,
    records: Vec<Record>,
    erased: usize,
    last_line_cut: bool,
}

/// The log format, the board and the record size from the configuration, the product's rule
/// for firmware before 1.0 included.
fn format_of(config: &Config, at: At) -> Result<(i64, Board, usize, bool), LogError> {
    let (log_format, from_product) = match config.log_format {
        Some(format) => (format, false),
        None if config
            .product
            .as_deref()
            .is_some_and(|product| product.starts_with("TeleMetrum-v1")) =>
        {
            (1, true)
        }
        None => {
            return Err(refuse(
                at,
                format!(
                    "the configuration states no log format, and its product, {:?}, isn't a \
                     TeleMetrum v1, whose firmware before 1.0 logged format 1",
                    config.product.as_deref().unwrap_or("none")
                ),
            ));
        }
    };
    let (board, size) = layout(log_format).ok_or_else(|| {
        refuse(
            at,
            format!(
                "log format {log_format}, which no sample file shows: its record size is \
                 unknown, and read at a wrong size the bytes would make a plausible flight"
            ),
        )
    })?;
    Ok((log_format, board, size, from_product))
}

/// The JSON form: the head, then the memory in hexadecimal.
fn json_form(text: &str) -> Result<Raw, LogError> {
    let (head, end) = json_head(text).ok_or_else(|| {
        refuse(
            At::Line(1),
            "the JSON object at the head doesn't parse".to_owned(),
        )
    })?;
    let int = |key: &str| {
        head.get(key)
            .and_then(Value::as_i64)
            .filter(|value| *value != NOT_SET)
    };
    let mut config = Config {
        log_format: int("log_format"),
        product: head
            .get("product")
            .and_then(Value::as_str)
            .map(str::to_owned),
        serial: int("serial"),
        version: head
            .get("version")
            .and_then(Value::as_str)
            .map(str::to_owned),
        accel_plus: int("accel_cal_plus").or(int("accel_cal_plus_cooked")),
        accel_minus: int("accel_cal_minus").or(int("accel_cal_minus_cooked")),
        altitude_32: int("altitude_32") == Some(1),
        main_deploy: int("main_deploy").map(|value| {
            Figure::new(
                "main_deploy",
                value.to_string(),
                Some(Quantity::Unrecognized),
                vec![value as f64],
            )
        }),
        apogee_delay: int("apogee_delay").map(|value| {
            Figure::new(
                "apogee_delay",
                value.to_string(),
                Some(Quantity::Unrecognized),
                vec![value as f64],
            )
        }),
        ..Config::default()
    };
    if let Some(Value::Object(ms5607)) = head.get("ms5607") {
        config.ms5611 = ms5607.get("ms5611").and_then(Value::as_bool) == Some(true);
        let constant = |key: &str| {
            ms5607
                .get(key)
                .and_then(Value::as_i64)
                .filter(|value| (0..=0xffff).contains(value))
        };
        if let [Some(c1), Some(c2), Some(c3), Some(c4), Some(c5), Some(c6)] =
            PROM_KEYS.map(|(key, _)| constant(key))
        {
            config.prom = Some([c1, c2, c3, c4, c5, c6]);
        }
    }
    let (log_format, board, size, from_product) = format_of(&config, At::Line(1))?;

    // The memory: two hexadecimal digits a byte.
    let first_line = text[..end].matches('\n').count() + 1;
    let mut bytes = Vec::with_capacity((text.len() - end) / 3);
    for (k, line) in text[end..].split('\n').enumerate() {
        for token in line.split_ascii_whitespace() {
            let byte = (token.len() == 2)
                .then(|| u8::from_str_radix(token, 16).ok())
                .flatten()
                .ok_or_else(|| {
                    refuse(
                        At::Line(first_line + k),
                        format!(
                            "{:?} where the memory's bytes are two hexadecimal digits each",
                            cut(token)
                        ),
                    )
                })?;
            bytes.push(byte);
        }
    }
    let mut records = Vec::new();
    let mut erased = 0usize;
    let mut first_erased: Option<usize> = None;
    for (k, chunk) in bytes.chunks(size).enumerate() {
        let offset = k * size;
        if chunk.iter().all(|byte| *byte == 0xff) {
            erased += 1;
            first_erased.get_or_insert(offset);
            continue;
        }
        if chunk.len() < size {
            return Err(refuse(
                At::Offset(offset),
                format!(
                    "the memory ends in {} bytes, part of a {size}-byte record",
                    chunk.len()
                ),
            ));
        }
        if let Some(erased_at) = first_erased {
            return Err(refuse(
                At::Offset(offset),
                format!(
                    "a record after the erased memory from byte {erased_at}: the memory's order \
                     is unknown"
                ),
            ));
        }
        let sum = chunk
            .iter()
            .fold(CHECKSUM_BASE, |sum, byte| sum.wrapping_add(*byte));
        if sum != 0 {
            return Err(refuse(
                At::Offset(offset),
                format!(
                    "record {} fails its checksum: 0x5A and its bytes sum to 0x{sum:02X}, not 0, \
                     modulo 256: the record is damaged",
                    k + 1
                ),
            ));
        }
        records.push(Record {
            at: At::Offset(offset),
            bytes: chunk.to_vec(),
        });
    }
    Ok(Raw {
        config,
        board,
        size,
        log_format,
        from_product,
        json: true,
        records,
        erased,
        last_line_cut: false,
    })
}

/// A text-form line's shape as a record: a type letter, a tick of 1 to 4 hexadecimal digits,
/// then at least one more token of them.
fn record_shaped(line: &str) -> bool {
    let mut tokens = line.split_ascii_whitespace();
    let hex = |token: &str, most: usize| {
        (1..=most).contains(&token.len()) && token.bytes().all(|b| b.is_ascii_hexdigit())
    };
    matches!(tokens.next(), Some(kind) if kind.len() == 1 && kind.as_bytes()[0].is_ascii_alphabetic())
        && tokens.next().is_some_and(|tick| hex(tick, 4))
        && tokens.next().is_some_and(|token| hex(token, 4))
}

/// A text-form record line read at the record size: `None` if it isn't one.
fn text_record(line: &str, size: usize) -> Option<Vec<u8>> {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
    let digits = |token: &str, least: usize, most: usize| {
        (least..=most).contains(&token.len()) && token.bytes().all(|b| b.is_ascii_hexdigit())
    };
    let (kind, tick, fields) = match tokens.as_slice() {
        [kind, tick, fields @ ..] => (*kind, *tick, fields),
        _ => return None,
    };
    if kind.len() != 1 || !kind.as_bytes()[0].is_ascii_alphabetic() || !digits(tick, 1, 4) {
        return None;
    }
    let tick = u16::from_str_radix(tick, 16).ok()?;
    let mut bytes = vec![kind.as_bytes()[0], 0];
    bytes.extend(tick.to_le_bytes());
    if size == 8 {
        // Format 1 prints its four field bytes as two 16-bit words.
        if fields.len() != 2 || !fields.iter().all(|word| digits(word, 1, 4)) {
            return None;
        }
        for word in fields {
            bytes.extend(u16::from_str_radix(word, 16).ok()?.to_le_bytes());
        }
    } else {
        if fields.len() != size - 4 || !fields.iter().all(|byte| digits(byte, 2, 2)) {
            return None;
        }
        for byte in fields {
            bytes.push(u8::from_str_radix(byte, 16).ok()?);
        }
    }
    Some(bytes)
}

/// The text form: the configuration as printed, then one record a line.
fn text_form(text: &str) -> Result<Raw, LogError> {
    let lines: Vec<&str> = text
        .split('\n')
        .map(|line| line.trim_end_matches('\r'))
        .collect();
    let start = lines
        .iter()
        .position(|line| record_shaped(line))
        .ok_or(LogError::NoData { format: FORMAT })?;

    let mut config = Config::default();
    // The first copy of each setting, and the line that set it, for a later copy to agree with.
    let mut seen: BTreeMap<&'static str, (String, usize)> = BTreeMap::new();
    let mut prom: [Option<i64>; 6] = [None; 6];
    for (k, line) in lines[..start].iter().enumerate() {
        let number = k + 1;
        let line = line.trim();
        let mut agree = |key: &'static str, value: String| -> Result<bool, LogError> {
            match seen.get(key) {
                Some((first, at)) if *first != value => Err(refuse(
                    At::Line(number),
                    format!(
                        "the configuration is printed more than once, and its {key} here, \
                         {value:?}, differs from line {at}'s, {first:?}"
                    ),
                )),
                Some(_) => Ok(false),
                None => {
                    seen.insert(key, (value, number));
                    Ok(true)
                }
            }
        };
        if let Some(value) = value_of(line, "product") {
            if agree("product", value.to_owned())? {
                config.product = Some(value.to_owned());
            }
        } else if let Some(value) = value_of(line, "serial-number") {
            if agree("serial number", value.to_owned())? {
                config.serial = value.parse().ok();
            }
        } else if let Some(value) = value_of(line, "log-format") {
            if agree("log format", value.to_owned())? {
                config.log_format =
                    Some(value.parse().map_err(|_| {
                        refuse(At::Line(number), format!("a log format of {value:?}"))
                    })?);
            }
        } else if let Some(value) = value_of(line, "software-version") {
            if agree("firmware", value.to_owned())? {
                config.version = Some(value.to_owned());
            }
        } else if let Some(value) = value_of(line, "altitude-32") {
            if agree("altitude-32", value.to_owned())? {
                config.altitude_32 = value == "1";
            }
        } else if let Some(rest) = line.strip_prefix("Accel cal +1g:") {
            if agree("accelerometer calibration", rest.trim().to_owned())? {
                let words: Vec<&str> = rest.split_whitespace().collect();
                if let [plus, "-1g:", minus, ..] = words.as_slice() {
                    config.accel_plus = plus.parse().ok();
                    config.accel_minus = minus.parse().ok();
                }
            }
        } else if let Some(rest) = line.strip_prefix("ms5607 ") {
            let constant = rest.split_once(':').and_then(|(key, value)| {
                let index = PROM_KEYS.iter().position(|(name, _)| *name == key.trim())?;
                Some((index, value.trim()))
            });
            if let Some((index, value)) = constant
                && agree(PROM_KEYS[index].1, value.to_owned())?
            {
                prom[index] = value.parse().ok().filter(|c| (0..=0xffff).contains(c));
            }
        } else if let Some(rest) = line.strip_prefix("Main deploy:")
            && config.main_deploy.is_none()
        {
            config.main_deploy = Some(stated_setting(
                "main_deploy",
                rest.trim(),
                "meters",
                Quantity::AltitudeAboveGround,
            ));
        } else if let Some(rest) = line.strip_prefix("Apogee delay:")
            && config.apogee_delay.is_none()
        {
            config.apogee_delay = Some(stated_setting(
                "apogee_delay",
                rest.trim(),
                "seconds",
                Quantity::Duration,
            ));
        }
    }
    if let [Some(c1), Some(c2), Some(c3), Some(c4), Some(c5), Some(c6)] = prom {
        config.prom = Some([c1, c2, c3, c4, c5, c6]);
    }
    let (log_format, board, size, from_product) = format_of(&config, At::Line(start + 1))?;

    // A file that ends without a line break may end mid-line, however well that line reads.
    let ends_cut = !text.ends_with('\n');
    let last = lines.len() - 1;
    let mut records = Vec::new();
    let mut last_line_cut = false;
    for (k, line) in lines.iter().enumerate().skip(start) {
        if line.trim().is_empty() {
            continue;
        }
        if k == last && ends_cut {
            last_line_cut = true;
            continue;
        }
        let bytes = text_record(line, size).ok_or_else(|| {
            refuse(
                At::Line(k + 1),
                format!(
                    "{:?} isn't a record of log format {log_format}: a type letter, the tick and \
                     {}",
                    cut(line),
                    if size == 8 {
                        "two 16-bit words in hexadecimal".to_owned()
                    } else {
                        format!("{} bytes in hexadecimal", size - 4)
                    }
                ),
            )
        })?;
        records.push(Record {
            at: At::Line(k + 1),
            bytes,
        });
    }
    Ok(Raw {
        config,
        board,
        size,
        log_format,
        from_product,
        json: false,
        records,
        erased: 0,
        last_line_cut,
    })
}

/// A setting the text form prints with its unit, such as `Main deploy: 250 meters`: in SI where
/// the word after the number is `unit`, kept as written otherwise.
fn stated_setting(name: &str, written: &str, unit: &str, quantity: Quantity) -> Figure {
    let mut words = written.split_whitespace();
    let value = words
        .next()
        .and_then(|word| word.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    match (value, words.next()) {
        (Some(value), Some(word)) if word == unit => {
            Figure::new(name, written, Some(quantity), vec![value])
        }
        (Some(value), _) => Figure::new(name, written, Some(Quantity::Unrecognized), vec![value]),
        _ => Figure::new(name, written, None, Vec::new()),
    }
}

/// Each record's tick on the turn nearest the latest so far, and how many records come before
/// the first one more than [`SILENCE_TICKS`] from it, which ends the flight.
fn unwind(records: &[Record]) -> (Vec<i64>, usize) {
    let mut latest: Option<i64> = None;
    let mut ticks = Vec::with_capacity(records.len());
    for (k, record) in records.iter().enumerate() {
        let tick = match latest {
            None => i64::from(record.tick()),
            Some(latest) => {
                // The step to the nearest turn, from −32,768 to 32,767 ticks.
                let step =
                    i64::from(record.tick().wrapping_sub(latest.rem_euclid(TURN) as u16) as i16);
                if step.abs() > SILENCE_TICKS {
                    return (ticks, k);
                }
                latest + step
            }
        };
        latest = Some(latest.map_or(tick, |latest| latest.max(tick)));
        ticks.push(tick);
    }
    let kept = ticks.len();
    (ticks, kept)
}

/// The MS5607's pressure, Pa, and temperature, hundredths of a °C, from its counts D1 and D2 and
/// its constants C1 to C6: the data sheet's first-order compensation (*MS5607-02BA03*, Figure 2,
/// p. 8) and, below 20 °C, its second order (Figure 3, p. 9), every division by 2ⁿ an arithmetic
/// shift, a floor. In 128-bit integers, which no 32-bit count or 16-bit constant can overflow.
fn ms5607(d1: u32, d2: u32, prom: &Prom) -> (i128, i128) {
    let [c1, c2, c3, c4, c5, c6] = prom.map(i128::from);
    let (d1, d2) = (i128::from(d1), i128::from(d2));
    let dt = d2 - (c5 << 8);
    let mut temp = 2000 + ((dt * c6) >> 23);
    let mut off = (c2 << 17) + ((c4 * dt) >> 6);
    let mut sens = (c1 << 16) + ((c3 * dt) >> 7);
    if temp < 2000 {
        let t2 = (dt * dt) >> 31;
        let mut off2 = (61 * (temp - 2000) * (temp - 2000)) >> 4;
        let mut sens2 = 2 * (temp - 2000) * (temp - 2000);
        if temp < -1500 {
            off2 += 15 * (temp + 1500) * (temp + 1500);
            sens2 += 8 * (temp + 1500) * (temp + 1500);
        }
        temp -= t2;
        off -= off2;
        sens -= sens2;
    }
    let pressure = (((d1 * sens) >> 21) - off) >> 15;
    (pressure, temp)
}

/// The TeleMetrum v1's pressure, Pa, from its count: the MP3H6115A's transfer function over the
/// CC1111's full scale.
fn mp3h6115a_pa(count: i16) -> f64 {
    (f64::from(count) / CC1111_FULL_SCALE + 0.095) / 0.009 * 1000.0
}

/// The channels being built, by name, in the order first met.
#[derive(Debug, Default)]
struct Channels {
    order: Vec<(String, Quantity)>,
    data: BTreeMap<String, (Vec<f64>, Vec<f64>)>,
}

impl Channels {
    fn push(&mut self, name: &str, quantity: Quantity, time_s: f64, value: f64) {
        if let Some((times, values)) = self.data.get_mut(name) {
            times.push(time_s);
            values.push(value);
            return;
        }
        self.order.push((name.to_owned(), quantity));
        self.data
            .insert(name.to_owned(), (vec![time_s], vec![value]));
    }

    fn get(&self, name: &str) -> Option<&(Vec<f64>, Vec<f64>)> {
        self.data.get(name)
    }

    fn into_channels(mut self) -> Vec<Channel> {
        self.order
            .into_iter()
            .filter_map(|(column, quantity)| {
                let (time_s, values) = self.data.remove(&column)?;
                Some(Channel {
                    column,
                    quantity,
                    time_s,
                    values,
                })
            })
            .collect()
    }
}

/// The flight record's fields.
#[derive(Debug, Default)]
struct Flight {
    number: u16,
    ground_accel: Option<i64>,
    ground_pressure: Option<u32>,
    imu_accel: [i16; 3],
    gyro_sums: [i32; 3],
}

/// What the accelerometer's counts become: m/s² from the zero and the counts a g, or as written
/// for the reason given.
#[derive(Debug, Clone, PartialEq)]
enum Accel {
    Converted { zero: f64, per_g: f64 },
    AsWritten(&'static str),
}

/// From the records to the flight record.
fn decode(raw: Raw) -> Result<FlightLog, LogError> {
    let Raw {
        config,
        board,
        size,
        log_format,
        from_product,
        json,
        records,
        erased,
        last_line_cut,
    } = raw;
    let read_in_file = records.len();
    let (ticks, kept) = unwind(&records);
    let after_silence = read_in_file - kept;

    // Exact repeats: the same bytes at the same tick.
    let mut seen: BTreeSet<(i64, &[u8])> = BTreeSet::new();
    let mut order: Vec<usize> = Vec::with_capacity(kept);
    let mut duplicates = 0usize;
    for (k, tick) in ticks.iter().enumerate() {
        if seen.insert((*tick, records[k].bytes.as_slice())) {
            order.push(k);
        } else {
            duplicates += 1;
        }
    }
    // Tick order, the file's order among equal ticks (the sort is stable).
    order.sort_by_key(|&k| ticks[k]);

    let flights: Vec<usize> = order
        .iter()
        .copied()
        .filter(|&k| records[k].kind() == b'F')
        .collect();
    let zero = match flights.as_slice() {
        [] => {
            return Err(refuse(
                records.first().map_or(At::Line(1), |record| record.at),
                "no flight record (`F`), whose tick is the flight's zero".to_owned(),
            ));
        }
        [only] => *only,
        [first, second, ..] => {
            return Err(refuse(
                records[*second].at,
                format!(
                    "a second flight record, after the one at {}: two flights in one file",
                    describe(records[*first].at)
                ),
            ));
        }
    };
    let zero_tick = ticks[zero];
    let f = &records[zero];
    let flight = match board {
        Board::TeleMetrumV1 => Flight {
            number: f.u16(6),
            ground_accel: Some(i64::from(f.u16(4))),
            ..Flight::default()
        },
        Board::EasyMini => Flight {
            number: f.u16(4),
            ground_pressure: Some(f.u32(8)),
            ..Flight::default()
        },
        Board::TeleMetrumV2 => Flight {
            number: f.u16(4),
            ground_accel: Some(i64::from(f.i16(6))),
            ground_pressure: Some(f.u32(8)),
            ..Flight::default()
        },
        Board::Mega => Flight {
            number: f.u16(4),
            ground_accel: Some(i64::from(f.i16(6))),
            ground_pressure: Some(f.u32(8)),
            imu_accel: [f.i16(12), f.i16(14), f.i16(16)],
            gyro_sums: [f.i32(20), f.i32(24), f.i32(28)],
        },
    };

    let accel = match (
        board,
        config.accel_plus,
        config.accel_minus,
        flight.ground_accel,
    ) {
        (Board::EasyMini, ..) => Accel::AsWritten("the board has no accelerometer"),
        (_, Some(plus), Some(minus), Some(ground)) if minus > plus => Accel::Converted {
            zero: ground as f64,
            per_g: (minus - plus) as f64 / 2.0,
        },
        (_, Some(_), Some(_), Some(_)) => Accel::AsWritten("the calibration's span isn't positive"),
        (_, _, _, Some(_)) => Accel::AsWritten("the calibration isn't set"),
        _ => Accel::AsWritten("the flight record holds no ground reading"),
    };
    let prom = match board {
        Board::TeleMetrumV1 => None,
        _ if config.ms5611 => None,
        _ => config.prom,
    };
    let imu_read = match board {
        Board::Mega => {
            log_format != 15 && (flight.imu_accel != [0; 3] || flight.gyro_sums != [0; 3])
        }
        _ => false,
    };

    // The GPS fixes' validity, by tick, for boards whose position records carry no flags.
    let valid_at: BTreeSet<i64> = order
        .iter()
        .filter(|&&k| {
            let record = &records[k];
            record.kind() == b'G'
                && matches!(board, Board::TeleMetrumV1 | Board::TeleMetrumV2)
                && record.bytes[7] & GPS_VALID != 0
        })
        .map(|&k| ticks[k])
        .collect();

    let mut channels = Channels::default();
    let mut events: Vec<Event> = Vec::new();
    let mut state: Option<u16> = None;
    let mut not_read: BTreeMap<u8, usize> = BTreeMap::new();
    let mut no_fix = 0usize;
    let mut sense_counts_off = 0usize;
    let mut satellites_cut = 0usize;
    let mut before_boost = 0usize;
    for &k in &order {
        let record = &records[k];
        let time = (ticks[k] - zero_tick) as f64 / TICKS_PER_S;
        if time < 0.0 {
            before_boost += 1;
        }
        let mut push = |name: &str, quantity: Quantity, value: f64| {
            channels.push(name, quantity, time, value);
        };
        let accelerometer = |push: &mut dyn FnMut(&str, Quantity, f64), count: i64| match &accel {
            Accel::Converted { zero, per_g } => push(
                "acceleration",
                Quantity::Acceleration,
                (zero - count as f64) / per_g * STANDARD_GRAVITY,
            ),
            Accel::AsWritten(_) => push("accel", Quantity::Unrecognized, count as f64),
        };
        let barometer = |push: &mut dyn FnMut(&str, Quantity, f64), d1: u32, d2: u32| match &prom {
            Some(prom) => {
                let (pa, centi) = ms5607(d1, d2, prom);
                push("pressure", Quantity::Pressure, pa as f64);
                push(
                    "temperature",
                    Quantity::Temperature,
                    centi as f64 / 100.0 + 273.15,
                );
            }
            None => {
                push("d1", Quantity::Unrecognized, f64::from(d1));
                push("d2", Quantity::Unrecognized, f64::from(d2));
            }
        };
        let mut position = |push: &mut dyn FnMut(&str, Quantity, f64),
                            valid: bool,
                            lat: Option<i32>,
                            lon: Option<i32>,
                            alt: Option<f64>| {
            if !valid {
                no_fix += 1;
            }
            let fix = |value: f64| if valid { value } else { f64::NAN };
            if let Some(lat) = lat {
                push("latitude", Quantity::Latitude, fix(f64::from(lat) / 1e7));
            }
            if let Some(lon) = lon {
                push("longitude", Quantity::Longitude, fix(f64::from(lon) / 1e7));
            }
            if let Some(alt) = alt {
                push("gps_altitude", Quantity::GnssAltitude, fix(alt));
            }
        };
        let altitude = |low: usize, high: usize| {
            if config.altitude_32 {
                f64::from(record.i16(high)) * 65_536.0 + f64::from(record.u16(low))
            } else {
                f64::from(record.i16(low))
            }
        };
        match (board, record.kind()) {
            (_, b'F') => {}
            (_, b'S') => {
                let now = record.u16(4);
                if state != Some(now) {
                    events.push(Event {
                        time_s: time,
                        name: state_name(now),
                    });
                    state = Some(now);
                }
                push("state", Quantity::Number, f64::from(now));
            }
            (Board::TeleMetrumV1, b'A') => {
                accelerometer(&mut push, i64::from(record.i16(4)));
                push("pressure", Quantity::Pressure, mp3h6115a_pa(record.i16(6)));
            }
            (Board::TeleMetrumV1, b'T') => {
                push("temp", Quantity::Unrecognized, f64::from(record.i16(4)));
                push("v_batt", Quantity::Unrecognized, f64::from(record.i16(6)));
            }
            (Board::TeleMetrumV1, b'D') => {
                push("sense_a", Quantity::Unrecognized, f64::from(record.i16(4)));
                push("sense_m", Quantity::Unrecognized, f64::from(record.i16(6)));
            }
            (Board::TeleMetrumV1 | Board::TeleMetrumV2, b'G') => {
                let flags = record.bytes[7];
                for (j, name) in ["hour", "minute", "second"].into_iter().enumerate() {
                    push(name, Quantity::Number, record.u8(4 + j));
                }
                push("gps_flags", Quantity::Number, f64::from(flags));
                push("nsat", Quantity::Number, f64::from(flags & 0x0f));
                if board == Board::TeleMetrumV2 {
                    for (j, name) in ["year", "month", "day"].into_iter().enumerate() {
                        push(name, Quantity::Number, record.u8(8 + j));
                    }
                    push("gps_byte_11", Quantity::Unrecognized, record.u8(11));
                }
            }
            (Board::TeleMetrumV1, b'N') => {
                let valid = valid_at.contains(&ticks[k]);
                position(&mut push, valid, Some(record.i32(4)), None, None);
            }
            (Board::TeleMetrumV1, b'W') => {
                let valid = valid_at.contains(&ticks[k]);
                position(&mut push, valid, None, Some(record.i32(4)), None);
            }
            (Board::TeleMetrumV1, b'H') => {
                let valid = valid_at.contains(&ticks[k]);
                position(&mut push, valid, None, None, Some(f64::from(record.i16(4))));
            }
            (Board::TeleMetrumV1, b'Y') => {
                for (j, name) in ["year", "month", "day"].into_iter().enumerate() {
                    push(name, Quantity::Number, record.u8(4 + j));
                }
            }
            (Board::TeleMetrumV1, b'V') => {
                push("svid", Quantity::Number, record.u8(4));
                push("c_n", Quantity::Number, record.u8(7));
            }
            (Board::EasyMini, b'A') => {
                barometer(&mut push, record.u24(4), record.u24(7));
                push("sense_a", Quantity::Unrecognized, f64::from(record.u16(10)));
                push("sense_m", Quantity::Unrecognized, f64::from(record.u16(12)));
                push("v_batt", Quantity::Unrecognized, f64::from(record.u16(14)));
            }
            (Board::TeleMetrumV2, b'A') => {
                barometer(&mut push, record.u32(4), record.u32(8));
                accelerometer(&mut push, i64::from(record.i16(12)));
            }
            (Board::TeleMetrumV2, b'T') => {
                push("v_batt", Quantity::Unrecognized, f64::from(record.i16(4)));
                push("sense_a", Quantity::Unrecognized, f64::from(record.i16(6)));
                push("sense_m", Quantity::Unrecognized, f64::from(record.i16(8)));
            }
            (Board::TeleMetrumV2, b'P') => {
                let valid = valid_at.contains(&ticks[k]);
                position(
                    &mut push,
                    valid,
                    Some(record.i32(4)),
                    Some(record.i32(8)),
                    Some(altitude(12, 14)),
                );
            }
            (Board::TeleMetrumV2 | Board::Mega, b'V') => {
                let count = usize::from(record.bytes[4]);
                let room = (size - 6) / 2;
                if count > room {
                    satellites_cut += 1;
                }
                push("sat_count", Quantity::Number, f64::from(record.bytes[4]));
                for j in 0..count.min(room) {
                    push("svid", Quantity::Number, record.u8(6 + 2 * j));
                    push("c_n", Quantity::Number, record.u8(7 + 2 * j));
                }
            }
            (Board::Mega, b'A') => {
                barometer(&mut push, record.u32(4), record.u32(8));
                if imu_read {
                    const IMU: [&str; 9] = [
                        "imu_accel_0",
                        "imu_accel_1",
                        "imu_accel_2",
                        "imu_gyro_0",
                        "imu_gyro_1",
                        "imu_gyro_2",
                        "imu_mag_0",
                        "imu_mag_1",
                        "imu_mag_2",
                    ];
                    for (j, name) in IMU.into_iter().enumerate() {
                        push(
                            name,
                            Quantity::Unrecognized,
                            f64::from(record.i16(12 + 2 * j)),
                        );
                    }
                }
                accelerometer(&mut push, i64::from(record.i16(30)));
            }
            (Board::Mega, b'T') => {
                push("v_batt", Quantity::Unrecognized, f64::from(record.i16(4)));
                push(
                    "pyro",
                    Quantity::Voltage,
                    f64::from(record.i16(6)) * MEGA_SENSE_V,
                );
                let senses = record.i16(8);
                push("n_sense", Quantity::Number, f64::from(senses));
                if senses == 6 {
                    const SENSES: [&str; 6] = [
                        "igniter_a",
                        "igniter_b",
                        "igniter_c",
                        "igniter_d",
                        "drogue_voltage",
                        "main_voltage",
                    ];
                    for (j, name) in SENSES.into_iter().enumerate() {
                        push(
                            name,
                            Quantity::Voltage,
                            f64::from(record.i16(10 + 2 * j)) * MEGA_SENSE_V,
                        );
                    }
                } else {
                    sense_counts_off += 1;
                    for j in 0..6 {
                        push(
                            [
                                "sense_0", "sense_1", "sense_2", "sense_3", "sense_4", "sense_5",
                            ][j],
                            Quantity::Unrecognized,
                            f64::from(record.i16(10 + 2 * j)),
                        );
                    }
                }
            }
            (Board::Mega, b'G') => {
                let flags = record.bytes[17];
                position(
                    &mut push,
                    flags & GPS_VALID != 0,
                    Some(record.i32(4)),
                    Some(record.i32(8)),
                    Some(altitude(12, 30)),
                );
                for (j, name) in ["hour", "minute", "second"].into_iter().enumerate() {
                    push(name, Quantity::Number, record.u8(14 + j));
                }
                push("gps_flags", Quantity::Number, f64::from(flags));
                push("nsat", Quantity::Number, f64::from(flags & 0x0f));
                for (j, name) in ["year", "month", "day"].into_iter().enumerate() {
                    push(name, Quantity::Number, record.u8(18 + j));
                }
                push("course", Quantity::Unrecognized, record.u8(21));
                push(
                    "ground_speed",
                    Quantity::Unrecognized,
                    f64::from(record.u16(22)),
                );
                push(
                    "climb_rate",
                    Quantity::Unrecognized,
                    f64::from(record.i16(24)),
                );
                for (j, name) in ["pdop", "hdop", "vdop", "gps_mode"].into_iter().enumerate() {
                    push(name, Quantity::Number, record.u8(26 + j));
                }
            }
            (_, kind) => *not_read.entry(kind).or_default() += 1,
        }
    }
    if channels.data.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    // The second check of the pressure's unit: the pad's.
    let pad = channels.get("pressure").and_then(|(times, values)| {
        let mut before: Vec<f64> = times
            .iter()
            .zip(values)
            .filter(|(time, value)| **time < 0.0 && value.is_finite())
            .map(|(_, value)| *value)
            .collect();
        crate::filter::median(&mut before).or_else(|| values.first().copied())
    });
    let mut pad_note = None;
    if let Some(pad) = pad {
        if !PAD_PRESSURE_PA.contains(&pad) {
            return Err(refuse(
                f.at,
                format!(
                    "the pad's pressure reads {pad:.0} Pa, outside every launch site's 10 to \
                     120 kPa: the records are misread or damaged"
                ),
            ));
        }
        match flight.ground_pressure {
            Some(ground) => {
                let ground = f64::from(ground);
                let off = (pad - ground).abs() / ground;
                // A NaN (a ground pressure of 0) fails the check, as it must.
                if off.is_nan() || off > GROUND_AGREES {
                    return Err(refuse(
                        f.at,
                        format!(
                            "the pad's pressure reads {pad:.0} Pa, {:.1}% from the flight \
                             record's ground pressure, {ground:.0} Pa: the records are misread or \
                             damaged",
                            100.0 * off
                        ),
                    ));
                }
                pad_note = Some(format!(
                    "`pressure` and `temperature` from the MS5607's counts by its data sheet's \
                     compensation, first and second order: the pad's pressure, {pad:.0} Pa, lies \
                     from 10 to 120 kPa and within {:.2}% of the flight record's ground pressure, \
                     {ground:.0} Pa",
                    100.0 * off
                ));
            }
            None => {
                pad_note = Some(format!(
                    "`pressure` from the MP3H6115A's transfer function over the CC1111's full \
                     scale, 32,752: the pad's pressure, {pad:.0} Pa, lies from 10 to 120 kPa"
                ));
            }
        }
    }

    let mut notes = Vec::new();
    notes.push(if json {
        format!(
            "{read_in_file} records of {size} bytes, log format {log_format}, from the JSON form: \
             every record's checksum checked, all pass"
        )
    } else {
        format!(
            "{read_in_file} records of {size} bytes, log format {log_format}, from the text form, \
             which prints no checksum"
        )
    });
    if from_product {
        notes.push(format!(
            "the configuration states no log format, as firmware before 1.0 didn't: format 1, \
             from its product, {}",
            config.product.as_deref().unwrap_or("")
        ));
    }
    if erased > 0 {
        notes.push(format!(
            "{erased} erased records (every byte 0xFF) after the last, skipped"
        ));
    }
    if last_line_cut {
        notes.push(
            "the file ends without a line break, so its last line may be cut short: left out"
                .to_owned(),
        );
    }
    if after_silence > 0 {
        notes.push(format!(
            "{after_silence} records from the first more than 10 s from the latest tick on are \
             left out: a silence no logging rate leaves, so not this flight's"
        ));
    }
    if duplicates > 0 {
        notes.push(format!(
            "{duplicates} records repeated exactly, the same bytes at the same tick, dropped"
        ));
    }
    notes.push(format!(
        "the clock is the board's tick, 10 ms, from the flight record's at tick {zero_tick} \
         (boost detection), each tick on the turn nearest the latest so far, from the file's \
         first record; the records in tick order, as the file's isn't; {before_boost} records \
         before boost, at negative times"
    ));
    if let Some(note) = pad_note {
        notes.push(note);
    }
    if prom.is_none() && board != Board::TeleMetrumV1 {
        notes.push(
            if config.ms5611 {
                "`d1` and `d2` kept as written: the configuration marks the barometer an MS5611, \
                 whose compensation isn't the MS5607's"
            } else {
                "`d1` and `d2` kept as written: the configuration holds no MS5607 constants"
            }
            .to_owned(),
        );
    }
    match &accel {
        Accel::Converted { zero, per_g } => notes.push(format!(
            "`acceleration` along the board's axis, m/s², from the high-g accelerometer's counts: \
             0 at the flight record's ground reading, {zero:.0}, and {per_g} counts a g from the \
             calibration's +1 g and −1 g, {} and {}",
            config.accel_plus.unwrap_or(0),
            config.accel_minus.unwrap_or(0)
        )),
        Accel::AsWritten(why) if board != Board::EasyMini => {
            notes.push(format!("`accel` kept as written: {why}"));
        }
        Accel::AsWritten(_) => {}
    }
    let kept: Vec<&str> = [
        "v_batt",
        "sense_a",
        "sense_m",
        "temp",
        "imu_accel_0",
        "gps_byte_11",
    ]
    .into_iter()
    .filter(|name| channels.get(name).is_some())
    .collect();
    if !kept.is_empty() {
        notes.push(format!(
            "kept as written, counts with no cited conversion: {}",
            kept.iter()
                .map(|name| match *name {
                    "imu_accel_0" => "the IMU's nine words".to_owned(),
                    "temp" => "`temp` (the CC1111's die temperature)".to_owned(),
                    "gps_byte_11" =>
                        "`gps_byte_11` (a dilution in some firmware, a stale byte in others)"
                            .to_owned(),
                    name => format!("`{name}`"),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if channels.get("pyro").is_some() {
        notes.push(
            "`pyro`, the igniters' and the apogee and main senses in V, as AltOS Telemetry \
             converts a 15 V Mega's: count / 4095 × 3.3 V × 127 / 27"
                .to_owned(),
        );
    }
    if sense_counts_off > 0 {
        notes.push(format!(
            "{sense_counts_off} sense records name a count of senses other than 6: their six \
             words kept as written, `sense_0` to `sense_5`"
        ));
    }
    if board == Board::Mega && !imu_read {
        notes.push(
            if log_format == 15 {
                "the IMU not read: format 15's layout of it is unconfirmed"
            } else {
                "the IMU not read: the flight record's IMU fields are all zero, so the records' \
                 IMU bytes are stale"
            }
            .to_owned(),
        );
    }
    if channels.get("gps_flags").is_some() {
        notes.push(
            "GPS: the time, date (two digits, and not reliably the flight's), dilutions (ten \
             times their value), course, speed, climb, satellite counts and signals kept as \
             written"
                .to_owned(),
        );
    }
    if no_fix > 0 {
        notes.push(format!(
            "{no_fix} GPS position records without a valid fix: their positions read as gaps"
        ));
    }
    if satellites_cut > 0 {
        notes.push(format!(
            "{satellites_cut} satellite records name more satellites than they hold: those that \
             fit are read"
        ));
    }
    for (kind, count) in &not_read {
        notes.push(format!(
            "{count} records of type {} not read: the board's layout holds no such record",
            if kind.is_ascii_graphic() {
                format!("`{}`", char::from(*kind))
            } else {
                format!("0x{kind:02X}")
            }
        ));
    }

    let mut figures = Vec::new();
    if let Some(ground) = flight.ground_pressure {
        figures.push(Figure::new(
            "ground_pressure",
            ground.to_string(),
            Some(Quantity::Pressure),
            vec![f64::from(ground)],
        ));
    }
    if let Some(ground) = flight.ground_accel {
        figures.push(Figure::new(
            "ground_accel",
            ground.to_string(),
            Some(Quantity::Unrecognized),
            vec![ground as f64],
        ));
    }
    if board != Board::EasyMini {
        for (name, value) in [
            ("accel_cal_plus", config.accel_plus),
            ("accel_cal_minus", config.accel_minus),
        ] {
            if let Some(value) = value {
                figures.push(Figure::new(
                    name,
                    value.to_string(),
                    Some(Quantity::Unrecognized),
                    vec![value as f64],
                ));
            }
        }
    }
    if imu_read {
        let accel: Vec<f64> = flight.imu_accel.iter().map(|v| f64::from(*v)).collect();
        let gyro: Vec<f64> = flight.gyro_sums.iter().map(|v| f64::from(*v)).collect();
        figures.push(Figure::new(
            "ground_imu_accel",
            join(&accel),
            Some(Quantity::Unrecognized),
            accel,
        ));
        figures.push(Figure::new(
            "ground_gyro_sums",
            join(&gyro),
            Some(Quantity::Unrecognized),
            gyro,
        ));
    }
    figures.extend(config.main_deploy);
    figures.extend(config.apogee_delay);

    Ok(FlightLog {
        format: LogFormat::AltosEeprom,
        logger: match &config.product {
            Some(product) => format!("AltOS {product}"),
            None => "AltOS".to_owned(),
        },
        serial_number: config.serial.map(|serial| serial.to_string()),
        firmware: config.version,
        flight_number: Some(u32::from(flight.number)),
        stated: Stated {
            figures,
            ..Stated::default()
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: channels.into_channels(),
        events,
        notes,
    })
}

/// Numbers joined by commas, as a figure writes a vector.
fn join(values: &[f64]) -> String {
    values
        .iter()
        .map(f64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// Where a record sits, in words.
fn describe(at: At) -> String {
    match at {
        At::Line(line) => format!("line {line}"),
        At::Offset(offset) => format!("byte {offset}"),
    }
}

/// A state's name, as AltosUI's exports write it, or `state N`.
fn state_name(number: u16) -> String {
    STATES
        .iter()
        .find(|(known, _)| *known == number)
        .map_or_else(|| format!("state {number}"), |(_, name)| (*name).to_owned())
}

/// Text cut to 80 characters, for an error.
fn cut(text: &str) -> String {
    text.chars().take(80).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The data sheet's own example constants, C1 to C6 (*MS5607-02BA03*, Figure 2, p. 8).
    const EXAMPLE_PROM: Prom = [46_372, 43_981, 29_059, 27_842, 31_553, 28_165];

    /// The data sheet's example counts, D1 and D2: 1,100.02 mbar at 20.00 °C.
    const EXAMPLE_D1: u32 = 6_465_444;
    const EXAMPLE_D2: u32 = 8_077_636;

    /// An invented TeleMetrum v2's configuration in the JSON form: the data sheet's example
    /// constants, and a calibration of 20 counts a g.
    const HEAD_V2: &str = r#"{
        "accel_cal_minus": 1040, "accel_cal_plus": 1000, "altitude_32": 1, "apogee_delay": 0,
        "flight": 2147483647, "log_format": 7, "main_deploy": 250,
        "ms5607": {"ms5611": false, "off": 43981, "sens": 46372, "tco": 27842, "tcs": 29059,
                   "tempsens": 28165, "tref": 31553},
        "product": "TeleMetrum-v2.0", "serial": 1234, "stored_flight": 9, "version": "1.9.22"
    }"#;

    /// The same configuration as the text form prints it.
    const TEXT_HEAD_V2: &[&str] = &[
        "~",
        "> E 0",
        "Config version: 1.27",
        "Main deploy: 250 meters",
        "Apogee delay: 0 seconds",
        "Accel cal +1g: 1000 -1g: 1040",
        "ms5607 reserved: 0",
        "ms5607 sens: 46372",
        "ms5607 off: 43981",
        "ms5607 tcs: 29059",
        "ms5607 tco: 27842",
        "ms5607 tref: 31553",
        "ms5607 tempsens: 28165",
        "manufacturer     altusmetrum.org",
        "product          TeleMetrum-v2.0",
        "serial-number    1234",
        "log-format       7",
        "altitude-32      1",
        "software-version 1.9.22",
        "done",
    ];

    /// A record of `size` bytes: its type, its checksum, its tick and its fields, padded with
    /// zeros, the checksum making 0x5A and every byte sum to 0 modulo 256.
    fn record(size: usize, kind: u8, tick: u16, fields: &[u8]) -> Vec<u8> {
        let mut bytes = vec![kind, 0];
        bytes.extend(tick.to_le_bytes());
        bytes.extend(fields);
        bytes.resize(size, 0);
        let sum = bytes
            .iter()
            .fold(CHECKSUM_BASE, |sum, byte| sum.wrapping_add(*byte));
        bytes[1] = 0u8.wrapping_sub(sum);
        bytes
    }

    /// A TeleMetrum v2's flight record: flight, ground reading, ground pressure.
    fn f7(tick: u16, flight: u16, ground_accel: i16, ground_pa: u32) -> Vec<u8> {
        let mut fields = flight.to_le_bytes().to_vec();
        fields.extend(ground_accel.to_le_bytes());
        fields.extend(ground_pa.to_le_bytes());
        record(16, b'F', tick, &fields)
    }

    /// A TeleMetrum v2's sensor record: D1, D2 and the accelerometer's count.
    fn a7(tick: u16, d1: u32, d2: u32, accel: i16) -> Vec<u8> {
        let mut fields = d1.to_le_bytes().to_vec();
        fields.extend(d2.to_le_bytes());
        fields.extend(accel.to_le_bytes());
        record(16, b'A', tick, &fields)
    }

    /// A state record of `size` bytes.
    fn s(size: usize, tick: u16, state: u16) -> Vec<u8> {
        record(size, b'S', tick, &state.to_le_bytes())
    }

    /// The JSON form: the head, then the records' bytes, 32 a line.
    fn json(head: &str, records: &[Vec<u8>]) -> String {
        let bytes = records.concat();
        let mut out = format!("{head}\n");
        for line in bytes.chunks(32) {
            let hex: Vec<String> = line.iter().map(|b| format!("{b:02x}")).collect();
            out.push_str(&hex.join(" "));
            out.push('\n');
        }
        out
    }

    /// The text form: the head's lines, then a record a line, its checksum left out; 8-byte
    /// records print their fields as two 16-bit words.
    fn text(head: &[&str], records: &[Vec<u8>]) -> String {
        let mut out: String = head.iter().map(|line| format!("{line}\n")).collect();
        for bytes in records {
            let tick = u16::from_le_bytes([bytes[2], bytes[3]]);
            let fields: Vec<String> = if bytes.len() == 8 {
                vec![
                    format!("{:4x}", u16::from_le_bytes([bytes[4], bytes[5]])),
                    format!("{:4x}", u16::from_le_bytes([bytes[6], bytes[7]])),
                ]
            } else {
                bytes[4..].iter().map(|b| format!("{b:02x}")).collect()
            };
            out.push_str(&format!(
                "{} {tick:x} {}\n",
                char::from(bytes[0]),
                fields.join(" ")
            ));
        }
        out
    }

    /// An invented flight: the pad's samples written after the flight record, a state record
    /// ahead of the samples it follows, a climb.
    fn flight_v2() -> Vec<Vec<u8>> {
        vec![
            f7(200, 3, 1000, 110_002),
            a7(198, EXAMPLE_D1, EXAMPLE_D2, 1000),
            a7(199, EXAMPLE_D1, EXAMPLE_D2, 1001),
            s(16, 201, 3),
            a7(200, EXAMPLE_D1, EXAMPLE_D2, 960),
            a7(201, EXAMPLE_D1 - 1000, EXAMPLE_D2, 900),
            a7(202, EXAMPLE_D1 - 2000, EXAMPLE_D2, 1040),
        ]
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> Option<&'a Channel> {
        log.channels.iter().find(|c| c.column == column)
    }

    fn values(log: &FlightLog, column: &str) -> Vec<f64> {
        channel(log, column)
            .unwrap_or_else(|| panic!("no {column}: {:#?}", log.notes))
            .values
            .clone()
    }

    fn has_note(log: &FlightLog, start: &str) -> bool {
        log.notes.iter().any(|note| note.starts_with(start))
    }

    /// The module's documented example record is what the helpers write, checksum included.
    #[test]
    fn the_documented_record_is_a_flight_record() {
        let written: Vec<String> = f7(0x164, 3, 25, 98_000)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(
            written.join(" "),
            "46 90 64 01 03 00 19 00 d0 7e 01 00 00 00 00 00"
        );
        let line = text(&[], &[f7(0x164, 3, 25, 98_000)]);
        assert_eq!(line, "F 164 03 00 19 00 d0 7e 01 00 00 00 00 00\n");
    }

    #[test]
    fn the_layout_is_recognized_in_both_forms() {
        assert!(recognizes(&json(HEAD_V2, &flight_v2())));
        assert!(recognizes(&text(TEXT_HEAD_V2, &flight_v2())));
        assert!(recognizes(
            "  {\"log_format\": 7, \"product\": \"EasyMini-v2.0\"}"
        ));
        assert!(!recognizes("{\"log_format\": 7}"), "no product");
        assert!(
            !recognizes("{\"product\": \"EasyMini-v2.0\"}"),
            "no log format"
        );
        assert!(!recognizes("{\"log_format\": 7, \"product\": 3}"));
        assert!(!recognizes(
            "Config version: 1.27\nproduct TeleMetrum-v2.0\n"
        ));
        assert!(!recognizes("manufacturer     altusmetrum.org\n"));
        assert!(!recognizes("TELEM 22ab\n"));
        assert!(!recognizes("#version,serial,flight,call,time\n"));
    }

    /// The data sheet's worked example, first order only at exactly 20.00 °C: dT 68, OFF
    /// 5,764,707,214, SENS 3,039,050,829, so P 110,002 (1,100.02 mbar) and TEMP 2,000 (p. 8).
    #[test]
    fn the_data_sheets_example_reads_to_its_values() {
        assert_eq!(
            ms5607(EXAMPLE_D1, EXAMPLE_D2, &EXAMPLE_PROM),
            (110_002, 2_000)
        );
    }

    /// 300,000 counts of D2 below the example: TEMP 9.92 °C by the first order, so the second
    /// applies (p. 9): T2 = dT² / 2³¹ = 41, OFF2 = 61 × 1,008² / 2⁴ = 3,873,744, SENS2 = 2 ×
    /// 1,008² = 2,032,128: 9.51 °C and 107,504 Pa, where the first order alone gives 107,577.
    /// Below −15 °C the third terms come in too.
    #[test]
    fn below_20_c_the_second_order_applies() {
        let cold = ms5607(EXAMPLE_D1, EXAMPLE_D2 - 300_000, &EXAMPLE_PROM);
        assert_eq!(cold, (107_504, 951));
        let very_cold = ms5607(EXAMPLE_D1, EXAMPLE_D2 - 1_500_000, &EXAMPLE_PROM);
        assert_eq!(very_cold, (95_359, -4_084));
    }

    /// Every division by 2ⁿ is a floor: in the cold case above, dT is −299,932, and
    /// `C4 × dT / 2⁶` and the others truncated toward zero instead give 107,505 Pa and 9.52 °C.
    /// The truncating version here is the mutation the test must catch.
    #[test]
    fn every_division_by_a_power_of_two_is_a_floor() {
        fn truncating(d1: u32, d2: u32, prom: &Prom) -> (i128, i128) {
            let [c1, c2, c3, c4, c5, c6] = prom.map(i128::from);
            let (d1, d2) = (i128::from(d1), i128::from(d2));
            let dt = d2 - c5 * 256;
            let mut temp = 2000 + dt * c6 / (1 << 23);
            let mut off = c2 * (1 << 17) + c4 * dt / (1 << 6);
            let mut sens = c1 * (1 << 16) + c3 * dt / (1 << 7);
            if temp < 2000 {
                off -= 61 * (temp - 2000).pow(2) / (1 << 4);
                sens -= 2 * (temp - 2000).pow(2);
                temp -= dt * dt / (1 << 31);
            }
            ((d1 * sens / (1 << 21) - off) / (1 << 15), temp)
        }
        let d2 = EXAMPLE_D2 - 300_000;
        assert_eq!(truncating(EXAMPLE_D1, d2, &EXAMPLE_PROM), (107_505, 952));
        assert_ne!(
            ms5607(EXAMPLE_D1, d2, &EXAMPLE_PROM),
            truncating(EXAMPLE_D1, d2, &EXAMPLE_PROM)
        );
        // A step just below the example: first order only at 19.90 °C would be 1,991 truncated.
        assert_eq!(
            ms5607(EXAMPLE_D1, EXAMPLE_D2 - 3_000, &EXAMPLE_PROM),
            (109_978, 1_990)
        );
    }

    /// `P = (count / 32,752 + 0.095) / 0.009` kPa: the CC1111's full scale reads 121.67 kPa, a
    /// count of 0 the sensor's 10.56 kPa offset.
    #[test]
    fn the_mp3h6115a_reads_by_its_transfer_function() {
        assert!((mp3h6115a_pa(32_752) - 1.095 / 0.009 * 1000.0).abs() < 1e-9);
        assert!((mp3h6115a_pa(0) - 0.095 / 0.009 * 1000.0).abs() < 1e-9);
        assert!((mp3h6115a_pa(24_000) - 91_975.5).abs() < 0.1);
        assert_eq!(CC1111_FULL_SCALE, f64::from(2047 * 16));
    }

    /// The flight reads: tick order, time from the flight record, the units, the figures.
    #[test]
    fn a_flight_reads_in_tick_order_from_the_flight_record() {
        let log = read(&json(HEAD_V2, &flight_v2())).unwrap();
        assert_eq!(log.format, LogFormat::AltosEeprom);
        assert_eq!(log.logger, "AltOS TeleMetrum-v2.0");
        assert_eq!(log.serial_number.as_deref(), Some("1234"));
        assert_eq!(log.firmware.as_deref(), Some("1.9.22"));
        // The flight record's number, not the configuration's `stored_flight`.
        assert_eq!(log.flight_number, Some(3));
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        let pressure = channel(&log, "pressure").unwrap();
        assert_eq!(pressure.quantity, Quantity::Pressure);
        assert_eq!(pressure.time_s, [-0.02, -0.01, 0.0, 0.01, 0.02]);
        assert_eq!(pressure.values[..3], [110_002.0; 3]);
        assert!(pressure.values[3] < pressure.values[2]);
        assert!((values(&log, "temperature")[0] - 293.15).abs() < 1e-9);
        // 20 counts a g, 0 at the ground reading of 1,000.
        let g = STANDARD_GRAVITY;
        let acceleration = values(&log, "acceleration");
        for (got, want) in acceleration
            .iter()
            .zip([0.0, -0.05 * g, 2.0 * g, 5.0 * g, -2.0 * g])
        {
            assert!((got - want).abs() < 1e-12, "{acceleration:?}");
        }
        assert_eq!(log.events.len(), 1);
        assert_eq!(
            (log.events[0].time_s, log.events[0].name.as_str()),
            (0.01, "boost")
        );
        let figure = |name: &str| log.stated.figures.iter().find(|f| f.name == name).unwrap();
        assert_eq!(figure("ground_pressure").values, [110_002.0]);
        assert_eq!(figure("ground_pressure").quantity, Some(Quantity::Pressure));
        assert_eq!(figure("accel_cal_minus").values, [1040.0]);
        assert_eq!(figure("main_deploy").quantity, Some(Quantity::Unrecognized));
        assert!(has_note(
            &log,
            "7 records of 16 bytes, log format 7, from the JSON form"
        ));
        assert!(has_note(
            &log,
            "the clock is the board's tick, 10 ms, from the flight record's at tick 200"
        ));
    }

    /// The text form of the same records reads to the same channels and events.
    #[test]
    fn the_text_and_json_forms_of_a_flight_read_alike() {
        let from_json = read(&json(HEAD_V2, &flight_v2())).unwrap();
        let from_text = read(&text(TEXT_HEAD_V2, &flight_v2())).unwrap();
        assert_eq!(from_json.channels, from_text.channels);
        assert_eq!(from_json.events, from_text.events);
        assert_eq!(from_json.serial_number, from_text.serial_number);
        assert_eq!(from_json.flight_number, from_text.flight_number);
        assert_eq!(from_json.firmware, from_text.firmware);
        assert!(has_note(
            &from_text,
            "7 records of 16 bytes, log format 7, from the text form"
        ));
        // The text form names its settings' units, so they are in SI.
        let main = from_text
            .stated
            .figures
            .iter()
            .find(|f| f.name == "main_deploy")
            .unwrap();
        assert_eq!(main.quantity, Some(Quantity::AltitudeAboveGround));
        assert_eq!(
            (main.written.as_str(), main.values.as_slice()),
            ("250 meters", &[250.0][..])
        );
    }

    /// One byte changed, the checksum not: refused at the record's byte.
    #[test]
    fn a_record_failing_its_checksum_refuses_the_file() {
        let mut records = flight_v2();
        records[2][9] ^= 0x01;
        match read(&json(HEAD_V2, &records)) {
            Err(LogError::Binary {
                offset, message, ..
            }) => {
                assert_eq!(offset, 32);
                assert!(
                    message.starts_with("record 3 fails its checksum"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
    }

    /// A log format the samples don't show is refused by its number, in both forms; one the
    /// configuration doesn't state is format 1 only for a TeleMetrum v1.
    #[test]
    fn a_log_format_no_sample_shows_is_refused_by_number() {
        for format in [2, 5, 8, 13, 17, 21, 23] {
            let head = HEAD_V2.replace("\"log_format\": 7", &format!("\"log_format\": {format}"));
            match read(&json(&head, &flight_v2())) {
                Err(LogError::Syntax { line, message, .. }) => {
                    assert_eq!(line, 1);
                    assert!(
                        message.starts_with(&format!("log format {format}, which no sample")),
                        "{message}"
                    );
                }
                other => panic!("{format}: {other:?}"),
            }
        }
        let lines: Vec<&str> = TEXT_HEAD_V2
            .iter()
            .map(|line| {
                if line.starts_with("log-format") {
                    "log-format       9"
                } else {
                    line
                }
            })
            .collect();
        match read(&text(&lines, &flight_v2())) {
            Err(LogError::Syntax { line, message, .. }) => {
                assert_eq!(line, TEXT_HEAD_V2.len() + 1);
                assert!(
                    message.starts_with("log format 9, which no sample"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
        // No log format: a TeleMetrum v1 is format 1, an EasyMini refused.
        let unstated = HEAD_V2.replace("\"log_format\": 7,", "\"log_format\": 2147483647,");
        match read(&json(&unstated, &flight_v2())) {
            Err(LogError::Syntax { message, .. }) => {
                assert!(
                    message.contains("states no log format, and its product, \"TeleMetrum-v2.0\""),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
        let head_v1 = r#"{"log_format": 2147483647, "product": "TeleMetrum-v1.0", "accel_cal_plus": 16000, "accel_cal_minus": 16500}"#;
        let records = [
            record(8, b'F', 100, &[0x80, 0x3e, 3, 0]),
            record(8, b'A', 100, &[0x80, 0x3e, 0xc0, 0x5d]),
        ];
        let log = read(&json(head_v1, &records)).unwrap();
        assert!(has_note(
            &log,
            "the configuration states no log format, as firmware before 1.0"
        ));
        assert!(has_note(&log, "2 records of 8 bytes, log format 1"));
    }

    /// Erased memory after the last record is counted; a record after it refuses the file.
    #[test]
    fn erased_memory_ends_the_records() {
        let mut records = flight_v2();
        records.push(vec![0xff; 16]);
        records.push(vec![0xff; 16]);
        let log = read(&json(HEAD_V2, &records)).unwrap();
        assert!(has_note(
            &log,
            "2 erased records (every byte 0xFF) after the last, skipped"
        ));
        records.push(s(16, 203, 4));
        match read(&json(HEAD_V2, &records)) {
            Err(LogError::Binary {
                offset, message, ..
            }) => {
                assert_eq!(offset, 9 * 16);
                assert!(
                    message.starts_with("a record after the erased memory from byte 112"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
        // A token that isn't a byte refuses the file, naming its line.
        let damaged = json(HEAD_V2, &flight_v2()).replacen("\n46 ", "\n4g ", 1);
        assert!(matches!(read(&damaged), Err(LogError::Syntax { .. })));
    }

    /// The last line of a text file that ends without a line break is left out, however well it
    /// reads; a malformed line anywhere else refuses the file.
    #[test]
    fn a_text_files_last_line_cut_short_is_left_out() {
        let whole = text(TEXT_HEAD_V2, &flight_v2());
        let cut = &whole[..whole.len() - 7];
        let log = read(cut).unwrap();
        assert!(has_note(
            &log,
            "the file ends without a line break, so its last line may be cut short"
        ));
        assert_eq!(values(&log, "pressure").len(), 4);
        // Cut at the line's end, the last record still reads as one, and is still left out.
        let log = read(whole.trim_end_matches('\n')).unwrap();
        assert_eq!(values(&log, "pressure").len(), 4);
        // A short line before the last refuses the file.
        let mut lines: Vec<&str> = whole.lines().collect();
        let short = "A c8 64 a7 62 00";
        lines.insert(TEXT_HEAD_V2.len() + 2, short);
        let damaged = lines.join("\n") + "\n";
        match read(&damaged) {
            Err(LogError::Syntax { line, message, .. }) => {
                assert_eq!(line, TEXT_HEAD_V2.len() + 3);
                assert!(
                    message.contains("isn't a record of log format 7"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
    }

    /// The 16-bit tick turns over: each record's tick goes on the turn nearest the latest so far,
    /// so file order across the turn differs from tick order, and the records are sorted.
    #[test]
    fn the_ticks_turns_are_unwound_and_the_records_sorted() {
        let records = vec![
            // The flight record just before the turn, the pad's samples written after it.
            f7(65_534, 3, 1000, 110_002),
            a7(65_533, EXAMPLE_D1, EXAMPLE_D2, 1000),
            a7(65_535, EXAMPLE_D1, EXAMPLE_D2, 1000),
            // Across the turn: a state record written ahead of a sample with an earlier tick.
            s(16, 1, 3),
            a7(0, EXAMPLE_D1, EXAMPLE_D2, 990),
            a7(1, EXAMPLE_D1, EXAMPLE_D2, 980),
            // A GPS record carrying its fix's earlier tick, from before the turn.
            record(16, b'G', 65_500, &[12, 0, 1, 0x10 | 6, 26, 10, 14, 40]),
            a7(2, EXAMPLE_D1, EXAMPLE_D2, 970),
        ];
        let log = read(&json(HEAD_V2, &records)).unwrap();
        let pressure = channel(&log, "pressure").unwrap();
        assert_eq!(pressure.time_s, [-0.01, 0.01, 0.02, 0.03, 0.04]);
        let acceleration = values(&log, "acceleration");
        assert_eq!(acceleration[..2], [0.0, 0.0]);
        assert!((acceleration[2] - 0.5 * STANDARD_GRAVITY).abs() < 1e-12);
        assert_eq!(log.events[0].time_s, 0.03);
        assert_eq!(channel(&log, "hour").unwrap().time_s, [-0.34]);
        assert!(has_note(
            &log,
            "the clock is the board's tick, 10 ms, from the flight record's at tick 65534"
        ));
        assert!(
            log.notes
                .iter()
                .any(|n| n.ends_with("2 records before boost, at negative times"))
        );
    }

    /// A record more than 10 s from the latest tick ends the flight: it and every record after
    /// it are left out. Exactly 10 s is kept.
    #[test]
    fn a_silence_of_more_than_10_s_ends_the_flight() {
        let mut records = flight_v2();
        records.push(s(16, 1_202, 8));
        let log = read(&json(HEAD_V2, &records)).unwrap();
        assert_eq!(log.events.last().map(|e| e.time_s), Some(10.02));
        records.push(s(16, 2_203, 2));
        records.push(a7(2_204, EXAMPLE_D1, EXAMPLE_D2, 1000));
        let log = read(&json(HEAD_V2, &records)).unwrap();
        assert_eq!(values(&log, "pressure").len(), 5);
        assert!(has_note(
            &log,
            "2 records from the first more than 10 s from the latest tick on are left out"
        ));
        // A record more than 10 s behind ends it too.
        let mut records = flight_v2();
        records.push(a7(60_000, EXAMPLE_D1, EXAMPLE_D2, 1000));
        let log = read(&json(HEAD_V2, &records)).unwrap();
        assert!(has_note(&log, "1 records from the first more than 10 s"));
    }

    /// A record repeated exactly is dropped and counted; the same bytes at another tick are not
    /// a repeat.
    #[test]
    fn records_repeated_exactly_are_dropped() {
        let mut records = flight_v2();
        records.insert(5, records[4].clone());
        records.push(s(16, 203, 3));
        let log = read(&text(TEXT_HEAD_V2, &records)).unwrap();
        assert!(has_note(&log, "1 records repeated exactly"));
        assert_eq!(values(&log, "pressure").len(), 5);
        assert_eq!(values(&log, "state"), [3.0, 3.0]);
    }

    #[test]
    fn a_file_needs_exactly_one_flight_record() {
        let mut records = flight_v2();
        records.push(f7(210, 4, 1000, 110_002));
        match read(&json(HEAD_V2, &records)) {
            Err(LogError::Binary {
                offset, message, ..
            }) => {
                assert_eq!(offset, 7 * 16);
                assert!(
                    message.starts_with("a second flight record, after the one at byte 0"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
        let records: Vec<Vec<u8>> = flight_v2().into_iter().skip(1).collect();
        match read(&json(HEAD_V2, &records)) {
            Err(LogError::Binary { message, .. }) => {
                assert!(message.starts_with("no flight record"))
            }
            other => panic!("{other:?}"),
        }
    }

    /// The pad's pressure must be a launch site's, 10 to 120 kPa, and an MS5607 board's within
    /// 1% of its flight record's ground pressure; otherwise the bytes are misread.
    #[test]
    fn the_pads_pressure_is_held_to_a_launch_site_and_the_flight_record() {
        let v1 = |count: i16| {
            let head = r#"{"log_format": 1, "product": "TeleMetrum-v1.2", "accel_cal_plus": 16000, "accel_cal_minus": 16500}"#;
            let [lo, hi] = count.to_le_bytes();
            read(&json(
                head,
                &[
                    record(8, b'F', 100, &[0x80, 0x3e, 3, 0]),
                    record(8, b'A', 99, &[0x80, 0x3e, lo, hi]),
                ],
            ))
        };
        // −163 counts is 10.002 kPa, −164 is 9.999; 32,260 is 119.998, 32,261 is 120.001.
        assert!(v1(-163).is_ok() && v1(32_260).is_ok());
        for count in [-164, 32_261] {
            match v1(count) {
                Err(LogError::Binary {
                    offset, message, ..
                }) => {
                    assert_eq!(offset, 0);
                    assert!(
                        message.contains("outside every launch site's 10 to 120 kPa"),
                        "{message}"
                    );
                }
                other => panic!("{count}: {other:?}"),
            }
        }
        // 110,002 Pa on the pad: a ground pressure 0.99% off is kept, 1.01% or more refused.
        let with_ground = |ground: u32| {
            let mut records = flight_v2();
            records[0] = f7(200, 3, 1000, ground);
            read(&json(HEAD_V2, &records))
        };
        assert!(with_ground(111_100).is_ok() && with_ground(108_920).is_ok());
        for ground in [111_125, 108_890] {
            match with_ground(ground) {
                Err(LogError::Binary { message, .. }) => {
                    assert!(
                        message.contains("from the flight record's ground pressure"),
                        "{message}"
                    );
                }
                other => panic!("{ground}: {other:?}"),
            }
        }
    }

    /// The acceleration comes from the file's calibration and the flight record's ground
    /// reading. A board mounted antenna down keeps its calibration flipped, `plus' = K − minus`
    /// and `minus' = K − plus`, with the same span, so either pair reads alike: K = 0 for a
    /// signed sensor, 32,767 for a TeleMetrum v1's.
    #[test]
    fn acceleration_is_from_the_files_own_calibration() {
        let cooked = HEAD_V2
            .replace("\"accel_cal_minus\": 1040, \"accel_cal_plus\": 1000", "\"accel_cal_minus\": 2147483647, \"accel_cal_plus\": 2147483647, \"accel_cal_minus_cooked\": -1000, \"accel_cal_plus_cooked\": -1040");
        let raw = read(&json(HEAD_V2, &flight_v2())).unwrap();
        let flipped = read(&json(&cooked, &flight_v2())).unwrap();
        assert_eq!(
            values(&raw, "acceleration"),
            values(&flipped, "acceleration")
        );

        let v1 = |plus: i64, minus: i64| {
            let head = format!(
                r#"{{"log_format": 1, "product": "TeleMetrum-v1.2", "accel_cal_plus": {plus}, "accel_cal_minus": {minus}}}"#
            );
            // Ground reading 16,000; a sample at 15,500.
            let records = [
                record(8, b'F', 100, &[0x80, 0x3e, 3, 0]),
                record(8, b'A', 100, &[0x8c, 0x3c, 0xc0, 0x5d]),
            ];
            read(&json(&head, &records)).unwrap()
        };
        let upright = values(&v1(16_000, 16_500), "acceleration");
        let down = values(&v1(32_767 - 16_500, 32_767 - 16_000), "acceleration");
        assert_eq!(upright, down);
        assert!((upright[0] - 2.0 * STANDARD_GRAVITY).abs() < 1e-12);
    }

    /// Without a calibration, or with a span that isn't positive, the counts are kept as written.
    #[test]
    fn an_unset_calibration_keeps_the_counts_as_written() {
        let unset = HEAD_V2.replace("\"accel_cal_plus\": 1000", "\"accel_cal_plus\": 2147483647");
        let log = read(&json(&unset, &flight_v2())).unwrap();
        assert!(channel(&log, "acceleration").is_none());
        let accel = channel(&log, "accel").unwrap();
        assert_eq!(accel.quantity, Quantity::Unrecognized);
        assert_eq!(accel.values, [1000.0, 1001.0, 960.0, 900.0, 1040.0]);
        assert!(has_note(
            &log,
            "`accel` kept as written: the calibration isn't set"
        ));
        let reversed = HEAD_V2.replace("\"accel_cal_minus\": 1040", "\"accel_cal_minus\": 1000");
        let log = read(&json(&reversed, &flight_v2())).unwrap();
        assert!(has_note(
            &log,
            "`accel` kept as written: the calibration's span isn't positive"
        ));
    }

    /// No MS5607 constants, or an MS5611: D1 and D2 kept as written, and no pad check.
    #[test]
    fn without_the_barometers_constants_its_counts_are_kept() {
        let ms5611 = HEAD_V2.replace("\"ms5611\": false", "\"ms5611\": true");
        let log = read(&json(&ms5611, &flight_v2())).unwrap();
        assert!(channel(&log, "pressure").is_none());
        assert_eq!(values(&log, "d1")[0], f64::from(EXAMPLE_D1));
        assert!(has_note(
            &log,
            "`d1` and `d2` kept as written: the configuration marks the barometer an MS5611"
        ));
        let no_tref = HEAD_V2.replace("\"tref\": 31553", "\"tref\": 70000");
        let log = read(&json(&no_tref, &flight_v2())).unwrap();
        assert!(has_note(
            &log,
            "`d1` and `d2` kept as written: the configuration holds no MS5607 constants"
        ));
    }

    /// A Mega's IMU: read on format 16 and 22 where the flight record's IMU fields aren't all zero;
    /// not on format 15, nor where they are.
    #[test]
    fn a_megas_imu_is_read_only_where_its_layout_is_confirmed() {
        let mega = |format: u32, imu: i16| {
            let head = HEAD_V2
                .replace("\"log_format\": 7", &format!("\"log_format\": {format}"))
                .replace("TeleMetrum-v2.0", "TeleMega-v6.0");
            let mut flight = 3u16.to_le_bytes().to_vec();
            flight.extend(1000i16.to_le_bytes());
            flight.extend(110_002u32.to_le_bytes());
            flight.extend([imu, 0, 0].iter().flat_map(|v| v.to_le_bytes()));
            let mut sample = EXAMPLE_D1.to_le_bytes().to_vec();
            sample.extend(EXAMPLE_D2.to_le_bytes());
            sample.extend((1..=9i16).flat_map(|v| v.to_le_bytes()));
            sample.extend(980i16.to_le_bytes());
            let mut sense = 2000i16.to_le_bytes().to_vec();
            sense.extend(4095i16.to_le_bytes());
            sense.extend(6i16.to_le_bytes());
            sense.extend(
                [0i16, 4095, 0, 0, 1000, 0]
                    .iter()
                    .flat_map(|v| v.to_le_bytes()),
            );
            let records = [
                record(32, b'F', 100, &flight),
                record(32, b'A', 99, &sample),
                record(32, b'T', 101, &sense),
            ];
            read(&json(&head, &records)).unwrap()
        };
        let log = mega(16, 5);
        assert_eq!(values(&log, "imu_gyro_0"), [4.0]);
        assert_eq!(
            channel(&log, "imu_mag_2").unwrap().quantity,
            Quantity::Unrecognized
        );
        assert!((values(&log, "acceleration")[0] - STANDARD_GRAVITY).abs() < 1e-12);
        // The 15 V boards' sense: 4,095 counts is 3.3 V × 127 / 27.
        let full = 3.3 * 127.0 / 27.0;
        assert!((values(&log, "pyro")[0] - full).abs() < 1e-12);
        assert!((values(&log, "igniter_b")[0] - full).abs() < 1e-12);
        assert_eq!(channel(&log, "pyro").unwrap().quantity, Quantity::Voltage);
        assert_eq!(values(&log, "v_batt"), [2000.0]);
        assert!(values(&log, "drogue_voltage")[0] > 0.0);
        let log = mega(15, 5);
        assert!(channel(&log, "imu_accel_0").is_none());
        assert!(has_note(
            &log,
            "the IMU not read: format 15's layout of it is unconfirmed"
        ));
        let log = mega(22, 0);
        assert!(channel(&log, "imu_accel_0").is_none());
        assert!(has_note(
            &log,
            "the IMU not read: the flight record's IMU fields are all zero"
        ));
    }

    /// A position without a valid fix at its tick is a gap; with `altitude-32`, the altitude's
    /// high 16 bits count.
    #[test]
    fn gps_positions_need_a_valid_fix() {
        let position = |tick: u16, lat: i32, lon: i32, low: u16, high: i16| {
            let mut fields = lat.to_le_bytes().to_vec();
            fields.extend(lon.to_le_bytes());
            fields.extend(low.to_le_bytes());
            fields.extend(high.to_le_bytes());
            record(16, b'P', tick, &fields)
        };
        let mut records = flight_v2();
        records.extend([
            position(150, 450_000_000, -1_220_000_000, 4_464, 1),
            record(16, b'G', 150, &[12, 0, 1, 0x10 | 6, 26, 10, 14, 40]),
            position(180, 450_000_000, -1_220_000_000, 1_000, 0),
            record(16, b'G', 180, &[12, 0, 1, 0x20 | 3, 26, 10, 14, 40]),
        ]);
        let log = read(&json(HEAD_V2, &records)).unwrap();
        let latitude = values(&log, "latitude");
        assert_eq!(latitude[0], 45.0);
        assert!(latitude[1].is_nan());
        assert_eq!(values(&log, "longitude")[0], -122.0);
        assert_eq!(values(&log, "gps_altitude")[0], 70_000.0);
        assert_eq!(values(&log, "nsat"), [6.0, 3.0]);
        assert!(has_note(&log, "1 GPS position records without a valid fix"));
    }

    /// States are named as AltosUI's exports name them; another number is `state N`.
    #[test]
    fn states_are_named_and_others_numbered() {
        let mut records = flight_v2();
        records.extend([s(16, 203, 5), s(16, 204, 5), s(16, 205, 9)]);
        let log = read(&json(HEAD_V2, &records)).unwrap();
        let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["boost", "coast", "state 9"]);
    }

    /// The text form's configuration printed twice must agree.
    #[test]
    fn a_configuration_printed_twice_must_agree() {
        let mut head: Vec<&str> = TEXT_HEAD_V2.to_vec();
        head.extend(TEXT_HEAD_V2);
        assert!(read(&text(&head, &flight_v2())).is_ok());
        head.push("serial-number    4321");
        match read(&text(&head, &flight_v2())) {
            Err(LogError::Syntax { line, message, .. }) => {
                assert_eq!(line, head.len());
                assert!(
                    message.contains("its serial number here, \"4321\", differs from line 16's"),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
    }

    /// A record type the board's layout doesn't hold is counted, not read.
    #[test]
    fn an_unknown_record_type_is_counted() {
        let mut records = flight_v2();
        records.push(record(16, b'Z', 203, &[1, 2, 3]));
        let log = read(&json(HEAD_V2, &records)).unwrap();
        assert!(has_note(&log, "1 records of type `Z` not read"));
    }
}
