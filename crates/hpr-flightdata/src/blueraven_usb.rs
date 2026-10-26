//! The Featherweight Blue Raven's flight data as its USB port downloads it: one `@ LOG_LOW` or
//! `@ LOG_HIR` line a sample, each closed by a CRC, as a terminal program saves them.
//!
//! The Blue Raven records a low-rate set of data "recorded at 50 Hz" and a high-rate set
//! "recorded at 500 Hz" for each flight (Featherweight, *Blue Raven User's Guide*, updated May 12
//! 2026, p18). Besides the phone app's download ([`crate::blueraven`]), the logger's USB port
//! sends either set to a serial terminal on a command, `<download l>` or `<download h>`, and the
//! manual describes the lines it sends (Appendix A, p23–24). Each line "starts with the mnemonic
//! `@ LOG_LOW` followed by the length of the string, and the time and date of the flight", then
//! the sync code and the data. The manual ends the low-rate line with a "16b Buypass CRC code for
//! line of text" (p23) and lists none for the high-rate line (p24), but the high-rate lines in the
//! sample logs close with a CRC too, of the same kind:
//!
//! ```text
//! @ LOG_HIR  139 2026  5 23 10:15:00.120   20    -15     42     -7     -1      3     98  -3000   1500    -40  29812      0      0 CRC: 7730
//! @ LOG_LOW  211 2026  5 23 10:15:00.140  40 Bo:  7212 49655 V:  4105    22    21    20    23 Vel:     0     0     0 Pos:     0     0     0 ang:   12    5    0 FER:  400  1c1   c3   c1   c3   0   0   0 CRC: F7EB
//! ```
//!
//! **Recognition.** A file is a download when one of its first 1,000 lines opens with `@ LOG_HIR`
//! or `@ LOG_LOW`, holds no other `@ LOG_`, and closes with a CRC that checks (below); no other
//! layout HPR Sim reads opens a line so.
//!
//! **Records and their checks.** A terminal saves what the port sends, and the sample logs show
//! what that costs: a record whose `CRC: XXXX` and line end were lost runs on into the next one, on
//! one line. So every line is cut before each `@ LOG_`, and each piece is a record only if it holds
//! two things the logger wrote for it:
//!
//! - *Its CRC.* The piece closes with `CRC:` and four hexadecimal digits, the CRC-16 the manual
//!   calls Buypass (polynomial `0x8005`, starting at 0, no reflection, no final XOR; its check value
//!   on `123456789` is `0xFEE8`) of the piece's text from its `@` to its last character that
//!   isn't a space before `CRC:`: the spaces inside the line count, and only the run of spaces
//!   just before `CRC:` is left out. The example lines' CRCs are `7730` and `F7EB`. The manual says the CRC is "for line of text" but
//!   not which characters it starts and ends at; this span is the one under which every CRC in
//!   the sample files checks.
//! - *Its length.* The number after the mnemonic must be the piece's length in bytes, from `@`
//!   through the CRC's last digit, plus 2 for the line end the logger sends: `139` for a high-rate
//!   line of 137 characters.
//!
//! A piece with no CRC, or whose CRC differs from its text, is dropped, and a note counts each
//! kind: it can't be told from a record the capture damaged. So is a line holding no `@ LOG_`
//! record, such as the live status line, `@ BLR_STAT`, which the port sends five times a second
//! (p22), or text before a line's first record. A record whose CRC checks is the logger's, so
//! anything else wrong in it refuses the file ([`LogError::Syntax`]), naming its line: a stated
//! length that isn't its own, a mnemonic other than these two, a date or time that can't be one, a
//! value that isn't a whole number (or, in `FER:`, hexadecimal digits), more than
//! [`MAX_VALUES`] values or [`MAX_GROUPS`] groups, a label longer than [`MAX_LABEL_BYTES`], or a
//! layout unlike the first record's, its kind or its groups and their
//! counts, so that a file holding both downloads is refused too.
//!
//! **The clock.** Each record states the logger's date and time of day to the millisecond, `2026
//! 5 23 10:15:00.120`; the record's clock counts from the first record's, and a time that doesn't
//! increase is refused, naming its line. The manual says the date stamp "starts with the build
//! time/date and increments from there", to be set by the phone (p22), so the date need not be the
//! flight's: a note gives the first record's as written. `Year`, `Month`, `Day` and `Sync` are
//! [`Quantity::Number`] channels and `Time` a [`Quantity::TimeOfDay`] one, as the app's CSV keeps
//! them. The sync code is "a millisecond counter recorded in both the low-rate and high-rate data
//! so that the two data sets can be aligned", which "rolls over every 250 msec" (p23).
//!
//! **The high-rate record** holds, after the sync code, "[gyro X deg/s x100] [gyro Y deg/s x100]
//! [gyro Z deg/s x100] [Accel X Gs x100] [Accel Y Gs x100] [Accel Z Gs x100] [Quaternion X x 30000]
//! [Quaternion Y x 30000] [Quaternion Z x 30000] [Quaternion magnitude x 30000]" (p24). The gyro
//! rates are "raw data in the sensor axis", and the accelerations come from the ±32 g
//! accelerometers below 32 g and from the high-range one above it. The quaternion is written "with
//! the rotation axis first and the magnitude term fourth", "scaled up by a factor of 30,000 to fit
//! into signed 16-bit integers".
//!
//! **The low-rate record** holds labeled groups (p23):
//!
//! | Group | The manual's fields, in its order | Read as |
//! |---|---|---|
//! | `Bo:` | temperature from the baro sensor; pressure in atm × 50,000 | the temperature as written, [`Quantity::Unrecognized`], as the manual states no unit for it here; the pressure in Pa |
//! | `V:` | battery, Apo, Main, 3rd and 4th outputs, mV; output current, mA | V; A |
//! | `Vel:` | upward, down-range and cross-range velocity, ft/s | m/s, [`Quantity::Speed`] |
//! | `Pos:` | inertial altitude, down-range and cross-range position, ft | m: [`Quantity::InertialAltitude`], then [`Quantity::Distance`] |
//! | `ang:` | tilt angle, deg × 10; roll angle, deg; future tilt angle, deg × 10 | degrees, [`Quantity::Angle`] |
//! | `FER:` | the rocket's flight event register, then the Apo, Main, 3rd and 4th outputs' | the registers' bits, [`Quantity::Number`], from hexadecimal |
//!
//! The velocities and positions are "ground-relative inertial coordinates (rather than rocket
//! axes)", and the registers "hex-encoded binary values that use one bit for each event" (p23–24);
//! the manual doesn't say which bit is which event, so no [`Event`](crate::log::Event) is taken
//! from them.
//!
//! **Counts unlike the manual's.** A logger's firmware need not write what the manual of the day
//! lists: in the sample logs, a high-rate record holds 12 values where the manual lists 10, `V:` holds 5 where it lists 6, and `FER:` 8 where it lists 5. One rule takes
//! them: a group holding at least the manual's count is read by the manual, in its order, and the
//! values past it are kept as written, [`Quantity::Unrecognized`] (in `FER:`, registers of unknown
//! meaning, [`Quantity::Number`]); a group holding fewer is kept as written whole, as nothing says
//! which value is missing. A group the manual doesn't list is kept as written too. A value kept as
//! written is named by its group and its place there, counted from 1, such as `V: 3` or `LOG_HIR
//! 11`, and a note says so for each group. The maker's forum describes two more 500 Hz fields than
//! the manual, the "deployment output current" and the "aux analog voltage" (Featherweight,
//! *Blue Raven development thread*, rocketryforum.com, 2023-03-31), but not their order, so the
//! high-rate record's 11th and 12th values stay as written.
//!
//! The sample logs' survey (`cargo xtask logs`) holds three parts of the reading to the file and
//! the same rocket's other files: every high-rate record's quaternion, read from the 7th to the
//! 10th value, has a norm within 2 of 30,000, as a unit quaternion cut to whole numbers must; the
//! first `FER:` value's lowest bit, the one the app's files call `Liftoff`, first sets within 0.1 s
//! after the three axes' acceleration first passes 3 g; and the pressure on the pad is within 2 %
//! of the TeleMega's in the same rocket. The gyro rates, velocities, positions, angles and
//! voltages are read by the manual's scales with no such check.
//!
//! The record has no barometric track, as the download writes the pressure, not an altitude:
//! [`FlightLog::time_s`] and [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`,
//! and every value is a channel.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Blue Raven USB download";

