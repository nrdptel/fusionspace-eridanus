//! The documentation site's look (M0.6a, ADR-164): the FusionSpace product system's mdBook theme
//! and fonts in `theme/`, every font served from the site, and the title block that ends the
//! landing page.
//!
//! - **Copied files stay as the system wrote them.** Each text file copied from the system opens
//!   with its SPDX line, and hpr-sim's own files with theirs. When `refs/fusionspace-design` is
//!   checked out (the local gate), every copied file is compared with the pinned revision byte for
//!   byte; CI has no `refs/`, and says so.
//! - **No request leaves the site.** No built page loads a stylesheet, script, image or frame from
//!   another server, and no built stylesheet names one in `url()` or `@import`; every `url()` the
//!   theme's stylesheets give resolves to a file the site serves.
//! - **The strip and the fonts on every page.** Every page of the guide links the theme, whose
//!   `body::before` draws the gradient strip, and `fonts/fonts.css`.
//! - **A title block ends the landing page**, with its fields from ISO 7200 as the system's
//!   `web.md` lists them. Its designation is hpr-sim's, its version the workspace's, its data row
//!   names the date of the bundled motor catalog, and its date of issue is filled in at build time
//!   with the date of the commit the site is built from.

use std::fs;
use std::path::Path;
use std::process::Command;

use super::{attributes, html_files, is_web, unescape, visible_text};

/// The theme folder, relative to the workspace root: mdBook's `theme/` beside `book.toml`.
const THEME: &str = "theme";
/// The product system's repository, checked out under `refs/` by `cargo xtask refs`.
pub(crate) const DESIGN_REFS: &str = "refs/fusionspace-design";
/// The revision of the product system the copied files come from (Rev A), as ADR-164 pins it
/// (moved by ADR-198).
/// `refs.lock.toml`'s pin must start with it.
pub(crate) const DESIGN_REV: &str = "f45454f";
/// The lock file that pins the `refs/` checkouts, relative to the workspace root.
pub(crate) const REFS_LOCK: &str = "validation/refs.lock.toml";
/// The product system's name in [`REFS_LOCK`].
const DESIGN_NAME: &str = "fusionspace-design";
/// hpr-sim's `@font-face` rules, relative to [`THEME`]. mdBook serves the folder it is in as
/// `fonts/`.
const FONTS_CSS: &str = "fonts/fonts.css";
/// The SPDX line a stylesheet copied from the product system opens with.
const SYSTEM_SPDX: &str = "/* SPDX-License-Identifier: Apache-2.0 · Copyright 2026 Neer Patel */";
/// The SPDX line hpr-sim's own stylesheets open with.
const OWN_SPDX: &str = "/* SPDX-License-Identifier: MIT OR Apache-2.0 */";
/// The files copied unchanged from the product system: where each sits under [`THEME`], and where
/// it sits in the system's repository. The icons are the ones `chrome.rs` draws in each page's
/// chrome; the favicon is the system's web kit's, its PNG the 32 px one.
pub(crate) const COPIED: [(&str, &str); 22] = [
    (
        "fusionspace-mdbook.css",
        "product/web/mdbook/fusionspace-mdbook.css",
    ),
    (
        "fonts/Archivo-Regular.woff2",
        "product/web/fonts/Archivo-Regular.woff2",
    ),
    (
        "fonts/Archivo-Italic.woff2",
        "product/web/fonts/Archivo-Italic.woff2",
    ),
    (
        "fonts/Archivo-SemiBold.woff2",
        "product/web/fonts/Archivo-SemiBold.woff2",
    ),
    (
        "fonts/CascadiaMono-Regular.woff2",
        "product/web/fonts/CascadiaMono-Regular.woff2",
    ),
    (
        "fonts/CascadiaMono-SemiBold.woff2",
        "product/web/fonts/CascadiaMono-SemiBold.woff2",
    ),
    ("fonts/OFL-Archivo.txt", "product/web/fonts/OFL-Archivo.txt"),
    (
        "fonts/OFL-CascadiaMono.txt",
        "product/web/fonts/OFL-CascadiaMono.txt",
    ),
    ("favicon.svg", "kit/web/favicon.svg"),
    ("favicon.png", "kit/web/favicon-32.png"),
    ("icons/chevron.svg", "product/icons/chevron.svg"),
    ("icons/copy.svg", "product/icons/copy.svg"),
    ("icons/edit.svg", "product/icons/edit.svg"),
    ("icons/external.svg", "product/icons/external.svg"),
    ("icons/menu.svg", "product/icons/menu.svg"),
    ("icons/minus.svg", "product/icons/minus.svg"),
    ("icons/plus.svg", "product/icons/plus.svg"),
    ("icons/print.svg", "product/icons/print.svg"),
    ("icons/refresh.svg", "product/icons/refresh.svg"),
    ("icons/search.svg", "product/icons/search.svg"),
    ("icons/simulate.svg", "product/icons/simulate.svg"),
    ("icons/sun.svg", "product/icons/sun.svg"),
];
/// hpr-sim's own stylesheets under [`THEME`].
const OWN: [&str; 1] = ["hpr.css"];
/// hpr-sim's own files under [`THEME`] that aren't stylesheets, each with the license line it
/// opens with: the script mdBook adds to every page, and what it adds to every page's head.
const OWN_OTHER: [(&str, &str); 2] = [
    ("hpr.js", "// SPDX-License-Identifier: MIT OR Apache-2.0"),
    (
        "head.hbs",
        "{{!-- SPDX-License-Identifier: MIT OR Apache-2.0 --}}",
    ),
];
/// The product system's `@font-face` rules, which [`FONTS_CSS`] holds with the system's folder
/// ([`FONTS_FOLDER`]) dropped from each address: mdBook serves the file beside the fonts.
const SYSTEM_FONTS_CSS: &str = "product/web/fonts.css";
/// How the system's font addresses start, and how hpr-sim's start.
const FONTS_FOLDER: (&str, &str) = ("url('fonts/", "url('");
/// The colors the browser's own bar takes on a light system and on a dark one, Paper and Void
/// (`web.md`, *Theme*; the system's `kit/web/head.html`).
const THEME_COLORS: [(&str, &str); 2] = [("light", "#F3F4F7"), ("dark", "#0B0F1C")];
/// The theme's stylesheet, which draws the strip, and the fonts' stylesheet, as every built page
/// of the guide links them.
const PAGE_STYLES: [&str; 2] = ["theme/fusionspace-mdbook.css", "fonts/fonts.css"];
/// The landing page's source, relative to the site's source folder; mdBook builds it as both
/// `start-here.html` and `index.html`.
const LANDING: &str = "start-here.md";
/// The built landing page, relative to the site's output folder.
const LANDING_HTML: &str = "index.html";
/// How the landing page's source opens the title block, and how its HTML does.
const BLOCK_OPEN: &str = "<div class=\"title-block\">";
/// hpr-sim's designation in the product system's register (ADR-164).
const DESIGNATION: &str = "`FS-ACHERNAR · SW · TOOL 001`";
/// The title block's fields, in order: ISO 7200's, as the product system's `web.md` names them.
const FIELDS: [&str; 9] = [
    "Owner",
    "Title",
    "Designation",
    "Version",
    "Date of issue",
    "Status",
    "Units",
    "Data",
    "Fonts",
];
/// The statuses the product system gives a tool.
const STATUSES: [&str; 2] = ["RELEASED", "IN PREPARATION"];
/// The date of issue as the source writes it, which the build replaces with the commit's date.
const ISSUE_DATE: &str =
    "<span id=\"issue-date\">the date of the commit the site is built from</span>";
