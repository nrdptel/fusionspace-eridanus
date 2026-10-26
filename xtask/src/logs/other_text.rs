//! The altimeters' other text (M7.1c21): MARSA's downloads, AltOS's telemetry decoded to a table,
//! a table cut from an AltOS export and an AltOS export a spreadsheet re-saved with formula cells,
//! each decoded here apart from the reader and held to what its units must keep
//! ([ADR-274][adr-274]).
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use std::collections::BTreeMap;

use hpr_flightdata::log::{FlightLog, Quantity};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{Sample, hex, standard_height_m, teensy_median};

/// The family of MARSA downloads, filed by the catalog as `xml` for their `<…>` lines.
pub(super) const MARSA_FAMILY: &str = "marsa-dat";
/// The family of AltOS's telemetry decoded to a table, filed as generic CSV.
pub(super) const TELEMETRY_CSV_FAMILY: &str = "altos-telem-csv";
/// The family of tables cut from an AltOS export, AltOS's column names with no `version` header,
/// filed as generic CSV.
pub(super) const ALTOS_TABLE_FAMILY: &str = "altos-table-csv";
/// The family of AltOS exports a spreadsheet re-saved, its header edited and formulas in its
/// cells, filed by the catalog as `altos-csv`.
pub(super) const RESAVED_FAMILY: &str = "altos-resaved-csv";
/// The increment that reads them.
pub(super) const INCREMENT: &str = "M7.1c21";

/// Why a file is a MARSA download.
pub(super) const MARSA: &str = "a MARSA altimeter's flight download: its settings as `<NAME> \
    value` lines, such as `<BARO_ZERO 0> 790`, then a row a sample of the time, the barometric \
    altitude in feet and the acceleration in g, as its software saves it to a `.dat` file or a \
    spreadsheet re-saves it; filed as XML for its `<…>` lines";
/// Why a file is AltOS's telemetry decoded to a table.
pub(super) const TELEMETRY_CSV: &str = "AltOS's radio telemetry decoded to a table, a TeleMega's \
    `0x09` packets one a row under a header of the packet's fields, \
    `serial,tick,ptype,state,…,height,crc`; filed as generic CSV by its extension";
/// Why a file is a table cut from an AltOS export.
pub(super) const ALTOS_TABLE: &str = "a table cut from an AltOS export, \
    `time,state,state_name,acceleration,…`: AltOS's own column names with no \
    `version,serial,flight` before them, read by its states; filed as generic CSV by its \
    extension";

/// Each MARSA download's other logger in the same flight, by id: the AltOS board in the flight's
/// folder of the collection, whose other log is the other stage's.
const MARSA_BESIDE: [(&str, &str); 2] = [
    ("4be82002a95c68db", "2a665d94d9ad35d1"),
    ("a80063e417b87028", "2f3fbd62dbc4f064"),
];

/// The apogee over the other logger's that the plan allows (ADR-274's 5 %).
const MARSA_APOGEE_BAND: std::ops::RangeInclusive<f64> = 0.95..=1.05;

/// The band a ratio that checks a unit must lie in: the `.eeprom` reader's climb band, 0.6 to
/// 5/3, symmetric about 1 in log terms ([`hpr_flightdata::altos_eeprom::CLIMB_RATIO`]), which no
/// unit off by a factor of 3.28 (feet for meters) or more can reach. Each check below is the ratio
/// of two readings that are equal in the right units: a barometric climb and an integrated
/// acceleration; a height's change over time and a speed; a speed's change over time and an
/// acceleration; an altitude's rise and its pressure's.
const UNIT_BAND: std::ops::RangeInclusive<f64> = hpr_flightdata::altos_eeprom::CLIMB_RATIO;

/// How far from 0 the acceleration's median before liftoff may be, g: nearer 0 than 1 g, as an
/// acceleration with gravity taken out reads at rest, where one with it in reads 1 g.
const MARSA_PAD_G_BOUND: f64 = 0.5;

/// The time after liftoff the climb is taken over, s.
const MARSA_CLIMB_S: f64 = 2.0;

/// Liftoff: the first acceleration above this, g.
const LIFTOFF_G: f64 = 3.0;

/// The height above the pad's reading over which a MARSA's barometer's one-count steps are
/// measured, ft.
const NEAR_PAD_FT: f64 = 300.0;

/// A foot, m; standard gravity, m/s².
const FOOT_M: f64 = 0.3048;
const G0: f64 = 9.806_65;

/// AltOS's flight states by number, as AltosUI's exports name them: the survey's own copy.
const ALTOS_STATES: [(u8, &str); 7] = [
    (2, "pad"),
    (3, "boost"),
    (4, "fast"),
    (5, "coast"),
    (6, "drogue"),
    (7, "main"),
    (8, "landed"),
];

/// The battery a TeleMega's telemetry reads, V, as a single LiPo cell's, 3.7 V nominal (*The
/// Altus Metrum System*, §3.2): read as the document's 12-bit counts behind its 100k/27k divider,
/// 4 V would be about 1,055.
const BATTERY_BAND_V: std::ops::RangeInclusive<f64> = 2.5..=5.0;

/// The pressures a launch site's pad can have, Pa.
const PAD_PRESSURE_PA: std::ops::RangeInclusive<f64> = 50_000.0..=110_000.0;

/// How far `altitude` less `height` may spread in a table cut from an AltOS export, half its
/// greatest less its least, m: each written to the centimeter, so their difference is within a
/// centimeter of one offset.
const HEIGHT_OFFSET_BOUND_M: f64 = 0.01 + 1e-9;

/// This increment's checks, one list a layout.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct Checks {
    marsa: Vec<MarsaFile>,
    telemetry_csv: Vec<TelemetryCsvFile>,
    altos_table: Vec<AltosTableFile>,
    resaved: Vec<ResavedFile>,
}

/// A MARSA download against its own lines, decoded here apart from the reader, and against the
/// other logger in the same rocket.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct MarsaFile {
    id: String,
    /// The settings, the headings and the samples decoded here.
    settings: usize,
    headings: usize,
    samples: usize,
    /// The reader's values against the samples decoded here, read in s, ft and g, each sample's
    /// time and altitude on the primary track and the times and values of its two channels, six
    /// comparisons a sample; those that differ.
    values_compared: usize,
    values_differing: usize,
    /// Whether the reader's figures are the settings decoded here, by name and value, and it
    /// read the altitude as a height above its zero and the acceleration as an acceleration.
    figures_agree: bool,
    quantities_read: bool,
    /// The other logger ([`MARSA_BESIDE`]).
    beside: String,
    /// The highest altitude read in feet over the other logger's highest height above its pad,
    /// both in m: within [`MARSA_APOGEE_BAND`]; and the same were the altitude read in meters,
    /// a measure of the unit.
    apogee_ratio: Option<f64>,
    apogee_ratio_in_meters: Option<f64>,
    /// The altitude on the pad, the median before liftoff, ft: the logger's zero isn't the
    /// pad's, a measure; and the acceleration's median there, g, which must be under
    /// [`MARSA_PAD_G_BOUND`] from 0: gravity taken out.
    pad_ft: Option<f64>,
    pad_g: Option<f64>,
    /// The time from liftoff to the highest altitude, less the other logger's from its zero, its
    /// boost, to its highest height, s: a measure of the clock.
    apogee_time_off_s: Option<f64>,
    /// The barometric climb over the first [`MARSA_CLIMB_S`] after liftoff, the first
    /// acceleration above [`LIFTOFF_G`], over the acceleration read in g integrated twice from
    /// rest over them: within [`UNIT_BAND`]; and the same read in m/s², a measure.
    climb_ratio: Option<f64>,
    climb_ratio_in_m_s2: Option<f64>,
    /// The least and greatest step between the altitudes written within [`NEAR_PAD_FT`] above
    /// the pad's reading that are one count, under 1.5 times the least, ft: a measure, from
    /// which [`hpr_flightdata::marsa::ALTITUDE_STEP_M`] is taken, so not a check of it.
    count_step_ft: Option<[f64; 2]>,
}

