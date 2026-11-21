//! `cargo xtask tip-off-flights`: what the tip-off phase moves on the real flights HPR Sim
//! compares (M1.14b14, ADR-363 decision 1).
//!
//! - **The question.** HPR Sim's tip-off phase ([`hpr_sim::Simulation::with_tip_off`]) is held
//!   code to code against RocketPy's (`validation/reports/tip-off-model.md`), and is opt-in.
//!   Before it becomes the default (M1.14b15), this measures what it does to the real flights'
//!   apogee errors.
//! - **The flights.** Three sets, each flown as its own report flies it, once guided to the last
//!   guide as today and once with the phase ([`hpr_validate::correction::Variant`]): the seven
//!   flights the real-flight report compared as soon as they were flown, the five it compares
//!   after their predictions were merged, and the private collection's logged flights. The
//!   collection's flights are flown by `cargo xtask fixture-flights`, which writes only their
//!   aggregate ([`CollectionTipOff`]) into its own report (ADR-151 decision 2, ADR-184 decision
//!   7); this command reads that aggregate from the committed report.
//! - **The errors.** Each set's error is its own report's: the real-flight report's apogee as the
//!   log's altimeter would read it, against the log's apogee; the collection's apogee as a
//!   height climbed in the day's air, against the logged apogee read the same way.
//! - **A landing point** is compared for none of these flights, so no landing move is given.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_validate::correction::Variant;
use hpr_validate::real_flight::{self, FLIGHTS, RealFlightReport, Varied};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "\
  tip-off-flights [--check]
                           Fly each public real flight the real-flight report compares guided
                           to its last guide and with the tip-off phase, read the private
                           collection's aggregate from validation/reports/fixture-flights.json,
                           and write validation/reports/tip-off-flights.json and .md. Needs the
                           pinned checkouts under refs/. --check fails if the committed report is
                           not this run's instead of writing.";

pub const REPORT: &str = "validation/reports/tip-off-flights.json";
pub const REPORT_MD: &str = "validation/reports/tip-off-flights.md";
/// How far a guided flight may land from the apogee its committed report gives, m: the same
/// flight on the same code, so any distance past rounding means the report is stale.
pub const GUIDED_WITHIN_M: f64 = 0.001;
/// The width of a bin of the phase's apogee move, per cent of the guided apogee (ADR-363).
pub const BIN_PERCENT: f64 = 0.1;
/// The fewest flights a published bin may hold; a bin with fewer is merged with its neighbor
/// (ADR-363, after ADR-346 §7's rule against values given by subtraction).
pub const BIN_FLOOR: usize = 5;
/// The set of flights the real-flight report compared as soon as they were flown.
pub const COMPARED_FIRST: &str = "compared first";
/// The set it compares after their predictions were merged.
pub const PREDICTED_FIRST: &str = "predicted first";
/// The private collection's logged flights.
pub const COLLECTION: &str = "the private collection";

/// One flight of the collection, kept on this machine: never written.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CollectionFlight {
    /// The SHA-256 of the design file's bytes, to count distinct designs; never written.
    pub design: String,
    /// Whether the design has two or more guides at different stations.
    pub two_guides: bool,
    /// The apogee flown guided, m climbed.
    pub guided_m: f64,
    /// The apogee flown with the phase, m climbed.
    pub tip_off_m: f64,
    /// The apogee flown guided in still air, m climbed: the day's air with its wind taken out.
    pub still_guided_m: f64,
    /// The same with the phase.
    pub still_tip_off_m: f64,
    /// The logged apogee as a height climbed, m.
    pub logged_m: f64,
}

/// A mean absolute error and the signed mean beside it, per cent.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    /// The mean of the absolute apogee errors, per cent.
    pub mean_absolute_percent: f64,
    /// The mean of the apogee errors with their signs, per cent.
    pub mean_percent: f64,
}

impl Errors {
    /// The two means of `errors`, per cent, folded from zero; `None` for no errors.
    pub fn of(errors: &[f64]) -> Option<Self> {
        if errors.is_empty() {
            return None;
        }
        let n = errors.len() as f64;
        Some(Self {
            mean_absolute_percent: errors.iter().fold(0.0, |sum, e| sum + e.abs()) / n,
            mean_percent: errors.iter().fold(0.0, |sum, e| sum + e) / n,
        })
    }
}

/// A bin of the phase's apogee moves: the flights whose move, per cent of the guided apogee,
/// lies in `[from_percent, to_percent)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bin {
    pub from_percent: f64,
    pub to_percent: f64,
    pub flights: usize,
}

