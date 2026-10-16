//! One team's radio telemetry packets, `255:…:254`: the flight computer's own log of them, the
//! packets a receiver heard, and the same packets in an RTF document a spreadsheet saved as CSV.
//!
//! A university team's flight computer writes each reading as a packet framed by `255` and `254`,
//! and sends the packets over its radio. The team published, for one flight, four files of them:
//! the flight computer's own log, the text a ground receiver wrote of what it heard, and two
//! exports of the log, cut to the flight, saved as RTF documents and then as CSV. No document
//! describes the packets, so this reader takes them from the files, and the sample logs' survey
//! (`cargo xtask logs`) holds the reading to the files: every packet the receiver or an export
//! holds that the onboard log also holds, by its stage, kind and clock, matches the log's field
//! for field ([ADR-274][adr-274]).
//!
//! **A packet** is fields separated by colons, or by commas in the exports:
//!
//! ```text
//! 255:T:11:PP:100150:950.12:21.50:3.25:254
//! 255:T:11:AA:100152:9.70:0.40:-0.80:254
//! 255:T:11:CC:100160:1.25:-0.50:14.00:2.50:254
//! 255:N:88:GG:100200:1.00:2.00:1234.56:9876.54:8.00:250.00:90.00:0.50:254
//! ```
//!
//! | Field | Read as |
//! |---|---|
//! | `255` | the packet's start |
//! | `T` | the path it came by, one capital letter: `T` from the flight computer, `N` relayed by a second transmitter, which also sends the GPS packets |
//! | `11` | two digits: the flight computer's stage of the flight, `00` to `55` in the samples, or `88` on a GPS packet |
//! | `PP` | its kind, two capital letters |
//! | `100150` | the flight computer's clock, ms, a whole number |
//! | the values | numbers, as many as the kind's first packet holds |
//! | `254` | the packet's end |
//!
//! Each kind is a set of channels on its own clock, named by the kind and the value's place from
//! 1, such as `PP 1`, and the kind's stage field is its own channel, `PP stage`. The clock is read
//! as milliseconds, the survey holding it to the flight: the time from liftoff (the first packet,
//! after the pad's stage `00`, whose pressure is more than 1 hPa below the median of the 5 s ending
//! 1 s before it) to the least pressure is 1.055 times √(2h/g), h the standard atmosphere's height
//! of the two pressures, where the check allows a factor of 2. `PP 1` is read as a pressure in
//! hectopascals, [`Quantity::Pressure`], the survey holding its pad reading between 500 and 1,100
//! hPa, where every launch site's is; the rest is kept as written, [`Quantity::Unrecognized`], as
//! nothing states its unit. Among them, `PP 3` climbs with the flight, but where it reads above 200 it is 1.11 to 1.24 times
//! the standard atmosphere's height of the pressure above the pad's in meters, by the survey, so no
//! unit is settled for it.
//!
//! **Lines that aren't packets.** A `Stage` line, `Stage` and a number's word from `One` to `Nine`,
//! such as `Stage Two`, is an event at the clock of the next packet; one heard again is a repeat. The onboard log opens with a whole number and `Gain[0]: …` lines, its filter's
//! gains, which a note counts. Any other line is broken, a packet the radio cut or ran into the next:
//! the receiver's text holds many. A broken line is dropped, and a note counts them; so is a
//! packet heard again, the same stage, kind, clock and values as one already read, by either path.
//! A packet whose kind has another count of values than its first packet is broken.
//!
//! **The exports** ([`LogFormat::RadioPacketRtf`]) are an RTF document whose lines a spreadsheet
//! split at the colons and saved as CSV, so each line has commas after it, the document's markup
//! sits in the first cells, and a `\` ends a line where the document broke it. The reader takes each
//! line on its own: the commas after it dropped, then the control words before a packet, such as
//! `\f0\fs24 \cf0 `, a `\` ending the line, and the `}` closing the document at its end. A line
//! of markup alone is dropped.
//!
//! **What refuses the file.** No packet at all ([`LogError::Syntax`]); more than [`MAX_KINDS`]
//! kinds, more than [`MAX_VALUES`] values in a packet, or a clock of 2³² ms, 49.7 days, or more.
//!
//! The record has no barometric track, as no packet states an altitude above the pad in a settled
//! unit: [`FlightLog::time_s`] and [`FlightLog::altitude_m`] are empty, and every value is a channel.
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use std::collections::HashSet;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{finite, numbered_lines, short};

