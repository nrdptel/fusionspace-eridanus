//! Shared pieces of the text formats: parse warnings and number reading.

use serde::{Deserialize, Serialize};

use crate::error::MotorError;

/// Something a lenient reader accepted but flagged, such as a missing entry separator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseWarning {
    /// The 1-based line number the warning is about.
    pub line: usize,
    /// Whether data was lost.
    pub kind: WarningKind,
    /// What was accepted, and how it was read.
    pub message: String,
}

/// How serious a [`ParseWarning`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WarningKind {
    /// A whole motor entry had an error and was left out of the result.
    Skipped,
    /// A value was left out, such as an unknown attribute or a partial mass column.
    Dropped,
    /// Something unusual was read as it stands, such as a curve that doesn't end at zero thrust.
    Unusual,
}

impl ParseWarning {
    pub(crate) fn new(line: usize, kind: WarningKind, message: impl Into<String>) -> Self {
        Self {
            line,
            kind,
            message: message.into(),
        }
    }
}

/// A parsed value and the warnings its reader raised.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parsed<T> {
    /// The value read.
    pub value: T,
    /// Warnings, in file order.
    pub warnings: Vec<ParseWarning>,
}

/// Reads a finite `f64`. Rust's parser also accepts `inf` and `NaN`, which no motor file means.
pub(crate) fn finite(text: &str, what: &'static str) -> Result<f64, MotorError> {
    match text.trim().parse::<f64>() {
        Ok(value) if value.is_finite() => Ok(value),
        _ => Err(MotorError::Parse {
            what,
            text: text.to_owned(),
        }),
    }
}

/// Checks a value the writers will print: finite, and non-negative when `non_negative`.
pub(crate) fn check_writable(
    value: f64,
    what: &'static str,
    non_negative: bool,
) -> Result<(), MotorError> {
    if value.is_finite() && !(non_negative && value < 0.0) {
        Ok(())
    } else {
        Err(MotorError::Domain { what, value })
    }
}

/// What follows something a motor file's reader or converter left out that no number HPR Sim
/// works out depends on: an attribute or element it doesn't read, a flag it doesn't use, a
/// delay a file lists (a file's delays are never flown: a flight's delay is chosen), a column it
/// works out itself (#952). The `.ork` reader says the same in the same words.
pub(crate) const NOTHING_DEPENDS: &str = "no number HPR Sim works out depends on it";

/// [`NOTHING_DEPENDS`] for `count` things: "on it" for one, "on them" for any other count.
pub(crate) fn nothing_depends(count: usize) -> &'static str {
    if count == 1 {
        NOTHING_DEPENDS
    } else {
        "no number HPR Sim works out depends on them"
    }
}

/// What HPR Sim does instead of reading a motor file's mass or center-of-gravity columns, which a
/// warning that leaves them out ends with: it works them out from the curve and the motor's
/// masses (`crate::mass`), so no number depends on the columns (#952).
pub(crate) const WORKED_OUT: &str =
    "HPR Sim works a motor's mass and center of gravity out from its curve and masses itself";

/// What follows a motor entry left out: none of its numbers are read, so a program that wanted
/// that motor finds no curve for it here (#952).
pub(crate) const NO_CURVE: &str = "this file gives no curve for its motor";
