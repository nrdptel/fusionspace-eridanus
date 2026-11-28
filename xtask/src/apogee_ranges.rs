//! How often the 50% and 90% apogee ranges of a Monte Carlo run hold the private collection's
//! logged apogees ([M2.8a][m2-8a]; [the decision][adr-386]).
//!
//! `cargo xtask fixture-flights` flies each flight it compares with its log once
//! ([`crate::fixture_flights`]). This flies each of them [`SAMPLES`] times more, as the run
//! `hpr mc` flies, on the spreads a design file is known by
//! (`hpr_validate::apogee_ranges::SPREADS`), and counts how often the run's ranges hold the
//! logged apogee, over every flight compared and over each half
//! (`hpr_validate::apogee_ranges::Coverage`). The report holds counts and medians over sets of
//! flights, nothing of one flight: the collection's license allows only aggregates (ADR-151).
//!
//! A flight's nominal inputs are the ones the collection's report flew: the design as read, in
//! the day's ERA5 air, from its rail, with the separations the climb flies and nothing deployed
//! ([`inputs_of`]). Before a run its nominal flight is flown again from those inputs and held to
//! the report's apogee, so the run is of the flight the errors are of.
//!
//! [m2-8a]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-8a
//! [adr-386]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use hpr::hpr_analysis::montecarlo::FlightInputs;
use hpr_io::ork;
use hpr_sim::FlightSettings;
use hpr_validate::apogee_ranges::{
    BOUNDS_CONFIDENCE, Coverage, FlightRanges, LEVELS, Place, RANK_BINS, SAMPLES, SEED_LABEL,
    SPREADS, design_file, fly, seed,
};
use serde::{Deserialize, Serialize};

use crate::fixture_flights::{
    CALIBRATION_HALF, CATALOG_TIER_A, CatalogTier, EVERY_FLIGHT, Entry, ForwardFlight,
    HELD_OUT_HALF, WITHOUT_EARLY,
};

/// The report, as data.
pub const REPORT_JSON: &str = "validation/reports/apogee-ranges.json";

/// The report, as a page.
pub const REPORT_MD: &str = "validation/reports/apogee-ranges.md";

/// How far a run's nominal flight may stand from the apogee the collection's report flew, as a
/// share of it: the two are the same flight, built twice, and a rerun of the collection moves an
/// apogee by at most 5 parts in 10⁷ (`docs/research/working-notes.md`).
const NOMINAL_SHARE: f64 = 1e-6;

/// One input's standard deviation, as the report keeps it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadRow {
    /// The input, in words.
    pub input: String,
    /// The standard deviation: a fraction of the input, or degrees.
    pub sigma: f64,
    /// `"fraction"` or `"degrees"`.
    pub unit: String,
    /// Where it is stated.
    pub from: String,
}

/// One set of flights and how often their ranges held.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Set {
    /// The set's name, one of the collection report's.
    pub name: String,
    /// Its counts.
    pub coverage: Coverage,
}

/// The committed report: counts and medians over sets of flights, no flight named.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// The milestone that wrote it.
    pub milestone: String,
    /// Its decision record.
    pub decision: String,
    /// The flights of each flight's run.
    pub samples: u64,
    /// What follows a flight's id in the bytes its run's seed is hashed from.
    pub seed_label: String,
    /// The confidence of each share's two-sided Clopper–Pearson interval.
    pub bounds_confidence: f64,
    /// The spreads every run flies.
    pub spreads: Vec<SpreadRow>,
    /// The sets: every flight compared, then the collection report's other four.
    pub sets: Vec<Set>,
    /// Why a run couldn't be set up, by the error's words, with the flights of each.
    pub runs_failed: BTreeMap<String, usize>,
}

/// One compared flight's run, with what sorts it into the sets.
struct Flown {
    held_out: bool,
    early: bool,
    below_tier_a: bool,
    ranges: Result<FlightRanges, String>,
}

