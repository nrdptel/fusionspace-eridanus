//! Warnings for known errors in hpr's drag, each by its issue number: where a flight's speed and
//! its design's shape meet the condition under which a number is known to read wrong
//! ([ADR-163 §4][adr-163], [ADR-180][adr-180]).
//!
//! Drag that reads high makes three numbers flatter than they are: the apogee reads low against
//! a waiver's ceiling; the top speed reads low, so the flutter margin and the largest dynamic
//! pressure look better; the drift reads small. Each warning names them. Every flight still
//! flies; a warning only says which numbers to trust less, and which way they lean. A flight
//! flown on a drag of its own (a table or a model in place of hpr's) meets none of these.
//!
//! | issue | the error | the condition |
//! |---|---|---|
//! | [#67][i67] | the closed-form cone's pressure drag reads high, transonic | a nose or a shoulder whose shape takes it ([`takes_cone_formula`]), past Mach 0.8 |
//! | [#68][i68] | base drag reads high from Mach 0.8 to 1.2, low from 1.5 | every rocket, past Mach 0.8 |
//! | [#70][i70] | a sharp fin section's drag reads high, supersonic | an `Airfoil` fin set, past Mach 1 |
//! | [#72][i72] | a steep boattail's drag reads high, supersonic | a boattail steeper than 10°, past Mach 1 |
//! | [#222][i222] | supersonic pressure drag about twice OpenRocket's | an `Ogive` nose or an `Airfoil` fin set, past Mach 1 |
//! | [#73][i73] | a boattail's drag by Niskanen's subsonic rule, up to +41.7% on a measured one | a boattail steeper than 9.46° ([`SUBSONIC_BOATTAIL_RULE_RAD`]), any flight |
//! | [#18][i18] | skin friction taken as fully turbulent reads high on a surface smooth enough for a laminar run | every flight ([`friction_issue_warnings`]) |
//!
//! Others warn where the margin reads high ([`stability_issue_warnings`],
//! [`layout_stability_issue_warnings`]):
//! a forward-swept fin past Mach 1 ([#64][i64]), fin sets sharing a station ([#325][i325]), a
//! freeform fin set at any speed ([#326][i326], [ADR-190][adr-190]), the
//! supersonic switches (#87, #120, #121) and #172 on every flight ([ADR-189][adr-189]).
//!
//! Two more warn where a part flies on its own after a separation, with only its devices' drag
//! ([`separated_part_issue_warnings`]): a booster ([#179][i179]), and a part that starts with
//! nothing open ([#354][i354]). They read the flight's bodies, not its drag, so a flight on a drag
//! of its own meets them too.
//!
//! Four more have signs that depend on the case, flattering on some flights and not on others
//! ([ADR-188 §2][adr-188], [ADR-201][adr-201]), and warn whatever the rocket's drag. They bear on
//! the flight's path, the apogee and the drift, rather than on the drag or the margin
//! ([`IssueKind::Flight`]):
//!
//! | issue | the error | the condition |
//! |---|---|---|
//! | [#219][i219] | hpr flies the moment the thrust makes about a center of gravity off the axis; OpenRocket doesn't, and no reference sizes it | the center of gravity more than [`OFF_AXIS_CG_LIMIT_M`] off the axis at launch, at a motor's ignition or burnout, at a separation, or fully burnt, on the stack or a sustainer ([`center_of_gravity_issue_warnings`]) |
//! | [#213][i213] | a single pod's drag and normal force act on the axis, so their trim and roll are left out | a pod set of one pod off the axis that holds a part ([`pod_issue_warnings`]) |
//! | [#106][i106] | a body part's pitch and yaw damping is sampled away from its own center of pressure | the shock-expansion join, past its start on the stack or on a sustainer, each by its own join and its own top speed ([`body_station_issue_warnings`]) |
//! | [#8][i8] | a near-calm wind level's direction still swings the interpolated direction | a tabulated level at or below [`NEAR_CALM_WIND_M_S`] turning against its neighbour, inside the flight's heights ([`wind_issue_warnings`]) |
//!
//! Each Mach edge is exclusive, as the envelope's are ([`crate::envelope`]): a flight whose top
//! Mach number is exactly at an edge raises nothing, and a NaN top Mach number raises every
//! warning whose shape the design has, so a broken number can't hide one.
//!
//! [adr-163]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0163-the-second-pass-products-and-priorities.md
//! [adr-180]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0180-the-drag-issue-warnings.md
//! [i67]: https://github.com/nrdptel/fusionspace-eridanus/issues/67
//! [i68]: https://github.com/nrdptel/fusionspace-eridanus/issues/68
//! [i70]: https://github.com/nrdptel/fusionspace-eridanus/issues/70
//! [i72]: https://github.com/nrdptel/fusionspace-eridanus/issues/72
//! [i222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
//! [i64]: https://github.com/nrdptel/fusionspace-eridanus/issues/64
//! [i73]: https://github.com/nrdptel/fusionspace-eridanus/issues/73
//! [i325]: https://github.com/nrdptel/fusionspace-eridanus/issues/325
//! [i18]: https://github.com/nrdptel/fusionspace-eridanus/issues/18
//! [i326]: https://github.com/nrdptel/fusionspace-eridanus/issues/326
//! [i179]: https://github.com/nrdptel/fusionspace-eridanus/issues/179
//! [i354]: https://github.com/nrdptel/fusionspace-eridanus/issues/354
//! [i219]: https://github.com/nrdptel/fusionspace-eridanus/issues/219
//! [i213]: https://github.com/nrdptel/fusionspace-eridanus/issues/213
//! [i106]: https://github.com/nrdptel/fusionspace-eridanus/issues/106
//! [i8]: https://github.com/nrdptel/fusionspace-eridanus/issues/8
//! [adr-188]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0188-release-notes-and-the-flattering-side-survey.md
//! [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
//! [adr-190]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0190-the-friction-and-freeform-fin-warnings.md
//! [adr-189]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0189-the-safety-and-shape-warnings.md

use hpr_aero::AeroModel;
use hpr_aero::nose_drag::takes_cone_formula;
use hpr_atmos::DirectionTurn;
use hpr_design::{
    FinCrossSection, FinPlanform, Layout, MassProperties, NoseShape, Part, PlacedComponent,
};
use serde::{Deserialize, Serialize};

use crate::FlightResult;
use crate::metrics::Peak;

/// Where the closed-form cone's transonic pressure drag starts to read high: Mach 0.8
/// ([issue #67](https://github.com/nrdptel/fusionspace-eridanus/issues/67): +87% at Mach 0.8 and +105% at
/// 0.85 against Stoney's measured 3:1 cone, NASA TR R-100; 2 to 9 times MIL-HDBK-762's 3:1
/// tangent ogive from Mach 0.9 to 1.2; still +15% at Mach 1.5).
pub const TRANSONIC_NOSE_DRAG_MACH: f64 = 0.8;

/// Where base drag starts to read high: Mach 0.8 ([issue #68](https://github.com/nrdptel/fusionspace-eridanus/issues/68):
/// Fleeman's formula gives 0.225 at Mach 0.9 where MIL-HDBK-762's measured data give 0.156).
pub const BASE_DRAG_HIGH_MACH: f64 = 0.8;

/// The last Mach number where base drag is measured to read high: 1.2 (0.208 against
/// MIL-HDBK-762's 0.194, [issue #68](https://github.com/nrdptel/fusionspace-eridanus/issues/68)). Past it a
/// flight's warning adds the low side ([`BASE_DRAG_LOW_MACH`]); between the two the sign is
/// unmeasured.
pub const BASE_DRAG_HIGH_TOP_MACH: f64 = 1.2;

/// Where base drag is measured to read low: from Mach 1.5 (5% under Love's correlation, NACA
/// TN 3819, at Mach 1.5, 17% at 2.0, 14% at 3.0; [issue #68](https://github.com/nrdptel/fusionspace-eridanus/issues/68)).
/// There the apogee reads high.
pub const BASE_DRAG_LOW_MACH: f64 = 1.5;

/// Where the supersonic drag warnings start: Mach 1 (issues
/// [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70),
/// [#72](https://github.com/nrdptel/fusionspace-eridanus/issues/72) and
/// [#222](https://github.com/nrdptel/fusionspace-eridanus/issues/222)). Their measured gaps start at Mach 1.0
/// to 1.5; the warning starts at the lowest.
pub const SUPERSONIC_DRAG_MACH: f64 = 1.0;

/// A boattail steeper than this, in radians, warns past [`SUPERSONIC_DRAG_MACH`]: 10°, the
/// steepest that [issue #72](https://github.com/nrdptel/fusionspace-eridanus/issues/72) measured inside its
/// spread (−6.3% to +17.4% at 10° and below). It measured 15° at +13.5% to +50.8% and 16° at
/// +26.4% to +54.1%; between 10° and 15° is unmeasured, so the warning takes it in rather than
/// leave a high drag unwarned.
pub const STEEP_BOATTAIL_RAD: f64 = 10.0_f64.to_radians();

/// Where the supersonic body switches start to matter: Mach 1.2, the earliest the
/// shock-expansion method joins slender-body theory ([`hpr_aero::SUPERSONIC_JOIN_START_MACH`]).
pub const SUPERSONIC_SWITCH_MACH: f64 = hpr_aero::SUPERSONIC_JOIN_START_MACH;

/// How far the static margin read high against OpenRocket 24.12, calibres: the largest gap on
/// the private designs, 0.1108 (four designs read 0.0350 to 0.1108 calibres more stable in hpr,
/// none with a measured cause; *How far to trust the margin* on
/// [the stability page](https://hpr.fusionspace.co/stability-for-certification.html#how-far-to-trust-the-margin),
/// [issue #172](https://github.com/nrdptel/fusionspace-eridanus/issues/172)).
pub const MARGIN_HIGH_BOUND_CAL: f64 = 0.1108;

/// Where a forward-swept fin's warning starts: Mach 1 ([ADR-188][adr-188]). Its measured error is
/// a normal-force slope that rises by up to 7.7% past linear theory's start, Mach 1.2
/// ([`hpr_aero::fins::SUPERSONIC_START_MACH`]), peaking up to 0.37 past it
/// ([issue #64](https://github.com/nrdptel/fusionspace-eridanus/issues/64)); the warning starts at the lowest
/// supersonic Mach number, below that rise. The transonic join into linear theory starts at Mach
/// 0.8 ([`hpr_aero::fins::TRANSONIC_START_MACH`]); below Mach 1 the error isn't measured.
///
/// [adr-188]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0188-release-notes-and-the-flattering-side-survey.md
pub const FORWARD_SWEEP_MACH: f64 = 1.0;

/// The steepest boattail that Niskanen's subsonic rule gives no drag, rad: where its length
/// ratio `γ = l/(d₁ − d₂)` is 3, so `atan(1/6)`, 9.46° (Niskanen 2009 eq. 3.88,
/// [`hpr_aero::drag::boattail_factor`]). A steeper one takes a share of base drag below Mach 0.8,
/// which read from −40.8% to +41.7% on Cubbage's measured 16° boattails and high on the Arcas
/// Robin's 15° one ([issue #73](https://github.com/nrdptel/fusionspace-eridanus/issues/73)); between 9.46° and
/// 15° is unmeasured, so the warning takes it in.
pub const SUBSONIC_BOATTAIL_RULE_RAD: f64 = 0.165_148_677_414_626_83;

/// The most fins that one station can hold before the fin–fin interference factor drops below
/// 1: four (Niskanen 2009 table 3.3, [`hpr_aero::fins::fin_count_factor`]). Fin sets at one station
/// with more than this between them are counted apart by hpr and together by OpenRocket
/// ([issue #325](https://github.com/nrdptel/fusionspace-eridanus/issues/325)).
pub const FINS_WITHOUT_INTERFERENCE: u32 = 4;

/// How much of the margin gap on OpenRocket's *Pods--airframes and winglets* example counting
/// fin sets apart explains, calibres: about 0.029 of 0.071 to 0.076
/// ([issue #325](https://github.com/nrdptel/fusionspace-eridanus/issues/325)).
pub const FIN_SETS_AT_ONE_STATION_CAL: f64 = 0.029;

/// How much of the friction a laminar run takes off on RocketPy's Calisto at Mach 0.3, where
/// hpr takes it fully turbulent ([issue #18](https://github.com/nrdptel/fusionspace-eridanus/issues/18)):
/// Barrowman 1967's transitional term `1700/R` (eq. 4-6) at `R = 1.8e7`, against Niskanen 2009's
/// smooth turbulent coefficient `1/(1.50 ln R − 5.6)²` (eq. 3.78), in percent. Barrowman gives no
/// rule for when a surface is too rough to stay laminar, so every flight on hpr's drag warns.
pub const LAMINAR_FRICTION_PERCENT: f64 = 3.6;

/// The share of the static margin, calibres, that a kinked freeform fin's center of pressure
/// takes on OpenRocket's *Pods--airframes and winglets* example, where hpr's sits 1.6 mm aft of
/// OpenRocket 24.12's ([issue #326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)). Unprobed on
/// other outlines, so every freeform fin set warns.
pub const FREEFORM_FIN_MARGIN_CAL: f64 = 0.047;

/// How far the center of gravity may sit from the rocket's axis, m, before it counts as off it
/// ([issue #219](https://github.com/nrdptel/fusionspace-eridanus/issues/219)): 1e-9 m. A symmetric design's
/// center of gravity sits about 1e-18 m off the axis, rounding in the sum of its parts' places;
/// the 54 mm test rocket's rail buttons put it about 5.8e-5 m off at launch
/// (`a_rail_button_puts_the_test_rocket_off_its_axis_and_none_leaves_it_on`). The limit lies
/// between, far from both.
pub const OFF_AXIS_CG_LIMIT_M: f64 = 1e-9;

/// The fastest wind, m/s, whose reported direction counts as near calm for
/// [issue #8](https://github.com/nrdptel/fusionspace-eridanus/issues/8): 1.5 m/s, the top of Beaufort force
/// 1, light air, 0.3 to 1.5 m/s (force 0, calm, is 0 to 0.2 m/s), in MeteoSwiss's *Table of
/// medium wind speeds according to Beaufort Wind Scale* (Federal Office of Meteorology and
/// Climatology, November 2023), 1 to 3 knots in the UK Met Office's *Beaufort wind force scale*.
/// Its land criterion, in the Hong Kong Observatory's *Beaufort Wind Scale – Land and Sea
/// Criteria*: "Direction of wind shown by smoke drift, but not by wind vanes." A vane reads no
/// direction in such a wind, so a level this slow has no direction worth interpolating.
pub const NEAR_CALM_WIND_M_S: f64 = 1.5;

