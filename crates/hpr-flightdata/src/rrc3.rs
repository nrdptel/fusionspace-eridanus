//! Missile Works' RRC3 flight logs, as its mDACS software exports a flight to a CSV file.
//!
//! The RRC3 is a barometric dual-deploy altimeter (*RRC3+ / RRC3 User Manual* v1.71, p. 2): it
//! records its pressure at 20 Hz, and its temperature and battery voltage at 1 Hz (p. 23).
//! mDACS works the altitude and the "displacement velocity" out of the pressure (*mDACS User
//! Manual* Rev 1.71, May 2024, p. 2) and exports "a standard Excel .CSV format", the columns shown
//! on its chart; "Time, Altitude and Pressure are always exported" (p. 10). One header line names
//! the columns; each row below is one sample:
//!
//! ```text
//! Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages
//! 0.00,0,1013.20,0,71.60, -,9.20
//! 0.05,2.71,1013.10,1.30,71.60, -,9.20
//! …
//! 52.30,0,1013.20,0, -
//! ```
//!
//! **Recognition.** A header whose names, compared without regard to case and with any bracketed
//! unit taken off, start `Time`, `Altitude`, `Pressure`, `Velocity`, and go on with any of
//! `Temperature`, `Events` and `Voltages`, each at most once. A PerfectFlite export, `Time,
//! Altitude, Velocity, …`, has no pressure and isn't one. The cells are separated by commas, or by
//! semicolons or tabs, in which case a number may be written with a decimal comma: Debrief's notes
//! describe a European-locale export, semicolon-separated with decimal commas
//! (`lib/parsers/missileworksRrc3.ts`, MIT, the project owner's own; see
//! `THIRD-PARTY-NOTICES.md`). As of 2026-10-12, every one of the 50 RRC3 files among the
//! project's sample logs is comma-separated.
//!
//! **Units.** Most exports name none; some name each in brackets, as `Time [s],Altitude
//! [ft],Pressure [mbar],Velocity [ft/s],Temperature [F],Events,Voltages [V]`. A bracketed unit is
//! taken as written, and one the reader doesn't know is refused ([`LogError::Unit`]). A unit the
//! header doesn't name is decided as the table says, and a note on the record states how and on
//! what evidence:
//!
//! | Column | Quantity | Read from | When the header names no unit |
//! |---|---|---|---|
//! | `Time` | the clock, s | `[s]` | seconds, from the step: the median step between rows within 5% of 0.05 s, the RRC3's 20 Hz pressure rate (RRC3 manual p. 23); otherwise assumed, as mDACS's chart is timed in seconds (mDACS manual p. 8) |
//! | `Altitude` | [`Quantity::AltitudeAboveGround`] | `[ft]`, `[m]` | feet or meters, from the pressure (below); feet assumed if the pressure can't tell |
//! | `Pressure` | [`Quantity::Pressure`] | `[mbar]`, `[hPa]`, × 100 Pa | hectopascals if the first row's value, on the pad, lies within 500 to 1,100 hPa (below) and the highest within the 10 to 1,200 mbar the RRC3 measures (RRC3 manual p. 9); otherwise kept as written |
//! | `Velocity` | [`Quantity::Speed`], up positive | `[ft/s]`, `[mph]`, `[m/s]`, `[km/h]` (or the RRC3's `fps`, `mps`, `kph`) | measured against the altitude (below); otherwise kept as written |
//! | `Temperature` | [`Quantity::Temperature`] | `[F]`, `[C]` | assumed: °F if the altitude is in feet, °C if in meters, as the RRC3's unit settings pair them (RRC3 manual p. 12) |
//! | `Voltages` | [`Quantity::Voltage`] | `[V]` | assumed volts: the RRC3's battery voltage (mDACS manual p. 2), from a battery of at most 10 V (RRC3 manual p. 6) |
//! | `Events` | [`FlightLog::events`] | | |
//!
//! mDACS sets the altitude's, the velocity's and the temperature's units by preference (mDACS
//! manual p. 5), and the velocity's apart from the altitude's: as of 2026-10-12, 8 of the 50 RRC3
//! files among the project's sample logs write their velocity in miles per hour beside an
//! altitude in feet. The RRC3's own settings name four sets: feet, °F and ft/s; meters, °C and
//! km/h; feet, °F and mph; meters, °C and m/s, the first the default (RRC3 manual p. 12).
//!
//! *The pressure on the pad.* The RRC3 measures 10 to 1,200 mbar, but kilopascals (about 100 at
//! sea level) and inches of mercury (about 30) lie within that too, so the first row's pressure,
//! taken on the pad, must also be a launch site's: at least 500 hPa, the U.S. Standard
//! Atmosphere's pressure about 5,570 m up, above any launch site, and at most 1,100 hPa, above
//! both the highest sea-level pressure on record below 750 m of elevation, 1,083.8 hPa (WMO,
//! *Records of Weather and Climate Extremes*, as of 31 July 2025), and the standard atmosphere's
//! 1,066 hPa at the lowest dry land, the Dead Sea's shore about 430 m below sea level. A pressure
//! outside either range is kept as written, with a note, and the altitude's unit is then assumed.
//!
//! *The altitude from the pressure.* mDACS converts pressure to altitude by NOAA's pressure
//! altitude, the standard atmosphere's (RRC3 manual p. 23), or by a geopotential method that
//! brings in the day's temperature and humidity (mDACS manual p. 5). Between the first row, on
//! the pad (the RRC3 keeps a second of samples from before launch, p. 23), and the row of lowest
//! pressure, the reader works out the climb the pressure implies in the U.S. Standard Atmosphere,
//! 1976 ([`hpr_atmos::ussa76`]) and compares the altitude column's climb between the same rows,
//! read in feet and in meters. The unit within 25% of the pressure's climb is taken: a day's air
//! from −40 °C to +50 °C is within about 20% of the standard's temperature, which is how far the
//! geopotential method can move the altitude, and feet and meters are 3.28 times apart, so at
//! most one unit can be within 25%. As of 2026-10-12, over the 50 RRC3 files among the project's
//! sample logs, feet came within 0.05% every time and meters at least 227% off. Neither within 25%
//! is refused ([`LogError::Unit`]), as is a stated unit the
//! pressure contradicts by more. Below a climb of 10 m the pressure can't settle it: a step of the
//! pressure, 0.1 hPa in every sample log, is 0.83 m of altitude at sea level, more than 8% of
//! such a climb. Then, or with a pressure of unknown unit, feet are assumed, the RRC3's default.
//!
//! *The velocity against the altitude.* The velocity is mDACS's rate of the altitude, in its own
//! unit: ft/s, mph, m/s and km/h are 3.2808, 2.2369, 1 and 3.6 times a speed in m/s. The RRC3
//! works it out through a 21-element filter at 20 Hz (RRC3 manual p. 23), so it runs behind the
//! altitude, and mDACS may smooth both for its viewer (mDACS manual p. 5). The reader integrates
//! the velocity by trapezoids from zero at the first row, on the pad, and measures the running
//! integral against the altitude above the first row, in meters, two ways:
//!
//! - *By the fit:* the least-squares slope, through zero, of the integral against the altitude as
//!   it was a delay earlier, the delay from −2 to 4 s, in steps of 0.05 s, that fits best. A
//!   velocity that is the altitude's rate a delay late integrates to the altitude that delay late,
//!   times the unit, so the delay is fitted out rather than read as a smaller slope. Smoothing
//!   still rounds off the integral's apogee, which moves the slope.
//! - *By the areas:* the area under the integral over the area under the delayed altitude, both
//!   over the record. A filter that passes a steady speed unchanged (one of unit gain) keeps the
//!   area of what it filters, however it delays or spreads it, so this measure holds exactly when
//!   the record runs from rest on the pad to rest on the ground, as the RRC3's does: it stops
//!   after the velocity has stayed above −3 ft/s for 2 s (RRC3 manual p. 10). A record that ends
//!   with the altitude still moving at a rate `v` leaves out about `σ²·v / 2` of the area, `σ²`
//!   the filter's variance. The reader takes `σ` as the delay (a first-order filter's spread
//!   equals its delay, a moving average's is 0.58 times it, a pure delay's is zero), `v` as the
//!   faster of the altitude's rates over the last second and over the second a delay earlier, and
//!   requires the share left out to be at most 0.1%; and the record must end at most halfway up
//!   the climb, as near apogee this estimate fails.
//!
//! The unit is taken only when both measures land within 3% of the same one. ft/s and km/h, the
//! nearest two, are 9.7% apart, so the runner-up is then at least 6.4% off. As of 2026-10-12, over
//! the 50 RRC3 files among the project's sample logs, 48 records end low and still enough, and on
//! each both measures came within 0.26% of the unit the file is in (ft/s or mph; 4 of them state
//! it in brackets) and put the runner-up at least 8.8% off; the other 2 records stop before
//! coming halfway down, so their velocity is kept as written. The 0.1% allowed is more than ten
//! times the largest share any of those records' ends could leave out. On invented flights 20 m
//! to 1 km up, coming down at 10 to 40 m/s, their velocity late by up to 3 s or smoothed by a
//! first-order filter or a trailing moving average over up to 3 s, in each unit, and cut off at
//! each tenth of the way through from 30%, none is decided in the wrong unit, and a whole record
//! up to 1 s late is decided from 50 m up (the module's tests; `tests/rrc3_csv.rs` adds km/h and
//! ft/s flights 1 and 2 s late, 100 to 300 m up). These filters only look back. A smoothing that
//! also looks ahead, centered on each sample, starts the velocity above rest at the record's
//! first row, so the integral misses the start of the climb: over a window of 3 to 5 s, a few in
//! a thousand such invented flights read about 8% low, enough to take km/h for ft/s. None of the
//! sample files starts that way. Otherwise the column is kept as written,
//! [`Quantity::Unrecognized`], with a note: also
//! when the altitude climbs under 50 m, where half a step of the pressure, 0.42 m at sea level,
//! is 0.8% of the climb and a larger share of the area of a flight that spends its time low; and
//! when the altitude's own unit was only assumed. A stated velocity unit that both measures
//! place within 3% of another unit is refused.
//!
//! **Events.** The `Events` column marks a deployment: ` -` for none, ` Drogue`, ` Main` or ` Aux`
//! at the sample it fired, the three events mDACS charts (mDACS manual p. 8). It holds words, not
//! numbers, so it isn't a channel: each word is an [`Event`] at its row's time, and with the
//! events the record keeps every column. A word other than those three is kept too, with a note;
//! a cell that isn't a word is refused.
//!
//! **Short rows.** The temperature and battery voltage are written on every row (their 1 Hz
//! snapshot repeated), but as of 2026-10-12, in 45 of the 50 RRC3 files among the project's sample
//! logs, the last 11 to 30 rows leave those cells out and close up the rest: a 7-column file's
//! tail rows hold 5 cells, `Time`, `Altitude`, `Pressure`, `Velocity`, `Events`, and a file with
//! one of the two columns, one cell fewer than its header. Two more pad those rows back to the
//! header's width with empty cells after `Events` instead; the 3 files with neither column have
//! no short rows. The reader takes a row as wide as the header, cell for cell; a row as wide as
//! the header without `Temperature` and `Voltages`, or as wide as the header but empty from that
//! width on, as the other columns in order, its temperature and voltage gaps (`NaN`), counted in
//! a note. So `-`, an event cell, can't land in the temperature. Some exports end with rows that
//! have no time and only an altitude cell; such rows are skipped, counted in a note. Any other
//! row of another width, a cell that isn't a number, or a time that doesn't increase, is refused,
//! naming its line. Lines may end in CR, LF or both, and a byte-order mark is dropped.
//!
//! **The primary track.** The time, the altitude, which is above the pad (the RRC3 tracks its
//! "floor" altitude on the pad, RRC3 manual p. 23), and the velocity as the vertical speed when
//! its unit was decided: it is the barometer's, as the RRC3 has no other sensor of motion. The
//! temperature and battery voltage share the clock, with gaps where a row leaves them out.
//!
//! **Where this comes from.** The columns, the units and the rules above are the manuals' and the
//! files'. Debrief's reader decides feet or meters from the pressure too, and is followed in
//! that; where this reader departs from it: it reads the velocity's unit from the data where
//! Debrief ties it to the altitude's (which reads a file in mph 32% low), it reads the short rows
//! by the rule above, and it refuses what Debrief passes over.

