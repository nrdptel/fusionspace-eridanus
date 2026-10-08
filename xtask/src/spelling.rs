//! US spelling and no em dashes in the project's own writing (M0.6d; ADR-164, `docs/writing.md`).
//!
//! The product system writes US English and never uses an em dash: a colon, a comma or a period
//! does the job. This check reads every tracked text file under [`SCOPE`] and reports each em dash
//! and each form of the eight words on [`RULES`]' list (centre, metre, catalogue, licence, colour,
//! analyse, behaviour, modelling) in the file's *writing*: the prose of a page, and the comments
//! and string literals of a source file.
//!
//! It leaves out four things, each for a reason:
//!
//! - **Frozen records** ([`FROZEN`]): a decision record or a finished roadmap entry is not edited
//!   after the fact, so it keeps the spelling it was written in; so does a link that quotes a
//!   record's title, `[ADR-008: Subsonic normal force and ...]`.
//! - **Code identifiers** in the writing: a name in backticks, a word joined to others by `_`, a
//!   word with a digit or a capital inside it, a string literal that is one bare word (a
//!   serialized key), and everything in a source file outside its comments and strings. Those are
//!   names, which [`names`] reads instead (below).
//! - **Addresses**: a URL or a link target names something elsewhere, often a heading of a frozen
//!   record, and changing it breaks the link.
//! - **Quotations, source titles and product names** listed in [`ALLOWED`], each with the phrase
//!   that holds it. The check prints how many it allows, and fails on an entry that no longer
//!   matches anything, so the list can't go stale.
//!
//! "Analyses" is not reported: it is also the plural of "analysis", which US English spells the
//! same way.
//!
//! **Names** (M0.6e) are read too: every name in a source file's code, every string literal that
//! is one bare key, and in writing each word shaped like a name, each name in a format string and
//! each word of inline code. A name is split into words at `_`, digits and camel case's capitals,
//! and is reported with its US form (`centre_of_pressure_m`, call it `center_of_pressure_m`). A
//! key that files written before M0.6e hold, or that another program's format holds (GDAL's
//! `"metre"`), is read and never written; it stays, listed in [`ALLOWED`] like a quotation.
//!
//! `cargo xtask spelling --fix` replaces the words (not the em dashes, which need a sentence
//! rewritten, nor the names, which need their old keys kept readable) wherever the check would
//! report them, with the case of the original kept.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

pub const USAGE: &str =
    "  spelling [--fix]        Check that the project's writing and names use US spelling and
                           the writing no em dash (M0.6d, M0.6e): pages, comments, strings and
                           code names, not frozen records. --fix replaces the misspelled words
                           in place, not the names.";

/// The paths checked, relative to the workspace root (M0.6d1; M0.6d2 adds `crates`, `xtask` and
/// `schema`; M10.1d1 the changelog).
const SCOPE: [&str; 8] = [
    "docs",
    "README.md",
    "CHANGELOG.md",
    "crates",
    "xtask",
    "schema",
    "python",
    "scripts",
];

/// This file, which isn't read: its word list and its tests' inputs hold every misspelling on
/// purpose. The summary names it.
const THIS_CHECK: &str = "xtask/src/spelling.rs";

/// Frozen records, never edited after the fact: the decision records, the finished roadmap, and
/// the index `cargo xtask records` writes from the records' own titles and summaries.
const FROZEN: [&str; 3] = [
    "docs/decisions/",
    "docs/roadmap-done.md",
    "docs/DECISIONS.md",
];

/// Each misspelling as a lowercase piece of a word, with its US form. A word is checked against
/// the pieces in this order and takes the first that it contains, so a longer piece comes before
/// a shorter one it holds ("centred" before "centre").
const RULES: [(&str, &str); 17] = [
    ("centred", "centered"),
    ("centring", "centering"),
    ("centre", "center"),
    ("metre", "meter"),
    ("catalogued", "cataloged"),
    ("cataloguing", "cataloging"),
    ("catalogue", "catalog"),
    ("licence", "license"),
    ("colour", "color"),
    ("analysed", "analyzed"),
    ("analysing", "analyzing"),
    ("analyser", "analyzer"),
    // "Analyses" is left alone (the module's docs say why); `misspelling` skips it.
    ("analyse", "analyze"),
    ("behaviour", "behavior"),
    ("modelling", "modeling"),
    ("modelled", "modeled"),
    ("modeller", "modeler"),
];

/// Why a match is allowed to stay.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reason {
    /// Someone else's words, quoted.
    Quotation,
    /// The title of a paper, report, book or standard.
    Title,
    /// A product, organization or command name, which keeps the spelling it has.
    Name,
    /// A key or value that an older file or another program's format holds: read, never written
    /// (M0.6e).
    Key,
}