/// A known error in hpr's drag, its stability margin or its flight's path that a flight can meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum KnownIssue {
    /// [#67](https://github.com/nrdptel/fusionspace-eridanus/issues/67): the closed-form cone's pressure
    /// drag, which cones, ogives and some power and parabolic series take, reads high from
    /// Mach 0.8.
    TransonicNoseDrag,
    /// [#68](https://github.com/nrdptel/fusionspace-eridanus/issues/68): base drag reads high from Mach 0.8
    /// to 1.2 and low from 1.5.
    BaseDrag,
    /// [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70): a sharp fin section's drag reads
    /// high faster than sound. hpr has no sharp section, so the warning goes with the airfoil.
    SharpFinDrag,
    /// [#72](https://github.com/nrdptel/fusionspace-eridanus/issues/72): a steep boattail's drag reads high
    /// faster than sound.
    SteepBoattailDrag,
    /// [#222](https://github.com/nrdptel/fusionspace-eridanus/issues/222): supersonic pressure drag about twice
    /// OpenRocket's on an ogive nose and airfoil fins.
    SupersonicPressureDrag,
    /// [#87](https://github.com/nrdptel/fusionspace-eridanus/issues/87): a step in radius takes the whole
    /// body off the shock-expansion method faster than Mach 1.2.
    RadiusStepFallback,
    /// [#120](https://github.com/nrdptel/fusionspace-eridanus/issues/120): a lip longer than its boattail's
    /// drop in diameter does the same.
    LongLipFallback,
    /// [#121](https://github.com/nrdptel/fusionspace-eridanus/issues/121): a pointed tip steeper than the cone
    /// tables' 30° does the same.
    SteepTipFallback,
    /// [#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172): the static margin read up to
    /// 0.1108 calibres higher than OpenRocket's on four private designs, cause unknown.
    MarginReadsHigh,
    /// [#64](https://github.com/nrdptel/fusionspace-eridanus/issues/64): a fin whose leading edge sweeps
    /// forward takes a supersonic normal-force slope that rises past linear theory's start,
    /// moving the center of pressure aft.
    ForwardSweptFinSlope,
    /// [#73](https://github.com/nrdptel/fusionspace-eridanus/issues/73): below Mach 0.8, Niskanen's rule gives
    /// a boattail steeper than 9.46° a share of base drag that read up to +41.7% on a measured
    /// one.
    SubsonicBoattailDrag,
    /// [#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325): fin sets at one station are counted
    /// apart for fin–fin interference, where OpenRocket counts them together.
    FinSetsAtOneStation,
    /// [#18](https://github.com/nrdptel/fusionspace-eridanus/issues/18): skin friction is taken as fully
    /// turbulent, so on a surface smooth enough to keep its boundary layer laminar for a way it
    /// reads high.
    TurbulentFriction,
    /// [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326): a kinked freeform fin's center of
    /// pressure sat 1.6 mm aft of OpenRocket's; other freeform outlines are unprobed.
    FreeformFinCenterOfPressure,
    /// [#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179): a booster dropped at a separation
    /// flies on as a point with only its devices' drag, tumbling side-on from the split where a
    /// real one first coasts nose-first on its airframe's drag.
    BoosterAirframeDrag,
    /// [#354](https://github.com/nrdptel/fusionspace-eridanus/issues/354): a separated part has no drag at all
    /// until its first device opens, where a real one has its airframe's.
    DragFreeSeparatedPart,
    /// [#219](https://github.com/nrdptel/fusionspace-eridanus/issues/219): a center of gravity off the axis
    /// (an off-axis lug, rail button, pod or mass) makes the thrust turn the rocket in hpr's
    /// flight, and not in OpenRocket's; no reference sizes it.
    OffAxisCenterOfGravity,
    /// [#213](https://github.com/nrdptel/fusionspace-eridanus/issues/213): a single pod's drag and normal force
    /// act on the rocket's axis, so the trim and roll they make off it are left out.
    SinglePodMoment,
    /// [#106](https://github.com/nrdptel/fusionspace-eridanus/issues/106): inside the body's supersonic join a
    /// body part's pitch and yaw damping is sampled at a station away from its own center of
    /// pressure ([`hpr_aero::AeroModel::component_station_m`]).
    BodyDampingStation,
    /// [#8](https://github.com/nrdptel/fusionspace-eridanus/issues/8): a tabulated wind level near calm still
    /// swings the direction interpolated between it and its neighbour.
    NearCalmWindDirection,
}

/// Which of a flight's numbers a known issue bears on.
///
/// Not `#[non_exhaustive]`, unlike [`KnownIssue`]: a caller that names each kind, as the command
/// line's output and the Python binding do through [`IssueKind::name`], must meet a new one when
/// it compiles, rather than print it as another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    /// The drag: the apogee, the top speed, the drift, the flutter margin.
    Drag,
    /// The stability margin and the center of pressure.
    Stability,
    /// The flight's path, through something other than the drag or the margin (a moment, the
    /// damping, the wind): the apogee and the drift.
    Flight,
}

impl IssueKind {
    /// Every kind, in the order declared.
    pub const ALL: [Self; 3] = [Self::Drag, Self::Stability, Self::Flight];

    /// The kind's name, as it serializes: `"drag"`, `"stability"` or `"flight"`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Drag => "drag",
            Self::Stability => "stability",
            Self::Flight => "flight",
        }
    }
}

impl KnownIssue {
    /// The issue's number on GitHub.
    #[must_use]
    pub fn number(self) -> u32 {
        match self {
            Self::TransonicNoseDrag => 67,
            Self::BaseDrag => 68,
            Self::SharpFinDrag => 70,
            Self::SteepBoattailDrag => 72,
            Self::SupersonicPressureDrag => 222,
            Self::RadiusStepFallback => 87,
            Self::LongLipFallback => 120,
            Self::SteepTipFallback => 121,
            Self::MarginReadsHigh => 172,
            Self::ForwardSweptFinSlope => 64,
            Self::SubsonicBoattailDrag => 73,
            Self::FinSetsAtOneStation => 325,
            Self::TurbulentFriction => 18,
            Self::FreeformFinCenterOfPressure => 326,
            Self::BoosterAirframeDrag => 179,
            Self::DragFreeSeparatedPart => 354,
            Self::OffAxisCenterOfGravity => 219,
            Self::SinglePodMoment => 213,
            Self::BodyDampingStation => 106,
            Self::NearCalmWindDirection => 8,
        }
    }

    /// Which numbers the issue bears on: the margin and center of pressure for #64, #87, #120,
    /// #121, #172, #325 and #326; the flight's path for #8, #106, #213 and #219, whose signs
    /// depend on the case ([ADR-201][adr-201]); the drag for the rest.
    ///
    /// [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
    #[must_use]
    pub fn kind(self) -> IssueKind {
        match self {
            Self::RadiusStepFallback
            | Self::LongLipFallback
            | Self::SteepTipFallback
            | Self::MarginReadsHigh
            | Self::ForwardSweptFinSlope
            | Self::FinSetsAtOneStation
            | Self::FreeformFinCenterOfPressure => IssueKind::Stability,
            Self::OffAxisCenterOfGravity
            | Self::SinglePodMoment
            | Self::BodyDampingStation
            | Self::NearCalmWindDirection => IssueKind::Flight,
            Self::TransonicNoseDrag
            | Self::BaseDrag
            | Self::SharpFinDrag
            | Self::SteepBoattailDrag
            | Self::SupersonicPressureDrag
            | Self::SubsonicBoattailDrag
            | Self::TurbulentFriction
            | Self::BoosterAirframeDrag
            | Self::DragFreeSeparatedPart => IssueKind::Drag,
        }
    }

    /// Whether the issue is about the margin and center of pressure (#64, #87, #120, #121, #172,
    /// #325, #326) rather than drag or the flight's path ([`Self::kind`]).
    #[must_use]
    pub fn is_stability(self) -> bool {
        self.kind() == IssueKind::Stability
    }

    /// Whether the issue has a Mach condition, so its message quotes the flight's top speed:
    /// every one but #73 (any flight is subsonic for a time), #18, #172, #325 and #326, which
    /// hold at any speed, #179 and #354, which are a separated part's, and #8, #213 and #219,
    /// which are the wind's and the rocket's at any speed.
    #[must_use]
    pub fn has_mach_condition(self) -> bool {
        !matches!(
            self,
            Self::MarginReadsHigh
                | Self::SubsonicBoattailDrag
                | Self::FinSetsAtOneStation
                | Self::TurbulentFriction
                | Self::FreeformFinCenterOfPressure
                | Self::BoosterAirframeDrag
                | Self::DragFreeSeparatedPart
                | Self::OffAxisCenterOfGravity
                | Self::SinglePodMoment
                | Self::NearCalmWindDirection
        )
    }

    /// The issue's address.
    #[must_use]
    pub fn url(self) -> String {
        format!(
            "https://github.com/nrdptel/fusionspace-eridanus/issues/{}",
            self.number()
        )
    }
}

/// A known issue a flight meets, with the flight's top Mach number and the parts whose shape
/// meets its condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueWarning {
    /// Which issue.
    pub issue: KnownIssue,
    /// The flight's top Mach number.
    pub max_mach: Peak,
    /// The ids of the parts whose shape meets the issue's condition, in the layout's order;
    /// empty for base drag ([`KnownIssue::BaseDrag`]) and friction ([`KnownIssue::TurbulentFriction`]),
    /// which every rocket has, for the issues that name no part (#121, #172, #179, #354), and for
    /// the wind's (#8, [`KnownIssue::NearCalmWindDirection`]) and the center of gravity's (#219,
    /// [`KnownIssue::OffAxisCenterOfGravity`]), which are no part's.
    pub parts: Vec<String>,
    /// The flight's own figures the message quotes, for the issues that have them (#8, #106,
    /// #219); `None` for the rest, and in a record saved before they were kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<IssueDetail>,
}

/// The flight's own figures an issue's message quotes ([`IssueWarning::detail`]).
///
/// The figures are plain numbers, as a [`Peak`]'s are: one that isn't a number (a NaN distance or
/// Mach number, which warns rather than hide a warning) serializes as `null`, as `serde_json`
/// writes any NaN, and a record holding one doesn't read back.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssueDetail {
    /// #219: how far the center of gravity sits from the rocket's axis, m: the largest of its
    /// distances at the instants sampled on each vehicle the flight flew
    /// ([`center_of_gravity_issue_warnings`], [`crate::FlownVehicle::mass_samples`]).
    CenterOfGravityOffset {
        /// The distance, m.
        offset_m: f64,
    },
    /// #106: the supersonic join of each vehicle the flight flew that passed its join's start,
    /// in the flight's order ([`body_station_issue_warnings`]).
    SupersonicJoins {
        /// The joins, one per vehicle that meets the issue.
        joins: Vec<SupersonicJoin>,
    },
    /// #8: the spans of the wind's table that swing the direction inside the flight's heights,
    /// lowest first.
    DirectionTurns {
        /// The spans, each with its two levels.
        turns: Vec<DirectionTurn>,
    },
}

/// Where one vehicle's supersonic join starts, and where its stations rejoin their centers of
/// pressure ([`IssueDetail::SupersonicJoins`]).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SupersonicJoin {
    /// Which vehicle: 0 for the launch stack, `k` for the sustainer the `k`-th powered
    /// separation handed the flight to ([`crate::Simulation::flown_vehicles`]).
    pub vehicle: usize,
    /// The join's start, Mach ([`hpr_aero::SupersonicBody::join_start_mach`]).
    pub join_start_mach: f64,
    /// The join's end, [`hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH`] past its start, where every
    /// station is its part's center of pressure again; `None` where some station never is, at
    /// any speed past the start.
    pub join_end_mach: Option<f64>,
}

impl SupersonicJoin {
    /// The join's span as a message quotes it: `from Mach 1.20 to 1.50`, or `at every speed past
    /// Mach 1.20` for a join that never ends.
    fn span(&self) -> String {
        match self.join_end_mach {
            Some(end) => format!("from Mach {:.2} to {end:.2}", self.join_start_mach),
            None => format!("at every speed past Mach {:.2}", self.join_start_mach),
        }
    }

    /// The vehicle as a message names it.
    fn vehicle_name(&self) -> String {
        match self.vehicle {
            0 => "the launch stack".to_owned(),
            1 => "the sustainer after the first separation".to_owned(),
            k => format!("the sustainer after separation {k}"),
        }
    }
}

/// A distance in millimeters, to three decimals; a smaller one as under a thousandth of one, and
/// one that isn't a number as unknown.
fn millimeters(distance_m: f64) -> String {
    let mm = distance_m * 1000.0;
    if !mm.is_finite() {
        "an unknown distance".to_owned()
    } else if mm < 0.001 {
        "under 0.001 mm".to_owned()
    } else {
        format!("{mm:.3} mm")
    }
}

impl IssueWarning {
    /// The issue's number on GitHub.
    #[must_use]
    pub fn number(&self) -> u32 {
        self.issue.number()
    }

