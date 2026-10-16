//! Tables a column mapping settles, held to physics (M7.1c22): sample tables whose headers state
//! too little or that have none, each read by the mapping stated for it in
//! `validation/log-held-mappings.json`, every unit the mapping states, and the clock's, held to a
//! physical check; and the files of the group that aren't logs ([ADR-274][adr-274]).
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use std::collections::{BTreeMap, BTreeSet, HashSet};

use hpr_flightdata::generic::{ColumnMapping, Mapping, RowForm};
use hpr_flightdata::log::{FlightLog, Quantity};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{Sample, hex, standard_height_m, teensy_median};

/// The family of tables read by a mapping held to physics, whatever the catalog files them as.
pub(super) const FAMILY: &str = "held-table";

/// The increment that reads [`FAMILY`]'s files.
pub(super) const INCREMENT: &str = "M7.1c22";

/// Why a file the catalog files as generic CSV is a table read by a mapping held to physics.
pub(super) const HELD_CSV: &str = "a table whose header states too little or that has none, read \
    by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it \
    states held to a physical check; filed as generic CSV by its extension";
/// The same, for a file the catalog files as text.
pub(super) const HELD_TXT: &str = "a table whose header states too little or that has none, read \
    by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states \
    held to a physical check; filed as text by its extension";
/// The same, for a table whose rows are written as lists.
pub(super) const HELD_LIST: &str = "a table with no header whose rows are written as lists, \
    `[…, True, True]`, read by the column mapping stated for it \
    (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as \
    text by its extension";
/// The same, for a table whose rows each open with a date and time.
pub(super) const HELD_STAMPED: &str = "a table with no header whose rows each open with a date \
    and time, `YYYY-MM-DD HH:MM:SS.ffffff: …`, read by the column mapping stated for it \
    (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as \
    text by its extension";
/// The same, for a workbook.
pub(super) const HELD_XLSX: &str = "an Excel workbook, `.xlsx`, whose first sheet is an \
    altimeter's table pasted in with cells about the flight beside it, read by the column mapping \
    stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical \
    check; filed as a spreadsheet";
/// The same, for the ground station's conversion of a SillyGoose's downlink.
pub(super) const HELD_GROUND_STATION: &str = "AerospaceNU's ground station's conversion of a \
    SillyGoose's downlink, a `CONFIG` line then 28 comma-separated columns with no header in the \
    order of the board's log with its settings, read by the column mapping stated for it \
    (`validation/log-held-mappings.json`), each unit it states held to a physical check";

/// The family of a computer's or a battery's housekeeping, not a flight's log.
pub(super) const HOUSEKEEPING_FAMILY: &str = "housekeeping-csv";
/// Why a flight computer's housekeeping isn't a log.
pub(super) const COMPUTER_HOUSEKEEPING: &str = "a flight computer's housekeeping: its \
    processor's load, its memory, disk and network counters and its core's temperature twice a \
    second, and no measurement of the flight; not a log";
/// Why a battery's housekeeping isn't a log.
pub(super) const BATTERY_HOUSEKEEPING: &str = "a battery pack's housekeeping: its cells' \
    voltages, its temperatures and its current once a second, and no measurement of the flight; \
    not a log";
/// The family of a track's coordinates copied from a map, with no clock.
pub(super) const TRACK_COPY_FAMILY: &str = "track-copy-csv";
/// Why a track's coordinates with no clock aren't a log.
pub(super) const TRACK_COPY: &str = "coordinates with no clock: a GPS track's latitude, \
    longitude and altitude as its map file holds them, the times left out; the tracker's own file \
    beside it holds the same fixes with their times; not a log";
/// The family of tables a program worked out from a log, not a logger's own record.
pub(super) const WORKED_OUT_FAMILY: &str = "worked-out-table";
/// Why a filter's output isn't a log.
pub(super) const FILTER_OUTPUT: &str = "a filter's output in a workbook: a Kalman filter's \
    state and its standard deviations by time, worked out from a log; its altitude runs to 169 km \
    and its speed to 87 km/s, so the filter diverged; not a log";
/// Why an IMU's recording turned into the rocket's frame isn't a log.
pub(super) const FRAME_OUTPUT: &str = "a filter's output: an inertial unit's recording turned \
    into the rocket's frame with gravity taken out and cut to 30 s, worked out from a recording \
    the collection doesn't hold; not a log";
/// Why a flyer's rebuild of a log isn't a log.
pub(super) const REBUILT: &str = "a flyer's rebuild of the altimeter's log beside it: rows the \
    flyer marks as made up, an altitude worked out again from the pressure and a speed from the \
    altitude; not what the logger wrote, which the survey reads; not a log";

/// The mappings, committed: a private table's column is named by the survey's own label or, where
/// its header names it, read from the file at each run; the one name a private header gives is
/// the last of a header that runs onto its first row.
const HELD: &str = include_str!("../../../validation/log-held-mappings.json");

/// The time to apogee over √(2h/g) a clock's unit must give: within a factor of 2, as ADR-274
/// asks of the Teensy logger's, where a clock misread by a power of ten is 10 or more times off.
const APOGEE_TIME_BAND: std::ops::RangeInclusive<f64> = 0.5..=2.0;

/// The band a ratio of two readings that are equal in the right units must lie in: the `.eeprom`
/// reader's climb band, 0.6 to 5/3, which no unit off by 3.28 (feet for meters) or more reaches
/// ([`hpr_flightdata::altos_eeprom::CLIMB_RATIO`]), shared with M7.1c21's checks.
const UNIT_BAND: std::ops::RangeInclusive<f64> = hpr_flightdata::altos_eeprom::CLIMB_RATIO;

/// The accelerometer's median magnitude over a recording, in g, mostly at rest on the pad and
/// under a parachute: within 15 % of 1 g, M7.1c20's band, where a unit off by 3.28 (ft/s²), 9.8
/// (g for m/s²) or a thousand (mg) is far outside.
const REST_BAND_G: std::ops::RangeInclusive<f64> = 0.85..=1.15;

/// The pressures a launch site's pad can have, Pa.
const PAD_PRESSURE_PA: std::ops::RangeInclusive<f64> = 50_000.0..=110_000.0;

/// A logger's median step over the same logger's in a file the clock check holds: within 10 %.
const CLOCK_RATE_BAND: std::ops::RangeInclusive<f64> = 0.9..=1.1;

/// The least share of the longer span two clocks of one logger must overlap by. Read right, a
/// clock overlapping another by at least half of the longer span spans at least half of it;
/// misread by a power of ten, it overlaps by a fifth of the longer span or less.
const BESIDE_OVERLAP: f64 = 0.5;

/// How long before apogee the pad is looked for, s: more than twice the time a rocket at Mach 3.5,
/// the simulator's limit, about 1.2 km/s, takes to coast to apogee against gravity alone, 122 s,
/// so that every climb in the envelope starts within it, while the drift of a barometer carried
/// to the pad, or of the weather over the hours before launch, doesn't move the pad.
const CLIMB_WINDOW_S: f64 = 300.0;

/// Liftoff: the last reading before apogee within this share of the climb above the pad.
const LIFTOFF_SHARE: f64 = 0.05;

/// The speeds below which a speed over a height's change isn't taken, m/s: near 0 the ratio is
/// all noise.
const SPEED_FLOOR_M_S: f64 = 10.0;

/// Standard gravity, m/s².
const G0: f64 = 9.806_65;

