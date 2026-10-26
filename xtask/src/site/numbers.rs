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
//! - **A label and a unit over each column of numbers** ("Column heads are labels … with the
//!   unit", *Tables*; "A number without a unit is a bug", *Numbers*). [`number_heads`] reads the
//!   head of each right-aligned column as a label, then a unit. The label is a word of two or more
//!   letters outside code and math that isn't a unit (a code identifier, `apogee_agl_m`, or a
//!   unit alone, `(kg)`, is no label). The unit is one of:
//!   - **in the head, where a unit goes**: the first word in brackets, after a comma, a `·` or a
//!     semicolon, or after "in" is a unit (`time (s)`, `Worst time, s`, `CP, calibers aft of the
//!     tip`, `inertia (kg·m²)`, `Offset in bytes`), or the head gives a rate, "per" and a unit or
//!     what it counts (`slope, per rad`, `per 100 words`); a unit word elsewhere (`Error in
//!     apogee`, `Apogee, in flight`, `Time per step`) isn't one, and `C` and `F` need their degree
//!     sign;
//!   - **a pure number's kind**, the label's last word: a coefficient, a ratio, a fraction, a
//!     share, a factor, a number, a count, a code, or Mach (`drag coefficient`, `Reynolds
//!     number`, `Mach`), or `(no unit)` where a unit goes; a head that ends in a figure (`Mach
//!     0.9`) is a value, a table on its side;
//!   - **beside every value**: each cell's first figure is followed by its unit (`+1.2%`, `0.4 m`,
//!     `5,486 ± 227 m`), not by a bracket (`3.80 (+73.2%)` gives the bracket's unit, not the
//!     value's);
//!   - **a count**: every value a whole number and the head naming what is counted, a plural
//!     (`Flights`, `Records read`) that isn't a measured quantity's (`Masses`, `Burn times`).
//!
//!   The check reads words, not meaning: a unit word it can't tell from another, or a plural of
//!   a quantity it doesn't list, can still pass. Reviews hold the rest, as `docs/writing.md`
//!   says of every check.

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
const UNITS: [&str; 49] = [
    "m", "km", "cm", "mm", "ft", "in", "mi", "s", "ms", "min", "h", "kg", "g", "lb", "lbf", "oz",
    "N", "kN", "Ns", "Pa", "kPa", "hPa", "MPa", "GPa", "psi", "K", "C", "F", "rad", "deg", "Hz",
    "J", "W", "mph", "kt", "kn", "cal", "caliber", "calibers", "AGL", "MSL", "Mach", "e", "x", "σ",
    "kB", "MB", "GB", "to",
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

/// The words of [`UNITS`] that are not units on their own: a range's "to", e-notation's `e`, a
/// factor's `x`, the heights' references, which follow a unit (`m AGL`), and the temperature
/// letters, which need their degree sign (`°C`; `Case (C)` gives none). Mach is a kind of
/// number ([`KINDS`]).
const NOT_UNITS: [&str; 8] = ["to", "e", "x", "AGL", "MSL", "Mach", "C", "F"];

/// The units [`number_heads`] reads besides [`UNITS`], in any case: the spelled-out names, the
/// lengths the project measures by (calibers and diameters) and milli-g.
const SPELLED_UNITS: [&str; 26] = [
    "meter",
    "meters",
    "second",
    "seconds",
    "foot",
    "feet",
    "inch",
    "inches",
    "kilogram",
    "kilograms",
    "gram",
    "grams",
    "newton",
    "newtons",
    "degree",
    "degrees",
    "radian",
    "radians",
    "percent",
    "byte",
    "bytes",
    "pound",
    "pounds",
    "diameters",
    "mg",
    "dB",
];

/// The kinds of pure number a head's last word can name, so it needs no unit (`drag
/// coefficient`, `Reynolds number`, `Exit code`, `Mach`). A head's unit position can give the
/// first five (`implied $C_D$ (coefficient)`); "number", "count" and "code" say nothing there,
/// and a pure number of no kind says `(no unit)`.
const KINDS: [&str; 9] = [
    "coefficient",
    "ratio",
    "fraction",
    "share",
    "factor",
    "number",
    "count",
    "code",
    "mach",
];

/// The words that end in "s" without being plurals, which a count's label can't rest on.
const NOT_PLURALS: [&str; 14] = [
    "this", "was", "has", "does", "its", "his", "thus", "plus", "gas", "bias", "axis", "radius",
    "status", "lens",
];

/// The marks a head or a value can give for its unit: a percentage and the angles.
const UNIT_MARKS: [char; 4] = ['%', '°', '′', '″'];

/// Each head of a right-aligned column on a page without a label or a unit, as (line, message).
pub(super) fn number_heads(text: &str) -> Vec<(usize, String)> {
    let mut problems = Vec::new();
    for table in super::tables(text) {
        for (column, alignment) in table.alignments.iter().enumerate() {
            if *alignment != pulldown_cmark::Alignment::Right {
                continue;
            }
            let head = table.header.get(column);
            let shown = head.map_or("", |cell| cell.source.trim());
            let head = Head::read(head.map_or("", |cell| cell.prose.as_str()));
            if !head.labeled() {
                problems.push((
                    table.line,
                    format!(
                        "column {} (\"{shown}\") has no label: name the quantity in words, then \
                         its unit, `apogee (m)`, not a code identifier, a figure or a unit alone \
                         (data.md, Tables: column heads are labels with the unit)",
                        column + 1
                    ),
                ));
                continue;
            }
            let cells: Vec<&super::Cell> = table
                .rows
                .iter()
                .filter_map(|(_, cells)| cells.get(column))
                .filter(|cell| !(cell.code.is_empty() && BLANK.contains(&cell.text.trim())))
                .collect();
            let unit = head.gives_a_unit()
                || (!cells.is_empty() && cells.iter().all(|cell| value_has_a_unit(cell)))
                || (!cells.is_empty()
                    && cells.iter().all(|cell| is_count(cell))
                    && head.names_a_count());
            if !unit {
                problems.push((
                    table.line,
                    format!(
                        "column {} (\"{shown}\") gives no unit: put it in the head, `time (s)` or \
                         `slope, per rad`, or beside each value, `0.4 m`; a pure number names its \
                         kind, `drag coefficient`; a count names what it counts, `Flights` \
                         (data.md, Numbers: a number without a unit is a bug)",
                        column + 1
                    ),
                ));
            }
        }
    }
    problems
}

/// A column head's text outside code and math, read as a label and the places a unit goes.
struct Head {
    /// The head outside its brackets, up to its first comma, `·` or semicolon.
    label: String,
    /// What opens each place a unit goes: each bracket's text, and what follows each comma,
    /// `·` or semicolon, or each "in" or "per".
    places: Vec<String>,
    /// What follows each "per": a rate's unit, or what it counts (`per 100 words`).
    rates: Vec<String>,
}

impl Head {
    fn read(prose: &str) -> Self {
        // The brackets' text, and the head without it.
        let mut outside = String::new();
        let mut places = Vec::new();
        let mut depth = 0usize;
        for c in prose.chars() {
            match c {
                '(' | '[' => {
                    if depth == 0 {
                        places.push(String::new());
                    }
                    depth += 1;
                }
                ')' | ']' => depth = depth.saturating_sub(1),
                _ if depth > 0 => {
                    if let Some(place) = places.last_mut() {
                        place.push(c);
                    }
                }
                _ => outside.push(c),
            }
        }
        // A comma between two digits groups a figure (`10,000 flights`); any other parts places.
        let chars: Vec<char> = outside.chars().collect();
        let mut parts = vec![String::new()];
        for (at, &c) in chars.iter().enumerate() {
            let grouping = c == ','
                && at > 0
                && chars[at - 1].is_ascii_digit()
                && chars.get(at + 1).is_some_and(char::is_ascii_digit);
            if matches!(c, ',' | '·' | ';') && !grouping {
                parts.push(String::new());
            } else if let Some(part) = parts.last_mut() {
                part.push(c);
            }
        }
        let mut parts = parts.into_iter();
        let label = parts.next().unwrap_or_default();
        places.extend(parts);
        let words: Vec<&str> = outside.split_whitespace().collect();
        let mut rates = Vec::new();
        for (at, word) in words.iter().enumerate() {
            let rest = words[at + 1..].join(" ");
            match word.to_lowercase().as_str() {
                "in" => places.push(rest),
                "per" => rates.push(rest),
                _ => {}
            }
        }
        // A place that opens with "per" is a rate's (`slope, per rad`).
        for place in &mut places {
            if let Some(rest) = place.trim_start().strip_prefix("per ") {
                rates.push(rest.to_owned());
                place.clear();
            }
        }
        Self {
            label,
            places,
            rates,
        }
    }

    /// Whether the label has a word of two or more letters that isn't a unit.
    fn labeled(&self) -> bool {
        self.label
            .split(|c: char| !c.is_alphabetic())
            .any(|word| word.chars().count() >= 2 && !is_unit(word))
    }

    /// Whether the head gives the column's unit: a place that opens with one, a rate whose
    /// "per" a unit or a counted plural follows, or a label whose last word names a kind of pure
    /// number.
    fn gives_a_unit(&self) -> bool {
        let last = self.label.split_whitespace().next_back().unwrap_or("");
        let kind = last
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        self.places.iter().any(|place| opens_with_unit(place))
            || self.rates.iter().any(|rate| {
                let first = after_figures(rate).next().unwrap_or("");
                opens_with_unit(rate) || is_plural(first.trim_matches(|c: char| !c.is_alphabetic()))
            })
            || KINDS.contains(&kind.as_str())
    }

    /// Whether the head names what a count counts: a plural that isn't a measured quantity's,
    /// or "count" or "number", in its label or where a unit goes (`Worst value, steps`), with
    /// no measured quantity as the label's last word (`Motors mass`).
    fn names_a_count(&self) -> bool {
        let last = self.label.split_whitespace().next_back().unwrap_or("");
        if is_quantity(&last.to_lowercase()) {
            return false;
        }
        std::iter::once(&self.label)
            .chain(&self.places)
            .flat_map(|part| part.split(|c: char| !(c.is_alphabetic() || c == '\'' || c == '’')))
            .any(|word| {
                let lower = word.to_lowercase();
                is_plural(word) || lower == "count" || lower == "number"
            })
    }
}

/// The words of `text` after any leading figures (`10⁶ psi`, `100 words`).
fn after_figures(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace().skip_while(|word| {
        word.chars()
            .all(|c| c.is_ascii_digit() || "⁰¹²³⁴⁵⁶⁷⁸⁹⁻×,.".contains(c))
    })
}

/// Whether a place opens with a unit: after any figures, a [`UNIT_MARKS`] mark, a unit (the
/// inch's "in" only alone: `Apogee, in flight` and `in-flight` give none), one of the first
/// five [`KINDS`] (`(coefficient)`), or `no unit`.
fn opens_with_unit(place: &str) -> bool {
    let mut words = after_figures(place);
    let first = words.next().unwrap_or("");
    let second = words.next();
    if (first == "in" && second.is_some()) || first.starts_with("in-") {
        return false;
    }
    let none = first.eq_ignore_ascii_case("no") && second.is_some_and(|w| w.starts_with("unit"));
    none || first.starts_with(UNIT_MARKS)
        || first
            .split(|c: char| !(c.is_alphabetic() || c == 'σ'))
            .find(|part| !part.is_empty())
            .is_some_and(|part| is_unit(part) || KINDS[..5].contains(&part.to_lowercase().as_str()))
}

/// The measured quantities, whose plural is no count (`Masses`, `Burn times`).
const QUANTITIES: [&str; 40] = [
    "mass",
    "time",
    "height",
    "speed",
    "length",
    "apogee",
    "altitude",
    "gain",
    "distance",
    "drift",
    "duration",
    "weight",
    "force",
    "pressure",
    "temperature",
    "angle",
    "velocity",
    "acceleration",
    "thrust",
    "impulse",
    "diameter",
    "radius",
    "area",
    "volume",
    "density",
    "rate",
    "margin",
    "slope",
    "drag",
    "delay",
    "span",
    "chord",
    "thickness",
    "frequency",
    "energy",
    "power",
    "load",
    "moment",
    "inertia",
    "spin",
];

/// Whether `word` (lower case) is a measured quantity, singular or plural.
fn is_quantity(word: &str) -> bool {
    let singulars = [
        Some(word.to_owned()),
        word.strip_suffix("ies").map(|stem| format!("{stem}y")),
        word.strip_suffix("es").map(str::to_owned),
        word.strip_suffix('s').map(str::to_owned),
    ];
    singulars
        .into_iter()
        .flatten()
        .any(|singular| QUANTITIES.contains(&singular.as_str()))
}

/// Whether `word` is a plural a count can name: three letters or more ending in "s", not
/// "ss", "us" or a possessive, not a word that only ends so ([`NOT_PLURALS`]), and not a
/// measured quantity's ([`is_quantity`]).
fn is_plural(word: &str) -> bool {
    let lower = word.to_lowercase();
    lower.chars().count() >= 3
        && lower.ends_with('s')
        && !lower.ends_with("ss")
        && !lower.ends_with("us")
        && !lower.contains(['\'', '’'])
        && !NOT_PLURALS.contains(&lower.as_str())
        && !is_quantity(&lower)
}

/// Whether `word` is a unit: one of [`UNITS`] but [`NOT_UNITS`], or of [`SPELLED_UNITS`].
fn is_unit(word: &str) -> bool {
    (UNITS.contains(&word) && !NOT_UNITS.contains(&word))
        || SPELLED_UNITS.contains(&word.to_lowercase().as_str())
}

/// Whether a value carries its unit: its first figure (with a sign, a spread or a range of
/// figures after it, if any) followed by a [`UNIT_MARKS`] mark or a unit.
fn value_has_a_unit(cell: &super::Cell) -> bool {
    let is_figure = |token: &str| {
        let token = token.trim_start_matches(['+', '−', '-', '±', '~', '≈', '<', '>', '≤', '≥']);
        token.starts_with(|c: char| c.is_ascii_digit())
            && token.chars().all(|c| {
                c.is_ascii_digit() || ".,–-−eE+×⁰¹²³⁴⁵⁶⁷⁸⁹⁻".contains(c) || UNIT_MARKS.contains(&c)
            })
    };
    let mut tokens = cell.prose.split_whitespace();
    let Some(first) = tokens.next().filter(|token| is_figure(token)) else {
        return false;
    };
    if first.ends_with(UNIT_MARKS) {
        return true;
    }
    for token in tokens {
        if matches!(token, "±" | "to" | "–") || is_figure(token) && !token.ends_with(UNIT_MARKS)
        {
            continue;
        }
        // A bracket gives another quantity's unit (`3.80 (+73.2%)`), not this value's.
        if token.starts_with(['(', '[']) {
            return false;
        }
        let word = token.trim_end_matches([',', ';', '.', ')']);
        return word.starts_with(UNIT_MARKS)
            || word.ends_with(UNIT_MARKS)
            || word
                .split(|c: char| !(c.is_alphabetic() || c == 'σ'))
                .find(|part| !part.is_empty())
                .is_some_and(is_unit);
    }
    false
}

/// Whether a cell is a count: a whole number, grouped or not, with no code.
fn is_count(cell: &super::Cell) -> bool {
    let text = cell.text.trim();
    cell.code.is_empty()
        && text.starts_with(|c: char| c.is_ascii_digit())
        && text.chars().all(|c| c.is_ascii_digit() || c == ',')
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
                ..Default::default()
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
                ..Default::default()
            };
            assert!(!is_number(&cell), "{text}");
        }
        let code = super::super::Cell {
            text: "0.3".to_owned(),
            code: vec!["0.3".to_owned()],
            ..Default::default()
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

    #[test]
    fn a_column_of_numbers_needs_a_label_and_a_unit() {
        let heads = |page: &str| -> Vec<String> {
            number_heads(page)
                .into_iter()
                .map(|(line, message)| format!("{line}: {message}"))
                .collect()
        };
        let column = |head: &str, values: &[&str]| {
            let rows: String = values
                .iter()
                .map(|value| format!("| {value} |\n"))
                .collect();
            heads(&format!("| {head} |\n|---:|\n{rows}"))
        };
        // No label: a code identifier, nothing, a figure, a unit alone, a unit beside code, or
        // only math. The message shows the head as written.
        for head in [
            "`apogee_agl_m`",
            "",
            "1.5",
            "(m/s)",
            "(kg)",
            "kPa",
            "`cd` (m)",
            "$C_D$",
        ] {
            let problems = column(head, &["12.5 m"]);
            assert_eq!(problems.len(), 1, "{head}: {problems:?}");
            assert!(problems[0].contains("has no label"), "{problems:?}");
            assert!(
                problems[0].contains(&format!("(\"{head}\")")),
                "{problems:?}"
            );
            assert!(problems[0].starts_with("1: "), "{problems:?}");
        }
        // A label and no unit, where a unit word stands in the label, not where a unit goes;
        // a figure ending the head (a table on its side); code that matches a prose word's
        // letters; a count whose label doesn't say what it counts; a bracket after a value.
        for (head, values) in [
            ("HPR Sim", &["2.552"][..]),
            ("Error in apogee", &["2.5"]),
            ("Apogee AGL", &["2.5"]),
            ("Burn times", &["2.5"]),
            ("Second stage apogee", &["2.5"]),
            ("Body diameter", &["2.5"]),
            ("Case C", &["2.5"]),
            ("Height to apogee", &["2.5"]),
            ("Mach 0.9", &["0.45"]),
            ("mass `m`", &["2.5"]),
            ("Steps `s`", &["2.5"]),
            ("Apogee", &["1234"]),
            ("HPR Sim", &["1,635"]),
            ("HPR Sim's", &["2"]),
            ("before", &["3.80 (+73.2%)"]),
            ("items", &["2.5"]),
            ("error", &["+1.2%", "0.4"]),
            ("Flights", &["53", "1.5"]),
            ("HPR Sim", &["none"]),
            ("Differing", &["3"]),
            ("value, as a share", &["0.5"]),
            ("Burn times", &["12", "15"]),
            ("Masses", &["12"]),
            ("Masses recorded", &["12"]),
            ("Motors mass", &["3"]),
            ("Altitude gains", &["12"]),
            ("Apogee, previous run", &["1200"]),
            ("Apogee, in flight", &["2.5"]),
            ("Apogee (in flight)", &["2.5"]),
            ("Apogee, in-flight", &["2.5"]),
            ("Time per step", &["2.5"]),
            ("Apogee error per flight", &["2.5"]),
            ("Case (C)", &["2.5"]),
        ] {
            let problems = column(head, values);
            assert_eq!(problems.len(), 1, "{head}: {problems:?}");
            assert!(
                problems[0].contains("gives no unit"),
                "{head}: {problems:?}"
            );
        }
        // A unit where a unit goes: brackets, after a comma, a `·` or "in", a rate; a kind of
        // pure number as the last word; a unit beside every value, after its spread or range;
        // a count that names what it counts. A label beside an identifier the page's checks
        // read.
        for (head, values) in [
            ("time (s)", &["1.5"][..]),
            ("TIME · s", &["1.5"]),
            ("Worst time, s", &["1.5"]),
            ("speed (m/s)", &["1.5"]),
            ("inertia (kg·m²)", &["1.5"]),
            ("slope, per rad", &["1.5"]),
            ("slope per radian", &["1.5"]),
            ("Limit per 100 words", &["0.5"]),
            ("CP, calibers aft of the tip", &["1.5"]),
            ("Offset in bytes", &["16"]),
            ("modulus $G$ (GPa)", &["26.2"]),
            ("implied $C_D$ (coefficient)", &["0.45"]),
            ("Step (mg, g)", &["5"]),
            ("drag coefficient", &["0.45"]),
            ("pressure ratio", &["0.45"]),
            ("Reynolds number", &["2.5e6"]),
            ("Mach", &["0.85"]),
            (
                "error",
                &[
                    "+1.2%",
                    "−0.4 m",
                    "none",
                    "5,486 ± 227 m",
                    "1,400 m (4,593 ft)",
                ],
            ),
            ("spread", &["1.1 σ", "0.3 to 0.5 m", "45°"]),
            ("Flights", &["53", "1,052"]),
            ("Records read", &["7"]),
            ("tube count", &["3"]),
            ("Worst value, steps", &["3"]),
            ("10,000 flights, 10 threads", &["1.5 s"]),
            ("Martin's mark, 10⁶ psi", &["0.45"]),
            ("Code to code · rows", &["3"]),
            ("HPR Sim median (no unit)", &["0.0012"]),
            ("Exit code", &["2"]),
            ("Length (in)", &["2.5"]),
            ("Errors per 100 words", &["0.5"]),
            ("Values differing", &["3"]),
            ("air temperature (°C)", &["15"]),
            ("apogee (`apogee_agl_m`)", &["+0.5%"]),
        ] {
            let problems = column(head, values);
            assert!(problems.is_empty(), "{head}: {problems:?}");
        }
        // A column left- or center-aligned is text, and isn't read.
        assert!(heads("| `name` |\n|---|\n| 2.5 |\n").is_empty());
        assert!(heads("| `name` |\n|:---:|\n| 2.5 |\n").is_empty());
    }
}
