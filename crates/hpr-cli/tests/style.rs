//! The product system's command-line rules (`product/cli.md`, ADR-164, ADR-174), checked on the
//! `hpr` binary: when each stream gets color, that piped and `--json` output hold no escape code,
//! that every hint is a `help:` line, and that `--version`, the `--json` documents and the files
//! `hpr` writes name the tool, its version and `FS-ACHERNAR · SW · TOOL 001`.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests run the binary and read the files it writes; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::{Path, PathBuf};
use std::process::Output;

use serde_json::Value;

/// A design that flies offline with `--motor H54`.
const PROBE: &str = "validation/fixtures/ork/pod-flights/pods-none.ork";
/// A design with two configurations, whose flight of the first prints a `help:` line naming the
/// other.
const TWO_CONFIGURATIONS: &str = "validation/fixtures/ork/guides/level-1.ork";
/// The escape that starts every color code.
const ESCAPE: &str = "\u{1b}[";
/// The color variables `cli.md` names, cleared before each run so the shell running the tests
/// doesn't decide.
const COLOR_VARIABLES: [&str; 3] = ["NO_COLOR", "FORCE_COLOR", "CLICOLOR_FORCE"];

/// The repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A file of the repository, by its path from the root.
fn repo_file(path: &str) -> String {
    root().join(path).to_string_lossy().into_owned()
}

/// Runs `hpr` offline with an empty cache, its streams piped, with `variables` set and every
/// other color variable cleared.
fn run(args: &[&str], variables: &[(&str, &str)]) -> Output {
    let cache = tempfile::tempdir().unwrap();
    let mut command = assert_cmd::cargo::cargo_bin_cmd!("hpr");
    command
        .args(args)
        .env("HPR_CACHE_DIR", cache.path())
        .env("HPR_OFFLINE", "1");
    for variable in COLOR_VARIABLES {
        command.env_remove(variable);
    }
    for (name, value) in variables {
        command.env(name, value);
    }
    command.output().unwrap()
}

/// Whether a stream's bytes hold an escape code.
fn colored(bytes: &[u8]) -> bool {
    String::from_utf8_lossy(bytes).contains(ESCAPE)
}

/// A refusal on standard error, and help on standard output: whether each stream got color.
fn decided(flags: &[&str], variables: &[(&str, &str)]) -> (bool, bool) {
    let mut help = vec!["motors", "--help"];
    help.extend(flags);
    let out = run(&help, variables);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let mut refused = vec!["motors", "show", "no-such-motor"];
    refused.extend(flags);
    let err = run(&refused, variables);
    assert_eq!(err.status.code(), Some(1), "{err:?}");
    (colored(&out.stdout), colored(&err.stderr))
}

/// A color case: the flags, the variables set, and whether standard output and standard error
/// get color.
type Case<'a> = (&'a [&'a str], &'a [(&'a str, &'a str)], (bool, bool));

