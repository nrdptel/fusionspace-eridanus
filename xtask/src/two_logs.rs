//! `cargo xtask two-logs [--check]`: holds the clock offset between two logs of one flight to the
//! synthetic flights (`hpr_forensics::synthetic::paired`, M7.2f, ADR-280), and writes
//! `validation/reports/two-logs.json` and `.md`.
//!
//! The two-logger calibration set fits the offset's shares again, which must equal the library's
//! to three significant figures. On two-logger sets A and B, on seeds of their own, every offset
//! must stay within the bound it states, and its 90% interval must pass ADR-257's test while the
//! intervals narrowed, widened and shifted down each fail it. On the staged set, a logger in each
//! stage, every offset must stay within its bound. Values are kept to three significant figures,
//! the digits every platform shares, so CI holds the report on macOS, Windows and Linux. `--check`
//! writes nothing, and fails unless the flights reproduce the committed report.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};

use hpr_flightdata::readings::Reason;
use hpr_flightdata::timeline::OFFSET_SHARES;
use hpr_forensics::synthetic::designs::DesignFiles;
use hpr_forensics::synthetic::intervals::{Probe, Test, three_figures};
use hpr_forensics::synthetic::paired::{self, Check, Said, Staged, TwoLogs};
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "  two-logs [--check]
                           Hold the clock offset between two logs of one flight to two-logger
                           and staged synthetic flights, writing validation/reports/two-logs.{json,md}
                           and the slice, two-logs.slice.json; --check writes nothing and fails
                           unless the flights reproduce all three.
  two-logs --slice         Fly only the slice, every 21st flight of each set, and hold each to the
                           committed slice and to the report's per-flight rules; writes nothing.";

const REPORT: &str = "validation/reports/two-logs";
const TWO_STAGE_DESIGN: &str = "validation/designs/synthetic-two-stage-75mm-54mm.json";

/// One set's section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Section {
    set: Set,
    seed: u64,
    drawn: u64,
    check: Check,
}

/// The report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Report {
    fitted_low: f64,
    fitted_high: f64,
    calibration_given: u64,
    calibration_withheld: BTreeMap<Reason, u64>,
    sections: Vec<Section>,
    staged: Section,
}

