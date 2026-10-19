//! The loggers of the synthetic flights: six profiles, each like a family HPR Sim's readers read,
//! and the log one writes of a flight's truth.
//!
//! Rates are the makers' (`docs/research/altimeter-smoothing-methods.md` §2); parts, noise, ranges
//! and steps are their datasheets' (`altimeter-sensor-noise.md` §2, §4–6); a logger turns pressure
//! into height by the standard atmosphere and zeroes it on the pad
//! (`altimeter-height-formulas.md`). Every range is
//! [ADR-280, the synthetic flights first](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md) §3's.

use hpr_core::DVec3;
use hpr_core::earth::Earth;
use hpr_core::geodesy::Geodetic;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2 as G;
use hpr_core::random::SeededRng;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use serde::{Deserialize, Serialize};

use super::noise::{self, Lag, between, ornstein_uhlenbeck, quantize, tau_for};
use super::truth::{SITE_LATITUDE_DEG, SITE_LONGITUDE_DEG, TRUTH_STEP_S, Truth, TruthSample};
use super::{Key, SyntheticError, stream};

/// The foot, m.
const FOOT_M: f64 = 0.3048;
/// One atmosphere, Pa.
const ATMOSPHERE_PA: f64 = 101_325.0;

/// A logger profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    /// Like a PerfectFlite StratoLoggerCF: 20 Hz, its altitude in whole feet.
    PerfectFlite,
    /// Like a Missile Works RRC3: 20 Hz, an MS5607's pressure in 10 Pa steps.
    Rrc3,
    /// Like an Eggtimer Quantum: 20 Hz, a Bosch barometer's altitude in whole feet.
    Eggtimer,
    /// Like an Altus Metrum TeleMetrum: 100 Hz up and 10 Hz down, an MS5607's pressure, a high-g
    /// accelerometer and a GNSS receiver.
    Altos,
    /// Like a Featherweight Blue Raven: 500 Hz, a Bosch barometer and a low-g accelerometer.
    Imu500,
    /// A GNSS tracker: position and altitude only.
    GnssTracker,
}

impl Profile {
    /// Every profile, each equally likely in a draw.
    pub const ALL: [Self; 6] = [
        Self::PerfectFlite,
        Self::Rrc3,
        Self::Eggtimer,
        Self::Altos,
        Self::Imu500,
        Self::GnssTracker,
    ];

    /// Its name in the census.
    pub fn name(self) -> &'static str {
        match self {
            Self::PerfectFlite => "perfectflite",
            Self::Rrc3 => "rrc3",
            Self::Eggtimer => "eggtimer",
            Self::Altos => "altos",
            Self::Imu500 => "imu-500hz",
            Self::GnssTracker => "gnss-tracker",
        }
    }

    /// The format of the family it is like, which its logs are tagged with.
    pub fn format(self) -> LogFormat {
        match self {
            Self::PerfectFlite => LogFormat::PerfectFlitePf2,
            Self::Rrc3 => LogFormat::Rrc3Csv,
            Self::Eggtimer => LogFormat::EggtimerCsv,
            Self::Altos => LogFormat::AltosCsv,
            Self::Imu500 => LogFormat::BlueRavenCsv,
            Self::GnssTracker => LogFormat::FeatherweightGpsCsv,
        }
    }

    /// How its log is written: its rates and its barometer's step, `None` for a profile with no
    /// barometer.
    pub fn recording(self) -> Option<Recording> {
        self.barometer().map(|baro| Recording {
            climbing_hz: self.rate_hz(true),
            descending_hz: self.rate_hz(false),
            written: baro.written,
        })
    }

    /// The barometer's sample rate, Hz, climbing or not: PerfectFlite 20 Hz ([PF1] p. 47),
    /// RRC3 20 Hz ([MW1]), Eggtimer 20 Hz ([E1] p. 25), AltOS 100 Hz up and 10 down ([A1]
    /// p. 77), Blue Raven's 500 Hz set ([F2]).
    fn rate_hz(self, climbing: bool) -> f64 {
        match self {
            Self::PerfectFlite | Self::Rrc3 | Self::Eggtimer => 20.0,
            Self::Altos if climbing => 100.0,
            Self::Altos => 10.0,
            Self::Imu500 => 500.0,
            Self::GnssTracker => 0.0,
        }
    }

    /// The pressure sensor.
    fn barometer(self) -> Option<Barometer> {
        match self {
            // PerfectFlite names no part: the noise measured at rest in public logs, the
            // MS5607's offset (sensor noise §3, §6), and its stated ±0.1% of the height, read as
            // a gain on the pressure's change (§1, "Steps and accuracy").
            Self::PerfectFlite => Some(Barometer {
                white_pa: (0.0, 0.0),
                offset_pa: 100.0,
                relative_gain: 1.0e-3,
                written: Written::Altitude { step_m: FOOT_M },
            }),
            // An MS5607 at 2.3 Pa, logged in 10 Pa steps (sensor noise §6). Its ±1.0 mbar after
            // the pad's zero, over the 400 mbar the datasheet states it on (700–1100 mbar, §2
            // note †), is read as a gain of ±0.25% on the change from the pad (an inference).
            Self::Rrc3 => Some(Barometer {
                white_pa: (2.3, 2.3),
                offset_pa: 100.0,
                relative_gain: MS5607_RELATIVE_GAIN,
                written: Written::Pressure { step_pa: 10.0 },
            }),
            // An MS5607 at 2.4–13 Pa by its oversampling, logged to 1 Pa, its error after the
            // pad's zero as the RRC3's.
            Self::Altos => Some(Barometer {
                white_pa: (2.4, 13.0),
                offset_pa: 100.0,
                relative_gain: MS5607_RELATIVE_GAIN,
                written: Written::Pressure { step_pa: 1.0 },
            }),
            // A Bosch BMP-class part, 0.2–6.6 Pa by its setting, relative accuracy 0.03–0.08 hPa
            // per 100 hPa (sensor noise §2, §6).
            Self::Eggtimer => Some(Barometer {
                white_pa: (0.2, 6.6),
                offset_pa: 0.0,
                relative_gain: 8.0e-4,
                written: Written::Altitude { step_m: FOOT_M },
            }),
            // The same part, logged in Blue Raven's 2 × 10⁻⁵ atm steps (its user's guide, p. 23).
            Self::Imu500 => Some(Barometer {
                white_pa: (0.2, 6.6),
                offset_pa: 0.0,
                relative_gain: 8.0e-4,
                written: Written::Pressure {
                    step_pa: 2.0e-5 * ATMOSPHERE_PA,
                },
            }),
            Self::GnssTracker => None,
        }
    }

    /// The accelerometer.
    fn accelerometer(self) -> Option<Accelerometer> {
        match self {
            // ADXL375: 5 mg/√Hz, 49 mg steps, its minimum range ±180 g, scale ±10% ([D1] p. 3).
            Self::Altos => Some(Accelerometer {
                density_g_per_root_hz: (0.005, 0.005),
                step_g: 0.049,
                range_g: 180.0,
                lower_range_g: None,
                scale: 0.10,
            }),
            // A low-g IMU axis: 160–400 µg/√Hz ([D3], [D9]), ±32 g as Blue Raven's parts ([F3]
            // p. 24) on 16 bits, scale ±3% ([D9]); past 32 g it clips, as the high-range part
            // that takes over isn't modeled. Such a part offers ±8 g too, as the BMX160 ([D8]
            // p. 8) and the MPU-6000 ([D9] p. 13) do, on the same 16 bits.
            Self::Imu500 => Some(Accelerometer {
                density_g_per_root_hz: (0.000_16, 0.000_4),
                step_g: 64.0 / 65_536.0,
                range_g: 32.0,
                lower_range_g: Some(LOWER_RANGE_G),
                scale: 0.03,
            }),
            _ => None,
        }
    }

    /// The range its accelerometer clips at, m/s²; `None` with no accelerometer.
    pub fn accelerometer_range_m_s2(self) -> Option<f64> {
        self.accelerometer().map(|accel| accel.range_g * G)
    }

    /// The lower range its accelerometer can be set to, m/s², which
    /// [`accelerometer_log_set_lower`] logs it at; `None` with no accelerometer or no such
    /// setting.
    pub fn accelerometer_lower_range_m_s2(self) -> Option<f64> {
        self.accelerometer()
            .and_then(|accel| accel.lower_range_g)
            .map(|range_g| range_g * G)
    }

    /// Whether it has a GNSS receiver.
    fn gnss(self) -> bool {
        matches!(self, Self::Altos | Self::GnssTracker)
    }
}

/// How a log is written: the barometer's rates and how its reading is written. A profile's own
/// is [`Profile::recording`]; the readings' check ([`super::held`]) writes each tuning flight
/// again at other rates and steps.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Recording {
    /// Samples a second while climbing, to the first sample past apogee.
    pub climbing_hz: f64,
    /// Samples a second after it.
    pub descending_hz: f64,
    /// How the barometer's reading is written.
    pub written: Written,
}