/// AltOS's telemetry decoded to a table, against its own rows decoded here apart from the
/// reader, and the packet's scales.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct TelemetryCsvFile {
    id: String,
    /// The rows, and those whose `crc` is `FALSE`.
    rows: usize,
    crc_false: usize,
    /// The packet types the rows name.
    ptypes: BTreeMap<String, usize>,
    /// The columns the reader kept as unknown names.
    unknown_columns: Vec<String>,
    /// The reader's values against the rows decoded here; those that differ.
    values_compared: usize,
    values_differing: usize,
    /// Whether every acceleration and speed is a whole number of sixteenths, every height a
    /// whole meter and every tick a whole hundredth of a second, as the packet carries them.
    sixteenths: bool,
    whole_meters: bool,
    hundredths: bool,
    /// The battery's least and greatest, V.
    battery_v: Option<[f64; 2]>,
    /// The events the reader states, in order.
    events: Vec<String>,
    /// Whether the first ground pressure is in [`PAD_PRESSURE_PA`]: it isn't written, as it
    /// gives the site's elevation.
    ground_pressure_in_band: Option<bool>,
    /// The acceleration in the pad state, the median, m/s²: about 0, as gravity is taken out.
    pad_acceleration_m_s2: Option<f64>,
    /// The median, from boost to coast, of the height's change between two rows over their
    /// time, over the mean of their speeds, where it is over 10 m/s; and of the speed's change
    /// over their time, over the mean of their accelerations, where it is over 5 m/s²: each
    /// within [`UNIT_BAND`], which holds the height's meters, the speed's and acceleration's
    /// sixteenths divided out, and the tick's seconds.
    climb_over_speed: Option<f64>,
    speed_change_over_acceleration: Option<f64>,
}

/// A table cut from an AltOS export, against its rows decoded here apart from the reader and
/// what AltOS's exports keep.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct AltosTableFile {
    id: String,
    rows: usize,
    /// The rows' states, `number name`, counted; and whether every one is AltOS's pair.
    states: BTreeMap<String, usize>,
    states_altos: bool,
    /// The reader's values against the rows decoded here; those that differ; whether the
    /// channels are the columns, in order.
    values_compared: usize,
    values_differing: usize,
    same_columns: bool,
    /// Half the spread of `altitude` less `height`, m: AltOS's height is its barometric
    /// altitude less the pad's, one offset, which isn't written as it gives the site's
    /// elevation; within [`HEIGHT_OFFSET_BOUND_M`].
    height_offset_spread_m: Option<f64>,
    /// The most `altitude` differs from the standard atmosphere's height of `pressure`, m: a
    /// measure of AltOS's own conversion.
    altitude_from_pressure_worst_m: Option<f64>,
    /// `altitude`'s rise from the first row to its highest over the standard atmosphere's
    /// height's rise of `pressure` over the same rows: within [`UNIT_BAND`], which holds both
    /// columns' units.
    altitude_rise_ratio: Option<f64>,
    /// The events the reader states, in order: each state named as AltOS's exports name it.
    events: Vec<String>,
}

/// An AltOS export a spreadsheet re-saved, against its cells decoded here apart from the reader.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct ResavedFile {
    id: String,
    rows: usize,
    /// The formula cells counted here, by column.
    formulas: BTreeMap<String, usize>,
    /// Each formula cell rebuilt from the reader's notes, its runs' formulas with `{row}` the
    /// cell's line, against the file's cell; those that differ; and whether every column of
    /// formulas alone is no channel.
    formulas_compared: usize,
    formulas_differing: usize,
    formula_columns_not_channels: bool,
    /// The columns whose name carries a unit after it, and the quantity the reader read each as.
    units: Vec<(String, String)>,
    /// The reader's values against the cells decoded here; those that differ.
    values_compared: usize,
    values_differing: usize,
    /// Cells of the channels that aren't numbers, formulas apart: gaps.
    not_numbers: usize,
    /// Whether the record has no primary track, the header naming no `height`.
    no_primary_track: bool,
}

/// A file's id, the first 16 hex digits of its SHA-256.
fn id_of(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))[..16].to_owned()
}

