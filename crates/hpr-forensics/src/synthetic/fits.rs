//! The least-squares fit's 90% intervals held to the synthetic flights ([M7.3b][m7-3],
//! [ADR-257][adr-257] §1, [ADR-316][adr-316]).
//!
//! Each flight's log is fitted ([`crate::fit`]), told what a flyer knows: the design, its motor at
//! its rated impulse and burn, the rail, the site and the day's air, and the structure's mass as
//! weighed, [`weighed_factor`] off the truth. The drag scale, impulse scale, wind and mass are each
//! compared with the flight's truth by the same three counts as the readings'
//! ([`super::intervals::Test`]), with the same three probes.
//!
//! **The truth.** On [`Set::A`] a parameter's truth is what the flight drew. On
//! [`Set::B`], whose drag curve, wind and thrust are shaped otherwise, it is the fit
//! to the noise-free flight ([`noise_free`]): the same bins of height and drift, as the sensor would
//! have read them with no noise, with the sensors' unknowns open as a log's are. The mass's truth is what the
//! flight drew on both.
//!
//! **What a log can tell.** A parameter counts as identified only when its median interval over
//! the flights is at most half the width of the 90% interval of its stated range with no log:
//! [`Identification`]. Pure noise in place of the log must leave every parameter not identified
//! ([`noise_log`]).
//!
//! [m7-3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md

use std::collections::BTreeMap;

use hpr_atmos::PowerLawWind;
use hpr_core::random::SeededRng;
use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings;
use hpr_flightdata::readings::interval::{Interval, Shares, Z_90};
use hpr_sim::Simulation;
use serde::{Deserialize, Serialize};

use super::designs::{DESIGNS, DesignFiles};
use super::intervals::{Counts, Probe, Test};
use super::logger::Profile;
use super::noise::between;
use super::truth::{self};
use super::{Draw, Flight, Model, Set, SyntheticError};
use crate::fit::{
    self, FLOWN, Fit, Forward, Noise, Observations, Parameters, Prior, Sampling, Stated,
};
use crate::posterior::Posterior;
use crate::residuals::{HeightKind, Simulated, Track};

/// The parameters asked of the fit, in the report's order.
pub const ASKED: [Asked; 5] = [
    Asked::DragScale,
    Asked::ImpulseScale,
    Asked::WindEast,
    Asked::WindNorth,
    Asked::MassScale,
];

/// How a fit states a parameter's 90% interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Method {
    /// The least-squares fit's standard error times the shares fitted on the calibration set
    /// ([`shares`]).
    LeastSquares,
    /// The Bayesian option's 5% and 95% points as they come ([`fit::bayesian`]).
    Bayesian,
}

impl Method {
    /// Both, in the report's order.
    pub const ALL: [Self; 2] = [Self::LeastSquares, Self::Bayesian];

    /// Its name in the report.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::LeastSquares => "least squares",
            Self::Bayesian => "Bayesian",
        }
    }
}

/// The pilot's draws the Bayesian option makes of a flight's posterior, which widen its proposal
/// where the posterior reaches further ([`crate::posterior::sample_in_two`]), and the draws read
/// after them; each draw flies twice, once for the middle of the unknowns other than the wind and
/// once for its weight. By estimate a few hundred keep 100 effective draws
/// ([`crate::posterior::MIN_EFFECTIVE_DRAWS`]) on a posterior near its peak's Gaussian. A judgment;
/// the report counts the flights where they fell short.
pub const POSTERIOR_PILOT: usize = 100;
/// And the draws read.
pub const POSTERIOR_DRAWS: usize = 400;

/// How flight `index` of `set` draws its posterior: [`POSTERIOR_PILOT`] and [`POSTERIOR_DRAWS`],
/// on a seed of its own.
#[must_use]
pub fn sampling(set: Set, index: u64) -> Sampling {
    Sampling {
        pilot: POSTERIOR_PILOT,
        draws: POSTERIOR_DRAWS,
        seed: SeededRng::for_stream(set.seed(), &[index, super::POSTERIOR_KEY]).next_u64(),
    }
}

/// A parameter asked of the fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Asked {
    /// The drag scale.
    DragScale,
    /// The impulse scale.
    ImpulseScale,
    /// The constant wind's speed toward the east, m/s.
    WindEast,
    /// Its speed toward the north, m/s.
    WindNorth,
    /// The factor on the weighed mass.
    MassScale,
}

