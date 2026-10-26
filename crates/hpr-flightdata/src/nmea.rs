//! A GPS receiver's NMEA 0183 sentences, as a logger writes them to a text file.
//!
//! An NMEA 0183 receiver writes text, one sentence a line: a `$`, a five-character address (a
//! two-letter talker, `GP` for GPS, `GN` for several systems at once, then a three-letter sentence
//! type), the sentence's fields separated by commas, a `*` and a two-digit hexadecimal checksum,
//! the exclusive-or of every character between the `$` and the `*`, and CR LF:
//!
//! ```text
//! $GPGGA,123519.000,4807.0380,N,01131.0000,E,1,08,0.9,545.4,M,46.9,M,,0000*59
//! $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
//! $GPRMC,123519.000,A,4807.0380,N,01131.0000,E,022.4,084.4,230394,,,A*62
//! ```
//!
//! The standard itself is sold by the National Marine Electronics Association; this reader works
//! from the public description gpsd's authors keep, Eric S. Raymond's *NMEA Revealed*
//! (<https://gpsd.gitlab.io/gpsd/NMEA.html>, as last updated 2026-10-11): the sentence layout and
//! checksum under "NMEA Encoding Conventions" and "NMEA 0183 physical protocol layer", the four
//! sentences read under "NMEA Standard Sentences" (`GGA`, `RMC`, `GSA`, `GSV`), the FAA mode
//! letters under "Error status indications", and the clock under "Dates and times".
//!
//! **The sample files.** As of 2026-10-12, among the project's sample logs, 12 distinct files are
//! NMEA 0183, all from one kind of receiver, all from the private collection of flights this
//! project checks against. Each holds talker `GP` only, and one second's sentences in the same
//! order: `GGA`, `GSA`, on one second in five a burst of three `GSV`, then `RMC`, the `RMC` with
//! the `GGA`'s time in every one of them. Lines end in CR LF. Every complete sentence carries a
//! checksum, and all but two check out. Their quirks, each handled below:
//!
//! - 10 open with a line that isn't a whole sentence: in 4, bytes of noise before a sentence's
//!   `$`; in 5, a few bytes of noise alone; in 1, a sentence cut at its start.
//! - 2 end in bytes that aren't text, NUL among them, on the line of their last sentence: in one
//!   that sentence is cut off before its checksum, in the other its checksum fails.
//! - 1 more holds a sentence with bytes that aren't text among its fields, whose checksum fails.
//! - Before its first fix the receiver writes `GGA` and `RMC` with an empty time (in 11 files),
//!   then a time with a placeholder date on `RMC` (one of two fixed dates, years before the
//!   flights), then the real date while the status is still `V`, void, and only then status `A`, a
//!   valid fix.
//! - 9 step back 2 s once (one by 1.999 s): in 7 a few minutes after the first fix, in 2 before
//!   it. Plausibly the receiver taking up the leap-second count the satellites broadcast "once
//!   every 12.5 minutes", reported UTC being "slightly inaccurate between a cold boot … and the
//!   following subframe broadcast" (*NMEA Revealed*, "Dates and times"); this is an inference.
//! - `GGA`'s fix quality is 0, 1 or, in 9 files, 6, dead reckoning, when `RMC`'s mode is `E`.
//! - One file, 80 lines and 10 s long, never gets a fix: its positions are all gaps.
//! - None crosses midnight UTC.
//!
//! **Sentences.** A line may hold noise around a sentence, or two sentences run together where a
//! line end was lost. So a sentence is read wherever a `$` opens one whose characters are
//! printable ASCII (none of `*`, `!`, `\` or `~` among its fields), ending at the first `*`
//! with two hexadecimal digits that equal its checksum; anything else on the line is left out,
//! and counted in a note. A line that holds no such sentence is skipped and counted, by kind:
//! no `$` at all; a sentence cut off before its checksum; or a checksum that fails, or bytes the
//! standard doesn't allow. A sentence without a checksum isn't read: every sample has one, and
//! *NMEA Revealed* calls it mandatory. Sentences longer than the standard's 82 characters are
//! read all the same. NUL bytes, and bytes that aren't UTF-8 (each a U+FFFD after decoding), are
//! counted in a note.
//!
//! A file is taken for NMEA 0183 when at least 3 of its first 50 lines that aren't blank open
//! with a sentence whose checksum checks out (or every line does, in a shorter file), so that a
//! first line of noise doesn't hide it, and a log of another kind that quotes sentences inside
//! its own lines, as a ground station's may, isn't taken for one.
//!
//! **Channels.** The file has no barometer: the record's primary track is empty, with a note, and
//! every value is a channel, named for its sentence's address and field, such as `GPGGA altitude`
//! or `GPGSV PRN 12 SNR` (the PRN without its leading zeros). The address keeps a file mixing talkers, as a multi-system receiver
//! writes `GNGGA` beside `GPGSV` and `GLGSV`, from mixing their channels. Each sentence read
//! adds one sample to each of its channels, in the file's order; an empty field is a gap, `NaN`,
//! and a field that isn't what its channel holds is a gap counted in a note.
//!
//! | Sentence | Channel | Quantity | From the field |
//! |---|---|---|---|
//! | `GGA` | `latitude`, `longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`], degrees | `ddmm.mmmm` and `N`/`S`, `dddmm.mmmm` and `E`/`W`, made signed degrees, south and west negative |
//! | `GGA` | `fix quality` | [`Quantity::Number`] | as written: 0 no fix, 1 GPS, 2 DGPS, 3 PPS, 4 RTK, 5 float RTK, 6 estimated (dead reckoning), 7 manual, 8 simulation |
//! | `GGA` | `satellites used`, `HDOP` | [`Quantity::Number`] | as written |
//! | `GGA` | `altitude` | [`Quantity::GnssAltitude`], m above mean sea level | meters, when the unit field is `M`, as the layout gives |
//! | `GGA` | `geoid separation` | [`Quantity::Distance`], m | meters, when its unit field is `M` |
//! | `GGA` | `DGPS age`, `DGPS station` | [`Quantity::Number`] | as written: seconds since the last differential correction, and the station's id |
//! | `RMC` | `status` | [`Quantity::Number`] | 1 for `A`, a valid fix; 0 for `V`, void |
//! | `RMC` | `latitude`, `longitude` | as `GGA`'s | as `GGA`'s |
//! | `RMC` | `speed` | [`Quantity::Speed`], m/s | knots, × 1852/3600 (the international nautical mile, 1852 m exactly) |
//! | `RMC` | `course` | [`Quantity::Angle`], degrees | degrees from true north, as written |
//! | `RMC` | `magnetic variation` | [`Quantity::Angle`], degrees | degrees and `E`/`W`, east positive |
//! | `RMC` | `mode` | [`Quantity::Number`] | the FAA mode letter as `GGA`'s fix quality code for the same mode: `N` 0, `A` 1, `D` 2, `P` 3, `R` 4, `F` 5, `E` 6, `M` 7, `S` 8 |
//! | `GSA` | `selection` | [`Quantity::Number`] | 1 for `A`, automatic 2D/3D; 0 for `M`, manual |
//! | `GSA` | `fix type` | [`Quantity::Number`] | as written: 1 no fix, 2 2D, 3 3D |
//! | `GSA` | `satellites used` | [`Quantity::Number`] | how many of its 12 satellite fields are filled |
//! | `GSA` | `PDOP`, `HDOP`, `VDOP` | [`Quantity::Number`] | as written |
//! | `GSV` | `satellites in view` | [`Quantity::Number`] | as written, once a burst, from its first sentence |
//! | `GSV` | `PRN n elevation`, `PRN n azimuth` | [`Quantity::Angle`], degrees | as written, azimuth from true north |
//! | `GSV` | `PRN n SNR` | [`Quantity::Number`] | as written, dB, 00 to 99 |
//!
//! `RMC`'s magnetic variation and mode, and `GGA`'s DGPS age and station, are left out when every
//! sentence leaves them empty (no sample fills the variation). A `GSV` sentence's message count
//! and number order the burst, and `GSA`'s list of satellites used is kept as its count; fields
//! past the ones in the table, such as NMEA 4.10's signal and system ids, are left out and
//! counted in a note. An altitude or geoid separation whose unit field isn't `M` is kept apart,
//! as written, in a channel such as `GPGGA altitude, unit "F"` ([`Quantity::Unrecognized`]), with
//! a note. Sentence types other than these four are counted in a note, their fields left out.
//!
//! **Clocks.** `GGA` and `RMC` state the time of day in UTC, `hhmmss.sss`; `RMC` states the date,
//! `ddmmyy` (a year `yy` of 80 or more is 19`yy`, else 20`yy`, as GPS began in 1980). A sample's
//! time is its sentence's, in seconds from the log's zero, the first sentence with a time. The
//! day is the date of the last `RMC` with status `A`: a void `RMC`'s date is a placeholder in the
//! samples, so it is left out, with a note. Before such an `RMC`, and in a file without one,
//! times are times of day alone. A time of day more than 12 hours before the last is the next
//! day, and one more than 12 hours after it the day before; where an `RMC` with status `A` dates
//! it, the date settles the day instead. `GSA` and `GSV` carry no time: each takes the time of the
//! `GGA` or `RMC` before it, its second's, as the samples' order shows, with a note giving the
//! file's own evidence; one before any time has a gap for its time, as does a `GGA` or `RMC` whose
//! time is empty. Each channel keeps its samples in the file's order, so a time may go back,
//! as the samples' 2 s step does: a note says how often and how far.
//!
//! **Hostile input.** Work is linear in the file: a line is cut at its `$`s once. At most
//! [`MAX_ADDRESSES`] sentence addresses are read and [`MAX_SATELLITES`] satellites a `GSV`
//! address, the rest counted; notes about single channels are capped; text quoted in a note is
//! cut short.

