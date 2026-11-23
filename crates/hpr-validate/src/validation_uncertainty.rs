//! A validation uncertainty for each real flight, after ASME V&V 20-2009 (M1.14b, ADR-351
//! decision 2 and ADR-373).
//!
//! V&V 20 writes a comparison error `E = S − D`, the simulation's value less the data's, as the
//! model's own error and three others: `E = δ_model + δ_num + δ_input − δ_D`. Each of the three
//! others has a standard uncertainty, and while they are independent the validation uncertainty
//! is their root sum of squares, `u_val = √(u_num² + u_input² + u_D²)`; the model's error then
//! lies in `E ± k·u_val`, `k = 2` for about 95% when the errors are near Gaussian. Read in
//! Eça, Dowding and Roache, "On the interpretation and scope of the V&V 20 standard", ASME V&V
//! Symposium 2020, SAND2020-3788C (OSTI 1774746), pp. 3 to 5, eq. 10; the standard's own text
//! is paywalled and was not read (`docs/research/validation-uncertainty-vv20.md`).
//!
//! The numerical part follows Roache's grid convergence index on two runs, the second with the
//! adaptive stepper's tolerances divided by 32 so that its steps are about half as long:
//! `GCI = F_s·|ε|/(r^p − 1)` with the safety factor `F_s = 3` that two runs take, the order
//! `p = 4` of the stepper's dense output, which places the apogee, and the refinement ratio `r`
//! measured as the second run's accepted steps over the first's, not assumed to be 2. The GCI is
//! a band of about 95%; it is divided by `k = 2` for a standard uncertainty, the divisor NIST's
//! summary gives for a Gaussian about the finer run (NISTIR 8298, 2020, §4, p. 17; it gives 1.1
//! to 1.15 for a shifted one, and neither was checked against the standard). The input and data
//! parts come in M1.14b17 and M1.14b18.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::correction::Variant;
use crate::predicted_first::ComparedPrediction;
use crate::real_flight::{FLIGHTS, FlightRow, RealFlightError, Varied, fly_varied};

/// Roache's safety factor for a grid convergence index from two runs.
pub const SAFETY_FACTOR_TWO_RUNS: f64 = 3.0;

/// The order the apogee converges at: Dormand–Prince 5(4)'s dense output, which places events
/// to fourth order (Hairer, Nørsett and Wanner, *Solving Ordinary Differential Equations I*,
/// 2nd ed., §II.6). The lower of the stepper's two orders, so the larger index.
pub const APOGEE_ORDER: f64 = 4.0;

/// The coverage factor between a standard uncertainty and a band of about 95%: V&V 20's
/// worked example draws `E ± 2·u_val` (Eça, Dowding and Roache 2020, p. 4).
pub const COVERAGE_K: f64 = 2.0;

/// The numerical part `u_num`, m: the standard uncertainty of an apogee flown at `coarse_m` and
/// again on steps `refinement_ratio` times shorter at `fine_m`, as Roache's two-run grid
/// convergence index over [`COVERAGE_K`]: `3·|fine − coarse|/(r⁴ − 1)/2`. `None` unless the ratio
/// is a finite number above 1: the second run must be the finer.
#[must_use]
pub fn numerical_uncertainty_m(coarse_m: f64, fine_m: f64, refinement_ratio: f64) -> Option<f64> {
    if !(refinement_ratio.is_finite() && refinement_ratio > 1.0) {
        return None;
    }
    let index = SAFETY_FACTOR_TWO_RUNS * (fine_m - coarse_m).abs()
        / (refinement_ratio.powf(APOGEE_ORDER) - 1.0);
    Some(index / COVERAGE_K)
}

/// The refinement ratio of two runs of one flight: the finer run's accepted steps over the
/// coarser's, their mean steps' ratio. `None` without steps in the coarser run.
#[must_use]
pub fn refinement_ratio(coarse_steps: u64, fine_steps: u64) -> Option<f64> {
    // Step counts are far under 2^53 (a flight's step limit is in the millions), so each is an
    // exact f64.
    #[expect(clippy::cast_precision_loss, reason = "step counts are far under 2^53")]
    let ratio = (coarse_steps > 0).then(|| fine_steps as f64 / coarse_steps as f64);
    ratio
}

