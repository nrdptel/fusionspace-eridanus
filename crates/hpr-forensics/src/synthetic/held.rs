//! The readings held to their bounds on the synthetic flights: each reading a log gives, against
//! its flight's truth, beside the bound the reading states ([M7.2b][m7-2b], the readings on every
//! logger).
//!
//! A tuning flight is checked on its own logger's log and, if that logger has a barometer, on
//! the same barometer written again at every rate of [`GRID_RATES_HZ`] in every way of
//! [`GRID_WRITTEN`]: from 500 samples a second to one every two seconds, and from no step to 10 m
//! in height or 100 Pa in pressure, the range the readers' families span; and, as a logger that
//! records from launch writes them, the two recordings of [`CROPPED`] cut to each start of
//! [`Crop`]. A reading holds where its error is no larger than its bound; `cargo xtask synthetic`
//! fails unless every reading given on the tuning set holds, and reports the worst share of its
//! bound each reading's error reached, on each log, and the readings withheld, by reason.
//!
//! **The truths**, on the log's clock (the truth's time plus the seconds logged before launch):
//! liftoff and landing are the truth's events, the center of mass starting along the rail and
//! reaching the ground; apogee is the center of mass's highest point, its height above where it
//! stood at launch; the top speed is the truth's highest vertical speed; the flight time is
//! landing less liftoff; and the mean rate of descent is the center of mass's height at apogee
//! above the ground over the time from apogee to landing.
//!
//! [m7-2b]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2b

use std::collections::BTreeMap;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::{self, Clip, Reading, Reason};
use serde::{Deserialize, Serialize};

use super::logger::{
    LOWER_RANGE_G, Recording, Written, accelerometer_log, accelerometer_log_set_lower,
    barometer_log,
};
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

/// The rates, samples a second, the check writes each accelerometer at again: Blue Raven's 500, a
/// Raven's axial 400 and AltOS's 100 lie within them, and 20 is the slowest a logger's accelerometer
/// is written at among the families read.
pub const ACCELEROMETER_RATES_HZ: [f64; 3] = [500.0, 100.0, 20.0];

/// The recordings the check also cuts to each [`Crop`]: 20 samples a second, as most loggers that
/// record from launch write, in whole feet as a PerfectFlite, an RRC3 or an Eggtimer, and as a
/// pressure alone in 10 Pa steps as a Raven or an AltOS keeps its raw pressure.
pub const CROPPED: [Recording; 2] = [
    Recording {
        climbing_hz: 20.0,
        descending_hz: 20.0,
        written: Written::Altitude { step_m: FOOT_M },
    },
    Recording {
        climbing_hz: 20.0,
        descending_hz: 20.0,
        written: Written::PressureAlone { step_pa: 10.0 },
    },
];

/// Where a logger that records from launch starts its log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Crop {
    /// 0.3 s before liftoff, as a Raven or an AltOS keeps a little of its pad.
    BeforeLiftoff,
    /// 0.1 s after liftoff, with nothing of the pad.
    AfterLiftoff,
    /// 0.5 s after liftoff, its first reading of the pad written first, as an RRC3 or an
    /// Eggtimer writes its zero and then the climb from launch detection.
    ZeroFirst,
}

impl Crop {
    /// Every crop.
    pub const ALL: [Self; 3] = [Self::BeforeLiftoff, Self::AfterLiftoff, Self::ZeroFirst];

