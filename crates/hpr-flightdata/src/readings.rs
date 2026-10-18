//! The readings taken from a flight log on its own: liftoff, apogee, the top speed, landing and
//! the descent, with no design file and no simulation.
//!
//! Each is a [`Reading`]: a value with where it came from, how it was read and the bound its error
//! stays within, or [`Reading::Withheld`] with the reason the log can't support it. A reading is
//! never guessed at; one the log can't support is left out and says why.
//!
//! **The track.** Every reading of a height or a time comes from one height track, the first of
//! these the log holds ([`Source`]): the logger's own barometric altitude (the record's primary
//! track); a pressure, turned into a height by the 1976 standard atmosphere, as every logger
//! documented does it (`docs/research/altimeter-height-formulas.md`); a barometric altitude kept
//! in a channel, above the pad or above sea level; or a GNSS receiver's altitude. A GNSS track
//! gives no reading: a receiver's filter lags a climb and overshoots its top, and its fix drops
//! out at liftoff ([`Reason::ReceiverLag`], [`Reason::ReceiverDropout`]).
//!
//! **Method.** The track is first filtered by a running median over [`MEDIAN_WINDOW_S`]
//! ([`crate::filter::running_median`]), which takes out the pressure pulse an ejection charge
//! punches into a barometric trace. Then:
//!
//! - the **climb** is the first filtered sample twice [`LIFTOFF_ALLOWANCE_M`] and the track's step
//!   above where the log starts: the allowance is the most the pad's noise lifts the filtered
//!   track, from below at the start or from above at the climb;
//! - the **pad** is the median of the samples to the last within half that of the start before
//!   the climb. A rocket that leaves the pad at [`LIFTOFF_ACCELERATION_M_S2`] or more, moving at
//!   `v` when the log starts, climbs `v t + A t²/2` while the track stays near its start, so its
//!   samples bound `v`, and with it how far up the rocket may already be, `v²/(2A)`: that much
//!   enters the bounds. Where it could pass the allowance, the pad is short
//!   ([`Reason::ShortPad`]): a track on the logger's own zero, or a channel's above the ground,
//!   reads its heights from that zero, and any other withholds them. A track that is above the
//!   pad only by a mapping's or a column's word must start within [`PAD_ZERO_M`] of its zero;
//! - **liftoff** is the last sample before apogee within the pad's allowance: how far the filtered
//!   pad strays from its median, at least [`LIFTOFF_ALLOWANCE_M`], and a step;
//! - **apogee** is the filtered track's highest value, at the middle of the run of samples that
//!   hold it (the track's step leaves a peak flat for a few samples), unless that top isn't a
//!   coast's ([`Reason::DisturbedTop`]): the track falls below a peak in the last
//!   [`COAST_CHECK_S`] and climbs back to it, or climbs into the top faster than a coast slowing
//!   at twice gravity and falls from it faster than a body at rest there ([`SPIKE_CHECK_S`]), as a
//!   charge's transient does;
//! - the **top speed** is the highest least-squares slope of the filtered track over
//!   [`SPEED_WINDOW_S`] from liftoff to apogee: a speed HPR Sim derives, withheld where the
//!   barometer's port may read the transonic flow ([`TRANSONIC_SPEED_M_S`]) and refused as
//!   Debrief refuses one ([`IMPLAUSIBLE_SPEED_M_S`], [`ASCENT_NOISE_FRACTION`]);
//! - **landing** is the first sample after apogee back within the pad's allowance, at least
//!   [`LANDING_HEIGHT_M`], that stays under it and [`LANDED_CEILING_M`]'s margin for
//!   [`LANDED_FOR_S`], and no sooner, give or take a sample, than a fall from rest in vacuum would
//!   lose that height, `√(2h/g)`: drag only slows a fall;
//! - the **top acceleration** is left to milestone [M7.2c][m7-2c], the rest of the readings:
//!   withheld when the log has no accelerometer, as differencing a height twice turns its step
//!   into spikes of many g, and not yet taken from one it has.
//!
//! Before any reading, a log must be sampled no slower than [`MAX_INTERVAL_S`] and in steps no
//! coarser than [`MAX_STEP_M`] (or [`MAX_PRESSURE_STEP_PA`]), the range the bounds were checked
//! over, on a clock that increases ([`Reason::ClockOutOfOrder`]); and its climb must lose more than
//! its noise and step in the [`COAST_CHECK_S`] before its top, as a coast does and as a clock in
//! the wrong unit, such as milliseconds read as seconds, can't.
//!
//! **Bounds.** Each reading states the bound its error stays within: the sum of the terms
//! [`docs/physics/log-readings.md`][page] lists, each from the noise model of HPR Sim's synthetic
//! flights ([ADR-280] §3) or measured on the log itself. Most of an apogee's bound is the day's
//! air: a logger converts pressure by the standard atmosphere, so a day [`AIR_TEMPERATURE_SPREAD_K`]
//! off standard moves its heights by up to [`TEMPERATURE_SHARE`] of themselves. On the synthetic
//! flights' tuning set, logged at every rate and step from 500 to half a sample a second and from
//! no step to 100 Pa, and started as loggers that record from launch start, from 0.3 s before
//! liftoff to 0.5 s after it, every reading given stayed within its bound (`cargo xtask
//! synthetic`).
//!
//! The landing's thresholds and the least climb, [`LIFTOFF_HEIGHT_M`], are Debrief's
//! (`lib/analyze/index.ts`, MIT, the project owner's own), set on its corpus of flight logs rather
//! than taken from a published source; the running median in place of Debrief's Hampel filter,
//! the climb's mark, the pad and its error, the liftoff allowance, the derived speed and the
//! bounds are HPR Sim's ([ADR-281]).
//!
//! [page]: https://hpr.fusionspace.co/physics/log-readings.html
//! [ADR-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
//! [ADR-281]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0281-readings-on-every-logger.md
//! [m7-2c]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c

use hpr_atmos::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use serde::{Deserialize, Serialize};

use crate::filter::{median, running_median};
use crate::log::{Channel, FlightLog, LogFormat, Quantity};

/// The running median's span, s: 0.3 s, Debrief's despiking window. Debrief runs a Hampel filter
/// over it ([`crate::filter::hampel`], threshold 4); HPR Sim takes the plain median (threshold 0). On
/// the public Pnut log Debrief ships, the ejection charge's pulse peaks at 1,028 ft, against the
/// 1,009 ft apogee the logger states. The samples around the pulse, a dip before it and a lasting
/// drop after it, widen its window's spread until the Hampel filter keeps it, and an apogee read
/// after it is the pulse. After the median it reads 1,010 ft. The median removes any pulse up to
/// half its window wide, and reads a noise-free peak bent by gravity alone no more than
/// [`peak_bound_m`] low, 0.077 m at 20 Hz. The Pnut numbers are checked only where the log has been
/// fetched into `refs/` (it isn't committed); CI checks an invented log of the same shape.
pub const MEDIAN_WINDOW_S: f64 = 0.3;
/// The most samples either side of its center the running median takes: HPR Sim's choice, the
/// [`MEDIAN_WINDOW_S`] window at over 6 kHz, faster than any logger HPR Sim reads. A log sampled
/// faster is withheld ([`Reason::SampledTooFast`]): the median's cost grows with its window.
pub const MAX_MEDIAN_HALF_WINDOW: usize = 1000;
/// The slowest a log may be sampled, s: the slowest rate the bounds were checked at on the
/// synthetic flights' tuning set, half a sample a second, below which the readings are withheld
/// ([`Reason::SampledTooSlowly`]). It is about twice a radio's telemetry interval, and a
/// millisecond clock read as seconds is slower at any rate under 500 samples a second.
pub const MAX_INTERVAL_S: f64 = 2.0;
/// The coarsest step a height track may have, m: the coarsest checked on the tuning set, above
/// the 8.8 m steps of the coarsest altitude track the readers read (MARSA's).
pub const MAX_STEP_M: f64 = 10.0;
/// The coarsest step a pressure track may have, Pa: the coarsest checked on the tuning set, above
/// the 92 Pa steps of the coarsest pressure track the readers read (an ARTS2's raw counts).
pub const MAX_PRESSURE_STEP_PA: f64 = 100.0;
/// The climb above the pad that marks a flight, m (Debrief's 3 m).
pub const LIFTOFF_HEIGHT_M: f64 = 3.0;
/// The least the filtered track may sit above the pad and still be on it, m: four times the
/// synthetic flights' fast correlated barometer noise (0.5 m, [ADR-280] §3), HPR Sim's choice.
/// Where the pad's own samples reach further, the allowance is their reach. Liftoff read at half a
/// foot above the pad, as before the readings were taken on every logger, came up to 6 s early on a
/// noisy 500 Hz barometer, the noise lifting the pad's last samples above it; with the pad's reach
/// measured but the allowance at 1 m, 6.8 s early at half a sample a second on one tuning flight,
/// against a 4.5 s bound, the noise moving the pad's last sample past the reach of the two before;
/// at 2 m, never.
///
/// [ADR-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
pub const LIFTOFF_ALLOWANCE_M: f64 = 2.0;
/// The least mean acceleration a rocket leaves the pad with over its first meters, m/s², which
/// sets how long it takes to climb out of the pad's noise and so how late liftoff can be read:
/// twice gravity, as the safety codes' 3:1 thrust to weight leaves on the motor's average thrust
/// (NAR's high-power code: liftoff weight no more than a third of the certified average thrust;
/// Tripoli's: below 3:1 only with an active stability system). The tuning set's slowest rocket
/// averages 30 m/s² over its first 5 m. A rocket slower off the pad is read later than the bound,
/// and may already be further up, when a log starts, than its pad's error allows.
pub const LIFTOFF_ACCELERATION_M_S2: f64 = 2.0 * STANDARD_GRAVITY_MPS2;
/// How close to the pad the altitude must come to mark landing, m (Debrief's 2 m).
pub const LANDING_HEIGHT_M: f64 = 2.0;
/// The height above the pad the altitude must then stay under, m (Debrief's 5 m)…
pub const LANDED_CEILING_M: f64 = 5.0;
/// …and for how long, s (Debrief's 1 s).
pub const LANDED_FOR_S: f64 = 1.0;
/// How far above the ground the logger may sit on the pad, m: it lands lower than it started by
/// as much, so its landing is read early by the time that height takes. HPR Sim's choice, a tall
/// rocket's electronics bay on a rail.
pub const PAD_ABOVE_GROUND_M: f64 = 3.0;
/// A top speed above this is refused, m/s (Debrief's ceiling).
pub const IMPLAUSIBLE_SPEED_M_S: f64 = 4000.0;
/// A top speed is refused when the climb's most negative speed is more than this share of it: a
/// trace swinging that far has no usable sign (Debrief's 20%).
pub const ASCENT_NOISE_FRACTION: f64 = 0.2;
/// The span of the least-squares fit the top speed is the slope of, s: HPR Sim's choice, long
/// enough to average the barometer's correlated noise down to a few meters a second, short enough
/// to keep most of the speed's peak.
pub const SPEED_WINDOW_S: f64 = 0.5;
/// The fewest samples the fit may take: the top speed is withheld on a log sampled slower than
/// [`SPEED_WINDOW_S`] over this less one.
pub const SPEED_MIN_SAMPLES: usize = 5;
/// The Mach number from which a barometer's port may read the transonic flow over the nose: the
/// ogive-cylinder's pressure coefficient behind its shoulder jumps between Mach 0.9 and 1.0
/// (`docs/research/altimeter-mach-error.md` §1–2), and on the tuning set a speed derived from it
/// was off by up to 850 m/s from Mach 0.9. HPR Sim's margin below it.
pub const TRANSONIC_MACH: f64 = 0.8;
/// The top speed at or above which it is withheld ([`Reason::Transonic`]), m/s:
/// [`TRANSONIC_MACH`] times the speed of sound in the coldest air the bounds allow below 20 km, the
/// standard atmosphere's 216.65 K less [`AIR_TEMPERATURE_SPREAD_K`]: `0.8 √(1.4 · 287.05 ·
/// 201.65)` = 227.7 m/s.
pub const TRANSONIC_SPEED_M_S: f64 = 227.736_107_633_374_67;
/// The pressure coefficient a port one caliber behind the nose's shoulder reads below Mach 0.9,
/// its size: the ogive-cylinder's measured −0.02 (`altimeter-mach-error.md` §1–2), the largest
/// the synthetic flights draw. A climb's dynamic pressure rises with its speed, so the port's
/// error lifts a speed derived while the rocket accelerates.
pub const PORT_PRESSURE_COEFFICIENT: f64 = 0.02;
/// How far the day's air may be from the 1976 standard atmosphere for the bounds to hold, K: the
/// synthetic flights' range ([ADR-280] §2).
///
/// [ADR-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
pub const AIR_TEMPERATURE_SPREAD_K: f64 = 15.0;
/// The share of a barometric height its error from the day's air can reach: a height read by the
/// standard atmosphere is `T_std/T` of the true one over the layer it spans (the hypsometric
/// equation), so off by `ΔT/T_std` at most, `T_std` no colder than the standard atmosphere's
/// 216.65 K below 20 km: 15/216.65 = 6.9%. About 0.35% a kelvin near the ground.
pub const TEMPERATURE_SHARE: f64 = AIR_TEMPERATURE_SPREAD_K / 216.65;
/// The share of a height a barometer's gain can be off after its pad zero: an MS5607's ±1.0 mbar
/// over the 400 mbar its datasheet states it on (an inference), the widest of the synthetic
/// flights' parts ([ADR-280] §3).
///
/// [ADR-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
pub const BAROMETER_GAIN: f64 = 0.0025;
/// The barometer's noise at a reading, m: what the fast and slow correlated noise, the part's
/// white noise, the pad's own median and the port's error at apogee's sideways speed add to a
/// height read off the filtered track, tuned on the tuning set.
pub const NOISE_M: f64 = 3.0;
/// The most a barometer's drift moves a height in a second, m: the synthetic flights' fastest
/// drift, 3 Pa in 30 s ([ADR-280] §3), at the density of air 5 km up (0.74 kg/m³).
///
/// [ADR-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
pub const DRIFT_M_PER_S: f64 = 0.015;
/// What the barometer's noise adds to a derived speed, m/s, tuned on the tuning set.
pub const SPEED_NOISE_M_S: f64 = 5.0;
/// How long before its top the track is checked, s. A coast loses `g t²/2` in the last `t` before
/// its top, 78 m in 4 s, gravity slowing it and drag only adding, and a flight whose motor burns
/// into those 4 s less; a track on a clock read a thousand times too slow, as milliseconds read as
/// seconds, loses about 80 µm, so a track must lose more than twice its noise and step. And a
/// coast doesn't fall and rise again: a track that falls more than that after a peak in those 4 s
/// and then climbs to its top has a top that isn't the coast's ([`Reason::DisturbedTop`]).
pub const COAST_CHECK_S: f64 = 4.0;
/// How long either side of its top the track is checked for a spike, s: a second, or two samples
/// of a slower track. A coast slowing at gravity and drag climbs no more than `g Δ²` in its last
/// Δ, and nothing falls from its top faster than gravity pulls it, so a track that climbs into
/// its top and falls from it faster than both holds a spike there ([`Reason::DisturbedTop`]), as
/// a breakup's in a public Blue Raven log does, 1.2 km over the coast. A parachute open before
/// apogee breaks only the first, a charge that lifts the bay's pressure at the top only the
/// second, and their tops are read.
pub const SPIKE_CHECK_S: f64 = 1.0;
/// The fastest the day's pressure changes in the ordinary run of weather, Pa an hour: the
/// National Weather Service's *pressure rising (falling) rapidly*, 0.06 inHg an hour or more
/// (its glossary; the threshold of METAR's `PRESRR` and `PRESFR` remarks).
pub const RAPID_PRESSURE_CHANGE_PA_PER_H: f64 = 0.06 * 3386.389;
/// How far from its zero a track whose heights are above the pad only by a mapping's or a column's
/// word may start and still be taken as a height above the pad, m: a logger zeroes on the pad, and
/// its zero moves by the weather and its noise before launch. An hour on the pad at
/// [`RAPID_PRESSURE_CHANGE_PA_PER_H`], at sea level's standard density, then [`NOISE_M`]: `203.2 /
/// (1.225 · 9.807) + 3` = 19.9 m. The hour is HPR Sim's choice. A track that starts further off is
/// withheld as [`Reason::StartsOffThePad`], as an altitude above sea level a column mapping calls
/// one above the ground reads (#646).
pub const PAD_ZERO_M: f64 =
    RAPID_PRESSURE_CHANGE_PA_PER_H / (1.225 * STANDARD_GRAVITY_MPS2) + NOISE_M;

