//! `cargo xtask fixture-flights` also seals the private collection's held-out half and reads each
//! of its flights' logs up to the boost's cut (M7.3d4, ADR-322, ADR-326):
//! [`HELD_OUT`] and `validation/reports/collection-boost.{json,md}`.
//!
//! **The halves.** A flight is held out when SHA-256 of its manifest `flight_id`'s UTF-8 bytes
//! followed directly by the seven bytes `ADR-209` starts with eight bytes that, read as a
//! big-endian unsigned integer, are below 2⁶⁴ / 3 ([`held_out`], ADR-322 §2, the encoding of
//! ADR-209 §4's rule). [`HELD_OUT`] holds the SHA-256 of the held-out ids, sorted bytewise, one
//! per line with a newline after each ([`held_out_digest`]). It is committed before any half's
//! number is: this report gives none. The ids stay private (ADR-151).
//!
//! **Each flight's log.** Of a flight's logs in the collection's catalog, outside its unpinned
//! `_scratch/` folder, each is read alone by `hpr_flightdata::import::read_one`, with no sheet or
//! recording chosen and no sibling file. Of those whose height track is barometric (the logger's
//! altitude, a pressure or a barometric altitude channel) and gives the analyzer an apogee, the
//! flight's log is the one whose apogee reading is nearest the manifest's logged apogee, the first
//! in the catalog's order on a tie ([`choose`]). It is cut by `hpr_forensics::boost::try_cut`, or
//! refused for the first reason the finder meets. Only counts over the whole collection are
//! written; nothing per flight or per half.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_flightdata::import;
use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::{self, Source};
use hpr_forensics::boost::{self, NoCut};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::collection_errors::{CutFlight, Predictions};
use crate::fixture_flights::{Entry, ForwardFlight};

/// The SHA-256 of the held-out flight ids (ADR-322 §2), committed before any half's number.
pub const HELD_OUT: &str = "validation/reports/held-out.sha256";
/// The report's data: counts only.
pub const REPORT_JSON: &str = "validation/reports/collection-boost.json";
/// The report's page.
pub const REPORT_MD: &str = "validation/reports/collection-boost.md";
/// The collection's catalog of logs, one row a file.
const CATALOG: &str = "refs/hpr-sim-fixtures/catalog/logs.csv";
/// The collection's checkout; the catalog's paths are relative to it.
const COLLECTION: &str = "refs/hpr-sim-fixtures";
/// The catalog's unpinned folder (as `cargo xtask logs` leaves it out).
const SCRATCH: &str = "_scratch/";
/// ADR-209 §4's seed, appended to each id before hashing.
const SEED: &[u8] = b"ADR-209";
/// The largest leading eight bytes of a held-out flight's digest: ⌊(2⁶⁴ − 1) / 3⌋ =
/// 6,148,914,691,236,517,205, the largest integer below 2⁶⁴ / 3 (2⁶⁴ − 1 is divisible by 3).
const HELD_OUT_MOST: u64 = u64::MAX / 3;

/// Whether a flight is held out (ADR-322 §2): SHA-256 of its id's UTF-8 bytes then `ADR-209`,
/// its first eight bytes big-endian below 2⁶⁴ / 3.
pub fn held_out(flight_id: &str) -> bool {
    leading_is_held_out(leading(flight_id))
}

/// The flights a fit across the collection's flights may take: the calibration half's only
/// (ADR-209 §4: held-out flights never feed a fit or tuning across flights, or a choice of
/// model; M2.7a, ADR-332).
///
/// It is built only by [`Calibration::of`], which reads each flight's half from its id again,
/// whatever its caller's own filter said, and refuses the whole set when any flight is held out.
/// A fit across flights must take a `Calibration`, so a held-out flight fed to one stops
/// `cargo xtask fixture-flights`, and with it `--check`, the gate's `refs` step; the guard checks
/// only the fits that take it. Today two take one: the calibration half's fit aggregates (the
/// drag, impulse and structure-mass scales M7.5's correction, for drag and impulse, would be
/// chosen from, ADR-322 §2; that correction must take one too), and the fixture report's
/// calibration half, the threshold the scoreboard holds its collection rows by (ADR-332 §4).
#[derive(Debug)]
pub struct Calibration<T>(Vec<T>);

