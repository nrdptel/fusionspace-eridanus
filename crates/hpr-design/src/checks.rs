//! Structural design checks: typed findings about a design that resolves but can't be built or
//! flown as described.
//!
//! A finding is an [`Severity::Error`] when the design is physically impossible and a simulation of
//! it would be wrong, usually on the flattering side: a motor wider than its mount (Loft reported
//! +69% apogee for one), or fins whose root touches no part of the tube they belong to. A
//! [`Severity::Warning`] marks something unusual that can be real, such as a motor mount that
//! sticks out of the airframe. Callers that simulate should refuse designs with errors.
//!
//! Lengths are compared with [`LENGTH_TOLERANCE_M`] of slack, so round-off never raises a finding.
//!
//! See `docs/physics/design.md`.

use serde::{Deserialize, Serialize};

use crate::config::{Configuration, LaidOut};
use crate::error::DesignError;
use crate::solids::Wall;
use crate::tree::{LENGTH_TOLERANCE_M, Layout, Part, PlacedComponent, Rocket};

/// The tolerance of a three-place inch dimension on AeroTech's RMS dimensional drawings, m:
/// 0.005 in (0.127 mm), the `.XXX` entry of their title blocks.
pub const DRAWING_TOLERANCE_M: f64 = 0.005 * 0.0254;

/// ISO 2768-1:1989, table 1, tolerance class *c* (coarse): the permissible deviation of a linear
/// dimension with no tolerance of its own. Each entry is `(upper size, ± deviation)`, m, for sizes
/// above the previous entry's and up to and including this one's; the first band starts at
/// 0.5 mm, included, below which the standard sets none. The standard is for machined parts; HPR Sim borrows
/// its coarse class for hobby airframe parts, which no standard covers.
pub const COARSE_GENERAL_TOLERANCE_M: [(f64, f64); 8] = [
    (0.003, 0.0002),
    (0.006, 0.0003),
    (0.030, 0.0005),
    (0.120, 0.0008),
    (0.400, 0.0012),
    (1.000, 0.002),
    (2.000, 0.003),
    (4.000, 0.004),
];

/// How far an internal part may reach past the room in its parent and still be a fit, m: the
/// coarse general tolerance ([`COARSE_GENERAL_TOLERANCE_M`]) of the room's diameter. A part
/// drawn that much wider than a bore of that size, both made to the tolerance, can be made as a
/// fit: the bore at `+t` and the part at `−t` close a diametral interference of `2t`, a radial one
/// of `t`. 0.5 mm for a bore of 6 to 30 mm, 0.8 mm for 30 to 120 mm, 1.2 mm for 120 to 400 mm;
/// none below 0.5 mm, and 4 mm past 4 m.
pub fn fit_tolerance_m(room_m: f64) -> f64 {
    let diameter_m = 2.0 * room_m;
    // A room that isn't a number gets no tolerance.
    if diameter_m.is_nan() || diameter_m < 0.0005 {
        return 0.0;
    }
    COARSE_GENERAL_TOLERANCE_M
        .iter()
        .find(|(upto, _)| diameter_m <= *upto)
        .map_or(0.004, |(_, t)| *t)
}

/// The nominal motor sizes whose cases are narrower than their names, with the largest case
/// outside diameter, m: AeroTech's RMS dimensional drawings (RMS-18/20, RMS-24/40, RMS-29/40-120
/// and HP RMS-29/120, archived from aerotech-rocketry.com in 2005; inches, `.XXX` to
/// [`DRAWING_TOLERANCE_M`]) give the cases as 0.698, 0.938 and 1.125 in across, so at most 0.703,
/// 0.943 and 1.130 in. Each entry is `(nominal size, largest case)`. ThrustCurve.org and RASP
/// files carry the nominal size, so a "29 mm" motor is 29 mm in HPR Sim, and an AeroTech RMS-29 case
/// at most 28.70 mm in the hand; other makers' cases may differ. The same drawings give 38, 54,
/// 75 and 98 mm cases as 1.500, 2.125, 2.965 and 3.870 in, so at the +0.005 in limit each is at
/// least as wide as its name, and those sizes get no slack.
pub const NARROWER_THAN_NOMINAL_M: [(f64, f64); 3] = [
    (0.018, 0.698 * 0.0254 + DRAWING_TOLERANCE_M),
    (0.024, 0.938 * 0.0254 + DRAWING_TOLERANCE_M),
    (0.029, 1.125 * 0.0254 + DRAWING_TOLERANCE_M),
];

/// How much wider than its mount's bore a motor's diameter may be and only warn, m: for a nominal
/// size in [`NARROWER_THAN_NOMINAL_M`], how much wider the name is than the largest case
/// (0.144 mm at 18 mm, 0.048 mm at 24 mm, 0.298 mm at 29 mm, so a 29 mm motor fits a 1.140 in
/// bore); for any other diameter, none.
pub fn motor_fit_slack_m(diameter_m: f64) -> f64 {
    NARROWER_THAN_NOMINAL_M
        .iter()
        .find(|(nominal, _)| (diameter_m - nominal).abs() <= LENGTH_TOLERANCE_M)
        .map_or(0.0, |(nominal, case)| nominal - case)
}

/// How serious a finding is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Unusual, but it can be built and flown.
    Warning,
    /// Impossible as described; a simulation would be wrong.
    Error,
}

/// A problem found in a design. Serialized with a `kind` tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Finding {
    /// The motor case is wider than the mount's inside diameter by more than
    /// [`motor_fit_slack_m`] allows (error).
    MotorWiderThanMount {
        /// Configuration id.
        configuration: String,
        /// Mount id.
        mount: String,
        /// Motor case diameter, m.
        motor_diameter_m: f64,
        /// Mount inside diameter, m.
        mount_inner_diameter_m: f64,
    },
    /// The motor's nominal diameter is wider than the mount's inside diameter, but by no more than
    /// [`motor_fit_slack_m`] allows: the real case fits, such as a 29 mm motor in a 1.140 in
    /// (28.956 mm) bore (warning).
    MotorTightInMount {
        /// Configuration id.
        configuration: String,
        /// Mount id.
        mount: String,
        /// The motor's nominal diameter, m.
        motor_diameter_m: f64,
        /// Mount inside diameter, m.
        mount_inner_diameter_m: f64,
    },
    /// The motor case doesn't overlap its mount along the axis at all, such as an overhang typed
    /// in millimeters as meters (error).
    MotorOutsideMount {
        /// Configuration id.
        configuration: String,
        /// Mount id.
        mount: String,
    },
    /// The motor case reaches forward past the mount's forward end (warning).
    MotorPastMountTop {
        /// Configuration id.
        configuration: String,
        /// Mount id.
        mount: String,
        /// How far past, m.
        excess_m: f64,
    },
    /// An external part (fin root, tube fins, lug, rail button or pod set) doesn't overlap the body
    /// tube it is attached to at all (error). A pod set of no length touches its tube when it sits
    /// between the tube's ends, and a part on a pod's tube of no length when it spans the tube's
    /// station.
    AttachmentOffBody {
        /// Component id.
        component: String,
        /// Body tube id.
        body: String,
    },
    /// An external part runs past an end of its body tube (warning). Pods are exempt: a pod hangs
    /// from a pylon and often runs past its tube. So is a part on a pod's tube of no length, which
    /// has no length to run past.
    AttachmentPastBodyEnd {
        /// Component id.
        component: String,
        /// Body tube id.
        body: String,
        /// How far past, m.
        excess_m: f64,
    },
    /// An internal part lies wholly forward of the nose tip or aft of the rocket's end, and touches
    /// none of the parts it hangs from (a motor mount may stick out) (error).
    PartOutsideRocket {
        /// Component id.
        component: String,
    },
    /// An internal part runs past an end of its parent (warning).
    InternalPartPastParentEnd {
        /// Component id.
        component: String,
        /// Parent id.
        parent: String,
        /// How far past, m.
        excess_m: f64,
    },
    /// An internal part reaches farther from its parent's axis than the parent has room, a tube's
    /// bore or a nose cone's or transition's inside radius along the part (for a rigid part the
    /// most room along it, for a packed part the least), by more than
    /// [`fit_tolerance_m`] (error). A centering ring that wraps its parent inner tube, on the
    /// tube's axis with its bore at least the tube's outside, is measured against what holds the
    /// tube instead.
    InternalPartWiderThanParent {
        /// Component id.
        component: String,
        /// The id of the part whose room it needs: its parent, or what holds the tube it wraps.
        parent: String,
        /// How far the part reaches from the parent's axis, m.
        reach_m: f64,
        /// The room in the parent, m.
        room_m: f64,
    },
    /// An internal part reaches past the room in its parent by no more than [`fit_tolerance_m`]: a
    /// fit to sand. The mass where it crosses its parent's wall counts twice (warning). A ring
    /// wrapping its parent tube is measured as for [`Finding::InternalPartWiderThanParent`].
    InternalPartTightInParent {
        /// Component id.
        component: String,
        /// The id of the part whose room it needs: its parent, or what holds the tube it wraps.
        parent: String,
        /// How far the part reaches from the parent's axis, m.
        reach_m: f64,
        /// The room in the parent, m.
        room_m: f64,
    },
    /// An internal part other than a packed part fits inside a nose cone or transition where the
    /// profile is widest along the part, to [`fit_tolerance_m`], but reaches into the wall where
    /// the profile narrows, past that tolerance: as drawn it can't slide that far in (warning,
    /// #313). It flies where it is drawn, so its mass sits a little nearer the narrow end than it
    /// could.
    InternalPartWedgedInParent {
        /// Component id.
        component: String,
        /// Parent id.
        parent: String,
        /// How far the part reaches from the parent's axis, m.
        reach_m: f64,
        /// The room where the profile is narrowest along the part, m.
        room_m: f64,
        /// The room where the profile is widest along the part, m.
        widest_room_m: f64,
    },
    /// A packed part (a mass component, parachute, streamer or shock cord) reaches past the room
    /// in its parent, its center inside the room (warning). Its width is how it is packed, which
    /// sets only its own share of the rocket's inertia; its mass, length and station fly as drawn.
    PackedPartWiderThanParent {
        /// Component id.
        component: String,
        /// Parent id.
        parent: String,
        /// How far the part reaches from the parent's axis, m.
        reach_m: f64,
        /// The room in the parent, m.
        room_m: f64,
        /// Whether the parent has more room aft of the part than along it, as a nose cone or a
        /// shoulder does: the part's mass can sit only farther aft than drawn, so its center of
        /// gravity flies forward of where it can be and the stability margin may read high
        /// ([issue #367](https://github.com/nrdptel/fusionspace-eridanus/issues/367)). Read as `false` from a
        /// finding saved before it was kept.
        #[serde(default)]
        room_widens_aft: bool,
    },
    /// A centering ring or bulkhead wider than its parent's room, past [`fit_tolerance_m`], sits
    /// at one of its parent's end faces and fits what holds its parent, to that tolerance: a cap
    /// glued against the end, such as a bulkhead sized to the airframe on a coupler's end. The
    /// mass where it crosses its parent's wall counts twice (warning).
    RingAgainstParentEnd {
        /// The ring's id.
        ring: String,
        /// Parent id.
        parent: String,
        /// How far the ring reaches from the axis of what holds its parent, m.
        reach_m: f64,
        /// The room in what holds its parent, m.
        room_m: f64,
    },
    /// Two tubes of a cluster are closer than a tube's diameter, so they cross: the mass where they
    /// cross is counted twice and their motors would not fit (warning).
    ClusterTubesOverlap {
        /// Inner tube id.
        tube: String,
        /// The distance between the closest two tubes' axes, m.
        apart_m: f64,
        /// The tube's outer diameter, m.
        diameter_m: f64,
    },
    /// A centering ring overlaps an inner tube beside it, so the mass where they cross is counted
    /// twice; an automatic inner radius only clears on-axis tubes (warning).
    RingOverlapsInnerTube {
        /// The ring's id.
        ring: String,
        /// The tube's id.
        tube: String,
    },
    /// A stage with an axial center-of-mass override (`cg_aft_m`), its own or a component's, has
    /// its center forward of the nose tip or aft of the rocket's end although every internal part
    /// in it is on the rocket, such as a center typed in millimeters as meters (error). Written
    /// before the move to US spelling as `centre_outside_rocket`, which is still read and never written.
    #[serde(alias = "centre_outside_rocket")]
    CenterOutsideRocket {
        /// The stage's id.
        stage: String,
        /// The center's station, m.
        station_m: f64,
    },
    /// Adjacent body components' radii differ where they meet (warning).
    RadiusStep {
        /// The forward component's id.
        fore: String,
        /// The aft component's id.
        aft: String,
        /// The forward component's aft radius, m.
        fore_radius_m: f64,
        /// The aft component's forward radius, m.
        aft_radius_m: f64,
    },
    /// The first body component is not a nose cone (warning).
    NoNoseCone {
        /// The first component's id.
        component: String,
    },
}

