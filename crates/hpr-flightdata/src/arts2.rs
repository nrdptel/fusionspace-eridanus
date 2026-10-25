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
//! not all in those units, as below. Each holds about 16,000 rows, the logger's memory, one every
//! 0.02 s (50 Hz), from launch:
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
//! | `Vel` | [`Quantity::Speed`]: the logger's integral of `Acc` | m/s or ft/s, from the apogee check and the step check (below), which must agree |
//! | `iAlt` | [`Quantity::InertialAltitude`]: the logger's integral of `Vel` | `Vel`'s length unit, from the inertial check (below) |
//! | `pAlt` | [`Quantity::AltitudeAboveGround`]: the barometric altitude, the primary track | `Vel`'s length unit, from the step check and the barometric check (below) |
//!
//! The length unit, of `Vel` and `pAlt`, rests on two pieces of evidence that don't depend on each
//! other: how fast `Vel` falls at the apogee, and how far `pAlt` moves for one count of the
//! barometer. A file is read only where both give the same unit.
//!
//! *The apogee check* decides `Vel`'s unit, and with it that `MET` is in seconds. `Vel` is the
//! logger's integral of `Acc`: over the boost, `Vel` rises by `Acc`'s integral times one factor in
//! every sample file (the boost check). At the apogee the rocket is slowest, so its drag is least,
//! and the accelerometer reads near zero; with gravity taken out, `Acc` then reads −1 g, and `Vel`
//! falls at g in its own unit: 9.81 m/s² or 32.17 ft/s². The reader finds the first sample after
//! `Vel`'s highest where it reaches zero, fits a straight line to `Vel` over the second of coast
//! before that zero, to the first sample at or below it, and takes the unit in which the line's
//! slope is within 25% of −g; m and ft are 3.28 times apart, so at most one unit can be. A clock
//! in milliseconds or minutes would put the slope 1,000 or 60 times off, so the same fit pins
//! `MET` in seconds. The second after the zero is left out: there the ejection's jolt can come, as
//! it does in the sample files 1 to 4.5 s after it.
//!
//! A parachute out before the zero adds its drag to the second before it: one out at 25 m/s on the
//! way up can make `Vel` fall at 3.7 g in m/s, which is 1.13 g in ft/s, within 25% of g in the
//! wrong unit. So the window is the *coast's* last second: where `Acc` jumps in the 2 s before the
//! zero, the line is fitted to the second before the first jump instead, and a jump earlier in
//! that second starts it after the jump. A row is a jump where it and the row after it are both
//! further from the median of the 3 rows before than half that median, about half a g near the
//! apogee, where `Acc` reads about −1 g in its unit; one row's spike is no jump, and the sample
//! files hold single-row spikes of a few g. A canopy out more than 2 s before the zero can make
//! `Vel` fall at no more than 1.28 g over its last second, so it can't pass for the other unit: a
//! canopy whose drag grows as the speed squared stops a rising rocket within `(v_t / g) π/2`, `v_t`
//! its descent rate, so taking over 2 s means `v_t` over `4g/π`, 12.49 m/s, and then the last
//! second's fall, `v_t tan(g · 1 s / v_t)` a second, is at most 1.28 g. As of 2026-10-13, no jump
//! ends the window in the 4 interpreted-data files among the project's sample logs, and every
//! one's slope is within 4% of g in the unit its step gives. A file whose `Vel` never reaches its
//! zero after its highest, whose window holds fewer than 3 samples, or whose slope is within 25%
//! of neither unit, is refused ([`LogError::Unit`]).
//!
//! *The step check* decides the length unit apart from `Vel`. `pAlt` is written to a hundredth but
//! moves by its barometer's count, a fixed step of pressure. The ARTS2 reads a Motorola MPXA4100A
//! sensor with a 10-bit converter (manual, p16); the sensor's output is its supply voltage times
//! `0.010 59 P − 0.1518`, `P` in kPa (*MPXAZ4100A Series* data sheet, Freescale, Rev 1, 2005, p4),
//! so a count, 1/1,024 of the supply, is `1 / (1,024 × 0.010 59)` kPa, 92.2 Pa, where the
//! converter is read against the sensor's supply (the manual doesn't say). Near the ground that is
//! `Δh = Δp / (ρ g)`: the reader takes the count as 92.2 Pa ± 15% and the ground's air from sea
//! level at 5% over standard pressure and −20 °C, 1.46 kg/m³, to a 3,000 m site at 5% under its
//! standard pressure (U.S. Standard Atmosphere, 1976) and 45 °C, 0.73 kg/m³, so a count is 5.46
//! to 14.83 m: written as 5.46 to 14.83 in meters, 17.91 to 48.64 in feet. The two bands don't
//! meet, so a step falls in one unit's at most. The step is `pAlt`'s smallest change between two
//! written values in a row, gaps passed over, that at least 3 changes come within 0.02 of: a
//! single smaller change, as a closing row of zeros makes, isn't a count. As of 2026-10-13, every
//! sample file's step is between 8.5 and 9 m, in the band of the unit its slope gives. A `pAlt`
//! with no step written 3 times, a step in neither band, or a step whose unit isn't the slope's,
//! is refused ([`LogError::Unit`]). The step is larger aloft, where the air is thinner, so a
//! record whose single counts all lie high above a high site can fall outside the band, and is
//! refused.
//!
//! *The boost check* decides `Acc`'s unit. From the first sample to `Vel`'s highest, the reader
//! sums `Vel`'s rise and `Acc`'s integral (trapezoids) over the steps where both are written: the
//! ratio is `Vel`'s unit over `Acc`'s, about 1 when `Acc` is in `Vel`'s unit per second and about
//! g in `Vel`'s unit when `Acc` is in standard gravities. Within 10% of one of them, that unit is
//! taken; otherwise `Acc` is kept as written, [`Quantity::Unrecognized`]. The software's own
//! arithmetic sets the ratio, so only the rounding of the written values moves it: as of
//! 2026-10-13 every sample file's ratio is within 0.3% of the one its unit gives.
//!
//! *The inertial check* decides `iAlt`'s unit, as the boost check does `Acc`'s: from the first
//! sample to the apogee, `iAlt`'s rise over `Vel`'s integral is 1 when they share a length unit.
//! Within 10% of 1, `iAlt` is read in `Vel`'s; otherwise it is kept as written. The sample files'
//! ratios are 1.000 to 4 digits.
//!
//! *The barometric check* holds `pAlt` to `Vel`. `pAlt` is the barometer's altitude, not an
//! integral, so it can't match `Vel`'s to the digit; but its highest value must be near the
//! climb `Vel` integrates to by the apogee, which the reader requires within 25% in the length
//! unit. 25% allows for a day's air that isn't the standard atmosphere (a few percent), the
//! integration's drift and the barometer's step, and is narrow enough that m and ft can't both be
//! within it. The sample files' highest `pAlt` is 0.95 to 0.99 times the climb. A climb under 176
//! m (20 of the barometer's steps, below), where one step would use a fifth of the margin, or a
//! `pAlt` outside the margin, is refused ([`LogError::Unit`]).
//!
//! **The primary track.** [`FlightLog::time_s`] is `MET` and [`FlightLog::altitude_m`] is `pAlt`:
//! the barometer's altitude above the ground. The file starts at launch, `MET` 0, and holds no
//! samples from the pad, so the zero of `pAlt` is the logger's own ground reference, not in the
//! file: the average of 16 pressure samples it takes as it enters flight mode on the pad, its
//! "Zero Altitude baseline" (p17). In every sample file `pAlt` reads 0.00 on the first row; where
//! it reads more than one barometer step (below) off zero there, a note says so. `iAlt` is kept as
//! a channel: an integral of a single-axis accelerometer drifts, and in every sample file it falls
//! to zero, and stays there, a minute or more before the barometer's descent ends, while `Vel`
//! reads hundreds to thousands below zero by the end of the file. So `Vel` is no vertical speed
//! either: [`FlightLog::vertical_speed_m_s`] is `None`, as for the Blue Raven's inertial velocity.
//!
//! **The closing zeros.** In every sample file the last row, or the last two, write `Vel` as
//! 0.00 where the row before reads hundreds to thousands below zero: they aren't an integral of
//! `Acc`, which moves `Vel` by under 1 a row there. A closing run of exactly 0 in `Vel` or `iAlt`
//! is a gap, with a note, where the value before it lies further from 0 than twice what its rate
//! (`Acc` times the boost check's ratio for `Vel`; `Vel` for `iAlt`), held at its largest over the
//! run, changes it by in the run's time, plus a hundredth for the rounding. Where `Acc` is kept as
//! written, it sets no rate, and `Vel`'s closing zeros are kept. In two of the sample files the
//! last row writes 0.00 in every column; its `Acc` and `pAlt` are kept as written. The first row's
//! `Acc` is 0.00 in every sample file too, also kept.
//!
//! **The barometer's step.** The format's resolution ([`LogFormat::altitude_resolution_m`]) is
//! the coarsest of the sample files' smallest `pAlt` steps near the ground, 28.89 ft (8.81 m);
//! all of them are between 8.5 and 9 m. A count is a fixed step of pressure, so the step is larger
//! aloft and at a higher launch site.
//!
//! **Cells.** Every row must hold five cells, and its `MET` a number greater than the row
//! before's; otherwise the file is refused, naming the line. Any other cell that is empty or isn't
//! a number is a gap (`NaN`), counted in a note. A value finite as written that its unit makes
//! infinite, such as `1e308` g, is refused, naming its line, as every reader refuses one. Lines may
//! end in CR, LF or both, blank lines are skipped, and a byte-order mark is dropped. As every row must be five cells wide, the record
//! never holds more values than the file has cells, and a row is read no further than its sixth
//! cell before it is refused.

