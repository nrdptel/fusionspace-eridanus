//! HPR Sim against MAPLEAF on three public designs: a second open simulator to compare with
//! ([M1.14b8], [ADR-378]).
//!
//! [MAPLEAF] (MIT) is a six-degree-of-freedom rocket flight simulator with its own aerodynamics,
//! written apart from HPR Sim and from RocketPy. It is run as an outside program at a pinned
//! commit, in its own Python environment (`validation/oracles/mapleaf/`).
//!
//! - **The inputs** ([`inputs`], `validation/fixtures/flight/mapleaf-inputs.json`) are written
//!   from HPR Sim's own reading of each design, so MAPLEAF is handed what HPR Sim flies and not
//!   a second reading of the design file: the outside shape, the mass without propellant, the
//!   motor's thrust, propellant flow and propellant inertia through the burn, the travel on the
//!   rail and its direction, and the site.
//! - **MAPLEAF's flights** (`validation/oracles/mapleaf/flights.py`,
//!   `validation/fixtures/flight/mapleaf-flights.json`) carry the hash of the inputs they were
//!   flown from, and the mass, center of mass and inertia MAPLEAF's own log gave back.
//! - **The report** ([`run`], `validation/reports/mapleaf-flights.md`) flies HPR Sim on the
//!   same designs, with what MAPLEAF cannot fly turned off, and gives the differences in apogee
//!   and in rail-exit speed, with each input the two could not share beside them.
//!
//! Neither code is the truth, and nothing here is held to a tolerance: the differences are
//! reported, never gated. The report is held to the committed files it is written from.
//!
//! [M1.14b8]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b8
//! [ADR-378]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0378-mapleaf-as-a-second-open-oracle-flown-on-three-public-designs.md
//! [MAPLEAF]: https://github.com/henrystoldt/MAPLEAF

use std::fmt::Write as _;
use std::path::Path;

use hpr_aero::{DragConditions, DragTable, Flow};
use hpr_core::DVec3;
use hpr_core::geodesy::Geodetic;
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use hpr_design::{FinCrossSection, FinPlanform, MassProperties, NoseShape, Part, Rocket};
use hpr_sim::{
    Environment, EventKind, FlightMetrics, FlightSettings, FlightStep, Guides, Observer, Rail,
    Sample, SimError, Simulation,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ValidateError;
use crate::real_flight::sha256_hex;
use crate::report::same_but_for_platform_rounding;

/// The inputs MAPLEAF is handed, from the repository root.
pub const INPUTS_JSON: &str = "validation/fixtures/flight/mapleaf-inputs.json";
/// What MAPLEAF flew and found, from the repository root.
pub const FLIGHTS_JSON: &str = "validation/fixtures/flight/mapleaf-flights.json";
/// The report's data, from the repository root.
pub const REPORT_JSON: &str = "validation/reports/mapleaf-flights.json";
/// The report's page, from the repository root.
pub const REPORT_MD: &str = "validation/reports/mapleaf-flights.md";
/// The MAPLEAF commit the oracle environment installs (`validation/oracles/mapleaf/uv.lock`).
pub const MAPLEAF_COMMIT: &str = "af970d3e8200832f5e70d537b15ad38dd74fa551";
/// The public designs flown, by the name after `rocketpy-` in `validation/designs/`.
///
/// The three RocketPy examples with a tangent-ogive nose, which is the one curved nose MAPLEAF
/// has: the other examples' noses are von Kármán or other shapes it could only fly as something
/// else.
pub const CASES: [&str; 3] = ["valetudo", "bella-lui", "ndrt-2020-nose-to-tail"];

/// Where each design's rail and site come from: RocketPy's examples, as its whole-flight fixture
/// records them.
const RAILS_JSON: &str = "validation/fixtures/flight/rocketpy-whole-flight.json";
/// The designs' one configuration.
const CONFIGURATION: &str = "example";
/// The rows written for each interval of the thrust curve, so the propellant's inertia, which
/// MAPLEAF joins with straight lines, follows HPR Sim's between the curve's points.
const ROWS_PER_INTERVAL: u32 = 10;
/// How long after burnout each code's drag coefficient is read, s.
const AFTER_BURNOUT_S: f64 = 1e-3;
/// The spacing of the shared drag table's Mach numbers.
const DRAG_MACH_STEP: f64 = 0.02;
/// Its rows above Mach 0: to Mach 1, above every one of these flights.
const DRAG_MACH_ROWS: u32 = 50;
/// The decision record's file, under `docs/decisions/`.
const ADR_FILE: &str = "0378-mapleaf-as-a-second-open-oracle-flown-on-three-public-designs.md";
/// How closely MAPLEAF's logged mass properties must give back the inputs', as a fraction of the
/// value (of the rocket's length, for a center of mass): `flights.py` refuses a run outside it
/// and [`run`] checks the committed flights again.
const MASS_TOLERANCE: f64 = 1e-6;

/// The inputs file: what HPR Sim flies of each design, for MAPLEAF to fly too.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inputs {
    /// The command that writes the file.
    pub generator: String,
    /// The MAPLEAF commit the inputs are for ([`MAPLEAF_COMMIT`]).
    pub mapleaf_commit: String,
    /// What the file holds, in a sentence.
    pub what: String,
    /// One per design, in [`CASES`]' order.
    pub cases: Vec<InputCase>,
}

/// One design as both codes fly it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputCase {
    /// The case's name ([`CASES`]).
    pub name: String,
    /// The design file, from the repository root.
    pub design: String,
    /// The design's configuration.
    pub configuration: String,
    /// The rocket's length, m.
    pub length_m: f64,
    /// HPR Sim's reference diameter, m.
    pub reference_diameter_m: f64,
    /// The airframe from the nose back.
    pub body: Vec<BodyPart>,
    /// The one fin set.
    pub fins: Fins,
    /// The design's parts that are not handed over, each as `id (kind)`: MAPLEAF has no such
    /// part, and their mass is inside [`Self::dry`].
    pub not_passed: Vec<String>,
    /// The rocket with its propellant gone.
    pub dry: MassPoint,
    /// The motor through its burn.
    pub motor: InputMotor,
    /// The whole rocket at ignition, as HPR Sim has it: what MAPLEAF's log must give back.
    pub ignition: MassPoint,
    /// The whole rocket at burnout, likewise.
    pub burnout: MassPoint,
    /// A drag both codes can fly, for the flights that share it.
    pub drag: SharedDrag,
    /// HPR Sim's gravity at the pad, m/s², at the height the center of mass stands at on it
    /// (the tail on the ground): what MAPLEAF's is matched to in the flights that measure what
    /// the two gravity models' difference moves.
    pub gravity_at_pad_m_s2: f64,
    /// The rail.
    pub rail: InputRail,
    /// The launch site.
    pub site: InputSite,
}

/// A piece of the airframe.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
    /// The component's id in the design.
    pub id: String,
    /// What it is.
    pub kind: BodyKind,
    /// A nose's shape; `None` for the rest.
    pub shape: Option<NoseKind>,
    /// Station of its forward end, m aft of the nose tip.
    pub fore_station_m: f64,
    /// Its length, m.
    pub length_m: f64,
    /// Its diameter at the forward end, m (0 for a nose).
    pub fore_diameter_m: f64,
    /// Its diameter at the aft end, m.
    pub aft_diameter_m: f64,
    /// Its finish's roughness height, m.
    pub roughness_m: f64,
}

/// The kinds of airframe piece MAPLEAF has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyKind {
    /// A nose cone.
    Nose,
    /// A body tube.
    Tube,
    /// A conical change of diameter.
    Transition,
}

/// The nose shapes MAPLEAF has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoseKind {
    /// A tangent ogive.
    TangentOgive,
    /// A cone.
    Cone,
}

/// A set of trapezoidal fins with square edges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fins {
    /// The component's id in the design.
    pub id: String,
    /// How many.
    pub count: u32,
    /// Station of the root chord's leading edge, m aft of the nose tip.
    pub fore_station_m: f64,
    /// Root chord, m.
    pub root_chord_m: f64,
    /// Tip chord, m.
    pub tip_chord_m: f64,
    /// Span of one fin, m.
    pub span_m: f64,
    /// How far aft the tip's leading edge is of the root's, m.
    pub sweep_m: f64,
    /// Thickness, m.
    pub thickness_m: f64,
    /// The finish's roughness height, m.
    pub roughness_m: f64,
    /// The edges' shape: `square`, the only one handed over.
    pub edges: String,
}

/// One table of drag coefficient against Mach number for both codes to fly, motor on or off.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedDrag {
    /// What the table is, in a sentence.
    pub what: String,
    /// The diameter of the reference area the coefficients are on, m.
    pub reference_diameter_m: f64,
    /// The Mach numbers, rising from zero.
    pub mach: Vec<f64>,
    /// The drag coefficient at each; straight lines between.
    pub drag_coefficient: Vec<f64>,
}

/// A mass with its center on the axis and its inertia about that center.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MassPoint {
    /// Mass, kg.
    pub mass_kg: f64,
    /// Station of the center of mass, m aft of the nose tip.
    pub cg_station_m: f64,
    /// Moment of inertia about an axis through the center, across the rocket's, kg m².
    pub transverse_inertia_kg_m2: f64,
    /// Moment of inertia about the rocket's axis, kg m².
    pub axial_inertia_kg_m2: f64,
}

/// The motor through its burn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputMotor {
    /// The motor's designation in the design.
    pub designation: String,
    /// The thrust curve's total impulse, N s.
    pub total_impulse_ns: f64,
    /// The nozzle exit, m aft of the nose tip: with the propellant's center, the lever of the
    /// internal-momentum terms HPR Sim's equations of motion carry (`docs/physics/flight.md`).
    pub nozzle_station_m: f64,
    /// What each number of a row is, in order ([`MOTOR_COLUMNS`]).
    pub columns: Vec<String>,
    /// The burn, from ignition to the curve's end: one row per time, its numbers in
    /// [`Self::columns`]' order ([`MotorRow::values`]).
    pub rows: Vec<[f64; 7]>,
}

/// The columns of [`InputMotor::rows`]: the fields of [`MotorRow`], in order.
pub const MOTOR_COLUMNS: [&str; 7] = [
    "time_s",
    "thrust_n",
    "mass_flow_kg_s",
    "propellant_mass_kg",
    "propellant_cg_station_m",
    "propellant_transverse_inertia_kg_m2",
    "propellant_axial_inertia_kg_m2",
];

/// The motor at one time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorRow {
    /// Time since ignition, s.
    pub time_s: f64,
    /// Thrust, N.
    pub thrust_n: f64,
    /// Propellant mass flow, kg/s.
    pub mass_flow_kg_s: f64,
    /// Propellant left, kg.
    pub propellant_mass_kg: f64,
    /// Station of the propellant's center, m aft of the nose tip.
    pub propellant_cg_station_m: f64,
    /// The propellant's inertia about its own center, across the axis, kg m².
    pub propellant_transverse_inertia_kg_m2: f64,
    /// The propellant's inertia about the axis, kg m².
    pub propellant_axial_inertia_kg_m2: f64,
}

impl MotorRow {
    /// The row as [`InputMotor::rows`] holds it, in [`MOTOR_COLUMNS`]' order.
    #[must_use]
    pub fn values(&self) -> [f64; 7] {
        [
            self.time_s,
            self.thrust_n,
            self.mass_flow_kg_s,
            self.propellant_mass_kg,
            self.propellant_cg_station_m,
            self.propellant_transverse_inertia_kg_m2,
            self.propellant_axial_inertia_kg_m2,
        ]
    }
}

/// The rail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputRail {
    /// The rail's length, m, as HPR Sim flies it.
    pub rail_length_m: f64,
    /// How far HPR Sim's rocket travels before its last guide leaves the rail, m
    /// ([`Guides::exit_travel_m`]): MAPLEAF's rail has no guides and lets go once the center of
    /// mass has traveled its length, so this is the length it is given.
    pub guided_travel_m: f64,
    /// The rail's angle above the horizon as both fly it, degrees: 90, straight up
    /// ([`RAIL_INCLINATION_DEG`]).
    pub inclination_deg: f64,
    /// The angle the example gives its rail, degrees, which is not flown: MAPLEAF at the pinned
    /// commit does not hold a rocket to a leaning rail's direction (`flights.py` measures it).
    pub example_inclination_deg: f64,
    /// The example's heading, clockwise from north, degrees: nothing to a rail that stands
    /// straight up, and what the leaning-rail measurement uses.
    pub heading_deg: f64,
    /// Where the length and the example's angles come from.
    pub source: String,
}

/// The rail's angle above the horizon in every case, degrees: straight up, the one direction
/// both codes hold a rocket to.
pub const RAIL_INCLINATION_DEG: f64 = 90.0;