/// The matches allowed to stay: the file, a phrase on one line of it that holds the match, and
/// why. Every match inside the phrase is allowed, on every line of the file that holds it.
const ALLOWED: &[(&str, &str, Reason)] = &[
    // Two decision records' file names, frozen with the records, which the name check's
    // allowlist names.
    (
        "xtask/src/names.rs",
        "0070-m2-2e-split-mass-and-centre-of-mass-first-then.md",
        Reason::Name,
    ),
    (
        "xtask/src/names.rs",
        "0102-tube-fins-centre-of-pressure-measured-against.md",
        Reason::Name,
    ),
    (
        "docs/format/era5.md",
        "European Centre for Medium-Range Weather Forecasts",
        Reason::Name,
    ),
    // The name runs on to the next line.
    ("docs/nomads.md", "the European Centre", Reason::Name),
    (
        "docs/glossary.md",
        "European Centre for Medium-Range Weather Forecasts",
        Reason::Name,
    ),
    (
        "docs/sensitivity.md",
        "*Environmental Modelling & Software*",
        Reason::Title,
    ),
    // The names of the Loft demo designs (`validation/fixtures/ork/loft-demo/`), as their `.ork`
    // files hold them: the guide's figure, the pages that show a run, and the reader's snapshots.
    (
        "docs/images/sim-plot.svg",
        "Loft Demo 54mm — dual deploy",
        Reason::Name,
    ),
    ("docs/cli.md", "Loft Demo 54mm — dual deploy", Reason::Name),
    (
        "docs/cli.md",
        "Loft Demo 38mm — motor comparison",
        Reason::Name,
    ),
    (
        "docs/format/hpr.md",
        "Loft Demo 38mm — stable trainer",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_boattail.snap",
        "Loft Demo 38mm — boattail + elliptical fins",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_dual_deploy.snap",
        "Loft Demo 54mm — dual deploy",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_multi_config.snap",
        "Loft Demo 38mm — motor comparison",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_payload_separation.snap",
        "Loft Demo 38mm — payload separation",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_quirks.snap",
        "Quirks regression — auto radii, legacy tags, boattail",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_single_deploy.snap",
        "Loft Demo 38mm — single deploy",
        Reason::Name,
    ),
    (
        "crates/hpr-io/src/ork/snapshots/hpr_io__ork__tests__demo_stable.snap",
        "Loft Demo 38mm — stable trainer",
        Reason::Name,
    ),
    // A test's copy of a frozen record: its file name and title.
    ("xtask/src/site.rs", "0001-licence", Reason::Name),
    ("xtask/src/site.rs", "ADR-001: Licence", Reason::Title),
    // The names of OpenRocket probes, which the oracle's recording
    // (`validation/fixtures/ork/openrocket-conventions.json`) keys its answers by.
    (
        "crates/hpr-validate/src/openrocket.rs",
        "a centre of gravity override on",
        Reason::Name,
    ),
    (
        "crates/hpr-validate/src/openrocket.rs",
        "the centre covering the part inside",
        Reason::Name,
    ),
    (
        "crates/hpr-validate/src/openrocket.rs",
        "covering the part inside and the centre not",
        Reason::Name,
    ),
    (
        "crates/hpr-validate/src/openrocket.rs",
        "a centre flag that says true",
        Reason::Name,
    ),
    // The motor site's API describing its hazmat labels, quoted.
    (
        "crates/hpr-net/src/motor_finder.rs",
        "the limit — vendor-dependent",
        Reason::Quotation,
    ),
    // The heading of motor.fusionspace.co's API page, quoted.
    ("docs/motor-stock.md", "\"Data licence\"", Reason::Quotation),
    // The style guide quoting what the check reports for a name.
    (
        "docs/writing.md",
        "reports `centre_of_pressure_m` as",
        Reason::Quotation,
    ),
    // A frozen decision record's title, ADR-001's, in a test of the records page.
    (
        "xtask/src/site.rs",
        "record(\"ADR-001\", \"Licence\")",
        Reason::Title,
    ),
    // GDAL's names for a raster's vertical unit, as GeoTIFF metadata and rasterio write them.
    (
        "crates/hpr-io/src/geotiff/mod.rs",
        "\"metre\" | \"meter\" | \"metres\"",
        Reason::Key,
    ),
    (
        "crates/hpr-io/src/geotiff/tests.rs",
        "role=\"unittype\"\"#, \"metre\"",
        Reason::Key,
    ),
    (
        "crates/hpr-io/tests/fixtures/geotiff/rasterio.json",
        "\"units\": \"metre\"",
        Reason::Key,
    ),
    (
        "crates/hpr-io/tests/geotiff_rasterio.rs",
        "\"metre\" => 1.0",
        Reason::Key,
    ),
    // The names written before M0.6e's US names, which still read: each serde alias, its
    // documentation, and the test that reads it.
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "`centre_east_m`",
        Reason::Key,
    ),
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "alias = \"centre_east_m\"",
        Reason::Key,
    ),
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "\\\"centre_east_m\\\"",
        Reason::Key,
    ),
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "`centre_north_m`",
        Reason::Key,
    ),
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "alias = \"centre_north_m\"",
        Reason::Key,
    ),
    (
        "crates/hpr-analysis/src/ellipse.rs",
        "\\\"centre_north_m\\\"",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/checks.rs",
        "`centre_outside_rocket`",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/checks.rs",
        "alias = \"centre_outside_rocket\"",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/checks.rs",
        ", \"centre_outside_rocket\")",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/tree.rs",
        "`centre_overridden`",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/tree.rs",
        "alias = \"centre_overridden\"",
        Reason::Key,
    ),
    (
        "crates/hpr-design/src/tree.rs",
        "\\\"centre_overridden\\\"",
        Reason::Key,
    ),
    (
        "crates/hpr-aero/src/shock_expansion.rs",
        "`centre_of_pressure_m`",
        Reason::Key,
    ),
    (
        "crates/hpr-aero/src/shock_expansion.rs",
        "alias = \"centre_of_pressure_m\"",
        Reason::Key,
    ),
    (
        "crates/hpr-aero/src/shock_expansion.rs",
        ", \"centre_of_pressure_m\")",
        Reason::Key,
    ),
    ("crates/hpr-io/src/geotiff/mod.rs", "`Metre`", Reason::Key),
    (
        "crates/hpr-io/src/geotiff/mod.rs",
        "alias = \"Metre\"",
        Reason::Key,
    ),
    ("crates/hpr-io/src/geotiff/tests.rs", "`Metre`", Reason::Key),
    (
        "crates/hpr-io/src/geotiff/tests.rs",
        "\\\"Metre\\\"",
        Reason::Key,
    ),
    (
        "crates/hpr-validate/src/flight_metrics.rs",
        "`centre_of_dry_mass`",
        Reason::Key,
    ),
    (
        "crates/hpr-validate/src/flight_metrics.rs",
        "alias = \"centre_of_dry_mass\"",
        Reason::Key,
    ),
    (
        "crates/hpr-validate/src/tests.rs",
        "`centre_of_dry_mass`",
        Reason::Key,
    ),
    (
        "crates/hpr-validate/src/tests.rs",
        "\\\"centre_of_dry_mass\\\"",
        Reason::Key,
    ),
];

/// One thing the check reports.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Finding {
    /// The byte offset in the file.
    offset: usize,
    /// The text found.
    found: String,
    /// The US form, or `None` for an em dash.
    instead: Option<String>,
    /// Whether it is a name ([`names`]) rather than a word of writing.
    name: bool,
}

/// What the check found over the whole scope.
struct Report {
    files: usize,
    /// Entries of [`SCOPE`] with no tracked file, named so the summary doesn't claim them.
    empty: Vec<&'static str>,
    /// `path:line: message`, one per finding not allowed.
    problems: Vec<String>,
    /// How many of the problems are code names.
    names: usize,
    /// How many matches each reason allowed.
    allowed: [usize; 4],
    /// [`ALLOWED`] entries that matched nothing.
    stale: Vec<String>,
}

