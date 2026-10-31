//! Two recordings of one synthetic flight, and a logger in each stage of a staged one: the check
//! of [`hpr_flightdata::timeline`]'s clock offset ([two recordings, in the plan for the readings][adr-280]).
//!
//! **What it is for.** A flight's truth says how each logger's clock stood against launch, so the
//! offset two logs' liftoffs recover can be held to the true one. Each flight here carries two
//! loggers, drawn independently from the ranges [`LoggerDraw::draw`] states, with their noise on
//! streams of their own, each clock's zero moved by up to [`MAX_CLOCK_S`] either way, as a board's
//! is by when it was switched on. The offset's bound must hold on every flight, and its 90%
//! interval must pass [ADR-257][adr-257] §1's test on [`Set::PairA`] and [`Set::PairB`] with its
//! shares fitted on [`Set::PairCalibration`], each on a seed of its own.
//!
//! **Stages** ([`Staged`]) fly `validation/designs/synthetic-two-stage-75mm-54mm.json`, handed in
//! by the caller, as the designs are ([`super::designs`]), with a logger in each stage. The
//! booster's truth after the split is HPR Sim's point-mass descent read between its integration
//! steps (`docs/physics/staging.md`), so the check says how well two stages' liftoffs line their
//! clocks up, not how well a booster's fall is modeled.
//!
//! **How far to trust it.** Two boards here share a flight and differ only in noise, rate and
//! step. Real boards also sit at different stations, and may see the pad's shock differently; the
//! one real pair checked, RocketPy's Prometheus, is on the guide's page.
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md

use std::collections::BTreeMap;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::interval::{self, Shares};
use hpr_flightdata::readings::{self, Reading, Reason};
use hpr_flightdata::timeline;
use serde::{Deserialize, Serialize};

use super::intervals::{Compared, Counts, Probe, Test};
use super::logger::{self, LoggerDraw, Profile};
use super::noise::between;
use super::truth::{self, StagedTruth};
use super::{Flyer, Key, Set, SyntheticError, draw, stream};

/// The most a logger's clock zero is moved from where its log starts, s, either way: about five
/// minutes on the pad, as a board switched on before the rocket went out to the rail is.
pub const MAX_CLOCK_S: f64 = 300.0;
/// What the second logger's noise streams add to the set's seed, so that two boards on one flight
/// draw independent noise.
const SECOND_LOGGER_STREAMS: u64 = 1_000_000;

/// Two logs of one flight, each on its own clock.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoLogs {
    /// Its index in its set.
    pub index: u64,
    /// The first logger's profile.
    pub first: Profile,
    /// The second logger's profile.
    pub second: Profile,
    /// The first log.
    pub first_log: FlightLog,
    /// The second log.
    pub second_log: FlightLog,
    /// The second's clock less the first's at one instant, s: the truth an alignment recovers.
    pub true_offset_s: f64,
}

/// `log` with `clock_s` added to every time it holds.
pub fn on_clock(mut log: FlightLog, clock_s: f64) -> FlightLog {
    for t in &mut log.time_s {
        *t += clock_s;
    }
    for channel in &mut log.channels {
        for t in &mut channel.time_s {
            *t += clock_s;
        }
    }
    for event in &mut log.events {
        event.time_s += clock_s;
    }
    log
}

/// Flight `index`'s two clock moves, s.
fn clocks(seed: u64, index: u64) -> (f64, f64) {
    let mut rng = stream(seed, index, Key::Clocks);
    let first = between(&mut rng, -MAX_CLOCK_S, MAX_CLOCK_S);
    (first, between(&mut rng, -MAX_CLOCK_S, MAX_CLOCK_S))
}

/// A second logger for flight `index`: its draw, on a stream of its own.
fn second_logger(seed: u64, index: u64) -> LoggerDraw {
    let mut rng = stream(seed, index, Key::SecondLogger);
    let profile = Profile::ALL
        [((rng.uniform() * Profile::ALL.len() as f64) as usize).min(Profile::ALL.len() - 1)];
    LoggerDraw::draw(&mut rng, profile)
}