impl Finding {
    /// The finding's severity.
    pub fn severity(&self) -> Severity {
        match self {
            Self::MotorWiderThanMount { .. }
            | Self::MotorOutsideMount { .. }
            | Self::AttachmentOffBody { .. }
            | Self::PartOutsideRocket { .. }
            | Self::InternalPartWiderThanParent { .. }
            | Self::CenterOutsideRocket { .. } => Severity::Error,
            Self::MotorTightInMount { .. }
            | Self::MotorPastMountTop { .. }
            | Self::AttachmentPastBodyEnd { .. }
            | Self::InternalPartPastParentEnd { .. }
            | Self::InternalPartTightInParent { .. }
            | Self::InternalPartWedgedInParent { .. }
            | Self::PackedPartWiderThanParent { .. }
            | Self::RingAgainstParentEnd { .. }
            | Self::ClusterTubesOverlap { .. }
            | Self::RingOverlapsInnerTube { .. }
            | Self::RadiusStep { .. }
            | Self::NoNoseCone { .. } => Severity::Warning,
        }
    }

    /// The finding as a sentence, without its severity: the parts by `name`, which is given a
    /// component's, stage's or configuration's id; sizes in millimeters.
    ///
    /// ```
    /// use hpr_design::checks::Finding;
    ///
    /// let finding = Finding::InternalPartPastParentEnd {
    ///     component: "c1".to_owned(),
    ///     parent: "c2".to_owned(),
    ///     excess_m: 0.0508,
    /// };
    /// let name = |id: &str| if id == "c1" { "Shock cord" } else { "Body tube" }.to_owned();
    /// assert_eq!(
    ///     finding.describe(&name),
    ///     "Shock cord runs 50.8 mm past an end of Body tube, which holds it"
    /// );
    /// ```
    pub fn describe(&self, name: &dyn Fn(&str) -> String) -> String {
        match self {
            Self::MotorWiderThanMount {
                configuration,
                mount,
                motor_diameter_m,
                mount_inner_diameter_m,
            } => {
                let (motor, bore) = mm_pair(*motor_diameter_m, *mount_inner_diameter_m);
                format!(
                    "the motor in {} is {motor} wide, wider than the mount's {bore} bore by more \
                     than a real case's slack (configuration {})",
                    name(mount),
                    name(configuration)
                )
            }
            Self::MotorTightInMount {
                configuration,
                mount,
                motor_diameter_m,
                mount_inner_diameter_m,
            } => {
                let (motor, bore) = mm_pair(*motor_diameter_m, *mount_inner_diameter_m);
                format!(
                    "the motor in {} is nominally {motor} wide in a {bore} bore: it fits, as real \
                     cases run under their nominal size (configuration {})",
                    name(mount),
                    name(configuration)
                )
            }
            Self::MotorOutsideMount {
                configuration,
                mount,
            } => format!(
                "the motor in {} lies wholly outside the mount along its length, such as an \
                 overhang typed in millimeters as meters (configuration {})",
                name(mount),
                name(configuration)
            ),
            Self::MotorPastMountTop {
                configuration,
                mount,
                excess_m,
            } => format!(
                "the motor in {} reaches {} past the mount's forward end (configuration {})",
                name(mount),
                mm(*excess_m),
                name(configuration)
            ),
            Self::AttachmentOffBody { component, body } => format!(
                "{} doesn't touch {}, the body tube it is attached to",
                name(component),
                name(body)
            ),
            Self::AttachmentPastBodyEnd {
                component,
                body,
                excess_m,
            } => format!(
                "{} runs {} past an end of {}, the body tube it is attached to",
                name(component),
                mm(*excess_m),
                name(body)
            ),
            Self::PartOutsideRocket { component } => format!(
                "{} lies wholly forward of the nose tip or aft of the rocket's end",
                name(component)
            ),
            Self::InternalPartPastParentEnd {
                component,
                parent,
                excess_m,
            } => format!(
                "{} runs {} past an end of {}, which holds it",
                name(component),
                mm(*excess_m),
                name(parent)
            ),
            Self::InternalPartWiderThanParent {
                component,
                parent,
                reach_m,
                room_m,
            } => {
                let (reach, room) = mm_pair(*reach_m, *room_m);
                format!(
                    "{} reaches {reach} from the axis of {}, which has {room} of room: too wide \
                     to fit, past the fit tolerance",
                    name(component),
                    name(parent)
                )
            }
            Self::InternalPartTightInParent {
                component,
                parent,
                reach_m,
                room_m,
            } => {
                let (reach, room) = mm_pair(*reach_m, *room_m);
                format!(
                    "{} reaches {reach} from the axis of {}, which has {room} of room: a fit to \
                     sand, and the mass where it crosses the wall counts twice",
                    name(component),
                    name(parent)
                )
            }
            Self::InternalPartWedgedInParent {
                component,
                parent,
                reach_m,
                room_m,
                widest_room_m,
            } => {
                let (reach, room) = mm_pair(*reach_m, *room_m);
                let (_, widest) = mm_pair(*reach_m, *widest_room_m);
                format!(
                    "{} reaches {reach} from the axis of {}, which narrows to {room} of room \
                     along it ({widest} where widest): it can't slide in that far as drawn, and \
                     flies where it is drawn",
                    name(component),
                    name(parent)
                )
            }
            Self::PackedPartWiderThanParent {
                component,
                parent,
                reach_m,
                room_m,
                room_widens_aft,
            } => {
                let (reach, room) = mm_pair(*reach_m, *room_m);
                let mut text = format!(
                    "{} is packed to reach {reach} from the axis of {}, which has {room} of \
                     room: it can't go in as drawn; its mass, length and station fly as drawn, and \
                     its width sets only its own inertia",
                    name(component),
                    name(parent)
                );
                if *room_widens_aft {
                    text.push_str(
                        ". The room widens aft of it, so its mass can sit only farther aft: the \
                         center of gravity flies forward of where it can be, and the stability \
                         margin may read high (issue #367, \
                         https://github.com/nrdptel/fusionspace-eridanus/issues/367)",
                    );
                }
                text
            }
            Self::RingAgainstParentEnd {
                ring,
                parent,
                reach_m,
                room_m,
            } => {
                let (reach, room) = mm_pair(*reach_m, *room_m);
                format!(
                    "{} reaches {reach} from the axis, wider than {}, and caps its end, in \
                     {room} of room around it; the mass where it crosses the wall counts twice",
                    name(ring),
                    name(parent)
                )
            }
            Self::ClusterTubesOverlap {
                tube,
                apart_m,
                diameter_m,
            } => {
                let (apart, diameter) = mm_pair(*apart_m, *diameter_m);
                format!(
                    "the cluster of {} has tubes {apart} apart, closer than their {diameter} \
                     diameter, so they cross: the mass where they cross counts twice and their \
                     motors would not fit",
                    name(tube)
                )
            }
            Self::RingOverlapsInnerTube { ring, tube } => format!(
                "{} overlaps {} beside it, so the mass where they cross counts twice",
                name(ring),
                name(tube)
            ),
            Self::CenterOutsideRocket { stage, station_m } => format!(
                "the center of mass set for {} is {} {} the nose tip, outside the rocket, such \
                 as a center typed in millimeters as meters",
                name(stage),
                mm(station_m.abs()),
                if *station_m < 0.0 {
                    "forward of"
                } else {
                    "aft of"
                }
            ),
            Self::RadiusStep {
                fore,
                aft,
                fore_radius_m,
                aft_radius_m,
            } => {
                let (fore_radius, aft_radius) = mm_pair(*fore_radius_m, *aft_radius_m);
                format!(
                    "{} ends {fore_radius} in radius where {} begins {aft_radius}: a step in the \
                     body's outline",
                    name(fore),
                    name(aft)
                )
            }
            Self::NoNoseCone { component } => format!(
                "the rocket's first body part, {}, is not a nose cone",
                name(component)
            ),
        }
    }
}

/// A length in millimeters: to a tenth, or under 1 mm to a hundredth, or under 0.1 mm to a
/// thousandth.
fn mm(length_m: f64) -> String {
    let mm = length_m * 1000.0;
    format!("{mm:.prec$} mm", prec = decimals(mm))
}

/// The decimals [`mm`] gives a size in millimeters.
fn decimals(mm: f64) -> usize {
    match mm.abs() {
        size if size >= 1.0 || size == 0.0 => 1,
        size if size >= 0.1 => 2,
        _ => 3,
    }
}

/// Two lengths a sentence compares, in millimeters, to as many decimals as tell them apart, up to
/// a ten-thousandth (0.1 µm): a 29 mm motor in a 28.956 mm bore is `29.00 mm` in a `28.96 mm`,
/// not `29.0 mm` in a `29.0 mm`.
fn mm_pair(a_m: f64, b_m: f64) -> (String, String) {
    let (a, b) = (a_m * 1000.0, b_m * 1000.0);
    // The finer of the two, so a small size beside a large one keeps its digits.
    let mut prec = decimals(a).max(decimals(b));
    while prec < 4 && format!("{a:.prec$}") == format!("{b:.prec$}") {
        prec += 1;
    }
    (format!("{a:.prec$} mm"), format!("{b:.prec$} mm"))
}

/// Every check on a design and all its configurations.
///
/// # Errors
///
/// As [`Rocket::layout`], [`Rocket::check_configuration_ids`] and [`Layout::place_motors`]: a
/// design that doesn't resolve can't be checked.
pub fn check(rocket: &Rocket) -> Result<Vec<Finding>, DesignError> {
    rocket.check_configuration_ids()?;
    check_on(rocket, &rocket.layout()?)
}

impl LaidOut {
    /// As [`check`], on this layout.
    ///
    /// # Errors
    ///
    /// As [`Layout::place_motors`].
    pub fn check(&self) -> Result<Vec<Finding>, DesignError> {
        check_on(self.rocket(), self.layout())
    }
}

/// Every check on `rocket`, whose layout is `layout`.
fn check_on(rocket: &Rocket, layout: &Layout) -> Result<Vec<Finding>, DesignError> {
    let mut findings = check_layout(layout);
    for configuration in &rocket.configurations {
        findings.extend(check_configuration(layout, configuration)?);
    }
    Ok(findings)
}

/// Whether any finding is an error.
pub fn has_errors(findings: &[Finding]) -> bool {
    findings.iter().any(|f| f.severity() == Severity::Error)
}

