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
//! The flights are flown on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a flight is the same however many came before it on its thread (the
//! module's tests), so the census doesn't depend on the number of threads. A flight that panics
//! is counted as not flown, with the panic's message, rather than lost with its thread.
//!
//! The designs' files are read from `validation/designs/` here: `hpr-forensics` is published
//! and can't hold files outside its own folder.

use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use hpr_forensics::synthetic::census::{self, Range, Row, SetCensus};
use hpr_forensics::synthetic::designs::{DESIGNS, DesignFiles};
use hpr_forensics::synthetic::held::{self, Outcome, Quantity, Tally};
use hpr_forensics::synthetic::logger::Profile;
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "  synthetic [--check]
                           Fly the synthetic flights' three sets (ADR-280) and write their census
                           to validation/reports/synthetic-flights.{json,md}, with the tuning
                           set's flight-log readings held to their bounds. --check writes
                           nothing and fails unless the flights reproduce the committed census.
                           Either fails if a reading's error passes its bound.";

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
}

/// Runs the command.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let files = design_files(&root)?;
    let tallies = read_logs(Set::Tuning.count(), &files)?;
    let missed = misses(&tallies);
    if !missed.is_empty() {
        return Err(format!(
            "a reading's error passes its bound on the tuning set:\n{}",
            missed.join("\n")
        ));
    }
    let report = Report {
        about: "The synthetic flights' census (M7.2a, ADR-280) and the flight-log readings held \
                to their bounds on the tuning set (M7.2b), written by `cargo xtask synthetic`."
            .to_owned(),
        sets: Set::ALL
            .iter()
            .map(|&set| fly(set, set.count(), &files))
            .collect(),
        readings: tallies.iter().map(rounded).collect(),
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

/// One tuning flight's logs, each named, with each reading's outcome.
type FlightOutcomes = (u64, Vec<(String, Vec<(Quantity, Outcome)>)>);

/// Logs the first `count` tuning flights at every rate and step of `held`'s grid on every core,
/// reads each log, and tallies the readings against their bounds. A flight that doesn't fly is
/// the census's to report; a log that can't be written, or a panic, fails the command.
fn read_logs(count: u64, files: &DesignFiles) -> Result<Vec<Tally>, String> {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let set = Set::Tuning;
    let mut flights: Vec<FlightOutcomes> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                let files = files.clone();
                scope.spawn(move || -> Result<Vec<FlightOutcomes>, String> {
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
                        done.push((index, outcomes));
                    }
                    Ok(done)
                })
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
    flights.sort_by_key(|(index, _)| *index);
    Ok(held::tally(&flights))
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

/// A set's name as the page heads its column.
fn title(set: Set) -> &'static str {
    match set {
        Set::Tuning => "Tuning",
        Set::A => "A",
        Set::B => "B",
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
    page
}

/// The page's section on the flight-log readings held to their bounds.
fn readings_section(page: &mut String, tallies: &[Tally]) {
    page.push_str(
        "\n## Readings within their bounds\n\n\
         Each tuning flight logged by its own logger, then again at every rate and step of the \
         grid, from 500 to half a sample a second and from no step to 100 Pa, and read by \
         `hpr_flightdata::readings` (M7.2b). Each cell gives how many of the flights' readings \
         were given, then the largest share of its bound an error took: 1 is the bound, and \
         `cargo xtask synthetic` fails above it. The JSON names each worst flight, its error and \
         its bound. What is method and what is measured: `docs/physics/log-readings.md`.\n\n",
    );
    let mut logs: Vec<&str> = Vec::new();
    for tally in tallies {
        if !logs.contains(&tally.log.as_str()) {
            logs.push(&tally.log);
        }
    }
    page.push_str("| Log |");
    for quantity in Quantity::ALL {
        let _ = write!(page, " {} |", quantity.name());
    }
    page.push_str("\n|---|");
    for _ in Quantity::ALL {
        page.push_str("---:|");
    }
    page.push('\n');
    for log in &logs {
        let _ = write!(page, "| {log} |");
        for quantity in Quantity::ALL {
            let cell = tallies
                .iter()
                .find(|tally| tally.log == *log && tally.quantity == quantity)
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
    page.push_str("\n### Readings withheld\n\n");
    let withheld: Vec<(&Tally, String, u64)> = tallies
        .iter()
        .flat_map(|tally| {
            tally.withheld.iter().map(move |(reason, count)| {
                let code = serde_json::to_value(reason)
                    .ok()
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default();
                (tally, code, *count)
            })
        })
        .collect();
    if withheld.is_empty() {
        page.push_str("None was withheld.\n");
    } else {
        page.push_str("| Log | Reading | Reason | Flights |\n|---|---|---|---:|\n");
        for (tally, code, count) in withheld {
            let _ = writeln!(
                page,
                "| {} | {} | `{code}` | {count} |",
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
    fn the_threaded_readings_are_the_readings_taken_in_order() {
        let root = crate::designs::root().unwrap();
        let files = design_files(&root).unwrap();
        let mut flyer = Flyer::new(files.clone());
        let flights: Vec<FlightOutcomes> = (0..3)
            .map(|index| {
                let flight = flyer.flight(Set::Tuning, index).unwrap();
                let logs = held::logs(&flight, Set::Tuning.seed(), index).unwrap();
                let outcomes = logs
                    .iter()
                    .map(|(name, log)| (name.clone(), held::outcomes(&flight, log)))
                    .collect();
                (index, outcomes)
            })
            .collect();
        assert_eq!(read_logs(3, &files).unwrap(), held::tally(&flights));
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
        });
        assert!(page.contains("| A | 0 |"), "{page}");
        assert!(page.contains("first \\| second third |"), "{page}");
    }

    #[test]
    fn a_number_reads_as_the_census_holds_it() {
        assert_eq!(number(4_810.0), "4810");
        assert_eq!(number(0.269), "0.269");
        assert_eq!(number(-14.6), "-14.6");
        assert_eq!(number(-0.0), "0");
        assert_eq!(number(0.000_123), "0.000123");
    }
}