    /// What the issue means for this flight, in a sentence or two, with which way its numbers
    /// lean.
    #[must_use]
    pub fn message(&self) -> String {
        let mach = self.max_mach.value;
        let low = "so the apogee, the top speed and the drift read low, and the flutter margin \
                   and the largest dynamic pressure look better than they are";
        let what = match self.issue {
            KnownIssue::TransonicNoseDrag => format!(
                "a cone-like nose's or shoulder's pressure drag reads high from Mach \
                 {TRANSONIC_NOSE_DRAG_MACH} (about twice a measured cone's at Mach 0.85, still \
                 +15% at 1.5), {low}"
            ),
            KnownIssue::BaseDrag => {
                let mut what = format!(
                    "base drag reads high from Mach {BASE_DRAG_HIGH_MACH} to \
                     {BASE_DRAG_HIGH_TOP_MACH} (0.225 against a measured 0.156 at Mach 0.9), \
                     {low}"
                );
                if past(mach, BASE_DRAG_HIGH_TOP_MACH) {
                    what.push_str(&format!(
                        "; from Mach {BASE_DRAG_LOW_MACH} it reads low (17% at Mach 2), so the \
                         apogee and the top speed there read high"
                    ));
                }
                what
            }
            KnownIssue::SharpFinDrag => format!(
                "hpr has no sharp-edged (double-wedge) fin section, and drawn as an airfoil such \
                 a fin's drag reads high faster than sound: if these fins are sharp, {low}"
            ),
            KnownIssue::SteepBoattailDrag => format!(
                "a boattail steeper than {:.0}° drags high faster than sound (+26% to +54% at \
                 16°), {low}",
                STEEP_BOATTAIL_RAD.to_degrees()
            ),
            KnownIssue::SupersonicPressureDrag => format!(
                "supersonic pressure drag on an ogive nose or airfoil fins is about twice \
                 OpenRocket's, and which is right is unresolved: if hpr's is high, {low}"
            ),
            KnownIssue::RadiusStepFallback
            | KnownIssue::LongLipFallback
            | KnownIssue::SteepTipFallback => {
                let switch = match self.issue {
                    KnownIssue::RadiusStepFallback => "a step in radius",
                    KnownIssue::LongLipFallback => {
                        "a lip longer than its boattail's drop in diameter"
                    }
                    _ => "a pointed tip steeper than the cone tables' 30°",
                };
                format!(
                    "{switch} takes the whole body off the shock-expansion method from Mach \
                     {SUPERSONIC_SWITCH_MACH}, so slender-body theory puts the center of pressure \
                     aft there and the margin reads high"
                )
            }
            KnownIssue::MarginReadsHigh => format!(
                "the static margin read up to {MARGIN_HIGH_BOUND_CAL} calibres higher than \
                 OpenRocket's on four private designs, for a reason not yet found, so this \
                 flight's margin may read high by as much"
            ),
            KnownIssue::ForwardSweptFinSlope => format!(
                "a fin whose leading edge sweeps forward takes a normal-force slope that rises by \
                 up to 7.7% past Mach {}, where it should fall, so faster than Mach \
                 {FORWARD_SWEEP_MACH} the center of pressure may sit aft and the margin read high",
                hpr_aero::fins::SUPERSONIC_START_MACH
            ),
            KnownIssue::SubsonicBoattailDrag => format!(
                "below Mach {} a boattail steeper than {:.1}° takes a share of base drag by a rule \
                 that read from −40.8% to +41.7% on measured 16° boattails; where its drag reads \
                 high, {low}",
                hpr_aero::drag::SUBSONIC_MACH_LIMIT,
                SUBSONIC_BOATTAIL_RULE_RAD.to_degrees()
            ),
            KnownIssue::FinSetsAtOneStation => format!(
                "fin sets that share a station, more than {FINS_WITHOUT_INTERFERENCE} fins between \
                 them, are each counted alone for fin–fin interference, where OpenRocket counts \
                 them together (about {FIN_SETS_AT_ONE_STATION_CAL} calibres of margin on its pods \
                 example), so the margin may read high"
            ),
            KnownIssue::TurbulentFriction => format!(
                "skin friction is taken as fully turbulent, but on a smooth surface the flow stays \
                 laminar near the nose, where friction is lower: hpr's reads high by \
                 {LAMINAR_FRICTION_PERCENT}% on RocketPy's Calisto at Mach 0.3; if this surface \
                 is that smooth, the drag reads high, {low}"
            ),
            KnownIssue::FreeformFinCenterOfPressure => format!(
                "a kinked freeform fin's center of pressure sat 1.6 mm aft of OpenRocket's \
                 (about {FREEFORM_FIN_MARGIN_CAL} calibres of margin on its pods example) and \
                 other freeform outlines are unprobed, so the margin may read high"
            ),
            KnownIssue::BoosterAirframeDrag => "a booster dropped at a separation flies on as a \
                 point with only its devices' drag, where a real one first coasts nose-first on \
                 its airframe's drag: tumbling side-on from the split, as hpr tumbles a .ork \
                 file's booster, the booster's peak and drift likely read short (with nothing \
                 open at the split, see #354); its flight is not validated"
                .to_owned(),
            KnownIssue::DragFreeSeparatedPart => "a separated part flies on as a point with only \
                 its open devices' drag, so it has no drag at all until its first device opens, \
                 where a real one has its airframe's: falling, it likely lands early and the wind \
                 carries it less far than it would, so its drift likely reads short; climbing, its \
                 peak reads high"
                .to_owned(),
            KnownIssue::OffAxisCenterOfGravity => {
                let offset = match &self.detail {
                    Some(IssueDetail::CenterOfGravityOffset { offset_m }) => millimeters(*offset_m),
                    _ => "a distance".to_owned(),
                };
                format!(
                    "the center of gravity sits {offset} off the rocket's axis, as an off-axis lug, \
                     rail button, pod or mass puts it: hpr flies the moment the thrust makes about \
                     it, OpenRocket's flights don't, and no reference measures which is right. The \
                     moment turns with the offset's side, so against a wind or a tilted rail the \
                     apogee and the drift may read low or high, and nothing measures by how much"
                )
            }
            KnownIssue::SinglePodMoment => {
                "a single pod off the axis has its drag and normal force applied on the rocket's \
                 axis, so the trim its drag makes and the roll its normal force makes are left \
                 out. Which way the flight moves depends on where the pod faces against the wind \
                 and the rail's tilt: the drift may read short and the apogee low, or the reverse, \
                 and nothing measures by how much"
                    .to_owned()
            }
            KnownIssue::BodyDampingStation => {
                let joins = match &self.detail {
                    Some(IssueDetail::SupersonicJoins { joins }) => joins.as_slice(),
                    _ => &[],
                };
                // The launch stack alone, as a single-stage flight's is, needs no name; any
                // sustainer's join is named with its vehicle, each vehicle's apart.
                let span = match joins {
                    [] => "inside the join".to_owned(),
                    [join] if join.vehicle == 0 => join.span(),
                    _ => {
                        let named: Vec<String> = joins
                            .iter()
                            .map(|join| format!("{} on {}", join.span(), join.vehicle_name()))
                            .collect();
                        match named.split_last() {
                            Some((last, [])) => last.clone(),
                            Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
                            None => String::new(),
                        }
                    }
                };
                format!(
                    "where the shock-expansion method joins slender-body theory, {span}, a body \
                     part's pitch and yaw damping is sampled at a station away from its own center \
                     of pressure (4.7 calibres apart at Mach 1.3 on a 54 mm test rocket). The static and \
                     flight margins don't use the station, but the apogee and the drift may lean \
                     either way, and nothing bounds by how much"
                )
            }
            KnownIssue::NearCalmWindDirection => {
                let turns = match &self.detail {
                    Some(IssueDetail::DirectionTurns { turns }) => turns.as_slice(),
                    _ => &[],
                };
                let span = turns.first().map_or_else(
                    || "a span of the wind's table".to_owned(),
                    |turn| {
                        let (lower, upper) = (turn.lower, turn.upper);
                        format!(
                            "{:.0} to {:.0} m above sea level, from {:.1} m/s at {:.0}° to {:.1} m/s \
                             at {:.0}°",
                            lower.height_msl_m,
                            upper.height_msl_m,
                            lower.speed_m_s,
                            lower.direction_from_rad.to_degrees(),
                            upper.speed_m_s,
                            upper.direction_from_rad.to_degrees()
                        )
                    },
                );
                let more = match turns.len() {
                    0 | 1 => String::new(),
                    2 => " (and one more span)".to_owned(),
                    n => format!(" (and {} more spans)", n - 1),
                };
                format!(
                    "the wind's direction is interpolated across {span}{more}, where a level is \
                     near calm (at most {NEAR_CALM_WIND_M_S} m/s, light air, whose direction a vane \
                     doesn't show): its reported direction means little, yet it swings the wind \
                     across the path. Where the other levels share one direction, the swing turns part \
                     of the wind off it, so the drift likely reads short; where they turn, it may \
                     read long. So the drift and the apogee may read short or long, and nothing \
                     measures by how much"
                )
            }
        };
        // An issue with no Mach condition doesn't quote the flight's top speed.
        if !self.issue.has_mach_condition() {
            return format!("issue #{}: {what} ({})", self.number(), self.issue.url());
        }
        format!(
            "issue #{}: {what}; this flight reaches Mach {mach:.2} at {:.1} s ({})",
            self.number(),
            self.max_mach.time_s,
            self.issue.url()
        )
    }
}

/// Whether `value` is past `edge`, a NaN counting as past it.
fn past(value: f64, edge: f64) -> bool {
    value.is_nan() || value > edge
}

/// The half-angle of a transition that narrows aft, rad: `atan((r_fore − r_aft)/l)`, its mean
/// angle (a conical boattail's own). `None` for one that doesn't narrow, and for one of no
/// length, which the drag buildup takes as a step, not a boattail.
fn boattail_angle_rad(fore_radius_m: f64, aft_radius_m: f64, length_m: f64) -> Option<f64> {
    let drop_m = fore_radius_m - aft_radius_m;
    // Written out so a NaN radius or length counts as narrowing: a broken number can't hide one.
    if drop_m <= 0.0 || length_m == 0.0 {
        return None;
    }
    Some(drop_m.atan2(length_m))
}

/// The issues a flight meets: one whose top Mach number is `max_mach`, of a rocket whose parts
/// are `parts`, each an id and its part, flown on hpr's own drag. In [`KnownIssue`]'s order, each
/// listing the parts that meet its condition. A flight with no top Mach number meets none.
#[must_use]
pub fn issue_warnings<'a>(
    parts: impl IntoIterator<Item = (&'a str, &'a Part)>,
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let mut cone_like = Vec::new();
    let mut ogive_or_airfoil = Vec::new();
    let mut airfoil = Vec::new();
    let mut steep = Vec::new();
    let mut ruled = Vec::new();
    for (id, part) in parts {
        match part {
            Part::NoseCone(cone) => {
                if takes_cone_formula(cone.shape) {
                    cone_like.push(id.to_owned());
                }
                if matches!(cone.shape, NoseShape::Ogive { .. }) {
                    ogive_or_airfoil.push(id.to_owned());
                }
            }
            Part::FinSet(fins) if fins.cross_section == FinCrossSection::Airfoil => {
                airfoil.push(id.to_owned());
                ogive_or_airfoil.push(id.to_owned());
            }
            Part::Transition(transition) => {
                let (fore, aft) = (transition.fore_radius_m, transition.aft_radius_m);
                // A shoulder (widening aft) drags as a nose does; a NaN radius counts.
                let widening = aft.is_nan() || fore.is_nan() || aft > fore;
                if widening && takes_cone_formula(transition.shape) {
                    cone_like.push(id.to_owned());
                }
                let angle = boattail_angle_rad(fore, aft, transition.length_m);
                if angle.is_some_and(|angle| past(angle, STEEP_BOATTAIL_RAD)) {
                    steep.push(id.to_owned());
                }
                if angle.is_some_and(|angle| past(angle, SUBSONIC_BOATTAIL_RULE_RAD)) {
                    ruled.push(id.to_owned());
                }
            }
            _ => {}
        }
    }
    let mach = max_mach.value;
    let supersonic = past(mach, SUPERSONIC_DRAG_MACH);
    let mut warnings = Vec::new();
    let mut warn = |issue, parts: Vec<String>| {
        warnings.push(IssueWarning {
            issue,
            max_mach,
            parts,
            detail: None,
        });
    };
    if past(mach, TRANSONIC_NOSE_DRAG_MACH) && !cone_like.is_empty() {
        warn(KnownIssue::TransonicNoseDrag, cone_like);
    }
    if past(mach, BASE_DRAG_HIGH_MACH) {
        warn(KnownIssue::BaseDrag, Vec::new());
    }
    if supersonic && !airfoil.is_empty() {
        warn(KnownIssue::SharpFinDrag, airfoil);
    }
    if supersonic && !steep.is_empty() {
        warn(KnownIssue::SteepBoattailDrag, steep);
    }
    if supersonic && !ogive_or_airfoil.is_empty() {
        warn(KnownIssue::SupersonicPressureDrag, ogive_or_airfoil);
    }
    // Every flight is subsonic for a time, so the rule's boattails warn at any speed.
    if !ruled.is_empty() {
        warn(KnownIssue::SubsonicBoattailDrag, ruled);
    }
    warnings
}

/// The friction issue a flight on hpr's own drag meets: [#18][i18] whenever it has a top Mach
/// number, at any speed and on any finish, since Barrowman 1967 gives no rule for when a surface
/// is too rough to stay laminar ([`LAMINAR_FRICTION_PERCENT`]). Apart from [`issue_warnings`],
/// which reads the design's shape.
///
/// [i18]: https://github.com/nrdptel/fusionspace-eridanus/issues/18
#[must_use]
pub fn friction_issue_warnings(max_mach: Option<Peak>) -> Vec<IssueWarning> {
    max_mach
        .map(|max_mach| IssueWarning {
            issue: KnownIssue::TurbulentFriction,
            max_mach,
            parts: Vec::new(),
            detail: None,
        })
        .into_iter()
        .collect()
}

/// The separated parts' issues a flight meets ([ADR-200][adr-200]), read from the bodies that
/// flew on their own ([`FlightResult::bodies`]), whatever the rocket's drag: each part's descent
/// is a point under its open devices ([`crate::recovery`]), never the rocket's drag.
///
/// - [#179][i179] when a body aft of a stage boundary flew on its own: a booster dropped at a
///   separation, or a parallel stage. Its first stage is past the nose's (stage 0), whatever
///   devices it carries; the error is unsized, so no speed at the split is excluded.
/// - [#354][i354] when a body started its own flight with no drag area open (zero, or not a
///   number, so a broken one can't hide it), and no device opened at that same instant (an event
///   at its start time, as a device fired by the split with no lag is): it flies with no drag
///   until its first device opens. A canopy that fills from nothing (a fill time) opened on the
///   stack at the split also starts at zero, so it warns too: an over-warning, on the safe side.
///   The flight refuses such a part while it climbs through a fired device's lag, and refuses one
///   that lands with nothing open ([`crate::recovery`]), so what is left is mostly a part already
///   falling.
///
/// Each warns once, whatever the number of bodies that meet it, and only for a flight with a top
/// Mach number (`max_mach`), as the others.
///
/// [adr-200]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0200-the-separated-parts-warnings.md
/// [i179]: https://github.com/nrdptel/fusionspace-eridanus/issues/179
/// [i354]: https://github.com/nrdptel/fusionspace-eridanus/issues/354
#[must_use]
pub fn separated_part_issue_warnings(
    result: &FlightResult,
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let booster = result.bodies.iter().any(|body| body.stages.0 > 0);
    let drag_free = result.bodies.iter().any(|body| {
        // A device that opens at the start's own instant is an event at its time, after the
        // start sample. A NaN area or time is never open.
        let start = &body.start_sample;
        let open = |area_m2: f64| area_m2 > 0.0;
        !open(start.recovery_drag_area_m2)
            && !body
                .events
                .iter()
                .take_while(|event| event.sample.time_s <= start.time_s)
                .any(|event| open(event.sample.recovery_drag_area_m2))
    });
    [
        (booster, KnownIssue::BoosterAirframeDrag),
        (drag_free, KnownIssue::DragFreeSeparatedPart),
    ]
    .into_iter()
    .filter(|(meets, _)| *meets)
    .map(|(_, issue)| IssueWarning {
        issue,
        max_mach,
        parts: Vec::new(),
        detail: None,
    })
    .collect()
}

