//! The flags a flight raises: the operating envelope's four, where a flight goes past what HPR Sim's
//! numbers have been checked for ([ADR-143 §1][adr-143], [ADR-179][adr-179]; the validation
//! plan's [Operating envelope][page]), and two for a rocket that is unstable while a motor burns
//! ([#335][i335]; the guide's [Unstable under power][unstable]), of which a flight raises at most
//! one.
//!
//! Every flight still flies; a flag only says that a number past its edge deserves less trust.
//!
//! - *unstable under power*: a static margin below zero, in the weakest plane, from the rail exit
//!   to apogee or the first deployment while a motor burns
//!   ([`FlightSummary::min_powered_static_margin_cal`](crate::FlightSummary::min_powered_static_margin_cal)).
//!   The air then turns the rocket away from its path rather than back, so the path flown, and
//!   its apogee, follow from small disturbances and from aerodynamics that hold only at small
//!   angles of attack: they are not a prediction.
//! - *unstable without a margin*: where HPR Sim can give no static margin while a motor burns (the
//!   net normal-force slope is not positive, or too small for the quotient to mean anything), a
//!   pitch-moment slope `C_mα` above zero
//!   ([`FlightSummary::max_powered_moment_slope_per_rad`](crate::FlightSummary::max_powered_moment_slope_per_rad)):
//!   the same instability, read from the moment instead. Raised only when the first isn't.
//!
//! The envelope's edges are by Mach number, and angle of attack is a separate condition:
//!
//! - *beyond the validated range*: faster than [`VALIDATED_MACH`], the fastest public flight HPR Sim
//!   has been compared with an independent reference on;
//! - *at high angle of attack*: above [`HIGH_ANGLE_OF_ATTACK_RAD`] (15°) more than
//!   [`HIGH_ANGLE_GRACE_S`] (1 s) after the rail exit and before apogee or the first deployment,
//!   while a 15° angle would give a normal force of at least [`HIGH_ANGLE_MIN_FORCE_SHARE`] of the
//!   rocket's weight: where the aerodynamics' small-angle assumptions stop holding and it matters;
//! - *outside the core band*: past [`CORE_BAND_MACH`] (2.5), where accuracy work goes second;
//! - *beyond the envelope*: past [`ENVELOPE_MACH`] (3.5), past commercial-motor flights.
//!
//! Each edge is exclusive: a flight that reaches it exactly, or whose margin or moment slope is
//! exactly zero, raises no flag. The flags come from a [`FlightSummary`](crate::FlightSummary):
//! its least static margin and largest moment slope without a margin under power, its top Mach
//! number, and its [`max_angle_of_attack_rad`](crate::FlightSummary::max_angle_of_attack_rad).
//!
//! [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
//! [adr-179]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0179-the-envelope-flags.md
//! [page]: https://hpr.fusionspace.co/VALIDATION.html#operating-envelope
//! [i335]: https://github.com/nrdptel/fusionspace-eridanus/issues/335
//! [unstable]: https://hpr.fusionspace.co/physics/metrics.html#unstable-under-power

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use serde::{Deserialize, Serialize};

use crate::metrics::Peak;

/// The fastest public whole flight compared with an independent reference: OpenRocket 24.12's
/// *Dual parachute deployment* example, Mach 1.1467, its own top Mach number in
/// `validation/reports/openrocket-flights.json`. A test in `hpr-validate` reads the committed
/// public reports (that one, and RocketPy's in `latest.json`) and fails when this differs from
/// their largest reference, so it rises as references are added ([the envelope's decision
/// record][adr-143], §1). Private flights never set it.
///
/// [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
pub const VALIDATED_MACH: f64 = 1.146_740_618_304_243_7;

/// The core band's top: Mach 2.5 ([the envelope's decision record][adr-143]). Accuracy work goes
/// below it first.
///
/// [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
pub const CORE_BAND_MACH: f64 = 2.5;

/// The envelope's top: Mach 3.5, about the fastest commercial-motor flights ([the envelope's
/// decision record][adr-143]).
///
/// [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
pub const ENVELOPE_MACH: f64 = 3.5;

/// The angle of attack that accuracy work assumes at most: 15°, in radians ([the envelope's
/// decision record][adr-143]).
///
/// [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
pub const HIGH_ANGLE_OF_ATTACK_RAD: f64 = 15.0_f64.to_radians();

/// How long after the rail exit a large angle of attack is the expected transient, not a flag:
/// 1 s, chosen rather than measured ([the envelope's decision record][adr-143];
/// [M1.14e, on large angles of attack][m1-14e], measures it).
///
/// [adr-143]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
/// [m1-14e]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14e
pub const HIGH_ANGLE_GRACE_S: f64 = 1.0;

