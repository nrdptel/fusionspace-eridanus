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
pub(super) const REST_BAND_G: std::ops::RangeInclusive<f64> = 0.85..=1.15;

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

/// The track speed check's name, as the report prints it.
const TRACK_SPEED: &str = "track speed";

/// Standard gravity, m/s².
const G0: f64 = 9.806_65;

/// How long before a track's highest fix its climb's deceleration is taken, s. Coasting, a
/// rocket's vertical deceleration is g plus drag's vertical share, ½ρC_DA/m · v · v_z, which
/// falls to 0 at apogee as v_z does. A 54 mm rocket of 1.5 kg with C_D 0.5 at ρ = 1.1 kg/m³
/// climbs at 50.8 m/s 5 s before apogee straight up, its drag adding 0.11 g there, and 0.33 g
/// carrying 100 m/s across at apogee; a parabola fitted to its fixes once a second reads 1.031 g
/// and 1.135 g (`a_coast_with_drag_reads_inside_the_band`), where leaving [`UNIT_BAND`] takes
/// 1.667 g. A longer window lets drag grow; a shorter one holds too few fixes of a receiver that
/// reports once a second to set the curvature ([`TRACK_C_VARIANCE`]).
const TRACK_WINDOW_S: f64 = 5.0;

/// The least fixes over [`TRACK_WINDOW_S`], the highest included, a parabola is fitted to: one
/// more than its three terms.
const TRACK_FIXES: usize = 4;

/// How loosely the fixes may set the parabola's curvature c: [(XᵀX)⁻¹]_cc, the variance of c per
/// square meter of independent noise on each fix, at most 1/12 s⁻⁴. That is a little looser than
/// 5 fixes 1 s apart, 1/14 s⁻⁴, so that 5 fixes of a receiver once a second, up to 3.8 % closer
/// together, take the check. The reading's standard error is then at most 2√(1/12)/g = 0.0589 g
/// per meter of noise: with 3 m, a right unit reading 1.03 g lies 2.4 standard errors above the
/// band's foot, and the same fixes in feet stated as meters 3.0 past its top. Fewer fixes or fixes
/// closer together, as a receiver that lost its fix on the way up leaves, set c more loosely: 4
/// fixes over 3 s give 1/4 s⁻⁴, where noise would decide.
const TRACK_C_VARIANCE: f64 = 1.0 / 12.0;

/// The Earth's mean radius, m: the IUGG's R₁ (Moritz, *Geodetic Reference System 1980*, 2000).
const EARTH_RADIUS_M: f64 = 6_371_008.8;

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
    /// A track's GPS altitude, by the track's own vertical speed: its deceleration over the
    /// [`TRACK_WINDOW_S`] before its highest fix ([`track_deceleration_g`]), in g, within
    /// [`UNIT_BAND`]. It doesn't hold the clock: read 10 times too short, the window would take
    /// 50 s of fixes, the pad and the boost among them, and the fit could land anywhere.
    TrackSpeed { altitude: usize },
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
            Self::TrackSpeed { .. } => TRACK_SPEED,
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
            | Self::StatedApogee { altitude }
            | Self::TrackSpeed { altitude } => vec![(*altitude, HEIGHTS)],
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
    /// A track's ground speed, measured: it holds no unit, so a track's own speed holds its
    /// altitude by its vertical speed, the track speed check (ADR-274's amendment for M7.1c22).
    #[serde(default)]
    track_speeds: TrackSpeeds,
}

/// A track's ground speed over the held tracks with a latitude and a longitude: its speed
/// between consecutive fixes, the great-circle distance over the track's own clock. Only
/// aggregates, as most of the tracks are private.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct TrackSpeeds {
    /// The files measured, each with a place check of a latitude and a longitude that passed.
    files: usize,
    /// The tracks among them, a file whose fixes are another's counted once.
    tracks: usize,
    /// The least and the greatest of the tracks' median ground speeds from fix to fix, m/s,
    /// rounded to 4 decimals.
    slowest_median_m_s: Option<f64>,
    fastest_median_m_s: Option<f64>,
    /// The greatest median over the least, from the values before rounding.
    spread: Option<f64>,
    /// The least and the greatest of the tracks' fastest single steps, m/s.
    least_fastest_step_m_s: Option<f64>,
    most_fastest_step_m_s: Option<f64>,
    /// Each GPS altitude of a track the place check holds that takes no track speed check
    /// ([`untaken_tracks`]): one that could take it fails the survey.
    #[serde(default)]
    untaken: Vec<UntakenTrack>,
}

