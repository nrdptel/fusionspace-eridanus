//! The command line's units: SI first, with the US units a flyer in the United States reads in
//! brackets after it, `1065.1 m (3494 ft)` (decision records ADR-164 §6 and ADR-210).
//!
//! The FusionSpace product system's `data.md` (*Units for rocketry*) names the US unit of each
//! quantity: feet for heights and distances, feet per second for speeds, miles per hour for the
//! wind, inches for lengths and diameters, ounces and pounds for masses, and g for accelerations.
//! Thrust stays in newtons and impulse in newton-seconds, as motors are rated. The command line
//! keeps SI first and gives both at once, in place of a `--units` switch; the `--json` output and
//! the exports stay SI, their names carrying the unit (ADR-174).
//!
//! The conversions are exact by definition: the international foot (0.3048 m) and inch
//! (0.0254 m), the international mile (1609.344 m), the avoirdupois pound (0.45359237 kg) and
//! ounce (a sixteenth of it), and standard gravity (9.80665 m/s², the CGPM's 1901 value).
//!
//! Numbers keep the terminal's style (`cli.md`): no digit grouping, a hyphen-minus for a negative,
//! and never `-0`, which a value that rounds to zero would otherwise print.

/// Meters in a foot, exactly (the international foot).
pub(crate) const FOOT_M: f64 = 0.3048;
/// Meters in an inch, exactly.
pub(crate) const INCH_M: f64 = 0.0254;
/// Meters in a statute mile, exactly (the international mile).
pub(crate) const MILE_M: f64 = 1609.344;
/// Kilograms in an avoirdupois pound, exactly.
pub(crate) const POUND_KG: f64 = 0.453_592_37;
/// Kilograms in an avoirdupois ounce, exactly: a sixteenth of a pound.
pub(crate) const OUNCE_KG: f64 = POUND_KG / 16.0;
/// Standard gravity, m/s², exactly: the acceleration one g names.
pub(crate) const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;

/// `value` to `decimals` places, as the terminal prints it, with no `-0`: a negative value that
/// rounds to zero prints as zero, unsigned.
pub(crate) fn fixed(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    match text.strip_prefix('-') {
        Some(digits) if digits.chars().all(|c| c == '0' || c == '.') => digits.to_owned(),
        _ => text,
    }
}

/// A height or a distance in feet, whole: `3494 ft`.
pub(crate) fn feet(meters: f64) -> String {
    format!("{} ft", fixed(meters / FOOT_M, 0))
}

/// A speed in feet per second, whole: `86 ft/s`.
pub(crate) fn feet_per_second(meters_per_second: f64) -> String {
    format!("{} ft/s", fixed(meters_per_second / FOOT_M, 0))
}

/// A wind speed in miles per hour, whole: `13 mph`.
pub(crate) fn miles_per_hour(meters_per_second: f64) -> String {
    format!("{} mph", fixed(meters_per_second * 3600.0 / MILE_M, 0))
}

/// A length in inches, to `decimals` places: `35.9 in`.
pub(crate) fn inches(meters: f64, decimals: usize) -> String {
    format!("{} in", fixed(meters / INCH_M, decimals))
}

/// A mass in ounces to a tenth below a pound, in pounds to a hundredth from one: `6.4 oz`,
/// `4.63 lb`.
pub(crate) fn ounces_or_pounds(kilograms: f64) -> String {
    let pounds = kilograms / POUND_KG;
    // Decided on the printed figure, so 15.99 oz, which prints as 16.0 oz, reads as 1.00 lb.
    if fixed(pounds * 16.0, 1)
        .parse::<f64>()
        .is_ok_and(|oz| oz.abs() < 16.0)
    {
        format!("{} oz", fixed(kilograms / OUNCE_KG, 1))
    } else {
        format!("{} lb", fixed(pounds, 2))
    }
}

/// A height or a distance in meters to a tenth, with feet: `1065.1 m (3494 ft)`.
pub(crate) fn meters(value: f64) -> String {
    format!("{} m ({})", fixed(value, 1), feet(value))
}

