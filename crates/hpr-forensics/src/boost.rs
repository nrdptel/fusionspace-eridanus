//! Apogee predicted from the boost: a log cut after burnout, fitted, and flown on to apogee
//! ([ADR-257][adr-257] §3; [ADR-321][adr-321], amended 2026-11-06).
//!
//! The cut must not read the flight after it. A test replaces every later value and time with
//! noise and the prediction stays bit-identical. Two rules hold that by construction:
//!
//! - [`find_cut`] walks the height track forward and decides at each sample from that sample and
//!   the ones before it. A vertical speed is fitted by least squares over the last
//!   [`SPEED_WINDOW_S`] of samples, leaving out a sample the analyzer's Hampel rule calls bad: one
//!   bad reading of 12 to 100 m put in set A's flights moved a cut by 0.70 s at most (measured).
//!   Liftoff is where that line stands [`LIFTOFF_RISE_M`] above the pad, the line where it last
//!   stood still ([`STILL_M_S`]), or before that the first window's. The cut is at
//!   the first sample where the speed has stood below its top by [`DROP_SHARE`] of it, and by at
//!   least [`DROP_FLOOR_M_S`], for [`HOLD_S`], and is the lowest it has been over the last
//!   [`HOLD_S`]. After burnout gravity takes about 9.8 m/s off the vertical speed every second,
//!   drag more, so the speed keeps falling and the log keeps the coast's start. A barometer's
//!   reading can dip in the boost and catch up: on the public logs the falls that dips gave
//!   recovered within the hold (measured once), and on the synthetic dips a fall that can outlast
//!   it (likely the fall back from a catch-up's overshoot, not traced) had begun to rise again
//!   while the motor burned.
//! - Everything after that reads the log cut there: its readings, the fit's observations and so
//!   the fit. The log's own clock is cut at the cut's sample, by position, so a time it logs later
//!   can't enter. A channel on a clock of its own is cut at its first time past the cut or out of
//!   order: a sample it stamps in order at or before the cut is read wherever the file puts it,
//!   as nothing else says when it was logged.
//!
//! The finder refuses the tracks the analyzer distrusts in the boost: a GNSS receiver's, and a
//! barometer's once its speed reaches [`readings::TRANSONIC_SPEED_M_S`] (Mach 0.8 in the coldest
//! air), where the port may read the transonic flow. Without that rule, and with an earlier rule's 1 s
//! hold in the cut, the fitted top speed was over 1.1 times the true one on 57 of set A's first 200
//! synthetic flights, the barometer's jump near Mach 1 read as the top, so the cut's place followed
//! the jump rather than burnout (a sweep run once). So a flight whose fitted climb speed reaches
//! 227.7 m/s gets no prediction from its barometer: on that set the slowest refused had a true top
//! of 221 m/s and the fastest cut a true 233 m/s, its fitted speed under the limit (measured
//! once). An accelerometer's cut isn't built.
//!
//! A motor whose thrust falls below the rocket's weight and drag well before it burns out, as a
//! long tail-off does, slows the climb by the fall the cut needs while still burning, and is cut
//! before its burnout; the cut is checked only on the synthetic flights' motors.
//!
//! The analyzer's own burnout reading isn't used: its spike filter, its burnout mean and its
//! noise span are centered windows, and [`readings::read`] takes the whole log at once.
//!
//! How far to trust it: the cut and the fit are tested for not reading past the cut; how close
//! the predicted apogee comes to a logged one is [not measured yet][plan], on real flights or
//! synthetic ones.
//!
//! [plan]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3d
//!
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-321]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md

use std::borrow::Cow;
use std::collections::VecDeque;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings;
use serde::{Deserialize, Serialize};

use crate::fit::{self, Fit, Forward, Noise, Observations, Stated};
use crate::residuals::HeightKind;

/// How far the window's fitted line must stand above the pad's for liftoff, m: past the
/// largest excursion of a raw height from the track's first on the pad, 6.8 m over sets A and B's
/// first 200 synthetic flights each (their barometers' logs, measured once), while a hobby rocket
/// climbs it in about a second or less. HPR Sim's choice.
pub const LIFTOFF_RISE_M: f64 = 10.0;

/// The fitted speed under which the line stands still, m/s, either way: where it does, the pad is
/// re-read there, and a liftoff, a top speed and a fall read before it are forgotten. A run of bad
/// samples on the pad or a step there that stays can lift the line past [`LIFTOFF_RISE_M`] and
/// read a speed faster than the flight's own top; the line stands still again once they leave the
/// window, or at the step's new level ([#1362][i1362]). On sets A and B's first 200 synthetic
/// flights' barometer logs with a cut (measured once, not committed), 98.8% and 98.4% of the pad's
/// windows read under it, the longest stretch of the pad that never did lasted 0.25 s (leaving
/// out a 500 Hz logger's first windows at its start), and no speed from liftoff to the cut read
/// under 17.7 m/s, the climb's speed 10 m up. A flight cut by the [`DROP_FLOOR_M_S`] fall held for
/// [`HOLD_S`] reaches it first only if its top speed is under about 22 m/s: it, the fall and 0.75 s
/// of gravity, 7.4 m/s. HPR Sim's choice.
///
/// [i1362]: https://github.com/nrdptel/fusionspace-eridanus/issues/1362
pub const STILL_M_S: f64 = 5.0;

/// The span of the trailing window the vertical speed is fitted over, s: a straight line by
/// least squares through the samples in it. HPR Sim's choice: long enough to average a
/// barometer's noise over about 10 samples at 20 Hz, short against a boost of a few seconds.
pub const SPEED_WINDOW_S: f64 = 0.5;

/// The share of its top the fitted speed must fall below that top for the cut. HPR Sim's choice:
/// 10%, a margin over the window's noise on a fast flight's speed, as a barometer's noise in height
/// grows with altitude. Unmeasured: on set A's first 200 synthetic flights the cut's test passes on
/// [`DROP_FLOOR_M_S`] alone.
pub const DROP_SHARE: f64 = 0.1;

/// The least fall below the top speed for the cut, m/s: about a second of gravity alone, so a
/// slow flight's noise doesn't cut it during the boost. HPR Sim's choice.
pub const DROP_FLOOR_M_S: f64 = 10.0;

/// How long the fall must hold before the cut, s, and the span its speed must be the lowest of:
/// a barometer's fitted speed can fall by [`DROP_FLOOR_M_S`] and more during the boost and
/// recover. On the public flights' logs, the longest such fall inside a burn lasted 0.48 s from
/// its first sample to the one it recovered at, and the deepest went 63.2 m/s below its top (Lince
/// and Juno III, measured once in a run not committed); after burnout gravity and drag keep the speed falling. HPR Sim's
/// choice: about half again the longest, 0.75 s. On set A's first 200 synthetic flights the cuts
/// came 1.08 s to 2.30 s after burnout, 0.33 s to 1.48 s before this rule.
pub const HOLD_S: f64 = 0.75;

/// How long the fitted flight is flown to find its apogee, s: past any hobby rocket's climb.
pub const APOGEE_HORIZON_S: f64 = 300.0;

/// Where the boost's log is cut.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cut {
    /// The index of the last sample the finder read, the cut's: in the log's own clock's samples,
    /// or, on a log of channels alone, in its track's channel up to its first time out of order.
    pub index: usize,
    /// That sample's time on the log's clock, s. The log is kept up to it, inclusive.
    pub time_s: f64,
    /// The end of the window whose fitted speed was the top, s.
    pub top_speed_s: f64,
    /// That top speed, m/s.
    pub top_speed_m_s: f64,
}

/// The slope and intercept of a straight line fitted by least squares through `points`, on times
/// taken from the first point's; `None` where its times don't spread.
fn line(points: &[(f64, f64)]) -> Option<(f64, f64)> {
    let t0 = points.first()?.0;
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a second's samples, far below 2^53"
    )]
    let n = points.len() as f64;
    let (st, sh) = points
        .iter()
        .fold((0.0, 0.0), |(st, sh), &(t, h)| (st + (t - t0), sh + h));
    let (mt, mh) = (st / n, sh / n);
    let (num, den) = points.iter().fold((0.0, 0.0), |(num, den), &(t, h)| {
        let dt = t - t0 - mt;
        (num + dt * (h - mh), den + dt * dt)
    });
    let slope = (den > 0.0).then(|| num / den)?;
    let intercept = mh - slope * mt;
    (slope.is_finite() && intercept.is_finite()).then_some((slope, intercept))
}

/// The vertical speed over `window`, m/s, and the height the line gives at its last time, m: a
/// straight line fitted by least squares without each sample whose residual from a first, robust
/// line (the median of the slopes between samples half the window's samples apart) lies further
/// than [`readings::SPIKE_THRESHOLD`] robust standard deviations (1.4826 times the residuals'
/// median absolute deviation) from their median, the analyzer's Hampel rule (Pearson et al.,
/// EUSIPCO 2015, §2). One bad sample that far out, a pulse at the port or a corrupt reading, is so
/// left out of every window that holds it; at 20 Hz three side by side can carry the first line
/// with them (the cut's test measures what that does), and at 8 to 10 Hz, with 5 or 6 samples to
/// a window, two may (untested). Where most residuals are 0, as on a quantized pad, only the samples on the line
/// are kept.
/// `None` where fewer than [`readings::SPEED_MIN_SAMPLES`] are kept or the times don't spread.
fn robust_line(window: &VecDeque<(f64, f64)>, scratch: &mut Scratch) -> Option<(f64, f64)> {
    let Scratch {
        points,
        values,
        residuals,
        kept,
    } = scratch;
    points.clear();
    points.extend(window.iter().copied());
    let t0 = points.first()?.0;
    // The first line, robust to one bad sample: the median of the slopes between samples half the
    // window apart, each sample in at most two of them, and the median of the heights' offsets.
    let apart = (points.len() / 2).max(1);
    values.clear();
    values.extend(
        points
            .iter()
            .zip(points.iter().skip(apart))
            .filter(|(a, b)| b.0 > a.0)
            .map(|(a, b)| (b.1 - a.1) / (b.0 - a.0)),
    );
    let slope = median_of(values);
    values.clear();
    values.extend(points.iter().map(|&(t, h)| h - slope * (t - t0)));
    let intercept = median_of(values);
    if !slope.is_finite() || !intercept.is_finite() {
        return None;
    }
    residuals.clear();
    residuals.extend(
        points
            .iter()
            .map(|&(t, h)| h - (intercept + slope * (t - t0))),
    );
    values.clear();
    values.extend(residuals.iter().copied());
    let center = median_of(values);
    values.clear();
    values.extend(residuals.iter().map(|r| (r - center).abs()));
    let spread = 1.4826 * median_of(values);
    let limit = readings::SPIKE_THRESHOLD * spread;
    kept.clear();
    kept.extend(
        points
            .iter()
            .zip(residuals.iter())
            .filter(|&(_, r)| (r - center).abs() <= limit)
            .map(|(&point, _)| point),
    );
    if kept.len() < readings::SPEED_MIN_SAMPLES {
        return None;
    }
    let (slope, intercept) = line(kept)?;
    let last_s = kept.last()?.0 - kept.first()?.0;
    Some((slope, intercept + slope * last_s))
}

