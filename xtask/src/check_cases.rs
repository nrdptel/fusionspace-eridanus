//! NASA's check cases 1 to 3 and 6 to 10 flown by a free body (M1.14b3 and M1.14b4), written by
//! `cargo xtask check-cases` to `validation/reports/check-cases.json` and `.md`.
//!
//! - **The reference** is NASA/TM-2015-218675 (Murri, Jackson and Shelton 2015; errata of
//!   2025-09-08): six-degree-of-freedom check cases that several NASA codes and JSBSim each flew,
//!   with every code's time history published beside the report. The report names no code as the
//!   answer; the codes' agreement is the measure. The histories carry no stated terms, so they
//!   are read from the pinned archive under `refs/` and never committed: this report holds
//!   differences and spreads only.
//! - **Case 1, the dragless sphere** (Volume II, Tables 1 and 25): 1 slug, dropped at rest
//!   relative to the Earth from 30,000 ft over 0° N, 0° E, for 30 s. Compared: its altitude.
//! - **Case 2, the tumbling brick** (Tables 4 and 26, with the errata's 30,000 ft): three unequal
//!   moments of inertia, started at inertial rates of 10, 20 and 30 deg/s about its roll, pitch
//!   and yaw axes, no aerodynamics. Compared: each inertial body rate.
//! - **Case 3, the tumbling brick with damping** (Tables 4, 5 and 27, the errata's 30,000 ft):
//!   case 2 with Table 5's constant roll, pitch and yaw damping coefficients, applied here as
//!   moments ([`Load::Damping`] states the law). Compared: each inertial body rate.
//! - **Cases 6 to 10, the sphere with drag** (Tables 1, 2 and 30 to 34): case 1's sphere with
//!   Table 2's constant drag coefficient, applied here along its velocity through the air
//!   ([`Load::Drag`]): dropped from 30,000 ft in still air (6), in a steady wind (7) and in a wind
//!   that grows with height (8), and shot from sea level at 1,000 ft/s up and 1,000 ft/s east (9)
//!   or north (10). Compared: the position, as the distance between HPR Sim's and the code's.
//! - **HPR Sim's side** is [`hpr_sim::FreeBody`]: the flight's gravity, Coriolis force, air,
//!   wind, rigid-body equations and integrator at their defaults, on a body of the report's mass
//!   and inertia, from the report's initial conditions through its unit factors (Table 73).
//! - **The codes' spread** is the largest difference between any two codes at a time every
//!   code's file has; for a position, the largest distance. One code is left out, and named,
//!   when leaving it out cuts that spread by more than half; the spread of the rest is then the
//!   measure.
//! - **A trace.** Where HPR Sim differs from a kept code by more than the spread, the difference
//!   is set beside the size of a named difference between the two models, computed here apart
//!   from the flight; what that leaves is the unexplained part.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Read as _;
use std::path::Path;

use hpr_atmos::{ConstantWind, LayeredWind, WindInterpolation, WindLevel};
use hpr_core::geodesy::{Ellipsoid, Geodetic};
use hpr_core::gravity::NormalGravity;
use hpr_core::{DMat3, DQuat, DVec3};
use hpr_sim::{
    Adaptive, Environment, FreeBody, FreeBodyLoads, LoadInputs, Loads, Method, SimError, State,
};
use serde_json::{Value, json};

/// The committed report.
pub const REPORT: &str = "validation/reports/check-cases.json";
/// The committed report as a page.
pub const REPORT_MD: &str = "validation/reports/check-cases.md";
/// The codes' time histories, as `validation/refs.lock.toml` pins them.
const ARCHIVE: &str = "refs/sources/nesc-check-cases/Atmospheric_checkcases.zip";

/// `cargo xtask check-cases`'s lines in the command list.
pub const USAGE: &str = "\
  check-cases [--check]    Fly NASA/TM-2015-218675's check cases 1 to 3 and 6 to 10 with a free
                           body against each code's time history, read from the pinned archive
                           under refs/, and write only differences and spreads to
                           validation/reports/check-cases.json and .md. --check fails if the
                           committed report differs instead of writing.";

/// Table 73's unit factors, exact: the foot in meters and the pound-force in newtons.
const FOOT_M: f64 = 0.3048;
const POUND_FORCE_N: f64 = 4.448_221_615_260_5;
/// The slug in kilograms and the slug·ft² in kg·m², from those two.
const SLUG_KG: f64 = POUND_FORCE_N / FOOT_M;
const SLUG_FT2_KG_M2: f64 = POUND_FORCE_N * FOOT_M;
/// Table 73's second zonal harmonic, the one the codes' gravitation carries.
const J2: f64 = 0.001_082_629_82;
/// The dropped cases start 30,000 ft above the ellipsoid (Tables 25 to 32, errata); every case
/// runs 30 s.
const START_HEIGHT_M: f64 = 30_000.0 * FOOT_M;
const DURATION_S: f64 = 30.0;

/// What a case compares, which decides how HPR Sim's state and the codes' files are read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Measure {
    /// The altitude above the ellipsoid, m (case 1).
    Altitude,
    /// The inertial body rates about the roll, pitch and yaw axes, deg/s (cases 2 and 3).
    InertialRates,
    /// The position in Earth-fixed axes, m, compared as the distance between two positions
    /// (cases 6 to 10).
    Position,
}

/// How a quantity is read from a code's file.
#[derive(Clone, Copy, Debug)]
enum Reading {
    /// One column, times a factor to the report's unit.
    Column { column: &'static str, factor: f64 },
    /// The latitude and longitude in degrees and the altitude in feet, turned into an
    /// Earth-fixed position on WGS 84. Every code's file holds those three; two hold no
    /// Earth-fixed position of their own.
    Position,
}

/// One compared quantity of a case.
#[derive(Clone, Copy, Debug)]
struct Quantity {
    /// Its key in the report.
    key: &'static str,
    /// What it is, in words.
    name: &'static str,
    /// How it is read from the codes' files.
    reading: Reading,
    /// The report's unit.
    unit: &'static str,
}

impl Quantity {
    /// Whether it is compared as a distance between two points, not as a signed difference.
    fn is_distance(&self) -> bool {
        matches!(self.reading, Reading::Position)
    }
}

/// The loads the test puts on the body; the free body has no aerodynamics of its own.
#[derive(Clone, Copy, Debug)]
enum Load {
    /// None but gravity and the Coriolis force (cases 1 and 2).
    None,
    /// Table 5's damping (case 3). The report gives the coefficients `C_lp`, `C_mq` and `C_nr`,
    /// a reference area `S`, a span `b` and a chord `c̄`, but no formula. The test takes the
    /// usual one: the moment about each axis is `q̄ S l C (ω l / 2V)`, with `q̄ = ½ρV²` the
    /// dynamic pressure, `V` the speed through the air, `ω` the inertial body rate about that
    /// axis and `l` its length, the span `b` for roll and yaw and the chord `c̄` for pitch;
    /// `ω l / 2V` is the rate made dimensionless. Written as `¼ ρ V S l² C ω`, it needs no
    /// division and is zero at rest. The rate is the inertial one, relative to the stars. The
    /// codes split on it: the moments in their files follow this law in the inertial rate for
    /// SIM 1 and SIM 4 and in the rate relative to the Earth for SIM 5 and SIM 6, each to its
    /// printed digits ([`damping_law_fit`], in the report). The test takes the first.
    Damping {
        area_m2: f64,
        span_m: f64,
        chord_m: f64,
        /// `C_lp`, `C_mq` and `C_nr`.
        coefficients: [f64; 3],
    },
    /// A constant drag coefficient (cases 6 to 10): `F = −q̄ S C_D v / |v|`, along the velocity
    /// through the air `v`, whatever way the body points. A sphere's drag acts so; HPR Sim's
    /// rocket drag acts along the body's axis instead.
    Drag { area_m2: f64, coefficient: f64 },
}

/// The test's loads on a body, as the free body asks for them.
struct TestLoads {
    load: Load,
    /// The Earth's rate in the launch frame's axes, rad/s, for the inertial body rate.
    earth_rate_enu_rad_s: DVec3,
}

impl FreeBodyLoads for TestLoads {
    fn loads(&self, inputs: &LoadInputs) -> Result<Loads, SimError> {
        let density = inputs.air.density_kg_m3;
        let through_air = inputs.air_velocity_body_m_s;
        let speed = through_air.length();
        Ok(match self.load {
            Load::None => Loads::default(),
            Load::Damping {
                area_m2,
                span_m,
                chord_m,
                coefficients,
            } => {
                let state = &inputs.state;
                let inertial = state.body_rate_rad_s
                    + state
                        .attitude
                        .conjugate()
                        .mul_vec3(self.earth_rate_enu_rad_s);
                let lengths = DVec3::new(span_m, chord_m, span_m);
                Loads {
                    force_body_n: DVec3::ZERO,
                    moment_body_n_m: lengths
                        * lengths
                        * DVec3::from_array(coefficients)
                        * inertial
                        * (0.25 * density * speed * area_m2),
                }
            }
            Load::Drag {
                area_m2,
                coefficient,
            } => Loads {
                force_body_n: through_air * (-0.5 * density * speed * area_m2 * coefficient),
                moment_body_n_m: DVec3::ZERO,
            },
        })
    }
}

/// The air's motion in a case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Air {
    /// Still air.
    Still,
    /// Case 7: a steady 20 ft/s from due west.
    Steady,
    /// Case 8: from the west at `(0.003 h − 20)` ft/s, `h` the height above mean sea level in
    /// feet (Table 32).
    Shear,
}

/// Case 8's wind, ft/s from the west, at `height_ft` above mean sea level.
fn shear_ft_s(height_ft: f64) -> f64 {
    0.003 * height_ft - 20.0
}

/// A case's start: its height above the ellipsoid, its velocity relative to the Earth, its
/// attitude and its body rates relative to the local frame (the errata's reading).
#[derive(Clone, Copy, Debug)]
struct Start {
    height_m: f64,
    /// North, east and down, ft/s, as the tables give it.
    velocity_ned_ft_s: [f64; 3],
    /// The body's axes in the launch frame's East-North-Up axes.
    attitude: DQuat,
    /// Roll, pitch and yaw rates relative to the local frame, deg/s.
    rates_deg_s: [f64; 3],
}

/// The named difference between the models that a case's trace sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Trace {
    /// WGS 84's normal gravity against gravitation cut off after `J₂` (case 1).
    Gravity,
    /// The Earth's rate left out of HPR Sim's rotational equations (cases 2 and 3).
    EarthRate,
    /// Case 1's gravity and each code's own air density, along a point mass's flight with the
    /// case's drag and wind (cases 6 to 10).
    GravityOnTheFlight,
}

/// A case: its body, its loads, its air, its start and what is compared.
struct Case {
    number: u32,
    name: &'static str,
    folder: &'static str,
    tables: &'static str,
    body: FreeBody,
    load: Load,
    air: Air,
    start: Start,
    measure: Measure,
    trace: Trace,
    /// The named difference between HPR Sim's model and the codes' that the trace sizes.
    model_difference: Option<&'static str>,
    quantities: Vec<Quantity>,
}

/// The report's body axes at the start of every case but 9, in the launch frame's
/// East-North-Up axes: x north, y east, z down (every Euler angle from the local North-East-Down
/// frame is zero).
fn start_attitude() -> DQuat {
    DQuat::from_mat3(&DMat3::from_cols(DVec3::Y, DVec3::X, -DVec3::Z))
}

fn cases() -> Result<Vec<Case>, String> {
    let body = |mass_slug: f64, moments_slug_ft2: [f64; 3]| {
        FreeBody::new(
            mass_slug * SLUG_KG,
            DMat3::from_diagonal(DVec3::from_array(moments_slug_ft2) * SLUG_FT2_KG_M2),
        )
        .map_err(|e| e.to_string())
    };
    let rate = |key, name, column| Quantity {
        key,
        name,
        reading: Reading::Column {
            column,
            factor: 1.0,
        },
        unit: "deg/s",
    };
    let rates = || {
        vec![
            rate(
                "roll_rate",
                "inertial roll rate",
                "bodyAngularRateWrtEi_deg_s_Roll",
            ),
            rate(
                "pitch_rate",
                "inertial pitch rate",
                "bodyAngularRateWrtEi_deg_s_Pitch",
            ),
            rate(
                "yaw_rate",
                "inertial yaw rate",
                "bodyAngularRateWrtEi_deg_s_Yaw",
            ),
        ]
    };
    let position = || {
        vec![Quantity {
            key: "position",
            name: "position",
            reading: Reading::Position,
            unit: "m",
        }]
    };
    // Tables 1 and 4: the sphere and the brick.
    let sphere = || body(1.0, [3.6, 3.6, 3.6]);
    let brick = || body(0.155_404_754, [0.001_894_220, 0.006_211_019, 0.007_194_665]);
    // Table 2: the sphere's constant drag coefficient on its reference area.
    let drag = Load::Drag {
        area_m2: 0.196_349_5 * FOOT_M * FOOT_M,
        coefficient: 0.1,
    };
    // Dropped at rest relative to the Earth: the roll rate relative to the local frame is the
    // Earth's, taken away, so the inertial rate is zero.
    let dropped = Start {
        height_m: START_HEIGHT_M,
        velocity_ned_ft_s: [0.0; 3],
        attitude: start_attitude(),
        rates_deg_s: [-0.004_178_073, 0.0, 0.0],
    };
    let tumbling = Start {
        rates_deg_s: [9.995_821_927, 20.0, 30.0],
        ..dropped
    };
    // Cases 9 and 10: from sea level, 1,000 ft/s up and 1,000 ft/s along the ground, the body
    // level and heading that way, not turning relative to the Earth.
    let shot = Start {
        height_m: 0.0,
        velocity_ned_ft_s: [1000.0, 0.0, -1000.0],
        attitude: start_attitude(),
        rates_deg_s: [0.0; 3],
    };
    let sphere_case = |number, name, folder, tables, air, start| -> Result<Case, String> {
        Ok(Case {
            number,
            name,
            folder,
            tables,
            body: sphere()?,
            load: drag,
            air,
            start,
            measure: Measure::Position,
            trace: Trace::GravityOnTheFlight,
            model_difference: Some(
                "WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less \
                 gravitation cut off after J2 with the centrifugal term and the code's own air \
                 density as its file gives it, which the code flies in: a point mass flown in \
                 each, with the same drag and wind, one position less the other",
            ),
            quantities: position(),
        })
    };
    Ok(vec![
        Case {
            number: 1,
            name: "the dragless sphere",
            folder: "Atmos_01_DroppedSphere",
            tables: "Volume II, Tables 1 and 25",
            body: sphere()?,
            load: Load::None,
            air: Air::Still,
            start: dropped,
            measure: Measure::Altitude,
            trace: Trace::Gravity,
            model_difference: Some(
                "WGS 84's normal gravity, which HPR Sim falls in, less gravitation cut off \
                 after J2 with the centrifugal term, which the codes fall in, integrated twice \
                 along the fall",
            ),
            quantities: vec![Quantity {
                key: "altitude",
                name: "altitude",
                reading: Reading::Column {
                    column: "altitudeMsl_ft",
                    factor: FOOT_M,
                },
                unit: "m",
            }],
        },
        Case {
            number: 2,
            name: "the tumbling brick",
            folder: "Atmos_02_TumblingBrickNoDamping",
            tables: "Volume II, Tables 4 and 26, with the errata's 30,000 ft",
            body: brick()?,
            load: Load::None,
            air: Air::Still,
            start: tumbling,
            measure: Measure::InertialRates,
            trace: Trace::EarthRate,
            model_difference: Some(
                "the Earth's rate left out of HPR Sim's rotational equations: Euler's \
                 equations in the rate relative to the Earth-fixed launch frame, less the same \
                 equations in the rate relative to the stars",
            ),
            quantities: rates(),
        },
        Case {
            number: 3,
            name: "the tumbling brick with damping",
            folder: "Atmos_03_TumblingBrickDamping",
            tables: "Volume II, Tables 4, 5 and 27, with the errata's 30,000 ft",
            body: brick()?,
            // Table 5.
            load: Load::Damping {
                area_m2: 0.222_22 * FOOT_M * FOOT_M,
                span_m: 0.333_33 * FOOT_M,
                chord_m: 0.666_67 * FOOT_M,
                coefficients: [-1.0, -1.0, -1.0],
            },
            air: Air::Still,
            start: tumbling,
            measure: Measure::InertialRates,
            trace: Trace::EarthRate,
            model_difference: Some(
                "the Earth's rate left out of HPR Sim's rotational equations: Euler's \
                 equations with the damping moment in the rate relative to the Earth-fixed \
                 launch frame, less the same equations in the rate relative to the stars, both \
                 damped by the inertial rate along the same fall",
            ),
            quantities: rates(),
        },
        sphere_case(
            6,
            "the sphere with drag",
            "Atmos_06_DroppedSphereEllipsoidalNoWind",
            "Volume II, Tables 1, 2 and 30",
            Air::Still,
            dropped,
        )?,
        sphere_case(
            7,
            "the sphere in a steady wind",
            "Atmos_07_DroppedSphereSteadyWind",
            "Volume II, Tables 1, 2 and 31",
            Air::Steady,
            dropped,
        )?,
        sphere_case(
            8,
            "the sphere in a wind that varies with height",
            "Atmos_08_DroppedSphere2DWindShear",
            "Volume II, Tables 1, 2 and 32",
            Air::Shear,
            dropped,
        )?,
        sphere_case(
            9,
            "the sphere shot east",
            "Atmos_09_EastwardCannonball",
            "Volume II, Tables 1, 2 and 33",
            Air::Still,
            // Table 33: east, the body's x axis east (yaw 90°): y south, z down.
            Start {
                velocity_ned_ft_s: [0.0, 1000.0, -1000.0],
                attitude: DQuat::from_mat3(&DMat3::from_cols(DVec3::X, -DVec3::Y, -DVec3::Z)),
                ..shot
            },
        )?,
        sphere_case(
            10,
            "the sphere shot north",
            "Atmos_10_NorthwardCannonball",
            "Volume II, Tables 1, 2 and 34",
            Air::Still,
            shot,
        )?,
    ])
}

