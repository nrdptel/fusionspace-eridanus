//! Tests of `scripts/lanes.sh`, which runs the gate's `refs` checks side by side, and of the
//! step's line in `scripts/gate.sh` that hands them to it.
//!
//! The checks under `refs/` each read their own files and write none, so side by side the step
//! takes as long as its longest check, not their sum: 207 s against 474 to 489 s on the machine
//! the gate runs on (2026-11-14, issue 700). What these tests hold is what running them at once
//! could lose: a failed check must still fail the step, every check must still be run, and the
//! step's log must read the same from run to run.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    // xtask's manifest folder is the workspace root's child.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// The checks the `refs` step runs, each as the lane the step's line must name.
const REFS_CHECKS: [&str; 12] = [
    "'$XTASK ork-flights --check'",
    "'$XTASK roll-drev --check'",
    "'$XTASK ork-flights --library --check'",
    "'$XTASK real-flights --check'",
    "'$XTASK public-predictions --check'",
    "'$XTASK ghost --check'",
    "'$XTASK ork-cli --check'",
    "'$XTASK fixture-flights --check'",
    "'$XTASK ork-export-flights --check'",
    "'$XTASK logs --check'",
    "'$XTASK evidence --samples'",
    "'node validation/oracles/thrustcurve/analyze_stats.js --recount'",
];

#[test]
fn the_refs_step_runs_every_check_as_a_lane_of_its_own() {
    let gate = fs::read_to_string(root().join("scripts/gate.sh")).unwrap();
    let step = gate
        .lines()
        .find(|line| line.trim_start().starts_with("refs) echo"))
        .unwrap();
    // xtask is built once, before the lanes start, so twelve `cargo run`s don't each build it.
    let lanes = step
        .split_once("cargo build --locked --quiet --package xtask && scripts/lanes.sh ")
        .unwrap_or_else(|| panic!("{step}"))
        .1;
    let mut rest = lanes;
    for check in REFS_CHECKS {
        let at = rest
            .find(check)
            .unwrap_or_else(|| panic!("{check} in {step}"));
        rest = &rest[at + check.len()..];
    }
    // Nothing is chained after the lanes: a check there would run only when every lane passed.
    assert_eq!(rest.trim_end(), "\" ;;", "{step}");
    assert!(!lanes.contains("&&"), "{step}");
    assert_eq!(lanes.matches('\'').count(), 2 * REFS_CHECKS.len(), "{step}");
}

#[cfg(unix)]
mod run {
    use super::root;
    use std::fs;
    use std::path::PathBuf;
    use std::process::{Command, Output};

    fn lanes(name: &str, commands: &[&str]) -> (Output, PathBuf) {
        let dir = std::env::temp_dir().join(format!("hpr-lanes-{}-{name}", std::process::id()));
        let out = Command::new("bash")
            .arg(root().join("scripts/lanes.sh"))
            .args(commands)
            .env("LANES_DIR", &dir)
            .output()
            .unwrap();
        (out, dir)
    }

    #[test]
    fn a_failed_lane_fails_the_run_and_is_named_last() {
        // The slow lane is first and the failing one in the middle: the output is still in the
        // order the commands were given, and the run still fails.
        let (out, dir) = lanes(
            "fail",
            &[
                "sleep 0.3; echo one",
                "echo two; echo oops >&2; exit 3",
                "echo three",
            ],
        );
        let text = String::from_utf8(out.stdout).unwrap();
        assert_eq!(out.status.code(), Some(1), "{text}");
        let order = [
            "== lane 1 (PASS, ",
            "\none\n",
            "== lane 2 (FAIL exit 3, ",
            "\ntwo\noops\n",
            "== lane 3 (PASS, ",
            "\nthree\n",
            "error: lane 2 failed (exit 3, ",
        ];
        let mut at = 0;
        for piece in order {
            at += text[at..]
                .find(piece)
                .unwrap_or_else(|| panic!("{piece:?} in {text}"));
        }
        let last = text.lines().last().unwrap();
        assert!(
            last.starts_with("error: lane 2 failed (exit 3, ")
                && last.ends_with("/2.log): echo two; echo oops >&2; exit 3"),
            "{last}"
        );
        // Each lane's output is kept in a file of its own.
        assert_eq!(fs::read_to_string(dir.join("3.log")).unwrap(), "three\n");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn passing_lanes_pass_and_a_killed_one_fails() {
        let (out, dir) = lanes("pass", &["echo one", "sleep 0.2; echo two"]);
        let text = String::from_utf8(out.stdout).unwrap();
        assert_eq!(out.status.code(), Some(0), "{text}");
        assert!(!text.contains("error:"), "{text}");
        assert!(
            text.contains("== lane 2 (PASS, ") && text.ends_with("two\n"),
            "{text}"
        );
        fs::remove_dir_all(dir).unwrap();

        // A lane killed outright never returns a status of its own choosing.
        let (out, dir) = lanes("killed", &["echo one", "kill -9 $$"]);
        let text = String::from_utf8(out.stdout).unwrap();
        assert_eq!(out.status.code(), Some(1), "{text}");
        assert!(text.contains("error: lane 2 failed (exit 137, "), "{text}");
        fs::remove_dir_all(dir).unwrap();

        // No command at all is a usage error, not a pass.
        let (out, _) = lanes("none", &[]);
        assert_eq!(out.status.code(), Some(64));
    }
}