/// The bundled motor catalog, relative to the workspace root, whose capture date the data row
/// names.
const CATALOG: &str = "crates/hpr-motor/data/thrustcurve/catalog.json";

/// What [`check_sources`] found: the problems, and whether the copies were compared with the
/// product system's checkout.
pub(super) struct Sources {
    pub problems: Vec<String>,
    pub compared: bool,
}

/// Checks `theme/` and the landing page's title block, before anything is built.
pub(super) fn check_sources(root: &Path, source: &Path) -> Result<Sources, String> {
    let theme = root.join(THEME);
    let mut problems = Vec::new();
    let copied_css = COPIED
        .iter()
        .map(|(file, _)| *file)
        .filter(|file| file.ends_with(".css"));
    for file in copied_css.chain([FONTS_CSS]) {
        let text = read(&theme.join(file))?;
        if text.lines().next() != Some(SYSTEM_SPDX) {
            problems.push(format!(
                "{THEME}/{file}: a file copied from the product system keeps its first line, \
                 `{SYSTEM_SPDX}`"
            ));
        }
    }
    problems.extend(missing_copies(&theme));
    for (file, spdx) in OWN_OTHER {
        if read(&theme.join(file))?.lines().next() != Some(spdx) {
            problems.push(format!(
                "{THEME}/{file}: hpr-sim's own file opens with `{spdx}`"
            ));
        }
    }
    for file in OWN.into_iter().chain([FONTS_CSS]) {
        let text = read(&theme.join(file))?;
        if OWN.contains(&file) && text.lines().next() != Some(OWN_SPDX) {
            problems.push(format!(
                "{THEME}/{file}: hpr-sim's own stylesheet opens with `{OWN_SPDX}`"
            ));
        }
        let dir = Path::new(file).parent().unwrap_or(Path::new(""));
        for url in css_urls(&text) {
            let target = theme.join(dir).join(&url);
            // mdBook serves a stylesheet at the theme's root from `theme/` in the site, so `../`
            // from it reaches the guide's own files, under `docs/`.
            let in_guide = dir.as_os_str().is_empty()
                && url
                    .strip_prefix("../")
                    .is_some_and(|rest| !rest.contains("..") && source.join(rest).is_file());
            if is_web(&url) || !(target.is_file() || in_guide) {
                problems.push(format!(
                    "{THEME}/{file}: `url({url})` is not a file in {THEME}/, so the site would \
                     not serve it"
                ));
            }
        }
    }
    problems.extend(fonts_without_license(&theme)?);
    let commit = locked_commit(&read(&root.join(REFS_LOCK))?)?;
    if !commit.starts_with(DESIGN_REV) {
        problems.push(format!(
            "{REFS_LOCK} pins {DESIGN_NAME} at {commit}, and the theme was copied from \
             {DESIGN_REV}: copy the theme again from the new revision and update {DESIGN_REV} \
             here, in {THEME}/README.md and in THIRD-PARTY-NOTICES.md"
        ));
    }
    let design = root.join(DESIGN_REFS);
    let compared = design.is_dir();
    if compared {
        for (file, original) in COPIED {
            let ours = fs::read(theme.join(file))
                .map_err(|err| format!("could not read {THEME}/{file}: {err}"))?;
            match committed_file(&design, &commit, original) {
                Ok(theirs) if theirs == ours => {}
                Ok(_) => problems.push(format!(
                    "{THEME}/{file} differs from {original} at {commit}: copy it unchanged from \
                     the pinned revision"
                )),
                Err(why) => problems.push(format!(
                    "could not read {original} at {commit} to compare with {THEME}/{file}: {why}"
                )),
            }
        }
        match committed_file(&design, &commit, SYSTEM_FONTS_CSS) {
            Ok(theirs) => problems.extend(
                fonts_problems(
                    &read(&theme.join(FONTS_CSS))?,
                    &String::from_utf8_lossy(&theirs),
                )
                .into_iter()
                .map(|why| format!("{THEME}/{FONTS_CSS}: {why} ({SYSTEM_FONTS_CSS} at {commit})")),
            ),
            Err(why) => problems.push(format!(
                "could not read {SYSTEM_FONTS_CSS} at {commit} to compare with \
                 {THEME}/{FONTS_CSS}: {why}"
            )),
        }
    }
    let faces = font_faces(&read(&theme.join(FONTS_CSS))?);
    for file in ["fusionspace-mdbook.css", "hpr.css"] {
        for family in first_families(&read(&theme.join(file))?) {
            if !faces.contains(&family) {
                problems.push(format!(
                    "{THEME}/{file} sets the font `{family}`, and {THEME}/{FONTS_CSS} has no \
                     `@font-face` for it, so the site would not serve it"
                ));
            }
        }
    }
    let landing = read(&source.join(LANDING))?;
    let catalog = read(&root.join(CATALOG))?;
    let captured = catalog_date(&catalog)?;
    problems.extend(
        title_block_problems(&landing, env!("CARGO_PKG_VERSION"), &captured)
            .into_iter()
            .map(|why| format!("{LANDING}: the title block {why}")),
    );
    Ok(Sources { problems, compared })
}

