//! Core math, units, frames, Earth, gravity and magnetic-field models, interpolation tables and
//! shared error types.
//!
//! **Guide:** [Frames and sign conventions][guide-frames], [Geodesy][guide-geodesy],
//! [Gravity][guide-gravity], [The magnetic field][guide-magnetic], [Interpolation
//! tables][guide-interpolation] and [Adaptive quadrature][guide-quadrature]: the models, their
//! sources, how well they are validated and what they leave out.
//!
//! [guide-frames]: https://hpr.fusionspace.co/physics/frames.html
//! [guide-geodesy]: https://hpr.fusionspace.co/physics/geodesy.html
//! [guide-gravity]: https://hpr.fusionspace.co/physics/gravity.html
//! [guide-magnetic]: https://hpr.fusionspace.co/physics/magnetic.html
//! [guide-interpolation]: https://hpr.fusionspace.co/physics/interpolation.html
//! [guide-quadrature]: https://hpr.fusionspace.co/physics/quadrature.html
//!
//! Vectors, quaternions and matrices are glam's `f64` types, re-exported here so every crate uses
//! the same ones. Quantities are SI; frames and sign conventions follow
//! [Frames and sign conventions][guide-frames].

pub mod attitude;
pub mod earth;
pub mod error;
pub mod frames;
pub mod geodesic;
pub mod geodesy;
pub mod gravity;
pub mod interp;
pub mod magnetic;
pub mod quadrature;
pub mod random;
pub mod tool;

pub use error::CoreError;
pub use glam::{DMat3, DQuat, DVec3};
