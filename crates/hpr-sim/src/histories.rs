//! Thrust, and mass with the center of mass, against flight time, given from outside in place of
//! the flight's own: for sizing the difference between two codes, not a model
//! ([M2.3c11][m2-3c11], the increment that added them; [ADR-349][adr-349]).
//!
//! The gap between HPR Sim's apogee and another tool's is split into parts by swapping one input
//! at a time for the other tool's ([ADR-226][adr-226]). A drag table swaps the drag
//! ([`crate::Simulation::with_drag_table`]); these swap the thrust ([`ThrustHistory`]) and the
//! mass and center of mass ([`MassHistory`]), each a list of samples at the other tool's own
//! times.
//!
//! **Interpolation.** Between two samples `(t_i, y_i)` and `(t_{i+1}, y_{i+1})` a history is the
//! straight line
//!
//! ```text
//! y(t) = y_i + (y_{i+1} − y_i) (t − t_i) / (t_{i+1} − t_i)
//! ```
//!
//! and before the first sample or after the last it holds that sample's value. So its rate is
//! the slope of the piece it is on, finite everywhere, and its second derivative is zero inside
//! a piece. The flight stops its integration steps at every sample where the slope changes, so
//! no step and no difference stencil of the mass properties straddles a corner: inside a step
//! the rates are the piece's slopes exactly.
//!
//! **Thrust.** The history's value is the thrust as flown, N, along the rocket's axis `+z_B`: it
//! takes the place of the sum of the motors' thrusts with HPR Sim's correction for the air's
//! pressure, which is not applied to it again. It acts at the mean of the nozzle exits of the
//! motors lit by a known time, so a symmetric cluster's thrust has no moment, as HPR Sim's own
//! has none, and an asymmetric cluster's moment is left out. HPR Sim's motors still set when a
//! motor burns: its burnout event, the burning motors' cross-section taken off the base drag,
//! and the mass flow behind jet damping and the nozzle's terms of the equations of motion
//! (`docs/physics/flight.md`).
//!
//! **Mass.** The history gives the whole rocket's mass `m(t)` and its center of mass `x(t)`, m
//! aft of the nose tip along the axis, the body origin `O`: in body axes the center of mass is
//! `r = (r_x, r_y, −x)`, its lateral offsets HPR Sim's own. The mass rate `ṁ`, the center of
//! mass's rate `r′` and its acceleration `r″` are the history's slopes and zero. The inertia
//! about the center of mass `I_c` stays HPR Sim's, and the inertia about `O` is taken from it by
//! the parallel-axis theorem with the history's mass and center:
//!
//! ```text
//! I_O = I_c + m (|r|² E − r rᵀ)
//! ```
//!
//! so the inertia's difference from the other tool's stays in the split's remainder.
//!
//! **Time.** A history's time is the flight's clock: 0 at launch.
//!
//! **What refuses a history.** A sample list that is empty, with times that don't rise, a value
//! that isn't finite, a negative thrust, a mass that isn't positive, a first sample after
//! launch, or a thrust that doesn't end at zero (the history would stop while the motor still
//! pushed); a thrust history on a flight with no motor lit by a known time; and either history
//! on a flight that splits, by a separation or an ejection, or moves or releases a part, since a
//! history is the whole stack's mass and thrust and gives no part's own.
//!
//! [adr-226]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0226-the-2026-10-10-review-the-gap-to-openrocket-by-cause.md
//! [adr-349]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0349-m2-3c6-in-four-the-public-nose-probe-first.md
//! [m2-3c11]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions-and-roadmap.md#m2-3c11

use serde::{Deserialize, Serialize};

use crate::error::SimError;

/// One quantity against time: samples at rising times, read by straight lines between them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Samples {
    times_s: Vec<f64>,
    values: Vec<f64>,
}

