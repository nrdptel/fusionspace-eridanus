//! Featherweight's summary files, as the Featherweight UI app exports a Blue Raven's or a GPS
//! tracker's flight beside its data: the figures the logger worked out about the flight, one a
//! row, and no samples.
//!
//! The Blue Raven "records three sets of data for each flight": "flight summary data such as
//! maximum altitude and speed", a low-rate set and a high-rate set, and the app downloads the
//! summary first "so that the flight performance is immediately available for review"
//! (Featherweight, *Blue Raven User's Guide*, updated May 12 2026, p18). The app exports the
//! summary as a file of its own, and a GPS tracker's flight gets one too. Each row is a figure's
//! name, then its value, the unit written after the number:
//!
//! ```text
//! Rocket Name,Club demo
//! Max Altitude,2500.25 feet
//! Max velocity,500.50 feet/sec
//! Thrust tilt efficiency,98.50%
//! Direction of 1st motion,-1.0000,0.0000,0.0000
//! ```
//!
//! A file is a summary when its first line that isn't blank names `Rocket Name`, and a line among
//! its first 200 names `Max Altitude` or `Launch time UTC`, figures only a summary states. A
//! name is matched without regard to case or repeated spaces, and kept as written. A tracker's
//! summary is told from a Blue Raven's by its `Launch time UTC`.
//!
//! **Figures.** Every row becomes a [`Figure`] in [`Stated::figures`], in the file's order: its
//! name and value as written, and the value in SI where the reader knows the figure. The unit is
//! the one the file writes after the number, read only when it is one the figure's name allows,
//! as a header's unit is read for a column. Every length takes `feet` or `ft`, every speed
//! `feet/sec` or `ft/sec`; the table gives the words the sample files write:
//!
//! | Figures | Quantity | Unit as written |
//! |---|---|---|
//! | `Max Altitude`, `Main chute deploy altitude` | [`Quantity::AltitudeAboveGround`] | `feet`, × 0.3048 m |
//! | `Inertial navigation max alt` | [`Quantity::InertialAltitude`] | `feet` |
//! | `Pad altitude ASL` (a Blue Raven's), `Pad altitude` (a tracker's) | [`Quantity::AltitudeMsl`], [`Quantity::GnssAltitude`] | `feet`, `ft` |
//! | `Distance at launch`, `… at apogee`, `… at landing` | [`Quantity::Distance`] | `ft` |
//! | `Max velocity`, `Drogue descent rate`, a tracker's `Max vertical velocity` and the other velocities | [`Quantity::Speed`] | `feet/sec`, `ft/sec`, × 0.3048 m/s |
//! | `Max motor burn acceleration`, `Peak drag deceleration`, `Apo channel max accel` and the other outputs', `Max landing accel` | [`Quantity::Acceleration`] | `Gs`, × 9.806 65 m/s² (standard gravity) |
//! | `Temperature on pad`, `Max Temperature`, `Min Temperature` | [`Quantity::Temperature`] | `F`, degrees Fahrenheit |
//! | `Battery voltage on pad`, `Min Battery voltage`, `Battery voltage at landing` | [`Quantity::Voltage`] | `V` |
//! | `Time to first burnout`, `Time to Apo channel fire` and the other outputs', `Time to gyro overload` | [`Quantity::Duration`] | `sec` |
//! | `Launch rod angle`, `Tilt angle at burnout` | [`Quantity::Angle`] | `deg` |
//! | `Roll rate at burnout` | [`Quantity::AngularRate`] | `deg/sec` |
//! | `Pad gyro rates (deg/sec)` | [`Quantity::AngularRate`], one value an axis | its name's `deg/sec` |
//! | `Pad Latitude`, `Pad Longtitude` (sic) | [`Quantity::Latitude`], [`Quantity::Longitude`] | `deg` |
//! | `Thrust tilt efficiency` | [`Quantity::Number`], a fraction | `%`, ÷ 100 |
//! | `Flight number`, `Number of ascent rolls` | [`Quantity::Number`] | none |
//! | `Direction of 1st motion`, `Gravity direction on pad` | [`Quantity::Number`], one value an axis | none: a direction of unit length |
//! | `Pad Low-G Accel`, `Pad Hi-G accel`, `Low-G accel pad offset`, `Hi-G accel pad offset` | [`Quantity::Unrecognized`], as written, one value an axis | none written (below) |
//! | `Rocket Name`, `Firmware`, `Serial number`, `Launch date`, `Launch time`, `Landing Time`, `Build date`, `Build time`, `Launch time UTC`, `Launch time local time zone` | text, `None` | |
//!
//! The manual's description of the same summary as the logger's USB port prints it (p31–33)
//! names its fields' units, and their words agree with the app's (the USB form scales some, such
//! as accelerations in g × 10, which isn't compared): the maximum altitude and the
//! pad's altitude in feet, the horizontal velocity at apogee, the drogue's velocity and the
//! maximum velocity in feet a second, the motor's, drag's, deployments' and landing's
//! accelerations in g, the tilt at burnout in degrees and the tilt efficiency in percent. Two
//! figures the app writes are kept as written instead:
//!
//! - *`Main chute descent rate`*, whose value the app writes in `feet`, a length, where the
//!   manual gives the USB summary's average descent under the main in "feet/second x 10". A
//!   figure whose unit isn't one its name allows is kept as written, [`Quantity::Unrecognized`],
//!   with a note, and isn't guessed into a speed.
//! - *The four pad accelerations and offsets*, each three numbers with no unit. The manual gives the
//!   USB summary's in "units of Gs x 10,000"; the app's are not scaled so, and no source states
//!   what they are in.
//!
//! A name the reader doesn't know is kept as written too: as numbers, [`Quantity::Unrecognized`],
//! if every cell is one, else as text. Each figure kept as written has a note, such as
//! `` `Main chute descent rate` kept as written, in a unit that isn't a speed: the file writes
//! `-18.0 feet` ``; for a name the reader doesn't know, `a figure HPR Sim doesn't know kept as
//! written: …`, the name after the colon, as the survey's public report counts each note by the
//! words before it. A temperature below absolute zero, and a latitude or longitude past its
//! range, is kept as written too.
//!
//! **What the summary states.** The record's [`Stated::apogee_m`] is `Max Altitude` in SI: the
//! barometer's altitude above the pad for a Blue Raven, and for a tracker its receiver's altitude
//! above the `Pad altitude` it states. [`Stated::ground_elevation_msl_m`] is `Pad altitude ASL`,
//! a Blue Raven's barometric altitude of the pad ("based on the barometric pressure", p33), or a
//! tracker's `Pad altitude`, its receiver's. The serial number and firmware are the rows' text as
//! written, and the flight number `Flight number` where it is a whole number. HPR Sim's
//! cross-check of these against the file beside each summary is in `cargo xtask logs`' report.
//!
//! **No samples.** The record's clock, altitude track and channels are empty, as a GPS log's
//! barometric track is, with a note saying so: the samples are in the file exported beside it,
//! the Blue Raven's low-rate CSV ([`crate::blueraven`]) or the tracker's CSV ([`crate::fwgps`]).
//!
//! **Cells and rows.** Cells are separated by commas and trimmed of spaces; empty cells at a
//! row's end are dropped, as some files pad every row to four cells. Blank lines are skipped;
//! lines may end in CR, LF or both; a byte-order mark is dropped. A row whose name is empty or a
//! number is refused, as is a name stated twice and a value too large for its unit's conversion.
//! A row of nothing but commas, padding, is skipped as a blank line is. A figure's value as
//! written is the row's text after the name, padding dropped from its end.
//! A known figure whose cells aren't the numbers it needs, such as a direction with two
//! components, is kept as written, with a note.
//!
//! **Where this comes from.** The layout, from the files themselves and the manual as cited;
//! the summary's names, and the `Main chute descent rate` kept out of the speeds, agree with
//! Debrief's reader (`lib/parsers/deviceSummary.ts`, MIT, the project owner's own; see
//! `THIRD-PARTY-NOTICES.md`). Where this reader departs from Debrief's: it keeps every row, where
//! Debrief keeps the few it compares; it keeps a stated zero, where Debrief drops it as "no
//! measurement": the file's zero is what it says, and what a zero means is a reading, milestone
//! [M7.2]'s; it keeps each sign as written, where Debrief takes the descent rates' and the
//! deployments' magnitudes; it reads only the unit words the sample files write, where Debrief
//! resolves any unit it knows, as the app "can be set to metric", so a metric summary's figures
//! are kept as written here, with notes, until a sample of one is seen; and it recognizes a
//! summary by its `Rocket Name` first and a figure only a summary states, where Debrief also
//! counts its rows of names and values.
//!
//! [M7.2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2

