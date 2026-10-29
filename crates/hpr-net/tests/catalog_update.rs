//! `hpr motors update`'s comparison (M5.7d) against ThrustCurve.org's recorded answers: its whole
//! catalog's search, and two motors' downloads, recorded unchanged on 2026-10-29 at 12:47 UTC and
//! used as published facts with its credit (ADR-287).
//!
//! On that day ThrustCurve.org listed nothing newer than the bundle (its snapshot of 2026-10-28),
//! so the recorded answers find nothing against the bundle as it is. To show what an update adds,
//! a test takes one motor (ZEVETA's ROS-40-5, the most recently updated, with one file) and one
//! file (AeroTech H128W's RockSim file) out of the bundle, as if the snapshot hadn't listed them,
//! and the same recorded answers find exactly those. The expected values are read from the
//! recordings with `serde_json`, not through the code under test.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests read the committed fixtures and write a cache; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::path::Path;

use hpr_motor::Catalog;
use hpr_net::catalog_update::{self, Bundle};
use hpr_net::thrustcurve::ThrustCurveError;
use hpr_net::{Cache, Client, Freshness, Mode, NetError, Replay, Transport};
use serde_json::Value;

/// 2026-10-29 12:50 UTC, just after the answers were recorded.
const NOW_S: u64 = 1_793_278_200;

/// ZEVETA ROS-40-5: its id and its one file, a RASP file.
const ROS_ID: &str = "6aa672715eabcd0002df9b36";
const ROS_FILE_ID: &str = "6aa6d2a7593d99000250d4f8";
/// AeroTech H128W: its id, and its RockSim file.
const H128W_ID: &str = "5f4294d20002310000000083";
const H128W_ROCKSIM_ID: &str = "5f4294d20002e90000000397";

fn replay() -> Replay {
    Replay::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay")).unwrap()
}

