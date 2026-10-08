//! The roadmap and its archive (M0.5b, ADR-147 in `docs/decisions/`).
//!
//! `docs/ROADMAP.md` holds open work only. A milestone checked off there is moved, its lines
//! unchanged, to `docs/roadmap-done.md` by `cargo xtask records`, so finishing a milestone is one
//! edit (the box) and one command. Both files share the roadmap's shape:
//!
//! - a head, everything above the first `## Phase` heading (the roadmap's rules and queue; the
//!   archive's is generated, with an index of one line per milestone);
//! - phases, each a `## Phase N: ...` heading, the lines before its first entry, and its entries.
//!
//! An entry is a milestone's checkbox line, `- [x] **M1.2 Title.**`, at any indent, and every line
//! after it up to the next entry or heading. An increment sits two spaces deeper than its parent.
//! When an open increment's parent is done, or a done increment's parent is open, the other file
//! holds a one-line stub for the parent so the nesting reads the same:
//!
//! ```text
//! - **M6.2** is open; its entry: [roadmap](ROADMAP.md#phase-3-uncertainty-optimization-challenges).
//! ```
//!
//! A stub has no checkbox, so nothing reads it as a milestone. Moving never edits an entry's lines:
//! an entry is cut from one file and pasted into the other whole, which
//! `tests::moving_keeps_every_entry_byte_for_byte` holds to.

use std::fmt::Write as _;

/// The roadmap, relative to the workspace root.
pub const ROADMAP: &str = "docs/ROADMAP.md";
/// The archive of done milestones, relative to the workspace root.
pub const ARCHIVE: &str = "docs/roadmap-done.md";

/// What the archive says above its index.
const ARCHIVE_HEAD: &str = "\
# Roadmap: done

The milestones and increments checked off in [the roadmap](ROADMAP.md), which holds open work
only. `cargo xtask records` moves each one here once its box is checked, its lines unchanged, so
every *done when* reads as it was written. Under each phase they keep the roadmap's nesting: a
done increment of a milestone that is still open sits under a one-line stub naming it. Never
edit an entry here; a change to a met milestone is a new ADR.

## Index

One line per milestone or increment, in the order it was archived within its phase.

";

/// What an entry of either file is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Open,
    Done,
    /// A one-line stand-in for a parent whose entry is in the other file.
    Stub,
}

