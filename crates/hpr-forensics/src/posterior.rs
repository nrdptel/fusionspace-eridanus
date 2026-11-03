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
//! weight and is counted in [`Posterior::failed`]. Tested on posteriors whose quantiles are known
//! in closed form or by quadrature.
//!
//! **In two blocks.** A posterior whose unknowns follow some of them along a curved ridge, as a
//! fit's drag and clock follow the wind on a log of height alone, leaves one Student-t about its
//! peak few effective draws. [`sample_in_two`] draws the marked unknowns first, then the others
//! about where the caller says they center given those, and widens its scales where a pilot's
//! draws reach further ([ADR-319][adr-319]). [`crate::fit::bayesian`] draws a fit's posterior so,
//! and `cargo xtask fit` counts its 90% intervals on the synthetic flights.
//!
//! [adr-319]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0319-m7-3c2-the-bayesian-option-in-two-blocks.md

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
    read_out(&points, draws, failed)
}

/// [`sample`] in two blocks, adapted on a pilot: for a posterior whose other unknowns follow some
/// of them along a ridge the Laplace approximation can't bend to, and which may reach further than
/// its curvature says.
///
/// **The blocks.** The unknowns `outer` marks are drawn first, from a Student-t about their share
/// of `peak`; then the others, from a Student-t about `inner_middle` of the point drawn so far, the
/// others still at `peak`. `inner_middle` gives a point whose unknowns not marked `outer` are where
/// the others are taken to center given the marked ones, as a step of Gauss–Newton's from `peak`
/// gives them; its marked unknowns aren't read. It must give the same answer for the same point:
/// the proposal's density is then known, so the draws weigh to the posterior whatever middle it
/// gives, a poor one costing effective draws only. `None` from it, or one not finite, makes a
/// failed draw.
///
/// **The pilot.** First `pilot` draws, the marked block's scale its share of `covariance` and the
/// others' their covariance given it (the Schur complement). Their weighted spread about the
/// proposal's center, `Ĉ = Σ ωᵢ eᵢ eᵢᵀ`, is set against each block's scale `S`: where, along an
/// eigenvector of `S^(-1/2) Ĉ S^(-T/2)`, the posterior reaches further than `S` says, that block's
/// scale is widened to it, up to [`MAX_WIDENING`] in variance; it is never narrowed. Then `draws`
/// more on the widened scales, and only those are read, so that given the pilot the answer is
/// plain importance sampling (adaptive importance sampling: M.-S. Oh and J. O. Berger, "Adaptive
/// importance sampling in Monte Carlo integration", *J. Statist. Comput. Simul.* 41, 1992,
/// pp. 143–168, who adapt a Student-t to a pilot's weighted moments). [`Posterior::draws`] and
/// [`Posterior::failed`] count both stages' draws; [`Posterior::effective_draws`] the read ones'.
///
/// Each draw's proposal is the product of two Student-t densities of ν = [`DEGREES_OF_FREEDOM`],
/// up to a constant `ln q = −(ν + K)/2 · ln(1 + δ₁²/ν) − (ν + N − K)/2 · ln(1 + δ₂²/ν)` for `K`
/// marked unknowns, each `δ²` its block's as in [`sample`]: a marginal times a conditional, not one
/// joint Student-t. Otherwise as [`sample`], whose `None`s it shares, and also `None` if a block's
/// scale isn't positive definite. With nothing marked it draws one Student-t about
/// `inner_middle(peak)`.
#[expect(
    clippy::too_many_arguments,
    reason = "a posterior, its blocks, two stages' draws, a seed and two functions"
)]
pub fn sample_in_two<const N: usize>(
    peak: &[f64; N],
    covariance: &[[f64; N]; N],
    outer: &[bool; N],
    pilot: usize,
    draws: usize,
    seed: u64,
    inner_middle: impl FnMut(&[f64; N]) -> Option<[f64; N]>,
    log_target: impl FnMut(&[f64; N]) -> Option<f64>,
) -> Option<Posterior<N>> {
    let stages = Stages {
        pilot,
        draws,
        adapt: true,
        proposal_in_weight: true,
    };
    sample_in_two_with(
        peak,
        covariance,
        outer,
        seed,
        stages,
        inner_middle,
        log_target,
    )
}

