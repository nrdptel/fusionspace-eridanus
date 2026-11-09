//! New flights predicted first (M2.7b, ADR-332 §1, ADR-333): a flight the private collection's
//! manifest gains after M2.7a is compared with its log only once its prediction, written from
//! its plan alone, sits in the collection and that file's SHA-256 reached main in an earlier
//! pull request.
//!
//! - `cargo xtask fixture-flights predict <plan.json>` reads a flight's [`Plan`]: its design,
//!   motor, day and site, and refuses a file holding any other field, a log's included. It flies
//!   it as `fixture-flights` does ([`crate::fixture_flights::predicted_apogee`]), writes the
//!   prediction under [`PREDICTIONS`] in the collection, never over an older one, and prints the
//!   file's SHA-256, the line for [`LIST`].
//! - [`LIST`], committed, holds only those hashes. Pull requests here are squash-merged, so a
//!   hash is on main before a comparison only if it merged in a pull request of its own: the
//!   check reads the list as it stands where the branch left main (`git merge-base HEAD
//!   origin/main`), not in the working tree.
//! - [`BEFORE`], committed, is the SHA-256 of M2.7a's manifest ids. A flight not marked
//!   `added_after_m2_7a` must be one of them, so a new flight can't pass as an old one, and an old
//!   one marked new needs a prediction it can't have had: either way the run stops.
//! - [`admit`] refuses, by id, every flight marked new whose prediction is missing, whose hash is
//!   not on the list at main, or whose prediction is of another plan, design file or ERA5 file
//!   than the flight compared. The ids are private (ADR-151); the refusal is printed on this
//!   machine only.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::fixture_flights::{Entry, Plan, TIER_A_ORK};

/// The committed list of the predictions' files' SHA-256, one a line (M2.7b).
pub(crate) const LIST: &str = "validation/reports/predictions.sha256";
/// The committed SHA-256 of M2.7a's manifest ids (M2.7b).
pub(crate) const BEFORE: &str = "validation/reports/m2.7a-flights.sha256";
/// Where the predictions sit in the private collection, one `<flight id>.json` each.
pub(crate) const PREDICTIONS: &str = "refs/hpr-sim-fixtures/predictions";
/// The branch whose history the list must already be in.
const MAIN: &str = "origin/main";
/// What a prediction's apogee is.
const WHAT: &str = "hpr's apogee in the day's ERA5 air, nothing deployed: the center of mass's \
     height at apogee above where it started, as fixture-flights compares it";

/// A plan's fields, the only ones a plan file may hold: a field outside them, a log's above
/// all, stops `predict` (a test holds this to [`Plan`]'s own).
const PLAN_KEYS: [&str; 13] = [
    "flight_id",
    "design",
    "configuration_id",
    "motor",
    "launch_utc",
    "launch_time_assumed",
    "latitude_deg",
    "longitude_deg",
    "elevation_m",
    "rail_length_m",
    "rail_from_vertical_deg",
    "rail_heading_deg",
    "era5",
];

/// A prediction's file, as `predict` writes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Prediction {
    /// The flight's manifest id.
    pub flight_id: String,
    /// The plan it was flown from.
    pub plan: Plan,
    /// SHA-256 of the design file flown.
    pub design_sha256: String,
    /// SHA-256 of the day's ERA5 file.
    pub era5_sha256: String,
    /// The commit of this repository that flew it.
    pub hpr_commit: String,
    /// The apogee, m.
    pub predicted_apogee_m: f64,
    /// What that apogee is, in words.
    pub what: String,
}

/// SHA-256 as lowercase hex.
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut out, byte| {
            let _ = write!(out, "{byte:02x}");
            out
        })
}

/// A plan from a plan file's text, refusing any field that isn't a plan's.
pub(crate) fn read_plan(text: &str) -> Result<Plan, String> {
    let value: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let object = value.as_object().ok_or("a plan is one JSON object")?;
    if let Some(key) = object.keys().find(|key| !PLAN_KEYS.contains(&key.as_str())) {
        return Err(format!(
            "`{key}` is not a field of a flight's plan: a prediction is written from the design, \
             motor, day and site alone, without the log (ADR-332 §1)"
        ));
    }
    serde_json::from_value(value).map_err(|error| error.to_string())
}

