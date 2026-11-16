//! The tangent-ogive nose's line inside the drag part of the gap to OpenRocket (M2.3c13, ADR-349).
//!
//! From Mach 1.3 up, OpenRocket 24.12 gives a tangent-ogive nose almost no pressure drag, where
//! HPR Sim gives it the equivalent cone's (M2.3c10; OpenRocket's issue #3391). Between Mach 1 and
//! 1.3 each joins that value to its own at Mach 1. This line sizes what the two noses' difference
//! above Mach 1 does to the apogee. On OpenRocket's thrust, mass and center of mass, HPR Sim
//! flies once more with its own pressure drag of the nose replaced, above Mach 1, by the column
//! OpenRocket's public interface reports for the nose ([`NoseSwap`]):
//!
//! ```text
//! C_D0' = C_D0 − C_D,nose (HPR Sim's, on the reference area)
//!              + C_D,nose (OpenRocket's column) · A_OR / A_ref      for Mach > 1
//! C_D0' = C_D0                                                      for Mach ≤ 1
//!
//! nose = (A₂ − A_n) / A_OR        A₂: on OpenRocket's thrust and mass (the split's)
//! rest = (A_n − A₃) / A_OR        A_n: and on its nose's column
//! drag = nose + rest              A₃: and on its whole drag (the split's)
//! ```
//!
//! `A_OR / A_ref` puts the column, which is on OpenRocket's reference area, on HPR Sim's. Every
//! other term of the drag stays HPR Sim's in `A_n`: its friction, its base drag, its fins and the
//! nose's own friction.
//!
//! The line's flights are those whose own HPR Sim flight passes Mach 1 with a tangent-ogive nose
//! at the front of the airframe ([`in_line`]). Each is swapped, or skipped under the first of
//! [`SKIPPED`] that applies. The sizes are published only over [`FLOOR`] flights or more: an
//! aggregate over one or two flights is a flight's own number (ADR-151).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use hpr_aero::{AeroError, DragModel, DragQuery};
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use hpr_design::{Layout, NoseShape, Part};
use hpr_sim::{FlightStep, Observer, SimError, Simulation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fixture_flights::Tally;
use crate::gap_split::{Air, NotSplit, OpenRocketInputs, Size, SplitFlight};

/// OpenRocket's column for each flight's nose (private, ADR-151).
pub(crate) const COLLECTION_RECORD: &str =
    "corpus-out/hpr-sim-fixtures/openrocket-nose-columns.json";

/// The same for the public designs.
#[cfg(test)]
pub(crate) const PUBLIC_RECORD: &str = "validation/fixtures/ork/openrocket-nose-columns.json";

/// The script that writes both records.
pub(crate) const ORACLE: &str = "validation/oracles/openrocket/nose_columns.py";

/// The oracle's reasons a flight has no column: the record's `reasons` must name exactly these.
pub(crate) const REASONS: [&str; 5] = [
    "not_opened",
    "no_nose",
    "several_noses",
    "not_finite",
    "not_repeatable",
];

/// Why a flight of the line is skipped, in the order they are tried, with the report's words.
pub(crate) const SKIPPED: [(&str, &str); 5] = [
    (
        "several_noses",
        "the design holds more than one nose cone, and no probe shows how OpenRocket's column \
         counts a repeated nose",
    ),
    ("not_split", "the flight is not split, as counted above"),
    (
        "no_column",
        "OpenRocket's record holds no column for the nose",
    ),
    (
        "stated",
        "the design states the nose's drag, so HPR Sim has no pressure drag of its own to swap",
    ),
    (
        "refused",
        "HPR Sim fails the flight with the column swapped in, or the flight leaves the Mach \
         numbers the column covers",
    ),
];

/// The Mach number from which M2.3c10's public probe shows OpenRocket 24.12 giving a
/// tangent-ogive nose almost no pressure drag (its probe from Mach 1, #1500, shows the two
/// joined below it). The report counts the swapped flights that pass it.
const PROBE_FROM_MACH: f64 = 1.3;

/// The fewest flights swapped whose sizes are published (ADR-349 §3, ADR-151).
pub(crate) const FLOOR: usize = 3;

/// The Mach number above which the nose's column is swapped in.
const ABOVE_MACH: f64 = 1.0;

/// The largest difference, per cent of OpenRocket's apogee, between the line and the rest summed
/// and the drag part: each is a few floating-point operations on apogees of the same size.
const SUM_TOLERANCE_PERCENT: f64 = 1e-9;

/// OpenRocket's pressure drag coefficient of one nose against Mach number, on OpenRocket's
/// reference area.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NoseColumn {
    table: Table1D,
    reference_area_m2: f64,
    /// OpenRocket's name for the nose's shape, and its shape parameter.
    shape: (String, f64),
}

