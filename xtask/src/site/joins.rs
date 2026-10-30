//! A figure held to its unit (#476, ADR-295 §3): the product system's `data.md` (*Numbers*) puts
//! "a space between a number and its unit (NIST SP 811, the US guide to SI), and a non-breaking
//! one, so a line never ends between them": `21.5 °C`, `644 ft/s`, with U+00A0 between.
//!
//! [`splits`] finds each figure a breaking space parts from the unit after it, in text as it
//! renders:
//!
//! - **A figure** is digits at the start of a number: not against a letter, a digit, `_`, a
//!   point or a comma, nor after a hyphen that follows a letter or a digit (`ADR-208`, `Pro38`,
//!   `0.1.0`, `2026-10-04` aren't figures). It runs through its grouping commas, its decimals, an
//!   exponent (`1e-9`) and superscript digits (`10⁶`). A sign before it stays where it is.
//! - **A figure written in groups of three with spaces** (NIST SP 811 §10.5.3, the style long
//!   decimals and quoted standards keep: `22 632 Pa`, `9.806 65`, `0.001 953 125`) is one
//!   figure, and a breaking space between its groups is a split too (#891): a line ending there
//!   reads `22` and `632 Pa` as two numbers. Before the point, a first group of one to three
//!   digits takes each further group of exactly three, unless commas group it; after the point,
//!   a group of three takes a next of one to three, and the groups go on while each holds three.
//!   A group isn't taken when a digit, or a comma and a digit, follow it, nor after the point
//!   when a point and a digit do (`0.500 0.800`, a list, stays two figures). The rule reads digits, not meaning: two counts written
//!   side by side (`12 100`) are joined as one, which only keeps them on one line.
//! - **A unit** is the symbol of one of [`UNITS`], the units `data.md` writes and the project's
//!   own (`cal`, `calibers`, `px`), as a whole word after the figure: its letters not followed
//!   by a digit 0 to 9 or `_` (`g06` is a code), though a superscript power may follow (`m²`).
//!   `in` is an inch unless a space and then a word, a figure or the end of the run follow it:
//!   `(0.12 in)`, `0.12 in.`, `1.52 in (38 mm)` and `4 in × 40` are inches. With one after it,
//!   it is also the preposition (`0.5 in both`, `1 in <code>SI</code>`), so a page joins an inch
//!   there by hand (`2.6&nbsp;in rocket`). Units spelled out (`3 minutes`) aren't read.
//! - **A breaking space** is any white space but the no-break spaces U+00A0, U+2007 and U+202F,
//!   a line break in the source included.
//!
//! The site's pages are joined in the built HTML, not in their source ([`join`]): after mdBook
//! writes the search index, which keeps the plain space, so search reads the same. It reads the
//! text of each page as the browser draws it, across inline tags (`<strong>5</strong> m` is one
//! run) but not across blocks, and never in code, preformatted text, keyboard or sample text,
//! math, SVG, scripts, styles or text areas, nor in the API reference. [`check`] then fails any
//! pair still split in the built pages, and an inline equation that ends in a number (`<mn>`) with
//! its unit after it ([`Math`]): Chrome ends a line between an equation's box and the no-break
//! space after it, so the unit goes inside the equation (`4593\,\mathrm{ft}$`).
//! The READMEs, which GitHub, crates.io and PyPI show as written, hold the no-break space in their
//! source ([`readme_splits`]): `5&nbsp;m`.
//!
//! The rule reads symbols, not meaning: a unit symbol used as a word after a figure is joined
//! too, and an inch with a word after it is left to the writer and to reviews.

use std::fs;
use std::ops::Range;
use std::path::Path;

use super::html_files;
use super::units::rendered_runs_with;
use super::words::Renderer;

/// The unit symbols a figure is joined to, as `data.md` writes them, and the project's own: the
/// caliber (`cal`, `caliber`, `calibers`) and the screen's pixel (`px`).
pub(super) const UNITS: [&str; 70] = [
    // Length.
    "m", "km", "cm", "mm", "µm", "μm", "nm", "ft", "in", "mi", "px", "cal", "caliber", "calibers",
    // Time and frequency.
    "s", "ms", "µs", "μs", "ns", "min", "h", "Hz", "kHz", "MHz", "rpm",
    // Mass and force.
    "kg", "g", "mg", "lb", "oz", "N", "kN", "lbf", "Ns", // Pressure.
    "Pa", "hPa", "kPa", "MPa", "GPa", "psi", "ksi", "mbar", "bar",
    // Temperature and angle.
    "K", "°C", "°F", "rad", "deg", // Energy, power and electricity.
    "J", "kJ", "W", "kW", "V", "mV", "mA", "mAh", "Ω",
    // Magnetic field, speed, data and sound.
    "nT", "µT", "μT", "mph", "kt", "kn", "kB", "MB", "GB", "KiB", "MiB", "GiB", "dB",
];

