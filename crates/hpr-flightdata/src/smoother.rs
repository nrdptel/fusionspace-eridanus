//! The smoother: a flight's height and vertical speed at every sample, each with its standard
//! error, from the barometer, the axial accelerometer and a GNSS receiver together, and a health
//! card saying how each sensor fared ([M7.2d][m7-2d]).
//!
//! **What it is for.** The [readings] take one number each from one track. The
//! smoother takes the whole flight from every sensor the log holds, and says at each moment how
//! far the truth may be: its 90% interval is the estimate ± [`INTERVAL_Z`] standard errors. On
//! HPR Sim's synthetic flights, whose truth is known, it is checked at burnout, the top speed,
//! apogee and a drawn moment (`cargo xtask synthetic`). There the three sets' cells hold between
//! 94.3% and 99.4% of the truths inside the 90% intervals. Two logger cells hold 89.2% and 89.7%,
//! which the check allows. Its rules were set with those sets in view.
//!
//! **How far to trust it.** It has not been compared with a real flight's truth. Its errors are
//! the sensors' errors as the synthetic flights draw them, from the makers' datasheets and
//! measured public logs ([ADR-280][adr-280] §3, the plan for the synthetic flights). A real logger
//! whose errors are larger or of another kind, such as a port that reads the flow differently or
//! a swing correlated over more than a second or two, can leave the truth outside the interval
//! more often than it says; the health card shows where its sensors disagreed. It needs a barometer: a GNSS track alone is never trusted, as its receiver
//! lags a climb by up to hundreds of meters while flagging its fix valid
//! (`docs/research/gnss-in-boost.md`).
//!
//! **Method** (`docs/research/altimeter-smoothing-methods.md`, "What this suggests" 1–2, 5;
//! [ADR-300][adr-300], the smoother and its check). A Kalman filter runs forward over every sample, then a Rauch–Tung–Striebel pass back
//! (`kalman`). Its state is the logger's height above the pad, its vertical speed and
//! acceleration, and the sensors' own errors: the accelerometer's bias, scale and a slowly
//! changing model error (the rocket's tilt puts its axis off the vertical); the barometer's slow
//! correlated noise, its drift and the drift's rate, and the static port's pressure coefficient in
//! the fall; the day's air off the standard atmosphere as a share of the height, which only a GNSS
//! receiver measures; and the receiver's offset. The acceleration is a white-jerk state that the
//! accelerometer measures (`a + g` along the axis, gravity's reaction in it), so a log with no
//! accelerometer, or one clipped, uses the same model with nothing measuring it. On the pad the
//! height, speed and acceleration are held at zero until liftoff, found from the accelerometer
//! where there is one.
//!
//! The climb's port coefficient can't be learned from the log, so it is held at its prior, and
//! the log is smoothed again with it a standard deviation off, replaying the first run's choices;
//! each standard error is widened by the difference ([`smooth_with`]).
//!
//! - **The barometer** is de-weighted from [`TRANSONIC_FROM_MACH`] to [`TRANSONIC_FULL_MACH`] and
//!   above, where its port's pressure coefficient jumps (`docs/research/altimeter-mach-error.md`
//!   §1–2), for [`CHARGE_WINDOW_S`] after each charge the log records, and around its own apogee
//!   ([`APOGEE_CHARGE_SPAN_M`]), where a charge fires whether or not the log names it.
//! - **The GNSS receiver** is fused at zero weight from liftoff until its vertical speed has
//!   agreed with the smoother's for [`GNSS_AGREE_WINDOW_S`] (`gnss-in-boost.md` §5), then with
//!   its variance inflated for [`GNSS_SETTLE_S`]; never on its fix flag.
//! - **Every sensor** is gated: a sample whose squared innovation passes [`GATE_SIGMAS`]² of its
//!   variance is rejected and listed, and a sensor rejected for [`GATE_RESET_S`] is taken back
//!   with the height's and speed's uncertainty widened by the gap.
//!
//! [m7-2d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2d
//! [adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
//! [adr-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md

mod kalman;

use hpr_atmos::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::log::{Channel, FlightLog, Quantity};
use crate::readings::{
    self, ACCEL_BIAS_M_S2, ACCEL_SCALE_SHARE, AIR_TEMPERATURE_SPREAD_K, BAROMETER_GAIN, Clip,
    DRIFT_M_PER_S, GRAVITY_TOLERANCE_G, Gravity, RAIL_TILT_DEG, Reading, Readings, Source,
};
use kalman::{Matrix, N, PACKED, Vector};

const G0: f64 = STANDARD_GRAVITY_MPS2;

// The state's entries.
/// Height above the pad, m.
const H: usize = 0;
/// Vertical speed, m/s, up.
const V: usize = 1;
/// Vertical acceleration, m/s², up.
const A: usize = 2;
/// The accelerometer's bias, m/s².
const BIAS: usize = 3;
/// The accelerometer's scale error.
const SCALE: usize = 4;
/// The accelerometer's model error, m/s²: its axis off the vertical, and the drag a sideways
/// speed adds along it.
const TILT: usize = 5;
/// The barometer's slow correlated noise, m.
const SLOW: usize = 6;
/// The barometer's drift, m.
const DRIFT: usize = 7;
/// The drift's rate, m/s.
const DRIFT_RATE: usize = 8;
/// The day's air off the standard atmosphere, as a share of the height: `−ΔT/T_pad`. Only a GNSS
/// receiver's geometric heights measure it; without one it is held at its prior and its spread
/// carried into the output.
const AIR: usize = 9;
/// The GNSS receiver's altitude less the height above the pad, m.
const GNSS: usize = 10;
/// The static port's pressure coefficient in a fall: its own, as a fall's attitude isn't the
/// climb's, and one shared would let a long fast fall set the climb's.
const PORT_FALL: usize = 11;

/// The standard normal quantile at 95%: the estimate ± this many standard errors is a two-sided
/// 90% interval.
pub const INTERVAL_Z: f64 = 1.644_853_626_951_472_2;

/// A sample is rejected when its innovation passes this many of its standard deviations: PX4's
/// default barometer gate, 5 (its EKF2 parameter `EKF2_BARO_GATE`; the smoothing note §7).
pub const GATE_SIGMAS: f64 = 5.0;

/// A sensor rejected this long is taken back, s: HPR Sim's choice, longer than a charge's pulse
/// (0.3 s on the public Pnut log the synthetic flights copy) and shorter than any transonic span.
pub const GATE_RESET_S: f64 = 1.0;

/// The barometer's fast correlated noise, 1σ, m: an AR(1) process of `φ = 0.5` at 50 Hz, measured
/// on public logs (`docs/research/altimeter-sensor-noise.md` §6).
pub const FAST_NOISE_M: f64 = 0.5;

/// Its time constant, s: `−1/(50 ln 0.5)`.
pub const FAST_NOISE_TAU_S: f64 = 0.028_853_900_817_779_27;

/// The barometer's slow correlated noise, 1σ, m: the top of the 0.24–0.29 m of a first-order
/// Gauss–Markov term measured on a BMP085 at rest (`altimeter-sensor-noise.md` §3).
pub const SLOW_NOISE_M: f64 = 0.29;

/// Its time constant, s: within the 0.65–1.26 s measured beside it.
pub const SLOW_NOISE_TAU_S: f64 = 1.0;

/// The prior on the port's pressure coefficient: its mean. The ogive-cylinder's measured
/// coefficient a caliber or more behind the nose's shoulder is −0.02 to 0.00 below Mach 0.9
/// (`altimeter-mach-error.md` §1–2); HPR Sim's prior is centered there…
pub const PORT_PRIOR_MEAN: f64 = -0.01;

/// …with this standard deviation, half the range again either side. The climb's coefficient is
/// held there and enters each interval by the consider run ([`smooth_with`]); learned from the
/// log, it came out biased by 0.010 claiming 0.001 on the synthetic flights' barometer-only
/// loggers ([ADR-300], the smoother and its check).
///
/// [ADR-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md
pub const PORT_PRIOR_SD: f64 = 0.02;

/// The Mach number, estimated high (`mach_high`), at which the barometer begins to be
/// de-weighted: below it the port's coefficient holds within 0.01 of its subsonic value on the
/// tunnel's ogive-cylinder a caliber or more behind the shoulder (`altimeter-mach-error.md`
/// §1–2). HPR Sim's margin below that note's 0.9.
pub const TRANSONIC_FROM_MACH: f64 = 0.8;

/// The Mach number from which the barometer is fully de-weighted.
pub const TRANSONIC_FULL_MACH: f64 = 0.9;

/// How far the port's coefficient may move from its subsonic value once fully de-weighted: the
/// tunnel's largest jump, −0.07 to −0.20 at the shoulder from Mach 0.7 to 1.0, rounded up.
pub const TRANSONIC_PORT_SHIFT: f64 = 0.2;

/// The time over which a de-weighted span's error is correlated, s: the second or so a climb takes
/// from Mach 0.8 through 1.0, and longer than a charge's pulse. HPR Sim's choice.
pub const DEWEIGHT_TAU_S: f64 = 1.0;

/// How much a first-order Gauss–Markov error of time constant `tau_s` weighs as one sample's white
/// variance when samples are `dt_s` apart: `(1 + φ)/(1 − φ)`, `φ = e^{−Δt/τ}`, the variance of the
/// mean of many such samples over that of white ones (the smoothing note §6).
fn correlated_factor(dt_s: f64, tau_s: f64) -> f64 {
    let phi = (-dt_s / tau_s).exp();
    (1.0 + phi) / (1.0 - phi)
}

/// How long after a charge the log records the barometer is de-weighted, s: the public Pnut
/// log's pulse lasts 0.3 s.
pub const CHARGE_WINDOW_S: f64 = 0.5;

/// A charge is expected at apogee whether or not the log names it (a dual-deploy drogue, or a
/// motor's ejection timed for it), so the barometer is de-weighted from the first time its
/// running median comes within this of its highest to the last, and [`CHARGE_WINDOW_S`] after,
/// m: a coast passes within 5 m of its apogee for about a second either side, `√(2·5/g)`, wider
/// than the running median's noise lets the top be placed. HPR Sim's choice.
pub const APOGEE_CHARGE_SPAN_M: f64 = 5.0;

/// The barometer's standard deviation in that window, m: twice the Pnut pulse's 16 m.
pub const CHARGE_SD_M: f64 = 32.0;

/// Words in an event's name that mark a charge.
pub const CHARGE_WORDS: [&str; 7] = [
    "drogue", "main", "charge", "deploy", "pyro", "eject", "apogee",
];

/// The accelerometer's model error as a share of its reading: the axis `θ` off the vertical
/// costs `1 − cos θ` of the reading. Under power the axis is within a rail's 20°
/// ([`RAIL_TILT_DEG`]). In the coast it pitches over as the climb slows while the sideways speed
/// through the air stays, `tan θ = v_side/v`: at most the burnout's `sin 20°` share of the top
/// speed plus a [`WIND_LIMIT_M_S`] wind, so the share grows toward one at apogee, where the
/// reading is small.
pub fn tilt_share(climb_m_s: f64, top_speed_m_s: f64, powered: bool) -> f64 {
    let rail = 1.0 - RAIL_TILT_DEG.to_radians().cos();
    if powered {
        return rail;
    }
    let side = RAIL_TILT_DEG.to_radians().sin() * top_speed_m_s + WIND_LIMIT_M_S;
    let climb = climb_m_s.max(0.0);
    let coast = 1.0 - climb / climb.hypot(side);
    rail.max(coast)
}

/// The largest wind a high-power flight leaves the pad in, m/s: 20 mph, the most the NAR
/// high-power safety code and NFPA 1127 allow.
pub const WIND_LIMIT_M_S: f64 = 8.9408;

/// The model error's floor, m/s²: the drag a sideways speed puts along the axis near apogee, where
/// the rocket may lie far over. HPR Sim's choice.
pub const TILT_FLOOR_M_S2: f64 = 0.5;

/// The model error's time constant, s. HPR Sim's choice: a tilt changes over seconds.
pub const TILT_TAU_S: f64 = 2.0;

/// The white jerk's spectral density on the pad, m²/s⁵.
pub const JERK_PAD: f64 = 1.0;

/// …while the motor may be burning, m²/s⁵: lets the acceleration rise by 100 m/s² within 0.05 s
/// at 3σ, as a motor's thrust does.
pub const JERK_POWERED: f64 = 1.0e5;

/// …in the coast to apogee, m²/s⁵: the drag changes slowly.
pub const JERK_COAST: f64 = 10.0;

/// …in the descent, m²/s⁵: a canopy's opening changes the speed within a second or two.
pub const JERK_DESCENT: f64 = 100.0;

/// How long the motor may still burn after the acceleration first falls below `−g/2`, s.
pub const POWERED_HOLD_S: f64 = 1.0;

/// A climb is under way, for finding its burnout and apogee, once its speed has passed this, m/s.
pub const APOGEE_SPEED_M_S: f64 = 10.0;

/// The pad's hold on the height, speed and acceleration: their standard deviation there.
pub const PAD_SD: f64 = 0.01;

/// The GNSS receiver's speed must agree with the smoother's this long before it is fused, s
/// (`gnss-in-boost.md` §5: "for at least 2 s").
pub const GNSS_AGREE_WINDOW_S: f64 = 2.0;

/// …within this, m/s ("within a few m/s"), and within twice their combined standard error.
pub const GNSS_AGREE_M_S: f64 = 3.0;

/// A gap between fixes this long ends the receiver's trust, s: a dropout.
pub const GNSS_GAP_S: f64 = 2.0;

/// After trust, its variance is inflated this long, s…
pub const GNSS_SETTLE_S: f64 = 5.0;

/// …this many times.
pub const GNSS_SETTLE_FACTOR: f64 = 4.0;

/// The receiver's altitude noise when its pad can't say, m; its pad's scatter is floored at
/// [`GNSS_SD_FLOOR_M`].
pub const GNSS_SD_DEFAULT_M: f64 = 10.0;

/// The least its noise is taken as, m.
pub const GNSS_SD_FLOOR_M: f64 = 2.0;

/// The prior on its offset, 1σ, m, set again each time it is trusted.
pub const GNSS_OFFSET_SD_M: f64 = 1000.0;

/// The offset's random walk, m²/s: a receiver's lead or lag changes as the acceleration does.
pub const GNSS_OFFSET_WALK: f64 = 4.0;

/// The pad's height above sea level taken where the log can't say, m, for the coldest air and
/// slowest sound the barometer's de-weighting allows: higher than most launch sites.
pub const UNKNOWN_PAD_MSL_M: f64 = 3000.0;

/// The fewest barometer samples on the pad that zero it and measure its noise. A log with fewer,
/// as one written from launch detection is, is smoothed on an assumed pad ([`Health::assumed_pad`]).
pub const PAD_MIN_SAMPLES: usize = 5;

/// The prior on the height at a launch-started log's first sample, 1σ, m, about that sample: wide
/// enough that the barometer alone places it, whatever the offset state below holds.
pub const ASSUMED_HEIGHT_SD_M: f64 = 100.0;

/// The prior on the speed at a launch-started log's first sample, 1σ, m/s, about zero: not about
/// the barometer's rate there, whose running median has too few samples at a log's edge to set one
/// garbled sample aside. A rocket that has climbed the 49 m (160 ft) a StratoLogger's launch detection
/// allows at 30 g is doing about 170 m/s, under two of these. HPR Sim's choice.
pub const ASSUMED_SPEED_SD_M_S: f64 = 100.0;

/// The prior on the acceleration at a launch-started log's first sample, 1σ, m/s²: 30 g, about
/// zero, which holds a motor's first spike on most hobby rockets. HPR Sim's choice.
pub const ASSUMED_ACCELERATION_SD_M_S2: f64 = 300.0;

/// The most steps the smoother takes, every sensor's samples together: two sensors at 500 Hz for
/// 33 minutes. The synthetic flights' longest 500 Hz logs pass 500,000, and the longest of the
/// project's sample logs holds 132,230 (0.6 s to smooth on a Mac). The filter holds two covariances
/// a step for the pass back, about 1.5 kB, so a step count without a limit is a memory limit a file
/// could reach: this one comes to about 3 GB and, by the sample log's rate, about 10 s.
pub const MAX_STEPS: usize = 2_000_000;

/// How the GNSS receiver is trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GnssTrust {
    /// At zero weight from liftoff until its speed agrees with the smoother's.
    #[default]
    Agreement,
    /// At its full weight from liftoff, its offset set on the pad: never right for a flight, a
    /// mutation that the synthetic flights' check shows fails.
    FromLiftoff,
}

