//! The evidence register (M2.10, ADR-278): one row per claim the site makes about what backs a
//! reader, a model or a check, with the kind of evidence behind it and the kind that would settle
//! it, read from `validation/evidence.toml`.
//!
//! `cargo xtask evidence` holds the register to its rules and the guide's pages to the register:
//!
//! - every block of a guide page (a paragraph, a heading, a table row, a list item's own text)
//!   that states a gap in one of the committed [`pages::PHRASES`] links the row that records it, as
//!   `../evidence.md#g1`, every row a page links exists, and a block that states a gap asks for
//!   a sample in the row's own issue, not the bare list of issues;
//! - every row is well formed, its pages' anchors are still there, and the pages that link it
//!   are the pages it lists;
//! - every row short of its wanted kind names an open GitHub issue labeled `needs-sample`, read
//!   with `gh` (`--offline` skips this, saying so);
//! - every `real` row lists its samples, each with its SHA-256 and the test that reads it;
//! - the report on `docs/evidence.md`, between [`REGION_START`] and [`REGION_END`], and the
//!   register's schema, `schema/validation/evidence.schema.json`, are what the register writes
//!   now (`--write` writes them).
//!
//! `--samples`, run by the gate's `refs` step (the samples live under the gitignored `refs/`, so
//! CI checks only the rest), checks each sample's file, its hash, and that its test runs and
//! passes without skipping: a skipped test is never evidence.

mod pages;
mod report;
mod samples;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use report::{REGION_END, REGION_START};

/// The register.
pub(crate) const REGISTER: &str = "validation/evidence.toml";
/// The register's schema, which `--write` generates from the types below.
pub(crate) const SCHEMA: &str = "schema/validation/evidence.schema.json";
/// The page that shows the report, from the repository's root.
pub(crate) const PAGE: &str = "docs/evidence.md";
/// The same page from `docs/`, as the guide's links reach it.
const PAGE_NAME: &str = "evidence.md";
/// The guide's sources, and its table of contents, which lists the pages the check reads.
const DOCS: &str = "docs";
/// The guide's table of contents, under [`DOCS`].
const SUMMARY: &str = "SUMMARY.md";
/// The decision records, one `NNNN-slug.md` file each.
const DECISIONS: &str = "docs/decisions";
/// The repository whose issues a row names.
const REPOSITORY: &str = "nrdptel/fusionspace-eridanus";
/// The label of the issues that ask for better evidence.
const LABEL: &str = "needs-sample";
/// The most issues one `gh issue list` call returns; a list this long may be cut short.
const ISSUE_LIMIT: usize = 1000;

pub const USAGE: &str = "  \
evidence [--check | --write | --samples] [--offline]
                           Hold validation/evidence.toml, the evidence register (ADR-278), to its
                           rules and the guide's pages to it: a page that states a gap links its
                           row, a row short of the evidence it wants names an open issue labeled
                           needs-sample, and the report on docs/evidence.md and the register's
                           schema are current. --write writes the report and the schema first;
                           --offline doesn't read the issues' states with `gh`; --samples checks
                           each real sample under refs/: its file, its hash and its test.";

/// A kind of evidence. The kinds say where the evidence comes from, not how strong it is, so a
/// row gives the kind it has and the kind it wants apart.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Kind {
    /// A file or a measurement from the device or the flight itself.
    Real,
    /// A real file from a related device or firmware.
    Sibling,
    /// Another program's answer on the same input, such as gpxpy's or OpenRocket's.
    CodeToCode,
    /// Built from a published layout, schema or standard.
    Spec,
    /// Checked against the data it was fitted to.
    Calibration,
    /// An argument only.
    Reasoning,
    /// Nothing yet.
    None,
}

impl Kind {
    /// Every kind, in the order the report's columns take.
    pub(crate) const ALL: [Self; 7] = [
        Self::Real,
        Self::Sibling,
        Self::CodeToCode,
        Self::Spec,
        Self::Calibration,
        Self::Reasoning,
        Self::None,
    ];

    /// The kind as the register writes it.
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Real => "real",
            Self::Sibling => "sibling",
            Self::CodeToCode => "code-to-code",
            Self::Spec => "spec",
            Self::Calibration => "calibration",
            Self::Reasoning => "reasoning",
            Self::None => "none",
        }
    }

    /// The kind in words, as the report writes it.
    pub(crate) fn words(self) -> &'static str {
        match self {
            Self::CodeToCode => "code to code",
            other => other.key(),
        }
    }
}