/// `cli.md`'s order, each case on the binary with piped streams: the flag, then `NO_COLOR`, then
/// `FORCE_COLOR` or `CLICOLOR_FORCE`, then auto, which colors no pipe. The terminal cases, auto
/// on a terminal and `TERM=dumb`, are `hpr_cli::console`'s unit tests, as a test can't give the
/// binary a terminal.
#[test]
fn color_follows_the_flag_then_no_color_then_force_then_auto() {
    let both = (true, true);
    let neither = (false, false);
    let cases: [Case; 14] = [
        // Auto on pipes: none.
        (&[], &[], neither),
        (&["--color", "auto"], &[], neither),
        // The flag first.
        (&["--color", "always"], &[], both),
        (&["--color=always"], &[], both),
        (&["--color", "always"], &[("NO_COLOR", "1")], both),
        (&["--color", "never"], &[("FORCE_COLOR", "1")], neither),
        (&["--color", "never"], &[("CLICOLOR_FORCE", "1")], neither),
        // Then NO_COLOR, before the variables that force color.
        (&[], &[("NO_COLOR", "1")], neither),
        (&[], &[("NO_COLOR", "1"), ("FORCE_COLOR", "1")], neither),
        (&[], &[("NO_COLOR", "1"), ("CLICOLOR_FORCE", "1")], neither),
        // An empty NO_COLOR is not set (no-color.org).
        (&[], &[("NO_COLOR", ""), ("FORCE_COLOR", "1")], both),
        // Then FORCE_COLOR or CLICOLOR_FORCE, pipes too; CLICOLOR_FORCE=0 doesn't.
        (&[], &[("FORCE_COLOR", "1")], both),
        (&[], &[("CLICOLOR_FORCE", "1")], both),
        (&[], &[("CLICOLOR_FORCE", "0")], neither),
    ];
    for (flags, variables, expected) in cases {
        assert_eq!(
            decided(flags, variables),
            expected,
            "{flags:?} {variables:?}"
        );
    }
}

/// The colors are the roles' ANSI codes: `error:` bold red, `help:` bold blue, on standard error,
/// where a refusal's lines and a result's `help:` lines both go. `hpr_cli::console`'s unit tests
/// hold `warning:` and `note:` to theirs.
#[test]
fn prefixes_are_colored_in_their_roles() {
    let refused = run(
        &["motors", "show", "no-such-motor", "--color", "always"],
        &[],
    );
    let stderr = String::from_utf8(refused.stderr).unwrap();
    assert!(
        stderr.starts_with("\u{1b}[1m\u{1b}[31merror:\u{1b}[0m no motor"),
        "{stderr:?}"
    );
    assert!(
        stderr.contains("\n\u{1b}[1m\u{1b}[34mhelp:\u{1b}[0m `hpr motors list`"),
        "{stderr:?}"
    );
    let flown = run(
        &["sim", &repo_file(TWO_CONFIGURATIONS), "--color", "always"],
        &[],
    );
    assert_eq!(flown.status.code(), Some(0), "{flown:?}");
    let stderr = String::from_utf8(flown.stderr).unwrap();
    assert!(
        stderr
            .lines()
            .any(|line| line.starts_with("\u{1b}[1m\u{1b}[34mhelp:\u{1b}[0m --config flies")),
        "{stderr:?}"
    );
    let stdout = String::from_utf8(flown.stdout).unwrap();
    assert!(!stdout.contains("help:"), "{stdout}");
}

/// A result's `warning:`, `note:` and `help:` lines take standard error's color, not standard
/// output's: colored when standard error is a terminal, plain when it is a pipe, whichever
/// standard output is. The binary's streams can't be terminals in a test, so this runs the tool
/// in-process with each [`Console`](hpr_cli::console::Console) split.
#[test]
fn a_results_diagnostics_take_standard_errors_color() {
    use hpr_cli::console::Console;
    let design = repo_file(TWO_CONFIGURATIONS);
    let args = ["hpr", "sim", &design];
    for (stdout_terminal, stderr_terminal) in [(true, false), (false, true)] {
        let console = Console {
            stdout_terminal,
            stderr_terminal,
            ..Console::PIPED
        };
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let exit = hpr_cli::run_with(args, &mut out, &mut err, console);
        assert_eq!(exit, hpr_cli::Exit::Success);
        let err = String::from_utf8(err).unwrap();
        let help = if stderr_terminal {
            "\u{1b}[1m\u{1b}[34mhelp:\u{1b}[0m --config flies"
        } else {
            "help: --config flies"
        };
        assert!(
            err.lines().any(|line| line.starts_with(help)),
            "{console:?}: {err:?}"
        );
        assert_eq!(
            colored(err.as_bytes()),
            stderr_terminal,
            "{console:?}: {err:?}"
        );
        let out = String::from_utf8(out).unwrap();
        assert!(!out.contains("help:"), "{console:?}: {out}");
    }
}