/// The phase's apogee moves counted in bins [`BIN_PERCENT`] wide, each starting at a multiple of
/// it, then merged until every bin holds at least [`BIN_FLOOR`] flights: the bin with the fewest
/// (the lowest of a tie) is merged with the smaller of its neighbors (the lower of a tie), and so
/// on. Only bins holding a flight are bins, so a merged bin spans any empty ones between its
/// parts. All the moves in one bin when there are fewer than [`BIN_FLOOR`] of them; none for no
/// moves.
pub fn bins(moves_percent: &[f64]) -> Vec<Bin> {
    let mut counts: std::collections::BTreeMap<i64, usize> = std::collections::BTreeMap::new();
    for &m in moves_percent {
        // Each move's bin by its multiple of the width, times the bins per per cent (10, exact)
        // rather than over the width: 0.3 / 0.1 is just under 3, so a move of exactly 0.3% would
        // fall a bin low. The moves are fractions of a per cent, so the index fits an i64.
        *counts
            .entry((m * (1.0 / BIN_PERCENT)).floor() as i64)
            .or_default() += 1;
    }
    // (first index, last index, flights), in order.
    let mut merged: Vec<(i64, i64, usize)> = counts.into_iter().map(|(i, n)| (i, i, n)).collect();
    while merged.len() > 1 {
        let Some(short) = (0..merged.len())
            .filter(|&k| merged[k].2 < BIN_FLOOR)
            .min_by_key(|&k| (merged[k].2, k))
        else {
            break;
        };
        let below = short.checked_sub(1);
        let above = (short + 1 < merged.len()).then_some(short + 1);
        let other = match (below, above) {
            (Some(b), Some(a)) if merged[a].2 < merged[b].2 => a,
            (Some(b), _) => b,
            (None, Some(a)) => a,
            // More than one bin, so a bin has a neighbor.
            (None, None) => break,
        };
        let (low, high) = (short.min(other), short.max(other));
        merged[low] = (
            merged[low].0,
            merged[high].1,
            merged[low].2 + merged[high].2,
        );
        merged.remove(high);
    }
    merged
        .into_iter()
        .map(|(from, to, flights)| Bin {
            from_percent: from as f64 * BIN_PERCENT,
            to_percent: (to + 1) as f64 * BIN_PERCENT,
            flights,
        })
        .collect()
}

/// One flight's figures the [`Split`] is made from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Moved {
    /// Whether the design has two or more guides at different stations.
    pub two_guides: bool,
    /// The apogee error guided, per cent.
    pub guided_error_percent: f64,
    /// The same with the phase.
    pub tip_off_error_percent: f64,
    /// The phase's apogee move in the report's wind, m.
    pub move_m: f64,
    /// The same in still air, m: gravity's part alone.
    pub still_move_m: f64,
}

/// Where a set's change in mean absolute error comes from: which way the phase moved each
/// flight's apogee, in the wind and in still air, and whether that took it toward the log or
/// away from it. Each flight with two guides counts once in each pair; a move of exactly zero
/// in neither.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Split {
    /// Flights whose apogee the phase raised, in the report's wind.
    pub moved_up: usize,
    /// Flights whose apogee it lowered.
    pub moved_down: usize,
    /// Flights whose apogee it raised in still air.
    pub still_air_up: usize,
    /// Flights whose apogee it lowered in still air.
    pub still_air_down: usize,
    /// Flights whose absolute error the phase raised: moved away from the log.
    pub away: usize,
    /// Flights whose absolute error it lowered: moved toward the log.
    pub toward: usize,
    /// The flights moved away's part of the change in the set's mean absolute error, percentage
    /// points: their rises in absolute error summed, over every flight of the set. `None` where
    /// fewer than [`BIN_FLOOR`] such flights would give a few flights' own figures (the private
    /// collection only).
    pub away_points: Option<f64>,
    /// The flights moved toward's part, points (zero or less), held back as `away_points` is.
    pub toward_points: Option<f64>,
}

impl Split {
    /// The split of a set of `moved` flights. With `floor`, a part made of fewer flights than
    /// [`BIN_FLOOR`] is held back.
    pub fn of(moved: &[Moved], floor: bool) -> Self {
        let n = moved.len() as f64;
        let two: Vec<&Moved> = moved.iter().filter(|m| m.two_guides).collect();
        let count = |pick: &dyn Fn(&Moved) -> bool| two.iter().filter(|m| pick(m)).count();
        let rise = |m: &Moved| m.tip_off_error_percent.abs() - m.guided_error_percent.abs();
        let part = |away: bool| {
            let members: Vec<f64> = two
                .iter()
                .map(|m| rise(m))
                .filter(|r| if away { *r > 0.0 } else { *r < 0.0 })
                .collect();
            (!(floor && members.len() < BIN_FLOOR) && n > 0.0)
                .then(|| members.iter().fold(0.0, |sum, r| sum + r) / n)
        };
        Self {
            moved_up: count(&|m| m.move_m > 0.0),
            moved_down: count(&|m| m.move_m < 0.0),
            still_air_up: count(&|m| m.still_move_m > 0.0),
            still_air_down: count(&|m| m.still_move_m < 0.0),
            away: count(&|m| rise(m) > 0.0),
            toward: count(&|m| rise(m) < 0.0),
            away_points: part(true),
            toward_points: part(false),
        }
    }
}

