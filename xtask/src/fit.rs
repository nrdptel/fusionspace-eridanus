//! `cargo xtask fit`: the least-squares fit's 90% intervals held to ADR-257's test on the synthetic
//! flights (M7.3b, ADR-316): each parameter's three counts on sets A and B, its three probes, what
//! the log can tell of it, and every parameter on pure noise.
//!
//! The flights are flown and fitted on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a fit is the same however many came before it on its thread (the shared
//! tables change no number), so the report doesn't depend on the number of threads.
//!
//! For the same reason, on one platform, it doesn't depend on the number of machines either. CI
//! fits the flights in [`Part`]s, each on a machine of its own, and a last job puts the parts' fits
//! back together and checks them as one run would. The fits pass from job to job in CBOR, which
//! keeps every bit.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use hpr_flightdata::readings::interval::{Interval, Shares};
use hpr_forensics::synthetic::designs::DesignFiles;
use hpr_forensics::synthetic::fits::{self, ASKED, Asked, Check, FlightFit, Identification};
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str =
    "  fit [--check]            Fit every flight of the synthetic sets A and B by least squares (
                           ADR-316) and the calibration set, and write
                           validation/reports/fit-intervals.{json,md} and the slice,
                           fit-intervals.slice.json. --check writes nothing and fails unless the
                           fits reproduce all three. Either fails if a parameter's interval fails
                           ADR-257's test, a probe of it passes, a scale isn't identified on set A,
                           anything is identified on pure noise, or the calibration set doesn't
                           give the library's shares.
  fit --check --other-platform
                           The same rules, on a platform other than the one that wrote the report:
                           its counts may stand a few flights off, its figures and the shares a
                           few percent, and the slice is held in its words, its numbers aside.
  fit --slice [--other-platform [--part <k>/<n>]]
                           Fit only the slice, every 21st flight of each run, and hold it to the
                           committed slice, byte for byte or (--other-platform) in its words, and
                           to the report's rules for one fit; writes nothing. --part fits part k
                           of n of the slice's flights, every nth from the kth, held in words.
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
/// [`NOISES`]; then the calibration set's flights and its flights with a track past them, which
/// the shares in force are held to.
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
    let (whole, _, extra) = with_track(
        Set::Calibration,
        Set::Calibration.count(),
        CALIBRATION_TRACK_FLIGHTS,
    );
    for indices in [whole, extra] {
        runs.push(Run {
            set: Set::Calibration,
            indices,
            noise: None,
        });
    }
    runs
}

/// The runs past the report's: the calibration set's two.
const CALIBRATION_RUNS: usize = 2;

impl Run {
    /// Its name in the slice: its set's, and the noise given in place of its logs.
    fn name(&self) -> String {
        match self.noise {
            None => self.set.name().to_owned(),
            Some(Noise {
                how,
                size_m,
                track_m,
            }) => format!(
                "{} given noise of ±{size_m} m{} {}",
                self.set.name(),
                track_m.map_or(String::new(), |track_m| format!(
                    " and a track of ±{track_m} m"
                )),
                match how {
                    Noisy::Log => "in place of its log",
                    Noisy::Bins => "past the reader",
                }
            ),
        }
    }
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

