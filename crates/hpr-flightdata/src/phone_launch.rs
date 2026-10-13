//! A phone app's `launch….csv`.
//!
//! Scaffold: the reader is written in milestone M7.1c4 (ADR-258).

use crate::error::LogError;
use crate::log::FlightLog;

/// The format's name in errors.
const FORMAT: &str = "phone app launch CSV";

/// The barometric altitude's step, m. Scaffold value, set by the reader.
pub const ALTITUDE_STEP_M: f64 = 0.01;

/// Whether the text is the app's CSV: a header row opening `Seconds,Timestamp,AltitudeMSLGPS`.
pub fn recognizes(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with("Seconds,Timestamp,AltitudeMSLGPS"))
}

/// Reads the app's CSV.
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