/// How the smoother runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Options {
    /// How the GNSS receiver is trusted.
    pub gnss_trust: GnssTrust,
}

/// Why a log isn't smoothed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Reason {
    /// The log has no barometer: no height track, or a GNSS receiver's alone.
    NoBarometer,
    /// The readings found no liftoff.
    NoLiftoff,
    /// More than [`MAX_STEPS`] samples.
    TooLong,
    /// The estimate left the finite numbers somewhere in the log: it is withheld whole rather
    /// than given with a hole.
    Diverged,
}

/// A log that isn't smoothed, and why.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Withheld {
    /// The reason, as a code.
    pub reason: Reason,
    /// The reason in words.
    pub detail: String,
}

/// The sensors.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Sensor {
    /// The barometer's height track.
    Barometer,
    /// The axial accelerometer.
    Accelerometer,
    /// The GNSS receiver's altitude.
    Gnss,
}

/// How liftoff was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LiftoffFrom {
    /// The accelerometer's first rise off its pad.
    Accelerometer,
    /// The barometer's track, as the readings find it.
    Barometer,
}

/// Liftoff, as the smoother holds it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Liftoff {
    /// The last moment on the pad, s on the log's clock.
    pub time_s: f64,
    /// The bound on its error, s: from the accelerometer, two of its intervals; from the
    /// barometer, the reading's.
    pub bound_s: f64,
    /// How it was found.
    pub from: LiftoffFrom,
}

/// Why the barometer was de-weighted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Deweighted {
    /// At or above [`TRANSONIC_FROM_MACH`].
    Transonic,
    /// Within [`CHARGE_WINDOW_S`] after a charge.
    Charge,
}

/// A span over which the barometer was de-weighted.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Span {
    /// Its first sample, s on the log's clock.
    pub start_s: f64,
    /// Its last sample, s.
    pub end_s: f64,
    /// Why.
    pub why: Deweighted,
}

/// A rejected sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rejected {
    /// The sensor.
    pub sensor: Sensor,
    /// When, s on the log's clock.
    pub time_s: f64,
    /// Its test ratio: the squared innovation over its variance, over [`GATE_SIGMAS`]²; above 1
    /// is rejected.
    pub ratio: f64,
}

/// A sensor's largest test ratio.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Largest {
    /// The ratio.
    pub ratio: f64,
    /// When, s on the log's clock.
    pub time_s: f64,
    /// The Mach number then, from the filter's vertical speed and the standard atmosphere's
    /// sound at its height.
    pub mach: f64,
}

/// How one sensor fared.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SensorHealth {
    /// The sensor.
    pub sensor: Sensor,
    /// Its samples.
    pub samples: usize,
    /// Those fused.
    pub used: usize,
    /// Those rejected by the gate.
    pub rejected: usize,
    /// Those not weighed: a GNSS fix before the receiver is trusted, an accelerometer's clipped
    /// sample or one past apogee.
    pub unweighed: usize,
    /// How many times the sensor was taken back after [`GATE_RESET_S`] of rejections.
    pub resets: usize,
    /// The share of the samples tested that the gate passed; `None` if none was tested.
    pub inside_gate_share: Option<f64>,
    /// The largest test ratio, where it was.
    pub largest: Option<Largest>,
    /// The mean normalized innovation squared over the samples fused, `ν²/S`, `S` with one
    /// sample's own variance as the gate weighs it: about 1 when the sensor's noise is what the
    /// smoother takes it to be, below 1 when it takes it as noisier.
    pub mean_nis: Option<f64>,
}

/// A log's health card: how each sensor fared against the others (the smoothing note §7, after
/// PX4's and ArduPilot's estimator status).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    /// Each sensor the log holds.
    pub sensors: Vec<SensorHealth>,
    /// The spans the barometer was de-weighted over.
    pub deweighted: Vec<Span>,
    /// Every sample rejected, in time order.
    pub rejected: Vec<Rejected>,
    /// When the GNSS receiver was first trusted, s on the log's clock; `None` if never, or if the
    /// log has none.
    pub gnss_trusted_s: Option<f64>,
    /// The barometer's height less the accelerometer's alone, integrated from liftoff with its
    /// pad's reading taken out, at the smoothed apogee, m; `None` with no accelerometer.
    pub baro_minus_inertial_at_apogee_m: Option<f64>,
    /// Steps where the pass back couldn't solve and kept the forward filter's estimate.
    pub unsmoothed_steps: usize,
    /// Whether the log holds fewer than [`PAD_MIN_SAMPLES`] barometer samples on the pad, as one
    /// written from launch detection does. The pad is then assumed: the height stands on the
    /// readings' zero (`Readings::pad_altitude_m`), the barometer's noise comes from the flight's
    /// own scatter, and no sample is held still.
    pub assumed_pad: bool,
    /// Whether the log's accelerometer had too little of the pad to take its level from, and was
    /// set aside: it isn't in [`Health::sensors`].
    pub accelerometer_set_aside: bool,
    /// The smoothed apogee's height less the reading's, m; `None` where the apogee isn't read.
    pub apogee_less_reading_m: Option<f64>,
    /// Whether that gap passes the reading's bound: the reading's error stays within its bound, so
    /// the track has gone wrong around its top, as it can where a long transonic span leaves it to
    /// the motion model. The bound holds a floor's truth only from below, and a smoothed apogee
    /// above a floor doesn't count.
    pub apogee_past_reading_bound: bool,
}

impl Sensor {
    /// Its name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Barometer => "barometer",
            Self::Accelerometer => "accelerometer",
            Self::Gnss => "GNSS",
        }
    }
}

impl Health {
    /// The card as text: a line each if the pad was assumed or the accelerometer set aside, a line
    /// for each sensor, then the barometer's de-weighted spans, when
    /// the GNSS receiver was first trusted, the barometer's height less the accelerometer's at
    /// apogee, and the pass back. Times are on the log's clock; numbers are to three significant
    /// figures.
    pub fn card(&self) -> String {
        let mut out = String::new();
        if self.assumed_pad {
            out.push_str(
                "pad assumed: too little of it in the log; heights on the logger's zero, noise \
                 from the flight's scatter\n",
            );
        }
        if self.accelerometer_set_aside {
            out.push_str("accelerometer set aside: too little of the pad to take its level from\n");
        }
        for s in &self.sensors {
            out.push_str(&format!(
                "{}: {} samples, {} fused, {} rejected, {} not weighed, {} {}",
                s.sensor.name(),
                s.samples,
                s.used,
                s.rejected,
                s.unweighed,
                s.resets,
                if s.resets == 1 { "reset" } else { "resets" }
            ));
            if let Some(share) = s.inside_gate_share {
                // Rounded down, so a share short of all reads short of 100%.
                let tenths = (1000.0 * share).floor() / 10.0;
                out.push_str(&format!("; {tenths:.1}% inside the gate"));
            }
            if let Some(nis) = s.mean_nis {
                out.push_str(&format!("; mean NIS {}", figures(nis)));
            }
            if let Some(l) = s.largest {
                out.push_str(&format!(
                    "; largest test ratio {} at {} s, Mach {:.2}",
                    figures(l.ratio),
                    figures(l.time_s),
                    l.mach
                ));
            }
            out.push('\n');
        }
        if self.apogee_past_reading_bound
            && let Some(gap) = self.apogee_less_reading_m
        {
            out.push_str(&format!(
                "smoothed apogee {} m from the reading, past the reading's bound: go by the \
                 reading\n",
                figures(gap)
            ));
        }
        out.push_str("barometer de-weighted: ");
        if self.deweighted.is_empty() {
            out.push_str("never");
        } else {
            let spans: Vec<String> = self
                .deweighted
                .iter()
                .map(|span| {
                    let why = match span.why {
                        Deweighted::Transonic => "transonic",
                        Deweighted::Charge => "charge",
                    };
                    format!("{why} {}–{} s", figures(span.start_s), figures(span.end_s))
                })
                .collect();
            out.push_str(&spans.join(", "));
        }
        out.push('\n');
        if self.sensors.iter().any(|s| s.sensor == Sensor::Gnss) {
            match self.gnss_trusted_s {
                Some(t) => out.push_str(&format!("GNSS trusted from {} s\n", figures(t))),
                None => out.push_str("GNSS never trusted\n"),
            }
        }
        if let Some(gap) = self.baro_minus_inertial_at_apogee_m {
            out.push_str(&format!(
                "barometer less accelerometer at apogee: {} m\n",
                figures(gap)
            ));
        }
        if self.unsmoothed_steps == 0 {
            out.push_str("pass back: every step smoothed\n");
        } else {
            out.push_str(&format!(
                "pass back: {} steps kept the forward estimate\n",
                self.unsmoothed_steps
            ));
        }
        out
    }
}

/// `x` to three significant figures, without an exponent; at least its whole part.
fn figures(x: f64) -> String {
    if !x.is_finite() {
        return format!("{x}");
    }
    if x == 0.0 {
        return "0".to_owned();
    }
    #[allow(
        clippy::cast_possible_truncation,
        reason = "x is finite and not zero (returned above), so its decimal exponent is within ±324"
    )]
    let digits = x.abs().log10().floor() as i32;
    #[allow(clippy::cast_sign_loss, reason = "not negative after max(0)")]
    let decimals = (2 - digits).max(0) as usize;
    let text = format!("{x:.decimals$}");
    if text
        .trim_start_matches('-')
        .chars()
        .all(|c| c == '0' || c == '.')
    {
        "0".to_owned()
    } else {
        text
    }
}

/// A flight smoothed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Smoothed {
    /// Each moment, s on the log's clock: every sample of every sensor.
    pub time_s: Vec<f64>,
    /// The height above the pad, m.
    pub height_m: Vec<f64>,
    /// Its standard error, m.
    pub height_sd_m: Vec<f64>,
    /// The vertical speed, m/s, up.
    pub speed_m_s: Vec<f64>,
    /// Its standard error, m/s.
    pub speed_sd_m_s: Vec<f64>,
    /// Liftoff.
    pub liftoff: Liftoff,
    /// The smoothed apogee: when the smoothed height after liftoff is highest, s on the log's
    /// clock.
    pub apogee_time_s: f64,
    /// The health card.
    pub health: Health,
}

/// One moment's smoothed state.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// The height above the pad, m.
    pub height_m: f64,
    /// Its standard error, m.
    pub height_sd_m: f64,
    /// The vertical speed, m/s.
    pub speed_m_s: f64,
    /// Its standard error, m/s.
    pub speed_sd_m_s: f64,
}

impl Smoothed {
    /// The state at `time_s` on the log's clock, linear between the moments either side; `None`
    /// outside the log.
    pub fn at(&self, time_s: f64) -> Option<State> {
        let n = self.time_s.len();
        // A value read back from elsewhere may hold columns of other lengths.
        let columns = [
            &self.height_m,
            &self.height_sd_m,
            &self.speed_m_s,
            &self.speed_sd_m_s,
        ];
        if n == 0
            || columns.iter().any(|c| c.len() != n)
            || !(self.time_s[0]..=self.time_s[n - 1]).contains(&time_s)
        {
            return None;
        }
        let after = self.time_s.partition_point(|&t| t < time_s);
        if after == 0 || self.time_s[after] == time_s {
            return Some(self.state(after));
        }
        let before = after - 1;
        let w = (time_s - self.time_s[before]) / (self.time_s[after] - self.time_s[before]);
        let lerp = |v: &[f64]| v[before] + w * (v[after] - v[before]);
        Some(State {
            height_m: lerp(&self.height_m),
            height_sd_m: lerp(&self.height_sd_m),
            speed_m_s: lerp(&self.speed_m_s),
            speed_sd_m_s: lerp(&self.speed_sd_m_s),
        })
    }

    fn state(&self, i: usize) -> State {
        State {
            height_m: self.height_m[i],
            height_sd_m: self.height_sd_m[i],
            speed_m_s: self.speed_m_s[i],
            speed_sd_m_s: self.speed_sd_m_s[i],
        }
    }
}

/// Smooths a flight log, the GNSS receiver trusted by agreement.
pub fn smooth(log: &FlightLog) -> Result<Smoothed, Withheld> {
    smooth_with(log, &readings::read(log), Options::default())
}

/// Smooths a flight log whose readings are already taken.
///
/// The climb's port coefficient isn't learned ([`PORT_PRIOR_SD`]): the log is smoothed at its
/// prior mean and again a standard deviation off, and each standard error `σ` becomes
/// `hypot(σ, x_off − x)`, the smoother's linear response to the coefficient's spread (a consider
/// analysis: S. F. Schmidt, 1966; D. Simon, *Optimal State Estimation*, Wiley, 2006, §7.6). The
/// second run replays the first's choices, so the difference is the coefficient's effect alone.
pub fn smooth_with(
    log: &FlightLog,
    readings: &Readings,
    options: Options,
) -> Result<Smoothed, Withheld> {
    let inputs = Inputs::gather(log, readings)?;
    // The barometer's row is linear in the coefficient, so one side's step is the other's.
    let (mut smoothed, choices) = run(&inputs, options, PORT_PRIOR_MEAN, None);
    let (off, _) = run(
        &inputs,
        options,
        PORT_PRIOR_MEAN - PORT_PRIOR_SD,
        Some(&choices),
    );
    let pairs = [
        (&mut smoothed.height_sd_m, &smoothed.height_m, &off.height_m),
        (
            &mut smoothed.speed_sd_m_s,
            &smoothed.speed_m_s,
            &off.speed_m_s,
        ),
    ];
    for (sd, at_mean, at_off) in pairs {
        for ((sd, a), b) in sd.iter_mut().zip(at_mean).zip(at_off) {
            *sd = sd.hypot(b - a);
        }
    }
    if let (Reading::Read(apogee), Some(state)) =
        (&readings.apogee, smoothed.at(smoothed.apogee_time_s))
    {
        let gap = state.height_m - apogee.height_m;
        smoothed.health.apogee_less_reading_m = Some(gap);
        smoothed.health.apogee_past_reading_bound = if apogee.is_floor {
            -gap > apogee.bound_m
        } else {
            gap.abs() > apogee.bound_m
        };
    }
    let outputs = [
        &smoothed.height_m,
        &smoothed.height_sd_m,
        &smoothed.speed_m_s,
        &smoothed.speed_sd_m_s,
    ];
    if let Some(k) = (0..smoothed.time_s.len()).find(|&k| outputs.iter().any(|o| !o[k].is_finite()))
    {
        return Err(Withheld {
            reason: Reason::Diverged,
            detail: format!(
                "the smoother's estimate isn't a finite number at {} s",
                smoothed.time_s[k]
            ),
        });
    }
    Ok(smoothed)
}

/// A barometer sample.
#[derive(Debug, Clone, Copy)]
struct Baro {
    time_s: f64,
    /// Above the pad, m.
    height_m: f64,
    /// The running median about it ([`REGRESSOR_HALF_S`]), m.
    smoothed_m: f64,
    /// The running medians' least-squares slope over [`RATE_HALF_S`] either side, m/s: of the
    /// medians, so one garbled sample doesn't bend its neighbors' rates.
    rate_m_s: f64,
    /// The heights' own white scatter about it ([`scatter`]), as a variance, m².
    scatter_var_m2: f64,
}

/// An accelerometer sample.
#[derive(Debug, Clone, Copy)]
struct Accel {
    time_s: f64,
    /// Along the axis, nose up, gravity's reaction in it, m/s².
    value_m_s2: f64,
    /// The running median about it ([`REGRESSOR_HALF_S`]), m/s²: the reading the part's scale
    /// multiplies.
    smoothed_m_s2: f64,
    clipped: bool,
}

