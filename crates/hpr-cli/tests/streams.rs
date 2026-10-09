//! Which stream each line goes to (`product/cli.md`, *Output*; issue #377): a command's result
//! alone on standard output, and every `warning:`, `note:` and `help:` line beside it on standard
//! error, so `hpr sim rocket.ork > flight.txt` writes the flight to the file and the warnings to
//! the terminal. One test for each command that prints such lines, run as a user runs the
//! binary, and one that `--json` is unchanged: one document on standard output, carrying the
//! warnings in its own fields, and nothing on standard error.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests run the binary and write its inputs; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::{Path, PathBuf};

use serde_json::Value;

/// The level 1 guide's rocket, which flies offline and prints `warning:`, `note:` and `help:`
/// lines.
const LEVEL_1: &str = "validation/fixtures/ork/guides/level-1.ork";
/// The prefixes a diagnostic line starts with.
const PREFIXES: [&str; 3] = ["warning: ", "note: ", "help: "];

/// The repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A file of the repository, by its path from the root.
fn repo_file(path: &str) -> String {
    root().join(path).to_string_lossy().into_owned()
}

/// `text` with its color codes, `ESC [ … m`, taken out.
fn plain(text: &str) -> String {
    let mut plain = String::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            plain.push(c);
        }
    }
    plain
}

/// The prefix `line` starts with, its color aside, if it is a diagnostic.
fn prefix(line: &str) -> Option<&'static str> {
    let line = plain(line);
    PREFIXES.into_iter().find(|prefix| line.starts_with(prefix))
}

/// What one run printed.
struct Printed {
    /// Standard output.
    out: String,
    /// Standard error.
    err: String,
}

/// Runs `hpr` with `args`, offline with `cache` as its cache and no color variable set, expects
/// success, and returns both streams.
fn run_cached(args: &[&str], cache: &Path) -> Printed {
    let mut command = assert_cmd::cargo::cargo_bin_cmd!("hpr");
    command
        .args(args)
        .env("HPR_CACHE_DIR", cache)
        .env("HPR_OFFLINE", "1");
    for variable in ["NO_COLOR", "FORCE_COLOR", "CLICOLOR_FORCE"] {
        command.env_remove(variable);
    }
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
    Printed {
        out: String::from_utf8(output.stdout).unwrap(),
        err: String::from_utf8(output.stderr).unwrap(),
    }
}

/// As [`run_cached`], with an empty cache.
fn run(args: &[&str]) -> Printed {
    let cache = tempfile::tempdir().unwrap();
    run_cached(args, cache.path())
}

/// Holds the split on what `args` printed, piped and with `--color always`: standard output has
/// a result and no `warning:`, `note:` or `help:` line; standard error has nothing else, and a
/// line of each prefix in `expected`. Returns the piped run's.
fn split(args: &[&str], cache: &Path, expected: &[&str]) -> Printed {
    let mut colored = args.to_vec();
    colored.extend(["--color", "always"]);
    let piped = run_cached(args, cache);
    for (printed, args) in [(&piped, args), (&run_cached(&colored, cache), &colored[..])] {
        assert!(!printed.out.trim().is_empty(), "{args:?}: no result");
        let on_stdout: Vec<&str> = printed
            .out
            .lines()
            .filter(|line| prefix(line).is_some())
            .collect();
        assert!(on_stdout.is_empty(), "{args:?}: {on_stdout:#?}");
        let others: Vec<&str> = printed
            .err
            .lines()
            .filter(|line| prefix(line).is_none())
            .collect();
        assert!(others.is_empty(), "{args:?}: {others:#?}");
        for wanted in expected {
            assert!(
                printed.err.lines().any(|line| prefix(line) == Some(wanted)),
                "{args:?}: no {wanted:?} line on standard error:\n{}",
                printed.err
            );
        }
    }
    piped
}

#[test]
fn sim_writes_its_diagnostics_to_stderr() {
    let cache = tempfile::tempdir().unwrap();
    let design = repo_file(LEVEL_1);
    let printed = split(&["sim", &design], cache.path(), &PREFIXES);
    assert!(printed.out.contains("\napogee "), "{}", printed.out);
    // The issue's count was seven lines, which once went to standard output; the help line
    // pointing at the Accuracy page has since become the trust note that ends the result.
    assert_eq!(printed.err.lines().count(), 6, "{}", printed.err);
    assert!(
        printed.out.contains("\nHow far to trust it. Simulated"),
        "{}",
        printed.out
    );
}

#[test]
fn mc_writes_its_diagnostics_to_stderr() {
    let cache = tempfile::tempdir().unwrap();
    let design = repo_file(LEVEL_1);
    let printed = split(
        &["mc", &design, "--runs", "3", "--seed", "1"],
        cache.path(),
        &PREFIXES,
    );
    assert!(
        printed.out.contains("3 simulated flights, seed 1: "),
        "{}",
        printed.out
    );
}

#[test]
fn motors_show_writes_its_warnings_to_stderr() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("odd.eng");
    // An invented motor whose delay list is unusual, as `cli.rs`'s two-motor file has.
    std::fs::write(
        &path,
        "T10 24 70 0-4 0.01 0.03 Test\n0.0 0.0\n0.5 20.0\n1.0 0.0\n;\n",
    )
    .unwrap();
    let path = path.to_string_lossy().into_owned();
    let cache = tempfile::tempdir().unwrap();
    let printed = split(&["motors", "show", &path], cache.path(), &["warning: "]);
    assert!(
        printed.out.starts_with("T10 (Test), from "),
        "{}",
        printed.out
    );
    assert!(
        printed
            .err
            .lines()
            .any(|line| line.starts_with("warning: T10: delay 0 in \"0-4\" is ambiguous")),
        "{}",
        printed.err
    );
}

