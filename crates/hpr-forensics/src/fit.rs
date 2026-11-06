//! Fitting a simulation to a flight's log by least squares ([M7.3b][m7-3], [ADR-257][adr-257] §1,
//! [ADR-316][adr-316]).
//!
//! **What it is for.** A flyer who has a log and a design wants to know what the flight says about
//! the design: was the drag what the simulator took, did the motor deliver its rated impulse, what
//! wind did the rocket feel? This module changes those inputs until the simulated flight, read as
//! the log's sensor reads it, passes through the log's samples, and says how far each could still
//! be off.
//!
//! **The method.** Levenberg–Marquardt (K. Levenberg, *Q. Appl. Math.* 2, 1944; D. W. Marquardt,
//! *J. SIAM* 11, 1963; the damped normal equations as in Press et al., *Numerical Recipes*, §15.5)
//! minimizes
//!
//! ```text
//! χ²(x) = Σ (yᵢ − g h(tᵢ − τ; x) − h₀ − d tᵢ − c p(tᵢ − τ; x))² / σ²
//!       + Σ (rₖ − r(tₖ − τ; x))² / σ_r² + Σ (xⱼ − μⱼ)² / sⱼ²
//! ```
//!
//! over `x`: the drag scale, impulse scale, burn time's scale, a constant wind vector and the mass
//! on the weighed mass, with what the log can't state as further unknowns: the clock offset `τ`,
//! the height offset `h₀`, the height's drift `d`, and for a barometer its gain `g` and port
//! coefficient `c` on the port's dynamic-pressure height `p` (each sized as [`crate::residuals`]
//! sizes it). Left out, these went into the drag scale. The second sum is the track `r`, where a log holds one. The
//! third holds each parameter to what is stated of it ([`Stated`]). That sum is what "mass within
//! its weighed uncertainty" means, and it is why a log with no word on the wind gives back the
//! wind's stated range. `σ` is the fit's own residual, floored at [`FLOOR_M`], and `σ_r` likewise
//! at [`FLOOR_HORIZONTAL_M`], re-estimated once after a first pass. A barometer's bins at or past
//! Mach 0.8 are left out, as its port's pressure there follows no one coefficient.
//!
//! Each parameter's standard error is the square root of the diagonal of `σ² (JᵀJ)⁻¹`, `J` the
//! Jacobian of the weighted residuals at the solution, and its 90% interval is that error times the
//! shares fitted on other flights ([`crate::synthetic::fits`]). The samples are taken as a median
//! every [`BIN_S`], since the barometer's noise is correlated from one sample to the next.
//!
//! **What it reads.** The height track the readings take, from a second before the reader's liftoff
//! to its apogee, on the reader's pad; before an ejection charge's pressure spike. Where the log
//! holds latitude and longitude, it reads the rocket's drift too, which is what speaks to the wind.
//!
//! **The Bayesian option.** [`bayesian`] fits as [`fit`] does, then draws the posterior of the same
//! unknowns about its answer, each draw a flight flown ([ADR-317][adr-317], [ADR-320][adr-320]):
//! the wind first, then the rest about where a Gauss–Newton step puts them in that wind, the
//! sensor's linear unknowns integrated out in closed form, and on a log of height alone the wind's
//! shear exponent drawn too. Its 90% intervals are the posterior's 5% and 95% points, with no
//! shares.
//!
//! **How far to trust it.** The errors are linearized about the solution, so they say nothing of a
//! second solution elsewhere. The 90% intervals hold on synthetic flights ([`crate::synthetic::fits`]
//! and the committed report), where the fit knows the design, rail, site and day's air: on a real
//! flight its model's errors go into the scales with nothing to say so. A scale is the factor that
//! best fits one flight, not the rocket's drag at every Mach.
//!
//! [m7-3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
//! [adr-317]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0317-m7-3c-in-two-the-sampler-then-the-test.md
//! [adr-320]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0320-m7-3c2-the-bayesian-option-in-two-blocks.md

use std::cell::{Cell, RefCell};

use hpr_flightdata::log::{FlightLog, Quantity};
use hpr_flightdata::readings::interval::Z_90;
use hpr_flightdata::readings::{self, Reading, Readings};
use serde::{Deserialize, Serialize};

use crate::posterior::{self, Posterior};
use crate::residuals::{HeightKind, Simulated};

/// The width of a bin of samples, s.
pub const BIN_S: f64 = 0.25;
/// The least noise a height's residual is taken to have, m: [`readings::NOISE_M`], what the
/// barometer's noise adds to a reading.
pub const FLOOR_M: f64 = readings::NOISE_M;
/// The least noise a horizontal position's residual is taken to have, m: a receiver's lag in the
/// boost, which the bins' median doesn't remove (`crate::synthetic::logger`'s loop).
pub const FLOOR_HORIZONTAL_M: f64 = 10.0;
/// The most iterations of a pass of the fit.
const MAX_ITERATIONS: usize = 60;
/// A step that lowers χ² by less than this ends a pass.
const CONVERGED_CHI2: f64 = 1e-3;
/// How far the first clock offset is searched either side of the reader's liftoff, s.
const OFFSET_SEARCH_S: f64 = 3.0;
/// The search's step, s.
const OFFSET_STEP_S: f64 = 0.05;
/// The mean radius used to turn small angles into meters, m (IUGG R₁).
const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// How many parameters a fit flies.
pub const FLOWN: usize = 6;
/// Its unknowns: the flown parameters, the clock offset, the height offset, and the sensor's gain,
/// drift and port.
const UNKNOWNS: usize = FLOWN + 5;
/// Where the wind's components stand among them, in [`Parameters::to_array`]'s order.
const WIND_EAST: usize = 3;
const WIND_NORTH: usize = 4;

/// What a fit may change in a flight.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    /// The factor on the zero-lift drag.
    pub drag_scale: f64,
    /// The factor on the motor's total impulse.
    pub impulse_scale: f64,
    /// The factor on its burn time. Not asked of the fit: it is here so a burn's length doesn't
    /// lean on the two scales.
    pub burn_time_scale: f64,
    /// The constant wind's speed toward the east, m/s.
    pub wind_east_m_s: f64,
    /// Its speed toward the north, m/s.
    pub wind_north_m_s: f64,
    /// The factor on the weighed mass of the structure, 1 if it weighs what was stated.
    pub mass_scale: f64,
}

impl Parameters {
    /// As an array, in [`Fit`]'s order.
    #[must_use]
    pub fn to_array(self) -> [f64; FLOWN] {
        [
            self.drag_scale,
            self.impulse_scale,
            self.burn_time_scale,
            self.wind_east_m_s,
            self.wind_north_m_s,
            self.mass_scale,
        ]
    }

    /// From an array in [`Fit`]'s order.
    #[must_use]
    pub fn from_array(x: [f64; FLOWN]) -> Self {
        Self {
            drag_scale: x[0],
            impulse_scale: x[1],
            burn_time_scale: x[2],
            wind_east_m_s: x[3],
            wind_north_m_s: x[4],
            mass_scale: x[5],
        }
    }
}

/// What is stated of a parameter before the log: normal, `mean ± sd`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Prior {
    /// Its middle.
    pub mean: f64,
    /// Its standard deviation.
    pub sd: f64,
}

/// What is stated of every parameter.
pub type Stated = [Prior; FLOWN];

/// How a fit takes its residuals' noise.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Noise {
    /// From the fit's residuals, floored at [`FLOOR_M`] and [`FLOOR_HORIZONTAL_M`].
    FromResiduals,
    /// As given, every sensor's unknown still open: the same fit as a log's, to a flight read with
    /// no sensor's noise or errors, so its port's coefficient is stated about nothing.
    Given {
        /// A height's, m.
        height_m: f64,
        /// A horizontal position's, m.
        horizontal_m: f64,
    },
    /// As given, on a clock and a pad known exactly: a flight whose truth is the data, with no
    /// sensor to estimate either from.
    Known {
        /// A height's, m.
        height_m: f64,
        /// A horizontal position's, m.
        horizontal_m: f64,
    },
}

/// What a fit reads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observations {
    /// What the heights stand for.
    pub kind: HeightKind,
    /// Each bin's time on the log's clock, s, and height above the pad, m.
    pub heights: Vec<(f64, f64)>,
    /// Each bin's time on the log's clock, s, and meters east and north of the pad.
    pub horizontal: Vec<(f64, f64, f64)>,
    /// Where the log's clock has the flight's liftoff, s, near enough to start from.
    pub liftoff_s: f64,
    /// How long the flight flew from liftoff to its last height, s.
    pub flown_s: f64,
    /// Whether heights at or past [`readings::TRANSONIC_MACH`] are read. A barometer's aren't: its
    /// port reads the flow over the nose there.
    pub reads_transonic: bool,
}

/// The median of `values`, emptied of what isn't a number; `None` if nothing is left.
fn median(values: &mut Vec<f64>) -> Option<f64> {
    values.retain(|v| v.is_finite());
    values.sort_by(f64::total_cmp);
    let n = values.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(values[n / 2]),
        _ => Some(0.5 * (values[n / 2 - 1] + values[n / 2])),
    }
}

/// The longest climb a log may state, s, from a second before liftoff to apogee: past any hobby
/// rocket's by far, so a clock that jumps can't ask for bins beyond any memory.
const MAX_WINDOW_S: f64 = 1_000.0;

/// How far a fit's residual may stand from its log, a share of the spread of the heights it read:
/// past it, the simulation explains too little of them for this to be the log of this flight, and
/// the log says nothing, whatever the errors come to. On a log of noise, nothing is explained and
/// the share stands near one; every synthetic flight fits far under it (the report's margin,
/// [ADR-316][adr-316]).
///
/// [adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
pub const MISFIT_SHARE: f64 = 0.1;

/// `samples`, in any order, as a median in each bin of [`BIN_S`] from `from_s` to `to_s`.
fn binned(samples: impl Iterator<Item = (f64, f64)>, from_s: f64, to_s: f64) -> Vec<(f64, f64)> {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a window of a flight's seconds in bins of a quarter second, checked to be positive"
    )]
    let bins = ((to_s - from_s) / BIN_S).ceil().max(0.0) as usize;
    let mut held: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for (t, h) in samples {
        if t.is_finite() && t >= from_s && t < to_s {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "inside the window, so positive and below the bins' count"
            )]
            let bin = ((t - from_s) / BIN_S) as usize;
            if let Some(values) = held.get_mut(bin) {
                values.push(h);
            }
        }
    }
    held.iter_mut()
        .enumerate()
        .filter_map(|(bin, values)| {
            #[expect(
                clippy::cast_precision_loss,
                reason = "a bin's number is far below 2^53"
            )]
            let t = from_s + (bin as f64 + 0.5) * BIN_S;
            median(values).map(|h| (t, h))
        })
        .collect()
}

/// A log's height and any horizontal track, on `read`'s pad, from `read`'s liftoff to its apogee;
/// `None` where the reader withheld the apogee, the pad, or too few bins hold a sample to fit, so
/// the log can say nothing.
#[must_use]
pub fn observations(log: &FlightLog, read: &Readings) -> Option<Observations> {
    let Reading::Read(apogee) = &read.apogee else {
        return None;
    };
    observations_until(
        log,
        read,
        apogee.time_s,
        HeightKind::of(apogee.source),
        false,
    )
}

