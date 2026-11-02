//! `cargo xtask fit`: the least-squares fit's 90% intervals held to ADR-257's test on the synthetic
//! flights (M7.3b, ADR-316): each parameter's three counts on sets A and B, its three probes, what
//! the log can tell of it, and every parameter on pure noise.
//!
//! The flights are flown and fitted on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a fit is the same however many came before it on its thread (the shared
//! tables change no number), so the report doesn't depend on the number of threads.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::time::Instant;

use hpr_flightdata::readings::interval::Shares;
use hpr_forensics::synthetic::designs::DesignFiles;
use hpr_forensics::synthetic::fits::{self, ASKED, Asked, Check, FlightFit, Identification};
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "  fit [--check]            Fit every flight of the synthetic sets A and B by least squares (
                           ADR-316) and write validation/reports/fit-intervals.{json,md}. --check
                           writes nothing and fails unless the fits reproduce the committed report
                           and the calibration set gives the library's shares. Either fails if a
                           parameter's interval fails ADR-257's test, a probe of it passes, a scale
                           isn't identified on set A, or anything is identified on pure noise.
  fit --check --other-platform
                           The same rules, on a platform other than the one that wrote the report:
                           its counts may stand a few flights off, and calibration isn't rerun.
  fit --calibrate          Fit the calibration set and print each parameter's shares; writes nothing.
  fit --try <set> <count>  Fit a set's first flights, print each parameter's shares and counts on
                           them, and the seconds a fit takes; writes nothing.";

/// The report's path under the repository, without its extension.
const REPORT: &str = "validation/reports/fit-intervals";
/// Flights with a track logger the shares are fitted on, from the calibration set.
const CALIBRATION_TRACK_FLIGHTS: u64 = 1_000;
/// Flights with a track logger the wind is counted on, from each test set. The wind's errors have
/// long tails, so a narrowed interval moves few flights: at 1,010 the probe narrowed to 0.8 came
/// out at the edge of its band on set B, and the bands are drawn tighter here for every count.
const TRACK_FLIGHTS: u64 = 1_500;
/// Flights counted on the drag, impulse and mass, from each test set: each with something to fit,
/// since a log that said nothing states its range whatever the fit does.
const COUNTED_FLIGHTS: usize = 1_000;
/// Flights fitted from each test set to find them, at the first of which about 82% have something
/// to fit (the report prints it).
const FIT_FLIGHTS: u64 = 1_300;
/// Flights of set A given pure noise in place of their logs.
const NOISE_FLIGHTS: u64 = 200;
/// Pure noise, each given in place of a log: what fills it, and its size, m.
const NOISES: [Noise; 7] = [
    // A sensor's size and a flight's, so that the reader may find a climb in it.
    Noise {
        how: Noisy::Log,
        size_m: 5.0,
        track_m: None,
    },
    Noise {
        how: Noisy::Log,
        size_m: 1_000.0,
        track_m: None,
    },
    // Past the reader, which turns those away: the fit itself, at sizes from a sensor's to a
    // flight's, of height alone and with a track of noise, which is what the wind listens to.
    Noise {
        how: Noisy::Bins,
        size_m: 5.0,
        track_m: None,
    },
    Noise {
        how: Noisy::Bins,
        size_m: 50.0,
        track_m: None,
    },
    Noise {
        how: Noisy::Bins,
        size_m: 1_000.0,
        track_m: None,
    },
    Noise {
        how: Noisy::Bins,
        size_m: 5.0,
        track_m: Some(5.0),
    },
    Noise {
        how: Noisy::Bins,
        size_m: 1_000.0,
        track_m: Some(5.0),
    },
];

/// What's given in place of a log.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Noise {
    how: Noisy,
    size_m: f64,
    track_m: Option<f64>,
}

/// How noise reaches a fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Noisy {
    /// In place of a log's heights, through the reader.
    Log,
    /// As the bins the reader would have handed on.
    Bins,
}