use std::collections::{BTreeMap, BTreeSet};

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{Notes, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "NMEA 0183";

/// How many lines that aren't blank [`recognizes`] looks at, from the file's start.
const RECOGNIZE_LINES: usize = 50;

/// How many of those must hold a sentence whose checksum checks out.
const RECOGNIZE_SENTENCES: usize = 3;

/// The most characters of a line [`recognizes`] looks at.
const RECOGNIZE_LINE_BYTES: usize = 4096;

/// The most sentence addresses of the four types read, such as `GPGGA` and `GNGGA`; sentences at
/// further addresses are counted in a note. The samples have four.
pub const MAX_ADDRESSES: usize = 16;

/// The most satellites one `GSV` address gives channels to; further satellites are counted in a
/// note. The samples see at most 28.
pub const MAX_SATELLITES: usize = 64;

/// The most other sentence types a note names, and the most it counts apart.
const MAX_OTHER_TYPES: usize = 8;

/// Meters per second in a knot: the international nautical mile is 1852 m exactly (BIPM, *The
/// International System of Units*, 9th ed., 2019, Table 8), so a knot is 1852/3600 m/s.
const KNOT_M_S: f64 = 1852.0 / 3600.0;

/// Nanoseconds in a day.
const DAY_NS: i128 = 86_400_000_000_000;

/// Whether the text is NMEA 0183: among its first [`RECOGNIZE_LINES`] lines that aren't blank, at
/// least [`RECOGNIZE_SENTENCES`] open with a sentence whose checksum checks out, or, in a shorter
/// file, every line does. A first line of noise, as most samples have, doesn't stop it; a file
/// that quotes sentences inside lines of its own, as a ground station's log may, isn't NMEA.
pub(crate) fn recognizes(text: &str) -> bool {
    let (mut lines, mut found) = (0usize, 0usize);
    for (_, line) in numbered_lines(text) {
        if line.trim().is_empty() {
            continue;
        }
        lines += 1;
        let opens = head(line.trim_start(), RECOGNIZE_LINE_BYTES)
            .strip_prefix('$')
            .and_then(|rest| rest.split('$').next())
            .is_some_and(|piece| sentence(piece).is_ok());
        if opens {
            found += 1;
        }
        if found >= RECOGNIZE_SENTENCES {
            return true;
        }
        if lines >= RECOGNIZE_LINES {
            return false;
        }
    }
    found > 0 && found == lines
}

/// The start of a line, at most `bytes` long, cut at a character's boundary.
fn head(line: &str, bytes: usize) -> &str {
    if line.len() <= bytes {
        return line;
    }
    let mut end = bytes;
    while !line.is_char_boundary(end) {
        end -= 1;
    }
    &line[..end]
}

/// A sentence whose checksum checks out: its address and its fields, the first field after the
/// address first.
#[derive(Debug, PartialEq)]
struct Sentence<'a> {
    address: &'a str,
    fields: Vec<&'a str>,
}

/// Why a `$` doesn't open a sentence.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Failure {
    /// No `*` and two hexadecimal digits before the line or the next `$` ends it.
    CutOff,
    /// A checksum that fails, a character the standard doesn't allow, or an address that isn't
    /// one.
    Damaged,
}

/// What one line holds.
#[derive(Debug, Default)]
struct LineScan<'a> {
    sentences: Vec<Sentence<'a>>,
    /// Bytes on the line outside any sentence read, but for spaces.
    noise: bool,
    /// Whether the line holds a `$`.
    dollar: bool,
    /// The first `$` that doesn't open a sentence, and why.
    failure: Option<Failure>,
}

/// The sentences on a line: each `$` that opens one whose checksum checks out. Each character is
/// looked at a fixed number of times.
fn scan_line(line: &str) -> LineScan<'_> {
    let mut scan = LineScan::default();
    let mut pieces = line.split('$');
    if pieces
        .next()
        .is_some_and(|before| !before.trim().is_empty())
    {
        scan.noise = true;
    }
    for piece in pieces {
        scan.dollar = true;
        match sentence(piece) {
            Ok((sentence, rest)) => {
                if !rest.trim().is_empty() {
                    scan.noise = true;
                }
                scan.sentences.push(sentence);
            }
            Err(failure) => {
                scan.noise = true;
                scan.failure.get_or_insert(failure);
            }
        }
    }
    scan
}

/// The sentence a piece of a line after a `$` opens, and what follows its checksum.
fn sentence(piece: &str) -> Result<(Sentence<'_>, &str), Failure> {
    let star = piece.find('*').ok_or(Failure::CutOff)?;
    let (body, after) = (&piece[..star], &piece[star + 1..]);
    let digits = after
        .get(..2)
        .filter(|digits| digits.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or(Failure::CutOff)?;
    let stated = u8::from_str_radix(digits, 16).map_err(|_| Failure::CutOff)?;
    let allowed = |b: u8| (0x20..=0x7e).contains(&b) && !matches!(b, b'!' | b'\\' | b'~');
    if !body.bytes().all(allowed) || checksum(body) != stated {
        return Err(Failure::Damaged);
    }
    let mut fields = body.split(',');
    // `split` yields at least one piece, so the address is always there.
    let address = fields.next().unwrap_or_default();
    let address_ok = (2..=16).contains(&address.len())
        && address
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
    if !address_ok {
        return Err(Failure::Damaged);
    }
    Ok((
        Sentence {
            address,
            fields: fields.collect(),
        },
        &after[2..],
    ))
}

/// A sentence's checksum: the exclusive-or of its characters between the `$` and the `*` (*NMEA
/// Revealed*, "NMEA Encoding Conventions").
fn checksum(body: &str) -> u8 {
    body.bytes().fold(0, |sum, b| sum ^ b)
}

/// The four sentence types read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Gga,
    Rmc,
    Gsa,
    Gsv,
}

impl Kind {
    fn of(address: &str) -> Option<Self> {
        if address.len() != 5 {
            return None;
        }
        match &address[2..] {
            "GGA" => Some(Self::Gga),
            "RMC" => Some(Self::Rmc),
            "GSA" => Some(Self::Gsa),
            "GSV" => Some(Self::Gsv),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Gga => "GGA",
            Self::Rmc => "RMC",
            Self::Gsa => "GSA",
            Self::Gsv => "GSV",
        }
    }

    /// The fields its layout has, and the most this reader reads; fields past the most are left
    /// out, counted.
    fn fields(self) -> (usize, usize) {
        match self {
            // NMEA 2.3 added RMC's mode; its layout without it is 11 fields.
            Self::Gga => (14, 14),
            Self::Rmc => (11, 12),
            Self::Gsa => (17, 17),
            // Three, then four for each satellite.
            Self::Gsv => (3, usize::MAX),
        }
    }
}

/// How a field becomes a channel's value.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rule {
    /// A number as written.
    Number,
    /// A number times a factor.
    Times(f64),
    /// A latitude or longitude, `ddmm.mmmm` or `dddmm.mmmm`, with its hemisphere field next.
    Coordinate {
        most_degrees: f64,
        negative: char,
        positive: char,
    },
    /// A number with an `E`/`W` field next, east positive.
    EastWest,
    /// A letter, as a code.
    Letter(&'static [(char, f64)]),
}

/// One channel, as read so far.
struct Track {
    name: String,
    quantity: Quantity,
    time_s: Vec<f64>,
    values: Vec<f64>,
    /// Left out of the record if every value is a gap.
    optional: bool,
    /// Fields that weren't what the channel holds, and the first as written.
    bad: usize,
    bad_example: Option<String>,
}