pub fn run(args: &[String]) -> Result<(), String> {
    let fix = match args {
        [] => false,
        [flag] if flag == "--fix" => true,
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    if fix {
        let changed = fix_words(&root)?;
        println!("spelling: replaced words in {changed} files");
    }
    let report = check(&root)?;
    println!("{}", summary(&report));
    if report.problems.is_empty() && report.stale.is_empty() {
        return Ok(());
    }
    let mut message = String::new();
    for problem in &report.problems {
        let _ = writeln!(message, "{problem}");
    }
    for entry in &report.stale {
        let _ = writeln!(message, "an allowed phrase that matches nothing: {entry}");
    }
    let _ = write!(
        message,
        "{} findings; docs/writing.md says what to write instead",
        report.problems.len()
    );
    Err(message)
}

/// The line the check prints, with the exceptions it allows.
fn summary(report: &Report) -> String {
    let [quotations, titles, names, keys] = report.allowed;
    let empty = if report.empty.is_empty() {
        String::new()
    } else {
        format!(" ({} holding none)", report.empty.join(", "))
    };
    format!(
        "spelling: {} files under {}{empty}, {} findings ({} in code names); {} exceptions \
         allowed ({quotations} in quotations, {titles} in source titles, {names} in names, {keys} \
         in keys read from older files or other formats), past frozen records ({}), this check's \
         own word list ({THIS_CHECK}) and addresses",
        report.files,
        SCOPE.join(" "),
        report.problems.len(),
        report.names,
        quotations + titles + names + keys,
        FROZEN.join(", "),
    )
}

/// The tracked files in scope that the check reads, relative to `root` (frozen records and data
/// left out), and the entries of [`SCOPE`] that hold no tracked file at all.
fn files(root: &Path) -> Result<(Vec<String>, Vec<&'static str>), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z", "--"])
        .args(SCOPE)
        .output()
        .map_err(|e| format!("running `git ls-files`: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "`git ls-files` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let listing = String::from_utf8(output.stdout).map_err(|e| format!("git ls-files: {e}"))?;
    let tracked: Vec<&str> = listing
        .split('\0')
        .filter(|path| !path.is_empty())
        .collect();
    let empty = SCOPE
        .into_iter()
        .filter(|entry| {
            !tracked
                .iter()
                .any(|path| path == entry || path.starts_with(&format!("{entry}/")))
        })
        .collect();
    let read = tracked
        .into_iter()
        .filter(|path| !FROZEN.iter().any(|frozen| path.starts_with(frozen)))
        .filter(|path| *path != THIS_CHECK)
        .filter(|path| kind(path).is_some())
        .map(str::to_owned)
        .collect();
    Ok((read, empty))
}

/// Each decision record's title as its heading gives it, `ADR-008: Subsonic ... (2026-09-17)`. A
/// link whose text is the start of one quotes a frozen record.
fn titles(root: &Path) -> Result<Vec<String>, String> {
    let dir = root.join(crate::records::DECISIONS_DIR);
    let entries = fs::read_dir(&dir).map_err(|e| format!("reading {}: {e}", dir.display()))?;
    let mut titles = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| format!("reading {}: {e}", dir.display()))?
            .path();
        let text =
            fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        if let Some(title) = text.lines().next().and_then(|l| l.strip_prefix("# ADR-")) {
            titles.push(format!("ADR-{title}"));
        }
    }
    Ok(titles)
}

/// The entry of [`ALLOWED`] that allows `finding` in `path`, if any.
fn allowing(path: &str, text: &str, finding: &Finding) -> Option<usize> {
    let (_, line, column) = locate(text, finding.offset);
    ALLOWED.iter().position(|(file, phrase, _)| {
        *file == path
            && line.match_indices(phrase).any(|(start, _)| {
                start <= column && column + finding.found.len() <= start + phrase.len()
            })
    })
}

fn check(root: &Path) -> Result<Report, String> {
    let (paths, empty) = files(root)?;
    let titles = titles(root)?;
    let mut report = Report {
        files: paths.len(),
        empty,
        problems: Vec::new(),
        names: 0,
        allowed: [0; 4],
        stale: Vec::new(),
    };
    let mut used = vec![false; ALLOWED.len()];
    for path in &paths {
        let Some(kind) = kind(path) else { continue };
        let text =
            fs::read_to_string(root.join(path)).map_err(|e| format!("reading {path}: {e}"))?;
        tally(&mut report, &mut used, path, &text, kind, &titles);
    }
    for (index, (file, phrase, _)) in ALLOWED.iter().enumerate() {
        if !used[index] {
            report.stale.push(format!("{file}: \"{phrase}\""));
        }
    }
    Ok(report)
}

/// Adds one file's findings to `report`: each one allowed is counted (and its [`ALLOWED`] entry
/// marked in `used`), every other one is a problem, wherever in [`SCOPE`] the file sits.
fn tally(
    report: &mut Report,
    used: &mut [bool],
    path: &str,
    text: &str,
    kind: Kind,
    titles: &[String],
) {
    let mut findings = scan(text, kind, titles);
    findings.extend(names(text, kind, titles));
    findings.sort_by_key(|finding| finding.offset);
    for finding in findings {
        let (line_number, _, _) = locate(text, finding.offset);
        if let Some(index) = allowing(path, text, &finding) {
            used[index] = true;
            report.allowed[ALLOWED[index].2 as usize] += 1;
            continue;
        }
        if finding.name {
            report.names += 1;
        }
        report.problems.push(match &finding.instead {
            Some(instead) if finding.name => format!(
                "{path}:{line_number}: the name `{}` holds UK spelling; call it `{instead}`",
                finding.found
            ),
            Some(instead) => format!(
                "{path}:{line_number}: \"{}\" is UK spelling; write \"{instead}\"",
                finding.found
            ),
            None => format!("{path}:{line_number}: an em dash; use a colon, a comma or a period"),
        });
    }
}

/// Replaces every misspelled word the check would report, file by file; returns how many files
/// changed. Allowed matches are left as they are.
fn fix_words(root: &Path) -> Result<usize, String> {
    let mut changed = 0;
    let titles = titles(root)?;
    for path in files(root)?.0 {
        let Some(kind) = kind(&path) else { continue };
        let full = root.join(&path);
        let text = fs::read_to_string(&full).map_err(|e| format!("reading {path}: {e}"))?;
        let fixed = fixed(&path, &text, kind, &titles);
        if fixed != text {
            fs::write(&full, fixed).map_err(|e| format!("writing {path}: {e}"))?;
            changed += 1;
        }
    }
    Ok(changed)
}

/// `text`, the contents of `path`, with each word the check would report replaced.
fn fixed(path: &str, text: &str, kind: Kind, titles: &[String]) -> String {
    let mut fixed = text.to_owned();
    let mut findings = scan(text, kind, titles);
    findings.retain(|finding| finding.instead.is_some() && allowing(path, text, finding).is_none());
    findings.sort_by_key(|finding| finding.offset);
    // From the end, so the offsets still ahead stay right.
    for finding in findings.iter().rev() {
        if let Some(instead) = &finding.instead {
            fixed.replace_range(
                finding.offset..finding.offset + finding.found.len(),
                instead,
            );
        }
    }
    fixed
}

/// The 1-based line number, the line itself and the byte column of `offset` in `text`.
fn locate(text: &str, offset: usize) -> (usize, &str, usize) {
    let start = text[..offset].rfind('\n').map_or(0, |i| i + 1);
    let end = text[offset..].find('\n').map_or(text.len(), |i| offset + i);
    let number = text[..offset].matches('\n').count() + 1;
    (number, &text[start..end], offset - start)
}

/// How a file's writing is told from its code.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// All of it is writing: plain text, SVG, snapshots.
    Prose,
    /// Writing outside its fenced code blocks, and each block as its language reads: a `rust` or
    /// `ts` block as [`Kind::CLike`], `python`, `sh`, `bash` or `toml` as [`Kind::Hash`], `json`
    /// as [`Kind::Json`], and any other (`text`, a program's output) as [`Kind::Prose`].
    Markdown,
    /// `//` and `/* */` comments and quoted strings: Rust, TypeScript, CSS.
    CLike,
    /// `#` comments and quoted strings: Python, shell, TOML, YAML.
    Hash,
    /// Strings only.
    Json,
}

