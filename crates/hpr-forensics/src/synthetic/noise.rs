//! The noise terms a synthetic logger adds to a flight's truth, each a function of its own so that
//! a test can hold it to its stated parameters.
//!
//! Sources, by term (the project's research notes, each citing its primary sources):
//!
//! - **Correlated barometer noise:** an Ornstein–Uhlenbeck process, the continuous form of an
//!   AR(1) series, of 0.5 m at 50 Hz with lag-one correlation 0.5, the noise measured at rest in
//!   public logs; a white-noise model holds a smoother's 90% interval about 70% of the time instead
//!   (`docs/research/altimeter-sensor-noise.md` §3, §6; `altimeter-smoothing-methods.md` §6).
//! - **Per-flight offset, gain and drift:** the MS5607's ±1.0 mbar after the pad's zero, the Bosch
//!   parts' relative accuracy of 0.03–0.08 hPa per 100 hPa, and a drift of 1–3 Pa over 30–300 s
//!   (`altimeter-sensor-noise.md` §2–3, §6).
//! - **The port's transonic error:** the pressure coefficient measured on a 3-caliber
//!   ogive-cylinder in Langley's 8-foot transonic tunnel, by calibers behind the nose's shoulder
//!   and Mach number (Czarnecki and Monta, NASA TN D-3516, 1966, Fig. 5, as
//!   `altimeter-mach-error.md` §1 reads it).
//! - **The ejection charge's pulse:** the shape on the public Pnut log Debrief ships, as
//!   `hpr_flightdata`'s closed-form synthetic log has it.
//! - **GNSS lag:** a second-order tracking loop, whose error under a constant acceleration `a` is
//!   `a/ωn²`, with dropouts at ignition and burnout (`gnss-in-boost.md` §3–4).

use hpr_core::random::SeededRng;

/// A number drawn uniformly from `[low, high)`.
pub fn between(rng: &mut SeededRng, low: f64, high: f64) -> f64 {
    low + (high - low) * rng.uniform()
}

/// An Ornstein–Uhlenbeck process sampled at `times_s`: stationary standard deviation `sigma`, time
/// constant `tau_s`, so that two samples `Δt` apart correlate by `exp(−Δt/τ)`.
///
/// The first sample is drawn from the stationary distribution, and each later one by the exact
/// discretization `x_k = φ x_{k−1} + σ √(1 − φ²) ε_k`, `φ = exp(−Δt_k/τ)`, which holds for any
/// spacing, so a logger that changes its rate in flight needs nothing more.
pub fn ornstein_uhlenbeck(
    rng: &mut SeededRng,
    times_s: &[f64],
    sigma: f64,
    tau_s: f64,
) -> Vec<f64> {
    let mut out = Vec::with_capacity(times_s.len());
    let mut previous: Option<(f64, f64)> = None;
    for &t in times_s {
        let x = match previous {
            None => sigma * rng.standard_normal(),
            Some((t0, x0)) => {
                let phi = (-(t - t0) / tau_s).exp();
                phi * x0 + sigma * (1.0 - phi * phi).sqrt() * rng.standard_normal()
            }
        };
        out.push(x);
        previous = Some((t, x));
    }
    out
}

/// The time constant of an AR(1) series with lag-one correlation `phi` at `rate_hz`, s.
pub fn tau_for(phi: f64, rate_hz: f64) -> f64 {
    -1.0 / (rate_hz * phi.ln())
}

/// White noise of standard deviation `sigma`, one draw a sample.
pub fn white(rng: &mut SeededRng, count: usize, sigma: f64) -> Vec<f64> {
    (0..count).map(|_| sigma * rng.standard_normal()).collect()
}

/// `value` rounded to the nearest multiple of `step`, as a logger writes it; `step` 0 keeps it.
pub fn quantize(value: f64, step: f64) -> f64 {
    if step > 0.0 {
        (value / step).round() * step
    } else {
        value
    }
}