/// The cases' site, 0° N, 0° E on the ellipsoid, on WGS 84 and the 1976 atmosphere, with the
/// case's wind.
fn environment(air: Air) -> Result<Environment, String> {
    let site = Geodetic::from_degrees(0.0, 0.0, 0.0).map_err(|e| e.to_string())?;
    let still = Environment::standard(site).map_err(|e| e.to_string())?;
    let from_west = 270.0_f64.to_radians();
    Ok(match air {
        Air::Still => still,
        Air::Steady => {
            still.with_wind(ConstantWind::new(20.0 * FOOT_M, from_west).map_err(|e| e.to_string())?)
        }
        Air::Shear => {
            // Linear in its east component between levels, so exact between the calm level,
            // where Table 32's formula crosses zero, and one above the case's start: the sphere
            // stays between the two.
            let level = |height_ft: f64| WindLevel {
                height_msl_m: height_ft * FOOT_M,
                speed_m_s: shear_ft_s(height_ft) * FOOT_M,
                direction_from_rad: from_west,
            };
            let wind = LayeredWind::new(
                vec![level(20.0 / 0.003), level(30_000.0), level(40_000.0)],
                WindInterpolation::Components,
            )
            .map_err(|e| e.to_string())?;
            still.with_wind(wind)
        }
    })
}

fn start_state(case: &Case) -> State {
    let [north, east, down] = case.start.velocity_ned_ft_s;
    State {
        position_enu_m: DVec3::new(0.0, 0.0, case.start.height_m),
        velocity_enu_m_s: DVec3::new(east, north, -down) * FOOT_M,
        attitude: case.start.attitude,
        body_rate_rad_s: DVec3::from_array(case.start.rates_deg_s.map(f64::to_radians)),
    }
}

/// One code's file: its name and, by time in milliseconds, its value of each compared quantity
/// (one number, or a position's three).
struct History {
    code: String,
    samples: BTreeMap<i64, Vec<Vec<f64>>>,
    /// For a position's case, the code's own air by time: its altitude, m, and its air density
    /// there, kg/m³.
    air: BTreeMap<i64, [f64; 2]>,
    /// The file as read, for the readings of its other columns ([`table`]).
    text: String,
}

impl History {
    fn times_s(&self) -> Vec<f64> {
        self.samples.keys().map(|&ms| ms as f64 / 1000.0).collect()
    }
}

/// A time as the key the files share: whole milliseconds.
fn key(time_s: f64) -> i64 {
    (time_s * 1000.0).round() as i64
}

/// Reads a code's file: the time and each quantity, by its columns' names, over the case's
/// duration. Rows are numbered as the file numbers its lines, blank ones included.
fn history(
    code: &str,
    text: &str,
    quantities: &[Quantity],
    ellipsoid: &Ellipsoid,
) -> Result<History, String> {
    let mut lines = text.lines().enumerate();
    let header: Vec<&str> = lines
        .next()
        .filter(|(_, line)| !line.trim().is_empty())
        .ok_or_else(|| format!("{code}: empty file"))?
        .1
        .split(',')
        .map(str::trim)
        .collect();
    let column = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .ok_or_else(|| format!("{code}: no column `{name}`"))
    };
    let time = column("time")?;
    // A position's case sizes its trace on the code's own air density.
    let air_columns = if quantities.iter().any(Quantity::is_distance) {
        Some([column("altitudeMsl_ft")?, column("airDensity_slug_ft3")?])
    } else {
        None
    };
    let columns = quantities
        .iter()
        .map(|q| match q.reading {
            Reading::Column { column: name, .. } => Ok(vec![column(name)?]),
            Reading::Position => Ok(vec![
                column("latitude_deg")?,
                column("longitude_deg")?,
                column("altitudeMsl_ft")?,
            ]),
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut samples = BTreeMap::new();
    let mut air = BTreeMap::new();
    for (index, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let row = index + 1;
        let cells: Vec<&str> = line.split(',').collect();
        let number = |index: usize| -> Result<f64, String> {
            let cell = cells
                .get(index)
                .ok_or_else(|| format!("{code}: row {row} is short"))?;
            let value: f64 = cell
                .trim()
                .parse()
                .map_err(|_| format!("{code}: row {row}: `{cell}` is no number"))?;
            if value.is_finite() {
                Ok(value)
            } else {
                Err(format!("{code}: row {row}: `{cell}` is not finite"))
            }
        };
        let time_s = number(time)?;
        if !(0.0..=DURATION_S + 0.0005).contains(&time_s) {
            continue;
        }
        let mut values = Vec::with_capacity(quantities.len());
        for (indices, quantity) in columns.iter().zip(quantities) {
            let read = indices
                .iter()
                .map(|&index| number(index))
                .collect::<Result<Vec<_>, _>>()?;
            values.push(match quantity.reading {
                Reading::Column { factor, .. } => vec![read[0] * factor],
                Reading::Position => {
                    let place = Geodetic::from_degrees(read[0], read[1], read[2] * FOOT_M)
                        .map_err(|e| format!("{code}: row {row}: {e}"))?;
                    ellipsoid.ecef_from_geodetic(place).to_array().to_vec()
                }
            });
        }
        if samples.insert(key(time_s), values).is_some() {
            return Err(format!("{code}: the time {time_s} s comes twice"));
        }
        if let Some([height, density]) = air_columns {
            air.insert(
                key(time_s),
                [
                    number(height)? * FOOT_M,
                    number(density)? * SLUG_KG / FOOT_M.powi(3),
                ],
            );
        }
    }
    let (Some((&first, _)), Some((&last, _))) =
        (samples.first_key_value(), samples.last_key_value())
    else {
        return Err(format!("{code}: no rows"));
    };
    if first != 0 || last != key(DURATION_S) {
        return Err(format!(
            "{code}: the file runs from {} to {} ms, not 0 to {DURATION_S} s",
            first, last
        ));
    }
    Ok(History {
        code: code.to_owned(),
        samples,
        air,
        text: text.to_owned(),
    })
}

/// A file's rows of the named columns, with each row's time, from the start to the case's end;
/// `None` if a column is missing or a cell is not a finite number. For readings beside the
/// comparison, which a code's file may not support.
fn table(text: &str, names: &[&str]) -> Option<Vec<(f64, Vec<f64>)>> {
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());
    let header: Vec<&str> = lines.next()?.split(',').map(str::trim).collect();
    let index = |name: &str| header.iter().position(|h| *h == name);
    let time = index("time")?;
    let columns = names.iter().map(|n| index(n)).collect::<Option<Vec<_>>>()?;
    let mut rows = Vec::new();
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        let number = |i: usize| {
            cells
                .get(i)?
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
        };
        let t = number(time)?;
        if !(0.0..=DURATION_S + 0.0005).contains(&t) {
            continue;
        }
        rows.push((
            t,
            columns
                .iter()
                .map(|&i| number(i))
                .collect::<Option<Vec<_>>>()?,
        ));
    }
    Some(rows)
}

/// The largest relative difference, about each axis, between a code's printed aerodynamic
/// moments and case 3's damping law ([`Load::Damping`]) on its own printed density, speed and
/// rates over the first 8 s, while the moments are large: on the inertial rate, and on the rate
/// relative to the Earth (the inertial rate less the Earth's, turned into the body by the
/// code's own Euler angles and latitude). `None` if the file lacks a column.
fn damping_law_fit(
    case: &Case,
    environment: &Environment,
    history: &History,
) -> Option<[[f64; 3]; 2]> {
    let Load::Damping {
        area_m2,
        span_m,
        chord_m,
        coefficients,
    } = case.load
    else {
        return None;
    };
    let rows = table(
        &history.text,
        &[
            "aero_bodyMoment_ftlbf_L",
            "aero_bodyMoment_ftlbf_M",
            "aero_bodyMoment_ftlbf_N",
            "airDensity_slug_ft3",
            "feVelocity_ft_s_X",
            "feVelocity_ft_s_Y",
            "feVelocity_ft_s_Z",
            "bodyAngularRateWrtEi_deg_s_Roll",
            "bodyAngularRateWrtEi_deg_s_Pitch",
            "bodyAngularRateWrtEi_deg_s_Yaw",
            "eulerAngle_deg_Yaw",
            "eulerAngle_deg_Pitch",
            "eulerAngle_deg_Roll",
            "latitude_deg",
        ],
    )?;
    let omega = environment.earth.field().angular_velocity_rad_s();
    let lengths = DVec3::new(span_m, chord_m, span_m);
    let per_rate = lengths * lengths * DVec3::from_array(coefficients) * (0.25 * area_m2);
    let mut worst = [[0.0_f64; 3]; 2];
    for (t, r) in rows {
        if !(t > 0.0 && t <= 8.0 + 0.0005) {
            continue;
        }
        let moment = DVec3::new(r[0], r[1], r[2]) * (POUND_FORCE_N * FOOT_M);
        let density = r[3] * SLUG_KG / FOOT_M.powi(3);
        let speed = DVec3::new(r[4], r[5], r[6]).length() * FOOT_M;
        let inertial = DVec3::new(r[7], r[8], r[9]) * (1.0_f64.to_radians());
        // North-East-Down to body, by yaw, pitch and roll in turn.
        let (yaw, pitch, roll) = (r[10].to_radians(), r[11].to_radians(), r[12].to_radians());
        let to_body = DQuat::from_rotation_x(roll).conjugate()
            * DQuat::from_rotation_y(pitch).conjugate()
            * DQuat::from_rotation_z(yaw).conjugate();
        let latitude = r[13].to_radians();
        let earth_ned = DVec3::new(omega * latitude.cos(), 0.0, -omega * latitude.sin());
        let relative = inertial - to_body.mul_vec3(earth_ned);
        for (at, rate) in [inertial, relative].into_iter().enumerate() {
            let law = per_rate * rate * (density * speed);
            for axis in 0..3 {
                if moment[axis].abs() > 1e-6 * POUND_FORCE_N * FOOT_M {
                    let off = (law[axis] / moment[axis] - 1.0).abs();
                    worst[at][axis] = worst[at][axis].max(off);
                }
            }
        }
    }
    Some(worst)
}

/// The largest relative difference between a code's printed drag force and `½ρV²SC_D` on its own
/// printed density and speed, in still air, where the speed through the air is the speed over
/// the ground. `None` if the case has wind or the file lacks a column.
fn drag_fit(case: &Case, history: &History) -> Option<f64> {
    let Load::Drag {
        area_m2,
        coefficient,
    } = case.load
    else {
        return None;
    };
    if case.air != Air::Still {
        return None;
    }
    let rows = table(
        &history.text,
        &[
            "aero_bodyForce_lbf_X",
            "aero_bodyForce_lbf_Y",
            "aero_bodyForce_lbf_Z",
            "airDensity_slug_ft3",
            "feVelocity_ft_s_X",
            "feVelocity_ft_s_Y",
            "feVelocity_ft_s_Z",
        ],
    )?;
    let mut worst = 0.0_f64;
    for (t, r) in rows {
        if t <= 0.0 {
            continue;
        }
        let force = DVec3::new(r[0], r[1], r[2]).length() * POUND_FORCE_N;
        let density = r[3] * SLUG_KG / FOOT_M.powi(3);
        let speed = DVec3::new(r[4], r[5], r[6]).length() * FOOT_M;
        let law = 0.5 * density * speed * speed * area_m2 * coefficient;
        if law > 0.0 {
            worst = worst.max((force / law - 1.0).abs());
        }
    }
    Some(worst)
}

