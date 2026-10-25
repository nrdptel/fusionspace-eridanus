//! Apostrophes after inline code, emphasis and links (#932). mdBook's smart punctuation curls a
//! straight `'` by the characters around it in the source, and pairs it with the others in its
//! paragraph. A span's closing delimiter (a backtick, `*`, `]` or `)`) is punctuation, so the `'`
//! of "`` `hpr`'s ``" counts as able to open; when a later `'` in the paragraph can only close (a
//! plural's possessive, "the flights' end"), the two pair, and the first comes out as an opening
//! quote, `‘s`, its hook facing away from the word it belongs to.
//!
//! A `'` straight after a span's end and straight before a letter or digit (`'s`, `'d`, `'ll`) is
//! an apostrophe: [`closing_quotes`] writes it as one (U+2019) before mdBook renders the page,
//! and smart punctuation leaves a curly mark as written. A `'` with a space after it is left
//! alone: it may close a quoted phrase opened earlier (`'quoted `x`' plain`), and only
//! straight quotes pair. The page check fails an opening mark drawn straight after one of these
//! elements and before a letter or digit.

use pulldown_cmark::{Event, Options, Parser, TagEnd};

/// The Markdown mdBook 0.5 reads (its `pulldown-cmark` options: tables, footnotes,
/// strikethrough, task lists, heading attributes, definition lists, GFM's blocks), with the
/// math the site's preprocessor reads, so the spans found are the ones mdBook draws.
const OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_TASKLISTS)
    .union(Options::ENABLE_HEADING_ATTRIBUTES)
    .union(Options::ENABLE_DEFINITION_LIST)
    .union(Options::ENABLE_GFM)
    .union(Options::ENABLE_MATH);

/// Returns `markdown` with each `'` right after an inline span's end (code, math, emphasis,
/// strong emphasis, strikethrough, a link, an image or a footnote's reference) and right before
/// a letter or digit written as an apostrophe, U+2019. The spans are pulldown-cmark's, so a
/// quote inside code is never touched.
pub(crate) fn closing_quotes(markdown: &str) -> String {
    let mut ends: Vec<usize> = Parser::new_ext(markdown, OPTIONS)
        .into_offset_iter()
        .filter_map(|(event, range)| match event {
            Event::Code(_)
            | Event::InlineMath(_)
            | Event::FootnoteReference(_)
            | Event::End(
                TagEnd::Emphasis
                | TagEnd::Strong
                | TagEnd::Strikethrough
                | TagEnd::Link
                | TagEnd::Image,
            ) => Some(range.end),
            _ => None,
        })
        .filter(|&end| {
            let mut after = markdown[end..].chars();
            after.next() == Some('\'') && after.next().is_some_and(char::is_alphanumeric)
        })
        .collect();
    // A span nested in another ends before the outer one's delimiter (`*`a`*`'s: the code at the
    // `*`), so only the outer end is followed by the mark; two ends at one byte would write the
    // mark twice and slice backwards, so keep one should they ever meet.
    ends.sort_unstable();
    ends.dedup();
    let mut out = String::with_capacity(markdown.len() + 2 * ends.len());
    let mut at = 0;
    for end in ends {
        out.push_str(&markdown[at..end]);
        out.push('’');
        // The `'` kept is one byte.
        at = end + 1;
    }
    out.push_str(&markdown[at..]);
    out
}

#[cfg(test)]
mod tests {
    use super::{OPTIONS, closing_quotes};
    use pulldown_cmark::{Event, Options, Parser};

    /// The text mdBook draws from `markdown` after [`closing_quotes`], its smart punctuation on:
    /// each code span as `<code>…</code>`, other markup left out.
    fn drawn(markdown: &str) -> String {
        let quoted = closing_quotes(markdown);
        let mut out = String::new();
        for event in Parser::new_ext(&quoted, OPTIONS | Options::ENABLE_SMART_PUNCTUATION) {
            match event {
                Event::Text(text) => out.push_str(&text),
                Event::Code(code) => out.push_str(&format!("<code>{code}</code>")),
                Event::InlineMath(math) => out.push_str(&format!("<math>{math}</math>")),
                Event::FootnoteReference(name) => out.push_str(&format!("<sup>{name}</sup>")),
                Event::SoftBreak => out.push(' '),
                _ => {}
            }
        }
        out
    }