impl Asked {
    /// Its name in the report.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::DragScale => "drag scale",
            Self::ImpulseScale => "impulse scale",
            Self::WindEast => "wind toward the east, m/s",
            Self::WindNorth => "wind toward the north, m/s",
            Self::MassScale => "mass on the weighed mass",
        }
    }

    /// Whether a log of height alone states an interval on it. The wind's doesn't: nothing in a
    /// height speaks to it, and with nothing from the log an interval is only the range stated, which
    /// can't be right on both sets, as set B's shear widens its truth ([ADR-316][adr-316], the
    /// fit's decisions).
    ///
    /// [adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
    #[must_use]
    pub fn stated_on_height_alone(self) -> bool {
        !matches!(self, Self::WindEast | Self::WindNorth)
    }

    /// Its place in [`Parameters::to_array`].
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::DragScale => 0,
            Self::ImpulseScale => 1,
            Self::WindEast => 3,
            Self::WindNorth => 4,
            Self::MassScale => 5,
        }
    }

    /// The 90% interval's width that its stated range gives with no log: the 5% to 95% points of
    /// what the flights draw. A wind component is a uniform speed along a uniform direction, whose
    /// points are [`wind_component_point`] either way.
    #[must_use]
    pub fn stated_width(self) -> f64 {
        match self {
            Self::DragScale => 0.9 * 0.4,
            Self::ImpulseScale => 0.9 * 0.12,
            Self::WindEast | Self::WindNorth => 2.0 * wind_component_point(),
            Self::MassScale => 0.9 * 2.0 * WEIGHED_WITHIN,
        }
    }
}

/// What's stated of a flight read as `observed`. A log of height alone can't tell the wind, and a
/// wind of either sign lowers a flight's top, so a fit in no wind would put that loss in the drag
/// and impulse: by 0.7% of the impulse in a wind of 6 to 9 m/s, on the first 240 flights of set A,
/// and by nothing under 3 m/s. It flies instead in a wind of the stated range's RMS speed, toward
/// the east, which leaves the mean of that loss out of them; the wind's way, which a tilted rail
/// would tell, is left to the wind's own stated range.
#[must_use]
pub fn stated_for(observed: Option<&Observations>) -> Stated {
    let mut stated = stated();
    if observed.is_some_and(|o| o.horizontal.is_empty()) {
        stated[Asked::WindEast.index()].mean = super::MAX_WIND_M_S / 3.0_f64.sqrt();
    }
    stated
}

/// How far a flight's weighed mass lies from its structure's, a share of it, either way: a
/// kitchen scale's and a loaded airframe's error, the maintainer's judgment; drawn uniform.
pub const WEIGHED_WITHIN: f64 = 0.02;

/// The 95% point of a wind component drawn as [`super::draw`] draws it, m/s: `x` with
/// `P(v sin θ ≤ x) = 0.95` for `v` uniform on `0..V` and `θ` uniform on a turn, found by bisection
/// on a midpoint sum over the direction.
#[must_use]
pub fn wind_component_point() -> f64 {
    let v_max = super::MAX_WIND_M_S;
    let upper_share = |x: f64| {
        // P(v sin θ > x) = mean over θ in the half turn of (1 − x/(V sin θ))⁺, over 2.
        let steps = 20_000_u32;
        let sum: f64 = (0..steps)
            .map(|k| {
                let theta = std::f64::consts::PI * (f64::from(k) + 0.5) / f64::from(steps);
                (1.0 - x / (v_max * theta.sin())).max(0.0)
            })
            .sum();
        0.5 * sum / f64::from(steps)
    };
    let (mut low, mut high) = (0.0, v_max);
    for _ in 0..60 {
        let middle = 0.5 * (low + high);
        if upper_share(middle) > 0.05 {
            low = middle;
        } else {
            high = middle;
        }
    }
    0.5 * (low + high)
}

/// What the fit is told of each parameter: normal, so that with no log its 90% interval is its
/// stated range's. The drag and impulse scales are centered on their rated 1; the burn time's range
/// is the draw's. These are the flights' stated ranges, so they mean something on these flights
/// alone; a caller on a real flight states its own.
#[must_use]
pub fn stated() -> Stated {
    let of_width = |mean: f64, width: f64| Prior {
        mean,
        sd: width / (2.0 * Z_90),
    };
    [
        of_width(1.0, Asked::DragScale.stated_width()),
        of_width(1.0, Asked::ImpulseScale.stated_width()),
        of_width(1.0, 0.9 * 0.2),
        of_width(0.0, Asked::WindEast.stated_width()),
        of_width(0.0, Asked::WindNorth.stated_width()),
        of_width(1.0, Asked::MassScale.stated_width()),
    ]
}