/// One entry, or a stub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub indent: usize,
    pub id: String,
    pub kind: Kind,
    /// The entry's lines, its checkbox line first, exactly as in the file.
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase {
    /// `## Phase 1: Physics core ...`.
    pub heading: String,
    /// The lines between the heading and the first entry.
    pub intro: Vec<String>,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Doc {
    pub head: Vec<String>,
    pub phases: Vec<Phase>,
}

/// The id and status of a checkbox line, `  - [x] [blocked] **M1.2a Title.**`: `None` for any
/// other line. The id runs to the first character that can't be in one.
pub fn checkbox(line: &str) -> Option<(usize, String, Kind)> {
    let trimmed = line.trim_start_matches(' ');
    let indent = line.len() - trimmed.len();
    let rest = trimmed.strip_prefix("- [")?;
    let kind = match rest.chars().next()? {
        ' ' => Kind::Open,
        'x' | 'X' => Kind::Done,
        _ => return None,
    };
    let rest = rest.get(1..)?.strip_prefix("] ")?;
    let rest = rest.strip_prefix("[blocked] ").unwrap_or(rest);
    let id = milestone_id(rest.strip_prefix("**")?)?;
    Some((indent, id, kind))
}

/// The id of a stub line, `- **M6.2** is open; ...`.
fn stub(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim_start_matches(' ');
    let indent = line.len() - trimmed.len();
    let rest = trimmed.strip_prefix("- **")?;
    let (id, after) = rest.split_once("** is ")?;
    (milestone_id(id).as_deref() == Some(id)
        && (after.starts_with("open;") || after.starts_with("done;")))
    .then(|| (indent, id.to_owned()))
}

/// `M1.2`, `M1.10b` or `M2.1b2` at the start of `text`.
pub fn milestone_id(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    if bytes.first() != Some(&b'M') {
        return None;
    }
    let digits = |from: usize| {
        let end = from
            + bytes[from..]
                .iter()
                .take_while(|b| b.is_ascii_digit())
                .count();
        (end > from).then_some(end)
    };
    let major = digits(1)?;
    if bytes.get(major) != Some(&b'.') {
        return None;
    }
    let mut end = digits(major + 1)?;
    if bytes.get(end).is_some_and(u8::is_ascii_lowercase) {
        end += 1;
        end = digits(end).unwrap_or(end);
    }
    (!bytes.get(end).is_some_and(u8::is_ascii_alphanumeric)).then(|| text[..end].to_owned())
}

/// Whether `line` is an ATX heading: one to six `#`, then a space or the line's end. A wrapped
/// line that starts with an issue number, `#298 tracks it`, is not.
pub fn is_heading(line: &str) -> bool {
    let hashes = line.bytes().take_while(|&b| b == b'#').count();
    (1..=6).contains(&hashes) && line[hashes..].chars().next().is_none_or(|c| c == ' ')
}

/// Splits a roadmap-shaped file into its head and phases. A file the move could get wrong is
/// refused, with the line: a heading other than a phase's after the first phase (an entry would
/// swallow it); a phase heading twice; text under a stub (the next move would drop it); text under
/// an increment indented less than the increment's own text (it would belong to the parent, but
/// move with the increment); and a line the site reads as a milestone that isn't one here
/// (`* [x] **M1.2**`, two spaces after the box), which would be glued to the entry above.
pub fn parse(text: &str) -> Result<Doc, String> {
    let mut doc = Doc::default();
    for (n, line) in text.lines().enumerate() {
        let at = n + 1;
        if line.starts_with("## Phase ") {
            if doc.phases.iter().any(|p| p.heading == line) {
                return Err(format!(
                    "line {at}: the phase heading `{line}` appears twice"
                ));
            }
            doc.phases.push(Phase {
                heading: line.to_owned(),
                intro: Vec::new(),
                entries: Vec::new(),
            });
            continue;
        }
        let Some(phase) = doc.phases.last_mut() else {
            doc.head.push(line.to_owned());
            continue;
        };
        if is_heading(line) {
            return Err(format!(
                "line {at}: `{line}` is a heading after the first phase; only `## Phase` \
                 headings may follow, or the entry above would take it in"
            ));
        }
        if let Some((indent, id, kind)) = checkbox(line) {
            phase.entries.push(Entry {
                indent,
                id,
                kind,
                lines: vec![line.to_owned()],
            });
        } else if let Some((indent, id)) = stub(line) {
            phase.entries.push(Entry {
                indent,
                id,
                kind: Kind::Stub,
                lines: vec![line.to_owned()],
            });
        } else if crate::site::reads_as_milestone(line) {
            return Err(format!(
                "line {at}: `{}` reads like a milestone but isn't one: write it \
                 `- [ ] **M<topic>.<place> Title.**`, one space after the box",
                line.trim()
            ));
        } else if let Some(entry) = phase.entries.last_mut() {
            let indent = line.len() - line.trim_start_matches(' ').len();
            if !line.trim().is_empty() {
                if entry.kind == Kind::Stub {
                    return Err(format!(
                        "line {at}: text under the stub for {}; a stub stands alone, and its \
                         entry is in the other file",
                        entry.id
                    ));
                }
                if entry.indent > 0 && indent < entry.indent + 2 {
                    return Err(format!(
                        "line {at}: text under {} indented less than its own text; it would \
                         move with {} though it reads as its parent's",
                        entry.id, entry.id
                    ));
                }
            }
            entry.lines.push(line.to_owned());
        } else {
            phase.intro.push(line.to_owned());
        }
    }
    Ok(doc)
}

/// The file's text again: [`parse`] then `render` gives back the same text, bar a final newline.
pub fn render(doc: &Doc) -> String {
    let mut out = String::new();
    let lines = doc.head.iter().chain(doc.phases.iter().flat_map(|phase| {
        std::iter::once(&phase.heading)
            .chain(&phase.intro)
            .chain(phase.entries.iter().flat_map(|entry| &entry.lines))
    }));
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// The ids of `entries[i]`'s parents, outermost first, as (index into `entries`).
fn ancestors(entries: &[Entry], i: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut indent = entries[i].indent;
    for j in (0..i).rev() {
        if entries[j].indent < indent {
            out.push(j);
            indent = entries[j].indent;
        }
    }
    out.reverse();
    out
}

/// A stub for `id`, whose entry is in the other file, under `heading`'s phase there.
fn stub_line(indent: usize, id: &str, open: bool, heading: &str) -> String {
    let pad = " ".repeat(indent);
    let anchor = crate::site::slug(heading.trim_start_matches("## "));
    if open {
        format!("{pad}- **{id}** is open; its entry: [roadmap](ROADMAP.md#{anchor}).")
    } else {
        format!("{pad}- **{id}** is done; its entry: [archive](roadmap-done.md#{anchor}).")
    }
}

/// Moves every done entry of `roadmap` into `archive`, its lines unchanged, and returns both. In
/// the roadmap, an open entry keeps a stub for each done parent; in the archive, a done entry
/// sits under its parent's entry, or under a stub when the parent is still open, as the last of
/// its children. A stub in the archive is replaced by its entry once that is done too.
pub fn archive(roadmap: &Doc, archive: &Doc) -> Result<(Doc, Doc), String> {
    let mut done = archive.clone();
    let mut open = Doc {
        head: roadmap.head.clone(),
        phases: Vec::new(),
    };
    for phase in &roadmap.phases {
        let entries = &phase.entries;
        // The archive's copy of this phase, made if it has none yet, in the roadmap's order.
        let at = match done.phases.iter().position(|p| p.heading == phase.heading) {
            Some(at) => at,
            None => {
                let before = roadmap
                    .phases
                    .iter()
                    .take_while(|p| p.heading != phase.heading)
                    .filter(|p| done.phases.iter().any(|d| d.heading == p.heading))
                    .count();
                done.phases.insert(
                    before,
                    Phase {
                        heading: phase.heading.clone(),
                        intro: vec![String::new()],
                        entries: Vec::new(),
                    },
                );
                before
            }
        };
        let mut kept = Vec::new();
        for (i, entry) in entries.iter().enumerate() {
            let chain = ancestors(entries, i);
            match entry.kind {
                Kind::Stub => {}
                Kind::Open => {
                    // Its done parents stand as stubs, once each, where they were.
                    for &j in &chain {
                        let parent = &entries[j];
                        if parent.kind != Kind::Open
                            && !kept.iter().any(|e: &Entry| e.id == parent.id)
                        {
                            kept.push(Entry {
                                indent: parent.indent,
                                id: parent.id.clone(),
                                kind: Kind::Stub,
                                lines: vec![stub_line(
                                    parent.indent,
                                    &parent.id,
                                    false,
                                    &phase.heading,
                                )],
                            });
                        }
                    }
                    kept.push(entry.clone());
                }
                Kind::Done => {
                    let path: Vec<&Entry> = chain.iter().map(|&j| &entries[j]).collect();
                    place(&mut done.phases[at].entries, &path, entry, &phase.heading)?;
                }
            }
        }
        open.phases.push(Phase {
            heading: phase.heading.clone(),
            intro: phase.intro.clone(),
            entries: kept,
        });
    }
    // Stubs are the tool's own lines, so they are rewritten each time in the current form.
    for phase in &mut done.phases {
        for entry in &mut phase.entries {
            if entry.kind == Kind::Stub {
                entry.lines = vec![stub_line(entry.indent, &entry.id, true, &phase.heading)];
            }
        }
    }
    done.head = archive_head(&done);
    Ok((open, done))
}

/// Puts `entry` into the archive's `entries` under its parents `path`, making a stub for each
/// parent the archive doesn't hold yet, or replacing the stub that stands for `entry`.
fn place(
    entries: &mut Vec<Entry>,
    path: &[&Entry],
    entry: &Entry,
    heading: &str,
) -> Result<(), String> {
    // Where each parent is, outermost first; a missing one is added at the end of its own parent.
    let mut parent: Option<usize> = None;
    for ancestor in path {
        let found = find_child(entries, parent, &ancestor.id);
        let index = match found {
            Some(index) => index,
            // A done parent's entry must already be here; a stub saying "open" would be false.
            None if ancestor.kind != Kind::Open => {
                return Err(format!(
                    "{} is done, but {ARCHIVE} has no entry for it under `{heading}`, where {} \
                     would go",
                    ancestor.id, entry.id
                ));
            }
            None => {
                let at = subtree_end(entries, parent);
                entries.insert(
                    at,
                    Entry {
                        indent: ancestor.indent,
                        id: ancestor.id.clone(),
                        kind: Kind::Stub,
                        lines: vec![stub_line(ancestor.indent, &ancestor.id, true, heading)],
                    },
                );
                at
            }
        };
        parent = Some(index);
    }
    match find_child(entries, parent, &entry.id) {
        Some(index) if entries[index].kind == Kind::Stub => {
            entries[index] = entry.clone();
        }
        // Already moved, byte for byte, by a run whose roadmap write failed: finish the move.
        Some(index) if entries[index].lines == entry.lines => {}
        Some(_) => {
            return Err(format!(
                "{} is checked off in {ROADMAP} and already in {ARCHIVE}",
                entry.id
            ));
        }
        None => {
            let at = subtree_end(entries, parent);
            entries.insert(at, entry.clone());
        }
    }
    Ok(())
}

/// The index of the entry `id` among the direct children of `parent` (the phase's top level for
/// `None`).
fn find_child(entries: &[Entry], parent: Option<usize>, id: &str) -> Option<usize> {
    let (start, end) = children_span(entries, parent);
    (start..end).find(|&i| entries[i].id == id)
}

/// The span of `parent`'s descendants: from right after it to the next entry no deeper than it.
fn children_span(entries: &[Entry], parent: Option<usize>) -> (usize, usize) {
    match parent {
        None => (0, entries.len()),
        Some(p) => {
            let indent = entries[p].indent;
            let end = (p + 1..entries.len())
                .find(|&i| entries[i].indent <= indent)
                .unwrap_or(entries.len());
            (p + 1, end)
        }
    }
}

fn subtree_end(entries: &[Entry], parent: Option<usize>) -> usize {
    children_span(entries, parent).1
}

/// The archive's head: its fixed text and an index of one line per archived entry.
fn archive_head(done: &Doc) -> Vec<String> {
    let mut out = String::from(ARCHIVE_HEAD);
    for phase in &done.phases {
        let name = phase.heading.trim_start_matches("## ");
        let anchor = crate::site::slug(name);
        let short = name.split(':').next().unwrap_or(name);
        for entry in phase.entries.iter().filter(|e| e.kind == Kind::Done) {
            let _ = writeln!(
                out,
                "- {} {} ([{short}](#{anchor}))",
                entry.id,
                title(&entry.lines[0])
            );
        }
    }
    out.push('\n');
    out.lines().map(str::to_owned).collect()
}

/// The bold title on an entry's checkbox line, without its id: `Core math, frames, Earth.`.
pub fn title(line: &str) -> String {
    let Some((_, bold)) = line.split_once("**") else {
        return String::new();
    };
    let bold = bold.split_once("**").map_or(bold, |(inside, _)| inside);
    let mut words = bold.splitn(2, ' ');
    words.next();
    words
        .next()
        .unwrap_or_default()
        .trim()
        .trim_end_matches(['.', ',', ':', ';'])
        .to_owned()
}

/// The current milestone and the chain under it, or why there is none (ADR-147). The current
/// milestone is the first entry of the roadmap's `## Queue` list that is open and not blocked; when
/// the queue runs out, the first such entry in file order that has no open parent. An entry counts
/// as blocked when marked `[blocked]`, or when it has open increments and every one of them counts
/// as blocked. The chain is the current milestone followed by its first open, unblocked increment,
/// that one's, and so on (`M2.1`, `M2.1b`, `M2.1b1`): any of them names the work in progress. A
/// queue line naming an id that isn't an open entry of the roadmap is a problem too.
pub fn current_milestones(text: &str) -> Result<Vec<String>, Vec<String>> {
    let doc = parse(text).map_err(|err| vec![format!("ROADMAP.md: {err}")])?;
    let entries: Vec<&Entry> = doc.phases.iter().flat_map(|p| &p.entries).collect();
    // Direct children by indent, and whether each entry is marked blocked.
    let children = |i: usize| -> Vec<usize> {
        let indent = entries[i].indent;
        let mut out = Vec::new();
        let mut child_indent = None;
        for (j, entry) in entries.iter().enumerate().skip(i + 1) {
            if entry.indent <= indent {
                break;
            }
            let want = *child_indent.get_or_insert(entry.indent);
            if entry.indent == want {
                out.push(j);
            }
        }
        out
    };
    fn blocked(i: usize, entries: &[&Entry], children: &dyn Fn(usize) -> Vec<usize>) -> bool {
        if entries[i].lines[0].contains("] [blocked] ") {
            return true;
        }
        let open: Vec<usize> = children(i)
            .into_iter()
            .filter(|&j| entries[j].kind == Kind::Open)
            .collect();
        !open.is_empty() && open.iter().all(|&j| blocked(j, entries, children))
    }
    // An entry under a parent marked blocked is blocked too, queued or not.
    let parent_blocked = |i: usize| {
        let mut indent = entries[i].indent;
        (0..i).rev().any(|j| {
            let parent = entries[j].indent < indent;
            if parent {
                indent = entries[j].indent;
            }
            parent && entries[j].lines[0].contains("] [blocked] ")
        })
    };
    let ready = |i: usize| {
        entries[i].kind == Kind::Open && !blocked(i, &entries, &children) && !parent_blocked(i)
    };

    let mut problems = Vec::new();
    let mut queue = Vec::new();
    let mut in_queue = false;
    let mut queue_sections = 0;
    for line in &doc.head {
        if is_heading(line) {
            in_queue = line == "## Queue";
            queue_sections += usize::from(in_queue);
            continue;
        }
        let trimmed = line.trim_start();
        let list_like = ["- ", "* ", "+ "].iter().any(|m| trimmed.starts_with(m))
            || trimmed.starts_with(|c: char| c.is_ascii_digit());
        if !in_queue || !list_like {
            continue;
        }
        // A queue line is `N. M<id> words`, at the margin; anything list-like that isn't would
        // otherwise be skipped, and the queue silently followed less than it says.
        let id = (trimmed.len() == line.len())
            .then(|| line.split_once(". "))
            .flatten()
            .filter(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|(_, rest)| rest.split_whitespace().next());
        let Some(id) = id else {
            problems.push(format!(
                "the roadmap's queue has `{line}`: write each line `N. M<topic>.<place> title`"
            ));
            continue;
        };
        match entries
            .iter()
            .position(|e| e.id == id && e.kind == Kind::Open)
        {
            Some(i) => queue.push(i),
            None => problems.push(format!(
                "the roadmap's queue names {id}, which is not an open milestone of ROADMAP.md \
                 (a done one has moved to the archive: take it off the queue)"
            )),
        }
    }
    if queue_sections != 1 {
        problems.push(format!(
            "ROADMAP.md's head needs exactly one `## Queue` section before the first phase; it \
             has {queue_sections}"
        ));
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let has_open_parent = |i: usize| {
        let mut indent = entries[i].indent;
        (0..i).rev().any(|j| {
            let parent = entries[j].indent < indent;
            if parent {
                indent = entries[j].indent;
            }
            parent && entries[j].kind == Kind::Open
        })
    };
    let first = queue
        .iter()
        .copied()
        .find(|&i| ready(i))
        .or_else(|| (0..entries.len()).find(|&i| ready(i) && !has_open_parent(i)));
    let Some(mut at) = first else {
        return Err(vec![
            "ROADMAP.md has no open, unblocked milestone".to_owned(),
        ]);
    };
    let mut chain = vec![entries[at].id.clone()];
    while let Some(next) = children(at).into_iter().find(|&j| ready(j)) {
        chain.push(entries[next].id.clone());
        at = next;
    }
    Ok(chain)
}

/// The `roadmap-current` command's line in `cargo xtask help`.
pub const CURRENT_USAGE: &str =
    "  roadmap-current          Print the current milestone of docs/ROADMAP.md (the first open,
                           unblocked entry of its queue, else of the file), then, one per
                           line, the open, unblocked increments it leads to.";

/// `cargo xtask roadmap-current`: prints [`current_milestones`], one id per line, current first.
pub fn run_current(args: &[String]) -> Result<(), String> {
    if let Some(arg) = args.first() {
        return Err(format!("unexpected argument `{arg}`\n\n{CURRENT_USAGE}"));
    }
    let root = crate::designs::root()?;
    let text = std::fs::read_to_string(root.join(ROADMAP))
        .map_err(|err| format!("could not read {ROADMAP}: {err}"))?;
    let chain = current_milestones(&text).map_err(|problems| problems.join("\n"))?;
    for id in chain {
        println!("{id}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_current_milestone_follows_the_queue_then_file_order() {
        let roadmap = |queue: &str| {
            format!(
                "# Roadmap\n\n## Queue\n\n{queue}\n## Phase 0: Foundations\n\n\
                 - [ ] [blocked] **M1.1 Core.** Waiting.\n\
                 - [ ] **M1.2 Atmosphere.**\n\
                 \x20 - [ ] **M1.2b Wind.**\n\
                 \x20   - [ ] **M1.2b1 Shear.**\n\
                 \x20   - [ ] **M1.2b2 Turbulence.**\n\
                 - [ ] **M2.3 Flights.**\n\
                 \x20 - [ ] [blocked] **M2.3c Logs.**\n\
                 - **M1.8** is done; its entry is in [the archive](roadmap-done.md).\n\
                 \x20 - [ ] **M1.8e16 Tip.**\n\
                 - [ ] **M6.2 Optimization.**\n\
                 \x20 - [ ] **M6.2e Robust.**\n"
            )
        };
        // No queue: file order, skipping the blocked M1.1; the chain runs down the first open,
        // unblocked increments under M1.2, and holds nothing else.
        let plain = roadmap("");
        assert_eq!(
            current_milestones(&plain).unwrap(),
            ["M1.2", "M1.2b", "M1.2b1"]
        );
        // The queue comes first; an entry whose open increments are all blocked is skipped, and
        // an open increment under a done parent's stub can be queued.
        let queued = roadmap("1. M2.3 Flights\n2. M6.2e Robust\n3. M1.8e16 Tip\n");
        assert_eq!(current_milestones(&queued).unwrap(), ["M6.2e"]);
        let stub_first = roadmap("1. M1.8e16 Tip\n");
        assert_eq!(current_milestones(&stub_first).unwrap(), ["M1.8e16"]);
        // A queue naming a missing or done id fails, as does a numbered line naming nothing.
        for queue in ["1. M9.9 Gone\n", "1. M1.1x\n"] {
            let err = current_milestones(&roadmap(queue)).unwrap_err();
            assert!(err[0].contains("not an open milestone"), "{err:?}");
        }
        let done = plain.replace(
            "- [ ] **M6.2 Optimization.**",
            "- [x] **M6.2 Optimization.**",
        );
        assert!(
            current_milestones(&done.replace("## Queue\n\n", "## Queue\n\n1. M6.2\n")).is_err()
        );
        // Numbered lines outside the queue are not read; nothing open at all is a problem.
        let elsewhere = plain.replace("# Roadmap\n", "# Roadmap\n\n1. M9.9 a rule\n");
        assert!(current_milestones(&elsewhere).is_ok());
        let closed = "# R\n## Queue\n## Phase 0: X\n- [x] **M1.1 A.**\n";
        assert_eq!(
            current_milestones(closed).unwrap_err(),
            ["ROADMAP.md has no open, unblocked milestone"]
        );
        let decimal = "# R\n## Queue\n## Phase 0: X\n- [ ] **M1.1 A.**\n- [ ] **M1.10 B.**\n";
        assert_eq!(current_milestones(decimal).unwrap(), ["M1.1"]);
        // A queue that is missing, doubled, misspelled or written as another kind of list fails
        // rather than being skipped.
        for (from, to) in [
            ("## Queue\n", ""),
            ("## Queue\n", "## Queue \n"),
            ("## Queue\n", "## Queue\n\n## Queue\n"),
        ] {
            let broken = queued.replacen(from, to, 1);
            let err = current_milestones(&broken).unwrap_err();
            assert!(
                err.iter().any(|e| e.contains("exactly one `## Queue`")),
                "{to:?}: {err:?}"
            );
        }
        for item in [
            "- M6.2e Robust\n",
            "1) M6.2e\n",
            "  1. M6.2e\n",
            "1.M6.2e\n",
        ] {
            let err = current_milestones(&roadmap(item)).unwrap_err();
            assert!(
                err[0].contains("write each line `N. M"),
                "{item:?}: {err:?}"
            );
        }
        // A queue placed after the first phase is a heading inside a phase, which the roadmap's
        // parser refuses.
        let late = format!("{plain}## Queue\n\n1. M6.2e\n");
        assert!(
            current_milestones(&late).unwrap_err()[0].contains("heading after the first phase")
        );
        // An increment under a blocked parent is blocked even when queued.
        let under_blocked = "# R\n## Queue\n\n1. M7.1a\n## Phase 5: Logs\n\
                             - [ ] [blocked] **M7.1 Logs.**\n  - [ ] **M7.1a Parse.**\n\
                             - [ ] **M7.2 Next.**\n";
        assert_eq!(current_milestones(under_blocked).unwrap(), ["M7.2"]);
    }

    const ROADMAP_TEXT: &str = "\
# Roadmap

Rules.

## Phase 0: Foundations

- [x] **M0.1 Workspace.** Done.
  *Done when:* it builds.
- [ ] **M0.5 Lean.**
  - [x] **M0.5a Files.** *Done when:* split.
  - [ ] **M0.5b Queue.**
## Phase 1: Physics

Intro line.

- [x] **M1.8 Aero.**
  - [x] **M1.8e Body.**

    A blank line above belongs to M1.8e.
    - [ ] [blocked] **M1.8e16 Tip.**
- [ ] **M1.14 Accuracy.**
";

    #[test]
    fn parsing_and_rendering_round_trip() {
        let doc = parse(ROADMAP_TEXT).unwrap();
        assert_eq!(render(&doc), ROADMAP_TEXT);
        assert_eq!(doc.phases.len(), 2);
        assert_eq!(doc.phases[1].intro, ["", "Intro line.", ""]);
        let ids: Vec<_> = doc.phases[1]
            .entries
            .iter()
            .map(|e| e.id.as_str())
            .collect();
        assert_eq!(ids, ["M1.8", "M1.8e", "M1.8e16", "M1.14"]);
        assert_eq!(doc.phases[1].entries[1].lines.len(), 3);
    }

    #[test]
    fn checkbox_and_stub_lines_are_read_strictly() {
        assert_eq!(
            checkbox("  - [ ] [blocked] **M2.1b2 Shear.**"),
            Some((2, "M2.1b2".to_owned(), Kind::Open))
        );
        assert_eq!(
            checkbox("- [x] **M1.10 A.**").map(|c| c.1),
            Some("M1.10".to_owned())
        );
        for line in [
            "- [ ] M1.1 no bold",
            "- [-] **M1.1 A.**",
            "- [x] **M1 A.**",
            "- **M1.1** x",
        ] {
            assert_eq!(checkbox(line), None, "{line}");
        }
        let heading = "## Phase 3: Optimization";
        assert_eq!(
            stub(&stub_line(2, "M6.2", true, heading)),
            Some((2, "M6.2".to_owned()))
        );
        assert_eq!(
            stub(&stub_line(0, "M1.8", false, heading)),
            Some((0, "M1.8".to_owned()))
        );
        assert_eq!(
            stub_line(0, "M6.2", true, heading),
            "- **M6.2** is open; its entry: [roadmap](ROADMAP.md#phase-3-optimization)."
        );
        assert_eq!(stub("- **M6.2** is nice"), None);
    }

    #[test]
    fn done_entries_move_and_open_ones_keep_their_parents_as_stubs() {
        let (open, done) = archive(&parse(ROADMAP_TEXT).unwrap(), &Doc::default()).unwrap();
        assert_eq!(
            render(&open),
            "\
# Roadmap

Rules.

## Phase 0: Foundations

- [ ] **M0.5 Lean.**
  - [ ] **M0.5b Queue.**
## Phase 1: Physics

Intro line.

- **M1.8** is done; its entry: [archive](roadmap-done.md#phase-1-physics).
  - **M1.8e** is done; its entry: [archive](roadmap-done.md#phase-1-physics).
    - [ ] [blocked] **M1.8e16 Tip.**
- [ ] **M1.14 Accuracy.**
"
        );
        let done = render(&done);
        let (head, body) = done.split_once("## Phase 0").unwrap();
        assert!(head.starts_with("# Roadmap: done\n"));
        assert!(head.ends_with(
            "- M0.1 Workspace ([Phase 0](#phase-0-foundations))\n\
             - M0.5a Files ([Phase 0](#phase-0-foundations))\n\
             - M1.8 Aero ([Phase 1](#phase-1-physics))\n\
             - M1.8e Body ([Phase 1](#phase-1-physics))\n\n"
        ));
        assert_eq!(
            body,
            ": Foundations

- [x] **M0.1 Workspace.** Done.
  *Done when:* it builds.
- **M0.5** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).
  - [x] **M0.5a Files.** *Done when:* split.
## Phase 1: Physics

- [x] **M1.8 Aero.**
  - [x] **M1.8e Body.**

    A blank line above belongs to M1.8e.
"
        );
    }

    #[test]
    fn a_later_move_replaces_the_stub_and_appends_after_earlier_children() {
        let (open, done) = archive(&parse(ROADMAP_TEXT).unwrap(), &Doc::default()).unwrap();
        // M0.5b and then M0.5 itself are checked off.
        let next = render(&open)
            .replace("- [ ] **M0.5b", "- [x] **M0.5b")
            .replace("- [ ] **M0.5 ", "- [x] **M0.5 ");
        let (open2, done2) = archive(&parse(&next).unwrap(), &done).unwrap();
        assert!(!render(&open2).contains("M0.5"));
        let text = render(&done2);
        assert!(text.contains(
            "- [x] **M0.5 Lean.**\n  - [x] **M0.5a Files.** *Done when:* split.\n  \
             - [x] **M0.5b Queue.**\n## Phase 1"
        ));
        assert!(!text.contains("**M0.5** is open"));
        // Archiving again moves nothing.
        assert_eq!(
            archive(&open2, &done2).unwrap(),
            (open2.clone(), done2.clone())
        );
        // An entry checked off that the archive already holds is refused.
        let twice =
            parse("# R\n## Phase 0: Foundations\n- [x] **M0.1 Workspace.** Done.\n").unwrap();
        assert!(archive(&twice, &done2).unwrap_err().contains("already in"));
        // But an entry the archive holds byte for byte (a move whose roadmap write failed) is
        // finished, not refused.
        let retry = parse(
            "# R\n## Phase 0: Foundations\n- [x] **M0.1 Workspace.** Done.\n  *Done when:* it builds.\n",
        )
        .unwrap();
        let (open3, done3) = archive(&retry, &done2).unwrap();
        assert!(!render(&open3).contains("M0.1"));
        assert_eq!(done3, done2);
        // A line opening with an issue number is not a heading.
        assert!(is_heading("## Phase 1: X") && is_heading("#") && !is_heading("#298 tracks it"));
        assert!(parse("## Phase 0: A\n- [ ] **M0.1 A.**\n#298 tracks it\n").is_ok());
    }

    #[test]
    fn moving_keeps_every_entry_byte_for_byte() {
        let before = parse(ROADMAP_TEXT).unwrap();
        let (open, done) = archive(&before, &Doc::default()).unwrap();
        for phase in &before.phases {
            for entry in phase.entries.iter().filter(|e| e.kind != Kind::Stub) {
                let homes: usize = [&open, &done]
                    .iter()
                    .flat_map(|doc| doc.phases.iter())
                    .flat_map(|p| p.entries.iter())
                    .filter(|e| e.lines == entry.lines)
                    .count();
                assert_eq!(homes, 1, "{}", entry.id);
            }
        }
    }

    #[test]
    fn a_file_the_move_could_get_wrong_is_refused() {
        let cases = [
            (
                "## Phase 0: A\n- [ ] **M0.1 A.**\n## Phase 0: A\n",
                "appears twice",
            ),
            (
                "## Phase 0: A\n- [x] **M0.1 A.**\n## Appendix\n",
                "heading after the first phase",
            ),
            (
                "## Phase 0: A\n- **M0.1** is done; x\n  a note\n",
                "text under the stub",
            ),
            (
                "## Phase 0: A\n- [ ] **M0.1 A.**\n  - [x] **M0.1a B.**\n  *Done when:* x\n",
                "indented less than its own text",
            ),
            (
                "## Phase 0: A\n- [ ] **M0.1 A.**\n- [x]  **M0.2 B.**\n",
                "reads like a milestone",
            ),
            (
                "## Phase 0: A\n* [x] **M0.2 B.**\n",
                "reads like a milestone",
            ),
        ];
        for (text, want) in cases {
            let err = parse(text).unwrap_err();
            assert!(err.contains(want), "{text:?}: {err}");
        }
        // Blank lines under a stub, and dedented text under a top-level entry, are fine.
        assert!(parse("## Phase 0: A\n- **M0.1** is done; x\n\n- [ ] **M0.2 B.**\nnote\n").is_ok());
    }

    #[test]
    fn a_done_parent_missing_from_the_archive_is_refused() {
        // The roadmap holds M1.8 as a done stub, but the archive has no M1.8 to put M1.8f under.
        let roadmap = parse(
            "## Phase 1: Physics\n- **M1.8** is done; its entry is in [the archive](x).\n  \
             - [x] **M1.8f New.**\n",
        )
        .unwrap();
        let err = archive(&roadmap, &Doc::default()).unwrap_err();
        assert!(err.contains("M1.8 is done, but"), "{err}");
    }

    #[test]
    fn an_open_parent_finished_after_its_increments_were_archived() {
        // M6.2a was archived under an "is open" stub; now M6.2 is checked off while M6.2e stays
        // open, and a new phase is added to an archive that already has phases.
        let first = parse(
            "# R\n## Phase 0: A\n- [x] **M0.1 A.**\n## Phase 3: Opt\n- [ ] **M6.2 Opt.**\n  \
             - [x] **M6.2a CMA.**\n  - [ ] **M6.2e Robust.**\n",
        )
        .unwrap();
        let (open, done) = archive(&first, &Doc::default()).unwrap();
        let next =
            parse(&render(&open).replace("- [ ] **M6.2 Opt.**", "- [x] **M6.2 Opt.**")).unwrap();
        let (open2, done2) = archive(&next, &done).unwrap();
        assert_eq!(
            render(&open2),
            "# R\n## Phase 0: A\n## Phase 3: Opt\n- **M6.2** is done; its entry: \
             [archive](roadmap-done.md#phase-3-opt).\n  - [ ] **M6.2e Robust.**\n"
        );
        let text = render(&done2);
        assert!(text.ends_with(
            "## Phase 0: A\n\n- [x] **M0.1 A.**\n## Phase 3: Opt\n\n- [x] **M6.2 Opt.**\n  \
             - [x] **M6.2a CMA.**\n"
        ));
        // And the roadmap's new stub reads back as a stub, so archiving again changes nothing.
        let again = parse(&render(&open2)).unwrap();
        assert_eq!(archive(&again, &done2).unwrap().0, open2);
    }
}
