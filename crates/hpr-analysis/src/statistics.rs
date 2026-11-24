//! Summaries of a sample of numbers: the spread of a Monte Carlo run's apogees, say.
//!
//! A [`Distribution`] keeps every value, sorted, and the number of samples that were tried, so a
//! sample that failed or never gave a value (a flight with no apogee) is counted, not dropped:
//! its share is [`Distribution::missing`] of [`Distribution::attempted`], and a probability is
//! reported as the bounds those unknowns allow ([`Distribution::share_at_least`]).
//!
//! - **Mean and standard deviation** are taken on the values shifted by the smallest one, the
//!   standard deviation by the two-pass formula with `n − 1` (T. F. Chan, G. H. Golub and R. J.
//!   LeVeque, "Algorithms for computing the sample variance: analysis and recommendations", *The
//!   American Statistician* 37(3), 242–247, 1983, <https://doi.org/10.2307/2683386>). Shifting
//!   by a value of the sample keeps the sums small, and makes a sample of equal values give that
//!   value and a deviation of exactly zero.
//! - **Quantiles** are Hyndman and Fan's definition 7, linear between order statistics, the
//!   default of R and NumPy: with the values sorted `x₀ ≤ … ≤ xₙ₋₁` and `h = (n − 1) p`,
//!   `Q(p) = x⌊h⌋ + (h − ⌊h⌋)(x⌊h⌋₊₁ − x⌊h⌋)` (R. J. Hyndman and Y. Fan, "Sample quantiles in
//!   statistical packages", *The American Statistician* 50(4), 361–365, 1996,
//!   <https://doi.org/10.2307/2684934>).
//! - **Quantiles counting the missing runs** ([`Distribution::quantile_counting_missing`]) take
//!   the same definition over every run tried, `h = (N − 1) p` with `N` the runs tried, each run
//!   with no value placed past one end of the values (as `+∞` above the largest, or `−∞` below
//!   the smallest). That is the worst case the missing runs allow: if they all lie beyond that
//!   end, the quantile of every run tried is this one. Where the interpolation puts weight on a
//!   missing run the quantile is [`CountedQuantile::Unbounded`]: the runs that gave a value can't bound it.
//!
//! Every sum runs over the sorted values in order, so a summary is bit-for-bit the same however
//! the values were computed, in parallel or not.
//!
//! [`normal_quantile`] is the standard normal distribution's inverse, for a sample stratified by
//! probability.

use serde::{Deserialize, Serialize};

use crate::error::AnalysisError;

/// `Φ⁻¹(p)`, the standard normal deviate with the share `p` of the distribution at or below it:
/// `−∞` at 0, `+∞` at 1, and NaN for a `p` outside `[0, 1]` or not a number.
///
/// It inverts `Φ(x) = erfc(−x/√2)/2` by bisection, with `erfc` from the `libm` crate, which
/// keeps its relative accuracy in the lower tail where `1 − erf` would cancel. A `p` over 1/2 is
/// mirrored, `Φ⁻¹(p) = −Φ⁻¹(1 − p)`, so the search is always in the lower tail. The bracket
/// starts at `[−40, 0]` (`Φ(−40)` is below the smallest positive `f64`) and is halved until its
/// ends are neighboring numbers, and the upper end is returned: the smallest `x` whose computed
/// `Φ` is at least `p`. So the result is within one step of `f64` of that `Φ`'s exact inverse,
/// and it never decreases as `p` grows.
///
/// The optimizer's integer margin keeps its own closed-form inverse, Wichura's AS 241, inside
/// its module. This one costs about 60 evaluations of `erfc` for a `p` away from 1/2, and about
/// 110 for one a rounding from it.
#[must_use]
pub fn normal_quantile(p: f64) -> f64 {
    if !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }
    if p == 0.0 {
        return f64::NEG_INFINITY;
    }
    if p == 1.0 {
        return f64::INFINITY;
    }
    if p == 0.5 {
        return 0.0;
    }
    if p > 0.5 {
        // 1 − p is exact for a p from 1/2 to 1 (Sterbenz).
        return -normal_quantile(1.0 - p);
    }
    let (mut low, mut high) = (-40.0_f64, 0.0_f64);
    loop {
        let middle = 0.5 * (low + high);
        if middle <= low || middle >= high {
            return high;
        }
        if 0.5 * libm::erfc(-middle / std::f64::consts::SQRT_2) < p {
            low = middle;
        } else {
            high = middle;
        }
    }
}

/// The values a sample of runs gave, sorted, and how many runs were tried. It serializes as
/// those two, and reads back through [`Distribution::new`]'s checks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "DistributionData")]
pub struct Distribution {
    attempted: usize,
    sorted: Vec<f64>,
}

/// The serialized form of a [`Distribution`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DistributionData {
    attempted: usize,
    sorted: Vec<f64>,
}

impl TryFrom<DistributionData> for Distribution {
    type Error = AnalysisError;