use std::collections::HashMap;

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Figure, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, finite, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Featherweight summary CSV";

/// How many lines that aren't blank, from the top, [`recognizes`] looks through for a figure only
/// a summary states: the files hold 14 to 48 rows.
const RECOGNITION_LINES: usize = 200;

/// How a unit as written converts to SI.
#[derive(Debug, Clone, Copy)]
enum Conversion {
    /// Multiply by this.
    Scale(f64),
    /// Degrees Fahrenheit to kelvins.
    Fahrenheit,
}

impl Conversion {
    fn apply(self, value: f64) -> f64 {
        match self {
            Self::Scale(scale) => value * scale,
            Self::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }
}

/// The unit words a figure of a quantity may carry, each with its conversion. A word is matched
/// as written, case and all: the files write these and no others.
const LENGTH: &[(&str, Conversion)] = &[
    ("feet", Conversion::Scale(FOOT_M)),
    ("ft", Conversion::Scale(FOOT_M)),
];
const SPEED: &[(&str, Conversion)] = &[
    ("feet/sec", Conversion::Scale(FOOT_M)),
    ("ft/sec", Conversion::Scale(FOOT_M)),
];
const ACCELERATION: &[(&str, Conversion)] = &[("Gs", Conversion::Scale(STANDARD_GRAVITY_MPS2))];
const TEMPERATURE: &[(&str, Conversion)] = &[("F", Conversion::Fahrenheit)];
const VOLTAGE: &[(&str, Conversion)] = &[("V", Conversion::Scale(1.0))];
const DURATION: &[(&str, Conversion)] = &[("sec", Conversion::Scale(1.0))];
const DEGREES: &[(&str, Conversion)] = &[("deg", Conversion::Scale(1.0))];
const DEGREES_PER_SECOND: &[(&str, Conversion)] = &[("deg/sec", Conversion::Scale(1.0))];
const PERCENT: &[(&str, Conversion)] = &[("%", Conversion::Scale(0.01))];

/// What a figure's name says it is.
#[derive(Debug, Clone, Copy)]
enum Kind {
    /// Text, such as a date or a name.
    Text,
    /// One number, its unit one of `units` written after it.
    Measured {
        quantity: Quantity,
        what: &'static str,
        units: &'static [(&'static str, Conversion)],
    },
    /// `components` numbers with no unit written, in `quantity`'s unit as they are.
    Bare {
        quantity: Quantity,
        components: usize,
    },
    /// Three numbers no source gives a unit for, kept as written.
    Unstated,
}

