//! A motor found by name on ThrustCurve.org (M4.5b, ADR-154), against answers in
//! `tests/fixtures/replay/`: F27R/L's RockSim download, recorded on 2026-10-01 (a public-domain
//! file), and stand-in searches in the API's shape with invented records but F27R/L's own id
//! (ADR-145: ThrustCurve.org grants no license for its records). F27R/L's RASP answer is a
//! stand-in holding no file, so the search falls through to the RockSim one.

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

use hpr_net::on_demand::{self, FindError, Wanted};
use hpr_net::thrustcurve::Format;
use hpr_net::{Cache, Client, Freshness, Mode, Replay, Transport};

const NOW_S: u64 = 1_790_843_400;

fn replay() -> Replay {
    Replay::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay")).unwrap()
}

/// A transport that must never be called.
struct Forbidden;

impl Transport for Forbidden {
    fn get(&self, url: &str) -> Result<Vec<u8>, String> {
        panic!("offline mode called the transport for {url}");
    }
}

/// F27R/L is found by its designation: one search, a RASP download with no file, then the RockSim
/// download's one file, which reads. Offline, the cache gives the same file with no request.
#[test]
fn a_motor_found_once_is_found_again_offline() {
    let dir = tempfile::tempdir().unwrap();
    let transport = replay();
    let online = Client::new(&transport, Cache::new(dir.path()), Mode::Online);
    let found = on_demand::find(&online, &Wanted::named("F27R/L"), NOW_S).unwrap();
    assert_eq!(found.record.designation, "F27R/L");
    assert_eq!(found.record.motor_id, "5f4294d200023100000001d4");
    assert_eq!(found.file.format, Format::RockSim);
    assert_eq!(
        (found.file.source.as_deref(), found.file.license.as_deref()),
        (Some("user"), Some("PD"))
    );
    assert_eq!(found.fetched.len(), 3);
    assert_eq!(transport.calls(), 3);
    assert!(
        found
            .fetched
            .iter()
            .all(|f| f.freshness == Freshness::Fetched)
    );

    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let again = on_demand::find(&offline, &Wanted::named("f27r/l"), NOW_S + 10).unwrap_err();
    // The cache is keyed by the URL, so a name spelled otherwise is a different question.
    assert!(again.is_not_cached(), "{again}");
    let again = on_demand::find(&offline, &Wanted::named("F27R/L"), NOW_S + 10).unwrap();
    assert_eq!(again.file, found.file);
    assert!(
        again
            .fetched
            .iter()
            .all(|f| f.freshness == Freshness::Cached)
    );
}

/// No motor has the designation G41, and two have it as their common name: both are named, by
/// maker and designation, sorted.
#[test]
fn a_name_two_motors_share_is_refused_with_both() {
    let dir = tempfile::tempdir().unwrap();
    let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
    match on_demand::find(&client, &Wanted::named("G41"), NOW_S) {
        Err(FindError::Ambiguous {
            wanted,
            matches,
            candidates,
        }) => {
            assert_eq!((wanted.as_str(), matches), ("G41", 2));
            assert_eq!(candidates, ["AeroTech G41-INVENTED", "Invented G41-OTHER"]);
        }
        other => panic!("{other:?}"),
    }
}

/// A name no motor has, and a hybrid, are refused by what they are.
#[test]
fn an_unknown_name_and_a_hybrid_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
    match on_demand::find(&client, &Wanted::named("Z9"), NOW_S) {
        Err(FindError::NotFound { wanted }) => assert_eq!(wanted, "Z9"),
        other => panic!("{other:?}"),
    }
    match on_demand::find(&client, &Wanted::named("H100-HYBRID"), NOW_S) {
        Err(FindError::Hybrid { motor }) => assert_eq!(motor, "Invented Hybrids H100-HYBRID"),
        other => panic!("{other:?}"),
    }
}

/// Offline with an empty cache, the search is refused as not cached, with no request made.
#[test]
fn offline_and_uncached_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let error = on_demand::find(&offline, &Wanted::named("F27R/L"), NOW_S).unwrap_err();
    assert!(error.is_not_cached(), "{error}");
}

