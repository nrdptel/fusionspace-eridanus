//! `cargo xtask records`: the generated indexes of the project's records (M0.5, ADR-144 §1a and
//! §1d in `docs/decisions/`).
//!
//! Each decision record (ADR) is one file under `docs/decisions/`, `NNNN-slug.md`, which opens
//!
//! ```text
//! # ADR-NNN: Title (YYYY-MM-DD)
//!
//! - **Status:** accepted
//! - **Summary:** one line that says what it decides
//! ```
//!
//! and the files are the only place a record is written. From them this command writes:
//!
//! - `docs/DECISIONS.md`, the index: one row per record with its link, summary and status. Each
//!   row also carries the anchor the record's heading had when the log was one file, so links
//!   made before the split (`DECISIONS.md#adr-003-frames-...`) still land on its row.
//! - the block of `[adr-NNN]: ...` link definitions in `docs/decisions-and-roadmap.md`, so the
//!   site's records page links every record by its file.
//!
//! `--check` writes nothing and fails if a file is out of date; `cargo test -p xtask` runs the
//! same check, so a new record without a regenerated index fails the gate.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::roadmap;

pub const USAGE: &str =
    "  records [--check]        Move milestones checked off in docs/ROADMAP.md to
                           docs/roadmap-done.md, lines unchanged; write docs/DECISIONS.md
                           and the decision links of docs/decisions-and-roadmap.md from
                           the records under docs/decisions/. --check writes nothing and
                           fails if any of them is out of date.";

/// The directory of decision records, relative to the workspace root.
pub const DECISIONS_DIR: &str = "docs/decisions";
/// The generated index of decision records, relative to the workspace root.
pub const DECISIONS_INDEX: &str = "docs/DECISIONS.md";
/// The site's page of records, relative to the workspace root.
const RECORDS_PAGE: &str = "docs/decisions-and-roadmap.md";
/// How a GitHub URL names a file of this repository on `main`.
const BLOB: &str = "https://github.com/nrdptel/fusionspace-eridanus/blob/main/";

/// What the index says above its table.
const INDEX_HEAD: &str = "\
# Decisions (ADR log)

Each architecture decision record (ADR) is one file under [`decisions/`](decisions/), named by its
number and title. This index is generated from those files by `cargo xtask records`; edit the
record, then rerun it. Never edit the table by hand: `cargo test -p xtask` fails when it is stale.
For ADR-000 to ADR-145 the summary is the old index's title column, so it often repeats the title.