use hpr_atmos::ussa76::Ussa76;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{self, Notes, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "RRC3 CSV";

/// Pascals in a hectopascal, a millibar.
const HPA_PA: f64 = 100.0;

/// The pressure the RRC3 measures, mbar: below 10 or above 1,200 is a sensor fault (RRC3 manual
/// p. 9).
const PRESSURE_RANGE_HPA: (f64, f64) = (10.0, 1_200.0);

/// The pressure, hPa, the first row (on the pad) may hold for the pressure to be read in
/// hectopascals: 500 hPa is the standard atmosphere's at about 5,570 m, above any launch site, and
/// 1,100 hPa is above the highest sea-level pressure on record and the air at the lowest dry land
/// (module docs).
const PAD_RANGE_HPA: (f64, f64) = (500.0, 1_100.0);

/// The RRC3's pressure rate, 20 Hz, as a step between rows, s (RRC3 manual p. 23).
const PRESSURE_STEP_S: f64 = 0.05;

/// How far, as a fraction, the median step between rows may be from [`PRESSURE_STEP_S`] for the
/// time to be read in seconds from it.
const STEP_MARGIN: f64 = 0.05;

/// The least climb, m, from which the pressure tells feet from meters (module docs).
const SETTLING_CLIMB_M: f64 = 10.0;

/// How far, as a fraction, the altitude's climb may be from the pressure's in its unit.
const ALTITUDE_MARGIN: f64 = 0.25;

/// The least climb, m, over which the velocity's unit is measured against the altitude.
const VELOCITY_CLIMB_M: f64 = 50.0;

/// How far, as a fraction, each of the velocity's two measures may be from its unit's.
const VELOCITY_MARGIN: f64 = 0.03;

/// The earliest and latest delay, s, of the velocity behind the altitude that the fit tries, and
/// its step: the RRC3's own filter runs about half a second behind (module docs).
const DELAY_SEARCH_S: (f64, f64, f64) = (-2.0, 4.0, 0.05);

/// The highest the record may end, as a fraction of the climb above the first row, for the
/// velocity to be measured: it must have come down at least halfway (module docs).
const END_HEIGHT: f64 = 0.5;

/// The most, as a fraction, that the record ending with the altitude still moving may move the
/// velocity's measure by the areas (module docs).
const END_ERROR: f64 = 0.001;

/// The velocity units mDACS may write: the unit's words, its bracketed spellings in lower case,
/// and meters per second in one of it. A mile is 1,609.344 m and a foot 0.3048 m, exactly.
const SPEEDS: [(&str, &[&str], f64); 4] = [
    ("feet per second", &["ft/s", "fps"], FOOT_M),
    ("miles per hour", &["mph"], 0.447_04),
    ("meters per second", &["m/s", "mps"], 1.0),
    ("kilometers per hour", &["km/h", "kph"], 1.0 / 3.6),
];

/// What a header column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Time,
    Altitude,
    Pressure,
    Velocity,
    Temperature,
    Events,
    Voltages,
}

impl Kind {
    fn from_name(name: &str) -> Option<Kind> {
        [
            ("time", Kind::Time),
            ("altitude", Kind::Altitude),
            ("pressure", Kind::Pressure),
            ("velocity", Kind::Velocity),
            ("temperature", Kind::Temperature),
            ("events", Kind::Events),
            ("voltages", Kind::Voltages),
        ]
        .into_iter()
        .find(|(known, _)| name.eq_ignore_ascii_case(known))
        .map(|(_, kind)| kind)
    }

    /// Whether the RRC3 logs it at 1 Hz, so that the last rows may leave it out.
    fn is_slow(self) -> bool {
        matches!(self, Kind::Temperature | Kind::Voltages)
    }
}

/// A header column: what it holds, its name as written, and its bracketed unit if any.
struct Column {
    kind: Kind,
    written: String,
    unit: Option<String>,
}

/// A header cell's name and its bracketed unit: `Altitude [ft]` is `Altitude` and `ft`.
fn split_name(cell: &str) -> (&str, Option<&str>) {
    let cell = cell.trim();
    match (cell.find('['), cell.strip_suffix(']')) {
        (Some(open), Some(inner)) if open < inner.len() => {
            (cell[..open].trim(), Some(inner[open + 1..].trim()))
        }
        _ => (cell, None),
    }
}

/// The separator a header line uses: a semicolon or a tab if it has one, else a comma.
fn separator(header: &str) -> char {
    if header.contains(';') {
        ';'
    } else if header.contains('\t') {
        '\t'
    } else {
        ','
    }
}

/// The header's separator and columns, if it is an RRC3 export's (module docs, *Recognition*).
fn header_columns(header: &str) -> Option<(char, Vec<Column>)> {
    let separator = separator(header);
    let mut columns: Vec<Column> = Vec::new();
    for cell in header.split(separator) {
        let (name, unit) = split_name(cell);
        let kind = Kind::from_name(name)?;
        if columns.iter().any(|column| column.kind == kind) {
            return None;
        }
        let first = [Kind::Time, Kind::Altitude, Kind::Pressure, Kind::Velocity];
        match first.get(columns.len()) {
            Some(expected) if *expected != kind => return None,
            _ => {}
        }
        columns.push(Column {
            kind,
            written: cell.trim().to_owned(),
            unit: unit.map(str::to_owned),
        });
    }
    (columns.len() >= 4).then_some((separator, columns))
}

/// Whether the text is in this reader's layout, judged from its first line that isn't blank,
/// never the file's name.
pub(crate) fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .map(|(_, line)| line)
        .find(|line| !line.trim().is_empty())
        .is_some_and(|header| header_columns(header).is_some())
}

/// A length unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Length {
    Feet,
    Meters,
}

impl Length {
    fn meters(self) -> f64 {
        match self {
            Length::Feet => FOOT_M,
            Length::Meters => 1.0,
        }
    }

    fn words(self) -> &'static str {
        match self {
            Length::Feet => "feet",
            Length::Meters => "meters",
        }
    }
}

/// A temperature unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Temperature {
    Fahrenheit,
    Celsius,
}

impl Temperature {
    fn kelvin(self, value: f64) -> f64 {
        match self {
            Temperature::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
            Temperature::Celsius => value + 273.15,
        }
    }
}

/// The units the header states in brackets.
#[derive(Default)]
struct HeaderUnits {
    altitude: Option<Length>,
    pressure_hpa: bool,
    /// An index into [`SPEEDS`].
    velocity: Option<usize>,
    temperature: Option<Temperature>,
}

/// The bracketed units, each checked against what its column can be in.
fn header_units(columns: &[Column], line: usize) -> Result<HeaderUnits, LogError> {
    let mut units = HeaderUnits::default();
    for column in columns {
        let Some(unit) = column.unit.as_deref() else {
            continue;
        };
        let lower = unit.to_ascii_lowercase();
        let known = match column.kind {
            Kind::Time => lower == "s",
            Kind::Altitude => {
                units.altitude = match lower.as_str() {
                    "ft" => Some(Length::Feet),
                    "m" => Some(Length::Meters),
                    _ => None,
                };
                units.altitude.is_some()
            }
            Kind::Pressure => {
                units.pressure_hpa = lower == "mbar" || lower == "hpa";
                units.pressure_hpa
            }
            Kind::Velocity => {
                units.velocity = SPEEDS
                    .iter()
                    .position(|(_, spellings, _)| spellings.contains(&lower.as_str()));
                units.velocity.is_some()
            }
            Kind::Temperature => {
                units.temperature = match lower.trim_start_matches('°') {
                    "f" => Some(Temperature::Fahrenheit),
                    "c" => Some(Temperature::Celsius),
                    _ => None,
                };
                units.temperature.is_some()
            }
            Kind::Voltages => lower == "v",
            Kind::Events => false,
        };
        if !known {
            return Err(LogError::Unit {
                format: FORMAT,
                line,
                message: format!(
                    "the column {:?} states a unit this reader doesn't know for it",
                    short(&column.written)
                ),
            });
        }
    }
    Ok(units)
}

/// The rows read: each column's values as written (`NaN` where a short row leaves it out), and
/// what the reader noticed.
struct Rows {
    lines: Vec<usize>,
    time_s: Vec<f64>,
    /// Per header column; empty for the time and events columns.
    values: Vec<Vec<f64>>,
    events: Vec<Event>,
    short: usize,
    padded: usize,
    first_short_line: Option<usize>,
    skipped: usize,
    first_skipped_line: Option<usize>,
}

