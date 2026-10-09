//! US units in brackets after the SI (ADR-164 §6, ADR-210; issue #378): `hpr sim` and `hpr mc`
//! give feet, feet per second and miles per hour beside every height, distance, speed and wind
//! they print, their notes included, and inches beside the stations the static margin came from;
//! `hpr motors show` reads the designation as the product system's `data.md` (*Units for
//! rocketry*) asks. Each test pins the exact lines for the level 1 guide's rocket, run as a user
//! runs the binary, so a line printed in SI alone fails it; `--json` stays SI, its names carrying
//! the unit.

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

/// The repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A file of the repository, by its path from the root.
fn repo_file(path: &str) -> String {
    root().join(path).to_string_lossy().into_owned()
}

/// Runs `hpr` with `args`, offline with an empty cache and no color, expects success, and
/// returns standard output.
fn out(args: &[&str]) -> String {
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
    String::from_utf8(output.stdout).unwrap()
}

/// Runs `hpr` as [`out`] does and returns standard error, where the notes and warnings go.
fn err(args: &[&str]) -> String {
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
    String::from_utf8(output.stderr).unwrap()
}

/// Asserts that `text` holds each of `lines` as a whole line.
fn holds_lines(text: &str, lines: &[&str]) {
    for line in lines {
        assert!(
            text.lines().any(|printed| printed == *line),
            "no line {line:?} in:\n{text}"
        );
    }
}

/// `hpr sim`'s summary, its launch line, its event table and its landing give feet, feet per
/// second and miles per hour in brackets after the SI, in a wind off a leaning rail.
#[test]
fn sim_gives_us_units_in_brackets() {
    let text = out(&[
        "sim",
        &repo_file(LEVEL_1),
        "--wind",
        "6",
        "--inclination",
        "84",
    ]);
    holds_lines(
        &text,
        &[
            "apogee                1033.4 m (3390 ft) above the site at 11.47 s",
            "rail exit speed       26.3 m/s (86 ft/s)",
            "descent               4.4 m/s (15 ft/s) at landing under `Parachute`",
            "launched at 0° N, 0° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) rail 84° \
             above the horizon, leaning toward 000° T, in a 6 m/s (13 mph) wind from 000° T",
            "event                   time                 height                    speed",
            "liftoff               0.00 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)",
            "rail exit             0.12 s      1.9 m      (6 ft)    26.3 m/s    (86 ft/s)",
            "burnout               2.14 s    398.6 m   (1308 ft)   239.5 m/s   (786 ft/s)",
            "apogee               11.47 s   1033.4 m   (3390 ft)     7.8 m/s    (25 ft/s)",
            "ground hit          238.98 s      0.0 m      (0 ft)     7.5 m/s    (24 ft/s)",
            "top speed             284.7 m/s (934 ft/s) at 1.73 s",
            "landing               1130.4 m (3709 ft) from the pad at 238.98 s, at 7.5 m/s (24 ft/s)",
        ],
    );
}

/// The static margin comes with the CG and CP stations it was taken from, in meters and inches,
/// the stations `--json` gives.
#[test]
fn sim_gives_the_cg_and_cp_of_its_static_margin() {
    let design = repo_file(LEVEL_1);
    let text = out(&["sim", &design]);
    holds_lines(
        &text,
        &[
            "static margin         1.70 calibres off the rail; least 1.70 calibres, at 0.12 s, \
             before apogee",
            "CG and CP             0.811 m (31.9 in) and 0.923 m (36.3 in) aft of the nose tip, \
             off the rail",
        ],
    );
    // The lines come straight after the margin's, before the apogee's.
    let lead: Vec<&str> = text
        .lines()
        .skip_while(|line| !line.starts_with("static margin"))
        .take(3)
        .collect();
    assert!(lead[1].starts_with("CG and CP "), "{lead:?}");
    assert!(lead[2].starts_with("apogee "), "{lead:?}");
    // The stations are the JSON's, which stays SI.
    let document: Value = serde_json::from_str(&out(&["sim", &design, "--json"])).unwrap();
    let stability = &document["summary"]["rail_exit_stability"];
    let cg_m = stability["cg_station_m"].as_f64().unwrap();
    let cp_m = stability["static_margin"]["cp_station_m"].as_f64().unwrap();
    assert_eq!(
        (format!("{cg_m:.3}"), format!("{cp_m:.3}")),
        ("0.811".to_owned(), "0.923".to_owned())
    );
    let json = document.to_string();
    assert!(!json.contains("_ft\"") && !json.contains("(ft"), "{json}");
}