impl Track {
    fn new(name: String, quantity: Quantity, optional: bool) -> Self {
        Self {
            name,
            quantity,
            time_s: Vec::new(),
            values: Vec::new(),
            optional,
            bad: 0,
            bad_example: None,
        }
    }

    fn push(&mut self, time_s: f64, value: f64) {
        self.time_s.push(time_s);
        self.values.push(value);
    }

    /// A field's value, a gap if empty; a field that isn't what the channel holds is a gap,
    /// counted.
    fn push_field(&mut self, time_s: f64, value: Option<Result<f64, &str>>) {
        let value = match value {
            None => f64::NAN,
            Some(Ok(value)) => value,
            Some(Err(written)) => {
                self.bad += 1;
                if self.bad_example.is_none() {
                    self.bad_example = Some(short(written));
                }
                f64::NAN
            }
        };
        self.push(time_s, value);
    }
}

/// `GGA`'s and `RMC`'s fixed channels: each one's name after the address, quantity, field place,
/// rule, and whether it is left out when always empty.
type Layout = &'static [(&'static str, Quantity, usize, Rule, bool)];

const LATITUDE: Rule = Rule::Coordinate {
    most_degrees: 90.0,
    negative: 'S',
    positive: 'N',
};
const LONGITUDE: Rule = Rule::Coordinate {
    most_degrees: 180.0,
    negative: 'W',
    positive: 'E',
};

/// The FAA mode letters, as `GGA`'s fix quality codes for the same mode.
const MODES: &[(char, f64)] = &[
    ('N', 0.0),
    ('A', 1.0),
    ('D', 2.0),
    ('P', 3.0),
    ('R', 4.0),
    ('F', 5.0),
    ('E', 6.0),
    ('M', 7.0),
    ('S', 8.0),
];

const GGA: Layout = &[
    ("latitude", Quantity::Latitude, 1, LATITUDE, false),
    ("longitude", Quantity::Longitude, 3, LONGITUDE, false),
    ("fix quality", Quantity::Number, 5, Rule::Number, false),
    ("satellites used", Quantity::Number, 6, Rule::Number, false),
    ("HDOP", Quantity::Number, 7, Rule::Number, false),
    // The altitude and the geoid separation (places 8 and 10) are read apart, by their units.
    ("DGPS age", Quantity::Number, 12, Rule::Number, true),
    ("DGPS station", Quantity::Number, 13, Rule::Number, true),
];

const RMC: Layout = &[
    (
        "status",
        Quantity::Number,
        1,
        Rule::Letter(&[('A', 1.0), ('V', 0.0)]),
        false,
    ),
    ("latitude", Quantity::Latitude, 2, LATITUDE, false),
    ("longitude", Quantity::Longitude, 4, LONGITUDE, false),
    ("speed", Quantity::Speed, 6, Rule::Times(KNOT_M_S), false),
    ("course", Quantity::Angle, 7, Rule::Number, false),
    (
        "magnetic variation",
        Quantity::Angle,
        9,
        Rule::EastWest,
        true,
    ),
    ("mode", Quantity::Number, 11, Rule::Letter(MODES), true),
];

const GSA: Layout = &[
    (
        "selection",
        Quantity::Number,
        0,
        Rule::Letter(&[('A', 1.0), ('M', 0.0)]),
        false,
    ),
    ("fix type", Quantity::Number, 1, Rule::Number, false),
    // `satellites used` is counted from places 2 to 13.
    ("PDOP", Quantity::Number, 14, Rule::Number, false),
    ("HDOP", Quantity::Number, 15, Rule::Number, false),
    ("VDOP", Quantity::Number, 16, Rule::Number, false),
];

/// A field by its rule: `None` if empty, `Err` with what is written if it isn't the channel's.
fn field_value<'a>(fields: &[&'a str], place: usize, rule: Rule) -> Option<Result<f64, &'a str>> {
    let cell = fields.get(place).copied().unwrap_or_default().trim();
    if cell.is_empty() {
        return None;
    }
    let next = || fields.get(place + 1).copied().unwrap_or_default().trim();
    let value = match rule {
        Rule::Number => number(cell, false),
        Rule::Times(factor) => number(cell, false).map(|value| value * factor),
        Rule::Coordinate {
            most_degrees,
            negative,
            positive,
        } => coordinate(cell, most_degrees)
            .and_then(|degrees| sign(next(), negative, positive, degrees)),
        Rule::EastWest => number(cell, false).and_then(|degrees| sign(next(), 'W', 'E', degrees)),
        Rule::Letter(codes) => {
            let mut chars = cell.chars();
            match (chars.next(), chars.next()) {
                (Some(letter), None) => codes
                    .iter()
                    .find(|(code, _)| *code == letter)
                    .map(|(_, value)| *value),
                _ => None,
            }
        }
    };
    Some(value.ok_or(cell))
}

/// A value made negative by its hemisphere's letter; `None` for another letter or none.
fn sign(hemisphere: &str, negative: char, positive: char, value: f64) -> Option<f64> {
    let mut chars = hemisphere.chars();
    match (chars.next(), chars.next()) {
        (Some(letter), None) if letter == positive => Some(value),
        (Some(letter), None) if letter == negative => Some(-value),
        _ => None,
    }
}

/// A latitude or longitude, `ddmm.mmmm` or `dddmm.mmmm`, as degrees: the two digits before the
/// point and those after it are minutes, the digits before them degrees (*NMEA Revealed*, "NMEA
/// Encoding Conventions"). `None` for anything else, minutes of 60 or more, or more degrees than
/// `most_degrees`.
fn coordinate(cell: &str, most_degrees: f64) -> Option<f64> {
    let (whole, fraction) = split_decimal(cell)?;
    let digits = |text: &str| text.bytes().all(|b| b.is_ascii_digit());
    if whole.len() < 2 || whole.len() > 5 || !digits(whole) || !digits(fraction) {
        return None;
    }
    let (degrees, minutes) = whole.split_at(whole.len() - 2);
    let degrees: f64 = if degrees.is_empty() {
        0.0
    } else {
        degrees.parse().ok()?
    };
    let minutes: f64 = if fraction.is_empty() {
        minutes.parse().ok()?
    } else {
        format!("{minutes}.{fraction}").parse().ok()?
    };
    let value = degrees + minutes / 60.0;
    (minutes < 60.0 && value <= most_degrees).then_some(value)
}

/// A cell's digits before and after its point; `None` for a point with no digit after it.
fn split_decimal(cell: &str) -> Option<(&str, &str)> {
    match cell.split_once('.') {
        Some((_, "")) => None,
        Some(parts) => Some(parts),
        None => Some((cell, "")),
    }
}

/// A time of day, `hhmmss` with up to nine decimals, as nanoseconds after midnight; `None` for
/// anything else, or a time past 23:59:60.999 999 999.
fn time_of_day(cell: &str) -> Option<i64> {
    let (whole, fraction) = split_decimal(cell)?;
    if whole.len() != 6
        || fraction.len() > 9
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let part = |range: std::ops::Range<usize>| whole[range].parse::<i64>().ok();
    let (hours, minutes, seconds) = (part(0..2)?, part(2..4)?, part(4..6)?);
    if hours > 23 || minutes > 59 || seconds > 60 {
        return None;
    }
    let mut nanos: i64 = 0;
    for (place, digit) in fraction.bytes().enumerate() {
        nanos += i64::from(digit - b'0') * 10_i64.pow(8 - place as u32);
    }
    Some(((hours * 60 + minutes) * 60 + seconds) * 1_000_000_000 + nanos)
}

/// A date, `ddmmyy`, as days since 1970-01-01; a year `yy` of 80 or more is 19`yy`, else
/// 20`yy`. `None` for anything else, or a day the month doesn't have.
fn date(cell: &str) -> Option<i64> {
    if cell.len() != 6 || !cell.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let part = |range: std::ops::Range<usize>| cell[range].parse::<i64>().ok();
    let (day, month, year) = (part(0..2)?, part(2..4)?, part(4..6)?);
    let year = if year >= 80 { 1900 + year } else { 2000 + year };
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let length = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    (1..=length)
        .contains(&day)
        .then(|| days_from_civil(year, month, day))
}

/// Days since 1970-01-01 of a date in the proleptic Gregorian calendar (H. Hinnant,
/// "chrono-Compatible Low-Level Date Algorithms", `days_from_civil`, public domain).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The date of a count of days since 1970-01-01 (Hinnant, `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// The log's clock, as the timed sentences set it.
#[derive(Default)]
struct Clock {
    /// The first timed sentence's time of day, ns: the log's zero, on day 0.
    zero_ns: Option<i64>,
    /// The day of the last timed sentence, counted from the zero's.
    day: i64,
    last_tod_ns: Option<i64>,
    last_abs_ns: Option<i128>,
    /// The last valid date read, days since 1970, and its day count.
    last_date: Option<(i64, i64)>,
    /// The first valid date read, and its day count.
    first_date: Option<(i64, i64)>,
    /// The time the untimed sentences take: the last timed sentence's, `NaN` before one.
    epoch_s: f64,
    /// How often, and how far at most, the time goes back, s.
    backs: usize,
    largest_back_s: f64,
    /// Days the time of day passed midnight, forward or back, and days a date settled.
    midnights: usize,
    settled: usize,
}

