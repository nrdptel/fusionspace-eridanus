//! One motor found by name on [ThrustCurve.org](https://www.thrustcurve.org), with the curve file
//! to fly it by: for a simulator that lacks the motor's curve ([M4.5b][m4-5b], motors on demand;
//! [ADR-154][adr-154], the decision on fetching them).
//!
//! [`find`] asks ThrustCurve's [search](crate::thrustcurve::Search) for the name as a designation
//! (`F27R/L`), then, if no motor has that designation, as a common name (`F27`), each time with
//! the manufacturer when one is given. Exactly one solid motor must answer: two or more are
//! [`FindError::Ambiguous`], listed by maker and designation so the caller can say which, and a
//! hybrid is refused, as HPR Sim flies commercial solid motors only. It then
//! [downloads](crate::thrustcurve::Download) the motor's RASP (`.eng`) files and takes the first
//! that reads as one motor, ranked by who measured it ([`rank`]): a certification test, then the
//! manufacturer, then a user, then a file that names no source, in the answer's order within each.
//! With no RASP file that reads, it does the same with the RockSim (`.rse`) files. A file the
//! caller names ([`Wanted::prefer`]) goes ahead of that order when it reads, the RockSim files
//! read too when the RASP ones lack it (offline with no copy of them, the RASP files are taken
//! as if none were named). [`find_with`]
//! takes the caller's own reading of a file instead, so the file taken is the first the caller
//! can use: a simulator that builds a motor from the file's masses refuses more than
//! `hpr_motor`'s reader does.
//!
//! Every answer goes through the [`Client`], so it is cached ([`crate::thrustcurve::TTL_S`], a
//! day) and, offline, read from the cache alone: a motor found once is found again with no
//! network. Nothing is bundled: ThrustCurve states no license for its records, and each file
//! carries its own ([`DataFile::license`]), which the caller shows.
//!
//! > **How far to trust it.** The curve is the file a contributor uploaded, as ThrustCurve serves
//! > it; which file is taken is the rule above, not a judgement of which is right. A name is matched
//! > as ThrustCurve's search matches it (on 2026-10-04, a designation exactly but for case), so a
//! > name ThrustCurve spells differently is not found rather than wrongly matched.
//!
//! [m4-5b]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m4-5b
//! [adr-154]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0154-motors-fetched-from-thrustcurve-by-name.md

use serde::{Deserialize, Serialize};

use crate::thrustcurve::{
    self, DataFile, Download, Format, MotorRecord, Search, SearchAnswer, ThrustCurveError,
};
use crate::{Client, Fetched, NetError, Transport};

/// The motor to find: a designation or common name, and its manufacturer if known.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wanted {
    /// The manufacturer's name or abbreviation, as ThrustCurve's search takes it (`AeroTech`).
    pub manufacturer: Option<String>,
    /// The designation (`F27R/L`) or common name (`F27`).
    pub name: String,
    /// A data file to take ahead of [`rank`]'s order when the motor's answer holds it and it
    /// reads: its ThrustCurve id ([`DataFile::simfile_id`]). A caller that knows which file holds
    /// the curve it wants, such as the one another simulator flies, names it here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer: Option<String>,
}

impl Wanted {
    /// A motor by its designation or common name alone.
    #[must_use]
    pub fn named(name: &str) -> Self {
        Self {
            manufacturer: None,
            name: name.to_owned(),
            prefer: None,
        }
    }

    /// A motor by its manufacturer and its designation or common name.
    #[must_use]
    pub fn by(manufacturer: &str, name: &str) -> Self {
        Self {
            manufacturer: Some(manufacturer.to_owned()),
            name: name.to_owned(),
            prefer: None,
        }
    }

    /// The same motor, taking the data file `simfile_id` first when the answer holds it
    /// ([`Wanted::prefer`]).
    #[must_use]
    pub fn preferring(mut self, simfile_id: &str) -> Self {
        self.prefer = Some(simfile_id.to_owned());
        self
    }

    /// The motor in words: `AeroTech F27R/L`, or the name alone.
    #[must_use]
    pub fn words(&self) -> String {
        match &self.manufacturer {
            Some(maker) => format!("{maker} {}", self.name),
            None => self.name.clone(),
        }
    }
}

/// A motor found, with the file to fly it by.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Found {
    /// ThrustCurve's record of the motor.
    pub record: MotorRecord,
    /// The data file taken ([`find`]'s rule): its format, source, license and text.
    pub file: DataFile,
    /// Every answer read, in the order asked, each saying whether it came from the network or
    /// the cache.
    pub fetched: Vec<Fetched>,
}