/// A track's GPS altitude with no track speed check, and why.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct UntakenTrack {
    /// The file's id.
    file: String,
    /// The altitude's column, from 0.
    column: usize,
    /// Its fixes in the [`track_window`], the highest included.
    fixes: usize,
    /// Whether its window could take the check ([`track_deceleration_g`] gives a value).
    could_take: bool,
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
        Check::TrackSpeed { altitude } => {
            let ratio = track_deceleration_g(&heights(log, &name(*altitude)));
            (vec![*altitude], ratio_in(ratio, &UNIT_BAND))
        }
    };
    Outcome {
        check: check.name().to_owned(),
        columns,
        measured,
        passed,
    }
}

/// A track's fixes over the [`TRACK_WINDOW_S`] before its highest, the highest included, taken
/// as apogee. An altitude repeated over consecutive rows counts once, at its first row, as a
/// logger writes rows faster than its receiver updates; so do two fixes at one altitude, as a
/// receiver rounding to whole meters gives near apogee, which moves the top a step early. The
/// highest is the last of the greatest where several, not consecutive, tie.
fn track_window(series: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut fixes: Vec<(f64, f64)> = Vec::new();
    for (t, h) in series {
        if fixes.last().is_none_or(|(_, last)| last != h) {
            fixes.push((*t, *h));
        }
    }
    let Some((top_s, _)) = fixes.iter().max_by(|a, b| a.1.total_cmp(&b.1)).copied() else {
        return Vec::new();
    };
    fixes
        .into_iter()
        .filter(|(t, _)| (top_s - TRACK_WINDOW_S..=top_s).contains(t))
        .collect()
}

/// A track's vertical deceleration over its [`track_window`], in g: the least-squares parabola
/// h = a + bτ + cτ² through those fixes, τ the time from their mean, gives −2c, over [`G0`].
/// `None` where the window can't take the check: fewer than [`TRACK_FIXES`] fixes, or fixes that
/// set c more loosely than [`TRACK_C_VARIANCE`], as where a receiver lost its fix on the way up.
fn track_deceleration_g(series: &[(f64, f64)]) -> Option<f64> {
    let window = track_window(series);
    if window.len() < TRACK_FIXES {
        return None;
    }
    let mean_s = window.iter().map(|(t, _)| t).sum::<f64>() / window.len() as f64;
    // The normal equations' sums: Στᵏ for k = 0..4, and Στᵏh for k = 0..2.
    let mut s = [0.0; 5];
    let mut y = [0.0; 3];
    for (t, h) in &window {
        let tau = t - mean_s;
        for (k, sum) in s.iter_mut().enumerate() {
            *sum += tau.powi(k as i32);
        }
        for (k, sum) in y.iter_mut().enumerate() {
            *sum += h * tau.powi(k as i32);
        }
    }
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let normal = [[s[0], s[1], s[2]], [s[1], s[2], s[3]], [s[2], s[3], s[4]]];
    // [(XᵀX)⁻¹]_cc: the cofactor of the corner over the determinant; infinite or NaN where the
    // fixes can't set c at all.
    let variance = (s[0] * s[2] - s[1] * s[1]) / det(normal);
    if variance.is_nan() || variance <= 0.0 || variance > TRACK_C_VARIANCE {
        return None;
    }
    // Cramer's rule for c: the third column replaced by the right-hand side.
    let mut for_c = normal;
    for (row, value) in for_c.iter_mut().zip(y) {
        row[2] = value;
    }
    let c = det(for_c) / det(normal);
    let ratio = -2.0 * c / G0;
    ratio.is_finite().then_some(ratio)
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
    let mut tracks = Vec::new();
    let mut untaken = Vec::new();
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
            untaken.extend(untaken_tracks(stated, &file.outcomes, &log, &mapping));
            // A track's own speed, measured where its place check holds its degrees.
            tracks.extend(
                stated
                    .checks
                    .iter()
                    .zip(&file.outcomes)
                    .filter(|(_, outcome)| outcome.passed)
                    .filter_map(|(check, _)| track_speed(check, &log, &mapping)),
            );
            file.record = match serde_json::to_vec(&log) {
                Ok(json) => hex(&Sha256::digest(&json))[..16].to_owned(),
                Err(_) => "unwritable".to_owned(),
            };
        }
        files.push(file);
    }
    files.sort_by(|a, b| a.id.cmp(&b.id));
    untaken.sort_by(|a, b| (&a.file, a.column).cmp(&(&b.file, b.column)));
    Checks {
        files,
        track_speeds: TrackSpeeds {
            untaken,
            ..track_speeds(&tracks)
        },
    }
}

