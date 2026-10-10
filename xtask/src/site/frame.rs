//! The page's frame (#384): what stands above every page's text and below it. The product system
//! opens a page with its header (the horizontal lockup at 24 px in one color, Void on light and
//! Paper on dark, then the navigation, with no menu button above 720 px) and ends it with a title
//! block, its fields from ISO 7200 (`web.md`, *Page anatomy*), as the `header` and `footer`
//! landmarks (`web.md`, *Accessibility*). mdBook 0.5 draws a menu bar with the book's name, shows
//! its sidebar only from 1,080 px with a menu button below, and ends a page with its arrows.
//!
//! After mdBook builds the site, and after [`super::chrome`] has made the menu bar's title a
//! paragraph, [`draw`] rewrites every page of the guide:
//!
//! - the menu bar becomes a `header`, and its title the brand kit's one-color horizontal lockup
//!   (`theme/logo/`, copied unchanged from the system; `theme.rs` compares them with the pinned
//!   revision), drawn inline in the text's color, then the product's name, HPR, both one link to
//!   the landing page;
//! - mdBook's script that shows the sidebar from 1,080 px, after what an earlier visit left,
//!   shows it from 720 px whatever was left (`theme/hpr.css` hides the menu button there, and
//!   `theme/hpr.js` opens the sidebar when a window widens past 720 px);
//! - every page ends with the system's title block in a `footer`, after the page's arrows. Its
//!   entries are written once, at the end of the landing page's source (`theme.rs` checks them
//!   there); after the site's title each page adds its own, its `h1`, as ISO 7200's
//!   supplementary title (the page field), and the owner's entry carries the mark.
//!
//! Anything the rewrite expects and doesn't find fails the build, so a newer mdBook that changes
//! its menu bar, its sidebar script or its arrows can't slip by. [`check`] then fails a built
//! page without the header, its lockup, the 720 px sidebar script or the title block, or with a
//! title block's fields out of order, a page field not the page's title or an undated issue.
//! `xtask site`'s page check reads how each is drawn (`pages.rs`, `Kind::Header` and
//! `Kind::TitleBlock`).

use std::fs;
use std::path::Path;

use super::theme::FIELDS;
use super::{attributes, html_files};

/// The brand kit's one-color horizontal lockup and mark, as `theme/logo/` holds them.
const LOCKUP: &str = include_str!("../../../theme/logo/fusion-space-horizontal-void.svg");
const MARK: &str = include_str!("../../../theme/logo/fusion-space-mark-void.svg");
/// The one color the brand's one-color files are drawn in, Void.
const VOID: &str = "#0B0F1C";