impl<T> Calibration<T> {
    /// The flights, each beside its manifest `flight_id`; an error naming how many are held out
    /// (not which: the ids are private, ADR-151) when any is.
    pub fn of<'a>(flights: impl IntoIterator<Item = (&'a str, T)>) -> Result<Self, String> {
        let mut kept = Vec::new();
        let mut held = 0_usize;
        for (flight_id, flight) in flights {
            if held_out(flight_id) {
                held += 1;
            } else {
                kept.push(flight);
            }
        }
        if held > 0 {
            return Err(format!(
                "{held} held-out flight{} fed a fit across flights: only the calibration half may \
                 (ADR-209 §4, ADR-322 §2)",
                if held == 1 { "" } else { "s" }
            ));
        }
        Ok(Self(kept))
    }

    /// The flights, in the order given.
    pub fn flights(&self) -> &[T] {
        &self.0
    }
}

/// The first eight bytes of SHA-256 of the id's UTF-8 bytes then `ADR-209`, read big-endian.
fn leading(flight_id: &str) -> u64 {
    let digest = Sha256::new()
        .chain_update(flight_id.as_bytes())
        .chain_update(SEED)
        .finalize();
    let mut first = [0u8; 8];
    first.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(first)
}

/// Whether a height track is a barometer's: the logger's altitude, a pressure, or a barometric
/// altitude channel. A GNSS receiver's is not, nor no track.
fn barometric(source: Option<Source>) -> bool {
    matches!(
        source,
        Some(Source::Barometer | Source::Pressure | Source::AltitudeChannel)
    )
}

/// The bound on a digest's leading eight bytes, read big-endian.
fn leading_is_held_out(leading: u64) -> bool {
    leading <= HELD_OUT_MOST
}

/// The SHA-256, as lowercase hex, of the given ids sorted bytewise, one per line with a newline
/// after each (ADR-322 §2). Repeated ids count once.
pub fn held_out_digest<'a>(ids: impl IntoIterator<Item = &'a str>) -> String {
    let sorted: std::collections::BTreeSet<&[u8]> = ids.into_iter().map(str::as_bytes).collect();
    let mut hasher = Sha256::new();
    for id in sorted {
        hasher.update(id);
        hasher.update(b"\n");
    }
    hasher
        .finalize()
        .iter()
        .fold(String::new(), |mut out, byte| {
            let _ = write!(out, "{byte:02x}");
            out
        })
}

/// [`HELD_OUT`]'s text: the digest, then what it is.
fn held_out_text(digest: &str) -> String {
    format!(
        "{digest}  SHA-256 of the private collection's held-out flight ids (ADR-322), sorted \
         bytewise, one per line with a newline after each\n"
    )
}

/// The report: counts over the whole collection, none per flight or per half.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// The milestone that wrote it.
    pub milestone: String,
    /// Its decision record.
    pub decision: String,
    /// The manifest's flights.
    pub flights: usize,
    /// Of those, the ones the manifest leaves out of the apogee errors.
    pub excluded: usize,
    /// The pinned files of the rest, each path counted once a flight.
    pub logs: usize,
    /// Of those, the ones read alone.
    pub logs_read: usize,
    /// Of those, the ones with an apogee reading on a barometric track.
    pub logs_with_apogee: usize,
    /// Flights none of whose pinned logs reads alone.
    pub no_log_read: usize,
    /// Flights with a log read, but none with an apogee on a barometric track.
    pub no_apogee: usize,
    /// Flights with a chosen log.
    pub chosen: usize,
    /// Of those, the ones whose chosen log's apogee reading is more than [`GAP_SHARE`] from the
    /// manifest's logged apogee.
    pub chosen_past_gap: usize,
    /// The chosen logs refused a cut, by the finder's reason.
    pub refused: BTreeMap<String, usize>,
    /// The chosen logs cut.
    pub cut: usize,
    /// Of those, the ones cut less than the flight's motor's burn after the log's liftoff.
    pub cut_in_burn: usize,
    /// Of those cut, the ones whose motor's burn or log's liftoff isn't known.
    pub cut_burn_unknown: usize,
    /// The cut flights' predictions and errors by halves (M7.3d6).
    pub predictions: Predictions,
}

