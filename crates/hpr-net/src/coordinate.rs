//! Coordinates as the request URLs write them, so a cache key survives a trip through radians.
//!
//! A [`Client`](crate::Client) caches an answer under its URL. A program that keeps a site in
//! radians, as `hpr_core::geodesy::Geodetic` does, and turns it back into degrees can get other
//! last digits (−106.91° comes back as −106.91000000000001); written with `f64`'s shortest digits,
//! that URL would miss the copy saved earlier. Every source that puts a coordinate in its URL
//! (the elevation service, Open-Meteo's weather, NOMADS' box) writes it through this module
//! instead: to [`URL_DECIMALS`] decimals, about 1 m on the ground.

/// The decimals each coordinate is written with in a URL: 1e-5° is at most 1.1 m on the ground.
pub(crate) const URL_DECIMALS: u32 = 5;

/// The decimals a coordinate is first written to, exactly, before it is rounded to
/// [`URL_DECIMALS`]. A trip through radians moves a value by a few units in its last place (under
/// 1e-13°), so a value given to 8 decimals or fewer is written the same after one.
const FIRST_DECIMALS: u32 = 9;

/// A coordinate in whole units of 10^-[`URL_DECIMALS`] degrees: its [`FIRST_DECIMALS`]-decimal
/// value rounded to [`URL_DECIMALS`] decimals, halves away from zero. A value that rounds to zero
/// is `0`, whatever its sign. A value given to more than 9 decimals is so rounded twice, by at
/// most 1e-5° in all.
///
/// Rounding the binary value straight to 5 decimals would let a trip through radians flip a value
/// that sits on a half step (32.990415 is 32.990415000000006 after one): it is written to
/// [`FIRST_DECIMALS`] first, exactly, and that decimal is rounded. `deg` is finite and within
/// ±180 (each caller checks its range first); any other value gives some number, not a panic.
pub(crate) fn units(deg: f64) -> i64 {
    let fixed = format!("{:.*}", FIRST_DECIMALS as usize, deg.abs());
    // The digits of `|deg|` in units of 10^-FIRST_DECIMALS: under 2e11 for a checked coordinate,
    // and saturating rather than overflowing for any other.
    let digits = fixed
        .bytes()
        .filter(u8::is_ascii_digit)
        .fold(0_u64, |n, b| {
            n.saturating_mul(10).saturating_add(u64::from(b - b'0'))
        });
    let step = 10_u64.pow(FIRST_DECIMALS - URL_DECIMALS);
    let kept = i64::try_from(digits.saturating_add(step / 2) / step).unwrap_or(i64::MAX);
    if deg < 0.0 { -kept } else { kept }
}

/// A coordinate as a URL writes it: [`units`] written with up to [`URL_DECIMALS`] decimals,
/// trailing zeros and a bare point dropped, and `0` for zero (never `-0`).
pub(crate) fn coordinate(deg: f64) -> String {
    let units = units(deg);
    let sign = if units < 0 { "-" } else { "" };
    let one = 10_u64.pow(URL_DECIMALS);
    let (whole, fraction) = (units.unsigned_abs() / one, units.unsigned_abs() % one);
    if fraction == 0 {
        return format!("{sign}{whole}");
    }
    let digits = format!("{fraction:0width$}", width = URL_DECIMALS as usize);
    format!("{sign}{whole}.{}", digits.trim_end_matches('0'))
}

/// A value in [`units`] rounded to hundredths of a degree, halves away from zero, and written with
/// exactly two decimals (`33.30`; `0.00` for a value that rounds to zero, never `-0.00`).
pub(crate) fn hundredths(units: i64) -> String {
    let step = 10_u64.pow(URL_DECIMALS - 2);
    let kept = units.unsigned_abs().saturating_add(step / 2) / step;
    let sign = if kept != 0 && units < 0 { "-" } else { "" };
    format!("{sign}{}.{:02}", kept / 100, kept % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values on a half step, given to 6 decimals (as a GPS gives them), and values given to 8,
    /// are written the same after a trip through radians. About 1 in 17 of the six-decimal values,
    /// all on a half step, would flip if rounded straight to 5 decimals (21,127 of 360,000 when
    /// measured), and 19 of the eight-decimal ones.
    #[test]
    fn half_steps_survive_a_round_trip_through_radians() {
        let mut changed = 0;
        let mut flips_if_rounded_straight = 0;
        for (units, scale) in (0..360_000)
            .map(|i: i64| (-179_999_995 + i * 1_000, 1e6))
            .chain((0..360_000).map(|i| (-17_999_999_999 + i * 99_999, 1e8)))
        {
            // Under 2^53, so exact as an `f64`.
            let deg = units as f64 / scale;
            let again = deg.to_radians().to_degrees();
            changed += usize::from(again.to_bits() != deg.to_bits());
            flips_if_rounded_straight += usize::from(format!("{deg:.5}") != format!("{again:.5}"));
            assert_eq!(coordinate(again), coordinate(deg), "{deg}");
        }
        assert!(changed > 10_000, "{changed}");
        assert!(
            flips_if_rounded_straight > 1_000,
            "{flips_if_rounded_straight}"
        );
    }

    #[test]
    fn coordinates_are_written_to_five_decimals() {
        assert_eq!(coordinate(0.0), "0");
        assert_eq!(coordinate(-0.0), "0");
        assert_eq!(coordinate(-0.000_001), "0");
        assert_eq!(coordinate(-0.000_01), "-0.00001");
        assert_eq!(coordinate(90.0), "90");
        assert_eq!(coordinate(-180.0), "-180");
        assert_eq!(coordinate(12.345_678), "12.34568");
        assert_eq!(coordinate(1.5), "1.5");
        // Half steps round away from zero, from the decimal value.
        assert_eq!(coordinate(32.990_415), "32.99042");
        assert_eq!(coordinate(-106.969_225), "-106.96923");
        assert_eq!(coordinate(-0.000_005), "-0.00001");
        assert_eq!(coordinate(0.000_004_999), "0");
        assert_eq!(coordinate(179.999_995), "180");
        assert_eq!(coordinate(-179.999_996), "-180");
        assert_eq!(coordinate(89.999_996), "90");
        assert_eq!(coordinate(-106.910_000_000_000_01), "-106.91");
    }

    #[test]
    fn units_are_signed_hundred_thousandths() {
        assert_eq!(units(0.0), 0);
        assert_eq!(units(-0.0), 0);
        assert_eq!(units(-0.000_004), 0);
        assert_eq!(units(32.99), 3_299_000);
        assert_eq!(units(-106.97), -10_697_000);
        assert_eq!(units(0.3), 30_000);
        assert_eq!(units(-180.0), -18_000_000);
    }

    #[test]
    fn hundredths_round_halves_away_from_zero() {
        assert_eq!(hundredths(3_329_000), "33.29");
        assert_eq!(hundredths(-10_727_000), "-107.27");
        assert_eq!(hundredths(3_330_000), "33.30");
        assert_eq!(hundredths(3_329_500), "33.30");
        assert_eq!(hundredths(3_329_499), "33.29");
        assert_eq!(hundredths(-3_329_500), "-33.30");
        assert_eq!(hundredths(-499), "0.00");
        assert_eq!(hundredths(-500), "-0.01");
        assert_eq!(hundredths(0), "0.00");
        assert_eq!(hundredths(9_000_000), "90.00");
        assert_eq!(hundredths(-18_030_000), "-180.30");
    }
}