impl Split {
    /// The split in words, for a set with `two_guides` flights that have two guides.
    pub fn sentence(&self, two_guides: usize) -> String {
        let points = |p: Option<f64>| {
            p.map_or_else(
                || "held back: fewer than five flights".to_owned(),
                |p| format!("{p:+.3} points"),
            )
        };
        format!(
            "Of the {two_guides} flights with two guides, the phase raised the apogee of {} and \
             lowered it on {} in the wind; in still air it raised {} and lowered {}. It moved {} \
             away from the log and {} toward it. The change in the mean absolute error is the \
             flights moved away's part ({}) plus the flights moved toward's ({}).\n\n",
            self.moved_up,
            self.moved_down,
            self.still_air_up,
            self.still_air_down,
            self.away,
            self.toward,
            points(self.away_points),
            points(self.toward_points)
        )
    }
}

/// A table of the phase's apogee moves by bin, flown `how`.
fn bins_table(how: &str, bins: &[Bin]) -> String {
    let mut md = format!(
        "The flights with two guides by the phase's apogee move {how}, in bins of 0.1% of the \
         apogee guided, a bin holding fewer than five merged with its smaller neighbor:\n\n| move \
         | flights |\n|---|---:|\n"
    );
    for bin in bins {
        let _ = writeln!(
            md,
            "| {:+.1}% to {:+.1}% | {} |",
            bin.from_percent, bin.to_percent, bin.flights
        );
    }
    md.push('\n');
    md
}

/// What the phase moves on the private collection's logged flights, in aggregate only: no
/// flight named or numbered, no apogee, no row of one flight (ADR-363 decision 1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionTipOff {
    /// The flights compared with their logs.
    pub flights: usize,
    /// Of those, the flights whose design has two or more guides at different stations.
    pub flights_with_two_guides: usize,
    /// The designs those flights fly: distinct design files, by their bytes.
    pub designs: usize,
    /// Of those, the designs with two or more guides at different stations.
    pub designs_with_two_guides: usize,
    /// The flights with two guides, by the phase's apogee move, per cent of the guided apogee.
    pub moves: Vec<Bin>,
    /// The same in still air, per cent of the guided apogee in still air.
    pub still_air_moves: Vec<Bin>,
    /// Where the change in the mean absolute error comes from.
    pub split: Split,
    /// The errors over every flight compared, flown guided.
    pub guided: Option<Errors>,
    /// The same with the phase.
    pub tip_off: Option<Errors>,
}

impl CollectionTipOff {
    /// The aggregate of `flights`.
    ///
    /// # Errors
    ///
    /// A flight with fewer than two guides whose apogee the phase moved: such a design has no
    /// stretch for the phase to fly.
    pub(crate) fn of(flights: &[CollectionFlight]) -> Result<Self, String> {
        if flights.iter().any(|f| {
            !f.two_guides
                && (f.tip_off_m.to_bits() != f.guided_m.to_bits()
                    || f.still_tip_off_m.to_bits() != f.still_guided_m.to_bits())
        }) {
            return Err(
                "a collection flight with fewer than two guides moved with the tip-off phase"
                    .to_owned(),
            );
        }
        let designs: BTreeSet<&str> = flights.iter().map(|f| f.design.as_str()).collect();
        let designs_two: BTreeSet<&str> = flights
            .iter()
            .filter(|f| f.two_guides)
            .map(|f| f.design.as_str())
            .collect();
        let percent =
            |apogee_m: f64, f: &CollectionFlight| 100.0 * (apogee_m - f.logged_m) / f.logged_m;
        let guided: Vec<f64> = flights.iter().map(|f| percent(f.guided_m, f)).collect();
        let tip_off: Vec<f64> = flights.iter().map(|f| percent(f.tip_off_m, f)).collect();
        let moves: Vec<f64> = flights
            .iter()
            .filter(|f| f.two_guides)
            .map(|f| 100.0 * (f.tip_off_m - f.guided_m) / f.guided_m)
            .collect();
        let still_moves: Vec<f64> = flights
            .iter()
            .filter(|f| f.two_guides)
            .map(|f| 100.0 * (f.still_tip_off_m - f.still_guided_m) / f.still_guided_m)
            .collect();
        let moved: Vec<Moved> = flights
            .iter()
            .zip(guided.iter().zip(&tip_off))
            .map(|(f, (&g, &t))| Moved {
                two_guides: f.two_guides,
                guided_error_percent: g,
                tip_off_error_percent: t,
                move_m: f.tip_off_m - f.guided_m,
                still_move_m: f.still_tip_off_m - f.still_guided_m,
            })
            .collect();
        Ok(Self {
            flights: flights.len(),
            flights_with_two_guides: moves.len(),
            designs: designs.len(),
            designs_with_two_guides: designs_two.len(),
            moves: bins(&moves),
            still_air_moves: bins(&still_moves),
            split: Split::of(&moved, true),
            guided: Errors::of(&guided),
            tip_off: Errors::of(&tip_off),
        })
    }