/// Both logs of a flight whose first logger is `first`, on their clocks.
fn logged(
    set: Set,
    index: u64,
    (first, first_truth): (&LoggerDraw, &truth::Truth),
    (second, second_truth): (&LoggerDraw, &truth::Truth),
) -> Result<TwoLogs, SyntheticError> {
    let seed = set.seed();
    let (first_clock, second_clock) = clocks(seed, index);
    let first_log = logger::log(first, first_truth, seed, index)?;
    let second_log = logger::log(second, second_truth, seed + SECOND_LOGGER_STREAMS, index)?;
    Ok(TwoLogs {
        index,
        first: first.profile,
        second: second.profile,
        first_log: on_clock(first_log, first_clock),
        second_log: on_clock(second_log, second_clock),
        // A log's time is since launch, plus its span on the pad, plus its clock's move.
        true_offset_s: (second.pad_s + second_clock) - (first.pad_s + first_clock),
    })
}

impl Flyer {
    /// Flight `index` of `set`, with two loggers.
    ///
    /// # Errors
    ///
    /// As [`Flyer::flight`].
    pub fn two_logs(&mut self, set: Set, index: u64) -> Result<TwoLogs, SyntheticError> {
        let flight = self.flight(set, index)?;
        let second = second_logger(set.seed(), index);
        logged(
            set,
            index,
            (&flight.draw.logger, &flight.truth),
            (&second, &flight.truth),
        )
    }
}

/// The two-stage design's flights.
#[derive(Debug, Clone)]
pub struct Staged {
    json: String,
}

impl Staged {
    /// Flights of the two-stage design in `json`.
    pub fn new(json: String) -> Self {
        Self { json }
    }

    /// Flight `index` of [`Set::Staged`]: the truth of each stage, and two logs, the sustainer's
    /// first and the booster's second. Its draw's design is unused.
    ///
    /// # Errors
    ///
    /// [`SyntheticError`] as [`truth::fly_staged`] and [`logger::log`].
    pub fn flight(&self, index: u64) -> Result<(StagedTruth, TwoLogs), SyntheticError> {
        let set = Set::Staged;
        let drawn = draw(set, index);
        let flown = truth::fly_staged(&drawn, &self.json)?;
        let second = second_logger(set.seed(), index);
        let logs = logged(
            set,
            index,
            (&drawn.logger, &flown.sustainer),
            (&second, &flown.booster),
        )?;
        Ok((flown, logs))
    }
}

/// What a flight's two logs said of the clock offset.
#[derive(Debug, Clone, PartialEq)]
pub struct Said {
    /// Its index in its set.
    pub index: u64,
    /// The two loggers' names, the alphabetically first first.
    pub loggers: (&'static str, &'static str),
    /// The offset against the truth, or why a log has no liftoff.
    pub offset: Result<Compared, Reason>,
}

/// What `logs` say of the offset, by [`timeline`].
pub fn said(logs: &TwoLogs) -> Said {
    let (a, b) = (logs.first.name(), logs.second.name());
    let loggers = if a <= b { (a, b) } else { (b, a) };
    let first = timeline::place(&readings::read(&logs.first_log));
    let second = timeline::place(&readings::read(&logs.second_log));
    let offset = match (&first, &second) {
        (Reading::Read(first), Reading::Read(second)) => {
            let offset = timeline::offset(first, second);
            match offset.interval_s {
                Some(interval) => Ok(Compared {
                    reading: offset.offset_s,
                    bound: offset.bound_s,
                    interval,
                    truth: logs.true_offset_s,
                }),
                None => Err(Reason::BadRecord),
            }
        }
        (Reading::Withheld(withheld), _) | (_, Reading::Withheld(withheld)) => Err(withheld.reason),
    };
    Said {
        index: logs.index,
        loggers,
        offset,
    }
}

/// How one pair of logger families' offsets did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct PairCell {
    /// Offsets given.
    pub given: u64,
    /// The largest error as a share of its bound.
    pub largest_share: f64,
}

/// One set's check of the offset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Check {
    /// Offsets given.
    pub given: u64,
    /// Those whose error stayed within the bound stated.
    pub within_bound: u64,
    /// The largest error as a share of its bound.
    pub largest_share: f64,
    /// The median error, s, its size.
    pub median_error_s: f64,
    /// The 90th percentile of its size, s.
    pub p90_error_s: f64,
    /// The largest error, s, its size.
    pub largest_error_s: f64,
    /// The median bound stated, s.
    pub median_bound_s: f64,
    /// The offsets withheld, by reason.
    pub withheld: BTreeMap<Reason, u64>,
    /// By pair of logger families.
    pub by_loggers: BTreeMap<String, PairCell>,
    /// The coverage test on the 90% interval.
    pub test: Test,
    /// The test on each probe's changed intervals.
    pub probes: BTreeMap<Probe, Test>,
}