impl Samples {
    /// The samples, checked: as many values as times, at least one, every number finite, the
    /// times rising, the first at or before launch.
    fn new(history: &'static str, times_s: Vec<f64>, values: Vec<f64>) -> Result<Self, SimError> {
        let refuse = |why: &'static str| SimError::History { history, why };
        if times_s.len() != values.len() {
            return Err(refuse("it holds another number of values than of times"));
        }
        let (Some(&first), Some(_)) = (times_s.first(), values.first()) else {
            return Err(refuse("it holds no sample"));
        };
        if !times_s.iter().chain(&values).all(|x| x.is_finite()) {
            return Err(refuse("a time or a value is not a finite number"));
        }
        if times_s.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(refuse("its times don't rise from each sample to the next"));
        }
        if first > 0.0 {
            return Err(refuse(
                "its first sample comes after launch, so it doesn't cover the flight's start",
            ));
        }
        Ok(Self { times_s, values })
    }

    /// The value at `t_s`: the line through the samples either side, or the first or last
    /// sample's value outside them.
    fn at(&self, t_s: f64) -> f64 {
        let times = &self.times_s;
        let after = times.partition_point(|&t| t <= t_s);
        if after == 0 {
            return self.values[0];
        }
        if after == times.len() {
            return self.values[times.len() - 1];
        }
        let (t0, t1) = (times[after - 1], times[after]);
        let (y0, y1) = (self.values[after - 1], self.values[after]);
        // `a + w (b − a)` keeps the samples' own values at the ends of a piece.
        y0 + (y1 - y0) * ((t_s - t0) / (t1 - t0))
    }

    /// Whether the slope changes at sample `index`: an end, or a corner between two pieces.
    fn corner(&self, index: usize) -> bool {
        if index == 0 || index + 1 >= self.times_s.len() {
            return true;
        }
        let slope = |i: usize| {
            (self.values[i + 1] - self.values[i]) / (self.times_s[i + 1] - self.times_s[i])
        };
        slope(index - 1) != slope(index)
    }

    /// The times of the corners, the ends among them.
    fn corners_s(&self) -> impl Iterator<Item = f64> + '_ {
        (0..self.times_s.len())
            .filter(|&index| self.corner(index))
            .map(|index| self.times_s[index])
    }

    /// Whether the value changes anywhere inside `(a, b)`.
    fn varies_within(&self, (a, b): (f64, f64)) -> bool {
        let times = &self.times_s;
        // The pieces that overlap `(a, b)`: from the one holding `a` to the one holding `b`.
        let first = times.partition_point(|&t| t <= a).saturating_sub(1);
        let last = times.partition_point(|&t| t < b).min(times.len() - 1);
        (first..last).any(|i| self.values[i + 1] != self.values[i])
    }
}

/// A thrust against flight time, N, flown in place of the motors' (the [module][self] says what
/// stays HPR Sim's).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrustHistory {
    thrust: Samples,
}

impl ThrustHistory {
    /// The thrust `thrust_n[i]` at flight time `times_s[i]`.
    ///
    /// # Errors
    ///
    /// [`SimError::History`] if the lists differ in length or are empty, a number isn't finite,
    /// the times don't rise, the first comes after launch (`0`), a thrust is negative, or the
    /// last isn't zero.
    pub fn new(times_s: Vec<f64>, thrust_n: Vec<f64>) -> Result<Self, SimError> {
        let thrust = Samples::new("thrust", times_s, thrust_n)?;
        let refuse = |why: &'static str| SimError::History {
            history: "thrust",
            why,
        };
        if thrust.values.iter().any(|&f| f < 0.0) {
            return Err(refuse("a thrust is negative"));
        }
        if thrust.values.last().is_some_and(|&f| f != 0.0) {
            return Err(refuse(
                "its last thrust isn't zero, so it would stop while the motor still pushed",
            ));
        }
        Ok(Self { thrust })
    }

    /// The thrust at flight time `t_s`, N.
    #[must_use]
    pub fn thrust_n(&self, t_s: f64) -> f64 {
        self.thrust.at(t_s)
    }

    /// The sample times, s.
    #[must_use]
    pub fn times_s(&self) -> &[f64] {
        &self.thrust.times_s
    }
}

/// A mass and center of mass against flight time, flown in place of the design's (the
/// [module][self] says what stays HPR Sim's).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MassHistory {
    mass: Samples,
    cg: Samples,
}

impl MassHistory {
    /// The mass `mass_kg[i]` and center of mass `cg_aft_of_nose_m[i]`, m aft of the nose tip, at
    /// flight time `times_s[i]`.
    ///
    /// # Errors
    ///
    /// [`SimError::History`] if the lists differ in length or are empty, a number isn't finite,
    /// the times don't rise, the first comes after launch (`0`), or a mass isn't positive.
    pub fn new(
        times_s: Vec<f64>,
        mass_kg: Vec<f64>,
        cg_aft_of_nose_m: Vec<f64>,
    ) -> Result<Self, SimError> {
        let mass = Samples::new("mass", times_s.clone(), mass_kg)?;
        let cg = Samples::new("mass", times_s, cg_aft_of_nose_m)?;
        if mass.values.iter().any(|&m| m <= 0.0) {
            return Err(SimError::History {
                history: "mass",
                why: "a mass isn't positive",
            });
        }
        Ok(Self { mass, cg })
    }

    /// The mass at flight time `t_s`, kg.
    #[must_use]
    pub fn mass_kg(&self, t_s: f64) -> f64 {
        self.mass.at(t_s)
    }

    /// The center of mass at flight time `t_s`, m aft of the nose tip.
    #[must_use]
    pub fn cg_aft_of_nose_m(&self, t_s: f64) -> f64 {
        self.cg.at(t_s)
    }

    /// The sample times, s.
    #[must_use]
    pub fn times_s(&self) -> &[f64] {
        &self.mass.times_s
    }

