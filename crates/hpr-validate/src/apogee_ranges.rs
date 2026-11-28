//! How often a flight's 50% and 90% apogee ranges hold its logged apogee ([M2.8a][m2-8a]; [the
//! decision][adr-386]).
//!
//! A Monte Carlo run flies a design many times, each with its inputs drawn about the values the
//! design file gives, and prints a range of apogees. A 90% range claims that nine real flights in
//! ten end inside it. This module counts whether they do: each logged flight is flown as such a
//! run on the spreads a design file is known by ([`SPREADS`]), and the logged apogee is placed
//! against the run's 50% and 90% ranges ([`FlightRanges::of`]). [`Coverage::of`] counts a set of
//! flights and bounds each share by Clopper and Pearson's two-sided 95% interval
//! (`hpr_analysis::statistics::clopper_pearson_lower`).
//!
//! **The run** is `hpr_analysis::montecarlo::MonteCarlo`, the dispersion `hpr mc` flies:
//! independent normal draws, one random stream a sample and input. A flight's seed is the first
//! eight bytes of the SHA-256 of its id followed by [`SEED_LABEL`] ([`seed`]), so a flight's
//! draws depend on nothing but its id and adding a flight leaves the others' runs alone.
//!
//! **A range** at level `p` runs from the `(1 − p)/2` quantile of the run's apogees to the
//! `(1 + p)/2` one, by Hyndman and Fan's definition 7 (R. J. Hyndman and Y. Fan, "Sample
//! quantiles in statistical packages", *The American Statistician* 50(4), 361–365, 1996), the
//! quantile `hpr mc` prints. The apogee is the center of mass's rise from where it stood at
//! launch (`hpr_sim::Apogee::gain_m`), the height the collection's report compares.
//!
//! **A failed sample** can't help a range hold. The low edge is taken with every failed sample
//! placed above the run's values and the high edge with every failed sample placed below them
//! (`Distribution::quantile_counting_missing`): the narrowest range the failed samples could
//! have left, whichever way they would have flown. With none failed that is the plain range, bit
//! for bit. A run with so many failed that an edge has no value, or whose edges cross, gives
//! [`Place::NoRange`], which counts as a miss; so does a flight whose run can't be set up at all
//! (a `None` in [`Coverage::of`]).
//!
//! **What a count can show.** A range drawn from `n` flights holds a calibrated truth a little
//! less often than its level: the expected share between two order statistics `k` apart is
//! `k/(n + 1)` (H. A. David and H. N. Nagaraja, *Order Statistics*, 3rd ed., 2003, §2.5), so a
//! 90% range from [`SAMPLES`] flights holds 89.8% and a 50% range 49.9%. Both are far inside what
//! 60 flights can tell apart.
//!
//! [m2-8a]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-8a
//! [adr-386]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md

use hpr_analysis::AnalysisError;
use hpr_analysis::montecarlo::{Dispersion, FlightInputs, MonteCarlo};
use hpr_analysis::statistics::{
    CountedQuantile, Distribution, MissingEnd, clopper_pearson_lower, clopper_pearson_upper,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::input_uncertainty::{IMPULSE_OTHER_END, MASS_DESIGN_FILE};

/// The flights of one logged flight's run.
pub const SAMPLES: u64 = 1_000;

/// What follows a flight's id in the bytes its seed is hashed from ([`seed`]).
pub const SEED_LABEL: &str = "ADR-386";

/// The levels of the ranges counted: the 50% range and the 90% range.
pub const LEVELS: [f64; 2] = [0.5, 0.9];

/// The confidence of the two-sided Clopper–Pearson interval on each share.
pub const BOUNDS_CONFIDENCE: f64 = 0.95;

/// The bins of [`Coverage::ranks`]: tenths of a run.
pub const RANK_BINS: usize = 10;

/// One input's standard deviation in the run, and where it is stated.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Spread {
    /// The input, in words.
    pub input: &'static str,
    /// The standard deviation: a fraction of the input, or degrees.
    pub sigma: f64,
    /// The unit of [`Spread::sigma`]: `"fraction"` or `"degrees"`.
    pub unit: &'static str,
    /// Where the standard deviation is stated, and how it is read from there.
    pub from: &'static str,
}

/// The normal distribution's interquartile range over its standard deviation, to the four figures
/// the spreads read from quartiles are divided by: `2 Φ⁻¹(0.75) = 1.3490`.
pub const QUARTILES_OVER_SIGMA: f64 = 1.349;

/// The drag's standard deviation as a fraction of the zero-lift drag coefficient: the quartiles
/// 0.89 and 1.11 of 14 published fits of a coasting rocket's drag over its design code's
/// (`docs/research/flight-derived-cd.md`, §2), read as a normal's, `(1.11 − 0.89)/1.349`, to
/// three figures.
pub const DRAG_SIGMA: f64 = 0.163;

/// The burn time's standard deviation as a fraction of it, at a fixed impulse: one certification
/// test's about its motor's mean over 14 AeroTech H to L motors tested again years apart
/// (`docs/research/hpr-burn-time-scatter.md`, §3).
pub const BURN_TIME_SIGMA: f64 = 0.092;