/// A figure's name as the reader matches it: lowercase, its spaces run together.
fn key(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// What the figure of a name (as [`key`] gives it) is; `None` for a name the reader doesn't know.
fn kind(key: &str) -> Option<Kind> {
    use Quantity as Q;
    let measured = |quantity, what, units| Kind::Measured {
        quantity,
        what,
        units,
    };
    let length = |quantity| measured(quantity, "length", LENGTH);
    Some(match key {
        "rocket name"
        | "firmware"
        | "serial number"
        | "searial number"
        | "launch date"
        | "launch time"
        | "landing time"
        | "build date"
        | "build time"
        | "launch time utc"
        | "launch time local time zone" => Kind::Text,
        "max altitude" | "main chute deploy altitude" => length(Q::AltitudeAboveGround),
        "inertial navigation max alt" => length(Q::InertialAltitude),
        "pad altitude asl" => length(Q::AltitudeMsl),
        "pad altitude" => length(Q::GnssAltitude),
        "distance at launch" | "distance at apogee" | "distance at landing" => length(Q::Distance),
        "max velocity"
        | "max vertical velocity"
        | "max horizontal velocity"
        | "horiz. velocity at apogee"
        | "horizontal velocity at apogee"
        | "horizontal velocity at laniding"
        | "horizontal velocity at landing"
        | "vertical velocity at landing"
        | "drogue descent rate"
        | "main chute descent rate" => measured(Q::Speed, "speed", SPEED),
        "max motor burn acceleration"
        | "peak drag deceleration"
        | "apo channel max accel"
        | "main channel max accel"
        | "3rd channel max accel"
        | "4th channel max accel"
        | "max landing accel" => measured(Q::Acceleration, "acceleration", ACCELERATION),
        "temperature on pad" | "max temperature" | "min temperature" => {
            measured(Q::Temperature, "temperature", TEMPERATURE)
        }
        "battery voltage on pad" | "min battery voltage" | "battery voltage at landing" => {
            measured(Q::Voltage, "voltage", VOLTAGE)
        }
        "time to first burnout"
        | "time to apo channel fire"
        | "time to main channel fire"
        | "time to third channel fire"
        | "time to fourth channel fire"
        | "time to gyro overload"
        | "time to gyro over-range" => measured(Q::Duration, "length of time", DURATION),
        "launch rod angle" | "tilt angle at burnout" => measured(Q::Angle, "angle", DEGREES),
        "roll rate at burnout" => measured(Q::AngularRate, "rate of turn", DEGREES_PER_SECOND),
        "pad latitude" => measured(Q::Latitude, "latitude", DEGREES),
        "pad longtitude" | "pad longitude" => measured(Q::Longitude, "longitude", DEGREES),
        "thrust tilt efficiency" => measured(Q::Number, "percentage", PERCENT),
        "flight number" | "number of ascent rolls" => Kind::Bare {
            quantity: Q::Number,
            components: 1,
        },
        "direction of 1st motion" | "gravity direction on pad" => Kind::Bare {
            quantity: Q::Number,
            components: 3,
        },
        "pad gyro rates (deg/sec)" => Kind::Bare {
            quantity: Q::AngularRate,
            components: 3,
        },
        "pad low-g accel"
        | "pad hi-g accel"
        | "low-g accel pad offset"
        | "hi-g accel pad offset" => Kind::Unstated,
        _ => return None,
    })
}

/// A row's first cell, trimmed.
fn first_cell(line: &str) -> &str {
    line.split(',').next().unwrap_or("").trim()
}

/// Whether `text` is a Featherweight summary: a first line that isn't blank naming `Rocket
/// Name`, and `Max Altitude` or `Launch time UTC` named within the first [`RECOGNITION_LINES`].
pub(crate) fn recognizes(text: &str) -> bool {
    let mut names = numbered_lines(text)
        .map(|(_, line)| line.trim())
        .filter(|line| !line.is_empty())
        .take(RECOGNITION_LINES)
        .map(|line| key(first_cell(line)));
    names.next().is_some_and(|name| name == "rocket name")
        && names.any(|name| name == "max altitude" || name == "launch time utc")
}

/// A cell as a number and the unit written after it, such as `2500.25 feet` as 2500.25 and
/// `feet`; `None` if it doesn't open with a finite number.
fn measured_cell(cell: &str) -> Option<(f64, &str)> {
    let bytes = cell.as_bytes();
    let mut end = 0;
    while let Some(&byte) = bytes.get(end) {
        // An exponent's `e` belongs to the number only before a digit or a sign: `feet` doesn't.
        let exponent = matches!(byte, b'e' | b'E')
            && bytes
                .get(end + 1)
                .is_some_and(|next| next.is_ascii_digit() || matches!(next, b'-' | b'+'));
        if !(byte.is_ascii_digit() || matches!(byte, b'-' | b'+' | b'.') || exponent) {
            break;
        }
        end += 1;
    }
    let value = number(&cell[..end], false)?;
    Some((value, cell[end..].trim()))
}

/// Reads a Featherweight summary.
///
/// # Errors
///
/// [`LogError::Unrecognized`] if the text isn't a summary; [`LogError::Syntax`] for a row whose
/// name is empty or a number, a name stated twice, or a value too large for its unit's
/// conversion to SI.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::Unrecognized {
            message: format!(
                "not a {FORMAT}: its first line doesn't name `Rocket Name`, or no line names \
                 `Max Altitude` or `Launch time UTC`"
            ),
        });
    }
    let mut figures = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    let mut lines_of: HashMap<String, usize> = HashMap::new();
    let mut notes = Notes::default();
    for (line, row) in numbered_lines(text) {
        // A row padded to four cells may be all padding, `,,,`: blank, as an empty line is.
        if row
            .trim_matches(|c: char| c == ',' || c.is_whitespace())
            .is_empty()
        {
            continue;
        }
        let (name, value) = row.split_once(',').unwrap_or((row, ""));
        let name = name.trim();
        // The value as written, but for the padding: empty cells at the row's end dropped.
        let value = value
            .trim_end_matches(|c: char| c == ',' || c.is_whitespace())
            .trim();
        let cells: Vec<&str> = if value.is_empty() {
            Vec::new()
        } else {
            value.split(',').map(str::trim).collect()
        };
        if name.is_empty() || number(name, false).is_some() {
            return Err(syntax(
                line,
                format!(
                    "a summary row opens with its figure's name, and this one opens with `{}`",
                    short(name)
                ),
            ));
        }
        let key = key(name);
        if let Some(first) = lines_of.insert(key.clone(), line) {
            return Err(syntax(
                line,
                format!(
                    "`{}` is stated twice, on line {first} and on this one",
                    short(name)
                ),
            ));
        }
        figures.push(figure(name, value, &cells, kind(&key), line, &mut notes)?);
        keys.push(key);
    }
    let tracker = keys.iter().any(|key| key == "launch time utc");
    let find = |wanted: &str| {
        keys.iter()
            .position(|key| key == wanted)
            .map(|index| &figures[index])
    };
    let si = |wanted: &str, quantity: Quantity| {
        find(wanted)
            .filter(|figure| figure.quantity == Some(quantity))
            .and_then(|figure| figure.values.first().copied())
    };
    let text_of = |wanted: &str| {
        find(wanted)
            .filter(|figure| figure.quantity.is_none() && !figure.written.is_empty())
            .map(|figure| figure.written.clone())
    };
    let stated = Stated {
        apogee_m: si("max altitude", Quantity::AltitudeAboveGround),
        ground_elevation_msl_m: if tracker {
            si("pad altitude", Quantity::GnssAltitude)
        } else {
            si("pad altitude asl", Quantity::AltitudeMsl)
        },
        samples: None,
        figures: Vec::new(),
    };
    let flight_number = si("flight number", Quantity::Number)
        .filter(|number| number.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(number))
        // A whole number in range, so the cast is exact.
        .map(|number| number as u32);
    let mut all_notes = vec![format!(
        "a summary holds no samples, so the record's clock and tracks are empty: the flight's \
         samples are in the {} exported beside it",
        if tracker {
            "tracker's CSV"
        } else {
            "Blue Raven's low-rate CSV"
        }
    )];
    all_notes.extend(notes.into_vec());
    Ok(FlightLog {
        format: LogFormat::FeatherweightSummaryCsv,
        logger: if tracker {
            "Featherweight GPS tracker"
        } else {
            "Featherweight Blue Raven"
        }
        .to_owned(),
        serial_number: text_of("serial number").or_else(|| text_of("searial number")),
        firmware: text_of("firmware"),
        flight_number,
        stated: Stated { figures, ..stated },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::new(),
        events: Vec::new(),
        notes: all_notes,
    })
}

