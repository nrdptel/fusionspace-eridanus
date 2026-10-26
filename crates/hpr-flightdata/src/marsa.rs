//! A MARSA altimeter's flight download: its settings, then a row a sample of the time, the
//! barometric altitude and the acceleration, as MARSA Systems' software saves it to a `.dat` file,
//! or as a spreadsheet re-saves that file.
//!
//! MARSA Systems' MARSA4 and MARSA54 flight computers share their firmware and their PC software,
//! MARSA_Connect (MARSA's home page, 2011), which downloads a flight from the altimeter's memory
//! and saves it to the computer (*MARSA54L Flight Computer User Manual*, V1.4, 2014, p. 18). No
//! document describes the file, so this reader takes the layout from the two sample files, and
//! the sample logs' survey (`cargo xtask logs`) holds each unit to a check ([ADR-274][adr-274]).
//! An invented download, cut short:
//!
//! ```text
//! <BARO_ZERO 0> 790
//! <ACCEL_ZERO 0> 498
//! <LAUNCH TIME 0> 12000
//! <FIRMWARE VER> 26
//! <MARSA VERSION> 2
//! <CHAN 1>
//! <CHAN APOGEE OPTION> 1
//! <CHAN ALTITUDE SETTING> 500
//! -0.2,-58.1,-0.08
//! -0.1,-58.1,-0.08
//! 0.0,-58.1,8.20
//! 0.1,-29.0,8.65
//! ```
//!
//! **The settings.** Each line before the samples is a setting, `<NAME> value`, or a section's
//! heading, `<NAME>` alone, such as `<CHAN 1>`, which opens a pyro channel's settings. Each setting
//! is a [`Figure`] of the record's [`Stated::figures`], named as the file names it, after its
//! section's heading where it has one (`CHAN 1, CHAN APOGEE OPTION`); its value as written, and a
//! number of [`Quantity::Number`] where it is one, as no document gives the settings' units. The
//! MARSA4 manual describes two: `Baro0` and `Accel0`, the file's `BARO_ZERO` and `ACCEL_ZERO`, are
//! "the A/D values of the ground barometric and sensor readings" (*MARSA4 User Manual*, Version
//! 1.0, 2009, p. 17). `FIRMWARE VER` is the record's [`FlightLog::firmware`]. A setting named twice
//! in one section is refused, as no reader could tell which value is the altimeter's.
//!
//! **The samples.** Each line after the settings is a sample: three numbers by commas, which no
//! line names. They are read as the time, s, from the logger's zero at liftoff; the barometric
//! altitude, in feet, above the logger's own zero; and the acceleration, in units of standard
//! gravity, 9.806 65 m/s², its reading at rest taken out. MARSA's manuals give the altitude in
//! feet: the altimeter's display reads "898 ft" in the MARSA54L manual's example (p. 12), and its
//! altitude events are set "in 100 ft increments". The software plots "Baro altitude and
//! acceleration" (MARSA4 manual, p. 18), the altitude "based on the barometric pressure according
//! to the standard atmospheric model" (p. 16). The survey holds the units to the files: each
//! file's apogee against an AltOS flight computer's in the same rocket, the feet's check, and the
//! barometric climb over the first seconds after liftoff against the acceleration integrated
//! twice, the g's.
//!
//! - The altitude is [`Quantity::AltitudeAboveGround`] and the record's primary track. It moves
//!   by its barometer's count, not by the tenth of a foot it is written in, and it needn't read 0
//!   on the pad: the logger's zero is its own ([`FlightLog::altitude_m`]), and in both sample
//!   files the pad reads several counts below it. The survey reports it.
//! - The acceleration is [`Quantity::Acceleration`]: it reads about 0 at rest, not 1 g, and so
//!   it is the rocket's acceleration along its axis while the rocket points straight up, as
//!   AltOS's is ([`crate::altos`]).
//!
//! A sample's time must be later than the one before, or the file is refused at its line: in
//! both sample files the steps are a tenth of a second early in the flight and grow later. A last
//! line the file ends in with no line end, which isn't a whole sample, is dropped with a note: a
//! download cut off as it was written.
//!
//! **A spreadsheet's re-save.** One sample file is a download a spreadsheet re-saved as CSV: each
//! setting's line ends in two empty cells, `<BARO_ZERO 0> 790,,`, and the numbers are rewritten,
//! `8.2` for `8.20`. Empty cells at a line's end are passed over, so both read the same.
//!
//! **Bounds.** A file may hold at most [`MAX_SETTINGS`] settings and headings, a name
//! [`MAX_NAME_BYTES`] bytes and a value [`MAX_VALUE_CHARS`] characters; past them it is refused,
//! so a hostile file can't make the record's figures fill memory.
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use std::collections::BTreeSet;

