//! The product's name on every surface that ships (ADR-199 §1, ADR-205; M0.9a), checked by
//! `cargo xtask names` and by `cargo test -p xtask`.
//!
//! The suite is FusionSpace HPR and its first product FusionSpace HPR · Sim, "HPR Sim" or "the
//! simulator" after its first mention; `hpr`, in code formatting, is the command and the Rust
//! library. Two rules find what the rename missed:
//!
//! - **the old name:** `hpr-sim`, in any case and with the non-breaking hyphen U+2011 too,
//!   anywhere in a surface's text, code formatting and link targets included. The packages'
//!   names since ADR-203, such as `fusionspace-hpr-sim`, are the new name and don't count;
//! - **a bare `hpr`:** a lowercase (or capitalised) `hpr` standing as a word in a surface's
//!   prose, outside code formatting, as in "hpr's apogees". A word joined to it (`hpr-core`,
//!   `hpr_sim`, `.hpr`, `hpr.fusionspace.co`, `@fusionspace/hpr`, `hpr::sim`) is a name of
//!   something else. In a string literal, which has no code formatting, `hpr` is the command
//!   when a subcommand, a flag or a `{}` placeholder follows it, or when it is the whole literal.
//!
//! The surfaces, from ADR-205 §1 (M0.9a1; the site's pages join in M0.9a2): the README and the
//! landing page, `docs/start-here.md`; every
//! crate's `Cargo.toml` description and README; the rustdoc and string literals (help text and
//! messages) under each published crate's `src/` and the Python bindings' (whose rustdoc is the
//! package's docstrings); the Python package's docstrings and `pyproject.toml`; the schemas'
//! descriptions and titles and the bindings generated from them; `LICENSE-MIT` and
//! `LICENSE-APACHE` and their copies, the release's license template, `THIRD-PARTY-NOTICES.md`
//! and `CHANGELOG.md`; and the banners' `project.json`.
//!
//! A mention is allowed only by an entry of [`ALLOW`]: a file, the exact text around the
//! mention, how many mentions it covers, and why it stays. An entry covers a mention only inside
//! its text, so a new mention on the same line still fails, and so does a new one in a copy of its
//! text, since its count no longer matches; an entry that covers nothing fails as stale. An entry
//! is one line of at most [`MAX_ALLOW_TEXT`] bytes with at least [`MIN_ALLOW_CONTEXT`] beyond its
//! mentions; only an ADR up to ADR-205, or the roadmap's archive, may be allowed whole
//! ([`Scope::Page`]).

use std::ops::Range;
use std::path::Path;
use std::process::Command;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub const USAGE: &str =
    "  names                    Check that no shipped surface says hpr-sim, or a bare hpr for
                           the product, outside its reasoned allowlist (ADR-205; M0.9a).";

/// The products shipped, by their full names (ADR-199 §1): the landing page's first paragraph
/// names each. Each release milestone that ships a product adds it here.
pub const SHIPPED: &[&str] = &["FusionSpace HPR · Sim"];

/// The suite's name, which the landing page's first paragraph names on its own too.
pub const SUITE: &str = "FusionSpace HPR";

/// The landing page.
const LANDING: &str = "docs/start-here.md";

/// The longest text an [`Allow`] entry may give: about a line of prose.
const MAX_ALLOW_TEXT: usize = 120;

/// The least text an [`Allow`] entry gives beyond the mentions in it, so that an entry names a
/// place and not just the name: a bare `hpr-sim` would allow every one in its file.
const MIN_ALLOW_CONTEXT: usize = 8;

/// The newest decision record that may be allowed whole: the one that set the rule.
const LAST_WHOLE_ADR: u32 = 205;

/// What an [`Allow`] entry covers in its file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// The mentions inside each occurrence of this exact text.
    Text(&'static str),
    /// The whole file: only an ADR up to [`LAST_WHOLE_ADR`], or the roadmap's archive.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no surface checked yet is a record allowed whole")
    )]
    Page,
}

/// A mention allowed to stay: its file, what it covers, and why.
#[derive(Debug, Clone, Copy)]
pub struct Allow {
    pub file: &'static str,
    pub scope: Scope,
    /// How many mentions it covers: one more in its text, or one fewer, fails.
    pub count: usize,
    pub why: &'static str,
}