/// The list's hashes: one per line, 64 lowercase hex digits; `#` starts a comment line.
pub(crate) fn parse_list(text: &str) -> Result<BTreeSet<String>, String> {
    let mut hashes = BTreeSet::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let hex = line.len() == 64
            && line
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
        if !hex {
            return Err(format!(
                "{LIST}:{}: not a SHA-256 in lowercase hex",
                number + 1
            ));
        }
        hashes.insert(line.to_owned());
    }
    Ok(hashes)
}

/// Runs git in `root`, its standard output on success.
fn git(root: &Path, args: &[&str]) -> Result<Option<String>, String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|error| format!("running git: {error}"))?;
    Ok(output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned()))
}

/// The list as it stands where `HEAD` left [`MAIN`]: only hashes merged before this branch.
/// No list there is an empty one.
pub(crate) fn list_at_main(root: &Path) -> Result<BTreeSet<String>, String> {
    let base = git(root, &["merge-base", "HEAD", MAIN])?
        .ok_or_else(|| format!("no common commit of HEAD and {MAIN}: run `git fetch origin`"))?;
    let at = format!("{}:{LIST}", base.trim());
    if git(root, &["cat-file", "-e", &at])?.is_none() {
        return Ok(BTreeSet::new());
    }
    let text = git(root, &["show", &at])?.ok_or_else(|| format!("git show {at} failed"))?;
    parse_list(&text)
}

/// [`BEFORE`]'s text for the given digest.
pub(crate) fn before_text(digest: &str) -> String {
    format!(
        "{digest}  SHA-256 of the private collection's manifest ids at M2.7a (ADR-332), sorted \
         bytewise, one per line with a newline after each\n"
    )
}

/// Checks the manifest's flights not marked `added_after_m2_7a` are M2.7a's: [`TIER_A_ORK`] of
/// them, whose ids hash to [`BEFORE`]'s digest.
pub(crate) fn check_before(root: &Path, entries: &[Entry]) -> Result<(), String> {
    let committed = fs::read_to_string(root.join(BEFORE))
        .map_err(|error| format!("reading {BEFORE}: {error}"))?;
    held_to_before(entries, &committed)
}

/// [`check_before`] against [`BEFORE`]'s committed text.
fn held_to_before(entries: &[Entry], committed: &str) -> Result<(), String> {
    let before: Vec<&str> = entries
        .iter()
        .filter(|entry| !entry.added_after_m2_7a)
        .map(|entry| entry.flight_id.as_str())
        .collect();
    let digest = crate::collection_boost::held_out_digest(before.iter().copied());
    if before.len() != TIER_A_ORK || committed != before_text(&digest) {
        return Err(format!(
            "the manifest's flights not marked `added_after_m2_7a` are {} that aren't M2.7a's \
             {TIER_A_ORK} ({BEFORE}): a flight added since must be marked, and compared only once \
             its prediction's hash reached main earlier (ADR-332 §1)",
            before.len()
        ));
    }
    Ok(())
}

/// A flight marked new, with what the check reads of it.
pub(crate) struct NewFlight<'a> {
    /// Its plan, as the manifest gives it.
    pub plan: &'a Plan,
    /// Its prediction file's bytes, if the collection holds one.
    pub prediction: Option<Vec<u8>>,
    /// SHA-256 of its design file now.
    pub design_sha256: String,
    /// SHA-256 of its day's ERA5 file now, if it names one.
    pub era5_sha256: Option<String>,
}

