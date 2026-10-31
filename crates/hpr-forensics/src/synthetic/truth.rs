//! A synthetic flight's truth: HPR Sim's flight of the drawn rocket to the ground, sampled every
//! [`TRUTH_STEP_S`], with its events.

use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use hpr_aero::{AeroError, DragModel, DragQuery};
use hpr_analysis::montecarlo::{DragOverride, FlightInputs, dispersed_motor};
use hpr_atmos::{AtmosphereModel, ConstantWind, PowerLawWind, Ussa76};
use hpr_core::DVec3;
use hpr_core::earth::Earth;
use hpr_core::geodesy::Geodetic;
use hpr_design::{Ignition, Rocket};
use hpr_motor::{SolidMotor, ThrustCurve};
use hpr_sim::recorder::{FlightStep, Observer, Sample};
use hpr_sim::recovery::{Device, DeviceDrag, Separation, Trigger};
use hpr_sim::{
    BodyFlight, BodySample, Environment, EventKind, FlightEvent, Phase, Rail, SimError, Simulation,
};
use serde::{Deserialize, Serialize};

use super::designs::{DESIGNS, Design, DesignFiles, Opens};
use super::{Draw, Model, SyntheticError};

/// The truth's sample interval, s: a row every 0.01 s from launch, and one at each event.
pub const TRUTH_STEP_S: f64 = 0.01;
/// The rail's length, m: RocketPy's Calisto example's 5.2 m, for every design.
pub const RAIL_LENGTH_M: f64 = 5.2;
/// The site's latitude, degrees north: fixed, as nothing here depends on where the field is but
/// its elevation, which is drawn.
pub const SITE_LATITUDE_DEG: f64 = 35.0;
/// The site's longitude, degrees east.
pub const SITE_LONGITUDE_DEG: f64 = -106.0;
/// The height at which set B's power-law wind takes the drawn speed, m: NASA/TM-2008-215633's
/// reference height, as `hpr_atmos::PowerLawWind` cites it.
pub const SHEAR_REFERENCE_HEIGHT_M: f64 = 18.3;
/// The sea-level pressure of the drawn air, Pa: the standard's.
const SEA_LEVEL_PRESSURE_PA: f64 = 101_325.0;
/// Standard gravity, m/s²: what an accelerometer at rest reads.
const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;

/// The truth at one instant.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TruthSample {
    /// Time since launch, s.
    pub time_s: f64,
    /// The center of mass's height above the ground (the site's ellipsoidal height), m.
    pub height_m: f64,
    /// Its rate, m/s, up positive.
    pub vertical_speed_m_s: f64,
    /// The specific force along the rocket's axis, nose positive, m/s²: what an accelerometer
    /// along the axis reads, gravity's reaction included (+9.8 m/s² standing on the pad), at the
    /// nose tip, where the simulator records the acceleration. Under a canopy the rocket is taken
    /// to hang along the vertical.
    pub axial_specific_force_m_s2: f64,
    /// The Mach number.
    pub mach: f64,
    /// The dynamic pressure, Pa.
    pub dynamic_pressure_pa: f64,
    /// The center of mass in the launch frame (east, north, up), m.
    pub position_enu_m: [f64; 3],
    /// Its velocity in the launch frame, m/s.
    pub velocity_enu_m_s: [f64; 3],
}

/// The events of a flight, s since launch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TruthEvents {
    /// The rocket started to move along the rail.
    pub liftoff_s: f64,
    /// The last rail guide left the rail.
    pub rail_exit_s: f64,
    /// The motor burned out.
    pub burnout_s: f64,
    /// The center of mass's highest point.
    pub apogee_s: f64,
    /// Its height there above the ground, m.
    pub apogee_m: f64,
    /// Each parachute's charge, by the parachute's name, in the order they fired.
    pub charges: Vec<(String, f64)>,
    /// Each parachute's opening, by name, in the order they opened.
    pub deployments: Vec<(String, f64)>,
    /// The center of mass reached the ground.
    pub landing_s: f64,
}

