//! A free rigid body of fixed mass and inertia, with no design, motor or aerodynamics: the
//! flight's gravity, Earth rotation, rigid-body equations and integrator on a body a test gives.
//!
//! It exists to fly published check cases whose bodies are not rockets, such as NASA's dragless
//! sphere and tumbling brick (NASA/TM-2015-218675, cases 1 and 2), through the code a rocket
//! flies on. Nothing here is a second model: the weight and the Coriolis force come from the
//! [`Environment`]'s Earth, as [`crate::dynamics`] takes them, the two accelerations from that
//! module's free-flight equations, and the steps from [`crate::integrator`]. A test holds the
//! derivative to the one a coasting rocket's flight evaluates at the same state.
//!
//! With the reference point at the center of mass, a constant mass `m` and a constant inertia `I`
//! about it, and no load but gravity `g` and the Coriolis force, the equations of
//! [`crate::dynamics`] reduce to Newton's and Euler's:
//!
//! ```text
//! a  = g(x) − 2 Ω × v
//! ω̇ = I⁻¹ (−ω × I ω)
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

/// A rigid body of fixed mass and inertia.
///
/// Its [`State`] is the center of mass's, where a rocket's is its nose tip's: the position and
/// velocity are the center of mass's in the launch frame, the attitude takes the body's axes to
/// the launch frame's, and the body rates are relative to the launch frame, in body axes. The
/// body's axes are whatever the inertia is given in; no axis is a nose.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FreeBody {
    mass_kg: f64,
    inertia_kg_m2: DMat3,
}

impl FreeBody {
    /// A body of mass `mass_kg` and inertia tensor `inertia_kg_m2` about its center of mass, in
    /// its own axes, kg·m².
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

    /// The state's time derivative in `environment`, in [`State::to_array`]'s order: the weight
    /// and the Coriolis force at the center of mass, as a rocket's flight takes them, through the
    /// flight's free-motion equations.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] for an attitude quaternion whose norm is zero or not finite;
    /// [`SimError::Core`] if the Earth model refuses the position.
    pub fn derivative(
        &self,
        environment: &Environment,
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
            [DVec3::ZERO, DVec3::ZERO, weight],
            DMat3::ZERO,
            [DVec3::ZERO; 3],
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

    /// Flies the body in `environment` from `state` at `t0_s` and gives its state at each of
    /// `times_s`, read from the steps' dense output as a flight's recorder reads its samples. The
    /// times don't shape the steps, but for the last, where the integration stops. Nothing ends
    /// the flight early: the ground is not looked for.
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
        method: Method,
        t0_s: f64,
        state: State,
        times_s: &[f64],
    ) -> Result<Vec<State>, SimError> {
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
        let mut integrator = Integrator::new(method, t0_s, state.to_array())?;
        let mut system = Flight {
            body: self,
            environment,
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
        self.body.derivative(self.environment, y)
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
        let derivative = body.derivative(&environment, &at_cg.to_array()).unwrap();
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
            .fly(&airless_earth(), Method::default(), 0.0, start, &times)
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
                .fly(&environment, Method::default(), 0.0, start, &[30.0])
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
            body.fly(&environment, Method::default(), 2.0, start, times)
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
    }
}