/// A log's height and any horizontal track, on `read`'s pad, from a second before `read`'s liftoff up to `to_s`
/// (not including it), its heights of `kind`; `None` where the reader withheld the pad, the window
/// is empty or longer than [`MAX_WINDOW_S`], or too few bins hold a sample to fit. With
/// `whole_bins` the window ends at the last edge of a whole [`BIN_S`] bin at or before `to_s`: a
/// bin is read at its middle, and a part bin's samples, all early in it, would be read late, as
/// they are on a climb cut short (at apogee the height barely moves, so the whole log needs none).
pub(crate) fn observations_until(
    log: &FlightLog,
    read: &Readings,
    to_s: f64,
    kind: HeightKind,
    whole_bins: bool,
) -> Option<Observations> {
    let pad_m = read.pad_altitude_m?;
    let (times, heights) = readings::height_track(log)?;
    let apogee_s = to_s;
    let liftoff_s = match &read.liftoff {
        Reading::Read(liftoff) => liftoff.time_s,
        // No liftoff to read, as a log from launch has: its first sample stands for it.
        Reading::Withheld(_) => times.iter().copied().find(|t| t.is_finite())?,
    };
    if apogee_s <= liftoff_s || apogee_s - liftoff_s > MAX_WINDOW_S {
        return None;
    }
    let from_s = liftoff_s - 1.0;
    let apogee_s = if whole_bins {
        from_s + ((apogee_s - from_s) / BIN_S).floor() * BIN_S
    } else {
        apogee_s
    };
    if apogee_s <= liftoff_s {
        return None;
    }
    let track = times.iter().copied().zip(heights.iter().map(|h| h - pad_m));
    let bins = binned(track, from_s, apogee_s);
    if bins.len() <= UNKNOWNS {
        return None;
    }
    Some(Observations {
        kind,
        reads_transonic: kind == HeightKind::Geometric,
        heights: bins,
        horizontal: horizontal(log, from_s, apogee_s, liftoff_s),
        liftoff_s,
        flown_s: apogee_s - liftoff_s,
    })
}

/// The log's latitude and longitude as meters from the pad, its median before `liftoff_s` or
/// else its first fix, binned; empty where it holds no track.
fn horizontal(log: &FlightLog, from_s: f64, to_s: f64, liftoff_s: f64) -> Vec<(f64, f64, f64)> {
    // A ground station's own position isn't the rocket's.
    let rockets = |quantity| {
        log.channels_of(quantity)
            .find(|channel| !channel.column.starts_with("gs_"))
    };
    let (Some(latitude), Some(longitude)) =
        (rockets(Quantity::Latitude), rockets(Quantity::Longitude))
    else {
        return Vec::new();
    };
    // A fix is a latitude and longitude of one time.
    if !latitude
        .time_s
        .iter()
        .map(|t| t.to_bits())
        .eq(longitude.time_s.iter().map(|t| t.to_bits()))
    {
        return Vec::new();
    }
    let before = |values: &[f64]| {
        let mut pad: Vec<f64> = latitude
            .time_s
            .iter()
            .zip(values)
            .filter(|(t, _)| **t < liftoff_s)
            .map(|(_, v)| *v)
            .collect();
        median(&mut pad).or_else(|| values.iter().copied().find(|v| v.is_finite()))
    };
    let (Some(pad_latitude), Some(pad_longitude)) =
        (before(&latitude.values), before(&longitude.values))
    else {
        return Vec::new();
    };
    let cos_latitude = pad_latitude.to_radians().cos();
    let times = || latitude.time_s.iter().copied();
    let north = binned(
        times().zip(
            latitude
                .values
                .iter()
                .map(|v| (v - pad_latitude).to_radians() * EARTH_RADIUS_M),
        ),
        from_s,
        to_s,
    );
    let east = binned(
        times().zip(
            longitude
                .values
                .iter()
                .map(|v| (v - pad_longitude).to_radians() * EARTH_RADIUS_M * cos_latitude),
        ),
        from_s,
        to_s,
    );
    east.iter()
        .filter_map(|&(t, e)| {
            // Both are in time order, in bins of one grid.
            north
                .binary_search_by(|(u, _)| u.total_cmp(&t))
                .ok()
                .map(|at| (t, e, north[at].1))
        })
        .collect()
}

/// Flies a flight for a fit.
pub trait Forward {
    /// The flight on `parameters`, flown for `until_s` seconds at least.
    ///
    /// # Errors
    ///
    /// Why it can't fly: the fit steps back from a step that can't.
    fn fly(&mut self, parameters: &Parameters, until_s: f64) -> Result<Simulated, String>;

    /// As [`Forward::fly`], with the wind's speed taken where its stated wind is measured and
    /// growing with height above it as the power `exponent`: `V(z) = V_ref (z / z_ref)^α`.
    ///
    /// # Errors
    ///
    /// As [`Forward::fly`].
    fn fly_sheared(
        &mut self,
        parameters: &Parameters,
        until_s: f64,
        exponent: f64,
    ) -> Result<Simulated, String>;
}

/// The wind's shear a fit of heights alone carries: the power-law exponent of IEC 61400-1's normal
/// wind profile (IEC 61400-1:2019, §6.3.2.2, α = 0.2), above the 1/7 law's neutral 0.14. The fit's
/// wind is constant with height, and a real one grows.
pub const SHEAR_EXPONENT: f64 = 0.2;

/// A fit's answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fit {
    /// The parameters at the solution.
    pub parameters: Parameters,
    /// Each one's standard error, in [`Parameters::to_array`]'s order.
    pub standard_errors: [f64; FLOWN],
    /// The clock's offset: the log's clock at the simulation's zero, s.
    pub clock_offset_s: f64,
    /// What the log's pad reads above the simulation's, m.
    pub height_offset_m: f64,
    /// The heights' residual's RMS, m; `None` where the log said nothing.
    pub rms_m: Option<f64>,
    /// The noise a height's residual was taken to have at the end, m.
    pub height_noise_m: f64,
    /// The noise a horizontal position's residual was taken to have at the end, m.
    pub horizontal_noise_m: f64,
    /// Whether the log was fitted at all. If not, each parameter is what was stated of it.
    pub fitted: bool,
    /// Whether its last pass ended where no step lowered χ² further: not where it reached its limit
    /// of 60 steps, or where a flight for its slopes failed.
    pub converged: bool,
    /// The heights' residual's RMS as a share of the spread of the heights read; past
    /// [`MISFIT_SHARE`] the log said nothing. `None` where it never came to one.
    pub misfit_share: Option<f64>,
    /// Whether each of the log's height bins was read, in their order; empty where the log said
    /// nothing. A barometer's bins at or past [`readings::TRANSONIC_MACH`] on the flight found are
    /// left out.
    pub read: Vec<bool>,
    /// Flights flown: the least-squares fit's and, by the Bayesian option on a log with a track
    /// whose noise the fit takes from the log, its search for the posterior's peak (21). The
    /// posterior's draws are counted apart.
    pub flown: u64,
}

impl Fit {
    /// What the fit says when the log says nothing: what was stated of each parameter.
    #[must_use]
    pub fn of_nothing(stated: &Stated) -> Self {
        Self {
            parameters: Parameters::from_array(stated.map(|p| p.mean)),
            standard_errors: stated.map(|p| p.sd),
            clock_offset_s: 0.0,
            height_offset_m: 0.0,
            rms_m: None,
            height_noise_m: FLOOR_M,
            horizontal_noise_m: FLOOR_HORIZONTAL_M,
            fitted: false,
            converged: false,
            misfit_share: None,
            read: Vec::new(),
            flown: 0,
        }
    }
}

/// The step, for the Jacobian's differences, on each flown parameter.
const STEPS: [f64; FLOWN] = [0.02, 0.01, 0.02, 1.0, 1.0, 0.01];
/// What is stated of the height offset: the reader's pad's reach, m.
const HEIGHT_OFFSET_SD_M: f64 = 5.0;
/// What is stated of the clock's offset about its search's best, s.
const CLOCK_OFFSET_SD_S: f64 = 2.0;
/// What is stated of a barometer's gain on the height above the pad: MS5607's ±0.25% taken as the
/// logger's model takes it, as a standard deviation of a uniform draw.
const GAIN_SD: f64 = 0.0025 / 1.732_050_807_568_877_2;
/// What is stated of a barometer's drift, m/s: a pad that moves 1 m in 20 s of flight.
const DRIFT_SD_M_S: f64 = 0.05;
/// What is stated of a barometer's port's pressure coefficient, on its reading above the true
/// height: a uniform draw from nothing up to [`readings::PORT_PRESSURE_COEFFICIENT`], as the
/// tunnel's coefficients below Mach 0.8 lie, negative in pressure (`synthetic::noise`). Its middle:
const PORT_MEAN: f64 = 0.5 * readings::PORT_PRESSURE_COEFFICIENT;
/// And its standard deviation.
const PORT_SD: f64 = readings::PORT_PRESSURE_COEFFICIENT / 3.464_101_615_137_754_6;
/// What is stated of an offset known exactly, in its unit: small enough to hold it, large enough
/// for the normal equations to stay well conditioned.
const KNOWN_SD: f64 = 1e-4;

/// The fit's residual, weighted: heights, then the horizontal track's east and north, then each
/// flown parameter from what is stated of it, then the two offsets.
#[derive(Clone)]
struct Problem<'a> {
    observed: &'a Observations,
    stated: &'a Stated,
    clock_mean_s: f64,
    clock_sd_s: f64,
    height_offset_sd_m: f64,
    gain_sd: f64,
    drift_sd_m_s: f64,
    port_mean: f64,
    port_sd: f64,
    height_sd_m: f64,
    horizontal_sd_m: f64,
    /// The natural frequency of the receiver's navigation filter the track's rows are read
    /// through, rad/s ([`Simulated::lagged_drift_m`]); `None` reads the drift as flown. Only the
    /// Bayesian option's proposal sets it.
    receiver_rad_s: Option<f64>,
    /// Whether the Bayesian option takes the track's noise, run, pad and receiver's filter as
    /// unknowns ([`Problem::track_noise_integrated`]): only where the noise comes from the log's
    /// own residuals, [`Noise::FromResiduals`]; noise given or known is taken as it is given.
    track_unknowns: bool,
    /// Whether each height bin is read: a barometer's bin at or past [`readings::TRANSONIC_MACH`]
    /// on the flight first flown isn't, as its port reads the flow over the nose.
    read: Vec<bool>,
}

impl Problem<'_> {
    /// The height the log's sensor would read at `t` on its clock, on `x`.
    fn height_m(&self, x: &[f64; UNKNOWNS], flown: &Simulated, t: f64) -> f64 {
        let (tau, h0, gain, drift, port) = (
            x[FLOWN],
            x[FLOWN + 1],
            x[FLOWN + 2],
            x[FLOWN + 3],
            x[FLOWN + 4],
        );
        let at = t - tau;
        (1.0 + gain) * flown.read_height_m(at, self.observed.kind)
            + h0
            + drift * (t - self.observed.liftoff_s)
            + port * flown.port_m(at)
    }

    fn residuals(&self, x: &[f64; UNKNOWNS], flown: &Simulated) -> Vec<f64> {
        let (tau, h0) = (x[FLOWN], x[FLOWN + 1]);
        let mut r = Vec::with_capacity(
            self.observed.heights.len() + 2 * self.observed.horizontal.len() + UNKNOWNS,
        );
        for (&(t, y), read) in self.observed.heights.iter().zip(&self.read) {
            r.push(if *read {
                (y - self.height_m(x, flown, t)) / self.height_sd_m
            } else {
                0.0
            });
        }
        for (east, north) in self.track_misfit_m(tau, flown) {
            r.push(east / self.horizontal_sd_m);
            r.push(north / self.horizontal_sd_m);
        }
        for (value, prior) in x.iter().zip(self.stated) {
            r.push((value - prior.mean) / prior.sd);
        }
        r.push((tau - self.clock_mean_s) / self.clock_sd_s);
        r.push(h0 / self.height_offset_sd_m);
        r.push(x[FLOWN + 2] / self.gain_sd);
        r.push(x[FLOWN + 3] / self.drift_sd_m_s);
        r.push((x[FLOWN + 4] - self.port_mean) / self.port_sd);
        r
    }

    /// The columns of the unknowns that fly nothing, by differences.
    fn sensor_columns(&self, x: &[f64; UNKNOWNS], flown: &Simulated) -> [Vec<f64>; 5] {
        let base = self.residuals(x, flown);
        [0.02, 1.0, 1e-3, 0.01, 1e-3]
            .into_iter()
            .enumerate()
            .map(|(k, delta)| {
                let mut ahead = *x;
                ahead[FLOWN + k] += delta;
                let mut column = self.residuals(&ahead, flown);
                for (c, b) in column.iter_mut().zip(&base) {
                    *c = (*c - b) / delta;
                }
                column
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!("five deltas make five columns"))
    }
}

