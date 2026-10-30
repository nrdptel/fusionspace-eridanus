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
//! A floor's truth may lie any amount above it, so it states only a lower end, tested on its lower
//! tail alone: the band on the share of truths below it must hold 5%, raised one stated standard
//! error and lowered two it must not ([`FloorCheck`]). A top acceleration's floors come from logs
//! clipping at 8 g, its top scaled to lie at the clip, where a lower end is hardest to hold, and
//! its top as flown, which may miss less but not more ([`top_over_clip`]).
//!
//! The intervals' shares are fitted on [`Set::Calibration`](super::Set::Calibration) ([`fitted`]); the test runs on
//! [`Set::A`](super::Set::A) and [`Set::B`](super::Set::B), which the shares never see.
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-308]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md

use std::collections::BTreeMap;

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::interval::{self, FloorShares, Interval, Shares, Z_90};
use hpr_flightdata::readings::{self, Reading, Reason};
use serde::{Deserialize, Serialize};

use super::designs::DESIGNS;
use super::held::{Quantity, top_specific_force};
use super::logger::{LOWER_RANGE_G, accelerometer_log_clipping_at, accelerometer_log_top_scaled};
use super::smoothed::{BAND_CONFIDENCE, MISS_SHARE, clopper_pearson_lower, clopper_pearson_upper};
use super::{Flight, SyntheticError};

/// The readings that state a 90% interval, in the report's order.
pub const STATED: [Quantity; 12] = [
    Quantity::Liftoff,
    Quantity::ApogeeHeight,
    Quantity::ApogeeTime,
    Quantity::TopSpeed,
    Quantity::Landing,
    Quantity::FlightTime,
    Quantity::DescentRate,
    Quantity::TopAcceleration,
    Quantity::Burnout,
    Quantity::DrogueRate,
    Quantity::MainOpening,
    Quantity::MainRate,
];

/// The share of readings whose truth the interval must hold.
pub const INSIDE_SHARE: f64 = 0.90;

/// The readings that state a lower end where they are a floor, in the report's order.
pub const FLOORED: [Quantity; 2] = [Quantity::TopAcceleration, Quantity::Burnout];

/// The library's shares for `quantity`; `None` for a reading that states no interval yet.
pub fn shares(quantity: Quantity) -> Option<Shares> {
    match quantity {
        Quantity::ApogeeHeight => Some(interval::APOGEE_HEIGHT_SHARES),
        Quantity::ApogeeTime => Some(interval::APOGEE_TIME_SHARES),
        Quantity::Liftoff => Some(interval::LIFTOFF_SHARES),
        Quantity::TopSpeed => Some(interval::TOP_SPEED_SHARES),
        Quantity::Landing => Some(interval::LANDING_SHARES),
        Quantity::FlightTime => Some(interval::FLIGHT_TIME_SHARES),
        Quantity::DescentRate => Some(interval::DESCENT_RATE_SHARES),
        Quantity::TopAcceleration => Some(interval::TOP_ACCELERATION_SHARES),
        Quantity::Burnout => Some(interval::BURNOUT_SHARES),
        Quantity::DrogueRate => Some(interval::DROGUE_RATE_SHARES),
        Quantity::MainOpening => Some(interval::MAIN_OPENING_SHARES),
        Quantity::MainRate => Some(interval::MAIN_RATE_SHARES),
    }
}

/// The library's floor shares for `quantity`; `None` for a reading that is never a floor here.
pub fn floor_shares(quantity: Quantity) -> Option<FloorShares> {
    match quantity {
        Quantity::TopAcceleration => Some(interval::TOP_ACCELERATION_FLOOR_SHARES),
        Quantity::Burnout => Some(interval::BURNOUT_FLOOR_SHARES),
        _ => None,
    }
}