/// What the smoother takes from a log.
#[derive(Debug, Clone)]
struct Inputs {
    baro: Vec<Baro>,
    /// The barometer's white noise and step, as a variance at the pad, m².
    baro_white_var: f64,
    /// The pad's height above sea level, m, on the standard atmosphere; `None` where the log
    /// can't say.
    pad_msl_m: Option<f64>,
    /// Whether the pad is assumed ([`Health::assumed_pad`]).
    assumed_pad: bool,
    /// Whether the log's accelerometer was set aside ([`Health::accelerometer_set_aside`]).
    accelerometer_set_aside: bool,
    /// How far the zero the heights stand on may be from the pad on an assumed pad, 1σ, m; 0 where
    /// the zero is the logger's own or the pad's samples set it.
    zero_sd_m: f64,
    accel: Vec<Accel>,
    /// The accelerometer's noise on the pad, 1σ, m/s².
    accel_sd: f64,
    /// Its median on the pad, m/s².
    accel_pad: f64,
    gnss: Vec<(f64, f64)>,
    gnss_sd: f64,
    liftoff: Liftoff,
    /// The pad's hold ends here, s.
    pad_end_s: f64,
    /// The spans the barometer is de-weighted in for the charges the log names, s.
    charges: Vec<(f64, f64)>,
    /// The landing, where the barometer settles, s.
    landing_s: Option<f64>,
    /// From here the barometer's own scatter weighs its samples ([`scatter`]): the end of the
    /// span about its top in which a charge is expected ([`APOGEE_CHARGE_SPAN_M`]), s.
    descent_from_s: f64,
}

/// The landing: the first time past the barometer's highest reading from which its running median
/// stays within [`LANDED_SPAN_M`] for [`LANDED_S`]. `None` if it never does.
fn landing(baro: &[Baro], liftoff_s: f64) -> Option<f64> {
    let top = baro
        .iter()
        .enumerate()
        .filter(|(_, b)| b.time_s > liftoff_s)
        .max_by(|(_, a), (_, b)| a.smoothed_m.total_cmp(&b.smoothed_m))?
        .0;
    let end_s = baro.last()?.time_s;
    (top..baro.len()).find_map(|i| {
        let start = baro[i];
        if end_s < start.time_s + LANDED_S {
            return None;
        }
        baro[i..]
            .iter()
            .take_while(|b| b.time_s <= start.time_s + LANDED_S)
            .all(|b| (b.smoothed_m - start.smoothed_m).abs() <= LANDED_SPAN_M)
            .then_some(start.time_s)
    })
}

/// The median of `values`, which mustn't be empty.
fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    }
}

/// The median and the median absolute deviation scaled to a normal σ (× 1.4826).
fn robust(values: &[f64]) -> (f64, f64) {
    let mut v = values.to_vec();
    let m = median(&mut v);
    let mut dev: Vec<f64> = values.iter().map(|x| (x - m).abs()).collect();
    (m, 1.4826 * median(&mut dev))
}

/// The smallest gap between distinct finite values, or 0 if there are fewer than two.
fn step_of(values: &[f64]) -> f64 {
    let mut v: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
    v.sort_by(f64::total_cmp);
    v.dedup();
    let gap = v
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold(f64::INFINITY, f64::min);
    if gap.is_finite() { gap } else { 0.0 }
}

impl Inputs {
    fn gather(log: &FlightLog, readings: &Readings) -> Result<Self, Withheld> {
        let withheld = |reason: Reason, detail: String| Withheld { reason, detail };
        let track = readings::track(log).map_err(|(_, detail)| {
            withheld(
                Reason::NoBarometer,
                format!("the smoother needs a barometer's height track: {detail}"),
            )
        })?;
        if track.source == Source::Gnss {
            return Err(withheld(
                Reason::NoBarometer,
                "the log's only height is a GNSS receiver's, which lags a climb with its fix \
                 flagged valid; the smoother needs a barometer to check it against"
                    .to_owned(),
            ));
        }
        // Before anything is worked out from the samples: a file's size sets how long this takes.
        let accel_samples_held = readings.accelerometer.as_ref().map_or(0, |settled| {
            log.channels_of(Quantity::Acceleration)
                .find(|c| c.column == settled.column)
                .map_or(0, |c| c.values.len())
        });
        let gnss_samples_held = log
            .channels_of(Quantity::GnssAltitude)
            .next()
            .map_or(0, |c| c.values.len());
        let steps = track.time_s.len() + accel_samples_held + gnss_samples_held;
        if steps > MAX_STEPS {
            return Err(withheld(
                Reason::TooLong,
                format!(
                    "the log holds {steps} samples for the smoother, more than the {MAX_STEPS} \
                     it takes"
                ),
            ));
        }
        let read_liftoff = match &readings.liftoff {
            Reading::Read(liftoff) => *liftoff,
            Reading::Withheld(w) => {
                return Err(withheld(
                    Reason::NoLiftoff,
                    format!("the readings found no liftoff: {}", w.detail),
                ));
            }
        };
        let pad = readings.pad_altitude_m.unwrap_or(0.0);
        let baro: Vec<Baro> = track
            .time_s
            .iter()
            .zip(&track.height_m)
            .filter(|(t, h)| t.is_finite() && h.is_finite())
            .map(|(&time_s, &h)| Baro {
                time_s,
                height_m: h - pad,
                smoothed_m: 0.0,
                rate_m_s: 0.0,
                scatter_var_m2: 0.0,
            })
            .collect();
        let mut baro = baro;
        let heights: Vec<f64> = baro.iter().map(|b| b.height_m).collect();
        let times: Vec<f64> = baro.iter().map(|b| b.time_s).collect();
        let medians = smoothed(&heights, &times);
        let rates = slopes(&medians, &times, RATE_HALF_S);
        for (((b, &m), rate), var) in baro
            .iter_mut()
            .zip(&medians)
            .zip(rates)
            .zip(scatter(&heights, &medians, &times))
        {
            b.smoothed_m = m;
            b.rate_m_s = rate;
            b.scatter_var_m2 = var;
        }
        let pad_msl_m = match track.source {
            Source::Pressure => Some(pad),
            Source::AltitudeChannel if !track.zeroed => Some(pad),
            _ => None,
        };

        // The accelerometer the readings settled, gravity put back and turned nose up.
        let mut accel = Vec::new();
        let mut accel_sd = 0.0;
        let mut accel_pad = G0;
        let mut accelerometer_set_aside = false;
        let mut liftoff = Liftoff {
            time_s: read_liftoff.time_s,
            bound_s: read_liftoff.bound_s,
            from: LiftoffFrom::Barometer,
        };
        if let Some(settled) = &readings.accelerometer
            && let Some(channel) = log
                .channels_of(Quantity::Acceleration)
                .find(|c| c.column == settled.column)
        {
            let (top, range) = match &readings.max_acceleration {
                Reading::Read(max) => (Some((max.acceleration_m_s2, max.clip)), max.range_m_s2),
                Reading::Withheld(_) => (None, None),
            };
            accel = accel_samples(channel, settled.gravity, settled.turned_over);
            let values: Vec<f64> = accel.iter().map(|a| a.value_m_s2).collect();
            let times: Vec<f64> = accel.iter().map(|a| a.time_s).collect();
            for (a, m) in accel.iter_mut().zip(smoothed(&values, &times)) {
                a.smoothed_m_s2 = m;
            }
            mark_clipped(&mut accel, top, range, settled.step_m_s2);
            let pad_until = read_liftoff.time_s - read_liftoff.bound_s - 0.2;
            let on_pad: Vec<f64> = accel
                .iter()
                .filter(|s| s.time_s <= pad_until && !s.clipped)
                .map(|s| s.value_m_s2)
                .collect();
            if on_pad.len() >= PAD_MIN_SAMPLES {
                let (level, sd) = robust(&on_pad);
                accel_pad = level;
                accel_sd = sd.max(settled.step_m_s2 / 12_f64.sqrt()).max(1.0e-3);
                if let Some(found) =
                    accel_liftoff(&accel, level, accel_sd, settled.step_m_s2, &read_liftoff)
                {
                    liftoff = found;
                }
            } else {
                accelerometer_set_aside = !accel.is_empty();
                accel.clear();
            }
        }

        let apogee_span = apogee_charge(&baro, liftoff.time_s);
        // Past the barometer's own top and the charge expected there, not from the filter's
        // apogee: the charge's pulse can hold the filter's decision back while the swing has
        // begun.
        let descent_from_s = apogee_span.map_or(f64::INFINITY, |(_, end)| end);

        let found_pad_end_s = match liftoff.from {
            LiftoffFrom::Accelerometer => liftoff.time_s,
            LiftoffFrom::Barometer => liftoff.time_s - liftoff.bound_s,
        };
        let on_pad: Vec<f64> = baro
            .iter()
            .filter(|s| s.time_s <= found_pad_end_s)
            .map(|s| s.height_m)
            .collect();
        let step = step_of(&track.height_m);
        // The pad's scatter holds the fast and slow correlated noise too, which the smoother
        // carries apart; what is left is the part's white noise. A step's own variance is the
        // least it can be.
        let correlated = FAST_NOISE_M * FAST_NOISE_M + SLOW_NOISE_M * SLOW_NOISE_M;
        let assumed_pad = on_pad.len() < PAD_MIN_SAMPLES;
        let measured_var = if assumed_pad {
            flight_noise_var(&baro, descent_from_s, pad_msl_m.unwrap_or(0.0), correlated)
        } else {
            let (_, pad_sd) = robust(&on_pad);
            pad_sd * pad_sd - correlated
        };
        let baro_white_var = measured_var.max(step * step / 12.0).max(1.0e-6);
        // With no pad, nothing is held still: a sample near the zero may already be moving.
        let pad_end_s = if assumed_pad {
            f64::NEG_INFINITY
        } else {
            found_pad_end_s
        };
        // A zero the logger set is the pad's; one that is the pad's only by a mapping's or a
        // column's word sits within `PAD_ZERO_M` of it ([`readings::PAD_ZERO_M`]), which the
        // readings' bounds hold and the height's standard error must too.
        let zero_sd_m = if assumed_pad && readings::zero_by_word(log, &track) {
            readings::PAD_ZERO_M / 3_f64.sqrt()
        } else {
            0.0
        };

        let mut gnss: Vec<(f64, f64)> = Vec::new();
        // A receiver that logs past the barometer's last sample would leave the track there to
        // the motion model and a receiver not yet trusted, so its fixes are taken within the
        // barometer's span.
        let baro_end_s = baro.last().map_or(f64::NEG_INFINITY, |b| b.time_s);
        if let Some(channel) = log.channels_of(Quantity::GnssAltitude).next() {
            for (&t, &z) in channel.time_s.iter().zip(&channel.values) {
                if t.is_finite()
                    && z.is_finite()
                    && t <= baro_end_s
                    && gnss.last().is_none_or(|&(last, _)| t > last)
                {
                    gnss.push((t, z));
                }
            }
        }
        let gnss_pad: Vec<f64> = gnss
            .iter()
            .filter(|(t, _)| *t <= pad_end_s)
            .map(|(_, z)| *z)
            .collect();
        let gnss_sd = if gnss_pad.len() >= 3 {
            robust(&gnss_pad).1.max(GNSS_SD_FLOOR_M)
        } else {
            GNSS_SD_DEFAULT_M
        };

        let mut charges: Vec<(f64, f64)> = log
            .events
            .iter()
            .filter(|e| {
                let name = e.name.to_lowercase();
                e.time_s.is_finite() && CHARGE_WORDS.iter().any(|w| name.contains(w))
            })
            .map(|e| (e.time_s, e.time_s + CHARGE_WINDOW_S))
            .collect();
        charges.extend(apogee_span);

        let landing_s = landing(&baro, liftoff.time_s);
        Ok(Self {
            landing_s,
            descent_from_s,
            baro,
            baro_white_var,
            pad_msl_m,
            assumed_pad,
            accelerometer_set_aside,
            zero_sd_m,
            accel,
            accel_sd,
            accel_pad,
            gnss,
            gnss_sd,
            liftoff,
            pad_end_s,
            charges,
        })
    }
}

/// The barometer's white noise on a log with no pad, as a variance at the pad's air, m²: the median
/// over the samples past `descent_from_s` (all of them if there are none) of the flight's own
/// scatter ([`scatter`]), brought to the pad's air as [`Inputs`]' noise is thinned from it, with the
/// correlated noise the smoother carries apart taken out. Under a canopy the scatter also holds the
/// swing, so this errs noisy, the side that widens an interval; a log that runs on past landing has
/// a median nearer the part's own noise.
fn flight_noise_var(baro: &[Baro], descent_from_s: f64, pad_msl_m: f64, correlated: f64) -> f64 {
    let rho_pad = standard_density(pad_msl_m);
    let at_pad = |b: &Baro| {
        let thin = (rho_pad / standard_density(pad_msl_m + b.smoothed_m)).powi(2);
        b.scatter_var_m2 / thin - correlated
    };
    let mut past: Vec<f64> = baro
        .iter()
        .filter(|b| b.time_s >= descent_from_s)
        .map(at_pad)
        .collect();
    if past.is_empty() {
        past = baro.iter().map(at_pad).collect();
    }
    if past.is_empty() {
        return 0.0;
    }
    median(&mut past)
}

/// The span around the barometer's apogee in which a charge is expected
/// ([`APOGEE_CHARGE_SPAN_M`]); `None` if the track has no sample after liftoff.
fn apogee_charge(baro: &[Baro], liftoff_s: f64) -> Option<(f64, f64)> {
    let after = || baro.iter().filter(|b| b.time_s > liftoff_s);
    let top = after().map(|b| b.smoothed_m).max_by(f64::total_cmp)?;
    let near = || after().filter(|b| b.smoothed_m >= top - APOGEE_CHARGE_SPAN_M);
    let first = near().next()?.time_s;
    let last = near().next_back()?.time_s;
    Some((first, last + CHARGE_WINDOW_S))
}

/// The channel's samples, gravity put back and nose up.
fn accel_samples(channel: &Channel, gravity: Gravity, turned_over: bool) -> Vec<Accel> {
    let mut out: Vec<Accel> = Vec::with_capacity(channel.values.len());
    for (&t, &v) in channel.time_s.iter().zip(&channel.values) {
        if !(t.is_finite() && v.is_finite()) || out.last().is_some_and(|last| t <= last.time_s) {
            continue;
        }
        let mut value = if turned_over { -v } else { v };
        if gravity == Gravity::Out {
            value += G0;
        }
        out.push(Accel {
            time_s: t,
            value_m_s2: value,
            smoothed_m_s2: value,
            clipped: false,
        });
    }
    out
}

/// The half-span of the running median a scale state's regressor is taken from, s. A scale
/// multiplies the quantity it scales; taken from the state, the product makes the filter extended,
/// and its linearization finds information along directions the sensors can't see (the scale and
/// the acceleration it multiplies, with the barometer de-weighted); taken from the raw sample, the
/// sample's own noise is fitted by the scale (a pad's noise, read as a height that scales). The
/// median keeps the track's shape and takes most of its noise out. HPR Sim's choice.
pub const REGRESSOR_HALF_S: f64 = 0.05;

/// The barometer has landed once its running median stays within this many meters ...
pub const LANDED_SPAN_M: f64 = 5.0;

/// ... for this many seconds: under the slowest main canopies, 2 m/s, a descent moves 6 m in it.
pub const LANDED_S: f64 = 3.0;

/// The impact is taken back this long before the landing's first settled reading, s: the running
/// median lags a stop by its half-width, and a ballistic stop takes a few hundredths of a second.
pub const LANDING_LEAD_S: f64 = 0.1;

/// The barometer's own climb rate is taken over this many seconds either side of a sample, for
/// the Mach number its de-weighting ends at, the burnout's and apogee's rules, and the speed the
/// port's term is taken at, which keeps the barometer's row linear in the states: its noise there
/// is a few m/s.
pub const RATE_HALF_S: f64 = 0.25;

/// The time over which the barometer's scatter in flight beyond the pad's is correlated, s: a
/// rocket swinging under its canopy on `L` = 3–15 m of cord and lines is a pendulum of period
/// `2π √(L/g)`, 3.5–7.8 s, and a first-order error of `τ = √(L/g)`, 0.55–1.24 s, averages away no
/// faster than its swing does. HPR Sim's estimate, taken past that range because the scatter's
/// quadratic takes part of a correlated error's variance out: with 2 s, descents swinging by
/// 3–6 m correlated over up to a second stay inside their 90% intervals in the tests, and one
/// correlated over 3 s doesn't. The noise note gives no correlation time for the swing.
pub const SWING_TAU_S: f64 = 2.0;

/// A barometer's scatter is taken over this many seconds either side of a sample, and over at
/// least [`SCATTER_MIN_SAMPLES`] samples either side: long against [`SWING_TAU_S`], so a
/// quadratic through the window can't follow a swing and take its variance out, and long enough
/// that a charge's pulse or a spike is a minority of it. Where the track bends faster than a
/// quadratic over six seconds, as a fall settles to its canopy's speed or the main opens, the
/// scatter reads the bend as noise too, and the barometer is weighed less there than it could be.
pub const SCATTER_HALF_S: f64 = 3.0;

