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
/// How far a guided flight may land from the apogee its committed report gives, m. On the
/// platform that wrote the report it lands on it to the bit; this leaves room for another
/// platform's rounding, and is 20 times smaller than the smallest move the report gives.
pub const GUIDED_WITHIN_M: f64 = 0.001;
/// A move smaller than this, m, either way, counts as not moved: on a vertical rail in still air
/// gravity has no moment about the last guide, and the public vertical rail's flight moves by
/// under a millimeter there (`validation/reports/tip-off-flights.json`, NDRT 2020).
pub const MOVED_FLOOR_M: f64 = 0.01;
/// The decimals the collection's errors and parts are kept to.
pub const COLLECTION_DECIMALS: i32 = 4;
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
    /// The apogees flown guided and with the phase in still air, m climbed (the day's air with
    /// its wind taken out), for a design with two guides; none for one with fewer, which moves
    /// in neither air.
    pub still: Option<(f64, f64)>,
    /// Whether the flight is flown from a vertical rail.
    pub vertical_rail: bool,
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
/// parts. With fewer than [`BIN_FLOOR`] moves in all there are no bins: the count of flights is
/// given alone, since even one bin would bound those few flights' moves.
pub fn bins(moves_percent: &[f64]) -> Vec<Bin> {
    if moves_percent.len() < BIN_FLOOR {
        return Vec::new();
    }
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
    /// The same in still air, m: gravity's turn alone. Zero for a design with fewer than two
    /// guides, which moves in neither.
    pub still_move_m: f64,
}

/// How many flights with two guides the phase raised, left within [`MOVED_FLOOR_M`] and lowered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ways {
    pub raised: usize,
    pub not_moved: usize,
    pub lowered: usize,
}

impl Ways {
    fn of(moves_m: impl Iterator<Item = f64>) -> Self {
        let mut ways = Self {
            raised: 0,
            not_moved: 0,
            lowered: 0,
        };
        for m in moves_m {
            if m >= MOVED_FLOOR_M {
                ways.raised += 1;
            } else if m <= -MOVED_FLOOR_M {
                ways.lowered += 1;
            } else {
                ways.not_moved += 1;
            }
        }
        ways
    }
}

/// Whether a group of `counts`, which together make up a set of flights, may be published from
/// the private collection: every count none or at least [`BIN_FLOOR`]. A count from one to four
/// would bound those few flights' moves as a bin of them would, and with the others published,
/// any one count follows from the rest.
fn publishable(counts: &[usize]) -> bool {
    counts.iter().all(|&n| n == 0 || n >= BIN_FLOOR)
}

/// Where a set's change in mean absolute error comes from: which way the phase moved the apogees
/// of the flights with two guides, in the wind and in still air, and whether that took each
/// toward its log or away from it. For the private collection a group holding from one to four
/// flights is held back with the rest of its group ([`publishable`]), as are both parts when
/// either is.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Split {
    /// Which way the phase moved the apogees, in the report's wind.
    pub in_the_wind: Option<Ways>,
    /// The same in still air.
    pub in_still_air: Option<Ways>,
    /// Flights whose absolute error the phase raised: moved away from the log.
    pub away: Option<usize>,
    /// Flights whose absolute error it lowered: moved toward the log.
    pub toward: Option<usize>,
    /// The flights moved away's part of the change in the set's mean absolute error, percentage
    /// points: their rises in absolute error summed, over every flight of the set.
    pub away_points: Option<f64>,
    /// The flights moved toward's part, points (zero or less). With `away_points` it sums to the
    /// change in the mean absolute error, so the two are held back together.
    pub toward_points: Option<f64>,
}

