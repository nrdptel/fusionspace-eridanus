//! US units on the site's pages (#400). The product system's `data.md` (*Units for rocketry*)
//! gives every height, distance and speed in the units US flyers read as well; the project keeps
//! SI first with the US units in brackets after it, as the command line prints them
//! ([ADR-210](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0210-us-units-in-brackets-on-the-command-line.md)):
//! `1,400 m (4,593 ft)`, `16.2 m/s (53 ft/s)`, `5 m/s (11 mph)`.
//!
//! [`si_alone`] reads a page's text as it renders (prose, headings, list items, table cells,
//! link text and image descriptions; not code, which quotes files and output) and fails each
//! number in meters, kilometers, meters per second or kilometers per hour that no bracket
//! follows, or whose bracket's US figure isn't that number converted:
//!
//! - **The units:** meters and kilometers in feet, inches or miles; meters and kilometers per
//!   hour in feet per second or miles per hour (the wind's). The writer picks which.
//! - **The figure** is the SI number as written, converted with the exact factors (a foot is
//!   0.3048 m, a mile 1,609.344 m), to any precision: it may differ from the exact conversion
//!   by half its own last place, plus what rounding the SI number to its own last place can
//!   move it. So `1,400 m (4,593 ft)` passes and `(4,600 ft)` doesn't: rounding the SI number
//!   first is the writer's choice, made in the meters. A range (`5,100–5,500 m`) needs one in
//!   the bracket, both ends checked.
//! - **Arithmetic is exempt:** a number joined to an operator (`×`, `·`, `÷`, `/`, `*`, `+`, a
//!   spaced `−` or `-`, `=`) on either side is a term of a worked calculation, which states its
//!   terms in the unit it computes in.
//! - **So is e-notation:** `1e-9 m/s` is a tolerance a test or a solver holds, written as a
//!   program writes it, not a quantity a flyer reads.
//!
//! Which pages: the guides, every page of the site but [`DEFERRED`] (the model pages and the
//! records, M0.9c16) and [`EXEMPT`] (the file formats' pages, which give what a file stores).

/// Pages, or directories of pages ending in `/`, the check doesn't read yet: the model pages and
/// the records, which M0.9c16 brings in (ADR-219).
pub(super) const DEFERRED: [&str; 4] = [
    "physics/",
    "accuracy.md",
    "VALIDATION.md",
    "decisions-and-roadmap.md",
];

/// Pages the check never reads: a file format's page gives the numbers a file stores, in the
/// file's own units (ADR-219).
pub(super) const EXEMPT: [&str; 1] = ["format/"];

/// Whether `name` (a page's path under `docs/`) is on a list of pages and directories.
fn listed(list: &[&str], name: &str) -> bool {
    list.iter().any(|entry| {
        if entry.ends_with('/') {
            name.starts_with(entry)
        } else {
            name == *entry
        }
    })
}

/// Whether the check reads the page `name` (its path under `docs/`).
pub(super) fn covers(name: &str) -> bool {
    !listed(&DEFERRED, name) && !listed(&EXEMPT, name)
}

/// Whether the page `name` waits for M0.9c16: the site check counts its failures, and doesn't
/// fail on them.
pub(super) fn deferred(name: &str) -> bool {
    listed(&DEFERRED, name)
}

/// Meters in a foot, an inch and a statute mile, exactly (the international foot).
const FOOT_M: f64 = 0.3048;
const INCH_M: f64 = 0.0254;
const MILE_M: f64 = 1609.344;

/// An SI unit the check reads, longest first so `km/h` isn't read as `km`, and its US units with
/// how many of each one of it makes.
const SI_UNITS: [(&str, &[(&str, f64)]); 4] = [
    (
        "km/h",
        &[("mph", 1000.0 / MILE_M), ("ft/s", 1000.0 / 3600.0 / FOOT_M)],
    ),
    ("m/s", &[("ft/s", 1.0 / FOOT_M), ("mph", 3600.0 / MILE_M)]),
    ("km", &[("mi", 1000.0 / MILE_M), ("ft", 1000.0 / FOOT_M)]),
    (
        "m",
        &[
            ("ft", 1.0 / FOOT_M),
            ("in", 1.0 / INCH_M),
            ("mi", 1.0 / MILE_M),
        ],
    ),
];