**What earns a record:** a decision that is hard to reverse or that constrains later work
([Nygard](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions),
[Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html),
[MADR](https://adr.github.io/madr/)). An increment's details go on its physics page or its roadmap
entry, not here.

**Writing one:** copy the newest file, take the next number, and keep the shape: the heading
`# ADR-NNN: Title (YYYY-MM-DD)`, then `- **Status:**` and `- **Summary:**` lines, then
**Context.**, **Decision.** and **Consequences.** The summary is one plain sentence: it also
becomes the record's row on the site's records page. Number records in order and never renumber.
Supersede a record with a new one that points back, and edit the old one's status line to say so.

";

/// One decision record, read from its file.
#[derive(Debug, Clone, PartialEq)]
pub struct Adr {
    /// `ADR-003`.
    pub id: String,
    /// The number, 3.
    pub number: u32,
    /// The heading less its `# `: `ADR-003: Frames, attitude, ... (2026-09-17)`.
    pub heading: String,
    /// The file's name under [`DECISIONS_DIR`].
    pub file: String,
    pub status: String,
    pub summary: String,
}

impl Adr {
    /// The anchor GitHub gave the record's heading when the log was one file.
    pub fn old_anchor(&self) -> String {
        crate::site::slug(&self.heading)
    }

    /// The title alone: the heading without its id and date.
    pub fn title(&self) -> &str {
        let rest = self.heading.split_once(": ").map_or("", |(_, rest)| rest);
        rest.rsplit_once(" (").map_or(rest, |(title, _)| title)
    }

    /// The record's GitHub URL.
    pub fn url(&self) -> String {
        format!("{BLOB}{DECISIONS_DIR}/{}", self.file)
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            _ => return Err(format!("unexpected argument `{arg}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let stale = write_all(&root, check)?;
    if check && !stale.is_empty() {
        return Err(format!(
            "out of date: {}; run `cargo xtask records`",
            stale.join(", ")
        ));
    }
    let verb = if check { "current" } else { "written" };
    println!(
        "records: {}, {}, {DECISIONS_INDEX} and {RECORDS_PAGE}'s decision links are {verb}",
        roadmap::ROADMAP,
        roadmap::ARCHIVE
    );
    Ok(())
}

/// Regenerates every generated file under `root`; with `check`, writes nothing. Returns the files
/// whose committed text differs from what they should be.
pub fn write_all(root: &Path, check: bool) -> Result<Vec<String>, String> {
    let adrs = read_adrs(&root.join(DECISIONS_DIR))?;
    let mut outputs = vec![(DECISIONS_INDEX, render_index(&adrs))];
    let roadmap = roadmap::parse(&read(root, roadmap::ROADMAP)?)
        .map_err(|err| format!("{}: {err}", roadmap::ROADMAP))?;
    // A missing archive is an empty one; any other failure to read it stops here, since writing
    // over it would lose every entry it holds.
    let archive = match fs::read_to_string(root.join(roadmap::ARCHIVE)) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(format!("could not read {}: {err}", roadmap::ARCHIVE)),
    };
    let archive = roadmap::parse(&archive).map_err(|err| format!("{}: {err}", roadmap::ARCHIVE))?;
    let (open, done) = roadmap::archive(&roadmap, &archive)?;
    // The archive is written before the roadmap, so a failed write never leaves a moved entry in
    // neither file.
    outputs.push((roadmap::ARCHIVE, roadmap::render(&done)));
    outputs.push((roadmap::ROADMAP, roadmap::render(&open)));
    let page = read(root, RECORDS_PAGE)?;
    let page = adr_rows(&page, &adrs, &open, &done)?;
    let page = milestone_table(&page, &open, &done)?;
    let page = phase_definitions(&page, &open, &done)?;
    outputs.push((RECORDS_PAGE, records_page(&page, &adrs)?));
    let mut stale = Vec::new();
    for (path, want) in outputs {
        let have = fs::read_to_string(root.join(path)).unwrap_or_default();
        if have != want {
            stale.push(path.to_owned());
            if !check {
                fs::write(root.join(path), want)
                    .map_err(|err| format!("could not write {path}: {err}"))?;
            }
        }
    }
    Ok(stale)
}

fn read(root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(path)).map_err(|err| format!("could not read {path}: {err}"))
}

/// Every record under `dir`, in number order. A file that doesn't parse, a gap in the numbers or
/// a file name that doesn't start with the record's number is an error, so a record can't be
/// dropped or misfiled unnoticed.
pub fn read_adrs(dir: &Path) -> Result<Vec<Adr>, String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("could not read {}: {err}", dir.display()))?;
    let mut adrs = Vec::new();
    let mut problems = Vec::new();
    for entry in entries.flatten() {
        let file = entry.file_name().to_string_lossy().into_owned();
        if !file.ends_with(".md") {
            continue;
        }
        let text = fs::read_to_string(entry.path())
            .map_err(|err| format!("could not read {file}: {err}"))?;
        match parse_adr(&file, &text) {
            Ok(adr) => adrs.push(adr),
            Err(why) => problems.push(format!("{DECISIONS_DIR}/{file}: {why}")),
        }
    }
    adrs.sort_by_key(|adr| adr.number);
    for (want, adr) in (0..).zip(&adrs) {
        if adr.number != want {
            problems.push(format!(
                "{DECISIONS_DIR}: the records skip or repeat a number at {}: expected ADR-{want:03}",
                adr.id
            ));
            break;
        }
    }
    if problems.is_empty() {
        Ok(adrs)
    } else {
        Err(problems.join("\n"))
    }
}

/// The text of the record `id` (`ADR-097`) under `root`'s [`DECISIONS_DIR`].
#[cfg(test)]
pub fn adr_text(root: &Path, id: &str) -> Result<String, String> {
    let number = id
        .strip_prefix("ADR-")
        .and_then(|digits| digits.parse::<u32>().ok())
        .ok_or_else(|| format!("`{id}` is not a record's id"))?;
    let dir = root.join(DECISIONS_DIR);
    let entries =
        fs::read_dir(&dir).map_err(|err| format!("could not read {}: {err}", dir.display()))?;
    let prefix = format!("{number:04}-");
    let path = entries
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with(&prefix))
        })
        .ok_or_else(|| format!("no file for {id} under {DECISIONS_DIR}"))?;
    fs::read_to_string(&path).map_err(|err| format!("could not read {}: {err}", path.display()))
}