fn chi2(r: &[f64]) -> f64 {
    r.iter().map(|v| v * v).sum()
}

/// The lower triangle `L` of `a = L Lᵀ`; `None` if `a` isn't positive definite.
pub(crate) fn cholesky<const N: usize>(a: &[[f64; N]; N]) -> Option<[[f64; N]; N]> {
    let mut l = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let sum: f64 = (0..j).map(|k| l[i][k] * l[j][k]).sum();
            if i == j {
                let d = a[i][i] - sum;
                if d <= 0.0 || !d.is_finite() {
                    return None;
                }
                l[i][i] = d.sqrt();
            } else {
                l[i][j] = (a[i][j] - sum) / l[j][j];
            }
        }
    }
    Some(l)
}

/// `x` with `L Lᵀ x = b`.
fn solve<const N: usize>(l: &[[f64; N]; N], b: &[f64; N]) -> [f64; N] {
    let mut x = *b;
    for i in 0..N {
        let sum: f64 = (0..i).map(|k| l[i][k] * x[k]).sum();
        x[i] = (x[i] - sum) / l[i][i];
    }
    for i in (0..N).rev() {
        let sum: f64 = (i + 1..N).map(|k| l[k][i] * x[k]).sum();
        x[i] = (x[i] - sum) / l[i][i];
    }
    x
}

/// The clock offset that best lays `flown` on the heights, within [`OFFSET_SEARCH_S`] of `first_s`.
fn best_offset_s(observed: &Observations, flown: &Simulated, first_s: f64) -> f64 {
    let steps = (OFFSET_SEARCH_S / OFFSET_STEP_S).round();
    let mut best = (f64::INFINITY, first_s);
    let mut k = -steps;
    while k <= steps {
        let tau = first_s + k * OFFSET_STEP_S;
        let cost: f64 = observed
            .heights
            .iter()
            .map(|&(t, y)| (y - flown.read_height_m(t - tau, observed.kind)).powi(2))
            .sum();
        if cost < best.0 {
            best = (cost, tau);
        }
        k += 1.0;
    }
    best.1
}

/// Fits `forward` to `observed`, each parameter held to `stated`. A log that says nothing, or a
/// flight that won't fly on what was stated, gives [`Fit::of_nothing`].
pub fn fit(
    observed: Option<&Observations>,
    stated: &Stated,
    noise: Noise,
    forward: &mut dyn Forward,
) -> Fit {
    fit_and_sample(observed, stated, noise, forward, None).0
}

/// How the Bayesian option draws its posterior ([`bayesian`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sampling {
    /// The pilot's draws, which widen the proposal and aren't read
    /// ([`posterior::sample_in_two`]).
    pub pilot: usize,
    /// The draws read after it.
    pub draws: usize,
    /// The draws' seed.
    pub seed: u64,
}

/// The Bayesian option's answer: the least-squares fit it starts from, and the posterior of the
/// flown parameters drawn around that fit's answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bayesian {
    /// The least-squares fit, as [`fit`] gives it.
    pub least_squares: Fit,
    /// The flown parameters' 5%, 50% and 95% points, in [`Parameters::to_array`]'s order, with the
    /// draws' counts. `None` where the least-squares fit wasn't fitted, so that each parameter is
    /// what was stated of it, or where no draw had a weight.
    pub posterior: Option<Posterior<FLOWN>>,
}

/// The Bayesian option ([ADR-317][adr-317], [ADR-320][adr-320]): [`fit`], then the posterior of
/// its unknowns drawn around its answer by [`posterior::sample_in_two`], each draw a flight flown.
///
/// The posterior is the least-squares fit's own on the heights: the same residuals at the same
/// noise, the noise widened for a residual that runs on from bin to bin as the fit's errors widen
/// it, the same bins read and the same priors. On a track whose noise the fit takes from the log,
/// it reads the drift through a receiver's navigation filter and integrates out the track's noise,
/// its run and the pad's position ([ADR-321][adr-321], `Problem::track_noise_integrated`); its
/// draws are then centered near that posterior's peak, by three Gauss-Newton steps. The
/// sensor's four linear unknowns (height offset, gain, drift, port) are integrated out in closed
/// form, `−2 ln p(θ) = χ²_min(θ) + ln det H(θ)`, so seven unknowns are drawn: the six flown and
/// the clock. On a log of height alone the wind's shear exponent is an eighth, uniform on
/// `[0, SHEAR_EXPONENT]`. The wind is drawn first, from its block of the fit's covariance at the
/// answer; the rest about where one Gauss-Newton step puts them in that wind, from their
/// covariance given the wind; a pilot widens either block's scale where the posterior reaches
/// further. Its 90% credible intervals are the posterior's 5% and 95% points as they come: no
/// shares widen them.
///
/// [adr-317]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0317-m7-3c-in-two-the-sampler-then-the-test.md
/// [adr-320]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0320-m7-3c2-the-bayesian-option-in-two-blocks.md
/// [adr-321]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
pub fn bayesian(
    observed: Option<&Observations>,
    stated: &Stated,
    noise: Noise,
    forward: &mut dyn Forward,
    sampling: Sampling,
) -> Bayesian {
    let (least_squares, posterior) =
        fit_and_sample(observed, stated, noise, forward, Some(sampling));
    Bayesian {
        least_squares,
        posterior,
    }
}

