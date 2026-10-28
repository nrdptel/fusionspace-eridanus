//! Every refusal says what to do next (#410): the product system's `cli.md` (*Errors*) asks for
//! the kind, what happened, where, and what to do. A refusal can't be built without a `help:`
//! line (`Failure` has no variant for one); these run a refusal of each kind the commands make,
//! offline and with an empty cache, and hold each to its `help:` line, as text and as JSON.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests write files for the commands to refuse; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::{Path, PathBuf};
use std::process::Output;

use serde_json::Value;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A path under the repository, as a string.
fn repo(path: &str) -> String {
    root().join(path).to_str().unwrap().to_owned()
}

/// Runs `hpr` with `args` from `folder`, offline with `folder`'s `cache` as its cache, and none
/// of the variables that ask for color.
fn hpr(args: &[String], folder: &Path) -> Output {
    assert_cmd::cargo::cargo_bin_cmd!("hpr")
        .args(args)
        .current_dir(folder)
        .env("HPR_CACHE_DIR", folder.join("cache"))
        .env("HPR_OFFLINE", "1")
        .env_remove("NO_COLOR")
        .env_remove("FORCE_COLOR")
        .env_remove("CLICOLOR_FORCE")
        .output()
        .unwrap()
}

/// A refusal: the command line, words one of its `help:` lines holds, and its exit status.
struct Case {
    args: Vec<String>,
    help: &'static str,
    status: i32,
}

/// A refused input, status 1.
fn case(args: &[&str], help: &'static str) -> Case {
    Case {
        args: args.iter().map(|&arg| arg.to_owned()).collect(),
        help,
        status: 1,
    }
}

/// The refusals, one of each kind, with files under `folder` the cases write.
fn cases(folder: &Path) -> Vec<Case> {
    let design = repo("validation/fixtures/ork/pod-flights/pods-none.ork");
    let guide = repo("validation/fixtures/ork/guides/level-1.ork");
    let file = |name: &str| folder.join(name).to_str().unwrap().to_owned();
    let bad_eng = file("bad.eng");
    std::fs::write(&bad_eng, "hello\n").unwrap();
    // A profile of two levels, 1,000 to 3,000 m above sea level, as `hpr weather` writes one.
    let profile = file("profile.json");
    std::fs::write(
        &profile,
        r#"{"levels": [
            {"height_msl_m": 1000.0, "temperature_k": 290.0, "pressure_pa": 90000.0,
             "wind_speed_m_s": 3.0, "wind_direction_from_rad": 4.7},
            {"height_msl_m": 3000.0, "temperature_k": 277.0, "pressure_pa": 70000.0,
             "wind_speed_m_s": 9.0, "wind_direction_from_rad": 4.8}
        ], "latitude_rad": 0.5, "wind_interpolation": "speed_direction"}"#,
    )
    .unwrap();
    std::fs::write(file("rocket.rkt"), "<RockSimDocument/>\n").unwrap();
    std::fs::write(file("empty.csv"), "").unwrap();
    let missing = folder.join("no-such-folder");
    let missing = |name: &str| missing.join(name).to_str().unwrap().to_owned();
    vec![
        // A command whose milestone hasn't come, status 3.
        Case {
            status: 3,
            ..case(
                &["optimize", "design.ork"],
                "follow M6.3 on the roadmap: https://hpr.fusionspace.co/decisions-and-roadmap.html#m6-3",
            )
        },
        // The issue's own: a forecast offline with nothing cached.
        case(
            &[
                "weather",
                "gfs",
                "--latitude",
                "40.865",
                "--longitude",
                "-119.063",
                "--cycle",
                "2026-09-26T00Z",
                "--hour",
                "12",
                "--offline",
            ],
            "without --offline",
        ),
        // A motor file given where the design goes.
        case(&["sim", &bad_eng], "is a motor file: give it with --motor"),
        // A weather profile that isn't one, and one whose heights the site lies outside.
        case(
            &["sim", &guide, "--weather", &bad_eng],
            "give the JSON file `hpr weather` writes with --output",
        ),
        case(
            &["sim", &guide, "--weather", &profile, "--elevation", "0"],
            "give the site's elevation within them",
        ),
        // A design neither its own motors nor --motor can fly: the file must change.
        case(
            &[
                "sim",
                &repo("validation/fixtures/ork/loft-demo/demo-quirks.ork"),
                "--motor",
                "G84",
                "--offline",
            ],
            "neither the file's motors nor --motor can fly it as it is",
        ),
        // A run asked for no flights.
        case(
            &["mc", &guide, "--runs", "0"],
            "give --runs a whole number from 1 to 100000",
        ),
        case(&["mc", &guide, "--mass-sd=-1"], "give --mass-sd 0 or more"),
        // A negative number typed after a space is the option's value, refused by its own rule
        // rather than read as an option.
        case(
            &["mc", &guide, "--mass-sd", "-1"],
            "give --mass-sd 0 or more",
        ),
        // An export named for a format it isn't.
        case(
            &["mc", &guide, "--export", &file("out.txt")],
            "end the name in .csv",
        ),
        // A design converted to a motor file.
        case(
            &["convert", &guide, &file("out.eng")],
            "end the output's name in .ork, .hpr or .hprz",
        ),
        // A motor written into a folder that isn't there.
        case(&["convert", "E26W", &missing("out.eng")], "make the folder"),
        // A file that isn't a flight log.
        case(
            &["analyze", &repo("Cargo.toml")],
            "another logger's table is read by a column mapping (--mapping)",
        ),
        // An empty log, which no mapping or layout can read.
        case(
            &["analyze", &file("empty.csv")],
            "download the log from the logger again with its maker's software: this file holds \
             no readings",
        ),
        // A forecast hour past what the model's run reaches.
        case(
            &[
                "weather",
                "rap",
                "--latitude",
                "40.865",
                "--longitude",
                "-119.063",
                "--cycle",
                "2026-09-26T00Z",
                "--hour",
                "40",
                "--offline",
            ],
            "and to 51 from its 03Z, 09Z, 15Z and 21Z runs",
        ),
        // A sounding station's number mistyped.
        case(
            &[
                "weather",
                "wyoming",
                "--station",
                "7236-4",
                "--time",
                "2026-09-26T00Z",
                "--offline",
            ],
            "letters and digits only, such as 72364",
        ),
        // A file that isn't ERA5's.
        case(
            &[
                "weather",
                "era5",
                &repo("crates/hpr-cli/Cargo.toml"),
                "--latitude",
                "1",
                "--longitude",
                "1",
                "--time",
                "2020-01-01T00Z",
            ],
            "https://hpr.fusionspace.co/format/era5.html",
        ),
        // A motor to fetch, offline with nothing cached.
        case(
            &["motors", "fetch", "H128W", "--offline"],
            "with a network connection, `hpr motors fetch H128W`",
        ),
        // A motor file that isn't one, and a list that isn't the motor finder's.
        case(
            &["motors", "show", &bad_eng],
            "https://hpr.fusionspace.co/format/eng.html",
        ),
        case(&["motors", "list", "--diameter", "-3"], "such as 38 or 54"),
        case(
            &[
                "motors",
                "search",
                "--from",
                &repo("crates/hpr-cli/Cargo.toml"),
            ],
            "motors.json or in-stock.json",
        ),
        // A flight's files named for formats they aren't, or one name given twice.
        case(
            &[
                "sim",
                &design,
                "--motor",
                "H54",
                "--export",
                &file("flight.txt"),
            ],
            "flight.csv",
        ),
        case(
            &[
                "sim",
                &design,
                "--motor",
                "H54",
                "--plot",
                &file("flight.png"),
            ],
            "flight.svg",
        ),
        case(
            &[
                "sim",
                &design,
                "--motor",
                "H54",
                "--export",
                &file("a.csv"),
                "--export",
                &file("a.csv"),
            ],
            "give each --export and --plot a file of its own",
        ),
        case(
            &["sim", &design, "--motor", "H54", "--interval", "0.0001"],
            "such as `--interval 0.01`",
        ),
        // A design in a format HPR Sim doesn't read, and a motor that matches several.
        case(
            &["sim", &file("rocket.rkt")],
            "open the design in OpenRocket and save it as",
        ),
        case(
            &["sim", &design, "--motor", "I175"],
            "give --motor one of those names, each of which names one motor alone",
        ),
        // A file that can't be written: its folder isn't there.
        case(
            &[
                "sim",
                &design,
                "--motor",
                "H54",
                "--plot",
                &missing("f.svg"),
            ],
            "make the folder",
        ),
    ]
}