/// What each command prints, piped with no color variable set, and with `--json` even when
/// color is forced: no escape code on either stream.
#[test]
fn piped_and_json_output_hold_no_escape_code() {
    let folder = tempfile::tempdir().unwrap();
    let csv = folder
        .path()
        .join("flight.csv")
        .to_string_lossy()
        .into_owned();
    let plot = folder
        .path()
        .join("flight.svg")
        .to_string_lossy()
        .into_owned();
    let rse = folder.path().join("h54.rse").to_string_lossy().into_owned();
    let probe = repo_file(PROBE);
    let commands: [&[&str]; 12] = [
        &["--help"],
        &["--version"],
        &["motors", "list", "--class", "H"],
        &["motors", "show", "H54"],
        &[
            "sim", &probe, "--motor", "H54", "--export", &csv, "--plot", &plot,
        ],
        &["convert", "H54", &rse],
        &["completions", "bash"],
        // Refusals: an input error, a usage error with a suggestion, a command not yet here.
        &["motors", "show", "no-such-motor"],
        &["sim", &probe, "--motr", "H54"],
        &["sim", &probe, "--motor", "H54", "--delay", "soon"],
        &["mc"],
        &["motors", "list", "--class", "Q9"],
    ];
    for args in commands {
        let piped = run(args, &[]);
        assert!(!colored(&piped.stdout), "{args:?}: {piped:?}");
        assert!(!colored(&piped.stderr), "{args:?}: {piped:?}");
        if args.contains(&"--help") || args.contains(&"--version") {
            continue;
        }
        let mut json = args.to_vec();
        json.push("--json");
        for variables in [&[][..], &[("FORCE_COLOR", "1")][..]] {
            let forced = run(&json, variables);
            assert!(!colored(&forced.stdout), "{json:?}: {forced:?}");
            assert!(forced.stderr.is_empty(), "{json:?}: {forced:?}");
        }
        let mut always = json.clone();
        always.extend(["--color", "always"]);
        let flagged = run(&always, &[]);
        assert!(!colored(&flagged.stdout), "{always:?}: {flagged:?}");
    }
    // Nor do the files a run writes, with color asked for.
    run(
        &[
            "sim", &probe, "--motor", "H54", "--export", &csv, "--plot", &plot, "--color", "always",
        ],
        &[],
    );
    let meta = folder.path().join("flight.meta.json");
    for file in [Path::new(&csv), Path::new(&plot), meta.as_path()] {
        assert!(
            !colored(&std::fs::read(file).unwrap()),
            "{}",
            file.display()
        );
    }
}