impl NoseColumn {
    /// A record flight's column: `Ok(Err(reason))` when the record lacks it.
    ///
    /// # Errors
    ///
    /// A record that is not `nose_columns.py`'s: no rows, a row that isn't two numbers, Mach
    /// numbers that don't rise, a coefficient below zero, or no reference area or shape.
    pub(crate) fn of(flight: &Value) -> Result<Result<Self, String>, String> {
        if let Some(lacks) = flight.get("lacks") {
            let reason = lacks.as_str().ok_or("a nose column's reason isn't text")?;
            return Ok(Err(reason.to_owned()));
        }
        let rows = flight["rows"]
            .as_array()
            .ok_or("a flight holding a nose column has no rows")?;
        let mut machs = Vec::with_capacity(rows.len());
        let mut coefficients = Vec::with_capacity(rows.len());
        for row in rows {
            let pair = (row[0].as_f64(), row[1].as_f64());
            let (Some(mach), Some(coefficient)) = pair else {
                return Err("a nose column's row isn't two numbers".to_owned());
            };
            if !(coefficient.is_finite() && coefficient >= 0.0) {
                return Err(format!("a nose column holds {coefficient} at Mach {mach}"));
            }
            machs.push(mach);
            coefficients.push(coefficient);
        }
        let table = Table1D::new(
            machs,
            coefficients,
            Interpolation::Linear,
            Extrapolation::Error,
        )
        .map_err(|error| format!("a nose column: {error}"))?;
        let reference_area_m2 = flight["reference_area_m2"]
            .as_f64()
            .filter(|area| area.is_finite() && *area > 0.0)
            .ok_or("a flight holding a nose column has no reference area")?;
        let shape = flight["nose"]["shape"]
            .as_str()
            .zip(flight["nose"]["shape_parameter"].as_f64())
            .ok_or("a flight holding a nose column doesn't say the nose's shape")?;
        Ok(Ok(Self {
            table,
            reference_area_m2,
            shape: (shape.0.to_owned(), shape.1),
        }))
    }

    /// Whether OpenRocket read the nose as a tangent ogive too.
    fn is_tangent_ogive(&self) -> bool {
        self.shape.0 == "OGIVE" && self.shape.1 == 1.0
    }
}

/// HPR Sim's drag with the pressure drag of the part `nose_id` replaced, above
/// [`NoseSwap::above_mach`], by OpenRocket's column for it. See the module's equations.
#[derive(Debug)]
pub(crate) struct NoseSwap {
    nose_id: String,
    column: NoseColumn,
    above_mach: f64,
}

impl NoseSwap {
    pub(crate) fn new(nose_id: &str, column: NoseColumn) -> Self {
        Self {
            nose_id: nose_id.to_owned(),
            column,
            above_mach: ABOVE_MACH,
        }
    }
}

impl DragModel for NoseSwap {
    fn zero_lift_drag(&self, query: &DragQuery<'_>) -> Result<f64, AeroError> {
        let own = query.buildup()?.zero_lift_coefficient;
        let mach = query.mach();
        // Not above it: HPR Sim's own drag, bit for bit. A Mach number that isn't a number
        // isn't above it either, and the buildup has already refused it.
        if mach.partial_cmp(&self.above_mach) != Some(std::cmp::Ordering::Greater) {
            return Ok(own);
        }
        let nose = query
            .buildup_components()?
            .into_iter()
            .find(|component| component.id == self.nose_id)
            .ok_or(AeroError::Domain {
                what: "Mach number at which the buildup lists no nose to swap",
                value: mach,
            })?
            .drag
            .pressure;
        let theirs = self
            .column
            .table
            .lookup(mach)
            .map_err(|_| AeroError::Domain {
                what: "Mach number past the end of OpenRocket's nose column",
                value: mach,
            })?
            .value;
        Ok(own - nose + theirs * self.column.reference_area_m2 / query.reference_area_m2())
    }
}

/// The id of the nose cone at the front of `layout`'s airframe when it is a tangent ogive (an
/// ogive whose radius is the tangent one's, `radius_ratio` 1), and how many nose cones the
/// layout holds.
pub(crate) fn tangent_ogive_nose(layout: &Layout) -> Option<(&str, usize)> {
    let first = layout.body().next()?;
    let Part::NoseCone(nose) = &first.part else {
        return None;
    };
    let tangent = matches!(nose.shape, NoseShape::Ogive { radius_ratio } if radius_ratio == 1.0);
    let noses = layout
        .components
        .iter()
        .filter(|component| matches!(component.part, Part::NoseCone(_)))
        .count();
    tangent.then_some((first.id.as_str(), noses))
}