/// [`fit`], and with `sampling` the posterior [`bayesian`] draws.
fn fit_and_sample(
    observed: Option<&Observations>,
    stated: &Stated,
    noise: Noise,
    forward: &mut dyn Forward,
    sampling: Option<Sampling>,
) -> (Fit, Option<Posterior<FLOWN>>) {
    let Some(observed) = observed else {
        return (Fit::of_nothing(stated), None);
    };
    // Long enough for a rocket that flies well past the stated flight's top.
    let until_s = 1.6 * observed.flown_s + 5.0;
    let flown = Cell::new(0_u64);
    let mut fly = |x: &[f64; UNKNOWNS]| {
        flown.set(flown.get() + 1);
        let mut parameters = [0.0; FLOWN];
        parameters.copy_from_slice(&x[..FLOWN]);
        forward
            .fly(&Parameters::from_array(parameters), until_s)
            .ok()
    };
    let mut x = [0.0; UNKNOWNS];
    for (value, prior) in x.iter_mut().zip(stated) {
        *value = prior.mean;
    }
    let nothing = || Fit {
        flown: flown.get(),
        ..Fit::of_nothing(stated)
    };
    let Some(mut at_x) = fly(&x) else {
        return (nothing(), None);
    };
    let open_sensors = |height_sd_m: f64, horizontal_sd_m: f64| Problem {
        observed,
        stated,
        // The simulated liftoff sits near the simulation's zero, so the log's liftoff starts the
        // clock.
        clock_mean_s: best_offset_s(observed, &at_x, observed.liftoff_s),
        clock_sd_s: CLOCK_OFFSET_SD_S,
        height_offset_sd_m: HEIGHT_OFFSET_SD_M,
        gain_sd: if observed.kind == HeightKind::Barometric {
            GAIN_SD
        } else {
            KNOWN_SD
        },
        drift_sd_m_s: DRIFT_SD_M_S,
        port_mean: if observed.kind == HeightKind::Barometric
            && matches!(noise, Noise::FromResiduals)
        {
            PORT_MEAN
        } else {
            0.0
        },
        port_sd: if observed.kind == HeightKind::Barometric {
            PORT_SD
        } else {
            KNOWN_SD
        },
        height_sd_m,
        horizontal_sd_m,
        receiver_rad_s: None,
        track_unknowns: matches!(noise, Noise::FromResiduals),
        read: Vec::new(),
    };
    let mut problem = match noise {
        Noise::FromResiduals => open_sensors(2.0 * FLOOR_M, 2.0 * FLOOR_HORIZONTAL_M),
        Noise::Given {
            height_m,
            horizontal_m,
        } => open_sensors(height_m, horizontal_m),
        Noise::Known {
            height_m,
            horizontal_m,
        } => Problem {
            observed,
            stated,
            clock_mean_s: 0.0,
            clock_sd_s: KNOWN_SD,
            height_offset_sd_m: KNOWN_SD,
            gain_sd: KNOWN_SD,
            drift_sd_m_s: KNOWN_SD,
            port_mean: 0.0,
            port_sd: KNOWN_SD,
            height_sd_m: height_m,
            horizontal_sd_m: horizontal_m,
            receiver_rad_s: None,
            track_unknowns: false,
            read: Vec::new(),
        },
    };
    x[FLOWN] = problem.clock_mean_s;
    x[FLOWN + 4] = problem.port_mean;
    let mut information = [[0.0; UNKNOWNS]; UNKNOWNS];
    let mut converged = false;
    // A first pass on the stated flight, then one on what it found: the noise from its residuals
    // and the bins it reads from the flight it found, as a rocket faster than stated passes
    // Mach 0.8 earlier. With the noise and clock known there's no second to take.
    let passes = if matches!(noise, Noise::Known { .. }) {
        1
    } else {
        2
    };
    for pass in 0..passes {
        problem.read = observed
            .heights
            .iter()
            .map(|&(t, _)| {
                observed.reads_transonic || at_x.mach(t - x[FLOWN]) < readings::TRANSONIC_MACH
            })
            .collect();
        if problem.read.iter().filter(|read| **read).count() <= UNKNOWNS {
            return (nothing(), None);
        }
        converged = false;
        let mut lambda = 1e-3;
        let mut r = problem.residuals(&x, &at_x);
        let mut cost = chi2(&r);
        for _ in 0..MAX_ITERATIONS {
            // The Jacobian of the residuals against the unknowns, by forward differences.
            let mut columns: Vec<Vec<f64>> = Vec::with_capacity(UNKNOWNS);
            let mut failed = false;
            for (j, step) in STEPS.iter().enumerate() {
                let mut ahead = x;
                ahead[j] += step;
                match fly(&ahead) {
                    Some(flight) => {
                        let mut column = problem.residuals(&ahead, &flight);
                        for (c, b) in column.iter_mut().zip(&r) {
                            *c = (*c - b) / step;
                        }
                        columns.push(column);
                    }
                    None => {
                        failed = true;
                        break;
                    }
                }
            }
            if failed {
                break;
            }
            columns.extend(problem.sensor_columns(&x, &at_x));
            let mut gradient = [0.0; UNKNOWNS];
            for i in 0..UNKNOWNS {
                gradient[i] = columns[i].iter().zip(&r).map(|(a, b)| a * b).sum();
                for j in 0..=i {
                    let value: f64 = columns[i].iter().zip(&columns[j]).map(|(a, b)| a * b).sum();
                    information[i][j] = value;
                    information[j][i] = value;
                }
            }
            let mut improved = None;
            while lambda < 1e8 {
                let mut damped = information;
                for (i, row) in damped.iter_mut().enumerate() {
                    row[i] *= 1.0 + lambda;
                }
                let Some(l) = cholesky(&damped) else {
                    lambda *= 10.0;
                    continue;
                };
                let step = solve(&l, &gradient.map(|g| -g));
                let mut next = x;
                for (value, delta) in next.iter_mut().zip(step) {
                    *value += delta;
                }
                let tried = fly(&next).and_then(|flight| {
                    let tried = problem.residuals(&next, &flight);
                    let tried_cost = chi2(&tried);
                    (tried_cost < cost).then_some((flight, tried, tried_cost))
                });
                if let Some(better) = tried {
                    x = next;
                    improved = Some(better);
                    lambda = (lambda * 0.1).max(1e-9);
                    break;
                }
                lambda *= 10.0;
            }
            let Some((flight, tried, tried_cost)) = improved else {
                // No step down at any damping: it stands at the bottom, to rounding.
                converged = true;
                break;
            };
            let drop = cost - tried_cost;
            at_x = flight;
            r = tried;
            cost = tried_cost;
            if drop < CONVERGED_CHI2 {
                converged = true;
                break;
            }
        }
        if pass + 1 < passes && matches!(noise, Noise::FromResiduals) {
            let (height, horizontal) = residual_noise(&problem, &x, &at_x);
            problem.height_sd_m = height.max(FLOOR_M);
            problem.horizontal_sd_m = horizontal.max(FLOOR_HORIZONTAL_M);
        }
    }
    // The noise as the fit weighed it, and as its errors take it: a residual that runs on from bin
    // to bin tells less than as many independent ones.
    let (height_noise_m, horizontal_noise_m) = (problem.height_sd_m, problem.horizontal_sd_m);
    if matches!(noise, Noise::FromResiduals) {
        let (height, horizontal) = residual_correlation(&problem, &x, &at_x);
        problem.height_sd_m *= inflation(height).sqrt();
        problem.horizontal_sd_m *= inflation(horizontal).sqrt();
    }
    // The Jacobian at the solution, with the noise as it ends.
    let r = problem.residuals(&x, &at_x);
    let mut columns: Vec<Vec<f64>> = Vec::with_capacity(UNKNOWNS);
    for (j, step) in STEPS.iter().enumerate() {
        let mut ahead = x;
        ahead[j] += step;
        let Some(flight) = fly(&ahead) else {
            return (nothing(), None);
        };
        let mut column = problem.residuals(&ahead, &flight);
        for (c, b) in column.iter_mut().zip(&r) {
            *c = (*c - b) / step;
        }
        columns.push(column);
    }
    columns.extend(problem.sensor_columns(&x, &at_x));
    for i in 0..UNKNOWNS {
        for j in 0..=i {
            let value: f64 = columns[i].iter().zip(&columns[j]).map(|(a, b)| a * b).sum();
            information[i][j] = value;
            information[j][i] = value;
        }
    }
    let Some(l) = cholesky(&information) else {
        return (nothing(), None);
    };
    let mut parameters = [0.0; FLOWN];
    parameters.copy_from_slice(&x[..FLOWN]);
    // Heights alone can't tell the wind, and its loss grows as its speed squared, which the
    // errors' first order at the stated wind leaves out. So the answer is moved, to first order,
    // to a calm and to a strong wind, the stated speed and 1.645 of a component's standard
    // deviations past it, grown with height by [`SHEAR_EXPONENT`], and
    // the wind's spread is anywhere between: the larger move either way, as the standard
    // deviation of a uniform spread, `δ/√3`.
    let mut windy = [0.0; UNKNOWNS];
    if matches!(noise, Noise::FromResiduals) && observed.horizontal.is_empty() {
        let (east, north) = (&stated[3], &stated[4]);
        // Toward the stated wind's way, clockwise from north; north where none is stated.
        let toward = east.mean.atan2(north.mean);
        let top = east.mean.hypot(north.mean) + Z_90 * east.sd.max(north.sd);
        let with_wind = |speed: f64| {
            let mut at = parameters;
            at[3] = speed * toward.sin();
            at[4] = speed * toward.cos();
            Parameters::from_array(at)
        };
        flown.set(flown.get() + 2);
        let (Ok(calm), Ok(strong)) = (
            forward.fly(&with_wind(0.0), until_s),
            forward.fly_sheared(&with_wind(top), until_s, SHEAR_EXPONENT),
        ) else {
            return (nothing(), None);
        };
        let moved = |flight: &Simulated| {
            let to = problem.residuals(&x, flight);
            let mut pull = [0.0; UNKNOWNS];
            for (p, column) in pull.iter_mut().zip(&columns) {
                *p = column
                    .iter()
                    .zip(to.iter().zip(&r))
                    .map(|(c, (m, b))| c * (m - b))
                    .sum();
            }
            solve(&l, &pull)
        };
        let (to_calm, to_strong) = (moved(&calm), moved(&strong));
        for (w, (a, b)) in windy.iter_mut().zip(to_calm.iter().zip(&to_strong)) {
            *w = a.abs().max(b.abs());
        }
    }
    let mut standard_errors = [0.0; FLOWN];
    for (j, se) in standard_errors.iter_mut().enumerate() {
        let mut unit = [0.0; UNKNOWNS];
        unit[j] = 1.0;
        *se = (solve(&l, &unit)[j] + windy[j].powi(2) / 3.0).sqrt();
    }
    let (rms_m, _) = residual_noise(&problem, &x, &at_x);
    let misfit_share = rms_m / problem.read_spread_m();
    if misfit_share.is_nan() || misfit_share > MISFIT_SHARE {
        return (
            Fit {
                misfit_share: Some(misfit_share),
                ..nothing()
            },
            None,
        );
    }
    let posterior = sampling.and_then(|sampling| {
        // The Bayesian option reads the track through the receiver's navigation filter and weighs
        // it by its own noise, both integrated out (`Problem::track_noise_integrated`), not as the
        // least-squares fit does. So its proposal is centered near that posterior's peak: three
        // undamped Gauss-Newton steps from the answer, each reading the track through the filter
        // the track's likelihood favors there and weighing it by its misfit's RMS, widened for a
        // misfit that runs on; the covariance and Jacobian are those of the last step.
        let mut posterior_problem = problem.clone();
        let mut posterior_columns = columns.clone();
        let mut posterior_l = l;
        let mut center = x;
        if problem.track_unknowns && !observed.horizontal.is_empty() {
            let as_parameters = |x: &[f64; UNKNOWNS]| {
                let mut parameters = [0.0; FLOWN];
                parameters.copy_from_slice(&x[..FLOWN]);
                Parameters::from_array(parameters)
            };
            for _ in 0..PEAK_STEPS {
                flown.set(flown.get() + 1);
                let at_center = forward.fly(&as_parameters(&center), until_s).ok()?;
                let omega = posterior_problem
                    .receiver_logs(center[FLOWN], &at_center)
                    .into_iter()
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(omega, _)| omega)?;
                posterior_problem.receiver_rad_s = Some(omega);
                let (_, rms_m) = residual_noise(&posterior_problem, &center, &at_center);
                let (_, rho) = residual_correlation(&posterior_problem, &center, &at_center);
                posterior_problem.horizontal_sd_m =
                    rms_m.max(PROPOSAL_HORIZONTAL_FLOOR_M) * inflation(rho).sqrt();
                let r = posterior_problem.residuals(&center, &at_center);
                let mut step_columns: Vec<Vec<f64>> = Vec::with_capacity(UNKNOWNS);
                for (j, step) in STEPS.iter().enumerate() {
                    let mut ahead = center;
                    ahead[j] += step;
                    flown.set(flown.get() + 1);
                    let flight = forward.fly(&as_parameters(&ahead), until_s).ok()?;
                    let mut column = posterior_problem.residuals(&ahead, &flight);
                    for (c, b) in column.iter_mut().zip(&r) {
                        *c = (*c - b) / step;
                    }
                    step_columns.push(column);
                }
                step_columns.extend(posterior_problem.sensor_columns(&center, &at_center));
                let mut information = [[0.0; UNKNOWNS]; UNKNOWNS];
                for i in 0..UNKNOWNS {
                    for j in 0..=i {
                        let value: f64 = step_columns[i]
                            .iter()
                            .zip(&step_columns[j])
                            .map(|(a, b)| a * b)
                            .sum();
                        information[i][j] = value;
                        information[j][i] = value;
                    }
                }
                let factor = cholesky(&information)?;
                let pull: [f64; UNKNOWNS] = std::array::from_fn(|j| {
                    -step_columns[j]
                        .iter()
                        .zip(&r)
                        .map(|(c, v)| c * v)
                        .sum::<f64>()
                });
                let step = solve(&factor, &pull);
                posterior_columns = step_columns;
                posterior_l = factor;
                if !step.iter().all(|v| v.is_finite()) {
                    break;
                }
                for (c, d) in center.iter_mut().zip(step) {
                    *c += d;
                }
            }
        }
        let mut covariance = [[0.0; UNKNOWNS]; UNKNOWNS];
        for j in 0..UNKNOWNS {
            let mut unit = [0.0; UNKNOWNS];
            unit[j] = 1.0;
            for (i, value) in solve(&posterior_l, &unit).into_iter().enumerate() {
                covariance[i][j] = value;
            }
        }
        let laplace = Laplace {
            x: &center,
            covariance: &covariance,
            columns: &posterior_columns,
            problem: &posterior_problem,
            until_s,
        };
        // On heights alone, the wind's growth with height is unknown too, as the errors' strong
        // wind is sheared.
        if matches!(noise, Noise::FromResiduals) && observed.horizontal.is_empty() {
            draw_posterior::<{ DRAWN + 1 }>(&laplace, forward, sampling)
        } else {
            draw_posterior::<DRAWN>(&laplace, forward, sampling)
        }
    });
    let fit = Fit {
        parameters: Parameters::from_array(parameters),
        standard_errors,
        clock_offset_s: x[FLOWN],
        height_offset_m: x[FLOWN + 1],
        rms_m: Some(rms_m),
        height_noise_m,
        horizontal_noise_m,
        fitted: true,
        converged,
        misfit_share: Some(misfit_share),
        read: problem.read,
        flown: flown.get(),
    };
    (fit, posterior)
}

/// What the Bayesian option draws about: the least-squares answer, its Laplace covariance, the
/// Jacobian's columns there, the problem as it ends, and how long a flight flies.
struct Laplace<'a> {
    x: &'a [f64; UNKNOWNS],
    covariance: &'a [[f64; UNKNOWNS]; UNKNOWNS],
    columns: &'a [Vec<f64>],
    problem: &'a Problem<'a>,
    until_s: f64,
}

/// The wind's shear exponent on a log of height alone, where the Bayesian option draws it as an
/// unknown: uniform from a constant wind's 0 to [`SHEAR_EXPONENT`], stated, not fitted.
const SHEAR_STATED: [f64; 2] = [0.0, SHEAR_EXPONENT];

/// The unknowns the Bayesian option draws: the flown parameters and the clock's offset. The
/// height offset and the sensor's gain, drift and port come after them, and are integrated out.
const DRAWN: usize = FLOWN + 1;

/// The sensor's unknowns that enter a height linearly, each with a normal prior: the height
/// offset, gain, drift and port. With the rest held, the fit's χ² is a quadratic in them.
const LINEAR: usize = UNKNOWNS - DRAWN;

/// The fit's χ² on given flown parameters and clock offset with the linear sensor unknowns
/// integrated out ([`Problem::integrated`]).
struct Integrated {
    /// The least χ² over the linear unknowns.
    chi2_min: f64,
    /// `ln det H`, `H` the χ²'s curvature in them, over 2.
    log_det: f64,
    /// Where they make it least, `β*`.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "read by the tests' check of the integral")
    )]
    best: [f64; LINEAR],
    /// The residuals there.
    at: Vec<f64>,
}

impl Integrated {
    /// `−2 ln` of the posterior, up to a constant: `χ²_min + ln det H`.
    fn minus_two_log(&self) -> f64 {
        self.chi2_min + self.log_det
    }
}

