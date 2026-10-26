//! What leaving a part out does to the numbers, in the words every warning that leaves one out
//! ends with (#952). The `.ork` reader and writer build those words here, from the part, so they
//! can't drift apart.
//!
//! A warning that leaves a part out says two things. First, what the part itself carried, which
//! holds whatever else is in the rocket: its mass, drag, lift and drag under canopy, each a flag
//! combined over the part and everything inside it ([`Counted`]). Then which way the rocket's
//! totals lean without it, against the same rocket with it ([`Lean`]). A lean has a direction
//! only when nothing else in the rocket changes with the part:
//!
//! - a part's mass is never negative, so without it the mass reads low;
//! - its drag is never negative, so the drag reads low;
//! - a canopy that opens in flight sets the descent rate, `v = √(2 m g / (ρ C_D S))` at terminal
//!   speed (Niskanen 2009, *Development of an Open Source model rocket simulation software*,
//!   §3.5), so without its drag area the descent rate reads high; one that never opens carried
//!   only mass, and with less mass `v` reads low;
//! - taking a mass `m` at station `x_p` from a rocket of mass `M` moves the center of gravity
//!   away from `x_p`: `x_cg' = (M x_cg − m x_p)/(M − m)`, so `x_cg' − x_cg = m (x_cg − x_p)/(M − m)`.
//!   With the center of pressure where it was, the margin `(x_cp − x_cg)/d` reads high for a part
//!   aft of the center of gravity and low for one ahead of it;
//! - a part that carries lift moves the center of pressure too, either way, so the margin can read
//!   high or low.
//!
//! What else can change with it, [`Around`]:
//!
//! - an override on a part above it that covers what that part holds (OpenRocket's
//!   `overridesubcomponentsmass`, `…cg` and `…cd`, read into `overrides_include_children` and
//!   [`DragOverride::include_children`]; a stage's override always covers it). The mass, center of
//!   gravity or drag it covers stays as the override sets it, whatever is inside;
//! - a part beside it that takes a size from it: a centering ring's automatic bore is the widest
//!   inner tube beside it ([`hpr_design::tree::AutoDimension::InnerRadius`]). Without the tube the
//!   ring can close to a solid disc, heavier than the tube and the ring were together;
//! - a part beside it placed after it, which moves forward when it goes.
//!
//! Where one of those changes the mass, the center of gravity or the margin, the warning says that
//! number can read high or low, and why. The `hpr` crate's tests (`crates/hpr/src/tests/left_out.rs`)
//! read invented designs with each kind of part, then with it left out, and pin each lean.

use std::ops::BitOr;

use hpr_design::tree::{Component, DragOverride, Overrides, Part, Stage};

use super::component::subcomponents;
use super::document::Element;

/// What follows "it was left out" for something no number HPR Sim works out depends on: XML
/// syntax, an id, OpenRocket's saved results, a catalog's lists, a part's count of none.
pub(crate) const NOTHING_DEPENDS: &str = "no number HPR Sim works out depends on it";

/// [`NOTHING_DEPENDS`] for `count` things: "on it" for one, "on them" for any other count.
pub(crate) fn nothing_depends(count: usize) -> &'static str {
    if count == 1 {
        NOTHING_DEPENDS
    } else {
        "no number HPR Sim works out depends on them"
    }
}

/// What a part, and everything inside it, carried into the numbers: one flag for each, combined
/// over the parts with `|`, so that no part's contribution can hide another's.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Counted {
    /// Mass: every part but a pod set or a stage, whose pods and parts carry it.
    pub(crate) mass: bool,
    /// Drag in flight: a part outside the body.
    pub(crate) drag: bool,
    /// Lift, a normal force that moves the center of pressure: a body component, a fin set or a
    /// tube fin set.
    pub(crate) lift: bool,
    /// Drag under canopy: a parachute or a streamer.
    pub(crate) canopy: bool,
    /// A tag HPR Sim doesn't read, which may carry any of the others: they're said as "any".
    pub(crate) unknown: bool,
}

impl BitOr for Counted {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            mass: self.mass | other.mass,
            drag: self.drag | other.drag,
            lift: self.lift | other.lift,
            canopy: self.canopy | other.canopy,
            unknown: self.unknown | other.unknown,
        }
    }
}