/// A flight's truth.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Truth {
    /// The samples, every [`TRUTH_STEP_S`] from launch and at each event, in time order.
    pub samples: Vec<TruthSample>,
    /// The events.
    pub events: TruthEvents,
    /// The center of mass's height above the ground at launch, m: where a logger in the rocket
    /// sat when it set its zero. A reading's truth is a height above it.
    pub pad_height_m: f64,
    /// The site's elevation above mean sea level, m.
    pub site_elevation_m: f64,
    /// The air's temperature offset from the 1976 standard, K.
    pub temperature_offset_k: f64,
}

impl Truth {
    /// The truth at `t` s since launch, read linearly between samples; before launch the rocket
    /// stands on the pad, and after landing it lies still on its side. `None` for a truth with no
    /// samples, or a `t` that isn't a number.
    pub fn at(&self, t: f64) -> Option<TruthSample> {
        let (first, last) = (*self.samples.first()?, *self.samples.last()?);
        if t.is_nan() {
            return None;
        }
        if t <= first.time_s {
            return Some(TruthSample { time_s: t, ..first });
        }
        if t > last.time_s {
            return Some(TruthSample {
                time_s: t,
                height_m: last.height_m.max(0.0),
                vertical_speed_m_s: 0.0,
                axial_specific_force_m_s2: 0.0,
                mach: 0.0,
                dynamic_pressure_pa: 0.0,
                velocity_enu_m_s: [0.0; 3],
                ..last
            });
        }
        // `t` lies in (first, last], so the sample at or before it has one after it.
        let i = (self.samples.partition_point(|s| s.time_s <= t) - 1).min(self.samples.len() - 2);
        let (a, b) = (self.samples[i], self.samples[i + 1]);
        let f = (t - a.time_s) / (b.time_s - a.time_s);
        let mix = |x: f64, y: f64| x + f * (y - x);
        let mix3 = |x: [f64; 3], y: [f64; 3]| [mix(x[0], y[0]), mix(x[1], y[1]), mix(x[2], y[2])];
        Some(TruthSample {
            time_s: t,
            height_m: mix(a.height_m, b.height_m),
            vertical_speed_m_s: mix(a.vertical_speed_m_s, b.vertical_speed_m_s),
            axial_specific_force_m_s2: mix(
                a.axial_specific_force_m_s2,
                b.axial_specific_force_m_s2,
            ),
            mach: mix(a.mach, b.mach),
            dynamic_pressure_pa: mix(a.dynamic_pressure_pa, b.dynamic_pressure_pa),
            position_enu_m: mix3(a.position_enu_m, b.position_enu_m),
            velocity_enu_m_s: mix3(a.velocity_enu_m_s, b.velocity_enu_m_s),
        })
    }

    /// The true static pressure at the center of mass `t` s since launch, Pa.
    ///
    /// # Errors
    ///
    /// As [`Ussa76::sample`], for a height it can't take; [`SyntheticError::Invalid`] where
    /// [`Truth::at`] has no truth.
    pub fn pressure_pa(&self, t: f64) -> Result<f64, SyntheticError> {
        let air = self.air()?;
        let state = self.at(t).ok_or(SyntheticError::Invalid {
            what: "a truth with no sample at that time",
        })?;
        Ok(air
            .sample(self.site_elevation_m + state.height_m)?
            .air
            .pressure_pa)
    }

    /// The flight's air.
    ///
    /// # Errors
    ///
    /// As [`Ussa76::with_offset`].
    pub fn air(&self) -> Result<Ussa76, SyntheticError> {
        Ok(Ussa76::with_offset(
            self.temperature_offset_k,
            SEA_LEVEL_PRESSURE_PA,
        )?)
    }

    /// The apogee above where the logger sat on the pad, m.
    pub fn apogee_above_pad_m(&self) -> f64 {
        self.events.apogee_m - self.pad_height_m
    }

    /// The highest vertical speed, m/s, and when; NaN if any sample's speed is NaN, so that a
    /// census refuses the flight rather than ranking around it.
    pub fn top_speed(&self) -> (f64, f64) {
        self.samples
            .iter()
            .map(|s| (s.vertical_speed_m_s, s.time_s))
            .fold((f64::NEG_INFINITY, 0.0), |best, x| {
                if x.0.is_nan() || best.0.is_nan() {
                    (f64::NAN, x.1)
                } else if x.0 > best.0 {
                    x
                } else {
                    best
                }
            })
    }