/// Each refusal that has something to suggest says it on its own `help:` line, the last, and its
/// `error:` line holds the fact alone; `--json` puts the same lines in `help`.
#[test]
fn every_hint_is_a_help_line() {
    let probe = repo_file(PROBE);
    let multi = repo_file("validation/fixtures/ork/loft-demo/demo-multi-config.ork");
    let refusals: [(&[&str], &str); 10] = [
        (
            &["motors", "show", "no-such-motor"],
            "`hpr motors list` lists the catalog's motors",
        ),
        (
            &["motors", "list", "--class", "Q9"],
            "use 1/8A, 1/4A, 1/2A or a letter A to Z",
        ),
        (
            &["motors", "list", "--manufacturer", "Nobody"],
            "use one of ",
        ),
        (
            &["convert", "no-such-motor", "x.eng"],
            "`hpr motors list` lists the catalog's motors",
        ),
        (&["convert", "I175", "x.eng"], "give one's designation"),
        (&["sim", &probe], "give a motor with --motor"),
        (
            &["sim", &probe, "--motor", "Z9999"],
            "with a network connection, `hpr motors fetch Z9999`",
        ),
        (
            &["sim", &multi, "--motor", "H54"],
            "--accept-design-errors flies it anyway",
        ),
        (
            &["sim", &probe, "--delay", "soon", "--motor", "H54"],
            "give the seconds from burnout",
        ),
        (
            &["sim", &probe, "--motr", "H54"],
            "a similar argument exists: '--motor'",
        ),
    ];
    for (args, hint) in refusals {
        let output = run(args, &[]);
        let stderr = String::from_utf8(output.stderr).unwrap();
        let lines: Vec<&str> = stderr.lines().filter(|line| !line.is_empty()).collect();
        assert!(lines[0].starts_with("error: "), "{args:?}: {stderr}");
        assert!(!lines[0].contains(hint), "{args:?}: {stderr}");
        let helps: Vec<&&str> = lines
            .iter()
            .filter(|line| line.starts_with("help: "))
            .collect();
        assert!(
            helps.iter().any(|line| line.contains(hint)),
            "{args:?}: {stderr}"
        );
        assert!(
            lines.last().unwrap().starts_with("help: "),
            "{args:?}: {stderr}"
        );
        let mut json = args.to_vec();
        json.push("--json");
        let document: Value = serde_json::from_slice(&run(&json, &[]).stdout).unwrap();
        let error = &document["error"];
        assert!(
            !error["message"].as_str().unwrap().contains(hint),
            "{document:#}"
        );
        assert!(
            error["help"]
                .as_array()
                .unwrap()
                .iter()
                .any(|help| help.as_str().unwrap().contains(hint)),
            "{document:#}"
        );
    }
}

/// The `tool` object every document opens with, as this build writes it.
fn tool() -> Value {
    serde_json::json!({
        "name": "FusionSpace HPR",
        "version": env!("CARGO_PKG_VERSION"),
        "designation": "FS-ACHERNAR · SW · TOOL 001",
    })
}

/// `hpr --version`, every `--json` document (success, refusal or usage error) and every schema
/// name the tool, its version and its designation.
#[test]
fn version_and_every_document_name_the_tool() {
    let version = run(&["--version"], &[]);
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!(
            "FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001\n",
            env!("CARGO_PKG_VERSION")
        )
    );
    let probe = repo_file(PROBE);
    let documents: [&[&str]; 6] = [
        &["motors", "list", "--class", "H", "--json"],
        &["sim", &probe, "--motor", "H54", "--json"],
        &["completions", "zsh", "--json"],
        &["motors", "show", "no-such-motor", "--json"],
        &["sim", "--motr", "--json"],
        &["mc", "--json"],
    ];
    for args in documents {
        let text = String::from_utf8(run(args, &[]).stdout).unwrap();
        let document: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(document["tool"], tool(), "{args:?}");
        // The title block comes first, before the command's own fields.
        assert!(text.starts_with("{\n  \"tool\": {"), "{args:?}: {text}");
    }
    for (name, schema) in hpr_cli::schemas() {
        let schema: Value = serde_json::from_str(&schema).unwrap();
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&Value::from("tool")),
            "{name}"
        );
        assert_eq!(
            schema["properties"]["tool"]["required"],
            serde_json::json!(["name", "version", "designation"]),
            "{name}"
        );
    }
}