/// The share of the logged apogee past which a chosen log's own apogee reading is counted: 2%, a
/// count of how often the nearest log is far from the logged number (a log of another altimeter
/// in the same rocket, or a reading the analyzer takes differently), not a filter. Within it the
/// log may still be another altimeter's: two in one rocket often agree that closely.
pub const GAP_SHARE: f64 = 0.02;

/// A log read: its own apogee reading and its liftoff on its clock.
struct Chosen {
    log: FlightLog,
    apogee_m: f64,
    liftoff_s: Option<f64>,
}

/// A refusal's name in the report.
fn refusal_name(why: &NoCut) -> &'static str {
    match why {
        NoCut::NoTrack => "no height track",
        NoCut::Gnss => "a GNSS track",
        NoCut::TooFast => "a log faster than the analyzer reads",
        NoCut::TooSparse => "a log slower than 8 Hz",
        NoCut::NeverLifted => "no liftoff",
        NoCut::Transonic { .. } => "the transonic limit before a cut",
        NoCut::NeverSlowed => "no fall in the climb speed",
    }
}

/// Of the logs read, the one whose apogee reading is nearest `logged_m`, the first on a tie.
fn choose<T>(read: Vec<T>, apogee_m: impl Fn(&T) -> f64, logged_m: f64) -> Option<T> {
    let mut best: Option<T> = None;
    for log in read {
        let nearer = best
            .as_ref()
            .is_none_or(|b| (apogee_m(&log) - logged_m).abs() < (apogee_m(b) - logged_m).abs());
        if nearer {
            best = Some(log);
        }
    }
    best
}

/// Whether a cut came less than the motor's burn after the log's liftoff; `None` where either
/// isn't known. Liftoff follows ignition, so this also takes in a cut up to that lag after the
/// burn ends, as the public flights' report counts it.
fn cut_in_burn(cut: &boost::Cut, liftoff_s: Option<f64>, burn_s: Option<f64>) -> Option<bool> {
    Some(cut.time_s - liftoff_s? < burn_s?)
}

/// The catalog's pinned logs of each flight, by flight id, in the catalog's order.
fn pinned_logs(root: &Path) -> Result<BTreeMap<String, Vec<String>>, String> {
    let text =
        fs::read_to_string(root.join(CATALOG)).map_err(|error| format!("{CATALOG}: {error}"))?;
    let mut records = crate::logs::csv_records(&text).into_iter();
    let headers = records.next().ok_or(format!("{CATALOG} is empty"))?;
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("{CATALOG} has no `{name}` column"))
    };
    let (path, flight) = (column("path")?, column("flight_id")?);
    let mut logs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in records {
        let (Some(file), Some(id)) = (row.get(path), row.get(flight)) else {
            return Err(format!("{CATALOG}: a row without its path or flight"));
        };
        if file.starts_with(SCRATCH) || id.is_empty() {
            continue;
        }
        let list = logs.entry(id.to_owned()).or_default();
        if !list.iter().any(|f| f == file) {
            list.push(file.to_owned());
        }
    }
    Ok(logs)
}

/// The counts, before the predictions.
struct Counts {
    report: Report,
    cuts: Vec<CutFlight>,
}