/// The spaces a line can't end at: no-break, figure and narrow no-break.
const NO_BREAK: [char; 3] = ['\u{a0}', '\u{2007}', '\u{202f}'];

/// A figure split from its unit, or a figure's groups split from each other, by a breaking
/// space, as byte ranges in the text read.
#[derive(Debug, PartialEq)]
pub(super) struct Split {
    /// The white space between them.
    pub(super) space: Range<usize>,
    /// The figure, the space and the unit; or, for a split inside a figure, the figure.
    pub(super) pair: Range<usize>,
    /// Whether the space parts two groups of one figure (`22 632`) rather than a figure and its
    /// unit.
    pub(super) inside: bool,
}

/// Whether `space` is white space a line can end at: not empty, nor only no-break spaces.
fn breaks(space: &str) -> bool {
    space.chars().any(|c| !NO_BREAK.contains(&c))
}

/// Each figure in `text` that a breaking space parts from its unit, and each breaking space
/// between the groups of a figure written with spaces.
pub(super) fn splits(text: &str) -> Vec<Split> {
    let mut found = Vec::new();
    let mut at = 0;
    while let Some((offset, _)) = text[at..].char_indices().find(|(_, c)| c.is_ascii_digit()) {
        let start = at + offset;
        if !figure_starts(&text[..start]) {
            at = start + 1;
            continue;
        }
        let (end, gaps) = figure_end(text, start);
        at = end;
        for gap in gaps.into_iter().filter(|gap| breaks(&text[gap.clone()])) {
            found.push(Split {
                space: gap,
                pair: start..end,
                inside: true,
            });
        }
        let space = end..end + white_space(&text[end..]);
        if space.is_empty() || !breaks(&text[space.clone()]) {
            continue;
        }
        if let Some(unit_end) = unit_after(text, space.end) {
            found.push(Split {
                space,
                pair: start..unit_end,
                inside: false,
            });
        }
    }
    found
}

/// Whether a figure can start after `before`: not inside a word, a number or a code.
fn figure_starts(before: &str) -> bool {
    let mut back = before.chars().rev();
    match back.next() {
        None => true,
        Some(c) if c.is_alphanumeric() || matches!(c, '_' | '.' | ',') => false,
        // A hyphen after a letter or a digit joins a name or a date (`ADR-208`, `2026-10-04`).
        Some('-' | '‐' | '‑') => !back.next().is_some_and(char::is_alphanumeric),
        Some(_) => true,
    }
}

/// The length of the white space that opens `text`.
fn white_space(text: &str) -> usize {
    text.char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map_or(text.len(), |(len, _)| len)
}

