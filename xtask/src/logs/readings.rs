//! The readings (M7.2b): every record the survey reads is read by `hpr_flightdata::readings`,
//! and each reading given or withheld is counted by family and by reason. Each reason has a
//! case: the corpus's first record withheld for it, by file id, or, where the corpus has none, an
//! invented sample committed under `validation/fixtures/logs/readings/`, which a test reads to
//! that reason with no `refs/` ([ADR-281][adr-281]).
//!
//! [adr-281]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0281-readings-on-every-logger.md

use std::collections::BTreeMap;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::{self, Reading, Readings, Reason};
use serde::{Deserialize, Serialize};

/// The folder of the invented samples, under the repository.
pub(crate) const INVENTED_DIR: &str = "validation/fixtures/logs/readings";

/// The invented sample of each reason the corpus has no case of, by file name under
/// [`INVENTED_DIR`]: a log in a format the importer reads, or, for a record no reader can make,
/// the record itself as JSON (`.record.json`). The folder's `make.py` writes them.
pub(crate) const INVENTED: [(Reason, &str); 5] = [
    (Reason::BadRecord, "bad-record.record.json"),
    (Reason::FasterThanFreeFall, "faster-than-free-fall.csv"),
    (Reason::SampledTooFast, "sampled-too-fast.csv"),
    (Reason::StartsOffThePad, "starts-off-the-pad.csv"),
    (Reason::StepTooCoarse, "step-too-coarse.csv"),
];

/// The record an invented sample holds: the JSON of a `.record.json`, else the file read by the
/// importer.
pub(crate) fn invented_record(name: &str, bytes: &[u8]) -> Result<FlightLog, String> {
    if name.ends_with(".record.json") {
        serde_json::from_slice(bytes).map_err(|error| format!("{name}: {error}"))
    } else {
        hpr_flightdata::import::read(bytes).map_err(|error| format!("{name}: {error}"))
    }
}

/// The readings of every record of a family.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct FamilyReadings {
    /// Records read: a file's, or each sheet's or recording's of a file that holds several.
    pub records: usize,
    /// Records by the height track their readings took (`barometer`, `pressure`,
    /// `altitude_channel`, `gnss`), or `none`.
    pub tracks: BTreeMap<String, usize>,
    /// Each reading, by its name: how many records gave it, and withheld it by reason.
    pub readings: BTreeMap<String, ReadingCounts>,
}

/// One reading's outcomes over a family's records.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct ReadingCounts {
    /// Records that gave it.
    pub given: usize,
    /// Records that withheld it, by reason.
    pub withheld: BTreeMap<String, usize>,
}

/// A reason's case.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct ReasonCase {
    /// The reason, as the readings name it.
    pub reason: String,
    /// Records the corpus withholds a reading of for it.
    pub records: usize,
    /// The first of their files by id, if any.
    pub first: Option<String>,
    /// The invented sample that stands in, where the corpus has none, under [`INVENTED_DIR`].
    pub invented: Option<String>,
}

/// The readings over the corpus.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct Survey {
    /// By family.
    pub families: BTreeMap<String, FamilyReadings>,
    /// Every reason, in the order the readings declare them.
    pub reasons: Vec<ReasonCase>,
}

/// The survey being gathered.
#[derive(Default)]
pub(super) struct Gathering {
    families: BTreeMap<String, FamilyReadings>,
    cases: BTreeMap<Reason, (usize, String)>,
}

/// A reason's code, as the JSON names it.
pub(crate) fn code(reason: Reason) -> String {
    serde_json::to_value(reason)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default()
}