impl Counted {
    /// Nothing: a pod set, a parallel stage or a stage of its own, which its parts carry.
    pub(crate) const NOTHING: Self = Self {
        mass: false,
        drag: false,
        lift: false,
        canopy: false,
        unknown: false,
    };
    /// A part inside the body: a tube, coupler, engine block, ring, bulkhead, mass component,
    /// shock cord or fin tab.
    pub(crate) const MASS: Self = Self {
        mass: true,
        ..Self::NOTHING
    };
    /// A part outside the body with no lift: a launch lug or a rail button.
    const OUTSIDE: Self = Self {
        drag: true,
        ..Self::MASS
    };
    /// A part with lift: a nose cone, body tube, transition, fin set or tube fin set.
    const LIFT: Self = Self {
        lift: true,
        ..Self::OUTSIDE
    };
    /// A parachute or a streamer.
    const CANOPY: Self = Self {
        canopy: true,
        ..Self::MASS
    };
    /// A tag HPR Sim doesn't read.
    const UNKNOWN: Self = Self {
        unknown: true,
        ..Self::LIFT
    };

    /// What the part an element of tag `tag` is carries of its own.
    pub(crate) fn of_tag(tag: &str) -> Self {
        match tag {
            "innertube" | "tubecoupler" | "engineblock" | "centeringring" | "bulkhead"
            | "masscomponent" | "shockcord" => Self::MASS,
            "parachute" | "streamer" => Self::CANOPY,
            "launchlug" | "railbutton" => Self::OUTSIDE,
            "trapezoidfinset" | "ellipticalfinset" | "freeformfinset" | "tubefinset"
            | "nosecone" | "bodytube" | "transition" => Self::LIFT,
            "podset" | "parallelstage" | "stage" => Self::NOTHING,
            _ => Self::UNKNOWN,
        }
    }

    /// What `part` carries of its own, as the design holds it.
    pub(crate) fn of_part(part: &Part) -> Self {
        match part {
            Part::InnerTube(_)
            | Part::CenteringRing(_)
            | Part::MassComponent(_)
            | Part::ShockCord(_) => Self::MASS,
            Part::Parachute(_) | Part::Streamer(_) => Self::CANOPY,
            Part::LaunchLug(_) | Part::RailButton(_) => Self::OUTSIDE,
            Part::NoseCone(_)
            | Part::BodyTube(_)
            | Part::Transition(_)
            | Part::FinSet(_)
            | Part::TubeFinSet(_) => Self::LIFT,
            Part::PodSet(_) => Self::NOTHING,
            _ => Self::UNKNOWN,
        }
    }

    /// What `element` and every element inside it, at any depth, carry.
    pub(crate) fn of_element(element: &Element) -> Self {
        subcomponents(element)
            .map(Self::of_element)
            .fold(Self::of_tag(&element.name), BitOr::bitor)
    }

    /// What `component` and every part inside it, at any depth, carry.
    pub(crate) fn of_component(component: &Component) -> Self {
        component
            .children
            .iter()
            .map(Self::of_component)
            .fold(Self::of_part(&component.part), BitOr::bitor)
    }
}

/// A part kind's name as a reader would say it: `inner tube`, not `inner_tube`.
pub(crate) fn spoken(part: &Part) -> String {
    part.kind_name().replace('_', " ")
}

/// Which of the rocket's totals an override above a part sets for everything it holds, the part
/// among them: for each, the kind of the outermost part whose override sets it, as said in a
/// warning ("body tube", "stage").
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Covered {
    /// The mass.
    mass: Option<String>,
    /// The center of gravity.
    cg: Option<String>,
    /// The drag.
    drag: Option<String>,
}

impl Covered {
    /// What is covered inside a part said as `kind` whose overrides are `overrides` and `drag`,
    /// `include` saying whether its mass and center-of-gravity overrides cover what it holds. An
    /// override further out already covering a total keeps it: it sets the total last.
    pub(crate) fn within(
        &self,
        kind: &str,
        overrides: &Overrides,
        include: bool,
        drag: Option<DragOverride>,
    ) -> Self {
        let set = |above: &Option<String>, here: bool| {
            above.clone().or_else(|| here.then(|| kind.to_owned()))
        };
        Self {
            mass: set(&self.mass, include && overrides.mass_kg.is_some()),
            cg: set(&self.cg, include && overrides.cg_aft_m.is_some()),
            drag: set(&self.drag, drag.is_some_and(|drag| drag.include_children)),
        }
    }

