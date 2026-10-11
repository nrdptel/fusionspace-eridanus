//! `cargo xtask scoreboard [--check]`: HPR Sim against OpenRocket and RocketPy, axis by axis, read
//! from the committed reports, and a ratchet that keeps each of HPR Sim's numbers from getting worse
//! (M0.10, ADR-209 §3, ADR-240).
//!
//! The scoreboard is `validation/reports/scoreboard.json`, with its page `scoreboard.md`. Each row
//! is one measure on one of ADR-209 §3's axes (accuracy, bias, honest uncertainty, speed, round
//! trips, features, steps to a first answer); each cell is one tool's value, read from a committed
//! report and kept at the precision the page prints, or "not measured" with the open milestone
//! that measures it.
//!
//! - No flag builds the scoreboard from the committed reports and writes it, unless one of HPR
//!   Sim's cells is worse than the committed scoreboard's. A missing scoreboard is refused, since
//!   it is the mark; `--first` writes one where none was ever committed.
//! - `--check` writes nothing, and fails on the same worsening or when the committed files are not
//!   what the reports write. `--against <revision>` also holds the reports to the scoreboard
//!   committed there: CI and the gate give `origin/main`, so a pull request that regenerates the
//!   scoreboard with a worse number still fails against main's.
//!
//! A cell is worse when its row's [`Worse`] rule says so at the printed precision, when it counts
//! fewer flights or designs, or when a measured cell is no longer measured; a worsening passes only when an accepted ADR names the
//! cell and the next release in a `- **Scoreboard:**` line (ADR-240). Speed is reported, never
//! gated (ADR-209 §3), and the other tools' numbers are shown, not held. The `xtask` tests hold
//! the committed scoreboard to the committed reports, and each unmeasured cell's milestone to an
//! open entry in `docs/ROADMAP.md`, so closing that milestone without wiring its cell fails.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fixture_flights::{ERA5_CREDIT, FLIGHT_DATA_CREDIT};

pub const USAGE: &str = "  scoreboard [--first | --check [--against <revision>]]
                           HPR Sim against OpenRocket and RocketPy on each axis of ADR-209 §3,
                           read from the committed reports. Writes
                           validation/reports/scoreboard.{json,md} unless one of HPR Sim's
                           numbers got worse than the committed scoreboard without an accepted
                           ADR naming it and the next release; --first writes one where none was
                           committed. --check writes nothing and fails on that or a stale file;
                           --against holds the reports to the scoreboard at that revision too.";

/// The committed scoreboard.
pub(crate) const SCOREBOARD_JSON: &str = "validation/reports/scoreboard.json";
/// Its page.
pub(crate) const SCOREBOARD_MD: &str = "validation/reports/scoreboard.md";

/// The public real flights (M2.3b).
const REAL_FLIGHTS: &str = "validation/reports/real-flights.json";
/// The private collection's flights, as aggregates (M2.3c1).
const FIXTURE_FLIGHTS: &str = "validation/reports/fixture-flights.json";
/// Its page, which names the OpenRocket release its record came from.
const FIXTURE_FLIGHTS_MD: &str = "validation/reports/fixture-flights.md";
/// OpenRocket flying HPR Sim's `.ork` exports (M3.2b).
const EXPORT_FLIGHTS: &str = "validation/reports/openrocket-export-flights.json";
/// HPR Sim against OpenRocket's flights of its own examples, which names its release.
const OPENROCKET_FLIGHTS: &str = "validation/reports/openrocket-flights.json";
/// The harness's report, whose sources name the RocketPy release.
const HARNESS: &str = "validation/reports/latest.json";
/// The workspace manifest, whose version names the release in progress.
const MANIFEST: &str = "Cargo.toml";
/// The ADRs, where a worsening is accepted.
const DECISIONS: &str = "docs/decisions";
/// The open milestones.
const ROADMAP: &str = "docs/ROADMAP.md";

/// The private collection's set the scoreboard reads: every flight both tools flew and compared.
const EVERY_FLIGHT: &str = "every flight compared";

/// A tool with a column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Tool {
    Hpr,
    Openrocket,
    Rocketpy,
}

impl Tool {
    /// The tool's part of a cell id.
    fn slug(self) -> &'static str {
        match self {
            Self::Hpr => "hpr",
            Self::Openrocket => "openrocket",
            Self::Rocketpy => "rocketpy",
        }
    }
}

/// One of ADR-209 §3's axes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Axis {
    Accuracy,
    Bias,
    Uncertainty,
    Speed,
    Interop,
    Features,
    Steps,
}

impl Axis {
    /// The page's heading for the axis.
    fn heading(self) -> &'static str {
        match self {
            Self::Accuracy => "Accuracy",
            Self::Bias => "Bias",
            Self::Uncertainty => "Honest uncertainty",
            Self::Speed => "Speed",
            Self::Interop => "Round trips",
            Self::Features => "Features",
            Self::Steps => "Steps to a first answer",
        }
    }
}

/// When a row's value is worse than before, as ADR-209 §3 defines it per axis.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Worse {
    /// A higher value: an error, a count of lossy round trips, steps.
    Higher,
    /// A lower value: features.
    Lower,
    /// A value farther from zero: a bias.
    FartherFromZero,
    /// A value farther from the level it states: a range's coverage, per cent.
    FartherFrom { level: f64 },
    /// Never: speed, reported from one machine and reviewed, not gated.
    NotGated,
}

impl Worse {
    /// Whether `new` is worse than `old`; both are already at the printed precision.
    fn worse(self, old: f64, new: f64) -> bool {
        match self {
            Self::Higher => new > old,
            Self::Lower => new < old,
            Self::FartherFromZero => new.abs() > old.abs(),
            Self::FartherFrom { level } => (new - level).abs() > (old - level).abs(),
            Self::NotGated => false,
        }
    }

    /// Which way is better, in words, for the page's column.
    fn words(self) -> String {
        match self {
            Self::Higher => "lower".to_owned(),
            Self::Lower => "higher".to_owned(),
            Self::FartherFromZero => "nearer zero".to_owned(),
            Self::FartherFrom { level } => format!("nearer {level}%"),
            Self::NotGated => "lower, not held".to_owned(),
        }
    }
}

/// One tool's cell: a value read from a report, or the milestone that will measure it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Cell {
    pub tool: Tool,
    /// The value at the row's printed precision, or `None` when not measured.
    pub value: Option<f64>,
    /// How many flights, configurations or designs the value counts over.
    pub count: Option<u64>,
    /// The report and the key the value was read from.
    pub source: Option<String>,
    /// The open milestone that measures an unmeasured cell.
    pub milestone: Option<String>,
}

/// One measure on one axis, with a cell per tool.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Row {
    /// Stable id; a cell's id is the row's and the tool's, `accuracy.public.hpr`.
    pub id: String,
    pub axis: Axis,
    pub label: String,
    /// What the value is in: `%`, or empty for a count.
    pub unit: String,
    /// Printed digits after the point.
    pub decimals: u32,
    /// How a count reads after the value: `{n}` stands for it.
    pub count_words: String,
    /// Whether a value prints its sign.
    pub signed: bool,
    pub worse: Worse,
    pub cells: Vec<Cell>,
}

impl Row {
    /// Whether the ratchet holds `tool`'s value in this row: HPR Sim's numbers only, which are
    /// its own column's and every round trip's (a round trip through OpenRocket's file is HPR
    /// Sim's reading and writing), and never speed's.
    fn holds(&self, tool: Tool) -> bool {
        self.worse != Worse::NotGated && (tool == Tool::Hpr || self.axis == Axis::Interop)
    }
}

/// The scoreboard.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Scoreboard {
    pub generated_by: String,
    /// Each tool's name with its release, in column order.
    pub tools: BTreeMap<Tool, String>,
    pub rows: Vec<Row>,
}