/// How many of a file's first lines are searched for a record whose CRC checks.
const RECOGNITION_LINES: usize = 1_000;

/// The most values a record may hold after its sync code, over ten times the 24 of the sample
/// logs' low-rate record; a file past it isn't a Blue Raven's.
pub const MAX_VALUES: usize = 256;

/// The most groups a low-rate record may hold, over twice the manual's 6, and the longest a
/// group's label may be, bytes, four times the manual's longest, `Vel:`: so that a record's
/// labels can't fill its columns' names, its notes or the time spent checking them.
pub const MAX_GROUPS: usize = 16;
/// See [`MAX_GROUPS`].
pub const MAX_LABEL_BYTES: usize = 16;

/// What opens every record.
const OPENING: &str = "@ LOG_";

/// The bytes a stated length counts past the record's text: the CR and LF the logger sends.
const LINE_END_BYTES: usize = 2;

/// The two records' mnemonics.
const HIGH_RATE: &str = "LOG_HIR";
const LOW_RATE: &str = "LOG_LOW";

/// The label of the group whose values are hexadecimal.
const REGISTERS: &str = "FER:";

/// The CRC-16 the manual calls Buypass (also CRC-16/UMTS): polynomial `0x8005`, initial value 0,
/// no reflection, no final XOR.
pub(crate) fn crc16_buypass(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in bytes {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x8005
            } else {
                crc << 1
            };
        }
    }
    crc
}

/// One value's place in a record: its column's name, what it measures, and how the integer the
/// file writes becomes the quantity's unit, divided by `per`, then multiplied by `unit`.
struct Field {
    column: &'static str,
    quantity: Quantity,
    per: f64,
    unit: f64,
}

const fn field(column: &'static str, quantity: Quantity, per: f64, unit: f64) -> Field {
    Field {
        column,
        quantity,
        per,
        unit,
    }
}

/// One standard atmosphere, Pa.
const ATMOSPHERE_PA: f64 = 101_325.0;

/// The high-rate record's values after the sync code, as the manual lists them (p24).
const HIGH_RATE_FIELDS: [Field; 10] = [
    field("Gyro X", Quantity::AngularRate, 100.0, 1.0),
    field("Gyro Y", Quantity::AngularRate, 100.0, 1.0),
    field("Gyro Z", Quantity::AngularRate, 100.0, 1.0),
    field(
        "Accel X",
        Quantity::Acceleration,
        100.0,
        STANDARD_GRAVITY_MPS2,
    ),
    field(
        "Accel Y",
        Quantity::Acceleration,
        100.0,
        STANDARD_GRAVITY_MPS2,
    ),
    field(
        "Accel Z",
        Quantity::Acceleration,
        100.0,
        STANDARD_GRAVITY_MPS2,
    ),
    field("Quaternion X", Quantity::Number, 30_000.0, 1.0),
    field("Quaternion Y", Quantity::Number, 30_000.0, 1.0),
    field("Quaternion Z", Quantity::Number, 30_000.0, 1.0),
    field("Quaternion magnitude", Quantity::Number, 30_000.0, 1.0),
];