/// The rule for the line's flights: the flight's own top Mach number is above 1, and its
/// airframe's nose is a tangent ogive.
pub(crate) fn in_line(max_mach: f64, layout: &Layout) -> bool {
    max_mach > ABOVE_MACH && tangent_ogive_nose(layout).is_some()
}

/// The largest Mach number at the end of a step, and whether every one was a number.
struct TopMach {
    top: f64,
    finite: bool,
}

impl Observer for TopMach {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let end = step.sample(step.end_s())?;
        // `f64::max` drops a NaN, so it is kept apart.
        self.finite &= end.mach.is_finite();
        self.top = self.top.max(end.mach);
        Ok(())
    }
}

/// The top Mach number of `simulation`'s flight.
///
/// # Errors
///
/// A flight that fails, or one with a Mach number that isn't a number at the end of a step.
pub(crate) fn max_mach(simulation: &Simulation) -> Result<f64, String> {
    let mut top = TopMach {
        top: 0.0,
        finite: true,
    };
    simulation
        .run(&mut top)
        .map_err(|error| error.to_string())?;
    if !top.finite {
        return Err("a Mach number along the flight is not a number".to_owned());
    }
    Ok(top.top)
}

/// One flight of the line.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NoseFlight {
    /// The nose's line and the rest of the drag part, each per cent of OpenRocket's apogee, with
    /// the drag part they sum to and the top Mach number of HPR Sim's own flight.
    Swapped {
        nose: f64,
        rest: f64,
        drag: f64,
        top_mach: f64,
    },
    /// Not swapped: an index into [`SKIPPED`].
    Skipped(usize),
}

/// Whether the design states the drag of the part `nose_id`, so that the buildup holds no
/// pressure drag of its own for it: the part's own stated drag, or one that covers it, its
/// stage's or a parent's with its children included. It is the buildup's own rule for which
/// parts a stated drag covers.
fn stated(simulation: &Simulation, nose_id: &str) -> bool {
    let layout = &simulation.assembly().layout;
    let Some((_, nose)) = layout.find(nose_id) else {
        return true;
    };
    if nose.drag_override.is_some() {
        return true;
    }
    let covers =
        |stated: Option<hpr_design::DragOverride>| stated.is_some_and(|o| o.include_children);
    if covers(layout.stages.get(nose.stage).and_then(|s| s.drag_override)) {
        return true;
    }
    // At most one step up per part, so a layout whose parents loop ends the walk.
    let mut parent = nose.parent;
    for _ in 0..layout.components.len() {
        let Some(up) = parent.and_then(|at| layout.components.get(at)) else {
            break;
        };
        if covers(up.drag_override) {
            return true;
        }
        parent = up.parent;
    }
    false
}

