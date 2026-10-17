//! The last binaries (M7.1c23): an AltiMAX's own `.ALS` file against the `.xls` export of the same
//! flight, and an Entacore AIM's `.xtra` files and raw memory dump against each other, each
//! decoded here apart from the readers and held to what its units must keep
//! ([ADR-274][adr-274]).
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md

use std::collections::BTreeMap;

use hpr_flightdata::entacore_raw;
use hpr_flightdata::log::FlightLog;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{Sample, hex, read_altimax, standard_height_m};

/// The family of AltiMAX `.ALS` files, which the catalog files as binary.
pub(super) const ALS_FAMILY: &str = "altimax-als";
/// The family of AIM `.xtra` files, as the catalog names it.
pub(super) const XTRA_FAMILY: &str = "entacore-xtra";
/// The family of AIM raw dumps, which the catalog files as binary.
pub(super) const DUMP_FAMILY: &str = "entacore-dump";
/// The increment that reads these families.
pub(super) const INCREMENT: &str = "M7.1c23";

/// Why a file the catalog files as binary is an AltiMAX `.ALS`.
pub(super) const ALS: &str = "an AltiMAX altimeter's own file of a flight, `.ALS`: a header of \
    32 bytes, then a record of 22 bytes a sample, its pressures, acceleration, event and one \
    voltage channel's count; filed as binary";
/// Why a file the catalog files as binary is an AIM raw dump.
pub(super) const DUMP: &str = "an Entacore AIM XTRA's memory as its software downloads it, \
    raw: recordings back to back, each record a type byte and its big-endian fields between \
    bytes that count the clock's ticks, then erased memory; filed as binary";

/// Each `.ALS` by id, and the `.xls` export of the same flight beside it.
const ALS_BESIDE: [(&str, &str); 1] = [("96bad107a7162e43", "444687655195f5c8")];

/// One flight's AIM files by id: the `.xtra` cut short, the whole `.xtra` the software made of
/// the dump, and the raw dump.
const AIM_FLIGHT: [&str; 3] = ["80ceccffb671441d", "073f4a6678b4bb0b", "b08cb7688c29d173"];

/// The factors the sample export turns each voltage channel's count into volts by, worked out
/// from it: channels 1 and 5 share one, 2 to 4 another. No document states them.
const VOLTS_PER_COUNT: [(f64, f64); 5] = [
    (1.0, 3.218_215_77 / 1024.0),
    (2.0, 5.463_126_83 / 2048.0),
    (3.0, 5.463_126_83 / 2048.0),
    (4.0, 5.463_126_83 / 2048.0),
    (5.0, 3.218_215_77 / 1024.0),
];

/// The export's speed before apogee: the acceleration's counts summed from liftoff, times
/// 0.01 g at 9.8065 m/s² and the 5 ms step, 0.000 490 325 m/s a count, as a whole number of
/// billionths.
const SPEED_PER_COUNT_E9: i64 = 490_325;

/// The export's event codes for liftoff and apogee.
const LIFTOFF_CODE: f64 = 3.0;
const APOGEE_CODE: f64 = 9.0;

/// The most a 15-digit value may differ from the export's, in units of its 15th digit: one,
/// the writer's rounding at a tie.
const MAX_UNITS_15: u64 = 1;

/// The raw dump's types as the survey reads them: the type, the bytes after it, the name of its
/// first field's channel, and its fields.
const AIM_TYPES: [(u8, usize, &str, usize); 16] = [
    (0x02, 2, "acceleration", 1),
    (0x03, 3, "pressure", 1),
    (0x04, 2, "battery 1 voltage", 1),
    (0x05, 2, "battery 2 voltage", 1),
    (0x06, 2, "temperature", 1),
    (0x07, 2, "A Voltage", 1),
    (0x08, 2, "B Voltage", 1),
    (0x09, 2, "C Voltage", 1),
    (0x0a, 2, "D Voltage", 1),
    (0x0b, 4, "lat. X accel.", 2),
    (0x0c, 6, "Gyro X", 3),
    (0x0d, 6, "Mag X", 3),
    (0x0e, 13, "lat", 5),
    (0x10, 2, "state", 9),
    (0x12, 7, "GPS Time of Week", 5),
    (0x15, 8, "quaternion 1", 4),
];

/// A recording's opening type and the bytes after it.
const AIM_RECORDING: u8 = 0x13;
const AIM_RECORDING_BYTES: usize = 101;

/// Standard gravity, m/s².
const G0: f64 = 9.806_65;

/// The band a time to apogee may lie in, over √(2h/g): within a factor of 2, as the Teensy
/// logger's (M7.1c18).
const APOGEE_TIME_BAND: std::ops::RangeInclusive<f64> = 0.5..=2.0;

/// The band the accelerometers' pad reading may lie in, g: an accelerometer's at rest, within
/// 15 % of 1 g ([`super::held::REST_BAND_G`], M7.1c20's), which a reading in another unit, or at
/// another of the sensor's ranges, each 2 or more times off, can't reach. The Teensy logger's
/// 3 % (M7.1c18) can't hold this sensor: the maker's own `.xtra` reads it a few percent low on
/// the pad too, value for value, and the report prints by how much.
const PAD_G_BAND: std::ops::RangeInclusive<f64> = super::held::REST_BAND_G;

/// The pad's readings: the last 10 s before liftoff.
const PAD_WINDOW_S: f64 = 10.0;

/// The axial reading, g, above which the rocket is taken to be under thrust: liftoff is the
/// first sample of the unbroken run above it that holds `state 1`, the logger's launch detect,
/// which comes about a second into the boost.
const BOOST_G: f64 = 1.5;

