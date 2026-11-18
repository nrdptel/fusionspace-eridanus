//! The launch rail: its direction, length and friction, and where a design's rail buttons or launch
//! lugs leave it.
//!
//! The rocket starts with its aft end (the aft-most point of its body, of any part on its outside,
//! fin tips and pods included, or of its nozzle exits) at the foot of the rail, its axis along
//! the rail. It is guided, with one degree of freedom along the rail, until the aft edge of its
//! aft-most guide passes the top of the rail. A rocket with no rail buttons or lugs is guided
//! until its aft end passes the top, as from a tower.
//!
//! Once the guide ahead of the aft-most one is off the rail, a real rocket is held by one guide
//! alone and gravity and the wind turn it about that guide: tip-off. A flight that asks for it
//! ([`crate::Simulation::with_tip_off`]) flies that as a phase of its own (`pivot_motion`, in this module):
//! the rocket slides on along the rail, free to pitch and yaw about its last guide with roll
//! held, until that guide leaves too. Without it the rocket stays guided to its last guide and
//! leaves the rail with no angular velocity.
//!
//! Friction is Coulomb friction, `μ |N|`, with `N` the rail's reaction perpendicular to the rail:
//! the component of gravity, the aerodynamic force and the variable-mass terms across the rocket's
//! axis. It opposes the motion along the rail, and on the pad it holds the rocket until the force
//! along the rail exceeds it.
//!
//! Method: `docs/physics/flight.md`.

use hpr_core::frames::LaunchAngles;
use hpr_core::{DMat3, DQuat, DVec3};
use hpr_design::{Assembly, Part};
use serde::{Deserialize, Serialize};

use crate::error::SimError;

/// A launch rail.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rail {
    /// The rail's length from its foot, where the rocket's aft end starts, to its top, m.
    pub length_m: f64,
    /// The heading of the rail, clockwise from true north, rad (`frames::LaunchAngles`).
    pub azimuth_rad: f64,
    /// The rail's angle above the horizon, rad (`π/2` is vertical).
    pub elevation_rad: f64,
    /// The rocket's roll on the rail: the turn of `x_B` about the rail axis, rad.
    pub roll_rad: f64,
    /// The Coulomb friction coefficient between the guides and the rail. There is no default
    /// value in the sources; zero means a frictionless rail.
    pub friction_coefficient: f64,
}

impl Rail {
    /// A frictionless vertical rail of length `length_m`.
    #[must_use]
    pub fn vertical(length_m: f64) -> Self {
        Self {
            length_m,
            azimuth_rad: 0.0,
            elevation_rad: std::f64::consts::FRAC_PI_2,
            roll_rad: 0.0,
            friction_coefficient: 0.0,
        }
    }

    /// Checks the length, angles and friction.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] for a non-positive length, a negative friction coefficient, an
    /// elevation outside `(0, π/2]` or a non-finite angle.
    pub fn validate(&self) -> Result<(), SimError> {
        let checks = [
            ("rail length", self.length_m, self.length_m > 0.0),
            (
                "rail friction coefficient",
                self.friction_coefficient,
                self.friction_coefficient >= 0.0,
            ),
            (
                "rail elevation",
                self.elevation_rad,
                self.elevation_rad > 0.0 && self.elevation_rad <= std::f64::consts::FRAC_PI_2,
            ),
            ("rail azimuth", self.azimuth_rad, true),
            ("rail roll", self.roll_rad, true),
        ];
        for (what, value, ok) in checks {
            if !value.is_finite() || !ok {
                return Err(SimError::Domain { what, value });
            }
        }
        Ok(())
    }

    /// The rocket's attitude on the rail.
    #[must_use]
    pub fn attitude(&self) -> DQuat {
        LaunchAngles {
            azimuth_rad: self.azimuth_rad,
            elevation_rad: self.elevation_rad,
            roll_rad: self.roll_rad,
        }
        .to_quaternion()
    }

    /// The unit vector along the rail, up, in the launch frame.
    #[must_use]
    pub fn direction_enu(&self) -> DVec3 {
        self.attitude().mul_vec3(DVec3::Z)
    }
}

