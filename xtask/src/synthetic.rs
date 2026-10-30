//! `cargo xtask synthetic [--check]`: flies every synthetic flight (`hpr_forensics::synthetic`,
//! M7.2a, ADR-280) and writes their census to `validation/reports/synthetic-flights.json` and
//! `.md`.
//!
//! The census counts each set's flights by design and by logger, the ranges of eight quantities
//! (apogee, top Mach number and speed, flight time, wind, rail angle, site elevation and the air's
//! offset), and every draw that didn't fly, with its reason. Values are kept to three
//! significant figures, the digits every platform shares, so CI's validate job holds the census on
//! macOS, Windows and Linux. `--check` writes nothing, and fails unless the flights reproduce the
//! committed census.
//!
//! It then holds the flight-log readings to their bounds (M7.2b): every tuning flight is logged by
//! its own logger and again at every rate and step of `held`'s grid, each log is read by
//! `hpr_flightdata::readings`, and each reading given is compared with the flight's truth. The
//! report tallies, for each log and reading, how many were given, the largest share of its bound
//! an error took, and how many were withheld, by reason. The command fails, writing nothing, if
//! any reading's error passes its bound, or any log can't be written.
//!
//! Last, it holds the readings' 90% intervals to ADR-257's test (M7.2e1, ADR-308): the
//! calibration set's 2,000 flights fit each interval's shares again, which must equal the
//! library's to three significant figures, and on sets A and B each reading's truths below, inside
//! and above its interval must pass the test while the intervals narrowed, widened and shifted
//! down each fail it. `--intervals` runs only that and prints its section.
//!
//! The flights are flown on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a flight is the same however many came before it on its thread (the
//! module's tests), so the census doesn't depend on the number of threads. A flight that panics
//! is counted as not flown, with the panic's message, rather than lost with its thread.
//!
//! The designs' files are read from `validation/designs/` here: `hpr-forensics` is published
//! and can't hold files outside its own folder.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use hpr_flightdata::readings::Clip;
use hpr_flightdata::readings::interval::Shares;
use hpr_flightdata::smoother::{GnssTrust, Options};
use hpr_forensics::synthetic::census::{self, Range, Row, SetCensus};
use hpr_forensics::synthetic::designs::{DESIGNS, DesignFiles};
use hpr_forensics::synthetic::held::{self, Errors, Outcome, Quantity, Tally};
use hpr_forensics::synthetic::intervals::{self, FlightSaid};
use hpr_forensics::synthetic::logger::Profile;
use hpr_forensics::synthetic::smoothed::{self, Checked, SetCheck};
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "  synthetic [--check]
                           Fly the synthetic flights' three sets (ADR-280) and write their census
                           to validation/reports/synthetic-flights.{json,md}, with the tuning
                           set's flight-log readings held to their bounds. --check writes
                           nothing and fails unless the flights reproduce the committed census.
                           Either fails if a reading's error passes its bound, or if a reading's
                           90% interval fails ADR-257's test or a probe of it passes.
  synthetic --intervals    Only the readings' intervals: fit their shares on the calibration set,
                           test them on sets A and B, print the section; writes nothing.";

/// The report's path under the repository, without its extension.
const REPORT: &str = "validation/reports/synthetic-flights";

/// The committed census of every set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Report {
    /// What it is.
    about: String,
    /// Each set's census, in [`Set::ALL`]'s order.
    sets: Vec<SetCensus>,
    /// The tuning set's flight-log readings against their bounds: each log and reading, in the
    /// order [`held::tally`] gives them, to three significant figures.
    readings: Vec<Tally>,
    /// The tuning set's errors, each reading's over every log, in the order [`held::errors`]
    /// gives them, to three significant figures.
    errors: Vec<Errors>,
    /// The tuning set's top accelerations against their parts' ranges.
    tops: Tops,
    /// The smoother held to each set's truth, in [`Set::ALL`]'s order.
    smoother: Vec<SmootherSet>,
    /// A health card for each logger: its first smoothed flight in the tuning set.
    cards: Vec<Card>,
    /// The readings' 90% intervals: their shares fitted on the calibration set, and ADR-257's
    /// test on the two test sets (M7.2e1, ADR-308).
    intervals: Intervals,
}

/// The readings' 90% intervals (ADR-257 §1–2, ADR-308).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Intervals {
    /// The calibration set's flights flown.
    calibration_flights: u64,
    /// Its apogees withheld: all from the GNSS-only tracker, whose log gives none.
    calibration_withheld: u64,
    /// Each reading's shares, fitted again on the calibration set and as the library holds them.
    shares: Vec<FittedShares>,
    /// ADR-257 §1's test on each test set, in A, B order.
    sets: Vec<IntervalSet>,
}

/// One reading's shares.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FittedShares {
    /// The reading.
    quantity: Quantity,
    /// The calibration set's readings that state an interval.
    stated: u64,
    /// Fitted on them, to three significant figures.
    fitted: Shares,
    /// As `hpr_flightdata::readings::interval` holds them.
    library: Shares,
}

/// One test set's checks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct IntervalSet {
    /// The set.
    set: Set,
    /// Each reading's check, in `intervals::STATED`'s order, its bands and mean error to three
    /// significant figures; whether it holds is decided before.
    checks: Vec<IntervalCheck>,
}

/// One reading's check on one set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct IntervalCheck {
    /// Whether the test passes on it and every probe fails.
    holds: bool,
    /// Whether the test passes on each probe's intervals, decided before the bands are rounded.
    probe_passes: BTreeMap<intervals::Probe, bool>,
    /// The check.
    check: intervals::Check,
}

/// One set's smoother check (M7.2d), and the same flights with each GNSS receiver trusted from
/// liftoff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SmootherSet {
    /// The set.
    set: Set,
    /// The check, to three significant figures.
    check: SetCheck,
    /// The flights whose log has a GNSS track, smoothed again with the receiver trusted from
    /// liftoff (`GnssTrust::FromLiftoff`); the others keep their check.
    gnss_rerun: u64,
    /// The cells with those flights trusted from liftoff, to three significant figures.
    gnss_from_liftoff: Vec<smoothed::Cell>,
}

impl SmootherSet {
    /// Whether every cell of the trusted-from-liftoff check holds: it shouldn't.
    fn mutation_holds(&self) -> bool {
        self.gnss_from_liftoff.iter().all(|c| c.holds)
    }
}

/// One flight's health card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Card {
    /// Its logger.
    logger: String,
    /// The flight's index in the tuning set.
    flight: u64,
    /// The card, as `Health::card` writes it.
    card: String,
}

/// The tuning set's top accelerations against the ranges their parts clip at: whether a top
/// read as clipped was one, or a plateau read as flat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tops {
    /// How many flights' true tops reach their part's range.
    reaching: u64,
    /// The flight whose true top came nearest its part's range; `None` if no logger has an
    /// accelerometer.
    nearest: Option<Top>,
    /// The flights logged again with their part set to a lower range
    /// ([`held::lower_range_name`]), and how many of their true tops reach it; `None` if no
    /// logger's part has one.
    lower: Option<Lower>,
    /// How many tops were read as floors, by cause.
    floors: Floors,
    /// Each top read as a floor, by flight and log, in the flights' order.
    clipped: Vec<Clipped>,
}

/// The flights logged with their part set to a lower range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Lower {
    /// The log's name.
    log: String,
    /// How many flights were logged so.
    flights: u64,
    /// How many of their true tops reach the lower range.
    reaching: u64,
}

/// How many tops were read as floors, by cause.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct Floors {
    /// Clipped at the part's stated range.
    at_range: u64,
    /// Held flat at the top, short of any stated range.
    flat: u64,
    /// A top whose thrust the accelerometer shows no end to before apogee, as where it ends in
    /// the thrust.
    thrust_in_part: u64,
}

/// A tuning flight's true top acceleration against its part's range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Top {
    /// The flight's index in the tuning set.
    flight: u64,
    /// Its logger's profile.
    profile: String,
    /// The truth's top over the range its part clips at, to three significant figures: 1
    /// reaches it.
    share: f64,
}

/// A top acceleration read as a floor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Clipped {
    /// The flight's index in the tuning set.
    flight: u64,
    /// Its logger's profile.
    profile: String,
    /// The log it was read from.
    log: String,
    /// Why it is a floor: `range`, `flat`, or `thrust_in_part`.
    cause: String,
}

