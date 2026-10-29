//! The solid motors and data files [ThrustCurve.org](https://www.thrustcurve.org) lists that the
//! bundled catalog lacks ([M5.7d][m5-7d], `hpr motors update`; [ADR-288][adr-288] §4).
//!
//! `hpr_motor` compiles in ThrustCurve.org's whole solid catalog as of one snapshot
//! ([`hpr_motor::Catalog::bundled`]). [`update`] asks ThrustCurve's [search](crate::thrustcurve::Search)
//! for every motor at once, keeps the solid ones (single-use and reload), and compares them with
//! the bundle ([`Bundle`]). A motor is looked at more closely when its id isn't in the bundle,
//! when its record lists another number of data files than the bundle holds for it, or when its
//! record was updated on or after the snapshot's day. For each, it [downloads](crate::thrustcurve::Download)
//! the RASP and the RockSim files, and every file whose id the bundle doesn't hold is newer than
//! it. Each answer goes through the [`Client`], so it lands in the cache: the downloads are the
//! ones [`crate::on_demand::find`] reads, so a caller that then finds each newer motor by name
//! has it offline too.
//!
//! > **How far to trust it.** It finds what ThrustCurve's records say changed. A file replaced by
//! > another under the same motor, with the record's file count and update day unchanged, isn't
//! > found: no answer short of downloading every motor's files shows it. A motor the bundle holds
//! > keeps flying its bundled file; a newer file of it is only cached. Tested on ThrustCurve's
//! > answers recorded on 2026-10-29, which list nothing newer than the bundle, and on the same
//! > answers against a bundle with one motor and one file taken out, which finds exactly those.
//!
//! [m5-7d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m5-7d
//! [adr-288]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0288-every-solid-motor-ships-in-the-package.md

use std::collections::{BTreeMap, BTreeSet};

use hpr_motor::Catalog;
use serde::{Deserialize, Serialize};

use crate::thrustcurve::{
    self, DataFile, Download, Format, MAX_RESULTS, MotorRecord, Search, ThrustCurveError,
};
use crate::{Client, Fetched, Transport};

/// What a bundled catalog holds, as [`update`] compares it: its snapshot's day and, for each
/// motor's ThrustCurve id, the ids of the data files the snapshot listed for it, indexed or left
/// out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle {
    /// The snapshot's day, `YYYY-MM-DD`.
    pub captured: String,
    motors: BTreeMap<String, BTreeSet<String>>,
}

impl Bundle {
    /// What `catalog` holds. Ids are kept in lower case, as the API writes them.
    #[must_use]
    pub fn of(catalog: &Catalog) -> Self {
        let mut motors: BTreeMap<String, BTreeSet<String>> = catalog
            .motors
            .iter()
            .map(|motor| {
                let files = motor
                    .curves
                    .iter()
                    .map(|curve| curve.simfile_id.to_ascii_lowercase())
                    .collect();
                (motor.motor_id.to_ascii_lowercase(), files)
            })
            .collect();
        for out in &catalog.left_out {
            motors
                .entry(out.motor_id.to_ascii_lowercase())
                .or_default()
                .insert(out.simfile_id.to_ascii_lowercase());
        }
        Self {
            captured: catalog.snapshot.captured.clone(),
            motors,
        }
    }

    /// Takes a motor out, with its files, as if the snapshot hadn't listed it. Whether it held it.
    pub fn remove_motor(&mut self, motor_id: &str) -> bool {
        self.motors.remove(&motor_id.to_ascii_lowercase()).is_some()
    }

    /// Takes one data file out, as if the snapshot hadn't listed it. Whether it held it.
    pub fn remove_file(&mut self, simfile_id: &str) -> bool {
        let id = simfile_id.to_ascii_lowercase();
        self.motors.values_mut().any(|files| files.remove(&id))
    }

    /// How many motors it holds.
    #[must_use]
    pub fn motors(&self) -> usize {
        self.motors.len()
    }

    /// Whether ThrustCurve's record may list what the bundle lacks: a motor it doesn't hold,
    /// another number of files than it holds, or an update on or after the snapshot's day (the
    /// record gives a day, not a time, so that day counts as after).
    fn may_differ(&self, record: &MotorRecord) -> bool {
        let Some(files) = self.motors.get(&record.motor_id.to_ascii_lowercase()) else {
            return true;
        };
        let listed = record.data_files.unwrap_or(0);
        let updated = record
            .updated_on
            .as_deref()
            .is_some_and(|day| day >= self.captured.as_str());
        usize::try_from(listed).ok() != Some(files.len()) || updated
    }

    fn holds_file(&self, file: &DataFile) -> bool {
        self.motors
            .get(&file.motor_id.to_ascii_lowercase())
            .is_some_and(|files| files.contains(&file.simfile_id.to_ascii_lowercase()))
    }
}

/// A solid motor ThrustCurve lists with something the bundle lacks.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Newer {
    /// Its record, as the search answered it.
    pub record: MotorRecord,
    /// Whether the bundle lacks the motor itself, not only some of its files.
    pub new_motor: bool,
    /// Its RASP and RockSim files the bundle lacks, in the answers' order, RASP first.
    pub files: Vec<DataFile>,
}