/// Reads one record's heading, status and summary.
pub fn parse_adr(file: &str, text: &str) -> Result<Adr, String> {
    let mut lines = text.lines();
    let heading = lines
        .next()
        .and_then(|line| line.strip_prefix("# "))
        .ok_or("the first line is not `# ADR-NNN: Title (YYYY-MM-DD)`")?;
    let (id, rest) = heading
        .split_once(": ")
        .ok_or("the heading has no `ADR-NNN: ` before its title")?;
    let number: u32 = id
        .strip_prefix("ADR-")
        .filter(|digits| digits.len() == 3 && digits.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|digits| digits.parse().ok())
        .ok_or_else(|| format!("`{id}` is not `ADR-` and three digits"))?;
    let dated = rest.rsplit_once(" (").is_some_and(|(_, date)| {
        let date = date.strip_suffix(')').unwrap_or_default();
        date.len() == 10
            && date.bytes().enumerate().all(|(i, b)| {
                if i == 4 || i == 7 {
                    b == b'-'
                } else {
                    b.is_ascii_digit()
                }
            })
    });
    if !dated {
        return Err("the heading doesn't end with its date, `(YYYY-MM-DD)`".to_owned());
    }
    if !file.starts_with(&format!("{number:04}-")) {
        return Err(format!("the file name doesn't start with `{number:04}-`"));
    }
    if lines.next() != Some("") {
        return Err("no blank line after the heading".to_owned());
    }
    let field = |line: Option<&str>, name: &str| {
        line.and_then(|line| line.strip_prefix(&format!("- **{name}:** ")))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                format!(
                    "line {} is not `- **{name}:** ...`",
                    if name == "Status" { 3 } else { 4 }
                )
            })
    };
    let status = field(lines.next(), "Status")?;
    let summary = field(lines.next(), "Summary")?;
    for value in [&status, &summary] {
        if value.contains('|') {
            return Err(
                "a status or summary holds `|`, which would break the index's table".to_owned(),
            );
        }
    }
    if rest.contains(['|', '[', ']']) {
        return Err(
            "the title holds `|`, `[` or `]`, which would break the records page's table"
                .to_owned(),
        );
    }
    Ok(Adr {
        id: id.to_owned(),
        number,
        heading: heading.to_owned(),
        file: file.to_owned(),
        status,
        summary,
    })
}

/// The text of [`DECISIONS_INDEX`].
pub fn render_index(adrs: &[Adr]) -> String {
    let mut out = String::from(INDEX_HEAD);
    out.push_str("| id | summary | status |\n|---|---|---|\n");
    for adr in adrs {
        out.push_str(&format!(
            "| <a id=\"{}\"></a>[{}](decisions/{}) | {} | {} |\n",
            adr.old_anchor(),
            adr.id,
            adr.file,
            adr.summary,
            adr.status
        ));
    }
    out
}

/// The records page with its `[adr-NNN]: ...` definitions replaced by one sorted block, one per
/// record, put where the first of them was.
pub fn records_page(page: &str, adrs: &[Adr]) -> Result<String, String> {
    let is_definition = |line: &str| {
        line.strip_prefix("[adr-")
            .and_then(|rest| rest.split_once("]: "))
            .is_some_and(|(digits, _)| {
                digits.len() == 3 && digits.bytes().all(|b| b.is_ascii_digit())
            })
    };
    let mut out = Vec::new();
    let mut placed = false;
    for line in page.lines() {
        if is_definition(line) {
            if !placed {
                for adr in adrs {
                    out.push(format!("[adr-{:03}]: {}", adr.number, adr.url()));
                }
                placed = true;
            }
            continue;
        }
        out.push(line.to_owned());
    }
    if !placed {
        return Err(format!(
            "{RECORDS_PAGE} has no `[adr-NNN]: ...` link definitions to replace"
        ));
    }
    Ok(out.join("\n") + "\n")
}

/// A milestone as the records page's table shows it.
struct Row {
    id: String,
    /// `done`, `blocked` or `not yet done`, as the site's check reads the roadmap.
    status: &'static str,
    /// The phase number, from `## Phase N: ...`.
    phase: u32,
    title: String,
}