impl Clock {
    fn new() -> Self {
        Self {
            epoch_s: f64::NAN,
            ..Self::default()
        }
    }

    /// The time, s from the log's zero, of a sentence stating a time of day and perhaps a valid
    /// date; `NaN` without a time of day.
    fn time(&mut self, tod_ns: Option<i64>, date: Option<i64>) -> f64 {
        let Some(tod_ns) = tod_ns else {
            self.epoch_s = f64::NAN;
            return f64::NAN;
        };
        if let Some(last) = self.last_tod_ns {
            let step = i128::from(tod_ns) - i128::from(last);
            if step < -DAY_NS / 2 {
                self.day += 1;
                self.midnights += 1;
            } else if step > DAY_NS / 2 {
                self.day -= 1;
                self.midnights += 1;
            }
        }
        if let Some(date) = date {
            if let Some((last_date, last_day)) = self.last_date {
                let day = last_day.saturating_add(date - last_date);
                if day != self.day {
                    self.settled += 1;
                    self.day = day;
                }
            }
            self.last_date = Some((date, self.day));
            self.first_date.get_or_insert((date, self.day));
        }
        let zero = *self.zero_ns.get_or_insert(tod_ns);
        let abs = i128::from(self.day) * DAY_NS + i128::from(tod_ns);
        if let Some(last) = self.last_abs_ns
            && abs < last
        {
            self.backs += 1;
            self.largest_back_s = self.largest_back_s.max((last - abs) as f64 / 1e9);
        }
        self.last_abs_ns = Some(abs);
        self.last_tod_ns = Some(tod_ns);
        self.epoch_s = (abs - i128::from(zero)) as f64 / 1e9;
        self.epoch_s
    }
}

/// The channels of one sentence address.
struct Address {
    address: String,
    kind: Kind,
    /// Indexes into the reader's tracks of the address's fixed channels, in [`Layout`] order.
    fixed: Vec<usize>,
    /// For `GGA`: the altitude's and geoid separation's tracks, by unit as written.
    by_unit: Vec<(usize, String, usize)>,
    /// For `GSV`: each satellite's three tracks, by PRN, and those left out past the cap.
    satellites: Vec<(u16, [usize; 3])>,
    satellites_left_out: BTreeSet<u16>,
    groups_bad: usize,
    sentences: usize,
    short: usize,
    extra: usize,
}

/// The reader's state.
struct Reader {
    tracks: Vec<Track>,
    addresses: Vec<Address>,
    clock: Clock,
    /// Sentences at addresses past [`MAX_ADDRESSES`].
    addresses_left_out: usize,
    /// Other sentence types, by address, at most [`MAX_OTHER_TYPES`] apart, and the rest.
    other: BTreeMap<String, usize>,
    other_left_out: usize,
    /// Timed sentences whose time is empty, or isn't a time.
    empty_times: usize,
    bad_times: usize,
    bad_time_example: Option<String>,
    /// Untimed sentences read before any time.
    untimed_before: usize,
    /// `RMC` dates: valid ones on void sentences that no valid sentence states, and bad ones.
    void_dates: BTreeSet<i64>,
    valid_dates: BTreeSet<i64>,
    void_dated: usize,
    rmc: usize,
    bad_dates: usize,
    /// `GGA` then `RMC` pairs, and those whose times agree.
    last_gga_tod: Option<i64>,
    pairs: usize,
    pairs_agree: usize,
}

/// Reads a GPS receiver's NMEA 0183 sentences.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if no line holds a sentence whose checksum checks out;
/// [`LogError::NoData`] if none of those is a `GGA`, `RMC`, `GSA` or `GSV`.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut reader = Reader {
        tracks: Vec::new(),
        addresses: Vec::new(),
        clock: Clock::new(),
        addresses_left_out: 0,
        other: BTreeMap::new(),
        other_left_out: 0,
        empty_times: 0,
        bad_times: 0,
        bad_time_example: None,
        untimed_before: 0,
        void_dates: BTreeSet::new(),
        valid_dates: BTreeSet::new(),
        void_dated: 0,
        rmc: 0,
        bad_dates: 0,
        last_gga_tod: None,
        pairs: 0,
        pairs_agree: 0,
    };
    let mut lines = LineCounts::default();
    let mut sentences = 0usize;
    for (number, line) in numbered_lines(text) {
        let not_text = line
            .chars()
            .filter(|c| *c == '\0' || *c == char::REPLACEMENT_CHARACTER)
            .count();
        if not_text > 0 {
            lines.not_text += not_text;
            lines.not_text_lines += 1;
            lines.last_not_text = number;
        }
        if line.trim().is_empty() {
            continue;
        }
        let scan = scan_line(line);
        if scan.sentences.is_empty() {
            let kind = match (scan.dollar, scan.failure) {
                (false, _) => &mut lines.no_start,
                (true, Some(Failure::Damaged)) => &mut lines.damaged,
                (true, _) => &mut lines.cut_off,
            };
            kind.count(number, line);
            continue;
        }
        if scan.noise {
            lines.noise.count(number, line);
        }
        for sentence in scan.sentences {
            sentences += 1;
            reader.sentence(&sentence);
        }
    }
    if sentences == 0 {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "no line holds a sentence, a `$`, an address, fields and a checksum that \
                      checks out"
                .to_owned(),
        });
    }
    if reader.addresses.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    Ok(reader.build(&lines))
}

/// Lines of one kind that hold no sentence, or noise beside one: how many, and the first.
#[derive(Default)]
struct LineKind {
    count: usize,
    first: Option<(usize, String)>,
}

impl LineKind {
    fn count(&mut self, number: usize, line: &str) {
        self.count += 1;
        self.first.get_or_insert_with(|| (number, short(line)));
    }
}

#[derive(Default)]
struct LineCounts {
    no_start: LineKind,
    cut_off: LineKind,
    damaged: LineKind,
    noise: LineKind,
    not_text: usize,
    not_text_lines: usize,
    last_not_text: usize,
}

impl Reader {
    /// Reads one sentence into its address's channels.
    fn sentence(&mut self, sentence: &Sentence<'_>) {
        let Some(kind) = Kind::of(sentence.address) else {
            if let Some(count) = self.other.get_mut(sentence.address) {
                *count += 1;
            } else if self.other.len() < MAX_OTHER_TYPES {
                self.other.insert(sentence.address.to_owned(), 1);
            } else {
                self.other_left_out += 1;
            }
            return;
        };
        let index = match self
            .addresses
            .iter()
            .position(|a| a.address == sentence.address)
        {
            Some(index) => index,
            None if self.addresses.len() < MAX_ADDRESSES => {
                self.add_address(sentence.address, kind)
            }
            None => {
                self.addresses_left_out += 1;
                return;
            }
        };
        let fields = &sentence.fields;
        let time_s = self.time(kind, fields);
        let address = &mut self.addresses[index];
        address.sentences += 1;
        let (layout_fields, most) = kind.fields();
        if fields.len() < layout_fields {
            address.short += 1;
        }
        if kind != Kind::Gsv && fields.len() > most {
            address.extra += 1;
        }
        let layout = match kind {
            Kind::Gga => GGA,
            Kind::Rmc => RMC,
            Kind::Gsa => GSA,
            Kind::Gsv => &[],
        };
        for (slot, (_, _, place, rule, _)) in layout.iter().enumerate() {
            let track = address.fixed[slot];
            self.tracks[track].push_field(time_s, field_value(fields, *place, *rule));
        }
        match kind {
            Kind::Gga => {
                self.by_unit(index, time_s, fields, 8, "altitude", Quantity::GnssAltitude);
                self.by_unit(
                    index,
                    time_s,
                    fields,
                    10,
                    "geoid separation",
                    Quantity::Distance,
                );
            }
            Kind::Gsa => {
                let used = (2..=13)
                    .filter(|place| fields.get(*place).is_some_and(|f| !f.trim().is_empty()))
                    .count();
                let track = self.addresses[index].fixed[GSA.len()];
                self.tracks[track].push(time_s, used as f64);
            }
            Kind::Gsv => self.gsv(index, time_s, fields),
            Kind::Rmc => {}
        }
    }