/// Flights `indices` of `set`, or pure noise of `noise_size_m` in place of their logs, on every
/// core, in index order.
fn fit_set(
    set: Set,
    indices: &[u64],
    files: &DesignFiles,
    noise: Option<Noise>,
) -> Result<Vec<FlightFit>, String> {
    let threads = std::thread::available_parallelism().map_or(1, usize::from);
    let results: Vec<Result<Vec<FlightFit>, String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|thread| {
                scope.spawn(move || -> Result<Vec<FlightFit>, String> {
                    let mut flyer = Flyer::new(files.clone());
                    let mut tables = BTreeMap::new();
                    let mut made = Vec::new();
                    for &index in indices.iter().skip(thread).step_by(threads) {
                        let flight = flyer
                            .flight(set, index)
                            .map_err(|e| format!("{set:?} flight {index}: {e}"))?;
                        let log = match noise {
                            Some(Noise {
                                how: Noisy::Bins,
                                size_m,
                                track_m,
                            }) => {
                                made.push(fits::fit_noise_bins(
                                    &flight,
                                    set,
                                    size_m,
                                    track_m,
                                    files,
                                    &mut tables,
                                ));
                                continue;
                            }
                            Some(Noise {
                                how: Noisy::Log,
                                size_m,
                                ..
                            }) => fits::noise_log(&flight.log, size_m, set.seed(), index),
                            None => flight.log.clone(),
                        };
                        made.push(
                            fits::fit_log(&log, &flight, set, files, &mut tables)
                                .map_err(|e| format!("{set:?} flight {index}: {e}"))?,
                        );
                    }
                    Ok(made)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .unwrap_or_else(|_| Err("a thread panicked".to_owned()))
            })
            .collect()
    });
    let mut flights = Vec::new();
    for result in results {
        flights.extend(result?);
    }
    flights.sort_by_key(|f| f.index);
    Ok(flights)
}

/// Flights `0..count`.
fn first(count: u64) -> Vec<u64> {
    (0..count).collect()
}

/// `set`'s flights and the wind's flights with a track, fitted, each once.
fn fit_with_track(
    set: Set,
    flights: u64,
    track_flights: u64,
    files: &DesignFiles,
) -> Result<(Vec<FlightFit>, Vec<FlightFit>), String> {
    let whole = first(flights);
    let track = fits::track_flights(set, track_flights);
    let extra: Vec<u64> = track.iter().copied().filter(|k| *k >= flights).collect();
    let made = fit_set(set, &whole, files, None)?;
    let more = fit_set(set, &extra, files, None)?;
    let tracked: Vec<FlightFit> = made
        .iter()
        .chain(&more)
        .filter(|f| track.contains(&f.index))
        .cloned()
        .collect();
    Ok((made, tracked))
}

/// A parameter's check on a set, for the report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SetReport {
    set: Set,
    /// Flights counted, each with something to fit.
    flights: u64,
    /// The set's first flights that it took to find them.
    drawn: u64,
    /// Of those, the logs with nothing to fit, each parameter then as stated, and not counted.
    said_nothing: u64,
    /// Counted fits that reached their iterations' limit, a truth's fit among them.
    not_converged: u64,
    /// The largest residual of a counted fit, as a share of the spread of the heights read.
    largest_misfit: f64,
    checks: Vec<Check>,
}

/// One size of noise's identification, for the report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NoiseReport {
    how: Noisy,
    size_m: f64,
    track_m: Option<f64>,
    /// The least residual, as a share of the spread of the heights read, of those that came to one.
    smallest_misfit: Option<f64>,
    flights: u64,
    said_nothing: u64,
    identified: BTreeMap<Asked, Identification>,
}

/// The committed report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Report {
    about: String,
    /// Each parameter's shares on a log of height alone, then one with a horizontal track.
    shares: BTreeMap<Asked, [Shares; 2]>,
    sets: Vec<SetReport>,
    noise: Vec<NoiseReport>,
}

fn three(x: f64) -> f64 {
    hpr_forensics::synthetic::intervals::three_figures(x)
}

/// How far apart two platforms' reports may stand: the same seeds fly with another math library,
/// so a flight near an interval's end can fall on its other side. Read on CI's Linux runner
/// against the report macOS wrote (ADR-316).
const OTHER_PLATFORM_FLIGHTS: u64 = 5;
/// The same for a figure that isn't a count, its share of the larger; the report's figures are held
/// to three, and a mean error sits near zero, hence the second, absolute allowance.
const OTHER_PLATFORM_SHARE: f64 = 0.05;
const OTHER_PLATFORM_ABSOLUTE: f64 = 0.002;