    #[test]
    fn an_apostrophe_after_inline_code_closes() {
        assert_eq!(closing_quotes("`hpr`'s help"), "`hpr`’s help");
        assert_eq!(closing_quotes("the `hpr`'s; then"), "the `hpr`’s; then");
        // A later `'` that can only close pairs with the one after the span: smart punctuation
        // alone draws `<code>hpr</code>‘s`.
        assert_eq!(
            drawn("`hpr`'s help and the flights' end"),
            "<code>hpr</code>’s help and the flights’ end"
        );
    }

    #[test]
    fn apostrophes_after_emphasis_links_math_and_footnotes_close() {
        // Each with a later `'` that can only close, which smart punctuation alone pairs with
        // the one after the span.
        let cases = [
            (
                "The *Tube fin rocket*'s margin, the flights' end",
                "The Tube fin rocket’s margin, the flights’ end",
            ),
            (
                "**bold**'s and the flights' end",
                "bold’s and the flights’ end",
            ),
            (
                "~~old~~'s and the flights' end",
                "old’s and the flights’ end",
            ),
            (
                "[OpenRocket](https://x.org)'s and the flights' end",
                "OpenRocket’s and the flights’ end",
            ),
            (
                "$C_D$'s and the flights' end",
                "<math>C_D</math>’s and the flights’ end",
            ),
            (
                "a note[^1]'s and the flights' end\n\n[^1]: N.\n",
                "a note<sup>1</sup>’s and the flights’ endN.",
            ),
        ];
        for (markdown, shown) in cases {
            assert_eq!(drawn(markdown), shown, "{markdown:?}");
        }
    }

    #[test]
    fn a_quoted_phrase_ending_at_a_span_keeps_both_marks() {
        assert_eq!(
            drawn("The 'quoted `x`' plain"),
            "The ‘quoted <code>x</code>’ plain"
        );
        assert_eq!(drawn("'`hpr`' alone"), "‘<code>hpr</code>’ alone");
        assert_eq!(
            drawn("\"the `hpr`\" command"),
            "“the <code>hpr</code>” command"
        );
        assert_eq!(drawn("**`a`**\"x\" and \"y\""), "<code>a</code>“x” and “y”");
    }

    #[test]
    fn nested_spans_close_the_mark_after_the_outer_one() {
        assert_eq!(closing_quotes("*`hpr`*'s"), "*`hpr`*’s");
        assert_eq!(closing_quotes("[`hpr`](x)'s"), "[`hpr`](x)’s");
    }

    #[test]
    fn quotes_in_code_or_not_before_a_letter_stay() {
        let unchanged = [
            "`it's`",
            "```\n`a`'s\n```\n",
            "    `a`'s in an indented block\n",
            "`a` 's",
            "`a` 'quoted'",
            "'quoted `a`' plain",
            "`a`'",
            "`a`'.",
            "it's a word's",
            "*a* \"b\"",
            "\"the `a`\" command",
            "`a`’s already curly",
        ];
        for text in unchanged {
            assert_eq!(closing_quotes(text), text, "{text:?}");
        }
    }

    #[test]
    fn a_table_cell_and_a_list_item_close_too() {
        assert_eq!(
            closing_quotes("| a |\n|---|\n| `x`'s |\n"),
            "| a |\n|---|\n| `x`’s |\n"
        );
        assert_eq!(closing_quotes("- `x`'s item\n"), "- `x`’s item\n");
        assert_eq!(closing_quotes("- a\n  - `x`'s\n"), "- a\n  - `x`’s\n");
        assert_eq!(
            closing_quotes("Term\n: `x`'s definition\n"),
            "Term\n: `x`’s definition\n"
        );
    }
}
