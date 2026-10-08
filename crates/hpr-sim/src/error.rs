//! Errors from setting up or running a flight.

use hpr_aero::AeroError;
use hpr_atmos::AtmosError;
use hpr_core::CoreError;
use hpr_design::{DesignError, Finding, Severity};
use thiserror::Error;

use crate::integrator::{IntegrationError, SettingsError};

/// An error setting up or running a flight.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SimError {
    /// An input outside its domain, such as a negative rail length.
    #[error("{what} is outside its domain: {value}")]
    Domain {
        /// What the value is.
        what: &'static str,
        /// The offending value.
        value: f64,
    },
    /// A model doesn't cover the input, such as a flutter panel of an elliptical fin.
    #[error("unsupported: {what}")]
    Unsupported {
        /// What isn't covered.
        what: &'static str,
    },
    /// A separation or ejection that the design can't make, with the component (or stage) id it
    /// names or runs through.
    #[error("{what}: `{component}`")]
    Parting {
        /// Why it can't be made.
        what: &'static str,
        /// The component or stage.
        component: String,
    },
    /// A mass shift that the design can't make, with the component id it names.
    #[error("{what}: `{component}`")]
    Shift {
        /// Why it can't be made.
        what: &'static str,
        /// The component.
        component: String,
    },
    /// A mass release that the design can't make, with the component id it names.
    #[error("{what}: `{component}`")]
    MassRelease {
        /// Why it can't be made.
        what: &'static str,
        /// The component.
        component: String,
    },
    /// The design's checks found errors, and the settings don't accept them: every finding,
    /// errors and warnings, in the checks' order. The message counts and names only the errors.
    #[error(
        "the design has {} error finding(s); the first is {}",
        error_count(.0),
        first_error(.0)
    )]
    DesignChecks(Vec<Finding>),
    /// The design can't be assembled.
    #[error(transparent)]
    Design(#[from] DesignError),
    /// The aerodynamic models refused the design or a flow condition (for example Mach 5 or
    /// faster, past the normal force's and the drag buildup's range).
    #[error(transparent)]
    Aero(#[from] AeroError),
    /// The atmosphere or wind model refused a height.
    #[error(transparent)]
    Atmosphere(#[from] AtmosError),
    /// A geodesy, gravity or numerics error.
    #[error(transparent)]
    Core(#[from] CoreError),
    /// Bad integrator settings or initial state.
    #[error(transparent)]
    Settings(#[from] SettingsError),
    /// The integration failed for a reason other than the step limit.
    #[error("the integration failed")]
    Integration(#[source] Box<IntegrationError<SimError>>),
}

/// How many findings are errors, for [`SimError::DesignChecks`]'s message.
fn error_count(findings: &[Finding]) -> usize {
    findings
        .iter()
        .filter(|finding| finding.severity() == Severity::Error)
        .count()
}

/// The first finding that is an error, for [`SimError::DesignChecks`]'s message.
fn first_error(findings: &[Finding]) -> String {
    findings
        .iter()
        .find(|finding| finding.severity() == Severity::Error)
        .map_or_else(|| "missing".to_owned(), |finding| format!("{finding:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// #46: a warning ahead of the error is neither counted nor named as the error.
    #[test]
    fn design_checks_message_counts_and_names_only_errors() {
        let warning = Finding::MotorPastMountTop {
            configuration: "c".to_owned(),
            mount: "mount".to_owned(),
            excess_m: 0.01,
        };
        let error = Finding::MotorWiderThanMount {
            configuration: "c".to_owned(),
            mount: "mount".to_owned(),
            motor_diameter_m: 0.038,
            mount_inner_diameter_m: 0.029,
        };
        let message = SimError::DesignChecks(vec![warning, error.clone()]).to_string();
        assert_eq!(
            message,
            format!("the design has 1 error finding(s); the first is {error:?}")
        );
    }
}
