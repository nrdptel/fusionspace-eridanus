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
//! The flights are flown on every core, each thread taking every `n`th index with a
//! [`Flyer`] of its own; a flight is the same however many came before it on its thread (the
//! module's tests), so the census doesn't depend on the number of threads.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_forensics::synthetic::census::{self, Range, Row, SetCensus};
use hpr_forensics::synthetic::designs::DESIGNS;
use hpr_forensics::synthetic::logger::Profile;
use hpr_forensics::synthetic::{Flyer, Set};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "  synthetic [--check]
                           Fly the synthetic flights' three sets (ADR-280) and write their census
                           to validation/reports/synthetic-flights.{json,md}. --check writes
                           nothing and fails unless the flights reproduce the committed census.";

/// The report's path under the repository, without its extension.
const REPORT: &str = "validation/reports/synthetic-flights";

/// The committed census of every set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Report {
    /// What it is.
    about: String,
    /// Each set's census, in [`Set::ALL`]'s order.
    sets: Vec<SetCensus>,
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
    let report = Report {
        about:
            "The synthetic flights' census (M7.2a, ADR-280), written by `cargo xtask synthetic`."
                .to_owned(),
        sets: Set::ALL.iter().map(|&set| fly(set)).collect(),
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

/// Flies every flight of `set` on every core and takes its census.
fn fly(set: Set) -> SetCensus {
    let count = set.count();
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16) as u64;
    let mut rows: Vec<Option<Result<Row, String>>> = vec![None; count as usize];
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                scope.spawn(move || {
                    let mut flyer = Flyer::new();
                    (start..count)
                        .step_by(threads as usize)
                        .map(|index| (index, census::row(&mut flyer, set, index)))
                        .collect::<Vec<_>>()
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

/// A number as the page shows it: up to three decimals, trailing zeros dropped.
fn number(x: f64) -> String {
    let text = format!("{x:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" {
        "0".to_owned()
    } else {
        text.to_owned()
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
    page
}