/// US units a bracket may give, longest first so `ft/s` isn't read as `ft`.
const US_UNITS: [&str; 5] = ["ft/s", "mph", "ft", "in", "mi"];

/// Operators that make a number a term of a worked calculation.
const OPERATORS: [char; 8] = ['×', '·', '÷', '/', '*', '+', '=', '−'];

/// The spaces a number and its unit may be written with: plain, no-break and narrow no-break.
fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\u{a0}' | '\u{202f}')
}

/// A number as written: its value, the size of its last place (`1,400` is 1, `16.2` is 0.1),
/// and whether it is in e-notation (`1e-9`).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Figure {
    value: f64,
    place: f64,
    scientific: bool,
}

/// Reads a number at the start of `text`: an optional sign (`−`, `-`, `+`), digits grouped by
/// commas or not, decimals, and an exponent (`e-9`). Returns it and its length in bytes.
fn figure(text: &str) -> Option<(Figure, usize)> {
    let mut chars = text.char_indices().peekable();
    let mut negative = false;
    let mut start = 0;
    if let Some(&(_, c)) = chars.peek()
        && matches!(c, '−' | '-' | '+')
    {
        negative = c != '+';
        chars.next();
        start = c.len_utf8();
    }
    let mut digits = String::new();
    let mut decimals: Option<usize> = None;
    let mut end = start;
    while let Some(&(at, c)) = chars.peek() {
        let next_is_digit = text[at + c.len_utf8()..]
            .chars()
            .next()
            .is_some_and(|d| d.is_ascii_digit());
        if c.is_ascii_digit() {
            digits.push(c);
            if let Some(count) = decimals.as_mut() {
                *count += 1;
            }
        } else if c == ',' && decimals.is_none() && !digits.is_empty() && next_is_digit {
            // A thousands separator only: a comma and a space is a list.
        } else if c == '.' && decimals.is_none() && next_is_digit {
            digits.push('.');
            decimals = Some(0);
        } else {
            break;
        }
        end = at + c.len_utf8();
        chars.next();
    }
    if digits.is_empty() || digits.starts_with('.') {
        return None;
    }
    // An exponent: `e` or `E`, a sign or none, and digits.
    let mut exponent = 0i32;
    let mut scientific = false;
    if let Some(rest) = text[end..].strip_prefix(['e', 'E']) {
        let (minus, sign_len, rest) = match rest.chars().next() {
            Some(c @ ('−' | '-' | '+')) => (c != '+', c.len_utf8(), &rest[c.len_utf8()..]),
            _ => (false, 0, rest),
        };
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if let Ok(power) = digits.parse::<i32>() {
            exponent = if minus { -power } else { power };
            scientific = true;
            end += 1 + sign_len + digits.len();
        }
    }
    let value: f64 = digits.parse::<f64>().ok()? * 10f64.powi(exponent);
    let place = 10f64.powi(exponent - decimals.unwrap_or(0) as i32);
    Some((
        Figure {
            value: if negative { -value } else { value },
            place,
            scientific,
        },
        end,
    ))
}

/// Reads one figure or a range of two (`5,100–5,500`, an en dash between), from the start.
fn figures(text: &str) -> Option<(Vec<Figure>, usize)> {
    let (first, mut end) = figure(text)?;
    let mut list = vec![first];
    if let Some(rest) = text[end..].strip_prefix('–')
        && let Some((second, len)) = figure(rest)
    {
        list.push(second);
        end += '–'.len_utf8() + len;
    }
    Some((list, end))
}