/// `hpr mc`'s spreads give each height and distance a row in feet under its row in meters, and
/// the landing's center and ellipses give feet in brackets: exactly for flights all flown
/// nominal in a wind, and cell by cell for scattered ones.
#[test]
fn mc_gives_feet_beside_meters() {
    let design = repo_file(LEVEL_1);
    let text = out(&["mc", &design, "--wind", "6", "--runs", "20"]);
    holds_lines(
        &text,
        &[
            "                        nominal     mean  std dev       5%   median      95%",
            "apogee (m AGL)           1058.3   1058.3      0.0   1058.3   1058.3   1058.3",
            "apogee (ft AGL)            3472     3472        0     3472     3472     3472",
            "landing distance (m)     1325.1   1325.1      0.0   1325.1   1325.1   1325.1",
            "landing distance (ft)      4347     4347        0     4347     4347     4347",
            "20 of 20 flights landed, centered 0.7 m (2 ft) west and 1325.1 m (4347 ft) south of \
             the pad",
            "landing ellipse                   semi-major            semi-minor  heading   flights inside",
            "50%                             0.0 m (0 ft)          0.0 m (0 ft)   090° T           100.0%",
        ],
    );
    // Scattered: each foot cell is its meter cell's, in feet, to the meter cell's rounding.
    let text = out(&[
        "mc",
        &design,
        "--wind",
        "6",
        "--wind-sd",
        "0.2",
        "--runs",
        "20",
    ]);
    let cells = |name: &str| -> Vec<f64> {
        let line = text
            .lines()
            .find(|line| line.starts_with(name))
            .unwrap_or_else(|| panic!("{name}: {text}"));
        line[name.len()..]
            .split_whitespace()
            .map(|cell| cell.parse().unwrap())
            .collect()
    };
    for (name, datum) in [("apogee", " AGL"), ("landing distance", "")] {
        let (m, ft) = (
            cells(&format!("{name} (m{datum})")),
            cells(&format!("{name} (ft{datum})")),
        );
        assert_eq!((m.len(), ft.len()), (6, 6), "{text}");
        for (m, ft) in m.iter().zip(&ft) {
            assert!(
                (m / 0.3048 - ft).abs() <= 0.5 + 0.05 / 0.3048,
                "{m} m, {ft} ft"
            );
        }
        assert!(m[2] > 0.0, "{name}: no spread in\n{text}");
    }
    let ellipse = text
        .lines()
        .find(|line| line.starts_with("50% "))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(
        ellipse.contains(" m (") && ellipse.contains(" ft) "),
        "{ellipse}"
    );
}

/// `hpr mc`'s landing center and ellipses print their meters to a tenth, as everywhere else, so
/// the feet in brackets are never finer than the meters: a semi-minor axis of a fifth of a meter
/// reads `0.2 m (1 ft)`, not `0 m (1 ft)`.
#[test]
fn mc_landing_meters_carry_a_tenth() {
    let text = out(&[
        "mc",
        &repo_file(LEVEL_1),
        "--runs",
        "30",
        "--wind",
        "6",
        "--wind-sd",
        "0.3",
        "--mass-sd",
        "0.05",
        "--drag-sd",
        "0.1",
        "--impulse-sd",
        "0.05",
    ]);
    holds_lines(
        &text,
        &[
            "30 of 30 flights landed, centered 0.7 m (2 ft) west and 1355.7 m (4448 ft) south of \
             the pad",
            "landing ellipse                   semi-major            semi-minor  heading   flights inside",
            "50%                        510.4 m (1674 ft)          0.1 m (0 ft)   180° T            63.3%",
            "95%                       1061.0 m (3481 ft)          0.2 m (1 ft)   180° T            90.0%",
            "95%, the next flight      1159.1 m (3803 ft)          0.2 m (1 ft)   180° T            93.3%",
        ],
    );
}

/// `hpr sim`'s notes give feet in brackets too: a device set to open above the apogee says its
/// height in meters and feet.
#[test]
fn sim_notes_give_feet() {
    let folder = tempfile::tempdir().unwrap();
    let design = folder.path().join("high.ork");
    let text = std::fs::read_to_string(root().join(LEVEL_1)).unwrap();
    let ejection = "<deployevent>ejection</deployevent>";
    assert_eq!(text.matches(ejection).count(), 1);
    std::fs::write(
        &design,
        text.replace(
            ejection,
            "<deployevent>altitude</deployevent><deployaltitude>2000</deployaltitude>",
        ),
    )
    .unwrap();
    holds_lines(
        &err(&["sim", &design.to_string_lossy()]),
        &[
            "note: `Parachute` is set to open 2000 m (6562 ft) above the site, above the apogee, \
           so it opened at apogee; OpenRocket's flight of such a file never opens it",
        ],
    );
}

