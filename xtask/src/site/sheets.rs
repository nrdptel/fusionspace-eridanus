//! The page's anatomy inside its frame (#384; the product system's `web.md`, *Page anatomy* and
//! *Accessibility*): every page opens with an intro, the designation tag over its title, and each
//! of its sections is a sheet, a 2 px ink rule on top and a rail with the sheet's number and name.
//!
//! After mdBook builds the site, [`draw`] rewrites each page's `main` (on the print page, each
//! chapter in it, from its title to the next):
//!
//! - the title, and what stands between it and the first `h2`, go in `<div class="fs-intro">`,
//!   which opens with the site's designation in the system's tag, `<p class="fs-tag">`;
//! - each `h2`, and what follows it up to the next `h2` or the chapter's end, goes in the
//!   system's sheet: `<section class="fs-sheet">`, then `<div class="fs-sheet-rail">` holding
//!   `<span class="fs-sheet-no">SHEET n / N</span>` and the `h2`, then `<div
//!   class="fs-sheet-body">` holding the rest.
//!
//! A title or an `h2` inside another element (a `div` a page writes in HTML, say) can't be wrapped
//! that way and fails the build, as does a page whose `main` doesn't open with its title.
//! [`check`] then fails a built page without the tag over every title, with an `h2` outside a
//! sheet or a sheet numbered out of order, or with a heading that skips a level going down (an
//! `h2` followed by an `h4`; `web.md` asks for headings in order). `theme/hpr.css` draws the
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
/// The intro, the tag and the sheet's parts, as [`draw`] writes them.
const INTRO: &str = "<div class=\"fs-intro\">";
const TAG: &str = "<p class=\"fs-tag\">";
const SHEET: &str = "<section class=\"fs-sheet\">";
const RAIL: &str = "<div class=\"fs-sheet-rail\">";
const NUMBER: &str = "<span class=\"fs-sheet-no\">";
const BODY: &str = "<div class=\"fs-sheet-body\">";
/// The sidebar alone, which mdBook loads in a frame for a reader without scripts: no page.
const SIDEBAR_FRAME: &str = "toc.html";

/// The elements HTML closes without an end tag, which [`balanced`] doesn't count.
const VOID: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr", "param",
];

/// The site's designation, as the tag shows it: the title block's entry, out of its code span.
fn designation() -> &'static str {
    DESIGNATION.trim_matches('`')
}

/// The guide's built pages under `output`: every HTML file but the API reference's and the
/// sidebar's frame.
fn guide_pages(output: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    names.retain(|name| !name.starts_with("api/") && name.as_str() != SIDEBAR_FRAME);
    Ok(names)
}

