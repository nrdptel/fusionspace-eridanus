//! AED Electronics' R-DAS flight computer's raw data, as its Windows software exports a flight
//! to text: a `*** Configuration ***` section, then `*** Raw data ***` and a table of the
//! converters' counts.
//!
//! The R-DAS records its accelerometer and its barometer, and as its owner chooses its four
//! digital inputs and six analog inputs from an expansion port, each through a 10-bit converter
//! over 0 to 5 V, at up to 200 samples a second (*R-DAS User Manual*, V1.5, AED Electronics,
//! 2001, p. 7, p. 14). It keeps the 2 s before liftoff and stores from there (p. 6). Its Windows
//! software exports a downloaded flight as text, either raw or converted with the calibration
//! values set in the software; the raw file's header "contains a summary of the measurement
//! configuration", and its values are "measured ADC-values", each step 5 V / 1,024 = 4.883 mV,
//! for the reader to convert with the calibration values (p. 14). The manual shows no file and
//! names no column. The sample logs hold the raw export only, 54 files from a college course's
//! flights in 2008: 28 recorded on board at 200 Hz, and 26 at 10 Hz, by their names the course's
//! telemetry records of the same flights, in the same layout but for a configuration whose
//! `Trigger:` line is blank and which names no firmware.
//!
//! **Recognition.** The first line that isn't blank is `*** Configuration ***`, and a line
//! `*** Raw data ***` comes within the file's first 200 lines; no other layout HPR Sim reads opens
//! so.
//!
//! **The configuration.** Two of its lines are needed, and a file without either is refused
//! ([`LogError::Unit`]): `Data is sampled at N Hz.`, which the clock is checked against, and
//! `Inputs that were sampled: …`, which says what the table's columns hold: `Acceleration` is
//! `Acc`, `Pressure` is `Prs`, `the digital inputs` is `Dig`, and `ADC0` to `ADC5` are
//! themselves. The table's header, after `Time`, must name those columns in that order, and a
//! list naming an input the reader doesn't know, or one twice, is refused ([`LogError::Syntax`]).
//! A line holding `with firmware version N` gives [`FlightLog::firmware`], the last if several do. Every line of the configuration that
//! isn't blank, its trigger and recovery settings among them, is kept in a note as written, at
//! most 40 lines of 200 characters.
//!
//! **Columns.** The file states no calibration, and the manual leaves the conversions to values
//! set in the software, which the file doesn't hold, so no column is converted:
//!
//! | Column | Quantity | As read |
//! |---|---|---|
//! | `Time` | the clock, s | from liftoff: checked against the stated rate (below) |
//! | `Acc`, `Prs`, `ADC0` … `ADC5` | [`Quantity::Unrecognized`] | the converter's counts, whole numbers 0 to 1,023, as written |
//! | `Dig` | [`Quantity::Number`] | a hexadecimal word, kept as its number |
//!
//! `Acc` is the accelerometer, an Analog Devices ADXL150 for ±50 g, and `Prs` the barometer, a
//! Motorola MPXS4100A for 20 to 105 kPa, in the manual's R-DAS (p. 7); but the file states
//! neither the calibration values nor which board and sensors wrote it, and others have been
//! fitted to the R-DAS (a college course's pressure lab names an MPXH6115A on its R-DAS: *E80
//! Pressure and Temperature Lab*, Harvey Mudd College,
//! <https://pages.hmc.edu/spjut/NewE80/PresTempLab.html>, section 2), so the counts aren't taken
//! through that sensor's data sheet. The manual says the digital byte holds the four digital
//! inputs and, in its other four bits, the flight's phase: before liftoff, flight, drogue and main
//! (p. 14), and, of firmware 2.2, that "bit 3 is set when timed recovery was used" (p. 22); it gives
//! no other bit. The files write a word of four hexadecimal digits, not a byte, and never set bit 3
//! of either byte: they set `0x80` and `0x8000`. In 27 of the 28 on-board sample files the word
//! reads `0000` from the first row, `0010` from within 0.25 s after the zero, then `0020`, `0030`
//! and, near the file's end, `8030`; the 28th writes `00B0` and `80B0` in place of the last two.
//! The low byte's high bits look like the phases in order, but that rests on the files, not a
//! document, so the word is kept as written and no event is taken from it.
//!
//! **The clock.** `Time` counts from liftoff, `t = 0` in the manual's figure 2.2, the g-switch, 2.5 g
//! held for 0.25 s, firing at 0.25 s (p. 10, p. 11): every sample file's first row is at −2.000,
//! the 2 s before liftoff the manual says it keeps. That it counts seconds rests on the
//! stated rate: the clock's average step, from the first row to the last, must be the stated
//! rate's interval within 1%, or the file is refused ([`LogError::Unit`]); a note gives both. In
//! every sample file the step is the interval exactly, 0.005 s at 200 Hz and 0.1 s at 10 Hz. A
//! step under half the interval or over one and a half of it is counted in the note.
//!
//! **The primary track.** The file holds counts and no altitude, so [`FlightLog::time_s`] and
//! [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`, and every column is a
//! channel on the file's clock.
//!
//! **Cells.** Every row must hold as many cells as the header names, its `Time` a number greater
//! than the row before's, and every count that is a number a whole one from 0 to 1,023, which
//! keeps a file of converted values from passing for counts; otherwise the file is refused,
//! naming the line. Any other cell that isn't a number, or for `Dig` a word of hexadecimal digits,
//! is a gap (`NaN`), counted in a note. Lines may end in LF or CR LF; a file whose lines end in a
//! lone CR isn't recognized.
//!
//! An invented file, in the layout the sample files share:
//!
//! ```text
//! *** Configuration ***
//!
//! Trigger: G-switch.
//! Data is sampled at 200 Hz.
//! Inputs that were sampled: Acceleration, Pressure, the digital inputs, ADC0, ADC1.
//!
//! This data was generated with version 4.1 of the RDAS windows software,
//! and downloaded from an RDAS board with firmware version 4.1
//!
//!
//! *** Raw data ***
//!
//!    Time  Acc  Prs  Dig ADC0 ADC1
//!  -2.000  510  800 0000  500  250
//!  -1.995  511  800 0000  501  250
//! ```

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};

