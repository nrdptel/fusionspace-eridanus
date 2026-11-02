//! The Bayesian option's sampler: a posterior's quantiles by importance sampling.
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
//! they're too few to read. A draw whose posterior can't be had (a flight that won't fly) gets no
//! weight and is counted in [`Posterior::failed`]. Tested on posteriors whose quantiles are known in closed form
//! or by quadrature; it is not yet run on a log or tested against the synthetic flights.

use hpr_core::random::SeededRng;

use crate::fit::cholesky;

/// The proposal's degrees of freedom: 4. Any Student-t has tails heavier than a Gaussian
/// posterior's, which keeps the weights bounded there (Geweke); a small ν widens them further for a
/// posterior with a heavy side, and ν = 4 still has a finite variance. A judgment, not a
/// measurement.
pub const DEGREES_OF_FREEDOM: usize = 4;

/// The fewest effective draws for a quantile to be read: 100. At 100, the 5% point's Monte Carlo
/// error is about `√(0.05 · 0.95 / 100) / φ(1.645)`, 0.22 posterior standard deviations rounded
/// up, past which an interval's ends say more of the draws than of the flight. A judgment, not a
/// measurement.
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
    /// The draws whose posterior couldn't be had, or whose weight came out infinite or NaN; each
    /// has no weight. A draw where the posterior is zero (`log_target` gives −∞) has no weight
    /// either, but isn't a failure.
    pub failed: usize,
    /// Whether [`Posterior::effective_draws`] reaches [`MIN_EFFECTIVE_DRAWS`]. It doesn't weigh
    /// [`Posterior::failed`]: many failed draws cut the posterior off where they fell, and
    /// `failed / draws` says how many.
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
/// `log_target` gives `None` where the posterior can't be had; that draw gets no weight. Only the
/// lower triangle of `covariance` is read. `None` overall if `peak` isn't finite, `covariance`
/// isn't positive definite, `draws` is 0, or no draw has a finite weight.
/// The same `seed` and a `log_target` that gives the same answer for the same point give the same
/// answer, bit for bit, on one platform (the draws go through the platform's `ln`, `ln_1p`,
/// `sqrt` and `exp`).
pub fn sample<const N: usize>(
    peak: &[f64; N],
    covariance: &[[f64; N]; N],
    draws: usize,
    seed: u64,
    log_target: impl FnMut(&[f64; N]) -> Option<f64>,
) -> Option<Posterior<N>> {
    sample_weighted(peak, covariance, draws, seed, log_target, true)
}

