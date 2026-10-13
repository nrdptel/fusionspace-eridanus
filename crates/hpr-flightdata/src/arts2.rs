//! Ozark Aerospace's ARTS2 flight computer's interpreted data, as its software exports a flight:
//! a `# MET Acc Vel iAlt pAlt` header, then comma-separated rows.
//!
//! The ARTS2 records an accelerometer and a barometer, each read by a 10-bit converter, at a rate
//! its owner sets, until its memory is full (*ARTS2 User Manual*, V1.0.0, Ozark Aerospace, 2007,
//! p9, p16). Its PC software, DataAnalyzer, downloads the flight and works out what it shows:
//! against the mission elapsed time (MET), a barometric altitude, an "accelerometer altitude",
//! the velocity and the acceleration (p11–12). *Flight → Export → Interpreted Data* writes them as
//! a CSV file (p14). The manual describes neither the file's columns nor their units, and its
//! screens show feet, ft/s and ft/s² (p10–11), but the files among the project's sample logs are
//! not all in those units, as below. Each holds 16,385 or 16,115 rows, one every 0.02 s (50 Hz),
//! from launch:
//!
//! ```text
//! # MET Acc Vel iAlt pAlt
//! 0.000,0.00,0.00,0.00,0.00
//! 0.020,61.00,0.61,0.01,0.00
//! ```
//!
//! **Recognition.** The first line that isn't blank is `#` and then the five names `MET`, `Acc`,
//! `Vel`, `iAlt` and `pAlt`, in that order and nothing else, parted by spaces, tabs or commas and
//! compared without regard to case. The ARTS2's raw data (`MET,Acc(HI),Acc/Baro(LO),Baro(HI),State`)
//! and AltOS's CSV, whose header also opens with `#` (`#version,serial,…`), name other columns.
//!
//! **Columns.** Nothing in the file says whether the software was set to English or metric units,
//! and the sample logs hold both, so every unit is decided from the file itself, by the checks
//! below, each of which the reader runs; a note on the record gives each decision and its numbers.
//!
//! | Column | Quantity | Unit, and from where |
//! |---|---|---|
//! | `MET` | the clock: seconds from launch | the name (mission elapsed time) and the apogee check (below), which a clock in another unit fails |
//! | `Acc` | [`Quantity::Acceleration`], along the logger's axis, gravity taken out | standard gravity (× 9.806 65 m/s²) or `Vel`'s unit per second, from the boost check (below) |
//! | `Vel` | [`Quantity::Speed`]: the logger's integral of `Acc` | m/s or ft/s, from the apogee check (below) |
//! | `iAlt` | [`Quantity::InertialAltitude`]: the logger's integral of `Vel` | `Vel`'s length unit, from the inertial check (below) |
//! | `pAlt` | [`Quantity::AltitudeAboveGround`]: the barometric altitude, the primary track | `Vel`'s length unit, from the barometric check (below) |
//!
//! *The apogee check* decides `Vel`'s unit. `Vel` is the logger's integral of `Acc`: over the
//! boost, `Vel` rises by `Acc`'s integral times one factor in every sample file (the boost check).
//! At the apogee the rocket is slowest, so its drag is least, and the accelerometer reads near
//! zero; with gravity taken out, `Acc` then reads −1 g, and `Vel` falls at g in its own unit:
//! 9.81 m/s² or 32.17 ft/s². The reader finds the first sample after `Vel`'s highest where it
//! reaches zero, fits a straight line to `Vel` over the second before that zero, to the first
//! sample at or below it, and takes the unit in which the line's slope is within 25% of −g; m and
//! ft are 3.28 times apart, so at most one unit can be. The second after the zero is left out:
//! there the ejection's jolt can come, as it does in the sample files 1 to 4.5 s after it. As of
//! 2026-10-13, in the 4 interpreted-data files among the project's sample logs, the slope is 1.03
//! g read in m/s in the one and 0.99 to 1.02 g read in ft/s in the three others. A file whose
//! `Vel` never reaches its zero after its highest, or whose slope is within 25% of neither, is
//! refused ([`LogError::Unit`]): the primary track's unit hangs on it.
//!
//! *The boost check* decides `Acc`'s unit. From the first sample to `Vel`'s highest, the reader
//! sums `Vel`'s rise and `Acc`'s integral (trapezoids) over the steps where both are written: the
//! ratio is `Vel`'s unit over `Acc`'s, about 1 when `Acc` is in `Vel`'s unit per second and about
//! g in `Vel`'s unit when `Acc` is in standard gravities. Within 10% of one of them, that unit is
//! taken; otherwise `Acc` is kept as written, [`Quantity::Unrecognized`]. The software's own
//! arithmetic sets the ratio, so only the rounding of the written values moves it: as of
//! 2026-10-13 the sample files' ratios are 1.000 (m/s², beside m/s) and 32.08 to 32.16 (g, beside
//! ft/s), within 0.3% of standard gravity's 32.174 ft/s².
//!
//! *The inertial check* decides `iAlt`'s unit, as the boost check does `Acc`'s: from the first
//! sample to the apogee, `iAlt`'s rise over `Vel`'s integral is 1 when they share a length unit.
//! Within 10% of 1, `iAlt` is read in `Vel`'s; otherwise it is kept as written. The sample files'
//! ratios are 1.000 to 4 digits.
//!
//! *The barometric check* decides `pAlt`'s unit. `pAlt` is the barometer's altitude, not an
//! integral, so it can't match `Vel`'s to the digit; but its highest value must be near the
//! climb `Vel` integrates to by the apogee, which the reader requires within 25% in `Vel`'s
//! length unit. 25% allows for a day's air that isn't the standard atmosphere (a few percent),
//! the integration's drift and the barometer's step, and is narrow enough that m and ft can't
//! both be within it. The sample files' highest `pAlt` is 0.95 to 0.99 times the climb. A climb
//! under 176 m (20 of the barometer's steps, below), where one step would use a fifth of the
//! margin, or a `pAlt` outside the margin, is refused ([`LogError::Unit`]).
//!
//! **The primary track.** [`FlightLog::time_s`] is `MET` and [`FlightLog::altitude_m`] is `pAlt`:
//! the barometer's altitude above the ground. The file starts at launch, `MET` 0, and holds no
//! samples from the pad, so the zero of `pAlt` is the logger's own ground reference, not in the
//! file: the average of 16 pressure samples it takes as it enters flight mode on the pad, its
//! "Zero Altitude baseline" (p17). In every sample file `pAlt` reads 0.00 on the first row; where
//! it reads more than one barometer step (below) off zero there, a note says so. `iAlt` is kept as
//! a channel: an integral of a single-axis accelerometer drifts, and in every sample file it falls
//! to zero, and stays there, a minute or more before the barometer's descent ends, while `Vel`
//! reads −210 to −3,160 by the end of the file. So `Vel` is no vertical speed either:
//! [`FlightLog::vertical_speed_m_s`] is `None`, as for the Blue Raven's inertial velocity.
//!
//! **The closing zeros.** In every sample file the last row, or the last two, write `Vel` as
//! 0.00 where the row before reads −210 to −3,160: they aren't an integral of `Acc`, which moves
//! `Vel` by under 1 a row there. A closing run of exactly 0 in `Vel` or `iAlt` is a gap, with a
//! note, where the value before it lies further from 0 than twice what its rate (`Acc` times the
//! boost check's ratio for `Vel`; `Vel` for `iAlt`), held at its largest over the run, changes it
//! by in the run's time, plus a hundredth for the rounding. In the two files of 16,385 rows the
//! last row writes 0.00 in every column; its `Acc` and `pAlt` are kept as written. The first row's
//! `Acc` is 0.00 in every sample file too, also kept.
//!
//! **The barometer's step.** `pAlt` is written to a hundredth but moves in steps of its
//! barometer's count: near the ground its smallest step is 8.57 m in the metric sample file and
//! 28.12 to 28.89 ft (8.57 to 8.81 m) in the English ones. The format's resolution
//! ([`LogFormat::altitude_resolution_m`]) is the coarsest, 28.89 ft. A count is a fixed step of
//! pressure, so the step is larger aloft and at a higher launch site.
//!
//! **Cells.** Every row must hold five cells, and its `MET` a number greater than the row
//! before's; otherwise the file is refused, naming the line. Any other cell that is empty or isn't
//! a number is a gap (`NaN`), counted in a note. Lines may end in CR, LF or both, blank lines are
//! skipped, and a byte-order mark is dropped. As every row must be five cells wide, the record
//! never holds more values than the file has cells.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{number, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "ARTS2 CSV";

