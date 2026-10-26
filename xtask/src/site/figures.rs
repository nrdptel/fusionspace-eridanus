//! Figures and dates in the text of the site's pages and the READMEs (#476, ADR-295), as the
//! product system's `data.md` and `writing.md` write them. Both rules read the text as it
//! renders (prose, headings, list items, table cells, link text and image descriptions; not
//! code, which quotes files and output, nor math, nor a block of raw HTML):
//!
//! - **Digits grouped in threes** (`data.md`, *Numbers*: "Group digits in threes with a comma,
//!   from four digits"). [`ungrouped_figures`] fails a figure of four or more digits before its
//!   decimal point written without the commas: `1500 m`, `12000 ft`. Digits after a decimal
//!   point aren't grouped. What isn't a quantity passes:
//!   - a figure inside a word or a code (`131G84`, `7268bcd`, `ADR-0295`, `#1000`, `D-1234`),
//!     with a letter, `_` or `#` against it, a `/`, `\`, `@`, `+`, `&` or `=` before it, a
//!     hyphen after a letter before it, a colon or a point right after a letter or a bracket
//!     (`arXiv:1109.4448`, `NGA.STND.0036`, `13(11):15692`), or a hyphen or a bracket before it
//!     in a code that holds a letter (`NASA/TM-2008-215633`, `050X(80)90013-3`);
//!   - an ISO date's year (`2026-10-04`), which the date rule reads; figures a hyphen joins
//!     otherwise (`1000-2000 ft`, `3005-row`) are read;
//!   - a figure right after a name that numbers documents or things ([`IDENTIFIERS`]:
//!     `NFPA 1127`, `ISO 8601`, `NACA 0012`, `station 72364`), and the second figure of a range
//!     after one (`pp. 1509–1518`);
//!   - a year ([`is_year`]): four digits from 1600 to 2100 with no unit or `%` after them
//!     (`in 1997`, but `2000 m` and `1500 ft` fail; `in` is read as the word, not inches), or any
//!     figure after a month, a day and a comma (`December 31, 9999`).
//! - **Dates in prose as words** (`writing.md`, *On screens*: "`October 4, 2026` in prose;
//!   `2026-10-04` in tables, file names, title blocks and data"). [`prose_dates`] fails an ISO
//!   date, `2026-10-04`, in the text outside tables. A date in a file's name is code, which isn't
//!   read; one in a link's address isn't text.
//!
//! The rules read figures, not meaning: a count that looks like a year (`2000 flights`) passes,
//! as does a quantity after a listed name, and an identifier after a name the list doesn't hold
//! fails; reviews hold the rest, as `docs/writing.md` says of every check.

use std::ops::Range;

use super::units::{Run, rendered_runs};

/// The names after which a figure numbers a document, a standard or a thing rather than counts
/// or measures it (the word right before the figure, as written), each one a page needs:
/// standards and their codes (`ISO 8601`, `NFPA 1125`, `RFC 4180`, `NMEA 0183`, EPSG's
/// `code 4269`), NACA's sections and the report series of NACA, NASA and the Navy (`NACA 0012`,
/// `TN 3527`, `TP 6575`, `NTRS 19670020050`, `arXiv 2511.04725`), alloys and a resin system
/// (`Al 6061`, `Hexcel 8552`), a sounding station, a forum thread and a source's pages.
const IDENTIFIERS: [&str; 23] = [
    // Standards and their bodies.
    "ISO", "NFPA", "RFC", "NMEA", "ESDU", "datum", "Code", "code",
    // NACA's sections, and NACA's, NASA's and the Navy's report series and archives.
    "NACA", "TN", "TR", "TP", "Report", "NTRS", "arXiv",
    // Named materials: an alloy, a resin system.
    "Aluminum", "Al", "Hexcel", "HexPly",
    // Things by number: a sounding station, a forum thread, a source's pages.
    "station", "thread", "Thread", "pp.",
];