/// One search for better evidence: the day, where it looked and what it found, so the next
/// search doesn't repeat it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Search {
    /// The day of the search, `YYYY-MM-DD`.
    #[schemars(regex(pattern = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"))]
    pub on: String,
    /// Where it looked, one place each: a site, an archive, a search or a collection, as a short
    /// phrase. The report lists each place as its own item.
    #[schemars(length(min = 1))]
    pub looked: Vec<String>,
    /// What it found, nothing included, as short sentences. The report writes it after "It
    /// found", so its first sentence goes on from those words, in lower case.
    pub found: String,
}

/// A real sample behind a `real` row: a file under the gitignored `refs/`, its hash, its license,
/// and the test that reads it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Sample {
    /// The file, from the repository's root: `refs/...`.
    #[schemars(regex(pattern = r"^refs/"))]
    pub path: String,
    /// The file's SHA-256, 64 lowercase hex digits.
    #[schemars(regex(pattern = r"^[0-9a-f]{64}$"))]
    pub sha256: String,
    /// The file's license, and where it came from.
    pub license: String,
    /// The cargo package whose test reads the file, such as `fusionspace-hpr-flightdata`.
    pub package: String,
    /// The test's target: `lib` for the package's library, or the name of an integration test,
    /// the `name` of `tests/<name>.rs`.
    pub target: String,
    /// The test's full path, as `cargo test -- --exact` takes it, such as
    /// `gpx::tests::reads_a_real_track`.
    pub test: String,
}

/// One claim the site makes about what backs a reader, a model or a check.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Row {
    /// `G` and a number, `G1`, `G2` and so on, increasing down the file. A page names the row by
    /// linking the report's anchor for it, `evidence.md#g1`.
    #[schemars(regex(pattern = r"^G[1-9][0-9]*$"))]
    pub id: String,
    /// The reader, model or output the claim is about; several rows may share one.
    pub surface: String,
    /// The claim, one sentence with no full stop at its end.
    pub claim: String,
    /// Each page that states the claim, from `docs/`, with the anchor of its section:
    /// `format/flight-logs.md#gpx`. Every page that links the row is listed, and every page
    /// listed links it.
    #[schemars(length(min = 1))]
    pub pages: Vec<String>,
    /// The kind of evidence behind the claim now.
    pub kind: Kind,
    /// The kind that would settle the claim: `real`, unless a decision record sets another.
    pub wanted: Kind,
    /// The decision record that sets a wanted kind other than `real`, as `ADR-145`; only then.
    #[serde(default)]
    #[schemars(regex(pattern = r"^ADR-[0-9]{3,4}$"))]
    pub wanted_adr: Option<String>,
    /// What the evidence doesn't cover; needed unless the row has its wanted kind.
    #[serde(default)]
    pub not_checked: Option<String>,
    /// The number of the open issue, labeled `needs-sample`, that asks for the wanted evidence;
    /// needed unless the row has its wanted kind.
    #[serde(default)]
    pub issue: Option<u64>,
    /// The searches for better evidence, oldest first.
    #[serde(default)]
    pub searched: Vec<Search>,
    /// The samples behind a `real` row, one or more; a row of any other kind has none.
    #[serde(default)]
    pub sample: Vec<Sample>,
}

/// The evidence register: one row per claim the site makes about what backs a reader, a model
/// or a check, with the kind of evidence behind it and the kind that would settle it
/// (ADR-278).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Register {
    /// The day the report counts searches from, `YYYY-MM-DD`: a row whose last search is more
    /// than 30 days before it is due another.
    #[schemars(regex(pattern = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"))]
    pub as_of: String,
    /// The rows, in the order of their ids.
    #[serde(default)]
    pub row: Vec<Row>,
}

impl Register {
    /// Reads the register's text; the error names the file and what TOML or the types refuse.
    pub(crate) fn parse(text: &str) -> Result<Self, String> {
        toml::from_str(text).map_err(|error| format!("{REGISTER}: {error}"))
    }
}

/// Whether an issue labeled `needs-sample` is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IssueState {
    /// Still open: the evidence it asks for hasn't been found.
    Open,
    /// Closed.
    Closed,
}

/// The issues labeled `needs-sample`, by number. An issue not here has no such label, or
/// doesn't exist.
pub(crate) type IssueStates = BTreeMap<u64, IssueState>;

/// What the check reads, gathered from the repository by [`Gathered::read`] or written by a
/// test.
#[derive(Debug)]
pub(crate) struct Inputs<'a> {
    /// The register's text.
    pub register: &'a str,
    /// The guide's pages the check reads, by their path from `docs/`, each with its text.
    pub pages: &'a BTreeMap<String, String>,
    /// The decision records' file names, by their number.
    pub decisions: &'a BTreeMap<u32, String>,
    /// The report's page, if it is there.
    pub page: Option<&'a str>,
    /// The committed schema, if it is there.
    pub schema: Option<&'a str>,
    /// The issues' states; `None` when they weren't read (`--offline`).
    pub issues: Option<&'a IssueStates>,
}

