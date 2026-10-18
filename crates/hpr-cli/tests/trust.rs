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
        "Against 53 logged flights of fliers' own designs, each logged apogee the height climbed \
         in the day's air, the simulated apogee averaged 8.7% above the logged one; it was within \
         10% on 26 and read low on 13, by at most 20%.",
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
    assert!(note.contains("53 logged flights"), "{note}");
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

/// The three parts of a note, each found after the one before.
fn in_order(note: &str, parts: &[&str]) {
    let mut from = 0;
    for part in parts {
        let at = note[from..].find(part);
        assert!(at.is_some(), "{part:?} after {from}: {note}");
        from += at.unwrap() + part.len();
    }
}

/// `hpr analyze` ends with how far to trust a log's readings (issue #395, ADR-213): measured,
/// what HPR Sim's readings and their bounds were checked on, and the altimeter's own reading to
/// log; `--json` carries it as `trust`, beside `kind: "measured"`.
#[test]
fn analyze_says_its_readings_are_measured_and_how_far_to_trust_them() {
    let log = repo_file("validation/fixtures/logs/synthetic-pnut.pf2");
    let (out, _) = run(&["analyze", &log]);
    let note = closing_note_at(&out, "reading-a-flight-log.html");
    in_order(
        &note,
        &[
            "Measured by the logger, read by HPR Sim from its log, not simulated.",
            "Each reading states the bound its error stayed within on HPR Sim's synthetic \
             flights, logged at every rate from 500 to 0.5 samples a second and in steps up to 10 \
             m or 100 Pa, in air within 15 K of standard;",
            "withheld from Mach 0.8,",
            "the altimeter's own reading is the one to log, and the RSO decides.",
        ],
    );
    let (json, _) = run(&["analyze", &log, "--json"]);
    let document: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(document["kind"], "measured");
    assert_eq!(document["trust"], note.as_str());
}

/// A log read by the flyer's own mapping or layout ends with its own note: the logger's samples
/// in the units the file states, and readings whose bounds hold only if those units are right.
#[test]
fn analyze_says_a_log_read_by_your_own_mapping_is_as_right_as_its_units() {
    let folder = "validation/fixtures/logs/own-logger";
    for (log, by, file) in [
        ("gps-track.csv", "--mapping", "gps-track.mapping.json"),
        ("flight.bin", "--layout", "flight.layout.json"),
    ] {
        let log = repo_file(&format!("{folder}/{log}"));
        let file = repo_file(&format!("{folder}/{file}"));
        let (out, _) = run(&["analyze", &log, by, &file]);
        let note = closing_note_at(&out, "your-own-logger.html");
        in_order(
            &note,
            &[
                "The logger's own samples, read by HPR Sim in the units your mapping, layout or \
                 FIP header states, not simulated: as right as those units are.",
                "Each reading states the bound its error stayed within",
                "Those bounds hold only if the units are right: check a unit against a known \
                 height,",
            ],
        );
        // A GPS tracker's table has no barometer: the note doesn't say one measured it.
        assert!(!note.contains("altimeter's barometer"), "{note}");
        let (json, _) = run(&["analyze", &log, by, &file, "--json"]);
        let document: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(document["kind"], "measured");
        assert_eq!(document["trust"], note.as_str());
    }
}

/// `hpr motors show` ends with where its figures come from and what to go by instead: the
/// catalog's download date for a catalog motor, the file for a motor file; `--json` carries it
/// as `trust`, beside `kind: "copied"`.
#[test]
fn motors_show_says_where_its_figures_come_from() {
    let (out, _) = run(&["motors", "show", "H170M"]);
    let note = closing_note_at(&out, "pick-a-motor.html");
    let captured = hpr::hpr_motor::catalog::Catalog::bundled()
        .unwrap()
        .snapshot
        .captured;
    in_order(
        &note,
        &[
            &format!(
                "Copied from ThrustCurve.org's curve file, downloaded {captured}, not measured by \
                 HPR Sim:"
            ),
            "The total impulse and peak thrust match OpenRocket's reading of the same file on \
             every bundled curve, a check of the arithmetic, not of the motor; no figure is \
             checked against the maker's or the certifying bodies' data.",
            "The motor's printed data and its maker's instructions come first, and the RSO \
             decides.",
        ],
    );
    let file = repo_file("crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000724.eng");
    let (out, _) = run(&["motors", "show", &file]);
    let file_note = closing_note_at(&out, "pick-a-motor.html");
    in_order(
        &file_note,
        &[
            "Read from the motor file named above, not measured by HPR Sim:",
            "HPR Sim doesn't check a motor file against the maker's or the certifying bodies' data.",
            "The motor's printed data",
        ],
    );
    for (args, expected) in [
        (vec!["motors", "show", "H170M", "--json"], &note),
        (vec!["motors", "show", file.as_str(), "--json"], &file_note),
    ] {
        let (json, _) = run(&args);
        let document: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(document["kind"], "copied", "{args:?}");
        assert_eq!(document["trust"], expected.as_str(), "{args:?}");
    }
}

/// The note at the end of `out`, as [`closing_note`] reads it, but with its link to `page`.
fn closing_note_at(out: &str, page: &str) -> String {
    let lines: Vec<&str> = out.lines().collect();
    let first = lines
        .iter()
        .rposition(|line| line.starts_with("How far to trust it."));
    assert!(first.is_some(), "no note: {out}");
    let first = first.unwrap();
    let more = format!("More: https://hpr.fusionspace.co/{page}");
    assert_eq!(lines.last(), Some(&more.as_str()), "{out}");
    assert!(first > 0 && lines[first - 1].is_empty(), "{out}");
    for line in &lines[first..] {
        assert!(line.chars().count() <= 100, "{line}");
    }
    let page = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(page.replace(".html", ".md"));
    assert!(page.is_file(), "{}", page.display());
    lines[first..].join(" ")
}