    /// What is covered inside `component`, by its overrides as the design holds them.
    pub(crate) fn within_component(&self, component: &Component) -> Self {
        self.within(
            &spoken(&component.part),
            &component.overrides,
            component.overrides_include_children,
            component.drag_override,
        )
    }

    /// What is covered inside `stage`: a stage's mass and center-of-gravity overrides are the
    /// whole stage's ([`hpr_design::Rocket::layout`]).
    pub(crate) fn within_stage(&self, stage: &Stage) -> Self {
        self.within("stage", &stage.overrides, true, stage.drag_override)
    }
}

/// What else in the rocket changes when a part is left out.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Around {
    /// What the overrides above it set, whatever is inside them.
    pub(crate) covered: Covered,
    /// A part beside it takes a size from it: a centering ring whose bore is automatic, beside a
    /// tube.
    pub(crate) sizes: bool,
    /// A part beside it is placed after it.
    pub(crate) places: bool,
}

/// Which way one of the rocket's totals moves without the parts left out.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Total {
    /// It doesn't move: the parts carried none of it.
    Same,
    /// It reads low (for the center of gravity: it moves).
    Low,
    /// It stays as the overrides on these kinds of part set it.
    Kept(Vec<String>),
    /// It can read high or low, because a part sized from one left out changes too.
    Either,
}

impl Total {
    /// Both parts' moves together.
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Same, total) | (total, Self::Same) => total,
            (Self::Either, _) | (_, Self::Either) => Self::Either,
            (Self::Low, _) | (_, Self::Low) => Self::Low,
            (Self::Kept(mut kinds), Self::Kept(more)) => {
                for kind in more {
                    if !kinds.contains(&kind) {
                        kinds.push(kind);
                    }
                }
                Self::Kept(kinds)
            }
        }
    }

    /// Whether an override on a part of `kind` sets it.
    fn kept_by(&self, kind: &str) -> bool {
        matches!(self, Self::Kept(kinds) if kinds.iter().any(|k| k == kind))
    }
}

/// Which way the stability margin moves without the parts left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Margin {
    /// It doesn't move.
    Same,
    /// High for parts aft of the center of gravity, low for parts ahead of it.
    ByPlace,
    /// Either way.
    Either,
}

impl Margin {
    /// Both parts' moves together.
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Same, margin) | (margin, Self::Same) => margin,
            (Self::ByPlace, Self::ByPlace) => Self::ByPlace,
            _ => Self::Either,
        }
    }
}

/// What leaving parts out does to the numbers: what they carried, and which way each total leans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lean {
    /// What they carried.
    counted: Counted,
    /// The rocket's mass.
    mass: Total,
    /// Its center of gravity, said only where an override holds it.
    cg: Total,
    /// Its drag in flight.
    drag: Total,
    /// Its stability margin.
    margin: Margin,
    /// A part placed after one left out moves.
    placed: bool,
    /// The kinds of part whose override sets the mass or the center of gravity of what it holds,
    /// not both, so that the center of gravity can move either way.
    partly: Vec<String>,
}

impl Lean {
    /// What leaving out parts that carried `counted`, with `around` around them, does.
    pub(crate) fn new(counted: Counted, around: &Around) -> Self {
        let covered = &around.covered;
        let kept = |by: &Option<String>| by.clone().map_or(Total::Low, |by| Total::Kept(vec![by]));
        let mass = match (counted.mass, kept(&covered.mass)) {
            (false, _) => Total::Same,
            (true, Total::Low) if around.sizes => Total::Either,
            (true, total) => total,
        };
        let drag = if counted.drag {
            kept(&covered.drag)
        } else {
            Total::Same
        };
        // The center of gravity of what an override holds stays put only when one override
        // sets both its mass and its center.
        let held = counted.mass && covered.mass.is_some() && covered.mass == covered.cg;
        let cg = match (counted.mass, held) {
            (false, _) => Total::Same,
            (true, true) => kept(&covered.cg),
            (true, false) => Total::Low,
        };
        let partly: Vec<String> = match (counted.mass && !held, &covered.mass, &covered.cg) {
            (true, Some(by), _) | (true, None, Some(by)) => vec![by.clone()],
            _ => Vec::new(),
        };
        let placed = counted.mass && !held && around.places;
        let margin = if counted.lift || mass == Total::Either || placed || !partly.is_empty() {
            Margin::Either
        } else if counted.mass && !held {
            Margin::ByPlace
        } else {
            Margin::Same
        };
        Self {
            counted,
            mass,
            cg,
            drag,
            margin,
            placed,
            partly,
        }
    }

