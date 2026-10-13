//! What the readers of one-clock tables share: an Eggtimer's or an RRC3's export, a header row
//! naming the columns and then one row per sample, every column on the row's time.
//!
//! Lines end in CR, LF or both. A cell is trimmed of spaces. A number is written with a point, or,
//! in a file whose cells are separated by something other than a comma, with a comma, as a
//! European locale writes `1,5` for 1.5: the comma can't be both a separator and a decimal mark.

use crate::error::LogError;

/// The most characters of the file an error or note quotes.
const QUOTED_CHARS: usize = 80;

/// The most notes about single lines a log keeps; past it, one more note counts the rest.
const MAX_NOTES: usize = 20;

/// The file's lines, numbered from 1, with any byte-order mark dropped and each line's end, CR, LF
/// or both, taken off.
pub(crate) fn numbered_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut rest = Some(text);
    let mut number = 0;
    std::iter::from_fn(move || {
        let current = rest?;
        number += 1;
        match current.find(['\r', '\n']) {
            Some(end) => {
                let skip = if current[end..].starts_with("\r\n") {
                    2
                } else {
                    1
                };
                rest = Some(&current[end + skip..]);
                Some((number, &current[..end]))
            }
            None => {
                rest = None;
                (!current.is_empty()).then_some((number, current))
            }
        }
    })
}

/// A cell as a number, `None` if it isn't one: a finite decimal, with a point, or with a comma if
/// `decimal_comma`.
pub(crate) fn number(cell: &str, decimal_comma: bool) -> Option<f64> {
    let cell = cell.trim();
    let parsed = if decimal_comma && cell.contains(',') && !cell.contains('.') {
        cell.replacen(',', ".", 1).parse::<f64>()
    } else {
        cell.parse::<f64>()
    };
    parsed.ok().filter(|value| value.is_finite())
}

/// A value after its unit conversion, refused if the conversion made it infinite: a finite cell
/// near the largest `f64`, such as `1.7e308` g, overflows when it is scaled to SI, and a channel
/// holds SI values or `NaN` gaps, nothing else ([ADR-258][adr-258] §3). `NaN`, a gap, passes.
/// The error names the row's line, or the line of the figure for a value a file states once.
///
/// Only a conversion that can grow a value can overflow, and the readers check each of those:
/// g to m/s² (× 9.806 65), thousandths of a g (multiplied by 9.806 65 before the division by
/// 1,000), atmospheres, hectopascals, kilopascals and inches of mercury to pascals, degrees
/// Fahrenheit to kelvins (`(v − 32) × 5 / 9`, whose `× 5` overflows first), radians per second
/// to degrees per second (× 180/π) and thousands of feet to meters (× 304.8). A shrinking one can't
/// overflow and isn't checked: feet to meters, milliseconds to seconds, knots to m/s, and
/// degrees Celsius to kelvins, whose `+ 273.15` keeps a finite value finite.
///
/// [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
pub(crate) fn finite(
    value: f64,
    format: &'static str,
    line: usize,
    column: &str,
) -> Result<f64, LogError> {
    if value.is_infinite() {
        Err(LogError::Syntax {
            format,
            line,
            message: format!(
                "the column {} holds a value too large for its unit's conversion to SI",
                short(column)
            ),
        })
    } else {
        Ok(value)
    }
}

/// Text from the file as an error or note quotes it: cut to [`QUOTED_CHARS`] characters, so a
/// binary file's "first line" doesn't fill the message.
pub(crate) fn short(text: &str) -> String {
    if text.chars().count() <= QUOTED_CHARS {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(QUOTED_CHARS - 1).collect();
        cut.push('…');
        cut
    }
}

/// Notes about single lines as the reader meets them: the first [`MAX_NOTES`] kept, the rest
/// only counted, so a file of many broken lines can't fill memory with them.
#[derive(Default)]
pub(crate) struct Notes {
    kept: Vec<String>,
    left_out: usize,
}

impl Notes {
    pub(crate) fn push(&mut self, note: String) {
        if self.kept.len() < MAX_NOTES {
            self.kept.push(note);
        } else {
            self.left_out += 1;
        }
    }

    /// The notes kept, then one counting those left out, if any.
    pub(crate) fn into_vec(self) -> Vec<String> {
        let mut notes = self.kept;
        if self.left_out > 0 {
            notes.push(format!(
                "and {} more notes about single lines, left out so that a broken file's list \
                 stays short",
                self.left_out
            ));
        }
        notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_end_in_cr_lf_or_both() {
        let lines: Vec<(usize, &str)> = numbered_lines("\u{feff}a\r\nb\rc\n\nd").collect();
        assert_eq!(lines, [(1, "a"), (2, "b"), (3, "c"), (4, ""), (5, "d")]);
        assert_eq!(numbered_lines("a\n").count(), 1);
        assert_eq!(numbered_lines("").count(), 0);
    }

    #[test]
    fn numbers_take_a_comma_only_where_it_isnt_the_separator() {
        assert_eq!(number(" 1.5 ", false), Some(1.5));
        assert_eq!(number("1,5", true), Some(1.5));
        assert_eq!(number("-0,25", true), Some(-0.25));
        assert_eq!(number("1,5", false), None);
        assert_eq!(number("1,000.5", true), None);
        assert_eq!(number("1,2,3", true), None);
        for cell in ["", "-", "NaN", "inf", "1e999", "12:30"] {
            assert_eq!(number(cell, true), None, "{cell:?}");
        }
    }

    #[test]
    fn notes_stay_few() {
        let mut notes = Notes::default();
        for index in 0..25 {
            notes.push(format!("note {index}"));
        }
        let notes = notes.into_vec();
        assert_eq!(notes.len(), MAX_NOTES + 1);
        assert!(notes[MAX_NOTES].starts_with("and 5 more"));
        assert_eq!(short(&"x".repeat(200)).chars().count(), QUOTED_CHARS);
    }
}