/// Where a design meets the rail, as stations (m aft of the nose tip).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Guides {
    /// The rocket's aft end, which starts at the foot of the rail: the aft-most point of its body,
    /// of every part on its outside (a fin's tip trailing edge where a swept fin reaches past the
    /// tube, a tube fin's or a pod's aft end) and of its nozzle exits. The rocket stands on that
    /// point (the decision record on the rail, [ADR-011][adr-011]; #1254).
    ///
    /// [adr-011]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-011-rigid-body-flight-equations-of-motion-aerodynamic-coupling-rail-phases-and-termination-2026-09-17
    pub aft_station_m: f64,
    /// The aft edge of the forward-most guide (a rail button's aft edge, or a lug's aft end), or
    /// `None` without guides.
    pub first_guide_station_m: Option<f64>,
    /// The aft edge of the aft-most guide, or `None` without guides.
    pub last_guide_station_m: Option<f64>,
    /// The aft edge of the guide next ahead of the aft-most one, or `None` with fewer than two
    /// guides at different stations. Once it passes the top of the rail the aft-most guide holds
    /// the rocket alone, and tip-off begins.
    #[serde(default)]
    pub next_guide_station_m: Option<f64>,
    /// The middle of the aft-most guide (a button's center, a lug's midpoint), where the rocket
    /// pivots during tip-off, or `None` without guides.
    #[serde(default)]
    pub pivot_station_m: Option<f64>,
}

impl Guides {
    /// The guides of an assembled design: every rail button of every row (each `outer_diameter`
    /// long, `spacing` apart) and every launch lug (each `length` long).
    #[must_use]
    pub fn of(assembly: &Assembly) -> Self {
        let body_aft = assembly.layout.length_m;
        // Every body component (a pod's too) and every part on the outside, as far aft as it
        // reaches: a fin set by its outline, not its root chord. A part with no copies isn't
        // there. Internal parts aren't counted (a design check warns when one runs past its
        // parent's end); a motor reaches out by its nozzle.
        let outside_aft = assembly
            .layout
            .components
            .iter()
            .filter(|c| (c.part.is_body() || c.part.is_external()) && !c.copies.is_empty())
            .map(|c| match &c.part {
                Part::FinSet(fins) => c.fore_station_m + fins.planform.aft_extent_m(),
                _ => c.aft_station_m(),
            })
            .fold(body_aft, f64::max);
        let aft_station_m = assembly
            .motors
            .iter()
            .map(|motor| motor.nozzle_station_m())
            .fold(outside_aft, f64::max);
        let mut first: Option<f64> = None;
        // The aft-most guide's aft edge and its length, and the aft edge of the one next ahead.
        let mut last: Option<(f64, f64)> = None;
        let mut next: Option<f64> = None;
        for component in &assembly.layout.components {
            let (count, spacing, extent) = match &component.part {
                Part::RailButton(button) => {
                    (button.count, button.spacing_m, button.outer_diameter_m)
                }
                Part::LaunchLug(lug) => (lug.count, lug.spacing_m, lug.length_m),
                _ => continue,
            };
            for k in 0..count {
                let aft_edge = component.fore_station_m + f64::from(k) * spacing + extent;
                first = Some(first.map_or(aft_edge, |f| f.min(aft_edge)));
                match last {
                    Some((edge, _)) if aft_edge > edge + SAME_GUIDE_M => {
                        next = Some(edge);
                        last = Some((aft_edge, extent));
                    }
                    Some((edge, _)) if aft_edge < edge - SAME_GUIDE_M => {
                        next = Some(next.map_or(aft_edge, |n| n.max(aft_edge)));
                    }
                    // A guide at the aft-most one's station (a second row beside it) leaves
                    // with it: the two are one guide here, and the aft-most edge of the two is
                    // the guide's.
                    Some((edge, kept)) => last = Some((edge.max(aft_edge), kept)),
                    None => last = Some((aft_edge, extent)),
                }
            }
        }
        Self {
            aft_station_m,
            first_guide_station_m: first,
            last_guide_station_m: last.map(|(edge, _)| edge),
            next_guide_station_m: next,
            pivot_station_m: last.map(|(edge, extent)| edge - 0.5 * extent),
        }
    }