impl Problem<'_> {
    /// The fit's χ² on `theta`, the flown parameters and the clock offset, with the linear sensor
    /// unknowns `β` integrated out. The residuals are exactly linear in them, `r = r₀ + A β`, so
    /// `χ²(β) = χ²_min + (β − β*)ᵀ H (β − β*)` with `H = AᵀA` and
    /// `∫ exp(−χ²/2) dβ ∝ exp(−χ²_min/2) det(H)^(−1/2)`, `H` changing with `theta` through the
    /// gain's and the port's columns. `None` where `H` isn't positive definite.
    fn integrated(&self, theta: &[f64; DRAWN], flown: &Simulated) -> Option<Integrated> {
        let mut base = [0.0; UNKNOWNS];
        base[..DRAWN].copy_from_slice(theta);
        let r0 = self.residuals(&base, flown);
        let columns: [Vec<f64>; LINEAR] = std::array::from_fn(|k| {
            let mut unit = base;
            unit[DRAWN + k] = 1.0;
            let mut column = self.residuals(&unit, flown);
            for (c, b) in column.iter_mut().zip(&r0) {
                *c -= b;
            }
            column
        });
        let mut h = [[0.0; LINEAR]; LINEAR];
        let mut pull = [0.0; LINEAR];
        for i in 0..LINEAR {
            pull[i] = -columns[i].iter().zip(&r0).map(|(a, b)| a * b).sum::<f64>();
            for j in 0..=i {
                let value: f64 = columns[i].iter().zip(&columns[j]).map(|(a, b)| a * b).sum();
                h[i][j] = value;
                h[j][i] = value;
            }
        }
        let l = cholesky(&h)?;
        let best = solve(&l, &pull);
        let mut at = r0;
        for (k, column) in columns.iter().enumerate() {
            for (r, c) in at.iter_mut().zip(column) {
                *r += best[k] * c;
            }
        }
        Some(Integrated {
            chi2_min: chi2(&at),
            log_det: (0..LINEAR).map(|i| 2.0 * l[i][i].ln()).sum(),
            best,
            at,
        })
    }
}

/// The posterior of the flown parameters about `laplace`'s answer, by [`posterior::sample_in_two`],
/// the linear sensor unknowns integrated out ([`Problem::integrated`]): the wind first, and with
/// `M` = [`DRAWN`] + 1 its shear exponent with it, drawn uniform on [`SHEAR_STATED`] and flown by
/// [`Forward::fly_sheared`]; then the rest, about where a Gauss–Newton step from the answer puts
/// them, `Δy = −H_y⁻¹ J_yᵀ r(w, y, β*)`, `H_y` the answer's information on them with the linear
/// unknowns integrated out (its Schur complement). `M` is [`DRAWN`] or one more.
fn draw_posterior<const M: usize>(
    laplace: &Laplace<'_>,
    forward: &mut dyn Forward,
    sampling: Sampling,
) -> Option<Posterior<FLOWN>> {
    let sheared = M > DRAWN;
    let (x, problem) = (laplace.x, laplace.problem);
    let [shear_low, shear_high] = SHEAR_STATED;
    let mut peak = [0.0; M];
    peak[..DRAWN].copy_from_slice(&x[..DRAWN]);
    // A normal's marginal is its covariance's block.
    let mut covariance = [[0.0; M]; M];
    for (row, from) in covariance.iter_mut().zip(laplace.covariance).take(DRAWN) {
        row[..DRAWN].copy_from_slice(&from[..DRAWN]);
    }
    let mut outer = [false; M];
    outer[WIND_EAST] = true;
    outer[WIND_NORTH] = true;
    if sheared {
        // The uniform's middle and variance.
        peak[DRAWN] = 0.5 * (shear_low + shear_high);
        covariance[DRAWN][DRAWN] = (shear_high - shear_low).powi(2) / 12.0;
        outer[DRAWN] = true;
    }
    let inner: Vec<usize> = (0..DRAWN).filter(|&i| !outer[i]).collect();
    // The information on the drawn unknowns with the linear ones integrated out: the inverse of
    // the covariance's block.
    let mut block = [[0.0; DRAWN]; DRAWN];
    for (row, from) in block.iter_mut().zip(laplace.covariance) {
        row.copy_from_slice(&from[..DRAWN]);
    }
    let block = cholesky(&block)?;
    let mut held = [[0.0; DRAWN - 2]; DRAWN - 2];
    let mut held_full = [[0.0; DRAWN]; DRAWN];
    for j in 0..DRAWN {
        let mut unit = [0.0; DRAWN];
        unit[j] = 1.0;
        for (i, value) in solve(&block, &unit).into_iter().enumerate() {
            held_full[i][j] = value;
        }
    }
    for (a, &i) in inner.iter().enumerate() {
        for (b, &j) in inner.iter().enumerate() {
            held[a][b] = held_full[i][j];
        }
    }
    let held = cholesky(&held)?;
    let forward = RefCell::new(forward);
    let fly_at = |point: &[f64; M]| {
        let mut parameters = [0.0; FLOWN];
        parameters.copy_from_slice(&point[..FLOWN]);
        let parameters = Parameters::from_array(parameters);
        let mut forward = forward.borrow_mut();
        if sheared {
            forward.fly_sheared(&parameters, laplace.until_s, point[DRAWN])
        } else {
            forward.fly(&parameters, laplace.until_s)
        }
        .ok()
    };
    let theta = |point: &[f64; M]| -> [f64; DRAWN] { std::array::from_fn(|i| point[i]) };
    let outside = |point: &[f64; M]| sheared && !(shear_low..=shear_high).contains(&point[DRAWN]);
    let drawn = posterior::sample_in_two(
        &peak,
        &covariance,
        &outer,
        sampling.pilot,
        sampling.draws,
        sampling.seed,
        |point| {
            if outside(point) {
                // Its weight is nothing: no flight is needed for where it centers.
                return Some(*point);
            }
            let flight = fly_at(point)?;
            let at = problem.integrated(&theta(point), &flight)?.at;
            let pull: [f64; DRAWN - 2] = std::array::from_fn(|a| {
                laplace.columns[inner[a]]
                    .iter()
                    .zip(&at)
                    .map(|(c, v)| c * v)
                    .sum()
            });
            let step = solve(&held, &pull);
            let mut middle = *point;
            for (a, &i) in inner.iter().enumerate() {
                middle[i] = x[i] - step[a];
            }
            Some(middle)
        },
        |point| {
            if outside(point) {
                return Some(f64::NEG_INFINITY);
            }
            let flight = fly_at(point)?;
            let theta = theta(point);
            let integrated = problem.integrated(&theta, &flight)?;
            Some(
                -0.5 * (integrated.minus_two_log()
                    + problem.track_noise_integrated(theta[FLOWN], &flight, &integrated.at)),
            )
        },
    )?;
    Some(Posterior {
        quantiles: std::array::from_fn(|j| drawn.quantiles[j]),
        effective_draws: drawn.effective_draws,
        draws: drawn.draws,
        failed: drawn.failed,
        enough: drawn.enough,
    })
}

impl Problem<'_> {
    /// The standard deviation of the heights read, m.
    fn read_spread_m(&self) -> f64 {
        let read: Vec<f64> = self
            .observed
            .heights
            .iter()
            .zip(&self.read)
            .filter(|(_, read)| **read)
            .map(|(&(_, h), _)| h)
            .collect();
        #[expect(
            clippy::cast_precision_loss,
            reason = "a count of bins is far below 2^53"
        )]
        let n = read.len().max(1) as f64;
        let mean = read.iter().sum::<f64>() / n;
        (read.iter().map(|h| (h - mean).powi(2)).sum::<f64>() / n).sqrt()
    }
}

impl Problem<'_> {
    /// What the Bayesian option's `−2 ln p` takes in place of the track's share of `at`, the
    /// residuals [`Problem::residuals`] gives: [`track_minus_two_log`] of the track's residuals in
    /// meters, less their weighted squares, which `χ²` holds. 0 for a log with no track. The track's
    /// residuals depend on no linear sensor unknown, so its share of `χ²_min` is theirs as they are.
    fn track_noise_integrated(&self, clock_s: f64, flown: &Simulated, at: &[f64]) -> f64 {
        if !self.track_unknowns || self.observed.horizontal.is_empty() {
            return 0.0;
        }
        // The residuals hold one per height bin, then east and north for each track bin.
        let start = self.observed.heights.len();
        let rows = &at[start..start + 2 * self.observed.horizontal.len()];
        let logs: Vec<f64> = self
            .receiver_logs(clock_s, flown)
            .into_iter()
            .map(|(_, log)| log)
            .collect();
        let top = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = logs.iter().map(|l| (l - top).exp()).sum();
        let nodes = f64::from(RECEIVER_LOOP_NODES);
        -2.0 * (top + (sum / nodes).ln()) - chi2(rows)
    }

    /// At each of [`RECEIVER_LOOP_NODES`] natural frequencies over [`RECEIVER_LOOP_STATED_RAD_S`],
    /// evenly in its logarithm, that frequency and `ln` of the track's likelihood read through a
    /// filter of it, the noise integrated out ([`track_minus_two_log`]), on clock offset `clock_s`.
    fn receiver_logs(&self, clock_s: f64, flown: &Simulated) -> Vec<(f64, f64)> {
        let times: Vec<f64> = self
            .observed
            .horizontal
            .iter()
            .map(|&(t, _, _)| t - clock_s)
            .collect();
        let nodes = f64::from(RECEIVER_LOOP_NODES);
        let [low, high] = RECEIVER_LOOP_STATED_RAD_S;
        (0..RECEIVER_LOOP_NODES)
            .map(|i| {
                let omega = low * (high / low).powf((f64::from(i) + 0.5) / nodes);
                let lagged = flown.lagged_drift_m(omega, RECEIVER_LOOP_DAMPING, &times);
                let (east, north): (Vec<f64>, Vec<f64>) = self
                    .observed
                    .horizontal
                    .iter()
                    .zip(&lagged)
                    .map(|(&(_, e, n), &(east, north))| (e - east, n - north))
                    .unzip();
                (omega, -0.5 * track_minus_two_log([&east, &north]))
            })
            .collect()
    }

    /// The track's misfit, east and north, m, at each track bin on clock offset `clock_s`: the
    /// log's drift less the flight's. Where [`Problem::receiver_rad_s`] is set, the flight's drift
    /// is read through that filter and each axis's mean is taken off, standing in for the pad's
    /// offset, so the noise and correlation measured from it are those after the offset.
    fn track_misfit_m(&self, clock_s: f64, flown: &Simulated) -> Vec<(f64, f64)> {
        let observed = &self.observed.horizontal;
        match self.receiver_rad_s {
            None => observed
                .iter()
                .map(|&(t, e, n)| {
                    let (east, north) = flown.drift_m(t - clock_s);
                    (e - east, n - north)
                })
                .collect(),
            Some(omega) => {
                let times: Vec<f64> = observed.iter().map(|&(t, _, _)| t - clock_s).collect();
                let misfit: Vec<(f64, f64)> = observed
                    .iter()
                    .zip(flown.lagged_drift_m(omega, RECEIVER_LOOP_DAMPING, &times))
                    .map(|(&(_, e, n), (east, north))| (e - east, n - north))
                    .collect();
                // The pad's position is integrated out too, so each axis's mean is taken off.
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "a count of bins is far below 2^53"
                )]
                let count = misfit.len().max(1) as f64;
                let east = misfit.iter().map(|m| m.0).sum::<f64>() / count;
                let north = misfit.iter().map(|m| m.1).sum::<f64>() / count;
                misfit.iter().map(|m| (m.0 - east, m.1 - north)).collect()
            }
        }
    }
}

/// A receiver's navigation filter's natural frequency, rad/s, as the Bayesian option takes it:
/// log-uniform over this range, stated, not fitted. It reaches under and past the 0.47 to
/// 0.69 rad/s a type-2 filter on 3 m fixes took in docs/research/gnss-in-boost.md §4's
/// simulation. At its top a filter lags a 10 g boost by `a/ωn²`, 3.9 m; a receiver that follows
/// its own Doppler lags under 1 m (the same §), which would take about 10 rad/s, so such a
/// receiver lies outside the range.
const RECEIVER_LOOP_STATED_RAD_S: [f64; 2] = [0.2, 5.0];

/// The navigation filter's damping ratio, 1/√2, the usual choice for a second-order loop.
const RECEIVER_LOOP_DAMPING: f64 = std::f64::consts::FRAC_1_SQRT_2;

/// The midpoint rule's nodes over [`RECEIVER_LOOP_STATED_RAD_S`], in its logarithm: 3.4% apart.
const RECEIVER_LOOP_NODES: u32 = 96;

