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
                           and staged synthetic flights, writing validation/reports/two-logs.{json,md}";

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
    for arg in args {
        match arg.as_str() {
            "--check" => check_only = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let files = crate::synthetic::design_files(&root)?;
    let two_stage = fs::read_to_string(root.join(TWO_STAGE_DESIGN))
        .map_err(|error| format!("{TWO_STAGE_DESIGN}: {error}"))?;
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
    let staged_flyer = Staged::new(two_stage);
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
    if !broken.is_empty() {
        return Err(broken.join("\n"));
    }

    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    let page = markdown(&report);
    let json_path = root.join(format!("{REPORT}.json"));
    let page_path = root.join(format!("{REPORT}.md"));
    if check_only {
        for (path, made) in [(&json_path, &json), (&page_path, &page)] {
            let committed =
                fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
            if committed.replace("\r\n", "\n") != *made {
                return Err(format!(
                    "{} doesn't match what the flights make; run `cargo xtask two-logs`",
                    path.display()
                ));
            }
        }
        println!("the flights reproduce {REPORT}.json and .md");
    } else {
        fs::write(&json_path, json).map_err(|e| e.to_string())?;
        fs::write(&page_path, page).map_err(|e| e.to_string())?;
        println!("wrote {REPORT}.json and .md");
    }
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