/// `hpr motors show` gives the class's range, the nominal thrust beside the measured where they
/// differ, the propellant its letter stands for, and masses and sizes in US units too.
#[test]
fn motors_show_reads_the_designation() {
    let text = out(&["motors", "show", "H170M"]);
    holds_lines(
        &text,
        &[
            "  impulse class    H (160.01–320 N·s)",
            "  total impulse    318.0 N·s",
            "  average thrust   165.4 N measured; 170 N nominal, as the designation names it",
            "  propellant       M, the maker's code for Metalstorm, as ThrustCurve.org names it",
            "  propellant mass  182.5 g (6.4 oz) of 330.0 g (11.6 oz) loaded",
            "  casing           38 mm (1.50 in) across, 191 mm (7.5 in) long",
        ],
    );
    // A motor file names no propellant: its code is shown as the maker's, with no meaning.
    let file = out(&[
        "motors",
        "show",
        &repo_file("crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000724.eng"),
    ]);
    holds_lines(
        &file,
        &[
            "  impulse class    G (80.01–160 N·s)",
            "  average thrust   84.2 N measured, as the designation names it",
            "  propellant       GR, the maker's code; the motor file doesn't name the propellant",
        ],
    );
}

/// `hpr sim --plot`'s panels each give their US unit over a right-hand scale.
#[test]
fn the_plot_gives_both_units() {
    let folder = tempfile::tempdir().unwrap();
    let svg = folder.path().join("level-1.svg");
    out(&["sim", &repo_file(LEVEL_1), "--plot", &svg.to_string_lossy()]);
    let figure = std::fs::read_to_string(&svg).unwrap();
    // The panels' titles, left and right, and the event table's heads, in meters and in feet.
    for (title, times) in [
        (">ALTITUDE · m AGL<", 2),
        (">ft AGL<", 1),
        (">SPEED · m/s<", 1),
        (">ft/s<", 1),
        (">ACCELERATION · m/s²<", 1),
        (">g<", 1),
        (">ALTITUDE · ft AGL<", 1),
    ] {
        assert_eq!(figure.matches(title).count(), times, "{title}");
    }
    assert!(
        figure.contains(
            "Apogee 1,065.1\u{a0}m (3,494\u{a0}ft) above the launch site at 11.62\u{a0}s; top \
                         speed 285.6\u{a0}m/s (937\u{a0}ft/s) at 1.73\u{a0}s."
        ),
        "{figure}"
    );
}

/// `hpr motors list` gives each motor's diameter and length in inches in brackets after the
/// millimeters, to the places `hpr motors show` gives them (issue #392, ADR-213), and its
/// columns line up on both parts.
#[test]
fn motors_list_gives_inches() {
    let text = out(&["motors", "list"]);
    holds_lines(
        &text,
        &[
            "designation   maker     class  dia mm (in)   len mm (in)  impulse N·s   avg N  burn s  \
             delays",
            "B4            Quest     B        18 (0.71)     80  (3.1)          4.9     4.4    1.11  \
             4-6",
            "H170M         AeroTech  H        38 (1.50)    191  (7.5)        318.0   165.4    1.92  \
             2,4,6,8,10,14",
            "K400C         AeroTech  K        54 (2.13)    359 (14.1)       1307.3   409.8    3.19  14",
        ],
    );
    let json = out(&["motors", "list", "--json"]);
    let document: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(document["motors"][0]["diameter_mm"], 18.0);
}

/// `hpr motors search` gives each motor's diameter in inches in brackets after the millimeters.
#[test]
fn motors_search_gives_inches() {
    let from = repo_file("crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json");
    let text = out(&[
        "motors",
        "search",
        "--in-stock",
        "--class",
        "L",
        "--max-price",
        "300",
        "--from",
        &from,
    ]);
    holds_lines(
        &text,
        &[
            "designation  maker     class  dia mm (in)  impulse N·s   avg N  burn s  each $  pack  \
             vendor",
        ],
    );
    let rows: Vec<&str> = text.lines().filter(|l| l.contains(" L ")).collect();
    assert!(!rows.is_empty(), "{text}");
    for row in rows {
        assert!(row.contains("        75 (2.95)  "), "{row}");
    }
}
