//! The Bayesian option's sampler (M7.3c1): a posterior's quantiles by importance sampling.
//!
//! **What it does.** Given where a posterior peaks and its curvature there (the least-squares
//! fit's answer and covariance, the Laplace approximation), it draws points around the peak,
//! weighs each by how much more or less likely the posterior makes it than the draw did, and reads
//! each unknown's 5%, 50% and 95% points off the weighted draws. Where the posterior is Gaussian
//! this gives what the covariance already says; where it bends or has a heavy side, as a wind told
//! by heights alone does, it gives the posterior's own shape.
//!
//! **The method.** Self-normalized importance sampling (J. Geweke, "Bayesian inference in
//! econometric models using Monte Carlo integration", *Econometrica* 57(6), 1989, pp. 1317–1339):
//! draws `xᵢ` from a proposal `q`, weights `wᵢ = p(xᵢ)/q(xᵢ)` with `p` the posterior known up to a
//! constant, and each expectation as `Σ wᵢ f(xᵢ) / Σ wᵢ`. The proposal is a multivariate Student-t
//! with ν = [`DEGREES_OF_FREEDOM`], centered on the peak, its scale the Laplace covariance: its
//! tails are heavier than a Gaussian posterior's, so the weights stay bounded where the posterior is
//! near Gaussian (Geweke on a proposal's tails). A draw is
//!
//! x = m + L z / √(u/ν),  z ~ N(0, I),  u ~ χ²_ν,  L Lᵀ = Σ,
//!
//! and up to a constant `ln q(x) = −(ν + N)/2 · ln(1 + δ²/ν)` with `δ² = |z|²/(u/ν)`.
//!
//! **How far to trust it.** The weighted draws count for `(Σ wᵢ)² / Σ wᵢ²` independent ones, Kish's
//! effective sample size (L. Kish, *Survey Sampling*, Wiley, 1965), a result for a weighted mean
//! read here for quantiles as an approximation. A posterior far from its Laplace approximation
//! leaves few effective draws; the quantiles still come back, and [`Posterior::enough`] says
//! they're too few to read. A draw whose posterior can't be had (a flight that won't fly) gets no weight and is
//! counted in [`Posterior::failed`]. Tested on posteriors whose quantiles are known in closed form
//! or by quadrature; it is not yet tested against the synthetic flights (M7.3c2).

use hpr_core::random::SeededRng;

use crate::fit::cholesky;

/// The proposal's degrees of freedom: 4. Any Student-t has tails heavier than a Gaussian
/// posterior's, which keeps the weights bounded there (Geweke); a small ν widens them further for a
/// posterior with a heavy side, and ν = 4 still has a finite variance. A judgment, not a
/// measurement.
pub const DEGREES_OF_FREEDOM: usize = 4;

/// The fewest effective draws for a quantile to be read: 100. At 100, the 5% point's Monte Carlo
/// error is about `√(0.05 · 0.95 / 100) / φ(1.645) ≈ 0.21` posterior standard deviations, past
/// which an interval's ends say more of the draws than of the flight. A judgment, not a measurement.
pub const MIN_EFFECTIVE_DRAWS: f64 = 100.0;

/// The probabilities [`Posterior::quantiles`] reads: the 90% interval's ends and the median.
pub const PROBABILITIES: [f64; 3] = [0.05, 0.5, 0.95];

/// What the weighted draws say of each unknown.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Posterior<const N: usize> {
    /// Each unknown's 5%, 50% and 95% points ([`PROBABILITIES`]), in the unknowns' order.
    #[serde(with = "rows")]
    pub quantiles: [[f64; 3]; N],
    /// Kish's effective sample size, `(Σ wᵢ)² / Σ wᵢ²`.
    pub effective_draws: f64,
    /// The draws made, failed ones included.
    pub draws: usize,
    /// The draws whose posterior couldn't be had; each has no weight.
    pub failed: usize,
    /// Whether [`Posterior::effective_draws`] reaches [`MIN_EFFECTIVE_DRAWS`].
    pub enough: bool,
}

