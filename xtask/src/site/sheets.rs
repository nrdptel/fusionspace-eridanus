//! The page's anatomy inside its frame (#384; the product system's `web.md`, *Page anatomy* and
//! *Accessibility*): every page opens with an intro, the designation tag and the status chip over
//! its title and one sentence in `lead` under it, and each of its sections is a sheet, a 2 px ink
//! rule on top and a rail with the sheet's number and name.
//!
//! After mdBook builds the site, [`draw`] rewrites each page's `main` (on the print page, each
//! chapter in it, from its title to the next):
//!
//! - the title, and what stands between it and the first `h2`, go in `<div class="fs-intro">`,
//!   which opens with a row, `<div class="fs-intro-row">`, of the site's designation in the
//!   system's tag, `<p class="fs-tag">`, and the site's status in the system's chip, `<p
//!   class="fs-status">` with its state, icon and word (the title block's *Status* entry, one of
//!   the system's drawing words, `principles.md`: RELEASED or IN PREPARATION);
//! - the paragraph right under the title becomes the lead, `<p class="fs-lead">`;
//! - each `h2`, and what follows it up to the next `h2` or the chapter's end, goes in the
//!   system's sheet: `<section class="fs-sheet">`, then `<div class="fs-sheet-rail">` holding
//!   `<span class="fs-sheet-no">SHEET n / N</span>` and the `h2`, then `<div
//!   class="fs-sheet-body">` holding the rest.
//!
//! A title or an `h2` inside another element (a `div` a page writes in HTML, say) can't be wrapped
//! that way and fails the build, as does a page whose `main` doesn't open with its title or whose
//! title isn't followed by a paragraph. [`check`] then fails a built page without the tag and the
//! chip over every title, without a lead under it, with a lead of more than one sentence or more
//! than [`LEAD_WORDS`] words, with an `h2` outside a sheet or a sheet numbered out of order, or
//! with a heading that skips a level going down (an `h2` followed by an `h4`; `web.md` asks for
//! headings in order). `theme/hpr.css` draws the
//! intro and the sheets, and the page check fails either drawn off the system's (`pages.rs`,
//! `Kind::Sheet`).

use std::fs;
use std::path::Path;

use super::html_files;
use super::theme::DESIGNATION;

/// How mdBook 0.5 opens and closes a page's text.
const MAIN: (&str, &str) = ("<main>", "</main>");
/// What mdBook 0.5's print page puts between two chapters, just before the second's title.
const CHAPTER_BREAK: &str = "<div style=\"break-before: page; page-break-before: always;\"></div>";
/// The intro, its first row, the tag, the chip, the lead and the sheet's parts, as [`draw`]
/// writes them.
const INTRO: &str = "<div class=\"fs-intro\">";
const ROW: &str = "<div class=\"fs-intro-row\">";
const TAG: &str = "<p class=\"fs-tag\">";
const CHIP: &str = "<p class=\"fs-status\" data-state=\"";
const LEAD: &str = "<p class=\"fs-lead\">";
const SHEET: &str = "<section class=\"fs-sheet\">";
const RAIL: &str = "<div class=\"fs-sheet-rail\">";
const NUMBER: &str = "<span class=\"fs-sheet-no\">";
const BODY: &str = "<div class=\"fs-sheet-body\">";
/// How mdBook 0.5 opens a paragraph, which [`draw`] makes the lead when it follows a title.
const PARAGRAPH: &str = "<p>";
/// The sidebar alone, which mdBook loads in a frame for a reader without scripts: no page.
const SIDEBAR_FRAME: &str = "toc.html";
/// The most words a lead may have. `web.md` asks for one sentence; `docs/writing.md`, rule 2,
/// flags a sentence over 25 words for a second look, and the lead, read first on every page, is
/// held to it.
pub(super) const LEAD_WORDS: usize = 25;
/// The site's statuses, the title block's *Status* entry before its colon (`theme.rs`,
/// `STATUSES`), each with its chip's state, icon and word as [`draw`] writes them. A release is
/// `ok`, whose icon is the system's circle for normal (`foundations.md`, *Icons*: status icons
/// differ in shape); work in preparation is `info`, the square for information. The chip sets
/// its word in capitals.
const CHIPS: [(&str, &str, &str, &str); 2] = [
    (
        "RELEASED",
        "ok",
        include_str!("../../../theme/icons/ok.svg"),
        "Released",
    ),
    (
        "IN PREPARATION",
        "info",
        include_str!("../../../theme/icons/info.svg"),
        "In preparation",
    ),
];