    /// Part `self` of `outer`'s flights, as one part of a run: every `self.of`th of `outer`'s,
    /// from its `self.number`th, is every `outer.of * self.of`th flight of the run, from its
    /// `outer.number + (self.number - 1) * outer.of`th.
    fn within(self, outer: Self) -> Self {
        Self {
            number: outer.number + (self.number - 1) * outer.of,
            of: outer.of * self.of,
        }
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

/// What doesn't come out as it must in `set`'s `checks`: a parameter's interval failing ADR-257's
/// test, a probe of it passing, and on set A a scale not identified. It reads each test's stored
/// bands, worked out from its counts when the report was made; `fit --slice` reads a committed
/// report's so, and doesn't work them out again.
fn check_problems(set: Set, checks: &[Check]) -> Vec<String> {
    let mut broken = Vec::new();
    for check in checks {
        let c = check.test.counts;
        if !check.test.passes() {
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
            let c = test.counts;
            if test.passes() {
                broken.push(format!(
                    "{set:?}, {}: the probe {} passes, {} below, {} inside, {} above of {}",
                    check.asked.name(),
                    probe.name(),
                    c.below,
                    c.inside,
                    c.above,
                    c.given
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
    broken
}

/// What doesn't come out as it must in a committed `report`, read from its stored bands, counts and
/// shares without a fit:
/// each set's [`check_problems`], too few flights counted, a log with nothing to fit that isn't a
/// GNSS receiver's alone, a residual past the misfit's cut, anything identified on pure noise, and
/// shares other than the library's.
fn report_problems(report: &Report) -> Vec<String> {
    let mut broken = Vec::new();
    for set in &report.sets {
        broken.extend(check_problems(set.set, &set.checks));
        if set.flights < COUNTED_FLIGHTS as u64 {
            broken.push(format!(
                "{:?}: {} flights counted, under {COUNTED_FLIGHTS}",
                set.set, set.flights
            ));
        }
        if set.said_nothing_gnss_alone != set.said_nothing {
            broken.push(format!(
                "{:?}: {} logs had nothing to fit, {} of them a GNSS receiver's alone",
                set.set, set.said_nothing, set.said_nothing_gnss_alone
            ));
        }
        if set.largest_misfit.is_nan() || set.largest_misfit > hpr_forensics::fit::MISFIT_SHARE {
            broken.push(format!(
                "{:?}: a fit stands past the misfit's cut, {} of the spread",
                set.set, set.largest_misfit
            ));
        }
    }
    for noise in &report.noise {
        for (asked, id) in &noise.identified {
            if id.identified {
                broken.push(format!(
                    "{}: identified on noise of ±{} m",
                    asked.name(),
                    noise.size_m
                ));
            }
        }
    }
    for asked in ASKED {
        let library = [fits::shares(asked, false), fits::shares(asked, true)];
        if report.shares.get(&asked) != Some(&library) {
            broken.push(format!(
                "{asked:?}: the report's shares aren't the library's, {library:?}"
            ));
        }
    }
    broken
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
    broken.extend(check_problems(set, &checks));
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

/// The report from the fits of its runs, in [`runs`]' order, with what doesn't come out as it must.
fn report_of(made_runs: &[Vec<FlightFit>]) -> Result<(Report, Vec<String>), String> {
    let wrong = || {
        format!(
            "{} runs of fits, where the report reads {}",
            made_runs.len(),
            4 + NOISES.len() + CALIBRATION_RUNS
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
    // The calibration set's runs, which [`shares_problems`] reads.
    if run.count() != CALIBRATION_RUNS {
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

/// The fit's slice (M0.16a2, ADR-149 amended): part 1 of 21 of every run, its 1st, 22nd, 43rd
/// fit and on, as the synthetic flights and two logs take every 21st flight. A fit is the same
/// whatever its thread or part fitted before it (`a_fit_is_the_same_whatever_its_thread_fitted_
/// before`, one of set A's flights fitted alone and after another), so the slice's fits are the
/// whole run's at the same places: the slice a whole run writes, `fit --slice` makes again byte
/// for byte.
const SLICE: Part = Part { number: 1, of: 21 };

/// The slice's `about` line.
const SLICE_ABOUT: &str = "The fit's slice (M0.16a2, ADR-149 amended): every 21st fit of each run, \
                           from the first, what it came to and each parameter's interval, written \
                           by `cargo run --profile report-check -p xtask -- fit`; `fit --slice` \
                           holds it.";

/// Where `truth` falls against `interval`, as the report counts it: a truth that isn't a number
/// misses on both sides, and a fit that states no interval is counted as missing on both.
fn side(interval: Option<Interval>, truth: f64) -> &'static str {
    let Some(interval) = interval else {
        return "no interval";
    };
    let below = truth.is_nan() || truth < interval.low;
    let above = truth.is_nan() || truth > interval.high;
    match (below, above) {
        (true, true) => "both",
        (true, false) => "below",
        (false, true) => "above",
        (false, false) => "inside",
    }
}

/// One fit's lines in the slice: what the fit came to, then each parameter's value, standard
/// error, truth and error over its standard error, with where its truth fell against the interval
/// the report counts, or "not counted" where the report counts none (a log with nothing to fit, or
/// a wind without a track). A fit of the calibration set, `calibration`, has neither: its errors
/// are what the shares are fitted on, so some sit on an interval's end by construction, where a
/// side would turn on a platform's last digits and say nothing the report counts.
fn slice_lines(fit: &FlightFit, calibration: bool) -> Vec<String> {
    use crate::slice::{figures, share};
    let design = hpr_forensics::synthetic::designs::DESIGNS
        .get(fit.design)
        .map_or("an unknown design", |design| design.name);
    let mut made = vec![format!(
        "fit: {}, {}, {}, {}, {}, {} flown, {design}",
        if fit.fitted {
            "fitted"
        } else {
            "nothing to fit"
        },
        if fit.drift {
            "with a track"
        } else {
            "height alone"
        },
        if fit.converged {
            "converged"
        } else {
            "at its step limit"
        },
        fit.misfit_share
            .map_or("no residual".to_owned(), |misfit| format!(
                "residual {} of the spread",
                share(misfit)
            )),
        fit.rms_m
            .map_or("no residual RMS".to_owned(), |rms_m| format!(
                "residual RMS {} m",
                figures(rms_m)
            )),
        fit.flown,
    )];
    for (&asked, c) in ASKED.iter().zip(&fit.compared) {
        let counted = fit.fitted && (fit.drift || asked.stated_on_height_alone());
        let outcome = if calibration {
            String::new()
        } else if counted {
            // The shares the report's counts use on a log with something to fit.
            let interval = fits::shares(asked, fit.drift).interval(c.value, c.standard_error);
            format!(", {}", side(interval, c.truth))
        } else {
            ", not counted".to_owned()
        };
        made.push(format!(
            "{}: {} ± {}, truth {}, share {}{outcome}",
            asked.name(),
            figures(c.value),
            figures(c.standard_error),
            figures(c.truth),
            share(c.share()),
        ));
    }
    made
}

/// What breaks the rules the report holds each of its fits to ([`report_of`]), for one fit of
/// sets A and B: a fit among the flights drawn, or one of the wind's, whose residual stands past
/// the misfit's cut or isn't a number; and a log among the flights drawn with nothing to fit that
/// isn't a GNSS receiver's alone. `drawn` is the set's flights drawn, `track` the wind's flights.
/// The runs of noise and the calibration set have no rule of their own for one fit.
fn slice_rules(set: Set, fit: &FlightFit, drawn: u64, track: &[u64]) -> Vec<String> {
    let mut broken = Vec::new();
    let among_drawn = fit.index < drawn;
    if (among_drawn || track.contains(&fit.index))
        && fit
            .misfit_share
            .is_some_and(|share| share.is_nan() || share > hpr_forensics::fit::MISFIT_SHARE)
    {
        broken.push(format!(
            "{set:?} flight {}: its fit stands past the misfit's cut, {:?} of the spread",
            fit.index, fit.misfit_share
        ));
    }
    if among_drawn
        && !fit.fitted
        && hpr_forensics::synthetic::draw(set, fit.index)
            .logger
            .profile
            != hpr_forensics::synthetic::logger::Profile::GnssTracker
    {
        broken.push(format!(
            "{set:?} flight {}: had nothing to fit, and its log isn't a GNSS receiver's alone",
            fit.index
        ));
    }
    broken
}

/// The slice of `made_runs`, the fits of [`runs`] in order, the whole run's or [`SLICE`]'s part:
/// each run's fits at the slice's places, with what breaks [`slice_rules`]. `drawn` is sets A's and
/// B's flights drawn, as a report gives them.
fn slice_of(
    made_runs: &[Vec<FlightFit>],
    drawn: &[(Set, u64)],
) -> Result<(crate::slice::Slice, Vec<String>), String> {
    slice_part_of(made_runs, drawn, Part::WHOLE)
}

/// [`slice_of`] for `part` of the slice: each run's fits at that part's places of the slice's.
fn slice_part_of(
    made_runs: &[Vec<FlightFit>],
    drawn: &[(Set, u64)],
    part: Part,
) -> Result<(crate::slice::Slice, Vec<String>), String> {
    let runs = runs();
    if made_runs.len() != runs.len() {
        return Err(format!(
            "{} runs of fits, where the slice reads {}",
            made_runs.len(),
            runs.len()
        ));
    }
    let mut flights = Vec::new();
    let mut broken = Vec::new();
    for (run, made) in runs.iter().zip(made_runs) {
        let places: BTreeSet<u64> = part
            .of_run(&SLICE.of_run(&run.indices))
            .into_iter()
            .collect();
        let in_slice: Vec<&FlightFit> = made.iter().filter(|f| places.contains(&f.index)).collect();
        if in_slice.len() != places.len() {
            return Err(format!(
                "{}: {} of the slice's {} flights fitted",
                run.name(),
                in_slice.len(),
                places.len()
            ));
        }
        let ruled = drawn
            .iter()
            .find(|(set, _)| *set == run.set)
            .filter(|_| run.noise.is_none())
            .map(|&(set, drawn)| (drawn, fits::track_flights(set, TRACK_FLIGHTS)));
        for fit in in_slice {
            if let Some((drawn, track)) = &ruled {
                broken.extend(slice_rules(run.set, fit, *drawn, track));
            }
            flights.push(crate::slice::Flight {
                set: run.name(),
                index: fit.index,
                made: slice_lines(fit, run.set == Set::Calibration),
            });
        }
    }
    Ok((
        crate::slice::Slice {
            about: SLICE_ABOUT.to_owned(),
            every: SLICE.of as u64,
            flights,
        },
        broken,
    ))
}

/// The set and index of each flight at `part`'s places of the slice, in every run.
fn part_places(part: Part) -> BTreeSet<(String, u64)> {
    runs()
        .iter()
        .flat_map(|run| {
            part.of_run(&SLICE.of_run(&run.indices))
                .into_iter()
                .map(move |index| (run.name(), index))
        })
        .collect()
}

/// Sets A's and B's flights drawn, as `report` gives them.
fn drawn_of(report: &Report) -> Vec<(Set, u64)> {
    report.sets.iter().map(|s| (s.set, s.drawn)).collect()
}

/// What separates the shares the calibration set's fits, the last of `made_runs`, give from the
/// library's: any difference at their three figures, `exact`, on the platform that wrote them;
/// elsewhere one past [`Apart`]'s rule for a figure, as the same seeds fly with another math
/// library.
fn shares_problems(made_runs: &[Vec<FlightFit>], exact: bool) -> Vec<String> {
    let [.., made, more] = made_runs else {
        return vec!["no calibration runs among the fits".to_owned()];
    };
    let tracked = tracked(
        &fits::track_flights(Set::Calibration, CALIBRATION_TRACK_FLIGHTS),
        made,
        more,
    );
    let mut broken = Vec::new();
    let mut most = 0.0_f64;
    for asked in ASKED {
        for (drift, on) in [(false, made.as_slice()), (true, tracked.as_slice())] {
            let in_force = fits::shares(asked, drift);
            let Some(got) = fits::fitted(asked, drift, on) else {
                broken.push(format!(
                    "{asked:?}, track {drift}: too few flights to fit its shares, or one that isn't \
                     a number"
                ));
                continue;
            };
            for (a, b) in [(got.low, in_force.low), (got.high, in_force.high)] {
                most = most.max((three(a) - b).abs());
            }
            if !shares_hold(got, in_force, exact) {
                broken.push(format!(
                    "{asked:?}, track {drift}: the shares fit {} and {}; the library has {} and {}",
                    got.low, got.high, in_force.low, in_force.high
                ));
            }
        }
    }
    println!(
        "fit: the calibration set's shares stand at most {most:.3} from the library's, at three \
         figures"
    );
    broken
}

/// Whether the shares `got` hold the library's `in_force`: at their three figures, `exact`; else
/// each within [`Apart`]'s rule for a figure.
fn shares_hold(got: Shares, in_force: Shares, exact: bool) -> bool {
    let (low, high) = (three(got.low), three(got.high));
    if exact {
        return (low, high) == (in_force.low, in_force.high);
    }
    let near = |a: f64, b: f64| {
        let apart = (a - b).abs();
        apart <= OTHER_PLATFORM_ABSOLUTE || apart / a.abs().max(b.abs()) <= OTHER_PLATFORM_SHARE
    };
    near(low, in_force.low) && near(high, in_force.high)
}

/// `--check --other-platform`: the report `made_runs` make, held to the committed report by
/// [`Apart`]'s rules, the shares by its rule for a figure, and the slice in its words.
fn other_platform(root: &Path, made_runs: &[Vec<FlightFit>]) -> Result<(), String> {
    let (report, mut broken) = report_of(made_runs)?;
    broken.extend(shares_problems(made_runs, false));
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
    let (slice, _) = slice_of(made_runs, &drawn_of(&report))?;
    broken.extend(crate::slice::compare(
        &root.join(format!("{REPORT}.slice.json")),
        &slice,
        crate::slice::Held::Words,
    ));
    if !broken.is_empty() {
        return Err(format!(
            "the fit's intervals don't come out as they must:\n{}",
            broken.join("\n")
        ));
    }
    println!(
        "fit: every parameter passes on sets A and B, and every probe fails; the calibration set \
         gives the library's shares, and the slice's {} fits hold {REPORT}.slice.json {}",
        slice.flights.len(),
        crate::slice::held_text(crate::slice::Held::Words)
    );
    Ok(())
}

/// `fit --slice`: the slice's fits made again, `part` of them, held to the committed slice as
/// `held` says and to the report's rules for one fit; the committed report read, and its page
/// rendered from it. A part less than the whole is held in words to the committed slice's flights
/// at its places (CI splits a slow platform's slice so; the gate fits it whole, byte for byte).
fn check_slice(
    root: &Path,
    files: &DesignFiles,
    held: crate::slice::Held,
    part: Part,
) -> Result<(), String> {
    let json_path = root.join(format!("{REPORT}.json"));
    let page_path = root.join(format!("{REPORT}.md"));
    let committed =
        fs::read_to_string(&json_path).map_err(|e| format!("{}: {e}", json_path.display()))?;
    let report: Report =
        serde_json::from_str(&committed).map_err(|e| format!("{}: {e}", json_path.display()))?;
    let mut problems = Vec::new();
    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    if committed.replace("\r\n", "\n") != json {
        problems.push(format!(
            "{} doesn't read and write back unchanged",
            json_path.display()
        ));
    }
    match fs::read_to_string(&page_path) {
        Ok(text) if text.replace("\r\n", "\n") == page(&report) => {}
        Ok(_) => problems.push(format!(
            "{} isn't what {} renders",
            page_path.display(),
            json_path.display()
        )),
        Err(e) => problems.push(format!("{}: {e}", page_path.display())),
    }
    let start = Instant::now();
    let made = fit_runs(files, part.within(SLICE))?;
    let (slice, mut broken) = slice_part_of(&made, &drawn_of(&report), part)?;
    broken.extend(report_problems(&report));
    let slice_path = root.join(format!("{REPORT}.slice.json"));
    problems.extend(if part == Part::WHOLE {
        crate::slice::compare(&slice_path, &slice, held)
    } else {
        crate::slice::compare_part(&slice_path, &slice, &part_places(part))
    });
    let mut text = Vec::new();
    if !broken.is_empty() {
        text.push(format!(
            "the fit's slice or its committed report breaks the report's rules; writing them again \
             won't mend it:\n{}",
            broken.join("\n")
        ));
    }
    if !problems.is_empty() {
        text.push(format!(
            "the fit's slice doesn't hold; run `cargo run --profile report-check -p xtask -- fit` \
             and commit what it writes if the change is meant:\n{}",
            problems.join("\n")
        ));
    }
    if !text.is_empty() {
        return Err(text.join("\n"));
    }
    let which = if part == Part::WHOLE {
        String::new()
    } else {
        format!(" (part {} of {})", part.number, part.of)
    };
    println!(
        "fit: the slice's {} fits{which}, made in {:.0} s, reproduce {REPORT}.slice.json {} and \
         keep the report's rules for one fit; {REPORT}.json reads, renders {REPORT}.md, keeps the \
         report's rules by its counts and holds the library's shares",
        slice.flights.len(),
        start.elapsed().as_secs_f64(),
        crate::slice::held_text(held)
    );
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
        ["--slice"] => check_slice(&root, &files, crate::slice::Held::Exact, Part::WHOLE),
        ["--slice", "--other-platform"] => {
            check_slice(&root, &files, crate::slice::Held::Words, Part::WHOLE)
        }
        ["--slice", "--other-platform", "--part", part] => {
            check_slice(&root, &files, crate::slice::Held::Words, Part::parse(part)?)
        }
        ["--check", "--other-platform"] => other_platform(&root, &fit_runs(&files, Part::WHOLE)?),
        ["--check", "--other-platform", "--parts", dir] => {
            other_platform(&root, &fits_from_parts(Path::new(dir))?)
        }
        [] | ["--check"] => {
            let check = !args.is_empty();
            let made_runs = fit_runs(&files, Part::WHOLE)?;
            let (report, mut broken) = report_of(&made_runs)?;
            broken.extend(shares_problems(&made_runs, true));
            let (slice, _) = slice_of(&made_runs, &drawn_of(&report))?;
            let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
            let text = page(&report);
            let json_path = root.join(format!("{REPORT}.json"));
            let page_path = root.join(format!("{REPORT}.md"));
            let slice_path = root.join(format!("{REPORT}.slice.json"));
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
                // This is the platform and build that write the report, so the slice is held
                // byte for byte.
                broken.extend(crate::slice::compare(
                    &slice_path,
                    &slice,
                    crate::slice::Held::Exact,
                ));
            } else {
                fs::write(&json_path, json).map_err(|e| e.to_string())?;
                fs::write(&page_path, text).map_err(|e| e.to_string())?;
                fs::write(&slice_path, crate::slice::json(&slice)?).map_err(|e| e.to_string())?;
            }
            if !broken.is_empty() {
                return Err(format!(
                    "the fit's intervals don't come out as they must:\n{}",
                    broken.join("\n")
                ));
            }
            println!(
                "fit: every parameter passes on sets A and B, and every probe fails; the \
                 calibration set gives the library's shares; the slice's {} fits {}",
                slice.flights.len(),
                if check {
                    "reproduce it byte for byte"
                } else {
                    "written"
                }
            );
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
    use hpr_forensics::synthetic::intervals::Test;
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
    /// shared tables change no number. So, on one platform, dividing the flights among threads,
    /// parts or machines changes no fit.
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
        assert_eq!(runs.len(), 4 + NOISES.len() + CALIBRATION_RUNS);
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
        assert_eq!(got.misfit_share.unwrap().to_bits(), f64::NAN.to_bits());
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

    /// The slice is part 1 of 21 of every run, 444 of the 9,192 fits: 62 of each test set's first
    /// 1,300 flights and 61 of the wind's flights past them, 10 of each run of noise, 96 of the
    /// calibration set's 2,000 (both of its models, half and half) and 32 of its wind's flights
    /// past them; each named once.
    #[test]
    fn the_slice_takes_every_21st_fit_of_each_run() {
        assert_eq!(SLICE, Part { number: 1, of: 21 });
        let runs = runs();
        let counts: Vec<usize> = runs
            .iter()
            .map(|run| SLICE.of_run(&run.indices).len())
            .collect();
        let fits: Vec<usize> = runs.iter().map(|run| run.indices.len()).collect();
        assert_eq!(counts[..4], [62, 61, 62, 61]);
        assert!(
            counts[4..4 + NOISES.len()].iter().all(|&n| n == 10),
            "{counts:?}"
        );
        assert_eq!(counts[4 + NOISES.len()..], [96, 32]);
        assert_eq!(counts.iter().sum::<usize>(), 444);
        assert_eq!(fits[..4], [1_300, 1_262, 1_300, 1_276]);
        assert_eq!(fits[4 + NOISES.len()..], [2_000, 654]);
        assert_eq!(fits.iter().sum::<usize>(), 9_192);
        let calibration = SLICE.of_run(&runs[4 + NOISES.len()].indices);
        let odd = calibration.iter().filter(|index| *index % 2 == 1).count();
        assert_eq!(odd * 2, calibration.len());
        let mut names = BTreeSet::new();
        for run in &runs {
            for index in SLICE.of_run(&run.indices) {
                assert!(names.insert((run.name(), index)), "{} {index}", run.name());
            }
        }
    }

    /// Made-up fits of the slice's places in every run, as [`fit_runs`] gives them.
    fn made_up_slice_runs() -> Vec<Vec<FlightFit>> {
        runs()
            .iter()
            .map(|run| SLICE.of_run(&run.indices).into_iter().map(a_fit).collect())
            .collect()
    }

    /// One fit changed writes other lines, in its numbers or its words, and a slice holding it
    /// fails, naming that flight alone; made again unchanged, it writes the same lines.
    #[test]
    fn one_changed_fit_writes_other_lines_and_fails_the_slice() {
        let fit = a_fit(21);
        let lines = slice_lines(&fit, false);
        assert_eq!(lines.len(), 1 + ASKED.len(), "{lines:?}");
        assert_eq!(slice_lines(&fit.clone(), false), lines);
        assert!(lines[1].ends_with(", inside"), "{lines:?}");
        let words = |lines: &[String]| -> Vec<String> {
            lines.iter().map(|l| crate::slice::words(l)).collect()
        };
        // A value moved a little: a number moves, no word.
        let mut nudged = fit.clone();
        nudged.compared[0].value += 0.01;
        let moved = slice_lines(&nudged, false);
        assert_ne!(moved[1], lines[1]);
        assert_eq!(words(&moved), words(&lines));
        // Its truth past the interval, a fit that stopped at its step limit, a log with nothing to
        // fit: each a word.
        let mut past = fit.clone();
        past.compared[0].truth = 10.0;
        assert!(slice_lines(&past, false)[1].ends_with(", above"));
        let mut stopped = fit.clone();
        stopped.converged = false;
        assert_ne!(words(&slice_lines(&stopped, false))[0], words(&lines)[0]);
        let mut nothing = fit.clone();
        nothing.fitted = false;
        assert!(
            slice_lines(&nothing, false)[1..]
                .iter()
                .all(|l| l.ends_with(", not counted"))
        );
        // The wind is counted only with a track.
        assert!(lines[3].ends_with(", not counted"), "{lines:?}");
        let mut tracked = fit.clone();
        tracked.drift = true;
        assert!(!slice_lines(&tracked, false)[3].ends_with(", not counted"));
        // A calibration fit's lines carry its numbers alone: its errors make the shares.
        let calibration = slice_lines(&fit, true);
        assert_eq!(calibration[0], lines[0]);
        for (line, sided) in calibration[1..].iter().zip(&lines[1..]) {
            assert!(sided.starts_with(line.as_str()), "{line} / {sided}");
            assert!(
                line.ends_with(&format!("share {}", crate::slice::share(0.5))),
                "{line}"
            );
        }

        let drawn = [(Set::A, 1_219), (Set::B, 1_219)];
        let committed = made_up_slice_runs();
        let (slice, broken) = slice_of(&committed, &drawn).unwrap();
        assert_eq!(broken, Vec::<String>::new());
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fit-intervals.slice.json");
        fs::write(&path, crate::slice::json(&slice).unwrap()).unwrap();
        for held in [crate::slice::Held::Exact, crate::slice::Held::Words] {
            let (again, _) = slice_of(&made_up_slice_runs(), &drawn).unwrap();
            assert_eq!(crate::slice::compare(&path, &again, held), None);
        }
        let mut changed = committed.clone();
        changed[2][3].compared[1].truth = -10.0;
        let (made, _) = slice_of(&changed, &drawn).unwrap();
        for held in [crate::slice::Held::Exact, crate::slice::Held::Words] {
            let problem = crate::slice::compare(&path, &made, held).unwrap();
            assert!(problem.contains("(1 differences)"), "{problem}");
            assert!(problem.contains("b flight 63 differs"), "{problem}");
        }
        // A fit missing from a run is refused, not passed over.
        let mut short = committed;
        short[0].pop();
        assert!(
            slice_of(&short, &drawn)
                .unwrap_err()
                .contains("61 of the slice's 62")
        );
    }

    /// The slice's parts (a slow platform's split in CI, M0.16a3): part k of n of the slice is the
    /// run's fits at those places of the slice's, the parts fit each of the slice's fits once, and
    /// each holds the committed slice's fits at its places in words, failing on one changed fit
    /// and on one fitted off its places.
    #[test]
    fn the_slices_parts_fit_it_once_and_each_holds_its_fits() {
        for run in runs() {
            for of in 1..=4 {
                for number in 1..=of {
                    let part = Part { number, of };
                    assert_eq!(
                        part.within(SLICE).of_run(&run.indices),
                        part.of_run(&SLICE.of_run(&run.indices)),
                        "{} part {number}/{of}",
                        run.name()
                    );
                }
            }
        }
        let drawn = [(Set::A, 1_219), (Set::B, 1_219)];
        let whole = made_up_slice_runs();
        let (slice, _) = slice_of(&whole, &drawn).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fit-intervals.slice.json");
        fs::write(&path, crate::slice::json(&slice).unwrap()).unwrap();
        // A part's fits as `fit_runs` gives them: the whole's at the part's places.
        let fits_of = |part: Part, runs_made: &[Vec<FlightFit>]| -> Vec<Vec<FlightFit>> {
            runs()
                .iter()
                .zip(runs_made)
                .map(|(run, made)| {
                    let at: BTreeSet<u64> = part
                        .within(SLICE)
                        .of_run(&run.indices)
                        .into_iter()
                        .collect();
                    made.iter()
                        .filter(|f| at.contains(&f.index))
                        .cloned()
                        .collect()
                })
                .collect()
        };
        let mut seen = Vec::new();
        for number in 1..=2 {
            let part = Part { number, of: 2 };
            let (made, broken) = slice_part_of(&fits_of(part, &whole), &drawn, part).unwrap();
            assert_eq!(broken, Vec::<String>::new());
            assert_eq!(
                crate::slice::compare_part(&path, &made, &part_places(part)),
                None
            );
            seen.extend(made.flights.iter().map(|f| (f.set.clone(), f.index)));
        }
        let mut all: Vec<_> = slice
            .flights
            .iter()
            .map(|f| (f.set.clone(), f.index))
            .collect();
        all.sort();
        seen.sort();
        assert_eq!(seen, all);
        // Set b's fit 63, the slice's fourth place in its run, is part 2's.
        let part = Part { number: 2, of: 2 };
        let mut changed = whole.clone();
        changed[2][3].compared[1].truth = -10.0;
        let (made, _) = slice_part_of(&fits_of(part, &changed), &drawn, part).unwrap();
        let problem = crate::slice::compare_part(&path, &made, &part_places(part)).unwrap();
        assert!(problem.contains("(1 differences)"), "{problem}");
        assert!(problem.contains("b flight 63 differs"), "{problem}");
        // Part 1's fits held to part 2's places: each is off them.
        let first = Part { number: 1, of: 2 };
        let (made, _) = slice_part_of(&fits_of(first, &whole), &drawn, first).unwrap();
        let problem = crate::slice::compare_part(&path, &made, &part_places(part)).unwrap();
        assert!(
            problem.contains("flight 0: made, but not in this part"),
            "{problem}"
        );
        // A part missing one of its fits is refused.
        let mut short = fits_of(part, &whole);
        short[0].pop();
        assert!(slice_part_of(&short, &drawn, part).is_err());
    }

    /// Each rule the report holds a fit to names the fit that breaks it, among the flights drawn
    /// or the wind's, and only there.
    #[test]
    fn the_rules_for_one_fit_name_it() {
        let track = fits::track_flights(Set::A, 3);
        let mut misfit = a_fit(5);
        misfit.misfit_share = Some(0.2);
        let broken = slice_rules(Set::A, &misfit, 1_000, &track);
        assert!(broken[0].contains("A flight 5: its fit stands past the misfit's cut"));
        misfit.misfit_share = Some(f64::NAN);
        assert_eq!(slice_rules(Set::A, &misfit, 1_000, &track).len(), 1);
        misfit.index = 1_100;
        assert!(slice_rules(Set::A, &misfit, 1_000, &track).is_empty());
        misfit.index = track[2];
        assert_eq!(slice_rules(Set::A, &misfit, 0, &track).len(), 1);
        let logger = |index: u64| hpr_forensics::synthetic::draw(Set::A, index).logger.profile;
        let gnss = (0..).find(|&k| logger(k) == Profile::GnssTracker).unwrap();
        let other = (0..).find(|&k| logger(k) != Profile::GnssTracker).unwrap();
        for (index, rules) in [(gnss, 0), (other, 1)] {
            let mut nothing = a_fit(index);
            nothing.fitted = false;
            let broken = slice_rules(Set::A, &nothing, 1_000, &[]);
            assert_eq!(broken.len(), rules, "{index}: {broken:?}");
            assert!(slice_rules(Set::A, &nothing, index, &[]).is_empty());
        }
        // The slice holds its fits of sets A and B to the rules, and none of the others.
        let drawn = [(Set::A, 1_219), (Set::B, 1_219)];
        let mut made = made_up_slice_runs();
        for run in [0, 4, 4 + NOISES.len()] {
            made[run][1].misfit_share = Some(0.2);
        }
        let (_, broken) = slice_of(&made, &drawn).unwrap();
        assert_eq!(broken.len(), 1, "{broken:?}");
        assert!(broken[0].starts_with("A flight 21: "), "{broken:?}");
    }

    /// The shares hold the library's at their three figures where they were written, and within a
    /// few percent elsewhere; NaN holds nowhere.
    #[test]
    fn the_shares_hold_exactly_here_and_nearly_elsewhere() {
        let library = Shares {
            low: -1.27,
            high: 1.43,
        };
        let at = |low, high| Shares { low, high };
        assert!(shares_hold(at(-1.2704, 1.4296), library, true));
        assert!(!shares_hold(at(-1.276, 1.43), library, true));
        assert!(shares_hold(at(-1.31, 1.40), library, false));
        assert!(!shares_hold(at(-1.27, 1.52), library, false));
        assert!(!shares_hold(at(f64::NAN, 1.43), library, false));
        assert!(!shares_hold(at(f64::NAN, 1.43), library, true));
        // Near zero the absolute 0.002 holds where 5% wouldn't.
        let small = Shares {
            low: -0.01,
            high: 1.43,
        };
        assert!(shares_hold(at(-0.0115, 1.43), small, false));
        assert!(!shares_hold(at(-0.0125, 1.43), small, false));
    }

    /// The committed report keeps its own rules, read from its stored bands, counts and shares, and
    /// fails them when a test (counts and bands), a noise's identification or a share is written
    /// otherwise.
    #[test]
    fn the_committed_report_keeps_its_rules_by_its_counts() {
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(format!("{REPORT}.json"))).unwrap();
        let report: Report = serde_json::from_str(&text).unwrap();
        assert_eq!(report_problems(&report), Vec::<String>::new());
        let fails = |change: &dyn Fn(&mut Report), what: &str| {
            let mut changed = report.clone();
            change(&mut changed);
            let broken = report_problems(&changed);
            assert!(
                broken.iter().any(|b| b.contains(what)),
                "{what}: {broken:?}"
            );
        };
        fails(
            &|r| {
                let mut c = r.sets[0].checks[0].test.counts;
                c.inside -= 60;
                c.below += 60;
                r.sets[0].checks[0].test = Test::of(c);
            },
            "A, drag scale: ",
        );
        fails(
            &|r| {
                let passing = r.sets[1].checks[1].test;
                let test = r.sets[1].checks[1].probes.values_mut().next().unwrap();
                *test = passing;
            },
            "the probe",
        );
        fails(
            &|r| r.sets[0].checks[0].identification.identified = false,
            "not identified on set A",
        );
        fails(&|r| r.sets[1].said_nothing += 1, "a GNSS receiver's alone");
        fails(&|r| r.sets[0].largest_misfit = f64::NAN, "the misfit's cut");
        fails(
            &|r| {
                let id = r.noise[0].identified.values_mut().next().unwrap();
                id.identified = true;
            },
            "identified on noise",
        );
        fails(
            &|r| r.shares.get_mut(&Asked::DragScale).unwrap()[0].high = 1.44,
            "the report's shares aren't the library's",
        );
    }

    /// Every pull request fits the slice on every platform, unconditioned but for a change to
    /// research notes alone, in jobs of their own, each platform's parts fitting every one of its
    /// fits once, and the gate fits it whole in place of the whole fit; the night fits the whole.
    #[test]
    fn every_pull_request_and_the_gate_check_the_fits_slice() {
        let root = crate::designs::root().unwrap();
        let ci = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap();
        let ci = ci.replace("\r\n", "\n");
        let job = &ci[ci.find("\n  fit-slice-part:\n").unwrap()..];
        let job = &job[..=job[1..].find("\n  # ").unwrap()];
        const PLATFORMS: [&str; 5] = [
            "ubuntu-latest",
            "ubuntu-24.04-arm",
            "macos-latest",
            "windows-latest",
            "windows-11-arm",
        ];
        // Each platform's parts are 1 to n of one split, n the same for each of them.
        let mut parts: BTreeMap<&str, Vec<Part>> = BTreeMap::new();
        for line in job
            .lines()
            .filter(|l| l.trim_start().starts_with("- { os: "))
        {
            let line = line
                .trim()
                .trim_start_matches("- { os: ")
                .trim_end_matches(" }");
            let (os, part) = line.split_once(", part: ").unwrap();
            parts
                .entry(os)
                .or_default()
                .push(Part::parse(part).unwrap());
        }
        assert_eq!(
            parts.keys().copied().collect::<BTreeSet<_>>(),
            PLATFORMS.iter().copied().collect::<BTreeSet<_>>()
        );
        for (os, split) in &parts {
            let of = split[0].of;
            let numbers: Vec<usize> = split.iter().map(|p| p.number).collect();
            assert!(split.iter().all(|p| p.of == of), "{os}");
            assert_eq!(numbers, (1..=of).collect::<Vec<_>>(), "{os}");
        }
        assert!(job.contains("    runs-on: ${{ matrix.os }}\n"));
        assert!(job.contains("    needs: scope\n"));
        assert!(job.contains("      RUN: ${{ needs.scope.outputs.notes != 'true' }}\n"));
        for unseen in ["continue-on-error", "exclude:", "    if:"] {
            assert!(!job.contains(unseen), "{unseen}");
        }
        assert!(job.contains(
            "\n      - if: env.RUN == 'true'\n        run: cargo run --locked --quiet --package \
             xtask --profile report-check -- fit --slice --other-platform --part ${{ matrix.part }}\n"
        ));
        assert_eq!(ci.matches("-- fit --slice").count(), 1);
        // The checks branch protection requires, one a platform, pass only when every part did.
        let all = &ci[ci.find("\n  fit-slice:\n").unwrap()..];
        let all = &all[..=all[1..].find("\n  # ").unwrap()];
        assert!(all.contains("    name: fit slice (${{ matrix.os }})\n"));
        assert!(all.contains(&format!("        os: [{}]\n", PLATFORMS.join(", "))));
        assert!(all.contains("    needs: fit-slice-part\n"));
        assert!(all.contains("    if: always()\n"));
        assert!(
            all.contains("        run: test \"${{ needs.fit-slice-part.result }}\" = success\n")
        );
        // What's deployed and the night's alert wait for it.
        for needs in [
            "\n    needs: [fmt, clippy,",
            "\n    needs: [scope, fmt, clippy,",
        ] {
            let line = &ci[ci.find(needs).unwrap() + 1..];
            let line = &line[..line.find('\n').unwrap()];
            assert!(line.contains(" fit-slice,"), "{line}");
        }
        // The night fits the whole, the calibration set among it, and holds the slice in words.
        assert!(ci.contains("-- fit --check --other-platform --parts target/fit-parts\n"));
        let gate = fs::read_to_string(root.join("scripts/gate.sh")).unwrap();
        let step = gate
            .lines()
            .find(|line| line.trim_start().starts_with("validate) echo"))
            .unwrap();
        assert!(
            step.contains("--profile report-check -- fit --slice && "),
            "{step}"
        );
        assert!(!step.contains("fit --check"), "{step}");
        assert!(!step.contains("--other-platform"), "{step}");
    }
}