/// Runs `cargo xtask two-logs`.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check_only = false;
    let mut only_slice = false;
    let mut other_platform = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check_only = true,
            "--slice" => only_slice = true,
            "--other-platform" => other_platform = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let files = crate::synthetic::design_files(&root)?;
    let two_stage = fs::read_to_string(root.join(TWO_STAGE_DESIGN))
        .map_err(|error| format!("{TWO_STAGE_DESIGN}: {error}"))?;
    let staged_flyer = Staged::new(two_stage);
    if other_platform && !only_slice {
        return Err("--other-platform goes with --slice".into());
    }
    if only_slice {
        if check_only {
            return Err("--slice takes no argument but --other-platform".into());
        }
        let held = if other_platform {
            crate::slice::Held::Words
        } else {
            crate::slice::Held::Exact
        };
        return check_slice(&root, &files, &staged_flyer, held);
    }
    let mut broken = Vec::new();

    let calibration = fly_set(
        Set::PairCalibration,
        |flyer, index| flyer.two_logs(Set::PairCalibration, index),
        &files,
    )?;
    let shares = paired::fitted(&calibration)
        .ok_or("the two-logger calibration set gave too few offsets to fit")?;
    let (fitted_low, fitted_high) = (three_figures(shares.low), three_figures(shares.high));
    if (fitted_low, fitted_high) != (OFFSET_SHARES.low, OFFSET_SHARES.high) {
        broken.push(format!(
            "the offset's shares fit {fitted_low} and {fitted_high}; the library has {} and {}",
            OFFSET_SHARES.low, OFFSET_SHARES.high
        ));
    }
    let calibration_check = paired::check(&calibration);
    problems_of(Set::PairCalibration, &calibration_check, false, &mut broken);

    let mut sections = Vec::new();
    for set in [Set::PairA, Set::PairB] {
        let flights = fly_set(set, |flyer, index| flyer.two_logs(set, index), &files)?;
        let check = paired::check(&flights);
        problems_of(set, &check, true, &mut broken);
        sections.push(section(set, check));
    }
    let flights = fly_staged(&staged_flyer)?;
    let check = paired::check(&flights);
    problems_of(Set::Staged, &check, false, &mut broken);
    let staged = section(Set::Staged, check);

    let report = Report {
        fitted_low,
        fitted_high,
        calibration_given: calibration_check.given,
        calibration_withheld: calibration_check.withheld,
        sections,
        staged,
    };
    for section in report.sections.iter().chain([&report.staged]) {
        println!(
            "{}: {} of {} within the bound, largest share {}, median error {} s, 90% of errors under {} s",
            section.set.name(),
            section.check.within_bound,
            section.check.given,
            section.check.largest_share,
            section.check.median_error_s,
            section.check.p90_error_s,
        );
        println!(
            "{}: below/inside/above {}/{}/{}",
            section.set.name(),
            section.check.test.counts.below,
            section.check.test.counts.inside,
            section.check.test.counts.above
        );
    }
    println!("offset shares fitted: {fitted_low}, {fitted_high}");
    // The slice's flights, which the whole run holds the committed slice to, and writes.
    let (slice, slice_broken) = make_slice(&files, &staged_flyer)?;
    broken.extend(slice_broken);
    if !broken.is_empty() {
        return Err(broken.join("\n"));
    }

    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    let page = markdown(&report);
    let json_path = root.join(format!("{REPORT}.json"));
    let page_path = root.join(format!("{REPORT}.md"));
    let slice_path = root.join(format!("{REPORT}.slice.json"));
    if check_only {
        // The whole check runs each night on every platform, in the optimized build: the slice's
        // words, its numbers aside (`Held::Words`). The gate's `--slice` holds them exactly.
        let mut problems: Vec<String> =
            crate::slice::compare(&slice_path, &slice, crate::slice::Held::Words)
                .into_iter()
                .collect();
        for (path, made) in [(&json_path, &json), (&page_path, &page)] {
            match fs::read_to_string(path) {
                Ok(committed) if committed.replace("\r\n", "\n") == *made => {}
                Ok(_) => problems.push(format!(
                    "{} doesn't match what the flights make; run `cargo xtask two-logs`",
                    path.display()
                )),
                Err(error) => problems.push(format!("{}: {error}", path.display())),
            }
        }
        if !problems.is_empty() {
            return Err(problems.join("\n"));
        }
        println!("the flights reproduce {REPORT}.json, .md and .slice.json");
    } else {
        fs::write(&json_path, json).map_err(|e| e.to_string())?;
        fs::write(&page_path, page).map_err(|e| e.to_string())?;
        fs::write(&slice_path, crate::slice::json(&slice)?).map_err(|e| e.to_string())?;
        println!("wrote {REPORT}.json, .md and .slice.json");
    }
    Ok(())
}

/// Two logs' slice takes every 21st flight of each set (M0.16a1, ADR-149 amended): 96 of the
/// calibration set's 2,000, 48 each of sets A and B, 10 of the staged set's 200. The step is odd,
/// as the synthetic flights' is, so the calibration set's slice holds both of its models.
const SLICE_EVERY: u64 = 21;

/// The slice's `about` line.
const SLICE_ABOUT: &str = "Two logs' slice (M0.16a1, ADR-149 amended): every 21st flight of each \
                           set, its logs and its offset, written by `cargo xtask two-logs`; \
                           `--slice` holds it.";

/// The sets, in the report's order.
const SETS: [Set; 4] = [Set::PairCalibration, Set::PairA, Set::PairB, Set::Staged];

