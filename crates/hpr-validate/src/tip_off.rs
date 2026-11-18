//! HPR Sim's tip-off phase, flown as RocketPy flew its own: the flights behind
//! `validation/reports/tip-off-model.json` ([the tip-off phase, held to RocketPy's][m1-14b11];
//! the tip-off decision record, [ADR-357][adr-357], decision 4).
//!
//! `validation/reports/tip-off.json` holds what RocketPy's tip-off phase does to thirteen designs
//! built from its examples, in still air and in a crosswind, with everything each flight carried. [`fly`]
//! flies one of those flights in HPR Sim from that record: the public design built from the same
//! example, the record's rail, site, wind, parachutes and declared drag coefficient, guided to
//! the last guide as HPR Sim flies by default, or with its tip-off phase
//! ([`hpr_sim::Simulation::with_tip_off`]).
//!
//! [Loft lesson L75][l75]: the inputs come from the oracle's own record of what it flew, and the
//! design is checked against that record (its dry mass, the area its drag acts on, its motor)
//! before any difference is read as physics.
//!
//! What is measured, as the oracle's script measures it:
//!
//! - **The apogee** is the center of mass's greatest height above where the center of dry mass
//!   started, which is where RocketPy's tracked point starts.
//! - **The landing point** is where the center of dry mass is, east and north of where it
//!   started, when it is back at its starting height.
//! - **Where the last guide leaves the rail**, the angle between the rocket's axis and the
//!   rail's as two parts, nose below the rail's line in the rail's vertical plane and nose to the
//!   right of its heading, and the rate at which the nose's direction moves, as the same two
//!   parts.
//!
//! [m1-14b11]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b11
//! [adr-357]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
//! [l75]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l75

use std::path::Path;

use hpr_aero::{DragTable, Flow, NormalForceColumn, NormalForceTable};
use hpr_core::DVec3;
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use hpr_design::{MassProperties, Rocket};
use hpr_sim::{
    Device, DeviceDrag, Direction, EventKind, FlightSettings, Rail, Sample, Simulation,
    Termination, Trigger, UserEvent,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::run::{AREA_AGREEMENT, MASS_AGREEMENT, ValidateError, rocketpy_environment};

/// The state where the last guide leaves the rail.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Release {
    /// The time after ignition, s.
    pub time_s: f64,
    /// The nose tip's speed, m/s.
    pub speed_m_s: f64,
    /// The angle of the rocket's axis below the rail's line, in the rail's vertical plane, deg.
    pub nose_down_deg: f64,
    /// Its angle to the right of the rail's heading, deg.
    pub nose_right_deg: f64,
    /// How fast the nose's direction swings below the rail's line, deg/s.
    pub pitch_rate_nose_down_deg_s: f64,
    /// How fast it swings to the right of the heading, deg/s.
    pub pitch_rate_nose_right_deg_s: f64,
    /// The angle of attack there, deg.
    pub angle_of_attack_deg: f64,
    /// The Mach number there.
    pub mach: f64,
    /// The normal-force coefficient the flight's aerodynamics give at that Mach number and angle
    /// of attack, on the reference area.
    pub normal_force_coefficient: f64,
    /// That force's moment about the last guide's middle per unit dynamic pressure and reference
    /// area, m: `Σ C_N,i (x_guide − X_i)`, positive when the force acts ahead of the guide and so
    /// swings the nose downwind.
    pub pivot_moment_m: f64,
}

/// One flight's figures.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Flown {
    /// The apogee above the starting height, m.
    pub apogee_agl_m: f64,
    /// The landing point east of the start, m.
    pub landing_east_m: f64,
    /// The landing point north of the start, m.
    pub landing_north_m: f64,
    /// The time of the landing, s.
    pub flight_time_s: f64,
    /// Where the last guide leaves the rail.
    pub last_guide_leaves: Release,
}