/// What the check found: its problems, one line each, and what it left unchecked.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Outcome {
    /// Each problem, one line naming the file and the line or the row.
    pub problems: Vec<String>,
    /// What the check didn't check, and why.
    pub notes: Vec<String>,
}

/// Runs `cargo xtask evidence`.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut flags = BTreeSet::new();
    for arg in args {
        match arg.as_str() {
            "--check" | "--write" | "--samples" | "--offline" => {
                flags.insert(arg.as_str());
            }
            _ => return Err(format!("unknown arguments {args:?}\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    if flags.contains("--samples") {
        if flags.contains("--check") || flags.contains("--write") {
            return Err(format!("--samples runs on its own\n\n{USAGE}"));
        }
        return samples::run(&root);
    }
    if flags.contains("--write") {
        write(&root)?;
    }
    let issues = if flags.contains("--offline") {
        None
    } else {
        Some(fetch_issue_states()?)
    };
    let gathered = Gathered::read(&root)?;
    let outcome = check(&gathered.inputs(issues.as_ref()));
    for note in &outcome.notes {
        println!("evidence: {note}");
    }
    if outcome.problems.is_empty() {
        let rows = Register::parse(&gathered.register).map_or(0, |register| register.row.len());
        println!(
            "evidence: the register's {rows} rows, the {} pages that state their claims, the \
             report and the schema agree",
            gathered.pages.len()
        );
        Ok(())
    } else {
        Err(format!(
            "evidence: {} problems:\n{}",
            outcome.problems.len(),
            outcome.problems.join("\n")
        ))
    }
}

/// The register's schema, from its types: pretty JSON with a trailing newline.
#[expect(
    clippy::expect_used,
    reason = "a `Schema` is a JSON value with string keys, which always serializes"
)]
pub(crate) fn schema() -> String {
    let schema = schemars::schema_for!(Register);
    let text = serde_json::to_string_pretty(&schema).expect("a JSON value serializes");
    format!("{text}\n")
}

/// Every problem the check finds in `inputs`, each its own line naming the file and the line or
/// the row, and what it left unchecked.
pub(crate) fn check(inputs: &Inputs) -> Outcome {
    let mut outcome = Outcome::default();
    let schema_problem = schema_problem(inputs.schema);
    let register = match Register::parse(inputs.register) {
        Ok(register) => register,
        Err(error) => {
            outcome.problems.push(error);
            outcome.problems.extend(schema_problem);
            return outcome;
        }
    };
    let structure = structure(&register, inputs.decisions);
    let structure_clean = structure.is_empty();
    outcome.problems.extend(structure);

    // Rules 1, 2 and 10: each block that states a gap names a row and no bare list of issues,
    // and each row named exists.
    let ids: BTreeSet<&str> = register.row.iter().map(|row| row.id.as_str()).collect();
    // The pages that name each row, with the first line that does.
    let mut named: BTreeMap<String, BTreeMap<&str, usize>> = BTreeMap::new();
    for (page, text) in inputs.pages {
        let reading = read_page(page, text);
        outcome.problems.extend(reading.problems);
        for (id, line) in reading.links {
            if !ids.contains(id.as_str()) {
                outcome.problems.push(format!(
                    "{DOCS}/{page}:{line}: links row {id}, which {REGISTER} doesn't have"
                ));
            }
            (named.entry(id).or_default())
                .entry(page.as_str())
                .or_insert(line);
        }
    }

    // Rules 4 and 5: each row's pages and anchors are there, and they are the pages that link it.
    for row in &register.row {
        outcome
            .problems
            .extend(page_problems(row, inputs.pages, named.get(&row.id)));
    }

    // Rule 6: each row short of its wanted kind names an open `needs-sample` issue.
    match inputs.issues {
        Some(issues) => outcome.problems.extend(issue_problems(&register, issues)),
        None => outcome.notes.push(format!(
            "the issues' states were not checked (--offline): run without it to check that each \
             row short of its wanted kind names an open issue labeled `{LABEL}`"
        )),
    }

    // Rule 8: the report is current. A register that breaks its rules writes no report to
    // compare, and its problems come first.
    if structure_clean {
        outcome
            .problems
            .extend(report_problem(&register, inputs.decisions, inputs.page));
    }
    // Rule 9: the schema is current.
    outcome.problems.extend(schema_problem);
    outcome
}

