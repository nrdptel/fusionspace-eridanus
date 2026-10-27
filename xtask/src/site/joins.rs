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
//! - **A unit** is the symbol of one of [`UNITS`], the units `data.md` writes and the project's
//!   own (`cal`, `calibers`, `px`), as a whole word after the figure. `in` is an inch only where
//!   no word or figure follows it (`(0.12 in)`, `0.12 in.`, `1.52 in (38 mm)`, `4 in × 40`): with
//!   one after it, or the end of the run after a space, it is also the preposition (`0.5 in
//!   both`, `1 in <code>SI</code>`), so a page joins an inch there by hand
//!   (`2.6&nbsp;in rocket`). Units spelled out (`3 minutes`) aren't read.
//! - **A breaking space** is any white space but the no-break spaces U+00A0, U+2007 and U+202F,
//!   a line break in the source included.
//!
//! The site's pages are joined in the built HTML, not in their source ([`join`]): after mdBook
//! writes the search index, which keeps the plain space, so search reads the same. It reads the
//! text of each page as the browser draws it, across inline tags (`<strong>5</strong> m` is one
//! run) but not across blocks, and never in code, preformatted text, keyboard or sample text,
//! math, SVG, scripts or styles. [`check`] then fails any pair still split in the built pages.
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
    "kg", "g", "mg", "lb", "oz", "N", "kN", "lbf", "Ns",
    // Pressure.
    "Pa", "hPa", "kPa", "MPa", "GPa", "psi", "ksi", "mbar", "bar",
    // Temperature and angle.
    "K", "°C", "°F", "rad", "deg",
    // Energy, power and electricity.
    "J", "kJ", "W", "kW", "V", "mV", "mA", "mAh", "Ω",
    // Magnetic field, speed, data and sound.
    "nT", "µT", "μT", "mph", "kt", "kn", "kB", "MB", "GB", "KiB", "MiB", "GiB", "dB",
];

/// The spaces a line can't end at: no-break, figure and narrow no-break.
const NO_BREAK: [char; 3] = ['\u{a0}', '\u{2007}', '\u{202f}'];

/// A figure split from its unit by a breaking space, as byte ranges in the text read.
#[derive(Debug, PartialEq)]
pub(super) struct Split {
    /// The white space between them.
    pub(super) space: Range<usize>,
    /// The figure, the space and the unit.
    pub(super) pair: Range<usize>,
}