/// Reads the flights' logs and counts what comes of each, keeping each cut flight's log.
fn count(root: &Path, entries: &[Entry], burns: &BTreeMap<String, f64>) -> Result<Counts, String> {
    let logs = pinned_logs(root)?;
    let mut cuts = Vec::new();
    let mut report = Report {
        milestone: "M7.3d6".to_owned(),
        decision: "ADR-326".to_owned(),
        flights: entries.len(),
        excluded: 0,
        logs: 0,
        logs_read: 0,
        logs_with_apogee: 0,
        no_log_read: 0,
        no_apogee: 0,
        chosen: 0,
        chosen_past_gap: 0,
        refused: BTreeMap::new(),
        cut: 0,
        cut_in_burn: 0,
        cut_burn_unknown: 0,
        // Filled once the cut flights are predicted.
        predictions: Predictions::default(),
    };
    for entry in entries {
        if entry.excluded.is_some() {
            report.excluded += 1;
            continue;
        }
        let files = logs.get(&entry.flight_id).map_or(&[][..], Vec::as_slice);
        let mut any_read = false;
        let mut read = Vec::new();
        for file in files {
            report.logs += 1;
            let at = format!("{COLLECTION}/{file}");
            let bytes = fs::read(root.join(&at)).map_err(|error| format!("{at}: {error}"))?;
            let Ok(log) = import::read_one(&bytes, None, None) else {
                continue;
            };
            report.logs_read += 1;
            any_read = true;
            let readings = readings::read(&log);
            let Some(apogee) = readings.apogee.value() else {
                continue;
            };
            // A height that isn't a number, chosen first, would never be replaced by a nearer one.
            if !barometric(readings.source) || !apogee.height_m.is_finite() {
                continue;
            }
            report.logs_with_apogee += 1;
            read.push(Chosen {
                apogee_m: apogee.height_m,
                liftoff_s: readings.liftoff.value().map(|l| l.time_s),
                log,
            });
        }
        if !any_read {
            report.no_log_read += 1;
            continue;
        }
        // The candidates' apogees, to count where the choice had others to choose from (#1332).
        let (low, high) = read
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), c| {
                (l.min(c.apogee_m), h.max(c.apogee_m))
            });
        let other_logs = read.len() > 1;
        let other_logs_apart = other_logs && high - low > GAP_SHARE * entry.logged_apogee_m;
        let Some(chosen) = choose(read, |c| c.apogee_m, entry.logged_apogee_m) else {
            report.no_apogee += 1;
            continue;
        };
        report.chosen += 1;
        if (chosen.apogee_m - entry.logged_apogee_m).abs() > GAP_SHARE * entry.logged_apogee_m {
            report.chosen_past_gap += 1;
        }
        // Only the chosen log is cut.
        match &boost::try_cut(&chosen.log) {
            Ok(cut) => {
                report.cut += 1;
                let in_burn =
                    cut_in_burn(cut, chosen.liftoff_s, burns.get(&entry.flight_id).copied());
                match in_burn {
                    Some(true) => report.cut_in_burn += 1,
                    Some(false) => {}
                    None => report.cut_burn_unknown += 1,
                }
                cuts.push(CutFlight {
                    flight_id: entry.flight_id.clone(),
                    log: chosen.log,
                    cut_in_burn: in_burn,
                    other_logs,
                    other_logs_apart,
                });
            }
            Err(why) => {
                *report
                    .refused
                    .entry(refusal_name(why).to_owned())
                    .or_default() += 1
            }
        }
    }
    Ok(Counts { report, cuts })
}