    /// Its section of the collection's report.
    pub(crate) fn markdown(&self) -> String {
        let mut md = String::from(
            "\n## The tip-off phase (M1.14b14)\n\nEach flight compared is flown again with the \
             tip-off phase: once the guide ahead of the aft-most one leaves the rail, the rocket \
             pivots about its last guide until that one leaves too. A design with fewer than two \
             guides at different stations has no such stretch and flies as it did. The move is \
             the apogee with the phase less the apogee guided, over the apogee guided. The \
             public flights' table and what it means are in [the tip-off flights' \
             report](tip-off-flights.md).\n\n",
        );
        let _ = writeln!(
            md,
            "| | compared | with two or more guides at different stations |\n|---|---:|---:|\n| \
             flights | {} | {} |\n| designs | {} | {} |\n",
            self.flights, self.flights_with_two_guides, self.designs, self.designs_with_two_guides
        );
        if self.moves.is_empty() {
            md.push_str("No flight has two guides, so the phase moves none.\n\n");
        } else {
            md.push_str(&bins_table("in the day's wind", &self.moves));
            md.push_str(&bins_table(
                "in still air (the day's air with its wind taken out, so gravity's part alone)",
                &self.still_air_moves,
            ));
        }
        md.push_str(&self.split.sentence(self.flights_with_two_guides));
        if let (Some(guided), Some(tip_off)) = (self.guided, self.tip_off) {
            let _ = writeln!(
                md,
                "Over every flight compared, guided, the mean absolute apogee error is {:.3}% \
                 and the mean {:+.3}%; with the phase, {:.3}% and {:+.3}%.",
                guided.mean_absolute_percent,
                guided.mean_percent,
                tip_off.mean_absolute_percent,
                tip_off.mean_percent
            );
        }
        md
    }
}

/// One public flight of the table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightRow {
    pub id: String,
    pub title: String,
    /// [`COMPARED_FIRST`] or [`PREDICTED_FIRST`].
    pub set: String,
    /// Whether the design has two or more guides at different stations.
    pub two_guides: bool,
    /// The log's apogee, m.
    pub log_apogee_m: f64,
    /// HPR Sim's apogee guided, as its report reads it, m.
    pub guided_apogee_m: f64,
    /// The same with the phase, m.
    pub tip_off_apogee_m: f64,
    /// The phase's apogee move, m.
    pub move_m: f64,
    /// The same, per cent of the guided apogee.
    pub move_percent: f64,
    /// The apogee error guided, per cent of the log's apogee.
    pub guided_error_percent: f64,
    /// The same with the phase.
    pub tip_off_error_percent: f64,
    /// HPR Sim's apogee guided in still air (the report's air, its wind taken out), m.
    pub still_guided_apogee_m: f64,
    /// The same with the phase, m.
    pub still_tip_off_apogee_m: f64,
    /// The phase's apogee move in still air, m: gravity's part alone.
    pub still_move_m: f64,
}

impl FlightRow {
    /// Its figures for the [`Split`].
    pub fn moved(&self) -> Moved {
        Moved {
            two_guides: self.two_guides,
            guided_error_percent: self.guided_error_percent,
            tip_off_error_percent: self.tip_off_error_percent,
            move_m: self.move_m,
            still_move_m: self.still_move_m,
        }
    }
}

/// One set's errors, guided and with the phase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRow {
    pub set: String,
    pub flights: usize,
    pub flights_with_two_guides: usize,
    pub guided: Errors,
    pub tip_off: Errors,
    /// Where the change in the mean absolute error comes from.
    pub split: Split,
}

impl SetRow {
    /// Whether the mean absolute error is higher with the phase.
    pub fn higher_with_the_phase(&self) -> bool {
        self.tip_off.mean_absolute_percent > self.guided.mean_absolute_percent
    }
}

/// The committed report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub generated_by: String,
    pub milestone: String,
    pub decision: String,
    /// The public flights, compared first then predicted first, each in its report's order.
    pub flights: Vec<FlightRow>,
    /// The private collection's aggregate, as the collection's report gives it.
    pub collection: CollectionTipOff,
    /// Each set's errors: the two public sets, then the collection.
    pub sets: Vec<SetRow>,
}

impl Report {
    /// Builds the report from the public flights and the collection's aggregate.
    ///
    /// # Errors
    ///
    /// A set with no flights.
    pub fn of(flights: Vec<FlightRow>, collection: CollectionTipOff) -> Result<Self, String> {
        let mut sets = Vec::new();
        for set in [COMPARED_FIRST, PREDICTED_FIRST] {
            let rows: Vec<&FlightRow> = flights.iter().filter(|f| f.set == set).collect();
            let errors = |pick: fn(&FlightRow) -> f64| {
                Errors::of(&rows.iter().map(|r| pick(r)).collect::<Vec<_>>())
                    .ok_or_else(|| format!("the set {set} has no flights"))
            };
            sets.push(SetRow {
                set: set.to_owned(),
                flights: rows.len(),
                flights_with_two_guides: rows.iter().filter(|r| r.two_guides).count(),
                guided: errors(|r| r.guided_error_percent)?,
                tip_off: errors(|r| r.tip_off_error_percent)?,
                split: Split::of(&rows.iter().map(|r| r.moved()).collect::<Vec<_>>(), false),
            });
        }
        let (Some(guided), Some(tip_off)) = (collection.guided, collection.tip_off) else {
            return Err("the collection's aggregate has no flights".to_owned());
        };
        sets.push(SetRow {
            set: COLLECTION.to_owned(),
            flights: collection.flights,
            flights_with_two_guides: collection.flights_with_two_guides,
            guided,
            tip_off,
            split: collection.split,
        });
        Ok(Self {
            generated_by: "cargo xtask tip-off-flights".to_owned(),
            milestone: "M1.14b14".to_owned(),
            decision: "ADR-363".to_owned(),
            flights,
            collection,
            sets,
        })
    }

