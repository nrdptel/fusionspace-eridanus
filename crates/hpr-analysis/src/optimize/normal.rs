//! The standard normal distribution's cumulative distribution function `Φ`, for the integer
//! variables' margin ([`super::cmaes`]) and the expected improvement ([`super::ego`]); its
//! inverse is [`crate::statistics::normal_quantile`].
//!
//! `Φ(x) = erfc(−x/√2)/2`, with `erfc` from the `libm` crate (a port of musl's, itself
//! FreeBSD's), which keeps the lower tail's relative accuracy where `1 − erf` would cancel.

/// `Φ(x)`, the chance that a standard normal draw is at most `x`.
pub(crate) fn cdf(x: f64) -> f64 {
    0.5 * libm::erfc(-x / std::f64::consts::SQRT_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::statistics::normal_quantile as quantile;

    /// `Φ(Φ⁻¹(p)) = p` across the range, in the lower tail to its relative accuracy.
    #[test]
    fn cdf_inverts_the_quantile() {
        for k in 1..2000 {
            let p = f64::from(k) / 2000.0;
            let back = cdf(quantile(p));
            assert!((back - p).abs() <= 4e-16, "Φ(Φ⁻¹({p})) = {back}");
        }
        // In the tail a relative error δ in `x` moves `Φ(x)` by about `x² δ` of itself.
        for e in 1..=300 {
            let p = 10f64.powi(-e);
            let x = quantile(p);
            let back = cdf(x);
            assert!(
                (back - p).abs() <= 1e-15 * (1.0 + x * x) * p,
                "Φ(Φ⁻¹({p})) = {back}"
            );
        }
        assert_eq!(cdf(0.0), 0.5);
        assert!((cdf(1.0) - 0.841_344_746_068_542_9).abs() <= 2e-16);
    }
}
