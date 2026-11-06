//! Apogee predicted from the boost: a log cut just after burnout, fitted, and flown on to apogee
//! ([ADR-257][adr-257] §3; [ADR-321][adr-321], amended 2026-11-06).
//!
//! The cut must not read the flight after it. A test replaces every later sample with noise and
//! the prediction stays bit-identical. Two rules hold that by construction:
//!
//! - [`find_cut`] walks the height track forward and decides at each sample from that sample and
//!   the ones before it: liftoff where the height first stands [`LIFTOFF_RISE_M`] above the
//!   track's first, then a vertical speed fitted over the last [`SPEED_WINDOW_S`] of samples, and
//!   the cut at the first sample where that speed has fallen [`DROP_SHARE`] of its top, and at
//!   least [`DROP_FLOOR_M_S`], below it. After burnout gravity takes about 9.8 m/s off the
//!   vertical speed every second, drag more, so the cut comes a second or two after burnout:
//!   the log keeps the coast's start.
//! - Everything after that reads the log [`truncated`] at the cut: its readings, the fit's
//!   observations and so the fit.
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

use std::collections::VecDeque;

use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::{self, Readings};
use serde::{Deserialize, Serialize};

use crate::fit::{self, Fit, Forward, Noise, Observations, Stated};
use crate::residuals::HeightKind;

/// How far above the track's first height the rocket must stand for liftoff, m: well past a
/// barometer's noise on the pad, while a hobby rocket climbs it in about a second or less. HPR
/// Sim's choice.
pub const LIFTOFF_RISE_M: f64 = 10.0;

/// The span of the trailing window the vertical speed is fitted over, s: a straight line by
/// least squares through the samples in it. HPR Sim's choice: long enough to average a
/// barometer's noise over about 10 samples at 20 Hz, short against a boost of a few seconds.
pub const SPEED_WINDOW_S: f64 = 0.5;

/// The share of its top the fitted speed must fall below that top for the cut. HPR Sim's choice:
/// 10%, past the window's noise on a fast flight's speed.
pub const DROP_SHARE: f64 = 0.1;

/// The least fall below the top speed for the cut, m/s: about a second of gravity alone, so a
/// slow flight's noise doesn't cut it during the boost. HPR Sim's choice.
pub const DROP_FLOOR_M_S: f64 = 10.0;

/// How long the fitted flight is flown to find its apogee, s: past any hobby rocket's climb.
pub const APOGEE_HORIZON_S: f64 = 300.0;

/// Where the boost's log is cut.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cut {
    /// The index of the last sample of the height track the finder read: the cut's.
    pub index: usize,
    /// That sample's time on the log's clock, s. The log is kept up to it, inclusive.
    pub time_s: f64,
    /// The end of the window whose fitted speed was the top, s: burnout falls in the
    /// [`SPEED_WINDOW_S`] before it.
    pub top_speed_s: f64,
    /// That top speed, m/s.
    pub top_speed_m_s: f64,
}

/// The slope of a straight line fitted by least squares through `window`, per second; `None`
/// where its times don't spread.
fn slope(window: &VecDeque<(f64, f64)>) -> Option<f64> {
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a second's samples, far below 2^53"
    )]
    let n = window.len() as f64;
    let (st, sh) = window
        .iter()
        .fold((0.0, 0.0), |(st, sh), &(t, h)| (st + t, sh + h));
    let (mt, mh) = (st / n, sh / n);
    let (num, den) = window.iter().fold((0.0, 0.0), |(num, den), &(t, h)| {
        (num + (t - mt) * (h - mh), den + (t - mt) * (t - mt))
    });
    (den > 0.0).then(|| num / den)
}