/// How a barometer's reading is written.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Written {
    /// As the logger's altitude in steps of `step_m`, none if 0: the record's primary track, as
    /// a PerfectFlite writes whole feet.
    Altitude {
        /// The step, m.
        step_m: f64,
    },
    /// As a pressure in steps of `step_pa`, kept in a channel, its altitude worked out beside it
    /// as the primary track.
    Pressure {
        /// The step, Pa.
        step_pa: f64,
    },
    /// As a pressure in steps of `step_pa` alone, in a channel, as a raw download keeps it.
    PressureAlone {
        /// The step, Pa.
        step_pa: f64,
    },
}

/// A pressure sensor's stated terms.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Barometer {
    /// The range of its white noise, 1σ, Pa.
    white_pa: (f64, f64),
    /// Its per-flight offset is drawn from ±this, Pa.
    offset_pa: f64,
    /// Its per-flight gain error on the pressure's change from the pad is drawn from ±this.
    relative_gain: f64,
    /// How it is written.
    written: Written,
}

/// An accelerometer's stated terms.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Accelerometer {
    /// The range of its noise density, g/√Hz.
    density_g_per_root_hz: (f64, f64),
    /// Its step, g.
    step_g: f64,
    /// It clips at ±this, g.
    range_g: f64,
    /// A lower full scale the part can be set to, g, on the same bits; `None` for a part with
    /// none.
    lower_range_g: Option<f64>,
    /// Its per-flight scale error is drawn from ±this.
    scale: f64,
}

/// The lower full scale a low-g IMU axis is set to in [`accelerometer_log_set_lower`], g: ±8 g,
/// as the BMX160 and the MPU-6000 offer, which most of the IMU-like logger's tuning flights' tops
/// pass and the rest don't, as the synthetic flights' report counts. The readings know it by its
/// column,
/// [`hpr_flightdata::readings::SYNTHETIC_LOWER_RANGE_COLUMN`], whose family states it.
pub const LOWER_RANGE_G: f64 = 8.0;

/// The MS5607's error after a zero at one pressure, ±1.0 mbar over 0–50 °C, read over the
/// 400 mbar (700–1100 mbar) its datasheet states the figure on as a gain on the pressure's change
/// from the pad (sensor noise §2 note †, §6; an inference).
pub const MS5607_RELATIVE_GAIN: f64 = 100.0 / 40_000.0;
/// The fast correlated barometer noise: 0.5 m at 50 Hz, lag-one correlation 0.5, as on the pads of
/// five Blue Raven logs (sensor noise §3, §6).
pub const CORRELATED_NOISE_M: f64 = 0.5;
/// The correlated noise's lag-one correlation at [`CORRELATED_NOISE_RATE_HZ`].
pub const CORRELATED_NOISE_PHI: f64 = 0.5;
/// The rate at which it was measured, Hz.
pub const CORRELATED_NOISE_RATE_HZ: f64 = 50.0;
/// The slow correlated barometer noise, a first-order Gauss–Markov term of 0.24–0.29 m and time
/// constant 0.65–1.26 s, as a BMP085's ten minutes at rest gave (sensor noise §3; Sabatini and
/// Genovese, Sensors 13(11):15692, 2013, Tables 1–2): the noise that a reading averaged over a
/// second keeps, which the fast term alone leaves out.
pub const SLOW_NOISE_M: (f64, f64) = (0.24, 0.29);
/// Its time constant's range, s.
pub const SLOW_NOISE_TAU_S: (f64, f64) = (0.65, 1.26);
/// The residual accelerometer bias after the pad's zero is drawn from ±this, g: the top of the
/// ±10–50 mg that sensor noise's *What this suggests* 1 calls a guess, as no source states one.
pub const ACCEL_BIAS_G: f64 = 0.05;
/// A GNSS fix's noise, 1σ along each axis, m: the filter's σ in GNSS in boost §4's simulation,
/// which was fed noise-free fixes, taken as a receiver's noise (an assumption).
pub const GNSS_NOISE_M: f64 = 3.0;

/// What one flight's logger draws.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LoggerDraw {
    /// The profile.
    pub profile: Profile,
    /// Seconds logged on the pad before launch.
    pub pad_s: f64,
    /// Seconds logged on the ground after landing.
    pub ground_s: f64,
    /// The barometer's white noise, 1σ, Pa.
    pub white_pa: f64,
    /// Its offset, Pa.
    pub offset_pa: f64,
    /// Its gain error on the pressure's change from the pad.
    pub gain: f64,
    /// Its drift, Pa/s.
    pub drift_pa_s: f64,
    /// Its port's distance behind the nose's shoulder, calibers.
    pub port_calibers: f64,
    /// The ejection pulses' scale on the Pnut's.
    pub pulse_scale: f64,
    /// The accelerometer's noise density, g/√Hz.
    pub density_g_per_root_hz: f64,
    /// Its scale error.
    pub accel_scale: f64,
    /// Its residual bias, m/s².
    pub accel_bias_m_s2: f64,
    /// The barometer's fast correlated noise, 1σ at the pad, m.
    pub fast_noise_m: f64,
    /// Its slow correlated noise, 1σ at the pad, m.
    pub slow_noise_m: f64,
    /// The slow noise's time constant, s.
    pub slow_noise_tau_s: f64,
    /// The GNSS receiver's fix rate, Hz.
    pub gnss_rate_hz: f64,
    /// Its loop's natural frequency, rad/s.
    pub gnss_omega_rad_s: f64,
    /// Its dropout from liftoff, s.
    pub gnss_dropout_liftoff_s: f64,
    /// Its dropout from burnout, s.
    pub gnss_dropout_burnout_s: f64,
}

impl LoggerDraw {
    /// Draws a profile's terms from their stated ranges.
    pub fn draw(rng: &mut SeededRng, profile: Profile) -> Self {
        let baro = profile.barometer();
        let accel = profile.accelerometer();
        let sign = |rng: &mut SeededRng| if rng.uniform() < 0.5 { -1.0 } else { 1.0 };
        let pad_s = between(rng, 2.0, 10.0);
        let ground_s = between(rng, 2.0, 10.0);
        let white_pa = baro.map_or(0.0, |b| between(rng, b.white_pa.0, b.white_pa.1));
        let offset_pa = baro.map_or(0.0, |b| between(rng, -b.offset_pa, b.offset_pa));
        let gain = baro.map_or(0.0, |b| between(rng, -b.relative_gain, b.relative_gain));
        // 1–3 Pa over 30–300 s, either way (sensor noise §3).
        let drift_pa_s = sign(rng) * between(rng, 1.0, 3.0) / between(rng, 30.0, 300.0);
        let port_calibers = between(rng, 1.0, 5.0);
        let pulse_scale = between(rng, 0.0, 2.0);
        let density = accel.map_or(0.0, |a| {
            between(rng, a.density_g_per_root_hz.0, a.density_g_per_root_hz.1)
        });
        let accel_scale = accel.map_or(0.0, |a| between(rng, -a.scale, a.scale));
        let accel_bias_m_s2 = between(rng, -ACCEL_BIAS_G, ACCEL_BIAS_G) * G;
        // 1 Hz by default, faster where set ([GNSS in boost] §1).
        let gnss_rate_hz = [1.0, 5.0, 10.0][(rng.uniform() * 3.0) as usize % 3];
        // About the simulated filter's 0.47–0.69 rad/s (GNSS in boost §4), widened both ways: an
        // assumption.
        let gnss_omega_rad_s = between(rng, 0.25, 1.0);
        let gnss_dropout_liftoff_s = between(rng, 0.0, 10.0);
        let gnss_dropout_burnout_s = between(rng, 0.0, 10.0);
        let fast_noise_m = baro.map_or(0.0, |_| CORRELATED_NOISE_M);
        let slow_noise_m = baro.map_or(0.0, |_| between(rng, SLOW_NOISE_M.0, SLOW_NOISE_M.1));
        let slow_noise_tau_s = between(rng, SLOW_NOISE_TAU_S.0, SLOW_NOISE_TAU_S.1);
        Self {
            profile,
            pad_s,
            ground_s,
            white_pa,
            offset_pa,
            gain,
            drift_pa_s,
            port_calibers,
            pulse_scale,
            density_g_per_root_hz: density,
            accel_scale,
            accel_bias_m_s2,
            fast_noise_m,
            slow_noise_m,
            slow_noise_tau_s,
            gnss_rate_hz,
            gnss_omega_rad_s,
            gnss_dropout_liftoff_s,
            gnss_dropout_burnout_s,
        }
    }
}

/// The logger's clock, s from its own zero, `pad_s` before launch to `ground_s` after landing.
fn clock(recording: &Recording, draw: &LoggerDraw, truth: &Truth) -> Vec<f64> {
    let end = draw.pad_s + truth.events.landing_s + draw.ground_s;
    let apogee = draw.pad_s + truth.events.apogee_s;
    let mut times = Vec::new();
    let mut t = 0.0;
    let mut k = 0_u64;
    let mut base = 0.0;
    let mut rate = recording.climbing_hz;
    while t <= end {
        times.push(t);
        k += 1;
        let next = base + k as f64 / rate;
        if next > apogee && rate != recording.descending_hz {
            // The rate changes at the first sample past apogee.
            rate = recording.descending_hz;
            base = next;
            k = 0;
            t = next;
        } else {
            t = next;
        }
    }
    times
}