/// The buffers [`robust_line`] fills at each sample, kept from one sample to the next so a walk
/// allocates them once rather than seven times a sample.
#[derive(Default)]
struct Scratch {
    points: Vec<(f64, f64)>,
    values: Vec<f64>,
    residuals: Vec<f64>,
    kept: Vec<(f64, f64)>,
}

/// `log` with each channel kept up to its first time that doesn't follow the one before it: a
/// channel-only log's track is refused whole when any of its times runs back, so a time logged
/// after the cut would decide whether there is one. The log's own clock is left whole; the finder
/// passes over its times out of order one by one.
fn in_order(log: &FlightLog) -> FlightLog {
    let mut ordered = log.clone();
    for channel in &mut ordered.channels {
        let kept = in_order_count(&channel.time_s, f64::INFINITY);
        channel.time_s.truncate(kept);
        channel.values.truncate(kept);
    }
    ordered
}

/// How many of `times` to keep: up to the last time that follows the one before it and isn't
/// past `cut_s`, before the first that is past it or doesn't follow. A time not a number between
/// two kept ones is kept where it stands, as a gap in the clock; one after the last kept isn't.
fn in_order_count(times: &[f64], cut_s: f64) -> usize {
    let mut last_s = f64::NEG_INFINITY;
    let mut kept = 0;
    for (index, &t) in times.iter().enumerate() {
        if t.is_nan() {
            continue;
        }
        if t <= last_s || t > cut_s {
            break;
        }
        last_s = t;
        kept = index + 1;
    }
    kept
}

/// Where the boost's log is cut, decided at each sample of the height track from it and the ones
/// before it (the module's rules); `None` where the log holds no height track, its track is a GNSS
/// receiver's, any [`SPEED_WINDOW_S`] of it holds more samples than the analyzer's fastest rate
/// allows ([`readings::MIN_INTERVAL_S`]), it never lifts off, it never holds
/// [`readings::SPEED_MIN_SAMPLES`] in a [`SPEED_WINDOW_S`] window (a log slower than 8 Hz, or one
/// whose samples can't be read), its
/// fitted speed reaches
/// [`readings::TRANSONIC_SPEED_M_S`] before the cut, or it never slows by [`DROP_SHARE`] of its
/// top speed and [`DROP_FLOOR_M_S`] for [`HOLD_S`], at a speed the lowest of the last [`HOLD_S`].
/// The cut stands where the window's line is more than [`LIFTOFF_RISE_M`] above the pad's:
/// three bad samples side by side on the pad can lift the line past that for a sample, and the
/// speed then falls back as it does at burnout. The pad is read again wherever the line stands
/// still ([`STILL_M_S`]), dropping the top speed and fall read before, so a pad spike faster than
/// the flight's own top, or a step up on the pad that stays, doesn't stay the top
/// ([#1362][i1362]).
///
/// A GNSS receiver's altitude in the boost can drop out at ignition and burnout and lag the
/// climb, by up to 440 m in one flight a flyer reports, while its fix reads valid
/// ([GNSS in boost][gnss]): on set A's first 60 synthetic flights such tracks were cut up to 11 s
/// after burnout or not at all (a sweep run once, with an earlier rule's 1 s hold in the cut). A staged
/// flight is cut at the first burnout whose coast lasts long enough for that fall; untested on
/// one.
///
/// [gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
/// [i1362]: https://github.com/nrdptel/fusionspace-eridanus/issues/1362
#[must_use]
pub fn find_cut(log: &FlightLog) -> Option<Cut> {
    try_cut(log).ok()
}

/// Why [`find_cut`] finds no cut in a log, in the order it decides.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NoCut {
    /// The log holds no height track.
    NoTrack,
    /// Its track is a GNSS receiver's.
    Gnss,
    /// A [`SPEED_WINDOW_S`] of it holds more samples than the analyzer's fastest rate allows.
    TooFast,
    /// No [`SPEED_WINDOW_S`] of it holds [`readings::SPEED_MIN_SAMPLES`] whose time and height are
    /// numbers, in order: a log slower than 8 Hz, or a track whose samples can't be read.
    TooSparse,
    /// Its fitted line never stands [`LIFTOFF_RISE_M`] above the pad's, where it last stood still
    /// ([`STILL_M_S`]), and stays off still until a cut.
    NeverLifted,
    /// Its fitted speed reached [`readings::TRANSONIC_SPEED_M_S`] before any cut.
    Transonic {
        /// The end of the window that reached it, s on the log's clock.
        time_s: f64,
        /// That window's fitted speed, m/s.
        speed_m_s: f64,
    },
    /// Its speed never fell by [`DROP_SHARE`] of its top and [`DROP_FLOOR_M_S`] for [`HOLD_S`], at
    /// a speed the lowest of the last [`HOLD_S`].
    NeverSlowed,
}

/// [`find_cut`]'s cut, or why there is none.
///
/// # Errors
///
/// [`NoCut`]: the first of the finder's refusals the log meets.
pub fn try_cut(log: &FlightLog) -> Result<Cut, NoCut> {
    walk(log, HOLD_S, &mut |_, _, _| {})
}

/// [`try_cut`]'s walk, with the fall held for `hold_s` before the cut, telling `fell` of each fall
/// that doesn't reach the cut: the time the fall first held, the first time it no longer did
/// (infinite for one still holding where the walk ends without a cut), and how far it went below
/// the top, m/s.
fn walk(log: &FlightLog, hold_s: f64, fell: &mut dyn FnMut(f64, f64, f64)) -> Result<Cut, NoCut> {
    // On a log with its own clock the track is that clock's: its channels are never read here.
    let ordered = if log.time_s.is_empty() {
        Cow::Owned(in_order(log))
    } else {
        Cow::Borrowed(log)
    };
    if readings::height_track_source(&ordered).ok_or(NoCut::NoTrack)? == readings::Source::Gnss {
        return Err(NoCut::Gnss);
    }
    let (times, heights) = readings::height_track(&ordered).ok_or(NoCut::NoTrack)?;
    let most = most_in(SPEED_WINDOW_S);
    let mut last_s = f64::NEG_INFINITY;
    let mut window: VecDeque<(f64, f64)> = VecDeque::new();
    let mut pad_m: Option<f64> = None;
    let mut lifted = false;
    let mut top: Option<(f64, f64)> = None;
    // Since when, and how deep, the speed has stood below its top by the fall the cut needs.
    let mut falling: Option<(f64, f64)> = None;
    // The speeds of the last `hold_s` since liftoff, to tell a fall still going from one settled.
    let mut recent: VecDeque<(f64, f64)> = VecDeque::new();
    let mut full = false;
    let mut scratch = Scratch::default();
    for (index, (&t, &h)) in times.iter().zip(&heights).enumerate() {
        // A sample out of order or not a number is passed over, as it is read.
        if !t.is_finite() || !h.is_finite() || t <= last_s {
            continue;
        }
        last_s = t;
        window.push_back((t, h));
        while window.front().is_some_and(|&(s, _)| s < t - SPEED_WINDOW_S) {
            window.pop_front();
        }
        // Faster than the analyzer reads a log: refused, which also bounds each sample's work.
        if window.len() > most {
            return Err(NoCut::TooFast);
        }
        if window.len() < readings::SPEED_MIN_SAMPLES {
            continue;
        }
        full = true;
        let Some((speed, level)) = robust_line(&window, &mut scratch) else {
            continue;
        };
        // The pad is where the line last stood still: bad samples on it or a step that stays lift
        // the line and read a speed, but it stands still again after them, before any flight's cut.
        if speed.abs() < STILL_M_S {
            pad_m = Some(level);
            if lifted {
                if let Some((since, deepest)) = falling.take() {
                    fell(since, t, deepest);
                }
                lifted = false;
                top = None;
                recent.clear();
            }
        }
        // Liftoff where the window's line, refitted without its outliers, stands high enough
        // above the pad: one bad sample on the pad doesn't lift it.
        let pad = *pad_m.get_or_insert(level);
        let aloft = level - pad > LIFTOFF_RISE_M;
        lifted = lifted || aloft;
        if !lifted {
            continue;
        }
        // The analyzer's transonic speed: a barometer's speed this fast may be the flow's.
        if speed >= readings::TRANSONIC_SPEED_M_S {
            if let Some((since, deepest)) = falling {
                fell(since, f64::INFINITY, deepest);
            }
            return Err(NoCut::Transonic {
                time_s: t,
                speed_m_s: speed,
            });
        }
        while recent.front().is_some_and(|&(s, _)| s < t - hold_s) {
            recent.pop_front();
        }
        recent.push_back((t, speed));
        // Whether this speed is the lowest of the last `hold_s`, the samples before this one; read
        // only once the fall has held, so a walk with no hold to end it (the tests' `fell` walks,
        // with an infinite hold and so every sample since liftoff kept) doesn't rescan the climb at
        // every sample.
        let lowest = |recent: &VecDeque<(f64, f64)>| {
            recent
                .iter()
                .take(recent.len() - 1)
                .all(|&(_, v)| speed <= v)
        };
        match top {
            Some((best, at)) if best - speed >= (DROP_SHARE * best).max(DROP_FLOOR_M_S) => {
                let (since, deepest) = falling.get_or_insert((t, 0.0));
                *deepest = deepest.max(best - speed);
                // Never on the pad: three bad samples side by side there can lift the line past
                // the rise for a sample, and the speed's fall back after it holds as burnout's does.
                if t - *since >= hold_s && lowest(&recent) && aloft {
                    return Ok(Cut {
                        index,
                        time_s: t,
                        top_speed_s: at,
                        top_speed_m_s: best,
                    });
                }
            }
            // A fall that doesn't hold: a dip in the reading, not burnout.
            Some((best, _)) if speed <= best => {
                if let Some((since, deepest)) = falling.take() {
                    fell(since, t, deepest);
                }
            }
            _ => {
                if let Some((since, deepest)) = falling.take() {
                    fell(since, t, deepest);
                }
                top = Some((speed, t));
            }
        }
    }
    if let Some((since, deepest)) = falling {
        fell(since, f64::INFINITY, deepest);
    }
    Err(match (full, lifted) {
        (false, _) => NoCut::TooSparse,
        (true, false) => NoCut::NeverLifted,
        (true, true) => NoCut::NeverSlowed,
    })
}

