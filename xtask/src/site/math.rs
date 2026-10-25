//! The site's equations, typeset (the product system's `foundations.md`, *Type*): a page writes math in
//! LaTeX between dollar signs, and this mdBook preprocessor turns each formula into MathML before
//! mdBook renders the page, so the browser draws it with the math font and no script runs.
//!
//! - **Inline math** is `$…$` inside a sentence: `the drag force $F_D$`.
//! - **A display equation** is `$$…$$` standing alone as its own paragraph, a blank line above
//!   and below. It becomes `<div class="fs-equation"><math display="block">…</math></div>`. One
//!   wider than a phone's column is written on several lines (`\begin{aligned}`), broken before
//!   `=` or `+`; the page check fails one wider than its column at 320 px, as it fails any box.
//!
//! The parser is pulldown-cmark's math extension, so a dollar sign in code is never math, and a
//! formula's text is exactly what the page wrote, backslashes and all. The converter is
//! [math-core](https://github.com/tmke8/math-core) (MIT), which writes MathML Core, the subset
//! every current browser draws. Anything it can't convert fails the build, with the page and the
//! line, as does a formula it warns about, math in a heading (the sidebar and the anchor would
//! carry it), and a display equation sharing its paragraph with text.
//!
//! It also writes a quote mark straight after inline code, emphasis or a link as a closing one
//! ([`super::quotes`], #932), which mdBook's smart punctuation would turn into an opening one.
//!
//! mdBook runs this as `cargo xtask site-math`: `book.toml` names it, and [`super::mdbook_build`]
//! points mdBook at the running `xtask` itself, so the build never compiles a second copy.

use std::io::{Read, Write};
use std::ops::Range;

use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde_json::Value;

pub(crate) const USAGE: &str = "  site-math [supports <renderer> | --check <page.md>...]
                           mdBook's preprocessor for the site's equations: reads the book
                           from stdin and writes it back with each `$…$` and `$$…$$` as
                           MathML. `cargo xtask site` runs it through mdBook. `--check`
                           converts the pages named, prints each problem, and notes each
                           formula long enough to risk a phone's column.";

/// The environment variable that points mdBook's `math` preprocessor at a command, overriding
/// `book.toml`'s (mdBook reads `MDBOOK_<table>__<key>` over the file's values).
pub(crate) const COMMAND_ENV: &str = "MDBOOK_PREPROCESSOR__MATH__COMMAND";

/// The class of the block that holds a display equation; `theme/fusionspace-mdbook.css` spaces
/// it and the page check measures it.
pub(crate) const EQUATION_CLASS: &str = "fs-equation";

/// Runs the preprocessor: `supports <renderer>` answers mdBook's question (only the HTML
/// renderer draws MathML), and no argument converts the book on stdin.
pub(crate) fn run(args: &[String]) -> Result<(), String> {
    match args {
        [supports, renderer] if supports == "supports" => {
            if renderer == "html" {
                Ok(())
            } else {
                Err(format!(
                    "the math preprocessor draws only HTML, not `{renderer}`"
                ))
            }
        }
        [] => {
            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .map_err(|err| format!("could not read the book from mdBook: {err}"))?;
            let output = preprocess(&input)?;
            std::io::stdout()
                .write_all(output.as_bytes())
                .map_err(|err| format!("could not hand the book back to mdBook: {err}"))
        }
        [check, pages @ ..] if check == "--check" && !pages.is_empty() => check_pages(pages),
        _ => Err(format!("unexpected arguments {args:?}\n\n{USAGE}")),
    }
}

/// The longest formula, in characters of LaTeX, `--check` passes without a note: an inline one
/// can't wrap, and a display one on one line must fit a 320 px phone. A rough guide for the
/// writer, not a rule; the page check measures the drawn width.
const LONG_INLINE: usize = 40;
const LONG_DISPLAY_LINE: usize = 60;