    fn try_from(data: DistributionData) -> Result<Self, AnalysisError> {
        Self::new(data.sorted, data.attempted)
    }
}

/// The usual numbers of a [`Distribution`], for a report.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    /// The runs tried.
    pub attempted: usize,
    /// The runs that gave a value.
    pub count: usize,
    /// The mean, `None` with no values.
    pub mean: Option<f64>,
    /// The sample standard deviation (`n − 1`), `None` with fewer than two values.
    pub standard_deviation: Option<f64>,
    /// The smallest value.
    pub min: Option<f64>,
    /// The 5th percentile ([`Distribution::quantile`]).
    pub p05: Option<f64>,
    /// The median.
    pub p50: Option<f64>,
    /// The 95th percentile.
    pub p95: Option<f64>,
    /// The largest value.
    pub max: Option<f64>,
    /// The 5th percentile with every run that gave no value placed below the smallest value
    /// ([`Distribution::quantile_counting_missing`]); `None` when every run gave a value, as the
    /// 5th percentile is then `p05`.
    pub p05_counting_missing: Option<CountedQuantile>,
    /// The 95th percentile with every run that gave no value placed above the largest value;
    /// `None` when every run gave a value.
    pub p95_counting_missing: Option<CountedQuantile>,
    /// Wilks's one-sided 95/95 upper tolerance bound ([`Distribution::upper_95_95`]): the figure
    /// to plan an upper limit with, where `p95` is the sample's own percentile. `None` with
    /// fewer than 59 runs tried.
    pub upper_95_95: Option<CountedQuantile>,
}

/// Which end of the values [`Distribution::quantile_counting_missing`] places the runs that gave
/// no value past.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingEnd {
    /// Below the smallest value: the worst case for a low quantile, such as the 5th percentile.
    Below,
    /// Above the largest value: the worst case for a high quantile, such as the 95th percentile.
    Above,
}

/// A quantile that counts the runs that gave no value ([`Distribution::quantile_counting_missing`]).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountedQuantile {
    /// The quantile, a value the runs gave or one between two of them.
    Value(f64),
    /// Too many runs gave no value for the rest to bound the quantile: its interpolation puts
    /// weight on a missing run.
    Unbounded,
}

/// Bounds on the share of all the runs tried whose value passed a test: `low` counts a run with
/// no value as failing it, `high` as passing. Those two are the sample's own shares; the true
/// share, of all the runs the same scatter could give, is bounded by `low_bound` and
/// `high_bound`, one-sided Clopper–Pearson bounds at [`SHARE_CONFIDENCE`] with each run that gave
/// no value on the unfavorable side of each.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Share {
    /// The runs with a value that passed, over every run tried.
    pub low: f64,
    /// The same, with every run that gave no value counted as passing.
    pub high: f64,
    /// The true share is at least this, at [`SHARE_CONFIDENCE`]: the one-sided Clopper–Pearson
    /// lower bound on the runs that passed, a run with no value counted as failing.
    pub low_bound: f64,
    /// The true share is at most this, at [`SHARE_CONFIDENCE`]: the one-sided Clopper–Pearson
    /// upper bound on the runs that passed, a run with no value counted as passing. With none
    /// passed and none missing it is `1 − (1 − c)^(1/n)`: 2.95% at 100 runs.
    pub high_bound: f64,
}

/// The confidence of each one-sided bound a [`Share`] and [`Distribution::upper_95_95`] give.
pub const SHARE_CONFIDENCE: f64 = 0.95;

impl Share {
    /// The share of `attempted` runs of which `passed` passed and `missing` gave no value, with
    /// its bounds. `None` with no runs, or with more passed and missing than runs.
    #[must_use]
    pub fn of(passed: usize, missing: usize, attempted: usize) -> Option<Self> {
        if attempted == 0 || passed + missing > attempted {
            return None;
        }
        // Cast: counts far below 2⁵³.
        let (passed_f, attempted_f) = (passed as f64, attempted as f64);
        let (x, gone, n) = (passed as u64, missing as u64, attempted as u64);
        // A one-sided bound at c is one edge of the two-sided interval at 2c − 1.
        let two_sided = 2.0 * SHARE_CONFIDENCE - 1.0;
        Some(Self {
            low: passed_f / attempted_f,
            high: (passed + missing) as f64 / attempted_f,
            low_bound: clopper_pearson_lower(x, n, two_sided),
            high_bound: clopper_pearson_upper(x + gone, n, two_sided),
        })
    }
}

