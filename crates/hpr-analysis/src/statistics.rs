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
//!   end, the quantile of every run tried is this one. Where `h` falls on or next to a missing
//!   run the quantile is [`Bound::Unbounded`]: the runs that gave a value can't bound it.
//!
//! Every sum runs over the sorted values in order, so a summary is bit-for-bit the same however
//! the values were computed, in parallel or not.

use serde::{Deserialize, Serialize};

use crate::error::AnalysisError;

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
    pub p05_counting_missing: Option<Bound>,
    /// The 95th percentile with every run that gave no value placed above the largest value;
    /// `None` when every run gave a value.
    pub p95_counting_missing: Option<Bound>,
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
pub enum Bound {
    /// The quantile, a value the runs gave or one between two of them.
    Value(f64),
    /// Too many runs gave no value for the rest to bound the quantile: it falls on or next to a
    /// missing run.
    Unbounded,
}

/// Bounds on the share of all the runs tried whose value passed a test: `low` counts a run with
/// no value as failing it, `high` as passing.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Share {
    /// The runs with a value that passed, over every run tried.
    pub low: f64,
    /// The same, with every run that gave no value counted as passing.
    pub high: f64,
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
    ) -> Result<Option<Bound>, AnalysisError> {
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
            MissingEnd::Below => j.checked_sub(missing).map(|i| self.sorted[i]),
        };
        // Cast: a count of runs is far below 2⁵³, and `h` lies in `[0, N − 1]`.
        let h = (self.attempted - 1) as f64 * p;
        let below = h.floor();
        let index = below as usize;
        let Some(low) = value(index) else {
            return Ok(Some(Bound::Unbounded));
        };
        if index + 1 == self.attempted {
            return Ok(Some(Bound::Value(low)));
        }
        match value(index + 1) {
            Some(above) => Ok(Some(Bound::Value(low + (h - below) * (above - low)))),
            // `h` on a value: the missing run next to it carries no weight.
            None if h == below => Ok(Some(Bound::Value(low))),
            None => Ok(Some(Bound::Unbounded)),
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
        // Cast: counts far below 2⁵³.
        let attempted = self.attempted as f64;
        Ok(Some(Share {
            low: passed as f64 / attempted,
            high: (passed + self.missing()) as f64 / attempted,
        }))
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(above(0.0), Some(Bound::Value(1.0)));
        assert_eq!(above(0.5), Some(Bound::Value(3.5)));
        // h = 3.0 sits on the 4, the last value: the missing run after it has no weight.
        assert_eq!(above(0.6), Some(Bound::Value(4.0)));
        assert_eq!(above(0.65), Some(Bound::Unbounded));
        assert_eq!(above(0.95), Some(Bound::Unbounded));
        // Below: −∞, −∞, 1, 2, 3, 4.
        assert_eq!(below(0.5), Some(Bound::Value(1.5)));
        assert_eq!(below(0.4), Some(Bound::Value(1.0)));
        assert_eq!(below(0.35), Some(Bound::Unbounded));
        assert_eq!(below(0.05), Some(Bound::Unbounded));
        assert_eq!(below(1.0), Some(Bound::Value(4.0)));
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
                Some(Bound::Unbounded)
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
                    Some(Bound::Value(plain)),
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
        assert_eq!(summary.p95_counting_missing, Some(Bound::Value(20.0)));
        assert_eq!(summary.p05_counting_missing, Some(Bound::Value(1.0)));
        // Two missing of 21: the top's interpolation reaches a missing run, and so does the
        // bottom's.
        let d = Distribution::new((1..=19).map(f64::from).collect(), 21).unwrap();
        let summary = d.summary();
        assert_eq!(summary.p95_counting_missing, Some(Bound::Unbounded));
        assert_eq!(summary.p05_counting_missing, Some(Bound::Unbounded));
    }

    #[test]
    fn a_bound_serializes_as_a_value_or_unbounded() {
        assert_eq!(
            serde_json::to_string(&Bound::Value(2.5)).unwrap(),
            r#"{"value":2.5}"#
        );
        assert_eq!(
            serde_json::to_string(&Bound::Unbounded).unwrap(),
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
}