/// The unit among `units` that `text` starts with, ending where a word would: not before a
/// letter, digit, `/`, `²`, `³` or `-` (`m/s²`, `mm`, `min`, `m-long`).
fn unit_at<'a>(text: &str, units: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    units.into_iter().find(|unit| {
        text.strip_prefix(unit).is_some_and(|rest| {
            rest.chars()
                .next()
                .is_none_or(|c| !(c.is_alphanumeric() || matches!(c, '/' | '²' | '³' | '-')))
        })
    })
}

/// Whether a US figure is the SI one converted by `factor`: within half the US figure's last
/// place, plus the SI figure's half place converted (with a little room for binary rounding).
fn converts(si: Figure, us: Figure, factor: f64) -> bool {
    let exact = si.value * factor;
    let allowed = 0.5 * us.place + 0.5 * si.place * factor;
    (us.value - exact).abs() <= allowed * (1.0 + 1e-9) + 1e-12
}

/// The bracket after a quantity, from its `(`: its length in bytes if it gives the quantity in a
/// US unit, converted right, else what is wrong with it.
fn bracket(after: &str, si: &[Figure], units: &[(&str, f64)]) -> Result<usize, String> {
    let inside = after
        .strip_prefix('(')
        .ok_or_else(|| "no bracket follows it".to_owned())?;
    let inside = inside
        .strip_prefix("about")
        .or_else(|| inside.strip_prefix('≈'))
        .map_or(inside, |rest| rest.trim_start_matches(is_space));
    let (us, len) =
        figures(inside).ok_or_else(|| "its bracket doesn't open with a figure".to_owned())?;
    let rest = inside[len..].trim_start_matches(is_space);
    let Some(unit) = unit_at(rest, US_UNITS) else {
        return Err("its bracket's figure has no US unit (ft, in, mi, ft/s, mph)".to_owned());
    };
    let Some(&(_, factor)) = units.iter().find(|(name, _)| *name == unit) else {
        return Err(format!(
            "its bracket gives {unit}, not a unit of the same quantity"
        ));
    };
    if us.len() != si.len() {
        return Err(
            "a range needs a range in its bracket, and a single figure a single one".to_owned(),
        );
    }
    if si
        .iter()
        .zip(&us)
        .all(|(si, us)| converts(*si, *us, factor))
    {
        after
            .find(')')
            .map(|end| end + 1)
            .ok_or_else(|| "its bracket isn't closed".to_owned())
    } else {
        let want: Vec<String> = si
            .iter()
            .map(|si| format!("{:.2}", si.value * factor))
            .collect();
        Err(format!(
            "its bracket's {unit} aren't it converted ({} {unit})",
            want.join("–")
        ))
    }
}

/// What to write in the bracket: the figure converted into the first US unit, to one place
/// fewer than the SI figure has and at least a tenth under 10, grouped by commas from 1,000 as
/// `writing.md` groups prose.
fn suggestion(si: &[Figure], units: &[(&str, f64)], space: &str) -> String {
    let (unit, factor) = units[0];
    let ends: Vec<String> = si
        .iter()
        .map(|figure| {
            let value = figure.value * factor;
            let places = (-figure.place.log10()).round() as i32 - 1;
            let places = if value.abs() < 10.0 {
                places.max(1)
            } else {
                places.max(0)
            } as usize;
            let text = format!("{:.*}", places, value.abs());
            let (whole, fraction) = text
                .split_once('.')
                .map_or((text.as_str(), ""), |(w, f)| (w, f));
            let grouped = if whole.len() > 3 {
                let mut out = String::new();
                for (i, c) in whole.chars().enumerate() {
                    if i > 0 && (whole.len() - i) % 3 == 0 {
                        out.push(',');
                    }
                    out.push(c);
                }
                out
            } else {
                whole.to_owned()
            };
            let sign = if value < 0.0 { "−" } else { "" };
            if fraction.is_empty() {
                format!("{sign}{grouped}")
            } else {
                format!("{sign}{grouped}.{fraction}")
            }
        })
        .collect();
    format!("({}{space}{unit})", ends.join("–"))
}

