//! The day's air, as the log records it at the pad (#1259): whether the bounds' allowance for the
//! day's air, [`AIR_TEMPERATURE_SPREAD_K`] from the 1976 US Standard Atmosphere, covers it.
//!
//! **Why it matters.** A barometric logger turns pressure into height by the standard
//! atmosphere. By the hypsometric equation, `dz = −(R T/g) dp/p`, the height between two
//! pressures goes as the air's temperature between them. So a height read by the standard
//! atmosphere is `T_std/T` of the true one over the layer it spans, and off by `ΔT/T_std` of
//! itself when the day's air is `ΔT` off the standard (`docs/research/altimeter-height-formulas.md`;
//! U.S. Standard Atmosphere, 1976, NOAA-S/T 76-1562). A speed or a rate of descent, a height over a
//! time, is off by the same share. Each bound holds [`TEMPERATURE_SHARE`] for it: 15 K over the
//! coldest standard air below 20 km. On a hot day the heights read low, and so do the speeds and
//! the rates; on a cold day, high. A summer afternoon at a high plains or desert field is often more
//! than 15 K over standard: at a 1,190 m field, 35 °C is 27.7 K over.
//!
//! **What is read.** The day's offset is the log's temperature at the pad less the standard
//! atmosphere's at the pad's height above sea level, `ΔT = T_log − T_std(z_pad)`. The log's
//! temperature at the pad is the median of its temperature samples over the span the readings take
//! the pad from ([`super::Readings::pad_altitude_m`]). The pad's height above sea level comes, in
//! this order, from a pressure channel over the same span, turned into a height by the standard
//! atmosphere (its pressure altitude: the height the standard atmosphere's own temperature is
//! read at, as the logger's conversion reads it); from a barometric altitude channel above sea
//! level; or from the ground elevation the logger states. With none of these, or no temperature
//! channel, there is nothing to compare, and no [`DayAir`].
//!
//! **The size**, stated on the unflattering side: `|ΔT| / T_std(z_top)`, the standard temperature
//! at the apogee's height above sea level, the coldest standard air the climb spans, so the
//! largest share the layer's mean temperature allows. With no apogee read, the coldest standard
//! air below 20 km, 216.65 K, stands for it.
//!
//! **A note, not a wider bound.** A logger's temperature channel is often the board's or the
//! sensor die's, warmed in the electronics bay by the sun and the electronics, not the air's
//! ([`crate::log::FlightLog::temperature_k`]). Widening every bound to it would move the bounds,
//! and the 90% intervals calibrated on the synthetic flights (#1055, #1061), by a figure that may
//! be the bay's. So the readings keep their bounds, and when the logged offset passes
//! [`AIR_TEMPERATURE_SPREAD_K`], the command line prints a note beside each reading the day's air
//! moves, with its size, for the flyer to weigh.

use hpr_atmos::Ussa76;
use serde::{Deserialize, Serialize};

use super::{AIR_TEMPERATURE_SPREAD_K, MAX_INTERVAL_S};
use crate::filter::median;
use crate::log::{FlightLog, Quantity};

#[cfg(doc)]
use super::TEMPERATURE_SHARE;

/// The coldest temperature in the 1976 standard atmosphere below 20 km, K: the tropopause's.
pub const COLDEST_STANDARD_K: f64 = 216.65;

/// The range a logged temperature must fall in to be taken for the pad's air, K: −100 °C to
/// +100 °C, well past the coldest and hottest air on record (−89.2 °C at Vostok, 56.7 °C at Death
/// Valley, WMO's archive of weather extremes), and room for a sun-warmed bay. A channel outside it,
/// such as a column of zeros a logger with no sensor writes, isn't the air, and gives no
/// [`DayAir`].
pub const LOGGED_AIR_RANGE_K: (f64, f64) = (173.15, 373.15);

/// The 1976 standard atmosphere's temperature at a height above sea level, K: 288.15 K at sea
/// level, less its 6.5 K/km lapse to 11 km, then its isothermal 216.65 K, taken to hold above
/// it (U.S. Standard Atmosphere, 1976, table 4). Heights here are geopotential, as a pressure
/// altitude is; below 11 km the two differ by under 0.2%.
pub(crate) fn standard_temperature_k(height_msl_m: f64) -> f64 {
    (288.15 - 0.0065 * height_msl_m.min(11_000.0)).max(COLDEST_STANDARD_K)
}