/// The factor between flight `index`'s weighed structure and its true one, from a stream of its own,
/// so that no flight's draw moves.
#[must_use]
pub fn weighed_factor(set: Set, index: u64) -> f64 {
    let mut rng = SeededRng::for_stream(set.seed(), &[index, super::WEIGHING_KEY]);
    between(&mut rng, 1.0 - WEIGHED_WITHIN, 1.0 + WEIGHED_WITHIN)
}

/// Flies a draw's design for a fit: told the rail, site and air, its motor rated, its wind and
/// scales as asked. The structure's mass is `weighed` times the true one, times the asked factor.
#[derive(Debug)]
pub struct Flown<'a> {
    draw: Draw,
    files: &'a DesignFiles,
    weighed: f64,
    tables: &'a mut BTreeMap<usize, Simulation>,
}

impl<'a> Flown<'a> {
    /// The fit's flights of `draw`.
    pub fn new(
        draw: &Draw,
        files: &'a DesignFiles,
        weighed: f64,
        tables: &'a mut BTreeMap<usize, Simulation>,
    ) -> Self {
        Self {
            draw: *draw,
            files,
            weighed,
            tables,
        }
    }
}

impl Forward for Flown<'_> {
    fn fly(&mut self, p: &Parameters, until_s: f64) -> Result<Simulated, String> {
        self.flown(p, until_s, None)
    }

    /// The stated wind is at [`truth::SHEAR_REFERENCE_HEIGHT_M`] above the pad, as the draws'.
    fn fly_sheared(
        &mut self,
        p: &Parameters,
        until_s: f64,
        exponent: f64,
    ) -> Result<Simulated, String> {
        self.flown(p, until_s, Some(exponent))
    }
}

impl Flown<'_> {
    /// The flight on `p`, its wind constant or growing with height by `shear`.
    fn flown(
        &mut self,
        p: &Parameters,
        until_s: f64,
        shear: Option<f64>,
    ) -> Result<Simulated, String> {
        if p.drag_scale <= 0.0 || p.impulse_scale <= 0.0 || p.burn_time_scale <= 0.0 {
            return Err("a scale that isn't positive".to_owned());
        }
        let wind_speed_m_s = p.wind_east_m_s.hypot(p.wind_north_m_s);
        // It blows toward (east, north), so from the opposite way, clockwise from north.
        let wind_from_rad = (-p.wind_east_m_s)
            .atan2(-p.wind_north_m_s)
            .rem_euclid(std::f64::consts::TAU);
        let told = Draw {
            impulse_scale: p.impulse_scale,
            burn_time_scale: p.burn_time_scale,
            drag_scale: p.drag_scale,
            wind_speed_m_s,
            wind_from_rad,
            model: Model::Same,
            ..self.draw
        };
        let mut inputs = truth::inputs(&told, self.files).map_err(|e| e.to_string())?;
        if let Some(exponent) = shear {
            inputs.environment = inputs.environment.with_wind(
                PowerLawWind::new(
                    wind_speed_m_s,
                    truth::SHEAR_REFERENCE_HEIGHT_M,
                    exponent,
                    wind_from_rad,
                    told.elevation_m,
                )
                .map_err(|e| e.to_string())?,
            );
        }
        inputs.recovery.clear();
        inputs.settings.max_time_s = until_s;
        let mut simulation = inputs.simulation().map_err(|e| e.to_string())?;
        match self.tables.get(&told.design) {
            Some(nominal) => {
                simulation.share_supersonic_table(nominal.aero());
            }
            None => {
                let nominal = inputs.simulation().map_err(|e| e.to_string())?;
                self.tables.insert(told.design, nominal);
            }
        }
        let simulation = simulation
            .with_structure_mass_scale(self.weighed * p.mass_scale)
            .map_err(|e| e.to_string())?;
        let mut track = Track::new();
        simulation.run(&mut track).map_err(|e| e.to_string())?;
        track
            .simulated(&inputs.environment)
            .map_err(|e| e.to_string())
    }
}

/// A fit's reading of one parameter against its truth.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Compared {
    /// The fit's value.
    pub value: f64,
    /// Its standard error.
    pub standard_error: f64,
    /// The truth.
    pub truth: f64,
}

impl Compared {
    /// `(value − truth)/standard error`, what the shares are fitted on.
    #[must_use]
    pub fn share(&self) -> f64 {
        (self.value - self.truth) / self.standard_error
    }
}