/// `--check`: converts each page as the build does and prints its problems, then notes formulas
/// long enough to risk a phone's column.
fn check_pages(pages: &[String]) -> Result<(), String> {
    let converter = converter()?;
    let mut problems = Vec::new();
    for page in pages {
        let markdown =
            std::fs::read_to_string(page).map_err(|err| format!("could not read {page}: {err}"))?;
        if let Err(found) = typeset(&markdown, &converter) {
            problems.extend(found.into_iter().map(|p| format!("{page}:{p}")));
        }
        let (formulas, _) = find(&markdown);
        for formula in formulas {
            let line = line_of(&markdown, formula.range.start);
            let longest = if formula.display {
                formula
                    .latex
                    .split("\\\\")
                    .map(|l| l.trim().chars().count())
                    .max()
                    .unwrap_or(0)
            } else {
                formula.latex.chars().count()
            };
            let limit = if formula.display {
                LONG_DISPLAY_LINE
            } else {
                LONG_INLINE
            };
            if longest > limit {
                println!(
                    "note: {page}:{line}: a {} formula {longest} characters long; check it at \
                     320 px, or break it",
                    if formula.display { "display" } else { "inline" }
                );
            }
        }
    }
    if problems.is_empty() {
        println!("site-math: {} page(s), every formula converts", pages.len());
        Ok(())
    } else {
        Err(format!(
            "{} problem(s) in the pages' math:\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// Converts mdBook's `[context, book]` JSON and returns the book's JSON, every chapter's math
/// typeset, or every problem found across the book.
fn preprocess(input: &str) -> Result<String, String> {
    let value: Value =
        serde_json::from_str(input).map_err(|err| format!("mdBook's input isn't JSON: {err}"))?;
    let Value::Array(mut pair) = value else {
        return Err("mdBook's input isn't a [context, book] pair".to_owned());
    };
    if pair.len() != 2 {
        return Err(format!(
            "mdBook's input has {} items, not a [context, book] pair",
            pair.len()
        ));
    }
    let mut book = pair.pop().unwrap_or(Value::Null);
    let converter = converter()?;
    let mut problems = Vec::new();
    convert_chapters(&mut book, &converter, &mut problems);
    if !problems.is_empty() {
        return Err(format!(
            "{} problem(s) in the pages' math:\n  {}",
            problems.len(),
            problems.join("\n  ")
        ));
    }
    serde_json::to_string(&book).map_err(|err| format!("could not write the book: {err}"))
}

/// The converter, with math-core's defaults: no line breaks in its output (a raw HTML block ends
/// at a blank line), unknown commands an error, no TeX annotation (the search index would carry
/// every formula twice).
fn converter() -> Result<LatexToMathML, String> {
    LatexToMathML::new(MathCoreConfig::default())
        .map_err(|err| format!("could not set up the math converter: {err:?}"))
}

/// Walks the book's items, whatever their nesting, and converts each chapter's `content`.
fn convert_chapters(value: &mut Value, converter: &LatexToMathML, problems: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Object(chapter)) = map.get_mut("Chapter") {
                let name = chapter
                    .get("source_path")
                    .or_else(|| chapter.get("path"))
                    .and_then(Value::as_str)
                    .unwrap_or("a chapter")
                    .to_owned();
                if let Some(Value::String(content)) = chapter.get_mut("content") {
                    // Quote marks after a span first, while the page is still the writer's
                    // Markdown (#932).
                    let content_quoted = super::quotes::closing_quotes(content);
                    match typeset(&content_quoted, converter) {
                        Ok(typeset) => *content = typeset,
                        Err(found) => {
                            problems.extend(found.into_iter().map(|p| format!("{name}:{p}")));
                        }
                    }
                }
            }
            for child in map.values_mut() {
                convert_chapters(child, converter, problems);
            }
        }
        Value::Array(items) => {
            for item in items {
                convert_chapters(item, converter, problems);
            }
        }
        _ => {}
    }
}

/// Arrow accents, which Chromium and Safari draw off center over the letter (the product system's
/// `foundations.md`, *Type*: vectors in bold).
const ARROW_ACCENTS: [&str; 3] = ["\\vec", "\\overrightarrow", "\\overleftarrow"];

/// A formula found in a page: where it sits in the source, its LaTeX, and whether it is a
/// display equation.
struct Formula {
    range: Range<usize>,
    latex: String,
    display: bool,
}

/// Returns `markdown` with every formula replaced by its MathML, or each problem as
/// `line: what`.
pub(crate) fn typeset(markdown: &str, converter: &LatexToMathML) -> Result<String, Vec<String>> {
    let (formulas, mut problems) = find(markdown);
    let mut out = String::with_capacity(markdown.len());
    let mut at = 0;
    for formula in &formulas {
        let line = line_of(markdown, formula.range.start);
        if !formula.display
            && markdown[formula.range.end..].starts_with(|c: char| c.is_ascii_digit())
        {
            problems.push(format!(
                "{line}: `${}$` runs into a digit, so it is likely two prices read as math: \
                 write a dollar sign that isn't math as `\\$`",
                formula.latex
            ));
            continue;
        }
        if formula.latex.starts_with('`') {
            problems.push(format!(
                "{line}: `${}$` is GitHub's code-span math, which the site doesn't read: write \
                 `$…$`",
                formula.latex
            ));
            continue;
        }
        if let Some(accent) = ARROW_ACCENTS.iter().find(|a| formula.latex.contains(*a)) {
            problems.push(format!(
                "{line}: `{}` uses `{accent}`, an arrow accent browsers misplace: write a vector \
                 in bold, `\\mathbf{{v}}`",
                formula.latex
            ));
            continue;
        }
        let mode = if formula.display {
            MathDisplay::Block
        } else {
            MathDisplay::Inline
        };
        match converter.convert_with_local_state(&formula.latex, mode) {
            Ok(result) => {
                if result.warnings.has_unknown_commands()
                    || result.warnings.has_undefined_references()
                {
                    problems.push(format!(
                        "{line}: `{}` has an unknown command or an undefined reference",
                        formula.latex
                    ));
                    continue;
                }
                if result.mathml.contains('\n') {
                    problems.push(format!(
                        "{line}: `{}` converts to more than one line of MathML",
                        formula.latex
                    ));
                    continue;
                }
                out.push_str(&markdown[at..formula.range.start]);
                let mathml = inert(&result.mathml);
                if formula.display {
                    out.push_str(&format!("<div class=\"{EQUATION_CLASS}\">{mathml}</div>"));
                } else {
                    out.push_str(&mathml);
                }
                at = formula.range.end;
            }
            Err(err) => problems.push(format!(
                "{line}: `{}` isn't math the converter reads: {err:?}",
                formula.latex
            )),
        }
    }
    out.push_str(&markdown[at..]);
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// MathML with every character Markdown would still read written as a character reference: the
/// MathML is raw HTML, but mdBook reads the text between its tags as Markdown, so a `|` would
/// end a table's cell, a pair of `*` or `_` would turn into emphasis, a backslash would escape
/// what follows, and its smart punctuation would curl the quotes and turn `--` into a dash in a
/// `\text{}`. Only text between tags is touched; a tag and its attributes are written as
/// math-core wrote them.
fn inert(mathml: &str) -> String {
    let mut out = String::with_capacity(mathml.len());
    let mut in_tag = false;
    for c in mathml.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ => {}
        }
        let markdown = matches!(
            c,
            '|' | '*' | '_' | '`' | '[' | ']' | '~' | '\\' | '$' | '!' | '\'' | '"' | '-'
        );
        if markdown && !in_tag {
            out.push_str(&format!("&#{};", u32::from(c)));
        } else {
            out.push(c);
        }
    }
    out
}