/// This increment's checks, one list a layout.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(super) struct Checks {
    als: Vec<AlsFile>,
    aim: Vec<AimFlight>,
}

/// An `.ALS` against the export beside it.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct AlsFile {
    id: String,
    export: String,
    /// The records the reader read, and the export's rows.
    records: usize,
    rows: usize,
    /// The values the file stores that the export writes unchanged: both pressures, the event
    /// code and the voltage channel, compared bit for bit.
    stored_compared: usize,
    stored_differing: usize,
    /// The acceleration, the voltages through [`VOLTS_PER_COUNT`], the clock and the speed
    /// before apogee, compared at the export's 15 significant digits: how many, how many
    /// differ, and the most, in units of the 15th digit.
    accel: [u64; 3],
    voltage: [u64; 3],
    time: [u64; 3],
    speed: [u64; 3],
    /// The export's rows from apogee on, whose speed the software works out from the pressure,
    /// not compared.
    speed_not_compared: usize,
    /// Whether the events are the export's, name for name.
    events_equal: bool,
    events: usize,
    /// The heights: the standard atmosphere's height of each pressure less that of the header's
    /// zero pressure, against the export's, the most they differ, m, filtered and raw; and the
    /// bound, half of what one pascal moves the zero's height.
    height_off_m: [Option<f64>; 2],
    height_bound_m: Option<f64>,
}

/// One flight's AIM files against each other.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct AimFlight {
    ids: Vec<String>,
    /// Whether the short `.xtra`'s bytes are the whole one's first.
    short_is_prefix: bool,
    /// The samples each read: the short `.xtra`, the whole one, the dump's recordings and the
    /// flight's, the last.
    short_samples: usize,
    xtra_samples: usize,
    recordings: usize,
    flight_samples: usize,
    /// The other recordings' samples, and the greatest climb any makes over its first
    /// pressure, m, in the standard atmosphere: none is a flight.
    other_samples: usize,
    other_climb_m: Option<f64>,
    /// The whole `.xtra`'s values against the dump's flight recording, channel by channel, and
    /// the samples the dump holds past the `.xtra`'s.
    values_compared: usize,
    values_differing: usize,
    dump_only_samples: usize,
    /// The times: the `.xtra`'s against the dump's ticks summed here as the software sums them,
    /// in 32-bit floats, a tick byte at a time; and the dump reader's against the ticks in ms.
    xtra_times_compared: usize,
    xtra_times_differing: usize,
    dump_times_compared: usize,
    dump_times_differing: usize,
    /// The short `.xtra` against the whole one's first samples, values and times.
    short_compared: usize,
    short_differing: usize,
    /// The flight's checks, on the dump's clock: the three accelerometers' medians over the
    /// pad's last 10 s, as one magnitude, g, which a tilted rail doesn't move, and the axial
    /// one's alone; the time from liftoff to the least pressure over √(2h/g); the GPS's ms a
    /// tick on the pad and in flight; the `.xtra`'s clock behind the ticks at its last sample,
    /// s, and its climb over the ticks'.
    pad_g: Option<f64>,
    pad_axial_g: Option<f64>,
    /// How long after liftoff `state 1`, the launch detect, comes, s.
    #[serde(default)]
    detect_after_liftoff_s: Option<f64>,
    apogee_time_ratio: Option<f64>,
    gps_ms_per_tick: [Option<f64>; 2],
    xtra_behind_s: Option<f64>,
    xtra_climb_ratio: Option<f64>,
}

/// A file's id, the first 16 hex digits of its SHA-256.
fn id_of(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))[..16].to_owned()
}

/// A value's 15 significant digits, as a whole number and the power of ten of its last digit,
/// correctly rounded; zero has no digits.
fn digits_15(value: f64) -> Option<(i64, i32)> {
    if value == 0.0 || !value.is_finite() {
        return None;
    }
    let text = format!("{value:.14e}");
    let (mantissa, exponent) = text.split_once('e')?;
    let digits: i64 = mantissa.replace('.', "").parse().ok()?;
    let exponent: i32 = exponent.parse().ok()?;
    Some((digits, exponent - 14))
}

/// How far `ours` is from `theirs` in units of the 15th digit, both rounded to 15 digits;
/// `u64::MAX` where their powers of ten differ.
fn units_15(ours: f64, theirs: f64) -> u64 {
    match (digits_15(ours), digits_15(theirs)) {
        (None, None) => 0,
        (Some((a, ea)), Some((b, eb))) if ea == eb => a.abs_diff(b),
        _ => u64::MAX,
    }
}

/// Adds a comparison at 15 digits to `[compared, differing, most]`.
fn tally(counts: &mut [u64; 3], ours: f64, theirs: f64) {
    let units = units_15(ours, theirs);
    counts[0] += 1;
    if units > 0 {
        counts[1] += 1;
    }
    counts[2] = counts[2].max(units);
}

/// A channel's values by name.
fn values<'a>(log: &'a FlightLog, name: &str) -> Option<&'a [f64]> {
    log.channels
        .iter()
        .find(|c| c.column == name)
        .map(|c| c.values.as_slice())
}

/// Every check of this increment.
pub(super) fn checks(samples: &[Sample]) -> Checks {
    Checks {
        als: als_checks(samples),
        aim: aim_checks(samples),
    }
}