/// The end of the figure that starts at `start`, and the white space between its groups when it
/// is written in groups with spaces: its digits and grouping commas or spaced groups, its
/// decimals and their spaced groups, an exponent and superscript digits.
fn figure_end(text: &str, start: usize) -> (usize, Vec<Range<usize>>) {
    let digits = |from: usize| {
        from + text[from..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(text.len() - from)
    };
    // The next group after the white space at `from`, if it holds `sizes` digits and no digit,
    // nor one of `marks` before a digit, follows it.
    let group = |from: usize, sizes: Range<usize>, marks: &[char]| {
        let open = from + white_space(&text[from..]);
        let close = digits(open);
        let rest = &text[close..];
        let more = rest
            .strip_prefix(marks)
            .is_some_and(|after| after.starts_with(|c: char| c.is_ascii_digit()));
        (open > from && sizes.contains(&(close - open)) && !more).then_some((open, close))
    };
    let mut gaps = Vec::new();
    let mut end = digits(start);
    if text[end..].starts_with(',') {
        // Grouping commas, each before three digits.
        while text[end..].starts_with(',') && digits(end + 1) == end + 4 {
            end += 4;
        }
    } else if end - start <= 3 {
        while let Some((open, close)) = group(end, 3..4, &[',']) {
            gaps.push(end..open);
            end = close;
        }
    }
    if text[end..].starts_with('.') && digits(end + 1) > end + 1 {
        let point = end;
        end = digits(end + 1);
        let mut size = end - point - 1;
        while size == 3 {
            let Some((open, close)) = group(end, 1..4, &['.', ',']) else {
                break;
            };
            gaps.push(end..open);
            size = close - open;
            end = close;
        }
    }
    // An exponent, `e` and a signed whole number, but not a word that opens with `e`.
    let rest = &text[end..];
    if let Some(sign) = rest.strip_prefix(['e', 'E']) {
        let unsigned = sign.trim_start_matches(['-', '+', '−']);
        if unsigned.starts_with(|c: char| c.is_ascii_digit()) {
            end = digits(end + (rest.len() - unsigned.len()));
        }
    }
    let end = end
        + text[end..]
            .find(|c: char| !"⁻⁰¹²³⁴⁵⁶⁷⁸⁹".contains(c))
            .unwrap_or(text.len() - end);
    (end, gaps)
}

/// The end of the unit symbol at `at`, if one of [`UNITS`] stands there as a whole word.
fn unit_after(text: &str, at: usize) -> Option<usize> {
    let rest = &text[at..];
    let len = rest
        .char_indices()
        .find(|&(_, c)| !(c.is_alphabetic() || c == '°'))
        .map_or(rest.len(), |(len, _)| len);
    let unit = &rest[..len];
    // A whole word: not the start of a code (`g06`, `m_dot`).
    if !UNITS.contains(&unit) || rest[len..].starts_with(|c: char| c.is_ascii_digit() || c == '_') {
        return None;
    }
    // An inch only with no word or figure after it: `in` before one is also the preposition.
    if unit == "in" {
        let after = rest[len..].trim_start();
        let spaced = after.len() < rest.len() - len;
        if spaced && after.chars().next().is_none_or(char::is_alphanumeric) {
            return None;
        }
    }
    Some(at + len)
}

/// Each figure split from its unit in a README, as (line, message), as `renderer` shows it.
pub(super) fn readme_splits(text: &str, renderer: Renderer) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let mut problems = Vec::new();
    for run in rendered_runs_with(text, renderer.options()) {
        for split in splits(&run.text) {
            problems.push((
                line(run.source(split.pair.start)),
                format!(
                    "{}: write the no-break space in the source, `{}`",
                    message(&run.text, &split),
                    joined(&run.text, &split, "&nbsp;")
                ),
            ));
        }
    }
    problems
}

/// What a split says, whatever is asked to fix it.
fn message(text: &str, split: &Split) -> String {
    let between = if split.inside {
        "inside the number (NIST SP 811 §10.5.3: a space that groups digits doesn't end a line)"
    } else {
        "between the figure and its unit (data.md, Numbers: a non-breaking space, so a line \
         never ends between them)"
    };
    format!(
        "`{}` in \"{}\" can break {between}",
        text[split.pair.clone()]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
        shown(&super::figures::context(text, split.pair.clone()))
    )
}

/// A message's context with each inline equation's box (U+FFFC) shown as a word, since a
/// terminal draws the box as nothing or a square.
fn shown(context: &str) -> String {
    context.replace('\u{fffc}', "[equation]")
}

/// The split pair with `space` between the figure and the unit.
fn joined(text: &str, split: &Split, space: &str) -> String {
    format!(
        "{}{space}{}",
        &text[split.pair.start..split.space.start],
        &text[split.space.end..split.pair.end]
    )
}

/// The elements whose text [`join`] and [`check`] don't read: code, preformatted, keyboard and
/// sample text, math (but whether an inline equation ends in a number, [`Math`]), SVG, scripts,
/// styles and text areas.
const UNREAD: [&str; 9] = [
    "code", "pre", "kbd", "samp", "math", "svg", "script", "style", "textarea",
];

/// The elements that continue a run of text, as the browser draws them on one line.
const INLINE: [&str; 24] = [
    "a", "abbr", "b", "bdi", "bdo", "cite", "data", "del", "dfn", "em", "i", "ins", "label",
    "mark", "q", "s", "small", "span", "strong", "sub", "sup", "time", "u", "var",
];

/// A run of a page's text, with where each of its characters came from in the HTML.
#[derive(Default)]
struct HtmlRun {
    /// The text, entities decoded; an inline equation stands as U+FFFC, the object replacement
    /// character, which is neither a space, a figure nor a unit.
    text: String,
    /// For each character's byte offset in [`HtmlRun::text`], its byte range in the HTML.
    from: Vec<(usize, Range<usize>)>,
    /// Where each inline equation that ends in a number ends in [`HtmlRun::text`], and the number.
    equations: Vec<(usize, String)>,
}