/// One flight's fit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightFit {
    /// The flight's index in its set.
    pub index: u64,
    /// The design's index in [`DESIGNS`].
    pub design: usize,
    /// Whether the log was fitted, or said nothing.
    pub fitted: bool,
    /// Whether its log was fitted and holds a horizontal track.
    pub drift: bool,
    /// Whether its fit, and any noise-free fit for its truth, stopped at the bottom.
    pub converged: bool,
    /// Its fit's residual as a share of the spread of the heights it read, where it came to one.
    pub misfit_share: Option<f64>,
    /// Each asked parameter, in [`ASKED`]'s order.
    pub compared: [Compared; ASKED.len()],
    /// The heights' residual's RMS, m.
    pub rms_m: Option<f64>,
    /// Flights flown for it, the noise-free fit's included, the Bayesian option's draws not.
    pub flown: u64,
    /// The Bayesian option's posterior of the flown parameters, where it was asked for and the
    /// least-squares fit had something to fit.
    pub posterior: Option<Posterior<FLOWN>>,
}

/// The bins of `observed` that its fit read (`read`, [`Fit::read`]) as the noise-free flight fills
/// them, on the flight's own clock: what the log's sensors would have read with no noise, no gain,
/// no drift and no port, in the same bins, so that the fit to them differs from the log's by the
/// noise alone.
///
/// # Errors
///
/// [`SyntheticError`] if the flight's air doesn't reach it.
pub fn noise_free_observations(
    observed: &Observations,
    read: &[bool],
    flight: &Flight,
    files: &DesignFiles,
) -> Result<Observations, SyntheticError> {
    let environment = truth::inputs(&flight.draw, files)?.environment;
    let flown = Simulated::new(flight.truth.samples.clone(), &environment).map_err(|_| {
        SyntheticError::Flight {
            what: "a truth the day's air doesn't reach",
        }
    })?;
    if read.len() != observed.heights.len() {
        return Err(SyntheticError::Flight {
            what: "a fit's bins that aren't this log's",
        });
    }
    let to_flight = |t: f64| t - flight.draw.logger.pad_s;
    Ok(Observations {
        kind: observed.kind,
        // The bins the log's fit read, and no others: a fit reads a different part of the flight
        // where it finds the rocket faster or slower, and a drag that isn't a scale of the
        // simulation's would then differ by more than the noise.
        heights: observed
            .heights
            .iter()
            .zip(read)
            .filter(|(_, read)| **read)
            .map(|(&(t, _), _)| {
                let at = to_flight(t);
                (at, flown.read_height_m(at, observed.kind))
            })
            .collect(),
        // The drift at every bin, whatever the log's own sensors held: a flight's truth is what the
        // whole flight says of it, and a log of height alone can't tell the wind's loss from the
        // drag's, which its errors must then hold.
        horizontal: observed
            .heights
            .iter()
            .map(|&(t, _)| {
                let at = to_flight(t);
                let (east, north) = flown.drift_m(at);
                (at, east, north)
            })
            .collect(),
        liftoff_s: flight.truth.events.liftoff_s,
        flown_s: observed.flown_s,
        // Those bins are already chosen.
        reads_transonic: true,
    })
}