/// The mentions that stay, each with its reason.
pub const ALLOW: &[Allow] = &[
    Allow {
        file: "CHANGELOG.md",
        scope: Scope::Text("(called hpr-sim before release 0.1)"),
        count: 1,
        why: "history: the name the project had until release 0.1",
    },
    Allow {
        file: "CHANGELOG.md",
        scope: Scope::Text("ork.html#how-many-configurations-hpr-sim-flies"),
        count: 1,
        why: "an anchor: the heading names the `hpr sim` command",
    },
    Allow {
        file: "README.md",
        scope: Scope::Text("(called hpr-sim before release 0.1)"),
        count: 1,
        why: "history: the name a reader may know it by",
    },
    Allow {
        file: "README.md",
        scope: Scope::Text("cli.html#hpr-sim)"),
        count: 1,
        why: "an anchor: the heading names the `hpr sim` command",
    },
    Allow {
        file: "docs/start-here.md",
        scope: Scope::Text("HPR Sim (called hpr-sim before release 0.1) simulates"),
        count: 1,
        why: "history: the name a reader may know it by",
    },
    Allow {
        file: "docs/start-here.md",
        scope: Scope::Text("(cli.md#hpr-sim)"),
        count: 2,
        why: "an anchor: the heading names the `hpr sim` command",
    },
    Allow {
        file: "docs/start-here.md",
        scope: Scope::Text("(format/ork.md#how-many-configurations-hpr-sim-flies)"),
        count: 1,
        why: "an anchor: the heading names the `hpr sim` command",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("`crates/hpr-sim/tests/data/"),
        count: 1,
        why: "a path under `crates/hpr-sim/`, the flight engine's folder",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("| `hpr-sim-fixtures` | `nrdptel/hpr-sim-fixtures` |"),
        count: 2,
        why: "`hpr-sim-fixtures`, the private fixtures repository, which kept its name (ADR-151)",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("| `bytes` | MIT | `hpr-sim` (tests only) |"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("| MIT | `hpr-sim`, `hpr-cli` (tests only) |"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("| Apache-2.0 | `hpr-sim` (tests only, as `parquet-reader`) |"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("the Parquet files `hpr-sim` writes"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("`hpr-io`; `hpr-sim` (tests only)"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "THIRD-PARTY-NOTICES.md",
        scope: Scope::Text("`hpr-motor`, `hpr-sim` (JSON and GeoJSON exports)"),
        count: 1,
        why: "a crate id: the table names each crate by its folder, as `hpr-cli`",
    },
    Allow {
        file: "crates/hpr-core/src/tool.rs",
        scope: Scope::Text("which readers still recognize: `hpr-sim`"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4)",
    },
    Allow {
        file: "crates/hpr-core/src/tool.rs",
        scope: Scope::Text("pub const EARLIER_NAME: &str = \"hpr-sim\";"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4)",
    },
    Allow {
        file: "crates/hpr-motor/src/eng.rs",
        scope: Scope::Text("the earlier name `hpr-sim` with either designation"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4)",
    },
    Allow {
        file: "crates/hpr-net/src/cache.rs",
        scope: Scope::Text("the `hpr-sim` folders earlier builds used"),
        count: 1,
        why: "history: the cache folders builds before the rename used, named to say they are not read",
    },
    Allow {
        file: "crates/hpr-format/src/lib.rs",
        scope: Scope::Text("(`hpr-sim` in a document an earlier build wrote)"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4)",
    },
    Allow {
        file: "schema/format/hpr-design-0.2.schema.json",
        scope: Scope::Text("(`hpr-sim` in a document an earlier build wrote)"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4); generated from `hpr-format`",
    },
    Allow {
        file: "schema/format/python/hpr_design.py",
        scope: Scope::Text("(`hpr-sim` in a document an earlier build wrote)"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4); generated from the schema",
    },
    Allow {
        file: "schema/format/typescript/hpr-design.ts",
        scope: Scope::Text("(`hpr-sim` in a document an earlier build wrote)"),
        count: 1,
        why: "an old stamp that must still read: files written before the rename carry it (ADR-199 §4); generated from the schema",
    },
    Allow {
        file: "schema/format/hpr-design-0.1.schema.json",
        scope: Scope::Text("The program, such as `hpr-sim`."),
        count: 1,
        why: "history: version 0.1's schema names the program as the builds that wrote 0.1 documents did",
    },
    Allow {
        file: "crates/hpr-io/src/ork/staging.rs",
        scope: Scope::Text("0165-an-unpowered-separation-in-hpr-sim.md"),
        count: 1,
        why: "an ADR's file name, which keeps the name it was written under",
    },
    Allow {
        file: "crates/hpr/src/ork.rs",
        scope: Scope::Text("0165-an-unpowered-separation-in-hpr-sim.md"),
        count: 4,
        why: "an ADR's file name, which keeps the name it was written under",
    },
    Allow {
        file: "crates/hpr/src/ork.rs",
        scope: Scope::Text("0159-a-powered-separation-in-hpr-sim.md"),
        count: 1,
        why: "an ADR's file name, which keeps the name it was written under",
    },
];

/// The kinds of surface, each read its own way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Markdown: the old name anywhere, a bare `hpr` in its text outside code.
    Markdown,
    /// Plain text, such as a license: both rules everywhere.
    Text,
    /// Rust source: rustdoc as Markdown, string literals as messages.
    Rust,
    /// Python: the old name anywhere, a bare `hpr` in docstrings and comments.
    Python,
    /// TypeScript bindings: the old name anywhere, a bare `hpr` in comments.
    TypeScript,
    /// A JSON schema: the old name anywhere, a bare `hpr` in descriptions and titles.
    Schema,
    /// A JSON document read whole, such as the banners' `project.json`: its string values.
    Json,
    /// A `Cargo.toml`: its package's description, homepage and links.
    CargoManifest,
    /// A `pyproject.toml`: the old name anywhere, a bare `hpr` in its description.
    PyProject,
}

/// Which rules a range of a surface's text is held to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Held {
    /// The old name only: code, link targets, names.
    OldName,
    /// Both rules: prose.
    Prose,
    /// Both rules, `hpr` as the command allowed: a string literal.
    Message,
}

/// The rule a mention breaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    OldName,
    BareHpr,
}

/// One mention: where it is in the file's text, and the rule it breaks.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Mention {
    at: Range<usize>,
    rule: Rule,
}

pub fn run(args: &[String]) -> Result<(), String> {
    if !args.is_empty() {
        return Err(format!("names takes no arguments\n\n{USAGE}"));
    }
    let root = crate::designs::root()?;
    let problems = problems(&root, ALLOW)?;
    if problems.is_empty() {
        println!("names: every surface names the product by its name");
        Ok(())
    } else {
        Err(format!(
            "{} problem(s):\n{}",
            problems.len(),
            problems.join("\n")
        ))
    }
}

/// The tracked files at `root`.
fn tracked(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z"])
        .output()
        .map_err(|e| format!("git ls-files: {e}"))?;
    if !output.status.success() {
        return Err("git ls-files failed".to_owned());
    }
    let listing = String::from_utf8(output.stdout).map_err(|e| format!("git ls-files: {e}"))?;
    Ok(listing
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Whether the crate whose `Cargo.toml` is `manifest` is published: it doesn't say
/// `publish = false`.
fn published(manifest: &str) -> bool {
    !manifest
        .lines()
        .any(|line| line.replace(' ', "") == "publish=false")
}

/// The surfaces among the tracked files `files` at `root`, each with its kind.
fn surfaces(root: &Path, files: &[String]) -> Vec<(String, Kind)> {
    let crates: Vec<&str> = files
        .iter()
        .filter_map(|path| {
            let rest = path.strip_prefix("crates/")?;
            let (name, file) = rest.split_once('/')?;
            (file == "Cargo.toml").then_some(name)
        })
        .collect();
    // The crates whose `src/` ships: the published ones, and the Python bindings, whose rustdoc
    // is the Python package's docstrings.
    let shipped: Vec<&str> = crates
        .iter()
        .copied()
        .filter(|name| {
            *name == "hpr-py"
                || std::fs::read_to_string(root.join("crates").join(name).join("Cargo.toml"))
                    .is_ok_and(|manifest| published(&manifest))
        })
        .collect();
    let mut found = Vec::new();
    for path in files {
        let kind = if matches!(
            path.as_str(),
            "README.md" | "CHANGELOG.md" | "THIRD-PARTY-NOTICES.md" | LANDING
        ) {
            Some(Kind::Markdown)
        } else if path == ".github/brand/project.json" {
            Some(Kind::Json)
        } else if path == "scripts/release/licenses.hbs" {
            Some(Kind::Text)
        } else if path.starts_with("schema/") {
            if path.ends_with(".schema.json") {
                Some(Kind::Schema)
            } else if path.ends_with(".py") {
                Some(Kind::Python)
            } else if path.ends_with(".ts") {
                Some(Kind::TypeScript)
            } else {
                None
            }
        } else if let Some(rest) = path.strip_prefix("crates/") {
            let (name, file) = rest.split_once('/').unwrap_or((rest, ""));
            match file {
                "Cargo.toml" => Some(Kind::CargoManifest),
                "README.md" => Some(Kind::Markdown),
                "LICENSE-MIT" | "LICENSE-APACHE" => Some(Kind::Text),
                "pyproject.toml" => Some(Kind::PyProject),
                _ if file.starts_with("python/") && file.ends_with(".py") => Some(Kind::Python),
                _ if file.starts_with("src/")
                    && file.ends_with(".rs")
                    && shipped.contains(&name) =>
                {
                    Some(Kind::Rust)
                }
                _ => None,
            }
        } else if matches!(path.as_str(), "LICENSE-MIT" | "LICENSE-APACHE") {
            Some(Kind::Text)
        } else {
            None
        };
        if let Some(kind) = kind {
            found.push((path.clone(), kind));
        }
    }
    // The files of modules declared under `#[cfg(test)]` ship nothing.
    let mut test_only: Vec<String> = Vec::new();
    for (path, kind) in &found {
        if *kind == Kind::Rust
            && let Ok(text) = std::fs::read_to_string(root.join(path))
        {
            for name in rust_test_modules(&text) {
                test_only.extend(module_paths(path, &name));
            }
        }
    }
    found.retain(|(path, _)| {
        !test_only
            .iter()
            .any(|test| path == test || test.ends_with('/') && path.starts_with(test.as_str()))
    });
    found
}

/// The surfaces at `root`, read: the tracked files, and each surface's path, text and mentions.
struct Read {
    files: Vec<String>,
    surfaces: Vec<(String, String, Vec<Mention>)>,
    landing: Vec<String>,
}

/// Reads every surface at `root` once, so that allowlists can be tried against it.
fn read(root: &Path) -> Result<Read, String> {
    let files = tracked(root)?;
    let mut surfaces = Vec::new();
    for (path, kind) in self::surfaces(root, &files) {
        // A file deleted in the working tree but not yet from the index names nothing.
        let Ok(bytes) = std::fs::read(root.join(&path)) else {
            continue;
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let found = mentions(kind, &text);
        surfaces.push((path, text, found));
    }
    Ok(Read {
        files,
        surfaces,
        landing: landing_problems(root),
    })
}

/// The problems at `root` under the allowlist `allow`: each mention no entry covers, as
/// `path:line: rule: text`, each entry that covers nothing, and each entry that isn't narrow.
fn problems(root: &Path, allow: &[Allow]) -> Result<Vec<String>, String> {
    Ok(problems_of(&read(root)?, allow))
}

/// The problems of the surfaces `read` under the allowlist `allow`.
fn problems_of(read: &Read, allow: &[Allow]) -> Vec<String> {
    let mut found = allowlist_problems(allow, &read.files);
    let mut used = vec![0_usize; allow.len()];
    for (path, text, mentions) in &read.surfaces {
        found.extend(uncovered(path, text, mentions, allow, &mut used));
    }
    for (entry, count) in allow.iter().zip(used) {
        if count == 0 {
            found.push(format!(
                "{}: the allowlist's entry {} covers nothing: drop it ({})",
                entry.file,
                describe(entry.scope),
                entry.why
            ));
        } else if count != entry.count {
            found.push(format!(
                "{}: the allowlist's entry {} covers {count} mention(s), not the {} it says",
                entry.file,
                describe(entry.scope),
                entry.count
            ));
        }
    }
    found.extend(read.landing.iter().cloned());
    found
}

/// An entry's scope in a message.
fn describe(scope: Scope) -> String {
    match scope {
        Scope::Text(text) => format!("`{text}`"),
        Scope::Page => "for the whole page".to_owned(),
    }
}

/// Whether `path` may be allowed whole: an ADR up to [`LAST_WHOLE_ADR`], or the roadmap's
/// archive.
fn may_be_whole(path: &str) -> bool {
    if path == "docs/roadmap-done.md" {
        return true;
    }
    path.strip_prefix("docs/decisions/")
        .and_then(|name| name.get(..4))
        .and_then(|number| number.parse::<u32>().ok())
        .is_some_and(|number| number <= LAST_WHOLE_ADR)
}

/// The entries of `allow` that aren't narrow, or name a file that isn't tracked.
fn allowlist_problems(allow: &[Allow], files: &[String]) -> Vec<String> {
    let mut found = Vec::new();
    for entry in allow {
        if !files.iter().any(|path| path == entry.file) {
            found.push(format!(
                "{}: the allowlist names a file that isn't tracked",
                entry.file
            ));
        }
        if entry.why.trim().is_empty() {
            found.push(format!(
                "{}: the allowlist's entry {} gives no reason",
                entry.file,
                describe(entry.scope)
            ));
        }
        match entry.scope {
            Scope::Page if !may_be_whole(entry.file) => found.push(format!(
                "{}: only an ADR up to ADR-{LAST_WHOLE_ADR} or the roadmap's archive may be \
                 allowed whole",
                entry.file
            )),
            Scope::Page => {}
            Scope::Text(text) => {
                if text.contains('\n') || text.len() > MAX_ALLOW_TEXT {
                    found.push(format!(
                        "{}: the allowlist's entry `{}…` is wider than a line of {MAX_ALLOW_TEXT} \
                         bytes",
                        entry.file,
                        text.chars().take(40).collect::<String>()
                    ));
                }
                let inside: usize = old_names(text, 0..text.len())
                    .into_iter()
                    .chain(bare_hprs(text, 0..text.len(), Held::Prose))
                    .map(|mention| mention.at.len())
                    .sum();
                if inside == 0 {
                    found.push(format!(
                        "{}: the allowlist's entry `{text}` holds no mention to allow",
                        entry.file
                    ));
                } else if text.len() - inside < MIN_ALLOW_CONTEXT {
                    found.push(format!(
                        "{}: the allowlist's entry `{text}` is little more than its mention: give \
                         it {MIN_ALLOW_CONTEXT} bytes of the text around it",
                        entry.file
                    ));
                }
            }
        }
    }
    found
}

/// The problems in the file `path` of kind `kind` with the text `text`: each mention that no
/// entry of `allow` covers. `used` counts, for each entry, the mentions it covered.
#[cfg(test)]
fn problems_in(
    path: &str,
    kind: Kind,
    text: &str,
    allow: &[Allow],
    used: &mut [usize],
) -> Vec<String> {
    uncovered(path, text, &mentions(kind, text), allow, used)
}

/// The mentions `mentions` in the file `path` with the text `text` that no entry of `allow`
/// covers, as problems. `used` counts, for each entry, the mentions it covered.
fn uncovered(
    path: &str,
    text: &str,
    mentions: &[Mention],
    allow: &[Allow],
    used: &mut [usize],
) -> Vec<String> {
    let mut problems = Vec::new();
    for mention in mentions {
        let covered = allow.iter().position(|entry| {
            entry.file == path
                && match entry.scope {
                    Scope::Page => true,
                    Scope::Text(allowed) => text.match_indices(allowed).any(|(at, _)| {
                        at <= mention.at.start && mention.at.end <= at + allowed.len()
                    }),
                }
        });
        match covered {
            Some(index) => used[index] += 1,
            None => {
                let line = text[..mention.at.start].matches('\n').count() + 1;
                let start = text[..mention.at.start].rfind('\n').map_or(0, |at| at + 1);
                let end = text[mention.at.end..]
                    .find('\n')
                    .map_or(text.len(), |at| mention.at.end + at);
                let what = match mention.rule {
                    Rule::OldName => "the old name",
                    Rule::BareHpr => "a bare `hpr` for the product",
                };
                problems.push(format!(
                    "{path}:{line}: {what}: {}",
                    text[start..end].trim()
                ));
            }
        }
    }
    problems
}

/// Every mention in a surface of kind `kind` with the text `text`, in order.
fn mentions(kind: Kind, text: &str) -> Vec<Mention> {
    let mut found = Vec::new();
    for (range, held) in ranges(kind, text) {
        found.extend(old_names(text, range.clone()));
        if held != Held::OldName {
            found.extend(bare_hprs(text, range, held));
        }
    }
    found.sort_by_key(|mention| (mention.at.start, mention.at.end));
    found.dedup();
    found
}

/// The ranges of `text` a surface of kind `kind` is checked in, each with the rules it is held to.
fn ranges(kind: Kind, text: &str) -> Vec<(Range<usize>, Held)> {
    let whole = 0..text.len();
    match kind {
        Kind::Text => vec![(whole, Held::Prose)],
        Kind::Markdown => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(
                markdown_prose(text)
                    .into_iter()
                    .map(|range| (range, Held::Prose)),
            );
            found
        }
        Kind::Rust => rust_ranges(text),
        Kind::Python => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(python_docstrings(text));
            found.extend(
                line_comments(text, "#")
                    .into_iter()
                    .flat_map(|range| prose_within(text, range)),
            );
            found
        }
        Kind::TypeScript => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(
                line_comments(text, "//")
                    .into_iter()
                    .chain(block_comments(text))
                    .flat_map(|range| prose_within(text, range)),
            );
            found
        }
        Kind::Schema => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(
                json_strings(text)
                    .into_iter()
                    .filter(|(key, _)| {
                        matches!(
                            key.as_deref(),
                            Some("description" | "title" | "$comment" | "markdownDescription")
                        )
                    })
                    .flat_map(|(_, range)| prose_within(text, range)),
            );
            found
        }
        Kind::Json => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(
                json_strings(text)
                    .into_iter()
                    .filter(|(key, _)| key.is_some())
                    .flat_map(|(_, range)| prose_within(text, range)),
            );
            found
        }
        Kind::CargoManifest => toml_values(
            text,
            &[
                "description",
                "homepage",
                "documentation",
                "repository",
                "keywords",
                "categories",
            ],
        ),
        Kind::PyProject => {
            let mut found = vec![(whole, Held::OldName)];
            found.extend(toml_values(text, &["description"]));
            found
        }
    }
}