/// The format's name in errors.
const FORMAT: &str = "R-DAS text";

/// The logger, as the record names it.
const LOGGER: &str = "AED Electronics R-DAS";

/// The line that opens the file.
const CONFIGURATION: &str = "*** Configuration ***";

/// The line that ends the configuration and opens the table.
const RAW_DATA: &str = "*** Raw data ***";

/// How many lines, from the first, may come before [`RAW_DATA`]: the sample files' configuration
/// takes 13; a file whose marker comes later isn't read as this layout, so that recognizing it
/// never walks a whole large file.
const CONFIGURATION_LINES: usize = 200;

/// The barometer's step, as the format's resolution ([`LogFormat::altitude_resolution_m`]):
/// the file writes no altitude, but its pressure moves by the barometer's count, 0.09 kPa in the
/// manual's R-DAS (p. 7), 7.5 m near sea level (`Δh = Δp / (ρ₀ g₀)`, as
/// [`LogFormat::altitude_resolution_m`] takes it). A board with another sensor steps otherwise.
pub const ALTITUDE_STEP_M: f64 = 90.0 / (1.225 * 9.806_65);

/// The largest count of the R-DAS's 10-bit converter.
const MAX_COUNT: f64 = 1_023.0;

/// How far the clock's average step may lie from the stated rate's, as a share of it.
const CLOCK_MARGIN: f64 = 0.01;

/// How many configuration lines a note quotes, and how many characters of each, so that a hostile
/// file can't make the record far larger than the samples it holds.
const QUOTED_LINES: usize = 40;
const QUOTED_CHARS: usize = 200;

