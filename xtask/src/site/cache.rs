//! What `cargo xtask site --cache` keeps of the page checks' passing runs, so that a run lays out
//! and measures again only the pages that changed. The gate's site step runs with it; CI runs
//! without, and checks every page and canary every time.
//!
//! The layout check ([`super::pages`]) and the vitals check ([`super::vitals`]) each keep a file
//! in the checkout's cache folder ([`dir`]), outside the site, which each build clears. A file holds its run's key and, for
//! each page that passed, the SHA-256 of the page's HTML (its date of issue left out), when it was checked and what the check
//! measured of it: the lines of its prose, or its vitals. A run reuses a page's result when its key
//! is the cache's and the page's HTML is the same but for its date of issue ([`undated`]); it
//! checks the rest. The key ([`key`]) is what
//! every page's result depends on besides its own HTML:
//!
//! - **The checker**: the sources of both checks, the harness and the product system's rules they
//!   run in the browser, and this file ([`SOURCES`]), embedded when xtask is built.
//! - **The browser**: Chrome's program and the version it prints, and the system it runs on.
//! - **The part** of the widths, or of the pages, a run takes (`--widths K/N`).
//! - **Every other built file**: each file of the built site but its pages ([`role`]), such as
//!   the stylesheets, scripts, fonts, images and the sidebar's `toc.js`, which lists every page,
//!   so adding or renaming a page changes it. Two are left out: `searchindex.js`, which a page
//!   loads only when a reader opens the search, as the checks never do, and the API reference
//!   under `api/`, which neither check reads.
//!
//! With a key unlike the cache's, or no cache, every page is checked and the canaries run, as
//! without `--cache`. With the same key the canaries don't run, as they prove the checker and the
//! browser, which are unchanged. Only a check that passed keeps its results: a failing run leaves
//! the cache as it was, so the pages it checked are checked again next time. A page no longer in
//! the site is dropped. The file is written whole to a temporary file and then renamed, so a run
//! stopped part way leaves the old one.
//!
//! Not in the key: the machine's own fonts, which a page whose font fails to load would fall back
//! to (the layout check fails such a page), and its speed, which the vitals check measures each
//! run and slows the CPU to match; a reused page's vitals are those of the run that measured them,
//! and the line the run prints says how much that run slowed the CPU.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::pages::{self, Part};

/// The caches' folder, in the checkout's own git folder ([`dir`]).
const DIR: &str = "site-check-cache";
/// Where the caches are kept when git can't name the checkout's folder, relative to the workspace
/// root: beside the site, which each build clears, not in it.
const FALLBACK: &str = "target/site-check-cache";

/// Where the caches are kept for the checkout at `root`: [`DIR`] in its own git folder (`.git`,
/// or `.git/worktrees/<name>` for a worktree), else [`FALLBACK`]. Under `target/`, `cargo clean`
/// deleted them. Of the main checkout's ten full gates from 2026-11-19 15:33 UTC to 11-22, four
/// checked every page, in 792 to 831 s, against 49 to 218 s for the rest; two of the four came
/// just after a `cargo clean` (the 2026-11-22 review). Each checkout keeps its own, as before, so
/// a worktree on another branch, whose key differs, doesn't overwrite the main checkout's.
///
/// Git is asked with `--absolute-git-dir` (git 2.13 on), and its answer is taken only when it is
/// one line naming a folder that exists by an absolute path: an older git that doesn't know the
/// option prints it back and still succeeds. Git's own variables are cleared, so a `GIT_DIR` set
/// around the run doesn't send the cache to another repository.
pub(super) fn dir(root: &Path) -> PathBuf {
    let mut git = std::process::Command::new("git");
    git.arg("-C")
        .arg(root)
        .args(["rev-parse", "--absolute-git-dir"]);
    for variable in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_COMMON_DIR",
        "GIT_INDEX_FILE",
    ] {
        git.env_remove(variable);
    }
    git.output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|text| text.trim_end_matches(['\r', '\n']).to_owned())
        .filter(|text| !text.is_empty() && !text.contains('\n'))
        .map(PathBuf::from)
        .filter(|folder| folder.is_absolute() && folder.is_dir())
        .map_or_else(|| root.join(FALLBACK), |folder| folder.join(DIR))
}
/// The cache files' format. A file of another is not read, and every page is checked.
const FORMAT: u32 = 1;
/// The site's search index, which a page loads only when the search is opened.
const SEARCH_INDEX: &str = super::search::FILE;

/// The checker's sources, each by its name: what both checks run, here and in the browser. They
/// are built into xtask, so a change to any of them changes every key.
const SOURCES: [(&str, &str); 7] = [
    ("pages.rs", include_str!("pages.rs")),
    ("pages.js", include_str!("pages.js")),
    ("fusionspace-check.js", include_str!("fusionspace-check.js")),
    ("vitals.rs", include_str!("vitals.rs")),
    ("vitals.js", include_str!("vitals.js")),
    ("cdp.rs", include_str!("cdp.rs")),
    ("cache.rs", include_str!("cache.rs")),
];