    /// How far the rocket travels along a rail of `rail_length_m` before its last guide leaves the
    /// top: `L − (s_aft − s_guide)`, with the aft end standing in for the guide without guides.
    #[must_use]
    pub fn exit_travel_m(&self, rail_length_m: f64) -> f64 {
        let guide = self.last_guide_station_m.unwrap_or(self.aft_station_m);
        rail_length_m - (self.aft_station_m - guide)
    }

    /// How far the rocket travels before its first guide leaves the top.
    #[must_use]
    pub fn first_guide_exit_travel_m(&self, rail_length_m: f64) -> f64 {
        let guide = self.first_guide_station_m.unwrap_or(self.aft_station_m);
        rail_length_m - (self.aft_station_m - guide)
    }

    /// How far the rocket travels before its last guide holds it alone, where tip-off begins:
    /// the guide next ahead of the last one leaving the top, `L − (s_aft − s_next)`. `None`
    /// with fewer than two guides at different stations: one guide, or a tower, has no such
    /// stretch.
    #[must_use]
    pub fn tip_off_travel_m(&self, rail_length_m: f64) -> Option<f64> {
        self.next_guide_station_m
            .map(|guide| rail_length_m - (self.aft_station_m - guide))
    }
}

/// How close two guides' aft edges are, m, for the two to count as one guide that leaves the
/// rail at once: a micron. Two rows of buttons at one nominal station can reach the layout
/// through different sums and land a few ulps apart; a stretch that short on the "last" of them
/// alone is rounding, not tip-off.
const SAME_GUIDE_M: f64 = 1e-6;

/// What a rocket on its last guide alone needs besides the free equations: where it pivots and
/// the line the pivot keeps to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pivot {
    /// The pivot in body axes, m from the nose tip: on the axis at the last guide's middle.
    pub(crate) point_m: DVec3,
    /// The unit vector along the rail, up, in the launch frame.
    pub(crate) rail_enu: DVec3,
}

