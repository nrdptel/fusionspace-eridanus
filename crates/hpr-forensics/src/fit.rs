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
//! the height offset `h₀`, and for a barometer its gain `g`, drift `d` and port coefficient `c` on
//! the port's dynamic-pressure height `p` (each sized as [`crate::residuals`] sizes it). Left out,
//! these went into the drag scale. The second sum is the track `r`, where a log holds one. The
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
//! **How far to trust it.** The errors are linearized about the solution, so they say nothing of a
//! second solution elsewhere. The 90% intervals hold on synthetic flights ([`crate::synthetic::fits`]
//! and the committed report), where the fit knows the design, rail, site and day's air: on a real
//! flight its model's errors go into the scales with nothing to say so. A scale is the factor that
//! best fits one flight, not the rocket's drag at every Mach.
//!
//! [m7-3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md

use hpr_flightdata::log::{FlightLog, Quantity};
use hpr_flightdata::readings::{self, Reading, Readings};
use serde::{Deserialize, Serialize};

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
const MAX_ITERATIONS: usize = 12;
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Noise {
    /// From the fit's residuals, floored at [`FLOOR_M`] and [`FLOOR_HORIZONTAL_M`].
    FromResiduals,
    /// As given, every sensor's unknown still open: the same fit as a log's, to a flight with no
    /// noise to estimate it from.
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
#[derive(Debug, Clone, PartialEq)]
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
    let pad_m = read.pad_altitude_m?;
    let (times, heights) = readings::height_track(log)?;
    let apogee_s = apogee.time_s;
    let liftoff_s = match &read.liftoff {
        Reading::Read(liftoff) => liftoff.time_s,
        // No liftoff to read, as a log from launch has: its first sample stands for it.
        Reading::Withheld(_) => times.iter().copied().find(|t| t.is_finite())?,
    };
    if apogee_s <= liftoff_s {
        return None;
    }
    let kind = HeightKind::of(apogee.source);
    let from_s = liftoff_s - 1.0;
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
    let (Some(latitude), Some(longitude)) = (
        log.channels_of(Quantity::Latitude).next(),
        log.channels_of(Quantity::Longitude).next(),
    ) else {
        return Vec::new();
    };
    if latitude.time_s.len() != longitude.time_s.len() {
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
        .filter_map(|&(t, e)| north.iter().find(|(u, _)| *u == t).map(|&(_, n)| (t, e, n)))
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
}

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
    /// Flights flown.
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
/// What is stated of a barometer's port's pressure coefficient: within
/// [`readings::PORT_PRESSURE_COEFFICIENT`] of nothing, as a uniform draw's standard deviation.
const PORT_SD: f64 = readings::PORT_PRESSURE_COEFFICIENT / 1.732_050_807_568_877_2;
/// What is stated of an offset known exactly, in its unit: small enough to hold it, large enough
/// for the normal equations to stay well conditioned.
const KNOWN_SD: f64 = 1e-4;

/// The fit's residual, weighted: heights, then the horizontal track's east and north, then each
/// flown parameter from what is stated of it, then the two offsets.
struct Problem<'a> {
    observed: &'a Observations,
    stated: &'a Stated,
    clock_mean_s: f64,
    clock_sd_s: f64,
    height_offset_sd_m: f64,
    gain_sd: f64,
    drift_sd_m_s: f64,
    port_sd: f64,
    height_sd_m: f64,
    horizontal_sd_m: f64,
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
        for &(t, e, n) in &self.observed.horizontal {
            let (east, north) = flown.drift_m(t - tau);
            r.push((e - east) / self.horizontal_sd_m);
            r.push((n - north) / self.horizontal_sd_m);
        }
        for (value, prior) in x.iter().zip(self.stated) {
            r.push((value - prior.mean) / prior.sd);
        }
        r.push((tau - self.clock_mean_s) / self.clock_sd_s);
        r.push(h0 / self.height_offset_sd_m);
        r.push(x[FLOWN + 2] / self.gain_sd);
        r.push(x[FLOWN + 3] / self.drift_sd_m_s);
        r.push(x[FLOWN + 4] / self.port_sd);
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
fn cholesky<const N: usize>(a: &[[f64; N]; N]) -> Option<[[f64; N]; N]> {
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
    let Some(observed) = observed else {
        return Fit::of_nothing(stated);
    };
    // Long enough for a rocket that flies well past the stated flight's top.
    let until_s = 1.6 * observed.flown_s + 5.0;
    let mut flown = 0_u64;
    let mut fly = |x: &[f64; UNKNOWNS]| {
        flown += 1;
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
    let Some(mut at_x) = fly(&x) else {
        return Fit::of_nothing(stated);
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
        port_sd: if observed.kind == HeightKind::Barometric {
            PORT_SD
        } else {
            KNOWN_SD
        },
        height_sd_m,
        horizontal_sd_m,
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
            port_sd: KNOWN_SD,
            height_sd_m: height_m,
            horizontal_sd_m: horizontal_m,
            read: Vec::new(),
        },
    };
    x[FLOWN] = problem.clock_mean_s;
    problem.read = observed
        .heights
        .iter()
        .map(|&(t, _)| {
            observed.reads_transonic || at_x.mach(t - x[FLOWN]) < readings::TRANSONIC_MACH
        })
        .collect();
    let mut information = [[0.0; UNKNOWNS]; UNKNOWNS];
    let passes = if matches!(noise, Noise::FromResiduals) {
        2
    } else {
        1
    };
    for pass in 0..passes {
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
                break;
            };
            let drop = cost - tried_cost;
            at_x = flight;
            r = tried;
            cost = tried_cost;
            if drop < CONVERGED_CHI2 {
                break;
            }
        }
        if pass + 1 < passes {
            let (height, horizontal) = residual_noise(&problem, &x, &at_x);
            problem.height_sd_m = height.max(FLOOR_M);
            problem.horizontal_sd_m = horizontal.max(FLOOR_HORIZONTAL_M);
        }
    }
    // The Jacobian at the solution, with the noise as it ends.
    let r = problem.residuals(&x, &at_x);
    let mut columns: Vec<Vec<f64>> = Vec::with_capacity(UNKNOWNS);
    for (j, step) in STEPS.iter().enumerate() {
        let mut ahead = x;
        ahead[j] += step;
        let Some(flight) = fly(&ahead) else {
            return Fit::of_nothing(stated);
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
        return Fit::of_nothing(stated);
    };
    let mut standard_errors = [0.0; FLOWN];
    for (j, se) in standard_errors.iter_mut().enumerate() {
        let mut unit = [0.0; UNKNOWNS];
        unit[j] = 1.0;
        *se = solve(&l, &unit)[j].sqrt();
    }
    let mut parameters = [0.0; FLOWN];
    parameters.copy_from_slice(&x[..FLOWN]);
    let (rms_m, _) = residual_noise(&problem, &x, &at_x);
    Fit {
        parameters: Parameters::from_array(parameters),
        standard_errors,
        clock_offset_s: x[FLOWN],
        height_offset_m: x[FLOWN + 1],
        rms_m: Some(rms_m),
        height_noise_m: problem.height_sd_m,
        horizontal_noise_m: problem.horizontal_sd_m,
        fitted: true,
        flown,
    }
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
    let horizontal: f64 = observed
        .horizontal
        .iter()
        .map(|&(t, e, n)| {
            let (east, north) = flown.drift_m(t - tau);
            (e - east).powi(2) + (n - north).powi(2)
        })
        .sum();
    (
        rms(heights, read),
        rms(horizontal, 2 * observed.horizontal.len()),
    )
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
}
