//! NASA's check cases 1 and 2 flown by a free body (M1.14b3), written by `cargo xtask
//! check-cases` to `validation/reports/check-cases.json` and `.md`.
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
//! - **HPR Sim's side** is [`hpr_sim::FreeBody`]: the flight's gravity, Coriolis force, rigid-body
//!   equations and integrator at their defaults, on a body of the report's mass and inertia, from
//!   the report's initial conditions through its unit factors (Table 73).
//! - **The codes' spread** is the largest difference between any two codes at a time every
//!   code's file has. One code is left out, and named, when leaving it out cuts that spread by
//!   more than half; the spread of the rest is then the measure.
//! - **A trace.** Where HPR Sim differs from a kept code by more than the spread, the difference
//!   is set beside the size of a named difference between the two models, computed here apart
//!   from the flight; what that leaves is the unexplained part.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Read as _;
use std::path::Path;

use hpr_core::geodesy::Geodetic;
use hpr_core::gravity::NormalGravity;
use hpr_core::{DMat3, DQuat, DVec3};
use hpr_sim::{Adaptive, Environment, FreeBody, Method, State};
use serde_json::{Value, json};

/// The committed report.
pub const REPORT: &str = "validation/reports/check-cases.json";
/// The committed report as a page.
pub const REPORT_MD: &str = "validation/reports/check-cases.md";
/// The codes' time histories, as `validation/refs.lock.toml` pins them.
const ARCHIVE: &str = "refs/sources/nesc-check-cases/Atmospheric_checkcases.zip";

/// `cargo xtask check-cases`'s lines in the command list.
pub const USAGE: &str = "\
  check-cases [--check]    Fly NASA/TM-2015-218675's check cases 1 and 2 with a free body against
                           each code's time history, read from the pinned archive under refs/, and
                           write only differences and spreads to validation/reports/check-cases.json
                           and .md. --check fails if the committed report differs instead of
                           writing.";

/// Table 73's unit factors, exact: the foot in meters and the pound-force in newtons.
const FOOT_M: f64 = 0.3048;
const POUND_FORCE_N: f64 = 4.448_221_615_260_5;
/// The slug in kilograms and the slug·ft² in kg·m², from those two.
const SLUG_KG: f64 = POUND_FORCE_N / FOOT_M;
const SLUG_FT2_KG_M2: f64 = POUND_FORCE_N * FOOT_M;
/// Table 73's second zonal harmonic, the one the codes' gravitation carries.
const J2: f64 = 0.001_082_629_82;
/// Both cases start 30,000 ft above the ellipsoid and run 30 s (Tables 25 and 26, errata).
const START_HEIGHT_M: f64 = 30_000.0 * FOOT_M;
const DURATION_S: f64 = 30.0;

/// One compared quantity of a case.
#[derive(Clone, Copy)]
struct Quantity {
    /// Its key in the report.
    key: &'static str,
    /// What it is, in words.
    name: &'static str,
    /// Its column in the codes' files.
    column: &'static str,
    /// The factor from the column's unit to the report's.
    factor: f64,
    /// The report's unit.
    unit: &'static str,
}

/// A case: its body, its start and what is compared.
struct Case {
    number: u32,
    name: &'static str,
    folder: &'static str,
    tables: &'static str,
    body: FreeBody,
    /// Roll, pitch and yaw rates relative to the local frame at the start, deg/s.
    rates_deg_s: [f64; 3],
    /// The named difference between HPR Sim's model and the codes' that the trace sizes.
    model_difference: &'static str,
    quantities: Vec<Quantity>,
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
        column,
        factor: 1.0,
        unit: "deg/s",
    };
    Ok(vec![
        Case {
            number: 1,
            name: "the dragless sphere",
            folder: "Atmos_01_DroppedSphere",
            tables: "Volume II, Tables 1 and 25",
            body: body(1.0, [3.6, 3.6, 3.6])?,
            rates_deg_s: [-0.004_178_073, 0.0, 0.0],
            model_difference: "WGS 84's normal gravity, which HPR Sim falls in, less gravitation \
                               cut off after J2 with the centrifugal term, which the codes fall \
                               in, integrated twice along the fall",
            quantities: vec![Quantity {
                key: "altitude",
                name: "altitude",
                column: "altitudeMsl_ft",
                factor: FOOT_M,
                unit: "m",
            }],
        },
        Case {
            number: 2,
            name: "the tumbling brick",
            folder: "Atmos_02_TumblingBrickNoDamping",
            tables: "Volume II, Tables 4 and 26, with the errata's 30,000 ft",
            body: body(0.155_404_754, [0.001_894_220, 0.006_211_019, 0.007_194_665])?,
            rates_deg_s: [9.995_821_927, 20.0, 30.0],
            model_difference: "the Earth's rate left out of HPR Sim's rotational equations: \
                               Euler's equations in the rate relative to the Earth-fixed launch \
                               frame, less the same equations in the rate relative to the stars",
            quantities: vec![
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
            ],
        },
    ])
}