/// The launch site.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InputSite {
    /// Latitude, degrees.
    pub latitude_deg: f64,
    /// Longitude, degrees.
    pub longitude_deg: f64,
    /// Elevation, m.
    pub elevation_m: f64,
}

fn refuse(what: impl Into<String>) -> ValidateError {
    ValidateError::Case(what.into())
}

fn read(root: &Path, file: &str, what: &'static str) -> Result<String, ValidateError> {
    let path = root.join(file);
    std::fs::read_to_string(&path).map_err(|source| ValidateError::Io {
        what,
        path: path.display().to_string(),
        source,
    })
}

fn parse<T: serde::de::DeserializeOwned>(file: &str, text: &str) -> Result<T, ValidateError> {
    serde_json::from_str(text).map_err(|source| ValidateError::Json {
        path: file.to_owned(),
        source: Box::new(source),
    })
}

fn design_file(name: &str) -> String {
    format!("validation/designs/rocketpy-{name}.json")
}

/// A rocket's mass properties as a point on the axis, refusing one that is not.
fn mass_point(name: &str, what: &str, mass: &MassProperties) -> Result<MassPoint, ValidateError> {
    let inertia = mass.inertia_kg_m2;
    if mass.cg_m.x != 0.0 || mass.cg_m.y != 0.0 || inertia.x_axis.x != inertia.y_axis.y {
        return Err(refuse(format!(
            "{name}: {what} is not on the axis and the same about both cross axes, which is all \
             MAPLEAF's point mass is handed"
        )));
    }
    Ok(MassPoint {
        mass_kg: mass.mass_kg,
        cg_station_m: -mass.cg_m.z,
        transverse_inertia_kg_m2: inertia.x_axis.x,
        axial_inertia_kg_m2: inertia.z_axis.z,
    })
}

/// The inputs for every case, from the designs as HPR Sim reads them.
///
/// # Errors
///
/// [`ValidateError`] for a file that cannot be read, or a design with something MAPLEAF cannot be
/// handed: a nose that is no tangent ogive or cone, a curved transition, fins that are not one
/// trapezoidal set with square edges and no cant, or a mass off the axis.
pub fn inputs(root: &Path) -> Result<Inputs, ValidateError> {
    let rails: Value = parse(RAILS_JSON, &read(root, RAILS_JSON, "reading the rails")?)?;
    let cases = CASES
        .iter()
        .map(|name| input_case(root, name, &rails))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Inputs {
        generator: "cargo xtask mapleaf-flights --inputs".to_owned(),
        mapleaf_commit: MAPLEAF_COMMIT.to_owned(),
        what: "what HPR Sim flies of each public design, for MAPLEAF to fly too: stations are \
               meters aft of the nose tip, inertias are about each mass's own center"
            .to_owned(),
        cases,
    })
}

fn input_case(root: &Path, name: &str, rails: &Value) -> Result<InputCase, ValidateError> {
    let design = design_file(name);
    let rocket: Rocket = parse(&design, &read(root, &design, "reading a design")?)?;
    let assembly = rocket
        .assemble(CONFIGURATION)
        .map_err(|error| refuse(format!("{name}: {error}")))?;
    let layout = &assembly.layout;

    let mut body = Vec::new();
    for component in layout.body() {
        let roughness_m = component
            .finish
            .roughness_m()
            .map_err(|error| refuse(format!("{name}: {error}")))?;
        let part = match &component.part {
            Part::NoseCone(nose) => BodyPart {
                id: component.id.clone(),
                kind: BodyKind::Nose,
                shape: Some(match nose.shape {
                    NoseShape::Ogive { radius_ratio: 1.0 } => NoseKind::TangentOgive,
                    NoseShape::Conical {} => NoseKind::Cone,
                    other => {
                        return Err(refuse(format!(
                            "{name}: MAPLEAF has no nose shaped {other:?}"
                        )));
                    }
                }),
                fore_station_m: component.fore_station_m,
                length_m: nose.length_m,
                fore_diameter_m: 0.0,
                aft_diameter_m: 2.0 * nose.base_radius_m,
                roughness_m,
            },
            Part::BodyTube(tube) => BodyPart {
                id: component.id.clone(),
                kind: BodyKind::Tube,
                shape: None,
                fore_station_m: component.fore_station_m,
                length_m: tube.length_m,
                fore_diameter_m: 2.0 * tube.outer_radius_m,
                aft_diameter_m: 2.0 * tube.outer_radius_m,
                roughness_m,
            },
            Part::Transition(transition) => {
                if !matches!(transition.shape, NoseShape::Conical {}) {
                    return Err(refuse(format!(
                        "{name}: MAPLEAF's transitions are conical, and {} is not",
                        component.id
                    )));
                }
                BodyPart {
                    id: component.id.clone(),
                    kind: BodyKind::Transition,
                    shape: None,
                    fore_station_m: component.fore_station_m,
                    length_m: transition.length_m,
                    fore_diameter_m: 2.0 * transition.fore_radius_m,
                    aft_diameter_m: 2.0 * transition.aft_radius_m,
                    roughness_m,
                }
            }
            other => {
                return Err(refuse(format!(
                    "{name}: {} is a body part MAPLEAF cannot be handed ({other:?})",
                    component.id
                )));
            }
        };
        body.push(part);
    }

    let mut fins = None;
    let mut not_passed = Vec::new();
    for component in layout.components.iter().filter(|c| c.parent.is_some()) {
        match &component.part {
            Part::FinSet(set) => {
                let FinPlanform::Trapezoidal {
                    root_chord_m,
                    tip_chord_m,
                    span_m,
                    sweep_m,
                } = set.planform
                else {
                    return Err(refuse(format!(
                        "{name}: {} is not trapezoidal",
                        component.id
                    )));
                };
                if set.cross_section != FinCrossSection::Square
                    || set.cant_rad != 0.0
                    || set.tab.is_some()
                    || fins.is_some()
                {
                    return Err(refuse(format!(
                        "{name}: only one set of uncanted, square-edged fins with no tab is \
                         handed over, and {} is not that",
                        component.id
                    )));
                }
                fins = Some(Fins {
                    id: component.id.clone(),
                    count: set.count,
                    fore_station_m: component.fore_station_m,
                    root_chord_m,
                    tip_chord_m,
                    span_m,
                    sweep_m,
                    thickness_m: set.thickness_m,
                    roughness_m: component
                        .finish
                        .roughness_m()
                        .map_err(|error| refuse(format!("{name}: {error}")))?,
                    edges: "square".to_owned(),
                });
            }
            Part::InnerTube(_) => not_passed.push(format!("{} (inner tube)", component.id)),
            Part::RailButton(_) => not_passed.push(format!("{} (rail buttons)", component.id)),
            other => {
                return Err(refuse(format!(
                    "{name}: {} is a part this comparison has no rule for ({other:?})",
                    component.id
                )));
            }
        }
    }
    let fins = fins.ok_or_else(|| refuse(format!("{name}: no fins")))?;

    let [placed] = &assembly.motors[..] else {
        return Err(refuse(format!(
            "{name}: one motor is flown, and it has {}",
            assembly.motors.len()
        )));
    };
    let motor = &placed.mounted.motor;
    let nozzle_station_m = placed.nozzle_station_m();
    let times = motor.curve().times_s();
    let mut instants = Vec::new();
    for pair in times.windows(2) {
        for step in 0..ROWS_PER_INTERVAL {
            // The interval's own points exactly, and even steps between them.
            let fraction = f64::from(step) / f64::from(ROWS_PER_INTERVAL);
            instants.push(pair[0] + (pair[1] - pair[0]) * fraction);
        }
    }
    let burn_out_s = *times
        .last()
        .ok_or_else(|| refuse(format!("{name}: an empty thrust curve")))?;
    instants.push(burn_out_s);
    let mut rows: Vec<MotorRow> = Vec::with_capacity(instants.len());
    for time_s in instants {
        let state = motor.state(time_s);
        let propellant = state.propellant;
        // With no propellant left its center is nowhere: the row keeps the one before's.
        let propellant_cg_station_m = if propellant.mass_kg > 0.0 {
            nozzle_station_m - propellant.cg_m
        } else {
            rows.last()
                .map_or(nozzle_station_m, |row| row.propellant_cg_station_m)
        };
        rows.push(MotorRow {
            time_s,
            thrust_n: state.thrust_n,
            mass_flow_kg_s: state.mass_flow_kg_s,
            propellant_mass_kg: propellant.mass_kg.max(0.0),
            propellant_cg_station_m,
            propellant_transverse_inertia_kg_m2: propellant.transverse_inertia_kg_m2.max(0.0),
            propellant_axial_inertia_kg_m2: propellant.axial_inertia_kg_m2.max(0.0),
        });
    }

    let found = rails
        .get("cases")
        .and_then(Value::as_array)
        .and_then(|cases| {
            cases
                .iter()
                .find(|case| case.get("name").and_then(Value::as_str) == Some(name))
        })
        .ok_or_else(|| refuse(format!("{name}: {RAILS_JSON} has no such case")))?;
    let number = |part: &str, key: &str| {
        found
            .get(part)
            .and_then(|part| part.get(key))
            .and_then(Value::as_f64)
            .ok_or_else(|| refuse(format!("{name}: {RAILS_JSON} gives no {part}.{key}")))
    };
    let rail_length_m = number("rail", "rail_length_m")?;
    if found.get("design").and_then(Value::as_str) != Some(design.as_str()) {
        return Err(refuse(format!(
            "{name}: {RAILS_JSON}'s case flew another design"
        )));
    }

    let rail = InputRail {
        rail_length_m,
        guided_travel_m: Guides::of(&assembly).exit_travel_m(rail_length_m),
        inclination_deg: RAIL_INCLINATION_DEG,
        example_inclination_deg: number("rail", "inclination_deg")?,
        heading_deg: number("rail", "heading_deg")?,
        source: format!(
            "{RAILS_JSON}, case {name}: {}",
            found
                .get("rail")
                .and_then(|rail| rail.get("source"))
                .and_then(Value::as_str)
                .unwrap_or("RocketPy's example")
        ),
    };
    let site = InputSite {
        latitude_deg: number("environment", "latitude_deg")?,
        longitude_deg: number("environment", "longitude_deg")?,
        elevation_m: number("environment", "elevation_m")?,
    };
    let ignition = mass_point(
        name,
        "the rocket at ignition",
        &assembly.mass_properties(0.0),
    )?;
    let simulation = simulation(&rocket, name, &site, &rail)?;
    let drag = shared_drag(&simulation, name, &site, layout.reference_diameter_m)?;
    let gravity_at_pad_m_s2 = simulation
        .environment()
        .earth
        .gravity_enu_mps2(DVec3::new(
            0.0,
            0.0,
            layout.length_m - ignition.cg_station_m,
        ))
        .map_err(|error| refuse(format!("{name}: {error}")))?
        .length();

    Ok(InputCase {
        name: name.to_owned(),
        design,
        configuration: CONFIGURATION.to_owned(),
        length_m: layout.length_m,
        reference_diameter_m: layout.reference_diameter_m,
        body,
        fins,
        not_passed,
        dry: mass_point(name, "the dry rocket", &assembly.dry_mass_properties())?,
        motor: InputMotor {
            designation: placed.mounted.designation.clone(),
            total_impulse_ns: motor.curve().total_impulse_ns(),
            nozzle_station_m,
            columns: MOTOR_COLUMNS
                .iter()
                .map(|&column| column.to_owned())
                .collect(),
            rows: rows.iter().map(MotorRow::values).collect(),
        },
        ignition,
        burnout: mass_point(
            name,
            "the rocket at burnout",
            &assembly.mass_properties(burn_out_s),
        )?,
        drag,
        gravity_at_pad_m_s2,
        rail,
        site,
    })
}

/// What one code found on one design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flown {
    /// How far the center of mass rose from where it stood at ignition to its apogee, m.
    pub apogee_rise_m: f64,
    /// The airframe's speed where it has traveled the guided length, m/s: a point fixed in the
    /// body, not the center of mass, which moves forward in the body as the propellant burns.
    pub rail_exit_speed_m_s: f64,
    /// The largest Mach number.
    pub max_mach: f64,
    /// Gravity where the rocket stood at ignition, m/s².
    pub gravity_at_ignition_m_s2: f64,
    /// The air's density there, kg/m³.
    pub air_density_at_ignition_kg_m3: f64,
    /// The Mach number just after burnout (1 ms after, for HPR Sim; MAPLEAF's
    /// first evaluation after).
    pub after_burnout_mach: f64,
    /// The aerodynamic force along the axis there, as a coefficient on HPR Sim's reference area:
    /// each code's own drag, with the motor off.
    pub after_burnout_axial_coefficient: f64,
    /// That coefficient by part, under the design's ids: the base drag is with the last body
    /// part in both codes.
    pub after_burnout_parts: Vec<PartDrag>,
}