/// The share of the rocket's weight below which an angle of attack doesn't count: at that
/// instant a 15° angle must give a normal force of at least a fifth of the weight. Chosen rather
/// than measured ([the envelope flags' decision record][adr-179]): below it gravity, not the air,
/// does most of the turning, so an error in that force moves the flight little. As the path turns
/// over near apogee the angle passes 15° on an ordinary flight where 15° gives 0.7% to 5.0% of
/// the weight (on the tests' RocketPy rockets off 84° and 85° rails); a wind layer met at speed,
/// 62% to 141%.
///
/// [adr-179]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0179-the-envelope-flags.md
pub const HIGH_ANGLE_MIN_FORCE_SHARE: f64 = 0.2;

/// The normal force, N, that a normal-force slope `normal_force_slope_per_rad` on a reference
/// diameter `reference_diameter_m` gives at an angle of attack `angle_of_attack_rad` in a
/// dynamic pressure `dynamic_pressure_pa`: `q · (π d²/4) · |C_Nα| · α`, the linear model, which
/// is what the aerodynamics assume.
#[must_use]
pub fn normal_force_n(
    dynamic_pressure_pa: f64,
    reference_diameter_m: f64,
    normal_force_slope_per_rad: f64,
    angle_of_attack_rad: f64,
) -> f64 {
    let area_m2 = std::f64::consts::PI * reference_diameter_m.powi(2) / 4.0;
    dynamic_pressure_pa * area_m2 * normal_force_slope_per_rad.abs() * angle_of_attack_rad
}

/// Whether an instant counts on a rocket of `mass_kg` where a 15° angle gives a normal force of
/// `normal_force_n`: at least [`HIGH_ANGLE_MIN_FORCE_SHARE`] of its weight in standard gravity.
/// A NaN counts, so a broken number can't hide a flag.
#[must_use]
pub fn force_counts(normal_force_n: f64, mass_kg: f64) -> bool {
    normal_force_n.is_nan()
        || normal_force_n >= HIGH_ANGLE_MIN_FORCE_SHARE * mass_kg * STANDARD_GRAVITY_MPS2
}

/// One flag a flight raises, with the peak that raised it. Serialized with a `flag` tag in
/// snake case.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "flag", rename_all = "snake_case")]
pub enum EnvelopeFlag {
    /// A static margin below zero while a motor burns, from the rail exit to apogee or the first
    /// deployment
    /// ([`FlightSummary::min_powered_static_margin_cal`](crate::FlightSummary::min_powered_static_margin_cal)):
    /// the rocket is unstable under power, and its apogee is not a prediction.
    UnstableUnderPower {
        /// The least static margin while a motor burns, calibres.
        static_margin_cal: Peak,
    },
    /// A pitch-moment slope `C_mα` above zero while a motor burns, from the rail exit to apogee or
    /// the first deployment, at an instant where HPR Sim can give no static margin
    /// ([`FlightSummary::max_powered_moment_slope_per_rad`](crate::FlightSummary::max_powered_moment_slope_per_rad)):
    /// the air turns the rocket away from its path, so it is unstable under power and its apogee
    /// is not a prediction. Raised only when [`UnstableUnderPower`](Self::UnstableUnderPower)
    /// isn't, so a flight raises one unstable flag at most.
    UnstableWithoutMargin {
        /// The largest static pitch-moment slope where the margin is undefined while a motor
        /// burns, per radian.
        pitch_moment_slope_per_rad: Peak,
    },
    /// Faster than [`VALIDATED_MACH`].
    BeyondValidatedRange {
        /// The flight's top Mach number.
        max_mach: Peak,
    },
    /// Above [`HIGH_ANGLE_OF_ATTACK_RAD`] more than [`HIGH_ANGLE_GRACE_S`] after the rail exit and
    /// before apogee or the first deployment, where a 15° angle would give a normal force of at
    /// least [`HIGH_ANGLE_MIN_FORCE_SHARE`] of the weight
    /// ([`FlightSummary::max_angle_of_attack_rad`](crate::FlightSummary::max_angle_of_attack_rad)).
    HighAngleOfAttack {
        /// The largest angle of attack that counts, rad.
        angle_of_attack_rad: Peak,
    },
    /// Past [`CORE_BAND_MACH`].
    OutsideCoreBand {
        /// The flight's top Mach number.
        max_mach: Peak,
    },
    /// Past [`ENVELOPE_MACH`].
    BeyondEnvelope {
        /// The flight's top Mach number.
        max_mach: Peak,
    },
}