/// Whether the text ends, past spaces, with an operator that makes what follows a term: a `−` or
/// `-` only with a space after it, since one joined to the figure is its sign.
fn operator_before(before: &str) -> bool {
    let trimmed = before.trim_end_matches(is_space);
    let spaced = trimmed.len() < before.len();
    match trimmed.chars().next_back() {
        Some('-') => spaced,
        Some(c) => OPERATORS.contains(&c),
        None => false,
    }
}

/// Whether the text starts, past spaces, with an operator: a `−` or `-` only with a space after.
fn operator_after(after: &str) -> bool {
    let trimmed = after.trim_start_matches(is_space);
    let mut chars = trimmed.chars();
    match chars.next() {
        Some('-' | '−') => chars.next().is_some_and(is_space),
        Some(c) => OPERATORS.contains(&c),
        None => false,
    }
}

/// Each quantity in SI alone on a page, as (line, message): its text as it renders, read in runs
/// that end where a block, a code span, raw HTML or an image's description does.
pub(super) fn si_alone(text: &str) -> Vec<(usize, String)> {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    let line = super::line_index(text);
    let mut problems = Vec::new();
    // The run, and where in the source each piece of it starts: (offset in the run, in the page).
    let mut run = String::new();
    let mut pieces: Vec<(usize, usize)> = Vec::new();
    let mut flush = |run: &mut String, pieces: &mut Vec<(usize, usize)>| {
        for (at, message) in si_alone_in(run) {
            let source = pieces
                .iter()
                .rev()
                .find(|(start, _)| *start <= at)
                .map_or(0, |(start, source)| source + (at - start));
            problems.push((line(source), message));
        }
        run.clear();
        pieces.clear();
    };
    let mut in_code_block = false;
    for (event, range) in Parser::new_ext(text, super::options()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                flush(&mut run, &mut pieces);
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::Text(words) if !in_code_block => {
                pieces.push((run.len(), range.start));
                run.push_str(&words);
            }
            Event::SoftBreak | Event::HardBreak => run.push(' '),
            // Inline formatting and links continue the run: `**324 m** (1,063 ft)` is one.
            Event::Start(Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link { .. })
            | Event::End(
                TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link,
            ) => {}
            _ => flush(&mut run, &mut pieces),
        }
    }
    flush(&mut run, &mut pieces);
    problems
}

/// Each quantity in SI alone in a run of rendered text, as (byte offset in the run, message).
pub(super) fn si_alone_in(run: &str) -> Vec<(usize, String)> {
    let mut problems = Vec::new();
    scan(run, |at, outcome| {
        if let Err(message) = outcome {
            problems.push((at, message));
        }
    });
    problems
}

/// `text` without the brackets that give a quantity's US units, so the numbers a page quotes
/// from a source are the SI ones it can trace: the feet are those converted.
pub(super) fn without_conversions(text: &str) -> String {
    let mut spans = Vec::new();
    scan(text, |_, outcome| {
        if let Ok(Some(span)) = outcome {
            spans.push(span);
        }
    });
    let mut out = String::with_capacity(text.len());
    let mut from = 0;
    for span in spans {
        out.push_str(&text[from..span.start]);
        from = span.end;
    }
    out.push_str(&text[from..]);
    out
}