/// Whether `quantity` withheld for `reason` from a log with a barometer is the reading's own rule
/// and not a reading lost: the top speed is withheld at or above
/// `hpr_flightdata::readings::TRANSONIC_SPEED_M_S`, 227.7 m/s, where the barometer's port may
/// read the transonic flow over the nose. The rule looks at the reading, not the truth, so the
/// test holds the readings it lets through, as a user sees them. A burnout taken from that speed
/// goes with it, and a log with no accelerometer gives no top acceleration.
pub fn withheld_by_rule(quantity: Quantity, reason: Reason) -> bool {
    matches!(
        (quantity, reason),
        (Quantity::TopSpeed | Quantity::Burnout, Reason::Transonic)
            | (Quantity::TopAcceleration, Reason::NoAccelerometer)
    )
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

/// One floor's lower end against its truth.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ComparedFloor {
    /// The reading.
    pub reading: f64,
    /// The bound it states.
    pub bound: f64,
    /// Its lower end.
    pub low: f64,
    /// The lower end's standard error.
    pub standard_error: f64,
    /// The truth.
    pub truth: f64,
}

impl ComparedFloor {
    /// Its error as a share of its bound, `(reading − truth)/bound`: what its share is fitted to.
    pub fn share(&self) -> f64 {
        (self.reading - self.truth) / self.bound
    }
}

/// What one log said of one reading.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Said {
    /// Read, with a 90% interval.
    Stated(Compared),
    /// Read as a floor, with its lower end.
    Floor(ComparedFloor),
    /// Read with neither: a bound no interval can be formed from.
    Unstated,
    /// A top acceleration's floor on the flight's log with its top as flown, past the clip by as
    /// much as its design takes it: checked apart, not fitted or tested as [`Said::Floor`] is.
    PastClip(ComparedFloor),
    /// None to read: a drogue, or a main's opening, on a flight that came down under one canopy
    /// or none.
    Absent,
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

/// What a log read of one reading, with its truth.
struct Part {
    reading: f64,
    bound: f64,
    interval: Option<Interval>,
    lower_end: Option<f64>,
    truth: f64,
}

impl Part {
    /// A reading that is never a floor.
    fn of(reading: f64, bound: f64, interval: Option<Interval>, truth: f64) -> Self {
        Self {
            reading,
            bound,
            interval,
            lower_end: None,
            truth,
        }
    }
}