/// Draws the intro and the sheets on every page of the guide built under `output`. Returns how
/// many pages and sheets it drew, or every page's problems.
pub(super) fn draw(output: &Path) -> Result<(usize, usize), String> {
    let mut pages = 0;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in guide_pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        match rewrite(&html) {
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
             inside another element):\n  {}",
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
fn rewrite(html: &str) -> Result<(String, usize), Vec<String>> {
    let open = html
        .find(MAIN.0)
        .map(|at| at + MAIN.0.len())
        .ok_or_else(|| vec![format!("no `{}`", MAIN.0)])?;
    let close = html[open..]
        .find(MAIN.1)
        .map(|at| open + at)
        .ok_or_else(|| vec![format!("no `{}`", MAIN.1)])?;
    let (text, count) = drawn_main(&html[open..close])?;
    Ok((format!("{}{text}{}", &html[..open], &html[close..]), count))
}

/// A page's text (inside its `main`) with each chapter's intro and sheets drawn.
fn drawn_main(text: &str) -> Result<(String, usize), Vec<String>> {
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
        match drawn_chapter(&text[start..end]) {
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

/// One chapter, from its title on, with its intro and sheets drawn.
fn drawn_chapter(chapter: &str) -> Result<(String, usize), String> {
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
    out.push_str(INTRO);
    out.push_str(TAG);
    out.push_str(designation());
    out.push_str("</p>");
    out.push_str(&chapter[..title_end]);
    out.push_str(&chapter[title_end..intro_end]);
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

/// Checks the guide's built pages under `output`: each has the intro and sheets [`draw`] gives
/// it, and its headings go in order.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    for name in guide_pages(output)? {
        let path = output.join(&name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        problems.extend(
            page_problems(&html)
                .into_iter()
                .map(|why| format!("{name}: {why}")),
        );
    }
    Ok(problems)
}

/// What one built page lacks of its intro and sheets, or the headings out of order in its `main`.
fn page_problems(html: &str) -> Vec<String> {
    let Some(open) = html.find(MAIN.0) else {
        return vec![format!("no `{}`", MAIN.0)];
    };
    let Some(close) = html[open..].find(MAIN.1).map(|at| open + at) else {
        return vec![format!("no `{}`", MAIN.1)];
    };
    let text = &html[open..close];
    let mut problems = Vec::new();
    let above = format!("{INTRO}{TAG}{}</p>", designation());
    let titles = starts(text, "h1");
    if titles.is_empty() {
        problems.push("no title (`h1`)".to_string());
    }
    for &title in &titles {
        if !text[..title].ends_with(&above) {
            problems.push(format!(
                "a title without the designation tag right above it, in the intro (`{above}`)"
            ));
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

    #[test]
    fn a_page_gets_its_intro_and_numbered_sheets() {
        let text = format!(
            "{}\n<p>Opening.</p>\n{}\n<p>One.</p>\n{}\n<p>A.</p>\n{}\n<p>Two.</p>",
            h(1, "t"),
            h(2, "one"),
            h(3, "a"),
            h(2, "two")
        );
        let (drawn, count) = rewrite(&page(&text)).unwrap();
        assert_eq!(count, 2);
        assert!(drawn.contains(&format!(
            "{INTRO}{TAG}FS-ACHERNAR · SW · TOOL 001</p>{}\n<p>Opening.</p>\n</div>",
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
        assert_eq!(page_problems(&drawn), Vec::<String>::new());
        // Drawn once only: a page drawn already has text before its title.
        assert!(rewrite(&drawn).unwrap_err()[0].contains("text before"));
    }

    #[test]
    fn a_page_without_sections_gets_its_intro_alone() {
        let (drawn, count) = rewrite(&page(&format!("{}\n<p>Text.</p>", h(1, "t")))).unwrap();
        assert_eq!(count, 0);
        assert!(!drawn.contains(SHEET));
        assert_eq!(page_problems(&drawn), Vec::<String>::new());
    }

    #[test]
    fn the_print_page_numbers_each_chapter_and_keeps_its_breaks_between() {
        let text = format!(
            "{}\n{}\n<p>A.</p>\n{CHAPTER_BREAK}\n{}\n{}\n<p>B.</p>\n{}\n<p>C.</p>",
            h(1, "first"),
            h(2, "a"),
            h(1, "second"),
            h(2, "b"),
            h(2, "c")
        );
        let (drawn, count) = rewrite(&page(&text)).unwrap();
        assert_eq!(count, 3);
        assert!(drawn.contains(&format!("SHEET 1 / 1</span>{}", h(2, "a"))));
        assert!(drawn.contains(&format!("SHEET 1 / 2</span>{}", h(2, "b"))));
        assert!(drawn.contains(&format!("SHEET 2 / 2</span>{}", h(2, "c"))));
        // The break stands between the chapters, outside the first one's last sheet.
        assert!(drawn.contains(&format!("</div></section>\n{CHAPTER_BREAK}\n{INTRO}{TAG}")));
        assert_eq!(page_problems(&drawn), Vec::<String>::new());
    }

    #[test]
    fn a_heading_inside_another_element_fails_the_build() {
        let text = format!("{}\n<div>\n{}\n<p>A.</p>\n</div>", h(1, "t"), h(2, "a"));
        let why = rewrite(&page(&text)).unwrap_err();
        assert!(why[0].contains("can't be a sheet"), "{why:?}");
        let text = format!("<div>{}</div>", h(1, "t"));
        assert!(rewrite(&page(&text)).unwrap_err()[0].contains("text before"));
        assert!(rewrite(&page("<p>No title.</p>")).unwrap_err()[0].contains("no title"));
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
            "{}\n{}\n<p>A.</p>\n{}\n<p>B.</p>",
            h(1, "t"),
            h(2, "a"),
            h(2, "b")
        );
        let (drawn, _) = rewrite(&page(&text)).unwrap();
        assert_eq!(page_problems(&drawn), Vec::<String>::new());
        let fails = |broken: String, words: &str| {
            let problems = page_problems(&broken);
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