/// Walks the quantities in SI units in a run of text, calling `found` with each one's byte
/// offset and what follows it: `Ok(Some(span))` a bracket giving its US units (the span from
/// the space before it to its `)`), `Ok(None)` a term of a worked calculation, `Err` what is
/// wrong.
fn scan(run: &str, mut found: impl FnMut(usize, Result<Option<std::ops::Range<usize>>, String>)) {
    let mut at = 0;
    while at < run.len() {
        let rest = &run[at..];
        let c = rest.chars().next().unwrap_or(' ');
        let starts = c.is_ascii_digit() || matches!(c, '−' | '-' | '+');
        // A figure starts after something that isn't part of a word or of another figure.
        let previous = run[..at].chars().next_back();
        let free =
            previous.is_none_or(|p| !(p.is_alphanumeric() || matches!(p, '.' | ',' | '_' | '–')));
        if !(starts && free) {
            at += c.len_utf8();
            continue;
        }
        let Some((si, len)) = figures(rest) else {
            at += c.len_utf8();
            continue;
        };
        let written = &rest[..len];
        let after_figure = &rest[len..];
        let space_len: usize = after_figure
            .chars()
            .take_while(|c| is_space(*c))
            .map(char::len_utf8)
            .sum();
        let space = &after_figure[..space_len];
        let unit_text = &after_figure[space_len..];
        let si_unit = SI_UNITS
            .iter()
            .find(|(unit, _)| unit_at(unit_text, [*unit]).is_some());
        let Some(&(unit, us_units)) = si_unit.filter(|_| space.chars().count() <= 1) else {
            at += len.max(c.len_utf8());
            continue;
        };
        let after_unit = &unit_text[unit.len()..];
        let quantity_end = at + len + space_len + unit.len();
        if operator_before(&run[..at])
            || operator_after(after_unit)
            || si.iter().any(|f| f.scientific)
        {
            found(at, Ok(None));
            at = quantity_end;
            continue;
        }
        let gap: usize = after_unit
            .chars()
            .take_while(|c| is_space(*c))
            .map(char::len_utf8)
            .sum();
        match bracket(&after_unit[gap..], &si, us_units) {
            Ok(len) => found(at, Ok(Some(quantity_end..quantity_end + gap + len))),
            Err(why) => {
                let sep = if space.is_empty() { " " } else { space };
                found(
                    at,
                    Err(format!(
                        "`{written}{space}{unit}` is in SI alone: {why}; give the US units in \
                         brackets after it, such as `{written}{space}{unit} {}` (#400, ADR-210)",
                        suggestion(&si, us_units, sep)
                    )),
                );
            }
        }
        at = quantity_end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn messages(run: &str) -> Vec<String> {
        si_alone_in(run)
            .into_iter()
            .map(|(_, message)| message)
            .collect()
    }

    #[test]
    fn a_quantity_with_its_bracket_passes() {
        for run in [
            "the site, 1,400\u{a0}m (4,593\u{a0}ft) above sea level",
            "a 3\u{a0}m (9.8\u{a0}ft) rail",
            "a 5\u{a0}m/s (11\u{a0}mph) wind",
            "leaves the rail at 16.2 m/s (53 ft/s)",
            "the drogue's 0.6 m (24 in) canopy",
            "130 km (81 mi) away",
            "settles at −14 m/s (−46 ft/s)",
            "from 5,100–5,500 m (16,732–18,045 ft)",
            "324 m (1,063 ft, above the pad)",
            "324 m (about 1,063 ft)",
            "a gust of 36 km/h (22 mph)",
        ] {
            assert_eq!(messages(run), Vec::<String>::new(), "{run}");
        }
    }

    #[test]
    fn a_quantity_in_si_alone_fails() {
        for run in [
            "a 3 m rail",
            "apogee at 779 m.",
            "a 5 m/s wind",
            "about 130 km from the pad",
            "the 1,400\u{a0}m site",
            "a (3 m) rail",
            "−14 m/s under the drogue",
            "5,100–5,500 m",
        ] {
            assert_eq!(messages(run).len(), 1, "{run}");
        }
    }

    #[test]
    fn a_wrong_conversion_fails() {
        // 1,400 m is 4,593.18 ft: 4,600 is the meters rounded after converting.
        let found = messages("1,400 m (4,600 ft)");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("aren't it converted"), "{found:?}");
        // Feet for a speed, ft/s for a height, a range against one figure, a bare bracket.
        for run in [
            "16.2 m/s (53 ft)",
            "324 m (1,063 ft/s)",
            "5,100–5,500 m (16,732 ft)",
            "324 m (high)",
            "324 m (1,063)",
            "16.2 m/s (38 mph)",
        ] {
            assert_eq!(messages(run).len(), 1, "{run}");
        }
        // The edge of what rounding allows: 3 m is 9.84 ft, and 3 can be 2.5 to 3.5 m.
        assert!(messages("3 m (8.2 ft)").is_empty());
        assert_eq!(messages("3 m (8.1 ft)").len(), 1);
        assert!(messages("3.0 m (9.7 ft)").is_empty());
        assert_eq!(messages("3.0 m (9.6 ft)").len(), 1);
    }

    #[test]
    fn other_units_and_words_are_left_alone() {
        for run in [
            "a 54 mm motor",
            "10 min of flight",
            "9.81 m/s² of gravity",
            "a 3 m-long rail",
            "M1 m",
            "in meters",
            "11 mph",
            "a 2 m² canopy",
            "x2 m",
            "v1.2 m",
        ] {
            assert_eq!(messages(run), Vec::<String>::new(), "{run}");
        }
    }

    #[test]
    fn worked_arithmetic_is_exempt() {
        for run in [
            "7,956.8 m × ln(557/549) = 115.1 m",
            "5,151 m − 5,035 m = 116 m",
            "30 m + 14.4 m",
            "2 × 3 m",
        ] {
            assert_eq!(messages(run), Vec::<String>::new(), "{run}");
        }
        // A sign joined to its figure is not an operator.
        assert_eq!(messages("falls at −14 m/s").len(), 1);
    }

    #[test]
    fn e_notation_is_read_whole_and_exempt() {
        // Not `9 m/s`: the exponent is part of the figure.
        for run in [
            "the wind to 1e-9 m/s",
            "within 1.1e-6 m of",
            "within 1e-11 m/s.",
            "2E+3 m",
        ] {
            assert_eq!(messages(run), Vec::<String>::new(), "{run}");
        }
        let (parsed, len) = figure("1.5e-3 m").unwrap();
        assert_eq!(len, "1.5e-3".len());
        assert!((parsed.value - 1.5e-3).abs() < 1e-18 && (parsed.place - 1e-4).abs() < 1e-18);
        assert!(parsed.scientific);
        // A word after a figure is not an exponent.
        assert_eq!(figure("3 each").unwrap().1, 1);
        assert_eq!(messages("3e m").len(), 0);
    }

    #[test]
    fn the_suggestion_is_a_bracket_the_check_passes() {
        for run in [
            "a 3 m rail",
            "779 m",
            "16.2 m/s",
            "1,400\u{a0}m",
            "130 km",
            "−14 m/s",
        ] {
            let found = si_alone_in(run);
            assert_eq!(found.len(), 1, "{run}");
            let message = &found[0].1;
            let start = message
                .find("such as `")
                .map(|i| i + "such as `".len())
                .unwrap();
            let end = message[start..].find('`').map(|i| start + i).unwrap();
            let fixed = &message[start..end];
            assert_eq!(messages(fixed), Vec::<String>::new(), "{fixed}");
        }
    }

    #[test]
    fn conversions_drop_out_of_quoted_numbers() {
        assert_eq!(
            without_conversions("apogee 324 m (1,063 ft) at 9.12 s, 16.2 m/s (53 ft/s), 3 m rail"),
            "apogee 324 m at 9.12 s, 16.2 m/s, 3 m rail"
        );
        // A bracket that isn't the conversion stays: its numbers are quoted like any other.
        assert_eq!(without_conversions("324 m (1,100 ft)"), "324 m (1,100 ft)");
    }

    #[test]
    fn deferred_and_exempt_pages_are_skipped() {
        assert!(covers("getting-started.md"));
        assert!(covers("cli.md"));
        assert!(!covers("physics/aero.md"));
        assert!(!covers("accuracy.md"));
        assert!(!covers("format/ork.md"));
    }
}