/// `P(X ≥ x)` for `X ~ Binomial(n, p)`, summing the binomial terms of whichever tail is the
/// smaller, each from `ln C(n, k)` by the recurrence `ln C(n, k + 1) = ln C(n, k) + ln(n − k) −
/// ln(k + 1)`: `n` steps at most.
fn binomial_at_least(x: u64, n: u64, p: f64) -> f64 {
    if x == 0 {
        return 1.0;
    }
    if x > n || p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return 1.0;
    }
    let (lp, lq) = (p.ln(), (-p).ln_1p());
    // Cast: counts far below 2⁵³.
    let term = |k: u64, ln_choose: f64| (ln_choose + k as f64 * lp + (n - k) as f64 * lq).exp();
    let upper = x as f64 > n as f64 * p;
    let (mut ln_choose, mut sum) = (0.0_f64, 0.0_f64);
    for k in 0..=n {
        // The upper tail sums from x on; the lower one up to x, and stops there.
        if upper == (k >= x) {
            sum += term(k, ln_choose);
        } else if !upper {
            break;
        }
        if k < n {
            ln_choose += ((n - k) as f64).ln() - ((k + 1) as f64).ln();
        }
    }
    if upper {
        sum.min(1.0)
    } else {
        (1.0 - sum).max(0.0)
    }
}

/// The lower edge of the two-sided Clopper–Pearson interval on a binomial share, at `confidence`:
/// the `p` at which `P(X ≥ x) = (1 − confidence)/2`, by bisection to neighboring numbers; 0 when
/// `x` is 0 (C. J. Clopper and E. S. Pearson, "The use of confidence or fiducial limits
/// illustrated in the case of the binomial", *Biometrika* 26(4), 404–413, 1934). NaN when `x`
/// exceeds `n` or `confidence` isn't strictly between 0 and 1. One edge alone is a one-sided
/// bound at `(1 + confidence)/2`.
#[must_use]
pub fn clopper_pearson_lower(x: u64, n: u64, confidence: f64) -> f64 {
    if x > n || !(confidence > 0.0 && confidence < 1.0) {
        return f64::NAN;
    }
    if x == 0 {
        return 0.0;
    }
    let target = 0.5 * (1.0 - confidence);
    // Cast: counts far below 2⁵³. P(X ≥ x) rises with p, and at p = x/n it is near 1/2.
    let (mut lo, mut hi) = (0.0_f64, x as f64 / n as f64);
    loop {
        let middle = 0.5 * (lo + hi);
        if middle <= lo || middle >= hi {
            return hi;
        }
        if binomial_at_least(x, n, middle) < target {
            lo = middle;
        } else {
            hi = middle;
        }
    }
}

/// The upper edge of the two-sided Clopper–Pearson interval on a binomial share at `confidence`,
/// `1 −` the lower edge of the share of the other outcome; 1 when `x` is `n`. NaN where
/// [`clopper_pearson_lower`] is.
#[must_use]
pub fn clopper_pearson_upper(x: u64, n: u64, confidence: f64) -> f64 {
    if x > n {
        return f64::NAN;
    }
    1.0 - clopper_pearson_lower(n - x, n, confidence)
}

impl Distribution {
    /// The distribution of `values`, from `attempted` runs (those that gave no value make up the
    /// difference).
    ///
    /// # Errors
    ///
    /// - [`AnalysisError::Domain`] for a value that isn't finite.
    /// - [`AnalysisError::Count`] for more values than runs.
    pub fn new(values: Vec<f64>, attempted: usize) -> Result<Self, AnalysisError> {
        if values.len() > attempted {
            return Err(AnalysisError::Count {
                what: "values in a distribution, against the runs tried",
                count: values.len(),
                limit: attempted,
            });
        }
        if let Some(&bad) = values.iter().find(|v| !v.is_finite()) {
            return Err(AnalysisError::Domain {
                what: "value in a distribution",
                value: bad,
            });
        }
        let mut sorted = values;
        sorted.sort_by(f64::total_cmp);
        Ok(Self { attempted, sorted })
    }

    /// The runs tried.
    pub fn attempted(&self) -> usize {
        self.attempted
    }

    /// The runs that gave a value.
    pub fn count(&self) -> usize {
        self.sorted.len()
    }

    /// The runs that gave no value: failed, or without the quantity asked for.
    pub fn missing(&self) -> usize {
        self.attempted - self.sorted.len()
    }

    /// The values, smallest first.
    pub fn sorted(&self) -> &[f64] {
        &self.sorted
    }

    /// The smallest value, `None` with no values.
    pub fn min(&self) -> Option<f64> {
        self.sorted.first().copied()
    }

    /// The largest value, `None` with no values.
    pub fn max(&self) -> Option<f64> {
        self.sorted.last().copied()
    }

    /// The mean, `x₀ + Σ(xᵢ − x₀)/n` with `x₀` the smallest value; `None` with no values.
    pub fn mean(&self) -> Option<f64> {
        let shift = self.min()?;
        Some(shift + self.shifted_mean(shift))
    }

    /// The sample standard deviation, `√(Σ(dᵢ − d̄)²/(n − 1))` with `dᵢ = xᵢ − x₀`; `None` with
    /// fewer than two values.
    pub fn standard_deviation(&self) -> Option<f64> {
        if self.sorted.len() < 2 {
            return None;
        }
        let shift = self.min()?;
        let mean = self.shifted_mean(shift);
        let sum_squares: f64 = self
            .sorted
            .iter()
            .map(|&x| {
                let d = (x - shift) - mean;
                d * d
            })
            .sum();
        // Cast: a count of values is far below 2⁵³.
        Some((sum_squares / (self.sorted.len() - 1) as f64).sqrt())
    }

