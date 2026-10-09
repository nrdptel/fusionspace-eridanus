//! The design audit (ADR-205 §2; M0.9b), checked by `cargo test -p xtask`.
//!
//! [`TABLE`] has a row for every `##` section of the 14 files under the FusionSpace product
//! system's `product/` folder, at the commit it names. Each row says which surfaces that ship
//! today the section governs, and how it stands:
//!
//! - **met**, held by a named live test (`path/to/file.rs::test_name`), or "reviewed at" the
//!   pinned commit with what was read;
//! - **not met**, with the issue (`#123`) and the open milestone that will meet it;
//! - **later**, for a surface that doesn't ship yet (an app, a watch, firmware, hardware), with
//!   the open milestone that will apply it;
//! - **n/a**, for a section that governs nothing the project ships or will ship, with the reason.
//!
//! The checks:
//!
//! - the table's commit is `validation/refs.lock.toml`'s pin and starts with the theme's
//!   [`DESIGN_REV`], so moving the pin reopens the audit;
//! - the table's rows are [`SECTIONS`]' list, in order, each once;
//! - each row's status is one of the four, with what holds it: a named test must be a live
//!   `#[test]` in the file it names, a review must name a prefix of the pinned commit, a milestone
//!   must be open in `docs/ROADMAP.md`;
//! - where `refs/fusionspace-design` is checked out (the local gate), [`SECTIONS`] is compared
//!   with the `##` headings of the 14 files at the pinned commit, and the folder must hold those 14
//!   files and no other. CI has no `refs/`, and the test says it skipped this.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::roadmap::{Kind, checkbox};
use crate::site::theme::{DESIGN_REFS, DESIGN_REV, REFS_LOCK, committed_file, locked_commit};

/// The audit table, relative to the workspace root.
const TABLE: &str = "docs/research/design-conformance.md";
/// The committed list of the sections the table must row, one `file.md: heading` per line.
const SECTIONS: &str = "docs/research/design-sections.txt";
/// The roadmap, whose open milestones the table may name.
const ROADMAP: &str = "docs/ROADMAP.md";
/// The product system's folder of rules, in its repository.
const PRODUCT: &str = "product";
/// The 14 files under [`PRODUCT`] that the audit covers: every Markdown file there.
const FILES: [&str; 14] = [
    "README.md",
    "principles.md",
    "foundations.md",
    "writing.md",
    "review.md",
    "data.md",
    "cli.md",
    "web.md",
    "desktop.md",
    "mobile.md",
    "watch.md",
    "embedded.md",
    "hardware.md",
    "rockets.md",
];
/// The surfaces that ship today, as the "Applies to" column names them.
const SURFACES: [&str; 6] = ["site", "CLI", "exports", "plot", "README", "banners"];
/// The "Applies to" cell of a row that governs no surface shipping today.
const NONE: &str = "none";
/// The table's header row.
const HEADER: &str = "| File | Section | Applies to | Status | Held by |";
/// What introduces the pinned commit in the table.
const PINNED: &str = "Pinned commit: `";
/// What opens the "Held by" cell of a row met by review.
const REVIEWED: &str = "reviewed at `";
/// The shortest commit prefix a review may name.
const MIN_PREFIX: usize = 7;

/// One row of the table.
#[derive(Debug)]
struct Row<'a> {
    line: usize,
    file: &'a str,
    section: &'a str,
    applies: &'a str,
    status: &'a str,
    held_by: &'a str,
}

/// The `##` headings of a Markdown file, as text, outside code blocks.
fn headings(markdown: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => current = Some(String::new()),
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => out.extend(current.take()),
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = current.as_mut() {
                    heading.push_str(&text);
                }
            }
            _ => {}
        }
    }
    out
}