/// The most samples a trailing window of `span_s` holds on a log sampled no faster than the
/// analyzer reads ([`readings::MIN_INTERVAL_S`], its clock a part in a thousand fast).
fn most_in(span_s: f64) -> usize {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a small positive count: 0.5 s over 2 ms is 250"
    )]
    let most = (span_s / (readings::MIN_INTERVAL_S * 0.999)).floor() as usize;
    most + 1
}

/// The median of `values`, reordering them: the middle one, or the mean of the middle two, in
/// `f64::total_cmp`'s order. Selection finds the same middle values a full sort would put there.
fn median_of(values: &mut [f64]) -> f64 {
    let count = values.len();
    if count == 0 {
        return f64::NAN;
    }
    let (lower, &mut upper, _) = values.select_nth_unstable_by(count / 2, f64::total_cmp);
    if count % 2 == 1 {
        return upper;
    }
    // The largest of those below the middle is the sorted order's one before it.
    lower
        .iter()
        .copied()
        .max_by(f64::total_cmp)
        .map_or(upper, |below| (below + upper) / 2.0)
}

/// Whether a sample at `t` stays in a log cut at `cut_s`: at or before it, or with no time to
/// place it, as a gap in the clock is kept where it stands.
fn kept_at(t: f64, cut_s: f64) -> bool {
    t <= cut_s || t.is_nan()
}

/// `log` with only its samples up to `cut_s`: its own clock's columns and each channel's samples
/// up to their first time past `cut_s` or out of order, and its events at or before `cut_s` (the
/// readings and the fit read none). What it says of itself is kept.
#[must_use]
pub fn truncated(log: &FlightLog, cut_s: f64) -> FlightLog {
    truncated_at(log, in_order_count(&log.time_s, cut_s), cut_s)
}

/// `log` with the first `kept` samples of its own clock's columns, each channel's samples up to
/// its first past `cut_s` or out of order, and its events at or before `cut_s`. Both clocks are
/// cut by position, so a time logged later can't enter.
fn truncated_at(log: &FlightLog, kept: usize, cut_s: f64) -> FlightLog {
    let cut = |values: &Vec<f64>| values.iter().copied().take(kept).collect::<Vec<f64>>();
    let mut boost = log.clone();
    boost.time_s = cut(&log.time_s);
    boost.altitude_m = cut(&log.altitude_m);
    boost.vertical_speed_m_s = log.vertical_speed_m_s.as_ref().map(cut);
    boost.temperature_k = log.temperature_k.as_ref().map(cut);
    boost.battery_v = log.battery_v.as_ref().map(cut);
    for channel in &mut boost.channels {
        let kept = in_order_count(&channel.time_s, cut_s);
        channel.time_s.truncate(kept);
        channel.values.truncate(kept);
    }
    boost.events.retain(|event| kept_at(event.time_s, cut_s));
    boost
}

/// Apogee predicted from the boost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    /// Where the log was cut.
    pub cut: Cut,
    /// The fit of the log up to the cut.
    pub fit: Fit,
    /// The fitted flight's apogee, s after its start, on the simulation's clock: the fit's
    /// `clock_offset_s` places it on the log's.
    pub apogee_s: f64,
    /// The fitted flight's apogee, m above the pad: the height climbed, not what a barometer reads
    /// for it.
    pub apogee_above_pad_m: f64,
    /// The fitted flight's height at its apogee as the log's sensor reads it, m above the pad: a
    /// barometer's reading of the day's pressure there, or the height climbed for a sensor of
    /// height. What a log's own apogee is compared with.
    pub apogee_reading_m: f64,
}

/// Why a log gives no prediction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unpredicted {
    /// The finder found no cut, and why.
    NoCut(NoCut),
    /// The log up to the cut holds too little to fit: no pad or height track read, a window from
    /// liftoff to the cut that is empty or longer than 1,000 s, or no more bins of
    /// [`fit::BIN_S`] with a sample, from a second before liftoff, than the fit has unknowns (11).
    /// A log with no liftoff read stands its first sample for liftoff, so that alone doesn't
    /// refuse it.
    /// A log cut within 2 s of its liftoff can't hold 12, nor a later one logged from liftoff only.
    NothingToFit(Cut),
    /// The fit came back with what was stated: its residual more than [`fit::MISFIT_SHARE`] of the
    /// spread of the heights it read (`misfit_share`), or a flight it couldn't fly.
    NotFitted {
        /// The cut.
        cut: Cut,
        /// The fit's residual as a share of the heights' spread, where it came to one.
        misfit_share: Option<f64>,
    },
    /// The fitted flight reaches no apogee within [`APOGEE_HORIZON_S`], or can't be flown.
    NoApogee(Cut),
}

/// What the prediction reads of `log`, and the log it read: the log [`truncated`] at the cut
/// and `extra` samples of its track past it (none but in the test's probe).
fn read_to_cut(log: &FlightLog, extra: usize) -> Result<(Cut, Observations), Unpredicted> {
    let cut = try_cut(log).map_err(Unpredicted::NoCut)?;
    let nothing = Unpredicted::NothingToFit(cut);
    let to_s = match extra {
        0 => cut.time_s,
        _ => {
            let (times, _) = readings::height_track(&in_order(log)).ok_or(nothing)?;
            *times.get(cut.index + extra).ok_or(nothing)?
        }
    };
    // On the log's own clock, the track the finder read, the cut is by position.
    let boost = if log.time_s.is_empty() {
        truncated(log, to_s)
    } else {
        truncated_at(log, cut.index + extra + 1, to_s)
    };
    let read = readings::read(&boost);
    let kind = HeightKind::of(read.source.ok_or(nothing)?);
    // The probe reads its extra sample into the fit, past the whole bins the prediction keeps to.
    let observed =
        fit::observations_until(&boost, &read, to_s.next_up(), kind, extra == 0).ok_or(nothing)?;
    Ok((cut, observed))
}

/// Apogee predicted from `log` cut after burnout: the boost fitted by [`fit::fit`], each
/// parameter held to what `stated` gives for the boost's observations, then flown on by `forward`
/// to its apogee. The fit's own `converged` says whether it settled.
///
/// # Errors
///
/// [`Unpredicted`]: why there is no prediction, where the log has no cut, the boost says nothing
/// to fit, the fit keeps to what was stated, or the fitted flight doesn't reach an apogee within
/// [`APOGEE_HORIZON_S`].
pub fn predict(
    log: &FlightLog,
    stated: impl FnOnce(&Observations) -> Stated,
    noise: Noise,
    forward: &mut dyn Forward,
) -> Result<Prediction, Unpredicted> {
    predict_reading(log, 0, stated, noise, forward)
}

fn predict_reading(
    log: &FlightLog,
    extra: usize,
    stated: impl FnOnce(&Observations) -> Stated,
    noise: Noise,
    forward: &mut dyn Forward,
) -> Result<Prediction, Unpredicted> {
    let (cut, observed) = read_to_cut(log, extra)?;
    let fit = fit::fit(Some(&observed), &stated(&observed), noise, forward);
    if !fit.fitted {
        return Err(Unpredicted::NotFitted {
            cut,
            misfit_share: fit.misfit_share,
        });
    }
    let no_apogee = Unpredicted::NoApogee(cut);
    let flown = forward
        .fly(&fit.parameters, APOGEE_HORIZON_S)
        .map_err(|_| no_apogee)?;
    let (apogee_s, apogee_above_pad_m) = flown.top().ok_or(no_apogee)?;
    let apogee_reading_m = flown.top_reading_m(observed.kind).ok_or(no_apogee)?;
    Ok(Prediction {
        cut,
        fit,
        apogee_s,
        apogee_above_pad_m,
        apogee_reading_m,
    })
}

/// The exact sign-flip interval on the center of `differences`, at a coverage of at least
/// `level`: the paired difference of two methods' errors on the same flights, where a bootstrap
/// undercovers on a few ([ADR-257][adr-257] §3).
///
/// The sign-flip test of a center `μ` takes the `2ⁿ` ways of flipping the signs of `dᵢ − μ` as
/// equally likely, as they are when each difference is drawn from a distribution symmetric about
/// `μ` and independent. Inverted, it gives Hartigan's interval (J. A. Hartigan, "Using subsample values as typical
/// values", *J. Am. Stat. Assoc.* 64, 1969, pp. 1303–1317, Theorem 1): the means of the `2ⁿ − 1`
/// non-empty subsets of the differences cut the line into `2ⁿ` intervals, each holding the
/// center with probability `2⁻ⁿ`. From the `k`-th smallest subset mean to the `k`-th largest
/// holds it with probability `1 − 2k/2ⁿ`, for independent differences each symmetric about it, so
/// `k` is the largest whole number with
/// `2k/2ⁿ ≤ 1 − level`. Where `k` is 0, no pattern of signs is rare enough to reject anything:
/// the interval is the whole line, `(−∞, ∞)`, as it is at 95% for 5 flights or fewer.
///
/// Returns `None` for no differences, more than 20 (a million subsets), a level not inside 0 to
/// 1, or a difference that isn't a number.
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
#[must_use]
pub fn sign_flip_interval(differences: &[f64], level: f64) -> Option<(f64, f64)> {
    let n = differences.len();
    if n == 0
        || n > 20
        || !(level > 0.0 && level < 1.0)
        || differences.iter().any(|d| !d.is_finite())
    {
        return None;
    }
    let patterns = 1_u32 << n;
    // The largest k with 2k ≤ (1 − level)·2ⁿ.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a count below 2^20, rounded down from a positive product"
    )]
    let k = ((1.0 - level) * f64::from(patterns) / 2.0).floor() as usize;
    if k == 0 {
        return Some((f64::NEG_INFINITY, f64::INFINITY));
    }
    let mut means: Vec<f64> = (1..patterns)
        .map(|subset| {
            let (sum, count) = differences
                .iter()
                .enumerate()
                .filter(|(i, _)| subset & (1 << i) != 0)
                .fold((0.0, 0_u32), |(sum, count), (_, d)| (sum + d, count + 1));
            sum / f64::from(count)
        })
        .collect();
    means.sort_by(f64::total_cmp);
    Some((means[k - 1], means[means.len() - k]))
}