/// One file's stated mapping and its checks, from [`HELD`].
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stated {
    /// The file's id: the first 16 hex digits of its SHA-256.
    id: String,
    /// What the file is, in general words.
    what: String,
    /// For a workbook, the sheet the table is on, from 0.
    #[serde(default)]
    sheet: Option<usize>,
    mapping: StatedMapping,
    checks: Vec<Check>,
}

/// A [`Mapping`] whose columns may take their names from the file's header.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatedMapping {
    header_line: Option<usize>,
    separator: char,
    decimal_comma: bool,
    clock: usize,
    columns: Vec<StatedColumn>,
    #[serde(default)]
    clock_wrap: Option<f64>,
    #[serde(default)]
    rows: RowForm,
}

/// A [`ColumnMapping`] whose name, `None`, is the header's.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatedColumn {
    name: Option<String>,
    quantity: Option<Quantity>,
    unit: Option<String>,
    #[serde(default)]
    skip: bool,
}

/// A physical check, by the columns it holds (from 0, as the mapping counts them).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "check", rename_all = "snake_case", deny_unknown_fields)]
enum Check {
    /// The clock: the time from liftoff to apogee of a height (a pressure's standard-atmosphere
    /// height or an altitude) over √(2h/g), within [`APOGEE_TIME_BAND`].
    ApogeeTime { height: usize },
    /// The clock: another file's, of the same logger, which a clock check holds: the two files'
    /// spans of time overlap by at least [`BESIDE_OVERLAP`] of each.
    ClockBeside { file: String },
    /// The clock: its median step over a file's of the same logger, which a clock check holds,
    /// within [`CLOCK_RATE_BAND`]; for a recording with no flight in it.
    ClockRate { file: String },
    /// Every column: each row of this file is a row of another, held, value for value.
    RowsOf { file: String },
    /// A pressure: its reading at the pad ([`pad_and_top`]) within [`PAD_PRESSURE_PA`].
    PadPressure { column: usize },
    /// An altitude: its rise from the pad to its highest over its pressure's standard-atmosphere
    /// height's, within [`UNIT_BAND`].
    StandardAtmosphere { altitude: usize, pressure: usize },
    /// An altitude: its rise over another logger's in the same flight, within [`UNIT_BAND`].
    OtherLogger { altitude: usize, file: String },
    /// An altitude: its rise over the apogee the flight's flyer states, from the collection's
    /// flight card, within [`UNIT_BAND`].
    StatedApogee { altitude: usize },
    /// Accelerometer axes, or one magnitude: the median magnitude over the recording within
    /// [`REST_BAND_G`].
    Rest { columns: Vec<usize> },
    /// A track: each column on its own, the median size of its readings other than 0 lies past
    /// what radians can reach, ±π/2 for the latitude and ±π for the longitude, so the degrees are
    /// degrees. Zeros are left out as a receiver writes them before its first fix.
    Place {
        #[serde(default)]
        latitude: Option<usize>,
        longitude: usize,
    },
    /// A vertical speed: an altitude's rate of change over it, from liftoff to apogee, the median,
    /// within [`UNIT_BAND`].
    SpeedOverHeight { speed: usize, height: usize },
}

impl Check {
    fn name(&self) -> &'static str {
        match self {
            Self::ApogeeTime { .. } => "apogee time",
            Self::ClockBeside { .. } => "clock beside",
            Self::ClockRate { .. } => "clock rate",
            Self::RowsOf { .. } => "rows of",
            Self::PadPressure { .. } => "pad pressure",
            Self::StandardAtmosphere { .. } => "standard atmosphere",
            Self::OtherLogger { .. } => "other logger",
            Self::StatedApogee { .. } => "stated apogee",
            Self::Rest { .. } => "1 g at rest",
            Self::Place { .. } => "place",
            Self::SpeedOverHeight { .. } => "speed over height",
        }
    }

    /// Whether it holds the clock.
    fn holds_clock(&self) -> bool {
        matches!(
            self,
            Self::ApogeeTime { .. }
                | Self::ClockBeside { .. }
                | Self::ClockRate { .. }
                | Self::RowsOf { .. }
        )
    }

    /// The columns whose units it holds, and the quantities each must have.
    fn holds(&self) -> Vec<(usize, &'static [Quantity])> {
        const HEIGHTS: &[Quantity] = &[
            Quantity::AltitudeAboveGround,
            Quantity::AltitudeMsl,
            Quantity::GnssAltitude,
            Quantity::Altitude,
        ];
        match self {
            Self::PadPressure { column } => vec![(*column, &[Quantity::Pressure])],
            Self::StandardAtmosphere { altitude, .. }
            | Self::OtherLogger { altitude, .. }
            | Self::StatedApogee { altitude } => vec![(*altitude, HEIGHTS)],
            Self::Rest { columns } => columns
                .iter()
                .map(|c| (*c, &[Quantity::Acceleration][..]))
                .collect(),
            Self::Place {
                latitude,
                longitude,
            } => latitude
                .map(|c| (c, &[Quantity::Latitude][..]))
                .into_iter()
                .chain([(*longitude, &[Quantity::Longitude][..])])
                .collect(),
            Self::SpeedOverHeight { speed, .. } => vec![(*speed, &[Quantity::Speed])],
            Self::ApogeeTime { .. }
            | Self::ClockBeside { .. }
            | Self::ClockRate { .. }
            | Self::RowsOf { .. } => Vec::new(),
        }
    }

    /// The columns it reads but doesn't hold, and the quantities each must have.
    fn reads(&self) -> Vec<(usize, &'static [Quantity])> {
        const ALTITUDES: &[Quantity] = &[
            Quantity::AltitudeAboveGround,
            Quantity::AltitudeMsl,
            Quantity::GnssAltitude,
            Quantity::Altitude,
        ];
        const HEIGHTS: &[Quantity] = &[
            Quantity::Pressure,
            Quantity::AltitudeAboveGround,
            Quantity::AltitudeMsl,
            Quantity::GnssAltitude,
            Quantity::Altitude,
        ];
        match self {
            Self::ApogeeTime { height } => vec![(*height, HEIGHTS)],
            // A speed is held against an altitude's change, not a pressure's.
            Self::SpeedOverHeight { height, .. } => vec![(*height, ALTITUDES)],
            Self::StandardAtmosphere { pressure, .. } => vec![(*pressure, &[Quantity::Pressure])],
            _ => Vec::new(),
        }
    }
}

/// The stated mappings, each checked: an id once, a clock check, a check holding every column
/// read in a unit, each check's columns of the quantity it takes. Errors name the file.
fn stated() -> Result<Vec<Stated>, String> {
    parse(HELD)
}