/// One part's share of a code's axial coefficient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartDrag {
    /// The component's id in the design.
    pub id: String,
    /// Its share of the axial coefficient, on HPR Sim's reference area.
    pub axial_coefficient: f64,
}

/// The two codes on one design with one drag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compared {
    /// HPR Sim's flight.
    pub hpr: Flown,
    /// MAPLEAF's, at its finer time step.
    pub mapleaf: Flown,
    /// MAPLEAF's apogee rise at its coarser step, m.
    pub mapleaf_coarser_apogee_rise_m: f64,
    /// Its rail-exit speed at its coarser step, m/s.
    pub mapleaf_coarser_rail_exit_speed_m_s: f64,
    /// HPR Sim's apogee rise less MAPLEAF's, m.
    pub apogee_difference_m: f64,
    /// That as a percentage of MAPLEAF's.
    pub apogee_difference_percent: f64,
    /// HPR Sim's rail-exit speed less MAPLEAF's, m/s.
    pub rail_exit_difference_m_s: f64,
    /// That as a percentage of MAPLEAF's.
    pub rail_exit_difference_percent: f64,
    /// HPR Sim's axial coefficient just after burnout less MAPLEAF's, as a percentage of
    /// MAPLEAF's.
    pub axial_coefficient_difference_percent: f64,
}

impl Compared {
    /// The two flights with their differences.
    fn of(
        hpr: Flown,
        mapleaf: Flown,
        mapleaf_coarser_apogee_rise_m: f64,
        mapleaf_coarser_rail_exit_speed_m_s: f64,
    ) -> Self {
        let apogee_difference_m = hpr.apogee_rise_m - mapleaf.apogee_rise_m;
        let rail_exit_difference_m_s = hpr.rail_exit_speed_m_s - mapleaf.rail_exit_speed_m_s;
        Self {
            mapleaf_coarser_apogee_rise_m,
            mapleaf_coarser_rail_exit_speed_m_s,
            apogee_difference_m,
            apogee_difference_percent: percent(apogee_difference_m, mapleaf.apogee_rise_m),
            rail_exit_difference_m_s,
            rail_exit_difference_percent: percent(
                rail_exit_difference_m_s,
                mapleaf.rail_exit_speed_m_s,
            ),
            axial_coefficient_difference_percent: percent(
                hpr.after_burnout_axial_coefficient - mapleaf.after_burnout_axial_coefficient,
                mapleaf.after_burnout_axial_coefficient,
            ),
            hpr,
            mapleaf,
        }
    }
}

/// MAPLEAF's same-drag flight flown again with its gravity constant scaled so that its gravity
/// at the pad is HPR Sim's: what the two gravity models' difference moves, measured. The second
/// such flight ([`FlightRow::momentum_matched`]) also adds HPR Sim's internal-momentum force to
/// MAPLEAF's thrust.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GravityMatched {
    /// The factor on MAPLEAF's gravity constant.
    pub mapleaf_gravity_scale: f64,
    /// MAPLEAF's gravity at ignition in this flight, m/s².
    pub mapleaf_gravity_at_ignition_m_s2: f64,
    /// MAPLEAF's apogee rise, m.
    pub mapleaf_apogee_rise_m: f64,
    /// MAPLEAF's rail-exit speed, m/s.
    pub mapleaf_rail_exit_speed_m_s: f64,
    /// MAPLEAF's apogee rise at its coarser time step, m.
    pub mapleaf_coarser_apogee_rise_m: f64,
    /// MAPLEAF's rail-exit speed at its coarser time step, m/s.
    pub mapleaf_coarser_rail_exit_speed_m_s: f64,
    /// HPR Sim's same-drag apogee rise less this one, m.
    pub apogee_difference_m: f64,
    /// That as a percentage of this one.
    pub apogee_difference_percent: f64,
    /// HPR Sim's same-drag rail-exit speed less this one, m/s.
    pub rail_exit_difference_m_s: f64,
    /// That as a percentage of this one.
    pub rail_exit_difference_percent: f64,
}

impl GravityMatched {
    /// The matched flight against HPR Sim's same-drag flight.
    fn of(
        hpr: &Flown,
        mapleaf_gravity_scale: f64,
        mapleaf_gravity_at_ignition_m_s2: f64,
        [mapleaf_apogee_rise_m, mapleaf_rail_exit_speed_m_s]: [f64; 2],
        [
            mapleaf_coarser_apogee_rise_m,
            mapleaf_coarser_rail_exit_speed_m_s,
        ]: [f64; 2],
    ) -> Self {
        let apogee_difference_m = hpr.apogee_rise_m - mapleaf_apogee_rise_m;
        let rail_exit_difference_m_s = hpr.rail_exit_speed_m_s - mapleaf_rail_exit_speed_m_s;
        Self {
            mapleaf_gravity_scale,
            mapleaf_gravity_at_ignition_m_s2,
            mapleaf_apogee_rise_m,
            mapleaf_rail_exit_speed_m_s,
            mapleaf_coarser_apogee_rise_m,
            mapleaf_coarser_rail_exit_speed_m_s,
            apogee_difference_m,
            apogee_difference_percent: percent(apogee_difference_m, mapleaf_apogee_rise_m),
            rail_exit_difference_m_s,
            rail_exit_difference_percent: percent(
                rail_exit_difference_m_s,
                mapleaf_rail_exit_speed_m_s,
            ),
        }
    }
}

/// One design, flown by both codes with the same drag and with each one's own.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightRow {
    /// The case's name.
    pub name: String,
    /// The design file.
    pub design: String,
    /// The motor's designation.
    pub motor: String,
    /// The mass at ignition both flew, kg.
    pub ignition_mass_kg: f64,
    /// The travel on the rail both were guided for, m.
    pub guided_travel_m: f64,
    /// The angle of the example's own rail from the horizontal, degrees; both codes fly
    /// [`RAIL_INCLINATION_DEG`] instead.
    pub example_inclination_deg: f64,
    /// Each code with its own aerodynamics.
    pub own_drag: Compared,
    /// Both codes on the inputs' drag table.
    pub same_drag: Compared,
    /// The same-drag flight again with MAPLEAF's gravity matched to HPR Sim's at the pad.
    pub gravity_matched: GravityMatched,
    /// That flight again with HPR Sim's internal-momentum force added to MAPLEAF's thrust:
    /// `(n − c) m̈`, the three terms HPR Sim's translational equation carries beside the thrust
    /// (`docs/physics/flight.md`), summed for a propellant center `c` fixed in the airframe.
    pub momentum_matched: GravityMatched,
    /// That force's lever, m: the nozzle exit's distance behind the propellant's center.
    pub internal_momentum_lever_m: f64,
    /// How far MAPLEAF's airframe had traveled when its own rail let go, m: its rail measures
    /// the center of mass's travel.
    pub mapleaf_release_body_travel_m: f64,
    /// The largest gap between HPR Sim's propellant center and the straight line MAPLEAF moves
    /// it on, m.
    pub propellant_center_line_departure_m: f64,
    /// The design's parts not handed to MAPLEAF.
    pub not_passed: Vec<String>,
}

/// An input the two codes could not share.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotShared {
    /// The input.
    pub input: String,
    /// The flights it differs in: every one, or only those with each code's own drag.
    pub flights: String,
    /// What HPR Sim flew.
    pub hpr: String,
    /// What MAPLEAF flew.
    pub mapleaf: String,
    /// What was measured of the difference, if anything was.
    pub measured: String,
}

/// What MAPLEAF does on a rail that leans, measured on one case: why every flight here leaves
/// a vertical rail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaningRail {
    /// The case flown.
    pub case: String,
    /// The rail's angle from the vertical, degrees: the example's own.
    pub rail_from_vertical_deg: f64,
    /// The angle from the vertical of the line the rocket travels while guided, degrees.
    pub travel_from_vertical_deg: f64,
    /// The angle of the rocket's axis from the vertical there, degrees.
    pub body_axis_from_vertical_deg: f64,
    /// MAPLEAF's own logged angle of attack there, degrees.
    pub angle_of_attack_deg: f64,
    /// The travel along the rail at which these were read, m.
    pub at_travel_m: f64,
}

/// One MAPLEAF setting left at its default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    /// The setting's name in a MAPLEAF simulation definition.
    pub name: String,
    /// Its value.
    pub value: String,
}

/// The report: `validation/reports/mapleaf-flights.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapleafReport {
    /// The command that writes it.
    pub generated_by: String,
    /// The oracle, as its fixture names it.
    pub oracle: String,
    /// The command that flew it.
    pub oracle_command: String,
    /// The inputs file, whose hash the oracle's flights carry.
    pub inputs: String,
    /// The inputs file's SHA-256.
    pub inputs_sha256: String,
    /// The oracle's flights file.
    pub flights: String,
    /// Its SHA-256.
    pub flights_sha256: String,
    /// MAPLEAF's two time steps, s.
    pub mapleaf_time_steps_s: Vec<f64>,
    /// The rail's angle from the horizontal in every flight, degrees.
    pub rail_inclination_deg: f64,
    /// What the same-drag flights' table is.
    pub shared_drag: String,
    /// The MAPLEAF settings left at their defaults that bear on the comparison.
    pub mapleaf_defaults_kept: Vec<Setting>,
    /// MAPLEAF on a leaning rail.
    pub leaning_rail: LeaningRail,
    /// One row per design.
    pub rows: Vec<FlightRow>,
    /// The inputs the two could not share.
    pub not_shared: Vec<NotShared>,
}

/// Reads the first sample a set time after ignition.
struct At {
    time_s: f64,
    found: Option<Sample>,
    max_mach: f64,
}

impl Observer for At {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let end = step.sample(step.end_s())?;
        self.max_mach = self.max_mach.max(end.mach);
        if self.found.is_none() && step.start_s() <= self.time_s && self.time_s <= step.end_s() {
            self.found = Some(step.sample(self.time_s)?);
        }
        Ok(())
    }
}

/// HPR Sim's flight of a design as MAPLEAF can fly it too: HPR Sim's own Earth and air (WGS 84
/// gravity, the 1976 standard atmosphere), calm, with no rail friction and no tip-off phase.
fn simulation(
    rocket: &Rocket,
    name: &str,
    site: &InputSite,
    rail: &InputRail,
) -> Result<Simulation, ValidateError> {
    let failed = |source: SimError| ValidateError::Flight {
        case: name.to_owned(),
        what: "building the flight".to_owned(),
        source: Some(Box::new(source)),
    };
    let site = Geodetic::from_degrees(site.latitude_deg, site.longitude_deg, site.elevation_m)
        .map_err(|error| refuse(format!("{name}: {error}")))?;
    Ok(Simulation::new(
        rocket,
        CONFIGURATION,
        Environment::standard(site).map_err(failed)?,
        Rail {
            length_m: rail.rail_length_m,
            azimuth_rad: rail.heading_deg.to_radians(),
            elevation_rad: rail.inclination_deg.to_radians(),
            roll_rad: 0.0,
            // MAPLEAF's rail has no friction.
            friction_coefficient: 0.0,
        },
        FlightSettings::default(),
    )
    .map_err(failed)?
    // MAPLEAF's rail holds the rocket straight until it lets go: no tip-off phase.
    .without_tip_off())
}

/// The drag both codes can fly: HPR Sim's own coasting drag at no angle of attack, in the air at
/// the site, at each Mach number of the table.
fn shared_drag(
    simulation: &Simulation,
    name: &str,
    site: &InputSite,
    reference_diameter_m: f64,
) -> Result<SharedDrag, ValidateError> {
    let air = simulation
        .environment()
        .atmosphere
        .air(site.elevation_m)
        .map_err(|error| refuse(format!("{name}: {error}")))?
        .air;
    let mut mach = vec![0.0];
    let mut drag_coefficient = Vec::new();
    for row in 1..=DRAG_MACH_ROWS {
        let at = f64::from(row) * DRAG_MACH_STEP;
        let reynolds_per_m =
            air.density_kg_m3 * at * air.speed_of_sound_m_s / air.dynamic_viscosity_pa_s;
        let drag = simulation
            .aero()
            .drag(
                &Flow::new(at, 0.0, 0.0),
                &DragConditions::coasting(reynolds_per_m),
            )
            .map_err(|error| refuse(format!("{name}: {error}")))?;
        mach.push(at);
        drag_coefficient.push(drag.axial_coefficient);
    }
    // At rest there is no Reynolds number to work a drag out at: the first row's value stands
    // from Mach 0.
    let slowest = drag_coefficient
        .first()
        .copied()
        .ok_or_else(|| refuse(format!("{name}: the drag table has no rows")))?;
    drag_coefficient.insert(0, slowest);
    Ok(SharedDrag {
        what: format!(
            "HPR Sim's own coasting drag at no angle of attack, in the standard atmosphere's \
             air at the site's elevation, every {DRAG_MACH_STEP} in Mach number to Mach {}, \
             with straight lines between and the first value held down to Mach 0",
            f64::from(DRAG_MACH_ROWS) * DRAG_MACH_STEP
        ),
        reference_diameter_m,
        mach,
        drag_coefficient,
    })
}