/// A file's GPS altitudes read in a unit, of a track its place check of a latitude and a
/// longitude holds, that take no track speed check; each with its fixes in the
/// [`track_window`], and whether they could take one.
fn untaken_tracks(
    stated: &Stated,
    outcomes: &[Outcome],
    log: &FlightLog,
    mapping: &Mapping,
) -> Vec<UntakenTrack> {
    let placed = stated.checks.iter().zip(outcomes).any(|(check, outcome)| {
        outcome.passed
            && matches!(
                check,
                Check::Place {
                    latitude: Some(_),
                    ..
                }
            )
    });
    if !placed {
        return Vec::new();
    }
    mapping
        .columns
        .iter()
        .enumerate()
        .filter(|(index, column)| {
            *index != mapping.clock
                && !column.skip
                && column.unit.is_some()
                && column.quantity == Some(Quantity::GnssAltitude)
                && !stated
                    .checks
                    .iter()
                    .any(|c| matches!(c, Check::TrackSpeed { altitude } if altitude == index))
        })
        .map(|(index, column)| {
            let series = heights(log, &column.name);
            UntakenTrack {
                file: stated.id.clone(),
                column: index,
                fixes: track_window(&series).len(),
                could_take: track_deceleration_g(&series).is_some(),
            }
        })
        .collect()
}

/// A track's ground speeds from each fix to the next the receiver reports, m/s: the
/// great-circle distance (haversine) over the clock's step from the row where a fix first
/// appears to the row where the next does. A fix repeated over several rows, as a logger writes
/// rows faster than its receiver updates, counts once, at its first row. A fix that isn't a
/// number or reads 0, 0, as a receiver writes before its first fix, is left out, and so is a step
/// that doesn't move the clock forward.
fn ground_speeds(log: &FlightLog, latitude: &str, longitude: &str) -> Vec<f64> {
    let channel = |name: &str| log.channels.iter().find(|c| c.column == name);
    let (Some(lat), Some(lon)) = (channel(latitude), channel(longitude)) else {
        return Vec::new();
    };
    let mut fixes: Vec<(f64, f64, f64)> = Vec::new();
    for ((t, a), b) in lat.time_s.iter().zip(&lat.values).zip(&lon.values) {
        let (t, a, b) = (*t, *a, *b);
        if !(t.is_finite() && a.is_finite() && b.is_finite()) || (a, b) == (0.0, 0.0) {
            continue;
        }
        if fixes.last().is_none_or(|(_, a0, b0)| (*a0, *b0) != (a, b)) {
            fixes.push((t, a, b));
        }
    }
    fixes
        .windows(2)
        .filter_map(|pair| {
            let ((t0, lat0, lon0), (t1, lat1, lon1)) = (pair[0], pair[1]);
            let dt = t1 - t0;
            if dt <= 0.0 {
                return None;
            }
            let (p0, p1) = (lat0.to_radians(), lat1.to_radians());
            let h = ((p1 - p0) / 2.0).sin().powi(2)
                + p0.cos() * p1.cos() * ((lon1 - lon0).to_radians() / 2.0).sin().powi(2);
            Some(2.0 * EARTH_RADIUS_M * h.sqrt().min(1.0).asin() / dt)
        })
        .collect()
}

/// A track's ground speeds, for a place check of a latitude and a longitude.
fn track_speed(check: &Check, log: &FlightLog, mapping: &Mapping) -> Option<Vec<f64>> {
    let Check::Place {
        latitude: Some(latitude),
        longitude,
    } = check
    else {
        return None;
    };
    let name = |index: usize| mapping.columns.get(index).map(|c| c.name.as_str());
    let speeds = ground_speeds(log, name(*latitude)?, name(*longitude)?);
    (!speeds.is_empty()).then_some(speeds)
}