/// [`stated`], from a file's text.
fn parse(text: &str) -> Result<Vec<Stated>, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct File {
        /// The file's note on itself: required, never read.
        #[serde(rename = "about")]
        _about: String,
        mappings: Vec<Stated>,
    }
    let file: File = serde_json::from_str(text)
        .map_err(|error| format!("validation/log-held-mappings.json: {error}"))?;
    let mut ids = BTreeSet::new();
    for stated in &file.mappings {
        let fail = |what: String| {
            Err(format!(
                "validation/log-held-mappings.json, {}: {what}",
                stated.id
            ))
        };
        if !ids.insert(stated.id.as_str()) {
            return fail("mapped twice".to_owned());
        }
        let columns = &stated.mapping.columns;
        if !stated.checks.iter().any(Check::holds_clock) {
            return fail("no check holds the clock".to_owned());
        }
        let every = stated
            .checks
            .iter()
            .any(|c| matches!(c, Check::RowsOf { .. }));
        let mut held = BTreeSet::new();
        for check in &stated.checks {
            for (index, wanted) in check.holds().into_iter().chain(check.reads()) {
                let quantity = columns.get(index).and_then(|c| c.quantity);
                if !quantity.is_some_and(|q| wanted.contains(&q)) {
                    return fail(format!(
                        "the {} check takes column {index}, which isn't one of {wanted:?}",
                        check.name()
                    ));
                }
            }
            held.extend(check.holds().into_iter().map(|(index, _)| index));
        }
        for (index, column) in columns.iter().enumerate() {
            if index != stated.mapping.clock
                && !column.skip
                && column.unit.is_some()
                && !every
                && !held.contains(&index)
            {
                return fail(format!("no check holds column {index}'s unit"));
            }
        }
    }
    Ok(file.mappings)
}

/// One check's outcome.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct Outcome {
    check: String,
    /// The columns it holds or reads, from 0.
    columns: Vec<usize>,
    /// What it measures, rounded to 4 decimals; `None` where the value would tell a private
    /// flight's place (a pad's pressure, a track's place) or none could be taken.
    measured: Option<f64>,
    passed: bool,
}

/// One table read by its mapping, and its checks.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct HeldFile {
    id: String,
    what: String,
    /// Why the reader refused it, if it did.
    refused: Option<String>,
    /// Rows read.
    rows: usize,
    /// Columns the mapping reads in a unit, keeps as written, and leaves out.
    read: usize,
    kept: usize,
    left_out: usize,
    /// The record's events.
    events: usize,
    outcomes: Vec<Outcome>,
    /// The record's id: the first 16 hex digits of the SHA-256 of its JSON.
    record: String,
}

/// This increment's checks.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct Checks {
    files: Vec<HeldFile>,
}

impl Checks {
    /// The file's record id, if the reader read it.
    pub(super) fn record(&self, id: &str) -> Option<&str> {
        self.files
            .iter()
            .find(|f| f.id == id && f.refused.is_none())
            .map(|f| f.record.as_str())
    }
}

fn id_of(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))[..16].to_owned()
}

/// The mapping with each name the header gives filled in.
fn resolve(stated: &Stated, text: &str) -> Result<Mapping, String> {
    let header: Option<Vec<String>> = stated.mapping.header_line.map(|line| {
        super::lines(text)
            .nth(line.saturating_sub(1))
            .unwrap_or_default()
            .split(stated.mapping.separator)
            .map(|c| c.trim().to_owned())
            .collect()
    });
    let mut columns = Vec::new();
    for (index, column) in stated.mapping.columns.iter().enumerate() {
        let name = match (&column.name, &header) {
            (Some(name), _) => name.clone(),
            (None, Some(header)) => header
                .get(index)
                .filter(|n| !n.is_empty())
                .cloned()
                .ok_or(format!("the header gives column {index} no name"))?,
            (None, None) => {
                return Err(format!(
                    "column {index} takes its name from the header, but the mapping names no \
                     header line"
                ));
            }
        };
        columns.push(ColumnMapping {
            name,
            quantity: column.quantity,
            unit: column.unit.clone(),
            skip: column.skip,
        });
    }
    let names: BTreeSet<&str> = columns.iter().map(|c| c.name.as_str()).collect();
    if names.len() != columns.len() {
        return Err("two columns have one name".to_owned());
    }
    Ok(Mapping {
        header_line: stated.mapping.header_line,
        separator: stated.mapping.separator,
        decimal_comma: stated.mapping.decimal_comma,
        clock: stated.mapping.clock,
        columns,
        clock_wrap: stated.mapping.clock_wrap,
        rows: stated.mapping.rows,
    })
}

/// The file read by its stated mapping, refused if a reader takes it on its own.
fn read(stated: &Stated, bytes: &[u8]) -> Result<(FlightLog, Mapping), String> {
    let text = match stated.sheet {
        Some(sheet) => {
            hpr_flightdata::workbook::sheets(bytes)
                .map_err(|e| e.to_string())?
                .into_iter()
                .nth(sheet)
                .ok_or(format!("the workbook has no sheet {sheet}"))?
                .text
        }
        None => hpr_flightdata::import::decode(bytes).into_owned(),
    };
    if let Ok(log) = hpr_flightdata::import::read(text.as_bytes()) {
        return Err(format!(
            "it reads as {} on its own, so a stated mapping can't be its reading",
            log.format.name()
        ));
    }
    let mapping = resolve(stated, &text)?;
    let log = hpr_flightdata::generic::read_with(&text, &mapping).map_err(|e| e.to_string())?;
    Ok((log, mapping))
}

/// A channel's readings as heights, m, in time order as the file holds them: a pressure's
/// standard-atmosphere height, or an altitude as read; readings that aren't finite, and pressures
/// of 0 or less, which aren't air's, left out.
fn heights(log: &FlightLog, name: &str) -> Vec<(f64, f64)> {
    let Some(channel) = log.channels.iter().find(|c| c.column == name) else {
        return Vec::new();
    };
    let pressure = channel.quantity == Quantity::Pressure;
    channel
        .time_s
        .iter()
        .zip(&channel.values)
        .filter(|(t, v)| t.is_finite() && v.is_finite() && (!pressure || **v > 0.0))
        .map(|(t, v)| (*t, if pressure { standard_height_m(*v) } else { *v }))
        .collect()
}

/// A record's own altitude above its pad, m: its primary track, or else its first channel of an
/// altitude above the ground, or else its first pressure's standard-atmosphere height.
fn primary(log: &FlightLog) -> Vec<(f64, f64)> {
    if !log.altitude_m.is_empty() {
        return log
            .time_s
            .iter()
            .zip(&log.altitude_m)
            .filter(|(t, h)| t.is_finite() && h.is_finite())
            .map(|(t, h)| (*t, *h))
            .collect();
    }
    [Quantity::AltitudeAboveGround, Quantity::Pressure]
        .iter()
        .find_map(|q| log.channels.iter().find(|c| c.quantity == *q))
        .map(|c| heights(log, &c.column))
        .unwrap_or_default()
}

/// The heights' running median of 5 readings, which a single wild reading doesn't move; the pad's
/// height, the lowest of it over the [`CLIMB_WINDOW_S`] before its highest, as a rocket only
/// climbs from the pad to apogee, so that a record starting at liftoff or hours before it has one;
/// and the index of its highest, the last of them where several tie.
fn pad_and_top(series: &[(f64, f64)]) -> Option<(Vec<f64>, f64, usize)> {
    // Centered, narrowing at the ends, so the first and last readings are their own.
    let smoothed: Vec<f64> = (0..series.len())
        .map(|k| {
            let half = 2.min(k).min(series.len() - 1 - k);
            let window = series[k - half..=k + half]
                .iter()
                .map(|(_, h)| *h)
                .collect();
            teensy_median(window).unwrap_or(f64::NAN)
        })
        .collect();
    let top = (0..smoothed.len()).max_by(|a, b| smoothed[*a].total_cmp(&smoothed[*b]))?;
    let from = series[top].0 - CLIMB_WINDOW_S;
    let pad_m = (0..=top)
        .filter(|k| series[*k].0 >= from)
        .map(|k| smoothed[k])
        .fold(f64::INFINITY, f64::min);
    Some((smoothed, pad_m, top))
}

