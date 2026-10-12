//! One entry point for every format HPR Sim reads: give [`read`] a file's bytes, and it
//! recognizes the layout from the content, never the file's name, and reads it into the record
//! every reader fills ([`FlightLog`]).
//!
//! The layouts it recognizes, by the first line that isn't blank or a `# ` comment:
//!
//! | Format | First line |
//! |---|---|
//! | [PerfectFlite `.pf2`](crate::perfectflite) | names PerfectFlite, such as `PerfectFlite Pnut` |
//! | [PerfectFlite CSV and text](crate::perfectflite::read_export) | `Time`, `Altitude` and more of an export's names, by commas or tabs |
//! | [AltOS CSV](crate::altos) | starts `#version,`, or `version,serial,` after `# ` comments |
//! | [Raven FIP CSV](crate::raven) | holds the Raven's `Axial Accel (Gs)` channel |
//!
//! Text is read as UTF-8 with any byte-order mark dropped; a byte that isn't UTF-8 becomes U+FFFD,
//! as a `.pf2`'s comments may be in another encoding and no reader uses them.
//!
//! ```
//! use hpr_flightdata::{import, log::LogFormat};
//!
//! let text = "Time,Altitude,Velocity\n0.00,0,0\n0.05,1,8\n0.10,3,30\n";
//! assert_eq!(import::detect(text.as_bytes()), Some(LogFormat::PerfectFliteCsv));
//! let log = import::read(text.as_bytes())?;
//! assert_eq!(log.channels.len(), 2); // the altitude and the velocity; the time is their clock
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use std::borrow::Cow;

use crate::error::LogError;
use crate::log::{FlightLog, LogFormat};
use crate::{altos, perfectflite, raven};

/// The formats [`read`] recognizes, in the order it tries them.
pub const FORMATS: [LogFormat; 5] = [
    LogFormat::PerfectFlitePf2,
    LogFormat::AltosCsv,
    LogFormat::RavenFipCsv,
    LogFormat::PerfectFliteCsv,
    LogFormat::PerfectFliteText,
];

/// A file's bytes as text: UTF-8, any byte-order mark dropped, a byte that isn't UTF-8 replaced.
pub fn decode(bytes: &[u8]) -> Cow<'_, str> {
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    String::from_utf8_lossy(bytes)
}

/// The format of a log, recognized from its first line that isn't blank; `None` for a layout HPR
/// Sim doesn't read.
pub fn detect(bytes: &[u8]) -> Option<LogFormat> {
    detect_text(&decode(bytes))
}

fn detect_text(text: &str) -> Option<LogFormat> {
    // A `# ` line is a comment, as Debrief's AltOS sample opens with; `#version,` is AltOS's header.
    let first = text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && *line != "#" && !line.starts_with("# "))?;
    let lower = first.to_ascii_lowercase();
    if lower.contains("perfectflite") {
        Some(LogFormat::PerfectFlitePf2)
    } else if first.starts_with("#version,") || first.starts_with("version,serial,") {
        Some(LogFormat::AltosCsv)
    } else if first.contains("Axial Accel (Gs)") {
        Some(LogFormat::RavenFipCsv)
    } else {
        match perfectflite::export_separator(first)? {
            ',' => Some(LogFormat::PerfectFliteCsv),
            _ => Some(LogFormat::PerfectFliteText),
        }
    }
}

/// Reads a flight log in any format HPR Sim reads.
///
/// # Errors
///
/// [`LogError::Unrecognized`] for a layout HPR Sim doesn't read, naming those it does; otherwise
/// whatever the format's reader refuses.
pub fn read(bytes: &[u8]) -> Result<FlightLog, LogError> {
    let text = decode(bytes);
    match detect_text(&text) {
        Some(LogFormat::PerfectFlitePf2) => perfectflite::read(&text),
        Some(LogFormat::PerfectFliteCsv | LogFormat::PerfectFliteText) => {
            perfectflite::read_export(&text)
        }
        Some(LogFormat::AltosCsv) => altos::read(&text),
        Some(LogFormat::RavenFipCsv) => raven::read(&text),
        None => {
            let known: Vec<&str> = FORMATS.iter().map(|format| format.name()).collect();
            Err(LogError::Unrecognized {
                message: format!(
                    "the file is in none of the layouts HPR Sim reads so far: {}",
                    known.join(", ")
                ),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_format_is_recognized_by_its_first_line() {
        let cases = [
            ("PerfectFlite Pnut\n", Some(LogFormat::PerfectFlitePf2)),
            (
                "\u{feff}\n\n#version,serial,flight,time\n",
                Some(LogFormat::AltosCsv),
            ),
            (
                "# Altus Metrum flight log\n# serial 1\nversion,serial,flight,time\n",
                Some(LogFormat::AltosCsv),
            ),
            ("#Component=Raven\n", None),
            (
                "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA\n",
                Some(LogFormat::RavenFipCsv),
            ),
            (
                "Time,LiftOff,Axial Accel (Gs),Baro (Atm)\n",
                Some(LogFormat::RavenFipCsv),
            ),
            (
                "Time, Altitude, Velocity\n",
                Some(LogFormat::PerfectFliteCsv),
            ),
            (
                "Time\tAltitude\tTemp.\tVoltage\n",
                Some(LogFormat::PerfectFliteText),
            ),
            // A team's own export in feet: the generic reader's, M7.1b.
            ("time_s,altitude_ft,temperature,voltage\n", None),
            ("Time,Pressure\n", None),
            ("time,altitude\n", None),
            ("Time,Altitude\n", None),
            // A Missile Works RRC3's: M7.1b.
            (
                "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages\n",
                None,
            ),
            ("", None),
        ];
        for (text, format) in cases {
            assert_eq!(detect(text.as_bytes()), format, "{text:?}");
        }
    }

    #[test]
    fn a_layout_no_reader_knows_is_refused_naming_those_that_exist() {
        let error = read(b"Time,Pressure\n0,101325\n").unwrap_err();
        let LogError::Unrecognized { message } = &error else {
            panic!("refused for another reason: {error:?}");
        };
        for format in FORMATS {
            assert!(message.contains(format.name()), "{message}");
        }
    }
}