    /// The mean of the values less `shift`.
    fn shifted_mean(&self, shift: f64) -> f64 {
        let sum: f64 = self.sorted.iter().map(|&x| x - shift).sum();
        // Cast: a count of values is far below 2⁵³.
        sum / self.sorted.len() as f64
    }

    /// The `p` quantile by Hyndman and Fan's definition 7 (the module's docs); `None` with no
    /// values.
    ///
    /// # Errors
    ///
    /// [`AnalysisError::Domain`] for `p` outside `[0, 1]`.
    pub fn quantile(&self, p: f64) -> Result<Option<f64>, AnalysisError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(AnalysisError::Domain {
                what: "quantile probability",
                value: p,
            });
        }
        let Some(&last) = self.sorted.last() else {
            return Ok(None);
        };
        // Cast: a count of values is far below 2⁵³, and `h` lies in `[0, n − 1]`.
        let h = (self.sorted.len() - 1) as f64 * p;
        let below = h.floor();
        let index = below as usize;
        let Some(&above) = self.sorted.get(index + 1) else {
            return Ok(Some(last));
        };
        let low = self.sorted[index];
        Ok(Some(low + (h - below) * (above - low)))
    }

    /// The `p` quantile of every run tried, each run that gave no value placed past `end` of the
    /// values (the module's docs); `None` with no runs tried. With no missing run it is
    /// [`Distribution::quantile`]'s value, bit for bit.
    ///
    /// The missing runs' worst case: a 95% value read over the runs that gave one alone sits
    /// below the 95% of every run tried if the missing runs lie above it, so the high quantile
    /// counts them [`MissingEnd::Above`] and the low one [`MissingEnd::Below`].
    ///
    /// # Errors
    ///
    /// [`AnalysisError::Domain`] for `p` outside `[0, 1]`.
    pub fn quantile_counting_missing(
        &self,
        p: f64,
        end: MissingEnd,
    ) -> Result<Option<CountedQuantile>, AnalysisError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(AnalysisError::Domain {
                what: "quantile probability",
                value: p,
            });
        }
        if self.attempted == 0 {
            return Ok(None);
        }
        let missing = self.missing();
        // The `j`th of every run tried, smallest first: `None` for a missing run.
        let value = |j: usize| match end {
            MissingEnd::Above => self.sorted.get(j).copied(),
            MissingEnd::Below => j
                .checked_sub(missing)
                .and_then(|i| self.sorted.get(i).copied()),
        };
        // Cast: a count of runs is far below 2⁵³, and `h` lies in `[0, N − 1]`.
        let h = (self.attempted - 1) as f64 * p;
        let below = h.floor();
        let index = below as usize;
        let Some(low) = value(index) else {
            return Ok(Some(CountedQuantile::Unbounded));
        };
        if index + 1 == self.attempted {
            return Ok(Some(CountedQuantile::Value(low)));
        }
        match value(index + 1) {
            Some(above) => Ok(Some(CountedQuantile::Value(
                low + (h - below) * (above - low),
            ))),
            // `h` on a value: the missing run next to it carries no weight.
            None if h == below => Ok(Some(CountedQuantile::Value(low))),
            None => Ok(Some(CountedQuantile::Unbounded)),
        }
    }

    /// Bounds on the share of the runs tried whose value is at least `threshold` ([`Share`]);
    /// `None` with no runs tried, when there is no share to bound. An infinite threshold is
    /// allowed.
    ///
    /// # Errors
    ///
    /// [`AnalysisError::Domain`] for a threshold that is not a number.
    pub fn share_at_least(&self, threshold: f64) -> Result<Option<Share>, AnalysisError> {
        if threshold.is_nan() {
            return Err(AnalysisError::Domain {
                what: "threshold of a share",
                value: threshold,
            });
        }
        if self.attempted == 0 {
            return Ok(None);
        }
        let passed = self.sorted.len() - self.sorted.partition_point(|&x| x < threshold);
        Ok(Share::of(passed, self.missing(), self.attempted))
    }

    /// Wilks's one-sided 95/95 upper tolerance bound: a value that, with 95% confidence
    /// ([`SHARE_CONFIDENCE`]), at least 95% of all the runs the same scatter could give lie at or
    /// below. It is the `m`-th largest of the runs tried, with `m` the largest order for which
    /// `P(Binomial(n, 0.05) ≥ m) ≥ 0.95`: the `m`-th largest lies at or above the true 95th
    /// percentile exactly when at least `m` of the `n` runs do (S. S. Wilks, "Determination of
    /// sample sizes for setting tolerance limits", *Annals of Mathematical Statistics* 12(1),
    /// 91–96, 1941). At 100 runs `m` is 2, the second largest; it takes 59 runs for the largest
    /// to be one, and with fewer there is none (`None`). A run with no value is placed above the
    /// largest, the worst case for an upper bound, so with `m` or more of them the bound is
    /// [`CountedQuantile::Unbounded`].
    #[must_use]
    pub fn upper_95_95(&self) -> Option<CountedQuantile> {
        let n = self.attempted as u64;
        let holds = |m: u64| binomial_at_least(m, n, 1.0 - SHARE_CONFIDENCE) >= SHARE_CONFIDENCE;
        if n == 0 || !holds(1) {
            return None;
        }
        // P(Binomial(n, 0.05) ≥ m) falls as m rises: the largest m that holds, by bisection
        // between one that holds and one that doesn't (none does past n).
        let (mut good, mut bad) = (1_u64, n + 1);
        while bad - good > 1 {
            let middle = good + (bad - good) / 2;
            if holds(middle) {
                good = middle;
            } else {
                bad = middle;
            }
        }
        let order = good;
        // Cast: an order of at most `attempted`.
        let order = order as usize;
        let missing = self.missing();
        if order <= missing {
            return Some(CountedQuantile::Unbounded);
        }
        // The (order − missing)-th largest value: the runs with none sit above it.
        self.sorted
            .get(self.sorted.len() - (order - missing))
            .map(|&value| CountedQuantile::Value(value))
    }

    /// The usual numbers, for a report: [`Summary`].
    pub fn summary(&self) -> Summary {
        // The probabilities are in [0, 1].
        let quantile = |p: f64| self.quantile(p).ok().flatten();
        let counting_missing = |p: f64, end: MissingEnd| {
            if self.missing() == 0 {
                None
            } else {
                self.quantile_counting_missing(p, end).ok().flatten()
            }
        };
        Summary {
            attempted: self.attempted,
            count: self.count(),
            mean: self.mean(),
            standard_deviation: self.standard_deviation(),
            min: self.min(),
            p05: quantile(0.05),
            p50: quantile(0.5),
            p95: quantile(0.95),
            max: self.max(),
            p05_counting_missing: counting_missing(0.05, MissingEnd::Below),
            p95_counting_missing: counting_missing(0.95, MissingEnd::Above),
            upper_95_95: self.upper_95_95(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one-sided 95% Clopper–Pearson bounds at 100 runs against exact values (the binomial
    /// sums in 40-digit arithmetic, mpmath, bisected): none of 100 is at most
    /// 1 − 0.05^(1/100) = 2.9513%; one of 100 at most 4.6560% and at least 0.05128%; four at most
    /// 8.9196%; 65 between 56.392% and 72.925%. A run with no value moves each bound to its
    /// unfavorable side: one passed and three missing is at most the four's 8.9196%.
    #[test]
    #[expect(
        clippy::excessive_precision,
        reason = "17 digits of each 40-digit reference, which round to the nearest f64"
    )]
    fn a_shares_bounds_at_a_hundred_runs_by_hand() {
        let share = |passed, missing| Share::of(passed, missing, 100).expect("a share");
        let close = |got: f64, exact: f64| (got - exact).abs() < 1e-12;
        let none = share(0, 0);
        assert_eq!((none.low, none.high, none.low_bound), (0.0, 0.0, 0.0));
        assert!(
            close(none.high_bound, 0.029_513_049_607_039_935),
            "{none:?}"
        );
        let one = share(1, 0);
        assert!(close(one.high_bound, 0.046_559_811_453_538_946), "{one:?}");
        assert!(close(one.low_bound, 0.000_512_801_416_262_292_1), "{one:?}");
        assert!(close(share(4, 0).high_bound, 0.089_196_250_158_879_886));
        let sixty_five = share(65, 0);
        assert!(close(sixty_five.low_bound, 0.563_916_359_180_062_84));
        assert!(close(sixty_five.high_bound, 0.729_245_538_188_444_45));
        let failed = share(1, 3);
        assert_eq!((failed.low, failed.high), (0.01, 0.04));
        assert_eq!(failed.low_bound, one.low_bound);
        assert_eq!(failed.high_bound, share(4, 0).high_bound);
        // All passed: at least 97.05%, the mirror of none.
        let all = share(100, 0);
        assert_eq!(all.high_bound, 1.0);
        assert!(
            close(all.low_bound, 1.0 - 0.029_513_049_607_039_935),
            "{all:?}"
        );
        assert_eq!(Share::of(1, 0, 0), None);
        assert_eq!(Share::of(60, 41, 100), None);
    }

    /// Wilks's order at 58, 59, 100 and 1,000 runs (none, the largest, the second, the 39th
    /// largest; mpmath's exact binomial sums), and the bound on a sample: the second largest of
    /// 100, moved down past each run with no value, unbounded with two of them.
    #[test]
    fn the_95_95_bound_takes_wilkss_order() {
        let ranks = |n: usize, missing: usize| {
            let values = (1..=n - missing).map(|i| i as f64).collect::<Vec<_>>();
            Distribution::new(values, n)
                .expect("a sample")
                .upper_95_95()
        };
        assert_eq!(ranks(58, 0), None);
        assert_eq!(ranks(59, 0), Some(CountedQuantile::Value(59.0)));
        assert_eq!(ranks(100, 0), Some(CountedQuantile::Value(99.0)));
        assert_eq!(ranks(1000, 0), Some(CountedQuantile::Value(962.0)));
        // One run with no value sits above the rest: the second largest of all is the largest
        // of the 99 values.
        assert_eq!(ranks(100, 1), Some(CountedQuantile::Value(99.0)));
        assert_eq!(ranks(100, 2), Some(CountedQuantile::Unbounded));
        assert_eq!(ranks(59, 1), Some(CountedQuantile::Unbounded));
        // The 95th percentile it stands beside is below it.
        let sample = Distribution::new((1..=100).map(f64::from).collect(), 100).expect("a sample");
        let p95 = sample.quantile(0.95).expect("in range").expect("values");
        assert!(p95 < 99.0, "{p95}");
    }

    /// The binomial tail by hand: `P(X ≥ 1)` of 100 at 0.05 is `1 − 0.95¹⁰⁰`, and both of its
    /// sums agree where they meet.
    #[test]
    fn the_binomial_tail_by_hand() {
        let at_least_one = binomial_at_least(1, 100, 0.05);
        assert!((at_least_one - (1.0 - 0.95_f64.powi(100))).abs() < 1e-14);
        assert_eq!(binomial_at_least(0, 100, 0.3), 1.0);
        assert_eq!(binomial_at_least(101, 100, 0.3), 0.0);
        // At the mean the sum switches tails; one either side still adds up.
        let (below, at, above) = (
            binomial_at_least(29, 100, 0.3),
            binomial_at_least(30, 100, 0.3),
            binomial_at_least(31, 100, 0.3),
        );
        assert!(below > at && at > above, "{below} {at} {above}");
        assert!((at - 0.537_660_263_984_637_6).abs() < 1e-12, "{at}");
    }

    #[test]
    fn equal_values_give_their_value_and_no_spread() {
        // A sum of 0.1s divided by their count isn't 0.1 in floating point; shifted by the
        // smallest value, it is.
        let d = Distribution::new(vec![0.1; 7], 7).unwrap();
        assert_eq!(d.mean(), Some(0.1));
        assert_eq!(d.standard_deviation(), Some(0.0));
        assert_eq!(d.quantile(0.37).unwrap(), Some(0.1));
    }

    #[test]
    fn moments_and_quantiles_by_hand() {
        let d = Distribution::new(vec![4.0, 1.0, 3.0, 2.0], 6).unwrap();
        assert_eq!(d.sorted(), &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!((d.count(), d.missing(), d.attempted()), (4, 2, 6));
        assert_eq!(d.mean(), Some(2.5));
        // Σ(x − 2.5)² = 5, over n − 1 = 3.
        assert_eq!(d.standard_deviation(), Some((5.0_f64 / 3.0).sqrt()));
        // h = 3p: p = 0.5 is halfway from 2 to 3; p = 0.9 is 0.7 of the way from 3 to 4. NumPy's
        // `np.quantile([1, 2, 3, 4], [0, 0.5, 0.9, 1])` gives the same: 1, 2.5, 3.7, 4.
        assert_eq!(d.quantile(0.0).unwrap(), Some(1.0));
        assert_eq!(d.quantile(0.5).unwrap(), Some(2.5));
        assert!((d.quantile(0.9).unwrap().unwrap() - 3.7).abs() < 1e-15);
        assert_eq!(d.quantile(1.0).unwrap(), Some(4.0));
        let summary = d.summary();
        assert_eq!((summary.min, summary.max), (Some(1.0), Some(4.0)));
        assert_eq!(summary.p50, Some(2.5));
    }

    #[test]
    fn a_share_is_bounded_by_the_runs_with_no_value() {
        // Two of six runs gave nothing: at least 2/6 and at most 4/6 reached 3.
        let d = Distribution::new(vec![1.0, 2.0, 3.0, 4.0], 6).unwrap();
        let share = d.share_at_least(3.0).unwrap().unwrap();
        assert_eq!((share.low, share.high), (2.0 / 6.0, 4.0 / 6.0));
        let all = Distribution::new(vec![1.0, 2.0], 2).unwrap();
        let share = all.share_at_least(1.0).unwrap().unwrap();
        assert_eq!((share.low, share.high), (1.0, 1.0));
        let share = all.share_at_least(f64::INFINITY).unwrap().unwrap();
        assert_eq!((share.low, share.high), (0.0, 0.0));
        let share = all.share_at_least(f64::NEG_INFINITY).unwrap().unwrap();
        assert_eq!((share.low, share.high), (1.0, 1.0));
        // No runs, no share; a threshold that isn't a number is refused.
        let none = Distribution::new(vec![], 0).unwrap();
        assert_eq!(none.share_at_least(1.0).unwrap(), None);
        assert!(matches!(
            all.share_at_least(f64::NAN),
            Err(AnalysisError::Domain {
                what: "threshold of a share",
                ..
            })
        ));
    }

    #[test]
    fn a_quantile_counting_the_missing_runs_by_hand() {
        // Four values of six runs. Above: the runs are 1, 2, 3, 4, +∞, +∞, and `h = 5p`.
        let d = Distribution::new(vec![4.0, 1.0, 3.0, 2.0], 6).unwrap();
        let above = |p| d.quantile_counting_missing(p, MissingEnd::Above).unwrap();
        let below = |p| d.quantile_counting_missing(p, MissingEnd::Below).unwrap();
        assert_eq!(above(0.0), Some(CountedQuantile::Value(1.0)));
        assert_eq!(above(0.5), Some(CountedQuantile::Value(3.5)));
        // h = 3.0 sits on the 4, the last value: the missing run after it has no weight.
        assert_eq!(above(0.6), Some(CountedQuantile::Value(4.0)));
        assert_eq!(above(0.65), Some(CountedQuantile::Unbounded));
        assert_eq!(above(0.95), Some(CountedQuantile::Unbounded));
        // Below: −∞, −∞, 1, 2, 3, 4.
        assert_eq!(below(0.5), Some(CountedQuantile::Value(1.5)));
        assert_eq!(below(0.4), Some(CountedQuantile::Value(1.0)));
        assert_eq!(below(0.35), Some(CountedQuantile::Unbounded));
        assert_eq!(below(0.05), Some(CountedQuantile::Unbounded));
        assert_eq!(below(1.0), Some(CountedQuantile::Value(4.0)));
        // Nothing tried, nothing to bound; every run missing, nothing bounded.
        let none = Distribution::new(vec![], 0).unwrap();
        assert_eq!(
            none.quantile_counting_missing(0.5, MissingEnd::Above)
                .unwrap(),
            None
        );
        let empty = Distribution::new(vec![], 3).unwrap();
        for end in [MissingEnd::Above, MissingEnd::Below] {
            assert_eq!(
                empty.quantile_counting_missing(0.0, end).unwrap(),
                Some(CountedQuantile::Unbounded)
            );
        }
        for p in [-0.1, 1.1, f64::NAN] {
            assert!(matches!(
                d.quantile_counting_missing(p, MissingEnd::Above),
                Err(AnalysisError::Domain { what: "quantile probability", value })
                    if value.to_bits() == p.to_bits()
            ));
        }
    }

    #[test]
    fn with_no_missing_run_it_is_the_quantile_bit_for_bit() {
        let values: Vec<f64> = (0..37)
            .map(|i| f64::from(i).mul_add(0.37, 1.0).sin())
            .collect();
        let d = Distribution::new(values, 37).unwrap();
        for step in 0..=200 {
            let p = f64::from(step) / 200.0;
            let plain = d.quantile(p).unwrap().unwrap();
            for end in [MissingEnd::Above, MissingEnd::Below] {
                assert_eq!(
                    d.quantile_counting_missing(p, end).unwrap(),
                    Some(CountedQuantile::Value(plain)),
                    "p = {p}"
                );
            }
        }
        // The summary leaves the counting quantiles out when nothing is missing.
        let summary = d.summary();
        assert_eq!(summary.p05_counting_missing, None);
        assert_eq!(summary.p95_counting_missing, None);
    }

    #[test]
    fn the_missing_runs_widen_each_tail_toward_its_own_end() {
        // 1 to 20 from 21 runs. Over the 20 values the 95th percentile is 19.05 and the 5th
        // 1.95. Counting the missing run past each end gives the worst case: higher at the top
        // (h = 20 × 0.95 = 19, the 20), lower at the bottom (h = 1, the 1).
        let d = Distribution::new((1..=20).map(f64::from).collect(), 21).unwrap();
        let summary = d.summary();
        assert!((summary.p95.unwrap() - 19.05).abs() < 1e-12);
        assert!((summary.p05.unwrap() - 1.95).abs() < 1e-12);
        assert_eq!(
            summary.p95_counting_missing,
            Some(CountedQuantile::Value(20.0))
        );
        assert_eq!(
            summary.p05_counting_missing,
            Some(CountedQuantile::Value(1.0))
        );
        // Two missing of 21: the top's interpolation reaches a missing run, and so does the
        // bottom's.
        let d = Distribution::new((1..=19).map(f64::from).collect(), 21).unwrap();
        let summary = d.summary();
        assert_eq!(
            summary.p95_counting_missing,
            Some(CountedQuantile::Unbounded)
        );
        assert_eq!(
            summary.p05_counting_missing,
            Some(CountedQuantile::Unbounded)
        );
    }

    #[test]
    fn a_bound_serializes_as_a_value_or_unbounded() {
        assert_eq!(
            serde_json::to_string(&CountedQuantile::Value(2.5)).unwrap(),
            r#"{"value":2.5}"#
        );
        assert_eq!(
            serde_json::to_string(&CountedQuantile::Unbounded).unwrap(),
            r#""unbounded""#
        );
    }

    #[test]
    fn empty_and_single_samples() {
        let empty = Distribution::new(vec![], 3).unwrap();
        assert_eq!(empty.mean(), None);
        assert_eq!(empty.quantile(0.5).unwrap(), None);
        assert_eq!(empty.missing(), 3);
        let one = Distribution::new(vec![2.0], 1).unwrap();
        assert_eq!(one.mean(), Some(2.0));
        assert_eq!(one.standard_deviation(), None);
        assert_eq!(one.quantile(0.3).unwrap(), Some(2.0));
    }

    #[test]
    fn a_distribution_reads_back_through_its_checks() {
        let d = Distribution::new(vec![3.0, 1.0, 2.0], 4).unwrap();
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, r#"{"attempted":4,"sorted":[1.0,2.0,3.0]}"#);
        assert_eq!(serde_json::from_str::<Distribution>(&json).unwrap(), d);
        // Unsorted values are sorted on the way in; more values than runs are refused.
        let unsorted: Distribution =
            serde_json::from_str(r#"{"attempted":3,"sorted":[3.0,1.0,2.0]}"#).unwrap();
        assert_eq!(unsorted.sorted(), &[1.0, 2.0, 3.0]);
        let error = serde_json::from_str::<Distribution>(r#"{"attempted":1,"sorted":[3.0,1.0]}"#)
            .unwrap_err();
        assert!(error.to_string().contains("more than 1"), "{error}");
    }

    #[test]
    fn bad_inputs_are_refused() {
        assert!(matches!(
            Distribution::new(vec![1.0, f64::NAN], 2),
            Err(AnalysisError::Domain { what: "value in a distribution", value }) if value.is_nan()
        ));
        assert!(matches!(
            Distribution::new(vec![1.0, 2.0], 1),
            Err(AnalysisError::Count {
                count: 2,
                limit: 1,
                ..
            })
        ));
        let d = Distribution::new(vec![1.0], 1).unwrap();
        for p in [-0.1, 1.1, f64::NAN] {
            assert!(matches!(
                d.quantile(p),
                Err(AnalysisError::Domain { what: "quantile probability", value })
                    if value.to_bits() == p.to_bits()
            ));
        }
    }

    /// The normal quantile on tabulated values: 1.959963984540054 at 0.975 (the two-sided 95%
    /// point), 1.2815515655446004 at 0.9 and 3.090232306167813 at 0.999, with their mirror
    /// images; 1 at Φ(1) = 0.8413447460685429; −6.361340902404056 at 10⁻¹⁰. It inverts Φ down
    /// to 10⁻¹⁰⁰, is mirrored about 1/2, never decreases, and gives the edges as the doc says.
    #[test]
    fn the_normal_quantile_on_tabulated_values_and_its_edges() {
        for (p, z) in [
            (0.975, 1.959_963_984_540_054),
            (0.9, 1.281_551_565_544_600_4),
            (0.999, 3.090_232_306_167_813),
            (0.841_344_746_068_542_9, 1.0),
        ] {
            assert!((normal_quantile(p) - z).abs() < 1e-13, "{p}");
            assert!((normal_quantile(1.0 - p) + z).abs() < 1e-13, "{p}");
        }
        assert!((normal_quantile(1e-10) + 6.361_340_902_404_056).abs() < 1e-12);
        // It inverts Φ, deep into the lower tail: the share at or below the result is at least
        // p, and a few steps of f64 below the result it is under p.
        let share = |x: f64| 0.5 * libm::erfc(-x / std::f64::consts::SQRT_2);
        for p in [1e-100, 1e-12, 0.005, 0.015, 0.25, 0.495, 0.5 - 1e-15] {
            let x = normal_quantile(p);
            assert!(x < 0.0, "{p}");
            assert!(share(x) >= p, "{p}: {}", share(x));
            assert!(share(x * (1.0 + 4.0 * f64::EPSILON)) < p, "{p}");
            assert_eq!(normal_quantile(1.0 - p), -normal_quantile(1.0 - (1.0 - p)));
        }
        // Never decreasing, through 1/2.
        let mut last = f64::NEG_INFINITY;
        for step in 0..=2000 {
            let x = normal_quantile(f64::from(step) / 2000.0);
            assert!(x >= last, "{step}");
            last = x;
        }
        assert_eq!(normal_quantile(0.5), 0.0);
        assert_eq!(normal_quantile(0.0), f64::NEG_INFINITY);
        assert_eq!(normal_quantile(1.0), f64::INFINITY);
        assert!(normal_quantile(-0.1).is_nan());
        assert!(normal_quantile(1.1).is_nan());
        assert!(normal_quantile(f64::NAN).is_nan());
        // The smallest positive number still has a quantile inside the bracket.
        assert!(normal_quantile(f64::MIN_POSITIVE) > -40.0);
    }
}