/// The rail's angle above the horizon, degrees.
pub const RAIL_ELEVATION_SIGMA_DEG: f64 = 1.0;

/// The rail's heading, degrees.
pub const RAIL_AZIMUTH_SIGMA_DEG: f64 = 5.0;

/// The spreads a design file's flight is flown with: what is known of a rocket from its design
/// file and its motor's published curve, with no record of a weighing, a test firing or a
/// flight. Where a source gives a range for a spread, the wider end is taken: a range that is
/// too narrow is the error that flatters.
///
/// Left at their nominal values, each of which would widen a range: the wind and the air (the
/// run's wind spread is a share of the speed, which is no measure of a reanalysis's error, and
/// it has no air temperature), the center of mass, and the altimeter's own scatter. The
/// decision record sizes them.
pub const SPREADS: [Spread; 6] = [
    Spread {
        input: "mass without motors",
        sigma: MASS_DESIGN_FILE.sigma,
        unit: "fraction",
        from: MASS_DESIGN_FILE.from,
    },
    Spread {
        input: "zero-lift drag coefficient",
        sigma: DRAG_SIGMA,
        unit: "fraction",
        from: "docs/research/flight-derived-cd.md, §2: 14 published fits of a coasting rocket's \
               drag stand at 0.89 to 1.11 of the design code's at the quartiles, a standard \
               deviation of 16.3% for a normal spread (the range over 1.349); the note behind \
               the run's inputs, docs/research/dispersion-sigmas.md, §4, gives 10% to 16% for a \
               design file's drag, and this is its wider end",
    },
    Spread {
        input: "total impulse",
        sigma: IMPULSE_OTHER_END,
        unit: "fraction",
        from: "docs/research/hpr-lot-impulse.md, §2: 21 AeroTech H to L motors that Tripoli \
               Motor Testing tested a second time differ by a standard deviation of 3.56 points \
               between their two tests; a flown motor stands from 2.5% to 3.56% off its file's \
               curve where each test fired one motor, and the wider end is taken",
    },
    Spread {
        input: "burn time, at the same impulse",
        sigma: BURN_TIME_SIGMA,
        unit: "fraction",
        from: "docs/research/hpr-burn-time-scatter.md, §3: over 14 AeroTech H to L motors \
               tested again years apart, one test's burn time stands 9.2% from its motor's \
               mean; within one lot and session the same note finds 4.1%, and the wider is \
               taken",
    },
    Spread {
        input: "rail's angle above the horizon",
        sigma: RAIL_ELEVATION_SIGMA_DEG,
        unit: "degrees",
        from: "docs/research/dispersion-sigmas.md, §5: a tilt of 1° in five of the seven hobby \
               sources that disperse a launch angle; each assumes it and none measured it",
    },
    Spread {
        input: "rail's heading",
        sigma: RAIL_AZIMUTH_SIGMA_DEG,
        unit: "degrees",
        from: "docs/research/dispersion-sigmas.md, §5: 1° to 5° of heading in the hobby sources \
               that disperse one, each assumed; the wider end is taken",
    },
];

/// [`SPREADS`] as the dispersion the run flies.
#[must_use]
pub fn design_file() -> Dispersion {
    Dispersion {
        dry_mass_sd_fraction: MASS_DESIGN_FILE.sigma,
        drag_sd_fraction: DRAG_SIGMA,
        impulse_sd_fraction: IMPULSE_OTHER_END,
        burn_time_sd_fraction: BURN_TIME_SIGMA,
        rail_elevation_sd_rad: RAIL_ELEVATION_SIGMA_DEG.to_radians(),
        rail_azimuth_sd_rad: RAIL_AZIMUTH_SIGMA_DEG.to_radians(),
        ..Dispersion::default()
    }
}

/// The seed of a flight's run: the first eight bytes, as a big-endian integer, of the SHA-256 of
/// the flight's id followed by [`SEED_LABEL`].
#[must_use]
pub fn seed(flight_id: &str) -> u64 {
    let digest = Sha256::new()
        .chain_update(flight_id.as_bytes())
        .chain_update(SEED_LABEL.as_bytes())
        .finalize();
    let mut first = [0_u8; 8];
    first.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(first)
}

/// Where a logged apogee sits against a run's range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Place {
    /// The range holds it: at or between the two edges.
    Inside,
    /// It is below the low edge: the run's flights went higher than the real one.
    Below,
    /// It is above the high edge: the run's flights stayed lower than the real one.
    Above,
    /// The run gave no range to place it against: a miss.
    NoRange,
}