/// The cases' site: 0° N, 0° E on the ellipsoid, WGS 84, the 1976 atmosphere, no wind.
fn environment() -> Result<Environment, String> {
    let site = Geodetic::from_degrees(0.0, 0.0, 0.0).map_err(|e| e.to_string())?;
    Environment::standard(site).map_err(|e| e.to_string())
}

/// The report's body axes at the start, in the launch frame's East-North-Up axes: x north, y
/// east, z down (every Euler angle from the local North-East-Down frame is zero).
fn start_attitude() -> DQuat {
    DQuat::from_mat3(&DMat3::from_cols(DVec3::Y, DVec3::X, -DVec3::Z))
}

fn start_state(case: &Case) -> State {
    State {
        position_enu_m: DVec3::new(0.0, 0.0, START_HEIGHT_M),
        velocity_enu_m_s: DVec3::ZERO,
        attitude: start_attitude(),
        body_rate_rad_s: DVec3::from_array(case.rates_deg_s.map(f64::to_radians)),
    }
}

/// One code's file: its name and its samples of each compared quantity, by time in milliseconds.
struct History {
    code: String,
    samples: BTreeMap<i64, Vec<f64>>,
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

/// Reads a code's file: the time and each quantity's column, by name, over the case's duration.
fn history(code: &str, text: &str, quantities: &[Quantity]) -> Result<History, String> {
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());
    let header: Vec<&str> = lines
        .next()
        .ok_or_else(|| format!("{code}: empty file"))?
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
    let columns = quantities
        .iter()
        .map(|q| column(q.column))
        .collect::<Result<Vec<_>, _>>()?;
    let mut samples = BTreeMap::new();
    for (row, line) in lines.enumerate() {
        let cells: Vec<&str> = line.split(',').collect();
        let number = |index: usize| -> Result<f64, String> {
            let cell = cells
                .get(index)
                .ok_or_else(|| format!("{code}: row {} is short", row + 2))?;
            let value: f64 = cell
                .trim()
                .parse()
                .map_err(|_| format!("{code}: row {}: `{cell}` is no number", row + 2))?;
            if value.is_finite() {
                Ok(value)
            } else {
                Err(format!("{code}: row {}: `{cell}` is not finite", row + 2))
            }
        };
        let time_s = number(time)?;
        if !(0.0..=DURATION_S + 0.0005).contains(&time_s) {
            continue;
        }
        let values = columns
            .iter()
            .zip(quantities)
            .map(|(&index, q)| number(index).map(|value| value * q.factor))
            .collect::<Result<Vec<_>, _>>()?;
        if samples.insert(key(time_s), values).is_some() {
            return Err(format!("{code}: the time {time_s} s comes twice"));
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
    })
}