/// One of the checks the cache keeps, each in its own file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Check {
    /// The pages' layout ([`super::pages`]).
    Layout,
    /// The pages' vitals ([`super::vitals`]).
    Vitals,
}

impl Check {
    fn name(self) -> &'static str {
        match self {
            Check::Layout => "layout",
            Check::Vitals => "vitals",
        }
    }

    /// The check's cache file, in [`dir`].
    fn file(self) -> String {
        format!("{}.json", self.name())
    }

    /// What the check does to a page, for the line a run prints.
    fn verb(self) -> &'static str {
        match self {
            Check::Layout => "checked",
            Check::Vitals => "measured",
        }
    }
}

/// What a file of the built site is to the checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    /// A page, which each check reads alone: its HTML decides whether it is checked again.
    Page,
    /// Any other file of the built site, in the key of every page.
    Shared,
    /// A file no check reads: the search index and the API reference.
    Unread,
}

/// What `path`, a file of the built site relative to it, is to the checks.
fn role(path: &str) -> Role {
    if path == SEARCH_INDEX || path.starts_with(&format!("{}/", super::API)) {
        Role::Unread
    } else if path.ends_with(".html") {
        Role::Page
    } else {
        Role::Shared
    }
}

/// What one run of a check depends on besides each page's own HTML, hashed into its [`key`].
struct Inputs<'a> {
    check: Check,
    /// The checker's sources, each by its name ([`SOURCES`]).
    sources: &'a [(&'a str, &'a str)],
    /// Chrome's program, and the version it prints.
    chrome: &'a str,
    version: &'a str,
    /// The system and processor, as `std::env::consts` names them.
    os: &'a str,
    arch: &'a str,
    part: Part,
    /// Every file of the built site a check may read, by its path relative to the site, to the
    /// SHA-256 of its bytes, a page's with its date of issue left out ([`snapshot`]); only the shared ones ([`role`]) go in the key.
    site: &'a BTreeMap<String, String>,
}

/// Adds one field to a key, its name and value each led by its length, so no two lists of fields
/// run together into the same bytes.
fn field(hasher: &mut Sha256, name: &str, value: &str) {
    for part in [name, value] {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
}

/// A run's key: the SHA-256 of its [`Inputs`]. Two runs with the same key check each page the
/// same way, so a page whose HTML is unchanged would get the same result.
fn key(inputs: &Inputs) -> String {
    let mut hasher = Sha256::new();
    field(&mut hasher, "format", &FORMAT.to_string());
    field(&mut hasher, "check", inputs.check.name());
    for (name, text) in inputs.sources {
        field(&mut hasher, "source", name);
        field(&mut hasher, "text", text);
    }
    field(&mut hasher, "chrome", inputs.chrome);
    field(&mut hasher, "version", inputs.version);
    field(&mut hasher, "os", inputs.os);
    field(&mut hasher, "arch", inputs.arch);
    let part = format!("{}/{}", inputs.part.index, inputs.part.count);
    field(&mut hasher, "part", &part);
    for (path, sha256) in inputs.site {
        if role(path) == Role::Shared {
            field(&mut hasher, "file", path);
            field(&mut hasher, "sha256", sha256);
        }
    }
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// A cache file: its run's key, and each page that passed.
#[derive(Serialize, Deserialize)]
struct File<T> {
    format: u32,
    key: String,
    pages: BTreeMap<String, Entry<T>>,
}

/// The first field of a cache file, read before the rest, so a file of another format is told
/// from a broken one.
#[derive(Deserialize)]
struct Format {
    format: u32,
}

/// One page that passed, as the cache keeps it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct Entry<T> {
    /// The SHA-256 of the page's HTML, its date of issue left out ([`undated`]).
    pub html: String,
    /// When it was checked, `YYYY-MM-DD HH:MM UTC`.
    pub checked: String,
    /// What the check measured of it.
    pub result: T,
}

/// Why a run reuses what it does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Basis {
    /// Chrome's version couldn't be read: nothing is reused or kept.
    Unversioned,
    /// No cache file, or one that couldn't be read.
    Absent,
    /// A file that isn't a cache of this format.
    Unreadable,
    /// A cache from a run with another key.
    Changed,
    /// A cache from a run with this run's key.
    Same,
}

/// What a run of a check takes from its cache ([`plan`]).
#[derive(Debug, PartialEq)]
pub(super) struct Plan<T> {
    /// The run's key; none when Chrome's version couldn't be read.
    key: Option<String>,
    /// Every page the run covers, to the SHA-256 of its HTML.
    pages: BTreeMap<String, String>,
    /// The pages to check now, in order of name.
    pub check: Vec<String>,
    /// Whether the canaries run.
    pub canaries: bool,
    /// The pages whose results are reused, each with its entry.
    pub reused: BTreeMap<String, Entry<T>>,
    pub basis: Basis,
}