/// The overlap of `[a0, a1]` and `[b0, b1]`, m (negative when they are apart).
fn overlap(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    a1.min(b1) - a0.max(b0)
}

/// How far `[fore, aft]` runs past `[parent_fore, parent_aft]`, m, or `None` when the two don't
/// overlap at all.
fn excess(fore: f64, aft: f64, parent_fore: f64, parent_aft: f64) -> Option<f64> {
    if overlap(fore, aft, parent_fore, parent_aft) <= 0.0 {
        return None;
    }
    Some((parent_fore - fore).max(aft - parent_aft).max(0.0))
}

/// The checks on the structure alone.
///
/// The stage-center check reads the `center_overridden` flags that [`crate::Rocket::layout`] sets;
/// a layout built by hand without them never raises [`Finding::CenterOutsideRocket`].
pub fn check_layout(layout: &Layout) -> Vec<Finding> {
    let mut findings = Vec::new();
    let body: Vec<_> = layout.body().collect();
    if let Some(first) = body.first()
        && !matches!(first.part, Part::NoseCone(_))
    {
        findings.push(Finding::NoNoseCone {
            component: first.id.clone(),
        });
    }
    for pair in body.windows(2) {
        let (fore, aft) = (pair[0], pair[1]);
        if let (Some(a), Some(b)) = (fore.part.aft_radius_m(), aft.part.fore_radius_m())
            && (a - b).abs() > LENGTH_TOLERANCE_M
        {
            findings.push(Finding::RadiusStep {
                fore: fore.id.clone(),
                aft: aft.id.clone(),
                fore_radius_m: a,
                aft_radius_m: b,
            });
        }
    }
    // Internal parts wholly off the rocket and off every part they hang from, and the components
    // holding one.
    let length = layout.length_m;
    // A part that only touches a span at an end face is on it, within round-off.
    let apart = |c: &PlacedComponent, fore: f64, aft: f64| {
        overlap(c.fore_station_m, c.aft_station_m(), fore, aft) < -LENGTH_TOLERANCE_M
    };
    let off_rocket: Vec<bool> = layout
        .components
        .iter()
        .map(|c| {
            // A pod's body components are outside the airframe, like the pod set they hang from.
            if c.parent.is_none()
                || c.part.is_external()
                || c.part.is_body()
                || !apart(c, 0.0, length)
            {
                return false;
            }
            let mut ancestor = c.parent;
            for _ in 0..layout.components.len() {
                let Some(a) = ancestor.and_then(|i| layout.components.get(i)) else {
                    break;
                };
                if !apart(c, a.fore_station_m, a.aft_station_m()) {
                    return false;
                }
                ancestor = a.parent;
            }
            true
        })
        .collect();
    let mut holds_off = off_rocket.clone();
    for (index, component) in layout.components.iter().enumerate().rev() {
        if holds_off[index]
            && let Some(flag) = component.parent.and_then(|p| holds_off.get_mut(p))
        {
            *flag = true;
        }
    }
    for (k, stage) in layout.stages.iter().enumerate() {
        let station = -stage.mass.cg_m.z;
        let in_stage = || {
            layout
                .components
                .iter()
                .enumerate()
                .filter(|(_, c)| c.stage == k)
        };
        let holds = in_stage().any(|(i, _)| holds_off[i]);
        let overridden = stage.center_overridden || in_stage().any(|(_, c)| c.center_overridden);
        if overridden
            && !holds
            && stage.mass.mass_kg > 0.0
            && !(-LENGTH_TOLERANCE_M..=length + LENGTH_TOLERANCE_M).contains(&station)
        {
            findings.push(Finding::CenterOutsideRocket {
                stage: stage.id.clone(),
                station_m: station,
            });
        }
    }
    for (index, component) in layout.components.iter().enumerate() {
        let Some(parent) = component.parent.and_then(|i| layout.components.get(i)) else {
            continue;
        };
        attached_findings(component, parent, off_rocket[index], layout, &mut findings);
    }
    findings
}

/// Interior stations at which [`room_m`] samples a profile across a part's span, besides the
/// span's two ends.
const ROOM_SAMPLES: usize = 31;

/// The room for internal parts in a parent, m, along a part's span.
#[derive(Debug, Clone, Copy)]
struct Room {
    /// Where the parent is narrowest along the part.
    least_m: f64,
    /// Where the parent is widest along the part.
    most_m: f64,
}

/// The room for an internal part spanning stations `[fore, aft]` in `part`: a tube's bore, or a
/// nose cone's or transition's inside radius along the part (#313). That is the profile's outer
/// radius less its wall (none when filled, and none where the wall closes in at a tip), its least
/// and most over the part's span within the profile: the span's two ends and [`ROOM_SAMPLES`]
/// stations between them. Every profile HPR Sim draws is concave (its radius never dips between two
/// stations), so the least is at an end; the samples guard that claim. The wall is measured
/// normal to the surface, so outer less wall overstates the inside radius by `t (1/cos θ − 1)` on
/// a slope `θ`: 1% of the wall at 8°, 0.1 mm of a 3 mm wall at 15°. An automatic radius in a
/// profile is its inside radius at the part's narrower end ([ADR-096][adr-096]), so it fits by
/// this measure.
///
/// A part that runs past the profile's ends is measured over the part of it inside them (the
/// rest is [`Finding::InternalPartPastParentEnd`]). One wholly outside them, or with length that
/// only touches an end from outside, is measured against
/// the profile's largest outer radius, as before #313: where it sits is not in the profile, and
/// the past-the-end finding already names it. `None` for a part with no room for internal parts,
/// or a profile that can't be drawn.
///
/// [adr-096]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0096-fin-fillets-and-an-automatic-radius-inside-a.md
fn room_m(part: &PlacedComponent, fore: f64, aft: f64) -> Option<Room> {
    let both = |room_m: f64| Room {
        least_m: room_m,
        most_m: room_m,
    };
    if let Some(bore) = part.part.inner_radius_m() {
        return Some(both(bore));
    }
    let (profile, wall) = match &part.part {
        Part::NoseCone(nose) => (nose.profile().ok()?, nose.wall),
        Part::Transition(transition) => (transition.profile().ok()?, transition.wall),
        _ => return None,
    };
    let length = part.length_m;
    let (x0, x1) = (fore - part.fore_station_m, aft - part.fore_station_m);
    // Outside: no length of the part inside the profile. A part of no length at an end is inside,
    // at that end: at a nose's tip it has no room.
    let inside_m = x1.min(length) - x0.max(0.0);
    if x1 < -LENGTH_TOLERANCE_M
        || x0 > length + LENGTH_TOLERANCE_M
        || (x1 - x0 > LENGTH_TOLERANCE_M && inside_m <= LENGTH_TOLERANCE_M)
    {
        return Some(both(profile.max_radius_m()));
    }
    let wall_m = match wall {
        Wall::Filled {} => return Some(both(0.0)),
        Wall::Shell { thickness_m } => thickness_m,
    };
    let (x0, x1) = (x0.clamp(0.0, length), x1.clamp(0.0, length));
    let (least, most) = (0..=ROOM_SAMPLES + 1)
        .map(|i| {
            let x = x0 + (x1 - x0) * (i as f64) / ((ROOM_SAMPLES + 1) as f64);
            profile.radius_m(x)
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), r| {
            (lo.min(r), hi.max(r))
        });
    Some(Room {
        least_m: (least - wall_m).max(0.0),
        most_m: (most - wall_m).max(0.0),
    })
}

/// What surrounds a centering ring that wraps `tube` over the ring's span `[fore, aft]`: out from
/// the tube, the first part with room for parts whose span overlaps the ring's, if it holds the
/// whole ring; `None` when one holds only part of it or none overlaps it, and the ring is measured
/// in the tube.
fn wrapped_ring_container<'a>(
    fore: f64,
    aft: f64,
    tube: &PlacedComponent,
    layout: &'a Layout,
) -> Option<&'a PlacedComponent> {
    let mut next = tube.parent;
    while let Some(part) = next.and_then(|i| layout.components.get(i)) {
        if room_m(part, fore, aft).is_some()
            && let Some(e) = excess(fore, aft, part.fore_station_m, part.aft_station_m())
        {
            return (e <= LENGTH_TOLERANCE_M).then_some(part);
        }
        next = part.parent;
    }
    None
}