/// Where the boost's log is cut, decided at each sample of the height track from it and the ones
/// before it (the module's rules); `None` where the log holds no height track, never lifts off,
/// never holds 3 samples in a [`SPEED_WINDOW_S`] window (a log slower than 4 Hz), or never slows
/// by [`DROP_SHARE`] of its top speed and [`DROP_FLOOR_M_S`]. A staged flight is cut at the first
/// burnout whose coast lasts long enough for that fall, about a second; untested on one.
#[must_use]
pub fn find_cut(log: &FlightLog) -> Option<Cut> {
    let (times, heights) = readings::height_track(log)?;
    let mut first_m: Option<f64> = None;
    let mut last_s = f64::NEG_INFINITY;
    let mut lifted = false;
    let mut window: VecDeque<(f64, f64)> = VecDeque::new();
    let mut top: Option<(f64, f64)> = None;
    for (index, (&t, &h)) in times.iter().zip(&heights).enumerate() {
        // A sample out of order or not a number is passed over, as it is read.
        if !t.is_finite() || !h.is_finite() || t <= last_s {
            continue;
        }
        last_s = t;
        let pad_m = *first_m.get_or_insert(h);
        window.push_back((t, h));
        while window.front().is_some_and(|&(s, _)| s < t - SPEED_WINDOW_S) {
            window.pop_front();
        }
        lifted = lifted || h - pad_m > LIFTOFF_RISE_M;
        if !lifted || window.len() < 3 {
            continue;
        }
        let Some(speed) = slope(&window) else {
            continue;
        };
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

/// Whether a sample at `t` stays in a log cut at `cut_s`: at or before it, or with no time to
/// place it, as a gap in the clock is kept where it stands.
fn kept_at(t: f64, cut_s: f64) -> bool {
    t <= cut_s || t.is_nan()
}

/// `log` with only its samples at or before `cut_s`: its own clock's columns, each channel's
/// samples and its events. What it says of itself is kept.
#[must_use]
pub fn truncated(log: &FlightLog, cut_s: f64) -> FlightLog {
    let kept = log
        .time_s
        .iter()
        .take_while(|&&t| kept_at(t, cut_s))
        .count();
    let cut = |values: &Vec<f64>| values.iter().copied().take(kept).collect::<Vec<f64>>();
    let mut boost = log.clone();
    boost.time_s = cut(&log.time_s);
    boost.altitude_m = cut(&log.altitude_m);
    boost.vertical_speed_m_s = log.vertical_speed_m_s.as_ref().map(cut);
    boost.temperature_k = log.temperature_k.as_ref().map(cut);
    boost.battery_v = log.battery_v.as_ref().map(cut);
    for channel in &mut boost.channels {
        let kept = channel
            .time_s
            .iter()
            .take_while(|&&t| kept_at(t, cut_s))
            .count();
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
    /// The fitted flight's apogee, s after its start.
    pub apogee_s: f64,
    /// The fitted flight's apogee, m above the pad.
    pub apogee_above_pad_m: f64,
}

/// What the prediction reads of `log`, and the log it read: the log [`truncated`] at the cut
/// and `extra` samples of its track past it (none but in the test's probe).
fn read_to_cut(log: &FlightLog, extra: usize) -> Option<(Cut, Readings, Observations)> {
    let cut = find_cut(log)?;
    let to_s = match extra {
        0 => cut.time_s,
        _ => {
            let (times, _) = readings::height_track(log)?;
            *times.get(cut.index + extra)?
        }
    };
    let boost = truncated(log, to_s);
    let read = readings::read(&boost);
    let kind = HeightKind::of(read.source?);
    let observed = fit::observations_until(&boost, &read, to_s.next_up(), kind)?;
    Some((cut, read, observed))
}

/// Apogee predicted from `log` cut just after burnout: the boost fitted by [`fit::fit`], each parameter
/// held to what `stated` gives for the boost's observations, then flown on by `forward` to its
/// apogee. `None` where the log has no cut, the boost says nothing to fit, or the fitted flight
/// doesn't reach an apogee within [`APOGEE_HORIZON_S`]. The fit's own `converged` says whether
/// it settled.
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
    let (cut, _, observed) = read_to_cut(log, extra)?;
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
    use crate::synthetic::{Flyer, Set};

    /// `log` with every sample after `cut_s` replaced by noise: the same clocks, each value a
    /// draw far from anything a flight logs.
    fn noisy_after(log: &FlightLog, cut_s: f64) -> FlightLog {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut draw = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            #[expect(clippy::cast_precision_loss, reason = "a draw's top bits, to a float")]
            let unit = (state >> 11) as f64 / (1_u64 << 53) as f64;
            5_000.0 * (unit - 0.5)
        };
        let mut noisy = log.clone();
        let mut fill = |times: &[f64], values: &mut [f64]| {
            for (t, v) in times.iter().zip(values.iter_mut()) {
                if *t > cut_s {
                    *v = draw();
                }
            }
        };
        let times = noisy.time_s.clone();
        fill(&times, &mut noisy.altitude_m);
        for column in [
            &mut noisy.vertical_speed_m_s,
            &mut noisy.temperature_k,
            &mut noisy.battery_v,
        ]
        .into_iter()
        .flatten()
        {
            fill(&times, column);
        }
        for channel in &mut noisy.channels {
            let times = channel.time_s.clone();
            fill(&times, &mut channel.values);
        }
        noisy
    }

    /// Apogee predicted from a synthetic flight's log cut just after burnout stays bit-identical
    /// when every later sample is noise; a probe that lets the cut read one more sample of the
    /// track doesn't. The cut falls after the top speed and before apogee.
    #[test]
    fn the_prediction_from_the_boost_reads_nothing_past_its_cut() {
        let files = DesignFiles::committed();
        let mut flyer = Flyer::new(DesignFiles::committed());
        let mut tables = BTreeMap::new();
        let set = Set::A;
        let (index, flight) = (0..20)
            .find_map(|k| {
                let flight = flyer.flight(set, k).ok()?;
                find_cut(&flight.log).map(|_| (k, flight))
            })
            .expect("one of set A's first 20 flights has a cut");
        let weighed = weighed_factor(set, index);
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
        assert!(
            cut.top_speed_s < cut.time_s && cut.time_s < apogee_s,
            "{cut:?}"
        );
        let noisy_log = noisy_after(&flight.log, cut.time_s);
        assert_ne!(noisy_log, flight.log);
        let noisy = predicted(&noisy_log, 0).expect("the noisy log predicts as well");
        assert_eq!(noisy.cut, cut);
        assert_eq!(
            noisy.apogee_above_pad_m.to_bits(),
            clean.apogee_above_pad_m.to_bits()
        );
        assert_eq!(noisy.apogee_s.to_bits(), clean.apogee_s.to_bits());
        assert_eq!(
            noisy.fit.parameters.to_array().map(f64::to_bits),
            clean.fit.parameters.to_array().map(f64::to_bits)
        );
        // The probe: one more sample of the track, and the noise reaches the prediction.
        let one_more = (predicted(&flight.log, 1), predicted(&noisy_log, 1));
        assert_ne!(one_more.0, one_more.1, "the probe must see the noise");
    }
}