    /// Its name in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::BeforeLiftoff => "from 0.3 s before liftoff",
            Self::AfterLiftoff => "from 0.1 s after liftoff",
            Self::ZeroFirst => "from 0.5 s after liftoff, its zero first",
        }
    }

    /// `log` cut to start where this crop does, `liftoff_s` on the log's clock: every track
    /// keeps its samples from then on, and a [`Crop::ZeroFirst`] puts each track's first sample
    /// back an interval before its first kept one.
    pub fn cut(self, log: &FlightLog, liftoff_s: f64) -> FlightLog {
        let from_s = match self {
            Self::BeforeLiftoff => liftoff_s - 0.3,
            Self::AfterLiftoff => liftoff_s + 0.1,
            Self::ZeroFirst => liftoff_s + 0.5,
        };
        let cut_track = |time_s: &[f64], values: &[f64]| -> (Vec<f64>, Vec<f64>) {
            let first = time_s.partition_point(|t| *t < from_s);
            let mut time: Vec<f64> = time_s[first..].to_vec();
            let mut kept: Vec<f64> = values[first..].to_vec();
            if let (Self::ZeroFirst, Some(&zero), [t0, t1, ..]) =
                (self, values.first(), time.as_slice())
            {
                let back = t0 - (t1 - t0);
                time.insert(0, back);
                kept.insert(0, zero);
            }
            (time, kept)
        };
        let mut cut = log.clone();
        (cut.time_s, cut.altitude_m) = cut_track(&log.time_s, &log.altitude_m);
        for channel in &mut cut.channels {
            (channel.time_s, channel.values) = cut_track(&channel.time_s, &channel.values);
        }
        cut
    }
}

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
        Written::Altitude { step_m: 0.0 } => "altitude, no step".to_owned(),
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
    /// The top acceleration, from the axial accelerometer.
    TopAcceleration,
    /// Burnout's time.
    Burnout,
    /// The rate of descent under the drogue.
    DrogueRate,
    /// The main's opening, found back from landing.
    MainOpening,
    /// The rate of descent under the main.
    MainRate,
}