/// One flight both simulators fly, as the line counts it: `None` when it is not one of the
/// line's flights ([`in_line`]). `build` and `climbed_m` are [`crate::gap_split::split`]'s;
/// `histories` is OpenRocket's record of the flight's inputs, `column` its record of the nose's
/// column, `split` the flight's split, and `openrocket_m` OpenRocket's apogee.
///
/// # Errors
///
/// HPR Sim's own flight failing here, when the report has already flown it; a record that is not
/// `nose_columns.py`'s; a nose OpenRocket read as another shape; or a line and a rest that don't
/// sum to the drag part.
pub(crate) fn flight(
    build: &dyn Fn(Air) -> Result<Simulation, String>,
    climbed_m: &dyn Fn(&Simulation) -> Result<f64, String>,
    histories: &Value,
    column: Option<&Value>,
    split: &Result<SplitFlight, NotSplit>,
    openrocket_m: f64,
) -> Result<Option<NoseFlight>, String> {
    let own = build(Air::Day)?;
    let Some((nose_id, noses)) = tangent_ogive_nose(&own.assembly().layout) else {
        return Ok(None);
    };
    let top_mach = max_mach(&own)?;
    if !in_line(top_mach, &own.assembly().layout) {
        return Ok(None);
    }
    let skipped = |name: &str| {
        let index = SKIPPED
            .iter()
            .position(|(reason, _)| *reason == name)
            .ok_or_else(|| format!("{name} is not a reason a flight of the line is skipped"))?;
        Ok(Some(NoseFlight::Skipped(index)))
    };
    if noses > 1 {
        return skipped("several_noses");
    }
    let Ok(split) = split else {
        return skipped("not_split");
    };
    let column = column.ok_or_else(|| {
        format!("{COLLECTION_RECORD} doesn't hold the flight; run {ORACLE} again")
    })?;
    let Ok(column) = NoseColumn::of(column)? else {
        return skipped("no_column");
    };
    if !column.is_tangent_ogive() {
        return Err(format!(
            "HPR Sim reads the nose as a tangent ogive and OpenRocket as {:?}",
            column.shape
        ));
    }
    if stated(&own, nose_id) {
        return skipped("stated");
    }
    // The split flew these histories, so they are there.
    let Ok(inputs) = OpenRocketInputs::of(histories)? else {
        return Err("a flight that is split has no histories".to_owned());
    };
    let swapped = || -> Result<f64, String> {
        let simulation = build(Air::Day)?
            .with_thrust_history(inputs.thrust.clone())
            .map_err(|error| error.to_string())?
            .with_mass_history(inputs.mass.clone())
            .map_err(|error| error.to_string())?
            .with_drag_model(NoseSwap::new(nose_id, column.clone()));
        climbed_m(&simulation)
    };
    let Ok(nose_m) = swapped() else {
        return skipped("refused");
    };
    let apogees = split.apogees;
    let share = |before_m: f64, after_m: f64| 100.0 * (before_m - after_m) / openrocket_m;
    let (nose, rest) = (share(apogees.mass_m, nose_m), share(nose_m, apogees.drag_m));
    let drag = split.parts.drag;
    let sum = nose + rest;
    if !sum.is_finite() || (sum - drag).abs() > SUM_TOLERANCE_PERCENT {
        return Err(format!(
            "the nose's line and the rest sum to {sum}%, not the drag part's {drag}%"
        ));
    }
    Ok(Some(NoseFlight::Swapped {
        nose,
        rest,
        drag,
        top_mach,
    }))
}

/// The collection's record of nose columns, checked against the scripts and conditions that
/// wrote it: its flights by id, each as [`NoseColumn::of`] reads it.
pub(crate) fn collection_columns(
    root: &Path,
    conditions: &str,
) -> Result<BTreeMap<String, Value>, String> {
    let rerun = format!("run `refs/venv/bin/python {ORACLE} --collection {COLLECTION_RECORD}`");
    let record = crate::ork_flights::read_json(&root.join(COLLECTION_RECORD))
        .map_err(|error| format!("{error}; {rerun}"))?;
    let stale = |what: &str| format!("{COLLECTION_RECORD} was written from other {what}; {rerun}");
    let inputs = record["inputs_sha256"]
        .as_object()
        .filter(|inputs| inputs.contains_key(ORACLE))
        .ok_or_else(|| format!("{COLLECTION_RECORD} has no inputs_sha256 for {ORACLE}"))?;
    for (path, recorded) in inputs {
        let bytes = std::fs::read(root.join(path)).map_err(|error| format!("{path}: {error}"))?;
        if recorded.as_str() != Some(crate::ork_supply::sha256(&bytes).as_str()) {
            return Err(stale(&format!("scripts ({path} differs)")));
        }
    }
    if record["conditions_sha256"].as_str()
        != Some(crate::ork_supply::sha256(conditions.as_bytes()).as_str())
    {
        return Err(stale("conditions"));
    }
    check_record(&record, COLLECTION_RECORD)?;
    let flights = record["flights"]
        .as_array()
        .ok_or_else(|| format!("{COLLECTION_RECORD} has no flights"))?;
    let mut by_id = BTreeMap::new();
    for flight in flights {
        let id = flight["flight_id"]
            .as_str()
            .ok_or_else(|| format!("{COLLECTION_RECORD}: a flight without an id"))?;
        if by_id.insert(id.to_owned(), flight.clone()).is_some() {
            return Err(format!("{COLLECTION_RECORD} holds a flight twice"));
        }
    }
    Ok(by_id)
}

/// Holds a record to OpenRocket 24.12 and to the reasons this module knows.
fn check_record(record: &Value, path: &str) -> Result<(), String> {
    if !record["openrocket"]
        .as_str()
        .is_some_and(|v| v.starts_with("24.12"))
    {
        return Err(format!("{path} is not OpenRocket 24.12's"));
    }
    let mut named: Vec<&str> = record["reasons"]
        .as_object()
        .ok_or_else(|| format!("{path} lists no reasons"))?
        .keys()
        .map(String::as_str)
        .collect();
    let mut expected = REASONS.to_vec();
    named.sort_unstable();
    expected.sort_unstable();
    if named != expected {
        return Err(format!(
            "{path}'s reasons are {named:?}, not the {expected:?} this module knows"
        ));
    }
    Ok(())
}