impl EnvelopeFlag {
    /// The flag's name in snake case, as it is serialized.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::UnstableUnderPower { .. } => "unstable_under_power",
            Self::UnstableWithoutMargin { .. } => "unstable_without_margin",
            Self::BeyondValidatedRange { .. } => "beyond_validated_range",
            Self::HighAngleOfAttack { .. } => "high_angle_of_attack",
            Self::OutsideCoreBand { .. } => "outside_core_band",
            Self::BeyondEnvelope { .. } => "beyond_envelope",
        }
    }

    /// The peak that raised the flag.
    #[must_use]
    pub fn peak(&self) -> Peak {
        match *self {
            Self::BeyondValidatedRange { max_mach }
            | Self::OutsideCoreBand { max_mach }
            | Self::BeyondEnvelope { max_mach } => max_mach,
            Self::HighAngleOfAttack {
                angle_of_attack_rad,
            } => angle_of_attack_rad,
            Self::UnstableUnderPower { static_margin_cal } => static_margin_cal,
            Self::UnstableWithoutMargin {
                pitch_moment_slope_per_rad,
            } => pitch_moment_slope_per_rad,
        }
    }

    /// What the flag means for this flight, in a sentence.
    #[must_use]
    pub fn message(&self) -> String {
        match *self {
            Self::UnstableUnderPower { static_margin_cal } => format!(
                "the static margin falls to {:.2} calibres at {:.2} s while a motor burns: the \
                 rocket is unstable under power, so the air turns it away from its path rather \
                 than back, and its apogee is not a prediction",
                static_margin_cal.value, static_margin_cal.time_s
            ),
            Self::UnstableWithoutMargin {
                pitch_moment_slope_per_rad,
            } => format!(
                "while a motor burns, the pitching moment turns the rocket away from its path \
                 (C_mα {:+.1} per radian at {:.2} s) where HPR Sim can give no static margin: the \
                 rocket is unstable under power, so its apogee is not a prediction",
                pitch_moment_slope_per_rad.value, pitch_moment_slope_per_rad.time_s
            ),
            Self::BeyondValidatedRange { max_mach } => format!(
                "reaches Mach {:.2} at {:.1} s, faster than any public flight HPR Sim has been \
                 compared with an independent reference (Mach {VALIDATED_MACH:.2}): its numbers \
                 past that speed are unchecked",
                max_mach.value, max_mach.time_s
            ),
            Self::HighAngleOfAttack {
                angle_of_attack_rad,
            } => format!(
                "flies at {:.1}° angle of attack at {:.1} s, past the {:.0}° its aerodynamics \
                 assume more than {HIGH_ANGLE_GRACE_S:.0} s after the rail exit: its numbers \
                 from then on are rough",
                angle_of_attack_rad.value.to_degrees(),
                angle_of_attack_rad.time_s,
                HIGH_ANGLE_OF_ATTACK_RAD.to_degrees()
            ),
            Self::OutsideCoreBand { max_mach } => format!(
                "reaches Mach {:.2} at {:.1} s, past the core band (Mach 0 to \
                 {CORE_BAND_MACH:.1}): its aerodynamics there are checked less",
                max_mach.value, max_mach.time_s
            ),
            Self::BeyondEnvelope { max_mach } => format!(
                "reaches Mach {:.2} at {:.1} s, beyond the envelope (Mach {ENVELOPE_MACH:.1}): \
                 its aerodynamics there are not validated",
                max_mach.value, max_mach.time_s
            ),
        }
    }
}

/// Whether `value` is past `edge`, a NaN counting as past it.
fn past(value: f64, edge: f64) -> bool {
    value.is_nan() || value > edge
}

/// Whether `value` is below `edge`, a NaN counting as below it.
fn below(value: f64, edge: f64) -> bool {
    value.is_nan() || value < edge
}