    /// Both sets of parts left out together.
    pub(crate) fn and(self, other: Self) -> Self {
        let mut partly = self.partly;
        for kind in other.partly {
            if !partly.contains(&kind) {
                partly.push(kind);
            }
        }
        Self {
            counted: self.counted | other.counted,
            mass: self.mass.and(other.mass),
            cg: self.cg.and(other.cg),
            drag: self.drag.and(other.drag),
            margin: self.margin.and(other.margin),
            placed: self.placed || other.placed,
            partly,
        }
    }

    /// The sentences that follow "it was left out": a space, then what isn't counted, then which
    /// way each total leans, with no stop at the end, as a warning has none. `many` says "their"
    /// for a part left out with the parts inside it, or several parts.
    pub(crate) fn sentence(&self, many: bool) -> String {
        let counted = self.counted;
        let (its, carries, opens, not) = if many {
            ("Their", "they carry", "one of them opens", "none does")
        } else {
            ("Its", "it carries", "it opens", "it doesn't")
        };
        let carried: Vec<&str> = [
            (counted.mass, "mass"),
            (counted.drag, "drag"),
            (counted.lift, "lift"),
            (counted.canopy, "drag under canopy"),
        ]
        .into_iter()
        .filter_map(|(carried, what)| carried.then_some(what))
        .collect();
        if carried.is_empty() {
            // The same words as everything else no number depends on, as a sentence of its own.
            let nothing = nothing_depends(if many { 2 } else { 1 });
            return format!(" No{}", nothing.strip_prefix("no").unwrap_or(nothing));
        }
        let mut out = if counted.unknown {
            format!(" Any {} {carries} isn't counted", list(&carried, "or"))
        } else {
            format!(
                " {its} {} {} counted",
                list(&carried, "and"),
                if carried.len() == 1 {
                    "isn't"
                } else {
                    "aren't"
                }
            )
        };
        let low: Vec<&str> = [(&self.mass, "mass"), (&self.drag, "drag")]
            .into_iter()
            .filter_map(|(total, what)| (*total == Total::Low).then_some(what))
            .collect();
        if !low.is_empty() {
            let reads = match (counted.unknown, low.len()) {
                (true, _) => "can read",
                (false, 1) => "reads",
                (false, _) => "read",
            };
            out += &format!(", so the {} {reads} low", list(&low, "and"));
        }
        let mut setters: Vec<&String> = Vec::new();
        for total in [&self.mass, &self.drag] {
            if let Total::Kept(kinds) = total {
                for kind in kinds {
                    if !setters.contains(&kind) {
                        setters.push(kind);
                    }
                }
            }
        }
        for kind in setters {
            let mut set = Vec::new();
            if self.mass.kept_by(kind) {
                set.push("mass");
                if self.cg.kept_by(kind) {
                    set.push("center of gravity");
                }
            }
            if self.drag.kept_by(kind) {
                set.push("drag");
            }
            let (stays, them) = if set.len() == 1 {
                ("stays", "it")
            } else {
                ("stay", "them")
            };
            out += &format!(
                ". The {} {stays} as the {kind}'s override sets {them}",
                list(&set, "and")
            );
        }
        if counted.canopy {
            out += &format!(". The descent rate reads high if {opens} in flight");
            if self.mass == Total::Low {
                out += &format!(", low if {not}");
            }
        }
        // Why the margin has no direction: the parts sized or placed from it, then each override
        // that sets only one of the mass and the center of gravity.
        let mut why: Vec<String> = Vec::new();
        if self.mass == Total::Either {
            why.push(format!(
                "parts {} from it change too",
                if self.placed {
                    "sized or placed"
                } else {
                    "sized"
                }
            ));
        } else if self.placed {
            why.push("parts placed after it move too".to_owned());
        }
        for kind in &self.partly {
            why.push(format!(
                "the {kind}'s override sets the mass or the center of gravity of what it holds, \
                 not both"
            ));
        }
        let why = if why.is_empty() {
            String::new()
        } else {
            format!(", as {}", why.join(", and "))
        };
        if self.mass == Total::Either {
            out += &format!(". The mass and stability margin can read high or low{why}");
            return out;
        }
        match self.margin {
            Margin::Same => {}
            Margin::ByPlace => {
                out += ". The stability margin reads high for a part aft of the center of \
                        gravity, low for one ahead";
            }
            Margin::Either => out += &format!(". The stability margin can read high or low{why}"),
        }
        out
    }
}