fn als_checks(samples: &[Sample]) -> Vec<AlsFile> {
    let by_id: BTreeMap<String, &Sample> = samples.iter().map(|s| (id_of(&s.bytes), s)).collect();
    let mut files = Vec::new();
    // A file the samples don't hold has no entry, as for the other checks; its export missing
    // fails it.
    for (als_id, export_id) in ALS_BESIDE
        .into_iter()
        .filter(|(id, _)| by_id.contains_key(*id))
    {
        let mut file = AlsFile {
            id: als_id.to_owned(),
            export: export_id.to_owned(),
            ..AlsFile::default()
        };
        let (Some(als), Some(export)) = (by_id.get(als_id), by_id.get(export_id)) else {
            files.push(file);
            continue;
        };
        let (Ok(ours), Ok(theirs)) = (
            hpr_flightdata::altimax_als::read(&als.bytes),
            read_altimax(&export.bytes),
        ) else {
            files.push(file);
            continue;
        };
        let channel = |log: &FlightLog, name: &str| values(log, name).unwrap_or(&[]).to_vec();
        let (filtered, raw) = (channel(&ours, "PRESS FILTER"), channel(&ours, "PRESS RAW"));
        let accel = channel(&ours, "ACCEL");
        let (codes, numbers) = (channel(&ours, "EVENT"), channel(&ours, "VOLT. CHANNEL"));
        file.records = filtered.len();
        file.rows = theirs.time_s.len();
        // The stored values, bit for bit.
        for (ours, theirs) in [
            (&filtered, channel(&theirs, "PRESS FILTER [hpa]")),
            (&raw, channel(&theirs, "PRESS RAW [hpa]")),
            (&codes, channel(&theirs, "EVENT")),
            (&numbers, channel(&theirs, "VOLT. CHANNEL")),
        ] {
            file.stored_compared += ours.len().max(theirs.len());
            file.stored_differing += ours
                .iter()
                .zip(&theirs)
                .filter(|(a, b)| a.to_bits() != b.to_bits())
                .count()
                + ours.len().abs_diff(theirs.len());
        }
        // At 15 digits: the acceleration and the clock row by row.
        let their_accel = channel(&theirs, "ACCEL [m/s2]");
        for (a, b) in accel.iter().zip(&their_accel) {
            tally(&mut file.accel, *a, *b);
        }
        file.accel[1] += accel.len().abs_diff(their_accel.len()) as u64;
        let our_time: Vec<f64> = ours
            .channels
            .first()
            .map(|c| c.time_s.clone())
            .unwrap_or_default();
        for (a, b) in our_time.iter().zip(&theirs.time_s) {
            tally(&mut file.time, *a, *b);
        }
        file.time[1] += our_time.len().abs_diff(theirs.time_s.len()) as u64;
        // The voltages: each channel's counts through its factor, against the export's volts.
        for (number, factor) in VOLTS_PER_COUNT {
            let counts = channel(&ours, &format!("VOLTAGE count (channel {number})"));
            let volts = channel(&theirs, &format!("VOLTAGE [V] (channel {number})"));
            for (count, volt) in counts.iter().zip(&volts) {
                tally(&mut file.voltage, count * factor, *volt);
            }
            file.voltage[0] += counts.len().abs_diff(volts.len()) as u64;
            file.voltage[1] += counts.len().abs_diff(volts.len()) as u64;
        }
        // The speed before apogee: zero before liftoff, then the counts summed.
        let their_speed = channel(&theirs, "SPEED [m/s]");
        let liftoff = codes.iter().position(|c| *c == LIFTOFF_CODE);
        let apogee = codes.iter().position(|c| *c == APOGEE_CODE);
        if let (Some(liftoff), Some(apogee)) = (liftoff, apogee) {
            let mut sum: i64 = 0;
            for (row, speed) in their_speed.iter().enumerate().take(apogee) {
                let ours = if row < liftoff {
                    0.0
                } else {
                    // The counts are whole numbers of 0.01 g: undo the reader's factor.
                    let Some(counts) = accel.get(row) else {
                        file.speed[1] += 1;
                        continue;
                    };
                    sum += (counts / 0.0981).round() as i64;
                    (sum * SPEED_PER_COUNT_E9) as f64 / 1e9
                };
                tally(&mut file.speed, ours, *speed);
            }
            file.speed_not_compared = their_speed.len().saturating_sub(apogee);
        }
        // The events, name for name.
        let names = |log: &FlightLog| {
            log.events
                .iter()
                .map(|e| e.name.clone())
                .collect::<Vec<_>>()
        };
        file.events = ours.events.len();
        file.events_equal = names(&ours) == names(&theirs);
        // The heights, from the header's zero pressure.
        let zero = ours
            .stated
            .figures
            .iter()
            .find(|f| f.name == "zero pressure")
            .and_then(|f| f.values.first().copied());
        if let Some(zero) = zero {
            let base = standard_height_m(zero);
            file.height_bound_m = Some(0.5 * (standard_height_m(zero - 1.0) - base));
            for (slot, (pressures, name)) in
                [(&filtered, "HEIGHT FILTER [m]"), (&raw, "HEIGHT RAW [m]")]
                    .into_iter()
                    .enumerate()
            {
                let heights = channel(&theirs, name);
                // A missing or non-finite height fails the file: `None`, not a skipped row.
                file.height_off_m[slot] = (pressures.len() == heights.len())
                    .then(|| {
                        pressures
                            .iter()
                            .zip(&heights)
                            .map(|(p, h)| (standard_height_m(*p) - base - h).abs())
                            .try_fold(None, |most: Option<f64>, off| {
                                off.is_finite()
                                    .then(|| Some(most.map_or(off, |m| m.max(off))))
                            })
                    })
                    .flatten()
                    .flatten();
            }
        }
        files.push(file);
    }
    files
}

/// One record of a raw dump as the survey walks it: its type, its ticks from the recording's
/// start, and its time as the software sums it, in 32-bit floats a tick byte at a time.
struct Walked {
    raw_type: u8,
    ticks: u64,
    time_f32: f32,
}

