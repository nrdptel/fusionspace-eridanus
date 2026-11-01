//! Synthetic flights: HPR Sim's flight of a drawn rocket, its truth, and the log a drawn logger
//! would write of it, for testing what a log's readings and fits claim against a known answer.
//!
//! **What it is for.** A reading taken from a log, or a parameter fitted to one, states a 90%
//! interval, and [ADR-257][adr-257] asks that each interval be tested on at least 1,000 flights
//! whose truth is known: it must miss about 5% of the time on each side. Only a simulated flight
//! has a known truth. This module draws those flights, on seeds fixed before any interval exists,
//! as [ADR-280][adr-280] §2–3 sets out; [M7.2][m7-2]'s readings and smoother and [M7.3][m7-3]'s
//! fits are checked on them.
//!
//! **How far to trust it.** The truth is HPR Sim's own flight, so a check on these flights shows
//! how well a method recovers HPR Sim's trajectory from noisy sensors, not how well HPR Sim
//! predicts a real flight. The noise is the makers' datasheets and the noise measured in public
//! logs; set [`Set::B`] flies a model shaped differently from HPR Sim's in three named ways, so
//! that a method can't pass by knowing HPR Sim's model. What the noise leaves out is listed on
//! the guide's page `physics/synthetic-flights.md`.
//!
//! **The sets.** [`Set::Tuning`], 200 flights, for anything fitted or tuned; [`Set::A`], 1,000
//! flights of HPR Sim's model; [`Set::B`], 1,000 differing from it; and [`Set::Calibration`],
//! 2,000 drawn as the tuning set is, for the factors that scale the readings' 90% intervals
//! ([`intervals`]), which [ADR-257][adr-257] §1, calibrated intervals, asks be fitted on seeds
//! of their own. Flight `k` of a set draws
//! each group of inputs (its design, motor, drag, wind, rail, site, model and logger) from its
//! own stream, keyed by the set's seed, `k` and the group ([`SeededRng::for_stream`]), so a set's
//! first flights don't change if it grows, and an input added to one group moves no other group.
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
//! [m7-2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2
//! [m7-3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3

pub mod census;
pub mod designs;
pub mod held;
pub mod intervals;
pub mod logger;
pub mod noise;
pub mod paired;
pub mod smoothed;
pub mod truth;

use std::collections::BTreeMap;

use hpr_core::random::SeededRng;
use hpr_flightdata::log::FlightLog;
use hpr_sim::Simulation;
use serde::{Deserialize, Serialize};

use designs::{DESIGNS, DesignFiles};
use logger::{LoggerDraw, Profile};
use noise::between;
use truth::Truth;

/// A set of synthetic flights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Set {
    /// 200 flights for anything fitted or tuned, alternating set A's draws (even `k`) and set
    /// B's (odd `k`).
    Tuning,
    /// 1,000 flights of HPR Sim's model.
    A,
    /// 1,000 flights of a model differing from HPR Sim's: drag against Mach number, wind shear
    /// and the thrust curve's shape.
    B,
    /// 2,000 flights drawn as the tuning set's are, alternating set A's model and set B's, to fit
    /// the factors that scale the readings' 90% intervals on seeds of their own ([ADR-257][adr-257]
    /// §1: "any factor that scales the intervals is fitted on other seeds"). Its seed was fixed on
    /// 2026-10-29, before any reading's interval existed. It isn't in [`Set::ALL`]: the census and
    /// the smoother's check don't fly it.
    ///
    /// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
    Calibration,
    /// 2,000 flights drawn as the calibration set's are, each with two loggers and clocks of
    /// their own, to fit the clock offset's interval ([`paired`]). Not in [`Set::ALL`], as none of
    /// the sets below is: the census doesn't fly them.
    PairCalibration,
    /// 1,000 flights of HPR Sim's model, each with two loggers and clocks of their own.
    PairA,
    /// 1,000 flights of the model set B flies, each with two loggers and clocks of their own.
    PairB,
    /// 200 flights of the two-stage design, a logger in each stage ([`paired::Staged`]).
    Staged,
}

impl Set {
    /// The census's sets: the tuning set and the two test sets.
    pub const ALL: [Self; 3] = [Self::Tuning, Self::A, Self::B];