impl Split {
    /// The split of a set of `moved` flights; with `floor`, the collection's rule
    /// ([`publishable`]) holds back what it must, and the parts are rounded to
    /// [`COLLECTION_DECIMALS`].
    pub fn of(moved: &[Moved], floor: bool) -> Self {
        let n = moved.len() as f64;
        let two: Vec<&Moved> = moved.iter().filter(|m| m.two_guides).collect();
        let rise = |m: &Moved| m.tip_off_error_percent.abs() - m.guided_error_percent.abs();
        let rises: Vec<f64> = two.iter().map(|m| rise(m)).collect();
        let away = rises.iter().filter(|r| **r > 0.0).count();
        let toward = rises.iter().filter(|r| **r < 0.0).count();
        let unchanged = rises.len() - away - toward;
        let ways = |pick: fn(&Moved) -> f64| {
            let ways = Ways::of(two.iter().map(|m| pick(m)));
            (!floor || publishable(&[ways.raised, ways.not_moved, ways.lowered])).then_some(ways)
        };
        let sides = !floor || publishable(&[away, toward, unchanged]);
        let part = |sign: f64| {
            let sum = rises
                .iter()
                .filter(|r| **r * sign > 0.0)
                .fold(0.0, |sum, r| sum + r);
            let part = if n > 0.0 { sum / n } else { 0.0 };
            if floor {
                round(part, COLLECTION_DECIMALS)
            } else {
                part
            }
        };
        Self {
            in_the_wind: ways(|m| m.move_m),
            in_still_air: ways(|m| m.still_move_m),
            away: sides.then_some(away),
            toward: sides.then_some(toward),
            away_points: sides.then(|| part(1.0)),
            toward_points: sides.then(|| part(-1.0)),
        }
    }

    /// The split in words, for a set with `two_guides` flights that have two guides.
    pub fn sentence(&self, two_guides: usize) -> String {
        let ways = |ways: Option<Ways>| {
            ways.map_or_else(
                || "how many it raised, left within 1 cm and lowered is held back".to_owned(),
                |w| {
                    format!(
                        "the phase raised {}, left {} within 1 cm and lowered {}",
                        w.raised, w.not_moved, w.lowered
                    )
                },
            )
        };
        let sides = match (self.away, self.toward, self.away_points, self.toward_points) {
            (Some(away), Some(toward), Some(a), Some(t)) => format!(
                "It moved {away} away from the log and {toward} toward it. The change in the mean \
                 absolute error is the flights moved away's part ({a:+.4} points) plus the \
                 flights moved toward's ({t:+.4} points), each rounded on its own."
            ),
            _ => "Which way it moved them against their logs is held back.".to_owned(),
        };
        format!(
            "Of the {two_guides} flights with two guides: in the wind, {}; in still air, {}. A \
             group of counts is held back where it would hold one to four flights. {sides}\n\n",
            ways(self.in_the_wind),
            ways(self.in_still_air)
        )
    }
}

/// A move per cent, as a bin's edge prints it: zero unsigned.
fn edge(percent: f64) -> String {
    if percent.abs() < 1e-9 {
        "0.0%".to_owned()
    } else {
        format!("{percent:+.1}%")
    }
}

/// A table of the phase's apogee moves by bin, flown `how`.
fn bins_table(how: &str, bins: &[Bin]) -> String {
    let mut md = format!(
        "The phase's apogee moves {how}, for the flights with two guides, in bins of 0.1% of \
         the apogee guided, each from its lower edge up to its upper one, a bin holding fewer \
         than five merged with its smaller neighbor:\n\n| move | flights |\n|---|---:|\n"
    );
    for bin in bins {
        let _ = writeln!(
            md,
            "| {} to {} | {} |",
            edge(bin.from_percent),
            edge(bin.to_percent),
            bin.flights
        );
    }
    if let [only] = bins {
        let _ = writeln!(md, "\nOne bin holds all {} flights.", only.flights);
    }
    md.push('\n');
    md
}

/// `value` rounded to `decimals` places.
fn round(value: f64, decimals: i32) -> f64 {
    let scale = 10f64.powi(decimals);
    (value * scale).round() / scale
}

