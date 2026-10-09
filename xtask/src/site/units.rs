//! US units on the site's pages (#400). The product system's `data.md` (*Units for rocketry*)
//! gives every height, distance and speed in the units US flyers read as well; the project keeps
//! SI first with the US units in brackets after it, as the command line prints them
//! ([ADR-210](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0210-us-units-in-brackets-on-the-command-line.md)):
//! `1,400 m (4,593 ft)`, `16.2 m/s (53 ft/s)`, `5 m/s (11 mph)`.
//!
//! [`si_alone`] reads a page's text as it renders (prose, headings, list items, table cells,
//! link text and image descriptions; not code, which quotes files and output) and fails each
//! number in meters, centimeters, kilometers, meters per second or kilometers per hour that no
//! bracket
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
//! - **Millimeters** take inches, but **a motor's or a mount's size** stays in millimeters alone,
//!   since motors and their mounts are named by it (`data.md`: "motor mounts by the motor's (29,
//!   38, 54, 75, 98 mm)"): one of [`MOTOR_DIAMETERS_MM`] followed, within three words, by a word
//!   of [`MOTOR_WORDS`] or a motor's designation (`a 54 mm motor`, `29 mm and 38 mm mounts`,
//!   `a 38 mm Pro38 case`, `a 38 mm Cesaroni I175`). A mount's size is the motor size it takes;
//!   a measured bore, like any other size in millimeters, a body tube's or a fin's, gives its
//!   inches: `a 28.956 mm (1.140 in) bore`, `2 mm (0.079 in)`.
//! - **A number grouped by spaces** (`63 781 370 m`) fails: the check would read only its last
//!   group, so it asks for commas.
//!
//! Which pages: every page of the site (the guides since M0.9c13; the model pages, the file
//! formats' pages, the accuracy page, the validation plan and the records since M0.9c16).

/// The motor diameters, in millimeters, that name a motor or a motor mount: the NAR's and
/// Tripoli's standard diameters (13, 18, 24, 29, 38, 54, 75 and 98 mm) and the 150 and 152 mm of
/// the largest commercial cases.
const MOTOR_DIAMETERS_MM: [f64; 10] =
    [13.0, 18.0, 24.0, 29.0, 38.0, 54.0, 75.0, 98.0, 150.0, 152.0];

/// Words that make a size in millimeters before them a motor's or a mount's; `case` and `casing`
/// do too, where [`motor_size`] says.
const MOTOR_WORDS: [&str; 7] = [
    "motor", "motors", "mount", "mounts", "reload", "reloads", "mmt",
];