/// A reading, or why the log can't support it.
///
/// Serialized, the status goes in beside the reading's own fields (`"status": "read"`), so `T`
/// must serialize as a map, as the reading structs here do.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Reading<T> {
    /// The reading, with where it came from.
    Read(T),
    /// The log can't support the reading.
    Withheld(Withheld),
}

impl<T> Reading<T> {
    /// The reading, if it wasn't withheld.
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Read(value) => Some(value),
            Self::Withheld(_) => None,
        }
    }

    /// Why it was withheld, if it was.
    pub fn reason(&self) -> Option<Reason> {
        match self {
            Self::Read(_) => None,
            Self::Withheld(withheld) => Some(withheld.reason),
        }
    }

    fn withheld(reason: Reason, detail: impl Into<String>) -> Self {
        Self::Withheld(Withheld {
            reason,
            detail: detail.into(),
        })
    }
}

/// Why a reading was withheld.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Withheld {
    /// The reason, as a code.
    pub reason: Reason,
    /// The reason in words, with the log's own numbers.
    pub detail: String,
}

/// Declares a fieldless enum and its `ALL`, every variant in the order declared, from one list,
/// so that a new variant can't be left out of `ALL`.
macro_rules! with_all {
    ($(#[$meta:meta])* pub enum $name:ident { $($(#[$variant_meta:meta])* $variant:ident,)* }) => {
        $(#[$meta])*
        pub enum $name {
            $($(#[$variant_meta])* $variant,)*
        }

        impl $name {
            /// Every variant, in the order declared: what a program that maps them, as the
            /// command line does, checks itself against.
            pub const ALL: &'static [Self] = &[$(Self::$variant),*];
        }
    };
}

with_all! {
/// The reason a reading was withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Reason {
    /// The track has fewer than three samples.
    TooShort,
    /// The altitude never climbs twice [`LIFTOFF_ALLOWANCE_M`] and the step above where the log
    /// starts, or [`LIFTOFF_HEIGHT_M`] above the pad, or the pad's allowance and a step if that is
    /// higher; or, for landing, falls too little past its step and noise for the descent's rate to
    /// be measured.
    NoClimb,
    /// A track whose heights are above the pad only by a mapping's or a column's word starts more
    /// than [`PAD_ZERO_M`] from its zero: the log didn't start on the pad, or its heights aren't
    /// above the pad, such as an altitude above sea level a column mapping calls one above the
    /// ground.
    StartsOffThePad,
    /// The log ends before the rocket is seen to land.
    EndsBeforeLanding,
    /// The altitude reaches the ground sooner than a fall from rest at apogee in vacuum could.
    FasterThanFreeFall,
    /// The top speed is above [`IMPLAUSIBLE_SPEED_M_S`].
    ImplausibleSpeed,
    /// The climb's speed swings negative by more than [`ASCENT_NOISE_FRACTION`] of its top.
    NoisySpeed,
    /// The log has no accelerometer.
    NoAccelerometer,
    /// The reading needs another, which was withheld.
    Needs,
    /// The record breaks what every reader guarantees: a primary track as long as its clock, on
    /// finite times that increase, and channels as long as their clocks. Only a record built by
    /// hand can.
    BadRecord,
    /// The samples come so often that the [`MEDIAN_WINDOW_S`] window would hold more than
    /// [`MAX_MEDIAN_HALF_WINDOW`] either side.
    SampledTooFast,
    /// The log holds no altitude, pressure or GNSS track.
    NoHeightTrack,
    /// The log's only height is an altitude whose sensor it doesn't state, such as a table's
    /// `Altitude` column read by its header: a barometer's bounds can't be stated for it.
    UnstatedSensor,
    /// The samples come further apart than [`MAX_INTERVAL_S`], or than the top speed's fit needs.
    SampledTooSlowly,
    /// The track's step is coarser than [`MAX_STEP_M`] or [`MAX_PRESSURE_STEP_PA`].
    StepTooCoarse,
    /// The top speed reaches [`TRANSONIC_SPEED_M_S`], where the barometer's port may read the
    /// transonic flow over the nose.
    Transonic,
    /// The only height is a GNSS receiver's, whose filter lags a climb and overshoots its top.
    ReceiverLag,
    /// The only height is a GNSS receiver's, whose fix drops out or lags at liftoff.
    ReceiverDropout,
    /// The track loses no more than twice its noise and step in the [`COAST_CHECK_S`] before its
    /// top: its clock is in another unit, such as milliseconds read as seconds, its top isn't a
    /// coast's, or it isn't a height.
    SlowerThanFreeCoast,
    /// The log has an accelerometer, from which HPR Sim doesn't take the reading yet (milestone
    /// M7.2c).
    NotYetTaken,
    /// The log's first samples can't rule out a rocket already more than [`LIFTOFF_ALLOWANCE_M`]
    /// up when it starts: it starts after liftoff, as a logger that records from launch writes
    /// its zero and then the climb, or holds too little of the pad to tell liftoff by. A track on
    /// the logger's own zero, or a channel's above the ground, still gives its heights from that
    /// zero, and withholds liftoff unless a sample sits within the allowance of it.
    ShortPad,
    /// The height track's clock doesn't increase, so no time can be read off it.
    ClockOutOfOrder,
    /// The track's top isn't a coast's: it falls more than twice its noise and step below a peak
    /// in the last [`COAST_CHECK_S`] before its top, then climbs to it, or climbs into it faster
    /// than a coast can and falls from it faster than gravity pulls a body at rest there
    /// ([`SPIKE_CHECK_S`]). A transient after apogee
    /// does either, such as a charge's that lasts longer than the median takes out and stands
    /// above the true top. Which is the apogee can't be told, so the apogee, the top speed and
    /// landing are withheld; liftoff is not.
    DisturbedTop,
}
}

with_all! {
/// The track a reading was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Source {
    /// The logger's barometric altitude, the record's primary track, above the logger's zero.
    Barometer,
    /// A pressure channel, turned into a height by the 1976 standard atmosphere and zeroed on
    /// the pad.
    Pressure,
    /// A barometric altitude channel, above the ground or above sea level, zeroed on the pad.
    AltitudeChannel,
    /// A GNSS receiver's altitude.
    Gnss,
}
}

with_all! {
/// Whether a reading was measured or derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Provenance {
    /// Read straight off the track: a height or a time it holds.
    Measured,
    /// Worked out from the track, such as a speed from the slope of its heights.
    Derived,
    /// Measured, but at the sensor's range, so the true value may lie beyond it (milestone
    /// M7.2c's top acceleration).
    Clipped,
}
}

with_all! {
/// How a reading was taken from its track.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Method {
    /// The last sample before apogee within the pad's allowance: [`LIFTOFF_ALLOWANCE_M`], or the
    /// filtered pad's own reach if that is more, and a step.
    LastOnThePad,
    /// The filtered track's highest value, at the middle of its flat run.
    HighestFiltered,
    /// The highest least-squares slope of the filtered track over [`SPEED_WINDOW_S`].
    SteepestSlope,
    /// The first sample after apogee back within the pad's allowance, as liftoff's, that stays
    /// down for [`LANDED_FOR_S`].
    BackOnThePad,
}
}

/// Liftoff.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Liftoff {
    /// The last sample on the pad, s on the track's clock: the filtered track was within the
    /// pad's allowance then, and never again before apogee.
    pub time_s: f64,
    /// The bound on its error, s: the time to climb twice the allowance at
    /// [`LIFTOFF_ACCELERATION_M_S2`], `√(4a/A)`, and the longer interval either side of the
    /// sample, twice.
    pub bound_s: f64,
    /// The track it came from.
    pub source: Source,
    /// Measured.
    pub provenance: Provenance,
    /// How it was read.
    pub method: Method,
}

/// The highest point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Apogee {
    /// When, s on the track's clock: the middle of the run of samples at the filtered track's
    /// highest value.
    pub time_s: f64,
    /// When, s after liftoff; `None` when liftoff is withheld.
    pub time_after_liftoff_s: Option<f64>,
    /// The bound on the time's error, s: the time the top of a coast takes to fall by the
    /// height's noise and step there, `√(2 (noise + step)/g)`, and the longest interval either
    /// side of the peak's flat run.
    pub time_bound_s: f64,
    /// The filtered track there, m above the pad.
    pub height_m: f64,
    /// The bound on the height's error, m: the day's air ([`TEMPERATURE_SHARE`]) and the
    /// barometer's gain ([`BAROMETER_GAIN`]) as shares of the height, then its noise
    /// ([`NOISE_M`]), drift ([`DRIFT_M_PER_S`]) since the log began, a step at the pad and one
    /// at the peak, and the median's [`peak_bound_m`].
    pub bound_m: f64,
    /// The filtered track there, m above the track's own zero: the logger's, for its own
    /// altitude; for a channel's, as the channel or the standard atmosphere gives it.
    pub altitude_m: f64,
    /// Whether the peak lies within half the median's window of the log's end, so the log may
    /// have stopped before the rocket did: the height is then a floor.
    pub is_floor: bool,
    /// The highest sample the track holds before the filter: above the apogee when the median
    /// set a pulse aside.
    pub highest_sample: Sample,
    /// The track it came from.
    pub source: Source,
    /// Measured.
    pub provenance: Provenance,
    /// How it was read.
    pub method: Method,
}

/// One sample of the track.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    /// When, s on the track's clock.
    pub time_s: f64,
    /// The height, m above the track's own zero.
    pub altitude_m: f64,
}

/// The top vertical speed in the climb.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MaxSpeed {
    /// The speed, m/s, up.
    pub speed_m_s: f64,
    /// The bound on its error, m/s: the day's air and the barometer's gain as shares of it; the
    /// port's error while the rocket accelerates, [`PORT_PRESSURE_COEFFICIENT`] `v a/g`; the fit
    /// cutting the peak, `(a₁ + a₂) w/4`, the climb's change of speed before and after it
    /// measured on the fits; and [`SPEED_NOISE_M_S`].
    pub bound_m_s: f64,
    /// When, s on the track's clock: the middle of the fit.
    pub time_s: f64,
    /// The filtered track then, m above the pad.
    pub height_m: f64,
    /// The track it came from.
    pub source: Source,
    /// Derived.
    pub provenance: Provenance,
    /// How it was read.
    pub method: Method,
}

/// The top acceleration in the climb. No reading fills it yet: milestone
/// [M7.2c](https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c), the rest of the readings,
/// takes it from an accelerometer.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MaxAcceleration {
    /// The acceleration, m/s².
    pub acceleration_m_s2: f64,
    /// When, s on the log's clock.
    pub time_s: f64,
}