/// Each figure in `text` that a breaking space parts from its unit.
pub(super) fn splits(text: &str) -> Vec<Split> {
    let mut found = Vec::new();
    let mut at = 0;
    while let Some((offset, _)) = text[at..].char_indices().find(|(_, c)| c.is_ascii_digit()) {
        let start = at + offset;
        if !figure_starts(&text[..start]) {
            at = start + 1;
            continue;
        }
        let end = figure_end(text, start);
        at = end;
        let space_len = text[end..]
            .char_indices()
            .find(|(_, c)| !c.is_whitespace())
            .map_or(text.len() - end, |(len, _)| len);
        let space = end..end + space_len;
        if space.is_empty() || !text[space.clone()].chars().any(|c| !NO_BREAK.contains(&c)) {
            continue;
        }
        if let Some(unit_end) = unit_after(text, space.end) {
            found.push(Split {
                space,
                pair: start..unit_end,
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

/// The end of the figure that starts at `start`: its digits and grouping commas, its decimals,
/// an exponent and superscript digits.
fn figure_end(text: &str, start: usize) -> usize {
    let digits = |from: usize| {
        from + text[from..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(text.len() - from)
    };
    let mut end = digits(start);
    // Grouping commas, each before three digits.
    while text[end..].starts_with(',') && digits(end + 1) == end + 4 {
        end += 4;
    }
    if text[end..].starts_with('.') && digits(end + 1) > end + 1 {
        end = digits(end + 1);
    }
    // An exponent, `e` and a signed whole number, but not a word that opens with `e`.
    let rest = &text[end..];
    if let Some(sign) = rest.strip_prefix(['e', 'E']) {
        let unsigned = sign.trim_start_matches(['-', '+', '−']);
        if unsigned.starts_with(|c: char| c.is_ascii_digit()) {
            end = digits(end + (rest.len() - unsigned.len()));
        }
    }
    end + text[end..]
        .find(|c: char| !"⁻⁰¹²³⁴⁵⁶⁷⁸⁹".contains(c))
        .unwrap_or(text.len() - end)
}

/// The end of the unit symbol at `at`, if one of [`UNITS`] stands there as a whole word.
fn unit_after(text: &str, at: usize) -> Option<usize> {
    let rest = &text[at..];
    let len = rest
        .char_indices()
        .find(|&(_, c)| !(c.is_alphabetic() || c == '°'))
        .map_or(rest.len(), |(len, _)| len);
    let unit = &rest[..len];
    if !UNITS.contains(&unit) {
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
    format!(
        "`{}` in \"{}\" can break between the figure and its unit (data.md, Numbers: a \
         non-breaking space, so a line never ends between them)",
        text[split.pair.clone()].split_whitespace().collect::<Vec<_>>().join(" "),
        super::figures::context(text, split.pair.clone())
    )
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
/// sample text, math, SVG, scripts and styles.
const UNREAD: [&str; 9] = ["code", "pre", "kbd", "samp", "math", "svg", "script", "style", "textarea"];

/// The elements that continue a run of text, as the browser draws them on one line.
const INLINE: [&str; 24] = [
    "a", "abbr", "b", "bdi", "bdo", "cite", "data", "del", "dfn", "em", "i", "ins", "label", "mark",
    "q", "s", "small", "span", "strong", "sub", "sup", "time", "u", "var",
];

/// A run of a page's text, with where each of its characters came from in the HTML.
#[derive(Default)]
struct HtmlRun {
    /// The text, entities decoded.
    text: String,
    /// For each character's byte offset in [`HtmlRun::text`], its byte range in the HTML.
    from: Vec<(usize, Range<usize>)>,
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
                at += html[at..].find(&close).map_or(html.len() - at, |end| {
                    end + html[at + end..].find('>').map_or(close.len(), |gt| gt + 1)
                });
                flush(&mut run, &mut runs);
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
        }
        at += len;
    }
    flush(&mut run, &mut runs);
    runs
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
    if let Some(entity) = html.strip_prefix('&') {
        if let Some(end) = entity.find(';').filter(|&end| end <= 10) {
            let name = &entity[..end];
            let decoded = match name {
                "nbsp" => Some('\u{a0}'),
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
    }
    // A slice of a `&str` that isn't empty starts with a character.
    let c = html.chars().next().unwrap_or('\u{fffd}');
    (c, c.len_utf8())
}

/// `html` with each figure joined to its unit by a no-break space, and how many it joined.
fn joined_html(html: &str) -> (String, usize) {
    // Each split's first space becomes a no-break space and the rest go, so the pair keeps one
    // space and any tag between them.
    let mut count = 0;
    let mut edits: Vec<(Range<usize>, &str)> = Vec::new();
    for run in html_runs(html) {
        for split in splits(&run.text) {
            count += 1;
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
/// the API reference, after mdBook has written the search index; how many it joined.
pub(super) fn join(output: &Path) -> Result<usize, String> {
    let mut count = 0;
    for name in pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        let (drawn, joined) = joined_html(&html);
        if joined > 0 {
            fs::write(&path, drawn)
                .map_err(|err| format!("could not write {}: {err}", path.display()))?;
        }
        count += joined;
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
        problems.extend(page_splits(&html).into_iter().map(|why| format!("{name}: {why}")));
    }
    Ok(problems)
}

/// What [`check`] says of one built page.
fn page_splits(html: &str) -> Vec<String> {
    html_runs(html)
        .iter()
        .flat_map(|run| {
            splits(&run.text).into_iter().map(|split| {
                format!(
                    "{}, which `join` (xtask/src/site/joins.rs) should have joined: `{}`",
                    message(&run.text, &split),
                    joined(&run.text, &split, "\u{a0}")
                )
            })
        })
        .collect()
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
        splits(text).into_iter().map(|split| &text[split.pair]).collect()
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
            ["3.89 s", "1e-9 m", "10⁶ psi", "0.5 mm", "4 in", "0 s", "1.44954e-4 m"]
        );
        // An inch with no word after it, and the project's units.
        assert_eq!(
            pairs("3 mm (0.12 in) thick, 0.4 calibers, 2 cal, 960 px wide, 5 in., 1.52 in (38 mm)"),
            ["3 mm", "0.12 in", "0.4 calibers", "2 cal", "960 px", "5 in", "1.52 in", "38 mm"]
        );
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
            "45°, 7.0%, 5m, 3 msec, 3 meters, 15:30 UTC",
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
        assert_eq!(count, 3, "{joined}");
        assert_eq!(
            joined,
            html.replacen("</strong> m", "</strong>\u{a0}m", 1)
                .replacen("1,063&#32;ft", "1,063\u{a0}ft", 1)
                .replacen("</em>\n  <span> kg", "</em>\u{a0}<span>kg", 1)
        );
        assert!(page_splits(&joined).is_empty(), "{:?}", page_splits(&joined));
    }

    #[test]
    fn a_page_left_split_fails() {
        let problems = page_splits("<p>The rail is <b>3</b>\nm (9.8 ft) long.</p>");
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(
            problems[0].contains("`3 m` in \"The rail is 3\nm (9.8 ft) long.\" can break"),
            "{}",
            problems[0]
        );
        assert!(problems[0].ends_with("`3\u{a0}m`"), "{}", problems[0]);
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
