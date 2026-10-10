//! The site's notes (#384; the product system's `web.md`, *Components*): every quote block a page
//! renders is drawn as the system's note, a panel with its signal word in a header strip above
//! the message, as an ANSI Z535.4 safety label lays them out.
//!
//! A page writes a note in Markdown, which GitHub renders too, in one of two shapes:
//!
//! - **A trust note,** a quote block opening with `**How far to trust it.**` (`docs/writing.md`,
//!   *How far to trust it*; its shape is checked in the source by `trust_notes`). Its strip reads
//!   "How far to trust it", and the label leaves the message.
//! - **An alert,** a quote block opening with `[!NOTE]`, `[!CAUTION]` or `[!WARNING]` (GitHub's
//!   alerts, which mdBook 0.5 renders with its own title and icon). The system has no tip or
//!   "important" note, so `[!TIP]` and `[!IMPORTANT]` fail the build.
//!
//! After mdBook builds the site, [`draw`] rewrites each such quote block in the guide's pages as
//! the system's specimen writes a note, `<aside class="fs-note" data-kind="…">` with its head and
//! body, the icon the specimen gives each kind in the head. Any other quote block fails the
//! build: a page has no quote block that isn't a note. `theme/hpr.css` draws the notes, and the
//! page check fails one not drawn as the system draws it.

use std::fs;
use std::path::Path;

use super::html_files;

/// The words a trust note opens with, as mdBook renders them.
const TRUST_OPEN: &str = "<blockquote>\n<p><strong>How far to trust it.</strong>";

/// How mdBook 0.5 opens an alert, up to its kind.
const ALERT_OPEN: &str = "<blockquote class=\"blockquote-tag blockquote-tag-";

/// The end of an alert's title, after mdBook's icon and the kind's word.
const ALERT_TITLE_END: &str = "</p>\n";

/// The icons' shared attributes, from the system's specimen (`product/web/index.html`).
const ICON: &str = "<svg class=\"\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" \
                    stroke-width=\"1.5\" stroke-linecap=\"square\" stroke-linejoin=\"miter\" \
                    stroke-miterlimit=\"4\" aria-hidden=\"true\" focusable=\"false\">";

/// The kinds of note a page can write: mdBook's alert class (none for a trust note), the
/// system's `data-kind`, the icon's shapes and the signal word, each from the specimen.
const KINDS: [(&str, &str, &str, &str); 4] = [
    (
        "",
        "trust",
        "<path d=\"M4 5 L4 19\"/><path d=\"M20 5 L20 19\"/><path d=\"M6.5 12 L17.5 12\"/><path \
         fill=\"currentColor\" stroke=\"none\" d=\"M4.75 12 L8.5 10 L8.5 14 Z\"/><path \
         fill=\"currentColor\" stroke=\"none\" d=\"M19.25 12 L15.5 10 L15.5 14 Z\"/>",
        "How far to trust it",
    ),
    (
        "note",
        "note",
        "<path d=\"M3 3 L21 3 L21 21 L3 21 Z\"/><path d=\"M12 10.5 L12 17\"/><circle cx=\"12\" \
         cy=\"7\" r=\"1.25\" fill=\"currentColor\" stroke=\"none\"/>",
        "Note",
    ),
    (
        "caution",
        "caution",
        "<path d=\"M12 3 L21.5 20 L2.5 20 Z\"/><path d=\"M12 9 L12 14\"/><circle cx=\"12\" \
         cy=\"16.75\" r=\"1.25\" fill=\"currentColor\" stroke=\"none\"/>",
        "Caution",
    ),
    (
        "warning",
        "warning",
        "<path d=\"M12 3 L21.5 20 L2.5 20 Z\"/><path d=\"M12 9 L12 14\"/><circle cx=\"12\" \
         cy=\"16.75\" r=\"1.25\" fill=\"currentColor\" stroke=\"none\"/>",
        "Warning",
    ),
];

