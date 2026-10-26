//! What a reader accepted but wants to say out loud.

use serde::{Deserialize, Serialize};

/// Something the reader accepted and flagged, rather than failing over.
///
/// The roadmap asks the `.ork` importer for "graceful warnings instead of failures": a real design
/// written by an older OpenRocket, or by another program, should still open. Every departure from
/// what the file format documents is reported here, with the place in the file it happened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Warning {
    /// Where it happened: a zip entry name, or a slash-separated path of element names from the
    /// root, such as `openrocket/rocket/subcomponents/nosecone`.
    pub at: String,
    /// Whether anything was lost.
    pub kind: WarningKind,
    /// What was read, and how.
    pub message: String,
}

/// How serious a [`Warning`] is.
///
/// The same three levels `hpr_motor::WarningKind` uses for `.eng` and `.rse` files. The two enums
/// are kept apart because their readers report different places (a line number there, a path into
/// an archive or an element tree here) and because `hpr-io` does not otherwise need `hpr-motor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WarningKind {
    /// A whole part of the file was left out of the result, such as an archive entry compressed
    /// by a method this reader does not implement.
    Skipped,
    /// A value was left out.
    Dropped,
    /// Something unusual was read as it stands, such as a schema version newer than any documented.
    Unusual,
}

impl Warning {
    /// Records a warning at `at`.
    pub(crate) fn new(
        at: impl Into<String>,
        kind: WarningKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            at: at.into(),
            kind,
            message: message.into(),
        }
    }
}

/// A value read from a `.ork` file and the warnings its reader raised, in the order they arose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Imported<T> {
    /// The value read.
    pub value: T,
    /// The warnings, in file order.
    pub warnings: Vec<Warning>,
}

impl<T> Imported<T> {
    /// Wraps a value with no warnings.
    pub(crate) fn clean(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
        }
    }

    /// Counts the warnings of one kind.
    pub fn count(&self, kind: WarningKind) -> usize {
        self.warnings.iter().filter(|w| w.kind == kind).count()
    }
}

/// What a part left out carried into the numbers, so that each warning of one says what's no
/// longer counted and which way that leans (#952). Every reader and the writer pick the kind from
/// the part, and the words come from here, so they can't drift apart.
///
/// The leans hold for the rocket as read, the part gone, against the rocket with it:
///
/// - a part's mass is never negative, so without it the mass reads low;
/// - its drag is never negative, so the drag reads low;
/// - taking a mass `m` at station `x_p` away moves the center of gravity, at `x_cg`, away from
///   `x_p`: `x_cg' = (M x_cg − m x_p)/(M − m)`, so `x_cg' − x_cg = m (x_cg − x_p)/(M − m)`. With
///   the center of pressure where it was, the margin `(x_cp − x_cg)/d` reads high for a part aft
///   of the center of gravity and low for one ahead of it;
/// - a part that carries lift moves the center of pressure too, either way, so the margin can
///   read high or low.
///
/// The `hpr` crate's tests read a design with each kind of part, then with it left out, and pin
/// each lean ([`Counted::sentence`]'s words among them).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Counted {
    /// Mass only, inside the body: a tube, coupler, engine block, ring, bulkhead, mass
    /// component, shock cord or fin tab.
    Mass,
    /// Mass, and drag under canopy: a parachute or a streamer.
    Descent,
    /// Mass and drag, outside the body: a launch lug or a rail button.
    MassAndDrag,
    /// Mass, drag and lift: a fin set, a tube fin set, a pod set, a parallel stage or a body
    /// component.
    Lift,
    /// A tag HPR Sim doesn't read, which may carry any of them.
    Unknown,
}

impl Counted {
    /// The kind of the part an element of tag `tag` is.
    pub(crate) fn of_tag(tag: &str) -> Self {
        match tag {
            "innertube" | "tubecoupler" | "engineblock" | "centeringring" | "bulkhead"
            | "masscomponent" | "shockcord" => Self::Mass,
            "parachute" | "streamer" => Self::Descent,
            "launchlug" | "railbutton" => Self::MassAndDrag,
            "trapezoidfinset" | "ellipticalfinset" | "freeformfinset" | "tubefinset" | "podset"
            | "parallelstage" | "nosecone" | "bodytube" | "transition" => Self::Lift,
            _ => Self::Unknown,
        }
    }

    /// The kind of `part`, as the design holds it.
    pub(crate) fn of_part(part: &hpr_design::tree::Part) -> Self {
        use hpr_design::tree::Part;
        match part {
            Part::InnerTube(_)
            | Part::CenteringRing(_)
            | Part::MassComponent(_)
            | Part::ShockCord(_) => Self::Mass,
            Part::Parachute(_) | Part::Streamer(_) => Self::Descent,
            Part::LaunchLug(_) | Part::RailButton(_) => Self::MassAndDrag,
            Part::NoseCone(_)
            | Part::BodyTube(_)
            | Part::Transition(_)
            | Part::FinSet(_)
            | Part::TubeFinSet(_)
            | Part::PodSet(_) => Self::Lift,
            _ => Self::Unknown,
        }
    }

    /// What leaving it out does to the numbers, as the sentences that follow "it was left out":
    /// a space, then the sentences, with no stop at the end, as a warning has none. `many` says
    /// "their" for a part left out with the parts inside it, or several parts.
    pub(crate) fn sentence(self, many: bool) -> String {
        let (its, it) = if many {
            ("Their", "they")
        } else {
            ("Its", "it")
        };
        let margin = format!(
            "The stability margin reads high if {it} sat aft of the center of gravity, low if \
             ahead"
        );
        match self {
            Self::Mass => format!(" {its} mass isn't counted, so the mass reads low. {margin}"),
            Self::Descent => format!(
                " {its} mass and drag under canopy aren't counted, so the mass reads low and the \
                 descent rate high. {margin}"
            ),
            Self::MassAndDrag => format!(" {its} mass and drag aren't counted, so both read low"),
            Self::Lift => format!(
                " {its} mass, drag and lift aren't counted, so the mass and drag read low, and \
                 the stability margin can read high or low"
            ),
            Self::Unknown => format!(
                " Any mass, drag or lift {} isn't counted, so the mass and drag can read low, \
                 and the stability margin high or low",
                if many { "they carry" } else { "it carries" }
            ),
        }
    }
}

/// What follows "it was left out" for something no number HPR Sim works out depends on: XML
/// syntax, an id, OpenRocket's saved results, a catalog's lists, a part's count of none.
pub(crate) const NOTHING_DEPENDS: &str = "no number HPR Sim works out depends on it";

/// What follows "it was left out" for a value that isn't the number, count or flag its tag
/// holds: the reader goes on as if the tag weren't there, and what a missing tag means applies.
pub(crate) const AS_IF_NOT_GIVEN: &str = "as if the file didn't give it";

/// What follows a thrust curve left out: where the motor's curve comes from instead
/// (`super::motors`: a supplied curve, then the bundled catalog's; a configuration none is found
/// for isn't flown).
pub(crate) const ANOTHER_CURVE: &str =
    "the motor flies another curve if one is found, or its configuration isn't flown";