/// The kind of file `path` is, or `None` for data the check doesn't read (motor files, logs,
/// rasters, other people's formats).
fn kind(path: &str) -> Option<Kind> {
    let extension = path.rsplit_once('.').map(|(_, e)| e)?;
    Some(match extension {
        "md" => Kind::Markdown,
        "txt" | "svg" | "snap" => Kind::Prose,
        "rs" | "ts" | "js" | "css" => Kind::CLike,
        "py" | "sh" | "toml" | "yml" | "yaml" => Kind::Hash,
        "json" => Kind::Json,
        _ => return None,
    })
}

/// A stretch of a file that is writing, at a byte offset, and whether it is a whole string
/// literal (one bare word there is a key, not a word).
struct Segment {
    offset: usize,
    end: usize,
    literal: bool,
}

/// Everything the check reports in `text`, in order.
fn scan(text: &str, kind: Kind, titles: &[String]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for segment in segments(text, kind) {
        let body = &text[segment.offset..segment.end];
        if segment.literal && is_bare_word(body) {
            continue;
        }
        for (start, end) in writing(body, titles) {
            words(&body[start..end], segment.offset + start, &mut findings);
        }
    }
    findings
}

/// A string literal that is one word or key, like `"centre"` or `"cg-centre"`.
fn is_bare_word(body: &str) -> bool {
    !body.is_empty()
        && body
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// The names in `text` that hold a word on the list (M0.6e): in a source file's code (outside its
/// comments and strings) every name; a string literal that is one bare key; and in writing, a word
/// shaped like an identifier ([`misspelling`] leaves those out) and every word of inline code.
/// Addresses (URLs, link targets) stay out, as they do for the writing.
fn names(text: &str, kind: Kind, titles: &[String]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut code_from = 0;
    for segment in segments(text, kind) {
        if segment.offset > code_from {
            named(
                &text[code_from..segment.offset],
                code_from,
                true,
                &mut findings,
            );
        }
        code_from = code_from.max(segment.end);
        let body = &text[segment.offset..segment.end];
        if segment.literal && is_bare_word(body) {
            named(body, segment.offset, true, &mut findings);
            continue;
        }
        let (words, spans) = writing_parts(body, titles);
        for (start, end) in words {
            named(
                &body[start..end],
                segment.offset + start,
                false,
                &mut findings,
            );
        }
        for (start, end) in spans {
            named(
                &body[start..end],
                segment.offset + start,
                true,
                &mut findings,
            );
        }
    }
    if code_from < text.len() {
        named(&text[code_from..], code_from, true, &mut findings);
    }
    findings.sort_by_key(|finding| finding.offset);
    findings
}

/// Pushes each name in `piece` (at `offset` in the file) that holds a word on the list: every
/// name if `every`, else only those shaped like an identifier.
fn named(piece: &str, offset: usize, every: bool, findings: &mut Vec<Finding>) {
    let mut token_start = None;
    for (i, c) in piece.char_indices().chain([(piece.len(), ' ')]) {
        if c.is_alphanumeric() || c == '_' {
            token_start.get_or_insert(i);
            continue;
        }
        let Some(start) = token_start.take() else {
            continue;
        };
        let token = &piece[start..i];
        // A plain word of writing is `words`' to report; a name in a format string, `{centre}`,
        // or a key quoted inside a string, `"{\"colour\": 1}"`, is a name.
        let captured = (piece[..start].ends_with('{') && matches!(c, '}' | ':' | '!'))
            || (piece[..start].ends_with("\\\"") && piece[i..].starts_with("\\\""));
        if !every && !captured && !is_identifier(token) {
            continue;
        }
        if let Some(instead) = renamed(token) {
            findings.push(Finding {
                offset: offset + start,
                found: token.to_owned(),
                instead: Some(instead),
                name: true,
            });
        }
    }
}

/// Whether `token` is shaped like an identifier rather than a word: joined by `_`, holding a
/// digit, or a capital inside a word that has small letters.
fn is_identifier(token: &str) -> bool {
    token.contains('_')
        || token.chars().any(|c| c.is_ascii_digit())
        || (token.chars().skip(1).any(char::is_uppercase) && token.chars().any(char::is_lowercase))
}

/// `name` with each of its words that is misspelled in US form, if any is: the words split at
/// `_`, digits and the capitals of camel case, so `centreOfMass` and `CENTRE_X` both read.
fn renamed(name: &str) -> Option<String> {
    let chars: Vec<(usize, char)> = name.char_indices().collect();
    let mut out = String::with_capacity(name.len());
    let mut changed = false;
    let mut k = 0;
    while k < chars.len() {
        let (at, c) = chars[k];
        if !c.is_alphabetic() {
            out.push(c);
            k += 1;
            continue;
        }
        // A word: a run of letters, ended by a small letter followed by a capital, or by the
        // last of several capitals followed by a small letter (`XMLParser` is `XML`, `Parser`).
        let mut j = k + 1;
        while j < chars.len() && chars[j].1.is_alphabetic() {
            let (previous, next) = (chars[j - 1].1, chars[j].1);
            let camel = previous.is_lowercase() && next.is_uppercase();
            let acronym_end = previous.is_uppercase()
                && next.is_uppercase()
                && chars
                    .get(j + 1)
                    .is_some_and(|(_, after)| after.is_lowercase());
            if camel || acronym_end {
                break;
            }
            j += 1;
        }
        let end = chars.get(j).map_or(name.len(), |(n, _)| *n);
        let word = &name[at..end];
        match misspelling(word) {
            Some(us) => {
                out.push_str(&us);
                changed = true;
            }
            None => out.push_str(word),
        }
        k = j;
    }
    changed.then_some(out)
}

/// The parts of a stretch of writing that are words: everything but inline code (in backticks),
/// link targets, URLs, HTML tags (bar the text of their `alt`, `title` and `aria-label`) and
/// links quoting a decision record's title, as byte ranges.
fn writing(body: &str, titles: &[String]) -> Vec<(usize, usize)> {
    writing_parts(body, titles).0
}

/// A stretch of a file or a piece of one, as byte offsets: where it starts and where it ends.
type Range = (usize, usize);

/// [`writing`]'s ranges, and the insides of the inline code spans it skips, as byte ranges: the
/// spans [`names`] reads, found by the same walk, so a backtick inside an address opens none.
fn writing_parts(body: &str, titles: &[String]) -> (Vec<Range>, Vec<Range>) {
    let bytes = body.as_bytes();
    let mut kept = Vec::new();
    let mut spans = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        let line_end = body[i..].find('\n').map_or(body.len(), |n| i + n);
        // Where a skipped stretch ends, and the ranges inside it that are still words.
        let skip_to: Option<(usize, Vec<(usize, usize)>)> = if bytes[i] == b'`' {
            // A code span closes at the next run of as many backticks, on its line or a later one
            // of its paragraph; a backtick with no partner is just a character.
            let run = bytes[i..].iter().take_while(|&&b| b == b'`').count();
            let fence = &body[i..i + run];
            let after = i + run;
            let paragraph_end = body[after..].find("\n\n").map_or(body.len(), |n| after + n);
            let close = body[after..paragraph_end].find(fence).map(|n| after + n);
            if let Some(close) = close {
                spans.push((after, close));
            }
            Some((close.map_or(after, |close| close + run), Vec::new()))
        } else if body[i..].starts_with("[ADR-") {
            // A link quoting a decision record's title, frozen with the record: its text must be
            // the start of the record's own heading.
            body[i..line_end]
                .find(']')
                .map(|n| i + n)
                .filter(|&end| {
                    let text = &body[i + 1..end];
                    text.contains(": ") && titles.iter().any(|title| title.starts_with(text))
                })
                .map(|end| (end, Vec::new()))
        } else if bytes[i] == b'<'
            && bytes
                .get(i + 1)
                .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'/')
        {
            // An HTML tag: its attributes (an `<a id>` keeping an old anchor) are addresses, but
            // the text a reader is shown stays writing.
            body[i..line_end].find('>').map(|n| {
                let end = i + n + 1;
                let tag = &body[i..end];
                let mut shown = Vec::new();
                for attribute in [" alt=\"", " title=\"", " aria-label=\""] {
                    for (at, _) in tag.match_indices(attribute) {
                        let from = i + at + attribute.len();
                        if let Some(len) = body[from..end].find('"') {
                            shown.push((from, from + len));
                        }
                    }
                }
                shown.sort_unstable();
                (end, shown)
            })
        } else if body[i..].starts_with("](") {
            body[i..line_end].find(')').map(|n| (i + n + 1, Vec::new()))
        } else if body[i..].starts_with("http://") || body[i..].starts_with("https://") {
            Some((
                body[i..]
                    .find(|c: char| c.is_whitespace() || c == ')' || c == '>' || c == '"')
                    .map_or(body.len(), |n| i + n),
                Vec::new(),
            ))
        } else if (i == 0 || bytes[i - 1] == b'\n') && bytes[i] == b'[' {
            // A reference definition, `[label]: target`, names an address; a title after the
            // target is still writing.
            body[i..line_end].find("]: ").map(|n| {
                let target = i + n + 3;
                let end = body[target..line_end]
                    .find(char::is_whitespace)
                    .map_or(line_end, |m| target + m);
                (end, Vec::new())
            })
        } else {
            None
        };
        match skip_to {
            Some((end, shown)) => {
                kept.push((start, i));
                kept.extend(shown);
                i = end.max(i + 1);
                start = i;
            }
            None => i += body[i..].chars().next().map_or(1, char::len_utf8),
        }
    }
    kept.push((start, bytes.len().max(start)));
    kept.retain(|(a, b)| a < b);
    (kept, spans)
}

