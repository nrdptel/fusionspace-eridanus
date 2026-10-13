//! AerospaceNU's SillyGoose flight computer's logs, as its board writes them: tab-separated text under a `timestampMs` header.
//!
//! Not written yet: [`read`] refuses every file.

use crate::error::LogError;
use crate::log::FlightLog;

/// The format's name in messages.
const FORMAT: &str = "SillyGoose text";

/// The step between two altitudes the format can tell apart near the ground, m
/// ([`crate::log::LogFormat::altitude_resolution_m`]).
pub(crate) const ALTITUDE_STEP_M: f64 = 1.0;

/// Whether a file's text is in this layout.
pub fn recognizes(text: &str) -> bool {
    let _ = text;
    false
}

/// Reads a log in this layout.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] for now: the reader isn't written yet.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let _ = text;
    Err(LogError::NotThisFormat {
        format: FORMAT,
        message: "not read yet".to_owned(),
    })
}