/// What `flight`'s own log says of each of [`STATED`], against its truth.
///
/// # Errors
///
/// [`SyntheticError`] if the log the top acceleration's floors are read from can't be written.
pub fn said(seed: u64, index: u64, flight: &Flight) -> Result<FlightSaid, SyntheticError> {
    let read = readings::read(&flight.log);
    let truth = &flight.truth;
    let pad_s = flight.draw.logger.pad_s;
    let events = &truth.events;
    // The truths are `held::outcomes`' own: times on the log's clock, the apogee above the pad,
    // the mean descent rate the apogee's height over the time from apogee to landing, and a leg's
    // rate the true height lost over the leg the reading names.
    let descent_rate = events.apogee_m / (events.landing_s - events.apogee_s);
    let height = |t_log: f64| truth.at(t_log - pad_s).map_or(f64::NAN, |x| x.height_m);
    let chord = |from_s: f64, to_s: f64| (height(from_s) - height(to_s)) / (to_s - from_s);
    let deployments = events.deployments.len();
    let two_canopies = deployments >= 2;
    fn of<T>(reading: &Reading<T>, take: impl Fn(&T) -> Part) -> Result<Part, Said> {
        match reading {
            Reading::Read(value) => Ok(take(value)),
            Reading::Withheld(withheld) => Err(Said::Withheld(withheld.reason)),
        }
    }
    let mut said: Vec<(Quantity, Said)> = STATED
        .iter()
        .map(|&quantity| {
            let part = match quantity {
                Quantity::Liftoff => of(&read.liftoff, |l| {
                    Part::of(l.time_s, l.bound_s, l.interval_s, pad_s + events.liftoff_s)
                }),
                Quantity::ApogeeHeight => of(&read.apogee, |a| {
                    Part::of(
                        a.height_m,
                        a.bound_m,
                        a.height_interval_m,
                        truth.apogee_above_pad_m(),
                    )
                }),
                Quantity::ApogeeTime => of(&read.apogee, |a| {
                    Part::of(
                        a.time_s,
                        a.time_bound_s,
                        a.time_interval_s,
                        pad_s + events.apogee_s,
                    )
                }),
                Quantity::TopSpeed => of(&read.max_speed, |s| {
                    Part::of(
                        s.speed_m_s,
                        s.bound_m_s,
                        s.interval_m_s,
                        truth.top_speed().0,
                    )
                }),
                Quantity::Landing => of(&read.landing, |l| {
                    Part::of(l.time_s, l.bound_s, l.interval_s, pad_s + events.landing_s)
                }),
                Quantity::FlightTime => of(&read.flight_time, |f| {
                    Part::of(
                        f.time_s,
                        f.bound_s,
                        f.interval_s,
                        events.landing_s - events.liftoff_s,
                    )
                }),
                Quantity::DescentRate => of(&read.landing, |l| {
                    Part::of(
                        l.mean_descent_rate_m_s,
                        l.mean_descent_rate_bound_m_s,
                        l.mean_descent_rate_interval_m_s,
                        descent_rate,
                    )
                }),
                Quantity::TopAcceleration => of(&read.max_acceleration, |a| Part {
                    reading: a.acceleration_m_s2,
                    bound: a.bound_m_s2,
                    interval: a.interval_m_s2,
                    lower_end: a.lower_end_m_s2,
                    truth: top_specific_force(flight),
                }),
                Quantity::Burnout => of(&read.burnout, |b| Part {
                    reading: b.time_s,
                    bound: b.bound_s,
                    interval: b.interval_s,
                    lower_end: b.lower_end_s,
                    truth: pad_s + events.burnout_s,
                }),
                Quantity::DrogueRate => match &read.drogue_descent {
                    Reading::Withheld(withheld)
                        if withheld.reason == Reason::OneCanopy && !two_canopies =>
                    {
                        Err(Said::Absent)
                    }
                    drogue => of(drogue, |d| {
                        Part::of(
                            d.rate_m_s,
                            d.bound_m_s,
                            d.interval_m_s,
                            chord(d.from_s, d.to_s),
                        )
                    }),
                },
                Quantity::MainOpening => match &read.main_descent {
                    Reading::Withheld(withheld) => Err(Said::Withheld(withheld.reason)),
                    Reading::Read(main) => match main.opening {
                        // A main named where the flight opened one canopy, or none, misses both
                        // ways.
                        Some(opening) => Ok(Part::of(
                            opening.time_s,
                            opening.bound_s,
                            opening.interval_s,
                            if two_canopies {
                                pad_s + events.deployments[deployments - 1].1
                            } else {
                                f64::NAN
                            },
                        )),
                        // One canopy read where the flight opened two.
                        None if two_canopies => Err(Said::Withheld(Reason::OneCanopy)),
                        None => Err(Said::Absent),
                    },
                },
                Quantity::MainRate => of(&read.main_descent, |d| {
                    Part::of(
                        d.rate_m_s,
                        d.bound_m_s,
                        d.interval_m_s,
                        chord(d.from_s, d.to_s),
                    )
                }),
            };
            let said = match part {
                Err(said) => said,
                Ok(Part {
                    reading,
                    bound,
                    interval: Some(interval),
                    truth,
                    ..
                }) => Said::Stated(Compared {
                    reading,
                    bound,
                    interval,
                    truth,
                }),
                Ok(Part {
                    reading,
                    bound,
                    lower_end: Some(low),
                    truth,
                    ..
                }) => floored(quantity, reading, bound, low, truth),
                Ok(_) => Said::Unstated,
            };
            (quantity, said)
        })
        .collect();
    // No flight's own accelerometer clips, so a top acceleration's floors come from logs that do.
    // Only a floor counts from them: a top the rule reads as whole is no floor.
    let top = top_specific_force(flight);
    if top > STANDARD_GRAVITY_MPS2 {
        let range = LOWER_RANGE_G * STANDARD_GRAVITY_MPS2;
        let at_clip = range * top_over_clip(index);
        let log = accelerometer_log_top_scaled(
            &flight.draw.logger,
            truth,
            (top, at_clip),
            LOWER_RANGE_G,
            seed,
            index,
        )?;
        if let Some(floor) = log.as_ref().and_then(|log| clipped_top(log, at_clip)) {
            said.push((Quantity::TopAcceleration, Said::Floor(floor)));
        }
        let log =
            accelerometer_log_clipping_at(&flight.draw.logger, truth, LOWER_RANGE_G, seed, index)?;
        if let Some(floor) = log.as_ref().and_then(|log| clipped_top(log, top)) {
            said.push((Quantity::TopAcceleration, Said::PastClip(floor)));
        }
    }
    Ok(FlightSaid {
        index,
        design: flight.draw.design,
        logger: flight.draw.logger.profile.name(),
        air_offset_k: flight.draw.temperature_offset_k,
        said,
    })
}