    /// The highest Mach number; NaN if any sample's is.
    pub fn top_mach(&self) -> f64 {
        self.samples.iter().map(|s| s.mach).fold(0.0, |best, m| {
            if m.is_nan() || best.is_nan() {
                f64::NAN
            } else {
                best.max(m)
            }
        })
    }
}

/// Set B's drag: HPR Sim's zero-lift drag times `scale · (1 + a·exp(−((M − 1)/0.2)²) + b·M)`, a
/// transonic rise and a slope shaped differently from HPR Sim's.
#[derive(Debug, Clone, Copy)]
pub struct ReshapedDrag {
    /// The constant factor.
    pub scale: f64,
    /// The transonic bump's height.
    pub bump: f64,
    /// The slope per unit Mach number.
    pub slope: f64,
}

impl ReshapedDrag {
    /// The factor on HPR Sim's zero-lift drag at Mach `mach`.
    pub fn factor(&self, mach: f64) -> f64 {
        let x = (mach - 1.0) / 0.2;
        self.scale * (1.0 + self.bump * (-x * x).exp() + self.slope * mach)
    }
}

impl DragModel for ReshapedDrag {
    fn zero_lift_drag(&self, query: &DragQuery<'_>) -> Result<f64, AeroError> {
        Ok(self.factor(query.mach()) * query.buildup()?.zero_lift_coefficient)
    }
}

/// `curve` tilted by `1 + tilt·(2t/t_b − 1)` over its burn `t_b` and rescaled to its own
/// impulse: progressive for a positive tilt, regressive for a negative one.
///
/// # Errors
///
/// As [`ThrustCurve::new`].
pub fn tilted(curve: &ThrustCurve, tilt: f64) -> Result<ThrustCurve, SyntheticError> {
    let times = curve.times_s();
    let burn = times[times.len() - 1];
    let thrusts: Vec<f64> = times
        .iter()
        .zip(curve.thrusts_n())
        .map(|(&t, &f)| f * (1.0 + tilt * (2.0 * t / burn - 1.0)))
        .collect();
    let raw = ThrustCurve::new(times.to_vec(), thrusts)?;
    let k = curve.total_impulse_ns() / raw.total_impulse_ns();
    Ok(ThrustCurve::new(
        times.to_vec(),
        raw.thrusts_n().iter().map(|f| f * k).collect(),
    )?)
}

/// The draw's design, if its index names one.
fn design_of(draw: &Draw) -> Result<&'static Design, SyntheticError> {
    DESIGNS.get(draw.design).ok_or(SyntheticError::Invalid {
        what: "a draw whose design isn't one of the ten",
    })
}

/// The drawn rocket, its motor scaled and, in set B, its thrust tilted.
fn rocket(draw: &Draw, files: &DesignFiles) -> Result<Rocket, SyntheticError> {
    let design = design_of(draw)?;
    let json = files.json(draw.design).ok_or(SyntheticError::Invalid {
        what: "no file for the draw's design",
    })?;
    let mut rocket: Rocket = serde_json::from_str(json)?;
    let configuration = rocket
        .configurations
        .iter_mut()
        .find(|c| c.id == design.configuration)
        .ok_or(SyntheticError::Design {
            design: design.name,
            what: "has no such configuration",
        })?;
    for mounted in &mut configuration.motors {
        let motor = dispersed_motor(&mounted.motor, draw.impulse_scale, draw.burn_time_scale)?;
        mounted.motor = match draw.model {
            Model::Same => motor,
            Model::Other { thrust_tilt, .. } => SolidMotor::new(
                tilted(motor.curve(), thrust_tilt)?,
                motor.propellant().clone(),
                motor.dry(),
                motor.nozzle(),
            )?,
        };
    }
    Ok(rocket)
}