/// The track noise's correlation from one bin to the next, as the Bayesian option takes it:
/// uniform from 0 to this, stated, not fitted. It reaches past the least-squares fit's cap,
/// [`MAX_CORRELATION`], so the integral doesn't cut off a closer run than that cap allows; it
/// stops short of 1, where the noise would have no scale of its own.
const TRACK_CORRELATION_STATED: f64 = 0.99;

/// The midpoint rule's nodes over [`TRACK_CORRELATION_STATED`]'s range, `0.005` apart: a track of
/// `N` residuals settles the correlation to about `1/√N`, 0.08 on a short one of 160.
const TRACK_CORRELATION_NODES: u32 = 198;

/// The Gauss-Newton steps the Bayesian option takes toward its posterior's peak on a log with a
/// track, from the least-squares answer, each `1 +` [`FLOWN`] flights.
const PEAK_STEPS: usize = 3;

/// The least the Bayesian option's proposal takes a track bin's noise as, m. The proposal only
/// sets where the draws fall; the weights are the posterior's own, whatever it is.
const PROPOSAL_HORIZONTAL_FLOOR_M: f64 = 0.5;

/// `−2 ln` of the horizontal track's likelihood with its noise's scale, its correlation from one
/// bin to the next and a constant per axis (the pad's position) integrated out, up to a constant,
/// on its residuals in meters, east and north.
///
/// Each axis's residuals `e₁ … eₙ`, in the order of their bins, are taken as a first-order
/// autoregression of one standard deviation `σ` and one lag-one correlation `φ` for both axes. Its
/// exact Gaussian likelihood (J. D. Hamilton, *Time Series Analysis*, Princeton, 1994, §5.2) is
/// `−2 ln p(e | σ, φ) = N ln σ² − S ln(1 − φ²) + Q(φ)/σ² + const`, `N` residuals in `S` series,
/// `Q(φ) = Σ [(1 − φ²) e₁² + Σₖ₌₂ (eₖ − φ eₖ₋₁)²]` over the axes, a quadratic in `φ`. Jeffreys'
/// prior on a scale, `p(σ) ∝ 1/σ`, integrates `σ` out in closed form,
/// `∫ σ^(−N−1) exp(−Q/2σ²) dσ ∝ Q^(−N/2)` (G. E. P. Box and G. C. Tiao, *Bayesian Inference in
/// Statistical Analysis*, Addison-Wesley, 1973, §2.4). Each axis's constant `c`, under a flat
/// prior, enters `eₖ − c` linearly; integrating it out takes off its weighted least-squares value
/// and leaves a factor `xx^(−1/2)` and one residual fewer, with `xx = (1 − φ²) + (n − 1)(1 − φ)²`
/// the axis's weight on a constant. That leaves
/// `p(e | φ) ∝ (1 − φ²)^(S/2) Πₐ xxₐ^(−1/2) Q̃(φ)^(−(N−S)/2)`, `Q̃` the quadratic with each
/// axis's constant taken off. `φ` is uniform on `[0, 0.99]`
/// ([`TRACK_CORRELATION_STATED`]), integrated by the midpoint rule on
/// [`TRACK_CORRELATION_NODES`] nodes, in logarithms. No flight is flown for it.
///
/// So the track is weighed by its own scatter and run, where the least-squares fit states it: at
/// least [`FLOOR_HORIZONTAL_M`], widened for residuals that run on.
fn track_minus_two_log(series: [&[f64]; 2]) -> f64 {
    // Per axis: its count, `e₁`, `Σ e²`, `Σ eₖeₖ₋₁`, `Σ eₖ₋₁²` less `e₁²` (the values strictly
    // inside), `Σₖ₌₂ eₖ` and `Σₖ₌₂ eₖ₋₁`.
    let axes: Vec<[f64; 7]> = series
        .iter()
        .filter(|e| e.len() >= 2)
        .map(|e| {
            let m = e.len();
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of residuals is far below 2^53"
            )]
            let count = m as f64;
            [
                count,
                e[0],
                e.iter().map(|v| v * v).sum(),
                e.windows(2).map(|w| w[0] * w[1]).sum(),
                e[1..m - 1].iter().map(|v| v * v).sum(),
                e[1..].iter().sum(),
                e[..m - 1].iter().sum(),
            ]
        })
        .collect();
    if axes.is_empty() {
        return 0.0;
    }
    let total: f64 = axes.iter().map(|a| a[0]).sum();
    #[expect(clippy::cast_precision_loss, reason = "two axes at most")]
    let offsets = axes.len() as f64;
    let nodes = f64::from(TRACK_CORRELATION_NODES);
    let logs: Vec<f64> = (0..TRACK_CORRELATION_NODES)
        .map(|i| {
            let phi = (f64::from(i) + 0.5) * TRACK_CORRELATION_STATED / nodes;
            let keep = 1.0 - phi * phi;
            let (mut q, mut log_det) = (0.0, 0.0);
            for &[count, first, all, along, inner, later, earlier] in &axes {
                // The whitened residuals `w` and the whitened constant `x`: `w₁ = √(1−φ²) e₁`,
                // `wₖ = eₖ − φ eₖ₋₁`; `x₁ = √(1−φ²)`, `xₖ = 1 − φ`.
                let ww = all - 2.0 * phi * along + phi * phi * inner;
                let wx = keep * first + (1.0 - phi) * (later - phi * earlier);
                let xx = keep + (count - 1.0) * (1.0 - phi).powi(2);
                q += ww - wx * wx / xx;
                log_det += xx.ln();
            }
            // A NaN misfit stays NaN, so the draw fails rather than `max` making it the likeliest.
            if q.is_nan() {
                return f64::NAN;
            }
            0.5 * offsets * keep.ln()
                - 0.5 * log_det
                - 0.5 * (total - offsets) * q.max(f64::MIN_POSITIVE).ln()
        })
        .collect();
    let top = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = logs.iter().map(|l| (l - top).exp()).sum();
    -2.0 * (top + (sum / nodes).ln())
}

/// The heights' and the horizontal track's residuals' RMS at `x`, m; 0 for a track with none.
fn residual_noise(problem: &Problem<'_>, x: &[f64; UNKNOWNS], flown: &Simulated) -> (f64, f64) {
    let tau = x[FLOWN];
    let observed = problem.observed;
    let rms = |sum: f64, n: usize| {
        if n == 0 {
            0.0
        } else {
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of bins is far below 2^53"
            )]
            let n = n as f64;
            (sum / n).sqrt()
        }
    };
    let (mut heights, mut read) = (0.0, 0);
    for (&(t, y), used) in observed.heights.iter().zip(&problem.read) {
        if *used {
            heights += (y - problem.height_m(x, flown, t)).powi(2);
            read += 1;
        }
    }
    let horizontal: f64 = problem
        .track_misfit_m(tau, flown)
        .iter()
        .map(|(east, north)| east.powi(2) + north.powi(2))
        .sum();
    (
        rms(heights, read),
        rms(horizontal, 2 * observed.horizontal.len()),
    )
}

/// The most a residual's correlation from one bin to the next is taken as, so that a residual that
/// barely moves still leaves the errors finite.
const MAX_CORRELATION: f64 = 0.9;

/// How much a residual of lag-one correlation `rho` widens a variance: `(1 + ρ)/(1 − ρ)`, the
/// ratio of a series' length to its effective number of independent values under a first-order
/// autoregression (Wilks, *Statistical Methods in the Atmospheric Sciences*, 3rd ed., 2011,
/// eq. 5.12). None where it's negative, which would narrow it.
fn inflation(rho: f64) -> f64 {
    let rho = rho.clamp(0.0, MAX_CORRELATION);
    (1.0 + rho) / (1.0 - rho)
}

/// The lag-one autocorrelation of the read heights' residuals and of the track's, east and north
/// pooled, each about its mean; 0 where there are too few to tell.
fn residual_correlation(
    problem: &Problem<'_>,
    x: &[f64; UNKNOWNS],
    flown: &Simulated,
) -> (f64, f64) {
    let observed = problem.observed;
    let heights: Vec<f64> = observed
        .heights
        .iter()
        .zip(&problem.read)
        .filter(|(_, used)| **used)
        .map(|(&(t, y), _)| y - problem.height_m(x, flown, t))
        .collect();
    let (east, north): (Vec<f64>, Vec<f64>) =
        problem.track_misfit_m(x[FLOWN], flown).into_iter().unzip();
    (lag_one(&[&heights]), lag_one(&[&east, &north]))
}