/// A cell worse than the committed scoreboard's.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Worsening {
    pub cell: String,
    pub was: String,
    pub now: String,
}

/// Runs the command.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    let mut first = false;
    let mut against = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--first" => first = true,
            "--against" => {
                against = Some(
                    args.next()
                        .ok_or_else(|| format!("`--against` needs a git revision\n\n{USAGE}"))?
                        .clone(),
                );
            }
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    if check && first {
        return Err(format!(
            "`--check` and `--first` exclude each other\n\n{USAGE}"
        ));
    }
    if against.is_some() && !check {
        return Err(format!("`--against` goes with `--check`\n\n{USAGE}"));
    }
    let root = crate::designs::root()?;
    if check {
        let lines = check_committed(&root, against.as_deref())?;
        for line in lines {
            println!("{line}");
        }
        println!("scoreboard: the committed scoreboard holds and is current");
        return Ok(());
    }
    let scoreboard = build(&root)?;
    let release = next_release(&root)?;
    let accepted = accepted_worsenings(&root)?;
    let lines = match mark(read_committed(&root)?, first)? {
        Some(committed) => gate(&committed, &scoreboard, &release, &accepted)?,
        None => Vec::new(),
    };
    for line in lines {
        println!("{line}");
    }
    let json = serde_json::to_string_pretty(&scoreboard).map_err(|error| error.to_string())? + "\n";
    let page = render(&scoreboard);
    for (path, text) in [(SCOREBOARD_JSON, json), (SCOREBOARD_MD, page)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("scoreboard: wrote {SCOREBOARD_JSON} and {SCOREBOARD_MD}");
    Ok(())
}

/// The scoreboard the writer holds a new one to: the committed one, which must be there unless
/// `first` says none ever was. Without its mark the writer couldn't tell a worse scoreboard from a
/// first one, so a deleted file would let any number through.
fn mark(committed: Option<Scoreboard>, first: bool) -> Result<Option<Scoreboard>, String> {
    match (committed, first) {
        (Some(committed), false) => Ok(Some(committed)),
        (None, true) => Ok(None),
        (Some(_), true) => Err(format!("`--first`: {SCOREBOARD_JSON} is already there")),
        (None, false) => Err(format!(
            "{SCOREBOARD_JSON} is missing: restore it (`git checkout -- {SCOREBOARD_JSON}`), since \
             the scoreboard is held to it; `--first` writes one where none was ever committed"
        )),
    }
}

/// Fails unless the committed reports are no worse than the committed scoreboard, nor than the one
/// at `against` when given (CI gives `origin/main`), or an accepted ADR names each worsening and
/// the next release; the committed files are what they write; and each unmeasured cell names an
/// open milestone. Returns the accepted worsenings' lines.
pub(crate) fn check_committed(root: &Path, against: Option<&str>) -> Result<Vec<String>, String> {
    let scoreboard = build(root)?;
    let committed = read_committed(root)?
        .ok_or_else(|| format!("{SCOREBOARD_JSON} is missing: restore it from git"))?;
    let release = next_release(root)?;
    let accepted = accepted_worsenings(root)?;
    let mut lines = gate(&committed, &scoreboard, &release, &accepted)?;
    if let Some(revision) = against
        && let Some(base) = scoreboard_at(root, revision)?
    {
        lines.extend(
            gate(&base, &scoreboard, &release, &accepted)
                .map_err(|error| format!("against {revision}: {error}"))?,
        );
    }
    if committed != scoreboard {
        return Err(format!(
            "{SCOREBOARD_JSON} is not what the committed reports give: run `cargo xtask scoreboard`"
        ));
    }
    let page = fs::read_to_string(root.join(SCOREBOARD_MD))
        .map_err(|error| format!("reading {SCOREBOARD_MD}: {error}"))?;
    if page.replace("\r\n", "\n") != render(&scoreboard) {
        return Err(format!(
            "{SCOREBOARD_MD} is not what {SCOREBOARD_JSON} writes: run `cargo xtask scoreboard`"
        ));
    }
    let roadmap = fs::read_to_string(root.join(ROADMAP))
        .map_err(|error| format!("reading {ROADMAP}: {error}"))?;
    check_milestones(&scoreboard, &roadmap)?;
    Ok(lines)
}

/// The committed scoreboard, or `None` before the first is written.
fn read_committed(root: &Path) -> Result<Option<Scoreboard>, String> {
    let path = root.join(SCOREBOARD_JSON);
    if !path.exists() {
        return Ok(None);
    }
    let text =
        fs::read_to_string(&path).map_err(|error| format!("reading {SCOREBOARD_JSON}: {error}"))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|error| format!("reading {SCOREBOARD_JSON}: {error}"))
}

/// The scoreboard committed at a git revision, or `None` when that revision has none (before
/// the first was committed). A revision git doesn't know fails, so a check told to compare never
/// passes for want of the other side.
fn scoreboard_at(root: &Path, revision: &str) -> Result<Option<Scoreboard>, String> {
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .map_err(|error| format!("could not run git: {error}"))
    };
    if !git(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("{revision}^{{commit}}"),
    ])?
    .status
    .success()
    {
        return Err(format!(
            "git knows no revision `{revision}` (fetch it first)"
        ));
    }
    let blob = format!("{revision}:{SCOREBOARD_JSON}");
    if !git(&["cat-file", "-e", &blob])?.status.success() {
        return Ok(None);
    }
    let shown = git(&["show", &blob])?;
    if !shown.status.success() {
        return Err(format!(
            "git show {blob}: {}",
            String::from_utf8_lossy(&shown.stderr).trim()
        ));
    }
    serde_json::from_slice(&shown.stdout)
        .map(Some)
        .map_err(|error| format!("reading {blob}: {error}"))
}

/// Holds `new` to `old`: every cell `old` measured that its row holds ([`Row::holds`]) must be no
/// worse in `new` and count no fewer flights or designs, and every measured cell of a row that
/// isn't speed must still be measured, unless `accepted` holds the cell with `release`. Returns a
/// line for each accepted worsening.
pub(crate) fn gate(
    old: &Scoreboard,
    new: &Scoreboard,
    release: &str,
    accepted: &BTreeSet<(String, String)>,
) -> Result<Vec<String>, String> {
    let mut refused = Vec::new();
    let mut lines = Vec::new();
    for worsening in worsenings(old, new) {
        if accepted.contains(&(worsening.cell.clone(), release.to_owned())) {
            lines.push(format!(
                "scoreboard: {} went from {} to {}, accepted for release {release}",
                worsening.cell, worsening.was, worsening.now
            ));
        } else {
            refused.push(format!(
                "  {}: {} → {}",
                worsening.cell, worsening.was, worsening.now
            ));
        }
    }
    if refused.is_empty() {
        Ok(lines)
    } else {
        Err(format!(
            "the scoreboard got worse than the committed one, and no accepted ADR names these \
             cells with release {release} in a `- **Scoreboard:**` line (ADR-240):\n{}",
            refused.join("\n")
        ))
    }
}