/// A logger's height of `pressure_pa`, m: the standard atmosphere's pressure altitude, except the
/// RRC3's NOAA formula, h = 145,366.45 ft × (1 − (P/1013.25 hPa)^0.190284) (MW1 p. 23).
fn height_of(profile: Profile, pressure_pa: f64) -> Result<f64, SyntheticError> {
    Ok(match profile {
        Profile::Rrc3 => 145_366.45 * FOOT_M * (1.0 - (pressure_pa / 101_325.0).powf(0.190_284)),
        _ => hpr_atmos::Ussa76::standard().pressure_altitude_m(pressure_pa)?,
    })
}

/// Refuses a draw or a truth no logger could write: a clock that doesn't run forward, a truth
/// with no samples or a time that isn't finite.
fn check(draw: &LoggerDraw, truth: &Truth) -> Result<(), SyntheticError> {
    let invalid = |what| Err(SyntheticError::Invalid { what });
    let spans = [draw.pad_s, draw.ground_s];
    if spans.iter().any(|s| !s.is_finite() || *s < 0.0) {
        return invalid("a logger's pad or ground span that isn't a finite length");
    }
    if draw.profile.gnss() && !(draw.gnss_rate_hz.is_finite() && draw.gnss_rate_hz > 0.0) {
        return invalid("a GNSS fix rate that isn't a positive number");
    }
    if !(draw.slow_noise_tau_s.is_finite() && draw.slow_noise_tau_s > 0.0) {
        return invalid("a slow noise time constant that isn't a positive number");
    }
    let events = &truth.events;
    let times = [
        events.liftoff_s,
        events.burnout_s,
        events.apogee_s,
        events.landing_s,
    ];
    if truth.samples.is_empty() || times.iter().any(|t| !t.is_finite()) {
        return invalid("a truth with no samples or an event at no finite time");
    }
    if truth.samples.iter().any(|s| !s.time_s.is_finite()) {
        return invalid("a truth sample at no finite time");
    }
    // The clock writes a sample for every step it runs, so a log longer than a day, which no
    // flight lasts, is refused rather than allocated.
    let length_s = draw.pad_s + events.landing_s + draw.ground_s;
    if length_s.is_nan() || length_s > MAX_LOG_S {
        return invalid("a log longer than a day");
    }
    Ok(())
}

/// The longest log a logger writes, s: a day.
const MAX_LOG_S: f64 = 86_400.0;

/// The truth at `t`, refused where there is none.
fn truth_at(truth: &Truth, t: f64) -> Result<TruthSample, SyntheticError> {
    truth.at(t).ok_or(SyntheticError::Invalid {
        what: "a truth with no sample at that time",
    })
}