    /// The report's page.
    pub fn markdown(&self) -> String {
        let mut md = String::from(
            "# What the tip-off phase moves on the real flights\n\nWritten by `cargo xtask \
             tip-off-flights` ([M1.14b14][m1-14b14], decision [ADR-363][adr-363]); do not edit. \
             HPR Sim's tip-off phase lets the rocket pivot about its last guide once the guide \
             ahead of it has left the rail, instead of guiding it to that last guide ([the \
             phase][flight]; held to RocketPy's in [the comparison](tip-off-model.md)). It is \
             off by default. This report flies every real flight HPR Sim compares twice, guided \
             as today and with the phase, before the phase becomes the default.\n\n- **What is \
             flown:** each flight as its own report flies it, the phase the only change: the \
             seven flights [the real-flight report](real-flights.md) compared as soon as they \
             were flown, the five it compares after their predictions were merged ([the \
             predictions](public-predictions.md)), and the private collection's logged flights \
             ([its report](fixture-flights.md#the-tip-off-phase-m114b14)), given in aggregate \
             only.\n- **Two guides:** a design with fewer than two guides (rail buttons or \
             launch lugs) at different stations has no stretch on its last guide alone, so the \
             phase moves nothing; such a flight is listed as not moved.\n- **The errors** are \
             each set's own report's: for the public flights, HPR Sim's apogee as the log's \
             altimeter would read it, less the log's apogee, over the log's apogee; for the \
             collection, the apogee as a height climbed in the day's air, against the logged \
             apogee read the same way.\n- **The move** is the apogee with the phase less the \
             apogee guided; its share is of the apogee guided.\n- **Landing points:** none of \
             these flights has a landing point compared (the reports compare apogees; landing \
             points against tracked landings are [M2.13][m2-13]), so no landing move is \
             given.\n\n[m1-14b14]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b14\n\
             [adr-363]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md\n\
             [flight]: https://hpr.fusionspace.co/physics/flight.html#what-leaving-tip-off-out-costs\n\
             [m2-13]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-13\n\n## The \
             errors, guided and with the phase\n\n| set | flights | with two guides | mean \
             absolute error, guided | with the phase | mean error, guided | with the phase | \
             higher with the phase |\n|---|---:|---:|---:|---:|---:|---:|---|\n",
        );
        for set in &self.sets {
            let _ = writeln!(
                md,
                "| {} | {} | {} | {:.3}% | {:.3}% | {:+.3}% | {:+.3}% | {} |",
                set.set,
                set.flights,
                set.flights_with_two_guides,
                set.guided.mean_absolute_percent,
                set.tip_off.mean_absolute_percent,
                set.guided.mean_percent,
                set.tip_off.mean_percent,
                if set.higher_with_the_phase() {
                    "yes"
                } else {
                    "no"
                }
            );
        }
        let higher: Vec<&str> = self
            .sets
            .iter()
            .filter(|s| s.higher_with_the_phase())
            .map(|s| s.set.as_str())
            .collect();
        md.push('\n');
        if higher.is_empty() {
            md.push_str("No set's mean absolute apogee error is higher with the phase.\n\n");
        } else {
            let _ = writeln!(
                md,
                "The mean absolute apogee error is higher with the phase over: {}. ADR-363 asks \
                 for a decision record that measures the rise and its cause before the phase \
                 becomes the default.\n",
                higher.join(", ")
            );
        }
        md.push_str(
            "## Where the change comes from\n\nFor each set: which way the phase moved the \
             apogees of the flights with two guides, in the wind each report flies and in still \
             air (the same air with its wind taken out, so that what moves is gravity's turn \
             about the last guide alone), and whether the move took the apogee toward the log or \
             away from it. The change in a set's mean absolute error is the sum of two parts: \
             the rises in absolute error of the flights moved away, and the falls of those moved \
             toward, each summed and divided by the set's flights.\n\n| set | with two guides \
             | raised | lowered | raised in still air | lowered in still air | moved away | \
             moved toward | away's part | toward's part |\n|---|---:|---:|---:|---:|---:|---:|\
             ---:|---:|---:|\n",
        );
        for set in &self.sets {
            let points =
                |p: Option<f64>| p.map_or_else(|| "held back".to_owned(), |p| format!("{p:+.3}"));
            let t = &set.split;
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                set.set,
                set.flights_with_two_guides,
                t.moved_up,
                t.moved_down,
                t.still_air_up,
                t.still_air_down,
                t.away,
                t.toward,
                points(t.away_points),
                points(t.toward_points)
            );
        }
        md.push_str(
            "\nThe parts are percentage points of the mean absolute error. The collection's part \
             of fewer than five flights is held back, so as not to give a few flights' own \
             figures.\n\n",
        );
        md.push_str(
            "## Each public flight\n\n| flight | set | two guides | log apogee (m) | apogee \
             guided (m) | with the phase (m) | move (m) | move | move in still air (m) | error \
             guided | with the phase |\n|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        );
        for f in &self.flights {
            if f.two_guides {
                let _ = writeln!(
                    md,
                    "| {} | {} | yes | {:.1} | {:.2} | {:.2} | {:+.2} | {:+.3}% | {:+.2} | {:+.3}% \
                     | {:+.3}% |",
                    f.title,
                    f.set,
                    f.log_apogee_m,
                    f.guided_apogee_m,
                    f.tip_off_apogee_m,
                    f.move_m,
                    f.move_percent,
                    f.still_move_m,
                    f.guided_error_percent,
                    f.tip_off_error_percent
                );
            } else {
                let _ = writeln!(
                    md,
                    "| {} | {} | no | {:.1} | {:.2} | not moved | | | | {:+.3}% | {:+.3}% |",
                    f.title,
                    f.set,
                    f.log_apogee_m,
                    f.guided_apogee_m,
                    f.guided_error_percent,
                    f.tip_off_error_percent
                );
            }
        }
        md.push_str("\n## The private collection\n\n");
        let c = &self.collection;
        let _ = writeln!(
            md,
            "Of the {} flights compared with their logs, {} fly a design with two or more guides \
             at different stations: {} of the {} designs. No flight is named or numbered here \
             and no apogee of one is given (decision records ADR-151 and ADR-184).\n",
            c.flights, c.flights_with_two_guides, c.designs_with_two_guides, c.designs
        );
        if !c.moves.is_empty() {
            md.push_str(&bins_table("in the day's wind", &c.moves));
            md.push_str(&bins_table(
                "in still air (the day's air with its wind taken out, so gravity's part alone)",
                &c.still_air_moves,
            ));
        }
        md
    }
}