/// Each cell `new` holds worse than `old` (ADR-209 §3): a measured value its row holds that is
/// worse at the printed precision or counts fewer flights or designs (a number made better by
/// leaving data out), or a measured cell, in any row but speed's, that is gone or no longer
/// measured.
pub(crate) fn worsenings(old: &Scoreboard, new: &Scoreboard) -> Vec<Worsening> {
    let mut found = Vec::new();
    for row in &old.rows {
        if row.worse == Worse::NotGated {
            continue;
        }
        let new_row = new.rows.iter().find(|candidate| candidate.id == row.id);
        for cell in &row.cells {
            let Some(was) = cell.value else { continue };
            let id = format!("{}.{}", row.id, cell.tool.slug());
            let now = new_row
                .and_then(|new_row| new_row.cells.iter().find(|c| c.tool == cell.tool))
                .and_then(|c| c.value.map(|value| (value, c.count)));
            match now {
                None => found.push(Worsening {
                    cell: id,
                    was: value_text(row, was),
                    now: "not measured".to_owned(),
                }),
                Some((now, _)) if row.holds(cell.tool) && row.worse.worse(was, now) => {
                    found.push(Worsening {
                        cell: id,
                        was: value_text(row, was),
                        now: value_text(row, now),
                    });
                }
                Some((now, Some(after)))
                    if row.holds(cell.tool) && cell.count.is_some_and(|before| after < before) =>
                {
                    let counted = |value, count| match count_text(row, count) {
                        Some(words) => format!("{} {words}", value_text(row, value)),
                        None => format!("{} over {count}", value_text(row, value)),
                    };
                    found.push(Worsening {
                        cell: id,
                        was: counted(was, cell.count.unwrap_or(after)),
                        now: counted(now, after),
                    });
                }
                Some(_) => {}
            }
        }
    }
    found
}

/// The release the work in progress ships in: the workspace version's next minor release, since
/// the version names the last one published (`0.1.0` until 0.2 is out).
fn next_release(root: &Path) -> Result<String, String> {
    let text = fs::read_to_string(root.join(MANIFEST))
        .map_err(|error| format!("reading {MANIFEST}: {error}"))?;
    let manifest: toml::Table =
        toml::from_str(&text).map_err(|error| format!("reading {MANIFEST}: {error}"))?;
    let version = manifest
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("version"))
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("{MANIFEST} has no workspace.package.version"))?;
    release_after(version).ok_or_else(|| format!("{MANIFEST}'s version `{version}` isn't x.y.z"))
}

/// `0.1.0` → `0.2`.
fn release_after(version: &str) -> Option<String> {
    let mut parts = version.split('.');
    let major: u64 = parts.next()?.parse().ok()?;
    let minor: u64 = parts.next()?.parse().ok()?;
    Some(format!("{major}.{}", minor.checked_add(1)?))
}

/// Every (cell, release) an accepted ADR names in a `- **Scoreboard:**` line.
fn accepted_worsenings(root: &Path) -> Result<BTreeSet<(String, String)>, String> {
    let dir = root.join(DECISIONS);
    let mut accepted = BTreeSet::new();
    let entries = fs::read_dir(&dir).map_err(|error| format!("reading {DECISIONS}: {error}"))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("reading {}: {error}", path.display()))?;
        accepted.extend(acceptances(&text));
    }
    Ok(accepted)
}

/// The (cell, release) pairs one ADR accepts: none unless its status line says accepted; else, on
/// each line that starts `- **Scoreboard:**`, every backticked cell id with the line's
/// `release <x.y>`.
pub(crate) fn acceptances(adr: &str) -> Vec<(String, String)> {
    let accepted = adr
        .lines()
        .find_map(|line| line.strip_prefix("- **Status:** "))
        .is_some_and(|status| status.starts_with("accepted"));
    if !accepted {
        return Vec::new();
    }
    let mut pairs = Vec::new();
    for line in adr.lines() {
        let Some(rest) = line.strip_prefix("- **Scoreboard:**") else {
            continue;
        };
        let Some(release) = rest.split_once("release ").map(|(_, after)| {
            after
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect::<String>()
                .trim_end_matches('.')
                .to_owned()
        }) else {
            continue;
        };
        if release.is_empty() {
            continue;
        }
        for (index, piece) in rest.split('`').enumerate() {
            if index % 2 == 1 && !piece.is_empty() {
                pairs.push((piece.to_owned(), release.clone()));
            }
        }
    }
    pairs
}

/// Fails unless each unmeasured cell names a milestone with an open entry in the roadmap, so a
/// milestone can't close while the cell it measures still says "not measured".
pub(crate) fn check_milestones(scoreboard: &Scoreboard, roadmap: &str) -> Result<(), String> {
    let open: BTreeSet<&str> = roadmap
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("- [ ] "))
        .filter_map(|rest| {
            let rest = rest.strip_prefix("[blocked] ").unwrap_or(rest);
            rest.strip_prefix("**")?.split_whitespace().next()
        })
        .collect();
    let mut problems = Vec::new();
    for row in &scoreboard.rows {
        for cell in &row.cells {
            let id = format!("{}.{}", row.id, cell.tool.slug());
            match (&cell.value, &cell.milestone) {
                (Some(_), None) => {}
                (None, Some(milestone)) if milestone_words(milestone).is_none() => problems.push(
                    format!("{id}: its milestone {milestone} has no words in `milestone_words`"),
                ),
                (None, Some(milestone)) if open.contains(milestone.as_str()) => {}
                (None, Some(milestone)) => problems.push(format!(
                    "{id}: not measured, and its milestone {milestone} has no open entry in \
                     {ROADMAP}: read the cell from the report it wrote, or name the milestone \
                     that will"
                )),
                (None, None) => problems.push(format!("{id}: not measured and names no milestone")),
                (Some(_), Some(_)) => {
                    problems.push(format!("{id}: measured, yet names a milestone"));
                }
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

/// A JSON report under the root.
fn report(root: &Path, path: &str) -> Result<Value, String> {
    let text =
        fs::read_to_string(root.join(path)).map_err(|error| format!("reading {path}: {error}"))?;
    serde_json::from_str(&text).map_err(|error| format!("reading {path}: {error}"))
}

/// The number at `pointer` in `report`.
fn number(report: &Value, path: &str, pointer: &str) -> Result<f64, String> {
    report
        .pointer(pointer)
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("{path} has no finite number at {pointer}"))
}

/// The whole number at `pointer` in `report`.
fn whole(report: &Value, path: &str, pointer: &str) -> Result<u64, String> {
    report
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{path} has no whole number at {pointer}"))
}

/// The string at `pointer` in `report`.
fn text<'a>(report: &'a Value, path: &str, pointer: &str) -> Result<&'a str, String> {
    report
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{path} has no string at {pointer}"))
}

/// `value` as the page prints it, to `decimals` places: formatted and read back, so the stored
/// value is the printed one even where `(x * 100).round() / 100` would differ (2.675 prints 2.67).
fn rounded(value: f64, decimals: u32) -> f64 {
    let decimals = usize::try_from(decimals).unwrap_or(usize::MAX);
    format!("{value:.decimals$}").parse().unwrap_or(value)
}

/// The reports' data the scoreboard reads.
pub(crate) struct Reports {
    pub real_flights: Value,
    pub fixture_flights: Value,
    pub fixture_flights_page: String,
    pub export_flights: Value,
    pub openrocket_flights: Value,
    pub harness: Value,
}

impl Reports {
    /// Reads the committed reports.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        Ok(Self {
            real_flights: report(root, REAL_FLIGHTS)?,
            fixture_flights: report(root, FIXTURE_FLIGHTS)?,
            fixture_flights_page: fs::read_to_string(root.join(FIXTURE_FLIGHTS_MD))
                .map_err(|error| format!("reading {FIXTURE_FLIGHTS_MD}: {error}"))?,
            export_flights: report(root, EXPORT_FLIGHTS)?,
            openrocket_flights: report(root, OPENROCKET_FLIGHTS)?,
            harness: report(root, HARNESS)?,
        })
    }
}

/// The scoreboard of the committed reports under `root`.
fn build(root: &Path) -> Result<Scoreboard, String> {
    scoreboard(&Reports::read(root)?)
}

/// A count as an `f64`: exact, since a report's counts are far below 2^53.
#[allow(
    clippy::cast_precision_loss,
    reason = "a report's counts are far below 2^53, so exact as an f64"
)]
fn exact(count: u64) -> f64 {
    count as f64
}

/// A measured cell.
fn measured(tool: Tool, value: f64, decimals: u32, count: u64, source: String) -> Cell {
    Cell {
        tool,
        value: Some(rounded(value, decimals)),
        count: Some(count),
        source: Some(source),
        milestone: None,
    }
}