impl Report {
    /// The page, written from the counts alone.
    pub fn to_markdown(&self) -> String {
        let mut page = String::new();
        let refused: usize = self.refused.values().sum();
        let _ = writeln!(page, "# The private collection's logs, cut at the boost\n");
        let _ = writeln!(
            page,
            "This page counts how many of the [private collection's](fixture-flights.md) flights \
             have a barometer's log that a prediction from the boost can start from: the log cut \
             after burnout, fitted, and flown on to apogee, as the \
             [public flights' report](from-boost.md) does for 7 public flights. Then it gives \
             those predictions' apogee errors for each half of the flights, beside the forward \
             simulation's on the same flights. The collection's license allows only aggregates, \
             so no flight is named.\n"
        );
        let _ = writeln!(
            page,
            "Written by `cargo xtask fixture-flights`; do not edit. It is the third of four steps \
             toward the collection's numbers for apogee from the boost ({}, decision record \
             [{}](../../docs/decisions/0326-m7-3d3-in-four-the-halves-sealed-first.md); this \
             step's choices are in \
             [ADR-329](../../docs/decisions/0329-m7-3d6-the-collections-errors-by-halves.md)).\n",
            self.milestone, self.decision
        );
        let _ = writeln!(
            page,
            "**The halves.** The predictions' errors are given for two halves: a held-out \
             share of the flights, kept from every later choice of model \
             ([ADR-209 §4](../../docs/decisions/0209-the-2026-10-08-best-on-the-market-measured.md)), \
             and the rest. The collection's [forward errors](fixture-flights.md#apogee-errors) \
             (hpr's simulated apogee against the logged one) were published before the held-out \
             digest was sealed on 2026-11-07, so the model choices made before then could draw on \
             every flight: the held-out flights are kept only from choices made after it (held \
             out, not blind). A flight is held out when a hash of its id puts it there: the first \
             eight bytes of the SHA-256 of the id followed by `ADR-209`, read as a big-endian number, fall \
             below 2⁶⁴ / 3, a third of the flights in expectation \
             ([ADR-322](../../docs/decisions/0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md)). \
             The held-out ids stay private \
             ([ADR-151](../../docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md)). The SHA-256 \
             of their sorted ids is committed in [`held-out.sha256`](held-out.sha256) before any \
             half's number, so any later change to which flights are held out shows as a change \
             to that hash. This page gives each half's errors, and the calibration half's fits as \
             aggregates.\n"
        );
        let _ = writeln!(
            page,
            "**Each flight's log.** A flight may have several logs, from more than one \
             altimeter or file.\n\n\
             - **Which files:** those the collection's catalog lists for the flight, outside its \
             `_scratch/` folder, which the collection's pinned version doesn't fix. Each is read \
             on its own: a workbook without exactly one sheet that reads as a log, or a log that \
             needs a second file or a stated column layout to read, counts as unread. A table whose \
             altitude column doesn't say which sensor measured it reads, but gives no apogee, so \
             it can't be chosen.\n\
             - **Which log:** of those whose height track is a barometer's (the logger's \
             altitude, a pressure, or a barometric altitude channel, never a GNSS receiver's) \
             and gives an apogee, the one whose apogee is nearest the logged apogee the \
             collection states; on a tie, the first in the catalog's order.\n\
             - **The cut:** the library's boost finder cuts it where its climb speed falls from \
             its top, or says why it can't.\n"
        );
        let _ = writeln!(page, "| | flights |\n|---|---:|");
        let _ = writeln!(page, "| in the collection's manifest | {} |", self.flights);
        let _ = writeln!(
            page,
            "| left out of the apogee errors by the manifest | {} |",
            self.excluded
        );
        let _ = writeln!(page, "| no file reads on its own | {} |", self.no_log_read);
        let _ = writeln!(
            page,
            "| a file read, but none gives an apogee on a barometer's track | {} |",
            self.no_apogee
        );
        let _ = writeln!(page, "| a log chosen | {} |", self.chosen);
        let _ = writeln!(page, "| chosen, then refused a cut | {refused} |");
        for (why, n) in &self.refused {
            let _ = writeln!(page, "| … refused: {why} | {n} |");
        }
        let _ = writeln!(page, "| chosen and cut | {} |", self.cut);
        let _ = writeln!(
            page,
            "| … cut less than the motor's burn after liftoff | {} |",
            self.cut_in_burn
        );
        let _ = writeln!(
            page,
            "| … cut, the motor's burn or the log's liftoff not known | {} |",
            self.cut_burn_unknown
        );
        let _ = writeln!(
            page,
            "\nOf the flights not left out, {} files were tried, {} read on their own and {} gave \
             an apogee on a barometer's track. Of the chosen logs, {} read an apogee more than \
             {:.0}% from the logged one: the nearest log may be another altimeter's, or a \
             reading the analyzer takes differently. The 2% is a line for counting, not a \
             filter.\n",
            self.logs,
            self.logs_read,
            self.logs_with_apogee,
            self.chosen_past_gap,
            GAP_SHARE * 100.0
        );
        let _ = writeln!(
            page,
            "**The refusals.** The finder refuses a log whose fitted climb speed reaches \
             227.7 m/s (747 ft/s), Mach 0.8 in the coldest air, before its cut: nearer the \
             [speed of sound](../../docs/glossary.md#transonic-and-supersonic) a barometer's \
             reading may follow the airflow, not the height. A log in which no half second holds \
             5 samples (under 8 a second) is too sparse to follow the climb.\n"
        );
        let _ = writeln!(
            page,
            "**Cuts inside the burn.** A barometer's reading can dip in the boost and catch up, \
             so the finder cuts only where the climb speed's fall has held {:.2} s and the speed \
             is the lowest it has been over that span \
             ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)). A cut counts \
             as inside the burn when it comes less than the motor's burn after the log's liftoff: \
             the burn is the end of the motor's thrust curve, as the flight's `.ork` and the \
             curves supplied for it give it; a design with more than one motor or a staging event \
             counts as not known. \
             Liftoff follows ignition, so that also takes in a cut up to that lag after the burn \
             ends. A prediction from such a cut wouldn't be the method's case.",
            boost::HOLD_S
        );
        self.predictions.write_markdown(&mut page);
        let _ = writeln!(
            page,
            "\n**Sources.** {}\n\n{}",
            crate::fixture_flights::FLIGHT_DATA_CREDIT,
            crate::fixture_flights::ERA5_CREDIT
        );
        page
    }