/// OpenRocket's column for the nose of each motor configuration of the design library, which
/// `nose_columns.py --public` writes from the library's designs (private, ADR-151).
pub(crate) const LIBRARY_RECORD: &str = "corpus-out/openrocket-library-nose-columns.json";

/// Why a flight of the design library's report in the line is not swapped, in the order they
/// are tried, with the report's words.
pub(crate) const LIBRARY_SKIPPED: [(&str, &str); 5] = [
    SKIPPED[0],
    (
        "separates",
        "the flight separates, and the column is of the nose on one airframe",
    ),
    SKIPPED[2],
    SKIPPED[3],
    SKIPPED[4],
];

/// The key under which [`crate::ork_flights::fly_design`]'s flight holds what
/// [`library_flight`] reads of HPR Sim's own flight ([`own_facts`]); removed before the flight is
/// written.
pub(crate) const FACTS: &str = "nose_line_facts";

/// What the rule for the line's flights reads of HPR Sim's own flight `simulation`, whose top
/// Mach number is `top_mach` (`None` when one along it is not a number): that number, and the
/// nose at the front of the airframe when it is a tangent ogive, with its id, how many nose cones
/// the layout holds and whether the design states the nose's drag.
pub(crate) fn own_facts(simulation: &Simulation, top_mach: Option<f64>) -> Value {
    let nose = tangent_ogive_nose(&simulation.assembly().layout).map(|(id, noses)| {
        serde_json::json!({ "id": id, "noses": noses, "stated": stated(simulation, id) })
    });
    serde_json::json!({ "top_mach": top_mach, "nose": nose })
}

/// The design library's columns, by the SHA-256 of the design: each design's flights as
/// `nose_columns.py --public` writes them.
#[derive(Debug, Clone, Default)]
pub(crate) struct LibraryColumns {
    by_sha: BTreeMap<String, Vec<Value>>,
}

impl LibraryColumns {
    /// The record at [`LIBRARY_RECORD`], checked against the scripts that wrote it and against
    /// the jar the library's flights were written with (`jar_sha256`).
    ///
    /// # Errors
    ///
    /// A record that is missing, was written by other scripts or another jar, or isn't
    /// `nose_columns.py`'s.
    pub(crate) fn load(root: &Path, jar_sha256: &Value) -> Result<Self, String> {
        let rerun = format!(
            "run `refs/venv/bin/python {ORACLE} --public {LIBRARY_RECORD}` with every library \
             design OpenRocket flew"
        );
        let record = crate::ork_flights::read_json(&root.join(LIBRARY_RECORD))
            .map_err(|error| format!("{error}; {rerun}"))?;
        let inputs = record["inputs_sha256"]
            .as_object()
            .filter(|inputs| inputs.contains_key(ORACLE))
            .ok_or_else(|| format!("{LIBRARY_RECORD} has no inputs_sha256 for {ORACLE}"))?;
        for (path, recorded) in inputs {
            let bytes =
                std::fs::read(root.join(path)).map_err(|error| format!("{path}: {error}"))?;
            if recorded.as_str() != Some(crate::ork_supply::sha256(&bytes).as_str()) {
                return Err(format!(
                    "{LIBRARY_RECORD} was written from other scripts ({path} differs); {rerun}"
                ));
            }
        }
        if record["jar_sha256"] != *jar_sha256 {
            return Err(format!(
                "{LIBRARY_RECORD} was written with another jar than the flights; {rerun}"
            ));
        }
        check_record(&record, LIBRARY_RECORD)?;
        let mut by_sha = BTreeMap::new();
        for design in record["designs"]
            .as_array()
            .ok_or_else(|| format!("{LIBRARY_RECORD} has no designs"))?
        {
            let sha = design["sha256"]
                .as_str()
                .ok_or_else(|| format!("{LIBRARY_RECORD}: a design without its SHA-256"))?;
            let flights = design["flights"]
                .as_array()
                .ok_or_else(|| format!("{LIBRARY_RECORD}: a design without flights"))?;
            by_sha.insert(sha.to_owned(), flights.clone());
        }
        Ok(Self { by_sha })
    }