/// The words after a four-digit figure that make it a quantity, not a year: the units of
/// `data.md` and the project's own, in short and in full.
const UNIT_WORDS: [&str; 69] = [
    "m", "km", "cm", "mm", "µm", "ft", "mi", "s", "ms", "min", "h", "kg", "g", "lb", "lbf", "oz",
    "N", "kN", "Ns", "N·s", "lbf·s", "Pa", "kPa", "hPa", "MPa", "GPa", "psi", "ksi", "Msi", "K",
    "°C", "°F", "°", "rad", "deg", "Hz", "kHz", "MHz", "J", "W", "mph", "kt", "kn", "m/s", "ft/s",
    "km/h", "kg/m³", "g/cm³", "m²", "m³", "ft²", "cal", "calibers", "AGL", "MSL", "B", "kB", "MB",
    "GB", "KiB", "MiB", "bytes", "px", "rpm", "feet", "meters", "seconds", "pounds", "newtons",
];

/// Whether the figure `digits`, followed by `after` (the run's text after it), reads as a year:
/// four digits from 1600 to 2100, with no unit word ([`UNIT_WORDS`]) or `%` after it.
fn is_year(digits: &str, after: &str) -> bool {
    let Ok(value) = digits.parse::<u32>() else {
        return false;
    };
    if digits.len() != 4 || !(1600..=2100).contains(&value) {
        return false;
    }
    let after = after.trim_start_matches([' ', '\u{a0}']);
    if after.starts_with('%') {
        return false;
    }
    let word: String = after
        .chars()
        .take_while(|c| c.is_alphanumeric() || matches!(c, '·' | '/' | '²' | '³' | '°'))
        .collect();
    !UNIT_WORDS.contains(&word.as_str())
}

/// Each figure of four or more digits on a page written without its grouping commas, as (line,
/// message).
pub(super) fn ungrouped_figures(text: &str) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let mut problems = Vec::new();
    for run in rendered_runs(text) {
        for digits in ungrouped(&run.text) {
            let figure = &run.text[digits.clone()];
            problems.push((
                line(run.source(digits.start)),
                format!(
                    "`{figure}` in \"{}\" has four or more digits ungrouped: write `{}` (data.md, \
                     Numbers: group digits in threes with a comma, from four digits)",
                    context(&run.text, digits.clone()),
                    grouped(figure)
                ),
            ));
        }
    }
    problems
}

/// The byte ranges in `run` of each figure [`ungrouped_figures`] fails.
fn ungrouped(run: &str) -> Vec<Range<usize>> {
    let bytes = run.as_bytes();
    let mut found = Vec::new();
    // The end of the last figure read as an identifier, if the last figure was one.
    let mut last_identifier: Option<usize> = None;
    let mut at = 0;
    while at < bytes.len() {
        if !bytes[at].is_ascii_digit() {
            at += 1;
            continue;
        }
        let start = at;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            at += 1;
        }
        let digits = start..at;
        // The second figure of a range whose first is an identifier is one too: pages
        // `pp. 1509–1518`.
        let continues = last_identifier.is_some_and(|end| run[end..start] == *"–");
        if continues || IDENTIFIERS.contains(&previous_word(&run[..start])) {
            last_identifier = Some(at);
            continue;
        }
        last_identifier = None;
        if digits.len() >= 4 && !passes(run, digits.clone()) {
            found.push(digits);
        }
    }
    found
}

/// Whether the digits at `digits` in `run` are something other than an ungrouped quantity: a
/// decimal part, a piece of a word or a code, a joined figure, an identifier or a year.
fn passes(run: &str, digits: Range<usize>) -> bool {
    let before = run[..digits.start].chars().next_back();
    let before_that = run[..digits.start].chars().rev().nth(1);
    let after_text = &run[digits.end..];
    let after = after_text.chars().next();
    // A decimal part: digits after a point aren't grouped.
    if before == Some('.') && before_that.is_some_and(|c| c.is_ascii_digit()) {
        return true;
    }
    // Inside a word or a code.
    let joins = |c: Option<char>| c.is_some_and(|c| c.is_alphabetic() || matches!(c, '_' | '#'));
    if joins(before) || joins(after) {
        return true;
    }
    if matches!(before, Some('/' | '\\' | '@' | '+' | '&' | '=')) {
        return true;
    }
    // A code's piece after a colon or a point that follows a letter or a bracket, with no
    // space: `arXiv:1109.4448`, `NGA.STND.0036`, `zenodo.32156`, `13(11):15692`.
    if matches!(before, Some(':' | '.'))
        && before_that.is_some_and(|c| c.is_alphabetic() || c == ')')
    {
        return true;
    }
    // A code's piece right after a bracket, in a code that holds a letter: a DOI's
    // `0771-050X(80)90013-3`.
    if before == Some(')') && in_code(run, digits.clone()) {
        return true;
    }
    // A hyphen joining it to a word before it (`ADR-0295`, `D-4014`), or to a figure in a code
    // that holds a letter (`NASA/TM-2008-215633`), or an ISO date's year, which the date rule
    // reads. Figures joined by a hyphen otherwise (`1000-2000 ft`) are read.
    if before == Some('-')
        && before_that.is_some_and(|c| {
            c.is_alphabetic() || c.is_ascii_digit() && in_code(run, digits.clone())
        })
    {
        return true;
    }
    if iso_dates(&run[digits.start..])
        .first()
        .is_some_and(|(at, ..)| *at == 0)
    {
        return true;
    }
    is_year(&run[digits.clone()], after_text) || dated(&run[..digits.start])
}