/// What the phase moves on the private collection's logged flights, in aggregate only: no
/// flight named or numbered, no apogee, no row of one flight (ADR-363 decision 1). Its errors and
/// parts are rounded to [`COLLECTION_DECIMALS`], as the collection's other aggregates are
/// rounded, so that no regeneration gives a figure to machine precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionTipOff {
    /// The flights compared with their logs.
    pub flights: usize,
    /// Of those, the flights whose design has two or more guides at different stations.
    pub flights_with_two_guides: usize,
    /// Of those, the flights flown from a vertical rail, where gravity has no moment about the
    /// last guide; the collection flies one wherever the flight's tilt is not known.
    pub flights_with_two_guides_on_a_vertical_rail: usize,
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
    /// An apogee that is not a finite number, a flight with fewer than two guides whose apogee
    /// the phase moved (such a design has no stretch for the phase to fly), or one with two
    /// guides and no flights in still air.
    pub(crate) fn of(flights: &[CollectionFlight]) -> Result<Self, String> {
        for f in flights {
            let still = f.still.map_or([0.0, 0.0], |(g, t)| [g, t]);
            if [f.guided_m, f.tip_off_m, f.logged_m, still[0], still[1]]
                .iter()
                .any(|v| !v.is_finite())
            {
                return Err("a collection flight's apogee is not a number".to_owned());
            }
            if f.two_guides != f.still.is_some() {
                return Err(
                    "a collection flight with two guides has no still-air flights, or one \
                     without has"
                        .to_owned(),
                );
            }
            if !f.two_guides && f.tip_off_m.to_bits() != f.guided_m.to_bits() {
                return Err(
                    "a collection flight with fewer than two guides moved with the tip-off phase"
                        .to_owned(),
                );
            }
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
            .filter_map(|f| f.still)
            .map(|(g, t)| 100.0 * (t - g) / g)
            .collect();
        let moved: Vec<Moved> = flights
            .iter()
            .zip(guided.iter().zip(&tip_off))
            .map(|(f, (&g, &t))| Moved {
                two_guides: f.two_guides,
                guided_error_percent: g,
                tip_off_error_percent: t,
                move_m: f.tip_off_m - f.guided_m,
                still_move_m: f.still.map_or(0.0, |(g, t)| t - g),
            })
            .collect();
        let rounded = |e: Option<Errors>| {
            e.map(|e| Errors {
                mean_absolute_percent: round(e.mean_absolute_percent, COLLECTION_DECIMALS),
                mean_percent: round(e.mean_percent, COLLECTION_DECIMALS),
            })
        };
        Ok(Self {
            flights: flights.len(),
            flights_with_two_guides: moves.len(),
            flights_with_two_guides_on_a_vertical_rail: flights
                .iter()
                .filter(|f| f.two_guides && f.vertical_rail)
                .count(),
            designs: designs.len(),
            designs_with_two_guides: designs_two.len(),
            moves: bins(&moves),
            still_air_moves: bins(&still_moves),
            split: Split::of(&moved, true),
            guided: rounded(Errors::of(&guided)),
            tip_off: rounded(Errors::of(&tip_off)),
        })
    }

    /// Its section of the collection's report.
    pub(crate) fn markdown(&self) -> String {
        let mut md = String::from(
            "\n## The tip-off phase (M1.14b14)\n\nEach flight compared is flown again with the \
             tip-off phase: once the guide ahead of the aft-most one leaves the rail, the rocket \
             pivots about its last guide until that one leaves too. Guided is the flight as \
             HPR Sim flies it by default, held straight to its last guide. A design with fewer \
             than two guides at different stations has no such stretch and flies as it did. The \
             move is the apogee with the phase less the apogee guided, over the apogee guided; \
             still air is the day's air with its wind taken out, so that what the phase moves \
             there is gravity's turn about the last guide alone. The public flights' table and \
             what it means are in [the tip-off flights' report](tip-off-flights.md).\n\n",
        );
        let _ = writeln!(
            md,
            "| | compared | with two or more guides at different stations |\n|---|---:|---:|\n| \
             flights | {} | {} |\n| designs | {} | {} |\n\nOf the flights with two guides, {} \
             fly from a vertical rail, where gravity has no moment about the last guide: the \
             collection flies one wherever a flight's tilt is not known.\n",
            self.flights,
            self.flights_with_two_guides,
            self.designs,
            self.designs_with_two_guides,
            self.flights_with_two_guides_on_a_vertical_rail
        );
        if self.flights_with_two_guides == 0 {
            md.push_str("No flight has two guides, so the phase moves none.\n\n");
        } else if self.moves.is_empty() {
            md.push_str(
                "Fewer than five flights have two guides, so their moves are not binned: their \
                 count is given alone.\n\n",
            );
        } else {
            md.push_str(&bins_table("in the day's wind", &self.moves));
            md.push_str(&bins_table("in still air", &self.still_air_moves));
        }
        md.push_str(&self.split.sentence(self.flights_with_two_guides));
        if let (Some(guided), Some(tip_off)) = (self.guided, self.tip_off) {
            let _ = writeln!(
                md,
                "Over every flight compared, guided, the mean absolute apogee error is {:.3}% \
                 and the mean {:+.3}%; with the phase, {:.3}% and {:+.3}%: a change of {:+.4} \
                 points in the mean absolute error.",
                guided.mean_absolute_percent,
                guided.mean_percent,
                tip_off.mean_absolute_percent,
                tip_off.mean_percent,
                tip_off.mean_absolute_percent - guided.mean_absolute_percent
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
             tip-off-flights` ([M1.14b14][m1-14b14], decision [ADR-363][adr-363]; the result \
             in [ADR-364][adr-364]); do not edit. HPR Sim's tip-off phase lets the rocket pivot \
             about its last guide once the guide ahead of it has left the rail ([the \
             phase][flight]; compared with RocketPy's in [the comparison](tip-off-model.md), \
             held in still air and not yet in a crosswind on HPR Sim's own normal force). It is \
             off by default. This report flies every real flight HPR Sim compares with the phase \
             and without it, before the phase becomes the default. It compares HPR Sim with \
             itself: it does not check the phase against a measurement.\n\n- **What is \
             flown:** each flight as its own report flies it, the phase the only change: the \
             seven flights [the real-flight report](real-flights.md) compared as soon as they \
             were flown (*compared first*), the five it compares after their predictions were \
             merged (*predicted first*; [the predictions](public-predictions.md)), and the \
             private collection's logged flights ([its \
             report](fixture-flights.md#the-tip-off-phase-m114b14)), given in aggregate \
             only.\n- **Guided** is the flight as HPR Sim flies it by default: held straight to \
             its last guide, then freed with no turning rate.\n- **Two guides:** a design with \
             fewer than two guides (rail buttons or launch lugs) at different stations has no \
             stretch on its last guide alone, so the phase moves nothing; such a flight is \
             listed as not moved.\n- **Still air:** each flight with two guides flies again, \
             guided and with the phase, in its report's air with the wind taken out. What the \
             phase moves there is gravity's turn about the last guide alone. Taking out the \
             wind changes the guided flight too, so the move in still air and the move in the \
             wind are not two parts of one sum.\n- **The errors** are each set's own report's: \
             for the public flights, HPR Sim's apogee as the log's altimeter would read it, \
             less the log's apogee, over the log's apogee; for the collection, the apogee as a \
             height climbed in the day's air, against the logged apogee read the same \
             way.\n- **The move** is the apogee with the phase less the apogee guided; its share \
             is of the apogee guided.\n- **Landing points:** none of these flights has a \
             landing point compared (the reports compare apogees; landing points against \
             tracked landings are [M2.13][m2-13]), so no landing move is \
             given.\n\n[m1-14b14]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b14\n\
             [adr-363]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md\n\
             [adr-364]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0364-the-tip-off-phase-on-the-real-flights-two-small-rises-measured.md\n\
             [adr-151]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md\n\
             [adr-184]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0184-logged-apogees-of-the-private-collection.md\n\
             [flight]: https://hpr.fusionspace.co/physics/flight.html#what-leaving-tip-off-out-costs\n\
             [m2-13]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-13\n\n## The \
             errors, guided and with the phase\n\n| set | flights | with two guides | mean \
             absolute error, guided | mean absolute error, with the phase | change (points) | \
             mean signed error, guided | mean signed error, with the phase \
             |\n|---|---:|---:|---:|---:|---:|---:|---:|\n",
        );
        let change =
            |set: &SetRow| set.tip_off.mean_absolute_percent - set.guided.mean_absolute_percent;
        for set in &self.sets {
            let _ = writeln!(
                md,
                "| {} | {} | {} | {:.3}% | {:.3}% | {:+.4} | {:+.3}% | {:+.3}% |",
                set.set,
                set.flights,
                set.flights_with_two_guides,
                set.guided.mean_absolute_percent,
                set.tip_off.mean_absolute_percent,
                change(set),
                set.guided.mean_percent,
                set.tip_off.mean_percent,
            );
        }
        let (higher, lower): (Vec<&SetRow>, Vec<&SetRow>) =
            self.sets.iter().partition(|s| s.higher_with_the_phase());
        let listed = |sets: &[&SetRow]| {
            sets.iter()
                .map(|s| format!("{} by {:.4} points", s.set, change(s).abs()))
                .collect::<Vec<_>>()
                .join(", ")
        };
        md.push('\n');
        if higher.is_empty() {
            md.push_str("No set's mean absolute apogee error is higher with the phase.\n\n");
        } else {
            let _ = writeln!(
                md,
                "With the phase the mean absolute apogee error is higher over {}{}. \
                 [ADR-364][adr-364] measures the rises and their cause, as [ADR-363][adr-363] asks before \
                 the phase becomes the default.\n",
                listed(&higher),
                if lower.is_empty() {
                    String::new()
                } else {
                    format!(", and lower over {}", listed(&lower))
                }
            );
        }
        md.push_str(
            "## Where the change comes from\n\nFor each set: which way the phase moved the \
             apogees of the flights with two guides, in the wind each report flies and in still \
             air, and whether the move took the apogee toward the log or away from it. A move \
             under 1 cm (0.4 in) either way counts as not moved: on a vertical rail in still \
             air gravity has no moment about the last guide. The change in a set's mean absolute \
             error is the sum of two parts: the rises in absolute error of the flights moved \
             away, and the falls of those moved toward, each summed and divided by the set's \
             flights. The parts are rounded on their own, so their sum may differ from the \
             change in its last digit.\n\n| set | with two guides | in the wind: raised, not moved, lowered | in \
             still air: raised, not moved, lowered | moved away | moved toward | away's part \
             (points) | toward's part (points) |\n|---|---:|---|---|---:|---:|---:|---:|\n",
        );
        let held = "held back".to_owned();
        for set in &self.sets {
            let t = &set.split;
            let ways = |w: Option<Ways>| {
                w.map_or_else(
                    || held.clone(),
                    |w| format!("{}, {}, {}", w.raised, w.not_moved, w.lowered),
                )
            };
            let count = |c: Option<usize>| c.map_or_else(|| held.clone(), |c| c.to_string());
            let points = |p: Option<f64>| p.map_or_else(|| held.clone(), |p| format!("{p:+.4}"));
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                set.set,
                set.flights_with_two_guides,
                ways(t.in_the_wind),
                ways(t.in_still_air),
                count(t.away),
                count(t.toward),
                points(t.away_points),
                points(t.toward_points)
            );
        }
        md.push_str(
            "\nThe parts are percentage points of the mean absolute error. For the private \
             collection a group of counts is given only where each holds no flight or at least \
             five, and the two parts only where both groups of flights do; otherwise they are \
             held back, so that no figure gives a few flights' own moves.\n\n",
        );
        md.push_str(
            "## Each public flight\n\n| flight | set | two guides | log apogee (m) | apogee \
             guided (m) | with the phase (m) | move (m) | move | move in still air (m) | error \
             guided | error with the phase |\n|---|---|---|---:|---:|---:|---:|---:|---:|---:|\
             ---:|\n",
        );
        for f in &self.flights {
            if f.two_guides {
                let _ = writeln!(
                    md,
                    "| {} | {} | yes | {:.1} | {:.2} | {:.2} | {:+.2} | {:+.3}% | {:+.3} | \
                     {:+.3}% | {:+.3}% |",
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
             at different stations: {} of the {} designs. {} of those flights fly from a \
             vertical rail. No flight is named or numbered here and no apogee of one is given: \
             the collection's terms allow aggregates only ([ADR-151][adr-151], \
             [ADR-184][adr-184]).\n",
            c.flights,
            c.flights_with_two_guides,
            c.designs_with_two_guides,
            c.designs,
            c.flights_with_two_guides_on_a_vertical_rail
        );
        if !c.moves.is_empty() {
            md.push_str(&bins_table("in the day's wind", &c.moves));
            md.push_str(&bins_table("in still air", &c.still_air_moves));
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
        // Six in bin -3, then three in bin 0: the three are too few and join the six, across the
        // two empty bins between them.
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
        // Fewer than five in all: no bin, the count alone; none for none.
        assert!(bins(&[0.5, 0.7, 0.9, 1.1]).is_empty());
        assert!(bins(&[]).is_empty());
        assert_eq!(bins(&[0.5, 0.7, 0.9, 1.1, 1.3]).len(), 1);
    }

    #[test]
    fn a_collection_flight_with_one_guide_must_not_move() {
        let flight = CollectionFlight {
            design: "a".to_owned(),
            two_guides: false,
            guided_m: 1000.0,
            tip_off_m: 1000.0,
            still: None,
            vertical_rail: true,
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

    fn moved(two_guides: bool, guided: f64, tip_off: f64, move_m: f64) -> Moved {
        Moved {
            two_guides,
            guided_error_percent: guided,
            tip_off_error_percent: tip_off,
            move_m,
            still_move_m: -move_m,
        }
    }

    #[test]
    fn the_split_sums_to_the_change_and_the_collection_holds_back_small_groups() {
        // Nineteen moved away (raised 1 m on flights flown 10% high) and four toward (raised on
        // flights flown 10% low), and a one-guide flight.
        let mut flights: Vec<Moved> = (0..19).map(|_| moved(true, 10.0, 10.1, 1.0)).collect();
        flights.extend((0..4).map(|_| moved(true, -10.0, -9.9, 1.0)));
        flights.push(moved(false, 5.0, 5.0, 0.0));
        let public = Split::of(&flights, false);
        assert_eq!((public.away, public.toward), (Some(19), Some(4)));
        let parts = public.away_points.unwrap() + public.toward_points.unwrap();
        let n = flights.len() as f64;
        let change = flights.iter().fold(0.0, |s, m| {
            s + m.tip_off_error_percent.abs() - m.guided_error_percent.abs()
        }) / n;
        assert!((parts - change).abs() < 1e-12);
        assert_eq!(
            public.in_the_wind,
            Some(Ways {
                raised: 23,
                not_moved: 0,
                lowered: 0
            })
        );
        assert_eq!(public.in_still_air.unwrap().lowered, 23);
        // The collection: four toward is a group of fewer than five, so both sides and both
        // parts are held back; the 23 raised in the wind stand, every group none or five or more.
        let collection = Split::of(&flights, true);
        assert_eq!(collection.away, None);
        assert_eq!(collection.toward, None);
        assert_eq!(collection.away_points, None);
        assert_eq!(collection.toward_points, None);
        assert!(collection.in_the_wind.is_some());
        // A move under a centimeter is not moved, and a group of one to four of those holds the
        // whole group back.
        flights[0].move_m = 0.005;
        let collection = Split::of(&flights, true);
        assert_eq!(collection.in_the_wind, None);
        assert_eq!(Split::of(&flights, false).in_the_wind.unwrap().not_moved, 1);
    }

    #[test]
    fn the_committed_guided_apogees_are_the_real_flight_reports() {
        // Without refs/: the report's guided apogees are the ones the real-flight report
        // commits, so a regenerated real-flight report with this one left stale fails here.
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(real_flight::REPORT_JSON)).unwrap();
        let real: RealFlightReport = serde_json::from_str(&text).unwrap();
        for f in &committed().flights {
            let committed_m = real
                .flights
                .iter()
                .find(|r| r.id == f.id)
                .map(|r| r.hpr_apogee_m)
                .or_else(|| {
                    real.predicted_first
                        .iter()
                        .find(|r| r.id == f.id)
                        .map(|r| r.hpr_apogee_m)
                })
                .unwrap();
            assert_eq!(f.guided_apogee_m, committed_m, "{}", f.id);
        }
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
        // Every published bin holds at least five flights, and the bins hold every flight with
        // two guides; fewer than five in all have no bins.
        let c = &report.collection;
        for moves in [&c.moves, &c.still_air_moves] {
            assert!(moves.iter().all(|b| b.flights >= BIN_FLOOR));
            let binned = moves.iter().map(|b| b.flights).sum::<usize>();
            if c.flights_with_two_guides < BIN_FLOOR {
                assert_eq!(binned, 0);
            } else {
                assert_eq!(binned, c.flights_with_two_guides);
            }
        }
    }
}