/// A speed in meters per second to a tenth, with feet per second: `26.3 m/s (86 ft/s)`.
pub(crate) fn speed(value: f64) -> String {
    format!("{} m/s ({})", fixed(value, 1), feet_per_second(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Zero, a hair either side of it, and a negative that rounds to zero all print unsigned:
    /// never `-0` nor `-0.0`.
    #[test]
    fn zero_prints_unsigned() {
        assert_eq!(meters(0.0), "0.0 m (0 ft)");
        assert_eq!(meters(-0.0), "0.0 m (0 ft)");
        assert_eq!(meters(-0.04), "0.0 m (0 ft)");
        assert_eq!(meters(-0.1), "-0.1 m (0 ft)");
        assert_eq!(speed(-0.0), "0.0 m/s (0 ft/s)");
        assert_eq!(fixed(-0.0004, 3), "0.000");
        assert_eq!(fixed(-0.0005001, 3), "-0.001");
        assert_eq!(miles_per_hour(-0.1), "0 mph");
        assert_eq!(inches(-0.0001, 1), "0.0 in");
    }

    /// A negative keeps its sign on both sides once either rounds away from zero.
    #[test]
    fn negatives_keep_their_sign() {
        assert_eq!(meters(-100.0), "-100.0 m (-328 ft)");
        assert_eq!(speed(-30.48), "-30.5 m/s (-100 ft/s)");
        assert_eq!(meters(-0.2), "-0.2 m (-1 ft)");
    }

    /// Large values print every digit, ungrouped, as the terminal's numbers do.
    #[test]
    fn large_values_print_ungrouped() {
        assert_eq!(meters(1_000_000.0), "1000000.0 m (3280840 ft)");
        assert_eq!(speed(1234.5), "1234.5 m/s (4050 ft/s)");
        assert_eq!(feet(30_480.0), "100000 ft");
    }

    /// The conversions are the exact definitions: a foot, an inch, a mile an hour, a pound, an
    /// ounce and a g each read back as one.
    #[test]
    fn the_definitions_are_exact() {
        assert_eq!(meters(0.3048), "0.3 m (1 ft)");
        assert_eq!(meters(304.8), "304.8 m (1000 ft)");
        assert_eq!(speed(0.3048), "0.3 m/s (1 ft/s)");
        assert_eq!(inches(0.0254, 2), "1.00 in");
        assert_eq!(inches(0.9144, 1), "36.0 in");
        assert_eq!(miles_per_hour(1609.344 / 3600.0), "1 mph");
        assert_eq!(miles_per_hour(8.9408), "20 mph");
        assert_eq!(ounces_or_pounds(OUNCE_KG), "1.0 oz");
        assert_eq!(ounces_or_pounds(POUND_KG), "1.00 lb");
        // A g is standard gravity, as the plot's acceleration scale divides by it.
        assert_eq!(STANDARD_GRAVITY_M_S2.to_bits(), 9.806_65_f64.to_bits());
        assert_eq!(fixed(9.806_65 / STANDARD_GRAVITY_M_S2, 6), "1.000000");
        assert_eq!(fixed(98.066_5 / STANDARD_GRAVITY_M_S2, 6), "10.000000");
    }

    /// Rounding at a half: a value just below one half rounds down, just above it up, on both
    /// sides of the brackets; the units' rounding never disagrees with the SI figure's sign.
    #[test]
    fn rounding_at_a_half() {
        // 0.15239 m is 0.49997 ft, 0.15241 m is 0.50003 ft.
        assert_eq!(meters(0.15239), "0.2 m (0 ft)");
        assert_eq!(meters(0.15241), "0.2 m (1 ft)");
        assert_eq!(meters(-0.15241), "-0.2 m (-1 ft)");
        // A tenth of a meter at its half: 1.25 is exact in binary and rounds to even, 1.2.
        assert_eq!(fixed(1.25, 1), "1.2");
        assert_eq!(fixed(1.2500001, 1), "1.3");
    }

    /// Masses switch from ounces to pounds where the printed ounces reach 16, so no mass prints
    /// as `16.0 oz`.
    #[test]
    fn masses_switch_to_pounds_at_a_pound() {
        assert_eq!(ounces_or_pounds(0.1825), "6.4 oz");
        assert_eq!(ounces_or_pounds(15.9 * OUNCE_KG), "15.9 oz");
        assert_eq!(ounces_or_pounds(15.96 * OUNCE_KG), "1.00 lb");
        assert_eq!(ounces_or_pounds(2.1), "4.63 lb");
        assert_eq!(ounces_or_pounds(0.0), "0.0 oz");
        assert_eq!(ounces_or_pounds(100.0), "220.46 lb");
    }

    /// A value that isn't a number prints as one that isn't, on both sides, not as a figure.
    #[test]
    fn not_a_number_says_so() {
        assert_eq!(meters(f64::NAN), "NaN m (NaN ft)");
        assert_eq!(speed(f64::INFINITY), "inf m/s (inf ft/s)");
    }
}
