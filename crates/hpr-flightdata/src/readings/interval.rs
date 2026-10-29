//! The readings' 90% intervals ([ADR-257][adr-257] §1, [ADR-308][adr-308]).
//!
//! A reading's bound is the most its error can be, held on the tuning set; a 90% interval is
//! narrower, and says how far off the reading usually is. Each comes from the reading's own
//! bound: the interval runs from the reading less [`Shares::high`] of its bound to the reading
//! less [`Shares::low`] of it. The two shares are the 5% and 95% points of the reading's error
//! as a share of its bound, `(reading − truth)/bound`, measured on the synthetic flights' own
//! calibration set (2,000 flights on a seed of their own, each flight's own logger's log) and
//! written here to three significant figures. The point from each end is the `⌊0.05 (n + 1)⌋`-th
//! of the `n` sorted shares, the split-conformal rank (V. Vovk, A. Gammerman and G. Shafer,
//! *Algorithmic Learning in a Random World*, Springer, 2005, ch. 2; J. Lei et al.,
//! "Distribution-free predictive inference for regression", *JASA* 113(523), 1094–1111, 2018).
//! `cargo xtask synthetic` fits them again and fails if they differ from the values here, then
//! holds the intervals to the coverage test of [ADR-257][adr-257], calibrated intervals, on the
//! two test sets.
//!
//! **How far to trust it.** Over flights drawn as the synthetic ones are, a reading's interval
//! holds the truth about 90% of the time and misses about 5% on each side. The 90% is over their
//! days, spread evenly within 15 K of the standard atmosphere, and the misses aren't: most of the
//! height's error is the day's air, so no height missed on a day within 10 K of standard, while a
//! third of the days warmer than that read low enough to leave the truth above the interval, the
//! flattering side for a waiver (#1055). On the AltOS logs, the apogee time's interval misses
//! late about one time in six or seven (#1056). A real flight can lie outside what the sets draw, as a
//! day more than 15 K from standard does; its interval then says less than 90%. No real flight
//! has yet checked one (evidence row G34).
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-308]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md

use serde::{Deserialize, Serialize};

/// A 90% interval on a reading: on the synthetic flights, the truth lay below [`Interval::low`]
/// about 5% of the time and above [`Interval::high`] about 5%.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Interval {
    /// Its lower end, in the reading's unit.
    pub low: f64,
    /// Its upper end, in the reading's unit.
    pub high: f64,
}

/// A reading's error, `(reading − truth)/bound`, at its 5% and 95% points on the calibration set.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shares {
    /// The 5% point: an error lower than this is the truth above the interval.
    pub low: f64,
    /// The 95% point: an error higher than this is the truth below the interval.
    pub high: f64,
}

impl Shares {
    /// The 90% interval on `reading` with its `bound`: from `reading − high · bound` to
    /// `reading − low · bound`. `None` when either isn't finite, or the bound isn't positive.
    pub fn interval(self, reading: f64, bound: f64) -> Option<Interval> {
        let (low, high) = (reading - self.high * bound, reading - self.low * bound);
        (reading.is_finite() && bound.is_finite() && bound > 0.0 && low.is_finite())
            .then_some(Interval { low, high })
            .filter(|i| i.high.is_finite() && i.low <= i.high)
    }
}

/// The apogee height's shares of its bound (`Apogee::bound_m`), fitted on the calibration set.
pub const APOGEE_HEIGHT_SHARES: Shares = Shares {
    low: -0.63,
    high: 0.637,
};

/// The apogee time's shares of its bound (`Apogee::time_bound_s`), fitted on the calibration set.
pub const APOGEE_TIME_SHARES: Shares = Shares {
    low: -0.337,
    high: 0.225,
};

/// The rank from each end of `n` sorted shares that is the 5% or 95% point, `⌊0.05 (n + 1)⌋`,
/// counted from 1; at least 1.
pub fn tail_rank(n: usize) -> usize {
    ((n + 1) / 20).max(1)
}

/// The 5% and 95% points of `shares`, each the [`tail_rank`]-th from its end. `None` when there
/// are fewer than 20 shares or any isn't finite.
pub fn fit(shares: &[f64]) -> Option<Shares> {
    if shares.len() < 20 || shares.iter().any(|s| !s.is_finite()) {
        return None;
    }
    let mut sorted = shares.to_vec();
    sorted.sort_by(f64::total_cmp);
    let k = tail_rank(sorted.len());
    Some(Shares {
        low: sorted[k - 1],
        high: sorted[sorted.len() - k],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_interval_runs_from_the_high_share_to_the_low_one() {
        let shares = Shares {
            low: -0.2,
            high: 0.5,
        };
        // An error of +0.5 bound means the truth sits 0.5 bound below the reading: the low end.
        let i = shares.interval(100.0, 10.0).unwrap();
        assert_eq!((i.low, i.high), (95.0, 102.0));
        assert!(shares.interval(100.0, 0.0).is_none());
        assert!(shares.interval(100.0, -1.0).is_none());
        assert!(shares.interval(f64::NAN, 1.0).is_none());
        assert!(shares.interval(100.0, f64::INFINITY).is_none());
        // Shares that cross give no interval.
        let crossed = Shares {
            low: 0.5,
            high: -0.2,
        };
        assert!(crossed.interval(100.0, 10.0).is_none());
    }

    #[test]
    fn the_fit_takes_the_conformal_rank_from_each_end() {
        // 0, 1, …, 1999 shuffled: n = 2000, ⌊0.05 · 2001⌋ = 100, so the 100th from each end.
        let shares: Vec<f64> = (0..2000).map(|k| ((k * 7919) % 2000) as f64).collect();
        assert_eq!(tail_rank(2000), 100);
        let fitted = fit(&shares).unwrap();
        assert_eq!((fitted.low, fitted.high), (99.0, 1900.0));
        assert_eq!(tail_rank(19), 1);
        assert_eq!(tail_rank(39), 2);
        assert!(fit(&shares[..19]).is_none());
        let mut bad = shares.clone();
        bad[7] = f64::NAN;
        assert!(fit(&bad).is_none());
    }
}
