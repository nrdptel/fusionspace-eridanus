//! Numbers on the site's pages as the product system's `data.md` sets them (M0.9e, #386):
//!
//! - **A real minus sign** (U+2212) on screen: `−7.28%`, not `-7.28%`. [`minus_signs`] reads a
//!   page's text as it renders (not code, which quotes files and output) and fails each
//!   hyphen-minus that stands for one: right before a digit, or before a decimal point and a
//!   digit, at the start of a run or after a space, an opening bracket or quote, an operator
//!   (`=`, `<`, `>`, `≤`, `≥`, `≈`, `~`, `±`, `/`, `×`), a comma, colon or semicolon, or an en
//!   dash. A hyphen after a letter or a digit joins
//!   words or names (`ADR-208`, `2026-10-04`, `1e-9`) and is left alone.
//! - **Numbers right-aligned in columns** ("Numbers right-aligned; text left-aligned", *Tables*).
//!   [`number_columns`] fails each table column whose body cells are all numbers, at least one
//!   of them, and whose delimiter doesn't right-align it (`---:`). A number cell is a figure,
//!   with a sign or a comparison before it if any, then only units, signs and brackets of
//!   figures: `12`, `−7.28%`, `1,400 m (4,593 ft)`, `5,486 ± 227`, `0.3–0.5`, `1.70 calibers`.
//!   A range may join two figures with "to". A cell in code is a value quoted from a file, a date
//!   (`2026-10-04`) or a version (`0.1.0`, or any cell of a column whose head says "version") is
//!   a label, and a cell with any other word is text, so a column holding one is a text column.
//!   An empty cell, a dash, "n/a", "none" or "Not used" doesn't count either way.

use super::units::rendered_runs;

/// The characters after which a hyphen-minus before a digit is a minus sign: an opening bracket
/// or quote, an operator, a list's comma, colon or semicolon, or a range's en dash
/// (`−5–-1 m`). A space or the start of a run count too.
const BEFORE_A_MINUS: [char; 21] = [
    '(', '[', '{', '=', '<', '>', '≤', '≥', '≈', '~', '±', '/', '×', ',', ':', ';', '"', '“', '\'',
    '‘', '–',
];

/// The words a number cell may hold after its figure: units, as `data.md` writes them, the
/// project's own (`cal`, `calibers`, `g` for acceleration), and the "to" of a range. Any other
/// word makes a cell text.
const UNITS: [&str; 48] = [
    "m", "km", "cm", "mm", "ft", "in", "mi", "s", "ms", "min", "h", "kg", "g", "lb", "lbf", "oz",
    "N", "kN", "Ns", "Pa", "kPa", "hPa", "MPa", "psi", "K", "C", "F", "rad", "deg", "Hz", "J", "W",
    "mph", "kt", "kn", "cal", "caliber", "calibers", "AGL", "MSL", "Mach", "e", "x", "σ", "kB",
    "MB", "GB", "to",
];

/// The cells that hold nothing: empty, a dash, "n/a", "none" or "Not used".
const BLANK: [&str; 9] = ["", "–", "-", "−", "n/a", "N/A", "none", "None", "Not used"];

/// Each hyphen-minus written for a minus sign on a page, as (line, message).
pub(super) fn minus_signs(text: &str) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let mut problems = Vec::new();
    for run in rendered_runs(text) {
        for at in hyphen_minuses(&run.text) {
            let number: String = run.text[at + 1..]
                .chars()
                .take_while(|c| c.is_ascii_digit() || matches!(c, '.' | ','))
                .collect();
            problems.push((
                line(run.source(at)),
                format!(
                    "`-{number}` has a hyphen-minus for its minus sign: write `−{number}` \
                     (U+2212; data.md, Numbers)"
                ),
            ));
        }
    }
    problems
}

/// The byte offsets in `run` of each hyphen-minus that stands for a minus sign.
fn hyphen_minuses(run: &str) -> Vec<usize> {
    let mut found = Vec::new();
    for (at, _) in run.match_indices('-') {
        let before = run[..at].chars().next_back();
        let mut after = run[at + 1..].chars();
        let figure = match after.next() {
            Some(c) if c.is_ascii_digit() => true,
            Some('.') => after.next().is_some_and(|c| c.is_ascii_digit()),
            _ => false,
        };
        let opens = before.is_none_or(|c| c.is_whitespace() || BEFORE_A_MINUS.contains(&c));
        if figure && opens {
            found.push(at);
        }
    }
    found
}

/// Each table column of numbers on a page that isn't right-aligned, as (line, message).
pub(super) fn number_columns(text: &str) -> Vec<(usize, String)> {
    let mut problems = Vec::new();
    for table in super::tables(text) {
        for (column, alignment) in table.alignments.iter().enumerate() {
            let head = table.header.get(column).map_or("", |cell| cell.text.trim());
            // A column of versions is a column of labels, `1.10` after `1.9`.
            if *alignment == pulldown_cmark::Alignment::Right
                || head
                    .split(|c: char| !c.is_alphanumeric())
                    .any(|word| word.eq_ignore_ascii_case("version"))
            {
                continue;
            }
            let cells: Vec<&super::Cell> = table
                .rows
                .iter()
                .filter_map(|(_, cells)| cells.get(column))
                .filter(|cell| !(cell.code.is_empty() && BLANK.contains(&cell.text.trim())))
                .collect();
            if !cells.is_empty() && cells.iter().all(|cell| is_number(cell)) {
                problems.push((
                    table.line,
                    format!(
                        "column {} (\"{head}\") holds only numbers but isn't right-aligned: end \
                         its delimiter with a colon, `---:` (data.md, Tables)",
                        column + 1
                    ),
                ));
            }
        }
    }
    problems
}