/// What a run of a check whose key is `key` takes from its cache file, `cached` (none when there
/// is no file), for `pages`, each page it covers to the SHA-256 of its HTML. With the cache's key,
/// a page whose HTML is the cache's is reused and the rest are checked, without the canaries;
/// otherwise every page is checked, with the canaries. A page of the cache not in `pages` is left
/// out.
pub(super) fn plan<T: DeserializeOwned>(
    cached: Option<&str>,
    key: Option<&str>,
    pages: &BTreeMap<String, String>,
) -> Plan<T> {
    let everything = |basis| Plan {
        key: key.map(str::to_owned),
        pages: pages.clone(),
        check: pages.keys().cloned().collect(),
        canaries: true,
        reused: BTreeMap::new(),
        basis,
    };
    let Some(key) = key else {
        return everything(Basis::Unversioned);
    };
    let Some(text) = cached else {
        return everything(Basis::Absent);
    };
    let ours = serde_json::from_str::<Format>(text).is_ok_and(|head| head.format == FORMAT);
    let Some(file) = ours
        .then(|| serde_json::from_str::<File<T>>(text).ok())
        .flatten()
    else {
        return everything(Basis::Unreadable);
    };
    if file.key != key {
        return everything(Basis::Changed);
    }
    let mut kept = file.pages;
    let mut check = Vec::new();
    let mut reused = BTreeMap::new();
    for (page, html) in pages {
        match kept.remove(page) {
            Some(entry) if entry.html == *html => {
                reused.insert(page.clone(), entry);
            }
            _ => check.push(page.clone()),
        }
    }
    Plan {
        key: Some(key.to_owned()),
        pages: pages.clone(),
        check,
        canaries: false,
        reused,
        basis: Basis::Same,
    }
}

impl<T: Serialize> Plan<T> {
    /// The cache file this run leaves once its check is done, with `checked`, the result of each
    /// page it checked, dated `now`: none when the check failed (`passed` not set), so its pages
    /// are checked again next run, or when the run has no key; else every page it covers, checked
    /// or reused.
    pub(super) fn finish(
        &self,
        checked: &BTreeMap<String, T>,
        passed: bool,
        now: &str,
    ) -> Result<Option<String>, String> {
        let Some(key) = self.key.as_ref().filter(|_| passed) else {
            return Ok(None);
        };
        let mut pages = BTreeMap::new();
        for (page, html) in &self.pages {
            let entry = match (checked.get(page), self.reused.get(page)) {
                (Some(result), _) => Entry {
                    html: html.clone(),
                    checked: now.to_owned(),
                    result,
                },
                (None, Some(entry)) => Entry {
                    html: entry.html.clone(),
                    checked: entry.checked.clone(),
                    result: &entry.result,
                },
                (None, None) => {
                    return Err(format!(
                        "the page check's cache has no result for {page}, neither checked nor \
                         reused"
                    ));
                }
            };
            pages.insert(page.clone(), entry);
        }
        let file = File {
            format: FORMAT,
            key: key.clone(),
            pages,
        };
        serde_json::to_string_pretty(&file)
            .map(Some)
            .map_err(|err| format!("could not write the page check's cache: {err}"))
    }
}

impl<T> Plan<T> {
    /// The line a run prints for `check`: how many pages it checks now and reuses, and why.
    pub(super) fn describe(&self, check: Check) -> String {
        let name = match check {
            Check::Layout => "page layout cache",
            Check::Vitals => "page vitals cache",
        };
        let verb = check.verb();
        let why = match self.basis {
            Basis::Same => {
                let reused = if self.reused.is_empty() {
                    "none reused".to_owned()
                } else {
                    format!(
                        "{} reused from passing runs {}",
                        self.reused.len(),
                        dates(self.reused.values().map(|entry| entry.checked.as_str()))
                    )
                };
                return format!(
                    "{name}: {} of {} pages {verb} now, their HTML new or changed, and {reused}; \
                     the canaries not run, as the checker, Chrome and the site's other files are \
                     unchanged",
                    self.check.len(),
                    self.pages.len()
                );
            }
            Basis::Unversioned => {
                "Chrome's version couldn't be read (`--version`), so nothing is reused or kept"
                    .to_owned()
            }
            Basis::Absent => "there is no cache".to_owned(),
            Basis::Unreadable => format!("its cache file, {}, is not of this format", check.file()),
            Basis::Changed => "the checker, Chrome, the part (`--widths`), the system or another \
                               file of the built site changed since the cache's run"
                .to_owned(),
        };
        format!(
            "{name}: every page {verb} now ({}) and the canaries, as {why}",
            self.pages.len()
        )
    }
}

