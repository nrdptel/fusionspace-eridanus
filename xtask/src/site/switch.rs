//! The theme switch (#384, #429; the product system's `web.md`, *Page anatomy*, *Components* and
//! *Theme*): the header ends with a segmented control, Auto, Light and Dark (Field joins with
//! #452), not mdBook's menu of six themes for two looks (Rust drew as Light, Coal and Ayu as
//! Navy).
//!
//! After mdBook builds the site, [`draw`] rewrites every page's theme menu as the system's
//! segmented control, `role="group"`, its buttons pressed or not (`aria-pressed`), and moves it
//! from beside the menu button to the start of the header's right-hand buttons. The buttons keep
//! mdBook's ids and class, so mdBook's script still sets the theme through them and marks the
//! chosen one; `theme/hpr.js` mirrors that mark as `aria-pressed`, and moves the switch to the
//! top of the sidebar on a window too narrow to hold it in the header ([`HEADER_FROM_PX`]).
//! mdBook's menu button stays in the page, `hidden`, since its script looks it up. [`check`]
//! fails a page whose switch isn't drawn this way, and the page check fails one drawn off the
//! system's or that doesn't set the theme (`pages.rs`, `Kind::Switch`).

use std::fs;
use std::path::Path;

use super::html_files;

/// The narrowest window whose header holds the switch on its row, beside the sidebar from
/// 720 px, the lockup, the search button and the right-hand buttons (ADR-231; `theme/hpr.js`
/// and `theme/hpr.css` use the same width). Narrower, the switch stands at the top of the
/// sidebar.
pub(super) const HEADER_FROM_PX: u32 = 960;

/// mdBook 0.5's theme menu button, up to its attributes' end, and as [`draw`] leaves it.
const TOGGLE: &str = "<button id=\"mdbook-theme-toggle\" class=\"icon-button\" type=\"button\"";
const TOGGLE_HIDDEN: &str =
    "<button id=\"mdbook-theme-toggle\" class=\"icon-button\" type=\"button\" hidden";
/// How mdBook 0.5 opens its theme menu, and how [`draw`] opens the switch.
const MENU: &str =
    "<ul id=\"mdbook-theme-list\" class=\"theme-popup\" aria-label=\"Themes\" role=\"menu\">";
const SWITCH: &str = "<ul id=\"mdbook-theme-list\" class=\"theme-popup fs-seg\" role=\"group\" aria-label=\"Theme\">";
/// mdBook 0.5's themes, by the id it gives each button, in its order.
const MDBOOK_THEMES: [&str; 6] = ["default_theme", "light", "rust", "coal", "navy", "ayu"];
/// The switch's choices: mdBook's theme each sets, and its name. `book.toml` makes Light the
/// default and Navy the theme a dark system asks for, so Auto follows the system.
pub(super) const CHOICES: [(&str, &str); 3] = [
    ("default_theme", "Auto"),
    ("light", "Light"),
    ("navy", "Dark"),
];
/// The site's script, which moves the switch, as the built site holds it.
const SCRIPT: &str = "theme/hpr.js";
/// Where the switch goes: the header's right-hand buttons, at their start.
const RIGHT_BUTTONS: &str = "<div class=\"right-buttons\">";

/// The switch as [`draw`] writes it.
fn switch() -> String {
    let buttons: String = CHOICES
        .iter()
        .map(|(id, name)| {
            format!(
                "<li role=\"none\"><button type=\"button\" class=\"theme\" \
                 id=\"mdbook-theme-{id}\" aria-pressed=\"false\">{name}</button></li>"
            )
        })
        .collect();
    format!("{SWITCH}{buttons}</ul>")
}