/// What [`update`] found.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    /// The solid motors ThrustCurve lists.
    pub listed: usize,
    /// The motors whose files were downloaded, as [`Bundle`] picks them.
    pub looked_at: usize,
    /// The motors with something newer than the bundle, in the search's order.
    pub newer: Vec<Newer>,
    /// The bundle's motors ThrustCurve no longer lists, by id.
    pub unlisted: Vec<String>,
    /// Each answer read, the search first.
    pub fetched: Vec<Fetched>,
}

/// Compares ThrustCurve's solid catalog with `bundle`, as the module describes, and returns what
/// it lists that the bundle lacks.
///
/// # Errors
/// [`ThrustCurveError::Incomplete`] when the search returns fewer motors than match; any error of
/// [`thrustcurve::fetch_search`] or [`thrustcurve::fetch_download`], offline with
/// [`crate::NetError::NotCached`] for an answer not in the cache.
pub fn update<T: Transport>(
    client: &Client<T>,
    bundle: &Bundle,
    now_s: u64,
) -> Result<Update, ThrustCurveError> {
    let search = Search {
        max_results: Some(MAX_RESULTS),
        ..Search::default()
    };
    let (answer, got) = thrustcurve::fetch_search(client, &search, now_s)?;
    if !answer.is_complete() {
        return Err(ThrustCurveError::Incomplete {
            matches: answer.matches,
            found: answer.results.len(),
        });
    }
    let mut fetched = vec![got];
    let solid: Vec<MotorRecord> = answer
        .results
        .into_iter()
        .filter(|record| matches!(record.motor_type.as_deref(), Some("SU" | "reload")))
        .collect();
    let listed: BTreeSet<String> = solid
        .iter()
        .map(|record| record.motor_id.to_ascii_lowercase())
        .collect();
    let unlisted = bundle
        .motors
        .keys()
        .filter(|id| !listed.contains(*id))
        .cloned()
        .collect();
    let mut looked_at = 0;
    let mut newer = Vec::new();
    for record in &solid {
        if !bundle.may_differ(record) {
            continue;
        }
        looked_at += 1;
        let mut files = Vec::new();
        for format in [Format::Rasp, Format::RockSim] {
            let download = Download::new(&record.motor_id, format)?;
            let (answer, got) = thrustcurve::fetch_download(client, &download, now_s)?;
            fetched.push(got);
            files.extend(
                answer
                    .results
                    .into_iter()
                    .filter(|file| !bundle.holds_file(file)),
            );
        }
        let new_motor = !bundle
            .motors
            .contains_key(&record.motor_id.to_ascii_lowercase());
        if new_motor || !files.is_empty() {
            newer.push(Newer {
                record: record.clone(),
                new_motor,
                files,
            });
        }
    }
    Ok(Update {
        listed: solid.len(),
        looked_at,
        newer,
        unlisted,
        fetched,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, files: u32, updated: &str) -> MotorRecord {
        let json = format!(
            r#"{{"motorId":"{id}","manufacturer":"M","designation":"D","type":"SU",
                "dataFiles":{files},"updatedOn":"{updated}"}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    fn bundle() -> Bundle {
        let mut motors = BTreeMap::new();
        motors.insert(
            "aa".repeat(12),
            ["bb".repeat(12), "cc".repeat(12)].into_iter().collect(),
        );
        Bundle {
            captured: "2026-10-28".to_owned(),
            motors,
        }
    }

    /// A motor is looked at when the bundle lacks it, holds another number of its files, or its
    /// record was updated on the snapshot's day or later; not otherwise.
    #[test]
    fn a_motor_is_looked_at_only_when_its_record_may_differ() {
        let bundle = bundle();
        let held = "aa".repeat(12);
        assert!(!bundle.may_differ(&record(&held, 2, "2026-10-27")));
        assert!(bundle.may_differ(&record(&held, 3, "2026-10-27")));
        assert!(bundle.may_differ(&record(&held, 1, "2026-10-27")));
        assert!(bundle.may_differ(&record(&held, 2, "2026-10-28")));
        assert!(bundle.may_differ(&record(&held, 2, "2027-01-02")));
        assert!(bundle.may_differ(&record(&"dd".repeat(12), 0, "2019-04-17")));
        // Ids in either case are one motor.
        assert!(!bundle.may_differ(&record(&"AA".repeat(12), 2, "2026-10-27")));
    }

    /// Taking a motor or a file out says whether the bundle held it.
    #[test]
    fn removing_says_whether_it_was_held() {
        let mut bundle = bundle();
        assert!(bundle.remove_file(&"CC".repeat(12)));
        assert!(!bundle.remove_file(&"cc".repeat(12)));
        assert!(bundle.remove_motor(&"aa".repeat(12)));
        assert!(!bundle.remove_motor(&"aa".repeat(12)));
        assert_eq!(bundle.motors(), 0);
    }
}