    /// Holds the page to the counts, and the counts to themselves.
    pub fn check_consistent(&self, page: &str) -> Result<(), String> {
        let refused: usize = self.refused.values().sum();
        let flights_ok = self.excluded + self.no_log_read + self.no_apogee + self.chosen
            == self.flights
            && refused + self.cut == self.chosen
            && self.cut_in_burn + self.cut_burn_unknown <= self.cut
            && self.chosen_past_gap <= self.chosen
            && self.logs_with_apogee <= self.logs_read
            && self.logs_read <= self.logs
            && self.predictions.adds_up(self.cut);
        if !flights_ok {
            return Err(format!("{REPORT_JSON}: its counts don't add up"));
        }
        if page != self.to_markdown() {
            return Err(format!(
                "{REPORT_MD} isn't the page {REPORT_JSON} writes; run `cargo xtask fixture-flights`"
            ));
        }
        Ok(())
    }
}

/// Writes, or with `check` checks, the held-out digest and the report.
pub fn run(
    root: &Path,
    entries: &[Entry],
    burns: &BTreeMap<String, f64>,
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
    check: bool,
) -> Result<(), String> {
    let digest = held_out_digest(
        entries
            .iter()
            .map(|e| e.flight_id.as_str())
            .filter(|id| held_out(id)),
    );
    let sealed = held_out_text(&digest);
    let Counts { mut report, cuts } = count(root, entries, burns)?;
    let (predictions, held_out_fits, audit) =
        crate::collection_errors::predict_all(&cuts, forward)?;
    report.predictions = predictions;
    let page = report.to_markdown();
    report.check_consistent(&page)?;
    if check {
        let committed = fs::read_to_string(root.join(HELD_OUT))
            .map_err(|error| format!("reading {HELD_OUT}: {error}"))?;
        if committed != sealed {
            return Err(format!(
                "{HELD_OUT} doesn't hold the SHA-256 of the manifest's held-out ids: the held-out \
                 flights changed (ADR-322 says what may change them)"
            ));
        }
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let stored: Report = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        if stored != report {
            return Err(format!(
                "{REPORT_JSON} does not reproduce; run `cargo xtask fixture-flights`"
            ));
        }
        let committed_page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        stored.check_consistent(&committed_page)?;
        crate::collection_errors::seal_held_out_fits(root, &held_out_fits, true)?;
        println!(
            "fixture-flights: the held-out digests, the boost's counts and the halves' errors reproduce"
        );
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(HELD_OUT, sealed), (REPORT_JSON, json), (REPORT_MD, page)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    // Sealed after the report it goes with is written.
    crate::collection_errors::seal_held_out_fits(root, &held_out_fits, false)?;
    crate::collection_errors::write_audit(root, &audit)?;
    println!("fixture-flights: wrote {HELD_OUT}, {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Invented ids, their digests' first eight bytes computed outside this code (Python's
    /// `hashlib.sha256(id.encode() + b"ADR-209")`), and whether each is below 2⁶⁴ / 3.
    #[test]
    fn the_split_is_adr_322s_encoding() {
        let cases = [
            ("example-flight-1", 0xe7ce_300d_86e7_b79a_u64, false),
            ("example-flight-2", 0xb07c_9542_cb11_f3a9, false),
            ("example-flight-3", 0x68e7_d4b7_6604_9fcc, false),
            ("example-flight-5", 0x8481_1252_a5bb_677a, false),
            ("example-flight-6", 0x3c29_5ce5_0af9_21de, true),
            ("a", 0x6637_8107_71ca_7524, false),
            ("", 0xe709_4cf5_1404_339f, false),
        ];
        for (id, first, expected) in cases {
            assert_eq!(leading(id), first, "{id}'s digest");
            assert_eq!(held_out(id), expected, "{id}");
        }
        // Of the first twelve invented ids, these five are held out (Python, as above).
        let held: Vec<String> = (1..=12)
            .map(|k| format!("example-flight-{k}"))
            .filter(|id| held_out(id))
            .collect();
        assert_eq!(
            held,
            [6, 7, 8, 9, 10].map(|k| format!("example-flight-{k}"))
        );
    }

    /// A fit across flights takes the calibration half only: a held-out flight among its flights,
    /// whatever the caller's own filter said, refuses the set, counting the held-out flights and
    /// naming none. Of the invented ids, 6 to 10 are held out (above).
    #[test]
    fn a_held_out_flight_never_feeds_a_fit_across_flights() {
        let ids: Vec<String> = (1..=12).map(|k| format!("example-flight-{k}")).collect();
        let calibration = Calibration::of(
            ids.iter()
                .filter(|id| !held_out(id))
                .map(|id| (id.as_str(), id.len())),
        )
        .unwrap();
        assert_eq!(calibration.flights().len(), 7);
        let one = Calibration::of([("example-flight-1", 1), ("example-flight-6", 6)]).unwrap_err();
        assert!(one.starts_with("1 held-out flight fed a fit"), "{one}");
        assert!(!one.contains("example-flight"), "{one}");
        let all = Calibration::of(ids.iter().map(|id| (id.as_str(), ()))).unwrap_err();
        assert!(all.starts_with("5 held-out flights fed a fit"), "{all}");
        assert!(Calibration::<()>::of([]).unwrap().flights().is_empty());
    }

    /// The bound is 2⁶⁴ / 3 = 6,148,914,691,236,517,205.33…: its floor is held out, the next
    /// integer isn't. A `<` for `<=`, or a bound off by one, fails here.
    #[test]
    fn the_bound_is_a_third_of_the_range() {
        assert_eq!(HELD_OUT_MOST, 6_148_914_691_236_517_205);
        assert!(leading_is_held_out(0));
        assert!(leading_is_held_out(6_148_914_691_236_517_205));
        assert!(!leading_is_held_out(6_148_914_691_236_517_206));
        assert!(!leading_is_held_out(u64::MAX));
    }

    /// The digest of the sorted list, computed outside this code: Python's
    /// `hashlib.sha256("".join(i + "\n" for i in sorted(ids)).encode())`. Bytewise order puts
    /// `example-flight-10` before `example-flight-6`, and `Alpha` before `beta`.
    #[test]
    fn the_digest_is_of_the_sorted_ids() {
        let held = [6, 7, 8, 9, 10].map(|k| format!("example-flight-{k}"));
        assert_eq!(
            held_out_digest(held.iter().rev().map(String::as_str)),
            "ce06f5e26a36a720038f1ff236a09525148527dc08c4d8dc61a98f49204189bf"
        );
        assert_eq!(
            held_out_digest(["zeta", "Alpha", "beta"]),
            "5ad18157934047fc62ebfb85390f5220f756636558e4eb00f544ee268d977c17"
        );
        // No ids: the SHA-256 of nothing.
        assert_eq!(
            held_out_digest([]),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// Only a barometer's track is a candidate: the three barometric sources, never a GNSS
    /// receiver's, the accelerometer, or no track.
    #[test]
    fn only_a_barometers_track_is_chosen() {
        for source in [Source::Barometer, Source::Pressure, Source::AltitudeChannel] {
            assert!(barometric(Some(source)), "{source:?}");
        }
        for source in [Some(Source::Gnss), Some(Source::Accelerometer), None] {
            assert!(!barometric(source), "{source:?}");
        }
    }

    /// A cut is inside the burn when it comes less than the burn after liftoff; unknown without
    /// either.
    #[test]
    fn a_cut_inside_the_burn_is_less_than_the_burn_after_liftoff() {
        let cut = boost::Cut {
            index: 0,
            time_s: 4.0,
            top_speed_s: 3.0,
            top_speed_m_s: 100.0,
        };
        assert_eq!(cut_in_burn(&cut, Some(0.5), Some(3.6)), Some(true));
        assert_eq!(cut_in_burn(&cut, Some(0.5), Some(3.5)), Some(false));
        assert_eq!(cut_in_burn(&cut, None, Some(3.6)), None);
        assert_eq!(cut_in_burn(&cut, Some(0.5), None), None);
    }

    /// The nearest apogee wins, the first on a tie.
    #[test]
    fn the_nearest_log_is_chosen() {
        // Each log by its apogee reading and its place in the catalog.
        let apogee = |log: &(f64, usize)| log.0;
        let chosen = choose(vec![(900.0, 0), (1_010.0, 1)], apogee, 1_000.0).unwrap();
        assert_eq!(chosen, (1_010.0, 1));
        let tie = choose(vec![(990.0, 0), (1_010.0, 1)], apogee, 1_000.0).unwrap();
        assert_eq!(tie, (990.0, 0));
        assert!(choose(Vec::<(f64, usize)>::new(), apogee, 1_000.0).is_none());
    }

    /// The committed digest is one line, and the committed report holds to itself, here and in
    /// CI, where the logs can't be read.
    #[test]
    fn the_committed_files_hold_to_themselves() {
        let root = crate::designs::root().unwrap();
        let sealed = fs::read_to_string(root.join(HELD_OUT)).unwrap();
        let digest = sealed.split("  ").next().unwrap();
        assert!(digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()));
        assert_eq!(sealed, held_out_text(digest));
        let report: Report =
            serde_json::from_str(&fs::read_to_string(root.join(REPORT_JSON)).unwrap()).unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        report.check_consistent(&page).unwrap();
    }
}