/// The larger of two numbers, NaN if either is, so a broken one can't hide a warning.
fn largest(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

/// The off-axis center of gravity's issue a flight meets ([#219][i219], [ADR-201][adr-201]):
/// when the center of gravity sits more than [`OFF_AXIS_CG_LIMIT_M`] from the rocket's axis in
/// any of `masses`, a NaN distance counting as more. A flight passes each vehicle it flew's
/// samples ([`crate::FlownVehicle::mass_samples`]): the stack at launch, at each motor's ignition
/// and burnout and fully burnt, and each sustainer from its separation to its end. Its distance is
/// `√(x² + y²)` of [`MassProperties::cg_m`], the largest over them. hpr's equations of motion are
/// written about the nose tip with the center of gravity where it is, so the thrust along the
/// axis turns the rocket about an offset one; no reference sizes how much that does, so the
/// warning holds whatever the drag and at any speed. None for a flight with no top Mach number.
///
/// [i219]: https://github.com/nrdptel/fusionspace-eridanus/issues/219
/// [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
#[must_use]
pub fn center_of_gravity_issue_warnings(
    masses: &[MassProperties],
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let offset_m = masses
        .iter()
        .map(|mass| mass.cg_m.x.hypot(mass.cg_m.y))
        .fold(f64::NEG_INFINITY, largest);
    // Written out so a NaN distance warns; no samples at all is none off the axis.
    if offset_m <= OFF_AXIS_CG_LIMIT_M {
        return Vec::new();
    }
    vec![IssueWarning {
        issue: KnownIssue::OffAxisCenterOfGravity,
        max_mach,
        parts: Vec::new(),
        detail: Some(IssueDetail::CenterOfGravityOffset { offset_m }),
    }]
}

/// The single pod's issue a flight meets ([#213][i213], [ADR-201][adr-201]): a pod set of one
/// pod ([`hpr_design::PodSet::count`]) off the rocket's axis (a radial offset not 0, a NaN one
/// counting as off) that holds at least one part, listing those pod sets' ids in the layout's
/// order. A pod set holding nothing adds no force ([`hpr_aero::AeroModel::new`]); two pods or more
/// stand evenly round the axis, so their offsets add to zero and so do the moments hpr leaves out
/// ([`hpr_aero::PodSetAero`]). A parallel stage is laid out as a pod set
/// ([`hpr_design::ParallelStage`]), so a single strap-on booster counts. A single pod inside a
/// set of several counts too, though the copies' moments may cancel: an over-warning, on the safe
/// side. At any speed and whatever the drag; none for a flight with no top Mach number.
///
/// [i213]: https://github.com/nrdptel/fusionspace-eridanus/issues/213
/// [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
#[must_use]
pub fn pod_issue_warnings(layout: &Layout, max_mach: Option<Peak>) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let parts: Vec<String> = layout
        .components
        .iter()
        .enumerate()
        .filter(|(index, component)| {
            matches!(&component.part, Part::PodSet(set)
                if set.count == 1 && !(set.radial_offset_m == 0.0))
                && layout
                    .components
                    .iter()
                    .any(|child| child.parent == Some(*index))
        })
        .map(|(_, component)| component.id.clone())
        .collect();
    if parts.is_empty() {
        return Vec::new();
    }
    vec![IssueWarning {
        issue: KnownIssue::SinglePodMoment,
        max_mach,
        parts,
        detail: None,
    }]
}

/// The body's damping-station issue a flight meets ([#106][i106], [ADR-201][adr-201]): when the
/// shock-expansion method flies a vehicle's body ([`AeroModel::supersonic_body`]) and that
/// vehicle's own top Mach number is past its join's start (exclusive; NaN counts as past). Each of
/// `vehicles` is a vehicle the flight flew as a rigid body, in order, with its aerodynamics and its
/// top Mach number while it was the one flying: the launch stack first, then each sustainer a
/// powered separation handed the flight to ([`crate::Simulation::flown_vehicles`],
/// [`crate::FlightMetrics::max_mach_by_vehicle`]). A sustainer is a shorter body than the stack,
/// so its join can start lower, and a stack the method doesn't fly can leave a sustainer it does.
/// The warning lists the body parts the method covers on any vehicle that meets it, each once, in
/// the layout's order, and keeps each such vehicle's join ([`IssueDetail::SupersonicJoins`]).
/// Inside the join a nose's or cylinder's station is joined linearly between the two models'
/// while its force blends their slopes and moments, so the two agree only at the join's ends
/// ([`AeroModel::component_station_m`]); the station is only where the flight samples the part's
/// local flow for pitch and yaw damping, and the margins never read it.
///
/// The stations rejoin their centers of pressure at the join's end,
/// [`hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH`] past its start, unless the body's shape takes less
/// than all of the method ([`hpr_aero::SupersonicBody::shape_weight`] below 1, or NaN), so the
/// join never ends, or a covered part keeps slender-body theory's station while its force takes
/// the method's (a boattail, a cylinder behind it: [`hpr_aero::SupersonicBody::has_station`]):
/// then the gap holds at every speed past the start, and the message says so.
///
/// A vehicle no faster than [`SUPERSONIC_SWITCH_MACH`], the earliest the join can start, meets
/// none without building the method's table; nor does one with no top Mach number (it took no
/// step), nor a flight with none (`max_mach`, the flight's, which the warning quotes). A NaN top
/// Mach number for the flight counts as past every vehicle's join. The stations are hpr's even
/// under a normal-force table of the flight's own, which keeps hpr's damping, so the warning
/// holds whatever the drag and the normal force.
///
/// [i106]: https://github.com/nrdptel/fusionspace-eridanus/issues/106
/// [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
#[must_use]
pub fn body_station_issue_warnings(
    vehicles: &[(&AeroModel, Option<Peak>)],
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let mut parts: Vec<String> = Vec::new();
    let mut joins: Vec<SupersonicJoin> = Vec::new();
    for (vehicle, &(aero, top)) in vehicles.iter().enumerate() {
        let Some(top) = top else {
            continue;
        };
        // A NaN top for the flight counts as past every vehicle's join, so a broken number in
        // one vehicle's steps can't hide another's warning.
        let top_mach = if max_mach.value.is_nan() {
            f64::NAN
        } else {
            top.value
        };
        if !past(top_mach, SUPERSONIC_SWITCH_MACH) {
            continue;
        }
        let Some(body) = aero.supersonic_body() else {
            continue;
        };
        if !past(top_mach, body.join_start_mach) {
            continue;
        }
        let covered = body.covered.min(aero.bodies().len());
        // A NaN shape weight counts as less than all of the method, so the join never ends.
        let endless = body.shape_weight.is_nan()
            || body.shape_weight < 1.0
            || (0..covered).any(|index| !body.has_station(index));
        joins.push(SupersonicJoin {
            vehicle,
            join_start_mach: body.join_start_mach,
            join_end_mach: (!endless)
                .then_some(body.join_start_mach + hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH),
        });
        // Every vehicle's body runs aft from the one nose, so its parts are the stack's first
        // ones: a part not yet listed comes after those that are.
        for part in &aero.bodies()[..covered] {
            if !parts.contains(&part.id) {
                parts.push(part.id.clone());
            }
        }
    }
    if joins.is_empty() {
        return Vec::new();
    }
    vec![IssueWarning {
        issue: KnownIssue::BodyDampingStation,
        max_mach,
        parts,
        detail: Some(IssueDetail::SupersonicJoins { joins }),
    }]
}

/// The highest a flight reached, m above the launch site: the largest height of its events, its
/// final sample, and each separated body's and released part's start, events and final sample
/// ([`crate::BodyFlight`], [`crate::ReleasedFlight`]). An apogee is an event, so it is among them.
/// NaN if any of those is, so a broken height can't hide a warning.
#[must_use]
pub fn highest_height_m(result: &FlightResult) -> f64 {
    let flight = result
        .events
        .iter()
        .map(|event| event.sample.height_above_ground_m)
        .chain(std::iter::once(result.final_sample.height_above_ground_m));
    let bodies = result.bodies.iter().flat_map(|body| {
        std::iter::once(&body.start_sample)
            .chain(body.events.iter().map(|event| &event.sample))
            .chain(std::iter::once(&body.final_sample))
            .map(|sample| sample.height_above_ground_m)
    });
    let released = result.released.iter().flat_map(|part| {
        std::iter::once(&part.start_sample)
            .chain(part.events.iter().map(|event| &event.sample))
            .chain(std::iter::once(&part.final_sample))
            .map(|sample| sample.height_above_ground_m)
    });
    flight
        .chain(bodies)
        .chain(released)
        .fold(f64::NEG_INFINITY, largest)
}

/// The wind's issue a flight meets ([#8][i8], [ADR-201][adr-201]): the spans of a tabulated wind
/// over which its direction is interpolated between two levels ([`hpr_atmos::Wind::direction_turns`])
/// where the direction still swings although a level is near calm, inside the flight's heights.
/// A span counts when neither level's speed is exactly 0 (a calm level takes its neighbour's
/// direction, so nothing turns), at least one is at or below [`NEAR_CALM_WIND_M_S`], their
/// directions differ, and the span reaches inside `heights_msl_m`, the flight's lowest and highest
/// heights above mean sea level, as a wind is read: the ground and the highest point any body
/// reached ([`highest_height_m`]). A span that only touches either end doesn't count: the wind
/// there is the end level's own. Any NaN counts the way that warns. The spans that count are kept
/// in the warning, lowest first.
///
/// The swing's sign depends on the levels and on which way the flight drifts through them: it
/// turns a wind of the same speed, so where the other levels share one direction it can only
/// shorten the drift's sum, and where they turn it can lengthen it. A wind of the flight's own that turns with no table reports no spans, so it meets
/// none; so does a flight with no top Mach number.
///
/// [i8]: https://github.com/nrdptel/fusionspace-eridanus/issues/8
/// [adr-201]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0201-the-unknown-signs.md
#[must_use]
pub fn wind_issue_warnings(
    turns: &[DirectionTurn],
    heights_msl_m: [f64; 2],
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let [ground_msl_m, top_msl_m] = heights_msl_m;
    let calm = |speed_m_s: f64| speed_m_s.is_nan() || speed_m_s <= NEAR_CALM_WIND_M_S;
    let turns: Vec<DirectionTurn> = turns
        .iter()
        .filter(|turn| {
            let (lower, upper) = (turn.lower, turn.upper);
            !(lower.speed_m_s == 0.0)
                && !(upper.speed_m_s == 0.0)
                && (calm(lower.speed_m_s) || calm(upper.speed_m_s))
                && !(lower.direction_from_rad == upper.direction_from_rad)
                && !(lower.height_msl_m >= top_msl_m || upper.height_msl_m <= ground_msl_m)
        })
        .copied()
        .collect();
    if turns.is_empty() {
        return Vec::new();
    }
    vec![IssueWarning {
        issue: KnownIssue::NearCalmWindDirection,
        max_mach,
        parts: Vec::new(),
        detail: Some(IssueDetail::DirectionTurns { turns }),
    }]
}

/// Whether a fin's leading edge sweeps forward anywhere: a trapezoid's tip ahead of its root's
/// leading edge; a freeform outline with a point ahead of it (the root's leading edge is the
/// outline's origin), or with an edge that runs outward and forward, as a kinked leading edge
/// does past its kink. An ellipse's edge never does. A NaN sweep or point counts, so a broken
/// number can't hide one.
fn sweeps_forward(planform: &FinPlanform) -> bool {
    match planform {
        FinPlanform::Trapezoidal { sweep_m, .. } => sweep_m.is_nan() || *sweep_m < 0.0,
        FinPlanform::Elliptical { .. } => false,
        FinPlanform::Freeform { points_m, .. } => {
            points_m.iter().any(|[x, _]| x.is_nan() || *x < 0.0)
                || points_m.windows(2).any(|pair| {
                    let ([x0, h0], [x1, h1]) = (pair[0], pair[1]);
                    h1 > h0 && x1 < x0
                })
        }
        // A planform added later warns until it is read: the cautious side.
        _ => true,
    }
}

/// The fin sets of `layout` that share a station with others, more than
/// [`FINS_WITHOUT_INTERFERENCE`] fins between them, in the layout's order
/// ([issue #325](https://github.com/nrdptel/fusionspace-eridanus/issues/325)). Two sets share a station when
/// their roots overlap or touch along the axis on the same stage and the same airframe or pod
/// set. OpenRocket's rule is unsized between overlapping roots and a common aft station; the
/// overlap is the wider of the two, so it can't miss a case the other would warn.
fn fin_sets_at_one_station(layout: &Layout) -> Vec<String> {
    let fin_sets: Vec<(usize, &PlacedComponent, u32)> = layout
        .components
        .iter()
        .enumerate()
        .filter_map(|(index, component)| match &component.part {
            Part::FinSet(fins) => Some((index, component, fins.count)),
            _ => None,
        })
        .collect();
    let shares = |(i, a, _): &(usize, &PlacedComponent, u32),
                  (j, b, _): &(usize, &PlacedComponent, u32)| {
        let (a_fore, a_aft) = (a.fore_station_m, a.fore_station_m + a.length_m);
        let (b_fore, b_aft) = (b.fore_station_m, b.fore_station_m + b.length_m);
        a.stage == b.stage
            && layout.pod_set_of(*i) == layout.pod_set_of(*j)
            // Written out so a NaN station counts as shared.
            && !(a_aft < b_fore || b_aft < a_fore)
    };
    fin_sets
        .iter()
        .filter(|set| {
            let mut sharing = false;
            // Summed wide, so a layout built by hand with huge counts can't overflow.
            let fins: u64 = fin_sets
                .iter()
                .filter(|other| shares(set, other))
                .map(|other| {
                    sharing |= other.0 != set.0;
                    u64::from(other.2)
                })
                .sum();
            sharing && fins > u64::from(FINS_WITHOUT_INTERFERENCE)
        })
        .map(|(_, component, _)| component.id.clone())
        .collect()
}

/// [`issue_warnings`] over every part of `layout`, pods' included.
#[must_use]
pub fn layout_issue_warnings(layout: &Layout, max_mach: Option<Peak>) -> Vec<IssueWarning> {
    issue_warnings(
        layout
            .components
            .iter()
            .map(|component| (component.id.as_str(), &component.part)),
        max_mach,
    )
}

