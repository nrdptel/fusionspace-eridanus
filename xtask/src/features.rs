//! The simulator guide's comparison (M0.8a3, ADR-209 §8, ADR-248): what HPR Sim and the
//! comparable simulators each do, one capability a row, read from
//! `validation/features/simulator.toml`.
//!
//! Each other tool's cell says yes, partly or no and names the page of that tool's documentation
//! that shows it, from the release and on the date the record gives; a partly or a no says what
//! the page shows, in a note. HPR Sim's cell names the page of this site that shows it, or the
//! open roadmap milestone that brings it. The reader refuses a cell without its page, a no
//! without its note, and a record in which no capability is HPR Sim's alone, since ADR-209 §8
//! asks the guide to name one.
//!
//! `cargo xtask guides` writes the table, its notes and its sources into the comparison page
//! between [`REGION_START`] and [`REGION_END`], and `--check` fails a stale region; the scoreboard
//! counts each tool's yes cells into its `features.count` row.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// The record.
pub(crate) const RECORD: &str = "validation/features/simulator.toml";
/// The page that shows it.
pub(crate) const PAGE: &str = "docs/simulator-compared.md";
/// The roadmap, whose open entries HPR Sim's missing capabilities name.
const ROADMAP: &str = "docs/ROADMAP.md";
/// The comment that opens the region `cargo xtask guides` writes, and the one that ends it.
pub(crate) const REGION_START: &str = "<!-- features: table; written by `cargo xtask guides` from validation/features/simulator.toml; do not edit -->";
pub(crate) const REGION_END: &str = "<!-- features: end -->";

/// The tools the scoreboard has columns for, first in the record and in this order.
const SCOREBOARD_TOOLS: [&str; 3] = ["hpr", "openrocket", "rocketpy"];

/// What a tool's documentation, or HPR Sim's site, shows of a capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Has {
    Yes,
    Partly,
    No,
}

impl Has {
    fn words(self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::Partly => "partly",
            Self::No => "no",
        }
    }
}

/// A page of a tool's documentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Source {
    pub id: String,
    pub title: String,
    pub url: String,
}

/// One tool: HPR Sim first, then the others.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Tool {
    pub id: String,
    /// The name the table heads its column with.
    pub name: String,
    /// The release whose documentation was read; HPR Sim's cells name this site's pages instead.
    #[serde(default)]
    pub release: Option<String>,
    /// When that release came out, `YYYY-MM-DD`.
    #[serde(default)]
    pub released: Option<String>,
    /// When the pages were read, `YYYY-MM-DD`.
    #[serde(default)]
    pub checked_on: Option<String>,
    /// What the reading assumes, or where the documentation may differ from the release.
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub source: Vec<Source>,
}

/// One tool's cell in one capability's row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Cell {
    pub tool: String,
    pub has: Has,
    /// The id of the tool's [`Source`] that shows it; another tool's cells only.
    #[serde(default)]
    pub source: Option<String>,
    /// The page of this site that shows it, from the root (`docs/cli.md#hpr-mc`); HPR Sim's yes
    /// or partly only.
    #[serde(default)]
    pub page: Option<String>,
    /// The open milestone that brings it; HPR Sim's partly or no only.
    #[serde(default)]
    pub milestone: Option<String>,
    /// What the page shows, in a few words; a partly or a no must have one.
    #[serde(default)]
    pub note: Option<String>,
}

/// One row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Capability {
    pub id: String,
    /// What a tool does that has it, as the table's first cell: "Flies a design from a command
    /// line".
    pub label: String,
    pub cell: Vec<Cell>,
}

/// The record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Record {
    pub tool: Vec<Tool>,
    pub capability: Vec<Capability>,
}

