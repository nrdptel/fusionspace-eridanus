//! A free rigid body of fixed mass and inertia, with no design, motor or aerodynamics: the
//! flight's gravity, Earth rotation, rigid-body equations and integrator on a body a test gives.
//!
//! It exists to fly published check cases whose bodies are not rockets, such as NASA's dragless
//! sphere and tumbling brick (NASA/TM-2015-218675, cases 1 to 3 and 6 to 10), through the code a
//! rocket flies on. Nothing here is a second model: the weight and the Coriolis force come from
//! the [`Environment`]'s Earth, as [`crate::dynamics`] takes them, the air and the wind from the
//! environment at the height a rocket's flight reads them at, the two accelerations from that
//! module's free-flight equations, and the steps from [`crate::integrator`]. A test holds the
//! derivative to the one a coasting rocket's flight evaluates at the same state.
//!
//! The caller may add loads of its own ([`FreeBodyLoads`]): a force `F` through the center of
//! mass and a moment `M` about it, in body axes, formed from the state, the air there and the
//! velocity through it ([`LoadInputs`]). A check case's constant drag or damping coefficient is
//! such a load; the body has no aerodynamics of its own.
//!
//! With the reference point at the center of mass, a constant mass `m` and a constant inertia `I`
//! about it, and no load but gravity `g`, the Coriolis force and the caller's, the equations of
//! [`crate::dynamics`] reduce to Newton's and Euler's:
//!
//! ```text
//! a  = g(x) − 2 Ω × v + q F / m
//! ω̇ = I⁻¹ (M − ω × I ω)
//! q̇ = ½ q ⊗ ω
//! ```
//!
//! `x` and `v` are the center of mass's position and velocity in the launch frame `L`, `Ω` the
//! Earth's rate, `q` the attitude (body to `L`) and `ω` the angular velocity relative to `L` in
//! body axes. As in a rocket's flight, the Earth's rate enters the translation only: the
//! rotational equations treat `L` as inertial (the decision record on rigid-body flight,
//! [ADR-011][adr-011]). Source: the equations of motion on the page
//! [Rigid-body flight][guide-rigid-body], which cites RocketPy's derivation.
//!
//! [adr-011]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0011-rigid-body-flight-equations-of-motion.md
//! [guide-rigid-body]: https://hpr.fusionspace.co/physics/flight.html

use std::ops::ControlFlow;

use hpr_atmos::AirState;
use hpr_core::attitude::quaternion_derivative;
use hpr_core::{DMat3, DVec3};
use serde::{Deserialize, Serialize};

use crate::dynamics::{MassState, free_motion};
use crate::environment::Environment;
use crate::error::SimError;
use crate::integrator::{IntegrationError, Integrator, Method, OdeSystem, Step};
use crate::state::{STATE_LEN, State};

/// [`SimError::Domain`]'s `what` for a mass that is not finite and positive.
pub const FREE_BODY_MASS_REFUSED: &str = "free body's mass (kg), which must be finite and positive";

/// [`SimError::Domain`]'s `what` for an inertia tensor with an entry that is not finite; the
/// value is the entry.
pub const FREE_BODY_INERTIA_NOT_FINITE: &str = "free body's inertia entry (kg·m²)";

/// [`SimError::Domain`]'s `what` for an inertia tensor that is not symmetric; the value is the
/// largest difference between an entry and its mirror, kg·m².
pub const FREE_BODY_INERTIA_NOT_SYMMETRIC: &str =
    "free body's inertia asymmetry (kg·m²), which must be zero";

/// [`SimError::Domain`]'s `what` for an inertia tensor that is not positive definite; the value
/// is the leading principal minor that is not positive.
pub const FREE_BODY_INERTIA_NOT_POSITIVE: &str =
    "free body's inertia leading minor, which must be positive";

/// [`SimError::Domain`]'s `what` for a sample time that is not finite, comes before the start or
/// comes before the sample ahead of it; the value is the time, s.
pub const FREE_BODY_TIME_REFUSED: &str =
    "free body's sample time (s), which must be finite, not before the start and not descending";

/// [`SimError::Domain`]'s `what` for a caller's load with a component that is not finite; the
/// value is the component.
pub const FREE_BODY_LOAD_NOT_FINITE: &str =
    "free body's load (N in the force, N·m in the moment), which must be finite";

/// What a free body's loads are formed from at one state, read as a rocket's flight reads them:
/// the air and the wind at the center of mass's height above mean sea level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LoadInputs {
    /// The state, its attitude normalized.
    pub state: State,
    /// The height above mean sea level, m: the ellipsoidal height less the environment's geoid
    /// undulation.
    pub height_msl_m: f64,
    /// The air there.
    pub air: AirState,
    /// The wind there, in the launch frame's East-North-Up axes, m/s.
    pub wind_enu_m_s: DVec3,
    /// The velocity through the air in body axes, m/s: the velocity less the wind, turned into
    /// the body.
    pub air_velocity_body_m_s: DVec3,
}