/// The checks on one attached part.
fn attached_findings(
    component: &PlacedComponent,
    parent: &PlacedComponent,
    off_rocket: bool,
    layout: &Layout,
    findings: &mut Vec<Finding>,
) {
    // A pod's body components stack along it, so they span the pod set exactly.
    if component.part.is_body() {
        return;
    }
    let (fore, aft) = (component.fore_station_m, component.aft_station_m());
    // Whether `[a0, a1]` holds the station `x`, to round-off.
    let holds =
        |a0: f64, a1: f64, x: f64| a0 - LENGTH_TOLERANCE_M <= x && x <= a1 + LENGTH_TOLERANCE_M;
    // Fins or a lug on a pod's body tube of no length hang off the airframe on that pod, as
    // OpenRocket draws winglets: they touch the tube if they span its station, and there is no
    // length for them to run past.
    if component.part.is_external() && parent.part.is_body() && parent.length_m == 0.0 {
        if !holds(fore, aft, parent.fore_station_m) {
            findings.push(Finding::AttachmentOffBody {
                component: component.id.clone(),
                body: parent.id.clone(),
            });
        }
        return;
    }
    let span = if component.length_m == 0.0 && matches!(component.part, Part::PodSet(_)) {
        // A pod set of no length (empty, or a pod of no length) is a point on its tube: it
        // touches the tube if it sits within the tube's ends.
        holds(parent.fore_station_m, parent.aft_station_m(), fore).then_some(0.0)
    } else {
        excess(fore, aft, parent.fore_station_m, parent.aft_station_m())
    };
    if component.part.is_external() {
        match span {
            None => findings.push(Finding::AttachmentOffBody {
                component: component.id.clone(),
                body: parent.id.clone(),
            }),
            // A pod is held by its pylon, not along its length: outboard pods often run past the
            // tube they hang from.
            Some(_) if matches!(component.part, Part::PodSet(_)) => {}
            Some(e) if e > LENGTH_TOLERANCE_M => {
                findings.push(Finding::AttachmentPastBodyEnd {
                    component: component.id.clone(),
                    body: parent.id.clone(),
                    excess_m: e,
                });
            }
            Some(_) => {}
        }
        return;
    }
    // An internal part entirely outside its parent runs past it by at least its own length.
    let e =
        span.unwrap_or_else(|| (parent.fore_station_m - fore).max(aft - parent.aft_station_m()));
    if off_rocket {
        findings.push(Finding::PartOutsideRocket {
            component: component.id.clone(),
        });
    } else if e > LENGTH_TOLERANCE_M {
        findings.push(Finding::InternalPartPastParentEnd {
            component: component.id.clone(),
            parent: parent.id.clone(),
            excess_m: e,
        });
    }
    let holder = parent.parent.and_then(|i| layout.components.get(i));
    // A ring shares its parent's axis when the parent is neither a cluster nor off the axis: a
    // cluster's tubes or an off-axis tube sit beside the ring, not around it.
    let concentric = parent.part.axis_offset_m() == component.part.axis_offset_m()
        && !matches!(&parent.part, Part::InnerTube(tube) if !tube.cluster_m.is_empty());
    // A centering ring whose bore takes its parent tube's outside wraps that tube, as OpenRocket's
    // pods examples draw the rings on their motor tubes: it sits in whatever surrounds the tube at
    // the ring's own station, so that is where it needs room.
    let wraps = concentric
        && matches!((&component.part, &parent.part), (Part::CenteringRing(ring), Part::InnerTube(tube))
            if ring.inner_radius_m >= tube.outer_radius_m - LENGTH_TOLERANCE_M);
    let container = if wraps {
        wrapped_ring_container(fore, aft, parent, layout).unwrap_or(parent)
    } else {
        parent
    };
    if let (Some(reach_m), Some(room)) = (
        component.part.reach_from_m(container.part.axis_offset_m()),
        room_m(container, fore, aft),
    ) && reach_m > room.least_m + LENGTH_TOLERANCE_M
    {
        let Room { least_m, most_m } = room;
        let fits = |room_m: f64| reach_m <= room_m + fit_tolerance_m(room_m) + LENGTH_TOLERANCE_M;
        // A ring at its parent's end face can sit against it, so it needs room in what holds its
        // parent instead. It covers one end face, not both, and shares its parent's axis.
        // A ring that wraps its tube is around it, not over its end face, so it is never a cap.
        let at_end = matches!(component.part, Part::CenteringRing(_))
            && concentric
            && !wraps
            && holds(fore, aft, parent.fore_station_m) != holds(fore, aft, parent.aft_station_m());
        let cap = holder.filter(|_| at_end).and_then(|holder| {
            let reach_m = component.part.reach_from_m(holder.part.axis_offset_m())?;
            let room_m = room_m(holder, fore, aft)?.most_m;
            (reach_m <= room_m + fit_tolerance_m(room_m) + LENGTH_TOLERANCE_M)
                .then_some((reach_m, room_m))
        });
        // A packed part's width is how it is packed: it sets only the part's own share of the
        // rocket's inertia, so a packed part drawn too wide still flies as drawn while its
        // center is in the room. It has no fit tolerance, and centered past the room it is an
        // error (ADR-170). In a profile the room is the narrowest along the part.
        let packed_center_inside = component
            .part
            .packing()
            .map(|packing| reach_m - packing.radius_m <= least_m + LENGTH_TOLERANCE_M);
        let rigid = packed_center_inside.is_none();
        findings.push(if packed_center_inside == Some(true) {
            // Room anywhere aft of the part, to the container's end, that is more than the least
            // along it: the part could fit only farther aft (#367). A NaN room counts.
            // A part that runs past the container's aft end is measured at that end.
            let end = container.aft_station_m();
            let room_widens_aft = room_m(container, aft.min(end), end).is_none_or(|aft_room| {
                let most_m = aft_room.most_m;
                most_m.is_nan() || least_m.is_nan() || most_m > least_m + LENGTH_TOLERANCE_M
            });
            Finding::PackedPartWiderThanParent {
                component: component.id.clone(),
                parent: container.id.clone(),
                reach_m,
                room_m: least_m,
                room_widens_aft,
            }
        } else if rigid && fits(least_m) {
            Finding::InternalPartTightInParent {
                component: component.id.clone(),
                parent: container.id.clone(),
                reach_m,
                room_m: least_m,
            }
        } else if rigid && fits(most_m) {
            // In a nose cone or transition: it fits where the profile is widest along it, and
            // runs into the wall where the profile narrows (#313).
            Finding::InternalPartWedgedInParent {
                component: component.id.clone(),
                parent: container.id.clone(),
                reach_m,
                room_m: least_m,
                widest_room_m: most_m,
            }
        } else if let Some((reach_m, room_m)) = cap {
            Finding::RingAgainstParentEnd {
                ring: component.id.clone(),
                parent: parent.id.clone(),
                reach_m,
                room_m,
            }
        } else {
            Finding::InternalPartWiderThanParent {
                component: component.id.clone(),
                parent: container.id.clone(),
                reach_m,
                room_m: if rigid { most_m } else { least_m },
            }
        });
    }
    if let Part::InnerTube(inner) = &component.part {
        let apart_m = inner
            .cluster_m
            .iter()
            .enumerate()
            .flat_map(|(i, &[x, y])| {
                inner.cluster_m[i + 1..]
                    .iter()
                    .map(move |&[u, v]| (x - u).hypot(y - v))
            })
            .fold(f64::INFINITY, f64::min);
        let diameter_m = 2.0 * inner.outer_radius_m;
        if apart_m < diameter_m - LENGTH_TOLERANCE_M {
            findings.push(Finding::ClusterTubesOverlap {
                tube: component.id.clone(),
                apart_m,
                diameter_m,
            });
        }
    }
    if let Part::CenteringRing(ring) = &component.part {
        // A ring that wraps its tube sits beside the tubes in what surrounds it too.
        let around = (container.id != parent.id)
            .then(|| layout.find(&container.id))
            .flatten()
            .map(|(i, _)| i);
        for tube in layout.components.iter().filter(|c| {
            (c.parent == component.parent || around.is_some_and(|i| c.parent == Some(i)))
                && c.id != component.id
                && c.id != parent.id
        }) {
            let Part::InnerTube(inner) = &tube.part else {
                continue;
            };
            // Each tube (every tube of a cluster) covers radii [d − R, d + R] about the ring's
            // (the body) axis.
            let [x, y] = tube.part.axis_offset_m();
            let tubes = if inner.cluster_m.is_empty() {
                &[[0.0, 0.0]][..]
            } else {
                inner.cluster_m.as_slice()
            };
            let radial = tubes
                .iter()
                .map(|&[u, v]| {
                    let d = (x + u).hypot(y + v);
                    (d + inner.outer_radius_m).min(ring.outer_radius_m)
                        - (d - inner.outer_radius_m).max(ring.inner_radius_m)
                })
                .fold(f64::NEG_INFINITY, f64::max);
            if overlap(fore, aft, tube.fore_station_m, tube.aft_station_m()) > LENGTH_TOLERANCE_M
                && radial > LENGTH_TOLERANCE_M
            {
                findings.push(Finding::RingOverlapsInnerTube {
                    ring: component.id.clone(),
                    tube: tube.id.clone(),
                });
            }
        }
    }
}