impl HtmlRun {
    /// The byte range in the HTML of each character in the text's bytes `range`, in order. A
    /// space between a figure and its unit may hold a tag (`5 <span> m`), which isn't in them.
    fn html(&self, range: Range<usize>) -> impl Iterator<Item = Range<usize>> + '_ {
        let first = self.from.partition_point(|(start, _)| *start < range.start);
        self.from[first..]
            .iter()
            .take_while(move |(start, _)| *start < range.end)
            .map(|(_, html)| html.clone())
    }
}

/// The runs of text `html` draws, outside the [`UNREAD`] elements, each ending at a block.
fn html_runs(html: &str) -> Vec<HtmlRun> {
    let mut runs = Vec::new();
    let mut run = HtmlRun::default();
    let mut unread = 0usize;
    let mut math: Option<Math> = None;
    let mut at = 0;
    let flush = |run: &mut HtmlRun, runs: &mut Vec<HtmlRun>| {
        if !run.text.is_empty() {
            runs.push(std::mem::take(run));
        }
    };
    while at < html.len() {
        let rest = &html[at..];
        if rest.starts_with("<!--") {
            at += rest.find("-->").map_or(rest.len(), |end| end + 3);
            continue;
        }
        if let Some(tag) = tag_at(rest) {
            let raw = matches!(tag.name.as_str(), "script" | "style" | "textarea");
            if raw && !tag.closing && !tag.self_closing {
                // Raw text, which holds no tags: skip it and its end tag.
                let close = format!("</{}", tag.name);
                at += tag.len;
                let lower = html[at..].to_ascii_lowercase();
                at += lower.find(&close).map_or(html.len() - at, |end| {
                    end + html[at + end..].find('>').map_or(close.len(), |gt| gt + 1)
                });
                flush(&mut run, &mut runs);
                continue;
            }
            if tag.name == "math" && !tag.self_closing && (tag.closing == math.is_some()) {
                at += tag.len;
                if tag.closing {
                    unread = unread.saturating_sub(1);
                    let ended = math.take();
                    if let Some(number) = ended.as_ref().and_then(Math::figure) {
                        run.equations.push((run.text.len(), number.to_owned()));
                    } else if ended.is_some_and(|ended| ended.block) {
                        flush(&mut run, &mut runs);
                    }
                } else if unread == 0 {
                    let block = rest[..tag.len].contains("display=\"block\"");
                    if block {
                        flush(&mut run, &mut runs);
                    } else {
                        // The equation is one box in the line, never edited.
                        run.from.push((run.text.len(), at..at));
                        run.text.push('\u{fffc}');
                    }
                    math = Some(Math {
                        block,
                        number: None,
                        open: None,
                    });
                    unread += 1;
                } else {
                    unread += 1;
                }
                continue;
            }
            if let Some(math) = math.as_mut() {
                // Inside an equation a tag is a part of it, never the end of the run.
                math.leaf(&tag);
                at += tag.len;
                continue;
            }
            if UNREAD.contains(&tag.name.as_str()) && !tag.self_closing && !raw {
                if tag.closing {
                    unread = unread.saturating_sub(1);
                } else {
                    unread += 1;
                }
            }
            if !INLINE.contains(&tag.name.as_str()) {
                flush(&mut run, &mut runs);
            }
            at += tag.len;
            continue;
        }
        let (c, len) = char_at(rest);
        if unread == 0 {
            run.from.push((run.text.len(), at..at + len));
            run.text.push(c);
        } else if let Some(open) = math.as_mut().and_then(|math| math.open.as_mut()) {
            open.push(c);
        }
        at += len;
    }
    flush(&mut run, &mut runs);
    runs
}

/// An equation being read. Inline math whose last leaf is a number (`<mn>`) ends in a figure, and
/// [`unit_after_equations`] fails a unit after it (`= 4593$ ft`): a browser can end a line between
/// the equation's box and a no-break space. Display math, a block of its own, ends the run.
struct Math {
    /// Whether it is display math, `<math display="block">`.
    block: bool,
    /// The last leaf's text, when it was a number.
    number: Option<String>,
    /// The text of the number leaf being read.
    open: Option<String>,
}