/// A number written in a cell, or `None`.
fn number(cell: &str) -> Option<f64> {
    cell.trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

/// Compares two lists value by value into a count of comparisons and one of those that differ
/// ([`differing`]), a value one list lacks counted in both.
fn compare(counts: &mut [usize; 2], ours: &[f64], theirs: &[f64]) {
    counts[0] += ours.len().max(theirs.len());
    counts[1] += differing(ours, theirs);
}

/// Two lists equal value by value, `NaN` equal to `NaN`: the count that differ, the lengths'
/// difference included.
fn differing(ours: &[f64], theirs: &[f64]) -> usize {
    ours.iter()
        .zip(theirs)
        .filter(|(a, b)| !(a == b || (a.is_nan() && b.is_nan())))
        .count()
        + ours.len().abs_diff(theirs.len())
}

/// The highest value of a track and its time.
fn peak(time_s: &[f64], values: &[f64]) -> Option<(f64, f64)> {
    let mut best: Option<(f64, f64)> = None;
    for (&t, &v) in time_s.iter().zip(values) {
        if v.is_finite() && best.is_none_or(|(_, b)| v > b) {
            best = Some((t, v));
        }
    }
    best
}

/// Every check of this increment.
pub(super) fn checks(samples: &[Sample]) -> Checks {
    Checks {
        marsa: marsa_checks(samples),
        telemetry_csv: telemetry_csv_checks(samples),
        altos_table: altos_table_checks(samples),
        resaved: resaved_checks(samples),
    }
}

/// A MARSA download decoded here: its settings by name, after their section's heading, with
/// their values; the headings counted; and its samples, as written.
type MarsaDecoded = (Vec<(String, String)>, usize, Vec<[f64; 3]>);

fn marsa_decode(text: &str) -> MarsaDecoded {
    let mut settings = Vec::new();
    let mut headings = 0;
    let mut samples = Vec::new();
    let mut section: Option<String> = None;
    for line in text.lines() {
        let line = line.trim().trim_end_matches(',').trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('<')
            && let Some((name, value)) = rest.split_once('>')
        {
            let (name, value) = (name.trim(), value.trim());
            if value.is_empty() {
                headings += 1;
                section = Some(name.to_owned());
            } else {
                let name = section
                    .as_ref()
                    .map_or_else(|| name.to_owned(), |s| format!("{s}, {name}"));
                settings.push((name, value.to_owned()));
            }
            continue;
        }
        let cells: Vec<Option<f64>> = line.split(',').map(number).collect();
        if let [Some(t), Some(h), Some(a)] = cells.as_slice() {
            samples.push([*t, *h, *a]);
        }
    }
    (settings, headings, samples)
}

fn marsa_checks(samples: &[Sample]) -> Vec<MarsaFile> {
    let mut files = Vec::new();
    for sample in samples.iter().filter(|s| s.family == MARSA_FAMILY) {
        let text = hpr_flightdata::import::decode(&sample.bytes);
        let mut file = MarsaFile {
            id: id_of(&sample.bytes),
            ..MarsaFile::default()
        };
        let (settings, headings, rows) = marsa_decode(&text);
        file.settings = settings.len();
        file.headings = headings;
        file.samples = rows.len();
        let t: Vec<f64> = rows.iter().map(|r| r[0]).collect();
        let feet: Vec<f64> = rows.iter().map(|r| r[1]).collect();
        let g: Vec<f64> = rows.iter().map(|r| r[2]).collect();
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            files.push(file);
            continue;
        };
        let altitude: Vec<f64> = feet.iter().map(|f| f * FOOT_M).collect();
        let acceleration: Vec<f64> = g.iter().map(|a| a * G0).collect();
        let channel = |column: &str, quantity: Quantity| {
            log.channels
                .iter()
                .find(|c| c.column == column && c.quantity == quantity)
        };
        let read_altitude = channel("altitude", Quantity::AltitudeAboveGround);
        let read_acceleration = channel("acceleration", Quantity::Acceleration);
        file.quantities_read = read_altitude.is_some() && read_acceleration.is_some();
        let mut counts = [0; 2];
        compare(&mut counts, &t, &log.time_s);
        compare(&mut counts, &altitude, &log.altitude_m);
        for (ours, read) in [
            (&altitude, read_altitude),
            (&acceleration, read_acceleration),
        ] {
            compare(&mut counts, ours, read.map_or(&[], |c| &c.values));
            compare(&mut counts, &t, read.map_or(&[], |c| &c.time_s));
        }
        [file.values_compared, file.values_differing] = counts;
        let figures: Vec<(String, String)> = log
            .stated
            .figures
            .iter()
            .map(|f| (f.name.clone(), f.written.clone()))
            .collect();
        file.figures_agree = figures == settings;

        // Liftoff, the pad, the climb and the apogee.
        if let Some(liftoff) = g.iter().position(|&a| a > LIFTOFF_G) {
            file.pad_ft = teensy_median(feet[..liftoff].to_vec());
            file.pad_g = teensy_median(g[..liftoff].to_vec());
            let pad_m = file.pad_ft.map(|p| p * FOOT_M);
            let start = liftoff.saturating_sub(1);
            let (mut v, mut z, mut v_si, mut z_si) = (0.0, 0.0, 0.0, 0.0);
            let mut end = start;
            while end + 1 < rows.len() && t[end + 1] - t[liftoff] <= MARSA_CLIMB_S + 1e-9 {
                let dt = t[end + 1] - t[end];
                let a = (g[end] + g[end + 1]) / 2.0;
                z += v * dt + 0.5 * a * G0 * dt * dt;
                v += a * G0 * dt;
                z_si += v_si * dt + 0.5 * a * dt * dt;
                v_si += a * dt;
                end += 1;
            }
            if let Some(pad) = pad_m
                && end > start
            {
                let climb = altitude[end] - pad;
                file.climb_ratio = (z > 0.0).then(|| climb / z);
                file.climb_ratio_in_m_s2 = (z_si > 0.0).then(|| climb / z_si);
            }
            // The one-count steps near the pad.
            if let Some(pad) = file.pad_ft {
                let mut near: Vec<f64> = feet
                    .iter()
                    .copied()
                    .filter(|&f| f >= pad && f <= pad + NEAR_PAD_FT)
                    .collect();
                near.sort_by(f64::total_cmp);
                near.dedup();
                let steps: Vec<f64> = near.windows(2).map(|w| w[1] - w[0]).collect();
                let least = steps.iter().copied().fold(f64::INFINITY, f64::min);
                let ones: Vec<f64> = steps.into_iter().filter(|&s| s < 1.5 * least).collect();
                if !ones.is_empty() {
                    let greatest = ones.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    file.count_step_ft = Some([least, greatest]);
                }
            }
            let ours = peak(&log.time_s, &log.altitude_m);
            let other = MARSA_BESIDE
                .iter()
                .find(|(marsa, _)| *marsa == file.id)
                .and_then(|(_, other)| {
                    file.beside = (*other).to_owned();
                    samples.iter().find(|s| id_of(&s.bytes) == *other)
                })
                .and_then(|other| hpr_flightdata::import::read(&other.bytes).ok());
            if let (Some((our_t, our_h)), Some(other)) = (ours, other)
                && let Some((their_t, their_h)) = peak(&other.time_s, &other.altitude_m)
                && their_h > 0.0
            {
                file.apogee_ratio = Some(our_h / their_h);
                file.apogee_ratio_in_meters = Some(our_h / FOOT_M / their_h);
                file.apogee_time_off_s = Some((our_t - t[liftoff]) - their_t);
            }
        }
        files.push(file);
    }
    files
}