/// The lines a refusal writes on standard error, and its status.
fn refused(case: &Case, folder: &Path) -> (Option<i32>, String) {
    let output = hpr(&case.args, folder);
    assert!(output.stdout.is_empty(), "{:?}", case.args);
    (
        output.status.code(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// Each refusal ends with a `help:` line holding its words, after an `error:` line.
#[test]
fn every_refusal_says_what_to_do() {
    let folder = tempfile::tempdir().unwrap();
    for case in cases(folder.path()) {
        let (status, text) = refused(&case, folder.path());
        assert_eq!(status, Some(case.status), "{:?}: {text}", case.args);
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines[0].starts_with("error: "), "{:?}: {text}", case.args);
        let last = lines.last().unwrap();
        assert!(last.starts_with("help: "), "{:?}: {text}", case.args);
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("help: ") && line.contains(case.help)),
            "{:?}: {text}",
            case.args
        );
    }
}

/// With `--json`, the same refusals carry their help in the error document, and standard error
/// stays empty.
#[test]
fn every_refusal_says_what_to_do_in_json() {
    let folder = tempfile::tempdir().unwrap();
    for mut case in cases(folder.path()) {
        case.args.push("--json".to_owned());
        let output = hpr(&case.args, folder.path());
        assert_eq!(output.status.code(), Some(case.status), "{:?}", case.args);
        assert!(output.stderr.is_empty(), "{:?}", case.args);
        let document: Value = serde_json::from_slice(&output.stdout).unwrap();
        let help = document["error"]["help"].as_array().unwrap();
        assert!(
            help.iter()
                .any(|line| line.as_str().unwrap().contains(case.help)),
            "{:?}: {document:#}",
            case.args
        );
    }
}