with_all! {
/// Where the pad's height above sea level came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PadHeightFrom {
    /// A pressure channel over the pad's span, turned into a height by the 1976 standard
    /// atmosphere: the pad's pressure altitude.
    Pressure,
    /// A barometric altitude channel above sea level, over the pad's span.
    AltitudeMsl,
    /// The ground elevation the logger states.
    StatedGroundElevation,
}
}

/// The day's air at the pad, from the log's own temperature, against the standard atmosphere's.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayAir {
    /// The log's temperature at the pad, K: the median of its samples over the pad's span.
    pub pad_temperature_k: f64,
    /// The temperature channel it came from, as the file names it.
    pub temperature_column: String,
    /// The pad's height above sea level, m.
    pub pad_height_msl_m: f64,
    /// Where that height came from.
    pub pad_height_from: PadHeightFrom,
    /// The 1976 standard atmosphere's temperature at the pad's height, K.
    pub pad_standard_temperature_k: f64,
    /// The day's offset from the standard at the pad, K: the log's temperature less the
    /// standard's; positive on a hot day, which reads barometric heights, speeds and rates low.
    pub offset_k: f64,
    /// Whether the bounds' allowance for the day's air covers the offset: its size no more than
    /// [`AIR_TEMPERATURE_SPREAD_K`].
    pub covered: bool,
    /// The standard temperature the share is taken at, K: at the apogee's height above sea level,
    /// the coldest standard air the climb spans; [`COLDEST_STANDARD_K`] with no apogee read.
    pub top_standard_temperature_k: f64,
    /// The share of a barometric height, speed or rate of descent the day's air may move it by,
    /// `|ΔT| / T_std(z_top)` (the hypsometric equation): up from the reading on a hot day, down on
    /// a cold one. Against the bounds' own [`TEMPERATURE_SHARE`].
    pub share: f64,
}

impl DayAir {
    /// Whether the day is hotter than the standard: the true heights, speeds and rates then lie
    /// above the readings.
    #[must_use]
    pub fn is_hot(&self) -> bool {
        self.offset_k > 0.0
    }

    /// The share taken at the top of a climb `top_height_m` above the pad.
    pub(crate) fn set_top(&mut self, top_height_m: f64) {
        let top = standard_temperature_k(self.pad_height_msl_m + top_height_m.max(0.0));
        self.top_standard_temperature_k = top;
        self.share = self.offset_k.abs() / top;
    }
}

/// The day's air at the pad, from the log's temperature over the pad's span `[from, to]` s, or
/// `None` where the log has no temperature, no height above sea level for the pad, or a
/// temperature that can't be the air's ([`LOGGED_AIR_RANGE_K`]).
pub(crate) fn day_air(log: &FlightLog, pad_span_s: (f64, f64)) -> Option<DayAir> {
    let (temperature_column, pad_temperature_k) = pad_temperature(log, pad_span_s)?;
    let (pad_height_msl_m, pad_height_from) = pad_height(log, pad_span_s)?;
    let pad_standard_temperature_k = standard_temperature_k(pad_height_msl_m);
    let offset_k = pad_temperature_k - pad_standard_temperature_k;
    Some(DayAir {
        pad_temperature_k,
        temperature_column,
        pad_height_msl_m,
        pad_height_from,
        pad_standard_temperature_k,
        offset_k,
        covered: offset_k.abs() <= AIR_TEMPERATURE_SPREAD_K,
        top_standard_temperature_k: COLDEST_STANDARD_K,
        share: offset_k.abs() / COLDEST_STANDARD_K,
    })
}

/// The log's temperature at the pad, K, and the column it came from: the first temperature
/// channel, or the record's own temperature on its clock.
fn pad_temperature(log: &FlightLog, span: (f64, f64)) -> Option<(String, f64)> {
    let (column, value) = match log.channels_of(Quantity::Temperature).next() {
        Some(channel) => (
            channel.column.clone(),
            over_span(&channel.time_s, &channel.values, span)?,
        ),
        None => (
            "temperature".to_owned(),
            over_span(&log.time_s, log.temperature_k.as_deref()?, span)?,
        ),
    };
    (LOGGED_AIR_RANGE_K.0..=LOGGED_AIR_RANGE_K.1)
        .contains(&value)
        .then_some((column, value))
}