/// The calibers behind the nose's shoulder at which the tunnel measured a port's pressure
/// coefficient.
const PORT_CALIBERS: [f64; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 6.0];
/// The Mach numbers it measured at.
const PORT_MACH: [f64; 5] = [0.7, 0.9, 1.0, 1.1, 1.2];
/// The pressure coefficient, by calibers (rows) and Mach number (columns). `NaN` marks the cells
/// past the tunnel wall's reflected shock at Mach 1.1, which the test calls artifacts; they are
/// read across in Mach number instead.
const PORT_CP: [[f64; 5]; 6] = [
    [-0.07, -0.08, -0.20, -0.14, -0.11],
    [-0.02, -0.02, -0.08, -0.07, -0.04],
    [-0.01, -0.01, -0.06, -0.05, -0.01],
    [-0.01, -0.01, 0.03, -0.02, -0.01],
    [-0.01, -0.01, 0.02, f64::NAN, -0.02],
    [0.00, -0.01, 0.00, f64::NAN, -0.01],
];

/// The index of the interval of `grid` holding `x`, and the fraction across it, `x` held to the
/// grid's ends.
fn bracket(grid: &[f64], x: f64) -> (usize, f64) {
    let last = grid.len() - 1;
    if x <= grid[0] {
        return (0, 0.0);
    }
    if x >= grid[last] {
        return (last - 1, 1.0);
    }
    let i = grid
        .iter()
        .rposition(|&g| g <= x)
        .unwrap_or(0)
        .min(last - 1);
    (i, (x - grid[i]) / (grid[i + 1] - grid[i]))
}

/// The tunnel's pressure coefficient at `calibers` behind the shoulder and `mach`, read
/// bilinearly; below Mach 0.7 and above 1.2 the ends hold, and past the measured calibers the
/// last row does.
pub fn port_pressure_coefficient(calibers: f64, mach: f64) -> f64 {
    let (row, fr) = bracket(&PORT_CALIBERS, calibers);
    let (col, fc) = bracket(&PORT_MACH, mach);
    let at = |r: usize, c: usize| -> f64 {
        let cp = PORT_CP[r][c];
        if cp.is_nan() {
            // Across in Mach number, between the measured columns either side.
            0.5 * (PORT_CP[r][c - 1] + PORT_CP[r][c + 1])
        } else {
            cp
        }
    };
    let lower = at(row, col) + fc * (at(row, col + 1) - at(row, col));
    let upper = at(row + 1, col) + fc * (at(row + 1, col + 1) - at(row + 1, col));
    lower + fr * (upper - lower)
}

/// The pressure a port reads above the true static pressure, Pa: `Cp · q`.
pub fn port_error_pa(calibers: f64, mach: f64, dynamic_pressure_pa: f64) -> f64 {
    port_pressure_coefficient(calibers, mach) * dynamic_pressure_pa
}

/// The ejection pulse's shape, m of indicated altitude at each 0.05 s step from the charge, as on
/// the public Pnut log Debrief ships (`hpr_flightdata`'s synthetic log copies it).
pub const PULSE_M: [f64; 7] = [-8.0, 1.0, 16.0, 15.0, 2.0, -3.0, -3.0];
/// The pulse's sample interval, s.
pub const PULSE_STEP_S: f64 = 0.05;

/// The pulse's indicated altitude `since_s` after the charge, m, at `scale` times the Pnut's,
/// read linearly between its samples and zero outside them.
pub fn pulse_m(since_s: f64, scale: f64) -> f64 {
    if !(0.0..=PULSE_STEP_S * (PULSE_M.len() - 1) as f64).contains(&since_s) {
        return 0.0;
    }
    let x = since_s / PULSE_STEP_S;
    let i = (x.floor() as usize).min(PULSE_M.len() - 2);
    let f = x - i as f64;
    scale * (PULSE_M[i] + f * (PULSE_M[i + 1] - PULSE_M[i]))
}

/// A second-order tracking loop, the shape of a GNSS receiver's navigation filter: its output
/// `x` follows the truth `p` by `ẍ = ωn² (p − x) + 2ζωn (ṗ − ẋ)`, so under a constant
/// acceleration `a` it settles `a/ωn²` behind.
///
/// It integrates the error `e = p − x`, whose equation `ë = p̈ − ωn² e − 2ζωn ė` takes the
/// truth's acceleration as its input: the error stays small however far and fast the rocket
/// goes, so the step adds no lag of its own, and a constant acceleration's fixed point is
/// `a/ωn²` exactly.
#[derive(Debug, Clone, Copy)]
pub struct Lag {
    omega_rad_s: f64,
    zeta: f64,
    error: f64,
    error_rate: f64,
    output: f64,
    rate: Option<f64>,
}

/// The loop's damping ratio, 1/√2: the usual choice for a second-order loop.
pub const LAG_DAMPING: f64 = std::f64::consts::FRAC_1_SQRT_2;