/// Every milestone and increment of the roadmap and its archive, in table order: by phase, then
/// by id, an increment right after its parent.
fn rows(open: &roadmap::Doc, done: &roadmap::Doc) -> Result<Vec<Row>, String> {
    let mut out = Vec::new();
    for doc in [open, done] {
        for phase in &doc.phases {
            let number = phase
                .heading
                .strip_prefix("## Phase ")
                .and_then(|rest| rest.split(':').next())
                .and_then(|n| n.trim().parse().ok())
                .ok_or_else(|| format!("`{}` has no phase number", phase.heading))?;
            for entry in &phase.entries {
                let status = match entry.kind {
                    roadmap::Kind::Stub => continue,
                    roadmap::Kind::Done => "done",
                    roadmap::Kind::Open if entry.lines[0].contains("] [blocked] ") => "blocked",
                    roadmap::Kind::Open => "not yet done",
                };
                out.push(Row {
                    id: entry.id.clone(),
                    status,
                    phase: number,
                    title: roadmap::title(&entry.lines[0]),
                });
            }
        }
    }
    out.sort_by_key(|row| (row.phase, id_key(&row.id)));
    let mut seen = BTreeSet::new();
    if let Some(row) = out.iter().find(|row| !seen.insert(row.id.clone())) {
        return Err(format!(
            "{} is an entry twice across {} and {}; it belongs in one place",
            row.id,
            roadmap::ROADMAP,
            roadmap::ARCHIVE
        ));
    }
    Ok(out)
}

/// Sorts milestone ids as numbers: M1.9 before M1.10, M2.1b before M2.1b1 before M2.1c.
fn id_key(id: &str) -> (u32, u32, Option<char>, Option<u32>) {
    let rest = id.trim_start_matches('M');
    let (topic, rest) = rest.split_once('.').unwrap_or((rest, ""));
    let place_end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    let (place, rest) = rest.split_at(place_end);
    let mut chars = rest.chars();
    let letter = chars.next();
    let digits: String = chars.collect();
    (
        topic.parse().unwrap_or(0),
        place.parse().unwrap_or(0),
        letter,
        digits.parse().ok(),
    )
}

/// The records page with its table of milestones regenerated from the roadmap and its archive:
/// one row per milestone or increment, its status, and a link to its phase in the file that holds
/// it (`[phase-N]` in the roadmap, `[done-N]` in the archive). A row's plain words are kept; a new
/// milestone's row starts from its title. A row for a milestone neither file has is an error.
pub fn milestone_table(
    page: &str,
    open: &roadmap::Doc,
    done: &roadmap::Doc,
) -> Result<String, String> {
    let lines: Vec<&str> = page.lines().collect();
    let is_row = |line: &str| line.starts_with("| <a id=\"m");
    let first = lines.iter().position(|l| is_row(l));
    let last = lines.iter().rposition(|l| is_row(l));
    let (Some(first), Some(last)) = (first, last) else {
        return Err(format!("{RECORDS_PAGE} has no table of milestones"));
    };
    let mut words = BTreeMap::new();
    for line in &lines[first..=last] {
        if !is_row(line) {
            return Err(format!(
                "{RECORDS_PAGE}: the table of milestones is broken by `{line}`"
            ));
        }
        let id = line
            .split_once("</a>[")
            .and_then(|(_, rest)| rest.split_once(']'))
            .map(|(id, _)| id.to_owned())
            .ok_or_else(|| format!("{RECORDS_PAGE}: can't read the row `{line}`"))?;
        let cells: Vec<&str> = line.trim_end_matches('|').split(" | ").collect();
        let text = cells
            .get(1)
            .ok_or_else(|| format!("{RECORDS_PAGE}: the row `{line}` has no plain words"))?;
        words.insert(id, (*text).to_owned());
    }
    let all = rows(open, done)?;
    for id in words.keys() {
        if !all.iter().any(|row| &row.id == id) {
            return Err(format!(
                "{RECORDS_PAGE} has a row for {id}, which neither {} nor {} holds",
                roadmap::ROADMAP,
                roadmap::ARCHIVE
            ));
        }
    }
    let table = all.iter().map(|row| {
        let kind = if row.status == "done" {
            "done"
        } else {
            "phase"
        };
        let text = words
            .get(&row.id)
            .cloned()
            .unwrap_or_else(|| row.title.clone());
        format!(
            "| <a id=\"{}\"></a>[{}][{kind}-{}] | {text} | {} |",
            crate::site::milestone_anchor(&row.id),
            row.id,
            row.phase,
            row.status
        )
    });
    let out: Vec<String> = lines[..first]
        .iter()
        .map(|l| (*l).to_owned())
        .chain(table)
        .chain(lines[last + 1..].iter().map(|l| (*l).to_owned()))
        .collect();
    Ok(out.join("\n") + "\n")
}

