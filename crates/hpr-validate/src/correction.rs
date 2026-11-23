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
use hpr_sim::{Environment, FlightSettings, Method, SimError, Simulation};
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
/// only the wind. With no correction, the phase and the report's wind it is that report's flight
/// itself: the phase is HPR Sim's default.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Variant {
    /// The correction flown, or none: the motors and the drag as the report flies them.
    pub correction: Option<Correction>,
    /// Whether the flight flies the tip-off phase.
    pub tip_off: bool,
    /// Whether the wind is taken out: what the phase moves in still air is gravity's part alone.
    pub still_air: bool,
    /// Whether the time step is halved: the flight flown again for its numerical uncertainty
    /// ([`Variant::settings`]; [the decision that splits the validation uncertainty][adr-373]).
    ///
    /// [adr-373]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0373-m1-14b7-in-three-the-numerical-part-first.md
    #[serde(default)]
    pub step_halved: bool,
}

/// What the adaptive stepper's tolerances are divided by to halve its step: `2⁵`. Dormand–Prince
/// 5(4) controls a local error estimate of order `h⁵` (Hairer, Nørsett and Wanner, *Solving
/// Ordinary Differential Equations I*, 2nd ed., §II.4; `docs/physics/integration.md`), so its
/// step goes as the tolerance to the power 1/5, and a tolerance `2⁵` times tighter asks for steps
/// about half as long. The ratio is measured, not assumed: on the report's flights it is 1.85 to
/// 2.00 ([the decision that splits the validation uncertainty][adr-373]).
///
/// [adr-373]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0373-m1-14b7-in-three-the-numerical-part-first.md
pub const STEP_HALVING_TOLERANCE_DIVISOR: f64 = 32.0;

impl Variant {
    /// The report's flight held straight to its last guide: no correction, the tip-off phase off.
    pub const GUIDED: Self = Self {
        correction: None,
        tip_off: false,
        still_air: false,
        step_halved: false,
    };

    /// The report's own flight: the tip-off phase, HPR Sim's default, and no correction.
    pub const TIP_OFF: Self = Self {
        correction: None,
        tip_off: true,
        still_air: false,
        step_halved: false,
    };

    /// [`Variant::TIP_OFF`], the report's own flight, with the time step halved.
    pub const STEP_HALVED: Self = Self {
        step_halved: true,
        ..Self::TIP_OFF
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

    /// `correction` flown as the report flies, with the tip-off phase.
    #[must_use]
    pub fn corrected(correction: Correction) -> Self {
        Self {
            correction: Some(correction),
            ..Self::TIP_OFF
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

    /// `settings` with the time step halved if the variant asks: the adaptive stepper's two
    /// tolerances divided by [`STEP_HALVING_TOLERANCE_DIVISOR`], a fixed step halved. Every other
    /// setting as given.
    #[must_use]
    pub fn settings(&self, settings: FlightSettings) -> FlightSettings {
        if !self.step_halved {
            return settings;
        }
        let method = match settings.method {
            Method::DormandPrince54(adaptive) => {
                let mut adaptive = adaptive;
                adaptive.relative_tolerance /= STEP_HALVING_TOLERANCE_DIVISOR;
                adaptive.absolute_tolerance /= STEP_HALVING_TOLERANCE_DIVISOR;
                Method::DormandPrince54(adaptive)
            }
            Method::Rk4 { step_s } => Method::Rk4 {
                step_s: step_s / 2.0,
            },
            // A method added later is flown as it is: its second run is then no finer, and the
            // numerical part refuses it ("not finer") rather than report a part of zero.
            other => other,
        };
        FlightSettings { method, ..settings }
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
            simulation.without_tip_off()
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

    #[test]
    fn a_variant_changes_only_what_it_names() {
        use hpr_sim::{FlightSettings, Rail};
        let rocket = bella_lui();
        let site = hpr_core::geodesy::Geodetic {
            latitude_rad: 0.8,
            longitude_rad: 0.1,
            height_m: 400.0,
        };
        let windy = Environment::standard(site)
            .unwrap()
            .with_wind(ConstantWind::new(6.0, 1.0).unwrap());
        // Still air takes out the wind and keeps the air.
        let still = Variant::STILL_GUIDED.environment(windy.clone());
        assert_eq!(
            still.wind.wind(500.0).unwrap().velocity_enu_m_s.length(),
            0.0
        );
        assert!(windy.wind.wind(500.0).unwrap().velocity_enu_m_s.length() > 5.9);
        assert_eq!(
            still.atmosphere.air(500.0).unwrap(),
            windy.atmosphere.air(500.0).unwrap()
        );
        // The report's wind is left as it is.
        let kept = Variant::TIP_OFF.environment(windy.clone());
        assert_eq!(
            kept.wind.wind(500.0).unwrap(),
            windy.wind.wind(500.0).unwrap()
        );
        // The guided variants alone turn the phase off; a correction flies as the report does,
        // with it. Only a correction scales the rocket.
        let simulation = || {
            Simulation::new(
                &rocket,
                "example",
                windy.clone(),
                Rail::vertical(5.0),
                FlightSettings::default(),
            )
            .unwrap()
        };
        for (variant, phase) in [
            (Variant::GUIDED, false),
            (Variant::TIP_OFF, true),
            (Variant::STILL_GUIDED, false),
            (Variant::STILL_TIP_OFF, true),
            (Variant::corrected(Correction::NONE), true),
        ] {
            assert_eq!(variant.simulation(simulation()).unwrap().tip_off(), phase);
        }
        assert!(Variant::TIP_OFF.rocket(&rocket).unwrap().is_none());
        assert!(
            Variant::corrected(Correction::NONE)
                .rocket(&rocket)
                .unwrap()
                .is_some()
        );
    }
}