/// The fewest samples either side a barometer's scatter is taken over.
pub const SCATTER_MIN_SAMPLES: usize = 5;

/// The most samples a scatter's window is read at, evenly spaced: enough for its median to
/// scatter by a tenth, and a fast logger's window no slower than a slow one's.
pub const SCATTER_MAX_SAMPLES: usize = 201;

/// Each sample's scatter, as a variance (m²): the heights' robust spread about the least-squares
/// quadratic through their running `medians` over [`SCATTER_HALF_S`] either side (so a spike
/// doesn't pull it), the median absolute residual scaled to a normal σ (× 1.4826). A quadratic follows a coast's or a steady descent's track to well
/// under a meter, so what is left is the barometer's noise in flight.
///
/// The pad's scatter measures the sensor; in flight a rocket swinging under its canopy and the
/// gusts on it add their own, correlated over a second or so: 0.3–7.5 m robust σ about
/// two-second quadratic fits on two public dual-altimeter pairs' descents
/// (`docs/research/altimeter-sensor-noise.md` §3). From the track's top on, past the charge expected
/// there, where those were measured, the
/// smoother weighs each sample by the pad's noise and the scatter beyond it, correlated over
/// [`SWING_TAU_S`], so a quiet pad doesn't make a swinging descent's samples fail the gate or
/// average a swing away. Before apogee the pad's noise and the transonic and charge de-weighting
/// stand alone. The fast
/// noise ([`FAST_NOISE_M`]), white at a logger's rates, is in the scatter whole and is taken out
/// of it there, as the pad's is.
fn scatter(values: &[f64], medians: &[f64], times: &[f64]) -> Vec<f64> {
    let n = values.len();
    let (mut residuals, mut picked) = (Vec::new(), Vec::new());
    (0..n)
        .map(|i| {
            let mut from = i.saturating_sub(SCATTER_MIN_SAMPLES);
            while from > 0 && times[i] - times[from - 1] <= SCATTER_HALF_S {
                from -= 1;
            }
            let mut to = (i + SCATTER_MIN_SAMPLES).min(n.saturating_sub(1));
            while to + 1 < n && times[to + 1] - times[i] <= SCATTER_HALF_S {
                to += 1;
            }
            // Every `stride`-th sample of the window.
            let stride = (to - from) / SCATTER_MAX_SAMPLES + 1;
            picked.clear();
            picked.extend((from..=to).filter(|j| (j - from) % stride == 0));
            let t: Vec<f64> = picked.iter().map(|&j| times[j]).collect();
            let m: Vec<f64> = picked.iter().map(|&j| medians[j]).collect();
            let Some(fit) = quadratic(&m, &t, times[i]) else {
                return 0.0;
            };
            residuals.clear();
            residuals.extend(picked.iter().map(|&j| {
                let tau = times[j] - times[i];
                (values[j] - (fit[0] + fit[1] * tau + fit[2] * tau * tau)).abs()
            }));
            let sd = 1.4826 * selected_median(&mut residuals);
            sd * sd
        })
        .collect()
}

/// The median of `values`, which mustn't be empty, by selection rather than a sort: the same
/// number as [`median`]'s, in linear time.
fn selected_median(values: &mut [f64]) -> f64 {
    let n = values.len();
    let (lower, &mut upper, _) = values.select_nth_unstable_by(n / 2, f64::total_cmp);
    if n % 2 == 1 {
        upper
    } else {
        // The lower middle is the largest of the half below.
        0.5 * (lower.iter().copied().fold(f64::NEG_INFINITY, f64::max) + upper)
    }
}

/// The least-squares quadratic `c₀ + c₁ τ + c₂ τ²`, `τ = t − center`, through the points, by
/// Cramer's rule on the normal equations; `None` with fewer than four points or three distinct
/// times' worth of spread.
fn quadratic(values: &[f64], times: &[f64], center: f64) -> Option<[f64; 3]> {
    if values.len() < 4 {
        return None;
    }
    // Σ τᵏ for k = 0..4 and Σ z τᵏ for k = 0..2, τ scaled by the window's half-width so the
    // determinant's terms are of one size.
    let scale = times
        .iter()
        .map(|t| (t - center).abs())
        .fold(0.0, f64::max)
        .max(1.0e-9);
    let (mut s, mut b) = ([0.0; 5], [0.0; 3]);
    for (&z, &t) in values.iter().zip(times) {
        let u = (t - center) / scale;
        let mut power = 1.0;
        for (k, sk) in s.iter_mut().enumerate() {
            *sk += power;
            if k < 3 {
                b[k] += z * power;
            }
            power *= u;
        }
    }
    let m = [[s[0], s[1], s[2]], [s[1], s[2], s[3]], [s[2], s[3], s[4]]];
    let det = |m: &[[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let d = det(&m);
    if d.is_nan() || d.abs() <= 1.0e-9 * s[0].powi(3) {
        return None;
    }
    let mut c = [0.0; 3];
    for (k, ck) in c.iter_mut().enumerate() {
        let mut mk = m;
        for row in 0..3 {
            mk[row][k] = b[row];
        }
        *ck = det(&mk) / d;
    }
    Some([c[0], c[1] / scale, c[2] / (scale * scale)])
}

/// Each sample's least-squares slope of `values` against `times` over `half_s` either side; zero
/// where the window holds fewer than two distinct times.
fn slopes(values: &[f64], times: &[f64], half_s: f64) -> Vec<f64> {
    let origin = times.first().copied().unwrap_or(0.0);
    let (mut from, mut to) = (0, 0);
    let (mut n, mut st, mut sz, mut stt, mut stz) = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut out = Vec::with_capacity(values.len());
    for &center in times {
        while to < times.len() && times[to] <= center + half_s {
            let t = times[to] - origin;
            n += 1.0;
            st += t;
            sz += values[to];
            stt += t * t;
            stz += t * values[to];
            to += 1;
        }
        while times[from] < center - half_s {
            let t = times[from] - origin;
            n -= 1.0;
            st -= t;
            sz -= values[from];
            stt -= t * t;
            stz -= t * values[from];
            from += 1;
        }
        let spread = n * stt - st * st;
        out.push(if n >= 2.0 && spread > 1.0e-12 * n * n {
            (n * stz - st * sz) / spread
        } else {
            0.0
        });
    }
    out
}

/// The running median of `values` over [`REGRESSOR_HALF_S`] either side, by the median interval.
fn smoothed(values: &[f64], times: &[f64]) -> Vec<f64> {
    if values.len() < 3 {
        return values.to_vec();
    }
    let mut gaps: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
    let interval = median(&mut gaps).max(1.0e-6);
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the interval is at least 10⁻⁶ s, so this is 0 to 5 × 10⁴"
    )]
    let half = ((REGRESSOR_HALF_S / interval).round() as usize).max(1);
    crate::filter::running_median(values, half)
}

/// Marks the samples the part couldn't read: at its stated range less two steps and
/// [`GRAVITY_TOLERANCE_G`], as the readings take a top as clipped, or on a flat top the readings
/// found.
fn mark_clipped(
    accel: &mut [Accel],
    top: Option<(f64, Option<Clip>)>,
    range: Option<f64>,
    step: f64,
) {
    let limit = range.map(|r| r - 2.0 * step - GRAVITY_TOLERANCE_G * G0);
    let flat = match top {
        Some((value, Some(Clip::Flat))) => Some(value - step),
        _ => None,
    };
    for sample in accel {
        let reading = sample.value_m_s2;
        sample.clipped =
            limit.is_some_and(|l| reading.abs() >= l) || flat.is_some_and(|f| reading >= f);
    }
}

/// Liftoff from the accelerometer: the last sample before the first run of three that rise past
/// five of the pad's σ and a step above its level, and at least 3 m/s², within a second of the
/// barometer's liftoff and its bound; walked back while the samples stay above twice σ and a
/// step. On the pad an accelerometer reads 1 g however hard the motor pushes, until the rocket
/// moves.
fn accel_liftoff(
    accel: &[Accel],
    level: f64,
    sd: f64,
    step: f64,
    read: &readings::Liftoff,
) -> Option<Liftoff> {
    let rise = (5.0 * sd + step).max(3.0);
    let from = read.time_s - read.bound_s - 1.0;
    let to = read.time_s + read.bound_s + 1.0;
    let above = |s: &Accel| s.clipped || s.value_m_s2 - level > rise;
    let start = accel.iter().position(|s| s.time_s >= from)?;
    let mut first = None;
    for i in start..accel.len().saturating_sub(2) {
        if accel[i].time_s > to {
            break;
        }
        if above(&accel[i]) && above(&accel[i + 1]) && above(&accel[i + 2]) {
            first = Some(i);
            break;
        }
    }
    let mut i = first?;
    while i > 0 && accel[i - 1].value_m_s2 - level > 2.0 * sd + step {
        i -= 1;
    }
    let last_on_pad = i.checked_sub(1)?;
    let interval = accel[i].time_s - accel[last_on_pad].time_s;
    Some(Liftoff {
        time_s: accel[last_on_pad].time_s,
        bound_s: 2.0 * interval,
        from: LiftoffFrom::Accelerometer,
    })
}

/// The standard atmosphere's temperature at a height above sea level, K: its lapse to 11 km, then
/// the isothermal layer (taken to hold above it).
fn standard_temperature_k(height_msl_m: f64) -> f64 {
    (288.15 - 0.0065 * height_msl_m.min(11_000.0)).max(216.65)
}

/// `T_pad ∫ dz/T(z)` from the pad to `height_m` above it, on the standard atmosphere, m: the height
/// a barometer's error from the day's air scales with. On a day `ΔT` off the standard, a logger
/// converting pressure by the standard atmosphere reads `h − (ΔT/T_pad) ψ`, from the hypsometric
/// equation `dz = −(R T/g) dp/p` (`altimeter-height-formulas.md`).
fn air_regressor(height_m: f64, pad_msl_m: f64) -> f64 {
    let t_pad = standard_temperature_k(pad_msl_m);
    let integral = |z: f64| -> f64 {
        // ∫₀^z dz/T from sea level.
        let troposphere = z.min(11_000.0);
        let lower = -(standard_temperature_k(troposphere) / 288.15).ln() / 0.0065;
        lower + (z - 11_000.0).max(0.0) / 216.65
    };
    t_pad * (integral(pad_msl_m + height_m) - integral(pad_msl_m))
}

/// The standard atmosphere's density at a height above sea level, kg/m³.
fn standard_density(height_msl_m: f64) -> f64 {
    Ussa76::standard()
        .sample(height_msl_m)
        .map_or(1.225, |s| s.air.density_kg_m3)
}

/// The speed of sound at a temperature, m/s.
fn sound(temperature_k: f64) -> f64 {
    (1.4 * 287.052_87 * temperature_k).sqrt()
}

/// The Mach number estimated high, for de-weighting: the vertical speed two of its standard
/// errors high, in air [`AIR_TEMPERATURE_SPREAD_K`] colder than the standard, the axis up to
/// [`RAIL_TILT_DEG`] off the vertical.
fn mach_high(speed: f64, speed_sd: f64, height_msl_m: f64) -> f64 {
    let temperature = standard_temperature_k(height_msl_m) - AIR_TEMPERATURE_SPREAD_K;
    (speed.abs() + 2.0 * speed_sd) / (sound(temperature) * RAIL_TILT_DEG.to_radians().cos())
}

/// The barometer's transonic span: opened once both the filter's speed and the barometer's own
/// rate say transonic, and closed on the rate alone, as in the span the filter's speed spreads
/// and would hold it open, or open it again, on its own spread.
#[derive(Debug, Clone, Copy, Default)]
struct Transonic {
    open: bool,
    /// Since when the barometer's rate has said subsonic, in an open span.
    clear_since_s: Option<f64>,
}

impl Transonic {
    /// The port coefficient's possible shift at this sample, and the speed it multiplies.
    fn weigh(&mut self, t: f64, v: f64, v_sd: f64, rate_m_s: f64, height_msl_m: f64) -> (f64, f64) {
        if !self.open {
            let filter = mach_high(v, v_sd, height_msl_m);
            let rate = mach_high(rate_m_s, 0.0, height_msl_m);
            let shift = transonic_shift(filter.min(rate));
            self.open = shift >= TRANSONIC_PORT_SHIFT;
            return (
                shift,
                (v.abs() + 2.0 * v_sd).min(rate_m_s.abs().max(v.abs())),
            );
        }
        // Open, the speed is the barometer's or the span's least, whichever is more.
        let cold = standard_temperature_k(height_msl_m) - AIR_TEMPERATURE_SPREAD_K;
        let least = TRANSONIC_FULL_MACH * sound(cold) * RAIL_TILT_DEG.to_radians().cos();
        if mach_high(rate_m_s, 0.0, height_msl_m) < TRANSONIC_FROM_MACH {
            let since = *self.clear_since_s.get_or_insert(t);
            if t - since >= TRANSONIC_CLEAR_S {
                self.open = false;
                self.clear_since_s = None;
                return (0.0, 0.0);
            }
        } else {
            self.clear_since_s = None;
        }
        (TRANSONIC_PORT_SHIFT, rate_m_s.abs().max(least))
    }
}

/// The burn is taken as over once the barometer's rate has fallen this share below its peak: a
/// coast's drag and gravity take a tenth of the speed within a few seconds, while the rate's own
/// noise is a few m/s. HPR Sim's choice.
pub const RATE_DROP_SHARE: f64 = 0.1;

/// An open transonic span closes once the barometer's rate has said subsonic this long, s: a
/// shock crossing the port swings the rate both ways for a few tenths of a second.
pub const TRANSONIC_CLEAR_S: f64 = 1.0;

/// The port's coefficient's possible shift at that Mach number.
fn transonic_shift(mach: f64) -> f64 {
    let w = ((mach - TRANSONIC_FROM_MACH) / (TRANSONIC_FULL_MACH - TRANSONIC_FROM_MACH))
        .clamp(0.0, 1.0);
    w * TRANSONIC_PORT_SHIFT
}

/// What happened to one sample at the gate.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Gated {
    Used,
    Rejected,
}

/// A sensor's running tally.
#[derive(Debug, Clone)]
struct Tally {
    health: SensorHealth,
    nis_sum: f64,
    rejecting_since: Option<f64>,
}

impl Tally {
    fn new(sensor: Sensor) -> Self {
        Self {
            health: SensorHealth {
                sensor,
                samples: 0,
                used: 0,
                rejected: 0,
                unweighed: 0,
                resets: 0,
                inside_gate_share: None,
                largest: None,
                mean_nis: None,
            },
            nis_sum: 0.0,
            rejecting_since: None,
        }
    }

    fn finish(mut self) -> SensorHealth {
        let tested = self.health.used + self.health.rejected;
        if tested > 0 {
            self.health.inside_gate_share = Some(self.health.used as f64 / tested as f64);
        }
        if self.health.used > 0 {
            self.health.mean_nis = Some(self.nis_sum / self.health.used as f64);
        }
        self.health
    }
}

/// The filter's running state and what it records for the pass back.
struct Filter {
    x: Vector,
    p: Matrix,
    /// What this step added to the covariance after its prediction was stored: a sensor taken
    /// back, a GNSS receiver newly trusted. It is added to the stored prediction too, as process
    /// noise of the step, so the pass back sees the covariance the filter used.
    jump: Matrix,
    /// The states no measurement moves now: the day's air without a GNSS receiver, and from the
    /// first apogee the accelerometer's, which nothing after it measures; a landing's impact would otherwise move
    /// them through their correlations with the speed and carry that back over the climb.
    held: [bool; N],
    rejected: Vec<Rejected>,
    pad_msl_for_sound_m: f64,
    /// Each sample's fate at the gate, in the order fused.
    gates: Vec<Gate>,
    /// A run's gate outcomes to replay, and the next one's index.
    replay: Option<(Vec<Gate>, usize)>,
}

/// A sample's fate at the gate.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Gate {
    Used,
    Rejected,
    /// Rejected for [`GATE_RESET_S`] and taken back, the state widened.
    TakenBack,
}