/// The paired bootstrap's percentile interval on the mean of `differences`, at `level`: the
/// paired difference of two methods' errors on the same flights, resampled by flight, as
/// [ADR-257][adr-257] §3 asks for the private collection's flights.
///
/// Each of `resamples` resamples draws `n` of the `n` differences with replacement, each index
/// equally likely, from a xoshiro256++ stream seeded with `seed` (`hpr_core::random`), and takes
/// their mean. The interval runs from the resample means' `(1 − level)/2` quantile to their
/// `(1 + level)/2` quantile, each the smallest mean at or below which that share of the means
/// lies (B. Efron and R. J. Tibshirani, *An Introduction to the Bootstrap*, Chapman & Hall,
/// 1993, §13.3, the percentile interval). Pairing keeps each flight's two errors together, so what
/// the flights share cancels in the difference.
///
/// It is approximate: on a few flights it can cover less than `level` (Efron and Tibshirani,
/// §13.5), which is why the public flights' report, on 7 flights at most, gives
/// [`sign_flip_interval`] instead. Same differences, level, count and seed, same interval, bit for
/// bit.
///
/// It keeps every resample's mean, 8 bytes each: 10,000 resamples take 80 kB. On `n` flights a
/// resample of one flight `n` times comes `(1/n)ⁿ` of the time; where that is at least
/// `(1 − level)/2` (3 flights or fewer at 95%) both ends are the smallest and largest difference,
/// and the interval is only their range.
///
/// Returns `None` for no differences, no resamples, a level not inside 0 to 1, a difference that
/// isn't finite, or an interval end that isn't finite (a mean overflowing in more than the
/// tail's share of the resamples).
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
#[must_use]
pub fn paired_bootstrap_interval(
    differences: &[f64],
    level: f64,
    resamples: u32,
    seed: u64,
) -> Option<(f64, f64)> {
    let n = differences.len();
    if n == 0
        || resamples == 0
        || !(level > 0.0 && level < 1.0)
        || differences.iter().any(|d| !d.is_finite())
    {
        return None;
    }
    let count = u64::try_from(n).ok()?;
    // A draw below the largest multiple of n that fits is kept, so each index is exactly as
    // likely as every other; the rest are drawn again.
    let zone = u64::MAX - u64::MAX % count;
    let mut rng = hpr_core::random::SeededRng::seed_from_u64(seed);
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of flights, far below 2^53"
    )]
    let size = n as f64;
    let mut means: Vec<f64> = (0..resamples)
        .map(|_| {
            let mut sum = 0.0;
            for _ in 0..n {
                let index = loop {
                    let draw = rng.next_u64();
                    if draw < zone {
                        break draw % count;
                    }
                };
                // The index is below n, a usize.
                sum += differences[usize::try_from(index).unwrap_or(0)];
            }
            sum / size
        })
        .collect();
    means.sort_by(f64::total_cmp);
    let tail = (1.0 - level) / 2.0;
    let at = |share: f64| means[percentile_rank(share, resamples) - 1];
    let (low, high) = (at(tail), at(1.0 - tail));
    (low.is_finite() && high.is_finite()).then_some((low, high))
}