    /// Whether the mass or the center of mass changes inside `window`.
    pub(crate) fn varies_within(&self, window: (f64, f64)) -> bool {
        self.mass.varies_within(window) || self.cg.varies_within(window)
    }
}

/// The histories a flight takes in place of its own, and where a thrust history acts.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Histories {
    pub(crate) thrust: Option<ThrustHistory>,
    /// The mean of the lit motors' nozzle exits in body axes, m: where the thrust acts.
    pub(crate) nozzle_m: hpr_core::DVec3,
    pub(crate) mass: Option<MassHistory>,
}

impl Histories {
    /// The times the integration stops at: every corner of either history, sorted, after `0`.
    pub(crate) fn corners_s(&self) -> Vec<f64> {
        let mut times: Vec<f64> = self
            .thrust
            .iter()
            .flat_map(|h| h.thrust.corners_s())
            .chain(
                self.mass
                    .iter()
                    .flat_map(|h| h.mass.corners_s().chain(h.cg.corners_s())),
            )
            .filter(|t| *t > 0.0)
            .collect();
        times.sort_by(f64::total_cmp);
        times.dedup();
        times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refusal(error: SimError) -> (&'static str, &'static str) {
        match error {
            SimError::History { history, why } => (history, why),
            other => panic!("not a history's refusal: {other}"),
        }
    }

    #[test]
    fn a_history_reads_the_line_between_samples_and_holds_its_ends() {
        let h = ThrustHistory::new(vec![0.0, 0.5, 2.0], vec![10.0, 40.0, 0.0]).unwrap();
        assert_eq!(h.thrust_n(-1.0), 10.0);
        assert_eq!(h.thrust_n(0.0), 10.0);
        assert_eq!(h.thrust_n(0.5), 40.0);
        assert_eq!(h.thrust_n(2.0), 0.0);
        assert_eq!(h.thrust_n(9.0), 0.0);
        // 10 + 30 × 0.25 / 0.5, and 40 − 40 × 0.75 / 1.5.
        assert!((h.thrust_n(0.25) - 25.0).abs() < 1e-12);
        assert!((h.thrust_n(1.25) - 20.0).abs() < 1e-12);
    }

    #[test]
    fn corners_leave_out_samples_on_a_straight_line() {
        let h = ThrustHistory::new(
            vec![-0.1, 0.0, 1.0, 2.0, 3.0, 4.0],
            vec![0.0, 0.0, 5.0, 10.0, 0.0, 0.0],
        )
        .unwrap();
        // 1.0 lies on the line from 0 to 2, and the ends always count; -0.1 is before launch.
        let histories = Histories {
            thrust: Some(h),
            ..Histories::default()
        };
        assert_eq!(histories.corners_s(), vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn a_mass_history_varies_only_where_its_samples_change() {
        let h = MassHistory::new(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![2.0, 1.5, 1.5, 1.5],
            vec![0.5, 0.45, 0.45, 0.45],
        )
        .unwrap();
        assert!(h.varies_within((0.2, 0.3)));
        assert!(h.varies_within((0.9, 1.1)));
        assert!(!h.varies_within((1.0, 2.5)));
        assert!(!h.varies_within((2.5, 9.0)));
        assert!((h.mass_kg(0.5) - 1.75).abs() < 1e-15);
        assert!((h.cg_aft_of_nose_m(0.5) - 0.475).abs() < 1e-15);
    }

    #[test]
    fn bad_samples_are_refused_with_why() {
        let thrust = |t: Vec<f64>, f: Vec<f64>| refusal(ThrustHistory::new(t, f).unwrap_err());
        assert_eq!(
            thrust(vec![0.0, 1.0], vec![1.0]).1,
            "it holds another number of values than of times"
        );
        assert_eq!(thrust(vec![], vec![]).1, "it holds no sample");
        assert_eq!(
            thrust(vec![0.0, f64::NAN], vec![1.0, 0.0]).1,
            "a time or a value is not a finite number"
        );
        assert_eq!(
            thrust(vec![0.0, 1.0, 1.0], vec![1.0, 1.0, 0.0]).1,
            "its times don't rise from each sample to the next"
        );
        assert_eq!(
            thrust(vec![0.1, 1.0], vec![1.0, 0.0]).1,
            "its first sample comes after launch, so it doesn't cover the flight's start"
        );
        assert_eq!(
            thrust(vec![0.0, 1.0], vec![-1.0, 0.0]).1,
            "a thrust is negative"
        );
        assert_eq!(
            thrust(vec![0.0, 1.0], vec![1.0, 0.5]),
            (
                "thrust",
                "its last thrust isn't zero, so it would stop while the motor still pushed"
            )
        );
        let mass =
            refusal(MassHistory::new(vec![0.0, 1.0], vec![1.0, 0.0], vec![0.5, 0.5]).unwrap_err());
        assert_eq!(mass, ("mass", "a mass isn't positive"));
    }
}