/// The narrowest range at `level` the run's failed samples could have left (the module's docs):
/// its low and high edges, m, or `None` where an edge has no value or the edges cross.
///
/// # Errors
///
/// `AnalysisError::Domain` for a level outside `(0, 1)`.
pub fn narrowest_range(
    apogees: &Distribution,
    level: f64,
) -> Result<Option<(f64, f64)>, AnalysisError> {
    if !(level > 0.0 && level < 1.0) {
        return Err(AnalysisError::Domain {
            what: "level of a range",
            value: level,
        });
    }
    let low = apogees.quantile_counting_missing(0.5 * (1.0 - level), MissingEnd::Above)?;
    let high = apogees.quantile_counting_missing(0.5 * (1.0 + level), MissingEnd::Below)?;
    Ok(match (low, high) {
        (Some(CountedQuantile::Value(low)), Some(CountedQuantile::Value(high))) if low <= high => {
            Some((low, high))
        }
        _ => None,
    })
}

/// One logged flight against its run: where the log sits, and how wide the run is. Every figure
/// is a ratio or a count, none a height.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlightRanges {
    /// The samples flown or tried.
    pub attempted: usize,
    /// Of those, the ones that failed.
    pub failed: usize,
    /// Where the logged apogee sits against the range at each of [`LEVELS`].
    pub places: [Place; 2],
    /// The share of the samples that flew whose apogee is below the logged one; `None` when
    /// none flew.
    pub rank: Option<f64>,
    /// The logged apogee less the run's median, over the median; `None` when none flew.
    pub offset: Option<f64>,
    /// The width of the range at each of [`LEVELS`] over the run's median; `None` with no range.
    pub widths: [Option<f64>; 2],
    /// The run's standard deviation over its median; `None` with fewer than two flown.
    pub spread: Option<f64>,
}

impl FlightRanges {
    /// `logged_m` against a run's `apogees`, both heights climbed, m.
    ///
    /// # Errors
    ///
    /// `AnalysisError::Domain` for a logged apogee that isn't finite.
    pub fn of(apogees: &Distribution, logged_m: f64) -> Result<Self, AnalysisError> {
        if !logged_m.is_finite() {
            return Err(AnalysisError::Domain {
                what: "logged apogee (m)",
                value: logged_m,
            });
        }
        let median = apogees.quantile(0.5)?;
        let mut places = [Place::NoRange; 2];
        let mut widths = [None; 2];
        for (index, level) in LEVELS.into_iter().enumerate() {
            if let Some((low, high)) = narrowest_range(apogees, level)? {
                places[index] = if logged_m < low {
                    Place::Below
                } else if logged_m > high {
                    Place::Above
                } else {
                    Place::Inside
                };
                widths[index] = median.map(|median| (high - low) / median);
            }
        }
        let flown = apogees.count();
        let below = apogees
            .sorted()
            .partition_point(|&apogee| apogee < logged_m);
        Ok(Self {
            attempted: apogees.attempted(),
            failed: apogees.missing(),
            places,
            // Cast: counts of samples, far below 2⁵³.
            rank: (flown > 0).then(|| below as f64 / flown as f64),
            offset: median.map(|median| (logged_m - median) / median),
            widths,
            spread: apogees
                .standard_deviation()
                .zip(median)
                .map(|(deviation, median)| deviation / median),
        })
    }
}

/// Flies `nominal` as a run of `samples` flights on `dispersion`, seeded with `seed`, and places
/// `logged_m`, the logged apogee as a height climbed (m), against its ranges. The samples are
/// flown one after another; a caller flies its flights side by side.
///
/// # Errors
///
/// - As `MonteCarlo::new`: a run that can't be set up, which a caller counts as a miss.
/// - As [`FlightRanges::of`].
pub fn fly(
    nominal: FlightInputs,
    dispersion: Dispersion,
    seed: u64,
    samples: u64,
    logged_m: f64,
) -> Result<FlightRanges, AnalysisError> {
    let run = MonteCarlo::new(nominal, dispersion)?.run(seed, samples);
    let apogees = run.distribution(|summary| summary.apogee.as_ref().and_then(|a| a.gain_m))?;
    FlightRanges::of(&apogees, logged_m)
}

/// How often one range held the logs of a set of flights.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LevelCount {
    /// The range's level: 0.5 or 0.9.
    pub level: f64,
    /// The flights whose range held the logged apogee.
    pub held: usize,
    /// The flights whose logged apogee is below the range: the run flew higher.
    pub log_below: usize,
    /// The flights whose logged apogee is above the range: the run stayed lower.
    pub log_above: usize,
    /// The flights with no range, each a miss.
    pub no_range: usize,
    /// `held` over the set's flights, per cent, to two decimals.
    pub held_percent: f64,
    /// The lower edge of the two-sided Clopper–Pearson interval on the share at
    /// [`BOUNDS_CONFIDENCE`], per cent, to two decimals.
    pub lower_percent: f64,
    /// Its upper edge, per cent, to two decimals.
    pub upper_percent: f64,
    /// Whether the level lies inside that interval, edges included, before rounding: whether
    /// this many flights can tell the share from the level.
    pub level_inside: bool,
}