/// One flight of the slice: its record, and what its logs say of the offset.
fn slice_flight(set: Set, logs: &TwoLogs) -> (crate::slice::Flight, Said) {
    let said = paired::said(logs);
    let log = |profile: hpr_forensics::synthetic::logger::Profile,
               log: &hpr_flightdata::log::FlightLog| {
        let samples = log.time_s.len();
        // A height that isn't a number shows as one: `f64::max` would pass over it.
        let heights = log.altitude_m.iter().copied();
        let highest = if log.altitude_m.iter().any(|h| !h.is_finite()) {
            Some(f64::NAN)
        } else {
            heights.reduce(f64::max)
        };
        match highest {
            Some(highest) => format!(
                "{}: {samples} samples, highest {} m",
                profile.name(),
                crate::slice::figures(highest)
            ),
            None => format!("{}: {samples} samples, no height", profile.name()),
        }
    };
    let offset = match &said.offset {
        Ok(c) => format!(
            "offset: {} s, truth {} s, {} of {} s, {}",
            crate::slice::figures(c.reading),
            crate::slice::figures(c.truth),
            crate::slice::share((c.reading - c.truth) / c.bound),
            crate::slice::figures(c.bound),
            match (
                c.truth.is_nan() || c.truth < c.interval.low,
                c.truth.is_nan() || c.truth > c.interval.high
            ) {
                (true, true) => "both",
                (true, false) => "below",
                (false, true) => "above",
                (false, false) => "inside",
            }
        ),
        Err(reason) => format!("offset: withheld ({reason:?})"),
    };
    let flight = crate::slice::Flight {
        set: set.name().to_owned(),
        index: logs.index,
        made: vec![
            log(logs.first, &logs.first_log),
            log(logs.second, &logs.second_log),
            offset,
        ],
    };
    (flight, said)
}

/// Flies the slice's flights on every core: the slice, and what breaks the rules the report holds
/// every flight to (an offset within its bound, withheld only for a receiver's dropout).
fn make_slice(
    files: &DesignFiles,
    staged: &Staged,
) -> Result<(crate::slice::Slice, Vec<String>), String> {
    let jobs: Vec<(Set, u64)> = SETS
        .iter()
        .flat_map(|&set| {
            crate::slice::indices(set.count(), SLICE_EVERY)
                .into_iter()
                .map(move |index| (set, index))
        })
        .collect();
    let made = crate::slice::on_cores(
        &jobs,
        |(set, index)| format!("{} flight {index}", set.name()),
        || Flyer::new(files.clone()),
        |flyer, &(set, index)| {
            let logs = if set == Set::Staged {
                staged.flight(index).map(|(_, logs)| logs)
            } else {
                flyer.two_logs(set, index)
            }
            .map_err(|e| e.to_string())?;
            Ok((set, slice_flight(set, &logs)))
        },
    )?;
    let mut broken = Vec::new();
    for set in SETS {
        let said: Vec<Said> = made
            .iter()
            .filter(|(s, _)| *s == set)
            .map(|(_, (_, said))| said.clone())
            .collect();
        problems_of(set, &paired::check(&said), false, &mut broken);
    }
    let slice = crate::slice::Slice {
        about: SLICE_ABOUT.to_owned(),
        every: SLICE_EVERY,
        flights: made.into_iter().map(|(_, (flight, _))| flight).collect(),
    };
    Ok((slice, broken))
}