/// The inputs of `flight` as the collection's report flew it ([`crate::fixture_flights`]'s
/// `simulation_of`): whatever HPR Sim's checks find in the design, the separations the climb
/// flies with each part tumbling, and nothing deployed.
fn inputs_of(flight: &ForwardFlight) -> Result<FlightInputs, String> {
    let mut inputs = FlightInputs::new(
        flight.design.rocket.clone(),
        flight.configuration.id.clone(),
        flight.day.clone(),
        flight.rail,
    );
    inputs.settings = FlightSettings {
        accept_design_errors: true,
        ..FlightSettings::default()
    };
    let simulation = inputs.simulation().map_err(|error| error.to_string())?;
    let stagings: Vec<ork::Staging> = flight.configuration.stagings().cloned().collect();
    let stagings = crate::ork_flights::climbing(&stagings, simulation.assembly())
        .map_err(|error| error.to_string())?;
    if !stagings.is_empty() {
        let (devices, separations) =
            crate::ork_flights::staged_parts(simulation.assembly(), stagings)
                .map_err(|error| error.to_string())?;
        inputs.recovery = devices;
        inputs.separations = separations;
    }
    Ok(inputs)
}

/// Flies one flight's run. An `Err` outside is a fault of this report (its nominal flight isn't
/// the collection report's); an `Err` inside is a run that couldn't be set up, a miss.
fn fly_one(id: &str, flight: &ForwardFlight) -> Result<Result<FlightRanges, String>, String> {
    let inputs = inputs_of(flight).map_err(|error| format!("{id}: {error}"))?;
    let nominal = inputs
        .fly()
        .map_err(|error| format!("{id}: {error}"))?
        .apogee
        .and_then(|apogee| apogee.gain_m)
        .ok_or_else(|| format!("{id}: the nominal flight has no apogee"))?;
    if (nominal - flight.hpr_m).abs() > NOMINAL_SHARE * flight.hpr_m.abs() {
        return Err(format!(
            "{id}: the run's nominal flight isn't the report's: {nominal} m against {} m",
            flight.hpr_m
        ));
    }
    Ok(
        fly(inputs, design_file(), seed(id), SAMPLES, flight.logged_m)
            .map_err(|error| error.to_string()),
    )
}

/// Flies every compared flight's run, the flights side by side.
fn fly_all(
    entries: &[Entry],
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
) -> Result<Vec<Flown>, String> {
    let tiers: BTreeMap<&str, CatalogTier> = entries
        .iter()
        .map(|entry| (entry.flight_id.as_str(), entry.catalog_tier))
        .collect();
    let flights: Vec<(&String, &ForwardFlight)> = forward
        .iter()
        .filter_map(|(id, flight)| flight.as_ref().ok().map(|flight| (id, flight)))
        .collect();
    let threads = std::thread::available_parallelism().map_or(4, usize::from);
    let next = AtomicUsize::new(0);
    let mut flown: Vec<(usize, Result<Result<FlightRanges, String>, String>)> =
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads.min(flights.len().max(1)))
                .map(|_| {
                    scope.spawn(|| {
                        let mut done = Vec::new();
                        loop {
                            let index = next.fetch_add(1, Ordering::Relaxed);
                            let Some((id, flight)) = flights.get(index) else {
                                return done;
                            };
                            done.push((index, fly_one(id, flight)));
                        }
                    })
                })
                .collect();
            workers
                .into_iter()
                .flat_map(|worker| worker.join().unwrap_or_default())
                .collect()
        });
    if flown.len() != flights.len() {
        return Err("a thread flying the runs panicked".to_owned());
    }
    flown.sort_by_key(|(index, _)| *index);
    flown
        .into_iter()
        .map(|(index, ranges)| {
            let (id, flight) = flights[index];
            let ranges = ranges?;
            match &ranges {
                Ok(run) => println!(
                    "apogee-ranges: {id:<48} log at {:>5.1}% of its run  50% {:<8} 90% {:<8} \
                     spread {:>5.2}%  failed {}",
                    100.0 * run.rank.unwrap_or(f64::NAN),
                    place_word(run.places[0]),
                    place_word(run.places[1]),
                    100.0 * run.spread.unwrap_or(f64::NAN),
                    run.failed
                ),
                Err(why) => println!("apogee-ranges: {id:<48} no run: {why}"),
            }
            Ok(Flown {
                held_out: crate::collection_boost::held_out(id),
                early: flight.early,
                below_tier_a: tiers
                    .get(id.as_str())
                    .is_none_or(|tier| *tier != CatalogTier::A),
                ranges,
            })
        })
        .collect()
}

