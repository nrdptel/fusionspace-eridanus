//! The writing rules the site's pages and the README are both held to (#476, ADR-295), from the
//! product system's `writing.md`. Each reads a page's text as it renders (prose, headings, list
//! items, table cells, link text and image descriptions; not code, which quotes files and
//! output):
//!
//! - **An en dash only in ranges** (*Mechanics*: "An en dash only in ranges"). [`joined_words`]
//!   fails an en dash with a word of two or more letters right before it and another right after
//!   it: `Dormand–Prince`, `fin–fin`. Joined names and words take a hyphen (`Dormand-Prince`). A
//!   range of figures (`0.3–0.5`, `5,100–5,500 ft`) passes, as does one of single letters (`H–O`,
//!   a span of motor classes), and so does a dash with a space, a figure or a sign on either side.
//!   On the site an en dash is also what a `--` renders as: mdBook turns on smart punctuation, so
//!   `Pods--powered` reaches the page as `Pods–powered` and fails. A name really spelled with two
//!   hyphens, such as OpenRocket's example *Pods-\-powered*, escapes one (`Pods-\-powered`), which
//!   the site and GitHub both show as two hyphens. GitHub, which shows the READMEs, leaves `--`
//!   as it stands.
//! - **Short notes and cells** (*Voice*: "Plain. Short sentences"; #476). [`long_blocks`] fails a
//!   note, a whole quote block (which the site draws as a note), or a table cell over
//!   [`MOST_WORDS`] words as it renders, code and link text included. A note says what the reader
//!   must know first; a cell holds a value or a phrase. What runs longer belongs in the page's
//!   own text, below the note or under the table. The text of HTML inside a note or a cell
//!   counts. A table or a quote block written in raw HTML fails: the checks read only
//!   Markdown's, so one in HTML would pass them unread. A footnote is neither a note nor a cell,
//!   and isn't read.

use pulldown_cmark::Options;

use super::units::rendered_runs_with;

/// The en dash, U+2013.
const EN_DASH: char = '–';

/// The most words a note or a table cell holds (#476).
pub(super) const MOST_WORDS: usize = 120;

/// What shows a page, which decides how its punctuation renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Renderer {
    /// The site, built by mdBook with smart punctuation: `--` is an en dash, `---` an em dash.
    Site,
    /// GitHub, crates.io and PyPI, which show the READMEs as written.
    GitHub,
}

impl Renderer {
    /// The parser's options for a page this renderer shows.
    fn options(self) -> Options {
        match self {
            Self::Site => super::options() | Options::ENABLE_SMART_PUNCTUATION,
            Self::GitHub => super::options(),
        }
    }
}

/// Each en dash on a page that joins two words, as (line, message), as `renderer` shows the page.
pub(super) fn joined_words(text: &str, renderer: Renderer) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let mut problems = Vec::new();
    for run in rendered_runs_with(text, renderer.options()) {
        for (at, _) in run.text.match_indices(EN_DASH) {
            let before: String = run.text[..at]
                .chars()
                .rev()
                .take_while(|c| c.is_alphabetic())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            let after: String = run.text[at + EN_DASH.len_utf8()..]
                .chars()
                .take_while(|c| c.is_alphabetic())
                .collect();
            if before.chars().count() >= 2 && after.chars().count() >= 2 {
                problems.push((
                    line(run.source(at)),
                    format!(
                        "`{before}–{after}` joins two words with an en dash (or a `--`, which the \
                         site shows as one): write `{before}-{after}`, or escape a real double \
                         hyphen as `-\\-` (writing.md, Mechanics: an en dash only in ranges)"
                    ),
                ));
            }
        }
    }
    problems
}