/// The commit [`REFS_LOCK`] pins [`DESIGN_NAME`] at.
pub(crate) fn locked_commit(lock: &str) -> Result<String, String> {
    let lock: toml::Value = toml::from_str(lock).map_err(|err| format!("{REFS_LOCK}: {err}"))?;
    lock.get("git")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("name").and_then(toml::Value::as_str) == Some(DESIGN_NAME))
        .and_then(|entry| entry.get("commit"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("{REFS_LOCK} pins no commit for {DESIGN_NAME}"))
}

/// A file's bytes as `commit` holds it in the git checkout at `design`: what was committed, not
/// what the working tree holds, so neither a later checkout nor a line-ending conversion moves
/// the comparison.
pub(crate) fn committed_file(design: &Path, commit: &str, path: &str) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(design)
        .args(["show", &format!("{commit}:{path}")])
        .output()
        .map_err(|err| format!("could not run git: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    Ok(output.stdout)
}

/// The font each `font-family` declaration of a stylesheet asks for first, its others being the
/// fallbacks, outside `@font-face` rules.
fn first_families(css: &str) -> Vec<String> {
    let mut families = Vec::new();
    for rule in css.split('}') {
        if rule.contains("@font-face") {
            continue;
        }
        for (at, _) in rule.match_indices("font-family:") {
            let value = &rule[at + "font-family:".len()..];
            let value = value.split([';', '}']).next().unwrap_or("");
            let first = value.split(',').next().unwrap_or("").trim();
            let family = first.trim_matches(['\'', '"']).to_owned();
            if !family.is_empty() && !families.contains(&family) {
                families.push(family);
            }
        }
    }
    families
}

/// Each `@font-face` rule of a stylesheet, from its at-keyword to its closing brace.
fn face_rules(css: &str) -> Vec<&str> {
    css.split_inclusive('}')
        .filter_map(|chunk| chunk.rfind("@font-face").map(|at| chunk[at..].trim()))
        .collect()
}

/// How hpr-sim's `@font-face` rules (`ours`) differ from the product system's (`theirs`), each
/// of the system's taken with [`FONTS_FOLDER`]'s change to its addresses: the same rules, in the
/// same order, character for character, so the system's `unicode-range` and metric-matched
/// fallback faces come with them (#383).
fn fonts_problems(ours: &str, theirs: &str) -> Vec<String> {
    let (folder, here) = FONTS_FOLDER;
    let wanted: Vec<String> = face_rules(theirs)
        .into_iter()
        .map(|rule| rule.replace(folder, here))
        .collect();
    let got = face_rules(ours);
    let mut problems = Vec::new();
    for (i, rule) in wanted.iter().enumerate() {
        match got.get(i) {
            Some(have) if have == rule => {}
            Some(have) => problems.push(format!(
                "`@font-face` rule {} is `{have}`, and the system's is `{rule}`",
                i + 1
            )),
            None => problems.push(format!("lacks the system's `{rule}`")),
        }
    }
    for extra in got.iter().skip(wanted.len()) {
        problems.push(format!("has `{extra}`, which the system's file lacks"));
    }
    problems
}

/// The families a stylesheet's `@font-face` rules declare.
fn font_faces(css: &str) -> Vec<String> {
    css.split('}')
        .filter(|rule| rule.contains("@font-face"))
        .flat_map(first_families_in_face)
        .collect()
}

/// The `font-family` an `@font-face` rule declares.
fn first_families_in_face(rule: &str) -> Option<String> {
    let (_, value) = rule.split_once("font-family:")?;
    let value = value.split(';').next()?.trim();
    Some(value.trim_matches(['\'', '"']).to_owned())
}

/// Reads a text file, naming it in the error.
fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("could not read {}: {err}", path.display()))
}

/// Every font file in `theme/fonts/` without its license beside it: `Family-Style.woff2` needs
/// `OFL-Family.txt`, as the product system ships them.
/// Each file of [`COPIED`] missing from `theme`. Where the system's checkout is absent, as in CI,
/// nothing else would see one gone: mdBook serves its own book as the favicon when the theme has
/// none, and the served-file check compares only the files the theme holds.
fn missing_copies(theme: &Path) -> Vec<String> {
    COPIED
        .iter()
        .filter(|(file, _)| !theme.join(file).is_file())
        .map(|(file, original)| {
            format!("{THEME}/{file} is missing: copy it unchanged from the system's {original}")
        })
        .collect()
}

fn fonts_without_license(theme: &Path) -> Result<Vec<String>, String> {
    let fonts = theme.join("fonts");
    let entries =
        fs::read_dir(&fonts).map_err(|err| format!("could not list {}: {err}", fonts.display()))?;
    let mut problems = Vec::new();
    for entry in entries {
        let name = entry
            .map_err(|err| format!("could not list {}: {err}", fonts.display()))?
            .file_name()
            .to_string_lossy()
            .into_owned();
        let Some(stem) = name.strip_suffix(".woff2") else {
            continue;
        };
        let family = stem.split_once('-').map_or(stem, |(family, _)| family);
        if !fonts.join(format!("OFL-{family}.txt")).is_file() {
            problems.push(format!(
                "{THEME}/fonts/{name}: no `OFL-{family}.txt` beside it; the SIL Open Font \
                 License travels with the font"
            ));
        }
    }
    Ok(problems)
}

