//! The readings' 90% intervals held to the synthetic flights' truth ([ADR-257][adr-257] §1–2,
//! [ADR-308][adr-308]).
//!
//! Each flight's own log is read by `hpr_flightdata::readings`, and each reading that states a
//! 90% interval ([`hpr_flightdata::readings::interval`]) is compared with the flight's truth:
//! below the interval, inside it, or above it. The coverage test of [ADR-257][adr-257] §1,
//! calibrated intervals, takes the three counts over a
//! set, each with its two-sided 99.9% Clopper–Pearson band: the two tails' bands must each hold
//! 5%, and the band of the count inside must hold 90%. At 1,000 readings a tail passes with 29 to
//! 74 misses. Three probes must each fail it: every interval narrowed to 0.8 of its width about
//! its middle, widened to 1.25, and shifted down by one stated standard error, the width over
//! `2 × 1.645`. For normal errors those hold the truth about 81% and 96% of the time, and the
//! shifted one misses above about 26% of the time.
//!
//! The intervals' shares are fitted on [`Set::Calibration`](super::Set::Calibration) ([`fitted`]); the test runs on
//! [`Set::A`](super::Set::A) and [`Set::B`](super::Set::B), which the shares never see.
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-308]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md

use std::collections::BTreeMap;

use hpr_flightdata::readings::interval::{self, Interval, Shares};
use hpr_flightdata::readings::{self, Reading, Reason};
use serde::{Deserialize, Serialize};

use super::Flight;
use super::designs::DESIGNS;
use super::held::Quantity;
use super::smoothed::{BAND_CONFIDENCE, MISS_SHARE, clopper_pearson_lower, clopper_pearson_upper};

/// The readings that state a 90% interval, in the report's order.
pub const STATED: [Quantity; 2] = [Quantity::ApogeeHeight, Quantity::ApogeeTime];

/// The share of readings whose truth the interval must hold.
pub const INSIDE_SHARE: f64 = 0.90;

/// The standard normal's 95% point: a 90% interval's half-width over its standard error.
pub const Z_90: f64 = 1.644_853_626_951_472_2;

/// The library's shares for `quantity`; `None` for a reading that states no interval yet.
pub fn shares(quantity: Quantity) -> Option<Shares> {
    match quantity {
        Quantity::ApogeeHeight => Some(interval::APOGEE_HEIGHT_SHARES),
        Quantity::ApogeeTime => Some(interval::APOGEE_TIME_SHARES),
        _ => None,
    }
}

/// One reading's interval against its truth.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Compared {
    /// The reading.
    pub reading: f64,
    /// The bound it states.
    pub bound: f64,
    /// Its 90% interval.
    pub interval: Interval,
    /// The truth, on the reading's own scale and clock.
    pub truth: f64,
}

impl Compared {
    /// Its error as a share of its bound, `(reading − truth)/bound`: what the shares are fitted
    /// to.
    pub fn share(&self) -> f64 {
        (self.reading - self.truth) / self.bound
    }
}

/// What one log said of one reading.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Said {
    /// Read, with a 90% interval.
    Stated(Compared),
    /// Read with no interval: a floor, or a bound no interval can be formed from.
    Unstated,
    /// Withheld, for this reason.
    Withheld(Reason),
}

/// A flight's index, its design, logger and day's air, and what its own log said of each of
/// [`STATED`].
#[derive(Debug, Clone, PartialEq)]
pub struct FlightSaid {
    /// Its index in its set.
    pub index: u64,
    /// Its design, an index into [`DESIGNS`].
    pub design: usize,
    /// Its logger's name.
    pub logger: &'static str,
    /// The day's air, K from the 1976 standard atmosphere.
    pub air_offset_k: f64,
    /// Each of [`STATED`], in that order.
    pub said: Vec<(Quantity, Said)>,
}