/// Landing, and the descent before it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Landing {
    /// The first sample after apogee back within the pad's allowance, at least
    /// [`LANDING_HEIGHT_M`], s on the track's clock: before touchdown by the time the last meters
    /// took.
    pub time_s: f64,
    /// The bound on its error, s: the time the descent takes to fall the pad's allowance, the
    /// barometer's noise and drift, a step and the logger's height on the pad
    /// ([`PAD_ABOVE_GROUND_M`]), at its rate near the ground taken slow by its step and noise,
    /// and the longer interval either side of the sample, twice.
    pub bound_s: f64,
    /// From liftoff to landing, s; `None` when liftoff is withheld.
    pub flight_time_s: Option<f64>,
    /// The bound on the flight time's error, s: liftoff's and landing's; `None` when liftoff is
    /// withheld.
    pub flight_time_bound_s: Option<f64>,
    /// From apogee to landing, s.
    pub descent_time_s: f64,
    /// The mean rate of descent from apogee to landing, m/s: the height lost over the time taken,
    /// drogue and main together.
    pub mean_descent_rate_m_s: f64,
    /// The bound on the rate's error, m/s: the apogee's height bound and the height landing is
    /// read at, then the rate times the apogee's and landing's time bounds, over the descent time.
    pub mean_descent_rate_bound_m_s: f64,
    /// The track it came from.
    pub source: Source,
    /// Measured: the time. The rate is derived from it and apogee.
    pub provenance: Provenance,
    /// How it was read.
    pub method: Method,
}

/// What was read from a flight log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Readings {
    /// The track the readings came from; `None` if the log has none, or is withheld whole as
    /// [`Reason::BadRecord`].
    pub source: Option<Source>,
    /// The median interval between the track's samples, s; `None` for a log too short to read.
    pub sample_interval_s: Option<f64>,
    /// The track's step, m: the reader's for the logger's own altitude, else the smallest gap
    /// between two of its values, a pressure's at the pad.
    pub step_m: Option<f64>,
    /// The running median's span, s, whole samples of the interval: [`MEDIAN_WINDOW_S`] rounded.
    pub median_window_s: Option<f64>,
    /// How far below its true peak the running median can read a peak bent by gravity alone, m
    /// ([`peak_bound_m`]).
    pub peak_bound_m: Option<f64>,
    /// The pad, m on the track's own scale: the median of the samples surely before liftoff, or,
    /// in a log that starts after liftoff, the logger's zero; `None` when every reading is
    /// withheld before it is found.
    pub pad_altitude_m: Option<f64>,
    /// Liftoff.
    pub liftoff: Reading<Liftoff>,
    /// The highest point.
    pub apogee: Reading<Apogee>,
    /// The top vertical speed from liftoff to apogee.
    pub max_speed: Reading<MaxSpeed>,
    /// The top acceleration.
    pub max_acceleration: Reading<MaxAcceleration>,
    /// Landing, and the descent before it.
    pub landing: Reading<Landing>,
}

/// The height track the readings take.
struct Track {
    source: Source,
    time_s: Vec<f64>,
    /// Heights, m: above the logger's zero for its own altitude, else as the channel or the
    /// standard atmosphere gives them; the readings measure from the pad.
    height_m: Vec<f64>,
    /// The step, m: at the pad's pressure for a pressure track.
    step_m: f64,
    /// A pressure track's step, Pa.
    pressure_step_pa: Option<f64>,
    /// Whether its heights stand on a zero set on the pad: the logger's own altitude, which a
    /// [`FlightLog`] holds above the logger's zero, or a channel named as above the ground. A
    /// log that starts after liftoff reads its heights from that zero.
    zeroed: bool,
}

impl Track {
    /// The step at a height of the track, m: a pressure track's step in height grows as the air
    /// thins, `Δh = Δp/(ρ g)`, the density from the standard atmosphere at that height.
    fn step_at(&self, height_m: f64) -> f64 {
        match self.pressure_step_pa {
            Some(step_pa) => step_pa / (standard_density(height_m) * STANDARD_GRAVITY_MPS2),
            None => self.step_m,
        }
    }
}

/// The 1976 standard atmosphere's density at a pressure altitude, kg/m³, its sea-level 1.225
/// outside the model's range.
fn standard_density(height_m: f64) -> f64 {
    Ussa76::standard()
        .sample(height_m)
        .map_or(1.225, |sample| sample.air.density_kg_m3)
}

/// Takes the readings from a flight log.
pub fn read(log: &FlightLog) -> Readings {
    let max_acceleration = acceleration(log);
    let every = |source: Option<Source>, reason: Reason, detail: &str| Readings {
        source,
        sample_interval_s: None,
        step_m: None,
        median_window_s: None,
        peak_bound_m: None,
        pad_altitude_m: None,
        liftoff: Reading::withheld(reason, detail),
        apogee: Reading::withheld(reason, detail),
        max_speed: Reading::withheld(reason, detail),
        max_acceleration: max_acceleration.clone(),
        landing: Reading::withheld(reason, detail),
    };
    if let Err(detail) = check_record(log) {
        return every(None, Reason::BadRecord, &detail);
    }
    let track = match track(log) {
        Ok(track) => track,
        Err((reason, detail)) => return every(None, reason, &detail),
    };
    let source = Some(track.source);
    if track.source == Source::Gnss {
        let mut readings = every(
            source,
            Reason::ReceiverLag,
            "the log's only height is a GNSS receiver's, whose navigation filter lags a climb and \
             overshoots its top by more than a barometer's bounds allow",
        );
        readings.liftoff = Reading::withheld(
            Reason::ReceiverDropout,
            "the log's only height is a GNSS receiver's, whose fix drops out or lags at liftoff",
        );
        readings.landing = Reading::withheld(
            Reason::Needs,
            "landing is found against the pad after apogee, and liftoff and apogee were withheld",
        );
        return readings;
    }
    let time = &track.time_s;
    let n = time.len();
    let mut steps: Vec<f64> = time.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let (Some(interval), true) = (median(&mut steps), n >= 3) else {
        return every(
            source,
            Reason::TooShort,
            &format!("the track has {n} samples, too few to take a reading from"),
        );
    };
    if interval > MAX_INTERVAL_S {
        let mut readings = every(
            source,
            Reason::SampledTooSlowly,
            &format!(
                "the track's samples come every {interval:.3} s, slower than the {MAX_INTERVAL_S} s \
                 the readings were checked at; a clock in milliseconds read as seconds looks so"
            ),
        );
        readings.sample_interval_s = Some(interval);
        return readings;
    }
    let half = half_window(interval);
    if half > MAX_MEDIAN_HALF_WINDOW {
        let mut readings = every(
            source,
            Reason::SampledTooFast,
            &format!(
                "the track's samples come every {interval:.3e} s, so a {MEDIAN_WINDOW_S} s running \
                 median would take more than {MAX_MEDIAN_HALF_WINDOW} samples either side: faster \
                 than any logger HPR Sim reads"
            ),
        );
        readings.sample_interval_s = Some(interval);
        return readings;
    }
    let coarse = match track.pressure_step_pa {
        Some(step) if step > MAX_PRESSURE_STEP_PA => Some(format!(
            "the track's pressure moves in steps of {step:.1} Pa, coarser than the \
             {MAX_PRESSURE_STEP_PA} Pa the readings were checked at"
        )),
        None if track.step_m > MAX_STEP_M => Some(format!(
            "the track moves in steps of {:.2} m, coarser than the {MAX_STEP_M} m the readings \
             were checked at",
            track.step_m
        )),
        _ => None,
    };
    if let Some(detail) = coarse {
        let mut readings = every(source, Reason::StepTooCoarse, &detail);
        readings.sample_interval_s = Some(interval);
        readings.step_m = Some(track.step_m);
        return readings;
    }
    let step = track.step_m;
    let filtered = running_median(&track.height_m, half);
    let bound = peak_bound_m(half, interval);
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a few samples converts exactly"
    )]
    let window_s = 2.0 * half as f64 * interval;
    let mut readings = every(source, Reason::TooShort, "");
    readings.sample_interval_s = Some(interval);
    readings.step_m = Some(step);
    readings.median_window_s = Some(window_s);
    readings.peak_bound_m = Some(bound);

    // The climb: where the filtered track first rises `rise` above where the log starts, twice
    // the pad's allowance and step, which the pad's noise doesn't reach (the allowance is the
    // most it lifts the filtered pad), from below at the start or from above at the climb.
    let start = filtered
        .iter()
        .copied()
        .find(|h| h.is_finite())
        .unwrap_or(f64::NAN);
    let rise = 2.0 * (LIFTOFF_ALLOWANCE_M + step);
    let Some(apogee_index) = arg_max(&filtered) else {
        return no_climb(readings, rise);
    };
    let Some(climb) = filtered[..=apogee_index]
        .iter()
        .position(|h| *h >= start + rise)
    else {
        return no_climb(readings, rise);
    };
    // The pad: the median of the samples to the last within half the rise of the start before
    // the climb, the allowance and a step, so that the pad's noise doesn't end it early and the
    // climb lends it few samples.
    let on_pad = filtered[..climb]
        .iter()
        .rposition(|h| *h <= start + 0.5 * rise)
        .map_or(1, |last| last + 1);
    // How high the rocket may already be when the log starts. The track stays within half the
    // rise of its start for `stay_s`; a rocket that left the pad at [`LIFTOFF_ACCELERATION_M_S2`]
    // or more, moving at `v` at the start, climbs `v t + A t²/2` in that time, no more than the
    // rise with the noise at either end, so `v ≤ (rise − A stay²/2)/stay`, and it had climbed
    // `v²/(2A)`. A log that may start further up than the allowance has no pad to tell liftoff by:
    // a logger that records from launch writes its zero, then climbs.
    let stay_s = time[on_pad - 1] - time[0];
    let start_speed = if stay_s > 0.0 {
        ((rise - 0.5 * LIFTOFF_ACCELERATION_M_S2 * stay_s * stay_s) / stay_s).max(0.0)
    } else {
        f64::INFINITY
    };
    let pad_error_m = start_speed * start_speed / (2.0 * LIFTOFF_ACCELERATION_M_S2);
    let short_pad = pad_error_m > LIFTOFF_ALLOWANCE_M;
    // With no pad, the log's first sample stands for it: the zero a logger that records from
    // launch writes first, or the climb's first height in one that writes none.
    let measured_pad = if short_pad {
        track
            .height_m
            .iter()
            .copied()
            .find(|h| h.is_finite())
            .unwrap_or(start)
    } else {
        let mut before: Vec<f64> = track.height_m[..on_pad]
            .iter()
            .copied()
            .filter(|h| h.is_finite())
            .collect();
        median(&mut before).unwrap_or(start)
    };
    // A mapped table's primary track is a height above the pad only by the mapping's word
    // (#646), and a channel above the ground only by its column's; a reader's own altitude is by
    // its format's.
    let mapped = matches!(
        log.format,
        LogFormat::GenericTable | LogFormat::BinaryRecords | LogFormat::ConsoleCapture
    );
    let by_word = match track.source {
        Source::Barometer => mapped,
        Source::AltitudeChannel => track.zeroed,
        Source::Pressure | Source::Gnss => false,
    };
    if by_word && measured_pad.abs() > PAD_ZERO_M {
        let detail = format!(
            "the log starts {measured_pad:.2} m from the logger's zero, more than the \
             {PAD_ZERO_M:.1} m the weather and the noise move a zero on the pad: it didn't start \
             on the pad, or its heights aren't above the pad (an altitude above sea level mapped \
             as one above the ground reads so; map it as altitude_msl)"
        );
        readings.liftoff = Reading::withheld(Reason::StartsOffThePad, detail.clone());
        readings.apogee = Reading::withheld(Reason::StartsOffThePad, detail.clone());
        readings.max_speed = Reading::withheld(Reason::StartsOffThePad, detail.clone());
        readings.landing = Reading::withheld(Reason::StartsOffThePad, detail);
        return readings;
    }
    let short_detail = format!(
        "the track stays within {:.1} m of its start for {stay_s:.2} s, so the rocket may have \
         climbed {pad_error_m:.1} m before the log starts, more than the pad's {:.0} m allowance: \
         the log starts after liftoff, as a logger that records from launch does, or holds too \
         little of the pad to tell liftoff by",
        0.5 * rise,
        LIFTOFF_ALLOWANCE_M
    );
    // With no pad, a track whose heights stand on a zero set on the pad reads from that zero;
    // one with no such zero, as a pressure track, can't place the pad.
    if short_pad && !track.zeroed {
        let mut withheld = every(source, Reason::ShortPad, &short_detail);
        withheld.sample_interval_s = readings.sample_interval_s;
        withheld.step_m = readings.step_m;
        withheld.median_window_s = readings.median_window_s;
        withheld.peak_bound_m = readings.peak_bound_m;
        return withheld;
    }
    // A short pad read from the logger's zero is as far off as the zero; a pad of samples, as
    // far as the rocket may have climbed before them.
    let (pad, pad_error_m) = if short_pad {
        (0.0, 0.0)
    } else {
        (measured_pad, pad_error_m)
    };
    readings.pad_altitude_m = Some(pad);
    // How far the filtered pad reaches from its median, at least the allowance, which is all a
    // short pad, with no samples of it, has; liftoff is the last sample before apogee within that
    // and a step of the pad.
    let pad_samples = if short_pad {
        &[][..]
    } else {
        &filtered[..on_pad]
    };
    let reach = pad_samples
        .iter()
        .filter(|h| h.is_finite())
        .map(|h| (h - pad).abs())
        .fold(LIFTOFF_ALLOWANCE_M, f64::max);
    let allowance = reach + step;
    let climb_height = LIFTOFF_HEIGHT_M.max(allowance + step);
    let top = filtered[apogee_index];
    if top < pad + climb_height {
        return no_climb(readings, climb_height);
    }
    // Liftoff: the last sample before apogee within the pad's allowance. A log with none, its
    // pad short, starts above its zero's allowance: after liftoff, and its climb is searched from
    // its start.
    let level = pad + allowance;
    let on_the_pad = filtered[..apogee_index].iter().rposition(|h| *h <= level);
    let first = on_the_pad.unwrap_or(0);

    // The track's step leaves the peak flat over a run of samples: the apogee is the run's
    // middle.
    let run_end = apogee_index
        + filtered[apogee_index..]
            .iter()
            .take_while(|h| **h == top)
            .count()
        - 1;
    let apogee_s = 0.5 * (time[apogee_index] + time[run_end]);
    let height = top - pad;

    // A coast loses g t²/2 in the last t before its top, and a track on a clock a thousand times
    // too slow next to nothing: a track must lose more than twice its noise and step.
    let before_top = time[apogee_index] - COAST_CHECK_S;
    let least = 2.0 * (NOISE_M + track.step_at(top));
    if before_top > time[first] {
        let index = time.partition_point(|t| *t <= before_top).saturating_sub(1);
        let lost = top - filtered[index];
        if lost <= least {
            let coast = 0.5 * STANDARD_GRAVITY_MPS2 * COAST_CHECK_S * COAST_CHECK_S;
            let detail = format!(
                "the track loses {lost:.2} m in the {COAST_CHECK_S} s before its top, no more than \
                 twice its noise and step ({least:.1} m), where a coast loses {coast:.0} m: its \
                 clock is in another unit, such as milliseconds read as seconds, or it isn't a \
                 height"
            );
            let mut withheld = every(source, Reason::SlowerThanFreeCoast, &detail);
            withheld.sample_interval_s = readings.sample_interval_s;
            withheld.step_m = readings.step_m;
            withheld.median_window_s = readings.median_window_s;
            withheld.peak_bound_m = readings.peak_bound_m;
            withheld.pad_altitude_m = readings.pad_altitude_m;
            return withheld;
        }
    }
    // A coast doesn't fall and rise again: a track that falls more than twice its noise and step
    // below a peak in the last [`COAST_CHECK_S`] before its top, then climbs to it, holds a top
    // that isn't the coast's, as a deployment's disturbance after apogee does when it lasts longer
    // than the median takes out. Which is the apogee can't be told.
    let window = time
        .partition_point(|t| *t < before_top)
        .max(first)
        .min(apogee_index);
    let mut peak = (f64::NEG_INFINITY, window);
    let mut fall = (0.0, window, window);
    for (index, height) in filtered.iter().enumerate().take(apogee_index).skip(window) {
        if *height > peak.0 {
            peak = (*height, index);
        }
        if peak.0 - height > fall.0 {
            fall = (peak.0 - height, peak.1, index);
        }
    }
    // Nor does a coast climb into its top as fast as a spike does and fall from it as fast. Slowing
    // at `a`, gravity and drag's upward share, a coast climbs `a Δ²/2` in its last Δ, and `a < 2g`
    // while that share, `ρ |v| v_z / (2β)`, is under g: over the last second at sea level, for a
    // ballistic coefficient β = m/(C_D A) over 24 kg/m² on a vertical coast and over 150 kg/m² at
    // 120 m/s across, where a 29 mm rocket of 0.3 kg has about 800. And nothing falls from its top
    // faster than gravity pulls it. The filtered top can trail the true one, but only while the
    // track falls less than its noise and step twice over and the median's window, `w`, lets it:
    // a fall from rest gathers at most `v = √(2g (2(N + s) + g w²/8))` by then, and from the
    // windows' first sample to their last, `L`, falls no more than `v L + g L²/2`. A parachute
    // open before apogee slows the climb harder than that, but not the fall; a charge that lifts
    // the bay's pressure at the top drops the track faster, after the coast's own climb. A track
    // that climbs and falls faster than both, past twice its noise and step, holds a spike at its
    // top, and which is the apogee can't be told. Checked over [`SPIKE_CHECK_S`] either side of
    // the top, or two samples.
    let span_s = SPIKE_CHECK_S.max(2.0 * interval);
    let back = time
        .partition_point(|t| *t <= time[apogee_index] - span_s)
        .saturating_sub(1);
    let climbed = top - filtered[back];
    let climb_s = time[apogee_index] - time[back];
    let climb_most = STANDARD_GRAVITY_MPS2 * climb_s * climb_s + least;
    let window_at =
        |index: usize| time[(index + half).min(n - 1)] - time[index.saturating_sub(half)];
    let held_m = least + STANDARD_GRAVITY_MPS2 * window_at(apogee_index).powi(2) / 8.0;
    let gathered_m_s = (2.0 * STANDARD_GRAVITY_MPS2 * held_m).sqrt();
    let falling = (run_end + 1..n)
        .take_while(|&index| time[index] - time[run_end] <= span_s)
        .find_map(|index| {
            let fallen = top - filtered[index];
            let over_s = time[(index + half).min(n - 1)] - time[apogee_index.saturating_sub(half)];
            let most =
                gathered_m_s * over_s + 0.5 * STANDARD_GRAVITY_MPS2 * over_s * over_s + least;
            (fallen > most).then_some((fallen, index, most))
        });
    let spike = falling
        .filter(|_| back >= first && climbed > climb_most)
        .map(|(fallen, index, most)| {
            format!(
                "the track climbs {climbed:.1} m into its top at {:.2} s in {climb_s:.2} s, more \
                 than the {climb_most:.1} m a coast slowing at twice gravity climbs, and falls \
                 {fallen:.1} m from it by {:.2} s, more than the {most:.1} m a body at rest there \
                 can fall, each past twice its noise and step: its top is a spike, such as a \
                 charge's, so which is the apogee can't be told",
                time[apogee_index], time[index]
            )
        });
    let disturbed = spike.or_else(|| {
        (fall.0 > least).then(|| {
        let (fallen, from, to) = fall;
        format!(
            "the track falls {fallen:.1} m from a peak at {:.2} s to {:.2} s, more than twice its \
             noise and step ({least:.1} m), then climbs to its top at {:.2} s: a coast doesn't, \
             and a disturbance after apogee, such as the deployment's, does, so which is the \
             apogee can't be told",
            time[from], time[to], time[apogee_index]
        )
    })
    });

    // Liftoff is read late by the time the rocket takes to climb out of the pad's reach, once
    // above it and once below, at the least acceleration off the pad, and by a sample either side.
    let lifted = on_the_pad.map(|liftoff| {
        let bound_s = (4.0 * (allowance + pad_error_m) / LIFTOFF_ACCELERATION_M_S2).sqrt()
            + 2.0 * local_interval(time, liftoff);
        (liftoff, bound_s)
    });
    readings.liftoff = match lifted {
        Some((liftoff, bound_s)) => Reading::Read(Liftoff {
            time_s: time[liftoff],
            bound_s,
            source: track.source,
            provenance: Provenance::Measured,
            method: Method::LastOnThePad,
        }),
        None => Reading::withheld(
            Reason::ShortPad,
            format!(
                "{short_detail}; the track starts {:.2} m above the logger's zero, past the \
                 pad's {allowance:.1} m allowance",
                start - pad
            ),
        ),
    };
    if let Some(detail) = disturbed {
        readings.apogee = Reading::withheld(Reason::DisturbedTop, detail.clone());
        readings.max_speed = Reading::withheld(Reason::DisturbedTop, detail.clone());
        readings.landing = Reading::withheld(Reason::DisturbedTop, detail);
        return readings;
    }
    let highest = arg_max(&track.height_m).unwrap_or(apogee_index);
    let top_step = track.step_at(top);
    let height_bound = (TEMPERATURE_SHARE + BAROMETER_GAIN) * height.abs()
        + NOISE_M
        + pad_error_m
        + DRIFT_M_PER_S * (apogee_s - time[0])
        + step
        + top_step
        + bound;
    let time_bound = (2.0 * (NOISE_M + top_step) / STANDARD_GRAVITY_MPS2).sqrt()
        + local_interval(time, apogee_index).max(local_interval(time, run_end));
    readings.apogee = Reading::Read(Apogee {
        time_s: apogee_s,
        time_after_liftoff_s: lifted.map(|(index, _)| apogee_s - time[index]),
        time_bound_s: time_bound,
        height_m: height,
        bound_m: height_bound,
        altitude_m: top,
        is_floor: run_end + half >= n - 1,
        highest_sample: Sample {
            time_s: time[highest],
            altitude_m: track.height_m[highest],
        },
        source: track.source,
        provenance: Provenance::Measured,
        method: Method::HighestFiltered,
    });
    readings.max_speed = max_speed(
        time,
        &filtered,
        (first, apogee_index),
        pad,
        interval,
        track.source,
    );
    // A log that starts after liftoff may have missed a faster climb before its first samples.
    if let (None, Some(speed)) = (lifted, readings.max_speed.value())
        && speed.time_s - time[0] < SPEED_WINDOW_S
    {
        readings.max_speed = Reading::withheld(
            Reason::ShortPad,
            format!(
                "the climb's speed peaks within {SPEED_WINDOW_S} s of the log's start, and the log \
                 holds no pad: it may have climbed faster before its first sample"
            ),
        );
    }
    readings.landing = landing(
        &track,
        &filtered,
        pad,
        &Apex {
            index: apogee_index,
            time_s: apogee_s,
            height_bound_m: height_bound,
            time_bound_s: time_bound,
        },
        (lifted, allowance, pad_error_m),
    );
    readings
}