/// The addresses a stylesheet names in `url(...)` and `@import`, unquoted.
fn css_urls(css: &str) -> Vec<String> {
    // CSS keywords ignore case. ASCII lowercasing keeps every byte where it was, so the matches
    // in `lower` index `css` too.
    let lower = css.to_ascii_lowercase();
    let mut urls = Vec::new();
    for (at, _) in lower.match_indices("url(") {
        let rest = &css[at + 4..];
        if let Some((inside, _)) = rest.split_once(')') {
            urls.push(inside.trim().trim_matches(['\'', '"']).to_owned());
        }
    }
    for (at, _) in lower.match_indices("@import") {
        let rest = css[at + 7..].trim_start();
        if rest.to_ascii_lowercase().starts_with("url(") {
            continue;
        }
        let quote = rest.chars().next().filter(|c| *c == '"' || *c == '\'');
        if let Some(quote) = quote
            && let Some((inside, _)) = rest[1..].split_once(quote)
        {
            urls.push(inside.to_owned());
        }
    }
    urls
}

/// The date `catalog.json` records for its snapshot of ThrustCurve.org, `snapshot.captured`.
fn catalog_date(catalog: &str) -> Result<String, String> {
    let json: serde_json::Value =
        serde_json::from_str(catalog).map_err(|err| format!("{CATALOG}: {err}"))?;
    json.pointer("/snapshot/captured")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("{CATALOG} has no `snapshot.captured` date"))
}

/// What is wrong with the landing page's title block, as phrases following "the title block":
/// it is missing or not last, a field is missing, out of order or extra, or an entry disagrees
/// with the workspace (`version`) or the motor catalog (`captured`).
fn title_block_problems(page: &str, version: &str, captured: &str) -> Vec<String> {
    let Some((_, block)) = page.split_once(BLOCK_OPEN) else {
        return vec![format!("is missing: open it with `{BLOCK_OPEN}`")];
    };
    let Some((block, after)) = block.split_once("</div>") else {
        return vec!["has no closing `</div>`".to_owned()];
    };
    let mut problems = Vec::new();
    // Link reference definitions print nothing, so they may follow it.
    let trailing = after
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !(line.starts_with('[') && line.contains("]: ")));
    if let Some(line) = trailing {
        problems.push(format!("must end the page, and `{line}` follows it"));
    }
    let rows: Vec<(String, String)> = block
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .skip(2)
        .filter_map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split(" | ").collect();
            match cells.as_slice() {
                [field, entry] => Some((field.trim().to_owned(), entry.trim().to_owned())),
                _ => None,
            }
        })
        .collect();
    let fields: Vec<&str> = rows.iter().map(|(field, _)| field.as_str()).collect();
    if fields != FIELDS {
        problems.push(format!(
            "has the fields {fields:?}; it needs {FIELDS:?}, in that order, one row each"
        ));
    }
    let entry = |name: &str| {
        rows.iter()
            .find(|(field, _)| field == name)
            .map_or("", |(_, entry)| entry.as_str())
    };
    if entry("Designation") != DESIGNATION {
        problems.push(format!("gives the designation as {DESIGNATION}"));
    }
    let given = entry("Version");
    let number = given.split_once(',').map_or(given, |(number, _)| number);
    if number != version {
        problems.push(format!(
            "gives version `{number}`, and the workspace is at {version}"
        ));
    }
    if entry("Date of issue") != ISSUE_DATE {
        problems.push(format!(
            "gives its date of issue as `{ISSUE_DATE}`, which the build fills in"
        ));
    }
    // The status may be followed by a colon and what it means.
    let status = entry("Status");
    let status = status.split_once(':').map_or(status, |(word, _)| word);
    if !STATUSES.contains(&status) {
        problems.push(format!("gives a status from {STATUSES:?}, first"));
    }
    if !entry("Data").contains(&format!("catalog captured {captured}")) {
        problems.push(format!(
            "names the motor catalog's date, `catalog captured {captured}`, from {CATALOG}"
        ));
    }
    problems
}