use crate::error::LogError;
use crate::log::{Channel, Figure, FlightLog, LogFormat, Quantity, Stated};
use crate::table::finite;

/// The format's name in errors.
const FORMAT: &str = "MARSA download";

/// The lines [`recognizes`] looks through for the `<MARSA VERSION>` setting.
const RECOGNITION_LINES: usize = 200;

/// The most settings and headings a file may hold.
pub const MAX_SETTINGS: usize = 256;

/// The longest name a setting may have, in bytes.
pub const MAX_NAME_BYTES: usize = 64;

/// The longest value a setting may have, in characters.
pub const MAX_VALUE_CHARS: usize = 64;

/// The setting whose value is the firmware's version.
const FIRMWARE: &str = "FIRMWARE VER";

/// The altitude's step, m: its barometer's count, 29.3 ft at most within 300 ft of the pad among
/// the sample files ([`LogFormat::altitude_resolution_m`]). It is measured there, not stated by
/// any document, and holds near the pad only: a count of pressure spans more feet where the air
/// is thinner, so the step is larger higher up.
pub const ALTITUDE_STEP_M: f64 = 29.3 * FOOT_M;

/// A foot, m.
const FOOT_M: f64 = 0.3048;

/// Standard gravity, m/s² (the 3rd CGPM, 1901).
const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;

/// The columns, as the record names them: no line of the file names them.
const COLUMNS: [&str; 2] = ["altitude", "acceleration"];

/// A line with any empty cells at its end, a spreadsheet's re-save's, taken off.
fn trimmed(line: &str) -> &str {
    line.trim()
        .trim_end_matches(|c: char| c == ',' || c.is_whitespace())
}

/// A setting's or heading's name and value, `<NAME> value` or `<NAME>`: `None` for a line that
/// isn't one. The name is ASCII letters, digits, spaces, `_`, `-` and `.`, as MARSA's are, such as
/// `CHAN ALT BITS 0 1`: an XML document's `<?xml …?>` or `<kml>` isn't a setting.
fn setting(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix('<')?;
    let (name, value) = rest.split_once('>')?;
    let allowed = |c: char| c.is_ascii_alphanumeric() || matches!(c, ' ' | '_' | '-' | '.');
    if name.trim().is_empty() || !name.chars().all(allowed) {
        return None;
    }
    Some((name.trim(), value.trim()))
}

/// Whether a text is a MARSA download: its first line that isn't blank is a setting or a
/// heading, and one of its first 200 is the `<MARSA VERSION>` setting with a value.
pub fn recognizes(text: &str) -> bool {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text
        .split(['\r', '\n'])
        .map(trimmed)
        .filter(|line| !line.is_empty());
    let Some(first) = lines.next() else {
        return false;
    };
    setting(first).is_some()
        && std::iter::once(first)
            .chain(lines)
            .take(RECOGNITION_LINES)
            .any(
                |line| matches!(setting(line), Some(("MARSA VERSION", value)) if !value.is_empty()),
            )
}

/// A sample's three numbers, or why the line isn't one.
fn sample(line: &str) -> Result<[f64; 3], String> {
    let cells: Vec<&str> = line.split(',').map(str::trim).collect();
    let [time, altitude, acceleration] = cells.as_slice() else {
        return Err(format!(
            "a sample is three numbers by commas, and the line has {} cells",
            cells.len()
        ));
    };
    let number = |cell: &str| {
        cell.parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| format!("{:?} isn't a number", short(cell)))
    };
    Ok([number(time)?, number(altitude)?, number(acceleration)?])
}