/// The logger's name on the record.
const LOGGER: &str = "Ozark Aerospace ARTS2";

/// The header's names after its `#`, in order.
const NAMES: [&str; 5] = ["MET", "Acc", "Vel", "iAlt", "pAlt"];

/// The coarsest of the sample files' smallest `pAlt` steps near the ground, in feet: 28.89 ft,
/// 8.81 m (module docs, *The barometer's step*).
const COARSEST_STEP_FT: f64 = 28.89;

/// The step between two altitudes the format can tell apart near the ground, m
/// ([`crate::log::LogFormat::altitude_resolution_m`]): 28.89 ft, 8.805 672 m, the coarsest of the
/// smallest steps `pAlt` moves by in the 4 interpreted-data files among the project's sample logs
/// as of 2026-10-13 (8.57 m in the metric one, 28.12 to 28.89 ft in the English ones). `pAlt` is
/// written to a hundredth, but moves by its barometer's count, a fixed step of pressure, so the
/// step is larger aloft and at a higher site.
pub(crate) const ALTITUDE_STEP_M: f64 = COARSEST_STEP_FT * FOOT_M;

/// How long before `Vel`'s zero at the apogee, s, the samples its slope is fitted to start: the
/// coast's last second, before any ejection's jolt.
const APOGEE_WINDOW_S: f64 = 1.0;

/// The fewest samples the apogee's line is fitted to.
const APOGEE_SAMPLES: usize = 3;

/// How far, as a share, the slope of `Vel` through the apogee may be from −g in a unit for `Vel`
/// to be read in it: the drag at the apogee is small, and m and ft are 3.28 times apart, so at
/// most one unit can be within 25%.
const GRAVITY_MARGIN: f64 = 0.25;

/// How far, as a share, a ratio the software's own integration sets (the boost and inertial
/// checks) may be from the one a unit gives: the written values' rounding moves them by at most
/// 0.3% in the sample files.
const INTEGRAL_MARGIN: f64 = 0.10;

/// How far, as a share, `pAlt`'s highest value may be from the climb `Vel` integrates to: a day's
/// air off the standard atmosphere, the integration's drift and the barometer's step; m and ft
/// can't both be within it.
const BAROMETRIC_MARGIN: f64 = 0.25;

/// The least climb, m, from which `pAlt`'s unit is decided: 20 of the barometer's steps, so that
/// one step is 5% of the climb, a fifth of [`BAROMETRIC_MARGIN`].
const SETTLING_CLIMB_M: f64 = 20.0 * ALTITUDE_STEP_M;