/// Why `flight` may not be compared yet, or `None` when it may: its prediction sits in the
/// collection, the file's SHA-256 is in `earlier` (the list at main), and the prediction is of
/// this flight's plan, design file and ERA5 file.
pub(crate) fn refusal(flight: &NewFlight<'_>, earlier: &BTreeSet<String>) -> Option<String> {
    let Some(bytes) = &flight.prediction else {
        return Some(format!(
            "no prediction in the collection ({PREDICTIONS}/{}.json)",
            flight.plan.flight_id
        ));
    };
    let hash = sha256_hex(bytes);
    if !earlier.contains(&hash) {
        return Some(format!(
            "its prediction's SHA-256 {hash} is not in {LIST} on main before this branch: merge \
             it in a pull request of its own first"
        ));
    }
    let prediction: Prediction = match serde_json::from_slice(bytes) {
        Ok(prediction) => prediction,
        Err(error) => return Some(format!("its prediction doesn't read: {error}")),
    };
    if prediction.flight_id != flight.plan.flight_id || &prediction.plan != flight.plan {
        return Some("its prediction is of another plan than the manifest's".to_owned());
    }
    if prediction.design_sha256 != flight.design_sha256 {
        return Some("its design file changed after the prediction".to_owned());
    }
    if Some(&prediction.era5_sha256) != flight.era5_sha256.as_ref() {
        return Some("its ERA5 file changed after the prediction".to_owned());
    }
    if !prediction.predicted_apogee_m.is_finite() || prediction.predicted_apogee_m <= 0.0 {
        return Some("its prediction holds no apogee".to_owned());
    }
    None
}

/// Every refusal among `flights`, by id; an error naming each when any is refused.
pub(crate) fn refusals(
    flights: &[NewFlight<'_>],
    earlier: &BTreeSet<String>,
) -> Result<(), String> {
    let refused: Vec<String> = flights
        .iter()
        .filter_map(|flight| {
            refusal(flight, earlier).map(|why| format!("  {}: {why}", flight.plan.flight_id))
        })
        .collect();
    if refused.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "a flight added after M2.7a is compared only once its prediction's hash reached main \
             in an earlier pull request (ADR-332 §1); refused:\n{}",
            refused.join("\n")
        ))
    }
}

/// SHA-256 of a file under `root`, or `None` when it can't be read.
fn file_sha256(root: &Path, path: &str) -> Option<String> {
    fs::read(root.join(path))
        .ok()
        .map(|bytes| sha256_hex(&bytes))
}

/// Refuses the run when a flight marked new may not be compared yet ([`refusal`]). With none
/// marked, it reads nothing.
pub(crate) fn admit(root: &Path, entries: &[Entry]) -> Result<(), String> {
    let added: Vec<&Entry> = entries.iter().filter(|e| e.added_after_m2_7a).collect();
    if added.is_empty() {
        return Ok(());
    }
    let flights: Vec<NewFlight<'_>> = added
        .iter()
        .map(|entry| NewFlight {
            plan: &entry.plan,
            prediction: fs::read(root.join(format!("{PREDICTIONS}/{}.json", entry.flight_id))).ok(),
            design_sha256: file_sha256(root, &entry.design).unwrap_or_default(),
            era5_sha256: entry
                .era5
                .as_deref()
                .and_then(|era5| file_sha256(root, era5)),
        })
        .collect();
    refusals(&flights, &list_at_main(root)?)
}

