//! AerospaceNU's flight computer board's CSV.
//!
//! Scaffold: the reader is written in milestone M7.1c4 (ADR-258).

use crate::error::LogError;
use crate::log::FlightLog;

/// The format's name in errors.
const FORMAT: &str = "AerospaceNU flight computer CSV";

/// The barometric altitude's step, m: the board writes its pressure in whole pascals over
/// 101,325, so one pascal, 0.083 m at sea level.
pub const ALTITUDE_STEP_M: f64 = 0.083;

/// Whether the text is the board's CSV: a header row whose first cell is empty and that names
/// `packetType` or a `timestamp_ms` or `timestamp_s` column.
pub fn recognizes(text: &str) -> bool {
    let first = text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("");
    first.starts_with(',')
        && first
            .split(',')
            .any(|name| matches!(name.trim(), "packetType" | "timestamp_ms" | "timestamp_s"))
}

/// Reads the board's CSV.
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