/// Whether `before` ends with a month and a day and a comma, so a figure after it is the
/// date's year, whatever it is: `December 31, 9999`.
fn dated(before: &str) -> bool {
    let Some(rest) = before.strip_suffix(", ") else {
        return false;
    };
    let day_start = rest
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map_or(rest.len(), |(at, _)| at);
    let (month, day) = rest.split_at(day_start);
    (1..=2).contains(&day.len())
        && MONTHS
            .iter()
            .any(|name| month.strip_suffix(' ').is_some_and(|m| m.ends_with(name)))
}

/// Whether the figure at `digits` is in a token of `run` (the text between spaces) that holds
/// a letter, as a report number or a DOI does: `NASA/TM-2008-215633`, `AS.1943-5525.0001331`.
fn in_code(run: &str, digits: Range<usize>) -> bool {
    let space = |c: char| c.is_whitespace();
    let start = run[..digits.start].rfind(space).map_or(0, |at| at + 1);
    let end = run[digits.end..]
        .find(space)
        .map_or(run.len(), |at| digits.end + at);
    run[start..end].chars().any(|c| c.is_ascii_alphabetic())
}

/// The word right before `before`'s end, past spaces: `NFPA` in `NFPA `.
fn previous_word(before: &str) -> &str {
    let trimmed = before.trim_end_matches([' ', '\u{a0}']);
    if trimmed.len() == before.len() {
        return "";
    }
    let start = trimmed
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_alphanumeric() || matches!(c, '.' | '-'))
        .last()
        .map_or(trimmed.len(), |(at, _)| at);
    &trimmed[start..]
}