/// What `flight`'s own log says of each of [`STATED`], against its truth.
pub fn said(index: u64, flight: &Flight) -> FlightSaid {
    let read = readings::read(&flight.log);
    let truth = &flight.truth;
    let pad_s = flight.draw.logger.pad_s;
    let said = STATED
        .iter()
        .map(|&quantity| {
            // The truths are `held::outcomes`' own: the apogee above the pad, and its time on the
            // log's clock.
            let parts = match (quantity, &read.apogee) {
                (_, Reading::Withheld(withheld)) => Err(Said::Withheld(withheld.reason)),
                (Quantity::ApogeeHeight, Reading::Read(apogee)) => Ok((
                    apogee.height_m,
                    apogee.bound_m,
                    apogee.height_interval_m,
                    truth.apogee_above_pad_m(),
                )),
                (Quantity::ApogeeTime, Reading::Read(apogee)) => Ok((
                    apogee.time_s,
                    apogee.time_bound_s,
                    apogee.time_interval_s,
                    pad_s + truth.events.apogee_s,
                )),
                // A quantity in `STATED` with no arm here states nothing: its test then compares
                // no reading and fails, as an empty count holds nothing.
                _ => Err(Said::Unstated),
            };
            let said = match parts {
                Err(said) => said,
                Ok((reading, bound, Some(interval), truth)) => Said::Stated(Compared {
                    reading,
                    bound,
                    interval,
                    truth,
                }),
                Ok((_, _, None, _)) => Said::Unstated,
            };
            (quantity, said)
        })
        .collect();
    FlightSaid {
        index,
        design: flight.draw.design,
        logger: flight.draw.logger.profile.name(),
        air_offset_k: flight.draw.temperature_offset_k,
        said,
    }
}

/// A change to every interval that the coverage test must catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Probe {
    /// Narrowed to 0.8 of its width about its middle.
    Narrowed,
    /// Widened to 1.25 of its width about its middle.
    Widened,
    /// Shifted down by one stated standard error, its width over `2 Z_90`.
    ShiftedDown,
}

impl Probe {
    /// Every probe.
    pub const ALL: [Self; 3] = [Self::Narrowed, Self::Widened, Self::ShiftedDown];

    /// Its name in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::Narrowed => "narrowed to 0.8",
            Self::Widened => "widened to 1.25",
            Self::ShiftedDown => "shifted down one standard error",
        }
    }

    /// `interval`, changed.
    pub fn apply(self, interval: Interval) -> Interval {
        let middle = 0.5 * (interval.low + interval.high);
        let half = 0.5 * (interval.high - interval.low);
        let about = |factor: f64| Interval {
            low: middle - factor * half,
            high: middle + factor * half,
        };
        match self {
            Self::Narrowed => about(0.8),
            Self::Widened => about(1.25),
            Self::ShiftedDown => {
                let sd = half / Z_90;
                Interval {
                    low: interval.low - sd,
                    high: interval.high - sd,
                }
            }
        }
    }
}

/// Readings whose truth lay below their interval, inside it and above it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    /// Readings compared.
    pub given: u64,
    /// The truth below the interval.
    pub below: u64,
    /// The truth inside it, ends included.
    pub inside: u64,
    /// The truth above it.
    pub above: u64,
}

impl Counts {
    /// Counts `truth` against `interval`. A truth that isn't a number misses on both sides.
    fn add(&mut self, interval: Interval, truth: f64) {
        self.given += 1;
        let below = truth.is_nan() || truth < interval.low;
        let above = truth.is_nan() || truth > interval.high;
        self.below += u64::from(below);
        self.above += u64::from(above);
        self.inside += u64::from(!below && !above);
    }
}

/// A share's two-sided 99.9% Clopper–Pearson band.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Band {
    /// Its lower edge.
    pub low: f64,
    /// Its upper edge.
    pub high: f64,
}

impl Band {
    /// The band on `x` of `n`; NaN, which holds nothing, when `n` is 0.
    fn of(x: u64, n: u64) -> Self {
        if n == 0 {
            return Self {
                low: f64::NAN,
                high: f64::NAN,
            };
        }
        Self {
            low: clopper_pearson_lower(x, n, BAND_CONFIDENCE),
            high: clopper_pearson_upper(x, n, BAND_CONFIDENCE),
        }
    }

    /// Whether it holds `share`. A band that isn't a number holds nothing.
    pub fn holds(&self, share: f64) -> bool {
        self.low <= share && share <= self.high
    }
}

/// The coverage test on one set of counts: each tail's band must hold 5%, the inside's 90%.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Test {
    /// The counts.
    pub counts: Counts,
    /// The band on the share below.
    pub below: Band,
    /// The band on the share inside.
    pub inside: Band,
    /// The band on the share above.
    pub above: Band,
}

impl Test {
    /// The test on `counts`.
    pub fn of(counts: Counts) -> Self {
        Self {
            counts,
            below: Band::of(counts.below, counts.given),
            inside: Band::of(counts.inside, counts.given),
            above: Band::of(counts.above, counts.given),
        }
    }

