//! `cargo xtask fit`: the least-squares fit's 90% intervals held to ADR-257's test on the synthetic
//! flights (M7.3b, ADR-316): each parameter's three counts on sets A and B, its three probes, what
//! the log can tell of it, and every parameter on pure noise.
//!
//! The flights are flown and fitted on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a fit is the same however many came before it on its thread (the shared
//! tables change no number), so the report doesn't depend on the number of threads.
//!
//! For the same reason it doesn't depend on the number of machines. CI fits the flights in
//! [`Part`]s, each on a machine of its own, and a last job puts the parts' fits back together and
//! checks them as one run would. The fits pass from job to job in CBOR, which keeps every bit.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use hpr_flightdata::readings::interval::Shares;
use hpr_forensics::synthetic::designs::DesignFiles;
use hpr_forensics::synthetic::fits::{self, ASKED, Asked, Check, FlightFit, Identification};
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str =
    "  fit [--check]            Fit every flight of the synthetic sets A and B by least squares (
                           ADR-316) and write validation/reports/fit-intervals.{json,md}. --check
                           writes nothing and fails unless the fits reproduce the committed report
                           and the calibration set gives the library's shares. Either fails if a
                           parameter's interval fails ADR-257's test, a probe of it passes, a scale
                           isn't identified on set A, or anything is identified on pure noise.
  fit --check --other-platform
                           The same rules, on a platform other than the one that wrote the report:
                           its counts may stand a few flights off, and calibration isn't rerun.
  fit --part <k>/<n> --out <dir>
                           Fit part k of n of those flights, every nth from the kth, and write
                           the fits to <dir>; checks nothing.
  fit --check --other-platform --parts <dir>
                           The check above, on the fits of every part in <dir>. Fails unless they
                           are parts 1 to n of one split, and fit each flight once.
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

/// `set`'s first `flights` flights; the first `track_flights` of its flights with a track logger;
/// and those of the latter past the former, which are fitted on their own.
fn with_track(set: Set, flights: u64, track_flights: u64) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let track = fits::track_flights(set, track_flights);
    let extra = track.iter().copied().filter(|k| *k >= flights).collect();
    (first(flights), track, extra)
}

/// The fits of `made` and `more` whose flights are in `track`, each once.
fn tracked(track: &[u64], made: &[FlightFit], more: &[FlightFit]) -> Vec<FlightFit> {
    made.iter()
        .chain(more)
        .filter(|f| track.contains(&f.index))
        .cloned()
        .collect()
}

/// `set`'s flights and the wind's flights with a track, fitted, each once.
fn fit_with_track(
    set: Set,
    flights: u64,
    track_flights: u64,
    files: &DesignFiles,
) -> Result<(Vec<FlightFit>, Vec<FlightFit>), String> {
    let (whole, track, extra) = with_track(set, flights, track_flights);
    let made = fit_set(set, &whole, files, None)?;
    let more = fit_set(set, &extra, files, None)?;
    let tracked = tracked(&track, &made, &more);
    Ok((made, tracked))
}

/// One run of fits the report reads: flights `indices` of `set`, each fitted to its log, or to
/// `noise` in its place.
struct Run {
    set: Set,
    indices: Vec<u64>,
    noise: Option<Noise>,
}

/// The report's runs, in order: for set A and then set B, its first [`FIT_FLIGHTS`] flights, then
/// the wind's flights with a track past them; then set A's first [`NOISE_FLIGHTS`] given each of
/// [`NOISES`].
fn runs() -> Vec<Run> {
    let mut runs = Vec::new();
    for set in [Set::A, Set::B] {
        let (whole, _, extra) = with_track(set, FIT_FLIGHTS, TRACK_FLIGHTS);
        for indices in [whole, extra] {
            runs.push(Run {
                set,
                indices,
                noise: None,
            });
        }
    }
    for given in NOISES {
        runs.push(Run {
            set: Set::A,
            indices: first(NOISE_FLIGHTS),
            noise: Some(given),
        });
    }
    runs
}

/// Part `number` of `of` of the report's fits: every `of`th flight of each run, from its
/// `number`th (counted from 1). The parts of one split fit each flight once between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Part {
    number: usize,
    of: usize,
}

impl Part {
    /// Every flight, in one part.
    const WHOLE: Self = Self { number: 1, of: 1 };

