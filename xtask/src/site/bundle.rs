//! One stylesheet a page (#443's LCP; ADR-232): after mdBook builds the site, [`draw`] joins the
//! stylesheets every page links, mdBook's and the theme's, into one file, `theme/site.css`, in
//! the order the pages link them, and links that file first in the page's head.
//!
//! A page can't be drawn until each stylesheet it links has arrived, and a phone on a slow
//! network waits about half a second before each request is answered. Ten stylesheets, the
//! page's first script and the fonts its first screen draws in are more requests than a browser
//! sends to one server at once over HTTP/1.1, so they arrived in rounds and a phone's first paint
//! came after 2.5 s; one stylesheet lets them go out together.
//!
//! - **Nothing is changed but where the rules come from.** Each file is copied whole, with its
//!   relative `url()`s rewritten to resolve from `theme/`. The files stay where mdBook put them.
//! - **The code colors are the theme's in both looks.** mdBook links three highlighting
//!   stylesheets and its script turns two of them off for the reader's theme; the theme's own
//!   rules (`theme/hpr.css`) give code its colors over all three. The bundle takes the first,
//!   `highlight.css`, for both looks, and the other two leave only their ids, on empty `<style>`
//!   elements, which mdBook's script still turns on and off.
//! - **Print's rules keep their place.** mdBook links `css/print.css` for print only, between
//!   its own stylesheets and the theme's, whose print rules override it; the bundle carries it
//!   there, inside `@media print`.
//! - **The stylesheet and the sidebar's script are asked for first.** A page can't be drawn
//!   before its stylesheet arrives, nor read past the top of its body before mdBook's sidebar
//!   script (`toc.js`) arrives, while the fonts it preloads (`theme/head.hbs`) can arrive after
//!   the text is first drawn. The link to the bundle and a preload of that script stand first in
//!   the head's additions, before the fonts, so a browser asks for them first. Over HTTP/1.1 the
//!   page, the stylesheet, the script and the first three fonts take the six connections a
//!   browser opens to one server, and the fourth font waits. With the fonts first, the
//!   stylesheet or the script waited a round trip on a long page, whose own text holds its
//!   connection longest: the aerodynamics page and the print page first drew at 2.4 to 2.5 s in
//!   the vitals check, and at 2.0 s with them first.
//!
//! [`check`] fails a built site whose `theme/site.css` isn't the join of the files as served, a
//! page that links one of the joined files itself, or not the bundle once, or that asks for a
//! font before the bundle, or before the sidebar's script on a page that loads it.

use std::fs;
use std::path::Path;

use super::html_files;

/// The bundle, in the site.
pub(super) const BUNDLE: &str = "theme/site.css";

/// mdBook 0.5's comment before the theme's additions to the head (`theme/head.hbs`), the font
/// preloads among them; [`draw`] asks for the bundle and the sidebar's script right after it.
const CUSTOM_HEAD: &str = "<!-- Custom HTML head -->";

/// mdBook 0.5's sidebar script, which it loads at the top of the body, and how it does.
const SIDEBAR_SCRIPT: &str = "toc.js";

/// How a page asks for a font before its stylesheet names it (`theme/head.hbs`).
const FONT_PRELOAD: &str = "<link rel=\"preload\" href=\"";
const FONT_PRELOAD_KIND: &str = "as=\"font\"";

/// A stylesheet a page links.
struct Sheet {
    /// Its path in the site.
    path: &'static str,
    /// The id mdBook gives its link, if any.
    id: Option<&'static str>,
    /// The media its link is for, if not all.
    media: Option<&'static str>,
    /// Whether the bundle carries its rules.
    joined: bool,
}

const fn sheet(path: &'static str) -> Sheet {
    Sheet {
        path,
        id: None,
        media: None,
        joined: true,
    }
}