/// Whether a size in millimeters names a motor or its mount: a standard motor diameter followed,
/// within three words, by a word of [`MOTOR_WORDS`] or a motor's designation after its maker
/// (`Cesaroni I175`, from [`MAKERS`]).
/// `after` is the text after `mm`. A list of sizes reaches its noun (`29 mm, 38 mm and 54 mm
/// motors`): figures, `mm`, and an `and`, `or`, `to`, article or comma before a figure are passed over,
/// but only before the first word; a figure, `and` or `or` after a word ends the search, so `a
/// 54 mm tube and a 38 mm motor` doesn't lend the tube the motor's noun. `case` and `casing`, also plain words
/// (`in either case`), count only straight after the size or after a capitalized name (`a 38
/// mm Pro38 case`). Quotation marks are passed over; other punctuation, a full stop included,
/// ends the search.
fn motor_size(si: &[Figure], after: &str) -> bool {
    if !si
        .iter()
        .all(|figure| MOTOR_DIAMETERS_MM.contains(&figure.value))
    {
        return false;
    }
    // Whether a figure comes next, past an article: `29 mm and a 38 mm motor`.
    let figure_next = |text: &str| {
        let text = text.trim_start_matches(is_space);
        let text = ["a", "an", "the"]
            .iter()
            .find_map(|article| {
                text.strip_prefix(article)
                    .filter(|rest| rest.starts_with(is_space))
            })
            .unwrap_or(text);
        text.trim_start_matches(is_space)
            .starts_with(|d: char| d.is_ascii_digit())
    };
    let mut words = 0;
    let mut named = false;
    let mut maker = false;
    let mut rest = after;
    loop {
        rest = rest.trim_start_matches(is_space);
        let Some(c) = rest.chars().next() else {
            return false;
        };
        if c == ',' {
            if words > 0 || !figure_next(&rest[1..]) {
                return false;
            }
            rest = &rest[1..];
            continue;
        }
        // A quotation mark is no break: `a "29 mm" case` names the case by its size.
        if matches!(c, '"' | '”' | '“' | '\'' | '’') {
            rest = &rest[c.len_utf8()..];
            continue;
        }
        if !(c.is_alphanumeric() || matches!(c, '-' | '–')) {
            return false;
        }
        let len: usize = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || matches!(c, '-' | '–' | '.' | '\'' | '’'))
            .map(char::len_utf8)
            .sum();
        // A full stop after a word ends the sentence, and the search after that word.
        let stops = rest[..len].ends_with('.');
        let raw = rest[..len]
            .trim_end_matches('.')
            .trim_end_matches("'s")
            .trim_end_matches("’s")
            .trim_matches(|c| matches!(c, '-' | '–'));
        rest = if stops { "." } else { &rest[len..] };
        let token = raw.to_lowercase();
        let in_list =
            token.is_empty() || token.starts_with(|d: char| d.is_ascii_digit()) || token == "mm";
        let joins = matches!(token.as_str(), "and" | "or" | "to" | "a" | "an" | "the");
        if in_list || (joins && figure_next(rest)) {
            if words > 0 {
                return false;
            }
            continue;
        }
        // After a word, `and` or `or` starts another noun: `the 54 mm airframe or motor tube`.
        if words > 0 && matches!(token.as_str(), "and" | "or") {
            return false;
        }
        // A designation names a motor only after its maker: `G10` alone is a fiberglass tube.
        let first = raw.split(['-', '–']).next().unwrap_or_default();
        if maker && designation(first) {
            return true;
        }
        for part in token.split(['-', '–']) {
            if matches!(part, "case" | "cases" | "casing" | "casings") {
                if words == 0 || named {
                    return true;
                }
            } else if MOTOR_WORDS.contains(&part) {
                return true;
            }
        }
        named = raw.starts_with(|c: char| c.is_uppercase());
        maker = MAKERS.contains(&token.as_str());
        words += 1;
        if words == 3 {
            return false;
        }
    }
}

/// Motor makers whose name, before a designation, makes it a motor's (`a 38 mm Cesaroni I175`).
const MAKERS: [&str; 10] = [
    "aerotech", "cesaroni", "cti", "loki", "estes", "quest", "apogee", "amw", "klima", "kosdon",
];

/// Whether a word is a motor's designation: an impulse class from A to O, at least two digits of
/// average thrust, and the maker's letters for propellant and delay (`I175`, `H170M`, `J350W`).
fn designation(word: &str) -> bool {
    let mut chars = word.chars();
    let class = chars.next().is_some_and(|c| ('A'..='O').contains(&c));
    let rest = chars.as_str();
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    class && digits >= 2 && rest[digits..].chars().all(|c| c.is_ascii_uppercase())
}

/// Meters in a foot, an inch and a statute mile, exactly (the international foot).
const FOOT_M: f64 = 0.3048;
const INCH_M: f64 = 0.0254;
const MILE_M: f64 = 1609.344;

/// An SI unit the check reads, longest first so `km/h` isn't read as `km`, and its US units with
/// how many of each one of it makes.
const SI_UNITS: [(&str, &[(&str, f64)]); 6] = [
    (
        "km/h",
        &[("mph", 1000.0 / MILE_M), ("ft/s", 1000.0 / 3600.0 / FOOT_M)],
    ),
    ("m/s", &[("ft/s", 1.0 / FOOT_M), ("mph", 3600.0 / MILE_M)]),
    ("km", &[("mi", 1000.0 / MILE_M), ("ft", 1000.0 / FOOT_M)]),
    ("cm", &[("in", 0.01 / INCH_M), ("ft", 0.01 / FOOT_M)]),
    ("mm", &[("in", 0.001 / INCH_M)]),
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
        } else if c == ','
            && decimals.is_none()
            && !digits.is_empty()
            && group_follows(&text[at + 1..])
        {
            // A thousands separator only, three digits after it: a comma and a space is a list,
            // and `1,40` is two numbers, not 140.
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
        // Past ±400 a figure is zero or infinite in f64 anyway; bounding it keeps the place's
        // arithmetic below from overflowing.
        if let Ok(power) = digits.parse::<i32>().map(|power| power.min(400)) {
            exponent = if minus { -power } else { power };
            scientific = true;
            end += 1 + sign_len + digits.len();
        }
    }
    let value: f64 = digits.parse::<f64>().ok()? * 10f64.powi(exponent);
    let decimals = i32::try_from(decimals.unwrap_or(0)).unwrap_or(i32::MAX);
    let place = 10f64.powi(exponent.saturating_sub(decimals));
    Some((
        Figure {
            value: if negative { -value } else { value },
            place,
            scientific,
        },
        end,
    ))
}