/// Reads a MARSA download from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the text isn't one ([`recognizes`]); [`LogError::Syntax`] for
/// a setting past the bounds or named twice in its section, a setting among the samples, a line
/// that isn't three numbers, a time no later than the one before, or a value too large for its
/// conversion to SI; [`LogError::NoData`] for a file with no samples.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the first line isn't a `<NAME> value` setting, or no `<MARSA VERSION>` \
                      setting comes within the first 200 lines"
                .to_owned(),
        });
    }
    let syntax = |line: usize, message: String| LogError::Syntax {
        format: FORMAT,
        line,
        message,
    };
    let ends_unended = !text.ends_with(['\r', '\n']);
    // Lines end in CR, LF or both.
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<(usize, &str)> = text
        .split('\n')
        .enumerate()
        .map(|(index, line)| (index + 1, trimmed(line)))
        .filter(|(_, line)| !line.is_empty())
        .collect();
    let last_line = lines.last().map(|&(number, _)| number);

    let mut figures = Vec::new();
    let mut named = BTreeSet::new();
    let mut section: Option<&str> = None;
    let mut settings = 0;
    let mut firmware = None;
    let mut time_s: Vec<f64> = Vec::new();
    let mut altitude_m = Vec::new();
    let mut acceleration = Vec::new();
    let mut dropped_last = false;
    for (number, line) in lines {
        if let Some((name, value)) = setting(line) {
            if !time_s.is_empty() {
                return Err(syntax(
                    number,
                    "a setting among the samples: the settings come before them".to_owned(),
                ));
            }
            settings += 1;
            if settings > MAX_SETTINGS {
                return Err(syntax(
                    number,
                    format!("more than {MAX_SETTINGS} settings and headings"),
                ));
            }
            if name.len() > MAX_NAME_BYTES || value.chars().count() > MAX_VALUE_CHARS {
                return Err(syntax(
                    number,
                    format!(
                        "a setting's name is at most {MAX_NAME_BYTES} bytes and its value \
                         {MAX_VALUE_CHARS} characters: {:?}",
                        short(line)
                    ),
                ));
            }
            if value.is_empty() {
                section = Some(name);
                continue;
            }
            let full = match section {
                Some(heading) => format!("{heading}, {name}"),
                None => name.to_owned(),
            };
            if !named.insert(full.clone()) {
                return Err(syntax(
                    number,
                    format!("the setting `{}` is named twice", short(&full)),
                ));
            }
            if section.is_none() && name == FIRMWARE {
                firmware = Some(value.to_owned());
            }
            let figure = match value.parse::<f64>().ok().filter(|v| v.is_finite()) {
                Some(number) => Figure::new(full, value, Some(Quantity::Number), vec![number]),
                None => Figure::new(full, value, None, Vec::new()),
            };
            figures.push(figure);
            continue;
        }
        let [time, altitude, accel] = match sample(line) {
            Ok(values) => values,
            Err(_) if ends_unended && Some(number) == last_line => {
                dropped_last = true;
                continue;
            }
            Err(message) => return Err(syntax(number, message)),
        };
        if let Some(&before) = time_s.last()
            && time <= before
        {
            return Err(syntax(
                number,
                format!("the time {time} s isn't later than the one before, {before} s"),
            ));
        }
        time_s.push(time);
        altitude_m.push(finite(altitude * FOOT_M, FORMAT, number, COLUMNS[0])?);
        acceleration.push(finite(
            accel * STANDARD_GRAVITY_M_S2,
            FORMAT,
            number,
            COLUMNS[1],
        )?);
    }
    if time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes = vec![
        "no line names the samples' three numbers. They are read as the time in s, the \
         barometric altitude in ft and the acceleration in g, as MARSA's manuals and software \
         give them. The sample logs' survey holds the feet and the g to the files"
            .to_owned(),
        "the acceleration reads about 0 at rest, not 1 g: it is kept so, the rocket's own \
         acceleration only while it points straight up"
            .to_owned(),
        "the settings are kept as written, as no document states their units".to_owned(),
    ];
    if dropped_last {
        notes.push(
            "the last line, which ends the file with no line end and isn't a whole sample, \
             dropped: a download cut off as it was written"
                .to_owned(),
        );
    }
    let channel = |column: &str, quantity, values: &[f64]| Channel {
        column: column.to_owned(),
        quantity,
        time_s: time_s.clone(),
        values: values.to_vec(),
    };
    let channels = vec![
        channel(COLUMNS[0], Quantity::AltitudeAboveGround, &altitude_m),
        channel(COLUMNS[1], Quantity::Acceleration, &acceleration),
    ];
    Ok(FlightLog {
        format: LogFormat::Marsa,
        logger: "MARSA".to_owned(),
        serial_number: None,
        firmware,
        flight_number: None,
        stated: Stated {
            apogee_m: None,
            ground_elevation_msl_m: None,
            samples: None,
            figures,
        },
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

/// The most characters of the file an error quotes.
const QUOTED_CHARS: usize = 80;

/// Text from the file as an error quotes it, cut to [`QUOTED_CHARS`] characters.
fn short(text: &str) -> String {
    if text.chars().count() <= QUOTED_CHARS {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(QUOTED_CHARS - 1).collect();
        cut.push('…');
        cut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An invented download: settings, a section, and four samples, CRLF and a trailing space as
    /// the altimeter's software writes them.
    const DOWNLOAD: &str = "<BARO_ZERO 0> 790\r\n<ACCEL_ZERO 0> 498\r\n<FIRMWARE VER> 26\r\n\
                            <MARSA VERSION> 2\r\n<CHAN 1>\r\n<CHAN APOGEE OPTION> 1\r\n\
                            <CHAN 2>\r\n<CHAN APOGEE OPTION> 0\r\n\
                            -0.1,-58.1,-0.08 \r\n0.0,-58.1,8.20 \r\n0.1,-29.0,8.65 \r\n\
                            0.2,0.0,9.00 \r\n";

    fn error_line(text: &str) -> (usize, String) {
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("expected a syntax error, got {other:?}"),
        }
    }

    #[test]
    fn the_samples_read_in_seconds_feet_and_g() {
        assert!(recognizes(DOWNLOAD));
        let log = read(DOWNLOAD).unwrap();
        assert_eq!(log.format, LogFormat::Marsa);
        assert_eq!(log.time_s, [-0.1, 0.0, 0.1, 0.2]);
        assert_eq!(
            log.altitude_m,
            [-58.1 * 0.3048, -58.1 * 0.3048, -29.0 * 0.3048, 0.0]
        );
        let acceleration = &log.channels[1];
        assert_eq!(
            (acceleration.column.as_str(), acceleration.quantity),
            ("acceleration", Quantity::Acceleration)
        );
        assert_eq!(acceleration.values[1], 8.20 * 9.806_65);
        assert_eq!(log.channels[0].quantity, Quantity::AltitudeAboveGround);
        assert_eq!(log.firmware.as_deref(), Some("26"));
    }

    #[test]
    fn each_setting_is_a_figure_named_after_its_section() {
        let log = read(DOWNLOAD).unwrap();
        let names: Vec<&str> = log.stated.figures.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "BARO_ZERO 0",
                "ACCEL_ZERO 0",
                "FIRMWARE VER",
                "MARSA VERSION",
                "CHAN 1, CHAN APOGEE OPTION",
                "CHAN 2, CHAN APOGEE OPTION",
            ]
        );
        let zero = &log.stated.figures[0];
        assert_eq!(
            (zero.written.as_str(), zero.quantity, zero.values.as_slice()),
            ("790", Some(Quantity::Number), &[790.0][..])
        );
    }

    #[test]
    fn a_spreadsheets_resave_reads_the_same() {
        let resaved = DOWNLOAD
            .replace(" \r\n", "\r\n")
            .replace("> 790\r\n", "> 790,,\r\n")
            .replace("<CHAN 1>\r\n", "<CHAN 1>,,\r\n")
            .replace("0.2,0.0,9.00", "0.2,0,9");
        assert_ne!(resaved, DOWNLOAD);
        let (a, b) = (read(DOWNLOAD).unwrap(), read(&resaved).unwrap());
        assert_eq!(a.time_s, b.time_s);
        assert_eq!(a.altitude_m, b.altitude_m);
        assert_eq!(a.channels, b.channels);
        assert_eq!(a.stated, b.stated);
    }

    #[test]
    fn a_time_no_later_than_the_one_before_is_refused_at_its_line() {
        let text = DOWNLOAD.replace("0.1,-29.0", "0.0,-29.0");
        let (line, message) = error_line(&text);
        assert_eq!(line, 11);
        assert!(message.contains("isn't later"), "{message}");
    }

    #[test]
    fn a_line_that_isnt_three_numbers_is_refused_at_its_line() {
        for (row, expected) in [
            ("0.1,-29.0", "cells"),
            ("0.1,-29.0,8.65,1", "cells"),
            ("0.1,x,8.65", "isn't a number"),
            ("0.1,inf,8.65", "isn't a number"),
        ] {
            let text = DOWNLOAD.replace("0.1,-29.0,8.65 ", row);
            let (line, message) = error_line(&text);
            assert_eq!(line, 11, "{row}");
            assert!(message.contains(expected), "{row}: {message}");
        }
    }

    #[test]
    fn a_setting_among_the_samples_or_named_twice_is_refused() {
        let among = DOWNLOAD.replace("0.1,-29.0,8.65 ", "<CHAN 3>");
        assert!(error_line(&among).1.contains("among the samples"));
        let twice = DOWNLOAD.replace("<ACCEL_ZERO 0>", "<BARO_ZERO 0>");
        let (line, message) = error_line(&twice);
        assert_eq!(line, 2);
        assert!(
            message.contains("`BARO_ZERO 0` is named twice"),
            "{message}"
        );
        // One name in two sections is two settings.
        assert!(read(DOWNLOAD).is_ok());
    }

    #[test]
    fn a_cut_last_line_is_dropped_only_with_no_line_end() {
        let cut = format!("{DOWNLOAD}0.3,29");
        let log = read(&cut).unwrap();
        assert_eq!(log.time_s.len(), 4);
        assert!(log.notes.iter().any(|n| n.contains("cut off")));
        let ended = format!("{DOWNLOAD}0.3,29\r\n");
        assert_eq!(error_line(&ended).0, 13);
        let whole = format!("{DOWNLOAD}0.3,29.1,9.1");
        let log = read(&whole).unwrap();
        assert_eq!(log.time_s.len(), 5);
        assert!(!log.notes.iter().any(|n| n.contains("cut off")));
    }

    #[test]
    fn the_bounds_hold_at_their_limits() {
        let settings: String = (1..MAX_SETTINGS)
            .map(|i| format!("<S {i}> {i}\n"))
            .collect();
        let text = format!("<MARSA VERSION> 2\n{settings}0,0,0\n");
        assert_eq!(read(&text).unwrap().stated.figures.len(), MAX_SETTINGS);
        let over = format!("<S 0> 0\n{text}");
        assert!(error_line(&over).1.contains("more than 256"));
        let long_name = "N".repeat(MAX_NAME_BYTES);
        let at = format!("<{long_name}> 1\n<MARSA VERSION> 2\n0,0,0\n");
        assert!(read(&at).is_ok());
        let past = format!("<{long_name}N> 1\n<MARSA VERSION> 2\n0,0,0\n");
        assert!(error_line(&past).1.contains("at most 64 bytes"));
        let value = "9".repeat(MAX_VALUE_CHARS);
        assert!(read(&format!("<V> {value}\n<MARSA VERSION> 2\n0,0,0\n")).is_ok());
        let past = format!("<V> {value}9\n<MARSA VERSION> 2\n0,0,0\n");
        assert_eq!(error_line(&past).0, 1);
    }

    #[test]
    fn a_value_too_large_for_its_conversion_is_refused() {
        let text = DOWNLOAD.replace("0.1,-29.0,8.65", "0.1,-29.0,1e308");
        let (line, message) = error_line(&text);
        assert_eq!(line, 11);
        assert!(message.contains("acceleration"), "{message}");
    }

    /// A byte-order mark, as a spreadsheet's "CSV UTF-8" re-save writes, is passed over.
    #[test]
    fn a_byte_order_mark_is_passed_over() {
        let marked = format!("\u{feff}{DOWNLOAD}");
        assert!(recognizes(&marked));
        assert_eq!(read(&marked).unwrap(), read(DOWNLOAD).unwrap());
    }

    #[test]
    fn a_file_with_no_version_setting_or_no_samples_isnt_read() {
        let unversioned = DOWNLOAD.replace("<MARSA VERSION> 2\r\n", "");
        assert!(!recognizes(&unversioned));
        assert!(matches!(
            read(&unversioned),
            Err(LogError::NotThisFormat { .. })
        ));
        // A version with no value is a heading, not the setting.
        assert!(!recognizes(
            &DOWNLOAD.replace("<MARSA VERSION> 2", "<MARSA VERSION>")
        ));
        let settings_only = "<BARO_ZERO 0> 790\n<MARSA VERSION> 2\n";
        assert!(matches!(read(settings_only), Err(LogError::NoData { .. })));
        // An XML document isn't a download, even one holding the setting.
        assert!(!recognizes(
            "<?xml version=\"1.0\"?>\n<MARSA VERSION> 2\n0,0,0\n"
        ));
        let kml = "<kml xmlns=\"http://www.opengis.net/kml/2.2\">\n<MARSA VERSION> 2\n</kml>\n";
        assert!(!recognizes(kml));
    }
}