/// The pad's height above sea level, m, and where it came from.
fn pad_height(log: &FlightLog, span: (f64, f64)) -> Option<(f64, PadHeightFrom)> {
    let pressure = log
        .channels_of(Quantity::Pressure)
        .next()
        .and_then(|channel| {
            let pressure_pa = over_span(&channel.time_s, &channel.values, span)?;
            Ussa76::standard().pressure_altitude_m(pressure_pa).ok()
        });
    let msl = || {
        log.channels_of(Quantity::AltitudeMsl)
            .next()
            .and_then(|channel| over_span(&channel.time_s, &channel.values, span))
    };
    pressure
        .map(|height| (height, PadHeightFrom::Pressure))
        .or_else(|| msl().map(|height| (height, PadHeightFrom::AltitudeMsl)))
        .or_else(|| {
            log.stated
                .ground_elevation_msl_m
                .map(|height| (height, PadHeightFrom::StatedGroundElevation))
        })
        .filter(|(height, _)| height.is_finite())
}

/// The median of a channel's finite samples within `[from, to]` s; with none there, its finite
/// sample nearest the span, if within [`MAX_INTERVAL_S`] of it, as a channel sampled more slowly
/// than the track may hold none on a short pad.
fn over_span(time_s: &[f64], values: &[f64], (from, to): (f64, f64)) -> Option<f64> {
    // A span the wrong way round, or not a number, holds no time.
    if from.partial_cmp(&to).is_none_or(std::cmp::Ordering::is_gt) {
        return None;
    }
    let finite = || {
        time_s
            .iter()
            .zip(values)
            .filter(|(t, v)| t.is_finite() && v.is_finite())
    };
    let mut within: Vec<f64> = finite()
        .filter(|(t, _)| (from..=to).contains(*t))
        .map(|(_, v)| *v)
        .collect();
    if let Some(value) = median(&mut within) {
        return Some(value);
    }
    finite()
        .map(|(t, v)| ((t - t.clamp(from, to)).abs(), *v))
        .filter(|(gap, _)| *gap <= MAX_INTERVAL_S)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, v)| v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::{Channel, LogFormat, Stated};
    use crate::readings::{Reading, Readings, TEMPERATURE_SHARE, read};
    use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

    const DT: f64 = 0.05;
    /// An invented field's height above sea level, m.
    const FIELD_M: f64 = 1_190.0;
    /// The invented flight's drogue and main rates, m/s, and the main's opening height, m.
    const DROGUE_M_S: f64 = 25.0;
    const MAIN_M_S: f64 = 6.0;
    const MAIN_AT_M: f64 = 200.0;

    /// An invented flight's true height above the pad, m, at `t` s: on the pad to 1 s, 60 m/s²
    /// for 2 s (120 m/s, 120 m), a vacuum coast to its top, 854.2 m at 15.24 s, then down under a
    /// drogue at 25 m/s to 200 m and a main at 6 m/s to the pad.
    fn flight(t: f64) -> f64 {
        let g = STANDARD_GRAVITY_MPS2;
        let (burn_s, accel) = (2.0, 60.0);
        let burnout_v = accel * burn_s;
        let burnout_h = 0.5 * accel * burn_s * burn_s;
        let top_s = 1.0 + burn_s + burnout_v / g;
        let top_m = burnout_h + burnout_v * burnout_v / (2.0 * g);
        let main_s = top_s + (top_m - MAIN_AT_M) / DROGUE_M_S;
        if t <= 1.0 {
            0.0
        } else if t < 1.0 + burn_s {
            0.5 * accel * (t - 1.0).powi(2)
        } else if t < top_s {
            let c = t - 1.0 - burn_s;
            burnout_h + burnout_v * c - 0.5 * g * c * c
        } else if t < main_s {
            top_m - DROGUE_M_S * (t - top_s)
        } else {
            (MAIN_AT_M - MAIN_M_S * (t - main_s)).max(0.0)
        }
    }

    /// The flight's true top, m above the pad.
    fn true_top_m() -> f64 {
        let v = 120.0;
        120.0 + v * v / (2.0 * STANDARD_GRAVITY_MPS2)
    }

    /// A log of the flight from a field [`FIELD_M`] up, on a day `offset_k` off the standard
    /// atmosphere at every height (the hydrostatic air of [`Ussa76::with_offset`], sea-level
    /// pressure standard): a pressure channel and a temperature channel reading the air.
    fn day_log(offset_k: f64) -> FlightLog {
        let day = Ussa76::with_offset(offset_k, 101_325.0).unwrap();
        let times: Vec<f64> = (0..)
            .map(|i| f64::from(i) * DT)
            .take_while(|t| *t <= 90.0)
            .collect();
        let air: Vec<_> = times
            .iter()
            .map(|t| day.sample(FIELD_M + flight(*t)).unwrap().air)
            .collect();
        FlightLog {
            format: LogFormat::GenericTable,
            logger: "invented".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            time_s: Vec::new(),
            altitude_m: Vec::new(),
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: vec![
                Channel::new(
                    "Pressure (Pa)",
                    Quantity::Pressure,
                    times.clone(),
                    air.iter().map(|a| a.pressure_pa).collect(),
                ),
                Channel::new(
                    "Temp (K)",
                    Quantity::Temperature,
                    times,
                    air.iter().map(|a| a.temperature_k).collect(),
                ),
            ],
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// The offset and share the module's rule gives on [`day_log`], worked here from the day's
    /// air at the field: the pad's pressure altitude, the standard's temperature there and at
    /// the read apogee's height above sea level.
    fn expected(offset_k: f64, apogee_m: f64) -> (f64, f64) {
        let day = Ussa76::with_offset(offset_k, 101_325.0).unwrap();
        let pad = day.sample(FIELD_M).unwrap().air;
        let pad_msl = Ussa76::standard()
            .pressure_altitude_m(pad.pressure_pa)
            .unwrap();
        let offset = pad.temperature_k - (288.15 - 0.0065 * pad_msl);
        let top = 288.15 - 0.0065 * (pad_msl + apogee_m);
        (offset, offset.abs() / top)
    }

    fn uncovered(readings: &Readings) -> &DayAir {
        let air = readings.day_air.as_ref().expect("the day's air");
        assert!(!air.covered, "{air:?}");
        air
    }

    /// A pad 30 K over standard at a 1,190 m field (#1259): the note fires, says the true values
    /// lie above the readings, and its share is `|ΔT| / T_std` at the apogee's height above sea
    /// level, worked here from the day's air. The truth lies past the reading plus its bound, as
    /// the issue found, and within the reading plus the note's share.
    #[test]
    fn a_hot_day_past_the_allowance_is_noted_with_its_size() {
        let readings = read(&day_log(30.0));
        let apogee = *readings.apogee.value().expect("an apogee");
        let air = uncovered(&readings);
        assert!(air.is_hot());
        assert_eq!(air.pad_height_from, PadHeightFrom::Pressure);
        assert_eq!(air.temperature_column, "Temp (K)");
        let (offset, share) = expected(30.0, apogee.height_m);
        assert!(
            (air.offset_k - offset).abs() < 1e-9,
            "{air:?} against {offset}"
        );
        assert!(offset > 25.0, "{offset}");
        assert!((air.share - share).abs() < 1e-12, "{air:?} against {share}");
        assert!(air.share > TEMPERATURE_SHARE, "{air:?}");
        // The issue's finding: the truth lies past the bound.
        let truth = true_top_m();
        assert!(
            truth > apogee.height_m + apogee.bound_m,
            "{truth} {apogee:?}"
        );
        // And within the note's share above the reading.
        assert!(
            truth <= apogee.height_m * (1.0 + air.share),
            "{truth} {} {}",
            apogee.height_m,
            air.share
        );
        // The main's steady rate likewise, read from its opening to landing.
        let Reading::Read(main) = &readings.main_descent else {
            panic!("{:?}", readings.main_descent);
        };
        assert!(main.rate_m_s < MAIN_M_S, "{main:?}");
        assert!(MAIN_M_S <= main.rate_m_s * (1.0 + air.share), "{main:?}");
    }

    /// A pad 30 K under standard: the note fires the other way, the true values below the
    /// readings, by the same rule.
    #[test]
    fn a_cold_day_past_the_allowance_is_noted_the_other_way() {
        let readings = read(&day_log(-30.0));
        let apogee = *readings.apogee.value().expect("an apogee");
        let air = uncovered(&readings);
        assert!(!air.is_hot());
        let (offset, share) = expected(-30.0, apogee.height_m);
        assert!(offset < -25.0, "{offset}");
        assert!(
            (air.offset_k - offset).abs() < 1e-9,
            "{air:?} against {offset}"
        );
        assert!((air.share - share).abs() < 1e-12, "{air:?} against {share}");
        let truth = true_top_m();
        assert!(
            truth < apogee.height_m - apogee.bound_m,
            "{truth} {apogee:?}"
        );
        assert!(
            truth >= apogee.height_m * (1.0 - air.share),
            "{truth} {} {}",
            apogee.height_m,
            air.share
        );
    }

    /// A day 10 K over standard is within the allowance: the day's air is read, and covered.
    #[test]
    fn a_day_within_the_allowance_is_covered() {
        let readings = read(&day_log(10.0));
        let air = readings.day_air.as_ref().expect("the day's air");
        assert!(air.covered, "{air:?}");
        assert!(air.offset_k > 5.0 && air.offset_k < 10.0, "{air:?}");
    }

    /// A temperature with no pressure, no altitude above sea level and no stated ground
    /// elevation has nothing to compare it with: no day's air, and the readings as before.
    #[test]
    fn a_temperature_with_no_height_above_sea_level_gives_none() {
        let mut log = day_log(30.0);
        let pressure = log.channels.remove(0);
        let air = Ussa76::standard();
        let pad = air.pressure_altitude_m(pressure.values[0]).unwrap();
        log.channels.insert(
            0,
            Channel::new(
                "Altitude (m)",
                Quantity::AltitudeAboveGround,
                pressure.time_s.clone(),
                pressure
                    .values
                    .iter()
                    .map(|p| air.pressure_altitude_m(*p).unwrap() - pad)
                    .collect(),
            ),
        );
        let readings = read(&log);
        assert!(readings.apogee.value().is_some(), "{:?}", readings.apogee);
        assert_eq!(readings.day_air, None);
        // With the logger's stated ground elevation, it is read again.
        log.stated.ground_elevation_msl_m = Some(FIELD_M);
        let air = read(&log).day_air.expect("the day's air");
        assert_eq!(air.pad_height_from, PadHeightFrom::StatedGroundElevation);
        assert!(!air.covered);
    }

    /// A temperature channel that can't be the air's, such as zeros from a logger with no sensor,
    /// or no temperature at all, gives no day's air.
    #[test]
    fn no_temperature_or_one_that_isnt_air_gives_none() {
        let mut log = day_log(0.0);
        log.channels[1].values.fill(0.0);
        assert_eq!(read(&log).day_air, None);
        log.channels.truncate(1);
        let readings = read(&log);
        assert!(readings.apogee.value().is_some());
        assert_eq!(readings.day_air, None);
    }

    /// The allowance is strict at its edge: an offset of exactly 15 K is covered, and one a
    /// hundredth of a kelvin past it, either way, isn't. On the record's own temperature and a
    /// stated ground elevation.
    #[test]
    fn the_allowance_holds_to_its_edge() {
        let ground = 600.0;
        let standard = standard_temperature_k(ground);
        for (offset, covered) in [
            (AIR_TEMPERATURE_SPREAD_K, true),
            (-AIR_TEMPERATURE_SPREAD_K, true),
            (AIR_TEMPERATURE_SPREAD_K + 0.01, false),
            (-AIR_TEMPERATURE_SPREAD_K - 0.01, false),
        ] {
            let channel = &day_log(0.0).channels[0];
            let mut log = day_log(0.0);
            log.channels.clear();
            log.time_s.clone_from(&channel.time_s);
            log.altitude_m = log.time_s.iter().map(|t| flight(*t)).collect();
            log.temperature_k = Some(vec![standard + offset; log.time_s.len()]);
            log.stated.ground_elevation_msl_m = Some(ground);
            let air = read(&log).day_air.expect("the day's air");
            assert_eq!(air.temperature_column, "temperature");
            assert_eq!(air.offset_k.abs() <= AIR_TEMPERATURE_SPREAD_K, covered);
            assert_eq!(air.covered, covered, "{offset}: {air:?}");
        }
    }

    /// The standard atmosphere's temperature: 288.15 K at sea level, 6.5 K/km less up to 11 km,
    /// then 216.65 K.
    #[test]
    fn the_standard_temperature_follows_the_1976_table() {
        assert_eq!(standard_temperature_k(0.0), 288.15);
        assert!((standard_temperature_k(1_000.0) - 281.65).abs() < 1e-12);
        assert!((standard_temperature_k(11_000.0) - 216.65).abs() < 1e-12);
        assert_eq!(standard_temperature_k(15_000.0), COLDEST_STANDARD_K);
        let air = Ussa76::standard();
        for height in [0.0, 1_190.0, 5_000.0, 10_000.0] {
            let table = air.sample(height).unwrap().air.temperature_k;
            // `sample` takes a geometric height; the two differ by under 0.2% below 11 km.
            assert!(
                (standard_temperature_k(height) - table).abs() < 0.11,
                "{height}"
            );
        }
    }
}