/// The motion of a rocket that slides along the rail on its last guide alone, free to pitch and
/// yaw about that guide with roll held: tip-off.
///
/// The free equations of motion ([`crate::dynamics`]) give the nose tip's acceleration `a` and
/// the angular acceleration `ω̇` the rocket would have with nothing holding it. The guide adds
/// what keeps it on the rail: a force `F` on the pivot `p`, square to the rail, and a moment
/// `τ z_B` about the long axis. With `d = p − r` the pivot from the center of mass, each changes
/// the motion by
///
/// ```text
/// Δω̇  = I_c⁻¹ (d × F + τ z_B)
/// Δa_p = F/m + Δω̇ × d            (the pivot's acceleration)
/// Δa   = F/m − Δω̇ × r            (the nose tip's)
/// ```
///
/// and the three unknowns, `F`'s two parts across the rail and `τ`, are fixed by three
/// conditions: the pivot's acceleration `a + ω̇ × p + ω × (ω × p)` has no part across the rail
/// (two), and the rocket's angular acceleration has none about its long axis (one). The three
/// equations are linear, with a symmetric positive-definite matrix.
///
/// Friction is the rail's Coulomb friction on that force: `μ |F|` on the pivot along the rail,
/// against the sliding (`sliding` is the sign of the pivot's speed up the rail). It turns the
/// rocket slightly, which changes `F`; with `x₀` the frictionless solution, `c` the conditions'
/// response to a unit force against the sliding and `u = J⁻¹ c`, the solution is `x₀ − μρu` with
/// `ρ = |F|` the non-negative root of `(1 − μ²|u_F|²) ρ² + 2μ (x₀·u)_F ρ − |x₀|_F² = 0`, where
/// the subscript keeps the force's two parts.
///
/// Source: the reaction-wrench form of RocketPy's tip-off phase (`Flight.udot_rail2`, its pull
/// request #920, MIT; `THIRD-PARTY-NOTICES.md`), written for this crate's equations about the
/// nose tip; the friction term is this crate's rail friction carried over. On a body of fixed
/// mass with no thrust and no air it reduces to a pendulum's `ω̇ = m g r cos θ / (I + m r²)`.
/// Method and a worked example: `docs/physics/flight.md`.
///
/// Returns the nose tip's acceleration and the angular acceleration, both in body axes.
///
/// # Errors
///
/// [`SimError::Domain`] if the inertia about the center of mass has no inverse, or the
/// conditions' matrix none, or the friction coefficient is so large that the rocket would jam on
/// its guide (`μ |u_F| ≥ 1`).
pub(crate) fn pivot_motion(
    (mass_kg, cg_m, inertia_cg): (f64, DVec3, DMat3),
    body_rate_rad_s: DVec3,
    (free_acceleration, free_angular_acceleration): (DVec3, DVec3),
    (pivot_m, rail_body): (DVec3, DVec3),
    (friction_coefficient, sliding): (f64, f64),
) -> Result<(DVec3, DVec3), SimError> {
    let positive = |what: &'static str, value: f64| {
        if value.is_finite() && value > 0.0 {
            Ok(value)
        } else {
            Err(SimError::Domain { what, value })
        }
    };
    positive(
        "determinant of the inertia about the center of mass",
        inertia_cg.determinant(),
    )?;
    let inverse_inertia = inertia_cg.inverse();
    let (r, p, omega) = (cg_m, pivot_m, body_rate_rad_s);
    let d = p - r;
    // Two unit vectors across the rail, in body axes: the first from the body axis least along
    // the rail.
    let n = rail_body.normalize();
    let (ax, ay, az) = (n.x.abs(), n.y.abs(), n.z.abs());
    let helper = if az <= ax && az <= ay {
        DVec3::Z
    } else if ay <= ax {
        DVec3::Y
    } else {
        DVec3::X
    };
    let e1 = n.cross(helper).normalize();
    let e2 = n.cross(e1);
    // What a force on the pivot and a moment about the long axis change: the angular
    // acceleration, and the three conditions.
    let turn = |force: DVec3, roll_moment: f64| {
        inverse_inertia * (d.cross(force) + DVec3::Z * roll_moment)
    };
    let conditions = |force: DVec3, roll_moment: f64| {
        let angular = turn(force, roll_moment);
        let pivot = force / mass_kg + angular.cross(d);
        DVec3::new(pivot.dot(e1), pivot.dot(e2), angular.z)
    };
    let jacobian = DMat3::from_cols(
        conditions(e1, 0.0),
        conditions(e2, 0.0),
        conditions(DVec3::ZERO, 1.0),
    );
    positive(
        "determinant of the tip-off conditions",
        jacobian.determinant(),
    )?;
    let inverse = jacobian.inverse();
    let pivot_free =
        free_acceleration + free_angular_acceleration.cross(p) + omega.cross(omega.cross(p));
    let unheld = DVec3::new(
        pivot_free.dot(e1),
        pivot_free.dot(e2),
        free_angular_acceleration.z,
    );
    let frictionless = inverse * -unheld;
    // Friction: a force on the pivot against its sliding, as large as μ times the force across.
    let against = n * -sliding;
    let mut solution = frictionless;
    let mut friction_n = 0.0;
    if friction_coefficient > 0.0 {
        let mu = friction_coefficient;
        let u = inverse * conditions(against, 0.0);
        let (x0, u2) = (frictionless.truncate(), u.truncate());
        let lead = positive(
            "room before the rocket jams on its last guide, 1 − μ²|u|² (the rail friction \
             coefficient is too large for tip-off)",
            1.0 - mu * mu * u2.length_squared(),
        )?;
        let b = mu * x0.dot(u2);
        let across = ((b * b + lead * x0.length_squared()).sqrt() - b) / lead;
        friction_n = mu * across;
        solution = frictionless - u * friction_n;
    }
    let force = e1 * solution.x + e2 * solution.y + against * friction_n;
    let change = turn(force, solution.z);
    let angular = free_angular_acceleration + change;
    let acceleration = free_acceleration + force / mass_kg - change.cross(r);
    // Roll is held: the solved component is rounding, and is dropped so it can't build up.
    Ok((acceleration, DVec3::new(angular.x, angular.y, 0.0)))
}

#[cfg(test)]
mod tests;