/// Of an invented motor's four RASP files, listed user, certification (unreadable),
/// certification (reads, as one motor), manufacturer: `find` skips the unreadable one and takes
/// the certification file that reads, ahead of the user's listed first; `find_with` passes over
/// a file its own reading refuses, to the manufacturer's next in rank, and with every file
/// refused tries the two RockSim files last.
#[test]
fn the_first_file_by_who_measured_it_that_reads_is_taken() {
    let dir = tempfile::tempdir().unwrap();
    let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
    let wanted = Wanted::named("Z10-INVENTED");
    let found = on_demand::find(&client, &wanted, NOW_S).unwrap();
    assert_eq!(found.file.simfile_id, "f000000000000000000000a4");
    let (found, ()) = on_demand::find_with(&client, &wanted, NOW_S, |file| {
        if file.simfile_id == "f000000000000000000000a4" {
            Err("refused by the caller".to_owned())
        } else {
            on_demand::reads_as_one_motor(file)
        }
    })
    .unwrap();
    assert_eq!(found.file.simfile_id, "f000000000000000000000a3");
    match on_demand::find_with(&client, &wanted, NOW_S, |_| {
        Err::<(), _>("refused".to_owned())
    }) {
        Err(FindError::NoFile { motor, why }) => {
            assert_eq!(motor, "Invented Motors Z10-INVENTED");
            assert_eq!(why.as_deref(), Some("5f4294d20002e900000004e3: refused"));
        }
        other => panic!("{other:?}"),
    }
}

/// A preferred file is taken ahead of the rank's order when it reads: the user's file, ranked
/// last, when named, and the RockSim file ahead of every RASP one. A preferred file that doesn't
/// read, or that neither answer holds, leaves the rank's order as it was; behind a preferred file
/// the rest keep that order. A preferred RASP file leaves the RockSim answer unread until the
/// RASP files are refused; one the RASP answer lacks has it read before any file is taken.
#[test]
fn a_preferred_file_is_taken_first_when_it_reads() {
    let dir = tempfile::tempdir().unwrap();
    let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
    let found = |prefer: &str| {
        let wanted = Wanted::named("Z10-INVENTED").preferring(prefer);
        on_demand::find(&client, &wanted, NOW_S).unwrap()
    };
    let taken = |prefer: &str| found(prefer).file.simfile_id;
    assert_eq!(
        taken("f000000000000000000000a1"),
        "f000000000000000000000a1"
    );
    assert_eq!(found("f000000000000000000000a1").fetched.len(), 2);
    let rocksim = found("f000000000000000000000b1");
    assert_eq!(
        (rocksim.file.simfile_id.as_str(), rocksim.file.format),
        ("f000000000000000000000b1", Format::RockSim)
    );
    assert_eq!(rocksim.fetched.len(), 3);
    assert_eq!(
        taken("f000000000000000000000a3"),
        "f000000000000000000000a3"
    );
    assert_eq!(
        taken("f000000000000000000000a2"),
        "f000000000000000000000a4"
    );
    assert_eq!(
        taken("f0000000000000000000ffff"),
        "f000000000000000000000a4"
    );
    let tried = std::cell::RefCell::new(Vec::new());
    let wanted = Wanted::named("Z10-INVENTED").preferring("f000000000000000000000a3");
    let _ = on_demand::find_with(&client, &wanted, NOW_S, |file| {
        tried.borrow_mut().push(file.simfile_id.clone());
        Err::<(), _>("refused".to_owned())
    });
    assert_eq!(
        tried.into_inner(),
        [
            "f000000000000000000000a3",
            "f000000000000000000000a2",
            "f000000000000000000000a4",
            "f000000000000000000000a1",
            "f000000000000000000000b1",
            "5f4294d20002e900000004e3",
        ]
    );
}

/// Offline, with a cache holding the motor's RASP answer alone, a preferred file the RASP answer
/// lacks is given up and the RASP files are taken as if none were named; with nothing to fall
/// back on, the missing answer is refused as not cached.
#[test]
fn offline_a_preference_the_cache_cannot_answer_is_given_up() {
    let dir = tempfile::tempdir().unwrap();
    let online = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
    let wanted = Wanted::named("Z10-INVENTED");
    on_demand::find(&online, &wanted, NOW_S).unwrap();
    let offline = Client::new(Forbidden, Cache::new(dir.path()), Mode::Offline);
    let found = on_demand::find(
        &offline,
        &wanted.clone().preferring("f000000000000000000000b1"),
        NOW_S,
    )
    .unwrap();
    assert_eq!(found.file.simfile_id, "f000000000000000000000a4");
    assert_eq!(found.fetched.len(), 2);
    let none = on_demand::find_with(
        &offline,
        &wanted.preferring("f000000000000000000000b1"),
        NOW_S,
        |_| Err::<(), _>("refused".to_owned()),
    );
    assert!(
        none.as_ref().is_err_and(FindError::is_not_cached),
        "{none:?}"
    );
}