/// The shared drag as a table HPR Sim can fly.
fn drag_table(case: &InputCase) -> Result<DragTable, ValidateError> {
    let table = Table1D::new(
        case.drag.mach.clone(),
        case.drag.drag_coefficient.clone(),
        Interpolation::Linear,
        // Past the table there is no shared drag, so none is invented.
        Extrapolation::Error,
    )
    .map_err(|error| refuse(format!("{}: {error}", case.name)))?;
    DragTable::new(table.clone(), Some(table))
        .with_reference_diameter_m(case.drag.reference_diameter_m)
        .map_err(|error| refuse(format!("{}: {error}", case.name)))
}

/// Flies HPR Sim on a case, with its own drag or with the inputs' table.
fn fly_hpr(root: &Path, case: &InputCase, same_drag: bool) -> Result<Flown, ValidateError> {
    let failed = |what: &str, source: SimError| ValidateError::Flight {
        case: case.name.clone(),
        what: what.to_owned(),
        source: Some(Box::new(source)),
    };
    let rocket: Rocket = parse(&case.design, &read(root, &case.design, "reading a design")?)?;
    let simulation = simulation(&rocket, &case.name, &case.site, &case.rail)?;
    let simulation = if same_drag {
        simulation.with_drag_table(drag_table(case)?)
    } else {
        simulation
    };
    let mut metrics = FlightMetrics::new();
    let result = simulation
        .run(&mut metrics)
        .map_err(|error| failed("flying", error))?;
    let summary = metrics
        .summary(&result, simulation.environment())
        .map_err(|error| failed("summing the flight up", error))?;
    let burnout_s = result
        .events
        .iter()
        .find(|event| event.kind == EventKind::Burnout)
        .map(|event| event.sample.time_s)
        .ok_or_else(|| refuse(format!("{}: no burnout", case.name)))?;
    let mut at = At {
        time_s: burnout_s + AFTER_BURNOUT_S,
        found: None,
        max_mach: 0.0,
    };
    simulation
        .run(&mut at)
        .map_err(|error| failed("flying again", error))?;
    let missing = |what: &str| refuse(format!("{}: HPR Sim's flight has no {what}", case.name));
    let after = at.found.ok_or_else(|| missing("step just after burnout"))?;
    let start = result
        .events
        .first()
        .map(|event| event.sample)
        .ok_or_else(|| missing("first event"))?;
    let environment = simulation.environment();
    let gravity = environment
        .earth
        .gravity_enu_mps2(start.cg_enu_m)
        .map_err(|error| refuse(format!("{}: {error}", case.name)))?;
    let air = environment
        .atmosphere
        .air(case.site.elevation_m + start.height_above_ground_m)
        .map_err(|error| refuse(format!("{}: {error}", case.name)))?;
    let after_burnout_parts = if same_drag {
        // One table is the whole drag: it has no parts.
        Vec::new()
    } else {
        // The drag buildup's parts at that instant, in the air HPR Sim flew it through.
        let there = environment
            .atmosphere
            .air(case.site.elevation_m + after.height_above_ground_m)
            .map_err(|error| refuse(format!("{}: {error}", case.name)))?
            .air;
        let parts = simulation
            .aero()
            .buildup_components(
                &Flow::new(after.mach, after.angle_of_attack_rad, 0.0),
                &DragConditions::coasting(
                    after.airspeed_m_s * there.density_kg_m3 / there.dynamic_viscosity_pa_s,
                ),
            )
            .map_err(|error| refuse(format!("{}: {error}", case.name)))?;
        let own: Vec<PartDrag> = parts
            .into_iter()
            .filter(|part| part.drag.axial_coefficient != 0.0)
            .map(|part| PartDrag {
                id: part.id,
                axial_coefficient: part.drag.axial_coefficient,
            })
            .collect();
        // The parts are the flight's own coefficient, worked out again from its sample.
        let sum: f64 = own.iter().map(|part| part.axial_coefficient).sum();
        if !within(
            sum,
            after.axial_coefficient,
            1e-6 * after.axial_coefficient.abs(),
        ) {
            return Err(refuse(format!(
                "{}: HPR Sim's parts sum to {sum}, and its flight's axial coefficient there is {}",
                case.name, after.axial_coefficient
            )));
        }
        own
    };
    Ok(Flown {
        apogee_rise_m: summary
            .apogee
            .and_then(|apogee| apogee.gain_m)
            .ok_or_else(|| missing("apogee"))?,
        rail_exit_speed_m_s: {
            // The nose tip's speed as the last guide leaves, and a check that it has then
            // traveled the length MAPLEAF was given.
            let exit = result
                .events
                .iter()
                .find(|event| event.kind == EventKind::RailExit)
                .ok_or_else(|| missing("rail exit"))?;
            let traveled = (exit.sample.state.position_enu_m - start.state.position_enu_m).length();
            if !within(traveled, case.rail.guided_travel_m, 1e-6) {
                return Err(refuse(format!(
                    "{}: HPR Sim's rocket left the rail after {traveled} m, not the {} m MAPLEAF \
                     was given",
                    case.name, case.rail.guided_travel_m
                )));
            }
            exit.sample.state.velocity_enu_m_s.length()
        },
        max_mach: at.max_mach,
        gravity_at_ignition_m_s2: gravity.length(),
        air_density_at_ignition_kg_m3: air.air.density_kg_m3,
        after_burnout_mach: after.mach,
        after_burnout_axial_coefficient: after.axial_coefficient,
        after_burnout_parts,
    })
}

fn number(value: &Value, path: &[&str]) -> Result<f64, ValidateError> {
    path.iter()
        .try_fold(value, |value, key| value.get(key))
        .and_then(Value::as_f64)
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no {}", path.join("."))))
}

/// Whether `value` is within `tolerance` of `of`. A value that is not a number is not, so a
/// check written with this refuses it.
fn within(value: f64, of: f64, tolerance: f64) -> bool {
    (value - of).abs() <= tolerance
}

fn percent(difference: f64, of: f64) -> f64 {
    100.0 * difference / of
}

/// Where two JSON values first differ by more than the last digits platforms round
/// differently, with both values; `None` when they do not.
fn first_difference(path: &str, committed: &Value, computed: &Value) -> Option<String> {
    let differs = || Some(format!("{path}: {committed} against {computed}"));
    match (committed, computed) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) if same_but_for_platform_rounding(a, b) => None,
            _ => differs(),
        },
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Some(format!("{path}: {} items against {}", a.len(), b.len()));
            }
            a.iter()
                .zip(b)
                .enumerate()
                .find_map(|(index, (a, b))| first_difference(&format!("{path}[{index}]"), a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            if a.len() != b.len() || a.keys().any(|key| !b.contains_key(key)) {
                return Some(format!("{path}: its fields differ"));
            }
            a.iter().find_map(|(key, a)| {
                b.get(key)
                    .and_then(|b| first_difference(&format!("{path}.{key}"), a, b))
            })
        }
        (a, b) if a == b => None,
        _ => differs(),
    }
}

/// Checks one kind of flight of one case and compares the two codes on it.
fn compare(
    root: &Path,
    case: &InputCase,
    found: &Value,
    flights: &str,
    same_drag: bool,
) -> Result<Compared, ValidateError> {
    let found = found
        .get(flights)
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives {} no {flights}", case.name)))?;
    // What MAPLEAF's own log gave back is what it was handed.
    for (when, given) in [("ignition", &case.ignition), ("burnout", &case.burnout)] {
        let logged = |key: &str| number(found, &["flown", when, key]);
        for (key, given) in [
            ("mass_kg", given.mass_kg),
            ("transverse_inertia_kg_m2", given.transverse_inertia_kg_m2),
            ("axial_inertia_kg_m2", given.axial_inertia_kg_m2),
        ] {
            let logged = logged(key)?;
            if !within(logged, given, MASS_TOLERANCE * given.abs()) {
                return Err(refuse(format!(
                    "{}: MAPLEAF's {key} at {when} is {logged}, the inputs' {given}",
                    case.name
                )));
            }
        }
        let logged = logged("cg_station_m")?;
        if !within(logged, given.cg_station_m, MASS_TOLERANCE * case.length_m) {
            return Err(refuse(format!(
                "{}: MAPLEAF's center of mass at {when} is {logged} m aft of the tip, the \
                 inputs' {}",
                case.name, given.cg_station_m
            )));
        }
    }
    let mapleaf = Flown {
        apogee_rise_m: number(found, &["metrics", "apogee_rise_m"])?,
        rail_exit_speed_m_s: number(found, &["metrics", "rail_exit_speed_m_s"])?,
        max_mach: number(found, &["flown", "max_mach"])?,
        gravity_at_ignition_m_s2: number(found, &["flown", "gravity_at_ignition_m_s2"])?,
        air_density_at_ignition_kg_m3: number(found, &["flown", "air_density_at_ignition_kg_m3"])?,
        after_burnout_mach: number(found, &["flown", "after_burnout", "mach"])?,
        after_burnout_axial_coefficient: number(
            found,
            &["flown", "after_burnout", "axial_coefficient"],
        )?,
        after_burnout_parts: found
            .get("flown")
            .and_then(|flown| flown.get("after_burnout"))
            .and_then(|after| after.get("parts"))
            .and_then(|parts| serde_json::from_value(parts.clone()).ok())
            .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no parts")))?,
    };
    let hpr = fly_hpr(root, case, same_drag)?;
    Ok(Compared::of(
        hpr,
        mapleaf,
        number(found, &["coarser_step", "apogee_rise_m"])?,
        number(found, &["coarser_step", "rail_exit_speed_m_s"])?,
    ))
}

/// Flies HPR Sim on every case and compares with MAPLEAF's committed flights.
///
/// # Errors
///
/// [`ValidateError`] for a file that cannot be read; for committed inputs that are no longer what
/// [`inputs`] writes, or MAPLEAF flights that were not flown from them (their hash), so a stale
/// oracle run is refused, not compared; for a flight whose logged mass properties are not the
/// inputs'; and for an HPR Sim flight that fails.
pub fn run(root: &Path) -> Result<MapleafReport, ValidateError> {
    let inputs_text = read(root, INPUTS_JSON, "reading MAPLEAF's inputs")?;
    let flights_text = read(root, FLIGHTS_JSON, "reading MAPLEAF's flights")?;
    compare_all(root, &inputs_text, &flights_text)
}