/// Why no motor was found.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum FindError {
    /// No motor has the name as its designation or its common name.
    #[error("ThrustCurve.org has no motor whose designation or common name is {wanted}")]
    NotFound {
        /// The motor asked for, in words.
        wanted: String,
    },
    /// Several solid motors answer the name.
    #[error("{matches} motors on ThrustCurve.org answer {wanted}: {}", candidates.join(", "))]
    Ambiguous {
        /// The motor asked for, in words.
        wanted: String,
        /// How many motors the search says match.
        matches: u32,
        /// Those the answer lists, each as `maker designation`.
        candidates: Vec<String>,
    },
    /// The one motor that answers is a hybrid.
    #[error("{motor} is a hybrid on ThrustCurve.org; HPR Sim flies commercial solid motors only")]
    Hybrid {
        /// The motor, as `maker designation`.
        motor: String,
    },
    /// The motor has no RASP or RockSim file that `hpr_motor` reads.
    #[error("{motor} has no RASP or RockSim file on ThrustCurve.org that HPR Sim reads{}", why.as_ref().map(|why| format!(": {why}")).unwrap_or_default())]
    NoFile {
        /// The motor, as `maker designation`.
        motor: String,
        /// Why the last file tried was refused; `None` when there were none.
        why: Option<String>,
    },
    /// A request or answer was refused, or the fetch failed.
    #[error(transparent)]
    ThrustCurve(#[from] ThrustCurveError),
}

impl FindError {
    /// Whether it failed only because, offline, an answer is not in the cache: a fetch with a
    /// network connection would answer.
    #[must_use]
    pub fn is_not_cached(&self) -> bool {
        matches!(
            self,
            Self::ThrustCurve(ThrustCurveError::Net(NetError::NotCached { .. }))
        )
    }
}

/// Where a data file's source puts it in [`find`]'s order: `cert` (a certification test) 0,
/// `mfr` (the manufacturer) 1, `user` 2, anything else or none 3.
#[must_use]
pub fn rank(file: &DataFile) -> u8 {
    match file.source.as_deref() {
        Some("cert") => 0,
        Some("mfr") => 1,
        Some("user") => 2,
        _ => 3,
    }
}

/// Whether a data file reads as exactly one motor's curve with `hpr_motor`'s reader: [`find`]'s
/// test of a file.
///
/// # Errors
/// Why not, in words: the file doesn't read, holds no motor or several, or its curve breaks
/// `hpr_motor`'s rules.
pub fn reads_as_one_motor(file: &DataFile) -> Result<(), String> {
    let curve = file.read().map_err(|error| error.to_string())?;
    let motors = match &curve {
        thrustcurve::Curve::Rasp(parsed) => parsed.value.entries.len(),
        thrustcurve::Curve::RockSim(parsed) => parsed.value.engines.len(),
    };
    if motors != 1 {
        return Err(format!("the file holds {motors} motors, not one"));
    }
    curve
        .thrust_curve()
        .map(drop)
        .map_err(|error| error.to_string())
}

/// Finds `wanted` on ThrustCurve.org through `client`, and the file to fly it by, as the module
/// says: the first file that [`reads_as_one_motor`].
///
/// # Errors
/// [`FindError::NotFound`], [`FindError::Ambiguous`] or [`FindError::Hybrid`] by the search;
/// [`FindError::NoFile`] when no file reads; [`FindError::ThrustCurve`] when a fetch fails or an
/// answer is refused, offline with [`NetError::NotCached`] for an answer not in the cache
/// ([`FindError::is_not_cached`]).
pub fn find<T: Transport>(
    client: &Client<T>,
    wanted: &Wanted,
    now_s: u64,
) -> Result<Found, FindError> {
    find_with(client, wanted, now_s, reads_as_one_motor).map(|(found, ())| found)
}