/// The stability issues a design's shape meets, on a flight whose top Mach number is
/// `max_mach` ([ADR-189][adr-189]): [#64](https://github.com/nrdptel/fusionspace-eridanus/issues/64) for its
/// forward-swept fin sets past [`FORWARD_SWEEP_MACH`], then
/// [#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325) for its fin sets that share a station,
/// then [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326) for its freeform fin sets at any
/// speed ([`FREEFORM_FIN_MARGIN_CAL`]). They are errors in hpr's normal force, not its drag, so
/// a flight on a drag of its own meets them and one on a normal-force table of its own doesn't,
/// as with [`stability_issue_warnings`]. A flight with no top Mach number meets none.
///
/// [adr-189]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0189-the-safety-and-shape-warnings.md
#[must_use]
pub fn layout_stability_issue_warnings(
    layout: &Layout,
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let mut warnings = Vec::new();
    let forward_swept: Vec<String> = layout
        .components
        .iter()
        .filter(|component| {
            matches!(&component.part, Part::FinSet(fins) if sweeps_forward(&fins.planform))
        })
        .map(|component| component.id.clone())
        .collect();
    if past(max_mach.value, FORWARD_SWEEP_MACH) && !forward_swept.is_empty() {
        warnings.push(IssueWarning {
            issue: KnownIssue::ForwardSweptFinSlope,
            max_mach,
            parts: forward_swept,
            detail: None,
        });
    }
    let parts = fin_sets_at_one_station(layout);
    if !parts.is_empty() {
        warnings.push(IssueWarning {
            issue: KnownIssue::FinSetsAtOneStation,
            max_mach,
            parts,
            detail: None,
        });
    }
    let freeform: Vec<String> = layout
        .components
        .iter()
        .filter(|component| {
            matches!(&component.part, Part::FinSet(fins)
                if matches!(fins.planform, FinPlanform::Freeform { .. }))
        })
        .map(|component| component.id.clone())
        .collect();
    if !freeform.is_empty() {
        warnings.push(IssueWarning {
            issue: KnownIssue::FreeformFinCenterOfPressure,
            max_mach,
            parts: freeform,
            detail: None,
        });
    }
    warnings
}

/// The stability issues a flight meets ([ADR-181][adr-181]): one whose top Mach number is
/// `max_mach`, whose aerodynamics stopped the supersonic body run for `fallback`
/// ([`hpr_aero::AeroModel::supersonic_fallback`]), and whose smallest static margin is
/// `min_static_margin`. Issues #87, #120 and #121 when their switch stopped the run and the flight
/// is past [`SUPERSONIC_SWITCH_MACH`], naming the component; #172 whenever the flight has a
/// smallest static margin. A flight with no top Mach number meets none.
///
/// [adr-181]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0181-the-stability-issue-warnings.md
#[must_use]
pub fn stability_issue_warnings(
    fallback: Option<&hpr_aero::SupersonicFallback>,
    min_static_margin: Option<Peak>,
    max_mach: Option<Peak>,
) -> Vec<IssueWarning> {
    use hpr_aero::SupersonicFallback as Fallback;
    let Some(max_mach) = max_mach else {
        return Vec::new();
    };
    let mut warnings = Vec::new();
    if past(max_mach.value, SUPERSONIC_SWITCH_MACH) {
        let switch = match fallback {
            Some(Fallback::RadiusStep { component }) => {
                Some((KnownIssue::RadiusStepFallback, vec![component.clone()]))
            }
            Some(Fallback::LongLip { component }) => {
                Some((KnownIssue::LongLipFallback, vec![component.clone()]))
            }
            Some(Fallback::SteepTip) => Some((KnownIssue::SteepTipFallback, Vec::new())),
            _ => None,
        };
        if let Some((issue, parts)) = switch {
            warnings.push(IssueWarning {
                issue,
                max_mach,
                parts,
                detail: None,
            });
        }
    }
    if min_static_margin.is_some() {
        warnings.push(IssueWarning {
            issue: KnownIssue::MarginReadsHigh,
            max_mach,
            parts: Vec::new(),
            detail: None,
        });
    }
    warnings
}

#[cfg(test)]
mod tests {
    use hpr_design::{FinPlanform, FinSet, Material, NoseCone, Transition, Wall};

    use super::*;

    fn wood() -> Material {
        Material::bulk("wood", 600.0)
    }

    fn nose(shape: NoseShape) -> Part {
        Part::NoseCone(NoseCone {
            shape,
            length_m: 0.3,
            base_radius_m: 0.05,
            wall: Wall::Filled {},
            shoulder: None,
            material: wood(),
        })
    }

    fn fins(cross_section: FinCrossSection) -> Part {
        Part::FinSet(FinSet {
            count: 3,
            planform: FinPlanform::Trapezoidal {
                root_chord_m: 0.2,
                tip_chord_m: 0.1,
                span_m: 0.1,
                sweep_m: 0.05,
            },
            thickness_m: 0.004,
            cross_section,
            tab: None,
            fillet: None,
            cant_rad: 0.0,
            base_angle_rad: 0.0,
            material: wood(),
        })
    }

    /// A conical transition from radius 0.05 m narrowing at `angle_rad` over 0.1 m.
    fn boattail(angle_rad: f64) -> Part {
        let length_m = 0.1;
        Part::Transition(Transition {
            shape: NoseShape::Conical {},
            clipped: false,
            length_m,
            fore_radius_m: 0.05,
            aft_radius_m: 0.05 - length_m * angle_rad.tan(),
            wall: Wall::Filled {},
            fore_shoulder: None,
            aft_shoulder: None,
            material: wood(),
        })
    }

    fn at(mach: f64) -> Option<Peak> {
        Some(Peak {
            value: mach,
            time_s: 2.5,
            height_above_ground_m: 300.0,
        })
    }

    fn numbers(parts: &[Part], mach: f64) -> Vec<u32> {
        let ids = ["a", "b", "c", "d", "e"];
        issue_warnings(ids.into_iter().zip(parts), at(mach))
            .iter()
            .map(IssueWarning::number)
            .collect()
    }

    /// The next `f64` above `x`.
    fn above(x: f64) -> f64 {
        f64::from_bits(x.to_bits() + 1)
    }

    #[test]
    fn a_cone_or_an_ogive_nose_warns_just_past_mach_0_8() {
        for shape in [
            NoseShape::Conical {},
            NoseShape::TANGENT_OGIVE,
            NoseShape::Ogive { radius_ratio: 2.0 },
        ] {
            let parts = [nose(shape)];
            assert_eq!(numbers(&parts, TRANSONIC_NOSE_DRAG_MACH), Vec::<u32>::new());
            assert_eq!(numbers(&parts, above(TRANSONIC_NOSE_DRAG_MACH)), [67, 68]);
        }
    }

    #[test]
    fn other_noses_never_meet_67() {
        for shape in [
            NoseShape::VON_KARMAN,
            NoseShape::Haack {
                parameter: 1.0 / 3.0,
            },
            NoseShape::Elliptical {},
            NoseShape::PowerSeries { exponent: 0.5 },
            NoseShape::ParabolicSeries { parameter: 1.0 },
        ] {
            assert_eq!(numbers(&[nose(shape)], 0.95), [68], "{shape:?}");
        }
    }

    #[test]
    fn base_drag_warns_past_mach_0_8_and_adds_its_low_side_only_past_1_2() {
        assert_eq!(numbers(&[], BASE_DRAG_HIGH_MACH), Vec::<u32>::new());
        assert_eq!(numbers(&[], above(BASE_DRAG_HIGH_MACH)), [68]);
        let message = |mach: f64| issue_warnings(std::iter::empty(), at(mach))[0].message();
        let low_side = "from Mach 1.5 it reads low";
        assert!(message(BASE_DRAG_HIGH_TOP_MACH).contains("read low"));
        assert!(!message(BASE_DRAG_HIGH_TOP_MACH).contains(low_side));
        let past_top = message(above(BASE_DRAG_HIGH_TOP_MACH));
        assert!(past_top.contains(low_side), "{past_top}");
        assert!(past_top.contains("there read high"), "{past_top}");
    }

    /// The edges are the issues' numbers: a test against each literal, so moving a constant
    /// fails here as well as in the comparisons above.
    #[test]
    fn the_edges_are_the_issues_numbers() {
        assert_eq!(TRANSONIC_NOSE_DRAG_MACH, 0.8);
        assert_eq!(BASE_DRAG_HIGH_MACH, 0.8);
        assert_eq!(BASE_DRAG_HIGH_TOP_MACH, 1.2);
        assert_eq!(BASE_DRAG_LOW_MACH, 1.5);
        assert_eq!(SUPERSONIC_DRAG_MACH, 1.0);
        assert_eq!(STEEP_BOATTAIL_RAD, 10f64.to_radians());
        // And the messages quote them.
        let message = |issue: KnownIssue| {
            IssueWarning {
                issue,
                max_mach: at(2.0).unwrap(),
                parts: Vec::new(),
                detail: None,
            }
            .message()
        };
        assert!(message(KnownIssue::TransonicNoseDrag).contains("from Mach 0.8 "));
        assert!(message(KnownIssue::BaseDrag).contains("from Mach 0.8 to 1.2 "));
        assert!(message(KnownIssue::BaseDrag).contains("from Mach 1.5 it reads low"));
        assert!(message(KnownIssue::SteepBoattailDrag).contains("steeper than 10°"));
    }

    #[test]
    fn a_cone_like_shoulder_meets_67_a_von_karman_one_or_a_boattail_doesnt() {
        let transition = |shape: NoseShape, fore_radius_m: f64, aft_radius_m: f64| {
            Part::Transition(Transition {
                shape,
                clipped: false,
                length_m: 0.2,
                fore_radius_m,
                aft_radius_m,
                wall: Wall::Filled {},
                fore_shoulder: None,
                aft_shoulder: None,
                material: wood(),
            })
        };
        let von_karman = nose(NoseShape::VON_KARMAN);
        for shape in [NoseShape::Conical {}, NoseShape::TANGENT_OGIVE] {
            let shoulder = transition(shape, 0.027, 0.049);
            assert_eq!(numbers(&[von_karman.clone(), shoulder], 0.95), [67, 68]);
            // Narrowing at 6°, the same shape is a gentle boattail: neither #67 nor #72.
            let boattail = transition(shape, 0.049, 0.028);
            assert_eq!(numbers(&[von_karman.clone(), boattail], 0.95), [68]);
        }
        let shoulder = transition(NoseShape::VON_KARMAN, 0.027, 0.049);
        assert_eq!(numbers(&[von_karman, shoulder], 0.95), [68]);
    }

    #[test]
    fn a_series_nose_that_blends_in_the_cone_meets_67() {
        let below = |x: f64| f64::from_bits(x.to_bits() - 1);
        for (shape, meets) in [
            (NoseShape::PowerSeries { exponent: 1.0 }, true),
            (
                NoseShape::PowerSeries {
                    exponent: above(0.75),
                },
                true,
            ),
            (NoseShape::PowerSeries { exponent: 0.75 }, false),
            (NoseShape::ParabolicSeries { parameter: 0.0 }, true),
            (
                NoseShape::ParabolicSeries {
                    parameter: below(0.5),
                },
                true,
            ),
            (NoseShape::ParabolicSeries { parameter: 0.5 }, false),
        ] {
            let want: &[u32] = if meets { &[67, 68] } else { &[68] };
            assert_eq!(numbers(&[nose(shape)], 0.95), want, "{shape:?}");
        }
    }

    #[test]
    fn airfoil_fins_warn_just_past_mach_1_other_sections_never() {
        let parts = [fins(FinCrossSection::Airfoil)];
        assert_eq!(numbers(&parts, SUPERSONIC_DRAG_MACH), [68]);
        assert_eq!(numbers(&parts, above(SUPERSONIC_DRAG_MACH)), [68, 70, 222]);
        for section in [FinCrossSection::Square, FinCrossSection::Rounded] {
            assert_eq!(numbers(&[fins(section)], 3.0), [68], "{section:?}");
        }
    }

    #[test]
    fn a_boattail_warns_just_steeper_than_10_degrees_past_mach_1() {
        // The angle's round trip through tan and atan2 lands within a few ulps of where it
        // started, so the edge is probed a hair either side.
        let edge = STEEP_BOATTAIL_RAD;
        // Steeper than 9.46°, each also meets #73 (below).
        assert_eq!(numbers(&[boattail(edge * (1.0 - 1e-12))], 2.0), [68, 73]);
        assert_eq!(
            numbers(&[boattail(edge * (1.0 + 1e-12))], 2.0),
            [68, 72, 73]
        );
        assert_eq!(numbers(&[boattail(0.5)], SUPERSONIC_DRAG_MACH), [68, 73]);
        assert_eq!(
            numbers(&[boattail(0.5)], above(SUPERSONIC_DRAG_MACH)),
            [68, 72, 73]
        );
    }

    #[test]
    fn a_boattails_angle_is_its_mean_and_a_flare_or_a_level_transition_is_none() {
        assert_eq!(
            boattail_angle_rad(0.5, 0.25, 0.25),
            Some(45f64.to_radians())
        );
        assert_eq!(boattail_angle_rad(0.04, 0.05, 0.01), None);
        assert_eq!(boattail_angle_rad(0.05, 0.05, 0.01), None);
        // A narrowing of no length is a step, which the drag buildup doesn't take as a boattail.
        assert_eq!(boattail_angle_rad(0.05, 0.04, 0.0), None);
    }

    #[test]
    fn an_ogive_nose_meets_222_supersonic_a_cone_doesnt() {
        assert_eq!(
            numbers(
                &[nose(NoseShape::TANGENT_OGIVE)],
                above(SUPERSONIC_DRAG_MACH)
            ),
            [67, 68, 222]
        );
        assert_eq!(numbers(&[nose(NoseShape::Conical {})], 2.0), [67, 68]);
    }

    #[test]
    fn each_warning_lists_its_parts_by_id() {
        let parts = [
            nose(NoseShape::TANGENT_OGIVE),
            boattail(0.4),
            fins(FinCrossSection::Airfoil),
            fins(FinCrossSection::Square),
        ];
        let ids = ["nose", "tail", "fins", "square"];
        let warnings = issue_warnings(ids.into_iter().zip(&parts), at(1.5));
        let listed: Vec<(u32, Vec<&str>)> = warnings
            .iter()
            .map(|warning| {
                let parts = warning.parts.iter().map(String::as_str).collect();
                (warning.number(), parts)
            })
            .collect();
        assert_eq!(
            listed,
            [
                (67, vec!["nose"]),
                (68, vec![]),
                (70, vec!["fins"]),
                (72, vec!["tail"]),
                (222, vec!["nose", "fins"]),
                (73, vec!["tail"]),
            ]
        );
        for warning in &warnings {
            let message = warning.message();
            assert!(
                message.contains(&format!("issue #{}", warning.number())),
                "{message}"
            );
            assert!(message.contains(&warning.issue.url()), "{message}");
        }
    }