/// What separates a report from the committed one.
#[derive(Default)]
struct Apart {
    differing: u64,
    most_flights: u64,
    most_share: f64,
    broken: Vec<String>,
}

impl Apart {
    fn walk(&mut self, at: &str, committed: &serde_json::Value, made: &serde_json::Value) {
        use serde_json::Value;
        match (committed, made) {
            (Value::Object(a), Value::Object(b)) => {
                if a.len() != b.len() {
                    self.broken.push(format!("{at}: its fields differ"));
                }
                for (key, value) in a {
                    match b.get(key) {
                        Some(other) => self.walk(&format!("{at}/{key}"), value, other),
                        None => self.broken.push(format!("{at}/{key}: not made")),
                    }
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    self.broken.push(format!("{at}: its length differs"));
                }
                for (k, (value, other)) in a.iter().zip(b).enumerate() {
                    self.walk(&format!("{at}/{k}"), value, other);
                }
            }
            (Value::Number(a), Value::Number(b)) if a != b => {
                self.differing += 1;
                if let (Some(a), Some(b)) = (a.as_u64(), b.as_u64()) {
                    let flights = a.abs_diff(b);
                    self.most_flights = self.most_flights.max(flights);
                    if flights > OTHER_PLATFORM_FLIGHTS {
                        self.broken.push(format!("{at}: {a} committed, {b} made"));
                    }
                } else if a.is_f64() != b.is_f64() {
                    self.broken.push(format!("{at}: a count against a figure"));
                } else {
                    let (a, b) = (
                        a.as_f64().unwrap_or(f64::NAN),
                        b.as_f64().unwrap_or(f64::NAN),
                    );
                    let apart = (a - b).abs();
                    let share = apart / a.abs().max(b.abs());
                    self.most_share = self.most_share.max(share);
                    if !(apart <= OTHER_PLATFORM_ABSOLUTE || share <= OTHER_PLATFORM_SHARE) {
                        self.broken.push(format!("{at}: {a} committed, {b} made"));
                    }
                }
            }
            _ if committed != made => self.broken.push(format!("{at}: not as committed")),
            _ => {}
        }
    }
}

/// The set's report, with what doesn't come out as it must.
fn set_report(
    set: Set,
    fits_made: &[FlightFit],
    tracked: &[FlightFit],
) -> (SetReport, Vec<String>) {
    let mut broken = Vec::new();
    let checks: Vec<Check> = ASKED
        .iter()
        .map(|&asked| {
            let on = if asked.stated_on_height_alone() {
                fits_made
            } else {
                tracked
            };
            fits::check(asked, on)
        })
        .collect();
    for check in &checks {
        if !check.test.passes() {
            let c = check.test.counts;
            broken.push(format!(
                "{set:?}, {}: {} below, {} inside, {} above of {}: fails the test",
                check.asked.name(),
                c.below,
                c.inside,
                c.above,
                c.given
            ));
        }
        for (probe, test) in &check.probes {
            if test.passes() {
                broken.push(format!(
                    "{set:?}, {}: the probe {} passes, {} below, {} inside, {} above of {}",
                    check.asked.name(),
                    probe.name(),
                    test.counts.below,
                    test.counts.inside,
                    test.counts.above,
                    test.counts.given
                ));
            }
        }
        if set == Set::A
            && matches!(check.asked, Asked::DragScale | Asked::ImpulseScale)
            && !check.identification.identified
        {
            broken.push(format!(
                "{}: not identified on set A, median {} against {} stated",
                check.asked.name(),
                check.identification.median_width,
                check.identification.stated_width
            ));
        }
    }
    (
        SetReport {
            set,
            flights: fits_made.len() as u64,
            drawn: 0,
            said_nothing: 0,
            not_converged: 0,
            largest_misfit: 0.0,
            checks,
        },
        broken,
    )
}