/// Runs the command.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    let mut only_intervals = false;
    let mut probe: Option<(Set, u64, Options)> = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--intervals" => only_intervals = true,
            "--smoother" => {
                let set = match args.next().map(String::as_str) {
                    Some("tuning") => Set::Tuning,
                    Some("a") => Set::A,
                    Some("b") => Set::B,
                    other => return Err(format!("--smoother takes tuning, a or b, not {other:?}")),
                };
                probe = Some((set, set.count(), Options::default()));
            }
            "--count" => {
                let count = args
                    .next()
                    .and_then(|n| n.parse().ok())
                    .ok_or("--count takes a number")?;
                probe.as_mut().ok_or("--count comes after --smoother")?.1 = count;
            }
            "--trust-gnss-from-liftoff" => {
                probe
                    .as_mut()
                    .ok_or("--trust-gnss-from-liftoff comes after --smoother")?
                    .2
                    .gnss_trust = GnssTrust::FromLiftoff;
            }
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let files = design_files(&root)?;
    if only_intervals {
        if check || probe.is_some() {
            return Err(
                "--intervals runs alone: it takes no --check, --smoother or --count".into(),
            );
        }
        let (made, broken) = interval_check(&files)?;
        let mut page = String::new();
        intervals_section(&mut page, &made);
        print!("{page}");
        if !broken.is_empty() {
            return Err(format!(
                "the readings' intervals (M7.2e1) don't come out as they must:\n{}",
                broken.join("\n")
            ));
        }
        return Ok(());
    }
    if let Some((set, count, options)) = probe {
        let checks: Vec<Checked> = smooth_set(set, count, &files, options, false)?
            .into_iter()
            .map(|(_, check, _)| check)
            .collect();
        print!("{}", smoother_table(set, &smoothed::tally(&checks)));
        return Ok(());
    }
    let (flights, each) = read_logs(Set::Tuning.count(), &files)?;
    let tallies = held::tally(&flights);
    let missed = misses(&tallies);
    if !missed.is_empty() {
        return Err(format!(
            "a reading's error passes its bound on the tuning set:\n{}",
            missed.join("\n")
        ));
    }
    let mut smoother = Vec::new();
    let mut cards = Vec::new();
    for set in Set::ALL {
        let (made, set_cards) = smoother_set(set, &files)?;
        println!(
            "{}: smoother {}, {} flights; trusting GNSS from liftoff {}",
            set.name(),
            if made.check.holds() { "holds" } else { "fails" },
            made.check.flights,
            if made.mutation_holds() {
                "holds"
            } else {
                "fails"
            }
        );
        if set == Set::Tuning {
            cards = set_cards;
        }
        smoother.push(made);
    }
    let broken: Vec<String> = smoother
        .iter()
        .flat_map(|s| {
            let fails = (!s.check.holds()).then(|| format!("{}: the smoother fails", s.set.name()));
            // A log withheld for any reason but having no barometer leaves every cell, its
            // misses with it, and so does a flight that wasn't flown: either fails the check.
            let lost = (s.check.lost() > 0).then(|| {
                format!(
                    "{}: {} logs withheld with a barometer ({:?})",
                    s.set.name(),
                    s.check.lost(),
                    s.check.withheld
                )
            });
            let unflown = (s.check.flights != s.set.count()).then(|| {
                format!(
                    "{}: {} of {} flights smoothed or withheld; the rest weren't flown",
                    s.set.name(),
                    s.check.flights,
                    s.set.count()
                )
            });
            // A step the pass back couldn't solve is a filter that lost the track.
            let unsmoothed = (s.check.health.unsmoothed_steps > 0).then(|| {
                format!(
                    "{}: {} steps the pass back couldn't smooth",
                    s.set.name(),
                    s.check.health.unsmoothed_steps
                )
            });
            // The tuning set's 200 flights are too few for a band at 99.9% to show the mutation's
            // misses; the two evaluation sets of 1,000 must.
            let mutation = (s.set != Set::Tuning && s.mutation_holds())
                .then(|| format!("{}: trusting GNSS from liftoff holds", s.set.name()));
            fails
                .into_iter()
                .chain(lost)
                .chain(unflown)
                .chain(unsmoothed)
                .chain(mutation)
        })
        .collect();
    if !broken.is_empty() {
        return Err(format!(
            "the smoother's check (M7.2d) doesn't come out as it must:\n{}",
            broken.join("\n")
        ));
    }
    let (intervals, broken) = interval_check(&files)?;
    if !broken.is_empty() {
        return Err(format!(
            "the readings' intervals (M7.2e1) don't come out as they must:\n{}",
            broken.join("\n")
        ));
    }
    let report = Report {
        about: "The synthetic flights' census (M7.2a, ADR-280) and the flight-log readings held \
                to their bounds on the tuning set, with their errors (M7.2b, M7.2c), written by \
                `cargo xtask synthetic`."
            .to_owned(),
        sets: Set::ALL
            .iter()
            .map(|&set| fly(set, set.count(), &files))
            .collect(),
        readings: tallies.iter().map(rounded).collect(),
        errors: held::errors(&flights).iter().map(rounded_errors).collect(),
        tops: tops(&flights, &each),
        smoother,
        cards,
        intervals,
    };
    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    let page = markdown(&report);
    for set in &report.sets {
        println!(
            "{}: {} drawn, {} flown, {} not flown",
            set.set.name(),
            set.drawn,
            set.flown,
            set.not_flown.len()
        );
    }
    let json_path = root.join(format!("{REPORT}.json"));
    let page_path = root.join(format!("{REPORT}.md"));
    if check {
        let mut problems = Vec::new();
        for (path, made) in [(&json_path, &json), (&page_path, &page)] {
            match fs::read_to_string(path) {
                Ok(committed) if committed.replace("\r\n", "\n") == *made => {}
                Ok(committed) => problems.push(difference(path, &committed, made)),
                Err(error) => problems.push(format!("{}: {error}", path.display())),
            }
        }
        if !problems.is_empty() {
            return Err(format!(
                "the synthetic flights don't reproduce the committed census; run \
                 `cargo xtask synthetic` and commit what it writes:\n{}",
                problems.join("\n")
            ));
        }
        println!("the synthetic flights reproduce {REPORT}.json and .md");
        return Ok(());
    }
    write(&json_path, &json)?;
    write(&page_path, &page)?;
    println!("wrote {REPORT}.json and .md");
    Ok(())
}

/// The designs' files under `root`, in [`DESIGNS`]' order.
fn design_files(root: &Path) -> Result<DesignFiles, String> {
    let files = DESIGNS
        .iter()
        .map(|design| {
            let path = root
                .join("validation/designs")
                .join(format!("{}.json", design.name));
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    DesignFiles::new(files).map_err(|error| error.to_string())
}

/// Flies the first `count` flights of `set` on every core and takes their census.
fn fly(set: Set, count: u64, files: &DesignFiles) -> SetCensus {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let mut rows: Vec<Option<Result<Row, String>>> = vec![None; count as usize];
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                let files = files.clone();
                scope.spawn(move || {
                    let mut flyer = Flyer::new(files.clone());
                    let mut done = Vec::new();
                    for index in (start..count).step_by(threads as usize) {
                        let row = panic::catch_unwind(AssertUnwindSafe(|| {
                            census::row(&mut flyer, set, index)
                        }))
                        .unwrap_or_else(|payload| {
                            // A panic may leave the flyer's cache half-built: start a new one.
                            flyer = Flyer::new(files.clone());
                            Err(format!("it panicked: {}", panic_message(&*payload)))
                        });
                        done.push((index, row));
                    }
                    done
                })
            })
            .collect();
        for handle in handles {
            match handle.join() {
                Ok(done) => {
                    for (index, row) in done {
                        rows[index as usize] = Some(row);
                    }
                }
                Err(_) => {
                    // A thread that panicked leaves its flights unfilled; they are counted as not
                    // flown below rather than lost.
                }
            }
        }
    });
    let rows: Vec<Result<Row, String>> = rows
        .into_iter()
        .map(|row| row.unwrap_or_else(|| Err("its thread panicked".to_owned())))
        .collect();
    census::from_rows(set, &rows)
}

/// One flight's index, its check, and with `mutate` and a GNSS track, its check with the
/// receiver trusted from liftoff.
type Smoothed = (u64, Checked, Option<Checked>);

/// Smooths the first `count` flights of `set` on every core and checks each against its truth,
/// in the flights' order; a flight that doesn't fly is left out, as the census counts it. With
/// `mutate`, each log with a GNSS track the smoother takes is smoothed again with the receiver
/// trusted from liftoff. A flight whose smoothing panics is an error, not a flight left out: a
/// check over the rest would hold on fewer flights than it says.
fn smooth_set(
    set: Set,
    count: u64,
    files: &DesignFiles,
    options: Options,
    mutate: bool,
) -> Result<Vec<Smoothed>, String> {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let mut checks: Vec<Option<Smoothed>> = vec![None; count as usize];
    let mut panicked: Vec<String> = Vec::new();
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                let files = files.clone();
                scope.spawn(move || {
                    let mut flyer = Flyer::new(files);
                    let mut done = Vec::new();
                    for index in (start..count).step_by(threads as usize) {
                        let Ok(flight) = flyer.flight(set, index) else {
                            continue;
                        };
                        let smoothed = panic::catch_unwind(AssertUnwindSafe(|| {
                            let check = smoothed::check(&flight, options);
                            let mutated = (mutate && check.has_gnss && check.withheld.is_none())
                                .then(|| {
                                    let trusted = Options {
                                        gnss_trust: GnssTrust::FromLiftoff,
                                    };
                                    smoothed::check(&flight, trusted)
                                });
                            (index, check, mutated)
                        }))
                        .map_err(|payload| {
                            format!(
                                "{} flight {index}: the smoother panicked: {}",
                                set.name(),
                                panic_message(&*payload)
                            )
                        });
                        done.push((index, smoothed));
                    }
                    done
                })
            })
            .collect();
        for handle in handles {
            match handle.join() {
                Ok(done) => {
                    for (index, smoothed) in done {
                        match smoothed {
                            Ok(check) => checks[index as usize] = Some(check),
                            Err(message) => panicked.push(message),
                        }
                    }
                }
                Err(payload) => panicked.push(format!(
                    "{}: a smoothing thread panicked: {}",
                    set.name(),
                    panic_message(&*payload)
                )),
            }
        }
    });
    if panicked.is_empty() {
        Ok(checks.into_iter().flatten().collect())
    } else {
        Err(panicked.join("\n"))
    }
}