/// A flight's climb, from a height series: the pad's height and apogee's ([`pad_and_top`]), and
/// the times of apogee and of liftoff, the last reading before apogee within [`LIFTOFF_SHARE`]
/// of the climb above the pad.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Climb {
    pad_m: f64,
    apogee_m: f64,
    apogee_s: f64,
    liftoff_s: f64,
}

fn climb(series: &[(f64, f64)]) -> Option<Climb> {
    let (smoothed, pad_m, top) = pad_and_top(series)?;
    let apogee_m = smoothed[top];
    let rise = apogee_m - pad_m;
    if rise.is_nan() || rise <= 0.0 {
        return None;
    }
    let liftoff = (0..top)
        .rev()
        .find(|k| smoothed[*k] <= pad_m + LIFTOFF_SHARE * rise)?;
    Some(Climb {
        pad_m,
        apogee_m,
        apogee_s: series[top].0,
        liftoff_s: series[liftoff].0,
    })
}

/// The pressure, Pa, whose standard-atmosphere height is `height_m`: the inverse of
/// [`standard_height_m`].
fn standard_pressure_pa(height_m: f64) -> f64 {
    101_325.0 * (1.0 - height_m / 44_330.77).powf(1.0 / 0.190_263)
}

fn rounded(value: f64) -> f64 {
    (value * 1e4).round() / 1e4
}

/// The first and last times of a record's channels, and its clock's median step forward.
fn span(log: &FlightLog) -> Option<(f64, f64, f64)> {
    let times: &[f64] = log
        .channels
        .first()
        .map(|c| c.time_s.as_slice())
        .filter(|t| !t.is_empty())
        .unwrap_or(&log.time_s);
    let first = times.iter().copied().fold(f64::INFINITY, f64::min);
    let last = times.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let step = teensy_median(
        times
            .windows(2)
            .map(|p| p[1] - p[0])
            .filter(|d| *d > 0.0)
            .collect(),
    )?;
    (first.is_finite() && last.is_finite()).then_some((first, last, step))
}

/// Each row's time and values, as bits, the channels in the record's order.
fn rows(log: &FlightLog) -> Vec<Vec<u64>> {
    let Some(first) = log.channels.first() else {
        return Vec::new();
    };
    (0..first.time_s.len())
        .map(|k| {
            std::iter::once(first.time_s[k].to_bits())
                .chain(
                    log.channels
                        .iter()
                        .map(|c| c.values.get(k).map_or(u64::MAX, |v| v.to_bits())),
                )
                .collect()
        })
        .collect()
}

/// The records of the other files the checks name, by id: held files first, then any file a
/// reader or a publisher's mapping reads.
struct Others<'a> {
    held: &'a BTreeMap<String, FlightLog>,
    samples: &'a [Sample],
}

impl Others<'_> {
    fn get(&self, id: &str) -> Option<FlightLog> {
        if let Some(log) = self.held.get(id) {
            return Some(log.clone());
        }
        let sample = self.samples.iter().find(|s| id_of(&s.bytes) == id)?;
        hpr_flightdata::import::read(&sample.bytes)
            .ok()
            .or_else(|| {
                super::read_mapped(id, &sample.bytes)
                    .ok()
                    .map(|(log, _)| log)
            })
    }
}

/// The apogee each flight's flyer states, m, by file id, from the collection's flight cards: read
/// at each run, as the cards are private.
fn stated_apogees() -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    let Ok(root) = crate::designs::root() else {
        return out;
    };
    let Ok(text) = std::fs::read_to_string(root.join(super::CATALOG)) else {
        return out;
    };
    let records = super::csv_records(&text);
    let Some((header, rows)) = records.split_first() else {
        return out;
    };
    let column = |name: &str| header.iter().position(|c| c == name);
    let (Some(sha), Some(flight)) = (column("sha256"), column("flight_id")) else {
        return out;
    };
    for row in rows {
        let (Some(sha), Some(flight)) = (row.get(sha), row.get(flight)) else {
            continue;
        };
        let card = root
            .join(super::COLLECTION)
            .join("catalog/flights")
            .join(format!("{flight}.json"));
        let Ok(card) = std::fs::read_to_string(card) else {
            continue;
        };
        let apogee = serde_json::from_str::<serde_json::Value>(&card)
            .ok()
            .and_then(|v| v["outcome"]["stated_apogee_m"].as_f64());
        if let (Some(apogee), Some(id)) = (apogee, sha.get(..16)) {
            out.insert(id.to_owned(), apogee);
        }
    }
    out
}