/// Draws every note in the guide's built pages under `output` (the API reference, under `api/`,
/// is rustdoc's) as the system's note. Returns how many it drew, or every page's problems: a
/// quote block that is not a note, or an alert of a kind the system doesn't have.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        if !html.contains("<blockquote") {
            continue;
        }
        match rewrite(&html) {
            Ok((drawn, notes)) => {
                fs::write(&path, drawn)
                    .map_err(|err| format!("could not write {}: {err}", path.display()))?;
                count += notes;
            }
            Err(why) => problems.extend(why.into_iter().map(|why| format!("{name}: {why}"))),
        }
    }
    if problems.is_empty() {
        Ok(count)
    } else {
        Err(format!(
            "{} quote block(s) that are not notes (a quote block on the site is a trust note, \
             opening with `**How far to trust it.**`, or an alert: `> [!NOTE]`, `> [!CAUTION]` \
             or `> [!WARNING]`):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// One page's HTML with each note drawn, and how many; or why a quote block is not a note.
fn rewrite(html: &str) -> Result<(String, usize), Vec<String>> {
    let mut out = String::with_capacity(html.len() + 2048);
    let mut problems = Vec::new();
    let mut count = 0;
    let mut rest = html;
    while let Some(at) = rest.find("<blockquote") {
        out.push_str(&rest[..at]);
        let quote = &rest[at..];
        let Some(end) = closing(quote) else {
            problems.push(format!("a quote block with no end: `{}`", lead(quote)));
            return Err(problems);
        };
        match note(&quote[..end]) {
            Ok((drawn, notes)) => {
                out.push_str(&drawn);
                count += notes;
            }
            Err(why) => {
                problems.extend(why);
                out.push_str(&quote[..end]);
            }
        }
        rest = &quote[end + "</blockquote>".len()..];
    }
    out.push_str(rest);
    if problems.is_empty() {
        Ok((out, count))
    } else {
        Err(problems)
    }
}

/// Where the `</blockquote>` that closes the quote block `quote` opens with starts, counting
/// quote blocks inside it.
fn closing(quote: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut at = 0;
    loop {
        let open = quote[at..].find("<blockquote").map(|i| at + i);
        let close = quote[at..].find("</blockquote>").map(|i| at + i)?;
        match open {
            Some(open) if open < close => {
                depth += 1;
                at = open + "<blockquote".len();
            }
            _ => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(close);
                }
                at = close + "</blockquote>".len();
            }
        }
    }
}

/// The first words of a quote block, for a message.
fn lead(quote: &str) -> String {
    quote
        .chars()
        .take(80)
        .collect::<String>()
        .replace('\n', " ")
}

/// The note drawn from one quote block, `<blockquote` to just before its `</blockquote>`, with
/// any quote block inside it drawn too, and how many notes that makes; or why it, or one inside
/// it, is not a note.
fn note(quote: &str) -> Result<(String, usize), Vec<String>> {
    let (kind, body) = if let Some(after) = quote.strip_prefix(TRUST_OPEN) {
        // The label leaves the message; a label alone in its paragraph leaves no paragraph.
        let after = after.trim_start();
        let body = match after.strip_prefix("</p>") {
            Some(next) => next.trim_start_matches('\n').to_owned(),
            None => format!("<p>{after}"),
        };
        (KINDS[0], body)
    } else if let Some(after) = quote.strip_prefix(ALERT_OPEN) {
        let class_end = after
            .find('"')
            .ok_or_else(|| vec![format!("an alert with no kind: `{}`", lead(quote))])?;
        let class = &after[..class_end];
        let kind = KINDS
            .iter()
            .find(|kind| !kind.0.is_empty() && kind.0 == class)
            .ok_or_else(|| {
                vec![format!(
                    "a `[!{}]` alert: the system's notes are NOTE, CAUTION and WARNING",
                    class.to_uppercase()
                )]
            })?;
        let title = after
            .find("<p class=\"blockquote-tag-title\">")
            .ok_or_else(|| vec![format!("an alert with no title: `{}`", lead(quote))])?;
        let title_end = after[title..].find(ALERT_TITLE_END).ok_or_else(|| {
            vec![format!(
                "an alert whose title has no end: `{}`",
                lead(quote)
            )]
        })?;
        (
            *kind,
            after[title + title_end + ALERT_TITLE_END.len()..].to_owned(),
        )
    } else {
        return Err(vec![format!("`{}`", lead(quote))]);
    };
    let (body, inside) = rewrite(&body)?;
    let (_, data_kind, shapes, word) = kind;
    Ok((
        format!(
            "<aside class=\"fs-note\" data-kind=\"{data_kind}\"><div class=\"fs-note-head\">\
             {ICON}{shapes}</svg>{word}</div><div class=\"fs-note-body\">\n{body}</div></aside>"
        ),
        1 + inside,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A page's `main`, as mdBook 0.5.4 renders the Markdown in each test (checked by building
    /// it), around `inside`.
    fn page(inside: &str) -> String {
        format!("<main>\n<h1>A</h1>\n{inside}\n</main>\n")
    }

    #[test]
    fn a_trust_note_loses_its_label_to_the_strip() {
        // > **How far to trust it.** A simulation,
        // > not validated.
        let (html, count) = rewrite(&page(
            "<blockquote>\n<p><strong>How far to trust it.</strong> A simulation,\nnot \
             validated.</p>\n</blockquote>",
        ))
        .unwrap();
        assert_eq!(count, 1);
        assert!(html.contains(
            "<aside class=\"fs-note\" data-kind=\"trust\"><div class=\"fs-note-head\"><svg"
        ));
        assert!(html.contains(
            "</svg>How far to trust it</div><div class=\"fs-note-body\">\n<p>A simulation,\nnot \
             validated.</p>\n</div></aside>"
        ));
        assert!(!html.contains("blockquote"));
        assert!(!html.contains("<strong>How far"));
    }

    #[test]
    fn a_label_alone_leaves_no_empty_paragraph() {
        // > **How far to trust it.**
        // >
        // > - A list.
        let (html, _) = rewrite(&page(
            "<blockquote>\n<p><strong>How far to trust it.</strong></p>\n<ul>\n<li>A \
             list.</li>\n</ul>\n</blockquote>",
        ))
        .unwrap();
        assert!(html.contains("<div class=\"fs-note-body\">\n<ul>\n<li>A list.</li>"));
        assert!(!html.contains("<p></p>"));
    }

    #[test]
    fn alerts_take_the_systems_kinds() {
        for (word, kind, signal) in [
            ("note", "note", "Note"),
            ("caution", "caution", "Caution"),
            ("warning", "warning", "Warning"),
        ] {
            let (html, count) = rewrite(&page(&format!(
                "<blockquote class=\"blockquote-tag blockquote-tag-{word}\">\n<p \
                 class=\"blockquote-tag-title\"><svg viewbox=\"0 0 16 16\" width=\"18\" \
                 height=\"18\"><path d=\"M0 8Z\"></path></svg>{signal}</p>\n<p>Text \
                 <strong>b</strong>.</p>\n<p>Two.</p>\n</blockquote>"
            )))
            .unwrap();
            assert_eq!(count, 1);
            assert!(html.contains(&format!(
                "<aside class=\"fs-note\" data-kind=\"{kind}\"><div class=\"fs-note-head\"><svg \
                 class=\"\" viewBox=\"0 0 24 24\""
            )));
            assert!(html.contains(&format!(
                "</svg>{signal}</div><div class=\"fs-note-body\">\n<p>Text \
                 <strong>b</strong>.</p>\n<p>Two.</p>\n</div></aside>"
            )));
            // mdBook's own icon and title are gone.
            assert!(!html.contains("blockquote-tag"), "{html}");
            assert!(!html.contains("viewbox=\"0 0 16 16\""));
        }
    }

    #[test]
    fn kinds_the_system_lacks_fail() {
        for word in ["tip", "important"] {
            let why = rewrite(&page(&format!(
                "<blockquote class=\"blockquote-tag blockquote-tag-{word}\">\n<p \
                 class=\"blockquote-tag-title\"><svg></svg>X</p>\n<p>T.</p>\n</blockquote>"
            )))
            .unwrap_err();
            assert_eq!(why.len(), 1);
            assert!(
                why[0].starts_with(&format!("a `[!{}]` alert", word.to_uppercase())),
                "{why:?}"
            );
        }
    }

    #[test]
    fn a_quote_block_that_is_no_note_fails() {
        for quote in [
            // > A quotation.
            "<blockquote>\n<p>A quotation.</p>\n</blockquote>",
            // > **Every number is an estimate.** Bold, but not the label.
            "<blockquote>\n<p><strong>Every number is an estimate.</strong> Bold.</p>\n\
             </blockquote>",
            // > Text, then **How far to trust it.** later.
            "<blockquote>\n<p>Text, then <strong>How far to trust it.</strong> \
             later.</p>\n</blockquote>",
        ] {
            let why = rewrite(&page(quote)).unwrap_err();
            assert_eq!(why.len(), 1, "{quote}");
        }
    }

    #[test]
    fn a_note_holding_a_quote_block_closes_at_its_own_end() {
        // A trust note holding a quotation fails for the quotation alone, and is not cut short
        // at the inner quote's end.
        let html = page(
            "<blockquote>\n<p><strong>How far to trust it.</strong> Checked.</p>\n<blockquote>\n\
             <p>Inner.</p>\n</blockquote>\n<p>After.</p>\n</blockquote>\n<p>Outside.</p>",
        );
        let quote = &html[html.find("<blockquote").unwrap()..];
        let end = closing(quote).unwrap();
        assert!(
            quote[..end].ends_with("<p>After.</p>\n"),
            "{}",
            &quote[..end]
        );
        let why = rewrite(&html).unwrap_err();
        assert_eq!(why, ["`<blockquote> <p>Inner.</p> `"]);
        // A note inside a note is drawn too.
        let (drawn, count) = rewrite(&html.replace(
            "<p>Inner.</p>",
            "<p><strong>How far to trust it.</strong> Inner.</p>",
        ))
        .unwrap();
        assert_eq!(count, 2);
        assert!(drawn.contains("<p>After.</p>\n</div></aside>\n<p>Outside.</p>"));
        assert!(!drawn.contains("blockquote"));
    }

    #[test]
    fn notes_side_by_side_and_text_between_survive() {
        let (html, count) = rewrite(&page(
            "<p>Before.</p>\n<blockquote>\n<p><strong>How far to trust it.</strong> \
             One.</p>\n</blockquote>\n<p>Between.</p>\n<ul>\n<li>\n<blockquote>\n<p><strong>How \
             far to trust it.</strong> Two.</p>\n</blockquote>\n</li>\n</ul>\n<p>After.</p>",
        ))
        .unwrap();
        assert_eq!(count, 2);
        assert!(html.starts_with("<main>\n<h1>A</h1>\n<p>Before.</p>\n<aside"));
        assert!(html.contains("<p>One.</p>\n</div></aside>\n<p>Between.</p>\n<ul>\n<li>\n<aside"));
        assert!(html.contains("<p>Two.</p>\n</div></aside>\n</li>\n</ul>\n<p>After.</p>\n</main>"));
    }

    #[test]
    fn a_page_without_quote_blocks_is_unchanged() {
        let html = page("<p>Nothing to draw.</p>");
        assert_eq!(rewrite(&html).unwrap(), (html.clone(), 0));
    }
}