/// The most a written value may be off its true one, in its unit: the export writes hundredths.
const ROUNDING: f64 = 0.01;

/// A length unit, of the velocity and the altitudes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Length {
    Meter,
    Foot,
}

impl Length {
    const BOTH: [Self; 2] = [Self::Meter, Self::Foot];

    /// Meters in the unit.
    fn meters(self) -> f64 {
        match self {
            Self::Meter => 1.0,
            Self::Foot => FOOT_M,
        }
    }

    /// Standard gravity in the unit per second squared.
    fn gravity(self) -> f64 {
        STANDARD_GRAVITY_MPS2 / self.meters()
    }

    /// The unit's name.
    fn name(self) -> &'static str {
        match self {
            Self::Meter => "meters",
            Self::Foot => "feet",
        }
    }

    /// The unit's symbol.
    fn symbol(self) -> &'static str {
        match self {
            Self::Meter => "m",
            Self::Foot => "ft",
        }
    }
}

/// The first line of the text that isn't blank, any byte-order mark dropped.
fn first_line(text: &str) -> Option<&str> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    text.split(['\r', '\n'])
        .map(str::trim)
        .find(|line| !line.is_empty())
}

/// Whether a line is the export's header: `#`, then [`NAMES`] and nothing else.
fn is_header(line: &str) -> bool {
    let Some(rest) = line.trim().strip_prefix('#') else {
        return false;
    };
    let mut names = rest.split([' ', '\t', ',']).filter(|name| !name.is_empty());
    NAMES.iter().all(|expected| {
        names
            .next()
            .is_some_and(|name| name.eq_ignore_ascii_case(expected))
    }) && names.next().is_none()
}

/// Whether a file's text is in this layout: its first line that isn't blank is the export's
/// header, `# MET Acc Vel iAlt pAlt`.
pub fn recognizes(text: &str) -> bool {
    first_line(text).is_some_and(is_header)
}

/// The file's rows, as written.
struct Rows {
    /// `MET`, s.
    time_s: Vec<f64>,
    /// `Acc`, `Vel`, `iAlt` and `pAlt` as written, `NaN` for a gap.
    values: [Vec<f64>; 4],
    /// Each row's line in the file.
    lines: Vec<usize>,
    /// Each column's gaps, and the line of the first.
    gaps: [(usize, usize); 4],
}

/// The columns' places in [`Rows::values`].
const ACC: usize = 0;
const VEL: usize = 1;
const IALT: usize = 2;
const PALT: usize = 3;

/// Reads a log in this layout.
///
/// # Errors
///
/// - [`LogError::NotThisFormat`] if the first line that isn't blank isn't the export's header.
/// - [`LogError::NoData`] without a row.
/// - [`LogError::Syntax`] for a row not five cells wide, or whose `MET` isn't a number greater
///   than the row before's.
/// - [`LogError::Unit`] where the file's evidence settles no unit for `Vel` or `pAlt` (module
///   docs): the primary track would have no unit.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    if !is_header(header) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't the ARTS2's interpreted-data header, \
                 `# MET Acc Vel iAlt pAlt`",
                short(header)
            ),
        });
    }
    let rows = rows(lines)?;
    if rows.time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    let units = decide(&rows, header_line)?;
    let mut notes = units.notes.clone();
    let mut values = rows.values.clone();
    closing_zeros(&rows, &units, &mut values, &mut notes);
    for (index, (count, line)) in rows.gaps.iter().enumerate() {
        if *count > 0 {
            notes.push(format!(
                "`{}` has {count} cells that are empty or aren't a number, the first on line \
                 {line}: gaps",
                NAMES[index + 1]
            ));
        }
    }
    let [acc, vel, ialt, palt] = values;
    let length = units.length.meters();
    let altitude_m: Vec<f64> = palt.iter().map(|value| value * length).collect();
    if let Some(first) = altitude_m.first()
        && first.is_finite()
        && first.abs() > ALTITUDE_STEP_M
    {
        notes.push(format!(
            "`pAlt` reads {first:.2} m on the first row, more than one barometer step from zero: \
             the logger's ground isn't where the record starts"
        ));
    }
    let channel = |index: usize, quantity: Quantity, values: Vec<f64>| Channel {
        column: NAMES[index + 1].to_owned(),
        quantity,
        time_s: rows.time_s.clone(),
        values,
    };
    let (acc_quantity, acc_scale) = units
        .acceleration
        .map_or((Quantity::Unrecognized, 1.0), |scale| {
            (Quantity::Acceleration, scale)
        });
    let (ialt_quantity, ialt_scale) = if units.inertial {
        (Quantity::InertialAltitude, length)
    } else {
        (Quantity::Unrecognized, 1.0)
    };
    let channels = vec![
        channel(
            ACC,
            acc_quantity,
            acc.iter().map(|value| value * acc_scale).collect(),
        ),
        channel(
            VEL,
            Quantity::Speed,
            vel.iter().map(|value| value * length).collect(),
        ),
        channel(
            IALT,
            ialt_quantity,
            ialt.iter().map(|value| value * ialt_scale).collect(),
        ),
        channel(PALT, Quantity::AltitudeAboveGround, altitude_m.clone()),
    ];
    Ok(FlightLog {
        format: LogFormat::Arts2Csv,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: rows.time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

/// The data rows below the header.
fn rows<'a>(lines: impl Iterator<Item = (usize, &'a str)>) -> Result<Rows, LogError> {
    let mut rows = Rows {
        time_s: Vec::new(),
        values: Default::default(),
        lines: Vec::new(),
        gaps: [(0, 0); 4],
    };
    for (line, text) in lines {
        let cells: Vec<&str> = text.split(',').collect();
        if cells.len() != NAMES.len() {
            return Err(LogError::Syntax {
                format: FORMAT,
                line,
                message: format!(
                    "a row of {} cells, where the header names {}: {:?}",
                    cells.len(),
                    NAMES.len(),
                    short(text)
                ),
            });
        }
        let Some(time) = cells.first().and_then(|cell| number(cell, false)) else {
            return Err(LogError::Syntax {
                format: FORMAT,
                line,
                message: format!("`MET` isn't a number: {:?}", short(text)),
            });
        };
        if let Some(previous) = rows.time_s.last()
            && time <= *previous
        {
            return Err(LogError::Syntax {
                format: FORMAT,
                line,
                message: format!(
                    "`MET` {time} doesn't come after the row before's, {previous}: the clock must \
                     increase"
                ),
            });
        }
        rows.time_s.push(time);
        rows.lines.push(line);
        for (index, cell) in cells.iter().skip(1).enumerate() {
            let value = number(cell, false).unwrap_or(f64::NAN);
            if value.is_nan() {
                let (count, first) = &mut rows.gaps[index];
                if *count == 0 {
                    *first = line;
                }
                *count += 1;
            }
            rows.values[index].push(value);
        }
    }
    Ok(rows)
}