impl Math {
    /// Note `tag`, inside the math: a number leaf opens or closes, or another leaf starts.
    fn leaf(&mut self, tag: &Tag) {
        match (tag.name.as_str(), tag.closing) {
            ("mn", false) => self.open = Some(String::new()),
            ("mn", true) => self.number = self.open.take(),
            ("mi" | "mo" | "mtext" | "ms" | "mspace", _) => {
                self.number = None;
                self.open = None;
            }
            _ => {}
        }
    }

    /// The figure the math ends with, if it is inline and ends with a number.
    fn figure(&self) -> Option<&str> {
        self.number
            .as_deref()
            .map(str::trim)
            .filter(|number| !self.block && number.ends_with(|c: char| c.is_ascii_digit()))
    }
}

/// A tag at the start of `html`.
struct Tag {
    /// Its name, in lower case.
    name: String,
    /// Whether it ends an element, `</p>`.
    closing: bool,
    /// Whether it ends itself, `<path />`.
    self_closing: bool,
    /// Its length in bytes, through its `>`.
    len: usize,
}

/// The tag `html` opens with, if it opens with one (`<` and a letter, `/` or `!`).
fn tag_at(html: &str) -> Option<Tag> {
    let after = html.strip_prefix('<')?;
    let closing = after.starts_with('/');
    let named = after.trim_start_matches(['/', '!']);
    if !named.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return None;
    }
    let name: String = named
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_ascii_lowercase();
    // The `>` that ends it, outside quoted attribute values.
    let mut quote = None;
    for (offset, c) in html.char_indices().skip(1) {
        match (quote, c) {
            (Some(q), _) if c == q => quote = None,
            (None, '"' | '\'') => quote = Some(c),
            (None, '>') => {
                return Some(Tag {
                    name,
                    closing,
                    self_closing: html[..offset].ends_with('/'),
                    len: offset + 1,
                });
            }
            _ => {}
        }
    }
    None
}

/// The character `html` opens with, an entity decoded, and its length in the HTML. An entity
/// this doesn't know stands as U+FFFD, which is neither a space, a figure nor a unit.
fn char_at(html: &str) -> (char, usize) {
    if let Some(entity) = html.strip_prefix('&')
        && let Some(end) = entity.find(';').filter(|&end| {
            end <= 10
                && entity[..end]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '#')
        })
    {
        let name = &entity[..end];
        let decoded = match name {
            "nbsp" => Some('\u{a0}'),
            "ensp" => Some('\u{2002}'),
            "emsp" => Some('\u{2003}'),
            "thinsp" => Some('\u{2009}'),
            "numsp" => Some('\u{2007}'),
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => name.strip_prefix('#').and_then(|code| {
                match code.strip_prefix(['x', 'X']) {
                    Some(hex) => u32::from_str_radix(hex, 16).ok(),
                    None => code.parse().ok(),
                }
                .and_then(char::from_u32)
            }),
        };
        return (decoded.unwrap_or('\u{fffd}'), end + 2);
    }
    // A slice of a `&str` that isn't empty starts with a character.
    let c = html.chars().next().unwrap_or('\u{fffd}');
    (c, c.len_utf8())
}

/// How many splits [`join`] joined.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Joined {
    /// Figures joined to their units.
    pub(super) units: usize,
    /// Spaces joined between the groups of a figure written with spaces.
    pub(super) groups: usize,
}

impl Joined {
    /// Whether it joined nothing.
    fn none(self) -> bool {
        self == Self::default()
    }

    /// Both counts summed with `other`'s.
    fn add(&mut self, other: Self) {
        self.units += other.units;
        self.groups += other.groups;
    }
}

/// `html` with each figure joined to its unit, and each figure's groups to each other, by a
/// no-break space, and how many it joined.
fn joined_html(html: &str) -> (String, Joined) {
    // Each split's first space becomes a no-break space and the rest go, so the pair keeps one
    // space and any tag between them.
    let mut count = Joined::default();
    let mut edits: Vec<(Range<usize>, &str)> = Vec::new();
    for run in html_runs(html) {
        for split in splits(&run.text) {
            if split.inside {
                count.groups += 1;
            } else {
                count.units += 1;
            }
            for (nth, space) in run.html(split.space.clone()).enumerate() {
                edits.push((space, if nth == 0 { "\u{a0}" } else { "" }));
            }
        }
    }
    edits.sort_by_key(|(space, _)| space.start);
    let mut out = String::with_capacity(html.len());
    let mut at = 0;
    for (space, with) in &edits {
        out.push_str(&html[at..space.start]);
        out.push_str(with);
        at = space.end;
    }
    out.push_str(&html[at..]);
    (out, count)
}