/// The records page with its `[phase-N]` and `[done-N]` definitions regenerated: one per phase of
/// the roadmap and of the archive, put where the first of them was.
pub fn phase_definitions(
    page: &str,
    open: &roadmap::Doc,
    done: &roadmap::Doc,
) -> Result<String, String> {
    let is_definition = |line: &str| {
        ["[phase-", "[done-"].iter().any(|p| {
            line.strip_prefix(p)
                .and_then(|rest| rest.split_once("]: "))
                .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        })
    };
    let mut block = Vec::new();
    for (doc, file, kind) in [
        (open, roadmap::ROADMAP, "phase"),
        (done, roadmap::ARCHIVE, "done"),
    ] {
        for phase in &doc.phases {
            let name = phase.heading.trim_start_matches("## ");
            let number = name
                .strip_prefix("Phase ")
                .and_then(|rest| rest.split(':').next())
                .unwrap_or_default();
            block.push(format!(
                "[{kind}-{number}]: {BLOB}{file}#{}",
                crate::site::slug(name)
            ));
        }
    }
    let mut out = Vec::new();
    let mut placed = false;
    for line in page.lines() {
        if is_definition(line) {
            if !placed {
                out.append(&mut block);
                placed = true;
            }
            continue;
        }
        out.push(line.to_owned());
    }
    if !placed {
        return Err(format!(
            "{RECORDS_PAGE} has no `[phase-N]: ...` definitions to replace"
        ));
    }
    Ok(out.join("\n") + "\n")
}

/// The records page with a row added to its table of decisions for each record that has none,
/// in number order: the record's title, its summary with milestone and record labels made links,
/// and an empty "where it shows" cell. Rows already there are kept as written.
pub fn adr_rows(
    page: &str,
    adrs: &[Adr],
    open: &roadmap::Doc,
    done: &roadmap::Doc,
) -> Result<String, String> {
    let row_number = |line: &str| -> Option<u32> {
        line.strip_prefix("| [ADR-")
            .and_then(|rest| rest.get(..3))
            .and_then(|digits| digits.parse().ok())
    };
    let mut lines: Vec<String> = page.lines().map(str::to_owned).collect();
    if !lines.iter().any(|l| row_number(l).is_some()) {
        return Err(format!("{RECORDS_PAGE} has no table of decisions"));
    }
    let milestones: BTreeSet<String> = rows(open, done)?.into_iter().map(|row| row.id).collect();
    for adr in adrs {
        if lines.iter().any(|l| row_number(l) == Some(adr.number)) {
            continue;
        }
        // After the last row with a smaller number, or before the first row.
        let at = lines
            .iter()
            .rposition(|l| row_number(l).is_some_and(|n| n < adr.number))
            .map(|i| i + 1)
            .or_else(|| lines.iter().position(|l| row_number(l).is_some()))
            .unwrap_or(lines.len());
        lines.insert(
            at,
            format!(
                "| [{}: {}][adr-{:03}] | {} |  |",
                adr.id,
                adr.title(),
                adr.number,
                linked_labels(&adr.summary, &milestones)
            ),
        );
    }
    Ok(lines.join("\n") + "\n")
}