/// Every file `hpr` writes names the tool, its version and its designation: each recording
/// format, the CSV's sidecar, the plot, the motor files, the design files and a weather profile.
#[test]
fn every_file_hpr_writes_names_the_tool() {
    let folder = tempfile::tempdir().unwrap();
    let path = |name: &str| folder.path().join(name).to_string_lossy().into_owned();
    let probe = repo_file(PROBE);
    let mut sim = vec![
        "sim".to_owned(),
        probe.clone(),
        "--motor".into(),
        "H54".into(),
    ];
    for name in ["f.csv", "f.json", "f.parquet", "f.geojson", "f.kml"] {
        sim.extend(["--export".to_owned(), path(name)]);
    }
    sim.extend(["--plot".to_owned(), path("f.svg")]);
    let sim: Vec<&str> = sim.iter().map(String::as_str).collect();
    let flown = run(&sim, &[]);
    assert_eq!(flown.status.code(), Some(0), "{flown:?}");
    for (args, written) in [
        (vec!["convert", "H54", &path("m.eng")], "m.eng"),
        (vec!["convert", "H54", &path("m.rse")], "m.rse"),
        (vec!["convert", &probe, &path("d.ork")], "d.ork"),
        (vec!["convert", &probe, &path("d.hpr")], "d.hpr"),
        (vec!["convert", &probe, &path("d.hprz")], "d.hprz"),
    ] {
        let converted = run(&args, &[]);
        assert_eq!(converted.status.code(), Some(0), "{written}: {converted:?}");
    }
    let stamp = format!(
        "FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001",
        env!("CARGO_PKG_VERSION")
    );
    // The JSON files carry the `tool` object; the rest the line, in the place their format keeps.
    for name in ["f.json", "f.geojson", "f.meta.json", "d.hpr"] {
        let document: Value =
            serde_json::from_str(&std::fs::read_to_string(path(name)).unwrap()).unwrap();
        let tool = match name {
            "d.hpr" => serde_json::json!({
                "name": document["provenance"]["tool"],
                "version": document["provenance"]["tool_version"],
                "designation": document["provenance"]["designation"],
            }),
            _ => document["tool"].clone(),
        };
        assert_eq!(tool, self::tool(), "{name}");
    }
    for name in ["f.kml", "f.svg", "m.eng", "m.rse"] {
        let text = std::fs::read_to_string(path(name)).unwrap();
        // The plot's in its metadata and in its last visible line.
        let count = if name == "f.svg" { 2 } else { 1 };
        assert_eq!(text.matches(&stamp).count(), count, "{name}: {text}");
    }
    // A Parquet file's key-value metadata, and a `.ork`'s zipped document, as bytes.
    let parquet = std::fs::read(path("f.parquet")).unwrap();
    for value in [
        "FusionSpace HPR",
        env!("CARGO_PKG_VERSION"),
        "FS-ACHERNAR · SW · TOOL 001",
    ] {
        assert!(
            parquet.windows(value.len()).any(|w| w == value.as_bytes()),
            "f.parquet: {value}"
        );
    }
    let ork = hpr::hpr_io::ork::read(&std::fs::read(path("d.ork")).unwrap()).unwrap();
    assert_eq!(ork.value.document.creator.as_deref(), Some(stamp.as_str()));
    let hprz = hpr::hpr_format::container::read(&std::fs::read(path("d.hprz")).unwrap()).unwrap();
    let provenance = &hprz.value.design.provenance;
    assert_eq!(
        (
            provenance.tool.as_str(),
            provenance.tool_version.as_str(),
            provenance.designation.as_deref()
        ),
        (
            "FusionSpace HPR",
            env!("CARGO_PKG_VERSION"),
            Some("FS-ACHERNAR · SW · TOOL 001")
        )
    );
    // The sidecar holds to its published schema and counts the CSV's rows.
    let meta: Value =
        serde_json::from_str(&std::fs::read_to_string(path("f.meta.json")).unwrap()).unwrap();
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(root().join("schema/cli/sim-export-meta.schema.json")).unwrap(),
    )
    .unwrap();
    assert!(
        jsonschema::validator_for(&schema).unwrap().is_valid(&meta),
        "{meta:#}"
    );
    assert_eq!(meta["file"], "f.csv");
    assert_eq!(meta["design"], "pods-none.ork");
    let rows = std::fs::read_to_string(path("f.csv"))
        .unwrap()
        .lines()
        .count()
        - 1;
    assert_eq!(meta["rows"], rows, "{meta:#}");
    // The CSV itself opens in a spreadsheet: its first line is the header.
    let csv = std::fs::read_to_string(path("f.csv")).unwrap();
    assert!(
        csv.starts_with("time [s]") || csv.starts_with("time_s"),
        "{csv:.80}"
    );
}