/// The readings of a track that never climbs `climb_height` above the pad.
fn no_climb(mut readings: Readings, climb_height: f64) -> Readings {
    let detail = format!("the altitude never climbs {climb_height:.1} m above the pad");
    readings.liftoff = Reading::withheld(Reason::NoClimb, detail.clone());
    readings.apogee = Reading::withheld(Reason::NoClimb, detail.clone());
    readings.max_speed = Reading::withheld(Reason::NoClimb, detail.clone());
    readings.landing = Reading::withheld(Reason::NoClimb, detail);
    readings
}

/// The longer of the two intervals either side of sample `index`: a logger that samples the pad
/// slowly and the flight fast leaves its liftoff sample a pad's interval from the next.
fn local_interval(time: &[f64], index: usize) -> f64 {
    let gap = |a: usize, b: usize| match (time.get(a), time.get(b)) {
        (Some(a), Some(b)) => b - a,
        _ => 0.0,
    };
    gap(index.wrapping_sub(1), index).max(gap(index, index + 1))
}

/// The track the readings take, the first the log holds: the logger's own altitude, a pressure,
/// a barometric altitude channel, a GNSS altitude; or why there is none.
fn track(log: &FlightLog) -> Result<Track, (Reason, String)> {
    if !log.time_s.is_empty() {
        let declared = log.format.altitude_resolution_m();
        let step = if declared > 0.0 {
            declared
        } else {
            smallest_step(&log.altitude_m)
        };
        return Ok(Track {
            source: Source::Barometer,
            time_s: log.time_s.clone(),
            height_m: log.altitude_m.clone(),
            step_m: step,
            pressure_step_pa: None,
            zeroed: true,
        });
    }
    let first = |quantity: Quantity| log.channels_of(quantity).next();
    if let Some(channel) = first(Quantity::Pressure) {
        let (time_s, values) = clocked(channel)?;
        let air = Ussa76::standard();
        let height_m: Vec<f64> = values
            .iter()
            .map(|p| air.pressure_altitude_m(*p).unwrap_or(f64::NAN))
            .collect();
        let step_pa = smallest_step(&values);
        // The step in height at the pad's pressure, dh/dp = −1/(ρg), the pad's density from the
        // standard atmosphere at the track's first height.
        let pad_m = height_m
            .iter()
            .copied()
            .find(|h| h.is_finite())
            .unwrap_or(0.0);
        return Ok(Track {
            source: Source::Pressure,
            time_s,
            height_m,
            step_m: step_pa / (standard_density(pad_m) * STANDARD_GRAVITY_MPS2),
            pressure_step_pa: Some(step_pa),
            zeroed: false,
        });
    }
    let channel_track = |channel: &Channel, source: Source| Track {
        source,
        time_s: channel.time_s.clone(),
        height_m: channel.values.clone(),
        step_m: smallest_step(&channel.values),
        pressure_step_pa: None,
        zeroed: false,
    };
    if let Some(channel) =
        first(Quantity::AltitudeAboveGround).or_else(|| first(Quantity::AltitudeMsl))
    {
        let (time_s, height_m) = clocked(channel)?;
        return Ok(Track {
            source: Source::AltitudeChannel,
            step_m: smallest_step(&height_m),
            time_s,
            height_m,
            pressure_step_pa: None,
            zeroed: channel.quantity == Quantity::AltitudeAboveGround,
        });
    }
    if let Some(channel) = first(Quantity::GnssAltitude) {
        return Ok(channel_track(channel, Source::Gnss));
    }
    if let Some(channel) = first(Quantity::Altitude) {
        // A GPX track's elevation is its receiver's: the format carries GNSS tracks.
        if log.format == LogFormat::Gpx {
            return Ok(channel_track(channel, Source::Gnss));
        }
        return Err((
            Reason::UnstatedSensor,
            format!(
                "the log's only height, `{}`, doesn't say whether a barometer or a GNSS receiver \
                 measured it; a column mapping can mark it as the barometer's primary track",
                channel.column
            ),
        ));
    }
    Err((
        Reason::NoHeightTrack,
        "the log holds no altitude, pressure or GNSS track".to_owned(),
    ))
}

/// The smallest gap between two of the track's distinct finite values, or 0 if it holds fewer
/// than two: a track's step, as a track in steps passes every level between its lowest and its
/// highest. Successive changes would miss it on a track sampled so slowly that every sample moves
/// several steps.
fn smallest_step(values: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    sorted.sort_by(f64::total_cmp);
    sorted.dedup();
    let smallest = sorted
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .fold(f64::INFINITY, f64::min);
    if smallest.is_finite() { smallest } else { 0.0 }
}