fn telemetry_csv_checks(samples: &[Sample]) -> Vec<TelemetryCsvFile> {
    let mut files = Vec::new();
    for sample in samples.iter().filter(|s| s.family == TELEMETRY_CSV_FAMILY) {
        let text = hpr_flightdata::import::decode(&sample.bytes);
        let mut file = TelemetryCsvFile {
            id: id_of(&sample.bytes),
            sixteenths: true,
            whole_meters: true,
            hundredths: true,
            ..TelemetryCsvFile::default()
        };
        let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty());
        let Some(header) = lines.next() else {
            files.push(file);
            continue;
        };
        let names: Vec<&str> = header.split(',').map(str::trim).collect();
        let find = |wanted: &str| names.iter().position(|n| *n == wanted);
        let mut decoded: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let mut time = Vec::new();
        let mut states = Vec::new();
        for line in lines {
            let cells: Vec<&str> = line.split(',').map(str::trim).collect();
            if cells.len() != names.len() {
                continue;
            }
            file.rows += 1;
            *file
                .ptypes
                .entry(find("ptype").map_or("", |i| cells[i]).to_owned())
                .or_default() += 1;
            if find("crc").is_some_and(|i| cells[i] == "FALSE") {
                file.crc_false += 1;
                continue;
            }
            let tick = find("tick")
                .and_then(|i| number(cells[i]))
                .unwrap_or(f64::NAN);
            file.hundredths &= ((tick * 100.0).round() - tick * 100.0).abs() < 1e-6;
            time.push(tick);
            for (index, name) in names.iter().enumerate() {
                match *name {
                    "tick" | "crc" => {}
                    "sense" => {
                        for (k, part) in cells[index].split_whitespace().enumerate() {
                            decoded
                                .entry(format!("sense {}", k + 1))
                                .or_default()
                                .push(number(part).unwrap_or(f64::NAN));
                        }
                    }
                    _ => {
                        let value = number(cells[index]).unwrap_or(f64::NAN);
                        match *name {
                            "acceleration" | "speed" => {
                                file.sixteenths &= (value * 16.0).fract() == 0.0;
                            }
                            "height" => file.whole_meters &= value.fract() == 0.0,
                            "state" => states.push(value),
                            _ => {}
                        }
                        decoded.entry((*name).to_owned()).or_default().push(value);
                    }
                }
            }
        }
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            files.push(file);
            continue;
        };
        file.unknown_columns = log
            .notes
            .iter()
            .filter_map(|n| n.strip_prefix("the column `"))
            .filter_map(|n| n.split_once('`').map(|(name, _)| name.to_owned()))
            .collect();
        let mut counts = [0; 2];
        for (column, ours) in &decoded {
            let read = log.channels.iter().find(|c| &c.column == column);
            compare(&mut counts, ours, read.map_or(&[], |c| &c.values));
            compare(&mut counts, &time, read.map_or(&[], |c| &c.time_s));
        }
        compare(&mut counts, &time, &log.time_s);
        [file.values_compared, file.values_differing] = counts;
        let values = |column: &str| decoded.get(column).cloned().unwrap_or_default();
        let battery = values("v_batt");
        if !battery.is_empty() {
            file.battery_v = Some([
                battery.iter().copied().fold(f64::INFINITY, f64::min),
                battery.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            ]);
        }
        file.events = log.events.iter().map(|e| e.name.clone()).collect();
        file.ground_pressure_in_band = values("ground_pres")
            .first()
            .map(|p| PAD_PRESSURE_PA.contains(p));
        let acceleration = values("acceleration");
        file.pad_acceleration_m_s2 = teensy_median(
            states
                .iter()
                .zip(&acceleration)
                .filter(|(s, _)| **s == 2.0)
                .map(|(_, a)| *a)
                .collect(),
        );
        let (height, speed) = (values("height"), values("speed"));
        let (mut ratios, mut speed_ratios) = (Vec::new(), Vec::new());
        for k in 1..time
            .len()
            .min(height.len())
            .min(speed.len())
            .min(acceleration.len())
            .min(states.len())
        {
            let flying = (3.0..=5.0).contains(&states[k - 1]) && (3.0..=5.0).contains(&states[k]);
            let dt = time[k] - time[k - 1];
            let mean = (speed[k] + speed[k - 1]) / 2.0;
            if flying && mean.abs() > 10.0 {
                ratios.push((height[k] - height[k - 1]) / dt / mean);
            }
            let mean_acceleration = (acceleration[k] + acceleration[k - 1]) / 2.0;
            if flying && mean_acceleration.abs() > 5.0 {
                speed_ratios.push((speed[k] - speed[k - 1]) / dt / mean_acceleration);
            }
        }
        file.climb_over_speed = teensy_median(ratios);
        file.speed_change_over_acceleration = teensy_median(speed_ratios);
        files.push(file);
    }
    files
}

/// A row's `state` and `state_name` as AltOS's exports pair them ([`ALTOS_STATES`]): whether
/// both name one state.
fn altos_pair(number: &str, name: &str) -> bool {
    ALTOS_STATES
        .iter()
        .any(|(n, s)| number.trim().parse::<u8>() == Ok(*n) && s.eq_ignore_ascii_case(name.trim()))
}

fn altos_table_checks(samples: &[Sample]) -> Vec<AltosTableFile> {
    let mut files = Vec::new();
    for sample in samples.iter().filter(|s| s.family == ALTOS_TABLE_FAMILY) {
        let text = hpr_flightdata::import::decode(&sample.bytes);
        let mut file = AltosTableFile {
            id: id_of(&sample.bytes),
            states_altos: true,
            ..AltosTableFile::default()
        };
        let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty());
        let Some(header) = lines.next() else {
            files.push(file);
            continue;
        };
        let names: Vec<&str> = header.split(',').map(str::trim).collect();
        let find = |wanted: &str| names.iter().position(|n| *n == wanted);
        let (state, state_name) = (find("state"), find("state_name"));
        let mut decoded: Vec<Vec<f64>> = vec![Vec::new(); names.len()];
        let mut time = Vec::new();
        let (mut offsets, mut from_pressure) = (Vec::new(), None::<f64>);
        // Each row's altitude and the standard atmosphere's height of its pressure.
        let mut pairs: Vec<(f64, f64)> = Vec::new();
        for line in lines {
            let cells: Vec<&str> = line.split(',').map(str::trim).collect();
            if cells.len() != names.len() {
                continue;
            }
            file.rows += 1;
            if let (Some(s), Some(n)) = (state, state_name) {
                *file
                    .states
                    .entry(format!("{} {}", cells[s], cells[n]))
                    .or_default() += 1;
                file.states_altos &= altos_pair(cells[s], cells[n]);
            }
            let cell = |name: &str| find(name).and_then(|i| number(cells[i]));
            time.push(cell("time").unwrap_or(f64::NAN));
            for (index, name) in names.iter().enumerate() {
                let value = number(cells[index]).unwrap_or(f64::NAN);
                decoded[index].push(if *name == "temperature" {
                    value + 273.15
                } else {
                    value
                });
            }
            if let (Some(altitude), Some(height)) = (cell("altitude"), cell("height")) {
                offsets.push(altitude - height);
            }
            if let (Some(altitude), Some(pressure)) = (cell("altitude"), cell("pressure")) {
                let off = (altitude - standard_height_m(pressure)).abs();
                from_pressure = Some(from_pressure.map_or(off, |worst| worst.max(off)));
                pairs.push((altitude, standard_height_m(pressure)));
            }
        }
        if let Some(&(first, first_standard)) = pairs.first()
            && let Some(&(top, top_standard)) = pairs.iter().max_by(|a, b| a.0.total_cmp(&b.0))
            && top_standard - first_standard > 0.0
        {
            file.altitude_rise_ratio = Some((top - first) / (top_standard - first_standard));
        }
        if !offsets.is_empty() {
            let least = offsets.iter().copied().fold(f64::INFINITY, f64::min);
            let greatest = offsets.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            file.height_offset_spread_m = Some((greatest - least) / 2.0);
        }
        file.altitude_from_pressure_worst_m = from_pressure;
        if let Ok(log) = hpr_flightdata::import::read(&sample.bytes) {
            let columns: Vec<&str> = names
                .iter()
                .copied()
                .filter(|n| !matches!(*n, "time" | "state_name"))
                .collect();
            let read: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
            file.same_columns = read == columns;
            file.events = log.events.iter().map(|e| e.name.clone()).collect();
            let mut counts = [0; 2];
            for (index, name) in names.iter().enumerate() {
                if matches!(*name, "time" | "state_name") {
                    continue;
                }
                let read = log.channels.iter().find(|c| c.column == *name);
                compare(
                    &mut counts,
                    &decoded[index],
                    read.map_or(&[], |c| &c.values),
                );
                compare(&mut counts, &time, read.map_or(&[], |c| &c.time_s));
            }
            // The primary track keeps the first row at each time, as AltOS's exports repeat one.
            let height = find("height").map_or(&[][..], |i| &decoded[i][..]);
            let mut kept: Vec<usize> = Vec::new();
            for (k, &t) in time.iter().enumerate() {
                if kept.last().is_none_or(|&last| t > time[last]) {
                    kept.push(k);
                }
            }
            let at = |values: &[f64]| -> Vec<f64> {
                kept.iter()
                    .filter_map(|&k| values.get(k).copied())
                    .collect()
            };
            compare(&mut counts, &at(&time), &log.time_s);
            compare(&mut counts, &at(height), &log.altitude_m);
            [file.values_compared, file.values_differing] = counts;
        }
        files.push(file);
    }
    files
}