/// [`run`] on the two files' text.
fn compare_all(
    root: &Path,
    inputs_text: &str,
    flights_text: &str,
) -> Result<MapleafReport, ValidateError> {
    let committed: Inputs = parse(INPUTS_JSON, inputs_text)?;
    let fresh = inputs(root)?;
    let as_value = |inputs: &Inputs| {
        serde_json::to_value(inputs).map_err(|source| ValidateError::Json {
            path: INPUTS_JSON.to_owned(),
            source: Box::new(source),
        })
    };
    if let Some(what) = first_difference("inputs", &as_value(&committed)?, &as_value(&fresh)?) {
        return Err(refuse(format!(
            "{INPUTS_JSON} is not what the designs give now ({what}): run `cargo xtask \
             mapleaf-flights --inputs`, fly MAPLEAF again and commit both"
        )));
    }
    let inputs_sha256 = sha256_hex(inputs_text.as_bytes());
    let flights: Value = parse(FLIGHTS_JSON, flights_text)?;
    let text = |key: &str| {
        flights
            .get(key)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no {key}")))
    };
    let oracle = text("oracle")?;
    if !oracle.contains(MAPLEAF_COMMIT) {
        return Err(refuse(format!(
            "{FLIGHTS_JSON} was flown by {oracle}, not the pinned commit {MAPLEAF_COMMIT}"
        )));
    }
    if flights
        .get("inputs")
        .and_then(|inputs| inputs.get("sha256"))
        .and_then(Value::as_str)
        != Some(inputs_sha256.as_str())
    {
        return Err(refuse(format!(
            "{FLIGHTS_JSON} was not flown from the committed {INPUTS_JSON} (its hash differs): \
             fly MAPLEAF again and commit it"
        )));
    }
    let flown = flights
        .get("cases")
        .and_then(Value::as_array)
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no cases")))?;
    if flown.len() != committed.cases.len() {
        return Err(refuse(format!(
            "{FLIGHTS_JSON} has {} cases for {} inputs",
            flown.len(),
            committed.cases.len()
        )));
    }

    let mut rows = Vec::new();
    for (case, found) in committed.cases.iter().zip(flown) {
        if found.get("name").and_then(Value::as_str) != Some(case.name.as_str()) {
            return Err(refuse(format!(
                "{FLIGHTS_JSON}'s cases are not the inputs', in order"
            )));
        }
        let own_drag = compare(root, case, found, "own_drag", false)?;
        let same_drag = compare(root, case, found, "same_drag", true)?;
        let matched = |flight: &str| -> Result<GravityMatched, ValidateError> {
            let value = |key: &str| number(found, &[flight, key]);
            let coarser = |key: &str| number(found, &[flight, "coarser_step", key]);
            Ok(GravityMatched::of(
                &same_drag.hpr,
                value("gravity_scale")?,
                value("gravity_at_ignition_m_s2")?,
                [value("apogee_rise_m")?, value("rail_exit_speed_m_s")?],
                [coarser("apogee_rise_m")?, coarser("rail_exit_speed_m_s")?],
            ))
        };
        let gravity_matched = matched("gravity_matched")?;
        let momentum_matched = matched("momentum_matched")?;
        // The propellant's center is the rows' fifth number ([`MOTOR_COLUMNS`]).
        let internal_momentum_lever_m = case
            .motor
            .rows
            .first()
            .map(|row| case.motor.nozzle_station_m - row[4])
            .ok_or_else(|| refuse(format!("{}: a motor with no rows", case.name)))?;
        rows.push(FlightRow {
            name: case.name.clone(),
            design: case.design.clone(),
            motor: case.motor.designation.clone(),
            ignition_mass_kg: case.ignition.mass_kg,
            guided_travel_m: case.rail.guided_travel_m,
            example_inclination_deg: case.rail.example_inclination_deg,
            own_drag,
            same_drag,
            gravity_matched,
            momentum_matched,
            internal_momentum_lever_m,
            mapleaf_release_body_travel_m: number(
                found,
                &["own_drag", "flown", "own_release", "body_travel_m"],
            )?,
            propellant_center_line_departure_m: number(
                found,
                &["propellant_center_line_departure_m"],
            )?,
            not_passed: case.not_passed.clone(),
        });
    }
    let mapleaf_time_steps_s: Vec<f64> = flights
        .get("time_steps_s")
        .and_then(Value::as_array)
        .map(|steps| steps.iter().filter_map(Value::as_f64).collect())
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no time_steps_s")))?;
    let leaning_rail: LeaningRail = flights
        .get("leaning_rail")
        .and_then(|leaning| serde_json::from_value(leaning.clone()).ok())
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no leaning_rail")))?;
    let mut mapleaf_defaults_kept: Vec<Setting> = flights
        .get("mapleaf_defaults_kept")
        .and_then(Value::as_object)
        .ok_or_else(|| refuse(format!("{FLIGHTS_JSON} gives no mapleaf_defaults_kept")))?
        .iter()
        .map(|(name, value)| Setting {
            name: name.clone(),
            value: value.as_str().unwrap_or_default().to_owned(),
        })
        .collect();
    mapleaf_defaults_kept.sort_by(|a, b| a.name.cmp(&b.name));
    let not_shared = not_shared(
        &rows,
        &leaning_rail,
        &mapleaf_defaults_kept,
        &mapleaf_time_steps_s,
    );
    Ok(MapleafReport {
        generated_by: "cargo xtask mapleaf-flights".to_owned(),
        oracle,
        oracle_command: text("command")?,
        inputs: INPUTS_JSON.to_owned(),
        inputs_sha256,
        flights: FLIGHTS_JSON.to_owned(),
        flights_sha256: sha256_hex(flights_text.as_bytes()),
        mapleaf_time_steps_s,
        rail_inclination_deg: RAIL_INCLINATION_DEG,
        shared_drag: committed
            .cases
            .first()
            .map(|case| case.drag.what.clone())
            .unwrap_or_default(),
        mapleaf_defaults_kept,
        leaning_rail,
        rows,
        not_shared,
    })
}

/// `value` rounded away from zero to `decimals` places, so a bound written with it holds.
fn outward(value: f64, decimals: i32) -> f64 {
    let scale = 10f64.powi(decimals);
    let scaled = value.abs() * scale;
    // A value already on the grid stays: only what the places cannot show is rounded up.
    let up = if (scaled - scaled.round()).abs() < 1e-9 {
        scaled.round()
    } else {
        scaled.ceil()
    };
    value.signum() * up / scale
}

/// One value per design, each with its design's name.
fn each(rows: &[FlightRow], value: impl Fn(&FlightRow) -> String) -> String {
    rows.iter()
        .map(|row| format!("{} (`{}`)", value(row), row.name))
        .collect::<Vec<_>>()
        .join(", ")
}

/// `value` in scientific notation with one decimal, its size rounded up so that a bound written
/// with it holds: 4.44e-16 is `4.5e-16`.
fn outward_e(value: f64) -> String {
    if !(value.is_finite() && value != 0.0) {
        return format!("{value}");
    }
    let mut exponent = value.abs().log10().floor();
    let mut mantissa = outward(value.abs() / 10f64.powf(exponent), 1);
    if mantissa >= 10.0 {
        mantissa /= 10.0;
        exponent += 1.0;
    }
    format!(
        "{}{mantissa:.1}e{exponent:.0}",
        if value < 0.0 { "-" } else { "" }
    )
}

/// The largest size of `value` over the flights given.
fn largest<'a, T: 'a>(flights: impl Iterator<Item = &'a T>, value: impl Fn(&T) -> f64) -> f64 {
    flights
        .map(|flight| value(flight).abs())
        .fold(0.0, f64::max)
}

/// How far a matched flight's apogee moves between MAPLEAF's two time steps, m.
fn matched_apogee_step_m(matched: &GravityMatched) -> f64 {
    matched.mapleaf_coarser_apogee_rise_m - matched.mapleaf_apogee_rise_m
}

/// How far a matched flight's rail-exit speed moves between MAPLEAF's two time steps, m/s.
fn matched_rail_exit_step_m_s(matched: &GravityMatched) -> f64 {
    matched.mapleaf_coarser_rail_exit_speed_m_s - matched.mapleaf_rail_exit_speed_m_s
}

/// The drag of the parts only HPR Sim has, as a share of its axial coefficient, %.
fn hpr_only_drag_percent(compared: &Compared) -> f64 {
    let only: f64 = compared
        .hpr
        .after_burnout_parts
        .iter()
        .filter(|part| {
            !compared
                .mapleaf
                .after_burnout_parts
                .iter()
                .any(|theirs| theirs.id == part.id)
        })
        .map(|part| part.axial_coefficient)
        .sum();
    percent(only, compared.hpr.after_burnout_axial_coefficient)
}

/// The inputs the two codes could not share, with what was measured of each.
fn not_shared(
    rows: &[FlightRow],
    leaning: &LeaningRail,
    defaults: &[Setting],
    time_steps_s: &[f64],
) -> Vec<NotShared> {
    let entry =
        |input: &str, flights: &str, hpr: String, mapleaf: String, measured: String| NotShared {
            input: input.to_owned(),
            flights: flights.to_owned(),
            hpr,
            mapleaf,
            measured,
        };
    let every = "every flight";
    let own = "own drag";
    let defaults = defaults
        .iter()
        .filter(|setting| setting.name.starts_with("Rocket.Aero."))
        .map(|setting| format!("`{} {}`", setting.name, setting.value))
        .collect::<Vec<_>>()
        .join(", ");
    let finest_ms = 1e3 * time_steps_s.iter().copied().fold(f64::INFINITY, f64::min);
    let coarsest_ms = 1e3 * time_steps_s.iter().copied().fold(0.0, f64::max);
    vec![
        entry(
            "Drag",
            own,
            "its own drag buildup, part by part".to_owned(),
            format!(
                "its own, from the same outside shape and surface roughness, with its defaults \
                 kept ({defaults})"
            ),
            format!(
                "The axial coefficient just after burnout, HPR Sim's less MAPLEAF's: {}. By \
                 part below.",
                each(rows, |row| format!(
                    "{:+.1}%",
                    row.own_drag.axial_coefficient_difference_percent
                ))
            ),
        ),
        entry(
            "Drag of the rail buttons",
            own,
            "in its drag buildup".to_owned(),
            "no such part".to_owned(),
            format!(
                "Of HPR Sim's axial coefficient just after burnout: {}.",
                each(rows, |row| format!(
                    "{:.1}%",
                    hpr_only_drag_percent(&row.own_drag)
                ))
            ),
        ),
        entry(
            "Normal force and moments",
            every,
            "its own".to_owned(),
            "its own with its own drag; none with the same drag, where no shape is flown"
                .to_owned(),
            "Not measured apart.".to_owned(),
        ),
        entry(
            "Gravity",
            every,
            "WGS 84 normal gravity at the site's latitude and the rocket's height, the \
             centrifugal term in it"
                .to_owned(),
            "its `Flat` Earth model, the default: GM/(R + z)² with R = 6371 km, the same at every \
             latitude. Its round and WGS 84 Earth models, which turn with the Earth and fly in \
             an inertial frame, were not flown"
                .to_owned(),
            format!(
                "At ignition, HPR Sim's against MAPLEAF's: {}. With MAPLEAF's gravity matched \
                 to HPR Sim's at the pad, the same-drag apogee differences go from {} to {} \
                 ([the table](#with-mapleafs-gravity-matched)).",
                each(rows, |row| format!(
                    "{:.4} against {:.4} m/s² ({:+.2}%)",
                    row.own_drag.hpr.gravity_at_ignition_m_s2,
                    row.own_drag.mapleaf.gravity_at_ignition_m_s2,
                    percent(
                        row.own_drag.hpr.gravity_at_ignition_m_s2
                            - row.own_drag.mapleaf.gravity_at_ignition_m_s2,
                        row.own_drag.mapleaf.gravity_at_ignition_m_s2
                    )
                )),
                each(rows, |row| format!(
                    "{:+.2}%",
                    row.same_drag.apogee_difference_percent
                )),
                each(rows, |row| format!(
                    "{:+.4}%",
                    row.gravity_matched.apogee_difference_percent
                )),
            ),
        ),
        entry(
            "The Earth's turning",
            every,
            "the Coriolis force on the moving rocket".to_owned(),
            "none in its `Flat` model".to_owned(),
            "Not measured apart.".to_owned(),
        ),
        entry(
            "The air",
            every,
            "the 1976 standard atmosphere, worked out at each height".to_owned(),
            "its table of the U.S. Standard Atmosphere".to_owned(),
            format!(
                "The center of mass starts at the same height in both. The air density at \
                 ignition then differs by at most {} of itself.",
                outward_e(largest(
                    rows.iter().flat_map(|row| [&row.own_drag, &row.same_drag]),
                    |compared| compared.hpr.air_density_at_ignition_kg_m3
                        / compared.mapleaf.air_density_at_ignition_kg_m3
                        - 1.0
                ))
            ),
        ),
        entry(
            "The rail's lean",
            every,
            format!(
                "straight up, where the examples' rails lean {} from the vertical",
                each(rows, |row| format!(
                    "{:.1}°",
                    RAIL_INCLINATION_DEG - row.example_inclination_deg
                ))
            ),
            "straight up too".to_owned(),
            format!(
                "On a rail {:.1}° from the vertical MAPLEAF's rocket travels {:.2}° from it \
                 with its body {:.2}° from it, an angle of attack of {:.2}° while still guided \
                 (`{}`, [below](#mapleaf-on-a-leaning-rail)).",
                leaning.rail_from_vertical_deg,
                leaning.travel_from_vertical_deg,
                leaning.body_axis_from_vertical_deg,
                leaning.angle_of_attack_deg,
                leaning.case
            ),
        ),
        entry(
            "Leaving the rail",
            every,
            "guided until the airframe has traveled the guided length; no rail friction and no \
             tip-off phase, neither of which MAPLEAF has"
                .to_owned(),
            "guided until its center of mass has traveled that length; the center moves \
             forward in the airframe as the propellant burns, so the airframe has then \
             traveled less"
                .to_owned(),
            format!(
                "MAPLEAF frees the rocket with its airframe short of the guided length by {}. \
                 Both rail-exit speeds here are read where the airframe has traveled all of it.",
                each(rows, |row| format!(
                    "{:.1} mm",
                    1e3 * (row.guided_travel_m - row.mapleaf_release_body_travel_m)
                ))
            ),
        ),
        entry(
            "Internal-momentum terms beside the thrust",
            every,
            "−m r″ − 2ṁ r′ + m̈ (n − r) beside the thrust, as RocketPy has them: m is the \
             rocket's mass, r its center of mass and n the nozzle exit. With the propellant's \
             center c fixed in the airframe, as on these motors, they sum to m̈ (n − c)"
                .to_owned(),
            "none: its motor's force is the thrust curve alone".to_owned(),
            format!(
                "Added to MAPLEAF's thrust in its gravity-matched flight, that force moves \
                 MAPLEAF's apogee by {} and its rail-exit speed by {}. Between MAPLEAF's two \
                 time steps that flight's apogee moves by up to {:.3} m \
                 ([the table](#with-the-internal-momentum-force-added)).",
                each(rows, |row| format!(
                    "{:+.3} m",
                    row.momentum_matched.mapleaf_apogee_rise_m
                        - row.gravity_matched.mapleaf_apogee_rise_m
                )),
                each(rows, |row| format!(
                    "{:+.5} m/s",
                    row.momentum_matched.mapleaf_rail_exit_speed_m_s
                        - row.gravity_matched.mapleaf_rail_exit_speed_m_s
                )),
                outward(
                    largest(
                        rows.iter().map(|row| &row.momentum_matched),
                        matched_apogee_step_m
                    ),
                    3
                ),
            ),
        ),
        entry(
            "Jet damping",
            every,
            "the exhaust's damping of pitch and yaw".to_owned(),
            "only as a `FractionalJetDamping` part, which these flights do not add".to_owned(),
            "It acts on a rocket that turns, and these flights go straight up in calm air. \
             Not measured apart."
                .to_owned(),
        ),
        entry(
            "Where the propellant's center of mass is",
            every,
            "where its motor model puts it at each moment".to_owned(),
            "on a straight line in the burnt fraction, between the two centers its motor file \
             gives"
                .to_owned(),
            format!(
                "HPR Sim's center leaves that line by at most {} m on these motors.",
                outward_e(largest(rows.iter(), |row| row.propellant_center_line_departure_m))
            ),
        ),
        entry(
            "Mass and inertia between the motor file's rows",
            every,
            "its motor model at every moment".to_owned(),
            format!(
                "straight lines between the rows HPR Sim's model was sampled into, \
                 {ROWS_PER_INTERVAL} to each interval of the thrust curve"
            ),
            "MAPLEAF's logged mass, center of mass and inertia at ignition and at burnout are \
             the inputs' to one part in a million, checked on every flight in the tables. \
             Between the rows, not measured."
                .to_owned(),
        ),
        entry(
            "Integration",
            every,
            "Dormand–Prince 5(4) with an adaptive step, relative and absolute tolerance 1e-8"
                .to_owned(),
            format!("classical fourth-order Runge–Kutta at a fixed {finest_ms:.0} ms"),
            format!(
                "At {coarsest_ms:.0} ms MAPLEAF's apogee moves by at most {:.3} m with its own \
                 drag and {:.3} m with the same drag, and its rail-exit speed by at most {:.4} \
                 m/s in either. The matched flights' steps are with their tables.",
                outward(
                    largest(rows.iter().map(|row| &row.own_drag), |compared| compared
                        .mapleaf_coarser_apogee_rise_m
                        - compared.mapleaf.apogee_rise_m),
                    3
                ),
                outward(
                    largest(rows.iter().map(|row| &row.same_drag), |compared| compared
                        .mapleaf_coarser_apogee_rise_m
                        - compared.mapleaf.apogee_rise_m),
                    3
                ),
                outward(
                    largest(
                        rows.iter().flat_map(|row| [&row.own_drag, &row.same_drag]),
                        |compared| compared.mapleaf_coarser_rail_exit_speed_m_s
                            - compared.mapleaf.rail_exit_speed_m_s
                    ),
                    4
                ),
            ),
        ),
    ]
}

