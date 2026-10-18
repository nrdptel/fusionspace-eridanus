//! The readings held to their bounds on the synthetic flights: each reading a log gives, against
//! its flight's truth, beside the bound the reading states (M7.2b).
//!
//! A tuning flight is checked on its own logger's log and, if that logger has a barometer, on
//! the same barometer written again at every rate and step of [`GRID`]: from 500 samples a second
//! to one every two seconds, and from no step to 10 m in height or 100 Pa in pressure, the range
//! the readers' families span. A reading holds where its error is no larger than its bound;
//! `cargo xtask synthetic` fails unless every reading given on the tuning set holds, and reports
//! the worst share of its bound each reading's error reached, on each log, and the readings
//! withheld, by reason.
//!
//! **The truths**, on the log's clock (the truth's time plus the seconds logged before launch):
//! liftoff and landing are the truth's events, the center of mass starting along the rail and
//! reaching the ground; apogee is the center of mass's highest point, its height above where it
//! stood at launch; the top speed is the truth's highest vertical speed; the flight time is
//! landing less liftoff; and the mean rate of descent is the center of mass's height at apogee
//! above the ground over the time from apogee to landing.

use std::collections::BTreeMap;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::{self, Reading, Reason};
use serde::{Deserialize, Serialize};

use super::logger::{Recording, Written, barometer_log};
use super::{Flight, SyntheticError};

/// The foot, m.
const FOOT_M: f64 = 0.3048;

/// The rates, samples a second, the grid writes a barometer at: Blue Raven's 500, AltOS's 100,
/// the 20 most barometric loggers keep, then slower than any but a radio's telemetry.
pub const GRID_RATES_HZ: [f64; 6] = [500.0, 100.0, 20.0, 5.0, 1.0, 0.5];

/// How the grid writes a barometer's reading: as an altitude with no step, in whole feet and in
/// 10 m steps, and as a pressure alone in 1, 10 and 100 Pa steps, past the coarsest the readers'
/// families write (MARSA's 8.8 m, an ARTS2's 92 Pa).
pub const GRID_WRITTEN: [Written; 6] = [
    Written::Altitude { step_m: 0.0 },
    Written::Altitude { step_m: FOOT_M },
    Written::Altitude { step_m: 10.0 },
    Written::PressureAlone { step_pa: 1.0 },
    Written::PressureAlone { step_pa: 10.0 },
    Written::PressureAlone { step_pa: 100.0 },
];

/// Every rate of [`GRID_RATES_HZ`] with every way of [`GRID_WRITTEN`], the same rate climbing
/// and descending.
pub fn grid() -> Vec<Recording> {
    GRID_RATES_HZ
        .iter()
        .flat_map(|&hz| {
            GRID_WRITTEN.iter().map(move |&written| Recording {
                climbing_hz: hz,
                descending_hz: hz,
                written,
            })
        })
        .collect()
}

/// A recording's name in the report, such as `20 Hz, altitude in 0.3048 m steps`.
pub fn name(recording: &Recording) -> String {
    let rate = if recording.climbing_hz == recording.descending_hz {
        format!("{} Hz", recording.climbing_hz)
    } else {
        format!(
            "{} Hz up, {} Hz down",
            recording.climbing_hz, recording.descending_hz
        )
    };
    let written = match recording.written {
        Written::Altitude { step_m } if step_m == 0.0 => "altitude, no step".to_owned(),
        Written::Altitude { step_m } => format!("altitude in {step_m} m steps"),
        Written::Pressure { step_pa } => format!("pressure in {step_pa} Pa steps and altitude"),
        Written::PressureAlone { step_pa } => format!("pressure alone in {step_pa} Pa steps"),
    };
    format!("{rate}, {written}")
}

/// A reading the check holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Quantity {
    /// Liftoff's time.
    Liftoff,
    /// Apogee's height above the pad.
    ApogeeHeight,
    /// Apogee's time.
    ApogeeTime,
    /// The top vertical speed.
    TopSpeed,
    /// Landing's time.
    Landing,
    /// The time from liftoff to landing.
    FlightTime,
    /// The mean rate of descent from apogee to landing.
    DescentRate,
}

impl Quantity {
    /// Every quantity, in the report's order.
    pub const ALL: [Self; 7] = [
        Self::Liftoff,
        Self::ApogeeHeight,
        Self::ApogeeTime,
        Self::TopSpeed,
        Self::Landing,
        Self::FlightTime,
        Self::DescentRate,
    ];