/// Whether `text` starts with a group of exactly three digits.
fn group_follows(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() >= 3
        && bytes[..3].iter().all(u8::is_ascii_digit)
        && bytes.get(3).is_none_or(|b| !b.is_ascii_digit())
}

/// Reads one figure or a range of two (`5,100–5,500` with an en dash between, or `15 to 30`),
/// from the start.
fn figures(text: &str) -> Option<(Vec<Figure>, usize)> {
    let (first, mut end) = figure(text)?;
    let mut list = vec![first];
    if let Some(join) = range_join(&text[end..])
        && let Some((second, len)) = figure(&text[end + join..])
    {
        list.push(second);
        end += join + len;
    }
    Some((list, end))
}

/// The length of what joins a range's two figures at the start of `text`: an en dash, or `to`
/// with a space on each side.
fn range_join(text: &str) -> Option<usize> {
    if text.starts_with('–') {
        return Some('–'.len_utf8());
    }
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    let rest = text[first.len_utf8()..].strip_prefix("to")?;
    let after = rest.chars().next()?;
    (is_space(first) && is_space(after)).then(|| text.len() - rest.len() + after.len_utf8())
}

/// The unit among `units` that `text` starts with, ending where a word would: not before a
/// letter, digit, `/`, `²` or `³` (`m/s²`, `mm`, `min`). A hyphen ends it: `3 m-long` is 3 m.
fn unit_at<'a>(text: &str, units: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    units.into_iter().find(|unit| {
        text.strip_prefix(unit).is_some_and(|rest| {
            rest.chars()
                .next()
                .is_none_or(|c| !(c.is_alphanumeric() || matches!(c, '/' | '²' | '³')))
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

/// The bracket after a quantity, from its `(`: if it gives the quantity in a US unit, converted
/// right, the length in bytes of the conversion (from the `(` to the unit's end, and the `)` too
/// when it follows the unit, so anything else the bracket says stays), else what is wrong.
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
        let unit_end = after.len() - rest.len() + unit.len();
        if after[unit_end..].starts_with(')') {
            Ok(unit_end + 1)
        } else if after[unit_end..].contains(')') {
            Ok(unit_end)
        } else {
            Err("its bracket isn't closed".to_owned())
        }
    } else {
        let want: Vec<String> = si
            .iter()
            .map(|si| format!("{:.2}", si.value * factor).replace('-', "−"))
            .collect();
        Err(format!(
            "its bracket's {unit} aren't it converted ({} {unit})",
            want.join("–")
        ))
    }
}