impl Check {
    /// Whether every offset given stayed within its bound.
    pub fn bound_holds(&self) -> bool {
        self.given > 0 && self.within_bound == self.given
    }
}

/// The `share`-th point of `sorted`, the nearest rank.
fn point(sorted: &[f64], share: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let rank = (share * sorted.len() as f64).ceil() as usize;
    sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
}

/// The check of `flights`.
pub fn check(flights: &[Said]) -> Check {
    let mut withheld: BTreeMap<Reason, u64> = BTreeMap::new();
    let mut by_loggers: BTreeMap<String, PairCell> = BTreeMap::new();
    let mut counts = Counts::default();
    let mut probes: BTreeMap<Probe, Counts> =
        Probe::ALL.iter().map(|&p| (p, Counts::default())).collect();
    let mut errors = Vec::new();
    let mut bounds = Vec::new();
    let (mut within_bound, mut largest_share) = (0, 0.0_f64);
    for flight in flights {
        match &flight.offset {
            Err(reason) => *withheld.entry(*reason).or_default() += 1,
            Ok(offset) => {
                let error = (offset.reading - offset.truth).abs();
                let share = error / offset.bound;
                within_bound += u64::from(error <= offset.bound);
                largest_share = largest_share.max(share);
                errors.push(error);
                bounds.push(offset.bound);
                counts.add(offset.interval, offset.truth);
                for (probe, probed) in &mut probes {
                    probed.add(probe.apply(offset.interval), offset.truth);
                }
                let cell = by_loggers
                    .entry(format!("{} + {}", flight.loggers.0, flight.loggers.1))
                    .or_default();
                cell.given += 1;
                cell.largest_share = cell.largest_share.max(share);
            }
        }
    }
    errors.sort_by(f64::total_cmp);
    bounds.sort_by(f64::total_cmp);
    Check {
        given: errors.len() as u64,
        within_bound,
        largest_share,
        median_error_s: point(&errors, 0.5),
        p90_error_s: point(&errors, 0.9),
        largest_error_s: errors.last().copied().unwrap_or(f64::NAN),
        median_bound_s: point(&bounds, 0.5),
        withheld,
        by_loggers,
        test: Test::of(counts),
        probes: probes
            .into_iter()
            .map(|(probe, counts)| (probe, Test::of(counts)))
            .collect(),
    }
}

/// The offset's shares fitted again on `flights`, which should be [`Set::PairCalibration`]'s.
pub fn fitted(flights: &[Said]) -> Option<Shares> {
    let shares: Vec<f64> = flights
        .iter()
        .filter_map(|flight| flight.offset.as_ref().ok().map(Compared::share))
        .collect();
    interval::fit(&shares)
}

#[cfg(test)]
mod tests {
    use super::super::designs::DesignFiles;
    use super::*;

    const TWO_STAGE: &str =
        include_str!("../../../../validation/designs/synthetic-two-stage-75mm-54mm.json");