/// Which normal force a flight flies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalForce {
    /// HPR Sim's own, from the design.
    Own,
    /// The one the oracle recorded as RocketPy's for the rocket (the record's `normal_force`):
    /// a slope against Mach number times the angle of attack, at a center of pressure against
    /// Mach number. The pitch and yaw damping stay HPR Sim's
    /// ([`hpr_sim::Simulation::with_normal_force_table`]).
    Recorded,
}

/// The largest angle of attack the recorded normal force's table is given at, rad: RocketPy's
/// force is its slope times the angle at any angle, and between two columns of one slope the
/// table is exactly that. Past it the table's own rule takes over. Where the last guide leaves
/// the angle is at most 0.45 rad (25.6°), far inside. Whether a flight passes 1.5 rad later, at
/// the top of its path, is not recorded; the guided and the tip-off flight fly the same table.
const RECORDED_TOP_ANGLE_RAD: f64 = 1.5;

/// Flies, in HPR Sim, the flight of `case` (a case of the oracle's report, with its `rail` and
/// `site`) in `wind` (one of the case's `winds`, with the wind and what the flight carried) on
/// the constant drag coefficient `declared_cd0` (the report's `[Mach, C_D]` rows) and the
/// normal force `normal_force`, with the tip-off phase if `tip_off` and guided to the last
/// guide otherwise.
///
/// # Errors
///
/// [`ValidateError`] if the record is not the oracle's shape, the design is not the rocket the
/// record flew (its dry mass, reference area or motor differ past rounding), or the flight fails
/// or does not end at the ground.
pub fn fly(
    root: &Path,
    case: &Value,
    wind: &Value,
    declared_cd0: &Value,
    (normal_force, tip_off): (NormalForce, bool),
) -> Result<Flown, ValidateError> {
    let name = case["name"].as_str().unwrap_or("(unnamed)").to_owned();
    let label = format!(
        "{name}, {}, {}, {}",
        wind["wind"].as_str().unwrap_or("(no wind named)"),
        match normal_force {
            NormalForce::Own => "its own normal force",
            NormalForce::Recorded => "the recorded normal force",
        },
        if tip_off { "tip-off" } else { "guided" }
    );
    let refuse = |what: String| ValidateError::Flight {
        case: label.clone(),
        what,
        source: None,
    };
    let failed = |source: hpr_sim::SimError| ValidateError::Flight {
        case: label.clone(),
        what: "the flight failed".to_owned(),
        source: Some(Box::new(source)),
    };
    let number = |value: &Value, what: &str| {
        value
            .as_f64()
            .filter(|x| x.is_finite())
            .ok_or_else(|| refuse(format!("the record has no number for {what}")))
    };
    let (rail, site, carried) = (&case["rail"], &case["site"], &wind["flown_with"]);
    let path = root.join(format!("validation/designs/rocketpy-{name}.json"));
    let text = std::fs::read_to_string(&path).map_err(|source| ValidateError::Io {
        what: "reading a design",
        path: path.display().to_string(),
        source,
    })?;
    let rocket: Rocket = serde_json::from_str(&text).map_err(|source| ValidateError::Json {
        path: path.display().to_string(),
        source: Box::new(source),
    })?;
    let [configuration] = rocket.configurations.as_slice() else {
        return Err(refuse(
            "the design has other than one configuration".to_owned(),
        ));
    };
    let elevation_m = number(&site["elevation"], "the site's elevation")?;
    let environment = rocketpy_environment(
        number(&site["latitude"], "the site's latitude")?,
        number(&site["longitude"], "the site's longitude")?,
        elevation_m,
        &[(
            elevation_m,
            number(&wind["wind_east_m_s"], "the wind's east part")?,
            number(&wind["wind_north_m_s"], "the wind's north part")?,
        )],
    )
    .map_err(&failed)?;
    let rail = Rail {
        length_m: number(&rail["rail_length_m"], "the rail's length")?,
        azimuth_rad: number(&rail["heading_deg"], "the rail's heading")?.to_radians(),
        elevation_rad: number(&rail["inclination_deg"], "the rail's inclination")?.to_radians(),
        roll_rad: 0.0,
        // RocketPy's rail has no friction (ADR-011).
        friction_coefficient: 0.0,
    };
    let rows = declared_cd0
        .as_array()
        .ok_or_else(|| refuse("the record declares no drag table".to_owned()))?
        .iter()
        .map(|row| Ok((number(&row[0], "a Mach number")?, number(&row[1], "a C_D")?)))
        .collect::<Result<Vec<_>, ValidateError>>()?;
    let (machs, coefficients): (Vec<f64>, Vec<f64>) = rows.into_iter().unzip();
    let radius_m = number(&carried["reference_radius_m"], "the reference radius")?;
    let simulation = Simulation::new(
        &rocket,
        &configuration.id,
        environment,
        rail,
        FlightSettings {
            max_time_s: 6000.0,
            ..FlightSettings::default()
        },
    )
    .map_err(&failed)?;
    let table = Table1D::new(
        machs,
        coefficients,
        Interpolation::Linear,
        // Outside the declared Mach range there is no declared drag, so none is invented.
        Extrapolation::Error,
    )
    .map_err(|error| refuse(format!("the declared drag table: {error}")))?;
    let simulation = simulation.with_drag_table(
        DragTable::new(table.clone(), Some(table))
            .with_reference_diameter_m(2.0 * radius_m)
            .map_err(|error| refuse(format!("the declared drag's reference diameter: {error}")))?,
    );
    let simulation = match normal_force {
        NormalForce::Own => simulation,
        NormalForce::Recorded => {
            let recorded = &carried["normal_force"];
            let column = |what: &str| {
                recorded[what]
                    .as_array()
                    .ok_or_else(|| refuse(format!("the record's normal force has no {what}")))?
                    .iter()
                    .map(|value| number(value, what))
                    .collect::<Result<Vec<_>, ValidateError>>()
            };
            let against_mach = |values: Vec<f64>| {
                Table1D::new(
                    column("mach")?,
                    values,
                    Interpolation::Linear,
                    Extrapolation::Clamp,
                )
                .map_err(|error| refuse(format!("the recorded normal force: {error}")))
            };
            let slope = against_mach(column("slope_per_rad")?)?;
            let station = against_mach(column("cp_station_m")?)?;
            let table = NormalForceTable::new(vec![
                NormalForceColumn::new(0.0, slope.clone(), station.clone()),
                NormalForceColumn::new(RECORDED_TOP_ANGLE_RAD, slope, station),
            ])
            .and_then(|table| table.with_reference_diameter_m(2.0 * radius_m))
            .map_err(|error| refuse(format!("the recorded normal force: {error}")))?;
            simulation.with_normal_force_table(table).map_err(&failed)?
        }
    };
    let simulation = if tip_off {
        simulation.with_tip_off()
    } else {
        simulation
    };

    // L75: the same rocket has to weigh the same, fly its drag on the same area and burn the
    // same motor in both codes.
    let assembly = simulation.assembly();
    let dry = assembly
        .motors
        .iter()
        .fold(assembly.layout.structure, |sum, motor| {
            MassProperties::combine([&sum, &motor.dry_mass_properties()])
        });
    let [placed] = assembly.motors.as_slice() else {
        return Err(refuse(format!(
            "the design flies {} motors, where RocketPy's examples fly one",
            assembly.motors.len()
        )));
    };
    let motor = &placed.mounted.motor;
    let recorded = &carried["motor"];
    for (what, hpr, rocketpy, agreement) in [
        (
            "dry mass, kg",
            dry.mass_kg,
            number(&carried["dry_mass_kg"], "the dry mass")?,
            MASS_AGREEMENT,
        ),
        (
            "reference radius, m",
            0.5 * assembly.layout.reference_diameter_m,
            radius_m,
            AREA_AGREEMENT,
        ),
        (
            "total impulse, N s",
            motor.curve().total_impulse_ns(),
            number(&recorded["total_impulse_ns"], "the total impulse")?,
            MASS_AGREEMENT,
        ),
        (
            "burn-out time, s",
            motor.burnout_time_s(),
            number(&recorded["burn_out_time_s"], "the burn-out time")?,
            MASS_AGREEMENT,
        ),
        (
            "initial propellant mass, kg",
            motor.propellant_initial_mass_kg(),
            number(
                &recorded["propellant_initial_mass_kg"],
                "the propellant mass",
            )?,
            MASS_AGREEMENT,
        ),
    ] {
        let gap = (hpr - rocketpy).abs() / rocketpy.abs();
        if !gap.is_finite() || gap > agreement {
            return Err(refuse(format!(
                "hpr's {what} is {hpr} where the record flew {rocketpy}, a relative {gap:.3e}"
            )));
        }
    }

    // RocketPy's tracked point, the center of dry mass, starts at the ground; hpr's starts `h0`
    // above it, with the rocket's aft end at the rail's foot. Heights are measured from `h0`.
    let start = simulation.initial_state();
    let origin = start.position_enu_m + start.attitude.mul_vec3(dry.cg_m);
    let h0 = origin.z;
    let devices = carried["devices"]
        .as_array()
        .ok_or_else(|| refuse("the record lists no devices".to_owned()))?;
    let count = devices.len();
    let devices = devices
        .iter()
        .enumerate()
        .map(|(index, device)| {
            let trigger = match device["trigger"]["kind"].as_str() {
                Some("apogee") if index == 0 => Trigger::Apogee,
                Some("descending_below_height_agl") if index > 0 => Trigger::Altitude {
                    height_above_ground_m: number(
                        &device["trigger"]["height_m"],
                        "a deployment height",
                    )? + h0,
                },
                other => {
                    return Err(refuse(format!(
                        "device {index} has the trigger {other:?}; RocketPy's first parachute \
                         opens at apogee and the rest at a height"
                    )));
                }
            };
            let mut built = Device::new(
                device["name"].as_str().unwrap_or("parachute"),
                DeviceDrag::DragArea {
                    cd_s_m2: number(&device["cd_s_m2"], "a drag area")?,
                },
                trigger,
            )
            .with_lag_s(number(&device["lag_s"], "a lag")?);
            // RocketPy flies one parachute at a time, the last deployed; hpr sums its open
            // devices, so each one releases the one before as it opens.
            if index + 1 < count {
                built = built.with_release_by(index + 1);
            }
            Ok(built)
        })
        .collect::<Result<Vec<_>, ValidateError>>()?;
    let simulation = simulation
        .with_recovery(devices)
        .map_err(&failed)?
        .with_event(UserEvent {
            name: "back at the starting height".to_owned(),
            direction: Direction::Falling,
            function: Box::new(move |sample: &Sample| sample.height_above_ground_m - h0),
        });
    let result = simulation.run(&mut ()).map_err(&failed)?;
    if result.termination != Termination::GroundHit {
        return Err(refuse(format!(
            "the flight ended as {:?}, not at the ground",
            result.termination
        )));
    }
    let event = |kind: EventKind| {
        result
            .event(kind)
            .map(|event| event.sample)
            .ok_or_else(|| refuse(format!("the flight has no {kind:?} event")))
    };
    let (exit, apogee, landing) = (
        event(EventKind::RailExit)?,
        event(EventKind::Apogee)?,
        event(EventKind::User(0))?,
    );
    let landed = landing.state.position_enu_m + landing.state.attitude.mul_vec3(dry.cg_m) - origin;

    // The rail's axis, the axis square to it in its vertical plane (pointing up) and the
    // horizontal axis to the right of its heading, as the oracle's script takes them.
    let (heading, inclination) = (rail.azimuth_rad, rail.elevation_rad);
    let up = DVec3::new(
        -inclination.sin() * heading.sin(),
        -inclination.sin() * heading.cos(),
        inclination.cos(),
    );
    let right = DVec3::new(heading.cos(), -heading.sin(), 0.0);
    // The static normal force where the last guide leaves, and its moment about that guide.
    let pivot_station_m = simulation
        .guides()
        .pivot_station_m
        .ok_or_else(|| refuse("the design has no guides".to_owned()))?;
    let force = simulation
        .aero()
        .normal_force(&Flow::new(exit.mach, exit.angle_of_attack_rad, 0.0))
        .map_err(|error| {
            refuse(format!(
                "the normal force where the last guide leaves: {error}"
            ))
        })?;
    let attitude = exit.state.unit_attitude();
    let axis = attitude.mul_vec3(DVec3::Z);
    // How fast the nose's direction moves: the body rate crossed with the long axis.
    let turning = attitude.mul_vec3(exit.state.body_rate_rad_s.cross(DVec3::Z));
    let flown = Flown {
        apogee_agl_m: apogee.height_above_ground_m - h0,
        landing_east_m: landed.x,
        landing_north_m: landed.y,
        flight_time_s: landing.time_s,
        last_guide_leaves: Release {
            time_s: exit.time_s,
            speed_m_s: exit.state.velocity_enu_m_s.length(),
            nose_down_deg: (-axis.dot(up)).asin().to_degrees(),
            nose_right_deg: axis.dot(right).asin().to_degrees(),
            pitch_rate_nose_down_deg_s: (-turning.dot(up)).to_degrees(),
            pitch_rate_nose_right_deg_s: turning.dot(right).to_degrees(),
            angle_of_attack_deg: exit.angle_of_attack_rad.to_degrees(),
            mach: exit.mach,
            normal_force_coefficient: force.coefficient,
            pivot_moment_m: force.coefficient * pivot_station_m - force.moment_m,
        },
    };
    let figures = [
        flown.apogee_agl_m,
        flown.landing_east_m,
        flown.landing_north_m,
        flown.flight_time_s,
        flown.last_guide_leaves.time_s,
        flown.last_guide_leaves.speed_m_s,
        flown.last_guide_leaves.nose_down_deg,
        flown.last_guide_leaves.nose_right_deg,
        flown.last_guide_leaves.pitch_rate_nose_down_deg_s,
        flown.last_guide_leaves.pitch_rate_nose_right_deg_s,
        flown.last_guide_leaves.angle_of_attack_deg,
        flown.last_guide_leaves.mach,
        flown.last_guide_leaves.normal_force_coefficient,
        flown.last_guide_leaves.pivot_moment_m,
    ];
    if figures.iter().any(|figure| !figure.is_finite()) {
        return Err(refuse(format!("a figure is not a number: {flown:?}")));
    }
    Ok(flown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::root;

    /// The committed reference's flights, and the case and wind named.
    fn reference() -> Value {
        let text = std::fs::read_to_string(root().join("validation/reports/tip-off.json"))
            .expect("the reference is committed");
        serde_json::from_str(&text).expect("the reference parses")
    }

    fn row(reference: &Value, name: &str, wind: &str) -> (Value, Value) {
        let case = reference["flown"]["cases"]
            .as_array()
            .expect("the reference lists its cases")
            .iter()
            .find(|case| case["name"] == name)
            .expect("the case is in the reference");
        let wind = case["winds"]
            .as_array()
            .expect("the case lists its winds")
            .iter()
            .find(|row| row["wind"] == wind)
            .expect("the wind is in the case");
        (case.clone(), wind.clone())
    }

    #[test]
    fn the_recorded_normal_force_is_rocketpys_line_at_the_angle_the_guide_leaves_at() {
        // RocketPy's normal force is its slope at the Mach number times the angle of attack, at
        // any angle. The table flown here has two columns of that one slope, so the coefficient
        // where the last guide leaves is the slope times the angle, at 22° as at 1°: a table
        // that bent, or a slope scaled by the wrong area, fails here.
        let reference = reference();
        let drag = &reference["flown"]["drag"]["declared_cd0"];
        let (case, wind) = row(&reference, "valetudo", "crosswind");
        let flown = fly(&root(), &case, &wind, drag, (NormalForce::Recorded, true))
            .expect("the flight flies");
        let release = flown.last_guide_leaves;
        assert!(
            release.angle_of_attack_deg > 20.0,
            "the crosswind flight leaves at a large angle: {release:?}"
        );
        let recorded = &wind["flown_with"]["normal_force"];
        let column = |what: &str| -> Vec<f64> {
            recorded[what]
                .as_array()
                .expect("the column is recorded")
                .iter()
                .map(|value| value.as_f64().expect("a number"))
                .collect()
        };
        let slope = Table1D::new(
            column("mach"),
            column("slope_per_rad"),
            Interpolation::Linear,
            Extrapolation::Error,
        )
        .expect("the recorded slope is a table")
        .eval(release.mach)
        .expect("the Mach number is inside the record");
        let line = slope * release.angle_of_attack_deg.to_radians();
        assert!(
            (release.normal_force_coefficient - line).abs() < 1e-9 * line,
            "C_N {} against slope × angle {line}",
            release.normal_force_coefficient
        );
        // HPR Sim's own normal force at that angle is more than twice the line.
        let own = fly(&root(), &case, &wind, drag, (NormalForce::Own, true))
            .expect("the flight flies")
            .last_guide_leaves;
        assert!(
            own.normal_force_coefficient > 2.0 * slope * own.angle_of_attack_deg.to_radians(),
            "{own:?}"
        );
    }

    #[test]
    fn a_design_that_is_not_the_rocket_the_record_flew_is_refused() {
        // L75: a comparison of two rockets is no comparison. Each of the record's five figures
        // moved by 1% is refused by name, as is a record with no normal force to fly on.
        let reference = reference();
        let drag = &reference["flown"]["drag"]["declared_cd0"];
        let (case, wind) = row(&reference, "valetudo", "still");
        for (pointer, named) in [
            ("/flown_with/dry_mass_kg", "dry mass"),
            ("/flown_with/reference_radius_m", "reference radius"),
            ("/flown_with/motor/total_impulse_ns", "total impulse"),
            ("/flown_with/motor/burn_out_time_s", "burn-out time"),
            (
                "/flown_with/motor/propellant_initial_mass_kg",
                "initial propellant mass",
            ),
        ] {
            let mut other = wind.clone();
            let figure = other
                .pointer_mut(pointer)
                .expect("the record has the figure");
            *figure = (1.01 * figure.as_f64().expect("a number")).into();
            let error = fly(&root(), &case, &other, drag, (NormalForce::Own, true))
                .expect_err("another rocket is refused")
                .to_string();
            assert!(error.contains(named), "{pointer}: {error}");
        }
        let mut bare = wind.clone();
        bare["flown_with"]["normal_force"] = Value::Null;
        let error = fly(&root(), &case, &bare, drag, (NormalForce::Recorded, true))
            .expect_err("no recorded normal force, no flight on it")
            .to_string();
        assert!(error.contains("normal force"), "{error}");
        // The same record flies on HPR Sim's own.
        fly(&root(), &case, &bare, drag, (NormalForce::Own, true)).expect("the flight flies");
    }

    #[test]
    fn the_phase_is_only_flown_when_asked_for() {
        // Guided, the rocket leaves its last guide along the rail with no turning rate; with the
        // phase it leaves turning nose down under gravity, at the same time and speed to a part
        // in a thousand.
        let reference = reference();
        let drag = &reference["flown"]["drag"]["declared_cd0"];
        let (case, wind) = row(&reference, "valetudo", "still");
        let guided = fly(&root(), &case, &wind, drag, (NormalForce::Own, false))
            .expect("the flight flies")
            .last_guide_leaves;
        let tipped = fly(&root(), &case, &wind, drag, (NormalForce::Own, true))
            .expect("the flight flies")
            .last_guide_leaves;
        assert!(guided.pitch_rate_nose_down_deg_s.abs() < 1e-9, "{guided:?}");
        assert!(guided.nose_down_deg.abs() < 1e-9, "{guided:?}");
        assert!(tipped.pitch_rate_nose_down_deg_s > 0.1, "{tipped:?}");
        assert!(tipped.nose_down_deg > 0.0, "{tipped:?}");
        assert!((tipped.time_s - guided.time_s).abs() < 1e-3 * guided.time_s);
        assert!((tipped.speed_m_s - guided.speed_m_s).abs() < 1e-3 * guided.speed_m_s);
    }
}