/// [`sample`], with the proposal's density in each weight, or left out of it for the tests' probe
/// that the weights need it.
fn sample_weighted<const N: usize>(
    peak: &[f64; N],
    covariance: &[[f64; N]; N],
    draws: usize,
    seed: u64,
    mut log_target: impl FnMut(&[f64; N]) -> Option<f64>,
    proposal_in_weight: bool,
) -> Option<Posterior<N>> {
    if !peak.iter().all(|v| v.is_finite()) {
        return None;
    }
    let l = cholesky(covariance)?;
    let nu = DEGREES_OF_FREEDOM as f64;
    // Cast: N is a count of unknowns, far below 2⁵³.
    let dimensions = N as f64;
    let mut rng = SeededRng::seed_from_u64(seed);
    // Grown as draws succeed, not reserved up front, so a huge `draws` can't fail an allocation
    // before the first flight: memory is about `draws · 8 (N + 4)` bytes, the points with their
    // weights twice over and one column to sort.
    let mut points: Vec<([f64; N], f64)> = Vec::new();
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
        let log_proposal = if proposal_in_weight {
            -0.5 * (nu + dimensions) * (distance2 / nu).ln_1p()
        } else {
            0.0
        };
        match log_target(&x) {
            // A posterior of zero there: no weight, and nothing failed.
            Some(log_p) if log_p == f64::NEG_INFINITY => {}
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

    const GAUSSIAN_MEAN: [f64; 2] = [1.0, -2.0];
    const GAUSSIAN_SD: [f64; 2] = [0.5, 2.0];
    /// The Gaussian case's correlation, a strong one: a draw that lost it (`Lᵀ z` for `L z`)
    /// misses the posterior's shape, which a diagonal case can't see.
    const GAUSSIAN_CORRELATION: f64 = 0.9;

    /// A correlated Gaussian posterior drawn from a proposal off center by 0.3 sd in each unknown
    /// and 1.5 times too wide, as a Laplace approximation that missed would be.
    fn gaussian_case(proposal_in_weight: bool) -> Posterior<2> {
        let ([m0, m1], [s0, s1], r) = (GAUSSIAN_MEAN, GAUSSIAN_SD, GAUSSIAN_CORRELATION);
        let target = move |x: &[f64; 2]| {
            let (a, b) = ((x[0] - m0) / s0, (x[1] - m1) / s1);
            Some(-0.5 * (a * a - 2.0 * r * a * b + b * b) / (1.0 - r * r))
        };
        let cross = 2.25 * r * s0 * s1;
        sample_weighted(
            &[1.15, -1.4],
            &[[2.25 * s0 * s0, cross], [cross, 2.25 * s1 * s1]],
            20_000,
            7,
            target,
            proposal_in_weight,
        )
        .unwrap()
    }

    /// The farthest of the Gaussian case's points from its closed form, in standard deviations.
    fn gaussian_worst(posterior: &Posterior<2>) -> f64 {
        (0..2)
            .flat_map(|i| {
                let (mean, sd) = (GAUSSIAN_MEAN[i], GAUSSIAN_SD[i]);
                let expected = [mean - Z_90 * sd, mean, mean + Z_90 * sd];
                posterior.quantiles[i]
                    .into_iter()
                    .zip(expected)
                    .map(move |(got, want)| ((got - want) / sd).abs())
            })
            .fold(0.0, f64::max)
    }

    #[test]
    fn a_gaussian_posterior_comes_back_at_its_closed_form() {
        // The 5% point's Monte Carlo error at n effective draws is about √(0.05·0.95/n)/φ(1.645)
        // sd: 0.024 sd at n = 8,000. That formula, a weighted mean's, is optimistic for a
        // quantile; over seeds 100 to 299 no case misses 0.06 sd, the worst 0.052 sd.
        let posterior = gaussian_case(true);
        assert!(
            posterior.enough && posterior.effective_draws > 8_000.0,
            "{}",
            posterior.effective_draws
        );
        assert_eq!(posterior.failed, 0);
        let worst = gaussian_worst(&posterior);
        assert!(worst < 0.06, "{worst} sd");
    }

    /// The curved case's b: y = b x² + e.
    const B: f64 = 0.5;

    /// y = b x² + e, x and e standard normal: y's distribution is skewed, which a Laplace
    /// approximation at the peak (0, 0), identity covariance, can't show.
    fn curved_case(proposal_in_weight: bool) -> Posterior<2> {
        let target =
            move |v: &[f64; 2]| Some(-0.5 * v[0] * v[0] - 0.5 * (v[1] - B * v[0] * v[0]).powi(2));
        sample_weighted(
            &[0.0, 0.0],
            &[[1.0, 0.0], [0.0, 1.0]],
            160_000,
            11,
            target,
            proposal_in_weight,
        )
        .unwrap()
    }

    /// The curved case's y at [`PROBABILITIES`], by quadrature, worked out once for every test:
    /// its CDF is P(Y ≤ t) = ∫ φ(x) Φ(t − b x²) dx, by Simpson's rule on [−8, 8], bisected.
    fn curved_truth() -> [f64; 3] {
        static TRUTH: std::sync::OnceLock<[f64; 3]> = std::sync::OnceLock::new();
        *TRUTH.get_or_init(|| {
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
                            * normal_cdf(t - B * x * x)
                    })
                    .sum::<f64>()
                    * h
                    / 3.0
            };
            PROBABILITIES.map(|p| {
                let (mut lo, mut hi) = (-10.0, 30.0);
                for _ in 0..80 {
                    let mid = 0.5 * (lo + hi);
                    if cdf(mid) < p { lo = mid } else { hi = mid }
                }
                0.5 * (lo + hi)
            })
        })
    }

    /// The farthest of the curved case's points for y from its quadrature, in y's units.
    fn curved_worst(posterior: &Posterior<2>) -> f64 {
        posterior.quantiles[1]
            .into_iter()
            .zip(curved_truth())
            .map(|(got, want)| (got - want).abs())
            .fold(0.0, f64::max)
    }

    #[test]
    fn a_curved_posterior_comes_back_at_its_quadrature() {
        let posterior = curved_case(true);
        // y's standard deviation is √(1 + 2b²) ≈ 1.2247, so 0.08 is 0.065 of it. y's 95% point
        // sits in its long upper tail, where the density is low and the point's error is largest:
        // over seeds 100 to 199 its error has a standard deviation of 0.018 at these draws, so 0.08
        // is over four of those, and no seed takes any of the three points past it.
        assert!(
            posterior.enough && posterior.effective_draws > 80_000.0,
            "{}",
            posterior.effective_draws
        );
        let worst = curved_worst(&posterior);
        assert!(
            worst < 0.08,
            "{:?} vs {:?}",
            posterior.quantiles[1],
            curved_truth()
        );
        // What a Laplace approximation can't show: the median stands above the peak's 0, and the
        // 95% point farther above it than the 5% point is below.
        let [low, mid, high] = posterior.quantiles[1];
        assert!(
            mid > 0.3 && high - mid > 1.1 * (mid - low),
            "{low} {mid} {high}"
        );
        // The fit's page quotes these three, to two places ("a median of 0.43 and an interval of
        // −1.35 to 2.57"); a change here needs the page changed too.
        let rounded = [low, mid, high].map(|v| (v * 100.0).round() / 100.0);
        assert_eq!(rounded, [-1.35, 0.43, 2.57]);
    }

    #[test]
    fn both_closed_form_tests_fail_with_the_proposal_left_out_of_the_weights() {
        // Weighing by the posterior alone reads the product of proposal and posterior, not the
        // posterior: the probe that the two tests above can see a wrong weight. (The probe that
        // they see a draw that lost its correlation, `Lᵀ z` for `L z`, was run by hand: 0.93 sd
        // off with 69 effective draws, so the Gaussian test fails it twice over.)
        let gaussian = gaussian_worst(&gaussian_case(false));
        assert!(gaussian >= 0.06, "{gaussian} sd");
        let curved = curved_worst(&curved_case(false));
        assert!(curved >= 0.08, "{curved}");
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
        assert_eq!(posterior.draws, 5_000);
        // The first unknown is then a normal cut at 1: its 95% point is Φ⁻¹(0.95 Φ(1)) ≈ 0.839.
        // Over seeds 100 to 299 this point's error has a standard deviation of 0.011 and is at most
        // 0.031, so 0.04 is over three and a half standard deviations, and no seed misses it.
        let (mut lo, mut hi) = (-5.0, 1.0);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if normal_cdf(mid) < 0.95 * normal_cdf(1.0) {
                lo = mid
            } else {
                hi = mid
            }
        }
        let truth = 0.5 * (lo + hi);
        assert!(
            (posterior.quantiles[0][2] - truth).abs() < 0.04,
            "{} vs {truth}",
            posterior.quantiles[0][2]
        );
    }

    #[test]
    fn a_draw_where_the_posterior_is_zero_has_no_weight_and_isnt_a_failure() {
        let mut inner = gaussian([0.0, 0.0], [1.0, 1.0]);
        let posterior = sample(&[0.0, 0.0], &[[1.0, 0.0], [0.0, 1.0]], 5_000, 3, |x| {
            if x[0] > 1.0 {
                Some(f64::NEG_INFINITY)
            } else {
                inner(x)
            }
        })
        .unwrap();
        assert_eq!(posterior.failed, 0);
        assert!(posterior.quantiles[0][2] <= 1.0);
    }

    #[test]
    fn a_posterior_reads_back_as_written_and_refuses_the_wrong_count() {
        let posterior = sample(
            &[0.0, 1.0],
            &[[1.0, 0.3], [0.3, 2.0]],
            500,
            1,
            gaussian([0.1, 0.9], [0.8, 1.2]),
        )
        .unwrap();
        let json = serde_json::to_string(&posterior).unwrap();
        assert_eq!(
            serde_json::from_str::<Posterior<2>>(&json).unwrap(),
            posterior
        );
        assert!(serde_json::from_str::<Posterior<3>>(&json).is_err());
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
        // Every field, the floats by their bits.
        let bits = |p: Option<Posterior<2>>| {
            p.map(|p| {
                (
                    p.quantiles.map(|row| row.map(f64::to_bits)),
                    p.effective_draws.to_bits(),
                    p.draws,
                    p.failed,
                    p.enough,
                )
            })
        };
        assert_eq!(bits(run(1)), bits(run(1)));
        assert_ne!(bits(run(1)), bits(run(2)));
        assert!(sample(&[0.0], &[[-1.0]], 10, 1, gaussian([0.0], [1.0])).is_none());
        assert!(sample(&[f64::NAN], &[[1.0]], 10, 1, gaussian([0.0], [1.0])).is_none());
        assert!(sample(&[0.0], &[[1.0]], 0, 1, gaussian([0.0], [1.0])).is_none());
        assert!(sample(&[0.0], &[[1.0]], 10, 1, |_: &[f64; 1]| None).is_none());
    }
}