/// Finds a page's formulas in source order, with the problems of where they stand.
fn find(markdown: &str) -> (Vec<Formula>, Vec<String>) {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_GFM
        | Options::ENABLE_MATH;
    let mut formulas = Vec::new();
    let mut problems = Vec::new();
    let mut in_heading = false;
    let mut in_image = false;
    // Each open paragraph: the events seen in it so far, and its display equations.
    let mut paragraph: Option<(usize, Vec<usize>)> = None;
    for (event, range) in Parser::new_ext(markdown, options).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => in_heading = true,
            Event::End(TagEnd::Heading(_)) => in_heading = false,
            Event::Start(Tag::Image { .. }) => in_image = true,
            Event::End(TagEnd::Image) => in_image = false,
            Event::Start(Tag::Paragraph) => paragraph = Some((0, Vec::new())),
            Event::End(TagEnd::Paragraph) => {
                if let Some((events, displays)) = paragraph.take()
                    && !displays.is_empty()
                {
                    // A display becomes a line opening `<div`, a raw HTML block that runs to the
                    // next blank line: a line right under it, a list item say, would be shown as
                    // raw text. So it needs its paragraph to itself and a blank line below.
                    if events != 1 || !blank_line_below(markdown, range.end) {
                        for start in displays {
                            problems.push(format!(
                                "{}: a display equation (`$$…$$`) stands alone as its own \
                                 paragraph, a blank line above and below",
                                line_of(markdown, start)
                            ));
                        }
                    }
                }
            }
            Event::InlineMath(_) | Event::DisplayMath(_) => {
                let (latex, display) = match event {
                    Event::DisplayMath(latex) => (latex, true),
                    Event::InlineMath(latex) => (latex, false),
                    _ => unreachable!("matched as math just above"),
                };
                if in_image {
                    problems.push(format!(
                        "{}: an image's description has math in it, which a screen reader \
                         would read out as markup: write it in words",
                        line_of(markdown, range.start)
                    ));
                }
                if in_heading {
                    problems.push(format!(
                        "{}: a heading has math in it; the sidebar and the anchor would \
                         carry the MathML, so write it in words",
                        line_of(markdown, range.start)
                    ));
                }
                if let Some((events, displays)) = paragraph.as_mut() {
                    *events += 1;
                    if display {
                        displays.push(range.start);
                    }
                } else if display {
                    problems.push(format!(
                        "{}: a display equation sits outside a paragraph",
                        line_of(markdown, range.start)
                    ));
                }
                formulas.push(Formula {
                    range,
                    latex: latex.into_string(),
                    display,
                });
            }
            Event::SoftBreak | Event::HardBreak => {}
            Event::Text(text) if text.trim().is_empty() => {}
            _ => {
                if let Some((events, _)) = paragraph.as_mut() {
                    *events += 1;
                }
            }
        }
    }
    (formulas, problems)
}