/// The units the checks decided (module docs), and a note on each.
#[derive(Debug, Clone)]
struct Units {
    /// The length unit of `Vel` and `pAlt`.
    length: Length,
    /// `Acc`'s factor to m/s², `None` if kept as written.
    acceleration: Option<f64>,
    /// Whether `iAlt` is in [`Self::length`]; kept as written if not.
    inertial: bool,
    /// `Vel`'s rise over `Acc`'s integral on the boost (the boost check), if measured.
    boost_ratio: Option<f64>,
    notes: Vec<String>,
}

/// Where `Vel` crosses zero at the apogee.
struct Apogee {
    /// The row of `Vel`'s highest value.
    peak: usize,
    /// The first row after it whose `Vel` is zero or below.
    row: usize,
    /// The time of the zero, on the line between that row and the one before written.
    time_s: f64,
}

/// `Vel`'s highest value and its first zero after it (module docs, *The apogee check*).
fn apogee(time_s: &[f64], vel: &[f64]) -> Option<Apogee> {
    let peak = vel
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .fold(
            None,
            |best: Option<(usize, f64)>, (row, value)| match best {
                Some((_, top)) if top >= *value => best,
                _ => Some((row, *value)),
            },
        )
        .filter(|(_, top)| *top > 0.0)?
        .0;
    let mut before = (peak, *vel.get(peak)?);
    for (row, value) in vel.iter().enumerate().skip(peak + 1) {
        if !value.is_finite() {
            continue;
        }
        if *value <= 0.0 {
            let (t0, t1) = (*time_s.get(before.0)?, *time_s.get(row)?);
            let share = before.1 / (before.1 - value);
            return Some(Apogee {
                peak,
                row,
                time_s: t0 + share * (t1 - t0),
            });
        }
        before = (row, *value);
    }
    None
}

/// The least-squares slope of `Vel` over its samples from [`APOGEE_WINDOW_S`] before its zero,
/// but not before its highest, to the first at or below the zero; and how many there are.
fn apogee_slope(time_s: &[f64], vel: &[f64], apogee: &Apogee) -> Option<(f64, usize)> {
    let points: Vec<(f64, f64)> = time_s
        .iter()
        .zip(vel)
        .take(apogee.row + 1)
        .skip(apogee.peak)
        .filter(|(time, value)| value.is_finite() && **time >= apogee.time_s - APOGEE_WINDOW_S)
        .map(|(time, value)| (*time, *value))
        .collect();
    if points.len() < APOGEE_SAMPLES {
        return None;
    }
    let count = points.len() as f64;
    let mean_t = points.iter().map(|p| p.0).sum::<f64>() / count;
    let mean_v = points.iter().map(|p| p.1).sum::<f64>() / count;
    let spread: f64 = points.iter().map(|p| (p.0 - mean_t).powi(2)).sum();
    let slope = points
        .iter()
        .map(|p| (p.0 - mean_t) * (p.1 - mean_v))
        .sum::<f64>()
        / spread;
    slope.is_finite().then_some((slope, points.len()))
}

/// Over the steps from the first row to `end` where both columns are written, the rise of `of`
/// and the trapezoid integral of `rate`.
fn rise_and_integral(time_s: &[f64], of: &[f64], rate: &[f64], end: usize) -> (f64, f64) {
    let mut rise = 0.0;
    let mut integral = 0.0;
    for row in 1..=end.min(time_s.len().saturating_sub(1)) {
        let (Some(&[t0, t1]), Some(&[a0, a1]), Some(&[r0, r1])) = (
            time_s
                .get(row - 1..=row)
                .and_then(|s| <&[f64; 2]>::try_from(s).ok()),
            of.get(row - 1..=row)
                .and_then(|s| <&[f64; 2]>::try_from(s).ok()),
            rate.get(row - 1..=row)
                .and_then(|s| <&[f64; 2]>::try_from(s).ok()),
        ) else {
            continue;
        };
        if [a0, a1, r0, r1].iter().all(|value| value.is_finite()) {
            rise += a1 - a0;
            integral += 0.5 * (r0 + r1) * (t1 - t0);
        }
    }
    (rise, integral)
}

/// Whether `value` is within `margin`, as a share, of `target`.
fn near(value: f64, target: f64, margin: f64) -> bool {
    value.is_finite() && (value - target).abs() <= margin * target.abs()
}