/// The packets' text form's name in errors.
const TEXT: &str = "radio packets";
/// The exports' name in errors.
const RTF: &str = "radio packets in RTF";

/// How many of a file's first lines are searched for a packet.
const RECOGNITION_LINES: usize = 100;

/// The most kinds a file may hold, eight times the sample files' four.
pub const MAX_KINDS: usize = 32;

/// The most values a packet may hold, four times the sample files' eight.
pub const MAX_VALUES: usize = 32;

/// The largest clock, ms: a 32-bit count of milliseconds.
const CLOCK_LIMIT_MS: u64 = u32::MAX as u64;

/// Hectopascals to pascals.
const HECTOPASCAL_PA: f64 = 100.0;

/// A whole packet's fields.
#[derive(Debug, Clone, PartialEq)]
struct Packet<'a> {
    stage: &'a str,
    kind: &'a str,
    clock_ms: u64,
    /// The values as written, for telling a packet heard again.
    written: Vec<&'a str>,
    values: Vec<f64>,
}

/// Whether a field is a number as the packets write one: an optional `-`, digits, and optionally
/// a point and more digits.
fn is_number(field: &str) -> bool {
    let digits = field.strip_prefix('-').unwrap_or(field);
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, "0"));
    !whole.is_empty()
        && !fraction.is_empty()
        && whole.bytes().all(|b| b.is_ascii_digit())
        && fraction.bytes().all(|b| b.is_ascii_digit())
}

/// A line as a whole packet with fields separated by `separator`, or `None`.
fn packet(line: &str, separator: char) -> Option<Packet<'_>> {
    let fields: Vec<&str> = line.split(separator).collect();
    let [start, source, stage, kind, clock, values @ .., end] = fields.as_slice() else {
        return None;
    };
    let capitals = |field: &str, count: usize| {
        field.len() == count && field.bytes().all(|b| b.is_ascii_uppercase())
    };
    let whole_clock =
        !clock.is_empty() && clock.len() <= 10 && clock.bytes().all(|b| b.is_ascii_digit());
    if *start != "255"
        || *end != "254"
        || !capitals(source, 1)
        || stage.len() != 2
        || !stage.bytes().all(|b| b.is_ascii_digit())
        || !capitals(kind, 2)
        || !whole_clock
        || values.is_empty()
        || !values.iter().all(|value| is_number(value))
    {
        return None;
    }
    let clock_ms = clock.parse::<u64>().ok()?;
    // A number of hundreds of digits parses as infinite: no packet writes one, so the line is
    // broken.
    let parsed = values
        .iter()
        .map(|value| value.parse::<f64>().ok().filter(|v| v.is_finite()))
        .collect::<Option<Vec<f64>>>()?;
    Some(Packet {
        stage,
        kind,
        clock_ms,
        written: values.to_vec(),
        values: parsed,
    })
}

/// Whether a line is one of the onboard log's filter gains, `Gain[0]: …`.
fn is_gain(line: &str) -> bool {
    line.starts_with("Gain[")
}

/// The words a stage's mark may number it by, as the onboard log writes them.
const STAGE_WORDS: [&str; 9] = [
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

/// Whether a line is a stage's mark, `Stage` and a number's word, such as `Stage Two`; a word
/// the radio garbled, such as `Nwo`, isn't one.
fn stage_mark(line: &str) -> Option<&str> {
    let word = line.strip_prefix("Stage ")?;
    STAGE_WORDS.contains(&word).then_some(line)
}

/// Whether the text is the packets' text form: its first line that isn't blank is a packet, a whole
/// number or a filter gain, and a packet separated by colons is within its first
/// [`RECOGNITION_LINES`] lines.
pub(crate) fn recognizes_text(text: &str) -> bool {
    let mut lines = numbered_lines(text)
        .take(RECOGNITION_LINES)
        .map(|(_, line)| line.trim())
        .filter(|line| !line.is_empty())
        .peekable();
    let Some(first) = lines.peek().copied() else {
        return false;
    };
    let opens = packet(first, ':').is_some()
        || (!first.is_empty() && first.len() <= 10 && first.bytes().all(|b| b.is_ascii_digit()))
        || is_gain(first);
    opens && lines.any(|line| packet(line, ':').is_some())
}

/// Whether the text is the exports' form: an RTF document with a packet separated by commas, its
/// markup taken off as [`rtf_line`] does, within its first [`RECOGNITION_LINES`] lines.
pub(crate) fn recognizes_rtf(text: &str) -> bool {
    crate::eggtimer::recognizes_rtf(text)
        && numbered_lines(text)
            .take(RECOGNITION_LINES)
            .any(|(_, line)| matches!(rtf_line(line, false), RtfLine::Text(text) if packet(text, ',').is_some()))
}

/// An export's line, its markup taken off.
#[derive(Debug, PartialEq)]
enum RtfLine<'a> {
    /// The document's markup alone, or nothing.
    Markup,
    /// The line's text.
    Text(&'a str),
}

/// An export's line with the spreadsheet's commas after it, the control words before its text, a
/// `\` ending it and, on the document's `last` line, the closing `}` taken off.
fn rtf_line(line: &str, last: bool) -> RtfLine<'_> {
    let mut text = line.trim_end_matches(',');
    if last {
        text = text.strip_suffix('}').unwrap_or(text);
    }
    text = text.strip_suffix('\\').unwrap_or(text);
    // Control words: `\`, letters, an optional signed number and an optional space.
    while let Some(rest) = text.strip_prefix('\\') {
        let letters = rest.bytes().take_while(u8::is_ascii_alphabetic).count();
        if letters == 0 {
            break;
        }
        let rest = &rest[letters..];
        let signed = rest.strip_prefix('-').unwrap_or(rest);
        let digits = signed.bytes().take_while(u8::is_ascii_digit).count();
        let rest = &signed[digits..];
        text = rest.strip_prefix(' ').unwrap_or(rest);
    }
    if text.is_empty() || text.starts_with('{') || text.starts_with('\\') {
        RtfLine::Markup
    } else {
        RtfLine::Text(text)
    }
}