/// One row's figure: `written` is its value as the file writes it, after the name, and `cells` the
/// value's cells, trimmed.
fn figure(
    name: &str,
    written: &str,
    cells: &[&str],
    kind: Option<Kind>,
    line: usize,
    notes: &mut Notes,
) -> Result<Figure, LogError> {
    let written = written.to_owned();
    let bare: Option<Vec<f64>> = cells.iter().map(|cell| number(cell, false)).collect();
    // Kept as written, each cell's number, its unit, if any, left in `written`.
    let numbers: Option<Vec<f64>> = cells
        .iter()
        .map(|cell| measured_cell(cell).map(|(value, _)| value))
        .collect();
    // A note's part before its first `: ` is what the survey's public report counts, so it holds
    // only the reader's own words and a name from its table; what the file writes comes after.
    let as_written = |notes: &mut Notes, why: String| {
        notes.push(match kind {
            Some(_) => format!(
                "`{}` kept as written, {why}: the file writes `{}`",
                short(name),
                short(&written)
            ),
            None => format!(
                "a figure HPR Sim doesn't know kept as written: `{}`, the file writes `{}`",
                short(name),
                short(&written)
            ),
        });
        match &numbers {
            Some(values) if !values.is_empty() => Figure::new(
                name,
                written.clone(),
                Some(Quantity::Unrecognized),
                values.clone(),
            ),
            _ => Figure::new(name, written.clone(), None, Vec::new()),
        }
    };
    let figure = match kind {
        Some(Kind::Text) => Figure::new(name, written.clone(), None, Vec::new()),
        Some(Kind::Measured {
            quantity,
            what,
            units,
        }) => match cells {
            [cell] => match measured_cell(cell) {
                Some((value, word)) => match units.iter().find(|(unit, _)| *unit == word) {
                    Some((_, conversion)) => {
                        let value = finite(conversion.apply(value), FORMAT, line, name)?;
                        let impossible = match quantity {
                            Quantity::Latitude => (value.abs() > 90.0).then_some("that large"),
                            Quantity::Longitude => (value.abs() > 180.0).then_some("that large"),
                            Quantity::Temperature => (value < 0.0).then_some("below absolute zero"),
                            _ => None,
                        };
                        if let Some(impossible) = impossible {
                            as_written(notes, format!("as no {what} is {impossible}"))
                        } else {
                            Figure::new(name, written.clone(), Some(quantity), vec![value])
                        }
                    }
                    None if word.is_empty() => {
                        as_written(notes, format!("with no unit, where a {what} needs one"))
                    }
                    None => as_written(notes, format!("in a unit that isn't a {what}")),
                },
                None => as_written(notes, "not a number".to_owned()),
            },
            [] => as_written(notes, "with no value".to_owned()),
            _ => as_written(
                notes,
                format!("as {} cells, where a {what} is one", cells.len()),
            ),
        },
        Some(Kind::Bare {
            quantity,
            components,
        }) => match &bare {
            Some(values) if values.len() == components => {
                Figure::new(name, written.clone(), Some(quantity), values.clone())
            }
            _ => as_written(
                notes,
                format!(
                    "as it isn't {components} plain number{}",
                    plural(components)
                ),
            ),
        },
        Some(Kind::Unstated) => as_written(notes, "in a unit no source states".to_owned()),
        None => as_written(notes, String::new()),
    };
    Ok(figure)
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

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

    const HEAD: &str = "Rocket Name,Demo\nMax Altitude,1000.00 feet\n";

    fn figure_of<'a>(log: &'a FlightLog, name: &str) -> &'a Figure {
        log.stated
            .figures
            .iter()
            .find(|figure| figure.name == name)
            .unwrap()
    }

    #[test]
    fn a_summary_is_told_by_its_first_name_and_a_figure_only_it_states() {
        assert!(recognizes(HEAD));
        assert!(recognizes(
            "\u{feff}\n rocket  name ,x\r\nLaunch time UTC,2026-01-01\r\n"
        ));
        // The name must open the file, and a summary's own figure must follow.
        assert!(!recognizes("Max Altitude,1000 feet\nRocket Name,Demo\n"));
        assert!(!recognizes("Rocket Name,Demo\nMax velocity,700 feet/sec\n"));
        assert!(!recognizes(""));
        // Past the first 200 lines that aren't blank, a figure isn't looked for.
        let mut far = String::from("Rocket Name,Demo\n");
        for i in 0..199 {
            far.push_str(&format!("Other {i},1\n"));
        }
        far.push_str("Max Altitude,1 feet\n");
        assert!(!recognizes(&far));
        assert!(recognizes(&far.replacen("Other 198,1\n", "", 1)));
    }

    #[test]
    fn each_unit_converts_as_its_word_says() {
        let text = format!(
            "{HEAD}Max velocity,100 feet/sec\nMax motor burn acceleration,2 Gs\n\
             Temperature on pad,212 F\nTime to first burnout,1.5 sec\n\
             Thrust tilt efficiency,99.5%\nBattery voltage on pad,4.1 V\n"
        );
        let log = read(&text).unwrap();
        let value = |name| figure_of(&log, name).values[0];
        assert_eq!(value("Max Altitude"), 1000.0 * 0.3048);
        assert_eq!(value("Max velocity"), 100.0 * 0.3048);
        assert_eq!(value("Max motor burn acceleration"), 2.0 * 9.806_65);
        assert!((value("Temperature on pad") - 373.15).abs() < 1e-12);
        assert_eq!(value("Time to first burnout"), 1.5);
        assert_eq!(value("Thrust tilt efficiency"), 99.5 * 0.01);
        assert_eq!(value("Battery voltage on pad"), 4.1);
        assert_eq!(
            figure_of(&log, "Temperature on pad").quantity,
            Some(Quantity::Temperature)
        );
    }

    #[test]
    fn a_unit_the_name_doesnt_allow_is_kept_as_written() {
        let text = format!("{HEAD}Main chute descent rate,-18.0 feet\n");
        let log = read(&text).unwrap();
        let rate = figure_of(&log, "Main chute descent rate");
        assert_eq!(rate.quantity, Some(Quantity::Unrecognized));
        assert_eq!(rate.values, [-18.0]);
        assert_eq!(rate.written, "-18.0 feet");
        assert!(
            log.notes.contains(
                &"`Main chute descent rate` kept as written, in a unit that isn't a speed: the \
              file writes `-18.0 feet`"
                    .to_owned()
            )
        );
    }

    #[test]
    fn a_length_in_meters_is_kept_as_written_not_guessed() {
        let log = read("Rocket Name,Demo\nMax Altitude,300 meters\n").unwrap();
        assert_eq!(log.stated.apogee_m, None);
        let apogee = figure_of(&log, "Max Altitude");
        assert_eq!(apogee.quantity, Some(Quantity::Unrecognized));
        assert_eq!(
            log.notes[1],
            "`Max Altitude` kept as written, in a unit that isn't a length: the file writes `300 \
             meters`"
        );
    }

    #[test]
    fn a_duplicate_name_or_a_numeric_name_is_refused() {
        let twice = read(&format!("{HEAD}MAX  altitude,1 feet\n")).unwrap_err();
        assert_eq!(
            twice.to_string(),
            "Featherweight summary CSV line 3: `MAX  altitude` is stated twice, on line 2 and on \
             this one"
        );
        let numeric = read(&format!("{HEAD}12.5,feet\n")).unwrap_err();
        assert!(
            numeric.to_string().ends_with(
                "line 3: a summary row opens with its figure's name, and this one opens with `12.5`"
            ),
            "{numeric}"
        );
        let empty = read(&format!("{HEAD},3\n")).unwrap_err();
        assert!(empty.to_string().contains("line 3:"), "{empty}");
    }

    #[test]
    fn a_value_too_large_for_its_conversion_is_refused() {
        // 1.7 × 10³⁰⁸ is finite, and × 9.806 65 isn't, written with an exponent or without.
        let plain = format!("17{}", "0".repeat(307));
        for value in ["1.7e308", plain.as_str()] {
            let error = read(&format!("{HEAD}Max landing accel,{value} Gs\n")).unwrap_err();
            assert_eq!(
                error.to_string(),
                "Featherweight summary CSV line 3: the column Max landing accel holds a value too \
                 large for its unit's conversion to SI"
            );
        }
        // A cell past the largest f64 isn't a number at all, so it is kept as written.
        let log = read(&format!("{HEAD}Max landing accel,1e309 Gs\n")).unwrap();
        assert_eq!(figure_of(&log, "Max landing accel").quantity, None);
    }

    #[test]
    fn an_exponent_is_part_of_the_number_and_feet_isnt() {
        assert_eq!(measured_cell("1.5e3 feet"), Some((1500.0, "feet")));
        assert_eq!(measured_cell("2E-1 Gs"), Some((0.2, "Gs")));
        assert_eq!(measured_cell("12feet"), Some((12.0, "feet")));
        assert_eq!(measured_cell("98.50%"), Some((98.5, "%")));
        assert_eq!(measured_cell("feet"), None);
        assert_eq!(measured_cell(""), None);
    }
}
