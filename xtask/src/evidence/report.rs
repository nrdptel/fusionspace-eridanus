//! The register's report, which `docs/evidence.md` holds between [`REGION_START`] and
//! [`REGION_END`]: the rows counted by kind and surface, with no combined score; the rows short
//! of their wanted kind; every row; the rows due a search; and the searches so far, each with
//! the places it looked as a list and what it found as a paragraph. No sentence it writes runs
//! over 25 words, whatever the counts, so the page holds to the writing guide's voice.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::{Kind, LABEL, PAGE, REPOSITORY, Register, Row, day_number, prose_date};

/// The comment that opens the region `cargo xtask evidence` writes, and the one that ends it.
pub(crate) const REGION_START: &str = "<!-- evidence: report; written by `cargo xtask evidence` from validation/evidence.toml; do not edit -->";
pub(crate) const REGION_END: &str = "<!-- evidence: end -->";

/// A row is due a search when its last one is more than this many days before the register's
/// `as_of` (ADR-278 §3): a search exactly this many days old is not due yet.
pub(crate) const DUE_AFTER_DAYS: i64 = 30;

/// The report, markers included, for `register`, whose decision records' file names
/// `decisions` gives by number. The register is held to its rules first
/// ([`super::structure`]); this refuses only what it can't write.
pub(crate) fn region(
    register: &Register,
    decisions: &BTreeMap<u32, String>,
) -> Result<String, String> {
    let rows = &register.row;
    let as_of = prose_date(&register.as_of)?;
    let mut out = format!("{REGION_START}\n\n");
    if rows.is_empty() {
        let _ = writeln!(out, "The register holds no rows yet, as of {as_of}.\n");
        out.push_str(REGION_END);
        return Ok(out);
    }
    let _ = writeln!(
        out,
        "The register holds {}, as of {as_of}.\n",
        counted(rows.len(), "row", "rows")
    );

    // By kind: one column per kind any row has, one line per surface, then the totals.
    let kinds: Vec<Kind> = (Kind::ALL.into_iter())
        .filter(|kind| rows.iter().any(|row| row.kind == *kind))
        .collect();
    let mut surfaces: Vec<&str> = Vec::new();
    for row in rows {
        if !surfaces.contains(&row.surface.as_str()) {
            surfaces.push(&row.surface);
        }
    }
    out.push_str(
        "**By kind.** Each surface's rows, counted by the kind of evidence behind them now:\n\n",
    );
    // Each head is a label and what it counts (data.md, *Tables*).
    let heads: Vec<String> = (kinds.iter())
        .map(|kind| format!("{} · rows", capitalized(kind.words())))
        .collect();
    let _ = writeln!(out, "| Surface | {} | Rows |", heads.join(" | "));
    let _ = writeln!(out, "|---|{}---:|", "---:|".repeat(kinds.len()));
    let line = |label: &str, of: &dyn Fn(&Row) -> bool| {
        let counts: Vec<String> = (kinds.iter())
            .map(|kind| {
                let count = rows.iter().filter(|row| of(row) && row.kind == *kind);
                count.count().to_string()
            })
            .collect();
        let total = rows.iter().filter(|row| of(row)).count();
        format!("| {} | {} | {total} |\n", cell(label), counts.join(" | "))
    };
    for surface in &surfaces {
        out.push_str(&line(surface, &|row: &Row| row.surface == *surface));
    }
    out.push_str(&line("All", &|_: &Row| true));

    // Short of what's wanted: the check holds each to an open issue.
    let short = rows.iter().filter(|row| row.kind != row.wanted).count();
    out.push_str("\n**Short of what's wanted.** ");
    match short {
        0 => out.push_str("Every row has the kind of evidence it wants.\n"),
        1 if rows.len() == 1 => {
            let _ = writeln!(
                out,
                "Its one row doesn't yet have the kind of evidence it wants. It has an open issue \
                 labeled `{LABEL}`, linked in its row below."
            );
        }
        1 => {
            let _ = writeln!(
                out,
                "1 of the {} doesn't yet have the kind of evidence it wants. It has an open issue \
                 labeled `{LABEL}`, linked in its row below.",
                counted(rows.len(), "row", "rows")
            );
        }
        short => {
            let _ = writeln!(
                out,
                "{short} of the {} rows don't yet have the kind of evidence they want. Each has \
                 an open issue labeled `{LABEL}`, linked in its row below.",
                rows.len()
            );
        }
    }

    // The rows.
    out.push_str("\n**The rows.**\n\n");
    out.push_str("| Row | Surface | Claim | Evidence | Wanted | Issue | Last search |\n");
    out.push_str("|---|---|---|---|---|---|---|\n");
    for row in rows {
        let mut claim = sentence(&row.claim);
        if let Some(not_checked) = row.not_checked.as_deref().filter(|t| !t.trim().is_empty()) {
            let _ = write!(claim, " **Not checked:** {}", sentence(not_checked));
        }
        let wanted = match &row.wanted_adr {
            Some(adr) => format!("{} ({})", row.wanted.words(), adr_link(adr, decisions)?),
            None => row.wanted.words().to_owned(),
        };
        let issue = match row.issue {
            Some(number) => {
                format!("[#{number}](https://github.com/{REPOSITORY}/issues/{number})")
            }
            None => "none".to_owned(),
        };
        let last = row
            .searched
            .last()
            .map_or_else(|| "none".to_owned(), |search| search.on.clone());
        let _ = writeln!(
            out,
            "| <a id=\"{}\"></a>{} | {} | {} | {} | {wanted} | {issue} | {} |",
            row.id.to_ascii_lowercase(),
            row.id,
            cell(&row.surface),
            cell(&claim),
            row.kind.words(),
            cell(&last),
        );
    }

    // Due a search: a list, so no sentence grows with the number of rows due.
    let due = due(register)?;
    let _ = write!(
        out,
        "\n**Due a search.** A row is due one when it has none yet, or its last was more than \
         {DUE_AFTER_DAYS} days before {as_of}:"
    );
    if due.is_empty() {
        out.push_str(" none.\n");
    } else {
        out.push_str("\n\n");
        for row in due {
            let _ = writeln!(out, "- {}", anchor_link(row));
        }
    }

    // Searches so far: each a list item holding its places, one nested item each, and then
    // what it found, a paragraph of its own.
    if rows.iter().any(|row| !row.searched.is_empty()) {
        out.push_str("\n**Searches so far.**\n\n");
        for row in rows {
            for search in &row.searched {
                let _ = writeln!(
                    out,
                    "- {}, {}, looked at:",
                    anchor_link(row),
                    prose_date(&search.on)?
                );
                for place in &search.looked {
                    let _ = writeln!(out, "  - {}", words(place));
                }
                let _ = writeln!(out, "\n  It found {}", sentence(&search.found));
            }
        }
    }
    out.push('\n');
    out.push_str(REGION_END);
    Ok(out)
}

