//! Stub, filled by M7.1c1.

use crate::error::LogError;
use crate::log::FlightLog;

/// Whether the text is in this reader's layout.
pub(crate) fn recognizes(_text: &str) -> bool {
    false
}

/// Reads the layout.
///
/// # Errors
///
/// Not written yet.
pub fn read(_text: &str) -> Result<FlightLog, LogError> {
    Err(LogError::NoData { format: "stub" })
}