/// The discrete choices a run makes from its own estimates: each step's phase and whether the
/// first apogee has passed, each barometer sample's transonic de-weighting, each GNSS fix's
/// trust, and each sample's fate at the gate. The consider run replays the first run's, so the
/// difference between the two is the port coefficient's effect alone, not that of a span closed
/// a sample later or a fix rejected in one run and not the other.
#[derive(Debug, Clone, Default, PartialEq)]
struct Choices {
    /// Each step after the first: its phase, and whether the first apogee has passed.
    steps: Vec<(Phase, bool)>,
    /// Each barometer sample's transonic shift and the speed it multiplies.
    transonic: Vec<(f64, f64)>,
    /// Each GNSS fix: trusted, trust begun, settling.
    gnss: Vec<(bool, bool, bool)>,
    /// Each sample's fate at the gate, in the order fused.
    gates: Vec<Gate>,
}

/// A measurement's variance: as fused, and as one sample's own for the gate.
#[derive(Debug, Clone, Copy)]
struct Variance {
    /// As fused: a correlated error's variance grown by its correlation over the samples it
    /// spans, so a fast logger's many samples don't average it away.
    fused: f64,
    /// One sample's own, which its innovation is gated against: grown, it would let a pulse
    /// through on a fast logger that a slow one rejects.
    gate: f64,
}

impl Variance {
    /// A white error's, the same both ways.
    fn white(variance: f64) -> Self {
        Self {
            fused: variance,
            gate: variance,
        }
    }
}

impl Filter {
    /// Gates and fuses one scalar measurement.
    fn fuse(
        &mut self,
        tally: &mut Tally,
        time_s: f64,
        h: &Vector,
        y: f64,
        variance: Variance,
        reset_after_s: Option<f64>,
    ) -> Gated {
        let r = variance.fused;
        let s = kalman::innovation_variance(&self.p, h, variance.gate);
        let nis = y * y / s;
        let ratio = nis / (GATE_SIGMAS * GATE_SIGMAS);
        if tally.health.largest.is_none_or(|l| ratio > l.ratio) {
            let height_msl = self.pad_msl_for_sound_m + self.x[H];
            let mach = self.x[V].abs() / sound(standard_temperature_k(height_msl));
            tally.health.largest = Some(Largest {
                ratio,
                time_s,
                mach,
            });
        }
        let replayed = self.replay.as_mut().and_then(|(gates, at)| {
            let gate = gates.get(*at).copied();
            *at += 1;
            gate
        });
        let gate = replayed.unwrap_or_else(|| {
            if ratio <= 1.0 {
                return Gate::Used;
            }
            // An innovation past the numbers can't size the widening a reset takes.
            if !y.is_finite() {
                return Gate::Rejected;
            }
            let since = *tally.rejecting_since.get_or_insert(time_s);
            if reset_after_s.is_some_and(|after| time_s - since >= after) {
                Gate::TakenBack
            } else {
                Gate::Rejected
            }
        });
        self.gates.push(gate);
        match gate {
            Gate::Used => {}
            Gate::Rejected => {
                tally.health.rejected += 1;
                self.rejected.push(Rejected {
                    sensor: tally.health.sensor,
                    time_s,
                    ratio,
                });
                return Gated::Rejected;
            }
            Gate::TakenBack => {
                // The height, speed and acceleration widened by what the innovation says they
                // missed, the gap closed over the time rejected.
                self.widen(H, y * y);
                self.widen(V, (y / GATE_RESET_S).powi(2));
                self.widen(A, (2.0 * y / (GATE_RESET_S * GATE_RESET_S)).powi(2));
                tally.health.resets += 1;
            }
        }
        tally.rejecting_since = None;
        kalman::update(&mut self.x, &mut self.p, h, y, r, &self.held);
        tally.health.used += 1;
        // Against one sample's own variance, as the gate weighs it: near 1 when the sensor's
        // noise is what the smoother takes it to be, though a correlated error is fused grown.
        tally.nis_sum += nis;
        Gated::Used
    }
}

impl Filter {
    /// Adds `variance` to a state's, as process noise of this step.
    fn widen(&mut self, state: usize, variance: f64) {
        self.p[state][state] += variance;
        self.jump[state][state] += variance;
    }
}

/// The transition over `dt` seconds.
fn transition(dt: f64) -> Matrix {
    let mut f = kalman::identity();
    f[H][V] = dt;
    f[H][A] = 0.5 * dt * dt;
    f[V][A] = dt;
    f[TILT][TILT] = (-dt / TILT_TAU_S).exp();
    f[SLOW][SLOW] = (-dt / SLOW_NOISE_TAU_S).exp();
    f[DRIFT][DRIFT_RATE] = dt;
    f
}

/// The process noise over `dt` seconds.
fn process_noise(dt: f64, jerk: f64, tilt_sd: f64, gnss_walk: f64) -> Matrix {
    let mut q = kalman::ZERO;
    let (d2, d3, d4, d5) = (dt * dt, dt.powi(3), dt.powi(4), dt.powi(5));
    q[H][H] = jerk * d5 / 20.0;
    q[H][V] = jerk * d4 / 8.0;
    q[H][A] = jerk * d3 / 6.0;
    q[V][V] = jerk * d3 / 3.0;
    q[V][A] = jerk * d2 / 2.0;
    q[A][A] = jerk * dt;
    q[V][H] = q[H][V];
    q[A][H] = q[H][A];
    q[A][V] = q[V][A];
    q[TILT][TILT] = tilt_sd * tilt_sd * (1.0 - (-2.0 * dt / TILT_TAU_S).exp());
    q[SLOW][SLOW] = SLOW_NOISE_M * SLOW_NOISE_M * (1.0 - (-2.0 * dt / SLOW_NOISE_TAU_S).exp());
    q[GNSS][GNSS] = gnss_walk * dt;
    q
}

/// One step's grid: what is measured then.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Measured {
    Accel(usize),
    Baro(usize),
    Gnss(usize),
}

/// The grid: every sample's time, and the samples taken then, in time order.
fn grid(inputs: &Inputs) -> Vec<(f64, Vec<Measured>)> {
    let mut all: Vec<(f64, Measured)> = Vec::new();
    all.extend(
        inputs
            .accel
            .iter()
            .enumerate()
            .map(|(i, s)| (s.time_s, Measured::Accel(i))),
    );
    all.extend(
        inputs
            .baro
            .iter()
            .enumerate()
            .map(|(i, s)| (s.time_s, Measured::Baro(i))),
    );
    all.extend(
        inputs
            .gnss
            .iter()
            .enumerate()
            .map(|(i, s)| (s.0, Measured::Gnss(i))),
    );
    all.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut out: Vec<(f64, Vec<Measured>)> = Vec::new();
    for (t, m) in all {
        match out.last_mut() {
            Some((last, list)) if *last == t => list.push(m),
            _ => out.push((t, vec![m])),
        }
    }
    out
}

/// The phase that sets the white jerk.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    Pad,
    Powered,
    Coast,
    Descent,
}

/// The GNSS receiver's trust, as the filter runs.
#[derive(Debug, Clone)]
struct Trust {
    mode: GnssTrust,
    trusted: bool,
    /// When the receiver was last trusted.
    since_s: Option<f64>,
    /// The first time it was.
    first_s: Option<f64>,
    /// The start of the current run of agreeing fixes.
    agree_from_s: Option<f64>,
    /// The fixes after liftoff, kept for the window's slope.
    recent: Vec<(f64, f64)>,
    misses: usize,
}

impl Trust {
    /// Weighs a fix at `t` against the filter's speed `v` (standard error `v_sd`); returns whether
    /// the receiver is trusted now, and whether trust began at this fix.
    fn weigh(
        &mut self,
        t: f64,
        z: f64,
        v: f64,
        v_sd: f64,
        sd: f64,
        liftoff_s: f64,
    ) -> (bool, bool) {
        if self.mode == GnssTrust::FromLiftoff {
            return (true, false);
        }
        if t <= liftoff_s {
            return (false, false);
        }
        if self
            .recent
            .last()
            .is_some_and(|&(last, _)| t - last > GNSS_GAP_S)
        {
            self.recent.clear();
            self.agree_from_s = None;
            self.trusted = false;
        }
        self.recent.push((t, z));
        self.recent.retain(|&(s, _)| t - s <= GNSS_AGREE_WINDOW_S);
        if self.trusted {
            return (true, false);
        }
        let agrees = self.recent.len() >= 3 && {
            let n = self.recent.len() as f64;
            let mean_t = self.recent.iter().map(|p| p.0).sum::<f64>() / n;
            let mean_z = self.recent.iter().map(|p| p.1).sum::<f64>() / n;
            let stt: f64 = self.recent.iter().map(|p| (p.0 - mean_t).powi(2)).sum();
            let stz: f64 = self
                .recent
                .iter()
                .map(|p| (p.0 - mean_t) * (p.1 - mean_z))
                .sum();
            let slope = stz / stt;
            let slope_sd = sd / stt.sqrt();
            let gap = (slope - v).abs();
            gap <= GNSS_AGREE_M_S && gap <= 2.0 * (slope_sd * slope_sd + v_sd * v_sd).sqrt()
        };
        if !agrees {
            self.agree_from_s = None;
            return (false, false);
        }
        let from = *self.agree_from_s.get_or_insert(t);
        if t - from >= GNSS_AGREE_WINDOW_S {
            self.trusted = true;
            self.since_s = Some(t);
            self.first_s.get_or_insert(t);
            return (true, true);
        }
        (false, false)
    }
}

