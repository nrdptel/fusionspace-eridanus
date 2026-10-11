//! The print page (#443; the product system's `web.md`, *Print* and *Performance*). mdBook's
//! `print.html` holds every chapter, one after another, and opens the browser's print window
//! once it loads. Drawn on screen that way, it is some 48,000 elements, which a phone parses,
//! lays out and walks again on each key and tap. In the vitals check (`vitals.rs`), its first
//! Tab took 0.4 s and a tap on the menu button 0.45 s, past INP's 200 ms, and its largest paint
//! came at 2.9 s, past LCP's 2.5 s.
//!
//! So after the anatomy is drawn ([`super::sheets`]), [`draw`] puts the chapters in a template,
//! `<template id="hpr-book">`. The browser reads a template's contents but neither shows them
//! nor counts them as part of the page. Before it, [`draw`] writes a cover: the designation tag,
//! the title "Print the guide", what the page prints, and a button that prints it. When the
//! browser is about to print (`beforeprint`), `theme/hpr.js` places a copy of the chapters in
//! the page, highlighting their code as mdBook's script highlights a page's as it loads, and
//! takes the copy away once printing ends (`afterprint`). `theme/hpr.css` hides the cover on
//! paper. On paper the page is the whole book, as before. On screen it is the cover, a layout
//! made for printing rather than the screen printed, as `web.md` asks. A reader without scripts
//! gets the cover, with a note to print each chapter from its own page.
//!
//! [`draw`] also takes out mdBook's script that prints as the page loads. A browser without a
//! print window, as a headless one is, starts to print (`beforeprint`) and never ends
//! (`afterprint`), which left the chapters in the page; and a phone opening a print window
//! unasked is no help either. The reader prints from the cover's button or the browser's own
//! Print command.
//!
//! [`check`] fails a print page whose `main` isn't the cover followed by the template holding
//! every chapter, whose cover counts them wrong, or that still prints as it loads. The page check
//! places the chapters, as printing does, before it reads the page on paper, and fails a page
//! where they don't arrive or don't leave afterwards (`pages.js`, `probePrint`).

use std::fs;
use std::path::Path;

use super::theme::DESIGNATION;

/// The page, as mdBook names it.
pub(super) const PAGE: &str = "print.html";
/// How mdBook 0.5 opens and closes a page's text.
const MAIN: (&str, &str) = ("<main>", "</main>");
/// The template the chapters wait in, and its end; `theme/hpr.js` finds it by its id.
const BOOK: &str = "<template id=\"hpr-book\">";
const BOOK_END: &str = "</template>";
/// The cover's box, which `theme/hpr.css` hides on paper.
const COVER: &str = "<div id=\"hpr-print-cover\">";
/// mdBook 0.5's script that prints the page once it loads, as its template writes it.
const AUTO_PRINT: &str = "<script>
        window.addEventListener('load', function() {
            window.setTimeout(window.print, 100);
        });
        </script>";
/// What [`check`] looks for of [`AUTO_PRINT`], whatever its spacing.
const AUTO_PRINT_CALL: &str = "window.print, ";

/// The cover of a book of `chapters` chapters, as [`draw`] writes it before the template.
fn cover(chapters: usize) -> String {
    let tag = DESIGNATION.trim_matches('`');
    format!(
        "{COVER}<div class=\"fs-intro\"><p class=\"fs-tag\">{tag}</p>\
         <h1 id=\"print-the-guide\">Print the guide</h1>\n\
         <p>This page prints the whole guide, its {chapters} chapters one after another, each \
         from a new sheet. Printed to a PDF file, the guide can be read offline, at the launch \
         site too.</p>\n\
         <p><button type=\"button\" class=\"fs-btn\" id=\"hpr-print\">Print the guide</button></p>\n\
         <p>The browser’s own Print command prints it as well: Ctrl+P, or Command+P on a Mac; on \
         a phone, Share and then Print. On screen the page shows only this cover, as every \
         chapter on one page is slow to load and slow to answer a tap on a phone. Each chapter is \
         also a page of its own, listed in the sidebar.</p>\n\
         <noscript><p>Printing the whole guide needs the page’s script, which is off here: print \
         each chapter from its own page.</p></noscript></div></div>\n"
    )
}

/// How many titles (`h1`) `html` holds.
fn titles(html: &str) -> usize {
    html.match_indices("<h1")
        .filter(|(at, _)| html[at + 3..].starts_with([' ', '>']))
        .count()
}