/// The design's parachutes as HPR Sim's devices, each released by the next.
fn devices(draw: &Draw) -> Result<Vec<Device>, SyntheticError> {
    let chutes = design_of(draw)?.chutes;
    Ok(chutes
        .iter()
        .enumerate()
        .map(|(index, chute)| {
            let trigger = match chute.opens {
                Opens::Apogee => Trigger::Apogee,
                Opens::Below(height_above_ground_m) => Trigger::Altitude {
                    height_above_ground_m,
                },
            };
            let device = Device::new(
                chute.name,
                DeviceDrag::DragArea {
                    cd_s_m2: chute.cd_s_m2,
                },
                trigger,
            )
            .with_lag_s(chute.lag_s);
            if index + 1 < chutes.len() {
                device.with_release_by(index + 1)
            } else {
                device
            }
        })
        .collect())
}

/// The inputs a draw flies from, its design read from `files`.
///
/// # Errors
///
/// [`SyntheticError`] for a design that doesn't read, a motor the draw can't scale, or an
/// environment out of its models' domains.
pub fn inputs(draw: &Draw, files: &DesignFiles) -> Result<FlightInputs, SyntheticError> {
    let mut inputs = FlightInputs::new(
        rocket(draw, files)?,
        design_of(draw)?.configuration,
        environment(draw)?,
        rail(draw),
    );
    inputs.recovery = devices(draw)?;
    match draw.model {
        Model::Same => inputs.drag_scale = draw.drag_scale,
        Model::Other {
            drag_bump,
            drag_slope,
            ..
        } => {
            inputs.drag = Some(DragOverride::Model(Arc::new(ReshapedDrag {
                scale: draw.drag_scale,
                bump: drag_bump,
                slope: drag_slope,
            })));
        }
    }
    Ok(inputs)
}

/// The draw's site, air and wind.
fn environment(draw: &Draw) -> Result<Environment, SyntheticError> {
    let site = Geodetic::from_degrees(SITE_LATITUDE_DEG, SITE_LONGITUDE_DEG, draw.elevation_m)?;
    let air = AtmosphereModel::Standard(Ussa76::with_offset(
        draw.temperature_offset_k,
        SEA_LEVEL_PRESSURE_PA,
    )?);
    let earth = Earth::wgs84(site)?;
    Ok(match draw.model {
        Model::Same => Environment::new(
            earth,
            air,
            ConstantWind::new(draw.wind_speed_m_s, draw.wind_from_rad)?,
        ),
        Model::Other { shear_exponent, .. } => Environment::new(
            earth,
            air,
            PowerLawWind::new(
                draw.wind_speed_m_s,
                SHEAR_REFERENCE_HEIGHT_M,
                shear_exponent,
                draw.wind_from_rad,
                draw.elevation_m,
            )?,
        ),
    })
}

/// The draw's rail.
fn rail(draw: &Draw) -> Rail {
    Rail {
        elevation_rad: FRAC_PI_2 - draw.tilt_rad,
        azimuth_rad: draw.azimuth_rad,
        ..Rail::vertical(RAIL_LENGTH_M)
    }
}

/// Keeps a flight's states every [`TRUTH_STEP_S`] and at each event.
#[derive(Default)]
struct Grid {
    next: u64,
    samples: Vec<Sample>,
    /// Each separated body's samples, by body, at the start of each of its steps.
    bodies: BTreeMap<usize, Vec<BodySample>>,
}

impl Observer for Grid {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let first = (step.start_s() / TRUTH_STEP_S).ceil();
        if (self.next as f64) < first {
            self.next = first as u64;
        }
        loop {
            let t = self.next as f64 * TRUTH_STEP_S;
            if t > step.end_s() {
                break;
            }
            self.samples.push(step.sample(t)?);
            self.next += 1;
        }
        Ok(())
    }

    fn event(&mut self, event: &FlightEvent) {
        self.samples.push(event.sample);
    }

    fn body_step(&mut self, body: usize, sample: &BodySample) {
        self.bodies.entry(body).or_default().push(*sample);
    }
}