    #[test]
    fn a_nan_top_mach_raises_every_warning_its_shape_allows_and_none_raises_none() {
        let parts = [nose(NoseShape::TANGENT_OGIVE), boattail(0.4)];
        assert_eq!(numbers(&parts, f64::NAN), [67, 68, 72, 222, 73]);
        let ids = ["a", "b"];
        assert!(issue_warnings(ids.into_iter().zip(&parts), None).is_empty());
    }

    #[test]
    fn a_nan_radius_counts_as_a_steep_boattail() {
        let angle = boattail_angle_rad(f64::NAN, 0.04, 0.1);
        assert!(angle.is_some_and(|angle| past(angle, STEEP_BOATTAIL_RAD)));
    }

    #[test]
    fn each_switch_warns_just_past_mach_1_2_naming_its_part() {
        use hpr_aero::SupersonicFallback as Fallback;
        let step = Fallback::RadiusStep {
            component: "tube".to_owned(),
        };
        let lip = Fallback::LongLip {
            component: "lip".to_owned(),
        };
        let numbers = |fallback: &Fallback, mach: f64| -> Vec<(u32, Vec<String>)> {
            stability_issue_warnings(Some(fallback), None, at(mach))
                .into_iter()
                .map(|w| (w.number(), w.parts))
                .collect()
        };
        for (fallback, number, parts) in [
            (&step, 87, vec!["tube".to_owned()]),
            (&lip, 120, vec!["lip".to_owned()]),
            (&Fallback::SteepTip, 121, Vec::new()),
        ] {
            assert!(numbers(fallback, SUPERSONIC_SWITCH_MACH).is_empty());
            assert_eq!(
                numbers(fallback, above(SUPERSONIC_SWITCH_MACH)),
                vec![(number, parts.clone())]
            );
            assert_eq!(numbers(fallback, f64::NAN), vec![(number, parts)]);
        }
        assert!(numbers(&Fallback::Other, 3.0).is_empty());
        assert!(stability_issue_warnings(None, None, at(3.0)).is_empty());
        assert!(stability_issue_warnings(Some(&step), None, None).is_empty());
    }

    #[test]
    fn a_margin_meets_172_on_any_flight_that_has_one() {
        let margin = at(1.5);
        let numbers: Vec<u32> = stability_issue_warnings(None, margin, at(0.3))
            .iter()
            .map(IssueWarning::number)
            .collect();
        assert_eq!(numbers, vec![172]);
        assert!(stability_issue_warnings(None, None, at(0.3)).is_empty());
        let message = stability_issue_warnings(None, margin, at(0.3))[0].message();
        assert!(message.contains("0.1108 calibres"), "{message}");
        assert!(message.contains("four private designs"), "{message}");
    }

    fn trapezoid(sweep_m: f64) -> FinPlanform {
        FinPlanform::Trapezoidal {
            root_chord_m: 0.2,
            tip_chord_m: 0.1,
            span_m: 0.1,
            sweep_m,
        }
    }

    /// The 54 mm test design with its fin set's outline `planform`.
    fn swept(planform: FinPlanform) -> Layout {
        let mut rocket = crate::testing::design("synthetic-54mm-three-fin");
        for child in &mut rocket.stages[0].components[1].children {
            if let Part::FinSet(fins) = &mut child.part {
                fins.planform = planform.clone();
            }
        }
        rocket.layout().unwrap_or_else(|error| panic!("{error}"))
    }

    fn stability_numbers(layout: &Layout, mach: f64) -> Vec<u32> {
        layout_stability_issue_warnings(layout, at(mach))
            .iter()
            .map(IssueWarning::number)
            .collect()
    }

    #[test]
    fn a_forward_swept_fin_meets_64_just_past_mach_1() {
        // The tip's leading edge a hair ahead of the root's: swept forward.
        let forward = swept(trapezoid(-1e-9));
        assert_eq!(stability_numbers(&forward, above(FORWARD_SWEEP_MACH)), [64]);
        assert!(stability_numbers(&forward, FORWARD_SWEEP_MACH).is_empty());
        // It is a margin error, not a drag one: the drag warnings never list it.
        assert!(
            !layout_issue_warnings(&forward, at(3.0))
                .iter()
                .any(|warning| warning.number() == 64)
        );
        // A straight leading edge, or one swept aft, never.
        for sweep_m in [0.0, 0.05] {
            assert!(stability_numbers(&swept(trapezoid(sweep_m)), 3.0).is_empty());
        }
        let ellipse = FinPlanform::Elliptical {
            root_chord_m: 0.2,
            span_m: 0.1,
        };
        assert!(stability_numbers(&swept(ellipse), 3.0).is_empty());
        // A freeform outline with a point ahead of the root's leading edge, and one without.
        let freeform = |tip_x_m: f64| FinPlanform::Freeform {
            points_m: vec![[0.0, 0.0], [tip_x_m, 0.1], [0.15, 0.1], [0.2, 0.0]],
            root_m: Vec::new(),
        };
        // Every freeform set also meets #326.
        assert_eq!(stability_numbers(&swept(freeform(-1e-9)), 1.5), [64, 326]);
        assert_eq!(stability_numbers(&swept(freeform(0.0)), 1.5), [326]);
        // A kinked leading edge that turns forward past its kink, its tip still aft of the root's
        // leading edge; and the same kink turning straight out.
        let kinked = |tip_x_m: f64| FinPlanform::Freeform {
            points_m: vec![
                [0.0, 0.0],
                [0.1, 0.05],
                [tip_x_m, 0.1],
                [0.15, 0.1],
                [0.2, 0.0],
            ],
            root_m: Vec::new(),
        };
        assert_eq!(
            stability_numbers(&swept(kinked(0.1 - 1e-9)), 1.5),
            [64, 326]
        );
        assert_eq!(stability_numbers(&swept(kinked(0.1)), 1.5), [326]);
        // A NaN sweep or point counts (a layout refuses one, so the rule is asked directly).
        assert!(sweeps_forward(&trapezoid(f64::NAN)));
        assert!(sweeps_forward(&freeform(f64::NAN)));
        assert!(KnownIssue::ForwardSweptFinSlope.is_stability());
        assert_eq!(FORWARD_SWEEP_MACH, 1.0);
        assert_eq!(hpr_aero::fins::SUPERSONIC_START_MACH, 1.2);
    }

    #[test]
    fn a_boattail_meets_73_steeper_than_where_the_rule_gives_drag_at_any_speed() {
        // The edge is where Niskanen's rule starts to give a boattail drag: γ = 3.
        assert!((SUBSONIC_BOATTAIL_RULE_RAD - (1.0_f64 / 6.0).atan()).abs() < 1e-16);
        let gamma = |angle_rad: f64| 1.0 / (2.0 * angle_rad.tan());
        let factor = |angle_rad: f64| {
            hpr_aero::drag::boattail_factor(gamma(angle_rad), 1.0, 0.0).unwrap_or(f64::NAN)
        };
        let edge = SUBSONIC_BOATTAIL_RULE_RAD;
        assert!(factor(edge * (1.0 + 1e-9)) > 0.0);
        assert_eq!(factor(edge * (1.0 - 1e-9)), 0.0);
        // A hair either side of it, at a walking pace and past Mach 1.
        assert_eq!(
            numbers(&[boattail(edge * (1.0 - 1e-12))], 0.3),
            Vec::<u32>::new()
        );
        assert_eq!(numbers(&[boattail(edge * (1.0 + 1e-12))], 0.3), [73]);
        assert_eq!(numbers(&[boattail(edge * (1.0 - 1e-12))], 0.9), [68]);
        assert_eq!(numbers(&[boattail(edge * (1.0 + 1e-12))], 0.9), [68, 73]);
        // A level transition or a flare never.
        assert_eq!(numbers(&[boattail(0.0)], 0.3), Vec::<u32>::new());
        assert_eq!(numbers(&[boattail(-0.3)], 0.3), Vec::<u32>::new());
        assert!(!KnownIssue::SubsonicBoattailDrag.is_stability());
        let warning = &issue_warnings([("tail", &boattail(0.3))], at(0.3))[0];
        let message = warning.message();
        assert!(message.contains("steeper than 9.5°"), "{message}");
        assert!(message.contains("below Mach 0.8 "), "{message}");
        assert!(!message.contains("this flight reaches"), "{message}");
    }

    /// The 54 mm test design with its fin set replaced by `sets`, each a count of fins and how
    /// far its root's trailing edge sits forward of the airframe's aft end, m. Root chords are
    /// 0.15 m.
    fn finned(sets: &[(u32, f64)]) -> Layout {
        use hpr_design::Position;
        let mut rocket = crate::testing::design("synthetic-54mm-three-fin");
        let airframe = &mut rocket.stages[0].components[1];
        let index = airframe
            .children
            .iter()
            .position(|child| child.id == "sustainer-fins")
            .unwrap_or_else(|| panic!("the test design has a fin set"));
        let original = airframe.children.remove(index);
        for (number, &(count, from_aft_m)) in sets.iter().enumerate() {
            let mut set = original.clone();
            set.id = format!("fins{number}");
            set.position = Some(Position::Bottom {
                aft_offset_m: -from_aft_m,
            });
            if let Part::FinSet(fins) = &mut set.part {
                fins.count = count;
            }
            airframe.children.push(set);
        }
        rocket.layout().unwrap_or_else(|error| panic!("{error}"))
    }

    fn station_numbers(layout: &Layout) -> Vec<(u32, Vec<String>)> {
        layout_stability_issue_warnings(layout, at(0.3))
            .into_iter()
            .map(|warning| (warning.number(), warning.parts))
            .collect()
    }

    #[test]
    fn fin_sets_sharing_a_station_meet_325_past_four_fins() {
        let both = vec![(325, vec!["fins0".to_owned(), "fins1".to_owned()])];
        // Three and three at one station, overlapping, and touching end to end.
        assert_eq!(station_numbers(&finned(&[(3, 0.0), (3, 0.0)])), both);
        assert_eq!(station_numbers(&finned(&[(3, 0.0), (3, 0.1)])), both);
        assert_eq!(station_numbers(&finned(&[(3, 0.0), (3, 0.15)])), both);
        // A millimeter apart, never.
        assert!(station_numbers(&finned(&[(3, 0.0), (3, 0.151)])).is_empty());
        // Four fins between them have no interference to miss: two and two, three and one.
        assert!(station_numbers(&finned(&[(2, 0.0), (2, 0.0)])).is_empty());
        assert!(station_numbers(&finned(&[(3, 0.0), (1, 0.0)])).is_empty());
        // Five fins, three and two: both listed.
        assert_eq!(station_numbers(&finned(&[(3, 0.0), (2, 0.0)])), both);
        // One set of six alone is counted as six already.
        assert!(station_numbers(&finned(&[(6, 0.0)])).is_empty());
        // A third set apart is not listed.
        assert_eq!(
            station_numbers(&finned(&[(3, 0.0), (3, 0.0), (3, 0.5)])),
            both
        );
        assert!(KnownIssue::FinSetsAtOneStation.is_stability());
        let warning = &layout_stability_issue_warnings(&finned(&[(3, 0.0), (3, 0.0)]), at(0.3))[0];
        let message = warning.message();
        assert!(message.contains("issue #325"), "{message}");
        assert!(message.contains("more than 4 fins"), "{message}");
        assert!(!message.contains("this flight reaches"), "{message}");
        // No top Mach number, no flight: nothing.
        assert!(layout_stability_issue_warnings(&finned(&[(3, 0.0), (3, 0.0)]), None).is_empty());
        // Nor among the drag warnings.
        assert!(layout_issue_warnings(&finned(&[(3, 0.0), (3, 0.0)]), at(0.3)).is_empty());
    }

    #[test]
    fn every_flight_on_hprs_drag_meets_18_and_none_without_a_top_mach() {
        // At any speed, and apart from the shape's warnings.
        for mach in [0.05, 0.3, 3.0] {
            let warnings = friction_issue_warnings(at(mach));
            let numbers: Vec<u32> = warnings.iter().map(IssueWarning::number).collect();
            assert_eq!(numbers, [18]);
            assert!(!self::numbers(&[nose(NoseShape::VON_KARMAN)], mach).contains(&18));
        }
        assert!(friction_issue_warnings(None).is_empty());
        assert!(!KnownIssue::TurbulentFriction.is_stability());
        let warning = &friction_issue_warnings(at(0.3))[0];
        let message = warning.message();
        assert!(message.contains("issue #18"), "{message}");
        assert!(
            message.contains("by 3.6% on RocketPy's Calisto"),
            "{message}"
        );
        assert!(
            message.contains("the apogee, the top speed and the drift read low"),
            "{message}"
        );
        assert!(!message.contains("this flight reaches"), "{message}");
        assert!(warning.parts.is_empty());
    }

    #[test]
    fn the_laminar_share_is_barrowmans_term_over_niskanens_turbulent_friction() {
        let reynolds: f64 = 1.8e7;
        let turbulent = 1.0 / (1.50 * reynolds.ln() - 5.6).powi(2);
        let percent = 100.0 * (1700.0 / reynolds) / turbulent;
        assert!(
            (percent - LAMINAR_FRICTION_PERCENT).abs() < 0.05,
            "{percent}"
        );
    }

    #[test]
    fn a_freeform_fin_set_meets_326_at_any_speed_a_trapezoid_or_an_ellipse_never() {
        let outline = FinPlanform::Freeform {
            points_m: vec![[0.0, 0.0], [0.05, 0.05], [0.1, 0.1], [0.2, 0.0]],
            root_m: Vec::new(),
        };
        let freeform = swept(outline);
        for mach in [0.05, 0.3, 3.0] {
            assert!(stability_numbers(&freeform, mach).contains(&326), "{mach}");
        }
        let warning = layout_stability_issue_warnings(&freeform, at(0.3))
            .into_iter()
            .find(|warning| warning.number() == 326)
            .unwrap_or_else(|| panic!("no #326"));
        assert_eq!(warning.parts.len(), 1);
        let message = warning.message();
        assert!(message.contains("issue #326"), "{message}");
        assert!(message.contains("0.047 calibres"), "{message}");
        assert!(!message.contains("this flight reaches"), "{message}");
        assert!(KnownIssue::FreeformFinCenterOfPressure.is_stability());
        // The same design's trapezoid and an ellipse never.
        assert!(!stability_numbers(&swept(trapezoid(0.05)), 3.0).contains(&326));
        let ellipse = FinPlanform::Elliptical {
            root_chord_m: 0.2,
            span_m: 0.1,
        };
        assert!(!stability_numbers(&swept(ellipse), 3.0).contains(&326));
        // Not among the drag warnings, and no top Mach number, no flight: nothing.
        assert!(
            !layout_issue_warnings(&freeform, at(0.3))
                .iter()
                .any(|warning| warning.number() == 326)
        );
        assert!(layout_stability_issue_warnings(&freeform, None).is_empty());
    }