/// Rounds a report's real numbers to three figures, so that its text moves only with a result.
fn rounded(mut report: Report) -> Report {
    for set in &mut report.sets {
        set.largest_misfit = three(set.largest_misfit);
        for check in &mut set.checks {
            check.mean_error = three(check.mean_error);
            let id = &mut check.identification;
            id.median_width = three(id.median_width);
            id.stated_width = three(id.stated_width);
        }
    }
    for noise in &mut report.noise {
        noise.smallest_misfit = noise.smallest_misfit.map(three);
        for id in noise.identified.values_mut() {
            id.median_width = three(id.median_width);
            id.stated_width = three(id.stated_width);
        }
    }
    report
}

/// The report's page.
fn page(report: &Report) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# The least-squares fit's 90% intervals\n\n{}\n\nWritten by `cargo xtask fit`. How to read \
         it: [the fit's page](../../docs/physics/flight-fit.md).\n",
        report.about
    );
    for set in &report.sets {
        let _ = writeln!(
            out,
            "## Set {:?}: {} flights counted\n\nOf its first {}, {} had nothing to fit and aren't \
             counted. A wind is counted on {TRACK_FLIGHTS} flights of the logger with a track, \
             some past these, each with something to fit. {} counted fits reached their iterations' limit. \
             The largest residual is {} of the spread of the heights read.\n\n| Parameter | Below | Inside | Above | \
             Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |\n\
             |---|---:|---:|---:|---|---|---:|---:|---|",
            set.set,
            set.flights,
            set.drawn,
            set.said_nothing,
            set.not_converged,
            set.largest_misfit,
        );
        for c in &set.checks {
            let probes: Vec<&str> = c
                .probes
                .values()
                .map(|t| if t.passes() { "passes" } else { "fails" })
                .collect();
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} / {} | {} |",
                c.asked.name(),
                c.test.counts.below,
                c.test.counts.inside,
                c.test.counts.above,
                if c.test.passes() { "passes" } else { "fails" },
                probes.join(", "),
                c.mean_error,
                c.identification.median_width,
                c.identification.stated_width,
                if c.identification.identified {
                    "yes"
                } else {
                    "no"
                }
            );
        }
        let _ = writeln!(out, "\nBy design (below / inside / above):\n");
        let names: Vec<String> = set
            .checks
            .first()
            .map(|c| c.by_design.keys().cloned().collect())
            .unwrap_or_default();
        let header: Vec<&str> = set.checks.iter().map(|c| c.asked.name()).collect();
        let _ = writeln!(out, "| Design | {} |", header.join(" | "));
        let _ = writeln!(out, "|---|{}", "---:|".repeat(set.checks.len()));
        for name in &names {
            let cells: Vec<String> = set
                .checks
                .iter()
                .map(|c| {
                    let d = c.by_design.get(name).copied().unwrap_or_default();
                    format!("{} / {} / {}", d.below, d.inside, d.above)
                })
                .collect();
            let _ = writeln!(out, "| {name} | {} |", cells.join(" | "));
        }
        let _ = writeln!(
            out,
            "\nBy how well the flight's heights fit (below / inside / above):\n"
        );
        let _ = writeln!(out, "| Residual | {} |", header.join(" | "));
        let _ = writeln!(out, "|---|{}", "---:|".repeat(set.checks.len()));
        for name in ["heights fit within 1 m", "heights fit to 1 m or worse"] {
            let cells: Vec<String> = set
                .checks
                .iter()
                .map(|c| {
                    let d = c.by_residual.get(name).copied().unwrap_or_default();
                    format!("{} / {} / {}", d.below, d.inside, d.above)
                })
                .collect();
            let _ = writeln!(out, "| {name} | {} |", cells.join(" | "));
        }
        let _ = writeln!(out);
    }
    let _ = writeln!(
        out,
        "## Pure noise in place of the log\n\nThe smallest residual is that of the fits that came to \
         one, a share of the spread of the heights read; past {} a log says nothing.\n\n| Height noise \
         | Track noise | Reaches | Flights | With nothing to fit | Smallest residual | Parameter | \
         Median width / stated | Identified |\n|---:|---:|---|---:|---:|---:|---|---:|---|",
        hpr_forensics::fit::MISFIT_SHARE
    );
    for noise in &report.noise {
        for (asked, id) in &noise.identified {
            let _ = writeln!(
                out,
                "| ±{} m | {} | {} | {} | {} | {} | {} | {} / {} | {} |",
                noise.size_m,
                noise
                    .track_m
                    .map_or("none".to_owned(), |track_m| format!("±{track_m} m")),
                match noise.how {
                    Noisy::Log => "the reader",
                    Noisy::Bins => "the fit",
                },
                noise.flights,
                noise.said_nothing,
                noise
                    .smallest_misfit
                    .map_or("none".to_owned(), |share| share.to_string()),
                asked.name(),
                id.median_width,
                id.stated_width,
                if id.identified { "yes" } else { "no" }
            );
        }
    }
    out
}