    /// Its name in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::Liftoff => "liftoff",
            Self::ApogeeHeight => "apogee height",
            Self::ApogeeTime => "apogee time",
            Self::TopSpeed => "top speed",
            Self::Landing => "landing",
            Self::FlightTime => "flight time",
            Self::DescentRate => "descent rate",
        }
    }

    /// Its unit.
    pub fn unit(self) -> &'static str {
        match self {
            Self::ApogeeHeight => "m",
            Self::TopSpeed | Self::DescentRate => "m/s",
            _ => "s",
        }
    }
}

/// What one reading of one log came to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    /// Given: its error from the truth and the bound it states.
    Given {
        /// The reading less the truth.
        error: f64,
        /// The bound it states.
        bound: f64,
    },
    /// Withheld, for this reason.
    Withheld(Reason),
}

/// Each reading `log` gives of `flight`, against the truth.
pub fn outcomes(flight: &Flight, log: &FlightLog) -> Vec<(Quantity, Outcome)> {
    let read = readings::read(log);
    let truth = &flight.truth;
    let events = &truth.events;
    let pad_s = flight.draw.logger.pad_s;
    let given = |error: f64, bound: f64| Outcome::Given { error, bound };
    fn of<T>(reading: &Reading<T>, take: impl Fn(&T) -> Outcome) -> Outcome {
        match reading {
            Reading::Read(value) => take(value),
            Reading::Withheld(withheld) => Outcome::Withheld(withheld.reason),
        }
    }
    let descent_rate = events.apogee_m / (events.landing_s - events.apogee_s);
    vec![
        (
            Quantity::Liftoff,
            of(&read.liftoff, |l| {
                given(l.time_s - (pad_s + events.liftoff_s), l.bound_s)
            }),
        ),
        (
            Quantity::ApogeeHeight,
            of(&read.apogee, |a| {
                given(a.height_m - truth.apogee_above_pad_m(), a.bound_m)
            }),
        ),
        (
            Quantity::ApogeeTime,
            of(&read.apogee, |a| {
                given(a.time_s - (pad_s + events.apogee_s), a.time_bound_s)
            }),
        ),
        (
            Quantity::TopSpeed,
            of(&read.max_speed, |s| {
                given(s.speed_m_s - truth.top_speed().0, s.bound_m_s)
            }),
        ),
        (
            Quantity::Landing,
            of(&read.landing, |l| {
                given(l.time_s - (pad_s + events.landing_s), l.bound_s)
            }),
        ),
        (
            Quantity::FlightTime,
            of(&read.landing, |l| {
                given(
                    l.flight_time_s - (events.landing_s - events.liftoff_s),
                    l.flight_time_bound_s,
                )
            }),
        ),
        (
            Quantity::DescentRate,
            of(&read.landing, |l| {
                given(
                    l.mean_descent_rate_m_s - descent_rate,
                    l.mean_descent_rate_bound_m_s,
                )
            }),
        ),
    ]
}

/// The logs flight `index` is checked on: its own logger's, named by its profile, and its
/// barometer's at each of [`grid`]'s recordings, named by [`name`]; a logger with no barometer
/// gives only its own.
///
/// # Errors
///
/// [`SyntheticError`] if a log can't be written.
pub fn logs(
    flight: &Flight,
    seed: u64,
    index: u64,
) -> Result<Vec<(String, FlightLog)>, SyntheticError> {
    let mut logs = vec![(
        flight.draw.logger.profile.name().to_owned(),
        flight.log.clone(),
    )];
    for recording in grid() {
        if let Some(log) = barometer_log(&flight.draw.logger, &recording, &flight.truth, seed, index)? {
            logs.push((name(&recording), log));
        }
    }
    Ok(logs)
}

/// The worst a reading came to on one kind of log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Worst {
    /// The flight's index in its set.
    pub flight: u64,
    /// Its error.
    pub error: f64,
    /// Its bound.
    pub bound: f64,
    /// The error's size over the bound: 1 or less holds.
    pub share: f64,
}

/// How one reading fared on one kind of log over a set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tally {
    /// The log: a profile's name, or a recording's.
    pub log: String,
    /// The reading.
    pub quantity: Quantity,
    /// How many flights gave it.
    pub given: u64,
    /// How many withheld it, by reason.
    pub withheld: BTreeMap<Reason, u64>,
    /// The flight whose error was the largest share of its bound; `None` if none gave it.
    pub worst: Option<Worst>,
}