    /// The time of a sentence: its own for `GGA` and `RMC`, else its second's.
    fn time(&mut self, kind: Kind, fields: &[&str]) -> f64 {
        if !matches!(kind, Kind::Gga | Kind::Rmc) {
            if self.clock.zero_ns.is_none() {
                self.untimed_before += 1;
            }
            return self.clock.epoch_s;
        }
        let cell = fields.first().copied().unwrap_or_default().trim();
        let tod = if cell.is_empty() {
            self.empty_times += 1;
            None
        } else {
            let tod = time_of_day(cell);
            if tod.is_none() {
                self.bad_times += 1;
                self.bad_time_example.get_or_insert_with(|| short(cell));
            }
            tod
        };
        let mut valid_date = None;
        match kind {
            Kind::Gga => self.last_gga_tod = tod,
            Kind::Rmc => {
                self.rmc += 1;
                if let Some(gga) = self.last_gga_tod.take() {
                    self.pairs += 1;
                    if tod == Some(gga) {
                        self.pairs_agree += 1;
                    }
                }
                let status = fields.get(1).copied().unwrap_or_default().trim();
                let cell = fields.get(8).copied().unwrap_or_default().trim();
                if !cell.is_empty() {
                    match date(cell) {
                        Some(day) if status == "A" => {
                            valid_date = Some(day);
                            self.valid_dates.insert(day);
                        }
                        Some(day) => {
                            self.void_dated += 1;
                            // Distinct days are few; a hostile file's are bounded by the 36,525
                            // a two-digit year spans.
                            self.void_dates.insert(day);
                        }
                        None => self.bad_dates += 1,
                    }
                }
            }
            Kind::Gsa | Kind::Gsv => {}
        }
        self.clock.time(tod, valid_date)
    }

    /// A new address's fixed channels.
    fn add_address(&mut self, address: &str, kind: Kind) -> usize {
        let layout = match kind {
            Kind::Gga => GGA,
            Kind::Rmc => RMC,
            Kind::Gsa => GSA,
            Kind::Gsv => &[],
        };
        let mut fixed = Vec::new();
        for (name, quantity, _, _, optional) in layout {
            fixed.push(self.tracks.len());
            self.tracks.push(Track::new(
                format!("{address} {name}"),
                *quantity,
                *optional,
            ));
        }
        let extra = match kind {
            Kind::Gsa => Some("satellites used"),
            Kind::Gsv => Some("satellites in view"),
            Kind::Gga | Kind::Rmc => None,
        };
        if let Some(name) = extra {
            fixed.push(self.tracks.len());
            self.tracks.push(Track::new(
                format!("{address} {name}"),
                Quantity::Number,
                false,
            ));
        }
        self.addresses.push(Address {
            address: address.to_owned(),
            kind,
            fixed,
            by_unit: Vec::new(),
            satellites: Vec::new(),
            satellites_left_out: BTreeSet::new(),
            groups_bad: 0,
            sentences: 0,
            short: 0,
            extra: 0,
        });
        self.addresses.len() - 1
    }

    /// A `GGA` value with a unit field after it: to the meters channel if the unit is `M` or the
    /// value empty, else to a channel for that unit, as written. Two units a sentence at most, a
    /// few kinds of unit an address: past four, a value in another unit is a gap, counted.
    fn by_unit(
        &mut self,
        index: usize,
        time_s: f64,
        fields: &[&str],
        place: usize,
        name: &str,
        quantity: Quantity,
    ) {
        let cell = fields.get(place).copied().unwrap_or_default().trim();
        let unit = fields.get(place + 1).copied().unwrap_or_default().trim();
        let unit = if cell.is_empty() || unit == "M" {
            "M"
        } else {
            unit
        };
        let found = self.addresses[index]
            .by_unit
            .iter()
            .find(|(field, written, _)| *field == place && written == unit)
            .map(|(_, _, track)| *track);
        let track = match found {
            Some(track) => track,
            None => {
                let kinds = self.addresses[index]
                    .by_unit
                    .iter()
                    .filter(|(field, _, _)| *field == place)
                    .count();
                let address = &self.addresses[index].address;
                let (track_name, quantity) = if unit == "M" {
                    (format!("{address} {name}"), quantity)
                } else if kinds < 4 {
                    (
                        format!("{address} {name}, unit {:?}", short(unit)),
                        Quantity::Unrecognized,
                    )
                } else {
                    // Past the cap, into the meters channel as a gap, counted as not its own.
                    let meters = self.addresses[index]
                        .by_unit
                        .iter()
                        .find(|(field, written, _)| *field == place && written == "M")
                        .map(|(_, _, track)| *track);
                    if let Some(track) = meters {
                        self.tracks[track].push_field(time_s, Some(Err(unit)));
                    }
                    return;
                };
                let track = self.tracks.len();
                self.tracks.push(Track::new(track_name, quantity, false));
                self.addresses[index]
                    .by_unit
                    .push((place, unit.to_owned(), track));
                track
            }
        };
        self.tracks[track].push_field(time_s, field_value(fields, place, Rule::Number));
    }

    /// A `GSV` sentence: the count in view on a burst's first, and each satellite's three
    /// channels.
    fn gsv(&mut self, index: usize, time_s: f64, fields: &[&str]) {
        if fields.get(1).map(|f| f.trim()) == Some("1") {
            let track = self.addresses[index].fixed[0];
            self.tracks[track].push_field(time_s, field_value(fields, 2, Rule::Number));
        }
        let groups = fields.get(3..).unwrap_or_default();
        if groups.len() % 4 != 0 {
            // NMEA 4.10's signal id, or what isn't a whole satellite.
            self.addresses[index].extra += 1;
        }
        for group in groups.as_chunks::<4>().0 {
            let prn = group[0].trim();
            if prn.is_empty() {
                continue;
            }
            let Some(prn) = (prn.len() <= 3 && prn.bytes().all(|b| b.is_ascii_digit()))
                .then(|| prn.parse::<u16>().ok())
                .flatten()
            else {
                self.addresses[index].groups_bad += 1;
                continue;
            };
            let found = self.addresses[index]
                .satellites
                .iter()
                .find(|(id, _)| *id == prn)
                .map(|(_, tracks)| *tracks);
            let tracks = match found {
                Some(tracks) => tracks,
                None if self.addresses[index].satellites.len() < MAX_SATELLITES => {
                    let address = self.addresses[index].address.clone();
                    let mut tracks = [0; 3];
                    for (slot, (name, quantity)) in [
                        ("elevation", Quantity::Angle),
                        ("azimuth", Quantity::Angle),
                        ("SNR", Quantity::Number),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        tracks[slot] = self.tracks.len();
                        self.tracks.push(Track::new(
                            format!("{address} PRN {prn} {name}"),
                            quantity,
                            false,
                        ));
                    }
                    self.addresses[index].satellites.push((prn, tracks));
                    tracks
                }
                None => {
                    self.addresses[index].satellites_left_out.insert(prn);
                    continue;
                }
            };
            for (slot, track) in tracks.into_iter().enumerate() {
                self.tracks[track]
                    .push_field(time_s, field_value(&group[..], slot + 1, Rule::Number));
            }
        }
    }