/// A force through the center of mass and a moment about it, in body axes.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Loads {
    /// The force, N.
    pub force_body_n: DVec3,
    /// The moment about the center of mass, N·m.
    pub moment_body_n_m: DVec3,
}

/// The loads a caller puts on a free body, formed at each state the integrator evaluates. Any
/// `Fn(&LoadInputs) -> Result<Loads, SimError>` is one; [`NoLoads`] is none.
pub trait FreeBodyLoads {
    /// The loads at `inputs`.
    ///
    /// # Errors
    ///
    /// Whatever the caller's loads refuse; the flight stops with it.
    fn loads(&self, inputs: &LoadInputs) -> Result<Loads, SimError>;
}

impl<F: Fn(&LoadInputs) -> Result<Loads, SimError>> FreeBodyLoads for F {
    fn loads(&self, inputs: &LoadInputs) -> Result<Loads, SimError> {
        self(inputs)
    }
}

/// No load but gravity and the Coriolis force.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NoLoads;

impl FreeBodyLoads for NoLoads {
    fn loads(&self, _inputs: &LoadInputs) -> Result<Loads, SimError> {
        Ok(Loads::default())
    }
}

/// A rigid body of fixed mass and inertia.
///
/// Its [`State`] is the center of mass's, where a rocket's is its nose tip's: the position and
/// velocity are the center of mass's in the launch frame, the attitude takes the body's axes to
/// the launch frame's, and the body rates are relative to the launch frame, in body axes. The
/// body's axes are whatever the inertia is given in; no axis is a nose.
///
/// It serializes as its `mass_kg` and `inertia_kg_m2` and is checked again when read, so a body
/// that [`FreeBody::new`] would refuse can't be read from a file either.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FreeBodyData", into = "FreeBodyData")]
pub struct FreeBody {
    mass_kg: f64,
    inertia_kg_m2: DMat3,
}

/// [`FreeBody`] as it is written: the two values [`FreeBody::new`] takes.
#[derive(Serialize, Deserialize)]
struct FreeBodyData {
    mass_kg: f64,
    inertia_kg_m2: DMat3,
}

impl TryFrom<FreeBodyData> for FreeBody {
    type Error = SimError;

    fn try_from(data: FreeBodyData) -> Result<Self, SimError> {
        Self::new(data.mass_kg, data.inertia_kg_m2)
    }
}

impl From<FreeBody> for FreeBodyData {
    fn from(body: FreeBody) -> Self {
        Self {
            mass_kg: body.mass_kg,
            inertia_kg_m2: body.inertia_kg_m2,
        }
    }
}

impl FreeBody {
    /// A body of mass `mass_kg` and inertia tensor `inertia_kg_m2` about its center of mass, in
    /// its own axes, kg·m².
    ///
    /// The tensor must be symmetric exactly, to the last bit. One computed by parts, such as a
    /// rocket's inertia moved to its center of mass, is often off by rounding; the caller
    /// averages it with its transpose first, `(I + Iᵀ)/2`, so that what is flown is what the
    /// caller chose, not a repair made here.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] naming what was refused: a mass that is not finite and positive
    /// ([`FREE_BODY_MASS_REFUSED`]), or an inertia with an entry that is not finite
    /// ([`FREE_BODY_INERTIA_NOT_FINITE`]), that is not symmetric
    /// ([`FREE_BODY_INERTIA_NOT_SYMMETRIC`]) or that is not positive definite
    /// ([`FREE_BODY_INERTIA_NOT_POSITIVE`]).
    pub fn new(mass_kg: f64, inertia_kg_m2: DMat3) -> Result<Self, SimError> {
        if !(mass_kg.is_finite() && mass_kg > 0.0) {
            return Err(SimError::Domain {
                what: FREE_BODY_MASS_REFUSED,
                value: mass_kg,
            });
        }
        let i = inertia_kg_m2;
        if let Some(&value) = i.to_cols_array().iter().find(|entry| !entry.is_finite()) {
            return Err(SimError::Domain {
                what: FREE_BODY_INERTIA_NOT_FINITE,
                value,
            });
        }
        let asymmetry = (i.x_axis.y - i.y_axis.x)
            .abs()
            .max((i.x_axis.z - i.z_axis.x).abs())
            .max((i.y_axis.z - i.z_axis.y).abs());
        if asymmetry != 0.0 {
            return Err(SimError::Domain {
                what: FREE_BODY_INERTIA_NOT_SYMMETRIC,
                value: asymmetry,
            });
        }
        // Sylvester's criterion: a symmetric matrix is positive definite exactly when its three
        // leading principal minors are positive.
        let minors = [
            i.x_axis.x,
            i.x_axis.x * i.y_axis.y - i.x_axis.y * i.y_axis.x,
            i.determinant(),
        ];
        if let Some(&value) = minors.iter().find(|minor| **minor <= 0.0) {
            return Err(SimError::Domain {
                what: FREE_BODY_INERTIA_NOT_POSITIVE,
                value,
            });
        }
        Ok(Self {
            mass_kg,
            inertia_kg_m2,
        })
    }