/// The old name in `text[range]`: `hpr-sim` in any case, with an ASCII or a non-breaking hyphen,
/// not as the end of a package's name since ADR-203 (`fusionspace-hpr-sim`).
fn old_names(text: &str, range: Range<usize>) -> Vec<Mention> {
    let lower = text.to_ascii_lowercase();
    let mut found = Vec::new();
    let mut from = range.start;
    while let Some(at) = lower[from..range.end].find("hpr").map(|at| from + at) {
        from = at + 3;
        let rest = &lower[at + 3..range.end];
        let Some(after) = rest
            .strip_prefix('-')
            .or_else(|| rest.strip_prefix('\u{2011}'))
        else {
            continue;
        };
        if !after.starts_with("sim") {
            continue;
        }
        let end = range.end - after.len() + 3;
        let before = &lower[..at];
        if before.ends_with("fusionspace-") || before.ends_with("fusionspace\u{2011}") {
            continue;
        }
        found.push(Mention {
            at: at..end,
            rule: Rule::OldName,
        });
    }
    found
}

/// Whether `c`, before an `hpr`, joins it into another name: `hpr_sim`, `.hpr`,
/// `fusionspace-hpr`, `@fusionspace/hpr`, `docs#hpr`.
fn joins_before(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '_' | '-' | '\u{2011}' | '.' | '/' | '@' | '#' | ':' | '\\'
        )
}