/// The sections [`SECTIONS`] lists, as `(file, heading)` pairs. Blank lines and lines opening
/// with `#` are notes.
fn listed(sections: &str) -> Result<Vec<(String, String)>, String> {
    sections
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|(n, line)| {
            line.split_once(": ")
                .map(|(file, heading)| (file.to_owned(), heading.to_owned()))
                .ok_or_else(|| format!("{SECTIONS}:{}: not `file.md: heading`", n + 1))
        })
        .collect()
}

/// The cells of a table row, trimmed, without the outer pipes.
fn cells(line: &str) -> Vec<&str> {
    let inner = line.trim();
    let inner = inner.strip_prefix('|').unwrap_or(inner);
    let inner = inner.strip_suffix('|').unwrap_or(inner);
    inner.split('|').map(str::trim).collect()
}

/// The commit the table names, and its rows.
fn parse(table: &str) -> Result<(String, Vec<Row<'_>>), Vec<String>> {
    let mut problems = Vec::new();
    let commits: Vec<&str> = table
        .lines()
        .filter_map(|line| line.strip_prefix(PINNED)?.split('`').next())
        .collect();
    let commit = match commits.as_slice() {
        [one] if one.len() == 40 && one.bytes().all(|b| b.is_ascii_hexdigit()) => (*one).to_owned(),
        [one] => {
            problems.push(format!("{TABLE}: `{one}` is not a full commit"));
            String::new()
        }
        _ => {
            problems.push(format!(
                "{TABLE}: name the pinned commit once, on a line opening with {PINNED}…`"
            ));
            String::new()
        }
    };
    let mut rows = Vec::new();
    let mut in_table = false;
    for (n, line) in table.lines().enumerate() {
        if line == HEADER {
            in_table = true;
            continue;
        }
        if !line.starts_with('|') {
            in_table = false;
            continue;
        }
        let row = cells(line);
        if !in_table
            || row
                .iter()
                .all(|cell| cell.chars().all(|c| c == '-' || c == ':'))
        {
            continue;
        }
        let [file, section, applies, status, held_by] = row.as_slice() else {
            problems.push(format!("{TABLE}:{}: a row has five cells", n + 1));
            continue;
        };
        let file = file
            .strip_prefix('`')
            .and_then(|f| f.strip_suffix('`'))
            .unwrap_or(file);
        rows.push(Row {
            line: n + 1,
            file,
            section,
            applies,
            status,
            held_by,
        });
    }
    if problems.is_empty() {
        Ok((commit, rows))
    } else {
        Err(problems)
    }
}

/// The milestone ids of a cell: each `M` followed by an id, such as `M0.7a` or `M9.1`.
fn milestones(cell: &str) -> Vec<String> {
    cell.char_indices()
        .filter(|&(at, c)| {
            c == 'M' && !cell[..at].ends_with(|p: char| p.is_ascii_alphanumeric() || p == '-')
        })
        .filter_map(|(at, _)| crate::roadmap::milestone_id(&cell[at..]))
        .collect()
}

/// The open milestones of the roadmap.
fn open_milestones(roadmap: &str) -> BTreeSet<String> {
    roadmap
        .lines()
        .filter_map(checkbox)
        .filter(|(_, _, kind)| *kind == Kind::Open)
        .map(|(_, id, _)| id)
        .collect()
}

/// The tests a cell names: each code span `path/to/file.rs::test_name`.
fn named_tests(cell: &str) -> Vec<(&str, &str)> {
    cell.split('`')
        .skip(1)
        .step_by(2)
        .filter_map(|span| span.split_once(".rs::"))
        .collect()
}