/// The flags raised by a flight whose top Mach number is `max_mach`, whose largest counted
/// angle of attack is `max_angle_of_attack_rad`, whose least static margin while a motor burns
/// is `min_powered_static_margin_cal`, and whose largest static pitch-moment slope where that
/// margin is undefined while a motor burns is `max_powered_moment_slope_per_rad`, in
/// [`EnvelopeFlag`]'s order. Each holds on its own, so a flight past Mach 3.5 raises all three
/// Mach flags, but of the two unstable flags only the first raised counts: a margin below zero
/// wins over a moment slope above it. A NaN peak raises its flags, so a broken number can't hide
/// one.
#[must_use]
pub fn flags(
    max_mach: Option<Peak>,
    max_angle_of_attack_rad: Option<Peak>,
    min_powered_static_margin_cal: Option<Peak>,
    max_powered_moment_slope_per_rad: Option<Peak>,
) -> Vec<EnvelopeFlag> {
    let mut flags = Vec::new();
    if let Some(static_margin_cal) =
        min_powered_static_margin_cal.filter(|peak| below(peak.value, 0.0))
    {
        flags.push(EnvelopeFlag::UnstableUnderPower { static_margin_cal });
    } else if let Some(pitch_moment_slope_per_rad) =
        max_powered_moment_slope_per_rad.filter(|peak| past(peak.value, 0.0))
    {
        flags.push(EnvelopeFlag::UnstableWithoutMargin {
            pitch_moment_slope_per_rad,
        });
    }
    if let Some(max_mach) = max_mach.filter(|peak| past(peak.value, VALIDATED_MACH)) {
        flags.push(EnvelopeFlag::BeyondValidatedRange { max_mach });
    }
    if let Some(angle_of_attack_rad) =
        max_angle_of_attack_rad.filter(|peak| past(peak.value, HIGH_ANGLE_OF_ATTACK_RAD))
    {
        flags.push(EnvelopeFlag::HighAngleOfAttack {
            angle_of_attack_rad,
        });
    }
    if let Some(max_mach) = max_mach.filter(|peak| past(peak.value, CORE_BAND_MACH)) {
        flags.push(EnvelopeFlag::OutsideCoreBand { max_mach });
    }
    if let Some(max_mach) = max_mach.filter(|peak| past(peak.value, ENVELOPE_MACH)) {
        flags.push(EnvelopeFlag::BeyondEnvelope { max_mach });
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peak(value: f64) -> Option<Peak> {
        Some(Peak {
            value,
            time_s: 3.0,
            height_above_ground_m: 400.0,
        })
    }

    fn names(max_mach: f64, angle_rad: f64) -> Vec<&'static str> {
        flags(peak(max_mach), peak(angle_rad), peak(1.5), peak(-2.0))
            .iter()
            .map(EnvelopeFlag::name)
            .collect()
    }

    #[test]
    fn each_mach_flag_starts_just_past_its_edge() {
        let calm = 0.1;
        for (edge, raised) in [
            (VALIDATED_MACH, vec!["beyond_validated_range"]),
            (
                CORE_BAND_MACH,
                vec!["beyond_validated_range", "outside_core_band"],
            ),
            (
                ENVELOPE_MACH,
                vec![
                    "beyond_validated_range",
                    "outside_core_band",
                    "beyond_envelope",
                ],
            ),
        ] {
            let below = &raised[..raised.len() - 1];
            assert_eq!(names(edge, calm), below, "at Mach {edge}");
            assert_eq!(names(edge.next_up(), calm), raised, "past Mach {edge}");
        }
        assert!(names(VALIDATED_MACH.next_down(), calm).is_empty());
    }

    #[test]
    fn the_angle_flag_starts_just_past_15_degrees() {
        assert_eq!(HIGH_ANGLE_OF_ATTACK_RAD, 0.261_799_387_799_149_4);
        assert!(names(0.5, HIGH_ANGLE_OF_ATTACK_RAD).is_empty());
        assert_eq!(
            names(0.5, HIGH_ANGLE_OF_ATTACK_RAD.next_up()),
            ["high_angle_of_attack"]
        );
    }

    #[test]
    fn nothing_flown_raises_nothing_and_a_nan_raises_everything() {
        assert!(flags(None, None, None, None).is_empty());
        let nan = flags(
            peak(f64::NAN),
            peak(f64::NAN),
            peak(f64::NAN),
            peak(f64::NAN),
        );
        assert_eq!(
            nan.iter().map(EnvelopeFlag::name).collect::<Vec<_>>(),
            [
                "unstable_under_power",
                "beyond_validated_range",
                "high_angle_of_attack",
                "outside_core_band",
                "beyond_envelope"
            ]
        );
    }

    #[test]
    fn the_force_floor_is_a_fifth_of_the_weight() {
        let mass_kg = 3.7;
        let fifth_n = HIGH_ANGLE_MIN_FORCE_SHARE * mass_kg * 9.806_65;
        assert!(force_counts(fifth_n, mass_kg));
        assert!(!force_counts(fifth_n.next_down(), mass_kg));
        assert!(force_counts(f64::NAN, mass_kg));
        // q (π d²/4) |C_Nα| α: 2000 Pa on a 0.1 m diameter, a slope of −10 per radian, 0.3 rad.
        let force = normal_force_n(2000.0, 0.1, -10.0, 0.3);
        assert!(
            (force - 2000.0 * 0.007_853_981_633_974_483 * 3.0).abs() < 1e-12,
            "{force}"
        );
    }

    #[test]
    fn the_stability_flag_starts_just_below_a_zero_margin() {
        // Below zero, however little, under power: unstable. Zero itself, or a stable margin,
        // raises nothing. Which margin counts (only while a motor burns) is the watcher's, tested
        // in `metrics`.
        let unstable = |margin_cal: f64| {
            flags(None, None, peak(margin_cal), None)
                .iter()
                .map(EnvelopeFlag::name)
                .collect::<Vec<_>>()
        };
        assert_eq!(unstable((-0.0_f64).next_down()), ["unstable_under_power"]);
        assert_eq!(unstable(-4.21), ["unstable_under_power"]);
        assert!(unstable(0.0).is_empty());
        assert!(unstable(-0.0).is_empty());
        assert!(unstable(0.0_f64.next_up()).is_empty());
        assert!(unstable(1.5).is_empty());
        let flag = flags(None, None, peak(-4.21), None)[0];
        assert_eq!(flag.peak(), peak(-4.21).unwrap());
        assert_eq!(
            flag.message(),
            "the static margin falls to -4.21 calibres at 3.00 s while a motor burns: the rocket \
             is unstable under power, so the air turns it away from its path rather than back, \
             and its apogee is not a prediction"
        );
        let json = serde_json::to_value(flag).unwrap();
        assert_eq!(json["flag"], "unstable_under_power");
        assert_eq!(json["static_margin_cal"]["value"], -4.21);
        let back: EnvelopeFlag = serde_json::from_value(json).unwrap();
        assert_eq!(back, flag);
    }

    #[test]
    fn the_moment_flag_starts_just_above_zero_and_only_without_the_margin_flag() {
        // With no margin to read, a moment slope above zero, however little, under power:
        // unstable. Zero itself, or a restoring slope, raises nothing. Which slopes count (only
        // while a motor burns, only where the margin is undefined) is the watcher's, tested in
        // `metrics`.
        let unstable = |margin_cal: Option<Peak>, slope: f64| {
            flags(None, None, margin_cal, peak(slope))
                .iter()
                .map(EnvelopeFlag::name)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            unstable(None, 0.0_f64.next_up()),
            ["unstable_without_margin"]
        );
        assert_eq!(unstable(None, 43.9), ["unstable_without_margin"]);
        assert_eq!(unstable(None, f64::NAN), ["unstable_without_margin"]);
        assert!(unstable(None, 0.0).is_empty());
        assert!(unstable(None, -0.0).is_empty());
        assert!(unstable(None, (-0.0_f64).next_down()).is_empty());
        // A stable margin elsewhere under power doesn't hide it.
        assert_eq!(unstable(peak(1.5), 43.9), ["unstable_without_margin"]);
        // One unstable flag a flight: a margin below zero wins.
        assert_eq!(unstable(peak(-4.21), 43.9), ["unstable_under_power"]);
        assert_eq!(unstable(peak(f64::NAN), 43.9), ["unstable_under_power"]);
        let flag = flags(None, None, None, peak(43.9))[0];
        assert_eq!(flag.peak(), peak(43.9).unwrap());
        assert_eq!(
            flag.message(),
            "while a motor burns, the pitching moment turns the rocket away from its path (C_mα \
             +43.9 per radian at 3.00 s) where hpr can give no static margin: the rocket is \
             unstable under power, so its apogee is not a prediction"
        );
        let json = serde_json::to_value(flag).unwrap();
        assert_eq!(json["flag"], "unstable_without_margin");
        assert_eq!(json["pitch_moment_slope_per_rad"]["value"], 43.9);
        let back: EnvelopeFlag = serde_json::from_value(json).unwrap();
        assert_eq!(back, flag);
    }

    #[test]
    fn a_flag_keeps_its_peak_and_serializes_by_name() {
        let flag = flags(peak(1.5), None, None, None)[0];
        assert_eq!(flag.peak(), peak(1.5).unwrap());
        assert!(flag.message().contains("Mach 1.50 at 3.0 s"));
        let json = serde_json::to_value(flag).unwrap();
        assert_eq!(json["flag"], "beyond_validated_range");
        assert_eq!(json["max_mach"]["value"], 1.5);
        let back: EnvelopeFlag = serde_json::from_value(json).unwrap();
        assert_eq!(back, flag);
        let angle = flags(None, peak(0.5), None, None)[0];
        assert_eq!(serde_json::to_value(angle).unwrap()["flag"], angle.name());
        assert!(angle.message().contains("28.6° angle of attack at 3.0 s"));
    }
}