/// Join each figure to its unit with a no-break space in the built pages under `output`, but
/// the API reference, after mdBook has written the search index; how many it joined, on the
/// pages and on the print page, which repeats them.
pub(super) fn join(output: &Path) -> Result<(Joined, Joined), String> {
    let mut count = (Joined::default(), Joined::default());
    for name in pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        let (drawn, joined) = joined_html(&html);
        if !joined.none() {
            fs::write(&path, drawn)
                .map_err(|err| format!("could not write {}: {err}", path.display()))?;
        }
        if name == super::print::PAGE {
            count.1.add(joined);
        } else {
            count.0.add(joined);
        }
    }
    Ok(count)
}

/// Each figure a breaking space still parts from its unit in the built pages under `output`, as
/// `page: message`.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    for name in pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        problems.extend(
            page_splits(&html)
                .into_iter()
                .map(|why| format!("{name}: {why}")),
        );
    }
    Ok(problems)
}

/// What [`check`] says of one built page.
fn page_splits(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for run in html_runs(html) {
        for split in splits(&run.text) {
            problems.push(format!(
                "{}, which `join` (xtask/src/site/joins.rs) should have joined: `{}`",
                message(&run.text, &split),
                joined(&run.text, &split, "\u{a0}")
            ));
        }
        problems.extend(unit_after_equations(&run));
    }
    problems
}

/// Each inline equation in `run` that ends in a number with a unit after it, as a message. A
/// browser can end a line after an equation's box even at a no-break space, so the unit goes
/// inside the equation, where no line ends (`= 4593\,\mathrm{ft}$`).
fn unit_after_equations(run: &HtmlRun) -> Vec<String> {
    let mut problems = Vec::new();
    for (end, number) in &run.equations {
        let rest = &run.text[*end..];
        let after = rest.trim_start();
        if after.len() == rest.len() {
            continue;
        }
        let at = run.text.len() - after.len();
        if let Some(unit_end) = unit_after(&run.text, at) {
            let unit = &run.text[at..unit_end];
            problems.push(format!(
                "an equation ending in `{number}` has its unit, `{unit}`, after it in \"{}\", where a \
                 line can break even at a no-break space: write the unit inside the equation, \
                 `{number}\\,\\mathrm{{{unit}}}$` (data.md, Numbers)",
                shown(&super::figures::context(&run.text, *end..unit_end))
            ));
        }
    }
    problems
}