/// Draws the theme switch on every page of the guide built under `output`. Returns how many
/// pages it drew it on, or every page's problems.
pub(super) fn draw(output: &Path) -> Result<usize, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut count = 0;
    let mut problems = Vec::new();
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = fs::read_to_string(&path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        if !html.contains("id=\"mdbook-theme-list\"") {
            continue;
        }
        match rewrite(&html) {
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
            "{} part(s) of mdBook's theme menu the switch can't be drawn from \
             (xtask/src/site/switch.rs):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

/// One page's HTML with its theme menu drawn as the switch; or what of mdBook's menu it lacks.
fn rewrite(html: &str) -> Result<String, Vec<String>> {
    let mut problems = Vec::new();
    let mut out = html.to_string();
    match out.matches(TOGGLE).count() {
        1 => out = out.replacen(TOGGLE, TOGGLE_HIDDEN, 1),
        found => problems.push(format!(
            "mdBook's theme menu button (`{TOGGLE}`) is there {found} times, not once"
        )),
    }
    match out.find(MENU) {
        Some(start) => match out[start..].find("</ul>") {
            Some(end) => {
                let menu = &out[start..start + end];
                let ids: Vec<&str> = super::attributes(menu, "id")
                    .skip(1)
                    .map(|id| id.trim_start_matches("mdbook-theme-"))
                    .collect();
                if ids == MDBOOK_THEMES {
                    out.replace_range(start..start + end + "</ul>".len(), "");
                } else {
                    problems.push(format!(
                        "mdBook's theme menu offers {ids:?}, not {MDBOOK_THEMES:?}"
                    ));
                }
            }
            None => problems.push("mdBook's theme menu is never closed".to_string()),
        },
        None => problems.push(format!("no theme menu (`{MENU}`)")),
    }
    match out.matches(RIGHT_BUTTONS).count() {
        1 => out = out.replacen(RIGHT_BUTTONS, &format!("{RIGHT_BUTTONS}{}", switch()), 1),
        found => problems.push(format!(
            "the header's right-hand buttons (`{RIGHT_BUTTONS}`) are there {found} times, not \
             once"
        )),
    }
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// Checks the guide's built pages under `output`: each has the switch [`draw`] gives it, once,
/// and no menu of mdBook's themes; and the site's script moves it at [`HEADER_FROM_PX`].
pub(super) fn check(output: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let mut problems = Vec::new();
    let script = output.join(SCRIPT);
    let moves = format!("matchMedia('(min-width: {HEADER_FROM_PX}px)')");
    match fs::read_to_string(&script) {
        Ok(text) if text.contains(&moves) => {}
        Ok(_) => problems.push(format!(
            "{SCRIPT}: doesn't move the theme switch at {HEADER_FROM_PX} px (`{moves}`)"
        )),
        Err(err) => return Err(format!("could not read {}: {err}", script.display())),
    }
    for name in names
        .iter()
        .filter(|name| !name.starts_with("api/") && name.as_str() != "toc.html")
    {
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

/// What one built page lacks of its theme switch: the switch once, at the start of the header's
/// right-hand buttons; mdBook's menu button once, hidden; no theme menu; no other theme button.
fn page_problems(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let drawn = format!("{RIGHT_BUTTONS}{}", switch());
    match html.matches(&drawn).count() {
        1 => {}
        found => problems.push(format!(
            "the theme switch (Auto, Light, Dark) opens the header's right-hand buttons {found} \
             times, not once"
        )),
    }
    if html.matches(TOGGLE_HIDDEN).count() != 1 || html.matches(TOGGLE).count() != 1 {
        problems.push("mdBook's theme menu button isn't there once, hidden".to_string());
    }
    if html.contains("role=\"menu\"") || html.contains("role=\"menuitem\"") {
        problems.push("a menu (`role=\"menu\"`) left in the page".to_string());
    }
    let themes = html.matches("class=\"theme\"").count();
    if themes != CHOICES.len() {
        problems.push(format!(
            "{themes} theme buttons, not the switch's {}",
            CHOICES.len()
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A header as mdBook 0.5.4 writes it, with the frame's rewrite.
    fn header() -> String {
        let menu: String = MDBOOK_THEMES
            .iter()
            .map(|id| {
                format!(
                    "\n    <li role=\"none\"><button role=\"menuitem\" class=\"theme\" \
                     id=\"mdbook-theme-{id}\">{id}</button></li>"
                )
            })
            .collect();
        format!(
            "<header id=\"mdbook-menu-bar\" class=\"menu-bar sticky\">\n<div \
             class=\"left-buttons\">\n{TOGGLE} title=\"Change theme\" \
             aria-controls=\"mdbook-theme-list\">\n<span class=fa-svg></span>\n</button>\n\
             {MENU}{menu}\n</ul>\n<button id=\"mdbook-search-toggle\"></button>\n</div>\n<a \
             class=\"menu-title fs-logo\" href=\"index.html\">HPR</a>\n{RIGHT_BUTTONS}\n<a \
             href=\"print.html\"></a>\n</div>\n</header><main><h1>t</h1></main>"
        )
    }

    #[test]
    fn the_menu_becomes_the_switch_at_the_start_of_the_right_hand_buttons() {
        let drawn = rewrite(&header()).unwrap();
        assert!(drawn.contains(&format!(
            "{RIGHT_BUTTONS}{}\n<a href=\"print.html\">",
            switch()
        )));
        assert!(drawn.contains(&format!("{TOGGLE_HIDDEN} title=\"Change theme\"")));
        assert!(!drawn.contains(MENU));
        assert!(!drawn.contains("mdbook-theme-coal"));
        assert!(switch().contains(
            "<button type=\"button\" class=\"theme\" id=\"mdbook-theme-navy\" \
             aria-pressed=\"false\">Dark</button>"
        ));
        assert_eq!(page_problems(&drawn), Vec::<String>::new());
        // A second rewrite finds no menu, and the check fails a page with the menu left.
        assert!(rewrite(&drawn).is_err());
        assert!(!page_problems(&header()).is_empty());
    }

    #[test]
    fn the_width_the_header_holds_the_switch_from_is_one_number() {
        let px = HEADER_FROM_PX;
        let script = include_str!("../../../theme/hpr.js");
        assert!(script.contains(&format!("matchMedia('(min-width: {px}px)')")));
        assert!(include_str!("pages.js").contains(&format!("const SWITCH_HEADER_PX = {px};")));
        let canaries = include_str!("pages.rs");
        assert_eq!(
            canaries
                .matches(&format!("matchMedia('(min-width: {px}px)')"))
                .count(),
            2
        );
        assert!(include_str!("../../../theme/hpr.css").contains(&format!("{px} px")));
    }

    #[test]
    fn a_menu_mdbook_changed_fails_the_build() {
        let more = header().replace(
            "</ul>",
            "<li role=\"none\"><button role=\"menuitem\" class=\"theme\" \
             id=\"mdbook-theme-new\">New</button></li></ul>",
        );
        assert!(rewrite(&more).unwrap_err()[0].contains("offers"));
        let toggled =
            header().replace(TOGGLE, "<button id=\"mdbook-theme-toggle\" type=\"button\"");
        assert!(rewrite(&toggled).unwrap_err()[0].contains("menu button"));
        let unplaced = header().replace(RIGHT_BUTTONS, "<div class=\"right\">");
        assert!(rewrite(&unplaced).unwrap_err()[0].contains("right-hand buttons"));
    }

    #[test]
    fn the_check_fails_each_part_off() {
        let drawn = rewrite(&header()).unwrap();
        for (broken, words) in [
            (drawn.replace(">Dark<", ">Navy<"), "theme switch"),
            (drawn.replace(TOGGLE_HIDDEN, TOGGLE), "hidden"),
            (
                drawn.replace("role=\"group\"", "role=\"menu\""),
                "theme switch",
            ),
            (
                drawn.replace(
                    "</header>",
                    "<button class=\"theme\">Coal</button></header>",
                ),
                "4 theme buttons",
            ),
            (
                drawn.replace("</header>", "<ul role=\"menu\"></ul></header>"),
                "a menu",
            ),
        ] {
            let problems = page_problems(&broken);
            assert!(
                problems.iter().any(|why| why.contains(words)),
                "{words}: {problems:?}"
            );
        }
    }
}