/// The checks on one configuration's motors in `layout`. The configuration need not be one of the
/// rocket's own, so a candidate motor can be checked before it is stored.
///
/// # Errors
///
/// As [`Layout::place_motors`].
pub fn check_configuration(
    layout: &Layout,
    configuration: &Configuration,
) -> Result<Vec<Finding>, DesignError> {
    let mut findings = Vec::new();
    // Every tube of a cluster holds the same motor the same way along the axis, so its first tube
    // speaks for the mount and each finding is made once.
    let placed = layout.place_motors(configuration)?;
    for motor in placed.into_iter().filter(|motor| motor.tube == 0) {
        let Some((_, mount)) = layout.find(&motor.mount) else {
            continue;
        };
        if let Some(inner) = mount.part.inner_radius_m() {
            let inner_diameter = 2.0 * inner;
            let motor_diameter_m = motor.mounted.diameter_m;
            let slack_m = motor_fit_slack_m(motor_diameter_m);
            if motor_diameter_m > inner_diameter + slack_m + LENGTH_TOLERANCE_M {
                findings.push(Finding::MotorWiderThanMount {
                    configuration: configuration.id.clone(),
                    mount: mount.id.clone(),
                    motor_diameter_m,
                    mount_inner_diameter_m: inner_diameter,
                });
            } else if motor_diameter_m > inner_diameter + LENGTH_TOLERANCE_M {
                findings.push(Finding::MotorTightInMount {
                    configuration: configuration.id.clone(),
                    mount: mount.id.clone(),
                    motor_diameter_m,
                    mount_inner_diameter_m: inner_diameter,
                });
            }
        }
        let (fore, nozzle) = (motor.fore_station_m(), motor.nozzle_station_m());
        if overlap(fore, nozzle, mount.fore_station_m, mount.aft_station_m()) <= 0.0 {
            findings.push(Finding::MotorOutsideMount {
                configuration: configuration.id.clone(),
                mount: mount.id.clone(),
            });
            continue;
        }
        let past = mount.fore_station_m - fore;
        if past > LENGTH_TOLERANCE_M {
            findings.push(Finding::MotorPastMountTop {
                configuration: configuration.id.clone(),
                mount: mount.id.clone(),
                excess_m: past,
            });
        }
    }
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{
        attached, body, bottom, inner_tube, mass_component, ring, rocket, stage, three_fin_rocket,
        top, tube,
    };
    use crate::{AutoDimension, MotorMount};
    use crate::{Part, Position};

    /// Every finding's sentence names its parts by `name`, never by id, and gives its sizes in
    /// millimeters.
    #[test]
    fn describe_names_parts_and_sizes_in_millimeters() {
        let id = |s: &str| s.to_owned();
        let findings = [
            Finding::MotorWiderThanMount {
                configuration: id("cfg"),
                mount: id("mt"),
                motor_diameter_m: 0.038,
                mount_inner_diameter_m: 0.029,
            },
            Finding::MotorTightInMount {
                configuration: id("cfg"),
                mount: id("mt"),
                motor_diameter_m: 0.029,
                mount_inner_diameter_m: 0.028_956,
            },
            Finding::MotorOutsideMount {
                configuration: id("cfg"),
                mount: id("mt"),
            },
            Finding::MotorPastMountTop {
                configuration: id("cfg"),
                mount: id("mt"),
                excess_m: 0.012,
            },
            Finding::AttachmentOffBody {
                component: id("fin"),
                body: id("bt"),
            },
            Finding::AttachmentPastBodyEnd {
                component: id("fin"),
                body: id("bt"),
                excess_m: 0.0004,
            },
            Finding::PartOutsideRocket {
                component: id("mass"),
            },
            Finding::InternalPartPastParentEnd {
                component: id("cord"),
                parent: id("bt"),
                excess_m: 0.050_800_000_000_000_01,
            },
            Finding::InternalPartWiderThanParent {
                component: id("ring"),
                parent: id("bt"),
                reach_m: 0.04,
                room_m: 0.035,
            },
            Finding::InternalPartTightInParent {
                component: id("ring"),
                parent: id("bt"),
                reach_m: 0.035_05,
                room_m: 0.035,
            },
            Finding::PackedPartWiderThanParent {
                component: id("payload"),
                parent: id("bt"),
                reach_m: 0.0125,
                room_m: 0.0105,
                room_widens_aft: false,
            },
            Finding::RingAgainstParentEnd {
                ring: id("ring"),
                parent: id("cp"),
                reach_m: 0.04,
                room_m: 0.0399,
            },
            Finding::ClusterTubesOverlap {
                tube: id("it"),
                apart_m: 0.02,
                diameter_m: 0.025,
            },
            Finding::RingOverlapsInnerTube {
                ring: id("ring"),
                tube: id("it"),
            },
            Finding::CenterOutsideRocket {
                stage: id("st"),
                station_m: -0.25,
            },
            Finding::RadiusStep {
                fore: id("nose"),
                aft: id("bt"),
                fore_radius_m: 0.0205,
                aft_radius_m: 0.0207,
            },
            Finding::NoNoseCone {
                component: id("bt"),
            },
            Finding::InternalPartWedgedInParent {
                component: id("coupler"),
                parent: id("nose"),
                reach_m: 0.0245,
                room_m: 0.0115,
                widest_room_m: 0.025,
            },
        ];
        let name = |id: &str| format!("<{id}>");
        let sentences: Vec<String> = findings.iter().map(|f| f.describe(&name)).collect();
        for (finding, sentence) in findings.iter().zip(&sentences) {
            // Every id the finding carries is named, as `name` gives it.
            let value = serde_json::to_value(finding).unwrap();
            for (key, field) in value.as_object().unwrap() {
                if let Some(text) = field.as_str().filter(|_| key != "kind") {
                    assert!(sentence.contains(&format!("<{text}>")), "{key}: {sentence}");
                }
            }
            assert!(
                !sentence.contains('{') && !sentence.contains('_'),
                "{sentence}"
            );
        }
        assert_eq!(
            sentences[0],
            "the motor in <mt> is 38.0 mm wide, wider than the mount's 29.0 mm bore by more \
             than a real case's slack (configuration <cfg>)"
        );
        assert!(sentences[5].contains(" 0.40 mm past "), "{}", sentences[5]);
        assert!(sentences[7].contains(" 50.8 mm past "), "{}", sentences[7]);
        assert!(sentences[14].contains(" 250.0 mm forward of the nose tip"));
        assert_eq!(mm(0.000_05), "0.050 mm");
        // Two sizes a sentence compares never print alike.
        assert!(
            sentences[1].contains(" is nominally 29.00 mm wide in a 28.96 mm bore"),
            "{}",
            sentences[1]
        );
        assert!(
            sentences[9].contains(" reaches 35.05 mm from the axis of <bt>, which has 35.00 mm")
        );
        assert!(sentences[10].contains(
            "<payload> is packed to reach 12.5 mm from the axis of <bt>, which has 10.5 mm of room"
        ));
        assert!(
            sentences[17].contains(
                "<coupler> reaches 24.5 mm from the axis of <nose>, which narrows to 11.5 mm of \
                 room along it (25.0 mm where widest)"
            ),
            "{}",
            sentences[17]
        );
        assert!(sentences[11].contains(
            " reaches 40.0 mm from the axis, wider than <cp>, and caps its end, in 39.9 mm"
        ));
        assert!(sentences[15].contains(" ends 20.5 mm in radius where <bt> begins 20.7 mm"));
        assert_eq!(
            mm_pair(0.038, 0.037_97),
            ("38.00 mm".to_owned(), "37.97 mm".to_owned())
        );
        assert_eq!(
            mm_pair(0.040, 0.000_049),
            ("40.000 mm".to_owned(), "0.049 mm".to_owned())
        );
        assert_eq!(
            mm_pair(0.025, 0.024_98),
            ("25.00 mm".to_owned(), "24.98 mm".to_owned())
        );
        assert_eq!(
            mm_pair(0.020_5, 0.020_500_01),
            ("20.5000 mm".to_owned(), "20.5000 mm".to_owned())
        );
        assert_eq!(mm(0.0005), "0.50 mm");
        assert_eq!(mm(0.0), "0.0 mm");
    }

    fn kinds(findings: &[Finding]) -> Vec<(String, Severity)> {
        findings
            .iter()
            .map(|f| {
                let tag = serde_json::to_value(f).unwrap()["kind"]
                    .as_str()
                    .unwrap()
                    .to_owned();
                (tag, f.severity())
            })
            .collect()
    }

    #[test]
    fn sample_rocket_passes_every_check() {
        let findings = check(&three_fin_rocket()).unwrap();
        assert!(findings.is_empty(), "{findings:?}");
    }

    /// Loft lesson L50: a 54 mm motor in a 38 mm mount flew, and flew high. Here it is an error, a
    /// motor exactly as wide as the bore is not, and a case longer than the mount only warns.
    #[test]
    fn motor_wider_than_mount_is_rejected() {
        let mut design = three_fin_rocket();
        design.configurations[0].motors[0] = crate::testing::motor("mmt", 0.054, 0.2);
        let findings = check(&design).unwrap();
        assert_eq!(
            findings,
            vec![Finding::MotorWiderThanMount {
                configuration: "main".to_owned(),
                mount: "mmt".to_owned(),
                motor_diameter_m: 0.054,
                mount_inner_diameter_m: 2.0 * (0.020 - 0.001),
            }]
        );
        assert_eq!(findings[0].severity(), Severity::Error);
        assert!(has_errors(&findings));
        // A cluster of three such tubes is still one finding: the motor is named once.
        if let Part::InnerTube(tube) = &mut design.stages[0].components[1].children[0].part {
            tube.cluster_m = vec![[0.0, 0.0], [0.04, 0.0], [-0.04, 0.0]];
        }
        let findings = check(&design).unwrap();
        let wider = findings
            .iter()
            .filter(|f| matches!(f, Finding::MotorWiderThanMount { .. }))
            .count();
        assert_eq!(wider, 1, "{findings:?}");

        // The sample's 38 mm motor fills its 38 mm bore exactly: no finding.
        let exact = three_fin_rocket();
        assert!(check(&exact).unwrap().is_empty());
        // Wider by more than round-off is an error again.
        let mut design = three_fin_rocket();
        design.configurations[0].motors[0] = crate::testing::motor("mmt", 0.038 + 1e-6, 0.2);
        assert!(has_errors(&check(&design).unwrap()));

        // A 0.35 m case in a 0.3 m mount with 0.01 m of overhang reaches 0.04 m past its top.
        let mut design = three_fin_rocket();
        design.configurations[0].motors[0] = crate::testing::motor("mmt", 0.038, 0.35);
        let findings = check(&design).unwrap();
        let [Finding::MotorPastMountTop { excess_m, .. }] = findings[..] else {
            panic!("{findings:?}")
        };
        assert!((excess_m - 0.04).abs() < 1e-12);
        assert!(!has_errors(&findings));
    }

    /// Issue #280: a nominal 29 mm motor in LOC's 1.140 in (28.956 mm) tube only warns, as
    /// AeroTech's 29 mm case is at most 1.130 in (28.702 mm) across; a hair past that, and the
    /// Loft demo's 28.0 mm bore, are errors. Sizes whose cases are as wide as their names get no
    /// slack, and neither does a diameter that isn't a nominal size.
    #[test]
    fn a_nominal_motor_in_its_matching_tube_only_warns() {
        let fitted = |motor_m: f64, bore_m: f64| {
            let mut design = three_fin_rocket();
            let Part::InnerTube(tube) = &mut design.stages[0].components[1].children[0].part else {
                panic!("the tests' rocket's mount is an inner tube");
            };
            tube.outer_radius_m = bore_m / 2.0 + tube.thickness_m;
            design.configurations[0].motors[0] = crate::testing::motor("mmt", motor_m, 0.2);
            check(&design).unwrap()
        };
        let findings = fitted(0.029, 0.028956);
        let [
            Finding::MotorTightInMount {
                motor_diameter_m,
                mount_inner_diameter_m,
                ..
            },
        ] = findings[..]
        else {
            panic!("{findings:?}")
        };
        assert_eq!(motor_diameter_m, 0.029);
        assert!((mount_inner_diameter_m - 0.028956).abs() < 1e-12);
        assert_eq!(findings[0].severity(), Severity::Warning);
        assert!(!has_errors(&findings));
        // The slack is the name less the largest case: 0.298, 0.0478 and 0.1438 mm.
        assert!((motor_fit_slack_m(0.029) - 0.000_298).abs() < 1e-12);
        assert!((motor_fit_slack_m(0.024) - 0.000_047_8).abs() < 1e-12);
        assert!((motor_fit_slack_m(0.018) - 0.000_143_8).abs() < 1e-12);
        assert_eq!(motor_fit_slack_m(0.038), 0.0);
        assert!(!has_errors(&fitted(0.029, 0.028702 + 1e-7)));
        for (motor_m, bore_m) in [
            (0.029, 0.028702 - 1e-6),
            (0.029, 0.028),
            (0.024, 0.023952 - 1e-6),
            (0.038, 0.038 - 1e-6),
            (0.054, 0.054 - 1e-6),
            // Not a nominal size: compared as it is.
            (0.0289, 0.0289 - 1e-6),
        ] {
            // A 54 mm mount is wider than the tests' airframe, so only the motor's findings count.
            let findings: Vec<_> = fitted(motor_m, bore_m)
                .into_iter()
                .filter(|f| {
                    matches!(
                        f,
                        Finding::MotorWiderThanMount { .. } | Finding::MotorTightInMount { .. }
                    )
                })
                .collect();
            assert!(
                matches!(&findings[..], [Finding::MotorWiderThanMount { .. }]),
                "{motor_m} in {bore_m}: {findings:?}"
            );
        }
    }

    /// A pod of no length (fins or a lug on a pod's tube of length 0, as OpenRocket draws
    /// winglets) and an empty pod set raise no finding where they sit on their tube, though they
    /// have no length to overlap it with; a pod set of no length beyond the tube's end is still
    /// off it (M1.13b2).
    #[test]
    fn a_pod_set_of_no_length_touches_its_tube_where_it_sits() {
        let podded = |children: Vec<crate::Component>, position: Position| {
            let mut design = three_fin_rocket();
            let mut pods = attached(
                "pods",
                Part::PodSet(crate::parts::PodSet {
                    count: 2,
                    radial_offset_m: 0.06,
                    angle_rad: 0.0,
                }),
                position,
            );
            pods.children = children;
            design.stages[0].components[1].children.push(pods);
            check(&design).unwrap()
        };
        let mut phantom = body("phantom", tube(0.0, 0.0, 0.0));
        phantom.children = vec![attached(
            "wings",
            crate::testing::fins(0.05, 0.03),
            bottom(0.0),
        )];
        let base = check(&three_fin_rocket()).unwrap();
        assert_eq!(podded(vec![phantom.clone()], top(0.1)), base);
        assert_eq!(podded(Vec::new(), top(0.1)), base);
        // At the tube's aft end, and just past it.
        assert_eq!(podded(Vec::new(), bottom(0.0)), base);
        let off = podded(vec![phantom], bottom(0.01));
        assert!(
            off.contains(&Finding::AttachmentOffBody {
                component: "pods".to_owned(),
                body: "airframe".to_owned(),
            }),
            "{off:?}"
        );
        assert_eq!(off.len(), base.len() + 1, "{off:?}");

        // Fins that don't reach their tube of no length are off it, as any fins are.
        let mut far = body("phantom", tube(0.0, 0.0, 0.0));
        far.children = vec![attached(
            "wings",
            crate::testing::fins(0.05, 0.03),
            top(0.01),
        )];
        let off = podded(vec![far], top(0.1));
        assert_eq!(
            off.iter()
                .filter(
                    |f| matches!(f, Finding::AttachmentOffBody { component, body }
                    if component == "wings" && body == "phantom")
                )
                .count(),
            1,
            "{off:?}"
        );
        assert_eq!(off.len(), base.len() + 1, "{off:?}");
    }

    /// Loft lesson L50: fins could sit off the airframe. A fin root that touches none of its body
    /// tube is an error; one that runs past the tube's end warns.
    #[test]
    fn fin_root_must_touch_body() {
        let with_fins = |offset: f64| {
            let mut design = three_fin_rocket();
            design.stages[0].components[1].children[3].position = Some(bottom(offset));
            check(&design).unwrap()
        };
        // Root chord 0.1 m; the tube ends at station 1.0.
        let off = with_fins(0.15);
        assert_eq!(
            off,
            vec![Finding::AttachmentOffBody {
                component: "fins".to_owned(),
                body: "airframe".to_owned(),
            }]
        );
        assert!(has_errors(&off));
        // Touching only at the tube's end is still off the body.
        assert!(has_errors(&with_fins(0.1)));
        let past = with_fins(0.04);
        let [
            Finding::AttachmentPastBodyEnd {
                ref component,
                excess_m,
                ..
            },
        ] = past[..]
        else {
            panic!("{past:?}")
        };
        assert_eq!(component, "fins");
        assert!((excess_m - 0.04).abs() < 1e-12);
        assert!(!has_errors(&past));
        assert!(with_fins(0.0).is_empty());
        // Forward of the tube too.
        let mut design = three_fin_rocket();
        design.stages[0].components[1].children[3].position = Some(top(-0.2));
        assert!(has_errors(&check(&design).unwrap()));
    }

    #[test]
    fn internal_parts_radius_steps_and_a_missing_nose_are_found() {
        let mut airframe = body("airframe", tube(0.5, 0.03, 0.001));
        airframe.children = vec![
            // 1 mm wider than the 29 mm bore, 0.5 mm past its fit tolerance.
            attached("wide-ring", ring(0.005, 0.03, 0.01), top(0.1)),
            // Hanging 0.05 m out of the tube's aft end.
            attached("long-mmt", inner_tube(0.2, 0.01, 0.001), bottom(0.05)),
            // Entirely forward of the tube.
            attached("lost", mass_component(0.1, 0.05, 0.01), top(-0.2)),
            // 0.5 mm wider than the bore: a tight fit.
            attached("tight-ring", ring(0.005, 0.0295, 0.01), top(0.2)),
        ];
        let mut mount = airframe.children[1].clone();
        mount.motor_mount = Some(MotorMount::default());
        airframe.children[1] = mount;
        let design = rocket(vec![stage(
            "s",
            vec![airframe, body("tail", tube(0.1, 0.02, 0.001))],
        )]);
        let findings = check(&design).unwrap();
        assert_eq!(
            kinds(&findings),
            vec![
                ("no_nose_cone".to_owned(), Severity::Warning),
                ("radius_step".to_owned(), Severity::Warning),
                (
                    "internal_part_wider_than_parent".to_owned(),
                    Severity::Error
                ),
                (
                    "internal_part_past_parent_end".to_owned(),
                    Severity::Warning
                ),
                // Wholly forward of the nose tip: one error, not also a warning.
                ("part_outside_rocket".to_owned(), Severity::Error),
                (
                    "internal_part_tight_in_parent".to_owned(),
                    Severity::Warning
                ),
            ]
        );

        // A ring left solid across the motor mount counts their mass twice.
        let mut design = three_fin_rocket();
        design.stages[0].components[1].children[1].auto = vec![AutoDimension::OuterRadius];
        assert_eq!(
            check(&design).unwrap(),
            vec![Finding::RingOverlapsInnerTube {
                ring: "ring-fore".to_owned(),
                tube: "mmt".to_owned(),
            }]
        );
    }

    /// Radial room is measured about the parent's own axis: a block centered in an off-axis pod
    /// fits, and a mass on the body axis attached to the pod doesn't. Rings in a cluster warn.
    #[test]
    fn internal_parts_are_measured_from_their_parents_axis() {
        let offset = |part: Part, r: f64| match part {
            Part::InnerTube(mut t) => {
                t.radial_offset_m = r;
                Part::InnerTube(t)
            }
            Part::MassComponent(mut m) => {
                m.packing.radial_offset_m = r;
                Part::MassComponent(m)
            }
            other => other,
        };
        let mut pod = attached(
            "pod",
            offset(inner_tube(0.3, 0.02, 0.001), 0.025),
            bottom(0.0),
        );
        pod.children = vec![attached(
            "block",
            offset(inner_tube(0.01, 0.019, 0.003), 0.025),
            top(0.0),
        )];
        let mut airframe = body("airframe", tube(0.8, 0.05, 0.002));
        airframe.children = vec![pod, attached("ring", ring(0.005, 0.048, 0.0), bottom(-0.1))];
        let design = rocket(vec![stage(
            "s",
            vec![body("nose", crate::testing::nose(0.2, 0.05)), airframe],
        )]);
        assert_eq!(
            check(&design).unwrap(),
            vec![Finding::RingOverlapsInnerTube {
                ring: "ring".to_owned(),
                tube: "pod".to_owned(),
            }]
        );

        let mut design = design;
        design.stages[0].components[1].children[0].children[0] =
            attached("on-axis", mass_component(0.1, 0.05, 0.004), top(0.1));
        let findings = check(&design).unwrap();
        let wide = findings
            .iter()
            .find_map(|f| match f {
                Finding::InternalPartWiderThanParent {
                    component,
                    reach_m,
                    room_m,
                    ..
                } if component == "on-axis" => Some((*reach_m, *room_m)),
                _ => None,
            })
            .unwrap();
        assert!(
            (wide.0 - 0.029).abs() < 1e-15 && (wide.1 - 0.019).abs() < 1e-15,
            "{wide:?}"
        );
    }

    /// A packed part drawn wider than its parent's bore warns, by any amount, while its center is
    /// in the bore: OpenRocket's *Deployable payload* packs a 25 mm payload into a 21 mm bore. Its
    /// width sets only its own inertia. A ring as wide is an error, and so is a packed part whose
    /// center is past the bore (ADR-170).
    #[test]
    fn a_packed_part_wider_than_its_bore_warns_while_its_center_is_inside() {
        // The airframe's bore is 0.0255 m; its fit tolerance (51 mm) is 0.8 mm.
        let room = 0.0255;
        let placed = |part: Part, offset_m: f64| {
            let part = match part {
                Part::MassComponent(mut m) => {
                    m.packing.radial_offset_m = offset_m;
                    Part::MassComponent(m)
                }
                Part::Parachute(mut p) => {
                    p.packing.radial_offset_m = offset_m;
                    Part::Parachute(p)
                }
                other => other,
            };
            let mut design = three_fin_rocket();
            design.stages[0].components[1]
                .children
                .push(attached("packed", part, top(0.2)));
            check(&design).unwrap()
        };
        let packed = |reach_m: f64| {
            vec![Finding::PackedPartWiderThanParent {
                component: "packed".to_owned(),
                parent: "airframe".to_owned(),
                reach_m,
                room_m: room,
                // A tube's bore is as wide aft of it: the part's center of gravity is no less
                // possible where it is drawn (#367).
                room_widens_aft: false,
            }]
        };
        // 5 mm too wide, or within the fit tolerance: a warning, not a fit to sand.
        for radius_m in [0.0305, 0.026] {
            assert_eq!(
                placed(mass_component(0.1, 0.05, radius_m), 0.0),
                packed(radius_m)
            );
        }
        let mut chute = three_fin_rocket().stages[0].components[1].children[4]
            .part
            .clone();
        if let Part::Parachute(p) = &mut chute {
            p.packing.radius_m = 0.03;
        }
        assert_eq!(placed(chute, 0.0), packed(0.03));
        // Its center on the bore's edge still warns; 0.1 µm past it, the part is an error.
        let reach = room + 0.002;
        assert_eq!(
            placed(mass_component(0.1, 0.05, 0.002), room),
            packed(reach)
        );
        assert_eq!(
            placed(mass_component(0.1, 0.05, 0.002), room + 1e-7),
            vec![Finding::InternalPartWiderThanParent {
                component: "packed".to_owned(),
                parent: "airframe".to_owned(),
                reach_m: room + 1e-7 + 0.002,
                room_m: room,
            }]
        );
        // A thin part centered in the wall, reaching past the bore by less than the fit
        // tolerance, is no fit to sand: packed parts have no tolerance.
        for (radius_m, offset_m) in [(0.0002, room + 0.0002), (0.0001, room + 0.0005)] {
            assert_eq!(
                placed(mass_component(0.1, 0.05, radius_m), offset_m),
                vec![Finding::InternalPartWiderThanParent {
                    component: "packed".to_owned(),
                    parent: "airframe".to_owned(),
                    reach_m: offset_m + radius_m,
                    room_m: room,
                }]
            );
        }
        // A centering ring as wide is not packed: an error.
        let findings = placed(ring(0.005, 0.0305, 0.0), 0.0);
        assert!(
            matches!(&findings[..], [Finding::InternalPartWiderThanParent { component, .. }]
                if component == "packed"),
            "{findings:?}"
        );
    }

    /// A part drawn wider than its parent's bore by up to the coarse general tolerance of the
    /// bore's diameter (ISO 2768-1 class c) is a fit and warns; 0.1 µm past that is an error. A
    /// bulkhead sized to the airframe's bore at a coupler's end, as OpenRocket's two-stage example
    /// draws its own, is a cap and warns; the same bulkhead inside the coupler, or one too wide
    /// for the airframe too, is an error.
    #[test]
    fn a_part_wider_than_its_parent_warns_only_as_a_fit_or_a_cap() {
        // The airframe: 0.027 m outside, 0.0255 m bore; the coupler's bore is 0.0245 m, a
        // diameter of 49 mm, whose tolerance is 0.8 mm.
        let fitted = |radius_m: f64, at: Position| {
            let mut design = three_fin_rocket();
            let mut coupler = attached("coupler", inner_tube(0.1, 0.0255, 0.001), top(0.1));
            coupler.children = vec![attached("bulkhead", ring(0.005, radius_m, 0.0), at)];
            design.stages[0].components[1].children.push(coupler);
            check(&design).unwrap()
        };
        let room = 0.0255 - 0.001;
        let tight = room + 0.0008;
        assert_eq!(
            fitted(tight, top(0.02)),
            vec![Finding::InternalPartTightInParent {
                component: "bulkhead".to_owned(),
                parent: "coupler".to_owned(),
                reach_m: tight,
                room_m: room,
            }]
        );
        assert_eq!(
            fitted(tight + 1e-7, top(0.02)),
            vec![Finding::InternalPartWiderThanParent {
                component: "bulkhead".to_owned(),
                parent: "coupler".to_owned(),
                reach_m: tight + 1e-7,
                room_m: room,
            }]
        );
        // At either end face, inside or outside the coupler, a bulkhead as wide as the airframe's
        // bore, or wider by its tolerance (0.8 mm for 51 mm), is a cap.
        let capped = 0.0255 + 0.0008;
        for (radius_m, at) in [
            (0.0255, top(0.0)),
            (0.0255, top(-0.005)),
            (0.0255, bottom(0.0)),
            (capped, bottom(0.005)),
        ] {
            let findings = fitted(radius_m, at);
            assert!(!has_errors(&findings));
            // One forward of the coupler's top also runs past its end, which only warns.
            let findings: Vec<_> = findings
                .into_iter()
                .filter(|f| !matches!(f, Finding::InternalPartPastParentEnd { .. }))
                .collect();
            assert_eq!(
                findings,
                vec![Finding::RingAgainstParentEnd {
                    ring: "bulkhead".to_owned(),
                    parent: "coupler".to_owned(),
                    reach_m: radius_m,
                    room_m: 0.0255,
                }],
                "{radius_m} at {at:?}"
            );
        }
        // Inside the coupler, away from its ends, or too wide for the airframe: an error.
        for (radius_m, at) in [(0.0255, top(0.02)), (capped + 1e-7, top(0.0))] {
            let findings = fitted(radius_m, at);
            assert!(
                matches!(&findings[..], [Finding::InternalPartWiderThanParent { component, .. }]
                    if component == "bulkhead"),
                "{radius_m} at {at:?}: {findings:?}"
            );
        }
        // A ring at the end of a clustered or off-axis tube sits beside it, not around it, and a
        // ring as long as its parent covers both ends: neither is a cap.
        for (cluster, offset_m, length_m) in [
            (vec![[0.018, 0.0], [-0.018, 0.0]], 0.0, 0.005),
            (Vec::new(), 0.018, 0.005),
            (Vec::new(), 0.0, 0.1),
        ] {
            let mut design = three_fin_rocket();
            let mut tube = inner_tube(0.1, 0.007, 0.001);
            if let Part::InnerTube(inner) = &mut tube {
                inner.cluster_m = cluster.clone();
                inner.radial_offset_m = offset_m;
            }
            let mut holder = attached("holder", tube, top(0.1));
            holder.children = vec![attached("ring", ring(length_m, 0.012, 0.0), top(0.0))];
            design.stages[0].components[1].children.push(holder);
            let findings = check(&design).unwrap();
            assert!(
                findings.iter().any(|f| matches!(f,
                    Finding::InternalPartWiderThanParent { component, .. } if component == "ring")),
                "{cluster:?} {offset_m} {length_m}: {findings:?}"
            );
        }
        // A tube is no cap, even at its parent's end.
        let mut design = three_fin_rocket();
        let mut coupler = attached("coupler", inner_tube(0.1, 0.0255, 0.001), top(0.1));
        coupler.children = vec![attached(
            "sleeve",
            inner_tube(0.01, 0.0255, 0.001),
            top(0.0),
        )];
        design.stages[0].components[1].children.push(coupler);
        assert!(matches!(
            &check(&design).unwrap()[..],
            [Finding::InternalPartWiderThanParent { component, .. }] if component == "sleeve"
        ));
    }

    /// A centering ring drawn as a child of the motor tube it wraps, as OpenRocket's pods examples
    /// draw theirs, needs room in the airframe around the tube, not in the tube's bore: it fits,
    /// warns as a fit or errs by the airframe's room. A ring whose bore is 1 µm short of the
    /// tube's outside, or whose tube is off the axis or a cluster, is still measured in the tube.
    #[test]
    fn a_ring_around_its_motor_tube_needs_room_in_the_airframe() {
        // The airframe's bore is 0.0255 m (a 51 mm diameter, tolerance 0.8 mm); the motor tube is
        // 0.020 m outside with a 0.019 m bore.
        let wrapped = |outer_m: f64, inner_m: f64, offset_m: f64, cluster: Vec<[f64; 2]>| {
            let mut design = three_fin_rocket();
            let mount = &mut design.stages[0].components[1].children[0];
            if let Part::InnerTube(tube) = &mut mount.part {
                tube.radial_offset_m = offset_m;
                tube.cluster_m = cluster;
            }
            mount.children = vec![attached("wrap", ring(0.003, outer_m, inner_m), top(0.1))];
            check(&design).unwrap()
        };
        let (outer, bore, airframe) = (0.020, 0.019, 0.0255);
        assert_eq!(wrapped(airframe, outer, 0.0, Vec::new()), vec![]);
        // A bore wider than the tube still sits on it.
        assert_eq!(wrapped(airframe, outer + 0.001, 0.0, Vec::new()), vec![]);
        let tight = airframe + 0.0008;
        assert_eq!(
            wrapped(tight, outer, 0.0, Vec::new()),
            vec![Finding::InternalPartTightInParent {
                component: "wrap".to_owned(),
                parent: "airframe".to_owned(),
                reach_m: tight,
                room_m: airframe,
            }]
        );
        assert_eq!(
            wrapped(tight + 1e-7, outer, 0.0, Vec::new()),
            vec![Finding::InternalPartWiderThanParent {
                component: "wrap".to_owned(),
                parent: "airframe".to_owned(),
                reach_m: tight + 1e-7,
                room_m: airframe,
            }]
        );
        // Not wrapping the tube: inside it, so measured against its bore, an error.
        let in_tube = |findings: Vec<Finding>| {
            findings.iter().any(|f| {
                matches!(f, Finding::InternalPartWiderThanParent { component, parent, room_m, .. }
                    if component == "wrap" && parent == "mmt" && *room_m == bore)
            })
        };
        assert!(in_tube(wrapped(airframe, outer - 1e-6, 0.0, Vec::new())));
        assert!(in_tube(wrapped(airframe, outer, 0.002, Vec::new())));
        assert!(in_tube(wrapped(
            airframe,
            outer,
            0.0,
            vec![[0.0, 0.0], [0.05, 0.0]]
        )));
        // The tube's outside less the round-off tolerance still wraps.
        assert_eq!(wrapped(airframe, outer - 5e-10, 0.0, Vec::new()), vec![]);

        // With the motor tube 50 mm out of the airframe's aft end, a ring wholly aft of the
        // airframe, or across its end, has nothing around it that holds it: it is measured in
        // the tube, an error, and no cap at the tube's end.
        let overhung = |at: Position| {
            let mut design = three_fin_rocket();
            let mount = &mut design.stages[0].components[1].children[0];
            mount.position = Some(bottom(0.05));
            mount.children = vec![attached("wrap", ring(0.003, airframe, outer), at)];
            check(&design).unwrap()
        };
        assert!(in_tube(overhung(bottom(0.0))));
        assert!(in_tube(overhung(bottom(-0.048))));
        assert_eq!(
            overhung(top(0.1))
                .into_iter()
                .filter(
                    |f| matches!(f, Finding::InternalPartWiderThanParent { component, .. }
                    | Finding::InternalPartTightInParent { component, .. } if component == "wrap")
                )
                .count(),
            0
        );

        // A tube beside the wrapped one in the airframe, crossing the ring, overlaps it.
        let mut design = three_fin_rocket();
        let airframe_part = &mut design.stages[0].components[1];
        airframe_part.children[0].children =
            vec![attached("wrap", ring(0.003, airframe, outer), top(0.1))];
        let mut side = inner_tube(0.04, 0.004, 0.0005);
        if let Part::InnerTube(tube) = &mut side {
            tube.radial_offset_m = 0.021;
        }
        airframe_part
            .children
            .push(attached("side", side, top(0.58)));
        assert_eq!(
            check(&design).unwrap(),
            vec![Finding::RingOverlapsInnerTube {
                ring: "wrap".to_owned(),
                tube: "side".to_owned(),
            }]
        );
    }

    /// ISO 2768-1's coarse class by the room's diameter, each band up to and including its upper
    /// size; none below 0.5 mm or for a room that isn't a number.
    #[test]
    fn the_fit_tolerance_is_the_coarse_general_tolerance_of_the_bore() {
        for (diameter_m, tolerance_m) in [
            (0.0004, 0.0),
            (0.0005, 0.0002),
            (0.002, 0.0002),
            (0.006, 0.0003),
            (0.0061, 0.0005),
            (0.030, 0.0005),
            (0.0301, 0.0008),
            (0.120, 0.0008),
            (0.150, 0.0012),
            (5.0, 0.004),
        ] {
            assert_eq!(
                fit_tolerance_m(diameter_m / 2.0),
                tolerance_m,
                "{diameter_m}"
            );
        }
        assert_eq!(fit_tolerance_m(f64::NAN), 0.0);
    }

    /// A motor that isn't in its mount at all is an error, whichever way it missed.
    #[test]
    fn motor_outside_its_mount_is_rejected() {
        for overhang in [5.0, 0.2, -0.3] {
            let mut design = three_fin_rocket();
            design.stages[0].components[1].children[0].motor_mount = Some(MotorMount {
                overhang_m: overhang,
            });
            let findings = check(&design).unwrap();
            assert_eq!(
                findings,
                vec![Finding::MotorOutsideMount {
                    configuration: "main".to_owned(),
                    mount: "mmt".to_owned(),
                }],
                "{overhang}"
            );
            assert!(has_errors(&findings));
        }
    }

    /// Parts wholly off the rocket are errors, and a stage center moved off it by an override too.
    #[test]
    fn parts_and_centers_off_the_rocket_are_rejected() {
        let mut design = three_fin_rocket();
        design.stages[0].components[1].children[4].position = Some(top(2.0));
        assert_eq!(
            check(&design).unwrap(),
            vec![Finding::PartOutsideRocket {
                component: "chute".to_owned(),
            }]
        );
        let mut design = three_fin_rocket();
        design.stages[0].overrides.cg_aft_m = Some(350.0);
        let findings = check(&design).unwrap();
        assert!(
            matches!(&findings[..], [Finding::CenterOutsideRocket { stage, station_m }]
                if stage == "sustainer" && *station_m == 350.0),
            "{findings:?}"
        );
        // A retainer on a motor mount sticking out past the airframe is on the rocket.
        let mut design = three_fin_rocket();
        let mount = &mut design.stages[0].components[1].children[0];
        mount.position = Some(bottom(0.012));
        mount.children = vec![attached(
            "retainer",
            mass_component(0.05, 0.01, 0.015),
            bottom(0.0),
        )];
        let findings = check(&design).unwrap();
        assert!(
            matches!(&findings[..], [Finding::InternalPartPastParentEnd { component, .. }]
                if component == "mmt"),
            "{findings:?}"
        );
        // So is one flush against the mount's aft end, wholly behind the airframe.
        let mut flush = design.clone();
        flush.stages[0].components[1].children[0].children[0].position = Some(top(0.3));
        let layout = flush.layout().unwrap();
        let (_, retainer) = layout.find("retainer").unwrap();
        assert!(
            retainer.fore_station_m >= layout.length_m,
            "the test needs it behind"
        );
        let findings = check(&flush).unwrap();
        assert!(
            findings
                .iter()
                .all(|f| !matches!(f, Finding::PartOutsideRocket { .. })),
            "{findings:?}"
        );
        assert!(!has_errors(&findings), "{findings:?}");
        // But one clear of the mount is off the rocket.
        let mut clear = design.clone();
        clear.stages[0].components[1].children[0].children[0].position = Some(top(0.35));
        assert_eq!(
            check(&clear).unwrap(),
            vec![
                Finding::InternalPartPastParentEnd {
                    component: "mmt".to_owned(),
                    parent: "airframe".to_owned(),
                    excess_m: check(&design)
                        .unwrap()
                        .iter()
                        .find_map(|f| match f {
                            Finding::InternalPartPastParentEnd { excess_m, .. } => Some(*excess_m),
                            _ => None,
                        })
                        .unwrap(),
                },
                Finding::PartOutsideRocket {
                    component: "retainer".to_owned(),
                },
            ]
        );
        // Fins trailing far past a light stage's end move its center off the rocket, but with no
        // override that is geometry, not a typo.
        let mut can = body("can", tube(0.3, 0.03, 0.0005));
        let mut fins = attached("trailing-fins", crate::testing::fins(0.2, 0.1), bottom(0.0));
        if let Part::FinSet(set) = &mut fins.part {
            set.planform = crate::FinPlanform::Trapezoidal {
                root_chord_m: 0.2,
                tip_chord_m: 0.2,
                span_m: 0.1,
                sweep_m: 0.4,
            };
        }
        can.children = vec![fins];
        let design = rocket(vec![stage("s", vec![can])]);
        let layout = design.layout().unwrap();
        assert!(
            -layout.stages[0].mass.cg_m.z > layout.length_m,
            "the test needs it off"
        );
        assert!(!has_errors(&check(&design).unwrap()));
        // A mass override can't move a center past its parts, so it doesn't arm the check.
        let mut heavier = design.clone();
        heavier.stages[0].components[0].children[0]
            .overrides
            .mass_kg = Some(1.0);
        assert!(!has_errors(&check(&heavier).unwrap()));
        // Nor can a sideways center override.
        let mut sideways = design.clone();
        sideways.stages[0].components[0].children[0]
            .overrides
            .cg_xy_m = Some([0.0, 0.0]);
        assert!(!has_errors(&check(&sideways).unwrap()));
        // A component's axial center override arms it.
        let mut moved = design.clone();
        moved.stages[0].components[0].children[0].overrides.cg_aft_m = Some(50.0);
        let findings = check(&moved).unwrap();
        assert!(
            matches!(&findings[..], [Finding::NoNoseCone { .. }, Finding::CenterOutsideRocket { stage, .. }] if stage == "s"),
            "{findings:?}"
        );
        // A point mass at the rocket's end is on it, whatever the round-off in the length
        // (0.1 + 0.7 is 0.7999999999999999).
        let mut design = rocket(vec![stage(
            "s",
            vec![
                body("nose", crate::testing::nose(0.1, 0.03)),
                body("airframe", tube(0.7, 0.03, 0.001)),
            ],
        )]);
        design.stages[0].components[1].children = vec![attached(
            "tail-weight",
            mass_component(0.05, 0.0, 0.01),
            Position::Absolute { station_m: 0.8 },
        )];
        assert!(design.layout().unwrap().length_m < 0.8);
        assert!(check(&design).unwrap().is_empty());
        // And a part flush behind the rocket's end is on it, however its length adds up
        // (0.1 + 0.2 is 0.30000000000000004, 0.15 + 0.15 is 0.3).
        for (nose, tube_length) in [(0.1, 0.2), (0.15, 0.15)] {
            let mut design = rocket(vec![stage(
                "s",
                vec![
                    body("nose", crate::testing::nose(nose, 0.03)),
                    body("airframe", tube(tube_length, 0.03, 0.001)),
                ],
            )]);
            design.stages[0].components[1].children = vec![attached(
                "tail-block",
                mass_component(0.05, 0.05, 0.01),
                Position::Absolute { station_m: 0.3 },
            )];
            let findings = check(&design).unwrap();
            assert!(
                matches!(&findings[..], [Finding::InternalPartPastParentEnd { component, .. }]
                    if component == "tail-block"),
                "{nose} + {tube_length}: {findings:?}"
            );
        }
        // Ballast wider than a nose cone, measured against the cone's inside where it sits: packed,
        // so a warning while its center is in the cone (ADR-170).
        let mut design = three_fin_rocket();
        design.stages[0].components[0].children = vec![attached(
            "ballast",
            mass_component(0.1, 0.02, 0.2),
            bottom(0.0),
        )];
        assert!(matches!(
            &check(&design).unwrap()[..],
            [Finding::PackedPartWiderThanParent { component, .. }] if component == "ballast"
        ));
        // A check of a layout with a bad parent index skips it instead of panicking.
        let mut layout = three_fin_rocket().layout().unwrap();
        layout.components[2].parent = Some(999);
        let _ = check_layout(&layout);
    }

    /// #367: a packed mass too wide for a transition's room where it is drawn could sit only
    /// where the transition is wider. Where that is aft (a shoulder, widening aft) its drawn
    /// center of gravity is forward of where it can be, the flattering side, and the warning
    /// names #367; where it is forward (a boattail, narrowing aft) it is the safe side, and the
    /// warning doesn't.
    #[test]
    fn a_packed_part_names_367_only_where_the_room_widens_aft_of_it() {
        let widens = |fore_radius_m: f64, aft_radius_m: f64, position: Option<Position>| {
            let transition = Part::Transition(crate::Transition {
                shape: crate::NoseShape::Conical {},
                clipped: false,
                length_m: 0.2,
                fore_radius_m,
                aft_radius_m,
                wall: crate::Wall::Shell { thickness_m: 0.002 },
                fore_shoulder: None,
                aft_shoulder: None,
                material: crate::Material::bulk("PLA", 1240.0),
            });
            let mut middle = body("transition", transition);
            // A 30 mm long, 20 mm radius mass, 10 mm from the transition's 15 mm end: 13.75 mm
            // of room or less along it, 23 mm at the other end.
            let position = position.unwrap_or(if fore_radius_m < aft_radius_m {
                top(0.01)
            } else {
                bottom(-0.01)
            });
            middle.children = vec![attached("mass", mass_component(0.05, 0.03, 0.02), position)];
            let design = rocket(vec![stage(
                "sustainer",
                vec![
                    body("nose", crate::testing::nose(0.2, fore_radius_m)),
                    middle,
                    body("airframe", tube(0.6, aft_radius_m, 0.0015)),
                ],
            )]);
            let findings = check(&design).unwrap();
            let packed: Vec<_> = findings
                .iter()
                .filter_map(|finding| match finding {
                    Finding::PackedPartWiderThanParent {
                        component,
                        room_widens_aft,
                        ..
                    } if component == "mass" => Some((
                        *room_widens_aft,
                        finding.describe(&|id: &str| id.to_owned()),
                    )),
                    _ => None,
                })
                .collect();
            assert_eq!(packed.len(), 1, "{findings:?}");
            let (flag, text) = packed[0].clone();
            assert_eq!(flag, text.contains("issue #367"), "{text}");
            flag
        };
        assert!(widens(0.015, 0.025, None), "a shoulder widens aft");
        assert!(!widens(0.025, 0.015, None), "a boattail narrows aft");
        // Run 10 mm past a boattail's aft end, it is measured at that end, not by the room the
        // profile gives outside it (found in review).
        assert!(
            !widens(0.025, 0.015, Some(bottom(0.01))),
            "past a boattail's end"
        );
    }

    /// #313: a part in a nose cone is measured against the cone's inside where it sits, not its
    /// base: the outer radius less the 2 mm wall over the part's span. The 0.2 m conical nose of
    /// 27 mm base radius has `r(x) = 0.135 x` outside, so `0.135 x − 0.002` inside.
    #[test]
    fn a_part_in_a_nose_cone_needs_room_where_it_sits() {
        let findings = |part: Part, position: Position| {
            let mut nose = body("nose", crate::testing::nose(0.2, 0.027));
            nose.children = vec![attached("part", part, position)];
            let design = rocket(vec![stage(
                "sustainer",
                vec![nose, body("airframe", tube(0.6, 0.027, 0.0015))],
            )]);
            check(&design).unwrap()
        };
        let room = |x_m: f64| (0.135 * x_m - 0.002_f64).max(0.0);
        // A 20 mm bulkhead at the tip has no room at all there, nor 10 mm aft: an error, measured
        // where the cone is widest along it. Before #313 the cone's 27 mm base let it pass.
        let bulkhead = || ring(0.01, 0.02, 0.0);
        let tip = findings(bulkhead(), top(0.0));
        assert!(
            matches!(&tip[..], [Finding::InternalPartWiderThanParent { component, parent, reach_m, room_m }]
                if component == "part" && parent == "nose" && *reach_m == 0.02 && *room_m == 0.0),
            "{tip:?}"
        );
        // At the base it fits: 23.65 mm of room at its fore face.
        assert_eq!(findings(bulkhead(), bottom(0.0)), vec![]);
        // So does a 20 mm mass there.
        assert_eq!(
            findings(mass_component(0.05, 0.01, 0.02), bottom(0.0)),
            vec![]
        );
        // A 20 mm mass at the tip is packed: a warning while its center is in the room
        // (ADR-170), here on the axis where the wall closes in.
        let packed = findings(mass_component(0.05, 0.03, 0.02), top(0.0));
        assert!(
            matches!(&packed[..], [Finding::PackedPartWiderThanParent { component, room_m, room_widens_aft, .. }]
                if component == "part" && *room_m == 0.0 && *room_widens_aft),
            "{packed:?}"
        );
        // The cone widens aft of it, so its mass can sit only farther aft: the margin may read
        // high, and the warning names #367.
        let text = packed[0].describe(&|id: &str| id.to_owned());
        assert!(text.contains("issue #367"), "{text}");
        assert!(text.contains("margin may read high"), "{text}");
        // Off the axis by 5 mm its center is out of that room: an error.
        let mut off_axis = mass_component(0.05, 0.03, 0.02);
        if let Part::MassComponent(mass) = &mut off_axis {
            mass.packing.radial_offset_m = 0.005;
        }
        let off = findings(off_axis, top(0.0));
        assert!(
            matches!(&off[..], [Finding::InternalPartWiderThanParent { component, reach_m, room_m, .. }]
                if component == "part" && (*reach_m - 0.025).abs() < 1e-15 && *room_m == 0.0),
            "{off:?}"
        );
        // A mass of no packed length at the tip is in the cone, where it has no room: a warning
        // centered, an error off the axis (found in review: it once counted as past the tip).
        let point = findings(mass_component(0.05, 0.0, 0.02), top(0.0));
        assert!(
            matches!(&point[..], [Finding::PackedPartWiderThanParent { room_m, room_widens_aft, .. }]
                if *room_m == 0.0 && *room_widens_aft),
            "{point:?}"
        );
        let mut point_off_axis = mass_component(0.05, 0.0, 0.02);
        if let Part::MassComponent(mass) = &mut point_off_axis {
            mass.packing.radial_offset_m = 0.005;
        }
        let point_off = findings(point_off_axis, top(0.0));
        assert!(
            matches!(&point_off[..], [Finding::InternalPartWiderThanParent { room_m, .. }] if *room_m == 0.0),
            "{point_off:?}"
        );
        // A 24.5 mm tube over the cone's aft 100 mm fits at the base (25 mm of room) and runs into
        // the wall forward, where the cone has 11.5 mm: wedged, a warning.
        let wedged = findings(inner_tube(0.1, 0.0245, 0.001), bottom(0.0));
        match &wedged[..] {
            [
                Finding::InternalPartWedgedInParent {
                    component,
                    parent,
                    reach_m,
                    room_m,
                    widest_room_m,
                },
            ] => {
                assert_eq!((component.as_str(), parent.as_str()), ("part", "nose"));
                assert_eq!(*reach_m, 0.0245);
                assert!((room_m - room(0.1)).abs() < 1e-15, "{room_m}");
                assert!((widest_room_m - room(0.2)).abs() < 1e-15, "{widest_room_m}");
            }
            other => panic!("{other:?}"),
        }
        // Past fit tolerance even at the base: an error, on the widest room.
        let wide = findings(inner_tube(0.1, 0.0265, 0.001), bottom(0.0));
        assert!(
            matches!(&wide[..], [Finding::InternalPartWiderThanParent { room_m, .. }]
                if (room_m - room(0.2)).abs() < 1e-15),
            "{wide:?}"
        );
        // Wholly aft of the base, touching it, a ring is measured as before #313, against the
        // cone's largest radius: it is past the cone's end, and that is the finding.
        let past = findings(ring(0.006, 0.026, 0.0), bottom(0.006));
        assert!(
            matches!(&past[..], [Finding::InternalPartPastParentEnd { component, .. }]
                if component == "part"),
            "{past:?}"
        );
    }

    /// A ring of three 40 mm tubes touches at `40 mm / √3` from the axis and crosses inside it.
    #[test]
    fn a_cluster_whose_tubes_cross_is_flagged() {
        let at = |r: f64| {
            let mut design = three_fin_rocket();
            if let Part::InnerTube(tube) = &mut design.stages[0].components[1].children[0].part {
                tube.cluster_m = [90.0_f64, 210.0, 330.0]
                    .iter()
                    .map(|a| [r * a.to_radians().cos(), r * a.to_radians().sin()])
                    .collect();
            }
            check(&design)
                .unwrap()
                .into_iter()
                .filter(|f| matches!(f, Finding::ClusterTubesOverlap { .. }))
                .collect::<Vec<_>>()
        };
        assert!(at(0.04 / 3.0_f64.sqrt()).is_empty());
        let [
            Finding::ClusterTubesOverlap {
                tube,
                apart_m,
                diameter_m,
            },
        ] = &at(0.02)[..]
        else {
            panic!("one finding");
        };
        assert_eq!(tube, "mmt");
        assert!((apart_m - 0.02 * 3.0_f64.sqrt()).abs() < 1e-15, "{apart_m}");
        assert_eq!(*diameter_m, 0.04);
        assert_eq!(at(0.02)[0].severity(), Severity::Warning);
    }

    /// A finding written before the move to US spelling, `centre_outside_rocket`, reads as the same
    /// finding.
    #[test]
    fn a_finding_with_the_old_uk_kind_reads_the_same() {
        let finding = Finding::CenterOutsideRocket {
            stage: "sustainer".to_owned(),
            station_m: 2.5,
        };
        let text = serde_json::to_string(&finding).unwrap();
        assert!(
            text.contains("\"kind\":\"center_outside_rocket\""),
            "{text}"
        );
        let old = text.replace("center_outside_rocket", "centre_outside_rocket");
        assert_eq!(serde_json::from_str::<Finding>(&old).unwrap(), finding);
    }
}