/// The formulas a record's notes keep for a column, by line: each run's formula with `{row}` its
/// line, as [`hpr_flightdata::altos`] writes them, `` `F` on line N `` or `` `F` on lines A–B ``.
fn formulas_in_notes(log: &FlightLog, column: &str) -> BTreeMap<usize, String> {
    let mut out = BTreeMap::new();
    let starts = [
        format!("the `{column}` column's "),
        format!("the `{column}` column has "),
    ];
    for note in &log.notes {
        if !starts.iter().any(|s| note.starts_with(s.as_str())) {
            continue;
        }
        let Some((_, runs)) = note.split_once("own row: ") else {
            continue;
        };
        for run in runs.split("; ") {
            let Some(rest) = run.strip_prefix('`') else {
                continue;
            };
            let Some((formula, lines)) = rest.rsplit_once("` on ") else {
                continue;
            };
            let range = if let Some(one) = lines.strip_prefix("line ") {
                one.parse::<usize>().ok().map(|n| (n, n))
            } else {
                lines
                    .strip_prefix("lines ")
                    .and_then(|r| r.split_once('–'))
                    .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
            };
            if let Some((first, last)) = range {
                for line in first..=last {
                    out.insert(line, formula.replace("{row}", &line.to_string()));
                }
            }
        }
    }
    out
}

/// Whether a column is AltOS's temperature in °C, which the reader writes in K: `temperature`,
/// with no unit after it or `C` or `°C`.
fn celsius(name: &str) -> bool {
    matches!(
        name.strip_prefix("temperature").map(str::trim),
        Some("" | "(C)" | "(°C)")
    )
}

fn resaved_checks(samples: &[Sample]) -> Vec<ResavedFile> {
    let mut files = Vec::new();
    for sample in samples.iter().filter(|s| s.family == RESAVED_FAMILY) {
        let text = hpr_flightdata::import::decode(&sample.bytes);
        let mut file = ResavedFile {
            id: id_of(&sample.bytes),
            ..ResavedFile::default()
        };
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            files.push(file);
            continue;
        };
        let mut lines = text
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l.trim()))
            .filter(|(_, l)| !l.is_empty());
        let Some((_, header)) = lines.next() else {
            files.push(file);
            continue;
        };
        let names: Vec<&str> = header
            .trim_start_matches('#')
            .split(',')
            .map(str::trim)
            .collect();
        let rows: Vec<(usize, Vec<&str>)> = lines
            .map(|(n, l)| (n, l.split(',').map(str::trim).collect()))
            .collect();
        file.rows = rows.len();
        let notes: Vec<BTreeMap<usize, String>> = names
            .iter()
            .map(|name| formulas_in_notes(&log, name))
            .collect();
        file.formula_columns_not_channels = true;
        // The columns that can be channels: those not formulas alone.
        let channel_columns: Vec<bool> = (0..names.len())
            .map(|index| {
                !rows.is_empty()
                    && !rows
                        .iter()
                        .all(|(_, cells)| cells.get(index).is_some_and(|c| c.starts_with('=')))
            })
            .collect();
        for (index, name) in names.iter().enumerate() {
            let formulas = rows
                .iter()
                .filter(|(_, cells)| cells.get(index).is_some_and(|c| c.starts_with('=')))
                .count();
            if formulas > 0 {
                *file.formulas.entry((*name).to_owned()).or_default() += formulas;
                if formulas == rows.len() {
                    file.formula_columns_not_channels &=
                        !log.channels.iter().any(|c| c.column == *name);
                }
            }
            for (line, cells) in &rows {
                if let Some(cell) = cells.get(index).filter(|c| c.starts_with('=')) {
                    file.formulas_compared += 1;
                    if notes[index].get(line).map(String::as_str) != Some(*cell) {
                        file.formulas_differing += 1;
                    }
                }
            }
            if let Some(unit) = name
                .strip_suffix(')')
                .and_then(|n| n.rsplit_once(" ("))
                .map(|(_, unit)| unit)
                && !unit.is_empty()
                && formulas < rows.len()
            {
                // The column's own channel: as many of its name before it as are channels.
                let before = (0..index)
                    .filter(|&i| names[i] == *name && channel_columns[i])
                    .count();
                let quantity = log
                    .channels
                    .iter()
                    .filter(|c| c.column == *name)
                    .nth(before)
                    .map_or(String::new(), |c| format!("{:?}", c.quantity));
                file.units.push(((*name).to_owned(), quantity));
            }
        }
        // The channels against the cells: each channel the next column of its name that isn't
        // formulas alone or text, as written but GPS positions of exactly (0, 0) as gaps.
        let lat = names.iter().position(|n| *n == "latitude");
        let lon = names.iter().position(|n| *n == "longitude");
        let gnss_altitude = lon.and_then(|lon| {
            names
                .iter()
                .enumerate()
                .skip(lon + 1)
                .find(|(_, n)| n.starts_with("altitude"))
                .map(|(i, _)| i)
        });
        let unfixed: Vec<bool> = rows
            .iter()
            .map(|(_, cells)| {
                let at = |i: Option<usize>| i.and_then(|i| cells.get(i)).and_then(|c| number(c));
                at(lat) == Some(0.0) && at(lon) == Some(0.0)
            })
            .collect();
        let mut used = vec![false; names.len()];
        let time: Vec<f64> = rows
            .iter()
            .map(|(_, cells)| {
                names
                    .iter()
                    .position(|n| *n == "time")
                    .and_then(|i| number(cells[i]))
                    .unwrap_or(f64::NAN)
            })
            .collect();
        let mut counts = [0; 2];
        for channel in &log.channels {
            let Some(index) = (0..names.len()).find(|&i| !used[i] && names[i] == channel.column)
            else {
                compare(&mut counts, &[], &channel.values);
                continue;
            };
            used[index] = true;
            let blanked = [lat, lon, gnss_altitude].contains(&Some(index));
            let ours: Vec<f64> = rows
                .iter()
                .zip(&unfixed)
                .map(|((_, cells), &unfixed)| {
                    let cell = cells.get(index).copied().unwrap_or("");
                    match number(cell) {
                        Some(_) if blanked && unfixed => f64::NAN,
                        Some(value) if celsius(names[index]) => value + 273.15,
                        Some(value) => value,
                        None => {
                            if !cell.is_empty() && !cell.starts_with('=') {
                                file.not_numbers += 1;
                            }
                            f64::NAN
                        }
                    }
                })
                .collect();
            compare(&mut counts, &ours, &channel.values);
            compare(&mut counts, &time, &channel.time_s);
        }
        [file.values_compared, file.values_differing] = counts;
        file.no_primary_track = log.time_s.is_empty() && log.altitude_m.is_empty();
        files.push(file);
    }
    files
}

/// Whether a record's events are each a state named as AltOS's exports name it, none `state N`.
fn states_named(events: &[String]) -> bool {
    !events.is_empty() && events.iter().all(|e| !e.starts_with("state "))
}

