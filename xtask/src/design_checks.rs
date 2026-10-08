//! `cargo xtask design-checks [--dir <path>]…`: runs the design checks
//! ([`hpr_design::checks::check`]) on every `.ork` design in the reference library and counts
//! their findings by kind.
//!
//! A change to a check is weighed by what it does to real designs: how many errors it adds (each
//! one refuses a flight in `hpr sim` until the design is fixed) and how many warnings. This prints
//! those counts, which may be published; the per-file detail names files in private corpora
//! (CONTRIBUTING.md rule 4), so it goes to `corpus-out/design-checks.json`, which is gitignored.
//!
//! With no `--dir` it reads [`crate::ork`]'s default survey: every `.ork` under `refs/` but the
//! scratch tree and the private flight collection, then OpenRocket's examples inside its jar.
//! Pass `--dir refs/hpr-sim-fixtures` to read the collection.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::ork::{collect_dir, default_survey, print_counts, root};

pub const USAGE: &str = "\
  design-checks [--dir <path>]…
                           Run the design checks on every .ork design in the reference
                           library (or under each --dir) and count their findings by kind.
                           Per-file detail (private corpus) goes to
                           corpus-out/design-checks.json.";

/// Where the per-file detail goes. Gitignored: it names files in private corpora.
const REPORT: &str = "corpus-out/design-checks.json";

pub fn run(args: &[String]) -> Result<(), String> {
    let root = root()?;
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut rest = args.iter();
    while let Some(argument) = rest.next() {
        match argument.as_str() {
            "--dir" => dirs.push(PathBuf::from(
                rest.next().ok_or_else(|| format!("usage:\n{USAGE}"))?,
            )),
            _ => return Err(format!("usage:\n{USAGE}")),
        }
    }
    let mut files = Vec::new();
    if dirs.is_empty() {
        files = default_survey(&root)?;
    } else {
        for dir in &dirs {
            collect_dir(dir, &mut files)?;
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    if files.is_empty() {
        return Err(
            "no .ork files found: the reference library is fetched by `cargo xtask refs fetch`"
                .to_owned(),
        );
    }

    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let mut outcomes: BTreeMap<&str, usize> = BTreeMap::new();
    let mut detail = Vec::new();
    for (name, bytes) in &files {
        let (outcome, findings) = match checked(bytes) {
            Ok(findings) => ("checked", findings),
            Err(why) => {
                detail.push(json!({ "file": name, "not_checked": why }));
                *outcomes.entry("not checked").or_default() += 1;
                continue;
            }
        };
        *outcomes.entry(outcome).or_default() += 1;
        for finding in &findings {
            *kinds.entry(kind(finding)).or_default() += 1;
        }
        detail.push(json!({ "file": name, "findings": findings }));
    }

    println!("{} .ork files", files.len());
    print_counts("Designs", &outcomes);
    print_counts("Findings by kind", &kinds);
    let report = root.join(REPORT);
    if let Some(parent) = report.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(&Value::Array(detail))
        .map_err(|error| format!("{REPORT}: {error}"))?;
    fs::write(&report, text).map_err(|error| format!("{}: {error}", report.display()))?;
    println!("\nPer-file detail: {REPORT}");
    Ok(())
}

/// The findings of one file's design, or why it has none: the file doesn't read, or its design
/// can't be laid out.
fn checked(bytes: &[u8]) -> Result<Vec<Value>, String> {
    let file = hpr_io::ork::read(bytes).map_err(|error| format!("read: {error}"))?;
    let design = hpr_io::ork::design(&file.value).value;
    let findings =
        hpr_design::checks::check(&design.rocket).map_err(|error| format!("layout: {error}"))?;
    findings
        .iter()
        .map(|finding| serde_json::to_value(finding).map_err(|error| error.to_string()))
        .collect()
}

/// A finding's kind as it serializes, such as `internal_part_wider_than_parent`.
fn kind(finding: &Value) -> String {
    finding["kind"].as_str().unwrap_or("unknown").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A finding is counted under the kind it serializes with.
    #[test]
    fn a_finding_counts_under_its_serialized_kind() {
        let finding = hpr_design::checks::Finding::NoNoseCone {
            component: "airframe".to_owned(),
        };
        assert_eq!(
            kind(&serde_json::to_value(&finding).unwrap()),
            "no_nose_cone"
        );
    }
}