/// An em dash as HTML writes it, which the site shows as one.
const EM_DASH_ENTITIES: [&str; 3] = ["&mdash;", "&#8212;", "&#x2014;"];

/// Pushes the em dashes and misspelled words in `piece`, which starts at `offset` in the file.
fn words(piece: &str, offset: usize, findings: &mut Vec<Finding>) {
    let mut token_start = None;
    for (i, c) in piece.char_indices().chain([(piece.len(), ' ')]) {
        if c == '—' {
            findings.push(Finding {
                offset: offset + i,
                found: "—".to_owned(),
                instead: None,
                name: false,
            });
        }
        if c.is_alphanumeric() || c == '_' {
            token_start.get_or_insert(i);
            continue;
        }
        if let Some(start) = token_start.take() {
            let token = &piece[start..i];
            // A name in a format string, `{centre}` or `{centre:e}`, is an identifier, and so is a
            // key quoted inside a string, `"{\"colour\": 1}"`.
            let captured = (piece[..start].ends_with('{') && matches!(c, '}' | ':' | '!'))
                || (piece[..start].ends_with("\\\"") && piece[i..].starts_with("\\\""));
            if let Some(instead) = misspelling(token).filter(|_| !captured) {
                findings.push(Finding {
                    offset: offset + start,
                    found: token.to_owned(),
                    instead: Some(instead),
                    name: false,
                });
            }
        }
    }
    for entity in EM_DASH_ENTITIES {
        for (at, _) in piece.match_indices(entity) {
            findings.push(Finding {
                offset: offset + at,
                found: entity.to_owned(),
                instead: None,
                name: false,
            });
        }
    }
}