fn run(
    inputs: &Inputs,
    options: Options,
    climb_port: f64,
    replay: Option<&Choices>,
) -> (Smoothed, Choices) {
    let mut choices = Choices::default();
    let steps = grid(inputs);
    let n = steps.len();
    let pad_msl_for_air_m = inputs.pad_msl_m.unwrap_or(0.0);
    let pad_msl_for_sound_m = inputs.pad_msl_m.unwrap_or(UNKNOWN_PAD_MSL_M);
    let rho_pad = standard_density(pad_msl_for_air_m);

    // The prior.
    let mut x = [0.0; N];
    let mut p = kalman::ZERO;
    p[H][H] = 1.0;
    p[V][V] = 1.0;
    p[A][A] = 1.0;
    let has_accel = !inputs.accel.is_empty();
    let has_gnss = !inputs.gnss.is_empty();
    // States a log can't move keep a unit variance and are never touched.
    p[BIAS][BIAS] = if has_accel {
        (ACCEL_BIAS_M_S2 / 3_f64.sqrt()).powi(2)
    } else {
        1.0
    };
    p[SCALE][SCALE] = if has_accel {
        (ACCEL_SCALE_SHARE / 3_f64.sqrt()).powi(2)
    } else {
        1.0
    };
    p[TILT][TILT] = if has_accel {
        TILT_FLOOR_M_S2.powi(2)
    } else {
        1.0
    };
    p[SLOW][SLOW] = SLOW_NOISE_M * SLOW_NOISE_M;
    p[DRIFT][DRIFT] = 1.0;
    p[DRIFT_RATE][DRIFT_RATE] = (DRIFT_M_PER_S / 3_f64.sqrt()).powi(2);
    let air_sd = (AIR_TEMPERATURE_SPREAD_K / standard_temperature_k(pad_msl_for_air_m)
        + BAROMETER_GAIN)
        / 3_f64.sqrt();
    // The states are on the barometer's scale, `h_b = h + k ψ(h)`, `k` the day's air (`−ΔT/T_pad`
    // and the sensor's gain): a barometer can't tell it from a trajectory scaled to fit, and the
    // dynamics that could (a coast's speed against its accelerometer) carry model errors of the
    // same size, so an estimate of it from them walks off by many of its own standard deviations.
    // Only a GNSS receiver's geometric heights measure it; without one it is held at its prior
    // and its spread carried into the output.
    p[AIR][AIR] = air_sd * air_sd;
    if inputs.assumed_pad {
        // Nothing holds the rocket still at the first sample: it may be moving and under thrust,
        // and the barometer alone places it. The zero's own error is an offset the height can't
        // be told from, so it goes on the drift and reaches the height's standard error.
        if let Some(first) = inputs.baro.first() {
            x[H] = first.height_m;
        }
        p[H][H] = ASSUMED_HEIGHT_SD_M * ASSUMED_HEIGHT_SD_M;
        p[V][V] = ASSUMED_SPEED_SD_M_S * ASSUMED_SPEED_SD_M_S;
        p[A][A] = ASSUMED_ACCELERATION_SD_M_S2 * ASSUMED_ACCELERATION_SD_M_S2;
        p[DRIFT][DRIFT] = p[DRIFT][DRIFT].max(inputs.zero_sd_m * inputs.zero_sd_m);
    }
    let mut held = [false; N];
    held[AIR] = !has_gnss;
    if has_accel {
        // On that scale, `a_b = (1 + k) a`, the accelerometer reads
        // `a + g + s f + b = a_b + g + (s − k) f + (b + k g)` to first order in `k`: its scale and
        // bias take the air's spread, anti-correlated so a pad's `s g + b` keeps its own.
        p[SCALE][SCALE] += air_sd * air_sd;
        p[BIAS][BIAS] += (G0 * air_sd).powi(2);
        p[SCALE][BIAS] -= G0 * air_sd * air_sd;
        p[BIAS][SCALE] = p[SCALE][BIAS];
    }
    x[PORT_FALL] = PORT_PRIOR_MEAN;
    p[PORT_FALL][PORT_FALL] = PORT_PRIOR_SD * PORT_PRIOR_SD;
    if has_gnss {
        x[GNSS] = inputs.gnss[0].1;
        p[GNSS][GNSS] = GNSS_OFFSET_SD_M * GNSS_OFFSET_SD_M;
    } else {
        p[GNSS][GNSS] = 1.0;
    }

    let mut filter = Filter {
        x,
        p,
        jump: kalman::ZERO,
        held,
        rejected: Vec::new(),
        pad_msl_for_sound_m,
        gates: Vec::new(),
        replay: replay.map(|c| (c.gates.clone(), 0)),
    };
    let mut baro_tally = Tally::new(Sensor::Barometer);
    let mut accel_tally = Tally::new(Sensor::Accelerometer);
    let mut gnss_tally = Tally::new(Sensor::Gnss);
    let mut trust = Trust {
        mode: options.gnss_trust,
        trusted: false,
        since_s: None,
        first_s: None,
        agree_from_s: None,
        recent: Vec::new(),
        misses: 0,
    };
    let mut deweighted: Vec<Span> = Vec::new();

    let mut x_pred: Vec<Vector> = Vec::with_capacity(n);
    let mut p_pred: Vec<f64> = Vec::with_capacity(n * PACKED);
    let mut x_filt: Vec<Vector> = Vec::with_capacity(n);
    let mut held_at: Vec<[bool; N]> = Vec::with_capacity(n);
    let mut p_filt: Vec<f64> = Vec::with_capacity(n * PACKED);

    let mut powered_until: Option<f64> = None;
    // Past the first apogee the rocket may lie over, tumble or fall nose down, and its axis says
    // nothing of the vertical: the accelerometer is set aside from then.
    let mut past_apogee = false;
    let mut top_speed = 0.0_f64;
    let mut peak_low = 0.0_f64;
    let mut rate_peak = 0.0_f64;
    let mut last_rate = 0.0_f64;
    let mut landed = false;
    let mut transonic = Transonic::default();
    let mut last_f = G0;
    let mut last_baro_s: Option<f64> = None;

    for (k, (t, measured)) in steps.iter().enumerate() {
        let t = *t;
        if k > 0 {
            let dt = t - steps[k - 1].0;
            let phase = if t <= inputs.pad_end_s {
                Phase::Pad
            } else {
                // The burn ends when the climb surely slows faster than half a g, or its speed is
                // surely past its peak, each by two standard errors, or the barometer's own rate
                // has fallen [`RATE_DROP_SHARE`] off its peak outside a transonic span (where the
                // filter's spread, wide under the burn's jerk, would never let the first two
                // say so); and the first apogee comes when the climb stops. None of these before
                // the rocket is off the pad and climbing, as a barometer's liftoff window opens on
                // noise: a burnout found early would hold the coast's jerk through the burn.
                let a_high = filter.x[A] + 2.0 * filter.p[A][A].max(0.0).sqrt();
                let v_sd = filter.p[V][V].max(0.0).sqrt();
                top_speed = top_speed.max(filter.x[V]);
                peak_low = peak_low.max(filter.x[V] - 2.0 * v_sd);
                let replayed = replay.and_then(|c| c.steps.get(k - 1)).copied();
                let climbing = t > inputs.liftoff.time_s && top_speed > APOGEE_SPEED_M_S;
                let rate_fell = !transonic.open
                    && rate_peak > APOGEE_SPEED_M_S
                    && last_rate < rate_peak * (1.0 - RATE_DROP_SHARE);
                let slowing =
                    a_high < -0.5 * G0 || filter.x[V] + 2.0 * v_sd < peak_low || rate_fell;
                if powered_until.is_none() && climbing && slowing {
                    powered_until = Some(t + POWERED_HOLD_S);
                }
                let apogee = match replayed {
                    Some((_, past)) => past,
                    None => {
                        past_apogee
                            || powered_until.is_some()
                                && filter.x[V] + 2.0 * v_sd <= 0.0
                                && last_rate <= 0.0
                    }
                };
                if apogee && !past_apogee {
                    past_apogee = true;
                    for state in [BIAS, SCALE, TILT] {
                        filter.held[state] = true;
                    }
                }
                match (replayed, powered_until) {
                    (Some((phase, _)), _) => phase,
                    (None, None) => Phase::Powered,
                    (None, Some(until)) if t <= until => Phase::Powered,
                    _ if filter.x[V] > 0.0 => Phase::Coast,
                    _ => Phase::Descent,
                }
            };
            choices.steps.push((phase, past_apogee));
            let measured_a = has_accel && !past_apogee;
            let jerk = match phase {
                Phase::Pad => JERK_PAD,
                Phase::Powered => JERK_POWERED,
                // The impact stops the rocket in a few hundredths of a second, and what it does
                // on the ground (a bounce, a drag across the field) no model of a flight holds.
                _ if measured_a || landed => JERK_POWERED,
                Phase::Coast => JERK_COAST,
                Phase::Descent => JERK_DESCENT,
            };
            let tilt_sd = if has_accel {
                tilt_share(filter.x[V], top_speed, phase == Phase::Powered) * last_f.abs()
                    + TILT_FLOOR_M_S2
            } else {
                0.0
            };
            let gnss_walk = if has_gnss { GNSS_OFFSET_WALK } else { 0.0 };
            let f = transition(dt);
            filter.x = kalman::apply(&f, &filter.x);
            filter.p =
                kalman::propagate(&f, &filter.p, &process_noise(dt, jerk, tilt_sd, gnss_walk));
        }
        x_pred.push(filter.x);
        kalman::pack(&filter.p, &mut p_pred);

        // The impact stops the rocket faster than any jerk the model allows; its speed and
        // acceleration let go there, so the stop moves them and not the sensors' constants, and
        // the pass back doesn't carry it over the descent.
        if let Some(landing_s) = inputs.landing_s
            && !landed
            && t >= landing_s - LANDING_LEAD_S
        {
            landed = true;
            let v = filter.x[V].abs() + 3.0 * filter.p[V][V].max(0.0).sqrt();
            filter.widen(V, v * v);
            filter.widen(A, (v / LANDING_LEAD_S).powi(2));
        }

        if t <= inputs.pad_end_s {
            for state in [H, V, A] {
                let mut h = [0.0; N];
                h[state] = 1.0;
                let y = -filter.x[state];
                kalman::update(
                    &mut filter.x,
                    &mut filter.p,
                    &h,
                    y,
                    PAD_SD * PAD_SD,
                    &filter.held,
                );
            }
        }

        for m in measured {
            match *m {
                Measured::Accel(i) => {
                    let s = inputs.accel[i];
                    accel_tally.health.samples += 1;
                    if s.clipped || past_apogee {
                        accel_tally.health.unweighed += 1;
                        continue;
                    }
                    last_f = s.value_m_s2;
                    // z = a + g + s z̃ + b + m, the scale on the running median z̃.
                    let x = filter.x;
                    let mut h = [0.0; N];
                    h[A] = 1.0;
                    h[SCALE] = s.smoothed_m_s2;
                    h[BIAS] = 1.0;
                    h[TILT] = 1.0;
                    let predicted = x[A] + G0 + x[SCALE] * s.smoothed_m_s2 + x[BIAS] + x[TILT];
                    let y = s.value_m_s2 - predicted;
                    filter.fuse(
                        &mut accel_tally,
                        t,
                        &h,
                        y,
                        Variance::white(inputs.accel_sd * inputs.accel_sd),
                        Some(GATE_RESET_S),
                    );
                }
                Measured::Baro(i) => {
                    let s = inputs.baro[i];
                    baro_tally.health.samples += 1;
                    if t > inputs.liftoff.time_s {
                        last_rate = s.rate_m_s;
                        if !transonic.open {
                            rate_peak = rate_peak.max(s.rate_m_s);
                        }
                    }
                    let dt_baro = last_baro_s.map_or(0.05, |last| (t - last).max(1.0e-4));
                    last_baro_s = Some(t);
                    // The air at the running median's height: a garbled sample's own would
                    // put it in no air at all.
                    let height_msl = pad_msl_for_air_m + s.smoothed_m;
                    let thin = (rho_pad / standard_density(height_msl)).powi(2);
                    let fast = FAST_NOISE_M * FAST_NOISE_M;
                    // A de-weighted span's error is correlated over a second, not white.
                    let held = correlated_factor(dt_baro, DEWEIGHT_TAU_S);
                    // The pad's white noise, thinned with the air, and what the flight's own
                    // scatter holds beyond it and the correlated noise the pad's was taken less
                    // of (the fast, added below, and the slow, a state): correlated, so fused as
                    // such.
                    let pad = inputs.baro_white_var * thin;
                    let slow = SLOW_NOISE_M * SLOW_NOISE_M;
                    let excess = if t >= inputs.descent_from_s {
                        (s.scatter_var_m2 - (fast + slow) * thin - pad).max(0.0)
                    } else {
                        0.0
                    };
                    let mut r = Variance {
                        fused: pad
                            + excess * correlated_factor(dt_baro, SWING_TAU_S)
                            + fast * correlated_factor(dt_baro, FAST_NOISE_TAU_S) * thin,
                        gate: pad + excess + fast * thin,
                    };
                    let v = filter.x[V];
                    let v_sd = filter.p[V][V].max(0.0).sqrt();
                    // A descent fast enough to be transonic is a failed recovery's, and a filter
                    // that lost the track would de-weight the barometer that can find it again.
                    let replayed = replay.and_then(|c| c.transonic.get(choices.transonic.len()));
                    let (shift, v_high) = match replayed {
                        Some(&chosen) => chosen,
                        None if past_apogee => (0.0, 0.0),
                        None => transonic.weigh(
                            t,
                            v,
                            v_sd,
                            s.rate_m_s,
                            pad_msl_for_sound_m + s.height_m,
                        ),
                    };
                    choices.transonic.push((shift, v_high));
                    if shift > 0.0 {
                        let error = (shift * v_high * v_high / (2.0 * G0)).powi(2);
                        r.fused += error * held;
                        r.gate += error;
                        note_span(&mut deweighted, t, Deweighted::Transonic);
                    }
                    if inputs
                        .charges
                        .iter()
                        .any(|&(from, to)| (from..=to).contains(&t))
                    {
                        r.fused += CHARGE_SD_M * CHARGE_SD_M * held;
                        r.gate += CHARGE_SD_M * CHARGE_SD_M;
                        note_span(&mut deweighted, t, Deweighted::Charge);
                    }
                    // z = h_b + e + d − Cp v̂²/(2g), the port's error `Cp q/(ρ g)` on the predicted
                    // speed v̂: the climb's coefficient given, the fall's learned (a fall's
                    // attitude isn't the climb's); under a canopy it is small.
                    let x = filter.x;
                    let q_term = if landed {
                        0.0
                    } else {
                        s.rate_m_s * s.rate_m_s / (2.0 * G0)
                    };
                    let mut h = [0.0; N];
                    h[H] = 1.0;
                    h[SLOW] = 1.0;
                    h[DRIFT] = 1.0;
                    let coefficient = if past_apogee {
                        h[PORT_FALL] = -q_term;
                        x[PORT_FALL]
                    } else {
                        climb_port
                    };
                    // The term's slope in the speed, `−Cp v/g`: left out, a speed gone high would
                    // move only the height to explain what its own square predicts.
                    let predicted = x[H] + x[SLOW] + x[DRIFT] - coefficient * q_term;
                    let y = s.height_m - predicted;
                    filter.fuse(&mut baro_tally, t, &h, y, r, Some(GATE_RESET_S));
                }
                Measured::Gnss(i) => {
                    let (_, z) = inputs.gnss[i];
                    gnss_tally.health.samples += 1;
                    let v_sd = filter.p[V][V].max(0.0).sqrt();
                    let replayed = replay.and_then(|c| c.gnss.get(choices.gnss.len()));
                    let (trusted, began, settling) = match replayed {
                        Some(&chosen) => chosen,
                        None => {
                            let (trusted, began) = trust.weigh(
                                t,
                                z,
                                filter.x[V],
                                v_sd,
                                inputs.gnss_sd,
                                inputs.liftoff.time_s,
                            );
                            let settling =
                                trust.since_s.is_some_and(|since| t - since < GNSS_SETTLE_S);
                            (trusted, began, settling)
                        }
                    };
                    choices.gnss.push((trusted, began, settling));
                    if !trusted {
                        gnss_tally.health.unweighed += 1;
                        continue;
                    }
                    if began {
                        // The offset let go: whatever the receiver's lag left in it is forgotten.
                        filter.widen(GNSS, GNSS_OFFSET_SD_M * GNSS_OFFSET_SD_M);
                    }
                    let r = inputs.gnss_sd
                        * inputs.gnss_sd
                        * if settling { GNSS_SETTLE_FACTOR } else { 1.0 };
                    // z = h_b − k ψ(h_b) + o: the true height, and the receiver's offset.
                    let psi = air_regressor(filter.x[H].max(0.0), pad_msl_for_air_m);
                    let mut h = [0.0; N];
                    h[H] = 1.0;
                    h[AIR] = -psi;
                    h[GNSS] = 1.0;
                    let y = z - (filter.x[H] - filter.x[AIR] * psi + filter.x[GNSS]);
                    let reset_after_s =
                        (trust.mode == GnssTrust::FromLiftoff).then_some(GATE_RESET_S);
                    match filter.fuse(&mut gnss_tally, t, &h, y, Variance::white(r), reset_after_s)
                    {
                        Gated::Used => trust.misses = 0,
                        Gated::Rejected => {
                            trust.misses += 1;
                            if trust.mode == GnssTrust::Agreement && trust.misses >= 2 {
                                trust.trusted = false;
                                trust.agree_from_s = None;
                                trust.misses = 0;
                            }
                        }
                    }
                }
            }
        }
        if filter.jump != kalman::ZERO {
            let stored = &mut p_pred[k * PACKED..(k + 1) * PACKED];
            let mut at = 0;
            for i in 0..N {
                for j in i..N {
                    stored[at] += filter.jump[i][j];
                    at += 1;
                }
            }
            filter.jump = kalman::ZERO;
        }
        held_at.push(filter.held);
        x_filt.push(filter.x);
        kalman::pack(&filter.p, &mut p_filt);
    }

    // The pass back.
    let mut height_m = vec![0.0; n];
    let mut height_sd_m = vec![0.0; n];
    let mut speed_m_s = vec![0.0; n];
    let mut speed_sd_m_s = vec![0.0; n];
    let mut unsmoothed_steps = 0;
    if n > 0 {
        let mut xs = x_filt[n - 1];
        let mut ps = kalman::unpack(&p_filt[(n - 1) * PACKED..n * PACKED]);
        // Off the barometer's scale: `h = h_b − k ψ(h_b)` and `v = v_b (1 − k ψ')`, `ψ' = T_pad/T`,
        // each with its variance `c P cᵀ` along its row `c`.
        let t_pad = standard_temperature_k(pad_msl_for_air_m);
        let mut put = |k: usize, xs: &Vector, ps: &Matrix| {
            let h_b = xs[H].max(0.0);
            let psi = air_regressor(h_b, pad_msl_for_air_m);
            let slope = t_pad / standard_temperature_k(pad_msl_for_air_m + h_b);
            let along = |i: usize, ci: f64, cair: f64| -> f64 {
                ci * ci * ps[i][i] + 2.0 * ci * cair * ps[i][AIR] + cair * cair * ps[AIR][AIR]
            };
            height_m[k] = xs[H] - xs[AIR] * psi;
            height_sd_m[k] = along(H, 1.0, -psi).max(0.0).sqrt();
            let gain = 1.0 - xs[AIR] * slope;
            speed_m_s[k] = xs[V] * gain;
            speed_sd_m_s[k] = along(V, gain, -slope * xs[V]).max(0.0).sqrt();
        };
        put(n - 1, &xs, &ps);
        for k in (0..n - 1).rev() {
            let dt = steps[k + 1].0 - steps[k].0;
            let f = transition(dt);
            let pf = kalman::unpack(&p_filt[k * PACKED..(k + 1) * PACKED]);
            let pp = kalman::unpack(&p_pred[(k + 1) * PACKED..(k + 2) * PACKED]);
            let c = kalman::mul(&f, &pf);
            match kalman::solve(&pp, &c) {
                Some(gain_t) => {
                    // G = (P_pred⁻¹ F P_f)ᵀ.
                    let mut gain = kalman::ZERO;
                    for i in 0..N {
                        for j in 0..N {
                            gain[i][j] = gain_t[j][i];
                        }
                    }
                    // A held state's estimate isn't moved going back either.
                    for (row, &held) in gain.iter_mut().zip(&held_at[k]) {
                        if held {
                            *row = [0.0; N];
                        }
                    }
                    let mut dx = [0.0; N];
                    for i in 0..N {
                        dx[i] = xs[i] - x_pred[k + 1][i];
                    }
                    let correction = kalman::apply(&gain, &dx);
                    let mut next = x_filt[k];
                    for i in 0..N {
                        next[i] += correction[i];
                    }
                    let mut dp = ps;
                    for i in 0..N {
                        for j in 0..N {
                            dp[i][j] -= pp[i][j];
                        }
                    }
                    let gdp = kalman::mul(&gain, &dp);
                    let mut next_p = kalman::mul_transposed(&gdp, &gain);
                    for i in 0..N {
                        for j in 0..N {
                            next_p[i][j] += pf[i][j];
                        }
                    }
                    kalman::symmetrize(&mut next_p);
                    xs = next;
                    ps = next_p;
                }
                None => {
                    unsmoothed_steps += 1;
                    xs = x_filt[k];
                    ps = pf;
                }
            }
            put(k, &xs, &ps);
        }
    }

    let times: Vec<f64> = steps.iter().map(|s| s.0).collect();
    let apogee_index = (0..n)
        .filter(|&k| times[k] > inputs.liftoff.time_s)
        .max_by(|&a, &b| height_m[a].total_cmp(&height_m[b]).then(b.cmp(&a)))
        .unwrap_or(0);
    let apogee_time_s = times.get(apogee_index).copied().unwrap_or(f64::NAN);
    let baro_minus_inertial_at_apogee_m = inertial_gap(inputs, apogee_time_s);

    let mut sensors = vec![baro_tally.finish()];
    if has_accel {
        sensors.push(accel_tally.finish());
    }
    if has_gnss {
        sensors.push(gnss_tally.finish());
    }
    let mut rejected = filter.rejected;
    rejected.sort_by(|a, b| a.time_s.total_cmp(&b.time_s).then(a.sensor.cmp(&b.sensor)));
    choices.gates = filter.gates;
    let smoothed = Smoothed {
        time_s: times,
        height_m,
        height_sd_m,
        speed_m_s,
        speed_sd_m_s,
        liftoff: inputs.liftoff,
        apogee_time_s,
        health: Health {
            sensors,
            deweighted,
            rejected,
            gnss_trusted_s: trust.first_s,
            baro_minus_inertial_at_apogee_m,
            unsmoothed_steps,
            assumed_pad: inputs.assumed_pad,
            accelerometer_set_aside: inputs.accelerometer_set_aside,
            apogee_less_reading_m: None,
            apogee_past_reading_bound: false,
        },
    };
    (smoothed, choices)
}

/// Adds a de-weighted sample to the spans: it extends the last span of its kind if it follows it
/// within a second.
fn note_span(spans: &mut Vec<Span>, t: f64, why: Deweighted) {
    if let Some(span) = spans.iter_mut().rev().find(|s| s.why == why)
        && t - span.end_s <= 1.0
    {
        span.end_s = t;
        return;
    }
    spans.push(Span {
        start_s: t,
        end_s: t,
        why,
    });
}