/// Every code's file of a case, from the archive, in the codes' order. Two files that name the
/// same code are refused, as is a case with fewer than three.
fn histories(archive: &[u8], case: &Case, ellipsoid: &Ellipsoid) -> Result<Vec<History>, String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive))
        .map_err(|e| format!("{ARCHIVE}: {e}"))?;
    let prefix = format!("Atmospheric_checkcases/{}/", case.folder);
    let mut files: BTreeMap<u32, (String, String)> = BTreeMap::new();
    for index in 0..zip.len() {
        let mut entry = zip
            .by_index(index)
            .map_err(|e| format!("{ARCHIVE}: entry {index}: {e}"))?;
        let name = entry
            .name()
            .map_err(|e| format!("{ARCHIVE}: entry {index}: {e}"))?
            .into_owned();
        if !(name.starts_with(&prefix) && name.ends_with(".csv")) {
            continue;
        }
        // `Atmos_01_sim_05.csv` is SIM 5's.
        let code = name
            .rsplit_once("_sim_")
            .and_then(|(_, rest)| rest.strip_suffix(".csv"))
            .and_then(|digits| digits.parse::<u32>().ok())
            .ok_or_else(|| format!("{name}: not a code's file by its name"))?;
        let mut text = String::new();
        entry
            .read_to_string(&mut text)
            .map_err(|e| format!("{name}: {e}"))?;
        if let Some((other, _)) = files.insert(code, (name.clone(), text)) {
            return Err(format!("{name} and {other} are both SIM {code}'s"));
        }
    }
    let mut out = Vec::new();
    for (code, (_, text)) in files {
        out.push(history(
            &format!("SIM {code}"),
            &text,
            &case.quantities,
            ellipsoid,
        )?);
    }
    if out.len() < 3 {
        return Err(format!(
            "case {}: {} files in the archive; the report published a case on three codes or more",
            case.number,
            out.len()
        ));
    }
    Ok(out)
}

/// HPR Sim's values of the case's quantities at `times_s`, flown by `method`.
fn flown(
    case: &Case,
    environment: &Environment,
    method: Method,
    times_s: &[f64],
) -> Result<Vec<Vec<Vec<f64>>>, String> {
    let earth_rate = environment.earth.angular_velocity_enu_rad_s();
    let loads = TestLoads {
        load: case.load,
        earth_rate_enu_rad_s: earth_rate,
    };
    let states = case
        .body
        .fly(environment, &loads, method, 0.0, start_state(case), times_s)
        .map_err(|e| format!("case {}: {e}", case.number))?;
    let frame = environment.earth.frame();
    states
        .iter()
        .map(|state| {
            Ok(match case.measure {
                Measure::Altitude => vec![vec![
                    frame
                        .geodetic_from_enu(state.position_enu_m)
                        .map_err(|e| e.to_string())?
                        .height_m,
                ]],
                Measure::InertialRates => {
                    // The rate relative to the stars, in body axes: the state's rate relative to
                    // the Earth-fixed launch frame plus the Earth's own, turned into the body.
                    let inertial = state.body_rate_rad_s
                        + state.unit_attitude().conjugate().mul_vec3(earth_rate);
                    inertial
                        .to_array()
                        .map(|rate| vec![rate.to_degrees()])
                        .to_vec()
                }
                Measure::Position => {
                    vec![
                        frame
                            .ecef_from_enu(state.position_enu_m)
                            .to_array()
                            .to_vec(),
                    ]
                }
            })
        })
        .collect()
}

/// `a` less `b`: signed for one number, and for a position the distance between the two.
fn less(a: &[f64], b: &[f64]) -> f64 {
    match (a, b) {
        ([a], [b]) => a - b,
        _ => a
            .iter()
            .zip(b)
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt(),
    }
}

/// `a` less `b` less `c`, the same way.
fn less_both(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    let ab: Vec<f64> = a.iter().zip(b).map(|(a, b)| a - b).collect();
    less(&ab, c)
}

/// The largest difference between any two of `histories` in quantity `index`, over the times
/// every one of them has: for one number the highest less the lowest, for a position the
/// largest distance between two.
fn spread(histories: &[&History], index: usize) -> f64 {
    let Some((first, rest)) = histories.split_first() else {
        return 0.0;
    };
    let mut worst: f64 = 0.0;
    for (time, values) in &first.samples {
        let mut at = vec![&values[index]];
        for other in rest {
            match other.samples.get(time) {
                Some(values) => at.push(&values[index]),
                None => break,
            }
        }
        if at.len() < histories.len() {
            continue;
        }
        for (i, a) in at.iter().enumerate() {
            for b in &at[i + 1..] {
                worst = worst.max(less(a, b).abs());
            }
        }
    }
    worst
}

/// The codes' spread in one quantity: all of them, and the rest when leaving one out cuts it by
/// more than half.
struct Spread {
    all: f64,
    left_out: Option<usize>,
    kept: f64,
}

/// The rule of the report: the code left out is the one whose leaving cuts the spread most, and
/// only if that cuts it by more than half. Two codes must remain for a spread, so with two none
/// is left out.
fn spread_rule(histories: &[History], index: usize) -> Spread {
    let all: Vec<&History> = histories.iter().collect();
    let whole = spread(&all, index);
    let mut best: Option<(usize, f64)> = None;
    if histories.len() > 2 {
        for out in 0..histories.len() {
            let rest: Vec<&History> = histories
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != out)
                .map(|(_, h)| h)
                .collect();
            let without = spread(&rest, index);
            if best.is_none_or(|(_, least)| without < least) {
                best = Some((out, without));
            }
        }
    }
    match best {
        Some((out, without)) if without < 0.5 * whole => Spread {
            all: whole,
            left_out: Some(out),
            kept: without,
        },
        _ => Spread {
            all: whole,
            left_out: None,
            kept: whole,
        },
    }
}

/// The largest `|a − b|` over two series of the same length, and the time it comes at.
fn largest(times_s: &[f64], differences: impl Iterator<Item = f64>) -> (f64, f64) {
    let mut worst = (0.0_f64, 0.0);
    for (time, difference) in times_s.iter().zip(differences) {
        if difference.abs() > worst.0.abs() {
            worst = (difference, *time);
        }
    }
    worst
}

/// The codes' gravitation with the centrifugal term, in Earth-fixed axes, m/s²: the gradient of
/// the report's eq. 24, `U = (μ/r)(1 − (J₂/2)(a/r)²(3 sin²φ − 1))`, plus `ω²(x, y, 0)`.
fn j2_gravity_ecef(field: &NormalGravity, p: DVec3) -> DVec3 {
    let (mu, a, omega) = (
        field.gm_m3_s2(),
        field.ellipsoid().semi_major_axis_m(),
        field.angular_velocity_rad_s(),
    );
    let r = p.length();
    let k = 1.5 * J2 * (a / r).powi(2);
    let s2 = (p.z / r).powi(2);
    let across = 1.0 + k * (1.0 - 5.0 * s2);
    let along = 1.0 + k * (3.0 - 5.0 * s2);
    DVec3::new(p.x * across, p.y * across, p.z * along) * (-mu / r.powi(3))
        + DVec3::new(p.x, p.y, 0.0) * (omega * omega)
}

/// The codes' fall and HPR Sim's gap from it, down the site's vertical: the upward acceleration
/// in the codes' field at `height_m`, and HPR Sim's normal gravity's less that.
fn fall_and_gap(environment: &Environment, height_m: f64) -> Result<[f64; 2], String> {
    let field = environment.earth.field();
    let frame = environment.earth.frame();
    let up_ecef = frame.ecef_from_enu_rotation() * DVec3::Z;
    let p = frame.ecef_from_enu(DVec3::new(0.0, 0.0, height_m));
    let codes = j2_gravity_ecef(field, p).dot(up_ecef);
    let normal = field.ecef_mps2(p).map_err(|e| e.to_string())?.dot(up_ecef);
    Ok([codes, normal - codes])
}

/// Case 1's named difference between the models, computed apart from the flight: HPR Sim falls in
/// WGS 84's normal gravity, whose closed form holds every even zonal harmonic of the level
/// ellipsoid, and the codes in gravitation cut off after `J₂`. The gap in the downward
/// acceleration is taken along the codes' own fall (a point dropped down the site's vertical in
/// their field, by RK4 at 1 ms) and integrated twice. Returns HPR Sim's altitude less the
/// codes', m, at each of `times_s`.
fn gravity_gap_m(environment: &Environment, times_s: &[f64]) -> Result<Vec<f64>, String> {
    // State: the codes' height and climb rate, and the altitude gap and its rate.
    let derivative = |y: [f64; 4]| -> Result<[f64; 4], String> {
        let [fall, gap] = fall_and_gap(environment, y[0])?;
        Ok([y[1], fall, y[3], gap])
    };
    let step = 0.001;
    let mut y = [START_HEIGHT_M, 0.0, 0.0, 0.0];
    let mut out = Vec::with_capacity(times_s.len());
    let mut done: i64 = 0;
    for &time in times_s {
        let target = key(time);
        while done < target {
            let add = |y: [f64; 4], k: [f64; 4], h: f64| [0, 1, 2, 3].map(|i| y[i] + h * k[i]);
            let k1 = derivative(y)?;
            let k2 = derivative(add(y, k1, 0.5 * step))?;
            let k3 = derivative(add(y, k2, 0.5 * step))?;
            let k4 = derivative(add(y, k3, step))?;
            y = [0, 1, 2, 3]
                .map(|i| y[i] + step / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]));
            done += 1;
        }
        out.push(y[2]);
    }
    Ok(out)
}

/// One step of the classic fourth-order method on `y` by `step`.
fn rk4<const N: usize>(
    t: f64,
    y: [f64; N],
    step: f64,
    derivative: impl Fn(f64, [f64; N]) -> Result<[f64; N], String>,
) -> Result<[f64; N], String> {
    let add = |k: [f64; N], h: f64| std::array::from_fn(|i| y[i] + h * k[i]);
    let k1 = derivative(t, y)?;
    let k2 = derivative(t + 0.5 * step, add(k1, 0.5 * step))?;
    let k3 = derivative(t + 0.5 * step, add(k2, 0.5 * step))?;
    let k4 = derivative(t + step, add(k3, step))?;
    Ok(std::array::from_fn(|i| {
        y[i] + step / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i])
    }))
}

/// The code's air density over HPR Sim's at the code's own altitude, at one of its samples.
fn sample_ratio(environment: &Environment, [height, density]: [f64; 2]) -> Result<f64, String> {
    let ours = environment
        .atmosphere
        .air(height - environment.geoid_undulation_m)
        .map_err(|e| e.to_string())?
        .air
        .density_kg_m3;
    Ok(density / ours)
}

/// The code's air density over HPR Sim's at the code's own altitude, at `time_s`: linear
/// between the file's samples, held past its ends.
fn density_ratio(environment: &Environment, history: &History, time_s: f64) -> Result<f64, String> {
    let ratio = |sample: [f64; 2]| sample_ratio(environment, sample);
    let at = (time_s * 1000.0).round() as i64;
    let below = history.air.range(..=at).next_back();
    let above = history.air.range(at..).next();
    match (below, above) {
        (Some((&t0, &a)), Some((&t1, &b))) if t1 > t0 => {
            let w = (time_s * 1000.0 - t0 as f64) / (t1 - t0) as f64;
            let (r0, r1) = (ratio(a)?, ratio(b)?);
            Ok(r0 + w * (r1 - r0))
        }
        (Some((_, &a)), _) | (None, Some((_, &a))) => ratio(a),
        (None, None) => Err(format!("{}: no air density", history.code)),
    }
}

/// The named difference of cases 6 to 10 between the models, computed apart from the flight:
/// the gravity and the air. A point mass of the case's mass is flown twice in the Earth-fixed
/// launch frame, from the case's start, by RK4 at 1 ms, each with the Coriolis force and the
/// case's drag and wind ([`Load::Drag`]): once as the code flies, in gravitation cut off after
/// `J₂` with the centrifugal term and through the code's own air density, as its file gives it
/// (HPR Sim's density at the height flown, times the code's density over HPR Sim's at the
/// code's own altitude and time); once as HPR Sim flies, in WGS 84's normal gravity and the
/// 1976 atmosphere. Returns the second's position less the first's, in Earth-fixed axes, m, at
/// each of `times_s`. It flies no rigid body and none of HPR Sim's flight code.
///
/// With `own_air` false, the code's side flies HPR Sim's air too, which leaves the gravity alone.
fn point_mass_gap_m(
    case: &Case,
    environment: &Environment,
    history: &History,
    own_air: bool,
    times_s: &[f64],
) -> Result<Vec<Vec<f64>>, String> {
    let Load::Drag {
        area_m2,
        coefficient,
    } = case.load
    else {
        return Err(format!(
            "case {}: the gravity gap along a flight is sized for a drag",
            case.number
        ));
    };
    let mass = case.body.mass_kg();
    let field = environment.earth.field();
    let frame = environment.earth.frame();
    let to_ecef = frame.ecef_from_enu_rotation();
    let to_enu = to_ecef.transpose();
    let earth_rate = environment.earth.angular_velocity_enu_rad_s();
    // State: the position and velocity in the launch frame, as the code flies (0 to 5) and as
    // HPR Sim flies (6 to 11).
    let accelerate = |t: f64, p: DVec3, v: DVec3, ours: bool| -> Result<DVec3, String> {
        let ecef = frame.ecef_from_enu(p);
        let gravity = if ours {
            field.ecef_mps2(ecef).map_err(|e| e.to_string())?
        } else {
            j2_gravity_ecef(field, ecef)
        };
        let height = frame
            .geodetic_from_enu(p)
            .map_err(|e| e.to_string())?
            .height_m
            - environment.geoid_undulation_m;
        let mut density = environment
            .atmosphere
            .air(height)
            .map_err(|e| e.to_string())?
            .air
            .density_kg_m3;
        if own_air && !ours {
            density *= density_ratio(environment, history, t)?;
        }
        let wind = environment
            .wind
            .wind(height)
            .map_err(|e| e.to_string())?
            .velocity_enu_m_s;
        let through_air = v - wind;
        let drag = through_air * (-0.5 * density * through_air.length() * area_m2 * coefficient);
        Ok(to_enu * gravity - 2.0 * earth_rate.cross(v) + drag / mass)
    };
    let derivative = |t: f64, y: [f64; 12]| -> Result<[f64; 12], String> {
        let mut out = [0.0; 12];
        for (at, ours) in [(0, false), (6, true)] {
            let p = DVec3::new(y[at], y[at + 1], y[at + 2]);
            let v = DVec3::new(y[at + 3], y[at + 4], y[at + 5]);
            let a = accelerate(t, p, v, ours)?;
            out[at..at + 6].copy_from_slice(&[v.x, v.y, v.z, a.x, a.y, a.z]);
        }
        Ok(out)
    };
    let start = start_state(case);
    let (p, v) = (start.position_enu_m, start.velocity_enu_m_s);
    let one = [p.x, p.y, p.z, v.x, v.y, v.z];
    let mut y = [0.0; 12];
    y[..6].copy_from_slice(&one);
    y[6..].copy_from_slice(&one);
    let step = 0.001;
    let mut out = Vec::with_capacity(times_s.len());
    let mut done: i64 = 0;
    for &time in times_s {
        let target = key(time);
        while done < target {
            y = rk4(done as f64 * step, y, step, derivative)?;
            done += 1;
        }
        let gap = DVec3::new(y[6] - y[0], y[7] - y[1], y[8] - y[2]);
        out.push((to_ecef * gap).to_array().to_vec());
    }
    Ok(out)
}