/// The raw dump's recordings, walked here apart from the reader; `None` where a byte isn't one
/// the survey knows.
fn walk_dump(bytes: &[u8]) -> Option<Vec<Vec<Walked>>> {
    let mut recordings: Vec<Vec<Walked>> = Vec::new();
    let (mut ticks, mut time) = (0_u64, 0.0_f32);
    let mut at = 0;
    while let Some(&byte) = bytes.get(at) {
        if byte == 0xFF {
            break;
        }
        if byte & 0x80 != 0 {
            ticks += u64::from(byte & 0x7F);
            time += f32::from(byte & 0x7F) * 0.001;
            at += 1;
        } else if byte == AIM_RECORDING {
            recordings.push(Vec::new());
            (ticks, time) = (0, 0.0);
            at += 1 + AIM_RECORDING_BYTES;
        } else {
            let (_, length, ..) = AIM_TYPES.iter().find(|(t, ..)| *t == byte)?;
            recordings.last_mut()?.push(Walked {
                raw_type: byte,
                ticks,
                time_f32: time,
            });
            at += 1 + length;
        }
    }
    Some(recordings)
}

/// A raw type's channels in a record: its first field's channel and the `fields` after it.
fn kind_channels<'a>(
    log: &'a FlightLog,
    first: &str,
    fields: usize,
) -> Vec<&'a hpr_flightdata::log::Channel> {
    let Some(start) = log.channels.iter().position(|c| c.column == first) else {
        return Vec::new();
    };
    log.channels[start..].iter().take(fields).collect()
}

fn aim_checks(samples: &[Sample]) -> Vec<AimFlight> {
    let by_id: BTreeMap<String, &Sample> = samples.iter().map(|s| (id_of(&s.bytes), s)).collect();
    let [short_id, xtra_id, dump_id] = AIM_FLIGHT;
    // Samples that hold none of the flight's files have no entry; any one missing fails it.
    if !AIM_FLIGHT.iter().any(|id| by_id.contains_key(*id)) {
        return Vec::new();
    }
    let mut flight = AimFlight {
        ids: AIM_FLIGHT.iter().map(|id| (*id).to_owned()).collect(),
        ..AimFlight::default()
    };
    let (Some(short), Some(xtra), Some(dump)) =
        (by_id.get(short_id), by_id.get(xtra_id), by_id.get(dump_id))
    else {
        return vec![flight];
    };
    flight.short_is_prefix = xtra.bytes.starts_with(&short.bytes);
    let (Ok(short_log), Ok(xtra_log), Ok(recordings), Some(walked)) = (
        entacore_raw::read_xtra(&short.bytes),
        entacore_raw::read_xtra(&xtra.bytes),
        entacore_raw::read_dump(&dump.bytes),
        walk_dump(&dump.bytes),
    ) else {
        return vec![flight];
    };
    let (Some(dump_log), Some(dump_walk)) = (recordings.last(), walked.last()) else {
        return vec![flight];
    };
    flight.short_samples = short_log.stated.samples.unwrap_or(0);
    flight.xtra_samples = xtra_log.stated.samples.unwrap_or(0);
    flight.recordings = recordings.len();
    flight.flight_samples = dump_log.stated.samples.unwrap_or(0);
    for other in &recordings[..recordings.len() - 1] {
        flight.other_samples += other.stated.samples.unwrap_or(0);
        if let Some(pressure) = values(other, "pressure").filter(|p| !p.is_empty()) {
            let least = pressure.iter().copied().fold(f64::INFINITY, f64::min);
            let climb = standard_height_m(least) - standard_height_m(pressure[0]);
            flight.other_climb_m = Some(flight.other_climb_m.map_or(climb, |m| m.max(climb)));
        }
    }
    // Values and times, kind by kind.
    for (raw_type, _, first, fields) in AIM_TYPES {
        let ours = kind_channels(&xtra_log, first, fields);
        let dumped = kind_channels(dump_log, first, fields);
        let shorts = kind_channels(&short_log, first, fields);
        let walk: Vec<&Walked> = dump_walk
            .iter()
            .filter(|w| w.raw_type == raw_type)
            .collect();
        for (index, channel) in ours.iter().enumerate() {
            let n = channel.values.len();
            match dumped.get(index) {
                Some(d) if d.column == channel.column => {
                    flight.values_compared += n;
                    flight.values_differing += channel
                        .values
                        .iter()
                        .zip(&d.values)
                        .filter(|(a, b)| a.to_bits() != b.to_bits())
                        .count()
                        + n.saturating_sub(d.values.len());
                    if index == 0 {
                        flight.dump_only_samples += d.values.len().saturating_sub(n);
                    }
                }
                _ => {
                    flight.values_compared += n;
                    flight.values_differing += n;
                }
            }
            if let Some(s) = shorts.get(index) {
                flight.short_compared += 2 * s.values.len();
                flight.short_differing += s
                    .values
                    .iter()
                    .zip(&channel.values)
                    .chain(s.time_s.iter().zip(&channel.time_s))
                    .filter(|(a, b)| a.to_bits() != b.to_bits())
                    .count()
                    + 2 * s.values.len().saturating_sub(n);
            }
        }
        // A dump channel the `.xtra` lacks fails the flight, every value of it.
        for d in dumped.iter().skip(ours.len()) {
            flight.values_compared += d.values.len();
            flight.values_differing += d.values.len().max(1);
        }
        if let Some(channel) = ours.first() {
            flight.xtra_times_compared += channel.time_s.len();
            flight.xtra_times_differing += channel
                .time_s
                .iter()
                .zip(&walk)
                .filter(|(t, w)| t.to_bits() != f64::from(w.time_f32).to_bits())
                .count()
                + channel.time_s.len().saturating_sub(walk.len());
        }
        if let Some(channel) = dumped.first() {
            flight.dump_times_compared += channel.time_s.len();
            flight.dump_times_differing += channel
                .time_s
                .iter()
                .zip(&walk)
                .filter(|(t, w)| t.to_bits() != (w.ticks as f64 / 1_000.0).to_bits())
                .count()
                + channel.time_s.len().abs_diff(walk.len());
        }
    }
    flight_physics(&mut flight, &xtra_log, dump_log);
    vec![flight]
}