/// A set's smoother check and its trusted-from-liftoff mutation for the report, with a health
/// card for each logger's first smoothed flight.
fn smoother_set(set: Set, files: &DesignFiles) -> Result<(SmootherSet, Vec<Card>), String> {
    let all = smooth_set(set, set.count(), files, Options::default(), true)?;
    let mut cards = Vec::new();
    for profile in Profile::ALL {
        if let Some((index, health)) = all.iter().find_map(|(index, check, _)| {
            (check.profile == profile)
                .then_some(check.health.as_ref())
                .flatten()
                .map(|h| (*index, h))
        }) {
            cards.push(Card {
                logger: profile.name().to_owned(),
                flight: index,
                card: health.card(),
            });
        }
    }
    let gnss_rerun = all.iter().filter(|(_, _, m)| m.is_some()).count() as u64;
    let checks: Vec<Checked> = all.iter().map(|(_, c, _)| c.clone()).collect();
    let mutated: Vec<Checked> = all
        .into_iter()
        .map(|(_, check, mutated)| mutated.unwrap_or(check))
        .collect();
    let mut check = smoothed::tally(&checks);
    round_health(&mut check.health);
    round_cells(&mut check.cells);
    for cells in check.by_logger.values_mut() {
        round_cells(cells);
    }
    let mut gnss_from_liftoff = smoothed::tally(&mutated).cells;
    round_cells(&mut gnss_from_liftoff);
    Ok((
        SmootherSet {
            set,
            check,
            gnss_rerun,
            gnss_from_liftoff,
        },
        cards,
    ))
}

/// What each flight of `set`'s own log says of the readings that state an interval, on every
/// core, in the flights' order. A draw that doesn't fly, or a reading that panics, is an error:
/// the shares and the test would hold on fewer flights than they say.
fn say_set(set: Set, files: &DesignFiles) -> Result<Vec<FlightSaid>, String> {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let count = set.count();
    let mut said: Vec<Option<FlightSaid>> = vec![None; count as usize];
    let mut problems: Vec<String> = Vec::new();
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                let files = files.clone();
                scope.spawn(move || {
                    let mut flyer = Flyer::new(files.clone());
                    let mut done = Vec::new();
                    for index in (start..count).step_by(threads as usize) {
                        let flown = panic::catch_unwind(AssertUnwindSafe(|| {
                            flyer
                                .flight(set, index)
                                .map(|flight| intervals::said(index, &flight))
                                .map_err(|e| e.to_string())
                        }))
                        .unwrap_or_else(|payload| {
                            flyer = Flyer::new(files.clone());
                            Err(format!("it panicked: {}", panic_message(&*payload)))
                        })
                        .map_err(|e| format!("{} flight {index}: {e}", set.name()));
                        done.push((index, flown));
                    }
                    done
                })
            })
            .collect();
        for handle in handles {
            match handle.join() {
                Ok(done) => {
                    for (index, flown) in done {
                        match flown {
                            Ok(flight) => said[index as usize] = Some(flight),
                            Err(message) => problems.push(message),
                        }
                    }
                }
                Err(payload) => problems.push(format!(
                    "{}: a reading thread panicked: {}",
                    set.name(),
                    panic_message(&*payload)
                )),
            }
        }
    });
    if !problems.is_empty() {
        problems.sort();
        return Err(problems.join("\n"));
    }
    said.into_iter()
        .enumerate()
        .map(|(index, s)| s.ok_or_else(|| format!("{} flight {index} wasn't read", set.name())))
        .collect()
}

/// What leaves `flights` of `set` without failing the test: a reading withheld from a log with a
/// barometer, or read with no interval. Either would take its misses out of the counts.
fn dropped(set: Set, flights: &[FlightSaid]) -> Vec<String> {
    let mut problems = Vec::new();
    for flight in flights {
        for (quantity, said) in &flight.said {
            let problem = match said {
                intervals::Said::Withheld(reason)
                    if flight.logger != Profile::GnssTracker.name() =>
                {
                    Some(format!("withheld ({reason:?})"))
                }
                intervals::Said::Unstated => Some("read with no interval".to_owned()),
                _ => None,
            };
            if let Some(problem) = problem {
                problems.push(format!(
                    "{} flight {} ({}): {} {problem}",
                    set.name(),
                    flight.index,
                    flight.logger,
                    quantity.name()
                ));
            }
        }
    }
    problems
}

/// The readings' intervals: their shares fitted on the calibration set, then ADR-257's test on
/// sets A and B, with what doesn't come out as it must.
fn interval_check(files: &DesignFiles) -> Result<(Intervals, Vec<String>), String> {
    let calibration = say_set(Set::Calibration, files)?;
    let mut broken = dropped(Set::Calibration, &calibration);
    let calibration_withheld = calibration
        .iter()
        .flat_map(|f| &f.said)
        .filter(|(q, said)| {
            *q == Quantity::ApogeeHeight && matches!(said, intervals::Said::Withheld(_))
        })
        .count() as u64;
    let mut shares = Vec::new();
    for quantity in intervals::STATED {
        let library = intervals::shares(quantity)
            .ok_or_else(|| format!("{} states no shares", quantity.name()))?;
        let stated = calibration
            .iter()
            .flat_map(|f| &f.said)
            .filter(|(q, said)| *q == quantity && matches!(said, intervals::Said::Stated(_)))
            .count() as u64;
        let fitted = intervals::fitted(quantity, &calibration)
            .ok_or_else(|| format!("{}'s shares can't be fitted", quantity.name()))?;
        let fitted = Shares {
            low: intervals::three_figures(fitted.low),
            high: intervals::three_figures(fitted.high),
        };
        if fitted != library {
            broken.push(format!(
                "{}: the calibration set fits shares {} and {}, the library holds {} and {}",
                quantity.name(),
                fitted.low,
                fitted.high,
                library.low,
                library.high
            ));
        }
        shares.push(FittedShares {
            quantity,
            stated,
            fitted,
            library,
        });
    }
    let mut sets = Vec::new();
    for set in [Set::A, Set::B] {
        let flights = say_set(set, files)?;
        broken.extend(dropped(set, &flights));
        let checks = intervals::STATED
            .iter()
            .map(|&quantity| {
                let check = intervals::check(quantity, &flights);
                let holds = check.holds();
                if !check.test.passes() {
                    broken.push(format!(
                        "{}: {}'s intervals fail: {:?}",
                        set.name(),
                        quantity.name(),
                        check.test.counts
                    ));
                }
                for (probe, test) in &check.probes {
                    if test.passes() {
                        broken.push(format!(
                            "{}: {}'s intervals {} still pass: {:?}",
                            set.name(),
                            quantity.name(),
                            probe.name(),
                            test.counts
                        ));
                    }
                }
                let probe_passes = check
                    .probes
                    .iter()
                    .map(|(probe, test)| (*probe, test.passes()))
                    .collect();
                IntervalCheck {
                    holds,
                    probe_passes,
                    check: rounded_check(check),
                }
            })
            .collect();
        sets.push(IntervalSet { set, checks });
    }
    Ok((
        Intervals {
            calibration_flights: calibration.len() as u64,
            calibration_withheld,
            shares,
            sets,
        },
        broken,
    ))
}

/// `check`'s bands and largest error to three significant figures, as the report holds them, and
/// its mean error at the figures of the largest ([`census::at_the_figures_of`]): the mean is a
/// small difference of errors that size, and its later digits differ from platform to platform.
fn rounded_check(mut check: intervals::Check) -> intervals::Check {
    let round = |test: &mut intervals::Test| {
        for band in [&mut test.below, &mut test.inside, &mut test.above] {
            band.low = intervals::three_figures(band.low);
            band.high = intervals::three_figures(band.high);
        }
    };
    round(&mut check.test);
    for test in check.probes.values_mut() {
        round(test);
    }
    check.largest_error = intervals::three_figures(check.largest_error);
    check.mean_error = signed_mean(check.mean_error, check.largest_error);
    check
}

/// `mean` at the figures of `largest` ([`census::at_the_figures_of`]), keeping its sign when it
/// rounds to zero: the apogee's error side is the sign.
fn signed_mean(mean: f64, largest: f64) -> f64 {
    let rounded = census::at_the_figures_of(mean, largest);
    if rounded == 0.0 {
        0.0_f64.copysign(mean)
    } else {
        rounded
    }
}

/// Cells to three significant figures, as the report holds them; whether each holds is decided
/// before.
fn round_cells(cells: &mut [smoothed::Cell]) {
    for c in cells {
        c.coverage = census::three_figures(c.coverage);
        c.median_sd = census::three_figures(c.median_sd);
        c.largest_sd = census::three_figures(c.largest_sd);
        for share in &mut c.lowest_miss_share {
            *share = census::three_figures(*share);
        }
    }
}

/// The health sums to three significant figures, as the report holds them. A flight's log ends
/// `ground_s` after the simulated landing, and the landing's time differs from one platform to
/// another, so a log can end a sample sooner on one; the sums, unlike the census's counts of
/// flights, then differ by a sample (ADR-300 item 11). A zero stays zero, so a step the pass
/// back couldn't smooth still shows.
fn round_health(health: &mut smoothed::HealthTally) {
    let figures = |n: &mut u64| *n = census::three_figures(*n as f64).round() as u64;
    for t in health.sensors.values_mut() {
        for n in [
            &mut t.logs,
            &mut t.samples,
            &mut t.rejected,
            &mut t.unweighed,
            &mut t.resets,
            &mut t.logs_rejecting,
        ] {
            figures(n);
        }
    }
    for n in [
        &mut health.transonic_spans,
        &mut health.charge_spans,
        &mut health.gnss_trusted,
        &mut health.unsmoothed_steps,
    ] {
        figures(n);
    }
}