/// Euler's equations for a body of principal moments `moments` under a moment `moment`:
/// `ω̇ = I⁻¹(M − ω × Iω)`.
fn euler_rate(moments: DVec3, omega: DVec3, moment: DVec3) -> DVec3 {
    (moment - omega.cross(moments * omega)) / moments
}

/// At one sample, the codes' model's inertial rates and HPR Sim's model's less those, deg/s.
type RatesAndGap = ([f64; 3], [f64; 3]);

/// The two rotational models of cases 2 and 3, and the fall that sets case 3's damping.
#[derive(Clone, Copy)]
struct Rotations {
    /// The codes' height above the ellipsoid and climb rate, m and m/s, down the site's
    /// vertical in their field.
    fall: [f64; 2],
    /// The codes' model: the rate relative to the stars, rad/s.
    inertial: DVec3,
    /// HPR Sim's model: the rate relative to the launch frame, rad/s, and the attitude in it.
    relative: DVec3,
    attitude: DQuat,
}

impl Rotations {
    fn plus(&self, rate: &Self, step: f64) -> Self {
        Self {
            fall: [
                self.fall[0] + step * rate.fall[0],
                self.fall[1] + step * rate.fall[1],
            ],
            inertial: self.inertial + rate.inertial * step,
            relative: self.relative + rate.relative * step,
            attitude: self.attitude + rate.attitude * step,
        }
    }

    /// The classic fourth-order step from the four slopes, `y + (k₁ + 2k₂ + 2k₃ + k₄) h/6`,
    /// the attitude normalized after.
    fn rk4(y: &Self, [k1, k2, k3, k4]: [&Self; 4], step: f64) -> Self {
        let fall = [0, 1].map(|i| {
            y.fall[i] + step / 6.0 * (k1.fall[i] + 2.0 * k2.fall[i] + 2.0 * k3.fall[i] + k4.fall[i])
        });
        Self {
            fall,
            inertial: y.inertial
                + (k1.inertial + k2.inertial * 2.0 + k3.inertial * 2.0 + k4.inertial)
                    * (step / 6.0),
            relative: y.relative
                + (k1.relative + k2.relative * 2.0 + k3.relative * 2.0 + k4.relative)
                    * (step / 6.0),
            attitude: (y.attitude
                + (k1.attitude + k2.attitude * 2.0 + k3.attitude * 2.0 + k4.attitude)
                    * (step / 6.0))
                .normalize(),
        }
    }
}

/// The named difference of cases 2 and 3 between the models, computed apart from the flight.
/// The codes turn the brick by Euler's equations in its rate relative to the stars. HPR Sim's
/// rotational equations leave the Earth's rate out (ADR-011): they turn it by the same equations
/// in its rate relative to the Earth-fixed launch frame, and the inertial rate is that plus the
/// Earth's, turned into the body. In case 3 both are damped by the same law in the inertial rate
/// ([`Load::Damping`]), along the codes' own fall with no drag (as in [`gravity_gap_m`]) through
/// the case's atmosphere. All of it is integrated here by RK4 at 1 ms from the report's start.
/// Returns, at each of `times_s`, the codes' model's inertial rates and HPR Sim's model's less
/// those, deg/s.
fn earth_rate_gap_deg_s(
    case: &Case,
    environment: &Environment,
    times_s: &[f64],
) -> Result<Vec<RatesAndGap>, String> {
    let inertia = case.body.inertia_kg_m2();
    let moments = DVec3::new(inertia.x_axis.x, inertia.y_axis.y, inertia.z_axis.z);
    let earth_rate = environment.earth.angular_velocity_enu_rad_s();
    let start = start_state(case);
    let inertial_of = |q: DQuat, relative: DVec3| relative + q.conjugate().mul_vec3(earth_rate);
    // The damping moment per unit inertial rate, `¼ ρ V S l² C` on each axis, at a point of the
    // fall; zero with no damping.
    let damping = |fall: [f64; 2]| -> Result<DVec3, String> {
        match case.load {
            Load::Damping {
                area_m2,
                span_m,
                chord_m,
                coefficients,
            } => {
                let density = environment
                    .atmosphere
                    .air(fall[0] - environment.geoid_undulation_m)
                    .map_err(|e| e.to_string())?
                    .air
                    .density_kg_m3;
                let lengths = DVec3::new(span_m, chord_m, span_m);
                Ok(lengths
                    * lengths
                    * DVec3::from_array(coefficients)
                    * (0.25 * density * fall[1].abs() * area_m2))
            }
            Load::None => Ok(DVec3::ZERO),
            Load::Drag { .. } => Err(format!(
                "case {}: the Earth's rate gap is not sized for a drag",
                case.number
            )),
        }
    };
    let q_dot = |q: DQuat, omega: DVec3| q * DQuat::from_xyzw(omega.x, omega.y, omega.z, 0.0) * 0.5;
    let derivative = |y: &Rotations| -> Result<Rotations, String> {
        let [fall, _] = fall_and_gap(environment, y.fall[0])?;
        let per_rate = damping(y.fall)?;
        let hpr_inertial = inertial_of(y.attitude, y.relative);
        Ok(Rotations {
            fall: [y.fall[1], fall],
            inertial: euler_rate(moments, y.inertial, per_rate * y.inertial),
            relative: euler_rate(moments, y.relative, per_rate * hpr_inertial),
            attitude: q_dot(y.attitude, y.relative),
        })
    };
    let mut y = Rotations {
        fall: [start.position_enu_m.z, 0.0],
        inertial: inertial_of(start.attitude, start.body_rate_rad_s),
        relative: start.body_rate_rad_s,
        attitude: start.attitude,
    };
    let step = 0.001;
    let mut out = Vec::with_capacity(times_s.len());
    let mut done: i64 = 0;
    for &time in times_s {
        let target = key(time);
        while done < target {
            let k1 = derivative(&y)?;
            let k2 = derivative(&y.plus(&k1, 0.5 * step))?;
            let k3 = derivative(&y.plus(&k2, 0.5 * step))?;
            let k4 = derivative(&y.plus(&k3, step))?;
            y = Rotations::rk4(&y, [&k1, &k2, &k3, &k4], step);
            done += 1;
        }
        let hpr = inertial_of(y.attitude, y.relative);
        out.push((
            y.inertial.to_array().map(f64::to_degrees),
            (hpr - y.inertial).to_array().map(f64::to_degrees),
        ));
    }
    Ok(out)
}

/// The exponent of `value` in scientific notation, from its printed form rather than a
/// logarithm, so that every platform counts the same digits.
fn exponent(value: f64) -> i32 {
    format!("{value:e}")
        .split_once('e')
        .and_then(|(_, power)| power.parse().ok())
        .unwrap_or(0)
}

/// A figure as the report keeps it: four significant digits, so that a rounding in the last
/// place of a difference of nearly equal numbers doesn't make the report stale.
fn kept(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let scale = 10f64.powi(3 - exponent(value));
    (value * scale).round() / scale
}

/// The tolerances of the second flight, a thousand and more times tighter than the default's
/// 1e-8: what is left against the codes there is not the integrator's step error.
const TIGHT_TOLERANCE: f64 = 1e-12;

fn tight() -> Method {
    Method::DormandPrince54(Adaptive {
        relative_tolerance: TIGHT_TOLERANCE,
        absolute_tolerance: TIGHT_TOLERANCE,
        ..Adaptive::default()
    })
}

/// HPR Sim against one code's file, in every compared quantity: at each sample, each
/// quantity's value.
struct Against {
    times_s: Vec<f64>,
    /// HPR Sim's flight at the default method, and at the tight tolerances.
    flown: Vec<Vec<Vec<f64>>>,
    tight: Vec<Vec<Vec<f64>>>,
    /// The codes' model as this file integrates it (cases 2 and 3).
    model: Option<Vec<Vec<Vec<f64>>>>,
    /// The named model difference, HPR Sim's less the codes'.
    gap: Vec<Vec<Vec<f64>>>,
    /// For a position, the model difference with the gravity alone, the code flying HPR Sim's
    /// air.
    gravity_alone: Option<Vec<Vec<Vec<f64>>>>,
}

fn against(case: &Case, environment: &Environment, history: &History) -> Result<Against, String> {
    let times_s = history.times_s();
    let (model, gap) = match case.trace {
        Trace::Gravity => (
            None,
            gravity_gap_m(environment, &times_s)?
                .into_iter()
                .map(|gap| vec![vec![gap]])
                .collect(),
        ),
        Trace::GravityOnTheFlight => (
            None,
            point_mass_gap_m(case, environment, history, true, &times_s)?
                .into_iter()
                .map(|gap| vec![gap])
                .collect(),
        ),
        Trace::EarthRate => {
            let both = earth_rate_gap_deg_s(case, environment, &times_s)?;
            let split = |rates: [f64; 3]| rates.map(|rate| vec![rate]).to_vec();
            (
                Some(both.iter().map(|(model, _)| split(*model)).collect()),
                both.iter().map(|(_, gap)| split(*gap)).collect(),
            )
        }
    };
    let gravity_alone = match case.trace {
        Trace::GravityOnTheFlight => Some(
            point_mass_gap_m(case, environment, history, false, &times_s)?
                .into_iter()
                .map(|gap| vec![gap])
                .collect(),
        ),
        _ => None,
    };
    Ok(Against {
        flown: flown(case, environment, Method::default(), &times_s)?,
        tight: flown(case, environment, tight(), &times_s)?,
        times_s,
        model,
        gap,
        gravity_alone,
    })
}

/// One case's part of the report.
fn compare(case: &Case, environment: &Environment, histories: &[History]) -> Result<Value, String> {
    let flights = histories
        .iter()
        .map(|history| against(case, environment, history))
        .collect::<Result<Vec<_>, _>>()?;
    let mut quantities = Vec::new();
    for (index, quantity) in case.quantities.iter().enumerate() {
        let rule = spread_rule(histories, index);
        let mut codes = Vec::new();
        // Over the kept codes: the largest difference, the most the model difference leaves of
        // a largest difference, the largest unexplained part at any sample, and the least of
        // the codes' largest unexplained parts (HPR Sim's distance from the code nearest it),
        // at the default method and at the tight tolerances.
        let mut largest_kept: f64 = 0.0;
        let mut left_kept: f64 = 0.0;
        let mut unexplained_kept: f64 = 0.0;
        let mut nearest = f64::INFINITY;
        let mut nearest_tight = f64::INFINITY;
        let mut model_gap = (0.0_f64, 0.0);
        let mut left_gravity_alone: Option<f64> = None;
        let zero = vec![0.0; if quantity.is_distance() { 3 } else { 1 }];
        for (at, (history, flight)) in histories.iter().zip(&flights).enumerate() {
            let code: Vec<&Vec<f64>> = history.samples.values().map(|v| &v[index]).collect();
            let residual = |hpr: &[Vec<Vec<f64>>]| -> Vec<f64> {
                hpr.iter()
                    .zip(&code)
                    .zip(&flight.gap)
                    .map(|((h, c), g)| less_both(&h[index], c, &g[index]))
                    .collect()
            };
            let difference = largest(
                &flight.times_s,
                flight
                    .flown
                    .iter()
                    .zip(&code)
                    .map(|(h, c)| less(&h[index], c)),
            );
            let unexplained = residual(&flight.flown);
            // What the model difference leaves of the largest difference: the residual at the
            // sample the largest difference comes at.
            let there = flight
                .times_s
                .iter()
                .position(|time| *time == difference.1)
                .ok_or("the largest difference's time is not a sample")?;
            let left = unexplained[there];
            let worst = largest(&flight.times_s, unexplained.iter().copied());
            let worst_tight = largest(&flight.times_s, residual(&flight.tight).into_iter());
            let gap = largest(
                &flight.times_s,
                flight.gap.iter().map(|g| less(&g[index], &zero)),
            );
            if gap.0.abs() > model_gap.0.abs() {
                model_gap = gap;
            }
            let left_out = rule.left_out == Some(at);
            if !left_out {
                largest_kept = largest_kept.max(difference.0.abs());
                left_kept = left_kept.max(left.abs());
                unexplained_kept = unexplained_kept.max(worst.0.abs());
                nearest = nearest.min(worst.0.abs());
                nearest_tight = nearest_tight.min(worst_tight.0.abs());
            }
            let mut row = json!({
                "code": history.code,
                "samples": code.len(),
                "left_out": left_out,
                "largest_difference": kept(difference.0),
                "largest_difference_at_s": difference.1,
                "model_difference_there": kept(less(&flight.gap[there][index], &zero)),
                "left_of_the_largest_difference": kept(left),
                "largest_unexplained": kept(worst.0),
                "largest_unexplained_at_s": worst.1,
                "largest_unexplained_at_tight_tolerances": kept(worst_tight.0),
            });
            if let Some(model) = &flight.model {
                // How far the code sits from its own model as this file integrates it: the
                // check that the report's start was read as the codes flew it.
                let from_model = largest(
                    &flight.times_s,
                    model.iter().zip(&code).map(|(m, c)| less(&m[index], c)),
                );
                row["largest_difference_of_the_codes_model"] = json!(kept(from_model.0));
            }
            if let Some(alone) = &flight.gravity_alone {
                // What the gravity alone leaves of the largest difference, at its sample.
                let left = less_both(
                    &flight.flown[there][index],
                    code[there],
                    &alone[there][index],
                );
                row["left_with_the_gravity_alone"] = json!(kept(left));
                if !left_out {
                    left_gravity_alone = Some(left_gravity_alone.unwrap_or(0.0).max(left.abs()));
                }
            }
            if let Some(fit) = drag_fit(case, history) {
                row["largest_drag_off_its_density"] = json!(kept(fit));
            }
            if let Some([inertial, relative]) = damping_law_fit(case, environment, history) {
                row["damping_law_off_on_the_inertial_rate"] = json!(inertial.map(kept));
                row["damping_law_off_on_the_rate_relative_to_the_earth"] =
                    json!(relative.map(kept));
            }
            if quantity.is_distance() {
                // The code's own air: its density's largest relative difference from HPR Sim's
                // 1976 atmosphere at the code's altitude, over its samples.
                let mut most = 0.0_f64;
                for sample in history.air.values() {
                    let off = sample_ratio(environment, *sample)? - 1.0;
                    if off.abs() > most.abs() {
                        most = off;
                    }
                }
                row["largest_air_density_difference"] = json!(kept(most));
            }
            codes.push(row);
        }
        quantities.push(json!({
            "key": quantity.key,
            "name": quantity.name,
            "unit": quantity.unit,
            "distance": quantity.is_distance(),
            "spread_of_all_codes": kept(rule.all),
            "code_left_out": rule.left_out.map(|out| histories[out].code.clone()),
            "spread": kept(rule.kept),
            "largest_difference_from_a_kept_code": kept(largest_kept),
            "within_the_spread": largest_kept <= rule.kept,
            "model_difference": kept(model_gap.0),
            "model_difference_at_s": model_gap.1,
            "most_left_of_a_kept_codes_largest_difference": kept(left_kept),
            "traced_to_within_the_spread": left_kept <= rule.kept,
            "largest_unexplained_from_a_kept_code": kept(unexplained_kept),
            "largest_unexplained_over_the_spread": kept((unexplained_kept - rule.kept).max(0.0)),
            "largest_unexplained_from_the_nearest_kept_code": kept(nearest),
            "the_same_at_tight_tolerances": kept(nearest_tight),
            "most_left_with_the_gravity_alone": left_gravity_alone.map(kept),
            "codes": codes,
        }));
    }
    Ok(json!({
        "case": case.number,
        "name": case.name,
        "source": case.tables,
        "codes": histories.len(),
        "model_difference": case.model_difference,
        "quantities": quantities,
    }))
}