/// The least or the greatest of some values.
fn extreme(values: impl Iterator<Item = f64>, pick: fn(f64, f64) -> f64) -> Option<f64> {
    values.reduce(pick)
}

/// The tracks' speeds, gathered: a track whose speeds are another's, the same fixes in a second
/// file, counted once; the least and the greatest of their medians and fastest steps.
fn track_speeds(tracks: &[Vec<f64>]) -> TrackSpeeds {
    let mut distinct: Vec<&Vec<f64>> = Vec::new();
    for track in tracks {
        if !distinct.contains(&track) {
            distinct.push(track);
        }
    }
    let medians: Vec<f64> = distinct
        .iter()
        .filter_map(|t| teensy_median((*t).clone()))
        .collect();
    let fastest: Vec<f64> = distinct
        .iter()
        .filter_map(|t| extreme(t.iter().copied(), f64::max))
        .collect();
    let slowest = extreme(medians.iter().copied(), f64::min);
    let quickest = extreme(medians.iter().copied(), f64::max);
    TrackSpeeds {
        files: tracks.len(),
        tracks: distinct.len(),
        slowest_median_m_s: slowest.map(rounded),
        fastest_median_m_s: quickest.map(rounded),
        spread: slowest
            .zip(quickest)
            .map(|(s, q)| q / s)
            .filter(|r| r.is_finite())
            .map(rounded),
        least_fastest_step_m_s: extreme(fastest.iter().copied(), f64::min).map(rounded),
        most_fastest_step_m_s: extreme(fastest.iter().copied(), f64::max).map(rounded),
        untaken: Vec::new(),
    }
}

/// The survey's failures: the mapping file's own, a held file with no mapping, a refusal, a unit
/// stated but contradicted by the values' check, a failed check, and a track's GPS altitude that
/// could take a track speed check and takes none ([`untaken_tracks`]). A mapping with no file can't
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
    for u in checks.track_speeds.untaken.iter().filter(|u| u.could_take) {
        out.push(format!(
            "  {} ({FAMILY}): its track's GPS altitude, column {}, has the fixes for a track speed \
             check in the {TRACK_WINDOW_S} s before its highest, {}, but takes none",
            u.file, u.column, u.fixes
        ));
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
         change over it from liftoff to apogee; *track speed*, a track's GPS altitude's, is how \
         fast the track's own vertical speed falls over the {TRACK_WINDOW_S} s before its \
         highest fix, in g, from a parabola fitted to those fixes, as a rocket coasting to \
         apogee slows at g and a little more from drag: each within {:.1}–5/3, which no unit off by 3.28 or more \
         reaches; *1 g at rest* is the accelerometer's median magnitude over the \
         recording, in g, within {:.2}–{:.2}; and *place*, a track's, is whether the median size of \
         each column's readings other than 0 lies past ±π/2 for a latitude and ±π for a \
         longitude, which no place has in radians, its values not shown. A column with no unit \
         is kept as written: a temperature, as no check here tells °C from °F at a launch's \
         temperatures, a rate of turn, which none holds, and every other column no check \
         holds.\n\n\
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
    ) + &track_paragraph(checks)
}