/// The barometer's height less the accelerometer's alone at `apogee_s`: the accelerometer's
/// readings less its pad's level, integrated twice from liftoff by the trapezoid rule, a clipped
/// sample held at the last good one.
fn inertial_gap(inputs: &Inputs, apogee_s: f64) -> Option<f64> {
    if inputs.accel.is_empty() || !apogee_s.is_finite() {
        return None;
    }
    let start = inputs.liftoff.time_s;
    let (mut v, mut h) = (0.0, 0.0);
    let mut last: Option<(f64, f64)> = None;
    for s in inputs
        .accel
        .iter()
        .filter(|s| s.time_s >= start && s.time_s <= apogee_s)
    {
        let a = if s.clipped {
            last.map_or(0.0, |(_, a)| a)
        } else {
            s.value_m_s2 - inputs.accel_pad
        };
        if let Some((t0, a0)) = last {
            let dt = s.time_s - t0;
            let v_next = v + 0.5 * (a0 + a) * dt;
            h += 0.5 * (v + v_next) * dt;
            v = v_next;
        }
        last = Some((s.time_s, a));
    }
    let baro = inputs.baro.iter().min_by(|a, b| {
        (a.time_s - apogee_s)
            .abs()
            .total_cmp(&(b.time_s - apogee_s).abs())
    })?;
    Some(baro.height_m - h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::{LogFormat, Stated};

    const DT: f64 = 0.05;
    const LIFTOFF_S: f64 = 5.0;
    const BURNOUT_S: f64 = 7.0;
    /// 5 g for 2 s: 98.1 m/s and 98.1 m at burnout.
    const BOOST_M_S2: f64 = 5.0 * G0;
    const DESCENT_M_S: f64 = 15.0;

    /// An invented flight: on the pad to 5 s, 5 g to 7 s, a coast without drag to apogee at 17 s
    /// (588.4 m), a fall to 15 m/s and down at that to the ground at 57.0 s, and 16 s on it.
    /// Height and vertical speed.
    fn truth(t: f64) -> (f64, f64) {
        let v_burnout = BOOST_M_S2 * (BURNOUT_S - LIFTOFF_S);
        let h_burnout = 0.5 * BOOST_M_S2 * (BURNOUT_S - LIFTOFF_S).powi(2);
        let apogee_s = BURNOUT_S + v_burnout / G0;
        let apogee_m = h_burnout + v_burnout * v_burnout / (2.0 * G0);
        if t <= LIFTOFF_S {
            (0.0, 0.0)
        } else if t <= BURNOUT_S {
            let c = t - LIFTOFF_S;
            (0.5 * BOOST_M_S2 * c * c, BOOST_M_S2 * c)
        } else if t <= apogee_s {
            let c = t - BURNOUT_S;
            (
                h_burnout + v_burnout * c - 0.5 * G0 * c * c,
                v_burnout - G0 * c,
            )
        } else {
            let c = t - apogee_s;
            let fall_s = DESCENT_M_S / G0;
            if c < fall_s {
                return (apogee_m - 0.5 * G0 * c * c, -G0 * c);
            }
            let h = apogee_m - 0.5 * G0 * fall_s * fall_s - DESCENT_M_S * (c - fall_s);
            if h > 0.0 {
                (h, -DESCENT_M_S)
            } else {
                (0.0, 0.0)
            }
        }
    }

    /// A noise sample uniform in ±0.5 m (0.29 m standard deviation), the same every run.
    fn noise(state: &mut u64) -> f64 {
        *state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (*state >> 11) as f64 / (1_u64 << 53) as f64 - 0.5
    }

    /// The flight logged by a 20 Hz barometer-only altimeter.
    fn log() -> FlightLog {
        let times: Vec<f64> = (0..1460).map(|i| f64::from(i) * DT).collect();
        let mut state = 7;
        let altitude_m = times
            .iter()
            .map(|&t| truth(t).0 + noise(&mut state))
            .collect();
        FlightLog {
            format: LogFormat::PerfectFlitePf2,
            logger: "PerfectFlite Pnut".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            time_s: times,
            altitude_m,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: Vec::new(),
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// The same flight with a GNSS receiver at 10 Hz on a pad 300 m above sea level, its altitude
    /// lagging the truth by 1.5 s throughout, as a receiver's filter does in a climb.
    fn with_gnss(mut log: FlightLog) -> FlightLog {
        let times: Vec<f64> = (0..730).map(|i| f64::from(i) * 0.1).collect();
        let values = times.iter().map(|&t| 300.0 + truth(t - 1.5).0).collect();
        log.channels
            .push(Channel::new("gnss", Quantity::GnssAltitude, times, values));
        log
    }

    fn smoothed(log: &FlightLog, options: Options) -> Smoothed {
        smooth_with(log, &readings::read(log), options).expect("the invented log smooths")
    }

    /// Asserts the truth lies inside the 90% interval at `t`, and the interval is no wider than
    /// `height_sd_m` and `speed_sd_m_s`.
    fn holds_at(smoothed: &Smoothed, t: f64, height_sd_m: f64, speed_sd_m_s: f64) {
        let state = smoothed.at(t).expect("inside the log");
        let (h, v) = truth(t);
        assert!(
            (state.height_m - h).abs() <= INTERVAL_Z * state.height_sd_m,
            "height at {t} s: {} ± {} against {h}",
            state.height_m,
            state.height_sd_m
        );
        assert!(
            (state.speed_m_s - v).abs() <= INTERVAL_Z * state.speed_sd_m_s,
            "speed at {t} s: {} ± {} against {v}",
            state.speed_m_s,
            state.speed_sd_m_s
        );
        assert!(
            state.height_sd_m <= height_sd_m,
            "height sd at {t} s: {}",
            state.height_sd_m
        );
        assert!(
            state.speed_sd_m_s <= speed_sd_m_s,
            "speed sd at {t} s: {}",
            state.speed_sd_m_s
        );
    }

    #[test]
    fn an_invented_flight_lies_inside_its_intervals() {
        let smoothed = smoothed(&log(), Options::default());
        let liftoff = smoothed.liftoff;
        assert!(
            (liftoff.time_s - LIFTOFF_S).abs() <= liftoff.bound_s,
            "{liftoff:?}"
        );
        // The intervals are bounded by the day's air, up to about 4% of the height without a
        // GNSS receiver, and at burnout by the climb's port coefficient's spread, 0.02 of
        // `v²/2g`, 9.8 m. On the ground the acceleration is let go ([`JERK_POWERED`]), so the
        // speed is the barometer's noise's alone.
        for (t, height_sd_m, speed_sd_m_s) in [
            (6.0, 15.0, 15.0),
            (BURNOUT_S, 20.0, 15.0),
            (12.0, 25.0, 5.0),
            (17.0, 30.0, 3.0),
            (30.0, 25.0, 2.0),
            (50.0, 10.0, 2.0),
            (70.0, 2.0, 5.0),
        ] {
            holds_at(&smoothed, t, height_sd_m, speed_sd_m_s);
        }
        assert!(
            (smoothed.apogee_time_s - 17.0).abs() < 0.5,
            "{}",
            smoothed.apogee_time_s
        );
        assert_eq!(smoothed.health.unsmoothed_steps, 0);
        assert!(
            smoothed.health.rejected.is_empty(),
            "{:?}",
            smoothed.health.rejected
        );
        // The log names no charge, and the barometer is de-weighted across its apogee anyway.
        assert!(
            smoothed
                .health
                .deweighted
                .iter()
                .any(|s| s.why == Deweighted::Charge && s.start_s <= 17.0 && s.end_s >= 17.0),
            "{:?}",
            smoothed.health.deweighted
        );
    }

    #[test]
    fn the_selected_median_is_the_sorted_one() {
        let mut state = 5;
        for n in [1, 2, 3, 4, 11, 40, 41] {
            let values: Vec<f64> = (0..n).map(|_| noise(&mut state)).collect();
            assert_eq!(
                selected_median(&mut values.clone()),
                median(&mut values.clone()),
                "{n}"
            );
        }
    }

    #[test]
    fn the_scatter_is_the_noise_about_a_curving_track() {
        // Normal noise of σ 0.5 m, by Box and Muller from the uniform draws; about a coast's
        // parabola the quadratic takes the track out whole, so the scatter is the noise's
        // variance, 0.25 m², less the fit's three parameters' share of 41 samples, 7%. A squared
        // MAD of 41 samples is skewed, so its median over the windows sits below that: 0.19 on
        // this draw. Read low by a quarter, the gate is still 4.3σ wide.
        let times: Vec<f64> = (0..400).map(|i| f64::from(i) * DT).collect();
        let mut state = 11;
        let values: Vec<f64> = times
            .iter()
            .map(|&t| {
                let (u, w) = (noise(&mut state) + 0.5, noise(&mut state) + 0.5);
                let normal = (-2.0 * (1.0 - u).ln()).sqrt() * (std::f64::consts::TAU * w).cos();
                900.0 + 80.0 * t - 0.5 * G0 * t * t + 0.5 * normal
            })
            .collect();
        let var = scatter(&values, &super::smoothed(&values, &times), &times);
        let mut sorted = var.clone();
        let middle = median(&mut sorted);
        assert!((0.7..1.05).contains(&(middle / 0.25)), "{middle}");
        // A noiseless track's is nothing, even curving, but at its top, where the running median
        // the quadratic is fitted to is a neighbor's, g Δt²/2 = 1.2 cm low at 20 Hz. Within a
        // window of the series' ends the median's first and last samples, a step off, pull the
        // fit too: a real log starts and ends at rest.
        let clean: Vec<f64> = times.iter().map(|&t| 80.0 * t - 0.5 * G0 * t * t).collect();
        let medians = super::smoothed(&clean, &times);
        let var = scatter(&clean, &medians, &times);
        let end = times.last().copied().expect("samples") - SCATTER_HALF_S;
        let largest = var
            .iter()
            .zip(&times)
            .filter(|&(_, &t)| t > SCATTER_HALF_S && t < end)
            .map(|(&v, _)| v)
            .fold(0.0, f64::max);
        assert!(largest < 0.012 * 0.012, "{largest}");
    }

    #[test]
    fn a_quiet_pad_does_not_make_a_swinging_descent_fail_the_gate() {
        // The pad reads one value throughout, so the pad gives the barometer no noise at all;
        // under the canopy the rocket's swing adds 6 m at 0.8 Hz to what it reads.
        let mut log = log();
        let apogee_s = 17.0;
        for (z, &t) in log.altitude_m.iter_mut().zip(&log.time_s) {
            if t <= LIFTOFF_S {
                *z = 0.0;
            } else if t > apogee_s + 3.0 && truth(t).0 > 0.0 {
                *z += 6.0 * (std::f64::consts::TAU * 0.8 * t).sin();
            }
        }
        let smoothed = smoothed(&log, Options::default());
        assert!(
            smoothed.health.rejected.is_empty(),
            "{:?}",
            smoothed.health.rejected
        );
        assert_eq!(smoothed.health.unsmoothed_steps, 0);
        // The swing reads as ±30 m/s of rate, and as an error correlated over seconds it is
        // smoothed by the motion model, not averaged away: the speed's interval is a few m/s.
        for t in [30.0, 40.0, 50.0] {
            holds_at(&smoothed, t, 30.0, 6.0);
        }
    }

    /// The share of the descent's heights, 50 s to 56 s every half second, outside their 90%
    /// intervals over `seeds` logs at `rate_hz` whose barometer adds, from 20 s on, a first-order
    /// error of `sd_m` and `tau_s`: a rocket swinging under its canopy.
    fn swinging_misses(rate_hz: f64, sd_m: f64, tau_s: f64, seeds: u64) -> f64 {
        let (mut missed, mut counted) = (0_u32, 0_u32);
        for seed in 1..=seeds {
            let mut log = log();
            let n = (73.0 * rate_hz) as u32;
            log.time_s = (0..n).map(|i| f64::from(i) / rate_hz).collect();
            let mut state = seed * 7919;
            let phi = (-1.0 / (rate_hz * tau_s)).exp();
            let mut swing = 0.0;
            log.altitude_m = log
                .time_s
                .iter()
                .map(|&t| {
                    let (u, w) = (noise(&mut state) + 0.5, noise(&mut state) + 0.5);
                    let normal = (-2.0 * (1.0 - u).ln()).sqrt() * (std::f64::consts::TAU * w).cos();
                    swing = phi * swing + sd_m * (1.0 - phi * phi).sqrt() * normal;
                    let pad = 0.29 * normal;
                    truth(t).0
                        + if t > 20.0 && truth(t).0 > 0.0 {
                            swing
                        } else {
                            pad
                        }
                })
                .collect();
            let smoothed = smoothed(&log, Options::default());
            for k in 0..=12 {
                let t = 50.0 + 0.5 * f64::from(k);
                let state = smoothed.at(t).expect("inside the log");
                counted += 1;
                if (state.height_m - truth(t).0).abs() > INTERVAL_Z * state.height_sd_m {
                    missed += 1;
                }
            }
        }
        f64::from(missed) / f64::from(counted)
    }

    #[test]
    fn a_swinging_descent_correlated_over_a_second_stays_inside_its_intervals() {
        // A 90% interval misses a tenth on average. Over 40 logs these missed 0%, 8.5% and 5%.
        for (rate_hz, sd_m, tau_s) in [(20.0, 3.0, 0.3), (20.0, 6.0, 1.0), (100.0, 6.0, 1.0)] {
            let share = swinging_misses(rate_hz, sd_m, tau_s, 40);
            assert!(share <= 0.1, "{rate_hz} Hz, {sd_m} m, {tau_s} s: {share}");
        }
        // The limit: a swing correlated over 3 s is partly read as the rocket's motion, and its
        // intervals miss more than they claim, 26% on these.
        let share = swinging_misses(20.0, 6.0, 3.0, 40);
        assert!(share > 0.1, "{share}");
    }

    #[test]
    fn a_smoothed_track_read_back_with_columns_of_other_lengths_gives_nothing() {
        let mut smoothed = smoothed(&log(), Options::default());
        assert!(smoothed.at(30.0).is_some());
        smoothed.height_m.truncate(10);
        assert!(smoothed.at(30.0).is_none());
    }

    #[test]
    fn a_receiver_logging_past_the_barometer_is_cut_at_its_end() {
        // The receiver logs 20 s past the barometer's last sample at 72.95 s.
        let mut log = log();
        let times: Vec<f64> = (0..930).map(|i| f64::from(i) * 0.1).collect();
        let values = times.iter().map(|&t| 300.0 + truth(t).0).collect();
        log.channels
            .push(Channel::new("gnss", Quantity::GnssAltitude, times, values));
        let smoothed = smoothed(&log, Options::default());
        let end = log.time_s.last().copied().expect("samples");
        assert!(
            smoothed.time_s.iter().all(|&t| t <= end),
            "{:?}",
            smoothed.time_s.last()
        );
        assert!(
            (smoothed.apogee_time_s - 17.0).abs() < 0.5,
            "{}",
            smoothed.apogee_time_s
        );
    }

    #[test]
    fn a_spike_is_rejected_at_the_gate_and_leaves_the_track_alone() {
        let mut log = log();
        let at = log
            .time_s
            .iter()
            .position(|&t| t >= 40.0)
            .expect("in the log");
        log.altitude_m[at] += 200.0;
        let smoothed = smoothed(&log, Options::default());
        let rejected: Vec<f64> = smoothed
            .health
            .rejected
            .iter()
            .filter(|r| r.sensor == Sensor::Barometer)
            .map(|r| r.time_s)
            .collect();
        assert_eq!(rejected, vec![log.time_s[at]]);
        holds_at(&smoothed, log.time_s[at], 20.0, 2.0);
    }

    #[test]
    fn a_garbled_sample_is_rejected_and_bends_nothing_around_it() {
        // A height a logger's garbled word reads as: 2 km, a centimeter count past an int32's
        // range, and an f32's largest. The rates the port's term and the transonic span read
        // are the running medians', so the sample's neighbors keep theirs.
        for spike in [2000.0, 1.0e7, 3.0e38] {
            let mut log = log();
            let at = log
                .time_s
                .iter()
                .position(|&t| t >= 10.0)
                .expect("in the log");
            log.altitude_m[at] = spike;
            let smoothed = smoothed(&log, Options::default());
            let rejected: Vec<f64> = smoothed.health.rejected.iter().map(|r| r.time_s).collect();
            assert_eq!(rejected, vec![log.time_s[at]], "{spike}");
            assert!(
                smoothed
                    .health
                    .deweighted
                    .iter()
                    .all(|s| s.why != Deweighted::Transonic),
                "{spike}: {:?}",
                smoothed.health.deweighted
            );
            for t in [10.0, 12.0, 30.0] {
                holds_at(&smoothed, t, 25.0, 5.0);
            }
        }
    }

    #[test]
    fn a_lagging_gnss_receiver_is_trusted_only_once_its_speed_agrees() {
        let log = with_gnss(log());
        let trusted = smoothed(&log, Options::default());
        let from = trusted.health.gnss_trusted_s.expect("the descent agrees");
        // Its speed lags the coast's by 1.5 s of gravity, 14.7 m/s, until the descent's steady
        // 15 m/s, which the truth reaches 1.5 s after apogee and the receiver 1.5 s later.
        assert!(from > 17.0 + 1.5 + 1.5, "{from}");
        let gnss = |s: &Smoothed| {
            s.health
                .sensors
                .iter()
                .find(|h| h.sensor == Sensor::Gnss)
                .cloned()
                .expect("a GNSS row")
        };
        assert!(gnss(&trusted).unweighed > 0);
        holds_at(&trusted, 12.0, 20.0, 5.0);

        // Trusted from liftoff, the lag is fused through the climb.
        let options = Options {
            gnss_trust: GnssTrust::FromLiftoff,
        };
        let from_liftoff = smoothed(&log, options);
        assert_eq!(gnss(&from_liftoff).unweighed, 0);
    }

    #[test]
    fn the_landing_is_where_the_track_settles() {
        let baro: Vec<Baro> = (0..1460)
            .map(|i| {
                let t = f64::from(i) * DT;
                let h = truth(t).0;
                Baro {
                    time_s: t,
                    height_m: h,
                    smoothed_m: h,
                    rate_m_s: truth(t).1,
                    scatter_var_m2: 0.0,
                }
            })
            .collect();
        // Down at 15 m/s, it lands at 57.0 s; the track is within 5 m of the ground from 56.7 s,
        // and stays there.
        let landing = landing(&baro, LIFTOFF_S).expect("it lands");
        assert!((56.65..=56.75).contains(&landing), "{landing}");
        // Cut before the ground holds for 3 s, it hasn't landed.
        assert_eq!(landing_cut(&baro, 58.0), None);

        fn landing_cut(baro: &[Baro], end_s: f64) -> Option<f64> {
            let cut: Vec<Baro> = baro.iter().copied().filter(|b| b.time_s <= end_s).collect();
            super::landing(&cut, LIFTOFF_S)
        }
    }

    #[test]
    fn slopes_are_the_derivative_of_a_parabola_inside_the_window() {
        // Sixteenths of a second, so the window's edges fall on samples exactly.
        let times: Vec<f64> = (0..200).map(|i| f64::from(i) / 16.0).collect();
        let values: Vec<f64> = times.iter().map(|t| 3.0 + 7.0 * t - 2.0 * t * t).collect();
        let rates = slopes(&values, &times, RATE_HALF_S);
        // A least-squares line over a window symmetric about its center has a parabola's slope
        // there exactly.
        for (i, (&t, &rate)) in times.iter().zip(&rates).enumerate().skip(4).take(192) {
            assert!((rate - (7.0 - 4.0 * t)).abs() < 1e-9, "{i}: {rate}");
        }
    }

    #[test]
    fn the_tilt_share_is_the_rails_under_power_and_grows_as_the_climb_slows() {
        let rail = 1.0 - RAIL_TILT_DEG.to_radians().cos();
        assert!((tilt_share(300.0, 300.0, true) - rail).abs() < 1e-15);
        assert!((tilt_share(5.0, 300.0, true) - rail).abs() < 1e-15);
        // In the coast, the sideways speed is the rail's share of the top speed and the wind.
        let side = RAIL_TILT_DEG.to_radians().sin() * 300.0 + WIND_LIMIT_M_S;
        let at = |v: f64| 1.0 - v / v.hypot(side);
        assert!((tilt_share(100.0, 300.0, false) - at(100.0)).abs() < 1e-15);
        assert!(tilt_share(10.0, 300.0, false) > tilt_share(100.0, 300.0, false));
        // Never less than the rail's.
        assert!((tilt_share(1.0e4, 300.0, false) - rail).abs() < 1e-15);
    }

    #[test]
    fn a_transonic_span_opens_on_both_speeds_and_closes_after_a_subsonic_second() {
        let cold = standard_temperature_k(0.0) - AIR_TEMPERATURE_SPREAD_K;
        let mach_one = sound(cold) * RAIL_TILT_DEG.to_radians().cos();
        let mut span = Transonic::default();
        // Subsonic by both: closed.
        assert_eq!(
            span.weigh(0.0, 0.5 * mach_one, 1.0, 0.5 * mach_one, 0.0).0,
            0.0
        );
        // The filter says transonic, the barometer's rate doesn't: still closed.
        assert_eq!(
            span.weigh(0.1, 0.95 * mach_one, 1.0, 0.5 * mach_one, 0.0).0,
            0.0
        );
        assert!(!span.open);
        // Both: open, at the full shift.
        let (shift, _) = span.weigh(0.2, 0.95 * mach_one, 1.0, 0.95 * mach_one, 0.0);
        assert_eq!(shift, TRANSONIC_PORT_SHIFT);
        assert!(span.open);
        // The rate subsonic for less than a second, then transonic again: still open.
        assert_eq!(
            span.weigh(0.3, 0.0, 1.0, 0.5 * mach_one, 0.0).0,
            TRANSONIC_PORT_SHIFT
        );
        assert_eq!(
            span.weigh(1.2, 0.0, 1.0, 0.95 * mach_one, 0.0).0,
            TRANSONIC_PORT_SHIFT
        );
        // Subsonic from 1.25 s: open until 2.25 s, closed there.
        assert_eq!(
            span.weigh(1.25, 0.0, 1.0, 0.5 * mach_one, 0.0).0,
            TRANSONIC_PORT_SHIFT
        );
        assert_eq!(
            span.weigh(2.0, 0.0, 1.0, 0.5 * mach_one, 0.0).0,
            TRANSONIC_PORT_SHIFT
        );
        assert_eq!(span.weigh(2.25, 0.0, 1.0, 0.5 * mach_one, 0.0), (0.0, 0.0));
        assert!(!span.open);
    }

    #[test]
    fn a_charge_is_expected_where_the_track_tops_out() {
        let baro: Vec<Baro> = (0..1460)
            .map(|i| {
                let t = f64::from(i) * DT;
                let h = truth(t).0;
                Baro {
                    time_s: t,
                    height_m: h,
                    smoothed_m: h,
                    rate_m_s: 0.0,
                    scatter_var_m2: 0.0,
                }
            })
            .collect();
        // Within 5 m of the top for √(2·5/g) = 1.01 s either side of apogee at 17 s.
        let (from, to) = apogee_charge(&baro, LIFTOFF_S).expect("a top");
        assert!((15.95..=16.0).contains(&from), "{from}");
        assert!((18.0..=18.05).contains(&(to - CHARGE_WINDOW_S)), "{to}");
    }

    #[test]
    fn the_consider_run_replays_the_first_runs_choices_and_widens_by_its_difference() {
        let log = with_gnss(log());
        let readings = readings::read(&log);
        let inputs = Inputs::gather(&log, &readings).expect("it gathers");
        let options = Options::default();
        let (mean, choices) = run(&inputs, options, PORT_PRIOR_MEAN, None);
        let (off, replayed) = run(
            &inputs,
            options,
            PORT_PRIOR_MEAN - PORT_PRIOR_SD,
            Some(&choices),
        );
        assert_eq!(replayed, choices);
        let smoothed = smooth_with(&log, &readings, options).expect("it smooths");
        for k in 0..mean.time_s.len() {
            let widened = mean.height_sd_m[k].hypot(off.height_m[k] - mean.height_m[k]);
            assert_eq!(smoothed.height_sd_m[k], widened);
        }
        // At burnout the port's spread, 0.02 of `v²/2g` = 9.8 m, is most of the interval.
        let k = mean.time_s.partition_point(|&t| t < BURNOUT_S);
        let difference = (off.height_m[k] - mean.height_m[k]).abs();
        assert!((7.0..=12.0).contains(&difference), "{difference}");
    }

    /// The flight as a logger that writes only from launch would have written it: nothing before
    /// liftoff, its clock and zero as they were.
    fn from_launch(mut log: FlightLog) -> FlightLog {
        let first = log.time_s.partition_point(|t| *t < LIFTOFF_S);
        log.time_s.drain(..first);
        log.altitude_m.drain(..first);
        log
    }

    #[test]
    fn a_log_written_from_launch_is_smoothed_on_an_assumed_pad() {
        let whole = smoothed(&log(), Options::default());
        assert!(!whole.health.assumed_pad);
        assert!(!whole.health.card().contains("pad assumed"));

        let started = smoothed(&from_launch(log()), Options::default());
        assert!(started.health.assumed_pad);
        assert!(started.health.card().starts_with("pad assumed: "));
        assert_eq!(started.health.unsmoothed_steps, 0);
        for t in [LIFTOFF_S + 1.0, BURNOUT_S, 20.0, 40.0, 60.0] {
            holds_at(&started, t, 80.0, 25.0);
        }
        // A log that starts well up the climb has no zero the readings can take, and the smoother
        // takes no liftoff from what they withhold.
        let mut later = log();
        let first = later.time_s.partition_point(|t| *t < LIFTOFF_S + 1.0);
        later.time_s.drain(..first);
        later.altitude_m.drain(..first);
        let withheld = smooth(&later).unwrap_err();
        assert_eq!(withheld.reason, Reason::NoLiftoff);
    }

    /// A public `.pf2` log's second sample read −65,535 ft, and its first rate, taken over the few
    /// samples at the log's edge, set the track off at 29 km/s until the prior left the rate out.
    #[test]
    fn a_garbled_sample_at_the_start_of_a_log_from_launch_moves_nothing() {
        let mut garbled = from_launch(log());
        garbled.altitude_m[1] = -65_535.0 * crate::perfectflite::FOOT_M;
        let started = smoothed(&garbled, Options::default());
        assert!(started.health.assumed_pad);
        assert!(
            started
                .health
                .rejected
                .iter()
                .any(|r| r.time_s == garbled.time_s[1])
        );
        let top = started.height_m.iter().copied().fold(f64::MIN, f64::max);
        assert!(
            top < 1.05 * truth(BURNOUT_S + BOOST_M_S2 * (BURNOUT_S - LIFTOFF_S) / G0).0,
            "{top}"
        );
        assert!(started.speed_m_s.iter().all(|v| v.abs() < 500.0));
        holds_at(&started, 20.0, 80.0, 25.0);
    }

    /// A zero that is the pad's only by a word may be off by `PAD_ZERO_M`: the height's standard
    /// error holds that, and the speed's doesn't change.
    #[test]
    fn a_zero_by_a_word_reaches_the_heights_standard_error() {
        let own = from_launch(log());
        let mut by_word = own.clone();
        by_word.format = LogFormat::GenericTable;
        let t = 20.0;
        let own_state = smoothed(&own, Options::default()).at(t).unwrap();
        let word_state = smoothed(&by_word, Options::default()).at(t).unwrap();
        let widened = word_state.height_sd_m * word_state.height_sd_m
            - own_state.height_sd_m * own_state.height_sd_m;
        let expected = readings::PAD_ZERO_M * readings::PAD_ZERO_M / 3.0;
        assert!(
            (widened / expected - 1.0).abs() < 0.1,
            "{widened} against {expected}"
        );
        assert!((word_state.speed_sd_m_s / own_state.speed_sd_m_s - 1.0).abs() < 0.02);
    }

    #[test]
    fn a_flights_own_scatter_stands_for_a_pads_noise() {
        // Past the descent's start the scatter holds the white noise, here 3 m, less the correlated
        // noise the smoother carries apart; at sea level, no air thins it.
        let baro: Vec<Baro> = (0..100)
            .map(|i| Baro {
                time_s: f64::from(i),
                height_m: 0.0,
                smoothed_m: 0.0,
                rate_m_s: 0.0,
                scatter_var_m2: if i < 50 { 400.0 } else { 9.0 },
            })
            .collect();
        let var = flight_noise_var(&baro, 50.0, 0.0, 1.0);
        assert!((var - 8.0).abs() < 1e-9, "{var}");
        // No sample past it, as a log ending at its top: the whole flight's.
        let var = flight_noise_var(&baro, f64::INFINITY, 0.0, 1.0);
        assert!((var - 0.5 * (9.0 + 400.0) + 1.0).abs() < 1e-9, "{var}");
        assert_eq!(flight_noise_var(&[], 0.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn a_log_past_the_step_limit_is_withheld_before_any_work() {
        let short = log();
        let read = readings::read(&short);
        let mut long = short;
        long.time_s = (0..=MAX_STEPS).map(|i| i as f64 * 0.001).collect();
        long.altitude_m = vec![0.0; MAX_STEPS + 1];
        let withheld = smooth_with(&long, &read, Options::default()).unwrap_err();
        assert_eq!(withheld.reason, Reason::TooLong);
    }

    #[test]
    fn a_smoothed_apogee_past_the_readings_bound_says_so_on_the_card() {
        let log = log();
        let mut read = readings::read(&log);
        let clean = smooth_with(&log, &read, Options::default()).unwrap();
        assert!(!clean.health.apogee_past_reading_bound);
        assert!(clean.health.apogee_less_reading_m.unwrap().abs() < 100.0);

        let Reading::Read(apogee) = &mut read.apogee else {
            panic!("the invented log's apogee reads");
        };
        let bound = apogee.bound_m;
        // The reading low by twice its bound.
        apogee.height_m -= 2.0 * bound;
        let high = smooth_with(&log, &read, Options::default()).unwrap();
        assert!(high.health.apogee_past_reading_bound);
        assert!(high.health.apogee_less_reading_m.unwrap() > bound);
        assert!(
            high.health
                .card()
                .contains("past the reading's bound: go by the reading")
        );
        // A floor's truth may lie any amount above it: the same gap says nothing there.
        let Reading::Read(apogee) = &mut read.apogee else {
            panic!("the invented log's apogee reads");
        };
        apogee.is_floor = true;
        let above_floor = smooth_with(&log, &read, Options::default()).unwrap();
        assert!(!above_floor.health.apogee_past_reading_bound);
        // Below a floor by more than its bound it does.
        let Reading::Read(apogee) = &mut read.apogee else {
            panic!("the invented log's apogee reads");
        };
        apogee.height_m += 4.0 * bound;
        let below_floor = smooth_with(&log, &read, Options::default()).unwrap();
        assert!(below_floor.health.apogee_past_reading_bound);
    }

    #[test]
    fn the_health_card_reads_each_sensor_and_what_was_done() {
        let health = Health {
            sensors: vec![SensorHealth {
                sensor: Sensor::Barometer,
                samples: 2000,
                used: 1997,
                rejected: 3,
                unweighed: 0,
                resets: 1,
                inside_gate_share: Some(0.9985),
                largest: Some(Largest {
                    ratio: 12.345,
                    time_s: 17.04,
                    mach: 0.004,
                }),
                mean_nis: Some(0.7012),
            }],
            deweighted: vec![Span {
                start_s: 9.5,
                end_s: 14.25,
                why: Deweighted::Transonic,
            }],
            rejected: Vec::new(),
            gnss_trusted_s: None,
            baro_minus_inertial_at_apogee_m: None,
            unsmoothed_steps: 0,
            assumed_pad: false,
            accelerometer_set_aside: false,
            apogee_less_reading_m: None,
            apogee_past_reading_bound: false,
        };
        assert_eq!(
            health.card(),
            "barometer: 2000 samples, 1997 fused, 3 rejected, 0 not weighed, 1 reset; 99.8% \
             inside the gate; mean NIS 0.701; largest test ratio 12.3 at 17.0 s, Mach 0.00\n\
             barometer de-weighted: transonic 9.50–14.2 s\n\
             pass back: every step smoothed\n"
        );
        let assumed = Health {
            sensors: Vec::new(),
            assumed_pad: true,
            accelerometer_set_aside: true,
            apogee_less_reading_m: Some(-412.0),
            apogee_past_reading_bound: true,
            ..health
        };
        assert_eq!(
            assumed.card(),
            "pad assumed: too little of it in the log; heights on the logger's zero, noise from \
             the flight's scatter\n\
             accelerometer set aside: too little of the pad to take its level from\n\
             smoothed apogee -412 m from the reading, past the reading's bound: go by the reading\n\
             barometer de-weighted: transonic 9.50–14.2 s\n\
             pass back: every step smoothed\n"
        );
    }
}