/// What one guide page says to the check: its problems (rules 1 and 10, one message each), and
/// each row its blocks link, with the line of the block.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct PageReading {
    /// One message per block that states a gap and links no row (rule 1), and per block that
    /// states one and asks for a sample in the bare list of issues (rule 10).
    pub problems: Vec<String>,
    /// Each row a block links, as its id and the block's line.
    pub links: Vec<(String, usize)>,
}

/// Reads `text`, the page `page` (its path from `docs/`), for [`PageReading`].
pub(crate) fn read_page(page: &str, text: &str) -> PageReading {
    let mut reading = PageReading::default();
    for block in pages::blocks(text) {
        let rows: Vec<String> = (block.links.iter())
            .filter_map(|dest| pages::row_named(page, dest))
            .collect();
        if let Some(phrase) = pages::gap(&block.text) {
            if rows.is_empty() {
                reading.problems.push(format!(
                    "{DOCS}/{page}:{}: states a gap (\"{phrase}\") but links no row of \
                     {REGISTER} (link {}{PAGE_NAME}#g<N>)",
                    block.line,
                    pages::up_to_docs(page)
                ));
            }
            if block.links.iter().any(|dest| pages::is_issue_list(dest)) {
                reading.problems.push(format!(
                    "{DOCS}/{page}:{}: asks for a sample in the issue list; link the row's issue \
                     instead",
                    block.line
                ));
            }
        }
        reading
            .links
            .extend(rows.into_iter().map(|id| (id, block.line)));
    }
    reading
}

/// What breaks the register's own rules: its ids, its fields, each row's decision record, its
/// samples and its searches. Each message names the register and the row.
pub(crate) fn structure(register: &Register, decisions: &BTreeMap<u32, String>) -> Vec<String> {
    let mut problems = Vec::new();
    let as_of = day_number(&register.as_of);
    if as_of.is_none() {
        problems.push(format!(
            "{REGISTER}: as_of `{}` isn't a date, YYYY-MM-DD",
            register.as_of
        ));
    }
    let mut seen = BTreeSet::new();
    let mut last: Option<(u64, &str)> = None;
    for row in &register.row {
        let at = |what: String| format!("{REGISTER}: {}: {what}", row.id);
        match row_number(&row.id) {
            None => problems.push(at(format!("`{}` isn't `G` and a number, as `G1`", row.id))),
            Some(number) => {
                if !seen.insert(number) {
                    problems.push(at("listed twice".to_owned()));
                } else if let Some((before, id)) = last
                    && number < before
                {
                    problems.push(at(format!(
                        "comes after {id}: the rows go in increasing order of their ids"
                    )));
                }
                last = Some((number, &row.id));
            }
        }
        for (field, text) in [("surface", &row.surface), ("claim", &row.claim)] {
            if text.trim().is_empty() {
                problems.push(at(format!("has no {field}")));
            }
        }
        let mut texts = vec![("surface", row.surface.as_str()), ("claim", &row.claim)];
        texts.extend(row.not_checked.as_deref().map(|text| ("not_checked", text)));
        for search in &row.searched {
            texts.extend(search.looked.iter().map(|place| ("looked", place.as_str())));
            texts.push(("found", &search.found));
        }
        for (field, text) in texts {
            if text.contains(['\n', '\r']) {
                problems.push(at(format!(
                    "its {field} holds a line break: the register keeps each text on one line"
                )));
            }
        }
        if row.pages.is_empty() {
            problems.push(at("lists no page that states its claim".to_owned()));
        }
        problems.extend(adr_problems(row, decisions).into_iter().map(&at));
        let short = row.kind != row.wanted;
        if short
            && row
                .not_checked
                .as_deref()
                .is_none_or(|text| text.trim().is_empty())
        {
            problems.push(at(format!(
                "has {} evidence and wants {}, but says nothing in not_checked: what the \
                 evidence doesn't cover",
                row.kind.key(),
                row.wanted.key()
            )));
        }
        if short && row.issue.is_none() {
            problems.push(at(format!(
                "has {} evidence and wants {}, but names no issue: file one labeled `{LABEL}` and \
                 set issue",
                row.kind.key(),
                row.wanted.key()
            )));
        }
        match (row.kind, row.sample.is_empty()) {
            (Kind::Real, true) => problems.push(at(
                "its kind is real but it lists no sample with its SHA-256 and the test that \
                 reads it"
                    .to_owned(),
            )),
            (Kind::Real, false) | (_, true) => {}
            (kind, false) => problems.push(at(format!(
                "lists samples, but its kind is {}: only a real row has samples",
                kind.key()
            ))),
        }
        for sample in &row.sample {
            problems.extend(
                sample_rules(sample)
                    .into_iter()
                    .map(|what| at(format!("sample {}: {what}", sample.path))),
            );
        }
        let mut previous: Option<i64> = None;
        for search in &row.searched {
            let Some(day) = day_number(&search.on) else {
                problems.push(at(format!(
                    "its search on `{}` isn't on a date, YYYY-MM-DD",
                    search.on
                )));
                continue;
            };
            if previous.is_some_and(|previous| day <= previous) {
                problems.push(at(format!(
                    "its search on {} isn't after the one before it: searches go oldest first",
                    search.on
                )));
            }
            if as_of.is_some_and(|as_of| day > as_of) {
                problems.push(at(format!(
                    "its search on {} is after as_of, {}",
                    search.on, register.as_of
                )));
            }
            // An empty list, or an empty place in it, is as bare as an empty `found`.
            let no_place = search.looked.is_empty()
                || (search.looked.iter()).any(|place| place.trim().is_empty());
            if no_place || search.found.trim().is_empty() {
                problems.push(at(format!(
                    "its search on {} needs both where it looked and what it found",
                    search.on
                )));
            }
            for place in &search.looked {
                if let Some(what) = opens_as_structure(place) {
                    problems.push(at(format!(
                        "its search on {} has a place that opens as {what} in the report's list: \
                         `{}`",
                        search.on,
                        place.trim()
                    )));
                }
            }
            previous = Some(day);
        }
    }
    problems
}