/// Where the text of the page's one `main` starts and ends.
fn main_text(html: &str) -> Result<(usize, usize), String> {
    for tag in [MAIN.0, MAIN.1] {
        let found = html.matches(tag).count();
        if found != 1 {
            return Err(format!("`{tag}` is there {found} times, not once"));
        }
    }
    let open = html.find(MAIN.0).map(|at| at + MAIN.0.len());
    let close = html.find(MAIN.1);
    match open.zip(close) {
        Some((open, close)) if open <= close => Ok((open, close)),
        _ => Err(format!("`{}` comes before `{}`", MAIN.1, MAIN.0)),
    }
}

/// Puts the chapters of the print page built under `output` in a template, with the cover
/// before it. Returns how many chapters the page holds.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let path = output.join(PAGE);
    let html = fs::read_to_string(&path)
        .map_err(|err| format!("could not read {}: {err}", path.display()))?;
    let (drawn, chapters) = rewrite(&html).map_err(|why| {
        format!("{PAGE}: the chapters can't be put in a template (xtask/src/site/print.rs): {why}")
    })?;
    fs::write(&path, drawn).map_err(|err| format!("could not write {}: {err}", path.display()))?;
    Ok(chapters)
}

/// The print page with its chapters in a template and the cover before it, and how many
/// chapters it holds; or why its text can't be moved there.
fn rewrite(html: &str) -> Result<(String, usize), String> {
    let (open, close) = main_text(html)?;
    let text = &html[open..close];
    // A template can't hold another: the inner one's end would close the outer. A script copied
    // out of a template runs when it is placed, so one there would run at every print.
    for (tag, what) in [("<template", "a template"), ("<script", "a script")] {
        if text.contains(tag) {
            return Err(format!("the chapters hold {what} (`{tag}`)"));
        }
    }
    let chapters = titles(text);
    if chapters == 0 {
        return Err("no chapter (`h1`) in the page's `main`".to_string());
    }
    let rest = &html[close..];
    let found = rest.matches(AUTO_PRINT).count();
    if found != 1 || html.matches(AUTO_PRINT_CALL).count() != 1 {
        return Err(format!(
            "mdBook's script that prints the page as it loads is there {found} times after the \
             page's text, not once as {AUTO_PRINT:?}"
        ));
    }
    Ok((
        format!(
            "{}\n{}{BOOK}{text}{BOOK_END}\n{}",
            &html[..open],
            cover(chapters),
            rest.replacen(AUTO_PRINT, "", 1)
        ),
        chapters,
    ))
}

/// The print page's problems, under `output`.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let path = output.join(PAGE);
    let html = fs::read_to_string(&path)
        .map_err(|err| format!("could not read {}: {err}", path.display()))?;
    Ok(page_problems(&html)
        .into_iter()
        .map(|why| format!("{PAGE}: {why}"))
        .collect())
}