/// The log `draw`'s logger writes of `truth`, with noise drawn from `seed`'s streams for flight
/// `index`.
///
/// # Errors
///
/// [`SyntheticError`] for a pressure or a position the atmosphere or the frame can't take, and
/// [`SyntheticError::Invalid`] for a draw or truth that `check` refuses.
pub fn log(
    draw: &LoggerDraw,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<FlightLog, SyntheticError> {
    let recording = draw.profile.recording();
    write(draw, recording.as_ref(), true, truth, seed, index)
}

/// The log `draw`'s barometer would write of `truth` at `recording`'s rates and step, with the
/// same noise terms and streams, and nothing else: no accelerometer, no GNSS receiver. Tagged as a
/// table read by a mapping ([`LogFormat::GenericTable`]), so that a reading takes the track's
/// step from the track itself. `None` for a profile with no barometer.
///
/// # Errors
///
/// As [`log`].
pub fn barometer_log(
    draw: &LoggerDraw,
    recording: &Recording,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<Option<FlightLog>, SyntheticError> {
    if draw.profile.barometer().is_none() {
        return Ok(None);
    }
    let mut log = write(draw, Some(recording), false, truth, seed, index)?;
    log.format = LogFormat::GenericTable;
    Ok(Some(log))
}

/// The accelerometer's channel at `times`, the part sampled at `rate_hz` while climbing, under
/// the synthetic flights' column.
fn accelerometer_channel(
    draw: &LoggerDraw,
    accel: &Accelerometer,
    rate_hz: f64,
    times: &[f64],
    truth: &Truth,
    (seed, index): (u64, u64),
) -> Result<Channel, SyntheticError> {
    accelerometer_channel_under(
        hpr_flightdata::readings::SYNTHETIC_COLUMN,
        draw,
        accel,
        rate_hz,
        times,
        truth,
        (seed, index),
    )
}

/// The accelerometer's channel under `column`, as [`accelerometer_channel`].
fn accelerometer_channel_under(
    column: &str,
    draw: &LoggerDraw,
    accel: &Accelerometer,
    rate_hz: f64,
    times: &[f64],
    truth: &Truth,
    (seed, index): (u64, u64),
) -> Result<Channel, SyntheticError> {
    let accel_noise = noise::white(&mut stream(seed, index, Key::Accel), times.len(), 1.0);
    // RMS from the density over the part's bandwidth, half the climbing rate it is sampled at:
    // σ = density × √(1.57 × bandwidth) (sensor noise §4). A logger that writes fewer samples
    // after apogee keeps the part's bandwidth, so its noise.
    let bandwidth_hz = rate_hz / 2.0;
    let sigma = draw.density_g_per_root_hz * (1.57 * bandwidth_hz).sqrt() * G;
    let range = accel.range_g * G;
    let values = times
        .iter()
        .enumerate()
        .map(|(k, &t_log)| {
            let f = truth_at(truth, t_log - draw.pad_s)?.axial_specific_force_m_s2;
            let read = (1.0 + draw.accel_scale) * f + draw.accel_bias_m_s2 + sigma * accel_noise[k];
            Ok(quantize(read.clamp(-range, range), accel.step_g * G))
        })
        .collect::<Result<Vec<_>, SyntheticError>>()?;
    Ok(Channel::new(
        column,
        Quantity::Acceleration,
        times.to_vec(),
        values,
    ))
}

/// The log `draw`'s logger writes of `truth`, its accelerometer set to the lower full scale its
/// part offers ([`Profile::accelerometer_lower_range_m_s2`]): the same part, noise terms and
/// streams on the same clock, clipping at the lower range, as the BMX160 holds its maximum code
/// past it (sensor noise §5, its datasheet p. 32), in steps of the part's 16 bits over it, under
/// [`hpr_flightdata::readings::SYNTHETIC_LOWER_RANGE_COLUMN`]. `None` for a part with no such
/// setting.
///
/// # Errors
///
/// As [`log`].
pub fn accelerometer_log_set_lower(
    draw: &LoggerDraw,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<Option<FlightLog>, SyntheticError> {
    let Some((accel, lower_g)) = draw
        .profile
        .accelerometer()
        .and_then(|accel| accel.lower_range_g.map(|lower_g| (accel, lower_g)))
    else {
        return Ok(None);
    };
    let set = Accelerometer {
        range_g: lower_g,
        // The part's bits over the range it is set to: its step over its own range, scaled.
        step_g: accel.step_g * lower_g / accel.range_g,
        ..accel
    };
    let mut log = log(draw, truth, seed, index)?;
    for kept in &mut log.channels {
        if kept.quantity == Quantity::Acceleration {
            *kept = accelerometer_channel_under(
                hpr_flightdata::readings::SYNTHETIC_LOWER_RANGE_COLUMN,
                draw,
                &set,
                draw.profile.rate_hz(true),
                &kept.time_s,
                truth,
                (seed, index),
            )?;
        }
    }
    Ok(Some(log))
}

/// The log `draw`'s logger writes of `truth`, its accelerometer sampled at `rate_hz` on its own
/// clock in place of the barometer's: the same part, noise terms and streams, its noise grown
/// with the rate's bandwidth. `None` for a profile with no accelerometer.
///
/// # Errors
///
/// As [`log`].
pub fn accelerometer_log(
    draw: &LoggerDraw,
    rate_hz: f64,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<Option<FlightLog>, SyntheticError> {
    let Some(accel) = draw.profile.accelerometer() else {
        return Ok(None);
    };
    let mut log = log(draw, truth, seed, index)?;
    let times = clock(
        &Recording {
            climbing_hz: rate_hz,
            descending_hz: rate_hz,
            written: Written::Altitude { step_m: 0.0 },
        },
        draw,
        truth,
    );
    let channel = accelerometer_channel(draw, &accel, rate_hz, &times, truth, (seed, index))?;
    for kept in &mut log.channels {
        if kept.quantity == Quantity::Acceleration {
            *kept = channel.clone();
        }
    }
    Ok(Some(log))
}

/// The log of `truth` its barometer writes at `recording`, and, `with_others`, its
/// accelerometer and GNSS receiver as the profile has them.
fn write(
    draw: &LoggerDraw,
    recording: Option<&Recording>,
    with_others: bool,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<FlightLog, SyntheticError> {
    check(draw, truth)?;
    let profile = draw.profile;
    let mut channels = Vec::new();
    let mut time_s = Vec::new();
    let mut altitude_m = Vec::new();
    if let (Some(_), Some(recording)) = (profile.barometer(), recording) {
        let written = recording.written;
        let times = clock(recording, draw, truth);
        let sim = |t: f64| t - draw.pad_s;
        let air = truth.air()?;
        let pad_pa = truth.pressure_pa(0.0)?;
        let pad_density = air.sample(truth.site_elevation_m)?.air.density_kg_m3;
        let tau = tau_for(CORRELATED_NOISE_PHI, CORRELATED_NOISE_RATE_HZ);
        let fast = ornstein_uhlenbeck(
            &mut stream(seed, index, Key::BaroCorrelated),
            &times,
            draw.fast_noise_m * pad_density * G,
            tau,
        );
        let slow = ornstein_uhlenbeck(
            &mut stream(seed, index, Key::BaroSlow),
            &times,
            draw.slow_noise_m * pad_density * G,
            draw.slow_noise_tau_s,
        );
        let white = noise::white(
            &mut stream(seed, index, Key::BaroWhite),
            times.len(),
            draw.white_pa,
        );
        let mut pressures = Vec::with_capacity(times.len());
        for (k, &t_log) in times.iter().enumerate() {
            let t = sim(t_log);
            let state = truth_at(truth, t)?;
            let h = truth.site_elevation_m + state.height_m;
            let sample = air.sample(h)?;
            let p = sample.air.pressure_pa;
            let pulse_m: f64 = truth
                .events
                .charges
                .iter()
                .map(|(_, at)| noise::pulse_m(t - at, draw.pulse_scale))
                .sum();
            let measured = p
                + draw.offset_pa
                + draw.gain * (p - pad_pa)
                + draw.drift_pa_s * t_log
                + fast[k]
                + slow[k]
                + white[k]
                + noise::port_error_pa(draw.port_calibers, state.mach, state.dynamic_pressure_pa)
                - sample.air.density_kg_m3 * G * pulse_m;
            pressures.push(match written {
                Written::Pressure { step_pa } | Written::PressureAlone { step_pa } => {
                    quantize(measured, step_pa)
                }
                Written::Altitude { .. } => measured,
            });
        }
        let heights = pressures
            .iter()
            .map(|&p| height_of(profile, p))
            .collect::<Result<Vec<_>, _>>()?;
        // The pad's zero: the mean of the samples before launch.
        let on_pad: Vec<f64> = times
            .iter()
            .zip(&heights)
            .filter(|(t, _)| sim(**t) < 0.0)
            .map(|(_, h)| *h)
            .collect();
        let zero = on_pad.iter().sum::<f64>() / on_pad.len().max(1) as f64;
        let alone = matches!(written, Written::PressureAlone { .. });
        if !alone {
            altitude_m = heights
                .iter()
                .map(|h| match written {
                    Written::Altitude { step_m } => quantize(h - zero, step_m),
                    Written::Pressure { .. } | Written::PressureAlone { .. } => h - zero,
                })
                .collect();
        }
        if let Written::Pressure { .. } | Written::PressureAlone { .. } = written {
            channels.push(Channel::new(
                "Pressure (synthetic)",
                Quantity::Pressure,
                times.clone(),
                pressures,
            ));
        }
        if let Some(accel) = profile.accelerometer().filter(|_| with_others) {
            channels.push(accelerometer_channel(
                draw,
                &accel,
                profile.rate_hz(true),
                &times,
                truth,
                (seed, index),
            )?);
        }
        if !alone {
            time_s = times;
        }
    }
    if profile.gnss() && with_others {
        channels.extend(gnss(draw, truth, seed, index)?);
    }
    Ok(FlightLog {
        format: profile.format(),
        logger: format!("HPR Sim synthetic flight, {} profile", profile.name()),
        serial_number: None,
        firmware: None,
        flight_number: Some(u32::try_from(index).unwrap_or(u32::MAX)),
        stated: Stated::default(),
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes: vec![
            "A synthetic flight (hpr_forensics::synthetic, ADR-280): HPR Sim's flight with noise drawn \
             as a logger of this family would add it, not a logged flight."
                .to_owned(),
        ],
    })
}

/// The GNSS receiver's channels: latitude, longitude and altitude, each fix through the loop's
/// lag and noise, none in the dropouts.
fn gnss(
    draw: &LoggerDraw,
    truth: &Truth,
    seed: u64,
    index: u64,
) -> Result<Vec<Channel>, SyntheticError> {
    let site = Geodetic::from_degrees(
        SITE_LATITUDE_DEG,
        SITE_LONGITUDE_DEG,
        truth.site_elevation_m,
    )?;
    let earth = Earth::wgs84(site)?;
    let mut rng = stream(seed, index, Key::Gnss);
    let end = draw.pad_s + truth.events.landing_s + draw.ground_s;
    let start = truth_at(truth, 0.0)?;
    let mut lags: Vec<Lag> = (0..3)
        .map(|axis| {
            let mut lag = Lag::new(draw.gnss_omega_rad_s, start.position_enu_m[axis]);
            lag.step(
                0.0,
                start.position_enu_m[axis],
                start.velocity_enu_m_s[axis],
            );
            lag
        })
        .collect();
    let (ignition, burnout) = (truth.events.liftoff_s, truth.events.burnout_s);
    let mut now = 0.0;
    let mut out = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut k = 0_u64;
    loop {
        let t_log = k as f64 / draw.gnss_rate_hz;
        if t_log > end {
            break;
        }
        k += 1;
        let t = t_log - draw.pad_s;
        // The loop runs from launch; before it, the receiver sits on the pad's position.
        while now < t {
            let dt = (t - now).min(TRUTH_STEP_S);
            now += dt;
            let state = truth_at(truth, now)?;
            for (axis, lag) in lags.iter_mut().enumerate() {
                lag.step(dt, state.position_enu_m[axis], state.velocity_enu_m_s[axis]);
            }
        }
        let position: Vec<f64> = lags.iter().map(Lag::output).collect();
        let noisy = DVec3::new(
            position[0] + GNSS_NOISE_M * rng.standard_normal(),
            position[1] + GNSS_NOISE_M * rng.standard_normal(),
            position[2] + GNSS_NOISE_M * rng.standard_normal(),
        );
        let dropped = (ignition..ignition + draw.gnss_dropout_liftoff_s).contains(&t)
            || (burnout..burnout + draw.gnss_dropout_burnout_s).contains(&t);
        if dropped {
            continue;
        }
        let fix = earth.frame().geodetic_from_enu(noisy)?;
        out[0].push(t_log);
        out[1].push(fix.latitude_rad.to_degrees());
        out[2].push(fix.longitude_rad.to_degrees());
        // Mean sea level is the ellipsoid here: the flights' environment sets N = 0.
        out[3].push(fix.height_m);
    }
    let [times, latitude, longitude, altitude] = out;
    Ok(vec![
        Channel::new(
            "Latitude (synthetic)",
            Quantity::Latitude,
            times.clone(),
            latitude,
        ),
        Channel::new(
            "Longitude (synthetic)",
            Quantity::Longitude,
            times.clone(),
            longitude,
        ),
        Channel::new(
            "GNSS altitude (synthetic)",
            Quantity::GnssAltitude,
            times,
            altitude,
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthetic::truth::{TruthEvents, TruthSample};
    use crate::synthetic::{Flyer, Set, draw};

    const SEED: u64 = 9_901;

    /// A truth standing still `seconds` long at 500 m, its axial specific force `f`, Mach number
    /// `mach` and dynamic pressure `q`, accelerating up at `climb` m/s² from launch: a truth for
    /// one noise term at a time.
    fn still(seconds: f64, f: f64, mach: f64, q: f64, climb: f64) -> Truth {
        let samples = (0..=(seconds / TRUTH_STEP_S) as usize)
            .map(|k| {
                let t = k as f64 * TRUTH_STEP_S;
                let z = 0.5 * climb * t * t;
                TruthSample {
                    time_s: t,
                    height_m: if climb > 0.0 { z } else { 0.0 },
                    vertical_speed_m_s: climb * t,
                    axial_specific_force_m_s2: f,
                    mach,
                    dynamic_pressure_pa: q,
                    position_enu_m: [0.0, 0.0, z],
                    velocity_enu_m_s: [0.0, 0.0, climb * t],
                }
            })
            .collect();
        Truth {
            samples,
            events: TruthEvents {
                liftoff_s: 0.0,
                rail_exit_s: 0.0,
                burnout_s: 10.0 * seconds,
                apogee_s: seconds,
                apogee_m: 0.0,
                charges: Vec::new(),
                deployments: Vec::new(),
                landing_s: seconds,
            },
            pad_height_m: 0.0,
            site_elevation_m: 500.0,
            temperature_offset_k: 0.0,
        }
    }

    /// A draw of `profile` with every per-flight term zero and the pad and ground spans short.
    fn quiet(profile: Profile) -> LoggerDraw {
        LoggerDraw {
            profile,
            pad_s: 2.0,
            ground_s: 0.0,
            white_pa: 0.0,
            offset_pa: 0.0,
            gain: 0.0,
            drift_pa_s: 0.0,
            port_calibers: 1.0,
            pulse_scale: 0.0,
            density_g_per_root_hz: 0.0,
            accel_scale: 0.0,
            accel_bias_m_s2: 0.0,
            fast_noise_m: 0.0,
            slow_noise_m: 0.0,
            slow_noise_tau_s: 1.0,
            gnss_rate_hz: 10.0,
            gnss_omega_rad_s: 0.5,
            gnss_dropout_liftoff_s: 0.0,
            gnss_dropout_burnout_s: 0.0,
        }
    }

    fn channel(log: &FlightLog, quantity: Quantity) -> &Channel {
        log.channels_of(quantity).next().unwrap()
    }

    /// Each logged pressure less the truth's, Pa, with each sample's time on the log's clock.
    fn pressure_residuals(log: &FlightLog, truth: &Truth, pad_s: f64) -> Vec<(f64, f64)> {
        let p = channel(log, Quantity::Pressure);
        p.time_s
            .iter()
            .zip(&p.values)
            .map(|(&t, &v)| (t, v - truth.pressure_pa(t - pad_s).unwrap()))
            .collect()
    }

    /// The accelerometer's readings up to `end_s` s after launch, the log's clock starting `pad_s`
    /// before it: the span the truth covers, before the rocket lies still.
    fn acceleration_until(log: &FlightLog, pad_s: f64, end_s: f64) -> Vec<f64> {
        let a = channel(log, Quantity::Acceleration);
        a.time_s
            .iter()
            .zip(&a.values)
            .filter(|(t, _)| **t - pad_s < end_s)
            .map(|(_, v)| *v)
            .collect()
    }

    fn mean(x: &[f64]) -> f64 {
        x.iter().sum::<f64>() / x.len() as f64
    }

    fn sd(x: &[f64]) -> f64 {
        let m = mean(x);
        (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
    }

    /// The residuals' least-squares slope against time, and their mean and spread about it.
    fn line_fit(residuals: &[(f64, f64)]) -> (f64, f64, f64) {
        let t: Vec<f64> = residuals.iter().map(|r| r.0).collect();
        let r: Vec<f64> = residuals.iter().map(|r| r.1).collect();
        let (tm, rm) = (mean(&t), mean(&r));
        let stt: f64 = t.iter().map(|x| (x - tm) * (x - tm)).sum();
        let slope = t
            .iter()
            .zip(&r)
            .map(|(x, y)| (x - tm) * (y - rm))
            .sum::<f64>()
            / stt;
        let detrended: Vec<f64> = t.iter().zip(&r).map(|(x, y)| y - slope * x).collect();
        (slope, mean(&detrended), sd(&detrended))
    }

    /// The lag-`k` autocorrelation of `x`.
    fn autocorrelation(x: &[f64], k: usize) -> f64 {
        let m = mean(x);
        let num: f64 = x.iter().zip(&x[k..]).map(|(a, b)| (a - m) * (b - m)).sum();
        let den: f64 = x.iter().map(|v| (v - m) * (v - m)).sum();
        num / den
    }

    #[test]
    fn a_barometers_offset_drift_and_white_noise_each_hold_alone_and_fail_at_half_and_half_again() {
        // 300 s at AltOS's 100 Hz, every other term off: each term is all the residual holds,
        // but for the 1 Pa step (0.29 Pa of spread).
        let truth = still(300.0, G, 0.0, 0.0, 0.0);
        let fit = |draw: &LoggerDraw| {
            let log = log(draw, &truth, SEED, 1).unwrap();
            line_fit(&pressure_residuals(&log, &truth, draw.pad_s))
        };
        let quiet = quiet(Profile::Altos);
        for factor in [1.0, 0.5, 1.5] {
            let (_, offset, _) = fit(&LoggerDraw {
                offset_pa: 80.0 * factor,
                ..quiet
            });
            let (drift, _, _) = fit(&LoggerDraw {
                drift_pa_s: 0.02 * factor,
                ..quiet
            });
            let (_, _, white) = fit(&LoggerDraw {
                white_pa: 13.0 * factor,
                ..quiet
            });
            // 30,000 samples: a mean to 0.5 Pa, a slope to the step's share, a spread to 0.41%.
            let held = (offset - 80.0).abs() < 0.5
                && (drift - 0.02).abs() < 1e-4
                && (white - 13.0).abs() < 5.0 * 0.0041 * 13.0;
            assert_eq!(held, factor == 1.0, "{factor}: {offset} {drift} {white}");
        }
    }

    #[test]
    fn the_correlated_noise_correlates_as_stated_at_the_loggers_rate() {
        let truth = still(300.0, G, 0.0, 0.0, 0.0);
        let fast_only = |profile| LoggerDraw {
            fast_noise_m: CORRELATED_NOISE_M,
            ..quiet(profile)
        };
        let slow = log(&fast_only(Profile::Altos), &truth, SEED, 2).unwrap();
        let r: Vec<f64> = pressure_residuals(&slow, &truth, 2.0)
            .iter()
            .map(|r| r.1)
            .collect();
        let m = mean(&r);
        let num: f64 = r.windows(2).map(|w| (w[0] - m) * (w[1] - m)).sum();
        let den: f64 = r.iter().map(|v| (v - m) * (v - m)).sum();
        // 0.5 at 50 Hz is exp(−Δt/τ) = 0.5^(50/100) ≈ 0.707 at AltOS's 100 Hz, less a little for
        // the 1 Pa step. White noise of the same spread would correlate by 0.
        let expect = 0.5_f64.sqrt();
        assert!((num / den - expect).abs() < 0.03, "{}", num / den);
        // At 500 Hz the same noise correlates by 0.5^(50/500) ≈ 0.933, less the 2.03 Pa step's
        // share; a correlation fixed per sample, rather than by the time between them, would
        // read 0.5 at every rate.
        let fast = log(&fast_only(Profile::Imu500), &truth, SEED, 2).unwrap();
        let r: Vec<f64> = pressure_residuals(&fast, &truth, 2.0)
            .iter()
            .map(|r| r.1)
            .collect();
        let m = mean(&r);
        let num: f64 = r.windows(2).map(|w| (w[0] - m) * (w[1] - m)).sum();
        let den: f64 = r.iter().map(|v| (v - m) * (v - m)).sum();
        assert!(
            (num / den - 0.5_f64.powf(0.1)).abs() < 0.03,
            "{}",
            num / den
        );
    }

    #[test]
    fn the_fast_and_slow_noise_hold_their_spreads_and_time_constants_and_fail_changed() {
        let truth = still(300.0, G, 0.0, 0.0, 0.0);
        let density = truth
            .air()
            .unwrap()
            .sample(500.0)
            .unwrap()
            .air
            .density_kg_m3;
        let residuals = |draw: &LoggerDraw, truth: &Truth| -> Vec<f64> {
            let log = log(draw, truth, SEED, 12).unwrap();
            pressure_residuals(&log, truth, draw.pad_s)
                .iter()
                .map(|r| r.1)
                .collect()
        };
        // The fast term at 100 Hz: 30,000 samples correlated by 0.707 hold its spread to about 1%.
        let fast = |m: f64| {
            sd(&residuals(
                &LoggerDraw {
                    fast_noise_m: m,
                    ..quiet(Profile::Altos)
                },
                &truth,
            ))
        };
        let stated = CORRELATED_NOISE_M * density * G;
        assert!(
            (fast(CORRELATED_NOISE_M) - stated).abs() < 0.05 * stated,
            "{}",
            fast(CORRELATED_NOISE_M)
        );
        assert!((fast(0.5 * CORRELATED_NOISE_M) - stated).abs() > 0.3 * stated);
        // The slow term over 3,000 s at AltOS's 10 Hz (its rate after apogee, here at launch): its
        // correlation one second apart is e^(−1/τ), 0.368 at τ = 1 s, to about 0.03 over some
        // 1,500 time constants, and its spread to about 2%.
        let mut long = still(3_000.0, G, 0.0, 0.0, 0.0);
        long.events.apogee_s = 0.0;
        let slow = |m: f64, tau: f64| {
            let draw = LoggerDraw {
                slow_noise_m: m,
                slow_noise_tau_s: tau,
                ..quiet(Profile::Altos)
            };
            let r = residuals(&draw, &long);
            (sd(&r), autocorrelation(&r, 10))
        };
        let stated = 0.27 * density * G;
        let (spread, one_second) = slow(0.27, 1.0);
        assert!(
            (spread - stated).abs() < 0.1 * stated,
            "{spread} against {stated}"
        );
        assert!((one_second - (-1.0_f64).exp()).abs() < 0.1, "{one_second}");
        let (halved, _) = slow(0.135, 1.0);
        assert!((halved - stated).abs() > 0.3 * stated);
        let (_, quicker) = slow(0.27, 0.5);
        assert!((quicker - (-1.0_f64).exp()).abs() > 0.15, "{quicker}");
    }

    #[test]
    fn a_gain_error_scales_the_pressures_change_from_the_pad() {
        // On a climb the gain shows; on a still truth it doesn't.
        let truth = still(20.0, G, 0.0, 0.0, 10.0);
        let base = log(&quiet(Profile::Imu500), &truth, SEED, 3).unwrap();
        let b = channel(&base, Quantity::Pressure);
        let k = b.values.len() - 1;
        let pad = truth.pressure_pa(0.0).unwrap();
        let change = truth.pressure_pa(b.time_s[k] - 2.0).unwrap() - pad;
        assert!(change < -10_000.0, "the climb moves the pressure: {change}");
        let step = 2.0e-5 * ATMOSPHERE_PA;
        // The last sample's pressure with `gain` drawn, less the one with none, against the
        // stated 0.08% of the change: within one of the part's steps.
        let holds = |gain: f64| {
            let gained = log(
                &LoggerDraw {
                    gain,
                    ..quiet(Profile::Imu500)
                },
                &truth,
                SEED,
                3,
            )
            .unwrap();
            let g = channel(&gained, Quantity::Pressure);
            (g.values[k] - b.values[k] - 8.0e-4 * change).abs() <= step
        };
        assert!(holds(8.0e-4));
        // About 8 steps of gain over the climb: none, half or half again reads off by 4 or more.
        for changed in [0.0, 4.0e-4, 1.2e-3] {
            assert!(!holds(changed), "{changed}");
        }
    }

    #[test]
    fn the_ports_transonic_error_reads_the_tunnels_coefficient() {
        let q = 40_000.0;
        let truth = still(60.0, G, 1.0, q, 0.0);
        let at = |calibers: f64| {
            let draw = LoggerDraw {
                port_calibers: calibers,
                ..quiet(Profile::Altos)
            };
            let r: Vec<f64> =
                pressure_residuals(&log(&draw, &truth, SEED, 4).unwrap(), &truth, 2.0)
                    .iter()
                    .map(|r| r.1)
                    .collect();
            mean(&r)
        };
        let expect = noise::port_pressure_coefficient(1.0, 1.0) * q;
        assert!(
            (at(1.0) - expect).abs() < 5.0,
            "{} against {expect}",
            at(1.0)
        );
        // Three calibers back the sign flips: a port there reads +1.2 kPa, not −3.2 kPa.
        assert!((at(3.0) - expect).abs() > 1_000.0);
    }

    #[test]
    fn a_charge_punches_the_pnut_pulse_scaled_and_none_without_a_charge() {
        let mut truth = still(60.0, G, 0.0, 0.0, 0.0);
        truth.events.charges = vec![("Drogue".to_owned(), 30.0)];
        let density = truth
            .air()
            .unwrap()
            .sample(500.0)
            .unwrap()
            .air
            .density_kg_m3;
        let residual_at = |scale: f64, since: f64| {
            let draw = LoggerDraw {
                pulse_scale: scale,
                ..quiet(Profile::Altos)
            };
            let log = log(&draw, &truth, SEED, 5).unwrap();
            let p = channel(&log, Quantity::Pressure);
            let k = p
                .time_s
                .iter()
                .position(|&t| (t - 2.0 - 30.0 - since).abs() < 1e-9)
                .unwrap();
            p.values[k] - truth.pressure_pa(p.time_s[k] - 2.0).unwrap()
        };
        // 0.15 s after the charge the Pnut's pulse reads +15 m: 1.5 times that is about −270 Pa,
        // against noise of a few pascals.
        let expect = -density * G * 1.5 * 15.0;
        assert!((residual_at(1.5, 0.15) - expect).abs() < 30.0);
        assert!(residual_at(0.0, 0.15).abs() < 30.0);
    }

    #[test]
    fn an_accelerometer_holds_its_scale_bias_and_noise_and_clips_at_its_range() {
        let f = 10.0 * G;
        let truth = still(60.0, f, 0.0, 0.0, 0.0);
        let stated = LoggerDraw {
            density_g_per_root_hz: 0.005,
            accel_scale: 0.08,
            accel_bias_m_s2: 0.03 * G,
            ..quiet(Profile::Altos)
        };
        let mean_reading = |draw: &LoggerDraw| {
            mean(&acceleration_until(
                &log(draw, &truth, SEED, 6).unwrap(),
                2.0,
                60.0,
            ))
        };
        let expect = 1.08 * f + 0.03 * G;
        // 44 mg of noise and a 49 mg step over 6,000 samples: the mean holds to about 1 mg.
        assert!((mean_reading(&stated) - expect).abs() < 0.005 * G);
        assert!(
            (mean_reading(&LoggerDraw {
                accel_scale: 0.0,
                ..stated
            }) - expect)
                .abs()
                > 0.5 * G
        );
        assert!(
            (mean_reading(&LoggerDraw {
                accel_bias_m_s2: 0.0,
                ..stated
            }) - expect)
                .abs()
                > 0.02 * G
        );
        let spread_of = |draw: &LoggerDraw| {
            sd(&acceleration_until(
                &log(draw, &truth, SEED, 6).unwrap(),
                2.0,
                60.0,
            ))
        };
        let sigma = 0.005 * (1.57 * 50.0_f64).sqrt() * G;
        let spread = spread_of(&stated);
        assert!(
            (spread - sigma).abs() < 0.1 * sigma,
            "{spread} against {sigma}"
        );
        let doubled = spread_of(&LoggerDraw {
            density_g_per_root_hz: 0.01,
            ..stated
        });
        assert!((doubled - sigma).abs() > 0.5 * sigma, "{doubled}");
        // Past apogee (here from launch) AltOS logs at 10 Hz, but its part keeps the 50 Hz
        // bandwidth of the climb and so its noise; a σ taken from the descent's rate reads √10
        // lower (0.31σ measured, the step's scatter included).
        let mut descending = still(600.0, f, 0.0, 0.0, 0.0);
        descending.events.apogee_s = 0.0;
        let descent_log = log(&stated, &descending, SEED, 6).unwrap();
        let a = channel(&descent_log, Quantity::Acceleration);
        let after: Vec<f64> = a
            .time_s
            .iter()
            .zip(&a.values)
            .filter(|(t, _)| **t > 3.0 && **t - 2.0 < 600.0)
            .map(|(_, v)| *v)
            .collect();
        assert!(after.len() > 5_000, "{}", after.len());
        let descent = sd(&after);
        assert!(
            (descent - sigma).abs() < 0.1 * sigma,
            "{descent} against {sigma}"
        );
        // A 40 g load on a ±32 g part reads 32 g on every sample.
        let hard = still(10.0, 40.0 * G, 0.0, 0.0, 0.0);
        let clipped = log(&quiet(Profile::Imu500), &hard, SEED, 7).unwrap();
        let values = acceleration_until(&clipped, 2.0, 10.0);
        assert!(!values.is_empty());
        assert!(
            values
                .iter()
                .all(|&v| (v - quantize(32.0 * G, 64.0 / 65_536.0 * G)).abs() < 1e-9)
        );
        // The ±180 g part reads the same 40 g load unclipped.
        let high = acceleration_until(
            &log(&quiet(Profile::Altos), &hard, SEED, 7).unwrap(),
            2.0,
            10.0,
        );
        assert!(!high.is_empty());
        assert!(
            high.iter()
                .all(|&v| (v - quantize(40.0 * G, 0.049 * G)).abs() < 1e-9)
        );
    }

    #[test]
    fn gnss_lags_a_constant_climb_by_a_over_omega_squared_and_drops_out_as_drawn() {
        let climb = 20.0;
        let truth = still(40.0, G, 0.0, 0.0, climb);
        let behind = |draw: &LoggerDraw| {
            let log = log(draw, &truth, SEED, 8).unwrap();
            let h = channel(&log, Quantity::GnssAltitude);
            let lags: Vec<f64> = h
                .time_s
                .iter()
                .zip(&h.values)
                .filter(|(t, _)| (**t - 2.0) > 25.0 && (**t - 2.0) < 40.0)
                .map(|(t, v)| truth.at(t - 2.0).unwrap().height_m + truth.site_elevation_m - v)
                .collect();
            mean(&lags)
        };
        let stated = quiet(Profile::GnssTracker);
        let expect = climb / (0.5 * 0.5);
        // 150 fixes of 3 m noise; the frame's curvature adds under a meter at 16 km.
        assert!(
            (behind(&stated) - expect).abs() < 3.0,
            "{}",
            behind(&stated)
        );
        assert!(
            (behind(&LoggerDraw {
                gnss_omega_rad_s: 1.0,
                ..stated
            }) - expect)
                .abs()
                > 30.0
        );
        // A dropout of 4 s from liftoff, or from burnout at 20 s, leaves no fix in it; none drawn
        // leaves the fixes there.
        let mut truth = truth;
        truth.events.burnout_s = 20.0;
        let fixes_in = |draw: &LoggerDraw, from: f64| {
            let log = log(draw, &truth, SEED, 9).unwrap();
            let times = &channel(&log, Quantity::Latitude).time_s;
            times
                .iter()
                .filter(|t| (from..from + 4.0).contains(&(**t - 2.0)))
                .count()
        };
        let at_liftoff = LoggerDraw {
            gnss_dropout_liftoff_s: 4.0,
            ..stated
        };
        let at_burnout = LoggerDraw {
            gnss_dropout_burnout_s: 4.0,
            ..stated
        };
        assert_eq!(fixes_in(&at_liftoff, 0.0), 0);
        assert_eq!(fixes_in(&at_liftoff, 20.0), 40);
        assert_eq!(fixes_in(&at_burnout, 20.0), 0);
        assert_eq!(fixes_in(&at_burnout, 0.0), 40);
        assert_eq!(fixes_in(&stated, 0.0), 40);
        assert_eq!(fixes_in(&stated, 20.0), 40);
    }

    #[test]
    fn the_log_lines_up_with_the_truth_by_its_drawn_pad_span() {
        // A climb at 10 m/s² from launch, its specific force rising 1 m/s² each second, logged
        // 7 s before launch with every noise term off: each value is the truth's 7 s earlier on
        // the log's clock, to its step. A log lined up by the 2 s every other test uses is off by
        // 125 m or more from launch; the pressure's half a step catches a shift of 0.05 s.
        let mut truth = still(30.0, 0.0, 0.0, 0.0, 10.0);
        for sample in &mut truth.samples {
            sample.axial_specific_force_m_s2 = sample.time_s;
        }
        let pad_s = 7.0;
        let draw = LoggerDraw {
            pad_s,
            gnss_omega_rad_s: 5.0,
            ..quiet(Profile::Altos)
        };
        let log = log(&draw, &truth, SEED, 13).unwrap();
        for (t, r) in pressure_residuals(&log, &truth, pad_s) {
            assert!(r.abs() <= 0.5 + 1e-9, "{t}: {r}");
        }
        let a = channel(&log, Quantity::Acceleration);
        for (&t, &v) in a.time_s.iter().zip(&a.values) {
            let f = (t - pad_s).clamp(0.0, 30.0);
            assert!((v - quantize(f, 0.049 * G)).abs() < 1e-9, "{t}: {v}");
        }
        // The fixes, lagging 0.4 m at 5 rad/s, each within five of its 3 m (9.8 ft) of noise.
        let h = channel(&log, Quantity::GnssAltitude);
        for (&t, &v) in h.time_s.iter().zip(&h.values) {
            let z = truth.at(t - pad_s).unwrap().height_m;
            assert!((v - truth.site_elevation_m - z).abs() < 15.0, "{t}: {v}");
        }
    }

    #[test]
    fn a_draw_or_truth_no_logger_could_write_is_refused_by_name() {
        let truth = still(10.0, G, 0.0, 0.0, 0.0);
        let refused = |draw: &LoggerDraw, truth: &Truth| match log(draw, truth, SEED, 1) {
            Err(SyntheticError::Invalid { what }) => what,
            other => panic!("{other:?}"),
        };
        let altos = quiet(Profile::Altos);
        assert_eq!(
            refused(
                &LoggerDraw {
                    pad_s: -1.0,
                    ..altos
                },
                &truth
            ),
            "a logger's pad or ground span that isn't a finite length"
        );
        assert_eq!(
            refused(
                &LoggerDraw {
                    ground_s: f64::NAN,
                    ..altos
                },
                &truth
            ),
            "a logger's pad or ground span that isn't a finite length"
        );
        assert_eq!(
            refused(
                &LoggerDraw {
                    gnss_rate_hz: 0.0,
                    ..altos
                },
                &truth
            ),
            "a GNSS fix rate that isn't a positive number"
        );
        assert_eq!(
            refused(
                &LoggerDraw {
                    slow_noise_tau_s: 0.0,
                    ..altos
                },
                &truth
            ),
            "a slow noise time constant that isn't a positive number"
        );
        assert_eq!(
            refused(
                &LoggerDraw {
                    ground_s: 1.0e6,
                    ..altos
                },
                &truth
            ),
            "a log longer than a day"
        );
        let mut empty = truth.clone();
        empty.samples.clear();
        assert_eq!(
            refused(&altos, &empty),
            "a truth with no samples or an event at no finite time"
        );
        let mut late = truth.clone();
        late.events.landing_s = f64::INFINITY;
        assert_eq!(
            refused(&altos, &late),
            "a truth with no samples or an event at no finite time"
        );
        let mut unclocked = truth;
        unclocked.samples[3].time_s = f64::NAN;
        assert_eq!(
            refused(&altos, &unclocked),
            "a truth sample at no finite time"
        );
        // A barometer-only logger has no fix rate to refuse.
        assert!(
            log(
                &LoggerDraw {
                    gnss_rate_hz: 0.0,
                    ..quiet(Profile::Rrc3)
                },
                &still(10.0, G, 0.0, 0.0, 0.0),
                SEED,
                1
            )
            .is_ok()
        );
    }

    #[test]
    fn the_clock_runs_from_the_pad_to_the_ground_at_each_parts_rate() {
        // A 40 s flight with apogee at 10 s, logged 3 s before launch and 5 s after landing.
        let mut truth = still(40.0, G, 0.0, 0.0, 0.0);
        truth.events.apogee_s = 10.0;
        let end = 3.0 + 40.0 + 5.0;
        // The log's own clock, or its fixes' for a GNSS receiver.
        let spans = |draw: &LoggerDraw| {
            let log = log(draw, &truth, SEED, 12).unwrap();
            if draw.profile == Profile::GnssTracker {
                channel(&log, Quantity::Latitude).time_s.clone()
            } else {
                log.time_s
            }
        };
        let steps = |times: &[f64]| times.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let span = |profile| LoggerDraw {
            pad_s: 3.0,
            ground_s: 5.0,
            ..quiet(profile)
        };
        // PerfectFlite logs at 20 Hz throughout; its first sample is at its own zero and its
        // last within one sample of the end.
        let times = spans(&span(Profile::PerfectFlite));
        assert_eq!(times[0], 0.0);
        assert!(steps(&times).iter().all(|d| (d - 0.05).abs() < 1e-9));
        let last = times[times.len() - 1];
        assert!(last <= end && last > end - 0.05, "{last}");
        // AltOS logs at 100 Hz to the first sample past apogee, then at 10 Hz.
        let times = spans(&span(Profile::Altos));
        let apogee = 3.0 + 10.0;
        for (w, d) in times.windows(2).zip(steps(&times)) {
            let expect = if w[0] <= apogee { 0.01 } else { 0.1 };
            assert!((d - expect).abs() < 1e-9, "{} to {}", w[0], w[1]);
        }
        let last = times[times.len() - 1];
        assert!(last <= end && last > end - 0.1, "{last}");
        // A GNSS receiver fixes at its drawn rate, 1, 5 or 10 Hz, over the same span.
        for rate in [1.0, 5.0, 10.0] {
            let times = spans(&LoggerDraw {
                gnss_rate_hz: rate,
                ..span(Profile::GnssTracker)
            });
            assert_eq!(times[0], 0.0);
            assert!(steps(&times).iter().all(|d| (d - 1.0 / rate).abs() < 1e-9));
            let last = times[times.len() - 1];
            assert!(last <= end && last > end - 1.0 / rate, "{rate}: {last}");
        }
    }

    #[test]
    fn gnss_fixes_scatter_three_meters_on_each_axis() {
        let truth = still(120.0, G, 0.0, 0.0, 0.0);
        let log = log(&quiet(Profile::GnssTracker), &truth, SEED, 10).unwrap();
        let h = channel(&log, Quantity::GnssAltitude);
        let lat = channel(&log, Quantity::Latitude);
        let up: Vec<f64> = h
            .values
            .iter()
            .map(|v| v - truth.site_elevation_m)
            .collect();
        // A degree of latitude is about 110.94 km at 35° N on WGS84, and a degree of longitude
        // 91.29 km.
        let north: Vec<f64> = lat
            .values
            .iter()
            .map(|v| (v - SITE_LATITUDE_DEG) * 110_940.0)
            .collect();
        let east: Vec<f64> = channel(&log, Quantity::Longitude)
            .values
            .iter()
            .map(|v| (v - SITE_LONGITUDE_DEG) * 91_290.0)
            .collect();
        // About 1,200 fixes: the spread's standard error is about 0.06 m.
        for axis in [&up, &north, &east] {
            assert!((sd(axis) - GNSS_NOISE_M).abs() < 0.3, "{}", sd(axis));
            assert!(mean(axis).abs() < 0.5, "{}", mean(axis));
        }
    }

    #[test]
    fn a_barometers_height_is_its_formulas_pressure_altitude_from_the_pad() {
        // 5,000 ft of the 1976 standard is 843.07 hPa; the NOAA formula the RRC3 uses reads 4,998 ft.
        let p = 84_307.0;
        assert!((height_of(Profile::Altos, p).unwrap() - 1_524.0).abs() < 0.5);
        assert!((height_of(Profile::Rrc3, p).unwrap() - 4_998.0 * FOOT_M).abs() < 0.5);
        // On a climb in standard air each barometer reads the truth's height above the pad, to its
        // correlated noise (0.5 m), its step and, for the RRC3, its formula's 0.83 m in 2 km.
        let truth = still(20.0, G, 0.0, 0.0, 10.0);
        for profile in [
            Profile::PerfectFlite,
            Profile::Rrc3,
            Profile::Eggtimer,
            Profile::Altos,
            Profile::Imu500,
        ] {
            let log = log(&quiet(profile), &truth, SEED, 11).unwrap();
            let worst = log
                .time_s
                .iter()
                .zip(&log.altitude_m)
                .map(|(t, h)| (h - truth.at(t - 2.0).unwrap().height_m).abs())
                .fold(0.0, f64::max);
            assert!(worst < 4.0, "{profile:?}: {worst}");
            assert!(log.altitude_m.iter().copied().fold(0.0, f64::max) > 1_900.0);
        }
    }

    #[test]
    fn each_logger_writes_its_familys_rate_step_and_channels() {
        let mut flyer = Flyer::new(crate::synthetic::designs::DesignFiles::committed());
        for profile in Profile::ALL {
            let index = (0..)
                .find(|&k| draw(Set::Tuning, k).logger.profile == profile)
                .unwrap();
            let flight = flyer.flight(Set::Tuning, index).unwrap();
            let log = &flight.log;
            assert_eq!(log.format, profile.format());
            let apogee = flight.draw.logger.pad_s + flight.truth.events.apogee_s;
            for pair in log.time_s.windows(2) {
                let rate = profile.rate_hz(pair[1] <= apogee + 1e-9);
                let step = pair[1] - pair[0];
                assert!(
                    (step - 1.0 / rate).abs() < 1e-9 || profile == Profile::Altos,
                    "{profile:?}"
                );
            }
            match profile.barometer().map(|b| b.written) {
                Some(Written::Altitude { step_m }) => {
                    assert_eq!(step_m, FOOT_M);
                    assert!(
                        log.altitude_m
                            .iter()
                            .all(|h| ((h / FOOT_M) - (h / FOOT_M).round()).abs() < 1e-9)
                    );
                    assert_eq!(log.channels_of(Quantity::Pressure).count(), 0);
                }
                Some(Written::Pressure { step_pa } | Written::PressureAlone { step_pa }) => {
                    let p = channel(log, Quantity::Pressure);
                    assert!(
                        p.values
                            .iter()
                            .all(|v| ((v / step_pa) - (v / step_pa).round()).abs() < 1e-6)
                    );
                    assert_eq!(p.values.len(), log.time_s.len());
                }
                None => {
                    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
                }
            }
            match profile.accelerometer() {
                Some(a) => {
                    let values = &channel(log, Quantity::Acceleration).values;
                    let step = a.step_g * G;
                    assert!(values.iter().all(|v| v.abs() <= a.range_g * G + step));
                    assert!(
                        values
                            .iter()
                            .all(|v| ((v / step) - (v / step).round()).abs() < 1e-6)
                    );
                }
                None => assert_eq!(log.channels_of(Quantity::Acceleration).count(), 0),
            }
            assert_eq!(
                log.channels_of(Quantity::GnssAltitude).count(),
                usize::from(profile.gnss())
            );
        }
    }

    #[test]
    fn each_loggers_draw_stays_inside_its_parts_ranges() {
        // Each part's stated terms, as the page and its sources give them, so that a changed
        // constant fails here and not only the draws it bounds.
        let stated_barometer = |profile| match profile {
            Profile::PerfectFlite => Some(((0.0, 0.0), 100.0, 0.001)),
            Profile::Rrc3 => Some(((2.3, 2.3), 100.0, 0.0025)),
            Profile::Altos => Some(((2.4, 13.0), 100.0, 0.0025)),
            Profile::Eggtimer | Profile::Imu500 => Some(((0.2, 6.6), 0.0, 0.0008)),
            Profile::GnssTracker => None,
        };
        let stated_accelerometer = |profile| match profile {
            Profile::Altos => Some(((0.005, 0.005), 0.049, 180.0, 0.10)),
            Profile::Imu500 => Some(((0.000_16, 0.000_4), 64.0 / 65_536.0, 32.0, 0.03)),
            _ => None,
        };
        // Every draw inside [lo, hi], and the least and greatest within 2% of the span of its
        // ends.
        let spans = |values: Vec<f64>, lo: f64, hi: f64| {
            let least = values.iter().copied().fold(f64::INFINITY, f64::min);
            let greatest = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let slack = 0.02 * (hi - lo);
            least >= lo && greatest <= hi && least <= lo + slack && greatest >= hi - slack
        };
        let mut rng = SeededRng::for_stream(SEED, &[0]);
        for profile in Profile::ALL {
            let draws: Vec<LoggerDraw> = (0..2_000)
                .map(|_| LoggerDraw::draw(&mut rng, profile))
                .collect();
            let column = |f: &dyn Fn(&LoggerDraw) -> f64| draws.iter().map(f).collect::<Vec<_>>();
            assert_eq!(
                profile
                    .barometer()
                    .map(|b| (b.white_pa, b.offset_pa, b.relative_gain)),
                stated_barometer(profile),
                "{profile:?}"
            );
            assert_eq!(
                profile.accelerometer().map(|a| (
                    a.density_g_per_root_hz,
                    a.step_g,
                    a.range_g,
                    a.scale
                )),
                stated_accelerometer(profile),
                "{profile:?}"
            );
            assert!(spans(column(&|d| d.slow_noise_tau_s), 0.65, 1.26));
            assert!(spans(column(&|d| d.accel_bias_m_s2 / G), -0.05, 0.05));
            assert!(spans(column(&|d| d.gnss_omega_rad_s), 0.25, 1.0));
            if let Some(((white_lo, white_hi), offset, gain)) = stated_barometer(profile) {
                assert!(spans(column(&|d| d.slow_noise_m), 0.24, 0.29));
                assert!(column(&|d| d.fast_noise_m).iter().all(|&m| m == 0.5));
                assert!(spans(column(&|d| d.gain), -gain, gain), "{profile:?}");
                if offset > 0.0 {
                    assert!(spans(column(&|d| d.offset_pa), -offset, offset));
                }
                if white_hi > white_lo {
                    assert!(spans(column(&|d| d.white_pa), white_lo, white_hi));
                }
            }
            if let Some(((lo, hi), _, _, scale)) = stated_accelerometer(profile) {
                if hi > lo {
                    assert!(spans(column(&|d| d.density_g_per_root_hz), lo, hi));
                }
                assert!(spans(column(&|d| d.accel_scale), -scale, scale));
            }
            let all = |f: &dyn Fn(&LoggerDraw) -> bool| draws.iter().all(f);
            assert!(all(
                &|d| (2.0..=10.0).contains(&d.pad_s) && (2.0..=10.0).contains(&d.ground_s)
            ));
            assert!(all(
                &|d| (1.0..=5.0).contains(&d.port_calibers) && (0.0..=2.0).contains(&d.pulse_scale)
            ));
            assert!(all(
                &|d| (1.0 / 300.0..=3.0 / 30.0).contains(&d.drift_pa_s.abs())
            ));
            assert!(
                draws.iter().any(|d| d.drift_pa_s < 0.0)
                    && draws.iter().any(|d| d.drift_pa_s > 0.0)
            );
            assert!(all(&|d| [1.0, 5.0, 10.0].contains(&d.gnss_rate_hz)));
            for rate in [1.0, 5.0, 10.0] {
                assert!(draws.iter().any(|d| d.gnss_rate_hz == rate));
            }
            assert!(all(&|d| (0.25..=1.0).contains(&d.gnss_omega_rad_s)));
            assert!(all(&|d| (0.0..=10.0).contains(&d.gnss_dropout_liftoff_s)));
            assert!(all(&|d| (0.0..=10.0).contains(&d.gnss_dropout_burnout_s)));
            assert!(all(&|d| d.accel_bias_m_s2.abs() <= ACCEL_BIAS_G * G));
            assert!(all(
                &|d| (SLOW_NOISE_TAU_S.0..=SLOW_NOISE_TAU_S.1).contains(&d.slow_noise_tau_s)
            ));
            match profile.barometer() {
                Some(b) => {
                    assert!(all(&|d| (b.white_pa.0..=b.white_pa.1).contains(&d.white_pa)));
                    assert!(all(&|d| d.offset_pa.abs() <= b.offset_pa));
                    assert!(all(&|d| d.gain.abs() <= b.relative_gain));
                    assert!(all(&|d| d.fast_noise_m == CORRELATED_NOISE_M));
                    assert!(all(
                        &|d| (SLOW_NOISE_M.0..=SLOW_NOISE_M.1).contains(&d.slow_noise_m)
                    ));
                }
                None => assert!(all(&|d| d.white_pa == 0.0
                    && d.offset_pa == 0.0
                    && d.gain == 0.0
                    && d.fast_noise_m == 0.0
                    && d.slow_noise_m == 0.0)),
            }
            match profile.accelerometer() {
                Some(a) => {
                    let (lo, hi) = a.density_g_per_root_hz;
                    assert!(all(&|d| (lo..=hi).contains(&d.density_g_per_root_hz)));
                    assert!(all(&|d| d.accel_scale.abs() <= a.scale));
                }
                None => assert!(all(
                    &|d| d.density_g_per_root_hz == 0.0 && d.accel_scale == 0.0
                )),
            }
        }
    }

    #[test]
    fn altos_logs_100_hz_to_apogee_and_10_hz_after() {
        let mut flyer = Flyer::new(crate::synthetic::designs::DesignFiles::committed());
        let index = (0..)
            .find(|&k| draw(Set::A, k).logger.profile == Profile::Altos)
            .unwrap();
        let flight = flyer.flight(Set::A, index).unwrap();
        let apogee = flight.draw.logger.pad_s + flight.truth.events.apogee_s;
        let steps: Vec<(f64, f64)> = flight
            .log
            .time_s
            .windows(2)
            .map(|w| (w[1], w[1] - w[0]))
            .collect();
        assert!(
            steps
                .iter()
                .filter(|(t, _)| *t <= apogee)
                .all(|(_, d)| (d - 0.01).abs() < 1e-9)
        );
        assert!(
            steps
                .iter()
                .filter(|(t, _)| *t > apogee + 0.1)
                .all(|(_, d)| (d - 0.1).abs() < 1e-9)
        );
    }
}