/// When the reused pages were checked: `on 2026-11-01 14:02 UTC`, or `from … to …`.
fn dates<'a>(dates: impl Iterator<Item = &'a str>) -> String {
    let mut dates: Vec<&str> = dates.collect();
    dates.sort_unstable();
    match (dates.first(), dates.last()) {
        (Some(first), Some(last)) if first != last => format!("from {first} to {last}"),
        (Some(first), _) => format!("on {first}"),
        _ => String::new(),
    }
}

/// What a run with `--cache` reads once for both checks: the built site's files, the browser and
/// the time.
pub(super) struct Run {
    /// Where the cache files are ([`dir`]).
    dir: PathBuf,
    /// Every file of the site a check may read, to the SHA-256 of its bytes, a page's with its
    /// date of issue left out ([`snapshot`]).
    site: BTreeMap<String, String>,
    chrome: String,
    /// What Chrome says its version is; none when that couldn't be read.
    version: Option<String>,
    part: Part,
    /// When the run started, as its entries are dated.
    now: String,
}

impl Run {
    /// Reads the site built in `output` and finds the browser, for a run of `part` in the
    /// workspace at `root`.
    pub(super) fn start(root: &Path, output: &Path, part: Part) -> Result<Run, String> {
        let chrome = pages::find_chrome()?;
        Ok(Run {
            dir: dir(root),
            site: snapshot(output)?,
            version: chrome_version(&chrome),
            chrome: chrome.display().to_string(),
            part,
            now: now(),
        })
    }

    fn key(&self, check: Check) -> Option<String> {
        let version = self.version.as_deref()?;
        Some(key(&Inputs {
            check,
            sources: &SOURCES,
            chrome: &self.chrome,
            version,
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            part: self.part,
            site: &self.site,
        }))
    }

    /// What `check` takes from its cache for `pages` ([`plan`]).
    pub(super) fn plan<T: DeserializeOwned>(
        &self,
        check: Check,
        pages: &[String],
    ) -> Result<Plan<T>, String> {
        let mut hashes = BTreeMap::new();
        for page in pages {
            let html = self
                .site
                .get(page)
                .ok_or_else(|| format!("the page check's cache didn't read {page}"))?;
            hashes.insert(page.clone(), html.clone());
        }
        // A file that can't be read is as none: every page is checked.
        let cached = fs::read_to_string(self.dir.join(check.file())).ok();
        Ok(plan(cached.as_deref(), self.key(check).as_deref(), &hashes))
    }

    /// Writes the cache `check`'s run leaves ([`Plan::finish`]), if it leaves one.
    pub(super) fn keep<T: Serialize>(
        &self,
        check: Check,
        plan: &Plan<T>,
        checked: &BTreeMap<String, T>,
        passed: bool,
    ) -> Result<(), String> {
        let Some(text) = plan.finish(checked, passed, &self.now)? else {
            return Ok(());
        };
        fs::create_dir_all(&self.dir)
            .map_err(|err| format!("could not create {}: {err}", self.dir.display()))?;
        let path = self.dir.join(check.file());
        let temporary = self
            .dir
            .join(format!("{}.{}.tmp", check.file(), std::process::id()));
        fs::write(&temporary, text)
            .map_err(|err| format!("could not write {}: {err}", temporary.display()))?;
        fs::rename(&temporary, &path)
            .map_err(|err| format!("could not write {}: {err}", path.display()))
    }
}

/// Every file of the site built in `output` that a check may read ([`role`]), by its path
/// relative to the site, `/` between folders, to the SHA-256 of its bytes; a page's are read
/// with its date of issue left out ([`undated`]).
fn snapshot(output: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();
    walk(output, "", &mut files)?;
    Ok(files)
}

fn walk(dir: &Path, prefix: &str, files: &mut BTreeMap<String, String>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("could not read {}: {err}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("could not read {}: {err}", dir.display()))?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let name = if prefix.is_empty() {
            file_name
        } else {
            format!("{prefix}/{file_name}")
        };
        let path = entry.path();
        if path.is_dir() {
            if role(&format!("{name}/")) != Role::Unread {
                walk(&path, &name, files)?;
            }
        } else if role(&name) != Role::Unread {
            let bytes = fs::read(&path)
                .map_err(|err| format!("could not read {}: {err}", path.display()))?;
            let digest = if role(&name) == Role::Page {
                crate::ork_supply::sha256(&undated(&bytes))
            } else {
                crate::ork_supply::sha256(&bytes)
            };
            files.insert(name, digest);
        }
    }
    Ok(())
}

/// What comes just before a built page's date of issue: the footer's `Issued` entry, which
/// [`super::frame`] writes on each page with a footer, and the title block's, which
/// [`super::theme::fill_issue_date`] fills on each page that carries the title block. Both carry the date of the
/// commit the site is built from, so before this every page's HTML changed with each new date
/// and the first run on a new date checked every page again (#1417).
const DATED: [&str; 2] = ["<span>Issued ", "<span id=\"issue-date\">"];