/// The report, from the pinned archive.
pub fn generate(root: &Path) -> Result<Value, String> {
    let archive = fs::read(root.join(ARCHIVE)).map_err(|e| {
        format!("{ARCHIVE}: {e} (the codes' files live only under refs/: `cargo xtask refs fetch`)")
    })?;
    let mut out = Vec::new();
    for case in cases()? {
        let environment = environment(case.air)?;
        let ellipsoid = environment.earth.field().ellipsoid();
        let histories = histories(&archive, &case, &ellipsoid)?;
        out.push(compare(&case, &environment, &histories)?);
    }
    Ok(json!({
        "generator": "cargo xtask check-cases",
        "reference": "NASA/TM-2015-218675, Volumes I and II (Murri, Jackson and Shelton 2015), \
                      errata of 2025-09-08, and the codes' time histories published with it",
        "tight_tolerance": TIGHT_TOLERANCE,
        "note": "HPR Sim's free body (hpr_sim::FreeBody: the flight's gravity, Coriolis force, \
                 air, wind, rigid-body equations and default integrator, under the test's \
                 damping or drag) against each code's file over 30 s. A difference is HPR Sim's \
                 value less the code's; a largest one is the largest in size over the file's \
                 samples, with its sign. A position is compared as the distance between HPR \
                 Sim's and the code's, each in Earth-fixed axes, the code's from its latitude, \
                 longitude and altitude on WGS 84. spread is the largest difference (or \
                 distance) between any two kept codes at a time every one of them has; a code is \
                 left out when leaving it cuts the spread of all by more than half. \
                 model_difference is the case's named difference between the two models, \
                 computed apart from the flight, or 0 where none is named. \
                 left_of_the_largest_difference is a code's largest difference less the model \
                 difference at the same sample, and a quantity is traced when that is no more \
                 than the spread for every kept code. largest_unexplained is the same \
                 subtraction's largest at any sample, a stricter figure: against a kept code \
                 that itself sets the spread it comes to the spread plus HPR Sim's own distance \
                 from the other codes, so its part over the spread is given, with HPR Sim's \
                 largest unexplained part from the kept code nearest it, at the default \
                 tolerances and at tight ones (tight_tolerance, relative and absolute). Figures \
                 keep four significant digits.",
        "cases": out,
    }))
}

/// A figure as the page shows it, to four significant digits in plain decimals, with its sign:
/// altitudes and positions in millimeters, rates in deg/s.
fn shown(value: f64, unit: &str) -> String {
    let (value, unit) = match unit {
        "m" => (1000.0 * value, "mm"),
        other => (value, other),
    };
    if value == 0.0 {
        return format!("0 {unit}");
    }
    let decimals = (3 - exponent(value)).max(0) as usize;
    format!("{value:+.decimals$} {unit}")
}

/// The same without a sign, for a size.
fn shown_size(value: f64, unit: &str) -> String {
    shown(value.abs(), unit).trim_start_matches('+').to_owned()
}

/// The report as a page.
pub fn markdown(report: &Value) -> Result<String, String> {
    let tight = report["tight_tolerance"]
        .as_f64()
        .ok_or_else(|| format!("{REPORT}: no `tight_tolerance`"))?;
    let mut out = format!(
        "# NASA's check cases 1 to 3 and 6 to 10\n\n\
         Written by `cargo xtask check-cases`; don't edit it by hand. What it checks and how far \
         to trust it: [the accuracy page](../../docs/accuracy.md) and \
         [rigid-body flight](../../docs/physics/flight.md). A free body of the report's mass and \
         inertia flies each case on HPR Sim's gravity, Coriolis force, air, wind, rigid-body \
         equations and default integrator, under the case's damping or drag applied by the \
         test, and is set beside each code's published time history (NASA/TM-2015-218675). The \
         codes' files are not committed; only differences and spreads are.\n\n\
         How to read it:\n\n\
         - **A difference** is HPR Sim's value less the code's. A largest difference is the \
         largest in size over the file's samples of the 30 s, with its sign. A position's is the \
         distance between HPR Sim's and the code's, each in Earth-fixed axes, the code's from \
         the latitude, longitude and altitude in its file.\n\
         - **The spread** is the largest difference between any two codes at a time every one \
         of them has. One code is left out, and named, when leaving it out cuts the spread by \
         more than half; the spread of the rest is then the measure.\n\
         - **The model difference** is a named difference between HPR Sim's model and the \
         codes', sized apart from the flight. For a position it is a distance too, and the \
         codes' side of it flies each code's own air density, so it differs from code to \
         code; the table gives the largest.\n\
         - **Left of the largest difference** is a code's largest difference less the model \
         difference at the same sample. A quantity is traced when that is no more than the \
         spread for every kept code.\n\
         - **The largest unexplained part** is that subtraction's largest at any sample, a \
         stricter figure. Against a kept code that itself sets the spread, it is the spread \
         plus HPR Sim's own distance from the other codes, so the table gives its part over the \
         spread, and HPR Sim's largest unexplained part from the kept code nearest it: at the \
         default tolerances (1e-8), and at {tight:e}, where the integrator's step error is \
         negligible.\n",
    );
    let text = |value: &Value, key: &str| -> Result<String, String> {
        value[key]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| format!("{REPORT}: no `{key}`"))
    };
    let number = |value: &Value, key: &str| -> Result<f64, String> {
        value[key]
            .as_f64()
            .ok_or_else(|| format!("{REPORT}: no `{key}`"))
    };
    let cases = report["cases"]
        .as_array()
        .ok_or_else(|| format!("{REPORT}: no `cases`"))?;
    for case in cases {
        let _ = write!(
            out,
            "\n## Case {}: {}\n\n{}; {} codes. The model difference: {}.\n\n",
            case["case"],
            text(case, "name")?,
            text(case, "source")?,
            case["codes"],
            case["model_difference"]
                .as_str()
                .unwrap_or("none named, so it is 0 throughout"),
        );
        out.push_str(
            "| Quantity | Spread of all codes | Code left out | Spread of the rest | \
             Largest difference from a kept code | Model difference | \
             Most left of a largest difference | Reading |\n\
             |---|--:|---|--:|--:|--:|--:|---|\n",
        );
        let quantities = case["quantities"]
            .as_array()
            .ok_or_else(|| format!("{REPORT}: no `quantities`"))?;
        for quantity in quantities {
            let unit = text(quantity, "unit")?;
            let reading = if quantity["within_the_spread"] == json!(true) {
                "within the spread"
            } else if quantity["traced_to_within_the_spread"] == json!(true) {
                "past the spread; traced to within it"
            } else {
                "past the spread; not traced to within it"
            };
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                text(quantity, "name")?,
                shown_size(number(quantity, "spread_of_all_codes")?, &unit),
                quantity["code_left_out"].as_str().unwrap_or("none"),
                shown_size(number(quantity, "spread")?, &unit),
                shown_size(
                    number(quantity, "largest_difference_from_a_kept_code")?,
                    &unit
                ),
                if quantity["distance"] == json!(true) {
                    shown_size(number(quantity, "model_difference")?, &unit)
                } else {
                    shown(number(quantity, "model_difference")?, &unit)
                },
                shown_size(
                    number(quantity, "most_left_of_a_kept_codes_largest_difference")?,
                    &unit
                ),
                reading,
            );
        }
        let _ = write!(
            out,
            "\n### Case {}: the stricter figure, at every sample\n\n\
             | Quantity | Largest unexplained part from a kept code | Its part over the spread | \
             From the nearest kept code | The same at tight tolerances |\n\
             |---|--:|--:|--:|--:|\n",
            case["case"],
        );
        for quantity in quantities {
            let unit = text(quantity, "unit")?;
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} |",
                text(quantity, "name")?,
                shown_size(
                    number(quantity, "largest_unexplained_from_a_kept_code")?,
                    &unit
                ),
                shown_size(
                    number(quantity, "largest_unexplained_over_the_spread")?,
                    &unit
                ),
                shown_size(
                    number(quantity, "largest_unexplained_from_the_nearest_kept_code")?,
                    &unit
                ),
                shown_size(number(quantity, "the_same_at_tight_tolerances")?, &unit),
            );
        }
        for quantity in quantities {
            let unit = text(quantity, "unit")?;
            let codes = quantity["codes"]
                .as_array()
                .ok_or_else(|| format!("{REPORT}: no `codes`"))?;
            // Case 2's files are also set beside the codes' own model as this tool integrates
            // it: how well the report's start was read.
            let model = codes
                .iter()
                .all(|code| code["largest_difference_of_the_codes_model"].is_number());
            let air = codes
                .iter()
                .all(|code| code["largest_air_density_difference"].is_number());
            let alone = codes
                .iter()
                .all(|code| code["left_with_the_gravity_alone"].is_number());
            let distance = quantity["distance"] == json!(true);
            let figure = |value: f64| {
                if distance {
                    shown_size(value, &unit)
                } else {
                    shown(value, &unit)
                }
            };
            let _ = write!(
                out,
                "\n### Case {}, {}: each code\n\n\
                 | Code | Samples | Kept | Largest difference | At, s | Model difference there | \
                 Left of it | Largest unexplained part | The same at tight tolerances |{}\n\
                 |---|--:|---|--:|--:|--:|--:|--:|--:|{}\n",
                case["case"],
                text(quantity, "name")?,
                if model {
                    " The codes' model, integrated here, less the code |"
                } else if air && alone {
                    " Its air density less the 1976 atmosphere's, largest, relative | \
                     Left with the gravity alone |"
                } else {
                    ""
                },
                if model {
                    "--:|"
                } else if air && alone {
                    "--:|--:|"
                } else {
                    ""
                },
            );
            for code in codes {
                let _ = write!(
                    out,
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                    text(code, "code")?,
                    code["samples"],
                    if code["left_out"] == json!(true) {
                        "left out"
                    } else {
                        "kept"
                    },
                    figure(number(code, "largest_difference")?),
                    number(code, "largest_difference_at_s")?,
                    figure(number(code, "model_difference_there")?),
                    figure(number(code, "left_of_the_largest_difference")?),
                    figure(number(code, "largest_unexplained")?),
                    figure(number(code, "largest_unexplained_at_tight_tolerances")?),
                );
                if model {
                    let _ = write!(
                        out,
                        " {} |",
                        shown(
                            number(code, "largest_difference_of_the_codes_model")?,
                            &unit
                        )
                    );
                } else if air && alone {
                    let _ = write!(
                        out,
                        " {:+.3e} | {} |",
                        number(code, "largest_air_density_difference")?,
                        figure(number(code, "left_with_the_gravity_alone")?),
                    );
                }
                out.push('\n');
            }
        }
        // Readings of the codes' files beside the comparison: which rate case 3's codes damp,
        // and whether a sphere's code drags on the density it prints.
        let codes = quantities
            .first()
            .and_then(|q| q["codes"].as_array())
            .ok_or_else(|| format!("{REPORT}: no `codes`"))?;
        if codes
            .iter()
            .any(|code| code["damping_law_off_on_the_inertial_rate"].is_array())
        {
            let _ = write!(
                out,
                "\n### Case {}: the damping law in each code's file\n\n\
                 Each code's printed moments against the test's law on its own printed density, \
                 speed and rates, over the first 8 s: the largest relative difference about roll, \
                 pitch and yaw, on the inertial rate and on the rate relative to the Earth.\n\n\
                 | Code | On the inertial rate | On the rate relative to the Earth |\n\
                 |---|---|---|\n",
                case["case"],
            );
            for code in codes {
                let three = |key: &str| -> String {
                    code[key].as_array().map_or_else(
                        || "no columns".to_owned(),
                        |v| {
                            v.iter()
                                .map(|x| format!("{:.3e}", x.as_f64().unwrap_or(f64::NAN)))
                                .collect::<Vec<_>>()
                                .join(", ")
                        },
                    )
                };
                let _ = writeln!(
                    out,
                    "| {} | {} | {} |",
                    text(code, "code")?,
                    three("damping_law_off_on_the_inertial_rate"),
                    three("damping_law_off_on_the_rate_relative_to_the_earth"),
                );
            }
        }
        if codes
            .iter()
            .any(|code| code["largest_drag_off_its_density"].is_number())
        {
            let _ = write!(
                out,
                "\n### Case {}: each code's drag on its own density\n\n\
                 The largest relative difference between a code's printed drag force and \
                 ½ρV²SC_D on its own printed density and speed, in still air.\n\n\
                 | Code | Largest relative difference |\n|---|--:|\n",
                case["case"],
            );
            for code in codes {
                let _ = writeln!(
                    out,
                    "| {} | {} |",
                    text(code, "code")?,
                    code["largest_drag_off_its_density"]
                        .as_f64()
                        .map_or_else(|| "no columns".to_owned(), |x| format!("{x:.3e}")),
                );
            }
        }
    }
    Ok(out)
}