/// The report's paragraph on a track's own speed: the vertical speed's check, and the ground
/// speed measured, and why it holds no unit.
fn track_paragraph(checks: &Checks) -> String {
    let speeds = &checks.track_speeds;
    let passed = |o: &&Outcome| o.check == TRACK_SPEED && o.passed;
    let files = checks
        .files
        .iter()
        .filter(|f| f.outcomes.iter().any(|o| passed(&o)))
        .count();
    let readings: Vec<f64> = checks
        .files
        .iter()
        .flat_map(|f| &f.outcomes)
        .filter(passed)
        .filter_map(|o| o.measured)
        .collect();
    let in_files = if files == 1 {
        "1 file".to_owned()
    } else {
        format!("{files} files")
    };
    let runs = match (
        extreme(readings.iter().copied(), f64::min),
        extreme(readings.iter().copied(), f64::max),
    ) {
        (Some(least), Some(most)) if least == most => {
            format!(" It passes in {in_files}, slowing at {least:.4} g.")
        }
        (Some(least), Some(most)) => {
            format!(" It passes in {in_files}, slowing at {least:.4}–{most:.4} g.")
        }
        _ => " No file passes it.".to_owned(),
    };
    let short: Vec<String> = speeds
        .untaken
        .iter()
        .filter(|u| !u.could_take)
        .map(|u| u.fixes.to_string())
        .collect();
    let untaken = match short.as_slice() {
        [] => String::new(),
        [one] => format!(
            " The other track with a GPS altitude has {one} {} there, too few or too close \
             together to take it, and its altitude is held by its other checks.",
            if one == "1" { "fix" } else { "fixes" }
        ),
        _ => format!(
            " The {} other tracks with a GPS altitude have {} fixes there, too few or too close \
             together to take it, and their altitudes are held by their other checks.",
            short.len(),
            short.join(", ")
        ),
    };
    let vertical = format!(
        "\nA track's own speed, which ADR-274's plan named among the checks, holds the track's \
         GPS altitude by its vertical speed: the *track speed* check above. It takes the highest \
         fix as apogee and needs {TRACK_FIXES} fixes or more in the {TRACK_WINDOW_S} s before \
         it, the highest included, that set the parabola's curvature about as well as 5 fixes \
         1 s apart; it holds a unit within a wide band, and doesn't measure drag.{runs} The \
         survey fails a GPS altitude of a track a place check holds that could take the check \
         and takes none.{untaken} It doesn't hold the clock, which the clock checks hold, as \
         every held file's: read 10 times too short, the window would take 50 s of fixes.\n"
    );
    let (Some(slowest), Some(fastest), Some(spread), Some(least_step), Some(most_step)) = (
        speeds.slowest_median_m_s,
        speeds.fastest_median_m_s,
        speeds.spread,
        speeds.least_fastest_step_m_s,
        speeds.most_fastest_step_m_s,
    ) else {
        return vertical;
    };
    // Radians stated as degrees read every distance, so every speed, 180/π times too small.
    let radians = fastest * std::f64::consts::PI / 180.0 < slowest;
    let step_spread = most_step / least_step;
    vertical
        + &format!(
            "\nA track's ground speed holds no unit. Over the {} tracks held by a place check of \
             a latitude and a longitude ({} files, a file whose fixes are another's counted \
             once), the median ground speed from each fix to the next the receiver reports, the \
             great-circle distance over the step of the track's own clock, taken over the whole \
             recording, the pad's hours included, so that a median can be a receiver's jitter as \
             much as the wind's drift, lies {slowest:.4}–{fastest:.4} m/s, a spread of \
             {spread:.2} times, {} the 10 times a clock misread by a factor of 10 moves it, so \
             {}. Nor could a band hold a clock on every flight: a track drifts with the wind, and \
             a launch may go on a calm day, so its speed has no floor (NFPA 1127 limits only the \
             surface wind, to 20 mph, 8.9 m/s). Each track's fastest step lies \
             {least_step:.4}–{most_step:.4} m/s, a spread of {step_spread:.2} times, so {}. \
             Radians stated as degrees would read 57 times slower, {}; the place check holds a \
             track's degrees with no band.\n",
            speeds.tracks,
            speeds.files,
            if spread > 10.0 {
                "wider than"
            } else {
                "narrower than"
            },
            if spread > 10.0 {
                "no band on these tracks refuses that misread on every one of them"
            } else {
                "a band on these tracks alone could refuse that misread"
            },
            if step_spread > 10.0 {
                "a track's top speed holds no clock either"
            } else {
                "a band on these tracks alone could hold a clock by its top speed, but a band \
                 fitted to them is a sample, not a bound"
            },
            if radians {
                "which a band could part on these tracks alone, but a band fitted to them is a \
                 sample, not a bound"
            } else {
                "which no band parts on these tracks"
            },
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
            // Meters stated as feet, and feet stated as meters.
            (Check::TrackSpeed { altitude: 2 }, with(2, "ft", 1.0)),
            (
                Check::TrackSpeed { altitude: 2 },
                with(2, "m", 1.0 / 0.3048),
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
        // The climb slows at 2 × 1,000 m / (15 s)² = 8.889 m/s², 0.9064 g.
        let track = outcome(&Check::TrackSpeed { altitude: 2 }, &right, &mapping, &[]);
        assert_eq!(track.measured, Some(0.9064));
    }

    /// A track's deceleration before its highest fix: a fix written over several rows counts
    /// once, the highest is the last of several, and a track with too few fixes before it gives
    /// none.
    #[test]
    fn a_tracks_vertical_speed_slows_at_g_before_its_top() {
        // Fixes once a second, coasting to 1,000 m at 35 s at g, then down at 10 m/s.
        let fix = |t: f64| {
            if t <= 35.0 {
                1_000.0 - G0 / 2.0 * (t - 35.0).powi(2)
            } else {
                1_000.0 - 10.0 * (t - 35.0)
            }
        };
        let fixes: Vec<(f64, f64)> = (20..60)
            .map(|k| (f64::from(k), fix(f64::from(k))))
            .collect();
        let ratio = track_deceleration_g(&fixes).map(rounded);
        assert_eq!(ratio, Some(1.0));
        // Rows 5 a second, each fix written until the next: the same fixes, the same ratio.
        let rows: Vec<(f64, f64)> = (100..300)
            .map(|k| {
                let t = f64::from(k) * 0.2;
                (t, fix(t.floor()))
            })
            .collect();
        assert_eq!(track_deceleration_g(&rows).map(rounded), Some(1.0));
        // Fixes kept at the given times only.
        let only = |times: &[f64]| -> Vec<(f64, f64)> {
            fixes
                .iter()
                .copied()
                .filter(|(t, _)| *t < 26.0 || *t > 35.0 || times.contains(t))
                .collect()
        };
        // 3 fixes in the 5 s before the highest, spread enough to set the curvature: too few.
        assert_eq!(track_deceleration_g(&only(&[30.0, 32.0, 35.0])), None);
        // 4 fixes over the last 3 s, as a receiver that lost its fix on the way up leaves: they
        // set the curvature 3 times as loosely as the bound allows.
        assert_eq!(track_deceleration_g(&only(&[32.0, 33.0, 34.0, 35.0])), None);
        // 4 fixes 0.1 s apart at the top, as a receiver at 10 Hz that just regained its fix.
        let fast: Vec<(f64, f64)> = [34.7, 34.8, 34.9, 35.0]
            .iter()
            .map(|t| (*t, fix(*t)))
            .collect();
        assert_eq!(track_deceleration_g(&fast), None);
        // 5 fixes 0.97 s apart take the check, and 0.95 s apart don't: the bound is 5 fixes
        // 0.962 s apart.
        let spaced = |step: f64| -> Vec<(f64, f64)> {
            (0..5)
                .map(|k| {
                    let t = 35.0 - step * f64::from(k);
                    (t, fix(t))
                })
                .rev()
                .collect()
        };
        assert_eq!(track_deceleration_g(&spaced(0.97)).map(rounded), Some(1.0));
        assert_eq!(track_deceleration_g(&spaced(0.95)), None);
        // A lone fix at the same height later: the last of the highest, with no climb before it.
        let mut twice = fixes.clone();
        twice.extend([(70.0, 1_000.0), (71.0, 500.0)]);
        assert_eq!(track_deceleration_g(&twice), None);
    }

    /// A coast with drag, the window's own example: a 54 mm rocket of 1.5 kg, C_D 0.5, in air of
    /// 1.1 kg/m³, its fixes once a second up to apogee, straight up and carrying 100 m/s across.
    /// Drag slows the climb a little more than g, well inside the band.
    #[test]
    fn a_coast_with_drag_reads_inside_the_band() {
        let k = 0.5 * 1.1 * 0.5 * std::f64::consts::PI * 0.027_f64.powi(2) / 1.5;
        // The state (height, across, up), integrated back from apogee by RK4 in 1 ms steps.
        let rate = |[_, vx, vz]: [f64; 3]| {
            let v = vx.hypot(vz);
            [vz, -k * v * vx, -G0 - k * v * vz]
        };
        let coast = |across: f64| {
            let mut state = [1_000.0, across, 0.0];
            let mut fixes = vec![(0.0, 1_000.0), (1.0, 995.0)];
            let dt = -0.001;
            for second in 1..=6 {
                for _ in 0..1_000 {
                    let step = |s: [f64; 3], r: [f64; 3], h: f64| {
                        [s[0] + h * r[0], s[1] + h * r[1], s[2] + h * r[2]]
                    };
                    let a = rate(state);
                    let b = rate(step(state, a, dt / 2.0));
                    let c = rate(step(state, b, dt / 2.0));
                    let d = rate(step(state, c, dt));
                    for i in 0..3 {
                        state[i] += dt / 6.0 * (a[i] + 2.0 * b[i] + 2.0 * c[i] + d[i]);
                    }
                }
                fixes.push((-f64::from(second), state[0]));
            }
            fixes.sort_by(|a, b| a.0.total_cmp(&b.0));
            track_deceleration_g(&fixes)
        };
        let up = coast(0.0).expect("6 fixes");
        let across = coast(100.0).expect("6 fixes");
        assert_eq!((up * 1e3).round() / 1e3, 1.031, "{up}");
        assert_eq!((across * 1e3).round() / 1e3, 1.135, "{across}");
        assert!(UNIT_BAND.contains(&up) && UNIT_BAND.contains(&across));
    }

    /// A track's GPS altitude that could take a track speed check and takes none fails the
    /// survey; one that takes it, a track no place check holds, and an altitude left out are
    /// passed over.
    #[test]
    fn a_track_with_the_fixes_for_its_speed_must_take_the_check() {
        // An invented track once a second: a coast to 1,000 m at 35 s at g, then down at
        // 10 m/s; a GPS altitude, and a second left out. (The reader refuses an altitude with
        // no unit.)
        let text: String = (0..60)
            .map(|k| {
                let t = f64::from(k);
                let h = if t <= 35.0 {
                    (1_000.0 - G0 / 2.0 * (t - 35.0).powi(2)).max(0.0)
                } else {
                    1_000.0 - 10.0 * (t - 35.0)
                };
                format!("{t},40.1,-100.2,{h},{h}\n")
            })
            .collect();
        let stated = |checks: &str, latitude: &str| {
            let json = format!(
                r#"{{"about":"","mappings":[{{"id":"a","what":"","mapping":{{"header_line":null,"separator":",","decimal_comma":false,"clock":0,"columns":[{{"name":"t","quantity":null,"unit":"s"}},{{"name":"lat","quantity":"latitude","unit":"deg"}},{{"name":"lon","quantity":"longitude","unit":"deg"}},{{"name":"gps","quantity":"gnss_altitude","unit":"m"}},{{"name":"gps 2","quantity":"gnss_altitude","unit":"m","skip":true}}]}},"checks":{checks}}}]}}"#
            );
            let stated = parse(&json).expect("parses").remove(0);
            let text = text.replace("40.1", latitude);
            let mapping = resolve(&stated, &text).expect("resolves");
            let log = hpr_flightdata::generic::read_with(&text, &mapping).expect("reads");
            let outcomes: Vec<Outcome> = stated
                .checks
                .iter()
                .map(|c| outcome(c, &log, &mapping, &[]))
                .collect();
            untaken_tracks(&stated, &outcomes, &log, &mapping)
        };
        let placed = r#"[{"check":"apogee_time","height":3},{"check":"place","latitude":1,"longitude":2},{"check":"stated_apogee","altitude":3}]"#;
        let untaken = stated(placed, "40.1");
        assert_eq!(
            untaken,
            vec![UntakenTrack {
                file: "a".to_owned(),
                column: 3,
                fixes: 6,
                could_take: true,
            }]
        );
        let checks = Checks {
            track_speeds: TrackSpeeds {
                untaken,
                ..TrackSpeeds::default()
            },
            ..Checks::default()
        };
        let failures = unread(&checks, &[]);
        assert_eq!(failures.len(), 1, "{failures:?}");
        assert!(failures[0].contains("but takes none"), "{failures:?}");
        // Taken, or a track whose latitude radians could reach, which no place check holds.
        let taken = r#"[{"check":"apogee_time","height":3},{"check":"place","latitude":1,"longitude":2},{"check":"stated_apogee","altitude":3},{"check":"track_speed","altitude":3}]"#;
        assert!(stated(taken, "40.1").is_empty());
        assert!(stated(placed, "0.7").is_empty());
        // Too few fixes: listed, but no failure.
        let short = Checks {
            track_speeds: TrackSpeeds {
                untaken: vec![UntakenTrack {
                    file: "a".to_owned(),
                    column: 3,
                    fixes: 1,
                    could_take: false,
                }],
                ..TrackSpeeds::default()
            },
            ..Checks::default()
        };
        assert!(unread(&short, &[]).is_empty());
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

    /// A track's ground speed: 0.001° of latitude is R·π/180·0.001 = 111.1951 m on the mean
    /// Earth; a fix of 0, 0 or not a number is left out, and a repeated fix moves 0 m/s.
    #[test]
    fn a_tracks_ground_speed_is_its_great_circle_steps_over_its_clock() {
        let mapping = Mapping {
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
                    unit: Some(if quantity.is_some() { "deg" } else { "s" }.to_owned()),
                    skip: false,
                })
                .collect(),
            clock_wrap: None,
            rows: RowForm::Cells,
        };
        let step = EARTH_RADIUS_M * std::f64::consts::PI / 180.0 * 0.001;
        assert!((step - 111.1951).abs() < 1e-4);
        let read = |text: &str| hpr_flightdata::generic::read_with(text, &mapping).expect("reads");
        // Rows 10 a second, the fix new each second: a fix repeated over rows counts once, at
        // its first row, so each step is 0.001° over 1 s, not over the 0.1 s of the last row.
        let text: String = (0..30)
            .map(|k| {
                format!(
                    "{}.{},{},-20.5\n",
                    k / 10,
                    k % 10,
                    10.25 + 0.001 * f64::from(k / 10)
                )
            })
            .collect();
        let speeds = ground_speeds(&read(&text), "lat", "lon");
        assert_eq!(speeds.len(), 2, "{speeds:?}");
        assert!(speeds.iter().all(|v| (v - step).abs() < 1e-6), "{speeds:?}");
        // A fix of 0, 0 and an empty row are left out, and so is a new fix on the clock's step
        // before it, which doesn't move the clock forward: 0.001° over 2 s, twice.
        let text = "0,0,0\n1,10.25,-20.5\n2,,\n3,10.251,-20.5\n3,10.252,-20.5\n5,10.253,-20.5\n";
        let log = read(text);
        let speeds = ground_speeds(&log, "lat", "lon");
        assert_eq!(speeds.len(), 2, "{speeds:?}");
        assert!(
            speeds.iter().all(|v| (v - step / 2.0).abs() < 1e-6),
            "{speeds:?}"
        );
        let check = Check::Place {
            latitude: Some(1),
            longitude: 2,
        };
        assert_eq!(track_speed(&check, &log, &mapping), Some(speeds));
        // The same fixes in two files count as one track.
        let gathered = track_speeds(&[vec![1.0, 3.0, 2.0], vec![1.0, 3.0, 2.0], vec![20.0]]);
        assert_eq!((gathered.files, gathered.tracks), (3, 2));
        assert_eq!(gathered.slowest_median_m_s, Some(2.0));
        assert_eq!(gathered.fastest_median_m_s, Some(20.0));
        assert_eq!(gathered.spread, Some(10.0));
        assert_eq!(gathered.least_fastest_step_m_s, Some(3.0));
        // A longitude alone makes no ground speed.
        let lone = Check::Place {
            latitude: None,
            longitude: 2,
        };
        assert_eq!(track_speed(&lone, &log, &mapping), None);
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
        // A track's own speed holds its GPS altitude, but neither the clock nor a pressure.
        let track = r#"[{"name":"t","quantity":null,"unit":"s"},{"name":"h","quantity":"gnss_altitude","unit":"m"}]"#;
        let message = parse(&one(r#"[{"check":"track_speed","altitude":1}]"#, track)).unwrap_err();
        assert!(message.contains("no check holds the clock"), "{message}");
        assert!(
            parse(&one(
                r#"[{"check":"apogee_time","height":1},{"check":"track_speed","altitude":1}]"#,
                track
            ))
            .is_ok()
        );
        let message = parse(&one(
            r#"[{"check":"apogee_time","height":1},{"check":"track_speed","altitude":1}]"#,
            columns,
        ))
        .unwrap_err();
        assert!(
            message.contains("the track speed check takes column 1"),
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
                    ..Checks::default()
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