/// Decides each column's unit from the file (module docs, *Columns*).
fn decide(rows: &Rows, header_line: usize) -> Result<Units, LogError> {
    let [acc, vel, ialt, palt] = &rows.values;
    let time_s = &rows.time_s;
    let line_of = |row: usize| rows.lines.get(row).copied().unwrap_or(header_line);
    let refuse = |line: usize, message: String| LogError::Unit {
        format: FORMAT,
        line,
        message,
    };

    // The apogee check: `Vel`'s unit.
    let Some(apogee) = apogee(time_s, vel) else {
        return Err(refuse(
            header_line,
            "`Vel` never comes back to zero after its highest value: without the apogee, no \
             unit for it, nor for `pAlt`, can be decided"
                .to_owned(),
        ));
    };
    let apogee_line = line_of(apogee.row);
    let Some((slope, points)) = apogee_slope(time_s, vel, &apogee) else {
        return Err(refuse(
            apogee_line,
            format!(
                "`Vel` has fewer than {APOGEE_SAMPLES} samples in the {APOGEE_WINDOW_S} s to its \
                 zero at {:.2} s: its slope at the apogee, which decides its unit, can't be fitted",
                apogee.time_s
            ),
        ));
    };
    let fits: Vec<Length> = Length::BOTH
        .into_iter()
        .filter(|unit| near(-slope, unit.gravity(), GRAVITY_MARGIN))
        .collect();
    let [length] = fits[..] else {
        return Err(refuse(
            apogee_line,
            format!(
                "into its zero at {:.2} s, `Vel` changes by {slope:.3} a second, within {:.0}% \
                 of standard gravity in neither m/s² (−9.807) nor ft/s² (−32.174): no unit for it, \
                 nor for `pAlt`, can be decided",
                apogee.time_s,
                GRAVITY_MARGIN * 100.0
            ),
        ));
    };
    let mut notes = vec![format!(
        "`Vel` read in {} per second, from the apogee: into its zero at {:.2} s, the line \
         through its {points} samples of the {APOGEE_WINDOW_S} s before falls at {:.3} a second, {:.3} \
         of standard gravity's {:.3} {}/s², within {:.0}%",
        length.name(),
        apogee.time_s,
        -slope,
        -slope / length.gravity(),
        length.gravity(),
        length.symbol(),
        GRAVITY_MARGIN * 100.0
    )];

    // The barometric check: `pAlt`'s unit.
    let (_, climb) = rise_and_integral(time_s, vel, vel, apogee.row);
    let climb_m = climb * length.meters();
    // A climb that isn't a number fails too.
    if !climb_m.is_finite() || climb_m < SETTLING_CLIMB_M {
        return Err(refuse(
            apogee_line,
            format!(
                "`Vel` integrates to a climb of {climb_m:.1} m by the apogee, under the \
                 {SETTLING_CLIMB_M:.0} m from which the barometer's step lets `pAlt`'s unit be \
                 decided"
            ),
        ));
    }
    let highest = palt
        .iter()
        .filter(|value| value.is_finite())
        .fold(f64::NEG_INFINITY, |top, value| top.max(*value));
    let share = highest / climb;
    if !near(share, 1.0, BAROMETRIC_MARGIN) {
        return Err(refuse(
            apogee_line,
            format!(
                "`pAlt`'s highest value, {highest:.2}, is {share:.3} times the climb `Vel` \
                 integrates to by the apogee, {climb:.2} {}, not within {:.0}%: no unit for the \
                 barometric altitude can be decided",
                length.symbol(),
                BAROMETRIC_MARGIN * 100.0
            ),
        ));
    }
    notes.push(format!(
        "`pAlt` read in {}, from `Vel`: its highest value, {highest:.2}, is {share:.3} times the \
         climb `Vel` integrates to by the apogee, {climb:.2} {}, within {:.0}%",
        length.name(),
        length.symbol(),
        BAROMETRIC_MARGIN * 100.0
    ));

    // The boost check: `Acc`'s unit.
    let (rise, integral) = rise_and_integral(time_s, vel, acc, apogee.peak);
    let ratio = rise / integral;
    let boost_ratio = ratio.is_finite().then_some(ratio);
    let acceleration = if near(ratio, 1.0, INTEGRAL_MARGIN) {
        notes.push(format!(
            "`Acc` read in {} per second squared, from `Vel`: over the boost, `Vel` rises \
             {ratio:.3} times `Acc`'s integral, within {:.0}% of 1",
            length.name(),
            INTEGRAL_MARGIN * 100.0
        ));
        Some(length.meters())
    } else if near(ratio, length.gravity(), INTEGRAL_MARGIN) {
        notes.push(format!(
            "`Acc` read in standard gravities, 9.806 65 m/s², from `Vel`: over the boost, `Vel` \
             rises {ratio:.3} times `Acc`'s integral, within {:.0}% of standard gravity's {:.3} \
             {}/s²",
            INTEGRAL_MARGIN * 100.0,
            length.gravity(),
            length.symbol()
        ));
        Some(STANDARD_GRAVITY_MPS2)
    } else {
        notes.push(format!(
            "`Acc` kept as written: over the boost, `Vel` rises {ratio:.3} times `Acc`'s \
             integral, within {:.0}% of neither 1 nor standard gravity in {} per second squared",
            INTEGRAL_MARGIN * 100.0,
            length.name()
        ));
        None
    };

    // The inertial check: `iAlt`'s unit.
    let (rise, integral) = rise_and_integral(time_s, ialt, vel, apogee.row);
    let ratio = rise / integral;
    let inertial = near(ratio, 1.0, INTEGRAL_MARGIN);
    notes.push(if inertial {
        format!(
            "`iAlt` read in {}, from `Vel`: to the apogee it rises {ratio:.4} times `Vel`'s \
             integral, within {:.0}% of 1",
            length.name(),
            INTEGRAL_MARGIN * 100.0
        )
    } else {
        format!(
            "`iAlt` kept as written: to the apogee it rises {ratio:.4} times `Vel`'s integral, \
             not within {:.0}% of 1",
            INTEGRAL_MARGIN * 100.0
        )
    });
    Ok(Units {
        length,
        acceleration,
        inertial,
        boost_ratio,
        notes,
    })
}

