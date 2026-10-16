//! A serial console's capture holding binary records as base64: what a flight computer prints when
//! asked, over its USB serial shell, to dump its memory, as a terminal program saves it.
//!
//! The capture is the terminal's text: the commands typed at the shell, then each dump
//! as a block of base64 lines between two marker lines that name it:
//!
//! ```text
//! uart:~$ log dump
//! ----++++//[[( data start )]]\\++++----
//! 6AMAAJqZykLyAwAAe5TKQvwDAABcj8pCBgQAAD2KykI=
//! …
//! ----++++//[[( data end )]]\\++++----
//! uart:~$
//! ```
//!
//! The markers are the firmware's own, a team's flight computer's, and its blocks hold binary
//! records whose layout the capture doesn't state. So [`blocks`] decodes each block's base64 to
//! its bytes, and [`read`] reads a block's records by the layout the caller states, as
//! [`records::read`] does. [`crate::import::read`] recognizes a capture and refuses it by name,
//! as it has no layout to read it by.
//!
//! **Recognition.** A start marker, `----++++//[[( ` then the block's name then
//! ` start )]]\\++++----`, on a line of its own within the file's first [`HEAD_LINES`] lines. A
//! name is 1 to [`MAX_NAME_CHARS`] ASCII letters, digits, `_` or `-`.
//!
//! **Blocks.** Each line between a block's markers is base64, RFC 4648's standard alphabet, its
//! spaces at the ends taken off; blank lines are skipped. The lines are joined and decoded as one,
//! its length a multiple of 4, its `=` padding only at the end and the bits the padding leaves over
//! zero (RFC 4648 §3.5). A character outside the
//! alphabet is refused, naming its line, as is a block that never ends, a marker inside a block, an
//! end marker with no block open, or two blocks of one name. Lines outside the blocks, the
//! shell's, are skipped.
//!
//! How far to trust it: the base64 is decoded exactly; the records are as far as their layout is
//! ([`records`]). The project's three sample captures, each held to the table the team decoded
//! from the same records, are read in `cargo xtask logs` ([ADR-271][adr-271]).
//!
//! [adr-271]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::error::LogError;
use crate::log::{FlightLog, LogFormat};
use crate::records::{self, RecordLayout};
use crate::table::{numbered_lines, short};

/// How many of the file's first lines are searched for a start marker.
pub const HEAD_LINES: usize = 1000;

/// The most characters in a block's name.
pub const MAX_NAME_CHARS: usize = 64;

/// The format's name, as its errors give it.
const FORMAT: &str = "console capture";

/// What opens a marker line, before the block's name.
const OPEN: &str = "----++++//[[( ";

/// What closes a start marker line, after the block's name.
const START: &str = " start )]]\\\\++++----";

/// What closes an end marker line, after the block's name.
const END: &str = " end )]]\\\\++++----";

/// One block of a capture, decoded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Block {
    /// The block's name, as its markers write it, such as `data`.
    pub name: String,
    /// The line of its start marker, from 1.
    pub line: usize,
    /// Its bytes, the base64 decoded.
    pub bytes: Vec<u8>,
}

/// A marker line's block name and whether it starts the block; `None` for another line.
fn marker(line: &str) -> Option<(&str, bool)> {
    let rest = line.trim().strip_prefix(OPEN)?;
    let (name, start) = if let Some(name) = rest.strip_suffix(START) {
        (name, true)
    } else {
        (rest.strip_suffix(END)?, false)
    };
    let named = !name.is_empty()
        && name.len() <= MAX_NAME_CHARS
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'));
    named.then_some((name, start))
}