/// The US form of `token`, if it is a word (not an identifier) holding a misspelling.
fn misspelling(token: &str) -> Option<String> {
    if is_identifier(token) {
        return None;
    }
    let lower = token.to_lowercase();
    if lower.ends_with("analyses") {
        return None;
    }
    let (piece, us) = RULES.iter().find(|(piece, _)| lower.contains(piece))?;
    let at = lower.find(piece)?;
    let replaced = format!("{}{us}{}", &lower[..at], &lower[at + piece.len()..]);
    Some(if token.chars().all(|c| !c.is_lowercase()) {
        replaced.to_uppercase()
    } else if token.chars().next().is_some_and(char::is_uppercase) {
        let mut chars = replaced.chars();
        chars
            .next()
            .map(|first| first.to_uppercase().chain(chars).collect())
            .unwrap_or_default()
    } else {
        replaced
    })
}

/// The stretches of `text` that are writing, by the rules of its kind.
fn segments(text: &str, kind: Kind) -> Vec<Segment> {
    match kind {
        Kind::Prose => vec![Segment {
            offset: 0,
            end: text.len(),
            literal: false,
        }],
        Kind::Markdown => {
            let mut out = outside_fences(text);
            for (start, end, info) in fences(text) {
                let language = info.split([',', ' ']).next().unwrap_or_default();
                let inner = match language {
                    "rust" | "ts" => Kind::CLike,
                    "python" | "sh" | "bash" | "toml" => Kind::Hash,
                    "json" => Kind::Json,
                    _ => Kind::Prose,
                };
                out.extend(
                    segments(&text[start..end], inner)
                        .into_iter()
                        .map(|segment| Segment {
                            offset: start + segment.offset,
                            end: start + segment.end,
                            literal: segment.literal,
                        }),
                );
            }
            out.sort_by_key(|segment| segment.offset);
            out
        }
        Kind::CLike | Kind::Hash | Kind::Json => code_segments(text, kind),
    }
}

/// A Markdown page's fenced code blocks: where each body starts and ends, and the fence's info
/// string (`rust,ignore`), trimmed. An unclosed block runs to the end of the page.
fn fences(text: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    let mut open: Option<(usize, &str)> = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let info = trimmed
            .strip_prefix("```")
            .or_else(|| trimmed.strip_prefix("~~~"));
        if let Some(info) = info {
            match open.take() {
                Some((from, info)) => out.push((from, offset, info)),
                None => open = Some((offset + line.len(), info.trim())),
            }
        }
        offset += line.len();
    }
    if let Some((from, info)) = open {
        out.push((from, text.len(), info));
    }
    out
}

/// The stretches of a Markdown page outside its fenced code blocks (opened and closed by a line
/// starting with three backticks or tildes, indented or not).
fn outside_fences(text: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut start = Some(0);
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            match start.take() {
                Some(from) => out.push(Segment {
                    offset: from,
                    end: offset,
                    literal: false,
                }),
                None => start = Some(offset + line.len()),
            }
        }
        offset += line.len();
    }
    if let Some(from) = start {
        out.push(Segment {
            offset: from,
            end: text.len(),
            literal: false,
        });
    }
    out
}

/// The comments and string literals of a source file. A rough lexer: it knows quotes, escapes,
/// Rust's raw strings and char literals, and Python's triple quotes, which is all the files in
/// scope need.
fn code_segments(text: &str, kind: Kind) -> Vec<Segment> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let line_comment: &[u8] = match kind {
        Kind::CLike => b"//",
        Kind::Hash => b"#",
        Kind::Json | Kind::Prose | Kind::Markdown => b"",
    };
    while i < bytes.len() {
        let rest = &bytes[i..];
        let at_word_start = i == 0 || bytes[i - 1].is_ascii_whitespace();
        if !line_comment.is_empty()
            && rest.starts_with(line_comment)
            && (kind == Kind::CLike || at_word_start || bytes[i - 1] == b'(')
        {
            let end = text[i..].find('\n').map_or(text.len(), |n| i + n);
            out.push(Segment {
                offset: i,
                end,
                literal: false,
            });
            i = end;
        } else if kind == Kind::CLike && rest.starts_with(b"/*") {
            let end = text[i + 2..].find("*/").map_or(text.len(), |n| i + 2 + n);
            out.push(Segment {
                offset: i + 2,
                end,
                literal: false,
            });
            i = end + 2;
        } else if kind == Kind::CLike && raw_string_hashes(text, i).is_some() {
            let (open, hashes) = raw_string_hashes(text, i).unwrap_or_default();
            let close = format!("\"{}", "#".repeat(hashes));
            let start = i + open;
            let end = text[start..].find(&close).map_or(text.len(), |n| start + n);
            out.push(Segment {
                offset: start,
                end,
                literal: true,
            });
            i = end + close.len();
        } else if kind == Kind::Hash && (rest.starts_with(b"\"\"\"") || rest.starts_with(b"'''")) {
            let quote = &text[i..i + 3];
            let start = i + 3;
            let end = text[start..].find(quote).map_or(text.len(), |n| start + n);
            out.push(Segment {
                offset: start,
                end,
                literal: true,
            });
            i = end + 3;
        } else if bytes[i] == b'"' || (bytes[i] == b'\'' && kind != Kind::Json) {
            let quote = bytes[i];
            if quote == b'\'' && kind == Kind::CLike && !is_char_literal(bytes, i) {
                // A Rust lifetime or label.
                i += 1;
                continue;
            }
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != quote {
                // A Hash-kind string doesn't run past its line; an unpaired apostrophe in a
                // shell word must not swallow the file.
                if kind == Kind::Hash && bytes[j] == b'\n' {
                    break;
                }
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = j.min(bytes.len());
            out.push(Segment {
                offset: start,
                end,
                literal: true,
            });
            i = end + 1;
        } else {
            i += 1;
        }
    }
    // A segment never ends inside a character, as every delimiter above is ASCII.
    out
}

/// At a Rust raw string (`r"`, `r#"`, `br"`, ...), the length of its opening and its count of
/// `#`.
fn raw_string_hashes(text: &str, i: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    if i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_') {
        return None;
    }
    let mut j = i;
    if bytes.get(j) == Some(&b'b') {
        j += 1;
    }
    if bytes.get(j) != Some(&b'r') {
        return None;
    }
    j += 1;
    let hashes = bytes[j..].iter().take_while(|&&b| b == b'#').count();
    (bytes.get(j + hashes) == Some(&b'"')).then_some((j + hashes + 1 - i, hashes))
}