/// A set's smoother check as text.
fn smoother_table(set: Set, check: &SetCheck) -> String {
    let mut out = format!(
        "{}: {} flights, withheld {:?}, holds: {}\n",
        set.name(),
        check.flights,
        check.withheld,
        check.holds()
    );
    let row = |out: &mut String, c: &smoothed::Cell| {
        let _ = writeln!(
            out,
            "  {:<15} {:<7} n {:>5} below {:>4} above {:>4} cover {:.3} sd {:>9.3} max {:>9.3} {}",
            c.moment.name(),
            c.state.name(),
            c.flights,
            c.below,
            c.above,
            c.coverage,
            c.median_sd,
            c.largest_sd,
            if c.holds { "" } else { "FAILS" }
        );
    };
    for c in &check.cells {
        row(&mut out, c);
    }
    for (logger, cells) in &check.by_logger {
        let _ = writeln!(out, " {logger}");
        for c in cells {
            row(&mut out, c);
        }
    }
    out
}

/// One tuning flight's logs, each named, with each reading's outcome.
type FlightOutcomes = (u64, Vec<(String, Vec<(Quantity, Outcome)>)>);

/// One tuning flight's index, its logger's profile, and its true top over its part's range
/// ([`held::top_over_range`]) and over the lower range its part can be set to
/// ([`held::top_over_lower_range`]).
type FlightTop = (u64, &'static str, Option<f64>, Option<f64>);

/// Logs the first `count` tuning flights at every rate and step of `held`'s grid on every core,
/// reads each log, and gives each reading's outcome, and each flight's top against its part's
/// range, in the flights' order. A flight that doesn't fly is the census's to report; a log that
/// can't be written, or a panic, fails the command.
fn read_logs(
    count: u64,
    files: &DesignFiles,
) -> Result<(Vec<FlightOutcomes>, Vec<FlightTop>), String> {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let set = Set::Tuning;
    let mut flights: Vec<(FlightOutcomes, FlightTop)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                let files = files.clone();
                scope.spawn(
                    move || -> Result<Vec<(FlightOutcomes, FlightTop)>, String> {
                        let mut flyer = Flyer::new(files);
                        let mut done = Vec::new();
                        for index in (start..count).step_by(threads as usize) {
                            let Ok(flight) = flyer.flight(set, index) else {
                                continue;
                            };
                            let logs = held::logs(&flight, set.seed(), index)
                                .map_err(|error| format!("tuning flight {index}: {error}"))?;
                            let outcomes = logs
                                .iter()
                                .map(|(name, log)| (name.clone(), held::outcomes(&flight, log)))
                                .collect();
                            let top = (
                                index,
                                flight.draw.logger.profile.name(),
                                held::top_over_range(&flight),
                                held::top_over_lower_range(&flight),
                            );
                            done.push(((index, outcomes), top));
                        }
                        Ok(done)
                    },
                )
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| match handle.join() {
                Ok(done) => done,
                Err(payload) => Err(format!(
                    "reading the tuning flights' logs panicked: {}",
                    panic_message(&*payload)
                )),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|done| done.into_iter().flatten().collect())
    })?;
    flights.sort_by_key(|((index, _), _)| *index);
    Ok(flights.into_iter().unzip())
}

/// The tuning set's top accelerations against their parts' ranges, from `flights`' outcomes and
/// `each` flight's top.
fn tops(flights: &[FlightOutcomes], each: &[FlightTop]) -> Tops {
    let profile = |flight: u64| {
        each.iter()
            .find(|(index, _, _, _)| *index == flight)
            .map_or("unknown", |(_, profile, _, _)| *profile)
            .to_owned()
    };
    let shares = each
        .iter()
        .filter_map(|(flight, profile, share, _)| share.map(|share| (*flight, *profile, share)));
    let lower_shares: Vec<f64> = each.iter().filter_map(|(_, _, _, lower)| *lower).collect();
    let lower = (!lower_shares.is_empty()).then(|| Lower {
        log: held::lower_range_name(),
        flights: lower_shares.len() as u64,
        reaching: lower_shares.iter().filter(|share| **share >= 1.0).count() as u64,
    });
    let reaching = shares.clone().filter(|(_, _, share)| *share >= 1.0).count() as u64;
    let nearest = shares
        .max_by(|a, b| a.2.total_cmp(&b.2))
        .map(|(flight, profile, share)| Top {
            flight,
            profile: profile.to_owned(),
            share: census::three_figures(share),
        });
    let clipped: Vec<Clipped> = flights
        .iter()
        .flat_map(|(flight, logs)| {
            logs.iter().flat_map(move |(log, results)| {
                results
                    .iter()
                    .filter_map(move |(quantity, outcome)| match outcome {
                        Outcome::Floor { clip, .. } if *quantity == Quantity::TopAcceleration => {
                            let cause = match clip {
                                Some(Clip::Range) => "range",
                                Some(Clip::Flat) => "flat",
                                Some(_) => "other",
                                None => "thrust_in_part",
                            };
                            Some((*flight, log.clone(), cause))
                        }
                        _ => None,
                    })
            })
        })
        .map(|(flight, log, cause)| Clipped {
            flight,
            profile: profile(flight),
            log,
            cause: cause.to_owned(),
        })
        .collect();
    let count = |cause: &str| clipped.iter().filter(|c| c.cause == cause).count() as u64;
    let floors = Floors {
        at_range: count("range"),
        flat: count("flat"),
        thrust_in_part: count("thrust_in_part"),
    };
    Tops {
        reaching,
        nearest,
        lower,
        floors,
        clipped,
    }
}

/// Each tally whose worst error passes its bound, in words.
fn misses(tallies: &[Tally]) -> Vec<String> {
    tallies
        .iter()
        .filter(|tally| !tally.holds())
        .map(|tally| {
            let worst = tally.worst.as_ref().map_or_else(String::new, |worst| {
                format!(
                    ": flight {}, error {} against a bound of {} {}",
                    worst.flight,
                    worst.error,
                    worst.bound,
                    tally.quantity.unit()
                )
            });
            format!("{}, {}{worst}", tally.log, tally.quantity.name())
        })
        .collect()
}

/// A tally to three significant figures, as the report holds it.
fn rounded(tally: &Tally) -> Tally {
    let mut tally = tally.clone();
    if let Some(worst) = &mut tally.worst {
        worst.error = census::three_figures(worst.error);
        worst.bound = census::three_figures(worst.bound);
        worst.share = census::three_figures(worst.share);
    }
    tally
}

/// A reading's errors as the report holds them: the least, the most and the mean bound to three
/// significant figures, and the mean error at the figures of the largest error
/// ([`census::at_the_figures_of`]), the digits every platform shares.
fn rounded_errors(errors: &Errors) -> Errors {
    let mut errors = errors.clone();
    if let Some(spread) = &mut errors.spread {
        spread.least_error = census::three_figures(spread.least_error);
        spread.most_error = census::three_figures(spread.most_error);
        let largest = spread.least_error.abs().max(spread.most_error.abs());
        spread.mean_error = census::at_the_figures_of(spread.mean_error, largest);
        spread.mean_bound = census::three_figures(spread.mean_bound);
    }
    errors
}

/// A panic's message, when it carries one.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no message".to_owned())
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
}

/// The first line where `committed` and `made` differ.
fn difference(path: &Path, committed: &str, made: &str) -> String {
    let committed = committed.replace("\r\n", "\n");
    let line = committed
        .lines()
        .zip(made.lines())
        .position(|(a, b)| a != b)
        .unwrap_or_else(|| committed.lines().count().min(made.lines().count()));
    format!(
        "{} differs at line {}: committed `{}`, made `{}`",
        path.display(),
        line + 1,
        committed.lines().nth(line).unwrap_or(""),
        made.lines().nth(line).unwrap_or("")
    )
}

/// A number as the page shows it: the census's three significant figures, as the JSON holds
/// them, and no sign on a zero.
fn number(x: f64) -> String {
    if x == 0.0 {
        "0".to_owned()
    } else {
        format!("{x}")
    }
}

/// `x` with its sign, `+0` and `-0` included, since a rounded mean keeps the sign it had; not a
/// number as [`number`] writes it.
fn signed(x: f64) -> String {
    if x.is_finite() {
        format!("{x:+}")
    } else {
        number(x)
    }
}

/// A set's name as the page heads its column.
fn title(set: Set) -> &'static str {
    match set {
        Set::Tuning => "Tuning",
        Set::A => "A",
        Set::B => "B",
        Set::Calibration => "Calibration",
    }
}

fn range(r: Option<Range>) -> String {
    r.map_or_else(
        || "none".to_owned(),
        |r| {
            format!(
                "{} / {} / {}",
                number(r.min),
                number(r.median),
                number(r.max)
            )
        },
    )
}

/// A row's cell of a set's census.
type Cell = fn(&SetCensus) -> String;
/// A quantity's range in a set's census.
type Span = fn(&SetCensus) -> Option<Range>;