/// Reads an RRC3's mDACS CSV export from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't an mDACS export's header;
/// [`LogError::Unit`] for a bracketed unit the reader doesn't know, or a unit the pressure or the
/// altitude contradicts (module docs); [`LogError::Syntax`] for a row of the wrong width, a cell
/// that isn't a number or an event, or a time that doesn't increase, naming the line;
/// [`LogError::NoData`] if no row has a time.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    let Some((separator, columns)) = header_columns(header) else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't an mDACS export's header: `Time`, `Altitude`, \
                 `Pressure`, `Velocity`, then any of `Temperature`, `Events` and `Voltages`",
                short(header)
            ),
        });
    };
    let units = header_units(&columns, header_line)?;
    let mut notes = Notes::default();
    let rows = read_rows(lines, separator, &columns, &mut notes)?;
    if rows.time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    build(&columns, &units, rows, header_line, notes)
}

/// Reads the data rows (module docs, *Short rows*).
fn read_rows<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
    separator: char,
    columns: &[Column],
    notes: &mut Notes,
) -> Result<Rows, LogError> {
    let decimal_comma = separator != ',';
    let width = columns.len();
    // The columns a short row holds, in order.
    let fast: Vec<usize> = (0..width).filter(|&i| !columns[i].kind.is_slow()).collect();
    let has_slow = fast.len() < width;
    let mut rows = Rows {
        lines: Vec::new(),
        time_s: Vec::new(),
        values: columns.iter().map(|_| Vec::new()).collect(),
        events: Vec::new(),
        short: 0,
        padded: 0,
        first_short_line: None,
        skipped: 0,
        first_skipped_line: None,
    };
    let mut placed: Vec<Option<&str>> = vec![None; width];
    for (number, line) in lines {
        let cells: Vec<&str> = line.split(separator).map(str::trim).collect();
        let empty_from = |from: usize| {
            cells
                .get(from..)
                .is_some_and(|rest| rest.iter().all(|c| c.is_empty()))
        };
        if cells.first().is_none_or(|time| time.is_empty()) {
            // A row with no time and only an altitude, as one sample log ends with.
            let only_altitude = cells
                .iter()
                .enumerate()
                .all(|(index, cell)| index == 1 || cell.is_empty());
            if cells.len() <= width && only_altitude {
                rows.skipped += 1;
                rows.first_skipped_line.get_or_insert(number);
                continue;
            }
            return Err(syntax(number, "the row has no time".to_owned()));
        }
        placed.fill(None);
        if cells.len() == width && !(has_slow && empty_from(fast.len())) {
            for (slot, cell) in placed.iter_mut().zip(&cells) {
                *slot = Some(*cell);
            }
        } else if has_slow
            && (cells.len() == fast.len() || (cells.len() == width && empty_from(fast.len())))
        {
            for (&index, cell) in fast.iter().zip(&cells) {
                if let Some(slot) = placed.get_mut(index) {
                    *slot = Some(*cell);
                }
            }
            rows.short += 1;
            if cells.len() == width {
                rows.padded += 1;
            }
            rows.first_short_line.get_or_insert(number);
        } else {
            let without = if has_slow {
                format!(", and a row without its 1 Hz cells has {}", fast.len())
            } else {
                String::new()
            };
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells: the header names {width}{without}",
                    cells.len()
                ),
            ));
        }
        read_row(&mut rows, columns, &placed, number, decimal_comma, notes)?;
    }
    Ok(rows)
}

/// Reads one row's cells, placed under their columns (`None` for a cell the row leaves out).
fn read_row(
    rows: &mut Rows,
    columns: &[Column],
    placed: &[Option<&str>],
    number: usize,
    decimal_comma: bool,
    notes: &mut Notes,
) -> Result<(), LogError> {
    let mut time_s = f64::NAN;
    let mut event = None;
    let mut values = Vec::with_capacity(columns.len());
    for (column, cell) in columns.iter().zip(placed) {
        let Some(cell) = *cell else {
            values.push(f64::NAN);
            continue;
        };
        match column.kind {
            Kind::Events => {
                if !(cell.is_empty() || cell == "-") {
                    if !cell.bytes().all(|b| b.is_ascii_alphabetic()) {
                        return Err(syntax(
                            number,
                            format!(
                                "{:?} under {:?} isn't an event, `-` or a word such as `Drogue`",
                                short(cell),
                                short(&column.written)
                            ),
                        ));
                    }
                    event = Some(cell.to_owned());
                }
                values.push(f64::NAN);
            }
            _ => {
                let value = table::number(cell, decimal_comma).ok_or_else(|| {
                    syntax(
                        number,
                        format!(
                            "{:?} under {:?} isn't a number",
                            short(cell),
                            short(&column.written)
                        ),
                    )
                })?;
                if column.kind == Kind::Time {
                    time_s = value;
                }
                values.push(value);
            }
        }
    }
    if let Some(&last) = rows.time_s.last()
        && time_s <= last
    {
        return Err(syntax(
            number,
            format!("the time {time_s} s doesn't come after the one above it, {last} s"),
        ));
    }
    rows.lines.push(number);
    rows.time_s.push(time_s);
    for ((column, store), value) in columns.iter().zip(&mut rows.values).zip(values) {
        if !matches!(column.kind, Kind::Time | Kind::Events) {
            store.push(value);
        }
    }
    if let Some(name) = event {
        if !["Drogue", "Main", "Aux"].contains(&name.as_str()) {
            notes.push(format!(
                "line {number}'s event {:?} isn't one of the three mDACS charts, `Drogue`, \
                 `Main` and `Aux`; it was kept as written",
                short(&name)
            ));
        }
        rows.events.push(Event { time_s, name });
    }
    Ok(())
}

/// What a column was decided to be: its quantity, and its values in SI.
struct Decided {
    quantity: Quantity,
    values: Vec<f64>,
}

/// The record from the rows read: each unit decided, every channel, the primary track and the
/// notes.
fn build(
    columns: &[Column],
    units: &HeaderUnits,
    rows: Rows,
    header_line: usize,
    mut notes: Notes,
) -> Result<FlightLog, LogError> {
    let mut log = FlightLog {
        format: LogFormat::Rrc3Csv,
        logger: "Missile Works RRC3".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::new(),
        events: rows.events,
        notes: Vec::new(),
    };
    let column_of = |kind: Kind| columns.iter().position(|column| column.kind == kind);
    let values_of = |kind: Kind| column_of(kind).and_then(|index| rows.values.get(index));
    let written_of = |kind: Kind| {
        column_of(kind)
            .map(|index| columns[index].written.as_str())
            .unwrap_or_default()
    };
    // Invariant: the header has `Altitude`, `Pressure` and `Velocity` (header_columns).
    let (Some(altitude), Some(pressure), Some(velocity)) = (
        values_of(Kind::Altitude),
        values_of(Kind::Pressure),
        values_of(Kind::Velocity),
    ) else {
        return Err(syntax(header_line, "the header lacks a column".to_owned()));
    };

    let pressure = decide_pressure(pressure, units, written_of(Kind::Pressure), &mut log.notes);
    let (length, length_from_evidence) = decide_altitude(
        altitude,
        &pressure,
        units,
        &rows.lines,
        written_of(Kind::Altitude),
        header_line,
        &mut log.notes,
    )?;
    let altitude_m: Vec<f64> = altitude
        .iter()
        .map(|value| value * length.meters())
        .collect();
    let track = Track {
        time_s: &rows.time_s,
        altitude_m: &altitude_m,
        known: length_from_evidence,
    };
    let velocity = decide_velocity(
        velocity,
        &track,
        units,
        written_of(Kind::Velocity),
        header_line,
        &mut log.notes,
    )?;

    for (column, values) in columns.iter().zip(&rows.values) {
        let decided = match column.kind {
            Kind::Time => {
                if column.unit.is_none() {
                    log.notes.push(time_note(&column.written, &rows.time_s));
                }
                continue;
            }
            Kind::Events => continue,
            Kind::Altitude => Decided {
                quantity: Quantity::AltitudeAboveGround,
                values: altitude_m.clone(),
            },
            Kind::Pressure => Decided {
                quantity: pressure.quantity,
                values: pressure.values.clone(),
            },
            Kind::Velocity => Decided {
                quantity: velocity.quantity,
                values: velocity.values.clone(),
            },
            Kind::Temperature => {
                let decided =
                    decide_temperature(values, units, length, &column.written, &mut log.notes);
                log.temperature_k = Some(decided.values.clone());
                decided
            }
            Kind::Voltages => {
                if column.unit.is_none() {
                    log.notes.push(format!(
                        "`{}` read in volts, assumed: the RRC3 records its battery's voltage \
                         (mDACS manual p. 2) and takes a battery of at most 10 V (RRC3 manual \
                         p. 6); its values, as written, run {}",
                        short(&column.written),
                        span(values)
                    ));
                }
                log.battery_v = Some(values.clone());
                Decided {
                    quantity: Quantity::Voltage,
                    values: values.clone(),
                }
            }
        };
        log.channels.push(Channel {
            column: column.written.clone(),
            quantity: decided.quantity,
            time_s: rows.time_s.clone(),
            values: decided.values,
        });
    }

    if let Some(&first) = altitude_m.first()
        && first.abs() > SETTLING_CLIMB_M
    {
        notes.push(format!(
            "the altitude starts at {first:.1} m, not at the pad's zero the RRC3 measures from"
        ));
    }
    if rows.short > 0 {
        log.notes.push(format!(
            "{} rows, the first on line {}, leave out the 1 Hz {}: they are gaps{}",
            rows.short,
            rows.first_short_line.unwrap_or_default(),
            slow_names(columns),
            if rows.padded > 0 {
                format!(
                    "; {} of them pad the row to the header's width with empty cells after the \
                     others",
                    rows.padded
                )
            } else {
                String::new()
            }
        ));
    }
    if rows.skipped > 0 {
        log.notes.push(format!(
            "{} rows with no time and nothing but an altitude cell, the first on line {}, were \
             skipped",
            rows.skipped,
            rows.first_skipped_line.unwrap_or_default()
        ));
    }
    log.time_s = rows.time_s;
    log.altitude_m = altitude_m;
    if velocity.quantity == Quantity::Speed {
        log.vertical_speed_m_s = Some(velocity.values);
    }
    log.notes.extend(notes.into_vec());
    Ok(log)
}

