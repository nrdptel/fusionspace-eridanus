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
//!   Liftoff is where that line stands [`LIFTOFF_RISE_M`] above the first window's. The cut is at
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

/// How far the window's fitted line must stand above the first window's for liftoff, m: past the
/// largest excursion of a raw height from the track's first on the pad, 6.8 m over sets A and B's
/// first 200 synthetic flights each (their barometers' logs, measured once), while a hobby rocket
/// climbs it in about a second or less. HPR Sim's choice.
pub const LIFTOFF_RISE_M: f64 = 10.0;

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
/// left out of every window that holds it; two side by side can carry the first line with them, and
/// aren't handled. Where most residuals are 0, as on a quantized pad, only the samples on the line
/// are kept.
/// `None` where fewer than [`readings::SPEED_MIN_SAMPLES`] are kept or the times don't spread.
fn robust_line(window: &VecDeque<(f64, f64)>) -> Option<(f64, f64)> {
    let points: Vec<(f64, f64)> = window.iter().copied().collect();
    let t0 = points.first()?.0;
    // The first line, robust to one bad sample: the median of the slopes between samples half the
    // window apart, each sample in at most two of them, and the median of the heights' offsets.
    let apart = (points.len() / 2).max(1);
    let slope = median(
        points
            .iter()
            .zip(points.iter().skip(apart))
            .filter(|(a, b)| b.0 > a.0)
            .map(|(a, b)| (b.1 - a.1) / (b.0 - a.0)),
    );
    let intercept = median(points.iter().map(|&(t, h)| h - slope * (t - t0)));
    if !slope.is_finite() || !intercept.is_finite() {
        return None;
    }
    let residuals: Vec<f64> = points
        .iter()
        .map(|&(t, h)| h - (intercept + slope * (t - t0)))
        .collect();
    let center = median(residuals.iter().copied());
    let spread = 1.4826 * median(residuals.iter().map(|r| (r - center).abs()));
    let limit = readings::SPIKE_THRESHOLD * spread;
    let kept: Vec<(f64, f64)> = points
        .iter()
        .zip(&residuals)
        .filter(|&(_, r)| (r - center).abs() <= limit)
        .map(|(&point, _)| point)
        .collect();
    if kept.len() < readings::SPEED_MIN_SAMPLES {
        return None;
    }
    let (slope, intercept) = line(&kept)?;
    let last_s = kept.last()?.0 - kept.first()?.0;
    Some((slope, intercept + slope * last_s))
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
/// [`readings::SPEED_MIN_SAMPLES`] in a [`SPEED_WINDOW_S`] window (a log slower than 8 Hz), its
/// fitted speed reaches
/// [`readings::TRANSONIC_SPEED_M_S`] before the cut, or it never slows by [`DROP_SHARE`] of its
/// top speed and [`DROP_FLOOR_M_S`] for [`HOLD_S`], at a speed the lowest of the last [`HOLD_S`].
///
/// A GNSS receiver's altitude in the boost can drop out at ignition and burnout and lag the
/// climb, by up to 440 m in one flight a flyer reports, while its fix reads valid
/// ([GNSS in boost][gnss]): on set A's first 60 synthetic flights such tracks were cut up to 11 s
/// after burnout or not at all (a sweep run once, with an earlier rule's 1 s hold in the cut). A staged
/// flight is cut at the first burnout whose coast lasts long enough for that fall; untested on
/// one.
///
/// [gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
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
    /// No [`SPEED_WINDOW_S`] of it holds [`readings::SPEED_MIN_SAMPLES`]: a log slower than 8 Hz.
    TooSparse,
    /// Its fitted line never stands [`LIFTOFF_RISE_M`] above the first.
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
        let Some((speed, level)) = robust_line(&window) else {
            continue;
        };
        // Liftoff where the window's line, refitted without its outliers, stands high enough
        // above the first: one bad sample on the pad doesn't lift it.
        let pad = *pad_m.get_or_insert(level);
        lifted = lifted || level - pad > LIFTOFF_RISE_M;
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
        let lowest = recent.iter().all(|&(_, v)| speed <= v);
        recent.push_back((t, speed));
        match top {
            Some((best, at)) if best - speed >= (DROP_SHARE * best).max(DROP_FLOOR_M_S) => {
                let (since, deepest) = falling.get_or_insert((t, 0.0));
                *deepest = deepest.max(best - speed);
                if t - *since >= hold_s && lowest {
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

/// The median of `values`: the middle one, or the mean of the middle two.
fn median(values: impl Iterator<Item = f64>) -> f64 {
    let mut sorted: Vec<f64> = values.collect();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    match (sorted.len() % 2, sorted.get(mid)) {
        (1, Some(&middle)) => middle,
        (_, Some(&upper)) => sorted
            .get(mid.wrapping_sub(1))
            .map_or(upper, |&lower| (lower + upper) / 2.0),
        (_, None) => f64::NAN,
    }
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
    /// The log up to the cut holds too little to fit: no pad or liftoff read, or no more bins of
    /// [`fit::BIN_S`] with a sample, from a second before liftoff, than the fit has unknowns (11).
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
    /// put in passes the same checks: 100 m or 15 m up halfway through the burn, 30 m down at four
    /// fifths of it, 12 m up on the pad, or 15 m down at the first. A GNSS log is refused as one, and
    /// any other uncut log for its speed. Measured, not asserted: 51 cut,
    /// 1.08 s to 2.30 s after burnout; 43 GNSS logs and 106 fast ones refused, the slowest of those
    /// at 221.3 m/s; a bad sample made or lost no cut and moved one by 0.70 s at most, on 8 of the
    /// 255 spiked logs of flights that are cut (785 in all). Set B's first 200, run once before the
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
        let (mut spiked_logs, mut moved, mut made_or_lost, mut moved_most_s) = (0, 0, 0, 0.0_f64);
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
                let mut spiked = flight.log.clone();
                let Some(i) = spiked.time_s.iter().position(|&t| t >= at_s) else {
                    continue;
                };
                spiked.altitude_m[i] += by_m;
                let what = format!(" with {by_m} m at {at_s} s (burnout {burn_s} s)");
                let spiked_found = find_cut(&spiked);
                if let Some(spiked_cut) = &spiked_found {
                    check(spiked_cut, &what);
                }
                spiked_logs += 1;
                match (&cut, &spiked_found) {
                    (Some(a), Some(b)) if a.time_s != b.time_s => {
                        moved += 1;
                        moved_most_s = moved_most_s.max((a.time_s - b.time_s).abs());
                    }
                    (Some(_), None) | (None, Some(_)) => made_or_lost += 1,
                    _ => {}
                }
            }
        }
        println!(
            "set A: {cut_count} cut, {early:.2} s to {late:.2} s after burnout; of {spiked_logs} \
             spiked logs {moved} moved, by {moved_most_s:.3} s at most, {made_or_lost} made or lost"
        );
        assert!(cut_count >= 51, "{cut_count} cut");
    }

    /// A barometer's dip in the boost isn't burnout. Over set A's first 200 synthetic flights, on
    /// each one cut, a dip is put in the height, `depth · sin²(π (t − start) / span)` below it, its
    /// start at ignition or 0.3 or 0.6 of the way through the burn, its span 0.2 to 0.6 s and its
    /// depth 10, 20 or 40 m. Every dipped log whose fitted speed falls before burnout by what the
    /// cut needs is cut after burnout, within the 2.83 s of
    /// [`the_cut_falls_just_after_the_true_burnout`], before apogee, or refused for the transonic
    /// speed its catch-up can read. Among the logs whose falls each last 0.5 s at most before
    /// burnout, as the public logs' did, the falls reach the public logs' deepest, 63.2 m/s, and
    /// last at least the public logs' longest, 0.48 s. Measured, not asserted: of 51 flights, 1,787 dipped logs give
    /// such a fall; 1,635 are cut, 0.14 s to 2.30 s after burnout, and 152 refused as transonic.
    /// 1,009 hold every fall to 0.5 s, up to 295 m/s deep; the other 778 have a fall that lasts
    /// longer (likely the fall back from a catch-up's overshoot, not traced). The other 508 dipped
    /// logs give no such fall. With the hold alone, not the lowest
    /// speed, 416 of those 778 were cut in the burn (run once).
    #[test]
    fn a_dip_in_the_boost_is_not_burnout() {
        let latest_s = (DROP_SHARE * readings::TRANSONIC_SPEED_M_S).max(DROP_FLOOR_M_S)
            / hpr_core::gravity::STANDARD_GRAVITY_MPS2
            + SPEED_WINDOW_S;
        let mut flyer = Flyer::new(DesignFiles::committed());
        let (mut dipped_count, mut cut_count, mut refused) = (0_usize, 0_usize, 0_usize);
        let mut longer = 0_usize;
        let (mut longest_s, mut deepest_m_s) = (0.0_f64, 0.0_f64);
        let (mut earliest_s, mut latest_found_s) = (f64::INFINITY, 0.0_f64);
        for k in 0..200 {
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
                        // end included.
                        let mut falls = Vec::new();
                        let _ = walk(&dipped, f64::INFINITY, &mut |since, until, deepest| {
                            if since < burnout_s {
                                falls.push((until.min(burnout_s) - since, deepest));
                            }
                        });
                        if falls.is_empty() {
                            continue;
                        }
                        dipped_count += 1;
                        if falls.iter().any(|&(held_s, _)| held_s > 0.5) {
                            longer += 1;
                        } else {
                            for &(held_s, deepest) in &falls {
                                longest_s = longest_s.max(held_s);
                                deepest_m_s = deepest_m_s.max(deepest);
                            }
                        }
                        let what = format!(
                            "flight {k}, a {depth_m} m dip over {span_s} s from {start_s} s \
                             (burnout {burnout_s} s)"
                        );
                        match try_cut(&dipped) {
                            Ok(cut) => {
                                cut_count += 1;
                                let after_s = cut.time_s - burnout_s;
                                assert!(
                                    after_s > 0.0 && after_s <= latest_s,
                                    "{what}: cut {after_s} s after burnout"
                                );
                                assert!(cut.time_s < pad_s + events.apogee_s, "{what}");
                                earliest_s = earliest_s.min(after_s);
                                latest_found_s = latest_found_s.max(after_s);
                            }
                            Err(NoCut::Transonic { .. }) => refused += 1,
                            Err(why) => panic!("{what}: refused, {why:?}"),
                        }
                    }
                }
            }
        }
        println!(
            "{dipped_count} dipped logs, {cut_count} cut {earliest_s:.2} s to {latest_found_s:.2} s \
             after burnout, {refused} refused as transonic; falls up to {longest_s:.2} s and \
             {deepest_m_s:.1} m/s in those whose falls each held 0.5 s at most; {longer} with a \
             longer one"
        );
        assert!(
            longest_s >= 0.48,
            "the longest fall kept held {longest_s} s"
        );
        assert!(
            deepest_m_s >= 63.2,
            "the deepest fall kept went {deepest_m_s} m/s"
        );
        assert!(cut_count >= 1600, "{cut_count} dipped logs cut");
    }

    /// A hostile log is refused without a panic, and each refusal names its reason: no samples, one,
    /// times all equal, not numbers or infinite, heights at the float's edge, and a log sampled
    /// every microsecond, faster than the analyzer reads, 200,000 samples refused at its first
    /// window.
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
        // And says why: the dense log reads too fast; a steady climb logged at 5 Hz too sparse, at
        // 20 Hz never slowing, and a log at rest at 20 Hz never lifting off.
        assert_eq!(try_cut(&cases[5]), Err(NoCut::TooFast));
        let (t, h) = climb(100, 0.2);
        assert_eq!(try_cut(&with(t, h)), Err(NoCut::TooSparse));
        let (t, h) = climb(400, 0.05);
        assert_eq!(try_cut(&with(t.clone(), h)), Err(NoCut::NeverSlowed));
        assert_eq!(try_cut(&with(t, vec![0.0; 400])), Err(NoCut::NeverLifted));
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
}