/// Every code's file of a case, from the archive, in the codes' order.
fn histories(archive: &[u8], case: &Case) -> Result<Vec<History>, String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive))
        .map_err(|e| format!("{ARCHIVE}: {e}"))?;
    let prefix = format!("Atmospheric_checkcases/{}/", case.folder);
    let mut files = Vec::new();
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
        files.push((code, text));
    }
    files.sort_by_key(|(code, _)| *code);
    let mut out = Vec::new();
    for (code, text) in files {
        out.push(history(&format!("SIM {code}"), &text, &case.quantities)?);
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
) -> Result<Vec<Vec<f64>>, String> {
    let states = case
        .body
        .fly(environment, method, 0.0, start_state(case), times_s)
        .map_err(|e| format!("case {}: {e}", case.number))?;
    let frame = environment.earth.frame();
    let earth_rate = environment.earth.angular_velocity_enu_rad_s();
    states
        .iter()
        .map(|state| {
            Ok(match case.number {
                1 => vec![
                    frame
                        .geodetic_from_enu(state.position_enu_m)
                        .map_err(|e| e.to_string())?
                        .height_m,
                ],
                _ => {
                    // The rate relative to the stars, in body axes: the state's rate relative to
                    // the Earth-fixed launch frame plus the Earth's own, turned into the body.
                    let inertial = state.body_rate_rad_s
                        + state.unit_attitude().conjugate().mul_vec3(earth_rate);
                    inertial.to_array().map(f64::to_degrees).to_vec()
                }
            })
        })
        .collect()
}