/// What the print page lacks of its cover and template: its `main` holds the cover, then the
/// template with every chapter, and nothing else; the cover counts the chapters right; and
/// nothing prints the page as it loads.
fn page_problems(html: &str) -> Vec<String> {
    let (open, close) = match main_text(html) {
        Ok(found) => found,
        Err(why) => return vec![why],
    };
    let text = html[open..close].trim();
    let mut problems = Vec::new();
    if html.contains(AUTO_PRINT_CALL) {
        problems.push(format!(
            "a script that prints the page as it loads (`{AUTO_PRINT_CALL}`)"
        ));
    }
    let found = text.matches(BOOK).count();
    let Some(at) = text.find(BOOK).filter(|_| found == 1) else {
        problems.push(format!(
            "the template the chapters wait in (`{BOOK}`) is there {found} times, not once"
        ));
        return problems;
    };
    let Some(book) = text[at + BOOK.len()..].strip_suffix(BOOK_END) else {
        problems.push(format!(
            "the page's text doesn't end with the template's end (`{BOOK_END}`)"
        ));
        return problems;
    };
    let chapters = titles(book);
    if chapters == 0 {
        problems.push("no chapter (`h1`) in the template".to_string());
    }
    if book.contains("<template") || book.contains("<script") {
        problems.push("a template or a script among the chapters".to_string());
    }
    let before = &text[..at];
    if !before.starts_with(COVER) {
        problems.push(format!(
            "the page's text doesn't open with the cover (`{COVER}`)"
        ));
    } else if before != cover(chapters) {
        problems.push(format!(
            "the cover isn't the one for {chapters} chapters, as the template holds"
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A print page as the anatomy leaves it: two chapters, the break between them.
    const PRINTED: &str = "<html><body><div id=\"mdbook-content\"><main>\n\
        <div class=\"fs-intro\"><p class=\"fs-tag\">t</p><h1 id=\"a\">A</h1><p>a</p></div>\n\
        <div style=\"break-before: page; page-break-before: always;\"></div>\
        <div class=\"fs-intro\"><p class=\"fs-tag\">t</p><h1 id=\"b\">B</h1><p>b</p></div>\n\
        </main><nav></nav></div><script src=\"book.js\"></script>\n\n        <script>
        window.addEventListener('load', function() {
            window.setTimeout(window.print, 100);
        });
        </script>\n</body></html>";

    #[test]
    fn the_chapters_wait_in_a_template_after_the_cover() {
        let (drawn, chapters) = rewrite(PRINTED).unwrap();
        assert_eq!(chapters, 2);
        assert!(
            page_problems(&drawn).is_empty(),
            "{:?}",
            page_problems(&drawn)
        );
        let (open, close) = main_text(&drawn).unwrap();
        let text = &drawn[open..close];
        let book = text.find(BOOK).unwrap();
        // The cover alone stands before the template, and it counts both chapters.
        assert_eq!(titles(&text[..book]), 1);
        assert!(text[..book].contains("its 2 chapters"));
        assert_eq!(titles(&text[book..]), 2);
        // Everything outside `main` is as it was, but for mdBook's script that prints as the
        // page loads.
        assert!(drawn.ends_with(
            "</main><nav></nav></div><script src=\"book.js\"></script>\n\n        \n</body></html>"
        ));
        assert!(!drawn.contains("window.print"));
        assert!(drawn.starts_with("<html><body><div id=\"mdbook-content\"><main>\n"));
    }

    #[test]
    fn a_page_the_template_cant_hold_is_refused() {
        let script = PRINTED.replace("<p>b</p>", "<p>b</p><script>go()</script>");
        assert!(rewrite(&script).unwrap_err().contains("a script"));
        let template = PRINTED.replace("<p>b</p>", "<template></template>");
        assert!(rewrite(&template).unwrap_err().contains("a template"));
        let untitled = PRINTED
            .replace("<h1 id=\"a\">A</h1>", "")
            .replace("<h1 id=\"b\">B</h1>", "");
        assert!(rewrite(&untitled).unwrap_err().contains("no chapter"));
        let twice = PRINTED.replace("<nav></nav>", "<main></main>");
        assert!(rewrite(&twice).unwrap_err().contains("2 times"));
        let swapped = PRINTED
            .replace("<main>", "<i>")
            .replace("</main>", "<main>")
            .replace("<i>", "</main>");
        assert!(rewrite(&swapped).unwrap_err().contains("comes before"));
        // mdBook's script that prints as the page loads, changed or gone.
        let changed = PRINTED.replace("window.print, 100", "window.print, 200");
        assert!(rewrite(&changed).unwrap_err().contains("0 times"));
        let gone = PRINTED.replace(AUTO_PRINT, "");
        assert!(rewrite(&gone).unwrap_err().contains("0 times"));
        // An `h1` is counted by its tag, not by a longer name that starts the same.
        assert_eq!(titles("<h1x></h1x><h1>a</h1><h1 id=\"b\">b</h1>"), 2);
    }

    #[test]
    fn the_check_fails_a_page_without_its_cover_or_template() {
        let (drawn, _) = rewrite(PRINTED).unwrap();
        // mdBook's page as it builds it: no template, and the script that prints as it loads.
        let built = page_problems(PRINTED);
        assert!(
            built[0].contains("prints the page as it loads"),
            "{built:?}"
        );
        assert!(built[1].contains("there 0 times"), "{built:?}");
        // The script put back.
        let printing = drawn.replace(
            "</body>",
            "<script>window.setTimeout(window.print, 9)</script></body>",
        );
        assert!(page_problems(&printing)[0].contains("prints the page as it loads"));
        // A cover counting the chapters wrong.
        let miscounted = drawn.replace("its 2 chapters", "its 3 chapters");
        assert!(page_problems(&miscounted)[0].contains("for 2 chapters"));
        // Text left after the template, in `main`.
        let after = drawn.replace("</template>\n", "</template>\n<p>left</p>");
        assert!(page_problems(&after)[0].contains("doesn't end with"));
        // No cover.
        let bare = drawn.replace(COVER, "<div>");
        assert!(page_problems(&bare)[0].contains("doesn't open with the cover"));
        // A chapter with a script.
        let script = drawn.replace("<p>b</p>", "<p>b</p><script>go()</script>");
        assert!(page_problems(&script)[0].contains("a script"));
        // An empty template.
        let (open, close) = main_text(&drawn).unwrap();
        let empty = format!(
            "{}\n{}{BOOK}{BOOK_END}\n{}",
            &drawn[..open],
            cover(0),
            &drawn[close..]
        );
        assert!(page_problems(&empty)[0].contains("no chapter"));
    }
}