/// `fixture-flights predict <plan.json> [--out <file>]`.
pub(crate) fn run(root: &Path, args: &[String]) -> Result<(), String> {
    let usage = || format!("usage:\n{}", crate::fixture_flights::USAGE);
    let (plan_path, out) = match args {
        [plan] => (plan, None),
        [plan, flag, out] if flag == "--out" => (plan, Some(out)),
        _ => return Err(usage()),
    };
    let text =
        fs::read_to_string(plan_path).map_err(|error| format!("reading {plan_path}: {error}"))?;
    let plan = read_plan(&text).map_err(|error| format!("{plan_path}: {error}"))?;
    let at = plan.flight_id.clone();
    let folder = format!("refs/hpr-sim-fixtures/flights/{at}/");
    if !root.join(&folder).is_dir() || !plan.design.starts_with(&folder) {
        return Err(format!(
            "{at} is not a flight of the collection, or its design is another's"
        ));
    }
    let era5 = plan
        .era5
        .as_deref()
        .ok_or_else(|| format!("{at}: {}", crate::fixture_flights::NO_ERA5))?;
    let prediction = Prediction {
        flight_id: at.clone(),
        design_sha256: file_sha256(root, &plan.design)
            .ok_or_else(|| format!("{at}: can't read {}", plan.design))?,
        era5_sha256: file_sha256(root, era5).ok_or_else(|| format!("{at}: can't read {era5}"))?,
        hpr_commit: git(root, &["rev-parse", "HEAD"])?
            .ok_or("git rev-parse HEAD failed")?
            .trim()
            .to_owned(),
        predicted_apogee_m: crate::fixture_flights::predicted_apogee(root, &plan)?,
        what: WHAT.to_owned(),
        plan,
    };
    let bytes =
        serde_json::to_string_pretty(&prediction).map_err(|error| error.to_string())? + "\n";
    let path = out.map_or_else(
        || root.join(format!("{PREDICTIONS}/{at}.json")),
        |out| Path::new(out).to_path_buf(),
    );
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    // Never over an older prediction: a second one would be written knowing more.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| format!("{}: {error} (a prediction is written once)", path.display()))?;
    std::io::Write::write_all(&mut file, bytes.as_bytes())
        .map_err(|error| format!("{}: {error}", path.display()))?;
    println!(
        "fixture-flights predict: wrote {}\n{}\nAdd that line to {LIST} in a pull request of its \
         own and merge it before the flight's log is compared; commit the prediction to the \
         collection.",
        path.display(),
        sha256_hex(bytes.as_bytes())
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// An invented flight's plan: no flight of the collection.
    fn invented(id: &str) -> Plan {
        serde_json::from_value(json!({
            "flight_id": id, "design": format!("refs/hpr-sim-fixtures/flights/{id}/design/a.ork"),
            "configuration_id": null, "motor": {"designation": "K1"},
            "launch_utc": "2027-01-01T12:00:00Z", "launch_time_assumed": false,
            "latitude_deg": 10.0, "longitude_deg": 20.0, "elevation_m": 300.0,
            "rail_length_m": null, "rail_from_vertical_deg": null, "rail_heading_deg": null,
            "era5": format!("refs/hpr-sim-fixtures/flights/{id}/weather/day.nc"),
        }))
        .expect("a plan")
    }

    /// Its prediction's file, as `predict` writes it.
    fn prediction(plan: &Plan) -> Vec<u8> {
        let prediction = Prediction {
            flight_id: plan.flight_id.clone(),
            plan: plan.clone(),
            design_sha256: "d".repeat(64),
            era5_sha256: "e".repeat(64),
            hpr_commit: "0".repeat(40),
            predicted_apogee_m: 1234.5,
            what: WHAT.to_owned(),
        };
        (serde_json::to_string_pretty(&prediction).expect("json") + "\n").into_bytes()
    }

    fn flight<'a>(plan: &'a Plan, prediction: Option<Vec<u8>>) -> NewFlight<'a> {
        NewFlight {
            plan,
            prediction,
            design_sha256: "d".repeat(64),
            era5_sha256: Some("e".repeat(64)),
        }
    }

    /// An invented flight added after M2.7a is compared only once its prediction's hash is on
    /// the list at main, and refused by its id otherwise (ADR-332 §1).
    #[test]
    fn an_invented_flight_is_compared_only_once_its_prediction_hash_merged() {
        let plan = invented("invented-20270101");
        let bytes = prediction(&plan);
        let hash = sha256_hex(&bytes);
        let on_main = BTreeSet::from([hash.clone()]);
        let empty = BTreeSet::new();

        // No prediction: refused by name.
        let error = refusals(&[flight(&plan, None)], &on_main).expect_err("no prediction");
        assert!(
            error.contains("invented-20270101: no prediction"),
            "{error}"
        );
        // A prediction whose hash isn't on main: refused, the hash named.
        let error = refusals(&[flight(&plan, Some(bytes.clone()))], &empty)
            .expect_err("not merged earlier");
        assert!(
            error.contains("invented-20270101: its prediction's SHA-256"),
            "{error}"
        );
        assert!(error.contains(&hash), "{error}");
        // On main: compared.
        assert_eq!(
            refusals(&[flight(&plan, Some(bytes.clone()))], &on_main),
            Ok(())
        );

        // A prediction edited after its hash merged no longer matches it.
        let mut edited = bytes.clone();
        let at = edited.len() - 3;
        edited[at] ^= 1;
        assert!(refusal(&flight(&plan, Some(edited)), &on_main).is_some());
        // Another flight's prediction, its hash on main, doesn't stand for this one.
        let other = invented("invented-20270102");
        let error = refusals(&[flight(&other, Some(bytes.clone()))], &on_main)
            .expect_err("another flight's");
        assert!(
            error.contains("invented-20270102: its prediction is of another plan"),
            "{error}"
        );
        // The plan changed after the prediction: refused.
        let mut moved = plan.clone();
        moved.elevation_m += 1.0;
        assert!(refusal(&flight(&moved, Some(bytes.clone())), &on_main).is_some());
        // The design or the day's file changed after it: refused.
        let mut redesigned = flight(&plan, Some(bytes.clone()));
        redesigned.design_sha256 = "f".repeat(64);
        assert_eq!(
            refusal(&redesigned, &on_main).as_deref(),
            Some("its design file changed after the prediction")
        );
        let mut reweathered = flight(&plan, Some(bytes.clone()));
        reweathered.era5_sha256 = None;
        assert_eq!(
            refusal(&reweathered, &on_main).as_deref(),
            Some("its ERA5 file changed after the prediction")
        );
        // Every refused flight is named, not only the first.
        let error = refusals(&[flight(&plan, None), flight(&other, None)], &on_main)
            .expect_err("both refused");
        assert!(error.contains("invented-20270101") && error.contains("invented-20270102"));
    }

    /// The list read at main: a hash committed on this branch alone is not on it, one merged
    /// before the branch left main is, and one merged after it is not yet (a scratch repository,
    /// as CI's checkouts are one commit deep).
    #[test]
    fn the_list_is_read_where_the_branch_left_main() {
        let dir = tempfile::tempdir().expect("a scratch directory");
        let root = dir.path();
        let run = |args: &[&str]| {
            let status = Command::new("git")
                .current_dir(root)
                .args(args)
                .output()
                .expect("git");
            assert!(status.status.success(), "git {args:?}: {status:?}");
        };
        let write_list = |hashes: &[&str]| {
            fs::create_dir_all(root.join("validation/reports")).expect("dirs");
            let text = std::iter::once("# predictions".to_owned())
                .chain(hashes.iter().map(|hash| (*hash).to_owned()))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(root.join(LIST), text + "\n").expect("the list");
        };
        let (a, b, c) = ("a".repeat(64), "b".repeat(64), "c".repeat(64));
        run(&["init", "-q", "-b", "main"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "test"]);
        run(&["config", "commit.gpgsign", "false"]);
        // No list on main yet: an empty one.
        fs::write(root.join("README"), "x\n").expect("a file");
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "first"]);
        run(&["update-ref", "refs/remotes/origin/main", "HEAD"]);
        assert_eq!(list_at_main(root), Ok(BTreeSet::new()));
        // A merged on main; then a branch adds B.
        write_list(&[&a]);
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "a"]);
        run(&["update-ref", "refs/remotes/origin/main", "HEAD"]);
        run(&["switch", "-q", "-c", "compare"]);
        write_list(&[&a, &b]);
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "b"]);
        assert_eq!(list_at_main(root), Ok(BTreeSet::from([a.clone()])));
        // C merges on main after the branch left it: not on this branch's list.
        run(&["switch", "-q", "main"]);
        write_list(&[&a, &c]);
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "c"]);
        run(&["update-ref", "refs/remotes/origin/main", "HEAD"]);
        run(&["switch", "-q", "compare"]);
        assert_eq!(list_at_main(root), Ok(BTreeSet::from([a.clone()])));
        // On main itself, all it holds.
        run(&["switch", "-q", "main"]);
        assert_eq!(list_at_main(root), Ok(BTreeSet::from([a, c])));
    }

    /// A plan file holding a log's field, or any field that isn't a plan's, is refused; the
    /// allowed fields are exactly [`Plan`]'s.
    #[test]
    fn a_plan_holds_no_log() {
        let plan = invented("invented-20270101");
        let value = serde_json::to_value(&plan).expect("json");
        let keys: BTreeSet<&str> = value
            .as_object()
            .expect("an object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, PLAN_KEYS.into_iter().collect::<BTreeSet<_>>());
        assert_eq!(read_plan(&value.to_string()), Ok(plan));
        for key in [
            "logged_apogee_m",
            "logged_kind",
            "early_deployment",
            "excluded",
            "catalog_tier",
            "added_after_m2_7a",
        ] {
            let mut with = value.clone();
            with[key] = json!(1);
            let error = read_plan(&with.to_string()).expect_err(key);
            assert!(
                error.starts_with(&format!("`{key}` is not a field")),
                "{error}"
            );
        }
    }

    /// The list's lines: hashes and comments only.
    #[test]
    fn the_list_holds_hashes_only() {
        let a = "a".repeat(64);
        assert_eq!(
            parse_list(&format!("# a comment\n\n{a}\n{a}\n")),
            Ok(BTreeSet::from([a.clone()]))
        );
        assert!(parse_list(&format!("{a}  a note\n")).is_err());
        assert!(parse_list(&"A".repeat(64)).is_err());
        assert!(parse_list(&"a".repeat(63)).is_err());
    }

    /// The manifest's flights not marked new must be M2.7a's: a new flight left unmarked
    /// changes the digest, and an old one marked new changes the count.
    #[test]
    fn unmarked_flights_are_m2_7a_s() {
        let entry = |id: &str, added: bool| -> Entry {
            let mut value = serde_json::to_value(invented(id)).expect("json");
            for (key, field) in [
                ("logged_apogee_m", json!(1000.0)),
                ("logged_kind", json!("barometric")),
                ("early_deployment", json!(null)),
                ("excluded", json!(null)),
                ("excluded_class", json!(null)),
                ("catalog_tier", json!("A")),
                ("added_after_m2_7a", json!(added)),
            ] {
                value[key] = field;
            }
            serde_json::from_value(value).expect("an entry")
        };
        let old: Vec<Entry> = (0..TIER_A_ORK)
            .map(|n| entry(&format!("old-{n:03}"), false))
            .collect();
        let ids: Vec<String> = old.iter().map(|e| e.flight_id.clone()).collect();
        let committed = before_text(&crate::collection_boost::held_out_digest(
            ids.iter().map(String::as_str),
        ));
        assert_eq!(held_to_before(&old, &committed), Ok(()));
        // A new flight marked: still M2.7a's.
        let mut with_new = old.clone();
        with_new.push(entry("invented-20270101", true));
        assert_eq!(held_to_before(&with_new, &committed), Ok(()));
        // Unmarked: refused.
        let mut unmarked = old.clone();
        unmarked.push(entry("invented-20270101", false));
        assert!(held_to_before(&unmarked, &committed).is_err());
        // An old flight marked new: refused.
        let mut remarked = old.clone();
        remarked[0].added_after_m2_7a = true;
        assert!(held_to_before(&remarked, &committed).is_err());
        // The field defaults to false, and is read when set.
        assert!(!old[0].added_after_m2_7a);
        assert!(with_new[TIER_A_ORK].added_after_m2_7a);
    }
}