/// The most [`sample_in_two`]'s pilot widens a block's scale along any direction, in variance: 4,
/// twice the standard deviation, so that a pilot ruled by one far draw can't spread the main
/// draws past use. A judgment, not a measurement.
pub const MAX_WIDENING: f64 = 4.0;

/// How [`sample_in_two_with`] draws: its pilot's draws, its main draws, whether the pilot widens
/// the scales, and whether the proposal's density is in each weight; the last two off only in the
/// tests' probes.
#[derive(Debug, Clone, Copy)]
struct Stages {
    pilot: usize,
    draws: usize,
    adapt: bool,
    proposal_in_weight: bool,
}

/// One draw of [`sample_in_two`]: its point, its proposal's center for the second block, and its
/// log weight.
struct Drawn<const N: usize> {
    x: [f64; N],
    middle: [f64; N],
    log_weight: f64,
}

/// [`sample_in_two`], drawn as `stages` says.
fn sample_in_two_with<const N: usize>(
    peak: &[f64; N],
    covariance: &[[f64; N]; N],
    outer: &[bool; N],
    seed: u64,
    stages: Stages,
    mut inner_middle: impl FnMut(&[f64; N]) -> Option<[f64; N]>,
    mut log_target: impl FnMut(&[f64; N]) -> Option<f64>,
) -> Option<Posterior<N>> {
    if !peak.iter().all(|v| v.is_finite()) {
        return None;
    }
    let first: Vec<usize> = (0..N).filter(|&i| outer[i]).collect();
    let second: Vec<usize> = (0..N).filter(|&i| !outer[i]).collect();
    let block = |rows: &[usize], columns: &[usize]| -> Vec<Vec<f64>> {
        rows.iter()
            .map(|&i| {
                columns
                    .iter()
                    .map(|&j| covariance[i.max(j)][i.min(j)])
                    .collect()
            })
            .collect()
    };
    let l_first = cholesky_of(&block(&first, &first))?;
    // The second block's covariance given the first: Σ₂₂ − Aᵀ A, with L₁ A = Σ₁₂.
    let across = block(&first, &second);
    let a: Vec<Vec<f64>> = (0..second.len())
        .map(|c| {
            let column: Vec<f64> = across.iter().map(|row| row[c]).collect();
            forward_solve(&l_first, &column)
        })
        .collect();
    let mut given = block(&second, &second);
    for (i, row) in given.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value -= a[i].iter().zip(&a[j]).map(|(p, q)| p * q).sum::<f64>();
        }
    }
    let l_second = cholesky_of(&given)?;
    let mut rng = SeededRng::seed_from_u64(seed);
    let mut stage = |l_first: &[Vec<f64>], l_second: &[Vec<f64>], count: usize| {
        let mut made: Vec<Drawn<N>> = Vec::new();
        let mut failed = 0;
        for _ in 0..count {
            match draw_in_two(
                peak,
                (&first, l_first),
                (&second, l_second),
                &mut rng,
                &mut inner_middle,
                &mut log_target,
                stages.proposal_in_weight,
            ) {
                Some(Some(drawn)) => made.push(drawn),
                Some(None) => {}
                None => failed += 1,
            }
        }
        (made, failed)
    };
    let (piloted, pilot_failed) = stage(&l_first, &l_second, stages.pilot);
    let (l_first, l_second) = if stages.adapt {
        let top = piloted
            .iter()
            .map(|d| d.log_weight)
            .fold(f64::NEG_INFINITY, f64::max);
        let weights: Vec<f64> = piloted.iter().map(|d| (d.log_weight - top).exp()).collect();
        let total: f64 = weights.iter().sum();
        let spread = |indices: &[usize], about: &dyn Fn(&Drawn<N>) -> [f64; N]| {
            let mut c = vec![vec![0.0; indices.len()]; indices.len()];
            if top.is_finite() && total > 0.0 {
                for (d, w) in piloted.iter().zip(&weights) {
                    let center = about(d);
                    for (p, &i) in indices.iter().enumerate() {
                        for (q, &j) in indices.iter().enumerate() {
                            c[p][q] += w / total * (d.x[i] - center[i]) * (d.x[j] - center[j]);
                        }
                    }
                }
            }
            c
        };
        let spread_first = spread(&first, &|_| *peak);
        let spread_second = spread(&second, &|d| d.middle);
        (
            widened(&l_first, &spread_first)?,
            widened(&l_second, &spread_second)?,
        )
    } else {
        (l_first, l_second)
    };
    let (points, failed) = stage(&l_first, &l_second, stages.draws);
    let points: Vec<([f64; N], f64)> = points.into_iter().map(|d| (d.x, d.log_weight)).collect();
    read_out(&points, stages.pilot + stages.draws, pilot_failed + failed)
}

