//! The library facade of FusionSpace HPR · Sim: a builder API for environments, motors, rockets
//! and flights, and the workspace's crates re-exported.
//!
//! The guide has three pages to start from. [Start here][guide-start] says what works and how far
//! to trust it. [Getting started][guide-first] flies a first rocket. [The builder][guide-builder]
//! walks through this crate's API. The [`guide`] module is the same walk-through in this
//! reference, in five short chapters.
//!
//! [guide-start]: https://hpr.fusionspace.co/start-here.html
//! [guide-first]: https://hpr.fusionspace.co/getting-started.html
//! [guide-builder]: https://hpr.fusionspace.co/the-builder.html
//!
//! Four types make a flight, in the order a program needs them:
//!
//! - [`Environment`]: the launch site, the atmosphere and the wind.
//! - [`Motor`]: a motor from the built-in catalog or a RASP `.eng` file.
//! - [`Rocket`]: parts added from the nose back, the motor, and the recovery devices.
//! - [`Flight`]: the rocket flown from a rail, with its apogee, speeds and landing.
//!
//! > **How far to trust it.** The builder adds no physics of its own. It makes the same design
//! > tree and flies the same simulation as the crates below it. A test flies a rocket built both
//! > ways to the same numbers, bit for bit. So its numbers are as good as those models are. The
//! > guide's [Accuracy][guide-accuracy] page says how good that is, with every comparison made.
//!
//! [guide-accuracy]: https://hpr.fusionspace.co/accuracy.html
//!
//! ```
//! use hpr::rocket::{Fins, Mass, MotorTube, Nose, Tube, material};
//! use hpr::{
//!     CanopyType, Device, DeviceDrag, Environment, FinPlanform, Flight, Motor, NoseShape,
//!     Position, Rocket, Trigger,
//! };
//!
//! let environment = Environment::new(32.99, -106.97, 1400.0)?;
//! let ogive = NoseShape::Ogive { radius_ratio: 1.0 };
//! let planform = FinPlanform::Trapezoidal {
//!     root_chord_m: 0.1,
//!     tip_chord_m: 0.04,
//!     span_m: 0.045,
//!     sweep_m: 0.05,
//! };
//! let mut rocket = Rocket::new("Small", 0.0563)?;
//! rocket
//!     .add_nose(Nose::hollow(ogive, 0.22, 0.0015, material("abs")?))?
//!     .add_tube(Tube::new(0.9, 0.00115, material("kraft_phenolic")?))?
//!     .add_fins(Fins::new(3, planform, 0.003175, material("birch_plywood")?))?
//!     .add_motor_tube(MotorTube::new(0.2, 0.029, 0.001, material("kraft_phenolic")?))?
//!     .add_mass(Mass::new(0.2, Position::Top { aft_offset_m: 0.07 }))?
//!     .set_motor(Motor::from_catalog("H54")?.with_delay_s(10.0)?)?
//!     .add_parachute(Device::new(
//!         "parachute",
//!         DeviceDrag::canopy(CanopyType::FlatCircular, 0.9),
//!         Trigger::MotorDelay { motor: 0 },
//!     ));
//! let flight = Flight::builder(&rocket, &environment, 1.8).fly()?;
//! assert!(flight.apogee_m().is_some_and(|apogee_m| apogee_m > 1000.0));
//! # Ok::<(), hpr::Error>(())
//! ```
//!
//! The builder makes the same things the crates below it use, and hands them over. A rocket's
//! [`Rocket::design`] is an [`hpr_design::Rocket`], and a flight's [`FlightBuilder::simulation`]
//! is an [`hpr_sim::Simulation`]. For what the builder doesn't offer, such as staging, clusters,
//! pods or mass shifts, build with the crates, which this one re-exports by name. Each covers
//! one part of the work:
//!
//! - [`hpr_core`]: the math, units, frames, the Earth and gravity.
//! - [`hpr_atmos`]: the atmosphere and the wind.
//! - [`hpr_motor`]: solid motors, their thrust curves and motor files.
//! - [`hpr_design`]: a rocket's parts, materials, mass and stages.
//! - [`hpr_aero`]: the aerodynamics: normal force, center of pressure and drag.
//! - [`hpr_sim`]: the flight itself, in six degrees of freedom, with recovery.
//! - [`hpr_analysis`]: Monte Carlo runs, sensitivity analysis and optimization.
//! - [`hpr_format`]: HPR's own design format.
//! - [`hpr_io`]: other programs' files: OpenRocket designs and parts, weather and elevation.
//! - [`hpr_flightdata`]: flight logs and the readings taken from them, with no simulator.
//! - [`hpr_forensics`]: a flight log against a simulation of it, planned; today it holds only
//!   synthetic flights to test the readings on.
//!
//! The `net` feature adds `hpr-net` (as `hpr::hpr_net`). It holds the cache, the offline mode and
//! the HTTP transport, and these sources:
//!
//! - `hpr_net::open_meteo`: a launch site's weather from Open-Meteo.
//! - `hpr_net::wyoming`: weather-balloon soundings from the University of Wyoming's archive.
//! - `hpr_net::nomads`: NOAA's GFS and RAP forecasts.
//! - `hpr_net::elevation`: a site's height above sea level from Open-Meteo.
//! - `hpr_net::motor_finder`: motor stock and prices from motor.fusionspace.co.
//! - `hpr_net::thrustcurve`: motor records and thrust curves from ThrustCurve.org.
//!
//! The `parquet` feature turns on `hpr_sim`'s Parquet export (`hpr_sim::export::parquet`).
//! [`ork`] flies the stage separation an OpenRocket `.ork` file describes.

pub mod environment;
pub mod error;
pub mod flight;
pub mod guide;
pub mod motor;
pub mod ork;
pub mod rocket;

pub use environment::Environment;
pub use error::{CatalogProblem, Error, Order};
pub use flight::{Flight, FlightBuilder};
pub use motor::Motor;
pub use rocket::Rocket;

pub use hpr_aero;
pub use hpr_analysis;
pub use hpr_atmos;
pub use hpr_core;
pub use hpr_design;
pub use hpr_flightdata;
pub use hpr_forensics;
pub use hpr_format;
pub use hpr_io;
pub use hpr_motor;
#[cfg(feature = "net")]
pub use hpr_net;
pub use hpr_sim;

pub use hpr_design::{FinCrossSection, FinPlanform, NoseShape, Position};
pub use hpr_sim::{CanopyType, Device, DeviceDrag, EnvelopeFlag, Separation, Trigger};

#[cfg(test)]
mod tests;
