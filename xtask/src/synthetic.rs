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
use hpr_forensics::synthetic::held::{self, Errors, Outcome, Quantity, Tally};
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
    /// The tuning set's errors, each reading's over every log, in the order [`held::errors`]
    /// gives them, to three significant figures.
    errors: Vec<Errors>,
    /// The tuning set's top accelerations against their parts' ranges.
    tops: Tops,
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
    /// Each top read as clipped, a floor, by flight and log, in the flights' order.
    clipped: Vec<Clipped>,
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

/// A top acceleration read as clipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Clipped {
    /// The flight's index in the tuning set.
    flight: u64,
    /// Its logger's profile.
    profile: String,
    /// The log it was read from.
    log: String,
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
    let (flights, each) = read_logs(Set::Tuning.count(), &files)?;
    let tallies = held::tally(&flights);
    let missed = misses(&tallies);
    if !missed.is_empty() {
        return Err(format!(
            "a reading's error passes its bound on the tuning set:\n{}",
            missed.join("\n")
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

/// One tuning flight's index, its logger's profile, and its true top over its part's range
/// ([`held::top_over_range`]).
type FlightTop = (u64, &'static str, Option<f64>);

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
            .find(|(index, _, _)| *index == flight)
            .map_or("unknown", |(_, profile, _)| *profile)
            .to_owned()
    };
    let shares = each
        .iter()
        .filter_map(|(flight, profile, share)| share.map(|share| (*flight, *profile, share)));
    let reaching = shares.clone().filter(|(_, _, share)| *share >= 1.0).count() as u64;
    let nearest = shares
        .max_by(|a, b| a.2.total_cmp(&b.2))
        .map(|(flight, profile, share)| Top {
            flight,
            profile: profile.to_owned(),
            share: census::three_figures(share),
        });
    let clipped = flights
        .iter()
        .flat_map(|(flight, logs)| {
            logs.iter()
                .filter(|(_, results)| {
                    results.iter().any(|(quantity, outcome)| {
                        *quantity == Quantity::TopAcceleration
                            && matches!(outcome, Outcome::Floor { .. })
                    })
                })
                .map(move |(log, _)| (*flight, log.clone()))
        })
        .map(|(flight, log)| Clipped {
            flight,
            profile: profile(flight),
            log,
        })
        .collect();
    Tops {
        reaching,
        nearest,
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

/// A reading's errors to three significant figures, as the report holds them.
fn rounded_errors(errors: &Errors) -> Errors {
    let mut errors = errors.clone();
    if let Some(spread) = &mut errors.spread {
        spread.mean_error = census::three_figures(spread.mean_error);
        spread.least_error = census::three_figures(spread.least_error);
        spread.most_error = census::three_figures(spread.most_error);
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
    errors_section(&mut page, &report.errors, &report.tops);
    page
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
         the most may pass the mean bound; the mean is for scale. A floor, a burnout from the \
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
        0 => "No tuning flight's true top acceleration reaches the range its part clips at"
            .to_owned(),
        1 => {
            "1 tuning flight's true top acceleration reaches the range its part clips at".to_owned()
        }
        n => format!(
            "{n} tuning flights' true top accelerations reach the range their part clips at"
        ),
    };
    let _ = write!(
        page,
        "\n{reach}: the nearest, flight {}'s on its {} logger, comes to {} of it. ",
        nearest.flight,
        nearest.profile,
        number(nearest.share)
    );
    if tops.clipped.is_empty() {
        page.push_str("No top was read as clipped.\n");
    } else {
        let each: Vec<String> = tops
            .clipped
            .iter()
            .map(|c| format!("flight {} ({}), {}", c.flight, c.profile, c.log))
            .collect();
        let _ = writeln!(
            page,
            "Tops read as clipped, each a floor, at the range or flat at the top: {}; {}.",
            tops.clipped.len(),
            each.join("; ")
        );
    }
}

/// The page's section on the flight-log readings held to their bounds.
fn readings_section(page: &mut String, tallies: &[Tally]) {
    page.push_str(
        "\n## Readings within their bounds\n\n\
         Each tuning flight logged by its own logger, then again at every rate and step of the \
         grid, from 500 to half a sample a second and from no step to 100 Pa, with its own \
         logger's accelerometer at 500, 100 and 20 Hz, and cropped as loggers start their logs, \
         and read by `hpr_flightdata::readings` (M7.2b, M7.2c). Each cell gives how many of the \
         flights' readings \
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
                clipped: Vec::new(),
            },
        });
        assert!(page.contains("| A | 0 |"), "{page}");
        assert!(page.contains("first \\| second third |"), "{page}");
    }

    /// The tops: a share of 1 reaches the range, the nearest is the largest share, and each
    /// clipped top is named by its flight's profile and its log; with none, the page says so.
    #[test]
    fn the_tops_name_the_nearest_and_each_clipped() {
        let floor = Outcome::Floor {
            error: -1.0,
            bound: 2.0,
        };
        let given = Outcome::Given {
            error: 0.5,
            bound: 2.0,
        };
        let flights: Vec<FlightOutcomes> = vec![
            (
                0,
                vec![
                    ("altos".to_owned(), vec![(Quantity::TopAcceleration, given)]),
                    (
                        "its accelerometer at 20 Hz".to_owned(),
                        vec![
                            (Quantity::TopAcceleration, floor),
                            (Quantity::Burnout, floor),
                        ],
                    ),
                ],
            ),
            (
                1,
                vec![("rrc3".to_owned(), vec![(Quantity::Burnout, floor)])],
            ),
            (
                2,
                vec![(
                    "imu-500hz".to_owned(),
                    vec![(Quantity::TopAcceleration, given)],
                )],
            ),
        ];
        let each: Vec<FlightTop> = vec![
            (0, "altos", Some(0.25)),
            (1, "rrc3", None),
            (2, "imu-500hz", Some(1.0)),
        ];
        let made = tops(&flights, &each);
        assert_eq!(made.reaching, 1);
        let nearest = made.nearest.as_ref().unwrap();
        assert_eq!((nearest.flight, nearest.share), (2, 1.0));
        assert_eq!(
            made.clipped,
            [Clipped {
                flight: 0,
                profile: "altos".to_owned(),
                log: "its accelerometer at 20 Hz".to_owned(),
            }]
        );
        let mut page = String::new();
        tops_paragraph(&mut page, &made);
        assert!(
            page.contains("1 tuning flight's true top acceleration reaches"),
            "{page}"
        );
        assert!(
            page.contains("flight 0 (altos), its accelerometer at 20 Hz"),
            "{page}"
        );
        let none = tops(&flights, &[(0, "altos", Some(0.25))]);
        let mut page = String::new();
        tops_paragraph(&mut page, &none);
        assert!(
            page.contains("No tuning flight's true top acceleration reaches"),
            "{page}"
        );
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
}