impl Record {
    /// Reads the record under `root` and holds it to the rules in the module's docs.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(root.join(RECORD))
            .map_err(|error| format!("reading {RECORD}: {error}"))?;
        let record: Self = toml::from_str(&text).map_err(|error| format!("{RECORD}: {error}"))?;
        let roadmap = fs::read_to_string(root.join(ROADMAP))
            .map_err(|error| format!("reading {ROADMAP}: {error}"))?;
        let problems = record.problems(&roadmap, &|page| root.join(page).is_file());
        if problems.is_empty() {
            Ok(record)
        } else {
            Err(format!("{RECORD}:\n  {}", problems.join("\n  ")))
        }
    }

    /// What breaks the rules, given the roadmap and which of the site's pages exist.
    fn problems(&self, roadmap: &str, exists: &dyn Fn(&str) -> bool) -> Vec<String> {
        let mut problems = Vec::new();
        let ids: Vec<&str> = self.tool.iter().map(|tool| tool.id.as_str()).collect();
        if ids.len() < 2 || ids[..SCOREBOARD_TOOLS.len().min(ids.len())] != SCOREBOARD_TOOLS {
            problems.push(format!(
                "lists the tools {ids:?}; it must start with {SCOREBOARD_TOOLS:?}, the \
                 scoreboard's columns, in that order"
            ));
        }
        if ids.iter().collect::<BTreeSet<_>>().len() != ids.len() {
            problems.push(format!("names a tool twice: {ids:?}"));
        }
        for tool in &self.tool {
            problems.extend(tool_problems(tool));
        }
        let open = crate::scoreboard::open_milestones(roadmap);
        let mut seen = BTreeSet::new();
        for capability in &self.capability {
            let at = |what: String| format!("{}: {what}", capability.id);
            if !seen.insert(capability.id.as_str()) {
                problems.push(at("listed twice".to_owned()));
            }
            if capability.label.trim().is_empty() {
                problems.push(at("has no label".to_owned()));
            }
            let tools: Vec<&str> = capability.cell.iter().map(|c| c.tool.as_str()).collect();
            if tools != ids {
                problems.push(at(format!(
                    "has cells for {tools:?}; it needs one for each tool, in the record's order \
                     {ids:?}"
                )));
            }
            for (cell, tool) in capability.cell.iter().zip(&self.tool) {
                for problem in cell_problems(cell, tool, &open, exists) {
                    problems.push(at(format!("{}: {problem}", cell.tool)));
                }
            }
        }
        if self.capability.is_empty() {
            problems.push("lists no capability".to_owned());
        } else if self.only_hpr().is_empty() {
            problems.push(
                "no capability is HPR Sim's alone: the guide must name at least one that none of \
                 the others has (ADR-209 §8), or say why in a decision record"
                    .to_owned(),
            );
        }
        problems
    }

    /// How many capabilities `tool` has in full: its yes cells.
    pub(crate) fn count(&self, tool: &str) -> usize {
        self.capability
            .iter()
            .filter(|capability| {
                capability
                    .cell
                    .iter()
                    .any(|cell| cell.tool == tool && cell.has == Has::Yes)
            })
            .count()
    }

    /// The capabilities HPR Sim has and none of the others has, even in part.
    pub(crate) fn only_hpr(&self) -> Vec<&Capability> {
        self.capability
            .iter()
            .filter(|capability| {
                capability.cell.iter().all(|cell| {
                    (cell.tool == "hpr") == (cell.has == Has::Yes)
                        && (cell.tool == "hpr" || cell.has == Has::No)
                })
            })
            .collect()
    }

    /// The tool `id`.
    fn tool(&self, id: &str) -> Option<&Tool> {
        self.tool.iter().find(|tool| tool.id == id)
    }

    /// The region the page holds between [`REGION_START`] and [`REGION_END`], markers included:
    /// the table, what HPR Sim alone has, where it has less, each partly or no in words, and the
    /// pages read.
    pub(crate) fn region(&self, roadmap: &str) -> Result<String, String> {
        let mut out = format!("{REGION_START}\n\n");
        // Two columns, every tool's answer in one cell that wraps, so the table fits a phone's
        // width without scrolling: a column a tool would push the last ones out of sight.
        out.push_str("| Capability | Each tool |\n|---|---|\n");
        for capability in &self.capability {
            let mut answers = Vec::new();
            for (cell, tool) in capability.cell.iter().zip(&self.tool) {
                // No-break spaces keep a tool's name whole and beside its answer.
                answers.push(format!(
                    "{}\u{a0}[{}]({})",
                    tool.name.replace(' ', "\u{a0}"),
                    cell.has.words(),
                    self.link(cell)?
                ));
            }
            let _ = writeln!(out, "| {} | {} |", capability.label, answers.join("; "));
        }
        out.push_str("\n**What none of the others has.** ");
        let only: Vec<String> = (self.only_hpr().iter())
            .map(|capability| format!("*{}*", sentence_case(&capability.label, false)))
            .collect();
        let _ = writeln!(
            out,
            "Of the capabilities above, HPR Sim alone {}: each of the others' cells in {} says \
             no, from that tool's documentation.",
            list(&only),
            if only.len() == 1 {
                "that row"
            } else {
                "those rows"
            },
        );

        let mut less = Vec::new();
        for capability in &self.capability {
            let hpr = &capability.cell[0];
            let ahead: Vec<&str> = (capability.cell.iter().zip(&self.tool))
                .skip(1)
                .filter(|(cell, _)| cell.has == Has::Yes)
                .map(|(_, tool)| tool.name.as_str())
                .collect();
            if hpr.has == Has::Yes || ahead.is_empty() {
                continue;
            }
            let milestone = hpr
                .milestone
                .as_deref()
                .ok_or_else(|| format!("{}: hpr names no milestone", capability.id))?;
            let does = if ahead.len() == 1 { "does" } else { "do" };
            less.push(format!(
                "- **{}:** {} {does}; HPR Sim {} ({}).",
                sentence_case(&capability.label, true),
                list(&ahead.iter().map(ToString::to_string).collect::<Vec<_>>()),
                if hpr.has == Has::Partly {
                    "does in part"
                } else {
                    "doesn't yet"
                },
                milestone_link(milestone, roadmap)?,
            ));
        }
        if !less.is_empty() {
            out.push_str(
                "\n**Where HPR Sim has less.** Each row where another tool says yes and HPR Sim \
                 doesn't, with the milestone that brings it:\n\n",
            );
            for line in less {
                out.push_str(&line);
                out.push('\n');
            }
        }

        out.push_str(
            "\n**Each partly and no, in words.** As the page its cell links to shows it.\n\n",
        );
        for capability in &self.capability {
            let mut notes = Vec::new();
            for (cell, tool) in capability.cell.iter().zip(&self.tool) {
                if cell.has == Has::Yes {
                    continue;
                }
                let note = cell
                    .note
                    .as_deref()
                    .ok_or_else(|| format!("{}: {} has no note", capability.id, cell.tool))?;
                notes.push(format!("{}: {}", tool.name, sentence_end(note)));
            }
            if !notes.is_empty() {
                let _ = writeln!(
                    out,
                    "- **{}.** {}",
                    sentence_case(&capability.label, true),
                    notes.join(" ")
                );
            }
        }

        out.push_str("\n**The pages read.**\n\n");
        for tool in &self.tool {
            if tool.id == "hpr" {
                let _ = writeln!(
                    out,
                    "- **{}:** each cell links the page of this site that shows it, or the \
                     milestone that brings it.",
                    tool.name
                );
                continue;
            }
            let (Some(release), Some(checked_on)) = (&tool.release, &tool.checked_on) else {
                return Err(format!("{}: no release or date read", tool.id));
            };
            let released = match &tool.released {
                Some(date) => format!(", released {}", prose_date(date)?),
                None => String::new(),
            };
            let sources: Vec<String> = (tool.source.iter())
                .map(|source| format!("[{}]({})", source.title, source.url))
                .collect();
            let note = match &tool.note {
                Some(note) => format!(" {}", sentence_end(note)),
                None => String::new(),
            };
            let _ = writeln!(
                out,
                "- **{} {release}**{released}, read on {}: {}.{note}",
                tool.name,
                prose_date(checked_on)?,
                sources.join(", ")
            );
        }
        out.push('\n');
        out.push_str(REGION_END);
        Ok(out)
    }

    /// Where a cell's word links: the tool's page, HPR Sim's page on this site, or the roadmap
    /// entry that brings what HPR Sim lacks.
    fn link(&self, cell: &Cell) -> Result<String, String> {
        if cell.tool == "hpr" {
            return match (&cell.page, &cell.milestone) {
                (Some(page), _) => Ok(page.strip_prefix("docs/").unwrap_or(page).to_owned()),
                (None, Some(milestone)) => Ok(roadmap_anchor(milestone)),
                (None, None) => Err("an hpr cell with neither page nor milestone".to_owned()),
            };
        }
        let tool = self
            .tool(&cell.tool)
            .ok_or_else(|| format!("no tool {}", cell.tool))?;
        let id = cell.source.as_deref().unwrap_or_default();
        (tool.source.iter())
            .find(|source| source.id == id)
            .map(|source| source.url.clone())
            .ok_or_else(|| format!("{} has no source {id}", tool.id))
    }
}