/// Whether the line after the one holding byte `end - 1` is blank, or there is none: a quote
/// block's `>` alone counts as blank.
fn blank_line_below(markdown: &str, end: usize) -> bool {
    let from = end.saturating_sub(1).min(markdown.len());
    let Some(newline) = markdown[from..].find('\n') else {
        return true;
    };
    markdown[from + newline + 1..]
        .lines()
        .next()
        .is_none_or(|line| line.trim_start_matches([' ', '\t', '>']).trim().is_empty())
}

/// The 1-based line of a byte offset.
fn line_of(markdown: &str, offset: usize) -> usize {
    markdown[..offset].matches('\n').count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typeset_ok(markdown: &str) -> String {
        let converter = converter().expect("the default configuration is valid");
        typeset(markdown, &converter).unwrap_or_else(|p| panic!("{p:?}"))
    }

    fn typeset_err(markdown: &str) -> Vec<String> {
        let converter = converter().expect("the default configuration is valid");
        match typeset(markdown, &converter) {
            Ok(out) => panic!("expected problems, got {out}"),
            Err(problems) => problems,
        }
    }

    #[test]
    fn inline_math_becomes_mathml_in_its_sentence() {
        let out = typeset_ok("The drag $F_D = \\tfrac12 \\rho v^2$ grows.\n");
        assert!(out.starts_with("The drag <math>"), "{out}");
        assert!(out.contains("<mi>ρ</mi>"), "{out}");
        assert!(out.ends_with("</math> grows.\n"), "{out}");
    }

    #[test]
    fn a_display_equation_becomes_one_block_line() {
        let out = typeset_ok(
            "Above.\n\n$$\n\\begin{aligned} a &= b \\\\ c &= d \\end{aligned}\n$$\n\nBelow.\n",
        );
        let line = out.lines().nth(2).expect("the equation's line");
        assert!(
            line.starts_with("<div class=\"fs-equation\"><math display=\"block\">"),
            "{out}"
        );
        assert!(line.ends_with("</math></div>"), "{out}");
        assert_eq!(out.lines().count(), 5, "{out}");
    }

    #[test]
    fn dollars_in_code_stay_as_written() {
        let page = "Run `echo $HOME` and\n\n```sh\nprice=$5 $x$\n```\n";
        assert_eq!(typeset_ok(page), page);
    }

    #[test]
    fn markdown_characters_in_math_are_inert() {
        let out = typeset_ok("| a | b |\n|---|---|\n| x | $\\lvert x \\rvert + \\text{a*b*c}$ |\n");
        let row = out.lines().nth(2).expect("the row");
        assert_eq!(row.matches('|').count(), 3, "{out}");
        assert!(
            row.contains("&#124;") && row.contains("a&#42;b&#42;c"),
            "{out}"
        );
        let text = typeset_ok("$\\text{it's \"x\" -- y}$\n");
        // math-core sets the quotes as TeX does; the dashes stay two hyphens, as written.
        assert!(text.contains("&#45;&#45;"), "{text}");
        assert!(text.starts_with("<math>"), "{text}");
    }

    #[test]
    fn prices_are_prose() {
        let page = "A motor costs $5, and $5 and $10 are prices too.\n";
        assert_eq!(typeset_ok(page), page);
    }

    #[test]
    fn a_price_range_read_as_math_fails() {
        let problems = typeset_err("From $5-$10.\n");
        assert!(problems[0].contains("two prices"), "{problems:?}");
        let escaped = "From \\$5-\\$10.\n";
        assert_eq!(typeset_ok(escaped), escaped);
    }

    #[test]
    fn an_arrow_accent_fails() {
        let problems = typeset_err("Velocity $\\vec{v}$.\n");
        assert!(problems[0].contains("\\mathbf"), "{problems:?}");
    }

    #[test]
    fn a_page_without_math_is_unchanged() {
        let page = "# Title\n\nA rocket costs 5 dollars.\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
        assert_eq!(typeset_ok(page), page);
    }

    #[test]
    fn an_unknown_command_fails_with_its_line() {
        let problems = typeset_err("One.\n\nTwo $\\notacommand{x}$.\n");
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].starts_with("3: `\\notacommand{x}`"),
            "{problems:?}"
        );
    }

    #[test]
    fn math_in_a_heading_fails() {
        let problems = typeset_err("## The force $F$\n");
        assert!(problems[0].contains("a heading has math"), "{problems:?}");
    }

    #[test]
    fn a_display_equation_inside_a_sentence_fails() {
        let problems = typeset_err("The equation\n$$a = b$$\nholds.\n");
        assert!(problems[0].contains("stands alone"), "{problems:?}");
    }

    #[test]
    fn a_line_right_under_a_display_equation_fails() {
        let problems = typeset_err("- a\n\n  $$x$$\n  - b\n");
        assert!(
            problems[0].starts_with("3: a display equation"),
            "{problems:?}"
        );
        let problems = typeset_err("$$x$$\n- b\n");
        assert!(problems[0].contains("stands alone"), "{problems:?}");
        let ok = typeset_ok("- a\n\n  $$x$$\n\n  - b\n");
        assert!(ok.contains("</div>\n\n  - b"), "{ok}");
        let quoted = typeset_ok("> $$x$$\n>\n> after\n");
        assert!(quoted.contains("</div>\n>\n> after"), "{quoted}");
    }

    #[test]
    fn math_in_an_image_description_fails() {
        let problems = typeset_err("![The force $F$](f.svg)\n");
        assert!(problems[0].contains("image's description"), "{problems:?}");
    }

    #[test]
    fn a_display_equation_in_a_list_item_keeps_the_item() {
        let out = typeset_ok("- First:\n\n  $$a = b$$\n\n- Second.\n");
        assert!(
            out.contains("\n  <div class=\"fs-equation\"><math display=\"block\">"),
            "{out}"
        );
    }

    #[test]
    fn supports_answers_only_html() {
        assert!(run(&["supports".to_owned(), "html".to_owned()]).is_ok());
        assert!(run(&["supports".to_owned(), "markdown".to_owned()]).is_err());
    }

    #[test]
    fn the_book_round_trips_with_its_chapters_converted() {
        let input = r#"[{"root":"."},{"items":[{"Chapter":{"name":"A","content":"x $a^2$","source_path":"a.md","sub_items":[{"Chapter":{"name":"B","content":"$b$","source_path":"b.md","sub_items":[]}}]}},"Separator"]}]"#;
        let out: Value =
            serde_json::from_str(&preprocess(input).expect("converts")).expect("JSON out");
        let a = &out["items"][0]["Chapter"];
        assert!(
            a["content"]
                .as_str()
                .is_some_and(|c| c.starts_with("x <math>"))
        );
        let b = &a["sub_items"][0]["Chapter"];
        assert!(
            b["content"]
                .as_str()
                .is_some_and(|c| c.starts_with("<math>"))
        );
        assert_eq!(out["items"][1], "Separator");
    }

    #[test]
    fn a_problem_names_its_page() {
        let input = r#"[{},{"items":[{"Chapter":{"name":"A","content":"$\\nope$","source_path":"physics/a.md","sub_items":[]}}]}]"#;
        let err = preprocess(input).expect_err("fails");
        assert!(err.contains("physics/a.md:1: `\\nope`"), "{err}");
    }
}