    /// Whether it passes: each tail's band holds 5% and the inside's 90%.
    pub fn passes(&self) -> bool {
        self.below.holds(MISS_SHARE)
            && self.above.holds(MISS_SHARE)
            && self.inside.holds(INSIDE_SHARE)
    }
}

/// One reading's check on one set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Check {
    /// The reading.
    pub quantity: Quantity,
    /// Flights flown.
    pub flights: u64,
    /// Read with no interval: floors.
    pub unstated: u64,
    /// Withheld, by reason.
    pub withheld: BTreeMap<Reason, u64>,
    /// The test on the intervals as stated.
    pub test: Test,
    /// The test on each probe's intervals; each must fail.
    pub probes: BTreeMap<Probe, Test>,
    /// The counts by design.
    pub by_design: BTreeMap<String, Counts>,
    /// The counts by logger.
    pub by_logger: BTreeMap<String, Counts>,
    /// The counts by the day's air, in [`AirBand`]'s order.
    pub by_air: BTreeMap<AirBand, Counts>,
    /// The mean error, reading less truth, over the readings with an interval, with its sign
    /// (the apogee's error side, which a reading too low flatters).
    pub mean_error: f64,
    /// The largest error's size, `|reading − truth|`, over the same readings: the scale the mean
    /// is a small difference on. Not a number if any truth isn't one.
    pub largest_error: f64,
}

impl Check {
    /// Whether it comes out as it must: the test passes and every probe fails it.
    pub fn holds(&self) -> bool {
        self.test.passes() && self.probes.values().all(|t| !t.passes())
    }
}

/// `quantity`'s check over `flights`.
pub fn check(quantity: Quantity, flights: &[FlightSaid]) -> Check {
    let mut counts = Counts::default();
    let mut probes: BTreeMap<Probe, Counts> =
        Probe::ALL.iter().map(|&p| (p, Counts::default())).collect();
    let mut by_design: BTreeMap<String, Counts> = BTreeMap::new();
    let mut by_logger: BTreeMap<String, Counts> = BTreeMap::new();
    let mut by_air: BTreeMap<AirBand, Counts> = BTreeMap::new();
    let mut withheld: BTreeMap<Reason, u64> = BTreeMap::new();
    let mut unstated = 0;
    let mut error_sum = 0.0;
    let mut largest_error = 0.0_f64;
    for flight in flights {
        for (q, said) in &flight.said {
            if *q != quantity {
                continue;
            }
            match said {
                Said::Stated(c) => {
                    counts.add(c.interval, c.truth);
                    for (probe, counted) in &mut probes {
                        counted.add(probe.apply(c.interval), c.truth);
                    }
                    by_design
                        .entry(DESIGNS[flight.design].name.to_owned())
                        .or_default()
                        .add(c.interval, c.truth);
                    by_logger
                        .entry(flight.logger.to_owned())
                        .or_default()
                        .add(c.interval, c.truth);
                    by_air
                        .entry(AirBand::of(flight.air_offset_k))
                        .or_default()
                        .add(c.interval, c.truth);
                    let error = c.reading - c.truth;
                    error_sum += error;
                    // `f64::max` would drop a NaN; a NaN, once in, stays.
                    if error.is_nan() || error.abs() > largest_error {
                        largest_error = error.abs();
                    }
                }
                Said::Unstated => unstated += 1,
                Said::Withheld(reason) => *withheld.entry(*reason).or_default() += 1,
            }
        }
    }
    Check {
        quantity,
        flights: flights.len() as u64,
        unstated,
        withheld,
        test: Test::of(counts),
        probes: probes.into_iter().map(|(p, c)| (p, Test::of(c))).collect(),
        by_design,
        by_logger,
        by_air,
        mean_error: if counts.given == 0 {
            f64::NAN
        } else {
            error_sum / counts.given as f64
        },
        largest_error: if counts.given == 0 {
            f64::NAN
        } else {
            largest_error
        },
    }
}

/// `quantity`'s shares fitted again on `flights`, which should be [`Set::Calibration`](super::Set::Calibration)'s; `None`
/// when fewer than 20 state an interval or a share isn't a number.
pub fn fitted(quantity: Quantity, flights: &[FlightSaid]) -> Option<Shares> {
    let shares: Vec<f64> = flights
        .iter()
        .flat_map(|f| &f.said)
        .filter_map(|(q, said)| match said {
            Said::Stated(c) if *q == quantity => Some(c.share()),
            _ => None,
        })
        .collect();
    interval::fit(&shares)
}

