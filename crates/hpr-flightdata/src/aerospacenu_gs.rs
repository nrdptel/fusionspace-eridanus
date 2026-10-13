//! AerospaceNU's ground station's text: the flight computer board's downlink, parsed.
//!
//! Scaffold: the reader is written in milestone M7.1c4 (ADR-258).

use crate::error::LogError;
use crate::log::FlightLog;

/// The format's name in errors.
const FORMAT: &str = "AerospaceNU ground station text";

/// The barometric altitude's step, m: the board's pressure, sent as a 32-bit float near 1 atm,
/// steps by about 0.012 Pa, finer than its one-pascal reading, 0.083 m at sea level.
pub const ALTITUDE_STEP_M: f64 = 0.083;

/// Whether the text is the ground station's: its first line that isn't blank opens `RUN START`.
pub fn recognizes(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with("RUN START"))
}

/// Reads the ground station's text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] until the reader is written.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let _ = text;
    Err(LogError::NotThisFormat {
        format: FORMAT,
        message: "not read yet".to_owned(),
    })
}