/// Whether the text is a console capture of base64 blocks (module docs, *Recognition*).
pub fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .take(HEAD_LINES)
        .any(|(_, line)| marker(line).is_some_and(|(_, start)| start))
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// The value of a base64 character, RFC 4648 table 1.
fn sextet(byte: u8) -> Option<u32> {
    match byte {
        b'A'..=b'Z' => Some(u32::from(byte - b'A')),
        b'a'..=b'z' => Some(u32::from(byte - b'a') + 26),
        b'0'..=b'9' => Some(u32::from(byte - b'0') + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// A block's base64, decoded as its lines come (module docs, *Blocks*): only the bytes are kept,
/// so the decoder takes no more memory than the block's three quarters of its text.
struct Base64<'a> {
    /// The block's name, for errors.
    name: &'a str,
    /// The bytes decoded so far.
    bytes: Vec<u8>,
    /// The characters of the group of 4 being read, as values, and how many there are.
    group: [u32; 4],
    filled: usize,
    /// Every character read, `=` included.
    count: usize,
    /// The `=` read so far, and the line of the first.
    padding: usize,
    padding_line: usize,
    /// The line of the last character read, and that character's value if it isn't `=`.
    last_line: usize,
    last_value: u32,
}

impl<'a> Base64<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            bytes: Vec::new(),
            group: [0; 4],
            filled: 0,
            count: 0,
            padding: 0,
            padding_line: 0,
            last_line: 0,
            last_value: 0,
        }
    }

    /// The characters of one line, its spaces at the ends taken off.
    fn line(&mut self, number: usize, line: &str) -> Result<(), LogError> {
        let name = self.name;
        for byte in line.bytes() {
            self.count += 1;
            self.last_line = number;
            if byte == b'=' {
                if self.padding == 0 {
                    self.padding_line = number;
                }
                self.padding += 1;
                if self.padding > 2 {
                    return Err(syntax(
                        number,
                        format!(
                            "the block `{name}` ends in 3 `=` or more, where base64 pads with 2 at most"
                        ),
                    ));
                }
                continue;
            }
            let Some(value) = sextet(byte) else {
                let what = if byte.is_ascii_graphic() {
                    format!("`{}`", char::from(byte))
                } else {
                    format!("the byte 0x{byte:02X}")
                };
                return Err(syntax(
                    number,
                    format!("the block `{name}` holds {what}, which isn't base64"),
                ));
            };
            if self.padding > 0 {
                return Err(syntax(
                    self.padding_line,
                    format!("the block `{name}` holds `=` before its end, where only padding may"),
                ));
            }
            self.last_value = value;
            self.group[self.filled] = value;
            self.filled += 1;
            if self.filled == 4 {
                let word = self
                    .group
                    .iter()
                    .fold(0u32, |word, value| word << 6 | value);
                self.bytes.extend(&word.to_be_bytes()[1..]);
                self.filled = 0;
            }
        }
        Ok(())
    }

    /// The bytes, once the block's lines are all read.
    fn finish(mut self, start: usize) -> Result<Vec<u8>, LogError> {
        let name = self.name;
        let line = if self.count == 0 {
            start
        } else {
            self.last_line
        };
        if !self.count.is_multiple_of(4) {
            return Err(syntax(
                line,
                format!(
                    "the block `{name}` holds {} base64 characters, not a multiple of 4",
                    self.count
                ),
            ));
        }
        // A multiple of 4 with at most 2 `=` leaves 4 − padding characters in the last group, so
        // `filled` is 3 under one `=`, 2 under two, and 0 with none.
        if self.padding > 0 {
            // The bits past the last byte, in the last character before the padding, must be
            // zero: 4 of them under two `=`, 2 under one (RFC 4648 §3.5), or the text isn't one
            // encoder's.
            let spare = if self.padding == 2 { 0b1111 } else { 0b11 };
            if self.last_value & spare != 0 {
                return Err(syntax(
                    self.padding_line,
                    format!("the block `{name}` ends in bits its padding should leave zero"),
                ));
            }
            let word = self.group[..self.filled]
                .iter()
                .fold(0u32, |word, value| word << 6 | value)
                << (6 * self.padding);
            self.bytes
                .extend(&word.to_be_bytes()[1..1 + (3 - self.padding)]);
        }
        Ok(self.bytes)
    }
}

/// A block's base64, its lines with their numbers, decoded (module docs, *Blocks*).
#[cfg(test)]
fn decode(name: &str, start: usize, lines: &[(usize, &str)]) -> Result<Vec<u8>, LogError> {
    let mut base64 = Base64::new(name);
    for &(number, line) in lines {
        base64.line(number, line)?;
    }
    base64.finish(start)
}

/// The block open while [`blocks`] walks the lines: its name, its marker's line, and its decoder.
type Open<'a> = (&'a str, usize, Base64<'a>);