/// How mdBook 0.5 opens its menu bar, and what [`draw`] makes of it; the bar closes with the
/// `</div>` that matches its opening.
const MENU_BAR: &str = "<div id=\"mdbook-menu-bar\" class=\"menu-bar sticky\">";
const HEADER: &str = "<header id=\"mdbook-menu-bar\" class=\"menu-bar sticky\">";
/// The menu bar's title as [`super::chrome`] leaves it, a paragraph around the book's name.
const TITLE: (&str, &str) = ("<p class=\"menu-title\">", "</p>");
/// The link the lockup and the product's name make, before its address.
const LOGO_OPEN: &str = "<a class=\"menu-title fs-logo\" href=\"";
/// The product's name, after the lockup.
const PRODUCT: &str = "<span class=\"fs-product\">HPR</span>";
/// mdBook 0.5's script that decides, before the page is drawn, whether the sidebar shows: from
/// 1,080 px, as an earlier visit left it. And what [`draw`] makes of it: from 720 px, always, by
/// the same media query the stylesheet hides the menu button with, so a window's scroll bar
/// can't leave a width with neither.
const SIDEBAR_SCRIPT: &str = "            if (document.body.clientWidth >= 1080) {
                try { sidebar = localStorage.getItem('mdbook-sidebar'); } catch(e) { }
                sidebar = sidebar || 'visible';
            } else {";
const SIDEBAR_DRAWN: &str = "            if (window.matchMedia('(min-width: 720px)').matches) {
                sidebar = 'visible';
            } else {";
/// The parts of mdBook 0.5's sidebar the frame keeps working: the sidebar, the box whose tick
/// draws it, and the menu button that ticks it under 720 px; each must be on every page once.
const SIDEBAR_PARTS: [(&str, &str); 3] = [
    ("<nav id=\"mdbook-sidebar\"", "the sidebar"),
    ("id=\"mdbook-sidebar-toggle-anchor\"", "the sidebar's box"),
    ("id=\"mdbook-sidebar-toggle\"", "the menu button"),
];
/// What mdBook 0.5 ends a page's text with: its arrows for a phone, in a `nav` of their own.
const ARROWS: &str = "<nav class=\"nav-wrapper\" aria-label=\"Page navigation\">";
/// How the title block opens, as [`draw`] writes it.
const FOOTER: &str = "<footer class=\"fs-titleblock\" aria-label=\"Title block\">";
/// How the landing page's source opens its title block, where the entries are written.
const SOURCE_BLOCK: &str = "<div class=\"title-block\">";
/// The landing page, built twice, and the page holding every chapter, which has no arrows and
/// whose title is the book's.
const LANDINGS: [&str; 2] = ["index.html", "start-here.html"];
const PRINT_PAGE: &str = "print.html";
/// The sidebar alone, which mdBook loads in a frame for a reader without scripts: no page.
const SIDEBAR_FRAME: &str = "toc.html";
/// The field each page fills with its own title, ISO 7200's supplementary title, which follows
/// the site's [`TITLE_FIELD`]; and the field whose entry carries the mark.
const PAGE: &str = "Page";
const TITLE_FIELD: &str = "Title";
const OWNER: &str = "Owner";
/// The field that gives the date of issue, which the build fills in as `YYYY-MM-DD`.
const ISSUED: &str = "Date of issue";
/// An entry whose text is longer than this takes a row of its own, so it reads as lines, not as
/// a column a few words wide.
const LONG_ENTRY_CHARS: usize = 48;

/// The brand file `svg`, drawn inline in the text's color: its paths alone, each filled with
/// `currentColor` where the file fills it with Void (and the `<svg>` itself, which a color check
/// reads too), in its own view box, hidden from screen readers (the link around it is named),
/// 24 px tall even where the stylesheet doesn't load. `class` names it for the stylesheet. Fails
/// on a file drawn in another color, moved by a transform, or holding anything but paths.
fn one_color(svg: &str, class: &str) -> Result<String, String> {
    let open = svg
        .find("<svg")
        .and_then(|start| svg[start..].find('>').map(|end| &svg[start..=start + end]))
        .ok_or("no `<svg>` element")?;
    let view_box = attributes(open, "viewBox")
        .next()
        .ok_or("no `viewBox` on its `<svg>`")?;
    if svg.contains("transform") {
        return Err("a transform, which inlining its paths would drop".to_string());
    }
    for shape in [
        "<rect",
        "<circle",
        "<ellipse",
        "<line",
        "<polyline",
        "<polygon",
        "<text",
        "<use",
        "<image",
    ] {
        if svg.contains(shape) {
            return Err(format!("a `{shape}>`; it may hold paths only"));
        }
    }
    let mut paths = String::new();
    for (at, _) in svg.match_indices("<path") {
        let tag = svg[at..]
            .find('>')
            .map(|end| &svg[at..=at + end])
            .ok_or("a `<path>` never closed")?;
        let fills: Vec<&str> = attributes(tag, "fill").collect();
        if fills != [VOID] {
            return Err(format!("a path filled {fills:?}, not Void ({VOID}) alone"));
        }
        let d = attributes(tag, "d")
            .next()
            .ok_or("a `<path>` with no `d`")?;
        paths.push_str(&format!("<path fill=\"currentColor\" d=\"{d}\"/>"));
    }
    if paths.is_empty() {
        return Err("no paths".to_string());
    }
    Ok(format!(
        "<svg class=\"{class}\" viewBox=\"{view_box}\" xmlns=\"http://www.w3.org/2000/svg\" \
         height=\"24\" fill=\"currentColor\" aria-hidden=\"true\" focusable=\"false\">{paths}</svg>"
    ))
}

/// The lockup and the mark as every page draws them.
fn drawn_logos() -> Result<(String, String), String> {
    let lockup = one_color(LOCKUP, "fs-lockup")
        .map_err(|why| format!("theme/logo/'s lockup can't be drawn inline: {why}"))?;
    let mark = one_color(MARK, "fs-mark")
        .map_err(|why| format!("theme/logo/'s mark can't be drawn inline: {why}"))?;
    Ok((lockup, mark))
}

/// The landing page's title block, as mdBook builds its source: each field with its entry, as
/// HTML, in the order written.
type Entries = Vec<(String, String)>;

/// Draws the frame of every page of the guide under `output` (the API reference, under `api/`,
/// is rustdoc's). Returns how many pages it drew, or every page's problems.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let (lockup, mark) = drawn_logos()?;
    let landing = output.join(LANDINGS[0]);
    let html = fs::read_to_string(&landing)
        .map_err(|err| format!("could not read {}: {err}", landing.display()))?;
    let entries = source_entries(&html).map_err(|why| format!("{}: {why}", LANDINGS[0]))?;
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in names
        .iter()
        .filter(|name| !name.starts_with("api/") && name.as_str() != SIDEBAR_FRAME)
    {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        match rewrite(name, &html, &entries, &lockup, &mark) {
            Ok(drawn) => {
                fs::write(&path, drawn)
                    .map_err(|err| format!("could not write {}: {err}", path.display()))?;
                count += 1;
            }
            Err(why) => problems.extend(why.into_iter().map(|why| format!("{name}: {why}"))),
        }
    }
    if problems.is_empty() {
        Ok(count)
    } else {
        Err(format!(
            "{} part(s) of mdBook's page the frame can't be drawn on (xtask/src/site/frame.rs):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// The fields and entries of the title block the landing page's source writes, from its built
/// HTML: a table of two columns, field then entry, after a head row.
fn source_entries(html: &str) -> Result<Entries, String> {
    let (start, end) =
        block_span(html, SOURCE_BLOCK)?.ok_or("no title block (`<div class=\"title-block\">`)")?;
    let block = &html[start..end];
    let body = block
        .split_once("<tbody>")
        .map(|(_, body)| body)
        .ok_or("the title block has no table body")?;
    let mut entries = Vec::new();
    for row in body.split("<tr>").skip(1) {
        let cells: Vec<&str> = row
            .split("<td")
            .skip(1)
            .filter_map(|cell| {
                let inner = &cell[cell.find('>')? + 1..];
                inner.split_once("</td>").map(|(inner, _)| inner.trim())
            })
            .collect();
        match cells.as_slice() {
            [field, entry] => entries.push((visible(field), (*entry).to_string())),
            _ => return Err(format!("a title block row of {} cells, not 2", cells.len())),
        }
    }
    let fields: Vec<&str> = entries.iter().map(|(field, _)| field.as_str()).collect();
    if fields != FIELDS {
        return Err(format!(
            "the title block has the fields {fields:?}; it needs {FIELDS:?}"
        ));
    }
    Ok(entries)
}

/// Where the element that `open` starts in `html` begins and ends, its closing `</div>` matched
/// by depth; `None` if `html` has no `open`; an error if it is never closed.
fn block_span(html: &str, open: &str) -> Result<Option<(usize, usize)>, String> {
    let Some(start) = html.find(open) else {
        return Ok(None);
    };
    let mut depth = 0usize;
    let mut at = start;
    while let Some(next) = html[at..]
        .find("<div")
        .into_iter()
        .chain(html[at..].find("</div>"))
        .min()
    {
        let here = at + next;
        if html[here..].starts_with("</div>") {
            depth = depth.saturating_sub(1);
            at = here + "</div>".len();
            if depth == 0 {
                return Ok(Some((start, at)));
            }
        } else {
            let after = html.as_bytes().get(here + 4);
            if after.is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>') {
                depth += 1;
            }
            at = here + 4;
        }
    }
    Err(format!("`{open}` is never closed"))
}

/// The text of an HTML fragment: its tags dropped, its escapes kept.
fn visible(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut inside = false;
    for c in html.chars() {
        match c {
            '<' => inside = true,
            '>' if inside => inside = false,
            _ if !inside => text.push(c),
            _ => {}
        }
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The way from page `name` back to the site's root: `../` for each folder it sits in.
fn to_root(name: &str) -> String {
    "../".repeat(name.matches('/').count())
}

/// An entry's HTML as it reads on page `name`: each link that pointed into the landing page, or
/// to another page from the site's root, pointed there from `name`.
fn rebased(entry: &str, name: &str) -> String {
    let root = to_root(name);
    let mut out = String::with_capacity(entry.len());
    let mut rest = entry;
    while let Some(at) = rest.find(" href=\"") {
        let start = at + " href=\"".len();
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let end = rest.find('"').unwrap_or(rest.len());
        let href = &rest[..end];
        if href.starts_with('#') {
            if !LANDINGS.contains(&name) {
                out.push_str(&root);
                out.push_str(LANDINGS[1]);
            }
        } else if !(href.contains(':') || href.starts_with('/')) {
            out.push_str(&root);
        }
        out.push_str(href);
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// The title of page `html`: its one `h1`'s text, or on a page with none or several (the print
/// page holds every chapter) the document's title.
fn page_title(html: &str) -> Result<String, String> {
    let headings: Vec<&str> = html
        .match_indices("<h1")
        .map(|(at, _)| &html[at..])
        .filter(|tag| tag[3..].starts_with([' ', '>']))
        .filter_map(|tag| tag.split_once("</h1>").map(|(heading, _)| heading))
        .collect();
    if let [heading] = headings.as_slice() {
        return Ok(visible(heading));
    }
    html.split_once("<title>")
        .and_then(|(_, rest)| rest.split_once("</title>"))
        .map(|(title, _)| visible(title))
        .filter(|title| !title.is_empty())
        .ok_or_else(|| format!("{} `h1` and no `<title>`", headings.len()))
}

/// The title block for page `name` titled `title`, from the landing page's `entries`, the page's
/// title after the site's.
fn title_block(name: &str, title: &str, entries: &Entries, mark: &str) -> String {
    let mut block = String::from(FOOTER);
    let mut cell = |field: &str, entry: &str, class: &str| {
        let class = if class.is_empty() && visible(entry).chars().count() > LONG_ENTRY_CHARS {
            " class=\"full\""
        } else {
            class
        };
        block.push_str(&format!(
            "<div{class}><span class=\"k\">{field}</span><span class=\"v\">{entry}</span></div>"
        ));
    };
    for (field, entry) in entries {
        if field == OWNER {
            cell(
                field,
                &format!("{mark}{}", rebased(entry, name)),
                " class=\"mark wide\"",
            );
        } else {
            cell(field, &rebased(entry, name), "");
        }
        if field == TITLE_FIELD {
            cell(PAGE, title, "");
        }
    }
    block.push_str("</footer>");
    block
}

/// Page `name`'s HTML with its frame drawn, from the landing page's title block `entries` and
/// the drawn `lockup` and `mark`; or what the rewrite expected and didn't find.
fn rewrite(
    name: &str,
    html: &str,
    entries: &Entries,
    lockup: &str,
    mark: &str,
) -> Result<String, Vec<String>> {
    let mut problems = Vec::new();
    let mut out = html.to_string();

    // The title block the landing page writes in its text, which every page now ends with.
    match block_span(&out, SOURCE_BLOCK) {
        Ok(Some((start, end))) => out.replace_range(start..end, ""),
        Ok(None) => {}
        Err(why) => problems.push(why),
    }

    // The menu bar, a `header` holding the lockup.
    match block_span(&out, MENU_BAR) {
        Ok(Some((start, end))) => {
            out.replace_range(end - "</div>".len()..end, "</header>");
            out.replace_range(start..start + MENU_BAR.len(), HEADER);
        }
        Ok(None) => problems.push(format!("no menu bar (`{MENU_BAR}`)")),
        Err(why) => problems.push(why),
    }
    match out.find(TITLE.0) {
        Some(start) => match out[start..].find(TITLE.1) {
            Some(end) => {
                let link = format!(
                    "{LOGO_OPEN}{}{}\" aria-label=\"FusionSpace HPR, start here\">{lockup}{PRODUCT}</a>",
                    to_root(name),
                    LANDINGS[0]
                );
                out.replace_range(start..start + end + TITLE.1.len(), &link);
            }
            None => problems.push("the menu bar's title is never closed".to_string()),
        },
        None => problems.push(format!("no menu bar title (`{}`)", TITLE.0)),
    }

    // The sidebar, shown from 720 px.
    match out.matches(SIDEBAR_SCRIPT).count() {
        1 => out = out.replace(SIDEBAR_SCRIPT, SIDEBAR_DRAWN),
        found => problems.push(format!(
            "mdBook's script that shows the sidebar from 1,080 px is there {found} times, not once"
        )),
    }

    // The title block, after the page's arrows, or after its text where it has none.
    let title = page_title(&out).map_err(|why| vec![why])?;
    let block = title_block(name, &title, entries, mark);
    let after = match out.find(ARROWS) {
        Some(start) => out[start..]
            .find("</nav>")
            .map(|end| start + end + "</nav>".len()),
        None if name == PRINT_PAGE => out.find("</main>").map(|end| end + "</main>".len()),
        None => None,
    };
    match after {
        Some(at) => out.insert_str(at, &format!("\n{block}")),
        None => problems.push(format!(
            "no end to the page's arrows (`{ARROWS}`) to follow"
        )),
    }

    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// Checks the guide's built pages under `output`: each has the frame [`draw`] gives it.
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let (lockup, mark) = drawn_logos()?;
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut problems = Vec::new();
    for name in names
        .iter()
        .filter(|name| !name.starts_with("api/") && name.as_str() != SIDEBAR_FRAME)
    {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        problems.extend(
            page_problems(&html, &lockup, &mark)
                .into_iter()
                .map(|why| format!("{name}: {why}")),
        );
    }
    Ok(problems)
}

/// The elements named `tag` in `html`, each from its `<` on.
fn elements<'a>(html: &'a str, tag: &'a str) -> impl Iterator<Item = (usize, &'a str)> + 'a {
    let open = format!("<{tag}");
    let found: Vec<(usize, &str)> = html
        .match_indices(&open)
        .map(|(at, _)| (at, &html[at..]))
        .filter(|(_, rest)| rest[open.len()..].starts_with([' ', '>']))
        .collect();
    found.into_iter()
}

/// What one built page lacks of its frame: one `header`, the menu bar, holding the lockup as
/// drawn; the sidebar, its box and its menu button, once each; mdBook's sidebar script shown
/// from 720 px, not 1,080; one `footer`, the title block,
/// after the page's `main`, with ISO 7200's fields in order, the page's own title, the mark and a
/// date of issue; and no title block left in the page's text.
fn page_problems(html: &str, lockup: &str, mark: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let headers: Vec<(usize, &str)> = elements(html, "header").collect();
    match headers.as_slice() {
        [(_, header)] if header.starts_with(HEADER) => {
            let inner = header
                .split_once("</header>")
                .map_or("", |(inner, _)| inner);
            let logo = inner
                .split_once(LOGO_OPEN)
                .and_then(|(_, rest)| rest.split_once("</a>"))
                .map_or("", |(logo, _)| logo);
            if !logo.contains(&format!(">{lockup}{PRODUCT}")) {
                problems.push(
                    "the header doesn't draw the lockup, then HPR, in one link to the landing \
                     page"
                        .to_string(),
                );
            }
        }
        [_] => problems.push(format!("the one `header` isn't the menu bar (`{HEADER}`)")),
        _ => problems.push(format!(
            "{} `header` (a page has one, the menu bar)",
            headers.len()
        )),
    }
    for (mark, what) in SIDEBAR_PARTS {
        let found = html.matches(mark).count();
        if found != 1 {
            problems.push(format!(
                "{what} (`{mark}`) is there {found} times, not once"
            ));
        }
    }
    if html.contains(SIDEBAR_SCRIPT) || !html.contains(SIDEBAR_DRAWN) {
        problems.push("the sidebar isn't shown from 720 px (mdBook's 1,080 px script)".to_string());
    }
    if html.contains(SOURCE_BLOCK) {
        problems.push("the landing page's title block is left in the page's text".to_string());
    }
    let footers: Vec<(usize, &str)> = elements(html, "footer").collect();
    let main_end = html.rfind("</main>");
    match footers.as_slice() {
        [(at, footer)] if footer.starts_with(FOOTER) => {
            if main_end.is_none_or(|end| end > *at) {
                problems.push("the title block isn't after the page's `main`".to_string());
            }
            let block = footer
                .split_once("</footer>")
                .map_or("", |(block, _)| block);
            problems.extend(block_problems(html, block, mark));
        }
        [_] => problems.push(format!(
            "the one `footer` isn't the title block (`{FOOTER}`)"
        )),
        _ => problems.push(format!(
            "{} `footer` (a page has one, the title block)",
            footers.len()
        )),
    }
    problems
}

/// The fields every page's title block draws, in order: the landing page's, with the page's own
/// title after the site's.
fn built_fields() -> Vec<&'static str> {
    let mut fields = Vec::with_capacity(FIELDS.len() + 1);
    for field in FIELDS {
        fields.push(field);
        if field == TITLE_FIELD {
            fields.push(PAGE);
        }
    }
    fields
}

/// What is wrong with title block `block` on page `html`: fields not ISO 7200's in order, a
/// page entry not the page's own title, an owner without the mark, a date of issue not
/// `YYYY-MM-DD`.
fn block_problems(html: &str, block: &str, mark: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let cells: Vec<(&str, &str)> = block
        .split("<span class=\"k\">")
        .skip(1)
        .filter_map(|cell| {
            let (field, rest) = cell.split_once("</span>")?;
            let entry = rest.strip_prefix("<span class=\"v\">")?;
            let entry = entry.find("</span></div>").map(|end| &entry[..end])?;
            Some((field, entry))
        })
        .collect();
    let fields: Vec<&str> = cells.iter().map(|(field, _)| *field).collect();
    let wanted = built_fields();
    if fields != wanted {
        problems.push(format!(
            "the title block has the fields {fields:?}; it needs {wanted:?}, in that order"
        ));
    }
    let entry = |name: &str| {
        cells
            .iter()
            .find(|(field, _)| *field == name)
            .map_or("", |(_, entry)| *entry)
    };
    match page_title(html) {
        Ok(title) if visible(entry(PAGE)) == title => {}
        Ok(title) => problems.push(format!(
            "the title block's page is `{}`, not the page's title, `{title}`",
            visible(entry(PAGE))
        )),
        Err(why) => problems.push(why),
    }
    if !entry(OWNER).starts_with(mark) {
        problems.push("the title block's owner doesn't carry the mark".to_string());
    }
    let issued = visible(entry(ISSUED));
    if !super::theme::is_date(&issued) {
        problems.push(format!(
            "the title block's date of issue isn't YYYY-MM-DD (`{issued}`)"
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The landing page's title block as mdBook builds its source, dated `date`.
    fn source(date: &str) -> String {
        let rows: String = FIELDS
            .iter()
            .map(|field| {
                let entry = match *field {
                    ISSUED => format!("<span id=\"issue-date\">{date}</span>"),
                    "Status" => "IN PREPARATION: how far to trust it is \
                                 <a href=\"#accuracy-so-far\">above</a>"
                        .to_string(),
                    "Data" => "The <a href=\"weather.html\">weather</a> you fetch".to_string(),
                    _ => format!("A {field}"),
                };
                format!("<tr><td>{field}</td><td>{entry}</td></tr>\n")
            })
            .collect();
        format!(
            "{SOURCE_BLOCK}\n<div class=\"table-wrapper\">\n<table>\n<thead>\n<tr><th>Field</th>\
             <th>Entry</th></tr>\n</thead>\n<tbody>\n{rows}</tbody>\n</table>\n</div>\n</div>\n"
        )
    }

    /// A page as mdBook 0.5 builds it, after [`super::super::chrome`]: the sidebar's script, the
    /// menu bar, `inside` the page's `main` after its title `h1`, and the arrows.
    fn page(h1: &str, inside: &str) -> String {
        format!(
            "<body>\n<input type=\"checkbox\" id=\"mdbook-sidebar-toggle-anchor\" \
             class=\"hidden\">\n<script>\n            let sidebar = null;\n{SIDEBAR_SCRIPT}\n                \
             sidebar = 'hidden';\n            }}\n</script>\n<nav id=\"mdbook-sidebar\" \
             class=\"sidebar\"></nav>\n<div class=\"page\">\n{MENU_BAR}\n\
             <div class=\"left-buttons\">\n<label id=\"mdbook-sidebar-toggle\"></label>\n</div>\n\
             <p class=\"menu-title\">FusionSpace HPR</p>\n<div class=\"right-buttons\">\n\
             <a href=\"print.html\"></a>\n</div>\n</div>\n<div id=\"mdbook-content\" \
             class=\"content\">\n<main>\n<h1 id=\"t\"><a class=\"header\" href=\"#t\">{h1}</a>\
             </h1>\n{inside}</main>\n{ARROWS}\n<a rel=\"next\" href=\"b.html\"></a>\n</nav>\n\
             </div>\n</div>\n</body>"
        )
    }

    fn drawn(name: &str, html: &str, date: &str) -> String {
        let (lockup, mark) = drawn_logos().unwrap();
        let landing = page("Start here", &source(date));
        let entries = source_entries(&landing).unwrap();
        rewrite(name, html, &entries, &lockup, &mark).unwrap()
    }

    fn problems(html: &str) -> Vec<String> {
        let (lockup, mark) = drawn_logos().unwrap();
        page_problems(html, &lockup, &mark)
    }

    #[test]
    fn the_brand_files_draw_in_the_texts_color() {
        let (lockup, mark) = drawn_logos().unwrap();
        for svg in [&lockup, &mark] {
            assert!(svg.contains("aria-hidden=\"true\""), "{svg}");
            assert!(!svg.contains(VOID), "{svg}");
            assert!(
                svg.matches("<path fill=\"currentColor\" d=\"").count() >= 4,
                "{svg}"
            );
        }
        assert!(lockup.contains("viewBox=\"0 0 680.485 115\""), "{lockup}");
        // A file in another color, one moved by a transform, or one with a shape that isn't a
        // path can't be inlined as one color.
        let two = LOCKUP.replacen(&format!("fill=\"{VOID}\""), "fill=\"#DA7C30\"", 1);
        assert!(one_color(&two, "x").unwrap_err().contains("not Void"));
        let moved = LOCKUP.replacen("<path ", "<path transform=\"scale(2)\" ", 1);
        assert!(one_color(&moved, "x").unwrap_err().contains("transform"));
        let shape = LOCKUP.replacen("<path ", "<rect width=\"1\"/><path ", 1);
        assert!(one_color(&shape, "x").unwrap_err().contains("<rect"));
    }

    #[test]
    fn a_page_gets_the_header_the_sidebar_and_the_title_block() {
        let html = drawn(
            "guide/a.html",
            &page("A <code>page</code>", "<p>Text.</p>\n"),
            "2026-10-10",
        );
        assert!(problems(&html).is_empty(), "{:?}", problems(&html));
        assert!(html.contains(&format!("{HEADER}\n")), "{html}");
        assert!(html.contains("</div>\n</header>\n"), "{html}");
        assert!(
            html.contains("<a class=\"menu-title fs-logo\" href=\"../index.html\""),
            "{html}"
        );
        assert!(
            html.contains(SIDEBAR_DRAWN) && !html.contains("1080"),
            "{html}"
        );
        // The title block follows the arrows, its title the page's, its links pointed from the
        // page's folder.
        let footer = html.split_once(FOOTER).unwrap().1;
        assert!(html.find("</nav>").unwrap() < html.find(FOOTER).unwrap());
        assert!(footer.contains("<span class=\"k\">Page</span><span class=\"v\">A page</span>"));
        assert!(
            footer.contains("href=\"../start-here.html#accuracy-so-far\""),
            "{footer}"
        );
        assert!(footer.contains("href=\"../weather.html\""), "{footer}");
        assert!(footer.contains(">2026-10-10</span>"), "{footer}");
    }

    #[test]
    fn the_landing_page_moves_its_title_block_to_its_end() {
        let html = drawn(
            "index.html",
            &page("Start here", &source("2026-10-10")),
            "2026-10-10",
        );
        assert!(problems(&html).is_empty(), "{:?}", problems(&html));
        assert!(!html.contains(SOURCE_BLOCK), "{html}");
        let footer = html.split_once(FOOTER).unwrap().1;
        assert!(footer.contains("href=\"#accuracy-so-far\""), "{footer}");
        assert!(footer.contains("href=\"weather.html\""), "{footer}");
        assert!(
            footer.contains("<span class=\"v\">Start here</span>"),
            "{footer}"
        );
    }

    #[test]
    fn the_print_page_ends_with_the_books_title_block() {
        let print = page("One", "<h1 id=\"two\">Two</h1>\n")
            .replace("<body>", "<title>The book</title>\n<body>")
            .replace(
                &format!("{ARROWS}\n<a rel=\"next\" href=\"b.html\"></a>\n</nav>\n"),
                "",
            );
        let html = drawn(PRINT_PAGE, &print, "2026-10-10");
        assert!(problems(&html).is_empty(), "{:?}", problems(&html));
        assert!(html.contains("<span class=\"v\">The book</span>"), "{html}");
        assert!(html.find("</main>").unwrap() < html.find(FOOTER).unwrap());
    }

    #[test]
    fn a_page_mdbook_draws_otherwise_fails_the_build() {
        let (lockup, mark) = drawn_logos().unwrap();
        let landing = page("Start here", &source("2026-10-10"));
        let entries = source_entries(&landing).unwrap();
        let ordinary = page("A", "");
        for (what, html, why) in [
            (
                "no menu bar",
                ordinary.replace(MENU_BAR, "<div class=\"bar\">"),
                "no menu bar",
            ),
            (
                "no title",
                ordinary.replace("menu-title", "title"),
                "no menu bar title",
            ),
            (
                "a new sidebar script",
                ordinary.replace("1080", "1200"),
                "1,080 px",
            ),
            ("no arrows", ordinary.replace(ARROWS, "<nav>"), "arrows"),
        ] {
            let found = rewrite("a.html", &html, &entries, &lockup, &mark).unwrap_err();
            assert!(found.iter().any(|f| f.contains(why)), "{what}: {found:?}");
        }
        let unclosed = format!("{SOURCE_BLOCK}<table><tbody><tr><td>Owner</td><td>x</td></tr>");
        assert!(
            source_entries(&unclosed)
                .unwrap_err()
                .contains("never closed")
        );
        let short = source("d").replace("<tr><td>Units</td><td>A Units</td></tr>\n", "");
        assert!(source_entries(&short).unwrap_err().contains("fields"));
    }

    #[test]
    fn a_page_without_its_frame_fails() {
        let good = drawn("a.html", &page("A", ""), "2026-10-10");
        assert!(problems(&good).is_empty());
        let footer_at = good.find(FOOTER).unwrap();
        let footer_end = good[footer_at..].find("</footer>").unwrap() + footer_at + 9;
        let cases: [(&str, String, &str); 10] = [
            (
                "mdBook's bar",
                good.replace(HEADER, MENU_BAR)
                    .replace("</header>", "</div>"),
                "0 `header`",
            ),
            (
                "a second header",
                good.replace("<main>", "<main><header>x</header>"),
                "2 `header`",
            ),
            (
                "no lockup",
                good.replace("class=\"fs-lockup\"", "class=\"x\""),
                "lockup",
            ),
            (
                "the old script",
                good.replace(SIDEBAR_DRAWN, SIDEBAR_SCRIPT),
                "720 px",
            ),
            (
                "no title block",
                format!("{}{}", &good[..footer_at], &good[footer_end..]),
                "0 `footer`",
            ),
            (
                "a block in the text",
                good.replace("<main>", &format!("<main>{SOURCE_BLOCK}</div>")),
                "left in",
            ),
            (
                "a block inside main",
                good.replace("</main>", "")
                    .replace("</footer>", "</footer></main>"),
                "after the page's `main`",
            ),
            (
                "another title",
                good.replace("<span class=\"v\">A</span>", "<span class=\"v\">B</span>"),
                "not the page's",
            ),
            (
                "no date",
                good.replace("2026-10-10", "the date"),
                "YYYY-MM-DD",
            ),
            (
                "fields swapped",
                good.replacen(
                    "<span class=\"k\">Units</span>",
                    "<span class=\"k\">Data</span>",
                    1,
                ),
                "fields",
            ),
        ];
        for (what, html, why) in cases {
            let found = problems(&html);
            assert!(found.iter().any(|f| f.contains(why)), "{what}: {found:?}");
        }
        for mark in [
            "<nav id=\"mdbook-sidebar\"",
            "id=\"mdbook-sidebar-toggle-anchor\"",
            "id=\"mdbook-sidebar-toggle\"",
        ] {
            let without = good.replacen(mark, "<nav", 1);
            assert!(
                problems(&without)
                    .iter()
                    .any(|f| f.contains("0 times, not once")),
                "{mark}: {:?}",
                problems(&without)
            );
        }
        let unmarked = good.replacen("class=\"fs-mark\"", "class=\"x\"", 1);
        assert!(
            problems(&unmarked).iter().any(|f| f.contains("mark")),
            "{:?}",
            problems(&unmarked)
        );
    }

    #[test]
    fn a_long_entry_takes_a_row_of_its_own() {
        let entries = vec![
            (OWNER.to_string(), "FusionSpace".to_string()),
            (TITLE_FIELD.to_string(), "The site".to_string()),
            (
                "Units".to_string(),
                "SI first, with US units in brackets after each height".repeat(2),
            ),
        ];
        let block = title_block("a.html", "A", &entries, "<svg class=\"fs-mark\"></svg>");
        assert!(
            block.contains("<div class=\"mark wide\"><span class=\"k\">Owner</span>"),
            "{block}"
        );
        assert!(
            block.contains(
                "<div><span class=\"k\">Title</span><span class=\"v\">The site</span></div>\
                 <div><span class=\"k\">Page</span><span class=\"v\">A</span></div>"
            ),
            "{block}"
        );
        assert!(
            block.contains("<div class=\"full\"><span class=\"k\">Units</span>"),
            "{block}"
        );
    }
}