#[test]
fn motors_fetch_writes_its_hints_to_stderr() {
    use hpr::hpr_net::{Cache, Client, Mode, Replay, on_demand};
    let cache = tempfile::tempdir().unwrap();
    let replay = Replay::open(root().join("crates/hpr-net/tests/fixtures/replay")).unwrap();
    let online = Client::new(replay, Cache::new(cache.path()), Mode::Online);
    let now_s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    on_demand::find(&online, &on_demand::Wanted::named("F27R/L"), now_s).unwrap();
    let printed = split(&["motors", "fetch", "F27R/L"], cache.path(), &["help: "]);
    assert!(printed.out.starts_with("F27R/L ("), "{}", printed.out);
    assert!(
        printed
            .err
            .contains("help: kept in the cache: `hpr sim --motor F27R/L` flies it"),
        "{}",
        printed.err
    );
}

#[test]
fn convert_writes_a_motors_warnings_to_stderr() {
    let folder = tempfile::tempdir().unwrap();
    let rse = repo_file("crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000719.rse");
    let eng = folder.path().join("h.eng").to_string_lossy().into_owned();
    let cache = tempfile::tempdir().unwrap();
    let printed = split(&["convert", &rse, &eng], cache.path(), &["warning: "]);
    assert!(printed.out.starts_with("read   "), "{}", printed.out);
    assert!(printed.out.contains("\nwrote  h.eng: "), "{}", printed.out);
    assert!(printed.err.contains(": dropped "), "{}", printed.err);
}

#[test]
fn convert_writes_a_designs_warnings_to_stderr() {
    let folder = tempfile::tempdir().unwrap();
    let path = |name: &str| folder.path().join(name).to_string_lossy().into_owned();
    let log = path("flight-1.csv");
    std::fs::write(&log, "time_s,altitude_m\n0,0\n").unwrap();
    let hprz = path("level-1.hprz");
    run(&["convert", &repo_file(LEVEL_1), &hprz, "--attach", &log]);
    // A .hpr has no place for the attachment, and says so.
    let hpr = path("level-1.hpr");
    let cache = tempfile::tempdir().unwrap();
    let printed = split(&["convert", &hprz, &hpr], cache.path(), &["warning: "]);
    assert!(
        printed
            .out
            .starts_with("read   level-1.hprz\nwrote  level-1.hpr: "),
        "{}",
        printed.out
    );
    assert!(
        printed.err.starts_with("warning: flight-1.csv: "),
        "{}",
        printed.err
    );
}

#[test]
fn analyze_writes_a_logs_notes_to_stderr() {
    let folder = tempfile::tempdir().unwrap();
    let log = std::fs::read_to_string(root().join("validation/fixtures/logs/synthetic-pnut.pf2"))
        .unwrap();
    assert_eq!(log.matches("NumSamps: 984").count(), 1);
    let path = folder.path().join("short.pf2");
    std::fs::write(&path, log.replace("NumSamps: 984", "NumSamps: 1")).unwrap();
    let path = path.to_string_lossy().into_owned();
    let cache = tempfile::tempdir().unwrap();
    let printed = split(&["analyze", &path], cache.path(), &["note: "]);
    assert!(
        printed.out.starts_with("short.pf2: PerfectFlite Pnut"),
        "{}",
        printed.out
    );
    assert_eq!(
        printed.err,
        "note: the file states 1 samples and holds 984\n"
    );
}

/// `--json` is unchanged: standard output is one JSON document, which carries the notes,
/// warnings, flags and issues in its own fields, and standard error stays empty, with color
/// asked for or not. The text run's `warning:` and `note:` lines say what those fields do, one
/// line each, in their order.
#[test]
fn json_output_is_one_document_carrying_the_warnings() {
    let cache = tempfile::tempdir().unwrap();
    let design = repo_file(LEVEL_1);
    let text = split(&["sim", &design], cache.path(), &PREFIXES);
    for color in [&[][..], &["--color", "always"][..]] {
        let mut args = vec!["sim", design.as_str(), "--json"];
        args.extend(color);
        let printed = run_cached(&args, cache.path());
        assert!(printed.err.is_empty(), "{args:?}: {}", printed.err);
        // `from_str` refuses anything after the document but white space.
        let document: Value = serde_json::from_str(&printed.out).unwrap();
        let schema: Value = serde_json::from_str(
            &std::fs::read_to_string(root().join("schema/cli/sim.schema.json")).unwrap(),
        )
        .unwrap();
        assert!(
            jsonschema::validator_for(&schema)
                .unwrap()
                .is_valid(&document),
            "{document:#}"
        );
        let mut said: Vec<String> = Vec::new();
        for note in document["notes"].as_array().unwrap() {
            said.push(format!("note: {}", note.as_str().unwrap()));
        }
        for warning in document["warnings"].as_array().unwrap() {
            said.push(format!(
                "warning: {}: {}",
                warning["at"].as_str().unwrap(),
                warning["message"].as_str().unwrap()
            ));
        }
        for issue in document["issues"].as_array().unwrap() {
            said.push(format!(
                "warning: {}: {}",
                issue["kind"].as_str().unwrap(),
                issue["message"].as_str().unwrap()
            ));
        }
        assert!(!said.is_empty(), "{document:#}");
        let lines: Vec<&str> = text
            .err
            .lines()
            .filter(|line| !line.starts_with("help: "))
            .collect();
        assert_eq!(lines, said, "{document:#}");
    }
}
