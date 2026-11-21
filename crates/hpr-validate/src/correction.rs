//! A correction of the bias the simulators share, flown on a public flight:
//! every motor's impulse scaled by one number and the drag by another, the rest of the flight as
//! its report flies it.
//!
//! The motors are scaled through `hpr_analysis::montecarlo::dispersed_motor` at the rated burn
//! time, which multiplies the thrust and the propellant's mass by the impulse scale, and the drag
//! through `Simulation::with_drag_scale`: the same functions the fit's model of a boost uses
//! (`hpr_forensics::fit`), so a scale carried from the fits means here what it meant there. Every
//! motor of the rocket is scaled, in every configuration: one lit after a boost's cut, an upper
//! stage's or a cluster's, is scaled too, since a correction in a flight's output wouldn't know
//! which motors a log measured.

use hpr_analysis::AnalysisError;
use hpr_analysis::montecarlo::dispersed_motor;
use hpr_atmos::ConstantWind;
use hpr_design::Rocket;
use hpr_sim::{Environment, SimError, Simulation};
use serde::{Deserialize, Serialize};

/// The two scales of a correction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Correction {
    /// Every motor's total impulse over its rated impulse, at the rated burn time.
    pub impulse_scale: f64,
    /// The drag over the model's.
    pub drag_scale: f64,
}

impl Correction {
    /// No correction: both scales 1. Flown, it must be the flight its report flies.
    pub const NONE: Self = Self {
        impulse_scale: 1.0,
        drag_scale: 1.0,
    };

    /// `rocket` with every motor of every configuration scaled by the impulse scale.
    ///
    /// # Errors
    ///
    /// A scale that isn't a positive number, or a motor whose propellant can't be scaled.
    pub fn rocket(&self, rocket: &Rocket) -> Result<Rocket, AnalysisError> {
        let mut rocket = rocket.clone();
        for configuration in &mut rocket.configurations {
            for mounted in &mut configuration.motors {
                mounted.motor = dispersed_motor(&mounted.motor, self.impulse_scale, 1.0)?;
            }
        }
        Ok(rocket)
    }

    /// `simulation` with its drag scaled by the drag scale.
    ///
    /// # Errors
    ///
    /// A drag scale that isn't a positive number.
    pub fn simulation(&self, simulation: Simulation) -> Result<Simulation, SimError> {
        simulation.with_drag_scale(self.drag_scale)
    }
}

/// A public flight flown beside its report's flight: with or without a [`Correction`], with or
/// without the tip-off phase (`Simulation::with_tip_off`; [what it moves on the real
/// flights](https://hpr.fusionspace.co/physics/flight.html#the-tip-off-phase-on-the-real-flights)),
/// and in its report's wind or in still air. Everything else is as the flight's report flies
/// it: still air keeps the report's air (its pressure, temperature and density) and takes out
/// only the wind. With no correction, no phase and the report's wind it is that report's flight
/// itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Variant {
    /// The correction flown, or none: the motors and the drag as the report flies them.
    pub correction: Option<Correction>,
    /// Whether the flight flies the tip-off phase.
    pub tip_off: bool,
    /// Whether the wind is taken out: what the phase moves in still air is gravity's part alone.
    pub still_air: bool,
}

impl Variant {
    /// The report's own flight: no correction, no tip-off phase.
    pub const GUIDED: Self = Self {
        correction: None,
        tip_off: false,
        still_air: false,
    };

    /// The report's flight with the tip-off phase and no correction.
    pub const TIP_OFF: Self = Self {
        correction: None,
        tip_off: true,
        still_air: false,
    };

    /// [`Variant::GUIDED`] in still air.
    pub const STILL_GUIDED: Self = Self {
        still_air: true,
        ..Self::GUIDED
    };

    /// [`Variant::TIP_OFF`] in still air.
    pub const STILL_TIP_OFF: Self = Self {
        still_air: true,
        ..Self::TIP_OFF
    };

    /// `correction` flown, without the tip-off phase.
    #[must_use]
    pub fn corrected(correction: Correction) -> Self {
        Self {
            correction: Some(correction),
            ..Self::GUIDED
        }
    }

    /// `environment` with its wind taken out if the variant asks for still air.
    #[must_use]
    pub fn environment(&self, environment: Environment) -> Environment {
        if self.still_air {
            environment.with_wind(ConstantWind::calm())
        } else {
            environment
        }
    }

    /// `rocket` as [`Correction::rocket`] scales it, or `None` with no correction: the rocket as
    /// given.
    ///
    /// # Errors
    ///
    /// As [`Correction::rocket`].
    pub fn rocket(&self, rocket: &Rocket) -> Result<Option<Rocket>, AnalysisError> {
        self.correction.map(|c| c.rocket(rocket)).transpose()
    }

    /// `simulation` with the correction's drag scale, if any, and the tip-off phase if asked.
    ///
    /// # Errors
    ///
    /// As [`Correction::simulation`].
    pub fn simulation(&self, simulation: Simulation) -> Result<Simulation, SimError> {
        let simulation = match self.correction {
            Some(correction) => correction.simulation(simulation)?,
            None => simulation,
        };
        Ok(if self.tip_off {
            simulation.with_tip_off()
        } else {
            simulation
        })
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;

    /// A committed design with a motor, read from the repository.
    fn bella_lui() -> Rocket {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../validation/designs/rocketpy-bella-lui.json"
        );
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    fn impulses(rocket: &Rocket) -> Vec<f64> {
        rocket
            .configurations
            .iter()
            .flat_map(|c| c.motors.iter())
            .map(|m| m.motor.curve().total_impulse_ns())
            .collect()
    }

    #[test]
    fn every_motor_is_scaled_by_the_impulse_scale() {
        let rocket = bella_lui();
        let rated = impulses(&rocket);
        assert!(!rated.is_empty());
        let scaled = Correction {
            impulse_scale: 0.94,
            drag_scale: 1.0,
        }
        .rocket(&rocket)
        .unwrap();
        for (rated, scaled) in rated.iter().zip(impulses(&scaled)) {
            assert!((scaled / rated - 0.94).abs() < 1e-12, "{scaled} of {rated}");
        }
        // No correction leaves every motor's impulse as it was.
        let same = Correction::NONE.rocket(&rocket).unwrap();
        assert_eq!(impulses(&same), rated);
    }

    #[test]
    fn a_scale_that_is_not_positive_is_refused() {
        let rocket = bella_lui();
        for impulse_scale in [0.0, -1.0, f64::NAN] {
            let correction = Correction {
                impulse_scale,
                drag_scale: 1.0,
            };
            assert!(correction.rocket(&rocket).is_err(), "{impulse_scale}");
        }
    }
}