    /// The record's entry for the configuration `configuration` of the design whose SHA-256 is
    /// `sha`, matched as `fly_design` matches the flights' record: by id, or the design's only
    /// configuration when OpenRocket `renamed` it.
    ///
    /// # Errors
    ///
    /// The record holding no such design or configuration: it was written from other files.
    pub(crate) fn of(
        &self,
        sha: &str,
        configuration: &str,
        renamed: bool,
    ) -> Result<Value, String> {
        let rerun = format!("{LIBRARY_RECORD} doesn't hold the flight; run {ORACLE} again");
        let flights = self.by_sha.get(sha).ok_or_else(|| rerun.clone())?;
        let found = flights.iter().find(|flight| {
            flight["configuration"]
                .as_str()
                .is_some_and(|id| id.eq_ignore_ascii_case(configuration))
        });
        let only = (renamed && flights.len() == 1)
            .then(|| flights.first())
            .flatten();
        found.or(only).cloned().ok_or(rerun)
    }
}

/// A flight of the design library's report, as the line counts it, from what [`own_facts`]
/// read of HPR Sim's own flight: `None` when it is not one of the line's flights (its top Mach
/// number not above 1, or no tangent-ogive nose at the front of its airframe, as [`in_line`]
/// has it); else the reason it is skipped, the first of [`LIBRARY_SKIPPED`] that applies, or what
/// `fly` gives of it flown with the nose's column swapped in, on HPR Sim's own thrust and mass,
/// without and then with the whole base under power. `separates` says whether the flight
/// separates; `column` gives the record's entry for the flight.
///
/// The library has no record of OpenRocket's thrust and mass against time, so this is not the
/// nose's line of the fixture report ([`flight`]), which is taken on them.
///
/// # Errors
///
/// HPR Sim's own top Mach number not being a number; a record that is not `nose_columns.py`'s or
/// lacks the flight; or a nose OpenRocket read as another shape.
pub(crate) fn library_flight(
    facts: &Value,
    separates: bool,
    column: impl FnOnce() -> Result<Value, String>,
    fly: &mut dyn FnMut(NoseSwap, bool) -> Result<Value, String>,
) -> Result<Option<Result<[Value; 2], &'static str>>, String> {
    let top_mach = facts["top_mach"]
        .as_f64()
        .ok_or("a Mach number along HPR Sim's own flight is not a number")?;
    let nose = &facts["nose"];
    let Some(nose_id) = nose["id"].as_str() else {
        return Ok(None);
    };
    if top_mach.partial_cmp(&ABOVE_MACH) != Some(std::cmp::Ordering::Greater) {
        return Ok(None);
    }
    let skipped = |name: &str| {
        LIBRARY_SKIPPED
            .iter()
            .find(|(reason, _)| *reason == name)
            .map(|(reason, _)| Some(Err(*reason)))
            .ok_or_else(|| format!("{name} is not a reason a library flight is skipped"))
    };
    if nose["noses"].as_u64() != Some(1) {
        return skipped("several_noses");
    }
    if separates {
        return skipped("separates");
    }
    let Ok(column) = NoseColumn::of(&column()?)? else {
        return skipped("no_column");
    };
    if !column.is_tangent_ogive() {
        return Err(format!(
            "HPR Sim reads the nose as a tangent ogive and OpenRocket as {:?}",
            column.shape
        ));
    }
    if nose["stated"] != Value::Bool(false) {
        return skipped("stated");
    }
    let mut flown = Vec::with_capacity(2);
    for whole_base in [false, true] {
        match fly(NoseSwap::new(nose_id, column.clone()), whole_base) {
            Ok(probe) => flown.push(probe),
            Err(_) => return skipped("refused"),
        }
    }
    let [own, whole] = <[Value; 2]>::try_from(flown).map_err(|_| "two flights, not two")?;
    Ok(Some(Ok([own, whole])))
}

/// The report's lines on the nose: counts, and sizes over the flights swapped.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct NoseLine {
    /// The flights both fly whose own HPR Sim flight passes Mach 1 with a tangent-ogive nose.
    pub flights: usize,
    /// Those flown again with OpenRocket's column for the nose.
    pub swapped: usize,
    /// The rest, under each reason of [`SKIPPED`], in its order.
    pub skipped: Vec<Tally>,
    /// Over the flights swapped, when there are [`FLOOR`] or more: the nose's line.
    pub nose: Option<Size>,
    /// The rest of the drag part on the same flights.
    pub rest: Option<Size>,
    /// The drag part on the same flights: the two above summed.
    pub drag: Option<Size>,
    /// The mean, over the same flights, of the top Mach number of HPR Sim's own flight, to two
    /// decimals: how far past Mach 1 the line's flights go.
    pub hpr_mean_top_mach: Option<f64>,
    /// How many of the same flights pass Mach 1.3 in HPR Sim's own flight, from which the public
    /// probe shows OpenRocket giving the nose almost no pressure drag ([`PROBE_FROM_MACH`]).
    pub hpr_top_mach_over_1_3: Option<usize>,
}