/// V&V 20's validation uncertainty, m: the root sum of squares of the numerical, input and data
/// parts, each a standard uncertainty (Eça, Dowding and Roache 2020, eq. 10). It holds while the
/// three errors are independent; where one source feeds both the simulation and the data, the
/// standard combines them otherwise.
#[must_use]
pub fn validation_uncertainty_m(numerical_m: f64, input_m: f64, data_m: f64) -> f64 {
    (numerical_m * numerical_m + input_m * input_m + data_m * data_m).sqrt()
}

/// One public real flight's comparison error and the numerical part of its validation
/// uncertainty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UncertaintyRow {
    /// The flight's id, as its row in the report gives it.
    pub id: String,
    /// The comparison error `E = S − D`, m: HPR Sim's apogee as its row gives it less the log's.
    pub comparison_error_m: f64,
    /// The apogee as the report flies it, m, as the log's altimeter would read it, unrounded.
    pub apogee_m: f64,
    /// The same flight with the time step halved, m ([`Variant::STEP_HALVED`]).
    pub apogee_step_halved_m: f64,
    /// The integrator's accepted steps over the whole flight as the report flies it.
    pub steps: u64,
    /// The same with the time step halved.
    pub steps_step_halved: u64,
    /// The numerical part `u_num`, m ([`numerical_uncertainty_m`] on the
    /// [`refinement_ratio`] of the two runs' steps).
    pub numerical_m: f64,
}

impl UncertaintyRow {
    /// The refinement ratio of its two runs ([`refinement_ratio`]).
    #[must_use]
    pub fn refinement_ratio(&self) -> Option<f64> {
        refinement_ratio(self.steps, self.steps_step_halved)
    }

    /// Whether the row holds together: its numerical part is its two runs', and everything is
    /// a finite number.
    #[must_use]
    pub fn holds(&self) -> bool {
        [
            self.comparison_error_m,
            self.apogee_m,
            self.apogee_step_halved_m,
            self.numerical_m,
        ]
        .iter()
        .all(|x| x.is_finite())
            && self.refinement_ratio().and_then(|ratio| {
                numerical_uncertainty_m(self.apogee_m, self.apogee_step_halved_m, ratio)
            }) == Some(self.numerical_m)
    }
}

