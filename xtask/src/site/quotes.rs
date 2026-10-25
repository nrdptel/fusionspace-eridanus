//! Quote marks after inline code, emphasis and links (#932). mdBook's smart punctuation curls a
//! straight quote by the character before it in the source, and a span's closing delimiter (a
//! backtick, `*`, `]` or `)`) is punctuation, so the `'` of "`` `hpr`'s ``" came out as an
//! opening quote, `‘s`, its hook facing away from the word it belongs to. A quote mark straight
//! after a span's end, with no space between, closes something or is an apostrophe: it is never an
//! opening one. [`closing_quotes`] writes each such mark as its closing character (U+2019 for
//! `'`, U+201D for `"`) before mdBook renders the page, which leaves a curly mark as written; the
//! page check fails an opening mark drawn straight after one of these elements.

use pulldown_cmark::{Event, Options, Parser, TagEnd};

/// Returns `markdown` with each straight quote right after an inline span's end (code, math,
/// emphasis, strong emphasis, strikethrough, a link or an image) written as its closing mark.
/// The spans are pulldown-cmark's, so a quote inside code is never touched.
pub(crate) fn closing_quotes(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_GFM
        | Options::ENABLE_MATH;
    let mut ends: Vec<usize> = Parser::new_ext(markdown, options)
        .into_offset_iter()
        .filter_map(|(event, range)| match event {
            Event::Code(_)
            | Event::InlineMath(_)
            | Event::End(
                TagEnd::Emphasis
                | TagEnd::Strong
                | TagEnd::Strikethrough
                | TagEnd::Link
                | TagEnd::Image,
            ) => Some(range.end),
            _ => None,
        })
        .filter(|&end| markdown[end..].starts_with(['\'', '"']))
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
        // Each end was kept for starting with one of the two marks.
        out.push(if markdown[end..].starts_with('\'') {
            '’'
        } else {
            '”'
        });
        at = end + 1;
    }
    out.push_str(&markdown[at..]);
    out
}

#[cfg(test)]
mod tests {
    use super::closing_quotes;

    #[test]
    fn an_apostrophe_after_inline_code_closes() {
        assert_eq!(closing_quotes("`hpr`'s help"), "`hpr`’s help");
        assert_eq!(closing_quotes("the `hpr`'s; then"), "the `hpr`’s; then");
    }

    #[test]
    fn quotes_after_emphasis_links_and_math_close() {
        assert_eq!(
            closing_quotes("*Tube fin rocket*'s margin"),
            "*Tube fin rocket*’s margin"
        );
        assert_eq!(closing_quotes("**bold**'s"), "**bold**’s");
        assert_eq!(closing_quotes("~~old~~'s"), "~~old~~’s");
        assert_eq!(
            closing_quotes("[OpenRocket](https://x.org)'s"),
            "[OpenRocket](https://x.org)’s"
        );
        assert_eq!(closing_quotes("$C_D$'s value"), "$C_D$’s value");
        assert_eq!(
            closing_quotes("\"the `hpr`\" command"),
            "\"the `hpr`” command"
        );
    }

    #[test]
    fn nested_spans_close_the_mark_after_the_outer_one() {
        assert_eq!(closing_quotes("*`hpr`*'s"), "*`hpr`*’s");
        assert_eq!(closing_quotes("[`hpr`](x)'s"), "[`hpr`](x)’s");
    }

    #[test]
    fn quotes_in_code_or_after_a_space_or_a_word_stay() {
        let unchanged = [
            "`it's`",
            "```\n`a`'s\n```\n",
            "    `a`'s in an indented block\n",
            "`a` 's",
            "`a` 'quoted'",
            "it's a word's",
            "*a* \"b\"",
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
    }
}