/// A place, in a word, for the terminal.
fn place_word(place: Place) -> &'static str {
    match place {
        Place::Inside => "held",
        Place::Below => "below",
        Place::Above => "above",
        Place::NoRange => "no range",
    }
}

impl Report {
    /// The report of `flown`, the compared flights' runs.
    fn of(flown: &[Flown]) -> Self {
        let set = |name: &str, keep: &dyn Fn(&Flown) -> bool| Set {
            name: name.to_owned(),
            coverage: Coverage::of(
                &flown
                    .iter()
                    .filter(|flight| keep(flight))
                    .map(|flight| flight.ranges.as_ref().ok().copied())
                    .collect::<Vec<_>>(),
            ),
        };
        let mut runs_failed = BTreeMap::new();
        for why in flown
            .iter()
            .filter_map(|flight| flight.ranges.as_ref().err())
        {
            *runs_failed.entry(why.clone()).or_insert(0) += 1;
        }
        Self {
            milestone: "M2.8a".to_owned(),
            decision: "ADR-386".to_owned(),
            samples: SAMPLES,
            seed_label: SEED_LABEL.to_owned(),
            bounds_confidence: BOUNDS_CONFIDENCE,
            spreads: spreads(),
            sets: vec![
                set(EVERY_FLIGHT, &|_| true),
                set(WITHOUT_EARLY, &|flight| !flight.early),
                set(CATALOG_TIER_A, &|flight| !flight.below_tier_a),
                set(CALIBRATION_HALF, &|flight| !flight.held_out),
                set(HELD_OUT_HALF, &|flight| flight.held_out),
            ],
            runs_failed,
        }
    }

    /// The set named `name`.
    pub fn set(&self, name: &str) -> Option<&Coverage> {
        self.sets
            .iter()
            .find(|set| set.name == name)
            .map(|set| &set.coverage)
    }