/// The census as a page.
fn markdown(report: &Report) -> String {
    let mut page = String::new();
    let sets = &report.sets;
    let header = |page: &mut String, first: &str| {
        let _ = write!(page, "| {first} |");
        for set in sets {
            let _ = write!(page, " {} |", title(set.set));
        }
        page.push('\n');
        page.push_str("|---|");
        for _ in sets {
            page.push_str("---:|");
        }
        page.push('\n');
    };
    page.push_str(
        "# Synthetic flights\n\n\
         The census of the synthetic flights (M7.2a, ADR-280): each set's flights by design and \
         by logger, the ranges they span, and each draw that didn't fly. Written by \
         `cargo xtask synthetic`; `cargo xtask synthetic --check` holds it on every platform. \
         Values are to three significant figures. What the flights are and how far to trust \
         them: `docs/physics/synthetic-flights.md`.\n\n",
    );
    page.push_str("## The sets\n\n");
    header(&mut page, "");
    let rows: [(&str, Cell); 5] = [
        ("Seed", |s| s.seed.to_string()),
        ("Drawn", |s| s.drawn.to_string()),
        ("Flown", |s| s.flown.to_string()),
        ("Of set B's model", |s| s.other_model.to_string()),
        ("Not flown", |s| s.not_flown.len().to_string()),
    ];
    for (name, cell) in rows {
        let _ = write!(page, "| {name} |");
        for set in sets {
            let _ = write!(page, " {} |", cell(set));
        }
        page.push('\n');
    }
    page.push_str("\n## Flights flown, by design\n\n");
    header(&mut page, "Design");
    for design in &DESIGNS {
        let _ = write!(page, "| {} |", design.name);
        for set in sets {
            let _ = write!(
                page,
                " {} |",
                set.by_design.get(design.name).copied().unwrap_or(0)
            );
        }
        page.push('\n');
    }
    page.push_str("\n## Flights flown, by logger\n\n");
    header(&mut page, "Logger");
    for profile in Profile::ALL {
        let _ = write!(page, "| {} |", profile.name());
        for set in sets {
            let _ = write!(
                page,
                " {} |",
                set.by_logger.get(profile.name()).copied().unwrap_or(0)
            );
        }
        page.push('\n');
    }
    page.push_str("\n## What the flights span\n\nLeast / median / greatest.\n\n");
    header(&mut page, "Quantity");
    let spans: [(&str, Span); 8] = [
        ("Apogee above the pad, m", |s| s.apogee_m),
        ("Top Mach number", |s| s.top_mach),
        ("Top vertical speed, m/s", |s| s.top_speed_m_s),
        ("Liftoff to landing, s", |s| s.flight_time_s),
        ("Wind, m/s", |s| s.wind_m_s),
        ("Rail from vertical, °", |s| s.tilt_deg),
        ("Site elevation, m", |s| s.elevation_m),
        ("Air's offset from standard, K", |s| s.temperature_offset_k),
    ];
    for (name, span) in spans {
        let _ = write!(page, "| {name} |");
        for set in sets {
            let _ = write!(page, " {} |", range(span(set)));
        }
        page.push('\n');
    }
    page.push_str("\n## Draws not flown\n\n");
    let missing: Vec<_> = sets
        .iter()
        .flat_map(|s| s.not_flown.iter().map(move |n| (title(s.set), n)))
        .collect();
    if missing.is_empty() {
        page.push_str("Every draw flew.\n");
    } else {
        page.push_str("| Set | Index | Design | Why |\n|---|---:|---|---|\n");
        for (set, n) in missing {
            let reason = n.reason.replace('|', "\\|").replace('\n', " ");
            let _ = writeln!(page, "| {set} | {} | {} | {reason} |", n.index, n.design);
        }
    }
    readings_section(&mut page, &report.readings);
    errors_section(&mut page, &report.errors, &report.tops);
    smoother_section(&mut page, &report.smoother, &report.cards);
    intervals_section(&mut page, &report.intervals);
    page
}

/// A share as a percentage to three significant figures.
fn percent(x: u64, n: u64) -> String {
    if n == 0 {
        return "none".to_owned();
    }
    format!("{}%", intervals::three_figures(100.0 * x as f64 / n as f64))
}

/// Counts as the page shows them: below / inside / above.
fn counts_text(c: &intervals::Counts) -> String {
    format!("{} / {} / {}", c.below, c.inside, c.above)
}

/// The readings' intervals (M7.2e1, ADR-308).
fn intervals_section(page: &mut String, made: &Intervals) {
    page.push_str("\n## The readings' 90% intervals\n\n");
    let _ = writeln!(
        page,
        "Each flight's own log read by `hpr_flightdata::readings`, and each reading that states a \
         90% interval compared with the flight's truth (M7.2e1, ADR-257 §1–2, ADR-308). An \
         interval runs from the reading less its 95% share of its bound to the reading less its \
         5% share; the shares are the 5% and 95% points of the readings' errors as shares of their \
         bounds on the calibration set's {} flights ({} of them the GNSS-only tracker's, which give \
         no apogee), a seed of their own that the test sets never share. `cargo xtask synthetic` fits them again and fails unless they equal the library's \
         to three significant figures. A reading **holds** when the two-sided 99.9% \
         Clopper–Pearson band on the share of truths below its interval and the band on the share \
         above each hold 5%, and the band on the share inside holds 90%; and every probe must \
         fail that test. The command also fails if a reading is withheld from a log with a barometer, or \
         read with no interval, on any of the three sets: either would leave the counts with its \
         misses.",
        made.calibration_flights, made.calibration_withheld
    );
    page.push_str("\n### The shares\n\n");
    page.push_str("| Reading | Readings fitted | 5% share | 95% share |\n|---|---:|---:|---:|\n");
    for s in &made.shares {
        let _ = writeln!(
            page,
            "| {} | {} | {} | {} |",
            s.quantity.name(),
            s.stated,
            number(s.fitted.low),
            number(s.fitted.high)
        );
    }
    page.push_str(
        "\n### The test\n\nThe truths below, inside and above each interval, then the shares, \
         each set's flights flown. Floors and withheld readings aren't compared.\n\n",
    );
    page.push_str(
        "| Set | Reading | Compared | Below / inside / above | Below | Above | Floors | Withheld \
         | Result |\n|---|---|---:|---:|---:|---:|---:|---:|---|\n",
    );
    for set in &made.sets {
        for c in &set.checks {
            let counts = &c.check.test.counts;
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                title(set.set),
                c.check.quantity.name(),
                counts.given,
                counts_text(counts),
                percent(counts.below, counts.given),
                percent(counts.above, counts.given),
                c.check.unstated,
                c.check.withheld.values().sum::<u64>(),
                if c.holds { "holds" } else { "**fails**" }
            );
        }
    }
    page.push_str(
        "\n### The probes\n\nEach interval narrowed to 0.8 of its width about its middle, widened \
         to 1.25, and shifted down by one stated standard error (its width over 2 × 1.645): \
         below / inside / above. Each must fail the test.\n\n",
    );
    page.push_str("| Set | Reading |");
    for probe in intervals::Probe::ALL {
        let _ = write!(page, " {} |", probe.name());
    }
    page.push_str("\n|---|---|");
    for _ in intervals::Probe::ALL {
        page.push_str("---:|");
    }
    page.push('\n');
    for set in &made.sets {
        for c in &set.checks {
            let _ = write!(page, "| {} | {} |", title(set.set), c.check.quantity.name());
            for probe in intervals::Probe::ALL {
                let test = &c.check.probes[&probe];
                let _ = write!(
                    page,
                    " {}, {} |",
                    counts_text(&test.counts),
                    if c.probe_passes[&probe] {
                        "**passes**"
                    } else {
                        "fails"
                    }
                );
            }
            page.push('\n');
        }
    }
    page.push_str(
        "\n### The apogee's error side\n\nThe apogee height's mean error, the reading less the \
         truth, over the readings compared (ADR-257 §2). A reading too low is the flattering side \
         for a waiver: the share of truths above the interval is that side's test, and the \
         shifted-down probe its mutation probe. The mean is given at the figures of the largest \
         error, the digits every platform shares, and with its sign: +0 is a mean too small for \
         those figures that isn't below zero.\n\n| Set | Mean error, m | Largest error, m | \
         Truths above |\n|---|---:|---:|---:|\n",
    );
    for set in &made.sets {
        if let Some(c) = set
            .checks
            .iter()
            .find(|c| c.check.quantity == Quantity::ApogeeHeight)
        {
            let counts = &c.check.test.counts;
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} |",
                title(set.set),
                signed(c.check.mean_error),
                number(c.check.largest_error),
                percent(counts.above, counts.given)
            );
        }
    }
    page.push_str("\n### By design\n\nBelow / inside / above.\n\n| Design |");
    for set in &made.sets {
        for c in &set.checks {
            let _ = write!(page, " {}, {} |", title(set.set), c.check.quantity.name());
        }
    }
    page.push_str("\n|---|");
    for set in &made.sets {
        for _ in &set.checks {
            page.push_str("---:|");
        }
    }
    page.push('\n');
    for design in DESIGNS {
        let _ = write!(page, "| {} |", design.name);
        for set in &made.sets {
            for c in &set.checks {
                let text = c
                    .check
                    .by_design
                    .get(design.name)
                    .map_or_else(|| "none".to_owned(), counts_text);
                let _ = write!(page, " {text} |");
            }
        }
        page.push('\n');
    }
    page.push_str(
        "\n### By logger and by the day's air\n\nBelow / inside / above, and in bold where that \
         group's counts alone would fail the test. The test is on each set as a whole; these \
         groups show where its misses fall. A barometric height is off by about the day's \
         offset from the standard atmosphere over its temperature, so the height's misses gather \
         on the coldest and warmest days: a warm day reads low, and its truth lies above the \
         interval.\n\n",
    );
    let groups =
        |page: &mut String,
         first: &str,
         rows: &[String],
         counts: &dyn Fn(&IntervalCheck, &str) -> Option<intervals::Counts>| {
            let _ = write!(page, "| {first} |");
            for set in &made.sets {
                for c in &set.checks {
                    let _ = write!(page, " {}, {} |", title(set.set), c.check.quantity.name());
                }
            }
            page.push_str("\n|---|");
            for set in &made.sets {
                for _ in &set.checks {
                    page.push_str("---:|");
                }
            }
            page.push('\n');
            for row in rows {
                let _ = write!(page, "| {row} |");
                for set in &made.sets {
                    for c in &set.checks {
                        let text = counts(c, row).map_or_else(
                            || "none".to_owned(),
                            |n| {
                                if intervals::Test::of(n).passes() {
                                    counts_text(&n)
                                } else {
                                    format!("**{}**", counts_text(&n))
                                }
                            },
                        );
                        let _ = write!(page, " {text} |");
                    }
                }
                page.push('\n');
            }
        };
    let loggers: Vec<String> = Profile::ALL
        .iter()
        .map(|p| p.name().to_owned())
        .filter(|name| {
            made.sets.iter().any(|s| {
                s.checks
                    .iter()
                    .any(|c| c.check.by_logger.contains_key(name))
            })
        })
        .collect();
    groups(page, "Logger", &loggers, &|c, row| {
        c.check.by_logger.get(row).copied()
    });
    page.push('\n');
    let bands: Vec<String> = intervals::AirBand::ALL
        .iter()
        .map(|b| b.name().to_owned())
        .collect();
    groups(page, "The day's air", &bands, &|c, row| {
        intervals::AirBand::ALL
            .iter()
            .find(|b| b.name() == row)
            .and_then(|b| c.check.by_air.get(b).copied())
    });
}