/// One kind's packets as they are read.
struct Kind<'a> {
    name: &'a str,
    count: usize,
    time_s: Vec<f64>,
    stages: Vec<f64>,
    values: Vec<Vec<f64>>,
}

/// The packets read so far, with the counts the notes give.
#[derive(Default)]
struct Reading<'a> {
    kinds: Vec<Kind<'a>>,
    heard: HashSet<(&'a str, &'a str, u64, Vec<&'a str>)>,
    events: Vec<Event>,
    pending_marks: Vec<String>,
    marks_seen: HashSet<String>,
    broken: usize,
    first_broken: Option<usize>,
    repeated: usize,
    repeated_marks: usize,
    gains: usize,
}

impl<'a> Reading<'a> {
    /// Takes a packet found on line `line`.
    fn take(
        &mut self,
        packet: Packet<'a>,
        format: &'static str,
        line: usize,
    ) -> Result<(), LogError> {
        if packet.written.len() > MAX_VALUES {
            return Err(syntax(
                format,
                line,
                format!("a packet holds more than {MAX_VALUES} values"),
            ));
        }
        if packet.clock_ms > CLOCK_LIMIT_MS {
            return Err(syntax(
                format,
                line,
                "a packet's clock is 2³² ms or more".to_owned(),
            ));
        }
        let index = match self.kinds.iter().position(|kind| kind.name == packet.kind) {
            Some(index) => index,
            None => {
                if self.kinds.len() >= MAX_KINDS {
                    return Err(syntax(
                        format,
                        line,
                        format!("the file holds more than {MAX_KINDS} kinds of packet"),
                    ));
                }
                self.kinds.push(Kind {
                    name: packet.kind,
                    count: packet.values.len(),
                    time_s: Vec::new(),
                    stages: Vec::new(),
                    values: vec![Vec::new(); packet.values.len()],
                });
                self.kinds.len() - 1
            }
        };
        if self.kinds[index].count != packet.values.len() {
            self.broke(line);
            return Ok(());
        }
        if !self.heard.insert((
            packet.stage,
            packet.kind,
            packet.clock_ms,
            packet.written.clone(),
        )) {
            self.repeated += 1;
            return Ok(());
        }
        // The clock's milliseconds as seconds: a shrinking conversion, which can't overflow.
        let time_s = packet.clock_ms as f64 / 1000.0;
        for name in self.pending_marks.drain(..) {
            self.events.push(Event { time_s, name });
        }
        let kind = &mut self.kinds[index];
        kind.time_s.push(time_s);
        kind.stages.push(f64::from(stage_number(packet.stage)));
        for (place, (column, value)) in kind.values.iter_mut().zip(&packet.values).enumerate() {
            let value = if packet.kind == "PP" && place == 0 {
                finite(value * HECTOPASCAL_PA, format, line, "PP 1")?
            } else {
                *value
            };
            column.push(value);
        }
        Ok(())
    }