/// `x` to three significant figures, as the library's shares are written.
pub fn three_figures(x: f64) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let scale = 10f64.powi(2 - x.abs().log10().floor() as i32);
    (x * scale).round() / scale
}

/// A band of the day's air, K from the 1976 standard atmosphere. A barometric height is off by
/// about `ΔT/T` of itself, so the height's errors, and its misses, gather at the ends of the
/// synthetic flights' ±15 K: a warm day reads low, a cold one high.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AirBand {
    /// Colder than −10 K.
    Cold,
    /// From −10 K to +10 K.
    Middle,
    /// Warmer than +10 K.
    Warm,
}

impl AirBand {
    /// Every band, coldest first.
    pub const ALL: [Self; 3] = [Self::Cold, Self::Middle, Self::Warm];

    /// The band `offset_k` lies in.
    pub fn of(offset_k: f64) -> Self {
        if offset_k < -10.0 {
            Self::Cold
        } else if offset_k > 10.0 {
            Self::Warm
        } else {
            Self::Middle
        }
    }

    /// Its name in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cold => "colder than −10 K",
            Self::Middle => "−10 to +10 K",
            Self::Warm => "warmer than +10 K",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compared(truth: f64) -> Said {
        Said::Stated(Compared {
            reading: 0.0,
            bound: 1.0,
            interval: Interval {
                low: -Z_90,
                high: Z_90,
            },
            truth,
        })
    }

    /// 1,000 flights whose truths sit at the standard normal's quantiles about a reading of 0
    /// with the interval ±`Z_90`: exactly what a correct 90% interval sees.
    fn normal_flights() -> Vec<FlightSaid> {
        (0..1000)
            .map(|k| {
                let p = (k as f64 + 0.5) / 1000.0;
                FlightSaid {
                    index: k,
                    design: (k % 10) as usize,
                    logger: "altos",
                    air_offset_k: 0.0,
                    said: vec![(Quantity::ApogeeHeight, compared(normal_quantile(p)))],
                }
            })
            .collect()
    }