/// The rows due a search: none yet, or the last more than [`DUE_AFTER_DAYS`] before `as_of`.
pub(crate) fn due(register: &Register) -> Result<Vec<&Row>, String> {
    let as_of = day_number(&register.as_of)
        .ok_or_else(|| format!("as_of `{}` isn't a date", register.as_of))?;
    let mut due = Vec::new();
    for row in &register.row {
        let Some(last) = row.searched.last() else {
            due.push(row);
            continue;
        };
        let day = day_number(&last.on)
            .ok_or_else(|| format!("{}: `{}` isn't a date", row.id, last.on))?;
        if as_of - day > DUE_AFTER_DAYS {
            due.push(row);
        }
    }
    Ok(due)
}

/// A link to a row's anchor in the report: `[G2](#g2)`.
fn anchor_link(row: &Row) -> String {
    format!("[{}](#{})", row.id, row.id.to_ascii_lowercase())
}

/// A decision record's label linked to its file on GitHub, which the site links a record by:
/// `[ADR-145](https://github.com/.../docs/decisions/0145-....md)`.
fn adr_link(adr: &str, decisions: &BTreeMap<u32, String>) -> Result<String, String> {
    let file = (adr.strip_prefix("ADR-"))
        .and_then(|digits| digits.parse::<u32>().ok())
        .and_then(|number| decisions.get(&number))
        .ok_or_else(|| format!("{adr} names no decision record"))?;
    Ok(format!(
        "[{adr}](https://github.com/{REPOSITORY}/blob/main/docs/decisions/{file})"
    ))
}

/// Text for a table's cell: one line, each `|` escaped so it doesn't end the cell.
fn cell(text: &str) -> String {
    words(text).replace('|', "\\|")
}

/// Text with each run of white space, line breaks included, one space.
fn words(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Text as a sentence: one line, ending in a full stop unless it ends in one already, or in a
/// question or an exclamation mark.
fn sentence(text: &str) -> String {
    let text = words(text);
    if text.ends_with(['.', '?', '!']) {
        text
    } else {
        format!("{text}.")
    }
}

/// `words` with its first letter a capital: "Code to code".
fn capitalized(words: &str) -> String {
    let mut chars = words.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

/// A count with its noun: "1 row", "24 rows".
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// The page's report as it stands, markers included, or why it can't be found: each marker must
/// stand once on the page, on a line of its own, the start before the end.
pub(crate) fn committed_region(page: &str) -> Result<&str, String> {
    for marker in [REGION_START, REGION_END] {
        let count = page.matches(marker).count();
        let alone = page.lines().filter(|line| line.trim() == marker).count();
        if count != 1 || alone != 1 {
            return Err(format!(
                "{PAGE} must hold `{marker}` once, on a line of its own; it holds it {count} \
                 times, {alone} on a line of its own"
            ));
        }
    }
    let start = page.find(REGION_START).unwrap_or_default();
    let end = page.find(REGION_END).unwrap_or_default();
    if end < start {
        return Err(format!("{PAGE}'s report ends before it starts"));
    }
    Ok(&page[start..end + REGION_END.len()])
}