/// A channel of a record by name.
fn channel<'a>(log: &'a FlightLog, name: &str) -> Option<&'a hpr_flightdata::log::Channel> {
    log.channels.iter().find(|c| c.column == name)
}

/// The flight's checks of its units, on the dump's clock.
/// Liftoff on a log's clock: the first sample of the unbroken run of axial readings above
/// [`BOOST_G`] that holds the `state 1` event, or `None` if there is no such event or the reading
/// at it isn't above.
fn liftoff_s(log: &FlightLog) -> Option<f64> {
    let detect = log.events.iter().find(|e| e.name == "state 1")?.time_s;
    let accel = channel(log, "acceleration")?;
    let at = accel.time_s.iter().rposition(|t| *t <= detect)?;
    let boost = |i: usize| accel.values[i] / G0 > BOOST_G;
    if !boost(at) {
        return None;
    }
    let first = (0..=at).rev().take_while(|i| boost(*i)).last()?;
    Some(accel.time_s[first])
}

fn flight_physics(flight: &mut AimFlight, xtra_log: &FlightLog, dump_log: &FlightLog) {
    let Some(liftoff) = liftoff_s(dump_log) else {
        return;
    };
    flight.detect_after_liftoff_s = dump_log
        .events
        .iter()
        .find(|e| e.name == "state 1")
        .map(|e| e.time_s - liftoff);
    let pad_median = |name: &str| -> Option<f64> {
        let axis = channel(dump_log, name)?;
        let mut pad: Vec<f64> = axis
            .time_s
            .iter()
            .zip(&axis.values)
            .filter(|(t, _)| **t < liftoff && **t >= liftoff - PAD_WINDOW_S)
            .map(|(_, v)| v / G0)
            .collect();
        pad.sort_by(f64::total_cmp);
        pad.get(pad.len() / 2).copied()
    };
    flight.pad_axial_g = pad_median("acceleration");
    if let (Some(axial), Some(x), Some(y)) = (
        flight.pad_axial_g,
        pad_median("lat. X accel."),
        pad_median("lat. Y accel."),
    ) {
        flight.pad_g = Some(axial.hypot(x).hypot(y));
    }
    let least = |log: &FlightLog| -> Option<(f64, f64)> {
        let pressure = channel(log, "pressure")?;
        pressure
            .time_s
            .iter()
            .zip(&pressure.values)
            .filter(|(t, _)| **t >= liftoff)
            .map(|(t, p)| (*t, *p))
            .min_by(|a, b| a.1.total_cmp(&b.1))
    };
    if let (Some(pressure), Some((apogee_s, least_pa))) =
        (channel(dump_log, "pressure"), least(dump_log))
    {
        let at_liftoff = pressure
            .time_s
            .iter()
            .zip(&pressure.values)
            .take_while(|(t, _)| **t < liftoff)
            .last()
            .map(|(_, p)| *p);
        if let Some(pad_pa) = at_liftoff {
            let height = standard_height_m(least_pa) - standard_height_m(pad_pa);
            flight.apogee_time_ratio = Some((apogee_s - liftoff) / (2.0 * height / G0).sqrt());
        }
        // The `.xtra`'s climb, from its own liftoff to its least pressure, over the ticks'.
        let xtra_liftoff = liftoff_s(xtra_log);
        let xtra_apogee = channel(xtra_log, "pressure").and_then(|p| {
            p.time_s
                .iter()
                .zip(&p.values)
                .filter(|(_, v)| **v == least_pa)
                .map(|(t, _)| *t)
                .next()
        });
        if let (Some(start), Some(end)) = (xtra_liftoff, xtra_apogee) {
            flight.xtra_climb_ratio = Some((end - start) / (apogee_s - liftoff));
        }
    }
    // The GPS's clock against the ticks, from its first fix to liftoff and from liftoff on.
    if let (Some(tow), Some(week)) = (
        channel(dump_log, "GPS Time of Week"),
        channel(dump_log, "GPS week"),
    ) {
        let fixes: Vec<(f64, f64)> = tow
            .time_s
            .iter()
            .zip(&tow.values)
            .zip(&week.values)
            .filter(|(_, w)| **w > 0.0)
            .map(|((t, ms), _)| (*t, *ms))
            .collect();
        let rate = |part: Vec<&(f64, f64)>| -> Option<f64> {
            let (first, last) = (part.first()?, part.last()?);
            (last.0 > first.0).then(|| (last.1 - first.1) / ((last.0 - first.0) * 1_000.0))
        };
        flight.gps_ms_per_tick = [
            rate(fixes.iter().filter(|(t, _)| *t < liftoff).collect()),
            rate(fixes.iter().filter(|(t, _)| *t >= liftoff).collect()),
        ];
    }
    // The `.xtra`'s clock behind the ticks at its last sample.
    if let (Some(ours), Some(theirs)) = (channel(xtra_log, "Gyro X"), channel(dump_log, "Gyro X")) {
        let last = ours.time_s.len().checked_sub(1);
        flight.xtra_behind_s = last.and_then(|i| Some(theirs.time_s.get(i)? - ours.time_s[i]));
    }
}