/// How far flight `index`'s top lands from the clip in the log its lower end is fitted on: its
/// top over the range, spread evenly over 0.95 to 1.05 by the golden ratio's multiples. The reader
/// calls a top a floor from a little below the range, as a part's 3% scale can carry a top under
/// it up to it, and a top just there is where a lower end is hardest to hold. Taking it for each
/// flight apart keeps a design's own thrust out of the fit.
pub fn top_over_clip(index: u64) -> f64 {
    0.95 + 0.10 * (index as f64 * 0.618_033_988_749_894_9).fract()
}

/// The floor `log` reads of its top acceleration against the `truth`, where it reads one.
fn clipped_top(log: &FlightLog, truth: f64) -> Option<ComparedFloor> {
    let Reading::Read(top) = readings::read(log).max_acceleration else {
        return None;
    };
    let low = top.lower_end_m_s2.filter(|_| top.is_floor)?;
    match floored(
        Quantity::TopAcceleration,
        top.acceleration_m_s2,
        top.bound_m_s2,
        low,
        truth,
    ) {
        Said::Floor(floor) => Some(floor),
        _ => None,
    }
}

/// A floor with its lower end's standard error. A quantity with no floor shares states none, so
/// its probes don't move it.
fn floored(quantity: Quantity, reading: f64, bound: f64, low: f64, truth: f64) -> Said {
    Said::Floor(ComparedFloor {
        reading,
        bound,
        low,
        standard_error: floor_shares(quantity).map_or(0.0, |shares| shares.standard_error(bound)),
        truth,
    })
}

/// A change to every floor's lower end that its test must catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FloorProbe {
    /// Raised by one stated standard error.
    Raised,
    /// Lowered by two stated standard errors. One is too little to fail on a few hundred floors:
    /// of 210 the band holds 5% down to 2 truths below, and a floor's lower side has a long tail.
    Lowered,
}

impl FloorProbe {
    /// Every probe.
    pub const ALL: [Self; 2] = [Self::Raised, Self::Lowered];

    /// Its name in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::Raised => "raised one standard error",
            Self::Lowered => "lowered two standard errors",
        }
    }

    /// `floor`'s lower end, changed.
    pub fn apply(self, floor: &ComparedFloor) -> f64 {
        match self {
            Self::Raised => floor.low + floor.standard_error,
            Self::Lowered => floor.low - 2.0 * floor.standard_error,
        }
    }
}

/// The test of a floor's lower end, the coverage test's lower tail: the band on the share of
/// truths below it must hold 5%.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FloorTest {
    /// Floors compared.
    pub given: u64,
    /// The truth below the lower end. A truth that isn't a number counts.
    pub below: u64,
    /// The band on the share below.
    pub band: Band,
}

impl FloorTest {
    /// The test on `below` of `given`.
    fn of(given: u64, below: u64) -> Self {
        Self {
            given,
            below,
            band: Band::of(below, given),
        }
    }

    /// Whether it passes: the band holds 5%.
    pub fn passes(&self) -> bool {
        self.band.holds(MISS_SHARE)
    }

    /// Whether its band reaches down to 5% or lower: the truths below it aren't more than 5%.
    pub fn no_more_than_its_share(&self) -> bool {
        self.band.low <= MISS_SHARE
    }
}

/// One reading's floors on one set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloorCheck {
    /// The test on the lower ends as stated.
    pub test: FloorTest,
    /// The test on each probe's lower ends; each must fail.
    pub probes: BTreeMap<FloorProbe, FloorTest>,
    /// Compared and below, by logger.
    pub by_logger: BTreeMap<String, (u64, u64)>,
}

impl FloorCheck {
    /// Whether it comes out as it must: the test passes and every probe fails it.
    pub fn holds(&self) -> bool {
        self.test.passes() && self.probes.values().all(|t| !t.passes())
    }