/// Each public real flight's row: the flights compared first, in `flights`' order, then those
/// predicted first, in `predicted`'s. Each is flown twice more, as the report flies it and with
/// the time step halved.
///
/// # Errors
///
/// [`RealFlightError`] if a flight doesn't fly, a row has no flight, or the second run of a
/// flight took no more steps than the first.
pub fn rows(
    root: &Path,
    flights: &[FlightRow],
    predicted: &[ComparedPrediction],
) -> Result<Vec<UncertaintyRow>, RealFlightError> {
    let missing = |id: &str| RealFlightError::Input {
        flight: id.to_owned(),
        what: "a row with no flight to fly again".to_owned(),
    };
    let row = |id: &str, error_m: f64, coarse: Varied, fine: Varied| {
        let numerical_m = refinement_ratio(coarse.accepted_steps, fine.accepted_steps)
            .and_then(|ratio| numerical_uncertainty_m(coarse.apogee_m, fine.apogee_m, ratio))
            .ok_or_else(|| RealFlightError::Input {
                flight: id.to_owned(),
                what: format!(
                    "the run with the step halved took {} steps, the report's {}: not finer",
                    fine.accepted_steps, coarse.accepted_steps
                ),
            })?;
        Ok(UncertaintyRow {
            id: id.to_owned(),
            comparison_error_m: error_m,
            apogee_m: coarse.apogee_m,
            apogee_step_halved_m: fine.apogee_m,
            steps: coarse.accepted_steps,
            steps_step_halved: fine.accepted_steps,
            numerical_m,
        })
    };
    let mut out = Vec::with_capacity(flights.len() + predicted.len());
    for compared in flights {
        let flight = FLIGHTS
            .iter()
            .find(|f| f.id == compared.id)
            .ok_or_else(|| missing(&compared.id))?;
        let coarse = fly_varied(root, flight, Variant::TIP_OFF)?;
        let fine = fly_varied(root, flight, Variant::STEP_HALVED)?;
        out.push(row(
            &compared.id,
            compared.hpr_apogee_m - compared.log_apogee_m,
            coarse,
            fine,
        )?);
    }
    let mut coarse = crate::public_predictions::varied_apogees(root, Variant::TIP_OFF)?;
    let mut fine = crate::public_predictions::varied_apogees(root, Variant::STEP_HALVED)?;
    let take = |list: &mut Vec<crate::public_predictions::VariedApogee>, id: &str| {
        let at = list
            .iter()
            .position(|(each, _)| each == id)
            .ok_or_else(|| missing(id))?;
        list.swap_remove(at).1
    };
    for compared in predicted {
        let coarse_run = take(&mut coarse, &compared.id)?;
        let fine_run = take(&mut fine, &compared.id)?;
        out.push(row(
            &compared.id,
            compared.hpr_apogee_m - compared.log_apogee_m,
            coarse_run,
            fine_run,
        )?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The worked case, by hand. Validation: `u_num` = 1 m, `u_input` = 20 m, `u_D` = 10 m give
    /// `u_val` = √(1 + 400 + 100) = √501 = 22.3830 m. Numerical: an apogee of 1000.0 m flown
    /// again at 1000.3 m on 1000 steps for the first run's 500 (r = 2) differs by 0.3 m; the index
    /// is 3 × 0.3 / (16 − 1) = 0.06 m, and over k = 2 the standard uncertainty is 0.03 m. On
    /// 1414 steps for 1000 (r ≈ √2, r⁴ ≈ 4) the index is 3 × 0.3 / 3 ≈ 0.3 m, five times as much.
    #[test]
    fn the_combination_on_a_case_worked_by_hand() {
        let u_val = validation_uncertainty_m(1.0, 20.0, 10.0);
        assert!((u_val - 22.383_029).abs() < 1e-6, "{u_val}");
        assert!((u_val * u_val - 501.0).abs() < 1e-9);
        let halved = refinement_ratio(500, 1000).expect("500 steps is some");
        assert_eq!(halved, 2.0);
        let u_num = numerical_uncertainty_m(1000.0, 1000.3, halved).expect("r = 2 is finer");
        assert!((u_num - 0.03).abs() < 1e-12, "{u_num}");
        let root_two = refinement_ratio(1000, 1414).expect("1000 steps is some");
        let wide = numerical_uncertainty_m(1000.0, 1000.3, root_two).expect("r > 1 is finer");
        assert!((wide - 0.15).abs() < 2e-4, "{wide}");
        // Which apogee is the coarse one doesn't matter; no difference, no part.
        assert_eq!(Some(u_num), numerical_uncertainty_m(1000.3, 1000.0, 2.0));
        assert_eq!(numerical_uncertainty_m(512.0, 512.0, 2.0), Some(0.0));
        // A second run no finer than the first, or no steps, gives no index.
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, 1.0), None);
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, 0.5), None);
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, f64::NAN), None);
        assert_eq!(refinement_ratio(0, 1000), None);
        // A part alone is itself, and the parts add in squares, not in sums.
        assert_eq!(validation_uncertainty_m(0.0, 3.0, 0.0), 3.0);
        assert_eq!(validation_uncertainty_m(3.0, 4.0, 0.0), 5.0);
    }

    /// A row holds only with its own numerical part.
    #[test]
    fn a_row_holds_with_its_own_numerical_part() {
        let own = numerical_uncertainty_m(1000.0, 1000.3, 2.0).expect("r = 2 is finer");
        let mut row = UncertaintyRow {
            id: "a".to_owned(),
            comparison_error_m: -12.0,
            apogee_m: 1000.0,
            apogee_step_halved_m: 1000.3,
            steps: 500,
            steps_step_halved: 1000,
            numerical_m: 0.03,
        };
        assert!(!row.holds(), "0.03 isn't the float the formula gives");
        row.numerical_m = own;
        assert!(row.holds());
        row.numerical_m *= 2.0;
        assert!(!row.holds());
        row.numerical_m = own;
        row.steps_step_halved = 900;
        assert!(!row.holds(), "the part follows the steps' ratio");
        row.steps_step_halved = 500;
        assert!(!row.holds(), "a second run no finer holds no part");
        row.steps_step_halved = 1000;
        row.comparison_error_m = f64::NAN;
        assert!(!row.holds());
    }
}