pub(super) fn unread(checks: &Checks) -> Vec<String> {
    let mut out = Vec::new();
    for f in &checks.als {
        let within_height = f.height_bound_m.is_some_and(|bound| {
            f.height_off_m
                .iter()
                .all(|off| off.is_some_and(|o| o <= bound))
        });
        // Every record is compared at 15 digits, but the speed from apogee on.
        let every = f.records as u64;
        if f.records == 0
            || f.records != f.rows
            || f.stored_differing > 0
            || f.accel[0] != every
            || f.accel[1] > 0
            || [f.voltage, f.time]
                .iter()
                .any(|c| c[0] != every || c[2] > MAX_UNITS_15)
            || f.speed[0] == 0
            || f.speed[2] > MAX_UNITS_15
            || !f.events_equal
            || !within_height
        {
            out.push(format!(
                "  {} (AltiMAX .ALS): fails its checks against its export {}: {} records, {} rows, \
                 {} of {} stored values differ, at 15 digits acceleration {:?}, voltage {:?}, \
                 time {:?}, speed {:?}, events equal: {}, heights off {:?} m within {:?}",
                f.id,
                f.export,
                f.records,
                f.rows,
                f.stored_differing,
                f.stored_compared,
                f.accel,
                f.voltage,
                f.time,
                f.speed,
                f.events_equal,
                f.height_off_m,
                f.height_bound_m
            ));
        }
    }
    for f in &checks.aim {
        if !f.short_is_prefix
            || f.values_compared == 0
            || f.values_differing > 0
            || f.xtra_times_differing > 0
            || f.dump_times_differing > 0
            || f.short_differing > 0
            || f.short_compared == 0
            || !f
                .apogee_time_ratio
                .is_some_and(|r| APOGEE_TIME_BAND.contains(&r))
            || !f.pad_g.is_some_and(|g| PAD_G_BAND.contains(&g))
        {
            out.push(format!(
                "  {:?} (Entacore AIM): fail their checks: short copy a prefix: {}, {} of {} \
                 values differ, {} of {} .xtra times, {} of {} dump times, {} of {} short-copy \
                 values, time to apogee over √(2h/g) {:?}, pad {:?} g",
                f.ids,
                f.short_is_prefix,
                f.values_differing,
                f.values_compared,
                f.xtra_times_differing,
                f.xtra_times_compared,
                f.dump_times_differing,
                f.dump_times_compared,
                f.short_differing,
                f.short_compared,
                f.apogee_time_ratio,
                f.pad_g
            ));
        }
    }
    out
}

fn figure(value: Option<f64>, digits: usize) -> String {
    value.map_or("–".to_owned(), |v| format!("{v:.digits$}"))
}

fn counts(c: [u64; 3]) -> String {
    format!("{} / {} / {}", c[0], c[1], c[2])
}