/// The files that fail a check, as lines of the unread list.
pub(super) fn unread(checks: &Checks) -> Vec<String> {
    let within = |value: Option<f64>, band: &std::ops::RangeInclusive<f64>| {
        value.is_some_and(|v| band.contains(&v))
    };
    let mut out = Vec::new();
    for f in &checks.marsa {
        if f.values_compared == 0
            || f.values_differing > 0
            || !f.figures_agree
            || !f.quantities_read
            || !within(f.apogee_ratio, &MARSA_APOGEE_BAND)
            || !within(f.climb_ratio, &UNIT_BAND)
            || !f.pad_g.is_some_and(|g| g.abs() < MARSA_PAD_G_BOUND)
        {
            out.push(format!(
                "  {} (MARSA): fails its checks: {} of {} values differ, figures agree: {}, the \
                 quantities read: {}, apogee over the other logger's {:?}, climb ratio {:?}, \
                 acceleration on the pad {:?} g, one-count steps {:?} ft",
                f.id,
                f.values_differing,
                f.values_compared,
                f.figures_agree,
                f.quantities_read,
                f.apogee_ratio,
                f.climb_ratio,
                f.pad_g,
                f.count_step_ft
            ));
        }
    }
    for f in &checks.telemetry_csv {
        if f.values_compared == 0
            || f.values_differing > 0
            || !f.unknown_columns.is_empty()
            || !f.sixteenths
            || !f.whole_meters
            || !f.hundredths
            || !f
                .battery_v
                .is_some_and(|[a, b]| BATTERY_BAND_V.contains(&a) && BATTERY_BAND_V.contains(&b))
            || !states_named(&f.events)
            || f.ground_pressure_in_band != Some(true)
            || !within(f.climb_over_speed, &UNIT_BAND)
            || !within(f.speed_change_over_acceleration, &UNIT_BAND)
        {
            out.push(format!(
                "  {} (AltOS telemetry CSV): fails its checks: {} of {} values differ, unknown \
                 columns {:?}, sixteenths {}, whole meters {}, hundredths {}, battery {:?} V, \
                 events {:?}, ground pressure in its band {:?}, climb over speed {:?}, speed's \
                 change over acceleration {:?}",
                f.id,
                f.values_differing,
                f.values_compared,
                f.unknown_columns,
                f.sixteenths,
                f.whole_meters,
                f.hundredths,
                f.battery_v,
                f.events,
                f.ground_pressure_in_band,
                f.climb_over_speed,
                f.speed_change_over_acceleration
            ));
        }
    }
    for f in &checks.altos_table {
        if f.values_compared == 0
            || f.values_differing > 0
            || !f.same_columns
            || !f.states_altos
            || !states_named(&f.events)
            || !within(f.altitude_rise_ratio, &UNIT_BAND)
            || !f
                .height_offset_spread_m
                .is_some_and(|s| s <= HEIGHT_OFFSET_BOUND_M)
        {
            out.push(format!(
                "  {} (a table cut from an AltOS export): fails its checks: {} of {} values \
                 differ, the same columns: {}, states AltOS's: {}, events {:?}, the altitude's \
                 rise over its pressure's {:?}, `altitude` less `height` spread {:?} m",
                f.id,
                f.values_differing,
                f.values_compared,
                f.same_columns,
                f.states_altos,
                f.events,
                f.altitude_rise_ratio,
                f.height_offset_spread_m
            ));
        }
    }
    for f in &checks.resaved {
        if f.formulas_compared == 0
            || f.formulas_differing > 0
            || !f.formula_columns_not_channels
            || f.values_compared == 0
            || f.values_differing > 0
            || !f.no_primary_track
        {
            out.push(format!(
                "  {} (a re-saved AltOS export): fails its checks: {} of {} formulas differ from \
                 the notes, formula columns no channels: {}, {} of {} values differ, no primary \
                 track: {}",
                f.id,
                f.formulas_differing,
                f.formulas_compared,
                f.formula_columns_not_channels,
                f.values_differing,
                f.values_compared,
                f.no_primary_track
            ));
        }
    }
    out
}

/// A figure to some digits, `–` for none.
fn figure(value: Option<f64>, digits: usize) -> String {
    value.map_or("–".to_owned(), |v| format!("{v:.digits$}"))
}