/// One draw in two blocks, each block its indices and its scale's lower triangle: `None` if it
/// failed, `Some(None)` where the posterior is zero, which has no weight and isn't a failure.
/// Every number a draw takes is drawn before any is used, so a failed draw moves no other.
fn draw_in_two<const N: usize>(
    peak: &[f64; N],
    (first, l_first): (&[usize], &[Vec<f64>]),
    (second, l_second): (&[usize], &[Vec<f64>]),
    rng: &mut SeededRng,
    inner_middle: &mut impl FnMut(&[f64; N]) -> Option<[f64; N]>,
    log_target: &mut impl FnMut(&[f64; N]) -> Option<f64>,
    proposal_in_weight: bool,
) -> Option<Option<Drawn<N>>> {
    let nu = DEGREES_OF_FREEDOM as f64;
    let z1: Vec<f64> = first.iter().map(|_| rng.standard_normal()).collect();
    let u1: f64 = (0..DEGREES_OF_FREEDOM)
        .map(|_| rng.standard_normal().powi(2))
        .sum();
    let z2: Vec<f64> = second.iter().map(|_| rng.standard_normal()).collect();
    let u2: f64 = (0..DEGREES_OF_FREEDOM)
        .map(|_| rng.standard_normal().powi(2))
        .sum();
    let (stretch1, stretch2) = (1.0 / (u1 / nu).sqrt(), 1.0 / (u2 / nu).sqrt());
    let mut x = *peak;
    for (row, &i) in first.iter().enumerate() {
        x[i] = peak[i] + stretch1 * (0..=row).map(|k| l_first[row][k] * z1[k]).sum::<f64>();
    }
    let middle = inner_middle(&x).filter(|m| second.iter().all(|&i| m[i].is_finite()))?;
    for (row, &i) in second.iter().enumerate() {
        x[i] = middle[i] + stretch2 * (0..=row).map(|k| l_second[row][k] * z2[k]).sum::<f64>();
    }
    // Casts: counts of unknowns, far below 2⁵³.
    let (k1, k2) = (first.len() as f64, second.len() as f64);
    let distance1 = z1.iter().map(|v| v * v).sum::<f64>() * stretch1 * stretch1;
    let distance2 = z2.iter().map(|v| v * v).sum::<f64>() * stretch2 * stretch2;
    let log_proposal = if proposal_in_weight {
        -0.5 * (nu + k1) * (distance1 / nu).ln_1p() - 0.5 * (nu + k2) * (distance2 / nu).ln_1p()
    } else {
        0.0
    };
    match log_target(&x) {
        Some(log_p) if log_p == f64::NEG_INFINITY => Some(None),
        Some(log_p) if (log_p - log_proposal).is_finite() => Some(Some(Drawn {
            x,
            middle,
            log_weight: log_p - log_proposal,
        })),
        _ => None,
    }
}