impl NoseLine {
    /// The lines from each flight of the line.
    pub(crate) fn of(flights: &[NoseFlight]) -> Self {
        let swapped: Vec<(f64, f64, f64, f64)> = flights
            .iter()
            .filter_map(|flight| match flight {
                NoseFlight::Swapped {
                    nose,
                    rest,
                    drag,
                    top_mach,
                } => Some((*nose, *rest, *drag, *top_mach)),
                NoseFlight::Skipped(_) => None,
            })
            .collect();
        let published = swapped.len() >= FLOOR;
        let over = |of: fn(&(f64, f64, f64, f64)) -> f64| {
            published.then(|| Size::of(swapped.iter().map(of)))
        };
        let hpr_mean_top_mach = published.then(|| {
            let mean = swapped.iter().fold(0.0, |sum, s| sum + s.3) / swapped.len() as f64;
            (mean * 100.0).round() / 100.0
        });
        Self {
            flights: flights.len(),
            swapped: swapped.len(),
            skipped: SKIPPED
                .iter()
                .enumerate()
                .map(|(i, (name, _))| Tally {
                    reason: (*name).to_owned(),
                    flights: flights
                        .iter()
                        .filter(|flight| **flight == NoseFlight::Skipped(i))
                        .count(),
                })
                .collect(),
            nose: over(|s| s.0),
            rest: over(|s| s.1),
            drag: over(|s| s.2),
            hpr_mean_top_mach,
            hpr_top_mach_over_1_3: published
                .then(|| swapped.iter().filter(|s| s.3 > PROBE_FROM_MACH).count()),
        }
    }

    /// Holds the lines to themselves and to the `split` flights split: each flight of the line
    /// swapped or under a reason, no more swapped than split, the sizes and the mean top Mach
    /// number and the count past Mach 1.3 there exactly from [`FLOOR`] flights swapped, that count
    /// no more than the flights swapped, that mean no lower than the Mach 1
    /// each of its flights passes, and the nose's mean and the rest's summing to the drag's to
    /// the rounding of each.
    pub(crate) fn check_consistent(
        &self,
        flown_by_both: usize,
        split: usize,
    ) -> Result<(), String> {
        let fail = |why: String| Err(format!("the nose's line: {why}"));
        let reasons: Vec<&str> = self.skipped.iter().map(|t| t.reason.as_str()).collect();
        let expected: Vec<&str> = SKIPPED.iter().map(|(name, _)| *name).collect();
        if reasons != expected {
            return fail(format!("the reasons are {reasons:?}, not {expected:?}"));
        }
        // Saturating: a count read from a file can be any number.
        let skipped = self
            .skipped
            .iter()
            .fold(0_usize, |sum, t| sum.saturating_add(t.flights));
        if self.swapped.saturating_add(skipped) != self.flights {
            return fail(format!(
                "{} swapped and {skipped} skipped are not its {} flights",
                self.swapped, self.flights
            ));
        }
        if self.flights > flown_by_both || self.swapped > split {
            return fail(format!(
                "{} flights of {flown_by_both} both fly, {} swapped of {split} split",
                self.flights, self.swapped
            ));
        }
        let sizes = [
            ("nose", self.nose),
            ("rest of the drag", self.rest),
            ("drag", self.drag),
        ];
        let published = self.swapped >= FLOOR;
        if sizes.iter().any(|(_, size)| size.is_some() != published)
            || self.hpr_mean_top_mach.is_some() != published
            || self.hpr_top_mach_over_1_3.is_some() != published
        {
            return fail(format!(
                "{} flights swapped, and sizes {}: they are given from {FLOOR} flights",
                self.swapped,
                if published { "missing" } else { "given" }
            ));
        }
        for (what, size) in sizes {
            if let Some(size) = size {
                size.consistent(what)?;
            }
        }
        // Each flight's top Mach number is above 1, so their mean is; `>=` for its rounding.
        if self
            .hpr_mean_top_mach
            .is_some_and(|mach| !(mach.is_finite() && mach >= ABOVE_MACH))
        {
            return fail(format!(
                "a mean top Mach number of {:?} over flights that pass Mach {ABOVE_MACH}",
                self.hpr_mean_top_mach
            ));
        }
        if self
            .hpr_top_mach_over_1_3
            .is_some_and(|over| over > self.swapped)
        {
            return fail(format!(
                "{:?} of {} flights swapped pass Mach {PROBE_FROM_MACH}",
                self.hpr_top_mach_over_1_3, self.swapped
            ));
        }
        if let (Some(nose), Some(rest), Some(drag)) = (self.nose, self.rest, self.drag) {
            // Each mean is rounded to 0.005: three of them.
            let sum = nose.mean_percent + rest.mean_percent;
            if (sum - drag.mean_percent).abs() > 0.015 + 1e-9 {
                return fail(format!(
                    "the nose's mean and the rest's sum to {sum}, not the drag's {}",
                    drag.mean_percent
                ));
            }
        }
        Ok(())
    }