/// The feet of a bound in meters that was rounded outward to three places, rounded outward
/// again: the pair then converts, and both still bound the value.
fn bound_feet(m: f64) -> f64 {
    outward(outward(m, 3) / 0.3048, 3)
}

/// A length in meters with its feet.
fn length(m: f64) -> String {
    format!("{m:.1} m ({:.0} ft)", m / 0.3048)
}

/// A speed in meters a second with its feet a second.
fn speed(m_s: f64) -> String {
    format!("{m_s:.3} m/s ({:.2} ft/s)", m_s / 0.3048)
}

impl MapleafReport {
    /// The report's page.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut page = String::new();
        let designs = self
            .rows
            .iter()
            .map(|row| format!("[`{}`](../../{})", row.name, row.design))
            .collect::<Vec<_>>()
            .join(", ");
        let largest_mach = self
            .rows
            .iter()
            .flat_map(|row| [&row.own_drag, &row.same_drag])
            .flat_map(|compared| [compared.hpr.max_mach, compared.mapleaf.max_mach])
            .fold(0.0, f64::max);
        let not_passed = each(&self.rows, |row| row.not_passed.join(", "));
        let _ = writeln!(
            page,
            "# HPR Sim against MAPLEAF on three public designs\n\n\
             Written by `{}` ([M1.14b8][m1-14b8], decision [ADR-378][adr-378]); do not edit. \
             [MAPLEAF][mapleaf] is an open-source (MIT) six-degree-of-freedom rocket flight \
             simulator with its own aerodynamics, written apart from HPR Sim and from RocketPy. \
             This report flies it and HPR Sim on the same {} public designs and gives how far \
             apart their apogees and rail-exit speeds are. Neither code is the truth: a \
             difference here is a measurement to explain, and nothing in this report is held to \
             a tolerance. It covers flights straight up in calm air to apogee, all subsonic \
             (the largest Mach number is {:.2}); it says nothing about wind, a leaning rail, \
             the descent or supersonic flight.\n",
            self.generated_by,
            self.rows.len(),
            outward(largest_mach, 2),
        );
        let _ = writeln!(
            page,
            "- **The oracle** is {}, run as an outside program in its own Python environment \
             (`validation/oracles/mapleaf/`); its flights are `{}`.\n\
             - **The designs** are {designs}: the RocketPy examples with a tangent-ogive nose, \
             the one curved nose MAPLEAF has.\n\
             - **The inputs** (`{}`) are written from HPR Sim's own reading of each design, so \
             MAPLEAF is handed what HPR Sim flies: the outside shape and its surface roughness, \
             the mass, center of mass and inertia without propellant, the motor's thrust, \
             propellant flow, propellant center and propellant inertia through the burn, the \
             guided travel on the rail, the site, and the height the center of mass stands at \
             on the pad. Parts MAPLEAF has no counterpart for are \
             not handed over, and their mass stays in the total: {not_passed}.\n\
             - **Same drag:** both codes fly one table of drag coefficient against Mach number, \
             motor on or off ({}). What is left between them is everything but the drag.\n\
             - **Own drag:** each code works its aerodynamics out from the same outside shape.\n\
             - **Apogee** is how far the center of mass rises above where it stood at ignition.\n\
             - **Rail-exit speed** is the airframe's speed where it has traveled the guided \
             length, which is the rail's length less the distance from the rocket's last guide \
             to its aft end.\n\
             - **A difference** is HPR Sim's value less MAPLEAF's, and its percentage is of \
             MAPLEAF's.\n",
            self.oracle, self.flights, self.inputs, self.shared_drag,
        );
        let _ = writeln!(
            page,
            "[m1-14b8]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b8\n\
             [adr-378]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/{ADR_FILE}\n\
             [mapleaf]: https://github.com/henrystoldt/MAPLEAF\n"
        );

        for (title, same_drag) in [("Same drag", true), ("Own drag", false)] {
            let _ = writeln!(page, "## {title}\n");
            let _ = writeln!(
                page,
                "| design | motor | mass at ignition | largest Mach number | apogee, HPR Sim | \
                 apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | \
                 difference |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|"
            );
            for row in &self.rows {
                let compared = if same_drag {
                    &row.same_drag
                } else {
                    &row.own_drag
                };
                let _ = writeln!(
                    page,
                    "| `{}` | {} | {:.3} kg | {:.3} | {} | {} | {:+.2} m ({:+.2}%) | {} | {} | \
                     {:+.4} m/s ({:+.3}%) |",
                    row.name,
                    row.motor,
                    row.ignition_mass_kg,
                    compared.hpr.max_mach,
                    length(compared.hpr.apogee_rise_m),
                    length(compared.mapleaf.apogee_rise_m),
                    compared.apogee_difference_m,
                    compared.apogee_difference_percent,
                    speed(compared.hpr.rail_exit_speed_m_s),
                    speed(compared.mapleaf.rail_exit_speed_m_s),
                    compared.rail_exit_difference_m_s,
                    compared.rail_exit_difference_percent,
                );
            }
            page.push('\n');
            if same_drag {
                self.gravity_matched_table(&mut page);
            } else {
                self.own_drag_tables(&mut page);
            }
        }