/// The lower triangle of the scale `L Lᵀ` widened to the spread `spread` where it reaches
/// further: with `A = L⁻¹ spread L⁻ᵀ = V Λ Vᵀ`, the scale `L V max(Λ, 1) Vᵀ Lᵀ`, each eigenvalue
/// held to [`MAX_WIDENING`]. `None` if that isn't positive definite.
fn widened(l: &[Vec<f64>], spread: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = l.len();
    if n == 0 {
        return Some(Vec::new());
    }
    // A = L⁻¹ C L⁻ᵀ: solve L B = C by columns, then L Aᵀ = Bᵀ.
    let b: Vec<Vec<f64>> = (0..n)
        .map(|c| forward_solve(l, &spread.iter().map(|row| row[c]).collect::<Vec<_>>()))
        .collect();
    // b[c] is column c of L⁻¹ C; A's column r is L⁻¹ (row r of L⁻¹ C).
    let rows: Vec<Vec<f64>> = (0..n).map(|r| (0..n).map(|c| b[c][r]).collect()).collect();
    let raw: Vec<Vec<f64>> = rows.iter().map(|row| forward_solve(l, row)).collect();
    // Symmetric to rounding: made so.
    let a: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| 0.5 * (raw[i][j] + raw[j][i])).collect())
        .collect();
    let (values, vectors) = jacobi_eigen(a);
    // B = V max(Λ, 1) Vᵀ, then L B Lᵀ.
    let held: Vec<f64> = values
        .iter()
        .map(|v| {
            if v.is_finite() {
                v.clamp(1.0, MAX_WIDENING)
            } else {
                1.0
            }
        })
        .collect();
    let mut m = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            m[i][j] = (0..n)
                .map(|k| vectors[i][k] * held[k] * vectors[j][k])
                .sum();
        }
    }
    let mut scale = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            scale[i][j] = (0..=i)
                .map(|p| (0..=j).map(|q| l[i][p] * m[p][q] * l[j][q]).sum::<f64>())
                .sum();
        }
    }
    cholesky_of(&scale)
}

/// The eigenvalues of the symmetric `a` and its eigenvectors as `V`'s columns, `a = V Λ Vᵀ`, by
/// Jacobi's rotations (W. H. Press et al., *Numerical Recipes*, 3rd ed., 2007, §11.1), swept until
/// what is off the diagonal is gone to rounding, or for 100 sweeps.
fn jacobi_eigen(mut a: Vec<Vec<f64>>) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut v: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    for _ in 0..100 {
        let off: f64 = (0..n)
            .flat_map(|i| (0..i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j] * a[i][j])
            .sum();
        let whole: f64 = (0..n).map(|i| a[i][i] * a[i][i]).sum::<f64>() + 2.0 * off;
        if off <= 1e-30 * whole || !off.is_finite() {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                if a[p][q] == 0.0 {
                    continue;
                }
                let theta = 0.5 * (a[q][q] - a[p][p]) / a[p][q];
                let t = theta.signum() / (theta.abs() + theta.hypot(1.0));
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / t.hypot(1.0);
                let s = t * c;
                for row in &mut a {
                    let (akp, akq) = (row[p], row[q]);
                    row[p] = c * akp - s * akq;
                    row[q] = s * akp + c * akq;
                }
                // Rows p and q, p < q.
                let (before, from_q) = a.split_at_mut(q);
                for (apk, aqk) in before[p].iter_mut().zip(from_q[0].iter_mut()) {
                    let (x, y) = (*apk, *aqk);
                    *apk = c * x - s * y;
                    *aqk = s * x + c * y;
                }
                for row in &mut v {
                    let (vkp, vkq) = (row[p], row[q]);
                    row[p] = c * vkp - s * vkq;
                    row[q] = s * vkp + c * vkq;
                }
            }
        }
    }
    ((0..n).map(|i| a[i][i]).collect(), v)
}