/// What a place would open as at the start of its item in the report's list, where Markdown reads
/// it as structure rather than text: a list marker, a heading, a quote, a code fence, a thematic
/// break or an HTML tag. `None` for a place that reads as text, such as "#645's thread" or "1.5 m
/// tubes".
fn opens_as_structure(place: &str) -> Option<&'static str> {
    let text = place.trim_start();
    // Whether the marker of `len` bytes at the start ends the text or is followed by a space.
    let marker = |len: usize| text[len..].chars().next().is_none_or(char::is_whitespace);
    let marks: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let hashes = text.chars().take_while(|&c| c == '#').count();
    let digits = text.chars().take_while(char::is_ascii_digit).count();
    if text.starts_with("```") || text.starts_with("~~~") {
        Some("a code fence")
    } else if marks.len() >= 3
        && ["-", "*", "_"]
            .iter()
            .any(|c| marks.replace(c, "").is_empty())
    {
        Some("a thematic break")
    } else if text.starts_with(['-', '+', '*']) && marker(1) {
        Some("a list marker")
    } else if (1..=6).contains(&hashes) && marker(hashes) {
        Some("a heading")
    } else if text.starts_with('>') {
        Some("a quote")
    } else if text.starts_with('<') {
        Some("an HTML tag")
    } else if (1..=9).contains(&digits)
        && text[digits..].starts_with(['.', ')'])
        && marker(digits + 1)
    {
        Some("a numbered list marker")
    } else {
        None
    }
}

/// What breaks the rule for a row's decision record: one exactly when it wants less than `real`,
/// and a record that exists.
fn adr_problems(row: &Row, decisions: &BTreeMap<u32, String>) -> Vec<String> {
    match (&row.wanted_adr, row.wanted) {
        (None, Kind::Real) => Vec::new(),
        (None, wanted) => vec![format!(
            "wants {} evidence, not real, so it needs wanted_adr: the decision record that sets \
             it",
            wanted.key()
        )],
        (Some(adr), Kind::Real) => vec![format!(
            "wants real evidence, which needs no wanted_adr, but names {adr}"
        )],
        (Some(adr), _) => match adr_number(adr) {
            None => vec![format!("wanted_adr `{adr}` isn't `ADR-` and 3 or 4 digits")],
            Some(number) if !decisions.contains_key(&number) => {
                vec![format!("wanted_adr {adr} names no record in {DECISIONS}/")]
            }
            Some(_) => Vec::new(),
        },
    }
}

/// What breaks the rules for one sample: its path under `refs/`, its hash, and the fields its
/// test needs.
fn sample_rules(sample: &Sample) -> Vec<String> {
    let mut problems = Vec::new();
    let leaves = sample.path.split(['/', '\\']).any(|part| part == "..");
    if !sample.path.starts_with("refs/") || leaves {
        problems.push("its path isn't under refs/".to_owned());
    }
    let hex = |b: u8| b.is_ascii_digit() || (b'a'..=b'f').contains(&b);
    if sample.sha256.len() != 64 || !sample.sha256.bytes().all(hex) {
        problems.push(format!(
            "its sha256 `{}` isn't 64 lowercase hex digits",
            sample.sha256
        ));
    }
    for (field, text) in [
        ("license", &sample.license),
        ("package", &sample.package),
        ("target", &sample.target),
        ("test", &sample.test),
    ] {
        if text.trim().is_empty() {
            problems.push(format!("has no {field}"));
        }
    }
    // The three go to cargo and the test harness as arguments: a name, never an option.
    let name = |b: u8| b.is_ascii_alphanumeric() || b"_:.-".contains(&b);
    for (field, text) in [
        ("package", &sample.package),
        ("target", &sample.target),
        ("test", &sample.test),
    ] {
        if !text.is_empty() && (text.starts_with('-') || !text.bytes().all(name)) {
            problems.push(format!(
                "its {field} `{text}` isn't a name: letters, digits and `_:.-`, not starting `-`"
            ));
        }
    }
    problems
}