    /// The mass, kg.
    #[must_use]
    pub fn mass_kg(&self) -> f64 {
        self.mass_kg
    }

    /// The inertia tensor about the center of mass, in body axes, kg·m².
    #[must_use]
    pub fn inertia_kg_m2(&self) -> DMat3 {
        self.inertia_kg_m2
    }

    /// The mass properties as the flight's equations take them: the center of mass at the
    /// reference point, and nothing changing.
    fn mass_state(&self) -> MassState {
        MassState {
            mass_kg: self.mass_kg,
            mass_rate_kg_s: 0.0,
            cg_m: DVec3::ZERO,
            cg_rate_m_s: DVec3::ZERO,
            cg_accel_m_s2: DVec3::ZERO,
            inertia_cg: self.inertia_kg_m2,
            inertia_o: self.inertia_kg_m2,
            inertia_o_rate: DMat3::ZERO,
            relative_momentum: DVec3::ZERO,
            relative_momentum_rate: DVec3::ZERO,
        }
    }

    /// The state's time derivative in `environment` under the caller's `loads`, in
    /// [`State::to_array`]'s order: the weight and the Coriolis force at the center of mass, as a
    /// rocket's flight takes them, and the caller's force and moment, through the flight's
    /// free-motion equations.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] for an attitude quaternion whose norm is zero or not finite, or with
    /// [`FREE_BODY_LOAD_NOT_FINITE`] for a load that is not finite; [`SimError::Core`] if the
    /// Earth model refuses the position; whatever the environment refuses of the air or the wind
    /// there; and whatever `loads` refuses.
    pub fn derivative(
        &self,
        environment: &Environment,
        loads: &dyn FreeBodyLoads,
        y: &[f64; STATE_LEN],
    ) -> Result<[f64; STATE_LEN], SimError> {
        let state = State::from_array(y);
        let norm = state.attitude.length();
        if !(norm.is_finite() && norm > 0.0) {
            return Err(SimError::Domain {
                what: "attitude quaternion norm",
                value: norm,
            });
        }
        let q = state.attitude / norm;
        let omega = state.body_rate_rad_s;
        // The air and the wind where the center of mass is, at the height a rocket's flight
        // reads them at (`crate::dynamics`).
        let geodetic = environment
            .earth
            .frame()
            .geodetic_from_enu(state.position_enu_m)?;
        let height_msl_m = geodetic.height_m - environment.geoid_undulation_m;
        let air = environment.air_at(height_msl_m)?;
        let wind_enu_m_s = environment.wind_enu_m_s(height_msl_m)?;
        let air_velocity_body_m_s = q
            .conjugate()
            .mul_vec3(state.velocity_enu_m_s - wind_enu_m_s);
        let applied = loads.loads(&LoadInputs {
            state: State {
                attitude: q,
                ..state
            },
            height_msl_m,
            air,
            wind_enu_m_s,
            air_velocity_body_m_s,
        })?;
        if let Some(&value) = applied
            .force_body_n
            .to_array()
            .iter()
            .chain(&applied.moment_body_n_m.to_array())
            .find(|component| !component.is_finite())
        {
            return Err(SimError::Domain {
                what: FREE_BODY_LOAD_NOT_FINITE,
                value,
            });
        }
        let gravity_enu = environment.earth.gravity_enu_mps2(state.position_enu_m)?;
        let coriolis_enu = environment
            .earth
            .rotation_acceleration_enu_mps2(state.velocity_enu_m_s);
        let weight = q
            .conjugate()
            .mul_vec3((gravity_enu + coriolis_enu) * self.mass_kg);
        let (a_body, omega_dot) = free_motion(
            &self.mass_state(),
            omega,
            [DVec3::ZERO, DVec3::ZERO, weight + applied.force_body_n],
            DMat3::ZERO,
            [DVec3::ZERO, applied.moment_body_n_m, DVec3::ZERO],
        )?;
        let a = q.mul_vec3(a_body);
        let v = state.velocity_enu_m_s;
        let q_dot = quaternion_derivative(state.attitude, omega);
        Ok([
            v.x,
            v.y,
            v.z,
            a.x,
            a.y,
            a.z,
            q_dot.w,
            q_dot.x,
            q_dot.y,
            q_dot.z,
            omega_dot.x,
            omega_dot.y,
            omega_dot.z,
        ])
    }