    /// The record: every channel kept, the empty primary track, and the notes.
    fn build(self, lines: &LineCounts) -> FlightLog {
        let mut notes = vec![
            "the file holds a GNSS receiver's sentences and no barometer, so the record's \
             altitude track is empty. Every value is a channel, named for its sentence's address \
             and field, such as `GPGGA altitude`"
                .to_owned(),
        ];
        notes.extend(self.clock_notes());
        notes.extend(self.file_notes(lines));
        let mut single = Notes::default();
        for track in &self.tracks {
            if track.bad > 0 {
                single.push(format!(
                    "`{}` holds {} fields that aren't what it holds, such as {:?}: they are gaps",
                    short(&track.name),
                    track.bad,
                    track.bad_example.as_deref().unwrap_or_default()
                ));
            }
            if track.quantity == Quantity::Unrecognized {
                let finite = track.values.iter().filter(|v| v.is_finite()).count();
                single.push(format!(
                    "`{}` kept as written, in a unit NMEA 0183 doesn't give: {finite} values, \
                     where the layout gives meters, `M`",
                    short(&track.name)
                ));
            }
        }
        for address in &self.addresses {
            let (layout, _) = address.kind.fields();
            if address.short > 0 {
                single.push(format!(
                    "{} of {} `{}` sentences have fewer fields than the layout's {layout}: the \
                     fields they leave out are gaps",
                    address.short, address.sentences, address.address
                ));
            }
            if address.extra > 0 {
                single.push(format!(
                    "{} `{}` sentences hold fields past those this reader reads, such as NMEA \
                     4.10's signal or system id: they are left out",
                    address.extra, address.address
                ));
            }
            if address.groups_bad > 0 {
                single.push(format!(
                    "{} satellites in `{}` sentences have a PRN that isn't one to three \
                     digits: they are left out",
                    address.groups_bad, address.address
                ));
            }
            if !address.satellites_left_out.is_empty() {
                single.push(format!(
                    "`{}` names {} satellites past the {MAX_SATELLITES} given channels: they are \
                     left out",
                    address.address,
                    address.satellites_left_out.len()
                ));
            }
        }
        notes.extend(single.into_vec());

        let mut talkers: Vec<&str> = self.addresses.iter().map(|a| &a.address[..2]).collect();
        talkers.sort_unstable();
        talkers.dedup();
        let logger = format!(
            "NMEA 0183 receiver (talker{} {})",
            if talkers.len() == 1 { "" } else { "s" },
            talkers.join(", ")
        );
        let channels = self
            .tracks
            .into_iter()
            .filter(|track| !(track.optional && track.values.iter().all(|v| v.is_nan())))
            .map(|track| Channel {
                column: track.name,
                quantity: track.quantity,
                time_s: track.time_s,
                values: track.values,
            })
            .collect();
        FlightLog {
            format: LogFormat::Nmea0183,
            logger,
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
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

    /// The notes on the clock: the date's and the untimed sentences' decisions, then what the
    /// time did.
    fn clock_notes(&self) -> Vec<String> {
        let clock = &self.clock;
        let mut notes = Vec::new();
        let zero = clock.zero_ns.map(|zero| {
            let (h, rest) = (zero / 3_600_000_000_000, zero % 3_600_000_000_000);
            let (m, rest) = (rest / 60_000_000_000, rest % 60_000_000_000);
            format!("{h:02}:{m:02}:{:06.3} UTC", rest as f64 / 1e9)
        });
        let void = self.void_dates.difference(&self.valid_dates).count();
        let void_text = if self.void_dated > 0 {
            format!(
                ". {} status-`V` `RMC` carry {} distinct dates, {void} of which no status-`A` one \
                 gives, as a receiver writes a placeholder before its fix",
                self.void_dated,
                self.void_dates.len()
            )
        } else {
            String::new()
        };
        let evidence = match (&zero, clock.first_date) {
            (None, _) => {
                "no `GGA` or `RMC` holds a time, so every sample's time is a gap".to_owned()
            }
            (Some(zero), Some((first, day))) => {
                let (y, m, d) = civil_from_days(first - day);
                let back = match day {
                    0 => String::new(),
                    1 => "a day before ".to_owned(),
                    _ if day < 0 => format!("{} days after ", day.unsigned_abs()),
                    _ => format!("{day} days before "),
                };
                format!(
                    "the log's zero, the first sentence with a time, is {zero} on \
                     {y:04}-{m:02}-{d:02}. That is {back}the date of the first `RMC` with status \
                     `A`{void_text}"
                )
            }
            (Some(zero), None) if self.rmc == 0 => format!(
                "the file holds no `RMC`, so its times are times of day alone, from the first \
                 sentence with a time, at {zero}"
            ),
            (Some(zero), None) => format!(
                "no `RMC` has status `A`, so the times are times of day alone, from the first \
                 sentence with a time, at {zero}{void_text}"
            ),
        };
        notes.push(format!(
            "`RMC date` read in UTC days, from status-`A` sentences only: {evidence}"
        ));
        let untimed: Vec<&str> = [Kind::Gsa, Kind::Gsv]
            .into_iter()
            .filter(|kind| self.addresses.iter().any(|a| a.kind == *kind))
            .map(Kind::name)
            .collect();
        if !untimed.is_empty() {
            let order = if self.pairs > 0 {
                format!(
                    "in {} of the file's {} `GGA` and `RMC` pairs, the `RMC` has the `GGA`'s time",
                    self.pairs_agree, self.pairs
                )
            } else {
                "the file has no `GGA` followed by an `RMC` to show its order".to_owned()
            };
            let before = if self.untimed_before > 0 {
                format!(
                    "; {} come before any sentence with a time, and their times are gaps",
                    self.untimed_before
                )
            } else {
                String::new()
            };
            notes.push(format!(
                "`{}` read in the time of the `GGA` or `RMC` before them, from the sentence \
                 order: {order}{before}",
                untimed.join("` and `")
            ));
        }
        if self.empty_times > 0 {
            notes.push(format!(
                "{} `GGA` and `RMC` sentences have an empty time, as a receiver writes before it \
                 knows the time: their samples' times are gaps",
                self.empty_times
            ));
        }
        if self.bad_times > 0 {
            notes.push(format!(
                "{} `GGA` and `RMC` sentences have a time that isn't `hhmmss.sss`, such as {:?}: \
                 their samples' times are gaps",
                self.bad_times,
                self.bad_time_example.as_deref().unwrap_or_default()
            ));
        }
        if self.bad_dates > 0 {
            notes.push(format!(
                "{} `RMC` sentences have a date that isn't one, `ddmmyy`: left out",
                self.bad_dates
            ));
        }
        if clock.backs > 0 {
            notes.push(format!(
                "the time goes back {} times, by up to {:.3} s. The samples are kept in the \
                 file's order, each channel's times as its sentences give them. A receiver's UTC \
                 may step a few minutes after a cold start, when the satellites' leap-second count \
                 reaches it",
                clock.backs, clock.largest_back_s
            ));
        }
        if clock.midnights > 0 {
            notes.push(format!(
                "the time of day passes midnight {} times. Without a date, a time of day more \
                 than 12 hours before the last is the next day's. One more than 12 hours \
                 after it is the day before's",
                clock.midnights
            ));
        }
        if clock.settled > 0 {
            notes.push(format!(
                "{} times, an `RMC`'s date moves the day the time of day alone gave: the date \
                 settles it",
                clock.settled
            ));
        }
        notes
    }

    /// The notes on the file's lines, the sentences left out and a fix never had.
    fn file_notes(&self, lines: &LineCounts) -> Vec<String> {
        let mut notes = Vec::new();
        let kinds = [
            (
                &lines.no_start,
                "hold no sentence, no `$`: noise, or a sentence cut at its start",
            ),
            (
                &lines.cut_off,
                "hold a sentence cut off before its checksum",
            ),
            (
                &lines.damaged,
                "hold a sentence whose checksum fails, or with bytes NMEA 0183 doesn't allow",
            ),
        ];
        for (kind, what) in kinds {
            if let Some((number, text)) = &kind.first {
                notes.push(format!(
                    "{} lines {what}, the first line {number}, {text:?}: skipped",
                    kind.count
                ));
            }
        }
        if let Some((number, text)) = &lines.noise.first {
            notes.push(format!(
                "{} lines hold bytes beside their sentences, the first line {number}, {text:?}: \
                 the sentences are read and the bytes left out",
                lines.noise.count
            ));
        }
        if lines.not_text > 0 {
            notes.push(format!(
                "the file holds {} NUL bytes, or stand-ins (U+FFFD) for bytes that aren't UTF-8, on \
                 {} lines, the last line {}: left out",
                lines.not_text, lines.not_text_lines, lines.last_not_text
            ));
        }
        if !self.other.is_empty() {
            let named: Vec<String> = self
                .other
                .iter()
                .map(|(address, count)| format!("`{}` ({count})", short(address)))
                .collect();
            let more = if self.other_left_out > 0 {
                format!(" and {} sentences of other types", self.other_left_out)
            } else {
                String::new()
            };
            notes.push(format!(
                "sentences of types this reader doesn't read, {}{more}: their fields are left out",
                named.join(", ")
            ));
        }
        if self.addresses_left_out > 0 {
            notes.push(format!(
                "{} sentences at addresses past the {MAX_ADDRESSES} read: left out",
                self.addresses_left_out
            ));
        }
        let positions = self
            .tracks
            .iter()
            .filter(|t| t.quantity == Quantity::Latitude)
            .collect::<Vec<_>>();
        if !positions.is_empty()
            && positions
                .iter()
                .all(|t| t.values.iter().all(|v| v.is_nan()))
        {
            notes.push(
                "no `GGA` or `RMC` holds a position: the receiver never had a fix, so every \
                 latitude and longitude is a gap"
                    .to_owned(),
            );
        }
        notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sentence with its checksum.
    fn sentence_line(body: &str) -> String {
        format!("${body}*{:02X}", checksum(body))
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .unwrap_or_else(|| panic!("no {column}"))
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    /// The checksum is the exclusive-or between `$` and `*`, as *NMEA Revealed*'s examples
    /// state it; a wrong one, a cut sentence and a forbidden byte are told apart.
    #[test]
    fn checksums_check_out() {
        let gga = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
        assert_eq!(checksum(gga), 0x47);
        let line = format!("${gga}*47");
        let scan = scan_line(&line);
        assert_eq!(scan.sentences.len(), 1);
        assert_eq!(scan.sentences[0].address, "GPGGA");
        assert_eq!(scan.sentences[0].fields.len(), 14);
        assert!(!scan.noise);
        assert_eq!(scan_line(&format!("${gga}*47")).sentences.len(), 1);
        assert_eq!(
            scan_line(&format!("${gga}*48")).failure,
            Some(Failure::Damaged)
        );
        assert_eq!(scan_line(&format!("${gga}")).failure, Some(Failure::CutOff));
        assert_eq!(
            scan_line(&format!("${gga}*4")).failure,
            Some(Failure::CutOff)
        );
        assert_eq!(
            scan_line(&format!("${gga}*+7")).failure,
            Some(Failure::CutOff)
        );
        let damaged = sentence_line("GPGSA,A,\u{fffd}\u{fffd},,");
        assert_eq!(scan_line(&damaged).failure, Some(Failure::Damaged));
        let lower = sentence_line("gpgga,1");
        assert_eq!(scan_line(&lower).failure, Some(Failure::Damaged));
        // Lower-case hexadecimal digits are accepted.
        assert_eq!(checksum("GPXXX,1"), 0x52);
        assert_eq!(scan_line("$GPXXX,1*52").sentences.len(), 1);
        assert_eq!(scan_line("$GPZDA,1*7c").sentences.len(), 0);
        let lower = format!("$GPZDA,1*{:02x}", checksum("GPZDA,1"));
        assert_eq!(scan_line(&lower).sentences.len(), 1);
    }

    /// Noise before a `$`, after a checksum, and two sentences run together: each sentence read,
    /// the noise marked.
    #[test]
    fn sentences_are_read_wherever_a_dollar_opens_one() {
        let a = sentence_line("GPGSA,A,1,,,,,,,,,,,,,99.99,99.99,99.99");
        let b = sentence_line("GPRMC,071812.50,V,,,,,,,,,,N");
        let glued = format!("\u{fffd}q$Z{a}{b}\0\0");
        let scan = scan_line(&glued);
        assert_eq!(scan.sentences.len(), 2);
        assert!(scan.noise);
        assert_eq!(scan.failure, Some(Failure::CutOff));
        let spaced = format!("{a}  ");
        let scan = scan_line(&spaced);
        assert!(!scan.noise);
        let scan = scan_line(",,0,00,99.99,,,,,,*48");
        assert!(scan.sentences.is_empty() && !scan.dollar);
    }

    #[test]
    fn coordinates_are_degrees_and_minutes() {
        assert!(close(coordinate("4807.038", 90.0).unwrap(), 48.1173));
        assert!(close(
            coordinate("01131.000", 180.0).unwrap(),
            11.516_666_666_666_667
        ));
        assert!(close(coordinate("0000.6", 90.0).unwrap(), 0.01));
        assert_eq!(coordinate("9000.000", 90.0), Some(90.0));
        assert!(close(
            coordinate("17959.99", 180.0).unwrap(),
            179.0 + 59.99 / 60.0
        ));
        for bad in [
            "", "4", "4807.", "4860.000", "9100.0", "18100.0", "48a7.0", "4807.0.1", "-4807.0",
            "123456.0",
        ] {
            assert_eq!(coordinate(bad, 90.0), None, "{bad:?}");
        }
        assert_eq!(coordinate("18100.0", 180.0), None);
        let fields = ["4807.038", "S", "01131.000", "W"];
        assert!(close(
            field_value(&fields, 0, LATITUDE).unwrap().unwrap(),
            -48.1173
        ));
        assert!(field_value(&fields, 2, LONGITUDE).unwrap().unwrap() < 0.0);
        assert_eq!(
            field_value(&["4807.038", ""], 0, LATITUDE),
            Some(Err("4807.038"))
        );
        assert_eq!(
            field_value(&["4807.038", "E"], 0, LATITUDE),
            Some(Err("4807.038"))
        );
        assert_eq!(field_value(&["", "N"], 0, LATITUDE), None);
        assert_eq!(
            field_value(&["3.5", "W"], 0, Rule::EastWest),
            Some(Ok(-3.5))
        );
    }

    #[test]
    fn speeds_and_letters() {
        let speed = field_value(&["022.4"], 0, Rule::Times(KNOT_M_S))
            .unwrap()
            .unwrap();
        assert!(close(speed, 11.523_555_555_555_556), "{speed}");
        assert_eq!(field_value(&["E"], 0, Rule::Letter(MODES)), Some(Ok(6.0)));
        assert_eq!(field_value(&["N"], 0, Rule::Letter(MODES)), Some(Ok(0.0)));
        assert_eq!(field_value(&["X"], 0, Rule::Letter(MODES)), Some(Err("X")));
        assert_eq!(
            field_value(&["AA"], 0, Rule::Letter(MODES)),
            Some(Err("AA"))
        );
        assert_eq!(field_value(&[], 3, Rule::Number), None);
    }

    #[test]
    fn times_and_dates_are_read_strictly() {
        assert_eq!(time_of_day("000000"), Some(0));
        assert_eq!(time_of_day("123519"), Some(45_319_000_000_000));
        assert_eq!(time_of_day("123519.969"), Some(45_319_969_000_000));
        assert_eq!(time_of_day("235960.123456789"), Some(86_400_123_456_789));
        for bad in [
            "",
            "12351",
            "1235190",
            "240000",
            "126000",
            "123561",
            "123519.",
            "123519.1234567890",
            "12:35:19",
            "+23519",
            "123519.x",
        ] {
            assert_eq!(time_of_day(bad), None, "{bad:?}");
        }
        assert_eq!(date("010170"), Some(days_from_civil(2070, 1, 1)));
        assert_eq!(date("230394"), Some(days_from_civil(1994, 3, 23)));
        assert_eq!(date("010180"), Some(3652));
        assert_eq!(date("290224"), Some(days_from_civil(2024, 2, 29)));
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
        for bad in [
            "", "290223", "310424", "001024", "011324", "0110245", "01-024",
        ] {
            assert_eq!(date(bad), None, "{bad:?}");
        }
        for days in [-1, 0, 59, 11_016, 11_017, 19_000, 40_000] {
            let (y, m, d) = civil_from_days(days);
            assert_eq!(days_from_civil(y, m, d), days);
        }
        assert_eq!(civil_from_days(11_017), (2000, 3, 1));
    }

    /// Midnight without a date is the nearest day; a valid date settles the day.
    #[test]
    fn the_clock_crosses_midnight_and_takes_dates() {
        let mut clock = Clock::new();
        let s = 1_000_000_000_i64;
        assert_eq!(clock.time(Some(86_399 * s), None), 0.0);
        assert_eq!(clock.time(Some(s), None), 2.0);
        assert_eq!(clock.time(Some(0), None), 1.0);
        assert_eq!(clock.backs, 1);
        assert_eq!(clock.time(Some(86_399 * s), None), 0.0);
        assert_eq!(clock.midnights, 2);
        assert!(clock.time(None, None).is_nan());
        assert!(clock.epoch_s.is_nan());
        let mut clock = Clock::new();
        clock.time(Some(10 * s), Some(100));
        // Three days later by date, the same time of day.
        assert_eq!(clock.time(Some(10 * s), Some(103)), 3.0 * 86_400.0);
        assert_eq!(clock.settled, 1);
    }

    #[test]
    fn recognized_past_noise_and_not_otherwise() {
        let gga = sentence_line("GPGGA,,,,,,0,00,99.99,,,,,,");
        let gsa = sentence_line("GPGSA,A,1,,,,,,,,,,,,,99.99,99.99,99.99");
        assert!(recognizes(&format!(
            "R\u{fffd}j3q\r\n{gga}\r\n{gsa}\r\n{gga}\r\n"
        )));
        assert!(recognizes(&gga));
        assert!(!recognizes(&format!("noise\n{gga}\n")));
        assert!(!recognizes("Time,Altitude\n0,0\n"));
        assert!(!recognizes(""));
        let late = format!("{}{gga}\n{gga}\n{gga}\n", "x\n".repeat(60));
        assert!(!recognizes(&late));
        assert!(!recognizes(&format!("{}\n", "$GPGGA,1*00".repeat(10))));
        // Sentences quoted inside another log's lines don't make it NMEA.
        let quoted = format!("10:00:00.000: gps {{'sentence': '{gga}'}}\n");
        assert!(!recognizes(&quoted.repeat(5)));
    }

    /// A file of sentences of other types reads as no data; a file of none isn't NMEA.
    #[test]
    fn files_without_sentences_read_are_refused() {
        let vtg = sentence_line("GPVTG,054.7,T,034.4,M,005.5,N,010.2,K");
        assert_eq!(
            read(&format!("{vtg}\n{vtg}\n")).unwrap_err(),
            LogError::NoData { format: FORMAT }
        );
        let LogError::NotThisFormat { format, .. } = read("x\n$GPGGA,1*00\n").unwrap_err() else {
            panic!("refused for another reason");
        };
        assert_eq!(format, FORMAT);
        assert!(read("").is_err());
    }

    /// The fields of every sentence read, its channels named for address and field, its untimed
    /// sentences on their second's time.
    #[test]
    fn reads_the_four_sentences() {
        let lines = [
            sentence_line("GPGGA,123519.000,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,0000"),
            sentence_line("GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1"),
            sentence_line("GPGSV,2,1,05,04,45,120,40,05,30,200,35,09,10,300,,12,60,010,44"),
            sentence_line("GPGSV,2,2,05,24,05,080,20"),
            sentence_line("GPRMC,123519.000,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W,A"),
            sentence_line("GPVTG,084.4,T,,M,022.4,N,041.5,K,A"),
            sentence_line("GPGGA,123520.000,4807.040,N,01131.002,E,6,08,0.9,545.0,F,46.9,M,,0000"),
        ];
        let log = read(&lines.join("\r\n")).unwrap();
        assert_eq!(log.format, LogFormat::Nmea0183);
        assert_eq!(log.logger, "NMEA 0183 receiver (talker GP)");
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        let latitude = channel(&log, "GPGGA latitude");
        assert_eq!(latitude.quantity, Quantity::Latitude);
        assert!(close(latitude.values[0], 48.1173));
        assert_eq!(latitude.time_s, [0.0, 1.0]);
        let altitude = channel(&log, "GPGGA altitude");
        assert_eq!(altitude.quantity, Quantity::GnssAltitude);
        assert_eq!(altitude.values, [545.4]);
        let feet = channel(&log, "GPGGA altitude, unit \"F\"");
        assert_eq!(feet.quantity, Quantity::Unrecognized);
        assert_eq!(
            (feet.time_s.as_slice(), feet.values.as_slice()),
            (&[1.0][..], &[545.0][..])
        );
        assert_eq!(channel(&log, "GPGGA fix quality").values, [1.0, 6.0]);
        assert_eq!(channel(&log, "GPGGA DGPS station").values, [0.0, 0.0]);
        assert!(log.channels.iter().all(|c| c.column != "GPGGA DGPS age"));
        assert_eq!(channel(&log, "GPGSA satellites used").values, [5.0]);
        assert_eq!(channel(&log, "GPGSA VDOP").values, [2.1]);
        assert_eq!(channel(&log, "GPGSA selection").values, [1.0]);
        assert_eq!(channel(&log, "GPGSV satellites in view").values, [5.0]);
        let snr = channel(&log, "GPGSV PRN 9 SNR");
        assert!(snr.values[0].is_nan());
        assert_eq!(snr.time_s, [0.0]);
        assert_eq!(channel(&log, "GPGSV PRN 24 azimuth").values, [80.0]);
        let speed = channel(&log, "GPRMC speed");
        assert_eq!(speed.quantity, Quantity::Speed);
        assert!(close(speed.values[0], 11.523_555_555_555_556));
        assert_eq!(channel(&log, "GPRMC magnetic variation").values, [-3.1]);
        assert_eq!(channel(&log, "GPRMC mode").values, [1.0]);
        assert_eq!(channel(&log, "GPRMC status").values, [1.0]);
        let notes = log.notes.join("\n");
        for expected in [
            "`RMC date` read in UTC days, from status-`A` sentences only: the log's zero, the \
             first sentence with a time, is 12:35:19.000 UTC on 1994-03-23",
            "`GSA` and `GSV` read in the time of the `GGA` or `RMC` before them, from the \
             sentence order: in 1 of the file's 1 `GGA` and `RMC` pairs",
            "sentences of types this reader doesn't read, `GPVTG` (1)",
            "`GPGGA altitude, unit \"F\"` kept as written, in a unit NMEA 0183 doesn't give: 1 values",
        ] {
            assert!(notes.contains(expected), "{expected}\n{notes}");
        }
    }

    /// Bad fields are gaps counted in a note; short sentences' missing fields are gaps.
    #[test]
    fn bad_and_missing_fields_are_gaps() {
        let lines = [
            sentence_line("GPGGA,000001,48x7.0,N,01131.000,E,1"),
            sentence_line("GPRMC,000001,Q,,,,,,,,,,Z"),
        ];
        let log = read(&lines.join("\n")).unwrap();
        assert!(channel(&log, "GPGGA latitude").values[0].is_nan());
        assert!(channel(&log, "GPGGA HDOP").values[0].is_nan());
        let notes = log.notes.join("\n");
        for expected in [
            "`GPGGA latitude` holds 1 fields that aren't what it holds, such as \"48x7.0\"",
            "`GPRMC status` holds 1 fields",
            "`GPRMC mode` holds 1 fields",
            "1 of 1 `GPGGA` sentences have fewer fields than the layout's 14",
        ] {
            assert!(notes.contains(expected), "{expected}\n{notes}");
        }
    }

    /// Past the caps, addresses, satellites and other types are counted, not kept.
    #[test]
    fn caps_hold() {
        let mut text = String::new();
        for prn in 1..=70 {
            text.push_str(&sentence_line(&format!("GPGSV,1,1,70,{prn},1,2,3")));
            text.push('\n');
        }
        for talker in 0..20 {
            text.push_str(&sentence_line(&format!(
                "G{}GGA,000001",
                (b'A' + talker) as char
            )));
            text.push('\n');
        }
        for other in 0..12 {
            text.push_str(&sentence_line(&format!("GPX{:02},1", other)));
            text.push('\n');
        }
        let log = read(&text).unwrap();
        let addresses: BTreeSet<&str> = log.channels.iter().map(|c| &c.column[..5]).collect();
        assert_eq!(addresses.len(), MAX_ADDRESSES);
        assert_eq!(
            log.channels
                .iter()
                .filter(|c| c.column.ends_with(" SNR"))
                .count(),
            MAX_SATELLITES
        );
        let notes = log.notes.join("\n");
        for expected in [
            "5 sentences at addresses past the 16 read",
            "`GPGSV` names 6 satellites past the 64 given channels",
            "and 4 sentences of other types",
        ] {
            assert!(notes.contains(expected), "{expected}\n{notes}");
        }
        assert!(
            log.logger
                .starts_with("NMEA 0183 receiver (talkers GA, GB,")
        );
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a time for each value.
        #[test]
        fn any_text_is_read_or_refused(
            lines in proptest::collection::vec("[$GPAMCSVR*,.0-9A-F NWE\u{fffd}\0]{0,60}", 0..12),
            sign in proptest::bool::ANY,
        ) {
            // Half the lines get a checksum that checks out, so that sentences are read.
            let text: Vec<String> = lines
                .iter()
                .enumerate()
                .map(|(index, line)| {
                    if sign && index % 2 == 0 {
                        let body: String = line.chars().filter(|c| !matches!(c, '$' | '*')).collect();
                        sentence_line(&format!("GPGGA,{body}"))
                    } else {
                        line.clone()
                    }
                })
                .collect();
            let text = text.join("\r\n");
            let _ = recognizes(&text);
            if let Ok(log) = read(&text) {
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
                proptest::prop_assert!(log.time_s.is_empty());
            }
        }

        /// A valid fix's fields read back as the numbers written.
        #[test]
        fn fixes_read_back(
            seconds in 0u32..86_400,
            degrees in 0u32..90,
            minutes in 0u32..600_000,
            centiknots in 0u32..90_000,
        ) {
            let (h, m, s) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
            let body = format!(
                "GPRMC,{h:02}{m:02}{s:02}.000,A,{degrees:02}{:02}.{:04},S,00000.0,E,{}.{:02},0,\
                 010124,,,A",
                minutes / 10_000,
                minutes % 10_000,
                centiknots / 100,
                centiknots % 100
            );
            let log = read(&sentence_line(&body)).unwrap();
            let latitude = channel(&log, "GPRMC latitude").values[0];
            let expected = f64::from(degrees) + f64::from(minutes) / 10_000.0 / 60.0;
            proptest::prop_assert!((latitude + expected).abs() < 1e-12);
            let speed = channel(&log, "GPRMC speed").values[0];
            let knots = f64::from(centiknots) / 100.0;
            proptest::prop_assert!((speed - knots * KNOT_M_S).abs() < 1e-9);
            proptest::prop_assert_eq!(&channel(&log, "GPRMC status").time_s, &vec![0.0]);
        }
    }
}