/// What breaks rules 4 and 5 for one row, given the pages the check reads and the pages that
/// name the row (with the first line that does).
fn page_problems(
    row: &Row,
    pages: &BTreeMap<String, String>,
    named: Option<&BTreeMap<&str, usize>>,
) -> Vec<String> {
    let at = |what: String| format!("{REGISTER}: {}: {what}", row.id);
    let mut problems = Vec::new();
    let mut listed = BTreeSet::new();
    for entry in &row.pages {
        let Some((page, anchor)) = entry.split_once('#').filter(|(_, a)| !a.is_empty()) else {
            problems.push(at(format!(
                "its page `{entry}` needs the anchor of its section: page#anchor"
            )));
            continue;
        };
        listed.insert(page);
        let Some(text) = pages.get(page) else {
            problems.push(at(format!(
                "its page {entry}: {DOCS}/{page} isn't a page the check reads ({DOCS}/{SUMMARY} \
                 lists them, less the excluded ones)"
            )));
            continue;
        };
        if !crate::site::anchors_of(text).contains(anchor) {
            problems.push(at(format!(
                "its page {entry}: {DOCS}/{page} has no anchor #{anchor}"
            )));
        }
        if named.is_some_and(|named| !named.contains_key(page)) {
            problems.push(at(format!(
                "its page {DOCS}/{page} never links it: link {}{PAGE_NAME}#{} where it states \
                 the claim",
                pages::up_to_docs(page),
                row.id.to_ascii_lowercase()
            )));
        }
    }
    match named {
        None if !row.pages.is_empty() => problems.push(at(format!(
            "no page links it: link {PAGE_NAME}#{} where each of its pages states the claim",
            row.id.to_ascii_lowercase()
        ))),
        None => {}
        Some(named) => {
            for (page, line) in named {
                if !listed.contains(page) {
                    problems.push(at(format!(
                        "{DOCS}/{page}:{line} links it, but its pages don't list {page}"
                    )));
                }
            }
        }
    }
    problems
}

/// What breaks rule 6: a row short of its wanted kind whose issue isn't open, or isn't labeled
/// `needs-sample`.
pub(crate) fn issue_problems(register: &Register, issues: &IssueStates) -> Vec<String> {
    let mut problems = Vec::new();
    for row in register.row.iter().filter(|row| row.kind != row.wanted) {
        // A missing issue is a problem of the register's structure.
        let Some(issue) = row.issue else { continue };
        match issues.get(&issue) {
            Some(IssueState::Open) => {}
            Some(IssueState::Closed) => problems.push(format!(
                "{REGISTER}: {}: its issue #{issue} is closed, but the row has {} evidence and \
                 wants {}: reopen the issue, or record the evidence that settles the row",
                row.id,
                row.kind.key(),
                row.wanted.key()
            )),
            None => problems.push(format!(
                "{REGISTER}: {}: its issue #{issue} isn't among the issues labeled `{LABEL}`: \
                 label it, or name the issue that asks for this evidence",
                row.id
            )),
        }
    }
    problems
}

/// Rule 8: the report region on the page, against what the register writes now.
fn report_problem(
    register: &Register,
    decisions: &BTreeMap<u32, String>,
    page: Option<&str>,
) -> Option<String> {
    let Some(page) = page else {
        return Some(format!(
            "{PAGE} is missing; it holds the register's report between `{REGION_START}` and \
             `{REGION_END}`"
        ));
    };
    let committed = match report::committed_region(page) {
        Ok(committed) => committed,
        Err(error) => return Some(error),
    };
    match report::region(register, decisions) {
        Err(error) => Some(format!("{REGISTER}: {error}")),
        Ok(region) if region != committed => Some(format!(
            "{PAGE}'s report is not what {REGISTER} writes now; run `cargo xtask evidence --write`"
        )),
        Ok(_) => None,
    }
}