/// Whether a file's text is in this layout: its first line that isn't blank is
/// `*** Configuration ***`, and a `*** Raw data ***` line comes within its first 200 lines.
pub fn recognizes(text: &str) -> bool {
    let mut lines = text
        .lines()
        .take(CONFIGURATION_LINES)
        .map(str::trim)
        .skip_while(|line| line.is_empty());
    lines.next() == Some(CONFIGURATION) && lines.any(|line| line == RAW_DATA)
}

/// What the configuration says, as the reader uses it.
struct Configuration {
    /// The stated sample rate, Hz, and its line.
    rate_hz: Option<(f64, usize)>,
    /// The inputs the configuration lists, each as the table's column names it, and the line.
    inputs: Option<(Vec<String>, usize)>,
    /// The board's firmware version, as written.
    firmware: Option<String>,
    /// The configuration's lines that aren't blank, as written, for the note.
    lines: Vec<String>,
}

/// The column the table writes for an input the configuration names, or `None` for an input the
/// reader doesn't know.
fn column_of(input: &str) -> Option<String> {
    match input {
        "Acceleration" => Some("Acc".to_owned()),
        "Pressure" => Some("Prs".to_owned()),
        "the digital inputs" => Some("Dig".to_owned()),
        // The six analog inputs of the expansion port (manual, p. 7, p. 14).
        "ADC0" | "ADC1" | "ADC2" | "ADC3" | "ADC4" | "ADC5" => Some(input.to_owned()),
        _ => None,
    }
}