    /// Holds the report to itself and to this code: the run's settings and spreads are the
    /// code's, each set's counts hold to one another, the halves add up to every flight
    /// compared, the runs that couldn't be set up are each under a reason, and the page is the
    /// data's.
    pub fn check_consistent(&self, page: &str) -> Result<(), String> {
        let fail = |why: String| Err(format!("{REPORT_JSON}: {why}"));
        if self.samples != SAMPLES
            || self.seed_label != SEED_LABEL
            || self.bounds_confidence != BOUNDS_CONFIDENCE
            || self.spreads != spreads()
        {
            return fail("the run's settings or spreads aren't the code's".to_owned());
        }
        let names = [
            EVERY_FLIGHT,
            WITHOUT_EARLY,
            CATALOG_TIER_A,
            CALIBRATION_HALF,
            HELD_OUT_HALF,
        ];
        if self.sets.len() != names.len()
            || self
                .sets
                .iter()
                .zip(names)
                .any(|(set, name)| set.name != name)
        {
            return fail("the sets aren't the collection report's five, in its order".to_owned());
        }
        for set in &self.sets {
            if let Err(why) = set.coverage.check() {
                return fail(format!("{}: {why}", set.name));
            }
            if set.coverage.samples
                != (set.coverage.flights - set.coverage.runs_failed) * SAMPLES as usize
            {
                return fail(format!(
                    "{}: a run of other than {SAMPLES} flights",
                    set.name
                ));
            }
        }
        let [every, without_early, tier_a, calibration, held_out] = &self.sets[..] else {
            return fail("the sets aren't five".to_owned());
        };
        let (every, calibration, held_out) =
            (&every.coverage, &calibration.coverage, &held_out.coverage);
        if calibration.flights + held_out.flights != every.flights
            || calibration.runs_failed + held_out.runs_failed != every.runs_failed
            || calibration.samples_failed + held_out.samples_failed != every.samples_failed
        {
            return fail("the two halves don't add up to every flight compared".to_owned());
        }
        for index in 0..LEVELS.len() {
            let (all, a, b) = (
                &every.levels[index],
                &calibration.levels[index],
                &held_out.levels[index],
            );
            if a.held + b.held != all.held
                || a.log_below + b.log_below != all.log_below
                || a.log_above + b.log_above != all.log_above
                || a.no_range + b.no_range != all.no_range
            {
                return fail(format!(
                    "the two halves' counts at {} don't add up to every flight's",
                    LEVELS[index]
                ));
            }
            for part in [&without_early.coverage, &tier_a.coverage] {
                if part.flights > every.flights || part.levels[index].held > all.held {
                    return fail("a part of the flights holds more than all of them".to_owned());
                }
            }
        }
        for (calibration, (held_out, every)) in calibration
            .ranks
            .iter()
            .zip(held_out.ranks.iter().zip(&every.ranks))
        {
            if calibration + held_out != *every {
                return fail("the two halves' tenths don't add up".to_owned());
            }
        }
        if self.runs_failed.values().sum::<usize>() != every.runs_failed {
            return fail("the runs that couldn't be set up aren't each under a reason".to_owned());
        }
        if page != self.to_markdown() {
            return Err(format!("{REPORT_MD} isn't the page of {REPORT_JSON}"));
        }
        Ok(())
    }