/// Rule 9: the committed schema, against what the register's types write now.
fn schema_problem(committed: Option<&str>) -> Option<String> {
    match committed {
        None => Some(format!(
            "{SCHEMA} is missing; run `cargo xtask evidence --write`"
        )),
        Some(text) if text.replace("\r\n", "\n") != schema() => Some(format!(
            "{SCHEMA} is not what the register's types write now; run `cargo xtask evidence \
             --write`"
        )),
        Some(_) => None,
    }
}

/// The number of a row's id, `G12` → 12; `None` unless it is `G` and a number with no leading
/// zero.
pub(crate) fn row_number(id: &str) -> Option<u64> {
    let digits = id.strip_prefix('G')?;
    let well_formed = !digits.is_empty()
        && !digits.starts_with('0')
        && digits.bytes().all(|b| b.is_ascii_digit());
    well_formed.then(|| digits.parse().ok()).flatten()
}

/// The number of a decision record's label, `ADR-145` → 145; `None` unless it is `ADR-` and 3 or
/// 4 digits.
fn adr_number(label: &str) -> Option<u32> {
    let digits = label.strip_prefix("ADR-")?;
    let well_formed = (3..=4).contains(&digits.len()) && digits.bytes().all(|b| b.is_ascii_digit());
    well_formed.then(|| digits.parse().ok()).flatten()
}

/// `YYYY-MM-DD` as year, month and day, if it is a day of the calendar (February 29 only in a
/// leap year).
fn calendar_date(date: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = date.split('-').collect();
    let [year, month, day] = parts[..] else {
        return None;
    };
    let number = |part: &str, len: usize| {
        (part.len() == len && part.bytes().all(|b| b.is_ascii_digit()))
            .then(|| part.parse::<i64>().ok())
            .flatten()
    };
    let (year, month, day) = (number(year, 4)?, number(month, 2)?, number(day, 2)?);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    (1..=days).contains(&day).then_some((year, month, day))
}

/// The day `date` (`YYYY-MM-DD`) falls on, counted from 1970-01-01, if it is a day of the
/// calendar. The proleptic Gregorian count of Howard Hinnant's `days_from_civil`
/// ("chrono-Compatible Low-Level Date Algorithms", 2013): the year starts in March, so
/// February's length doesn't move the days before it.
pub(crate) fn day_number(date: &str) -> Option<i64> {
    let (year, month, day) = calendar_date(date)?;
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era - 719_468)
}

/// `YYYY-MM-DD` as running prose writes a date: October 17, 2026.
pub(crate) fn prose_date(date: &str) -> Result<String, String> {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let (year, month, day) = calendar_date(date).ok_or_else(|| format!("`{date}` isn't a date"))?;
    let name = (usize::try_from(month - 1).ok())
        .and_then(|at| MONTHS.get(at))
        .ok_or_else(|| format!("`{date}` isn't a date"))?;
    Ok(format!("{name} {day}, {year}"))
}

/// What the check reads from the repository, owned.
#[derive(Debug)]
pub(crate) struct Gathered {
    /// The register's text.
    pub register: String,
    /// The pages the check reads, by their path from `docs/`.
    pub pages: BTreeMap<String, String>,
    /// The decision records' file names, by number.
    pub decisions: BTreeMap<u32, String>,
    /// The report's page, if it is there.
    pub page: Option<String>,
    /// The committed schema, if it is there.
    pub schema: Option<String>,
}

impl Gathered {
    /// Reads the register, the pages `docs/SUMMARY.md` lists (less [`pages::EXCLUDED`]), the decision
    /// records' names, the report's page and the schema under `root`.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        let register = read(root, REGISTER)?;
        let summary = read(root, &format!("{DOCS}/{SUMMARY}"))?;
        let mut pages = BTreeMap::new();
        for name in pages::scanned(&summary) {
            let text = read(root, &format!("{DOCS}/{name}")).map_err(|error| {
                format!("{DOCS}/{SUMMARY} lists {name}, which can't be read: {error}")
            })?;
            pages.insert(name, text);
        }
        Ok(Self {
            register,
            pages,
            decisions: decisions(root)?,
            page: read_if_there(root, PAGE)?,
            schema: read_if_there(root, SCHEMA)?,
        })
    }

    /// The check's inputs, borrowing these and the issues' states.
    pub(crate) fn inputs<'a>(&'a self, issues: Option<&'a IssueStates>) -> Inputs<'a> {
        Inputs {
            register: &self.register,
            pages: &self.pages,
            decisions: &self.decisions,
            page: self.page.as_deref(),
            schema: self.schema.as_deref(),
            issues,
        }
    }
}

