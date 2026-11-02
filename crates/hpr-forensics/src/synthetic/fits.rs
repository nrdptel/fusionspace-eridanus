//! The least-squares fit's 90% intervals held to the synthetic flights ([M7.3b][m7-3],
//! [ADR-257][adr-257] §1, [ADR-316][adr-316]).
//!
//! Each flight's log is fitted ([`crate::fit`]), told what a flyer knows: the design, its motor at
//! its rated impulse and burn, the rail, the site and the day's air, and the structure's mass as
//! weighed, [`weighed_factor`] off the truth. The drag scale, impulse scale, wind and mass are each
//! compared with the flight's truth by the same three counts as the readings'
//! ([`intervals::Test`]), with the same three probes.
//!
//! **The truth.** On [`Set::A`](super::Set::A) a parameter's truth is what the flight drew. On
//! [`Set::B`](super::Set::B), whose drag curve, wind and thrust are shaped otherwise, it is the fit
//! to the noise-free flight ([`noise_free`]): the same bins of height and drift, as the sensor would
//! have read them with no noise, on a clock and pad known exactly. The mass's truth is what the
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
use super::truth::{self, Truth};
use super::{Draw, Flight, Model, Set, SyntheticError};
use crate::fit::{self, FLOWN, Fit, Forward, Noise, Observations, Parameters, Prior, Stated};
use crate::residuals::{HeightKind, Simulated, Track};

/// The parameters asked of the fit, in the report's order.
pub const ASKED: [Asked; 5] = [
    Asked::DragScale,
    Asked::ImpulseScale,
    Asked::WindEast,
    Asked::WindNorth,
    Asked::MassScale,
];

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
    /// can't be right on both sets, as set B's shear widens its truth (ADR-316).
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
    /// Whether its log holds a horizontal track.
    pub drift: bool,
    /// Each asked parameter, in [`ASKED`]'s order.
    pub compared: [Compared; ASKED.len()],
    /// The heights' residual's RMS, m.
    pub rms_m: Option<f64>,
    /// Flights flown for it, the noise-free fit's included.
    pub flown: u64,
}

/// What a flight's height and drift were, as a fit reads them: known whole, on the flight's own
/// clock, at each bin to its apogee.
#[must_use]
pub fn truth_observations(truth: &Truth) -> Observations {
    let apogee_s = truth.events.apogee_s;
    let start = truth.samples.first().map_or([0.0; 3], |s| s.position_enu_m);
    let bins = (apogee_s / fit::BIN_S).floor();
    let mut heights = Vec::new();
    let mut horizontal = Vec::new();
    let mut k = 0.0;
    while k < bins {
        let t = k * fit::BIN_S;
        if let Some(sample) = truth.at(t) {
            heights.push((t, sample.height_m - truth.pad_height_m));
            horizontal.push((
                t,
                sample.position_enu_m[0] - start[0],
                sample.position_enu_m[1] - start[1],
            ));
        }
        k += 1.0;
    }
    Observations {
        kind: HeightKind::Geometric,
        heights,
        horizontal,
        liftoff_s: truth.events.liftoff_s,
        flown_s: apogee_s - truth.events.liftoff_s,
        // The same part of the flight as a barometer's fit reads, so the two weigh its drag at the
        // same Mach numbers.
        reads_transonic: false,
    }
}