impl Quantity {
    /// Every quantity, in the report's order.
    pub const ALL: [Self; 12] = [
        Self::Liftoff,
        Self::ApogeeHeight,
        Self::ApogeeTime,
        Self::TopSpeed,
        Self::Landing,
        Self::FlightTime,
        Self::DescentRate,
        Self::TopAcceleration,
        Self::Burnout,
        Self::DrogueRate,
        Self::MainOpening,
        Self::MainRate,
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
            Self::TopAcceleration => "top acceleration",
            Self::Burnout => "burnout",
            Self::DrogueRate => "drogue rate",
            Self::MainOpening => "main opening",
            Self::MainRate => "main rate",
        }
    }

    /// Its unit.
    pub fn unit(self) -> &'static str {
        match self {
            Self::ApogeeHeight => "m",
            Self::TopSpeed | Self::DescentRate | Self::DrogueRate | Self::MainRate => "m/s",
            Self::TopAcceleration => "m/s²",
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
    /// Given as a floor: the truth is no lower than the reading less its bound, and may be higher
    /// by any amount, as a clipped top acceleration or a burnout from the speed's peak states.
    Floor {
        /// The reading less the truth: it holds while this is no more than the bound.
        error: f64,
        /// The bound it states.
        bound: f64,
        /// Why a top acceleration is clipped, where it is; `None` for any other floor, such as a
        /// burnout from the speed's peak or a top whose thrust the accelerometer shows no end to.
        clip: Option<Clip>,
    },
    /// Withheld, for this reason.
    Withheld(Reason),
    /// Not read where the truth has none to read: a drogue, or a main's opening after one, on a
    /// flight that came down under one canopy or none.
    Absent,
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
    // The truth on the log's clock.
    let height = |t_log: f64| truth.at(t_log - pad_s).map_or(f64::NAN, |x| x.height_m);
    let chord = |from_s: f64, to_s: f64| (height(from_s) - height(to_s)) / (to_s - from_s);
    let top_acceleration = top_specific_force(flight);
    let floor_or_given = |is_floor: bool, error: f64, bound: f64, clip: Option<Clip>| {
        if is_floor {
            Outcome::Floor { error, bound, clip }
        } else {
            Outcome::Given { error, bound }
        }
    };
    let main_opening = match &read.main_descent {
        Reading::Read(main) => match (&main.opening, events.deployments.len()) {
            (Some(opening), n) if n >= 2 => {
                let opened = pad_s + events.deployments[n - 1].1;
                given(opening.time_s - opened, opening.bound_s)
            }
            // A main named where the flight opened one canopy, or none, is a miss; so is one
            // canopy read where the flight opened two, as its main's leg then holds the drogue's.
            (Some(opening), _) => given(f64::INFINITY, opening.bound_s),
            (None, n) if n >= 2 => given(f64::INFINITY, 0.0),
            (None, _) => Outcome::Absent,
        },
        Reading::Withheld(withheld) => Outcome::Withheld(withheld.reason),
    };
    let two_canopies = events.deployments.len() >= 2;
    let drogue_rate = match &read.drogue_descent {
        Reading::Read(d) => given(d.rate_m_s - chord(d.from_s, d.to_s), d.bound_m_s),
        Reading::Withheld(withheld) if withheld.reason == Reason::OneCanopy && !two_canopies => {
            Outcome::Absent
        }
        Reading::Withheld(withheld) => Outcome::Withheld(withheld.reason),
    };
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
            of(&read.flight_time, |f| {
                given(f.time_s - (events.landing_s - events.liftoff_s), f.bound_s)
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
        (
            Quantity::TopAcceleration,
            of(&read.max_acceleration, |a| {
                floor_or_given(
                    a.is_floor,
                    a.acceleration_m_s2 - top_acceleration,
                    a.bound_m_s2,
                    a.clip,
                )
            }),
        ),
        (
            Quantity::Burnout,
            of(&read.burnout, |b| {
                floor_or_given(
                    b.is_floor,
                    b.time_s - (pad_s + events.burnout_s),
                    b.bound_s,
                    None,
                )
            }),
        ),
        (Quantity::DrogueRate, drogue_rate),
        (Quantity::MainOpening, main_opening),
        (
            Quantity::MainRate,
            of(&read.main_descent, |d| {
                given(d.rate_m_s - chord(d.from_s, d.to_s), d.bound_m_s)
            }),
        ),
    ]
}

/// The truth's highest specific force along the axis from liftoff to apogee, m/s²: the top
/// acceleration a reading is checked against.
pub(super) fn top_specific_force(flight: &Flight) -> f64 {
    let events = &flight.truth.events;
    flight
        .truth
        .samples
        .iter()
        .filter(|x| x.time_s >= events.liftoff_s && x.time_s <= events.apogee_s)
        .map(|x| x.axial_specific_force_m_s2)
        .fold(f64::NEG_INFINITY, f64::max)
}

/// How near flight's top acceleration came to the range its logger's accelerometer clips at: the
/// truth's top over the range, which a part reaches at 1; `None` for a logger with no
/// accelerometer.
pub fn top_over_range(flight: &Flight) -> Option<f64> {
    flight
        .draw
        .logger
        .profile
        .accelerometer_range_m_s2()
        .map(|range| top_specific_force(flight) / range)
}

/// As [`top_over_range`], against the lower range its logger's accelerometer is set to on the
/// log [`accelerometer_log_set_lower`] writes; `None` for a part with no such setting.
pub fn top_over_lower_range(flight: &Flight) -> Option<f64> {
    flight
        .draw
        .logger
        .profile
        .accelerometer_lower_range_m_s2()
        .map(|range| top_specific_force(flight) / range)
}

/// The name in the report of the log [`accelerometer_log_set_lower`] writes.
pub fn lower_range_name() -> String {
    format!("its accelerometer set to ±{LOWER_RANGE_G} g")
}

/// The logs flight `index` is checked on: its own logger's, named by its profile; its
/// barometer's at each of [`grid`]'s recordings, named by [`name`]; its own logger's log with the
/// accelerometer at each of [`ACCELEROMETER_RATES_HZ`], and with it set to the lower range its
/// part offers, named by [`lower_range_name`], so that some tops clip at the range and the rest
/// don't; and its barometer's at each of [`CROPPED`]'s recordings, cut each [`Crop`] way. A
/// logger with no barometer gives no grid's or crop's, one with no accelerometer no
/// accelerometer's, and one whose part has no lower setting no lower range's.
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
        if let Some(log) =
            barometer_log(&flight.draw.logger, &recording, &flight.truth, seed, index)?
        {
            logs.push((name(&recording), log));
        }
    }
    for rate_hz in ACCELEROMETER_RATES_HZ {
        if let Some(log) =
            accelerometer_log(&flight.draw.logger, rate_hz, &flight.truth, seed, index)?
        {
            logs.push((format!("its accelerometer at {rate_hz} Hz"), log));
        }
    }
    if let Some(log) = accelerometer_log_set_lower(&flight.draw.logger, &flight.truth, seed, index)?
    {
        logs.push((lower_range_name(), log));
    }
    let liftoff_s = flight.draw.logger.pad_s + flight.truth.events.liftoff_s;
    for recording in CROPPED {
        if let Some(log) =
            barometer_log(&flight.draw.logger, &recording, &flight.truth, seed, index)?
        {
            for crop in Crop::ALL {
                let name = format!("{}, {}", name(&recording), crop.name());
                logs.push((name, crop.cut(&log, liftoff_s)));
            }
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
    /// How many had none to read: a drogue on a flight under one canopy.
    #[serde(default)]
    pub absent: u64,
    /// The flight whose error was the largest share of its bound; `None` if none gave it.
    pub worst: Option<Worst>,
}

impl Tally {
    /// Whether every reading given held to its bound.
    pub fn holds(&self) -> bool {
        self.worst.as_ref().is_none_or(|worst| worst.share <= 1.0)
    }
}

/// A flight's outcomes: its index in its set, and each of its logs, named, with each quantity's
/// outcome.
pub type FlightOutcomes = (u64, Vec<(String, Vec<(Quantity, Outcome)>)>);

/// The tallies of every flight's outcomes: `outcomes[k]` holds flight `flights[k]`'s logs, each
/// named, with their outcomes. In the order logs are first met, then [`Quantity::ALL`]'s.
pub fn tally(flights: &[FlightOutcomes]) -> Vec<Tally> {
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
                        absent: 0,
                        worst: None,
                    });
                match *outcome {
                    Outcome::Given { error, bound } | Outcome::Floor { error, bound, .. } => {
                        tally.given += 1;
                        // A floor errs only above the truth; a NaN, which `max` would take to 0, is
                        // never a hold.
                        let size = match outcome {
                            Outcome::Floor { .. } if error.is_nan() => f64::INFINITY,
                            Outcome::Floor { .. } => error.max(0.0),
                            _ => error.abs(),
                        };
                        // A bound of 0 holds only an exact reading; NaN is never a hold.
                        let share = if bound > 0.0 {
                            size / bound
                        } else if size == 0.0 {
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
                    Outcome::Absent => tally.absent += 1,
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

/// The errors one reading gave over a set, every log of it together: a floor's apart from the
/// same reading's measured.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    /// The reading.
    pub quantity: Quantity,
    /// Whether these were given as floors.
    pub floor: bool,
    /// How many readings were given.
    pub given: u64,
    /// The finite errors' spread, the reading less the truth; `None` if none was finite.
    pub spread: Option<Spread>,
}

/// The spread of a reading's finite errors.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    /// The mean error.
    pub mean_error: f64,
    /// The least error, with its sign.
    pub least_error: f64,
    /// The most error, with its sign.
    pub most_error: f64,
    /// The mean bound the readings stated.
    pub mean_bound: f64,
}

/// The errors of every reading given in `flights`, every log together, in [`Quantity::ALL`]'s
/// order, each reading's measured before its floors. An error of infinite size, such as a main
/// named on a flight that opened one canopy, is counted in `given` and left out of the spread;
/// the bound's check fails on it.
pub fn errors(flights: &[FlightOutcomes]) -> Vec<Errors> {
    #[derive(Default)]
    struct Sum {
        given: u64,
        finite: u64,
        error: f64,
        bound: f64,
        least: Option<f64>,
        most: Option<f64>,
    }
    let mut sums: BTreeMap<(Quantity, bool), Sum> = BTreeMap::new();
    for (_, logs) in flights {
        for (_, results) in logs {
            for (quantity, outcome) in results {
                let (error, bound, floor) = match *outcome {
                    Outcome::Given { error, bound } => (error, bound, false),
                    Outcome::Floor { error, bound, .. } => (error, bound, true),
                    Outcome::Withheld(_) | Outcome::Absent => continue,
                };
                let sum = sums.entry((*quantity, floor)).or_default();
                sum.given += 1;
                if error.is_finite() && bound.is_finite() {
                    sum.finite += 1;
                    sum.error += error;
                    sum.bound += bound;
                    sum.least = Some(sum.least.map_or(error, |least| least.min(error)));
                    sum.most = Some(sum.most.map_or(error, |most| most.max(error)));
                }
            }
        }
    }
    sums.into_iter()
        .map(|((quantity, floor), sum)| {
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of readings converts exactly"
            )]
            let count = sum.finite as f64;
            Errors {
                quantity,
                floor,
                given: sum.given,
                spread: sum.least.zip(sum.most).map(|(least, most)| Spread {
                    mean_error: sum.error / count,
                    least_error: least,
                    most_error: most,
                    mean_bound: sum.bound / count,
                }),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthetic::designs::DesignFiles;
    use crate::synthetic::{Flyer, Set};

    /// The errors keep a floor apart from the same reading measured, count an infinite miss
    /// without letting it into the spread, and come in [`Quantity::ALL`]'s order.
    #[test]
    fn the_errors_keep_floors_apart() {
        let mut sorted = Quantity::ALL;
        sorted.sort();
        assert_eq!(sorted, Quantity::ALL, "the errors' map orders by `Ord`");
        let given = |error: f64| Outcome::Given { error, bound: 1.0 };
        let flights: Vec<FlightOutcomes> = vec![
            (
                0,
                vec![(
                    "a".to_owned(),
                    vec![
                        (Quantity::MainRate, given(0.5)),
                        (Quantity::Burnout, given(-0.25)),
                        (Quantity::MainOpening, given(f64::INFINITY)),
                    ],
                )],
            ),
            (
                1,
                vec![(
                    "b".to_owned(),
                    vec![
                        (
                            Quantity::Burnout,
                            Outcome::Floor {
                                error: -0.75,
                                bound: 2.0,
                                clip: None,
                            },
                        ),
                        (Quantity::Burnout, given(0.75)),
                        (Quantity::Liftoff, Outcome::Withheld(Reason::TooShort)),
                        (Quantity::DrogueRate, Outcome::Absent),
                    ],
                )],
            ),
        ];
        // None to read is counted apart, neither given nor withheld.
        let drogue = tally(&flights)
            .into_iter()
            .find(|t| t.quantity == Quantity::DrogueRate)
            .unwrap();
        assert_eq!((drogue.given, drogue.absent), (0, 1));
        assert!(drogue.withheld.is_empty());
        let errors = errors(&flights);
        let rows: Vec<(Quantity, bool, u64)> = errors
            .iter()
            .map(|row| (row.quantity, row.floor, row.given))
            .collect();
        assert_eq!(
            rows,
            [
                (Quantity::Burnout, false, 2),
                (Quantity::Burnout, true, 1),
                (Quantity::MainOpening, false, 1),
                (Quantity::MainRate, false, 1),
            ]
        );
        assert_eq!(
            errors[0].spread,
            Some(Spread {
                mean_error: 0.25,
                least_error: -0.25,
                most_error: 0.75,
                mean_bound: 1.0
            })
        );
        assert_eq!(errors[1].spread.map(|s| s.mean_bound), Some(2.0));
        assert_eq!(errors[2].spread, None);
    }

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

    /// A floor holds while its error is no more than its bound above the truth, however far below;
    /// a NaN error, which `max` would take to 0, never holds, a floor's or a reading's.
    #[test]
    fn a_floor_holds_one_side_and_never_on_nan() {
        let floor = |error: f64| Outcome::Floor {
            error,
            bound: 1.0,
            clip: None,
        };
        let holds = |outcome: Outcome| {
            tally(&[(
                0,
                vec![("a".to_owned(), vec![(Quantity::Burnout, outcome)])],
            )])[0]
                .holds()
        };
        assert!(holds(floor(-100.0)));
        assert!(holds(floor(1.0)));
        assert!(!holds(floor(1.01)));
        assert!(!holds(floor(f64::NAN)));
        assert!(!holds(Outcome::Given {
            error: f64::NAN,
            bound: 1.0
        }));
    }

    /// A flight's top acceleration is the truth's over its logger's accelerometer range, and a
    /// logger with none has no share.
    #[test]
    fn a_top_is_measured_against_its_parts_range() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut flights = (0..40).filter_map(|k| flyer.flight(Set::Tuning, k).ok());
        let with = flights
            .by_ref()
            .find(|f| f.draw.logger.profile.accelerometer_range_m_s2().is_some())
            .unwrap();
        let range = with.draw.logger.profile.accelerometer_range_m_s2().unwrap();
        let share = top_over_range(&with).unwrap();
        assert_eq!(share, top_specific_force(&with) / range);
        // A climb's top acceleration is some g, short of the 32 g and 180 g parts' ranges.
        assert!(share > 0.0 && share < 1.0, "{share}");
        let without = flights
            .find(|f| f.draw.logger.profile.accelerometer_range_m_s2().is_none())
            .unwrap();
        assert_eq!(top_over_range(&without), None);
    }

    /// The IMU-like logger's part set to its lower range: the readings state the range it clips
    /// at, and a true top past it reads as clipped at the range, a floor that holds; a part with
    /// no lower setting writes no such log.
    #[test]
    fn a_part_set_to_its_lower_range_clips_there() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut flights = (0..).filter_map(|k| flyer.flight(Set::Tuning, k).ok().map(|f| (k, f)));
        let (index, imu) = flights
            .by_ref()
            .take(40)
            .find(|(_, f)| top_over_lower_range(f).is_some_and(|share| share > 1.1))
            .unwrap();
        let lower = imu.draw.logger.profile.accelerometer_lower_range_m_s2();
        assert_eq!(
            lower,
            Some(LOWER_RANGE_G * hpr_core::gravity::STANDARD_GRAVITY_MPS2)
        );
        let log =
            accelerometer_log_set_lower(&imu.draw.logger, &imu.truth, Set::Tuning.seed(), index)
                .unwrap()
                .unwrap();
        let read = readings::read(&log);
        let top = read.max_acceleration.value().unwrap();
        assert_eq!(top.range_m_s2, lower, "{top:?}");
        assert_eq!(top.clip, Some(Clip::Range), "{top:?}");
        assert!(top.is_floor, "{top:?}");
        let outcome = outcomes(&imu, &log)
            .into_iter()
            .find(|(quantity, _)| *quantity == Quantity::TopAcceleration)
            .map(|(_, outcome)| outcome);
        let Some(Outcome::Floor { error, bound, clip }) = outcome else {
            panic!("{outcome:?}");
        };
        assert_eq!(clip, Some(Clip::Range));
        assert!(error < 0.0 && error <= bound, "{error}, {bound}");
        let (index, altos) = flights
            .take(40)
            .find(|(_, f)| {
                f.draw.logger.profile.accelerometer_range_m_s2().is_some()
                    && top_over_lower_range(f).is_none()
            })
            .unwrap();
        assert!(
            accelerometer_log_set_lower(
                &altos.draw.logger,
                &altos.truth,
                Set::Tuning.seed(),
                index
            )
            .unwrap()
            .is_none()
        );
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
        let accelerometers = if flight
            .log
            .channels
            .iter()
            .any(|channel| channel.quantity == hpr_flightdata::log::Quantity::Acceleration)
        {
            ACCELEROMETER_RATES_HZ.len()
        } else {
            0
        } + usize::from(
            flight
                .draw
                .logger
                .profile
                .accelerometer_lower_range_m_s2()
                .is_some(),
        );
        assert_eq!(
            logs.len(),
            1 + grid().len() + accelerometers + CROPPED.len() * Crop::ALL.len()
        );
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
            if let (Quantity::ApogeeHeight, Outcome::Given { error, bound }) = (quantity, outcome) {
                *error = 1.01 * *bound;
            }
        }
        let tallies = tally(&[(index, missed)]);
        assert!(!tallies[1].holds(), "{:?}", tallies[1]);
        assert_eq!(tallies[1].quantity, Quantity::ApogeeHeight);
    }
}