/// The four flights of each public flight, in this order: guided and with the phase in the
/// report's wind, then the same in still air.
const VARIANTS: [Variant; 4] = [
    Variant::GUIDED,
    Variant::TIP_OFF,
    Variant::STILL_GUIDED,
    Variant::STILL_TIP_OFF,
];

/// One public flight's row from its four flights ([`VARIANTS`]' order), after checking the guided
/// one is its report's and that a design with fewer than two guides moved in neither air.
fn row(
    (id, title, set): (&str, &str, &str),
    log_apogee_m: f64,
    committed_m: f64,
    [guided, tip_off, still_guided, still_tip_off]: [Varied; 4],
) -> Result<FlightRow, String> {
    let distance = (guided.apogee_m - committed_m).abs();
    if !distance.is_finite() || distance > GUIDED_WITHIN_M {
        return Err(format!(
            "{id}: flown guided, it lands {distance:.4} m from its committed apogee; run its \
             report again"
        ));
    }
    if [tip_off, still_guided, still_tip_off]
        .iter()
        .any(|v| v.two_guides != guided.two_guides)
    {
        return Err(format!("{id}: its four flights read different guides"));
    }
    if !guided.two_guides
        && (tip_off.apogee_m.to_bits() != guided.apogee_m.to_bits()
            || still_tip_off.apogee_m.to_bits() != still_guided.apogee_m.to_bits())
    {
        return Err(format!(
            "{id}: fewer than two guides, yet the phase moved its apogee"
        ));
    }
    let percent = |apogee_m: f64| 100.0 * (apogee_m - log_apogee_m) / log_apogee_m;
    let move_m = tip_off.apogee_m - guided.apogee_m;
    Ok(FlightRow {
        id: id.to_owned(),
        title: title.to_owned(),
        set: set.to_owned(),
        two_guides: guided.two_guides,
        log_apogee_m,
        guided_apogee_m: guided.apogee_m,
        tip_off_apogee_m: tip_off.apogee_m,
        move_m,
        move_percent: 100.0 * move_m / guided.apogee_m,
        guided_error_percent: percent(guided.apogee_m),
        tip_off_error_percent: percent(tip_off.apogee_m),
        still_guided_apogee_m: still_guided.apogee_m,
        still_tip_off_apogee_m: still_tip_off.apogee_m,
        still_move_m: still_tip_off.apogee_m - still_guided.apogee_m,
    })
}