/// A bare `hpr` (or `Hpr`) in `text[range]`, held as `held`.
fn bare_hprs(text: &str, range: Range<usize>, held: Held) -> Vec<Mention> {
    let mut found = Vec::new();
    let slice = &text[range.clone()];
    for (offset, _) in slice.match_indices("hpr").chain(slice.match_indices("Hpr")) {
        let at = range.start + offset;
        let end = at + 3;
        // An escape such as `\n` ends the word before it, though its letter is alphanumeric.
        let escaped = ["\\n", "\\t", "\\r", "\\0"]
            .iter()
            .any(|escape| text[..at].ends_with(escape));
        if !escaped && text[..at].chars().next_back().is_some_and(joins_before) {
            continue;
        }
        let after = &text[end..];
        let mut next = after.chars();
        let joined = match next.next() {
            None => false,
            Some(c) if c.is_alphanumeric() || matches!(c, '_' | '-' | '\u{2011}' | '/') => true,
            Some('.') => next.next().is_some_and(char::is_alphanumeric),
            Some(':') => next.next() == Some(':'),
            Some(_) => false,
        };
        if joined {
            continue;
        }
        if held == Held::Message && is_the_command(&text[range.clone()], at - range.start) {
            continue;
        }
        found.push(Mention {
            at: at..end,
            rule: Rule::BareHpr,
        });
    }
    found
}

/// Whether the `hpr` at byte `at` of the string literal `literal` is the command: the whole
/// literal, or followed by a subcommand, a flag or a `{}` placeholder.
fn is_the_command(literal: &str, at: usize) -> bool {
    if literal.trim() == "hpr" {
        return true;
    }
    // A line continuation (`\` at a line's end) and spaces stand between words.
    let rest = literal[at + 3..].trim_start_matches(|c: char| c.is_whitespace() || c == '\\');
    if rest.len() == literal[at + 3..].len() {
        return false;
    }
    let flag = rest.starts_with("--")
        || rest
            .strip_prefix('-')
            .is_some_and(|after| after.starts_with(|c: char| c.is_ascii_alphabetic()));
    if flag || rest.starts_with('{') {
        return true;
    }
    let word: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    subcommands().contains(&word)
}

/// The command line's subcommands, by name.
fn subcommands() -> &'static [String] {
    static NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    NAMES.get_or_init(|| {
        hpr_cli::command()
            .get_subcommands()
            .flat_map(|command| {
                std::iter::once(command.get_name().to_owned()).chain(
                    command
                        .get_subcommands()
                        .map(|sub| sub.get_name().to_owned()),
                )
            })
            .collect()
    })
}

/// The prose in `text[range]`, a string holding Markdown: its text outside code formatting.
fn prose_within(text: &str, range: Range<usize>) -> Vec<(Range<usize>, Held)> {
    markdown_prose(&text[range.clone()])
        .into_iter()
        .map(|inner| {
            (
                range.start + inner.start..range.start + inner.end,
                Held::Prose,
            )
        })
        .collect()
}

/// The ranges of Markdown `text` that are prose: text outside code blocks, code spans and HTML.
/// Link targets aren't text, so they aren't included.
fn markdown_prose(text: &str) -> Vec<Range<usize>> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let mut found = Vec::new();
    let mut in_code = 0_usize;
    for (event, range) in Parser::new_ext(text, options).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code += 1,
            Event::End(TagEnd::CodeBlock) => in_code = in_code.saturating_sub(1),
            Event::Text(_) if in_code == 0 => found.push(range),
            _ => {}
        }
    }
    found
}

/// The ranges of Rust source `text` to check: its rustdoc's prose (`///` and `//!` lines read
/// together as Markdown), the whole of each rustdoc line for the old name, and each string
/// literal's contents as a message. An item under `#[cfg(test)]` is compiled only for tests and
/// ships nothing, so it is skipped.
fn rust_ranges(text: &str) -> Vec<(Range<usize>, Held)> {
    rust_lex(text).0
}

/// The modules `text` declares in files of their own under `#[cfg(test)]` (`mod tests;`), which
/// ship nothing either.
fn rust_test_modules(text: &str) -> Vec<String> {
    rust_lex(text).1
}