/// The note on the time read in seconds when the header names no unit: from the step between
/// rows if it is the RRC3's 20 Hz, else assumed (module docs).
fn time_note(written: &str, time_s: &[f64]) -> String {
    let mut steps: Vec<f64> = time_s.windows(2).map(|pair| pair[1] - pair[0]).collect();
    steps.sort_by(f64::total_cmp);
    let chart = "mDACS's chart is timed in seconds (mDACS manual p. 8)";
    match steps.get(steps.len() / 2) {
        Some(&median) if (median / PRESSURE_STEP_S - 1.0).abs() <= STEP_MARGIN => format!(
            "`{}` read in seconds, from the step: the median step between rows, {median} as \
             written, is within {:.0}% of the {PRESSURE_STEP_S} s of the 20 Hz the RRC3 records \
             its pressure at (RRC3 manual p. 23)",
            short(written),
            STEP_MARGIN * 100.0
        ),
        Some(&median) => format!(
            "`{}` read in seconds, assumed: the median step between rows, {median} as written, \
             isn't within {:.0}% of the {PRESSURE_STEP_S} s of the 20 Hz the RRC3 records its \
             pressure at (RRC3 manual p. 23); {chart}",
            short(written),
            STEP_MARGIN * 100.0
        ),
        None => format!(
            "`{}` read in seconds, assumed: a single row has no step to measure; {chart}",
            short(written)
        ),
    }
}

/// The 1 Hz columns the header names, as a note lists them.
fn slow_names(columns: &[Column]) -> String {
    let names: Vec<String> = columns
        .iter()
        .filter(|column| column.kind.is_slow())
        .map(|column| format!("`{}`", short(&column.written)))
        .collect();
    match names.as_slice() {
        [one] => format!("{one} cell"),
        _ => format!("{} cells", names.join(" and ")),
    }
}

/// The least and greatest of a column's values that are numbers, as a note writes them.
fn span(values: &[f64]) -> String {
    let (low, high) = values
        .iter()
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &value| {
            (low.min(value), high.max(value))
        });
    if low.is_finite() {
        format!("from {low} to {high}")
    } else {
        "to no number".to_owned()
    }
}

/// The pressure's unit: hectopascals if stated, or if the first row's value is a launch site's
/// and the highest is within what the RRC3 measures; otherwise kept as written (module docs).
fn decide_pressure(
    values: &[f64],
    units: &HeaderUnits,
    written: &str,
    notes: &mut Vec<String>,
) -> Decided {
    let in_hpa = Decided {
        quantity: Quantity::Pressure,
        values: values.iter().map(|value| value * HPA_PA).collect(),
    };
    if units.pressure_hpa {
        return in_hpa;
    }
    let highest = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let pad = values.first().copied().unwrap_or(f64::NAN);
    let (low, high) = PRESSURE_RANGE_HPA;
    let (pad_low, pad_high) = PAD_RANGE_HPA;
    let measured = (low..=high).contains(&highest);
    let on_a_pad = (pad_low..=pad_high).contains(&pad);
    let ranges = format!(
        "the first row's, {pad}, {} the {pad_low} to {pad_high} hPa of any launch site, and the \
         highest, {highest}, {} the {low} to {high} mbar the RRC3 measures (RRC3 manual p. 9)",
        if on_a_pad {
            "lies within"
        } else {
            "lies outside"
        },
        if measured { "within" } else { "outside" }
    );
    if measured && on_a_pad {
        notes.push(format!(
            "`{}` read in hectopascals, from the values: {ranges}",
            short(written)
        ));
        in_hpa
    } else {
        notes.push(format!(
            "`{}` kept as written, its unit unknown: {ranges}",
            short(written)
        ));
        Decided {
            quantity: Quantity::Unrecognized,
            values: values.to_vec(),
        }
    }
}

/// The climb the pressure implies between the first row and the row of lowest pressure: that
/// row's index and the climb, m, in the standard atmosphere; `None` if the pressure's unit isn't
/// known or a pressure isn't positive.
fn pressure_climb(pressure: &Decided) -> Option<(usize, f64)> {
    if pressure.quantity != Quantity::Pressure {
        return None;
    }
    let atmosphere = Ussa76::standard();
    let first = *pressure.values.first()?;
    let (lowest, lowest_pa) = pressure.values.iter().copied().enumerate().fold(
        (0, first),
        |(at, low), (index, value)| {
            if value < low {
                (index, value)
            } else {
                (at, low)
            }
        },
    );
    let climb_m = atmosphere.pressure_altitude_m(lowest_pa).ok()?
        - atmosphere.pressure_altitude_m(first).ok()?;
    Some((lowest, climb_m))
}

/// The altitude's unit, and whether it was stated or decided from the pressure rather than
/// assumed (module docs).
fn decide_altitude(
    altitude: &[f64],
    pressure: &Decided,
    units: &HeaderUnits,
    lines: &[usize],
    written: &str,
    header_line: usize,
    notes: &mut Vec<String>,
) -> Result<(Length, bool), LogError> {
    let climb = pressure_climb(pressure);
    let Some((lowest, climb_m)) = climb.filter(|(_, climb_m)| *climb_m >= SETTLING_CLIMB_M) else {
        if let Some(stated) = units.altitude {
            return Ok((stated, true));
        }
        let why = match climb {
            _ if pressure.quantity != Quantity::Pressure => {
                "the pressure's unit is unknown, so it can't tell feet from meters".to_owned()
            }
            Some((_, climb_m)) => format!(
                "the pressure climbs {climb_m:.1} m, under the {SETTLING_CLIMB_M} m from which \
                 it tells feet from meters"
            ),
            None => "a pressure isn't positive, so it gives no climb".to_owned(),
        };
        notes.push(format!(
            "`{}` read in feet, assumed: {why}; feet are the RRC3's default unit (RRC3 manual p. \
             12)",
            short(written)
        ));
        return Ok((Length::Feet, false));
    };
    let (Some(&first), Some(&at_lowest)) = (altitude.first(), altitude.get(lowest)) else {
        return Ok((Length::Feet, false));
    };
    let written_climb = at_lowest - first;
    let miss = |length: Length| written_climb * length.meters() / climb_m - 1.0;
    let (feet, meters) = (miss(Length::Feet), miss(Length::Meters));
    let evidence = format!(
        "from line {} to line {} the pressure falls from {:.2} hPa to {:.2} hPa, a climb of \
         {:.1} m in the U.S. Standard Atmosphere (1976), and the altitude climbs {:.2} as \
         written, {:+.2}% off it read in feet and {:+.1}% read in meters",
        lines.first().copied().unwrap_or_default(),
        lines.get(lowest).copied().unwrap_or_default(),
        pressure.values.first().copied().unwrap_or_default() / HPA_PA,
        pressure.values.get(lowest).copied().unwrap_or_default() / HPA_PA,
        climb_m,
        written_climb,
        feet * 100.0,
        meters * 100.0
    );
    let fits = |miss: f64| miss.abs() <= ALTITUDE_MARGIN;
    if let Some(stated) = units.altitude {
        let stated_miss = miss(stated);
        if !fits(stated_miss) {
            return Err(LogError::Unit {
                format: FORMAT,
                line: header_line,
                message: format!(
                    "{:?} states {}, which the pressure contradicts: {evidence}",
                    short(written),
                    stated.words()
                ),
            });
        }
        return Ok((stated, true));
    }
    let length = if fits(feet) {
        Length::Feet
    } else if fits(meters) {
        Length::Meters
    } else {
        return Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "{:?} is in neither feet nor meters by the pressure, within {:.0}%: {evidence}",
                short(written),
                ALTITUDE_MARGIN * 100.0
            ),
        });
    };
    notes.push(format!(
        "`{}` read in {}, from the pressure: {evidence}",
        short(written),
        length.words()
    ));
    Ok((length, true))
}

/// The velocity's unit per m/s of the altitude's rate, measured two ways (module docs).
#[derive(Debug, Clone, Copy)]
struct Scale {
    /// The area under the velocity's running integral over the area under the altitude above
    /// the first row, both over the whole record.
    by_area: f64,
    /// The least-squares slope, through zero, of the running integral against the altitude above
    /// the first row taken `delay_s` earlier, the delay that fits best.
    by_fit: f64,
    /// How far, s, the velocity runs behind the altitude, as the fit finds it.
    delay_s: f64,
    /// The last row's altitude above the first row's, as a fraction of the climb.
    end_height: f64,
    /// About the most a filter as spread as the delay can move `by_area`, as a fraction, given
    /// the altitude's rate at the record's end (module docs).
    end_error: f64,
}

/// The velocity's running integral by trapezoids, zero at the first row.
fn running_integral(time_s: &[f64], velocity: &[f64]) -> Vec<f64> {
    let mut integral = 0.0;
    let mut out = Vec::with_capacity(time_s.len().min(velocity.len()));
    let mut previous: Option<(f64, f64)> = None;
    for (&time, &value) in time_s.iter().zip(velocity) {
        if let Some((before_s, before)) = previous {
            integral += 0.5 * (value + before) * (time - before_s);
        }
        out.push(integral);
        previous = Some((time, value));
    }
    out
}

/// The altitude above the first row at a time, linear between rows, and the first or the last
/// row's before or after the record. `cursor` is the row at or before the time last asked for,
/// so that asking in increasing order walks the rows once.
fn altitude_at(time_s: &[f64], above_m: &[f64], time: f64, cursor: &mut usize) -> f64 {
    while time_s.get(*cursor + 1).is_some_and(|next| *next <= time) {
        *cursor += 1;
    }
    match (
        time_s.get(*cursor),
        time_s.get(*cursor + 1),
        above_m.get(*cursor),
        above_m.get(*cursor + 1),
    ) {
        (Some(&from_s), _, Some(&from), _) if time <= from_s => from,
        (Some(&from_s), Some(&to_s), Some(&from), Some(&to)) => {
            from + (to - from) * (time - from_s) / (to_s - from_s)
        }
        _ => above_m.last().copied().unwrap_or_default(),
    }
}