/// What breaks the rules in one tool's entry.
fn tool_problems(tool: &Tool) -> Vec<String> {
    let mut problems = Vec::new();
    let at = |what: &str| format!("{}: {what}", tool.id);
    if tool.id == "hpr" {
        if tool.release.is_some() || tool.checked_on.is_some() || !tool.source.is_empty() {
            problems.push(at(
                "HPR Sim's cells name this site's pages, so it has no release, date or sources",
            ));
        }
        return problems;
    }
    if tool.release.as_deref().is_none_or(str::is_empty) {
        problems.push(at("names no release whose documentation was read"));
    }
    match &tool.checked_on {
        Some(date) if crate::steps::is_date(date) => {}
        _ => problems.push(at(
            "needs checked_on, the date its pages were read, YYYY-MM-DD",
        )),
    }
    if let Some(date) = &tool.released
        && !crate::steps::is_date(date)
    {
        problems.push(at(&format!("released `{date}` isn't a date")));
    }
    if tool.source.is_empty() {
        problems.push(at("lists no source"));
    }
    let mut ids = BTreeSet::new();
    for source in &tool.source {
        if !ids.insert(source.id.as_str()) {
            problems.push(at(&format!("source {} listed twice", source.id)));
        }
        if !source.url.starts_with("https://") || source.title.trim().is_empty() {
            problems.push(at(&format!(
                "source {} needs a title and an https address",
                source.id
            )));
        }
    }
    problems
}