/// The date of the commit the site is built from, `YYYY-MM-DD`, as git gives it.
pub(super) fn commit_date(root: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%cs", "HEAD"])
        .current_dir(root)
        .output()
        .map_err(|err| format!("could not run git for the title block's date: {err}"))?;
    let date = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if !output.status.success() || !is_date(&date) {
        return Err(format!(
            "git gave no commit date for the title block (`{date}`): {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(date)
}

/// Whether `text` is a date as `YYYY-MM-DD`.
fn is_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(at, byte)| match at {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

/// Fills the title block's date of issue with `date` on every built page of the guide that
/// carries it (the landing page twice, as `index.html` and `start-here.html`, and the print
/// page). Returns how many pages it filled; the landing page must be one.
pub(super) fn fill_issue_date(output: &Path, date: &str) -> Result<usize, String> {
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    let filled = ISSUE_DATE.replace("the date of the commit the site is built from", date);
    let mut count = 0;
    let mut landing = false;
    for name in names.iter().filter(|name| !name.starts_with("api/")) {
        let path = output.join(name);
        let html = read(&path)?;
        if !html.contains(ISSUE_DATE) {
            continue;
        }
        fs::write(&path, html.replace(ISSUE_DATE, &filled))
            .map_err(|err| format!("could not write {}: {err}", path.display()))?;
        count += 1;
        landing |= name == LANDING_HTML;
    }
    if !landing {
        return Err(format!(
            "{LANDING_HTML} has no `{ISSUE_DATE}` to fill with the date of issue"
        ));
    }
    Ok(count)
}

/// Checks the built site: every page of the guide links the theme and the fonts, no page or
/// stylesheet makes a request that leaves the site, and the title block, its date filled in,
/// ends the landing page.
pub(super) fn check_built(root: &Path, output: &Path) -> Result<Vec<String>, String> {
    let mut problems = served_unchanged(&root.join(THEME), output)?;
    let mut names = Vec::new();
    html_files(output, "", &mut names)?;
    for name in &names {
        let html = read(&output.join(name))?;
        for request in outside_requests(&html) {
            problems.push(format!("{name}: {request} leaves the site"));
        }
        if !name.starts_with("api/") {
            problems.extend(
                missing_page_styles(&html)
                    .into_iter()
                    .map(|wanted| format!("{name}: no stylesheet link to `{wanted}`")),
            );
            problems.extend(
                head_problems(&html)
                    .into_iter()
                    .map(|why| format!("{name}: {why}")),
            );
        }
    }
    let mut sheets = Vec::new();
    css_files(output, "", &mut sheets)?;
    for name in &sheets {
        let css = read(&output.join(name))?;
        let dir = output.join(Path::new(name).parent().unwrap_or(Path::new("")));
        let ours = name.starts_with("theme/") || name.starts_with("fonts/");
        problems.extend(stylesheet_problems(name, &css, ours, |path| {
            dir.join(path).is_file()
        }));
    }
    let landing = read(&output.join(LANDING_HTML))?;
    problems.extend(
        built_title_block_problems(&landing)
            .into_iter()
            .map(|why| format!("{LANDING_HTML}: the title block {why}")),
    );
    Ok(problems)
}

/// Whether the site serves `theme/` as it is committed: each file under `theme/fonts/` as
/// `fonts/` in the site, each stylesheet and script in `theme/` as `theme/`, and the favicons
/// at the site's root. mdBook serves its own fonts at the same address, `fonts/fonts.css`, when
/// it doesn't take the theme's, and its own book as the favicon when the theme has none.
fn served_unchanged(theme: &Path, output: &Path) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    let mut served = Vec::new();
    for (dir, site) in [("fonts", "fonts"), ("", "theme")] {
        let from = theme.join(dir);
        let entries = fs::read_dir(&from)
            .map_err(|err| format!("could not list {}: {err}", from.display()))?;
        for entry in entries {
            let entry = entry.map_err(|err| format!("could not list {}: {err}", from.display()))?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let favicon = dir.is_empty() && name.starts_with("favicon.");
            if path.is_dir()
                || (dir.is_empty() && !(name.ends_with(".css") || name.ends_with(".js") || favicon))
            {
                continue;
            }
            let committed = if dir.is_empty() {
                format!("{THEME}/{name}")
            } else {
                format!("{THEME}/{dir}/{name}")
            };
            let at = if favicon {
                name.clone()
            } else {
                format!("{site}/{name}")
            };
            served.push((path, committed, at));
        }
    }
    for (path, committed, site) in served {
        let ours =
            fs::read(&path).map_err(|err| format!("could not read {}: {err}", path.display()))?;
        if fs::read(output.join(&site)).ok().as_deref() != Some(ours.as_slice()) {
            problems.push(format!(
                "{site} is not {committed} as committed: mdBook didn't serve the theme's file"
            ));
        }
    }
    Ok(problems)
}

/// What a built page's head lacks (#383; `web.md`, *Using it in a project* and *Theme*): a
/// `color-scheme` meta of `light dark`, and the `theme-color` a browser takes on a light system
/// and on a dark one, [`THEME_COLORS`]. A browser takes the first `theme-color` meta whose
/// `media` matches, or that has none, so mdBook's own white one, after hpr-sim's, is never
/// taken. A `media` other than a color scheme is a problem: the check can't say when it holds.
fn head_problems(html: &str) -> Vec<String> {
    let metas: Vec<&str> = tags(html, "meta").collect();
    let named = |name: &str| {
        metas
            .iter()
            .filter(|tag| attributes(tag, "name").any(|value| value == name))
            .copied()
            .collect::<Vec<&str>>()
    };
    let mut problems = Vec::new();
    let schemes = named("color-scheme");
    if !schemes
        .iter()
        .any(|tag| attributes(tag, "content").any(|value| value == "light dark"))
    {
        problems.push("no `<meta name=\"color-scheme\" content=\"light dark\">`".to_owned());
    }
    let colors = named("theme-color");
    for (scheme, wanted) in THEME_COLORS {
        let mut taken = None;
        for tag in &colors {
            let media = attributes(tag, "media").map(unescape).next();
            let matches = match media.as_deref().map(str::trim) {
                None | Some("") => true,
                Some(media) => match media
                    .strip_prefix('(')
                    .and_then(|rest| rest.strip_suffix(')'))
                    .and_then(|rest| rest.split_once(':'))
                    .map(|(feature, value)| (feature.trim(), value.trim()))
                {
                    Some(("prefers-color-scheme", value)) => value == scheme,
                    _ => {
                        problems.push(format!(
                            "a `theme-color` meta for `{media}`, which the check can't read"
                        ));
                        false
                    }
                },
            };
            if matches {
                taken = attributes(tag, "content").map(unescape).next();
                break;
            }
        }
        match taken {
            Some(color) if color.eq_ignore_ascii_case(wanted) => {}
            Some(color) => problems.push(format!(
                "a browser on a {scheme} system takes the `theme-color` `{color}`, not `{wanted}`"
            )),
            None => problems.push(format!(
                "no `theme-color` meta for a {scheme} system (`{wanted}`)"
            )),
        }
    }
    problems.dedup();
    problems
}

/// The stylesheets of [`PAGE_STYLES`] a built page of the guide doesn't link.
fn missing_page_styles(html: &str) -> Vec<&'static str> {
    let styles: Vec<String> = tags(html, "link")
        .filter(|tag| attributes(tag, "rel").any(|rel| rel == "stylesheet"))
        .flat_map(|tag| attributes(tag, "href").map(unescape).collect::<Vec<_>>())
        .collect();
    PAGE_STYLES
        .into_iter()
        .filter(|wanted| {
            !styles
                .iter()
                .any(|href| href == wanted || href.ends_with(&format!("/{wanted}")))
        })
        .collect()
}

/// What is wrong with a built stylesheet, `name`: an address on another server in `url()` or
/// `@import`, and, for the theme's own (`ours`), an address that names no file the site serves
/// (`exists` answers for a path relative to the stylesheet).
fn stylesheet_problems(
    name: &str,
    css: &str,
    ours: bool,
    exists: impl Fn(&str) -> bool,
) -> Vec<String> {
    let mut problems = Vec::new();
    for url in css_urls(css) {
        if url.starts_with("data:") {
            continue;
        }
        if is_web(&url) {
            problems.push(format!("{name}: `{url}` leaves the site"));
        } else if ours && !exists(url.split(['?', '#']).next().unwrap_or("")) {
            problems.push(format!("{name}: `url({url})`: no such file in the site"));
        }
    }
    problems
}

/// What is wrong with the title block on the built landing page: it is missing, text follows it
/// in the page's `<main>`, or its date of issue was not filled in.
fn built_title_block_problems(html: &str) -> Vec<String> {
    let Some((_, block)) = html.split_once(BLOCK_OPEN) else {
        return vec!["is missing".to_owned()];
    };
    let mut problems = Vec::new();
    let Some((block, after)) = block.split_once("</table>") else {
        return vec!["has no table".to_owned()];
    };
    let rest = after.split_once("</main>").map_or(after, |(rest, _)| rest);
    let text = visible_text(rest);
    if !text.trim().is_empty() {
        problems.push(format!(
            "must end the page, and `{}` follows it",
            text.trim()
        ));
    }
    let date = block
        .split_once("<span id=\"issue-date\">")
        .and_then(|(_, rest)| rest.split_once("</span>"))
        .map(|(date, _)| date);
    if !date.is_some_and(is_date) {
        problems.push(format!(
            "has no date of issue as YYYY-MM-DD (`{}`)",
            date.unwrap_or("")
        ));
    }
    problems
}

/// The elements named `name` in `html`, each from its `<` to its `>`.
fn tags<'a>(html: &'a str, name: &str) -> impl Iterator<Item = &'a str> + 'a {
    let open = format!("<{name}");
    let starts: Vec<usize> = html
        .match_indices(&open)
        .filter(|(at, _)| {
            html.as_bytes()
                .get(at + open.len())
                .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>')
        })
        .map(|(at, _)| at)
        .collect();
    starts.into_iter().filter_map(move |start| {
        html[start..]
            .find('>')
            .map(|end| &html[start..=start + end])
    })
}

/// The requests a page makes to another server: a `<link>`'s `href` (a stylesheet, an icon, a
/// preloaded font) and any element's `src` or `srcset`, when it names an address on the web. A
/// link a reader follows, `<a href>`, is not a request.
fn outside_requests(html: &str) -> Vec<String> {
    let mut requests = Vec::new();
    for tag in tags(html, "link") {
        for href in attributes(tag, "href").map(unescape) {
            if is_web(&href) {
                requests.push(format!("`<link href=\"{href}\">`"));
            }
        }
    }
    for name in ["src", "srcset"] {
        for value in attributes(html, name).map(unescape) {
            if value.split(',').any(|candidate| is_web(candidate.trim())) {
                requests.push(format!("`{name}=\"{value}\"`"));
            }
        }
    }
    requests
}

/// Lists the stylesheets under `dir`, relative to it, into `out`.
fn css_files(dir: &Path, prefix: &str, out: &mut Vec<String>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("could not list {}: {err}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("could not list {}: {err}", dir.display()))?;
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        let path = entry.path();
        if path.is_dir() {
            css_files(&path, &format!("{name}/"), out)?;
        } else if name.ends_with(".css") {
            out.push(name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A title block as the landing page writes it, for version 1.2.3 and a catalog captured on
    /// 2026-01-02, followed by a link definition.
    fn block() -> String {
        format!(
            "Prose.\n\n{BLOCK_OPEN}\n\n| Field | Entry |\n|---|---|\n| Owner | FusionSpace |\n\
             | Title | t |\n| Designation | {DESIGNATION} |\n| Version | 1.2.3, not yet released |\n\
             | Date of issue | {ISSUE_DATE} |\n| Status | IN PREPARATION |\n| Units | SI |\n\
             | Data | Thrust curves: catalog captured 2026-01-02. |\n| Fonts | f |\n\n</div>\n\n\
             [x]: https://example.com\n"
        )
    }

    #[test]
    fn a_title_block_as_the_page_writes_it_passes() {
        assert_eq!(
            title_block_problems(&block(), "1.2.3", "2026-01-02"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_title_block_must_be_there_and_last() {
        let missing = title_block_problems("Prose.\n", "1.2.3", "2026-01-02");
        assert!(missing[0].starts_with("is missing"), "{missing:?}");
        let followed = format!("{}\nMore prose.\n", block());
        let problems = title_block_problems(&followed, "1.2.3", "2026-01-02");
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("`More prose.` follows it"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_title_block_names_every_field_in_order() {
        let dropped = block().replace("| Units | SI |\n", "");
        let problems = title_block_problems(&dropped, "1.2.3", "2026-01-02");
        assert!(problems[0].starts_with("has the fields"), "{problems:?}");
        let swapped = block().replace(
            "| Owner | FusionSpace |\n| Title | t |",
            "| Title | t |\n| Owner | FusionSpace |",
        );
        let problems = title_block_problems(&swapped, "1.2.3", "2026-01-02");
        assert!(problems[0].starts_with("has the fields"), "{problems:?}");
    }

    #[test]
    fn a_title_block_agrees_with_the_workspace_and_the_catalog() {
        let problems = title_block_problems(&block(), "1.2.4", "2026-01-02");
        assert_eq!(
            problems,
            ["gives version `1.2.3`, and the workspace is at 1.2.4"]
        );
        let problems = title_block_problems(&block(), "1.2.3", "2026-01-03");
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("catalog captured 2026-01-03"),
            "{problems:?}"
        );
        let other = block().replace(DESIGNATION, "`FS · SW · TOOL 004`");
        assert_eq!(
            title_block_problems(&other, "1.2.3", "2026-01-02"),
            [format!("gives the designation as {DESIGNATION}")]
        );
        let explained = block().replace("IN PREPARATION", "IN PREPARATION: no release yet");
        assert!(title_block_problems(&explained, "1.2.3", "2026-01-02").is_empty());
        let status = block().replace("IN PREPARATION", "BETA: IN PREPARATION");
        assert_eq!(
            title_block_problems(&status, "1.2.3", "2026-01-02").len(),
            1
        );
        let date = block().replace(ISSUE_DATE, "2026-10-05");
        assert_eq!(title_block_problems(&date, "1.2.3", "2026-01-02").len(), 1);
    }

    #[test]
    fn requests_to_other_servers_are_found_and_links_are_not() {
        let html = "<link rel=\"stylesheet\" href=\"https://fonts.example/a.css\">\
                    <link rel=\"icon\" href=\"favicon.svg\">\
                    <script src=\"//cdn.example/x.js\"></script><img src=\"a.png\">\
                    <img srcset=\"a.png 1x, https://cdn.example/b.png 2x\">\
                    <a href=\"https://github.com/\">a link</a>";
        let requests = outside_requests(html);
        assert_eq!(requests.len(), 3, "{requests:?}");
        assert!(requests[0].contains("fonts.example"));
        assert!(requests[1].contains("//cdn.example/x.js"));
        assert!(requests[2].contains("srcset"));
        assert!(outside_requests("<a href=\"https://github.com/\">x</a>").is_empty());
    }

    #[test]
    fn stylesheet_addresses_are_read_from_url_and_import() {
        let css = "@import \"https://fonts.example/a.css\";\n@import url('b.css');\n\
                   @font-face { src: url('A.woff2') format('woff2'); }\n\
                   body { background: url(\"data:image/png;base64,xx\"); }";
        assert_eq!(
            css_urls(css),
            [
                "b.css",
                "A.woff2",
                "data:image/png;base64,xx",
                "https://fonts.example/a.css"
            ]
        );
    }

    #[test]
    fn the_built_title_block_must_be_last_and_dated() {
        let page = |date: &str, after: &str| {
            format!(
                "<main><p>x</p>{BLOCK_OPEN}<div class=\"table-wrapper\"><table><tr><td>Date of \
                 issue</td><td><span id=\"issue-date\">{date}</span></td></tr></table></div>\
                 </div>{after}</main><nav>next</nav>"
            )
        };
        assert!(built_title_block_problems(&page("2026-10-05", "\n")).is_empty());
        let unfilled = built_title_block_problems(&page("the date", ""));
        assert_eq!(unfilled.len(), 1, "{unfilled:?}");
        assert!(unfilled[0].contains("YYYY-MM-DD"), "{unfilled:?}");
        let followed = built_title_block_problems(&page("2026-10-05", "<p>Later</p>"));
        assert_eq!(followed, ["must end the page, and `Later` follows it"]);
        assert_eq!(built_title_block_problems("<main></main>"), ["is missing"]);
    }

    #[test]
    fn a_page_links_the_theme_and_the_fonts_by_their_own_names() {
        let link = |href: &str| format!("<link rel=\"stylesheet\" href=\"{href}\">");
        let both = format!(
            "{}{}",
            link("../theme/fusionspace-mdbook.css"),
            link("../fonts/fonts.css")
        );
        assert!(missing_page_styles(&both).is_empty());
        // A hashed name is not the theme's, and an icon is not a stylesheet.
        let hashed = format!(
            "{}{}<link rel=\"icon\" href=\"fonts/fonts.css\">",
            link("theme/fusionspace-mdbook-1a2b.css"),
            link("myfonts/fonts.css")
        );
        assert_eq!(missing_page_styles(&hashed), PAGE_STYLES);
    }

    #[test]
    fn a_page_head_names_the_scheme_and_the_bar_colors_first() {
        let scheme = "<meta name=\"color-scheme\" content=\"light dark\">";
        let light = "<meta name=\"theme-color\" content=\"#F3F4F7\" \
                     media=\"(prefers-color-scheme: light)\">";
        let dark = "<meta name=\"theme-color\" content=\"#0b0f1c\" \
                    media=\"(prefers-color-scheme: dark)\">";
        let mdbook = "<meta name=\"theme-color\" content=\"#ffffff\">";
        let ours = format!("{scheme}{light}{dark}{mdbook}");
        assert_eq!(head_problems(&ours), Vec::<String>::new());
        // mdBook's head alone: its white bar on both systems, and no scheme.
        assert_eq!(
            head_problems(mdbook),
            [
                "no `<meta name=\"color-scheme\" content=\"light dark\">`",
                "a browser on a light system takes the `theme-color` `#ffffff`, not `#F3F4F7`",
                "a browser on a dark system takes the `theme-color` `#ffffff`, not `#0B0F1C`",
            ]
        );
        // mdBook's meta first is taken first, on both systems.
        let late = format!("{scheme}{mdbook}{light}{dark}");
        assert_eq!(head_problems(&late).len(), 2, "{:?}", head_problems(&late));
        // Only the light one: a dark system takes mdBook's.
        let half = format!("{scheme}{light}{mdbook}");
        assert_eq!(
            head_problems(&half),
            ["a browser on a dark system takes the `theme-color` `#ffffff`, not `#0B0F1C`"]
        );
        // Without mdBook's, a dark system has none.
        assert_eq!(
            head_problems(&format!("{scheme}{light}")),
            ["no `theme-color` meta for a dark system (`#0B0F1C`)"]
        );
        // A media the check can't read is named, once.
        let wide = "<meta name=\"theme-color\" content=\"#F3F4F7\" media=\"(min-width: 9px)\">";
        let problems = head_problems(&format!("{scheme}{wide}{light}{dark}"));
        assert_eq!(
            problems,
            ["a `theme-color` meta for `(min-width: 9px)`, which the check can't read"]
        );
    }

    #[test]
    fn the_fonts_are_the_systems_with_their_folder_dropped() {
        let theirs = "/* the system's */\n\
            @font-face { font-family: 'Archivo'; src: url('fonts/Archivo-Regular.woff2') \
            format('woff2'); unicode-range: U+0000-00FF; }\n\
            @font-face { font-family: 'Archivo Fallback'; src: local('Arial'); size-adjust: 101%; }\n";
        let ours = "/* hpr-sim's, naming its `@font-face` rules */\n\
            @font-face { font-family: 'Archivo'; src: url('Archivo-Regular.woff2') \
            format('woff2'); unicode-range: U+0000-00FF; }\n\
            @font-face { font-family: 'Archivo Fallback'; src: local('Arial'); size-adjust: 101%; }\n";
        assert_eq!(fonts_problems(ours, theirs), Vec::<String>::new());
        // A hand-written file: no `unicode-range`, no fallback face.
        let own = "@font-face { font-family: 'Archivo'; src: url('Archivo-Regular.woff2') \
            format('woff2'); }\n";
        let problems = fonts_problems(own, theirs);
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(
            problems[0].starts_with("`@font-face` rule 1 is"),
            "{}",
            problems[0]
        );
        assert!(problems[1].contains("Archivo Fallback"), "{}", problems[1]);
        // The system's addresses as they are name a folder mdBook doesn't serve.
        assert_eq!(fonts_problems(theirs, theirs).len(), 1);
        // A face the system lacks.
        let extra = format!("{ours}@font-face {{ font-family: 'Georgia'; }}\n");
        assert_eq!(
            fonts_problems(&extra, theirs),
            ["has `@font-face { font-family: 'Georgia'; }`, which the system's file lacks"]
        );
    }

    #[test]
    fn a_stylesheet_names_only_files_the_site_serves() {
        let css = "@font-face { src: url('A.woff2'); }\n@font-face { src: URL('B.woff2'); }\n\
                   @IMPORT \"https://fonts.example/c.css\";";
        let exists = |path: &str| path == "A.woff2";
        assert_eq!(
            stylesheet_problems("fonts/fonts.css", css, true, exists),
            [
                "fonts/fonts.css: `url(B.woff2)`: no such file in the site",
                "fonts/fonts.css: `https://fonts.example/c.css` leaves the site"
            ]
        );
        // mdBook's own stylesheets name files only it knows about; only their requests count.
        assert_eq!(
            stylesheet_problems("css/general.css", css, false, exists),
            ["css/general.css: `https://fonts.example/c.css` leaves the site"]
        );
    }

    #[test]
    fn every_font_the_theme_sets_has_a_face() {
        let theme = "html { font-family: 'Archivo', system-ui, sans-serif; }\n\
                     code, pre { font-family: 'Cascadia Mono', Menlo !important; }\n\
                     h1 { font-family: \"Cascadia Mono\", monospace; }";
        assert_eq!(first_families(theme), ["Archivo", "Cascadia Mono"]);
        let faces = "@font-face { font-family: 'Archivo'; src: url('a.woff2'); }";
        assert_eq!(font_faces(faces), ["Archivo"]);
        assert!(first_families(faces).is_empty());
    }

    #[test]
    fn the_served_theme_must_be_the_committed_one() {
        let dir = std::env::temp_dir().join(format!("hpr-served-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (theme, site) = (dir.join("theme"), dir.join("site"));
        for path in [theme.join("fonts"), site.join("fonts"), site.join("theme")] {
            fs::create_dir_all(path).unwrap();
        }
        fs::write(theme.join("fonts/fonts.css"), "ours").unwrap();
        fs::write(theme.join("x.css"), "x").unwrap();
        fs::write(theme.join("README.md"), "not served").unwrap();
        fs::write(site.join("fonts/fonts.css"), "ours").unwrap();
        fs::write(site.join("theme/x.css"), "x").unwrap();
        fs::write(theme.join("favicon.svg"), "the mark").unwrap();
        fs::write(site.join("favicon.svg"), "the mark").unwrap();
        assert!(served_unchanged(&theme, &site).unwrap().is_empty());
        // mdBook's own book as the favicon.
        fs::write(site.join("favicon.svg"), "a book").unwrap();
        let problems = served_unchanged(&theme, &site).unwrap();
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].starts_with("favicon.svg is not theme/favicon.svg"),
            "{problems:?}"
        );
        fs::write(site.join("favicon.svg"), "the mark").unwrap();
        // mdBook's own fonts at the same address.
        fs::write(site.join("fonts/fonts.css"), "open sans").unwrap();
        let problems = served_unchanged(&theme, &site).unwrap();
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].starts_with("fonts/fonts.css is not"),
            "{problems:?}"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_copy_missing_from_the_theme_fails() {
        assert!(missing_copies(&crate::designs::root().unwrap().join(THEME)).is_empty());
        let dir = std::env::temp_dir().join(format!("hpr-copies-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        for (file, _) in COPIED {
            let path = dir.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "x").unwrap();
        }
        assert!(missing_copies(&dir).is_empty());
        fs::remove_file(dir.join("favicon.png")).unwrap();
        let problems = missing_copies(&dir);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].starts_with("theme/favicon.png is missing"),
            "{problems:?}"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_lock_names_the_design_commit() {
        let lock = "[[git]]\nname = \"other\"\ncommit = \"aaa\"\n\n[[git]]\n\
                    name = \"fusionspace-design\"\ncommit = \"32770a4f\"\n";
        assert_eq!(locked_commit(lock).unwrap(), "32770a4f");
        assert!(locked_commit("[[git]]\nname = \"other\"\n").is_err());
    }

    #[test]
    fn dates_are_iso() {
        assert!(is_date("2026-10-05"));
        assert!(!is_date("2026-10-5"));
        assert!(!is_date("2026/10/05"));
        assert!(!is_date("the date"));
    }

    #[test]
    fn a_font_needs_its_license_beside_it() {
        let dir = std::env::temp_dir().join(format!("hpr-theme-{}", std::process::id()));
        let fonts = dir.join("fonts");
        // A run that panicked here may have left the folder behind under the same process id.
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&fonts).unwrap();
        fs::write(fonts.join("Face-Regular.woff2"), b"x").unwrap();
        let problems = fonts_without_license(&dir).unwrap();
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("OFL-Face.txt"), "{problems:?}");
        fs::write(fonts.join("OFL-Face.txt"), b"license").unwrap();
        assert!(fonts_without_license(&dir).unwrap().is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    /// The committed theme passes its own source checks, and the copied stylesheet still draws
    /// the strip.
    #[test]
    fn the_committed_theme_passes() {
        let root = crate::designs::root().unwrap();
        let sources = check_sources(&root, &root.join(super::super::SOURCE)).unwrap();
        assert_eq!(sources.problems, Vec::<String>::new());
        let css = read(&root.join(THEME).join("fusionspace-mdbook.css")).unwrap();
        assert!(
            css.contains("body::before"),
            "the theme no longer draws the strip"
        );
    }
}