/// Each note (a whole quote block) and table cell on a page over [`MOST_WORDS`] words, as
/// (line, message), as `renderer` shows the page.
pub(super) fn long_blocks(text: &str, renderer: Renderer) -> Vec<(usize, String)> {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    let line = super::line_index(text);
    let mut problems = Vec::new();
    // The note being read (its start and text) and how deep in quote blocks the reader is; and
    // the table cell being read.
    let mut note: Option<(usize, String)> = None;
    let mut depth = 0usize;
    let mut cell: Option<(usize, String)> = None;
    for (event, range) in Parser::new_ext(text, renderer.options()).into_offset_iter() {
        let mut ends = None;
        match &event {
            Event::Start(Tag::BlockQuote(_)) => {
                if depth == 0 {
                    note = Some((range.start, String::new()));
                }
                depth += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    ends = note.take().map(|(at, words)| (at, words, "a note"));
                }
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let html = without_comments(html).to_ascii_lowercase();
                for tag in ["<table", "<blockquote"] {
                    if html.contains(tag) {
                        problems.push((
                            line(range.start),
                            format!(
                                "a `{tag}>` in raw HTML, which the checks of notes, cells and \
                                 heads can't read: write it in Markdown"
                            ),
                        ));
                    }
                }
            }
            Event::Start(Tag::TableCell) => cell = Some((range.start, String::new())),
            Event::End(TagEnd::TableCell) => {
                ends = cell.take().map(|(at, words)| (at, words, "a table cell"));
            }
            _ => {}
        }
        // What the event adds to the note's and the cell's text: its words, or a break
        // between words. Inline formatting and links join their words to the next.
        let piece = match &event {
            Event::Text(words)
            | Event::Code(words)
            | Event::InlineMath(words)
            | Event::DisplayMath(words) => Some(words.to_string()),
            // HTML's text shows on the page; its tags and comments part words.
            Event::Html(html) | Event::InlineHtml(html) => {
                Some(format!(" {} ", without_tags(&without_comments(html))))
            }
            Event::Start(
                Tag::Emphasis
                | Tag::Strong
                | Tag::Strikethrough
                | Tag::Link { .. }
                | Tag::Image { .. }
                | Tag::Superscript
                | Tag::Subscript,
            )
            | Event::End(
                TagEnd::Emphasis
                | TagEnd::Strong
                | TagEnd::Strikethrough
                | TagEnd::Link
                | TagEnd::Image
                | TagEnd::Superscript
                | TagEnd::Subscript,
            ) => None,
            _ => Some(" ".to_owned()),
        };
        if let Some(piece) = piece {
            for open in [&mut note, &mut cell].into_iter().flatten() {
                open.1.push_str(&piece);
            }
        }
        if let Some((at, words, what)) = ends {
            let count = word_count(&words);
            if count > MOST_WORDS {
                problems.push((
                    line(at),
                    format!(
                        "{what} of {count} words: hold it to {MOST_WORDS}, and move the rest \
                         into the page's own text (writing.md, Voice: plain, short sentences; \
                         #476)"
                    ),
                ));
            }
        }
    }
    problems
}

/// `html` with each comment, `<!-- … -->`, a space; an unclosed one runs to the end.
fn without_comments(html: &str) -> String {
    let mut out = String::new();
    let mut rest = html;
    while let Some(at) = rest.find("<!--") {
        out.push_str(&rest[..at]);
        out.push(' ');
        rest = rest[at..]
            .find("-->")
            .map_or("", |end| &rest[at + end + 3..]);
    }
    out.push_str(rest);
    out
}