/// Whether a cell is a number: no code, a figure first (after a sign or comparison, if any),
/// then only units, figures and the signs between them.
fn is_number(cell: &super::Cell) -> bool {
    if !cell.code.is_empty() {
        return false;
    }
    let text = cell.text.trim();
    let start = text.trim_start_matches(['+', '−', '-', '±', '~', '≈', '<', '>', '≤', '≥', ' ']);
    if !start.starts_with(|c: char| c.is_ascii_digit()) {
        return false;
    }
    // A date or a version is a label: digits joined by hyphens, or a figure with two points.
    let figure: String = start
        .chars()
        .take_while(|c| c.is_ascii_digit() || matches!(c, '.' | ',' | '-'))
        .collect();
    if figure.contains('-') || figure.matches('.').count() > 1 {
        return false;
    }
    let mut word = String::new();
    for c in start.chars().chain([' ']) {
        if c.is_alphabetic() {
            word.push(c);
            continue;
        }
        if !word.is_empty() && !UNITS.contains(&word.as_str()) {
            return false;
        }
        word.clear();
        let allowed = c.is_ascii_digit()
            || c.is_whitespace()
            || ".,%°′″/·²³⁰¹⁴⁵⁶⁷⁸⁹⁻+−-±×–()≈~<>≤≥".contains(c);
        if !allowed {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(problems: &[(usize, String)]) -> Vec<usize> {
        problems.iter().map(|(line, _)| *line).collect()
    }

    #[test]
    fn a_hyphen_minus_before_a_figure_is_a_minus_sign() {
        let page = "From -7.28% to +10.30%.\n\nA drop (-0.5 m), x = -3, and -.25.\n";
        let problems = minus_signs(page);
        assert_eq!(lines(&problems), [1, 3, 3, 3]);
        assert!(problems[0].1.contains("`−7.28`"), "{problems:?}");
        assert!(minus_signs("From −7.28% to +10.30%.\n").is_empty());
    }

    #[test]
    fn hyphens_that_join_are_left_alone() {
        for fine in [
            "ADR-208 and M0.9e-1",
            "read on 2026-10-04",
            "a tolerance of 1e-9",
            "the 3-caliber rule, 10-20 m",
            "a dash - then text",
            "`-7.28`",
        ] {
            assert!(minus_signs(fine).is_empty(), "{fine}");
        }
        let block = "```text\n-7.28\n```\n";
        assert!(minus_signs(block).is_empty());
    }

    #[test]
    fn a_minus_in_a_table_cell_or_a_link_is_read() {
        let page = "| A | B |\n|---|---:|\n| x | -1.5 |\n\n[down -3 m](a.md)\n";
        assert_eq!(lines(&minus_signs(page)), [3, 5]);
        // After a list's comma, a colon, a quote or a range's en dash.
        let page = "At (−1.2,-0.4), a drop:-3 m, \"-5 m\" and −5–-1 m.\n";
        assert_eq!(minus_signs(page).len(), 4);
    }

    #[test]
    fn a_column_of_numbers_must_be_right_aligned() {
        let page = "| Case | Apogee | Error |\n|---|---|:---:|\n| a | 1,400 m (4,593 ft) | −7.28% |\n\
                    | b | 12 | n/a |\n";
        let problems = number_columns(page);
        assert_eq!(lines(&problems), [1, 1]);
        assert!(
            problems[0].1.contains("column 2 (\"Apogee\")"),
            "{problems:?}"
        );
        assert!(
            problems[1].1.contains("column 3 (\"Error\")"),
            "{problems:?}"
        );
        let right = "| Case | Apogee |\n|---|---:|\n| a | 1,400 m |\n";
        assert!(number_columns(right).is_empty());
    }

    #[test]
    fn number_cells_take_units_spreads_and_ranges() {
        for number in [
            "12",
            "−7.28%",
            "+8.73%",
            "5,486 ± 227",
            "0.3–0.5",
            "1.70 calibers",
            "45°",
            "~3 s",
            "≤ 15°",
            "644 ft/s",
            "2.5 × 10⁻³",
            "1e-9",
            "53 (64%)",
            "−29 to −34%",
        ] {
            let cell = super::super::Cell {
                text: number.to_owned(),
                code: Vec::new(),
            };
            assert!(is_number(&cell), "{number}");
        }
        for text in [
            "2026-10-04",
            "0.1.0",
            "53 flights",
            "M1.14a",
            "none yet",
            "#386",
            "Yes",
        ] {
            let cell = super::super::Cell {
                text: text.to_owned(),
                code: Vec::new(),
            };
            assert!(!is_number(&cell), "{text}");
        }
        let code = super::super::Cell {
            text: "0.3".to_owned(),
            code: vec!["0.3".to_owned()],
        };
        assert!(!is_number(&code));
    }

    #[test]
    fn text_and_blank_columns_are_left_alone() {
        let page =
            "| Case | Note | Count |\n|---|---|---|\n| 1 | fine | n/a |\n| 2 | 3 flights | – |\n";
        // The first column is numbers; the second text; the third only blanks.
        assert_eq!(number_columns(page).len(), 1);
        // "none" is blank, so the second column is numbers; a version column is labels.
        let page = "| Version | Error |\n|---|---|\n| 1.4 | none |\n| 1.10 | +7.0% |\n";
        let problems = number_columns(page);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].1.contains("column 2"), "{problems:?}");
        // "Conversion" holds the word's letters, not the word.
        let page = "| Conversion factor |\n|---|\n| 0.3048 |\n";
        assert_eq!(number_columns(page).len(), 1);
    }
}
