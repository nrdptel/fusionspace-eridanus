//! Where a result comes from, carried with it (#380, ADR-214): the product system's
//! `principles.md` §2 (*Show the working*) and `data.md` (*Files and exports*, *Copying and typing
//! numbers*). Each export carries the bundled motor catalog's as-of date and how far to trust it;
//! the text names the catalog's date beside a motor taken from it; a CSV header gives each unit in
//! brackets; and every JSON document `hpr` writes is ASCII.

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

use hpr::hpr_motor::Catalog;
use serde_json::Value;

/// A design that flies offline with `--motor H54`.
const PROBE: &str = "validation/fixtures/ork/pod-flights/pods-none.ork";

/// The repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Runs `hpr` offline with an empty cache, its streams piped.
fn run(args: &[&str]) -> Output {
    let cache = tempfile::tempdir().unwrap();
    let output = assert_cmd::cargo::cargo_bin_cmd!("hpr")
        .args(args)
        .env("HPR_CACHE_DIR", cache.path())
        .env("HPR_OFFLINE", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
    output
}

/// The bundled catalog's as-of date: the day its curve files were downloaded.
fn as_of() -> String {
    Catalog::bundled().unwrap().snapshot.captured
}

/// A JSON file `hpr` wrote, held to be ASCII, then read.
fn ascii_json(text: &str, what: &str) -> Value {
    assert!(text.is_ascii(), "{what} isn't ASCII: {text:.400}");
    serde_json::from_str(text).unwrap()
}

/// `hpr sim`'s JSON recording and the sidecar beside its CSV carry the bundled catalog's as-of
/// date and the trust note `hpr sim --json` has, as `kind` and `trust`; `hpr mc`'s sidecar
/// carries its own note. Before #380 none had a date or a note.
#[test]
fn every_export_carries_the_catalogs_date_and_how_far_to_trust_it() {
    let folder = tempfile::tempdir().unwrap();
    let path = |name: &str| folder.path().join(name).to_string_lossy().into_owned();
    let probe = root().join(PROBE).to_string_lossy().into_owned();
    let flight = &["sim", &probe, "--motor", "H54"];
    let printed = ascii_json(
        &String::from_utf8(run(&[&flight[..], &["--json"]].concat()).stdout).unwrap(),
        "hpr sim --json",
    );
    assert_eq!(printed["kind"], "simulated");
    let trust = printed["trust"].as_str().unwrap();
    assert!(trust.starts_with("How far to trust it."), "{trust}");
    run(&[
        &flight[..],
        &["--export", &path("f.csv"), "--export", &path("f.json")],
    ]
    .concat());
    let as_of = as_of();
    for name in ["f.meta.json", "f.json"] {
        let document = ascii_json(&std::fs::read_to_string(path(name)).unwrap(), name);
        assert_eq!(document["catalog_as_of"], as_of.as_str(), "{name}");
        assert_eq!(document["kind"], "simulated", "{name}");
        assert_eq!(document["trust"], trust, "{name}");
        assert_eq!(document["design"], "pods-none.ork", "{name}");
        assert_eq!(
            document["configuration"], printed["design"]["configuration_label"],
            "{name}"
        );
    }
    let recording = ascii_json(&std::fs::read_to_string(path("f.json")).unwrap(), "f.json");
    let keys: Vec<&String> = recording.as_object().unwrap().keys().collect();
    assert_eq!(keys.len(), 8, "{keys:?}");

    let runs = &[
        "mc",
        &probe,
        "--motor",
        "H54",
        "--runs",
        "4",
        "--impulse-sd",
        "0.03",
    ];
    let printed = ascii_json(
        &String::from_utf8(run(&[&runs[..], &["--json"]].concat()).stdout).unwrap(),
        "hpr mc --json",
    );
    run(&[&runs[..], &["--export", &path("runs.csv")]].concat());
    let meta = ascii_json(
        &std::fs::read_to_string(path("runs.meta.json")).unwrap(),
        "runs.meta.json",
    );
    assert_eq!(meta["catalog_as_of"], as_of.as_str());
    assert_eq!(meta["kind"], "simulated");
    assert_eq!(meta["trust"], printed["trust"]);
    assert_ne!(meta["trust"], trust);
}

/// The text names the catalog's as-of date beside a motor taken from it, as JSON does.
#[test]
fn the_text_dates_a_motor_from_the_bundled_catalog() {
    let probe = root().join(PROBE).to_string_lossy().into_owned();
    let text = String::from_utf8(run(&["sim", &probe, "--motor", "H54"]).stdout).unwrap();
    let as_of = as_of();
    assert!(
        text.contains(&format!(
            "168H54-10A (from the bundled catalog, as of {as_of})"
        )),
        "{text}"
    );
    let document = ascii_json(
        &String::from_utf8(run(&["sim", &probe, "--motor", "H54", "--json"]).stdout).unwrap(),
        "hpr sim --json",
    );
    assert_eq!(
        document["motors"][0]["source"],
        serde_json::json!({"kind": "catalog", "as_of": as_of})
    );
}

/// Both CSV files give each column's unit in brackets after its words, in ASCII, as `data.md`
/// writes a header (`time [s]`); before #380 the header was the names, `time_s`.
#[test]
fn a_csv_header_gives_each_unit_in_brackets() {
    let folder = tempfile::tempdir().unwrap();
    let path = |name: &str| folder.path().join(name).to_string_lossy().into_owned();
    let probe = root().join(PROBE).to_string_lossy().into_owned();
    run(&["sim", &probe, "--motor", "H54", "--export", &path("f.csv")]);
    run(&[
        "mc",
        &probe,
        "--motor",
        "H54",
        "--runs",
        "2",
        "--export",
        &path("runs.csv"),
    ]);
    // The columns with no unit: numbers of their own kind, scales, words and counts.
    let unitless = |cell: &str| {
        cell == "mach"
            || cell.starts_with("attitude ")
            || cell.ends_with(" coefficient")
            || cell.ends_with(" scale")
            || cell.ends_with(" mach")
            || [
                "index",
                "outcome",
                "failed at",
                "reason",
                "termination",
                "flags",
            ]
            .contains(&cell)
    };
    for (name, first) in [("f.csv", "time [s]"), ("runs.csv", "index")] {
        let text = std::fs::read_to_string(path(name)).unwrap();
        let header = text.lines().next().unwrap();
        assert!(header.is_ascii(), "{name}: {header}");
        let cells: Vec<&str> = header.split(',').collect();
        assert_eq!(cells[0], first, "{name}");
        for cell in &cells {
            assert!(!cell.contains('_'), "{name}: {cell}");
            let bracketed = cell.ends_with(']') && cell.contains(" [");
            assert!(bracketed || unitless(cell), "{name}: {cell}");
        }
    }
}

/// Every JSON document `hpr` writes is ASCII: `--json`, the recordings and the sidecars, even
/// with the designation's middle dots and a design whose file name has a letter outside ASCII,
/// each written as its escape and read back as itself.
#[test]
fn every_json_document_is_ascii() {
    let folder = tempfile::tempdir().unwrap();
    let path = |name: &str| folder.path().join(name).to_string_lossy().into_owned();
    let design = path("Mjölnir.ork");
    std::fs::copy(root().join(PROBE), &design).unwrap();
    let printed = String::from_utf8(
        run(&[
            "sim",
            &design,
            "--motor",
            "H54",
            "--json",
            "--export",
            &path("f.csv"),
            "--export",
            &path("f.json"),
            "--export",
            &path("f.geojson"),
        ])
        .stdout,
    )
    .unwrap();
    let printed = ascii_json(&printed, "hpr sim --json");
    assert_eq!(
        printed["tool"]["designation"],
        "FS-ACHERNAR · SW · TOOL 001"
    );
    assert_eq!(printed["design"]["file"], "Mjölnir.ork");
    for name in ["f.meta.json", "f.json", "f.geojson"] {
        let text = std::fs::read_to_string(path(name)).unwrap();
        assert!(text.contains("FS-ACHERNAR \\u00b7 SW"), "{name}");
        let document = ascii_json(&text, name);
        assert_eq!(
            document["tool"]["designation"], "FS-ACHERNAR · SW · TOOL 001",
            "{name}"
        );
    }
    let meta = ascii_json(
        &std::fs::read_to_string(path("f.meta.json")).unwrap(),
        "f.meta.json",
    );
    assert_eq!(meta["design"], "Mjölnir.ork");
}