/// One sample's truth.
fn truth_sample(environment: &Environment, sample: &Sample) -> Result<TruthSample, SimError> {
    let a = sample.acceleration_enu_m_s2;
    let gravity = environment
        .earth
        .gravity_enu_mps2(sample.state.position_enu_m)?;
    let specific_force = a - gravity;
    let axis = match sample.phase {
        Phase::Descent => DVec3::Z,
        _ => sample.state.unit_attitude() * DVec3::Z,
    };
    Ok(TruthSample {
        time_s: sample.time_s,
        height_m: sample.height_above_ground_m,
        vertical_speed_m_s: sample.vertical_speed_m_s,
        axial_specific_force_m_s2: specific_force.dot(axis),
        mach: sample.mach,
        dynamic_pressure_pa: sample.dynamic_pressure_pa,
        position_enu_m: sample.cg_enu_m.to_array(),
        velocity_enu_m_s: sample.cg_velocity_enu_m_s.to_array(),
    })
}

/// Flies `simulation`, built from `inputs`, and keeps its truth.
///
/// # Errors
///
/// [`SyntheticError::Sim`] if HPR Sim refuses the flight, and [`SyntheticError::Flight`] if it
/// doesn't come down with every event a truth needs.
pub fn fly(
    draw: &Draw,
    inputs: &FlightInputs,
    simulation: &Simulation,
) -> Result<Truth, SyntheticError> {
    let mut grid = Grid::default();
    let result = simulation.run(&mut grid)?;
    let mut samples = grid.samples;
    samples.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));
    samples.dedup_by(|b, a| a.time_s == b.time_s);
    let environment = &inputs.environment;
    let samples = samples
        .iter()
        .map(|s| truth_sample(environment, s))
        .collect::<Result<Vec<_>, _>>()?;
    let names: Vec<&str> = design_of(draw)?.chutes.iter().map(|c| c.name).collect();
    assemble(draw, &result, samples, &names)
}

/// A flight's truth from its samples and its result, `names` naming its parachutes in the order
/// flown.
fn assemble(
    draw: &Draw,
    result: &hpr_sim::FlightResult,
    samples: Vec<TruthSample>,
    names: &[&str],
) -> Result<Truth, SyntheticError> {
    let at = |kind: EventKind, what: &'static str| {
        result
            .event(kind)
            .map(|e| e.sample)
            .ok_or(SyntheticError::Flight { what })
    };
    let apogee = at(EventKind::Apogee, "no apogee")?;
    let by_kind = |kind: fn(usize) -> EventKind| -> Vec<(String, f64)> {
        let mut events: Vec<(String, f64)> = names
            .iter()
            .enumerate()
            .filter_map(|(i, name)| {
                result
                    .event(kind(i))
                    .map(|e| ((*name).to_owned(), e.sample.time_s))
            })
            .collect();
        events.sort_by(|a, b| a.1.total_cmp(&b.1));
        events
    };
    let pad_height_m = samples.first().map_or(0.0, |s| s.height_m);
    Ok(Truth {
        pad_height_m,
        samples,
        events: TruthEvents {
            liftoff_s: at(EventKind::Liftoff, "no liftoff")?.time_s,
            rail_exit_s: at(EventKind::RailExit, "no rail exit")?.time_s,
            burnout_s: at(EventKind::Burnout, "no burnout")?.time_s,
            apogee_s: apogee.time_s,
            apogee_m: apogee.height_above_ground_m,
            charges: by_kind(EventKind::Trigger),
            deployments: by_kind(EventKind::Deployment),
            landing_s: at(EventKind::GroundHit, "no landing")?.time_s,
        },
        site_elevation_m: draw.elevation_m,
        temperature_offset_k: draw.temperature_offset_k,
    })
}

/// The two-stage design's configuration.
const STAGED_CONFIGURATION: &str = "j760-i175";
/// Its booster's motor mount.
const BOOSTER_MOUNT: &str = "booster-motor-mount";
/// Its sustainer's motor mount.
const SUSTAINER_MOUNT: &str = "sustainer-motor-mount";
/// How long after the booster burns out the stack comes apart, s.
const SEPARATION_DELAY_S: f64 = 0.5;
/// How long after that the sustainer lights, s.
const SUSTAINER_DELAY_S: f64 = 0.5;
/// The sustainer's canopy's drag area, m²: invented, about a 1 m canopy's.
const STAGE_DRAG_AREA_M2: f64 = 0.6;
/// The canopy's lag after the sustainer's apogee, s.
const STAGE_LAG_S: f64 = 1.0;
/// The devices' names: the sustainer's canopy, then the booster's tumble.
const STAGE_CHUTES: [&str; 2] = ["sustainer", "booster"];