    /// Its seed, fixed on 2026-10-17 before any interval existed ([ADR-280][adr-280] §2, the
    /// synthetic flights first). A seed changed after a failure needs a decision record of its
    /// own ([ADR-257][adr-257] §1, calibrated intervals).
    ///
    /// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
    /// [adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
    pub fn seed(self) -> u64 {
        match self {
            Self::Tuning => 2_026_101_700,
            Self::A => 2_026_101_701,
            Self::B => 2_026_101_702,
            Self::Calibration => 2_026_102_900,
            Self::PairCalibration => 2_026_103_100,
            Self::PairA => 2_026_103_101,
            Self::PairB => 2_026_103_102,
            Self::Staged => 2_026_103_103,
        }
    }

    /// Its flights.
    pub fn count(self) -> u64 {
        match self {
            Self::Tuning => 200,
            Self::A | Self::B | Self::PairA | Self::PairB => 1_000,
            Self::Calibration | Self::PairCalibration => 2_000,
            Self::Staged => 200,
        }
    }

    /// Its name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Tuning => "tuning",
            Self::A => "a",
            Self::B => "b",
            Self::Calibration => "calibration",
            Self::PairCalibration => "pair-calibration",
            Self::PairA => "pair-a",
            Self::PairB => "pair-b",
            Self::Staged => "staged",
        }
    }

    /// Whether flight `index` draws set B's model.
    fn other_model(self, index: u64) -> bool {
        match self {
            Self::Tuning | Self::Calibration | Self::PairCalibration => index % 2 == 1,
            Self::A | Self::PairA | Self::Staged => false,
            Self::B | Self::PairB => true,
        }
    }
}

/// The model a flight flies.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "model", rename_all = "kebab-case")]
pub enum Model {
    /// HPR Sim's: its drag, a wind the same at every height, the motor's own thrust curve.
    Same,
    /// Set B's: the drag times `1 + bump·exp(−((M − 1)/0.2)²) + slope·M`, a power-law wind, the
    /// thrust tilted by `1 + tilt·(2t/t_b − 1)` at the same impulse.
    Other {
        /// The transonic bump's height, −0.2–0.2.
        drag_bump: f64,
        /// The drag's slope per unit Mach number, −0.05–0.05.
        drag_slope: f64,
        /// The wind's power-law exponent, 0.14–0.20 (NASA/TM-2008-215633 §2.2.5.2).
        shear_exponent: f64,
        /// The thrust's tilt, −0.3–0.3.
        thrust_tilt: f64,
    },
}

/// Everything one synthetic flight draws.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Draw {
    /// Its set.
    pub set: Set,
    /// Its index in the set.
    pub index: u64,
    /// The design, an index into [`DESIGNS`].
    pub design: usize,
    /// The motor's impulse factor, 0.94–1.06.
    pub impulse_scale: f64,
    /// Its burn time's factor, 0.90–1.10.
    pub burn_time_scale: f64,
    /// The zero-lift drag's factor, 0.8–1.2.
    pub drag_scale: f64,
    /// The wind's speed, 0–8.9 m/s (at 18.3 m in set B).
    pub wind_speed_m_s: f64,
    /// The direction it blows from, rad clockwise from north.
    pub wind_from_rad: f64,
    /// The rail's angle from vertical, 0–10°, rad.
    pub tilt_rad: f64,
    /// The direction it leans toward, rad clockwise from north.
    pub azimuth_rad: f64,
    /// The site's elevation above mean sea level, 0–1,500 m.
    pub elevation_m: f64,
    /// The air's temperature offset from the 1976 standard, −15–15 K.
    pub temperature_offset_k: f64,
    /// The model it flies.
    pub model: Model,
    /// Its logger.
    pub logger: LoggerDraw,
}

/// What a draw's inputs are keyed by, each its own stream.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
enum Key {
    Design = 1,
    Motor = 2,
    Drag = 3,
    Wind = 4,
    Rail = 5,
    Site = 6,
    Model = 7,
    Logger = 8,
    BaroCorrelated = 9,
    BaroWhite = 10,
    Accel = 11,
    Gnss = 12,
    BaroSlow = 13,
    SecondLogger = 14,
    Clocks = 15,
}