/// `figure`'s digits grouped in threes with commas: `12000` → `12,000`.
fn grouped(figure: &str) -> String {
    let mut out = String::new();
    for (at, digit) in figure.chars().enumerate() {
        if at > 0 && (figure.len() - at).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// A few words of `run` around `span`, for a message.
fn context(run: &str, span: Range<usize>) -> String {
    let start = run[..span.start]
        .char_indices()
        .rev()
        .take(30)
        .last()
        .map_or(span.start, |(at, _)| at);
    let end = run[span.end..]
        .char_indices()
        .nth(30)
        .map_or(run.len(), |(at, _)| span.end + at);
    run[start..end].trim().to_owned()
}

/// The months' names, for a message's suggestion.
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Each ISO date (`2026-10-04`) on a page outside its tables, as (line, message).
pub(super) fn prose_dates(text: &str) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let tables = table_spans(text);
    let mut problems = Vec::new();
    for run in rendered_runs(text) {
        if run_in(&run, &tables) {
            continue;
        }
        for (at, date, words) in iso_dates(&run.text) {
            problems.push((
                line(run.source(at)),
                format!(
                    "`{date}` is an ISO date in prose: write `{words}` (writing.md, On screens: \
                     `October 4, 2026` in prose, `2026-10-04` in tables)"
                ),
            ));
        }
    }
    problems
}

/// The byte spans of a page's tables.
fn table_spans(text: &str) -> Vec<Range<usize>> {
    use pulldown_cmark::{Event, Parser, Tag};

    Parser::new_ext(text, super::options())
        .into_offset_iter()
        .filter_map(|(event, range)| matches!(event, Event::Start(Tag::Table(_))).then_some(range))
        .collect()
}

/// Whether `run` starts inside one of `spans`.
fn run_in(run: &Run, spans: &[Range<usize>]) -> bool {
    let start = run.source(0);
    spans.iter().any(|span| span.contains(&start))
}

/// Each ISO date in `run`, as (byte offset, the date, the date in words): four digits, a hyphen,
/// a month 01 to 12, a hyphen and a day 01 to 31, with no digit, letter or hyphen against
/// either end.
fn iso_dates(run: &str) -> Vec<(usize, String, String)> {
    let bytes = run.as_bytes();
    let mut found = Vec::new();
    let digits = |range: Range<usize>| range.clone().all(|at| bytes[at].is_ascii_digit());
    let mut at = 0;
    while at + 10 <= bytes.len() {
        // The shape is ASCII, so where it matches, both its ends are characters' boundaries.
        let shape = digits(at..at + 4)
            && bytes[at + 4] == b'-'
            && digits(at + 5..at + 7)
            && bytes[at + 7] == b'-'
            && digits(at + 8..at + 10);
        let free = |c: Option<char>| c.is_none_or(|c| !(c.is_alphanumeric() || c == '-'));
        if shape && free(run[..at].chars().next_back()) && free(run[at + 10..].chars().next()) {
            let month: usize = run[at + 5..at + 7].parse().unwrap_or(0);
            let day: u32 = run[at + 8..at + 10].parse().unwrap_or(0);
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                found.push((
                    at,
                    run[at..at + 10].to_owned(),
                    format!("{} {day}, {}", MONTHS[month - 1], &run[at..at + 4]),
                ));
                at += 10;
                continue;
            }
        }
        at += 1;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The figures [`ungrouped_figures`] fails in `text`.
    fn ungrouped_in(text: &str) -> Vec<String> {
        ungrouped_figures(text)
            .into_iter()
            .map(|(_, message)| message.split('`').nth(1).unwrap_or_default().to_owned())
            .collect()
    }

    #[test]
    fn an_ungrouped_figure_fails() {
        // #476's own examples, a figure before its decimal point, a negative one, one joined to a
        // word by a hyphen, a list's figures, one in a heading, a table cell and link text.
        let page = "# Heights over 2500 m\n\nA 1000 m apogee, 8800 N s. It reaches 1144.5 m \
                    (3755 ft), −1280 m, a 3005-row file, (494, 1442 and 5060).\n\n\
                    | case | apogee, m |\n|---|---:|\n| a | 1320.4 |\n\n[12000 ft](x.md)\n";
        assert_eq!(
            ungrouped_in(page),
            [
                "2500", "1000", "8800", "1144", "3755", "1280", "3005", "1442", "5060", "1320",
                "12000"
            ]
        );
        let message = &ungrouped_figures("It is 12000 ft.")[0].1;
        assert!(message.contains("write `12,000`"), "{message}");
        // Each is reported on its own line.
        let lines: Vec<usize> = ungrouped_figures("One.\n\nAt 1500 m\nand 2500 m.\n")
            .into_iter()
            .map(|(line, _)| line)
            .collect();
        assert_eq!(lines, [3, 4]);
    }

    #[test]
    fn a_grouped_figure_a_decimal_part_and_code_pass() {
        for text in [
            "A 1,000 m apogee, 12,217 ft and 5,241,600 values.",
            "A height RMS of 35.350931 m and 0.000123 s.",
            "Three figures, 999 m, pass.",
            "`12000` and `--elevation 1280` are code.",
            "```\n1144.5 m (3755 ft)\n```\n",
            "$8314.32$ is math.",
            "A [link](https://example.com/issues/12345) and an image's address.",
        ] {
            assert!(
                ungrouped_in(text).is_empty(),
                "{text}: {:?}",
                ungrouped_in(text)
            );
        }
    }

    #[test]
    fn an_identifier_passes() {
        // A figure inside a word or a code, joined by a hyphen, or after a name that numbers
        // its documents: one of each kind the module lists.
        for text in [
            "A 131G84 and a 7268bcd and C_D0123.",
            "ADR-0295, D-4014, #1000, 2026-10-04 and ISO 2768-1.",
            "arXiv:1109.4448v2, NGA.STND.0036, zenodo.32156, Sensors 13(11):15692.",
            "Paths 12/2048 and \\1234 and @5678 and +1234 and &1234 and =1234.",
        ] {
            assert!(
                ungrouped_in(text).is_empty(),
                "{text}: {:?}",
                ungrouped_in(text)
            );
        }
        for name in IDENTIFIERS {
            let text = format!("By {name} 1125, the rule.");
            assert!(ungrouped_in(&text).is_empty(), "{text}");
        }
        // A range of pages after an identifier is one; a range of quantities is not.
        assert!(ungrouped_in("Software 22, pp. 1509–1518, 2007.").is_empty());
        assert_eq!(ungrouped_in("From 1000–2000 ft."), ["1000", "2000"]);
        // A hyphen joins a figure to a word before it or makes a date's year, nothing else.
        assert_eq!(
            ungrouped_in("From 1000-2000 ft, 2026-10-04."),
            ["1000", "2000"]
        );
        assert_eq!(
            ungrouped_in("A 3005-row file, 12-3456 and 2026-13-04."),
            ["3005", "3456"]
        );
        assert!(
            ungrouped_in("NASA/TM-2008-215633, AS.1943-5525.0001331 and 050X(80)90013-3.")
                .is_empty()
        );
        assert_eq!(ungrouped_in("(80)90013 and (1)2345."), ["90013", "2345"]);
        // A name the list doesn't hold numbers nothing: a material's density fails.
        assert_eq!(ungrouped_in("Blue Tube 1250 kg/m³."), ["1250"]);
        // The name must be right before the figure, not a word earlier.
        assert_eq!(ungrouped_in("NFPA 1125 and 1127."), ["1127"]);
    }

    #[test]
    fn a_year_passes_and_a_quantity_in_its_range_fails() {
        for text in [
            "In 1997, Prometheus 2022 in wind and NDRT 2020's apogee.",
            "Barrowman 1967 thesis, (1958), 1600 and 2100.",
            "A date after December 31, 9999 and before March 1, 1900.",
        ] {
            assert!(
                ungrouped_in(text).is_empty(),
                "{text}: {:?}",
                ungrouped_in(text)
            );
        }
        // A unit or a percent sign after a four-digit figure makes it a quantity, and a figure
        // outside 1600 to 2100 is no year.
        assert_eq!(
            ungrouped_in("At 2000 m, 1500 ft, 1600 kg/m³, 2000 °C, 2010%, 1599 and 2101 years."),
            ["2000", "1500", "1600", "2000", "2010", "1599", "2101"]
        );
        // A day and a comma before a figure make it a year only after a month's name.
        assert_eq!(
            ungrouped_in("Rows 31, 9999 and 12, 2500."),
            ["9999", "2500"]
        );
    }

    /// The dates [`prose_dates`] fails in `text`, with their suggestions.
    fn dates_in(text: &str) -> Vec<String> {
        prose_dates(text)
            .into_iter()
            .map(|(line, message)| {
                let quoted: Vec<&str> = message.split('`').collect();
                format!("{line}: {} -> {}", quoted[1], quoted[3])
            })
            .collect()
    }

    #[test]
    fn an_iso_date_in_prose_fails() {
        // #476's own example, one in a heading, a list item, a note and link text.
        let page = "# Read on 2026-10-26\n\nRead on 2026-10-04.\n\n- From 2026-01-31,\n\n\
                    > Fetched 2025-12-01.\n\n[the 1899-12-30 epoch](x.md)\n";
        assert_eq!(
            dates_in(page),
            [
                "1: 2026-10-26 -> October 26, 2026",
                "3: 2026-10-04 -> October 4, 2026",
                "5: 2026-01-31 -> January 31, 2026",
                "7: 2025-12-01 -> December 1, 2025",
                "9: 1899-12-30 -> December 30, 1899",
            ]
        );
    }

    #[test]
    fn a_date_in_a_table_code_or_an_address_passes() {
        for text in [
            "| date | flights |\n|---|---:|\n| 2026-10-04 | 3 |\n",
            "The file `2026-10-04.md`.\n\n```\n2026-10-04\n```\n",
            "A [link](https://example.com/2026-10-04/) and <https://example.com/x>.",
            // Not a date: no month 13, no day 32, and glued to a word or a hyphen.
            "2026-13-01, 2026-10-32, x2026-10-04, 2026-10-04x, 1-2026-10-04 and 2026-10-04-1.",
            "October 4, 2026.",
        ] {
            assert!(dates_in(text).is_empty(), "{text}: {:?}", dates_in(text));
        }
    }
}