/// The lower triangle `L` of `a = L Lᵀ`, `a` square; `None` if it isn't positive definite. An
/// empty `a` gives an empty `L`.
fn cholesky_of(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
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

/// `y` with `L y = b`, `L` lower triangular.
fn forward_solve(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let mut y = b.to_vec();
    for i in 0..y.len() {
        let sum: f64 = (0..i).map(|k| l[i][k] * y[k]).sum();
        y[i] = (y[i] - sum) / l[i][i];
    }
    y
}

/// The posterior the weighted `points` give, each a point and its log weight, of `draws` draws of
/// which `failed` failed.
fn read_out<const N: usize>(
    points: &[([f64; N], f64)],
    draws: usize,
    failed: usize,
) -> Option<Posterior<N>> {
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

    /// The curved case in two blocks: `v₀` first, then `v₁` about `middle(v₀)`.
    fn curved_in_two(middle: fn(f64) -> f64, stages: Stages) -> Posterior<2> {
        let target =
            move |v: &[f64; 2]| Some(-0.5 * v[0] * v[0] - 0.5 * (v[1] - B * v[0] * v[0]).powi(2));
        sample_in_two_with(
            &[0.0, 0.0],
            &[[1.0, 0.0], [0.0, 1.0]],
            &[true, false],
            13,
            stages,
            |v| Some([v[0], middle(v[0])]),
            target,
        )
        .unwrap()
    }

    const IN_TWO: Stages = Stages {
        pilot: 2_000,
        draws: 40_000,
        adapt: true,
        proposal_in_weight: true,
    };

    #[test]
    fn in_two_blocks_a_curved_posterior_comes_back_at_its_quadrature() {
        // With the conditional's own middle, `B v₀²`, the draws follow the curve: far more of them
        // count than the one Student-t's about the peak keeps on the same posterior.
        let exact = curved_in_two(|v0| B * v0 * v0, IN_TWO);
        // Over seeds 0 to 199 (measured in development) the effective draws were at least 35,121
        // of 40,000, and the worst point's error had a standard deviation of 0.0068 and was at most
        // 0.039: 0.08 is over ten of those.
        assert!(
            exact.effective_draws > 0.75 * 40_000.0,
            "{}",
            exact.effective_draws
        );
        assert_eq!(exact.draws, 42_000);
        assert_eq!(exact.failed, 0);
        assert!(curved_worst(&exact) < 0.08, "{:?}", exact.quantiles[1]);
        // A middle that ignores the curve still weighs to the posterior: it costs draws only. Over
        // the same seeds it kept at most 0.70 of the exact middle's effective draws, and its worst
        // error had a standard deviation of 0.020 and was at most 0.13.
        let poor = curved_in_two(|_| 0.0, IN_TWO);
        assert!(poor.effective_draws < 0.75 * exact.effective_draws);
        assert!(curved_worst(&poor) < 0.15, "{:?}", poor.quantiles[1]);
    }

    #[test]
    fn in_two_blocks_the_proposal_must_be_in_the_weights() {
        let probe = curved_in_two(
            |v0| B * v0 * v0,
            Stages {
                proposal_in_weight: false,
                ..IN_TWO
            },
        );
        // Over seeds 0 to 199 its worst error was at least 0.93, past the poor middle's 0.15.
        assert!(curved_worst(&probe) > 0.5, "{:?}", probe.quantiles[1]);
    }

    /// A Gaussian of standard deviations `sd`, uncorrelated, drawn in two blocks (the first
    /// unknown, then the other two) from a unit scale, so that each unknown is `sd` times as wide
    /// as the scale says.
    fn gaussian_in_two(sd: [f64; 3], adapt: bool, seed: u64) -> Posterior<3> {
        sample_in_two_with(
            &[0.0; 3],
            &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &[true, false, false],
            seed,
            Stages {
                pilot: 1_000,
                draws: 4_000,
                adapt,
                proposal_in_weight: true,
            },
            |v| Some([v[0], 0.0, 0.0]),
            gaussian([0.0; 3], sd),
        )
        .unwrap()
    }

    /// The farthest of a centered Gaussian's points from its closed form, in its standard
    /// deviations.
    fn centered_worst<const N: usize>(posterior: &Posterior<N>, sd: [f64; N]) -> f64 {
        (0..N)
            .flat_map(|i| {
                let expected = [-Z_90 * sd[i], 0.0, Z_90 * sd[i]];
                posterior.quantiles[i]
                    .into_iter()
                    .zip(expected)
                    .map(move |(got, want)| ((got - want) / sd[i]).abs())
            })
            .fold(0.0, f64::max)
    }

    #[test]
    fn the_pilot_widens_a_scale_where_the_posterior_reaches_further() {
        // The second unknown is 1.8 times as wide as the scale says, the others as it says.
        let sd = [1.0, 1.8, 1.0];
        let adapted = gaussian_in_two(sd, true, 21);
        let fixed = gaussian_in_two(sd, false, 21);
        // Widened, the draws reach it: over seeds 0 to 199, at least 1.36 times as many effective
        // draws (measured in development).
        assert!(
            adapted.effective_draws > 1.3 * fixed.effective_draws,
            "{} against {}",
            adapted.effective_draws,
            fixed.effective_draws
        );
        // Over the same seeds the worst point's error had a standard deviation of 0.016 and was at
        // most 0.10, in the 1.8-wide unknown's tails.
        assert!(
            centered_worst(&adapted, sd) < 0.12,
            "{:?}",
            adapted.quantiles
        );
        assert_eq!(adapted.draws, 5_000);
    }

    #[test]
    fn the_pilot_never_narrows_a_scale_and_holds_its_widening() {
        // A scale twice too wide stays as it is: the posterior's points still come back.
        let narrow = [0.5, 0.5, 0.5];
        let adapted = gaussian_in_two(narrow, true, 22);
        let fixed = gaussian_in_two(narrow, false, 22);
        assert_eq!(
            adapted.effective_draws.to_bits(),
            fixed.effective_draws.to_bits()
        );
        // The widening is held to MAX_WIDENING in variance, so a spread of 10² in one direction
        // gives a scale of 2 there, not 10.
        let l = widened(
            &[vec![1.0, 0.0], vec![0.0, 1.0]],
            &[vec![100.0, 0.0], vec![0.0, 0.25]],
        )
        .unwrap();
        assert!(
            (l[0][0] - 2.0).abs() < 1e-12 && (l[1][1] - 1.0).abs() < 1e-12,
            "{l:?}"
        );
        assert!(l[1][0].abs() < 1e-12);
    }

    #[test]
    fn jacobi_gives_each_eigenvalue_and_its_vector() {
        let a = vec![
            vec![4.0, 1.0, -2.0, 0.5],
            vec![1.0, 3.0, 0.0, 1.5],
            vec![-2.0, 0.0, 5.0, -1.0],
            vec![0.5, 1.5, -1.0, 2.0],
        ];
        let (values, v) = jacobi_eigen(a.clone());
        for i in 0..4 {
            for j in 0..4 {
                let back: f64 = (0..4).map(|k| v[i][k] * values[k] * v[j][k]).sum();
                assert!((back - a[i][j]).abs() < 1e-12, "{i} {j}: {back}");
                let dot: f64 = (0..4).map(|k| v[k][i] * v[k][j]).sum();
                let unit = if i == j { 1.0 } else { 0.0 };
                assert!((dot - unit).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn in_two_blocks_a_failed_middle_is_counted_and_the_seed_holds() {
        let run = |seed| {
            sample_in_two(
                &[0.0, 0.0],
                &[[1.0, 0.0], [0.0, 1.0]],
                &[true, false],
                200,
                800,
                seed,
                |v| (v[0] < 1.0).then_some([v[0], 0.0]),
                gaussian([0.0, 0.0], [1.0, 1.0]),
            )
        };
        let posterior = run(4).unwrap();
        assert_eq!(posterior.draws, 1_000);
        // About a Student-t's share past 1 of its scale, 18.7% of both stages' draws: from 159 to
        // 215 over seeds 0 to 199 (measured in development).
        assert!(
            (120..260).contains(&posterior.failed),
            "{}",
            posterior.failed
        );
        let bits = |p: Option<Posterior<2>>| {
            p.map(|p| {
                (
                    p.quantiles.map(|row| row.map(f64::to_bits)),
                    p.effective_draws.to_bits(),
                    p.failed,
                )
            })
        };
        assert_eq!(bits(run(4)), bits(run(4)));
        assert_ne!(bits(run(4)), bits(run(5)));
        // A scale that isn't positive definite, in either block, gives none.
        let given = |c: [[f64; 2]; 2]| {
            sample_in_two(
                &[0.0, 0.0],
                &c,
                &[true, false],
                10,
                10,
                1,
                |v| Some(*v),
                gaussian([0.0, 0.0], [1.0, 1.0]),
            )
        };
        assert!(given([[-1.0, 0.0], [0.0, 1.0]]).is_none());
        assert!(given([[1.0, 0.0], [1.0, 1.0]]).is_none());
        assert!(given([[1.0, 0.0], [0.0, 1.0]]).is_some());
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