use hpr_atmos::ussa76::{DRY_AIR_GAS_CONSTANT_J_PER_KG_K, SEA_LEVEL_PRESSURE_PA, Ussa76};
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{finite, number, numbered_lines, short};

/// The format's name in messages.
const FORMAT: &str = "ARTS2 CSV";

/// The logger's name on the record.
const LOGGER: &str = "Ozark Aerospace ARTS2";

/// The header's names after its `#`, in order.
const NAMES: [&str; 5] = ["MET", "Acc", "Vel", "iAlt", "pAlt"];

/// The coarsest of the sample files' smallest `pAlt` steps near the ground, in feet (module docs,
/// *The barometer's step*).
const COARSEST_STEP_FT: f64 = 28.89;

/// The step between two altitudes the format can tell apart near the ground, m
/// ([`crate::log::LogFormat::altitude_resolution_m`]): the coarsest of the smallest steps `pAlt`
/// moves by in the 4 interpreted-data files among the project's sample logs as of 2026-10-13,
/// 28.89 ft; every one of them is between 8.5 and 9 m. `pAlt` is written to a hundredth, but moves
/// by its barometer's count, a fixed step of pressure ([`COUNT_PA`]), so the step is larger aloft
/// and at a higher site.
pub(crate) const ALTITUDE_STEP_M: f64 = COARSEST_STEP_FT * FOOT_M;