    /// The report as a page.
    pub fn to_markdown(&self) -> String {
        let mut page = String::new();
        let percent = |value: Option<f64>| match value {
            Some(value) => format!("{value:+.2}%"),
            None => "none".to_owned(),
        };
        let unsigned = |value: Option<f64>| match value {
            Some(value) => format!("{value:.2}%"),
            None => "none".to_owned(),
        };
        let every = self.set(EVERY_FLIGHT);
        let _ = writeln!(page, "# How often the apogee ranges hold the logs\n");
        let _ = writeln!(
            page,
            "Written by `cargo xtask fixture-flights`; do not edit ({}, decision record \
             [{}](../../docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md)). \
             A Monte Carlo run flies a design many times with its inputs scattered, and gives a \
             range of apogees. A 90% range says that nine real flights in ten end inside it. \
             This page counts whether they do. Each flight of the private collection that is \
             [compared with its log](fixture-flights.md) is flown {} times more, its inputs drawn \
             about the design file's values, and the logged apogee is placed against that run's \
             50% and 90% ranges. The collection's license allows only aggregates, so no flight \
             is named or numbered here.\n",
            self.milestone,
            self.decision,
            thousands(self.samples)
        );
        let _ = writeln!(
            page,
            "How far to trust it: the counts are of {} flights, {} of them held out, so each \
             share comes with wide bounds, and the flights are the ones fliers chose to publish \
             with a design file and a log. The spreads are stated ones, taken from published \
             figures before any count was made, and nothing was fitted to these flights.\n",
            every.map_or(0, |set| set.flights),
            self.set(HELD_OUT_HALF).map_or(0, |set| set.flights)
        );
        let _ = writeln!(page, "## The run\n");
        let _ = writeln!(
            page,
            "- **The flights:** each flight as the [collection's report](fixture-flights.md) \
             flies it: the design as read, in the day's ERA5 air and wind, from its rail, \
             nothing deployed. The run is the one `hpr mc` flies: every input an independent \
             normal draw about its value, one random stream a sample and input. A flight's seed \
             is the first eight bytes of the SHA-256 of its id followed by `{}`.\n\
             - **A range** at 50% runs from the run's 25% point to its 75% point, and at 90% \
             from its 5% point to its 95% point, of the height the center of mass climbed. The \
             logged apogee is that height too, as the collection's report reads it from the log.\n\
             - **A sample that fails** can't help a range hold: the low edge counts each failed \
             sample as above the run's apogees and the high edge as below them, the narrowest \
             range they could have left. A run with no range left, or one that can't be set up, \
             is a miss.\n\
             - **The bounds** on each share are the two-sided {:.0}% Clopper–Pearson interval: \
             the shares of all such flights that this many could have come from. A level \
             outside the bounds is a range these flights show to be wrong; a level inside them \
             is one they can't tell from right.\n",
            self.seed_label,
            100.0 * self.bounds_confidence
        );
        let _ = writeln!(
            page,
            "The standard deviation of each input drawn, the same for every flight:\n"
        );
        let _ = writeln!(page, "| input | standard deviation | where it is stated |");
        let _ = writeln!(page, "|---|---:|---|");
        for spread in &self.spreads {
            let sigma = if spread.unit == "degrees" {
                format!("{}°", spread.sigma)
            } else {
                format!("{}%", trimmed(100.0 * spread.sigma))
            };
            let _ = writeln!(page, "| {} | {sigma} | {} |", spread.input, spread.from);
        }
        let _ = writeln!(
            page,
            "\nLeft at the values flown, each of which would widen a range: the wind and the \
             air, the center of mass, and the altimeter's own scatter. The \
             [decision record](../../docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md) \
             sizes them.\n"
        );
        for (index, level) in LEVELS.into_iter().enumerate() {
            let _ = writeln!(page, "## The {:.0}% range\n", 100.0 * level);
            let _ = writeln!(
                page,
                "| flights | count | held | share | {:.0}% bounds | {:.0}% inside the bounds | \
                 log below the range | log above the range | no range |",
                100.0 * self.bounds_confidence,
                100.0 * level
            );
            let _ = writeln!(page, "|---|---:|---:|---:|---|---|---:|---:|---:|");
            for set in &self.sets {
                let count = &set.coverage.levels[index];
                let _ = writeln!(
                    page,
                    "| {} | {} | {} | {:.2}% | {:.2}% to {:.2}% | {} | {} | {} | {} |",
                    set.name,
                    set.coverage.flights,
                    count.held,
                    count.held_percent,
                    count.lower_percent,
                    count.upper_percent,
                    if count.level_inside { "yes" } else { "no" },
                    count.log_below,
                    count.log_above,
                    count.no_range
                );
            }
            let _ = writeln!(page);
        }
        let _ = writeln!(
            page,
            "A log below its range is a flight the run flew higher than; a log above it, one \
             the run kept lower than.\n"
        );
        let _ = writeln!(page, "## Where the logs fall in their runs\n");
        let _ = writeln!(
            page,
            "Each flight is counted in the tenth of its run its logged apogee falls in: the \
             first column counts the logs below the lowest tenth of their run's apogees, the \
             last those above the highest. Runs whose ranges hold as often as they say fill the \
             ten evenly. The offset is the median, over the flights, of the logged apogee less \
             the run's median, as a share of that median.\n"
        );
        let tenths: Vec<String> = (1..=RANK_BINS).map(|tenth| tenth.to_string()).collect();
        let _ = writeln!(page, "| flights | median offset | {} |", tenths.join(" | "));
        let _ = writeln!(page, "|---|---:|{}", "---:|".repeat(RANK_BINS));
        for set in &self.sets {
            let ranks: Vec<String> = set.coverage.ranks.iter().map(usize::to_string).collect();
            let _ = writeln!(
                page,
                "| {} | {} | {} |",
                set.name,
                percent(set.coverage.median_offset_percent),
                ranks.join(" | ")
            );
        }
        let _ = writeln!(page, "\n## How wide the runs are\n");
        let _ = writeln!(
            page,
            "Each width and standard deviation is a share of its run's median apogee; the \
             medians are over the set's flights.\n"
        );
        let _ = writeln!(
            page,
            "| flights | median width of the 50% range | median width of the 90% range | median \
             standard deviation | least | most |"
        );
        let _ = writeln!(page, "|---|---:|---:|---:|---:|---:|");
        for set in &self.sets {
            let coverage = &set.coverage;
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} | {} | {} |",
                set.name,
                unsigned(coverage.median_width_percent[0]),
                unsigned(coverage.median_width_percent[1]),
                unsigned(coverage.median_spread_percent),
                unsigned(coverage.least_spread_percent),
                unsigned(coverage.most_spread_percent)
            );
        }
        let _ = writeln!(page, "\n## Samples that failed\n");
        let _ = writeln!(
            page,
            "| flights | samples | failed | flights with a failed sample | runs that couldn't \
             be set up |"
        );
        let _ = writeln!(page, "|---|---:|---:|---:|---:|");
        for set in &self.sets {
            let coverage = &set.coverage;
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} | {} |",
                set.name,
                thousands(coverage.samples as u64),
                coverage.samples_failed,
                coverage.flights_with_a_failed_sample,
                coverage.runs_failed
            );
        }
        if !self.runs_failed.is_empty() {
            let _ = writeln!(page, "\nWhy a run couldn't be set up:\n");
            for (why, flights) in &self.runs_failed {
                let _ = writeln!(page, "- {why}: {flights}");
            }
        }
        page
    }
}