/// `[[f64; 3]; N]` as a list of rows, since serde's arrays stop at 32 and take no const length.
mod rows {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer, const N: usize>(
        rows: &[[f64; 3]; N],
        s: S,
    ) -> Result<S::Ok, S::Error> {
        rows.as_slice().serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>, const N: usize>(
        d: D,
    ) -> Result<[[f64; 3]; N], D::Error> {
        let rows = Vec::<[f64; 3]>::deserialize(d)?;
        let count = rows.len();
        rows.try_into()
            .map_err(|_| serde::de::Error::invalid_length(count, &"one row for each unknown"))
    }
}

/// The posterior whose log density, up to a constant, is `log_target`, by `draws` weighted draws
/// around `peak` with the Laplace covariance `covariance`.
///
/// `log_target` gives `None` where the posterior can't be had; that draw gets no weight. `None`
/// overall if `covariance` isn't positive definite, `draws` is 0, or no draw has a finite weight.
/// The same `seed` gives the same answer, bit for bit.
pub fn sample<const N: usize>(
    peak: &[f64; N],
    covariance: &[[f64; N]; N],
    draws: usize,
    seed: u64,
    mut log_target: impl FnMut(&[f64; N]) -> Option<f64>,
) -> Option<Posterior<N>> {
    let l = cholesky(covariance)?;
    let nu = DEGREES_OF_FREEDOM as f64;
    // Cast: N is a count of unknowns, far below 2⁵³.
    let dimensions = N as f64;
    let mut rng = SeededRng::seed_from_u64(seed);
    let mut points: Vec<([f64; N], f64)> = Vec::with_capacity(draws);
    let mut failed = 0;
    for _ in 0..draws {
        let z: [f64; N] = std::array::from_fn(|_| rng.standard_normal());
        let u: f64 = (0..DEGREES_OF_FREEDOM)
            .map(|_| rng.standard_normal().powi(2))
            .sum();
        let stretch = 1.0 / (u / nu).sqrt();
        let x: [f64; N] = std::array::from_fn(|i| {
            peak[i] + stretch * (0..=i).map(|k| l[i][k] * z[k]).sum::<f64>()
        });
        let distance2 = z.iter().map(|v| v * v).sum::<f64>() * stretch * stretch;
        let log_proposal = -0.5 * (nu + dimensions) * (distance2 / nu).ln_1p();
        match log_target(&x) {
            Some(log_p) if (log_p - log_proposal).is_finite() => {
                points.push((x, log_p - log_proposal))
            }
            _ => failed += 1,
        }
    }
    let top = points
        .iter()
        .map(|(_, w)| *w)
        .fold(f64::NEG_INFINITY, f64::max);
    if !top.is_finite() {
        return None;
    }
    let weights: Vec<f64> = points.iter().map(|(_, w)| (w - top).exp()).collect();
    let total: f64 = weights.iter().sum();
    let squares: f64 = weights.iter().map(|w| w * w).sum();
    let effective_draws = total * total / squares;
    let quantiles = std::array::from_fn(|j| {
        let mut column: Vec<(f64, f64)> = points
            .iter()
            .zip(&weights)
            .map(|((x, _), w)| (x[j], *w))
            .collect();
        column.sort_by(|a, b| a.0.total_cmp(&b.0));
        PROBABILITIES.map(|p| weighted_quantile(&column, total, p))
    });
    Some(Posterior {
        quantiles,
        effective_draws,
        draws,
        failed,
        enough: effective_draws >= MIN_EFFECTIVE_DRAWS,
    })
}

/// The least value of `sorted` (value, weight) pairs whose cumulative weight reaches `p` of `total`:
/// the weighted empirical distribution's lower `p` quantile.
fn weighted_quantile(sorted: &[(f64, f64)], total: f64, p: f64) -> f64 {
    let goal = p * total;
    let mut running = 0.0;
    for &(value, weight) in sorted {
        running += weight;
        if running >= goal {
            return value;
        }
    }
    // Rounding can leave the running sum a hair under the total; the largest value is the answer.
    sorted.last().map_or(f64::NAN, |&(value, _)| value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hpr_flightdata::readings::interval::Z_90;

    /// A Gaussian posterior with mean `mean` and standard deviations `sd`, uncorrelated.
    fn gaussian<const N: usize>(
        mean: [f64; N],
        sd: [f64; N],
    ) -> impl FnMut(&[f64; N]) -> Option<f64> {
        move |x| {
            Some(
                -0.5 * (0..N)
                    .map(|i| ((x[i] - mean[i]) / sd[i]).powi(2))
                    .sum::<f64>(),
            )
        }
    }

    #[test]
    fn a_gaussian_posterior_comes_back_at_its_closed_form() {
        // The proposal is off center by 0.3 sd and 1.5 times too wide, as a Laplace approximation
        // that missed would be. The 5% point's Monte Carlo error at n effective draws is about
        // √(0.05·0.95/n)/φ(1.645) sd: 0.024 sd at n = 8,000, so 0.06 sd is two and a half of them.
        let mean = [1.0, -2.0];
        let sd = [0.5, 2.0];
        let posterior = sample(
            &[1.15, -1.4],
            &[[0.5625, 0.0], [0.0, 9.0]],
            20_000,
            7,
            gaussian(mean, sd),
        )
        .unwrap();
        assert!(
            posterior.enough && posterior.effective_draws > 8_000.0,
            "{}",
            posterior.effective_draws
        );
        assert_eq!(posterior.failed, 0);
        for i in 0..2 {
            let expected = [mean[i] - Z_90 * sd[i], mean[i], mean[i] + Z_90 * sd[i]];
            for (k, (got, want)) in posterior.quantiles[i].iter().zip(expected).enumerate() {
                let off = (got - want) / sd[i];
                assert!(off.abs() < 0.06, "unknown {i}, point {k}: {off} sd");
            }
        }
    }

    #[test]
    fn a_curved_posterior_comes_back_at_its_quadrature() {
        // y = b x² + e, x and e standard normal: y's distribution is skewed, which a Laplace
        // approximation at the peak (0, 0), identity covariance, can't show. Its CDF is
        // P(Y ≤ t) = ∫ φ(x) Φ(t − b x²) dx, by Simpson's rule on [−8, 8].
        let b = 0.5;
        let cdf = |t: f64| {
            let n = 4_000;
            let h = 16.0 / n as f64;
            (0..=n)
                .map(|k| {
                    let x = -8.0 + h * k as f64;
                    let weight = if k == 0 || k == n {
                        1.0
                    } else if k % 2 == 1 {
                        4.0
                    } else {
                        2.0
                    };
                    weight * (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
                        * normal_cdf(t - b * x * x)
                })
                .sum::<f64>()
                * h
                / 3.0
        };
        let quantile = |p: f64| {
            let (mut lo, mut hi) = (-10.0, 30.0);
            for _ in 0..80 {
                let mid = 0.5 * (lo + hi);
                if cdf(mid) < p { lo = mid } else { hi = mid }
            }
            0.5 * (lo + hi)
        };
        let target =
            move |v: &[f64; 2]| Some(-0.5 * v[0] * v[0] - 0.5 * (v[1] - b * v[0] * v[0]).powi(2));
        let posterior = sample(&[0.0, 0.0], &[[1.0, 0.0], [0.0, 1.0]], 40_000, 11, target).unwrap();
        assert!(posterior.enough);
        // y's standard deviation is √(1 + 2b²) ≈ 1.22; 0.08 is about four Monte Carlo errors.
        for (k, p) in PROBABILITIES.into_iter().enumerate() {
            let want = quantile(p);
            assert!(
                (posterior.quantiles[1][k] - want).abs() < 0.08,
                "p {p}: {} vs {want}",
                posterior.quantiles[1][k]
            );
        }
        // What a Laplace approximation can't show: the median stands above the peak's 0, and the
        // 95% point farther above it than the 5% point is below.
        let [low, mid, high] = posterior.quantiles[1];
        assert!(
            mid > 0.3 && high - mid > 1.1 * (mid - low),
            "{low} {mid} {high}"
        );
        // The fit's page quotes these three, to two places ("a median of 0.42 and an interval of
        // −1.36 to 2.53"); a change here needs the page changed too.
        let rounded = [low, mid, high].map(|v| (v * 100.0).round() / 100.0);
        assert_eq!(rounded, [-1.36, 0.42, 2.53]);
    }

    /// Φ by Simpson's rule on φ from −9, where Φ is below 10⁻¹⁸.
    fn normal_cdf(t: f64) -> f64 {
        if t < -9.0 {
            return 0.0;
        }
        let n = 2_000;
        let h = (t + 9.0) / n as f64;
        (0..=n)
            .map(|k| {
                let x = -9.0 + h * k as f64;
                let weight = if k == 0 || k == n {
                    1.0
                } else if k % 2 == 1 {
                    4.0
                } else {
                    2.0
                };
                weight * (-0.5 * x * x).exp()
            })
            .sum::<f64>()
            * h
            / 3.0
            / (2.0 * std::f64::consts::PI).sqrt()
    }

    #[test]
    fn a_draw_that_cant_be_had_has_no_weight_and_is_counted() {
        // Where the first unknown passes 1, the posterior can't be had, as a flight that won't fly.
        let mut inner = gaussian([0.0, 0.0], [1.0, 1.0]);
        let posterior = sample(&[0.0, 0.0], &[[1.0, 0.0], [0.0, 1.0]], 5_000, 3, |x| {
            if x[0] > 1.0 { None } else { inner(x) }
        })
        .unwrap();
        assert!(posterior.failed > 500, "{}", posterior.failed);
        assert!(posterior.quantiles[0][2] <= 1.0);
        assert_eq!(posterior.draws, 5_000);
    }

    #[test]
    fn a_proposal_far_from_the_posterior_says_its_draws_are_too_few() {
        // The posterior sits 6 proposal sd away and 20 times narrower: few draws land on it.
        let posterior = sample(&[0.0], &[[1.0]], 2_000, 5, gaussian([6.0], [0.05])).unwrap();
        assert!(!posterior.enough, "{}", posterior.effective_draws);
    }

    #[test]
    fn the_same_seed_gives_the_same_answer_and_nothing_gives_none() {
        let run = |seed| {
            sample(
                &[0.0, 1.0],
                &[[1.0, 0.3], [0.3, 2.0]],
                500,
                seed,
                gaussian([0.1, 0.9], [0.8, 1.2]),
            )
        };
        let bits = |p: Option<Posterior<2>>| {
            p.map(|p| (p.quantiles.map(|row| row.map(f64::to_bits)), p.effective_draws.to_bits()))
        };
        assert_eq!(bits(run(1)), bits(run(1)));
        assert_ne!(bits(run(1)), bits(run(2)));
        assert!(sample(&[0.0], &[[-1.0]], 10, 1, gaussian([0.0], [1.0])).is_none());
        assert!(sample(&[0.0], &[[1.0]], 0, 1, gaussian([0.0], [1.0])).is_none());
        assert!(sample(&[0.0], &[[1.0]], 10, 1, |_: &[f64; 1]| None).is_none());
    }
}