/// Each block of a capture, decoded, in the file's order (module docs, *Blocks*).
///
/// # Errors
///
/// [`LogError::NotThisFormat`] for a text that isn't a capture; [`LogError::Syntax`], naming the
/// line, for a character outside base64's alphabet, padding out of place or over bits that aren't
/// zero, a block whose base64 isn't a multiple of 4 characters, a block that never ends, a marker
/// inside a block, an end marker with no block open, or two blocks of one name.
pub fn blocks(text: &str) -> Result<Vec<Block>, LogError> {
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "within its first {HEAD_LINES} lines there is no start marker, such as \
                 `----++++//[[( data start )]]\\\\++++----`"
            ),
        });
    }
    let mut blocks: Vec<Block> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut open: Option<Open<'_>> = None;
    for (number, line) in numbered_lines(text) {
        match (marker(line), open.take()) {
            (Some((name, true)), None) => {
                if !seen.insert(name) {
                    return Err(syntax(number, format!("a second block named `{name}`")));
                }
                open = Some((name, number, Base64::new(name)));
            }
            (Some((name, false)), Some((open_name, start, base64))) if name == open_name => {
                blocks.push(Block {
                    name: name.to_owned(),
                    line: start,
                    bytes: base64.finish(start)?,
                });
            }
            (Some((name, _)), Some((open_name, start, _))) => {
                return Err(syntax(
                    number,
                    format!(
                        "a marker of the block `{name}` inside the block `{open_name}`, opened \
                         at line {start}"
                    ),
                ));
            }
            (Some((name, false)), None) => {
                return Err(syntax(
                    number,
                    format!("the block `{name}` ends here but never started"),
                ));
            }
            (None, Some((name, start, mut base64))) => {
                base64.line(number, line.trim())?;
                open = Some((name, start, base64));
            }
            (None, None) => {}
        }
    }
    if let Some((name, start, _)) = open {
        return Err(syntax(
            start,
            format!("the block `{name}`, opened here, never ends"),
        ));
    }
    Ok(blocks)
}

/// The most block names an error lists; the rest are counted.
const LISTED: usize = 8;

/// Block names as an error lists them: the first [`LISTED`], the rest counted.
fn listed<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let mut named = Vec::new();
    let mut more = 0usize;
    for name in names {
        if named.len() < LISTED {
            named.push(format!("`{name}`"));
        } else {
            more += 1;
        }
    }
    if more > 0 {
        named.push(format!("{more} more"));
    }
    named.join(", ")
}

/// Why [`crate::import::read`] refuses a capture: it has no layout to read the records by. The
/// blocks are named from their start markers, not decoded.
pub(crate) fn needs_a_layout(text: &str) -> LogError {
    let names = numbered_lines(text)
        .filter_map(|(_, line)| marker(line))
        .filter(|(_, start)| *start)
        .map(|(name, _)| name);
    LogError::Layout {
        message: format!(
            "a serial console's capture of binary records as base64, its blocks {}: the capture \
             doesn't state the records' layout, so read a block by the one the logger writes \
             (`capture::read`)",
            listed(names)
        ),
    }
}