/// [`rust_ranges`] and [`rust_test_modules`] in one pass.
fn rust_lex(text: &str) -> (Vec<(Range<usize>, Held)>, Vec<String>) {
    let mut found = Vec::new();
    let mut test_modules = Vec::new();
    // The braces open in code; where the `#[cfg(test)]` item being read began and at what depth,
    // if one is; and while inside such an item, the depth its body closes at.
    let mut depth = 0_usize;
    let mut test_item: Option<(usize, usize)> = None;
    let mut skip_to: Option<usize> = None;
    // Each run of consecutive rustdoc lines: the Markdown read from them, and for each line where
    // its text starts in the file and in the Markdown.
    let mut doc = String::new();
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let flush = |doc: &mut String, starts: &mut Vec<(usize, usize)>, found: &mut Vec<_>| {
        for range in markdown_prose(doc) {
            // Each line's part of the range, mapped to where that line's text is in the file.
            let mut line = starts.partition_point(|(_, in_doc)| *in_doc <= range.start) - 1;
            let mut from = range.start;
            while from < range.end && line < starts.len() {
                let (in_file, in_doc) = starts[line];
                let line_end = starts.get(line + 1).map_or(doc.len(), |next| next.1 - 1);
                let to = range.end.min(line_end);
                if from < to {
                    found.push((in_file + from - in_doc..in_file + to - in_doc, Held::Prose));
                }
                line += 1;
                from = starts.get(line).map_or(range.end, |next| next.1);
            }
        }
        doc.clear();
        starts.clear();
    };
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &text[i..];
        let skipping = skip_to.is_some();
        if rest.starts_with("#[cfg(test)]") {
            if !skipping {
                test_item = Some((i, depth));
            }
            i += "#[cfg(test)]".len();
        } else if bytes[i] == b','
            && test_item.is_some_and(|(start, at)| at == depth && ends_a_field(&text[start..i]))
        {
            // The attribute was on a field, a variant or a match arm, which ends here: nothing
            // braced after it is test-only.
            test_item = None;
            i += 1;
        } else if test_item.is_some() && (bytes[i] == b'{' || bytes[i] == b';') {
            if bytes[i] == b'{' {
                skip_to = Some(depth);
                depth += 1;
            } else if let Some((start, _)) = test_item {
                // `mod name;` after the attribute (and any others) declares a file of its own.
                let item = text[start..i].rsplit(']').next().unwrap_or("").trim();
                if let Some(name) = item
                    .strip_prefix("pub(crate) mod ")
                    .or_else(|| item.strip_prefix("pub(super) mod "))
                    .or_else(|| item.strip_prefix("pub mod "))
                    .or_else(|| item.strip_prefix("mod "))
                {
                    test_modules.push(name.trim().to_owned());
                }
            }
            test_item = None;
            i += 1;
        } else if bytes[i] == b'{' {
            depth += 1;
            i += 1;
        } else if bytes[i] == b'}' {
            depth = depth.saturating_sub(1);
            if skip_to == Some(depth) {
                skip_to = None;
            }
            // A last field or variant under the attribute ends with its enclosing braces.
            if test_item.is_some_and(|(_, at)| depth < at) {
                test_item = None;
            }
            i += 1;
        } else if rest.starts_with("//") {
            let line_end = rest.find('\n').map_or(text.len(), |at| i + at);
            let is_doc =
                (rest.starts_with("///") && !rest.starts_with("////")) || rest.starts_with("//!");
            if is_doc {
                let mut content = i + 3;
                if text[content..line_end].starts_with(' ') {
                    content += 1;
                }
                if !skipping {
                    found.push((content..line_end, Held::OldName));
                    starts.push((content, doc.len()));
                    doc.push_str(&text[content..line_end]);
                    doc.push('\n');
                }
                // The next line continues the run only if it is rustdoc too.
                let next = text[line_end..].trim_start_matches(['\n', '\r', ' ', '\t']);
                if !(next.starts_with("///") && !next.starts_with("////")
                    || next.starts_with("//!"))
                {
                    flush(&mut doc, &mut starts, &mut found);
                }
            }
            i = line_end;
        } else if rest.starts_with("/*") {
            // Compared byte by byte: a comment's text may hold any character.
            let mut nested = 0_usize;
            let mut j = i;
            while j < bytes.len() {
                if bytes[j..].starts_with(b"/*") {
                    nested += 1;
                    j += 2;
                } else if bytes[j..].starts_with(b"*/") {
                    nested -= 1;
                    j += 2;
                    if nested == 0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            // `/** … */` and `/*! … */` are rustdoc, as `///` and `//!` are.
            let is_doc =
                (rest.starts_with("/**") && !rest.starts_with("/***") && !rest.starts_with("/**/"))
                    || rest.starts_with("/*!");
            if is_doc && !skipping && j >= i + 5 {
                let inside = i + 3..j - 2;
                found.push((inside.clone(), Held::OldName));
                found.extend(prose_within(text, inside));
            }
            i = j;
        } else if let Some((content, end)) = raw_string(text, i) {
            if !skipping {
                found.push((content, Held::Message));
            }
            i = end;
        } else if bytes[i] == b'"'
            && !text[..i].ends_with(|c: char| c.is_alphanumeric() || c == '_')
            || (rest.starts_with("b\"") || rest.starts_with("c\""))
                && !text[..i].ends_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let open = i + rest.find('"').unwrap_or(0) + 1;
            let mut j = open;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let close = j.min(bytes.len());
            if !skipping {
                found.push((open..close, Held::Message));
            }
            i = close + 1;
        } else if bytes[i] == b'\'' {
            // A character literal, or a lifetime or label, which has no closing quote.
            let after = &text[i + 1..];
            if let Some(escaped) = after.strip_prefix('\\') {
                // The escaped character (`'`, `n`, `u`, …), then the closing quote.
                i = escaped
                    .get(1..)
                    .and_then(|rest| rest.find('\''))
                    .map_or(text.len(), |at| i + at + 4);
            } else {
                let mut chars = after.char_indices();
                match (chars.next(), chars.next()) {
                    (Some(_), Some((at, '\''))) => i += 1 + at + 1,
                    _ => i += 1,
                }
            }
        } else {
            i += rest.chars().next().map_or(1, char::len_utf8);
        }
    }
    (found, test_modules)
}

/// Whether a comma after `header`, the text from a `#[cfg(test)]` on, ends what the attribute is on:
/// a field, a variant or a match arm. It doesn't when the comma is inside brackets (an attribute's
/// arguments, a tuple) or follows an item's keyword (a function's parameters, an `impl`'s
/// generics), where the item goes on to its body.
fn ends_a_field(header: &str) -> bool {
    const ITEMS: [&str; 13] = [
        "fn",
        "impl",
        "mod",
        "struct",
        "enum",
        "union",
        "trait",
        "use",
        "const",
        "static",
        "type",
        "extern",
        "macro_rules",
    ];
    let open = header.matches(['(', '[']).count();
    let closed = header.matches([')', ']']).count();
    open == closed
        && !header
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '!'))
            .any(|word| ITEMS.contains(&word.trim_end_matches('!')))
}

/// The files of the module `name` declared in the Rust file `parent`: `name.rs` and
/// `name/mod.rs` beside a `lib.rs`, `main.rs` or `mod.rs`, under a folder named for the file
/// otherwise; as a folder too, for the modules inside it.
fn module_paths(parent: &str, name: &str) -> [String; 2] {
    let (dir, file) = parent.rsplit_once('/').unwrap_or(("", parent));
    let base = if matches!(file, "lib.rs" | "main.rs" | "mod.rs") {
        dir.to_owned()
    } else {
        format!("{dir}/{}", file.trim_end_matches(".rs"))
    };
    [format!("{base}/{name}.rs"), format!("{base}/{name}/")]
}