/// What breaks the rules in one cell, given the open milestones and which pages exist.
fn cell_problems(
    cell: &Cell,
    tool: &Tool,
    open: &BTreeSet<&str>,
    exists: &dyn Fn(&str) -> bool,
) -> Vec<String> {
    let mut problems = Vec::new();
    let note = cell
        .note
        .as_deref()
        .is_some_and(|note| !note.trim().is_empty());
    if cell.has != Has::Yes && !note {
        problems.push(format!("a {} needs a note saying why", cell.has.words()));
    }
    if tool.id == "hpr" {
        if cell.source.is_some() {
            problems.push("HPR Sim's cells name a page of this site, not a source".to_owned());
        }
        match (cell.has, &cell.page) {
            (Has::No, Some(_)) => problems.push("a no names no page".to_owned()),
            (Has::No, None) => {}
            (_, None) => problems.push(format!("a {} needs its page", cell.has.words())),
            (_, Some(page)) => {
                let file = page.split('#').next().unwrap_or_default();
                if !file.starts_with("docs/") || !file.ends_with(".md") || !exists(file) {
                    problems.push(format!("its page {page} is no page of the site"));
                }
            }
        }
        match (cell.has, &cell.milestone) {
            (Has::Yes, Some(_)) => problems.push("a yes names no milestone".to_owned()),
            (Has::Yes, None) => {}
            (_, None) => problems.push(format!(
                "a {} needs the open milestone that brings it",
                cell.has.words()
            )),
            (_, Some(milestone)) if !open.contains(milestone.as_str()) => problems.push(format!(
                "its milestone {milestone} has no open entry in {ROADMAP}"
            )),
            (_, Some(_)) => {}
        }
        return problems;
    }
    if cell.page.is_some() || cell.milestone.is_some() {
        problems.push("another tool's cell names its source, not a page or a milestone".to_owned());
    }
    match &cell.source {
        Some(id) if tool.source.iter().any(|source| &source.id == id) => {}
        Some(id) => problems.push(format!("its source {id} isn't among {}'s", tool.id)),
        None => problems.push("needs the source that shows it".to_owned()),
    }
    problems
}