/// The report's section for this increment.
pub(super) fn markdown(checks: &Checks) -> String {
    let mut out = String::new();
    for f in &checks.als {
        out += &format!(
            "\nThe AltiMAX `.ALS` (M7.1c23), read by its reader, against the `.xls` export of the \
             same flight beside it, read by the export's: *Records* and *Rows* count each; \
             *Stored* compares the values the file stores that the export writes unchanged, both \
             pressures, the event code and the voltage channel, bit for bit, as compared / \
             differing. The rest are compared at the export's 15 significant digits, as \
             compared / differing / the most, in units of the 15th digit: the acceleration; \
             each voltage channel's counts times the export's factor, {:.11} V a count on \
             channels 1 and 5, {:.11} on 2 to 4, which no document states; the clock, 5 ms or \
             25 ms a record; and the speed before apogee, 0 before `BARO-LIFTOFF` and then the \
             counts summed, {} billionths of a m/s each. A value may differ by \
             {MAX_UNITS_15} unit, the writer's rounding at a tie. *Speed left* counts the rows \
             from apogee on, whose speed the software works out from the pressure: not \
             compared. *Heights off* is the most the standard atmosphere's height of each \
             pressure, less that of the header's zero pressure, differs from the export's, \
             filtered and raw, in m, which must be within half what one pascal moves the zero's \
             height, *Bound*.\n\n\
             | File | Export | Records | Rows | Stored | Acceleration | Voltage | Clock | Speed \
             | Speed left | Events | Heights off, m | Bound, m |\n\
             |---|---|---:|---:|---|---|---|---|---|---:|---|---|---:|\n\
             | `{}` | `{}` | {} | {} | {} / {} | {} | {} | {} | {} | {} | {} {} | {} / {} | {} |\n",
            VOLTS_PER_COUNT[0].1,
            VOLTS_PER_COUNT[1].1,
            SPEED_PER_COUNT_E9,
            f.id,
            f.export,
            f.records,
            f.rows,
            f.stored_compared,
            f.stored_differing,
            counts(f.accel),
            counts(f.voltage),
            counts(f.time),
            counts(f.speed),
            f.speed_not_compared,
            f.events,
            if f.events_equal { "equal" } else { "differ" },
            figure(f.height_off_m[0], 4),
            figure(f.height_off_m[1], 4),
            figure(f.height_bound_m, 4),
        );
    }
    for f in &checks.aim {
        out += &format!(
            "\nThe Entacore AIM's files of one flight (M7.1c23), `{}`, `{}` and `{}`: a short \
             copy of an `.xtra`, the whole `.xtra` its software made of the raw dump, and the \
             dump. *Prefix* is whether the short copy's bytes are the whole one's first. \
             *Samples* counts what each reader read: the short copy, the whole `.xtra`, the \
             dump's recordings and its last, the flight. *Others* counts the other recordings' \
             samples, and *Other climb* the most any rises over its first pressure in the \
             standard atmosphere, m. *Values* compares the whole `.xtra`'s channels with the \
             flight recording's, value for value, bit for bit, as compared / differing; *Dump \
             only* counts the samples the dump holds past them. *`.xtra` times* compares the \
             `.xtra`'s times with the dump's ticks summed here as the software sums them, in \
             32-bit floats a tick byte at a time, and *Dump times* the dump reader's with the \
             ticks in ms, the dump walked here apart from the reader; *Short* compares the short \
             copy's values and times with the whole one's first. On the flight, on the dump's \
             clock, liftoff is the first of the unbroken run of axial readings above {BOOST_G} g \
             that holds `state 1`, the logger's launch detect: *Pad, g* is the magnitude of the \
             three accelerometers' medians over the last {PAD_WINDOW_S} s before liftoff, which \
             must be within {:.2}–{:.2}, an accelerometer's band at rest, and then the axial \
             one's alone; *Detect, s* how long after liftoff `state 1` comes; *Apogee time* the time from liftoff to the least \
             pressure over √(2h/g), h the standard atmosphere's climb from the pad, within \
             {:.1}–{:.1}; *GPS ms a tick* the GPS's time of week over the ticks, on the pad and \
             from liftoff on; *Behind, s* how far the `.xtra`'s \
             clock is behind the ticks at its last sample, and *Climb* its climb's time over the \
             ticks'.\n\n\
             | Prefix | Samples | Others | Other climb, m | Values | Dump only | `.xtra` times | \
             Dump times | Short | Pad, g | Detect, s | Apogee time | GPS ms a tick | Behind, s | Climb |\n\
             |---|---|---:|---:|---|---:|---|---|---|---:|---:|---:|---|---:|---:|\n\
             | {} | {} / {} / {} / {} | {} | {} | {} / {} | {} | {} / {} | {} / {} | {} / {} | {} \
             / {} | {} | {} | {} / {} | {} | {} |\n",
            f.ids.first().map_or("", String::as_str),
            f.ids.get(1).map_or("", String::as_str),
            f.ids.get(2).map_or("", String::as_str),
            PAD_G_BAND.start(),
            PAD_G_BAND.end(),
            APOGEE_TIME_BAND.start(),
            APOGEE_TIME_BAND.end(),
            if f.short_is_prefix { "yes" } else { "no" },
            f.short_samples,
            f.xtra_samples,
            f.recordings,
            f.flight_samples,
            f.other_samples,
            figure(f.other_climb_m, 1),
            f.values_compared,
            f.values_differing,
            f.dump_only_samples,
            f.xtra_times_compared,
            f.xtra_times_differing,
            f.dump_times_compared,
            f.dump_times_differing,
            f.short_compared,
            f.short_differing,
            figure(f.pad_g, 3),
            figure(f.pad_axial_g, 3),
            figure(f.detect_after_liftoff_s, 2),
            figure(f.apogee_time_ratio, 3),
            figure(f.gps_ms_per_tick[0], 4),
            figure(f.gps_ms_per_tick[1], 4),
            figure(f.xtra_behind_s, 2),
            figure(f.xtra_climb_ratio, 4),
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifteen_digits_round_and_count_units() {
        assert_eq!(digits_15(1.0), Some((100_000_000_000_000, -14)));
        assert_eq!(digits_15(0.0), None);
        // 0.1 + 0.2 is 0.30000000000000004, the same at 15 digits.
        assert_eq!(units_15(0.1 + 0.2, 0.3), 0);
        assert_eq!(units_15(1.000_000_000_000_01, 1.0), 1);
        assert_eq!(units_15(1.0, 10.0), u64::MAX);
        let mut c = [0; 3];
        tally(&mut c, 2.0, 2.000_000_000_000_02);
        tally(&mut c, 3.0, 3.0);
        assert_eq!(c, [2, 1, 2]);
    }

    #[test]
    fn the_walk_sums_ticks_and_floats_by_tick_byte() {
        let mut bytes = vec![AIM_RECORDING];
        bytes.extend([0; AIM_RECORDING_BYTES]);
        // 126 + 4 ticks, a pressure; 1 tick, an acceleration; then erased memory.
        bytes.extend([0xFE, 0x84, 0x03, 1, 2, 3, 0x81, 0x02, 0, 0, 0xFF, 0xFF]);
        let walked = walk_dump(&bytes).expect("a dump the walk knows");
        let records = &walked[0];
        assert_eq!(records.len(), 2);
        assert_eq!((records[0].raw_type, records[0].ticks), (0x03, 130));
        assert_eq!(records[1].ticks, 131);
        let step = |ticks: f32| ticks * 0.001;
        assert_eq!(
            records[1].time_f32,
            0.0 + step(126.0) + step(4.0) + step(1.0)
        );
        // A type the survey doesn't know stops it.
        let mut unknown = vec![AIM_RECORDING];
        unknown.extend([0; AIM_RECORDING_BYTES]);
        unknown.push(0x7E);
        assert!(walk_dump(&unknown).is_none());
    }

    /// A log of one axial channel, readings in g every 0.1 s, and a `state 1` event.
    fn boost_log(g: &[f64], detect_s: f64) -> FlightLog {
        let log = entacore_raw::read_dump(&[DUMP_PAD, &[0xFF]].concat())
            .expect("a dump of one pressure")
            .remove(0);
        let mut json = serde_json::to_value(log).expect("a log as JSON");
        let channel = &mut json["channels"][0];
        channel["column"] = "acceleration".into();
        channel["quantity"] = serde_json::to_value(hpr_flightdata::log::Quantity::Acceleration)
            .expect("a quantity as JSON");
        channel["time_s"] = (0..g.len()).map(|i| i as f64 * 0.1).collect();
        channel["values"] = g.iter().map(|v| v * G0).collect();
        json["events"] = serde_json::json!([{ "time_s": detect_s, "name": "state 1" }]);
        serde_json::from_value(json).expect("the log back")
    }

    /// One recording of one pad pressure, 97,700 Pa.
    const DUMP_PAD: &[u8] = &{
        let mut bytes = [0; 1 + AIM_RECORDING_BYTES + 5];
        bytes[0] = AIM_RECORDING;
        let at = 1 + AIM_RECORDING_BYTES;
        bytes[at] = 0x8A;
        bytes[at + 1] = 0x03;
        bytes[at + 2] = 0x01;
        bytes[at + 3] = 0x7D;
        bytes[at + 4] = 0x94;
        bytes
    };

    #[test]
    fn liftoff_is_the_first_reading_of_the_boost_that_holds_the_detect() {
        // At rest, then the boost from 0.4 s, the detect at 0.75 s.
        let log = boost_log(&[1.0, 1.0, 0.98, 1.2, 2.0, 7.0, 7.5, 7.8, 7.9], 0.75);
        assert_eq!(liftoff_s(&log), Some(0.4));
        // A detect whose reading isn't above the boost's is no liftoff.
        assert_eq!(liftoff_s(&boost_log(&[1.0, 1.0, 7.0, 1.0], 0.35)), None);
        // A detect before the first reading, or no detect, either.
        assert_eq!(liftoff_s(&boost_log(&[7.0, 7.0], -0.1)), None);
        let mut none = boost_log(&[7.0, 7.0], 0.1);
        none.events.clear();
        assert_eq!(liftoff_s(&none), None);
    }

    /// Samples that hold none of the files have no entry, and nothing unread.
    #[test]
    fn no_files_no_entries() {
        let none = checks(&[]);
        assert_eq!(none, Checks::default());
        assert!(unread(&none).is_empty());
    }

    fn passing_als() -> AlsFile {
        AlsFile {
            records: 10,
            rows: 10,
            stored_compared: 40,
            accel: [10, 0, 0],
            voltage: [10, 2, 1],
            time: [10, 1, 1],
            speed: [4, 0, 0],
            events_equal: true,
            height_off_m: [Some(0.01), Some(0.02)],
            height_bound_m: Some(0.04),
            ..AlsFile::default()
        }
    }

    fn passing_aim() -> AimFlight {
        AimFlight {
            short_is_prefix: true,
            values_compared: 100,
            short_compared: 50,
            apogee_time_ratio: Some(1.06),
            pad_g: Some(0.965),
            ..AimFlight::default()
        }
    }

    /// Each check fails its file alone, and the bands' edges are inside.
    #[test]
    fn each_check_fails_on_its_own() {
        let als = |f: fn(&mut AlsFile)| {
            let mut file = passing_als();
            f(&mut file);
            unread(&Checks {
                als: vec![file],
                aim: Vec::new(),
            })
            .len()
        };
        assert_eq!(als(|_| {}), 0);
        assert_eq!(als(|f| f.height_off_m[1] = Some(0.04)), 0);
        for fail in [
            (|f| f.records = 0) as fn(&mut AlsFile),
            |f| f.rows = 9,
            |f| f.stored_differing = 1,
            |f| f.accel[1] = 1,
            // A record whose value went uncompared, or more than one unit off.
            |f| f.accel[0] = 9,
            |f| f.voltage[0] = 9,
            |f| f.voltage[2] = 2,
            |f| f.time[0] = 9,
            |f| f.time[2] = 2,
            |f| f.speed = [0, 0, 0],
            |f| f.speed[2] = 2,
            |f| f.events_equal = false,
            |f| f.height_off_m[0] = Some(0.041),
            |f| f.height_off_m[1] = None,
            |f| f.height_bound_m = None,
        ] {
            assert_eq!(als(fail), 1);
        }
        let aim = |f: fn(&mut AimFlight)| {
            let mut flight = passing_aim();
            f(&mut flight);
            unread(&Checks {
                als: Vec::new(),
                aim: vec![flight],
            })
            .len()
        };
        assert_eq!(aim(|_| {}), 0);
        assert_eq!(aim(|f| f.pad_g = Some(*PAD_G_BAND.start())), 0);
        assert_eq!(aim(|f| f.pad_g = Some(*PAD_G_BAND.end())), 0);
        assert_eq!(
            aim(|f| f.apogee_time_ratio = Some(*APOGEE_TIME_BAND.end())),
            0
        );
        for fail in [
            (|f| f.short_is_prefix = false) as fn(&mut AimFlight),
            |f| f.values_compared = 0,
            |f| f.values_differing = 1,
            |f| f.xtra_times_differing = 1,
            |f| f.dump_times_differing = 1,
            |f| f.short_differing = 1,
            |f| f.short_compared = 0,
            |f| f.apogee_time_ratio = None,
            |f| f.apogee_time_ratio = Some(2.01),
            |f| f.apogee_time_ratio = Some(0.49),
            |f| f.pad_g = None,
            // Read in m/s² for g, or at half or twice the sensor's range.
            |f| f.pad_g = Some(9.46),
            |f| f.pad_g = Some(0.48),
            |f| f.pad_g = Some(1.93),
            |f| f.pad_g = Some(0.849),
            |f| f.pad_g = Some(1.151),
        ] {
            assert_eq!(aim(fail), 1);
        }
    }
}