/// A cell as the page shows it: the share inside the interval, as a percentage, and how many
/// truths fell below and above it.
fn cell_text(c: &smoothed::Cell) -> String {
    format!(
        "{}% ({} / {}){}",
        number(census::three_figures(100.0 * c.coverage)),
        c.below,
        c.above,
        if c.holds { "" } else { " **fails**" }
    )
}

/// The page's section on the smoother held to each set's truth (M7.2d).
fn smoother_section(page: &mut String, sets: &[SmootherSet], cards: &[Card]) {
    page.push_str(
        "\n## The smoother against the truth\n\n\
         Each flight's own log smoothed (`hpr_flightdata::smoother`, M7.2d, ADR-300) and its \
         height above the pad and vertical speed compared with the truth's at burnout, the top \
         vertical speed, apogee and one moment drawn per flight. Each cell gives the share of \
         flights whose truth lies inside the smoother's 90% interval, then how many lay below and \
         above it. A cell **fails** when the lower edge of the 99.9% Clopper–Pearson band on \
         either side's miss share is above 5%. Logs without a barometer are withheld.\n\n",
    );
    let header = |page: &mut String| {
        page.push_str("| Moment | State |");
        for s in sets {
            let _ = write!(page, " {} |", title(s.set));
        }
        page.push_str("\n|---|---|");
        for _ in sets {
            page.push_str("---:|");
        }
        page.push('\n');
    };
    let rows = |page: &mut String,
                pick: &dyn Fn(&SmootherSet) -> &[smoothed::Cell],
                show: fn(&smoothed::Cell) -> String| {
        let Some(first) = sets.first() else { return };
        for (k, c) in pick(first).iter().enumerate() {
            let _ = write!(page, "| {} | {} |", c.moment.name(), c.state.name());
            for s in sets {
                let _ = write!(page, " {} |", pick(s).get(k).map_or_else(String::new, show));
            }
            page.push('\n');
        }
    };
    header(page);
    page.push_str("| Flights smoothed | |");
    for s in sets {
        let withheld: u64 = s.check.withheld.values().sum();
        let _ = write!(page, " {} |", s.check.flights - withheld);
    }
    page.push('\n');
    rows(page, &|s| &s.check.cells, cell_text);
    page.push_str(
        "\n### The standard errors\n\nThe median and the largest, of height in m and of speed in \
         m/s.\n\n",
    );
    header(page);
    rows(page, &|s| &s.check.cells, |c| {
        format!("{} / {}", number(c.median_sd), number(c.largest_sd))
    });
    page.push_str(
        "\n### By logger\n\nThe share inside the interval, each set's flights by logger.\n\n",
    );
    for s in sets {
        let _ = writeln!(page, "#### {}\n", title(s.set));
        page.push_str("| Moment | State |");
        for logger in s.check.by_logger.keys() {
            let _ = write!(page, " {logger} |");
        }
        page.push_str("\n|---|---|");
        for _ in s.check.by_logger.keys() {
            page.push_str("---:|");
        }
        page.push('\n');
        for (k, c) in s.check.cells.iter().enumerate() {
            let _ = write!(page, "| {} | {} |", c.moment.name(), c.state.name());
            for cells in s.check.by_logger.values() {
                let _ = write!(
                    page,
                    " {} |",
                    cells.get(k).map_or_else(String::new, cell_text)
                );
            }
            page.push('\n');
        }
        page.push('\n');
    }
    page.push_str(
        "### Trusting the GNSS receiver from liftoff\n\n\
         The same flights, each log with a GNSS track smoothed again with its receiver trusted \
         from liftoff rather than once its speed agrees with the smoother's. A receiver lags a \
         climb, so this must fail.\n\n",
    );
    header(page);
    page.push_str("| Flights smoothed again | |");
    for s in sets {
        let _ = write!(page, " {} |", s.gnss_rerun);
    }
    page.push('\n');
    rows(page, &|s| &s.gnss_from_liftoff, cell_text);
    page.push_str("\n### Health\n\nThe smoothed flights' health cards, summed.\n\n");
    let names: Vec<&String> = {
        let mut names: Vec<&String> = sets
            .iter()
            .flat_map(|s| s.check.health.sensors.keys())
            .collect();
        names.sort();
        names.dedup();
        names
    };
    page.push_str("| |");
    for s in sets {
        let _ = write!(page, " {} |", title(s.set));
    }
    page.push_str("\n|---|");
    for _ in sets {
        page.push_str("---:|");
    }
    page.push('\n');
    let line = |page: &mut String, name: &str, value: &dyn Fn(&SmootherSet) -> String| {
        let _ = write!(page, "| {name} |");
        for s in sets {
            let _ = write!(page, " {} |", value(s));
        }
        page.push('\n');
    };
    for name in names {
        let get = |s: &SmootherSet| {
            s.check
                .health
                .sensors
                .get(name)
                .cloned()
                .unwrap_or_default()
        };
        line(page, &format!("{name}: logs"), &|s| get(s).logs.to_string());
        line(
            page,
            &format!("{name}: samples rejected / logged / not weighed"),
            &|s| {
                let t = get(s);
                format!("{} / {} / {}", t.rejected, t.samples, t.unweighed)
            },
        );
        line(page, &format!("{name}: logs with a rejection"), &|s| {
            get(s).logs_rejecting.to_string()
        });
        line(page, &format!("{name}: resets"), &|s| {
            get(s).resets.to_string()
        });
    }
    line(page, "GNSS receivers trusted", &|s| {
        s.check.health.gnss_trusted.to_string()
    });
    line(page, "Transonic spans de-weighted", &|s| {
        s.check.health.transonic_spans.to_string()
    });
    line(page, "Charge spans de-weighted", &|s| {
        s.check.health.charge_spans.to_string()
    });
    line(page, "Steps the pass back kept forward", &|s| {
        s.check.health.unsmoothed_steps.to_string()
    });
    page.push_str("\n### A health card for each logger\n\nEach logger's first smoothed flight in the tuning set; times are on the log's clock.\n");
    for card in cards {
        let _ = write!(
            page,
            "\n**{}**, flight {}:\n\n```text\n{}```\n",
            card.logger, card.flight, card.card
        );
    }
}

/// How many of [`Quantity::ALL`]'s readings, from its start, are the height track's.
const HEIGHT_READINGS: usize = 7;

/// The page's section on the readings' errors over the tuning set, and the top accelerations
/// against their parts' ranges.
fn errors_section(page: &mut String, errors: &[Errors], tops: &Tops) {
    page.push_str(
        "\n## Errors on the tuning set\n\n\
         Each reading's errors, the reading less the truth, over every tuning flight and every \
         log above together, beside the mean bound the readings stated. Each error is held to its \
         own reading's bound, which differs from flight to flight and log to log, so the least or \
         the most may pass the mean bound; the mean is for scale, and is given at the decimal \
         place of the larger of the least and the most to three figures, the digits every \
         platform shares. A canopy's rates are compared with the truth over the span the reader \
         chose, its leg's ends, so an opening read early or late shows in the opening's error, \
         not in the rates'. A floor, a burnout from the \
         speed's peak or a clipped top acceleration, is apart: it holds while it is no more than \
         its bound above the truth, and may lie any amount below it.\n\n\
         | Reading | Given | Mean error | Least | Most | Mean bound |\n\
         |---|---:|---:|---:|---:|---:|\n",
    );
    for row in errors {
        let name = if row.floor {
            format!("{}, a floor", row.quantity.name())
        } else {
            row.quantity.name().to_owned()
        };
        let unit = row.quantity.unit();
        match &row.spread {
            Some(spread) => {
                let _ = writeln!(
                    page,
                    "| {name} | {} | {} {unit} | {} {unit} | {} {unit} | {} {unit} |",
                    row.given,
                    number(spread.mean_error),
                    number(spread.least_error),
                    number(spread.most_error),
                    number(spread.mean_bound),
                );
            }
            None => {
                let _ = writeln!(page, "| {name} | {} | none finite | | | |", row.given);
            }
        }
    }
    tops_paragraph(page, tops);
}