/// An unmeasured cell and the milestone that measures it.
fn later(tool: Tool, milestone: &str) -> Cell {
    Cell {
        tool,
        value: None,
        count: None,
        source: None,
        milestone: Some(milestone.to_owned()),
    }
}

/// Each tool's release, read from the reports, with the OpenRocket release checked to be the
/// same in every report an OpenRocket cell reads.
fn tools(reports: &Reports) -> Result<BTreeMap<Tool, String>, String> {
    let openrocket = text(
        &reports.openrocket_flights,
        OPENROCKET_FLIGHTS,
        "/reference/version",
    )?;
    let export = text(&reports.export_flights, EXPORT_FLIGHTS, "/openrocket")?;
    if export != openrocket {
        return Err(format!(
            "{EXPORT_FLIGHTS} flew OpenRocket {export}, {OPENROCKET_FLIGHTS} {openrocket}"
        ));
    }
    let fixture = reports
        .fixture_flights_page
        .lines()
        .find_map(|line| line.strip_prefix("| flown by OpenRocket "))
        .and_then(|rest| rest.split_whitespace().next())
        .ok_or_else(|| format!("{FIXTURE_FLIGHTS_MD} names no OpenRocket release"))?;
    if fixture != openrocket {
        return Err(format!(
            "{FIXTURE_FLIGHTS_MD} flew OpenRocket {fixture}, {OPENROCKET_FLIGHTS} {openrocket}"
        ));
    }
    let sources = reports
        .harness
        .pointer("/sources")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{HARNESS} has no sources"))?;
    let rocketpy: BTreeSet<&str> = sources
        .iter()
        .filter_map(|source| source.get("oracle").and_then(Value::as_str))
        .filter_map(|oracle| oracle.strip_prefix("rocketpy "))
        .filter_map(|rest| rest.split_whitespace().next())
        .collect();
    let [rocketpy] = rocketpy.into_iter().collect::<Vec<_>>()[..] else {
        return Err(format!("{HARNESS}'s sources name no one RocketPy release"));
    };
    Ok(BTreeMap::from([
        (Tool::Hpr, "HPR Sim".to_owned()),
        (Tool::Openrocket, format!("OpenRocket {openrocket}")),
        (Tool::Rocketpy, format!("RocketPy {rocketpy}")),
    ]))
}

/// The private collection's set of every compared flight, and its flight count, checked to be
/// each tool's count.
fn every_flight(reports: &Reports) -> Result<(&Value, String, u64), String> {
    let sets = reports
        .fixture_flights
        .pointer("/sets")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{FIXTURE_FLIGHTS} has no sets"))?;
    let index = sets
        .iter()
        .position(|set| set.get("name").and_then(Value::as_str) == Some(EVERY_FLIGHT))
        .ok_or_else(|| format!("{FIXTURE_FLIGHTS} has no set named \"{EVERY_FLIGHT}\""))?;
    let set = &sets[index];
    let pointer = format!("/sets/{index}");
    let flights = whole(set, FIXTURE_FLIGHTS, "/flights")?;
    for tool in ["hpr", "openrocket"] {
        let n = whole(set, FIXTURE_FLIGHTS, &format!("/{tool}/n"))?;
        if n != flights {
            return Err(format!(
                "{FIXTURE_FLIGHTS}{pointer}: {tool} counts {n} flights of the set's {flights}"
            ));
        }
    }
    Ok((set, pointer, flights))
}