/// The stylesheets a page links, in order.
const PARTS: [Sheet; 10] = [
    sheet("css/variables.css"),
    sheet("css/general.css"),
    sheet("css/chrome.css"),
    Sheet {
        media: Some("print"),
        ..sheet("css/print.css")
    },
    sheet("fonts/fonts.css"),
    Sheet {
        id: Some("mdbook-highlight-css"),
        ..sheet("highlight.css")
    },
    Sheet {
        id: Some("mdbook-tomorrow-night-css"),
        joined: false,
        ..sheet("tomorrow-night.css")
    },
    Sheet {
        id: Some("mdbook-ayu-highlight-css"),
        joined: false,
        ..sheet("ayu-highlight.css")
    },
    sheet("theme/fusionspace-mdbook.css"),
    sheet("theme/hpr.css"),
];

/// The paths of the stylesheets the bundle carries, in order.
pub(super) fn parts() -> impl Iterator<Item = &'static str> {
    PARTS
        .iter()
        .filter(|sheet| sheet.joined)
        .map(|sheet| sheet.path)
}

/// Writes the bundle into the site built in `output` and links it from every page in place of
/// the files it joins. Returns how many pages link it.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let bundle = join(output)?;
    let path = output.join(BUNDLE);
    fs::write(&path, bundle).map_err(|err| format!("could not write {}: {err}", path.display()))?;
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        match rewrite(&html) {
            Ok(Some(linked)) => {
                fs::write(&path, linked)
                    .map_err(|err| format!("could not write {}: {err}", path.display()))?;
                count += 1;
            }
            Ok(None) => {}
            Err(why) => problems.push(format!("{name}: {why}")),
        }
    }
    if problems.is_empty() {
        Ok(count)
    } else {
        Err(format!(
            "{} page(s) whose stylesheets can't be joined (xtask/src/site/bundle.rs):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// The bundle of the site built in `output`: each joined file as served, its relative `url()`s
/// rewritten from its folder to the bundle's.
fn join(output: &Path) -> Result<String, String> {
    let mut bundle = format!(
        "/* The site's stylesheets for the screen, joined in the order its pages linked them, so \
         a page asks for one (#443). Written by `cargo xtask site` (xtask/src/site/bundle.rs) \
         from: {}. */\n",
        parts().collect::<Vec<_>>().join(", ")
    );
    for part in PARTS.iter().filter(|sheet| sheet.joined) {
        let path = part.path;
        let file = output.join(path);
        let css = fs::read_to_string(&file)
            .map_err(|err| format!("could not read {}: {err}", file.display()))?;
        let lower = css.to_ascii_lowercase();
        if lower.contains("@import") {
            return Err(format!(
                "{path} imports another stylesheet, which the bundle can't follow"
            ));
        }
        // `rebase` reads addresses written `url(` in lower case; another spelling, or an
        // address given as a string to `image-set()`, would keep a path the bundle breaks.
        if lower.matches("url(").count() != css.matches("url(").count()
            || lower.contains("image-set(")
        {
            return Err(format!(
                "{path} gives an address other than as `url(` in lower case, which the bundle \
                 can't rebase"
            ));
        }
        let folder = path.rsplit_once('/').map_or("", |(dir, _)| dir);
        let mut rules = rebase(&css, folder);
        if !rules.ends_with('\n') {
            rules.push('\n');
        }
        match part.media {
            Some(media) => {
                bundle.push_str(&format!("\n/* {path} */\n@media {media} {{\n{rules}}}\n"))
            }
            None => bundle.push_str(&format!("\n/* {path} */\n{rules}")),
        }
    }
    Ok(bundle)
}

/// `css` with each relative address in a `url()` rewritten from the folder `from` (relative to
/// the site, `""` for its root) to the bundle's folder, `theme`. Addresses with a scheme
/// (`data:`, `https:`), from the site's root (`/`) or to a fragment (`#`) are left alone.
fn rebase(css: &str, from: &str) -> String {
    let theme = BUNDLE.rsplit_once('/').map_or("", |(dir, _)| dir);
    if from == theme {
        return css.to_owned();
    }
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(at) = rest.find("url(") {
        let (before, after) = rest.split_at(at + "url(".len());
        out.push_str(before);
        let inner = after.trim_start();
        out.push_str(&after[..after.len() - inner.len()]);
        let quote = inner.chars().next().filter(|c| *c == '"' || *c == '\'');
        let body = quote.map_or(inner, |q| &inner[q.len_utf8()..]);
        let end = match quote {
            Some(q) => body.find(q),
            None => body.find(')'),
        };
        let Some(end) = end else {
            out.push_str(inner);
            return out;
        };
        let address = body[..end].trim_end();
        if let Some(q) = quote {
            out.push(q);
        }
        if address.is_empty() || address.starts_with(['/', '#']) || has_scheme(address) {
            out.push_str(address);
        } else {
            let mut path = String::from("../");
            if !from.is_empty() {
                path.push_str(from);
                path.push('/');
            }
            path.push_str(address);
            out.push_str(&path);
        }
        rest = &body[address.len()..];
    }
    out.push_str(rest);
    out
}

/// Whether an address starts with a scheme, `name:`.
fn has_scheme(address: &str) -> bool {
    address.split_once(':').is_some_and(|(scheme, _)| {
        !scheme.is_empty()
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    })
}

/// A stylesheet link as mdBook writes it, for `href`, with `id` and `media` if it has them.
fn link(href: &str, id: Option<&str>, media: Option<&str>) -> String {
    let id = id.map_or_else(String::new, |id| format!(" id=\"{id}\""));
    let media = media.map_or_else(String::new, |media| format!(" media=\"{media}\""));
    format!("<link rel=\"stylesheet\"{id} href=\"{href}\"{media}>")
}

/// One page's HTML with its stylesheets replaced by the bundle; `None` for a page that links
/// none of them (an API page), and what is off for one that links them otherwise than mdBook
/// does.
/// A preload of the script at `src`.
fn preload_script(src: &str) -> String {
    format!("<link rel=\"preload\" href=\"{src}\" as=\"script\">")
}

/// The address the page loads the sidebar script from, if it does.
fn sidebar_script(html: &str) -> Option<&str> {
    html.match_indices("<script src=\"")
        .map(|(at, open)| &html[at + open.len()..])
        .filter_map(|rest| rest.find('"').map(|end| &rest[..end]))
        .find(|src| src.rsplit('/').next() == Some(SIDEBAR_SCRIPT))
}

/// Where the page's first font preload starts, if it has one.
fn first_font(html: &str) -> Option<usize> {
    html.match_indices(FONT_PRELOAD)
        .map(|(at, _)| at)
        .find(|&at| {
            html[at..]
                .find('>')
                .is_some_and(|end| html[at..at + end].contains(FONT_PRELOAD_KIND))
        })
}

fn rewrite(html: &str) -> Result<Option<String>, String> {
    let first = format!("{}\">", PARTS[0].path);
    let Some(at) = html.find(&first) else {
        return Ok(None);
    };
    let tag = html[..at]
        .rfind("<link rel=\"stylesheet\" href=\"")
        .ok_or("its first stylesheet link isn't written as mdBook writes it")?;
    let prefix = &html[tag + "<link rel=\"stylesheet\" href=\"".len()..at];
    let mut out = html.to_owned();
    for part in &PARTS {
        let href = format!("{prefix}{}", part.path);
        let tag = link(&href, part.id, part.media);
        match out.matches(&tag).count() {
            // The table of contents has no highlighting.
            0 if part.id.is_some() => continue,
            1 => {}
            n => return Err(format!("links `{tag}` {n} times, not once")),
        }
        let with = match part.id {
            Some(id) => format!("<style id=\"{id}\"></style>"),
            None => String::new(),
        };
        out = out.replacen(&tag, &with, 1);
    }
    let at = match out.matches(CUSTOM_HEAD).count() {
        1 => out.find(CUSTOM_HEAD).map_or(0, |at| at + CUSTOM_HEAD.len()),
        n => {
            return Err(format!(
                "holds mdBook's `{CUSTOM_HEAD}` {n} times, not once"
            ));
        }
    };
    let mut asked = format!(
        "\n        {}",
        link(&format!("{prefix}{BUNDLE}"), None, None)
    );
    if let Some(src) = sidebar_script(&out) {
        asked.push_str(&format!("\n        {}", preload_script(src)));
    }
    out.insert_str(at, &asked);
    Ok(Some(out))
}

/// What is wrong with the bundle in the site built in `output`, and with each page's links.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    let served = output.join(BUNDLE);
    match fs::read_to_string(&served) {
        Ok(text) if text == join(output)? => {}
        Ok(_) => problems.push(format!(
            "{BUNDLE} is not the join of {} as the site serves them",
            parts().collect::<Vec<_>>().join(", ")
        )),
        Err(err) => problems.push(format!("{BUNDLE}: could not read it: {err}")),
    }
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
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

/// What is wrong with one built page's stylesheets: the bundle linked other than once, one of
/// the files it joins (or drops) linked as well, or a font asked for before the bundle or the
/// sidebar's script.
fn page_problems(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let links: Vec<&str> = html
        .match_indices("<link rel=\"stylesheet\"")
        .filter_map(|(at, _)| html[at..].find('>').map(|end| &html[at..=at + end]))
        .collect();
    let bundles = links
        .iter()
        .filter(|tag| tag.contains(&format!("{BUNDLE}\"")))
        .count();
    if bundles != 1 {
        problems.push(format!("links {BUNDLE} {bundles} times, not once"));
    }
    for part in &PARTS {
        if links
            .iter()
            .any(|tag| tag.contains(&format!("{}\"", part.path)))
        {
            problems.push(format!("links {}, which {BUNDLE} replaces", part.path));
        }
    }
    let Some(font) = first_font(html) else {
        return problems;
    };
    let bundle = html
        .match_indices("<link rel=\"stylesheet\"")
        .map(|(at, _)| at)
        .find(|&at| {
            html[at..]
                .find('>')
                .is_some_and(|end| html[at..at + end].contains(BUNDLE))
        });
    if bundle.is_some_and(|at| at > font) {
        problems.push(format!("asks for a font before {BUNDLE}"));
    }
    if let Some(src) = sidebar_script(html) {
        let preload = preload_script(src);
        match html.matches(&preload).count() {
            1 if html.find(&preload).is_some_and(|at| at > font) => {
                problems.push(format!("asks for a font before {src}"));
            }
            1 => {}
            n => problems.push(format!("preloads {src} {n} times, not once (`{preload}`)")),
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A page's head as mdBook 0.5.4 writes it, from `prefix`: the theme's additions with a font
    /// preload, then the stylesheet links; then the body's sidebar script.
    fn head(prefix: &str) -> String {
        format!(
            "<head>\n<!-- Custom HTML head -->\n\
             <link rel=\"preload\" href=\"{p}fonts/A.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n\
             <link rel=\"stylesheet\" href=\"{p}css/variables.css\">\n\
             <link rel=\"stylesheet\" href=\"{p}css/general.css\">\n\
             <link rel=\"stylesheet\" href=\"{p}css/chrome.css\">\n\
             <link rel=\"stylesheet\" href=\"{p}css/print.css\" media=\"print\">\n\
             <link rel=\"stylesheet\" href=\"{p}fonts/fonts.css\">\n\
             <link rel=\"stylesheet\" id=\"mdbook-highlight-css\" href=\"{p}highlight.css\">\n\
             <link rel=\"stylesheet\" id=\"mdbook-tomorrow-night-css\" href=\"{p}tomorrow-night.css\">\n\
             <link rel=\"stylesheet\" id=\"mdbook-ayu-highlight-css\" href=\"{p}ayu-highlight.css\">\n\
             <link rel=\"stylesheet\" href=\"{p}theme/fusionspace-mdbook.css\">\n\
             <link rel=\"stylesheet\" href=\"{p}theme/hpr.css\">\n</head>\n\
             <body><script src=\"{p}toc.js\"></script></body>",
            p = prefix
        )
    }

    #[test]
    fn a_page_links_the_bundle_in_place_of_the_files_it_joins() {
        let linked = rewrite(&head("../")).unwrap().unwrap();
        assert!(linked.contains("<link rel=\"stylesheet\" href=\"../theme/site.css\">"));
        for id in [
            "mdbook-highlight-css",
            "mdbook-tomorrow-night-css",
            "mdbook-ayu-highlight-css",
        ] {
            assert!(
                linked.contains(&format!("<style id=\"{id}\"></style>")),
                "{id}"
            );
        }
        assert_eq!(linked.matches("rel=\"stylesheet\"").count(), 1);
        assert_eq!(page_problems(&linked), Vec::<String>::new());
        // At the root, and on the table of contents, which has no highlighting.
        let root = rewrite(&head("")).unwrap().unwrap();
        assert!(root.contains("<link rel=\"stylesheet\" href=\"theme/site.css\">"));
        let toc: String = head("")
            .lines()
            .filter(|line| !line.contains(" id=\"") && !line.contains("<script"))
            .collect::<Vec<_>>()
            .join("\n");
        let toc = rewrite(&toc).unwrap().unwrap();
        assert_eq!(page_problems(&toc), Vec::<String>::new());
        assert!(!toc.contains("<style"));
        // The table of contents doesn't load the sidebar's script, so doesn't preload it.
        assert!(!toc.contains("as=\"script\""));
    }

    #[test]
    fn the_bundle_and_the_sidebar_script_are_asked_for_before_the_fonts() {
        let linked = rewrite(&head("../")).unwrap().unwrap();
        let bundle = linked.find("href=\"../theme/site.css\"").unwrap();
        let script = linked
            .find("<link rel=\"preload\" href=\"../toc.js\" as=\"script\">")
            .unwrap();
        let font = linked.find("fonts/A.woff2").unwrap();
        assert!(bundle < script && script < font, "{linked}");
        assert_eq!(linked.matches("toc.js").count(), 2);
        // The check fails each asked for after a font, the script's preload missing or twice,
        // and a page whose head has no place for them.
        let late = linked
            .replacen(
                "<link rel=\"stylesheet\" href=\"../theme/site.css\">",
                "",
                1,
            )
            .replace(
                "</head>",
                "<link rel=\"stylesheet\" href=\"../theme/site.css\"></head>",
            );
        assert_eq!(
            page_problems(&late),
            ["asks for a font before theme/site.css"]
        );
        let preload = "<link rel=\"preload\" href=\"../toc.js\" as=\"script\">";
        let script_late = linked
            .replacen(preload, "", 1)
            .replace("</head>", &format!("{preload}</head>"));
        assert_eq!(
            page_problems(&script_late),
            ["asks for a font before ../toc.js"]
        );
        let unloaded = linked.replacen(preload, "", 1);
        assert!(page_problems(&unloaded)[0].contains("preloads ../toc.js 0 times"));
        let twice = linked.replacen(preload, &format!("{preload}{preload}"), 1);
        assert!(page_problems(&twice)[0].contains("2 times"));
        let unmarked = head("").replace(CUSTOM_HEAD, "");
        assert!(rewrite(&unmarked).unwrap_err().contains("0 times"));
        // A script whose name only ends the same is not the sidebar's.
        assert_eq!(sidebar_script("<script src=\"mytoc.js\"></script>"), None);
        assert_eq!(
            sidebar_script("<script src=\"../toc.js\"></script>"),
            Some("../toc.js")
        );
    }

    #[test]
    fn a_page_without_the_stylesheets_is_left_and_one_off_mdbooks_is_refused() {
        assert_eq!(rewrite("<head></head>").unwrap(), None);
        let twice = head("").replace(
            "</head>",
            "<link rel=\"stylesheet\" href=\"theme/hpr.css\">\n</head>",
        );
        assert!(rewrite(&twice).unwrap_err().contains("2 times"));
        // Print's stylesheet linked for every medium would be joined in the wrong place.
        let no_print = head("").replace(" media=\"print\"", "");
        assert!(rewrite(&no_print).unwrap_err().contains("0 times"));
    }

    #[test]
    fn the_check_fails_a_page_that_links_a_joined_file_or_not_the_bundle() {
        let linked = rewrite(&head("")).unwrap().unwrap();
        let extra = linked.replace(
            "</head>",
            "<link rel=\"stylesheet\" href=\"theme/hpr.css\"></head>",
        );
        assert_eq!(
            page_problems(&extra),
            ["links theme/hpr.css, which theme/site.css replaces"]
        );
        let none = linked.replace("theme/site.css", "theme/other.css");
        assert_eq!(
            page_problems(&none),
            ["links theme/site.css 0 times, not once"]
        );
        assert_eq!(
            page_problems(&head("")).len(),
            PARTS.len() + 2,
            "an unbundled page fails for each file, the missing bundle and the script's preload"
        );
    }

    #[test]
    fn addresses_are_rebased_from_their_folder_to_the_bundles() {
        let css = "a { b: url('X.woff2') } c { d: url(\"../images/e.svg\") } \
                   f { g: url(h.png) } i { j: url( data:image/png;base64,AA ) } \
                   k { l: url(https://example.com/m) } n { o: url(/p) } q { r: url(#s) }";
        assert_eq!(
            rebase(css, "fonts"),
            "a { b: url('../fonts/X.woff2') } c { d: url(\"../fonts/../images/e.svg\") } \
             f { g: url(../fonts/h.png) } i { j: url( data:image/png;base64,AA ) } \
             k { l: url(https://example.com/m) } n { o: url(/p) } q { r: url(#s) }"
        );
        assert_eq!(rebase("a { b: url(c.png) }", ""), "a { b: url(../c.png) }");
        // The bundle's own folder: unchanged.
        assert_eq!(rebase(css, "theme"), css);
        // An unclosed `url(` is copied as it is.
        assert_eq!(rebase("a { b: url('c", "css"), "a { b: url('c");
    }

    #[test]
    fn the_bundle_carries_every_file_but_two_highlighting_themes_in_order() {
        let joined: Vec<&str> = parts().collect();
        assert_eq!(joined.len(), PARTS.len() - 2);
        assert!(joined.contains(&"theme/hpr.css") && joined.contains(&"fonts/fonts.css"));
        assert!(!joined.contains(&"ayu-highlight.css"));
        // Print's rules before the theme's, which override them in print, and the theme's own
        // stylesheet last, so its rules win as they did.
        let at = |path: &str| joined.iter().position(|p| *p == path).unwrap();
        assert!(at("css/print.css") < at("theme/hpr.css"));
        assert_eq!(joined.last(), Some(&"theme/hpr.css"));
    }

    #[test]
    fn the_join_wraps_print_in_its_medium_and_rebases_each_file() {
        let dir = tempfile::tempdir().unwrap();
        for part in &PARTS {
            let path = dir.path().join(part.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, format!("/* {} */ a {{ b: url(x.png) }}", part.path)).unwrap();
        }
        let bundle = join(dir.path()).unwrap();
        assert!(bundle.contains(
            "/* css/print.css */\n@media print {\n/* css/print.css */ a { b: url(../css/x.png) }\n}\n"
        ));
        assert!(bundle.contains("/* theme/hpr.css */ a { b: url(x.png) }"));
        assert!(bundle.contains("/* highlight.css */ a { b: url(../x.png) }"));
        assert!(!bundle.contains("/* ayu-highlight.css */ a"));
        fs::write(dir.path().join("css/chrome.css"), "@import url(more.css);").unwrap();
        assert!(join(dir.path()).unwrap_err().contains("imports"));
        for spelling in ["a { b: URL(x.png) }", "a { b: image-set(\"x.png\" 1x) }"] {
            fs::write(dir.path().join("css/chrome.css"), spelling).unwrap();
            assert!(
                join(dir.path()).unwrap_err().contains("can't rebase"),
                "{spelling}"
            );
        }
    }
}