/// Marks the closing run of `Vel` and `iAlt` zeros gaps where it is a placeholder (module docs,
/// *The closing zeros*).
fn closing_zeros(rows: &Rows, units: &Units, values: &mut [Vec<f64>; 4], notes: &mut Vec<String>) {
    let count = rows.time_s.len();
    let at = |column: usize, row: usize| rows.values[column].get(row).copied().unwrap_or(f64::NAN);
    for (column, rate, scale) in [
        (VEL, ACC, units.boost_ratio.map(f64::abs)),
        (IALT, VEL, Some(1.0)),
    ] {
        let run = (0..count)
            .rev()
            .take_while(|row| at(column, *row) == 0.0)
            .count();
        let Some(before) = count.checked_sub(run + 1) else {
            continue;
        };
        let (Some(scale), Some(last), Some(&start)) =
            (scale, count.checked_sub(1), rows.time_s.get(before))
        else {
            continue;
        };
        let span = rows.time_s.get(last).map_or(f64::NAN, |end| end - start);
        // The most the rate can move the column over the run, its largest written value held
        // throughout.
        let fastest = (before..count)
            .map(|row| at(rate, row).abs())
            .filter(|value| value.is_finite())
            .fold(0.0, f64::max);
        let reach = fastest * scale * span;
        let previous = at(column, before);
        if run > 0
            && previous.is_finite()
            && reach.is_finite()
            && previous.abs() > 2.0 * reach + ROUNDING
        {
            for row in before + 1..count {
                if let Some(cell) = values[column].get_mut(row) {
                    *cell = f64::NAN;
                }
            }
            notes.push(format!(
                "the last {run} rows, from line {}, write `{}` as 0, where the row before reads \
                 {previous:.2}: more than twice what `{}` can change it by in {span:.2} s, so they \
                 aren't its integral, and are gaps",
                rows.lines.get(before + 1).copied().unwrap_or_default(),
                NAMES[column + 1],
                NAMES[rate + 1],
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An invented flight as the export writes it, a row every 0.1 s: a boost of `boost_s` at
    /// `boost_m_s2` net, a coast at −g less a drag of 0.0004 v² a meter, and 5 s of descent with
    /// the accelerometer reading −0.3 m/s²; `Vel` and `iAlt` the trapezoid integrals of the
    /// written `Acc` and `Vel`, `pAlt` 0.97 of the climb then falling at 20 m/s, in steps of 9 m.
    /// `Vel`, `iAlt` and `pAlt` are written in units of `length_m` meters and `Acc` in units of
    /// `acc_m_s2`.
    fn flight(boost_s: f64, boost_m_s2: f64, length_m: f64, acc_m_s2: f64) -> String {
        let dt = 0.1;
        let mut text = String::from("# MET Acc Vel iAlt pAlt \n");
        let (mut vel, mut ialt, mut previous) = (0.0_f64, 0.0_f64, None::<f64>);
        let mut apogee: Option<(f64, f64)> = None;
        for row in 0..2_000 {
            let time = f64::from(row) * dt;
            let acc_si = if row == 0 {
                0.0
            } else if time <= boost_s + 1e-9 {
                boost_m_s2
            } else if apogee.is_none() {
                -STANDARD_GRAVITY_MPS2 - 0.0004 * vel * vel.abs()
            } else {
                -0.3
            };
            let acc = (acc_si / acc_m_s2 * 100.0).round() / 100.0;
            if let Some(before) = previous {
                let next = vel + 0.5 * (before + acc) * acc_m_s2 * dt;
                ialt = (ialt + 0.5 * (vel + next) * dt).max(0.0);
                if apogee.is_none() && vel > 0.0 && next <= 0.0 {
                    apogee = Some((time, 0.97 * ialt));
                }
                vel = next;
            }
            previous = Some(acc);
            let baro = apogee.map_or(0.97 * ialt, |(at, top)| (top - 20.0 * (time - at)).max(0.0));
            let palt = (baro / 9.0).round() * 9.0;
            text.push_str(&format!(
                "{time:.3},{acc:.2},{:.2},{:.2},{:.2}\n",
                vel / length_m,
                ialt / length_m,
                palt / length_m
            ));
            if apogee.is_some_and(|(at, _)| time > at + 5.0) {
                break;
            }
        }
        text
    }

    /// The metric flight: 2 s at 70 m/s², about 830 m up.
    fn metric() -> String {
        flight(2.0, 70.0, 1.0, 1.0)
    }

    /// The same flight as the English files write it: feet, ft/s, and `Acc` in g.
    fn english() -> String {
        flight(2.0, 70.0, FOOT_M, STANDARD_GRAVITY_MPS2)
    }

    /// The text with one column's values, past the header, multiplied by `factor`.
    fn scaled(text: &str, column: usize, factor: f64) -> String {
        text.lines()
            .map(|line| {
                if line.starts_with('#') {
                    return format!("{line}\n");
                }
                let cells: Vec<String> = line
                    .split(',')
                    .enumerate()
                    .map(|(index, cell)| {
                        if index == column + 1 {
                            format!("{:.2}", cell.parse::<f64>().unwrap() * factor)
                        } else {
                            cell.to_owned()
                        }
                    })
                    .collect();
                format!("{}\n", cells.join(","))
            })
            .collect()
    }

    fn highest(values: &[f64]) -> f64 {
        values
            .iter()
            .filter(|value| value.is_finite())
            .fold(f64::NEG_INFINITY, |top, value| top.max(*value))
    }

    fn unit_message(result: Result<FlightLog, LogError>) -> String {
        match result {
            Err(LogError::Unit { message, .. }) => message,
            other => panic!("expected a unit error, got {other:?}"),
        }
    }

    #[test]
    fn the_header_is_recognized_and_others_are_not() {
        assert!(recognizes(
            "# MET Acc Vel iAlt pAlt \n0.000,0.00,0.00,0.00,0.00\n"
        ));
        assert!(recognizes("\u{feff}\r\n#MET,Acc,Vel,iAlt,pAlt\r\n"));
        assert!(recognizes("# met acc vel ialt palt\n"));
        for other in [
            "MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n0.000,512,4,777,64\n",
            "#version,serial,flight,call,time,clock,rssi,lqi\n",
            "# Comments\n# MET Acc Vel iAlt pAlt\n",
            "# MET Acc Vel iAlt\n",
            "# MET Acc Vel iAlt pAlt State\n",
            "# MET Acc Vel pAlt iAlt\n",
            "MET Acc Vel iAlt pAlt\n",
            "",
        ] {
            assert!(!recognizes(other), "{other:?}");
        }
    }

    /// The two unit systems of the sample files, each read from its own evidence, and the same
    /// flight in each: reading feet as meters, or g as m/s², would put the two 3.28 or 9.81
    /// times apart.
    #[test]
    fn metric_and_english_files_read_to_the_same_flight() {
        let metric = read(&metric()).unwrap();
        let english = read(&english()).unwrap();
        let top_m = highest(&metric.altitude_m);
        let top_e = highest(&english.altitude_m);
        assert!(top_m > 700.0 && top_m < 1_000.0, "{top_m}");
        // `pAlt` is written to a hundredth of a foot, 0.003 m.
        assert!((top_m - top_e).abs() < 0.01, "{top_m} m against {top_e} m");
        // `Acc` written to a hundredth of a g is within 0.05 m/s² of its value; `Vel` and `iAlt`
        // integrate the written `Acc`, so they drift apart by a little more, and `pAlt`, worked
        // out from `iAlt`, can round to the next of its 9 m steps.
        for (column, allowed) in [(0, 0.06), (1, 3.0), (2, 3.0), (3, 9.01)] {
            let (a, b) = (&metric.channels[column], &english.channels[column]);
            assert_eq!(a.quantity, b.quantity, "{}", a.column);
            let worst = a
                .values
                .iter()
                .zip(&b.values)
                .filter(|(x, y)| x.is_finite() && y.is_finite())
                .map(|(x, y)| (x - y).abs())
                .fold(0.0, f64::max);
            assert!(worst < allowed, "{}: {worst}", a.column);
        }
        let quantities: Vec<Quantity> = metric.channels.iter().map(|c| c.quantity).collect();
        assert_eq!(
            quantities,
            [
                Quantity::Acceleration,
                Quantity::Speed,
                Quantity::InertialAltitude,
                Quantity::AltitudeAboveGround
            ]
        );
        assert!(metric.notes[0].starts_with("`Vel` read in meters per second, from the apogee: "));
        assert!(english.notes[0].starts_with("`Vel` read in feet per second, from the apogee: "));
        assert!(english.notes[2].starts_with("`Acc` read in standard gravities, "));
        assert!(metric.notes[2].starts_with("`Acc` read in meters per second squared, "));
        assert!(metric.vertical_speed_m_s.is_none());
    }

    /// A velocity whose slope at the apogee is g in neither unit settles no unit for the primary
    /// track: the file is refused, not read in a guessed unit.
    #[test]
    fn a_velocity_that_falls_at_no_gravity_is_refused() {
        for factor in [2.0, 0.5, 1.6 / FOOT_M] {
            let message = unit_message(read(&scaled(&metric(), VEL, factor)));
            assert!(
                message.contains("within 25% of standard gravity in neither"),
                "{factor}: {message}"
            );
        }
    }

    /// The barometric altitude in feet beside a velocity in m/s, or in meters beside one in ft/s:
    /// the units disagree, and the file is refused.
    #[test]
    fn a_barometric_altitude_in_another_unit_is_refused() {
        let message = unit_message(read(&scaled(&metric(), PALT, 1.0 / FOOT_M)));
        assert!(message.starts_with("`pAlt`'s highest value, "), "{message}");
        let message = unit_message(read(&scaled(&english(), PALT, FOOT_M)));
        assert!(message.starts_with("`pAlt`'s highest value, "), "{message}");
    }

    /// `Acc` and `iAlt` that no check bears out are kept as written, the file still read.
    #[test]
    fn columns_no_check_bears_out_are_kept_as_written() {
        let log = read(&scaled(&metric(), ACC, 3.0)).unwrap();
        assert_eq!(log.channels[0].quantity, Quantity::Unrecognized);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("`Acc` kept as written: "))
        );
        let log = read(&scaled(&metric(), IALT, 1.0 / FOOT_M)).unwrap();
        assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("`iAlt` kept as written: "))
        );
        // `Acc` in g beside m/s, a pairing no sample file has, is read so from its own check.
        let log = read(&scaled(&metric(), ACC, 1.0 / STANDARD_GRAVITY_MPS2)).unwrap();
        assert_eq!(log.channels[0].quantity, Quantity::Acceleration);
        assert!(log.notes[2].starts_with("`Acc` read in standard gravities"));
    }

    /// The edges of the apogee check, each side of 25% of g: a slope of 1.22 g reads, 1.28 g
    /// doesn't; 0.78 g reads, 0.72 g doesn't. Every column is scaled together, so the other
    /// checks hold and only the slope moves.
    #[test]
    fn the_apogee_margin_is_25_percent() {
        for (factor, reads) in [(1.22, true), (1.28, false), (0.78, true), (0.72, false)] {
            let text = [ACC, VEL, IALT, PALT]
                .into_iter()
                .fold(metric(), |text, column| scaled(&text, column, factor));
            let result = read(&text);
            if reads {
                let log = result.unwrap();
                assert!(log.notes[0].starts_with("`Vel` read in meters per second"));
            } else {
                assert!(matches!(result, Err(LogError::Unit { .. })), "{factor}");
            }
        }
    }

    /// A flight too low for the barometer's step to settle `pAlt`'s unit, and a file cut before
    /// its apogee, are refused.
    #[test]
    fn low_and_unfinished_flights_are_refused() {
        let message = unit_message(read(&flight(0.3, 30.0, 1.0, 1.0)));
        assert!(message.contains("under the 176 m"), "{message}");
        let cut: String = metric()
            .lines()
            .take(60)
            .map(|l| format!("{l}\n"))
            .collect();
        let message = unit_message(read(&cut));
        assert!(
            message.starts_with("`Vel` never comes back to zero"),
            "{message}"
        );
        let message = unit_message(read("# MET Acc Vel iAlt pAlt\n0,,,,\n0.02,,,,\n"));
        assert!(
            message.starts_with("`Vel` never comes back to zero"),
            "{message}"
        );
    }

    /// The closing zeros of `Vel` are gaps; a zero `Vel` its rate could reach is kept.
    #[test]
    fn closing_zeros_beyond_reach_are_gaps() {
        let mut text = metric();
        let last = text.trim_end().rsplit_once('\n').unwrap().1.to_owned();
        let cells: Vec<&str> = last.split(',').collect();
        let time: f64 = cells[0].parse().unwrap();
        text.push_str(&format!(
            "{:.3},{},0.00,{},{}\n",
            time + 0.1,
            cells[1],
            cells[3],
            cells[4]
        ));
        text.push_str(&format!("{:.3},0.00,0.00,0.00,0.00\n", time + 0.2));
        let log = read(&text).unwrap();
        let vel = &log.channels[1].values;
        assert!(vel[vel.len() - 2..].iter().all(|v| v.is_nan()));
        assert!(vel[vel.len() - 3].is_finite());
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("the last 2 rows, from line "))
        );
        // `pAlt` and `Acc` keep their zeros.
        assert_eq!(log.altitude_m.last(), Some(&0.0));
        assert_eq!(log.channels[0].values.last(), Some(&0.0));
        // A small velocity coming to zero in one step of a large `Acc` is no placeholder.
        let text = "# MET Acc Vel iAlt pAlt\n0,0,0,0,0\n0.02,50,0.5,0,0\n0.04,-50,0,0.01,0\n";
        let rows = rows(numbered_lines(text).skip(1)).unwrap();
        let units = Units {
            length: Length::Meter,
            acceleration: Some(1.0),
            inertial: true,
            boost_ratio: Some(1.0),
            notes: Vec::new(),
        };
        let mut values = rows.values.clone();
        let mut notes = Vec::new();
        closing_zeros(&rows, &units, &mut values, &mut notes);
        assert_eq!(values[VEL][2], 0.0);
        assert!(notes.is_empty(), "{notes:?}");
    }

    /// Rows not five cells wide, and clocks that don't increase, are refused by line; other cells
    /// that aren't numbers are gaps.
    #[test]
    fn rows_are_checked_and_bad_cells_are_gaps() {
        let header = "# MET Acc Vel iAlt pAlt\n";
        for (body, line) in [
            ("0,0,0,0,0\n0.1,0,0,0\n", 3),
            ("0,0,0,0,0\n0,0,0,0,0\n", 3),
            ("0,0,0,0,0\n,0,0,0,0\n", 3),
            ("0,0,0,0,0,\n", 2),
        ] {
            match read(&format!("{header}{body}")) {
                Err(LogError::Syntax { line: at, .. }) => assert_eq!(at, line, "{body:?}"),
                other => panic!("{body:?}: {other:?}"),
            }
        }
        assert_eq!(read(header), Err(LogError::NoData { format: FORMAT }));
        let text = metric().replacen("\n1.000,", "\n1.000,x", 1);
        let log = read(&text).unwrap();
        assert!(log.channels[0].values[10].is_nan());
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("`Acc` has 1 cells that are empty or aren't a number"))
        );
    }

    /// A barometric altitude more than a step off zero on the first row gets a note.
    #[test]
    fn a_ground_off_zero_is_noted() {
        let text = metric().replacen(
            "\n0.000,0.00,0.00,0.00,0.00\n",
            "\n0.000,0.00,0.00,0.00,27.00\n",
            1,
        );
        let log = read(&text).unwrap();
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("`pAlt` reads 27.00 m on the first row"))
        );
        let log = read(&metric()).unwrap();
        assert!(!log.notes.iter().any(|n| n.contains("first row")));
    }

    #[test]
    fn the_step_is_the_coarsest_of_the_sample_files() {
        assert!((ALTITUDE_STEP_M - 8.805_672).abs() < 1e-9);
    }
}