/// The paragraph on the top accelerations against their parts' ranges.
fn tops_paragraph(page: &mut String, tops: &Tops) {
    let Some(nearest) = &tops.nearest else {
        page.push_str("\nNo tuning flight's logger has an accelerometer.\n");
        return;
    };
    let reach = match tops.reaching {
        0 => "No tuning flight's true top acceleration reaches the range its own part clips at"
            .to_owned(),
        1 => "1 tuning flight's true top acceleration reaches the range its own part clips at"
            .to_owned(),
        n => format!(
            "{n} tuning flights' true top accelerations reach the range their own parts clip at"
        ),
    };
    let _ = write!(
        page,
        "\n{reach}: the nearest, flight {}'s on its {} logger, comes to {} of it. ",
        nearest.flight,
        nearest.profile,
        number(nearest.share)
    );
    if let Some(lower) = &tops.lower {
        let _ = write!(
            page,
            "With {}, {} of the {} flights logged so have a true top that reaches that range, and \
             {} don't. ",
            lower.log,
            lower.reaching,
            lower.flights,
            lower.flights - lower.reaching
        );
    }
    if tops.clipped.is_empty() {
        page.push_str("No top was read as a floor.\n");
        return;
    }
    let _ = writeln!(
        page,
        "Tops read as floors: {}; clipped at the part's range, {}; held flat at the top, {}; \
         where the accelerometer holds the thrust only in part, with no start or no end to it, {}.",
        tops.clipped.len(),
        tops.floors.at_range,
        tops.floors.flat,
        tops.floors.thrust_in_part
    );
    // Each cause's tops, by log and logger, in the order first met.
    for (cause, words) in [
        ("range", "Clipped at the part's range"),
        ("flat", "Held flat at the top"),
        ("thrust_in_part", "The thrust held in part"),
        ("other", "Clipped otherwise"),
    ] {
        let mut groups: Vec<((&str, &str), Vec<String>)> = Vec::new();
        for c in tops.clipped.iter().filter(|c| c.cause == cause) {
            let key = (c.log.as_str(), c.profile.as_str());
            match groups.iter_mut().find(|(k, _)| *k == key) {
                Some((_, flights)) => flights.push(c.flight.to_string()),
                None => groups.push((key, vec![c.flight.to_string()])),
            }
        }
        if !groups.is_empty() {
            let each: Vec<String> = groups
                .iter()
                .map(|((log, profile), flights)| {
                    format!("{log}, {profile} flights {}", flights.join(", "))
                })
                .collect();
            let _ = writeln!(page, "\n{words}: {}.", each.join("; "));
        }
    }
}

