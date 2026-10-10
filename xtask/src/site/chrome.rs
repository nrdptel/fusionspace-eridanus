//! The page's chrome (#383): its icons and its title. mdBook 0.5 draws its menu bar's buttons,
//! the page arrows, the search spinner and the code blocks' buttons with Font Awesome glyphs,
//! inline, each in a `<span class=fa-svg>`, and names the book in an `h1` above the page's own.
//! The product system draws icons from its own set, on a 24 px grid in the text's color
//! (`foundations.md`, *Icons*; `web.md`, *Using it in a project*), and gives a page one `h1`
//! (`web.md`, *Accessibility*).
//!
//! After mdBook builds the site, [`draw`] swaps each glyph in the guide's pages for the system's
//! icon of the same meaning, from `theme/icons/`, where each is copied unchanged from the system
//! (`theme.rs` compares them with the pinned revision), and makes the menu bar's title a
//! paragraph. A glyph in a place it doesn't know fails the build, so a newer mdBook's glyph can't
//! slip through. [`check`] then fails a built page with a Font Awesome glyph, an icon that isn't
//! the system's, or an `h1` other than its own title. `theme/hpr.css` sizes the icons, and the
//! page check fails one drawn at a size other than the system's 16, 20 or 24 px.
//!
//! `print.html`, every chapter on one page for printing, keeps each chapter's `h1`, as a printed
//! book opens each chapter with its title; it loses the menu bar's.
//!
//! The checks read the glyphs' spans only. The copy button's icon is painted through a mask from
//! `docs/images/copy.svg` (`theme/hpr.css`), and mdBook's script and stylesheets still draw three
//! text glyphs (`❱` on the sidebar's fold toggles, `✓` beside the chosen theme, `»` before a
//! heading jumped to), which no check reads yet (#383).

use std::fs;
use std::path::Path;

use super::html_files;

/// How mdBook 0.5 opens the span around a glyph, up to its attributes.
const GLYPH_OPEN: &str = "<span class=fa-svg";
/// The notice inside each Font Awesome glyph mdBook inlines, wherever it stands.
const GLYPH_NOTICE: &str = "<!--! Font Awesome";

/// How mdBook 0.5 opens and closes the menu bar's title, and what [`draw`] makes of it.
const TITLE: (&str, &str) = ("<h1 class=\"menu-title\">", "</h1>");
const TITLE_DRAWN: (&str, &str) = ("<p class=\"menu-title\">", "</p>");

/// The page that holds every chapter, which keeps each chapter's `h1`.
const PRINT_PAGE: &str = "print.html";
/// The sidebar alone, which mdBook loads in a frame for a reader without scripts: no page.
const SIDEBAR_FRAME: &str = "toc.html";

/// The system's icons the chrome draws, by name, as `theme/icons/` holds them.
const ICONS: [(&str, &str); 12] = [
    ("chevron", include_str!("../../../theme/icons/chevron.svg")),
    ("copy", include_str!("../../../theme/icons/copy.svg")),
    ("edit", include_str!("../../../theme/icons/edit.svg")),
    (
        "external",
        include_str!("../../../theme/icons/external.svg"),
    ),
    ("menu", include_str!("../../../theme/icons/menu.svg")),
    ("minus", include_str!("../../../theme/icons/minus.svg")),
    ("plus", include_str!("../../../theme/icons/plus.svg")),
    ("print", include_str!("../../../theme/icons/print.svg")),
    ("refresh", include_str!("../../../theme/icons/refresh.svg")),
    ("search", include_str!("../../../theme/icons/search.svg")),
    (
        "simulate",
        include_str!("../../../theme/icons/simulate.svg"),
    ),
    ("sun", include_str!("../../../theme/icons/sun.svg")),
];

/// Where mdBook 0.5 draws a glyph, by a mark in the opening tag the glyph's span sits in (or, for
/// the spinner, the span's own `id`), and the system's icon drawn there. The book's repository
/// link takes `external`, as it leaves the site: the system draws no maker's logos. The code
/// blocks' templates take `plus` and `minus` to show and hide a block's hidden lines, `simulate`
/// to run it and `refresh` to undo an edit; the site runs no code, so those three aren't drawn.
const PLACES: [(&str, &str); 14] = [
    ("id=\"mdbook-sidebar-toggle\"", "menu"),
    ("id=\"mdbook-theme-toggle\"", "sun"),
    ("id=\"mdbook-search-toggle\"", "search"),
    ("title=\"Print this book\"", "print"),
    ("title=\"Git repository\"", "external"),
    ("rel=\"edit\"", "edit"),
    ("id=\"fa-spin\"", "refresh"),
    ("rel=\"prev\"", "chevron"),
    ("rel=\"next", "chevron"),
    ("<template id=fa-eye>", "plus"),
    ("<template id=fa-eye-slash>", "minus"),
    ("<template id=fa-copy>", "copy"),
    ("<template id=fa-play>", "simulate"),
    ("<template id=fa-clock-rotate-left>", "refresh"),
];