/// The roadmap page's anchor for `milestone`: `M6.6` is `decisions-and-roadmap.md#m6-6`.
fn roadmap_anchor(milestone: &str) -> String {
    format!(
        "decisions-and-roadmap.md#{}",
        milestone.to_ascii_lowercase().replace('.', "-")
    )
}

/// `milestone` as a link with its roadmap title: `[M6.6, Submission packs](…#m6-6)`.
fn milestone_link(milestone: &str, roadmap: &str) -> Result<String, String> {
    let title = roadmap
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("- [ ] "))
        .map(|rest| rest.strip_prefix("[blocked] ").unwrap_or(rest))
        .filter_map(|rest| {
            rest.strip_prefix("**")?
                .strip_prefix(milestone)?
                .strip_prefix(' ')
        })
        .find_map(|rest| rest.split("**").next())
        .ok_or_else(|| format!("{milestone} has no open entry in {ROADMAP}"))?;
    let title = title.trim_end_matches([',', '.', ':']).trim();
    Ok(format!(
        "[{milestone}, {title}]({})",
        roadmap_anchor(milestone)
    ))
}

/// `label` with its first letter in capitals (`upper`) or not, for the start or the middle of a
/// sentence.
fn sentence_case(label: &str, upper: bool) -> String {
    let mut chars = label.chars();
    match chars.next() {
        Some(first) if upper => first.to_uppercase().chain(chars).collect(),
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// `note` ending in a full stop.
fn sentence_end(note: &str) -> String {
    let note = note.trim();
    if note.ends_with(['.', '!', '?']) {
        note.to_owned()
    } else {
        format!("{note}.")
    }
}

/// Items joined as a sentence lists them: "a", "a and b", "a, b and c".
fn list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// `YYYY-MM-DD` as running prose writes a date: October 12, 2026.
fn prose_date(date: &str) -> Result<String, String> {
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
    let parts: Vec<&str> = date.split('-').collect();
    let [year, month, day] = parts[..] else {
        return Err(format!("`{date}` isn't a date"));
    };
    let month: usize = month
        .parse()
        .map_err(|_| format!("`{date}` isn't a date"))?;
    let day: u32 = day.parse().map_err(|_| format!("`{date}` isn't a date"))?;
    let name = (month.checked_sub(1))
        .and_then(|at| MONTHS.get(at))
        .ok_or_else(|| format!("`{date}` isn't a date"))?;
    Ok(format!("{name} {day}, {year}"))
}

/// The page's region as it stands, markers included, or why it can't be found.
pub(crate) fn committed_region(page: &str) -> Result<&str, String> {
    let start = page
        .find(REGION_START)
        .ok_or_else(|| format!("{PAGE} has no `{REGION_START}`"))?;
    let end = page[start..]
        .find(REGION_END)
        .map(|at| start + at + REGION_END.len())
        .ok_or_else(|| format!("{PAGE}'s table never ends with `{REGION_END}`"))?;
    if page[end..].contains(REGION_START) {
        return Err(format!("{PAGE} holds the table twice"));
    }
    Ok(&page[start..end])
}

/// Whether the page's region is what the record writes now: `Some(problem)` if not.
pub(crate) fn stale(root: &Path) -> Result<Option<String>, String> {
    let page = fs::read_to_string(root.join(PAGE)).map_err(|error| format!("{PAGE}: {error}"))?;
    let page = page.replace("\r\n", "\n");
    let roadmap = fs::read_to_string(root.join(ROADMAP))
        .map_err(|error| format!("reading {ROADMAP}: {error}"))?;
    let region = Record::read(root)?.region(&roadmap)?;
    Ok((committed_region(&page)? != region).then(|| {
        format!("{PAGE}'s table is not what {RECORD} writes now; run `cargo xtask guides`")
    }))
}

/// Writes the page's region from the record, if it is stale.
pub(crate) fn write(root: &Path) -> Result<(), String> {
    if stale(root)?.is_none() {
        return Ok(());
    }
    let page = fs::read_to_string(root.join(PAGE)).map_err(|error| format!("{PAGE}: {error}"))?;
    let page = page.replace("\r\n", "\n");
    let roadmap = fs::read_to_string(root.join(ROADMAP))
        .map_err(|error| format!("reading {ROADMAP}: {error}"))?;
    let region = Record::read(root)?.region(&roadmap)?;
    let old = committed_region(&page)?;
    fs::write(root.join(PAGE), page.replacen(old, &region, 1))
        .map_err(|error| format!("{PAGE}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROADMAP: &str = "- [ ] **M9.1 Desktop app shell.** A window.\n\
                           - [ ] [blocked] **M6.6 Submission packs** (ADR-163).\n";

    fn record() -> Record {
        toml::from_str(
            r#"
            [[tool]]
            id = "hpr"
            name = "HPR Sim"

            [[tool]]
            id = "openrocket"
            name = "OpenRocket"
            release = "24.12"
            released = "2025-07-27"
            checked_on = "2026-10-12"
            [[tool.source]]
            id = "guide"
            title = "Its guide"
            url = "https://example.org/guide"

            [[tool]]
            id = "rocketpy"
            name = "RocketPy"
            release = "1.13.0"
            checked_on = "2026-10-12"
            [[tool.source]]
            id = "docs"
            title = "Its docs"
            url = "https://example.org/docs"

            [[capability]]
            id = "command-line"
            label = "Flies a design from a command line"
            [[capability.cell]]
            tool = "hpr"
            has = "yes"
            page = "docs/cli.md"
            [[capability.cell]]
            tool = "openrocket"
            has = "no"
            source = "guide"
            note = "its arguments set options for the window"
            [[capability.cell]]
            tool = "rocketpy"
            has = "no"
            source = "docs"
            note = "a Python library"

            [[capability]]
            id = "editor"
            label = "Draws a design in a window"
            [[capability.cell]]
            tool = "hpr"
            has = "no"
            milestone = "M9.1"
            note = "no window yet"
            [[capability.cell]]
            tool = "openrocket"
            has = "yes"
            source = "guide"
            [[capability.cell]]
            tool = "rocketpy"
            has = "no"
            source = "docs"
            note = "no window"
            "#,
        )
        .unwrap()
    }

    fn problems(record: &Record) -> Vec<String> {
        record.problems(ROADMAP, &|page| page == "docs/cli.md")
    }

    /// The record the tests start from keeps the rules, and counts and finds what it should.
    #[test]
    fn a_kept_record_passes_and_counts() {
        let record = record();
        assert_eq!(problems(&record), Vec::<String>::new());
        assert_eq!(record.count("hpr"), 1);
        assert_eq!(record.count("openrocket"), 1);
        assert_eq!(record.count("rocketpy"), 0);
        let only: Vec<&str> = record.only_hpr().iter().map(|c| c.id.as_str()).collect();
        assert_eq!(only, ["command-line"]);
    }

    /// Each rule fails a record that breaks it, by its own message.
    #[test]
    fn each_rule_fails_its_record() {
        let fails = |change: fn(&mut Record), expected: &str| {
            let mut record = record();
            change(&mut record);
            let problems = problems(&record);
            assert!(
                problems.iter().any(|problem| problem.contains(expected)),
                "{expected:?} not in {problems:?}"
            );
        };
        fails(|r| r.capability[0].cell[1].note = None, "a no needs a note");
        fails(
            |r| r.capability[0].cell[1].source = None,
            "needs the source",
        );
        fails(
            |r| r.capability[0].cell[1].source = Some("other".to_owned()),
            "isn't among openrocket's",
        );
        fails(
            |r| r.capability[0].cell[0].page = None,
            "a yes needs its page",
        );
        fails(
            |r| r.capability[0].cell[0].page = Some("docs/missing.md".to_owned()),
            "is no page of the site",
        );
        fails(
            |r| r.capability[1].cell[0].milestone = Some("M2.9".to_owned()),
            "has no open entry",
        );
        fails(
            |r| r.capability[1].cell[0].milestone = None,
            "needs the open milestone",
        );
        fails(
            |r| r.capability[0].cell[2].has = Has::Partly,
            "no capability is HPR Sim's alone",
        );
        fails(|r| r.capability[0].cell.swap(1, 2), "in the record's order");
        fails(|r| r.tool.swap(1, 2), "must start with");
        fails(|r| r.tool[1].checked_on = None, "needs checked_on");
        fails(
            |r| r.tool[1].released = Some("2025-13-01".to_owned()),
            "isn't a date",
        );
        fails(
            |r| r.capability[1].id = "command-line".to_owned(),
            "listed twice",
        );
    }

    /// A partly isn't a yes: it counts for nothing and keeps a capability from being HPR Sim's
    /// alone, whichever side holds it.
    #[test]
    fn a_partly_is_not_a_yes() {
        let mut record = record();
        record.capability[0].cell[0].has = Has::Partly;
        record.capability[0].cell[0].milestone = Some("M6.6".to_owned());
        record.capability[0].cell[0].note = Some("in the library".to_owned());
        assert_eq!(record.count("hpr"), 0);
        assert!(record.only_hpr().is_empty());
    }

    /// The region holds the table, the capability HPR Sim alone has, where it has less with the
    /// milestone's link and title, each no in words, and each tool's release and date in prose.
    #[test]
    fn the_region_says_it_all() {
        let region = record().region(ROADMAP).unwrap();
        assert!(region.starts_with(REGION_START) && region.ends_with(REGION_END));
        for expected in [
            "| Capability | Each tool |",
            "| Flies a design from a command line | HPR\u{a0}Sim\u{a0}[yes](cli.md); OpenRocket\u{a0}[no](https://example.org/guide); RocketPy\u{a0}[no](https://example.org/docs) |",
            "HPR Sim alone *flies a design from a command line*",
            "- **Draws a design in a window:** OpenRocket does; HPR Sim doesn't yet ([M9.1, Desktop app shell](decisions-and-roadmap.md#m9-1)).",
            "- **Flies a design from a command line.** OpenRocket: its arguments set options for the window. RocketPy: a Python library.",
            "- **OpenRocket 24.12**, released July 27, 2025, read on October 12, 2026: [Its guide](https://example.org/guide).",
        ] {
            assert!(region.contains(expected), "{expected:?} not in\n{region}");
        }
    }

    /// The committed record keeps the rules, and the page holds its region as it writes it now.
    #[test]
    fn the_committed_record_and_page_are_current() {
        let root = crate::designs::root().unwrap();
        Record::read(&root).unwrap();
        assert_eq!(stale(&root).unwrap(), None);
    }

    #[test]
    fn dates_read_as_prose() {
        assert_eq!(prose_date("2025-07-27").unwrap(), "July 27, 2025");
        assert!(prose_date("2025-00-27").is_err());
        assert!(prose_date("2025-13-27").is_err());
    }
}