/// Runs one check on a record read by `mapping`.
fn run(
    check: &Check,
    log: &FlightLog,
    mapping: &Mapping,
    others: &Others<'_>,
    stated_apogee: Option<f64>,
) -> Outcome {
    let name = |index: usize| {
        mapping
            .columns
            .get(index)
            .map_or(String::new(), |c| c.name.clone())
    };
    let ratio_in = |value: Option<f64>, band: &std::ops::RangeInclusive<f64>| {
        (value.map(rounded), value.is_some_and(|v| band.contains(&v)))
    };
    let (columns, (measured, passed)) = match check {
        Check::ApogeeTime { height } => {
            let ratio = climb(&heights(log, &name(*height)))
                .map(|c| (c.apogee_s - c.liftoff_s) / (2.0 * (c.apogee_m - c.pad_m) / G0).sqrt());
            (vec![*height], ratio_in(ratio, &APOGEE_TIME_BAND))
        }
        Check::ClockBeside { file } => {
            let share = others.get(file).and_then(|other| {
                let (a0, a1, _) = span(log)?;
                let (b0, b1, _) = span(&other)?;
                let overlap = (a1.min(b1) - a0.max(b0)).max(0.0);
                Some(overlap / (a1 - a0).max(b1 - b0))
            });
            (
                Vec::new(),
                (
                    share.map(rounded),
                    share.is_some_and(|s| s >= BESIDE_OVERLAP),
                ),
            )
        }
        Check::ClockRate { file } => {
            let ratio = others.get(file).and_then(|other| {
                let (_, _, a) = span(log)?;
                let (_, _, b) = span(&other)?;
                Some(a / b)
            });
            (Vec::new(), ratio_in(ratio, &CLOCK_RATE_BAND))
        }
        Check::RowsOf { file } => {
            let share = others.get(file).and_then(|other| {
                let theirs: HashSet<Vec<u64>> = rows(&other).into_iter().collect();
                let ours = rows(log);
                (!ours.is_empty() && other.channels.len() == log.channels.len()).then(|| {
                    ours.iter().filter(|r| theirs.contains(*r)).count() as f64 / ours.len() as f64
                })
            });
            (
                Vec::new(),
                (share.map(rounded), share.is_some_and(|s| s == 1.0)),
            )
        }
        Check::PadPressure { column } => {
            let pad = pad_and_top(&heights(log, &name(*column)))
                .map(|(_, pad_m, _)| standard_pressure_pa(pad_m));
            (
                vec![*column],
                (None, pad.is_some_and(|p| PAD_PRESSURE_PA.contains(&p))),
            )
        }
        Check::StandardAtmosphere { altitude, pressure } => {
            let rise =
                |index: usize| climb(&heights(log, &name(index))).map(|c| c.apogee_m - c.pad_m);
            let ratio = rise(*altitude).zip(rise(*pressure)).map(|(a, p)| a / p);
            (vec![*altitude, *pressure], ratio_in(ratio, &UNIT_BAND))
        }
        Check::OtherLogger { altitude, file } => {
            let ours = climb(&heights(log, &name(*altitude))).map(|c| c.apogee_m - c.pad_m);
            let theirs = others
                .get(file)
                .and_then(|other| climb(&primary(&other)))
                .map(|c| c.apogee_m - c.pad_m);
            let ratio = ours.zip(theirs).map(|(a, b)| a / b);
            (vec![*altitude], ratio_in(ratio, &UNIT_BAND))
        }
        Check::StatedApogee { altitude } => {
            let ours = climb(&heights(log, &name(*altitude))).map(|c| c.apogee_m - c.pad_m);
            let ratio = ours.zip(stated_apogee).map(|(a, b)| a / b);
            (vec![*altitude], ratio_in(ratio, &UNIT_BAND))
        }
        Check::Rest { columns } => {
            let channels: Vec<_> = columns
                .iter()
                .filter_map(|c| log.channels.iter().find(|ch| ch.column == name(*c)))
                .collect();
            let magnitude = (channels.len() == columns.len())
                .then(|| {
                    let rows = channels.iter().map(|c| c.values.len()).min().unwrap_or(0);
                    teensy_median(
                        (0..rows)
                            .map(|k| channels.iter().map(|c| c.values[k].powi(2)).sum::<f64>())
                            .filter(|s| s.is_finite())
                            .map(|s| s.sqrt() / G0)
                            .collect(),
                    )
                })
                .flatten();
            (columns.clone(), ratio_in(magnitude, &REST_BAND_G))
        }
        Check::Place {
            latitude,
            longitude,
        } => {
            let past = |index: usize, bound: f64| {
                log.channels
                    .iter()
                    .find(|c| c.column == name(index))
                    .is_some_and(|c| {
                        let sizes: Vec<f64> = c
                            .values
                            .iter()
                            .map(|v| v.abs())
                            .filter(|v| v.is_finite() && *v > 0.0)
                            .collect();
                        teensy_median(sizes).is_some_and(|m| m > bound)
                    })
            };
            let passed = latitude.is_none_or(|c| past(c, std::f64::consts::FRAC_PI_2))
                && past(*longitude, std::f64::consts::PI);
            let columns = latitude.iter().copied().chain([*longitude]).collect();
            (columns, (None, passed))
        }
        Check::SpeedOverHeight { speed, height } => {
            let series = heights(log, &name(*height));
            let channel = log.channels.iter().find(|c| c.column == name(*speed));
            let ratio = climb(&series).zip(channel).and_then(|(c, speeds)| {
                let height_channel = log.channels.iter().find(|ch| ch.column == name(*height))?;
                let mut ratios = Vec::new();
                for k in 1..height_channel.values.len().min(speeds.values.len()) {
                    let t = height_channel.time_s[k];
                    if t <= c.liftoff_s || t > c.apogee_s {
                        continue;
                    }
                    let dt = t - height_channel.time_s[k - 1];
                    let mean = (speeds.values[k] + speeds.values[k - 1]) / 2.0;
                    let rate = (height_channel.values[k] - height_channel.values[k - 1]) / dt;
                    if dt > 0.0 && mean.abs() > SPEED_FLOOR_M_S && rate.is_finite() {
                        ratios.push(rate / mean);
                    }
                }
                teensy_median(ratios)
            });
            (vec![*speed, *height], ratio_in(ratio, &UNIT_BAND))
        }
    };
    Outcome {
        check: check.name().to_owned(),
        columns,
        measured,
        passed,
    }
}

/// Every held file read and checked. A file's checks that name another held file run after that
/// file is read, so each runs on records read once.
pub(super) fn checks(samples: &[Sample]) -> Checks {
    let Ok(stated) = stated() else {
        return Checks::default();
    };
    let held_samples: Vec<&Sample> = samples.iter().filter(|s| s.family == FAMILY).collect();
    let apogees = stated_apogees();
    let mut logs = BTreeMap::new();
    let mut read_files = Vec::new();
    for sample in &held_samples {
        let id = id_of(&sample.bytes);
        let Some(stated) = stated.iter().find(|s| s.id == id) else {
            continue;
        };
        match read(stated, &sample.bytes) {
            Ok((log, mapping)) => {
                logs.insert(id.clone(), log.clone());
                read_files.push((stated, Some((log, mapping)), None));
            }
            Err(error) => read_files.push((stated, None, Some(error))),
        }
    }
    let others = Others {
        held: &logs,
        samples,
    };
    let mut files = Vec::new();
    for (stated, read, refused) in read_files {
        let mut file = HeldFile {
            id: stated.id.clone(),
            what: stated.what.clone(),
            refused,
            ..HeldFile::default()
        };
        if let Some((log, mapping)) = read {
            file.rows = log.channels.first().map_or(0, |c| c.values.len());
            for (index, column) in mapping.columns.iter().enumerate() {
                if index == mapping.clock {
                    continue;
                }
                let channel = log.channels.iter().find(|c| c.column == column.name);
                match (
                    column.skip,
                    column.unit.is_some(),
                    channel.map(|c| c.quantity),
                ) {
                    (true, ..) => file.left_out += 1,
                    (false, true, Some(q)) if q != Quantity::Unrecognized => file.read += 1,
                    _ => file.kept += 1,
                }
            }
            file.events = log.events.len();
            file.outcomes = stated
                .checks
                .iter()
                .map(|check| {
                    run(
                        check,
                        &log,
                        &mapping,
                        &others,
                        apogees.get(&stated.id).copied(),
                    )
                })
                .collect();
            file.record = match serde_json::to_vec(&log) {
                Ok(json) => hex(&Sha256::digest(&json))[..16].to_owned(),
                Err(_) => "unwritable".to_owned(),
            };
        }
        files.push(file);
    }
    files.sort_by(|a, b| a.id.cmp(&b.id));
    Checks { files }
}

/// The survey's failures: the mapping file's own, a held file with no mapping, a refusal, a unit
/// stated but contradicted by the values' check, and a failed check. A mapping with no file can't
/// be: each mapped id is reclassified to [`FAMILY`], and a reclassified file the survey doesn't
/// meet stops it (a test holds the two lists equal).
pub(super) fn unread(checks: &Checks, samples: &[Sample]) -> Vec<String> {
    let mut out = Vec::new();
    let stated = match stated() {
        Ok(stated) => stated,
        Err(error) => return vec![format!("  {error}")],
    };
    let held: BTreeSet<String> = samples
        .iter()
        .filter(|s| s.family == FAMILY)
        .map(|s| id_of(&s.bytes))
        .collect();
    for id in &held {
        if !stated.iter().any(|s| &s.id == id) {
            out.push(format!(
                "  {id} ({FAMILY}): no mapping is stated for it in validation/log-held-mappings.json"
            ));
        }
    }
    for f in &checks.files {
        if let Some(refused) = &f.refused {
            out.push(format!("  {} ({FAMILY}): {refused}", f.id));
            continue;
        }
        let stated_units = stated.iter().find(|s| s.id == f.id).map_or(0, |s| {
            s.mapping
                .columns
                .iter()
                .enumerate()
                .filter(|(i, c)| *i != s.mapping.clock && !c.skip && c.unit.is_some())
                .count()
        });
        if f.read != stated_units {
            out.push(format!(
                "  {} ({FAMILY}): {} of the {stated_units} columns the mapping reads in a unit \
                 are read in it",
                f.id, f.read
            ));
        }
        for o in f.outcomes.iter().filter(|o| !o.passed) {
            out.push(format!(
                "  {} ({FAMILY}): its {} check of columns {:?} fails: {:?}",
                f.id, o.check, o.columns, o.measured
            ));
        }
    }
    out
}