/// `["mass", "drag", "lift"]` as `mass, drag and lift`, with `and` or `or` before the last.
fn list(items: &[&str], and: &str) -> String {
    match items {
        [] => String::new(),
        [one] => (*one).to_owned(),
        [rest @ .., last] => format!("{} {and} {last}", rest.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words for each set of flags, with nothing else changing.
    #[test]
    fn each_part_says_what_it_carried_and_which_way_the_totals_lean() {
        let say =
            |counted: Counted, many: bool| Lean::new(counted, &Around::default()).sentence(many);
        assert_eq!(
            say(Counted::MASS, false),
            " Its mass isn't counted, so the mass reads low. The stability margin reads high for a \
             part aft of the center of gravity, low for one ahead"
        );
        assert_eq!(
            say(Counted::OUTSIDE, false),
            " Its mass and drag aren't counted, so the mass and drag read low. The stability \
             margin reads high for a part aft of the center of gravity, low for one ahead"
        );
        assert_eq!(
            say(Counted::LIFT, false),
            " Its mass, drag and lift aren't counted, so the mass and drag read low. The stability \
             margin can read high or low"
        );
        assert_eq!(
            say(Counted::CANOPY, false),
            " Its mass and drag under canopy aren't counted, so the mass reads low. The descent \
             rate reads high if it opens in flight, low if it doesn't. The stability margin reads \
             high for a part aft of the center of gravity, low for one ahead"
        );
        assert_eq!(
            say(Counted::LIFT | Counted::CANOPY, true),
            " Their mass, drag, lift and drag under canopy aren't counted, so the mass and drag \
             read low. The descent rate reads high if one of them opens in flight, low if none \
             does. The stability margin can read high or low"
        );
        assert_eq!(
            say(Counted::UNKNOWN, false),
            " Any mass, drag or lift it carries isn't counted, so the mass and drag can read low. \
             The stability margin can read high or low"
        );
        assert_eq!(
            say(Counted::NOTHING, false),
            " No number HPR Sim works out depends on it"
        );
        assert_eq!(
            say(Counted::NOTHING, true),
            " No number HPR Sim works out depends on them"
        );
    }

    /// Each flag of a part inside another counts in the union, so none of its clauses is lost:
    /// a pod set's lift and its parachute's canopy, a lug's margin beside a ring's.
    #[test]
    fn flags_combine_over_the_parts_inside() {
        let pods =
            Counted::of_tag("podset") | Counted::of_tag("bodytube") | Counted::of_tag("parachute");
        assert_eq!(pods, Counted::LIFT | Counted::CANOPY);
        for (tag, part) in [
            ("launchlug", Counted::OUTSIDE),
            ("railbutton", Counted::OUTSIDE),
            ("trapezoidfinset", Counted::LIFT),
            ("tubefinset", Counted::LIFT),
            ("innertube", Counted::MASS),
            ("streamer", Counted::CANOPY),
            ("podset", Counted::NOTHING),
            ("boosterset", Counted::UNKNOWN),
        ] {
            assert_eq!(Counted::of_tag(tag), part, "{tag}");
        }
    }

    /// A total an override above sets stays as it sets it, with no direction; the margin moves
    /// either way when the override sets only one of the mass and the center of gravity.
    #[test]
    fn an_override_above_keeps_what_it_covers() {
        let covers = |mass: bool, cg: bool, drag: bool| Around {
            covered: Covered::default().within(
                "body tube",
                &Overrides {
                    mass_kg: mass.then_some(1.0),
                    cg_aft_m: cg.then_some(0.1),
                    ..Overrides::default()
                },
                true,
                drag.then_some(DragOverride {
                    coefficient: 0.5,
                    include_children: true,
                }),
            ),
            ..Around::default()
        };
        assert_eq!(
            Lean::new(Counted::MASS, &covers(true, true, false)).sentence(false),
            " Its mass isn't counted. The mass and center of gravity stay as the body tube's \
             override sets them"
        );
        assert_eq!(
            Lean::new(Counted::OUTSIDE, &covers(true, true, true)).sentence(false),
            " Its mass and drag aren't counted. The mass, center of gravity and drag stay as the \
             body tube's override sets them"
        );
        assert_eq!(
            Lean::new(Counted::MASS, &covers(true, false, false)).sentence(false),
            " Its mass isn't counted. The mass stays as the body tube's override sets it. The \
             stability margin can read high or low, as the body tube's override sets the mass or \
             the center of gravity of what it holds, not both"
        );
        assert_eq!(
            Lean::new(Counted::MASS, &covers(false, true, false)).sentence(false),
            " Its mass isn't counted, so the mass reads low. The stability margin can read high \
             or low, as the body tube's override sets the mass or the center of gravity of what \
             it holds, not both"
        );
        assert_eq!(
            Lean::new(Counted::OUTSIDE, &covers(false, false, true)).sentence(false),
            " Its mass and drag aren't counted, so the mass reads low. The drag stays as the body \
             tube's override sets it. The stability margin reads high for a part aft of the \
             center of gravity, low for one ahead"
        );
        // An override that doesn't cover what its part holds keeps nothing.
        let own = Around {
            covered: Covered::default().within(
                "body tube",
                &Overrides {
                    mass_kg: Some(1.0),
                    ..Overrides::default()
                },
                false,
                None,
            ),
            ..Around::default()
        };
        assert_eq!(own, Around::default());
        // The outermost override sets the total last.
        let outer = Covered::default()
            .within(
                "stage",
                &Overrides {
                    mass_kg: Some(2.0),
                    ..Overrides::default()
                },
                true,
                None,
            )
            .within(
                "body tube",
                &Overrides {
                    mass_kg: Some(1.0),
                    ..Overrides::default()
                },
                true,
                None,
            );
        assert_eq!(outer.mass.as_deref(), Some("stage"));
    }

    /// A part sized or placed from one left out changes too, so the totals it moves have no
    /// direction.
    #[test]
    fn parts_sized_or_placed_from_it_take_the_direction_away() {
        let sized = Around {
            sizes: true,
            ..Around::default()
        };
        assert_eq!(
            Lean::new(Counted::MASS, &sized).sentence(false),
            " Its mass isn't counted. The mass and stability margin can read high or low, as \
             parts sized from it change too"
        );
        let both = Around {
            places: true,
            ..sized
        };
        assert_eq!(
            Lean::new(Counted::MASS, &both).sentence(false),
            " Its mass isn't counted. The mass and stability margin can read high or low, as \
             parts sized or placed from it change too"
        );
        let placed = Around {
            places: true,
            ..Around::default()
        };
        assert_eq!(
            Lean::new(Counted::OUTSIDE, &placed).sentence(false),
            " Its mass and drag aren't counted, so the mass and drag read low. The stability \
             margin can read high or low, as parts placed after it move too"
        );
        // An override above that sets only the center of gravity of what it holds is said too,
        // beside the parts sized from it.
        let partly = Around {
            covered: Covered::default().within(
                "body tube",
                &Overrides {
                    cg_aft_m: Some(0.1),
                    ..Overrides::default()
                },
                true,
                None,
            ),
            ..sized
        };
        assert_eq!(
            Lean::new(Counted::MASS, &partly).sentence(false),
            " Its mass isn't counted. The mass and stability margin can read high or low, as \
             parts sized from it change too, and the body tube's override sets the mass or the \
             center of gravity of what it holds, not both"
        );
    }

    /// Leans of several parts combine: a low total stays low beside a kept one, and a margin by
    /// place beside one either way is either way.
    #[test]
    fn leans_combine_over_several_parts() {
        let kept = Around {
            covered: Covered {
                mass: Some("stage".to_owned()),
                cg: Some("stage".to_owned()),
                drag: None,
            },
            ..Around::default()
        };
        let both =
            Lean::new(Counted::MASS, &kept).and(Lean::new(Counted::MASS, &Around::default()));
        assert_eq!(both, Lean::new(Counted::MASS, &Around::default()));
        let fins =
            Lean::new(Counted::MASS, &Around::default()).and(Lean::new(Counted::LIFT, &kept));
        assert_eq!(
            fins.sentence(true),
            " Their mass, drag and lift aren't counted, so the mass and drag read low. The \
             stability margin can read high or low"
        );
    }
}