/// The scoreboard of `reports`.
pub(crate) fn scoreboard(reports: &Reports) -> Result<Scoreboard, String> {
    use Tool::{Hpr, Openrocket, Rocketpy};
    let tools = tools(reports)?;
    let public = &reports.real_flights;
    let public_flights = whole(public, REAL_FLIGHTS, "/summary/flights")?;
    let (set, set_pointer, collection_flights) = every_flight(reports)?;
    let percent = |axis, id: &str, label: &str, signed, worse, cells| Row {
        id: id.to_owned(),
        axis,
        label: label.to_owned(),
        unit: "%".to_owned(),
        decimals: 2,
        count_words: "over {n} flights".to_owned(),
        signed,
        worse,
        cells,
    };
    let collection = |field: &str, tool: Tool, key: &str| -> Result<Cell, String> {
        let pointer = format!("/{key}/{field}");
        Ok(measured(
            tool,
            number(set, FIXTURE_FLIGHTS, &pointer)?,
            2,
            collection_flights,
            format!("{FIXTURE_FLIGHTS} {set_pointer}{pointer}"),
        ))
    };
    let public_cell = |pointer: &str| -> Result<Cell, String> {
        Ok(measured(
            Hpr,
            number(public, REAL_FLIGHTS, pointer)?,
            2,
            public_flights,
            format!("{REAL_FLIGHTS} {pointer}"),
        ))
    };

    let total = |key: &str| {
        whole(
            &reports.export_flights,
            EXPORT_FLIGHTS,
            &format!("/total/{key}"),
        )
    };
    let compared = total("compared")?;
    let within = total("within")?;
    let unmatched = total("unmatched")?;
    let designs = total("designs")?;
    let unread = total("unread")?;
    let refused_differently = total("refused_differently")?;
    let originals_opened = total("originals_opened")?;
    let exports_opened = total("exports_opened")?;
    let only_exports_opened = total("only_exports_opened")?;
    let changed = compared
        .checked_sub(within)
        .and_then(|outside| outside.checked_add(unmatched))
        .ok_or_else(|| format!("{EXPORT_FLIGHTS}: more within tolerance than compared"))?;
    // Designs that don't make the trip: HPR Sim can't read them, or OpenRocket opens one of the
    // original and the export but not the other, or refuses the two for different reasons.
    // `exports_opened` counts only designs whose original opened too.
    let failed = originals_opened
        .checked_sub(exports_opened)
        .and_then(|lost| lost.checked_add(only_exports_opened))
        .and_then(|lost| lost.checked_add(unread))
        .and_then(|lost| lost.checked_add(refused_differently))
        .ok_or_else(|| format!("{EXPORT_FLIGHTS}: its design counts don't add up"))?;
    let largest = number(
        &reports.export_flights,
        EXPORT_FLIGHTS,
        "/total/largest_percent",
    )?;
    let tolerance = number(
        &reports.export_flights,
        EXPORT_FLIGHTS,
        "/tolerance_percent",
    )?;
    let counted =
        |id: &str, label: String, words: &str, value: u64, count: u64, source: &str| Row {
            id: id.to_owned(),
            axis: Axis::Interop,
            label,
            unit: String::new(),
            decimals: 0,
            count_words: words.to_owned(),
            signed: false,
            worse: Worse::Higher,
            cells: vec![
                measured(
                    Openrocket,
                    exact(value),
                    0,
                    count,
                    format!("{EXPORT_FLIGHTS} {source}"),
                ),
                later(Rocketpy, "M2.9"),
            ],
        };

    let rows = vec![
        percent(
            Axis::Accuracy,
            "accuracy.public",
            "Mean absolute apogee error, public logged flights",
            false,
            Worse::Higher,
            vec![
                public_cell("/summary/mean_absolute_apogee_error_percent")?,
                later(Openrocket, "M2.6"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Accuracy,
            "accuracy.collection",
            "Mean absolute apogee error, the private collection's logged flights",
            false,
            Worse::Higher,
            vec![
                collection("mean_absolute_percent", Hpr, "hpr")?,
                collection("mean_absolute_percent", Openrocket, "openrocket")?,
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Accuracy,
            "accuracy.held-out",
            "Mean absolute apogee error, held-out flights",
            false,
            Worse::Higher,
            vec![
                later(Hpr, "M2.7"),
                later(Openrocket, "M2.7"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Accuracy,
            "accuracy.supersonic",
            "Mean absolute apogee error, published flights past Mach 1",
            false,
            Worse::Higher,
            vec![
                later(Hpr, "M2.3c7"),
                later(Openrocket, "M2.9"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Bias,
            "bias.public",
            "Mean apogee error, public logged flights",
            true,
            Worse::FartherFromZero,
            vec![
                public_cell("/summary/mean_apogee_error_percent")?,
                later(Openrocket, "M2.6"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Bias,
            "bias.collection",
            "Mean apogee error, the private collection's logged flights",
            true,
            Worse::FartherFromZero,
            vec![
                collection("mean_percent", Hpr, "hpr")?,
                collection("mean_percent", Openrocket, "openrocket")?,
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Bias,
            "bias.held-out",
            "Mean apogee error, held-out flights",
            true,
            Worse::FartherFromZero,
            vec![
                later(Hpr, "M2.7"),
                later(Openrocket, "M2.7"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Uncertainty,
            "uncertainty.range-50",
            "Logged apogees inside the 50% range",
            false,
            Worse::FartherFrom { level: 50.0 },
            vec![
                later(Hpr, "M2.8"),
                later(Openrocket, "M2.9"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        percent(
            Axis::Uncertainty,
            "uncertainty.range-90",
            "Logged apogees inside the 90% range",
            false,
            Worse::FartherFrom { level: 90.0 },
            vec![
                later(Hpr, "M2.8"),
                later(Openrocket, "M2.9"),
                later(Rocketpy, "M2.9"),
            ],
        ),
        Row {
            id: "speed.flight".to_owned(),
            axis: Axis::Speed,
            label: "One flight, pad to landing, on one reference machine".to_owned(),
            unit: " ms".to_owned(),
            decimals: 1,
            count_words: String::new(),
            signed: false,
            worse: Worse::NotGated,
            cells: vec![
                later(Hpr, "M2.9"),
                later(Openrocket, "M2.9"),
                later(Rocketpy, "M2.9"),
            ],
        },
        counted(
            "interop.changed",
            format!(
                "Configurations whose apogee moves by more than {tolerance}%, or that fly only one \
                 way, after HPR Sim reads the tool's file and writes it back"
            ),
            "of {n}",
            changed,
            compared + unmatched,
            "/total: compared − within + unmatched",
        ),
        counted(
            "interop.failed",
            "Designs that don't make the trip: HPR Sim can't read them, or the tool opens one of \
             the original and the export but not the other, or refuses the two differently"
                .to_owned(),
            "of {n}",
            failed,
            designs,
            "/total: originals_opened − exports_opened + only_exports_opened + unread + \
             refused_differently",
        ),
        Row {
            id: "interop.largest".to_owned(),
            axis: Axis::Interop,
            label: "Largest apogee change through a round trip".to_owned(),
            unit: "%".to_owned(),
            decimals: 6,
            count_words: "over {n} configurations".to_owned(),
            signed: false,
            worse: Worse::Higher,
            cells: vec![
                measured(
                    Openrocket,
                    largest,
                    6,
                    compared,
                    format!("{EXPORT_FLIGHTS} /total/largest_percent"),
                ),
                later(Rocketpy, "M2.9"),
            ],
        },
        Row {
            id: "features.count".to_owned(),
            axis: Axis::Features,
            label: "Capabilities of the simulator guide's comparison table that the tool has"
                .to_owned(),
            unit: String::new(),
            decimals: 0,
            count_words: "of {n}".to_owned(),
            signed: false,
            worse: Worse::Lower,
            cells: vec![
                later(Hpr, "M0.8a"),
                later(Openrocket, "M0.8a"),
                later(Rocketpy, "M0.8a"),
            ],
        },
        Row {
            id: "steps.first-answer".to_owned(),
            axis: Axis::Steps,
            label: "Steps to open a design, fly it and get its landing spread".to_owned(),
            unit: String::new(),
            decimals: 0,
            count_words: String::new(),
            signed: false,
            worse: Worse::Higher,
            cells: vec![
                later(Hpr, "M0.11"),
                later(Openrocket, "M0.11"),
                later(Rocketpy, "M0.11"),
            ],
        },
    ];
    Ok(Scoreboard {
        generated_by: "cargo xtask scoreboard".to_owned(),
        tools,
        rows,
    })
}

/// A value as the page prints it: its decimals, its sign where the row has one (a minus sign,
/// never a hyphen), and its unit.
fn value_text(row: &Row, value: f64) -> String {
    let decimals = usize::try_from(row.decimals).unwrap_or(usize::MAX);
    let digits = format!("{:.decimals$}", value.abs());
    let zero = !digits.chars().any(|c| c.is_ascii_digit() && c != '0');
    let sign = if zero {
        ""
    } else if value < 0.0 {
        "\u{2212}"
    } else if row.signed {
        "+"
    } else {
        ""
    };
    format!("{sign}{digits}{}", row.unit)
}

/// A count as the row words it after a value (`over 53 flights`), or `None` for a row that
/// prints none.
fn count_text(row: &Row, count: u64) -> Option<String> {
    (!row.count_words.is_empty()).then(|| row.count_words.replace("{n}", &count.to_string()))
}

/// A cell as the page prints it.
fn cell_text(row: &Row, cell: &Cell) -> String {
    match (cell.value, &cell.milestone) {
        (Some(value), _) => {
            let mut text = value_text(row, value);
            if let Some(words) = cell.count.and_then(|count| count_text(row, count)) {
                let _ = write!(text, " {words}");
            }
            text
        }
        (None, Some(milestone)) => format!("not measured: {}", milestone_link(milestone)),
        (None, None) => "not measured".to_owned(),
    }
}

/// A link to a milestone's row on the records page, its words in the link.
fn milestone_link(milestone: &str) -> String {
    let anchor = milestone.to_ascii_lowercase().replace('.', "-");
    let words = milestone_words(milestone).unwrap_or("a milestone");
    format!(
        "[{milestone}, {words}](https://hpr.fusionspace.co/decisions-and-roadmap.html#{anchor})"
    )
}

/// What each milestone a cell waits on will do, in a few words; `None` for one the page doesn't
/// know, which [`check_milestones`] refuses.
fn milestone_words(milestone: &str) -> Option<&'static str> {
    Some(match milestone {
        "M0.8a" => "the simulator's guide",
        "M0.11" => "steps to a first answer",
        "M2.3c7" => "published flights past Mach 1",
        "M2.6" => "an open benchmark",
        "M2.7" => "held-out flights",
        "M2.8" => "honest uncertainty",
        "M2.9" => "the other tools' cells",
        _ => return None,
    })
}

/// What an axis measures, in words, above its table.
fn axis_words(axis: Axis, hpr: &str) -> String {
    match axis {
        Axis::Accuracy => "How far each simulator's apogee lands from the logged one, its sign \
            dropped, averaged over the flights: the mean absolute error, per cent of the logged \
            apogee. The public flights are RocketPy's example rockets whose altimeter logs are \
            published; the private collection's are logged flights whose owners allow only \
            aggregates. Held-out flights are ones no model or setting was tuned on, predicted \
            before their logs are compared."
            .to_owned(),
        Axis::Bias => "The same errors with their signs, averaged: above zero, the simulator's \
            apogee comes out higher than the log on average."
            .to_owned(),
        Axis::Uncertainty => "How often the logged apogee falls inside the range a simulator \
            predicts for it from a scattered run (a Monte Carlo): a 50% range should hold half \
            the logs, a 90% range nine in ten."
            .to_owned(),
        Axis::Speed => "Time on one machine, the same for every tool. Shown, never held, since \
            machines differ."
            .to_owned(),
        Axis::Interop => format!(
            "How well {hpr} reads each tool's design file and writes it back: the tool flies the \
             original and {hpr}'s copy, and the two are compared. So the numbers in the \
             OpenRocket column are {hpr}'s, through OpenRocket's file, and {hpr}'s own column \
             is n/a. A configuration is one of a design's motor choices; a design can have \
             several."
        ),
        Axis::Features => "How many of the capabilities listed in the simulator guide's \
            comparison table each tool has; the table comes with that guide."
            .to_owned(),
        Axis::Steps => "How many steps it takes to open a design, fly it and get where it may \
            land, clicks, typed entries and script lines counted apart."
            .to_owned(),
    }
}

/// Who is ahead in a row between HPR Sim's value and another tool's, by the row's rule (lower
/// for speed): `Some(true)` when HPR Sim, `Some(false)` when the other, `None` when level.
fn hpr_ahead(row: &Row, hpr: f64, other: f64) -> Option<bool> {
    let rule = if row.worse == Worse::NotGated {
        Worse::Higher
    } else {
        row.worse
    };
    // `worse(old, new)`: whether `new` is the worse of the two.
    if rule.worse(hpr, other) {
        Some(true)
    } else if rule.worse(other, hpr) {
        Some(false)
    } else {
        None
    }
}

/// The opening summary: how many cells are measured, per tool, and each row where HPR Sim and
/// another tool are both measured, with which is ahead.
fn standing(scoreboard: &Scoreboard) -> String {
    let name = |tool: Tool| scoreboard.tools.get(&tool).map_or("?", String::as_str);
    let hpr = name(Tool::Hpr);
    let cells: Vec<&Cell> = scoreboard.rows.iter().flat_map(|row| &row.cells).collect();
    let measured = cells.iter().filter(|cell| cell.value.is_some()).count();
    let per_tool: Vec<String> = scoreboard
        .tools
        .iter()
        .map(|(tool, tool_name)| {
            let of_tool: Vec<_> = cells.iter().filter(|cell| cell.tool == *tool).collect();
            let done = of_tool.iter().filter(|cell| cell.value.is_some()).count();
            format!("{tool_name}'s {done} of {}", of_tool.len())
        })
        .collect();
    let mut both = Vec::new();
    for row in &scoreboard.rows {
        let Some(mine) = row
            .cells
            .iter()
            .find(|cell| cell.tool == Tool::Hpr)
            .and_then(|cell| cell.value)
        else {
            continue;
        };
        for cell in row.cells.iter().filter(|cell| cell.tool != Tool::Hpr) {
            let Some(theirs) = cell.value else { continue };
            let other = name(cell.tool);
            let verdict = match hpr_ahead(row, mine, theirs) {
                Some(true) => format!("{hpr} ahead"),
                Some(false) => format!("{other} ahead"),
                None => "level".to_owned(),
            };
            both.push(format!(
                "`{}`, {hpr} {} and {other} {}: {verdict}",
                row.id,
                value_text(row, mine),
                value_text(row, theirs)
            ));
        }
    }
    let mut text = format!(
        "**Where it stands.** {measured} of its {} cells are measured: {}.",
        cells.len(),
        per_tool.join(", ")
    );
    if both.is_empty() {
        let _ = write!(text, " No row yet measures {hpr} and another tool both.");
    } else {
        let _ = write!(
            text,
            " Where {hpr} and another tool are both measured: {}.",
            both.join("; ")
        );
    }
    text
}

/// The scoreboard's page.
pub(crate) fn render(scoreboard: &Scoreboard) -> String {
    let names: Vec<&str> = scoreboard.tools.values().map(String::as_str).collect();
    let hpr = scoreboard.tools.get(&Tool::Hpr).map_or("?", String::as_str);
    let mut page = String::new();
    let _ = writeln!(page, "# The scoreboard\n");
    let _ = writeln!(
        page,
        "{hpr} against {}, axis by axis, by the project's standard: never worse than the best \
         comparable tool on any axis it measures \
         ([ADR-209](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0209-the-2026-10-08-best-on-the-market-measured.md), \
         the decision record that set it). Generated by `{}` from the committed reports: each \
         number is read from the report and key the sources below name, rounded as printed; a \
         cell no report measures yet says so and links the milestone that will.\n",
        names.get(1..).unwrap_or_default().join(" and "),
        scoreboard.generated_by
    );
    let _ = writeln!(page, "{}\n", standing(scoreboard));
    let _ = writeln!(
        page,
        "> **How far to trust it.** A cell is exactly as good as the report it comes from. Errors \
         are each simulator's apogee less the logged one, over the logged one; the private \
         collection's are aggregates of flights whose designs and logs are not public. A check \
         fails when one of {hpr}'s numbers gets worse than this page, or counts fewer flights or \
         designs, or a measured cell is no longer measured, unless an accepted decision record \
         names that cell and the release the change ships in \
         ([ADR-240](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0240-the-scoreboard.md), \
         how the check works). The other tools' numbers are shown, not held, and so is speed.\n"
    );
    let mut milestones: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for row in &scoreboard.rows {
        for cell in &row.cells {
            if let Some(milestone) = &cell.milestone {
                milestones
                    .entry(milestone.as_str())
                    .or_default()
                    .push(format!("`{}.{}`", row.id, cell.tool.slug()));
            }
        }
    }
    let _ = writeln!(page, "## Reading it\n");
    let _ = writeln!(
        page,
        "- **A cell's id** is its row's and its tool's, such as `accuracy.public.hpr`: the name a \
         decision record uses to accept a worse number."
    );
    let _ = writeln!(
        page,
        "- **Not measured** means no committed report measures the cell yet; its link names the \
         milestone that will. Those milestones, and the cells each fills:"
    );
    for (milestone, ids) in &milestones {
        let _ = writeln!(
            page,
            "  - {}: {}.",
            milestone_link(milestone),
            ids.join(", ")
        );
    }
    let _ = writeln!(
        page,
        "- **n/a:** a round-trip row measures {hpr} reading and writing another tool's file, so \
         {hpr} has no cell of its own there.\n"
    );
    for (index, row) in scoreboard.rows.iter().enumerate() {
        if index == 0 || scoreboard.rows[index - 1].axis != row.axis {
            let _ = writeln!(page, "## {}\n", row.axis.heading());
            let _ = writeln!(page, "{}\n", axis_words(row.axis, hpr));
            let _ = writeln!(page, "| measure | better | {} |", names.join(" | "));
            let _ = writeln!(page, "|---|---|{}", "---:|".repeat(names.len()));
        }
        let cells: Vec<String> = scoreboard
            .tools
            .keys()
            .map(|tool| {
                row.cells
                    .iter()
                    .find(|cell| cell.tool == *tool)
                    .map_or_else(|| "n/a".to_owned(), |cell| cell_text(row, cell))
            })
            .collect();
        let _ = writeln!(
            page,
            "| {} (`{}`) | {} | {} |",
            row.label,
            row.id,
            row.worse.words(),
            cells.join(" | ")
        );
        if scoreboard
            .rows
            .get(index + 1)
            .is_none_or(|next| next.axis != row.axis)
        {
            page.push('\n');
        }
    }
    let _ = writeln!(page, "## Sources\n");
    for row in &scoreboard.rows {
        for cell in &row.cells {
            if let Some(source) = &cell.source {
                let _ = writeln!(page, "- `{}.{}`: `{source}`", row.id, cell.tool.slug());
            }
        }
    }
    let _ = writeln!(page, "\n## Credits\n");
    let _ = writeln!(page, "{FLIGHT_DATA_CREDIT}\n");
    let _ = writeln!(page, "{ERA5_CREDIT}");
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed reports and scoreboard.
    fn committed() -> (std::path::PathBuf, Reports, Scoreboard) {
        let root = crate::designs::root().expect("the repository root");
        let reports = Reports::read(&root).expect("the committed reports");
        let committed = read_committed(&root)
            .expect("the committed scoreboard")
            .expect("a committed scoreboard");
        (root, reports, committed)
    }

    /// The live check: the committed scoreboard is no worse than the committed reports give, and
    /// current, and each unmeasured cell's milestone is open.
    #[test]
    fn the_committed_scoreboard_holds() {
        let root = crate::designs::root().expect("the repository root");
        check_committed(&root, None).expect("the committed scoreboard holds");
    }

    /// Every row has a cell per tool except the round trips', which have none for HPR Sim, and
    /// every axis ADR-209 §3 names has a row.
    #[test]
    fn every_axis_has_a_row_and_every_tool_a_cell() {
        let (_, reports, _) = committed();
        let scoreboard = scoreboard(&reports).expect("the scoreboard");
        for axis in [
            Axis::Accuracy,
            Axis::Bias,
            Axis::Uncertainty,
            Axis::Speed,
            Axis::Interop,
            Axis::Features,
            Axis::Steps,
        ] {
            assert!(
                scoreboard.rows.iter().any(|row| row.axis == axis),
                "{axis:?}"
            );
        }
        let mut ids = BTreeSet::new();
        for row in &scoreboard.rows {
            assert!(ids.insert(row.id.clone()), "{} twice", row.id);
            let tools: Vec<Tool> = row.cells.iter().map(|cell| cell.tool).collect();
            let expected = if row.axis == Axis::Interop {
                vec![Tool::Openrocket, Tool::Rocketpy]
            } else {
                vec![Tool::Hpr, Tool::Openrocket, Tool::Rocketpy]
            };
            assert_eq!(tools, expected, "{}", row.id);
        }
    }

    /// A worse HPR Sim number in a report fails the gate, for each kind of rule that holds a
    /// measured cell today; a better one passes.
    #[test]
    fn a_report_that_worsens_a_number_fails() {
        let (root, mut reports, committed) = committed();
        let release = next_release(&root).expect("the release");
        let none = BTreeSet::new();
        let real = reports.real_flights.clone();
        let fixture = reports.fixture_flights.clone();
        let export = reports.export_flights.clone();
        let set = every_flight(&reports).expect("the set").1;

        // A higher mean absolute error, by one printed step.
        let pointer = "/summary/mean_absolute_apogee_error_percent";
        let value = number(&real, REAL_FLIGHTS, pointer).expect("the error");
        *reports
            .real_flights
            .pointer_mut(pointer)
            .expect("the error") = (value + 0.01).into();
        let worse = scoreboard(&reports).expect("the scoreboard");
        let error = gate(&committed, &worse, &release, &none).expect_err("worse fails");
        assert!(error.contains("accuracy.public.hpr"), "{error}");
        // A lower one passes.
        *reports
            .real_flights
            .pointer_mut(pointer)
            .expect("the error") = (value - 0.01).into();
        let better = scoreboard(&reports).expect("the scoreboard");
        gate(&committed, &better, &release, &none).expect("better passes");
        reports.real_flights = real;

        // A bias farther from zero on the other side.
        let pointer = format!("{set}/hpr/mean_percent");
        let value = number(&fixture, FIXTURE_FLIGHTS, &pointer).expect("the bias");
        *reports
            .fixture_flights
            .pointer_mut(&pointer)
            .expect("the bias") = (-(value.abs() + 0.01)).into();
        let worse = scoreboard(&reports).expect("the scoreboard");
        let error = gate(&committed, &worse, &release, &none).expect_err("worse fails");
        assert!(error.contains("bias.collection.hpr"), "{error}");
        // The same size on the other side is no worse.
        *reports
            .fixture_flights
            .pointer_mut(&pointer)
            .expect("the bias") = (-value).into();
        let mirrored = scoreboard(&reports).expect("the scoreboard");
        gate(&committed, &mirrored, &release, &none).expect("the same size passes");
        reports.fixture_flights = fixture.clone();

        // A round trip that loses a configuration.
        let within = whole(&export, EXPORT_FLIGHTS, "/total/within").expect("within");
        *reports
            .export_flights
            .pointer_mut("/total/within")
            .expect("within") = (within - 1).into();
        let worse = scoreboard(&reports).expect("the scoreboard");
        let error = gate(&committed, &worse, &release, &none).expect_err("worse fails");
        assert!(error.contains("interop.changed.openrocket"), "{error}");
        reports.export_flights = export;

        // OpenRocket's own number getting worse is shown, not held.
        let pointer = format!("{set}/openrocket/mean_absolute_percent");
        let value = number(&fixture, FIXTURE_FLIGHTS, &pointer).expect("the error");
        *reports
            .fixture_flights
            .pointer_mut(&pointer)
            .expect("the error") = (value + 1.0).into();
        let theirs = scoreboard(&reports).expect("the scoreboard");
        gate(&committed, &theirs, &release, &none).expect("another tool's number is shown");
    }

    /// A change below the printed precision is no change.
    #[test]
    fn a_change_below_the_printed_precision_passes() {
        let (root, mut reports, committed) = committed();
        let release = next_release(&root).expect("the release");
        let pointer = "/summary/mean_absolute_apogee_error_percent";
        let value = number(&reports.real_flights, REAL_FLIGHTS, pointer).expect("the error");
        let printed = rounded(value, 2);
        // Just under the next printed step, without crossing it.
        *reports
            .real_flights
            .pointer_mut(pointer)
            .expect("the error") = (printed + 0.004_9).into();
        let nudged = scoreboard(&reports).expect("the scoreboard");
        gate(&committed, &nudged, &release, &BTreeSet::new()).expect("same printed value");
    }

    /// A measured cell that is no longer measured fails, in any column.
    #[test]
    fn a_measured_cell_turned_not_measured_fails() {
        let (root, reports, committed) = committed();
        let release = next_release(&root).expect("the release");
        let mut new = scoreboard(&reports).expect("the scoreboard");
        let row = new
            .rows
            .iter_mut()
            .find(|row| row.id == "accuracy.collection")
            .expect("the row");
        for cell in &mut row.cells {
            if cell.tool == Tool::Openrocket {
                *cell = later(Tool::Openrocket, "M2.3c3");
            }
        }
        let error = gate(&committed, &new, &release, &BTreeSet::new()).expect_err("dropped fails");
        assert!(error.contains("accuracy.collection.openrocket"), "{error}");
        new.rows.retain(|row| row.id != "bias.public");
        let error = gate(&committed, &new, &release, &BTreeSet::new()).expect_err("dropped fails");
        assert!(error.contains("bias.public.hpr"), "{error}");
    }

    /// An accepted ADR naming the cell and the next release lets a worsening through; the wrong
    /// release, another cell, or an ADR not accepted doesn't.
    #[test]
    fn only_an_accepted_adr_naming_the_cell_and_release_accepts_it() {
        let (root, mut reports, committed) = committed();
        let release = next_release(&root).expect("the release");
        let pointer = "/summary/mean_absolute_apogee_error_percent";
        let value = number(&reports.real_flights, REAL_FLIGHTS, pointer).expect("the error");
        *reports
            .real_flights
            .pointer_mut(pointer)
            .expect("the error") = (value + 1.0).into();
        let worse = scoreboard(&reports).expect("the scoreboard");
        let adr = |status: &str, line: &str| -> BTreeSet<(String, String)> {
            acceptances(&format!(
                "# ADR-999: x\n\n- **Status:** {status}\n\n{line}\n"
            ))
            .into_iter()
            .collect()
        };
        let line = format!("- **Scoreboard:** `accuracy.public.hpr` worse in release {release}.");
        let lines = gate(&committed, &worse, &release, &adr("accepted", &line)).expect("accepted");
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(gate(&committed, &worse, &release, &adr("proposed", &line)).is_err());
        let other = "- **Scoreboard:** `accuracy.public.hpr` worse in release 9.9.";
        assert!(gate(&committed, &worse, &release, &adr("accepted", other)).is_err());
        let cell = format!("- **Scoreboard:** `bias.public.hpr` worse in release {release}.");
        assert!(gate(&committed, &worse, &release, &adr("accepted", &cell)).is_err());
        let prose = format!("The scoreboard's `accuracy.public.hpr` in release {release}.");
        assert!(gate(&committed, &worse, &release, &adr("accepted", &prose)).is_err());
    }

    /// The writer needs its mark: a missing scoreboard is refused unless `--first` says none was
    /// ever committed, and `--first` is refused when one is there.
    #[test]
    fn the_writer_needs_its_mark() {
        let (_, _, committed) = committed();
        assert!(mark(None, false).is_err());
        assert_eq!(mark(None, true), Ok(None));
        assert!(mark(Some(committed.clone()), true).is_err());
        assert_eq!(mark(Some(committed.clone()), false), Ok(Some(committed)));
    }

    /// A revision git doesn't know fails rather than comparing with nothing; one it knows gives
    /// the scoreboard committed there, or none before the first.
    #[test]
    fn the_check_against_a_revision_fails_closed() {
        let root = crate::designs::root().expect("the repository root");
        let error =
            scoreboard_at(&root, "no-such-revision-of-the-scoreboard").expect_err("unknown");
        assert!(error.contains("no revision"), "{error}");
        // The repository's first commit holds no scoreboard.
        let first = Command::new("git")
            .args(["rev-list", "--max-parents=0", "HEAD"])
            .current_dir(&root)
            .output()
            .expect("git");
        let first = String::from_utf8_lossy(&first.stdout);
        let first = first.lines().next().expect("a root commit");
        assert_eq!(scoreboard_at(&root, first), Ok(None));
    }

    /// A held number made better by counting fewer flights is worse; another tool's isn't held.
    #[test]
    fn fewer_flights_is_worse() {
        let (root, mut reports, committed) = committed();
        let release = next_release(&root).expect("the release");
        let flights = whole(&reports.real_flights, REAL_FLIGHTS, "/summary/flights").expect("n");
        *reports
            .real_flights
            .pointer_mut("/summary/flights")
            .expect("n") = (flights - 1).into();
        let fewer = scoreboard(&reports).expect("the scoreboard");
        let error = gate(&committed, &fewer, &release, &BTreeSet::new()).expect_err("fewer fails");
        assert!(error.contains("accuracy.public.hpr"), "{error}");
        assert!(
            error.contains(&format!("over {} flights", flights - 1)),
            "{error}"
        );
        *reports
            .real_flights
            .pointer_mut("/summary/flights")
            .expect("n") = (flights + 1).into();
        let more = scoreboard(&reports).expect("the scoreboard");
        gate(&committed, &more, &release, &BTreeSet::new()).expect("more passes");
    }

    /// Values are stored as the page prints them, by formatting, not by scaling.
    #[test]
    fn values_round_as_printed() {
        assert_eq!(rounded(2.675, 2).to_string(), format!("{:.2}", 2.675_f64));
        assert_eq!(rounded(0.125, 2).to_string(), format!("{:.2}", 0.125_f64));
        assert_eq!(rounded(5e-7, 6), 0.0);
        assert_eq!(rounded(0.000_191, 6), 0.000_191);
    }

    /// Who is ahead, by each row's rule: the lower error, the bias nearer zero, the coverage
    /// nearer its level, the lower time.
    #[test]
    fn the_standing_names_who_is_ahead() {
        let (_, reports, _) = committed();
        let board = scoreboard(&reports).expect("the scoreboard");
        let row = |id: &str| board.rows.iter().find(|row| row.id == id).expect("the row");
        assert_eq!(
            hpr_ahead(row("accuracy.collection"), 12.95, 12.02),
            Some(false)
        );
        assert_eq!(
            hpr_ahead(row("accuracy.collection"), 12.02, 12.95),
            Some(true)
        );
        assert_eq!(hpr_ahead(row("bias.collection"), -7.0, 8.0), Some(true));
        assert_eq!(hpr_ahead(row("bias.collection"), 8.0, -8.0), None);
        assert_eq!(
            hpr_ahead(row("uncertainty.range-90"), 85.0, 96.0),
            Some(true)
        );
        assert_eq!(hpr_ahead(row("speed.flight"), 2.0, 3.0), Some(true));
        // The committed collection rows, both measured: the page says who leads each.
        let mine = |id: &str, tool: Tool| {
            row(id)
                .cells
                .iter()
                .find(|c| c.tool == tool)
                .and_then(|c| c.value)
                .expect("measured")
        };
        let text = standing(&board);
        for id in ["accuracy.collection", "bias.collection"] {
            let ahead = hpr_ahead(row(id), mine(id, Tool::Hpr), mine(id, Tool::Openrocket));
            let leader = match ahead {
                Some(true) => "HPR Sim ahead",
                Some(false) => "OpenRocket 24.12 ahead",
                None => "level",
            };
            assert!(text.contains(&format!("`{id}`")), "{text}");
            let after = text
                .split(&format!("`{id}`"))
                .nth(1)
                .expect("the row's entry");
            let entry = after.split(';').next().expect("its verdict");
            assert!(entry.contains(leader), "{id}: {entry}");
        }
    }

    /// The next release is the version's next minor.
    #[test]
    fn the_next_release_is_the_next_minor() {
        assert_eq!(release_after("0.1.0").as_deref(), Some("0.2"));
        assert_eq!(release_after("1.9.3").as_deref(), Some("1.10"));
        assert_eq!(release_after("x"), None);
    }

    /// An unmeasured cell's milestone must be open in the roadmap.
    #[test]
    fn an_unmeasured_cell_needs_an_open_milestone() {
        let (_, reports, _) = committed();
        let scoreboard = scoreboard(&reports).expect("the scoreboard");
        let open = "- [ ] **M0.8a x\n- [ ] **M0.11 x\n  - [ ] **M2.3c7 x\n- [ ] **M2.6 x\n\
                    - [ ] **M2.7 x\n- [ ] **M2.8 x\n- [ ] [blocked] **M2.9 x\n";
        check_milestones(&scoreboard, open).expect("all open");
        let closed = open.replace("- [ ] **M2.8", "- [x] **M2.8");
        let error = check_milestones(&scoreboard, &closed).expect_err("M2.8 closed");
        assert!(error.contains("uncertainty.range-50.hpr"), "{error}");
        // A milestone the page has no words for fails, open or not.
        let mut unknown = scoreboard.clone();
        unknown.rows[2].cells[0].milestone = Some("M9.99".to_owned());
        let open = format!("{open}- [ ] **M9.99 x\n");
        let error = check_milestones(&unknown, &open).expect_err("no words");
        assert!(error.contains("M9.99"), "{error}");
    }

    /// A worse rule at its edges.
    #[test]
    fn worse_rules() {
        assert!(Worse::Higher.worse(1.0, 1.01));
        assert!(!Worse::Higher.worse(1.0, 1.0));
        assert!(Worse::Lower.worse(3.0, 2.0));
        assert!(Worse::FartherFromZero.worse(-1.0, 1.01));
        assert!(!Worse::FartherFromZero.worse(-1.0, 1.0));
        let fifty = Worse::FartherFrom { level: 50.0 };
        assert!(fifty.worse(52.0, 47.0));
        assert!(!fifty.worse(47.0, 53.0));
        assert!(!Worse::NotGated.worse(1.0, 100.0));
    }

    /// The page prints a minus sign, a plus on a signed row, and no sign on a printed zero.
    #[test]
    fn values_print_signs() {
        let (_, reports, _) = committed();
        let scoreboard = scoreboard(&reports).expect("the scoreboard");
        let bias = scoreboard
            .rows
            .iter()
            .find(|r| r.id == "bias.public")
            .expect("the row");
        assert_eq!(value_text(bias, -0.25), "\u{2212}0.25%");
        assert_eq!(value_text(bias, 8.73), "+8.73%");
        assert_eq!(value_text(bias, -0.001), "0.00%");
    }
}