/// A raw string literal (`r"…"`, `r#"…"#`, `br#"…"#`, `cr"…"`) starting at byte `at` of `text`:
/// its contents and the byte after it.
fn raw_string(text: &str, at: usize) -> Option<(Range<usize>, usize)> {
    if text[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let rest = &text[at..];
    let after_prefix = rest
        .strip_prefix("br")
        .or_else(|| rest.strip_prefix("cr"))
        .or_else(|| rest.strip_prefix('r'))?;
    let hashes = after_prefix.len() - after_prefix.trim_start_matches('#').len();
    let after_hashes = &after_prefix[hashes..];
    if !after_hashes.starts_with('"') {
        return None;
    }
    let open = text.len() - after_hashes.len() + 1;
    let close_mark = format!("\"{}", "#".repeat(hashes));
    let close = text[open..]
        .find(&close_mark)
        .map_or(text.len(), |found| open + found);
    Some((open..close, (close + close_mark.len()).min(text.len())))
}

/// The prose ranges of a Python file `text`: its triple-quoted strings (its docstrings) read as
/// Markdown, whose code spans cover reStructuredText's ``literals`` too.
fn python_docstrings(text: &str) -> Vec<(Range<usize>, Held)> {
    let mut found = Vec::new();
    let mut from = 0;
    loop {
        let next = ["\"\"\"", "'''"]
            .iter()
            .filter_map(|quote| text[from..].find(quote).map(|at| (from + at, *quote)))
            .min_by_key(|(at, _)| *at);
        let Some((open, quote)) = next else {
            break;
        };
        let start = open + 3;
        let end = text[start..]
            .find(quote)
            .map_or(text.len(), |at| start + at);
        found.extend(
            markdown_prose(&text[start..end])
                .into_iter()
                .map(|range| (start + range.start..start + range.end, Held::Prose)),
        );
        from = (end + 3).min(text.len());
    }
    found
}

/// The text after `marker` on each line of `text` that has it outside a string.
fn line_comments(text: &str, marker: &str) -> Vec<Range<usize>> {
    let mut found = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        let mut quote = None;
        let mut chars = line.char_indices().peekable();
        while let Some((at, c)) = chars.next() {
            match quote {
                Some(q) if c == '\\' => {
                    let _ = q;
                    chars.next();
                }
                Some(q) if c == q => quote = None,
                Some(_) => {}
                None if c == '"' || c == '\'' || c == '`' => quote = Some(c),
                None if line[at..].starts_with(marker) => {
                    found.push(start + at + marker.len()..start + line.trim_end().len());
                    break;
                }
                None => {}
            }
        }
        start += line.len();
    }
    found
}

/// The insides of each `/* … */` comment in `text`.
fn block_comments(text: &str) -> Vec<Range<usize>> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(open) = text[from..].find("/*").map(|at| from + at + 2) {
        let close = text[open..].find("*/").map_or(text.len(), |at| open + at);
        found.push(open..close);
        from = (close + 2).min(text.len());
    }
    found
}

/// Each string in the JSON `text`, as the key it is the value of (`None` for a key itself, and
/// for a string in an array, the array's key) and the range of its contents.
fn json_strings(text: &str) -> Vec<(Option<String>, Range<usize>)> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    // The key each open object or array is the value of.
    let mut keys: Vec<Option<String>> = Vec::new();
    let mut key: Option<String> = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                let open = i + 1;
                let mut j = open;
                while j < bytes.len() && bytes[j] != b'"' {
                    j += if bytes[j] == b'\\' { 2 } else { 1 };
                }
                let close = j.min(bytes.len());
                let after = text[(close + 1).min(text.len())..].trim_start();
                if after.starts_with(':') {
                    key = Some(text[open..close].to_owned());
                    found.push((None, open..close));
                } else {
                    let of = key.take().or_else(|| keys.last().cloned().flatten());
                    found.push((of, open..close));
                }
                i = close + 1;
            }
            b'{' | b'[' => {
                keys.push(key.take());
                i += 1;
            }
            b'}' | b']' => {
                keys.pop();
                i += 1;
            }
            b',' => {
                key = None;
                i += 1;
            }
            _ => i += 1,
        }
    }
    found
}

/// The values of the keys `names` in the TOML `text`, read line by line: a key's line from its
/// `=` on, held as prose if it is a description and for the old name only otherwise.
fn toml_values(text: &str, names: &[&str]) -> Vec<(Range<usize>, Held)> {
    let mut found = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        if let Some((key, _)) = line.split_once('=') {
            let key = key.trim();
            if names.contains(&key) {
                let value = start + line.find('=').unwrap_or(0) + 1;
                let end = start + line.trim_end().len();
                found.push((value..end, Held::OldName));
                // A description is prose, whose `code` is code formatting as in Markdown.
                if key == "description" {
                    found.extend(
                        markdown_prose(&text[value..end])
                            .into_iter()
                            .map(|range| (value + range.start..value + range.end, Held::Prose)),
                    );
                }
            }
        }
        start += line.len();
    }
    found
}

/// The landing page's problems: its first paragraph must name the suite on its own and each
/// shipped product.
fn landing_problems(root: &Path) -> Vec<String> {
    match std::fs::read_to_string(root.join(LANDING)) {
        Ok(text) => landing_problems_in(&text),
        Err(e) => vec![format!("{LANDING}: {e}")],
    }
}