fn recorded(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/replay")
        .join(name);
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

fn bundle() -> Bundle {
    Bundle::of(Catalog::bundled().unwrap())
}

/// A transport that fails the test if called: offline, every answer must come from the cache.
struct Forbidden;

impl Transport for Forbidden {
    fn get(&self, url: &str) -> Result<Vec<u8>, String> {
        panic!("offline, {url} was fetched")
    }
}

/// The recorded whole-catalog search lists exactly the bundle's motors as its solid ones, the
/// hybrids aside, so nothing is downloaded and nothing is newer.
#[test]
fn the_recorded_catalog_lists_nothing_newer_than_the_bundle() {
    let dir = tempfile::tempdir().unwrap();
    let transport = replay();
    let client = Client::new(&transport, Cache::new(dir.path()), Mode::Online);
    let bundle = bundle();
    let update = catalog_update::update(&client, &bundle, NOW_S).unwrap();

    let search = recorded("thrustcurve-search-all.json");
    let results = search["results"].as_array().unwrap();
    let solid = results
        .iter()
        .filter(|r| matches!(r["type"].as_str(), Some("SU" | "reload")))
        .count();
    assert_eq!((results.len(), solid), (1_156, 1_004));
    assert_eq!(update.listed, solid);
    assert_eq!(bundle.motors(), solid);
    assert_eq!((update.looked_at, update.newer.len()), (0, 0));
    assert!(update.unlisted.is_empty());
    assert_eq!(update.fetched.len(), 1);
    assert_eq!(transport.calls(), 1);
}

/// With one motor and one file taken out of the bundle, the update downloads those two motors'
/// files and finds exactly what was taken out, each file as recorded; the answers land in the
/// cache, so the same update offline finds the same, never calling the network.
#[test]
fn a_motor_and_a_file_the_bundle_lacks_are_found_and_cached() {
    let dir = tempfile::tempdir().unwrap();
    let transport = replay();
    let online = Client::new(&transport, Cache::new(dir.path()), Mode::Online);
    let mut bundle = bundle();
    assert!(bundle.remove_motor(ROS_ID));
    assert!(bundle.remove_file(H128W_ROCKSIM_ID));

    let update = catalog_update::update(&online, &bundle, NOW_S).unwrap();
    assert_eq!(update.looked_at, 2);
    // The search lists the ROS-40-5 ahead of H128W, and the update keeps its order.
    let found: Vec<(&str, bool, Vec<&str>)> = update
        .newer
        .iter()
        .map(|newer| {
            let files = newer.files.iter().map(|f| f.simfile_id.as_str()).collect();
            (newer.record.motor_id.as_str(), newer.new_motor, files)
        })
        .collect();
    assert_eq!(
        found,
        [
            (ROS_ID, true, vec![ROS_FILE_ID]),
            (H128W_ID, false, vec![H128W_ROCKSIM_ID]),
        ]
    );
    // Each file is the download's own, field for field.
    for (newer, name) in update.newer.iter().zip([
        "thrustcurve-download-ROS-40-5.json",
        "thrustcurve-download-H128W-rocksim.json",
    ]) {
        let file = &recorded(name)["results"][0];
        let ours = &newer.files[0];
        assert_eq!(ours.simfile_id, file["simfileId"].as_str().unwrap());
        assert_eq!(ours.motor_id, file["motorId"].as_str().unwrap());
        assert_eq!(ours.data, file["data"].as_str().unwrap());
        assert_eq!(ours.source.as_deref(), file["source"].as_str());
        assert_eq!(ours.license.as_deref(), file["license"].as_str());
    }
    let ros = &recorded("thrustcurve-search-all.json")["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["motorId"] == ROS_ID)
        .unwrap()
        .clone();
    let record = &update.newer[0].record;
    assert_eq!(
        (record.designation.as_str(), record.manufacturer.as_str()),
        (
            ros["designation"].as_str().unwrap(),
            ros["manufacturer"].as_str().unwrap()
        )
    );
    // The search and two downloads (RASP, RockSim) for each of the two motors.
    assert_eq!((update.fetched.len(), transport.calls()), (5, 5));

    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let again = catalog_update::update(&offline, &bundle, NOW_S + 60).unwrap();
    assert_eq!(again.newer, update.newer);
    assert!(
        again
            .fetched
            .iter()
            .all(|f| f.freshness == Freshness::Cached)
    );
}

/// Offline with nothing in the cache, the update fails as not cached, so a caller can tell that
/// from a network failure and name the bundle's date instead.
#[test]
fn offline_with_no_answer_cached_is_not_cached() {
    let dir = tempfile::tempdir().unwrap();
    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let error = catalog_update::update(&offline, &bundle(), NOW_S).unwrap_err();
    assert!(
        matches!(error, ThrustCurveError::Net(NetError::NotCached { .. })),
        "{error:?}"
    );
}

/// A search cut short by its `maxResults` is refused, not read as the whole catalog (a motor past
/// the cut would look as if it weren't listed), and never cached: with a whole copy in the cache,
/// online past its day the update reads that copy, stale, and offline it still finds it.
#[test]
fn a_search_cut_short_is_refused_and_not_cached() {
    struct Short;
    impl Transport for Short {
        fn get(&self, _url: &str) -> Result<Vec<u8>, String> {
            let mut answer = recorded("thrustcurve-search-all.json");
            answer["results"].as_array_mut().unwrap().truncate(1_000);
            Ok(serde_json::to_vec(&answer).unwrap())
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let short = Client::new(Short, Cache::new(dir.path()), Mode::Online);
    let error = catalog_update::update(&short, &bundle(), NOW_S).unwrap_err();
    assert!(
        matches!(
            &error,
            ThrustCurveError::Net(NetError::Refused { reason, .. })
                if reason.contains("matches 1156 motors but returns only 1000")
        ),
        "{error:?}"
    );
    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let error = catalog_update::update(&offline, &bundle(), NOW_S).unwrap_err();
    assert!(
        matches!(error, ThrustCurveError::Net(NetError::NotCached { .. })),
        "the cut-short answer was cached: {error:?}"
    );

    let transport = replay();
    let whole = Client::new(&transport, Cache::new(dir.path()), Mode::Online);
    catalog_update::update(&whole, &bundle(), NOW_S).unwrap();
    let later = NOW_S + hpr_net::thrustcurve::TTL_S;
    let update = catalog_update::update(&short, &bundle(), later).unwrap();
    assert_eq!(update.fetched[0].freshness, Freshness::Stale);
    assert_eq!(update.listed, 1_004);
    let update = catalog_update::update(&offline, &bundle(), later).unwrap();
    assert_eq!((update.listed, update.newer.len()), (1_004, 0));
}
