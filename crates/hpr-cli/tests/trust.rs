//! How far to trust a result (ADR-211; issue #379): `hpr sim` and `hpr mc` say under the
//! design's name that their figures are simulated, and end their result, on standard output, with
//! the three-part note; `--json` carries the same note as `trust`, beside `kind`. The note's
//! numbers are held to the committed report by `hpr_cli::trust`'s unit tests.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests run the binary and read its inputs; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::{Path, PathBuf};

use serde_json::Value;

/// The level 1 guide's rocket, which flies offline.
const LEVEL_1: &str = "validation/fixtures/ork/guides/level-1.ork";

/// The note's last line: its link.
const MORE: &str = "More: https://hpr.fusionspace.co/accuracy.html";

/// A file of the repository, by its path from the root.
fn repo_file(path: &str) -> String {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    root.join(path).to_string_lossy().into_owned()
}

/// Runs `hpr` with `args`, offline with an empty cache and no color, expects success, and
/// returns standard output and standard error.
fn run(args: &[&str]) -> (String, String) {
    let cache = tempfile::tempdir().unwrap();
    let mut command = assert_cmd::cargo::cargo_bin_cmd!("hpr");
    command
        .args(args)
        .env("HPR_CACHE_DIR", cache.path())
        .env("HPR_OFFLINE", "1");
    for variable in ["NO_COLOR", "FORCE_COLOR", "CLICOLOR_FORCE"] {
        command.env_remove(variable);
    }
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
    (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// The note at the end of `out`: the lines from its label to the end, joined as one paragraph,
/// its link on the last line.
fn closing_note(out: &str) -> String {
    let lines: Vec<&str> = out.lines().collect();
    let first = lines
        .iter()
        .rposition(|line| line.starts_with("How far to trust it."));
    assert!(first.is_some(), "no note: {out}");
    let first = first.unwrap();
    assert_eq!(lines.last(), Some(&MORE), "{out}");
    assert!(first > 0 && lines[first - 1].is_empty(), "{out}");
    for line in &lines[first..] {
        assert!(line.chars().count() <= 100, "{line}");
    }
    lines[first..].join(" ")
}

#[test]
fn sim_says_its_figures_are_simulated_and_ends_with_how_far_to_trust_them() {
    let design = repo_file(LEVEL_1);
    let (out, err) = run(&["sim", &design]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[2], "a simulated flight, not a measurement", "{out}");
    let note = closing_note(&out);
    // The three parts, in order: the kind, what it was checked against, what to rely on instead.
    let parts = [
        "Simulated from the design file, not measured.",
        "Against 55 logged flights of fliers' own designs, the simulated apogee averaged 9.8% \
         above the altimeter's; it was within 10% on 27 and low on 14, by at most 20%.",
        "Plan a waiver or a field's ceiling with room above this apogee; the altimeter's \
         reading is the one to log, and the RSO decides.",
    ];
    let mut from = 0;
    for part in parts {
        let at = note[from..]
            .find(part)
            .unwrap_or_else(|| panic!("{part:?} after {from}: {note}"));
        from += at + part.len();
    }
    // The note replaced the `help:` line that pointed at the Accuracy page.
    assert!(!err.contains("Accuracy page"), "{err}");
}

#[test]
fn mc_says_its_spread_is_the_inputs_not_the_models() {
    let design = repo_file(LEVEL_1);
    let (out, _) = run(&["mc", &design, "--runs", "3", "--seed", "1"]);
    assert!(out.contains("\n3 simulated flights, seed 1: "), "{out}");
    let note = closing_note(&out);
    assert!(
        note.contains(
            "not measured; the spread above comes from the inputs' scatter alone, not from the \
             model's error."
        ),
        "{note}"
    );
    assert!(note.contains("55 logged flights"), "{note}");
}

#[test]
fn json_carries_the_kind_and_the_same_note() {
    let design = repo_file(LEVEL_1);
    for args in [
        vec!["sim", design.as_str()],
        vec!["mc", design.as_str(), "--runs", "3", "--seed", "1"],
    ] {
        let (text, _) = run(&args);
        let mut json_args = args.clone();
        json_args.push("--json");
        let (json, err) = run(&json_args);
        assert!(err.is_empty(), "{err}");
        let document: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(document["kind"], "simulated", "{args:?}");
        let trust = document["trust"].as_str().unwrap();
        // The text form wraps the note and puts its link on a line of its own.
        assert_eq!(closing_note(&text), trust, "{args:?}");
    }
}