/// As [`find`], with `build` reading a file instead: the file taken is the first, in [`find`]'s
/// order, that `build` turns into an `M`, returned with it. `build`'s refusal of the last file
/// tried is [`FindError::NoFile`]'s reason.
///
/// # Errors
/// As [`find`].
pub fn find_with<T: Transport, M>(
    client: &Client<T>,
    wanted: &Wanted,
    now_s: u64,
    build: impl Fn(&DataFile) -> Result<M, String>,
) -> Result<(Found, M), FindError> {
    let mut fetched = Vec::new();
    let mut search = |designation: bool| -> Result<SearchAnswer, FindError> {
        let name = Some(wanted.name.clone());
        let request = Search {
            manufacturer: wanted.manufacturer.clone(),
            designation: if designation { name.clone() } else { None },
            common_name: if designation { None } else { name },
            ..Search::default()
        };
        let (answer, got) = thrustcurve::fetch_search(client, &request, now_s)?;
        fetched.push(got);
        Ok(answer)
    };
    let mut answer = search(true)?;
    if answer.results.is_empty() {
        answer = search(false)?;
    }
    let record = one(answer, wanted)?;
    let motor = maker_and_designation(&record);
    let mut queue = ranked(client, &record, Format::Rasp, now_s, &mut fetched)?;
    let mut rest = Some(Format::RockSim);
    if let Some(id) = &wanted.prefer {
        // A preferred file the RASP answer lacks is looked for in the RockSim one too, ahead of
        // every RASP file. Offline with no copy of that answer, the RASP files are taken as
        // before: the preference is given up, not the motor.
        if !queue.iter().any(|file| file.simfile_id == *id) {
            match ranked(client, &record, Format::RockSim, now_s, &mut fetched) {
                Ok(files) => {
                    queue.extend(files);
                    rest = None;
                }
                Err(error) if error.is_not_cached() && !queue.is_empty() => {}
                Err(error) => return Err(error),
            }
        }
        // The preferred file goes first; the rest keep their order behind it.
        if let Some(at) = queue.iter().position(|file| file.simfile_id == *id) {
            queue[..=at].rotate_right(1);
        }
    }
    let mut why = None;
    loop {
        for file in queue {
            match build(&file) {
                Ok(built) => {
                    let found = Found {
                        record,
                        file,
                        fetched,
                    };
                    return Ok((found, built));
                }
                Err(error) => why = Some(format!("{}: {error}", file.simfile_id)),
            }
        }
        match rest.take() {
            Some(format) => queue = ranked(client, &record, format, now_s, &mut fetched)?,
            None => return Err(FindError::NoFile { motor, why }),
        }
    }
}

/// The motor's files in `format`, in [`rank`]'s order, the answer's within a rank; the answer read
/// is added to `fetched`.
fn ranked<T: Transport>(
    client: &Client<T>,
    record: &MotorRecord,
    format: Format,
    now_s: u64,
    fetched: &mut Vec<Fetched>,
) -> Result<Vec<DataFile>, FindError> {
    let download = Download::new(&record.motor_id, format)?;
    let (files, got) = thrustcurve::fetch_download(client, &download, now_s)?;
    fetched.push(got);
    let mut files = files.results;
    // Stable: within a rank, the answer's order.
    files.sort_by_key(rank);
    Ok(files)
}

/// The one solid motor a search answers, or why there isn't one.
fn one(answer: SearchAnswer, wanted: &Wanted) -> Result<MotorRecord, FindError> {
    let hybrid = |record: &MotorRecord| {
        record
            .motor_type
            .as_deref()
            .is_some_and(|kind| kind.eq_ignore_ascii_case("hybrid"))
    };
    let (hybrids, solids): (Vec<MotorRecord>, Vec<MotorRecord>) =
        answer.results.into_iter().partition(hybrid);
    let unlisted = usize::try_from(answer.matches)
        .unwrap_or(usize::MAX)
        .saturating_sub(hybrids.len() + solids.len());
    match (&solids[..], unlisted) {
        ([record], 0) => Ok(record.clone()),
        ([], 0) => match &hybrids[..] {
            [] => Err(FindError::NotFound {
                wanted: wanted.words(),
            }),
            [hybrid, ..] => Err(FindError::Hybrid {
                motor: maker_and_designation(hybrid),
            }),
        },
        _ => {
            let mut candidates: Vec<String> = solids.iter().map(maker_and_designation).collect();
            candidates.sort();
            Err(FindError::Ambiguous {
                wanted: wanted.words(),
                matches: answer.matches,
                candidates,
            })
        }
    }
}

/// `AeroTech F27R/L`: the short maker name where the record has one.
fn maker_and_designation(record: &MotorRecord) -> String {
    let maker = record
        .manufacturer_abbrev
        .as_deref()
        .unwrap_or(&record.manufacturer);
    format!("{maker} {}", record.designation)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(source: Option<&str>) -> DataFile {
        serde_json::from_value(serde_json::json!({
            "motorId": "e00000000000000000000001",
            "simfileId": "f00000000000000000000001",
            "format": "RASP",
            "source": source,
            "data": "",
        }))
        .unwrap()
    }

    /// A certification test's file ranks first, then the manufacturer's, a user's, and one with
    /// no source or an unknown one.
    #[test]
    fn files_rank_by_who_measured_them() {
        let ranks: Vec<u8> = [Some("cert"), Some("mfr"), Some("user"), None, Some("other")]
            .into_iter()
            .map(|source| rank(&file(source)))
            .collect();
        assert_eq!(ranks, [0, 1, 2, 3, 3]);
    }

    #[test]
    fn a_motor_in_words_names_its_maker_when_known() {
        assert_eq!(Wanted::named("F27R/L").words(), "F27R/L");
        assert_eq!(Wanted::by("AeroTech", "F27R/L").words(), "AeroTech F27R/L");
    }
}