/// The public flights, each flown four ways ([`VARIANTS`]).
fn public_flights(root: &Path) -> Result<Vec<FlightRow>, String> {
    let text = fs::read_to_string(root.join(real_flight::REPORT_JSON))
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;
    let report: RealFlightReport = serde_json::from_str(&text)
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;

    let first: Vec<_> = FLIGHTS
        .iter()
        .map(|flight| {
            report
                .flights
                .iter()
                .find(|r| r.id == flight.id)
                .map(|r| (flight, r))
                .ok_or_else(|| format!("{}: not in {}", flight.id, real_flight::REPORT_JSON))
        })
        .collect::<Result<_, String>>()?;
    let flown = crate::collection_correction::parallel(&first, |(flight, _)| {
        let mut four = Vec::new();
        for variant in VARIANTS {
            four.push(
                real_flight::fly_varied(root, flight, variant)
                    .map_err(|error| error.to_string())?,
            );
        }
        <[Varied; 4]>::try_from(four).map_err(|_| "four flights".to_owned())
    });
    let mut rows = Vec::new();
    for ((_, r), flown) in first.iter().zip(flown) {
        rows.push(row(
            (&r.id, &r.title, COMPARED_FIRST),
            r.log_apogee_m,
            r.hpr_apogee_m,
            flown?,
        )?);
    }

    let by_variant = crate::collection_correction::parallel(&VARIANTS, |variant| {
        hpr_validate::public_predictions::varied_apogees(root, *variant)
            .map_err(|error| error.to_string())
    })
    .into_iter()
    .collect::<Result<Vec<_>, String>>()?;
    for compared in &report.predicted_first {
        let id = compared.id.as_str();
        let mut four = Vec::new();
        for apogees in &by_variant {
            let (_, varied) = apogees
                .iter()
                .find(|(each, _)| each == id)
                .ok_or_else(|| format!("{id}: not flown by the public predictions"))?;
            four.push(*varied.as_ref().map_err(|e| format!("{id}: {e}"))?);
        }
        rows.push(row(
            (id, &compared.title, PREDICTED_FIRST),
            compared.log_apogee_m,
            compared.hpr_apogee_m,
            <[Varied; 4]>::try_from(four).map_err(|_| format!("{id}: four flights"))?,
        )?);
    }
    Ok(rows)
}

/// The collection's aggregate, from the committed collection report.
fn collection(root: &Path) -> Result<CollectionTipOff, String> {
    let path = crate::fixture_flights::REPORT_JSON;
    let text =
        fs::read_to_string(root.join(path)).map_err(|error| format!("reading {path}: {error}"))?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| format!("reading {path}: {error}"))?;
    serde_json::from_value(value["tip_off"].clone())
        .map_err(|error| format!("reading {path}'s tip_off: {error}"))
}