/// `cargo xtask two-logs --slice`: the slice's flights flown, held to the committed slice and to
/// the report's per-flight rules; the committed report read and its page rendered from it.
fn check_slice(
    root: &std::path::Path,
    files: &DesignFiles,
    staged: &Staged,
    held: crate::slice::Held,
) -> Result<(), String> {
    let json_path = root.join(format!("{REPORT}.json"));
    let page_path = root.join(format!("{REPORT}.md"));
    let committed = fs::read_to_string(&json_path)
        .map_err(|error| format!("{}: {error}", json_path.display()))?;
    let report: Report = serde_json::from_str(&committed)
        .map_err(|error| format!("{}: {error}", json_path.display()))?;
    let mut problems = Vec::new();
    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    if committed.replace("\r\n", "\n") != json {
        problems.push(format!(
            "{} doesn't read and write back unchanged",
            json_path.display()
        ));
    }
    match fs::read_to_string(&page_path) {
        Ok(page) if page.replace("\r\n", "\n") == markdown(&report) => {}
        Ok(_) => problems.push(format!(
            "{} isn't what {} renders",
            page_path.display(),
            json_path.display()
        )),
        Err(error) => problems.push(format!("{}: {error}", page_path.display())),
    }
    let (made, broken) = make_slice(files, staged)?;
    problems.extend(broken);
    problems.extend(crate::slice::compare(
        &root.join(format!("{REPORT}.slice.json")),
        &made,
        held,
    ));
    if !problems.is_empty() {
        return Err(format!(
            "two logs' slice doesn't hold; run `cargo xtask two-logs` and commit what it writes \
             if the change is meant:\n{}",
            problems.join("\n")
        ));
    }
    println!(
        "the slice's {} flights reproduce {REPORT}.slice.json {} and keep the report's rules; \
         {REPORT}.json reads and renders {REPORT}.md",
        made.flights.len(),
        crate::slice::held_text(held)
    );
    Ok(())
}

/// What fails the check on `set`: a bound passed, a flight withheld for an unforeseen reason and,
/// `with_interval`, the coverage test.
fn problems_of(set: Set, check: &Check, with_interval: bool, broken: &mut Vec<String>) {
    // A flight dropped for another reason would take its misses out of the counts.
    broken.extend(
        check
            .withheld
            .iter()
            .filter(|(reason, _)| **reason != Reason::ReceiverDropout)
            .map(|(reason, count)| {
                format!("{}: {count} offsets withheld as {reason:?}", set.name())
            }),
    );
    if !check.bound_holds() {
        broken.push(format!(
            "{}: {} of {} offsets within their bound, the largest {} of it",
            set.name(),
            check.within_bound,
            check.given,
            check.largest_share
        ));
    }
    if !with_interval {
        return;
    }
    if !check.test.passes() {
        broken.push(format!(
            "{}: the offset's intervals fail: {:?}",
            set.name(),
            check.test.counts
        ));
    }
    for (probe, test) in &check.probes {
        if test.passes() {
            broken.push(format!(
                "{}: the offset's intervals {} still pass: {:?}",
                set.name(),
                probe.name(),
                test.counts
            ));
        }
    }
}

/// `check` as the report holds it, to the digits the platforms share.
fn section(set: Set, mut check: Check) -> Section {
    check.largest_share = three_figures(check.largest_share);
    check.median_error_s = three_figures(check.median_error_s);
    check.p90_error_s = three_figures(check.p90_error_s);
    check.largest_error_s = three_figures(check.largest_error_s);
    check.median_bound_s = three_figures(check.median_bound_s);
    for cell in check.by_loggers.values_mut() {
        cell.largest_share = three_figures(cell.largest_share);
    }
    let round_band = |test: &mut Test| {
        for band in [&mut test.below, &mut test.inside, &mut test.above] {
            band.low = three_figures(band.low);
            band.high = three_figures(band.high);
        }
    };
    round_band(&mut check.test);
    check.probes.values_mut().for_each(round_band);
    Section {
        set,
        seed: set.seed(),
        drawn: set.count(),
        check,
    }
}

/// The threads to share a set's flights among.
fn threads() -> u64 {
    std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64
}