/// The low-rate record's groups, each its label and the manual's fields in its order (p23).
const LOW_RATE_GROUPS: [(&str, &[Field]); 6] = [
    (
        "Bo:",
        &[
            field("Bo: temperature", Quantity::Unrecognized, 1.0, 1.0),
            field("Bo: pressure", Quantity::Pressure, 50_000.0, ATMOSPHERE_PA),
        ],
    ),
    (
        "V:",
        &[
            field("V: battery", Quantity::Voltage, 1_000.0, 1.0),
            field("V: Apo", Quantity::Voltage, 1_000.0, 1.0),
            field("V: Main", Quantity::Voltage, 1_000.0, 1.0),
            field("V: 3rd", Quantity::Voltage, 1_000.0, 1.0),
            field("V: 4th", Quantity::Voltage, 1_000.0, 1.0),
            field("V: current", Quantity::Current, 1_000.0, 1.0),
        ],
    ),
    (
        "Vel:",
        &[
            field("Vel: up", Quantity::Speed, 1.0, FOOT_M),
            field("Vel: down-range", Quantity::Speed, 1.0, FOOT_M),
            field("Vel: cross-range", Quantity::Speed, 1.0, FOOT_M),
        ],
    ),
    (
        "Pos:",
        &[
            field("Pos: altitude", Quantity::InertialAltitude, 1.0, FOOT_M),
            field("Pos: down-range", Quantity::Distance, 1.0, FOOT_M),
            field("Pos: cross-range", Quantity::Distance, 1.0, FOOT_M),
        ],
    ),
    (
        "ang:",
        &[
            field("ang: tilt", Quantity::Angle, 10.0, 1.0),
            field("ang: roll", Quantity::Angle, 1.0, 1.0),
            field("ang: future tilt", Quantity::Angle, 10.0, 1.0),
        ],
    ),
    (
        REGISTERS,
        &[
            field("FER: rocket", Quantity::Number, 1.0, 1.0),
            field("FER: Apo", Quantity::Number, 1.0, 1.0),
            field("FER: Main", Quantity::Number, 1.0, 1.0),
            field("FER: 3rd", Quantity::Number, 1.0, 1.0),
            field("FER: 4th", Quantity::Number, 1.0, 1.0),
        ],
    ),
];

/// The manual's fields for a group, by its label (`None` for the high-rate record's values, which
/// have none).
fn manual_fields(kind: &str, label: Option<&str>) -> &'static [Field] {
    match label {
        None if kind == HIGH_RATE => &HIGH_RATE_FIELDS,
        None => &[],
        Some(label) => LOW_RATE_GROUPS
            .iter()
            .find(|(known, _)| *known == label)
            .map_or(&[], |(_, fields)| fields),
    }
}

/// A piece of a line that opens `@ LOG_`, with what the logger wrote to check it.
#[derive(Debug, PartialEq)]
enum Piece<'a> {
    /// No `CRC:` and four hexadecimal digits close it.
    NoCrc,
    /// Its CRC differs from its text's.
    CrcDiffers,
    /// Its CRC checks: the text from `@` to the CRC's last digit.
    Checked(&'a str),
}

/// The pieces of a line, each from an `@ LOG_` to the next or the line's end, and whether text
/// that isn't blank comes before the first.
fn pieces(line: &str) -> (bool, impl Iterator<Item = Piece<'_>>) {
    let starts: Vec<usize> = line.match_indices(OPENING).map(|(at, _)| at).collect();
    let before = starts
        .first()
        .map_or(line, |&first| &line[..first])
        .trim()
        .is_empty();
    let ends = starts
        .iter()
        .skip(1)
        .copied()
        .chain(std::iter::once(line.len()));
    let spans: Vec<(usize, usize)> = starts.iter().copied().zip(ends).collect();
    (
        !before,
        spans
            .into_iter()
            .map(move |(start, end)| piece(line[start..end].trim_end())),
    )
}

/// A piece checked against the CRC that closes it.
fn piece(text: &str) -> Piece<'_> {
    let Some(at) = text.rfind("CRC:") else {
        return Piece::NoCrc;
    };
    let digits = text[at + 4..].trim_start();
    let stated = (digits.len() == 4)
        .then(|| u16::from_str_radix(digits, 16).ok())
        .flatten()
        .filter(|_| digits.bytes().all(|byte| byte.is_ascii_hexdigit()));
    let Some(stated) = stated else {
        return Piece::NoCrc;
    };
    let body = text[..at].trim_end_matches(' ');
    if crc16_buypass(body.as_bytes()) == stated {
        Piece::Checked(text)
    } else {
        Piece::CrcDiffers
    }
}

/// Whether `text` is a Blue Raven's USB download: one of its first [`RECOGNITION_LINES`] lines
/// opens with `@ LOG_HIR` or `@ LOG_LOW` and closes with a CRC that checks.
pub(crate) fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .take(RECOGNITION_LINES)
        .any(|(_, line)| {
            let line = line.trim();
            (line.starts_with("@ LOG_HIR") || line.starts_with("@ LOG_LOW"))
                && line.match_indices(OPENING).count() == 1
                && matches!(piece(line), Piece::Checked(_))
        })
}

/// A record's groups: each its label (`None` for the high-rate record's values) and its values'
/// count.
type Layout = Vec<(Option<String>, usize)>;

