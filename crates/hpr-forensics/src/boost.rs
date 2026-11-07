//! Apogee predicted from the boost: a log cut just after burnout, fitted, and flown on to apogee
//! ([ADR-257][adr-257] §3; [ADR-321][adr-321], amended 2026-11-06).
//!
//! The cut must not read the flight after it. A test replaces every later value and time with
//! noise and the prediction stays bit-identical. Two rules hold that by construction:
//!
//! - [`find_cut`] walks the height track forward and decides at each sample from that sample and
//!   the ones before it. A vertical speed is fitted by least squares over the last
//!   [`SPEED_WINDOW_S`] of samples, leaving out a sample the analyzer's Hampel rule calls bad, so
//!   that one bad reading moves nothing. Liftoff is where that line stands [`LIFTOFF_RISE_M`] above
//!   the first, and the cut is at the first sample where the speed has fallen below its top by
//!   [`DROP_SHARE`] of it, and by at least [`DROP_FLOOR_M_S`]. After burnout gravity takes about
//!   9.8 m/s off the vertical speed every second, drag more, so the log keeps the coast's start.
//! - Everything after that reads the log cut there: its readings, the fit's observations and so
//!   the fit. The log's own clock is cut at the cut's sample, by position, so a time it logs later
//!   can't enter. A channel on a clock of its own is cut at its first time past the cut or out of
//!   order: a sample it stamps in order at or before the cut is read wherever the file puts it,
//!   as nothing else says when it was logged.
//!
//! The finder refuses the tracks the analyzer distrusts in the boost: a GNSS receiver's, and a
//! barometer's once its speed reaches [`readings::TRANSONIC_SPEED_M_S`] (Mach 0.8 in the coldest
//! air), where the port may read the transonic flow. Without that rule, and with a 1 s hold then
//! in it, the fitted top speed was over 1.1 times the true one on 57 of set A's first 200
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

/// How far above the track's first height the rocket must stand for liftoff, m: past the largest
/// excursion from it on the pad, 6.8 m over sets A and B's first 200 synthetic flights each (their
/// barometers' logs, measured once), while a hobby rocket climbs it in about a second or less. HPR
/// Sim's choice.
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
/// line (the median of the slopes between samples half the window apart) lies further than
/// [`readings::SPIKE_THRESHOLD`] robust standard deviations (1.4826 times the residuals' median
/// absolute deviation) from their median, the analyzer's Hampel rule (Pearson et al., EUSIPCO
/// 2015, §2). One bad sample, a pulse at the port or a corrupt reading, is so left out of every
/// window that holds it; two side by side can carry the first line with them, and aren't handled.
/// Where most residuals are 0, as on a quantized pad, only the samples on the line are kept. `None` where fewer than [`readings::SPEED_MIN_SAMPLES`] are kept or the
/// times don't spread.
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
/// allows ([`readings::MIN_INTERVAL_S`]), it never lifts off, it never holds [`readings::SPEED_MIN_SAMPLES`] in a
/// [`SPEED_WINDOW_S`] window (a log slower than 8 Hz), its fitted speed reaches
/// [`readings::TRANSONIC_SPEED_M_S`] before the cut, or it never slows by [`DROP_SHARE`] of its
/// top speed and [`DROP_FLOOR_M_S`].
///
/// A GNSS receiver's altitude in the boost can drop out at ignition and burnout and lag the
/// climb, by up to 440 m in one flight a flyer reports, while its fix reads valid
/// ([GNSS in boost][gnss]): on set A's first 60 synthetic flights such tracks were cut up to 11 s
/// after burnout or not at all (a sweep run once, with a 1 s hold then in the rule). A staged
/// flight is cut at the first burnout whose coast lasts long enough for that fall; untested on
/// one.
///
/// [gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
#[must_use]
pub fn find_cut(log: &FlightLog) -> Option<Cut> {
    // On a log with its own clock the track is that clock's: its channels are never read here.
    let ordered = if log.time_s.is_empty() {
        Cow::Owned(in_order(log))
    } else {
        Cow::Borrowed(log)
    };
    if readings::height_track_source(&ordered)? == readings::Source::Gnss {
        return None;
    }
    let (times, heights) = readings::height_track(&ordered)?;
    let most = most_in(SPEED_WINDOW_S);
    let mut last_s = f64::NEG_INFINITY;
    let mut window: VecDeque<(f64, f64)> = VecDeque::new();
    let mut pad_m: Option<f64> = None;
    let mut lifted = false;
    let mut top: Option<(f64, f64)> = None;
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
            return None;
        }
        if window.len() < readings::SPEED_MIN_SAMPLES {
            continue;
        }
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
            return None;
        }
        match top {
            Some((best, at)) if speed <= best => {
                if best - speed >= (DROP_SHARE * best).max(DROP_FLOOR_M_S) {
                    return Some(Cut {
                        index,
                        time_s: t,
                        top_speed_s: at,
                        top_speed_m_s: best,
                    });
                }
            }
            _ => top = Some((speed, t)),
        }
    }
    None
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
}