/// The velocity's unit per m/s, by the areas and by the fit with a delay (module docs); `None`
/// if the altitude never leaves the first row's or a measure isn't a number.
fn velocity_scale(time_s: &[f64], altitude_m: &[f64], velocity: &[f64]) -> Option<Scale> {
    let first = *altitude_m.first()?;
    let above: Vec<f64> = altitude_m.iter().map(|value| value - first).collect();
    let integral = running_integral(time_s, velocity);
    let rows = integral.len().min(above.len());
    let (time_s, above, integral) = (&time_s[..rows], &above[..rows], &integral[..rows]);

    let (earliest, latest, step) = DELAY_SEARCH_S;
    let steps = ((latest - earliest) / step).round() as i32;
    let mut best: Option<(f64, f64, f64)> = None;
    for at in 0..=steps {
        let delay_s = earliest + f64::from(at) * step;
        let (mut product, mut square) = (0.0, 0.0);
        let mut cursor = 0;
        for (&time, &value) in time_s.iter().zip(integral) {
            let earlier = altitude_at(time_s, above, time - delay_s, &mut cursor);
            product += value * earlier;
            square += earlier * earlier;
        }
        // The residual is Σ I² − (Σ I h)² / Σ h²: the best delay explains the most of it.
        let explained = product * product / square;
        if square > 0.0 && best.is_none_or(|(most, _, _)| explained > most) {
            best = Some((explained, product / square, delay_s));
        }
    }
    let (_, by_fit, delay_s) = best?;
    // The areas, both over the record, under the integral and under the altitude as it was
    // `delay_s` earlier, by trapezoids.
    let mut cursor = 0;
    let earlier: Vec<f64> = time_s
        .iter()
        .map(|time| altitude_at(time_s, above, time - delay_s, &mut cursor))
        .collect();
    let (mut integral_area, mut altitude_area) = (0.0, 0.0);
    for index in 1..rows {
        let step_s = time_s[index] - time_s[index - 1];
        integral_area += 0.5 * (integral[index] + integral[index - 1]) * step_s;
        altitude_area += 0.5 * (earlier[index] + earlier[index - 1]) * step_s;
    }
    let by_area = integral_area / altitude_area;
    let climb_m = above.iter().copied().fold(0.0, f64::max);
    let (&end_s, &end_m) = (time_s.last()?, above.last()?);
    // The altitude's rate over the record's last second, and over the second the velocity's
    // last row reflects, `delay_s` earlier: the faster of the two.
    let span_s = (end_s - time_s.first()?).min(1.0);
    let rate_to = |to_s: f64| {
        (altitude_at(time_s, above, to_s, &mut 0)
            - altitude_at(time_s, above, to_s - span_s, &mut 0))
            / span_s
    };
    let end_rate = rate_to(end_s).abs().max(rate_to(end_s - delay_s).abs());
    let end_error = delay_s * delay_s * end_rate / (2.0 * altitude_area);
    (altitude_area > 0.0 && by_area.is_finite() && by_fit.is_finite()).then_some(Scale {
        by_area,
        by_fit,
        delay_s,
        end_height: end_m / climb_m,
        end_error: if span_s > 0.0 {
            end_error
        } else {
            f64::INFINITY
        },
    })
}

/// Each speed unit's miss from a slope, as a fraction, nearest first.
fn speed_misses(slope: f64) -> Vec<(usize, f64)> {
    let mut misses: Vec<(usize, f64)> = SPEEDS
        .iter()
        .enumerate()
        .map(|(index, (_, _, m_s))| (index, slope * m_s - 1.0))
        .collect();
    misses.sort_by(|a, b| (a.1 + 1.0).ln().abs().total_cmp(&(b.1 + 1.0).ln().abs()));
    misses
}

/// The altitude a velocity is measured against: its clock, its values in meters, and whether its
/// unit was stated or decided rather than assumed.
struct Track<'a> {
    time_s: &'a [f64],
    altitude_m: &'a [f64],
    known: bool,
}

/// The speed unit both measures land nearest, each within [`VELOCITY_MARGIN`], as an index into
/// [`SPEEDS`]; `None` if they don't, or if the record ends too high or still moving for the
/// measure by the areas to hold (module docs).
fn agreed_unit(scale: &Scale) -> Option<usize> {
    if !(scale.end_height <= END_HEIGHT && scale.end_error <= END_ERROR) {
        return None;
    }
    let (area_unit, area_miss) = *speed_misses(scale.by_area).first()?;
    let (fit_unit, fit_miss) = *speed_misses(scale.by_fit).first()?;
    (area_unit == fit_unit
        && area_miss.abs() <= VELOCITY_MARGIN
        && fit_miss.abs() <= VELOCITY_MARGIN)
        .then_some(area_unit)
}

/// A speed unit's words and its miss from each measure, as a note writes them.
fn unit_misses(index: usize, scale: &Scale) -> (&'static str, f64, f64) {
    let (words, _, m_s) = SPEEDS.get(index).copied().unwrap_or(SPEEDS[0]);
    (words, scale.by_area * m_s - 1.0, scale.by_fit * m_s - 1.0)
}

/// The velocity's unit, from the header or measured against the altitude (module docs).
fn decide_velocity(
    values: &[f64],
    track: &Track,
    units: &HeaderUnits,
    written: &str,
    header_line: usize,
    notes: &mut Vec<String>,
) -> Result<Decided, LogError> {
    let Track {
        time_s,
        altitude_m,
        known: altitude_known,
    } = *track;
    let first = altitude_m.first().copied().unwrap_or_default();
    let climb_m = altitude_m.iter().copied().fold(first, f64::max) - first;
    let scale = (altitude_known && climb_m >= VELOCITY_CLIMB_M)
        .then(|| velocity_scale(time_s, altitude_m, values))
        .flatten();
    let in_si = |index: usize| {
        let m_s = SPEEDS.get(index).map_or(1.0, |(_, _, m_s)| *m_s);
        Decided {
            quantity: Quantity::Speed,
            values: values.iter().map(|value| value * m_s).collect(),
        }
    };
    // The unit both measures land nearest, each within the margin, if there is one.
    let nearest = scale.map(|scale| {
        let by_area = speed_misses(scale.by_area);
        let by_fit = speed_misses(scale.by_fit);
        (scale, by_area, by_fit)
    });
    let decided = nearest.as_ref().and_then(|(scale, by_area, _)| {
        let index = agreed_unit(scale)?;
        Some((*scale, index, by_area.get(1).map(|(other, _)| *other)))
    });
    let measures = |scale: &Scale| {
        format!(
            "against the altitude above the first row, in meters, as it was {:.2} s earlier, \
             the area under its running integral is {:.4} times the altitude's, and the \
             integral's least-squares slope is {:.4}",
            scale.delay_s, scale.by_area, scale.by_fit
        )
    };
    if let Some(stated) = units.velocity {
        if let Some((scale, index, _)) = decided
            && index != stated
        {
            return Err(LogError::Unit {
                format: FORMAT,
                line: header_line,
                message: format!(
                    "{:?} states {}, but {}, which is {}",
                    short(written),
                    SPEEDS.get(stated).map_or("", |unit| unit.0),
                    measures(&scale),
                    SPEEDS.get(index).map_or("", |unit| unit.0)
                ),
            });
        }
        return Ok(in_si(stated));
    }
    match decided {
        Some((scale, index, runner_up)) => {
            let runner_up = runner_up
                .map(|other| {
                    let (words, by_area, by_fit) = unit_misses(other, &scale);
                    format!(
                        "; the next nearest, {words}, would miss them by {:+.1}% and {:+.1}%",
                        by_area * 100.0,
                        by_fit * 100.0
                    )
                })
                .unwrap_or_default();
            let (words, by_area, by_fit) = unit_misses(index, &scale);
            notes.push(format!(
                "`{}` read in {words}, from the altitude: over {} rows, {}; {words} would give \
                 {:.4} per m/s, {:+.2}% off the first and {:+.2}% off the second{runner_up}",
                short(written),
                time_s.len(),
                measures(&scale),
                1.0 / SPEEDS.get(index).map_or(1.0, |unit| unit.2),
                by_area * 100.0,
                by_fit * 100.0
            ));
            Ok(in_si(index))
        }
        None => {
            let why = if !altitude_known {
                "the altitude's unit is only assumed, so the velocity can't be measured against \
                 it"
                .to_owned()
            } else if climb_m < VELOCITY_CLIMB_M {
                format!(
                    "the altitude climbs {climb_m:.1} m, under the {VELOCITY_CLIMB_M} m over which \
                     the velocity is measured against it"
                )
            } else {
                match &nearest {
                    Some((scale, _, _))
                        if scale.end_height > END_HEIGHT || scale.end_height.is_nan() =>
                    {
                        format!(
                            "the record ends {:.0}% of the way up the climb, above the {:.0}% it must \
                         come down to for the velocity to be measured against the altitude",
                            scale.end_height * 100.0,
                            END_HEIGHT * 100.0
                        )
                    }
                    Some((scale, _, _))
                        if scale.end_error > END_ERROR || scale.end_error.is_nan() =>
                    {
                        format!(
                            "the record ends with the altitude still moving, which could move \
                             the area under the velocity's running integral by about {:.2}%, \
                             over the {:.1}% allowed",
                            scale.end_error * 100.0,
                            END_ERROR * 100.0
                        )
                    }
                    Some((scale, by_area, by_fit)) => {
                        let nearest_of = |misses: &[(usize, f64)]| {
                            misses.first().map_or(String::new(), |(index, miss)| {
                                format!(
                                    "{} {:+.1}%",
                                    SPEEDS.get(*index).map_or("", |unit| unit.0),
                                    miss * 100.0
                                )
                            })
                        };
                        format!(
                            "{}; the two don't both land within {:.0}% of one of ft/s, mph, \
                             m/s and km/h (the nearest {} and {})",
                            measures(scale),
                            VELOCITY_MARGIN * 100.0,
                            nearest_of(by_area),
                            nearest_of(by_fit)
                        )
                    }
                    None => {
                        "its running integral can't be measured against the altitude".to_owned()
                    }
                }
            };
            notes.push(format!(
                "`{}` kept as written, its unit unknown: {why}",
                short(written)
            ));
            Ok(Decided {
                quantity: Quantity::Unrecognized,
                values: values.to_vec(),
            })
        }
    }
}