impl LevelCount {
    /// The count of `places`, one a flight, at `level`.
    fn of(level: f64, places: impl Iterator<Item = Place>) -> Self {
        let (mut held, mut log_below, mut log_above, mut no_range) = (0, 0, 0, 0);
        for place in places {
            match place {
                Place::Inside => held += 1,
                Place::Below => log_below += 1,
                Place::Above => log_above += 1,
                Place::NoRange => no_range += 1,
            }
        }
        let flights = held + log_below + log_above + no_range;
        let (lower, upper) = bounds(held, flights);
        Self {
            level,
            held,
            log_below,
            log_above,
            no_range,
            // Cast: counts of flights, far below 2⁵³.
            held_percent: round2(100.0 * held as f64 / flights.max(1) as f64),
            lower_percent: round2(100.0 * lower),
            upper_percent: round2(100.0 * upper),
            level_inside: flights > 0 && lower <= level && level <= upper,
        }
    }

    /// The flights counted.
    #[must_use]
    pub fn flights(&self) -> usize {
        self.held + self.log_below + self.log_above + self.no_range
    }
}

/// The two-sided Clopper–Pearson interval at [`BOUNDS_CONFIDENCE`] on a share of `held` in
/// `flights`, as fractions; the whole of `[0, 1]` with no flights.
#[must_use]
pub fn bounds(held: usize, flights: usize) -> (f64, f64) {
    if flights == 0 || held > flights {
        return (0.0, 1.0);
    }
    // Cast: counts of flights, far below 2⁶⁴.
    let (x, n) = (held as u64, flights as u64);
    (
        clopper_pearson_lower(x, n, BOUNDS_CONFIDENCE),
        clopper_pearson_upper(x, n, BOUNDS_CONFIDENCE),
    )
}

/// `value` to two decimals.
fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// The median of `values`, by Hyndman and Fan's definition 7; `None` with none.
fn median(values: Vec<f64>) -> Option<f64> {
    let count = values.len();
    Distribution::new(values, count)
        .ok()?
        .quantile(0.5)
        .ok()
        .flatten()
}

/// A set of flights against their runs: counts and medians over the set, nothing of one flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coverage {
    /// The flights of the set.
    pub flights: usize,
    /// Of those, the ones whose run couldn't be set up: no sample was tried, and each is a miss
    /// at every level.
    pub runs_failed: usize,
    /// The samples tried over the set's runs.
    pub samples: usize,
    /// Of those, the ones that failed.
    pub samples_failed: usize,
    /// The flights with at least one failed sample.
    pub flights_with_a_failed_sample: usize,
    /// Each of [`LEVELS`]' counts.
    pub levels: Vec<LevelCount>,
    /// The flights by the tenth of their run the logged apogee falls in: the first counts the
    /// logs below a run's lowest tenth, the last those above its highest. A run that flew no
    /// sample is in none. Runs whose ranges hold as often as they say fill the tenths evenly.
    pub ranks: [usize; RANK_BINS],
    /// The median, over the set, of the logged apogee less the run's median, over that median,
    /// per cent, to two decimals: negative where the logs sit below their runs' middles.
    pub median_offset_percent: Option<f64>,
    /// The median, over the set, of each range's width over its run's median, per cent, to two
    /// decimals, by [`LEVELS`].
    pub median_width_percent: [Option<f64>; 2],
    /// The median of the runs' standard deviations over their medians, per cent, to two
    /// decimals.
    pub median_spread_percent: Option<f64>,
    /// The least of those.
    pub least_spread_percent: Option<f64>,
    /// The most.
    pub most_spread_percent: Option<f64>,
}

impl Coverage {
    /// The counts of `flights`: each a flight's run against its log, or `None` for a run that
    /// couldn't be set up.
    #[must_use]
    pub fn of(flights: &[Option<FlightRanges>]) -> Self {
        let ran: Vec<&FlightRanges> = flights.iter().flatten().collect();
        let levels = LEVELS
            .into_iter()
            .enumerate()
            .map(|(index, level)| {
                LevelCount::of(
                    level,
                    flights
                        .iter()
                        .map(|flight| flight.map_or(Place::NoRange, |ranges| ranges.places[index])),
                )
            })
            .collect();
        let mut ranks = [0; RANK_BINS];
        for rank in ran.iter().filter_map(|ranges| ranges.rank) {
            // Cast: a rank in [0, 1] times the bins, floored; 1 itself goes in the last bin.
            let bin = ((rank * RANK_BINS as f64).floor() as usize).min(RANK_BINS - 1);
            ranks[bin] += 1;
        }
        let percent = |value: Option<f64>| value.map(|value| round2(100.0 * value));
        let spreads: Vec<f64> = ran.iter().filter_map(|ranges| ranges.spread).collect();
        let least = spreads.iter().copied().min_by(f64::total_cmp);
        let most = spreads.iter().copied().max_by(f64::total_cmp);
        let width = |index: usize| {
            percent(median(
                ran.iter()
                    .filter_map(|ranges| ranges.widths[index])
                    .collect(),
            ))
        };
        Self {
            flights: flights.len(),
            runs_failed: flights.len() - ran.len(),
            samples: ran.iter().map(|ranges| ranges.attempted).sum(),
            samples_failed: ran.iter().map(|ranges| ranges.failed).sum(),
            flights_with_a_failed_sample: ran.iter().filter(|ranges| ranges.failed > 0).count(),
            levels,
            ranks,
            median_offset_percent: percent(median(
                ran.iter().filter_map(|ranges| ranges.offset).collect(),
            )),
            median_width_percent: [width(0), width(1)],
            median_spread_percent: percent(median(spreads)),
            least_spread_percent: percent(least),
            most_spread_percent: percent(most),
        }
    }