    #[test]
    fn moving_a_clock_moves_each_reading_by_as_much_and_changes_nothing_else() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        for index in 0..6 {
            let flight = flyer.flight(Set::PairA, index).unwrap();
            let before = readings::read(&flight.log);
            let after = readings::read(&on_clock(flight.log.clone(), 137.25));
            let (Reading::Read(b), Reading::Read(a)) = (&before.liftoff, &after.liftoff) else {
                assert_eq!(before.liftoff.reason(), after.liftoff.reason());
                continue;
            };
            assert!(
                (a.time_s - b.time_s - 137.25).abs() < 1e-9,
                "flight {index}"
            );
            assert!((a.bound_s - b.bound_s).abs() < 1e-9, "flight {index}");
        }
    }

    #[test]
    fn the_true_offset_is_each_clock_less_the_other_and_the_flight_redraws_the_same() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        let once = flyer.two_logs(Set::PairB, 4).unwrap();
        let again = flyer.two_logs(Set::PairB, 4).unwrap();
        assert_eq!(once, again);
        let first = draw(Set::PairB, 4).logger;
        let second = second_logger(Set::PairB.seed(), 4);
        let (a, b) = clocks(Set::PairB.seed(), 4);
        assert_eq!(once.true_offset_s, (second.pad_s + b) - (first.pad_s + a));
        // The second board's noise is on streams of its own, not the first's.
        let flight = flyer.flight(Set::PairB, 4).unwrap();
        let seed = Set::PairB.seed();
        let own = logger::log(&second, &flight.truth, seed + SECOND_LOGGER_STREAMS, 4).unwrap();
        let shared = logger::log(&second, &flight.truth, seed, 4).unwrap();
        assert_eq!(once.second_log, on_clock(own.clone(), b));
        assert_ne!(own, shared);
    }

    #[test]
    fn a_flight_says_its_offset_and_the_bound_holds_on_a_few() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        let flights: Vec<Said> = (0..12)
            .map(|k| said(&flyer.two_logs(Set::PairCalibration, k).unwrap()))
            .collect();
        let checked = check(&flights);
        assert!(checked.given > 0, "{checked:?}");
        assert!(checked.bound_holds(), "{checked:?}");
    }

    #[test]
    fn an_offset_off_by_a_bound_and_a_bit_fails_the_bound() {
        // The check can fail: a reading moved past its bound isn't counted as within.
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut flights: Vec<Said> = (0..12)
            .map(|k| said(&flyer.two_logs(Set::PairCalibration, k).unwrap()))
            .collect();
        let offset = flights
            .iter_mut()
            .find_map(|f| f.offset.as_mut().ok())
            .unwrap();
        offset.reading = offset.truth + offset.bound * 1.01;
        assert!(!check(&flights).bound_holds());
    }

    #[test]
    fn the_staged_sets_draws_stay_inside_their_ranges_and_fly_hpr_sims_model() {
        // 200 draws leave a gap at a range's end too often for the larger sets' test that each
        // range fills: only that they stay inside. They come from the same `draw`.
        for index in 0..Set::Staged.count() {
            let d = draw(Set::Staged, index);
            assert!((-15.0..=15.0).contains(&d.temperature_offset_k), "{d:?}");
            assert!((0.0..=1_500.0).contains(&d.elevation_m), "{d:?}");
            assert!(
                (0.0..=crate::synthetic::MAX_WIND_M_S).contains(&d.wind_speed_m_s),
                "{d:?}"
            );
            assert!((0.8..=1.2).contains(&d.drag_scale), "{d:?}");
            assert_eq!(d.model, crate::synthetic::Model::Same);
        }
    }

    #[test]
    fn a_staged_flight_parts_and_each_stage_has_its_own_truth_and_log() {
        let staged = Staged::new(TWO_STAGE.to_owned());
        let (truth, logs) = staged.flight(0).unwrap();
        // The sustainer lights and goes higher; the booster falls alone from the split.
        assert!(truth.separation_s > truth.booster.events.liftoff_s);
        assert!(truth.sustainer.apogee_above_pad_m() > truth.booster.apogee_above_pad_m());
        assert!(truth.booster.events.apogee_s > truth.separation_s);
        assert_eq!(
            truth.sustainer.events.liftoff_s,
            truth.booster.events.liftoff_s
        );
        // Before the split the two truths are one stack; after it they part.
        let before = truth.separation_s - 0.1;
        assert_eq!(
            truth.sustainer.at(before).unwrap().height_m,
            truth.booster.at(before).unwrap().height_m
        );
        let after = truth.booster.events.apogee_s;
        assert!(
            (truth.sustainer.at(after).unwrap().height_m
                - truth.booster.at(after).unwrap().height_m)
                .abs()
                > 1.0
        );
        // The booster reaches the ground where its truth says.
        let landing = truth.booster.at(truth.booster.events.landing_s).unwrap();
        assert!(landing.height_m.abs() < 1.0, "{landing:?}");
        let offset = said(&logs).offset.unwrap();
        assert!(
            (offset.reading - offset.truth).abs() <= offset.bound,
            "{offset:?}"
        );
    }
}