/// The report's section for this increment.
pub(super) fn markdown(checks: &Checks) -> String {
    if checks.files.is_empty() {
        return String::new();
    }
    let mut rows = String::new();
    for f in &checks.files {
        let outcomes: Vec<String> = f
            .outcomes
            .iter()
            .map(|o| {
                let columns = if o.columns.is_empty() {
                    String::new()
                } else {
                    format!(
                        " ({})",
                        o.columns
                            .iter()
                            .map(usize::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                let measured = o.measured.map_or(String::new(), |m| format!(" {m:.4}"));
                format!(
                    "{}{columns}{measured} {}",
                    o.check,
                    if o.passed { "✓" } else { "✗" }
                )
            })
            .collect();
        rows += &format!(
            "| `{}` | {} | {} | {} | {} | {} | {} | {} |\n",
            f.id,
            f.what,
            f.refused
                .as_deref()
                .map_or(f.rows.to_string(), |_| "refused".to_owned()),
            f.read,
            f.kept,
            f.left_out,
            f.events,
            outcomes.join("; ")
        );
    }
    format!(
        "\nThe {} tables read by a column mapping held to physics (M7.1c22, ADR-274): tables \
         whose headers state too little or that have none, each read by the mapping stated for it \
         in `validation/log-held-mappings.json`, which names a private table's columns by its own \
         labels or by the header at each run, but for the last name of a header that runs onto \
         its first row. *Read* counts the columns read in the unit the mapping states, *Kept* those kept \
         as written, with no unit, and *Left out* those the mapping leaves out. Each check holds a \
         unit, its columns counted from 0: *apogee time*, the clock's, is the time from liftoff \
         to apogee of a height over √(2h/g), within {:.1}–{:.1}; *clock beside*, the clock's, \
         is the overlap of this file's span of times and another's, of the same logger, over the \
         longer of the two, at least {BESIDE_OVERLAP}; *clock rate*, \
         for a recording with no flight, is its median step over the same logger's in a file \
         held by its flight, within {:.1}–{:.1}; *rows of* is the share of its rows that are \
         rows of another file, value for value, which must be 1; *pad pressure*, a pressure's, \
         is its reading at the pad within {:.0}–{:.0} kPa, its value not shown, as it would give \
         the site's elevation; *standard atmosphere*, an altitude's, is its \
         rise over the standard-atmosphere height's rise of the file's pressure; *other logger* \
         its rise over another logger's in the same flight; *stated apogee* its rise over the \
         apogee the flyer states; *speed over height*, a vertical speed's, is an altitude's \
         change over it from liftoff to apogee: each within {:.1}–5/3, which no unit off by \
         3.28 or more reaches; *1 g at rest* is the accelerometer's median magnitude over the \
         recording, in g, within {:.2}–{:.2}; and *place*, a track's, is whether the median size of \
         each column's readings other than 0 lies past ±π/2 for a latitude and ±π for a \
         longitude, which no place has in radians, its values not shown. A column with no unit is kept as written: a temperature, as no check here tells \
         °C from °F at a launch's temperatures, and a rate of turn, which none holds.\n\n\
         | File | What | Rows | Read | Kept | Left out | Events | Checks |\n\
         |---|---|---:|---:|---:|---:|---:|---|\n{rows}",
        checks.files.len(),
        APOGEE_TIME_BAND.start(),
        APOGEE_TIME_BAND.end(),
        CLOCK_RATE_BAND.start(),
        CLOCK_RATE_BAND.end(),
        PAD_PRESSURE_PA.start() / 1_000.0,
        PAD_PRESSURE_PA.end() / 1_000.0,
        UNIT_BAND.start(),
        REST_BAND_G.start(),
        REST_BAND_G.end(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An invented flight at 2 readings a second: 20 s on a pad 200 m above sea level, a climb
    /// to 1,000 m in 15 s that slows to 0 at apogee, then a descent at 10 m/s. Each row the time,
    /// the pressure, the height above the pad, the vertical speed and an accelerometer's three
    /// axes at 1 g on the pad and under the parachute, 0 in the climb.
    fn flight() -> Vec<[f64; 7]> {
        let (pad_s, climb_s, top_m) = (20.0, 15.0, 1_000.0);
        (0..400)
            .map(|k| {
                let t = f64::from(k) * 0.5;
                let (h, v, a) = if t < pad_s {
                    (0.0, 0.0, G0)
                } else if t <= pad_s + climb_s {
                    let x = (t - pad_s) / climb_s;
                    (
                        top_m * (1.0 - (1.0 - x).powi(2)),
                        2.0 * top_m / climb_s * (1.0 - x),
                        0.0,
                    )
                } else {
                    let h = (top_m - 10.0 * (t - pad_s - climb_s)).max(0.0);
                    (h, if h > 0.0 { -10.0 } else { 0.0 }, G0)
                };
                [
                    t,
                    standard_pressure_pa(200.0 + h),
                    h,
                    v,
                    a * 0.6,
                    a * 0.8,
                    0.0,
                ]
            })
            .collect()
    }

    /// The flight read as a table, its columns in the given units: the clock, pressure, height,
    /// speed and three accelerometer axes, each written in the unit given, times `scale`.
    fn read_flight(units: [&str; 7], scales: [f64; 7]) -> (FlightLog, Mapping) {
        let quantities = [
            None,
            Some(Quantity::Pressure),
            Some(Quantity::AltitudeAboveGround),
            Some(Quantity::Speed),
            Some(Quantity::Acceleration),
            Some(Quantity::Acceleration),
            Some(Quantity::Acceleration),
        ];
        let names = ["t", "p", "h", "v", "x", "y", "z"];
        let columns = (0..7)
            .map(|k| ColumnMapping {
                name: names[k].to_owned(),
                quantity: quantities[k],
                unit: Some(units[k].to_owned()),
                skip: false,
            })
            .collect();
        let mapping = Mapping {
            header_line: None,
            separator: ',',
            decimal_comma: false,
            clock: 0,
            columns,
            clock_wrap: None,
            rows: RowForm::Cells,
        };
        let text: String = flight()
            .iter()
            .map(|row| {
                let cells: Vec<String> = row
                    .iter()
                    .zip(scales)
                    .map(|(v, s)| format!("{}", v * s))
                    .collect();
                cells.join(",") + "\n"
            })
            .collect();
        let log = hpr_flightdata::generic::read_with(&text, &mapping).expect("reads");
        (log, mapping)
    }

    const SI: [&str; 7] = ["s", "Pa", "m", "m/s", "m/s^2", "m/s^2", "m/s^2"];
    const ONE: [f64; 7] = [1.0; 7];

    fn outcome(
        check: &Check,
        log: &FlightLog,
        mapping: &Mapping,
        held: &[(&str, FlightLog)],
    ) -> Outcome {
        let held: BTreeMap<String, FlightLog> = held
            .iter()
            .map(|(id, log)| ((*id).to_owned(), log.clone()))
            .collect();
        let others = Others {
            held: &held,
            samples: &[],
        };
        run(check, log, mapping, &others, Some(1_000.0))
    }

    /// Each check passes the flight read in its units, and fails it read in a wrong one that the
    /// values' check lets through.
    #[test]
    fn each_check_passes_the_right_unit_and_fails_a_wrong_one() {
        let (right, mapping) = read_flight(SI, ONE);
        let with_all = |columns: &[usize], written: &str, scale: f64| {
            let mut units = SI;
            let mut scales = ONE;
            for unit in columns {
                units[*unit] = written;
                scales[*unit] = scale;
            }
            read_flight(units, scales)
        };
        let with = |unit: usize, written: &str, scale: f64| with_all(&[unit], written, scale);
        let cases: Vec<(Check, (FlightLog, Mapping))> = vec![
            // The clock in s written as ms: a thousand times too short.
            (Check::ApogeeTime { height: 1 }, with(0, "ms", 1.0)),
            (Check::ApogeeTime { height: 2 }, with(0, "ms", 1.0)),
            // Hectopascals stated as pascals.
            (Check::PadPressure { column: 1 }, with(1, "Pa", 0.01)),
            // Meters stated as feet.
            (
                Check::StandardAtmosphere {
                    altitude: 2,
                    pressure: 1,
                },
                with(2, "ft", 1.0),
            ),
            (Check::StatedApogee { altitude: 2 }, with(2, "ft", 1.0)),
            (
                Check::OtherLogger {
                    altitude: 2,
                    file: "other".to_owned(),
                },
                with(2, "ft", 1.0),
            ),
            // m/s² stated as g, and g stated as m/s².
            (
                Check::Rest {
                    columns: vec![4, 5, 6],
                },
                with_all(&[4, 5, 6], "g", 1.0),
            ),
            (
                Check::Rest {
                    columns: vec![4, 5, 6],
                },
                with_all(&[4, 5, 6], "m/s^2", 1.0 / G0),
            ),
            // m/s stated as ft/s.
            (
                Check::SpeedOverHeight {
                    speed: 3,
                    height: 2,
                },
                with(3, "ft/s", 1.0),
            ),
        ];
        for (check, (wrong, wrong_mapping)) in &cases {
            let held = [("other", right.clone())];
            let good = outcome(check, &right, &mapping, &held);
            assert!(good.passed, "{}: {good:?}", check.name());
            let bad = outcome(check, wrong, wrong_mapping, &held);
            assert!(!bad.passed, "{}: {bad:?}", check.name());
        }
        // The g a rest check reads and the ratios it gives, by hand.
        let rest = outcome(
            &Check::Rest {
                columns: vec![4, 5, 6],
            },
            &right,
            &mapping,
            &[],
        );
        assert_eq!(rest.measured, Some(1.0));
        let wrong = with(2, "ft", 1.0);
        let feet = outcome(
            &Check::StandardAtmosphere {
                altitude: 2,
                pressure: 1,
            },
            &wrong.0,
            &wrong.1,
            &[],
        );
        assert_eq!(feet.measured, Some(0.3048));
        let time = outcome(&Check::ApogeeTime { height: 2 }, &right, &mapping, &[]);
        // 15 s and a reading's step to liftoff, over √(2 × 1,000 / g) = 14.28 s.
        assert!(
            (1.0..1.15).contains(&time.measured.unwrap_or(0.0)),
            "{time:?}"
        );
    }

    #[test]
    fn the_clock_checks_hold_a_clock_against_another_files() {
        let (right, mapping) = read_flight(SI, ONE);
        let mut units = SI;
        units[0] = "ms";
        let (fast, fast_mapping) = read_flight(units, ONE);
        let held = [("other", right.clone())];
        for check in [
            Check::ClockBeside {
                file: "other".to_owned(),
            },
            Check::ClockRate {
                file: "other".to_owned(),
            },
            Check::RowsOf {
                file: "other".to_owned(),
            },
        ] {
            assert!(
                outcome(&check, &right, &mapping, &held).passed,
                "{}",
                check.name()
            );
            assert!(
                !outcome(&check, &fast, &fast_mapping, &held).passed,
                "{}",
                check.name()
            );
        }
        // A file that isn't there fails.
        let missing = Check::ClockBeside {
            file: "none".to_owned(),
        };
        assert!(!outcome(&missing, &right, &mapping, &held).passed);
    }

    #[test]
    fn a_place_in_degrees_lies_past_what_radians_can() {
        let mapping = |lat: &str| Mapping {
            header_line: None,
            separator: ',',
            decimal_comma: false,
            clock: 0,
            columns: ["t", "lat", "lon"]
                .iter()
                .zip([None, Some(Quantity::Latitude), Some(Quantity::Longitude)])
                .map(|(name, quantity)| ColumnMapping {
                    name: (*name).to_owned(),
                    quantity,
                    unit: Some(if quantity.is_some() { lat } else { "s" }.to_owned()),
                    skip: false,
                })
                .collect(),
            clock_wrap: None,
            rows: RowForm::Cells,
        };
        let check = Check::Place {
            latitude: Some(1),
            longitude: 2,
        };
        // Invented places. Each column is held on its own: one in radians fails the check, and so
        // does a column in radians with one glitch past the bound; zeros before a fix don't count.
        let degrees = "0,10.25,-20.5\n1,10.25,-20.5\n";
        let radians = "0,0.1789,-0.3578\n1,0.1789,-0.3578\n";
        let latitude_in_radians = "0,0.1789,-20.5\n1,0.1789,-20.5\n";
        let longitude_in_radians = "0,10.25,-0.3578\n1,10.25,-0.3578\n";
        let a_glitch = "0,10.25,-0.3578\n1,10.25,-99\n2,10.25,-0.3578\n";
        let before_a_fix = "0,0,0\n1,0,0\n2,0,0\n3,10.25,-20.5\n4,10.25,-20.5\n";
        for (text, passed) in [
            (degrees, true),
            (radians, false),
            (latitude_in_radians, false),
            (longitude_in_radians, false),
            (a_glitch, false),
            (before_a_fix, true),
        ] {
            let mapping = mapping("deg");
            let log = hpr_flightdata::generic::read_with(text, &mapping).expect("reads");
            assert_eq!(
                outcome(&check, &log, &mapping, &[]).passed,
                passed,
                "{text}"
            );
        }
    }

    /// The pad is the lowest height over the [`CLIMB_WINDOW_S`] before apogee: a barometer that
    /// drifts down 100 m over an hour on the ground doesn't move liftoff, and a record that starts
    /// at liftoff has a pad.
    #[test]
    fn the_pad_is_the_lowest_height_just_before_the_climb() {
        let mut series: Vec<(f64, f64)> = (0..3_600)
            .map(|k| (f64::from(k), 100.0 - f64::from(k) / 36.0))
            .collect();
        series.extend((0..20).map(|k| (3_600.0 + f64::from(k), 50.0 * f64::from(k))));
        let climb = climb(&series).expect("climbs");
        assert!(
            (climb.pad_m - (100.0 - 3_599.0 / 36.0)).abs() < 0.1,
            "{climb:?}"
        );
        assert!(
            (climb.apogee_s - climb.liftoff_s - 19.0).abs() <= 1.0,
            "{climb:?}"
        );
        let from_liftoff: Vec<(f64, f64)> = (0..20)
            .map(|k| (f64::from(k), 50.0 * f64::from(k)))
            .collect();
        let climb = super::climb(&from_liftoff).expect("climbs");
        assert_eq!(climb.pad_m, 0.0);
        // A barometer that read 300 m lower an hour before launch and drifted up since: the pad
        // is looked for only in the 300 s before apogee, so the hour before doesn't move it.
        let mut drifted: Vec<(f64, f64)> = (0..3_600)
            .map(|k| (f64::from(k), -200.0 + f64::from(k) / 12.0))
            .collect();
        drifted.extend((0..20).map(|k| (3_600.0 + f64::from(k), 100.0 + 50.0 * f64::from(k))));
        let climb = super::climb(&drifted).expect("climbs");
        assert!((70.0..=100.0).contains(&climb.pad_m), "{climb:?}");
    }

    /// Speeds under the floor aren't taken, as there a speed's noise outweighs the height's
    /// change: a climb at 100 m/s for 5 s, then 40 s rising at 0.1 m/s that the speed reads as
    /// 5 m/s. Taken, those 80 rows' ratios of 0.02 would be the median.
    #[test]
    fn slow_speeds_are_left_out_of_the_speed_check() {
        let height = |t: f64| {
            if t < 20.0 {
                0.0
            } else if t < 25.0 {
                100.0 * (t - 20.0)
            } else if t < 65.0 {
                500.0 + 0.1 * (t - 25.0)
            } else {
                (504.0 - 10.0 * (t - 65.0)).max(0.0)
            }
        };
        let speed = |t: f64| match t {
            t if t < 20.0 => 0.0,
            t if t < 25.0 => 100.0,
            t if t < 65.0 => 5.0,
            _ => -10.0,
        };
        let text: String = (0..240)
            .map(|k| {
                let t = f64::from(k) * 0.5;
                format!("{t},{},{}\n", height(t), speed(t))
            })
            .collect();
        let mapping = Mapping {
            header_line: None,
            separator: ',',
            decimal_comma: false,
            clock: 0,
            columns: vec![
                ColumnMapping {
                    name: "t".to_owned(),
                    quantity: None,
                    unit: Some("s".to_owned()),
                    skip: false,
                },
                ColumnMapping {
                    name: "h".to_owned(),
                    quantity: Some(Quantity::AltitudeAboveGround),
                    unit: Some("m".to_owned()),
                    skip: false,
                },
                ColumnMapping {
                    name: "v".to_owned(),
                    quantity: Some(Quantity::Speed),
                    unit: Some("m/s".to_owned()),
                    skip: false,
                },
            ],
            clock_wrap: None,
            rows: RowForm::Cells,
        };
        let log = hpr_flightdata::generic::read_with(&text, &mapping).expect("reads");
        let check = Check::SpeedOverHeight {
            speed: 2,
            height: 1,
        };
        let held = outcome(&check, &log, &mapping, &[]);
        assert!(held.passed, "{held:?}");
        assert_eq!(held.measured, Some(1.0));
    }

    /// The report prints the unit band's end as 5/3.
    #[test]
    fn the_unit_band_ends_at_five_thirds() {
        assert_eq!(*UNIT_BAND.end(), 5.0 / 3.0);
    }

    #[test]
    fn the_mappings_cover_the_held_tables() {
        let stated: BTreeSet<String> = stated()
            .expect("parses")
            .into_iter()
            .map(|s| s.id)
            .collect();
        let held: BTreeSet<String> = super::super::RECLASSIFIED
            .iter()
            .filter(|(_, _, to, _)| *to == FAMILY)
            .map(|(id, ..)| (*id).to_owned())
            .collect();
        assert_eq!(stated, held);
    }

    #[test]
    fn the_committed_mappings_hold_every_unit() {
        let stated = stated().expect("parses");
        assert_eq!(stated.len(), 20);
        let one = |checks: &str, columns: &str| {
            format!(
                r#"{{"about":"","mappings":[{{"id":"a","what":"","mapping":{{"header_line":null,"separator":",","decimal_comma":false,"clock":0,"columns":{columns}}},"checks":{checks}}}]}}"#
            )
        };
        let columns = r#"[{"name":"t","quantity":null,"unit":"s"},{"name":"p","quantity":"pressure","unit":"Pa"}]"#;
        let held = r#"[{"check":"apogee_time","height":1},{"check":"pad_pressure","column":1}]"#;
        assert!(parse(&one(held, columns)).is_ok());
        for (checks, error) in [
            (
                r#"[{"check":"pad_pressure","column":1}]"#,
                "no check holds the clock",
            ),
            (
                r#"[{"check":"apogee_time","height":1}]"#,
                "no check holds column 1's unit",
            ),
            (
                r#"[{"check":"apogee_time","height":1},{"check":"rest","columns":[1]}]"#,
                "the 1 g at rest check takes column 1",
            ),
            (
                r#"[{"check":"apogee_time","height":0}]"#,
                "the apogee time check takes column 0",
            ),
        ] {
            let message = parse(&one(checks, columns)).unwrap_err();
            assert!(message.contains(error), "{message}");
        }
        // Every column is held by the rows of a held file.
        assert!(parse(&one(r#"[{"check":"rows_of","file":"b"}]"#, columns)).is_ok());
        // A speed is held against a height, not a pressure.
        let with_speed = r#"[{"name":"t","quantity":null,"unit":"s"},{"name":"p","quantity":"pressure","unit":"Pa"},{"name":"v","quantity":"speed","unit":"m/s"}]"#;
        let message = parse(&one(
            r#"[{"check":"apogee_time","height":1},{"check":"pad_pressure","column":1},{"check":"speed_over_height","speed":2,"height":1}]"#,
            with_speed,
        ))
        .unwrap_err();
        assert!(
            message.contains("the speed over height check takes column 1"),
            "{message}"
        );
    }

    /// A refusal, a column the mapping reads in a unit but the reader doesn't, and a failed check
    /// each leave a file unread; a file with none of them is read.
    #[test]
    fn a_refusal_or_a_failed_check_leaves_a_file_unread() {
        let stated = stated().expect("the committed mappings parse");
        let first = &stated[0];
        let in_a_unit = first
            .mapping
            .columns
            .iter()
            .enumerate()
            .filter(|(i, c)| *i != first.mapping.clock && !c.skip && c.unit.is_some())
            .count();
        let read = HeldFile {
            id: first.id.clone(),
            read: in_a_unit,
            outcomes: vec![Outcome {
                check: "apogee time".to_owned(),
                columns: vec![1],
                measured: Some(1.0),
                passed: true,
            }],
            ..HeldFile::default()
        };
        let lines = |file: &HeldFile| {
            unread(
                &Checks {
                    files: vec![file.clone()],
                },
                &[],
            )
        };
        assert!(lines(&read).is_empty());
        let mut failed = read.clone();
        failed.outcomes[0].passed = false;
        assert!(lines(&failed)[0].contains("its apogee time check of columns [1] fails"));
        let mut short = read.clone();
        short.read -= 1;
        assert!(lines(&short)[0].contains("columns the mapping reads in a unit are read in it"));
        let mut refused = read;
        refused.refused = Some("a refusal".to_owned());
        assert!(lines(&refused)[0].ends_with(": a refusal"));
    }
}