/// The lag-one autocorrelation of `series`, pooled, each about its own mean.
fn lag_one(series: &[&[f64]]) -> f64 {
    let (mut along, mut whole) = (0.0, 0.0);
    for values in series {
        if values.len() < 3 {
            continue;
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "a count of bins is far below 2^53"
        )]
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        along += values
            .windows(2)
            .map(|w| (w[0] - mean) * (w[1] - mean))
            .sum::<f64>();
        whole += values.iter().map(|v| (v - mean).powi(2)).sum::<f64>();
    }
    if whole > 0.0 { along / whole } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::residuals::tests_support::simulated_of;

    /// A straight-line "flight": height `a t + b t²`, `a` from the drag scale, `b` from the impulse
    /// scale; drift from the wind: the fit's linear case, whose answer and errors are closed form.
    struct Line;

    impl Forward for Line {
        fn fly(&mut self, p: &Parameters, until_s: f64) -> Result<Simulated, String> {
            if p.drag_scale <= 0.0 {
                return Err("no drag".to_owned());
            }
            Ok(simulated_of(until_s, |t| {
                (
                    100.0 * p.drag_scale * t + 5.0 * p.impulse_scale * t * t,
                    p.wind_east_m_s * t,
                    p.wind_north_m_s * t,
                )
            }))
        }

        /// A wind that grows with height pulls the line down by its exponent.
        fn fly_sheared(
            &mut self,
            p: &Parameters,
            until_s: f64,
            exponent: f64,
        ) -> Result<Simulated, String> {
            if p.drag_scale <= 0.0 {
                return Err("no drag".to_owned());
            }
            Ok(simulated_of(until_s, |t| {
                (
                    (100.0 * p.drag_scale - 10.0 * exponent) * t + 5.0 * p.impulse_scale * t * t,
                    p.wind_east_m_s * t,
                    p.wind_north_m_s * t,
                )
            }))
        }
    }

    /// The rocket held as stated.
    fn wide_held() -> Stated {
        let mut stated = wide();
        stated[0].sd = 1e-6;
        stated[1].sd = 1e-6;
        stated
    }

    fn wide() -> Stated {
        [Prior {
            mean: 1.0,
            sd: 100.0,
        }; FLOWN]
    }

    fn observed(drag: f64, impulse: f64, east: f64, north: f64) -> Observations {
        let times = (0..80).map(|k| 0.25 * f64::from(k));
        Observations {
            kind: HeightKind::Geometric,
            heights: times
                .clone()
                .map(|t| (t, 100.0 * drag * t + 5.0 * impulse * t * t))
                .collect(),
            horizontal: times.map(|t| (t, east * t, north * t)).collect(),
            liftoff_s: 0.0,
            flown_s: 20.0,
            reads_transonic: true,
        }
    }

    /// Many draws, so that a test's points stand far from their Monte Carlo error.
    const MANY: Sampling = Sampling {
        pilot: 500,
        draws: 4_000,
        seed: 9,
    };

    #[test]
    fn on_a_line_the_bayesian_points_are_the_closed_forms() {
        // A linear model with normal priors and noise: the posterior is the normal the
        // least-squares fit's errors give, so its points are the answer ± 1.645 of them.
        let seen = observed(1.1, 0.9, 3.0, -2.0);
        let mut seen_noisy = seen.clone();
        let mut rng = hpr_core::random::SeededRng::seed_from_u64(3);
        for (_, h) in &mut seen_noisy.heights {
            *h += rng.standard_normal();
        }
        let mut stated = wide();
        stated[0].sd = 0.2;
        let got = bayesian(
            Some(&seen_noisy),
            &stated,
            Noise::Known {
                height_m: 1.0,
                horizontal_m: 1.0,
            },
            &mut Line,
            MANY,
        );
        let fit = &got.least_squares;
        let posterior = got.posterior.expect("a posterior");
        assert!(posterior.enough, "{}", posterior.effective_draws);
        let values = fit.parameters.to_array();
        for j in [0, 1, 3, 4] {
            let se = fit.standard_errors[j];
            let [low, mid, high] = posterior.quantiles[j];
            for (got, want) in [low, mid, high].into_iter().zip([
                values[j] - Z_90 * se,
                values[j],
                values[j] + Z_90 * se,
            ]) {
                assert!(
                    ((got - want) / se).abs() < 0.1,
                    "{j}: {got} against {want}, se {se}"
                );
            }
        }
    }

    #[test]
    fn on_heights_alone_the_wind_shear_is_drawn_as_stated() {
        // A sheared wind pulls the line's speed down by 10 α, and the wind moves no height: so
        // heights alone tell 100 d − 10 α, and with α uniform on 0 to 0.2 the drag's posterior on
        // them is its posterior with a track (where the shear isn't drawn, and the track speaks
        // to the wind alone) plus an independent uniform on 0 to 0.02. Each point then stands
        // between its place with a track and 0.02 above it; the median about the uniform's mean,
        // 0.01, above; the 5% point less than that and the 95% point more, as the sum is wider.
        // A barometer's and a receiver's noise, white, as a log's: on noise-free heights the
        // residuals are rounding, and how far their run widens the heights' noise is chance; a
        // noise-free track leaves the track's noise, integrated out, nothing to settle on.
        let mut tracked_seen = observed(1.0, 1.0, 0.0, 0.0);
        let mut rng = hpr_core::random::SeededRng::seed_from_u64(5);
        for (_, h) in &mut tracked_seen.heights {
            *h += rng.standard_normal();
        }
        for (_, east, north) in &mut tracked_seen.horizontal {
            *east += 2.0 * rng.standard_normal();
            *north += 2.0 * rng.standard_normal();
        }
        let tracked = bayesian(
            Some(&tracked_seen),
            &wide(),
            Noise::FromResiduals,
            &mut Line,
            MANY,
        );
        let with_track = tracked.posterior.expect("a posterior").quantiles[0];
        let mut seen = tracked_seen;
        seen.horizontal.clear();
        let alone = bayesian(Some(&seen), &wide(), Noise::FromResiduals, &mut Line, MANY);
        assert!(alone.least_squares.fitted);
        let alone = alone.posterior.expect("a posterior").quantiles[0];
        let shift: Vec<f64> = alone.iter().zip(&with_track).map(|(a, t)| a - t).collect();
        assert!(shift.iter().all(|d| (0.0..0.02).contains(d)), "{shift:?}");
        assert!((shift[1] - 0.01).abs() < 0.001, "{shift:?}");
        assert!(shift[0] < 0.009 && shift[2] > 0.011, "{shift:?}");
    }

    #[test]
    fn the_linear_sensor_unknowns_integrate_out_exactly() {
        // On a barometer's log, every sensor's unknown open: χ²(β) less its least is the quadratic
        // in β − β* that H gives, to rounding, for any β.
        let mut seen = observed(1.05, 0.95, 0.0, 0.0);
        seen.kind = HeightKind::Barometric;
        seen.horizontal.clear();
        for (k, (t, h)) in seen.heights.iter_mut().enumerate() {
            *h = 1.003 * *h + 2.0 + 0.02 * *t + if k % 2 == 0 { 0.4 } else { -0.4 };
        }
        let stated = wide();
        let problem = Problem {
            observed: &seen,
            stated: &stated,
            clock_mean_s: 0.0,
            clock_sd_s: CLOCK_OFFSET_SD_S,
            height_offset_sd_m: HEIGHT_OFFSET_SD_M,
            gain_sd: GAIN_SD,
            drift_sd_m_s: DRIFT_SD_M_S,
            port_mean: PORT_MEAN,
            port_sd: PORT_SD,
            height_sd_m: 0.5,
            horizontal_sd_m: FLOOR_HORIZONTAL_M,
            receiver_rad_s: None,
            track_unknowns: false,
            read: vec![true; seen.heights.len()],
        };
        let theta = [1.04, 0.96, 1.0, 0.0, 0.0, 1.0, 0.01];
        let mut parameters = [0.0; FLOWN];
        parameters.copy_from_slice(&theta[..FLOWN]);
        let flown = Line.fly(&Parameters::from_array(parameters), 30.0).unwrap();
        let got = problem.integrated(&theta, &flown).unwrap();
        let mut at_best = [0.0; UNKNOWNS];
        at_best[..DRAWN].copy_from_slice(&theta);
        at_best[DRAWN..].copy_from_slice(&got.best);
        assert!((chi2(&problem.residuals(&at_best, &flown)) - got.chi2_min).abs() < 1e-6);
        let mut rng = hpr_core::random::SeededRng::seed_from_u64(5);
        for _ in 0..5 {
            let offset: [f64; LINEAR] = [
                rng.standard_normal(),
                1e-3 * rng.standard_normal(),
                0.05 * rng.standard_normal(),
                0.01 * rng.standard_normal(),
            ];
            let mut x = at_best;
            for (v, o) in x[DRAWN..].iter_mut().zip(offset) {
                *v += o;
            }
            let rise = chi2(&problem.residuals(&x, &flown)) - got.chi2_min;
            // H by the columns, as the integration takes them.
            let r0 = problem.residuals(&at_best, &flown);
            let quadratic: f64 = {
                let mut total = 0.0;
                let columns: Vec<Vec<f64>> = (0..LINEAR)
                    .map(|k| {
                        let mut unit = at_best;
                        unit[DRAWN + k] += 1.0;
                        problem
                            .residuals(&unit, &flown)
                            .iter()
                            .zip(&r0)
                            .map(|(a, b)| a - b)
                            .collect()
                    })
                    .collect();
                for i in 0..LINEAR {
                    for j in 0..LINEAR {
                        let hij: f64 = columns[i].iter().zip(&columns[j]).map(|(a, b)| a * b).sum();
                        total += offset[i] * hij * offset[j];
                    }
                }
                total
            };
            assert!(
                (rise - quadratic).abs() < 1e-6 * quadratic.max(1.0),
                "{rise} against {quadratic}"
            );
        }
    }

    #[test]
    fn a_noise_free_line_comes_back_with_the_errors_its_design_matrix_gives() {
        let seen = observed(1.1, 0.9, 3.0, -2.0);
        let got = fit(
            Some(&seen),
            &wide(),
            Noise::Known {
                height_m: 1.0,
                horizontal_m: 1.0,
            },
            &mut Line,
        );
        assert!(got.fitted);
        assert!((got.parameters.drag_scale - 1.1).abs() < 1e-3, "{got:?}");
        assert!((got.parameters.impulse_scale - 0.9).abs() < 1e-3, "{got:?}");
        assert!((got.parameters.wind_east_m_s - 3.0).abs() < 1e-3, "{got:?}");
        assert!(
            (got.parameters.wind_north_m_s + 2.0).abs() < 1e-3,
            "{got:?}"
        );
        // Wind: x = w t with unit noise and the clock known: var w = 1 / Σ t², clear of the other
        // columns.
        let sum_t2: f64 = (0..80).map(|k| (0.25 * f64::from(k)).powi(2)).sum();
        let expected = sum_t2.recip().sqrt();
        assert!(
            (got.standard_errors[3] / expected - 1.0).abs() < 1e-2,
            "{} against {expected}",
            got.standard_errors[3]
        );
        // A parameter nothing speaks to keeps what was stated of it.
        assert!((got.parameters.mass_scale - 1.0).abs() < 1e-6);
        assert!((got.standard_errors[5] - 100.0).abs() < 1e-3);
    }

    #[test]
    fn what_is_stated_pulls_as_much_as_it_says() {
        // One unknown against one datum of equal weight meets halfway, by the normal equations.
        let mut stated = wide();
        stated[3] = Prior { mean: 0.0, sd: 0.1 };
        let seen = observed(1.0, 1.0, 3.0, 0.0);
        let got = fit(
            Some(&seen),
            &stated,
            Noise::Known {
                height_m: 1.0,
                horizontal_m: 1.0,
            },
            &mut Line,
        );
        let sum_t2: f64 = (0..80).map(|k| (0.25 * f64::from(k)).powi(2)).sum();
        let expected = 3.0 * sum_t2 / (sum_t2 + 100.0);
        assert!(
            (got.parameters.wind_east_m_s - expected).abs() < 1e-3,
            "{} against {expected}",
            got.parameters.wind_east_m_s
        );
    }

    #[test]
    fn a_log_that_says_nothing_or_a_flight_that_wont_fly_gives_what_was_stated() {
        let stated = wide();
        let nothing = fit(None, &stated, Noise::FromResiduals, &mut Line);
        assert!(!nothing.fitted);
        assert_eq!(nothing.parameters.drag_scale, 1.0);
        assert_eq!(nothing.standard_errors[0], 100.0);
        let mut grounded = stated;
        grounded[0].mean = -1.0;
        let seen = observed(1.0, 1.0, 0.0, 0.0);
        let refused = fit(Some(&seen), &grounded, Noise::FromResiduals, &mut Line);
        assert!(!refused.fitted);
        assert_eq!(refused.parameters.drag_scale, -1.0);
    }

    /// A barometer's gain is the fit's to find, with the rocket held as stated. A GNSS height has
    /// none, so the same samples leave it the curve a clock offset and a drift can't take up.
    #[test]
    fn a_barometers_gain_goes_to_the_sensor_not_the_rocket() {
        let held = {
            let mut stated = wide();
            stated[0].sd = 1e-6;
            stated[1].sd = 1e-6;
            stated
        };
        let mut seen = observed(1.0, 1.0, 0.0, 0.0);
        seen.horizontal.clear();
        for sample in &mut seen.heights {
            sample.1 *= 1.004;
        }
        let height_only = fit(Some(&seen), &held, Noise::FromResiduals, &mut Line);
        seen.kind = HeightKind::Barometric;
        let barometer = fit(Some(&seen), &held, Noise::FromResiduals, &mut Line);
        let (rms_height, rms_barometer) = (height_only.rms_m.unwrap(), barometer.rms_m.unwrap());
        assert!(
            rms_barometer < 0.7 * rms_height,
            "{rms_barometer} {rms_height}"
        );
        assert!((barometer.parameters.drag_scale - 1.0).abs() < 1e-4);
    }

    /// A barometer's log of a flight to about 610 m, every 0.05 s, its clock jumping by `jump_s`
    /// from 3 s before apogee.
    fn jumping_log(jump_s: f64) -> FlightLog {
        let pad_s = 2.0;
        let height = |tau: f64| {
            if tau <= 0.0 {
                0.0
            } else if tau < 2.0 {
                25.0 * tau * tau
            } else if tau < 12.2 {
                100.0 + 100.0 * (tau - 2.0) - 4.9 * (tau - 2.0).powi(2)
            } else {
                (610.2 - 20.0 * (tau - 12.2)).max(0.0)
            }
        };
        let time_s: Vec<f64> = (0..1_400).map(|i| f64::from(i) * 0.05).collect();
        let altitude_m = time_s.iter().map(|&t| height(t - pad_s)).collect();
        let time_s = time_s
            .into_iter()
            .map(|t| if t - pad_s > 9.2 { t + jump_s } else { t })
            .collect();
        FlightLog {
            format: hpr_flightdata::log::LogFormat::GenericTable,
            logger: "a test's".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: hpr_flightdata::log::Stated::default(),
            time_s,
            altitude_m,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: Vec::new(),
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// A clock that jumps between liftoff and apogee by more than any flight's climb says nothing,
    /// so it can't ask for bins beyond memory.
    #[test]
    fn a_clock_that_jumps_past_any_flight_says_nothing() {
        let plain = jumping_log(0.0);
        assert!(observations(&plain, &readings::read(&plain)).is_some());
        let jumped = jumping_log(5_000.0);
        let read = readings::read(&jumped);
        assert!(matches!(read.apogee, Reading::Read(_)), "{:?}", read.apogee);
        assert!(observations(&jumped, &read).is_none());
    }

    /// A clock that jumps can't ask for bins beyond memory; the window's bound is what stops it.
    #[test]
    fn a_window_past_any_flight_says_nothing() {
        const { assert!(MAX_WINDOW_S < 1e4) };
        let held = binned([(0.0, 1.0), (999.0, 2.0)].into_iter(), 0.0, MAX_WINDOW_S);
        assert_eq!(held.len(), 2);
    }

    /// Bins that can't be a flight's, flat where the simulation climbs, say nothing whatever their
    /// standard errors come to: a fit that sits far from its log isn't told by it.
    #[test]
    fn a_log_far_from_any_flight_says_nothing() {
        let mut seen = observed(1.0, 1.0, 0.0, 0.0);
        seen.horizontal.clear();
        for sample in &mut seen.heights {
            sample.1 = 300.0 * (3.0 * sample.0).sin();
        }
        let got = fit(Some(&seen), &wide_held(), Noise::FromResiduals, &mut Line);
        assert!(!got.fitted);
        let share = got.misfit_share.unwrap();
        assert!(share > MISFIT_SHARE && share < 2.0, "{share}");
        assert!(got.flown > 0);
        assert_eq!(got.rms_m, None);
    }

    /// A barometer's bins at or past Mach 0.8 aren't read; with too few left, it says nothing.
    #[test]
    fn a_barometer_with_too_few_bins_below_mach_0_8_says_nothing() {
        struct Fast;
        impl Forward for Fast {
            fn fly(&mut self, p: &Parameters, until_s: f64) -> Result<Simulated, String> {
                let _ = p;
                Ok(crate::residuals::tests_support::simulated_at_mach(
                    until_s,
                    2.3,
                    |t| (800.0 * t, 0.0, 0.0),
                ))
            }

            fn fly_sheared(
                &mut self,
                p: &Parameters,
                until_s: f64,
                _: f64,
            ) -> Result<Simulated, String> {
                self.fly(p, until_s)
            }
        }
        let mut seen = observed(1.0, 1.0, 0.0, 0.0);
        seen.kind = HeightKind::Barometric;
        seen.reads_transonic = false;
        seen.horizontal.clear();
        let got = fit(Some(&seen), &wide(), Noise::FromResiduals, &mut Fast);
        assert!(!got.fitted);
        assert_eq!(got.flown, 1);
    }

    /// On heights alone, a wind that grows with height widens each error by the move it makes, to
    /// first order, over √3: the line's sheared climb is 10 α m/s slower, so its drag moves by
    /// `0.1 α`, and its calm one moves nothing. With a track, nothing is added.
    #[test]
    fn a_wind_heights_alone_cant_tell_widens_the_errors() {
        struct Unsheared(Line);
        impl Forward for Unsheared {
            fn fly(&mut self, p: &Parameters, until_s: f64) -> Result<Simulated, String> {
                self.0.fly(p, until_s)
            }

            fn fly_sheared(
                &mut self,
                p: &Parameters,
                until_s: f64,
                _: f64,
            ) -> Result<Simulated, String> {
                self.0.fly(p, until_s)
            }
        }
        let mut seen = observed(1.0, 1.0, 0.0, 0.0);
        seen.horizontal.clear();
        let sheared = fit(Some(&seen), &wide(), Noise::FromResiduals, &mut Line);
        let flat = fit(
            Some(&seen),
            &wide(),
            Noise::FromResiduals,
            &mut Unsheared(Line),
        );
        assert!(sheared.fitted && flat.fitted);
        assert_eq!(sheared.parameters, flat.parameters);
        let moved = 0.1 * SHEAR_EXPONENT;
        let added = sheared.standard_errors[0].powi(2) - flat.standard_errors[0].powi(2);
        assert!(
            (added - moved * moved / 3.0).abs() < 1e-9,
            "{added} against {}",
            moved * moved / 3.0
        );
        let tracked = observed(1.0, 1.0, 0.0, 0.0);
        let with_track = fit(Some(&tracked), &wide(), Noise::FromResiduals, &mut Line);
        let without = fit(
            Some(&tracked),
            &wide(),
            Noise::FromResiduals,
            &mut Unsheared(Line),
        );
        assert_eq!(with_track.standard_errors, without.standard_errors);
    }

    /// A residual that runs on from bin to bin widens a variance by `(1 + ρ)/(1 − ρ)`, capped;
    /// one that alternates leaves it as it is.
    #[test]
    fn a_residual_that_runs_on_tells_less() {
        let alternating = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        assert!(lag_one(&[&alternating]) < 0.0);
        assert_eq!(inflation(lag_one(&[&alternating])), 1.0);
        let slow: Vec<f64> = (0..200).map(|k| (f64::from(k) * 0.01).sin()).collect();
        assert!(lag_one(&[&slow]) > MAX_CORRELATION);
        assert!((inflation(1.0) - 19.0).abs() < 1e-12);
        assert!((inflation(0.5) - 3.0).abs() < 1e-12);
        assert_eq!(lag_one(&[&[1.0, 2.0]]), 0.0);
    }

    #[test]
    fn a_clock_offset_is_found_and_the_noise_comes_from_the_residuals() {
        let mut seen = observed(1.0, 1.0, 0.0, 0.0);
        let mut state = 5_u64;
        for sample in &mut seen.heights {
            sample.0 += 1.5;
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            #[expect(
                clippy::cast_precision_loss,
                reason = "noise for a test, any rounding will do"
            )]
            let u = (state >> 11) as f64 / (1_u64 << 53) as f64 - 0.5;
            sample.1 += 40.0 * u;
        }
        seen.horizontal.clear();
        seen.liftoff_s = 1.0;
        let got = fit(Some(&seen), &wide(), Noise::FromResiduals, &mut Line);
        assert!(
            (got.clock_offset_s - 1.5).abs() < 0.3,
            "{}",
            got.clock_offset_s
        );
        // Uniform noise of width 40 has an RMS of 40/√12 = 11.5.
        let rms = got.rms_m.unwrap();
        assert!((9.0..13.0).contains(&rms), "{rms}");
    }

    #[test]
    fn bins_take_a_median_so_one_garbled_sample_doesnt_reach_them() {
        let samples = [(0.01, 1.0), (0.1, 900.0), (0.2, 2.0), (0.3, 7.0)];
        let bins = binned(samples.into_iter(), 0.0, 0.5);
        assert_eq!(bins, vec![(0.125, 2.0), (0.375, 7.0)]);
    }

    #[test]
    fn cholesky_solves_and_refuses_what_isnt_positive() {
        let a = [[4.0, 2.0], [2.0, 3.0]];
        let l = cholesky(&a).unwrap();
        let x = solve(&l, &[6.0, 5.0]);
        assert!((x[0] - 1.0).abs() < 1e-12 && (x[1] - 1.0).abs() < 1e-12);
        assert!(cholesky(&[[1.0, 2.0], [2.0, 1.0]]).is_none());
    }

    /// `−2 ln` of the track's likelihood by dense matrices, apart from the whitening: on each
    /// axis the AR(1) covariance `Σᵢⱼ = φ^|i−j| / (1 − φ²)` of unit innovations, the offset
    /// integrated as generalized least squares on a column of ones, `σ` by Jeffreys' prior, then
    /// `φ` by the same midpoint rule.
    #[expect(
        clippy::needless_range_loop,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        reason = "a dense matrix written out by its indices, of a few rows"
    )]
    fn dense_track_minus_two_log(series: [&[f64]; 2]) -> f64 {
        let nodes = f64::from(TRACK_CORRELATION_NODES);
        let total: usize = series.iter().map(|e| e.len()).sum();
        let logs: Vec<f64> = (0..TRACK_CORRELATION_NODES)
            .map(|i| {
                let phi = (f64::from(i) + 0.5) * TRACK_CORRELATION_STATED / nodes;
                let (mut q, mut log_terms) = (0.0, 0.0);
                for e in series {
                    let m = e.len();
                    // Σ⁻¹ by Gauss–Jordan on [Σ | I].
                    let mut a = vec![vec![0.0; 2 * m]; m];
                    for r in 0..m {
                        for c in 0..m {
                            a[r][c] = phi.powi((r as i32 - c as i32).abs()) / (1.0 - phi * phi);
                        }
                        a[r][m + r] = 1.0;
                    }
                    let mut log_det = 0.0;
                    for c in 0..m {
                        let pivot = a[c][c];
                        log_det += pivot.ln();
                        for v in a[c].iter_mut() {
                            *v /= pivot;
                        }
                        for r in 0..m {
                            if r != c {
                                let f = a[r][c];
                                let row = a[c].clone();
                                for (v, w) in a[r].iter_mut().zip(&row) {
                                    *v -= f * w;
                                }
                            }
                        }
                    }
                    let inv = |r: usize, c: usize| a[r][m + c];
                    let (mut ee, mut oe, mut oo) = (0.0, 0.0, 0.0);
                    for r in 0..m {
                        for c in 0..m {
                            ee += e[r] * inv(r, c) * e[c];
                            oe += inv(r, c) * e[c];
                            oo += inv(r, c);
                        }
                    }
                    q += ee - oe * oe / oo;
                    log_terms += -0.5 * log_det - 0.5 * oo.ln();
                }
                log_terms - 0.5 * (total as f64 - 2.0) * q.ln()
            })
            .collect();
        let top = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = logs.iter().map(|l| (l - top).exp()).sum();
        -2.0 * (top + (sum / nodes).ln())
    }

    /// The track's likelihood with its noise, run and pad integrated out is the dense
    /// computation's, takes no notice of a constant added to an axis (the pad), and moves by
    /// `(N − 2) ln k²` when every residual is `k` times as large.
    #[test]
    fn the_tracks_noise_and_pad_integrate_out_as_the_dense_computation_does() {
        let east = [1.3, -0.4, 2.2, 0.9, -1.7, 0.5, 1.1];
        let north = [-0.6, 0.2, 0.8, 1.9, 1.4, -0.3];
        let got = track_minus_two_log([&east, &north]);
        let dense = dense_track_minus_two_log([&east, &north]);
        assert!(
            (got - dense).abs() < 1e-9 * dense.abs().max(1.0),
            "{got} and {dense}"
        );
        let moved: Vec<f64> = east.iter().map(|v| v + 40.0).collect();
        let shifted = track_minus_two_log([&moved, &north]);
        assert!((shifted - got).abs() < 1e-9, "{shifted} and {got}");
        let k: f64 = 3.0;
        let (big_east, big_north): (Vec<f64>, Vec<f64>) = (
            east.iter().map(|v| k * v).collect(),
            north.iter().map(|v| k * v).collect(),
        );
        let scaled = track_minus_two_log([&big_east, &big_north]);
        let expected = got + (13.0 - 2.0) * (k * k).ln();
        assert!((scaled - expected).abs() < 1e-9, "{scaled} and {expected}");
        // A residual that runs on is likelier read with its run than a scattered one of the same
        // spread, so the integral favors it: the order of its values matters.
        let run = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let mut scattered = run;
        scattered.swap(1, 5);
        scattered.swap(2, 6);
        assert!(track_minus_two_log([&run, &[]]) < track_minus_two_log([&scattered, &[]]));
    }

    /// A NaN residual leaves the track's likelihood NaN, so its draw fails, rather than the floor
    /// on the misfit reading it as a perfect fit, the likeliest draw of all.
    #[test]
    fn a_nan_residual_leaves_the_tracks_likelihood_nan() {
        let east = [1.3, -0.4, f64::NAN, 0.9, -1.7];
        let north = [-0.6, 0.2, 0.8, 1.9];
        assert!(track_minus_two_log([&east, &north]).is_nan());
        assert!(track_minus_two_log([&north, &east]).is_nan());
    }
}