/// The product system's terminal styles are copied in unchanged (ADR-164): their license line
/// first and, when `refs/fusionspace-design` is checked out at the pinned revision, the same
/// bytes as its `product/cli/fs_style.rs`.
#[test]
fn the_styles_are_the_product_systems() {
    let copied = std::fs::read_to_string(root().join("crates/hpr-cli/src/fs_style.rs")).unwrap();
    assert!(
        copied.starts_with("// SPDX-License-Identifier: Apache-2.0 · Copyright 2026 Neer Patel\n")
    );
    let pinned = root().join("refs/fusionspace-design/product/cli/fs_style.rs");
    if pinned.is_file() {
        assert_eq!(copied, std::fs::read_to_string(pinned).unwrap());
    }
}

/// Clap's own hints, in color too: no `tip:` and no bracketed list of values is left, and each
/// is a `help:` line in its role.
#[test]
fn clap_hints_are_help_lines_in_color() {
    let help = "\u{1b}[1m\u{1b}[34mhelp:\u{1b}[0m";
    for (args, hints) in [
        (
            &["sim", "x.ork", "--motr", "H54", "--color", "always"][..],
            2,
        ),
        (&["motors", "list", "--color", "sometimes"][..], 2),
    ] {
        let stderr = String::from_utf8(run(args, &[]).stderr).unwrap();
        assert!(!stderr.contains("tip:"), "{args:?}: {stderr:?}");
        assert!(!stderr.contains("[possible values"), "{args:?}: {stderr:?}");
        let colored = args.contains(&"always");
        let prefix = if colored { help } else { "help:" };
        let lines = stderr
            .lines()
            .filter(|line| line.starts_with(prefix))
            .count();
        assert_eq!(lines, hints, "{args:?}: {stderr:?}");
    }
}

/// A library's message that ends with its hint, as a motor file whose figures can't be a solid
/// motor's, is split: the fact on the `error:` line, the hint on a `help:` line.
#[test]
fn a_librarys_hint_is_a_help_line() {
    let folder = tempfile::tempdir().unwrap();
    // 10 N·s from a gram of propellant: 10 km/s, no solid motor's exhaust velocity.
    let file = folder.path().join("odd.eng");
    std::fs::write(&file, "X3 29 100 P 0.001 0.08 Maker\n 0.5 20\n 1 0\n").unwrap();
    let file = file.to_string_lossy().into_owned();
    let output = run(&["sim", &repo_file(PROBE), "--motor", &file], &[]);
    let stderr = String::from_utf8(output.stderr).unwrap();
    let lines: Vec<&str> = stderr.lines().collect();
    assert!(lines[0].starts_with("error: "), "{stderr}");
    assert!(lines[0].contains("exhaust velocity"), "{stderr}");
    assert!(!lines[0].contains("check the units"), "{stderr}");
    assert_eq!(
        lines.last(),
        Some(&"help: check the units of the masses and the curve"),
        "{stderr}"
    );
}

/// A CSV's sidecar may not be another file the run writes: refused before the flight, with
/// nothing written.
#[test]
fn a_sidecar_may_not_be_another_output() {
    let folder = tempfile::tempdir().unwrap();
    let csv = folder.path().join("a.csv").to_string_lossy().into_owned();
    let meta = folder
        .path()
        .join("a.meta.json")
        .to_string_lossy()
        .into_owned();
    let output = run(
        &[
            "sim",
            &repo_file(PROBE),
            "--motor",
            "H54",
            "--export",
            &csv,
            "--export",
            &meta,
        ],
        &[],
    );
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("--export writes its sidecar") && stderr.contains("a.meta.json"),
        "{stderr}"
    );
    assert!(!Path::new(&csv).exists());
    assert!(!Path::new(&meta).exists());
}