/// The 1-based rank in `resamples` sorted values of the `share` quantile: the smallest `r` with
/// `r ≥ share · resamples`, at least 1 and at most `resamples`. A product within `1e-9 ·
/// resamples` of a whole number counts as that number, so `(1 − 0.95)/2` of 10,000 is rank 250,
/// not 251.
fn percentile_rank(share: f64, resamples: u32) -> usize {
    let total = f64::from(resamples);
    let product = share * total;
    let rounded = product.round();
    let rank = if (product - rounded).abs() <= 1e-9 * total {
        rounded
    } else {
        product.ceil()
    };
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a rank clamped to 1..=resamples, below 2^32"
    )]
    let rank = rank.clamp(1.0, total) as usize;
    rank
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::synthetic::designs::DesignFiles;
    use crate::synthetic::fits::{Flown, stated_for, weighed_factor};
    use crate::synthetic::logger::Profile;
    use crate::synthetic::{Flight, Flyer, Set};

    /// A draw far from anything a flight logs, in any column's unit: ±2,500.
    fn draws() -> impl FnMut() -> f64 {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            #[expect(clippy::cast_precision_loss, reason = "a draw's top bits, to a float")]
            let unit = (state >> 11) as f64 / (1_u64 << 53) as f64;
            5_000.0 * (unit - 0.5)
        }
    }

    /// `log` with every value after `cut_s` replaced by a draw, or with `nudge` moved by a
    /// thousandth of one (±2.5 in its unit); with `clocks`, each clock's later times noise too.
    /// On the log's own clock, cut by position, they fall around the cut, the first a millisecond
    /// before it. A channel on a clock of its own is cut by its times, so its first
    /// later time repeats the one before it, out of order, and the rest fall after the cut.
    fn noisy_after(log: &FlightLog, cut_s: f64, clocks: bool, nudge: bool) -> FlightLog {
        let mut draw = draws();
        let value = |v: f64, draw: f64| if nudge { v + draw / 1_000.0 } else { draw };
        let mut noisy = log.clone();
        let later = |times: &[f64]| times.iter().position(|&t| t > cut_s);
        let own = log.time_s.clone();
        for column in [
            Some(&mut noisy.altitude_m),
            noisy.vertical_speed_m_s.as_mut(),
            noisy.temperature_k.as_mut(),
            noisy.battery_v.as_mut(),
        ]
        .into_iter()
        .flatten()
        {
            for (t, v) in own.iter().zip(column.iter_mut()) {
                if *t > cut_s {
                    *v = value(*v, draw());
                }
            }
        }
        if clocks && let Some(first) = later(&own) {
            for t in &mut noisy.time_s[first..] {
                *t = cut_s + draw() / 1_000.0;
            }
            noisy.time_s[first] = cut_s - 1e-3;
        }
        for channel in &mut noisy.channels {
            let Some(first) = later(&channel.time_s) else {
                continue;
            };
            for v in &mut channel.values[first..] {
                *v = value(*v, draw());
            }
            if clocks {
                for t in &mut channel.time_s[first..] {
                    *t = cut_s + 1e-3 + draw().abs() / 1_000.0;
                }
                if first > 0 {
                    channel.time_s[first] = channel.time_s[first - 1];
                }
            }
        }
        noisy
    }

    /// `log` without its own clock's columns, as a log of channels alone; `None` if it has no
    /// pressure channel to keep its track.
    fn channels_only(log: &FlightLog) -> Option<FlightLog> {
        log.channels_of(hpr_flightdata::log::Quantity::Pressure)
            .next()?;
        let mut only = log.clone();
        only.time_s.clear();
        only.altitude_m.clear();
        only.vertical_speed_m_s = None;
        only.temperature_k = None;
        only.battery_v = None;
        Some(only)
    }

    /// The first of set A's first 60 flights with a cut that `pick` takes, as it takes it.
    fn first_cut(flyer: &mut Flyer, pick: impl Fn(Flight) -> Option<Flight>) -> (u64, Flight) {
        (0..60)
            .find_map(|k| {
                let flight = pick(flyer.flight(Set::A, k).ok()?)?;
                find_cut(&flight.log).map(|_| (k, flight))
            })
            .expect("one of set A's first 60 flights of each kind has a cut")
    }

    /// Apogee predicted from a synthetic flight's log cut after burnout stays bit-identical
    /// when every later value and time is noise; a probe that lets the prediction read a whole bin
    /// past the cut, the values noise but the times kept, doesn't. Run on three of set A's flights:
    /// the first with a cut; the first with a cut whose logger, like a TeleMetrum, writes channels
    /// besides the track (pressure, an accelerometer and a GNSS receiver's fixes); and the first
    /// with a pressure channel and a cut, its own clock's columns taken out so that the channel's
    /// clock is the track's. The finder decides from the samples up to the cut: on the log cut
    /// there it finds the same cut, and on the log cut one sample sooner none.
    #[test]
    fn the_prediction_from_the_boost_reads_nothing_past_its_cut() {
        let files = DesignFiles::committed();
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut tables = BTreeMap::new();
        let logs = [
            first_cut(&mut flyer, Some),
            first_cut(&mut flyer, |flight| {
                (flight.draw.logger.profile == Profile::Altos).then_some(flight)
            }),
            first_cut(&mut flyer, |mut flight| {
                flight.log = channels_only(&flight.log)?;
                Some(flight)
            }),
        ];
        assert!(logs[1].1.log.channels.len() >= 3, "the TeleMetrum-like log");
        for (index, flight) in logs {
            let weighed = weighed_factor(Set::A, index);
            let mut predicted = |log: &FlightLog, extra: usize| {
                let mut forward = Flown::new(&flight.draw, &files, weighed, &mut tables);
                predict_reading(
                    log,
                    extra,
                    |observed| stated_for(Some(observed)),
                    Noise::FromResiduals,
                    &mut forward,
                )
            };
            let clean = predicted(&flight.log, 0).expect("the flight's boost predicts an apogee");
            let cut = clean.cut;
            let read = readings::read(&flight.log);
            let apogee_s = match &read.apogee {
                readings::Reading::Read(apogee) => apogee.time_s,
                readings::Reading::Withheld(_) => panic!("the synthetic log reads its apogee"),
            };
            assert!(cut.time_s < apogee_s, "flight {index}: {cut:?}");
            assert_eq!(find_cut(&truncated(&flight.log, cut.time_s)), Some(cut));
            let (times, _) = readings::height_track(&in_order(&flight.log)).expect("a track");
            let sooner = times[..cut.index]
                .iter()
                .rev()
                .copied()
                .find(|t| t.is_finite())
                .expect("samples before the cut");
            assert_eq!(find_cut(&truncated(&flight.log, sooner)), None);
            let noisy_log = noisy_after(&flight.log, cut.time_s, true, false);
            assert_ne!(noisy_log, flight.log);
            let noisy = predicted(&noisy_log, 0).expect("the noisy log predicts as well");
            assert_eq!(noisy.cut, cut, "flight {index}");
            assert_eq!(
                noisy.apogee_above_pad_m.to_bits(),
                clean.apogee_above_pad_m.to_bits(),
                "flight {index}"
            );
            assert_eq!(noisy.apogee_s.to_bits(), clean.apogee_s.to_bits());
            assert_eq!(
                noisy.apogee_reading_m.to_bits(),
                clean.apogee_reading_m.to_bits()
            );
            assert_eq!(
                noisy.fit.parameters.to_array().map(f64::to_bits),
                clean.fit.parameters.to_array().map(f64::to_bits)
            );
            // The probe: the track's samples up to two bins past the cut, so that a whole bin is
            // read; with the values moved by noise of ±2.5 in each unit, the prediction sees it.
            let bin = times[cut.index + 1..]
                .iter()
                .take_while(|&&t| t <= cut.time_s + 2.0 * fit::BIN_S)
                .count();
            let values_noisy = noisy_after(&flight.log, cut.time_s, false, true);
            let probed = (
                predicted(&flight.log, bin).expect("the probe predicts"),
                predicted(&values_noisy, bin).expect("the probe predicts on noise"),
            );
            assert_ne!(
                probed.0.apogee_above_pad_m.to_bits(),
                probed.1.apogee_above_pad_m.to_bits(),
                "flight {index}: the probe must see the noise"
            );
        }
    }

    /// Over set A's first 200 synthetic flights: no GNSS tracker's log is cut, nor any whose fitted
    /// speed reaches the transonic speed; every other flight whose true top vertical speed stays
    /// 10% below that speed is cut (the fitted speed can read above the true one by its noise and
    /// the port's error). Every cut falls after the true peak of the vertical speed and after
    /// burnout, before apogee, and within 2.83 s of burnout: the time the largest fall the cut
    /// needs takes under gravity alone, plus a window. Each cut found on a log with one bad sample
    /// put in, or two or three side by side, passes the same checks: 100 m or 15 m up halfway
    /// through the burn, 30 m down at four fifths of it, 12 m up on the pad, or 15 m down at the
    /// first. A cut the bad samples lose is refused for its speed. A GNSS log is refused as one, and
    /// any other uncut log for its speed. Measured, not asserted: 51 cut,
    /// 1.08 s to 2.30 s after burnout; 43 GNSS logs and 106 fast ones refused, the slowest of those
    /// at 221.3 m/s. Of the 785 spiked logs of each run's length, 255 of flights that are cut: one
    /// bad sample made or lost no cut and moved 8 by 0.70 s at most; two side by side, 9 by 0.70 s
    /// at most, none made or lost; three side by side moved 23 by 1.35 s at most and lost 40, their
    /// speed read past the transonic one (100 m up halfway through the burn, or 30 m down at four
    /// fifths). Measured once on the earlier rule, before a cut had to stand above the pad by
    /// [`LIFTOFF_RISE_M`]: three 12 m up on the pad gave 33 cuts before the speed's peak, 32 of them
    /// before ignition, 2.37 s to 7.85 s before burnout. A step halfway through the pad that stays,
    /// of 8, 12, 30, 60 or 100 m, is cut as the flight is or refused for its speed. Measured, not
    /// asserted: 51 cut and 106 refused for each step to 60 m, all 157 refused at 100 m; steps of 12
    /// to 60 m cut 142 before the peak before [`STILL_M_S`] (#1362, measured once). Set B's first 200, run once before the
    /// [`HOLD_S`] rule, not in the test: 57 cut, 0.35 s to 1.55 s after burnout, 34 GNSS and 109
    /// fast refused.
    #[test]
    fn the_cut_falls_just_after_the_true_burnout() {
        let latest_s = (DROP_SHARE * readings::TRANSONIC_SPEED_M_S).max(DROP_FLOOR_M_S)
            / hpr_core::gravity::STANDARD_GRAVITY_MPS2
            + SPEED_WINDOW_S;
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut cut_count = 0_usize;
        let (mut early, mut late) = (f64::INFINITY, 0.0_f64);
        #[derive(Default)]
        struct Spiked {
            logs: usize,
            moved: usize,
            made: usize,
            lost: usize,
            moved_most_s: f64,
        }
        let mut runs: [Spiked; 3] = Default::default();
        const STEPS_M: [f64; 5] = [8.0, 12.0, 30.0, 60.0, 100.0];
        // Of each step's logs, those cut and those refused as transonic.
        let mut steps = [(0_usize, 0_usize); STEPS_M.len()];
        for k in 0..200 {
            let flight = flyer.flight(Set::A, k).expect("set A's flights fly");
            let cut = find_cut(&flight.log);
            if flight.draw.logger.profile == Profile::GnssTracker {
                assert_eq!(try_cut(&flight.log), Err(NoCut::Gnss), "flight {k}");
                continue;
            }
            let peak = flight
                .truth
                .samples
                .iter()
                .max_by(|a, b| a.vertical_speed_m_s.total_cmp(&b.vertical_speed_m_s))
                .expect("the truth has samples");
            let pad_s = flight.draw.logger.pad_s;
            let events = &flight.truth.events;
            let check = |cut: &Cut, what: &str| {
                let after_burnout_s = cut.time_s - (pad_s + events.burnout_s);
                assert!(
                    cut.time_s > pad_s + peak.time_s,
                    "flight {k}{what}: cut before the speed's peak"
                );
                assert!(
                    after_burnout_s > 0.0 && after_burnout_s <= latest_s,
                    "flight {k}{what}: cut {after_burnout_s} s after burnout"
                );
                assert!(
                    cut.time_s < pad_s + events.apogee_s,
                    "flight {k}{what}: cut at or past apogee"
                );
                assert!(cut.top_speed_m_s < readings::TRANSONIC_SPEED_M_S);
                assert!(
                    cut.top_speed_s > pad_s,
                    "flight {k}{what}: top speed read before ignition"
                );
            };
            match &cut {
                Some(cut) => {
                    cut_count += 1;
                    let after = cut.time_s - (pad_s + events.burnout_s);
                    early = early.min(after);
                    late = late.max(after);
                    check(cut, "");
                }
                None => {
                    assert!(
                        peak.vertical_speed_m_s >= 0.9 * readings::TRANSONIC_SPEED_M_S,
                        "flight {k}, its top {} m/s, has no cut",
                        peak.vertical_speed_m_s
                    );
                    assert!(
                        matches!(try_cut(&flight.log), Err(NoCut::Transonic { speed_m_s, .. })
                            if speed_m_s >= readings::TRANSONIC_SPEED_M_S),
                        "flight {k}: refused, but not for its speed"
                    );
                }
            }
            let burn_s = pad_s + events.burnout_s;
            for (at_s, by_m) in [
                (pad_s + 0.5 * events.burnout_s, 100.0),
                (pad_s + 0.5 * events.burnout_s, 15.0),
                (pad_s + 0.8 * events.burnout_s, -30.0),
                (0.5 * pad_s, 12.0),
                (f64::NEG_INFINITY, -15.0),
            ] {
                let Some(i) = flight.log.time_s.iter().position(|&t| t >= at_s) else {
                    continue;
                };
                for run in 1..=3 {
                    let mut spiked = flight.log.clone();
                    let Some(bad) = spiked.altitude_m.get_mut(i..i + run) else {
                        continue;
                    };
                    for h in bad {
                        *h += by_m;
                    }
                    let what = format!(" with {by_m} m on {run} at {at_s} s (burnout {burn_s} s)");
                    let spiked_found = find_cut(&spiked);
                    if let Some(spiked_cut) = &spiked_found {
                        check(spiked_cut, &what);
                    }
                    let tally = &mut runs[run - 1];
                    tally.logs += 1;
                    match (&cut, &spiked_found) {
                        (Some(a), Some(b)) if a.time_s != b.time_s => {
                            tally.moved += 1;
                            tally.moved_most_s =
                                tally.moved_most_s.max((a.time_s - b.time_s).abs());
                        }
                        (Some(_), None) => {
                            assert!(
                                matches!(try_cut(&spiked), Err(NoCut::Transonic { .. })),
                                "flight {k}{what}: cut lost, but not for its speed"
                            );
                            tally.lost += 1;
                        }
                        (None, Some(_)) => tally.made += 1,
                        _ => {}
                    }
                }
            }
            // A step halfway through the pad that stays, every later sample raised.
            for (j, by_m) in STEPS_M.into_iter().enumerate() {
                let mut stepped = flight.log.clone();
                for (t, h) in stepped.time_s.iter().zip(&mut stepped.altitude_m) {
                    if *t >= 0.5 * pad_s {
                        *h += by_m;
                    }
                }
                let what = format!(" with a {by_m} m step from {} s", 0.5 * pad_s);
                match try_cut(&stepped) {
                    Ok(stepped_cut) => {
                        check(&stepped_cut, &what);
                        steps[j].0 += 1;
                    }
                    Err(NoCut::Transonic { .. }) => steps[j].1 += 1,
                    Err(why) => panic!("flight {k}{what}: refused, {why:?}"),
                }
            }
        }
        println!("set A: {cut_count} cut, {early:.2} s to {late:.2} s after burnout");
        for (run, tally) in runs.iter().enumerate() {
            println!(
                "{} bad side by side: of {} spiked logs {} moved, by {:.3} s at most, {} made, {} lost",
                run + 1,
                tally.logs,
                tally.moved,
                tally.moved_most_s,
                tally.made,
                tally.lost
            );
        }
        for (by_m, (cut, refused)) in STEPS_M.iter().zip(steps) {
            println!(
                "a {by_m} m step on the pad that stays: {cut} cut, {refused} refused as transonic"
            );
        }
        assert!(cut_count >= 51, "{cut_count} cut");
    }

    /// A barometer's log every 0.05 s, on the pad until 3.0 s with 0.3 m of noise on every
    /// sample (uniform, its standard deviation 0.3 m), then a climb at net acceleration `climb`
    /// of the time since ignition until `burn_s` after it, then a coast in gravity alone; with
    /// `bad_m` added to the three samples from 1.5 s on, or to every sample from 1.5 s on when
    /// `stays`. The height track of [#1362][i1362]'s hand-built logs.
    ///
    /// [i1362]: https://github.com/nrdptel/fusionspace-eridanus/issues/1362
    fn pad_spiked_log(
        climb: &dyn Fn(f64) -> f64,
        burn_s: f64,
        bad_m: f64,
        stays: bool,
    ) -> FlightLog {
        const STEP_S: f64 = 0.05;
        const SUBSTEPS: u32 = 50;
        let g = hpr_core::gravity::STANDARD_GRAVITY_MPS2;
        let mut draw = draws();
        let (mut v, mut h) = (0.0_f64, 0.0_f64);
        let mut time_s = Vec::new();
        let mut altitude_m = Vec::new();
        for i in 0..500_u32 {
            let t = f64::from(i) * STEP_S;
            if i > 0 {
                // The climb by the midpoint rule in small steps: a millimetre's error at most.
                let dt = STEP_S / f64::from(SUBSTEPS);
                for j in 0..SUBSTEPS {
                    let tau = t - STEP_S + (f64::from(j) + 0.5) * dt - 3.0;
                    let a = if tau <= 0.0 {
                        0.0
                    } else if tau < burn_s {
                        climb(tau)
                    } else {
                        -g
                    };
                    h += v * dt + 0.5 * a * dt * dt;
                    v += a * dt;
                }
                h = h.max(0.0);
            }
            // A draw is uniform over ±2,500: scaled to ±0.3·√3 m, its deviation is 0.3 m.
            let noise = draw() / 2_500.0 * 0.3 * 3.0_f64.sqrt();
            let bad = if (1.5..1.5 + 3.0 * STEP_S - 1e-9).contains(&t) || (stays && t >= 1.5) {
                bad_m
            } else {
                0.0
            };
            time_s.push(t);
            altitude_m.push(h + noise + bad);
        }
        FlightLog {
            format: hpr_flightdata::log::LogFormat::GenericTable,
            logger: "a test's".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: hpr_flightdata::log::Stated::default(),
            time_s,
            altitude_m,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: Vec::new(),
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// [#1362][i1362]'s logs: three bad samples side by side on the pad, faster than the flight's
    /// own top speed, don't stay its top, and a 15 m step on the pad that stays doesn't cut the
    /// flight early. Ignition is at 3.0 s. A 2.0 s burn at 40 m/s² to 80 m/s with 45 m on the
    /// pad, a 1.25 s burn to 50 m/s with 30 m, and a long tail-off, its net push falling to the
    /// weight 4.95 s into a 6.0 s burn, with 100 m; then the first flight with 15 m from 1.5 s on.
    /// Each is cut where its clean log is, its top speed read after ignition, after its true peak
    /// and its burnout. Before [`STILL_M_S`] the spiked three were cut at 5.90, 5.05 and 8.75 s
    /// (their clean logs at 7.25, 6.50 and 10.60 s), each top read at 1.6 s on the pad, the
    /// tail-off's cut before its burnout; the step was cut at 3.05 s, 0.05 s after ignition.
    ///
    /// [i1362]: https://github.com/nrdptel/fusionspace-eridanus/issues/1362
    #[test]
    fn a_bad_sample_on_the_pad_is_not_the_top_speed() {
        let g = hpr_core::gravity::STANDARD_GRAVITY_MPS2;
        // The tail-off's net push falls in a line to the weight's pull at its burnout, 6.0 s in.
        let tail_off = move |tau: f64| g / 1.05 * (4.95 - tau);
        let constant = |_tau: f64| 40.0;
        // (what, net push, burn, true peak's time, bad metres, a step that stays)
        let cases: [(&str, &dyn Fn(f64) -> f64, f64, f64, f64, bool); 4] = [
            ("80 m/s", &constant, 2.0, 5.0, 45.0, false),
            ("50 m/s", &constant, 1.25, 4.25, 30.0, false),
            ("the tail-off", &tail_off, 6.0, 7.95, 100.0, false),
            ("80 m/s, a step", &constant, 2.0, 5.0, 15.0, true),
        ];
        let found: Vec<_> = cases
            .iter()
            .map(|&(what, climb, burn_s, _, bad_m, stays)| {
                let clean = try_cut(&pad_spiked_log(climb, burn_s, 0.0, false));
                let spiked = try_cut(&pad_spiked_log(climb, burn_s, bad_m, stays));
                println!("{what}: clean {clean:?}, {bad_m} m on the pad: {spiked:?}");
                (clean, spiked)
            })
            .collect();
        for ((what, _, burn_s, peak_s, _, _), (clean, spiked)) in cases.into_iter().zip(found) {
            let clean = clean.unwrap_or_else(|why| panic!("{what}, clean: refused, {why:?}"));
            for (cut, which) in [(Ok(clean), "clean"), (spiked, "with the bad samples")] {
                let cut = cut.unwrap_or_else(|why| panic!("{what}, {which}: refused, {why:?}"));
                assert!(
                    cut.top_speed_s > 3.0,
                    "{what}, {which}: top read at {} s",
                    cut.top_speed_s
                );
                assert!(
                    cut.time_s > peak_s,
                    "{what}, {which}: cut at {} s",
                    cut.time_s
                );
                assert!(
                    cut.time_s > 3.0 + burn_s,
                    "{what}, {which}: cut at {} s",
                    cut.time_s
                );
                assert_eq!(
                    cut.index, clean.index,
                    "{what}, {which}: moved from the clean cut"
                );
            }
        }
    }

    /// [`median_of`] gives exactly what sorting by `f64::total_cmp` and taking the middle gives,
    /// on odd and even counts, repeats, signed zeros, infinities and NaN.
    #[test]
    fn the_median_by_selection_is_the_sorted_one() {
        let sorted_median = |values: &[f64]| {
            let mut sorted = values.to_vec();
            sorted.sort_by(f64::total_cmp);
            let mid = sorted.len() / 2;
            match (sorted.len() % 2, sorted.get(mid)) {
                (1, Some(&middle)) => middle,
                (_, Some(&upper)) => sorted
                    .get(mid.wrapping_sub(1))
                    .map_or(upper, |&lower| (lower + upper) / 2.0),
                (_, None) => f64::NAN,
            }
        };
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let specials = [
            0.0,
            -0.0,
            1.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            1e-300,
        ];
        for count in 0..40 {
            for round in 0..50 {
                let values: Vec<f64> = (0..count)
                    .map(|_| {
                        state ^= state << 13;
                        state ^= state >> 7;
                        state ^= state << 17;
                        let pick = usize::try_from(state % 64).unwrap_or(0);
                        if round % 5 == 0 && pick < specials.len() {
                            specials[pick]
                        } else {
                            // Coarse values repeat often.
                            f64::from(u32::try_from(state % 7).unwrap_or(0)) - 3.0
                        }
                    })
                    .collect();
                let want = sorted_median(&values);
                let got = median_of(&mut values.clone());
                assert_eq!(want.to_bits(), got.to_bits(), "{values:?}");
            }
        }
    }

    /// A barometer's dip in the boost isn't burnout. Over set A's first 200 synthetic flights, on
    /// each one cut, a dip is put in the height, `depth · sin²(π (t − start) / span)` below it, its
    /// start at ignition or 0.3 or 0.6 of the way through the burn, its span 0.2 to 0.6 s and its
    /// depth 10, 20 or 40 m. Every dipped log whose fitted speed falls before burnout by what the
    /// cut needs is cut after burnout, within the 2.83 s of
    /// [`the_cut_falls_just_after_the_true_burnout`], before apogee, or refused for the transonic
    /// speed its catch-up can read. Among the logs whose falls each last 0.5 s at most and recover
    /// before burnout, as the public logs' did, the falls reach the public logs' deepest, 63.2 m/s, and
    /// last at least the public logs' longest, 0.48 s. Measured, not asserted: of 51 flights, 1,787 dipped logs give
    /// such a fall; 1,635 are cut, 0.14 s to 2.30 s after burnout, and 152 refused as transonic.
    /// 791 hold every fall to 0.5 s and recover in the burn, up to 98.9 m/s deep; 218 have a fall
    /// still running at burnout, whose depth takes in the coast's, so it isn't counted; the other
    /// 778 have a fall that lasts longer (likely the fall back from a catch-up's overshoot, not traced). The other 508 dipped
    /// logs give no such fall. With the hold alone, not the lowest
    /// speed, 416 of those 778 were cut in the burn (run once).
    #[test]
    fn a_dip_in_the_boost_is_not_burnout() {
        let latest_s = (DROP_SHARE * readings::TRANSONIC_SPEED_M_S).max(DROP_FLOOR_M_S)
            / hpr_core::gravity::STANDARD_GRAVITY_MPS2
            + SPEED_WINDOW_S;
        /// One worker's counts; each is a sum, a least or a most, so the order the workers' flights
        /// are taken in can't change the totals.
        #[derive(Clone, Copy)]
        struct Tally {
            dipped: usize,
            cut: usize,
            refused: usize,
            longer: usize,
            running: usize,
            longest_s: f64,
            deepest_m_s: f64,
            earliest_s: f64,
            latest_s: f64,
        }
        let empty = Tally {
            dipped: 0,
            cut: 0,
            refused: 0,
            longer: 0,
            running: 0,
            longest_s: 0.0,
            deepest_m_s: 0.0,
            earliest_s: f64::INFINITY,
            latest_s: 0.0,
        };
        // The 200 flights are independent: each worker flies every `workers`-th one with a flyer
        // of its own, so the test's wall time is a share of its 2,295 dipped logs, every one still
        // flown and held to the same assertions.
        let fly = |worker: u64, workers: u64| -> Tally {
            let mut flyer = Flyer::new(DesignFiles::committed());
            let mut tally = empty;
            for k in (0..200_u64).filter(|k| k % workers == worker) {
                let flight = flyer.flight(Set::A, k).expect("set A's flights fly");
                if find_cut(&flight.log).is_none() {
                    continue;
                }
                let pad_s = flight.draw.logger.pad_s;
                let events = &flight.truth.events;
                let burnout_s = pad_s + events.burnout_s;
                for start in [0.0, 0.3, 0.6] {
                    for span_s in [0.2, 0.3, 0.4, 0.5, 0.6] {
                        for depth_m in [10.0, 20.0, 40.0] {
                            let start_s = pad_s + start * events.burnout_s;
                            let mut dipped = flight.log.clone();
                            for (t, h) in dipped.time_s.iter().zip(&mut dipped.altitude_m) {
                                if (start_s..=start_s + span_s).contains(t) {
                                    *h -= depth_m
                                        * (std::f64::consts::PI * (t - start_s) / span_s)
                                            .sin()
                                            .powi(2);
                                }
                            }
                            // Each fall's time before burnout, a fall still holding at the walk's
                            // end included, and whether it still runs at burnout: its depth then
                            // takes in the coast's, so it isn't a dip's.
                            let mut falls = Vec::new();
                            let _ = walk(&dipped, f64::INFINITY, &mut |since, until, deepest| {
                                if since < burnout_s {
                                    falls.push((
                                        until.min(burnout_s) - since,
                                        deepest,
                                        until > burnout_s,
                                    ));
                                }
                            });
                            if falls.is_empty() {
                                continue;
                            }
                            tally.dipped += 1;
                            if falls.iter().any(|&(held_s, _, _)| held_s > 0.5) {
                                tally.longer += 1;
                            } else if falls.iter().any(|&(_, _, running)| running) {
                                tally.running += 1;
                            } else {
                                for &(held_s, deepest, _) in &falls {
                                    tally.longest_s = tally.longest_s.max(held_s);
                                    tally.deepest_m_s = tally.deepest_m_s.max(deepest);
                                }
                            }
                            let what = format!(
                                "flight {k}, a {depth_m} m dip over {span_s} s from {start_s} s \
                             (burnout {burnout_s} s)"
                            );
                            match try_cut(&dipped) {
                                Ok(cut) => {
                                    tally.cut += 1;
                                    let after_s = cut.time_s - burnout_s;
                                    assert!(
                                        after_s > 0.0 && after_s <= latest_s,
                                        "{what}: cut {after_s} s after burnout"
                                    );
                                    assert!(cut.time_s < pad_s + events.apogee_s, "{what}");
                                    tally.earliest_s = tally.earliest_s.min(after_s);
                                    tally.latest_s = tally.latest_s.max(after_s);
                                }
                                Err(NoCut::Transonic { .. }) => tally.refused += 1,
                                Err(why) => panic!("{what}: refused, {why:?}"),
                            }
                        }
                    }
                }
            }
            tally
        };
        let workers =
            std::thread::available_parallelism().map_or(1, |n| u64::try_from(n.get()).unwrap_or(1));
        let fly = &fly;
        let tallies: Vec<Tally> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..workers)
                .map(|worker| scope.spawn(move || fly(worker, workers)))
                .collect();
            handles
                .into_iter()
                .map(|handle| {
                    handle
                        .join()
                        .expect("a worker's assertion failed; see its message")
                })
                .collect()
        });
        let mut all = empty;
        for tally in tallies {
            all.dipped += tally.dipped;
            all.cut += tally.cut;
            all.refused += tally.refused;
            all.longer += tally.longer;
            all.running += tally.running;
            all.longest_s = all.longest_s.max(tally.longest_s);
            all.deepest_m_s = all.deepest_m_s.max(tally.deepest_m_s);
            all.earliest_s = all.earliest_s.min(tally.earliest_s);
            all.latest_s = all.latest_s.max(tally.latest_s);
        }
        let Tally {
            dipped,
            cut,
            refused,
            longer,
            running,
            longest_s,
            deepest_m_s,
            earliest_s,
            latest_s: latest_found_s,
        } = all;
        println!(
            "{dipped} dipped logs, {cut} cut {earliest_s:.2} s to {latest_found_s:.2} s \
             after burnout, {refused} refused as transonic; falls up to {longest_s:.2} s and \
             {deepest_m_s:.1} m/s in those whose falls each held 0.5 s at most and recovered in \
             the burn; {longer} with a longer one; {running} with one still running at burnout"
        );
        assert!(
            longest_s >= 0.48,
            "the longest fall kept held {longest_s} s"
        );
        assert!(
            deepest_m_s >= 63.2,
            "the deepest fall kept went {deepest_m_s} m/s"
        );
        assert!(cut >= 1600, "{cut} dipped logs cut");
    }

    /// A hostile log is refused without a panic, and each refusal names its reason: no samples, one,
    /// times all equal, not numbers or infinite, heights at the float's edge, and a log sampled
    /// every microsecond, faster than the analyzer reads, 200,000 samples refused at its first
    /// window; a log of channels alone is refused too, naming its reason: too sparse with its track
    /// emptied, its times or values not numbers, or its values infinite; too fast with its samples a
    /// microsecond apart. A log at the
    /// analyzer's fastest rate fills a
    /// window to the most it allows, 251 samples, and is read.
    #[test]
    fn a_hostile_log_is_refused_without_a_panic() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        let base = flyer.flight(Set::A, 1).expect("a flight").log;
        let with = |time_s: Vec<f64>, altitude_m: Vec<f64>| {
            let mut log = base.clone();
            log.channels.clear();
            log.vertical_speed_m_s = None;
            log.temperature_k = None;
            log.battery_v = None;
            log.time_s = time_s;
            log.altitude_m = altitude_m;
            log
        };
        #[expect(clippy::cast_precision_loss, reason = "small counts")]
        let climb = |n: usize, dt: f64| {
            let times: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
            let heights = times.iter().map(|t| 100.0 * t).collect();
            (times, heights)
        };
        let (dense_t, dense_h) = climb(200_000, 1e-6);
        let cases = [
            with(Vec::new(), Vec::new()),
            with(vec![0.0], vec![0.0]),
            with(vec![1.0; 50], (0..50).map(f64::from).collect()),
            with(
                (0..50)
                    .map(|i| if i % 2 == 0 { f64::NAN } else { f64::INFINITY })
                    .collect(),
                vec![0.0; 50],
            ),
            with(
                (0..50).map(|i| 1e300 * f64::from(i)).collect(),
                (0..50).map(|i| 1e308 * f64::from(i % 2)).collect(),
            ),
            with(dense_t, dense_h),
        ];
        for log in &cases {
            assert_eq!(find_cut(log), None);
        }
        // And says why: an empty log holds no track; one sample, times all equal, not numbers, or
        // a lifetime apart are too few to a window; the dense log reads too fast.
        let why: Vec<_> = cases.iter().map(try_cut).collect();
        assert_eq!(
            why,
            [
                Err(NoCut::NoTrack),
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooFast),
            ]
        );
        // A steady climb logged at 5 Hz is too sparse, at 20 Hz never slowing, and a log at rest
        // at 20 Hz never lifts off.
        let (t, h) = climb(100, 0.2);
        assert_eq!(try_cut(&with(t, h)), Err(NoCut::TooSparse));
        let (t, h) = climb(400, 0.05);
        assert_eq!(try_cut(&with(t.clone(), h)), Err(NoCut::NeverSlowed));
        assert_eq!(try_cut(&with(t, vec![0.0; 400])), Err(NoCut::NeverLifted));
        // At the analyzer's fastest rate, 500 Hz, and with intervals a part in a thousand shorter,
        // a window holds 251 samples, the most allowed, and is read; with intervals 0.6% or 2%
        // shorter it holds 252 or 256 and is refused.
        assert_eq!(most_in(SPEED_WINDOW_S), 251);
        for (dt, held, why) in [
            (readings::MIN_INTERVAL_S, 251, NoCut::NeverSlowed),
            (readings::MIN_INTERVAL_S * 0.999, 251, NoCut::NeverSlowed),
            (readings::MIN_INTERVAL_S * 0.994, 252, NoCut::TooFast),
            (readings::MIN_INTERVAL_S * 0.98, 256, NoCut::TooFast),
        ] {
            let (t, h) = climb(1_000, dt);
            let last = t[t.len() - 1];
            let most = t.iter().filter(|&&s| s >= last - SPEED_WINDOW_S).count();
            assert_eq!(most, held, "{dt} s");
            assert_eq!(try_cut(&with(t, h)), Err(why), "{dt} s");
        }
        // A log of channels alone, with no clock of its own, its pressure channel the track: emptied,
        // its times or values not numbers, or its values infinite, the track holds no window of
        // samples it can read; its samples 1 µs apart, it reads too fast. An emptied channel is
        // still the log's track, so it is too sparse, where a log with no columns has none.
        let only = (0..60)
            .find_map(|k| channels_only(&flyer.flight(Set::A, k).ok()?.log))
            .expect("one of set A's first 60 flights logs pressure");
        let channeled = |change: &dyn Fn(&mut hpr_flightdata::log::Channel)| {
            let mut log = only.clone();
            for channel in &mut log.channels {
                if channel.quantity == hpr_flightdata::log::Quantity::Pressure {
                    change(channel);
                }
            }
            log
        };
        let refused = [
            channeled(&|c| {
                c.time_s.clear();
                c.values.clear();
            }),
            channeled(&|c| c.time_s.fill(f64::NAN)),
            channeled(&|c| c.values.fill(f64::NAN)),
            channeled(&|c| c.values.fill(f64::INFINITY)),
            channeled(&|c| {
                #[expect(clippy::cast_precision_loss, reason = "small counts")]
                for (i, t) in c.time_s.iter_mut().enumerate() {
                    *t = i as f64 * 1e-6;
                }
            }),
        ];
        let why: Vec<_> = refused.iter().map(try_cut).collect();
        assert_eq!(
            why,
            [
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooSparse),
                Err(NoCut::TooFast),
            ]
        );
    }

    /// A forward model that flies as the flight's own does but refuses each flight `refuses`
    /// takes, by the time it is flown to.
    struct Refusing<'a, 'b> {
        inner: &'a mut Flown<'b>,
        refuses: fn(f64) -> bool,
    }

    impl Forward for Refusing<'_, '_> {
        fn fly(
            &mut self,
            p: &fit::Parameters,
            until_s: f64,
        ) -> Result<crate::residuals::Simulated, String> {
            if (self.refuses)(until_s) {
                return Err("refused".to_owned());
            }
            self.inner.fly(p, until_s)
        }

        fn fly_sheared(
            &mut self,
            p: &fit::Parameters,
            until_s: f64,
            exponent: f64,
        ) -> Result<crate::residuals::Simulated, String> {
            if (self.refuses)(until_s) {
                return Err("refused".to_owned());
            }
            self.inner.fly_sheared(p, until_s, exponent)
        }
    }

    /// Each variant of [`Unpredicted`] is matched by one case: a log with no cut says why the finder
    /// found none; a boost cut within 2 s of liftoff holds too little to fit; a flight that can't be
    /// flown keeps the fit to what was stated; and a fitted flight that can't be flown to its apogee
    /// reaches none. The flight that predicts is set A's first with a cut. Not pinned: the other
    /// ways a boost holds too little, and a fit refused for its misfit share.
    #[test]
    fn each_refusal_of_a_prediction_names_its_reason() {
        let files = DesignFiles::committed();
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut tables = BTreeMap::new();
        let (index, flight) = first_cut(&mut flyer, Some);
        let weighed = weighed_factor(Set::A, index);
        let mut flown = Flown::new(&flight.draw, &files, weighed, &mut tables);
        let mut predicted = |log: &FlightLog, refuses: fn(f64) -> bool| {
            let mut forward = Refusing {
                inner: &mut flown,
                refuses,
            };
            predict(
                log,
                |observed| stated_for(Some(observed)),
                Noise::FromResiduals,
                &mut forward,
            )
        };
        let cut = find_cut(&flight.log).expect("the flight is cut");
        assert!(predicted(&flight.log, |_| false).is_ok());
        assert_eq!(
            predicted(&flight.log, |_| true),
            Err(Unpredicted::NotFitted {
                cut,
                misfit_share: None
            })
        );
        assert_eq!(
            predicted(&flight.log, |until_s| until_s >= APOGEE_HORIZON_S),
            Err(Unpredicted::NoApogee(cut))
        );
        // A log at rest, and a short boost: 2 s on the pad at 20 Hz, 0.5 s at 200 m/s², then a
        // coast slowing at 60 m/s².
        let mut log = flight.log.clone();
        log.channels.clear();
        log.vertical_speed_m_s = None;
        log.temperature_k = None;
        log.battery_v = None;
        log.time_s = (0..100).map(|i| f64::from(i) * 0.05).collect();
        log.altitude_m = vec![0.0; 100];
        assert_eq!(
            predicted(&log, |_| false),
            Err(Unpredicted::NoCut(NoCut::NeverLifted))
        );
        log.altitude_m = log
            .time_s
            .iter()
            .map(|&t| match t {
                t if t <= 2.0 => 0.0,
                t if t <= 2.5 => 100.0 * (t - 2.0).powi(2),
                t => 25.0 + 100.0 * (t - 2.5) - 30.0 * (t - 2.5).powi(2),
            })
            .collect();
        let short = find_cut(&log).expect("the short boost is cut");
        assert!(short.time_s < 4.0, "{short:?}");
        assert_eq!(
            predicted(&log, |_| false),
            Err(Unpredicted::NothingToFit(short))
        );
    }

    /// The two one-sided p-values of the sign-flip test of the center `mu`, every pattern of signs
    /// counted: the share of patterns whose sum of `dᵢ − μ` is at most the observed one's, and at
    /// least.
    fn sign_flip_p(differences: &[f64], mu: f64) -> (f64, f64) {
        let n = differences.len();
        let observed: f64 = differences.iter().map(|d| d - mu).sum();
        let (mut low, mut high) = (0_u32, 0_u32);
        for pattern in 0..(1_u32 << n) {
            let sum: f64 = differences
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    if pattern & (1 << i) == 0 {
                        d - mu
                    } else {
                        mu - d
                    }
                })
                .sum();
            low += u32::from(sum <= observed);
            high += u32::from(sum >= observed);
        }
        let all = f64::from(1_u32 << n);
        (f64::from(low) / all, f64::from(high) / all)
    }

    /// Up to 5 differences no pattern of signs is rare enough at 95%: the whole line. At 6 the
    /// interval runs from the least to the greatest, holding the center with probability
    /// `1 − 2/64`. At 7 and 9 each end is where the test, every pattern counted, first rejects at
    /// 2.5% a side: a center just inside holds both p-values above 2.5%, one just outside doesn't.
    #[test]
    fn the_sign_flip_interval_is_the_tests_inversion() {
        let five = [1.0, -2.0, 3.5, 0.25, 7.0];
        assert_eq!(
            sign_flip_interval(&five, 0.95),
            Some((f64::NEG_INFINITY, f64::INFINITY))
        );
        let six = [1.0, -2.0, 3.5, 0.25, 7.0, -0.5];
        assert_eq!(sign_flip_interval(&six, 0.95), Some((-2.0, 7.0)));
        for differences in [
            vec![-3.1, 0.4, 2.2, -0.7, 5.3, 1.9, -1.2],
            vec![0.3, -4.2, 1.1, 2.7, -0.2, 3.3, 6.1, -2.4, 0.9],
        ] {
            let (low, high) = sign_flip_interval(&differences, 0.95).expect("an interval");
            assert!(low.is_finite() && high.is_finite() && low < high);
            let step = 1e-9;
            for (mu, inside) in [
                (low + step, true),
                (high - step, true),
                (low - step, false),
                (high + step, false),
            ] {
                let (p_low, p_high) = sign_flip_p(&differences, mu);
                assert_eq!(
                    p_low > 0.025 && p_high > 0.025,
                    inside,
                    "{differences:?} at {mu}: p {p_low}, {p_high}"
                );
            }
        }
        assert_eq!(sign_flip_interval(&[], 0.95), None);
        assert_eq!(sign_flip_interval(&[1.0, f64::NAN], 0.95), None);
        assert_eq!(sign_flip_interval(&six, 1.0), None);
    }

    /// On 20,000 draws of 7 differences from a symmetric distribution about 0.3 (a uniform one, on
    /// a fixed stream), the interval holds the center about `1 − 6/128 = 95.3%` of the time:
    /// within 0.8 points, five standard errors. Narrowed by one subset mean a side it would hold
    /// it `1 − 8/128 = 93.75%` of the time, which the band excludes.
    #[test]
    fn the_sign_flip_interval_covers_as_it_states() {
        let mut draw = draws();
        let trials = 20_000;
        let mut held = 0_u32;
        for _ in 0..trials {
            let differences: Vec<f64> = (0..7).map(|_| 0.3 + draw() / 2_500.0).collect();
            let (low, high) = sign_flip_interval(&differences, 0.95).expect("an interval");
            held += u32::from(low <= 0.3 && 0.3 <= high);
        }
        let share = f64::from(held) / f64::from(trials);
        assert!((share - (1.0 - 6.0 / 128.0)).abs() < 0.008, "held {share}");
        assert!(share > 1.0 - 8.0 / 128.0 + 0.008, "held {share}");
    }

    /// The quantile's rank: 2.5% and 97.5% of 10,000 are ranks 250 and 9,750 (as `(1 − 0.95)/2`
    /// computes them too, a hair above 0.025), a share between two ranks takes the higher, a share
    /// of 0 takes the first rank (clamped up from 0) and a share of 1 the last.
    #[test]
    fn the_percentile_rank_is_the_smallest_reaching_the_share() {
        assert_eq!(percentile_rank(0.025, 10_000), 250);
        assert_eq!(percentile_rank((1.0 - 0.95) / 2.0, 10_000), 250);
        assert_eq!(percentile_rank(1.0 - (1.0 - 0.95) / 2.0, 10_000), 9_750);
        assert_eq!(percentile_rank(0.5, 3), 2);
        assert_eq!(percentile_rank(0.0001, 100), 1);
        assert_eq!(percentile_rank(0.0, 100), 1);
        assert_eq!(percentile_rank(1.0, 100), 100);
    }

    /// Exact cases, whatever the stream draws. One difference: every resample is it. Equal ones:
    /// the same. Two, `a` and `b`: a resample's mean is `a` a quarter of the time, so both 2.5%
    /// tails sit on the ends. Four zeros and a 10: a resample's mean is 2k for k ~ Binomial(5, 1/5),
    /// P(k ≤ 2) = 0.942 and P(k ≤ 3) = 0.9933, so the 97.5% quantile is 6 and the 2.5% one is 0
    /// (P(k = 0) = 0.328); on 10,000 resamples each share sits more than 11 standard errors from 97.5%.
    #[test]
    fn the_paired_bootstrap_takes_the_percentiles() {
        for seed in [1, 7, 2_026] {
            assert_eq!(
                paired_bootstrap_interval(&[3.5], 0.95, 10_000, seed),
                Some((3.5, 3.5))
            );
            assert_eq!(
                paired_bootstrap_interval(&[-1.25; 6], 0.95, 10_000, seed),
                Some((-1.25, -1.25))
            );
            assert_eq!(
                paired_bootstrap_interval(&[-2.0, 5.0], 0.95, 10_000, seed),
                Some((-2.0, 5.0))
            );
            assert_eq!(
                paired_bootstrap_interval(&[0.0, 0.0, 10.0, 0.0, 0.0], 0.95, 10_000, seed),
                Some((0.0, 6.0))
            );
        }
        assert_eq!(paired_bootstrap_interval(&[], 0.95, 10_000, 1), None);
        assert_eq!(paired_bootstrap_interval(&[1.0], 0.95, 0, 1), None);
        assert_eq!(paired_bootstrap_interval(&[1.0], 1.0, 10_000, 1), None);
        assert_eq!(
            paired_bootstrap_interval(&[1.0, f64::NAN], 0.95, 10, 1),
            None
        );
        // Finite differences whose sum overflows.
        assert_eq!(paired_bootstrap_interval(&[f64::MAX; 3], 0.95, 10, 1), None);
    }

    /// On 2 resamples the 2.5% point is rank 1, the lower mean, and the 97.5% point rank 2, the
    /// higher: an off-by-one in the rank would put both ends on one mean. Seed 3 draws two
    /// different resample means of these two differences (a resample's mean is −2, 1.5 or 5).
    #[test]
    fn the_paired_bootstrap_takes_each_rank_from_the_sorted_means() {
        let (low, high) = paired_bootstrap_interval(&[-2.0, 5.0], 0.95, 2, 3).expect("an interval");
        assert!(low < high, "{low}, {high}");
        for end in [low, high] {
            assert!([-2.0, 1.5, 5.0].contains(&end), "{end}");
        }
    }

    /// Same differences and seed, same interval bit for bit; another seed moves it a little.
    #[test]
    fn the_paired_bootstrap_is_its_seeds() {
        let differences = [-3.1, 0.4, 2.2, -0.7, 5.3, 1.9, -1.2, 0.8, -2.6, 4.4];
        let one = paired_bootstrap_interval(&differences, 0.95, 2_000, 11).expect("an interval");
        let again = paired_bootstrap_interval(&differences, 0.95, 2_000, 11).expect("an interval");
        let other = paired_bootstrap_interval(&differences, 0.95, 2_000, 12).expect("an interval");
        assert_eq!(
            (one.0.to_bits(), one.1.to_bits()),
            (again.0.to_bits(), again.1.to_bits())
        );
        assert_ne!(one, other);
        assert!(one.0 < 0.74 && 0.74 < one.1, "{one:?} holds the mean");
    }

    /// On 2,000 draws of 40 differences from a symmetric distribution about 0.3 (a uniform one, on
    /// a fixed stream), the 95% interval holds the center 92% to 96.5% of the time: the percentile
    /// interval's coverage on 40 is near its level. Against 95% the band runs about 6 standard
    /// errors of the share below and 3 above. Taking a 90% interval instead held it 89.25% of the
    /// time, outside the band (a probe, run once).
    #[test]
    fn the_paired_bootstrap_covers_near_its_level() {
        let mut draw = draws();
        let trials = 2_000;
        let mut held = 0_u32;
        for trial in 0..trials {
            let differences: Vec<f64> = (0..40).map(|_| 0.3 + draw() / 2_500.0).collect();
            let (low, high) =
                paired_bootstrap_interval(&differences, 0.95, 1_000, trial).expect("an interval");
            held += u32::from(low <= 0.3 && 0.3 <= high);
        }
        let share = f64::from(held) / 2_000.0;
        assert!((0.92..=0.965).contains(&share), "held {share}");
    }
}