/// The key of the stream the smoother's check draws its moment from ([`smoothed::moment_drawn`]):
/// after every [`Key`] its sets' flights draw, so no flight's other draws move. [`Key::SecondLogger`]
/// shares its number; the two-logger sets, which alone draw that key, don't run the smoother's check.
pub(crate) const SMOOTHER_MOMENT_KEY: u64 = 14;

/// Flight `index`'s stream for `key`.
fn stream(seed: u64, index: u64, key: Key) -> SeededRng {
    SeededRng::for_stream(seed, &[index, key as u64])
}

/// The most the wind blows, m/s: 20 mph, the most the NAR's high-power safety code allows.
pub const MAX_WIND_M_S: f64 = 20.0 * 0.447_04;
/// The rail's largest angle from vertical, degrees, inside the safety code's 20°.
pub const MAX_TILT_DEG: f64 = 10.0;

/// Draws flight `index` of `set`.
pub fn draw(set: Set, index: u64) -> Draw {
    let seed = set.seed();
    let turn = std::f64::consts::TAU;
    let design = {
        let mut rng = stream(seed, index, Key::Design);
        ((rng.uniform() * DESIGNS.len() as f64) as usize).min(DESIGNS.len() - 1)
    };
    let mut motor = stream(seed, index, Key::Motor);
    let impulse_scale = between(&mut motor, 0.94, 1.06);
    let burn_time_scale = between(&mut motor, 0.90, 1.10);
    let drag_scale = between(&mut stream(seed, index, Key::Drag), 0.8, 1.2);
    let mut wind = stream(seed, index, Key::Wind);
    let wind_speed_m_s = between(&mut wind, 0.0, MAX_WIND_M_S);
    let wind_from_rad = between(&mut wind, 0.0, turn);
    let mut rail = stream(seed, index, Key::Rail);
    let tilt_rad = between(&mut rail, 0.0, MAX_TILT_DEG).to_radians();
    let azimuth_rad = between(&mut rail, 0.0, turn);
    let mut site = stream(seed, index, Key::Site);
    let elevation_m = between(&mut site, 0.0, 1_500.0);
    let temperature_offset_k = between(&mut site, -15.0, 15.0);
    let model = if set.other_model(index) {
        let mut rng = stream(seed, index, Key::Model);
        Model::Other {
            drag_bump: between(&mut rng, -0.2, 0.2),
            drag_slope: between(&mut rng, -0.05, 0.05),
            shear_exponent: between(&mut rng, 0.14, 0.20),
            thrust_tilt: between(&mut rng, -0.3, 0.3),
        }
    } else {
        Model::Same
    };
    let mut logger = stream(seed, index, Key::Logger);
    let profile = Profile::ALL
        [((logger.uniform() * Profile::ALL.len() as f64) as usize).min(Profile::ALL.len() - 1)];
    Draw {
        set,
        index,
        design,
        impulse_scale,
        burn_time_scale,
        drag_scale,
        wind_speed_m_s,
        wind_from_rad,
        tilt_rad,
        azimuth_rad,
        elevation_m,
        temperature_offset_k,
        model,
        logger: LoggerDraw::draw(&mut logger, profile),
    }
}