/// The problems of the landing page whose text is `text`.
fn landing_problems_in(text: &str) -> Vec<String> {
    let mut paragraph = String::new();
    let mut inside = false;
    for event in Parser::new(text) {
        match event {
            Event::Start(Tag::Paragraph) => inside = true,
            Event::End(TagEnd::Paragraph) => break,
            Event::Text(words) | Event::Code(words) if inside => paragraph.push_str(&words),
            Event::SoftBreak | Event::HardBreak if inside => paragraph.push(' '),
            _ => {}
        }
    }
    let mut found = Vec::new();
    let mut rest = paragraph.clone();
    for product in SHIPPED {
        if !paragraph.contains(product) {
            found.push(format!(
                "{LANDING}: the first paragraph doesn't name {product}"
            ));
        }
        rest = rest.replace(product, "");
    }
    if !rest.contains(SUITE) {
        found.push(format!(
            "{LANDING}: the first paragraph doesn't name the suite, {SUITE}, on its own"
        ));
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The old name's two pieces, so that this file's own tests don't spell it.
    fn old() -> String {
        ["hpr", "sim"].join("-")
    }

    #[test]
    fn every_surface_names_the_product_by_its_name() {
        let root = crate::designs::root().unwrap();
        assert_eq!(problems(&root, ALLOW).unwrap(), Vec::<String>::new());
    }

    /// One mention planted in each kind of surface is found, by the rule that kind holds it to.
    #[test]
    fn a_mention_planted_in_each_kind_of_surface_fails() {
        let old = old();
        let nbh = old.replace('-', "\u{2011}");
        let planted: [(Kind, String); 14] = [
            (Kind::Markdown, format!("# Title\n\nWhat {old} is.\n")),
            (Kind::Markdown, "So hpr's apogees are low.\n".to_owned()),
            (Kind::Markdown, format!("See [the page](cli.md#{old}-x).\n")),
            (
                Kind::Text,
                format!(
                    "Copyright (c) 2026 the {} contributors\n",
                    old.to_uppercase()
                ),
            ),
            (
                Kind::Rust,
                "/// Flies the rocket, as hpr does.\nfn f() {}\n".to_owned(),
            ),
            (
                Kind::Rust,
                format!("fn f() {{ eprintln!(\"{nbh} could not\"); }}\n"),
            ),
            (
                Kind::Rust,
                "fn f() -> &'static str { \"a format hpr reads\" }\n".to_owned(),
            ),
            // After an escaped quote's character literal, a string is still read as one.
            (
                Kind::Rust,
                "fn f() -> [char; 2] { ['\\'','\"'] }\nfn g() -> &'static str { \"a format hpr reads\" }\n"
                    .to_owned(),
            ),
            (
                Kind::Python,
                "def f():\n    \"\"\"Fly it as hpr would.\"\"\"\n".to_owned(),
            ),
            (
                Kind::TypeScript,
                "// Types for hpr's format.\nexport type X = 1;\n".to_owned(),
            ),
            (
                Kind::Schema,
                "{\"title\": \"x\", \"description\": \"What hpr reads\"}".to_owned(),
            ),
            (Kind::Json, "{\"desc\": \"hpr, a simulator\"}".to_owned()),
            (
                Kind::CargoManifest,
                "[package]\ndescription = \"The hpr library.\"\n".to_owned(),
            ),
            (
                Kind::PyProject,
                "[project]\ndescription = \"Python for hpr\"\n".to_owned(),
            ),
        ];
        let more: Vec<(Kind, String)> = vec![
            // The old name in a JSON document's code span and as a key.
            (Kind::Json, format!("{{\"subtitle\": \"`{old}` 0.1\"}}")),
            (Kind::Json, format!("{{\"{old}\": \"x\"}}")),
            // An escape's letter before the name doesn't join it.
            (Kind::Rust, "fn f() { eprintln!(\"bad input:\\nhpr reads only .ork files\"); }\n".to_owned()),
            (Kind::Schema, "{\"description\": \"Line.\\nhpr reads it\"}".to_owned()),
            // A `#[cfg(test)]` field ends at its comma; what follows is read.
            (
                Kind::Rust,
                "struct S {\n    #[cfg(test)]\n    probe: bool,\n}\nimpl S { fn f() -> &'static str { \"a format hpr reads\" } }\n"
                    .to_owned(),
            ),
            // So does a last field, at its struct's closing brace, and a match arm.
            (
                Kind::Rust,
                "enum E {\n    #[cfg(test)]\n    Probe\n}\nfn f() -> &'static str { \"a format hpr reads\" }\n".to_owned(),
            ),
            (
                Kind::Rust,
                "fn g(x: u8) -> u8 { match x {\n #[cfg(test)]\n 9 => 1,\n _ => 0 } }\nfn f() -> &'static str { \"hpr reads it\" }\n".to_owned(),
            ),
            // A match arm under the attribute ends at its comma, so the next arm's block is read.
            (
                Kind::Rust,
                "fn g(x: u8) -> &'static str { match x {\n #[cfg(test)]\n 9 => \"\",\n _ => { \"hpr reads it\" } } }\n"
                    .to_owned(),
            ),
            // Block rustdoc is rustdoc.
            (Kind::Rust, "/** Flies as hpr does. */\nfn f() {}\n".to_owned()),
            (Kind::Rust, format!("/*! The {old} library. */\n")),
            // A dash and a space after `hpr` is no flag.
            (Kind::Rust, "fn f() -> &'static str { \"hpr - a rocket simulator\" }\n".to_owned()),
        ];
        for (kind, text) in planted.iter().chain(&more) {
            let mut used = [];
            let found = problems_in("x", *kind, text, &[], &mut used);
            assert_eq!(found.len(), 1, "{kind:?}: {text:?}: {found:?}");
            // Each planted text holds one mention, of the old name or a bare `hpr`.
            let lower = text.to_lowercase();
            let rule = if lower.contains(&old) || lower.contains(&nbh) {
                "the old name"
            } else {
                "a bare `hpr`"
            };
            assert!(found[0].contains(rule), "{kind:?}: {text:?}: {found:?}");
        }
    }

    /// Test-only code ships nothing: an item under `#[cfg(test)]` is skipped, and a module file
    /// declared under it is not a surface.
    #[test]
    fn test_only_code_is_not_read() {
        let skipped = [
            "#[cfg(test)]\nmod tests {\n    fn f() -> &'static str { \"hpr reads\" }\n}\n",
            "#[cfg(test)]\n#[allow(dead_code)]\nfn probe() -> &'static str { \"hpr reads\" }\n",
            "fn g(x: u8) -> u8 { match x {\n #[cfg(test)]\n 9 => { let _ = \"hpr reads\"; 1 }\n _ => 0 } }\n",
            // A comma in the item's own header doesn't end it.
            "#[cfg(test)]\nfn helper(a: u8, b: u8) -> &'static str { \"hpr reads\" }\n",
            "#[cfg(test)]\n#[allow(dead_code, unused)]\nfn probe() -> &'static str { \"hpr reads\" }\n",
            "#[cfg(test)]\nimpl<A, B> X<A, B> { fn f() -> &'static str { \"hpr reads\" } }\n",
            "#[cfg(test)]\nfn t() -> (u8, u8) { let _ = \"hpr reads\"; (1, 2) }\n",
        ];
        for text in skipped {
            let mut used = [];
            assert_eq!(
                problems_in("x", Kind::Rust, text, &[], &mut used),
                Vec::<String>::new(),
                "{text:?}"
            );
        }
        assert_eq!(
            rust_test_modules(
                "#[cfg(test)]\nmod tests;\n#[cfg(test)]\npub(super) mod probe;\nmod real;\n"
            ),
            ["tests", "probe"]
        );
        assert_eq!(
            module_paths("crates/x/src/lib.rs", "tests"),
            ["crates/x/src/tests.rs", "crates/x/src/tests/"]
        );
        assert_eq!(
            module_paths("crates/x/src/ork.rs", "tests"),
            ["crates/x/src/ork/tests.rs", "crates/x/src/ork/tests/"]
        );
    }

    /// Every kind of surface ADR-206 §1 lists is read at a file of the real tree, by its kind; an
    /// unpublished crate's sources and test-only module files are not.
    #[test]
    fn every_listed_surface_is_read() {
        let root = crate::designs::root().unwrap();
        let read = surfaces(&root, &tracked(&root).unwrap());
        let kind_of = |path: &str| {
            read.iter()
                .find(|(surface, _)| surface == path)
                .map(|(_, kind)| *kind)
        };
        let expected = [
            ("README.md", Kind::Markdown),
            ("docs/start-here.md", Kind::Markdown),
            ("CHANGELOG.md", Kind::Markdown),
            ("THIRD-PARTY-NOTICES.md", Kind::Markdown),
            ("crates/hpr-py/README.md", Kind::Markdown),
            ("LICENSE-MIT", Kind::Text),
            ("LICENSE-APACHE", Kind::Text),
            ("crates/hpr-sim/LICENSE-MIT", Kind::Text),
            ("crates/hpr-sim/LICENSE-APACHE", Kind::Text),
            ("scripts/release/licenses.hbs", Kind::Text),
            ("crates/hpr/Cargo.toml", Kind::CargoManifest),
            ("crates/hpr-cli/Cargo.toml", Kind::CargoManifest),
            ("crates/hpr-py/pyproject.toml", Kind::PyProject),
            (
                "crates/hpr-py/python/fusionspace/hpr/__init__.py",
                Kind::Python,
            ),
            ("crates/hpr/src/lib.rs", Kind::Rust),
            ("crates/hpr-cli/src/main.rs", Kind::Rust),
            ("crates/hpr-aero/src/lib.rs", Kind::Rust),
            ("crates/hpr-py/src/rocket.rs", Kind::Rust),
            ("schema/format/hpr-design-0.2.schema.json", Kind::Schema),
            ("schema/cli/sim.schema.json", Kind::Schema),
            ("schema/format/python/hpr_design.py", Kind::Python),
            ("schema/format/typescript/hpr-design.ts", Kind::TypeScript),
            (".github/brand/project.json", Kind::Json),
        ];
        for (path, kind) in expected {
            assert_eq!(kind_of(path), Some(kind), "{path}");
        }
        for path in [
            "crates/hpr-validate/src/lib.rs",
            "crates/hpr-ffi/src/lib.rs",
            "crates/hpr-sim/src/tests.rs",
            "crates/hpr-io/src/ork/tests.rs",
            "crates/hpr-sim/tests/cli.rs",
            "docs/cli.md",
        ] {
            assert_eq!(kind_of(path), None, "{path}");
        }
        // Every published crate's sources are read.
        for (path, _) in read
            .iter()
            .filter(|(path, _)| path.ends_with("/Cargo.toml"))
        {
            let manifest = std::fs::read_to_string(root.join(path)).unwrap();
            let lib = path.replace("Cargo.toml", "src/lib.rs");
            if published(&manifest) && root.join(&lib).exists() {
                assert_eq!(kind_of(&lib), Some(Kind::Rust), "{lib}");
            }
        }
    }

    /// Names joined to `hpr`, code formatting, the command in a message and the packages' names
    /// are not mentions.
    #[test]
    fn other_names_and_code_are_not_mentions() {
        let old = old();
        let clean: [(Kind, String); 11] = [
            (
                Kind::Markdown,
                "Run `hpr sim`; read `.hpr`, hpr.fusionspace.co, `hpr::sim`, hpr-core, hpr_sim.\n"
                    .to_owned(),
            ),
            (Kind::Markdown, "```sh\nhpr sim x.ork\n```\n\nThe HPR Sim reads it.\n".to_owned()),
            (Kind::Markdown, "Install `fusionspace-hpr-sim` or @fusionspace/hpr.\n".to_owned()),
            (
                Kind::Rust,
                "/// What `hpr sim` reads:\n///\n/// ```\n/// use hpr::Rocket;\n/// ```\nfn f() {}\n"
                    .to_owned(),
            ),
            (
                Kind::Rust,
                "fn f() { let _ = (\"hpr\", \"run hpr sim\", \"hpr {} writes\", \"hpr --json\"); }\n"
                    .to_owned(),
            ),
            (Kind::Rust, "fn f<'a>(x: &'a str) -> char { let _ = x; 'h' }\n".to_owned()),
            (Kind::Rust, "// a plain comment says hpr's own\nfn f() {}\n".to_owned()),
            (Kind::Python, "from fusionspace import hpr\n\"\"\"Uses ``hpr.Rocket``.\"\"\"\n".to_owned()),
            (Kind::CargoManifest, "name = \"fusionspace-hpr-sim\"\nhpr = { path = \"../hpr\" }\n".to_owned()),
            (Kind::Text, format!("see fusionspace-{old} and fusionspace\u{2011}{old}\n")),
            // A plain block comment, with any character in it, is no surface.
            (Kind::Rust, "/* 15° in Zürich, hpr's own */\nfn f() {}\n".to_owned()),
        ];
        for (kind, text) in &clean {
            let mut used = [];
            assert_eq!(
                problems_in("x", *kind, text, &[], &mut used),
                Vec::<String>::new(),
                "{kind:?}: {text:?}"
            );
        }
    }

    /// An entry covers only the mentions inside its own text in its own file.
    #[test]
    fn an_entry_covers_only_its_text_in_its_file() {
        let old = old();
        let allow = [Allow {
            file: "README.md",
            scope: Scope::Text("(called hpr-sim before release 0.1)"),
            count: 1,
            why: "history",
        }];
        let line = format!("HPR Sim (called {old} before release 0.1) flies.\n");
        let mut used = [0];
        assert_eq!(
            problems_in("README.md", Kind::Markdown, &line, &allow, &mut used),
            Vec::<String>::new()
        );
        assert_eq!(used, [1]);
        let grown = format!("HPR Sim (called {old} before release 0.1) and {old} fly.\n");
        assert_eq!(
            problems_in("README.md", Kind::Markdown, &grown, &allow, &mut used).len(),
            1
        );
        assert_eq!(
            problems_in("CHANGELOG.md", Kind::Markdown, &line, &allow, &mut used).len(),
            1
        );
    }

    /// Dropping any one entry from the real allowlist leaves its mentions to fail.
    #[test]
    fn dropping_an_entry_fails() {
        let read = read(&crate::designs::root().unwrap()).unwrap();
        for index in 0..ALLOW.len() {
            let mut fewer = ALLOW.to_vec();
            let dropped = fewer.remove(index);
            let found = problems_of(&read, &fewer);
            assert!(
                found
                    .iter()
                    .any(|problem| problem.starts_with(dropped.file)),
                "dropping {:?} found nothing: {found:?}",
                dropped.scope
            );
        }
    }

    /// An entry widened to a whole page fails, as a page and as the page's text; an entry that
    /// covers nothing fails as stale.
    #[test]
    fn an_entry_widened_to_a_page_or_stale_fails() {
        let root = crate::designs::root().unwrap();
        let read = read(&root).unwrap();
        let files = read.files.clone();
        let mut wide = ALLOW.to_vec();
        wide[1].scope = Scope::Page;
        let found = problems_of(&read, &wide);
        assert!(
            found.iter().any(|p| p.contains("may be allowed whole")),
            "{found:?}"
        );
        let readme: &'static str = Box::leak(
            std::fs::read_to_string(root.join("README.md"))
                .unwrap()
                .into_boxed_str(),
        );
        let mut wide = ALLOW.to_vec();
        wide[1].scope = Scope::Text(readme);
        let found = allowlist_problems(&wide, &files);
        assert!(
            found.iter().any(|p| p.contains("wider than a line")),
            "{found:?}"
        );
        // An ADR and the roadmap's archive may be allowed whole; a later ADR may not.
        assert!(may_be_whole(
            "docs/decisions/0205-the-2026-10-08-one-name-one-design.md"
        ));
        assert!(may_be_whole("docs/roadmap-done.md"));
        assert!(!may_be_whole("docs/decisions/0206-x.md"));
        assert!(!may_be_whole("docs/start-here.md"));
        let mut stale = ALLOW.to_vec();
        stale.push(Allow {
            file: "README.md",
            scope: Scope::Text("the hpr-sim of nowhere"),
            count: 1,
            why: "a test",
        });
        let found = problems_of(&read, &stale);
        assert!(
            found.iter().any(|p| p.contains("covers nothing")),
            "{found:?}"
        );
        // An entry no wider than its mention would allow every one in its file.
        let mut bare = ALLOW.to_vec();
        bare[2].scope = Scope::Text("`hpr-sim`");
        let found = allowlist_problems(&bare, &files);
        assert!(
            found
                .iter()
                .any(|p| p.contains("little more than its mention")),
            "{found:?}"
        );
        // One more mention in an entry's text than it says fails, and so does one fewer.
        for change in [1_isize, -1] {
            let mut counted = ALLOW.to_vec();
            counted[2].count = counted[2].count.saturating_add_signed(change);
            let found = problems_of(&read, &counted);
            assert!(
                found.iter().any(|p| p.contains("it says")),
                "{change}: {found:?}"
            );
        }
        let mut empty = ALLOW.to_vec();
        empty.push(Allow {
            file: "README.md",
            scope: Scope::Text("FusionSpace"),
            count: 1,
            why: "",
        });
        let found = allowlist_problems(&empty, &files);
        assert!(found.iter().any(|p| p.contains("no reason")), "{found:?}");
        assert!(found.iter().any(|p| p.contains("no mention")), "{found:?}");
    }

    /// The landing page's first paragraph names the suite on its own and each shipped product.
    #[test]
    fn the_landing_page_names_the_suite_and_each_product() {
        let root = crate::designs::root().unwrap();
        assert_eq!(landing_problems(&root), Vec::<String>::new());
        let good = "# Start here\n\n**FusionSpace HPR** is a suite; FusionSpace HPR · Sim, its\n\
                    first product, flies rockets.\n\nMore.\n";
        assert_eq!(landing_problems_in(good), Vec::<String>::new());
        let product_only = "# Start here\n\nFusionSpace HPR · Sim flies rockets.\n";
        assert_eq!(landing_problems_in(product_only).len(), 1);
        let suite_only = "# Start here\n\nFusionSpace HPR is a suite.\n";
        assert_eq!(landing_problems_in(suite_only).len(), SHIPPED.len());
        let later = "# Start here\n\nA suite.\n\nFusionSpace HPR · Sim and FusionSpace HPR.\n";
        assert_eq!(landing_problems_in(later).len(), SHIPPED.len() + 1);
    }
}