/// The built pages [`join`] and [`check`] read: every HTML file but the API reference's.
fn pages(output: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    names.retain(|name| !name.starts_with(&format!("{}/", super::API)));
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pairs [`splits`] finds in `text`, as written.
    fn pairs(text: &str) -> Vec<&str> {
        splits(text)
            .into_iter()
            .map(|split| &text[split.pair])
            .collect()
    }

    #[test]
    fn a_figure_split_from_its_unit_is_found() {
        assert_eq!(
            pairs("A 175 m/s (574 ft/s) flight, 1,400 m up at 21.5 °C, for 3.0\ns."),
            ["175 m", "574 ft", "1,400 m", "21.5 °C", "3.0\ns"]
        );
        // Signs, exponents, superscripts, ranges, fractions and brackets.
        assert_eq!(
            pairs("−3.89 s, 1e-9 m/s, 10⁶ psi, 0.3–0.5 mm, 1/4 in, (0 s), 1.44954e-4 m²"),
            [
                "3.89 s",
                "1e-9 m",
                "10⁶ psi",
                "0.5 mm",
                "4 in",
                "0 s",
                "1.44954e-4 m"
            ]
        );
        // An inch with no word after it, and the project's units.
        assert_eq!(
            pairs(
                "3 mm (0.12 in) thick, 0.4 calibers, 2 cal, 960 px wide, 5 in., 1.52 in (38 mm), 4 in × 40"
            ),
            [
                "3 mm",
                "0.12 in",
                "0.4 calibers",
                "2 cal",
                "960 px",
                "5 in",
                "1.52 in",
                "38 mm",
                "4 in"
            ]
        );
    }

    /// The groups on either side of each breaking space [`splits`] finds inside a figure.
    fn inside(text: &str) -> Vec<&str> {
        splits(text)
            .into_iter()
            .filter(|split| split.inside)
            .map(|split| {
                let start = text[..split.space.start]
                    .rfind(|c: char| !c.is_ascii_digit())
                    .map_or(0, |at| at + 1);
                let end = split.space.end
                    + text[split.space.end..]
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(text.len() - split.space.end);
                &text[start..end]
            })
            .collect()
    }

    #[test]
    fn a_figure_written_in_groups_with_spaces_is_one_figure() {
        // #891's misreading: "22" ends a line and "699.96 Pa" starts the next.
        let text = "216.774 K and 22 699.96 Pa";
        let found = splits(text);
        assert_eq!(found.len(), 3, "{found:?}");
        assert_eq!(&text[found[1].pair.clone()], "22 699.96");
        assert!(found[1].inside && !found[2].inside);
        assert_eq!(&text[found[2].pair.clone()], "22 699.96 Pa");
        // Groups before and after the point, a line break between them, a figure with no unit,
        // and the no-break spaces, which aren't splits.
        assert_eq!(
            inside(
                "(20 001 samples), 9.806 65 m/s², 0.001\n953 125; 1 220 050.95, 101\u{a0}325 Pa"
            ),
            [
                "20 001", "806 65", "001\n953", "953 125", "1 220", "220 050"
            ]
        );
        // A split inside is the figure; one before the unit, the figure and its unit.
        assert_eq!(
            pairs("101 325\u{a0}Pa and 9.806 65 m/s²"),
            ["101 325", "9.806 65", "9.806 65 m"]
        );
    }

    #[test]
    fn figures_side_by_side_that_are_not_groups_stay_apart() {
        for text in [
            // A list of decimals, a four-digit group, a group of two, a short decimal, commas.
            "0.500 0.800 1.200, 12 2026, 3 45 m, 0.25 100, 1,400 500, 5 1000 m",
            // A group a comma and a digit follow.
            "3 100,5",
        ] {
            assert!(inside(text).is_empty(), "{text}: {:?}", inside(text));
        }
        // A short decimal group ends the figure: the figure after it is its own.
        assert_eq!(inside("0.001 95 125 m"), ["001 95"]);
        assert_eq!(pairs("0.001 95 125 m"), ["0.001 95", "125 m"]);
    }

    #[test]
    fn a_joined_pair_or_no_unit_passes() {
        for text in [
            "175\u{a0}m, 21.5\u{202f}°C, 4\u{2007}ft",
            // Not units: counts, words and the preposition.
            "12 Mach numbers, 4 of 170, 0.25 to 0.31, 20 seeds, 3 minutes",
            "0.5 in both, Mach 0.5 in sea-level air, 1.92 in The builder’s, 0 in 715, is 1 in ",
            // Not figures: names, versions, dates and codes.
            "ADR-208 s, Pro38 m, 0.1.0 s, 2026-10-04 m, x2 m, M1.6 in.",
            // No space, a unit run into a word, and degrees and percent, which take none.
            "45°, 7.0%, 5m, 3 msec, 3 meters, 15:30 UTC, CEC 2006 g06, 2 m_dot",
        ] {
            assert!(pairs(text).is_empty(), "{text}: {:?}", pairs(text));
        }
    }

    #[test]
    fn the_built_pages_are_joined_across_inline_tags_but_not_code_or_blocks() {
        let html = "<p><strong>324</strong> m (1,063&#32;ft), <em>5</em>\n  <span> kg</span></p>\
                    <p><code>5 m</code> <kbd>3 s</kbd> <math><mn>2</mn> <mi>m</mi></math></p>\
                    <p>12</p><p>m, a 4</p><div>in.</div>\
                    <script>let x = '5 m';</script><style>/* 5 m */</style><!-- 5 m -->\
                    <p title=\"5 m\">2.6&nbsp;in rocket</p><svg><text>5 m</text></svg>";
        let (joined, count) = joined_html(html);
        assert_eq!(
            count,
            Joined {
                units: 3,
                groups: 0
            },
            "{joined}"
        );
        assert_eq!(
            joined,
            html.replacen("</strong> m", "</strong>\u{a0}m", 1)
                .replacen("1,063&#32;ft", "1,063\u{a0}ft", 1)
                .replacen("</em>\n  <span> kg", "</em>\u{a0}<span>kg", 1)
        );
        assert!(
            page_splits(&joined).is_empty(),
            "{:?}",
            page_splits(&joined)
        );
    }

    #[test]
    fn a_unit_after_an_equation_that_ends_in_a_number_fails() {
        // Chrome can end a line between an equation's box and a no-break space after it, so an
        // inline equation ending in a number takes its unit inside it. One ending in a symbol,
        // display math, a block of its own, and a figure before an equation don't count, and
        // nothing is joined across an equation.
        let html = "<p>its altitude <math><mrow><mo>=</mo><mn>4593</mn></mrow></math> ft, \
                    <math><msup><mn>10</mn><mn>4</mn></msup></math> m, \
                    <math><mn>7</mn></math>&nbsp;kg, <math><mn>2</mn><mi>x</mi></math> s, \
                    <math><mi>v</mi></math> m, 5 <math><mi>x</mi></math> m</p>\
                    <p><math display=\"block\"><mn>5</mn></math> m</p>";
        let problems = page_splits(html);
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert!(
            problems[0].starts_with("an equation ending in `4593` has its unit, `ft`, after it")
                && problems[0].ends_with("`4593\\,\\mathrm{ft}$` (data.md, Numbers)"),
            "{}",
            problems[0]
        );
        assert!(problems[0].contains("[equation] ft"), "{}", problems[0]);
        assert!(problems[1].contains("ending in `4`") && problems[2].contains("`7`"));
        let thin = "<p>at <math><mn>3</mn></math>&thinsp;s</p>";
        assert_eq!(page_splits(thin).len(), 1, "{:?}", page_splits(thin));
        assert_eq!(joined_html(html), (html.to_owned(), Joined::default()));
        // The unit inside the equation passes.
        let inside = "<p>its altitude <math><mn>4593</mn><mspace width=\"0.1667em\"/>\
                      <mi mathvariant=\"normal\">ft</mi></math>, read back</p>";
        assert!(page_splits(inside).is_empty());
    }

    #[test]
    fn a_page_left_split_fails() {
        let problems = page_splits("<p>The rail is <b>3</b>\nm (9.8 ft) long.</p>");
        assert_eq!(problems.len(), 2, "{problems:?}");
        // Any breaking space, written as a character or an entity, a tab or a space in a tag.
        for page in [
            "<p>5\tm</p>",
            "<p>5\u{2009}m</p>",
            "<p>5&thinsp;m</p>",
            "<p>5&ensp;m</p>",
            "<p>5&#x20;m</p>",
            "<p>5<span> </span>m</p>",
            "<p>5\u{a0}\nm</p>",
        ] {
            assert_eq!(page_splits(page).len(), 1, "{page}");
        }
        // An upper-case end tag still ends a script, and `&` before a word isn't an entity.
        assert_eq!(page_splits("<SCRIPT>x</SCRIPT><p>R&D 5 m; ok</p>").len(), 1);
        // But not the no-break spaces, by character or entity.
        for page in ["<p>5&#160;m</p>", "<p>5&numsp;m</p>", "<p>5\u{202f}m</p>"] {
            assert!(page_splits(page).is_empty(), "{page}");
        }
        assert!(
            problems[0].contains("`3 m` in \"The rail is 3\nm (9.8 ft) long.\" can break"),
            "{}",
            problems[0]
        );
        assert!(problems[0].ends_with("`3\u{a0}m`"), "{}", problems[0]);
        // A figure's groups split, in a table cell, joined once with its unit.
        let cell = "<table><tr><td>22 632 Pa</td><td>101\n325</td></tr></table>";
        let problems = page_splits(cell);
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert!(
            problems[0].contains("`22 632` in \"22 632 Pa\" can break inside the number"),
            "{}",
            problems[0]
        );
        let (joined, count) = joined_html(cell);
        assert_eq!(
            count,
            Joined {
                units: 1,
                groups: 2
            }
        );
        assert_eq!(
            joined,
            cell.replace("22 632 Pa", "22\u{a0}632\u{a0}Pa")
                .replace("101\n", "101\u{a0}")
        );
        assert!(page_splits(&joined).is_empty());
        // Groups across an inline tag, an entity and a source line break inside a list item.
        for page in [
            "<p><b>22</b> 632 Pa</p>",
            "<li>101&#32;325</li>",
            "<p>0.001 953\n  125;</p>",
        ] {
            assert!(!page_splits(page).is_empty(), "{page}");
        }
    }

    #[test]
    fn a_readme_split_fails_with_its_line() {
        let readme = "# A README\n\nIt flies to 1,400 m\n(4,593&nbsp;ft) in `5 m` code.\n";
        let problems = readme_splits(readme, Renderer::GitHub);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert_eq!(problems[0].0, 3);
        assert!(
            problems[0].1.contains("`1,400 m`") && problems[0].1.ends_with("`1,400&nbsp;m`"),
            "{}",
            problems[0].1
        );
    }
}