/// The parameters' truth on a flight of another model: the fit to the noise-free flight
/// ([ADR-257][adr-257] §1). Where the log said anything it is the same fit as `answer`, on the same
/// bins at the same noise with the sensors' unknowns open, so that the two differ by the noise alone.
/// The structure's mass is held as flown. Where the log said nothing, the flight's height and drift
/// known exactly.
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
///
/// # Errors
///
/// [`SyntheticError`] if the flight's air doesn't reach it.
pub fn noise_free(
    observed: &Observations,
    answer: &Fit,
    flight: &Flight,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> Result<Fit, SyntheticError> {
    let filled = noise_free_observations(observed, &answer.read, flight, files)?;
    let mut told = stated_for(Some(&filled));
    told[Asked::MassScale.index()] = Prior {
        mean: 1.0,
        sd: 1e-6,
    };
    Ok(fit::fit(
        Some(&filled),
        &told,
        Noise::Given {
            height_m: answer.height_noise_m,
            horizontal_m: answer.horizontal_noise_m,
        },
        &mut Flown::new(&flight.draw, files, 1.0, tables),
    ))
}

/// A log's fit, `log` being the flight's own or one in place of it, against the flight's truth. A
/// flight of another model whose log said nothing has none: it isn't counted.
///
/// # Errors
///
/// [`SyntheticError::Flight`] if a flight of another model can't be fitted noise-free: a truth is
/// needed for the count.
pub fn fit_log(
    log: &FlightLog,
    flight: &Flight,
    set: Set,
    bayesian: bool,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> Result<FlightFit, SyntheticError> {
    let draw = &flight.draw;
    let weighed = weighed_factor(set, draw.index);
    let read = readings::read(log);
    let observed = fit::observations(log, &read);
    let told = stated_for(observed.as_ref());
    let (answer, posterior) = fitted_and_sampled(
        observed.as_ref(),
        &told,
        bayesian.then(|| sampling(set, draw.index)),
        &mut Flown::new(draw, files, weighed, tables),
    );
    let mut flown = answer.flown;
    let mut converged = answer.converged;
    let truth: [f64; FLOWN] = match draw.model {
        Model::Same => {
            // It blows from `wind_from_rad`, so toward the opposite way.
            let (speed, from) = (draw.wind_speed_m_s, draw.wind_from_rad);
            [
                draw.drag_scale,
                draw.impulse_scale,
                draw.burn_time_scale,
                -speed * from.sin(),
                -speed * from.cos(),
                f64::NAN,
            ]
        }
        Model::Other { .. } => match observed.as_ref().filter(|_| answer.fitted) {
            Some(observed) => {
                let free = noise_free(observed, &answer, flight, files, tables)?;
                if !free.fitted {
                    return Err(SyntheticError::Flight {
                        what: "no noise-free fit",
                    });
                }
                converged &= free.converged;
                flown += free.flown;
                free.parameters.to_array()
            }
            None => [f64::NAN; FLOWN],
        },
    };
    let values = answer.parameters.to_array();
    let compared = ASKED.map(|asked| {
        let i = asked.index();
        Compared {
            value: values[i],
            standard_error: answer.standard_errors[i],
            // The fit's mass is a factor on the weighed mass: the truth undoes the weighing.
            truth: if asked == Asked::MassScale {
                1.0 / weighed
            } else {
                truth[i]
            },
        }
    });
    Ok(FlightFit {
        index: draw.index,
        design: draw.design,
        fitted: answer.fitted,
        drift: answer.fitted && observed.is_some_and(|o| !o.horizontal.is_empty()),
        converged,
        misfit_share: answer.misfit_share,
        compared,
        rms_m: answer.rms_m,
        flown,
        posterior,
    })
}

/// A log's fit from its residuals' noise, and with `sampling` the Bayesian option's posterior.
fn fitted_and_sampled(
    observed: Option<&Observations>,
    told: &Stated,
    sampling: Option<Sampling>,
    forward: &mut dyn Forward,
) -> (Fit, Option<Posterior<FLOWN>>) {
    match sampling {
        Some(sampling) => {
            let answer = fit::bayesian(observed, told, Noise::FromResiduals, forward, sampling);
            (answer.least_squares, answer.posterior)
        }
        None => (
            fit::fit(observed, told, Noise::FromResiduals, forward),
            None,
        ),
    }
}

/// A log of pure noise in place of `log`: the same clock, each height a uniform draw of `size_m`
/// about nothing, and no other channel.
#[must_use]
pub fn noise_log(log: &FlightLog, size_m: f64, seed: u64, index: u64) -> FlightLog {
    let mut rng = SeededRng::for_stream(seed, &[index, super::NOISE_LOG_KEY]);
    let (time_s, _) = readings::height_track(log).unwrap_or_default();
    let altitude_m = time_s
        .iter()
        .map(|_| between(&mut rng, -size_m, size_m))
        .collect();
    FlightLog {
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        channels: Vec::new(),
        events: Vec::new(),
        ..log.clone()
    }
}

/// The shares a flight's parameter takes: those of what its log holds, and on a log that said
/// nothing, its stated range's own, the normal's, as the fit held it to.
fn shares_of(fit: &FlightFit, asked: Asked) -> Shares {
    if fit.fitted {
        shares(asked, fit.drift)
    } else {
        Shares {
            low: -Z_90,
            high: Z_90,
        }
    }
}

/// A flight's fit with bins of pure noise handed to it past the reader, as if the reader had found
/// a flight of 20 s in them: heights of `size_m` either way, and a track of `track_m` in each
/// direction where that's a size.
pub fn fit_noise_bins(
    flight: &Flight,
    set: Set,
    bayesian: bool,
    size_m: f64,
    track_m: Option<f64>,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> FlightFit {
    let index = flight.draw.index;
    let mut rng = SeededRng::for_stream(set.seed(), &[index, super::NOISE_LOG_KEY, 1]);
    let heights = (0..80)
        .map(|k| (0.25 * f64::from(k), between(&mut rng, -size_m, size_m)))
        .collect();
    let horizontal: Vec<(f64, f64, f64)> = track_m.map_or_else(Vec::new, |track_m| {
        (0..80)
            .map(|k| {
                (
                    0.25 * f64::from(k),
                    between(&mut rng, -track_m, track_m),
                    between(&mut rng, -track_m, track_m),
                )
            })
            .collect()
    });
    let drift = !horizontal.is_empty();
    let observed = Observations {
        kind: HeightKind::Barometric,
        heights,
        horizontal,
        liftoff_s: 0.0,
        flown_s: 20.0,
        reads_transonic: false,
    };
    let weighed = weighed_factor(set, index);
    let (answer, posterior) = fitted_and_sampled(
        Some(&observed),
        &stated_for(Some(&observed)),
        bayesian.then(|| sampling(set, index)),
        &mut Flown::new(&flight.draw, files, weighed, tables),
    );
    let values = answer.parameters.to_array();
    FlightFit {
        index,
        design: flight.draw.design,
        fitted: answer.fitted,
        drift: answer.fitted && drift,
        converged: answer.converged,
        misfit_share: answer.misfit_share,
        compared: ASKED.map(|asked| Compared {
            value: values[asked.index()],
            standard_error: answer.standard_errors[asked.index()],
            truth: f64::NAN,
        }),
        rms_m: answer.rms_m,
        flown: answer.flown,
        posterior,
    }
}

/// `asked`'s 90% interval on `fit` by `method`; `None` where it can't be stated. A log that said
/// nothing states the stated range's own by either: what was stated of it is all there is.
#[must_use]
pub fn interval_of(method: Method, fit: &FlightFit, asked: Asked) -> Option<Interval> {
    let slot = ASKED.iter().position(|a| *a == asked)?;
    let compared = fit.compared[slot];
    match method {
        Method::Bayesian if fit.fitted => {
            let [low, _, high] = fit.posterior.as_ref()?.quantiles[asked.index()];
            (low.is_finite() && high.is_finite() && low <= high).then_some(Interval { low, high })
        }
        _ => shares_of(fit, asked).interval(compared.value, compared.standard_error),
    }
}

/// `asked`'s point on `fit` by `method`: the least-squares answer, or the posterior's median.
fn point_of(method: Method, fit: &FlightFit, asked: Asked, slot: usize) -> f64 {
    match (method, &fit.posterior) {
        (Method::Bayesian, Some(posterior)) if fit.fitted => posterior.quantiles[asked.index()][1],
        (Method::Bayesian, None) if fit.fitted => f64::NAN,
        _ => fit.compared[slot].value,
    }
}

/// Whether a parameter is identified on a set: [ADR-257][adr-257] §1's rule.
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Identification {
    /// The median interval's width over the flights.
    pub median_width: f64,
    /// The 90% interval's width of its stated range.
    pub stated_width: f64,
    /// Whether the median is at most half the stated.
    pub identified: bool,
}

impl Identification {
    /// `asked`'s, over `fits`.
    #[must_use]
    pub fn of(asked: Asked, fits: &[FlightFit]) -> Self {
        Self::by(Method::LeastSquares, asked, fits)
    }

    /// `asked`'s, over `fits`, by `method`. An interval that can't be stated counts as wider than
    /// any.
    #[must_use]
    pub fn by(method: Method, asked: Asked, fits: &[FlightFit]) -> Self {
        let slot = ASKED.iter().position(|a| *a == asked).unwrap_or(0);
        let mut widths: Vec<f64> = fits
            .iter()
            .map(|f| match method {
                Method::LeastSquares => {
                    let shares = shares_of(f, asked);
                    (shares.high - shares.low) * f.compared[slot].standard_error
                }
                Method::Bayesian => interval_of(method, f, asked)
                    .map_or(f64::INFINITY, |interval| interval.high - interval.low),
            })
            .collect();
        widths.sort_by(f64::total_cmp);
        let median_width = widths.get(widths.len() / 2).copied().unwrap_or(f64::NAN);
        let stated_width = asked.stated_width();
        Self {
            median_width,
            stated_width,
            identified: median_width <= 0.5 * stated_width,
        }
    }
}

/// The residual that splits [`Check::by_residual`], m: past it a flight's errors run larger than
/// its standard errors say, and the shares, one pair for both sides, cover it less.
pub const RESIDUAL_SPLIT_M: f64 = 1.0;

/// One parameter's check on one set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Check {
    /// The parameter.
    pub asked: Asked,
    /// The test on the intervals as stated.
    pub test: Test,
    /// The test on each probe's intervals; each must fail.
    pub probes: BTreeMap<Probe, Test>,
    /// The counts by design.
    pub by_design: BTreeMap<String, Counts>,
    /// The counts of flights whose log holds a horizontal track, and of those that don't.
    pub by_drift: BTreeMap<String, Counts>,
    /// The counts by how well the flight's heights fit, [`RESIDUAL_SPLIT_M`]: the shares are one
    /// pair for both, so each side's coverage shows here.
    pub by_residual: BTreeMap<String, Counts>,
    /// The mean error, fit less truth, with its sign.
    pub mean_error: f64,
    /// What the log can tell of it.
    pub identification: Identification,
}

/// `asked`'s check over `fits`, of the least-squares fit's intervals.
#[must_use]
pub fn check(asked: Asked, fits: &[FlightFit]) -> Check {
    check_by(Method::LeastSquares, asked, fits)
}

/// `asked`'s check over `fits`, of `method`'s intervals.
#[must_use]
pub fn check_by(method: Method, asked: Asked, fits: &[FlightFit]) -> Check {
    let slot = ASKED.iter().position(|a| *a == asked).unwrap_or(0);
    let mut counts = Counts::default();
    let mut probes: BTreeMap<Probe, Counts> =
        Probe::ALL.iter().map(|&p| (p, Counts::default())).collect();
    let mut by_design: BTreeMap<String, Counts> = BTreeMap::new();
    let mut by_drift: BTreeMap<String, Counts> = BTreeMap::new();
    let mut by_residual: BTreeMap<String, Counts> = BTreeMap::new();
    let mut error_sum = 0.0;
    let mut stated_count = 0_u32;
    for fit in fits {
        // A log that said nothing states no interval of its own, and its range would hold 90%
        // whatever the fit did. A wind is counted only where a track speaks to it.
        if !fit.fitted || (!fit.drift && !asked.stated_on_height_alone()) {
            continue;
        }
        stated_count += 1;
        let compared = fit.compared[slot];
        let Some(interval) = interval_of(method, fit, asked) else {
            // An interval that can't be stated misses on both sides.
            let nowhere = Interval {
                low: f64::NAN,
                high: f64::NAN,
            };
            counts.add(nowhere, f64::NAN);
            continue;
        };
        counts.add(interval, compared.truth);
        for (probe, counted) in &mut probes {
            counted.add(probe.apply(interval), compared.truth);
        }
        by_design
            .entry(
                DESIGNS
                    .get(fit.design)
                    .map_or("an unknown design", |design| design.name)
                    .to_owned(),
            )
            .or_default()
            .add(interval, compared.truth);
        by_drift
            .entry(
                if fit.drift {
                    "with a horizontal track"
                } else {
                    "height alone"
                }
                .to_owned(),
            )
            .or_default()
            .add(interval, compared.truth);
        if let Some(rms_m) = fit.rms_m {
            by_residual
                .entry(
                    if rms_m < RESIDUAL_SPLIT_M {
                        "heights fit within 1 m"
                    } else {
                        "heights fit to 1 m or worse"
                    }
                    .to_owned(),
                )
                .or_default()
                .add(interval, compared.truth);
        }
        error_sum += point_of(method, fit, asked, slot) - compared.truth;
    }
    Check {
        asked,
        test: Test::of(counts),
        probes: probes.into_iter().map(|(p, c)| (p, Test::of(c))).collect(),
        by_design,
        by_drift,
        by_residual,
        mean_error: error_sum / f64::from(stated_count.max(1)),
        identification: Identification::by(method, asked, fits),
    }
}

/// The first `count` flights of `set` whose logger holds a horizontal track and a barometer, whose
/// fits are what the wind is counted on. A set's draw at any index is fixed, so these run on past
/// its count without moving its flights.
#[must_use]
pub fn track_flights(set: Set, count: u64) -> Vec<u64> {
    (0..)
        .filter(|&index| super::draw(set, index).logger.profile == Profile::Altos)
        .take(usize::try_from(count).unwrap_or(usize::MAX))
        .collect()
}

/// The shares of `asked`, fitted on the fits of `fits` whose logs hold a horizontal track or don't,
/// by `drift`: its share's 5% and 95% points; `None` for fewer than 20, or one that isn't a number.
#[must_use]
pub fn fitted(asked: Asked, drift: bool, fits: &[FlightFit]) -> Option<Shares> {
    let slot = ASKED.iter().position(|a| *a == asked)?;
    let shares: Vec<f64> = fits
        .iter()
        .filter(|f| f.fitted && f.drift == drift)
        .map(|f| f.compared[slot].share())
        .collect();
    hpr_flightdata::readings::interval::fit(&shares)
}

/// The shares in force on a log that holds a horizontal track, or doesn't, by `drift`, fitted on
/// [`Set::Calibration`]: `cargo xtask fit --calibrate` prints them. A
/// track speaks to the wind, so its errors are small and of another kind: the two are told apart,
/// as the log itself says which it is.
#[must_use]
pub fn shares(asked: Asked, drift: bool) -> Shares {
    let (low, high) = match (asked, drift) {
        (Asked::DragScale, false) => (-1.27, 1.43),
        (Asked::ImpulseScale, false) => (-1.53, 1.47),
        (Asked::WindEast, false) => (-0.369, 3.25),
        (Asked::WindNorth, false) => (-1.73, 1.74),
        (Asked::MassScale, false) => (-1.66, 1.67),
        (Asked::DragScale, true) => (-1.28, 2.15),
        (Asked::ImpulseScale, true) => (-1.5, 1.83),
        (Asked::WindEast, true) => (-1.36, 1.29),
        (Asked::WindNorth, true) => (-1.5, 1.38),
        (Asked::MassScale, true) => (-1.69, 1.7),
    };
    Shares { low, high }
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
    fn the_wind_components_stated_width_is_what_the_draw_gives() {
        // The draw's own component's 5% and 95% points, on many uniform draws.
        let mut rng = SeededRng::for_stream(7, &[1]);
        let mut values: Vec<f64> = (0..200_000)
            .map(|_| {
                let v = between(&mut rng, 0.0, super::super::MAX_WIND_M_S);
                let theta = between(&mut rng, 0.0, std::f64::consts::TAU);
                v * theta.sin()
            })
            .collect();
        values.sort_by(f64::total_cmp);
        let point = wind_component_point();
        assert!(
            (values[190_000] - point).abs() < 0.05,
            "{} against {point}",
            values[190_000]
        );
        assert!((values[10_000] + point).abs() < 0.05);
    }

    #[test]
    fn with_no_log_each_interval_is_its_stated_range() {
        for asked in ASKED {
            let told = stated()[asked.index()];
            let width = 2.0 * Z_90 * told.sd;
            assert!((width - asked.stated_width()).abs() < 1e-12, "{asked:?}");
        }
    }

    #[test]
    fn each_method_states_its_own_interval() {
        let compared = Compared {
            value: 1.0,
            standard_error: 0.1,
            truth: 1.05,
        };
        let posterior = Posterior {
            // Each unknown's row its own: 0.9 − i/10 to 1.3 + i/10.
            quantiles: std::array::from_fn(|i| {
                let i = f64::from(u8::try_from(i).unwrap());
                [0.9 - 0.1 * i, 1.0, 1.3 + 0.1 * i]
            }),
            effective_draws: 150.0,
            draws: 500,
            failed: 0,
            enough: true,
        };
        let fit = FlightFit {
            index: 0,
            design: 0,
            fitted: true,
            drift: false,
            converged: true,
            misfit_share: Some(0.01),
            compared: [compared; ASKED.len()],
            rms_m: Some(0.5),
            flown: 70,
            posterior: Some(posterior),
        };
        // The posterior's 5% and 95% points as they come, for the parameter's own row.
        let bayes = interval_of(Method::Bayesian, &fit, Asked::DragScale).unwrap();
        assert_eq!((bayes.low, bayes.high), (0.9, 1.3));
        let mass = interval_of(Method::Bayesian, &fit, Asked::MassScale).unwrap();
        assert!((mass.low - 0.4).abs() < 1e-12 && (mass.high - 1.8).abs() < 1e-12);
        // The least squares' from its shares.
        let shares = shares(Asked::DragScale, false);
        let least = interval_of(Method::LeastSquares, &fit, Asked::DragScale).unwrap();
        assert!((least.low - (1.0 - shares.high * 0.1)).abs() < 1e-12);
        // A fitted log whose posterior had no weight states none; a log that said nothing states
        // its stated range by either.
        let without = FlightFit {
            posterior: None,
            ..fit.clone()
        };
        assert!(interval_of(Method::Bayesian, &without, Asked::DragScale).is_none());
        let nothing = FlightFit {
            fitted: false,
            posterior: None,
            ..fit
        };
        let both = Method::ALL.map(|m| interval_of(m, &nothing, Asked::DragScale).unwrap());
        assert_eq!(both[0], both[1]);
        assert!((both[0].high - (1.0 + Z_90 * 0.1)).abs() < 1e-12);
    }

    #[test]
    fn the_weighing_factor_stays_in_its_range_and_fills_it() {
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
        for k in 0..2_000 {
            let f = weighed_factor(Set::A, k);
            low = low.min(f);
            high = high.max(f);
        }
        assert!((0.98..0.981).contains(&low), "{low}");
        assert!(high <= 1.02 && high > 1.019, "{high}");
    }
}