impl Lag {
    /// A loop of natural frequency `omega_rad_s`, settled on `p`.
    pub fn new(omega_rad_s: f64, p: f64) -> Self {
        Self {
            omega_rad_s,
            zeta: LAG_DAMPING,
            error: 0.0,
            error_rate: 0.0,
            output: p,
            rate: None,
        }
    }

    /// Its output.
    pub fn output(&self) -> f64 {
        self.output
    }

    /// Steps the loop `dt_s` to the truth `p`, moving at `p_rate` (its acceleration taken from
    /// the change in `p_rate` over the step), and returns its output. Semi-implicit Euler on the
    /// error, accurate while `ωn·dt` is small (0.01 at 1 rad/s and 0.01 s).
    pub fn step(&mut self, dt_s: f64, p: f64, p_rate: f64) -> f64 {
        if dt_s > 0.0 {
            let accel = self.rate.map_or(0.0, |previous| (p_rate - previous) / dt_s);
            let w = self.omega_rad_s;
            let e_dot_dot = accel - w * w * self.error - 2.0 * self.zeta * w * self.error_rate;
            self.error_rate += e_dot_dot * dt_s;
            self.error += self.error_rate * dt_s;
        }
        self.rate = Some(p_rate);
        self.output = p - self.error;
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 7_202;

    fn mean(x: &[f64]) -> f64 {
        x.iter().sum::<f64>() / x.len() as f64
    }

    fn sd(x: &[f64]) -> f64 {
        let m = mean(x);
        (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
    }

    fn lag_one(x: &[f64]) -> f64 {
        let m = mean(x);
        let num: f64 = x.windows(2).map(|w| (w[0] - m) * (w[1] - m)).sum();
        let den: f64 = x.iter().map(|v| (v - m) * (v - m)).sum();
        num / den
    }

    /// Whether a series holds an AR(1)'s stated spread and lag-one correlation, within five
    /// times their sampling errors at this length.
    fn holds_ar1(x: &[f64], sigma: f64, phi: f64) -> bool {
        let n = x.len() as f64;
        // The sample standard deviation of an AR(1) series: σ √((1 + φ²)/(2n(1 − φ²))) for its
        // variance's spread, halved; and the lag-one estimate's √((1 − φ²)/n).
        let sd_error = sigma * ((1.0 + phi * phi) / (2.0 * n * (1.0 - phi * phi))).sqrt();
        let phi_error = ((1.0 - phi * phi) / n).sqrt();
        (sd(x) - sigma).abs() < 5.0 * sd_error && (lag_one(x) - phi).abs() < 5.0 * phi_error
    }

    #[test]
    fn the_correlated_noise_holds_its_spread_and_correlation_and_white_noise_fails() {
        let rate = 50.0;
        let times: Vec<f64> = (0..200_000).map(|k| k as f64 / rate).collect();
        let tau = tau_for(0.5, rate);
        let mut rng = SeededRng::for_stream(SEED, &[1]);
        let x = ornstein_uhlenbeck(&mut rng, &times, 0.5, tau);
        assert!(holds_ar1(&x, 0.5, 0.5), "sd {} phi {}", sd(&x), lag_one(&x));
        // Mutations: white noise of the same spread, and the right correlation at 1.2 times the
        // spread, each fail.
        let mut rng = SeededRng::for_stream(SEED, &[1]);
        let w = white(&mut rng, times.len(), 0.5);
        assert!(!holds_ar1(&w, 0.5, 0.5));
        let wide: Vec<f64> = x.iter().map(|v| 1.2 * v).collect();
        assert!(!holds_ar1(&wide, 0.5, 0.5));
    }

    #[test]
    fn the_correlation_follows_the_spacing_when_the_rate_changes() {
        // 100 Hz then 10 Hz, as an AltOS logger records: the lag-one correlation at each rate is
        // exp(−Δt/τ).
        let tau = tau_for(0.5, 50.0);
        let fast: Vec<f64> = (0..100_000).map(|k| k as f64 / 100.0).collect();
        let slow: Vec<f64> = (0..100_000).map(|k| 1_000.0 + k as f64 / 10.0).collect();
        let times: Vec<f64> = fast.iter().chain(&slow).copied().collect();
        let mut rng = SeededRng::for_stream(SEED, &[2]);
        let x = ornstein_uhlenbeck(&mut rng, &times, 1.0, tau);
        assert!(holds_ar1(&x[..100_000], 1.0, (-0.01 / tau).exp()));
        assert!(holds_ar1(&x[100_000..], 1.0, (-0.1 / tau).exp()));
    }

    #[test]
    fn quantizing_rounds_to_the_step_and_keeps_a_zero_step() {
        assert_eq!(quantize(101_325.4, 1.0), 101_325.0);
        assert_eq!(quantize(101_326.0, 10.0), 101_330.0);
        assert_eq!(quantize(0.3048 * 2.6, 0.3048), 0.3048 * 3.0);
        assert_eq!(quantize(1.234, 0.0), 1.234);
    }

    #[test]
    fn the_port_reads_the_tunnels_pressure_coefficients_at_its_measured_points() {
        for (r, &cal) in PORT_CALIBERS.iter().enumerate() {
            for (c, &mach) in PORT_MACH.iter().enumerate() {
                let cp = PORT_CP[r][c];
                let read = port_pressure_coefficient(cal, mach);
                if cp.is_nan() {
                    let across = 0.5 * (PORT_CP[r][c - 1] + PORT_CP[r][c + 1]);
                    assert!((read - across).abs() < 1e-15);
                } else {
                    assert!((read - cp).abs() < 1e-15, "{cal} cal, Mach {mach}");
                }
            }
        }
        // Between points, bilinear; past the ends, held.
        let mid = port_pressure_coefficient(1.5, 0.95);
        let expect = 0.25 * (-0.02 - 0.08 - 0.01 - 0.06);
        assert!((mid - expect).abs() < 1e-15);
        assert_eq!(port_pressure_coefficient(1.0, 0.3), -0.02);
        assert_eq!(port_pressure_coefficient(1.0, 2.0), -0.04);
        // A port one caliber back reads high through Mach 1 (Cp −0.08 at q = 40 kPa: −3.2 kPa of
        // pressure, so the altitude reads high); the sign flipped would read low.
        let error = port_error_pa(1.0, 1.0, 40_000.0);
        assert!((error + 3_200.0).abs() < 1e-9);
        assert!(error < 0.0);
    }

    #[test]
    fn the_pulse_has_the_pnut_shape_scaled_and_nothing_outside_it() {
        for (k, &h) in PULSE_M.iter().enumerate() {
            let t = k as f64 * PULSE_STEP_S;
            assert!((pulse_m(t, 1.0) - h).abs() < 1e-12);
            assert!((pulse_m(t, 1.5) - 1.5 * h).abs() < 1e-12);
        }
        assert_eq!(pulse_m(-0.01, 1.0), 0.0);
        assert_eq!(pulse_m(0.31, 1.0), 0.0);
        assert!((pulse_m(0.125, 1.0) - 15.5).abs() < 1e-12);
    }

    #[test]
    fn the_gnss_loop_settles_a_over_omega_squared_behind_a_constant_acceleration() {
        let a = 100.0;
        let dt = 0.01;
        for omega in [0.25, 0.5, 1.0] {
            let mut lag = Lag::new(omega, 0.0);
            lag.step(0.0, 0.0, 0.0);
            let mut out = 0.0;
            let mut p = 0.0;
            for k in 1..=(60.0 / omega / dt) as usize {
                let t = k as f64 * dt;
                p = 0.5 * a * t * t;
                out = lag.step(dt, p, a * t);
            }
            let behind = p - out;
            assert!(
                (behind - a / (omega * omega)).abs() < 1e-6 * a / (omega * omega),
                "ωn {omega}: {behind}"
            );
        }
        // A loop that doesn't lag (its output the truth) is 0 behind: the check tells them apart.
        assert!((0.0_f64 - a / 0.25).abs() > 0.01 * a / 0.0625);
    }

    #[test]
    fn a_draw_between_bounds_stays_within_and_fills_them() {
        let mut rng = SeededRng::for_stream(SEED, &[3]);
        let x: Vec<f64> = (0..10_000)
            .map(|_| between(&mut rng, -100.0, 100.0))
            .collect();
        assert!(x.iter().all(|v| (-100.0..100.0).contains(v)));
        let lo = x.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(lo < -99.0 && hi > 99.0);
        // A uniform's standard deviation is the width over √12.
        assert!((sd(&x) - 200.0 / 12_f64.sqrt()).abs() < 2.0);
    }
}