/// Every reading of `read`, by name, with why it was withheld if it was.
pub(crate) fn outcomes(read: &Readings) -> [(&'static str, Option<Reason>); 5] {
    fn reason<T>(reading: &Reading<T>) -> Option<Reason> {
        reading.reason()
    }
    [
        ("liftoff", reason(&read.liftoff)),
        ("apogee", reason(&read.apogee)),
        ("max_speed", reason(&read.max_speed)),
        ("max_acceleration", reason(&read.max_acceleration)),
        ("landing", reason(&read.landing)),
    ]
}

impl Gathering {
    /// Reads `log`, a record of file `id` of `family`, and counts its readings.
    pub(super) fn add(&mut self, family: &str, id: &str, log: &FlightLog) {
        let read = readings::read(log);
        let row = self.families.entry(family.to_owned()).or_default();
        row.records += 1;
        let track = read.source.map_or_else(
            || "none".to_owned(),
            |source| {
                serde_json::to_value(source)
                    .ok()
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default()
            },
        );
        *row.tracks.entry(track).or_default() += 1;
        let mut reasons = Vec::new();
        for (name, reason) in outcomes(&read) {
            let counts = row.readings.entry(name.to_owned()).or_default();
            match reason {
                None => counts.given += 1,
                Some(reason) => {
                    *counts.withheld.entry(code(reason)).or_default() += 1;
                    if !reasons.contains(&reason) {
                        reasons.push(reason);
                    }
                }
            }
        }
        for reason in reasons {
            let case = self
                .cases
                .entry(reason)
                .or_insert_with(|| (0, id.to_owned()));
            case.0 += 1;
            if id < case.1.as_str() {
                id.clone_into(&mut case.1);
            }
        }
    }

    /// The survey, each reason with its case.
    pub(super) fn finish(self) -> Survey {
        let reasons = Reason::ALL
            .iter()
            .map(|&reason| {
                let (records, first) = self
                    .cases
                    .get(&reason)
                    .map_or((0, None), |(n, id)| (*n, Some(id.clone())));
                let invented = (records == 0)
                    .then(|| {
                        INVENTED
                            .iter()
                            .find(|(r, _)| *r == reason)
                            .map(|(_, file)| (*file).to_owned())
                    })
                    .flatten();
                ReasonCase {
                    reason: code(reason),
                    records,
                    first,
                    invented,
                }
            })
            .collect();
        Survey {
            families: self.families,
            reasons,
        }
    }
}

/// Each reason with neither a case in the corpus nor an invented sample that reads to it, as the
/// survey's unread lines: the done-when asks for a case of every reason. Each invented sample the
/// survey lists is read from under `root` and must withhold a reading for its reason.
pub(super) fn unread(survey: &Survey, root: &std::path::Path) -> Vec<String> {
    let mut unread = Vec::new();
    for case in survey.reasons.iter().filter(|case| case.records == 0) {
        let Some(name) = &case.invented else {
            unread.push(format!(
                "  the readings' reason `{}` has no case in the corpus and no invented sample: \
                 add one under {INVENTED_DIR} and to `INVENTED`",
                case.reason
            ));
            continue;
        };
        let read = std::fs::read(root.join(INVENTED_DIR).join(name))
            .map_err(|error| format!("{name}: {error}"))
            .and_then(|bytes| invented_record(name, &bytes));
        let reasons = match read {
            Ok(log) => outcomes(&readings::read(&log)).map(|(_, reason)| reason.map(code)),
            Err(error) => {
                unread.push(format!("  the invented sample {error}"));
                continue;
            }
        };
        if !reasons.contains(&Some(case.reason.clone())) {
            unread.push(format!(
                "  the invented sample {name} doesn't withhold a reading for `{}`",
                case.reason
            ));
        }
    }
    unread
}

/// The page's section on the readings.
pub(super) fn markdown(survey: &Survey) -> String {
    let mut out = String::from(
        "\n## Readings\n\n\
         Every record read, taken by `hpr_flightdata::readings` (M7.2b): for each family, its \
         records by the height track the readings took, then each reading given, and withheld \
         by reason. A file that holds several records, a workbook's sheets or a dump's \
         recordings, counts each.\n\n\
         | Family | Records | Tracks | Liftoff | Apogee | Top speed | Top acceleration | Landing |\n\
         |---|---:|---|---|---|---|---|---|\n",
    );
    let cell = |counts: Option<&ReadingCounts>| {
        let Some(counts) = counts else {
            return "none".to_owned();
        };
        let mut parts = vec![format!("{} given", counts.given)];
        parts.extend(
            counts
                .withheld
                .iter()
                .map(|(reason, n)| format!("{n} `{reason}`")),
        );
        parts.join(", ")
    };
    for (family, row) in &survey.families {
        let tracks: Vec<String> = row
            .tracks
            .iter()
            .map(|(track, n)| format!("{n} {track}"))
            .collect();
        out += &format!(
            "| {family} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.records,
            tracks.join(", "),
            cell(row.readings.get("liftoff")),
            cell(row.readings.get("apogee")),
            cell(row.readings.get("max_speed")),
            cell(row.readings.get("max_acceleration")),
            cell(row.readings.get("landing")),
        );
    }
    out += &format!(
        "\nEach reason with its case: the records the corpus withholds a reading of for it, and \
         the first of their files by id; where the corpus has none, the invented sample under \
         `{INVENTED_DIR}` that a test reads to it.\n\n\
         | Reason | Records | First file | Invented sample |\n|---|---:|---|---|\n"
    );
    for case in &survey.reasons {
        out += &format!(
            "| `{}` | {} | {} | {} |\n",
            case.reason,
            case.records,
            case.first.as_deref().map_or("none".to_owned(), |id| format!("`{id}`")),
            case.invented
                .as_deref()
                .map_or("none".to_owned(), |file| format!("`{file}`")),
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(INVENTED_DIR)
    }

    /// Each invented sample, read with no `refs/`, withholds a reading for its reason, so the
    /// survey's case for it is a real one.
    #[test]
    fn each_invented_sample_reads_to_its_reason() {
        for (reason, name) in INVENTED {
            let bytes = std::fs::read(folder().join(name)).unwrap();
            let log = invented_record(name, &bytes).unwrap();
            let read = readings::read(&log);
            let reasons: Vec<Option<Reason>> = outcomes(&read).iter().map(|(_, r)| *r).collect();
            assert!(reasons.contains(&Some(reason)), "{name}: {reasons:?}");
        }
    }

    /// The folder holds the samples `INVENTED` lists and the script that writes them, nothing
    /// else, and no reason twice.
    #[test]
    fn the_folder_holds_what_invented_lists() {
        let mut found: Vec<String> = std::fs::read_dir(folder())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name != "make.py")
            .collect();
        found.sort();
        let mut listed: Vec<String> = INVENTED.iter().map(|(_, name)| (*name).to_owned()).collect();
        listed.sort();
        assert_eq!(found, listed);
        let mut reasons: Vec<Reason> = INVENTED.iter().map(|(reason, _)| *reason).collect();
        reasons.sort();
        reasons.dedup();
        assert_eq!(reasons.len(), INVENTED.len());
    }
}