    /// Checks the counts against one another: each level's four counts add up to the set's
    /// flights, a level's misses with no range are at least the runs that failed, the tenths
    /// hold no more flights than ran, and each share and its bounds are the counts' own.
    ///
    /// # Errors
    ///
    /// What doesn't hold, in words.
    pub fn check(&self) -> Result<(), String> {
        if self.levels.len() != LEVELS.len() {
            return Err(format!(
                "{} levels, not {}",
                self.levels.len(),
                LEVELS.len()
            ));
        }
        if self.runs_failed > self.flights || self.samples_failed > self.samples {
            return Err("more failed than tried".to_owned());
        }
        let ran = self.flights - self.runs_failed;
        if self.ranks.iter().sum::<usize>() > ran || self.flights_with_a_failed_sample > ran {
            return Err("more flights placed than ran".to_owned());
        }
        for (count, level) in self.levels.iter().zip(LEVELS) {
            if count.level != level {
                return Err(format!("a level of {}, not {level}", count.level));
            }
            if count.flights() != self.flights {
                return Err(format!(
                    "the {level} range's counts add up to {}, not the set's {} flights",
                    count.flights(),
                    self.flights
                ));
            }
            if count.no_range < self.runs_failed {
                return Err(format!(
                    "the {level} range counts {} flights with no range, fewer than the {} runs \
                     that failed",
                    count.no_range, self.runs_failed
                ));
            }
            let again = LevelCount::of(
                level,
                [
                    (Place::Inside, count.held),
                    (Place::Below, count.log_below),
                    (Place::Above, count.log_above),
                    (Place::NoRange, count.no_range),
                ]
                .into_iter()
                .flat_map(|(place, times)| std::iter::repeat_n(place, times)),
            );
            if again != *count {
                return Err(format!(
                    "the {level} range's share or bounds aren't its counts'"
                ));
            }
        }
        // The 90% range is the wider: it holds every log the 50% range holds.
        if self.levels[1].held < self.levels[0].held {
            return Err("the 90% range holds fewer logs than the 50% range".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hpr_core::random::SeededRng;

    use super::*;

    /// Holds the narrowest range of `apogees` at `level` to `expected`, to a part in 10¹²: a
    /// tail such as `(1 − 0.9)/2` isn't exact in binary, so an edge isn't either.
    fn assert_range(apogees: &Distribution, level: f64, expected: (f64, f64)) {
        let (low, high) = narrowest_range(apogees, level).unwrap().unwrap();
        assert!(
            (low - expected.0).abs() < 1e-9 && (high - expected.1).abs() < 1e-9,
            "{low} to {high}, not {expected:?}"
        );
    }

    /// A run's apogees: `count` normal draws about `center` with standard deviation `sigma`, on
    /// stream `key` of `seed`, of which the last `failed` are dropped as failed samples.
    fn run(
        seed: u64,
        key: u64,
        center: f64,
        sigma: f64,
        count: usize,
        failed: usize,
    ) -> Distribution {
        let mut rng = SeededRng::for_stream(seed, &[key, 1]);
        let values = (0..count - failed)
            .map(|_| center + sigma * rng.standard_normal())
            .collect();
        Distribution::new(values, count).unwrap()
    }

    /// `flights` invented flights on `seed`: each truth one normal draw about 1,000 m with a
    /// standard deviation of 100 m, each run `samples` draws about `center` with `sigma`.
    fn flights(seed: u64, flights: u64, samples: usize, center: f64, sigma: f64) -> Coverage {
        let rows: Vec<Option<FlightRanges>> = (0..flights)
            .map(|flight| {
                let truth =
                    1_000.0 + 100.0 * SeededRng::for_stream(seed, &[flight, 0]).standard_normal();
                let apogees = run(seed, flight, center, sigma, samples, 0);
                Some(FlightRanges::of(&apogees, truth).unwrap())
            })
            .collect();
        Coverage::of(&rows)
    }

    /// The spreads are their sources' arithmetic, and the dispersion flies exactly them: the
    /// drag's quartiles over 1.349, the mass's as the validation uncertainty reads it, the
    /// impulse's wider end, and nothing else dispersed.
    #[test]
    fn the_spreads_are_their_sources_figures() {
        assert!((QUARTILES_OVER_SIGMA - 2.0 * 0.674_489_750_196_081_7).abs() < 5e-5);
        assert!(((1.11_f64 - 0.89) / QUARTILES_OVER_SIGMA - DRAG_SIGMA).abs() < 5e-4);
        assert!(((0.131_f64 + 0.014) / QUARTILES_OVER_SIGMA - MASS_DESIGN_FILE.sigma).abs() < 5e-5);
        assert_eq!(IMPULSE_OTHER_END, 0.0356);
        let dispersion = design_file();
        assert_eq!(
            dispersion,
            Dispersion {
                dry_mass_sd_fraction: 0.1075,
                drag_sd_fraction: 0.163,
                impulse_sd_fraction: 0.0356,
                burn_time_sd_fraction: 0.092,
                rail_elevation_sd_rad: 1.0_f64.to_radians(),
                rail_azimuth_sd_rad: 5.0_f64.to_radians(),
                cg_sd_m: 0.0,
                ejection_delay_sd_s: 0.0,
                wind_speed_sd_fraction: 0.0,
                wind_heading_sd_rad: 0.0,
                deployment_lag_sd_s: 0.0,
            }
        );
        dispersion.validate().unwrap();
        // The table and the dispersion say the same six numbers.
        let table: Vec<f64> = SPREADS.iter().map(|spread| spread.sigma).collect();
        assert_eq!(table, [0.1075, 0.163, 0.0356, 0.092, 1.0, 5.0]);
    }

    /// The wider end is the one taken wherever a source gives two: a narrower impulse, drag or
    /// burn time than the rejected end fails here.
    #[test]
    fn no_spread_is_the_narrower_end_of_its_sources_range() {
        // The impulse's range is 2.5% to 3.56%, the drag's 10% to 16%, the burn time's 4.1% to
        // 9.2%, the heading's 1° to 5°.
        let narrower_ends = [
            ("total impulse", 0.025),
            ("zero-lift drag coefficient", 0.10),
            ("burn time, at the same impulse", 0.041),
            ("rail's heading", 1.0),
        ];
        for (input, narrower) in narrower_ends {
            let taken: Vec<f64> = SPREADS
                .iter()
                .filter(|spread| spread.input == input)
                .map(|spread| spread.sigma)
                .collect();
            assert_eq!(taken.len(), 1, "{input}");
            assert!(taken[0] > narrower, "{input}: {} taken", taken[0]);
        }
    }

    /// Seeds computed outside this code: Python's
    /// `int.from_bytes(hashlib.sha256(id.encode() + b"ADR-386").digest()[:8], "big")`.
    #[test]
    fn a_flights_seed_is_its_ids_digest() {
        assert_eq!(seed("example-flight-1"), SEED_OF_EXAMPLE_1);
        assert_eq!(seed("example-flight-2"), SEED_OF_EXAMPLE_2);
        assert_eq!(seed(""), SEED_OF_NOTHING);
        assert_ne!(seed("example-flight-1"), seed("example-flight-2"));
    }

    const SEED_OF_EXAMPLE_1: u64 = 1_612_627_318_722_683_556;
    const SEED_OF_EXAMPLE_2: u64 = 10_387_327_150_652_347_090;
    const SEED_OF_NOTHING: u64 = 6_574_970_023_348_783_343;

    /// Eleven apogees 0, 10, …, 100 m: by definition 7 the 5% point is 5 m, the 25% point 25 m,
    /// the 75% point 75 m and the 95% point 95 m. A log on an edge is held; one just past it
    /// isn't, and which side it missed on is kept.
    #[test]
    fn a_log_is_placed_against_both_ranges() {
        let apogees =
            Distribution::new((0..=10).map(|k| 10.0 * f64::from(k)).collect(), 11).unwrap();
        assert_range(&apogees, 0.5, (25.0, 75.0));
        assert_range(&apogees, 0.9, (5.0, 95.0));
        let place = |logged: f64| FlightRanges::of(&apogees, logged).unwrap().places;
        assert_eq!(place(50.0), [Place::Inside, Place::Inside]);
        assert_eq!(place(25.0), [Place::Inside, Place::Inside]);
        assert_eq!(place(75.0), [Place::Inside, Place::Inside]);
        assert_eq!(place(24.999), [Place::Below, Place::Inside]);
        assert_eq!(place(75.001), [Place::Above, Place::Inside]);
        assert_eq!(place(5.001), [Place::Below, Place::Inside]);
        assert_eq!(place(95.0), [Place::Above, Place::Inside]);
        assert_eq!(place(4.999), [Place::Below, Place::Below]);
        assert_eq!(place(95.001), [Place::Above, Place::Above]);
        assert_eq!(place(-1.0e6), [Place::Below, Place::Below]);
        assert_eq!(place(1.0e6), [Place::Above, Place::Above]);
        let ranges = FlightRanges::of(&apogees, 40.0).unwrap();
        // Four of the eleven are below 40 m; the median is 50 m.
        assert_eq!(ranges.rank, Some(4.0 / 11.0));
        assert_eq!(ranges.offset, Some(-0.2));
        let [half, ninety] = ranges.widths.map(Option::unwrap);
        assert!((half - 1.0).abs() < 1e-12 && (ninety - 1.8).abs() < 1e-12);
        assert_eq!((ranges.attempted, ranges.failed), (11, 0));
        assert!(FlightRanges::of(&apogees, f64::NAN).is_err());
        assert!(narrowest_range(&apogees, 1.0).is_err());
        assert!(narrowest_range(&apogees, 0.0).is_err());
    }

    /// A failed sample narrows the range, and never widens it. Nine apogees 10, …, 90 m of 11
    /// tried: the low edges count the two failed as above (the 0.5th and 2.5th of 11, 15 and
    /// 35 m), the high edges as below (the 7.5th and 9.5th, less two: 65 and 85 m). Over the
    /// nine alone the ranges would be 30 to 70 m and 14 to 86 m.
    #[test]
    fn failed_samples_narrow_a_range() {
        let nine = || (1..=9).map(|k| 10.0 * f64::from(k)).collect();
        let apogees = Distribution::new(nine(), 11).unwrap();
        assert_range(&apogees, 0.9, (15.0, 85.0));
        assert_range(&apogees, 0.5, (35.0, 65.0));
        let plain = Distribution::new(nine(), 9).unwrap();
        assert_range(&plain, 0.9, (14.0, 86.0));
        assert_range(&plain, 0.5, (30.0, 70.0));
        let ranges = FlightRanges::of(&apogees, 32.0).unwrap();
        // 32 m is inside the nine's own 50% range and outside the narrowest one.
        assert_eq!(ranges.places, [Place::Below, Place::Inside]);
        assert_eq!((ranges.attempted, ranges.failed), (11, 2));
    }

    /// With too many failed an edge has no value or the edges cross, and the flight is a miss at
    /// that level. Three apogees of eleven tried leave no 50% range (its low edge, the 2.5th of
    /// the eleven, falls on a failed sample) and a 90% range of 15 to 25 m; with the same three
    /// of 31 tried the 90% range's edges cross (the 1.5th is 25 m, the 28.5th less 28 is 15 m);
    /// one apogee of eleven leaves no range at either level, and none flown leaves nothing at
    /// all.
    #[test]
    fn a_run_with_no_range_is_a_miss() {
        let three = Distribution::new(vec![10.0, 20.0, 30.0], 11).unwrap();
        assert_eq!(narrowest_range(&three, 0.5).unwrap(), None);
        assert_range(&three, 0.9, (15.0, 25.0));
        let ranges = FlightRanges::of(&three, 20.0).unwrap();
        assert_eq!(ranges.places, [Place::NoRange, Place::Inside]);
        let crossed = Distribution::new(vec![10.0, 20.0, 30.0], 31).unwrap();
        assert_eq!(narrowest_range(&crossed, 0.9).unwrap(), None);
        let few = Distribution::new(vec![20.0], 11).unwrap();
        assert_eq!(narrowest_range(&few, 0.5).unwrap(), None);
        assert_eq!(narrowest_range(&few, 0.9).unwrap(), None);
        let ranges = FlightRanges::of(&few, 20.0).unwrap();
        assert_eq!(ranges.places, [Place::NoRange, Place::NoRange]);
        assert_eq!(ranges.widths, [None, None]);
        let none = Distribution::new(Vec::new(), 11).unwrap();
        let ranges = FlightRanges::of(&none, 20.0).unwrap();
        assert_eq!(ranges.places, [Place::NoRange, Place::NoRange]);
        assert_eq!(
            (ranges.rank, ranges.offset, ranges.spread),
            (None, None, None)
        );
        // A run that couldn't be set up and a run with no range both count as misses.
        let coverage = Coverage::of(&[None, Some(ranges)]);
        assert_eq!((coverage.flights, coverage.runs_failed), (2, 1));
        for count in &coverage.levels {
            assert_eq!((count.held, count.no_range), (0, 2));
            assert_eq!(count.held_percent, 0.0);
        }
        assert_eq!(coverage.samples_failed, 11);
        assert_eq!(coverage.flights_with_a_failed_sample, 1);
        coverage.check().unwrap();
    }

    /// The bounds are Clopper and Pearson's, to the figures their paper's method gives: 30 of 60
    /// is 36.81% to 63.19%, 54 of 60 is 79.49% to 96.24%, 0 of 23 is 0 to 14.82%, 23 of 23 is
    /// 85.18% to 100% (the closed forms `1 − 0.025^(1/23)` and `0.025^(1/23)`; the others from
    /// SciPy 1.18's `binomtest(k, n).proportion_ci(0.95, "exact")`).
    #[test]
    fn the_bounds_are_clopper_and_pearsons() {
        let percent = |held, flights| {
            let (lower, upper) = bounds(held, flights);
            (round2(100.0 * lower), round2(100.0 * upper))
        };
        assert_eq!(percent(30, 60), (36.81, 63.19));
        assert_eq!(percent(54, 60), (79.49, 96.24));
        assert_eq!(percent(0, 23), (0.0, 14.82));
        assert_eq!(percent(23, 23), (85.18, 100.0));
        let (_, upper) = bounds(0, 23);
        assert!((upper - (1.0 - 0.025_f64.powf(1.0 / 23.0))).abs() < 1e-12);
        assert_eq!(bounds(0, 0), (0.0, 1.0));
        assert_eq!(bounds(3, 2), (0.0, 1.0));
    }

    /// One seed, pinned: 400 invented flights whose runs are drawn as their truths are. The 50%
    /// range holds about half and the 90% range about nine in ten, both levels inside their
    /// bounds, the tenths near even; the same runs at half the spread hold too few, at twice the
    /// spread too many at 50%, and with their centers one standard deviation high they miss on
    /// the low side. Each wrong run is told from the level by its bounds.
    #[test]
    fn a_calibrated_run_passes_and_wrong_ones_are_told_apart() {
        let calibrated = flights(7, 400, 1_000, 1_000.0, 100.0);
        calibrated.check().unwrap();
        assert_eq!(calibrated.flights, 400);
        assert!(
            calibrated.levels.iter().all(|count| count.level_inside),
            "{calibrated:?}"
        );
        assert!(
            calibrated
                .ranks
                .iter()
                .all(|&count| (25..=55).contains(&count))
        );
        let narrow = flights(7, 400, 1_000, 1_000.0, 50.0);
        assert!(narrow.levels.iter().all(|count| !count.level_inside));
        assert!(narrow.levels[0].upper_percent < 50.0 && narrow.levels[1].upper_percent < 90.0);
        let wide = flights(7, 400, 1_000, 1_000.0, 200.0);
        assert!(wide.levels.iter().all(|count| !count.level_inside));
        assert!(wide.levels[0].lower_percent > 50.0 && wide.levels[1].lower_percent > 90.0);
        let high = flights(7, 400, 1_000, 1_100.0, 100.0);
        assert!(high.levels.iter().all(|count| !count.level_inside));
        for count in &high.levels {
            assert!(count.log_below > 3 * count.log_above, "{count:?}");
        }
        assert!(high.median_offset_percent.unwrap() < -5.0);
        // A calibrated run's logs sit about its middle, and its ranges are a normal's widths:
        // 1.349 and 3.290 standard deviations of 10% of the median.
        assert!(calibrated.median_offset_percent.unwrap().abs() < 2.0);
        let [half, ninety] = calibrated.median_width_percent.map(Option::unwrap);
        assert!(
            (half - 13.49).abs() < 0.3 && (ninety - 32.9).abs() < 0.6,
            "{half} {ninety}"
        );
    }

    /// Over 100 seeds, 60 flights each (the collection's size) and 200 samples a run: a
    /// calibrated run's level is inside its bounds on at least 90 of the 100 at each level (the
    /// interval is built to hold it on 95 or more), while a run at a third of the spread is
    /// never inside at either level and one at three times the spread never at 50%.
    #[test]
    fn sixty_flights_tell_a_run_three_times_off() {
        let mut inside = [0_usize; 2];
        for seed in 0..100 {
            let calibrated = flights(seed, 60, 200, 1_000.0, 100.0);
            for (count, total) in calibrated.levels.iter().zip(&mut inside) {
                *total += usize::from(count.level_inside);
            }
            let narrow = flights(seed, 60, 200, 1_000.0, 100.0 / 3.0);
            assert!(
                narrow.levels.iter().all(|count| !count.level_inside),
                "seed {seed}"
            );
            let wide = flights(seed, 60, 200, 1_000.0, 300.0);
            assert!(!wide.levels[0].level_inside, "seed {seed}");
        }
        assert!(inside.iter().all(|&count| count >= 90), "{inside:?}");
    }

    /// The counts must hold to one another: a spoiled share, a count that doesn't add up, or a
    /// 90% range holding fewer than the 50% one is refused.
    #[test]
    fn spoiled_counts_are_refused() {
        let good = flights(3, 60, 200, 1_000.0, 100.0);
        good.check().unwrap();
        let mut share = good.clone();
        share.levels[0].held_percent += 0.01;
        assert!(share.check().unwrap_err().contains("share or bounds"));
        let mut bound = good.clone();
        bound.levels[1].upper_percent -= 0.01;
        assert!(bound.check().unwrap_err().contains("share or bounds"));
        let mut inside = good.clone();
        inside.levels[1].level_inside = !inside.levels[1].level_inside;
        assert!(inside.check().unwrap_err().contains("share or bounds"));
        let mut sum = good.clone();
        sum.levels[0].held += 1;
        assert!(sum.check().unwrap_err().contains("add up"));
        let mut order = good.clone();
        let moved = order.levels[1].held;
        order.levels[1].held = 0;
        order.levels[1].log_below += moved;
        let again = LevelCount::of(
            0.9,
            std::iter::repeat_n(Place::Below, order.levels[1].log_below)
                .chain(std::iter::repeat_n(Place::Above, order.levels[1].log_above)),
        );
        order.levels[1] = again;
        assert!(order.check().unwrap_err().contains("fewer logs"));
        let mut placed = good.clone();
        placed.runs_failed = 1;
        assert!(placed.check().unwrap_err().contains("more flights placed"));
        // With one flight taken out of the tenths, the runs that failed outnumber the misses
        // with no range.
        let mut failed = good;
        failed.runs_failed = 1;
        let first = failed.ranks.iter().position(|&count| count > 0).unwrap();
        failed.ranks[first] -= 1;
        assert!(
            failed
                .check()
                .unwrap_err()
                .contains("fewer than the 1 runs")
        );
    }
}