/// Why a synthetic flight couldn't be made.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SyntheticError {
    /// HPR Sim refused or failed the flight.
    #[error("the flight: {0}")]
    Sim(#[from] hpr_sim::SimError),
    /// The motor couldn't be scaled.
    #[error("the motor: {0}")]
    Analysis(#[from] hpr_analysis::AnalysisError),
    /// The motor couldn't be rebuilt.
    #[error("the motor: {0}")]
    Motor(#[from] hpr_motor::MotorError),
    /// The air or the wind refused its inputs.
    #[error("the air: {0}")]
    Atmos(#[from] hpr_atmos::AtmosError),
    /// The site or its frame refused its inputs.
    #[error("the site: {0}")]
    Core(#[from] hpr_core::CoreError),
    /// A design's JSON didn't read.
    #[error("a design: {0}")]
    Json(#[from] serde_json::Error),
    /// A design isn't as this module expects.
    #[error("design {design} {what}")]
    Design {
        /// The design.
        design: &'static str,
        /// What is wrong.
        what: &'static str,
    },
    /// The flight came down without an event a truth needs.
    #[error("the flight came down with {what}")]
    Flight {
        /// The missing event.
        what: &'static str,
    },
    /// An input no flight or logger can have.
    #[error("{what}")]
    Invalid {
        /// What was wrong.
        what: &'static str,
    },
}

/// A synthetic flight: what it drew, its truth and its logger's log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    /// What it drew.
    pub draw: Draw,
    /// Its truth.
    pub truth: Truth,
    /// Its logger's log.
    pub log: FlightLog,
}

/// Flies synthetic flights, sharing each design's supersonic drag table across its flights (it
/// takes about 0.2 s to build, `docs/perf.md`).
#[derive(Debug)]
pub struct Flyer {
    files: DesignFiles,
    nominal: BTreeMap<usize, Simulation>,
}

impl Flyer {
    /// A flyer of the designs in `files`, with no tables built yet.
    pub fn new(files: DesignFiles) -> Self {
        Self {
            files,
            nominal: BTreeMap::new(),
        }
    }

    /// Flight `index` of `set`.
    ///
    /// # Errors
    ///
    /// [`SyntheticError`] if the draw can't fly or its log can't be written: a census counts
    /// these, it doesn't skip them.
    pub fn flight(&mut self, set: Set, index: u64) -> Result<Flight, SyntheticError> {
        let draw = draw(set, index);
        let inputs = truth::inputs(&draw, &self.files)?;
        let mut simulation = inputs.simulation()?;
        let shared = match self.nominal.get(&draw.design) {
            Some(nominal) => simulation.share_supersonic_table(nominal.aero()),
            None => false,
        };
        let truth = truth::fly(&draw, &inputs, &simulation);
        if !shared && !self.nominal.contains_key(&draw.design) {
            // The design's first flight keeps its table for the rest.
            self.nominal.insert(draw.design, simulation);
        }
        let truth = truth?;
        let log = logger::log(&draw.logger, &truth, set.seed(), index)?;
        Ok(Flight { draw, truth, log })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether every one of `values` lies in `[lo, hi]` and their least and greatest come within
    /// `6/n` of the range of its ends: `n` uniform draws leave a gap wider than that at one end
    /// with odds `(1 − 6/n)ⁿ ≈ e⁻⁶`, 0.25%; a range drawn even 10% short fails.
    fn fills(values: &[f64], lo: f64, hi: f64) -> bool {
        let (least, greatest) = values
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &v| {
                (a.min(v), b.max(v))
            });
        let slack = 6.0 / values.len() as f64 * (hi - lo);
        values.iter().all(|v| (lo..=hi).contains(v)) && least < lo + slack && greatest > hi - slack
    }

    #[test]
    fn every_draw_stays_inside_its_range_and_fills_it() {
        for set in Set::ALL.into_iter().chain([
            Set::Calibration,
            Set::PairCalibration,
            Set::PairA,
            Set::PairB,
        ]) {
            let draws: Vec<Draw> = (0..set.count()).map(|k| draw(set, k)).collect();
            let column = |f: fn(&Draw) -> f64| draws.iter().map(f).collect::<Vec<_>>();
            assert!(fills(&column(|d| d.impulse_scale), 0.94, 1.06), "{set:?}");
            assert!(fills(&column(|d| d.burn_time_scale), 0.90, 1.10), "{set:?}");
            assert!(fills(&column(|d| d.drag_scale), 0.8, 1.2), "{set:?}");
            assert!(
                fills(&column(|d| d.wind_speed_m_s), 0.0, MAX_WIND_M_S),
                "{set:?}"
            );
            assert!(
                fills(&column(|d| d.wind_from_rad), 0.0, std::f64::consts::TAU),
                "{set:?}"
            );
            assert!(
                fills(&column(|d| d.tilt_rad.to_degrees()), 0.0, MAX_TILT_DEG),
                "{set:?}"
            );
            assert!(
                fills(&column(|d| d.azimuth_rad), 0.0, std::f64::consts::TAU),
                "{set:?}"
            );
            assert!(fills(&column(|d| d.elevation_m), 0.0, 1_500.0), "{set:?}");
            assert!(
                fills(&column(|d| d.temperature_offset_k), -15.0, 15.0),
                "{set:?}"
            );
            for design in 0..DESIGNS.len() {
                assert!(
                    draws.iter().any(|d| d.design == design),
                    "{set:?} lacks design {design}"
                );
            }
            for profile in Profile::ALL {
                assert!(
                    draws.iter().any(|d| d.logger.profile == profile),
                    "{set:?} lacks {profile:?}"
                );
            }
            let others: Vec<[f64; 4]> = draws
                .iter()
                .filter_map(|d| match d.model {
                    Model::Same => None,
                    Model::Other {
                        drag_bump,
                        drag_slope,
                        shear_exponent,
                        thrust_tilt,
                    } => Some([drag_bump, drag_slope, shear_exponent, thrust_tilt]),
                })
                .collect();
            let other = |k: usize| others.iter().map(|o| o[k]).collect::<Vec<_>>();
            match set {
                Set::A | Set::PairA | Set::Staged => assert!(others.is_empty()),
                Set::B | Set::PairB => assert_eq!(others.len(), draws.len()),
                Set::Tuning | Set::Calibration | Set::PairCalibration => {
                    let alternate = draws
                        .iter()
                        .all(|d| matches!(d.model, Model::Other { .. }) == (d.index % 2 == 1));
                    assert!(alternate);
                }
            }
            if !others.is_empty() {
                assert!(fills(&other(0), -0.2, 0.2), "{set:?}");
                assert!(fills(&other(1), -0.05, 0.05), "{set:?}");
                assert!(fills(&other(2), 0.14, 0.20), "{set:?}");
                assert!(fills(&other(3), -0.3, 0.3), "{set:?}");
            }
        }
    }

    #[test]
    fn the_sets_draw_on_their_own_seeds() {
        let seeds: Vec<u64> = Set::ALL.iter().map(|s| s.seed()).collect();
        assert_eq!(seeds, [2_026_101_700, 2_026_101_701, 2_026_101_702]);
        assert_eq!(Set::ALL.map(Set::count), [200, 1_000, 1_000]);
        assert_eq!(Set::Calibration.seed(), 2_026_102_900);
        assert_eq!(Set::Calibration.count(), 2_000);
        assert!(!Set::ALL.contains(&Set::Calibration));
        // The same index in two sets draws two different flights.
        let (a, b) = (draw(Set::A, 3), draw(Set::B, 3));
        assert_ne!(a.impulse_scale, b.impulse_scale);
        assert_ne!(a.logger.pad_s, b.logger.pad_s);
    }

    #[test]
    fn a_flight_drawn_again_is_bit_identical() {
        // Two flights of one design: the second shares the first's supersonic table when flown
        // after it, and builds its own when flown alone; either way it comes out the same.
        let first = 0;
        let design = draw(Set::B, first).design;
        let second = (1..).find(|&k| draw(Set::B, k).design == design).unwrap();
        let mut flyer = Flyer::new(DesignFiles::committed());
        let earlier = flyer.flight(Set::B, first).unwrap();
        let after = flyer.flight(Set::B, second).unwrap();
        let alone = Flyer::new(DesignFiles::committed())
            .flight(Set::B, second)
            .unwrap();
        let text = |flight: &Flight| serde_json::to_string(flight).unwrap();
        assert_eq!(text(&after), text(&alone));
        assert_eq!(
            text(&earlier),
            text(
                &Flyer::new(DesignFiles::committed())
                    .flight(Set::B, first)
                    .unwrap()
            )
        );
        let bits = |flight: &Flight| -> Vec<u64> {
            let channels = flight.log.channels.iter().flat_map(|c| c.values.iter());
            let samples = flight.truth.samples.iter().map(|s| &s.height_m);
            flight
                .log
                .altitude_m
                .iter()
                .chain(channels)
                .chain(samples)
                .map(|v| v.to_bits())
                .collect()
        };
        assert_eq!(bits(&after), bits(&alone));
        // And another flight differs.
        assert_ne!(text(&earlier), text(&after));
        assert!(!after.log.time_s.is_empty() || !after.log.channels.is_empty());
    }
}