/// The bins of `observed` as the noise-free flight fills them, on the flight's own clock: what the
/// log's sensors would have read with no noise, no gain, no drift and no port, in the same bins, so
/// that the fit to them differs from the log's by the noise alone.
///
/// # Errors
///
/// [`SyntheticError`] if the flight's air doesn't reach it.
pub fn noise_free_observations(
    observed: &Observations,
    flight: &Flight,
    files: &DesignFiles,
) -> Result<Observations, SyntheticError> {
    let environment = truth::inputs(&flight.draw, files)?.environment;
    let flown = Simulated::new(flight.truth.samples.clone(), &environment).map_err(|_| {
        SyntheticError::Flight {
            what: "a truth the day's air doesn't reach",
        }
    })?;
    let to_flight = |t: f64| t - flight.draw.logger.pad_s;
    Ok(Observations {
        kind: observed.kind,
        heights: observed
            .heights
            .iter()
            .map(|&(t, _)| {
                let at = to_flight(t);
                (at, flown.read_height_m(at, observed.kind))
            })
            .collect(),
        horizontal: observed
            .horizontal
            .iter()
            .map(|&(t, _, _)| {
                let at = to_flight(t);
                let (east, north) = flown.drift_m(at);
                (at, east, north)
            })
            .collect(),
        liftoff_s: flight.truth.events.liftoff_s,
        flown_s: observed.flown_s,
        reads_transonic: observed.reads_transonic,
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
    observed: Option<&Observations>,
    answer: &Fit,
    flight: &Flight,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> Result<Fit, SyntheticError> {
    let mut told = stated();
    told[Asked::MassScale.index()] = Prior {
        mean: 1.0,
        sd: 1e-6,
    };
    let (filled, noise) = match observed {
        Some(observed) => (
            noise_free_observations(observed, flight, files)?,
            Noise::Given {
                height_m: answer.height_noise_m,
                horizontal_m: answer.horizontal_noise_m,
            },
        ),
        None => (
            truth_observations(&flight.truth),
            Noise::Known {
                height_m: 1.0,
                horizontal_m: 1.0,
            },
        ),
    };
    Ok(fit::fit(
        Some(&filled),
        &told,
        noise,
        &mut Flown::new(&flight.draw, files, 1.0, tables),
    ))
}

/// A flight's log's fit, against its truth.
///
/// # Errors
///
/// [`SyntheticError::Flight`] if a flight of another model can't be fitted noise-free: a truth is
/// needed for the count.
pub fn fit_flight(
    flight: &Flight,
    set: Set,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> Result<FlightFit, SyntheticError> {
    fit_log(&flight.log, flight, set, files, tables)
}

/// As [`fit_flight`], with `log` in place of the flight's own.
///
/// # Errors
///
/// As [`fit_flight`].
pub fn fit_log(
    log: &FlightLog,
    flight: &Flight,
    set: Set,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> Result<FlightFit, SyntheticError> {
    let draw = &flight.draw;
    let weighed = weighed_factor(set, draw.index);
    let read = readings::read(log);
    let observed = fit::observations(log, &read);
    let told = stated();
    let answer = fit::fit(
        observed.as_ref(),
        &told,
        Noise::FromResiduals,
        &mut Flown::new(draw, files, weighed, tables),
    );
    let mut flown = answer.flown;
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
        Model::Other { .. } => {
            let free = noise_free(observed.as_ref(), &answer, flight, files, tables)?;
            if !free.fitted {
                return Err(SyntheticError::Flight {
                    what: "no noise-free fit",
                });
            }
            flown += free.flown;
            free.parameters.to_array()
        }
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
        drift: observed.is_some_and(|o| !o.horizontal.is_empty()),
        compared,
        rms_m: answer.rms_m,
        flown,
    })
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

/// A flight's fit with bins of pure noise of `size_m` handed to it past the reader, as if the
/// reader had found a flight of 20 s in them.
pub fn fit_noise_bins(
    flight: &Flight,
    set: Set,
    size_m: f64,
    files: &DesignFiles,
    tables: &mut BTreeMap<usize, Simulation>,
) -> FlightFit {
    let index = flight.draw.index;
    let mut rng = SeededRng::for_stream(set.seed(), &[index, super::NOISE_LOG_KEY, 1]);
    let heights = (0..80)
        .map(|k| (0.25 * f64::from(k), between(&mut rng, -size_m, size_m)))
        .collect();
    let observed = Observations {
        kind: HeightKind::Barometric,
        heights,
        horizontal: Vec::new(),
        liftoff_s: 0.0,
        flown_s: 20.0,
        reads_transonic: false,
    };
    let weighed = weighed_factor(set, index);
    let answer = fit::fit(
        Some(&observed),
        &stated(),
        Noise::FromResiduals,
        &mut Flown::new(&flight.draw, files, weighed, tables),
    );
    let values = answer.parameters.to_array();
    FlightFit {
        index,
        design: flight.draw.design,
        fitted: answer.fitted,
        drift: false,
        compared: ASKED.map(|asked| Compared {
            value: values[asked.index()],
            standard_error: answer.standard_errors[asked.index()],
            truth: f64::NAN,
        }),
        rms_m: answer.rms_m,
        flown: answer.flown,
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
        let slot = ASKED.iter().position(|a| *a == asked).unwrap_or(0);
        let mut widths: Vec<f64> = fits
            .iter()
            .map(|f| {
                let shares = shares_of(f, asked);
                (shares.high - shares.low) * f.compared[slot].standard_error
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

impl Check {
    /// Whether it comes out as it must: the test passes and every probe fails it.
    #[must_use]
    pub fn holds(&self) -> bool {
        self.test.passes() && self.probes.values().all(|t| !t.passes())
    }
}

/// `asked`'s check over `fits`.
#[must_use]
pub fn check(asked: Asked, fits: &[FlightFit]) -> Check {
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
        if !fit.drift && !asked.stated_on_height_alone() {
            continue;
        }
        stated_count += 1;
        let compared = fit.compared[slot];
        let shares = shares_of(fit, asked);
        let Some(interval) = shares.interval(compared.value, compared.standard_error) else {
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
            .entry(DESIGNS[fit.design].name.to_owned())
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
        error_sum += compared.value - compared.truth;
    }
    Check {
        asked,
        test: Test::of(counts),
        probes: probes.into_iter().map(|(p, c)| (p, Test::of(c))).collect(),
        by_design,
        by_drift,
        by_residual,
        mean_error: error_sum / f64::from(stated_count.max(1)),
        identification: Identification::of(asked, fits),
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
/// [`Set::Calibration`](super::Set::Calibration): `cargo xtask fit --calibrate` prints them. A
/// track speaks to the wind, so its errors are small and of another kind: the two are told apart,
/// as the log itself says which it is.
#[must_use]
pub fn shares(asked: Asked, drift: bool) -> Shares {
    let (low, high) = match (asked, drift) {
        (Asked::DragScale, false) => (-1.82, 1.83),
        (Asked::ImpulseScale, false) => (-1.92, 1.47),
        (Asked::WindEast, false) => (-1.36, 1.43),
        (Asked::WindNorth, false) => (-1.41, 1.33),
        (Asked::MassScale, false) => (-1.64, 1.72),
        (Asked::DragScale, true) => (-1.59, 2.59),
        (Asked::ImpulseScale, true) => (-1.64, 1.83),
        (Asked::WindEast, true) => (-1.94, 1.81),
        (Asked::WindNorth, true) => (-2.08, 1.79),
        (Asked::MassScale, true) => (-1.68, 1.72),
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