/// The report's section for this increment.
pub(super) fn markdown(checks: &Checks) -> String {
    let mut out = String::new();
    if !checks.marsa.is_empty() {
        let mut rows = String::new();
        for f in &checks.marsa {
            rows += &format!(
                "| `{}` | {} | {} | {} | {} | {} | {} | `{}` | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                f.id,
                f.settings,
                f.samples,
                f.values_compared,
                f.values_differing,
                if f.figures_agree { "yes" } else { "no" },
                if f.quantities_read { "yes" } else { "no" },
                f.beside,
                figure(f.apogee_ratio, 3),
                figure(f.apogee_ratio_in_meters, 2),
                figure(f.pad_ft, 1),
                figure(f.pad_g, 2),
                figure(f.apogee_time_off_s, 1),
                figure(f.climb_ratio, 3),
                figure(f.climb_ratio_in_m_s2, 3),
                f.count_step_ft
                    .map_or("–".to_owned(), |[a, b]| format!("{a:.1}–{b:.1}")),
            );
        }
        out += &format!(
            "\nThe {} MARSA downloads (M7.1c21), each decoded here apart from the reader: \
             *Settings* counts its `<NAME> value` lines, *Samples* its rows, and each sample's \
             time in s, altitude in ft and acceleration in g is compared with the reader's \
             record, six comparisons a sample: the time and altitude on the primary track and \
             the times and values of the altitude and acceleration channels; *Figures* is \
             whether the record's figures are the settings, by name and value, and *Quantities* whether it read the altitude as a height above its \
             zero and the acceleration as an acceleration. *Beside* is the other logger in the \
             same flight, the AltOS board in the flight's folder, whose other log is the other \
             stage's. \
             *Apogee ratio*, the check of the feet, is the highest altitude over that board's \
             highest height above its pad, both in m, which must be within {:.2}–{:.2}; *In \
             meters* the same were the altitude read in meters. *Pad, ft* is the altitude before \
             liftoff, the first acceleration above {LIFTOFF_G} g, the median: the MARSA's zero \
             isn't its reading on the pad. *Pad, g* is the acceleration's median there, which \
             must be within {MARSA_PAD_G_BOUND} g of 0, nearer 0 than 1 g, as gravity is taken \
             out. *Time off, s* is the time from liftoff to the highest altitude less the \
             board's, a measure. *Climb*, the check of the g, is the barometric climb over the \
             first {MARSA_CLIMB_S} s after liftoff over the acceleration integrated twice from rest \
             over them, which must be within {:.1}–{:.3}, the band AltOS's `.eeprom` reader \
             holds its own climb to; *In m/s²* the same were the acceleration read in m/s². \
             *Count step, ft* is the least and greatest step between the altitudes written \
             within {NEAR_PAD_FT} ft above the pad's reading that are one count, under 1.5 \
             times the least: a measure, from which the format's step, {:.1} ft, is taken, so it \
             holds near the pad only, a count spanning more feet where the air is thinner.\n\n\
             | File | Settings | Samples | Values compared | Differing | Figures | Quantities | \
             Beside | Apogee ratio | In meters | Pad, ft | Pad, g | Time off, s | Climb | In \
             m/s² | Count step, ft |\n\
             |---|---:|---:|---:|---:|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|\n{rows}",
            checks.marsa.len(),
            MARSA_APOGEE_BAND.start(),
            MARSA_APOGEE_BAND.end(),
            UNIT_BAND.start(),
            UNIT_BAND.end(),
            hpr_flightdata::marsa::ALTITUDE_STEP_M / FOOT_M,
        );
    }
    if !checks.telemetry_csv.is_empty() {
        let yes = |b: bool| if b { "yes" } else { "no" };
        let mut rows = String::new();
        for f in &checks.telemetry_csv {
            let ptypes: Vec<String> = f.ptypes.iter().map(|(p, n)| format!("{p} ×{n}")).collect();
            rows += &format!(
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                f.id,
                f.rows,
                f.crc_false,
                ptypes.join(", "),
                f.values_compared,
                f.values_differing,
                if f.unknown_columns.is_empty() {
                    "none".to_owned()
                } else {
                    f.unknown_columns.join(", ")
                },
                yes(f.sixteenths),
                yes(f.whole_meters),
                yes(f.hundredths),
                f.battery_v
                    .map_or("–".to_owned(), |[a, b]| format!("{a:.2}–{b:.2}")),
                f.events.join(", "),
                figure(f.pad_acceleration_m_s2, 3),
                figure(f.climb_over_speed, 3),
                figure(f.speed_change_over_acceleration, 3),
            );
        }
        out += &format!(
            "\nAltOS's radio telemetry decoded to a table (M7.1c21), each read by its header's \
             names, the fields of the TeleMega's `0x09` packet in *AltOS Telemetry* (§2.3, Table \
             6), and decoded here apart from the reader: *Rows* counts them, *CRC false* those \
             left out as packets that failed their check, and *Packet types* the types they \
             name; each value of the rest and its time is compared with the reader's channel, \
             and each time with its primary track, *Values compared* counting the comparisons; \
             *Unknown* \
             lists the columns it kept as names it doesn't know. The resolution the packet \
             carries, which no wrong unit need fail: *Sixteenths* is whether every acceleration \
             and speed is a whole number of sixteenths, *Meters* every height a whole meter, \
             *Hundredths* every tick a whole hundredth of a second. *Battery, V* is `v_batt`'s least and greatest, which must be \
             within {:.1}–{:.1} V, a single LiPo cell's, 3.7 V nominal (*The Altus Metrum \
             System*, §3.2): read as the document's 12-bit counts behind its 100k/27k divider, 4 \
             V would be about 1,055. *Events* are the reader's, each state named as AltOS's \
             exports name its number, none `state N`, and the first `ground_pres` must be inside \
             {}–{} kPa, the value not written. *Pad accel* is the acceleration's median in the \
             pad state, m/s², about 0 with gravity taken out, a measure. The checks of the units: \
             *Climb over speed* is the median from boost to coast of the height's change between \
             two rows over their time, over their mean speed, where it is over 10 m/s; *Speed \
             over accel* the same of the speed's change over the mean acceleration, where it is \
             over 5 m/s². Each must be within {:.1}–{:.3}, the band of the `.eeprom` reader's \
             climb check: a speed or an acceleration in the packet's sixteenths, a height in \
             feet or a tick in its 10 ms counts would miss it.\n\n\
             | File | Rows | CRC false | Packet types | Values compared | Differing | Unknown | \
             Sixteenths | Meters | Hundredths | Battery, V | Events | Pad accel | Climb over \
             speed | Speed over accel |\n\
             |---|---:|---:|---|---:|---:|---|---|---|---|---|---|---:|---:|---:|\n{rows}",
            BATTERY_BAND_V.start(),
            BATTERY_BAND_V.end(),
            PAD_PRESSURE_PA.start() / 1_000.0,
            PAD_PRESSURE_PA.end() / 1_000.0,
            UNIT_BAND.start(),
            UNIT_BAND.end(),
        );
    }
    if !checks.altos_table.is_empty() {
        let mut rows = String::new();
        for f in &checks.altos_table {
            let states: Vec<String> = f.states.iter().map(|(s, n)| format!("{s} ×{n}")).collect();
            rows += &format!(
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                f.id,
                f.rows,
                states.join(", "),
                if f.states_altos { "yes" } else { "no" },
                f.values_compared,
                f.values_differing,
                if f.same_columns { "yes" } else { "no" },
                f.events.join(", "),
                figure(f.height_offset_spread_m, 4),
                figure(f.altitude_rise_ratio, 4),
                figure(f.altitude_from_pressure_worst_m, 4),
            );
        }
        out += &format!(
            "\nTables cut from an AltOS export (M7.1c21), AltOS's own column names with no \
             `version,serial,flight` before them, each read by its states and decoded here apart \
             from the reader: *States* counts each row's `state` and `state_name`, and *AltOS's* \
             is whether every pair is one AltOS's exports write, 2 `pad` to 8 `landed`; each \
             value and its time is compared with the reader's channel, and the primary track's \
             times and heights with the first row at each time, as AltOS repeats one, *Values \
             compared* counting the comparisons; the channels must be the table's \
             columns but the clock and `state_name`, in order, and *Events* the reader's, each \
             state named as AltOS's exports name it. *Height offset, m* is half the \
             spread of `altitude` less `height`, which AltOS writes as its barometric altitude \
             less the pad's, one offset, not written as it gives the site's elevation: each to \
             the centimeter, it must be within {HEIGHT_OFFSET_BOUND_M:.2} m. *Rise ratio*, the \
             check of the units, is `altitude`'s rise from the first row to its highest over the \
             standard atmosphere's height's rise of `pressure` over the same rows, 44,330.77 × (1 \
             − (P / 101,325)^0.190263) m, which must be within {:.1}–{:.3}, the band of the \
             `.eeprom` reader's climb check: an altitude in feet, or a pressure in hPa, would miss \
             it. *From pressure, m* is the most `altitude` differs from that height: a measure of \
             AltOS's own conversion.\n\n\
             | File | Rows | States | AltOS's | Values compared | Differing | Same columns | \
             Events | Height offset, m | Rise ratio | From pressure, m |\n\
             |---|---:|---|---|---:|---:|---|---|---:|---:|---:|\n{rows}",
            UNIT_BAND.start(),
            UNIT_BAND.end(),
        );
    }
    if !checks.resaved.is_empty() {
        let mut rows = String::new();
        for f in &checks.resaved {
            let formulas: Vec<String> = f
                .formulas
                .iter()
                .map(|(c, n)| format!("`{c}` {n}"))
                .collect();
            let units: Vec<String> = f
                .units
                .iter()
                .map(|(c, q)| format!("`{c}` {}", if q.is_empty() { "–" } else { q }))
                .collect();
            rows += &format!(
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                f.id,
                f.rows,
                formulas.join(", "),
                f.formulas_compared,
                f.formulas_differing,
                if f.formula_columns_not_channels {
                    "yes"
                } else {
                    "no"
                },
                units.join(", "),
                f.values_compared,
                f.values_differing,
                f.not_numbers,
                if f.no_primary_track { "none" } else { "kept" },
            );
        }
        out += &format!(
            "\nAltOS exports a spreadsheet re-saved (M7.1c21), decoded here apart from the \
             reader: *Formulas* counts the cells of each column that hold a formula, `=…`; each \
             is rebuilt from the reader's notes, which give a column's formulas run by run, \
             `{{row}}` standing for each cell's own row, and compared with the file's cell \
             (*Compared*, *Differing*); *Not channels* is whether every column of formulas alone \
             is left out of the channels, as text. *Units* lists the columns whose name carries a \
             unit after it and the quantity the reader read each as: AltOS's where the unit is \
             the one it reads the column in, kept as written (`Unrecognized`) where it isn't. \
             Each value of the channels, a formula or a cell that isn't a number a gap, and its \
             time is compared with the reader's channel, *Values compared* counting the \
             comparisons; *Not numbers* counts the cells, formulas apart, \
             read as gaps. *Primary track* is none, as the header names no `height`.\n\n\
             | File | Rows | Formulas | Compared | Differing | Not channels | Units | Values \
             compared | Differing | Not numbers | Primary track |\n\
             |---|---:|---|---:|---:|---|---|---:|---:|---:|---|\n{rows}"
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marsa(apogee: f64, climb: f64, step: f64) -> MarsaFile {
        MarsaFile {
            id: "m".to_owned(),
            values_compared: 10,
            figures_agree: true,
            quantities_read: true,
            apogee_ratio: Some(apogee),
            climb_ratio: Some(climb),
            pad_g: Some(-0.12),
            count_step_ft: Some([29.0, step]),
            ..MarsaFile::default()
        }
    }

    /// Each MARSA check fails a file past its bound, and only then.
    #[test]
    fn marsa_files_fail_on_any_check() {
        let checks = |files| Checks {
            marsa: files,
            ..Checks::default()
        };
        assert!(unread(&checks(vec![marsa(0.98, 1.0, 29.3)])).is_empty());
        // The count step is a measure, not a check.
        assert!(unread(&checks(vec![marsa(0.98, 1.0, 40.0)])).is_empty());
        for bad in [
            marsa(0.94, 1.0, 29.3),
            marsa(1.06, 1.0, 29.3),
            marsa(0.98, 0.59, 29.3),
            marsa(0.98, 1.67, 29.3),
            MarsaFile {
                values_differing: 1,
                ..marsa(0.98, 1.0, 29.3)
            },
            MarsaFile {
                figures_agree: false,
                ..marsa(0.98, 1.0, 29.3)
            },
            MarsaFile {
                pad_g: Some(0.98),
                ..marsa(0.98, 1.0, 29.3)
            },
        ] {
            assert_eq!(unread(&checks(vec![bad.clone()])).len(), 1, "{bad:?}");
        }
    }

    /// Each check of the other three layouts fails a file past its bound, and only then.
    #[test]
    fn the_tables_fail_on_any_check() {
        let telemetry = TelemetryCsvFile {
            id: "t".to_owned(),
            values_compared: 10,
            sixteenths: true,
            whole_meters: true,
            hundredths: true,
            battery_v: Some([4.0, 4.1]),
            events: vec!["pad".to_owned(), "boost".to_owned()],
            ground_pressure_in_band: Some(true),
            climb_over_speed: Some(1.02),
            speed_change_over_acceleration: Some(0.93),
            ..TelemetryCsvFile::default()
        };
        let table = AltosTableFile {
            id: "a".to_owned(),
            values_compared: 10,
            states_altos: true,
            same_columns: true,
            height_offset_spread_m: Some(0.01),
            altitude_rise_ratio: Some(1.0),
            events: vec!["boost".to_owned(), "coast".to_owned()],
            ..AltosTableFile::default()
        };
        let resaved = ResavedFile {
            id: "r".to_owned(),
            formulas_compared: 3,
            formula_columns_not_channels: true,
            values_compared: 10,
            no_primary_track: true,
            ..ResavedFile::default()
        };
        let all = |telemetry: &TelemetryCsvFile, table: &AltosTableFile, resaved: &ResavedFile| {
            unread(&Checks {
                telemetry_csv: vec![telemetry.clone()],
                altos_table: vec![table.clone()],
                resaved: vec![resaved.clone()],
                ..Checks::default()
            })
            .len()
        };
        assert_eq!(all(&telemetry, &table, &resaved), 0);
        for bad in [
            TelemetryCsvFile {
                values_differing: 1,
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                unknown_columns: vec!["x".to_owned()],
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                sixteenths: false,
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                battery_v: Some([4.0, 1_055.0]),
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                events: vec!["pad".to_owned(), "state 9".to_owned()],
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                ground_pressure_in_band: Some(false),
                ..telemetry.clone()
            },
            // Speed in the packet's sixteenths, height in feet, the tick in its 10 ms counts.
            TelemetryCsvFile {
                climb_over_speed: Some(1.02 / 16.0),
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                climb_over_speed: Some(1.02 * 3.28),
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                speed_change_over_acceleration: Some(0.93 / 100.0),
                ..telemetry.clone()
            },
            TelemetryCsvFile {
                speed_change_over_acceleration: None,
                ..telemetry.clone()
            },
        ] {
            assert_eq!(all(&bad, &table, &resaved), 1, "{bad:?}");
        }
        for bad in [
            AltosTableFile {
                values_differing: 1,
                ..table.clone()
            },
            AltosTableFile {
                states_altos: false,
                ..table.clone()
            },
            AltosTableFile {
                same_columns: false,
                ..table.clone()
            },
            AltosTableFile {
                height_offset_spread_m: Some(0.0101),
                ..table.clone()
            },
            AltosTableFile {
                altitude_rise_ratio: Some(3.28),
                ..table.clone()
            },
            AltosTableFile {
                events: vec!["boost".to_owned(), "state 9".to_owned()],
                ..table.clone()
            },
        ] {
            assert_eq!(all(&telemetry, &bad, &resaved), 1, "{bad:?}");
        }
        for bad in [
            ResavedFile {
                formulas_differing: 1,
                ..resaved.clone()
            },
            ResavedFile {
                formula_columns_not_channels: false,
                ..resaved.clone()
            },
            ResavedFile {
                values_differing: 1,
                ..resaved.clone()
            },
            ResavedFile {
                no_primary_track: false,
                ..resaved.clone()
            },
        ] {
            assert_eq!(all(&telemetry, &table, &bad), 1, "{bad:?}");
        }
    }

    /// Only AltOS's temperature in °C is read in K.
    #[test]
    fn celsius_columns() {
        assert!(
            celsius("temperature") && celsius("temperature (C)") && celsius("temperature (°C)")
        );
        assert!(!celsius("temperature (F)") && !celsius("temperature (©)") && !celsius("temp"));
    }

    /// The formulas a note gives, rebuilt line by line.
    #[test]
    fn formulas_are_rebuilt_from_the_notes() {
        let log = FlightLog {
            notes: vec![
                "the `altitude (ft)` column's 3 cells are a spreadsheet's formulas, not values. \
                 The column is not a channel. Each formula is kept here as text, not worked out. \
                 `{row}` stands for each cell's own row: `=3.28084*J{row}` on line 2; \
                 `=3.28*J{row}` on lines 3–4"
                    .to_owned(),
            ],
            ..hpr_flightdata::import::read(b"Time,Altitude,Velocity\n0,0,0\n0.5,1,2\n").unwrap()
        };
        let rebuilt = formulas_in_notes(&log, "altitude (ft)");
        assert_eq!(
            rebuilt.into_iter().collect::<Vec<_>>(),
            [
                (2, "=3.28084*J2".to_owned()),
                (3, "=3.28*J3".to_owned()),
                (4, "=3.28*J4".to_owned())
            ]
        );
        assert!(formulas_in_notes(&log, "altitude").is_empty());
    }

    /// AltOS's pairs, and only those.
    #[test]
    fn altos_pairs() {
        assert!(altos_pair("3", " boost"));
        assert!(!altos_pair("3", "ignition"));
        assert!(!altos_pair("1", "pad"));
    }
}