/// The temperature's unit, from the header or assumed from the altitude's (module docs).
fn decide_temperature(
    values: &[f64],
    units: &HeaderUnits,
    length: Length,
    written: &str,
    notes: &mut Vec<String>,
) -> Decided {
    let unit = units.temperature.unwrap_or_else(|| {
        let (unit, words) = match length {
            Length::Feet => (Temperature::Fahrenheit, "degrees Fahrenheit"),
            Length::Meters => (Temperature::Celsius, "degrees Celsius"),
        };
        notes.push(format!(
            "`{}` read in {words}, assumed: the altitude is in {}, and the RRC3's unit settings \
             pair feet with °F and meters with °C (RRC3 manual p. 12), though mDACS sets each by \
             its own preference (mDACS manual p. 5) and the file states neither; its values, as \
             written, run {}",
            short(written),
            length.words(),
            span(values)
        ));
        unit
    });
    Decided {
        quantity: Quantity::Temperature,
        values: values.iter().map(|value| unit.kelvin(*value)).collect(),
    }
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

    const HEADER: &str = "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages";

    /// The pressure, hPa, at a height above the pad, m, in the standard atmosphere's troposphere
    /// (the altimeter formula of [`Ussa76::pressure_altitude_m`], inverted), the pad at sea level.
    fn pressure_hpa(height_m: f64) -> f64 {
        1_013.25 * (1.0 - height_m / 44_330.8).powf(1.0 / 0.190_263)
    }

    /// An invented flight, one row every half second: a climb at `climb_m_s` to `top_m`, then a
    /// descent at a third of it, the altitude written in `length`, the velocity's m/s multiplied by
    /// `per_m_s`, the temperature 70 and the battery 9.2; `tail` short rows at the end.
    fn flight(top_m: f64, climb_m_s: f64, length: f64, per_m_s: f64, tail: usize) -> String {
        let mut text = format!("{HEADER}\n");
        let mut heights = vec![0.0_f64];
        let mut climbing = true;
        while climbing || heights.last().is_some_and(|height| *height > 0.0) {
            let height = heights.last().copied().unwrap_or_default();
            climbing &= height < top_m;
            let speed = if climbing {
                climb_m_s
            } else {
                -climb_m_s / 3.0
            };
            heights.push((height + speed * 0.5).clamp(0.0, top_m));
        }
        heights.push(0.0);
        let count = heights.len();
        // The speed at each row: the altitude's rate across its neighbors, or to its one
        // neighbor at either end.
        let speed = |index: usize| {
            let before = index.saturating_sub(1);
            let after = (index + 1).min(count - 1);
            (heights[after] - heights[before]) / ((after - before) as f64 * 0.5)
        };
        for (index, &height) in heights.iter().enumerate() {
            let (time, speed) = (index as f64 * 0.5, speed(index));
            let event = if index == 3 { "Drogue" } else { "-" };
            let common = format!(
                "{time:.2},{:.3},{:.2},{:.3}",
                height / length,
                pressure_hpa(height),
                speed * per_m_s
            );
            if index + tail >= count {
                text.push_str(&format!("{common}, {event}\n"));
            } else {
                text.push_str(&format!("{common},70.00, {event},9.20\n"));
            }
        }
        text
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-9 * b.abs().max(1.0)
    }

    fn notes_starting<'a>(log: &'a FlightLog, start: &str) -> Vec<&'a String> {
        log.notes.iter().filter(|n| n.starts_with(start)).collect()
    }

    #[test]
    fn recognizes_only_an_mdacs_header() {
        let yes = [
            HEADER,
            "\u{feff}\r\n\r\nTime,Altitude,Pressure,Velocity,Events",
            "Time,Altitude,Pressure,Velocity,Temperature",
            "Time,Altitude,Pressure,Velocity,Voltages",
            "time,ALTITUDE,pressure,Velocity",
            "Time [s],Altitude [ft],Pressure [mbar],Velocity [ft/s],Temperature [F],Events,Voltages [V]",
            "Time;Altitude;Pressure;Velocity;Temperature;Events;Voltages",
            "Time\tAltitude\tPressure\tVelocity\tEvents",
            // An unknown unit is recognized, then refused by `read` as a unit.
            "Time,Altitude [yd],Pressure,Velocity",
        ];
        for header in yes {
            assert!(recognizes(header), "{header:?}");
        }
        let no = [
            "",
            // PerfectFlite's exports: no pressure.
            "Time,Altitude,Velocity,Temperature (F),Voltage",
            "Time, Altitude, Velocity",
            "Time,Altitude,Pressure",
            "Time,Pressure,Altitude,Velocity",
            "Time,Altitude,Pressure,Velocity,Temperature,Temperature",
            "Time,Altitude,Pressure,Velocity,Accel",
            "Time,Altitude,Pressure,Velocity,",
            "# Time,Altitude,Pressure,Velocity",
            "Time,Altitude [ft,Pressure,Velocity",
        ];
        for header in no {
            assert!(!recognizes(header), "{header:?}");
        }
        // Only the first line that isn't blank is looked at.
        assert!(!recognizes("x\nTime,Altitude,Pressure,Velocity\n"));
    }

    /// The common file: feet and ft/s decided from the file, °F and volts assumed, events at
    /// their rows, the short tail's 1 Hz cells gaps.
    #[test]
    fn decides_feet_and_feet_per_second_from_the_file() {
        let text = flight(300.0, 60.0, FOOT_M, 1.0 / FOOT_M, 2);
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::Rrc3Csv);
        let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
        assert_eq!(
            quantities,
            [
                Quantity::AltitudeAboveGround,
                Quantity::Pressure,
                Quantity::Speed,
                Quantity::Temperature,
                Quantity::Voltage
            ]
        );
        // Written to three decimals: 98.425 ft and 196.850 ft/s.
        assert!(close(log.altitude_m[1], 98.425 * FOOT_M));
        assert!(close(
            log.vertical_speed_m_s.as_deref().unwrap()[1],
            196.85 * FOOT_M
        ));
        let temperature = log.temperature_k.as_deref().unwrap();
        // 70 °F is 294.261 K (21.111 °C).
        assert!(
            (temperature[0] - 294.261).abs() < 1e-3,
            "{}",
            temperature[0]
        );
        let n = log.time_s.len();
        assert!(temperature[n - 2..].iter().all(|t| t.is_nan()));
        assert!(temperature[n - 3].is_finite());
        assert_eq!(log.events.len(), 1);
        assert_eq!(
            (log.events[0].time_s, log.events[0].name.as_str()),
            (1.5, "Drogue")
        );
        for start in [
            "`Altitude` read in feet, from the pressure: ",
            "`Velocity` read in feet per second, from the altitude: ",
            "`Pressure` read in hectopascals, from the values: ",
            "`Temperature` read in degrees Fahrenheit, assumed: ",
            "`Voltages` read in volts, assumed: ",
            "`Time` read in seconds, assumed: ",
            "2 rows, the first on line ",
        ] {
            assert_eq!(
                notes_starting(&log, start).len(),
                1,
                "{start}: {:?}",
                log.notes
            );
        }
    }

    /// Each velocity unit is told from the others, km/h from ft/s too, 9.7% apart.
    #[test]
    fn each_velocity_unit_is_told_apart() {
        for (words, _, m_s) in SPEEDS {
            let text = flight(300.0, 60.0, FOOT_M, 1.0 / m_s, 0);
            let log = read(&text).unwrap();
            let start = format!("`Velocity` read in {words}, from the altitude: ");
            assert_eq!(
                notes_starting(&log, &start).len(),
                1,
                "{words}: {:?}",
                log.notes
            );
            // Written to three decimals, within half a thousandth of the unit.
            assert!(
                (log.vertical_speed_m_s.as_deref().unwrap()[1] - 60.0).abs() < 5e-4 * m_s.max(1.0)
            );
        }
    }

    /// Meters decided from the pressure; the temperature then assumed in °C.
    #[test]
    fn decides_meters_from_the_pressure() {
        let text = flight(300.0, 60.0, 1.0, 1.0, 0);
        let log = read(&text).unwrap();
        assert!(close(log.altitude_m[1], 30.0));
        assert_eq!(
            notes_starting(&log, "`Altitude` read in meters, from the pressure: ").len(),
            1
        );
        assert!(close(log.temperature_k.as_deref().unwrap()[0], 343.15));
        assert_eq!(
            notes_starting(&log, "`Temperature` read in degrees Celsius, assumed: ").len(),
            1
        );
    }

    /// A velocity in none of the four units, or measured over too short a climb, or against an
    /// altitude whose unit is only assumed, is kept as written.
    #[test]
    fn a_velocity_that_cant_be_told_is_kept_as_written() {
        // Cut off before the descent: the last row stands above half the climb.
        let full = flight(300.0, 60.0, FOOT_M, 1.0 / FOOT_M, 0);
        let cut: String = full
            .lines()
            .take(14)
            .map(|line| format!("{line}\n"))
            .collect();
        let cases = [
            (
                flight(300.0, 60.0, FOOT_M, 3.0, 0),
                "don't both land within 3% of one",
            ),
            (
                flight(40.0, 20.0, FOOT_M, 1.0 / FOOT_M, 0),
                "under the 50 m",
            ),
            (flight(8.0, 4.0, FOOT_M, 1.0 / FOOT_M, 0), "only assumed"),
            (cut, "of the way up the climb, above the 50%"),
        ];
        for (text, why) in cases {
            let log = read(&text).unwrap();
            assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
            assert!(log.vertical_speed_m_s.is_none());
            let kept = notes_starting(&log, "`Velocity` kept as written, its unit unknown: ");
            assert_eq!(kept.len(), 1, "{:?}", log.notes);
            assert!(kept[0].contains(why), "{why}: {kept:?}");
        }
        // Under 10 m of climb, feet are assumed and said so.
        let log = read(&flight(8.0, 4.0, FOOT_M, 1.0 / FOOT_M, 0)).unwrap();
        let assumed = notes_starting(&log, "`Altitude` read in feet, assumed: ");
        assert!(assumed[0].contains("climbs 8.0 m"), "{assumed:?}");
    }

    /// A pressure outside the RRC3's range is kept as written, and the altitude assumed in feet.
    #[test]
    fn a_pressure_of_unknown_unit_is_kept_as_written() {
        let text = format!("{HEADER}\n0,0,101325,0,70, -,9.2\n0.05,1,101310,1,70, -,9.2\n");
        let log = read(&text).unwrap();
        assert_eq!(log.channels[1].quantity, Quantity::Unrecognized);
        assert_eq!(log.channels[1].values, [101_325.0, 101_310.0]);
        assert_eq!(
            notes_starting(&log, "`Pressure` kept as written, its unit unknown: ").len(),
            1
        );
        let assumed = notes_starting(&log, "`Altitude` read in feet, assumed: ");
        assert!(assumed[0].contains("unit is unknown"), "{assumed:?}");
    }

    /// Units in brackets are taken as written, cross-checked, and an unknown one refused.
    #[test]
    fn bracketed_units() {
        let text = flight(300.0, 60.0, 1.0, 3.6, 0).replacen(
            HEADER,
            "Time [s],Altitude [m],Pressure [hPa],Velocity [kph],Temperature [°F],Events,Voltages [V]",
            1,
        );
        let log = read(&text).unwrap();
        assert!(close(log.vertical_speed_m_s.as_deref().unwrap()[1], 60.0));
        // 70 °F is 294.261 K.
        assert!((log.temperature_k.as_deref().unwrap()[0] - 294.261).abs() < 1e-3);
        assert!(
            log.notes.iter().all(|n| !n.contains(" read in ")),
            "{:?}",
            log.notes
        );

        // A stated unit the file contradicts is refused as a unit.
        let wrong_altitude = text.replacen("Altitude [m]", "Altitude [ft]", 1);
        let error = read(&wrong_altitude).unwrap_err();
        assert!(
            matches!(&error, LogError::Unit { line: 1, message, .. } if message.contains("states feet, which the pressure contradicts")),
            "{error:?}"
        );
        let wrong_speed = text.replacen("Velocity [kph]", "Velocity [m/s]", 1);
        let error = read(&wrong_speed).unwrap_err();
        assert!(
            matches!(&error, LogError::Unit { line: 1, message, .. } if message.contains("\"Velocity [m/s]\" states meters per second, but against the altitude") && message.contains("which is kilometers per hour")),
            "{error:?}"
        );
        for header in [
            "Time [ms],Altitude,Pressure,Velocity",
            "Time,Altitude [yd],Pressure,Velocity",
            "Time,Altitude,Pressure [psi],Velocity",
            "Time,Altitude,Pressure,Velocity [knots]",
            "Time,Altitude,Pressure,Velocity,Temperature [K]",
            "Time,Altitude,Pressure,Velocity,Events [x]",
            "Time,Altitude,Pressure,Velocity,Voltages [mV]",
        ] {
            let error = read(&format!("{header}\n0,0,1000,0\n")).unwrap_err();
            assert!(
                matches!(&error, LogError::Unit { line: 1, message, .. } if message.contains("states a unit this reader doesn't know")),
                "{header}: {error:?}"
            );
        }
    }

    /// An altitude whose climb matches the pressure's in neither unit is refused.
    #[test]
    fn an_altitude_in_neither_unit_is_refused() {
        let text = flight(300.0, 60.0, 1.0 / 1.6, 1.0, 0);
        let error = read(&text).unwrap_err();
        assert!(
            matches!(&error, LogError::Unit { line: 1, message, .. } if message.contains("neither feet nor meters")),
            "{error:?}"
        );
    }

    /// Semicolons or tabs with decimal commas; a comma file doesn't take them.
    #[test]
    fn semicolons_and_tabs_take_decimal_commas() {
        let comma = "0,00;0;1013,25;0;21,5; -;9,2\n0,05;1,5;1013,07;2,5;21,5; Main;9,1\n";
        let log = read(&format!("{}\n{comma}", HEADER.replace(',', ";"))).unwrap();
        assert_eq!(log.time_s, [0.0, 0.05]);
        assert_eq!(log.events[0].name, "Main");
        assert!(close(log.channels[1].values[1], 101_307.0));
        let tabbed = comma.replace(';', "\t");
        let log = read(&format!("{}\n{tabbed}", HEADER.replace(',', "\t"))).unwrap();
        assert_eq!(log.battery_v.as_deref(), Some(&[9.2, 9.1][..]));
        let error = read(&format!("{HEADER}\n0,0,1013,25,0,70, -,9,2\n")).unwrap_err();
        assert!(
            error.to_string().contains("line 2: the row has 9 cells"),
            "{error}"
        );
    }

    /// Short rows: the 1 Hz cells left out, or padded back empty; `-` never a temperature.
    #[test]
    fn short_rows_leave_out_the_one_hertz_cells() {
        let text =
            format!("{HEADER}\n0,0,1000,0,70, -,9.2\n0.05,1,999.9,1, Drogue\n0.1,1,999.9,1, -,,\n");
        let log = read(&text).unwrap();
        assert_eq!(log.events.len(), 1);
        assert_eq!(log.events[0].time_s, 0.05);
        let temperature = log.temperature_k.as_deref().unwrap();
        assert!(temperature[0].is_finite() && temperature[1].is_nan() && temperature[2].is_nan());
        assert!(log.notes.iter().any(|n| n.starts_with(
            "2 rows, the first on line 3, leave out the 1 Hz `Temperature` and `Voltages` cells: \
             they are gaps; 1 of them pad"
        )));
        // One 1 Hz column alone: four cells.
        let log = read("Time,Altitude,Pressure,Velocity,Temperature\n0,0,1000,0,70\n1,1,999.9,1\n")
            .unwrap();
        assert!(log.temperature_k.as_deref().unwrap()[1].is_nan());
        // A file without 1 Hz columns has no short rows.
        let error = read("Time,Altitude,Pressure,Velocity,Events\n0,0,1000,0\n").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("line 2: the row has 4 cells: the header names 5"),
            "{error}"
        );
        // `-` where the temperature stands, in a full row, isn't a number.
        let error = read(&format!("{HEADER}\n0,0,1000,0, -, -,9.2\n")).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("line 2: \"-\" under \"Temperature\" isn't a number"),
            "{error}"
        );
        // A number where a short row's event stands isn't an event.
        let error = read(&format!("{HEADER}\n0,0,1000,0,70,,\n")).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("line 2: \"70\" under \"Events\" isn't an event"),
            "{error}"
        );
    }

    /// Rows with no time and only an altitude are skipped and counted; other rows with no time
    /// are refused.
    #[test]
    fn rows_with_no_time() {
        let text = format!("{HEADER}\n0,0,1000,0,70, -,9.2\n,4,,,,,\n,4\n");
        let log = read(&text).unwrap();
        assert_eq!(log.time_s.len(), 1);
        assert!(log.notes.iter().any(|n| n.starts_with(
            "2 rows with no time and nothing but an altitude cell, the first on line 3"
        )));
        let error = read(&format!("{HEADER}\n,4,1000,,,,\n")).unwrap_err();
        assert!(
            error.to_string().contains("line 2: the row has no time"),
            "{error}"
        );
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let cases = [
            (String::new(), "the file is empty"),
            (
                "Time,Altitude,Velocity\n0,0,0\n".to_owned(),
                "isn't an mDACS export's header",
            ),
            (format!("{HEADER}\n"), "no data rows"),
            (format!("{HEADER}\n,4\n"), "no data rows"),
            (
                format!("{HEADER}\n0,0,1000,0,70, -,9.2\n0,0,1000,0,70, -,9.2\n"),
                "line 3: the time 0 s doesn't come after the one above it, 0 s",
            ),
            (
                format!("{HEADER}\n1,0,1000,0,70, -,9.2\n0.5,0,1000,0,70, -,9.2\n"),
                "line 3: the time 0.5 s doesn't come after",
            ),
            (
                format!("{HEADER}\nx,0,1000,0,70, -,9.2\n"),
                "line 2: \"x\" under \"Time\" isn't a number",
            ),
            (
                format!("{HEADER}\n0,,1000,0,70, -,9.2\n"),
                "line 2: \"\" under \"Altitude\" isn't a number",
            ),
            (
                format!("{HEADER}\n0,0,1000,0,70, Main 2,9.2\n"),
                "line 2: \"Main 2\" under \"Events\"",
            ),
            (
                format!("{HEADER}\n0,0,1000,0,70, -,9.2,1\n"),
                "line 2: the row has 8 cells",
            ),
            (
                format!("{HEADER}\n0,0,1000\n"),
                "line 2: the row has 3 cells",
            ),
        ];
        for (text, expected) in cases {
            let error = read(&text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
    }

    /// An event word the manual doesn't name is kept, with a note; the notes stay few.
    #[test]
    fn unknown_events_are_kept_and_notes_stay_few() {
        let mut text = format!("{HEADER}\n");
        for index in 0..30 {
            text.push_str(&format!("{index},0,1000,0,70, Apogee,9.2\n"));
        }
        let log = read(&text).unwrap();
        assert_eq!(log.events.len(), 30);
        assert_eq!(log.events[0].name, "Apogee");
        let about_events = log
            .notes
            .iter()
            .filter(|n| n.contains("\"Apogee\""))
            .count();
        assert_eq!(about_events, 20);
        assert!(
            log.notes.iter().any(|n| n.starts_with("and 10 more notes")),
            "{:?}",
            log.notes
        );
    }

    /// Lines end in CR, LF or both; a byte-order mark and blank lines are dropped; a long line
    /// is quoted short.
    #[test]
    fn line_ends_and_quotes() {
        let rows = ["0,0,1000,0,70, -,9.2", "0.05,0.5,999.98,1,70, -,9.2"];
        for end in ["\r", "\n", "\r\n"] {
            let text = format!("\u{feff}{HEADER}{end}{}{end}{end}{}{end}", rows[0], rows[1]);
            let log = read(&text).unwrap();
            assert_eq!(log.time_s, [0.0, 0.05], "{end:?}");
        }
        let error = read(&format!("{HEADER}\n{}\n", "9".repeat(10_000))).unwrap_err();
        assert!(error.to_string().chars().count() < 400, "{error}");
        let error = read(&"\u{fffd}".repeat(100_000)).unwrap_err();
        assert!(error.to_string().chars().count() < 400, "{error}");
    }

    /// An invented flight sampled at 20 Hz, as (time, height, climb rate): a second on the pad, a
    /// boost at 80 m/s² and a coast at 12 m/s² to `top_m`, a fall at 9.8 m/s² to
    /// `descent_m_s`, that speed to the ground, and three seconds there.
    fn arc(top_m: f64, descent_m_s: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let (boost, coast, gravity) = (80.0, 12.0, 9.8);
        let boost_s = (top_m / (boost / 2.0 + boost * boost / (2.0 * coast))).sqrt();
        let burnout_m_s = boost * boost_s;
        let apogee_s = boost_s + burnout_m_s / coast;
        let fall_s = descent_m_s / gravity;
        let (mut times, mut heights, mut rates) = (vec![], vec![], vec![]);
        let mut landed_s = None;
        for row in 0_u32.. {
            let t = f64::from(row) * 0.05 - 1.0;
            let (height, rate): (f64, f64) = if t < 0.0 {
                (0.0, 0.0)
            } else if t < boost_s {
                (boost * t * t / 2.0, boost * t)
            } else if t < apogee_s {
                let u = t - boost_s;
                (
                    boost * boost_s * boost_s / 2.0 + burnout_m_s * u - coast * u * u / 2.0,
                    burnout_m_s - coast * u,
                )
            } else if t < apogee_s + fall_s {
                let u = t - apogee_s;
                (top_m - gravity * u * u / 2.0, -gravity * u)
            } else {
                let u = t - apogee_s - fall_s;
                (
                    top_m - gravity * fall_s * fall_s / 2.0 - descent_m_s * u,
                    -descent_m_s,
                )
            };
            let (height, rate) = if height <= 0.0 && t > 0.0 {
                (0.0, 0.0)
            } else {
                (height, rate)
            };
            if t > 0.0 && height == 0.0 {
                landed_s.get_or_insert(t);
            }
            if landed_s.is_some_and(|landed| t > landed + 3.0) {
                break;
            }
            times.push(t + 1.0);
            heights.push(height);
            rates.push(rate);
        }
        (times, heights, rates)
    }

    /// The velocity's unit is never decided wrong on invented flights 20 m to 1 km up, coming
    /// down at 10 to 40 m/s, the pressure written to 0.1 hPa and the altitude worked back from
    /// it, the velocity late by up to 3 s or smoothed over up to 3 s, in each of the four units,
    /// and each record cut off anywhere from 30% of the way through to its end. Many are left
    /// unknown; none is decided in another unit.
    #[test]
    fn the_velocity_unit_is_never_decided_wrong() {
        let atmosphere = Ussa76::standard();
        let height_of = |hpa: f64| {
            atmosphere
                .pressure_altitude_m(hpa * HPA_PA)
                .unwrap_or(f64::NAN)
        };
        let pad_m = height_of((pressure_hpa(0.0) * 10.0).round() / 10.0);
        let lags = [
            ("late", 0.0),
            ("late", 1.0),
            ("late", 2.0),
            ("late", 3.0),
            ("first-order", 0.5),
            ("first-order", 1.0),
            ("first-order", 2.0),
            ("first-order", 3.0),
            ("moving average", 1.0),
            ("moving average", 2.0),
            ("moving average", 3.0),
        ];
        let (mut decided, mut cases) = (0, 0);
        for top_m in [20.0, 30.0, 50.0, 100.0, 200.0, 300.0, 1_000.0] {
            for descent_m_s in [10.0, 25.0, 40.0] {
                let (time_s, height_m, rate_m_s) = arc(top_m, descent_m_s);
                let altitude_m: Vec<f64> = height_m
                    .iter()
                    .map(|height| {
                        let hpa = (pressure_hpa(*height) * 10.0).round() / 10.0;
                        ((height_of(hpa) - pad_m) * 100.0).round() / 100.0
                    })
                    .collect();
                for (kind, lag_s) in lags {
                    let steps = (lag_s / 0.05_f64).round() as usize;
                    let lagged: Vec<f64> = match kind {
                        "late" => (0..rate_m_s.len())
                            .map(|i| i.checked_sub(steps).map_or(0.0, |j| rate_m_s[j]))
                            .collect(),
                        "first-order" => {
                            let mut smoothed = 0.0;
                            rate_m_s
                                .iter()
                                .map(|rate| {
                                    smoothed += (rate - smoothed) * 0.05 / (lag_s + 0.05);
                                    smoothed
                                })
                                .collect()
                        }
                        _ => (0..rate_m_s.len())
                            .map(|i| {
                                let from = (i + 1).saturating_sub(steps);
                                rate_m_s[from..=i].iter().sum::<f64>() / steps as f64
                            })
                            .collect(),
                    };
                    for (unit, (words, _, m_s)) in SPEEDS.iter().enumerate() {
                        let velocity: Vec<f64> = lagged
                            .iter()
                            .map(|rate| (rate / m_s * 100.0).round() / 100.0)
                            .collect();
                        for tenths in 3..=10 {
                            let rows = time_s.len() * tenths / 10;
                            let Some(scale) =
                                velocity_scale(&time_s[..rows], &altitude_m[..rows], &velocity)
                            else {
                                continue;
                            };
                            cases += 1;
                            let agreed = agreed_unit(&scale);
                            // A whole record, its velocity up to 1 s late, is decided from 50 m up.
                            if kind == "late" && lag_s <= 1.0 && tenths == 10 && top_m >= 50.0 {
                                assert_eq!(
                                    agreed,
                                    Some(unit),
                                    "{top_m} m, {descent_m_s} m/s, {lag_s} s late, {words}: \
                                     {scale:?}"
                                );
                            }
                            if let Some(index) = agreed {
                                decided += 1;
                                assert_eq!(
                                    index, unit,
                                    "{top_m} m, {descent_m_s} m/s, {kind} {lag_s} s, {words}, \
                                     {tenths}0% of the record: {scale:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
        // The rule decides a fair share, not only the whole records.
        assert!(decided * 10 > cases, "{decided} of {cases}");
    }

    /// The two measures of the velocity's unit, on a flight whose velocity runs 1.5 s behind
    /// its altitude: the delay found, and neither measure moved by it.
    #[test]
    fn the_velocity_scale_takes_out_a_delay() {
        let time_s: Vec<f64> = (0..800).map(|i| f64::from(i) * 0.05).collect();
        // Up at 50 m/s for 5 s, down at 12.5 m/s to the pad, then at rest.
        let rate = |t: f64| match t {
            t if t < 1.0 => 0.0,
            t if t < 6.0 => 50.0,
            t if t < 26.0 => -12.5,
            _ => 0.0,
        };
        let mut altitude_m = vec![0.0];
        for pair in time_s.windows(2) {
            let last = altitude_m.last().copied().unwrap_or_default();
            altitude_m.push(last + 0.5 * (rate(pair[0]) + rate(pair[1])) * (pair[1] - pair[0]));
        }
        let velocity: Vec<f64> = time_s.iter().map(|t| rate(t - 1.5) * 2.0).collect();
        let scale = velocity_scale(&time_s, &altitude_m, &velocity).unwrap();
        assert!((scale.delay_s - 1.5).abs() < 1e-9, "{scale:?}");
        assert!((scale.by_area / 2.0 - 1.0).abs() < 1e-3, "{scale:?}");
        assert!((scale.by_fit / 2.0 - 1.0).abs() < 1e-3, "{scale:?}");
        assert!(
            scale.end_height.abs() < 1e-9 && scale.end_error < 1e-9,
            "{scale:?}"
        );
        assert!(velocity_scale(&[0.0, 1.0], &[5.0, 5.0], &[1.0, 1.0]).is_none());
    }

    /// The time read in seconds from a 20 Hz step, assumed at another step or with one row, and
    /// with no note when the header states `[s]`.
    #[test]
    fn the_time_is_read_in_seconds_from_the_step_or_assumed() {
        let rows = |step: f64, count: usize| -> String {
            (0..count)
                .map(|i| format!("{:.2},0,1000,0\n", i as f64 * step))
                .collect()
        };
        let header = "Time,Altitude,Pressure,Velocity\n";
        let log = read(&format!("{header}{}", rows(0.05, 5))).unwrap();
        let from_step = notes_starting(&log, "`Time` read in seconds, from the step: ");
        assert!(
            from_step[0].contains("step between rows, 0.05"),
            "{from_step:?}"
        );
        let log = read(&format!("{header}{}", rows(0.5, 5))).unwrap();
        let assumed = notes_starting(&log, "`Time` read in seconds, assumed: ");
        assert!(
            assumed[0].contains("0.5 as written, isn't within 5%"),
            "{assumed:?}"
        );
        let log = read(&format!("{header}{}", rows(0.05, 1))).unwrap();
        assert_eq!(
            notes_starting(&log, "`Time` read in seconds, assumed: a single row").len(),
            1
        );
        let log = read(&format!(
            "Time [s],Altitude,Pressure,Velocity\n{}",
            rows(0.05, 5)
        ))
        .unwrap();
        assert!(
            log.notes.iter().all(|n| !n.contains("`Time")),
            "{:?}",
            log.notes
        );
    }

    /// A pressure whose first row isn't a launch site's, in hectopascals, is kept as written:
    /// kilopascals (about 100) and inches of mercury (about 30) lie within what the RRC3 measures,
    /// so only the pad tells them apart; the altitude is then assumed in feet.
    #[test]
    fn a_pressure_off_the_pad_range_is_kept_as_written() {
        for (pad, next) in [
            (101.3, 101.2),
            (29.92, 29.9),
            (499.0, 498.9),
            (1_101.0, 1_100.9),
        ] {
            let text = format!("{HEADER}\n0,0,{pad},0,70, -,9.2\n0.05,1,{next},1,70, -,9.2\n");
            let log = read(&text).unwrap();
            assert_eq!(log.channels[1].quantity, Quantity::Unrecognized, "{pad}");
            assert_eq!(log.channels[1].values, [pad, next]);
            let kept = notes_starting(&log, "`Pressure` kept as written, its unit unknown: ");
            assert!(
                kept[0].contains(&format!(
                    "the first row's, {pad}, lies outside the 500 to 1100"
                )),
                "{kept:?}"
            );
            let assumed = notes_starting(&log, "`Altitude` read in feet, assumed: ");
            assert!(assumed[0].contains("unit is unknown"), "{assumed:?}");
        }
        // At either edge of the pad's range, hectopascals.
        for (pad, next) in [(500.0, 499.9), (1_100.0, 1_099.9)] {
            let text = format!("{HEADER}\n0,0,{pad},0,70, -,9.2\n0.05,1,{next},1,70, -,9.2\n");
            let log = read(&text).unwrap();
            assert_eq!(log.channels[1].quantity, Quantity::Pressure, "{pad}");
        }
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has increasing times, every
        /// channel on them.
        #[test]
        fn any_text_is_read_or_refused(rows in proptest::collection::vec("[-0-9.,; DrogueMainx]{0,32}", 0..12)) {
            let text = format!("{HEADER}\n{}", rows.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert_eq!(&channel.time_s, &log.time_s);
                    proptest::prop_assert_eq!(channel.values.len(), log.time_s.len());
                }
            }
        }
    }
}