/// What every flight of `set` says, flown by `flown`, each thread taking every `n`th index with a
/// flyer of its own, as `cargo xtask synthetic` does. A flight that doesn't fly fails the check.
fn fly_set(
    set: Set,
    flown: impl Fn(&mut Flyer, u64) -> Result<TwoLogs, hpr_forensics::synthetic::SyntheticError> + Sync,
    files: &DesignFiles,
) -> Result<Vec<Said>, String> {
    collected(set, |start, threads| {
        let mut flyer = Flyer::new(files.clone());
        let mut done = Vec::new();
        for index in (start..set.count()).step_by(threads as usize) {
            let said = panic::catch_unwind(AssertUnwindSafe(|| {
                flown(&mut flyer, index)
                    .map(|logs| paired::said(&logs))
                    .map_err(|e| e.to_string())
            }))
            .unwrap_or_else(|_| {
                flyer = Flyer::new(files.clone());
                Err("it panicked".to_owned())
            })
            .map_err(|e| format!("{} flight {index}: {e}", set.name()));
            done.push(said);
        }
        done
    })
}

/// What every flight of the staged set says.
fn fly_staged(staged: &Staged) -> Result<Vec<Said>, String> {
    let set = Set::Staged;
    collected(set, |start, threads| {
        (start..set.count())
            .step_by(threads as usize)
            .map(|index| {
                panic::catch_unwind(AssertUnwindSafe(|| {
                    staged
                        .flight(index)
                        .map(|(_, logs)| paired::said(&logs))
                        .map_err(|e| e.to_string())
                }))
                .unwrap_or_else(|_| Err("it panicked".to_owned()))
                .map_err(|e| format!("{} flight {index}: {e}", set.name()))
            })
            .collect()
    })
}

/// Runs `each` on every thread and gathers its flights in index order, or every failure.
fn collected(
    set: Set,
    each: impl Fn(u64, u64) -> Vec<Result<Said, String>> + Sync,
) -> Result<Vec<Said>, String> {
    let threads = threads();
    let each = &each;
    let mut said = Vec::new();
    let mut problems = Vec::new();
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| scope.spawn(move || each(start, threads)))
            .collect();
        for handle in handles {
            match handle.join() {
                Ok(done) => {
                    for flight in done {
                        match flight {
                            Ok(flight) => said.push(flight),
                            Err(message) => problems.push(message),
                        }
                    }
                }
                Err(_) => problems.push(format!("{}: a thread panicked", set.name())),
            }
        }
    });
    if !problems.is_empty() {
        problems.sort();
        return Err(problems.join("\n"));
    }
    said.sort_by_key(|flight| flight.index);
    Ok(said)
}