/// One record, read.
struct Record {
    kind: String,
    layout: Layout,
    /// The logger's date and time, ms after 1970-01-01 on its own clock.
    at_ms: i64,
    year: i64,
    month: i64,
    day: i64,
    /// The time of day, ms after the logger's midnight.
    time_of_day_ms: i64,
    sync: i64,
    /// The values after the sync code, in the record's order.
    values: Vec<f64>,
}

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar (H. Hinnant,
/// *chrono-Compatible Low-Level Date Algorithms*, `days_from_civil`, public domain).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let shifted_month = (month + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// A whole number as the logger writes it: an optional `-`, then at most 15 digits, so that it
/// is exact as an `f64`.
fn whole(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    (!digits.is_empty() && digits.len() <= 15 && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| token.parse().ok())
        .flatten()
}

/// A register as the logger writes it: one to eight hexadecimal digits.
fn register(token: &str) -> Option<i64> {
    (!token.is_empty() && token.len() <= 8 && token.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| i64::from_str_radix(token, 16).ok())
        .flatten()
}

/// A time of day, `H:MM:SS` or `HH:MM:SS` with up to three decimals, as ms after midnight.
fn time_of_day_ms(token: &str) -> Option<i64> {
    let mut parts = token.split(':');
    let (hours, minutes, seconds) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let (whole_seconds, fraction) = seconds.split_once('.').unwrap_or((seconds, ""));
    let digits = |text: &str, most: usize| {
        (!text.is_empty() && text.len() <= most && text.bytes().all(|byte| byte.is_ascii_digit()))
            .then(|| text.parse::<i64>().ok())
            .flatten()
    };
    let hours = digits(hours, 2).filter(|hours| *hours < 24)?;
    let minutes = digits(minutes, 2).filter(|minutes| *minutes < 60)?;
    let whole_seconds = digits(whole_seconds, 2).filter(|seconds| *seconds < 60)?;
    let milliseconds = if fraction.is_empty() {
        (!seconds.contains('.')).then_some(0)?
    } else {
        // `.5` is 500 ms: padded to three digits.
        digits(fraction, 3)? * 10_i64.pow(3 - fraction.len() as u32)
    };
    Some(((hours * 60 + minutes) * 60 + whole_seconds) * 1_000 + milliseconds)
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// Reads a record whose CRC checked; any fault in it refuses the file, as the logger wrote it.
fn record(text: &str, line: usize) -> Result<Record, LogError> {
    // The CRC checked, so `CRC:` is in the text.
    let crc_at = text.rfind("CRC:").unwrap_or(text.len());
    let tokens: Vec<&str> = text[..crc_at].split_ascii_whitespace().collect();
    let kind = tokens.get(1).copied().unwrap_or_default();
    if kind != HIGH_RATE && kind != LOW_RATE {
        return Err(syntax(
            line,
            format!(
                "a record `{}` whose CRC checks, which the manual doesn't describe: only \
                 `@ LOG_HIR` and `@ LOG_LOW` are read",
                short(kind)
            ),
        ));
    }
    let header = |index: usize, what: &str| {
        tokens.get(index).copied().ok_or_else(|| {
            syntax(
                line,
                format!("the record ends before its {what}: `{}`", short(text)),
            )
        })
    };
    let stated = header(2, "length")?;
    if whole(stated).and_then(|length| usize::try_from(length).ok())
        != Some(text.len() + LINE_END_BYTES)
    {
        return Err(syntax(
            line,
            format!(
                "the record states its length as `{}`, and it is {} bytes with its line end",
                short(stated),
                text.len() + LINE_END_BYTES
            ),
        ));
    }
    let date = |index: usize, what: &str| -> Result<i64, LogError> {
        let token = header(index, what)?;
        whole(token).ok_or_else(|| {
            syntax(
                line,
                format!("the record's {what} is `{}`, not a number", short(token)),
            )
        })
    };
    let (year, month, day) = (date(3, "year")?, date(4, "month")?, date(5, "day")?);
    if !(1..=9_999).contains(&year)
        || !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
    {
        return Err(syntax(
            line,
            format!("the record's date, {year} {month} {day}, isn't one"),
        ));
    }
    let clock = header(6, "time")?;
    let time_of_day_ms = time_of_day_ms(clock).ok_or_else(|| {
        syntax(
            line,
            format!(
                "the record's time is `{}`, not `HH:MM:SS.fff`",
                short(clock)
            ),
        )
    })?;
    let sync = date(7, "sync code")?;
    let mut layout: Layout = Vec::new();
    let mut values = Vec::new();
    if kind == HIGH_RATE {
        layout.push((None, 0));
    }
    for &token in &tokens[8..] {
        if token.ends_with(':') && kind == LOW_RATE {
            if layout.len() >= MAX_GROUPS {
                return Err(syntax(
                    line,
                    format!(
                        "the record's group `{}` is past the {MAX_GROUPS} groups a record may \
                         hold",
                        short(token)
                    ),
                ));
            }
            if token.len() > MAX_LABEL_BYTES {
                return Err(syntax(
                    line,
                    format!(
                        "the record's label `{}` is longer than the {MAX_LABEL_BYTES} bytes a \
                         label may be",
                        short(token)
                    ),
                ));
            }
            if layout
                .iter()
                .any(|(label, _)| label.as_deref() == Some(token))
            {
                return Err(syntax(
                    line,
                    format!("the record holds the group `{}` twice", short(token)),
                ));
            }
            layout.push((Some(token.to_owned()), 0));
            continue;
        }
        let Some((label, count)) = layout.last_mut() else {
            return Err(syntax(
                line,
                format!(
                    "the low-rate record holds `{}` before its first group's label",
                    short(token)
                ),
            ));
        };
        let value = if label.as_deref() == Some(REGISTERS) {
            register(token)
        } else {
            whole(token)
        };
        let Some(value) = value else {
            return Err(syntax(
                line,
                format!(
                    "the record's value `{}`{} isn't {}",
                    short(token),
                    label
                        .as_deref()
                        .map(|label| format!(" in `{}`", short(label)))
                        .unwrap_or_default(),
                    if label.as_deref() == Some(REGISTERS) {
                        "a register in hexadecimal"
                    } else {
                        "a whole number"
                    }
                ),
            ));
        };
        *count += 1;
        values.push(value as f64);
        if values.len() > MAX_VALUES {
            return Err(syntax(
                line,
                format!("the record holds more than {MAX_VALUES} values"),
            ));
        }
    }
    Ok(Record {
        kind: kind.to_owned(),
        layout,
        at_ms: days_from_civil(year, month, day) * 86_400_000 + time_of_day_ms,
        year,
        month,
        day,
        time_of_day_ms,
        sync,
        values,
    })
}

/// A column's name, its quantity, and the divisor then the factor that take a value the file
/// writes to the quantity's unit, as a [`Field`]'s.
type Column = (String, Quantity, f64, f64);

/// A column for each value of a record laid out as `layout`, and a note for each group read
/// otherwise than by the manual whole.
fn columns(kind: &str, layout: &Layout) -> (Vec<Column>, Vec<String>) {
    let mut columns = Vec::new();
    let mut notes = Vec::new();
    for (label, count) in layout {
        let manual = manual_fields(kind, label.as_deref());
        let named = label.as_deref().unwrap_or(kind);
        let read = if *count >= manual.len() {
            manual.len()
        } else {
            0
        };
        for field in &manual[..read] {
            columns.push((
                field.column.to_owned(),
                field.quantity,
                field.per,
                field.unit,
            ));
        }
        let as_written = if label.as_deref() == Some(REGISTERS) {
            Quantity::Number
        } else {
            Quantity::Unrecognized
        };
        for place in read + 1..=*count {
            columns.push((format!("{named} {place}"), as_written, 1.0, 1.0));
        }
        let what = if label.is_some() {
            format!("`{named}` holds")
        } else {
            "the high-rate record holds".to_owned()
        };
        if manual.is_empty() {
            notes.push(format!(
                "{what} {count} values in a group the manual doesn't list: kept as written"
            ));
        } else if *count > manual.len() {
            notes.push(format!(
                "{what} {count} values where the manual lists {}: the manual's read in its \
                 order, the other {} kept as written",
                manual.len(),
                count - manual.len()
            ));
        } else if *count < manual.len() {
            notes.push(format!(
                "{what} {count} values where the manual lists {}: kept as written, as nothing \
                 says which is missing",
                manual.len()
            ));
        }
    }
    (columns, notes)
}

/// Reads a Blue Raven's USB download.
///
/// # Errors
///
/// [`LogError::Unrecognized`] if no line among the first 1,000 is a record whose CRC checks;
/// [`LogError::Syntax`] for a record whose CRC checks but which can't be read (see the module's
/// docs).
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::Unrecognized {
            message: format!(
                "not a {FORMAT}: none of its first {RECOGNITION_LINES} lines is an `@ LOG_HIR` or \
                 `@ LOG_LOW` record whose CRC checks"
            ),
        });
    }
    let mut first: Option<Record> = None;
    let mut times_ms: Vec<i64> = Vec::new();
    let mut header_values: [Vec<f64>; 5] = Default::default();
    let mut values: Vec<Vec<f64>> = Vec::new();
    let mut first_line = 0;
    let (mut no_crc, mut crc_differs, mut other_lines) = (0_usize, 0_usize, 0_usize);
    for (line, text) in numbered_lines(text) {
        let (text_before, line_pieces) = pieces(text);
        let mut any = false;
        for piece in line_pieces {
            any = true;
            let record = match piece {
                Piece::NoCrc => {
                    no_crc += 1;
                    continue;
                }
                Piece::CrcDiffers => {
                    crc_differs += 1;
                    continue;
                }
                Piece::Checked(text) => record(text, line)?,
            };
            if let Some(first) = &first {
                if record.kind != first.kind || record.layout != first.layout {
                    return Err(syntax(
                        line,
                        format!(
                            "a record laid out unlike the first, on line {}: a download holds \
                             one kind of record, each with the same groups",
                            first_line
                        ),
                    ));
                }
                let previous = times_ms.last().copied().unwrap_or(i64::MIN);
                if record.at_ms <= previous {
                    return Err(syntax(
                        line,
                        format!(
                            "the record's time, {}, doesn't come after the one before",
                            clock_text(&record)
                        ),
                    ));
                }
            } else {
                values = vec![Vec::new(); record.values.len()];
            }
            times_ms.push(record.at_ms);
            for (column, value) in header_values.iter_mut().zip([
                record.year as f64,
                record.month as f64,
                record.day as f64,
                record.time_of_day_ms as f64 / 1_000.0,
                record.sync as f64,
            ]) {
                column.push(value);
            }
            for (column, value) in values.iter_mut().zip(&record.values) {
                column.push(*value);
            }
            if first.is_none() {
                first_line = line;
                first = Some(record);
            }
        }
        if text_before || (!any && !text.trim().is_empty()) {
            other_lines += 1;
        }
    }
    // Recognition found a record whose CRC checks, which the loop reads or refuses, so a first
    // record is always there; refused rather than unwrapped all the same.
    let Some(first) = first else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "no record's CRC checks".to_owned(),
        });
    };
    let zero_ms = times_ms[0];
    let time_s: Vec<f64> = times_ms
        .iter()
        .map(|at| (at - zero_ms) as f64 / 1_000.0)
        .collect();
    let mut channels = Vec::new();
    for ((column, quantity), values) in [
        ("Year", Quantity::Number),
        ("Month", Quantity::Number),
        ("Day", Quantity::Number),
        ("Time", Quantity::TimeOfDay),
        ("Sync", Quantity::Number),
    ]
    .into_iter()
    .zip(header_values)
    {
        channels.push(Channel {
            column: column.to_owned(),
            quantity,
            time_s: time_s.clone(),
            values,
        });
    }
    let (columns, layout_notes) = columns(&first.kind, &first.layout);
    for ((column, quantity, per, unit), counts) in columns.into_iter().zip(values) {
        channels.push(Channel {
            column,
            quantity,
            time_s: time_s.clone(),
            values: counts.into_iter().map(|count| count / per * unit).collect(),
        });
    }
    let mut notes = vec![format!(
        "a {} download: the clock counts from the first record's time on the logger's clock, \
         {}. The manual says that clock's date starts from the firmware's build date unless \
         the phone has set it",
        if first.kind == HIGH_RATE {
            "high-rate"
        } else {
            "low-rate"
        },
        clock_text(&first)
    )];
    notes.extend(layout_notes);
    let temperature_read = first
        .layout
        .iter()
        .any(|(label, count)| label.as_deref() == Some("Bo:") && *count >= 2);
    if temperature_read {
        notes.push(
            "`Bo: temperature` kept as written: the manual states no unit for it in the download"
                .to_owned(),
        );
    }
    if no_crc > 0 {
        notes.push(if no_crc == 1 {
            "1 piece that opens `@ LOG_` but closes with no CRC, a record the capture cut or ran \
             into the next, dropped"
                .to_owned()
        } else {
            format!(
                "{no_crc} pieces that open `@ LOG_` but close with no CRC, records the capture cut \
                 or ran into the next, dropped"
            )
        });
    }
    if crc_differs > 0 {
        notes.push(if crc_differs == 1 {
            "1 piece whose CRC differs from its text, dropped".to_owned()
        } else {
            format!("{crc_differs} pieces whose CRC differs from their text, dropped")
        });
    }
    if other_lines > 0 {
        notes.push(format!(
            "{other_lines} {} holding text other than a record, skipped",
            plural(other_lines, "line")
        ));
    }
    Ok(FlightLog {
        format: LogFormat::BlueRavenUsb,
        logger: "Featherweight Blue Raven".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            apogee_m: None,
            ground_elevation_msl_m: None,
            samples: None,
            figures: Vec::new(),
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

fn clock_text(record: &Record) -> String {
    let ms = record.time_of_day_ms;
    format!(
        "{} {} {} {:02}:{:02}:{:02}.{:03}",
        record.year,
        record.month,
        record.day,
        ms / 3_600_000,
        ms / 60_000 % 60,
        ms / 1_000 % 60,
        ms % 1_000
    )
}

fn plural(count: usize, word: &str) -> String {
    if count == 1 {
        word.to_owned()
    } else {
        format!("{word}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import;

    /// Two invented high-rate records, their CRCs and lengths worked out apart from this reader.
    const HIGH: &str = "@ LOG_HIR  139 2026  5 23 10:15:00.120   20    -15     42     -7     -1      3     98  -3000   1500    -40  29812      0      0 CRC: 7730\r\n\
                        @ LOG_HIR  139 2026  5 23 10:15:00.122   22    -12     40     -6     -2      4     99  -3001   1500    -41  29811      0      0 CRC: A68A\r\n";

    /// Two invented low-rate records, laid out as the sample logs' firmware writes them.
    const LOW: &str = "@ LOG_LOW  211 2026  5 23 10:15:00.140  40 Bo:  7212 49655 V:  4105    22    21    20    23 Vel:     0     0     0 Pos:     0     0     0 ang:   12    5    0 FER:  400  1c1   c3   c1   c3   0   0   0 CRC: F7EB\r\n\
                       @ LOG_LOW  211 2026  5 23 10:15:00.160  60 Bo:  7212 49654 V:  4105    22    21    20    23 Vel:     1     0     0 Pos:     0     0     0 ang:   12    5    0 FER:  400  1c1   c3   c1   c3   0   0   0 CRC: F1E8\r\n";

    /// A record of `kind` holding `rest`, its length and CRC worked out by this reader's CRC.
    fn line(kind: &str, rest: &str) -> String {
        (50..1_000)
            .find_map(|length| {
                let body = format!("@ {kind} {length:>4} {rest}");
                let text = format!("{body} CRC: {:04X}", crc16_buypass(body.as_bytes()));
                (text.len() + 2 == length).then_some(text)
            })
            .expect("a length fits")
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .unwrap_or_else(|| panic!("no channel `{column}`"))
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    #[test]
    fn the_crc_is_buypass() {
        // The cataloged check value of CRC-16/UMTS, also called BUYPASS.
        assert_eq!(crc16_buypass(b"123456789"), 0xFEE8);
    }

    #[test]
    fn reads_high_rate_records_by_the_manual() {
        let log = read(HIGH).expect("reads");
        assert_eq!(log.format, LogFormat::BlueRavenUsb);
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        assert_eq!(log.channels.len(), 5 + 12);
        let gyro = channel(&log, "Gyro X");
        assert_eq!(gyro.quantity, Quantity::AngularRate);
        assert_eq!(gyro.time_s, vec![0.0, 0.002]);
        assert!(close(gyro.values[0], -0.15));
        let accel = channel(&log, "Accel Z");
        assert_eq!(accel.quantity, Quantity::Acceleration);
        assert!(close(accel.values[1], 0.99 * STANDARD_GRAVITY_MPS2));
        let magnitude = channel(&log, "Quaternion magnitude");
        assert!(close(magnitude.values[0], 29_812.0 / 30_000.0));
        let extra = channel(&log, "LOG_HIR 11");
        assert_eq!(extra.quantity, Quantity::Unrecognized);
        assert_eq!(extra.values, vec![0.0, 0.0]);
        assert_eq!(channel(&log, "Sync").values, vec![20.0, 22.0]);
        assert_eq!(
            channel(&log, "Time").values,
            vec![36_900.12, 36_900.122],
            "10:15:00.120 as seconds after midnight"
        );
        assert!(
            log.notes
                .iter()
                .any(|note| note.starts_with("the high-rate record holds 12 values where the manual lists 10: the manual's read in its order, the other 2 kept as written")),
            "{:?}",
            log.notes
        );
        assert!(log.notes[0].contains("2026 5 23 10:15:00.120"));
    }

    #[test]
    fn reads_low_rate_groups_and_keeps_short_ones_as_written() {
        let log = read(LOW).expect("reads");
        let pressure = channel(&log, "Bo: pressure");
        assert_eq!(pressure.quantity, Quantity::Pressure);
        assert!(close(pressure.values[0], 49_655.0 / 50_000.0 * 101_325.0));
        let temperature = channel(&log, "Bo: temperature");
        assert_eq!(temperature.quantity, Quantity::Unrecognized);
        assert_eq!(temperature.values[0], 7_212.0);
        // Five voltages where the manual lists six: none read as the manual's.
        assert!(
            log.channels
                .iter()
                .all(|channel| channel.column != "V: battery")
        );
        let first = channel(&log, "V: 1");
        assert_eq!(first.quantity, Quantity::Unrecognized);
        assert_eq!(first.values[0], 4_105.0);
        assert!(log.channels.iter().all(|channel| channel.column != "V: 6"));
        let up = channel(&log, "Vel: up");
        assert_eq!(up.quantity, Quantity::Speed);
        assert!(close(up.values[1], FOOT_M));
        assert_eq!(
            channel(&log, "Pos: altitude").quantity,
            Quantity::InertialAltitude
        );
        assert!(close(channel(&log, "ang: tilt").values[0], 1.2));
        assert!(close(channel(&log, "ang: roll").values[0], 5.0));
        // Eight registers where the manual lists five: its five, then three kept.
        let rocket = channel(&log, "FER: rocket");
        assert_eq!(rocket.quantity, Quantity::Number);
        assert_eq!(rocket.values[0], f64::from(0x400));
        assert_eq!(channel(&log, "FER: Apo").values[0], f64::from(0x1c1));
        assert_eq!(channel(&log, "FER: 8").quantity, Quantity::Number);
        assert!(
            log.channels
                .iter()
                .all(|channel| channel.column != "FER: 9")
        );
        for wanted in [
            "`V:` holds 5 values where the manual lists 6: kept as written, as nothing says which is missing",
            "`FER:` holds 8 values where the manual lists 5: the manual's read in its order, the other 3 kept as written",
            "`Bo: temperature` kept as written",
        ] {
            assert!(
                log.notes.iter().any(|note| note.starts_with(wanted)),
                "{wanted}: {:?}",
                log.notes
            );
        }
    }

    #[test]
    fn a_short_high_rate_record_is_kept_as_written() {
        let rest = "2026  5 23 10:15:00.120   20    -15     42     -7     -1      3     98  -3000   1500    -40";
        let log = read(&line("LOG_HIR", rest)).expect("reads");
        assert!(
            log.channels
                .iter()
                .all(|channel| channel.column != "Gyro X")
        );
        assert_eq!(channel(&log, "LOG_HIR 1").values[0], -15.0);
        assert_eq!(channel(&log, "LOG_HIR 9").quantity, Quantity::Unrecognized);
    }

    #[test]
    fn drops_pieces_whose_crc_is_missing_or_differs() {
        let mut lines: Vec<String> = HIGH.lines().map(str::to_owned).collect();
        // A record whose CRC and line end the capture lost runs into the next one.
        let cut = lines[0].rfind(" CRC:").expect("a CRC");
        let run_on = format!("{}{}", &lines[0][..cut], lines[1]);
        // A record with one digit changed: its CRC no longer checks.
        let damaged = lines[1].replacen("-12", "-13", 1);
        let later = line(
            "LOG_HIR",
            "2026  5 23 10:15:00.124   24    -12     40     -6     -2      4     99  -3001   1500    -41  29811      0      0",
        );
        lines = vec![run_on, damaged, "@ BLR_STAT and the like".to_owned(), later];
        let log = read(&lines.join("\r\n")).expect("reads");
        assert_eq!(channel(&log, "Sync").values, vec![22.0, 24.0]);
        for wanted in [
            "1 piece that opens `@ LOG_` but closes with no CRC",
            "1 piece whose CRC differs from its text, dropped",
            "1 line holding text other than a record, skipped",
        ] {
            assert!(
                log.notes.iter().any(|note| note.starts_with(wanted)),
                "{wanted}: {:?}",
                log.notes
            );
        }
    }

    fn refusal(text: &str) -> (usize, String) {
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("not refused by a line: {other:?}"),
        }
    }

    #[test]
    fn refuses_a_checked_record_whose_length_isnt_its_own() {
        // Its CRC checks, but it states 140 bytes for a record of 139.
        let text = format!(
            "{HIGH}@ LOG_HIR  140 2026  5 23 10:15:00.124   24    -12     40     -6     -2      4     99  -3001   1500    -41  29811      0      0 CRC: 8AC7\r\n"
        );
        let (line, message) = refusal(&text);
        assert_eq!(line, 3);
        assert!(
            message.contains("`140`") && message.contains("139 bytes"),
            "{message}"
        );
    }

    #[test]
    fn refuses_a_clock_that_doesnt_increase() {
        let rest = "2026  5 23 10:15:00.122   24    -12     40     -6     -2      4     99  -3001   1500    -41  29811      0      0";
        let (line, message) = refusal(&format!("{HIGH}{}\r\n", line("LOG_HIR", rest)));
        assert_eq!(line, 3);
        assert!(message.contains("doesn't come after"), "{message}");
    }

    #[test]
    fn a_clock_crosses_midnight_by_its_date() {
        let first = line("LOG_LOW", "2026 12 31 23:59:59.990  40 Bo:  7212 49655");
        let second = line("LOG_LOW", "2027  1  1 00:00:00.010  60 Bo:  7212 49655");
        let log = read(&format!("{first}\n{second}\n")).expect("reads");
        let time = &channel(&log, "Bo: pressure").time_s;
        assert!(close(time[1], 0.02), "{time:?}");
    }

    #[test]
    fn refuses_a_second_layout_or_kind() {
        let (at, message) = refusal(&format!("{HIGH}{LOW}"));
        assert_eq!(at, 3);
        assert!(message.contains("unlike the first, on line 1"), "{message}");
        let fewer = line(
            "LOG_LOW",
            "2026  5 23 10:15:00.180  80 Bo:  7212 49655 V:  4105    15    14    14",
        );
        let (at, _) = refusal(&format!("{LOW}{fewer}\r\n"));
        assert_eq!(at, 3);
    }

    #[test]
    fn refuses_what_a_checked_record_cant_be() {
        let refused = |rest: &str, kind: &str| refusal(&format!("{HIGH}{}\r\n", line(kind, rest)));
        let values = "   26    -12     40     -6     -2      4     99  -3001   1500    -41  29811      0      0";
        let (_, message) = refused(&format!("2026  2 30 10:15:00.126 {values}"), "LOG_HIR");
        assert!(message.contains("isn't one"), "{message}");
        let (_, message) = refused(&format!("2026  5 23 10:15:60.126 {values}"), "LOG_HIR");
        assert!(message.contains("not `HH:MM:SS.fff`"), "{message}");
        let (_, message) = refused(
            "2026  5 23 10:15:00.126   26    -12     4.5     -6     -2      4     99  -3001   1500    -41  29811      0      0",
            "LOG_HIR",
        );
        assert!(
            message.contains("`4.5`") && message.contains("a whole number"),
            "{message}"
        );
        let (_, message) = refused(&format!("2026  5 23 10:15:00.126 {values}"), "LOG_GPS");
        assert!(message.contains("`LOG_GPS`"), "{message}");
        let many = " 1".repeat(MAX_VALUES + 1);
        let (_, message) = refused(&format!("2026  5 23 10:15:00.126   26{many}"), "LOG_HIR");
        assert!(message.contains("more than 256 values"), "{message}");
        let (_, message) = refusal(&line(
            "LOG_LOW",
            "2026  5 23 10:15:00.140  40 7212 Bo: 49655",
        ));
        assert!(
            message.contains("before its first group's label"),
            "{message}"
        );
        let labels: String = (0..=MAX_GROUPS).map(|k| format!(" g{k}: 1")).collect();
        let (_, message) = refusal(&line(
            "LOG_LOW",
            &format!("2026  5 23 10:15:00.140  40{labels}"),
        ));
        assert!(
            message.contains("`g16:` is past the 16 groups"),
            "{message}"
        );
        let long = format!("{}:", "x".repeat(MAX_LABEL_BYTES));
        let (_, message) = refusal(&line(
            "LOG_LOW",
            &format!("2026  5 23 10:15:00.140  40 {long} 1"),
        ));
        assert!(message.contains("longer than the 16 bytes"), "{message}");
        // At the caps, the record is read: 16 groups, the last one's label 16 bytes.
        let labels: String = (1..MAX_GROUPS)
            .map(|k| format!(" g{k}: 1"))
            .chain([format!(" {}: 1", "x".repeat(MAX_LABEL_BYTES - 1))])
            .collect();
        let log = read(&line(
            "LOG_LOW",
            &format!("2026  5 23 10:15:00.140  40{labels}"),
        ))
        .unwrap();
        assert!(log.channels.iter().any(|c| c.column.starts_with("xxxx")));
        let (_, message) = refusal(&line("LOG_LOW", "2026  5 23 10:15:00.140  40 FER: 60g"));
        assert!(message.contains("hexadecimal"), "{message}");
    }

    #[test]
    fn a_file_with_no_checked_record_isnt_one() {
        let damaged = HIGH.replace("  1500 ", "  1501 ");
        assert!(matches!(read(&damaged), Err(LogError::Unrecognized { .. })));
        assert!(!recognizes("@ LOG_HIR  139 no CRC here"));
    }

    #[test]
    fn import_recognizes_a_download() {
        let log = import::read(LOW.as_bytes()).expect("reads");
        assert_eq!(log.format, LogFormat::BlueRavenUsb);
        assert_eq!(log.format.name(), "Blue Raven USB download");
    }
}