/// The system's icon `name`, as it is inlined: the file without its closing line break.
fn icon(name: &str) -> Option<&'static str> {
    ICONS
        .iter()
        .find(|(icon, _)| *icon == name)
        .map(|(_, svg)| svg.trim_end())
}

/// Draws the chrome of every page of the guide under `output` (the API reference, under `api/`,
/// is rustdoc's) with the system's icons and one `h1`. Returns how many glyphs it swapped, or
/// every page's problems: a glyph in a place [`PLACES`] doesn't name, or one not closed.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        match rewrite(&html) {
            Ok((drawn, glyphs)) => {
                if drawn != html {
                    fs::write(&path, drawn)
                        .map_err(|err| format!("could not write {}: {err}", path.display()))?;
                }
                count += glyphs;
            }
            Err(why) => problems.extend(why.into_iter().map(|why| format!("{name}: {why}"))),
        }
    }
    if problems.is_empty() {
        Ok(count)
    } else {
        Err(format!(
            "{} glyph(s) the site can't draw as the system's icons (name each new place in \
             `PLACES`, in xtask/src/site/chrome.rs, with the system's icon of its meaning):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// One page's HTML with its glyphs swapped for the system's icons and the menu bar's title made
/// a paragraph, and how many glyphs; or why a glyph can't be swapped.
fn rewrite(html: &str) -> Result<(String, usize), Vec<String>> {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    let mut count = 0;
    let mut problems = Vec::new();
    while let Some(at) = rest.find(GLYPH_OPEN) {
        let before = &rest[..at];
        out.push_str(before);
        let span = &rest[at..];
        let Some(open_end) = span.find('>') else {
            problems.push("a glyph's span is never closed".to_string());
            return Err(problems);
        };
        let open = &span[..=open_end];
        let Some(close) = span.find("</span>") else {
            problems.push(format!("`{open}` is never closed"));
            return Err(problems);
        };
        // The tag the span sits in: the last opening tag before it, or the span's own.
        let parent = {
            let text = out.trim_end();
            text.rfind('<').map_or("", |start| &text[start..])
        };
        let found: Vec<&str> = PLACES
            .iter()
            .filter(|(mark, _)| open.contains(mark) || parent.contains(mark))
            .map(|(_, name)| *name)
            .collect();
        match found.as_slice() {
            [name] => match icon(name) {
                Some(svg) => {
                    out.push_str(open);
                    out.push_str(svg);
                    out.push_str("</span>");
                    count += 1;
                }
                None => problems.push(format!("`{name}` isn't one of the system's icons here")),
            },
            [] => problems.push(format!(
                "a glyph in `{}`, a place not named",
                snippet(parent)
            )),
            _ => problems.push(format!(
                "a glyph in `{}`, a place named {} times",
                snippet(parent),
                found.len()
            )),
        }
        rest = &span[close + "</span>".len()..];
    }
    out.push_str(rest);
    if !problems.is_empty() {
        return Err(problems);
    }
    let out = match out.find(TITLE.0) {
        Some(start) => match out[start..].find(TITLE.1) {
            Some(end) => {
                let inner = &out[start + TITLE.0.len()..start + end];
                format!(
                    "{}{}{inner}{}{}",
                    &out[..start],
                    TITLE_DRAWN.0,
                    TITLE_DRAWN.1,
                    &out[start + end + TITLE.1.len()..]
                )
            }
            None => return Err(vec!["the menu bar's title is never closed".to_string()]),
        },
        None => out,
    };
    Ok((out, count))
}

/// The first few characters of `text`, for a message.
fn snippet(text: &str) -> &str {
    let end = text
        .char_indices()
        .nth(80)
        .map_or(text.len(), |(index, _)| index);
    &text[..end]
}

/// Checks the guide's built pages under `output`: what a page's chrome still draws that isn't
/// the system's.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut problems = Vec::new();
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        problems.extend(
            page_problems(name, &html)
                .into_iter()
                .map(|why| format!("{name}: {why}")),
        );
    }
    Ok(problems)
}

/// What one built page named `name` draws that isn't the system's: a Font Awesome glyph, a
/// glyph's span holding anything but one of the system's icons, or an `h1` other than the page's
/// title (on [`PRINT_PAGE`], one that doesn't open a chapter).
fn page_problems(name: &str, html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let glyphs = html.matches(GLYPH_NOTICE).count();
    if glyphs > 0 {
        problems.push(format!("{glyphs} Font Awesome glyph(s)"));
    }
    let mut rest = html;
    while let Some(at) = rest.find(GLYPH_OPEN) {
        let span = &rest[at..];
        let inner = span
            .find('>')
            .zip(span.find("</span>"))
            .filter(|(open_end, close)| open_end < close)
            .map(|(open_end, close)| &span[open_end + 1..close]);
        match inner {
            Some(inner) if ICONS.iter().any(|(_, svg)| svg.trim_end() == inner) => {}
            Some(inner) => problems.push(format!(
                "an icon that isn't one of the system's: `{}`",
                snippet(inner)
            )),
            None => problems.push("an icon's span is never closed".to_string()),
        }
        rest = &span[GLYPH_OPEN.len()..];
    }
    if name == SIDEBAR_FRAME {
        return problems;
    }
    let headings: Vec<&str> = html
        .match_indices("<h1")
        .map(|(at, _)| &html[at..])
        .filter(|tag| tag[3..].starts_with([' ', '>']))
        .collect();
    if name == PRINT_PAGE {
        let stray = headings
            .iter()
            .filter(|tag| !tag.starts_with("<h1 id="))
            .count();
        if headings.is_empty() || stray > 0 {
            problems.push(format!(
                "{} `h1` that open no chapter, of {} (the page holds every chapter, each \
                 opening at its own `h1`)",
                stray,
                headings.len()
            ));
        }
    } else if headings.len() != 1 {
        problems.push(format!(
            "{} `h1` (a page has one, its title)",
            headings.len()
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A page's chrome as mdBook 0.5 builds it, with `glyph` in each glyph's span and `title`'s
    /// tags around the book's name, then the page's own title.
    fn page(glyph: &str, title: (&str, &str)) -> String {
        format!(
            "<div class=\"left-buttons\">\n<label id=\"mdbook-sidebar-toggle\" class=\"icon-button\">\n\
             <span class=fa-svg>{glyph}</span>\n</label>\n<button id=\"mdbook-theme-toggle\" \
             class=\"icon-button\" type=\"button\" title=\"Change theme\">\n\
             <span class=fa-svg>{glyph}</span>\n</button>\n<button id=\"mdbook-search-toggle\">\n\
             <span class=fa-svg>{glyph}</span>\n</button>\n</div>\n{}FusionSpace HPR{}\n\
             <a href=\"print.html\" title=\"Print this book\" aria-label=\"Print this book\">\n\
             <span class=fa-svg id=\"print-button\">{glyph}</span>\n</a>\n\
             <a href=\"https://example.org/book\" title=\"Git repository\" aria-label=\"Git \
             repository\">\n<span class=fa-svg>{glyph}</span>\n</a>\n\
             <a href=\"https://example.org/edit\" title=\"Suggest an edit\" aria-label=\"Suggest \
             an edit\" rel=\"edit\">\n<span class=fa-svg id=\"git-edit-button\">{glyph}</span>\n</a>\n\
             <div class=\"spinner-wrapper\">\n<span class=fa-svg id=\"fa-spin\">{glyph}</span>\n</div>\n\
             <main><h1 id=\"a-page\"><a class=\"header\" href=\"#a-page\">A page</a></h1></main>\n\
             <a rel=\"prev\" href=\"a.html\" class=\"nav-chapters previous\"><span class=fa-svg>{glyph}</span></a>\n\
             <a rel=\"next prefetch\" href=\"b.html\" class=\"nav-chapters next\"><span class=fa-svg>{glyph}</span></a>\n\
             <template id=fa-eye><span class=fa-svg>{glyph}</span></template>\n\
             <template id=fa-eye-slash><span class=fa-svg>{glyph}</span></template>\n\
             <template id=fa-copy><span class=fa-svg>{glyph}</span></template>\n\
             <template id=fa-play><span class=fa-svg>{glyph}</span></template>\n\
             <template id=fa-clock-rotate-left><span class=fa-svg>{glyph}</span></template>\n",
            title.0, title.1
        )
    }

    /// A glyph as mdBook 0.5 inlines one, with Font Awesome's notice, its paths cut.
    const GLYPH: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 512 512\"><!--! \
                         Font Awesome Free 6.2.0 by @fontawesome --><path d=\"M0 0\"/></svg>";

    #[test]
    fn every_glyph_takes_the_systems_icon_of_its_place() {
        let (drawn, count) = rewrite(&page(GLYPH, TITLE)).unwrap();
        assert_eq!(count, 14);
        assert!(!drawn.contains(GLYPH_NOTICE), "{drawn}");
        for (mark, name) in [
            ("mdbook-sidebar-toggle", "menu"),
            ("mdbook-theme-toggle", "sun"),
            ("mdbook-search-toggle", "search"),
            ("print-button", "print"),
            ("title=\"Git repository\"", "external"),
            ("git-edit-button", "edit"),
            ("fa-spin", "refresh"),
            ("rel=\"prev\"", "chevron"),
            ("rel=\"next", "chevron"),
            ("<template id=fa-eye>", "plus"),
            ("<template id=fa-eye-slash>", "minus"),
            ("<template id=fa-copy>", "copy"),
            ("<template id=fa-play>", "simulate"),
            ("<template id=fa-clock-rotate-left>", "refresh"),
        ] {
            let at = drawn.find(mark).unwrap();
            let svg = &drawn[at..][drawn[at..].find("<svg").unwrap()..];
            assert!(
                svg.starts_with(icon(name).unwrap()),
                "{mark} should draw {name}: {svg}"
            );
        }
        assert!(page_problems("a.html", &drawn).is_empty(), "{drawn}");
    }

    #[test]
    fn the_menu_bars_title_leaves_the_page_one_h1() {
        let (drawn, _) = rewrite(&page(GLYPH, TITLE)).unwrap();
        assert!(
            drawn.contains("<p class=\"menu-title\">FusionSpace HPR</p>"),
            "{drawn}"
        );
        assert_eq!(drawn.matches("<h1").count(), 1);
        let kept = page(icon("menu").unwrap(), TITLE);
        assert_eq!(
            page_problems("a.html", &kept),
            ["2 `h1` (a page has one, its title)"]
        );
    }

    #[test]
    fn a_glyph_in_a_place_not_named_fails() {
        let html =
            format!("<button id=\"mdbook-new-button\"><span class=fa-svg>{GLYPH}</span></button>");
        let why = rewrite(&html).unwrap_err();
        assert_eq!(why.len(), 1);
        assert!(why[0].contains("a place not named"), "{why:?}");
        assert!(why[0].contains("mdbook-new-button"), "{why:?}");
    }

    #[test]
    fn a_glyph_named_by_two_places_fails() {
        let html = format!(
            "<a rel=\"edit\" title=\"Git repository\"><span class=fa-svg>{GLYPH}</span></a>"
        );
        let why = rewrite(&html).unwrap_err();
        assert!(why[0].contains("named 2 times"), "{why:?}");
    }

    #[test]
    fn every_place_draws_an_icon_the_set_holds() {
        for (mark, name) in PLACES {
            assert!(
                icon(name).is_some(),
                "{mark} names {name}, which ICONS lacks"
            );
        }
        for (name, svg) in ICONS {
            assert!(
                svg.starts_with("<svg ")
                    && svg.contains("viewBox=\"0 0 24 24\"")
                    && svg.contains("stroke=\"currentColor\""),
                "{name} isn't drawn on the system's grid in the text's color"
            );
        }
    }

    #[test]
    fn a_glyph_left_or_an_icon_not_the_systems_fails() {
        let left = page(GLYPH, TITLE_DRAWN);
        let why = page_problems("a.html", &left);
        assert_eq!(why[0], "14 Font Awesome glyph(s)");
        assert_eq!(why.len(), 15, "{why:?}");
        let changed = icon("menu")
            .unwrap()
            .replace("stroke-width=\"1.5\"", "stroke-width=\"2\"");
        let why = page_problems("a.html", &page(&changed, TITLE_DRAWN));
        assert_eq!(why.len(), 14, "{why:?}");
        assert!(
            why[0].starts_with("an icon that isn't one of the system's"),
            "{why:?}"
        );
        assert!(page_problems("a.html", &page(icon("menu").unwrap(), TITLE_DRAWN)).is_empty());
        // Prose that names Font Awesome is no glyph.
        let named = format!(
            "{}<p>Font Awesome's glyphs</p>",
            page(icon("menu").unwrap(), TITLE_DRAWN)
        );
        assert!(page_problems("a.html", &named).is_empty());
    }

    #[test]
    fn the_print_page_keeps_each_chapters_h1_and_loses_the_menu_bars() {
        let chapters = "<h1 id=\"a\">A</h1><h1 id=\"b\">B</h1>";
        let menu = icon("menu").unwrap();
        let drawn = format!("{}{chapters}", page(menu, TITLE_DRAWN));
        assert!(page_problems(PRINT_PAGE, &drawn).is_empty());
        let kept = format!("{}{chapters}", page(menu, TITLE));
        assert_eq!(
            page_problems(PRINT_PAGE, &kept),
            [
                "1 `h1` that open no chapter, of 4 (the page holds every chapter, each opening at \
              its own `h1`)"
            ]
        );
        assert_eq!(
            page_problems("a.html", &format!("{}{chapters}", page(menu, TITLE_DRAWN))).len(),
            1
        );
    }

    #[test]
    fn a_page_without_glyphs_is_unchanged() {
        let html = "<main><h1 id=\"a\">A</h1><p>Text.</p></main>";
        assert_eq!(rewrite(html).unwrap(), (html.to_string(), 0));
    }
}