/// One count of the ARTS2's barometer, Pa, nominal. The ARTS2 reads a Motorola MPXA4100A pressure
/// sensor with its processor's 10-bit converter (*ARTS2 User Manual*, V1.0.0, p16). The sensor's
/// output is `V_S (0.010 59 P − 0.1518)`, `P` in kPa, ratiometric to its supply `V_S` (*MPXAZ4100A
/// Series* data sheet, Freescale, Rev 1, 2005: Table 2 and its note 2, p2; the transfer function,
/// p4). Read against that supply, one of the converter's 1,024 counts is `1 / (1,024 × 0.010 59)`
/// kPa, 92.2 Pa.
const COUNT_PA: f64 = 1_000.0 / (1_024.0 * 0.010_59);

/// How far, as a share, a count may be from [`COUNT_PA`]: the manual doesn't say what the
/// converter's reference is, and the sensor's supply may be 4.85 to 5.35 V (±5% of its 5.1 V, the
/// data sheet's Table 2), so a fixed reference beside it moves the count by that much; the rest
/// allows for the sensor's span off nominal (its accuracy is ±1.8% of the span) and the software's
/// conversion to altitude.
const COUNT_TOLERANCE: f64 = 0.15;

/// The highest launch site, m above sea level, whose ground step the band allows.
const HIGHEST_SITE_M: f64 = 3_000.0;

/// How far, as a share, a day's ground pressure may be from the standard atmosphere's at its site.
const WEATHER_SPREAD: f64 = 0.05;

/// The coldest and the hottest air at the ground, K, that the band allows: −20 °C and 45 °C.
const COLDEST_K: f64 = 253.15;
const HOTTEST_K: f64 = 318.15;

/// How often `pAlt`'s smallest step must be written, to within [`STEP_SPREAD`], to count: once
/// could be a placeholder, as a closing row of zeros is.
const STEP_REPEATS: usize = 3;

/// How far apart, in the written unit, two of `pAlt`'s steps may be and count as one: two
/// hundredths, the rounding of the two values each step is the difference of.
const STEP_SPREAD: f64 = 2.0 * ROUNDING;

/// How long before `Vel`'s zero at the apogee, s, the samples its slope is fitted to start: the
/// coast's last second, before any ejection's jolt.
const APOGEE_WINDOW_S: f64 = 1.0;

/// How long before `Vel`'s zero, s, a jump in `Acc` ends the window early (module docs, *The
/// apogee check*). A canopy whose drag grows as the speed squared, `g (v / v_t)²` for a descent
/// rate `v_t`, stops a rising rocket within `(v_t / g) atan(v₀ / v_t)`, under `(v_t / g) π/2`; so
/// one out more than 2 s before the zero has `v_t` over `4g/π`, 12.49 m/s, and makes `Vel` fall
/// over the zero's last second by `v_t tan(g · 1 s / v_t)` a second, at most 1.28 g, read in the
/// other unit 0.39 or 4.2 g: far from g ([`GRAVITY_MARGIN`]). A parachute out later is found by
/// its jump.
const DEPLOYMENT_SEARCH_S: f64 = 2.0;

/// How many rows before a row its `Acc` is compared with, through their median.
const JUMP_REFERENCE_ROWS: usize = 3;

