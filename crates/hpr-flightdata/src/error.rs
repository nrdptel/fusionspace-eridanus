//! Why a flight log couldn't be read.

use thiserror::Error;

/// An error reading a flight log: the text isn't the format it should be, or its data can't be a
/// flight's record.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum LogError {
    /// A line of the file is malformed.
    #[error("{format} line {line}: {message}")]
    Syntax {
        /// The file format, such as `.pf2`.
        format: &'static str,
        /// The 1-based line number.
        line: usize,
        /// What is wrong.
        message: String,
    },
    /// The file isn't in the format asked for.
    #[error("not read as {format}: {message}")]
    NotThisFormat {
        /// The format the file was read as.
        format: &'static str,
        /// Why it isn't one.
        message: String,
    },
    /// The file is in none of the formats HPR Sim reads.
    #[error("{message}")]
    Unrecognized {
        /// What the file looks like, and the formats HPR Sim reads.
        message: String,
    },
    /// The file has no data rows.
    #[error("{format}: the file has no data rows")]
    NoData {
        /// The file format.
        format: &'static str,
    },
    /// A binary file's bytes aren't laid out as its format's, such as a frame whose checksum fails.
    #[error("{format} byte {offset}: {message}")]
    Binary {
        /// The file format.
        format: &'static str,
        /// The 0-based offset of the byte where the reader found the fault.
        offset: usize,
        /// What is wrong.
        message: String,
    },
    /// A zip archive can't be unpacked, or holds more than the bounds allow
    /// ([`archive`](crate::archive)).
    #[error("zip archive: {message}")]
    Archive {
        /// What is wrong.
        message: String,
    },
    /// An `.xlsx` workbook's parts can't be read as one, or its sheets pass the bounds
    /// ([`workbook`](crate::workbook)).
    #[error("Excel workbook: {message}")]
    Workbook {
        /// What is wrong.
        message: String,
    },
    /// The file states its values in a unit the reader doesn't know, such as a PerfectFlite file
    /// whose stated apogee carries no foot mark.
    #[error("{format} line {line}: {message}")]
    Unit {
        /// The file format.
        format: &'static str,
        /// The 1-based line number.
        line: usize,
        /// What is wrong.
        message: String,
    },
    /// A column mapping the caller gave can't read the file: it names no column or too many,
    /// its clock isn't one of its columns or isn't in a unit of time, or a column's unit isn't
    /// one the reader knows or doesn't measure what its quantity does
    /// ([`generic::read_with`](crate::generic::read_with)).
    #[error("column mapping: {message}")]
    Mapping {
        /// What is wrong.
        message: String,
    },
    /// A file of binary records needs a layout the caller states, as a console capture does
    /// ([`capture`](crate::capture)), or the layout the caller gave can't read it: no field or too
    /// many, a name the table can't hold, names that state no clock, or a table past its bound
    /// ([`records`](crate::records)).
    #[error("record layout: {message}")]
    Layout {
        /// What is wrong.
        message: String,
    },
}