/// The elements HTML closes without an end tag, which [`balanced`] doesn't count.
const VOID: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr", "param",
];

/// The site's designation, as the tag shows it: the title block's entry, out of its code span.
fn designation() -> &'static str {
    DESIGNATION.trim_matches('`')
}

/// The intro's opening as [`draw`] writes it before every title: the intro, and its row of the
/// designation tag and the chip of the site's `status`, or why `status` has no chip.
pub(super) fn intro_open(status: &str) -> Result<String, String> {
    let (_, state, icon, word) = CHIPS
        .iter()
        .find(|(name, ..)| *name == status)
        .ok_or_else(|| {
            format!(
                "the title block's status `{status}` has no chip; the chips are {:?}",
                CHIPS.map(|(name, ..)| name)
            )
        })?;
    Ok(format!(
        "{INTRO}{ROW}{TAG}{}</p>{CHIP}{state}\">{}{word}</p></div>",
        designation(),
        icon.trim_end()
    ))
}

/// The guide's built pages under `output`: every HTML file but the API reference's and the
/// sidebar's frame.
fn guide_pages(output: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    names.retain(|name| !name.starts_with("api/") && name.as_str() != SIDEBAR_FRAME);
    Ok(names)
}

/// Draws the intro, its chip of the site's `status`, and the sheets on every page of the guide
/// built under `output`. Returns how many pages and sheets it drew, or every page's problems.
pub(super) fn draw(output: &Path, status: &str) -> Result<(usize, usize), String> {
    let open = intro_open(status)?;
    let mut pages = 0;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in guide_pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        match rewrite(&html, &open) {
            Ok((drawn, sheets)) => {
                fs::write(&path, drawn)
                    .map_err(|err| format!("could not write {}: {err}", path.display()))?;
                pages += 1;
                count += sheets;
            }
            Err(why) => problems.extend(why.into_iter().map(|why| format!("{name}: {why}"))),
        }
    }
    if problems.is_empty() {
        Ok((pages, count))
    } else {
        Err(format!(
            "{} part(s) of a page that can't be drawn as the system's intro and sheets \
             (xtask/src/site/sheets.rs; a title and each `h2` stand in the page's text, not \
             inside another element, and a paragraph, the lead, follows each title):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// The byte offsets of each `<tag` element's start in `html` (`<h2 ` or `<h2>`, not `<h2x`).
fn starts(html: &str, tag: &str) -> Vec<usize> {
    let open = format!("<{tag}");
    html.match_indices(&open)
        .map(|(at, _)| at)
        .filter(|&at| html[at + open.len()..].starts_with([' ', '>']))
        .collect()
}

/// Where the element opening at `html`'s start ends, after its `</tag>`, if it does.
fn element_end(html: &str, tag: &str) -> Option<usize> {
    let close = format!("</{tag}>");
    html.find(&close).map(|at| at + close.len())
}

/// Whether `html` closes every element it opens, and opens every element it closes, so it can be
/// wrapped in an element of its own. Comments, scripts and styles are skipped; void elements and
/// those closed with `/>` (an SVG's paths) count as nothing.
fn balanced(html: &str) -> bool {
    let mut depth: i64 = 0;
    let mut rest = html;
    while let Some(at) = rest.find('<') {
        rest = &rest[at..];
        if let Some(after) = rest.strip_prefix("<!--") {
            let Some(end) = after.find("-->") else {
                return false;
            };
            rest = &after[end + 3..];
            continue;
        }
        let Some(end) = rest.find('>') else {
            return false;
        };
        let tag = &rest[1..end];
        rest = &rest[end + 1..];
        if let Some(name) = tag.strip_prefix('/') {
            if !VOID.contains(&name.trim()) {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            continue;
        }
        let name: String = tag
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
            .to_ascii_lowercase();
        if name.is_empty() || tag.starts_with('!') || tag.ends_with('/') {
            continue;
        }
        if VOID.contains(&name.as_str()) {
            continue;
        }
        if name == "script" || name == "style" {
            let close = format!("</{name}>");
            let Some(end) = rest.find(&close) else {
                return false;
            };
            rest = &rest[end + close.len()..];
            continue;
        }
        depth += 1;
    }
    depth == 0
}

/// One page's HTML with its intro and sheets drawn, and how many sheets; or why it can't be.
fn rewrite(html: &str, open: &str) -> Result<(String, usize), Vec<String>> {
    let start = html
        .find(MAIN.0)
        .map(|at| at + MAIN.0.len())
        .ok_or_else(|| vec![format!("no `{}`", MAIN.0)])?;
    let close = html[start..]
        .find(MAIN.1)
        .map(|at| start + at)
        .ok_or_else(|| vec![format!("no `{}`", MAIN.1)])?;
    let (text, count) = drawn_main(&html[start..close], open)?;
    Ok((format!("{}{text}{}", &html[..start], &html[close..]), count))
}

/// A page's text (inside its `main`) with each chapter's intro and sheets drawn.
fn drawn_main(text: &str, open: &str) -> Result<(String, usize), Vec<String>> {
    let titles = starts(text, "h1");
    let Some(&first) = titles.first() else {
        return Err(vec!["no title (`h1`) in the page's `main`".to_string()]);
    };
    if !text[..first].trim().is_empty() {
        return Err(vec![format!(
            "text before the page's title: `{}`",
            snippet(text[..first].trim())
        )]);
    }
    let mut out = String::with_capacity(text.len() + 4096);
    out.push_str(&text[..first]);
    let mut problems = Vec::new();
    let mut count = 0;
    for (i, &start) in titles.iter().enumerate() {
        // A chapter runs to the next title, less the print page's break before it.
        let end = titles.get(i + 1).map_or(text.len(), |&next| {
            let before = text[start..next].trim_end();
            if before.ends_with(CHAPTER_BREAK) {
                start + before.len() - CHAPTER_BREAK.len()
            } else {
                next
            }
        });
        match drawn_chapter(&text[start..end], open) {
            Ok((drawn, sheets)) => {
                out.push_str(&drawn);
                count += sheets;
            }
            Err(why) => problems.push(why),
        }
        let next = titles.get(i + 1).copied().unwrap_or(text.len());
        out.push_str(&text[end..next]);
    }
    if problems.is_empty() {
        Ok((out, count))
    } else {
        Err(problems)
    }
}

/// One chapter, from its title on, with its intro, opening with `open` ([`intro_open`]), its
/// lead and its sheets drawn.
fn drawn_chapter(chapter: &str, open: &str) -> Result<(String, usize), String> {
    let title_end = element_end(chapter, "h1").ok_or("a title (`h1`) never closed")?;
    let heads = starts(chapter, "h2");
    let mut cuts = vec![0];
    cuts.extend(&heads);
    cuts.push(chapter.len());
    for pair in cuts.windows(2) {
        let part = &chapter[pair[0]..pair[1]];
        if !balanced(part) {
            return Err(format!(
                "a section that opens or closes an element around a heading, so it can't be a \
                 sheet: `{}`",
                snippet(part)
            ));
        }
    }
    let total = heads.len();
    let mut out = String::with_capacity(chapter.len() + 256 * (total + 1));
    let intro_end = heads.first().copied().unwrap_or(chapter.len());
    out.push_str(open);
    out.push_str(&chapter[..title_end]);
    let after = &chapter[title_end..intro_end];
    let gap = after.len() - after.trim_start().len();
    let Some(rest) = after[gap..].strip_prefix(PARAGRAPH) else {
        return Err(format!(
            "a title not followed by a paragraph, its lead: `{}`",
            snippet(after.trim_start())
        ));
    };
    out.push_str(&after[..gap]);
    out.push_str(LEAD);
    out.push_str(rest);
    out.push_str("</div>\n");
    for (n, &start) in heads.iter().enumerate() {
        let end = heads.get(n + 1).copied().unwrap_or(chapter.len());
        let head_end = start
            + element_end(&chapter[start..], "h2")
                .ok_or_else(|| "a section's name (`h2`) never closed".to_string())?;
        out.push_str(&format!(
            "{SHEET}{RAIL}{NUMBER}SHEET {} / {total}</span>{}</div>{BODY}{}</div></section>\n",
            n + 1,
            &chapter[start..head_end],
            &chapter[head_end..end]
        ));
    }
    Ok((out, total))
}

/// The first 80 characters of `text`, for a message.
fn snippet(text: &str) -> &str {
    text.char_indices()
        .nth(80)
        .map_or(text, |(at, _)| &text[..at])
}

/// Checks the guide's built pages under `output`: each has the intro, with the chip of the site's
/// `status`, the lead and the sheets [`draw`] gives it, and its headings go in order.
pub(super) fn check(output: &Path, status: &str) -> Result<Vec<String>, String> {
    let open = intro_open(status)?;
    let mut problems = Vec::new();
    for name in guide_pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        problems.extend(
            page_problems(&html, &open)
                .into_iter()
                .map(|why| format!("{name}: {why}")),
        );
    }
    Ok(problems)
}

/// What one built page lacks of its intro, opening with `open` ([`intro_open`]), its lead and its
/// sheets, or the headings out of order in its `main`.
fn page_problems(html: &str, open: &str) -> Vec<String> {
    let Some(start) = html.find(MAIN.0) else {
        return vec![format!("no `{}`", MAIN.0)];
    };
    let Some(close) = html[start..].find(MAIN.1).map(|at| start + at) else {
        return vec![format!("no `{}`", MAIN.1)];
    };
    let text = &html[start..close];
    let mut problems = Vec::new();
    let titles = starts(text, "h1");
    if titles.is_empty() {
        problems.push("no title (`h1`)".to_string());
    }
    for &title in &titles {
        if !text[..title].ends_with(open) {
            problems.push(format!(
                "a title without the designation tag and the site's status chip right above it, \
                 in the intro's row (`{}`)",
                snippet(open)
            ));
        }
        let after = element_end(&text[title..], "h1").map_or("", |end| &text[title + end..]);
        match after
            .trim_start()
            .strip_prefix(LEAD)
            .and_then(|lead| lead.split_once("</p>"))
        {
            Some((lead, _)) => problems.extend(lead_problems(&super::visible_text(lead))),
            None => problems.push(format!(
                "a title without its lead (`{LEAD}`) right under it: `{}`",
                snippet(after.trim_start())
            )),
        }
    }
    // The sheets, numbered within each chapter.
    let mut chapters = titles.clone();
    chapters.push(text.len());
    for pair in chapters.windows(2) {
        let chapter = &text[pair[0]..pair[1]];
        let heads = starts(chapter, "h2");
        let total = heads.len();
        if chapter.matches(SHEET).count() != total {
            problems.push(format!(
                "{} sheets for {total} sections (`h2`)",
                chapter.matches(SHEET).count()
            ));
        }
        for (n, &head) in heads.iter().enumerate() {
            let rail = format!("{SHEET}{RAIL}{NUMBER}SHEET {} / {total}</span>", n + 1);
            if !chapter[..head].ends_with(&rail) {
                problems.push(format!(
                    "a section (`h2`) not opening sheet {} of {total}: `{}`",
                    n + 1,
                    snippet(&chapter[head..])
                ));
            }
        }
    }
    problems.extend(headings_out_of_order(text));
    problems
}

/// What is wrong with a lead whose text a reader sees is `text`: `web.md` asks for one sentence,
/// and [`LEAD_WORDS`] bounds it. A sentence ends at a period, question mark or exclamation mark
/// followed by a space, so a lead names no abbreviation with a period inside it ("e.g.", "U.S.").
fn lead_problems(text: &str) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let joined = words.join(" ");
    let mut problems = Vec::new();
    if words.len() > LEAD_WORDS {
        problems.push(format!(
            "a lead of {} words, more than {LEAD_WORDS}: `{}`",
            words.len(),
            snippet(&joined)
        ));
    }
    if !joined.ends_with('.') {
        problems.push(format!(
            "a lead that doesn't end its sentence with a period: `{}`",
            snippet(&joined)
        ));
    }
    if [". ", "? ", "! "].iter().any(|end| joined.contains(end)) {
        problems.push(format!(
            "a lead of more than one sentence: `{}`",
            snippet(&joined)
        ));
    }
    problems
}

/// Each heading in `text` that skips a level going down from the one before it (an `h4` after
/// an `h2`), or a first heading that isn't the title; `web.md`, *Accessibility*: "headings in
/// order". Going up any number of levels is in order.
fn headings_out_of_order(text: &str) -> Vec<String> {
    let mut found: Vec<(usize, u8)> = Vec::new();
    for level in 1..=6u8 {
        found.extend(
            starts(text, &format!("h{level}"))
                .into_iter()
                .map(|at| (at, level)),
        );
    }
    found.sort_unstable();
    let mut problems = Vec::new();
    let mut previous = 0u8;
    for (at, level) in found {
        if level > previous + 1 {
            let what = if previous == 0 {
                "the first heading".to_string()
            } else {
                format!("an `h{previous}`")
            };
            problems.push(format!(
                "an `h{level}` after {what}, skipping a level (headings go in order): `{}`",
                snippet(&text[at..])
            ));
        }
        previous = level;
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A built page around `text`, as mdBook writes one.
    fn page(text: &str) -> String {
        format!("<html><body><nav>x</nav><main>\n{text}\n</main><footer>y</footer></body></html>")
    }

    fn h(level: u8, id: &str) -> String {
        format!("<h{level} id=\"{id}\"><a class=\"header\" href=\"#{id}\">{id}</a></h{level}>")
    }

    /// The intro's opening for a site in preparation.
    fn open() -> String {
        intro_open("IN PREPARATION").unwrap()
    }

    #[test]
    fn a_page_gets_its_intro_and_numbered_sheets() {
        let text = format!(
            "{}\n<p>Opening.</p>\n{}\n<p>One.</p>\n{}\n<p>A.</p>\n{}\n<p>Two.</p>",
            h(1, "t"),
            h(2, "one"),
            h(3, "a"),
            h(2, "two")
        );
        let (drawn, count) = rewrite(&page(&text), &open()).unwrap();
        assert_eq!(count, 2);
        assert!(drawn.contains(&format!(
            "{INTRO}{ROW}{TAG}FS-ACHERNAR · SW · TOOL 001</p>{CHIP}info\"><svg "
        )));
        assert!(drawn.contains(&format!(
            "</svg>In preparation</p></div>{}\n{LEAD}Opening.</p>\n</div>",
            h(1, "t")
        )));
        assert!(drawn.contains(&format!(
            "{SHEET}{RAIL}{NUMBER}SHEET 1 / 2</span>{}</div>{BODY}\n<p>One.</p>\n{}\n<p>A.</p>\n</div></section>",
            h(2, "one"),
            h(3, "a")
        )));
        assert!(drawn.contains(&format!(
            "{SHEET}{RAIL}{NUMBER}SHEET 2 / 2</span>{}</div>{BODY}\n<p>Two.</p>\n</div></section>",
            h(2, "two")
        )));
        assert!(drawn.ends_with("</main><footer>y</footer></body></html>"));
        assert_eq!(page_problems(&drawn, &open()), Vec::<String>::new());
        // Drawn once only: a page drawn already has text before its title.
        assert!(rewrite(&drawn, &open()).unwrap_err()[0].contains("text before"));
    }

    #[test]
    fn a_page_without_sections_gets_its_intro_alone() {
        let (drawn, count) =
            rewrite(&page(&format!("{}\n<p>Text.</p>", h(1, "t"))), &open()).unwrap();
        assert_eq!(count, 0);
        assert!(!drawn.contains(SHEET));
        assert_eq!(page_problems(&drawn, &open()), Vec::<String>::new());
    }

    #[test]
    fn the_print_page_numbers_each_chapter_and_keeps_its_breaks_between() {
        let text = format!(
            "{}\n<p>First.</p>\n{}\n<p>A.</p>\n{CHAPTER_BREAK}\n{}\n<p>Second.</p>\n{}\n<p>B.</p>\n{}\n<p>C.</p>",
            h(1, "first"),
            h(2, "a"),
            h(1, "second"),
            h(2, "b"),
            h(2, "c")
        );
        let (drawn, count) = rewrite(&page(&text), &open()).unwrap();
        assert_eq!(count, 3);
        assert!(drawn.contains(&format!("SHEET 1 / 1</span>{}", h(2, "a"))));
        assert!(drawn.contains(&format!("SHEET 1 / 2</span>{}", h(2, "b"))));
        assert!(drawn.contains(&format!("SHEET 2 / 2</span>{}", h(2, "c"))));
        // The break stands between the chapters, outside the first one's last sheet.
        assert!(drawn.contains(&format!("</div></section>\n{CHAPTER_BREAK}\n{INTRO}{ROW}{TAG}")));
        assert!(drawn.contains(&format!("{}\n{LEAD}Second.</p>", h(1, "second"))));
        assert_eq!(page_problems(&drawn, &open()), Vec::<String>::new());
    }

    #[test]
    fn a_heading_inside_another_element_fails_the_build() {
        let text = format!(
            "{}\n<p>Lead.</p>\n<div>\n{}\n<p>A.</p>\n</div>",
            h(1, "t"),
            h(2, "a")
        );
        let why = rewrite(&page(&text), &open()).unwrap_err();
        assert!(why[0].contains("can't be a sheet"), "{why:?}");
        let text = format!("<div>{}</div>", h(1, "t"));
        assert!(rewrite(&page(&text), &open()).unwrap_err()[0].contains("text before"));
        assert!(
            rewrite(&page("<p>No title.</p>"), &open()).unwrap_err()[0].contains("no title")
        );
    }

    #[test]
    fn a_title_without_a_paragraph_under_it_fails_the_build() {
        for text in [
            format!("{}\n{}\n<p>A.</p>", h(1, "t"), h(2, "a")),
            format!("{}\n<ul><li>A.</li></ul>", h(1, "t")),
            h(1, "t"),
        ] {
            let why = rewrite(&page(&text), &open()).unwrap_err();
            assert!(why[0].contains("not followed by a paragraph"), "{why:?}");
        }
    }

    #[test]
    fn the_chip_names_each_status_and_no_other() {
        let released = intro_open("RELEASED").unwrap();
        assert!(released.contains(&format!("{CHIP}ok\"><svg ")));
        assert!(released.contains("<circle cx=\"12\" cy=\"12\" r=\"9\"/>"));
        assert!(released.ends_with("</svg>Released</p></div>"));
        let preparing = open();
        assert!(preparing.contains(&format!("{CHIP}info\"><svg ")));
        assert!(preparing.contains("<path d=\"M3 3 L21 3 L21 21 L3 21 Z\"/>"));
        assert!(preparing.ends_with("</svg>In preparation</p></div>"));
        // Every status the title block may give has a chip, and nothing else does.
        for status in super::super::theme::STATUSES {
            assert!(intro_open(status).is_ok(), "{status}");
        }
        assert_eq!(CHIPS.len(), super::super::theme::STATUSES.len());
        for other in ["WITHDRAWN", "Released", "", "IN PREPARATION: soon"] {
            let why = intro_open(other).unwrap_err();
            assert!(why.contains("has no chip"), "{other}: {why}");
        }
        // A missing site status fails the build before any page is drawn.
        let why = draw(Path::new("/nonexistent"), "WITHDRAWN").unwrap_err();
        assert!(why.contains("has no chip"), "{why}");
    }

    #[test]
    fn a_lead_is_one_sentence_of_at_most_25_words() {
        assert!(lead_problems("What this page covers, for whom, and why.").is_empty());
        assert!(lead_problems("Version 1.5 reads `a.eng` files, at 0.5 s steps.").is_empty());
        let words = |n: usize| format!("{}.", vec!["word"; n].join(" "));
        assert!(lead_problems(&words(LEAD_WORDS)).is_empty());
        let fails = |text: &str, words: &str| {
            let problems = lead_problems(text);
            assert!(
                problems.iter().any(|why| why.contains(words)),
                "{text}: {problems:?}"
            );
        };
        fails(&words(LEAD_WORDS + 1), "26 words, more than 25");
        fails("One sentence. And another.", "more than one sentence");
        fails("A question? And an answer.", "more than one sentence");
        fails("Stop! Then go.", "more than one sentence");
        fails("The U.S. Standard Atmosphere, by height.", "more than one sentence");
        fails("No period at its end", "with a period");
        fails("A question at its end?", "with a period");
        fails("", "with a period");
    }

    #[test]
    fn balance_skips_comments_scripts_and_void_elements() {
        assert!(balanced(
            "<p>a<br>b<img src=\"x\"></p><!-- <div> --><svg><path d=\"M0\"/></svg>"
        ));
        assert!(balanced(
            "<script>if (a < b) { x = '<div>'; }</script><p></p>"
        ));
        assert!(!balanced("<div><p></p>"));
        assert!(!balanced("</div><div>"));
    }

    #[test]
    fn the_check_fails_each_part_missing() {
        let text = format!(
            "{}\n<p>The lead.</p>\n{}\n<p>A.</p>\n{}\n<p>B.</p>",
            h(1, "t"),
            h(2, "a"),
            h(2, "b")
        );
        let (drawn, _) = rewrite(&page(&text), &open()).unwrap();
        assert_eq!(page_problems(&drawn, &open()), Vec::<String>::new());
        let fails = |broken: String, words: &str| {
            let problems = page_problems(&broken, &open());
            assert!(
                problems.iter().any(|why| why.contains(words)),
                "{words}: {problems:?}"
            );
        };
        fails(drawn.replace(TAG, "<p class=\"other\">"), "designation tag");
        fails(
            drawn.replace("FS-ACHERNAR · SW · TOOL 001", "FS · SW · TOOL 002"),
            "designation tag",
        );
        fails(
            drawn.replace("SHEET 2 / 2", "SHEET 3 / 2"),
            "not opening sheet 2 of 2",
        );
        fails(
            drawn.replace("SHEET 1 / 2", "SHEET 1 / 3"),
            "not opening sheet 1 of 2",
        );
        fails(
            drawn.replacen(SHEET, "<section class=\"other\">", 1),
            "1 sheets for 2 sections",
        );
        fails(page(&text), "designation tag");
        // The chip: gone, of another state, or not the site's status.
        fails(drawn.replace(CHIP, "<p class=\"other\" data-state=\""), "status chip");
        fails(drawn.replace("info\"><svg", "ok\"><svg"), "status chip");
        fails(drawn.replace("In preparation", "Released"), "status chip");
        let problems = page_problems(&drawn, &intro_open("RELEASED").unwrap());
        assert!(
            problems.iter().any(|why| why.contains("status chip")),
            "{problems:?}"
        );
        // The lead: gone, or more than one sentence.
        fails(drawn.replace(LEAD, "<p>"), "without its lead");
        fails(
            drawn.replace("The lead.", "The lead. A second sentence."),
            "more than one sentence",
        );
    }

    #[test]
    fn headings_skipping_a_level_going_down_are_out_of_order() {
        let order = |levels: &[u8]| {
            let text: String = levels
                .iter()
                .enumerate()
                .map(|(i, &l)| h(l, &format!("h{i}")))
                .collect();
            headings_out_of_order(&text).len()
        };
        assert_eq!(order(&[1, 2, 3, 4, 2, 3, 2]), 0);
        assert_eq!(order(&[1, 2, 3, 4, 5, 6, 2, 1, 2]), 0);
        assert_eq!(order(&[1, 3]), 1);
        assert_eq!(order(&[1, 2, 4]), 1);
        assert_eq!(order(&[1, 2, 3, 2, 4, 2, 5]), 2);
        assert_eq!(order(&[2]), 1);
        // `<h2x>` and a heading quoted in a code block (escaped) are not headings.
        assert!(headings_out_of_order("<h1>a</h1><h3x>b</h3x>&lt;h4&gt;").is_empty());
    }
}