/// A staged flight's truth, one to a stage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StagedTruth {
    /// The stack, then the sustainer.
    pub sustainer: Truth,
    /// The stack, then the booster tumbling alone as a point mass (`docs/physics/staging.md`).
    pub booster: Truth,
    /// When the stack came apart, s since launch.
    pub separation_s: f64,
}

/// Flies `draw` on the two-stage design `json`: the booster burns out, the stack parts half a
/// second later and the sustainer lights half a second after that. The sustainer comes down under
/// a canopy, the booster tumbling.
///
/// # Errors
///
/// As [`fly`], and [`SyntheticError::Flight`] if the stack doesn't part or the booster has no
/// flight to the ground.
pub fn fly_staged(draw: &Draw, json: &str) -> Result<StagedTruth, SyntheticError> {
    let mut rocket: Rocket = serde_json::from_str(json)?;
    let configuration = rocket
        .configurations
        .iter_mut()
        .find(|c| c.id == STAGED_CONFIGURATION)
        .ok_or(SyntheticError::Invalid {
            what: "a two-stage design with no such configuration",
        })?;
    for mounted in &mut configuration.motors {
        mounted.motor = dispersed_motor(&mounted.motor, draw.impulse_scale, draw.burn_time_scale)?;
        if mounted.mount == SUSTAINER_MOUNT {
            mounted.ignition = Ignition::Separation {
                delay_s: SUSTAINER_DELAY_S,
            };
        }
    }
    let mut inputs =
        FlightInputs::new(rocket, STAGED_CONFIGURATION, environment(draw)?, rail(draw));
    inputs.drag_scale = draw.drag_scale;
    let stack = inputs.simulation()?;
    let booster_motor = stack
        .assembly()
        .motors
        .iter()
        .position(|motor| motor.mounted.mount == BOOSTER_MOUNT)
        .ok_or(SyntheticError::Invalid {
            what: "a two-stage design with no booster motor",
        })?;
    // HPR Sim flies a booster dropped under power with its devices' drag alone, so one must act
    // from the split: the booster tumbles, as `.ork` files' boosters do (`docs/physics/staging.md`).
    let tumble = DeviceDrag::tumbling_stages(stack.assembly(), (1, 1))?;
    inputs.recovery = vec![
        Device::new(
            STAGE_CHUTES[0],
            DeviceDrag::DragArea {
                cd_s_m2: STAGE_DRAG_AREA_M2,
            },
            Trigger::Apogee,
        )
        .with_lag_s(STAGE_LAG_S),
        Device::new(STAGE_CHUTES[1], tumble, Trigger::Time { time_s: 0.0 }).on_body(1),
    ];
    inputs.separations = vec![Separation::new(
        Trigger::Burnout {
            motor: booster_motor,
            delay_s: SEPARATION_DELAY_S,
        },
        0,
    )];
    let simulation = inputs.simulation()?;
    let mut grid = Grid::default();
    let result = simulation.run(&mut grid)?;
    let separation_s = result
        .event(EventKind::Separation)
        .map(|event| event.sample.time_s)
        .ok_or(SyntheticError::Flight {
            what: "no separation",
        })?;
    let flown = result.bodies.first().ok_or(SyntheticError::Flight {
        what: "no booster flight",
    })?;
    let mut samples = std::mem::take(&mut grid.samples);
    samples.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));
    samples.dedup_by(|b, a| a.time_s == b.time_s);
    let environment = &inputs.environment;
    let stack_and_sustainer = samples
        .iter()
        .map(|s| truth_sample(environment, s))
        .collect::<Result<Vec<_>, _>>()?;
    let sustainer = assemble(draw, &result, stack_and_sustainer, &STAGE_CHUTES[..1])?;
    let mut steps = grid.bodies.remove(&flown.body).unwrap_or_default();
    steps.push(flown.final_sample);
    let booster = booster_truth(&sustainer, flown, &steps, separation_s)?;
    Ok(StagedTruth {
        sustainer,
        booster,
        separation_s,
    })
}