    /// `k/n`, with `n` at least 1 and `k` from 1 to `n`.
    fn parse(text: &str) -> Result<Self, String> {
        let wrong = || format!("--part takes k/n, k from 1 to n, not {text}");
        let (number, of) = text.split_once('/').ok_or_else(wrong)?;
        let part = Self {
            number: number.parse().map_err(|_| wrong())?,
            of: of.parse().map_err(|_| wrong())?,
        };
        if part.number == 0 || part.number > part.of {
            return Err(wrong());
        }
        Ok(part)
    }

    /// This part's flights of a run.
    fn of_run(self, indices: &[u64]) -> Vec<u64> {
        indices
            .iter()
            .copied()
            .skip(self.number - 1)
            .step_by(self.of)
            .collect()
    }

    /// This part's file in `dir`.
    fn file(self, dir: &Path) -> PathBuf {
        dir.join(format!("fit-part-{}-of-{}.cbor", self.number, self.of))
    }
}

/// A part's fits, run by run in [`runs`]' order, as one CI job hands them to the next. CBOR keeps
/// each number's bits; JSON can't, as it has no NaN, and the misfit's cut counts NaNs.
#[derive(Debug, Serialize, Deserialize)]
struct PartFile {
    part: Part,
    fits: Vec<Vec<FlightFit>>,
}

/// `part`'s fits, run by run in [`runs`]' order.
fn fit_runs(files: &DesignFiles, part: Part) -> Result<Vec<Vec<FlightFit>>, String> {
    runs()
        .iter()
        .map(|run| fit_set(run.set, &part.of_run(&run.indices), files, run.noise))
        .collect()
}

/// Writes `part`'s `fits` to its file in `dir`, and returns the file's path.
fn write_part(dir: &Path, part: Part, fits: Vec<Vec<FlightFit>>) -> Result<PathBuf, String> {
    fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = part.file(dir);
    let mut bytes = Vec::new();
    ciborium::into_writer(&PartFile { part, fits }, &mut bytes).map_err(|e| e.to_string())?;
    fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

/// The report's fits from the part files (`*.cbor`) in `dir`, as [`fit_runs`] makes them in one
/// part. Refused unless the files are parts 1 to `n` of one split, each once, and give back each
/// run's flights once each.
fn fits_from_parts(dir: &Path) -> Result<Vec<Vec<FlightFit>>, String> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if path.extension().is_some_and(|x| x == "cbor") {
            paths.push(path);
        }
    }
    paths.sort();
    let runs = runs();
    let flights: usize = runs.iter().map(|run| run.indices.len()).sum();
    let mut fits: Vec<Vec<FlightFit>> = vec![Vec::new(); runs.len()];
    let mut seen = BTreeSet::new();
    let mut split = None;
    for path in &paths {
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let file: PartFile = ciborium::from_reader(bytes.as_slice())
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let Part { number, of } = file.part;
        if of > flights {
            return Err(format!(
                "{}: a split in {of} parts, more than the report's {flights} flights",
                path.display()
            ));
        }
        let of_all = *split.get_or_insert(of);
        if of != of_all {
            return Err(format!(
                "{}: a part of {of}, where another file's is of {of_all}",
                path.display()
            ));
        }
        if number == 0 || number > of || !seen.insert(number) {
            return Err(format!(
                "{}: part {number} of {of}, out of range or there twice",
                path.display()
            ));
        }
        if file.fits.len() != runs.len() {
            return Err(format!(
                "{}: {} runs, where the report has {}",
                path.display(),
                file.fits.len(),
                runs.len()
            ));
        }
        for (all, mine) in fits.iter_mut().zip(file.fits) {
            all.extend(mine);
        }
    }
    let Some(of) = split else {
        return Err(format!("{}: no part files (*.cbor)", dir.display()));
    };
    let missing: Vec<usize> = (1..=of).filter(|n| !seen.contains(n)).collect();
    if !missing.is_empty() {
        return Err(format!("parts {missing:?} of {of} are missing"));
    }
    for (run, all) in runs.iter().zip(&mut fits) {
        all.sort_by_key(|f| f.index);
        let mut want = run.indices.clone();
        want.sort_unstable();
        if !all.iter().map(|f| f.index).eq(want.iter().copied()) {
            return Err(format!(
                "a run of set {:?}'s {} flights came back from the parts as {} fits, not each once",
                run.set,
                want.len(),
                all.len()
            ));
        }
    }
    Ok(fits)
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
    /// Of those, the logs whose only height is a GNSS receiver's, from which the reader takes no
    /// apogee.
    said_nothing_gnss_alone: u64,
    /// Flights counted here or for the wind whose fit, or whose truth's fit, reached its step limit.
    not_converged: u64,
    /// The largest residual of any fit to a flight drawn or of the wind's, refused ones included, as a
    /// share of the spread of the heights read.
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
            said_nothing_gnss_alone: 0,
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
             counted, {} of them logs whose only height is a GNSS receiver's, from which the reader \
             takes no apogee. A wind is counted on the {} of {TRACK_FLIGHTS} flights of the logger \
             with a track, some past these, fitted with a track read. Of the flights counted here or \
             for the wind, {} reached the fit's step limit. The largest residual of any of them is {} \
             of the spread of the heights read; past {} a fit says nothing.\n\n| Parameter | Below | Inside | Above | \
             Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |\n\
             |---|---:|---:|---:|---|---|---:|---:|---|",
            set.set,
            set.flights,
            set.drawn,
            set.said_nothing,
            set.said_nothing_gnss_alone,
            set.checks
                .iter()
                .find(|c| c.asked == Asked::WindEast)
                .map_or(0, |c| c.test.counts.given),
            set.not_converged,
            set.largest_misfit,
            hpr_forensics::fit::MISFIT_SHARE,
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
    report_of(&fit_runs(files, Part::WHOLE)?)
}