/// Writes the report, or with `--check` fails if the committed one differs.
pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let fresh = generate(&root)?;
    let page = markdown(&fresh)?;
    let (path, page_path) = (root.join(REPORT), root.join(REPORT_MD));
    if check {
        let committed = fs::read_to_string(&path).map_err(|e| format!("{REPORT}: {e}"))?;
        let committed: Value =
            serde_json::from_str(&committed).map_err(|e| format!("{REPORT}: {e}"))?;
        if !crate::designs::same(&committed, &fresh) {
            return Err(format!(
                "{REPORT} is stale: run `cargo xtask check-cases` and commit it: {}",
                crate::designs::difference(&committed, &fresh).unwrap_or_default()
            ));
        }
        let committed_page =
            fs::read_to_string(&page_path).map_err(|e| format!("{REPORT_MD}: {e}"))?;
        if committed_page != markdown(&committed)? {
            return Err(format!(
                "{REPORT_MD} is not {REPORT}'s page: run `cargo xtask check-cases` and commit it"
            ));
        }
        println!("{REPORT} is current");
        return Ok(());
    }
    let text = serde_json::to_string_pretty(&fresh).map_err(|e| e.to_string())? + "\n";
    fs::write(&path, text).map_err(|e| format!("{REPORT}: {e}"))?;
    fs::write(&page_path, page).map_err(|e| format!("{REPORT_MD}: {e}"))?;
    println!("wrote {REPORT} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A code's history of one quantity from `(time, value)` pairs.
    fn series(code: &str, samples: &[(f64, f64)]) -> History {
        History {
            code: code.to_owned(),
            samples: samples
                .iter()
                .map(|(time, value)| (key(*time), vec![vec![*value]]))
                .collect(),
            air: BTreeMap::new(),
            text: String::new(),
        }
    }

    /// A code's history of one position from `(time, [x, y, z])` pairs.
    fn track(code: &str, samples: &[(f64, [f64; 3])]) -> History {
        History {
            code: code.to_owned(),
            samples: samples
                .iter()
                .map(|(time, p)| (key(*time), vec![p.to_vec()]))
                .collect(),
            air: BTreeMap::new(),
            text: String::new(),
        }
    }

    /// Four codes, three within 1 of each other and a fourth `off` from the first.
    fn four(off: f64) -> Vec<History> {
        vec![
            series("SIM 1", &[(0.0, 0.0), (0.1, 10.0)]),
            series("SIM 2", &[(0.0, 0.0), (0.1, 10.5)]),
            series("SIM 3", &[(0.0, 0.0), (0.1, 11.0)]),
            series("SIM 4", &[(0.0, 0.0), (0.1, 10.0 + off)]),
        ]
    }

    /// The code whose leaving cuts the spread by more than half is left out; one whose leaving
    /// cuts it by exactly half, or less, is kept.
    #[test]
    fn a_code_is_left_out_only_when_that_cuts_the_spread_by_more_than_half() {
        let rule = spread_rule(&four(2.5), 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (2.5, Some(3), 1.0));
        // Exactly half: the spread of all is 2, of the rest 1.
        let rule = spread_rule(&four(2.0), 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (2.0, None, 2.0));
        let rule = spread_rule(&four(1.25), 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (1.25, None, 1.25));
        // The outlier can sit below the others as well as above.
        let rule = spread_rule(&four(-3.0), 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (4.0, Some(3), 1.0));
    }

    /// With three codes the rule is the same: two remain, enough for a spread. With two none
    /// is left out, since one alone has no spread.
    #[test]
    fn three_codes_may_leave_one_out_and_two_may_not() {
        let mut codes = four(50.0);
        codes.remove(1);
        let rule = spread_rule(&codes, 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (50.0, Some(2), 1.0));
        // Three evenly spaced keep every code: leaving an end out halves the spread, no more.
        let mut codes = four(0.0);
        codes.truncate(3);
        let rule = spread_rule(&codes, 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (1.0, None, 1.0));
        codes.truncate(2);
        let rule = spread_rule(&codes, 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (0.5, None, 0.5));
    }

    /// A position's spread is the largest distance between two codes, not the largest
    /// difference in one axis; a scalar's is the highest less the lowest, with its sign kept
    /// in a difference.
    #[test]
    fn a_position_is_compared_as_a_distance() {
        assert_eq!(less(&[3.0], &[5.0]), -2.0);
        assert_eq!(less(&[1.0, 2.0, 2.0], &[0.0; 3]), 3.0);
        assert_eq!(
            less_both(&[4.0, 6.0, 0.0], &[1.0, 2.0, 0.0], &[0.0; 3]),
            5.0
        );
        let a = track("SIM 1", &[(0.0, [0.0; 3]), (0.1, [3.0, 0.0, 0.0])]);
        let b = track("SIM 2", &[(0.0, [0.0; 3]), (0.1, [0.0, 4.0, 0.0])]);
        let c = track("SIM 3", &[(0.0, [0.0; 3]), (0.1, [0.0, 0.0, 0.0])]);
        // SIM 1 and SIM 2 are 5 apart, though no axis differs by more than 4.
        assert_eq!(spread(&[&a, &b, &c], 0), 5.0);
    }

    /// The spread is taken only where every code has a sample: a finer file's extra times, where
    /// it alone has a value, don't count.
    #[test]
    fn the_spread_is_over_the_times_every_code_has() {
        let coarse = series("SIM 1", &[(0.0, 0.0), (0.1, 1.0)]);
        let fine = series("SIM 5", &[(0.0, 0.0), (0.05, 99.0), (0.1, 1.5)]);
        assert_eq!(spread(&[&coarse, &fine], 0), 0.5);
        assert_eq!(spread(&[&fine, &coarse], 0), 0.5);
    }

    /// A file is read by its columns' names, in its own unit, over the case's 30 s; what can't be
    /// read is refused, naming the code.
    #[test]
    fn a_history_is_read_by_column_name() {
        let wgs84 = NormalGravity::wgs84().ellipsoid();
        let altitude = [Quantity {
            key: "altitude",
            name: "altitude",
            reading: Reading::Column {
                column: "altitudeMsl_ft",
                factor: FOOT_M,
            },
            unit: "m",
        }];
        let history = |code: &str, text: &str, quantities: &[Quantity]| {
            super::history(code, text, quantities, &wgs84)
        };
        let rows: String = (1..=300)
            .map(|i| format!("{},7,{}\n", 0.1 * f64::from(i), 30_000 - i))
            .collect();
        let text = format!("time,other,altitudeMsl_ft\n0.0,7,30000.0\n{rows}30.1,7,1.0\n");
        let read = history("SIM 9", &text, &altitude).unwrap();
        assert_eq!(read.samples.len(), 301);
        assert_eq!(read.samples[&0], vec![vec![9144.0]]);
        assert_eq!(read.samples[&30_000], vec![vec![29_700.0 * FOOT_M]]);
        assert_eq!(read.times_s()[3], 0.3);
        assert!(read.air.is_empty());

        let refused = |text: &str| history("SIM 9", text, &altitude).err().unwrap();
        assert_eq!(refused(""), "SIM 9: empty file");
        assert_eq!(
            refused("time,altitude\n0,1\n"),
            "SIM 9: no column `altitudeMsl_ft`"
        );
        assert_eq!(
            refused("time,altitudeMsl_ft\n0,x\n"),
            "SIM 9: row 2: `x` is no number"
        );
        assert_eq!(
            refused("time,altitudeMsl_ft\n0,NaN\n"),
            "SIM 9: row 2: `NaN` is not finite"
        );
        assert_eq!(refused("time,altitudeMsl_ft\n0\n"), "SIM 9: row 2 is short");
        assert_eq!(
            refused("time,altitudeMsl_ft\n0,1\n0.0001,2\n"),
            "SIM 9: the time 0.0001 s comes twice"
        );
        // A file that stops early is not a 30 s history.
        assert!(refused("time,altitudeMsl_ft\n0,1\n29.9,2\n").contains("not 0 to 30 s"));
        // Blank lines are skipped but keep their numbers: the bad cell is on the file's line 4.
        assert_eq!(
            refused("time,altitudeMsl_ft\n0,1\n\n0.1,x\n"),
            "SIM 9: row 4: `x` is no number"
        );
        assert_eq!(refused("\ntime,altitudeMsl_ft\n0,1\n"), "SIM 9: empty file");

        // A position is read from the latitude, longitude and altitude onto WGS 84, with the
        // code's air beside it.
        let position = [Quantity {
            key: "position",
            name: "position",
            reading: Reading::Position,
            unit: "m",
        }];
        let header = "time,latitude_deg,longitude_deg,altitudeMsl_ft,airDensity_slug_ft3";
        let rows: String = (0..=300)
            .map(|i| format!("{},0,90,1000,0.002\n", 0.1 * f64::from(i)))
            .collect();
        let read = history("SIM 9", &format!("{header}\n{rows}"), &position).unwrap();
        let p = &read.samples[&0][0];
        let a = wgs84.semi_major_axis_m() + 1000.0 * FOOT_M;
        assert!(
            p[0].abs() < 1e-6 && (p[1] - a).abs() < 1e-6 && p[2] == 0.0,
            "{p:?}"
        );
        let [height, density] = read.air[&30_000];
        assert_eq!(height, 1000.0 * FOOT_M);
        assert!(
            (density - 0.002 * SLUG_KG / FOOT_M.powi(3)).abs() < 1e-15,
            "{density}"
        );
        assert_eq!(
            history(
                "SIM 9",
                "time,latitude_deg,longitude_deg,altitudeMsl_ft\n0,0,0,1\n",
                &position
            )
            .err()
            .unwrap(),
            "SIM 9: no column `airDensity_slug_ft3`"
        );
    }

    /// An archive in memory: the case's folder only, each file named by its code and read in
    /// the codes' order; two files of one code, a file whose name names no code, and fewer than
    /// three codes are refused.
    #[test]
    fn the_archive_is_read_by_folder_and_code() {
        use std::io::Write as _;
        let wgs84 = NormalGravity::wgs84().ellipsoid();
        let zipped = |files: &[(&str, &str)]| -> Vec<u8> {
            let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            for (name, text) in files {
                writer.start_file(*name, options).unwrap();
                writer.write_all(text.as_bytes()).unwrap();
            }
            writer.finish().unwrap().into_inner()
        };
        let file = |altitude: f64| {
            let rows: String = (0..=300)
                .map(|i| format!("{},{altitude}\n", 0.1 * f64::from(i)))
                .collect();
            format!("time,altitudeMsl_ft\n{rows}")
        };
        let case = cases().unwrap().remove(0);
        let folder = "Atmospheric_checkcases/Atmos_01_DroppedSphere";
        let (one, two, three) = (file(1.0), file(2.0), file(3.0));
        let read = |files: &[(&str, &str)]| histories(&zipped(files), &case, &wgs84);
        let codes = read(&[
            (&format!("{folder}/Atmos_01_sim_06.csv"), &three),
            (&format!("{folder}/Atmos_01_sim_02.csv"), &one),
            (&format!("{folder}/notes.txt"), "not a file of a code"),
            (
                "Atmospheric_checkcases/Atmos_02_TumblingBrickNoDamping/Atmos_02_sim_04.csv",
                &one,
            ),
            (&format!("{folder}/Atmos_01_sim_10.csv"), &two),
        ])
        .unwrap();
        let names: Vec<&str> = codes.iter().map(|h| h.code.as_str()).collect();
        assert_eq!(names, ["SIM 2", "SIM 6", "SIM 10"]);
        assert_eq!(codes[1].samples[&0], vec![vec![3.0 * FOOT_M]]);

        let refused = |files: &[(&str, &str)]| read(files).err().unwrap();
        let twice = refused(&[
            (&format!("{folder}/Atmos_01_sim_02.csv"), &one),
            (&format!("{folder}/old/Atmos_01_sim_2.csv"), &two),
            (&format!("{folder}/Atmos_01_sim_03.csv"), &three),
        ]);
        assert!(twice.contains("are both SIM 2's"), "{twice}");
        let unnamed = refused(&[(&format!("{folder}/Atmos_01_extra.csv"), &one)]);
        assert!(
            unnamed.contains("not a code's file by its name"),
            "{unnamed}"
        );
        let few = refused(&[
            (&format!("{folder}/Atmos_01_sim_02.csv"), &one),
            (&format!("{folder}/Atmos_01_sim_03.csv"), &two),
        ]);
        assert!(few.contains("2 files in the archive"), "{few}");
    }

    /// The codes' gravitation against eq. 24's gradient worked by hand on the equator and at the
    /// pole: `−(μ/r²)(1 + (3/2)J₂(a/r)²)` less the centrifugal `ω²r`, and
    /// `−(μ/r²)(1 − 3J₂(a/r)²)`.
    #[test]
    fn the_codes_gravitation_is_eq_24s_gradient() {
        let field = NormalGravity::wgs84();
        let (mu, a, omega) = (3.986_004_418e14_f64, 6_378_137.0_f64, 7.292_115e-5_f64);
        let r = a + START_HEIGHT_M;
        let equator = j2_gravity_ecef(&field, DVec3::new(r, 0.0, 0.0));
        let by_hand = -(mu / (r * r)) * (1.0 + 1.5 * J2 * (a / r).powi(2)) + omega * omega * r;
        assert!(
            (equator.x - by_hand).abs() < 1e-14,
            "{} {by_hand}",
            equator.x
        );
        assert_eq!((equator.y, equator.z), (0.0, 0.0));
        let pole = j2_gravity_ecef(&field, DVec3::new(0.0, 0.0, r));
        let by_hand = -(mu / (r * r)) * (1.0 - 3.0 * J2 * (a / r).powi(2));
        assert!((pole.z - by_hand).abs() < 1e-14, "{} {by_hand}", pole.z);
        assert_eq!((pole.x, pole.y), (0.0, 0.0));
    }

    /// Case 1's model difference by hand. Past `J₂` the level ellipsoid's field holds `J₄`,
    /// −2.3709e-6 for WGS 84, which on the equator adds `(15/8)|J₄|(a/r)⁴`, 4.42e-6, to the
    /// attraction (`J₆` adds about 1e-8 more): 4.3e-5 m/s² of 9.77. Over 30 s that is
    /// `½ · 4.3e-5 · 30²`, 19.5 mm lower, growing as `t²`.
    #[test]
    fn the_gravity_gap_is_j4s_over_the_fall() {
        let environment = environment(Air::Still).unwrap();
        let gap = gravity_gap_m(&environment, &[0.0, 15.0, 30.0]).unwrap();
        assert_eq!(gap[0], 0.0);
        let by_hand = -0.5 * (15.0 / 8.0) * 2.3709e-6 * 9.770 * 30.0_f64.powi(2);
        assert!((by_hand + 0.0195).abs() < 0.0001, "{by_hand}");
        assert!((gap[2] - by_hand).abs() < 0.0002, "{} {by_hand}", gap[2]);
        assert!(
            (gap[1] / gap[2] - 0.25).abs() < 0.001,
            "{}",
            gap[1] / gap[2]
        );
    }

    /// Case 2's two models: each keeps its own rotational energy and the size of its angular
    /// momentum, as Euler's equations do; they start one Earth's rate apart in the body rate
    /// and report the same inertial rate at the start, so the gap starts at zero; and it stays
    /// of the Earth's rate's order, 0.0042 deg/s.
    #[test]
    fn the_earth_rate_gap_starts_at_zero_and_keeps_eulers_integrals() {
        let case = cases().unwrap().remove(1);
        let environment = environment(Air::Still).unwrap();
        let times: Vec<f64> = (0..=300).map(|i| 0.1 * f64::from(i)).collect();
        let both = earth_rate_gap_deg_s(&case, &environment, &times).unwrap();
        // The start is the report's inertial rates, 10, 20 and 30 deg/s: the table's roll rate
        // relative to the Earth, printed to nine decimals, plus the Earth's rate comes to 10
        // within 1.2e-9 deg/s.
        let (start, gap) = both[0];
        for (rate, stated) in start.iter().zip([10.0, 20.0, 30.0]) {
            assert!((rate - stated).abs() < 2e-9, "{rate} {stated}");
        }
        assert_eq!(gap, [0.0; 3]);
        let inertia = case.body.inertia_kg_m2();
        let moments = DVec3::new(inertia.x_axis.x, inertia.y_axis.y, inertia.z_axis.z);
        let energy = |w: [f64; 3]| {
            let w = DVec3::from_array(w);
            w.dot(moments * w)
        };
        let momentum = |w: [f64; 3]| (moments * DVec3::from_array(w)).length();
        let (e0, h0) = (energy(start), momentum(start));
        let mut largest_gap: f64 = 0.0;
        let mut turned: f64 = 0.0;
        for (model, gap) in &both {
            assert!((energy(*model) / e0 - 1.0).abs() < 1e-12);
            assert!((momentum(*model) / h0 - 1.0).abs() < 1e-12);
            largest_gap = gap.iter().fold(largest_gap, |a, g| a.max(g.abs()));
            turned = turned.max((model[0] - start[0]).abs());
        }
        assert!(turned > 5.0, "{turned} deg/s");
        let earth_deg_s = 7.292_115e-5_f64.to_degrees();
        assert!(
            largest_gap > earth_deg_s && largest_gap < 5.0 * earth_deg_s,
            "{largest_gap} deg/s against the Earth's {earth_deg_s}"
        );
    }

    /// The test's drag acts along the velocity through the air whichever way the body points.
    /// At an angle of attack of 90° (the body's x axis level and north, the sphere falling
    /// along its z axis) it is the whole `½ρV²SC_D`, straight up, where a drag along the body's
    /// axis would be none; in case 7's wind it acts against the velocity through the air, so it
    /// also pushes east.
    #[test]
    fn the_drag_acts_along_the_velocity_through_the_air_at_90_degrees() {
        let case = cases()
            .unwrap()
            .into_iter()
            .find(|c| c.number == 7)
            .unwrap();
        let Load::Drag {
            area_m2,
            coefficient,
        } = case.load
        else {
            panic!("case 7 flies a drag");
        };
        for air in [Air::Still, Air::Steady] {
            let environment = environment(air).unwrap();
            let loads = TestLoads {
                load: case.load,
                earth_rate_enu_rad_s: environment.earth.angular_velocity_enu_rad_s(),
            };
            let falling = State {
                velocity_enu_m_s: DVec3::new(0.0, 0.0, -150.0),
                ..start_state(&case)
            };
            let in_body = falling
                .attitude
                .conjugate()
                .mul_vec3(falling.velocity_enu_m_s);
            assert!(in_body.x.abs() < 1e-12 && (in_body.z - 150.0).abs() < 1e-12);
            let y = falling.to_array();
            let dragged = case.body.derivative(&environment, &loads, &y).unwrap();
            let free = case
                .body
                .derivative(&environment, &hpr_sim::NoLoads, &y)
                .unwrap();
            let drag = (DVec3::from_slice(&dragged[3..6]) - DVec3::from_slice(&free[3..6]))
                * case.body.mass_kg();
            let wind = environment
                .wind
                .wind(START_HEIGHT_M)
                .unwrap()
                .velocity_enu_m_s;
            let through_air = falling.velocity_enu_m_s - wind;
            let density = environment
                .atmosphere
                .air(START_HEIGHT_M)
                .unwrap()
                .air
                .density_kg_m3;
            let expected =
                through_air * (-0.5 * density * through_air.length() * area_m2 * coefficient);
            assert!(
                (drag - expected).length() < 1e-9 * expected.length(),
                "{air:?}: {drag:?} against {expected:?}"
            );
            // A real force: about 9.4 N up, `½ρV²SC_D` at 150 m/s in the air 30,000 ft up.
            assert!(drag.z > 9.3, "{drag:?}");
            match air {
                Air::Steady => assert!(drag.x > 0.05 && drag.y.abs() < 1e-9, "{drag:?}"),
                _ => assert!(drag.x.abs() < 1e-9 && drag.y.abs() < 1e-9, "{drag:?}"),
            }
        }
    }

    /// Case 3's damping, as the test states it: about each axis `¼ ρ V S l² C ω`, the span `b`
    /// for roll and yaw and the chord `c̄` for pitch, on the inertial rate. Pitch's moment per
    /// unit rate is `(c̄/b)²`, 4.0001, times roll's and yaw's, and each acts against its rate.
    #[test]
    fn the_damping_is_on_the_inertial_rate_with_the_span_and_the_chord() {
        let case = cases()
            .unwrap()
            .into_iter()
            .find(|c| c.number == 3)
            .unwrap();
        let environment = environment(Air::Still).unwrap();
        let earth_rate = environment.earth.angular_velocity_enu_rad_s();
        let loads = TestLoads {
            load: case.load,
            earth_rate_enu_rad_s: earth_rate,
        };
        let falling = State {
            velocity_enu_m_s: DVec3::new(0.0, 0.0, -100.0),
            ..start_state(&case)
        };
        let y = falling.to_array();
        let damped = case.body.derivative(&environment, &loads, &y).unwrap();
        let free = case
            .body
            .derivative(&environment, &hpr_sim::NoLoads, &y)
            .unwrap();
        let moment = case.body.inertia_kg_m2()
            * (DVec3::from_slice(&damped[10..13]) - DVec3::from_slice(&free[10..13]));
        let inertial =
            falling.body_rate_rad_s + falling.unit_attitude().conjugate().mul_vec3(earth_rate);
        let density = environment
            .atmosphere
            .air(START_HEIGHT_M)
            .unwrap()
            .air
            .density_kg_m3;
        let (s, b, c) = (
            0.222_22 * FOOT_M * FOOT_M,
            0.333_33 * FOOT_M,
            0.666_67 * FOOT_M,
        );
        let expected = DVec3::new(b * b, c * c, b * b) * inertial * (-0.25 * density * 100.0 * s);
        assert!(
            (moment - expected).length() < 1e-9 * expected.length(),
            "{moment:?} against {expected:?}"
        );
        let per_rate = moment / inertial;
        assert!((per_rate.y / per_rate.x - 4.000_2).abs() < 1e-4);
        assert!((per_rate.z / per_rate.x - 1.0).abs() < 1e-12);
        assert!(per_rate.max_element() < 0.0);
    }

    /// Case 7's wind is 20 ft/s toward the east at every height, and case 8's Table 32's
    /// `(0.003 h − 20)` ft/s toward the east, to rounding, over the heights the sphere falls
    /// through.
    #[test]
    fn the_winds_are_the_tables() {
        let steady = environment(Air::Steady).unwrap();
        let shear = environment(Air::Shear).unwrap();
        for height_ft in (16_000..=30_000).step_by(250).map(f64::from) {
            let at = |environment: &Environment| {
                environment
                    .wind
                    .wind(height_ft * FOOT_M)
                    .unwrap()
                    .velocity_enu_m_s
            };
            let w = at(&steady);
            assert!(
                (w.x - 20.0 * FOOT_M).abs() < 1e-12 && w.y.abs() < 1e-12,
                "{w:?}"
            );
            let w = at(&shear);
            let expected = shear_ft_s(height_ft) * FOOT_M;
            assert!(
                (w.x - expected).abs() < 1e-12 && w.y.abs() < 1e-12,
                "{height_ft} ft: {w:?} against {expected}"
            );
        }
        assert_eq!(shear_ft_s(30_000.0), 70.0);
    }

    /// The gap of cases 6 to 10 starts at zero. With a code whose air is HPR Sim's own it is
    /// the gravity's alone: HPR Sim lower, by less than case 1's 19.5 mm, as the drag slows the
    /// fall. With a code whose air is 0.1% denser, its sphere is held back more and falls less,
    /// so HPR Sim's sits lower still, by more than 0.1 m.
    #[test]
    fn the_gap_of_the_sphere_is_its_gravity_and_its_air() {
        let case = cases()
            .unwrap()
            .into_iter()
            .find(|c| c.number == 6)
            .unwrap();
        let environment = environment(Air::Still).unwrap();
        let ours = |height: f64| {
            environment
                .atmosphere
                .air(height)
                .unwrap()
                .air
                .density_kg_m3
        };
        let code = |ratio: f64| History {
            code: "SIM 9".to_owned(),
            samples: BTreeMap::new(),
            air: [(0, START_HEIGHT_M), (30_000, 5_000.0)]
                .into_iter()
                .map(|(time, height)| (time, [height, ratio * ours(height)]))
                .collect(),
            text: String::new(),
        };
        let times = [0.0, 30.0];
        // At 0° N, 0° E the local up is the Earth-fixed x axis.
        let same = point_mass_gap_m(&case, &environment, &code(1.0), true, &times).unwrap();
        assert_eq!(same[0], vec![0.0; 3]);
        let up = same[1][0];
        assert!(up < -0.015 && up > -0.0195, "{up}");
        assert!(same[1][1].abs() < 0.1 * up.abs() && same[1][2].abs() < 1e-9);
        let denser = point_mass_gap_m(&case, &environment, &code(1.001), true, &times).unwrap();
        assert!(denser[1][0] < up - 0.1, "{:?}", denser[1]);
        // Without the code's own air the denser code's gap is the gravity's again.
        let alone = point_mass_gap_m(&case, &environment, &code(1.001), false, &times).unwrap();
        assert_eq!(alone, same);
    }

    #[test]
    fn figures_keep_four_significant_digits() {
        assert_eq!(kept(0.019_509_3), 0.019_51);
        assert_eq!(kept(-0.000_899_308), -0.000_899_3);
        assert_eq!(kept(123_456.0), 123_500.0);
        assert_eq!(kept(0.0), 0.0);
        assert_eq!(shown(0.019_51, "m"), "+19.51 mm");
        assert_eq!(shown(-0.000_136_9, "m"), "-0.1369 mm");
        assert_eq!(shown(0.010_77, "deg/s"), "+0.01077 deg/s");
        assert_eq!(shown_size(-5.866e-7, "deg/s"), "0.0000005866 deg/s");
        assert_eq!(shown(0.0, "m"), "0 mm");
        // The exponent is the printed one, so powers of ten count their digits exactly.
        assert_eq!(
            (exponent(1e-3), exponent(0.000_999_9), exponent(1000.0)),
            (-3, -4, 3)
        );
        assert_eq!(kept(0.001), 0.001);
        assert_eq!(shown(1.0, "deg/s"), "+1.000 deg/s");
        assert_eq!(shown(0.1, "m"), "+100.0 mm");
    }

    fn committed() -> Value {
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(REPORT)).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    /// The committed report holds the increment's rule on its own figures, which CI can't
    /// regenerate (the codes' files are not committed): in each quantity one code at most is
    /// left out, and only one whose leaving more than halves the spread; the largest difference
    /// from a kept code is the kept codes' largest; and every quantity is within the spread or
    /// traced, with what the model difference leaves of each kept code's largest difference no
    /// more than the spread.
    #[test]
    fn the_committed_report_is_within_the_spread_or_traced() {
        let report = committed();
        let cases = report["cases"].as_array().unwrap();
        assert_eq!(
            cases
                .iter()
                .map(|c| c["case"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            [1, 2, 3, 6, 7, 8, 9, 10]
        );
        let mut quantities = 0;
        for case in cases {
            for quantity in case["quantities"].as_array().unwrap() {
                quantities += 1;
                let number = |key: &str| quantity[key].as_f64().unwrap();
                let (all, spread) = (number("spread_of_all_codes"), number("spread"));
                let codes = quantity["codes"].as_array().unwrap();
                assert_eq!(codes.len() as u64, case["codes"].as_u64().unwrap());
                let left_out: Vec<&str> = codes
                    .iter()
                    .filter(|code| code["left_out"] == json!(true))
                    .map(|code| code["code"].as_str().unwrap())
                    .collect();
                match quantity["code_left_out"].as_str() {
                    Some(code) => {
                        assert_eq!(left_out, [code]);
                        assert!(spread < 0.5 * all, "{spread} {all}");
                    }
                    None => {
                        assert!(left_out.is_empty());
                        assert_eq!(spread, all);
                    }
                }
                let kept_codes = || codes.iter().filter(|code| code["left_out"] == json!(false));
                let largest = kept_codes()
                    .map(|code| code["largest_difference"].as_f64().unwrap().abs())
                    .fold(0.0, f64::max);
                assert_eq!(largest, number("largest_difference_from_a_kept_code"));
                let left = kept_codes()
                    .map(|code| {
                        code["left_of_the_largest_difference"]
                            .as_f64()
                            .unwrap()
                            .abs()
                    })
                    .fold(0.0, f64::max);
                assert_eq!(left, number("most_left_of_a_kept_codes_largest_difference"));
                let distance = quantity["distance"] == json!(true);
                for code in codes {
                    // What is left is the difference less the model difference at that sample,
                    // to the four digits each is kept at. For a position the three are lengths
                    // of vectors, so the triangle's sides bound what is left.
                    let (difference, model, left) = (
                        code["largest_difference"].as_f64().unwrap(),
                        code["model_difference_there"].as_f64().unwrap(),
                        code["left_of_the_largest_difference"].as_f64().unwrap(),
                    );
                    let rounding = 1e-3 * difference.abs().max(model.abs());
                    if distance {
                        assert!(difference >= 0.0 && model >= 0.0 && left >= 0.0);
                        assert!(
                            left + rounding >= (difference - model).abs()
                                && left <= difference + model + rounding,
                            "{difference} {model} {left}"
                        );
                    } else {
                        assert!(
                            (difference - model - left).abs() <= rounding,
                            "{difference} {model} {left}"
                        );
                    }
                }
                let within = quantity["within_the_spread"] == json!(true);
                let traced = quantity["traced_to_within_the_spread"] == json!(true);
                assert_eq!(within, largest <= spread);
                assert_eq!(traced, left <= spread);
                assert!(
                    within || traced,
                    "case {} {}: {largest} from a kept code against a spread of {spread}, and \
                     the model difference leaves {left}",
                    case["case"],
                    quantity["key"]
                );
                // The stricter figure's excess is stated, not hidden: it is the largest
                // unexplained part less the spread, to their kept digits.
                let over = number("largest_unexplained_over_the_spread");
                let unexplained = number("largest_unexplained_from_a_kept_code");
                assert!((over - (unexplained - spread).max(0.0)).abs() <= 1e-3 * spread);
            }
        }
        assert_eq!(quantities, 12);

        // The readings the decision rests on. Case 3's codes split on the rate they damp: SIM 4
        // the inertial one, SIM 5 and SIM 6 the one relative to the Earth.
        let code_of = |case: u64, name: &str| -> Value {
            cases.iter().find(|c| c["case"] == json!(case)).unwrap()["quantities"][0]["codes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|code| code["code"] == json!(name))
                .unwrap()
                .clone()
        };
        let worst = |code: &Value, key: &str| {
            code[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .fold(0.0, f64::max)
        };
        let inertial = "damping_law_off_on_the_inertial_rate";
        let relative = "damping_law_off_on_the_rate_relative_to_the_earth";
        let sim4 = code_of(3, "SIM 4");
        assert!(worst(&sim4, inertial) < 1e-4 && worst(&sim4, relative) > 1e-3);
        for name in ["SIM 5", "SIM 6"] {
            let code = code_of(3, name);
            assert!(
                worst(&code, relative) < 1e-9 && worst(&code, inertial) > 1e-3,
                "{name}"
            );
        }
        // SIM 1 drags on the density it prints, and that density is the far one.
        for case in [6, 9, 10] {
            let sim1 = code_of(case, "SIM 1");
            assert!(sim1["largest_drag_off_its_density"].as_f64().unwrap() < 1e-9);
        }
        assert!(
            code_of(6, "SIM 1")["largest_air_density_difference"]
                .as_f64()
                .unwrap()
                > 1e-3
        );
        // The gravity alone leaves cases 6 and 7 past the spread and case 8 inside it; the air
        // brings all three inside.
        for (case, past) in [(6, true), (7, true), (8, false)] {
            let quantity =
                &cases.iter().find(|c| c["case"] == json!(case)).unwrap()["quantities"][0];
            let alone = quantity["most_left_with_the_gravity_alone"]
                .as_f64()
                .unwrap();
            let spread = quantity["spread"].as_f64().unwrap();
            assert_eq!(
                alone > spread,
                past,
                "case {case}: {alone} against {spread}"
            );
            assert_eq!(quantity["traced_to_within_the_spread"], json!(true));
        }
    }

    /// The pages quote the committed report as it is, which CI can't regenerate: the accuracy
    /// page's rows for frames, gravity and rigid-body flight each cite the report and carry its
    /// figures, the page on rigid-body flight carries every figure of its table and of the
    /// stricter reading, and the gravity, frames and integration pages carry the figures they
    /// quote.
    #[test]
    fn the_pages_quote_the_committed_report() {
        let root = crate::designs::root().unwrap();
        // A page gives inches in brackets after each length; the report's figures are compared
        // with those brackets taken out.
        let page = |path: &str| {
            let text = fs::read_to_string(root.join(path)).unwrap();
            let mut plain = String::new();
            let mut rest = text.as_str();
            while let Some(at) = rest.find(" (") {
                let inside = &rest[at + 2..];
                let inches = inside.find(')').and_then(|end| {
                    let number = inside[..end].strip_suffix(" in")?;
                    let digits = number.strip_prefix('−').unwrap_or(number);
                    let numeric = !digits.is_empty()
                        && digits.chars().all(|c| c.is_ascii_digit() || c == '.');
                    numeric.then_some(end)
                });
                match inches {
                    Some(end) => {
                        plain.push_str(&rest[..at]);
                        rest = &inside[end + 1..];
                    }
                    None => {
                        plain.push_str(&rest[..at + 2]);
                        rest = inside;
                    }
                }
            }
            // The pages group digits in threes (`1,980 mm`); the report doesn't.
            let plain: Vec<char> = (plain + rest).chars().collect();
            let digit = |at: usize| plain.get(at).is_some_and(char::is_ascii_digit);
            plain
                .iter()
                .enumerate()
                .filter(|&(at, &c)| {
                    let grouping = c == ','
                        && at > 0
                        && digit(at - 1)
                        && (1..=3).all(|k| digit(at + k))
                        && !digit(at + 4);
                    !grouping
                })
                .map(|(_, &c)| c)
                .collect::<String>()
        };
        let report = committed();
        let quantities: Vec<&Value> = report["cases"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|case| case["quantities"].as_array().unwrap())
            .collect();
        // A case's quantities.
        let of = |number: u64| -> Vec<&Value> {
            report["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|case| case["case"] == json!(number))
                .flat_map(|case| case["quantities"].as_array().unwrap())
                .collect()
        };
        // A size as a page prints it, and a signed figure with the pages' minus sign; a
        // distance has no sign.
        let size = |quantity: &Value, key: &str| {
            shown_size(
                quantity[key].as_f64().unwrap(),
                quantity["unit"].as_str().unwrap(),
            )
        };
        let signed = |quantity: &Value, key: &str| {
            if quantity["distance"] == json!(true) {
                return size(quantity, key);
            }
            shown(
                quantity[key].as_f64().unwrap(),
                quantity["unit"].as_str().unwrap(),
            )
            .replace('-', "−")
        };
        // A figure is quoted when it stands whole, not inside a longer number; line breaks and
        // runs of spaces don't matter to a quotation.
        let has = |path: &str, text: &str, figure: &str| {
            let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
            let part_of_a_number =
                |c: Option<char>| c.is_some_and(|c| c.is_ascii_digit() || c == '.');
            let whole = text.match_indices(figure).any(|(at, _)| {
                let before = text[..at].chars().next_back();
                let mut after = text[at + figure.len()..].chars();
                let next = after.next();
                // A full stop that ends a sentence follows a figure; a digit after it doesn't.
                let runs_on = next.is_some_and(|c| c.is_ascii_digit())
                    || (next == Some('.') && after.next().is_some_and(|c| c.is_ascii_digit()));
                !part_of_a_number(before) && !runs_on
            });
            assert!(whole, "{path} doesn't quote {figure}");
        };

        let flight = page("docs/physics/flight.md");
        let section = flight
            .split_once("### A free body against NASA's check cases")
            .unwrap()
            .1
            .split_once("\n### ")
            .unwrap()
            .0;
        for quantity in &quantities {
            // The table's row: the spread, the largest difference, the model difference and
            // what it leaves, in that order.
            let row = format!(
                "| {} | {} | {} | {} |",
                size(quantity, "spread"),
                size(quantity, "largest_difference_from_a_kept_code"),
                signed(quantity, "model_difference"),
                size(quantity, "most_left_of_a_kept_codes_largest_difference"),
            );
            has("docs/physics/flight.md", section, &row);
        }
        // The stricter reading gives cases 1 and 2's figures.
        for quantity in of(1).into_iter().chain(of(2)) {
            for key in [
                "largest_unexplained_from_a_kept_code",
                "largest_unexplained_from_the_nearest_kept_code",
            ] {
                // The stricter reading gives case 2's figures; case 1's largest alone.
                if quantity["unit"] == json!("deg/s") || key.ends_with("a_kept_code") {
                    let figure = size(quantity, key);
                    has(
                        "docs/physics/flight.md",
                        section,
                        figure.trim_end_matches(" deg/s"),
                    );
                }
            }
            if quantity["unit"] == json!("deg/s") {
                for key in [
                    "largest_unexplained_over_the_spread",
                    "the_same_at_tight_tolerances",
                ] {
                    let figure = size(quantity, key);
                    has(
                        "docs/physics/flight.md",
                        section,
                        figure.trim_end_matches(" deg/s"),
                    );
                }
            }
        }

        let accuracy = page("docs/accuracy.md");
        let row = |model: &str| {
            accuracy
                .lines()
                .find(|line| line.starts_with(model) && line.contains("[report][check-cases]"))
                .unwrap_or_else(|| panic!("docs/accuracy.md: no row of {model} cites the report"))
        };
        let (altitude, rates) = (of(1)[0], &of(2)[..]);
        let damped = &of(3)[..];
        let positions: Vec<&Value> = (6..=10).map(|case| of(case)[0]).collect();
        // A row of a model that names the case it holds.
        let row_of = |model: &str, case: &str| {
            accuracy
                .lines()
                .find(|line| line.starts_with(model) && line.contains(case))
                .unwrap_or_else(|| panic!("docs/accuracy.md: no row of {model} for {case}"))
        };
        let gravity = row("| [Gravity](physics/gravity.md) |");
        for key in [
            "largest_difference_from_a_kept_code",
            "spread",
            "model_difference",
            "most_left_of_a_kept_codes_largest_difference",
        ] {
            has(
                "docs/accuracy.md's gravity row",
                gravity,
                &size(altitude, key),
            );
        }
        let rigid = row("| [Rigid-body flight](physics/flight.md) |");
        for rate in rates {
            for key in [
                "largest_difference_from_a_kept_code",
                "most_left_of_a_kept_codes_largest_difference",
            ] {
                let figure = size(rate, key);
                has(
                    "docs/accuracy.md's rigid-body row",
                    rigid,
                    figure.trim_end_matches(" deg/s"),
                );
            }
        }
        // Case 3's row: the largest differences, the spreads and what the Earth's rate leaves.
        let rigid = row_of(
            "| [Rigid-body flight](physics/flight.md) |",
            "NASA's check case 3,",
        );
        for rate in damped {
            for key in [
                "largest_difference_from_a_kept_code",
                "spread",
                "most_left_of_a_kept_codes_largest_difference",
            ] {
                let figure = size(rate, key);
                has(
                    "docs/accuracy.md's case 3 row",
                    rigid,
                    figure.trim_end_matches(" deg/s"),
                );
            }
            assert_eq!(rate["largest_unexplained_over_the_spread"], json!(0.0));
        }
        // The sphere's rows: case 6 on the atmosphere's, 7 and 8 on the wind's, 9 and 10 on the
        // frames'.
        let millimeters =
            |quantity: &Value, key: &str| size(quantity, key).trim_end_matches(" mm").to_owned();
        let atmosphere = row_of(
            "| [Atmosphere](physics/atmosphere.md) |",
            "NASA's check case 6,",
        );
        let wind = row_of("| [Wind](physics/wind.md) |", "NASA's check cases 7 and 8,");
        let shots = row_of(
            "| [Frames](physics/frames.md) |",
            "NASA's check cases 9 and 10,",
        );
        for (at, row, keys) in [
            (
                0,
                atmosphere,
                &[
                    "largest_difference_from_a_kept_code",
                    "spread",
                    "most_left_of_a_kept_codes_largest_difference",
                ][..],
            ),
            (
                1,
                wind,
                &[
                    "largest_difference_from_a_kept_code",
                    "spread",
                    "most_left_of_a_kept_codes_largest_difference",
                ][..],
            ),
            (
                2,
                wind,
                &[
                    "largest_difference_from_a_kept_code",
                    "spread",
                    "most_left_of_a_kept_codes_largest_difference",
                ][..],
            ),
            (
                3,
                shots,
                &[
                    "largest_difference_from_a_kept_code",
                    "spread",
                    "largest_unexplained_from_the_nearest_kept_code",
                ][..],
            ),
            (
                4,
                shots,
                &[
                    "largest_difference_from_a_kept_code",
                    "spread",
                    "largest_unexplained_from_the_nearest_kept_code",
                ][..],
            ),
        ] {
            for key in keys {
                has(
                    "docs/accuracy.md's sphere rows",
                    row,
                    &millimeters(positions[at], key),
                );
            }
        }
        // Cases 6 to 8 are past the spread and traced; 9 and 10 inside it.
        for (at, position) in positions.iter().enumerate() {
            let within = position["within_the_spread"] == json!(true);
            let traced = position["traced_to_within_the_spread"] == json!(true);
            assert_eq!((within, traced || within), (at >= 3, true), "{position}");
        }
        // The flight page gives the nearest kept code's distance once the model difference is
        // taken off: the least and the most of cases 6 to 8, and cases 9 and 10's.
        let nearest = |at: usize| {
            millimeters(
                positions[at],
                "largest_unexplained_from_the_nearest_kept_code",
            )
        };
        let mut dropped: Vec<f64> = (0..3)
            .map(|at| {
                positions[at]["largest_unexplained_from_the_nearest_kept_code"]
                    .as_f64()
                    .unwrap()
            })
            .collect();
        dropped.sort_by(f64::total_cmp);
        for figure in [shown_size(dropped[0], "m"), shown_size(dropped[2], "m")] {
            has(
                "docs/physics/flight.md",
                section,
                figure.trim_end_matches(" mm"),
            );
        }
        for at in [3, 4] {
            has("docs/physics/flight.md", section, &nearest(at));
        }

        // The frames row gives the most the model differences leave: of the altitude, and of
        // any rate.
        let frames = row("| [Frames](physics/frames.md) |");
        let key = "most_left_of_a_kept_codes_largest_difference";
        has(
            "docs/accuracy.md's frames row",
            frames,
            &size(altitude, key),
        );
        let most = rates
            .iter()
            .max_by(|a, b| {
                a[key]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b[key].as_f64().unwrap())
            })
            .unwrap();
        has("docs/accuracy.md's frames row", frames, &size(most, key));
        assert!(accuracy.contains(
            "[check-cases]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/check-cases.md"
        ));

        // The start page's paragraph gives the altitude's largest difference, the largest of
        // the rates' and the largest of the dragged sphere's past the spread.
        let start = page("docs/start-here.md");
        let key = "largest_difference_from_a_kept_code";
        has("docs/start-here.md", &start, &size(altitude, key));
        let most = rates
            .iter()
            .max_by(|a, b| {
                a[key]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b[key].as_f64().unwrap())
            })
            .unwrap();
        has("docs/start-here.md", &start, &size(most, key));
        let farthest = positions[..3]
            .iter()
            .max_by(|a, b| {
                a[key]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b[key].as_f64().unwrap())
            })
            .unwrap();
        has("docs/start-here.md", &start, &size(farthest, key));

        // The other model pages: gravity's four figures of the altitude, the frames page's two
        // remainders, and the integration page's distances from the nearest code at both
        // tolerances.
        let gravity = page("docs/physics/gravity.md");
        for key in [
            "largest_difference_from_a_kept_code",
            "spread",
            "model_difference",
            "most_left_of_a_kept_codes_largest_difference",
        ] {
            has("docs/physics/gravity.md", &gravity, &size(altitude, key));
        }
        let frames = page("docs/physics/frames.md");
        let key = "most_left_of_a_kept_codes_largest_difference";
        has("docs/physics/frames.md", &frames, &size(altitude, key));
        let most_left = rates
            .iter()
            .max_by(|a, b| {
                a[key]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b[key].as_f64().unwrap())
            })
            .unwrap();
        has(
            "docs/physics/frames.md",
            &frames,
            size(most_left, key).trim_end_matches(" deg/s"),
        );
        let integration = page("docs/physics/integration.md");
        for rate in rates {
            for key in [
                "largest_unexplained_from_the_nearest_kept_code",
                "the_same_at_tight_tolerances",
            ] {
                has(
                    "docs/physics/integration.md",
                    &integration,
                    size(rate, key).trim_end_matches(" deg/s"),
                );
            }
        }
    }

    /// The committed page is the committed report's, so the two can't drift apart.
    #[test]
    fn the_committed_page_is_the_reports() {
        let root = crate::designs::root().unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(page, markdown(&committed()).unwrap());
    }
}