/// The booster's truth: the stack's samples before the separation, then its own steps.
fn booster_truth(
    stack: &Truth,
    flown: &BodyFlight,
    steps: &[BodySample],
    separation_s: f64,
) -> Result<Truth, SyntheticError> {
    let air = stack.air()?;
    let mut samples: Vec<TruthSample> = stack
        .samples
        .iter()
        .take_while(|s| s.time_s < separation_s)
        .copied()
        .collect();
    for (k, step) in steps.iter().enumerate() {
        let before = &steps[k.saturating_sub(1)];
        let after = &steps[(k + 1).min(steps.len() - 1)];
        let span = after.time_s - before.time_s;
        // The rate of its vertical speed less gravity's: what an accelerometer hanging along the
        // vertical reads under a canopy. Across the step, so that an instant's step is not a spike.
        let specific_force = if span > 0.0 {
            (after.vertical_speed_m_s - before.vertical_speed_m_s) / span + STANDARD_GRAVITY_M_S2
        } else {
            STANDARD_GRAVITY_M_S2
        };
        let state = air
            .sample(stack.site_elevation_m + step.height_above_ground_m)?
            .air;
        samples.push(TruthSample {
            time_s: step.time_s,
            height_m: step.height_above_ground_m,
            vertical_speed_m_s: step.vertical_speed_m_s,
            axial_specific_force_m_s2: specific_force,
            mach: step.airspeed_m_s / state.speed_of_sound_m_s,
            dynamic_pressure_pa: 0.5 * state.density_kg_m3 * step.airspeed_m_s.powi(2),
            position_enu_m: step.cg_enu_m.to_array(),
            velocity_enu_m_s: step.cg_velocity_enu_m_s.to_array(),
        });
    }
    samples.dedup_by(|b, a| a.time_s == b.time_s);
    let at = |kind: EventKind, what: &'static str| {
        flown
            .event(kind)
            .map(|event| event.sample)
            .ok_or(SyntheticError::Flight { what })
    };
    let apogee = at(EventKind::Apogee, "no booster apogee")?;
    let landing = at(EventKind::GroundHit, "no booster landing")?;
    let named = |kind: EventKind| -> Vec<(String, f64)> {
        flown
            .event(kind)
            .map(|event| vec![(STAGE_CHUTES[1].to_owned(), event.sample.time_s)])
            .unwrap_or_default()
    };
    Ok(Truth {
        samples,
        events: TruthEvents {
            apogee_s: apogee.time_s,
            apogee_m: apogee.height_above_ground_m,
            charges: named(EventKind::Trigger(1)),
            deployments: named(EventKind::Deployment(1)),
            landing_s: landing.time_s,
            ..stack.events.clone()
        },
        ..stack.clone()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_truth_with_no_samples_or_a_time_that_isnt_a_number_has_no_sample_there() {
        let sample = |time_s: f64, height_m: f64| TruthSample {
            time_s,
            height_m,
            vertical_speed_m_s: 0.0,
            axial_specific_force_m_s2: 0.0,
            mach: 0.0,
            dynamic_pressure_pa: 0.0,
            position_enu_m: [0.0; 3],
            velocity_enu_m_s: [0.0; 3],
        };
        let mut truth = Truth {
            samples: vec![sample(0.0, 0.0), sample(1.0, 10.0)],
            events: TruthEvents {
                liftoff_s: 0.0,
                rail_exit_s: 0.0,
                burnout_s: 0.0,
                apogee_s: 1.0,
                apogee_m: 10.0,
                charges: Vec::new(),
                deployments: Vec::new(),
                landing_s: 1.0,
            },
            pad_height_m: 0.0,
            site_elevation_m: 0.0,
            temperature_offset_k: 0.0,
        };
        assert_eq!(truth.at(0.25).unwrap().height_m, 2.5);
        // Before the first sample the first holds; after the last, the last.
        assert_eq!(truth.at(-1.0).unwrap().height_m, 0.0);
        assert_eq!(truth.at(f64::INFINITY).unwrap().height_m, 10.0);
        assert!(truth.at(f64::NAN).is_none());
        truth.samples.clear();
        assert!(truth.at(0.5).is_none());
        assert!(matches!(
            truth.pressure_pa(0.5),
            Err(SyntheticError::Invalid { .. })
        ));
    }

    #[test]
    fn a_tilted_curve_keeps_its_impulse_and_leans_the_way_asked() {
        let curve =
            ThrustCurve::new(vec![0.0, 0.1, 1.0, 2.0], vec![0.0, 1_000.0, 1_000.0, 0.0]).unwrap();
        for tilt in [-0.3, 0.0, 0.3] {
            let t = tilted(&curve, tilt).unwrap();
            let relative =
                (t.total_impulse_ns() - curve.total_impulse_ns()).abs() / curve.total_impulse_ns();
            assert!(relative < 1e-12, "tilt {tilt}: {relative}");
        }
        let progressive = tilted(&curve, 0.3).unwrap();
        // Late thrust rises against early thrust: at 1.0 s over 0.1 s, the ratio 1.0/0.73.
        let ratio = progressive.thrusts_n()[2] / progressive.thrusts_n()[1];
        assert!((ratio - (1.0 + 0.3 * 0.0) / (1.0 + 0.3 * (0.1 - 1.0))).abs() < 1e-12);
        assert!(ratio > 1.0);
        let regressive = tilted(&curve, -0.3).unwrap();
        assert!(regressive.thrusts_n()[2] < regressive.thrusts_n()[1]);
    }

    #[test]
    fn set_bs_three_reshapes_reach_its_flights_inputs_and_set_as_keep_hpr_sims() {
        use crate::synthetic::{Set, draw};
        let files = DesignFiles::committed();
        let a = draw(Set::A, 0);
        let b = draw(Set::B, 0);
        let (ia, ib) = (inputs(&a, &files).unwrap(), inputs(&b, &files).unwrap());
        // The drag: set B overrides HPR Sim's with its reshaped model, set A doesn't.
        assert!(ia.drag.is_none());
        assert!(matches!(ib.drag, Some(DragOverride::Model(_))));
        // The wind: set A's is the same at every height, set B's grows with it.
        let speed = |i: &FlightInputs, d: &Draw, above_m: f64| {
            i.environment
                .wind
                .wind(d.elevation_m + above_m)
                .unwrap()
                .velocity_enu_m_s
                .length()
        };
        assert!((speed(&ia, &a, 10.0) - speed(&ia, &a, 500.0)).abs() < 1e-12);
        assert!(speed(&ib, &b, 500.0) > 1.2 * speed(&ib, &b, 10.0));
        // The thrust: set B's curve is tilted from the scaled motor's, set A's is the scaled one.
        let curve = |i: &FlightInputs| -> Vec<f64> {
            let configuration = i
                .rocket
                .configurations
                .iter()
                .find(|c| c.id == "example")
                .unwrap();
            configuration.motors[0].motor.curve().thrusts_n().to_vec()
        };
        let Model::Other { thrust_tilt, .. } = b.model else {
            panic!("set B flies its model")
        };
        let mut scaled = rocket(
            &Draw {
                model: Model::Same,
                ..b
            },
            &files,
        )
        .unwrap();
        let configuration = scaled
            .configurations
            .iter_mut()
            .find(|c| c.id == "example")
            .unwrap();
        let plain = configuration.motors[0].motor.curve().clone();
        let expect = tilted(&plain, thrust_tilt).unwrap();
        assert_eq!(curve(&ib), expect.thrusts_n());
        assert_ne!(curve(&ib), plain.thrusts_n());
    }

    #[test]
    fn set_bs_drag_factor_reshapes_around_mach_one() {
        let flat = ReshapedDrag {
            scale: 1.1,
            bump: 0.0,
            slope: 0.0,
        };
        assert_eq!(flat.factor(0.3), 1.1);
        assert_eq!(flat.factor(2.0), 1.1);
        let bumped = ReshapedDrag {
            scale: 1.0,
            bump: 0.2,
            slope: 0.0,
        };
        assert!((bumped.factor(1.0) - 1.2).abs() < 1e-15);
        assert!((bumped.factor(1.2) - (1.0 + 0.2 * (-1.0_f64).exp())).abs() < 1e-15);
        assert!((bumped.factor(0.3) - 1.0).abs() < 1e-4);
    }
}
