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

use hpr_analysis::montecarlo::dispersed_motor;
use hpr_design::Rocket;
use hpr_sim::{SimError, Simulation};
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
    pub fn rocket(&self, rocket: &Rocket) -> Result<Rocket, String> {
        let mut rocket = rocket.clone();
        for configuration in &mut rocket.configurations {
            for mounted in &mut configuration.motors {
                mounted.motor = dispersed_motor(&mounted.motor, self.impulse_scale, 1.0)
                    .map_err(|error| error.to_string())?;
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