/// What a page's digest reads in place of each date after one of [`DATED`].
const UNDATED: &str = "YYYY-MM-DD";

/// A page's bytes as its digest reads them: each date of issue, a `YYYY-MM-DD` right after one
/// of [`DATED`] and right before `</span>`, read as [`UNDATED`]; every other byte as it is, and
/// bytes that aren't UTF-8 all as they are. So a page whose only change is its date is reused,
/// and any other change, a date elsewhere in its text among them, still has it checked again.
///
/// A new date changes digits within each date's ten characters. A page reused on that
/// account is not laid out with its new date by the gate; CI checks every page with no cache, so
/// a layout the date alone moved would be caught there.
fn undated(bytes: &[u8]) -> Cow<'_, [u8]> {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Cow::Borrowed(bytes);
    };
    let mut read = String::with_capacity(text.len());
    let mut rest = text;
    while let Some((at, marker)) = DATED
        .iter()
        .filter_map(|marker| rest.find(marker).map(|at| (at, marker.len())))
        .min()
    {
        read.push_str(&rest[..at + marker]);
        rest = &rest[at + marker..];
        if let Some(date) = rest.get(..UNDATED.len())
            && super::theme::is_date(date)
            && rest[UNDATED.len()..].starts_with("</span>")
        {
            read.push_str(UNDATED);
            rest = &rest[UNDATED.len()..];
        }
    }
    read.push_str(rest);
    Cow::Owned(read.into_bytes())
}

/// What `chrome --version` prints, such as `Google Chrome 142.0.7444.60`; none when it prints
/// nothing or fails. On Windows Chrome's program is a windowed one that prints nothing there, so
/// the version is not read, and a run with `--cache` checks every page.
fn chrome_version(chrome: &Path) -> Option<String> {
    if cfg!(windows) {
        return None;
    }
    let output = Command::new(chrome)
        .arg("--version")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let version = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    (output.status.success() && !version.is_empty()).then_some(version)
}