    /// Flies the body in `environment` under the caller's `loads` from `state` at `t0_s` and
    /// gives its state at each of `times_s`, read from the steps' dense output as a flight's
    /// recorder reads its samples. The times don't shape the steps, but for the last, where the
    /// integration stops. Nothing ends the flight early: the ground is not looked for.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] with [`FREE_BODY_TIME_REFUSED`] for a sample time that is not finite,
    /// is before `t0_s` or is before the one ahead of it; [`SimError::Settings`] for a bad method
    /// or a start that is not finite; whatever [`Self::derivative`] refuses; and
    /// [`SimError::Integration`] if the integration fails, the step limit included.
    pub fn fly(
        &self,
        environment: &Environment,
        loads: &dyn FreeBodyLoads,
        method: Method,
        t0_s: f64,
        state: State,
        times_s: &[f64],
    ) -> Result<Vec<State>, SimError> {
        // The integrator refuses a start that is not finite; checked first, so a sample time
        // is not blamed for it.
        let mut integrator = Integrator::new(method, t0_s, state.to_array())?;
        let mut last = t0_s;
        for &time in times_s {
            if !(time.is_finite() && time >= last) {
                return Err(SimError::Domain {
                    what: FREE_BODY_TIME_REFUSED,
                    value: time,
                });
            }
            last = time;
        }
        let mut system = Flight {
            body: self,
            environment,
            loads,
            times_s,
            states: Vec::with_capacity(times_s.len()),
        };
        // The samples at the start itself, which no step covers.
        while system.next_time().is_some_and(|time| time <= t0_s) {
            system.states.push(state);
        }
        match integrator.advance(&mut system, last) {
            Ok(_) => {}
            Err(IntegrationError::Derivative { source, .. }) => return Err(source),
            Err(error) => return Err(SimError::Integration(Box::new(error))),
        }
        // A last sample within the rounding of the stop, which the integrator reaches without a
        // step.
        let end = State::from_array(integrator.state());
        while system.next_time().is_some() {
            system.states.push(end);
        }
        Ok(system.states)
    }
}

/// A free body's flight as the integrator sees it, collecting the samples.
struct Flight<'a> {
    body: &'a FreeBody,
    environment: &'a Environment,
    loads: &'a dyn FreeBodyLoads,
    times_s: &'a [f64],
    states: Vec<State>,
}

impl Flight<'_> {
    /// The next sample time not yet given a state.
    fn next_time(&self) -> Option<f64> {
        self.times_s.get(self.states.len()).copied()
    }
}