/// Whether a cell names an issue, `#123`.
fn names_issue(cell: &str) -> bool {
    cell.match_indices('#').any(|(at, _)| {
        cell[at + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
    })
}

/// Checks the table against the section list, the lock's commit and the roadmap.
/// `test_exists(file, name)` answers whether `file` (relative to the workspace root, without
/// `.rs`) defines `name` as a live test.
fn check(
    table: &str,
    sections: &str,
    locked: &str,
    open: &BTreeSet<String>,
    test_exists: &dyn Fn(&str, &str) -> bool,
) -> Vec<String> {
    let (commit, rows) = match parse(table) {
        Ok(parsed) => parsed,
        Err(problems) => return problems,
    };
    let mut problems = Vec::new();
    if commit != locked {
        problems.push(format!(
            "{TABLE} audits {commit}, and {REFS_LOCK} pins {locked}: audit every section again \
             at the new commit, then name it"
        ));
    }
    if !commit.starts_with(DESIGN_REV) {
        problems.push(format!(
            "{TABLE} audits {commit}, and the theme was copied from {DESIGN_REV}"
        ));
    }
    let listed = match listed(sections) {
        Ok(listed) => listed,
        Err(problem) => return [problems, vec![problem]].concat(),
    };
    let have: Vec<(String, String)> = rows
        .iter()
        .map(|row| (row.file.to_owned(), row.section.to_owned()))
        .collect();
    let wanted: BTreeSet<&(String, String)> = listed.iter().collect();
    let mut seen = BTreeSet::new();
    for (row, key) in rows.iter().zip(&have) {
        if !wanted.contains(key) {
            problems.push(format!(
                "{TABLE}:{}: `{}` has no section `{}` in {SECTIONS}",
                row.line, row.file, row.section
            ));
        } else if !seen.insert(key) {
            problems.push(format!(
                "{TABLE}:{}: `{}` `{}` has a row already",
                row.line, row.file, row.section
            ));
        }
    }
    for key in &listed {
        if !seen.contains(key) {
            problems.push(format!(
                "{TABLE}: no row for `{}` `{}` ({SECTIONS})",
                key.0, key.1
            ));
        }
    }
    let order: Vec<&(String, String)> = have.iter().filter(|key| wanted.contains(key)).collect();
    let listed_order: Vec<&(String, String)> =
        listed.iter().filter(|key| seen.contains(key)).collect();
    if problems.is_empty() && order != listed_order {
        problems.push(format!("{TABLE}: the rows are not in {SECTIONS}' order"));
    }
    for row in &rows {
        problems.extend(row_problems(row, &commit, open, test_exists));
    }
    problems
}

/// What is wrong with one row's "Applies to", status and "Held by".
fn row_problems(
    row: &Row<'_>,
    commit: &str,
    open: &BTreeSet<String>,
    test_exists: &dyn Fn(&str, &str) -> bool,
) -> Vec<String> {
    let at = format!("{TABLE}:{}", row.line);
    let mut problems = Vec::new();
    let surfaces: Vec<&str> = if row.applies == NONE {
        Vec::new()
    } else {
        row.applies.split(", ").collect()
    };
    for surface in &surfaces {
        if !SURFACES.contains(surface) {
            problems.push(format!(
                "{at}: `{surface}` is not a surface that ships ({}), nor `{NONE}`",
                SURFACES.join(", ")
            ));
        }
    }
    let ships = !surfaces.is_empty();
    let milestones = milestones(row.held_by);
    for id in &milestones {
        if !open.contains(id) {
            problems.push(format!("{at}: {id} is not an open milestone in {ROADMAP}"));
        }
    }
    let tests = named_tests(row.held_by);
    for (file, name) in &tests {
        if !test_exists(file, name) {
            problems.push(format!("{at}: `{file}.rs` defines no live test `{name}`"));
        }
    }
    match row.status {
        "met" => {
            if !ships {
                problems.push(format!("{at}: met, on no surface that ships"));
            }
            let reviewed = row.held_by.strip_prefix(REVIEWED).map(|rest| {
                let prefix = rest.split('`').next().unwrap_or_default();
                prefix.len() >= MIN_PREFIX && commit.starts_with(prefix)
            });
            match reviewed {
                Some(true) => {}
                Some(false) => problems.push(format!(
                    "{at}: a review names the pinned commit, at least {MIN_PREFIX} characters"
                )),
                None if tests.is_empty() => problems.push(format!(
                    "{at}: met needs a named test (`path/to/file.rs::test`) or {REVIEWED}…`"
                )),
                None => {}
            }
        }
        "not met" => {
            if !ships {
                problems.push(format!("{at}: not met, on no surface that ships"));
            }
            if !names_issue(row.held_by) || milestones.is_empty() {
                problems.push(format!(
                    "{at}: not met needs its issue (`#123`) and the milestone that will meet it"
                ));
            }
        }
        "later" => {
            if ships {
                problems.push(format!(
                    "{at}: later governs no surface that ships: `{NONE}`, or a status for today's"
                ));
            }
            if milestones.is_empty() {
                problems.push(format!(
                    "{at}: later needs the milestone that will apply it"
                ));
            }
        }
        "n/a" => {
            if ships {
                problems.push(format!("{at}: n/a governs no surface: `{NONE}`"));
            }
            if row.held_by.split_whitespace().count() < 3 {
                problems.push(format!("{at}: n/a needs its reason"));
            }
        }
        other => problems.push(format!(
            "{at}: status `{other}` is not met, not met, later or n/a"
        )),
    }
    problems
}

/// Compares the section list with the files under [`PRODUCT`] at `commit`: `files` are the
/// Markdown files there, `text(file)` reads one.
fn check_refs(
    sections: &str,
    files: &[String],
    text: &dyn Fn(&str) -> Result<String, String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    let expected: BTreeSet<&str> = FILES.into_iter().collect();
    let found: BTreeSet<&str> = files.iter().map(String::as_str).collect();
    for file in found.difference(&expected) {
        problems.push(format!(
            "{PRODUCT}/{file} is not audited: add it to the audit's files and its sections"
        ));
    }
    for file in expected.difference(&found) {
        problems.push(format!("{PRODUCT}/{file} is not at the pinned commit"));
    }
    let listed = match listed(sections) {
        Ok(listed) => listed,
        Err(problem) => return [problems, vec![problem]].concat(),
    };
    let mut by_file: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for (file, heading) in &listed {
        by_file.entry(file).or_default().push(heading.clone());
    }
    for file in FILES {
        if !found.contains(file) {
            continue;
        }
        let actual = match text(file) {
            Ok(markdown) => headings(&markdown),
            Err(why) => {
                problems.push(format!("could not read {PRODUCT}/{file}: {why}"));
                continue;
            }
        };
        let committed = by_file.remove(file).unwrap_or_default();
        if actual != committed {
            problems.push(format!(
                "{SECTIONS} lists {committed:?} for {file}, and the pinned commit has {actual:?}"
            ));
        }
    }
    for file in by_file.keys() {
        problems.push(format!("{SECTIONS} lists {file}, which is not audited"));
    }
    problems
}

/// The Markdown files under [`PRODUCT`] at `commit` in the checkout at `design`.
fn product_files(design: &Path, commit: &str) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(design)
        .args(["ls-tree", "--name-only", commit, &format!("{PRODUCT}/")])
        .output()
        .map_err(|err| format!("could not run git: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|path| path.strip_prefix(&format!("{PRODUCT}/")))
        .filter(|name| name.ends_with(".md"))
        .map(str::to_owned)
        .collect())
}

/// A file of the workspace, as text.
fn read(root: &Path, path: &str) -> String {
    // A missing committed file is the test's failure, and the message names it.
    fs::read_to_string(root.join(path)).unwrap_or_else(|err| panic!("could not read {path}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMIT: &str = "f45454f44669cc049222f2ab19eb648796e56a52";

    fn table(rows: &str) -> String {
        format!("# Audit\n\n{PINNED}{COMMIT}`.\n\n{HEADER}\n|---|---|---|---|---|\n{rows}")
    }

    const SECTIONS_OK: &str = "# note\ncli.md: Output\ncli.md: Color\nwatch.md: Screens\n";

    fn rows_ok() -> String {
        "| `cli.md` | Output | CLI | met | `xtask/src/cli.rs::help_fits` |\n\
         | `cli.md` | Color | CLI, site | not met | #401, M0.7a: the hue is off |\n\
         | `watch.md` | Screens | none | later | M9.4 |\n"
            .to_owned()
    }

    fn open() -> BTreeSet<String> {
        ["M0.7a", "M9.4"].into_iter().map(str::to_owned).collect()
    }

    fn run(table: &str, sections: &str, locked: &str) -> Vec<String> {
        check(table, sections, locked, &open(), &|file, name| {
            file == "xtask/src/cli" && name == "help_fits"
        })
    }

    #[test]
    fn a_complete_table_passes() {
        assert_eq!(
            run(&table(&rows_ok()), SECTIONS_OK, COMMIT),
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_missing_section_fails() {
        let rows = rows_ok().replace("| `watch.md` | Screens | none | later | M9.4 |\n", "");
        let problems = run(&table(&rows), SECTIONS_OK, COMMIT);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("no row for `watch.md` `Screens`"));
    }

    #[test]
    fn an_unlisted_duplicate_or_reordered_row_fails() {
        let extra = format!(
            "{}| `cli.md` | Banner | CLI | n/a | not shipped at all |\n",
            rows_ok()
        );
        assert!(run(&table(&extra), SECTIONS_OK, COMMIT)[0].contains("has no section `Banner`"));
        let twice = format!(
            "{}| `watch.md` | Screens | none | later | M9.4 |\n",
            rows_ok()
        );
        assert!(run(&table(&twice), SECTIONS_OK, COMMIT)[0].contains("has a row already"));
        let ok = rows_ok();
        let lines: Vec<&str> = ok.lines().collect();
        let swapped = format!("{}\n{}\n{}\n", lines[1], lines[0], lines[2]);
        assert!(run(&table(&swapped), SECTIONS_OK, COMMIT)[0].contains("not in"));
    }

    #[test]
    fn a_commit_other_than_the_lock_or_the_theme_fails() {
        let moved = "0123456789012345678901234567890123456789";
        let problems = run(&table(&rows_ok()), SECTIONS_OK, moved);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("audit every section again"));
        let both = table(&rows_ok()).replace(COMMIT, moved);
        let problems = run(&both, SECTIONS_OK, moved);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("the theme was copied from"));
        let short = table(&rows_ok()).replace(COMMIT, "f45454f");
        assert!(run(&short, SECTIONS_OK, COMMIT)[0].contains("not a full commit"));
    }

    #[test]
    fn each_status_needs_what_holds_it() {
        let cases = [
            (
                "| `cli.md` | Output | CLI | met | `xtask/src/cli.rs::gone` |",
                "defines no live test `gone`",
            ),
            (
                "| `cli.md` | Output | CLI | met | looked at it |",
                "met needs a named test",
            ),
            (
                "| `cli.md` | Output | CLI | met | reviewed at `0123456`: fine |",
                "a review names the pinned commit",
            ),
            (
                "| `cli.md` | Output | CLI | met | reviewed at `f454`: fine |",
                "a review names the pinned commit",
            ),
            (
                "| `cli.md` | Output | none | met | reviewed at `f45454f`: fine |",
                "met, on no surface",
            ),
            (
                "| `cli.md` | Output | CLI | not met | M0.7a: off |",
                "not met needs its issue",
            ),
            (
                "| `cli.md` | Output | CLI | not met | #12: off |",
                "not met needs its issue",
            ),
            (
                "| `cli.md` | Output | CLI | not met | #12, M0.6: off |",
                "M0.6 is not an open milestone",
            ),
            (
                "| `cli.md` | Output | CLI | later | M9.4 |",
                "later governs no surface that ships",
            ),
            (
                "| `cli.md` | Output | none | later | an app |",
                "later needs the milestone",
            ),
            (
                "| `cli.md` | Output | none | n/a | none |",
                "n/a needs its reason",
            ),
            (
                "| `cli.md` | Output | site | n/a | the company's site |",
                "n/a governs no surface",
            ),
            (
                "| `cli.md` | Output | app | later | M9.4 |",
                "`app` is not a surface that ships",
            ),
            ("| `cli.md` | Output | CLI | done | M9.4 |", "status `done`"),
            ("| `cli.md` | Output | CLI | met |", "a row has five cells"),
        ];
        let first = rows_ok().lines().next().unwrap().to_owned();
        for (row, expected) in cases {
            let rows = rows_ok().replace(&first, row);
            let problems = run(&table(&rows), SECTIONS_OK, COMMIT);
            assert!(
                problems.iter().any(|p| p.contains(expected)),
                "{row}: wanted `{expected}`, got {problems:?}"
            );
        }
        let reviewed = rows_ok().replace(
            &first,
            "| `cli.md` | Output | CLI | met | reviewed at `f45454f44`: the help |",
        );
        assert_eq!(
            run(&table(&reviewed), SECTIONS_OK, COMMIT),
            Vec::<String>::new()
        );
    }

    #[test]
    fn headings_are_read_outside_code_blocks() {
        let markdown =
            "# Title\n\n## One `code`\n\ntext\n\n```\n## not\n```\n\n### Three\n\n## Two\n";
        assert_eq!(headings(markdown), ["One code", "Two"]);
    }

    #[test]
    fn milestone_ids_are_read_as_words() {
        assert_eq!(
            milestones("M0.7a, then M9.1; not HM1.2 or xM2.1"),
            ["M0.7a", "M9.1"]
        );
    }

    #[test]
    fn the_section_list_must_match_the_pinned_files() {
        let files: Vec<String> = FILES.iter().map(|f| (*f).to_owned()).collect();
        let text = |file: &str| -> Result<String, String> {
            Ok(match file {
                "cli.md" => "# CLI\n## Output\n## Color\n".to_owned(),
                "watch.md" => "## Screens\n".to_owned(),
                _ => "# Empty\n".to_owned(),
            })
        };
        assert_eq!(check_refs(SECTIONS_OK, &files, &text), Vec::<String>::new());
        let renamed = |file: &str| -> Result<String, String> {
            if file == "cli.md" {
                Ok("## Output\n## Hue\n".to_owned())
            } else {
                text(file)
            }
        };
        assert!(check_refs(SECTIONS_OK, &files, &renamed)[0].contains("for cli.md"));
        let mut more = files.clone();
        more.push("print.md".to_owned());
        assert!(check_refs(SECTIONS_OK, &more, &text)[0].contains("print.md is not audited"));
        assert!(check_refs(SECTIONS_OK, &files[1..], &text)[0].contains("README.md is not at"));
    }

    #[test]
    fn the_committed_audit_passes() {
        let root = crate::designs::root().unwrap();
        let locked = locked_commit(&read(&root, REFS_LOCK)).unwrap();
        let sections = read(&root, SECTIONS);
        let open = open_milestones(&read(&root, ROADMAP));
        let test_exists = |file: &str, name: &str| {
            fs::read_to_string(root.join(format!("{file}.rs")))
                .is_ok_and(|src| crate::docs::defines_test(&src, name))
        };
        let problems = check(&read(&root, TABLE), &sections, &locked, &open, &test_exists);
        assert_eq!(problems, Vec::<String>::new());

        let design = root.join(DESIGN_REFS);
        if !design.is_dir() {
            eprintln!(
                "skipped: no {DESIGN_REFS} here (CI), so {SECTIONS} was not compared with the \
                 pinned files"
            );
            return;
        }
        let files = product_files(&design, &locked).unwrap();
        let text = |file: &str| {
            committed_file(&design, &locked, &format!("{PRODUCT}/{file}"))
                .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        };
        assert_eq!(check_refs(&sections, &files, &text), Vec::<String>::new());
    }
}