/// The time now, in UTC, to the minute: `2026-11-01 14:02 UTC`.
fn now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    format!(
        "{} {:02}:{:02} UTC",
        crate::refs::download::civil_date(seconds / 86_400),
        seconds % 86_400 / 3600,
        seconds % 3600 / 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::pages::Lines;
    use crate::site::vitals::Reading;

    /// When the tests' runs check their pages.
    const NOW: &str = "2026-11-01 14:02 UTC";

    /// What a run's key is made of, and the site it checks.
    #[derive(Clone)]
    struct Setting {
        sources: Vec<(&'static str, &'static str)>,
        chrome: &'static str,
        version: &'static str,
        part: Part,
        site: BTreeMap<String, String>,
    }

    impl Setting {
        /// Two pages, the files they load, the search index and the API reference's files, each
        /// path to a stand-in for its hash.
        fn new() -> Setting {
            let site = [
                ("index.html", "landing"),
                ("physics/aero.html", "aero"),
                ("css/site.css", "style"),
                ("toc.js", "sidebar"),
                ("fonts/Archivo.woff2", "face"),
                ("searchindex.js", "index"),
                ("api/hpr/index.html", "crate"),
                ("api/static.files/rustdoc.css", "rustdoc"),
            ];
            Setting {
                sources: vec![("pages.rs", "fn check() {}"), ("pages.js", "measure();")],
                chrome: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                version: "Google Chrome 142.0.7444.60",
                part: Part::WHOLE,
                site: site
                    .into_iter()
                    .map(|(path, hash)| (path.to_owned(), hash.to_owned()))
                    .collect(),
            }
        }

        fn key(&self) -> String {
            key(&Inputs {
                check: Check::Layout,
                sources: &self.sources,
                chrome: self.chrome,
                version: self.version,
                os: "macos",
                arch: "aarch64",
                part: self.part,
                site: &self.site,
            })
        }

        /// The pages a run covers, each to its HTML's hash.
        fn pages(&self) -> BTreeMap<String, String> {
            self.site
                .iter()
                .filter(|(path, _)| role(path) == Role::Page)
                .map(|(path, hash)| (path.clone(), hash.clone()))
                .collect()
        }

        /// What a run in this setting takes from `cached`.
        fn plan(&self, cached: Option<&str>) -> Plan<u32> {
            plan(cached, Some(&self.key()), &self.pages())
        }

        /// The cache a first run leaves that passes.
        fn first(&self) -> String {
            let plan = self.plan(None);
            assert_eq!(plan.basis, Basis::Absent);
            run(&plan, true, None).unwrap()
        }
    }

    /// Checks what `plan` checks, every page passing or every page failing (`passed`), and
    /// returns the cache file left: the new one, or `cached` when the run leaves none, as
    /// [`Run::keep`] then leaves the file as it was.
    fn run(plan: &Plan<u32>, passed: bool, cached: Option<String>) -> Option<String> {
        let checked = plan
            .check
            .iter()
            .map(|page| (page.clone(), page.len() as u32))
            .collect();
        plan.finish(&checked, passed, NOW).unwrap().or(cached)
    }

    /// Whether `plan` checks every page of `setting` and the canaries, reusing nothing.
    fn everything(plan: &Plan<u32>, setting: &Setting) -> bool {
        plan.check == setting.pages().into_keys().collect::<Vec<_>>()
            && plan.canaries
            && plan.reused.is_empty()
    }

    fn reused(plan: &Plan<u32>) -> Vec<&str> {
        plan.reused.keys().map(String::as_str).collect()
    }

    #[test]
    fn with_nothing_changed_nothing_is_checked() {
        let setting = Setting::new();
        let cached = setting.first();
        let plan = setting.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Same);
        assert!(plan.check.is_empty(), "{:?}", plan.check);
        assert!(!plan.canaries);
        assert_eq!(reused(&plan), ["index.html", "physics/aero.html"]);
        assert_eq!(plan.reused["physics/aero.html"].result, 17);
        // The run leaves the cache as it found it, each page dated when it was checked.
        let left = run(&plan, true, None).unwrap();
        assert_eq!(left, cached);
    }

    #[test]
    fn a_changed_page_alone_is_checked() {
        let mut setting = Setting::new();
        let cached = setting.first();
        setting
            .site
            .insert("physics/aero.html".to_owned(), "aero, edited".to_owned());
        let plan = setting.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Same);
        assert_eq!(plan.check, ["physics/aero.html"]);
        assert!(!plan.canaries);
        assert_eq!(reused(&plan), ["index.html"]);
        // Kept with its new HTML, so the next run reuses it.
        let left = run(&plan, true, None).unwrap();
        assert!(setting.plan(Some(&left)).check.is_empty());
    }

    /// #1417: the same pages built under two dates of issue are all reused, and a change to
    /// anything else in a page, a date in its text among them, still has that page checked.
    #[test]
    fn a_page_whose_only_change_is_its_date_of_issue_is_reused() {
        let page = |date: &str, text: &str| {
            format!(
                "<main><p>{text}</p></main><footer><span>Version 0.1.0</span>\
                 <span>Issued {date}</span></footer>"
            )
        };
        let build = |date: &str, aero: &str| {
            let dir = tempfile::tempdir().unwrap();
            fs::create_dir(dir.path().join("physics")).unwrap();
            let landing = format!(
                "<span id=\"issue-date\">{date}</span>{}",
                page(date, "home")
            );
            fs::write(dir.path().join("index.html"), landing).unwrap();
            fs::write(dir.path().join("physics/aero.html"), page(date, aero)).unwrap();
            fs::write(dir.path().join("site.css"), "p { margin: 0 }").unwrap();
            snapshot(dir.path()).unwrap()
        };
        let mut setting = Setting::new();
        setting.site = build("2026-11-18", "drag");
        let cached = setting.first();

        setting.site = build("2026-11-19", "drag");
        let plan = setting.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Same);
        assert!(plan.check.is_empty(), "{:?}", plan.check);
        assert!(!plan.canaries);
        assert_eq!(reused(&plan), ["index.html", "physics/aero.html"]);

        // A date in the page's own text is not its date of issue: cached with one, the page is
        // checked again when that date changes.
        setting.site = build("2026-11-18", "drag, read 2026-11-18");
        let cached = run(&setting.plan(Some(&cached)), true, None).unwrap();
        setting.site = build("2026-11-19", "drag, read 2026-11-19");
        assert_eq!(setting.plan(Some(&cached)).check, ["physics/aero.html"]);
        // Nor is a word changed beside a new date.
        setting.site = build("2026-11-19", "lift");
        assert_eq!(setting.plan(Some(&cached)).check, ["physics/aero.html"]);
    }

    #[test]
    fn only_a_date_right_after_an_issue_marker_and_before_its_span_s_end_is_read_as_undated() {
        let read = |text: &str| String::from_utf8(undated(text.as_bytes()).into_owned()).unwrap();
        assert_eq!(
            read("<span>Issued 2026-11-19</span><span id=\"issue-date\">2026-11-19</span>"),
            "<span>Issued YYYY-MM-DD</span><span id=\"issue-date\">YYYY-MM-DD</span>"
        );
        // Not a date, a date running on, a date in other markup, the marker last, a character
        // of more than one byte where the date would end: each as it is.
        for text in [
            "<span>Issued soon</span>",
            "<span>Issued 2026-11-19, late</span>",
            "<span>Read 2026-11-19</span>",
            "<p>Issued 2026-11-19</p>",
            "the end: <span>Issued ",
            "<span>Issued 2026-11-1\u{e9}</span>",
        ] {
            assert_eq!(read(text), text);
        }
        // Bytes that aren't UTF-8 are read as they are.
        let raw = [0xff, b'<', 0xfe];
        assert_eq!(undated(&raw).as_ref(), raw);
    }

    #[test]
    fn a_new_page_is_checked() {
        let mut setting = Setting::new();
        let cached = setting.first();
        setting
            .site
            .insert("weather.html".to_owned(), "weather".to_owned());
        let plan = setting.plan(Some(&cached));
        assert_eq!(plan.check, ["weather.html"]);
        assert!(!plan.canaries);
        assert_eq!(reused(&plan), ["index.html", "physics/aero.html"]);
    }

    #[test]
    fn a_removed_page_is_dropped() {
        let mut setting = Setting::new();
        let cached = setting.first();
        setting.site.remove("physics/aero.html");
        let plan = setting.plan(Some(&cached));
        assert!(plan.check.is_empty(), "{:?}", plan.check);
        assert_eq!(reused(&plan), ["index.html"]);
        let left = run(&plan, true, None).unwrap();
        let file: File<u32> = serde_json::from_str(&left).unwrap();
        assert_eq!(file.pages.keys().collect::<Vec<_>>(), ["index.html"]);
    }

    #[test]
    fn a_changed_file_every_page_loads_checks_everything() {
        let setting = Setting::new();
        let cached = setting.first();
        for file in ["css/site.css", "toc.js", "fonts/Archivo.woff2"] {
            let mut changed = setting.clone();
            changed.site.insert(file.to_owned(), "edited".to_owned());
            let plan = changed.plan(Some(&cached));
            assert_eq!(plan.basis, Basis::Changed, "{file}");
            assert!(everything(&plan, &changed), "{file}: {plan:?}");
        }
        // A new one too, such as an image.
        let mut changed = setting.clone();
        changed
            .site
            .insert("images/drag.svg".to_owned(), "figure".to_owned());
        assert!(everything(&changed.plan(Some(&cached)), &changed));
    }

    #[test]
    fn the_search_index_and_the_api_reference_check_nothing() {
        let setting = Setting::new();
        let cached = setting.first();
        for file in [
            "searchindex.js",
            "api/hpr/index.html",
            "api/static.files/rustdoc.css",
        ] {
            let mut changed = setting.clone();
            changed.site.insert(file.to_owned(), "edited".to_owned());
            let plan = changed.plan(Some(&cached));
            assert_eq!(plan.basis, Basis::Same, "{file}");
            assert!(plan.check.is_empty(), "{file}: {:?}", plan.check);
            assert!(!plan.canaries, "{file}");
        }
    }

    #[test]
    fn a_changed_checker_checks_everything() {
        let setting = Setting::new();
        let cached = setting.first();
        let mut changed = setting.clone();
        changed.sources[1].1 = "measure(); measure();";
        let plan = changed.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Changed);
        assert!(everything(&plan, &changed), "{plan:?}");
        // The real key holds every file the checks build in with `include_str!`.
        let names: Vec<&str> = SOURCES.iter().map(|(name, _)| *name).collect();
        for (name, text) in SOURCES {
            for (at, _) in text.match_indices("include_str!(\"") {
                let rest = &text[at + "include_str!(\"".len()..];
                let included = &rest[..rest.find('"').unwrap()];
                assert!(names.contains(&included), "{name} builds in {included}");
            }
        }
    }

    #[test]
    fn another_chrome_checks_everything() {
        let setting = Setting::new();
        let cached = setting.first();
        let mut newer = setting.clone();
        newer.version = "Google Chrome 143.0.7499.40";
        let plan = newer.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Changed);
        assert!(everything(&plan, &newer), "{plan:?}");
        let mut other = setting.clone();
        other.chrome = "/Applications/Chromium.app/Contents/MacOS/Chromium";
        assert!(everything(&other.plan(Some(&cached)), &other));
    }

    #[test]
    fn without_chrome_s_version_nothing_is_reused_or_kept() {
        let setting = Setting::new();
        let cached = setting.first();
        let plan: Plan<u32> = plan(Some(&cached), None, &setting.pages());
        assert_eq!(plan.basis, Basis::Unversioned);
        assert!(everything(&plan, &setting), "{plan:?}");
        assert_eq!(run(&plan, true, None), None);
    }

    #[test]
    fn another_part_checks_everything() {
        let setting = Setting::new();
        let cached = setting.first();
        let mut other = setting.clone();
        other.part = Part { index: 2, count: 3 };
        let plan = other.plan(Some(&cached));
        assert_eq!(plan.basis, Basis::Changed);
        assert!(everything(&plan, &other), "{plan:?}");
    }

    #[test]
    fn a_broken_or_foreign_cache_checks_everything() {
        let setting = Setting::new();
        let cached = setting.first();
        let format = format!("\"format\": {FORMAT}");
        assert!(cached.contains(&format), "{cached}");
        let shaped = cached.replace("\"result\": 17", "\"result\": \"seventeen\"");
        assert_ne!(shaped, cached);
        for broken in [
            String::new(),
            "not a cache".to_owned(),
            cached[..cached.len() / 2].to_owned(),
            cached.replace(&format, &format!("\"format\": {}", FORMAT + 1)),
            cached.replace(&format, "\"format\": \"1\""),
            shaped,
        ] {
            let plan = setting.plan(Some(&broken));
            assert_eq!(plan.basis, Basis::Unreadable, "{broken}");
            assert!(everything(&plan, &setting), "{broken}: {plan:?}");
        }
    }

    #[test]
    fn a_page_that_failed_is_checked_again() {
        let mut setting = Setting::new();
        let cached = setting.first();
        setting
            .site
            .insert("index.html".to_owned(), "landing, broken".to_owned());
        let plan = setting.plan(Some(&cached));
        assert_eq!(plan.check, ["index.html"]);
        // The check fails: the cache is left as it was.
        let left = run(&plan, false, Some(cached.clone())).unwrap();
        assert_eq!(left, cached);
        let again = setting.plan(Some(&left));
        assert_eq!(again.check, ["index.html"]);
        assert_eq!(reused(&again), ["physics/aero.html"]);
        // A first run that fails leaves no cache at all.
        let first = Setting::new().plan(None);
        assert_eq!(run(&first, false, None), None);
    }

    #[test]
    fn kept_results_read_back_as_measured() {
        let pages: BTreeMap<String, String> = [("index.html".to_owned(), "landing".to_owned())]
            .into_iter()
            .collect();
        fn only<T>(value: T) -> BTreeMap<String, T> {
            [("index.html".to_owned(), value)].into_iter().collect()
        }
        let lines = Lines {
            medians: Some((0.1 + 0.2, 61.5)),
            longest: 88,
        };
        let text = plan::<Lines>(None, Some("key"), &pages)
            .finish(&only(lines.clone()), true, NOW)
            .unwrap()
            .unwrap();
        let back = plan::<Lines>(Some(&text), Some("key"), &pages);
        assert_eq!(back.reused["index.html"].result, lines);
        assert_eq!(back.reused["index.html"].checked, NOW);
        let reading = Reading {
            lcp_ms: 1_234.567_890_123,
            cls: 0.1 + 0.2,
            inp_ms: 48.000_000_000_001,
            benchmark: 4412.25,
            slowdown: 3.0 + 3112.25 / 233.0,
        };
        let text = plan::<Reading>(None, Some("key"), &pages)
            .finish(&only(reading), true, NOW)
            .unwrap()
            .unwrap();
        let back = plan::<Reading>(Some(&text), Some("key"), &pages);
        assert_eq!(back.reused["index.html"].result, reading);
    }

    #[test]
    fn the_line_says_what_was_checked_and_reused() {
        let mut setting = Setting::new();
        let cached = setting.first();
        setting
            .site
            .insert("index.html".to_owned(), "landing, edited".to_owned());
        let line = setting.plan(Some(&cached)).describe(Check::Layout);
        assert_eq!(
            line,
            "page layout cache: 1 of 2 pages checked now, their HTML new or changed, and 1 \
             reused from passing runs on 2026-11-01 14:02 UTC; the canaries not run, as the \
             checker, Chrome and the site's other files are unchanged"
        );
        let line = setting.plan(None).describe(Check::Vitals);
        assert_eq!(
            line,
            "page vitals cache: every page measured now (2) and the canaries, as there is no \
             cache"
        );
        assert_eq!(
            dates(["2026-11-01 14:02 UTC", "2026-10-30 09:12 UTC"].into_iter()),
            "from 2026-10-30 09:12 UTC to 2026-11-01 14:02 UTC"
        );
    }

    #[test]
    fn the_time_is_written_to_the_minute_in_utc() {
        let now = now();
        assert_eq!(now.len(), "2026-11-01 14:02 UTC".len(), "{now}");
        assert!(now.ends_with(" UTC") && now.as_bytes()[13] == b':', "{now}");
    }

    #[test]
    fn the_cache_is_kept_in_the_checkouts_git_folder_where_cargo_clean_leaves_it() {
        // The workspace is a git checkout here and on CI, so its cache is in its git folder.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask sits in the workspace's root");
        let kept = dir(root);
        assert!(
            kept.ends_with(DIR) && !kept.starts_with(root.join("target")),
            "{}",
            kept.display()
        );
        // The folder git named is a real git folder: it holds a HEAD.
        let folder = kept.parent().expect("the cache sits in a folder");
        assert!(
            folder.is_absolute() && folder.join("HEAD").is_file(),
            "{}",
            folder.display()
        );
        // A folder git doesn't know keeps the cache beside the site, as before.
        let outside = std::env::temp_dir().join(format!("hpr-site-cache-{}", std::process::id()));
        fs::create_dir_all(&outside).expect("a scratch folder");
        let fallback = dir(&outside);
        fs::remove_dir_all(&outside).expect("the scratch folder removed");
        assert_eq!(fallback, outside.join(FALLBACK));
    }
}