/// `text` with its bare labels made links for the records page: a milestone id that the roadmap
/// or its archive holds becomes a link to its row, and `ADR-NNN` a link to its record. Left alone:
/// code spans, a link's text and its target, autolinks (`<...>`), bare web addresses, and an id
/// that runs on into more of one (`M1.2.3`, `ADR-0012`).
fn linked_labels(text: &str, milestones: &BTreeSet<String>) -> String {
    let mut out = String::new();
    let mut in_code = false;
    let mut depth = 0usize;
    // Inside a link's `(target)`, an autolink's `<...>` or a bare address, until it ends.
    let mut skip_until: Option<char> = None;
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let c = rest.chars().next().unwrap_or_default();
        let next = |len: usize| rest[len..].chars().next();
        let runs_on = |len: usize| {
            next(len).is_some_and(|n| n.is_alphanumeric())
                || (next(len) == Some('.')
                    && rest[len + 1..].starts_with(|d: char| d.is_ascii_digit()))
        };
        let boundary = out
            .chars()
            .last()
            .is_none_or(|p| !p.is_alphanumeric() && p != '[' && p != '-' && p != '/');
        if let Some(end) = skip_until {
            // A bare address and an autolink end at whitespace too: an autolink holds none, so a
            // lone `<` (`AoA < 15°`) can't swallow the rest of the text.
            if c == end || (end != ')' && c.is_whitespace()) {
                skip_until = None;
            }
        } else if c == '`' {
            in_code = !in_code;
        } else if in_code {
        } else if c == '[' {
            depth += 1;
        } else if c == ']' {
            depth = depth.saturating_sub(1);
            if next(1) == Some('(') {
                skip_until = Some(')');
            }
        } else if c == '<' {
            skip_until = Some('>');
        } else if rest.starts_with("http://") || rest.starts_with("https://") {
            skip_until = Some(' ');
        } else if depth == 0 && boundary {
            if let Some(digits) = rest.strip_prefix("ADR-").and_then(|r| r.get(..3))
                && digits.bytes().all(|b| b.is_ascii_digit())
                && !runs_on(7)
            {
                out.push_str(&format!("[ADR-{digits}][adr-{digits}]"));
                i += 7;
                continue;
            }
            if let Some(id) = roadmap::milestone_id(rest).filter(|id| milestones.contains(id))
                && !runs_on(id.len())
            {
                out.push_str(&format!("[{id}](#{})", crate::site::milestone_anchor(&id)));
                i += id.len();
                continue;
            }
        }
        out.push(c);
        i += c.len_utf8();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    const RECORD: &str = "# ADR-003: Frames, attitude (2026-09-17)\n\n- **Status:** accepted\n\
                          - **Summary:** The axes\n\n**Context.** Text.\n";

    #[test]
    fn a_record_is_read_from_its_header() {
        let adr = parse_adr("0003-frames.md", RECORD).unwrap();
        assert_eq!(adr.id, "ADR-003");
        assert_eq!(adr.number, 3);
        assert_eq!(adr.status, "accepted");
        assert_eq!(adr.summary, "The axes");
        assert_eq!(adr.old_anchor(), "adr-003-frames-attitude-2026-09-17");
        assert_eq!(
            adr.url(),
            "https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0003-frames.md"
        );
    }

    #[test]
    fn a_malformed_record_is_refused_with_its_reason() {
        let cases = [
            ("0003-frames.md", "ADR-003: no hash\n", "the first line"),
            (
                "0003-frames.md",
                "# ADR-3: Short (2026-09-17)\n",
                "three digits",
            ),
            ("0003-frames.md", "# ADR-003: Undated\n", "its date"),
            ("0004-frames.md", RECORD, "doesn't start with `0003-`"),
            (
                "0003-frames.md",
                "# ADR-003: T (2026-09-17)\n\n- **Summary:** s\n",
                "line 3 is not `- **Status:**",
            ),
            (
                "0003-frames.md",
                "# ADR-003: T (2026-09-17)\n\n- **Status:** a|b\n- **Summary:** s\n",
                "`|`",
            ),
        ];
        for (file, text, want) in cases {
            let err = parse_adr(file, text).unwrap_err();
            assert!(err.contains(want), "{file}: {err:?} lacks {want:?}");
        }
    }

    #[test]
    fn the_index_keeps_the_old_anchors() {
        let adr = parse_adr("0003-frames.md", RECORD).unwrap();
        let index = render_index(&[adr]);
        assert!(index.ends_with(
            "| <a id=\"adr-003-frames-attitude-2026-09-17\"></a>[ADR-003](decisions/0003-frames.md) \
             | The axes | accepted |\n"
        ));
    }

    #[test]
    fn the_records_page_gets_one_sorted_block_of_definitions() {
        let adr = parse_adr("0003-frames.md", RECORD).unwrap();
        let page = "# Records\n\n[adr-003]: old\n[x]: y\n[adr-001]: old\n";
        assert_eq!(
            records_page(page, &[adr]).unwrap(),
            "# Records\n\n[adr-003]: \
             https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0003-frames.md\n[x]: y\n"
        );
        assert!(records_page("# Records\n", &[]).is_err());
    }

    /// Every relative link in a record reaches a file of the repository. The records aren't site
    /// pages, so the site's link check doesn't read them; the split into files once broke two.
    #[test]
    fn every_relative_link_in_a_record_resolves() {
        let dir = root().join(DECISIONS_DIR);
        let mut broken = Vec::new();
        let mut checked = 0;
        for entry in fs::read_dir(&dir).unwrap().flatten() {
            // Code spans quote text as written, links included; they link nothing.
            let raw = fs::read_to_string(entry.path()).unwrap();
            let text: String = raw
                .split('`')
                .enumerate()
                .filter(|(i, _)| i % 2 == 0)
                .map(|(_, part)| part)
                .collect::<Vec<_>>()
                .join(" ");
            let name = entry.file_name().to_string_lossy().into_owned();
            let mut targets = Vec::new();
            for (i, _) in text.match_indices("](") {
                let rest = &text[i + 2..];
                if let Some(end) = rest.find(')') {
                    targets.push(&rest[..end]);
                }
            }
            for line in text.lines() {
                if let Some((_, target)) = line.strip_prefix('[').and_then(|l| l.split_once("]: "))
                {
                    targets.push(target.trim());
                }
            }
            for target in targets {
                let path = target.split('#').next().unwrap_or_default();
                if path.is_empty() || path.contains("://") || path.starts_with("mailto:") {
                    continue;
                }
                checked += 1;
                if !dir.join(path).exists() {
                    broken.push(format!("{name}: {target}"));
                }
            }
        }
        assert!(checked > 0);
        assert!(broken.is_empty(), "broken links:\n{}", broken.join("\n"));
    }

    fn docs() -> (roadmap::Doc, roadmap::Doc) {
        let open = roadmap::parse(
            "# R\n## Queue\n## Phase 0: Foundations\n- [ ] **M0.5 Lean.**\n  \
             - [ ] [blocked] **M0.5i Report.**\n## Phase 1: Physics\n- [ ] **M1.14 Accuracy.**\n",
        )
        .unwrap();
        let done = roadmap::parse(
            "# D\n## Phase 0: Foundations\n- [x] **M0.1 Workspace, CI.**\n\
             - **M0.5** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).\n  \
             - [x] **M0.5a Files.**\n## Phase 1: Physics\n- [x] **M1.2 Atmosphere.**\n",
        )
        .unwrap();
        (open, done)
    }

    #[test]
    fn labels_become_links_except_where_they_are_quoted_or_part_of_more() {
        let milestones: BTreeSet<String> = ["M1.2", "M1.2b"].map(str::to_owned).into();
        let cases = [
            ("M1.2 and ADR-007.", "[M1.2](#m1-2) and [ADR-007][adr-007]."),
            (
                "(M1.2b; ADR-144 §2)",
                "([M1.2b](#m1-2b); [ADR-144][adr-144] §2)",
            ),
            ("`M1.2` and `ADR-007`", "`M1.2` and `ADR-007`"),
            (
                "[M1.2 text](x.md) and [ADR-007][adr-007]",
                "[M1.2 text](x.md) and [ADR-007][adr-007]",
            ),
            (
                "[x](https://example.com/ADR-001/M1.2)",
                "[x](https://example.com/ADR-001/M1.2)",
            ),
            (
                "<https://example.com/ADR-001>",
                "<https://example.com/ADR-001>",
            ),
            (
                "see https://example.com/M1.2 then",
                "see https://example.com/M1.2 then",
            ),
            (
                "M1.2.3, ADR-0012, XM1.2, M9.9",
                "M1.2.3, ADR-0012, XM1.2, M9.9",
            ),
            // Each skipped region ends, and a label after it is linked.
            ("[x](page.md#M1.2) M1.2", "[x](page.md#M1.2) [M1.2](#m1-2)"),
            ("<M1.2> M1.2", "<M1.2> [M1.2](#m1-2)"),
            (
                "https://h.io/p?ref=M1.2 now M1.2",
                "https://h.io/p?ref=M1.2 now [M1.2](#m1-2)",
            ),
            (
                "AoA < 15 deg, see ADR-143",
                "AoA < 15 deg, see [ADR-143][adr-143]",
            ),
        ];
        for (text, want) in cases {
            assert_eq!(linked_labels(text, &milestones), want, "{text}");
        }
    }

    #[test]
    fn a_record_without_a_row_gets_one_in_number_order() {
        let (open, done) = docs();
        let record = |n: u32, summary: &str| {
            parse_adr(
                &format!("{n:04}-x.md"),
                &format!(
                    "# ADR-{n:03}: Title {n} (2026-10-04)\n\n- **Status:** accepted\n\
                          - **Summary:** {summary}\n"
                ),
            )
            .unwrap()
        };
        let page = "# P\n\n| record | decides | shows |\n|---|---|---|\n\
                    | [ADR-002: B][adr-002] | b | x |\n| [ADR-004: D][adr-004] | d | y |\n\nEnd.\n";
        let adrs = [
            record(1, "First, for M1.2."),
            record(2, "kept"),
            record(3, "Between, see ADR-002."),
            record(4, "kept"),
            record(5, "Last, `M0.5`."),
        ];
        assert_eq!(
            adr_rows(page, &adrs, &open, &done).unwrap(),
            "# P\n\n| record | decides | shows |\n|---|---|---|\n\
             | [ADR-001: Title 1][adr-001] | First, for [M1.2](#m1-2). |  |\n\
             | [ADR-002: B][adr-002] | b | x |\n\
             | [ADR-003: Title 3][adr-003] | Between, see [ADR-002][adr-002]. |  |\n\
             | [ADR-004: D][adr-004] | d | y |\n\
             | [ADR-005: Title 5][adr-005] | Last, `M0.5`. |  |\n\nEnd.\n"
        );
        assert!(
            adr_rows("# P\n", &adrs, &open, &done)
                .unwrap_err()
                .contains("no table of decisions")
        );
        // A title that would break the table is refused when the record is read.
        let err = parse_adr(
            "0001-x.md",
            "# ADR-001: a | b (2026-10-04)\n\n- **Status:** accepted\n- **Summary:** s\n",
        )
        .unwrap_err();
        assert!(err.contains("the title holds"), "{err}");
    }

    #[test]
    fn the_milestone_table_follows_both_files() {
        let (open, done) = docs();
        let page = "# P\n\n| milestone | covers | status |\n|---|---|---|\n\
                    | <a id=\"m1-2\"></a>[M1.2][phase-1] | The air | not yet done |\n\
                    | <a id=\"m0-5\"></a>[M0.5][phase-0] | Lean words | done |\n\nAfter.\n";
        assert_eq!(
            milestone_table(page, &open, &done).unwrap(),
            "# P\n\n| milestone | covers | status |\n|---|---|---|\n\
             | <a id=\"m0-1\"></a>[M0.1][done-0] | Workspace, CI | done |\n\
             | <a id=\"m0-5\"></a>[M0.5][phase-0] | Lean words | not yet done |\n\
             | <a id=\"m0-5a\"></a>[M0.5a][done-0] | Files | done |\n\
             | <a id=\"m0-5i\"></a>[M0.5i][phase-0] | Report | blocked |\n\
             | <a id=\"m1-2\"></a>[M1.2][done-1] | The air | done |\n\
             | <a id=\"m1-14\"></a>[M1.14][phase-1] | Accuracy | not yet done |\n\nAfter.\n"
        );
        let unknown = page.replace("[M1.2][phase-1]", "[M9.9][phase-1]");
        assert!(
            milestone_table(&unknown, &open, &done)
                .unwrap_err()
                .contains("M9.9")
        );
        let broken = page.replace("\n| <a id=\"m0-5\"", "\nA stray line.\n| <a id=\"m0-5\"");
        assert!(
            milestone_table(&broken, &open, &done)
                .unwrap_err()
                .contains("broken")
        );
        assert!(
            milestone_table("# P\n", &open, &done)
                .unwrap_err()
                .contains("no table of milestones")
        );
        // An entry in both files is refused.
        let twice = roadmap::parse("# R\n## Phase 1: Physics\n- [ ] **M1.2 Again.**\n").unwrap();
        assert!(
            milestone_table(page, &twice, &done)
                .unwrap_err()
                .contains("twice")
        );
        // Also when the two files put it under different phases.
        let elsewhere = roadmap::parse("# R\n## Phase 2: Other\n- [ ] **M1.2 Again.**\n").unwrap();
        assert!(
            milestone_table(page, &elsewhere, &done)
                .unwrap_err()
                .contains("twice")
        );
    }

    #[test]
    fn phase_links_name_each_file() {
        let (open, done) = docs();
        let page = "x\n[phase-9]: old\n[done-1]: old\n[other]: y\n";
        assert_eq!(
            phase_definitions(page, &open, &done).unwrap(),
            "x\n[phase-0]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md#phase-0-foundations\n\
             [phase-1]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md#phase-1-physics\n\
             [done-0]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/roadmap-done.md#phase-0-foundations\n\
             [done-1]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/roadmap-done.md#phase-1-physics\n\
             [other]: y\n"
        );
        assert!(phase_definitions("x\n", &open, &done).is_err());
    }

    /// The committed index and records page are what `cargo xtask records` writes.
    #[test]
    fn the_generated_records_are_current() {
        let stale = write_all(&root(), true).unwrap();
        assert!(
            stale.is_empty(),
            "out of date: {}; run `cargo xtask records`",
            stale.join(", ")
        );
    }
}