/// The code's spreads as the report keeps them.
fn spreads() -> Vec<SpreadRow> {
    SPREADS
        .iter()
        .map(|spread| SpreadRow {
            input: spread.input.to_owned(),
            sigma: spread.sigma,
            unit: spread.unit.to_owned(),
            from: spread.from.to_owned(),
        })
        .collect()
}

/// `count` with a comma between its thousands.
fn thousands(count: u64) -> String {
    let digits = count.to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// A percentage with no trailing zeros: 10.75, 16.3, 3.56, 9.2.
fn trimmed(percent: f64) -> String {
    let text = format!("{percent:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

/// Flies the runs and writes, or with `check` checks, the report.
pub fn run(
    root: &Path,
    entries: &[Entry],
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
    check: bool,
) -> Result<(), String> {
    let started = Instant::now();
    let flown = fly_all(entries, forward)?;
    let report = Report::of(&flown);
    let page = report.to_markdown();
    report.check_consistent(&page)?;
    for set in &report.sets {
        let [half, ninety] = &set.coverage.levels[..] else {
            continue;
        };
        println!(
            "apogee-ranges: {:<56} {:>2} flights  50% held {:>2} ({:.2}%)  90% held {:>2} ({:.2}%)",
            set.name,
            set.coverage.flights,
            half.held,
            half.held_percent,
            ninety.held,
            ninety.held_percent
        );
    }
    println!(
        "apogee-ranges: {} runs of {SAMPLES} flights in {:.0} s",
        flown.len(),
        started.elapsed().as_secs_f64()
    );
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let stored: Report = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        if stored != report {
            return Err(format!(
                "{REPORT_JSON} does not reproduce; run `cargo xtask fixture-flights`"
            ));
        }
        let committed = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        stored.check_consistent(&committed)?;
        println!("fixture-flights: the apogee ranges' counts reproduce");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, page)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("fixture-flights: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed report holds to itself and to the code, here and in CI, where the flights
    /// can't be flown; and its sets are the sizes of the collection report's, which flew the
    /// same flights.
    #[test]
    fn the_committed_report_holds_to_itself() {
        let root = crate::designs::root().unwrap();
        let report: Report =
            serde_json::from_str(&fs::read_to_string(root.join(REPORT_JSON)).unwrap()).unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        report.check_consistent(&page).unwrap();
        let collection: crate::fixture_flights::Report = serde_json::from_str(
            &fs::read_to_string(root.join(crate::fixture_flights::REPORT_JSON)).unwrap(),
        )
        .unwrap();
        assert_eq!(report.sets.len(), collection.sets.len());
        for (set, errors) in report.sets.iter().zip(&collection.sets) {
            assert_eq!(set.name, errors.name);
            assert_eq!(set.coverage.flights, errors.flights, "{}", set.name);
        }
    }

    /// A page that isn't the data's, a set out of place, a half that doesn't add up, a run of
    /// another length and a spread that isn't the code's are each refused.
    #[test]
    fn a_spoiled_report_is_refused() {
        let ranges = |place: Place, failed: usize| FlightRanges {
            attempted: SAMPLES as usize,
            failed,
            places: [place, Place::Inside],
            rank: Some(0.4),
            offset: Some(-0.03),
            widths: [Some(0.1), Some(0.3)],
            spread: Some(0.09),
        };
        let flown: Vec<Flown> = (0..9)
            .map(|k| Flown {
                held_out: k % 3 == 0,
                early: k % 4 == 0,
                below_tier_a: k == 5,
                ranges: if k == 8 {
                    Err("a motor this can't disperse".to_owned())
                } else {
                    Ok(ranges(
                        if k % 2 == 0 {
                            Place::Inside
                        } else {
                            Place::Below
                        },
                        usize::from(k == 1),
                    ))
                },
            })
            .collect();
        let report = Report::of(&flown);
        let page = report.to_markdown();
        report.check_consistent(&page).unwrap();
        let every = report.set(EVERY_FLIGHT).unwrap();
        assert_eq!((every.flights, every.runs_failed), (9, 1));
        // Four of the eight that ran held at 50% (k = 0, 2, 4, 6); the ninth is a miss.
        assert_eq!(every.levels[0].held, 4);
        assert_eq!(every.levels[0].log_below, 4);
        assert_eq!(every.levels[0].no_range, 1);
        assert_eq!(every.levels[1].held, 8);
        assert_eq!(report.set(HELD_OUT_HALF).unwrap().flights, 3);
        assert_eq!(report.set(WITHOUT_EARLY).unwrap().flights, 6);
        assert_eq!(report.set(CATALOG_TIER_A).unwrap().flights, 8);
        assert_eq!(report.runs_failed.values().sum::<usize>(), 1);
        assert!(
            page.contains("| every flight compared | 9 | 4 | 44.44% | 13.70% to 78.80% | yes |")
        );
        assert!(page.contains("- a motor this can't disperse: 1"));

        let refused = |spoil: &dyn Fn(&mut Report), words: &str| {
            let mut spoiled = report.clone();
            spoil(&mut spoiled);
            let page = spoiled.to_markdown();
            let why = spoiled.check_consistent(&page).unwrap_err();
            assert!(why.contains(words), "{why}");
        };
        assert!(
            report
                .check_consistent(&page.replace("44.44%", "55.56%"))
                .unwrap_err()
                .contains("isn't the page")
        );
        refused(&|report| report.sets.swap(3, 4), "in its order");
        refused(&|report| report.samples = 500, "settings or spreads");
        refused(
            &|report| report.spreads[1].sigma = 0.10,
            "settings or spreads",
        );
        refused(&|report| report.spreads.truncate(5), "settings or spreads");
        refused(
            &|report| report.sets[3].coverage = report.sets[0].coverage.clone(),
            "don't add up",
        );
        refused(
            &|report| report.sets[0].coverage.samples -= 1,
            "a run of other than",
        );
        refused(&|report| report.runs_failed.clear(), "under a reason");
    }

    /// Counts and percentages read as the page writes them.
    #[test]
    fn numbers_are_written_plainly() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(thousands(60_000), "60,000");
        assert_eq!(thousands(1_234_567), "1,234,567");
        assert_eq!(trimmed(10.75), "10.75");
        assert_eq!(trimmed(16.3), "16.3");
        assert_eq!(trimmed(3.56), "3.56");
        assert_eq!(trimmed(9.2), "9.2");
        assert_eq!(trimmed(20.0), "20");
    }
}