    /// The check over `floors`, each with its logger; `None` when there are none.
    fn of<'a>(floors: impl Iterator<Item = (&'a str, &'a ComparedFloor)>) -> Option<Self> {
        let below = |low: f64, truth: f64| truth.is_nan() || truth < low;
        let (mut given, mut misses) = (0, 0);
        let mut probed: BTreeMap<FloorProbe, u64> =
            FloorProbe::ALL.iter().map(|&p| (p, 0)).collect();
        let mut by_logger: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        for (logger, floor) in floors {
            let missed = u64::from(below(floor.low, floor.truth));
            given += 1;
            misses += missed;
            let entry = by_logger.entry(logger.to_owned()).or_default();
            entry.0 += 1;
            entry.1 += missed;
            for (probe, count) in &mut probed {
                *count += u64::from(below(probe.apply(floor), floor.truth));
            }
        }
        (given > 0).then(|| Self {
            test: FloorTest::of(given, misses),
            probes: probed
                .into_iter()
                .map(|(p, x)| (p, FloorTest::of(given, x)))
                .collect(),
            by_logger,
        })
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
    /// Read with neither an interval nor a lower end.
    pub unstated: u64,
    /// Read as floors.
    pub floors: u64,
    /// The floors' test, where any were read.
    pub floor: Option<FloorCheck>,
    /// The top acceleration's floors with their tops as flown, past the clip by as much as a
    /// design takes them: a lower end fitted at the clip must miss no more than 5% of them.
    pub past_clip: Option<FloorTest>,
    /// None to read.
    pub absent: u64,
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
        self.test.passes()
            && self.probes.values().all(|t| !t.passes())
            && self.floor.as_ref().is_none_or(FloorCheck::holds)
            && self
                .past_clip
                .as_ref()
                .is_none_or(FloorTest::no_more_than_its_share)
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
    let mut floors = 0;
    let mut absent = 0;
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
                Said::Floor(_) => floors += 1,
                Said::PastClip(_) => {}
                Said::Unstated => unstated += 1,
                Said::Absent => absent += 1,
                Said::Withheld(reason) => *withheld.entry(*reason).or_default() += 1,
            }
        }
    }
    Check {
        quantity,
        flights: flights.len() as u64,
        unstated,
        floors,
        past_clip: {
            let (mut given, mut below) = (0, 0);
            for (q, said) in flights.iter().flat_map(|f| &f.said) {
                if let (true, Said::PastClip(floor)) = (*q == quantity, said) {
                    given += 1;
                    below += u64::from(floor.truth.is_nan() || floor.truth < floor.low);
                }
            }
            (given > 0).then(|| FloorTest::of(given, below))
        },
        floor: FloorCheck::of(flights.iter().flat_map(|flight| {
            flight.said.iter().filter_map(move |(q, said)| match said {
                Said::Floor(floor) if *q == quantity => Some((flight.logger, floor)),
                _ => None,
            })
        })),
        absent,
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

/// `quantity`'s floor shares fitted again on `flights`; `None` when fewer than 20 floors are read
/// or a share isn't a number.
pub fn fitted_floor(quantity: Quantity, flights: &[FlightSaid]) -> Option<FloorShares> {
    let shares: Vec<f64> = flights
        .iter()
        .flat_map(|f| &f.said)
        .filter_map(|(q, said)| match said {
            Said::Floor(floor) if *q == quantity => Some(floor.share()),
            _ => None,
        })
        .collect();
    interval::fit_floor(&shares)
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

    /// Withheld by its own rule: the top speed past its transonic mark, a burnout taken from it,
    /// and a top acceleration where the log has no accelerometer; no reading for another reason.
    #[test]
    fn a_reading_withheld_by_rule_is_named() {
        for (quantity, reason) in [
            (Quantity::TopSpeed, Reason::Transonic),
            (Quantity::Burnout, Reason::Transonic),
            (Quantity::TopAcceleration, Reason::NoAccelerometer),
        ] {
            assert!(withheld_by_rule(quantity, reason), "{quantity:?}");
        }
        assert!(!withheld_by_rule(Quantity::TopSpeed, Reason::NoClimb));
        for quantity in STATED {
            assert!(
                !withheld_by_rule(quantity, Reason::EndsBeforeLanding),
                "{quantity:?}"
            );
            if !matches!(quantity, Quantity::TopSpeed | Quantity::Burnout) {
                assert!(
                    !withheld_by_rule(quantity, Reason::Transonic),
                    "{quantity:?}"
                );
            }
            if quantity != Quantity::TopAcceleration {
                assert!(
                    !withheld_by_rule(quantity, Reason::NoAccelerometer),
                    "{quantity:?}"
                );
            }
        }
    }

    /// 1,000 floors whose errors sit at the standard normal's quantiles: `reading − truth` from
    /// −3 to +3 about −1.645, the lower end at the 5% tail, its standard error 1.
    fn normal_floors() -> Vec<FlightSaid> {
        (0..1000)
            .map(|k| {
                let p = (k as f64 + 0.5) / 1000.0;
                FlightSaid {
                    index: k,
                    design: 0,
                    logger: if k % 2 == 0 { "altos" } else { "rrc3" },
                    air_offset_k: 0.0,
                    said: vec![(
                        Quantity::Burnout,
                        Said::Floor(ComparedFloor {
                            reading: 0.0,
                            bound: 1.0,
                            low: -Z_90,
                            standard_error: 1.0,
                            truth: normal_quantile(p),
                        }),
                    )],
                }
            })
            .collect()
    }

    #[test]
    fn a_correct_floor_passes_on_its_lower_tail_and_its_probes_fail() {
        let check = check(Quantity::Burnout, &normal_floors());
        assert_eq!(check.floors, 1000);
        // A floor isn't compared as an interval is.
        assert_eq!(check.test.counts.given, 0);
        let floor = check.floor.as_ref().unwrap();
        assert_eq!((floor.test.given, floor.test.below), (1000, 50));
        assert!(floor.test.passes());
        assert!(floor.holds());
        // Raised one standard error: about 26% below. Lowered two: about 0.1%.
        let raised = floor.probes[&FloorProbe::Raised];
        assert!((250..=270).contains(&raised.below), "{raised:?}");
        assert!(!raised.passes());
        let lowered = floor.probes[&FloorProbe::Lowered];
        assert!(lowered.below <= 3, "{lowered:?}");
        assert!(!lowered.passes());
        assert_eq!(floor.by_logger["altos"].0, 500);
    }

    #[test]
    fn a_floor_missing_too_often_or_too_rarely_fails() {
        for (shift, tail) in [(-1.0, "too often"), (2.0, "too rarely")] {
            let mut flights = normal_floors();
            for f in &mut flights {
                if let (_, Said::Floor(c)) = &mut f.said[0] {
                    // Higher truths leave fewer below the lower end.
                    c.truth += shift;
                }
            }
            let floor = check(Quantity::Burnout, &flights).floor.unwrap();
            assert!(!floor.test.passes(), "{tail}: {:?}", floor.test);
            assert!(!floor.holds(), "{tail}");
        }
    }

    #[test]
    fn a_floors_truth_that_is_not_a_number_counts_below() {
        let mut flights = normal_floors();
        if let (_, Said::Floor(c)) = &mut flights[500].said[0] {
            c.truth = f64::NAN;
        }
        let floor = check(Quantity::Burnout, &flights).floor.unwrap();
        assert_eq!(floor.test.below, 51);
        // No normal quantile of the 1,000 lies as low as the lowered end: only the NaN does.
        assert_eq!(floor.probes[&FloorProbe::Lowered].below, 1);
    }

    /// Tops past the clip are checked apart: never fitted, and failing only by missing more than
    /// their share. All 200 missing fails; none missing doesn't.
    #[test]
    fn tops_past_the_clip_may_miss_less_than_their_share_but_not_more() {
        let past = |truth: f64| -> Vec<FlightSaid> {
            (0..200)
                .map(|k| FlightSaid {
                    index: k,
                    design: 0,
                    logger: "altos",
                    air_offset_k: 0.0,
                    said: vec![(
                        Quantity::TopAcceleration,
                        Said::PastClip(ComparedFloor {
                            reading: 0.0,
                            bound: 1.0,
                            low: 0.0,
                            standard_error: 1.0,
                            truth,
                        }),
                    )],
                })
                .collect()
        };
        let none = check(Quantity::TopAcceleration, &past(1.0));
        let test = none.past_clip.unwrap();
        assert_eq!((test.given, test.below), (200, 0));
        assert!(test.no_more_than_its_share());
        assert!(fitted_floor(Quantity::TopAcceleration, &past(1.0)).is_none());
        let all = check(Quantity::TopAcceleration, &past(-1.0));
        let test = all.past_clip.unwrap();
        assert!(!test.no_more_than_its_share(), "{test:?}");
        // Exactly its share: the band reaches 5% from both sides, and that doesn't fail.
        let mut some = past(1.0);
        for flight in &mut some[..10] {
            flight.said = past(-1.0)[0].said.clone();
        }
        let test = check(Quantity::TopAcceleration, &some).past_clip.unwrap();
        assert_eq!(test.below, 10);
        assert!(test.band.low < MISS_SHARE && test.band.high > MISS_SHARE);
        assert!(test.no_more_than_its_share(), "{test:?}");
    }

    #[test]
    fn a_flight_with_no_canopy_to_read_is_counted_apart() {
        let flights = vec![FlightSaid {
            index: 0,
            design: 0,
            logger: "altos",
            air_offset_k: 0.0,
            said: vec![(Quantity::DrogueRate, Said::Absent)],
        }];
        let check = check(Quantity::DrogueRate, &flights);
        assert_eq!((check.absent, check.test.counts.given), (1, 0));
        assert!(check.withheld.is_empty());
    }

    #[test]
    fn a_top_lands_within_five_percent_of_the_clip() {
        let ratios: Vec<f64> = (0..1000).map(top_over_clip).collect();
        assert!(ratios.iter().all(|r| (0.95..1.05).contains(r)));
        let mean = ratios.iter().sum::<f64>() / 1000.0;
        assert!((mean - 1.0).abs() < 1e-3, "{mean}");
        // Spread over the band, not gathered.
        assert_eq!(ratios.iter().filter(|r| **r < 0.96).count(), 100);
    }

    /// Scaled to a top at the clip, a flight's log reads a floor there whose truth the fitted
    /// lower end is measured against; with its top as flown, past the clip, the same.
    #[test]
    fn a_flight_says_its_top_accelerations_floors_from_clipping_logs() {
        use super::super::designs::DesignFiles;
        use super::super::{Flyer, Set};
        let mut flyer = Flyer::new(DesignFiles::committed());
        let range = LOWER_RANGE_G * STANDARD_GRAVITY_MPS2;
        // Past 1.2 times the clip, and with its scaled top read as a floor, as about two in three
        // are.
        let (index, flight, said) = (0..80)
            .filter_map(|k| flyer.flight(Set::Tuning, k).ok().map(|f| (k, f)))
            .filter(|(_, f)| {
                f.draw.logger.profile.accelerometer_range_m_s2().is_some()
                    && top_specific_force(f) > 1.2 * range
            })
            .map(|(k, f)| {
                let said = said(Set::Tuning.seed(), k, &f).unwrap();
                (k, f, said)
            })
            .find(|(_, _, s)| s.said.iter().any(|(_, x)| matches!(x, Said::Floor(_))))
            .unwrap();
        let tops: Vec<&Said> = said
            .said
            .iter()
            .filter(|(q, _)| *q == Quantity::TopAcceleration)
            .map(|(_, s)| s)
            .collect();
        // Its own log's reading, whole, then the floor at the clip and the floor as flown.
        assert!(matches!(tops[0], Said::Stated(_)), "{tops:?}");
        let Said::Floor(at_clip) = tops[1] else {
            panic!("{tops:?}");
        };
        assert_eq!(at_clip.truth, range * top_over_clip(index));
        assert!((at_clip.reading - range).abs() < 0.1 * range, "{at_clip:?}");
        let Said::PastClip(past) = tops[2] else {
            panic!("{tops:?}");
        };
        let top = top_specific_force(&flight);
        assert_eq!(past.truth, top);
        assert!(past.reading < top, "{past:?}");
        assert!(past.low < past.reading, "{past:?}");
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