/// Whether the `'` at `i` opens a Rust char literal rather than a lifetime or label.
fn is_char_literal(bytes: &[u8], i: usize) -> bool {
    match bytes.get(i + 1) {
        Some(b'\\') => true,
        Some(&first) => {
            // One character, which may be several bytes, then the closing quote.
            let width = match first {
                0x00..=0x7f => 1,
                0xc0..=0xdf => 2,
                0xe0..=0xef => 3,
                _ => 4,
            };
            bytes.get(i + 1 + width) == Some(&b'\'')
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(text: &str, kind: Kind) -> Vec<String> {
        scan(text, kind, &titles())
            .into_iter()
            .map(|f| f.found)
            .collect()
    }

    fn titles() -> Vec<String> {
        vec![
            "ADR-070: M2.2e split: mass and centre of mass first, then the corpus (2026-09-25)"
                .to_owned(),
        ]
    }

    #[test]
    fn each_word_on_the_list_is_found_with_its_us_form() {
        let cases = [
            ("centre", "center"),
            ("Centres", "Centers"),
            ("centred", "centered"),
            ("centring", "centering"),
            ("centreline", "centerline"),
            ("millimetres", "millimeters"),
            ("catalogue", "catalog"),
            ("catalogued", "cataloged"),
            ("licence", "license"),
            ("colours", "colors"),
            ("analyse", "analyze"),
            ("analysed", "analyzed"),
            ("analysing", "analyzing"),
            ("Behaviour", "Behavior"),
            ("modelling", "modeling"),
            ("modelled", "modeled"),
            ("CENTRE", "CENTER"),
        ];
        for (uk, us) in cases {
            assert_eq!(misspelling(uk).as_deref(), Some(us), "{uk}");
        }
        for fine in [
            "center", "analysis", "analyses", "model", "modeling", "meter", "diameter",
        ] {
            assert_eq!(misspelling(fine), None, "{fine}");
        }
    }

    #[test]
    fn identifiers_are_left_alone() {
        for name in [
            "centre_m",
            "cg_centre",
            "centreX",
            "CentreOfPressure",
            "centre2",
        ] {
            assert_eq!(misspelling(name), None, "{name}");
        }
        assert!(found("the `centre` field and ``a centre``", Kind::Prose).is_empty());
        assert_eq!(found("the centre and `centre`", Kind::Prose), ["centre"]);
        assert_eq!(
            found("`a\nb` centre `c\n\nd` metre", Kind::Prose),
            ["centre", "metre"]
        );
    }

    #[test]
    fn fenced_code_blocks_are_read_by_their_language() {
        let page = "A centre.\n\n```text\ncentre of gravity — 0.6\n```\n\nIts colour.\n";
        assert_eq!(
            found(page, Kind::Markdown),
            ["centre", "centre", "—", "colour"]
        );
        let indented = "- item\n  ~~~rust\n  let centre = 1; // centre\n  ~~~\n- a centre\n";
        assert_eq!(found(indented, Kind::Markdown), ["centre", "centre"]);
        assert!(found("format!(\"{centre} and {colour:?}\")", Kind::CLike).is_empty());
        assert!(found("let j = \"{\\\"colour\\\": 1}\";", Kind::CLike).is_empty());
        let options = "```rust,ignore\nlet s = \"a centre\"; colour();\n```\n";
        assert_eq!(found(options, Kind::Markdown), ["centre"]);
        let unclosed = "```json\n{\"colour\": \"the colour\"}\n";
        assert_eq!(found(unclosed, Kind::Markdown), ["colour"]);
    }

    /// An em dash is reported in a page's code blocks and in a source file's comments and strings,
    /// as in a page's prose (M0.6d3), but not in code or as an escape.
    #[test]
    fn em_dashes_are_reported_in_code_blocks_and_sources() {
        let page = "Prose — more.\n\n```text\nA — b\n```\n```rust\nlet a = b — c; // d — e\n```\n";
        assert_eq!(found(page, Kind::Markdown), ["—"; 3]);
        let source = "// a — b\nlet s = \"x — y\";\nlet c = '\\u{2014}';\nlet d = e — f;\n";
        assert_eq!(found(source, Kind::CLike), ["—"; 2]);
        // Each is a problem wherever the file sits, nothing set aside for later.
        let mut report = Report {
            files: 0,
            empty: Vec::new(),
            problems: Vec::new(),
            names: 0,
            allowed: [0; 4],
            stale: Vec::new(),
        };
        let mut used = vec![false; ALLOWED.len()];
        for (path, text, kind) in [
            ("crates/a/src/lib.rs", source, Kind::CLike),
            ("xtask/src/a.rs", source, Kind::CLike),
            ("schema/a.json", "{\"description\": \"a — b\"}", Kind::Json),
            ("docs/a.md", page, Kind::Markdown),
        ] {
            tally(&mut report, &mut used, path, text, kind, &[]);
        }
        assert_eq!(
            report.problems.len(),
            2 + 2 + 1 + 3,
            "{:?}",
            report.problems
        );
    }

    #[test]
    fn addresses_are_left_alone() {
        let quoted = "[ADR-070: M2.2e split: mass and centre of mass first][adr-070]";
        assert!(found(quoted, Kind::Prose).is_empty());
        let paraphrased = "[ADR-070: the centre of it all](x) and [ADR-071: a centre](y)";
        assert_eq!(found(paraphrased, Kind::Prose), ["centre", "centre"]);
        assert_eq!(found("a ](x\n\nThe centre.", Kind::Prose), ["centre"]);
        assert_eq!(
            found("[x]: https://a/centre \"The centre\"", Kind::Prose),
            ["centre"]
        );
        assert_eq!(found("[the rec][r]: the centre", Kind::Prose), ["centre"]);
        assert_eq!(
            found(
                "<img src=\"centre.png\" alt=\"a colour plot\" id=\"centre\">",
                Kind::Prose
            ),
            ["colour"]
        );
        assert_eq!(
            found("a &mdash; b &#8212; c", Kind::Prose),
            ["&mdash;", "&#8212;"]
        );
        assert!(found("<a id=\"centre-of-mass\"></a>", Kind::Prose).is_empty());
        assert_eq!(found("x <centre", Kind::Prose), ["centre"]);
        assert_eq!(found("x < centre", Kind::Prose), ["centre"]);
        assert_eq!(
            found("[ADR-070][adr-070] centres", Kind::Prose),
            ["centres"]
        );
        let text = "See [the record](decisions/adr-070-centre-of-mass.md) or \
                    https://example.org/centre — and\n[adr-070]: https://x/#centre-of-mass\n";
        assert_eq!(found(text, Kind::Prose), ["—"]);
    }

    #[test]
    fn source_files_are_checked_in_comments_and_strings_only() {
        let rust = "/// The centre.\nlet centre = 1; // its centre\nlet s = \"a centre — b\";\n\
                    let key = \"centre\";\nlet c = '—'; fn f<'a>(x: &'a str) {}\n/* centred */\n\
                    let r = r#\"a metre\"#;\n";
        assert_eq!(
            found(rust, Kind::CLike),
            ["centre", "centre", "centre", "—", "—", "centred", "metre"]
        );
        let python =
            "x = 1  # behaviour\ny = \"colour\"\nz = 'the colour'\n\"\"\"Modelled.\"\"\"\n";
        assert_eq!(
            found(python, Kind::Hash),
            ["behaviour", "colour", "Modelled"]
        );
        let shell = "echo ${#list} don't # licence\n";
        assert_eq!(found(shell, Kind::Hash), ["licence"]);
        let json = "{\"centre\": \"the centre\", \"a\": \"centre\"}";
        assert_eq!(found(json, Kind::Json), ["centre"]);
    }

    #[test]
    fn a_word_is_replaced_in_place() {
        let text = "Its centre — and `centre_m`; \"Data licence\", a licence.";
        assert_eq!(
            fixed("a.md", text, Kind::Markdown, &titles()),
            "Its center — and `centre_m`; \"Data license\", a license."
        );
        // An allowed phrase keeps its spelling.
        assert_eq!(
            fixed("docs/motor-stock.md", text, Kind::Markdown, &titles()),
            "Its center — and `centre_m`; \"Data licence\", a license."
        );
    }

    #[test]
    fn every_allowed_phrase_holds_a_word_on_the_list() {
        for (file, phrase, _) in ALLOWED {
            assert!(
                phrase.contains('—')
                    || phrase
                        .split(|c: char| !c.is_alphanumeric())
                        .any(|w| misspelling(w).is_some()),
                "{file}: \"{phrase}\" allows nothing"
            );
        }
    }

    fn named_in(text: &str, kind: Kind) -> Vec<(String, String)> {
        names(text, kind, &titles())
            .into_iter()
            .map(|f| (f.found, f.instead.unwrap_or_default()))
            .collect()
    }

    /// A name is split into words at `_`, digits and camel case's capitals, and each word on the
    /// list takes its US form in the case it was written (M0.6e).
    #[test]
    fn a_name_is_renamed_word_by_word_in_its_case() {
        for (name, us) in [
            ("centre_m", "center_m"),
            ("CentreOfMass", "CenterOfMass"),
            ("centreX", "centerX"),
            ("WP_CENTRE_FRACTION", "WP_CENTER_FRACTION"),
            ("XMLCentre", "XMLCenter"),
            ("metres_per_unit2", "meters_per_unit2"),
            ("Metre", "Meter"),
            ("analyse", "analyze"),
            ("a_catalogue_nose", "a_catalog_nose"),
        ] {
            assert_eq!(renamed(name).as_deref(), Some(us), "{name}");
        }
        for fine in [
            "center_m",
            "run_analyses",
            "AnalysisError",
            "diameter_m",
            "MetreX",
        ] {
            let expected = (fine == "MetreX").then(|| "MeterX".to_owned());
            assert_eq!(renamed(fine), expected, "{fine}");
        }
    }

    /// In code every name is read; in writing only a word shaped like a name, a name in a
    /// format string and a word in inline code, so a plain word is reported once, by `scan`.
    #[test]
    fn names_are_read_in_code_keys_and_inline_code() {
        let source = "let centre = metres_per_unit; // the centre and `centre_m`\n\
                      let key = \"centre_m\"; let s = format!(\"{colour} centre\");\n\
                      // https://example.com/centre_m and run_analyses\n";
        assert_eq!(
            named_in(source, Kind::CLike),
            [
                ("centre".to_owned(), "center".to_owned()),
                ("metres_per_unit".to_owned(), "meters_per_unit".to_owned()),
                ("centre_m".to_owned(), "center_m".to_owned()),
                ("centre_m".to_owned(), "center_m".to_owned()),
                ("colour".to_owned(), "color".to_owned()),
            ]
        );
        // The plain words of the writing are `scan`'s, and it leaves the names alone.
        assert_eq!(found(source, Kind::CLike), ["centre", "centre"]);
        let page = "The `centre` field, a centre_m key, and [a link](centre_m.md).\n\n\
                    ```rust\nlet c = CentreOfMass; // centre\n```\n";
        assert_eq!(
            named_in(page, Kind::Markdown),
            [
                ("centre".to_owned(), "center".to_owned()),
                ("centre_m".to_owned(), "center_m".to_owned()),
                ("CentreOfMass".to_owned(), "CenterOfMass".to_owned()),
            ]
        );
        // A backtick inside an address opens no inline code, so the plain word after it is
        // writing, not a name.
        assert!(named_in("see https://x.org/a`b, a centre` here\n", Kind::Prose).is_empty());
        assert_eq!(
            named_in("{\"centre_east_m\": 1, \"note\": \"a centre\"}", Kind::Json),
            [("centre_east_m".to_owned(), "center_east_m".to_owned())]
        );
    }

    /// A name is a problem with its own message, counted apart; an allowed one is counted under
    /// its reason.
    #[test]
    fn a_name_is_reported_with_its_us_form() {
        let mut report = Report {
            files: 0,
            empty: Vec::new(),
            problems: Vec::new(),
            names: 0,
            allowed: [0; 4],
            stale: Vec::new(),
        };
        let mut used = vec![false; ALLOWED.len()];
        let source = "pub fn sets_centre() {}\n";
        tally(
            &mut report,
            &mut used,
            "crates/a/src/lib.rs",
            source,
            Kind::CLike,
            &titles(),
        );
        assert_eq!(
            report.problems,
            [
                "crates/a/src/lib.rs:1: the name `sets_centre` holds UK spelling; call it \
              `sets_center`"
            ]
        );
        assert_eq!(report.names, 1);
        let alias = "    #[serde(default, alias = \"centre_overridden\")]\n";
        tally(
            &mut report,
            &mut used,
            "crates/hpr-design/src/tree.rs",
            alias,
            Kind::CLike,
            &titles(),
        );
        assert_eq!(
            (report.problems.len(), report.allowed[Reason::Key as usize]),
            (1, 1)
        );
    }

    /// The check itself, over the repository: CI's `site` step runs it too.
    #[test]
    fn the_scope_has_no_findings() {
        let report = check(&crate::designs::root().unwrap()).unwrap();
        assert!(
            report.problems.is_empty() && report.stale.is_empty(),
            "{}\n{}",
            report.problems.join("\n"),
            report.stale.join("\n")
        );
    }
}