/// The largest difference between any two of `histories` in quantity `index`, over the times
/// every one of them has.
fn spread(histories: &[&History], index: usize) -> f64 {
    let Some((first, rest)) = histories.split_first() else {
        return 0.0;
    };
    let mut worst: f64 = 0.0;
    for (time, values) in &first.samples {
        let mut low = values[index];
        let mut high = low;
        let mut shared = true;
        for other in rest {
            match other.samples.get(time) {
                Some(values) => {
                    low = low.min(values[index]);
                    high = high.max(values[index]);
                }
                None => shared = false,
            }
        }
        if shared {
            worst = worst.max(high - low);
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
/// only if that cuts it by more than half. With three codes or fewer none is left out: two codes
/// always agree with each other better than three do.
fn spread_rule(histories: &[History], index: usize) -> Spread {
    let all: Vec<&History> = histories.iter().collect();
    let whole = spread(&all, index);
    let mut best: Option<(usize, f64)> = None;
    if histories.len() > 3 {
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

/// Case 1's named difference between the models, computed apart from the flight: HPR Sim falls in
/// WGS 84's normal gravity, whose closed form holds every even zonal harmonic of the level
/// ellipsoid, and the codes in gravitation cut off after `J₂`. The gap in the downward
/// acceleration is taken along the codes' own fall (a point dropped down the site's vertical in
/// their field, by RK4 at 1 ms) and integrated twice. Returns HPR Sim's altitude less the
/// codes', m, at each of `times_s`.
fn gravity_gap_m(environment: &Environment, times_s: &[f64]) -> Result<Vec<f64>, String> {
    let field = environment.earth.field();
    let frame = environment.earth.frame();
    let up_ecef = frame.ecef_from_enu_rotation() * DVec3::Z;
    let down_gap = |height_m: f64| -> Result<[f64; 2], String> {
        let p = frame.ecef_from_enu(DVec3::new(0.0, 0.0, height_m));
        let codes = j2_gravity_ecef(field, p).dot(up_ecef);
        let normal = field.ecef_mps2(p).map_err(|e| e.to_string())?.dot(up_ecef);
        Ok([codes, normal - codes])
    };
    // State: the codes' height and climb rate, and the altitude gap and its rate.
    let derivative = |y: [f64; 4]| -> Result<[f64; 4], String> {
        let [fall, gap] = down_gap(y[0])?;
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

/// Euler's equations for a body with no torque on it: `ω̇ = I⁻¹(−ω × Iω)`, `I` diagonal.
fn euler_rate(moments: DVec3, omega: DVec3) -> DVec3 {
    -omega.cross(moments * omega) / moments
}

/// Case 2's named difference between the models, computed apart from the flight. The codes turn
/// the brick by Euler's equations in its rate relative to the stars. HPR Sim's rotational
/// equations leave the Earth's rate out (ADR-011): they turn it by the same equations in its rate
/// relative to the Earth-fixed launch frame, and the inertial rate is that plus the Earth's,
/// turned into the body. Both are integrated here by RK4 at 1 ms from the report's start.
/// Returns, at each of `times_s`, the codes' model's inertial rates and HPR Sim's model's less
/// those, deg/s.
fn earth_rate_gap_deg_s(
    case: &Case,
    environment: &Environment,
    times_s: &[f64],
) -> Vec<([f64; 3], [f64; 3])> {
    let inertia = case.body.inertia_kg_m2();
    let moments = DVec3::new(inertia.x_axis.x, inertia.y_axis.y, inertia.z_axis.z);
    let earth_rate = environment.earth.angular_velocity_enu_rad_s();
    let start = start_state(case);
    let inertial_of = |q: DQuat, relative: DVec3| relative + q.conjugate().mul_vec3(earth_rate);
    // The codes' model: the inertial rate alone.
    let mut inertial = inertial_of(start.attitude, start.body_rate_rad_s);
    // HPR Sim's model: the rate relative to the launch frame, and the attitude in it.
    let mut relative = start.body_rate_rad_s;
    let mut q = start.attitude;
    let q_dot = |q: DQuat, omega: DVec3| q * DQuat::from_xyzw(omega.x, omega.y, omega.z, 0.0) * 0.5;
    let step = 0.001;
    let mut out = Vec::with_capacity(times_s.len());
    let mut done: i64 = 0;
    for &time in times_s {
        let target = key(time);
        while done < target {
            let k1 = euler_rate(moments, inertial);
            let k2 = euler_rate(moments, inertial + k1 * (0.5 * step));
            let k3 = euler_rate(moments, inertial + k2 * (0.5 * step));
            let k4 = euler_rate(moments, inertial + k3 * step);
            inertial += (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (step / 6.0);

            let (w1, p1) = (euler_rate(moments, relative), q_dot(q, relative));
            let (wa, qa) = (relative + w1 * (0.5 * step), q + p1 * (0.5 * step));
            let (w2, p2) = (euler_rate(moments, wa), q_dot(qa, wa));
            let (wb, qb) = (relative + w2 * (0.5 * step), q + p2 * (0.5 * step));
            let (w3, p3) = (euler_rate(moments, wb), q_dot(qb, wb));
            let (wc, qc) = (relative + w3 * step, q + p3 * step);
            let (w4, p4) = (euler_rate(moments, wc), q_dot(qc, wc));
            relative += (w1 + w2 * 2.0 + w3 * 2.0 + w4) * (step / 6.0);
            q = (q + (p1 + p2 * 2.0 + p3 * 2.0 + p4) * (step / 6.0)).normalize();
            done += 1;
        }
        let hpr = inertial_of(q, relative);
        out.push((
            inertial.to_array().map(f64::to_degrees),
            (hpr - inertial).to_array().map(f64::to_degrees),
        ));
    }
    out
}

/// A figure as the report keeps it: four significant digits, so that a rounding in the last
/// place of a difference of nearly equal numbers doesn't make the report stale.
fn kept(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let digits = 3 - value.abs().log10().floor() as i32;
    let scale = 10f64.powi(digits);
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

/// HPR Sim against one code's file, in every compared quantity.
struct Against {
    times_s: Vec<f64>,
    /// HPR Sim's flight at the default method, and at the tight tolerances.
    flown: Vec<Vec<f64>>,
    tight: Vec<Vec<f64>>,
    /// The codes' model as this file integrates it (case 2 only).
    model: Option<Vec<[f64; 3]>>,
    /// The named model difference, HPR Sim's less the codes'.
    gap: Vec<Vec<f64>>,
}

fn against(case: &Case, environment: &Environment, history: &History) -> Result<Against, String> {
    let times_s = history.times_s();
    let (model, gap) = match case.number {
        1 => (
            None,
            gravity_gap_m(environment, &times_s)?
                .into_iter()
                .map(|gap| vec![gap])
                .collect(),
        ),
        _ => {
            let both = earth_rate_gap_deg_s(case, environment, &times_s);
            (
                Some(both.iter().map(|(model, _)| *model).collect()),
                both.iter().map(|(_, gap)| gap.to_vec()).collect(),
            )
        }
    };
    Ok(Against {
        flown: flown(case, environment, Method::default(), &times_s)?,
        tight: flown(case, environment, tight(), &times_s)?,
        times_s,
        model,
        gap,
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
        for (at, (history, flight)) in histories.iter().zip(&flights).enumerate() {
            let code: Vec<f64> = history.samples.values().map(|v| v[index]).collect();
            let residual = |hpr: &[Vec<f64>]| -> Vec<f64> {
                hpr.iter()
                    .zip(&code)
                    .zip(&flight.gap)
                    .map(|((h, c), g)| h[index] - c - g[index])
                    .collect()
            };
            let difference = largest(
                &flight.times_s,
                flight.flown.iter().zip(&code).map(|(h, c)| h[index] - c),
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
            let gap = largest(&flight.times_s, flight.gap.iter().map(|g| g[index]));
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
                "model_difference_there": kept(flight.gap[there][index]),
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
                    model.iter().zip(&code).map(|(m, c)| m[index] - c),
                );
                row["largest_difference_of_the_codes_model"] = json!(kept(from_model.0));
            }
            codes.push(row);
        }
        quantities.push(json!({
            "key": quantity.key,
            "name": quantity.name,
            "unit": quantity.unit,
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
    let environment = environment()?;
    let mut out = Vec::new();
    for case in cases()? {
        let histories = histories(&archive, &case)?;
        out.push(compare(&case, &environment, &histories)?);
    }
    Ok(json!({
        "generator": "cargo xtask check-cases",
        "reference": "NASA/TM-2015-218675, Volumes I and II (Murri, Jackson and Shelton 2015), \
                      errata of 2025-09-08, and the codes' time histories published with it",
        "tight_tolerance": TIGHT_TOLERANCE,
        "note": "HPR Sim's free body (hpr_sim::FreeBody: the flight's gravity, Coriolis force, \
                 rigid-body equations and default integrator) against each code's file over 30 s. \
                 A difference is HPR Sim's value less the code's; a largest one is the largest in \
                 size over the file's samples, with its sign. spread is the largest difference \
                 between any two kept codes at a time every one of them has; a code is left out \
                 when leaving it cuts the spread of all by more than half. model_difference is \
                 the case's named difference between the two models, computed apart from the \
                 flight. left_of_the_largest_difference is a code's largest difference less the \
                 model difference at the same sample, and a quantity is traced when that is no \
                 more than the spread for every kept code. largest_unexplained is the same \
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
/// altitudes in millimeters, rates in deg/s.
fn shown(value: f64, unit: &str) -> String {
    let (value, unit) = match unit {
        "m" => (1000.0 * value, "mm"),
        other => (value, other),
    };
    if value == 0.0 {
        return format!("0 {unit}");
    }
    let decimals = (3 - value.abs().log10().floor() as i32).max(0) as usize;
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
        "# NASA's check cases 1 and 2\n\n\
         Written by `cargo xtask check-cases`; don't edit it by hand. What it checks and how far \
         to trust it: [the accuracy page](../../docs/accuracy.md) and \
         [rigid-body flight](../../docs/physics/flight.md). A free body of the report's mass and \
         inertia flies each case on HPR Sim's gravity, Coriolis force, rigid-body equations and \
         default integrator, and is set beside each code's published time history \
         (NASA/TM-2015-218675). The codes' files are not committed; only differences and spreads \
         are.\n\n\
         How to read it:\n\n\
         - **A difference** is HPR Sim's value less the code's. A largest difference is the \
         largest in size over the file's samples of the 30 s, with its sign.\n\
         - **The spread** is the largest difference between any two codes at a time every one \
         of them has. One code is left out, and named, when leaving it out cuts the spread by \
         more than half; the spread of the rest is then the measure.\n\
         - **The model difference** is a named difference between HPR Sim's model and the \
         codes', sized apart from the flight.\n\
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
            text(case, "model_difference")?,
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
                shown(number(quantity, "model_difference")?, &unit),
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
                } else {
                    ""
                },
                if model { "--:|" } else { "" },
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
                    shown(number(code, "largest_difference")?, &unit),
                    number(code, "largest_difference_at_s")?,
                    shown(number(code, "model_difference_there")?, &unit),
                    shown(number(code, "left_of_the_largest_difference")?, &unit),
                    shown(number(code, "largest_unexplained")?, &unit),
                    shown(
                        number(code, "largest_unexplained_at_tight_tolerances")?,
                        &unit
                    ),
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
                }
                out.push('\n');
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
                .map(|(time, value)| (key(*time), vec![*value]))
                .collect(),
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

    /// With three codes none is left out: any two always agree better than the three.
    #[test]
    fn three_codes_are_all_kept() {
        let mut codes = four(50.0);
        codes.remove(1);
        let rule = spread_rule(&codes, 0);
        assert_eq!((rule.all, rule.left_out, rule.kept), (50.0, None, 50.0));
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
        let altitude = [Quantity {
            key: "altitude",
            name: "altitude",
            column: "altitudeMsl_ft",
            factor: FOOT_M,
            unit: "m",
        }];
        let rows: String = (1..=300)
            .map(|i| format!("{},7,{}\n", 0.1 * f64::from(i), 30_000 - i))
            .collect();
        let text = format!("time,other,altitudeMsl_ft\n0.0,7,30000.0\n{rows}30.1,7,1.0\n");
        let read = history("SIM 9", &text, &altitude).unwrap();
        assert_eq!(read.samples.len(), 301);
        assert_eq!(read.samples[&0], vec![9144.0]);
        assert_eq!(read.samples[&30_000], vec![29_700.0 * FOOT_M]);
        assert_eq!(read.times_s()[3], 0.3);

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
        // The gravitation alone at the start is the 32.1065 ft/s² of the report's sphere.
        let gravitation = (by_hand - omega * omega * r) / FOOT_M;
        assert!((gravitation + 32.106_54).abs() < 1e-5, "{gravitation}");
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
        let environment = environment().unwrap();
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
        let environment = environment().unwrap();
        let times: Vec<f64> = (0..=300).map(|i| 0.1 * f64::from(i)).collect();
        let both = earth_rate_gap_deg_s(&case, &environment, &times);
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
            [1, 2]
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
                for code in codes {
                    // What is left is the difference less the model difference at that sample,
                    // to the four digits each is kept at.
                    let (difference, model, left) = (
                        code["largest_difference"].as_f64().unwrap(),
                        code["model_difference_there"].as_f64().unwrap(),
                        code["left_of_the_largest_difference"].as_f64().unwrap(),
                    );
                    let rounding = 1e-3 * difference.abs().max(model.abs());
                    assert!(
                        (difference - model - left).abs() <= rounding,
                        "{difference} {model} {left}"
                    );
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
        assert_eq!(quantities, 4);
    }

    /// The pages quote the committed report as it is, which CI can't regenerate: the accuracy
    /// page's rows for frames, gravity and rigid-body flight each cite the report and carry its
    /// figures, and the page on rigid-body flight carries every figure of its table and of the
    /// stricter reading.
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
            plain + rest
        };
        let report = committed();
        let quantities: Vec<&Value> = report["cases"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|case| case["quantities"].as_array().unwrap())
            .collect();
        // A size as a page prints it, and a signed figure with the pages' minus sign.
        let size = |quantity: &Value, key: &str| {
            shown_size(
                quantity[key].as_f64().unwrap(),
                quantity["unit"].as_str().unwrap(),
            )
        };
        let signed = |quantity: &Value, key: &str| {
            shown(
                quantity[key].as_f64().unwrap(),
                quantity["unit"].as_str().unwrap(),
            )
            .replace('-', "−")
        };
        let has = |path: &str, text: &str, figure: &str| {
            assert!(text.contains(figure), "{path} doesn't quote {figure}");
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
        let (altitude, rates) = (quantities[0], &quantities[1..]);
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

        // The start page's paragraph gives the altitude's largest difference and the largest
        // of the rates'.
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
        has(
            "docs/start-here.md",
            &start,
            &size(most, key).replace(" deg/s", "\ndeg/s"),
        );
    }

    /// The committed page is the committed report's, so the two can't drift apart.
    #[test]
    fn the_committed_page_is_the_reports() {
        let root = crate::designs::root().unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(page, markdown(&committed()).unwrap());
    }
}