/// Whole samples either side of the running median's center: [`MEDIAN_WINDOW_S`] over two
/// intervals, rounded half up, at least one. At 20 Hz it is 3. A log's intervals are written to a
/// few decimals, so a rounding tie, such as 1.5 at 10 Hz, must not turn on the last bit of the
/// interval's median: a part in 10⁹ settles it upwards.
fn half_window(interval: f64) -> usize {
    let exact = MEDIAN_WINDOW_S / (2.0 * interval);
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a positive, finite count of samples, rounded, clamped to at least one"
    )]
    let half = (exact * (1.0 + 1e-9)).round().max(1.0) as usize;
    half
}

/// How far below its true peak the running median of `half` samples either side can read a trace
/// sampled every `interval` s and bent by gravity alone near its peak, m:
/// `g ((⌈half/2⌉ + ½) Δt)² / 2`. At the highest sample, `half + 1` of the window's samples lie
/// within `⌈half/2⌉` places of it, so the median is no lower than they are, and the true peak lies
/// within half a sample of the highest sample. At 20 Hz, 0.077 m.
pub fn peak_bound_m(half: usize, interval: f64) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a few samples converts exactly"
    )]
    let reach = (half.div_ceil(2) as f64 + 0.5) * interval;
    0.5 * STANDARD_GRAVITY_MPS2 * reach * reach
}

/// Whether the record holds to what every reader guarantees: one time per altitude, each channel
/// as long as its clock, and finite times that increase on the record's clock.
fn check_record(log: &FlightLog) -> Result<(), String> {
    let n = log.time_s.len();
    let channels = [
        ("altitude", Some(log.altitude_m.len())),
        ("speed", log.vertical_speed_m_s.as_ref().map(Vec::len)),
        ("temperature", log.temperature_k.as_ref().map(Vec::len)),
        ("battery", log.battery_v.as_ref().map(Vec::len)),
    ];
    for (name, len) in channels {
        if let Some(len) = len
            && len != n
        {
            return Err(format!(
                "the record has {n} times and {len} {name} samples: a reader gives each channel \
                 one sample per time"
            ));
        }
    }
    increasing("the record's", &log.time_s)?;
    for channel in &log.channels {
        if channel.values.len() != channel.time_s.len() {
            return Err(format!(
                "channel `{}` has {} times and {} values: a reader gives a channel one value per \
                 time",
                channel.column,
                channel.time_s.len(),
                channel.values.len()
            ));
        }
    }
    Ok(())
}

/// A channel's samples on its clock, those with no time dropped, as a track takes them; or, if
/// its clock doesn't increase, why not. A reader keeps a channel's clock as the file writes it,
/// which can repeat or run back, as a GNSS receiver's fixes or a team's merged table do.
fn clocked(channel: &Channel) -> Result<(Vec<f64>, Vec<f64>), (Reason, String)> {
    let (time_s, values): (Vec<f64>, Vec<f64>) = channel
        .time_s
        .iter()
        .zip(&channel.values)
        .filter(|(t, _)| t.is_finite())
        .map(|(t, v)| (*t, *v))
        .unzip();
    increasing(&format!("channel `{}`'s", channel.column), &time_s)
        .map_err(|detail| (Reason::ClockOutOfOrder, detail))?;
    Ok((time_s, values))
}

/// Whether `times` are finite and increase.
fn increasing(whose: &str, times: &[f64]) -> Result<(), String> {
    match times
        .windows(2)
        .position(|pair| !(pair[0].is_finite() && pair[1].is_finite() && pair[1] > pair[0]))
    {
        Some(index) => Err(format!(
            "{whose} times don't increase at sample {}: {} s, then {} s",
            index + 1,
            times[index],
            times[index + 1]
        )),
        None => Ok(()),
    }
}

/// The index of the first greatest finite value.
fn arg_max(values: &[f64]) -> Option<usize> {
    values
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .fold(
            None,
            |best: Option<(usize, f64)>, (index, value)| match best {
                Some((_, top)) if *value <= top => best,
                _ => Some((index, *value)),
            },
        )
        .map(|(index, _)| index)
}

/// The top acceleration, withheld: milestone
/// [M7.2c](https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c), the rest of the readings,
/// takes it from an accelerometer, and none is taken from a height.
fn acceleration(log: &FlightLog) -> Reading<MaxAcceleration> {
    if log.channels_of(Quantity::Acceleration).next().is_some() {
        Reading::withheld(
            Reason::NotYetTaken,
            "HPR Sim doesn't take the top acceleration from an accelerometer yet: roadmap \
             milestone M7.2c does",
        )
    } else {
        Reading::withheld(
            Reason::NoAccelerometer,
            "the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, \
             as its steps would read as spikes of many g",
        )
    }
}

/// The least-squares slope of `heights` against `times` over the samples within half of
/// [`SPEED_WINDOW_S`] of `times[center]`, skipping gaps; `None` with fewer than
/// [`SPEED_MIN_SAMPLES`].
fn slope(times: &[f64], heights: &[f64], center: usize) -> Option<f64> {
    let t0 = times[center];
    let reach = 0.5 * SPEED_WINDOW_S;
    let from = times.partition_point(|t| *t < t0 - reach);
    let to = times.partition_point(|t| *t <= t0 + reach);
    let window = || {
        times[from..to]
            .iter()
            .zip(&heights[from..to])
            .filter(|(_, h)| h.is_finite())
            .map(|(t, h)| (t - t0, *h))
    };
    let (count, sum_t, sum_h) = window().fold((0_usize, 0.0, 0.0), |(c, st, sh), (t, h)| {
        (c + 1, st + t, sh + h)
    });
    if count < SPEED_MIN_SAMPLES {
        return None;
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of at most a few thousand samples converts exactly"
    )]
    let count_f = count as f64;
    let (mean_t, mean_h) = (sum_t / count_f, sum_h / count_f);
    let (num, den) = window().fold((0.0, 0.0), |(num, den), (t, h)| {
        (
            num + (t - mean_t) * (h - mean_h),
            den + (t - mean_t) * (t - mean_t),
        )
    });
    (den > 0.0).then(|| num / den)
}

/// The top vertical speed from liftoff to apogee, or why it is withheld.
fn max_speed(
    time: &[f64],
    filtered: &[f64],
    (liftoff, apogee): (usize, usize),
    pad: f64,
    interval: f64,
    source: Source,
) -> Reading<MaxSpeed> {
    #[expect(
        clippy::cast_precision_loss,
        reason = "a handful of samples converts exactly"
    )]
    let slowest = SPEED_WINDOW_S / (SPEED_MIN_SAMPLES - 1) as f64;
    if interval > slowest {
        return Reading::withheld(
            Reason::SampledTooSlowly,
            format!(
                "the track's samples come every {interval:.3} s, too far apart for a \
                 {SPEED_WINDOW_S} s fit of {SPEED_MIN_SAMPLES} samples, which needs {slowest} s"
            ),
        );
    }
    let speeds: Vec<(usize, f64)> = (liftoff..=apogee)
        .filter_map(|index| slope(time, filtered, index).map(|v| (index, v)))
        .collect();
    let Some(&(index, top)) = speeds
        .iter()
        .fold(None, |best: Option<&(usize, f64)>, item| match best {
            Some(b) if item.1 <= b.1 => best,
            _ => Some(item),
        })
    else {
        return Reading::withheld(
            Reason::SampledTooSlowly,
            "the climb is too short for a single fit of the speed",
        );
    };
    let worst = speeds.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min);
    if top > IMPLAUSIBLE_SPEED_M_S {
        return Reading::withheld(
            Reason::ImplausibleSpeed,
            format!("the climb's speed peaks at {top:.1} m/s, above {IMPLAUSIBLE_SPEED_M_S} m/s"),
        );
    }
    if top >= TRANSONIC_SPEED_M_S {
        return Reading::withheld(
            Reason::Transonic,
            format!(
                "the climb's speed reaches {top:.1} m/s, at or above {TRANSONIC_SPEED_M_S:.1} m/s, \
                 where the barometer's port may read the transonic flow over the nose"
            ),
        );
    }
    if top <= 0.0 || -worst > ASCENT_NOISE_FRACTION * top {
        return Reading::withheld(
            Reason::NoisySpeed,
            format!(
                "the speed from liftoff to apogee swings from {worst:.1} to {top:.1} m/s: a \
                 negative swing over {:.0}% of the top has no usable sign",
                ASCENT_NOISE_FRACTION * 100.0
            ),
        );
    }
    // The climb's change of speed either side of the top, from the fits a window away: a fit
    // cuts a peak by about (a₁ + a₂) w/8; twice that is allowed.
    let at = |t: f64| {
        let target = time.partition_point(|s| *s < t).clamp(liftoff, apogee);
        speeds
            .iter()
            .min_by_key(|(i, _)| (i.abs_diff(target), *i))
            .map_or(top, |(_, v)| *v)
    };
    let rising = ((top - at(time[index] - SPEED_WINDOW_S)) / SPEED_WINDOW_S).max(0.0);
    let falling = ((top - at(time[index] + SPEED_WINDOW_S)) / SPEED_WINDOW_S).max(0.0);
    let bound_m_s = (TEMPERATURE_SHARE + BAROMETER_GAIN) * top
        + PORT_PRESSURE_COEFFICIENT * top * rising / STANDARD_GRAVITY_MPS2
        + 0.25 * (rising + falling) * SPEED_WINDOW_S
        + SPEED_NOISE_M_S;
    Reading::Read(MaxSpeed {
        speed_m_s: top,
        bound_m_s,
        time_s: time[index],
        height_m: filtered[index] - pad,
        source,
        provenance: Provenance::Derived,
        method: Method::SteepestSlope,
    })
}

/// Apogee, as landing needs it.
struct Apex {
    index: usize,
    time_s: f64,
    height_bound_m: f64,
    time_bound_s: f64,
}