/// The report from the fits of its runs, in [`runs`]' order, with what doesn't come out as it must.
fn report_of(made_runs: &[Vec<FlightFit>]) -> Result<(Report, Vec<String>), String> {
    let wrong = || {
        format!(
            "{} runs of fits, where the report reads {}",
            made_runs.len(),
            4 + NOISES.len()
        )
    };
    let mut run = made_runs.iter();
    let mut broken = Vec::new();
    let mut sets = Vec::new();
    for set in [Set::A, Set::B] {
        let (Some(made), Some(more)) = (run.next(), run.next()) else {
            return Err(wrong());
        };
        let tracked = tracked(&fits::track_flights(set, TRACK_FLIGHTS), made, more);
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
        // A flight among both the counted and the wind's is one flight.
        let stopped: std::collections::BTreeSet<u64> = counted
            .iter()
            .chain(&tracked)
            .filter(|f| f.fitted && !f.converged)
            .map(|f| f.index)
            .collect();
        report.not_converged = stopped.len() as u64;
        report.said_nothing_gnss_alone = made
            .iter()
            .filter(|f| !f.fitted && f.index < report.drawn)
            .filter(|f| {
                hpr_forensics::synthetic::draw(set, f.index).logger.profile
                    == hpr_forensics::synthetic::logger::Profile::GnssTracker
            })
            .count() as u64;
        // The pages say every log refused is one of these; a refusal of another kind is news.
        if report.said_nothing_gnss_alone != report.said_nothing {
            broken.push(format!(
                "{set:?}: {} logs had nothing to fit, {} of them a GNSS receiver's alone",
                report.said_nothing, report.said_nothing_gnss_alone
            ));
        }
        // Over every flight a fit came to, those it then refused included, so that a flight the
        // misfit's cut drops can't leave the counts unseen.
        let misfits: Vec<f64> = made
            .iter()
            .filter(|f| f.index < report.drawn)
            .chain(&tracked)
            .filter_map(|f| f.misfit_share)
            .collect();
        report.largest_misfit = misfits.iter().copied().fold(0.0, f64::max);
        let cut = misfits
            .iter()
            .filter(|share| share.is_nan() || **share > hpr_forensics::fit::MISFIT_SHARE)
            .count();
        if cut > 0 {
            broken.push(format!(
                "{set:?}: {cut} flights' fits stand past the misfit's cut, and aren't counted"
            ));
        }
        sets.push(report);
        broken.extend(problems);
    }
    let mut noise = Vec::new();
    for given in NOISES {
        let Some(made) = run.next() else {
            return Err(wrong());
        };
        let Noise {
            how,
            size_m,
            track_m,
        } = given;
        let identified: BTreeMap<Asked, Identification> = ASKED
            .iter()
            .map(|&asked| (asked, Identification::of(asked, made)))
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
                .filter(|share| share.is_finite())
                .min_by(f64::total_cmp),
            flights: made.len() as u64,
            said_nothing: made.iter().filter(|f| !f.fitted).count() as u64,
            identified,
        });
    }
    if run.next().is_some() {
        return Err(wrong());
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

/// `--check --other-platform`: `made`, the report with what doesn't come out as it must, held to
/// the committed report by [`Apart`]'s rules.
fn other_platform(root: &Path, made: (Report, Vec<String>)) -> Result<(), String> {
    let (report, mut broken) = made;
    let json_path = root.join(format!("{REPORT}.json"));
    let committed =
        fs::read_to_string(&json_path).map_err(|e| format!("{}: {e}", json_path.display()))?;
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
        ["--part", part, "--out", dir] => {
            let part = Part::parse(part)?;
            let start = Instant::now();
            let fits = fit_runs(&files, part)?;
            let count: usize = fits.iter().map(Vec::len).sum();
            let path = write_part(Path::new(dir), part, fits)?;
            println!(
                "fit: part {} of {}, {count} fits in {:.0} s, written to {}",
                part.number,
                part.of,
                start.elapsed().as_secs_f64(),
                path.display()
            );
            Ok(())
        }
        ["--check", "--other-platform"] => other_platform(&root, make(&files)?),
        ["--check", "--other-platform", "--parts", dir] => {
            other_platform(&root, report_of(&fits_from_parts(Path::new(dir))?)?)
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

    /// A made-up fit of flight `index`, for the parts' bookkeeping: nothing is flown.
    fn a_fit(index: u64) -> FlightFit {
        let compared = fits::Compared {
            value: 1.0,
            standard_error: 0.5,
            truth: 0.75,
        };
        FlightFit {
            index,
            design: 0,
            fitted: true,
            drift: false,
            converged: true,
            misfit_share: Some(0.01),
            compared: [compared; ASKED.len()],
            rms_m: Some(1.5),
            flown: 70,
        }
    }

    /// Writes part `part` of the report's runs as made-up fits, leaving flight `skip` out. Each
    /// fit's `design` is its run's place in [`runs`], so a run put back in another's place shows.
    fn write_made_up(dir: &Path, part: Part, skip: Option<u64>) {
        let fits = runs()
            .iter()
            .enumerate()
            .map(|(place, run)| {
                part.of_run(&run.indices)
                    .into_iter()
                    .filter(|index| Some(*index) != skip)
                    .map(|index| FlightFit {
                        design: place,
                        ..a_fit(index)
                    })
                    .collect()
            })
            .collect();
        write_part(dir, part, fits).unwrap();
    }

    /// A fit is the same whatever its thread fitted before it, to the bit: the flyer and the
    /// shared tables change no number. So threads, parts and machines all make the same fits.
    #[test]
    fn a_fit_is_the_same_whatever_its_thread_fitted_before() {
        let files = files();
        let in_turn = |indices: &[u64]| -> Vec<Vec<u8>> {
            let mut flyer = Flyer::new(files.clone());
            let mut tables = BTreeMap::new();
            indices
                .iter()
                .map(|&index| {
                    let flight = flyer.flight(Set::A, index).unwrap();
                    let fit =
                        fits::fit_log(&flight.log, &flight, Set::A, &files, &mut tables).unwrap();
                    assert!(fit.fitted, "flight {index}");
                    let mut bytes = Vec::new();
                    ciborium::into_writer(&fit, &mut bytes).unwrap();
                    bytes
                })
                .collect()
        };
        let track = fits::track_flights(Set::A, 2);
        let after = in_turn(&[track[1], track[0]]);
        let alone = in_turn(&[track[0]]);
        assert_eq!(after[1], alone[0]);
    }

    /// The parts of a split fit each flight of each run once between them, however many there are.
    #[test]
    fn the_parts_of_a_split_fit_each_flight_once() {
        let runs = runs();
        assert_eq!(runs.len(), 4 + NOISES.len());
        for run in &runs {
            for of in [1, 2, 3, 8, 13] {
                let mut all: Vec<u64> = (1..=of)
                    .flat_map(|number| Part { number, of }.of_run(&run.indices))
                    .collect();
                all.sort_unstable();
                let mut want = run.indices.clone();
                want.sort_unstable();
                assert_eq!(all, want, "{:?}, {of} parts", run.set);
            }
        }
        assert_eq!(Part::parse("3/8"), Ok(Part { number: 3, of: 8 }));
        for wrong in [
            "0/8",
            "9/8",
            "1/0",
            "8",
            "",
            "a/8",
            "1/8/2",
            "-1/8",
            " 1/8",
            "1/99999999999999999999",
        ] {
            assert!(Part::parse(wrong).is_err(), "{wrong}");
        }
    }

    /// A part's file gives back each number to the bit: a NaN, an infinity, a negative zero and a
    /// subnormal number among them, which JSON would lose or refuse.
    #[test]
    fn a_part_file_keeps_every_number_to_the_bit() {
        let mut fit = a_fit(7);
        fit.misfit_share = Some(f64::NAN);
        fit.rms_m = Some(-0.0);
        fit.compared[0] = fits::Compared {
            value: f64::INFINITY,
            standard_error: f64::MIN_POSITIVE / 3.0,
            truth: 0.1 + 0.2,
        };
        let dir = tempfile::tempdir().unwrap();
        let path = write_part(dir.path(), Part::WHOLE, vec![vec![fit.clone()]]).unwrap();
        let back: PartFile = ciborium::from_reader(fs::read(path).unwrap().as_slice()).unwrap();
        let got = &back.fits[0][0];
        assert!(got.misfit_share.unwrap().is_nan());
        assert_eq!(got.rms_m.unwrap().to_bits(), (-0.0_f64).to_bits());
        let bits = |c: &fits::Compared| {
            [
                c.value.to_bits(),
                c.standard_error.to_bits(),
                c.truth.to_bits(),
            ]
        };
        for (got, want) in got.compared.iter().zip(&fit.compared) {
            assert_eq!(bits(got), bits(want));
        }
        assert_eq!(back.part, Part::WHOLE);
    }

    /// Parts come back as the runs' flights in order, or not at all: a part missing, a part there
    /// twice, parts of two splits, and a flight left out are each refused.
    #[test]
    fn parts_come_back_whole_or_not_at_all() {
        let runs = runs();
        let whole = tempfile::tempdir().unwrap();
        for number in 1..=3 {
            write_made_up(whole.path(), Part { number, of: 3 }, None);
        }
        let fits = fits_from_parts(whole.path()).unwrap();
        assert_eq!(fits.len(), runs.len());
        for (place, (run, got)) in runs.iter().zip(&fits).enumerate() {
            assert!(got.iter().map(|f| f.index).eq(run.indices.iter().copied()));
            assert!(got.iter().all(|f| f.design == place), "run {place}");
        }

        let refused = |dir: &tempfile::TempDir, what: &str| {
            let error = fits_from_parts(dir.path()).unwrap_err();
            assert!(error.contains(what), "{error}");
        };
        let missing = tempfile::tempdir().unwrap();
        for number in [1, 3] {
            write_made_up(missing.path(), Part { number, of: 3 }, None);
        }
        refused(&missing, "parts [2] of 3 are missing");

        let twice = tempfile::tempdir().unwrap();
        for number in 1..=2 {
            write_made_up(twice.path(), Part { number, of: 2 }, None);
        }
        fs::copy(
            Part { number: 1, of: 2 }.file(twice.path()),
            twice.path().join("again.cbor"),
        )
        .unwrap();
        refused(&twice, "there twice");

        let past = tempfile::tempdir().unwrap();
        for number in 1..=4 {
            write_made_up(past.path(), Part { number, of: 3 }, None);
        }
        refused(&past, "out of range");

        let too_many = tempfile::tempdir().unwrap();
        write_made_up(
            too_many.path(),
            Part {
                number: 1,
                of: usize::MAX,
            },
            None,
        );
        refused(&too_many, "more than the report's");

        let runs_short = tempfile::tempdir().unwrap();
        write_part(runs_short.path(), Part::WHOLE, vec![Vec::new()]).unwrap();
        refused(&runs_short, "1 runs, where the report has");

        let two_splits = tempfile::tempdir().unwrap();
        for number in 1..=2 {
            write_made_up(two_splits.path(), Part { number, of: 2 }, None);
        }
        write_made_up(two_splits.path(), Part { number: 3, of: 3 }, None);
        refused(&two_splits, "where another file's is of");

        let short = tempfile::tempdir().unwrap();
        for number in 1..=3 {
            write_made_up(short.path(), Part { number, of: 3 }, Some(5));
        }
        refused(&short, "not each once");

        refused(&tempfile::tempdir().unwrap(), "no part files");

        let broken = tempfile::tempdir().unwrap();
        write_made_up(broken.path(), Part::WHOLE, None);
        let path = Part::WHOLE.file(broken.path());
        let bytes = fs::read(&path).unwrap();
        fs::write(&path, &bytes[..bytes.len() / 2]).unwrap();
        refused(&broken, "fit-part-1-of-1.cbor");
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