/// What to write in the bracket: the figure converted into the first US unit, to one place
/// fewer than the SI figure has, at least a tenth under 10 and at least two significant figures
/// (`2 mm` is `0.079 in`), grouped by commas from 1,000 as `writing.md` groups prose.
fn suggestion(si: &[Figure], units: &[(&str, f64)], space: &str, join: &str) -> String {
    let (unit, factor) = units[0];
    let ends: Vec<String> = si
        .iter()
        .map(|figure| {
            let value = figure.value * factor;
            let places = (-figure.place.log10()).round() as i32 - 1;
            // At most 12 places: a figure written finer than that has no use for more.
            let places = if value.abs() < 10.0 {
                places.max(1)
            } else {
                places.max(0)
            };
            // Two significant figures: a value under 1 needs places past its leading zeros.
            let significant = if value == 0.0 {
                0
            } else {
                1 - value.abs().log10().floor() as i32
            };
            let places = places.max(significant).clamp(0, 12) as usize;
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
    format!("({}{space}{unit})", ends.join(join))
}

/// Whether the text ends, past spaces, with an operator that makes what follows a term: a `−` or
/// `-` only with a space after it, since one joined to the figure is its sign.
fn operator_before(before: &str) -> bool {
    // A term in brackets is one too: `π × (5 mm)²`.
    let before = before
        .trim_end_matches(is_space)
        .strip_suffix('(')
        .unwrap_or(before);
    let trimmed = before.trim_end_matches(is_space);
    let spaced = trimmed.len() < before.len();
    match trimmed.chars().next_back() {
        Some('-') => spaced,
        Some(c) => OPERATORS.contains(&c),
        None => false,
    }
}

/// Whether the SI figures are themselves the conversion of a US figure before them, in brackets
/// after it: `1.52 in (38 mm)`, `0.583 ft (0.178 m)`, `1/8 in (3.175 mm)`. `before` is the run
/// up to the figures; the US figure may be a fraction, and must convert to the SI one within half
/// the SI figure's last place plus half the US figure's own.
fn converted_from_us(before: &str, si: &[Figure], units: &[(&str, f64)]) -> bool {
    let Some(before) = before
        .trim_end_matches(is_space)
        .strip_suffix('(')
        .map(|text| text.trim_end_matches(is_space))
    else {
        return false;
    };
    let Some((unit, factor)) = units
        .iter()
        .find(|(unit, _)| before.ends_with(unit))
        .map(|(unit, factor)| (*unit, *factor))
    else {
        return false;
    };
    let number = before[..before.len() - unit.len()].trim_end_matches(is_space);
    let start = number
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_digit() || matches!(c, '.' | ',' | '/' | '–' | '−'))
        .last()
        .map_or(number.len(), |(at, _)| at);
    // A range written with `to` (`15 to 30 ft`) is read whole, back to its first figure.
    let start = number[..start]
        .strip_suffix(|c| is_space(c))
        .and_then(|head| head.strip_suffix("to"))
        .and_then(|head| head.strip_suffix(|c| is_space(c)))
        .and_then(|head| {
            let from = head
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_ascii_digit() || matches!(c, '.' | ',' | '−'))
                .last()?
                .0;
            figure(&head[from..])
                .filter(|(_, len)| from + len == head.len())
                .map(|_| from)
        })
        .unwrap_or(start);
    let text = &number[start..];
    // A word right before the figure makes it part of a name (`MMT-1.52`), not a quantity.
    if text.is_empty()
        || number[..start]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || matches!(c, '-' | '_'))
    {
        return false;
    }
    let us: Vec<Figure> = if let Some((numerator, denominator)) = text.split_once('/') {
        let (Ok(numerator), Ok(denominator)) =
            (numerator.parse::<f64>(), denominator.parse::<f64>())
        else {
            return false;
        };
        if denominator == 0.0 {
            return false;
        }
        // A fraction states its value exactly.
        vec![Figure {
            value: numerator / denominator,
            place: 0.0,
            scientific: false,
        }]
    } else {
        match figures(text) {
            Some((list, len)) if len == text.len() => list,
            _ => return false,
        }
    };
    us.len() == si.len()
        && si
            .iter()
            .zip(&us)
            .all(|(si, us)| converts(*us, *si, 1.0 / factor))
}