impl Tally {
    /// Whether every reading given held to its bound.
    pub fn holds(&self) -> bool {
        self.worst.as_ref().is_none_or(|worst| worst.share <= 1.0)
    }
}

/// The tallies of every flight's outcomes: `outcomes[k]` holds flight `flights[k]`'s logs, each
/// named, with their outcomes. In the order logs are first met, then [`Quantity::ALL`]'s.
pub fn tally(flights: &[(u64, Vec<(String, Vec<(Quantity, Outcome)>)>)]) -> Vec<Tally> {
    let mut order: Vec<String> = Vec::new();
    let mut tallies: BTreeMap<(String, Quantity), Tally> = BTreeMap::new();
    for (flight, logs) in flights {
        for (log, results) in logs {
            if !order.contains(log) {
                order.push(log.clone());
            }
            for (quantity, outcome) in results {
                let tally = tallies
                    .entry((log.clone(), *quantity))
                    .or_insert_with(|| Tally {
                        log: log.clone(),
                        quantity: *quantity,
                        given: 0,
                        withheld: BTreeMap::new(),
                        worst: None,
                    });
                match *outcome {
                    Outcome::Given { error, bound } => {
                        tally.given += 1;
                        // A bound of 0 holds only an exact reading; NaN is never a hold.
                        let share = if bound > 0.0 {
                            error.abs() / bound
                        } else if error == 0.0 {
                            0.0
                        } else {
                            f64::INFINITY
                        };
                        let share = if share.is_nan() { f64::INFINITY } else { share };
                        if tally.worst.as_ref().is_none_or(|w| share > w.share) {
                            tally.worst = Some(Worst {
                                flight: *flight,
                                error,
                                bound,
                                share,
                            });
                        }
                    }
                    Outcome::Withheld(reason) => {
                        *tally.withheld.entry(reason).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    order
        .iter()
        .flat_map(|log| {
            Quantity::ALL
                .iter()
                .filter_map(|quantity| tallies.remove(&(log.clone(), *quantity)))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthetic::designs::DesignFiles;
    use crate::synthetic::{Flyer, Set};

    /// The grid spans every rate and way of writing, each once, and names each apart.
    #[test]
    fn the_grid_names_each_recording_apart() {
        let names: Vec<String> = grid().iter().map(name).collect();
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), GRID_RATES_HZ.len() * GRID_WRITTEN.len());
        assert!(names.contains(&"20 Hz, altitude in 0.3048 m steps".to_owned()));
        assert!(names.contains(&"0.5 Hz, pressure alone in 100 Pa steps".to_owned()));
    }

    /// A tuning flight's readings hold on its own log and every grid log; a reading pushed past
    /// its bound is counted as not holding.
    #[test]
    fn a_tuning_flight_holds_and_a_miss_is_caught() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        // The first tuning flight with a barometer.
        let (index, flight) = (0..)
            .map(|k| (k, flyer.flight(Set::Tuning, k)))
            .find_map(|(k, flight)| {
                flight
                    .ok()
                    .filter(|f| f.draw.logger.profile.recording().is_some())
                    .map(|f| (k, f))
            })
            .unwrap();
        let logs = logs(&flight, Set::Tuning.seed(), index).unwrap();
        assert_eq!(logs.len(), 1 + grid().len());
        let results: Vec<(String, Vec<(Quantity, Outcome)>)> = logs
            .iter()
            .map(|(name, log)| (name.clone(), outcomes(&flight, log)))
            .collect();
        let tallies = tally(&[(index, results.clone())]);
        for tally in &tallies {
            assert!(tally.holds(), "{tally:?}");
        }
        // The same outcomes with one apogee's error past its bound.
        let mut missed = results;
        for (quantity, outcome) in &mut missed[0].1 {
            if let (Quantity::ApogeeHeight, Outcome::Given { error, bound }) = (quantity, outcome)
            {
                *error = 1.01 * *bound;
            }
        }
        let tallies = tally(&[(index, missed)]);
        assert!(!tallies[1].holds(), "{:?}", tallies[1]);
        assert_eq!(tallies[1].quantity, Quantity::ApogeeHeight);
    }
}