/// What the prediction reads of `log`, and the log it read: the log [`truncated`] at the cut
/// and `extra` samples of its track past it (none but in the test's probe).
fn read_to_cut(log: &FlightLog, extra: usize) -> Option<(Cut, Observations)> {
    let cut = find_cut(log)?;
    let to_s = match extra {
        0 => cut.time_s,
        _ => {
            let (times, _) = readings::height_track(&in_order(log))?;
            *times.get(cut.index + extra)?
        }
    };
    // On the log's own clock, the track the finder read, the cut is by position.
    let boost = if log.time_s.is_empty() {
        truncated(log, to_s)
    } else {
        truncated_at(log, cut.index + extra + 1, to_s)
    };
    let read = readings::read(&boost);
    let kind = HeightKind::of(read.source?);
    // The probe reads its extra sample into the fit, past the whole bins the prediction keeps to.
    let observed = fit::observations_until(&boost, &read, to_s.next_up(), kind, extra == 0)?;
    Some((cut, observed))
}

/// Apogee predicted from `log` cut just after burnout: the boost fitted by [`fit::fit`], each
/// parameter held to what `stated` gives for the boost's observations, then flown on by `forward`
/// to its apogee. `None` where the log has no cut, the boost says nothing to fit, or the fitted
/// flight doesn't reach an apogee within [`APOGEE_HORIZON_S`]. The fit's own `converged` says whether
/// it settled.
#[must_use]
pub fn predict(
    log: &FlightLog,
    stated: impl FnOnce(&Observations) -> Stated,
    noise: Noise,
    forward: &mut dyn Forward,
) -> Option<Prediction> {
    predict_reading(log, 0, stated, noise, forward)
}

fn predict_reading(
    log: &FlightLog,
    extra: usize,
    stated: impl FnOnce(&Observations) -> Stated,
    noise: Noise,
    forward: &mut dyn Forward,
) -> Option<Prediction> {
    let (cut, observed) = read_to_cut(log, extra)?;
    let fit = fit::fit(Some(&observed), &stated(&observed), noise, forward);
    if !fit.fitted {
        return None;
    }
    let flown = forward.fly(&fit.parameters, APOGEE_HORIZON_S).ok()?;
    let (apogee_s, apogee_above_pad_m) = flown.top()?;
    Some(Prediction {
        cut,
        fit,
        apogee_s,
        apogee_above_pad_m,
    })
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

    /// Apogee predicted from a synthetic flight's log cut just after burnout stays bit-identical
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

    /// Over set A's first 200 synthetic flights: no GNSS tracker's log is cut, nor any whose
    /// fitted speed reaches the transonic speed; every other flight whose true top vertical speed
    /// stays 10% below that speed is cut (the fitted speed can read above the true one by its
    /// noise and the port's error). Every cut falls after the true peak of the vertical speed and
    /// after burnout, before apogee, and within 2.83 s of burnout: the time the largest fall the
    /// cut needs takes under gravity alone, plus a window. The same holds with one bad sample put
    /// in the log: 100 m or 15 m up halfway through the burn, 30 m down at four fifths of it, 12 m
    /// up on the pad, or 15 m down at the first. Measured, not asserted: 51 cut, 0.33 s to 1.48 s
    /// after burnout; 43 GNSS logs and 106 fast ones refused, the slowest of those at 221.3 m/s;
    /// a bad sample moved a cut by 0.05 s at most (up to three samples on a 100 Hz log), on 8 of
    /// the 785 logs, and made or lost none. Set B's first 200, run once, not in the test: 57 cut,
    /// 0.35 s to 1.55 s after burnout, 34 GNSS and 109 fast refused.
    #[test]
    fn the_cut_falls_just_after_the_true_burnout() {
        let latest_s = (DROP_SHARE * readings::TRANSONIC_SPEED_M_S).max(DROP_FLOOR_M_S)
            / hpr_core::gravity::STANDARD_GRAVITY_MPS2
            + SPEED_WINDOW_S;
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut cut_count = 0_usize;
        for k in 0..200 {
            let flight = flyer.flight(Set::A, k).expect("set A's flights fly");
            let cut = find_cut(&flight.log);
            if flight.draw.logger.profile == Profile::GnssTracker {
                assert_eq!(cut, None, "flight {k}: a GNSS track is cut");
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
                    check(cut, "");
                }
                None => assert!(
                    peak.vertical_speed_m_s >= 0.9 * readings::TRANSONIC_SPEED_M_S,
                    "flight {k}, its top {} m/s, has no cut",
                    peak.vertical_speed_m_s
                ),
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
                if let Some(spiked_cut) = find_cut(&spiked) {
                    check(&spiked_cut, &what);
                }
            }
        }
        assert!(cut_count >= 51, "{cut_count} cut");
    }

    /// A hostile log is refused, or cut, without a panic and in a bounded time: no samples, one,
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
    }
}