    /// The synthetic 54 mm design with its rail buttons, the only part off its axis, taken out.
    fn buttonless() -> hpr_design::Rocket {
        let mut rocket = crate::testing::design("synthetic-54mm-three-fin");
        let airframe = &mut rocket.stages[0].components[1];
        let before = airframe.children.len();
        airframe
            .children
            .retain(|child| !matches!(child.part, Part::RailButton(_)));
        assert_eq!(
            airframe.children.len(),
            before - 1,
            "the design has one button row"
        );
        rocket
    }

    /// The center of gravity's distance from the axis, m, at launch and at burnout.
    fn lateral_offsets_m(rocket: &hpr_design::Rocket) -> [f64; 2] {
        let assembly = rocket
            .assemble(&rocket.configurations[0].id)
            .unwrap_or_else(|error| panic!("{error}"));
        [
            assembly.mass_properties(0.0),
            assembly.dry_mass_properties(),
        ]
        .map(|mass| mass.cg_m.x.hypot(mass.cg_m.y))
    }

    fn off_axis(offset_m: [f64; 2]) -> Vec<IssueWarning> {
        let mass = |x: f64| MassProperties {
            mass_kg: 1.0,
            cg_m: hpr_core::DVec3::new(x, 0.0, -0.5),
            inertia_kg_m2: hpr_core::DMat3::IDENTITY,
        };
        center_of_gravity_issue_warnings(&[mass(offset_m[0]), mass(offset_m[1])], at(0.5))
    }

    #[test]
    fn a_center_of_gravity_off_the_axis_meets_219_past_its_limit() {
        // M10.1d6 (ADR-201): at the limit nothing; the next float above, at launch or at
        // burnout, warns; a NaN distance warns; no top Mach number, nothing.
        assert!(off_axis([OFF_AXIS_CG_LIMIT_M, 0.0]).is_empty());
        assert!(off_axis([-OFF_AXIS_CG_LIMIT_M, OFF_AXIS_CG_LIMIT_M]).is_empty());
        let edge = above(OFF_AXIS_CG_LIMIT_M);
        for offsets in [
            [edge, 0.0],
            [0.0, edge],
            [-edge, 0.0],
            [f64::NAN, 0.0],
            [0.0, f64::NAN],
        ] {
            let warnings = off_axis(offsets);
            assert_eq!(
                warnings
                    .iter()
                    .map(IssueWarning::number)
                    .collect::<Vec<_>>(),
                [219],
                "{offsets:?}"
            );
            assert!(warnings[0].parts.is_empty());
        }
        let mass = MassProperties {
            mass_kg: 1.0,
            cg_m: hpr_core::DVec3::new(0.0, 2.0 * OFF_AXIS_CG_LIMIT_M, 0.0),
            inertia_kg_m2: hpr_core::DMat3::IDENTITY,
        };
        assert!(center_of_gravity_issue_warnings(&[mass, mass], None).is_empty());
        assert_eq!(
            center_of_gravity_issue_warnings(&[mass, mass], at(0.5)).len(),
            1
        );
        // No samples at all is nothing off the axis; the largest of many is the one quoted.
        assert!(center_of_gravity_issue_warnings(&[], at(0.5)).is_empty());
        let shifted = |x: f64| MassProperties {
            cg_m: hpr_core::DVec3::new(x, 0.0, -0.5),
            ..mass
        };
        let [warning] = &center_of_gravity_issue_warnings(
            &[shifted(0.0), shifted(3e-4), shifted(-1e-4)],
            at(0.5),
        )[..] else {
            panic!("one warning")
        };
        assert_eq!(
            warning.detail,
            Some(IssueDetail::CenterOfGravityOffset { offset_m: 3e-4 })
        );
        // The message quotes the larger distance in millimeters, and which way it leans.
        let [warning] = &off_axis([2.5e-4, 7.5e-5])[..] else {
            panic!("one warning")
        };
        let message = warning.message();
        assert!(
            message.starts_with("issue #219: the center of gravity sits 0.250 mm off"),
            "{message}"
        );
        assert!(
            message.contains("no reference measures which is right"),
            "{message}"
        );
        assert!(
            message.contains("the apogee and the drift may read low or high"),
            "{message}"
        );
        assert!(!message.contains("Mach"), "{message}");
        assert!(
            off_axis([f64::NAN, 0.0])[0]
                .message()
                .contains("an unknown distance")
        );
        assert!(
            off_axis([edge, 0.0])[0]
                .message()
                .contains("under 0.001 mm")
        );
        assert_eq!(OFF_AXIS_CG_LIMIT_M, 1e-9);
    }

    #[test]
    fn a_rail_button_puts_the_test_rocket_off_its_axis_and_none_leaves_it_on() {
        // The synthetic design's button row at 60° puts its center of gravity 5.75e-5 m off the
        // axis at launch and 7.88e-5 m at burnout, above the limit by far; without the buttons it
        // sits on the axis to rounding, far below it.
        let rocket = crate::testing::design("synthetic-54mm-three-fin");
        let [launch, burnout] = lateral_offsets_m(&rocket);
        assert!((launch / 5.75e-5 - 1.0).abs() < 0.01, "{launch:e}");
        assert!((burnout / 7.88e-5 - 1.0).abs() < 0.01, "{burnout:e}");
        let symmetric = buttonless();
        for offset_m in lateral_offsets_m(&symmetric) {
            assert!(offset_m < 1e-15, "{offset_m:e}");
        }
        let warnings = |rocket: &hpr_design::Rocket| {
            let assembly = rocket.assemble(&rocket.configurations[0].id).unwrap();
            center_of_gravity_issue_warnings(
                &[
                    assembly.mass_properties(0.0),
                    assembly.dry_mass_properties(),
                ],
                at(0.5),
            )
        };
        let [warning] = &warnings(&rocket)[..] else {
            panic!("one warning")
        };
        assert_eq!(
            warning.detail,
            Some(IssueDetail::CenterOfGravityOffset { offset_m: burnout })
        );
        assert!(
            warning
                .message()
                .contains(&format!("sits {:.3} mm off", burnout * 1000.0)),
            "{}",
            warning.message()
        );
        assert!(warnings(&symmetric).is_empty());
    }

    /// The synthetic design with a pod set of `count` pods `offset_m` from the axis on its
    /// airframe, each a tube 0.2 m long and 10 mm in radius, or holding nothing when `empty`.
    fn podded(count: u32, offset_m: f64, empty: bool) -> Layout {
        let mut rocket = buttonless();
        let airframe = &mut rocket.stages[0].components[1];
        let mut pods = airframe.children[0].clone();
        pods.id = "pods".to_owned();
        pods.part = Part::PodSet(hpr_design::PodSet {
            count,
            radial_offset_m: offset_m,
            angle_rad: 0.3,
        });
        pods.position = Some(hpr_design::Position::Top { aft_offset_m: 0.1 });
        pods.motor_mount = None;
        pods.children = Vec::new();
        if !empty {
            let mut tube = airframe.clone();
            tube.id = "pod-tube".to_owned();
            tube.children = Vec::new();
            tube.part = Part::BodyTube(hpr_design::BodyTube {
                length_m: 0.2,
                outer_radius_m: 0.01,
                thickness_m: 0.001,
                material: wood(),
            });
            pods.children.push(tube);
        }
        airframe.children.push(pods);
        rocket.layout().unwrap_or_else(|error| panic!("{error}"))
    }

    fn pod_numbers(layout: &Layout) -> Vec<(u32, Vec<String>)> {
        pod_issue_warnings(layout, at(0.5))
            .into_iter()
            .map(|warning| (warning.number(), warning.parts))
            .collect()
    }

    #[test]
    fn a_single_pod_off_the_axis_meets_213_and_evenly_spaced_pods_dont() {
        // M10.1d6 (ADR-201): one pod off the axis that holds a part warns, naming its set, at any
        // offset above zero; two or three pods, one on the axis, or one holding nothing don't.
        let named = vec![(213, vec!["pods".to_owned()])];
        assert_eq!(pod_numbers(&podded(1, 0.04, false)), named);
        assert_eq!(pod_numbers(&podded(1, f64::MIN_POSITIVE, false)), named);
        for count in [2, 3] {
            assert!(
                pod_numbers(&podded(count, 0.04, false)).is_empty(),
                "{count}"
            );
        }
        assert!(pod_numbers(&podded(1, 0.0, false)).is_empty());
        assert!(pod_numbers(&podded(1, 0.04, true)).is_empty());
        assert!(pod_issue_warnings(&podded(1, 0.04, false), None).is_empty());
        // The symmetric rocket has no pods at all.
        let bare = buttonless().layout().unwrap();
        assert!(pod_numbers(&bare).is_empty());
        // A NaN offset counts as off the axis; a layout can't hold one, so set it by hand.
        let mut broken = podded(1, 0.04, false);
        for component in &mut broken.components {
            if let Part::PodSet(set) = &mut component.part {
                set.radial_offset_m = f64::NAN;
            }
        }
        assert_eq!(pod_numbers(&broken), named);
        // A single strap-on booster, a parallel stage of one pod, counts as one pod.
        let mut boosted = buttonless();
        let mut tube = boosted.stages[0].components[1].clone();
        tube.id = "booster-tube".to_owned();
        tube.children = Vec::new();
        tube.part = Part::BodyTube(hpr_design::BodyTube {
            length_m: 0.3,
            outer_radius_m: 0.012,
            thickness_m: 0.001,
            material: wood(),
        });
        let mut stage = boosted.stages[0].clone();
        stage.id = "booster".to_owned();
        stage.components = vec![tube];
        stage.parallel = Some(hpr_design::ParallelStage {
            on: "sustainer-airframe".to_owned(),
            position: hpr_design::Position::Top { aft_offset_m: 0.4 },
            pods: hpr_design::PodSet {
                count: 1,
                radial_offset_m: 0.045,
                angle_rad: 0.0,
            },
        });
        boosted.stages.push(stage);
        let layout = boosted.layout().unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(pod_numbers(&layout), [(213, vec!["booster".to_owned()])]);
        // The message says which way the flight may move.
        let [warning] = &pod_issue_warnings(&podded(1, 0.04, false), at(0.5))[..] else {
            panic!("one warning")
        };
        let message = warning.message();
        assert!(
            message.starts_with("issue #213: a single pod off the axis"),
            "{message}"
        );
        assert!(
            message.contains("the drift may read short and the apogee low, or the reverse"),
            "{message}"
        );
        assert!(
            message.contains("nothing measures by how much"),
            "{message}"
        );
    }

    /// The synthetic design's aerodynamics, with `extra` appended to its airframe's stack.
    fn synthetic_aero(extra: Option<Part>) -> AeroModel {
        let mut rocket = buttonless();
        if let Some(part) = extra {
            let mut tail = rocket.stages[0].components[1].clone();
            tail.id = "tail".to_owned();
            tail.children = Vec::new();
            tail.auto = Vec::new();
            tail.part = part;
            rocket.stages[0].components.push(tail);
        }
        AeroModel::new(&rocket.layout().unwrap_or_else(|error| panic!("{error}")))
            .unwrap_or_else(|error| panic!("{error}"))
    }

    fn station_warnings(aero: &AeroModel, mach: f64) -> Vec<IssueWarning> {
        body_station_issue_warnings(&[(aero, at(mach))], at(mach))
    }

    /// One join, the stack's.
    fn stack_join(join_start_mach: f64, join_end_mach: Option<f64>) -> Option<IssueDetail> {
        Some(IssueDetail::SupersonicJoins {
            joins: vec![SupersonicJoin {
                vehicle: 0,
                join_start_mach,
                join_end_mach,
            }],
        })
    }

    #[test]
    fn a_body_station_meets_106_just_past_the_joins_start() {
        // M10.1d6 (ADR-201): the synthetic design's body takes the shock-expansion method from
        // Mach 1.2, its nose's and airframe's stations joined over 0.3 in Mach.
        let aero = synthetic_aero(None);
        let body = aero.supersonic_body().unwrap();
        let start = body.join_start_mach;
        assert_eq!(start, SUPERSONIC_SWITCH_MACH);
        assert!(station_warnings(&aero, start).is_empty());
        assert!(station_warnings(&aero, 0.9).is_empty());
        assert!(body_station_issue_warnings(&[(&aero, at(2.0))], None).is_empty());
        assert!(body_station_issue_warnings(&[(&aero, None)], at(2.0)).is_empty());
        assert!(body_station_issue_warnings(&[], at(2.0)).is_empty());
        for mach in [above(start), 2.3, f64::NAN] {
            let [warning] = &station_warnings(&aero, mach)[..] else {
                panic!("one warning at Mach {mach}")
            };
            assert_eq!(warning.number(), 106);
            assert_eq!(warning.parts, ["nose", "sustainer-airframe"]);
            assert_eq!(
                warning.detail,
                stack_join(start, Some(start + hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH))
            );
            let message = warning.message();
            assert!(message.contains("from Mach 1.20 to 1.50"), "{message}");
            assert!(message.contains("nothing bounds by how much"), "{message}");
            assert!(message.contains("this flight reaches Mach"), "{message}");
        }
        // The premise, as the message quotes it: inside the join the airframe's station sits
        // calibres from its own small-angle center of pressure (4.71 at Mach 1.3); at the join's end the two
        // agree again.
        let index = aero
            .bodies()
            .iter()
            .position(|part| part.id == "sustainer-airframe")
            .unwrap();
        // The small-angle center of pressure, at an angle small enough that body lift's share,
        // which grows with the angle, is lost in rounding.
        let gap_cal = |mach: f64| {
            let station = aero.component_station_m(index, mach).unwrap();
            let force = aero
                .component_normal_force(index, &hpr_aero::Flow::new(mach, 1e-10, 0.0))
                .unwrap();
            let cp = force.moment_slope_m / force.slope_per_rad;
            (station - cp).abs() / aero.reference_diameter_m()
        };
        let inside = gap_cal(1.3);
        assert!((inside - 4.71).abs() < 0.005, "{inside}");
        let end = gap_cal(start + hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH + 0.01);
        assert!(end < 1e-6, "{end}");
    }