/// Reads the configuration's lines, `lines` numbered from 1.
fn configuration<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
) -> Result<Configuration, LogError> {
    let mut configuration = Configuration {
        rate_hz: None,
        inputs: None,
        firmware: None,
        lines: Vec::new(),
    };
    for (number, line) in lines {
        let line = line.trim();
        if line.is_empty() || line == CONFIGURATION {
            continue;
        }
        configuration.lines.push(line.to_owned());
        if let Some(rest) = line.strip_prefix("Data is sampled at ") {
            let rate = rest
                .strip_suffix(" Hz.")
                .and_then(|rate| rate.trim().parse::<f64>().ok())
                .filter(|rate| rate.is_finite() && *rate > 0.0)
                .ok_or_else(|| {
                    syntax(
                        number,
                        format!("a sample rate that isn't one: {:?}", cut(line)),
                    )
                })?;
            if configuration.rate_hz.replace((rate, number)).is_some() {
                return Err(syntax(number, "a second sample rate".to_owned()));
            }
        } else if let Some(rest) = line.strip_prefix("Inputs that were sampled: ") {
            let rest = rest.strip_suffix('.').unwrap_or(rest);
            let columns = rest
                .split(',')
                .map(|input| {
                    column_of(input.trim()).ok_or_else(|| {
                        syntax(
                            number,
                            format!("an input the reader doesn't know: {:?}", cut(input.trim())),
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if let Some(twice) = columns
                .iter()
                .enumerate()
                .find_map(|(k, column)| columns[..k].contains(column).then_some(column))
            {
                return Err(syntax(number, format!("an input listed twice: `{twice}`")));
            }
            if configuration.inputs.replace((columns, number)).is_some() {
                return Err(syntax(number, "a second list of inputs".to_owned()));
            }
        } else if let Some((_, version)) = line.split_once("with firmware version ") {
            let version = version.trim().trim_end_matches('.').trim();
            if !version.is_empty() {
                configuration.firmware = Some(version.to_owned());
            }
        }
    }
    Ok(configuration)
}

/// Reads a log in this layout.
///
/// # Errors
///
/// - [`LogError::NotThisFormat`] if the file doesn't open with `*** Configuration ***` and a
///   `*** Raw data ***` line within its first 200 lines.
/// - [`LogError::Syntax`] for a configuration line the reader can't take (a sample rate that
///   isn't a positive number, an input it doesn't know or listed twice, a rate or a list stated
///   twice), a table header that doesn't open with `Time` or doesn't name the inputs the
///   configuration lists, a row not as wide as the header, a `Time` that isn't a number greater
///   than the row before's, or a count that is a number but not a whole one from 0 to 1,023.
/// - [`LogError::NoData`] without a row.
/// - [`LogError::Unit`] where the configuration states no sample rate or lists no inputs, the
///   file holds a single row, or the clock's average step isn't the stated rate's within 1%:
///   then nothing in the file says the clock counts seconds.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!("the file doesn't open with `{CONFIGURATION}` and `{RAW_DATA}`"),
        });
    }
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line));
    let config_lines: Vec<(usize, &str)> = lines
        .by_ref()
        .take_while(|(_, line)| line.trim() != RAW_DATA)
        .collect();
    let configuration = configuration(config_lines.into_iter())?;

    // The table's header: the first line after the marker that isn't blank.
    let Some((header_line, header)) = lines.by_ref().find(|(_, line)| !line.trim().is_empty())
    else {
        return Err(LogError::NoData { format: FORMAT });
    };
    let names: Vec<&str> = header.split_whitespace().collect();
    if names.first() != Some(&"Time") {
        return Err(syntax(
            header_line,
            format!(
                "a table header that doesn't open with `Time`: {:?}",
                cut(header.trim())
            ),
        ));
    }
    let names = &names[1..];
    let (inputs, inputs_line) = configuration
        .inputs
        .as_ref()
        .ok_or_else(|| LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: "the configuration lists no inputs, so nothing says what the table's columns \
                  hold"
                .to_owned(),
        })?;
    if names.iter().copied().ne(inputs.iter().map(String::as_str)) {
        return Err(syntax(
            header_line,
            format!(
                "the table's columns, {}, aren't the inputs line {inputs_line} lists, {}",
                cut(&names.join(" ")),
                cut(&inputs.join(" "))
            ),
        ));
    }
    let digital = names.iter().position(|name| *name == "Dig");

    let mut time_s = Vec::new();
    let mut values: Vec<Vec<f64>> = vec![Vec::new(); names.len()];
    let mut gaps = vec![0usize; names.len()];
    let mut first_gap = None;
    for (number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() != names.len() + 1 {
            return Err(syntax(
                number,
                format!(
                    "a row of {} cells where the header names {}",
                    cells.len(),
                    names.len() + 1
                ),
            ));
        }
        let time = cells[0]
            .parse::<f64>()
            .ok()
            .filter(|time| time.is_finite())
            .ok_or_else(|| {
                syntax(
                    number,
                    format!("a `Time` that isn't a number: {:?}", cut(cells[0])),
                )
            })?;
        if time_s.last().is_some_and(|last| time <= *last) {
            return Err(syntax(
                number,
                format!("a `Time`, {time}, not after the row before's"),
            ));
        }
        time_s.push(time);
        for (k, cell) in cells[1..].iter().enumerate() {
            let value = if Some(k) == digital {
                // Hexadecimal digits only: `from_str_radix` would also take a sign.
                cell.bytes()
                    .all(|b| b.is_ascii_hexdigit())
                    .then(|| u32::from_str_radix(cell, 16).ok())
                    .flatten()
                    .map(f64::from)
            } else {
                let value = cell.parse::<f64>().ok().filter(|value| value.is_finite());
                if let Some(value) = value
                    && !(value.fract() == 0.0 && (0.0..=MAX_COUNT).contains(&value))
                {
                    return Err(syntax(
                        number,
                        format!(
                            "`{}` holds {:?}, not a count of the 10-bit converter, a whole \
                             number from 0 to 1,023: a file of converted values isn't read",
                            names[k],
                            cut(cell)
                        ),
                    ));
                }
                value
            };
            values[k].push(value.unwrap_or_else(|| {
                gaps[k] += 1;
                first_gap.get_or_insert(number);
                f64::NAN
            }));
        }
    }
    if time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes = vec![clock(&configuration, &time_s, header_line)?];
    let counts: Vec<String> = names
        .iter()
        .filter(|name| **name != "Dig")
        .map(|name| format!("`{name}`"))
        .collect();
    if !counts.is_empty() {
        notes.push(format!(
            "{} kept as written: the converters' counts, which the file states no calibration \
             for",
            counts.join(", ")
        ));
    }
    if digital.is_some() {
        notes.push(
            "`Dig` read in hexadecimal and kept as its number: the digital inputs and the \
             flight's phase, which the file doesn't name bit by bit"
                .to_owned(),
        );
    }
    let total_gaps: usize = gaps.iter().sum();
    if let Some(line) = first_gap {
        notes.push(if total_gaps == 1 {
            format!("1 cell that isn't a number read as a gap, on line {line}")
        } else {
            format!("{total_gaps} cells that aren't numbers read as gaps, the first on line {line}")
        });
    }
    let shown = configuration.lines.len().min(QUOTED_LINES);
    let mut quoted: Vec<String> = configuration.lines[..shown]
        .iter()
        .map(|line| cut(line))
        .collect();
    if configuration.lines.len() > shown {
        quoted.push(format!(
            "and {} more lines",
            configuration.lines.len() - shown
        ));
    }
    notes.push(format!(
        "the configuration, as written: {}",
        quoted.join(" | ")
    ));
    notes.push(
        "the file holds counts and no altitude, so the record's primary track is empty: every \
         column is a channel on the file's clock"
            .to_owned(),
    );

    let channels = names
        .iter()
        .zip(values)
        .map(|(name, values)| Channel {
            column: (*name).to_owned(),
            quantity: if *name == "Dig" {
                Quantity::Number
            } else {
                Quantity::Unrecognized
            },
            time_s: time_s.clone(),
            values,
        })
        .collect();

    Ok(FlightLog {
        format: LogFormat::RdasText,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: configuration.firmware,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

/// The clock's check against the stated rate, as a note; a refusal where it fails.
fn clock(
    configuration: &Configuration,
    time_s: &[f64],
    header_line: usize,
) -> Result<String, LogError> {
    let unit = |message: String| LogError::Unit {
        format: FORMAT,
        line: header_line,
        message,
    };
    let (rate_hz, rate_line) = configuration.rate_hz.ok_or_else(|| {
        unit(
            "the configuration states no sample rate, which the clock is checked against"
                .to_owned(),
        )
    })?;
    let (Some(first), Some(last)) = (time_s.first(), time_s.last()) else {
        return Err(LogError::NoData { format: FORMAT });
    };
    if time_s.len() < 2 {
        return Err(unit(
            "a single row, whose clock can't be checked against the stated rate".to_owned(),
        ));
    }
    let step_s = (last - first) / (time_s.len() - 1) as f64;
    let ratio = step_s * rate_hz;
    if (ratio - 1.0).abs() > CLOCK_MARGIN {
        return Err(unit(format!(
            "`Time` steps by {step_s} on average, {ratio:.3} times the {rate_hz} Hz line \
             {rate_line} states, not within 1%: nothing says the clock counts seconds"
        )));
    }
    let interval_s = 1.0 / rate_hz;
    let uneven = time_s
        .windows(2)
        .filter(|pair| {
            let step = pair[1] - pair[0];
            step < 0.5 * interval_s || step > 1.5 * interval_s
        })
        .count();
    let mut note = format!(
        "`Time` read in seconds: it steps by {step_s:.6} on average, the {rate_hz} Hz line \
         {rate_line} states within 1% ({ratio:.4} times its {interval_s:.6} s)"
    );
    if uneven > 0 {
        note.push_str(&format!(
            "; {uneven} steps are under half or over one and a half of it"
        ));
    }
    Ok(note)
}

/// Text from the file cut to [`QUOTED_CHARS`] characters, for a note or an error.
fn cut(text: &str) -> String {
    text.chars().take(QUOTED_CHARS).collect()
}

/// A refusal at a line of the file.
fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file with the configuration's lines `config`, a header `header` and rows `rows`.
    fn file(config: &[&str], header: &str, rows: &[&str]) -> String {
        let mut lines = vec![CONFIGURATION, ""];
        lines.extend_from_slice(config);
        lines.extend_from_slice(&["", RAW_DATA, "", header]);
        lines.extend_from_slice(rows);
        lines.join("\n") + "\n"
    }

    const RATE: &str = "Data is sampled at 100 Hz.";
    const INPUTS: &str = "Inputs that were sampled: Acceleration, Pressure, the digital inputs.";
    const HEADER: &str = "   Time  Acc  Prs  Dig";

    /// Rows every 0.01 s from 0, as many as `n`.
    fn rows(n: usize) -> Vec<String> {
        (0..n)
            .map(|k| format!("{:7.3} 510 800 0010", k as f64 * 0.01))
            .collect()
    }

    fn good(n: usize) -> String {
        let rows = rows(n);
        let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
        file(&[RATE, INPUTS], HEADER, &rows)
    }

    /// The message of a refusal of the kind `kind`, which must be the one returned.
    fn refusal(text: &str, kind: &str) -> String {
        match (read(text), kind) {
            (Err(LogError::Unit { message, .. }), "unit") => message,
            (Err(LogError::Syntax { message, line, .. }), "syntax") => {
                format!("line {line}: {message}")
            }
            (Err(LogError::NoData { .. }), "no data") => String::new(),
            (other, _) => panic!("expected a {kind} refusal, got {other:?}"),
        }
    }

    #[test]
    fn the_layout_is_recognized_by_its_two_markers() {
        assert!(recognizes(&good(3)));
        assert!(recognizes(&format!(
            "\r\n\r\n{}",
            good(3).replace('\n', "\r\n")
        )));
        // No table marker, or one past the first 200 lines, isn't the layout.
        assert!(!recognizes(
            "*** Configuration ***\nData is sampled at 100 Hz.\n"
        ));
        // The marker on line 201, then on line 200.
        let late = format!("{CONFIGURATION}\n{}{RAW_DATA}\n", "x\n".repeat(199));
        assert!(!recognizes(&late));
        let in_time = format!("{CONFIGURATION}\n{}{RAW_DATA}\n", "x\n".repeat(198));
        assert!(recognizes(&in_time));
        // The configuration's marker must come first.
        assert!(!recognizes(&format!("Time,Altitude\n{}", good(3))));
        assert!(matches!(
            read("Time,Altitude\n0,0\n"),
            Err(LogError::NotThisFormat { .. })
        ));
    }

    #[test]
    fn a_good_file_reads_its_firmware_and_notes_the_clock() {
        let text = file(
            &[
                RATE,
                INPUTS,
                "and downloaded from an RDAS board with firmware version 4.1",
            ],
            HEADER,
            &[
                "  0.000 510 800 0000",
                "  0.010 511 801 001F",
                "  0.020 512 802 0010",
            ],
        );
        let log = read(&text).unwrap();
        assert_eq!(log.firmware.as_deref(), Some("4.1"));
        assert_eq!(log.channels[2].values, [0.0, 31.0, 16.0]);
        assert!(
            log.notes[0].starts_with("`Time` read in seconds: it steps by 0.010000 on average")
        );
        assert_eq!(log.format.altitude_resolution_m(), ALTITUDE_STEP_M);
    }

    #[test]
    fn a_clock_off_the_stated_rate_by_more_than_one_percent_is_refused() {
        // 0.01009 s a step at 100 Hz is 1.009 intervals: read.
        let near = file(
            &[RATE, INPUTS],
            HEADER,
            &["0 1 1 0", "0.01009 1 1 0", "0.02018 1 1 0"],
        );
        assert!(read(&near).is_ok());
        let early = file(
            &[RATE, INPUTS],
            HEADER,
            &["0 1 1 0", "0.00991 1 1 0", "0.01982 1 1 0"],
        );
        assert!(read(&early).is_ok());
        // 1.011 and 0.989 intervals: refused.
        for step in ["0.01011", "0.00989"] {
            let second = format!("{step} 1 1 0");
            let text = file(&[RATE, INPUTS], HEADER, &["0 1 1 0", &second]);
            let message = refusal(&text, "unit");
            assert!(message.contains("not within 1%"), "{message}");
        }
        // A clock in milliseconds is 1,000 intervals a step.
        let millis = file(
            &[RATE, INPUTS],
            HEADER,
            &["0 1 1 0", "10 1 1 0", "20 1 1 0"],
        );
        assert!(refusal(&millis, "unit").contains("1000.000 times the 100 Hz line"));
    }

    #[test]
    fn uneven_steps_are_counted() {
        // A dropped sample: one step of 0.02 s, two intervals, among 0.01 s ones; the average stays
        // within 1% of the interval over a long file.
        let mut rows = rows(200);
        rows.remove(100);
        let mut more = super::tests::rows(1);
        more[0] = format!("{:7.3} 510 800 0010", 2.0);
        rows.extend(more);
        let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
        let log = read(&file(&[RATE, INPUTS], HEADER, &rows)).unwrap();
        assert!(
            log.notes[0].ends_with("; 1 steps are under half or over one and a half of it"),
            "{}",
            log.notes[0]
        );
    }

    #[test]
    fn a_configuration_without_its_rate_or_inputs_is_refused() {
        let rows = ["0 1 1 0", "0.01 1 1 0"];
        let message = refusal(&file(&[INPUTS], HEADER, &rows), "unit");
        assert!(message.contains("states no sample rate"), "{message}");
        let message = refusal(&file(&[RATE], HEADER, &rows), "unit");
        assert!(message.contains("lists no inputs"), "{message}");
        let message = refusal(&file(&[RATE, RATE, INPUTS], HEADER, &rows), "syntax");
        assert!(message.contains("a second sample rate"), "{message}");
        let message = refusal(&file(&[RATE, INPUTS, INPUTS], HEADER, &rows), "syntax");
        assert!(message.contains("a second list of inputs"), "{message}");
        for rate in [
            "Data is sampled at 0 Hz.",
            "Data is sampled at fast Hz.",
            "Data is sampled at 100 Hz",
        ] {
            let message = refusal(&file(&[rate, INPUTS], HEADER, &rows), "syntax");
            assert!(
                message.contains("a sample rate that isn't one"),
                "{message}"
            );
        }
        let gyro = "Inputs that were sampled: Acceleration, Gyro.";
        let message = refusal(&file(&[RATE, gyro], HEADER, &rows), "syntax");
        assert!(
            message.contains("an input the reader doesn't know: \"Gyro\""),
            "{message}"
        );
        // The port's six analog inputs are ADC0 to ADC5.
        let analog = "Inputs that were sampled: ADC0, ADC5.";
        let ok = file(&[RATE, analog], "Time ADC0 ADC5", &["0 1 1", "0.01 1 1"]);
        assert_eq!(read(&ok).unwrap().channels.len(), 2);
        for input in ["ADC6", "ADC00", "ADC"] {
            let inputs = format!("Inputs that were sampled: ADC0, {input}.");
            let header = format!("Time ADC0 {input}");
            let text = file(&[RATE, &inputs], &header, &["0 1 1", "0.01 1 1"]);
            let message = refusal(&text, "syntax");
            let expected = format!("an input the reader doesn't know: \"{input}\"");
            assert!(message.contains(&expected), "{message}");
        }
        // An input listed twice would make two columns of one name.
        let twice = "Inputs that were sampled: the digital inputs, the digital inputs.";
        let text = file(
            &[RATE, twice],
            "Time Dig Dig",
            &["0 0010 0010", "0.01 0010 0010"],
        );
        let message = refusal(&text, "syntax");
        assert!(
            message.contains("an input listed twice: `Dig`"),
            "{message}"
        );
    }

    #[test]
    fn a_header_must_name_the_listed_inputs() {
        let rows = ["0 1 1 0", "0.01 1 1 0"];
        let message = refusal(
            &file(&[RATE, INPUTS], "   Time  Prs  Acc  Dig", &rows),
            "syntax",
        );
        assert!(
            message.starts_with(
                "line 8: the table's columns, Prs Acc Dig, aren't the inputs line 4 lists"
            ),
            "{message}"
        );
        let message = refusal(&file(&[RATE, INPUTS], "   Time  Acc  Prs", &rows), "syntax");
        assert!(message.contains("aren't the inputs"), "{message}");
        let message = refusal(
            &file(&[RATE, INPUTS], "   T  Acc  Prs  Dig", &rows),
            "syntax",
        );
        assert!(message.contains("doesn't open with `Time`"), "{message}");
    }

    #[test]
    fn rows_must_be_whole_and_in_time_order() {
        let message = refusal(
            &file(&[RATE, INPUTS], HEADER, &["0 1 1 0", "0.01 1 1"]),
            "syntax",
        );
        assert!(
            message.starts_with("line 10: a row of 3 cells where the header names 4"),
            "{message}"
        );
        let message = refusal(
            &file(&[RATE, INPUTS], HEADER, &["0.01 1 1 0", "0.01 1 1 0"]),
            "syntax",
        );
        assert!(
            message.starts_with("line 10: a `Time`, 0.01, not after"),
            "{message}"
        );
        let message = refusal(
            &file(&[RATE, INPUTS], HEADER, &["0 1 1 0", "inf 1 1 0"]),
            "syntax",
        );
        assert!(
            message.contains("a `Time` that isn't a number"),
            "{message}"
        );
        let message = refusal(&file(&[RATE, INPUTS], HEADER, &["0 1 1 0"]), "unit");
        assert!(message.contains("a single row"), "{message}");
        refusal(&file(&[RATE, INPUTS], HEADER, &[]), "no data");
        refusal(
            &format!("{CONFIGURATION}\n{RATE}\n{INPUTS}\n{RAW_DATA}\n\n"),
            "no data",
        );
    }

    #[test]
    fn cells_that_arent_numbers_are_gaps() {
        let text = file(
            &[RATE, INPUTS],
            HEADER,
            &["0 1 1 0", "0.01 x 1 G", "0.02 1 1e400 +10"],
        );
        let log = read(&text).unwrap();
        assert!(log.channels[0].values[1].is_nan() && log.channels[2].values[1].is_nan());
        // A sign isn't a hexadecimal digit.
        assert!(log.channels[1].values[2].is_nan() && log.channels[2].values[2].is_nan());
        assert!(log.notes.contains(
            &"4 cells that aren't numbers read as gaps, the first on line 10".to_owned()
        ));
    }

    #[test]
    fn a_count_must_be_a_whole_number_the_converter_can_write() {
        let ok = file(&[RATE, INPUTS], HEADER, &["0 0 1023 0", "0.01 1 1 0"]);
        assert!(read(&ok).is_ok());
        for cell in ["1024", "-1", "1.5", "101.3"] {
            let row = format!("0.01 {cell} 1 0");
            let message = refusal(&file(&[RATE, INPUTS], HEADER, &["0 1 1 0", &row]), "syntax");
            assert!(
                message.starts_with(&format!("line 10: `Acc` holds \"{cell}\", not a count")),
                "{message}"
            );
        }
    }

    #[test]
    fn quoted_text_is_bounded() {
        let long = "y".repeat(10_000);
        let many: Vec<String> = (0..150).map(|k| format!("Setting {k}.")).collect();
        let mut config: Vec<&str> = vec![RATE, INPUTS, &long];
        config.extend(many.iter().map(String::as_str));
        let log = read(&file(&config, HEADER, &["0 1 1 0", "0.01 1 1 0"])).unwrap();
        let quoted = log
            .notes
            .iter()
            .find(|n| n.starts_with("the configuration"))
            .unwrap();
        assert!(
            quoted.ends_with(" | and 113 more lines"),
            "{}",
            &quoted[quoted.len() - 60..]
        );
        assert!(quoted.len() < 40 * (QUOTED_CHARS + 3) + 100);
        // An error quoting a long line quotes 200 characters of it.
        let rate = format!("Data is sampled at {long}");
        let message = refusal(&file(&[&rate, INPUTS], HEADER, &["0 1 1 0"]), "syntax");
        assert!(message.len() < QUOTED_CHARS + 60, "{}", message.len());
    }
}