/// The page's section on the flight-log readings held to their bounds.
fn readings_section(page: &mut String, tallies: &[Tally]) {
    let _ = write!(
        page,
        "\n## Readings within their bounds\n\n\
         Each tuning flight logged by its own logger, then again at every rate and step of the \
         grid, from 500 to half a sample a second and from no step to 100 Pa, with its own \
         logger's accelerometer at 500, 100 and 20 Hz and, where its part can be, {}, so that \
         some tops clip at the part's range and the rest don't, and cropped as loggers start \
         their logs, and read by `hpr_flightdata::readings` (M7.2b, M7.2c). Each cell gives how \
         many of the flights' readings were given, then the largest share of its bound an error \
         took: 1 is the bound, and `cargo xtask synthetic` fails above it. The JSON names each \
         worst flight, its error and its bound. What is method and what is measured: \
         `docs/physics/log-readings.md`.\n\n",
        held::lower_range_name().trim_start_matches("its accelerometer ")
    );
    let mut logs: Vec<&str> = Vec::new();
    for tally in tallies {
        if !logs.contains(&tally.log.as_str()) {
            logs.push(&tally.log);
        }
    }
    // Two tables, the height track's readings and then the accelerometer's and the canopies',
    // so neither runs wider than a page.
    let (heights, rest) = Quantity::ALL.split_at(HEIGHT_READINGS);
    for (title, quantities) in [
        ("The height track's readings", heights),
        ("The accelerometer's readings and the canopies'", rest),
    ] {
        let _ = write!(page, "### {title}\n\n| Log |");
        for quantity in quantities {
            let _ = write!(page, " {} |", quantity.name());
        }
        page.push_str("\n|---|");
        for _ in quantities {
            page.push_str("---:|");
        }
        page.push('\n');
        for log in &logs {
            let _ = write!(page, "| {log} |");
            for quantity in quantities {
                let cell = tallies
                    .iter()
                    .find(|tally| tally.log == *log && tally.quantity == *quantity)
                    .map_or_else(
                        || "none".to_owned(),
                        |tally| match &tally.worst {
                            Some(worst) => format!("{}, {}", tally.given, number(worst.share)),
                            None => tally.given.to_string(),
                        },
                    );
                let _ = write!(page, " {cell} |");
            }
            page.push('\n');
        }
        page.push('\n');
    }
    page.push_str(
        "### Readings withheld\n\n\
         Each reading withheld, by reason; beside them, a drogue or a main's opening that the \
         truth has none of, on a flight that came down under one canopy or none, as none to \
         read.\n\n",
    );
    let withheld: Vec<(&Tally, String, u64)> = tallies
        .iter()
        .flat_map(|tally| {
            let absent =
                (tally.absent > 0).then(|| (tally, "none to read".to_owned(), tally.absent));
            tally
                .withheld
                .iter()
                .map(move |(reason, count)| {
                    let code = serde_json::to_value(reason)
                        .ok()
                        .and_then(|value| value.as_str().map(str::to_owned))
                        .unwrap_or_default();
                    (tally, format!("`{code}`"), *count)
                })
                .chain(absent)
        })
        .collect();
    if withheld.is_empty() {
        page.push_str("None was withheld.\n");
    } else {
        page.push_str("| Log | Reading | Reason | Flights |\n|---|---|---|---:|\n");
        for (tally, code, count) in withheld {
            let _ = writeln!(
                page,
                "| {} | {} | {code} | {count} |",
                tally.log,
                tally.quantity.name()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_health_sums_hold_the_digits_every_platform_shares() {
        let sums = |samples: u64, unsmoothed: u64| {
            let mut health = smoothed::HealthTally {
                unsmoothed_steps: unsmoothed,
                ..Default::default()
            };
            health.sensors.insert(
                "barometer".to_owned(),
                smoothed::SensorTally {
                    samples,
                    rejected: 13_436,
                    resets: 35,
                    ..Default::default()
                },
            );
            round_health(&mut health);
            health
        };
        // A log a sample shorter on one platform leaves the sums the same.
        assert_eq!(sums(4_859_244, 0), sums(4_859_243, 0));
        let held = &sums(4_859_244, 0).sensors["barometer"];
        assert_eq!(
            (held.samples, held.rejected, held.resets),
            (4_860_000, 13_400, 35)
        );
        // A step the pass back couldn't smooth still shows.
        assert_eq!(sums(4_859_244, 0).unsmoothed_steps, 0);
        assert_eq!(sums(4_859_244, 1).unsmoothed_steps, 1);
    }

    #[test]
    fn the_threaded_readings_are_the_readings_taken_in_order() {
        let root = crate::designs::root().unwrap();
        let files = design_files(&root).unwrap();
        let mut flyer = Flyer::new(files.clone());
        let (flights, tops): (Vec<FlightOutcomes>, Vec<FlightTop>) = (0..3)
            .map(|index| {
                let flight = flyer.flight(Set::Tuning, index).unwrap();
                let logs = held::logs(&flight, Set::Tuning.seed(), index).unwrap();
                let outcomes = logs
                    .iter()
                    .map(|(name, log)| (name.clone(), held::outcomes(&flight, log)))
                    .collect();
                let top = (
                    index,
                    flight.draw.logger.profile.name(),
                    held::top_over_range(&flight),
                    held::top_over_lower_range(&flight),
                );
                ((index, outcomes), top)
            })
            .unzip();
        assert_eq!(read_logs(3, &files).unwrap(), (flights, tops));
    }

    /// A tally whose worst error passes its bound is named, with the flight, and fails the
    /// command; one at its bound holds.
    #[test]
    fn a_reading_past_its_bound_is_named() {
        let tally = |share: f64| Tally {
            log: "20 Hz, altitude, no step".to_owned(),
            quantity: Quantity::Landing,
            given: 1,
            withheld: std::collections::BTreeMap::new(),
            absent: 0,
            worst: Some(held::Worst {
                flight: 7,
                error: -share,
                bound: 1.0,
                share,
            }),
        };
        assert!(misses(&[tally(1.0)]).is_empty());
        assert_eq!(
            misses(&[tally(1.0), tally(1.5)]),
            ["20 Hz, altitude, no step, landing: flight 7, error -1.5 against a bound of 1 s"]
        );
    }

    #[test]
    fn the_threaded_census_is_the_census_flown_in_order() {
        let root = crate::designs::root().unwrap();
        let files = design_files(&root).unwrap();
        for set in Set::ALL {
            assert_eq!(fly(set, 12, &files), census::census(set, 12, files.clone()));
        }
    }

    #[test]
    fn the_page_escapes_a_reasons_bars_and_line_breaks() {
        let root = crate::designs::root().unwrap();
        let files = design_files(&root).unwrap();
        let rows = vec![
            Err("first | second\nthird".to_owned()),
            census::row(&mut Flyer::new(files), Set::A, 1),
        ];
        let page = markdown(&Report {
            about: String::new(),
            sets: vec![census::from_rows(Set::A, &rows)],
            readings: Vec::new(),
            errors: Vec::new(),
            tops: Tops {
                reaching: 0,
                nearest: None,
                lower: None,
                floors: Floors::default(),
                clipped: Vec::new(),
            },
            smoother: Vec::new(),
            cards: Vec::new(),
            intervals: Intervals {
                calibration_flights: 0,
                calibration_withheld: 0,
                shares: Vec::new(),
                sets: Vec::new(),
            },
        });
        assert!(page.contains("| A | 0 |"), "{page}");
        assert!(page.contains("first \\| second third |"), "{page}");
    }

    /// The intervals' section names a failing reading, and each probe that passes, in bold.
    #[test]
    fn the_intervals_section_marks_what_fails() {
        use hpr_flightdata::readings::Interval;
        use intervals::{Compared, Said};
        // Every truth at the middle of its interval: no tail misses its 5%, so the test fails, and
        // so does every probe.
        let flights: Vec<FlightSaid> = (0..100)
            .map(|index| FlightSaid {
                index,
                design: 0,
                logger: "altos",
                air_offset_k: 0.0,
                said: vec![(
                    Quantity::ApogeeHeight,
                    Said::Stated(Compared {
                        reading: 0.0,
                        bound: 1.0,
                        interval: Interval {
                            low: -1.0,
                            high: 1.0,
                        },
                        truth: 0.0,
                    }),
                )],
            })
            .collect();
        let check = intervals::check(Quantity::ApogeeHeight, &flights);
        let probe_passes = check
            .probes
            .iter()
            .map(|(probe, test)| (*probe, test.passes()))
            .collect();
        let made = Intervals {
            calibration_flights: 2000,
            calibration_withheld: 362,
            shares: vec![FittedShares {
                quantity: Quantity::ApogeeHeight,
                stated: 1638,
                fitted: Shares {
                    low: -0.63,
                    high: 0.637,
                },
                library: Shares {
                    low: -0.63,
                    high: 0.637,
                },
            }],
            sets: vec![IntervalSet {
                set: Set::A,
                checks: vec![IntervalCheck {
                    holds: check.holds(),
                    probe_passes,
                    check,
                }],
            }],
        };
        let mut page = String::new();
        intervals_section(&mut page, &made);
        assert!(
            page.contains("| apogee height | 1638 | -0.63 | 0.637 |"),
            "{page}"
        );
        assert!(
            page.contains(
                "| A | apogee height | 100 | 0 / 100 / 0 | 0% | 0% | 0 | 0 | **fails** |"
            ),
            "{page}"
        );
        assert!(page.contains("0 / 100 / 0, fails |"), "{page}");
        assert!(
            page.contains("| rocketpy-bella-lui | 0 / 100 / 0 |"),
            "{page}"
        );
        // Each group's counts alone fail too: in bold.
        assert!(page.contains("| altos | **0 / 100 / 0** |"), "{page}");
        assert!(
            page.contains("| −10 to +10 K | **0 / 100 / 0** |"),
            "{page}"
        );
    }

    /// A reading withheld from a log with a barometer, or read with no interval, would leave the
    /// counts with its misses: each is named. The GNSS-only tracker's withheld apogee isn't.
    #[test]
    fn a_reading_that_leaves_the_counts_is_named() {
        use hpr_flightdata::readings::Reason;
        use intervals::Said;
        let flight = |logger: &'static str, said: Said| FlightSaid {
            index: 7,
            design: 0,
            logger,
            air_offset_k: 0.0,
            said: vec![(Quantity::ApogeeHeight, said)],
        };
        let flights = [
            flight("gnss-tracker", Said::Withheld(Reason::ReceiverLag)),
            flight("altos", Said::Withheld(Reason::NoClimb)),
            flight("rrc3", Said::Unstated),
        ];
        let problems = dropped(Set::A, &flights);
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(problems[0].contains("a flight 7 (altos): apogee height withheld (NoClimb)"));
        assert!(problems[1].contains("(rrc3): apogee height read with no interval"));
    }

    /// The tops: a share of 1 reaches the range, the nearest is the largest share, the flights
    /// set to a lower range are counted against it, and each top read as a floor is named by its
    /// flight's profile, its log and its cause, and counted by cause; with none, the page says so.
    #[test]
    fn the_tops_name_the_nearest_and_each_clipped() {
        let floor = |clip: Option<Clip>| Outcome::Floor {
            error: -1.0,
            bound: 2.0,
            clip,
        };
        let given = Outcome::Given {
            error: 0.5,
            bound: 2.0,
        };
        let lower = held::lower_range_name();
        let flights: Vec<FlightOutcomes> = vec![
            (
                0,
                vec![
                    ("altos".to_owned(), vec![(Quantity::TopAcceleration, given)]),
                    (
                        "its accelerometer at 20 Hz".to_owned(),
                        vec![
                            (Quantity::TopAcceleration, floor(Some(Clip::Flat))),
                            (Quantity::Burnout, floor(None)),
                        ],
                    ),
                ],
            ),
            (
                1,
                vec![("rrc3".to_owned(), vec![(Quantity::Burnout, floor(None))])],
            ),
            (
                2,
                vec![
                    (
                        "imu-500hz".to_owned(),
                        vec![(Quantity::TopAcceleration, given)],
                    ),
                    (
                        lower.clone(),
                        vec![(Quantity::TopAcceleration, floor(Some(Clip::Range)))],
                    ),
                ],
            ),
            (
                3,
                vec![(
                    "imu-500hz".to_owned(),
                    vec![(Quantity::TopAcceleration, floor(None))],
                )],
            ),
        ];
        let each: Vec<FlightTop> = vec![
            (0, "altos", Some(0.25), None),
            (1, "rrc3", None, None),
            (2, "imu-500hz", Some(1.0), Some(4.0)),
            (3, "imu-500hz", Some(0.1), Some(0.4)),
        ];
        let made = tops(&flights, &each);
        assert_eq!(made.reaching, 1);
        let nearest = made.nearest.as_ref().unwrap();
        assert_eq!((nearest.flight, nearest.share), (2, 1.0));
        assert_eq!(
            made.lower,
            Some(Lower {
                log: lower.clone(),
                flights: 2,
                reaching: 1,
            })
        );
        assert_eq!(
            made.floors,
            Floors {
                at_range: 1,
                flat: 1,
                thrust_in_part: 1,
            }
        );
        let causes: Vec<(u64, &str, &str)> = made
            .clipped
            .iter()
            .map(|c| (c.flight, c.log.as_str(), c.cause.as_str()))
            .collect();
        assert_eq!(
            causes,
            [
                (0, "its accelerometer at 20 Hz", "flat"),
                (2, lower.as_str(), "range"),
                (3, "imu-500hz", "thrust_in_part"),
            ]
        );
        let mut page = String::new();
        tops_paragraph(&mut page, &made);
        assert!(
            page.contains("1 tuning flight's true top acceleration reaches"),
            "{page}"
        );
        assert!(
            page.contains(&format!(
                "With {lower}, 1 of the 2 flights logged so have a true top that reaches"
            )),
            "{page}"
        );
        assert!(
            page.contains(
                "Tops read as floors: 3; clipped at the part's range, 1; held flat at the top, 1; \
                 where the accelerometer holds the thrust only in part, with no start or no end to \
                 it, 1."
            ),
            "{page}"
        );
        assert!(
            page.contains("Held flat at the top: its accelerometer at 20 Hz, altos flights 0."),
            "{page}"
        );
        assert!(
            page.contains(&format!(
                "Clipped at the part's range: {lower}, imu-500hz flights 2."
            )),
            "{page}"
        );
        assert!(
            page.contains("The thrust held in part: imu-500hz, imu-500hz flights 3."),
            "{page}"
        );
        let none = tops(&flights[..1], &[(0, "altos", Some(0.25), None)]);
        let mut page = String::new();
        tops_paragraph(&mut page, &none);
        assert!(
            page.contains("No tuning flight's true top acceleration reaches"),
            "{page}"
        );
        assert!(!page.contains("logged so"), "{page}");
        let mut page = String::new();
        tops_paragraph(&mut page, &tops(&[], &[]));
        assert!(
            page.contains("No tuning flight's logger has an accelerometer"),
            "{page}"
        );
    }

    /// The height track's readings are the first of [`Quantity::ALL`], the table's split.
    #[test]
    fn the_first_table_holds_the_height_tracks_readings() {
        assert_eq!(Quantity::ALL[HEIGHT_READINGS - 1], Quantity::DescentRate);
        assert_eq!(Quantity::ALL[HEIGHT_READINGS], Quantity::TopAcceleration);
    }

    #[test]
    fn a_number_reads_as_the_census_holds_it() {
        assert_eq!(number(4_810.0), "4810");
        assert_eq!(number(0.269), "0.269");
        assert_eq!(number(-14.6), "-14.6");
        assert_eq!(number(-0.0), "0");
        assert_eq!(number(0.000_123), "0.000123");
    }

    #[test]
    fn a_mean_rounded_to_zero_keeps_its_sign() {
        let mean = |x: f64| signed(signed_mean(x, 237.0));
        assert_eq!(mean(0.0742), "+0");
        assert_eq!(mean(-0.0738), "-0");
        assert_eq!(mean(1.07), "+1");
        assert_eq!(mean(-1.6), "-2");
        assert_eq!(mean(f64::NAN), "NaN");
    }
}