impl OdeSystem<STATE_LEN> for Flight<'_> {
    type Error = SimError;

    fn derivative(
        &mut self,
        _t_s: f64,
        y: &[f64; STATE_LEN],
    ) -> Result<[f64; STATE_LEN], SimError> {
        self.body.derivative(self.environment, self.loads, y)
    }

    fn accept_step(&mut self, step: &Step<STATE_LEN>) -> ControlFlow<()> {
        while let Some(time) = self.next_time() {
            if time > step.end_s() {
                break;
            }
            self.states.push(State::from_array(&step.state_at(time)));
        }
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod tests {
    use hpr_atmos::ConstantWind;
    use hpr_core::DQuat;
    use hpr_core::earth::Earth;

    use super::*;
    use crate::dynamics::{Conditions, Phase, Vehicle};
    use crate::flight::{FlightSettings, Simulation};
    use crate::rail::Rail;
    use crate::testing::{UniformAir, analytic_environment, design, site};

    /// The tumbling brick of NASA/TM-2015-218675 (Volume II, Table 4), in SI: three unequal
    /// moments about its own axes.
    fn brick() -> FreeBody {
        const SLUG_FT2_KG_M2: f64 = 4.448_221_615_260_5 * 0.3048;
        FreeBody::new(
            0.155_404_754 * 4.448_221_615_260_5 / 0.3048,
            DMat3::from_diagonal(DVec3::new(
                0.001_894_220 * SLUG_FT2_KG_M2,
                0.006_211_019 * SLUG_FT2_KG_M2,
                0.007_194_665 * SLUG_FT2_KG_M2,
            )),
        )
        .unwrap()
    }

    /// WGS 84's gravity at the body's own place and the Coriolis force, with no air.
    fn airless_earth() -> Environment {
        Environment::new(
            Earth::wgs84(site()).unwrap(),
            UniformAir::vacuum(),
            ConstantWind::calm(),
        )
    }

    fn tumbling() -> State {
        State {
            position_enu_m: DVec3::new(40.0, -25.0, 900.0),
            velocity_enu_m_s: DVec3::new(30.0, -20.0, 80.0),
            attitude: DQuat::from_rotation_z(0.3)
                * DQuat::from_rotation_y(0.4)
                * DQuat::from_rotation_x(0.2),
            body_rate_rad_s: DVec3::new(0.4, -0.3, 3.0),
        }
    }

    /// The free body's derivative is the flight's: a coasting rocket in no air, evaluated by the
    /// equations a flight integrates, accelerates and turns as a free body of its mass and
    /// inertia does at the same state. The rocket's state is its nose tip's, so its center of
    /// mass's acceleration is `a_O + q (ω̇ × r + ω × (ω × r))`.
    #[test]
    fn the_derivative_is_a_coasting_rockets() {
        let environment = airless_earth();
        let simulation = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            environment.clone(),
            Rail::vertical(3.0),
            FlightSettings::default(),
        )
        .unwrap();
        // Valetudo's motor burns out at 3.26 s; at 10 s nothing about its mass changes.
        let t = 10.0;
        let vehicle =
            Vehicle::new(simulation.assembly().clone(), simulation.aero().clone()).unwrap();
        let nose = tumbling();
        let evaluation = vehicle
            .evaluate(
                &environment,
                0.0,
                Conditions {
                    phase: Phase::Free,
                    window: (t, t + 1.0),
                    drag_area_m2: 0.0,
                    pivot: None,
                },
                t,
                &nose.to_array(),
            )
            .unwrap();
        let mass = evaluation.mass;
        assert_eq!(mass.mass_rate_kg_s, 0.0);
        assert!(mass.cg_m.length() > 1.0, "{:?}", mass.cg_m);
        let (q, omega, r) = (nose.unit_attitude(), nose.body_rate_rad_s, mass.cg_m);
        let omega_dot = DVec3::from_slice(&evaluation.derivative[10..13]);
        let rocket_cg_acceleration = evaluation.acceleration_enu_m_s2
            + q.mul_vec3(omega_dot.cross(r) + omega.cross(omega.cross(r)));

        let body = FreeBody::new(mass.mass_kg, symmetric(mass.inertia_cg)).unwrap();
        let at_cg = State {
            position_enu_m: evaluation.cg_enu_m,
            velocity_enu_m_s: evaluation.cg_velocity_enu_m_s,
            ..nose
        };
        let derivative = body
            .derivative(&environment, &NoLoads, &at_cg.to_array())
            .unwrap();
        let acceleration = DVec3::from_slice(&derivative[3..6]);
        let turning = DVec3::from_slice(&derivative[10..13]);

        // The two differ by rounding only, measured 3.7e-15 m/s² and 2.4e-14 rad/s²: the rocket's
        // side adds and takes away centripetal terms of about 20 m/s², for a nose tip 2 m from
        // the center of mass turning at 3 rad/s. The bound leaves forty times that.
        let acceleration_gap = (acceleration - rocket_cg_acceleration).length();
        assert!(acceleration_gap < 1e-12, "{acceleration_gap:e} m/s²");
        let turning_gap = (turning - omega_dot).length();
        assert!(turning_gap < 1e-12, "{turning_gap:e} rad/s²");
        // The comparison has something to hold: gravity, the Coriolis force and the gyroscopic
        // term are each far above the bound.
        assert!(acceleration.length() > 9.7);
        assert!(omega_dot.length() > 1e-3, "{omega_dot:?}");
        let coriolis = environment
            .earth
            .rotation_acceleration_enu_mps2(at_cg.velocity_enu_m_s);
        assert!(coriolis.length() > 1e-3, "{coriolis:?}");
    }

    /// A tensor with its rounding asymmetry averaged out, as [`FreeBody::new`] asks.
    fn symmetric(i: DMat3) -> DMat3 {
        (i + i.transpose()) * 0.5
    }

    /// With no torque, a body of three unequal moments keeps its angular momentum, seen from the
    /// launch frame, and its rotational energy: Euler's equations' two integrals. The brick
    /// tumbles for 30 s on the default method.
    #[test]
    fn a_tumbling_brick_keeps_its_angular_momentum_and_energy() {
        let body = brick();
        let inertia = body.inertia_kg_m2();
        let start = State {
            position_enu_m: DVec3::new(0.0, 0.0, 9144.0),
            velocity_enu_m_s: DVec3::ZERO,
            attitude: DQuat::IDENTITY,
            body_rate_rad_s: DVec3::new(10.0, 20.0, 30.0).map(f64::to_radians),
        };
        let times: Vec<f64> = (0..=300).map(|i| 0.1 * f64::from(i)).collect();
        let states = body
            .fly(
                &airless_earth(),
                &NoLoads,
                Method::default(),
                0.0,
                start,
                &times,
            )
            .unwrap();
        assert_eq!(states.len(), times.len());
        assert_eq!(states[0], start);
        let momentum = |s: &State| s.unit_attitude().mul_vec3(inertia * s.body_rate_rad_s);
        let energy = |s: &State| 0.5 * s.body_rate_rad_s.dot(inertia * s.body_rate_rad_s);
        let (h0, e0) = (momentum(&start), energy(&start));
        let mut worst_momentum: f64 = 0.0;
        let mut worst_energy: f64 = 0.0;
        let mut largest_turn: f64 = 0.0;
        for state in &states {
            worst_momentum = worst_momentum.max((momentum(state) - h0).length() / h0.length());
            worst_energy = worst_energy.max((energy(state) - e0).abs() / e0);
            largest_turn =
                largest_turn.max((state.body_rate_rad_s - start.body_rate_rad_s).length());
        }
        // Measured 3.4e-8 and 2.7e-8 at the default tolerances (1e-8 a step); about ten times
        // that is allowed.
        assert!(worst_momentum < 4e-7, "{worst_momentum:e}");
        assert!(worst_energy < 4e-7, "{worst_energy:e}");
        // The rates are far from constant (they move by up to 0.79 rad/s), so the integrals are
        // not held trivially.
        assert!(largest_turn > 0.5, "{largest_turn} rad/s");
    }

    /// Under uniform gravity with the Earth's rotation off, the center of mass follows the
    /// parabola whatever the body does.
    #[test]
    fn the_center_of_mass_follows_the_parabola() {
        const G: f64 = 9.806_65;
        let body = brick();
        let start = tumbling();
        let times = [0.0, 0.5, 3.0, 7.25, 12.0];
        let states = body
            .fly(
                &analytic_environment(UniformAir::vacuum(), G),
                &NoLoads,
                Method::default(),
                0.0,
                start,
                &times,
            )
            .unwrap();
        for (time, state) in times.iter().zip(&states) {
            let expected = start.position_enu_m + start.velocity_enu_m_s * *time
                - DVec3::Z * (0.5 * G * time * time);
            let gap = (state.position_enu_m - expected).length();
            assert!(gap < 1e-8, "{gap:e} m at {time} s");
        }
    }

    /// A body dropped at rest over the equator is carried east by the Coriolis force, at
    /// `Ω g t²` after `t` seconds, and that eastward speed is in turn pushed up, at `2 Ω² g t²`:
    /// with the Earth's rotation on, the fall ends `Ω² g t⁴/6` higher than with it off, 7.0 mm
    /// after 30 s. The lift is even in the sign of `Ω`: a Coriolis force of the wrong sign
    /// would drift the body west and lift it the same, so a fall's height holds the term's
    /// size and not its sign, which the eastward drift holds.
    #[test]
    fn the_coriolis_force_lifts_a_long_fall() {
        use hpr_core::earth::{EarthRotation, GravityModel};
        use hpr_core::geodesy::Geodetic;
        use hpr_core::gravity::NormalGravity;
        let equator = Geodetic::from_degrees(0.0, 0.0, 0.0).unwrap();
        let environment = |rotation| {
            Environment::new(
                Earth::new(
                    NormalGravity::wgs84(),
                    equator,
                    GravityModel::default(),
                    rotation,
                )
                .unwrap(),
                UniformAir::vacuum(),
                ConstantWind::calm(),
            )
        };
        let start = State {
            position_enu_m: DVec3::new(0.0, 0.0, 9144.0),
            velocity_enu_m_s: DVec3::ZERO,
            attitude: DQuat::IDENTITY,
            body_rate_rad_s: DVec3::ZERO,
        };
        let end = |rotation| {
            let environment = environment(rotation);
            let states = brick()
                .fly(
                    &environment,
                    &NoLoads,
                    Method::default(),
                    0.0,
                    start,
                    &[30.0],
                )
                .unwrap();
            let height = environment
                .earth
                .frame()
                .geodetic_from_enu(states[0].position_enu_m)
                .unwrap()
                .height_m;
            (height, states[0].velocity_enu_m_s.x)
        };
        let (turning, east_m_s) = end(EarthRotation::Coriolis);
        let (still, no_drift_m_s) = end(EarthRotation::Ignore);
        let omega: f64 = 7.292_115e-5;
        // Gravity over the fall, from the fall itself, 4.4 km in 30 s: the equator's 9.780 m/s²
        // less 3.086e-6 m/s² a meter over heights of 9.1 to 4.8 km.
        let g = 2.0 * (9144.0 - still) / 30.0_f64.powi(2);
        assert!((g - 9.754).abs() < 0.005, "{g}");
        let lift_m = omega * omega * g * 30.0_f64.powi(4) / 6.0;
        assert!((lift_m - 0.0070).abs() < 0.0001, "{lift_m}");
        let lifted_m = turning - still;
        assert!(
            (lifted_m / lift_m - 1.0).abs() < 0.01,
            "{lifted_m} {lift_m}"
        );
        // East, at Ω g t²: 0.64 m/s. With the rotation off there is no drift at all.
        let drift_m_s = omega * g * 30.0_f64.powi(2);
        assert!(
            (east_m_s / drift_m_s - 1.0).abs() < 0.01,
            "{east_m_s} {drift_m_s}"
        );
        assert_eq!(no_drift_m_s, 0.0);
    }

    /// The samples are the integration's own states, whatever times are asked for: a flight
    /// sampled at two times gives the states a flight sampled at many gives there, to the dense
    /// output's accuracy, and a repeated time or the start itself gives its state each time.
    #[test]
    fn samples_do_not_shape_the_flight() {
        let body = brick();
        let start = tumbling();
        let environment = airless_earth();
        let fly = |times: &[f64]| {
            body.fly(&environment, &NoLoads, Method::default(), 2.0, start, times)
                .unwrap()
        };
        let dense: Vec<f64> = (0..=100).map(|i| 2.0 + 0.05 * f64::from(i)).collect();
        let many = fly(&dense);
        let few = fly(&[2.0, 2.0, 4.5, 7.0]);
        assert_eq!(few[0], start);
        assert_eq!(few[1], start);
        assert_eq!(few[3], many[100]);
        let gap = (few[2].position_enu_m - many[50].position_enu_m).length();
        assert!(gap < 1e-9, "{gap:e} m");
        assert!(fly(&[]).is_empty());
    }

    /// Each refusal names what it refused.
    #[test]
    fn bad_bodies_and_times_are_refused_by_name() {
        let good = DMat3::from_diagonal(DVec3::new(1.0, 2.0, 3.0));
        let what = |result: Result<FreeBody, SimError>| match result {
            Err(SimError::Domain { what, value }) => (what, value),
            other => panic!("{other:?}"),
        };
        assert_eq!(what(FreeBody::new(0.0, good)).0, FREE_BODY_MASS_REFUSED);
        assert_eq!(
            what(FreeBody::new(-1.0, good)),
            (FREE_BODY_MASS_REFUSED, -1.0)
        );
        assert_eq!(
            what(FreeBody::new(f64::INFINITY, good)).0,
            FREE_BODY_MASS_REFUSED
        );
        let mut not_finite = good;
        not_finite.y_axis.z = f64::NAN;
        assert_eq!(
            what(FreeBody::new(1.0, not_finite)).0,
            FREE_BODY_INERTIA_NOT_FINITE
        );
        let mut lopsided = good;
        lopsided.x_axis.y = 0.25;
        assert_eq!(
            what(FreeBody::new(1.0, lopsided)),
            (FREE_BODY_INERTIA_NOT_SYMMETRIC, 0.25)
        );
        // Symmetric, with a product of inertia too large for its moments: the second minor is
        // 1·2 − 2² = −2.
        let mut indefinite = good;
        indefinite.x_axis.y = 2.0;
        indefinite.y_axis.x = 2.0;
        assert_eq!(
            what(FreeBody::new(1.0, indefinite)),
            (FREE_BODY_INERTIA_NOT_POSITIVE, -2.0)
        );
        assert_eq!(
            what(FreeBody::new(1.0, DMat3::ZERO)),
            (FREE_BODY_INERTIA_NOT_POSITIVE, 0.0)
        );

        let body = FreeBody::new(1.0, good).unwrap();
        let environment = airless_earth();
        let refused = |t0: f64, times: &[f64]| match body.fly(
            &environment,
            &NoLoads,
            Method::default(),
            t0,
            tumbling(),
            times,
        ) {
            Err(SimError::Domain { what, value }) => (what, value),
            other => panic!("{other:?}"),
        };
        assert_eq!(refused(1.0, &[0.5]), (FREE_BODY_TIME_REFUSED, 0.5));
        assert_eq!(refused(0.0, &[1.0, 0.75]), (FREE_BODY_TIME_REFUSED, 0.75));
        assert_eq!(refused(0.0, &[f64::INFINITY]).0, FREE_BODY_TIME_REFUSED);
        assert!(refused(0.0, &[f64::NAN]).1.is_nan());
        // A start that is not finite is the integrator's to refuse, not a sample time's.
        let start = body.fly(
            &environment,
            &NoLoads,
            Method::default(),
            f64::NAN,
            tumbling(),
            &[1.0],
        );
        assert!(matches!(start, Err(SimError::Settings(_))), "{start:?}");
    }

    /// A body read from JSON goes through [`FreeBody::new`] again: a mass of zero, two negative
    /// moments (whose determinant is positive, so the equations of motion would take them) and
    /// a tensor that is not symmetric are each refused by name, and a good body comes back as
    /// it was written.
    #[test]
    fn a_body_read_from_json_is_checked_again() {
        let read = |json: &str| serde_json::from_str::<FreeBody>(json).map_err(|e| e.to_string());
        let refused = |json: &str, what: &str| {
            let error = read(json).unwrap_err();
            assert!(error.contains(what), "{error}");
        };
        refused(
            r#"{"mass_kg":0.0,"inertia_kg_m2":[1,0,0,0,1,0,0,0,1]}"#,
            FREE_BODY_MASS_REFUSED,
        );
        refused(
            r#"{"mass_kg":-1.0,"inertia_kg_m2":[1,0,0,0,1,0,0,0,1]}"#,
            FREE_BODY_MASS_REFUSED,
        );
        refused(
            r#"{"mass_kg":1.0,"inertia_kg_m2":[-1,0,0,0,-2,0,0,0,3]}"#,
            FREE_BODY_INERTIA_NOT_POSITIVE,
        );
        refused(
            r#"{"mass_kg":1.0,"inertia_kg_m2":[1,0.25,0,0,2,0,0,0,3]}"#,
            FREE_BODY_INERTIA_NOT_SYMMETRIC,
        );
        let body = brick();
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.starts_with(r#"{"mass_kg":"#), "{json}");
        assert_eq!(read(&json).unwrap(), body);
    }

    /// A caller's loads are formed from the air and the wind where a rocket's flight reads them,
    /// and enter as a force through the center of mass and a moment about it: the acceleration
    /// gains `q F / m` and the turning `I⁻¹ M`, each to rounding, at a state that tumbles in a
    /// wind.
    #[test]
    fn loads_see_the_air_as_a_flight_does_and_enter_newtons_and_eulers_equations() {
        use std::cell::Cell;

        // The 1976 atmosphere, which changes with height, and a geoid 17 m above the
        // ellipsoid: air read at the ellipsoidal height, or with the undulation's sign turned,
        // is other air.
        let undulation_m = 17.0;
        let wind = DVec3::new(-6.0, 4.0, 0.0);
        let environment = Environment::standard(site())
            .unwrap()
            .with_wind(
                ConstantWind::new(wind.length(), wind.x.atan2(wind.y) + std::f64::consts::PI)
                    .unwrap(),
            )
            .with_geoid_undulation_m(undulation_m);
        let body = brick();
        let state = tumbling();
        let force = DVec3::new(0.7, -1.1, 2.3);
        let moment = DVec3::new(0.002, 0.003, -0.001);
        let seen = Cell::new(None);
        let loads = |inputs: &LoadInputs| {
            seen.set(Some(*inputs));
            Ok(Loads {
                force_body_n: force,
                moment_body_n_m: moment,
            })
        };
        let y = state.to_array();
        let loaded = body.derivative(&environment, &loads, &y).unwrap();
        let free = body.derivative(&environment, &NoLoads, &y).unwrap();

        let inputs = seen.get().unwrap();
        let q = state.unit_attitude();
        let height = environment
            .earth
            .frame()
            .geodetic_from_enu(state.position_enu_m)
            .unwrap()
            .height_m;
        assert_eq!(inputs.height_msl_m, height - undulation_m);
        let density = |at_m: f64| environment.atmosphere.air(at_m).unwrap().air.density_kg_m3;
        assert_eq!(inputs.air.density_kg_m3, density(height - undulation_m));
        for wrong in [height, height + undulation_m] {
            assert!((density(wrong) / density(height - undulation_m) - 1.0).abs() > 1e-4);
        }
        assert!(
            (inputs.wind_enu_m_s - wind).length() < 1e-12,
            "{:?}",
            inputs.wind_enu_m_s
        );
        let through_air = q.conjugate().mul_vec3(state.velocity_enu_m_s - wind);
        assert!((inputs.air_velocity_body_m_s - through_air).length() < 1e-12);
        assert_eq!(inputs.state.body_rate_rad_s, state.body_rate_rad_s);

        let gained = DVec3::from_slice(&loaded[3..6]) - DVec3::from_slice(&free[3..6]);
        let expected = q.mul_vec3(force) / body.mass_kg();
        assert!(
            (gained - expected).length() < 1e-12,
            "{gained:?} vs {expected:?}"
        );
        let turned = DVec3::from_slice(&loaded[10..13]) - DVec3::from_slice(&free[10..13]);
        let expected = body.inertia_kg_m2().inverse() * moment;
        assert!(
            (turned - expected).length() < 1e-9 * expected.length(),
            "{turned:?} vs {expected:?}"
        );
        // The position and attitude rates don't depend on the loads.
        assert_eq!(loaded[0..3], free[0..3]);
        assert_eq!(loaded[6..10], free[6..10]);
    }

    /// A moment `M = −c ω` on a body of equal moments `J` damps its spin as `ω₀ e^(−c t / J)`,
    /// with no gyroscopic term to couple the axes: the integrator carries the caller's moment
    /// through a 5 s decay to 1e-7 of the start.
    #[test]
    fn a_damped_spin_decays_as_the_exponential() {
        let j = 0.05;
        let c = 0.02;
        let body = FreeBody::new(1.0, DMat3::from_diagonal(DVec3::splat(j))).unwrap();
        let damping = |inputs: &LoadInputs| {
            Ok(Loads {
                force_body_n: DVec3::ZERO,
                moment_body_n_m: inputs.state.body_rate_rad_s * -c,
            })
        };
        let start = State {
            body_rate_rad_s: DVec3::new(1.0, -2.0, 0.5),
            ..tumbling()
        };
        let times = [1.0, 2.5, 5.0];
        let states = body
            .fly(
                &analytic_environment(UniformAir::vacuum(), 9.806_65),
                &damping,
                Method::default(),
                0.0,
                start,
                &times,
            )
            .unwrap();
        for (time, state) in times.iter().zip(&states) {
            let expected = start.body_rate_rad_s * (-c * time / j).exp();
            let gap = (state.body_rate_rad_s - expected).length();
            assert!(
                gap < 1e-7 * start.body_rate_rad_s.length(),
                "{time} s: {gap:e}"
            );
        }
        // The decay is real: the spin fell to e^(−2) of its start.
        assert!(states[2].body_rate_rad_s.length() < 0.14 * start.body_rate_rad_s.length());
    }

    /// A load that is not finite stops the derivative, naming the component.
    #[test]
    fn a_load_that_is_not_finite_is_refused() {
        let body = brick();
        let bad = |_: &LoadInputs| {
            Ok(Loads {
                force_body_n: DVec3::ZERO,
                moment_body_n_m: DVec3::new(0.0, f64::NAN, 0.0),
            })
        };
        let error = body
            .derivative(&airless_earth(), &bad, &tumbling().to_array())
            .unwrap_err();
        assert!(
            matches!(
                error,
                SimError::Domain { what, value }
                    if what == FREE_BODY_LOAD_NOT_FINITE && value.is_nan()
            ),
            "{error:?}"
        );
    }
}