    /// The lines, for the end of the split's section.
    pub(crate) fn to_markdown(&self) -> String {
        let mut page = String::new();
        let flights = |n: usize| if n == 1 { "flight" } else { "flights" };
        let _ = writeln!(
            page,
            "\n### The tangent-ogive nose's line\n\nFor M2.3c13 (ADR-349, ADR-350), the drag part is \
             split once more. From Mach 1.3 up, OpenRocket 24.12 gives a tangent-ogive nose almost \
             no pressure drag, where HPR Sim gives it the drag of the cone of its length and \
             base; between Mach 1 and 1.3 each joins that to its own value at Mach 1 \
             (OpenRocket's issue #3391, fixed on its development line; no release carried the \
             fix by 2026-11-14). On \
             OpenRocket's thrust and mass, HPR Sim flies once more with its own pressure drag of \
             the nose replaced, above Mach 1, by the pressure drag OpenRocket's public interface \
             reports for the nose; everything else in the drag stays HPR Sim's. The nose's line \
             is the apogee before that swap less the apogee after it. The rest of the drag is \
             the apogee after it less the apogee on OpenRocket's whole drag, so the two sum to \
             the drag part, and the run stops if they don't on a flight.\n"
        );
        let _ = writeln!(
            page,
            "The line's flights are those whose own HPR Sim flight passes Mach 1 with a \
             tangent-ogive nose at the front of the airframe: {} of the flights both simulators \
             fly. Of those, {} {} swapped.\n",
            self.flights,
            self.swapped,
            if self.swapped == 1 { "is" } else { "are" },
        );
        if let (Some(nose), Some(rest), Some(drag)) = (self.nose, self.rest, self.drag) {
            let _ = writeln!(
                page,
                "| over the {} {} swapped | mean | median | mean absolute |\n|---|---:|---:|---:|",
                self.swapped,
                flights(self.swapped)
            );
            for (name, size) in [
                ("the nose's pressure drag above Mach 1", nose),
                ("the rest of the drag", rest),
                ("the drag part", drag),
            ] {
                let _ = writeln!(
                    page,
                    "| {name} | {:+.2}% | {:+.2}% | {:.2}% |",
                    size.mean_percent, size.median_percent, size.mean_absolute_percent
                );
            }
            if let (Some(mach), Some(over)) = (self.hpr_mean_top_mach, self.hpr_top_mach_over_1_3) {
                let _ = writeln!(
                    page,
                    "\nEach is per cent of OpenRocket's apogee. The top Mach numbers of these \
                     flights, by HPR Sim's own flights, average {mach:.2}, and {over} of the {} \
                     {} Mach {PROBE_FROM_MACH}, from which the public probe shows OpenRocket \
                     giving the nose almost no pressure drag. The line is measured that far \
                     past Mach 1, and says nothing of a faster flight.",
                    self.swapped,
                    if over == 1 { "passes" } else { "pass" },
                );
            }
        } else {
            let _ = writeln!(
                page,
                "The line's sizes are not given: over fewer than {FLOOR} flights an aggregate is \
                 a flight's own number, and the flights are private."
            );
        }
        let _ = writeln!(
            page,
            "\nA positive line means HPR Sim climbs higher on its own nose drag than on \
             OpenRocket's, a negative one lower. A flight of the line that is not swapped is \
             counted under the first reason that applies:\n"
        );
        let _ = writeln!(page, "| why a flight is skipped | flights |\n|---|---:|");
        for (tally, (_, words)) in self.skipped.iter().zip(SKIPPED) {
            let _ = writeln!(page, "| {words} | {} |", tally.flights);
        }
        page
    }
}

#[cfg(test)]
mod tests;