        let _ = writeln!(
            page,
            "## What the two could not share\n\n\
             Each row is an input or a model the two codes differ in, in the flights named, \
             with what was measured of it. *Not measured apart* means its effect is inside the \
             differences above and has no number of its own.\n\n\
             | input | flights | HPR Sim | MAPLEAF | measured |\n|---|---|---|---|---|"
        );
        for entry in &self.not_shared {
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} | {} |",
                entry.input, entry.flights, entry.hpr, entry.mapleaf, entry.measured
            );
        }
        let leaning = &self.leaning_rail;
        let _ = writeln!(
            page,
            "\n## MAPLEAF on a leaning rail\n\n\
             Every flight here leaves a vertical rail ({:.0}° from the horizontal), though the \
             examples' rails lean. Flown on `{}`'s rail as its example has it, {:.1}° from the \
             vertical, MAPLEAF at this commit moves the rocket up a line {:.2}° from the \
             vertical while its body points {:.2}° from it: halfway up the rail ({:.2} m of \
             travel) the rocket is still guided and already flies at an angle of attack of \
             {:.2}°. A rail holds a rocket's travel and its axis on one line, so that is not \
             the launch the input asks for, and the lean is left out of both codes' flights \
             instead of compared.\n",
            self.rail_inclination_deg,
            leaning.case,
            leaning.rail_from_vertical_deg,
            leaning.travel_from_vertical_deg,
            leaning.body_axis_from_vertical_deg,
            leaning.at_travel_m,
            leaning.angle_of_attack_deg,
        );
        let _ = writeln!(
            page,
            "## Running it again\n\n\
             ```sh\n\
             cargo xtask mapleaf-flights --inputs   # HPR Sim's reading of the designs\n\
             UV_PROJECT_ENVIRONMENT=$PWD/refs/venv-mapleaf uv sync --locked --project validation/oracles/mapleaf\n\
             {}   # MAPLEAF's flights\n\
             cargo xtask mapleaf-flights            # this report\n\
             ```\n\n\
             `cargo xtask mapleaf-flights --check` flies HPR Sim again and fails unless the \
             committed inputs, report and page reproduce; it refuses MAPLEAF flights that were \
             not flown from the committed inputs (their SHA-256, `{}`). MAPLEAF itself runs \
             only where its environment is installed.",
            self.oracle_command, self.inputs_sha256,
        );
        page
    }

    /// The same-drag section's flights with MAPLEAF's gravity matched.
    fn gravity_matched_table(&self, page: &mut String) {
        let _ = writeln!(
            page,
            "### With MAPLEAF's gravity matched\n\n\
             The two codes' gravity differs at the pad by up to {:.3}% \
             ([the list below](#what-the-two-could-not-share)). To measure what that moves, \
             MAPLEAF flew each same-drag flight once more with its gravity constant scaled so \
             that its gravity at the pad is HPR Sim's. That is a measurement, not MAPLEAF as it \
             ships; HPR Sim's flights are the same-drag ones above.\n\n\
             | design | factor on MAPLEAF's gravity constant | apogee, HPR Sim | apogee, \
             MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |\n\
             |---|---:|---:|---:|---:|---:|---:|---:|",
            outward(
                self.rows
                    .iter()
                    .map(|row| (row.gravity_matched.mapleaf_gravity_scale - 1.0).abs())
                    .fold(0.0, f64::max)
                    * 100.0,
                3
            )
        );
        for row in &self.rows {
            let matched = &row.gravity_matched;
            let _ = writeln!(
                page,
                "| `{}` | {:.6} | {:.2} m | {:.2} m | {:+.4} m ({:+.5}%) | {:.4} m/s | {:.4} m/s \
                 | {:+.5} m/s ({:+.4}%) |",
                row.name,
                matched.mapleaf_gravity_scale,
                row.same_drag.hpr.apogee_rise_m,
                matched.mapleaf_apogee_rise_m,
                matched.apogee_difference_m,
                matched.apogee_difference_percent,
                row.same_drag.hpr.rail_exit_speed_m_s,
                matched.mapleaf_rail_exit_speed_m_s,
                matched.rail_exit_difference_m_s,
                matched.rail_exit_difference_percent,
            );
        }
        let most = |value: fn(&GravityMatched) -> f64| {
            largest(self.rows.iter().map(|row| &row.gravity_matched), value)
        };
        let apogee_m = most(|matched| matched.apogee_difference_m);
        let rail_exit_m_s = most(|matched| matched.rail_exit_difference_m_s);
        let step_m = most(matched_apogee_step_m);
        let step_m_s = most(matched_rail_exit_step_m_s);
        let _ = writeln!(
            page,
            "\nWith the drag and the gravity both shared, the two codes' apogees are within \
             {:.3} m ({:.3} ft, {:.4}%) of each other and their rail-exit speeds within {:.5} \
             m/s ({:.4} ft/s, {:.4}%). Between its two time steps MAPLEAF's apogee in these \
             flights moves by up to {:.3} m ({:.3} ft) and its rail-exit speed by up to {:.5} \
             m/s ({:.4} ft/s), so a difference smaller than that is not resolved here.\n",
            outward(apogee_m, 3),
            bound_feet(apogee_m),
            outward(most(|matched| matched.apogee_difference_percent), 4),
            outward(rail_exit_m_s, 5),
            outward(rail_exit_m_s / 0.3048, 4),
            outward(most(|matched| matched.rail_exit_difference_percent), 4),
            outward(step_m, 3),
            bound_feet(step_m),
            outward(step_m_s, 5),
            outward(step_m_s / 0.3048, 4),
        );
        self.momentum_matched_table(page);
    }

    /// The matched flights again with HPR Sim's internal-momentum force on MAPLEAF's thrust.
    fn momentum_matched_table(&self, page: &mut String) {
        let _ = writeln!(
            page,
            "### With the internal-momentum force added\n\n\
             HPR Sim's equations of motion carry three internal-momentum terms beside the \
             thrust, as RocketPy's do, and MAPLEAF's carry none \
             ([the list below](#what-the-two-could-not-share)). With the propellant's center \
             fixed in the airframe, as on these motors, the terms sum to one force along the \
             axis: the rate of change of the propellant flow times the lever, the nozzle \
             exit's distance behind the propellant's center. In a second measurement MAPLEAF \
             flew each gravity-matched flight again with that force added to its thrust. \
             *Moved by* is this flight less the gravity-matched one; *difference* is HPR \
             Sim's same-drag flight less this one; *step* is this flight at MAPLEAF's coarser \
             time step less at its finer.\n\n\
             | design | lever | apogee, MAPLEAF | moved by | difference | step | rail exit, \
             MAPLEAF | moved by | difference | step |\n\
             |---|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
        for row in &self.rows {
            let (matched, before) = (&row.momentum_matched, &row.gravity_matched);
            let _ = writeln!(
                page,
                "| `{}` | {:.3} m | {:.2} m | {:+.3} m | {:+.3} m ({:+.4}%) | {:+.3} m | {:.4} \
                 m/s | {:+.5} m/s | {:+.5} m/s ({:+.4}%) | {:+.5} m/s |",
                row.name,
                row.internal_momentum_lever_m,
                matched.mapleaf_apogee_rise_m,
                matched.mapleaf_apogee_rise_m - before.mapleaf_apogee_rise_m,
                matched.apogee_difference_m,
                matched.apogee_difference_percent,
                matched_apogee_step_m(matched),
                matched.mapleaf_rail_exit_speed_m_s,
                matched.mapleaf_rail_exit_speed_m_s - before.mapleaf_rail_exit_speed_m_s,
                matched.rail_exit_difference_m_s,
                matched.rail_exit_difference_percent,
                matched_rail_exit_step_m_s(matched),
            );
        }
        let most = |value: fn(&GravityMatched) -> f64| {
            largest(self.rows.iter().map(|row| &row.momentum_matched), value)
        };
        let step_m = most(matched_apogee_step_m);
        let step_m_s = most(matched_rail_exit_step_m_s);
        let moved = |value: fn(&GravityMatched) -> f64| {
            largest(self.rows.iter(), |row| {
                value(&row.momentum_matched) - value(&row.gravity_matched)
            })
        };
        let moved_m = moved(|matched| matched.mapleaf_apogee_rise_m);
        let moved_m_s = moved(|matched| matched.mapleaf_rail_exit_speed_m_s);
        let _ = writeln!(
            page,
            "\nThe added force moves MAPLEAF's apogee by up to {:.3} m ({:.3} ft) and its \
             rail-exit speed by up to {:.5} m/s ({:.4} ft/s). It changes at once where the \
             thrust curve bends, and between MAPLEAF's two time steps these flights move by \
             up to {:.3} m ({:.3} ft) of apogee and {:.5} m/s ({:.4} ft/s) of rail-exit speed. \
             A change the force makes that is no larger than that is not resolved here: this \
             table sizes what the terms move, and does not settle it.\n",
            outward(moved_m, 3),
            bound_feet(moved_m),
            outward(moved_m_s, 5),
            outward(moved_m_s / 0.3048, 4),
            outward(step_m, 3),
            bound_feet(step_m),
            outward(step_m_s, 5),
            outward(step_m_s / 0.3048, 4),
        );
    }

    /// The own-drag section's coefficient tables.
    fn own_drag_tables(&self, page: &mut String) {
        let _ = writeln!(
            page,
            "### The drag each flew\n\n\
             Each code's aerodynamic force along the axis just after burnout, as a coefficient \
             on HPR Sim's reference area, whole and by part. The base drag is counted with the \
             last body part in both codes. The two are read at slightly different Mach numbers, \
             each at its own flight's.\n\n\
             | design | Mach number, HPR Sim | Mach number, MAPLEAF | axial coefficient, HPR \
             Sim | axial coefficient, MAPLEAF | difference |\n|---|---:|---:|---:|---:|---:|"
        );
        for row in &self.rows {
            let compared = &row.own_drag;
            let _ = writeln!(
                page,
                "| `{}` | {:.3} | {:.3} | {:.4} | {:.4} | {:+.1}% |",
                row.name,
                compared.hpr.after_burnout_mach,
                compared.mapleaf.after_burnout_mach,
                compared.hpr.after_burnout_axial_coefficient,
                compared.mapleaf.after_burnout_axial_coefficient,
                compared.axial_coefficient_difference_percent,
            );
        }
        let _ = writeln!(
            page,
            "\n| design | part | HPR Sim | MAPLEAF |\n|---|---|---:|---:|"
        );
        for row in &self.rows {
            let (hpr, mapleaf) = (
                &row.own_drag.hpr.after_burnout_parts,
                &row.own_drag.mapleaf.after_burnout_parts,
            );
            let coefficient = |parts: &[PartDrag], id: &str| {
                parts.iter().find(|part| part.id == id).map_or_else(
                    || "none".to_owned(),
                    |part| format!("{:.4}", part.axial_coefficient),
                )
            };
            let mut ids: Vec<&str> = hpr.iter().map(|part| part.id.as_str()).collect();
            for part in mapleaf {
                if !ids.contains(&part.id.as_str()) {
                    ids.push(&part.id);
                }
            }
            for id in ids {
                let _ = writeln!(
                    page,
                    "| `{}` | `{id}` | {} | {} |",
                    row.name,
                    coefficient(hpr, id),
                    coefficient(mapleaf, id)
                );
            }
        }
        let shared: Vec<(f64, f64)> = self
            .rows
            .iter()
            .flat_map(|row| {
                let mapleaf = &row.own_drag.mapleaf.after_burnout_parts;
                row.own_drag
                    .hpr
                    .after_burnout_parts
                    .iter()
                    .filter_map(|ours| {
                        mapleaf
                            .iter()
                            .find(|theirs| theirs.id == ours.id)
                            .map(|theirs| (ours.axial_coefficient, theirs.axial_coefficient))
                    })
            })
            .collect();
        let _ = writeln!(
            page,
            "\nOf the {} parts both codes have, MAPLEAF's coefficient is the higher on {}.\n",
            shared.len(),
            shared.iter().filter(|(ours, theirs)| theirs > ours).count(),
        );
    }

    /// Whether the report holds together and `markdown` is its page.
    ///
    /// # Errors
    ///
    /// The first thing that does not hold, in words: a case list that is not [`CASES`], a flight
    /// with a number that is not finite and positive, a difference that is not its two flights',
    /// parts that do not sum to their whole, a not-shared list that is not the numbers', or a
    /// page that is not [`Self::to_markdown`]'s.
    pub fn check_consistent(&self, markdown: &str) -> Result<(), String> {
        if !self.oracle.contains(MAPLEAF_COMMIT) {
            return Err(format!(
                "the oracle is not at the pinned commit: {}",
                self.oracle
            ));
        }
        if !self.rows.iter().map(|row| row.name.as_str()).eq(CASES) {
            return Err("the report's designs are not the cases'".to_owned());
        }
        if self.inputs_sha256.len() != 64 || self.flights_sha256.len() != 64 {
            return Err("a file has no SHA-256".to_owned());
        }
        for row in &self.rows {
            for (kind, compared, same_drag) in [
                ("own drag", &row.own_drag, false),
                ("same drag", &row.same_drag, true),
            ] {
                for (code, flown) in [("HPR Sim", &compared.hpr), ("MAPLEAF", &compared.mapleaf)] {
                    for (what, value) in [
                        ("apogee", flown.apogee_rise_m),
                        ("rail-exit speed", flown.rail_exit_speed_m_s),
                        ("largest Mach number", flown.max_mach),
                        ("gravity", flown.gravity_at_ignition_m_s2),
                        ("air density", flown.air_density_at_ignition_kg_m3),
                        ("Mach number after burnout", flown.after_burnout_mach),
                        ("axial coefficient", flown.after_burnout_axial_coefficient),
                    ] {
                        if !(value.is_finite() && value > 0.0) {
                            return Err(format!(
                                "{}, {kind}: {code}'s {what} is {value}",
                                row.name
                            ));
                        }
                    }
                    let sum: f64 = flown
                        .after_burnout_parts
                        .iter()
                        .map(|part| part.axial_coefficient)
                        .sum();
                    let whole = flown.after_burnout_axial_coefficient;
                    if same_drag != flown.after_burnout_parts.is_empty()
                        || (!same_drag && !within(sum, whole, 1e-6 * whole))
                    {
                        return Err(format!(
                            "{}, {kind}: {code}'s parts are not its axial coefficient's",
                            row.name
                        ));
                    }
                }
                let again = Compared::of(
                    compared.hpr.clone(),
                    compared.mapleaf.clone(),
                    compared.mapleaf_coarser_apogee_rise_m,
                    compared.mapleaf_coarser_rail_exit_speed_m_s,
                );
                if *compared != again {
                    return Err(format!(
                        "{}, {kind}: its differences are not its two flights'",
                        row.name
                    ));
                }
            }
        }
        // The two start at the same height, so in the same air: a start height left unshared
        // shows as a density difference of a part in ten thousand.
        for row in &self.rows {
            for (kind, compared) in [("own drag", &row.own_drag), ("same drag", &row.same_drag)] {
                let (hpr, mapleaf) = (
                    compared.hpr.air_density_at_ignition_kg_m3,
                    compared.mapleaf.air_density_at_ignition_kg_m3,
                );
                if !within(mapleaf, hpr, 1e-6 * hpr) {
                    return Err(format!(
                        "{}, {kind}: the air density at ignition is {hpr} in HPR Sim and \
                         {mapleaf} in MAPLEAF: the two do not start at the same height",
                        row.name
                    ));
                }
            }
        }
        for row in &self.rows {
            for (kind, matched) in [
                ("gravity matched", &row.gravity_matched),
                ("internal momentum added", &row.momentum_matched),
            ] {
                let again = GravityMatched::of(
                    &row.same_drag.hpr,
                    matched.mapleaf_gravity_scale,
                    matched.mapleaf_gravity_at_ignition_m_s2,
                    [
                        matched.mapleaf_apogee_rise_m,
                        matched.mapleaf_rail_exit_speed_m_s,
                    ],
                    [
                        matched.mapleaf_coarser_apogee_rise_m,
                        matched.mapleaf_coarser_rail_exit_speed_m_s,
                    ],
                );
                if *matched != again {
                    return Err(format!(
                        "{}, {kind}: its differences are not its two flights'",
                        row.name
                    ));
                }
                // A matched flight is matched: MAPLEAF's gravity at ignition is HPR Sim's.
                let hpr = row.same_drag.hpr.gravity_at_ignition_m_s2;
                if !within(matched.mapleaf_gravity_at_ignition_m_s2, hpr, 1e-7 * hpr) {
                    return Err(format!(
                        "{}, {kind}: MAPLEAF's gravity at ignition is {}, HPR Sim's {hpr}",
                        row.name, matched.mapleaf_gravity_at_ignition_m_s2
                    ));
                }
            }
        }
        if self.not_shared
            != not_shared(
                &self.rows,
                &self.leaning_rail,
                &self.mapleaf_defaults_kept,
                &self.mapleaf_time_steps_s,
            )
        {
            return Err("the not-shared list is not the report's numbers'".to_owned());
        }
        if markdown != self.to_markdown() {
            return Err(format!(
                "{REPORT_MD} is not the report's page: run `cargo xtask mapleaf-flights`"
            ));
        }
        Ok(())
    }

    /// Whether `other`, flown now, is this committed report: every word the same and every
    /// flown number the same but for the last digits platforms round differently.
    ///
    /// The differences and the not-shared list's sentences are left out: they are written from
    /// the flown numbers, and [`Self::check_consistent`] holds each report's to its own.
    ///
    /// # Errors
    ///
    /// Where the first difference is, with both values.
    pub fn reproduces(&self, other: &MapleafReport) -> Result<(), String> {
        let flown = |report: &MapleafReport| {
            let mut report = report.clone();
            report.not_shared.clear();
            for row in &mut report.rows {
                for compared in [&mut row.own_drag, &mut row.same_drag] {
                    compared.apogee_difference_m = 0.0;
                    compared.apogee_difference_percent = 0.0;
                    compared.rail_exit_difference_m_s = 0.0;
                    compared.rail_exit_difference_percent = 0.0;
                    compared.axial_coefficient_difference_percent = 0.0;
                }
                for matched in [&mut row.gravity_matched, &mut row.momentum_matched] {
                    matched.apogee_difference_m = 0.0;
                    matched.apogee_difference_percent = 0.0;
                    matched.rail_exit_difference_m_s = 0.0;
                    matched.rail_exit_difference_percent = 0.0;
                }
            }
            serde_json::to_value(&report).map_err(|error| error.to_string())
        };
        match first_difference("the report", &flown(self)?, &flown(other)?) {
            Some(what) => Err(format!(
                "{REPORT_JSON} does not reproduce ({what}): run `cargo xtask mapleaf-flights`"
            )),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::root;

    fn committed() -> MapleafReport {
        let text = std::fs::read_to_string(root().join(REPORT_JSON)).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    fn page() -> String {
        std::fs::read_to_string(root().join(REPORT_MD)).unwrap()
    }

    /// The committed report is what HPR Sim flies now against MAPLEAF's committed flights, and
    /// its page is the report's.
    #[test]
    fn the_committed_report_reproduces() {
        let flown = run(&root()).unwrap();
        let committed = committed();
        committed.reproduces(&flown).unwrap();
        committed.check_consistent(&page()).unwrap();
        // The done-when's count: at least three public designs, each with both numbers.
        assert!(committed.rows.len() >= 3);
        for row in &committed.rows {
            assert!(
                row.design.starts_with("validation/designs/"),
                "{}",
                row.design
            );
        }
    }

    /// Each refusal fires on its own cause, before any flight is compared.
    #[test]
    fn stale_or_foreign_oracle_flights_are_refused() {
        let root = root();
        let inputs_text = std::fs::read_to_string(root.join(INPUTS_JSON)).unwrap();
        let flights_text = std::fs::read_to_string(root.join(FLIGHTS_JSON)).unwrap();
        let refusal = |inputs: &str, flights: &str| {
            compare_all(&root, inputs, flights).unwrap_err().to_string()
        };

        // Flights flown from other inputs: their hash is not the committed file's.
        let hash = sha256_hex(inputs_text.as_bytes());
        assert!(flights_text.contains(&hash));
        let other_hash = flights_text.replace(&hash, &"0".repeat(64));
        let said = refusal(&inputs_text, &other_hash);
        assert!(said.contains("its hash differs"), "{said}");

        // Flights from another MAPLEAF commit.
        assert!(flights_text.contains(MAPLEAF_COMMIT));
        let other_commit = flights_text.replace(MAPLEAF_COMMIT, &"1".repeat(40));
        let said = refusal(&inputs_text, &other_commit);
        assert!(said.contains("not the pinned commit"), "{said}");

        // Inputs that are no longer what the designs give: the same text with one mass moved,
        // which changes the hash too, but the inputs are checked first.
        let mut inputs: Value = serde_json::from_str(&inputs_text).unwrap();
        let mass = inputs["cases"][0]["dry"]["mass_kg"].as_f64().unwrap();
        inputs["cases"][0]["dry"]["mass_kg"] = (mass * 1.001).into();
        let said = refusal(&serde_json::to_string(&inputs).unwrap(), &flights_text);
        assert!(said.contains("is not what the designs give now"), "{said}");
        assert!(said.contains("dry.mass_kg"), "{said}");

        // A MAPLEAF run whose log did not give the inputs' mass back.
        let mut flights: Value = serde_json::from_str(&flights_text).unwrap();
        let logged = flights["cases"][0]["own_drag"]["flown"]["burnout"]["mass_kg"]
            .as_f64()
            .unwrap();
        flights["cases"][0]["own_drag"]["flown"]["burnout"]["mass_kg"] = (logged * 1.001).into();
        let said = refusal(&inputs_text, &serde_json::to_string(&flights).unwrap());
        assert!(said.contains("MAPLEAF's mass_kg at burnout"), "{said}");
    }

    /// An edited number, sentence or page is caught by the report's own check.
    #[test]
    fn check_consistent_catches_each_edit() {
        let report = committed();
        let page = page();
        report.check_consistent(&page).unwrap();

        let mut edited = report.clone();
        edited.rows[2].own_drag.apogee_difference_percent += 0.01;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("own drag: its differences are not its two flights'"),
            "{said}"
        );

        let mut edited = report.clone();
        edited.rows[0].same_drag.hpr.rail_exit_speed_m_s += 0.5;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("same drag: its differences are not its two flights'"),
            "{said}"
        );

        let mut edited = report.clone();
        edited.rows[1].own_drag.mapleaf.after_burnout_parts[0].axial_coefficient += 0.01;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("MAPLEAF's parts are not its axial coefficient's"),
            "{said}"
        );

        let mut edited = report.clone();
        edited.rows[0].gravity_matched.apogee_difference_m += 0.01;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(said.contains("gravity matched: its differences"), "{said}");

        // MAPLEAF's center of mass started on the ground: a meter lower, in denser air.
        let mut edited = report.clone();
        edited.rows[1]
            .same_drag
            .mapleaf
            .air_density_at_ignition_kg_m3 *= 1.0001;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("the two do not start at the same height"),
            "{said}"
        );

        let mut edited = report.clone();
        edited.rows[2].momentum_matched.rail_exit_difference_m_s += 1e-4;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("internal momentum added: its differences"),
            "{said}"
        );

        // A "matched" flight that kept MAPLEAF's own gravity is not one.
        let mut edited = report.clone();
        edited.rows[0]
            .gravity_matched
            .mapleaf_gravity_at_ignition_m_s2 =
            edited.rows[0].same_drag.mapleaf.gravity_at_ignition_m_s2;
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(
            said.contains("gravity matched: MAPLEAF's gravity at ignition"),
            "{said}"
        );

        let mut edited = report.clone();
        edited.not_shared[0].measured.push_str(" And more.");
        let said = edited.check_consistent(&page).unwrap_err();
        assert!(said.contains("the not-shared list"), "{said}");

        let said = report
            .check_consistent(&page.replacen("MAPLEAF", "MAPLEAF ", 1))
            .unwrap_err();
        assert!(said.contains("is not the report's page"), "{said}");
    }

    /// A flown number may move by what platforms round differently, and by no more; a word
    /// may not move at all.
    #[test]
    fn reproduces_allows_rounding_and_nothing_else() {
        let report = committed();
        let mut rounded = report.clone();
        rounded.rows[0].own_drag.hpr.apogee_rise_m *= 1.0 + 1e-9;
        report.reproduces(&rounded).unwrap();

        let mut moved = report.clone();
        moved.rows[0].own_drag.hpr.apogee_rise_m *= 1.0 + 1e-4;
        let said = report.reproduces(&moved).unwrap_err();
        assert!(
            said.contains("rows[0].own_drag.hpr.apogee_rise_m"),
            "{said}"
        );

        let mut renamed = report.clone();
        renamed.rows[1].motor.push('x');
        let said = report.reproduces(&renamed).unwrap_err();
        assert!(said.contains("rows[1].motor"), "{said}");

        let mut shorter = report.clone();
        shorter.rows.pop();
        assert!(report.reproduces(&shorter).is_err());
    }

    /// The inputs hold together: the body runs unbroken from the tip to the tail, the motor's
    /// rows carry its impulse and all its propellant, and the shared drag covers every flight.
    #[test]
    fn the_inputs_hold_together() {
        let inputs = inputs(&root()).unwrap();
        let report = committed();
        assert_eq!(
            inputs
                .cases
                .iter()
                .map(|case| case.name.as_str())
                .collect::<Vec<_>>(),
            CASES
        );
        for (case, row) in inputs.cases.iter().zip(&report.rows) {
            let mut station = 0.0;
            for part in &case.body {
                assert!(
                    (part.fore_station_m - station).abs() < 1e-12,
                    "{}",
                    case.name
                );
                station += part.length_m;
            }
            assert!((station - case.length_m).abs() < 1e-12, "{}", case.name);

            let rows = &case.motor.rows;
            let column = |name: &str| MOTOR_COLUMNS.iter().position(|&c| c == name).unwrap();
            let (time, thrust, mass) = (
                column("time_s"),
                column("thrust_n"),
                column("propellant_mass_kg"),
            );
            assert!(rows.windows(2).all(|pair| pair[0][time] < pair[1][time]));
            let impulse: f64 = rows
                .windows(2)
                .map(|pair| {
                    0.5 * (pair[0][thrust] + pair[1][thrust]) * (pair[1][time] - pair[0][time])
                })
                .sum();
            assert!(
                (impulse - case.motor.total_impulse_ns).abs() < 1e-9 * impulse,
                "{}: {impulse} against {}",
                case.name,
                case.motor.total_impulse_ns
            );
            let (first, last) = (rows.first().unwrap(), rows.last().unwrap());
            assert!(
                last[mass].abs() < 1e-9,
                "{}: {} kg left",
                case.name,
                last[mass]
            );
            assert!(
                (case.dry.mass_kg + first[mass] - case.ignition.mass_kg).abs() < 1e-9,
                "{}",
                case.name
            );
            assert!(
                (case.burnout.mass_kg - case.dry.mass_kg).abs() < 1e-9,
                "{}",
                case.name
            );

            let drag = &case.drag;
            assert_eq!(drag.mach.len(), drag.drag_coefficient.len());
            assert_eq!(drag.mach.first(), Some(&0.0));
            assert!(drag.mach.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(
                drag.drag_coefficient
                    .iter()
                    .all(|&cd| cd.is_finite() && cd > 0.0)
            );
            let fastest = [&row.own_drag, &row.same_drag]
                .into_iter()
                .flat_map(|compared| [compared.hpr.max_mach, compared.mapleaf.max_mach])
                .fold(0.0, f64::max);
            assert!(drag.mach.last().unwrap() > &fastest, "{}", case.name);
            assert_eq!(drag.reference_diameter_m, case.reference_diameter_m);
            // Both rails stand straight up, whatever the example's does.
            assert_eq!(case.rail.inclination_deg, 90.0);
        }
    }

    /// With the same drag, the two codes read the same table: each one's coefficient just after
    /// burnout is the table's at its own Mach number.
    #[test]
    fn same_drag_flights_fly_the_table() {
        let inputs = inputs(&root()).unwrap();
        for (case, row) in inputs.cases.iter().zip(&committed().rows) {
            let table = Table1D::new(
                case.drag.mach.clone(),
                case.drag.drag_coefficient.clone(),
                Interpolation::Linear,
                Extrapolation::Error,
            )
            .unwrap();
            for flown in [&row.same_drag.hpr, &row.same_drag.mapleaf] {
                let at = table.eval(flown.after_burnout_mach).unwrap();
                assert!(
                    (flown.after_burnout_axial_coefficient - at).abs() < 1e-6 * at,
                    "{}: {} against the table's {at}",
                    case.name,
                    flown.after_burnout_axial_coefficient
                );
            }
        }
    }

    /// A bound is rounded away from zero, never toward it.
    #[test]
    fn bounds_round_outward() {
        assert_eq!(outward(0.05874, 3), 0.059);
        assert_eq!(outward(0.0590000001, 3), 0.06);
        assert_eq!(outward(0.059, 3), 0.059);
        assert_eq!(outward(-0.0581, 3), -0.059);
        assert_eq!(outward(0.0, 3), 0.0);
        // A bound's feet come from its rounded meters, so 0.0186 m is 0.019 m (0.063 ft).
        assert_eq!(bound_feet(0.0186), 0.063);
        // Scientific notation rounds its one decimal up too, and carries into the exponent.
        assert_eq!(outward_e(4.44e-16), "4.5e-16");
        assert_eq!(outward_e(4.4e-16), "4.4e-16");
        assert_eq!(outward_e(9.96e-9), "1.0e-8");
        assert_eq!(outward_e(-2.561e-9), "-2.6e-9");
        assert_eq!(outward_e(0.0), "0");
    }
}