    /// Takes a stage's mark; one already taken is a repeat.
    fn mark(&mut self, mark: &str) {
        if self.marks_seen.insert(mark.to_owned()) {
            self.pending_marks.push(mark.to_owned());
        } else {
            self.repeated_marks += 1;
        }
    }

    /// Counts a broken line.
    fn broke(&mut self, line: usize) {
        self.broken += 1;
        self.first_broken.get_or_insert(line);
    }

    /// The record, or a refusal if it holds no packet.
    fn finish(self, format: LogFormat, name: &'static str) -> Result<FlightLog, LogError> {
        if self.kinds.is_empty() {
            return Err(syntax(
                name,
                1,
                "the file holds no whole packet, `255`, its fields and `254`".to_owned(),
            ));
        }
        let mut notes = Vec::new();
        if self.gains > 0 {
            notes.push(format!(
                "{} `Gain[…]` lines, the flight computer's filter gains, kept out",
                self.gains
            ));
        }
        if let Some(line) = self.first_broken {
            notes.push(format!(
                "{} broken lines dropped, packets the radio cut or ran into the next, or with \
                 another count of values than their kind's first, the first on line {line}",
                self.broken
            ));
        }
        if self.repeated > 0 {
            notes.push(format!("{} packets heard again dropped", self.repeated));
        }
        if self.repeated_marks > 0 {
            notes.push(format!(
                "{} `Stage` lines heard again dropped",
                self.repeated_marks
            ));
        }
        for name in self.pending_marks {
            notes.push(format!(
                "the mark `{}` at the file's end, no packet after it, kept out",
                short(&name)
            ));
        }
        let mut channels = Vec::new();
        for kind in self.kinds {
            channels.push(Channel {
                column: format!("{} stage", kind.name),
                quantity: Quantity::Unrecognized,
                time_s: kind.time_s.clone(),
                values: kind.stages,
            });
            for (place, values) in kind.values.into_iter().enumerate() {
                let quantity = if kind.name == "PP" && place == 0 {
                    Quantity::Pressure
                } else {
                    Quantity::Unrecognized
                };
                channels.push(Channel {
                    column: format!("{} {}", kind.name, place + 1),
                    quantity,
                    time_s: kind.time_s.clone(),
                    values,
                });
            }
        }
        Ok(FlightLog {
            format,
            logger: "radio telemetry packets".to_owned(),
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
            events: self.events,
            notes,
        })
    }
}

/// A stage's two digits as a number.
fn stage_number(stage: &str) -> u8 {
    stage.bytes().fold(0, |sum, b| sum * 10 + (b - b'0'))
}

fn syntax(format: &'static str, line: usize, message: String) -> LogError {
    LogError::Syntax {
        format,
        line,
        message,
    }
}

/// Reads the packets' text form, [`LogFormat::RadioPacketText`]: the onboard log, or the text a
/// receiver wrote of what it heard.
///
/// # Errors
///
/// [`LogError::Syntax`] for a file the module's documentation says is refused.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut reading = Reading::default();
    let mut first = true;
    for (number, line) in numbered_lines(text) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let opening = std::mem::take(&mut first);
        if let Some(packet) = packet(line, ':') {
            reading.take(packet, TEXT, number)?;
        } else if let Some(mark) = stage_mark(line) {
            reading.mark(mark);
        } else if is_gain(line) {
            reading.gains += 1;
        } else if opening && line.len() <= 10 && line.bytes().all(|b| b.is_ascii_digit()) {
            // The onboard log's first line, a whole number that no packet explains.
        } else {
            reading.broke(number);
        }
    }
    reading.finish(LogFormat::RadioPacketText, TEXT)
}

/// Reads the exports' form, [`LogFormat::RadioPacketRtf`].
///
/// # Errors
///
/// [`LogError::Syntax`] for a file the module's documentation says is refused.
pub fn read_rtf(text: &str) -> Result<FlightLog, LogError> {
    let lines: Vec<(usize, &str)> = numbered_lines(text).collect();
    let last = lines
        .iter()
        .rposition(|(_, line)| !line.trim().is_empty())
        .unwrap_or(0);
    let mut reading = Reading::default();
    for (index, (number, line)) in lines.iter().enumerate() {
        let RtfLine::Text(line) = rtf_line(line.trim(), index == last) else {
            continue;
        };
        if let Some(packet) = packet(line, ',') {
            reading.take(packet, RTF, *number)?;
        } else if let Some(mark) = stage_mark(line) {
            reading.mark(mark);
        } else {
            reading.broke(*number);
        }
    }
    reading.finish(LogFormat::RadioPacketRtf, RTF)
}