/// `html` with each tag, `<…>`, a space.
fn without_tags(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

/// The words in `text`: each run of characters between spaces that holds a letter or a digit.
fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(text: &str) -> Vec<String> {
        joined_words(text, Renderer::Site)
            .into_iter()
            .map(|(line, message)| format!("{line}: {message}"))
            .collect()
    }

    #[test]
    fn an_en_dash_between_words_fails() {
        // The audit's own example, a lowercase join, and one in a heading, a list item and a
        // table cell, each on its own line.
        let text = "The Dormand–Prince pair.\n\nfin–fin interference\n\n## Prandtl–Meyer flow\n\n\
                    - a Gauss–Kronrod rule\n\n| A |\n|---|\n| log–log |\n";
        let problems = found(text);
        assert_eq!(problems.len(), 5, "{problems:#?}");
        assert!(problems[0].starts_with("1: `Dormand–Prince` joins two words"));
        assert!(problems[0].contains("write `Dormand-Prince`"));
        assert!(problems[1].starts_with("3: `fin–fin`"));
        assert!(problems[2].starts_with("5: `Prandtl–Meyer`"));
        assert!(problems[3].starts_with("7: `Gauss–Kronrod`"));
        assert!(problems[4].starts_with("11: `log–log`"));
        // And where the run is nested or styled: a nested list, an image's description,
        // emphasis on each word, inline HTML, a footnote and a note.
        for text in [
            "- a\n  - nested Dormand–Prince\n",
            "![a Dormand–Prince figure](x.png)",
            "*Dormand*–*Prince*",
            "<span>Dormand–Prince</span>",
            "Text[^1].\n\n[^1]: Dormand–Prince.\n",
            "> [!NOTE]\n> fin–fin\n",
        ] {
            assert_eq!(found(text).len(), 1, "{text}");
        }
    }

    #[test]
    fn a_double_hyphen_between_words_fails_on_the_site_only() {
        // mdBook's smart punctuation shows `--` as an en dash; GitHub shows it as it stands.
        let text = "OpenRocket's *Pods--powered* example";
        let problems = found(text);
        assert_eq!(problems.len(), 1, "{problems:#?}");
        assert!(problems[0].starts_with("1: `Pods–powered` joins two words"));
        assert_eq!(joined_words(text, Renderer::GitHub), []);
        // An escaped hyphen keeps a real double hyphen, which passes; so does an em dash's
        // `---`, which another check reads.
        assert!(found("OpenRocket's *Pods-\\-powered* example").is_empty());
        assert!(found("a word---another").is_empty());
    }

    #[test]
    fn a_range_passes() {
        // Figures, signs, single letters, spaced dashes, and a word against a figure.
        for text in [
            "from 5,100–5,500 ft",
            "Mach 0.3–0.5",
            "−29–−34%",
            "an H–O motor",
            "classes A–C",
            "a pause – then",
            "Mach–1",
            "M2.3c2–c3",
            "#890–#897",
            "Mach 0.3--0.5",
        ] {
            assert!(found(text).is_empty(), "{text}: {:?}", found(text));
        }
    }

    #[test]
    fn code_is_not_read() {
        // A code span or block quotes a file or output as it stands.
        assert!(found("`Dormand–Prince`\n\n```text\nfin–fin\n```\n").is_empty());
        assert!(found("`Pods--powered`").is_empty());
    }

    #[test]
    fn a_hyphen_join_passes() {
        assert!(found("The Dormand-Prince pair, fin-fin interference.").is_empty());
    }

    #[test]
    fn link_text_is_read_and_its_address_is_not() {
        let problems = found("[Runge–Kutta](https://example.org/runge–kutta)");
        assert_eq!(problems.len(), 1, "{problems:#?}");
        assert!(problems[0].contains("`Runge–Kutta`"));
        // A comment and a link's title don't show on the page.
        assert!(found("<!-- Dormand–Prince -->").is_empty());
        assert!(found("[x](https://example.org \"Dormand–Prince\")").is_empty());
    }

    fn words(count: usize) -> String {
        vec!["word"; count].join(" ")
    }

    #[test]
    fn a_long_note_or_cell_fails() {
        let long = |text: &str| long_blocks(text, Renderer::Site);
        // A note: a whole quote block, over 120 words, at its first line.
        let text = format!("Intro.\n\n> {}\n", words(MOST_WORDS + 1));
        let problems = long(&text);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert_eq!(problems[0].0, 3);
        assert!(
            problems[0]
                .1
                .starts_with("a note of 121 words: hold it to 120")
        );
        assert!(long(&format!("> {}\n", words(MOST_WORDS))).is_empty());
        // Its paragraphs, list items and nested quotes are one note; code and link text count,
        // and a soft break, a list item or a tag parts two words.
        let half = words(60);
        for text in [
            format!("> {half}\n>\n> {half} one\n"),
            format!("> {half}\n> - {half}\n> - one\n"),
            format!("> {half}\n>\n> > {half} one\n"),
            format!("> {half} `one`\n>\n> [{half}](x.md)\n"),
            format!("> {half}<br>{half}<br>one\n"),
            format!("> [!NOTE]\n> {half}\n> {half} one\n"),
        ] {
            assert_eq!(long(&text).len(), 1, "{text}");
        }
        // Two notes of 70 words each, apart, pass; so does emphasis inside a word.
        assert!(long(&format!("> {}\n\nText.\n\n> {}\n", words(70), words(70))).is_empty());
        assert!(long(&format!("> {} *Dor*mand\n", words(MOST_WORDS - 1))).is_empty());
        // A table cell, head or body, over 120 words fails, on its row's line; a row of short
        // cells passes however long the row.
        let cell = words(MOST_WORDS + 1);
        let problems = long(&format!("| A | B |\n|---|---|\n| x | {cell} |\n"));
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert_eq!(problems[0].0, 3);
        assert!(problems[0].1.starts_with("a table cell of 121 words"));
        assert_eq!(long(&format!("| {cell} |\n|---|\n| x |\n")).len(), 1);
        let row = vec![words(100); 3].join(" | ");
        assert!(long(&format!("| A | B | C |\n|---|---|---|\n| {row} |\n")).is_empty());
        // Marks and figures: a word holds a letter or a digit, so a lone dash, en or em, doesn't
        // count.
        assert!(long(&format!("> {} – \u{2014} · 1\n", words(MOST_WORDS - 1))).is_empty());
        // Both renderers read the same.
        assert_eq!(long_blocks(&text, Renderer::GitHub).len(), 1);
        // A table or a quote block in raw HTML fails, however short, as a block or inline;
        // other HTML passes.
        for text in [
            "<table><tr><td>1</td></tr></table>\n",
            "<BLOCKQUOTE>a</BLOCKQUOTE>\n",
            "Text <blockquote>a</blockquote>.\n",
            "<div>\n<table>\n</table>\n</div>\n",
        ] {
            let problems = long(text);
            assert_eq!(problems.len(), 1, "{text}: {problems:?}");
            assert!(problems[0].1.contains("in raw HTML"), "{problems:?}");
        }
        assert!(long("<span>a</span> <br> <!-- note -->\n").is_empty());
        // A table or a quote block named in a comment isn't one.
        assert!(long("<!-- a <table> here -->\n").is_empty());
        assert!(long("Text <!-- <blockquote> --> here.\n").is_empty());
        // HTML's text inside a note or a cell counts, its tags don't.
        let many = words(MOST_WORDS + 1);
        assert_eq!(long(&format!("> <div>\n> {many}\n> </div>\n")).len(), 1);
        assert_eq!(long(&format!("> <p>{many}</p>\n")).len(), 1);
        assert_eq!(long(&format!("| A |\n|---|\n| <b>{many}</b> |\n")).len(), 1);
        let tags = vec!["<br>"; MOST_WORDS + 1].join(" ");
        assert!(long(&format!("> {tags} word\n")).is_empty());
    }
}