/// The report's page.
fn markdown(report: &Report) -> String {
    let mut page = String::from(
        "# Two logs of one flight: the clock offset\n\n\
         Written by `cargo xtask two-logs`; don't edit it by hand. How the offset is read: \
         [reading a flight log](../../docs/reading-a-flight-log.md). What it checks: each flight \
         carries two simulated loggers, each with a clock of its own, and the offset their two \
         liftoffs give is held to the true one. It says how well a shared liftoff recovers a \
         clock offset on HPR Sim's flights, not on a real airframe.\n\n",
    );
    let _ = writeln!(
        page,
        "The offset's 90% interval takes the shares {} and {} of its bound, fitted on {} offsets \
         of the two-logger calibration set.\n",
        report.fitted_low, report.fitted_high, report.calibration_given
    );
    page.push_str(
        "| Set | Flights | Offsets given | Within the bound | Largest share of the bound | \
         Median error, s | 90% of errors under, s | Largest error, s | Median bound, s |\n\
         |---|--:|--:|--:|--:|--:|--:|--:|--:|\n",
    );
    for section in report.sections.iter().chain([&report.staged]) {
        let c = &section.check;
        let _ = writeln!(
            page,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            section.set.name(),
            section.drawn,
            c.given,
            c.within_bound,
            c.largest_share,
            c.median_error_s,
            c.p90_error_s,
            c.largest_error_s,
            c.median_bound_s
        );
    }
    page.push_str(
        "\n## The 90% interval\n\nADR-257's test: per tail a 99.9% Clopper–Pearson band holding \
         5%, the count inside holding 90%; the probes must fail it.\n\n\
         | Set | Below | Inside | Above | Passes |",
    );
    for probe in Probe::ALL {
        let _ = write!(page, " {} |", probe.name());
    }
    page.push_str("\n|---|--:|--:|--:|---|---|---|---|\n");
    for section in &report.sections {
        let c = &section.check;
        let word = |pass: bool| if pass { "passes" } else { "fails" };
        let _ = write!(
            page,
            "| {} | {} | {} | {} | {} |",
            section.set.name(),
            c.test.counts.below,
            c.test.counts.inside,
            c.test.counts.above,
            word(c.test.passes())
        );
        for probe in Probe::ALL {
            let _ = write!(page, " {} |", word(c.probes[&probe].passes()));
        }
        page.push('\n');
    }
    page.push_str("\n## Offsets withheld\n\nA log with no liftoff sits on no timeline.\n\n");
    page.push_str("| Set | Reason | Flights |\n|---|---|--:|\n");
    for (name, withheld) in [("pair-calibration", &report.calibration_withheld)]
        .into_iter()
        .chain(
            report
                .sections
                .iter()
                .chain([&report.staged])
                .map(|s| (s.set.name(), &s.check.withheld)),
        )
    {
        if withheld.is_empty() {
            let _ = writeln!(page, "| {name} | none | 0 |");
        }
        for (reason, count) in withheld {
            let _ = writeln!(page, "| {name} | {reason:?} | {count} |");
        }
    }
    page.push_str("\n## By logger pair\n\n");
    page.push_str("| Set | Loggers | Offsets | Largest share of the bound |\n|---|---|--:|--:|\n");
    for section in report.sections.iter().chain([&report.staged]) {
        for (loggers, cell) in &section.check.by_loggers {
            let _ = writeln!(
                page,
                "| {} | {loggers} | {} | {} |",
                section.set.name(),
                cell.given,
                cell.largest_share
            );
        }
    }
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One changed flight writes other lines, so a slice holding it fails ([`crate::slice`]'s
    /// tests name a flight whose lines differ); flown again unchanged, it writes the same lines.
    #[test]
    fn one_changed_flight_writes_other_lines() {
        let root = crate::designs::root().unwrap();
        let files = crate::synthetic::design_files(&root).unwrap();
        let mut flyer = Flyer::new(files);
        // The slice's first flight of set A whose two logs give an offset.
        let logs = crate::slice::indices(Set::PairA.count(), SLICE_EVERY)
            .into_iter()
            .map(|index| flyer.two_logs(Set::PairA, index).unwrap())
            .find(|logs| paired::said(logs).offset.is_ok())
            .unwrap();
        let (flight, _) = slice_flight(Set::PairA, &logs);
        assert_eq!(flight.made.len(), 3, "{:?}", flight.made);
        assert!(flight.made[2].starts_with("offset: "), "{:?}", flight.made);
        let again = flyer.two_logs(Set::PairA, logs.index).unwrap();
        assert_eq!(slice_flight(Set::PairA, &again).0, flight);
        // The second log's clock a tenth of a second later: its offset's line.
        let mut later = logs.clone();
        for time in &mut later.second_log.time_s {
            *time += 0.1;
        }
        later.true_offset_s += 0.1;
        let (changed, _) = slice_flight(Set::PairA, &later);
        assert_eq!(changed.made[..2], flight.made[..2]);
        assert_ne!(changed.made[2], flight.made[2]);
        // The first log read 0.5% high: its own line.
        let mut misread = logs.clone();
        for height in &mut misread.first_log.altitude_m {
            *height *= 1.005;
        }
        assert_ne!(slice_flight(Set::PairA, &misread).0.made[0], flight.made[0]);
    }

    /// The slice's flights are fixed: every 21st of each set, from the first.
    #[test]
    fn the_slice_takes_every_21st_flight_of_each_set() {
        assert_eq!(SLICE_EVERY, 21);
        let counts: Vec<usize> = SETS
            .iter()
            .map(|set| crate::slice::indices(set.count(), SLICE_EVERY).len())
            .collect();
        assert_eq!(counts, vec![96, 48, 48, 10]);
    }
}