/// How far, as a share of the level before it, `Acc` must move, on a row and the row after it, for
/// the row to be a jump: about half a g near the apogee, where `Acc` reads about −1 g.
const JUMP_SHARE: f64 = 0.5;

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
///   than the row before's, or a value too large to convert to SI.
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
    let length = units.length.meters();
    let acc_scale = units.acceleration.unwrap_or(1.0);
    let ialt_scale = if units.inertial { length } else { 1.0 };
    let convert_column = |column: usize, scale: f64| {
        let column_values = values.get(column).map_or(&[][..], Vec::as_slice);
        convert(column_values, scale, &rows.lines, NAMES[column + 1])
    };
    let acc = convert_column(ACC, acc_scale)?;
    let vel = convert_column(VEL, length)?;
    let ialt = convert_column(IALT, ialt_scale)?;
    let altitude_m = convert_column(PALT, length)?;
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
    let acc_quantity = if units.acceleration.is_some() {
        Quantity::Acceleration
    } else {
        Quantity::Unrecognized
    };
    let ialt_quantity = if units.inertial {
        Quantity::InertialAltitude
    } else {
        Quantity::Unrecognized
    };
    let channels = vec![
        channel(ACC, acc_quantity, acc),
        channel(VEL, Quantity::Speed, vel),
        channel(IALT, ialt_quantity, ialt),
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

/// A column's values times `scale`, each row's on its line in `lines` (the rows keep a line for
/// every value).
///
/// # Errors
///
/// [`LogError::Syntax`] for a value finite as written that `scale` makes infinite, such as
/// `1e308` g, on its row's line ([ADR-258] §3).
///
/// [ADR-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
fn convert(
    values: &[f64],
    scale: f64,
    lines: &[usize],
    column: &str,
) -> Result<Vec<f64>, LogError> {
    values
        .iter()
        .zip(lines)
        .map(|(value, &line)| finite(value * scale, FORMAT, line, column))
        .collect()
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
        // One more than the header names, so that a wider row is told apart without holding all
        // of its cells.
        let cells: Vec<&str> = text.split(',').take(NAMES.len() + 1).collect();
        if cells.len() != NAMES.len() {
            return Err(LogError::Syntax {
                format: FORMAT,
                line,
                message: format!(
                    "a row of {} cells, where the header names {}: {:?}",
                    if cells.len() > NAMES.len() {
                        format!("more than {}", NAMES.len())
                    } else {
                        cells.len().to_string()
                    },
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

/// Whether `Acc` jumps on a row: it and the row after it are both further from the median of the
/// [`JUMP_REFERENCE_ROWS`] rows before than [`JUMP_SHARE`] of that median, so that one row's spike
/// is no jump. A row without as many written rows before it, or whose own or next value isn't
/// written, counts as a jump: the coast can't be shown to go on through it.
fn acc_jumps(acc: &[f64], row: usize) -> bool {
    let Some(before) = row
        .checked_sub(JUMP_REFERENCE_ROWS)
        .and_then(|start| acc.get(start..row))
    else {
        return true;
    };
    let mut before = before.to_vec();
    if before.iter().any(|value| !value.is_finite()) {
        return true;
    }
    before.sort_by(f64::total_cmp);
    let Some(&level) = before.get(JUMP_REFERENCE_ROWS / 2) else {
        return true;
    };
    // Written so that a value that isn't a number is off the level.
    let off = |row: usize| {
        acc.get(row).is_none_or(|value| {
            let close = (value - level).abs() <= JUMP_SHARE * level.abs();
            !close
        })
    };
    off(row) && off(row + 1)
}

/// The stretch of coast `Vel`'s slope is fitted over, and the fit.
struct Coast {
    /// The least-squares slope of `Vel`, its unit a second.
    slope: f64,
    /// How many samples it is fitted to.
    points: usize,
    /// Where a jump in `Acc` ends the stretch before `Vel`'s zero, s, if one does.
    jump_s: Option<f64>,
}

/// Why no slope could be fitted: the time the stretch ends at, a jump's if one ended it, and how
/// many samples it holds.
struct NoCoast {
    jump_s: Option<f64>,
    points: usize,
}

/// The least-squares slope of `Vel` over the coast's last [`APOGEE_WINDOW_S`] before its zero
/// (module docs, *The apogee check*): the samples from that long before its zero, but not before
/// its highest, to the first at or below the zero; or, where `Acc` jumps in the
/// [`DEPLOYMENT_SEARCH_S`] before the zero, the samples of that long before the first jump, to the
/// row before it. A jump earlier in the window starts it after the jump.
fn coast_slope(
    time_s: &[f64],
    vel: &[f64],
    acc: &[f64],
    apogee: &Apogee,
) -> Result<Coast, NoCoast> {
    let time = |row: usize| time_s.get(row).copied().unwrap_or(f64::NAN);
    let scan_from_s = apogee.time_s - DEPLOYMENT_SEARCH_S - APOGEE_WINDOW_S;
    let jumps: Vec<usize> = (apogee.peak + 1..=apogee.row)
        .filter(|row| time(*row) >= scan_from_s && acc_jumps(acc, *row))
        .collect();
    let first = jumps
        .iter()
        .copied()
        .find(|row| time(*row) >= apogee.time_s - DEPLOYMENT_SEARCH_S);
    let (end, to_s, jump_s) = match first {
        Some(row) => (row, time(row), Some(time(row))),
        None => (apogee.row + 1, apogee.time_s, None),
    };
    let start = jumps
        .iter()
        .copied()
        .filter(|row| *row < end)
        .fold(apogee.peak, usize::max);
    let points: Vec<(f64, f64)> = (start..end)
        .filter_map(|row| Some((*time_s.get(row)?, *vel.get(row)?)))
        .filter(|(time, value)| value.is_finite() && *time >= to_s - APOGEE_WINDOW_S)
        .collect();
    let fit = fitted_slope(&points);
    match fit {
        Some(slope) if points.len() >= APOGEE_SAMPLES => Ok(Coast {
            slope,
            points: points.len(),
            jump_s,
        }),
        _ => Err(NoCoast {
            jump_s,
            points: points.len(),
        }),
    }
}

/// The least-squares slope through points `(t, v)`, if one is defined.
fn fitted_slope(points: &[(f64, f64)]) -> Option<f64> {
    let count = points.len() as f64;
    let mean_t = points.iter().map(|p| p.0).sum::<f64>() / count;
    let mean_v = points.iter().map(|p| p.1).sum::<f64>() / count;
    let spread: f64 = points.iter().map(|p| (p.0 - mean_t).powi(2)).sum();
    let slope = points
        .iter()
        .map(|p| (p.0 - mean_t) * (p.1 - mean_v))
        .sum::<f64>()
        / spread;
    slope.is_finite().then_some(slope)
}

/// The air's density at the ground, kg/m³, densest and thinnest, that the barometer's step band
/// allows (module docs, *The barometer's step*): sea level's standard pressure 5% high in the
/// coldest air, and the standard pressure at the highest site (U.S. Standard Atmosphere, 1976,
/// [`Ussa76`]) 5% low in the hottest, by the ideal gas law `ρ = p / (R T)`.
fn ground_densities_kg_m3() -> (f64, f64) {
    let gas = DRY_AIR_GAS_CONSTANT_J_PER_KG_K;
    let densest = SEA_LEVEL_PRESSURE_PA * (1.0 + WEATHER_SPREAD) / (gas * COLDEST_K);
    let highest_pa = Ussa76::standard()
        .sample(HIGHEST_SITE_M)
        .map_or(f64::NAN, |sample| sample.air.pressure_pa);
    let thinnest = highest_pa * (1.0 - WEATHER_SPREAD) / (gas * HOTTEST_K);
    (densest, thinnest)
}

/// The least and the most, m, one barometer count is in altitude at the ground: the hydrostatic
/// `Δh = Δp / (ρ g)`, [`COUNT_PA`] within [`COUNT_TOLERANCE`], in the densest and the thinnest
/// air of [`ground_densities_kg_m3`].
fn step_band_m() -> (f64, f64) {
    let (densest, thinnest) = ground_densities_kg_m3();
    (
        COUNT_PA * (1.0 - COUNT_TOLERANCE) / (densest * STANDARD_GRAVITY_MPS2),
        COUNT_PA * (1.0 + COUNT_TOLERANCE) / (thinnest * STANDARD_GRAVITY_MPS2),
    )
}

/// The band [`step_band_m`] in a unit, as the file would write it.
fn step_band(unit: Length) -> (f64, f64) {
    let (least, most) = step_band_m();
    (least / unit.meters(), most / unit.meters())
}

/// `pAlt`'s smallest step, in its written unit, and how many of its steps lie within
/// [`STEP_SPREAD`] above it: the least change between two written values in a row, gaps passed
/// over, that at least [`STEP_REPEATS`] changes come within [`STEP_SPREAD`] of. `None` if none
/// does.
fn smallest_step(palt: &[f64]) -> Option<(f64, usize)> {
    let written: Vec<f64> = palt.iter().copied().filter(|v| v.is_finite()).collect();
    // In hundredths, as written; a change that isn't finite or is under a hundredth is no step.
    let mut steps: Vec<f64> = written
        .windows(2)
        .filter_map(|pair| match pair {
            [a, b] => Some(((b - a).abs() * 100.0).round()),
            _ => None,
        })
        .filter(|step| step.is_finite() && *step >= 1.0)
        .collect();
    steps.sort_by(f64::total_cmp);
    let spread = (STEP_SPREAD * 100.0).round();
    steps.iter().enumerate().find_map(|(at, step)| {
        let within = steps
            .get(at..)
            .unwrap_or_default()
            .iter()
            .take_while(|other| **other <= step + spread)
            .count();
        (within >= STEP_REPEATS).then_some((step / 100.0, within))
    })
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
    let window = |jump_s: Option<f64>| {
        jump_s.map_or_else(
            || format!("the {APOGEE_WINDOW_S} s before"),
            |at| format!("the {APOGEE_WINDOW_S} s of coast before `Acc` jumps at {at:.2} s"),
        )
    };
    let coast = match coast_slope(time_s, vel, acc, &apogee) {
        Ok(coast) => coast,
        Err(none) => {
            return Err(refuse(
                apogee_line,
                format!(
                    "into its zero at {:.2} s, `Vel` has {} samples, fewer than {APOGEE_SAMPLES}, \
                     in {}. Its slope at the apogee decides its unit, and it can't be fitted",
                    apogee.time_s,
                    none.points,
                    window(none.jump_s)
                ),
            ));
        }
    };
    let slope = coast.slope;
    let by_slope: Vec<Length> = Length::BOTH
        .into_iter()
        .filter(|unit| near(-slope, unit.gravity(), GRAVITY_MARGIN))
        .collect();
    let [length] = by_slope[..] else {
        return Err(refuse(
            apogee_line,
            format!(
                "into its zero at {:.2} s, over {}, `Vel` changes by {slope:.3} a second. That is \
                 not within {:.0}% of standard gravity in either m/s² (−9.807) or ft/s² \
                 (−32.174). No unit for it, nor for `pAlt`, can be decided",
                apogee.time_s,
                window(coast.jump_s),
                GRAVITY_MARGIN * 100.0
            ),
        ));
    };

    // The step check: the barometer's count, apart from `Vel`.
    let (_, climb) = rise_and_integral(time_s, vel, vel, apogee.row);
    let climb_m = climb * length.meters();
    // A climb that isn't a number fails too.
    let settles = climb_m >= SETTLING_CLIMB_M;
    if !settles {
        return Err(refuse(
            apogee_line,
            format!(
                "`Vel` integrates to a climb of {climb_m:.1} m by the apogee. That is under the \
                 {SETTLING_CLIMB_M:.0} m from which the barometer's step lets `pAlt`'s unit be \
                 decided"
            ),
        ));
    }
    let bands = Length::BOTH.map(|unit| {
        let (least, most) = step_band(unit);
        format!("{least:.2} to {most:.2} {}", unit.symbol())
    });
    let Some((step, repeats)) = smallest_step(palt) else {
        return Err(refuse(
            header_line,
            format!(
                "`pAlt` has no step written {STEP_REPEATS} times, to within {STEP_SPREAD:.2}. The \
                 barometer's count can't be measured, and it must bear out the unit `Vel`'s \
                 slope gives"
            ),
        ));
    };
    let by_step: Vec<Length> = Length::BOTH
        .into_iter()
        .filter(|unit| {
            let (least, most) = step_band(*unit);
            step >= least && step <= most
        })
        .collect();
    let [stepped] = by_step[..] else {
        return Err(refuse(
            header_line,
            format!(
                "`pAlt`'s smallest step, {step:.2}, is one count of the barometer near the ground \
                 in neither meters ({}) nor feet ({}). No unit for it, nor for `Vel`, can be \
                 decided",
                bands[0], bands[1]
            ),
        ));
    };
    if stepped != length {
        return Err(refuse(
            apogee_line,
            format!(
                "`Vel`'s slope into its zero at {:.2} s, {slope:.3} a second, is standard gravity \
                 in {} per second squared. `pAlt`'s smallest step, {step:.2}, is one count of \
                 the barometer in {}. The two disagree, and no unit is decided",
                apogee.time_s,
                length.name(),
                stepped.name()
            ),
        ));
    }
    let gravities = -slope / length.gravity();
    let mut notes = vec![format!(
        "`Vel` read in {} per second, from the apogee and the barometer's step: its line falls \
         at {:.3} a second. That line runs through its {} samples of {}, into its zero at \
         {:.2} s. The line's fall is {gravities:.3} of standard gravity's {:.3} {}/s², within \
         {:.0}%, and `pAlt`'s step is a count in {} too",
        length.name(),
        -slope,
        coast.points,
        window(coast.jump_s),
        apogee.time_s,
        length.gravity(),
        length.symbol(),
        GRAVITY_MARGIN * 100.0,
        length.name()
    )];

    // The barometric check: `pAlt`'s highest value against `Vel`'s climb.
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
                 integrates to by the apogee, {climb:.2} {}, not within {:.0}%. No unit for the \
                 barometric altitude can be decided",
                length.symbol(),
                BAROMETRIC_MARGIN * 100.0
            ),
        ));
    }
    let band = match length {
        Length::Meter => &bands[0],
        Length::Foot => &bands[1],
    };
    notes.push(format!(
        "`pAlt` read in {}, from its step and `Vel`: its smallest step is {step:.2}. {repeats} of \
         its steps are within {STEP_SPREAD:.2} of it. The smallest step is one count of the \
         barometer near the ground in {} ({band}). `pAlt`'s highest value, {highest:.2}, is \
         {share:.3} times the climb `Vel` integrates to by the apogee, {climb:.2} {}, within \
         {:.0}%",
        length.name(),
        length.name(),
        length.symbol(),
        BAROMETRIC_MARGIN * 100.0
    ));
    let met_note = format!(
        "`MET` read in seconds, from the apogee: `Vel` is in {} per second, as the barometer's \
         step has it. `Vel` falls into its zero at {gravities:.3} of standard gravity a second. A \
         clock in milliseconds or minutes would put that fall 1,000 or 60 times off",
        length.name()
    );

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
             rises {ratio:.3} times `Acc`'s integral. That is within {:.0}% of standard \
             gravity's {:.3} {}/s²",
            INTEGRAL_MARGIN * 100.0,
            length.gravity(),
            length.symbol()
        ));
        Some(STANDARD_GRAVITY_MPS2)
    } else {
        notes.push(format!(
            "`Acc` kept as written: over the boost, `Vel` rises {ratio:.3} times `Acc`'s \
             integral. That is within {:.0}% of neither 1 nor standard gravity in {} per second \
             squared",
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
    notes.push(met_note);
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
    // `Acc`'s rate moves `Vel` by the boost check's ratio only where that check bore `Acc` out;
    // kept as written, `Acc` sets no reach, and `Vel`'s zeros are kept.
    let vel_scale = units.acceleration.and(units.boost_ratio).map(f64::abs);
    for (column, rate, scale) in [(VEL, ACC, vel_scale), (IALT, VEL, Some(1.0))] {
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
                 {previous:.2}. That is more than twice what `{}` can change it by in {span:.2} s. \
                 So they aren't its integral, and they are gaps",
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
        flight_deploying(boost_s, boost_m_s2, length_m, acc_m_s2, None, 0.1)
    }

    /// [`flight`], with a parachute out once the rocket has slowed to `deploy_m_s` on its way up,
    /// if given: from then on to the apogee, the accelerometer reads its drag besides the
    /// rocket's, g (v / 5 m/s)², the drag of a canopy that would bring the rocket down at 5 m/s;
    /// a row every `dt` seconds.
    fn flight_deploying(
        boost_s: f64,
        boost_m_s2: f64,
        length_m: f64,
        acc_m_s2: f64,
        deploy_m_s: Option<f64>,
        dt: f64,
    ) -> String {
        let mut deployed = false;
        let mut text = String::from("# MET Acc Vel iAlt pAlt \n");
        let (mut vel, mut ialt, mut previous) = (0.0_f64, 0.0_f64, None::<f64>);
        let mut apogee: Option<(f64, f64)> = None;
        for row in 0..10_000 {
            let time = f64::from(row) * dt;
            let acc_si = if row == 0 {
                0.0
            } else if time <= boost_s + 1e-9 {
                boost_m_s2
            } else if apogee.is_none() {
                deployed |= deploy_m_s.is_some_and(|speed| vel <= speed);
                let chute = if deployed {
                    STANDARD_GRAVITY_MPS2 * (vel / 5.0).powi(2)
                } else {
                    0.0
                };
                -STANDARD_GRAVITY_MPS2 - 0.0004 * vel * vel.abs() - chute
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
        let from = "per second, from the apogee and the barometer's step: ";
        assert!(metric.notes[0].starts_with(&format!("`Vel` read in meters {from}")));
        assert!(english.notes[0].starts_with(&format!("`Vel` read in feet {from}")));
        assert!(metric.notes[1].starts_with("`pAlt` read in meters, from its step and `Vel`: "));
        assert!(english.notes[1].starts_with("`pAlt` read in feet, from its step and `Vel`: "));
        assert!(english.notes[2].starts_with("`Acc` read in standard gravities, "));
        assert!(metric.notes[2].starts_with("`Acc` read in meters per second squared, "));
        for log in [&metric, &english] {
            assert!(log.notes[4].starts_with("`MET` read in seconds, from the apogee: "));
        }
        assert!(metric.vertical_speed_m_s.is_none());
    }

    /// A velocity whose slope at the apogee is g in neither unit settles no unit for the primary
    /// track: the file is refused, not read in a guessed unit.
    #[test]
    fn a_velocity_that_falls_at_no_gravity_is_refused() {
        for factor in [2.0, 0.5, 1.6 / FOOT_M] {
            let message = unit_message(read(&scaled(&metric(), VEL, factor)));
            assert!(
                message.contains("not within 25% of standard gravity in either"),
                "{factor}: {message}"
            );
        }
    }

    /// The barometric altitude in feet beside a velocity in m/s, or in meters beside one in ft/s:
    /// the barometer's step and the velocity's slope disagree, and the file is refused. A step
    /// that is a count in both units' bands but a highest value half again the climb is refused
    /// by the barometric check.
    #[test]
    fn a_barometric_altitude_in_another_unit_is_refused() {
        for (text, slope, step) in [
            (scaled(&metric(), PALT, 1.0 / FOOT_M), "meters", "feet"),
            (scaled(&english(), PALT, FOOT_M), "feet", "meters"),
        ] {
            let message = unit_message(read(&text));
            assert!(
                message.contains(&format!(
                    "is standard gravity in {slope} per second squared"
                )) && message.contains(&format!(
                    "one count of the barometer in {step}. The two disagree"
                )),
                "{message}"
            );
        }
        for text in [metric(), english()] {
            let message = unit_message(read(&scaled(&text, PALT, 1.5)));
            assert!(message.starts_with("`pAlt`'s highest value, "), "{message}");
        }
    }

    /// The barometer's step settles a unit only inside one unit's band: a step of a third of a
    /// count, or one between the bands, is refused, and so is a `pAlt` with no step written
    /// three times.
    #[test]
    fn a_step_in_neither_band_is_refused() {
        let (least_m, most_m) = step_band(Length::Meter);
        let (least_ft, most_ft) = step_band(Length::Foot);
        // The bands are apart: the most a count is in meters is under the least in feet.
        assert!(most_m < least_ft, "{most_m} m against {least_ft} ft");
        // 1 / (1,024 × 0.010 59) kPa; the densest air 1.464 kg/m³, the thinnest 0.729.
        assert!((COUNT_PA - 92.216).abs() < 1e-3, "{COUNT_PA}");
        let (densest, thinnest) = ground_densities_kg_m3();
        assert!((densest - 1.4640).abs() < 1e-3, "{densest}");
        assert!((thinnest - 0.7294).abs() < 1e-3, "{thinnest}");
        assert!((least_m - 5.460).abs() < 1e-3 && (most_m - 14.83).abs() < 1e-2);
        assert!((least_ft - 17.91).abs() < 1e-2 && (most_ft - 48.64).abs() < 1e-2);
        // Each scale moves the 9 m step and the highest value together, so only the step's band
        // decides: 9 × 0.5 = 4.5 m is below the metric band, 9 × 1.8 = 16.2 between the bands.
        for factor in [0.5, 1.8] {
            let message = unit_message(read(&scaled(&metric(), PALT, factor)));
            assert!(
                message.contains("in neither meters ("),
                "{factor}: {message}"
            );
        }
        // A `pAlt` that steps by 9 once, and by more each other time, has no step to measure; a
        // single smaller step, as a closing zero can make, isn't taken for the count.
        assert_eq!(smallest_step(&[0.0, 9.0, 29.0, 70.0, 153.0]), None);
        let steps = [0.0, 9.0, 18.01, f64::NAN, 27.02, 36.02, 30.0];
        assert_eq!(smallest_step(&steps), Some((9.0, 4)));
        // The metric flight with `pAlt` stepping by 0.37 more on each row than the row before.
        let text: String = metric()
            .lines()
            .enumerate()
            .map(|(row, line)| match line.rsplit_once(',') {
                Some((head, _)) if row > 0 => {
                    let row = row as f64;
                    format!("{head},{:.2}\n", 0.37 * row * (row + 1.0) / 2.0)
                }
                _ => format!("{line}\n"),
            })
            .collect();
        let message = unit_message(read(&text));
        assert!(
            message.starts_with("`pAlt` has no step written 3 times"),
            "{message}"
        );
    }

    /// The bound [`DEPLOYMENT_SEARCH_S`] rests on: a canopy that takes more than 2 s to stop the
    /// rocket has a descent rate over `4g/π`, and then makes `Vel` fall at no more than 1.28 g
    /// over the zero's last second, outside the other unit's 25% of g either way.
    #[test]
    fn a_parachute_out_more_than_2_s_early_slows_vel_little() {
        let g = STANDARD_GRAVITY_MPS2;
        // The time to stop from any speed is under (v_t / g) π/2: 2 s at v_t = 4g/π.
        let least = 4.0 * g / std::f64::consts::PI;
        assert!((least * std::f64::consts::FRAC_PI_2 / g - DEPLOYMENT_SEARCH_S).abs() < 1e-12);
        // The mean fall over the last second, v_t tan(g / v_t), falls toward g as v_t grows.
        let fall = |rate: f64| rate * (g / rate).tan() / g;
        let mut before = fall(least);
        assert!(before < 1.28 && before > 1.27, "{before}");
        for rate in (1..=100).map(|step| least + f64::from(step)) {
            let now = fall(rate);
            assert!(now < before && now > 1.0, "{rate}: {now}");
            before = now;
        }
        // Read in the other unit, 1.28 g is 0.39 g or 4.2 g.
        assert!(!near(1.28 * FOOT_M, 1.0, GRAVITY_MARGIN));
        assert!(!near(1.28 / FOOT_M, 1.0, GRAVITY_MARGIN));
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
        // 1 s at 40 m/s², about 100 m up after a 4 s coast.
        let message = unit_message(read(&flight(1.0, 40.0, 1.0, 1.0)));
        assert!(message.contains("under the 176 m"), "{message}");
        // 0.3 s at 30 m/s²: under a second of coast, too little to fit `Vel`'s slope to.
        let message = unit_message(read(&flight(0.3, 30.0, 1.0, 1.0)));
        assert!(
            message.contains("`Vel` has 1 samples, fewer than 3, in the 1 s of coast before"),
            "{message}"
        );
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
        // A row wider than five cells is refused without reading past its sixth.
        match read(&format!("{header}0,0,0,0,0{}\n", ",0".repeat(1_000))) {
            Err(LogError::Syntax {
                line: 2, message, ..
            }) => {
                assert!(
                    message.starts_with("a row of more than 5 cells, "),
                    "{message}"
                );
            }
            other => panic!("{other:?}"),
        }
    }

    /// An `Acc` in g finite as written and infinite in m/s² is refused on its line (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let text = english();
        let at = text.find("\n17.000,").unwrap() + 1;
        let end = at + text[at..].find(',').unwrap();
        let next = end + 1 + text[end + 1..].find(',').unwrap();
        let text = format!("{}1e308{}", &text[..=end], &text[next..]);
        assert_eq!(
            read(&text).unwrap_err().to_string(),
            "ARTS2 CSV line 172: the column Acc holds a value too large for its unit's \
             conversion to SI"
        );
        // `1e308` is finite as written, and as written it is kept when `Acc` has no unit.
        let values = convert(&[1e308, f64::NAN, 2.0], 1.0, &[2, 3, 4], "Acc").unwrap();
        assert!(values[0] == 1e308 && values[1].is_nan() && values[2] == 2.0);
        // The first value the scale makes infinite is refused, on its own line.
        assert_eq!(
            convert(&[2.0, -1e308, 1e308], 10.0, &[2, 3, 4], "Acc")
                .unwrap_err()
                .to_string(),
            "ARTS2 CSV line 3: the column Acc holds a value too large for its unit's conversion \
             to SI"
        );
    }

    /// `Vel`'s closing zeros are judged against `Acc`'s reach only where the boost check read
    /// `Acc` in a unit: kept as written, `Acc` sets no reach, and the zeros are kept.
    #[test]
    fn closing_zeros_need_an_acceleration_in_a_unit() {
        let text = "# MET Acc Vel iAlt pAlt\n0,0,0,0,0\n0.02,1,100,0,0\n0.04,1,0,2,0\n";
        let rows = rows(numbered_lines(text).skip(1)).unwrap();
        for (acceleration, kept) in [(Some(1.0), false), (None, true)] {
            let units = Units {
                length: Length::Meter,
                acceleration,
                inertial: true,
                boost_ratio: Some(1.0),
                notes: Vec::new(),
            };
            let mut values = rows.values.clone();
            let mut notes = Vec::new();
            closing_zeros(&rows, &units, &mut values, &mut notes);
            assert_eq!(values[VEL][2] == 0.0, kept, "{acceleration:?}");
            assert_eq!(notes.is_empty(), kept, "{notes:?}");
        }
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

    /// A metric flight, a row every 0.02 s as the logger's 50 Hz, whose parachute comes out at
    /// 25 m/s on the way up, 0.62 s before `Vel`'s zero. Over the second before the zero, the
    /// chute's drag makes `Vel` fall at 3.7 g in m/s, 1.13 g in ft/s, within 25% of g in feet:
    /// fitted there, as the reader once did, the file would be read in feet, its heights 3.28
    /// times too low. The jump in `Acc` ends the window at the deployment, where `Vel` falls at
    /// 1.04 g in m/s, and the barometer's step agrees: it is read in meters.
    #[test]
    fn a_parachute_out_before_the_apogee_does_not_flip_the_unit() {
        let text = flight_deploying(2.0, 70.0, 1.0, 1.0, Some(25.0), 0.02);
        let rows = rows(numbered_lines(&text).skip(1)).unwrap();
        let [acc, vel, _, palt] = &rows.values;
        let apogee = apogee(&rows.time_s, vel).unwrap();
        // The fit over the second before the zero, as the reader made it before the jump check.
        let old: Vec<(f64, f64)> = (apogee.peak..=apogee.row)
            .map(|row| (rows.time_s[row], vel[row]))
            .filter(|(time, _)| *time >= apogee.time_s - APOGEE_WINDOW_S)
            .collect();
        let old_slope = fitted_slope(&old).unwrap();
        assert!(
            near(-old_slope, Length::Foot.gravity(), GRAVITY_MARGIN),
            "{old_slope}"
        );
        assert!(!near(-old_slope, Length::Meter.gravity(), GRAVITY_MARGIN));
        // The coast before the jump falls at g in m/s, and the step is a count in meters.
        let Ok(coast) = coast_slope(&rows.time_s, vel, acc, &apogee) else {
            panic!("no coast");
        };
        let jump = coast.jump_s.unwrap();
        assert!(
            jump < apogee.time_s - 0.5 && jump > apogee.time_s - 1.0,
            "{jump}"
        );
        assert!(
            near(-coast.slope, Length::Meter.gravity(), 0.1),
            "{}",
            coast.slope
        );
        assert_eq!(smallest_step(palt).map(|(step, _)| step), Some(9.0));
        let log = read(&text).unwrap();
        let note = &log.notes[0];
        assert!(
            note.starts_with(
                "`Vel` read in meters per second, from the apogee and the barometer's step: "
            ) && note.contains(&format!("of coast before `Acc` jumps at {jump:.2} s")),
            "{note}"
        );
        let top = highest(&log.altitude_m);
        assert!(top > 700.0 && top < 1_000.0, "{top}");
    }

    #[test]
    fn the_step_is_the_coarsest_of_the_sample_files() {
        assert!((ALTITUDE_STEP_M - 8.805_672).abs() < 1e-9);
    }
}