/// Makes the report, with what doesn't come out as it must.
fn make(files: &DesignFiles) -> Result<(Report, Vec<String>), String> {
    let mut broken = Vec::new();
    let mut sets = Vec::new();
    for set in [Set::A, Set::B] {
        let (made, tracked) = fit_with_track(set, FIT_FLIGHTS, TRACK_FLIGHTS, files)?;
        let fitted = made.iter().filter(|f| f.fitted).count();
        if fitted < COUNTED_FLIGHTS {
            broken.push(format!(
                "{set:?}: {fitted} of {} flights have something to fit, under {COUNTED_FLIGHTS}",
                made.len()
            ));
        }
        let counted: Vec<FlightFit> = made
            .iter()
            .filter(|f| f.fitted)
            .take(COUNTED_FLIGHTS)
            .cloned()
            .collect();
        let (mut report, problems) = set_report(set, &counted, &tracked);
        report.drawn = counted.last().map_or(0, |f| f.index + 1);
        report.said_nothing = made
            .iter()
            .filter(|f| !f.fitted && f.index < report.drawn)
            .count() as u64;
        let tally =
            |flights: &[FlightFit]| flights.iter().filter(|f| f.fitted && !f.converged).count();
        report.not_converged = (tally(&counted) + tally(&tracked)) as u64;
        report.largest_misfit = counted
            .iter()
            .chain(&tracked)
            .filter(|f| f.fitted)
            .filter_map(|f| f.misfit_share)
            .fold(0.0, f64::max);
        sets.push(report);
        broken.extend(problems);
    }
    let mut noise = Vec::new();
    for given in NOISES {
        let made = fit_set(Set::A, &first(NOISE_FLIGHTS), files, Some(given))?;
        let Noise {
            how,
            size_m,
            track_m,
        } = given;
        let identified: BTreeMap<Asked, Identification> = ASKED
            .iter()
            .map(|&asked| (asked, Identification::of(asked, &made)))
            .collect();
        for (asked, id) in &identified {
            if id.identified {
                broken.push(format!(
                    "{}: identified on noise of ±{size_m} m, median {} against {} stated",
                    asked.name(),
                    id.median_width,
                    id.stated_width
                ));
            }
        }
        noise.push(NoiseReport {
            how,
            size_m,
            track_m,
            smallest_misfit: made
                .iter()
                .filter_map(|f| f.misfit_share)
                .min_by(f64::total_cmp),
            flights: made.len() as u64,
            said_nothing: made.iter().filter(|f| !f.fitted).count() as u64,
            identified,
        });
    }
    Ok((
        rounded(Report {
            about: "On sets A and B of the synthetic flights, each parameter's 90% interval is held \
                    to three counts, each with a two-sided 99.9% Clopper–Pearson band: the two tails' \
                    bands must hold 5% and the inside's 90%. Shares fitted on the calibration set."
                .to_owned(),
            shares: ASKED
                .iter()
                .map(|&a| (a, [fits::shares(a, false), fits::shares(a, true)]))
                .collect(),
            sets,
            noise,
        }),
        broken,
    ))
}

pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::designs::root()?;
    let files = crate::synthetic::design_files(&root)?;
    match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        ["--calibrate"] => {
            let (made, tracked) = fit_with_track(
                Set::Calibration,
                Set::Calibration.count(),
                CALIBRATION_TRACK_FLIGHTS,
                &files,
            )?;
            for asked in ASKED {
                println!(
                    "{asked:?}: height {:?}, track {:?}",
                    fits::fitted(asked, false, &made),
                    fits::fitted(asked, true, &tracked)
                );
            }
            Ok(())
        }
        ["--try", set, count] => {
            let set = match set {
                "a" => Set::A,
                "b" => Set::B,
                "calibration" => Set::Calibration,
                other => return Err(format!("--try takes a, b or calibration, not {other}")),
            };
            let count: u64 = count.parse().map_err(|_| "--try takes a count")?;
            let start = Instant::now();
            let made = fit_set(set, &first(count), &files, None)?;
            let seconds = start.elapsed().as_secs_f64();
            let flown: u64 = made.iter().map(|f| f.flown).sum();
            println!(
                "{} fits in {seconds:.1} s wall; {:.1} flights flown a fit; {} with nothing to fit",
                made.len(),
                flown as f64 / made.len().max(1) as f64,
                made.iter().filter(|f| !f.fitted).count()
            );
            for asked in ASKED {
                let check = fits::check(asked, &made);
                let c = check.test.counts;
                println!(
                    "{:32} shares, height {:?}, track {:?}; as stated {}/{}/{}; mean error {:.4}; median width {:.4} of {:.4}",
                    asked.name(),
                    fits::fitted(asked, false, &made),
                    fits::fitted(asked, true, &made),
                    c.below,
                    c.inside,
                    c.above,
                    check.mean_error,
                    check.identification.median_width,
                    check.identification.stated_width
                );
            }
            Ok(())
        }
        ["--check", "--other-platform"] => {
            let (report, broken) = make(&files)?;
            let json_path = root.join(format!("{REPORT}.json"));
            let committed = fs::read_to_string(&json_path)
                .map_err(|e| format!("{}: {e}", json_path.display()))?;
            let committed: serde_json::Value =
                serde_json::from_str(&committed).map_err(|e| e.to_string())?;
            let made = serde_json::to_value(&report).map_err(|e| e.to_string())?;
            let mut apart = Apart::default();
            apart.walk("", &committed, &made);
            println!(
                "fit: {} numbers differ from the committed report, by at most {} flights and {:.2}%",
                apart.differing,
                apart.most_flights,
                100.0 * apart.most_share
            );
            let mut broken = broken;
            broken.extend(apart.broken);
            if !broken.is_empty() {
                return Err(format!(
                    "the fit's intervals don't come out as they must:\n{}",
                    broken.join("\n")
                ));
            }
            println!("fit: every parameter passes on sets A and B, and every probe fails");
            Ok(())
        }
        [] | ["--check"] => {
            let check = !args.is_empty();
            let (report, mut broken) = make(&files)?;
            if check {
                let (made, tracked) = fit_with_track(
                    Set::Calibration,
                    Set::Calibration.count(),
                    CALIBRATION_TRACK_FLIGHTS,
                    &files,
                )?;
                for asked in ASKED {
                    for (drift, on) in [(false, &made), (true, &tracked)] {
                        let in_force = fits::shares(asked, drift);
                        let Some(got) = fits::fitted(asked, drift, on) else {
                            broken.push(format!("{asked:?}: too few flights to fit its shares"));
                            continue;
                        };
                        if (three(got.low), three(got.high)) != (in_force.low, in_force.high) {
                            broken.push(format!(
                                "{asked:?}, track {drift}: the shares fit {} and {}; the library \
                                 has {} and {}",
                                got.low, got.high, in_force.low, in_force.high
                            ));
                        }
                    }
                }
            }
            let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
            let text = page(&report);
            let json_path = root.join(format!("{REPORT}.json"));
            let page_path = root.join(format!("{REPORT}.md"));
            if check {
                let committed = fs::read_to_string(&json_path)
                    .map_err(|e| format!("{}: {e}", json_path.display()))?;
                let committed_page = fs::read_to_string(&page_path)
                    .map_err(|e| format!("{}: {e}", page_path.display()))?;
                if committed != json || committed_page != text {
                    broken.push(format!(
                        "{REPORT}.{{json,md}} don't reproduce: run `cargo run --profile \
                         report-check -p xtask -- fit`"
                    ));
                }
            } else {
                fs::write(&json_path, json).map_err(|e| e.to_string())?;
                fs::write(&page_path, text).map_err(|e| e.to_string())?;
            }
            if !broken.is_empty() {
                return Err(format!(
                    "the fit's intervals don't come out as they must:\n{}",
                    broken.join("\n")
                ));
            }
            println!("fit: every parameter passes on sets A and B, and every probe fails");
            Ok(())
        }
        _ => Err(format!("unknown arguments\n\n{USAGE}")),
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;
    use hpr_forensics::synthetic::logger::Profile;

    fn files() -> DesignFiles {
        crate::synthetic::design_files(&crate::designs::root().unwrap()).unwrap()
    }

    /// Runs the fit's whole road on real flights: each parameter's error within a few of its
    /// standard errors, on both models, and a track that speaks to the wind where height doesn't.
    #[test]
    fn a_flight_comes_back_within_its_errors_and_a_track_speaks_to_the_wind() {
        let files = files();
        let track = fits::track_flights(Set::A, 1)[0];
        let height = (0..)
            .find(|k| {
                hpr_forensics::synthetic::draw(Set::A, *k).logger.profile == Profile::PerfectFlite
            })
            .unwrap();
        let made = fit_set(Set::A, &[track, height], &files, None).unwrap();
        let (with_track, alone) = if made[0].index == track {
            (&made[0], &made[1])
        } else {
            (&made[1], &made[0])
        };
        for fit in [with_track, alone] {
            assert!(fit.fitted);
            for (compared, asked) in fit.compared.iter().zip(ASKED) {
                assert!(compared.share().abs() < 5.0, "{asked:?}: {compared:?}");
            }
        }
        let wind = ASKED.iter().position(|a| *a == Asked::WindEast).unwrap();
        assert!(
            with_track.compared[wind].standard_error < 1.0,
            "{with_track:?}"
        );
        assert!(alone.compared[wind].standard_error > 2.0, "{alone:?}");
    }

    /// Noise past the reader leaves every parameter at about its stated range's width, and the
    /// reader turns away noise in place of a log.
    #[test]
    fn noise_tells_a_fit_nothing() {
        let files = files();
        let bins = fit_set(
            Set::A,
            &first(2),
            &files,
            Some(Noise {
                how: Noisy::Bins,
                size_m: 1_000.0,
                track_m: Some(5.0),
            }),
        )
        .unwrap();
        let log = fit_set(
            Set::A,
            &first(2),
            &files,
            Some(Noise {
                how: Noisy::Log,
                size_m: 5.0,
                track_m: None,
            }),
        )
        .unwrap();
        assert!(bins.iter().all(|f| !f.fitted));
        assert!(log.iter().all(|f| !f.fitted));
        for asked in ASKED {
            assert!(!Identification::of(asked, &bins).identified, "{asked:?}");
            assert!(!Identification::of(asked, &log).identified, "{asked:?}");
        }
    }

    /// The fit on a flight of another model comes back with the truth its noise-free fit gives.
    #[test]
    fn a_flight_of_another_model_has_a_truth_from_its_noise_free_fit() {
        let files = files();
        let made = fit_set(Set::B, &first(2), &files, None).unwrap();
        for fit in made.iter().filter(|f| f.fitted) {
            let drag = fit.compared[0];
            assert!(
                drag.truth.is_finite() && drag.share().abs() < 5.0,
                "{drag:?}"
            );
        }
    }

    /// Two platforms' reports may stand a few flights apart, no more, and nothing else may change.
    #[test]
    fn another_platform_stands_a_few_flights_off_at_most() {
        use serde_json::json;
        let apart = |a: serde_json::Value, b: serde_json::Value| {
            let mut apart = Apart::default();
            apart.walk("", &a, &b);
            apart
        };
        let within = apart(
            json!({"below": 50, "width": 0.112, "name": "drag"}),
            json!({"below": 45, "width": 0.113, "name": "drag"}),
        );
        assert!(within.broken.is_empty(), "{:?}", within.broken);
        assert_eq!((within.differing, within.most_flights), (2, 5));
        assert!(
            !apart(json!({"below": 50}), json!({"below": 44}))
                .broken
                .is_empty()
        );
        assert!(
            !apart(json!({"width": 0.1}), json!({"width": 0.2}))
                .broken
                .is_empty()
        );
        assert!(
            !apart(json!({"name": "a"}), json!({"name": "b"}))
                .broken
                .is_empty()
        );
        assert!(!apart(json!({"a": 1}), json!({"b": 1})).broken.is_empty());
        assert!(
            !apart(json!({"n": 100}), json!({"n": 104.0}))
                .broken
                .is_empty()
        );
        assert!(!apart(json!([1, 2]), json!([1])).broken.is_empty());
    }

    /// The worked example on the fit's page is this flight's.
    #[test]
    fn the_pages_worked_example_is_a_real_flights() {
        let files = files();
        let made = fit_set(Set::A, &[140], &files, None).unwrap();
        let page = std::fs::read_to_string(
            crate::designs::root()
                .unwrap()
                .join("docs/physics/flight-fit.md"),
        )
        .unwrap();
        assert!(page.contains("Calisto"));
        assert!(page.contains(&format!(
            "{:.2} m (3.2 ft), in {} simulated flights",
            made[0].rms_m.unwrap(),
            made[0].flown
        )));
        assert!(
            hpr_forensics::synthetic::designs::DESIGNS[made[0].design]
                .name
                .contains("calisto")
        );
        for (compared, asked) in made[0].compared.iter().zip(ASKED) {
            let shares = fits::shares(asked, made[0].drift);
            let interval = shares
                .interval(compared.value, compared.standard_error)
                .unwrap();
            let decimals = if matches!(asked, Asked::WindEast | Asked::WindNorth) {
                2
            } else {
                3
            };
            let row = format!(
                "| {} | {:.d$} | {:.d$} to {:.d$} | {:.d$} |",
                asked.name(),
                compared.value,
                interval.low,
                interval.high,
                compared.truth,
                d = decimals
            )
            .replace('-', "\u{2212}");
            assert!(page.contains(&row), "the page lacks {row}");
        }
    }

    #[test]
    fn temp_noise_free_on_a() {
        let files = files();
        let mut tables = BTreeMap::new();
        let mut rows = Vec::new();
        let mut flyer = hpr_forensics::synthetic::Flyer::new(files.clone());
        for index in 0..240_u64 {
            let flight = flyer.flight(Set::A, index).unwrap();
            let read = hpr_flightdata::readings::read(&flight.log);
            let Some(observed) = hpr_forensics::fit::observations(&flight.log, &read) else {
                continue;
            };
            let weighed = fits::weighed_factor(Set::A, index);
            let answer = hpr_forensics::fit::fit(
                Some(&observed),
                &fits::stated_for(Some(&observed)),
                hpr_forensics::fit::Noise::FromResiduals,
                &mut fits::Flown::new(&flight.draw, &files, weighed, &mut tables),
            );
            if !answer.fitted {
                continue;
            }
            let free = fits::noise_free(&observed, &answer, &flight, &files, &mut tables).unwrap();
            let p = free.parameters;
            println!(
                "ROW {index} {} {} {:.5} {:.5} {:.5} {:.5} {:.5} {:.4} {:.4} {:.4} {:.4} {:.4} {:.2} {:.3}",
                flight.draw.design,
                !observed.horizontal.is_empty(),
                p.drag_scale - flight.draw.drag_scale,
                p.impulse_scale - flight.draw.impulse_scale,
                answer.standard_errors[1],
                answer.parameters.impulse_scale - flight.draw.impulse_scale,
                answer.parameters.drag_scale - flight.draw.drag_scale,
                flight.draw.wind_speed_m_s,
                flight.draw.burn_time_scale,
                flight.draw.impulse_scale,
                flight.draw.drag_scale,
                p.burn_time_scale,
                p.wind_east_m_s.hypot(p.wind_north_m_s),
                free.rms_m.unwrap()
            );
            rows.push(index);
        }
    }

    /// A check counts the wind only where a track speaks to it.
    #[test]
    fn the_wind_is_counted_on_a_track_alone() {
        let files = files();
        let track = fits::track_flights(Set::A, 1)[0];
        let height = (0..)
            .find(|k| hpr_forensics::synthetic::draw(Set::A, *k).logger.profile == Profile::Rrc3)
            .unwrap();
        let made = fit_set(Set::A, &[track, height], &files, None).unwrap();
        assert_eq!(fits::check(Asked::WindEast, &made).test.counts.given, 1);
        assert_eq!(fits::check(Asked::DragScale, &made).test.counts.given, 2);
    }
}
