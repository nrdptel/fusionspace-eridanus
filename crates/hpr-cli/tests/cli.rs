//! The `hpr` binary run as a user runs it: exit codes, standard output and standard error, and
//! every `--json` document checked against its committed schema in `schema/cli/`.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests read the committed schemas and write motor files; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::{Path, PathBuf};
use std::process::Output;

use clap::ValueEnum;
use clap_complete::Shell;
use hpr::hpr_flightdata::perfectflite::FOOT_M;
use hpr::hpr_motor::Catalog;
use hpr_cli::motors::MotorFile;
use hpr_cli::registry::{self, Availability, PLANNED};
use serde_json::{Value, json};

/// The repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A bundled curve file, by its catalog path.
fn curve_file(name: &str) -> String {
    root()
        .join("crates/hpr-motor/data/thrustcurve")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

/// Runs `hpr` with `args`, offline with an empty cache: no test reaches the network, and a motor
/// with no curve is never fetched.
fn hpr(args: &[&str]) -> Output {
    static EMPTY: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    let empty = EMPTY.get_or_init(|| tempfile::tempdir().unwrap());
    hpr_cached(args, empty.path())
}

/// Runs `hpr` with `args`, offline, with `cache` as its cache, and none of the variables that
/// ask for color or turn it off, so that the shell running the tests doesn't change the text.
fn hpr_cached(args: &[&str], cache: &Path) -> Output {
    assert_cmd::cargo::cargo_bin_cmd!("hpr")
        .args(args)
        .env("HPR_CACHE_DIR", cache)
        .env("HPR_OFFLINE", "1")
        .env_remove("NO_COLOR")
        .env_remove("FORCE_COLOR")
        .env_remove("CLICOLOR_FORCE")
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// Checks `document` against the committed schema `name`.
fn validate(name: &str, document: &Value) {
    let path = root().join("schema/cli").join(name);
    let schema: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let errors: Vec<String> = validator
        .iter_errors(document)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{name}: {errors:?}\n{document:#}");
}

/// Runs `hpr` with `--json` added, expects `code`, an empty standard error and one document on
/// standard output that the schema `name` accepts, and returns the document.
fn json(args: &[&str], code: i32, schema: &str) -> Value {
    json_of(args, code, schema, hpr)
}

/// As [`json`], offline with `cache` as the cache.
fn json_cached(args: &[&str], code: i32, schema: &str, cache: &Path) -> Value {
    json_of(args, code, schema, |args| hpr_cached(args, cache))
}

fn json_of(args: &[&str], code: i32, schema: &str, run: impl Fn(&[&str]) -> Output) -> Value {
    let mut args = args.to_vec();
    args.push("--json");
    let output = run(&args);
    assert_eq!(output.status.code(), Some(code), "{args:?}: {output:?}");
    assert!(
        output.stderr.is_empty(),
        "{args:?}: {}",
        text(&output.stderr)
    );
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    validate(schema, &document);
    document
}

/// Runs `hpr` without `--json`, expects success and nothing on standard error, and returns the
/// text.
fn text_ok(args: &[&str]) -> String {
    let output = hpr(args);
    assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
    assert!(
        output.stderr.is_empty(),
        "{args:?}: {}",
        text(&output.stderr)
    );
    text(&output.stdout)
}

/// What a run without `--json` printed: the result on standard output, and the `warning:`,
/// `note:` and `help:` lines beside it on standard error.
struct Printed {
    /// Standard output.
    out: String,
    /// Standard error.
    err: String,
}

impl Printed {
    /// Both streams, standard output first: for a test that some words appear on neither.
    fn both(&self) -> String {
        format!("{}{}", self.out, self.err)
    }
}

/// Whether `line` is a `warning:`, `note:` or `help:` line, its color codes aside.
fn is_diagnostic(line: &str) -> bool {
    let mut plain = String::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            // An SGR sequence, `ESC [ … m`.
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            plain.push(c);
        }
    }
    ["warning: ", "note: ", "help: "]
        .iter()
        .any(|prefix| plain.starts_with(prefix))
}

/// Runs `hpr` without `--json`, expects success, and returns what it printed, holding the
/// streams apart: standard output has no `warning:`, `note:` or `help:` line, and every line of
/// standard error is one.
fn streams(args: &[&str]) -> Printed {
    printed_of(args, hpr(args))
}

/// As [`streams`], offline with `cache` as the cache.
fn streams_cached(args: &[&str], cache: &Path) -> Printed {
    printed_of(args, hpr_cached(args, cache))
}

fn printed_of(args: &[&str], output: Output) -> Printed {
    assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
    let printed = Printed {
        out: text(&output.stdout),
        err: text(&output.stderr),
    };
    assert!(
        !printed.out.lines().any(is_diagnostic),
        "{args:?}: a diagnostic on standard output:\n{}",
        printed.out
    );
    assert!(
        printed.err.lines().all(is_diagnostic),
        "{args:?}: standard error has a line that isn't a diagnostic:\n{}",
        printed.err
    );
    printed
}

/// An error document of `kind`, from a run with `--json`.
fn json_error(args: &[&str], code: i32, kind: &str) -> Value {
    let document = json(args, code, "error.schema.json");
    assert_eq!(document["error"]["kind"], kind, "{document:#}");
    document
}

/// An error document's message and its `help:` lines, as the text output writes them after
/// `error: `.
fn said(document: &Value) -> String {
    let error = &document["error"];
    let mut text = error["message"].as_str().unwrap().to_owned();
    for help in error["help"].as_array().unwrap() {
        text.push_str("\nhelp: ");
        text.push_str(help.as_str().unwrap());
    }
    text
}

/// Every command registered before its milestone refuses with status 3 and names that
/// milestone, whatever arguments it is given, as text on standard error or as a JSON document.
#[test]
fn every_planned_command_refuses_with_its_milestone() {
    let planned: Vec<_> = registry::commands()
        .into_iter()
        .filter(|c| matches!(c.availability, Some(Availability::Planned { .. })))
        .collect();
    assert_eq!(planned.len(), PLANNED.len());
    for (name, milestone) in PLANNED {
        let output = hpr(&[name, "design.ork", "--rail", "2"]);
        assert_eq!(output.status.code(), Some(3), "{name}: {output:?}");
        assert!(output.stdout.is_empty(), "{name}");
        let message = text(&output.stderr);
        assert!(
            message.starts_with(&format!("error: hpr {name} is not available yet"))
                && message.contains(milestone),
            "{message}"
        );
        let document = json_error(&[name, "design.ork", "--rail", "2"], 3, "not_available");
        assert_eq!(document["error"]["command"], name);
        assert_eq!(document["error"]["milestone"], milestone);
        // `--json` first, before the command, works the same.
        let first = hpr(&["--json", name]);
        assert_eq!(first.status.code(), Some(3));
        assert_eq!(
            serde_json::from_slice::<Value>(&first.stdout).unwrap(),
            document
        );
    }
}

/// `hpr motors list` lists the bundled catalog in its order, with the figures its curve files
/// give, and each filter narrows it.
#[test]
fn motors_list_is_the_catalog() {
    let catalog = Catalog::bundled().unwrap();
    let all = json(&["motors", "list"], 0, "motors-list.schema.json");
    let motors = all["motors"].as_array().unwrap();
    assert_eq!(motors.len(), catalog.motors.len());
    for (listed, motor) in motors.iter().zip(&catalog.motors) {
        assert_eq!(listed["designation"], motor.designation.as_str());
        assert_eq!(
            listed["impulse_class"],
            motor.impulse_class.label().as_str()
        );
        assert_eq!(listed["total_impulse_ns"], motor.total_impulse_ns);
        assert_eq!(listed["burn_time_s"], motor.burn_time_s);
        assert_eq!(listed["diameter_mm"], motor.diameter_mm);
    }
    assert_eq!(
        all["catalog"]["captured"],
        catalog.snapshot.captured.as_str()
    );

    let class_j = json(
        &["motors", "list", "--class", "j"],
        0,
        "motors-list.schema.json",
    );
    let class_j = class_j["motors"].as_array().unwrap();
    let expected = catalog
        .motors
        .iter()
        .filter(|m| m.impulse_class.label() == "J")
        .count();
    assert!(expected > 0);
    assert_eq!(class_j.len(), expected);
    assert!(class_j.iter().all(|m| m["impulse_class"] == "J"));

    let wide = json(
        &["motors", "list", "--diameter", "54"],
        0,
        "motors-list.schema.json",
    );
    let wide = wide["motors"].as_array().unwrap();
    assert!(!wide.is_empty());
    assert!(wide.iter().all(|m| m["diameter_mm"] == 54.0));

    let loki = json(
        &["motors", "list", "--manufacturer", "LOKI", "--class", "J"],
        0,
        "motors-list.schema.json",
    );
    let loki = loki["motors"].as_array().unwrap();
    assert_eq!(loki.len(), 1);
    assert_eq!(loki[0]["manufacturer_abbrev"], "Loki");

    let none = json(
        &["motors", "list", "--class", "Z"],
        0,
        "motors-list.schema.json",
    );
    assert!(none["motors"].as_array().unwrap().is_empty());

    let table = text_ok(&["motors", "list", "--class", "J"]);
    assert!(table.starts_with(&format!("{expected} motors from ThrustCurve.org")));
    assert!(table.contains("designation"));
    assert_eq!(table.lines().count(), 3 + expected);
}

/// A filter that can't mean anything is refused rather than matching nothing.
#[test]
fn a_bad_filter_is_an_input_error() {
    for args in [
        ["motors", "list", "--class", "JJ"],
        ["motors", "list", "--diameter", "-3"],
        ["motors", "list", "--diameter", "NaN"],
        ["motors", "list", "--manufacturer", "CTI"],
        ["motors", "list", "--manufacturer", "ces"],
    ] {
        let document = json_error(&args, 1, "input");
        assert_eq!(document["error"]["command"], "motors");
        let output = hpr(&args);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(text(&output.stderr).starts_with("error: --"));
    }
}

/// A maker is named as the catalog spells it, or by its full name, in any case: the guide's
/// examples.
#[test]
fn a_maker_is_named_by_abbreviation_or_full_name() {
    let listed = |maker: &str| {
        let document = json(
            &["motors", "list", "--manufacturer", maker],
            0,
            "motors-list.schema.json",
        );
        document["motors"].as_array().unwrap().clone()
    };
    let cesaroni = listed("cesaroni");
    assert!(!cesaroni.is_empty());
    assert!(
        cesaroni
            .iter()
            .all(|m| m["manufacturer_abbrev"] == "Cesaroni")
    );
    assert_eq!(listed("Cesaroni Technology"), cesaroni);
    assert_eq!(listed("AEROTECH").len(), listed("AeroTech").len());
    let refused = hpr(&["motors", "list", "--manufacturer", "CTI"]);
    let message = text(&refused.stderr);
    assert!(
        message.contains("use one of AMW, AeroTech, Cesaroni"),
        "{message}"
    );
}

/// After `--`, `--json` is an argument, such as a motor's name, and the output stays text.
#[test]
fn json_after_a_double_dash_is_an_argument() {
    let output = hpr(&["motors", "show", "--", "--json"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(text(&output.stderr).contains("is called --json"));
}

/// `hpr motors show` reports the library's own figures for a catalog motor's bundled curve, and
/// names the public-domain file they come from.
#[test]
fn motors_show_reports_the_librarys_figures() {
    let catalog = Catalog::bundled().unwrap();
    let motor = catalog.find("1266J760-19A").next().unwrap();
    let solid = motor.bundled_motor().unwrap();
    let curve = solid.curve();
    for name in ["1266J760-19A", "j760", "J 760"] {
        let document = json(&["motors", "show", name], 0, "motors-show.schema.json");
        let shown = &document["motors"][0];
        assert_eq!(document["motors"].as_array().unwrap().len(), 1);
        assert_eq!(shown["name"], "1266J760-19A");
        assert_eq!(shown["source"]["kind"], "catalog");
        assert_eq!(shown["total_impulse_ns"], curve.total_impulse_ns());
        assert_eq!(shown["average_thrust_n"], curve.average_thrust_n());
        assert_eq!(shown["peak_thrust_n"], curve.peak_thrust_n());
        assert_eq!(shown["burn_time_s"], curve.burn_time_s());
        assert_eq!(shown["burn_start_s"], curve.burn_window_s().0);
        assert_eq!(shown["burn_end_s"], curve.burn_window_s().1);
        assert_eq!(shown["curve_end_s"], curve.end_time_s());
        assert_eq!(shown["impulse_class"], "J");
        assert_eq!(
            shown["propellant_mass_kg"],
            solid.propellant_initial_mass_kg()
        );
        assert_eq!(shown["diameter_m"], 0.054);
        assert_eq!(
            shown["source"]["curve_url"],
            "https://www.thrustcurve.org/simfiles/5f4294d20002e9000000074a/"
        );
        assert_eq!(
            shown["delays"].as_array().unwrap().len(),
            motor.delays().delays.len()
        );
        // The delays are the curve file's header's, `8-10-12-14-16-18-19` (#295).
        assert_eq!(
            shown["delays"][0],
            serde_json::json!({"kind": "seconds", "value": 8.0})
        );
    }
    let lines = text_ok(&["motors", "show", "J760"]);
    assert!(lines.starts_with("1266J760-19A (Cesaroni Technology), from the bundled catalog\n"));
    assert!(lines.contains("  total impulse    1267.3 N·s\n"), "{lines}");
    assert!(lines.contains(
        "  curve file       https://www.thrustcurve.org/simfiles/5f4294d20002e9000000074a/ (public \
         domain)\n"
    ));
    assert!(!lines.contains("ThrustCurve.org  "), "{lines}");
}

/// The figures `hpr motors list` gives for every catalog motor are the ones `hpr motors show`
/// works out from its curve, bit for bit: since issue #295 the bundled catalog works them out
/// from its public-domain curve files, and none is ThrustCurve.org's published figure. (The guide's
/// comparison with those, within 1% and peak thrust from 16.7% below to 2.1% above, is printed and
/// the 1% held by `cargo xtask motor-catalog`, from ThrustCurve.org's catalog kept outside the
/// repository.)
#[test]
fn the_catalogs_figures_are_what_show_works_out() {
    let list = json(&["motors", "list"], 0, "motors-list.schema.json");
    let listed = list["motors"].as_array().unwrap();
    assert_eq!(listed.len(), 32);
    for entry in listed {
        let designation = entry["designation"].as_str().unwrap();
        let document = json(
            &["motors", "show", designation],
            0,
            "motors-show.schema.json",
        );
        let shown = document["motors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["name"] == designation)
            .unwrap()
            .clone();
        assert!(shown.get("stated").is_none(), "{designation}");
        for key in ["total_impulse_ns", "average_thrust_n", "burn_time_s"] {
            assert_eq!(
                entry[key].as_f64().unwrap().to_bits(),
                shown[key].as_f64().unwrap().to_bits(),
                "{designation}: {key}"
            );
        }
        assert_eq!(
            entry["impulse_class"], shown["impulse_class"],
            "{designation}"
        );
        assert_eq!(
            entry["diameter_mm"].as_f64().unwrap() / 1000.0,
            shown["diameter_m"].as_f64().unwrap(),
            "{designation}"
        );
    }
}

/// A name several catalog motors share shows each of them.
#[test]
fn a_shared_name_shows_every_match() {
    let catalog = Catalog::bundled().unwrap();
    let expected: Vec<&str> = catalog
        .find("I175")
        .map(|m| m.designation.as_str())
        .collect();
    assert_eq!(expected.len(), 2);
    let document = json(&["motors", "show", "I175"], 0, "motors-show.schema.json");
    let names: Vec<&str> = document["motors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, expected);
}

/// Every motor file format the command table claims is read, from a bundled curve file, to the
/// figures the catalog's own reading of that file gives.
#[test]
fn every_motor_file_format_is_read() {
    let catalog = Catalog::bundled().unwrap();
    for format in MotorFile::ALL {
        let motor = catalog
            .motors
            .iter()
            .find(|m| m.curves[0].file.ends_with(format.extension()))
            .unwrap();
        let path = curve_file(&motor.curves[0].file);
        assert_eq!(MotorFile::of(&path), Some(format));
        let document = json(&["motors", "show", &path], 0, "motors-show.schema.json");
        let shown = &document["motors"][0];
        assert_eq!(shown["source"]["kind"], "file");
        assert_eq!(shown["source"]["path"], path.as_str());
        assert_eq!(shown["source"]["format"], &format.extension()[1..]);
        let curve = motor.bundled_motor().unwrap();
        assert_eq!(shown["total_impulse_ns"], curve.curve().total_impulse_ns());
        assert_eq!(shown["burn_time_s"], curve.curve().burn_time_s());
        let lines = text_ok(&["motors", "show", &path]);
        assert!(lines.contains(&format!("from {path}\n")), "{lines}");
    }
}

/// A `.eng` file with two motors shows both, and a `0` delay is shown with its warning.
#[test]
fn a_file_with_two_motors_shows_both_with_warnings() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("two.ENG");
    // Invented motors: a triangle and a rectangle of thrust, with round numbers.
    std::fs::write(
        &path,
        "; two test motors\n\
         T10 24 70 0-4 0.01 0.03 Test\n\
         0.0 0.0\n0.5 20.0\n1.0 0.0\n;\n\
         T20 24 70 P 0.02 0.04 Test\n\
         0.0 20.0\n1.0 20.0\n1.0 0.0\n;\n",
    )
    .unwrap();
    let path = path.to_string_lossy().into_owned();
    let document = json(&["motors", "show", &path], 0, "motors-show.schema.json");
    let motors = document["motors"].as_array().unwrap();
    assert_eq!(motors.len(), 2);
    assert_eq!(motors[0]["name"], "T10");
    assert_eq!(motors[0]["total_impulse_ns"], 10.0);
    assert_eq!(motors[0]["peak_thrust_n"], 20.0);
    assert_eq!(
        motors[0]["delays"][0],
        serde_json::json!({"kind": "zero_or_plugged"})
    );
    assert_eq!(motors[1]["name"], "T20");
    assert_eq!(motors[1]["total_impulse_ns"], 20.0);
    assert_eq!(motors[1]["impulse_class"], "D");
    assert_eq!(
        motors[1]["delays"][0],
        serde_json::json!({"kind": "plugged"})
    );
    let warnings = document["warnings"].as_array().unwrap();
    assert!(
        warnings
            .iter()
            .any(|w| w["motor"] == "T10" && w["kind"] == "unusual"),
        "{warnings:?}"
    );
    let lines = streams(&["motors", "show", &path]);
    assert!(lines.out.contains("\nT20 (Test), from "));
    assert!(
        lines
            .err
            .lines()
            .any(|line| line.starts_with("warning: T10: ")),
        "{}",
        lines.err
    );
}

/// A motor that can't be found or read is an input error, with nothing on standard output.
#[test]
fn a_missing_or_unreadable_motor_is_an_input_error() {
    let dir = tempfile::tempdir().unwrap();
    let broken = dir.path().join("broken.rse");
    std::fs::write(&broken, "<engine-database><engine-list>").unwrap();
    let not_text = dir.path().join("binary.eng");
    std::fs::write(&not_text, [0xff, 0xfe, 0x00]).unwrap();
    let missing = dir.path().join("missing.eng");
    for motor in [
        "Z99999",
        &broken.to_string_lossy(),
        &not_text.to_string_lossy(),
        &missing.to_string_lossy(),
    ] {
        let document = json_error(&["motors", "show", motor], 1, "input");
        assert_eq!(document["error"]["command"], "motors");
        let output = hpr(&["motors", "show", motor]);
        assert_eq!(output.status.code(), Some(1), "{motor}");
        assert!(output.stdout.is_empty(), "{motor}");
        assert!(text(&output.stderr).starts_with("error: "), "{motor}");
    }
}

/// `hpr completions` writes a script for every shell clap_complete knows, naming the commands.
#[test]
fn completions_for_every_shell() {
    for shell in Shell::value_variants() {
        let name = shell.to_string();
        let script = text_ok(&["completions", &name]);
        assert!(
            script.contains("motors") && script.contains("completions"),
            "{name}"
        );
        let document = json(&["completions", &name], 0, "completions.schema.json");
        assert_eq!(document["shell"], name.as_str());
        assert_eq!(document["script"], script.as_str());
    }
}

/// A wrong command line exits with 2: clap's text on standard error, or an error document.
#[test]
fn usage_errors_exit_with_2() {
    for args in [
        vec!["motors", "shwo"],
        vec!["motors", "show"],
        vec!["motors", "list", "--diameter", "wide"],
        vec!["launch"],
        vec!["completions", "cmd.exe"],
        vec![],
    ] {
        let output = hpr(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(!output.stderr.is_empty(), "{args:?}");
        let document = json_error(&args, 2, "usage");
        assert!(document["error"]["command"].is_null());
    }
}

/// Help and version are answers, not errors: standard output, status 0, even with `--json`.
#[test]
fn help_and_version_print_text() {
    // clap's own `help` command takes command names only, not `--json`.
    assert!(!text_ok(&["help"]).is_empty());
    for args in [["--help"], ["--version"]] {
        let help = text_ok(&args);
        assert!(!help.is_empty());
        let with_json = hpr(&[args[0], "--json"]);
        assert_eq!(with_json.status.code(), Some(0));
        assert_eq!(text(&with_json.stdout), help);
    }
    let help = text_ok(&["--help"]);
    for command in registry::commands() {
        assert!(
            help.contains(&format!("  {} ", command.name)),
            "{}",
            command.name
        );
    }
    assert_eq!(
        text_ok(&["--version"]),
        format!(
            "FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001\n",
            env!("CARGO_PKG_VERSION")
        )
    );
}

/// The committed schemas are the ones the output types generate, and no others.
#[test]
fn the_committed_schemas_are_generated() {
    let generated = hpr_cli::schemas();
    let mut committed: Vec<String> = std::fs::read_dir(root().join("schema/cli"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".schema.json"))
        .collect();
    committed.sort();
    let names: Vec<&str> = generated.iter().map(|(name, _)| *name).collect();
    assert_eq!(committed, names);
    for (name, text) in generated {
        let file = std::fs::read_to_string(root().join("schema/cli").join(name)).unwrap();
        assert_eq!(file, text, "{name} is stale: run `cargo xtask cli`");
    }
}

/// The repository's own pod probe with no pods: a single-stage `.ork`, public, with a 29 mm mount
/// and an AeroTech H128W that the bundled catalog doesn't hold.
const PROBE: &str = "validation/fixtures/ork/pod-flights/pods-none.ork";

/// A repository file's path, as a string for the command line.
fn repo_file(path: &str) -> String {
    root().join(path).to_string_lossy().into_owned()
}

/// `a` and `b` hold the same keys and the same numbers, bit for bit (`float_roundtrip` reads the
/// printed numbers back exactly).
fn same_bits(a: &Value, b: &Value, at: &str) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            assert_eq!(x.to_bits(), y.to_bits(), "{at}: {x} != {y}");
        }
        (Value::Object(x), Value::Object(y)) => {
            let keys = |o: &serde_json::Map<String, Value>| o.keys().cloned().collect::<Vec<_>>();
            assert_eq!(keys(x), keys(y), "{at}");
            for (key, value) in x {
                same_bits(value, &y[key], &format!("{at}.{key}"));
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            assert_eq!(x.len(), y.len(), "{at}");
            for (i, (x, y)) in x.iter().zip(y).enumerate() {
                same_bits(x, y, &format!("{at}[{i}]"));
            }
        }
        _ => assert_eq!(a, b, "{at}"),
    }
}

/// The library's flight of a design with `motor` put in the configuration `id`'s one mount, from
/// `builder`'s rail, recorded every 0.01 s as `hpr sim` records it.
fn library_flight(
    design: hpr::hpr_design::Rocket,
    id: &str,
    mount: &str,
    motor: Option<&hpr::Motor>,
    environment: &hpr::Environment,
    launch: impl Fn(hpr::FlightBuilder<'_>) -> hpr::FlightBuilder<'_>,
) -> (hpr::Flight, hpr::hpr_sim::Recorder) {
    library_flight_recovered(design, None, id, mount, motor, environment, launch)
}

/// As [`library_flight`], with the `.ork` file's `recovery` mapped by `hpr::ork::recovery` onto
/// the configuration flown, as a program would.
fn library_flight_recovered(
    mut design: hpr::hpr_design::Rocket,
    recovery: Option<&hpr::hpr_io::ork::Recovery>,
    id: &str,
    mount: &str,
    motor: Option<&hpr::Motor>,
    environment: &hpr::Environment,
    launch: impl Fn(hpr::FlightBuilder<'_>) -> hpr::FlightBuilder<'_>,
) -> (hpr::Flight, hpr::hpr_sim::Recorder) {
    use hpr::hpr_design::{Configuration, Ignition, MountedMotor};
    if let Some(motor) = motor {
        design.configurations.retain(|c| c.id != id);
        design.configurations.push(Configuration {
            id: id.to_owned(),
            name: String::new(),
            motors: vec![MountedMotor {
                mount: mount.to_owned(),
                designation: motor.designation().to_owned(),
                diameter_m: motor.diameter_m(),
                length_m: motor.length_m(),
                motor: motor.solid_motor().clone(),
                delay: motor.delay(),
                ignition: Ignition::Launch,
                failed_tubes: Vec::new(),
            }],
        });
    }
    let devices = recovery.map_or_else(Vec::new, |recovery| {
        let assembly = design.assemble(id).unwrap();
        hpr::ork::recovery(recovery, &design, &assembly)
            .unwrap()
            .devices
    });
    let mut rocket = hpr::Rocket::from_design(design, id).unwrap();
    for device in devices {
        rocket.add_parachute(device);
    }
    let mut recorder = hpr::hpr_sim::Recorder::new(
        hpr::hpr_sim::Channel::ALL.to_vec(),
        Some(hpr_cli::sim::DEFAULT_INTERVAL_S),
    )
    .unwrap();
    let flight = launch(hpr::Flight::builder(
        &rocket,
        environment,
        hpr_cli::sim::DEFAULT_RAIL_LENGTH_M,
    ))
    .fly_with(&mut recorder)
    .unwrap();
    (flight, recorder)
}

/// The probe read as the library reads it, with its default configuration's id and mount.
fn probe_design() -> (hpr::hpr_design::Rocket, String, String) {
    let bytes = std::fs::read(repo_file(PROBE)).unwrap();
    let file = hpr::hpr_io::ork::read(&bytes).unwrap().value;
    let design = hpr::hpr_io::ork::design(&file).value;
    let configuration = design.motors.default_configuration().unwrap();
    let id = configuration.id.clone();
    let mount = configuration.motors[0].mount.clone();
    (design.rocket, id, mount)
}

/// Checks `document`'s summary and events against the library's flight, bit for bit.
fn same_flight(document: &Value, flight: &hpr::Flight) {
    let mut summary = serde_json::to_value(flight.summary()).unwrap();
    // The library names its variants as Rust does; the command's schema, in snake case.
    summary["termination"] = "ground_hit".into();
    assert_eq!(
        flight.result().termination,
        hpr::hpr_sim::Termination::GroundHit
    );
    // Each peak's mark is the command's own: whether it came after apogee.
    let mut printed = document["summary"].clone();
    let apogee_s = summary["apogee"]["time_s"].as_f64().unwrap();
    let mut marked = 0;
    for (key, value) in printed.as_object_mut().unwrap() {
        if let Some(peak) = value.as_object_mut()
            && let Some(after) = peak.remove("after_apogee")
        {
            let time_s = peak["time_s"].as_f64().unwrap();
            assert_eq!(after, time_s > apogee_s, "{key}");
            marked += 1;
        }
    }
    assert!(marked >= 6, "{marked} peaks marked");
    same_bits(&printed, &summary, "summary");
    let events = document["events"].as_array().unwrap();
    assert_eq!(events.len(), flight.result().events.len());
    for (printed, event) in events.iter().zip(&flight.result().events) {
        let sample = &event.sample;
        for (key, value) in [
            ("time_s", sample.time_s),
            ("height_above_ground_m", sample.height_above_ground_m),
            ("speed_m_s", sample.cg_velocity_enu_m_s.length()),
        ] {
            assert_eq!(
                printed[key].as_f64().unwrap().to_bits(),
                value.to_bits(),
                "{key}"
            );
        }
    }
}

/// M4.2b's bullet: a public `.ork` flown by `hpr sim` gives the library's flight bit for bit,
/// and its JSON validates. The probe's own motor has no bundled curve, so the catalog's H54 flies
/// in its mount, as `--motor H54` asks; the library is called as a program would call it.
#[test]
fn sim_flies_a_public_ork_as_the_library_does() {
    let folder = tempfile::tempdir().unwrap();
    let csv = folder.path().join("flight.csv");
    let csv_arg = csv.to_string_lossy().into_owned();
    let design = repo_file(PROBE);
    let document = json(
        &["sim", &design, "--motor", "H54", "--export", &csv_arg],
        0,
        "sim.schema.json",
    );

    let (rocket, id, mount) = probe_design();
    let motor = hpr::Motor::from_catalog("H54").unwrap();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, recorder) = library_flight(rocket, &id, &mount, Some(&motor), &environment, |b| b);
    same_flight(&document, &flight);
    // The recording, every value printed to its last bit.
    let written = std::fs::read_to_string(&csv).unwrap();
    assert_eq!(written, hpr::hpr_sim::export::csv(&recorder).unwrap());
    assert_eq!(document["exports"][0]["rows"], recorder.rows().len());
    assert_eq!(document["exports"][0]["format"], "csv");

    assert_eq!(document["design"]["file"], "pods-none.ork");
    assert_eq!(document["design"]["format"], "ork");
    assert_eq!(document["design"]["configuration"], id.as_str());
    assert_eq!(document["motors"][0]["designation"], "168H54-10A");
    assert_eq!(document["motors"][0]["mount"], mount.as_str());
    assert_eq!(document["motors"][0]["source"]["kind"], "catalog");
    // The catalog's as-of date, as `hpr motors list` gives it: the day its files were downloaded.
    let as_of = Catalog::bundled().unwrap().snapshot.captured;
    assert_eq!(document["motors"][0]["source"]["as_of"], as_of.as_str());
    assert_eq!(document["launch"]["rail_length_m"], 1.5);
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes[0].as_str().unwrap().contains("no recovery device"),
        "{notes:?}"
    );
    // The apogee is well above the rail, and the text says what the JSON says.
    let apogee = flight.apogee_m().unwrap();
    assert!(apogee > 100.0, "{apogee}");
    let text = streams(&["sim", &design, "--motor", "H54"]).out;
    assert!(
        text.contains(&format!(
            "\napogee                {apogee:.1} m ({:.0} ft) above the site",
            apogee / FOOT_M
        )),
        "{text}"
    );
    assert!(
        text.contains(&format!(
            "168H54-10A (from the bundled catalog, as of {as_of})"
        )),
        "{text}"
    );
}

/// The `.ork` text of the library's staging example, `ork_two_stage`: two stages, an Estes F15 in
/// each, the booster dropping away at its burnout.
fn two_stage_ork() -> String {
    let example =
        std::fs::read_to_string(root().join("crates/hpr/examples/ork_two_stage.rs")).unwrap();
    let start = example.find("r#\"").unwrap() + 3;
    let end = example[start..].find("\"#").unwrap() + start;
    example[start..end].to_owned()
}

/// A parachute's `.ork` text, opening at apogee.
fn apogee_parachute(name: &str, diameter_m: f64) -> String {
    format!(
        r#"<parachute><name>{name}</name><id>{name}</id><axialoffset method="top">0.05</axialoffset>
          <packedlength>0.03</packedlength><packedradius>0.012</packedradius><cd>auto</cd>
          <material type="surface" density="0.067">Ripstop nylon</material>
          <deployevent>apogee</deployevent><deploydelay>0.0</deploydelay>
          <diameter>{diameter_m}</diameter><linecount>6</linecount><linelength>0.3</linelength>
          <linematerial type="line" density="0.0025">Nylon</linematerial></parachute>"#
    )
}

/// The library's flight of the staged `.ork` at `path`, flown as a program would: its staging
/// through `hpr::ork::separations`, its devices through `hpr::ork::separated_recovery`, on the
/// facade's flight, recorded every 0.01 s as `hpr sim` records it.
fn library_staged_flight(path: &str) -> (hpr::Flight, Vec<hpr::Device>) {
    let bytes = std::fs::read(path).unwrap();
    let file = hpr::hpr_io::ork::read(&bytes).unwrap().value;
    let ork = hpr::hpr_io::ork::design(&file).value;
    let configuration = ork.motors.default_configuration().unwrap();
    assert!(configuration.staging.is_some());
    let assembly = ork.rocket.assemble(&configuration.id).unwrap();
    let separations = hpr::ork::separations(configuration.stagings(), &assembly).unwrap();
    let devices = hpr::ork::separated_recovery(&ork.recovery, &ork.rocket, &assembly, &separations)
        .unwrap()
        .devices;
    let mut rocket = hpr::Rocket::from_design(ork.rocket.clone(), &configuration.id).unwrap();
    for device in &devices {
        rocket.add_parachute(device.clone());
    }
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let flight = hpr::Flight::builder(&rocket, &environment, hpr_cli::sim::DEFAULT_RAIL_LENGTH_M)
        .separations(separations)
        .fly()
        .unwrap();
    (flight, devices)
}

/// ADR-172: the two-stage example with a second F15 in the booster, in an inner tube lit 1 s after
/// launch, and the sustainer lit at the booster's `burnout`: the booster's first burnout is its
/// main tube's, where it separates, and `hpr sim` says the booster drops with the late F15 still
/// burning and how much of its impulse the booster's flight leaves out.
#[test]
fn a_booster_dropped_at_its_first_burnout_names_what_still_burns() {
    let two = two_stage_ork();
    // The sustainer's mount is the file's first.
    let automatic = "<ignitionevent>automatic</ignitionevent>";
    let at = two.find(automatic).unwrap();
    let mut ork = format!(
        "{}<ignitionevent>burnout</ignitionevent>{}",
        &two[..at],
        &two[at + automatic.len()..]
    );
    let fins = "<subcomponents><trapezoidfinset><name>Booster fins</name>";
    assert_eq!(ork.matches(fins).count(), 1);
    ork = ork.replace(
        fins,
        r#"<subcomponents><innertube><name>Late tube</name><id>late</id>
            <material type="bulk" density="680.0">Cardboard</material>
            <axialoffset method="top">0.0</axialoffset><length>0.12</length>
            <outerradius>0.0152</outerradius><thickness>0.0005</thickness>
            <motormount><ignitionevent>launch</ignitionevent><ignitiondelay>1.0</ignitiondelay>
              <overhang>0.0</overhang>
              <motor configid="f15-f15"><type>single</type><manufacturer>Estes</manufacturer>
                <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
                <delay>0.0</delay></motor></motormount></innertube>
          <trapezoidfinset><name>Booster fins</name>"#,
    );
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("late.ork");
    std::fs::write(&path, &ork).unwrap();

    let file = hpr::hpr_io::ork::read(ork.as_bytes()).unwrap().value;
    let design = hpr::hpr_io::ork::design(&file).value;
    let configuration = design.motors.default_configuration().unwrap();
    let staging = configuration.staging.as_ref().expect("a separation");
    let assembly = design.rocket.assemble(&configuration.id).unwrap();
    let late = assembly
        .motors
        .iter()
        .find(|motor| motor.mount == "late")
        .unwrap();
    let main = assembly
        .motors
        .iter()
        .find(|motor| motor.mount == "booster")
        .unwrap();
    // The main tube's F15, lit at launch, burns out first, and the split comes then.
    assert_eq!(staging.time_s, main.mounted.motor.burnout_time_s());
    let curve = late.mounted.motor.curve();
    let left_n_s = curve.total_impulse_ns() - curve.impulse_ns(staging.time_s - 1.0);
    assert!(left_n_s > 0.0, "{left_n_s}");

    let text = streams(&["sim", path.to_str().unwrap()]);
    let note = format!(
        "note: part 1 drops at {:.3} s with `F15` still burning, as OpenRocket drops a stage's \
         other motors at its first burnout: its flight as a point leaves out the {left_n_s:.3} N·s \
         they had left, of {:.3} N·s in all",
        staging.time_s,
        curve.total_impulse_ns()
    );
    assert!(text.err.contains(&note), "{note}\n{}", text.err);
    assert!(
        text.out
            .contains("lit 0 s after the motor in `Booster tube` burns out"),
        "{}",
        text.out
    );

    // The example as written drops nothing burning, and says nothing of it.
    let plain = folder.path().join("two-stage.ork");
    std::fs::write(&plain, two_stage_ork()).unwrap();
    let text = streams(&["sim", plain.to_str().unwrap()]).both();
    assert!(!text.contains("still burning"), "{text}");
}

/// The two-stage example with a copy of its booster stage between the two as a middle stage,
/// separating at its own burnout as the booster does: three F15s, each lit at the burnout of the
/// one below.
fn three_stage_ork() -> String {
    let two = two_stage_ork();
    let start = two.find("<stage><name>Booster</name>").unwrap();
    let end = two[start..].find("</stage>").unwrap() + start + "</stage>".len();
    let middle = two[start..end]
        .replace("Booster", "Middle")
        .replace("booster", "middle");
    let mut three = two.clone();
    three.insert_str(start, &format!("{middle}\n      "));
    three.replace("Two-stage example", "Three-stage example")
}

/// The two-stage example made a payload rocket: the top stage holds no motor and a parachute that
/// opens at the lower stage's separation, and the booster, its motor's delay `delay_s`, drops away
/// at its ejection charge with its own parachute opening there.
fn payload_ork(delay_s: f64, payload_opens: &str) -> String {
    let two = two_stage_ork();
    let start = two.find("<motormount>").unwrap();
    let end = two[start..].find("</motormount>").unwrap() + start + "</motormount>".len();
    let mut ork = two.clone();
    ork.replace_range(start..end, "");
    let parachute = |name: &str, event: &str| {
        apogee_parachute(name, 0.4).replace(
            "<deployevent>apogee</deployevent>",
            &format!("<deployevent>{event}</deployevent>"),
        )
    };
    ork.replacen(
        "<subcomponents><trapezoidfinset><name>Sustainer fins</name>",
        &format!(
            "<subcomponents>{}<trapezoidfinset><name>Sustainer fins</name>",
            parachute("Payload chute", payload_opens)
        ),
        1,
    )
    .replacen(
        "<subcomponents><trapezoidfinset><name>Booster fins</name>",
        &format!(
            "<subcomponents>{}<trapezoidfinset><name>Booster fins</name>",
            parachute("Booster chute", "ejection")
        ),
        1,
    )
    .replacen(
        "<separationevent>burnout</separationevent>",
        "<separationevent>ejection</separationevent>",
        1,
    )
    .replacen(
        "<delay>0.0</delay>",
        &format!("<delay>{delay_s}</delay>"),
        1,
    )
}

/// M4.5g3: a `.ork` payload that drops away from its spent booster at the booster's ejection
/// charge, before apogee, flies in `hpr sim` as the library flies it, bit for bit: the stack's
/// flight ends at the split, and the part that keeps the nose flies on as a point under the
/// parachute that opens there, its apogee and landing the flight's. A payload whose parachute
/// waits for apogee would coast from the split with no drag, and is refused by name (ADR-165).
#[test]
fn sim_flies_a_payload_dropped_with_nothing_left_to_burn_as_the_library_does() {
    let folder = tempfile::tempdir().unwrap();
    let ork = payload_ork(2.0, "lowerstageseparation");
    assert_eq!(ork.matches("<motor ").count(), 1);
    assert_eq!(ork.matches("<parachute>").count(), 2);
    let path = folder.path().join("payload.ork");
    std::fs::write(&path, ork).unwrap();
    let path = path.to_string_lossy().into_owned();
    let document = json(&["sim", &path], 0, "sim.schema.json");
    let (flight, devices) = library_staged_flight(&path);
    let result = flight.result();
    assert_eq!(result.termination, hpr::hpr_sim::Termination::Separated);
    // Before apogee: the stack has none, so the flight's is the payload's own.
    let split_s = result
        .event(hpr::hpr_sim::EventKind::Separation)
        .unwrap()
        .sample
        .time_s;
    assert!(result.event(hpr::hpr_sim::EventKind::Apogee).is_none());
    let nose = result.bodies.iter().find(|body| body.body == 0).unwrap();
    let apogee = nose.event(hpr::hpr_sim::EventKind::Apogee).unwrap();
    assert!(apogee.sample.time_s > split_s + 0.5, "{split_s}");
    let summary = flight.summary();
    assert_eq!(
        summary.apogee.unwrap().time_s.to_bits(),
        apogee.sample.time_s.to_bits()
    );
    let mut expected = serde_json::to_value(summary).unwrap();
    expected["termination"] = "separated".into();
    let mut printed = document["summary"].clone();
    for value in printed.as_object_mut().unwrap().values_mut() {
        if let Some(peak) = value.as_object_mut() {
            peak.remove("after_apogee");
        }
    }
    same_bits(&printed, &expected, "summary");
    // The stack's events, then the payload's own: its apogee and its landing.
    let flown: Vec<(f64, f64)> = result
        .events
        .iter()
        .map(|event| (event.sample.time_s, event.sample.height_above_ground_m))
        .chain(
            nose.events
                .iter()
                .map(|event| (event.sample.time_s, event.sample.height_above_ground_m)),
        )
        .collect();
    let events = document["events"].as_array().unwrap();
    assert_eq!(events.len(), flown.len());
    for (printed, (time_s, height_m)) in events.iter().zip(&flown) {
        assert_eq!(
            printed["time_s"].as_f64().unwrap().to_bits(),
            time_s.to_bits()
        );
        assert_eq!(
            printed["height_above_ground_m"].as_f64().unwrap().to_bits(),
            height_m.to_bits()
        );
    }
    let kinds: Vec<&str> = events
        .iter()
        .rev()
        .take(2)
        .map(|event| event["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["ground_hit", "apogee"]);

    // The payload's parachute opens at the split, on the part that keeps the nose; the booster's
    // at its own charge, the same instant; the booster's tumble, which it releases, is hpr's.
    let recovery = document["recovery"].as_array().unwrap();
    let described: Vec<(&str, &Value, &str, bool)> = recovery
        .iter()
        .map(|d| {
            (
                d["name"].as_str().unwrap(),
                &d["body"],
                d["opens_at"].as_str().unwrap(),
                d["added"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        described,
        [
            ("Payload chute", &Value::Null, "separation", false),
            ("Booster chute", &Value::from(1), "ejection", false),
            ("Booster, tumbling", &Value::from(1), "separation", true),
        ]
    );
    assert_eq!(devices[2].released_by, Some(1));
    for device in &recovery[..2] {
        assert_eq!(
            device["opened_s"].as_f64().unwrap().to_bits(),
            split_s.to_bits()
        );
    }

    let text = streams(&["sim", &path]);
    let landing = summary
        .body_landings
        .iter()
        .find(|landing| landing.body == Some(0))
        .unwrap();
    let note = format!(
        "note: the stack comes apart at {split_s:.3} s with nothing left to burn: part 1, `Booster`, drops away"
    );
    assert!(text.err.contains(&note), "{note}\n{}", text.err);
    for line in [
        "recovery: `Payload chute` at the separation, ",
        "recovery: `Booster chute` on part 1 at the ejection charge",
        // The payload's apogee, under its parachute from the split, is not a coast to tell.
        "delay                 the stack comes apart 2.00 s after burnout, before apogee; the motor's set delay: 2 s\n",
        &format!(
            "descent               {:.1} m/s ({:.0} ft/s) at landing under `Payload chute`",
            landing.descent_rate_m_s,
            landing.descent_rate_m_s / FOOT_M
        ),
        &format!(
            "landing, part 0       {:.1} m ({:.0} ft) from the pad at {:.2} s",
            landing.distance_m,
            landing.distance_m / FOOT_M,
            landing.time_s
        ),
    ] {
        assert!(text.out.contains(line), "{line}\n{}", text.out);
    }
    let all = text.both();
    assert!(!all.contains("under power"), "{all}");
    assert!(!all.contains("the flight ended"), "{all}");
    assert!(!all.contains("its charge fires"), "{all}");
    // The notes, on standard error.
    let text = text.err;
    // A dropped part that climbs above the payload's apogee is said; one that doesn't, not.
    let peak = |body: &hpr::hpr_sim::BodyFlight| {
        body.event(hpr::hpr_sim::EventKind::Apogee)
            .map(|event| event.sample)
    };
    for body in result.bodies.iter().filter(|body| body.body > 0) {
        let said = format!("note: part {} peaks at", body.body);
        match peak(body).filter(|p| p.height_above_ground_m > apogee.sample.height_above_ground_m) {
            Some(p) => assert!(
                text.contains(&format!(
                    "{said} {:.1} m ({:.0} ft) above the site at {:.2} s, above the flight's \
                     apogee",
                    p.height_above_ground_m,
                    p.height_above_ground_m / FOOT_M,
                    p.time_s
                )),
                "{text}"
            ),
            None => assert!(!text.contains(&said), "{text}"),
        }
    }
    // The figure follows the stack, which ends at the split, and says so; without one, no note.
    let ends =
        format!("note: the figure and the exported rows end at the split, at {split_s:.3} s");
    assert!(!text.contains(&ends), "{text}");
    let svg = folder.path().join("payload.svg");
    let svg = svg.to_string_lossy().into_owned();
    let plotted = streams(&["sim", &path, "--plot", &svg]).err;
    assert!(plotted.contains(&ends), "{plotted}");
    let csv = folder.path().join("payload.csv");
    let csv = csv.to_string_lossy().into_owned();
    let exported = streams(&["sim", &path, "--export", &csv]).err;
    assert!(exported.contains(&ends), "{exported}");

    // The payload's parachute set to apogee instead: from the split to its apogee it would coast
    // with no drag at all.
    let coasting = folder.path().join("coasting.ork");
    std::fs::write(&coasting, payload_ork(2.0, "apogee")).unwrap();
    let coasting = coasting.to_string_lossy().into_owned();
    let document = json_error(&["sim", &coasting], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(
        message.contains(&format!(
            "`Sustainer`, the part that keeps the nose, would coast with no drag from the \
             separation at {split_s:.3} s"
        )),
        "{message}"
    );
    // Once, as the refusal it is, and with what to change.
    assert_eq!(message.matches("would coast").count(), 1, "{message}");
    assert!(!message.contains("recovery is not flown"), "{message}");
    assert!(
        message.ends_with(
            "set one of them to open at the lower stage's separation, with no delay, for HPR Sim to \
             fly it"
        ),
        "{message}"
    );
}

/// M4.5g2: a `.ork` configuration that drops two stages under power, the booster and then the
/// middle stage, flies in `hpr sim` as the library flies it, bit for bit: each part under its own
/// devices, each dropped part tumbling from its own split, and each part's landing printed.
#[test]
fn sim_flies_two_powered_separations_as_the_library_does() {
    let folder = tempfile::tempdir().unwrap();
    // A parachute in the sustainer and one in the middle stage; the booster has none.
    let ork = three_stage_ork()
        .replacen(
            "<subcomponents><trapezoidfinset><name>Sustainer fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Sustainer fins</name>",
                apogee_parachute("Sustainer chute", 0.45)
            ),
            1,
        )
        .replacen(
            "<subcomponents><trapezoidfinset><name>Middle fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Middle fins</name>",
                apogee_parachute("Middle chute", 0.3)
            ),
            1,
        );
    assert_eq!(ork.matches("<parachute>").count(), 2);
    assert_eq!(ork.matches("<stage>").count(), 3);
    let path = folder.path().join("three-stage.ork");
    std::fs::write(&path, ork).unwrap();
    let path = path.to_string_lossy().into_owned();
    let document = json(&["sim", &path], 0, "sim.schema.json");
    let (flight, devices) = library_staged_flight(&path);
    same_flight(&document, &flight);
    let separations_s: Vec<f64> = flight
        .result()
        .events
        .iter()
        .filter(|event| event.kind == hpr::hpr_sim::EventKind::Separation)
        .map(|event| event.sample.time_s)
        .collect();
    assert_eq!(separations_s.len(), 2);
    assert!(separations_s[0] < separations_s[1], "{separations_s:?}");
    // Both dropped parts land, the booster as part 1 and the middle as part 2.
    let landings = &flight.summary().body_landings;
    assert_eq!(
        landings
            .iter()
            .map(|landing| landing.body)
            .collect::<Vec<_>>(),
        [Some(1), Some(2)]
    );
    assert_eq!(document["summary"]["body_landings"][1]["body"], 2);

    // Each device on its part; each dropped part tumbles from its own split, and the middle's
    // parachute releases its tumble.
    let recovery = document["recovery"].as_array().unwrap();
    let described: Vec<(&str, &Value, &str, bool)> = recovery
        .iter()
        .map(|d| {
            (
                d["name"].as_str().unwrap(),
                &d["body"],
                d["opens_at"].as_str().unwrap(),
                d["added"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        described,
        [
            ("Sustainer chute", &Value::Null, "apogee", false),
            ("Middle chute", &Value::from(2), "apogee", false),
            ("Booster, tumbling", &Value::from(1), "separation", true),
            ("Middle, tumbling", &Value::from(2), "separation", true),
        ]
    );
    assert_eq!(devices[3].released_by, Some(1));
    assert_eq!(devices[2].released_by, None);
    assert_eq!(recovery[2]["opened_s"], separations_s[0]);
    assert_eq!(recovery[3]["opened_s"], separations_s[1]);

    let text = streams(&["sim", &path]);
    for line in [
        "recovery: `Middle chute` on part 2 at apogee",
        "recovery: `Booster, tumbling` on part 1 at the separation",
        "recovery: `Middle, tumbling` on part 2 at the separation",
        "lit 0 s after the motor in `Middle tube` burns out",
        "lit 0 s after the motor in `Booster tube` burns out",
        &format!(
            "landing, part 2       {:.1} m ({:.0} ft) from the pad at {:.2} s",
            landings[1].distance_m,
            landings[1].distance_m / FOOT_M,
            landings[1].time_s
        ),
    ] {
        assert!(text.out.contains(line), "{line}\n{}", text.out);
    }
    for line in [
        "note: the stack comes apart under power 2 times: at ",
        "part 2, `Middle`, drops away",
    ] {
        assert!(text.err.contains(line), "{line}\n{}", text.err);
    }
}

/// M4.5g1: a `.ork` configuration that drops its booster under power flies in `hpr sim` as the
/// library flies it, bit for bit: each part under its own parachute, the booster tumbling from
/// the split until its parachute opens, and each part's landing printed. With no device on a
/// part, the booster tumbles to the ground and the sustainer tumbles from its apogee.
#[test]
fn sim_flies_a_powered_separation_as_the_library_does() {
    let folder = tempfile::tempdir().unwrap();
    // A parachute in each stage's tube.
    let ork = two_stage_ork()
        .replacen(
            "<subcomponents><trapezoidfinset><name>Sustainer fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Sustainer fins</name>",
                apogee_parachute("Sustainer chute", 0.45)
            ),
            1,
        )
        .replacen(
            "<subcomponents><trapezoidfinset><name>Booster fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Booster fins</name>",
                apogee_parachute("Booster chute", 0.3)
            ),
            1,
        );
    assert_eq!(ork.matches("<parachute>").count(), 2);
    let path = folder.path().join("two-stage-chutes.ork");
    std::fs::write(&path, ork).unwrap();
    let path = path.to_string_lossy().into_owned();
    let map = folder.path().join("chutes.geojson");
    let map_arg = map.to_string_lossy().into_owned();
    let document = json(&["sim", &path, "--export", &map_arg], 0, "sim.schema.json");
    let (flight, devices) = library_staged_flight(&path);
    same_flight(&document, &flight);
    // The booster's landing is among the summary's, which `same_flight` holds to the bit.
    let landings = &flight.summary().body_landings;
    assert_eq!(landings.len(), 1);
    assert_eq!(landings[0].body, Some(1));
    assert_eq!(document["summary"]["body_landings"][0]["body"], 1);

    // The devices: each parachute on its part, and the booster's tumble from the split, released
    // as its parachute opens.
    let recovery = document["recovery"].as_array().unwrap();
    let described: Vec<(&str, &Value, &str, bool)> = recovery
        .iter()
        .map(|d| {
            (
                d["name"].as_str().unwrap(),
                &d["body"],
                d["opens_at"].as_str().unwrap(),
                d["added"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        described,
        [
            ("Sustainer chute", &Value::Null, "apogee", false),
            ("Booster chute", &Value::from(1), "apogee", false),
            ("Booster, tumbling", &Value::from(1), "separation", true),
        ]
    );
    // The sustainer's landing is pinned on the map, under its parachute; the booster's, rough
    // whatever opened, is not.
    let pinned = std::fs::read_to_string(&map).unwrap();
    assert!(pinned.contains("\"landing\""), "{pinned}");
    assert!(!pinned.contains("body 1"), "{pinned}");
    assert_eq!(devices[2].released_by, Some(1));
    let separated_s = flight
        .result()
        .event(hpr::hpr_sim::EventKind::Separation)
        .unwrap()
        .sample
        .time_s;
    let booster = &flight.result().bodies[0];
    let opened = |index: usize| {
        booster
            .event(hpr::hpr_sim::EventKind::Deployment(index))
            .unwrap()
            .sample
            .time_s
    };
    assert_eq!(recovery[2]["opened_s"], separated_s);
    assert_eq!(recovery[1]["opened_s"], opened(1));
    assert!(opened(1) > separated_s, "{} {separated_s}", opened(1));
    // The booster's parachute releases its tumble as it opens.
    assert_eq!(
        booster
            .event(hpr::hpr_sim::EventKind::Release(2))
            .map(|event| event.sample.time_s),
        Some(opened(1))
    );
    let apogee_s = flight.apogee_time_s().unwrap();
    assert_eq!(recovery[0]["opened_s"], apogee_s);

    let text = streams(&["sim", &path]);
    assert!(
        text.err
            .contains("note: the stack comes apart under power at "),
        "{}",
        text.err
    );
    for line in [
        "recovery: `Booster chute` on part 1 at apogee",
        "recovery: `Booster, tumbling` on part 1 at the separation",
        "lit 0 s after the motor in `Booster tube` burns out",
        // The sustainer flies on: its coast from its own burnout to apogee is told.
        "delay                 apogee ",
        &format!(
            "landing, part 1       {:.1} m ({:.0} ft) from the pad at {:.2} s",
            landings[0].distance_m,
            landings[0].distance_m / FOOT_M,
            landings[0].time_s
        ),
    ] {
        assert!(text.out.contains(line), "{line}\n{}", text.out);
    }
    let text = text.both();
    assert!(!text.contains("before apogee;"), "{text}");
    assert!(!text.contains("fly as one stack"), "{text}");

    // The example as written, with no device: each part tumbles.
    let bare = folder.path().join("two-stage.ork");
    std::fs::write(&bare, two_stage_ork()).unwrap();
    let bare = bare.to_string_lossy().into_owned();
    let document = json(&["sim", &bare], 0, "sim.schema.json");
    let (flight, _) = library_staged_flight(&bare);
    same_flight(&document, &flight);
    let names: Vec<&str> = document["recovery"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Sustainer, tumbling", "Booster, tumbling"]);
    // hpr's own tumble brakes no fall it predicts: the sustainer's landing keeps its caveat.
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes.iter().any(|note| note
            .as_str()
            .unwrap()
            .starts_with("the sustainer has no device of the file's that opens")),
        "{notes:?}"
    );
    let text = streams(&["sim", &bare]).out;
    let landing = text
        .lines()
        .find(|line| line.starts_with("landing   "))
        .unwrap();
    assert!(
        landing.ends_with(": with no recovery device opened soon after apogee, not a prediction"),
        "{text}"
    );

    // A separation edited to a boundary with no stage aft of it is refused, not flown.
    let converted = folder.path().join("two-stage.hpr");
    let converted = converted.to_string_lossy().into_owned();
    assert_eq!(hpr(&["convert", &bare, &converted]).status.code(), Some(0));
    let written = std::fs::read_to_string(&converted).unwrap();
    assert_eq!(
        written.matches("\"after_stage\": 0").count(),
        1,
        "{written}"
    );
    let edited = folder.path().join("edited.hpr");
    std::fs::write(
        &edited,
        written.replace("\"after_stage\": 0", "\"after_stage\": 5"),
    )
    .unwrap();
    let edited = edited.to_string_lossy().into_owned();
    let document = json_error(&["sim", &edited], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(
        message.contains("no stage is aft of the boundary after stage 6"),
        "{message}"
    );
    // The converted design flies the same separation.
    let document = json(&["sim", &converted], 0, "sim.schema.json");
    same_flight(&document, &flight);
}

/// M4.5a: a `.ork`'s parachutes fly in `hpr sim` as the library maps and flies them, bit for bit:
/// the Loft demo's drogue at apogee and its main at 150 m, on the catalog's H54. The landing is a
/// prediction once a device opened, so its caveat goes and the map keeps its landing.
#[test]
fn sim_flies_a_ork_s_recovery_as_the_library_does() {
    let file = "validation/fixtures/ork/loft-demo/demo-dual-deploy.ork";
    let folder = tempfile::tempdir().unwrap();
    let csv = folder.path().join("flight.csv");
    let geojson = folder.path().join("flight.geojson");
    let (csv_arg, geojson_arg) = (
        csv.to_string_lossy().into_owned(),
        geojson.to_string_lossy().into_owned(),
    );
    let design = repo_file(file);
    let document = json(
        &[
            "sim",
            &design,
            "--motor",
            "H54",
            "--export",
            &csv_arg,
            "--export",
            &geojson_arg,
        ],
        0,
        "sim.schema.json",
    );

    let bytes = std::fs::read(&design).unwrap();
    let read = hpr::hpr_io::ork::read(&bytes).unwrap().value;
    let ork = hpr::hpr_io::ork::design(&read).value;
    let configuration = ork.motors.default_configuration().unwrap();
    let (id, mount) = (
        configuration.id.clone(),
        configuration.motors[0].mount.clone(),
    );
    let motor = hpr::Motor::from_catalog("H54").unwrap();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, recorder) = library_flight_recovered(
        ork.rocket.clone(),
        Some(&ork.recovery),
        &id,
        &mount,
        Some(&motor),
        &environment,
        |b| b,
    );
    same_flight(&document, &flight);
    assert_eq!(
        std::fs::read_to_string(&csv).unwrap(),
        hpr::hpr_sim::export::csv(&recorder).unwrap()
    );

    // Both devices, in the file's order, each opened when the library's flight opened it.
    let recovery = document["recovery"].as_array().unwrap();
    let names: Vec<&str> = recovery
        .iter()
        .map(|d| d["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Main parachute", "Drogue parachute"]);
    assert_eq!(recovery[0]["opens_at"], "altitude");
    assert_eq!(recovery[0]["height_above_ground_m"], 150.0);
    assert_eq!(recovery[1]["opens_at"], "apogee");
    for (index, device) in recovery.iter().enumerate() {
        let opened = flight
            .result()
            .events
            .iter()
            .find(|event| event.kind == hpr::hpr_sim::EventKind::Deployment(index))
            .map(|event| event.sample.time_s)
            .unwrap();
        assert_eq!(
            device["opened_s"].as_f64().unwrap().to_bits(),
            opened.to_bits()
        );
    }
    let drogue = recovery[1]["drag_area_m2"].as_f64().unwrap();
    assert_eq!(drogue, 0.8 * std::f64::consts::PI * 0.46 * 0.46 / 4.0);

    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes.iter().any(|n| n
            .as_str()
            .unwrap()
            .starts_with("the file's 2 recovery devices fly as OpenRocket flies them")),
        "{notes:?}"
    );
    assert!(
        !notes
            .iter()
            .any(|n| n.as_str().unwrap().contains("not a prediction")),
        "{notes:?}"
    );
    // The map keeps the landing the flight predicts.
    let map = std::fs::read_to_string(&geojson).unwrap();
    assert!(map.contains("anding"), "{map}");
    let text = streams(&["sim", &design, "--motor", "H54"]);
    assert!(
        text.out
            .contains("\nrecovery: `Drogue parachute` at apogee, 0.133 m² of drag area, opened at"),
        "{}",
        text.out
    );
    let text = text.both();
    assert!(!text.contains("not a prediction"), "{text}");
}

/// A motor file flies as the library reads it: the F15's `.rse` and the H54's `.eng`, each in
/// the probe's mount, and every recording format is written.
#[test]
fn sim_flies_a_motor_file_as_the_library_does() {
    let (rocket, id, mount) = probe_design();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    for (file, format) in [
        ("curves/5f923edb1bca5800041716ab.rse", "rse"),
        ("curves/5f4294d20002e90000000735.eng", "eng"),
    ] {
        let path = curve_file(file);
        let text = std::fs::read_to_string(&path).unwrap();
        let motor = match format {
            "rse" => hpr::Motor::from_rse(&text).unwrap(),
            _ => hpr::Motor::from_eng(&text).unwrap(),
        };
        let folder = tempfile::tempdir().unwrap();
        let exports: Vec<String> = ["f.csv", "f.json", "f.parquet", "f.geojson", "f.kml"]
            .iter()
            .map(|name| folder.path().join(name).to_string_lossy().into_owned())
            .collect();
        let design = repo_file(PROBE);
        let mut args = vec!["sim", design.as_str(), "--motor", path.as_str()];
        for export in &exports {
            args.extend(["--export", export.as_str()]);
        }
        let document = json(&args, 0, "sim.schema.json");
        let (flight, recorder) = library_flight(
            rocket.clone(),
            &id,
            &mount,
            Some(&motor),
            &environment,
            |b| b,
        );
        same_flight(&document, &flight);
        assert_eq!(document["motors"][0]["source"]["kind"], "file", "{file}");
        assert_eq!(document["motors"][0]["source"]["format"], format, "{file}");
        assert_eq!(document["motors"][0]["designation"], motor.designation());
        let written: Vec<&str> = document["exports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["format"].as_str().unwrap())
            .collect();
        assert_eq!(written, ["csv", "json", "parquet", "geojson", "kml"]);
        assert_eq!(
            std::fs::read(&exports[2]).unwrap(),
            hpr::hpr_sim::export::parquet(&recorder).unwrap()
        );
        // With no recovery device opened, the maps pin no landing: only the path.
        let map: Value =
            serde_json::from_str(&std::fs::read_to_string(&exports[3]).unwrap()).unwrap();
        let kinds: Vec<&str> = map["features"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["geometry"]["type"].as_str().unwrap())
            .collect();
        assert_eq!(kinds, ["LineString"], "{file}");
        let kml = std::fs::read_to_string(&exports[4]).unwrap();
        assert!(
            kml.contains("<LineString>") && !kml.contains("<Point>"),
            "{file}"
        );
        for export in &exports {
            assert!(!std::fs::read(export).unwrap().is_empty(), "{export}");
        }
    }
}

/// An hpr design file flies with its own motor, from a site, a leaning rail and a wind given on
/// the command line, as the library flies it with the same.
#[test]
fn sim_flies_an_hpr_design_from_a_given_launch() {
    let path = repo_file("validation/designs/synthetic-54mm-three-fin.json");
    let document = json(
        &[
            "sim",
            &path,
            "--latitude",
            "32.99",
            "--longitude",
            "-106.97",
            "--elevation",
            "1400",
            "--rail-length",
            "1.5",
            "--inclination",
            "85",
            "--heading",
            "90",
            "--wind",
            "5",
            "--wind-from",
            "-90",
        ],
        0,
        "sim.schema.json",
    );
    let design: hpr::hpr_design::Rocket =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    // A wind from -90°, which is 270°: a west wind.
    let environment = hpr::Environment::new(32.99, -106.97, 1400.0)
        .unwrap()
        .with_constant_wind(5.0, -90.0)
        .unwrap();
    let (flight, _) = library_flight(design.clone(), "i175", "", None, &environment, |b| {
        b.inclination_deg(85.0).heading_deg(90.0)
    });
    same_flight(&document, &flight);
    assert_eq!(document["design"]["format"], "hpr_json");
    assert_eq!(document["motors"][0]["source"]["kind"], "design");
    assert_eq!(document["motors"][0]["ignition"], "at launch");
    assert_eq!(document["motors"][0]["count"], 1);
    assert_eq!(document["launch"]["longitude_deg"], -106.97);
    assert_eq!(document["launch"]["wind_from_deg"], -90.0);
    // The rail leans east, downwind: the rocket climbs to apogee east of where the same rail
    // leaning west, into the wind, puts it. (Where it lands isn't asserted: with no recovery
    // device the descent isn't a prediction, #241.)
    let (upwind, _) = library_flight(design, "i175", "", None, &environment, |b| {
        b.inclination_deg(85.0).heading_deg(270.0)
    });
    let east_at_apogee = |flight: &hpr::Flight| {
        let mut events = flight.result().events.iter();
        let apogee = events
            .rfind(|e| e.kind == hpr::hpr_sim::EventKind::Apogee)
            .unwrap();
        apogee.sample.cg_enu_m.x
    };
    let (east, west) = (east_at_apogee(&flight), east_at_apogee(&upwind));
    assert!(east > west + 10.0, "{east} m, {west} m");
}

/// What `hpr sim` refuses, each with status 1 and an error document naming the reason.
#[test]
fn sim_refuses_what_it_cant_fly() {
    let folder = tempfile::tempdir().unwrap();
    let probe = repo_file(PROBE);
    let refused = |args: &[&str], reason: &str| {
        let document = json_error(args, 1, "input");
        let message = said(&document);
        assert!(message.contains(reason), "{args:?}: {message}");
        assert_eq!(document["error"]["command"], "sim");
        let output = hpr(args);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(text(&output.stderr).contains(reason));
    };
    // The file's own motor has no curve in the bundled catalog.
    refused(&["sim", &probe], "no thrust curve for H128W");
    refused(&["sim", &probe], "give a motor with --motor");
    refused(
        &["sim", &probe, "--config", "nope"],
        "no motor configuration `nope`",
    );
    refused(
        &["sim", &probe, "--motor", "H54", "--mount", "nope"],
        "no motor mount `nope`",
    );
    refused(
        &["sim", &probe, "--motor", "Z9999"],
        "Z9999 is not in HPR Sim's cache of ThrustCurve.org, and the run is offline (HPR_OFFLINE is \
         set)\nhelp: with a network connection, `hpr motors fetch Z9999` fetches it into the cache",
    );
    refused(
        &["sim", &probe, "--motor", "I175"],
        "matches several motors",
    );
    let text_export = folder.path().join("f.txt");
    refused(
        &[
            "sim",
            &probe,
            "--motor",
            "H54",
            "--export",
            &text_export.to_string_lossy(),
        ],
        "--export writes",
    );
    assert!(!text_export.exists());
    refused(&["sim", "missing.ork"], "missing.ork");
    refused(
        &["sim", "design.rkt"],
        "reads an OpenRocket .ork file, an HPR design (.hpr or .hprz), or a rocket's JSON (.json)",
    );
    let not_json = folder.path().join("design.json");
    std::fs::write(&not_json, "{}").unwrap();
    refused(&["sim", &not_json.to_string_lossy()], "not a rocket's JSON");
    refused(
        &[
            "sim",
            &repo_file("validation/fixtures/ork/loft-demo/demo-multi-config.ork"),
            "--motor",
            "H54",
        ],
        "--accept-design-errors",
    );

    // The two-stage design with its sustainer lit by the separation, which never comes.
    let path = repo_file("validation/designs/synthetic-two-stage-75mm-54mm.json");
    let mut design: hpr::hpr_design::Rocket =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    for motor in &mut design.configurations[0].motors {
        if motor.mount == "sustainer-motor-mount" {
            motor.ignition = hpr::hpr_design::Ignition::Separation { delay_s: 1.0 };
        }
    }
    let separated = folder.path().join("separated.json");
    std::fs::write(&separated, serde_json::to_string(&design).unwrap()).unwrap();
    refused(
        &["sim", &separated.to_string_lossy()],
        "at its stage's separation",
    );

    // The staging example's rocket: its booster drops away under power at a time its own
    // motors set, so another motor can't take their place.
    let staged = folder.path().join("two-stage.ork");
    std::fs::write(&staged, two_stage_ork()).unwrap();
    let staged = staged.to_string_lossy().into_owned();
    refused(
        &["sim", &staged, "--motor", "F15"],
        "stage 1 drops away at 3.450 s, a time set by the file's motors",
    );
    // A payload's split, with nothing left to burn, is refused the same way, not as one under
    // power.
    let payload = folder.path().join("payload.ork");
    std::fs::write(&payload, payload_ork(2.0, "lowerstageseparation")).unwrap();
    let payload = payload.to_string_lossy().into_owned();
    let document = json_error(&["sim", &payload, "--motor", "F15"], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(
        message.contains("a time set by the file's motors"),
        "{message}"
    );
    assert!(!message.contains("under power"), "{message}");

    // The probe with a parallel stage hpr doesn't read, one with no body component in it: its
    // airframe isn't the file's, so no motor flies it, though its configuration's first reason is
    // the missing curve.
    let xml = std::fs::read_to_string(&probe).unwrap();
    let at = xml.find("<bodytube>").unwrap();
    let inside = xml[at..].find("<subcomponents>").unwrap() + at + "<subcomponents>".len();
    let boosters = format!(
        "{}<parallelstage><name>Boosters</name><id>boosters</id>\
         <instancecount>2</instancecount></parallelstage>{}",
        &xml[..inside],
        &xml[inside..]
    );
    let reduced = folder.path().join("reduced.ork");
    std::fs::write(&reduced, boosters).unwrap();
    let reduced = reduced.to_string_lossy().into_owned();
    refused(&["sim", &reduced], "no thrust curve for H128W");
    refused(
        &["sim", &reduced, "--motor", "H54"],
        "not read exactly as written",
    );
    // The same from its .hpr and .hprz: the document keeps why, though the .ork written from it
    // need not say.
    for extension in ["hpr", "hprz"] {
        let converted = folder.path().join(format!("reduced.{extension}"));
        let converted = converted.to_string_lossy().into_owned();
        assert_eq!(
            hpr(&["convert", &reduced, &converted]).status.code(),
            Some(0)
        );
        refused(&["sim", &converted], "no thrust curve for H128W");
        refused(
            &["sim", &converted, "--motor", "H54"],
            "the airframe was not read exactly as written: a parallel stage with no nose cone, \
             body tube or transition in it",
        );
    }
    // As a 0.1 document, which didn't record why, nothing shows whether the airframe was read as
    // written, so no other motor flies it either.
    let converted = folder.path().join("reduced.hpr");
    let mut document: Value =
        serde_json::from_str(&std::fs::read_to_string(&converted).unwrap()).unwrap();
    let object = document.as_object_mut().unwrap();
    object.insert("version".to_owned(), Value::from("0.1"));
    let files = object.remove("source_files").unwrap();
    object.insert("attachments".to_owned(), files);
    let source = document["provenance"]["source"].as_object_mut().unwrap();
    source.remove("airframe_not_as_written").unwrap();
    let old = folder.path().join("reduced-0.1.hpr");
    std::fs::write(&old, document.to_string()).unwrap();
    refused(
        &["sim", &old.to_string_lossy(), "--motor", "H54"],
        "whether the airframe was read exactly as written is unknown: the document is from \
         version 0.1",
    );
    // A document whose provenance says nothing flies another motor, with a note on the parts it
    // keeps aside.
    document["version"] = Value::from("0.2");
    let object = document.as_object_mut().unwrap();
    let files = object.remove("attachments").unwrap();
    object.insert("source_files".to_owned(), files);
    let edited = folder.path().join("edited.hpr");
    std::fs::write(&edited, document.to_string()).unwrap();
    let flown = json(
        &["sim", &edited.to_string_lossy(), "--motor", "H54"],
        0,
        "sim.schema.json",
    );
    assert!(
        flown["notes"].as_array().unwrap().iter().any(|note| note
            == "the file has parts HPR Sim keeps aside instead of flying, such as a parallel stage: \
                the rocket flown is the rest of it"),
        "{flown:#}"
    );
}

/// `--delay` gives `--motor`'s motor its ejection delay: the charge fires that long after
/// burnout, so a parachute set to the charge opens; `P` plugs the motor, and without `--delay`
/// it fires no charge. The label names the delay; a delay without `--motor`, below zero or not a
/// number is refused.
#[test]
fn sim_delay_sets_the_motors_charge() {
    let path = repo_file("validation/fixtures/ork/guides/level-1.ork");
    let flown = |delay: &[&str]| {
        let mut args = vec!["sim", path.as_str(), "--motor", "H125-CT"];
        args.extend(delay);
        json(&args, 0, "sim.schema.json")
    };
    let burnout = |document: &Value| {
        document["events"]
            .as_array()
            .unwrap()
            .iter()
            .find(|event| event["kind"] == "burnout")
            .unwrap()["time_s"]
            .as_f64()
            .unwrap()
    };
    let opened = |document: &Value| document["recovery"][0]["opened_s"].as_f64();

    let ten = flown(&["--delay", "10"]);
    assert_eq!(
        ten["design"]["configuration_label"],
        "[H170M-10] with --motor H125-CT --delay 10"
    );
    assert_eq!(ten["motors"][0]["delay"]["kind"], "seconds");
    assert_eq!(ten["motors"][0]["delay"]["value"], 10.0);
    let after_burnout = opened(&ten).unwrap() - burnout(&ten);
    assert!((after_burnout - 10.0).abs() < 1e-6, "{after_burnout} s");
    // A shorter delay opens it earlier, by the difference.
    let six = flown(&["--delay", "6"]);
    let earlier = opened(&ten).unwrap() - opened(&six).unwrap();
    assert!((earlier - 4.0).abs() < 1e-6, "{earlier} s");

    for (delay, label) in [(&["--delay", "p"][..], "P"), (&[][..], "")] {
        let unopened = flown(delay);
        assert_eq!(opened(&unopened), None, "{delay:?}");
        if label.is_empty() {
            assert!(unopened["motors"][0]["delay"].is_null(), "{delay:?}");
        } else {
            assert_eq!(unopened["motors"][0]["delay"]["kind"], "plugged");
        }
        assert_eq!(
            unopened["design"]["configuration_label"],
            if label.is_empty() {
                "[H170M-10] with --motor H125-CT".to_owned()
            } else {
                format!("[H170M-10] with --motor H125-CT --delay {label}")
            }
        );
    }

    for refused in [
        &["sim", path.as_str(), "--delay", "10"][..],
        &["sim", path.as_str(), "--motor", "H125-CT", "--delay", "nan"],
        &["sim", path.as_str(), "--motor", "H125-CT", "--delay", "-3"],
        &["sim", path.as_str(), "--motor", "H125-CT", "--delay", "ten"],
    ] {
        assert_eq!(hpr(refused).status.code(), Some(2), "{refused:?}");
    }
}

/// A design flown past its failed checks says so in its notes.
#[test]
fn sim_notes_accepted_design_errors() {
    let path = repo_file("validation/fixtures/ork/loft-demo/demo-multi-config.ork");
    let document = json(
        &["sim", &path, "--motor", "H54", "--accept-design-errors"],
        0,
        "sim.schema.json",
    );
    let notes: Vec<&str> = document["notes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|note| note.as_str().unwrap())
        .collect();
    assert!(
        notes.iter().any(|note| note.contains(
            "the motor in `Motor mount tube` is 29.0 mm wide, wider than the mount's 28.0 mm \
                 bore by more than a real case's slack (configuration [H128W-6] with --motor \
                 H54)"
        )),
        "{notes:?}"
    );
    assert!(
        notes
            .iter()
            .any(|note| note
                .starts_with("the file's 1 recovery device flies as OpenRocket flies it")),
        "{notes:?}"
    );
}

/// The probe's own motor, as written: an AeroTech H128W with OpenRocket's digest of its curve.
const PROBE_MOTOR: &str = "<manufacturer>AeroTech</manufacturer><designation>H128W</designation>\
                           <digest>501239de7374691072270406476cb243</digest>\
                           <diameter>0.029</diameter><length>0.194</length>";

/// The probe with `from` replaced by `to`, written to `folder` as `name`; its path.
fn probe_with(folder: &Path, name: &str, from: &str, to: &str) -> String {
    let xml = std::fs::read_to_string(repo_file(PROBE)).unwrap();
    assert_eq!(xml.matches(from).count(), 1, "{from}");
    let path = folder.join(name);
    std::fs::write(&path, xml.replacen(from, to, 1)).unwrap();
    path.to_string_lossy().into_owned()
}

/// A `.ork` flies its own configuration as the library flies it, when its motor is one the
/// catalog holds: the probe with the H54 written in place of its H128W.
#[test]
fn sim_flies_a_orks_own_motor_as_the_library_does() {
    let folder = tempfile::tempdir().unwrap();
    let path = probe_with(
        folder.path(),
        "own.ork",
        PROBE_MOTOR,
        "<manufacturer>Cesaroni Technology</manufacturer><designation>168H54-10A</designation>\
         <diameter>0.029</diameter><length>0.187</length>",
    );
    let document = json(&["sim", &path], 0, "sim.schema.json");
    let file = hpr::hpr_io::ork::read(&std::fs::read(&path).unwrap())
        .unwrap()
        .value;
    let design = hpr::hpr_io::ork::design(&file).value;
    let id = design.motors.default_configuration().unwrap().id.clone();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, _) = library_flight(design.rocket, &id, "", None, &environment, |b| b);
    same_flight(&document, &flight);
    // The file holds no curve: the reader found it in the bundled catalog, which the source
    // dates (ADR-214).
    assert_eq!(
        document["motors"][0]["source"],
        json!({"kind": "catalog", "as_of": Catalog::bundled().unwrap().snapshot.captured})
    );
    assert_eq!(document["motors"][0]["designation"], "168H54-10A");
    assert_eq!(document["design"]["configurations"][0]["flies"], true);
}

/// The replay of ThrustCurve.org's answers (ADR-145: stand-in searches, recorded downloads).
fn thrustcurve_replay() -> hpr::hpr_net::Replay {
    hpr::hpr_net::Replay::open(root().join("crates/hpr-net/tests/fixtures/replay")).unwrap()
}

/// A cache holding what ThrustCurve.org answers for `wanted`, as a fetch with a network
/// connection leaves it.
fn cache_with(wanted: &hpr::hpr_net::on_demand::Wanted) -> tempfile::TempDir {
    use hpr::hpr_net::{Cache, Client, Mode};
    let cache = tempfile::tempdir().unwrap();
    let online = Client::new(thrustcurve_replay(), Cache::new(cache.path()), Mode::Online);
    // Now, so the copy is fresh, as a fetch just made leaves it.
    let now_s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    hpr::hpr_net::on_demand::find(&online, wanted, now_s).unwrap();
    cache
}

/// F27R/L's RockSim file, as ThrustCurve.org served it (public domain): the motor a fetch flies.
fn f27() -> hpr::Motor {
    let answer: Value =
        serde_json::from_slice(
            &std::fs::read(root().join(
                "crates/hpr-net/tests/fixtures/replay/thrustcurve-download-F27R_L-rocksim.json",
            ))
            .unwrap(),
        )
        .unwrap();
    let file = hpr::hpr_net::thrustcurve::parse_download(answer.to_string().as_bytes()).unwrap();
    hpr::Motor::from_rse(&file.results[0].text().unwrap()).unwrap()
}

/// `--motor` with a name the bundled catalog lacks flies ThrustCurve.org's curve for it, from the
/// cache with no network, as the library flies that file; with an empty cache it is refused with
/// the command that fetches it.
#[test]
fn sim_flies_a_fetched_motor_the_catalog_lacks() {
    let cache = cache_with(&hpr::hpr_net::on_demand::Wanted::named("F27R/L"));
    let (rocket, id, mount) = probe_design();
    let probe = repo_file(PROBE);
    let document = json_cached(
        &["sim", &probe, "--motor", "F27R/L"],
        0,
        "sim.schema.json",
        cache.path(),
    );
    let motor = f27();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, _) = library_flight(rocket, &id, &mount, Some(&motor), &environment, |b| b);
    same_flight(&document, &flight);
    let source = &document["motors"][0]["source"];
    assert_eq!(source["kind"], "thrust_curve");
    assert_eq!(source["designation"], "F27R/L");
    assert_eq!(source["motor_id"], "5f4294d200023100000001d4");
    assert_eq!(source["format"], "rse");
    assert_eq!(
        (&source["measured_by"], &source["license"]),
        (&json!("user"), &json!("PD"))
    );
    assert_eq!(source["read_from"]["kind"], "cache");
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes
            .iter()
            .any(|n| n == hpr::hpr_net::thrustcurve::ATTRIBUTION),
        "{notes:?}"
    );

    let document = json_error(&["sim", &probe, "--motor", "F27R/L"], 1, "input");
    assert!(
        said(&document).ends_with(
            "\nhelp: with a network connection, `hpr motors fetch F27R/L` fetches it into the \
             cache"
        ),
        "{document:#}"
    );
}

/// A `.ork` motor with no curve in the file or the bundled catalog flies ThrustCurve.org's curve
/// found by its manufacturer and designation, supplied for its digest, as the library flies that
/// supply; the refusal with an empty cache names the command whose cache the flight reads.
#[test]
fn sim_fetches_a_orks_missing_motor_by_maker_and_designation() {
    use hpr::hpr_io::ork;
    let folder = tempfile::tempdir().unwrap();
    let digest = "0123456789abcdef0123456789abcdef";
    let path = probe_with(
        folder.path(),
        "f27.ork",
        PROBE_MOTOR,
        &format!(
            "<manufacturer>AeroTech</manufacturer><designation>F27R/L</designation>\
             <digest>{digest}</digest><diameter>0.029</diameter><length>0.083</length>"
        ),
    );
    let refused = json_error(&["sim", &path], 1, "input");
    let message = said(&refused);
    let command = "hpr motors fetch --manufacturer AeroTech F27R/L";
    assert!(message.contains(&format!("`{command}`")), "{message}");

    let cache = cache_with(&hpr::hpr_net::on_demand::Wanted::by("AeroTech", "F27R/L"));
    let document = json_cached(&["sim", &path], 0, "sim.schema.json", cache.path());
    let motor = f27();
    let mut supplied = ork::SuppliedCurves::new("ThrustCurve.org");
    let case = ork::CaseSize {
        diameter_m: motor.diameter_m(),
        length_m: motor.length_m(),
    };
    supplied
        .insert(digest, case, motor.solid_motor().clone())
        .unwrap();
    let file = ork::read(&std::fs::read(&path).unwrap()).unwrap().value;
    let design = ork::design_with(&file, &supplied).value;
    let id = design.motors.default_configuration().unwrap().id.clone();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, _) = library_flight(design.rocket, &id, "", None, &environment, |b| b);
    same_flight(&document, &flight);
    let source = &document["motors"][0]["source"];
    assert_eq!(source["kind"], "thrust_curve");
    assert_eq!(source["manufacturer"], "AeroTech");
    assert_eq!(document["motors"][0]["designation"], "F27R/L");
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes.iter().any(|n| n.as_str().unwrap().starts_with(
            "`F27R/L` has no curve in the file or the bundled catalog, so it flies \
             ThrustCurve.org's AeroTech F27R/L"
        )),
        "{notes:?}"
    );
}

/// `hpr motors fetch` reads a motor from the cache offline, and says how `hpr sim` flies it.
#[test]
fn motors_fetch_reads_a_cached_motor() {
    let cache = cache_with(&hpr::hpr_net::on_demand::Wanted::named("F27R/L"));
    let document = json_cached(
        &["motors", "fetch", "F27R/L", "--offline"],
        0,
        "motors-fetch.schema.json",
        cache.path(),
    );
    assert_eq!(document["motor"]["designation"], "F27R/L");
    assert_eq!(document["motor"]["read_from"]["kind"], "cache");
    assert_eq!(
        document["attribution"][0],
        hpr::hpr_net::thrustcurve::ATTRIBUTION
    );
    let fetched = streams_cached(&["motors", "fetch", "F27R/L"], cache.path());
    // The case in inches too, in brackets after the SI (issue #392).
    assert!(
        fetched
            .out
            .contains("\n  case             29 × 83 mm (1.14 × 3.3 in)\n"),
        "{}",
        fetched.out
    );
    assert!(
        fetched
            .err
            .contains("help: kept in the cache: `hpr sim --motor F27R/L` flies it, offline too"),
        "{}",
        fetched.err
    );
    let output = hpr(&["motors", "fetch", "F27R/L"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(text(&output.stderr).contains("`hpr motors fetch F27R/L`"));
}

/// Of an invented motor's files, the certification file that reads but whose masses make no motor
/// hpr flies (an exhaust speed under 200 m/s) is passed over for the manufacturer's, next in rank:
/// the file taken is the first hpr can fly, not only read.
#[test]
fn motors_fetch_takes_the_first_file_hpr_can_fly() {
    let cache = cache_with(&hpr::hpr_net::on_demand::Wanted::named("Z10-INVENTED"));
    let document = json_cached(
        &["motors", "fetch", "Z10-INVENTED"],
        0,
        "motors-fetch.schema.json",
        cache.path(),
    );
    assert_eq!(document["motor"]["file_id"], "f000000000000000000000a3");
    assert_eq!(document["motor"]["measured_by"], "mfr");
}

/// `--file` takes the file named ahead of the usual order: the user's RASP file, ranked last, and
/// a RockSim file once the cache holds the motor's RockSim answer. With a cache holding only the
/// RASP answer, the RockSim file asked for is given up for the usual order's, and the output says
/// so; with an empty cache, the refusal's command carries `--file`.
#[test]
fn motors_fetch_takes_the_file_named_first() {
    use hpr::hpr_net::on_demand::Wanted;
    let cache = cache_with(&Wanted::named("Z10-INVENTED"));
    let args = [
        "motors",
        "fetch",
        "--file",
        "f000000000000000000000a1",
        "Z10-INVENTED",
    ];
    let document = json_cached(&args, 0, "motors-fetch.schema.json", cache.path());
    assert_eq!(document["motor"]["file_id"], "f000000000000000000000a1");
    assert_eq!(document["motor"]["measured_by"], "user");
    let rocksim = "f000000000000000000000b1";
    let args = ["motors", "fetch", "--file", rocksim, "Z10-INVENTED"];
    let fetched = streams_cached(&args, cache.path());
    assert!(
        fetched.out.contains("f000000000000000000000a3, RASP"),
        "{}",
        fetched.out
    );
    assert!(
        fetched
            .err
            .contains(&format!("note: file {rocksim} was not taken")),
        "{}",
        fetched.err
    );
    let output = hpr(&args);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        text(&output.stderr).contains(&format!("`hpr motors fetch --file {rocksim} Z10-INVENTED`")),
        "{}",
        text(&output.stderr)
    );
    let cache = cache_with(&Wanted::named("Z10-INVENTED").preferring(rocksim));
    let document = json_cached(&args, 0, "motors-fetch.schema.json", cache.path());
    assert_eq!(document["motor"]["file_id"], rocksim);
    assert_eq!(document["motor"]["format"], "rse");
}

/// A `.ork` motor whose digest hpr's table of OpenRocket's curves holds asks ThrustCurve for that
/// file first (ADR-161). The invented motor's RockSim answer holds a stand-in under the table's
/// id: with that answer cached, the motor flies it and the note says it is OpenRocket's curve.
/// With only the RASP answer cached, the preference is given up and the note says the curve may
/// not be OpenRocket's; with nothing cached, the refusal's command carries `--file`.
#[test]
fn a_digest_the_table_holds_asks_for_openrockets_file() {
    use hpr::hpr_net::on_demand::Wanted;
    let folder = tempfile::tempdir().unwrap();
    let openrocket = "5f4294d20002e900000004e3";
    let z10 = "<manufacturer>Invented</manufacturer><designation>Z10-INVENTED</designation>\
               <digest>22aec01287ea1e3b8c6f66b26fe5fea6</digest>\
               <diameter>0.029</diameter><length>0.1</length>";
    let xml = std::fs::read_to_string(repo_file(PROBE))
        .unwrap()
        .replacen(PROBE_MOTOR, z10, 1);
    let path = folder.path().join("table.ork");
    std::fs::write(&path, xml).unwrap();
    let path = path.to_string_lossy().into_owned();
    let wanted = Wanted::by("Invented", "Z10-INVENTED");
    let notes = |cache: &Path| {
        json_cached(&["sim", &path], 0, "sim.schema.json", cache)["notes"].to_string()
    };
    let notes_of_openrockets = notes(cache_with(&wanted.clone().preferring(openrocket)).path());
    assert!(
        notes_of_openrockets.contains(&format!("file {openrocket}"))
            && notes_of_openrockets.contains("the file whose curve is the one OpenRocket flies"),
        "{notes_of_openrockets}"
    );
    let notes_of_rasp = notes(cache_with(&wanted).path());
    assert!(
        notes_of_rasp.contains("file f000000000000000000000a3")
            && notes_of_rasp.contains("it may not be the curve OpenRocket flies"),
        "{notes_of_rasp}"
    );
    let output = hpr(&["sim", &path]);
    assert_eq!(output.status.code(), Some(1));
    let refusal = text(&output.stderr);
    assert!(
        refusal.contains(&format!(
            "`hpr motors fetch --manufacturer Invented --file {openrocket} Z10-INVENTED`"
        )),
        "{refusal}"
    );
}

/// Two motors of different designations whose `.ork` records the same digest both fly the curve
/// fetched for the first, and both are named as flying it.
#[test]
fn a_digest_two_motors_share_flies_and_names_one_fetched_curve() {
    let folder = tempfile::tempdir().unwrap();
    let digest = "0123456789abcdef0123456789abcdef";
    let xml = std::fs::read_to_string(repo_file(PROBE)).unwrap();
    let f27 = format!(
        "<manufacturer>AeroTech</manufacturer><designation>F27R/L</designation>\
         <digest>{digest}</digest><diameter>0.029</diameter><length>0.083</length>"
    );
    let xml = two_mounts(xml.replacen(PROBE_MOTOR, &f27, 1), str::to_owned);
    let second = xml.rfind("<designation>F27R/L</designation>").unwrap();
    let xml = format!(
        "{}<designation>F27X</designation>{}",
        &xml[..second],
        &xml[second + "<designation>F27R/L</designation>".len()..]
    );
    let path = folder.path().join("shared.ork");
    std::fs::write(&path, xml).unwrap();
    let path = path.to_string_lossy().into_owned();
    let cache = cache_with(&hpr::hpr_net::on_demand::Wanted::by("AeroTech", "F27R/L"));
    let document = json_cached(&["sim", &path], 0, "sim.schema.json", cache.path());
    let motors = document["motors"].as_array().unwrap();
    let designations: Vec<&str> = motors
        .iter()
        .map(|m| m["designation"].as_str().unwrap())
        .collect();
    assert_eq!(designations, ["F27R/L", "F27X"]);
    for motor in motors {
        assert_eq!(motor["source"]["kind"], "thrust_curve", "{motor:#}");
        assert_eq!(motor["source"]["designation"], "F27R/L", "{motor:#}");
    }
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes.iter().any(|n| n.as_str().unwrap().starts_with(
            "`F27X` has no curve in the file or the bundled catalog, so it flies \
             ThrustCurve.org's AeroTech F27R/L"
        )),
        "{notes:?}"
    );
}

/// `--offline` alone, with no `HPR_OFFLINE`, keeps `hpr sim` off the network: with an empty cache
/// a name the catalog lacks is refused as not cached.
#[test]
fn sim_offline_flag_alone_reads_the_cache_only() {
    let cache = tempfile::tempdir().unwrap();
    let output = assert_cmd::cargo::cargo_bin_cmd!("hpr")
        .args(["sim", &repo_file(PROBE), "--motor", "F27R/L", "--offline"])
        .env("HPR_CACHE_DIR", cache.path())
        .env_remove("HPR_OFFLINE")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = text(&output.stderr);
    assert!(
        stderr.contains(
            "error: F27R/L is not in HPR Sim's cache of ThrustCurve.org, and the run is offline\n\
             help: with a network connection, `hpr motors fetch F27R/L` fetches it into the \
             cache\n"
        ),
        "{stderr}"
    );
}

/// A `.ork` with no motor configuration flies `--motor` in its only mount, in a configuration
/// named after the motor, as the library flies it there.
#[test]
fn sim_flies_a_motor_in_a_file_with_no_configuration() {
    let folder = tempfile::tempdir().unwrap();
    let xml = std::fs::read_to_string(repo_file(PROBE)).unwrap();
    let cut = |xml: &str, open: &str, close: &str| {
        let start = xml.find(open).unwrap();
        let end = xml[start..].find(close).unwrap() + start + close.len();
        format!("{}{}", &xml[..start], &xml[end..])
    };
    let bare = cut(&xml, "<motor configid", "</motor>");
    let bare = cut(&bare, "<motorconfiguration ", "</motorconfiguration>");
    let path = folder.path().join("bare.ork");
    std::fs::write(&path, bare).unwrap();
    let path = path.to_string_lossy().into_owned();
    let document = json_error(&["sim", &path], 1, "input");
    let message = said(&document);
    assert!(
        message.ends_with("no motor configuration\nhelp: give a motor with --motor"),
        "{message}"
    );

    let document = json(&["sim", &path, "--motor", "H54"], 0, "sim.schema.json");
    assert_eq!(document["design"]["configuration"], "168H54-10A");
    assert_eq!(document["design"]["configurations"], serde_json::json!([]));
    let (rocket, _, mount) = probe_design();
    let motor = hpr::Motor::from_catalog("H54").unwrap();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let (flight, _) = library_flight(
        rocket,
        "168H54-10A",
        &mount,
        Some(&motor),
        &environment,
        |b| b,
    );
    same_flight(&document, &flight);
}

/// What `--motor` refuses on top of the rest: a configuration that switches a stage off, a hybrid
/// motor, and a pod set's motors are counted, one per pod.
#[test]
fn sim_motor_refusals_and_counts() {
    let folder = tempfile::tempdir().unwrap();
    let off = probe_with(
        folder.path(),
        "off.ork",
        r#"<stage number="0" active="true"/>"#,
        r#"<stage number="0" active="false"/>"#,
    );
    let document = json_error(&["sim", &off, "--motor", "H54"], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(message.contains("switches a stage off"), "{message}");
    let document = json_error(&["sim", &off], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(message.contains("nor can it with --motor"), "{message}");

    let rse = std::fs::read_to_string(curve_file("curves/5f923edb1bca5800041716ab.rse")).unwrap();
    let at = rse.find("Type=\"").unwrap() + "Type=\"".len();
    let end = rse[at..].find('"').unwrap() + at;
    let hybrid = folder.path().join("hybrid.rse");
    std::fs::write(&hybrid, format!("{}hybrid{}", &rse[..at], &rse[end..])).unwrap();
    let probe = repo_file(PROBE);
    let document = json_error(
        &["sim", &probe, "--motor", &hybrid.to_string_lossy()],
        1,
        "input",
    );
    let message = document["error"]["message"].as_str().unwrap();
    assert!(message.contains("hybrid"), "{message}");

    let pods = repo_file("validation/fixtures/ork/pod-flights/pods-motors-2.ork");
    let document = json(&["sim", &pods, "--motor", "H54"], 0, "sim.schema.json");
    assert_eq!(document["motors"][0]["count"], 2);
    assert_eq!(document["motors"][0]["unlit"], 0);
}

/// `sim::left_out` gives the reader's reason for a configuration `hpr sim` refuses as the file
/// has it, `None` for one it flies, and an error for a file or configuration it can't find: what
/// `cargo xtask ork-cli` counts refusals by (M4.5j). The guide's rocket flies motors of the
/// bundled catalog, so no cache is read.
#[test]
fn sim_left_out_names_the_readers_reason() {
    use hpr::hpr_io::ork::NotFlown;
    use hpr_cli::sim::left_out;
    let folder = tempfile::tempdir().unwrap();
    let design = repo_file("validation/fixtures/ork/guides/level-1.ork");
    let (first, second) = (
        "6a1d0000-0000-4000-8000-0000000000c1",
        "6a1d0000-0000-4000-8000-0000000000c2",
    );
    json(&["sim", &design, "--config", second], 0, "sim.schema.json");
    assert_eq!(left_out(&design, second), Ok(None));
    // The first configuration's stage switched off, and only that configuration's.
    let xml = std::fs::read_to_string(&design).unwrap();
    let on = r#"<stage number="0" active="true"/>"#;
    assert_eq!(xml.matches(on).count(), 2);
    let off = folder.path().join("off.ork");
    std::fs::write(
        &off,
        xml.replacen(on, r#"<stage number="0" active="false"/>"#, 1),
    )
    .unwrap();
    let off = off.to_string_lossy().into_owned();
    json_error(&["sim", &off, "--config", first], 1, "input");
    json(&["sim", &off, "--config", second], 0, "sim.schema.json");
    assert_eq!(left_out(&off, first), Ok(Some(NotFlown::InactiveStage)));
    assert_eq!(left_out(&off, second), Ok(None));
    assert!(left_out(&design, "no such configuration").is_err());
    assert!(left_out(&folder.path().join("missing.ork").to_string_lossy(), first).is_err());
}

/// The probe's motor element for its one configuration.
const PROBE_MOTOR_ELEMENT: (&str, &str) = (
    r#"<motor configid="00000000-0000-4000-8000-000000000097">"#,
    "</motor>",
);

/// The probe with `edit` applied to its text, written to `folder` as `name`; its path.
fn probe_edited(folder: &Path, name: &str, edit: impl Fn(String) -> String) -> String {
    let xml = std::fs::read_to_string(repo_file(PROBE)).unwrap();
    let path = folder.join(name);
    std::fs::write(&path, edit(xml)).unwrap();
    path.to_string_lossy().into_owned()
}

/// `text` without its one element from `open` to `close`.
fn without(text: &str, (open, close): (&str, &str)) -> String {
    assert_eq!(text.matches(open).count(), 1, "{open}");
    let at = text.find(open).unwrap();
    let end = text[at..].find(close).unwrap() + at + close.len();
    format!("{}{}", &text[..at], &text[end..])
}

/// The probe's motor-mount tube written twice, the copy with its own id, after `mount` edits it.
fn two_mounts(xml: String, mount: impl Fn(&str) -> String) -> String {
    let (open, close) = ("<innertube>", "</innertube>");
    let at = xml.find(open).unwrap();
    let end = xml[at..].find(close).unwrap() + at + close.len();
    let tube = mount(&xml[at..end]);
    let copy = tube.replacen(
        "<id>00000000-0000-4000-8000-000000000003</id>",
        "<id>00000000-0000-4000-8000-000000000006</id>",
        1,
    );
    assert_ne!(copy, tube);
    format!("{}{tube}{copy}{}", &xml[..at], &xml[end..])
}

/// Each refusal of a `.ork` that `--motor` can't fix, or that `--motor` needs told more, names
/// its reason; a refused configuration lists the file's configurations that fly as written.
#[test]
fn sim_refuses_a_motor_it_cant_place() {
    let folder = tempfile::tempdir().unwrap();
    let message = |args: &[&str]| -> String { said(&json_error(args, 1, "input")) };

    // More than one stage: nothing says when they would separate.
    let staged = repo_file("validation/fixtures/ork/loft-demo/demo-payload-separation.ork");
    let got = message(&["sim", &staged, "--motor", "H54"]);
    assert!(got.contains("the rocket has 2 stages"), "{got}");

    // No configuration at all, and two stages: `--motor` can't fly it either.
    let booster = "<stage><name>Booster</name><id>00000000-0000-4000-8000-000000000019</id>\
                   <subcomponents><bodytube><name>Booster tube</name>\
                   <id>00000000-0000-4000-8000-000000000012</id><length>0.3</length>\
                   <thickness>0.001</thickness><radius>0.03</radius>\
                   <material type=\"bulk\" density=\"1000.0\">Probe</material>\
                   <finish>normal</finish></bodytube></subcomponents></stage>";
    let bare = probe_edited(folder.path(), "bare-two-stage.ork", |xml| {
        let xml = without(&xml, ("<motorconfiguration ", "</motorconfiguration>"));
        let xml = without(&xml, PROBE_MOTOR_ELEMENT);
        assert_eq!(xml.matches("</stage>").count(), 1);
        xml.replacen("</stage>", &format!("</stage>{booster}"), 1)
    });
    let got = message(&["sim", &bare]);
    assert!(
        got.contains(
            "no motor configuration, nor can it fly with --motor: the rocket has 2 stages"
        ),
        "{got}"
    );

    // No motor in the configuration: `--motor` fixes that, unless the stage is switched off,
    // which the left-out reason (the missing motor) doesn't say.
    let empty = probe_edited(folder.path(), "empty.ork", |xml| {
        without(&xml, PROBE_MOTOR_ELEMENT)
    });
    let got = message(&["sim", &empty]);
    assert!(got.contains("give a motor with --motor"), "{got}");
    let off = probe_edited(folder.path(), "empty-off.ork", |xml| {
        without(&xml, PROBE_MOTOR_ELEMENT).replacen(
            r#"<stage number="0" active="true"/>"#,
            r#"<stage number="0" active="false"/>"#,
            1,
        )
    });
    let got = message(&["sim", &off]);
    assert!(got.contains("nor can it with --motor"), "{got}");
    assert!(got.contains("switches a stage off"), "{got}");
    let got = message(&["sim", &off, "--motor", "H54"]);
    assert!(got.contains("switches a stage off"), "{got}");

    // Two mounts and no motor: `--mount` says which.
    let open = probe_edited(folder.path(), "two-open.ork", |xml| {
        two_mounts(xml, |tube| without(tube, PROBE_MOTOR_ELEMENT))
    });
    let got = message(&["sim", &open, "--motor", "H54"]);
    assert!(
        got.contains("2 motor mounts\nhelp: say which with --mount: "),
        "{got}"
    );
    let got = message(&["sim", &open, "--motor", "H54", "--mount", "nowhere"]);
    assert!(got.contains("no motor mount `nowhere`"), "{got}");
    let second = "00000000-0000-4000-8000-000000000006";
    let document = json(
        &["sim", &open, "--motor", "H54", "--mount", second],
        0,
        "sim.schema.json",
    );
    assert_eq!(document["motors"][0]["mount"], second);

    // Two motors in the configuration: one `--motor` can't stand for both.
    let both = probe_edited(folder.path(), "two-motors.ork", |xml| {
        two_mounts(xml, str::to_owned)
    });
    let got = message(&["sim", &both, "--motor", "H54"]);
    assert!(got.contains("has 2 motors"), "{got}");

    // A second configuration that flies, named when the first is refused.
    let flying = "00000000-0000-4000-8000-000000000096";
    let pair = probe_edited(folder.path(), "pair.ork", |xml| {
        let xml = xml.replacen(
            "</motorconfiguration>",
            &format!(
                r#"</motorconfiguration><motorconfiguration configid="{flying}"><stage number="0" active="true"/></motorconfiguration>"#
            ),
            1,
        );
        xml.replacen(
            "</motormount>",
            &format!(
                r#"<motor configid="{flying}"><type>single</type><manufacturer>Cesaroni Technology</manufacturer><designation>168H54-10A</designation><diameter>0.029</diameter><length>0.187</length><delay>0.0</delay></motor></motormount>"#
            ),
            1,
        )
    });
    let got = message(&["sim", &pair]);
    assert!(
        got.contains(
            "\nhelp: --config flies one of the file's configurations that fly as written: 2 \
             [168H54-10A]"
        ),
        "{got}"
    );
    assert!(!got.contains(flying), "{got}");
    // `--config` takes its id, its number or its name.
    for config in [flying, "2", "168H54-10A"] {
        let flown = json(&["sim", &pair, "--config", config], 0, "sim.schema.json");
        assert_eq!(flown["design"]["configuration"], flying, "{config}");
        assert_eq!(flown["design"]["configuration_label"], "[168H54-10A]");
    }
}

/// A motor file's reading caveats and the design's warnings come out as warnings, each saying
/// where it came from.
#[test]
fn sim_passes_on_what_the_readers_and_checks_found() {
    let folder = tempfile::tempdir().unwrap();
    let eng = std::fs::read_to_string(curve_file("curves/5f4294d20002e90000000735.eng")).unwrap();
    let skipped = folder.path().join("skipped.eng");
    std::fs::write(
        &skipped,
        format!("A1 18 70 3 0.003 0.016 X\n 0.1 1\n 0.2 5 1 2 3 4 5\n 0.3 0\n;\n{eng}"),
    )
    .unwrap();
    let probe = repo_file(PROBE);
    let document = json(
        &["sim", &probe, "--motor", &skipped.to_string_lossy()],
        0,
        "sim.schema.json",
    );
    let warnings = document["warnings"].as_array().unwrap();
    assert!(
        warnings
            .iter()
            .any(|w| w["at"] == "skipped.eng, line 3" && w["kind"] == "skipped"),
        "{warnings:?}"
    );

    let stepped = probe_with(
        folder.path(),
        "stepped.ork",
        "<aftradius>0.03</aftradius>",
        "<aftradius>0.029</aftradius>",
    );
    let document = json(&["sim", &stepped, "--motor", "H54"], 0, "sim.schema.json");
    let warnings = document["warnings"].as_array().unwrap();
    assert!(
        warnings.iter().any(|w| w["at"] == "design checks"),
        "{warnings:?}"
    );
}

/// Whether `text` holds an id in OpenRocket's form, a UUID: 8-4-4-4-12 hex digits.
fn holds_uuid(text: &str) -> bool {
    let bytes = text.as_bytes();
    let groups = [8, 4, 4, 4, 12];
    (0..bytes.len()).any(|start| {
        let mut at = start;
        groups.iter().enumerate().all(|(index, &len)| {
            let hex = bytes
                .get(at..at + len)
                .is_some_and(|run| run.iter().all(u8::is_ascii_hexdigit));
            at += len;
            let dash = index == groups.len() - 1 || bytes.get(at) == Some(&b'-');
            at += 1;
            hex && dash
        })
    })
}

/// The text form reads by name (M4.5d): the configuration by its motors, the mount and the
/// design checks' parts by their names, never a UUID, and the checks in sentences, not JSON; the
/// summary leads with margin, apogee, rail exit speed, delay and descent. `--json` keeps the ids.
#[test]
fn sim_text_reads_by_name() {
    assert!(holds_uuid("x 00000000-0000-4000-8000-000000000098 y"));
    assert!(!holds_uuid("00000000-0000-4000-8000-00000000009"));
    let folder = tempfile::tempdir().unwrap();
    // A radius step for the checks to find, between parts the file names by UUID.
    let stepped = probe_with(
        folder.path(),
        "stepped.ork",
        "<aftradius>0.03</aftradius>",
        "<aftradius>0.029</aftradius>",
    );
    let run = streams(&["sim", &stepped, "--motor", "H54"]);
    let all = run.both();
    assert!(!holds_uuid(&all), "{all}");
    assert!(!all.contains("{\""), "{all}");
    let warnings = &run.err;
    assert!(
        warnings.contains("warning: design checks: `")
            && warnings.contains("a step in the body's outline"),
        "{warnings}"
    );
    let text = run.out;
    assert!(
        text.contains("\nconfiguration 1 of 1: [H128W-0] with --motor H54\n"),
        "{text}"
    );
    assert!(text.contains(" in `Motor mount`, lit at launch"), "{text}");
    let lead: Vec<&str> = text
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .take(6)
        .map(|line| line.split("  ").next().unwrap())
        .collect();
    assert_eq!(
        lead,
        [
            "static margin",
            "CG and CP",
            "apogee",
            "rail exit speed",
            "delay",
            "descent"
        ],
        "{text}"
    );
    assert!(
        text.contains(
            "descent               no recovery device opened, so the fall is not a prediction"
        ),
        "{text}"
    );
    let document = json(&["sim", &stepped, "--motor", "H54"], 0, "sim.schema.json");
    assert!(holds_uuid(&document["design"]["configuration"].to_string()));
    assert!(
        document["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|w| !holds_uuid(&w["message"].to_string())),
        "{document:#}"
    );

    // The file's own motor, a catalog one, with a 10 s delay: the delay against the coast.
    let delayed = probe_with(
        folder.path(),
        "delayed.ork",
        "<manufacturer>AeroTech</manufacturer><designation>H128W</designation>\
         <digest>501239de7374691072270406476cb243</digest><diameter>0.029</diameter>\
         <length>0.194</length><delay>0.0</delay>",
        "<manufacturer>Cesaroni Technology</manufacturer><designation>168H54-10A</designation>\
         <diameter>0.029</diameter><length>0.187</length><delay>10</delay>",
    );
    let flown = streams(&["sim", &delayed]).out;
    assert!(
        flown.contains("\nconfiguration 1 of 1: [168H54-10A]\n"),
        "{flown}"
    );
    // The coast is the flight's own: apogee less burnout, from its events.
    let document = json(&["sim", &delayed], 0, "sim.schema.json");
    let at = |kind: &str| {
        document["events"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["kind"] == kind)
            .unwrap_or_else(|| panic!("{kind}: {document:#}"))["time_s"]
            .as_f64()
            .unwrap()
    };
    let coast_s = at("apogee") - at("burnout");
    assert!(coast_s > 3.0 && coast_s < 10.0, "{coast_s}");
    // Set 10 s, the charge fires after apogee; set 3 s, before. The sign is the check.
    let late = format!(
        "\ndelay                 apogee {coast_s:.2} s after burnout; the motor's set delay: 10 s; \
         its charge fires {:.2} s after apogee\n",
        10.0 - coast_s
    );
    assert!(flown.contains(&late), "{late}: {flown}");
    let early = probe_with(
        folder.path(),
        "early.ork",
        "<manufacturer>AeroTech</manufacturer><designation>H128W</designation>\
         <digest>501239de7374691072270406476cb243</digest><diameter>0.029</diameter>\
         <length>0.194</length><delay>0.0</delay>",
        "<manufacturer>Cesaroni Technology</manufacturer><designation>168H54-10A</designation>\
         <diameter>0.029</diameter><length>0.187</length><delay>3</delay>",
    );
    let flown = streams(&["sim", &early]).out;
    let early = format!(
        "; the motor's set delay: 3 s; its charge fires {:.2} s before apogee\n",
        coast_s - 3.0
    );
    assert!(flown.contains(&early), "{early}: {flown}");

    // The descent under each set of parachutes, as the next opens and at landing: the
    // vertical speed down, from the flight's own events, the landing's the summary's.
    let dual = repo_file("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork");
    let flown = streams(&["sim", &dual, "--motor", "H54"]).out;
    let document = json(&["sim", &dual, "--motor", "H54"], 0, "sim.schema.json");
    let events = document["events"].as_array().unwrap();
    let main_opens = events
        .iter()
        .filter(|e| e["kind"] == "deployment")
        .nth(1)
        .unwrap_or_else(|| panic!("{document:#}"));
    let ground = events.iter().find(|e| e["kind"] == "ground_hit").unwrap();
    let down = |event: &Value| -event["vertical_velocity_m_s"].as_f64().unwrap();
    let landing = document["summary"]["landing"]["descent_rate_m_s"]
        .as_f64()
        .unwrap();
    assert!(down(main_opens) > 5.0 && down(ground) > 2.0, "{document:#}");
    assert!((down(ground) - landing).abs() < 1e-9, "{document:#}");
    let line = format!(
        "\ndescent               {:.1} m/s ({:.0} ft/s) at 150.0 m (492 ft) under `Drogue \
         parachute`; {landing:.1} m/s ({:.0} ft/s) at landing with `Main parachute` open too\n",
        down(main_opens),
        down(main_opens) / FOOT_M,
        landing / FOOT_M
    );
    assert!(flown.contains(&line), "{line}: {flown}");

    // The drogue set never to open: the main alone, late, carries the landing's caveat.
    let bytes = std::fs::read(&dual).unwrap();
    let xml = hpr::hpr_io::ork::container::unpack(&bytes)
        .unwrap()
        .value
        .design;
    assert_eq!(xml.matches("<deployevent>apogee</deployevent>").count(), 1);
    let late = folder.path().join("late.ork");
    std::fs::write(
        &late,
        xml.replace(
            "<deployevent>apogee</deployevent>",
            "<deployevent>never</deployevent>",
        ),
    )
    .unwrap();
    let flown = streams(&["sim", &late.to_string_lossy(), "--motor", "H54"]).out;
    let descent = flown
        .lines()
        .find(|line| line.starts_with("descent "))
        .unwrap_or_else(|| panic!("{flown}"));
    assert!(
        descent.contains(
            " ft/s) at landing under `Main parachute`: with no recovery device opened \
             soon after apogee, not a prediction"
        ),
        "{flown}"
    );
}

/// A peak the fall from apogee sets is marked as no prediction, in the text and the JSON: the
/// dual-deploy demo with both its parachutes set never to open is fastest as it reaches the
/// ground.
#[test]
fn sim_marks_what_the_fall_sets() {
    let bytes = std::fs::read(repo_file(
        "validation/fixtures/ork/loft-demo/demo-dual-deploy.ork",
    ))
    .unwrap();
    let xml = hpr::hpr_io::ork::container::unpack(&bytes)
        .unwrap()
        .value
        .design;
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("unopened.ork");
    let never = xml
        .replace(
            "<deployevent>apogee</deployevent>",
            "<deployevent>never</deployevent>",
        )
        .replace(
            "<deployevent>altitude</deployevent>",
            "<deployevent>never</deployevent>",
        );
    assert_eq!(never.matches("<deployevent>never</deployevent>").count(), 2);
    std::fs::write(&path, never).unwrap();
    let design = path.to_string_lossy().into_owned();
    let document = json(&["sim", &design, "--motor", "H54"], 0, "sim.schema.json");
    assert!(document["recovery"].as_array().unwrap().is_empty());
    let notes = document["notes"].as_array().unwrap();
    for name in ["Main parachute", "Drogue parachute"] {
        let never = format!("`{name}` never opens: the file sets it to never");
        assert!(notes.iter().any(|n| n == never.as_str()), "{notes:?}");
    }
    assert!(
        notes.iter().any(|n| n.as_str().unwrap().starts_with(
            "no recovery device of the file opens in this configuration, so the rocket falls"
        )),
        "{notes:?}"
    );
    let summary = &document["summary"];
    let apogee_s = summary["apogee"]["time_s"].as_f64().unwrap();
    assert!(summary["max_speed_m_s"]["time_s"].as_f64().unwrap() > apogee_s);
    assert_eq!(summary["max_speed_m_s"]["after_apogee"], true);
    assert_eq!(summary["rail_exit_speed_m_s"]["after_apogee"], false);
    let shown = streams(&["sim", &design, "--motor", "H54"]).out;
    let line = |start: &str| {
        shown
            .lines()
            .find(|line| line.starts_with(start))
            .unwrap_or_else(|| panic!("{shown}"))
            .to_owned()
    };
    assert!(line("top speed").ends_with("in the fall: not a prediction"));
    assert!(line("top Mach").ends_with("in the fall: not a prediction"));
    assert!(!line("rail exit speed").contains("prediction"));
    assert!(line("landing").ends_with("not a prediction"));

    // The probe is fastest at burnout, before apogee: nothing to mark.
    let probe = streams(&["sim", &repo_file(PROBE), "--motor", "H54"]).out;
    let top = probe.lines().find(|l| l.starts_with("top speed")).unwrap();
    assert!(!top.contains("prediction"), "{top}");
}

/// A device that opens long after apogee leaves the fall before it unpredicted: the dual-deploy
/// demo with its drogue set never to open falls from apogee to its main's 150 m on aerodynamics
/// that hold only at small angles of attack, so the landing keeps its caveat, a note says how long
/// the fall was, and the map pins no landing.
#[test]
fn sim_keeps_the_caveat_until_a_late_device_opens() {
    let bytes = std::fs::read(repo_file(
        "validation/fixtures/ork/loft-demo/demo-dual-deploy.ork",
    ))
    .unwrap();
    let xml = hpr::hpr_io::ork::container::unpack(&bytes)
        .unwrap()
        .value
        .design;
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("main-only.ork");
    let main_only = xml.replace(
        "<deployevent>apogee</deployevent>",
        "<deployevent>never</deployevent>",
    );
    assert_eq!(
        main_only
            .matches("<deployevent>never</deployevent>")
            .count(),
        1
    );
    std::fs::write(&path, main_only).unwrap();
    let design = path.to_string_lossy().into_owned();
    let geojson = folder.path().join("flight.geojson");
    let geojson_arg = geojson.to_string_lossy().into_owned();
    let document = json(
        &["sim", &design, "--motor", "H54", "--export", &geojson_arg],
        0,
        "sim.schema.json",
    );
    let recovery = document["recovery"].as_array().unwrap();
    assert_eq!(recovery.len(), 1);
    assert_eq!(recovery[0]["name"], "Main parachute");
    let apogee_s = document["summary"]["apogee"]["time_s"].as_f64().unwrap();
    let opened_s = recovery[0]["opened_s"].as_f64().unwrap();
    assert!(opened_s > apogee_s + 1.0, "{opened_s} {apogee_s}");
    let notes = document["notes"].as_array().unwrap();
    let fell = format!(
        "the rocket fell {:.1} s from apogee before `Main parachute` opened",
        opened_s - apogee_s
    );
    assert!(
        notes.iter().any(|n| n.as_str().unwrap().starts_with(&fell)),
        "{notes:?}"
    );
    // `hpr mc` says the same of its nominal flight (M4.6a, found in review).
    let mc = json(
        &["mc", &design, "--motor", "H54", "--runs", "2"],
        0,
        "mc.schema.json",
    );
    let fell_note = notes
        .iter()
        .find(|n| n.as_str().unwrap().starts_with(&fell))
        .unwrap()
        .as_str()
        .unwrap();
    let mc_notes = mc["notes"].as_array().unwrap();
    assert!(
        mc_notes
            .iter()
            .any(|n| n.as_str().unwrap() == format!("the nominal flight: {fell_note}")),
        "{mc_notes:?}"
    );
    // The map draws the track alone, with no landing pinned.
    let map: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&geojson).unwrap()).unwrap();
    let kinds: Vec<&str> = map["features"]
        .as_array()
        .unwrap()
        .iter()
        .map(|feature| feature["geometry"]["type"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["LineString"]);
    let shown = streams(&["sim", &design, "--motor", "H54"]).out;
    let line = |start: &str| {
        shown
            .lines()
            .find(|line| line.starts_with(start))
            .unwrap_or_else(|| panic!("{shown}"))
            .to_owned()
    };
    assert!(
        line("landing")
            .ends_with("with no recovery device opened soon after apogee, not a prediction"),
        "{shown}"
    );
    // A peak in the fall before the main opens keeps its mark; the climb's does not.
    let speed = &document["summary"]["max_speed_m_s"];
    let in_fall = speed["after_apogee"] == true && speed["time_s"].as_f64().unwrap() < opened_s;
    assert_eq!(
        line("top speed").ends_with("in the fall: not a prediction"),
        in_fall,
        "{shown}"
    );
}

/// `--export` never writes over a file the run reads, nor the same file twice, and a missing
/// folder or a bad interval is refused before the flight.
#[test]
fn sim_exports_are_checked_before_the_flight() {
    let folder = tempfile::tempdir().unwrap();
    let design = folder.path().join("design.json");
    std::fs::copy(
        repo_file("validation/designs/synthetic-54mm-three-fin.json"),
        &design,
    )
    .unwrap();
    let before = std::fs::read(&design).unwrap();
    let design = design.to_string_lossy().into_owned();
    let csv = folder.path().join("f.csv").to_string_lossy().into_owned();
    let lost = folder
        .path()
        .join("no/f.csv")
        .to_string_lossy()
        .into_owned();
    let lost_svg = folder
        .path()
        .join("no/f.svg")
        .to_string_lossy()
        .into_owned();
    for (args, reason) in [
        (
            vec!["--export", design.as_str()],
            "this run reads that file",
        ),
        (
            vec!["--export", &csv, "--export", &csv],
            "names a file twice",
        ),
        (vec!["--export", &lost], "there is no folder"),
        (vec!["--plot", &csv], "--plot writes an .svg file"),
        (vec!["--plot", &lost_svg], "there is no folder"),
        (vec!["--interval", "0"], "--interval"),
        (vec!["--interval", "0.0005"], "at least 0.001 s"),
    ] {
        let mut line = vec!["sim", design.as_str()];
        line.extend(args);
        let document = json_error(&line, 1, "input");
        let message = document["error"]["message"].as_str().unwrap();
        assert!(message.contains(reason), "{line:?}: {message}");
    }
    assert_eq!(std::fs::read(&design).unwrap(), before);
    assert!(!Path::new(&csv).exists());
}

/// `--plot` draws the flight: three panels, a numbered marker for each instant with events, each
/// event a row of the table with its time; a `<desc>` giving the summary's apogee, its time and
/// top speed; a fall with no recovery device open is hatched "not a prediction", and a braked one
/// isn't. The JSON names the file and the text says it was written.
#[test]
fn sim_plots_the_flight() {
    let folder = tempfile::tempdir().unwrap();
    let svg = folder.path().join("f.svg").to_string_lossy().into_owned();
    let design = repo_file("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork");
    let args = [
        "sim",
        design.as_str(),
        "--motor",
        "H54",
        "--plot",
        svg.as_str(),
    ];
    let document = json(&args, 0, "sim.schema.json");
    assert_eq!(document["plot"], svg.as_str());
    let figure = std::fs::read_to_string(&svg).unwrap();
    let parsed = roxmltree::Document::parse(&figure).unwrap();
    let ids: Vec<&str> = parsed
        .descendants()
        .filter_map(|node| node.attribute("id"))
        .collect();
    for id in ["altitude", "speed", "acceleration"] {
        assert!(ids.contains(&id), "{id}: {ids:?}");
    }
    // One marker for each instant with events.
    let mut instants: Vec<f64> = document["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|event| event["time_s"].as_f64().unwrap())
        .collect();
    instants.dedup();
    let markers = ids.iter().filter(|id| id.starts_with("event-")).count();
    assert_eq!(markers, instants.len(), "{ids:?}");
    assert_eq!(markers, 6);
    // The table's rows, one per event, each a line of five cells: number, time, altitude in
    // meters and in feet, name.
    let table = parsed
        .descendants()
        .find(|node| node.attribute("id") == Some("events"))
        .unwrap();
    let mut rows: Vec<(String, Vec<&str>)> = Vec::new();
    for cell in table.children().filter(|node| node.has_tag_name("text")) {
        let y = cell.attribute("y").unwrap().to_owned();
        let text = cell.text().unwrap_or_default();
        match rows.last_mut() {
            Some((last, cells)) if *last == y => cells.push(text),
            _ => rows.push((y, vec![text])),
        }
    }
    let rows: Vec<Vec<&str>> = rows
        .into_iter()
        .map(|(_, cells)| cells)
        .filter(|cells| cells.len() == 5 && cells[0] != "NO.")
        .collect();
    assert_eq!(rows.len(), document["events"].as_array().unwrap().len());
    for named in [
        "apogee",
        "charge for `Drogue parachute`",
        "`Drogue parachute` opens",
    ] {
        assert!(
            rows.contains(&vec!["4", "9.12", "324.0", "1,063", named]),
            "{named}: {rows:?}"
        );
    }
    // The summary, as the JSON's figures give it.
    let summary = &document["summary"];
    let desc = parsed
        .descendants()
        .find(|node| node.has_tag_name("desc"))
        .and_then(|node| node.text())
        .unwrap();
    // Whole feet, grouped in threes from four digits as the figure's numbers are.
    let grouped = |value: f64| {
        let digits = format!("{value:.0}");
        match digits.len() {
            4..=6 => format!(
                "{},{}",
                &digits[..digits.len() - 3],
                &digits[digits.len() - 3..]
            ),
            _ => digits,
        }
    };
    let apogee_m = summary["apogee"]["height_above_ground_m"].as_f64().unwrap();
    let top_m_s = summary["max_speed_m_s"]["value"].as_f64().unwrap();
    assert_eq!(
        desc,
        format!(
            "Apogee {apogee_m:.1} m ({} ft) above the launch site at {:.2} s; top speed \
             {top_m_s:.1} m/s ({} ft/s) at {:.2} s.",
            grouped(apogee_m / FOOT_M),
            summary["apogee"]["time_s"].as_f64().unwrap(),
            grouped(top_m_s / FOOT_M),
            summary["max_speed_m_s"]["time_s"].as_f64().unwrap(),
        )
    );
    assert!(!figure.contains("not a prediction"));
    let output = hpr(&args);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!(
        "wrote {svg} (altitude, speed and acceleration against time)"
    )));

    // With --export too, both files are written, and the recording keeps the rows it keeps
    // alone: the figure's observer adds none to it.
    let csv = folder.path().join("f.csv").to_string_lossy().into_owned();
    let alone = json(
        &[
            "sim",
            design.as_str(),
            "--motor",
            "H54",
            "--export",
            csv.as_str(),
        ],
        0,
        "sim.schema.json",
    );
    std::fs::remove_file(&svg).unwrap();
    let both = json(
        &[
            "sim",
            design.as_str(),
            "--motor",
            "H54",
            "--export",
            csv.as_str(),
            "--plot",
            svg.as_str(),
        ],
        0,
        "sim.schema.json",
    );
    assert_eq!(both["exports"][0]["rows"], alone["exports"][0]["rows"]);
    assert_eq!(both["summary"], alone["summary"]);
    assert_eq!(std::fs::read_to_string(&svg).unwrap(), figure);

    // With no recovery device, the fall from apogee is shaded.
    let probe = repo_file(PROBE);
    json(
        &[
            "sim",
            probe.as_str(),
            "--motor",
            "H54",
            "--plot",
            svg.as_str(),
        ],
        0,
        "sim.schema.json",
    );
    let figure = std::fs::read_to_string(&svg).unwrap();
    assert_eq!(figure.matches(">not a prediction<").count(), 1, "{figure}");
    assert!(figure.contains("Hatched, 11.21 s to 29.71 s:"), "{figure}");
}

/// The guide's launch section says the example rocket climbs about 8% higher from a 1,400 m site
/// than from sea level, and that 45° N moves its apogee by less than 0.2%.
#[test]
fn the_guides_launch_figures_hold() {
    let probe = repo_file(PROBE);
    let apogee = |extra: &[&str]| {
        let mut args = vec!["sim", probe.as_str(), "--motor", "H54"];
        args.extend(extra);
        json(&args, 0, "sim.schema.json")["summary"]["apogee"]["height_above_ground_m"]
            .as_f64()
            .unwrap()
    };
    let sea = apogee(&[]);
    let high = apogee(&["--elevation", "1400"]) / sea - 1.0;
    assert!((0.075..0.085).contains(&high), "{high}");
    let north = apogee(&["--latitude", "45"]) / sea - 1.0;
    assert!(north.abs() < 0.002, "{north}");

    // The second example, from Spaceport America on a leaning rail in a wind, climbs about 4%
    // higher, the wind costing more than the lean.
    let site = [
        "--latitude",
        "32.99",
        "--longitude",
        "-106.97",
        "--elevation",
        "1400",
        "--rail-length",
        "3",
    ];
    let lean = ["--inclination", "85", "--heading", "270"];
    let wind = ["--wind", "5", "--wind-from", "270"];
    let spaceport = apogee(&[&site[..], &lean, &wind].concat()) / sea - 1.0;
    assert!((0.035..0.045).contains(&spaceport), "{spaceport}");
    let calm = apogee(&[&site[..], &lean].concat());
    let upright = apogee(&[&site[..], &wind].concat());
    let vertical = apogee(&site);
    assert!(
        vertical - upright > vertical - calm,
        "{vertical} {upright} {calm}"
    );
}

/// A bundled `.rse` curve and a `.eng` one.
const RSE_CURVE: &str = "curves/5f923edb1bca5800041716ab.rse";
/// A bundled `.rse` curve whose maker is one word, which a `.eng` header keeps as it is.
const RSE_ONE_WORD_MAKER: &str = "curves/5f4294d20002e90000000719.rse";
const ENG_CURVE: &str = "curves/5f4294d20002e90000000724.eng";

/// `hpr convert` round-trips a `.eng` file through `.rse`, and a `.rse` file through `.eng`: the
/// motors come back as they were, and the second file of each trip is the first's, byte for byte.
#[test]
fn convert_round_trips_eng_and_rse() {
    use hpr::hpr_motor::{eng, rse};
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let read = |name: &str| std::fs::read_to_string(dir.path().join(name)).unwrap();

    // .eng → .rse → .eng: every value of every entry, bit for bit.
    let original = eng::parse(&std::fs::read_to_string(curve_file(ENG_CURVE)).unwrap())
        .unwrap()
        .value;
    let there = json(
        &["convert", &curve_file(ENG_CURVE), &path("a.rse")],
        0,
        "convert.schema.json",
    );
    assert_eq!(there["output"]["format"], "rse");
    assert_eq!(there["motors"], serde_json::json!(["131-G84-GR-10A"]));
    assert_eq!(there["warnings"], Value::Array(Vec::new()));
    let back = json(
        &["convert", &path("a.rse"), &path("b.eng")],
        0,
        "convert.schema.json",
    );
    assert_eq!(eng::parse(&read("b.eng")).unwrap().value, original);
    // What `.eng` has no place for is said, and only that.
    let [warning] = &back["warnings"].as_array().unwrap()[..] else {
        panic!("{back:#}");
    };
    assert_eq!(warning["kind"], "dropped");
    assert!(
        warning["message"]
            .as_str()
            .unwrap()
            .starts_with("dropped Type, auto-calc-mass, auto-calc-cg, avgThrust, peakThrust, Itot, burn-time, massFrac, Isp, m, cg:"),
        "{warning:#}"
    );
    // Converted again, the file is the same, byte for byte.
    streams(&["convert", &path("b.eng"), &path("c.rse")]);
    assert_eq!(read("c.rse"), read("a.rse"));

    // .rse → .eng → .rse: the code, maker, casing, masses, delays and points.
    let original = rse::parse(&std::fs::read_to_string(curve_file(RSE_ONE_WORD_MAKER)).unwrap())
        .unwrap()
        .value;
    streams(&["convert", &curve_file(RSE_ONE_WORD_MAKER), &path("d.eng")]);
    streams(&["convert", &path("d.eng"), &path("e.rse")]);
    let back = rse::parse(&read("e.rse")).unwrap().value;
    assert_eq!(back.engines.len(), original.engines.len());
    for (a, b) in original.engines.iter().zip(&back.engines) {
        assert_eq!(
            (&a.code, &a.manufacturer, &a.delays),
            (&b.code, &b.manufacturer, &b.delays)
        );
        for (x, y) in [
            (a.diameter_mm, b.diameter_mm),
            (a.length_mm, b.length_mm),
            (a.initial_mass_g, b.initial_mass_g),
            (a.propellant_mass_g, b.propellant_mass_g),
        ] {
            assert_eq!(x.to_bits(), y.to_bits());
        }
        let points = |e: &hpr::hpr_motor::rse::RseEngine| -> Vec<(u64, u64)> {
            e.points
                .iter()
                .map(|p| (p.time_s.to_bits(), p.thrust_n.to_bits()))
                .collect()
        };
        assert_eq!(points(a), points(b));
    }
    streams(&["convert", &path("e.rse"), &path("f.eng")]);
    assert_eq!(read("f.eng"), read("d.eng"));
}

/// No bundled motor's size or masses disagree with its own curve file's header (#295), so writing
/// any of them as either format warns of none: a header's kilograms that differ from the
/// catalog's grams only by the conversion's last bits are the same figure.
#[test]
fn convert_warns_of_no_disagreement_for_any_bundled_motor() {
    let dir = tempfile::tempdir().unwrap();
    let catalog = hpr::hpr_motor::catalog::Catalog::bundled().unwrap();
    assert_eq!(catalog.motors.len(), 32);
    for motor in &catalog.motors {
        for extension in ["eng", "rse"] {
            let out = dir.path().join(format!("m.{extension}"));
            let document = json(
                &["convert", &motor.designation, &out.to_string_lossy()],
                0,
                "convert.schema.json",
            );
            let disagreements: Vec<_> = document["warnings"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|warning| warning.to_string().contains("its curve file gives"))
                .collect();
            assert!(
                disagreements.is_empty(),
                "{} as .{extension}: {disagreements:?}",
                motor.designation
            );
        }
    }
}

/// A catalog motor is written from its bundled curve, and the same format in and out rewrites the
/// file in hpr's layout.
#[test]
fn convert_writes_a_catalog_motor_and_rewrites_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let document = json(
        &["convert", "H170M", &path("h.eng")],
        0,
        "convert.schema.json",
    );
    assert_eq!(document["input"]["kind"], "catalog");
    assert_eq!(document["input"]["format"], "rse");
    assert_eq!(document["motors"], serde_json::json!(["H170M"]));
    let flown =
        hpr::Motor::from_eng(&std::fs::read_to_string(dir.path().join("h.eng")).unwrap()).unwrap();
    let catalog = hpr::Motor::from_catalog("H170M").unwrap();
    // The curve the catalog flies, point for point, and its size and masses, which the catalog
    // gives in grams and the file in kilograms, to the last bit or so.
    assert_eq!(flown.solid_motor().curve(), catalog.solid_motor().curve());
    assert_eq!(
        (flown.diameter_m(), flown.length_m()),
        (catalog.diameter_m(), catalog.length_m())
    );
    let close = |a: f64, b: f64| (a - b).abs() <= 4.0 * f64::EPSILON * b.abs();
    let (a, b) = (flown.solid_motor(), catalog.solid_motor());
    assert!(close(
        a.propellant_initial_mass_kg(),
        b.propellant_initial_mass_kg()
    ));
    assert!(close(a.dry().mass_kg, b.dry().mass_kg));
    // Since issue #295 the bundled catalog's size and masses are its curve files' headers', so no
    // catalog motor's header disagrees with it any more: the 26E31-15A, whose header's 0.0169 kg
    // of propellant ThrustCurve.org's record gave as 0.0111 kg, is written as its file gives it,
    // with no warning. (`convert.rs`'s unit tests hold what is written when they differ.)
    let document = json(
        &["convert", "26E31-15A", &path("e.eng")],
        0,
        "convert.schema.json",
    );
    assert_eq!(document["warnings"], serde_json::json!([]));
    let written =
        hpr::Motor::from_eng(&std::fs::read_to_string(dir.path().join("e.eng")).unwrap()).unwrap();
    let catalog = hpr::Motor::from_catalog("26E31-15A").unwrap();
    assert!(close(
        written.solid_motor().propellant_initial_mass_kg(),
        catalog.solid_motor().propellant_initial_mass_kg()
    ));
    assert!(close(
        written.solid_motor().propellant_initial_mass_kg(),
        0.0169
    ));
    // A header that is already the mass hpr flies (`g × 1e-3`) keeps its digits, with no warning.
    let document = json(
        &["convert", "411I175-14A", &path("i.eng")],
        0,
        "convert.schema.json",
    );
    assert_eq!(document["warnings"], serde_json::json!([]));
    let header = std::fs::read_to_string(dir.path().join("i.eng")).unwrap();
    assert!(
        header
            .lines()
            .any(|line| line.split_whitespace().nth(4) == Some("0.22890000000000002")),
        "{header}"
    );
    assert_eq!(0.228_900_000_000_000_02, 228.9 * 1e-3);
    // A `.rse` catalog motor keeps its file's masses and the figures worked out from them (#295:
    // the D5's loaded mass was 44.1 g in ThrustCurve.org's record and is its file's 45.1 g now).
    let document = json(&["convert", "D5", &path("d.rse")], 0, "convert.schema.json");
    assert_eq!(document["warnings"], serde_json::json!([]));
    let written =
        hpr::hpr_motor::rse::parse(&std::fs::read_to_string(dir.path().join("d.rse")).unwrap())
            .unwrap()
            .value;
    let engine = &written.engines[0];
    assert_eq!(engine.initial_mass_g, 45.1);
    assert_eq!(engine.mass_fraction_pct, Some(53.22));
    let text = text_ok(&["convert", &curve_file(ENG_CURVE), &path("same.eng")]);
    assert_eq!(
        text,
        "read   5f4294d20002e90000000724.eng\nwrote  same.eng: 131-G84-GR-10A\n"
    );
}

/// What `hpr convert` changes and what it leaves: `--delays` fills only the engines without
/// delays, a hybrid is refused as a hybrid, and a `.eng` maker of several words is joined.
#[test]
fn convert_fills_only_what_a_file_lacks() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let engine = |code: &str, extra: &str| {
        format!(
            "<engine mfg=\"M\" code=\"{code}\" dia=\"29\" len=\"100\" initWt=\"80\" \
             propWt=\"40\"{extra}><data><eng-data t=\"0\" f=\"0\"/><eng-data t=\"0.5\" \
             f=\"20\"/><eng-data t=\"1\" f=\"0\"/></data></engine>"
        )
    };
    let file = |engines: &[String]| {
        format!(
            "<engine-database><engine-list>{}</engine-list></engine-database>\n",
            engines.concat()
        )
    };
    std::fs::write(
        dir.path().join("two.rse"),
        file(&[engine("X1", " delays=\"4,6\""), engine("X1", "")]),
    )
    .unwrap();
    text_ok(&[
        "convert",
        &path("two.rse"),
        &path("two.eng"),
        "--delays",
        "P",
    ]);
    let delays: Vec<String> = std::fs::read_to_string(dir.path().join("two.eng"))
        .unwrap()
        .lines()
        .filter(|line| line.starts_with("X1 "))
        .map(|line| line.split_whitespace().nth(3).unwrap().to_owned())
        .collect();
    assert_eq!(delays, ["4-6", "P"]);
    std::fs::write(
        dir.path().join("hybrid.rse"),
        file(&[engine("X2", " Type=\"hybrid\"")]),
    )
    .unwrap();
    let document = json_error(
        &["convert", &path("hybrid.rse"), &path("hybrid.eng")],
        1,
        "input",
    );
    let message = document["error"]["message"].as_str().unwrap();
    assert!(message.contains("X2 is a hybrid"), "{message}");
    std::fs::write(
        dir.path().join("maker.eng"),
        "X3 29 100 P 0.04 0.08 Some Maker\n 0.5 20\n 1 0\n",
    )
    .unwrap();
    let document = json(
        &["convert", &path("maker.eng"), &path("joined.eng")],
        0,
        "convert.schema.json",
    );
    assert_eq!(
        document["warnings"][1]["message"],
        "a .eng maker is one word, so \"Some Maker\" is written \"Some_Maker\""
    );
    let header = std::fs::read_to_string(dir.path().join("joined.eng")).unwrap();
    assert!(
        header.starts_with(&format!(
            "; {}\nX3 29 100 P 0.04 0.08 Some_Maker\n",
            hpr::hpr_core::tool::stamp()
        )),
        "{header}"
    );
}

/// What `hpr convert` refuses: an output that isn't a motor file, the input itself, a missing
/// folder, a name that isn't a file or a catalog motor, `--delays` where it can't be used, and a
/// `.rse` motor without delays written as `.eng` until `--delays` gives them.
#[test]
fn convert_refuses_what_it_cant_write() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let refused = |args: &[&str], says: &str| {
        let message = said(&json_error(args, 1, "input"));
        assert!(message.contains(says), "{args:?}: {message}");
    };
    // A copy: were the check to fail, the bundled curve would be written over.
    let eng = path("in.eng");
    std::fs::copy(curve_file(ENG_CURVE), &eng).unwrap();
    refused(
        &["convert", &eng, &path("out.txt")],
        "writes a .eng or a .rse file",
    );
    refused(
        &["convert", &eng, &eng],
        "that is the file hpr convert reads",
    );
    refused(
        &["convert", "I175", &path("out.rse")],
        "I175 names 2 catalog motors: ",
    );
    for delays in ["abc", "6 10", "6-x", ""] {
        refused(
            &["convert", &eng, &path("out.rse"), "--delays", delays],
            "not a list of delays",
        );
    }
    std::fs::write(
        dir.path().join("latin1.eng"),
        b"; caf\xe9\nX 1 2 P 0.1 0.2 M\n 1 1\n",
    )
    .unwrap();
    refused(
        &["convert", &path("latin1.eng"), &path("out.rse")],
        "not a text file in UTF-8",
    );
    refused(
        &["convert", &eng, &path("missing/out.rse")],
        "there is no folder",
    );
    refused(
        &["convert", "no-such-motor", &path("out.rse")],
        "neither a .eng or .rse file nor a motor in the bundled catalog",
    );
    refused(
        &["convert", &eng, &path("out.rse"), "--delays", "P"],
        "no motor this conversion writes needs them",
    );
    let rse = std::fs::read_to_string(curve_file(RSE_CURVE)).unwrap();
    let start = rse.find(" delays=\"").unwrap();
    let end = start + 1 + rse[start + 1..].find('"').unwrap();
    let end = end + 1 + rse[end + 1..].find('"').unwrap();
    let undelayed = format!("{}{}", &rse[..start], &rse[end + 1..]);
    std::fs::write(dir.path().join("undelayed.rse"), undelayed).unwrap();
    refused(
        &["convert", &path("undelayed.rse"), &path("out.eng")],
        "F15 give(s) no delays, and a .eng header must\nhelp: give them with --delays",
    );
    json(
        &[
            "convert",
            &path("undelayed.rse"),
            &path("out.eng"),
            "--delays",
            "P",
        ],
        0,
        "convert.schema.json",
    );
    assert!(
        std::fs::read_to_string(dir.path().join("out.eng"))
            .unwrap()
            .lines()
            .any(|line| line.starts_with("F15 ") && line.split_whitespace().nth(3) == Some("P"))
    );
    // An existing file is replaced.
    std::fs::write(dir.path().join("old.rse"), "not a motor").unwrap();
    text_ok(&["convert", &eng, &path("old.rse")]);
    let replaced = std::fs::read_to_string(dir.path().join("old.rse")).unwrap();
    assert!(
        replaced.contains("<engine-database>") && !replaced.contains("not a motor"),
        "{replaced}"
    );
}

/// The invented log `hpr_flightdata`'s tests write, in PerfectFlite's `.pf2`.
const SYNTHETIC_LOG: &str = "validation/fixtures/logs/synthetic-pnut.pf2";

/// M4.2d's done-when: `hpr analyze` reads a log in a folder that holds nothing else, so no design
/// file is present, and its JSON validates. Every reading is the library's.
#[test]
fn analyze_reads_a_log_with_no_design_file_present() {
    use hpr::hpr_flightdata::{perfectflite, readings};

    let folder = tempfile::tempdir().unwrap();
    std::fs::copy(root().join(SYNTHETIC_LOG), folder.path().join("flight.pf2")).unwrap();
    let files: Vec<_> = std::fs::read_dir(folder.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(files, ["flight.pf2"]);
    let run = |json: bool| {
        let mut command = assert_cmd::cargo::cargo_bin_cmd!("hpr");
        command
            .current_dir(folder.path())
            .args(["analyze", "flight.pf2"]);
        if json {
            command.arg("--json");
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert!(output.stderr.is_empty(), "{}", text(&output.stderr));
        text(&output.stdout)
    };
    let document: Value = serde_json::from_str(&run(true)).unwrap();
    validate("analyze.schema.json", &document);

    let log =
        perfectflite::read(&std::fs::read_to_string(root().join(SYNTHETIC_LOG)).unwrap()).unwrap();
    let read = readings::read(&log);
    assert_eq!(document["log"]["path"], "flight.pf2");
    assert_eq!(document["log"]["format"], "perfect_flite_pf2");
    assert_eq!(document["log"]["logger"], "PerfectFlite Pnut");
    assert_eq!(document["log"]["samples"], log.time_s.len());
    assert_eq!(document["stated"]["apogee_m"], log.stated.apogee_m.unwrap());
    let apogee = read.apogee.value().unwrap();
    assert_eq!(document["apogee"]["status"], "read");
    assert_eq!(document["apogee"]["altitude_m"], apogee.altitude_m);
    assert_eq!(document["apogee"]["time_s"], apogee.time_s);
    assert_eq!(document["apogee"]["source"], "barometer");
    assert_eq!(
        document["apogee"]["highest_sample"]["altitude_m"],
        apogee.highest_sample.altitude_m
    );
    let liftoff = read.liftoff.value().unwrap();
    assert_eq!(document["liftoff"]["time_s"], liftoff.time_s);
    let speed = read.max_speed.value().unwrap();
    assert_eq!(document["max_speed"]["speed_m_s"], speed.speed_m_s);
    assert_eq!(
        document["max_speed"]["source"],
        "logger_speed_from_barometer"
    );
    assert_eq!(document["max_acceleration"]["status"], "withheld");
    assert_eq!(document["max_acceleration"]["reason"], "no_accelerometer");
    let landing = read.landing.value().unwrap();
    assert_eq!(document["landing"]["time_s"], landing.time_s);
    assert_eq!(
        document["landing"]["mean_descent_rate_m_s"],
        landing.mean_descent_rate_m_s
    );

    // The text says the same, in meters and feet.
    let printed = run(false);
    for expected in [
        "flight.pf2: PerfectFlite Pnut, serial 0, flight 1",
        "the logger states: apogee 390.4 m (1281 ft)",
        "apogee            390.1 m (1280 ft) at 10.28 s",
        "set aside by the median",
        "top speed         79.9 m/s (262 ft/s) at 2.10 s",
        "top acceleration  withheld: a PerfectFlite logger has no accelerometer",
    ] {
        assert!(printed.contains(expected), "{expected:?} in\n{printed}");
    }
}

/// What `hpr analyze` can't read is an input error that says why: a missing file, a file in
/// another format, and a `.pf2` that goes wrong at a line.
#[test]
fn analyze_refuses_what_it_cannot_read() {
    let folder = tempfile::tempdir().unwrap();
    let missing = folder.path().join("none.pf2");
    let document = json_error(&["analyze", missing.to_str().unwrap()], 1, "input");
    let message = document["error"]["message"].as_str().unwrap();
    // The operating system's own "not found", the same code on all three.
    assert!(
        message.contains("none.pf2: ") && message.contains("(os error 2)"),
        "{message}"
    );

    let design = repo_file("validation/fixtures/ork/pod-flights/pods-none.ork");
    let document = json_error(&["analyze", &design], 1, "input");
    assert!(
        document["error"]["message"]
            .as_str()
            .unwrap()
            .contains("reads PerfectFlite .pf2 logs so far, and this isn't one"),
        "{document:#}"
    );

    let broken = folder.path().join("broken.pf2");
    std::fs::write(&broken, "PerfectFlite Pnut\n0.00, 0, 0\n0.00, 1, 0\n").unwrap();
    let output = hpr(&["analyze", broken.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let message = text(&output.stderr);
    assert!(
        message.contains(".pf2 line 3: the time 0 s doesn't come after"),
        "{message}"
    );

    // A comment in another encoding than UTF-8 doesn't refuse the flight.
    let latin = folder.path().join("latin.pf2");
    let mut bytes = std::fs::read(root().join(SYNTHETIC_LOG)).unwrap();
    let at = bytes.windows(9).position(|w| w == b"Comments:").unwrap() + 10;
    bytes.splice(at..at, *b"caf\xe9 ");
    std::fs::write(&latin, bytes).unwrap();
    let document = json(
        &["analyze", latin.to_str().unwrap()],
        0,
        "analyze.schema.json",
    );
    assert_eq!(document["apogee"]["status"], "read");

    // A PerfectFlite log named otherwise is read by its first line.
    let renamed = folder.path().join("flight.txt");
    std::fs::copy(root().join(SYNTHETIC_LOG), &renamed).unwrap();
    let document = json(
        &["analyze", renamed.to_str().unwrap()],
        0,
        "analyze.schema.json",
    );
    assert_eq!(document["apogee"]["status"], "read");
}

/// The public Pnut log Debrief ships, where `refs/` has it (`cargo xtask refs fetch`): hpr reads
/// the apogee the logger states to a foot, and sets aside the ejection pulse a Hampel filter keeps.
/// The guide quotes these numbers. The file's upstream terms are unclear, so it is not committed
/// and CI doesn't have it.
#[test]
fn analyze_reads_the_public_pnut_log_as_it_states() {
    use hpr::hpr_flightdata::filter;
    use hpr::hpr_flightdata::perfectflite::{self, FOOT_M};

    let path =
        root().join("refs/fusionspace-debrief/lib/parsers/__fixtures__/perfectflite-pnut.pf2");
    if !path.is_file() {
        eprintln!("skipped: {} isn't fetched", path.display());
        return;
    }
    let document = json(
        &["analyze", path.to_str().unwrap()],
        0,
        "analyze.schema.json",
    );
    let feet = |value: &Value| (value.as_f64().unwrap() / FOOT_M).round();
    assert_eq!(feet(&document["stated"]["apogee_m"]), 1009.0);
    assert_eq!(feet(&document["apogee"]["altitude_m"]), 1010.0);
    assert_eq!(
        feet(&document["apogee"]["highest_sample"]["altitude_m"]),
        1028.0
    );
    assert_eq!(document["liftoff"]["time_s"], 0.15);
    assert_eq!(feet(&document["max_speed"]["speed_m_s"]), 257.0);
    assert_eq!(document["landing"]["status"], "read");

    // Debrief's Hampel filter, 0.3 s at threshold 4, keeps the pulse: its highest is the pulse's.
    let log = perfectflite::read(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let highest = |trace: &[f64]| trace.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(
        (highest(&filter::hampel(&log.altitude_m, 3, 4.0)) / FOOT_M).round(),
        1028.0
    );
}

/// A public design taken `.ork` → `.hpr` → `.hprz` (with a file attached) → `.ork`: each file is
/// the library's, byte for byte, and the JSON says what was read and written.
#[test]
fn convert_takes_a_design_through_every_format() {
    use hpr::hpr_format::{self, DesignFile, container};
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let source = root().join("validation/fixtures/ork/loft-demo/demo-multi-config.ork");
    let source = source.to_string_lossy().into_owned();
    let expected = DesignFile::from_ork(&std::fs::read(&source).unwrap())
        .unwrap()
        .value;

    let hpr_file = path("demo.hpr");
    let document = json(&["convert", &source, &hpr_file], 0, "convert.schema.json");
    assert_eq!(document["input"]["format"], "ork");
    assert_eq!(document["output"]["format"], "hpr");
    assert_eq!(document["rocket"], expected.rocket.name.as_str());
    assert_eq!(document["configurations"], 2);
    assert_eq!(document["migrated_from"], Value::Null);
    let written = std::fs::read_to_string(&hpr_file).unwrap();
    assert_eq!(written, hpr_format::to_json(&expected).unwrap());

    let log = path("flight-1.csv");
    std::fs::write(&log, "time_s,altitude_m\n0,0\n").unwrap();
    let hprz_file = path("demo.hprz");
    let document = json(
        &["convert", &hpr_file, &hprz_file, "--attach", &log],
        0,
        "convert.schema.json",
    );
    assert_eq!(document["attachments"], serde_json::json!(["flight-1.csv"]));
    let read = container::read(&std::fs::read(&hprz_file).unwrap()).unwrap();
    assert_eq!(read.value.design, expected);
    assert_eq!(read.value.attachments.len(), 1);
    assert_eq!(read.value.attachments[0].name, "flight-1.csv");
    assert_eq!(read.value.attachments[0].bytes, b"time_s,altitude_m\n0,0\n");

    // A .hprz to a .hprz keeps its attachments; to a .ork, each is named as left out.
    let again = path("again.hprz");
    let document = json(&["convert", &hprz_file, &again], 0, "convert.schema.json");
    assert_eq!(document["attachments"], serde_json::json!(["flight-1.csv"]));
    assert_eq!(
        std::fs::read(&again).unwrap(),
        std::fs::read(&hprz_file).unwrap()
    );
    let ork_file = path("demo.ork");
    let document = json(
        &["convert", &hprz_file, &ork_file],
        0,
        "convert.schema.json",
    );
    assert_eq!(document["warnings"][0]["at"], "flight-1.csv");
    assert_eq!(document["warnings"][0]["kind"], "dropped");
    assert_eq!(
        std::fs::read(&ork_file).unwrap(),
        expected.to_ork().unwrap().value
    );

    // The text says the same.
    let shown = streams(&["convert", &hprz_file, &path("text.hpr")]);
    assert!(
        shown.out.starts_with("read   demo.hprz\nwrote  text.hpr: "),
        "{}",
        shown.out
    );
    assert!(
        shown
            .out
            .contains("2 motor configurations, HPR design format 0.2"),
        "{}",
        shown.out
    );
    assert!(
        shown
            .err
            .starts_with("warning: flight-1.csv: the .hpr file has no place"),
        "{}",
        shown.err
    );
}

/// A document of the format's older version is migrated, and the JSON says from which; the file
/// written names this program as the one that wrote it, and keeps where the design came from.
#[test]
fn convert_migrates_an_older_design() {
    use hpr::hpr_format;
    let dir = tempfile::tempdir().unwrap();
    let old = root().join("crates/hpr-format/fixtures/embedded-curve-0.1.hpr");
    let old = old.to_string_lossy().into_owned();
    let out = dir.path().join("new.hpr").to_string_lossy().into_owned();
    let document = json(&["convert", &old, &out], 0, "convert.schema.json");
    assert_eq!(document["migrated_from"], "0.1");
    let migrated = hpr_format::from_json(&std::fs::read_to_string(&old).unwrap()).unwrap();
    assert_eq!(
        std::fs::read_to_string(&out).unwrap(),
        hpr_format::to_json(&migrated.stamped()).unwrap()
    );
    let output = hpr(&["convert", &old, &out]);
    assert!(
        text(&output.stdout)
            .starts_with("read   embedded-curve-0.1.hpr, migrated from version 0.1\n"),
        "{output:?}"
    );
}

#[test]
fn convert_refuses_a_design_it_cant_write() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let refused = |args: &[&str], says: &str| {
        let document = json_error(args, 1, "input");
        let message = document["error"]["message"].as_str().unwrap().to_owned();
        assert!(message.contains(says), "{args:?}: {message}");
    };
    let ork = path("in.ork");
    std::fs::copy(
        root().join("validation/fixtures/ork/loft-demo/demo-stable.ork"),
        &ork,
    )
    .unwrap();
    let from_motors = "writes a design (.ork, .hpr or .hprz) from a design";
    refused(&["convert", &ork, &path("out.eng")], from_motors);
    refused(
        &["convert", &curve_file(ENG_CURVE), &path("out.hpr")],
        from_motors,
    );
    refused(&["convert", "H170M", &path("out.hprz")], from_motors);
    refused(
        &["convert", &ork, &ork],
        "that is the file hpr convert reads",
    );
    refused(
        &["convert", &ork, &path("missing/out.hpr")],
        "there is no folder",
    );
    refused(
        &["convert", &ork, &path("out.hpr"), "--delays", "6"],
        "hpr convert is writing a design",
    );
    refused(
        &["convert", &ork, &path("out.hpr"), "--attach", &ork],
        "--attach adds files to a .hprz, and this is not one",
    );
    refused(
        &[
            "convert",
            &curve_file(ENG_CURVE),
            &path("out.rse"),
            "--attach",
            &ork,
        ],
        "--attach adds files to a .hprz design",
    );
    refused(
        &[
            "convert",
            &ork,
            &path("out.hprz"),
            "--attach",
            &path("absent.csv"),
        ],
        "absent.csv",
    );
    refused(
        &[
            "convert",
            &ork,
            &path("out.hprz"),
            "--attach",
            &ork,
            "--attach",
            &ork,
        ],
        "two attachments are named \"in.ork\"",
    );
    let design = path("design.hpr");
    std::fs::write(&design, "{}").unwrap();
    refused(
        &["convert", &ork, &path("out.hprz"), "--attach", &design],
        "which is the design's entry",
    );
    refused(&["convert", &design, &path("out.ork")], "not an HPR design");
    std::fs::write(path("bad.hprz"), b"PK\x03\x04 and no more").unwrap();
    refused(
        &["convert", &path("bad.hprz"), &path("out.hpr")],
        "not a valid .hprz",
    );
    std::fs::write(path("latin1.hpr"), b"{\"caf\xe9\": 1}").unwrap();
    refused(
        &["convert", &path("latin1.hpr"), &path("out.ork")],
        "UTF-8 text",
    );
    assert!(!dir.path().join("out.hpr").exists() && !dir.path().join("out.hprz").exists());
}

/// A design flies the same from its `.ork`, its `.hpr` and its `.hprz`: only the file named and
/// the container's note differ.
#[test]
fn sim_flies_an_hpr_design_as_its_ork() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_string_lossy().into_owned();
    let ork = root().join("validation/fixtures/ork/pod-flights/pods-none.ork");
    let ork = ork.to_string_lossy().into_owned();
    assert_eq!(
        hpr(&["convert", &ork, &path("pods.hpr")]).status.code(),
        Some(0)
    );
    let log = path("log.csv");
    std::fs::write(&log, "t\n").unwrap();
    let written = hpr(&["convert", &ork, &path("pods.hprz"), "--attach", &log]);
    assert_eq!(written.status.code(), Some(0));
    let fly = |file: &str| json(&["sim", file, "--motor", "H54"], 0, "sim.schema.json");
    let from_ork = fly(&ork);
    for (file, format) in [(path("pods.hpr"), "hpr"), (path("pods.hprz"), "hprz")] {
        let mut flown = fly(&file);
        assert_eq!(flown["design"]["format"], format);
        flown["design"]["file"] = from_ork["design"]["file"].clone();
        flown["design"]["format"] = from_ork["design"]["format"].clone();
        if format == "hprz" {
            let notes = flown["notes"].as_array_mut().unwrap();
            let at = notes
                .iter()
                .position(|note| note == "the container's 1 attachment is not read")
                .unwrap();
            notes.remove(at);
        }
        assert_eq!(flown, from_ork, "{file}");
    }
    // An older document flies too, with a note that it was migrated.
    let old = root().join("crates/hpr-format/fixtures/embedded-curve-0.1.hpr");
    let flown = json(&["sim", &old.to_string_lossy()], 0, "sim.schema.json");
    assert_eq!(
        flown["notes"][0],
        "the design is version 0.1 of the HPR design format, read as version 0.2"
    );
    assert_eq!(flown["motors"][0]["designation"], "H128W");
}

#[test]
fn sim_flags_a_rocket_unstable_under_power() {
    // Valetudo with its fins taken off: its nose alone carries the normal force, ahead of the
    // center of mass, so its static margin is below zero while the motor burns. Stock, the margin
    // stays above zero and nothing is raised.
    let stock = repo_file("validation/designs/rocketpy-valetudo.json");
    let document = json(&["sim", &stock, "--offline"], 0, "sim.schema.json");
    let least = &document["summary"]["min_powered_static_margin_cal"];
    assert!(least["value"].as_f64().unwrap() > 0.0, "{least}");
    assert_eq!(document["flags"], serde_json::json!([]));
    let mut rocket: Value =
        serde_json::from_str(&std::fs::read_to_string(&stock).unwrap()).unwrap();
    let children = rocket["stages"][0]["components"][1]["children"]
        .as_array_mut()
        .unwrap();
    let before = children.len();
    children.retain(|child| child["id"] != "fins-1");
    assert_eq!(children.len(), before - 1);
    let folder = tempfile::tempdir().unwrap();
    let finless = folder.path().join("finless.json");
    std::fs::write(&finless, serde_json::to_string(&rocket).unwrap()).unwrap();
    let path = finless.to_string_lossy().into_owned();
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let least = &document["summary"]["min_powered_static_margin_cal"];
    assert!(least["value"].as_f64().unwrap() < 0.0, "{least}");
    let flags = document["flags"].as_array().unwrap();
    assert_eq!(flags[0]["flag"], "unstable_under_power", "{flags:?}");
    assert_eq!(&flags[0]["peak"], least);
    let message = flags[0]["message"].as_str().unwrap();
    assert!(
        message.starts_with(&format!(
            "the static margin falls to {:.2} calibres",
            least["value"].as_f64().unwrap()
        )),
        "{message}"
    );
    assert!(
        message.ends_with("its apogee is not a prediction"),
        "{message}"
    );
    let shown = streams(&["sim", &path, "--offline", "--color", "never"]);
    let err = &shown.err;
    assert!(
        err.contains(&format!("warning: unstable: {message}\n")),
        "{err}"
    );
    assert!(
        err.contains(
            "help: unstable under power: \
             https://hpr.fusionspace.co/physics/metrics.html#unstable-under-power\n"
        ),
        "{err}"
    );
    let out = &shown.out;
    assert!(
        out.contains(", not a prediction: unstable under power\n"),
        "{out}"
    );
}

#[test]
fn sim_flags_a_rocket_unstable_where_it_has_no_margin() {
    // Valetudo with its fins taken off and a conical tail that narrows from the body's 40.45 mm
    // radius to 22 mm over 0.08 m: the tail's negative normal force leaves a net slope too small
    // against the parts' sum for a margin to mean anything, so hpr gives none, yet the pitching
    // moment still turns the rocket away from its path. With the tail ending at 30 mm the margin
    // is defined, below zero, and the margin's flag fires instead: one unstable flag a flight.
    let stock = repo_file("validation/designs/rocketpy-valetudo.json");
    let folder = tempfile::tempdir().unwrap();
    let design = |aft_radius_m: f64, name: &str| {
        let mut rocket: Value =
            serde_json::from_str(&std::fs::read_to_string(&stock).unwrap()).unwrap();
        let components = rocket["stages"][0]["components"].as_array_mut().unwrap();
        let children = components[1]["children"].as_array_mut().unwrap();
        let before = children.len();
        children.retain(|child| child["id"] != "fins-1");
        assert_eq!(children.len(), before - 1);
        assert_eq!(
            components[1]["part"]["body_tube"]["outer_radius_m"],
            0.04045
        );
        components.push(serde_json::json!({
            "id": "tail-1",
            "name": "",
            "part": {"transition": {
                "shape": {"kind": "conical"},
                "clipped": false,
                "length_m": 0.08,
                "fore_radius_m": 0.04045,
                "aft_radius_m": aft_radius_m,
                "wall": {"kind": "shell", "thickness_m": 0.002},
                "fore_shoulder": null,
                "aft_shoulder": null,
                "material": {
                    "name": "Fiberglass laminate (G10/FR-4)",
                    "density": {"kind": "bulk", "kg_m3": 1800.0}
                }
            }}
        }));
        let path = folder.path().join(name);
        std::fs::write(&path, serde_json::to_string(&rocket).unwrap()).unwrap();
        path.to_string_lossy().into_owned()
    };
    let path = design(0.022, "narrow-tail.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let summary = &document["summary"];
    assert_eq!(summary["min_powered_static_margin_cal"], Value::Null);
    let rail = &summary["rail_exit_stability"]["static_margin"];
    assert_eq!(rail["margin_cal"], Value::Null, "{rail}");
    let slope = &summary["max_powered_moment_slope_per_rad"];
    assert!(slope["value"].as_f64().unwrap() > 1.0, "{slope}");
    let flags = document["flags"].as_array().unwrap();
    assert_eq!(flags.len(), 1, "{flags:?}");
    assert_eq!(flags[0]["flag"], "unstable_without_margin");
    assert_eq!(&flags[0]["peak"], slope);
    let message = flags[0]["message"].as_str().unwrap();
    assert!(
        message.starts_with(&format!(
            "while a motor burns, the pitching moment turns the rocket away from its path (C_mα \
             {:+.1} per radian at",
            slope["value"].as_f64().unwrap()
        )),
        "{message}"
    );
    assert!(
        message.ends_with("its apogee is not a prediction"),
        "{message}"
    );
    let shown = streams(&["sim", &path, "--offline", "--color", "never"]);
    let err = &shown.err;
    assert!(
        err.contains(&format!("warning: unstable: {message}\n")),
        "{err}"
    );
    assert!(
        err.contains(
            "help: unstable under power: \
             https://hpr.fusionspace.co/physics/metrics.html#unstable-under-power\n"
        ),
        "{err}"
    );
    let out = &shown.out;
    assert!(
        out.contains(", not a prediction: unstable under power\n"),
        "{out}"
    );
    // A wider tail: the margin is defined, below zero, and only its flag is raised.
    let path = design(0.030, "wider-tail.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let least = &document["summary"]["min_powered_static_margin_cal"];
    assert!(least["value"].as_f64().unwrap() < 0.0, "{least}");
    let flags: Vec<&Value> = document["flags"].as_array().unwrap().iter().collect();
    assert_eq!(flags.len(), 1, "{flags:?}");
    assert_eq!(flags[0]["flag"], "unstable_under_power");
}

#[test]
fn sim_flags_a_flight_past_the_validated_range() {
    // The synthetic two-stage rocket flies as one stack to Mach 1.61, past the fastest public
    // flight compared with an independent reference (Mach 1.147) but inside the core band.
    let path = repo_file("validation/designs/synthetic-two-stage-75mm-54mm.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let flags = document["flags"].as_array().unwrap();
    assert_eq!(flags.len(), 1, "{flags:?}");
    assert_eq!(flags[0]["flag"], "beyond_validated_range");
    let mach = flags[0]["peak"]["value"].as_f64().unwrap();
    assert!(mach > 1.147 && mach < 2.5, "{mach}");
    assert_eq!(flags[0]["peak"], document["summary"]["max_mach"]);
    let message = flags[0]["message"].as_str().unwrap();
    assert!(
        message.contains(&format!("reaches Mach {mach:.2}")),
        "{message}"
    );
    let err = streams(&["sim", &path, "--offline", "--color", "never"]).err;
    assert!(
        err.contains(&format!("warning: envelope: {message}\n")),
        "{err}"
    );
    assert!(err.contains("help: the operating envelope: "), "{err}");
    // A subsonic flight raises none, and says nothing of the envelope.
    let path = repo_file("validation/designs/rocketpy-calisto-tests-motor-at-minus-1.373.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    assert_eq!(document["flags"], serde_json::json!([]));
    assert!(
        document["summary"]["max_angle_of_attack_rad"]["value"]
            .as_f64()
            .unwrap()
            < 0.26
    );
    let out = streams(&["sim", &path, "--offline", "--color", "never"]).both();
    assert!(!out.contains("envelope"), "{out}");
}

#[test]
fn sim_warns_of_the_drag_issues_a_flight_meets_by_number() {
    // The synthetic two-stage rocket, past Mach 1: #67 (its tangent-ogive nose and its conical
    // interstage, a shoulder, transonic), #68 (base drag, every rocket) and #222 (the nose's
    // supersonic pressure drag), #18 (fully turbulent friction, every flight on hpr's drag) and
    // the stability issue #172, which every flight with a margin raises (ADR-181); then two of
    // the flight's path, whose signs depend on the case (ADR-201): #219, its rail buttons putting
    // its center of gravity off the axis, and #106, past the body's supersonic join's start.
    let path = repo_file("validation/designs/synthetic-two-stage-75mm-54mm.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let issues = document["issues"].as_array().unwrap();
    let numbers: Vec<u64> = issues
        .iter()
        .map(|i| i["issue"].as_u64().unwrap())
        .collect();
    assert_eq!(numbers, [67, 68, 222, 18, 172, 219, 106]);
    let kinds: Vec<&str> = issues.iter().map(|i| i["kind"].as_str().unwrap()).collect();
    assert_eq!(
        kinds,
        [
            "drag",
            "drag",
            "drag",
            "drag",
            "stability",
            "flight",
            "flight"
        ]
    );
    assert_eq!(issues[5]["parts"], serde_json::json!([]));
    assert_eq!(
        issues[6]["parts"],
        serde_json::json!(["nose", "sustainer-airframe", "interstage"])
    );
    assert_eq!(
        issues[0]["parts"],
        serde_json::json!(["nose", "interstage"])
    );
    assert_eq!(issues[2]["parts"], serde_json::json!(["nose"]));
    assert_eq!(issues[1]["parts"], serde_json::json!([]));
    assert_eq!(issues[3]["parts"], serde_json::json!([]));
    let out = streams(&["sim", &path, "--offline", "--color", "never"]).err;
    for issue in issues {
        assert_eq!(issue["max_mach"], document["summary"]["max_mach"]);
        let message = issue["message"].as_str().unwrap();
        assert!(
            message.contains(&format!("issue #{}", issue["issue"])),
            "{message}"
        );
        assert!(
            out.contains(&format!(
                "warning: {}: {message}\n",
                issue["kind"].as_str().unwrap()
            )),
            "{out}"
        );
    }
    // Calisto tops out at Mach 0.73, short of every Mach edge (Mach 0.8 the lowest): it raises
    // #73 for its 18.4° boattail, which needs no speed (ADR-189), #18 and #172.
    let path = repo_file("validation/designs/rocketpy-calisto-tests-motor-at-minus-1.373.json");
    let document = json(&["sim", &path, "--offline"], 0, "sim.schema.json");
    let numbers: Vec<u64> = document["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["issue"].as_u64().unwrap())
        .collect();
    assert_eq!(numbers, [73, 18, 172]);
    let out = streams(&["sim", &path, "--offline", "--color", "never"]).both();
    let drag: Vec<&str> = out.lines().filter(|line| line.contains("drag:")).collect();
    assert_eq!(drag.len(), 2, "{out}");
    assert!(drag[0].starts_with("warning: drag: issue #73: "), "{out}");
    assert!(drag[1].starts_with("warning: drag: issue #18: "), "{out}");
}

/// The CSV `hpr mc --export` wrote at `csv`, checked against the library's `run`: the same
/// text, so every number is the run's to the bit (the table's own test reads each back), and a
/// sidecar naming the run.
fn same_runs(csv: &Path, run: &hpr::hpr_analysis::montecarlo::Run, document: &Value) {
    let table = hpr::hpr_analysis::table::RunTable::new(run);
    assert_eq!(std::fs::read_to_string(csv).unwrap(), table.csv().unwrap());
    let meta: Value =
        serde_json::from_str(&std::fs::read_to_string(csv.with_extension("meta.json")).unwrap())
            .unwrap();
    validate("mc-export-meta.schema.json", &meta);
    assert_eq!(meta["seed"], run.seed);
    assert_eq!(meta["runs"], run.samples.len());
    assert_eq!(document["export"]["rows"], run.samples.len());
    // The spreads, to the bit.
    let failed = run.failed().count();
    assert_eq!(document["failed"]["count"], failed);
    assert_eq!(document["flown"], run.samples.len() - failed);
    let apogee = serde_json::to_value(run.apogee().unwrap().summary()).unwrap();
    same_bits(&document["apogee_m"], &apogee, "apogee_m");
    let distance = run
        .distribution(|flight| flight.nose_landing().map(|landing| landing.distance_m))
        .unwrap();
    let distance = serde_json::to_value(distance.summary()).unwrap();
    same_bits(
        &document["landing_distance_m"],
        &distance,
        "landing_distance_m",
    );
    let landing = run.landing().unwrap();
    let ellipse = landing.ellipse(0.95).unwrap().unwrap();
    let printed = &document["landing"]["ellipses"][1];
    assert_eq!(printed["kind"], "scatter");
    for (key, value) in [
        ("semi_major_m", ellipse.semi_major_m),
        ("semi_minor_m", ellipse.semi_minor_m),
        ("center_east_m", ellipse.center_east_m),
    ] {
        assert_eq!(
            printed[key].as_f64().unwrap().to_bits(),
            value.to_bits(),
            "{key}"
        );
    }
    assert_eq!(
        printed["inside"].as_f64().unwrap().to_bits(),
        landing.share_inside(&ellipse).unwrap().low.to_bits()
    );
}

/// M4.6a's bullet: `hpr mc` on a public `.ork`, with every scatter given, flies the runs the
/// Rust API's `MonteCarlo::run` flies for the same seed, bit for bit: the CLI spreads its flights
/// over threads, the library here flies them one after another. Its JSON validates, and the
/// nominal flight is `hpr sim`'s.
#[test]
fn mc_flies_a_public_ork_as_the_library_does() {
    use hpr::hpr_analysis::montecarlo::{Dispersion, MonteCarlo};
    let folder = tempfile::tempdir().unwrap();
    let csv = folder.path().join("runs.csv");
    let csv_arg = csv.to_string_lossy().into_owned();
    let design = repo_file(PROBE);
    let launch = [
        "--motor",
        "H54",
        "--wind",
        "4",
        "--wind-from",
        "270",
        "--inclination",
        "85",
        "--heading",
        "270",
    ];
    let scatter = [
        "--mass-sd",
        "0.02",
        "--cg-sd",
        "0.005",
        "--drag-sd",
        "0.05",
        "--impulse-sd",
        "0.03",
        "--burn-time-sd",
        "0.02",
        "--delay-sd",
        "0.5",
        "--wind-sd",
        "0.25",
        "--wind-from-sd",
        "15",
        "--inclination-sd",
        "1",
        "--heading-sd",
        "2",
        "--deployment-lag-sd",
        "0.3",
    ];
    let mut args = vec!["mc", &design, "--runs", "24", "--seed", "2026"];
    args.extend(launch);
    args.extend(scatter);
    args.extend(["--export", &csv_arg]);
    let document = json(&args, 0, "mc.schema.json");

    let (rocket, id, mount) = probe_design();
    let motor = hpr::Motor::from_catalog("H54").unwrap();
    let environment = hpr::Environment::new(0.0, 0.0, 0.0)
        .unwrap()
        .with_constant_wind(4.0, 270.0)
        .unwrap();
    let (nominal, _) = library_flight(
        rocket.clone(),
        &id,
        &mount,
        Some(&motor),
        &environment,
        |b| b.inclination_deg(85.0).heading_deg(270.0),
    );
    let mut design_rocket = rocket;
    design_rocket.configurations.retain(|c| c.id != id);
    design_rocket
        .configurations
        .push(hpr::hpr_design::Configuration {
            id: id.clone(),
            name: String::new(),
            motors: vec![hpr::hpr_design::MountedMotor {
                mount: mount.clone(),
                designation: motor.designation().to_owned(),
                diameter_m: motor.diameter_m(),
                length_m: motor.length_m(),
                motor: motor.solid_motor().clone(),
                delay: motor.delay(),
                ignition: hpr::hpr_design::Ignition::Launch,
                failed_tubes: Vec::new(),
            }],
        });
    let rocket = hpr::Rocket::from_design(design_rocket, &id).unwrap();
    let inputs = hpr::Flight::builder(&rocket, &environment, hpr_cli::sim::DEFAULT_RAIL_LENGTH_M)
        .inclination_deg(85.0)
        .heading_deg(270.0)
        .inputs()
        .unwrap();
    let dispersion = Dispersion {
        dry_mass_sd_fraction: 0.02,
        cg_sd_m: 0.005,
        drag_sd_fraction: 0.05,
        impulse_sd_fraction: 0.03,
        burn_time_sd_fraction: 0.02,
        ejection_delay_sd_s: 0.5,
        wind_speed_sd_fraction: 0.25,
        wind_heading_sd_rad: 15_f64.to_radians(),
        rail_elevation_sd_rad: 1_f64.to_radians(),
        rail_azimuth_sd_rad: 2_f64.to_radians(),
        deployment_lag_sd_s: 0.3,
    };
    let run = MonteCarlo::new(inputs, dispersion).unwrap().run(2026, 24);
    assert_eq!(run.failed().count(), 0);
    same_runs(&csv, &run, &document);
    // Every input moved: no column of the draw holds one value.
    let table = hpr::hpr_analysis::table::RunTable::new(&run);
    for name in [
        "drag_scale",
        "wind_speed_scale",
        "wind_turn_rad",
        "rail_elevation_offset_rad",
        "rail_azimuth_offset_rad",
        "stage_1_dry_mass_scale",
        "stage_1_cg_shift_m",
        "motor_1_impulse_scale",
        "motor_1_burn_time_scale",
        "motor_1_ejection_delay_offset_s",
    ] {
        let hpr::hpr_analysis::table::Values::Numbers(values) = &table.column(name).unwrap().values
        else {
            panic!("{name} isn't numbers");
        };
        assert!(values.iter().any(|v| *v != values[0]), "{name}");
    }
    // The nominal flight is `hpr sim`'s.
    let summary = nominal.summary();
    assert_eq!(
        document["nominal"]["apogee_m"].as_f64().unwrap().to_bits(),
        summary.apogee.unwrap().height_above_ground_m.to_bits()
    );
    assert_eq!(
        document["nominal"]["landing_distance_m"]
            .as_f64()
            .unwrap()
            .to_bits(),
        summary.landing.unwrap().distance_m.to_bits()
    );
    assert_eq!(document["dispersion"]["wind_from_sd_deg"], 15.0);
    assert_eq!(document["seed"], 2026);
    assert_eq!(document["runs"], 24);
    assert_eq!(document["failed"]["reasons"], json!([]));
    assert_eq!(document["landing"]["ellipses"].as_array().unwrap().len(), 3);
    // The text says what the JSON says.
    let mut args = vec!["mc", &design, "--runs", "24", "--seed", "2026"];
    args.extend(launch);
    args.extend(scatter);
    let text = streams(&args).out;
    let apogee = run.apogee().unwrap().summary();
    assert!(
        text.contains(&format!(
            "apogee (m AGL)        {:>9.1}{:>9.1}{:>9.1}",
            summary.apogee.unwrap().height_above_ground_m,
            apogee.mean.unwrap(),
            apogee.standard_deviation.unwrap()
        )),
        "{text}"
    );
    assert!(
        text.contains("24 simulated flights, seed 2026: 0 failed"),
        "{text}"
    );
    assert!(text.contains("--wind-from-sd 15"), "{text}");
}

/// M4.6a's bullet: a staged `.ork`, its booster dropping under power with a parachute on each
/// part, is scattered with its separation, and `hpr mc` flies the library's runs bit for bit:
/// the booster's landing in its own columns, both parachutes' lags drawn.
#[test]
fn mc_flies_a_staged_ork_as_the_library_does() {
    use hpr::hpr_analysis::montecarlo::{Dispersion, MonteCarlo};
    let ork = two_stage_ork()
        .replacen(
            "<subcomponents><trapezoidfinset><name>Sustainer fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Sustainer fins</name>",
                apogee_parachute("Sustainer chute", 0.45)
            ),
            1,
        )
        .replacen(
            "<subcomponents><trapezoidfinset><name>Booster fins</name>",
            &format!(
                "<subcomponents>{}<trapezoidfinset><name>Booster fins</name>",
                apogee_parachute("Booster chute", 0.3)
            ),
            1,
        );
    assert_eq!(ork.matches("<parachute>").count(), 2);
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("two-stage.ork");
    std::fs::write(&path, &ork).unwrap();
    let path = path.to_string_lossy().into_owned();
    let csv = folder.path().join("runs.csv");
    let csv_arg = csv.to_string_lossy().into_owned();
    let document = json(
        &[
            "mc",
            &path,
            "--runs",
            "12",
            "--seed",
            "5",
            "--wind",
            "3",
            "--wind-sd",
            "0.2",
            "--mass-sd",
            "0.02",
            "--impulse-sd",
            "0.03",
            "--burn-time-sd",
            "0.02",
            "--deployment-lag-sd",
            "0.5",
            "--export",
            &csv_arg,
        ],
        0,
        "mc.schema.json",
    );

    let (inputs, tumbles) = staged_inputs(&ork, 3.0);
    // The two parachutes, and the booster's tumble from the split until its own opens.
    assert_eq!(tumbles, [false, false, true]);
    let dispersion = Dispersion {
        dry_mass_sd_fraction: 0.02,
        impulse_sd_fraction: 0.03,
        burn_time_sd_fraction: 0.02,
        wind_speed_sd_fraction: 0.2,
        deployment_lag_sd_s: 0.5,
        ..Dispersion::default()
    };
    let run = MonteCarlo::new(inputs, dispersion).unwrap().run(5, 12);
    assert_eq!(run.failed().count(), 0);
    same_runs(&csv, &run, &document);
    let table = hpr::hpr_analysis::table::RunTable::new(&run);
    let names: Vec<&str> = table.columns().iter().map(|c| c.name.as_str()).collect();
    for name in [
        "stage_2_dry_mass_scale",
        "motor_2_impulse_scale",
        "device_2_deployment_lag_offset_s",
        "part_1_landing_east_m",
    ] {
        assert!(names.contains(&name), "{name}: {names:?}");
    }
    for sample in &run.samples {
        let summary = sample.summary().unwrap();
        assert_eq!(summary.body_landings.len(), 1);
        // Each parachute's lag is drawn; the tumble, triggered at the split, has none.
        let lags = &sample.draw.deployment_lag_offset_s;
        assert!(
            lags[0] != 0.0 && lags[1] != 0.0 && lags[2] == 0.0,
            "{lags:?}"
        );
    }
}

/// The library's inputs for a staged `.ork`, as `hpr sim` flies it in a constant wind from the
/// east (m/s), and which of its devices are hpr's tumbles.
fn staged_inputs(
    ork: &str,
    wind_m_s: f64,
) -> (hpr::hpr_analysis::montecarlo::FlightInputs, Vec<bool>) {
    let file = hpr::hpr_io::ork::read(ork.as_bytes()).unwrap().value;
    let design = hpr::hpr_io::ork::design(&file).value;
    let configuration = design.motors.default_configuration().unwrap();
    let assembly = design.rocket.assemble(&configuration.id).unwrap();
    let separations = hpr::ork::separations(configuration.stagings(), &assembly).unwrap();
    let devices =
        hpr::ork::separated_recovery(&design.recovery, &design.rocket, &assembly, &separations)
            .unwrap()
            .devices;
    let tumbles = devices
        .iter()
        .map(|device| matches!(device.drag, hpr::DeviceDrag::Tumble { .. }))
        .collect();
    let mut rocket = hpr::Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    for device in devices {
        rocket.add_parachute(device);
    }
    let environment = hpr::Environment::new(0.0, 0.0, 0.0)
        .unwrap()
        .with_constant_wind(wind_m_s, 0.0)
        .unwrap();
    let inputs = hpr::Flight::builder(&rocket, &environment, hpr_cli::sim::DEFAULT_RAIL_LENGTH_M)
        .separations(separations)
        .inputs()
        .unwrap();
    (inputs, tumbles)
}

/// M4.6a, found in review: a payload dropped from its spent booster before apogee ends the
/// stack's flight at the split, so its landing is the part's that keeps the nose. `hpr mc`
/// counts that landing in its spreads, ellipses, CSV and nominal flight, as the library's run
/// has it. A drawn lag on the payload's parachute, which fires at the split, would leave it
/// climbing with no drag: those flights fail by name and are counted.
#[test]
fn mc_lands_a_payload_dropped_before_apogee() {
    use hpr::hpr_analysis::montecarlo::{Dispersion, MonteCarlo};
    let folder = tempfile::tempdir().unwrap();
    let ork = payload_ork(2.0, "lowerstageseparation");
    let path = folder.path().join("payload.ork");
    std::fs::write(&path, &ork).unwrap();
    let path = path.to_string_lossy().into_owned();
    let csv = folder.path().join("runs.csv");
    let csv_arg = csv.to_string_lossy().into_owned();
    let document = json(
        &[
            "mc",
            &path,
            "--runs",
            "10",
            "--seed",
            "3",
            "--wind",
            "3",
            "--wind-sd",
            "0.2",
            "--impulse-sd",
            "0.03",
            "--export",
            &csv_arg,
        ],
        0,
        "mc.schema.json",
    );
    let (inputs, _) = staged_inputs(&ork, 3.0);
    let dispersion = Dispersion {
        impulse_sd_fraction: 0.03,
        wind_speed_sd_fraction: 0.2,
        ..Dispersion::default()
    };
    let run = MonteCarlo::new(inputs.clone(), dispersion)
        .unwrap()
        .run(3, 10);
    assert_eq!(run.failed().count(), 0);
    for sample in &run.samples {
        let summary = sample.summary().unwrap();
        assert!(summary.landing.is_none());
        assert!(summary.nose_landing().is_some());
    }
    same_runs(&csv, &run, &document);
    assert_eq!(document["landing_distance_m"]["count"], 10);
    assert!(document["nominal"]["landing_distance_m"].as_f64().unwrap() > 0.0);
    let table = std::fs::read_to_string(&csv).unwrap();
    let header: Vec<&str> = table.lines().next().unwrap().split(',').collect();
    let column = header
        .iter()
        .position(|name| *name == "landing distance [m]")
        .unwrap();
    for line in table.lines().skip(1) {
        assert!(!line.split(',').nth(column).unwrap().is_empty(), "{line}");
    }

    // With the parachute's lag scattered, the flights whose drawn lag is positive fail by name.
    let document = json(
        &[
            "mc",
            &path,
            "--runs",
            "12",
            "--seed",
            "3",
            "--deployment-lag-sd",
            "0.5",
        ],
        0,
        "mc.schema.json",
    );
    let dispersion = Dispersion {
        deployment_lag_sd_s: 0.5,
        ..Dispersion::default()
    };
    let run = MonteCarlo::new(inputs, dispersion).unwrap().run(3, 12);
    let failed = run.failed().count();
    assert!(0 < failed && failed < 12, "{failed}");
    assert_eq!(document["failed"]["count"], failed);
    let reasons = document["failed"]["reasons"].to_string();
    assert!(
        reasons.contains("time of a split with nothing left to burn, before apogee"),
        "{reasons}"
    );
}

/// M4.6a's bullet: failed flights are counted, never dropped: each reason printed with how many
/// failed so, the spreads counting them as tried, and their rows in the CSV marked failed.
#[test]
fn mc_counts_and_prints_its_failed_flights() {
    let folder = tempfile::tempdir().unwrap();
    let csv = folder.path().join("runs.csv");
    let csv_arg = csv.to_string_lossy().into_owned();
    let design = repo_file(PROBE);
    // A 60% scatter in the mass draws a negative mass now and then.
    let args = [
        "mc",
        &design,
        "--motor",
        "H54",
        "--runs",
        "20",
        "--mass-sd",
        "0.6",
        "--export",
        &csv_arg,
    ];
    let document = json(&args, 0, "mc.schema.json");
    let failed = document["failed"]["count"].as_u64().unwrap();
    assert!(failed > 0 && failed < 20, "{failed}");
    assert_eq!(document["flown"].as_u64().unwrap() + failed, 20);
    assert_eq!(document["apogee_m"]["attempted"], 20);
    assert_eq!(document["apogee_m"]["count"].as_u64().unwrap(), 20 - failed);
    let reasons = document["failed"]["reasons"].as_array().unwrap();
    let counted: u64 = reasons.iter().map(|r| r["count"].as_u64().unwrap()).sum();
    assert_eq!(counted, failed);
    let rows = std::fs::read_to_string(&csv).unwrap();
    assert_eq!(
        rows.lines()
            .filter(|line| line.contains(",failed,"))
            .count() as u64,
        failed
    );
    let first = reasons[0]["first_index"].as_u64().unwrap();
    assert!(
        rows.lines()
            .nth(first as usize + 1)
            .unwrap()
            .starts_with(&format!("{first},failed,flight,")),
        "{rows}"
    );
    let text = streams(&args[..args.len() - 2]);
    assert!(
        text.out
            .contains(&format!("20 simulated flights, seed 0: {failed} failed")),
        "{}",
        text.out
    );
    // The failed flights' reasons, on standard error.
    assert!(text.err.contains("warning: "), "{}", text.err);
    assert!(text.err.contains("mass"), "{}", text.err);
}

/// What `hpr mc` refuses before it flies, each by its option: a run of no flights or too many, a
/// negative or non-finite standard deviation, an export that isn't a CSV, in a missing folder or
/// over the design; and a run with nothing scattered says so.
#[test]
fn mc_refuses_bad_options_by_name() {
    let design = repo_file(PROBE);
    let refused = |extra: &[&str], words: &str| {
        let mut args = vec!["mc", design.as_str(), "--motor", "H54"];
        args.extend(extra);
        let document = json_error(&args, 1, "input");
        let message = document["error"]["message"].as_str().unwrap();
        assert!(message.contains(words), "{extra:?}: {message}");
    };
    refused(
        &["--runs", "0"],
        "--runs 0: a run flies 1 to 100000 flights",
    );
    refused(&["--runs", "100001"], "--runs 100001");
    refused(&["--mass-sd=-0.1"], "--mass-sd -0.1: a standard deviation");
    refused(&["--heading-sd", "NaN"], "--heading-sd NaN");
    refused(&["--export", "runs.txt"], "writes a .csv file");
    refused(
        &["--export", "no/such/folder/runs.csv"],
        "there is no folder",
    );
    let folder = tempfile::tempdir().unwrap();
    let copy = folder.path().join("probe.csv");
    std::fs::copy(repo_file(PROBE), &copy).unwrap();
    let copy = copy.to_string_lossy().into_owned();
    let made = folder.path().join("made.csv");
    std::fs::create_dir(&made).unwrap();
    refused(
        &["--export", &made.to_string_lossy()],
        "a folder, so hpr mc --export",
    );
    let document = json_error(&["mc", &copy, "--export", &copy], 1, "input");
    assert!(
        document["error"]["message"]
            .as_str()
            .unwrap()
            .contains("this run reads that file"),
        "{document:#}"
    );
    let document = json(
        &["mc", &design, "--motor", "H54", "--runs", "2"],
        0,
        "mc.schema.json",
    );
    let notes = document["notes"].as_array().unwrap();
    assert!(
        notes
            .iter()
            .any(|note| note.as_str().unwrap().starts_with("no input is scattered")),
        "{notes:?}"
    );
    // With nothing scattered, every flight is the nominal one.
    assert_eq!(
        document["apogee_m"]["min"], document["apogee_m"]["max"],
        "{document:#}"
    );
    assert_eq!(document["apogee_m"]["min"], document["nominal"]["apogee_m"]);
}

/// #242: a rocket off a rail 10° above the horizon records its apogee once. The apogee watch
/// stays armed after the apogee and found the same root again about 5e-13 s later.
#[test]
fn sim_records_one_apogee_off_a_near_horizontal_rail() {
    let design = repo_file(PROBE);
    let document = json(
        &["sim", &design, "--motor", "H54", "--inclination", "10"],
        0,
        "sim.schema.json",
    );
    let apogees = document["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|event| event["kind"] == "apogee")
        .count();
    assert_eq!(apogees, 1, "{}", document["events"]);
}