    #[test]
    fn each_vehicle_meets_106_by_its_own_join_and_its_own_top_speed() {
        // M10.1d6 (ADR-201): a two-stage stack's body is longer than its sustainer's, so its
        // join starts later; each vehicle is judged on its own join and the speed it reached
        // while it flew, and the warning keeps each one's join, naming the vehicles.
        let two_stage = crate::testing::design("synthetic-two-stage-75mm-54mm");
        let stack = AeroModel::new(&two_stage.layout().unwrap()).unwrap();
        let stack_start = stack.supersonic_body().unwrap().join_start_mach;
        assert!(stack_start > 1.35, "{stack_start}");
        let sustainer = synthetic_aero(None);
        let sustainer_start = sustainer.supersonic_body().unwrap().join_start_mach;
        assert_eq!(sustainer_start, SUPERSONIC_SWITCH_MACH);
        let width = hpr_aero::SUPERSONIC_JOIN_WIDTH_MACH;
        let join = |vehicle, start: f64| SupersonicJoin {
            vehicle,
            join_start_mach: start,
            join_end_mach: Some(start + width),
        };
        let warn = |stack_top: f64, sustainer_top: f64, flight_top: f64| {
            body_station_issue_warnings(
                &[(&stack, at(stack_top)), (&sustainer, at(sustainer_top))],
                at(flight_top),
            )
        };
        let joins_of = |warnings: &[IssueWarning]| match warnings {
            [warning] => match &warning.detail {
                Some(IssueDetail::SupersonicJoins { joins }) => joins.clone(),
                other => panic!("{other:?}"),
            },
            other => panic!("{other:?}"),
        };
        // The sustainer alone past its own join, the stack below its: the sustainer's join, by
        // name, and its parts.
        let alone = warn(1.33, 1.33, 1.33);
        assert_eq!(joins_of(&alone), [join(1, sustainer_start)]);
        assert_eq!(alone[0].parts, ["nose", "sustainer-airframe"]);
        let message = alone[0].message();
        assert!(
            message.contains(
                ", from Mach 1.20 to 1.50 on the sustainer after the first separation, a body"
            ),
            "{message}"
        );
        // The stack alone: its own join, unnamed, as a single stage's.
        let stacked = warn(1.5, 1.1, 1.5);
        assert_eq!(joins_of(&stacked), [join(0, stack_start)]);
        let message = stacked[0].message();
        assert!(
            message.contains(&format!(
                ", from Mach {stack_start:.2} to {:.2}, a body",
                stack_start + width
            )),
            "{message}"
        );
        // Both: each join, each named, and the parts of both, each once.
        let both = warn(1.5, 1.6, 1.6);
        assert_eq!(
            joins_of(&both),
            [join(0, stack_start), join(1, sustainer_start)]
        );
        assert_eq!(both[0].parts[..2], ["nose", "sustainer-airframe"]);
        let mut unique = both[0].parts.clone();
        unique.dedup();
        assert_eq!(unique, both[0].parts);
        let message = both[0].message();
        assert!(
            message.contains(&format!(
                "from Mach {stack_start:.2} to {:.2} on the launch stack and from Mach 1.20 to \
                 1.50 on the sustainer after the first separation",
                stack_start + width
            )),
            "{message}"
        );
        // Neither past its own join, though the flight's top is past the sustainer's: none.
        assert!(warn(1.3, 1.1, 1.3).is_empty());
        // At each join's start exactly, none; a NaN top for the flight warns of both.
        assert!(warn(stack_start, sustainer_start, stack_start).is_empty());
        assert_eq!(joins_of(&warn(0.5, 0.5, f64::NAN)).len(), 2);
        // A vehicle that took no step meets none.
        assert!(
            body_station_issue_warnings(&[(&stack, at(1.0)), (&sustainer, None)], at(1.0))
                .is_empty()
        );
        // A later sustainer is named by its separation.
        let third = IssueWarning {
            issue: KnownIssue::BodyDampingStation,
            max_mach: at(1.6).unwrap(),
            parts: Vec::new(),
            detail: Some(IssueDetail::SupersonicJoins {
                joins: vec![join(2, 1.25)],
            }),
        };
        assert!(
            third
                .message()
                .contains("from Mach 1.25 to 1.55 on the sustainer after separation 2,"),
            "{}",
            third.message()
        );
    }

    #[test]
    fn an_issue_kinds_name_is_its_serialized_form() {
        // Every kind has its place in the list: a new one fails to compile here until it has.
        let place = |kind: IssueKind| match kind {
            IssueKind::Drag => 0,
            IssueKind::Stability => 1,
            IssueKind::Flight => 2,
        };
        for (index, kind) in IssueKind::ALL.into_iter().enumerate() {
            assert_eq!(place(kind), index);
            assert_eq!(
                serde_json::to_value(kind).unwrap(),
                serde_json::Value::from(kind.name())
            );
        }
        assert_eq!(
            IssueKind::ALL.map(IssueKind::name),
            ["drag", "stability", "flight"]
        );
    }

    #[test]
    fn a_boattail_keeps_106_at_every_speed_and_a_step_in_radius_meets_none() {
        // A conical boattail behind the airframe keeps slender-body theory's station while its
        // force takes the method's, so its gap holds past the join's end: the message says every
        // speed past the start.
        let boattail = Part::Transition(Transition {
            shape: NoseShape::Conical {},
            clipped: false,
            length_m: 0.06,
            fore_radius_m: 0.02815,
            aft_radius_m: 0.022,
            wall: Wall::Filled {},
            fore_shoulder: None,
            aft_shoulder: None,
            material: wood(),
        });
        let aero = synthetic_aero(Some(boattail));
        let body = aero.supersonic_body().unwrap();
        assert!(!body.has_station(2) && body.has_station(0));
        let [warning] = &station_warnings(&aero, 2.0)[..] else {
            panic!("one warning")
        };
        assert_eq!(warning.parts, ["nose", "sustainer-airframe", "tail"]);
        let message = warning.message();
        assert!(
            message.contains(&format!(
                "at every speed past Mach {:.2}",
                body.join_start_mach
            )),
            "{message}"
        );
        // A step down in radius takes the body off the method: no table, so no station issue.
        let step = Part::BodyTube(hpr_design::BodyTube {
            length_m: 0.1,
            outer_radius_m: 0.02,
            thickness_m: 0.001,
            material: wood(),
        });
        let stepped = synthetic_aero(Some(step));
        assert!(stepped.supersonic_body().is_none());
        assert!(station_warnings(&stepped, 2.0).is_empty());
        // Both forms of the message, as a record would hold them.
        let quoted = |join_end_mach| {
            IssueWarning {
                issue: KnownIssue::BodyDampingStation,
                max_mach: at(1.6).unwrap(),
                parts: Vec::new(),
                detail: stack_join(1.25, join_end_mach),
            }
            .message()
        };
        assert!(quoted(Some(1.55)).contains(", from Mach 1.25 to 1.55, a body part's pitch"));
        assert!(quoted(None).contains(", at every speed past Mach 1.25, a body part's pitch"));
        assert!(quoted(None).contains("the apogee and the drift may lean either way"));
    }

    fn turn(lower: (f64, f64, f64), upper: (f64, f64, f64)) -> DirectionTurn {
        let level = |(height_msl_m, speed_m_s, direction_from_rad)| hpr_atmos::WindLevel {
            height_msl_m,
            speed_m_s,
            direction_from_rad,
        };
        DirectionTurn {
            lower: level(lower),
            upper: level(upper),
        }
    }

    /// The flight's heights in the wind tests: a site 1,400 m above sea level, a top 1,140 m
    /// above it.
    const HEIGHTS_MSL_M: [f64; 2] = [1400.0, 2540.0];

    fn wind_numbers(turns: &[DirectionTurn]) -> Vec<u32> {
        wind_issue_warnings(turns, HEIGHTS_MSL_M, at(0.5))
            .iter()
            .map(IssueWarning::number)
            .collect()
    }

    #[test]
    fn a_near_calm_level_that_turns_inside_the_flight_meets_8() {
        // M10.1d6 (ADR-201): the issue's case, 0.1 m/s from the north under 10 m/s from the
        // west, inside the flight.
        let west = 1.5 * std::f64::consts::PI;
        let calm_over = |speed: f64| turn((1500.0, speed, 0.0), (1800.0, 10.0, west));
        assert_eq!(wind_numbers(&[calm_over(0.1)]), [8]);
        // The calm limit: at it warns, the next float above doesn't; exactly calm doesn't (the
        // table takes the other's direction); the smallest speed above calm does; NaN does.
        assert_eq!(wind_numbers(&[calm_over(NEAR_CALM_WIND_M_S)]), [8]);
        assert!(wind_numbers(&[calm_over(above(NEAR_CALM_WIND_M_S))]).is_empty());
        assert!(wind_numbers(&[calm_over(0.0)]).is_empty());
        assert_eq!(wind_numbers(&[calm_over(f64::from_bits(1))]), [8]);
        assert_eq!(wind_numbers(&[calm_over(f64::MIN_POSITIVE)]), [8]);
        assert_eq!(wind_numbers(&[calm_over(f64::NAN)]), [8]);
        // Either level may be the calm one; a calm upper level with a zero lower doesn't.
        assert_eq!(
            wind_numbers(&[turn((1500.0, 10.0, west), (1800.0, 0.4, 0.0))]),
            [8]
        );
        assert!(wind_numbers(&[turn((1500.0, 0.0, west), (1800.0, 0.4, 0.0))]).is_empty());
        // Equal directions don't turn; directions 1e-12 rad apart do.
        assert!(wind_numbers(&[turn((1500.0, 0.1, west), (1800.0, 10.0, west))]).is_empty());
        assert_eq!(
            wind_numbers(&[turn((1500.0, 0.1, 1e-12), (1800.0, 10.0, 0.0))]),
            [8]
        );
        // A span just above the top or just below the ground doesn't; one that reaches a float
        // inside either does.
        let [ground, top] = HEIGHTS_MSL_M;
        let span = |lower: f64, upper: f64| turn((lower, 0.1, 0.0), (upper, 10.0, west));
        assert!(wind_numbers(&[span(top, 3000.0)]).is_empty());
        assert!(wind_numbers(&[span(above(top), 3000.0)]).is_empty());
        assert_eq!(wind_numbers(&[span(top.next_down(), 3000.0)]), [8]);
        assert!(wind_numbers(&[span(0.0, ground)]).is_empty());
        assert_eq!(wind_numbers(&[span(0.0, above(ground))]), [8]);
        assert_eq!(wind_numbers(&[span(0.0, 5000.0)]), [8]);
        assert_eq!(
            wind_issue_warnings(&[span(top, 3000.0)], [ground, f64::NAN], at(0.5)).len(),
            1
        );
        assert!(wind_issue_warnings(&[calm_over(0.1)], HEIGHTS_MSL_M, None).is_empty());
        // The warning keeps only the spans that count, lowest first, and quotes the first.
        let spans = [span(top, 3000.0), calm_over(0.1), calm_over(0.3)];
        let [warning] = &wind_issue_warnings(&spans, HEIGHTS_MSL_M, at(0.5))[..] else {
            panic!("one warning")
        };
        assert_eq!(
            warning.detail,
            Some(IssueDetail::DirectionTurns {
                turns: vec![spans[1], spans[2]]
            })
        );
        let message = warning.message();
        assert!(
            message.starts_with(
                "issue #8: the wind's direction is interpolated across 1500 to 1800 m above sea \
                 level, from 0.1 m/s at 0° to 10.0 m/s at 270° (and one more span)"
            ),
            "{message}"
        );
        assert!(
            message.contains("the drift and the apogee may read short or long"),
            "{message}"
        );
        assert!(!message.contains("Mach"), "{message}");
    }

    #[test]
    fn a_tables_turns_reach_the_warning_through_its_wrapper() {
        use hpr_atmos::{LayeredWind, Wind, WindInterpolation, WindLevel, WindModel};
        let levels = vec![
            WindLevel {
                height_msl_m: 1500.0,
                speed_m_s: 0.1,
                direction_from_rad: 0.0,
            },
            WindLevel {
                height_msl_m: 1800.0,
                speed_m_s: 10.0,
                direction_from_rad: 1.5 * std::f64::consts::PI,
            },
        ];
        let polar = LayeredWind::new(levels.clone(), WindInterpolation::SpeedDirection).unwrap();
        let wrapped = WindModel::Layered(polar.clone());
        assert_eq!(wind_numbers(&polar.direction_turns()), [8]);
        assert_eq!(wind_numbers(&wrapped.direction_turns()), [8]);
        // Interpolating the components turns no direction, so never warns.
        let components =
            WindModel::Layered(LayeredWind::new(levels, WindInterpolation::Components).unwrap());
        assert!(wind_numbers(&components.direction_turns()).is_empty());
        let constant = hpr_atmos::ConstantWind::new(0.1, 1.0).unwrap();
        assert!(wind_numbers(&constant.direction_turns()).is_empty());
    }

    #[test]
    fn the_unknown_signs_bear_on_the_flights_path_by_their_names() {
        // M10.1d6 (ADR-201): each of the four is of kind flight, by its number and its name in a
        // record; only #106, which starts at the join, quotes the top speed.
        for (issue, number, name, mach) in [
            (
                KnownIssue::OffAxisCenterOfGravity,
                219,
                "\"off_axis_center_of_gravity\"",
                false,
            ),
            (
                KnownIssue::SinglePodMoment,
                213,
                "\"single_pod_moment\"",
                false,
            ),
            (
                KnownIssue::BodyDampingStation,
                106,
                "\"body_damping_station\"",
                true,
            ),
            (
                KnownIssue::NearCalmWindDirection,
                8,
                "\"near_calm_wind_direction\"",
                false,
            ),
        ] {
            assert_eq!(issue.number(), number);
            assert_eq!(issue.kind(), IssueKind::Flight);
            assert!(!issue.is_stability());
            assert_eq!(issue.has_mach_condition(), mach, "{number}");
            assert_eq!(serde_json::to_string(&issue).unwrap(), name);
        }
        assert_eq!(KnownIssue::BaseDrag.kind(), IssueKind::Drag);
        assert_eq!(KnownIssue::MarginReadsHigh.kind(), IssueKind::Stability);
        assert_eq!(
            serde_json::to_string(&IssueKind::Flight).unwrap(),
            "\"flight\""
        );
        // A warning keeps its figures through a record, and one saved before them reads back
        // with none.
        let warning = off_axis([2.5e-4, 0.0]).remove(0);
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("\"detail\""), "{json}");
        assert_eq!(
            serde_json::from_str::<IssueWarning>(&json).unwrap(),
            warning
        );
        let mut old = warning.clone();
        old.detail = None;
        let old_json = serde_json::to_string(&old).unwrap();
        assert!(!old_json.contains("detail"), "{old_json}");
        assert_eq!(
            serde_json::from_str::<IssueWarning>(&old_json).unwrap(),
            old
        );
        assert_eq!(NEAR_CALM_WIND_M_S, 1.5);
    }
}
