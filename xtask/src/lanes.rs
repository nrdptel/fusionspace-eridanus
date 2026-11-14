//! Tests of `scripts/lanes.sh`, which runs the gate's `refs` checks side by side, and of the
//! step's line in `scripts/gate.sh` that hands them to it.
//!
//! The checks under `refs/` read their own files, and the three that write anything (`ghost`,
//! `ork-cli`, and the tests `evidence --samples` builds) write folders no other check touches. So
//! side by side the step takes as long as its longest check, not their sum: 207 s against 474
//! to 489 s on the machine the gate runs on (2026-11-14, issue 700). What these tests hold is
//! what running them at once could lose: a failed check must still fail the step, every check
//! must still be run, the step's log must read the same from run to run, and a run must never
//! read a result another run left.

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
    use std::process::{Command, Output, Stdio};
    use std::thread::sleep;
    use std::time::Duration;

    fn runner(dir: &PathBuf, commands: &[&str]) -> Command {
        let mut runner = Command::new("bash");
        runner
            .arg(root().join("scripts/lanes.sh"))
            .args(commands)
            .env("LANES_DIR", dir);
        runner
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("hpr-lanes-{}-{name}", std::process::id()));
        // A folder an earlier, stopped test run left would only hold old runs' folders.
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn lanes(dir: &PathBuf, commands: &[&str]) -> (Output, String) {
        let out = runner(dir, commands).output().unwrap();
        let text = String::from_utf8(out.stdout.clone()).unwrap();
        (out, text)
    }

    #[test]
    fn a_failed_lane_fails_the_run_and_is_named_last() {
        // The slow lane is first and the failing one in the middle: the output is still in the
        // order the commands were given, and the run still fails. The failing lane's output ends
        // without a newline, which must not glue the next line to it.
        let dir = scratch("fail");
        let (out, text) = lanes(
            &dir,
            &[
                "sleep 0.3; echo one",
                "echo two; printf oops >&2; exit 3",
                "echo three",
            ],
        );
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
                && last.ends_with("/2.log): echo two; printf oops >&2; exit 3"),
            "{last}"
        );
        // Each lane's output is kept in a file of its own, in the one folder this run made.
        let runs: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        assert_eq!(runs.len(), 1, "{runs:?}");
        assert_eq!(
            fs::read_to_string(runs[0].join("3.log")).unwrap(),
            "three\n"
        );
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn passing_lanes_pass_and_a_killed_one_fails() {
        let dir = scratch("pass");
        let (out, text) = lanes(&dir, &["echo one", "sleep 0.2; echo two"]);
        assert_eq!(out.status.code(), Some(0), "{text}");
        assert!(!text.contains("error:"), "{text}");
        assert!(
            text.contains("== lane 2 (PASS, ") && text.ends_with("two\n"),
            "{text}"
        );

        // A lane's command killed outright: its status is the signal's.
        let (out, text) = lanes(&dir, &["echo one", "kill -9 $$"]);
        assert_eq!(out.status.code(), Some(1), "{text}");
        assert!(text.contains("error: lane 2 failed (exit 137, "), "{text}");

        // The lane's own shell killed, so that it writes no result: still a failure, never a pass.
        let (out, text) = lanes(&dir, &["echo one", "kill -9 $PPID"]);
        assert_eq!(out.status.code(), Some(1), "{text}");
        assert!(text.contains("== lane 2 (FAIL exit 137, ?s)"), "{text}");

        // No command at all is a usage error, not a pass.
        let (out, _) = lanes(&dir, &[]);
        assert_eq!(out.status.code(), Some(64));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn lanes_run_at_the_same_time() {
        // The first lane ends only once the second has made its file: one after another, the
        // first would give up and fail.
        let dir = scratch("together");
        let go = dir.join("go");
        let wait = format!(
            "i=0; while [ ! -f {0} ] && [ $i -lt 200 ]; do sleep 0.05; i=$((i+1)); done; [ -f {0} ]",
            go.display()
        );
        let (out, text) = lanes(&dir, &[&wait, &format!("touch {}", go.display())]);
        assert_eq!(out.status.code(), Some(0), "{text}");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn two_runs_at_once_keep_their_own_results() {
        // The first run's one lane passes half a second in. The second run starts meanwhile, in
        // the same folder, and its first lane fails at once: it must not read the other's pass.
        let dir = scratch("twice");
        let mut first = runner(&dir, &["sleep 0.5"])
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        sleep(Duration::from_millis(100));
        let (out, text) = lanes(&dir, &["false", "sleep 1"]);
        assert_eq!(out.status.code(), Some(1), "{text}");
        assert!(text.contains("== lane 1 (FAIL exit 1, "), "{text}");
        // And the second run's start left the first run's folder alone.
        assert_eq!(first.wait().unwrap().code(), Some(0));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_stopped_run_stops_its_lanes_and_leaves_the_next_run_nothing_to_read() {
        for signal in ["-INT", "-TERM"] {
            let dir = scratch(&format!("stop{signal}"));
            let marker = dir.join("marker");
            // The stopped run's lane would pass, and write its marker, a second later.
            let late = format!("sleep 1; touch {}", marker.display());
            let mut first = runner(&dir, &[&late])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            sleep(Duration::from_millis(300));
            let killed = Command::new("kill")
                .args([signal, &first.id().to_string()])
                .status()
                .unwrap();
            assert!(killed.success());
            assert_eq!(first.wait().unwrap().code(), Some(130), "{signal}");
            // The next run, in the same folder, fails on its own failing lane, whatever the first
            // run's lane would have written by now.
            let (out, text) = lanes(&dir, &["false", "sleep 1.2"]);
            assert_eq!(out.status.code(), Some(1), "{signal}: {text}");
            assert!(
                text.contains("== lane 1 (FAIL exit 1, "),
                "{signal}: {text}"
            );
            // And the first run's lane was stopped with it: its marker never appears.
            assert!(!marker.exists(), "{signal}: the stopped run's lane ran on");
            fs::remove_dir_all(dir).unwrap();
        }
    }
}