/// The text of the file at `path` under `root`, with Windows line ends made Unix ones.
fn read(root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(path))
        .map(|text| text.replace("\r\n", "\n"))
        .map_err(|error| format!("reading {path}: {error}"))
}

/// As [`read`], but `None` for a file that isn't there.
fn read_if_there(root: &Path, path: &str) -> Result<Option<String>, String> {
    match fs::read_to_string(root.join(path)) {
        Ok(text) => Ok(Some(text.replace("\r\n", "\n"))),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("reading {path}: {error}")),
    }
}

/// The decision records' file names under `root`, by number: `0278-....md` is 278's.
fn decisions(root: &Path) -> Result<BTreeMap<u32, String>, String> {
    let entries =
        fs::read_dir(root.join(DECISIONS)).map_err(|error| format!("{DECISIONS}: {error}"))?;
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .collect();
    names.sort();
    let mut found = BTreeMap::new();
    for name in names {
        let number = (name.get(..4))
            .filter(|digits| digits.bytes().all(|b| b.is_ascii_digit()))
            .filter(|_| name.get(4..5) == Some("-") && name.ends_with(".md"))
            .and_then(|digits| digits.parse().ok());
        if let Some(number) = number {
            found.entry(number).or_insert(name);
        }
    }
    Ok(found)
}

/// Writes the schema and the report, each only if it is stale.
fn write(root: &Path) -> Result<(), String> {
    let gathered = Gathered::read(root)?;
    let register = Register::parse(&gathered.register)?;
    let problems = structure(&register, &gathered.decisions);
    if !problems.is_empty() {
        return Err(format!(
            "{REGISTER} breaks its rules, so no report is written:\n{}",
            problems.join("\n")
        ));
    }
    let schema = schema();
    if gathered.schema.as_deref() != Some(schema.as_str()) {
        let path = root.join(SCHEMA);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).map_err(|error| format!("{SCHEMA}: {error}"))?;
        }
        fs::write(&path, schema).map_err(|error| format!("{SCHEMA}: {error}"))?;
    }
    let page = gathered.page.ok_or_else(|| {
        format!(
            "{PAGE} is missing; write it with `{REGION_START}` and `{REGION_END}` on lines of \
             their own, and the report goes between them"
        )
    })?;
    let region = report::region(&register, &gathered.decisions)?;
    let old = report::committed_region(&page)?;
    if old != region {
        fs::write(root.join(PAGE), page.replacen(old, &region, 1))
            .map_err(|error| format!("{PAGE}: {error}"))?;
    }
    Ok(())
}

/// The issues labeled `needs-sample`, open and closed, read once with `gh`. The error says what
/// failed, and that `--offline` checks the rest without them.
fn fetch_issue_states() -> Result<IssueStates, String> {
    let limit = ISSUE_LIMIT.to_string();
    let offline = "pass --offline to check the rest without the issues' states";
    let output = Command::new("gh")
        .args(["issue", "list", "--repo", REPOSITORY, "--label", LABEL])
        .args([
            "--state",
            "all",
            "--limit",
            &limit,
            "--json",
            "number,state",
        ])
        .output()
        .map_err(|error| format!("running `gh issue list`: {error}; {offline}"))?;
    if !output.status.success() {
        return Err(format!(
            "`gh issue list` failed: {}; {offline}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    parse_issue_states(&String::from_utf8_lossy(&output.stdout))
}

/// One issue as `gh issue list --json number,state` writes it.
#[derive(Debug, Deserialize)]
struct ListedIssue {
    /// The issue's number.
    number: u64,
    /// `OPEN` or `CLOSED`.
    state: String,
}

/// The issues' states from `gh issue list --json number,state`'s output, refusing a state that
/// is neither `OPEN` nor `CLOSED`, and a list as long as the limit, which may have been cut.
pub(crate) fn parse_issue_states(json: &str) -> Result<IssueStates, String> {
    let listed: Vec<ListedIssue> = serde_json::from_str(json)
        .map_err(|error| format!("reading `gh issue list`'s output: {error}"))?;
    if listed.len() >= ISSUE_LIMIT {
        return Err(format!(
            "`gh issue list` returned {} issues labeled `{LABEL}`, its limit, so some may be \
             missing; raise ISSUE_LIMIT",
            listed.len()
        ));
    }
    let mut states = IssueStates::new();
    for issue in listed {
        let state = match issue.state.as_str() {
            "OPEN" => IssueState::Open,
            "CLOSED" => IssueState::Closed,
            other => {
                return Err(format!(
                    "`gh issue list` gives issue #{} the state `{other}`, neither OPEN nor CLOSED",
                    issue.number
                ));
            }
        };
        states.insert(issue.number, state);
    }
    Ok(states)
}