/// The guided figures of each public set held to the real-flight report's summary, to the two
/// decimals it prints: the same flights, by the same error.
fn check_against_summary(root: &Path, report: &Report) -> Result<(), String> {
    let text = fs::read_to_string(root.join(real_flight::REPORT_JSON))
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;
    let committed: RealFlightReport = serde_json::from_str(&text)
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;
    let summary = &committed.summary;
    for (name, means) in [
        (COMPARED_FIRST, &summary.compared_first),
        (PREDICTED_FIRST, &summary.predicted_first),
    ] {
        let set = report
            .sets
            .iter()
            .find(|s| s.set == name)
            .ok_or_else(|| format!("no set {name}"))?;
        let two = |v: f64| format!("{v:.2}");
        if set.flights != means.flights
            || two(set.guided.mean_absolute_percent)
                != two(means.mean_absolute_apogee_error_percent)
            || two(set.guided.mean_percent) != two(means.mean_apogee_error_percent)
        {
            return Err(format!(
                "the set {name} flown guided is not the real-flight report's: {} flights, {:.2}% \
                 and {:+.2}% against {} flights, {:.2}% and {:+.2}%",
                set.flights,
                set.guided.mean_absolute_percent,
                set.guided.mean_percent,
                means.flights,
                means.mean_absolute_apogee_error_percent,
                means.mean_apogee_error_percent
            ));
        }
    }
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let fresh = Report::of(public_flights(&root)?, collection(&root)?)?;
    check_against_summary(&root, &fresh)?;
    let (path, page_path) = (root.join(REPORT), root.join(REPORT_MD));
    if check {
        let text = fs::read_to_string(&path).map_err(|e| format!("{REPORT}: {e}"))?;
        let committed: Report =
            serde_json::from_str(&text).map_err(|e| format!("{REPORT}: {e}"))?;
        if committed != fresh {
            return Err(format!(
                "{REPORT} is not this run's: run `cargo xtask tip-off-flights` and commit it"
            ));
        }
        let page = fs::read_to_string(&page_path).map_err(|e| format!("{REPORT_MD}: {e}"))?;
        if page != committed.markdown() {
            return Err(format!(
                "{REPORT_MD} is not {REPORT}'s page: run `cargo xtask tip-off-flights`"
            ));
        }
        println!("{REPORT} is current");
        return Ok(());
    }
    let text = serde_json::to_string_pretty(&fresh).map_err(|e| e.to_string())? + "\n";
    fs::write(&path, text).map_err(|e| format!("{REPORT}: {e}"))?;
    fs::write(&page_path, fresh.markdown()).map_err(|e| format!("{REPORT_MD}: {e}"))?;
    println!("wrote {REPORT} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;

    #[test]
    fn bins_merge_a_short_bin_with_its_smaller_neighbor() {
        // Five in [0.0, 0.1), two in [0.1, 0.2), six in [0.3, 0.4): the two join the five, the
        // smaller neighbor, and the six stay their own bin.
        let moves = [
            0.01, 0.02, 0.03, 0.04, 0.05, 0.11, 0.12, 0.31, 0.32, 0.33, 0.34, 0.35, 0.36,
        ];
        let got = bins(&moves);
        assert_eq!(got.len(), 2);
        assert_eq!((got[0].flights, got[1].flights), (7, 6));
        assert!((got[0].from_percent - 0.0).abs() < 1e-12);
        assert!((got[0].to_percent - 0.2).abs() < 1e-12);
        assert!((got[1].from_percent - 0.3).abs() < 1e-12);
        assert!((got[1].to_percent - 0.4).abs() < 1e-12);
        // Two short tails around a large middle: each tail is merged into the middle, the
        // fewest first, and the result holds every flight.
        let mut moves = vec![-0.35, -0.25];
        moves.extend([0.05; 12]);
        moves.extend([0.25, 0.26, 0.27]);
        let got = bins(&moves);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].flights, 17);
        // A move of exactly 0.3% is in the bin from 0.3%.
        let got = bins(&[0.3; 5]);
        assert!((got[0].from_percent - 0.3).abs() < 1e-12);
    }

    #[test]
    fn a_short_last_bin_joins_the_one_below_and_full_bins_stay_apart() {
        // Seven moves in one bin: one bin of seven.
        let got = bins(&[-0.05; 7]);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].flights, 7);
        // Six, then three two bins up: the three are too few and join the six, across the empty
        // bin between them.
        let mut moves = vec![-0.25; 6];
        moves.extend([0.05, 0.06, 0.07]);
        let got = bins(&moves);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].flights, 9);
        assert!((got[0].from_percent + 0.3).abs() < 1e-12);
        assert!((got[0].to_percent - 0.1).abs() < 1e-12);
        // Six and six in neighboring bins: two bins, each its own.
        let mut moves = vec![0.05; 6];
        moves.extend([0.15; 6]);
        let got = bins(&moves);
        assert_eq!(got.len(), 2);
        assert_eq!((got[0].flights, got[1].flights), (6, 6));
        // Fewer than five in all: one bin; none for none.
        assert_eq!(bins(&[0.5, 0.7]).len(), 1);
        assert!(bins(&[]).is_empty());
    }

    #[test]
    fn a_collection_flight_with_one_guide_must_not_move() {
        let flight = CollectionFlight {
            design: "a".to_owned(),
            two_guides: false,
            guided_m: 1000.0,
            tip_off_m: 1000.0,
            still_guided_m: 1001.0,
            still_tip_off_m: 1001.0,
            logged_m: 900.0,
        };
        let aggregate = CollectionTipOff::of(std::slice::from_ref(&flight)).unwrap();
        assert_eq!(aggregate.flights_with_two_guides, 0);
        assert!(aggregate.moves.is_empty());
        let moved = CollectionFlight {
            tip_off_m: 999.0,
            ..flight
        };
        assert!(CollectionTipOff::of(&[moved]).is_err());
    }

    fn committed() -> Report {
        let root = crate::designs::root().unwrap();
        serde_json::from_str(&fs::read_to_string(root.join(REPORT)).unwrap()).unwrap()
    }

    #[test]
    fn the_committed_page_is_the_committed_reports() {
        let root = crate::designs::root().unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(page, committed().markdown());
    }

    #[test]
    fn the_committed_sets_are_their_flights_and_the_collections() {
        // The public sets' means are their rows', and the collection's row its aggregate's: the
        // report holds together without refs/.
        let report = committed();
        let rebuilt = Report::of(report.flights.clone(), report.collection.clone()).unwrap();
        assert_eq!(rebuilt, report);
        // Each row's figures are its apogees'.
        for f in &report.flights {
            let percent = |a: f64| 100.0 * (a - f.log_apogee_m) / f.log_apogee_m;
            assert_eq!(
                f.guided_error_percent,
                percent(f.guided_apogee_m),
                "{}",
                f.id
            );
            assert_eq!(
                f.tip_off_error_percent,
                percent(f.tip_off_apogee_m),
                "{}",
                f.id
            );
            assert_eq!(f.move_m, f.tip_off_apogee_m - f.guided_apogee_m, "{}", f.id);
            if !f.two_guides {
                assert_eq!(f.move_m, 0.0, "{}", f.id);
            }
        }
        // The collection's aggregate is the one its own report commits.
        let root = crate::designs::root().unwrap();
        assert_eq!(collection(&root).unwrap(), report.collection);
        // Every published bin holds at least five flights, unless one bin holds them all.
        let c = &report.collection;
        assert_eq!(
            c.moves.iter().map(|b| b.flights).sum::<usize>(),
            c.flights_with_two_guides
        );
        if c.moves.len() > 1 {
            assert!(c.moves.iter().all(|b| b.flights >= BIN_FLOOR));
        }
    }
}