    /// The standard normal's quantile by bisection on its distribution, `½ erfc(−x/√2)`.
    fn normal_quantile(p: f64) -> f64 {
        let cdf = |x: f64| 0.5 * erfc(-x / std::f64::consts::SQRT_2);
        let (mut lo, mut hi) = (-10.0, 10.0);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if cdf(mid) < p {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    /// `erfc` by its Chebyshev fit, fractional error below 1.2e-7 everywhere (W. H. Press et al.,
    /// *Numerical Recipes in C*, 2nd ed., 1992, §6.2, `erfcc`).
    fn erfc(x: f64) -> f64 {
        let z = x.abs();
        let t = 2.0 / (2.0 + z);
        let ans = t
            * (-z * z - 1.265_512_23
                + t * (1.000_023_68
                    + t * (0.374_091_96
                        + t * (0.096_784_18
                            + t * (-0.186_288_06
                                + t * (0.278_868_07
                                    + t * (-1.135_203_98
                                        + t * (1.488_515_87
                                            + t * (-0.822_152_23 + t * 0.170_872_77)))))))))
                .exp();
        if x >= 0.0 { ans } else { 2.0 - ans }
    }

    #[test]
    fn a_correct_interval_passes_and_every_probe_fails() {
        let flights = normal_flights();
        let check = check(Quantity::ApogeeHeight, &flights);
        assert_eq!(check.test.counts.given, 1000);
        assert_eq!(check.test.counts.below, 50);
        assert_eq!(check.test.counts.above, 50);
        assert!(check.test.passes());
        for (probe, test) in &check.probes {
            assert!(!test.passes(), "{probe:?}: {:?}", test.counts);
        }
        assert!(check.holds());
        assert_eq!(check.by_design.len(), 10);
        assert!(check.mean_error.abs() < 1e-3, "{}", check.mean_error);
        // The farthest of the 1,000 points is the 0.05% one.
        let farthest = normal_quantile(0.0005)
            .abs()
            .max(normal_quantile(0.9995).abs());
        assert_eq!(check.largest_error, farthest);
        // The narrowed probe holds about 81%, the widened about 96%, and the shifted one misses
        // above about 26% of the time.
        let narrowed = check.probes[&Probe::Narrowed].counts;
        assert!(
            (185..=195).contains(&(narrowed.below + narrowed.above)),
            "{narrowed:?}"
        );
        let widened = check.probes[&Probe::Widened].counts;
        assert!(
            (35..=45).contains(&(widened.below + widened.above)),
            "{widened:?}"
        );
        let shifted = check.probes[&Probe::ShiftedDown].counts;
        assert!((255..=265).contains(&shifted.above), "{shifted:?}");
    }

    #[test]
    fn a_one_sided_miss_fails_on_its_own_tail() {
        // Shift every truth up by 0.3: the inside count stays near 90%, the tails don't.
        let mut flights = normal_flights();
        for f in &mut flights {
            if let (_, Said::Stated(c)) = &mut f.said[0] {
                c.truth += 0.3;
            }
        }
        let check = check(Quantity::ApogeeHeight, &flights);
        assert!(!check.test.passes(), "{:?}", check.test.counts);
        assert!(!check.test.above.holds(MISS_SHARE));
        assert!(!check.test.below.holds(MISS_SHARE));
    }

    #[test]
    fn a_truth_that_is_not_a_number_leaves_no_error_scale() {
        let mut flights = normal_flights();
        flights[3].said = vec![(Quantity::ApogeeHeight, compared(f64::NAN))];
        let check = check(Quantity::ApogeeHeight, &flights);
        assert!(check.mean_error.is_nan());
        assert!(check.largest_error.is_nan(), "{}", check.largest_error);
    }

    #[test]
    fn a_truth_that_is_not_a_number_misses_both_ways() {
        let mut counts = Counts::default();
        counts.add(
            Interval {
                low: 0.0,
                high: 1.0,
            },
            f64::NAN,
        );
        assert_eq!(
            (counts.given, counts.below, counts.inside, counts.above),
            (1, 1, 0, 1)
        );
        counts.add(
            Interval {
                low: 0.0,
                high: 1.0,
            },
            1.0,
        );
        assert_eq!(counts.inside, 1);
    }

    #[test]
    fn floors_and_withheld_readings_are_counted_apart() {
        let flights = vec![
            FlightSaid {
                index: 0,
                design: 0,
                logger: "altos",
                air_offset_k: 0.0,
                said: vec![(Quantity::ApogeeHeight, Said::Unstated)],
            },
            FlightSaid {
                index: 1,
                design: 0,
                logger: "altos",
                air_offset_k: 0.0,
                said: vec![(Quantity::ApogeeHeight, Said::Withheld(Reason::NoClimb))],
            },
        ];
        let check = check(Quantity::ApogeeHeight, &flights);
        assert_eq!(
            (check.flights, check.unstated, check.test.counts.given),
            (2, 1, 0)
        );
        assert_eq!(check.withheld[&Reason::NoClimb], 1);
        // No reading compared: nothing to pass on.
        assert!(!check.test.passes());
    }

    #[test]
    fn the_fit_reads_the_errors_as_shares_of_the_bound() {
        let flights: Vec<FlightSaid> = (0..199)
            .map(|k| FlightSaid {
                index: k,
                design: 0,
                logger: "altos",
                air_offset_k: 0.0,
                said: vec![(
                    Quantity::ApogeeHeight,
                    Said::Stated(Compared {
                        reading: k as f64,
                        bound: 2.0,
                        interval: Interval {
                            low: 0.0,
                            high: 1.0,
                        },
                        truth: 0.0,
                    }),
                )],
            })
            .collect();
        // Shares 0, 0.5, …, 99: n = 199, the 10th from each end.
        let fitted = fitted(Quantity::ApogeeHeight, &flights).unwrap();
        assert_eq!((fitted.low, fitted.high), (4.5, 94.5));
        assert!(fitted_none_below_twenty());
    }

    fn fitted_none_below_twenty() -> bool {
        fitted(Quantity::ApogeeTime, &[]).is_none()
    }

    #[test]
    fn three_figures_rounds_as_the_library_writes() {
        assert_eq!(three_figures(0.612_49), 0.612);
        assert_eq!(three_figures(-0.000_123_56), -0.000_124);
        assert_eq!(three_figures(12_345.0), 12_300.0);
        assert_eq!(three_figures(0.0), 0.0);
    }
}