/// Reads the block `block` of a capture by the layout the caller states, as [`records::read`]
/// reads a file of records, as [`LogFormat::ConsoleCapture`].
///
/// # Errors
///
/// As [`blocks`]; [`LogError::Layout`] for a block the capture doesn't hold; otherwise as
/// [`records::read`].
pub fn read(text: &str, block: &str, layout: &RecordLayout) -> Result<FlightLog, LogError> {
    let blocks = blocks(text)?;
    let Some(found) = blocks.iter().find(|candidate| candidate.name == block) else {
        return Err(LogError::Layout {
            message: format!(
                "the capture holds no block `{}`; its blocks are {}",
                short(block),
                listed(blocks.iter().map(|b| b.name.as_str()))
            ),
        });
    };
    let table = records::table(&found.bytes, layout)?;
    let mut log = records::read_table(&table)?;
    log.format = LogFormat::ConsoleCapture;
    log.notes.insert(
        0,
        format!(
            "read from the block `{}` of a console capture, line {}: {} bytes of base64 decoded",
            found.name,
            found.line,
            found.bytes.len()
        ),
    );
    Ok(log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::{Field, FieldKind};

    /// RFC 4648 §10's test vectors.
    #[test]
    fn base64_decodes_rfc_4648s_vectors() {
        for (encoded, decoded) in [
            ("", ""),
            ("Zg==", "f"),
            ("Zm8=", "fo"),
            ("Zm9v", "foo"),
            ("Zm9vYg==", "foob"),
            ("Zm9vYmE=", "fooba"),
            ("Zm9vYmFy", "foobar"),
        ] {
            let lines = [(2, encoded)];
            let lines = if encoded.is_empty() {
                &lines[..0]
            } else {
                &lines[..]
            };
            assert_eq!(
                decode("d", 1, lines).unwrap(),
                decoded.as_bytes(),
                "{encoded}"
            );
        }
        // Across lines, the same as joined.
        assert_eq!(
            decode("d", 1, &[(2, "Zm9v"), (3, "YmFy")]).unwrap(),
            b"foobar"
        );
        assert_eq!(
            decode("d", 1, &[(2, "Zm9vY"), (3, "mFy")]).unwrap(),
            b"foobar"
        );
    }

    #[test]
    fn base64_out_of_its_alphabet_or_form_is_refused_at_its_line() {
        type Case<'a> = (&'a [(usize, &'a str)], usize, &'a str);
        let cases: [Case<'_>; 10] = [
            // A character out of the alphabet is named at its line, whatever the block's length.
            (
                &[(3, "Zm-v"), (4, "YmFy"), (5, "YmF")],
                3,
                "holds `-`, which isn't base64",
            ),
            (
                &[(2, "Zm9v"), (3, "Zm 9v")],
                3,
                "holds the byte 0x20, which isn't base64",
            ),
            (
                &[(2, "Zm9v"), (3, "Ym-y")],
                3,
                "holds `-`, which isn't base64",
            ),
            (
                &[(2, "Zm9v"), (3, "YmF")],
                3,
                "7 base64 characters, not a multiple of 4",
            ),
            (&[(2, "Zg=="), (3, "Zm9v")], 2, "`=` before its end"),
            (&[(2, "Z=9v")], 2, "`=` before its end"),
            (&[(2, "Zm9v"), (3, "Y===")], 3, "ends in 3 `=`"),
            (
                &[(2, "Zm9v"), (3, "Zh==")],
                3,
                "bits its padding should leave zero",
            ),
            (
                &[(2, "Zm9v"), (3, "Zm9=")],
                3,
                "bits its padding should leave zero",
            ),
            (
                &[(2, "Zm9v"), (3, "Ym\u{e9}")],
                3,
                "holds the byte 0xC3, which isn't base64",
            ),
        ];
        for (lines, at, why) in cases {
            match decode("d", 1, lines) {
                Err(LogError::Syntax { line, message, .. }) => {
                    assert_eq!(line, at, "{message}");
                    assert!(message.contains(why), "{message} lacks {why}");
                }
                other => panic!("expected a refusal for {why}, got {other:?}"),
            }
        }
    }

    fn capture(blocks: &[(&str, &str)]) -> String {
        let mut text = String::from("uart:~$ dump\n");
        for (name, base64) in blocks {
            text += &format!("----++++//[[( {name} start )]]\\\\++++----\n{base64}\n");
            text += &format!("----++++//[[( {name} end )]]\\\\++++----\nuart:~$ \n");
        }
        text
    }

    #[test]
    fn a_capture_is_recognized_by_a_start_marker_only() {
        assert!(recognizes(&capture(&[("data", "Zm9v")])));
        assert!(recognizes(
            "\r\n  ----++++//[[( a-b_9 start )]]\\\\++++----  \r\n"
        ));
        for text in [
            "----++++//[[( data end )]]\\\\++++----\n",
            "----++++//[[(  start )]]\\\\++++----\n",
            "----++++//[[( da ta start )]]\\\\++++----\n",
            "----++++//[[( data start )]]\\++++----\n",
            "x ----++++//[[( data start )]]\\\\++++----\n",
            "uart:~$ dump\n",
        ] {
            assert!(!recognizes(text), "{text:?}");
        }
        let long = format!(
            "----++++//[[( {} start )]]\\\\++++----\n",
            "n".repeat(MAX_NAME_CHARS)
        );
        assert!(recognizes(&long));
        let longer = long.replacen('n', "nn", 1);
        assert!(!recognizes(&longer));
        // Past the first lines, a marker isn't looked for.
        let late = "\n".repeat(HEAD_LINES) + &capture(&[("data", "Zm9v")]);
        assert!(!recognizes(&late));
        let within = "\n".repeat(HEAD_LINES - 1) + "----++++//[[( d start )]]\\\\++++----\n";
        assert!(recognizes(&within));
    }

    #[test]
    fn blocks_decode_in_order_with_the_shells_lines_skipped() {
        let text = capture(&[("params", "Zm9v"), ("data", "Zm9v\n\n  YmFy  ")]);
        let blocks = blocks(&text).unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!((blocks[0].name.as_str(), blocks[0].line), ("params", 2));
        assert_eq!(blocks[0].bytes, b"foo");
        assert_eq!((blocks[1].name.as_str(), blocks[1].line), ("data", 6));
        assert_eq!(blocks[1].bytes, b"foobar");
    }

    #[test]
    fn a_malformed_capture_is_refused_naming_the_line() {
        let start = |name: &str| format!("----++++//[[( {name} start )]]\\\\++++----\n");
        let end = |name: &str| format!("----++++//[[( {name} end )]]\\\\++++----\n");
        let cases = [
            (
                format!("{}Zm9v\n", start("data")),
                1,
                "the block `data`, opened here, never ends",
            ),
            (
                format!("{}Zm9v\n{}{}", start("data"), start("more"), end("data")),
                3,
                "a marker of the block `more` inside the block `data`, opened at line 1",
            ),
            (
                format!("{}{}{}", start("a"), end("b"), end("a")),
                2,
                "a marker of the block `b` inside the block `a`",
            ),
            (
                format!("{}{}{}", start("a"), end("a"), end("a")),
                3,
                "the block `a` ends here but never started",
            ),
            (
                format!("{}{}{}{}", start("a"), end("a"), start("a"), end("a")),
                3,
                "a second block named `a`",
            ),
        ];
        for (text, at, why) in cases {
            match blocks(&text) {
                Err(LogError::Syntax { line, message, .. }) => {
                    assert_eq!(line, at, "{message}");
                    assert!(message.contains(why), "{message} lacks {why}");
                }
                other => panic!("expected a refusal for {why}, got {other:?}"),
            }
        }
        assert!(matches!(
            blocks("uart:~$\n"),
            Err(LogError::NotThisFormat { .. })
        ));
    }

    fn base64(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let mut word = [0u8; 3];
            word[..chunk.len()].copy_from_slice(chunk);
            let n = u32::from_be_bytes([0, word[0], word[1], word[2]]);
            for i in 0..4 {
                if i <= chunk.len() {
                    out.push(char::from(ALPHABET[(n >> (18 - 6 * i) & 63) as usize]));
                } else {
                    out.push('=');
                }
            }
        }
        out
    }

    #[test]
    fn a_block_is_read_by_the_callers_layout() {
        let mut bytes = Vec::new();
        for (ms, kpa) in [(1000_u32, 101.3_f32), (1010, 101.2), (1020, 101.1)] {
            bytes.extend(ms.to_le_bytes());
            bytes.extend(kpa.to_le_bytes());
        }
        // Dumped from flash 48 bytes a line, past the last record into erased memory.
        bytes.extend([0xFF; 4]);
        let encoded = base64(&bytes);
        let text = capture(&[("params", "AAAA"), ("data", &encoded)]);
        let layout = RecordLayout::new(vec![
            Field::new("timestamp__ms", FieldKind::U32),
            Field::new("pressure__kpa", FieldKind::F32),
        ]);
        let log = read(&text, "data", &layout).unwrap();
        assert_eq!(log.format, LogFormat::ConsoleCapture);
        assert_eq!(log.channels[0].time_s, [1.0, 1.01, 1.02]);
        assert_eq!(log.channels[0].values[2], f64::from(101.1_f32) * 1000.0);
        assert_eq!(
            log.notes[..3],
            [
                "read from the block `data` of a console capture, line 6: 28 bytes of base64 decoded",
                "3 binary records read by the caller's layout, written out as a table and read by its header",
                "4 bytes past the last whole record, each 0xFF, dropped as erased memory",
            ]
        );
        match read(&text, "flight", &layout) {
            Err(LogError::Layout { message }) => {
                assert!(
                    message.contains("no block `flight`; its blocks are `params`, `data`"),
                    "{message}"
                );
            }
            other => panic!("expected a layout refusal, got {other:?}"),
        }
    }

    #[test]
    fn import_refuses_a_capture_for_its_layout_listing_a_few_blocks() {
        let named: Vec<(String, &str)> = (0..10).map(|i| (format!("b{i}"), "Zm9v")).collect();
        let named: Vec<(&str, &str)> = named.iter().map(|(n, b)| (n.as_str(), *b)).collect();
        let text = capture(&named);
        assert_eq!(
            crate::import::detect(text.as_bytes()),
            Some(LogFormat::ConsoleCapture)
        );
        match crate::import::read(text.as_bytes()) {
            Err(LogError::Layout { message }) => {
                assert!(message.contains("`b0`, `b1`"), "{message}");
                assert!(message.contains("`b7`, 2 more:"), "{message}");
                assert!(!message.contains("`b8`"), "{message}");
                assert!(message.contains("capture::read"), "{message}");
            }
            other => panic!("expected a layout refusal, got {other:?}"),
        }
        // The blocks are named from their markers, so a block that doesn't decode is named too.
        let broken = capture(&[("data", "Zm9")]);
        match crate::import::read(broken.as_bytes()) {
            Err(LogError::Layout { message }) => {
                assert!(message.contains("its blocks `data`:"), "{message}");
            }
            other => panic!("expected a layout refusal, got {other:?}"),
        }
    }
}
