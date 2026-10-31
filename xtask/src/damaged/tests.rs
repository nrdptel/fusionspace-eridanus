//! The list of readers of outside bytes, and the probes that walk it (M0.15).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use hpr_flightdata::import::FORMATS;
use hpr_flightdata::log::LogFormat;
use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use super::{Ended, evenly, probe, probe_within};
use readers::{EXTRA_READERS, NO_OUTSIDE_BYTES, READERS};

mod readers;

/// The crates M0.15 covers, whose sources the list is checked against.
const CRATES: [&str; 5] = [
    "hpr-io",
    "hpr-flightdata",
    "hpr-motor",
    "hpr-format",
    "hpr-net",
];

/// Cuts of each sample, and of its bytes changed.
const PLACES: usize = 64;
/// Cases of arbitrary bytes for each reader.
const ARBITRARY_CASES: u32 = 256;
/// The longest arbitrary input.
const ARBITRARY_LEN: usize = 4096;

/// The end of a failure's line that a hang gives.
const HUNG: &str = ": still reading at the limit";

/// Where a reader's real input comes from.
#[derive(Clone, Copy)]
enum Sample {
    /// A committed public file, from the workspace's root.
    File(&'static str),
    /// Bytes built in the test, with why no committed file serves.
    Built(fn() -> Vec<u8>, &'static str),
}

/// One reader of outside bytes.
struct Reader {
    /// The crate, and its file under `src/`, that hold the function.
    krate: &'static str,
    file: &'static str,
    /// The function's name, as the source spells it.
    function: &'static str,
    /// The formats of [`FORMATS`] this row stands for, read through this function.
    formats: &'static [LogFormat],
    sample: Sample,
    /// Reads the bytes, and whatever a caller would read next from what it gives; says if it gave
    /// a value.
    read: fn(&[u8]) -> bool,
}

/// A function named `read…` or `parse…` that takes no outside bytes, and why.
struct NoOutsideBytes {
    krate: &'static str,
    file: &'static str,
    function: &'static str,
    why: &'static str,
}

fn text(bytes: &[u8]) -> std::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

/// `bytes` with the byte at `at` changed: all its bits at every other place, its lowest elsewhere,
/// so a length field moves far and a digit stays a digit.
fn changed(bytes: &[u8], at: usize, place: usize) -> Vec<u8> {
    let mut copy = bytes.to_vec();
    if let Some(byte) = copy.get_mut(at) {
        *byte ^= if place.is_multiple_of(2) { 0xFF } else { 0x01 };
    }
    copy
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn sample_bytes(sample: Sample) -> Vec<u8> {
    match sample {
        Sample::File(path) => {
            std::fs::read(root().join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
        }
        Sample::Built(build, _) => build(),
    }
}

/// Every `pub fn read…` and `pub fn parse…` in a source tree, as (file under `src/`, function):
/// each line that opens with one, tests' too, so a source's own test helper has to be listed
/// rather than one's real reader passing unseen.
fn public_readers(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for line in source.lines() {
        let mut rest = line.trim_start();
        let Some(after) = rest.strip_prefix("pub ") else {
            continue;
        };
        rest = after;
        for qualifier in ["const ", "async ", "unsafe "] {
            rest = rest.strip_prefix(qualifier).unwrap_or(rest);
        }
        let Some(name) = rest.strip_prefix("fn ") else {
            continue;
        };
        let name: String = name
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.starts_with("read") || name.starts_with("parse") {
            found.push(name);
        }
    }
    found
}

fn sources_under(dir: &Path, into: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.map(|e| e.path()).unwrap_or_default();
        if path.is_dir() {
            sources_under(&path, into);
        } else if path.extension().is_some_and(|e| e == "rs")
            && path.file_name().is_some_and(|n| n != "tests.rs")
        {
            into.push(path);
        }
    }
}

/// The keys `crate file function` with how many times each is found, for the list and the sources.
fn count(keys: impl Iterator<Item = String>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for key in keys {
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

/// What differs between the sources' readers and the list's, one line each.
fn unlisted(
    in_sources: &BTreeMap<String, usize>,
    in_list: &BTreeMap<String, usize>,
) -> Vec<String> {
    let mut lines = Vec::new();
    for (key, found) in in_sources {
        let listed = in_list.get(key).copied().unwrap_or(0);
        if listed == 0 {
            lines.push(format!("{key}: in the source, not in the list"));
        } else if listed != *found {
            lines.push(format!(
                "{key}: {found} in the source, {listed} in the list"
            ));
        }
    }
    for key in in_list.keys() {
        if !in_sources.contains_key(key) {
            lines.push(format!("{key}: in the list, not in the source"));
        }
    }
    lines
}

#[test]
fn a_reader_missing_from_the_list_is_named() {
    let source = "pub fn read(a: &str) {}\n  pub const fn parse_x() {}\npub(crate) fn read_private() {}\n\
                  fn read_private_too() {}\n/// pub fn read_doc()\npub fn other() {}\n";
    assert_eq!(public_readers(source), ["read", "parse_x"]);
    let sources = count(
        ["c f read", "c f read", "c f parse"]
            .map(str::to_owned)
            .into_iter(),
    );
    let list = count(["c f read", "c f gone"].map(str::to_owned).into_iter());
    assert_eq!(
        unlisted(&sources, &list),
        [
            "c f parse: in the source, not in the list",
            "c f read: 2 in the source, 1 in the list",
            "c f gone: in the list, not in the source",
        ]
    );
    let once = count(["c f parse"].map(str::to_owned).into_iter());
    let twice = count(["c f parse", "c f parse"].map(str::to_owned).into_iter());
    assert_eq!(
        unlisted(&once, &twice),
        ["c f parse: 1 in the source, 2 in the list"]
    );
}

#[test]
fn the_list_names_every_public_reader_of_those_crates() {
    let mut in_sources = Vec::new();
    for krate in CRATES {
        let src = root().join("crates").join(krate).join("src");
        let mut files = Vec::new();
        sources_under(&src, &mut files);
        for file in files {
            let relative = file
                .strip_prefix(&src)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            let source = std::fs::read_to_string(&file).unwrap_or_default();
            for name in public_readers(&source) {
                in_sources.push(format!("{krate} {relative} {name}"));
            }
        }
    }
    let list = READERS
        .iter()
        .map(|r| format!("{} {} {}", r.krate, r.file, r.function))
        .chain(
            NO_OUTSIDE_BYTES
                .iter()
                .map(|r| format!("{} {} {}", r.krate, r.file, r.function)),
        );
    let differ = unlisted(&count(in_sources.into_iter()), &count(list));
    assert!(differ.is_empty(), "{}", differ.join("\n"));
}

#[test]
fn every_format_has_a_reader_on_the_list() {
    let missing: Vec<String> = FORMATS
        .iter()
        .filter(|format| {
            !READERS
                .iter()
                .chain(EXTRA_READERS)
                .any(|r| r.formats.contains(format))
        })
        .map(|format| format.name().to_owned())
        .collect();
    assert!(missing.is_empty(), "{}", missing.join("\n"));
    assert!(
        NO_OUTSIDE_BYTES.iter().all(|n| n.why.len() > 10),
        "a row of what takes no outside bytes says why"
    );
    for reader in READERS.iter().chain(EXTRA_READERS) {
        if let Sample::Built(_, why) = reader.sample {
            assert!(
                why.len() > 10,
                "{}: a built sample says why",
                reader.function
            );
        }
    }
}

#[test]
fn places_spread_evenly_and_a_change_changes_one_byte() {
    assert_eq!(evenly(100, 4), [0, 25, 50, 75]);
    assert_eq!(evenly(0, 2), [0, 0]);
    let bytes = *b"123";
    assert_eq!(changed(&bytes, 1, 0), [b'1', b'2' ^ 0xFF, b'3']);
    assert_eq!(changed(&bytes, 1, 1), [b'1', b'3', b'3']);
    assert_eq!(changed(&bytes, 9, 0), bytes);
}

#[test]
fn a_panic_and_a_refusal_are_told_apart() {
    let empty: Arc<[u8]> = Arc::from(Vec::new());
    assert_eq!(probe(|_| true, empty.clone()), Ended::Read);
    assert_eq!(probe(|_| false, empty), Ended::Refused);
    let Ended::Panicked { at, message } = probe(|bytes| bytes[9] == 0, Arc::from(&b"short"[..]))
    else {
        panic!("an index past the end is a panic");
    };
    assert!(at.contains("tests.rs"), "{at}");
    assert!(message.contains("index out of bounds"), "{message}");
}

#[test]
fn a_cut_tally_counts_and_a_panic_fails_its_check() {
    let mut tally = super::Tally::default();
    tally.cuts_of("a file", &[0; 90], |bytes| bytes.len() > 40);
    assert_eq!((tally.read, tally.refused), (4, 4));
    assert!(tally.panics.is_empty());
    assert!(tally.check("log").is_ok());
    tally.cuts_of("b file", &[0; 90], |bytes| bytes[50] == 0);
    assert_eq!((tally.read, tally.refused, tally.panics.len()), (7, 4, 5));
    assert!(tally.panics[0].starts_with("b file: cut at byte 10: panicked at "));
    let error = tally.check("log").expect_err("a panic fails the check");
    assert!(error.starts_with("5 cut(s) of a log panicked:"), "{error}");
}

#[test]
fn a_read_past_the_limit_is_a_hang_and_a_slow_cut_is_named_not_failed() {
    let spins = |_: &[u8]| loop {
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    let limit = std::time::Duration::from_millis(50);
    assert_eq!(
        probe_within(spins, Arc::from(Vec::new()), limit),
        Ended::Hung
    );
    let mut tally = super::Tally::default();
    tally.count("a file: cut at byte 10", Ended::Hung);
    assert_eq!(tally.slow, ["a file: cut at byte 10"]);
    assert!(tally.check("log").is_ok());
}

#[test]
fn a_file_that_cannot_be_loaded_fails_the_check() {
    let mut tally = super::Tally::default();
    tally.load("gone", Path::new("/no/such/file.ork"), |_, _| {
        panic!("a missing file isn't cut")
    });
    let error = tally.check("design").expect_err("it fails the check");
    assert!(error.contains("couldn't be loaded"), "{error}");
}

#[test]
fn a_collections_designs_are_found_in_subfolders_each_content_once() {
    let dir = tempfile::tempdir().expect("a temporary folder");
    std::fs::create_dir(dir.path().join("flight")).expect("a subfolder");
    std::fs::write(dir.path().join("a.ork"), "one").expect("a file");
    std::fs::write(dir.path().join("flight/same.ORK"), "one").expect("a file");
    std::fs::write(dir.path().join("flight/other.ork"), "two").expect("a file");
    std::fs::write(dir.path().join("flight/log.csv"), "three").expect("a file");
    let designs = super::designs_under(dir.path()).expect("a readable folder");
    assert_eq!(designs.len(), 2, "{designs:?}");
    assert!(super::designs_under(&dir.path().join("none")).is_err());
}

#[test]
fn every_file_of_a_corpus_is_cut_once() {
    let files: Vec<usize> = (0..100).collect();
    let tally = super::tally_cuts(&files, |_, tally| {
        tally.cuts_of("f", &[0; 90], |_| true);
    });
    assert_eq!((tally.read, tally.refused), (800, 0));
}

/// Feeds one reader all its damage; a line for each read that panicked or ran past the limit.
fn damage(reader: &Reader) -> Vec<String> {
    let name = format!("{} {} {}", reader.krate, reader.file, reader.function);
    let mut failures = Vec::new();
    let bytes = sample_bytes(reader.sample);
    assert!(
        bytes.len() > PLACES,
        "{name}: a sample of {} bytes has no {PLACES} distinct places",
        bytes.len()
    );
    assert!(
        probe(reader.read, Arc::from(&bytes[..])) == Ended::Read,
        "{name}: the whole sample doesn't read, so its damage tests only the first check"
    );
    let told = |what: String, ended: Ended, failures: &mut Vec<String>| match ended {
        Ended::Panicked { at, message } => {
            failures.push(format!("{name}: {what}: panicked at {at}: {message}"));
        }
        Ended::Hung => failures.push(format!("{name}: {what}{HUNG}")),
        Ended::Read | Ended::Refused => {}
    };
    for (place, at) in evenly(bytes.len(), PLACES).into_iter().enumerate() {
        // A row that hangs would cost 10 s at every place, and its spinning threads would push
        // other rows past the limit: its first hang ends it.
        if failures.iter().any(|f: &String| f.ends_with(HUNG)) {
            return failures;
        }
        let cut: Arc<[u8]> = Arc::from(&bytes[..at]);
        told(
            format!("cut at byte {at}"),
            probe(reader.read, cut),
            &mut failures,
        );
        let once: Arc<[u8]> = Arc::from(changed(&bytes, at, place));
        told(
            format!("byte {at} changed"),
            probe(reader.read, once),
            &mut failures,
        );
    }
    // The seed is the reader's own, so a failure comes back on the next run on any machine.
    let sample = match reader.sample {
        Sample::File(path) => path,
        Sample::Built(_, why) => why,
    };
    let mut seed = [0_u8; 32];
    for (at, byte) in format!("{name} {sample}").bytes().enumerate() {
        seed[at % seed.len()] ^= byte.rotate_left((at / seed.len()) as u32);
    }
    let mut runner = TestRunner::new_with_rng(
        Config {
            cases: ARBITRARY_CASES,
            failure_persistence: None,
            // A hang costs its 10 s at each step of shrinking it.
            max_shrink_time: 30_000,
            ..Config::default()
        },
        TestRng::from_seed(RngAlgorithm::ChaCha, &seed),
    );
    let read = reader.read;
    let result = runner.run(
        &proptest::collection::vec(any::<u8>(), 0..=ARBITRARY_LEN),
        |case| match probe(read, Arc::from(&case[..])) {
            Ended::Read | Ended::Refused => Ok(()),
            ended => Err(TestCaseError::fail(format!("{ended:?}"))),
        },
    );
    if let Err(error) = result {
        failures.push(format!("{name}: arbitrary bytes: {error}"));
    }
    failures
}

/// Walks the rows of one crate, each row on its own thread.
fn damage_all(krate: &str) {
    let started = Instant::now();
    let readers: Vec<&Reader> = READERS
        .iter()
        .chain(EXTRA_READERS)
        .filter(|r| r.krate == krate)
        .collect();
    assert!(!readers.is_empty(), "{krate}: no readers on the list");
    let width = std::thread::available_parallelism().map_or(2, usize::from);
    let mut failures = Vec::new();
    for batch in readers.chunks(width) {
        let found: Vec<Vec<String>> = std::thread::scope(|scope| {
            let handles: Vec<_> = batch
                .iter()
                .map(|reader| scope.spawn(|| damage(reader)))
                .collect();
            handles
                .into_iter()
                .map(|h| {
                    h.join()
                        .unwrap_or_else(|_| vec![format!("{krate}: a row's own checks failed")])
                })
                .collect()
        });
        failures.extend(found.into_iter().flatten());
    }
    eprintln!(
        "{krate}: {} readers in {:?}",
        readers.len(),
        started.elapsed()
    );
    assert!(
        failures.is_empty(),
        "{} damaged reads failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn damaged_input_never_panics_or_hangs_in_hpr_flightdata() {
    damage_all("hpr-flightdata");
}

#[test]
fn damaged_input_never_panics_or_hangs_in_hpr_io() {
    damage_all("hpr-io");
}

#[test]
fn damaged_input_never_panics_or_hangs_in_hpr_motor() {
    damage_all("hpr-motor");
}

#[test]
fn damaged_input_never_panics_or_hangs_in_hpr_format() {
    damage_all("hpr-format");
}

#[test]
fn damaged_input_never_panics_or_hangs_in_hpr_net() {
    damage_all("hpr-net");
}