/// Landing and the descent, or why they are withheld.
fn landing(
    track: &Track,
    filtered: &[f64],
    pad: f64,
    apex: &Apex,
    (lifted, allowance, pad_error_m): (Option<(usize, f64)>, f64, f64),
) -> Reading<Landing> {
    let time = &track.time_s;
    let end = time[filtered.len() - 1];
    // Debrief's marks, raised to the pad's own reach and step where those are more, as the
    // liftoff's is, so a coarse or noisy track flickering on the ground doesn't hold landing off.
    let level = LANDING_HEIGHT_M.max(allowance);
    let ceiling = level + (LANDED_CEILING_M - LANDING_HEIGHT_M) + track.step_m;
    let landed = (apex.index + 1..filtered.len()).find(|&index| {
        filtered[index] < pad + level
            && time[index] + LANDED_FOR_S <= end
            && filtered[index..]
                .iter()
                .zip(&time[index..])
                .take_while(|(_, t)| **t <= time[index] + LANDED_FOR_S)
                .all(|(h, _)| *h < pad + ceiling)
    });
    let Some(index) = landed else {
        return Reading::withheld(
            Reason::EndsBeforeLanding,
            format!(
                "the log ends at {end:.2} s, {:.1} m above the pad, before the altitude comes \
                 within {level:.1} m of it and stays under {ceiling:.1} m for {LANDED_FOR_S} s",
                filtered[filtered.len() - 1] - pad
            ),
        );
    };
    let descent_time_s = time[index] - apex.time_s;
    let drop = filtered[apex.index] - filtered[index];
    // The quickest any fall from rest at apogee can lose that height: in vacuum. Both heights
    // are stepped, so the drop can read up to one step long; and the apogee's time, the middle
    // of its flat run, can sit about half a sample from the true one, so a sample is allowed. A
    // test passes a vacuum fall in feet at 10 to 100 samples a second, wherever its apogee falls
    // within a foot and its liftoff between samples, and fails without either.
    let quickest = (2.0 * (drop - track.step_m).max(0.0) / STANDARD_GRAVITY_MPS2).sqrt();
    if descent_time_s + local_interval(time, index) < quickest {
        return Reading::withheld(
            Reason::FasterThanFreeFall,
            format!(
                "the altitude reaches the pad {descent_time_s:.2} s after apogee, sooner than a \
                 fall from rest in vacuum could ({quickest:.2} s): the trace isn't a height there"
            ),
        );
    }
    // The descent's rate near the ground, which sets how early the landing mark and the logger's
    // height on the pad put it: over the last 5 s before landing, or back to where the track was
    // four times its step and noise higher if that is longer, and taken a step and the noise at
    // either end slow, as a stepped, noisy drop can read that much long.
    let margin_m = track.step_m + 2.0 * NOISE_M;
    let back = time
        .partition_point(|t| *t < time[index] - 5.0)
        .min(
            (apex.index..index)
                .rev()
                .find(|&back| filtered[back] - filtered[index] >= 4.0 * margin_m)
                .unwrap_or(apex.index),
        )
        .max(apex.index);
    let rate = (filtered[back] - filtered[index] - margin_m) / (time[index] - time[back]);
    if rate.is_nan() || rate <= 0.0 {
        return Reading::withheld(
            Reason::NoClimb,
            format!(
                "the track falls {drop:.1} m from apogee to landing, too little past its step and \
                 noise ({margin_m:.1} m) to time the descent"
            ),
        );
    }
    let mean_rate = drop / descent_time_s;
    let early_m = level
        + NOISE_M
        + pad_error_m
        + PAD_ABOVE_GROUND_M
        + track.step_m
        + DRIFT_M_PER_S * (time[index] - time[0]);
    let bound_s = early_m / rate + 2.0 * local_interval(time, index);
    let rate_bound = (apex.height_bound_m + early_m + mean_rate * (apex.time_bound_s + bound_s))
        / descent_time_s;
    Reading::Read(Landing {
        time_s: time[index],
        bound_s,
        flight_time_s: lifted.map(|(liftoff, _)| time[index] - time[liftoff]),
        flight_time_bound_s: lifted.map(|(_, liftoff_bound_s)| liftoff_bound_s + bound_s),
        descent_time_s,
        mean_descent_rate_m_s: mean_rate,
        mean_descent_rate_bound_m_s: rate_bound,
        source: track.source,
        provenance: Provenance::Measured,
        method: Method::BackOnThePad,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::Stated;

    const DT: f64 = 0.05;

    /// A log sampled every 0.05 s to `end_s`, its height from `flight`.
    fn log(end_s: f64, flight: impl Fn(f64) -> f64) -> FlightLog {
        log_every(DT, end_s, flight)
    }

    /// A log sampled every `dt` s to `end_s`, its height from `flight`.
    fn log_every(dt: f64, end_s: f64, flight: impl Fn(f64) -> f64) -> FlightLog {
        let times: Vec<f64> = (0..)
            .map(|i| f64::from(i) * dt)
            .take_while(|t| *t <= end_s + 1e-9)
            .collect();
        let altitude_m: Vec<f64> = times.iter().map(|t| flight(*t)).collect();
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

    /// The same log with its track moved to a channel of `quantity`, the record's own emptied,
    /// each height turned by `value`.
    fn as_channel(log: &FlightLog, quantity: Quantity, value: impl Fn(f64) -> f64) -> FlightLog {
        let mut moved = log.clone();
        moved.format = LogFormat::GenericTable;
        moved.channels = vec![Channel::new(
            "track",
            quantity,
            log.time_s.clone(),
            log.altitude_m.iter().map(|h| value(*h)).collect(),
        )];
        moved.time_s.clear();
        moved.altitude_m.clear();
        moved
    }

    /// A climb at 4 g from 1 s to 2 s (19.6 m, 39.2 m/s), a vacuum coast to apogee at 6 s
    /// (98.1 m, `10 g` in meters), then down at `descent` m/s to the pad.
    fn flight(descent: f64) -> impl Fn(f64) -> f64 {
        let g = STANDARD_GRAVITY_MPS2;
        move |t| {
            if t <= 1.0 {
                0.0
            } else if t < 2.0 {
                2.0 * g * (t - 1.0).powi(2)
            } else if t < 6.0 {
                let c = t - 2.0;
                2.0 * g + 4.0 * g * c - 0.5 * g * c * c
            } else {
                let h = 10.0 * g - descent * (t - 6.0);
                h.max(0.0)
            }
        }
    }

    /// The test flight reads: liftoff by the allowance's climb after 1 s, apogee at 6 s, the top
    /// speed at 2 s, landing below 2 m, each within its bound.
    #[test]
    fn a_plain_flight_reads() {
        let readings = read(&log(30.0, flight(10.0)));
        assert_eq!(readings.source, Some(Source::Barometer));
        let liftoff = readings.liftoff.value().unwrap();
        // 2 m above the pad at 1 + √(1/g) = 1.32 s: the last sample at or below it.
        assert!((liftoff.time_s - 1.3).abs() < 1e-9, "{}", liftoff.time_s);
        assert!(liftoff.time_s - 1.0 <= liftoff.bound_s);
        let apogee = readings.apogee.value().unwrap();
        let g = STANDARD_GRAVITY_MPS2;
        assert!((apogee.time_s - 6.0).abs() <= apogee.time_bound_s);
        assert!((apogee.height_m - 10.0 * g).abs() <= apogee.bound_m);
        // The peak is a corner, the descent starting at once: the median takes 0.36 m off it.
        assert!(
            (apogee.height_m - 10.0 * g).abs() < 0.5,
            "{}",
            apogee.height_m
        );
        assert_eq!(apogee.provenance, Provenance::Measured);
        let speed = readings.max_speed.value().unwrap();
        assert!((speed.speed_m_s - 4.0 * g).abs() <= speed.bound_m_s);
        assert!(
            (speed.time_s - 2.0).abs() <= 0.5 * SPEED_WINDOW_S,
            "{}",
            speed.time_s
        );
        assert_eq!(speed.provenance, Provenance::Derived);
        assert_eq!(speed.method, Method::SteepestSlope);
        let landing = readings.landing.value().unwrap();
        // 98.1 m at 10 m/s from 6 s: down at 15.81 s.
        let touchdown = 6.0 + g;
        assert!((landing.time_s - touchdown).abs() <= landing.bound_s);
        assert!(landing.time_s < touchdown);
        let rate = 10.0 * g / (touchdown - 6.0);
        assert!(
            (landing.mean_descent_rate_m_s - rate).abs() <= landing.mean_descent_rate_bound_m_s
        );
        assert_eq!(
            readings.max_acceleration.reason(),
            Some(Reason::NoAccelerometer)
        );
    }

    /// The same flight reads alike from a pressure channel, a barometric altitude channel above
    /// sea level and a primary track; a GNSS altitude gives no reading, by name.
    #[test]
    fn every_track_reads_but_gnss() {
        let primary = log(30.0, flight(10.0));
        let air = Ussa76::standard();
        let site = 1200.0;
        let pressure = as_channel(&primary, Quantity::Pressure, |h| {
            air.sample(site + h).unwrap().air.pressure_pa
        });
        let msl = as_channel(&primary, Quantity::AltitudeMsl, |h| site + h);
        let base = read(&primary);
        let height = base.apogee.value().unwrap().height_m;
        for (record, source) in [
            (&pressure, Source::Pressure),
            (&msl, Source::AltitudeChannel),
        ] {
            let readings = read(record);
            assert_eq!(readings.source, Some(source));
            let apogee = readings.apogee.value().unwrap();
            assert_eq!(apogee.source, source);
            // Geometric and geopotential heights differ by 0.02% here.
            assert!(
                (apogee.height_m - height).abs() < 0.05,
                "{source:?}: {}",
                apogee.height_m
            );
            assert!(readings.landing.value().is_some(), "{source:?}");
            assert!(readings.liftoff.value().is_some(), "{source:?}");
        }
        let gnss = read(&as_channel(&primary, Quantity::GnssAltitude, |h| site + h));
        assert_eq!(gnss.source, Some(Source::Gnss));
        assert_eq!(gnss.liftoff.reason(), Some(Reason::ReceiverDropout));
        assert_eq!(gnss.apogee.reason(), Some(Reason::ReceiverLag));
        assert_eq!(gnss.max_speed.reason(), Some(Reason::ReceiverLag));
        assert_eq!(gnss.landing.reason(), Some(Reason::Needs));
        let mut gpx = as_channel(&primary, Quantity::Altitude, |h| site + h);
        assert_eq!(read(&gpx).apogee.reason(), Some(Reason::UnstatedSensor));
        gpx.format = LogFormat::Gpx;
        assert_eq!(read(&gpx).apogee.reason(), Some(Reason::ReceiverLag));
        let mut nothing = as_channel(&primary, Quantity::Temperature, |h| h);
        assert_eq!(read(&nothing).apogee.reason(), Some(Reason::NoHeightTrack));
        nothing.channels.clear();
        assert_eq!(read(&nothing).landing.reason(), Some(Reason::NoHeightTrack));
    }

    /// Each withheld reading names the reason that fired, and only the readings that need it go.
    #[test]
    fn each_refusal_names_its_reason() {
        let short = read(&log(0.05, flight(10.0)));
        for reason_of in [
            short.liftoff.reason(),
            short.apogee.reason(),
            short.max_speed.reason(),
            short.landing.reason(),
        ] {
            assert_eq!(reason_of, Some(Reason::TooShort));
        }

        let flat = read(&log(10.0, |_| 0.5));
        assert_eq!(flat.apogee.reason(), Some(Reason::NoClimb));
        assert_eq!(flat.landing.reason(), Some(Reason::NoClimb));

        // Starting 19.6 m up and climbing, as a log that starts after liftoff: no pad. A reader's
        // altitude stands on the logger's zero, so the apogee reads from it; liftoff, before the
        // log, and the top speed, at its start, are withheld.
        let aloft = read(&log(30.0, |t| flight(10.0)(t + 2.0)));
        assert_eq!(aloft.pad_altitude_m, Some(0.0));
        assert_eq!(aloft.liftoff.reason(), Some(Reason::ShortPad));
        assert_eq!(aloft.max_speed.reason(), Some(Reason::ShortPad));
        // The median reads the sharp top, where the descent starts at 10 m/s, 0.11 m low.
        let apogee = aloft.apogee.value().unwrap();
        assert!(
            (apogee.height_m - 10.0 * STANDARD_GRAVITY_MPS2).abs() < 0.2,
            "{}",
            apogee.height_m
        );
        assert_eq!(apogee.time_after_liftoff_s, None);
        let landing = aloft.landing.value().unwrap();
        assert_eq!(
            (landing.flight_time_s, landing.flight_time_bound_s),
            (None, None)
        );
        // A mapped table that writes its zero, then the climb from 23.5 m up, as a logger that
        // records from launch does: its first sample says its heights stand on the pad, which
        // the median, lifted into the climb at the start, wouldn't.
        let mut zero_first = log(30.0, |t| flight(10.0)(t + 2.1));
        zero_first.format = LogFormat::GenericTable;
        zero_first.altitude_m[0] = 0.0;
        let zero_first = read(&zero_first);
        assert_eq!(zero_first.pad_altitude_m, Some(0.0));
        assert_eq!(zero_first.liftoff.reason(), Some(Reason::ShortPad));
        assert!(zero_first.apogee.value().is_some());
        // A track with no zero on the pad, as a pressure track, can't place it: every height
        // reading is withheld.
        let pressure = as_channel(
            &log(30.0, |t| flight(10.0)(t + 2.0)),
            Quantity::Pressure,
            |h| 101_325.0 - 12.0 * h,
        );
        let pressure = read(&pressure);
        for reason_of in [
            pressure.liftoff.reason(),
            pressure.apogee.reason(),
            pressure.max_speed.reason(),
            pressure.landing.reason(),
        ] {
            assert_eq!(reason_of, Some(Reason::ShortPad));
        }

        // A record a reader would never give: refused whole, and saying why.
        let mut short_speed = log(20.0, flight(10.0));
        short_speed.vertical_speed_m_s = Some(vec![0.0; 3]);
        let refused = read(&short_speed);
        assert_eq!(refused.apogee.reason(), Some(Reason::BadRecord));
        assert_eq!(refused.landing.reason(), Some(Reason::BadRecord));
        let mut repeated = log(20.0, flight(10.0));
        repeated.time_s[10] = repeated.time_s[9];
        assert_eq!(read(&repeated).liftoff.reason(), Some(Reason::BadRecord));
        // A channel's clock is the file's: one that runs back can't be read off, and one with a
        // time that isn't a number drops that sample.
        let pressure = as_channel(&log(20.0, flight(10.0)), Quantity::Pressure, |h| {
            101_325.0 - 12.0 * h
        });
        let mut channel_back = pressure.clone();
        channel_back.channels[0].time_s[10] = 0.0;
        assert_eq!(
            read(&channel_back).apogee.reason(),
            Some(Reason::ClockOutOfOrder)
        );
        let mut channel_gap = pressure;
        channel_gap.channels[0].time_s[10] = f64::NAN;
        assert!(read(&channel_gap).apogee.value().is_some());

        let cut = read(&log(8.0, flight(10.0)));
        assert_eq!(cut.landing.reason(), Some(Reason::EndsBeforeLanding));
        assert!(cut.apogee.value().is_some());

        // 98.1 m in 2.5 s: a vacuum fall from rest takes √(2 · 98.1/g) = 4.5 s.
        let dropped = read(&log(20.0, flight(4.0 * STANDARD_GRAVITY_MPS2)));
        assert_eq!(dropped.landing.reason(), Some(Reason::FasterThanFreeFall));

        // Still climbing at the end: the apogee is a floor.
        let climbing = read(&log(4.0, flight(10.0)));
        assert!(climbing.apogee.value().unwrap().is_floor);
        assert_eq!(climbing.landing.reason(), Some(Reason::EndsBeforeLanding));

        // An accelerometer's top acceleration is left to M7.2c.
        let mut accelerometer = log(30.0, flight(10.0));
        accelerometer.channels.push(Channel::new(
            "accel",
            Quantity::Acceleration,
            accelerometer.time_s.clone(),
            vec![0.0; accelerometer.time_s.len()],
        ));
        assert_eq!(
            read(&accelerometer).max_acceleration.reason(),
            Some(Reason::NotYetTaken)
        );
    }

    /// The pad is the median of everything before the climb: a jitter in the first samples
    /// doesn't move liftoff to the start of the log.
    #[test]
    fn early_jitter_leaves_the_pad_where_it_is() {
        let foot = crate::perfectflite::FOOT_M;
        let mut log = log(30.0, |t| flight(10.0)(t - 2.0));
        for (index, feet) in [0.0, -1.0, -1.0, 0.0].into_iter().enumerate() {
            log.altitude_m[index] = feet * foot;
        }
        let read = read(&log);
        assert_eq!(read.pad_altitude_m, Some(0.0));
        assert!((read.liftoff.value().unwrap().time_s - 3.3).abs() < 1e-9);
    }

    /// Noise on the pad up to the allowance doesn't move liftoff earlier: at half a foot, the
    /// rule before M7.2b, it read at the pad's last low sample, two seconds early.
    #[test]
    fn pad_noise_within_the_allowance_leaves_liftoff() {
        let mut noisy = log(40.0, |t| flight(10.0)(t - 5.0));
        // The pad from 0 to 6 s, its last two seconds 1.5 m up, below the 2 m allowance.
        for (time, height) in noisy.time_s.iter().zip(noisy.altitude_m.iter_mut()) {
            if (4.0..=6.0).contains(time) {
                *height = 1.5;
            }
        }
        let readings = read(&noisy);
        let liftoff = readings.liftoff.value().unwrap();
        assert!((liftoff.time_s - 6.3).abs() < 1e-9, "{}", liftoff.time_s);
        assert!(liftoff.time_s - 6.0 <= liftoff.bound_s);
    }

    /// A vacuum hop from `t0` to `apogee_m`, falling back at `fall_g` times gravity, its heights
    /// rounded to whole feet as a PerfectFlite writes them; and when it is back on the pad.
    fn hop(apogee_m: f64, t0: f64, fall_g: f64) -> (impl Fn(f64) -> f64, f64) {
        let g = STANDARD_GRAVITY_MPS2;
        let foot = crate::perfectflite::FOOT_M;
        let v0 = (2.0 * g * apogee_m).sqrt();
        let top = t0 + v0 / g;
        let fall = fall_g * g;
        let down = top + (2.0 * apogee_m / fall).sqrt();
        let flight = move |t: f64| {
            let h = if t <= t0 {
                0.0
            } else if t <= top {
                v0 * (t - t0) - 0.5 * g * (t - t0).powi(2)
            } else if t < down {
                apogee_m - 0.5 * fall * (t - top).powi(2)
            } else {
                0.0
            };
            (h / foot).round() * foot
        };
        (flight, down)
    }

    /// The free-fall check passes a fall in vacuum wherever it falls between samples, with the
    /// heights rounded to feet, at 10 to 100 samples a second, and refuses one at 1.3 g. A low hop,
    /// just past the climb's mark of twice the allowance and the step, passes it and is then withheld,
    /// as it falls too little past its step and the noise for the descent's rate to be measured;
    /// the higher ones land. One below the mark never climbs.
    #[test]
    fn a_vacuum_fall_lands_and_a_faster_one_is_refused() {
        let foot = crate::perfectflite::FOOT_M;
        // Low hops, their apogees a fiftieth of a foot apart, where the rounding matters most,
        // just past the 4.6 m mark; and higher ones.
        let mark = 2.0 * (LIFTOFF_ALLOWANCE_M + foot);
        assert!(16.0 * foot > mark && 15.0 * foot < mark);
        let apogees: Vec<f64> = (0..50)
            .map(|k| (16.0 + f64::from(k) / 50.0) * foot)
            .chain([30.0, 300.0, 3000.0])
            .collect();
        for dt in [0.01, 0.05, 0.1] {
            for &apogee_m in &apogees {
                for step in 0..13 {
                    let t0 = 1.0 + f64::from(step) * dt / 13.0;
                    let (flight, down) = hop(apogee_m, t0, 1.0);
                    let read = read(&log_every(dt, down + 3.0, flight));
                    let low = apogee_m < 30.0;
                    let lands = match &read.landing {
                        Reading::Read(_) => !low,
                        Reading::Withheld(withheld) => {
                            low && withheld.reason == Reason::NoClimb
                                && withheld.detail.contains("to time the descent")
                        }
                    };
                    assert!(
                        lands,
                        "{dt} s, {apogee_m} m, from {t0} s: {:?}",
                        read.landing
                    );
                }
            }
        }
        let (below, down) = hop(15.0 * foot, 1.0, 1.0);
        assert_eq!(
            read(&log(down + 3.0, below)).apogee.reason(),
            Some(Reason::NoClimb)
        );
        for apogee_m in [100.0, 1000.0] {
            let (flight, down) = hop(apogee_m, 1.0, 1.3);
            let read = read(&log(down + 3.0, flight));
            assert_eq!(
                read.landing.reason(),
                Some(Reason::FasterThanFreeFall),
                "{apogee_m} m"
            );
        }
    }

    /// A log that starts 0.1 s before a fast liftoff, as a logger that records from launch writes
    /// its pad: its samples can't rule out that the rocket had climbed 52 m before the log, so the
    /// pad is the logger's zero, and liftoff is read at the last sample within its allowance.
    #[test]
    fn a_short_pad_is_the_loggers_zero() {
        let (flight, down) = hop(300.0, 0.1, 1.0);
        let read = read(&log(down + 3.0, flight));
        assert_eq!(read.pad_altitude_m, Some(0.0));
        let liftoff = read.liftoff.value().unwrap();
        assert!((liftoff.time_s - 0.1).abs() < 1e-9, "{}", liftoff.time_s);
        let apogee = read.apogee.value().unwrap();
        assert!(apogee.time_after_liftoff_s.is_some());
    }

    /// The pad is short where its samples can't rule out a rocket already past the allowance: a
    /// PerfectFlite's track 1 m off its zero, on the pad for `pad_s`, then climbing at 10 g for a
    /// second and coasting at gravity, sampled at 1,000 Hz, stays within half the 4.61 m rise of its start until 0.217 s after
    /// liftoff. A rocket at 2 g moving at `v` at the start would climb the rise in that time unless
    /// `v ≤ (rise − g stay²)/stay`, so it may have climbed `v²/(4g)`: 1.70 m after a 0.17 s pad,
    /// within the 2 m allowance, so the pad is its samples' and the apogee's bound grows by that;
    /// 2.49 m after 0.13 s, past it, so the pad is the zero. The climb's first samples lend the
    /// median 26 mm, within the eighth of the rise a constant acceleration can lend it.
    #[test]
    fn the_pad_is_short_where_the_rocket_may_have_left_it() {
        let g = STANDARD_GRAVITY_MPS2;
        let climb = |pad_s: f64| {
            move |t: f64| {
                let c = (t - pad_s).clamp(0.0, 1.0);
                let coast = (t - pad_s - 1.0).max(0.0);
                1.0 + 5.0 * g * c * c + 10.0 * g * coast - 0.5 * g * coast * coast
            }
        };
        let after = read(&log_every(0.001, 14.0, climb(0.17)));
        let step = after.step_m.unwrap();
        let rise = 2.0 * (LIFTOFF_ALLOWANCE_M + step);
        let lent = after.pad_altitude_m.unwrap() - 1.0;
        assert!(lent > 0.0 && lent <= rise / 8.0, "{lent}");
        let before = read(&log_every(0.001, 14.0, climb(0.13)));
        assert_eq!(before.pad_altitude_m, Some(0.0));

        // The bound's share, against the same flight after a 2 s pad, whose samples rule it out;
        // the lent pad shortens the height, and its share of the bound, by under 3 mm.
        let long = read(&log_every(0.001, 16.0, climb(2.0)));
        // The stay ends at the last sample within half the rise: the median of a rising track
        // is its sample.
        let stay = (0..)
            .map(|k| f64::from(k) * 0.001)
            .take_while(|t| climb(0.17)(*t) <= 1.0 + 0.5 * rise)
            .last()
            .unwrap();
        let speed = (rise - g * stay * stay) / stay;
        let pad_error = speed * speed / (4.0 * g);
        assert!((pad_error - 1.70).abs() < 0.01, "{pad_error}");
        let bound = |read: &Readings| read.apogee.value().unwrap().bound_m;
        let drift = DRIFT_M_PER_S * (0.17 - 2.0);
        assert!(
            (bound(&after) - bound(&long) - (pad_error + drift)).abs() < 3e-3,
            "{} against {}",
            bound(&after) - bound(&long),
            pad_error + drift
        );
    }

    /// A noisy pad sampled slowly is still a pad: its first samples wobble by more than a meter,
    /// which the pad's start doesn't take for a climb, as a 1 m rise from the first sample did.
    #[test]
    fn a_noisy_slow_pad_is_still_a_pad() {
        let wobble = [
            -0.19, -2.42, -0.06, -1.71, -0.9, -1.3, -0.4, -2.0, -1.1, -0.7,
        ];
        let mut noisy = log_every(0.2, 40.0, |t| flight(10.0)(t - 2.0));
        for (height, noise) in noisy.altitude_m.iter_mut().zip(wobble.iter().cycle()) {
            *height += noise;
        }
        let read = read(&noisy);
        let liftoff = read.liftoff.value().unwrap();
        assert!(
            (liftoff.time_s - 3.0).abs() <= liftoff.bound_s,
            "{} within {}",
            liftoff.time_s,
            liftoff.bound_s
        );
        let pad = read.pad_altitude_m.unwrap();
        assert!((-2.42..=-0.06).contains(&pad), "{pad}");
    }

    /// #646, both ways: a mapped altitude above sea level is refused as starting off the pad,
    /// and a millisecond clock read as seconds as sampled too slowly, or, from a logger fast
    /// enough to pass that, as slower than a coast; the same flights in their right units read.
    #[test]
    fn a_track_in_the_wrong_unit_is_refused() {
        let right = log(30.0, flight(10.0));
        assert!(read(&right).apogee.value().is_some());
        let mut msl = right.clone();
        msl.format = LogFormat::GenericTable;
        for height in &mut msl.altitude_m {
            *height += 1188.7;
        }
        let readings = read(&msl);
        assert_eq!(readings.apogee.reason(), Some(Reason::StartsOffThePad));
        let detail = match &readings.apogee {
            Reading::Withheld(withheld) => withheld.detail.clone(),
            Reading::Read(_) => String::new(),
        };
        assert!(detail.contains("above sea level"), "{detail}");
        // At the edge of a zero's hour of weather: just inside it reads, from the pad; just
        // outside, refused. A reader's altitude is above the pad by its format, so the same offset
        // in a PerfectFlite's log reads.
        let offset = |format: LogFormat, by: f64| {
            let mut moved = right.clone();
            moved.format = format;
            for height in &mut moved.altitude_m {
                *height += by;
            }
            read(&moved)
        };
        let inside = offset(LogFormat::GenericTable, PAD_ZERO_M - 0.01);
        assert_eq!(inside.pad_altitude_m, Some(PAD_ZERO_M - 0.01));
        assert!(inside.apogee.value().is_some());
        let outside = offset(LogFormat::GenericTable, PAD_ZERO_M + 0.01);
        assert_eq!(outside.apogee.reason(), Some(Reason::StartsOffThePad));
        assert_eq!(outside.liftoff.reason(), Some(Reason::StartsOffThePad));
        let reader = offset(LogFormat::PerfectFlitePf2, PAD_ZERO_M + 0.01);
        assert!(reader.apogee.value().is_some());
        assert!(reader.liftoff.value().is_some());
        // A channel's column says the same: above the ground, refused; above sea level, read
        // from its pad.
        let agl = as_channel(&right, Quantity::AltitudeAboveGround, |h| h + 1188.7);
        assert_eq!(read(&agl).apogee.reason(), Some(Reason::StartsOffThePad));
        let msl = read(&as_channel(&right, Quantity::AltitudeMsl, |h| h + 1188.7));
        assert!((msl.pad_altitude_m.unwrap() - 1188.7).abs() < 1e-9);
        assert!(msl.apogee.value().is_some());

        // 20 Hz in milliseconds read as seconds: samples 50 s apart.
        let mut millis = right.clone();
        for time in &mut millis.time_s {
            *time *= 1000.0;
        }
        assert_eq!(
            read(&millis).apogee.reason(),
            Some(Reason::SampledTooSlowly)
        );
        // 2 kHz, so 0.5 s apart when misread: the climb takes 5,000 s, and loses nothing in the
        // last 4 s before its top.
        let fast = log_every(0.0005, 30.0, flight(10.0));
        assert!(read(&fast).apogee.value().is_some());
        let mut fast_millis = fast;
        for time in &mut fast_millis.time_s {
            *time *= 1000.0;
        }
        let readings = read(&fast_millis);
        assert_eq!(readings.apogee.reason(), Some(Reason::SlowerThanFreeCoast));
        assert_eq!(readings.liftoff.reason(), Some(Reason::SlowerThanFreeCoast));
    }

    /// A track whose step is coarser than the bounds were checked at is withheld, at the edge.
    #[test]
    fn a_coarse_step_is_withheld() {
        // The flight ten times as high and √10 times as long, so it coasts at gravity still.
        let stepped = |step: f64| {
            let tall = |t: f64| 10.0 * flight(10.0)(t / 10f64.sqrt());
            let mut log = log(60.0, |t| (tall(t) / step).round() * step);
            log.format = LogFormat::GenericTable;
            read(&log)
        };
        assert!(stepped(MAX_STEP_M).apogee.value().is_some());
        assert_eq!(
            stepped(MAX_STEP_M * 1.01).apogee.reason(),
            Some(Reason::StepTooCoarse)
        );
    }

    /// A clock too fine for the median's window is withheld whole, saying so, at the edge: 0.3 s
    /// is 1,000 samples either side at 0.15 ms, and 1,001 at 0.3/2002 s.
    #[test]
    fn a_clock_too_fine_for_the_window_is_withheld() {
        let with_interval = |interval: f64| {
            let mut log = log(30.0, flight(10.0));
            for (index, time) in log.time_s.iter_mut().enumerate() {
                *time = f64::from(u32::try_from(index).unwrap()) * interval;
            }
            read(&log)
        };
        for interval in [1e-30, 1e-6, MEDIAN_WINDOW_S / 2002.0] {
            let read = with_interval(interval);
            assert_eq!(
                read.apogee.reason(),
                Some(Reason::SampledTooFast),
                "{interval}"
            );
            assert_eq!(read.landing.reason(), Some(Reason::SampledTooFast));
            assert_eq!(read.sample_interval_s.map(|s| s > 0.0), Some(true));
        }
        // At 0.15 ms the window is exactly 1,000 samples either side, and the log is read.
        let read = with_interval(0.000_15);
        assert_ne!(read.apogee.reason(), Some(Reason::SampledTooFast));
        let window_s = read.median_window_s.unwrap();
        assert!((window_s - 0.3).abs() < 1e-12, "{window_s}");
    }

    /// The window's half-width at a rounding tie doesn't turn on the interval's last bit.
    #[test]
    fn the_window_settles_ties_upwards() {
        assert_eq!(half_window(0.05), 3);
        for interval in [0.1, 0.1 + 1e-15, 0.1 - 1e-15, 0.2 - 0.1] {
            assert_eq!(half_window(interval), 2, "{interval}");
        }
        assert_eq!(half_window(1.0), 1);
        assert!((peak_bound_m(3, 0.05) - 0.076_614).abs() < 1e-6);
    }

    /// The constants worked out from others say so.
    #[test]
    fn derived_constants_follow_their_formulas() {
        let sound = (1.4 * 287.05 * (216.65 - AIR_TEMPERATURE_SPREAD_K)).sqrt();
        assert!((TRANSONIC_SPEED_M_S - TRANSONIC_MACH * sound).abs() < 1e-9);
        assert!((TEMPERATURE_SHARE - 0.069_236).abs() < 1e-6);
    }

    proptest::proptest! {
        /// The median's peak bound holds wherever the true peak falls between samples, at any
        /// half-width, and the median never reads above the peak.
        #[test]
        fn the_peak_bound_holds(phase in 0.0..1.0_f64, half in 1_usize..7, dt in 0.01..0.2_f64) {
            let trace: Vec<f64> = (-60..=60)
                .map(|i| {
                    let t = (f64::from(i) - phase) * dt;
                    -0.5 * STANDARD_GRAVITY_MPS2 * t * t
                })
                .collect();
            let top = running_median(&trace, half)
                .into_iter()
                .fold(f64::NEG_INFINITY, f64::max);
            proptest::prop_assert!(top <= 0.0);
            proptest::prop_assert!(-top <= peak_bound_m(half, dt) * (1.0 + 1e-12));
        }
    }

    /// A coast doesn't fall and rise again, and nothing falls from its top faster than gravity
    /// pulls it: a transient that stands above the coast's top, as a charge's does, holds the
    /// apogee, the top speed and landing back, by name; liftoff is read. A step 20 m above the
    /// top, 0.6 s wide, 0.3 s after apogee, before the track has fallen past twice its noise and
    /// step (6.6 m), falls 29.0 m from its top in a sample, where a body at rest there falls at
    /// most 20.9 m past the median's window and the noise; a climb at 12 m/s from 10 m below the
    /// top to 26 m above it, then down at 10 m/s, falls no faster than that, but follows a fall of
    /// 8.9 m. Each is caught by its own rule, and the same flight with neither reads its apogee at
    /// the coast's top.
    #[test]
    fn a_transient_top_is_withheld() {
        let g = STANDARD_GRAVITY_MPS2;
        let plain = read(&log(30.0, flight(10.0)));
        let apogee = plain.apogee.value().unwrap();
        assert!((apogee.time_s - 6.0).abs() <= apogee.time_bound_s);

        fn by<T: std::fmt::Debug>(reading: &Reading<T>, rule: &str) {
            let Reading::Withheld(withheld) = reading else {
                panic!("{reading:?}");
            };
            assert_eq!(withheld.reason, Reason::DisturbedTop);
            assert!(withheld.detail.contains(rule), "{}", withheld.detail);
        }
        let withheld = |read: &Readings, rule: &str| {
            assert!(read.liftoff.value().is_some());
            by(&read.apogee, rule);
            by(&read.max_speed, rule);
            by(&read.landing, rule);
        };
        let stepped = |t: f64| {
            if (6.3..6.9).contains(&t) {
                10.0 * g + 20.0
            } else {
                flight(10.0)(t)
            }
        };
        withheld(&read(&log(30.0, stepped)), "a body at rest there can fall");
        let returning = |t: f64| {
            if t < 7.0 {
                flight(10.0)(t)
            } else if t < 10.0 {
                10.0 * g - 10.0 + 12.0 * (t - 7.0)
            } else {
                (10.0 * g + 26.0 - 10.0 * (t - 10.0)).max(0.0)
            }
        };
        withheld(&read(&log(30.0, returning)), "then climbs to its top");
    }

    /// A top that is the vehicle's is read, whichever half of a spike follows it. A parachute open
    /// before apogee slows the climb into the top, not the fall from it: a coast that opens its
    /// chute at 40 m/s and stops at 40 m/s² climbs 20 m in its last second, more than a coast
    /// slowing at twice gravity could, then falls at gravity to 8 m/s. A charge that lifts the
    /// bay's pressure at the top drops the track after the coast's own climb: 50 m for half a
    /// second from 0.4 s after the top.
    #[test]
    fn a_top_that_isnt_a_spike_is_read() {
        let g = STANDARD_GRAVITY_MPS2;
        // Off the pad at 1 s at 40 m/s² for 3 s (180 m, 120 m/s), a vacuum coast to 40 m/s, then
        // 40 m/s² to the top 20 m higher, and down at gravity to 8 m/s.
        let coast_s = 80.0 / g;
        let opened_m = 180.0 + 120.0 * coast_s - 0.5 * g * coast_s * coast_s;
        let (top_s, top_m) = (4.0 + coast_s + 1.0, opened_m + 20.0);
        let chuted = move |t: f64| {
            if t <= 1.0 {
                0.0
            } else if t < 4.0 {
                20.0 * (t - 1.0).powi(2)
            } else if t < 4.0 + coast_s {
                let c = t - 4.0;
                180.0 + 120.0 * c - 0.5 * g * c * c
            } else if t < top_s {
                let c = top_s - t;
                top_m - 20.0 * c * c
            } else {
                let d = t - top_s;
                let speeding = (8.0 / g).min(d);
                (top_m - 0.5 * g * speeding * speeding - 8.0 * (d - speeding)).max(0.0)
            }
        };
        let chute = read(&log(140.0, chuted));
        let apogee = chute.apogee.value().unwrap();
        assert!(
            (apogee.height_m - top_m).abs() <= apogee.bound_m,
            "{apogee:?}"
        );
        assert!(
            (apogee.time_s - top_s).abs() <= apogee.time_bound_s,
            "{apogee:?}"
        );
        assert!(chute.landing.value().is_some(), "{:?}", chute.landing);

        let charged = read(&log(30.0, |t| {
            let dropped = if (6.4..6.9).contains(&t) { 50.0 } else { 0.0 };
            flight(10.0)(t) - dropped
        }));
        let apogee = charged.apogee.value().unwrap();
        assert!(
            (apogee.height_m - 10.0 * g).abs() <= apogee.bound_m,
            "{apogee:?}"
        );
        assert!(
            (apogee.time_s - 6.0).abs() <= apogee.time_bound_s,
            "{apogee:?}"
        );
    }

    /// A charge's transient, in the shapes of two public logs whose licenses keep them out of
    /// the repository, with invented numbers: a telemetry logger that sends a sample a second on
    /// the pad and after its charge and ten a second between, its charge at the top of a 914 m
    /// coast, then one sample 130 m under the top and the next 150 m over it, reads the coast's
    /// top, the spike taken out by the median; a breakup that reads a climb to 40% over the
    /// coast's top within a second, mid-coast, and back in half a second, is withheld by name: it
    /// falls 50.0 m in a sample from its top, where a body at rest there falls at most 13.6 m.
    #[test]
    fn a_charges_transient_reads_the_coast_or_is_withheld() {
        let g = STANDARD_GRAVITY_MPS2;
        // Off the pad at 300 s at 40 m/s² for 3 s (180 m, 120 m/s), a vacuum coast to the top
        // at 315.23 s (913.9 m), then down at 30 m/s.
        let (top_s, top_m) = (303.0 + 120.0 / g, 180.0 + 120.0 * 120.0 / (2.0 * g));
        let flight = move |t: f64| {
            if t <= 300.0 {
                0.0
            } else if t < 303.0 {
                20.0 * (t - 300.0).powi(2)
            } else if t < top_s {
                let c = t - 303.0;
                180.0 + 120.0 * c - 0.5 * g * c * c
            } else {
                (top_m - 30.0 * (t - top_s)).max(0.0)
            }
        };
        let made = |time_s: Vec<f64>, height: &dyn Fn(usize, f64) -> f64| {
            let mut made = log_every(1.0, 0.0, |_| 0.0);
            made.altitude_m = time_s
                .iter()
                .enumerate()
                .map(|(i, t)| height(i, *t))
                .collect();
            made.time_s = time_s;
            made
        };

        let mut time_s: Vec<f64> = (0..=300).map(f64::from).collect();
        time_s.extend((1..=153).map(|k| 300.0 + 0.1 * f64::from(k)));
        let charge = time_s.len();
        time_s.extend((1..=37).map(|k| 315.3 + f64::from(k)));
        let telemetry = read(&made(time_s, &|i, t| match i.checked_sub(charge) {
            Some(0) => top_m - 130.0,
            Some(1) => top_m + 150.0,
            _ => flight(t),
        }));
        let apogee = telemetry.apogee.value().unwrap();
        assert!(
            (apogee.height_m - top_m).abs() <= apogee.bound_m,
            "{apogee:?}"
        );
        assert!(
            (apogee.time_s - top_s).abs() <= apogee.time_bound_s,
            "{apogee:?}"
        );

        let time_s: Vec<f64> = (0..=1200).map(|k| 295.0 + 0.05 * f64::from(k)).collect();
        let breakup = read(&made(time_s, &|_, t| {
            let over = 1.4 * top_m - flight(308.0);
            let lift = if (308.0..309.0).contains(&t) {
                over * (t - 308.0)
            } else if (309.0..309.5).contains(&t) {
                over * (1.0 - 2.0 * (t - 309.0))
            } else {
                0.0
            };
            flight(t) + lift
        }));
        assert!(breakup.liftoff.value().is_some());
        for reason in [
            breakup.apogee.reason(),
            breakup.max_speed.reason(),
            breakup.landing.reason(),
        ] {
            assert_eq!(reason, Some(Reason::DisturbedTop));
        }
    }

    /// The top speed's guards, each on its own.
    #[test]
    fn a_top_speed_is_refused_by_name() {
        let g = STANDARD_GRAVITY_MPS2;
        // A climb at `a` m/s² for 3 s from 1 s, then a vacuum coast and a 30 m/s descent.
        let boost = |a: f64| {
            move |t: f64| {
                let v = a * 3.0;
                let coast = v / g;
                let top = 4.5 * a + 0.5 * v * coast;
                if t <= 1.0 {
                    0.0
                } else if t < 4.0 {
                    0.5 * a * (t - 1.0).powi(2)
                } else if t < 4.0 + coast {
                    let c = t - 4.0;
                    4.5 * a + v * c - 0.5 * g * c * c
                } else {
                    (top - 30.0 * (t - 4.0 - coast)).max(0.0)
                }
            }
        };
        // 3 s at 70 m/s² is 210 m/s, under the transonic mark; at 80 m/s², 240 m/s, over it.
        let below = read(&log(300.0, boost(70.0)));
        let speed = below.max_speed.value().unwrap();
        assert!((speed.speed_m_s - 210.0).abs() <= speed.bound_m_s);
        assert!(speed.speed_m_s < TRANSONIC_SPEED_M_S);
        let over = read(&log(300.0, boost(80.0)));
        assert_eq!(over.max_speed.reason(), Some(Reason::Transonic));
        assert!(over.apogee.value().is_some());
        // 1,400 m/s² for 3 s: implausible, refused ahead of the transonic mark.
        let absurd = read(&log(2000.0, boost(1400.0)));
        assert_eq!(absurd.max_speed.reason(), Some(Reason::ImplausibleSpeed));
        // A 60 m dip in the coast 4.4 s before the top, down and back at 200 m/s, where the
        // coast climbs at 43 m/s: the climb's speed swings past −42 m/s, 20% of its top.
        let dipped = read(&log(300.0, |t| {
            let dip = if (21.0..21.3).contains(&t) {
                200.0 * (t - 21.0)
            } else if (21.3..21.6).contains(&t) {
                200.0 * (21.6 - t)
            } else {
                0.0
            };
            boost(70.0)(t) - dip
        }));
        assert_eq!(dipped.max_speed.reason(), Some(Reason::NoisySpeed));
        // Logged at 10 Hz the 0.5 s fit has 5 or 6 samples; at 8 Hz 5; at 7.7 Hz, too few.
        let slow = |dt: f64| read(&log_every(dt, 30.0, flight(10.0))).max_speed;
        assert!(slow(0.1).value().is_some());
        assert!(slow(0.125).value().is_some());
        assert_eq!(slow(0.13).reason(), Some(Reason::SampledTooSlowly));
    }

    /// `ALL` is built from the enum's own list, so it can't miss a variant; each is listed once,
    /// with its own code.
    #[test]
    fn all_lists_every_variant_once() {
        fn codes<T: Serialize>(all: &[T]) -> (usize, usize) {
            let mut codes: Vec<String> = all
                .iter()
                .map(|value| serde_json::json!(value).to_string())
                .collect();
            let listed = codes.len();
            codes.sort();
            codes.dedup();
            (listed, codes.len())
        }
        assert_eq!(codes(Reason::ALL), (23, 23));
        assert_eq!(codes(Source::ALL), (4, 4));
        assert_eq!(codes(Provenance::ALL), (3, 3));
        assert_eq!(codes(Method::ALL), (4, 4));
    }

    /// Serialized, a reading carries its status beside its fields.
    #[test]
    fn a_reading_serializes_with_its_status() {
        let read = read(&log(30.0, flight(10.0)));
        let json = serde_json::to_value(&read).unwrap();
        assert_eq!(json["apogee"]["status"], "read");
        assert_eq!(json["apogee"]["source"], "barometer");
        assert_eq!(json["apogee"]["provenance"], "measured");
        assert_eq!(json["max_speed"]["provenance"], "derived");
        assert_eq!(json["max_acceleration"]["status"], "withheld");
        assert_eq!(json["max_acceleration"]["reason"], "no_accelerometer");
        let back: Readings = serde_json::from_value(json).unwrap();
        assert_eq!(back, read);
    }
}
