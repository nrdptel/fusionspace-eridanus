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

use super::units::rendered_runs;

/// The en dash, U+2013.
const EN_DASH: char = '–';

/// Each en dash on a page that joins two words, as (line, message).
pub(super) fn joined_words(text: &str) -> Vec<(usize, String)> {
    let line = super::line_index(text);
    let mut problems = Vec::new();
    for run in rendered_runs(text) {
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
                        "`{before}–{after}` joins two words with an en dash: write \
                         `{before}-{after}` (writing.md, Mechanics: an en dash only in ranges)"
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
        joined_words(text)
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
        ] {
            assert!(found(text).is_empty(), "{text}: {:?}", found(text));
        }
    }

    #[test]
    fn code_is_not_read() {
        // A code span or block quotes a file or output as it stands.
        assert!(found("`Dormand–Prince`\n\n```text\nfin–fin\n```\n").is_empty());
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
    }
}
