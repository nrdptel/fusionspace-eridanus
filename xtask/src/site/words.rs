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

use pulldown_cmark::Options;

use super::units::rendered_runs_with;

/// The en dash, U+2013.
const EN_DASH: char = '–';

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
}