/// Whether the text starts, past spaces, with an operator: a `−` or `-` only with a space after.
fn operator_after(after: &str) -> bool {
    let mut trimmed = after.trim_start_matches(is_space);
    // A term in brackets is one too, and a power of it: `(5 mm)² × 2`.
    while let Some(closed) = trimmed.strip_prefix(')') {
        if closed.starts_with(['²', '³']) {
            return true;
        }
        trimmed = closed.trim_start_matches(is_space);
    }
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
pub(crate) fn without_conversions(text: &str) -> String {
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
/// the space before it to its `)`), `Ok(None)` a term of a worked calculation, a motor's size or
/// the conversion of a US figure before it, `Err` what is wrong.
fn scan(run: &str, mut found: impl FnMut(usize, Result<Option<std::ops::Range<usize>>, String>)) {
    let mut at = 0;
    while at < run.len() {
        let rest = &run[at..];
        let c = rest.chars().next().unwrap_or(' ');
        let starts = c.is_ascii_digit() || matches!(c, '−' | '-' | '+');
        // A figure starts after something that isn't part of a word or of another figure. A
        // comma may come before it, since a whole figure is read at once: `1,40 m` is 40 m.
        let previous = run[..at].chars().next_back();
        let free = previous.is_none_or(|p| !(p.is_alphanumeric() || matches!(p, '.' | '_' | '–')));
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
        // The last group of a number grouped by spaces (`63 781 370 m`): only commas group.
        let before = &run[..at];
        let grouped = written.len() == 3
            && written.bytes().all(|b| b.is_ascii_digit())
            && before
                .strip_suffix(|c| is_space(c))
                .is_some_and(|head| head.ends_with(|c: char| c.is_ascii_digit()));
        if grouped {
            found(
                at,
                Err(format!(
                    "`{written}{space}{unit}` is the last group of a number grouped by spaces, \
                     which the check can't read whole; group it with commas (`63,781,370 m`) \
                     and give its US units in brackets after it (#400, ADR-210)"
                )),
            );
            at = quantity_end;
            continue;
        }
        if operator_before(before)
            || operator_after(after_unit)
            || si.iter().any(|f| f.scientific)
            || (unit == "mm" && motor_size(&si, after_unit))
            || converted_from_us(&run[..at], &si, us_units)
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
                        suggestion(
                            &si,
                            us_units,
                            sep,
                            if si.len() > 1 && !written.contains('–') {
                                " to "
                            } else {
                                "–"
                            }
                        )
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
            "a 90 cm (35 in) flat parachute",
            "a 3 m (9.8 ft) long rail",
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
            "a 3 m-long rail",
            "a 10 km-wide area",
            "a 90 cm flat parachute",
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
    fn odd_figures_neither_panic_nor_pass() {
        // A comma is a thousands separator only before three digits.
        assert_eq!(messages("1,40 m (459 ft)").len(), 1);
        assert!(messages("1,400 m (4,593 ft)").is_empty());
        // Huge exponents and absurd decimals are read without overflowing.
        let _ = messages("1.25e-2147483647 m and 1e2147483647 m");
        let long = format!("0.{}1 m", "0".repeat(400));
        assert_eq!(messages(&long).len(), 1);
        // A negative conversion's message uses the minus sign.
        let found = messages("−3 m (−12 ft)");
        assert!(found[0].contains("−9.84 ft"), "{found:?}");
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
        // So does what a bracket says after the conversion: only the feet drop out.
        assert_eq!(
            without_conversions("apogee 324 m (1,063 ft, +3.2% on OpenRocket) here"),
            "apogee 324 m, +3.2% on OpenRocket) here"
        );
    }

    #[test]
    fn millimeters_take_inches() {
        assert!(messages("a 2\u{a0}mm (0.079\u{a0}in) wall, 25.4 mm (1.00 in) across").is_empty());
        let found = messages("a 2 mm wall");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("`2 mm (0.079 in)`"), "{found:?}");
        // Inches only: a size in millimeters isn't given in feet.
        assert!(messages("a 300 mm (0.98 ft) tube")[0].contains("not a unit of the same quantity"));
        assert!(messages("a 25 mm (1.5 in) tube")[0].contains("aren't it converted"));
        // `mm` is read whole, not as meters, and `mm²` isn't a length.
        assert_eq!(messages("a 2 mm wall")[0].matches("`2 mm`").count(), 1);
        assert!(messages("an area of 40 mm² here").is_empty());
    }

    #[test]
    fn a_motors_or_mounts_size_stays_in_millimeters() {
        for run in [
            "a 54 mm motor",
            "the 38\u{a0}mm mount",
            "a 29 mm and a 38 mm motor",
            "29 mm, 38 mm and 54 mm motors",
            "a 98 mm Pro98 case",
            "a 75 mm reloadable motor's thrust",
            "a 24 mm single-use motor",
            "a 54 mm motor-mount tube",
            "a 38 mm MMT",
            "a \"29 mm\" case",
            "a 38 mm Cesaroni I175",
            "a 38 mm AeroTech H170M's thrust",
            "a 38 mm case",
        ] {
            assert!(messages(run).is_empty(), "{run}: {:?}", messages(run));
        }
        // Not a motor's size: another diameter, no motor word close enough, or punctuation
        // between.
        for run in [
            "a 55 mm motor",
            "a 54 mm body tube",
            "a 54 mm body tube around the motor",
            "a 54 mm airframe. The motor",
            "a 54 mm tube, the motor's",
            "a 2 mm wall on the mount",
            "a 54 mm (2.1 in) motor",
            "a 98 mm airframe, 54 mm motor mount",
            "a 54 mm body tube and 38 mm motor mount",
            "a 75 mm airframe and its motor",
            "the 54 mm airframe or motor tube",
            "a 54 mm nose cone 2 motors",
            "The airframe is 98 mm in either case.",
            "a 54 mm tube or a 38 mm motor",
            "a 54 mm M3 screw",
            "a 54 mm G10 tube",
            "a 98 mm G10 airframe",
            "a 54 mm C130 frame",
            "the 38 mm H170M's thrust",
        ] {
            let found = messages(run);
            if run.contains("(2.1 in)") {
                assert!(found.is_empty(), "{run}: {found:?}");
            } else {
                // Only the first size fails: a motor's size after it stays exempt.
                let first = &run[run.find(|c: char| c.is_ascii_digit()).unwrap()..];
                let first = &first[..first.find(" mm").unwrap() + 3];
                assert_eq!(found.len(), 1, "{run}: {found:?}");
                assert!(
                    found[0].starts_with(&format!("`{first}`")),
                    "{run}: {found:?}"
                );
            }
        }
    }

    #[test]
    fn si_in_brackets_after_us_figures_is_their_conversion() {
        for run in [
            "the 1.52 in (38 mm) motor tube",
            "`d` = 0.583 ft (0.178 m). Its nose",
            "1/8 in (3.175 mm) birch plywood",
            "| 1,010 ft (307.8 m) |",
            "a 1.140 in (28.956 mm) tube",
            "60–70 ft (18–21 m) high",
            "15 to 30 ft (4.6 to 9.1 m) high",
            "4,000 to 6,000 ft (1,219 to 1,829 m) up",
        ] {
            assert!(messages(run).is_empty(), "{run}: {:?}", messages(run));
        }
        // Not the conversion, a name's number, or not in brackets right after the US figure.
        for run in [
            "the 1.52 in (40 mm) motor tube",
            "MMT-1.52 in (38 mm) tube",
            "1.52 in, or 38 mm here",
            "3 ft (2 m) high",
            "1/0 in (3 mm) here",
            "1 to 3 ft (0.9 m) tall",
        ] {
            assert_eq!(messages(run).len(), 1, "{run}: {:?}", messages(run));
        }
    }

    #[test]
    fn suggestions_keep_two_significant_figures() {
        let mm = SI_UNITS.iter().find(|(unit, _)| *unit == "mm").unwrap().1;
        let m = SI_UNITS.iter().find(|(unit, _)| *unit == "m").unwrap().1;
        let one = |text: &str| figures(text).unwrap().0;
        assert_eq!(suggestion(&one("2"), mm, " ", "–"), "(0.079 in)");
        assert_eq!(suggestion(&one("0.5"), mm, " ", "–"), "(0.020 in)");
        assert_eq!(suggestion(&one("54"), mm, " ", "–"), "(2.1 in)");
        assert_eq!(suggestion(&one("0.3"), m, " ", "–"), "(0.98 ft)");
        assert_eq!(suggestion(&one("1,400"), m, " ", "–"), "(4,593 ft)");
        assert_eq!(suggestion(&one("0"), m, " ", "–"), "(0.0 ft)");
        assert_eq!(
            suggestion(&one("15 to 30"), m, " ", " to "),
            "(49 to 98 ft)"
        );
    }

    #[test]
    fn ranges_written_with_to_need_both_ends() {
        assert!(messages("at 15 to 30 m/s (49 to 98 ft/s) the air").is_empty());
        assert!(messages("at 15 to 30 m/s (49–98 ft/s) the air").is_empty());
        let found = messages("at 15 to 30 m/s (98 ft/s) the air");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("a range needs a range"), "{found:?}");
        assert!(
            found[0].contains("`15 to 30 m/s (49 to 98 ft/s)`"),
            "{found:?}"
        );
        // `to` needs its spaces, and a word that merely starts with it is no range.
        assert_eq!(range_join(" to 30"), Some(4));
        assert_eq!(range_join(" to30"), None);
        assert_eq!(range_join(" tons 30"), None);
        assert_eq!(range_join("to 30"), None);
        assert!(messages("from 15 tons 30 m away")[0].starts_with("`30 m`"));
    }

    #[test]
    fn numbers_grouped_by_spaces_fail() {
        let found = messages("the default radius, 63 781 370 m (1,214 ft) here");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("grouped by spaces"), "{found:?}");
        assert!(messages("the default radius, 63,781,370 m (39,632 mi) here").is_empty());
        // A count before a quantity is not a group: it isn't three digits, or no digit precedes.
        assert!(messages("in 3 1,400 m (4,593 ft) runs").is_empty());
        assert!(messages("rows of 120 m (394 ft)").is_empty());
    }

    #[test]
    fn bracketed_terms_of_a_calculation_are_exempt() {
        for run in [
            "2/3 × π × (5 mm)² × 2 mm here",
            "a mass of (2 m + 3 m) here",
            "× (5 mm) is",
        ] {
            assert!(messages(run).is_empty(), "{run}: {:?}", messages(run));
        }
        // A bracket after a word is a parenthesis, not a term.
        assert_eq!(messages("the gap (5 mm) here").len(), 1);
    }
}
