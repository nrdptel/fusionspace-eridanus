//! The descent under each canopy ([M7.2c][m7-2c], the rest of the readings): the main's opening
//! found back from landing as Debrief finds it, then the rate of each leg.
//!
//! **The main's opening.** The rate of descent is the least-squares slope of the filtered track
//! over [`DESCENT_WINDOW_S`]. The main's rate is its median over the last [`DESCENT_TAIL_S`]
//! before landing, or the fit's span where that is longer, the last half of [`DESCENT_GUARD_S`]
//! left out; the run of samples back from landing whose rate stays within
//! [`MAIN_TOLERANCE_SHARE`] of it, or [`MAIN_TOLERANCE_M_S`] more if that is more, is the main's
//! descent, and its first sample the opening. The run is walked on the rate's running median over
//! [`DESCENT_TAIL_S`], or twice the fit where that is longer, in samples at the descent's median
//! interval and never more than it holds, so a spike of the fit's noise on a long leg doesn't end
//! it, as a step, the opening, stays where it is under a median. Every rate is
//! compared at the ground's density, scaled by `exp(−h/2H)` for its height `h` above the pad with
//! the least scale height `H`, [`DENSITY_SCALE_HEIGHT_M`]: a canopy falls as the density to the
//! −½, so one canopy from a high apogee would otherwise fall fast enough up high to read as two.
//! A drogue came before
//! it if the descent from apogee to there fell faster than [`DROGUE_RATIO`] times the main's rate,
//! by its median, as Debrief asks, and if, as HPR Sim adds, the [`DESCENT_TAIL_S`] just before the
//! run fell faster than the run's mark: a drogue whose rate crosses the mark leaves the run's
//! start in the drogue's leg. So a drogue no faster than the mark, at the ground's density, reads
//! as one canopy, whatever its ratio to the main, and the main's descent is then the whole
//! descent's ([`Reason::OneCanopy`]). These are Debrief's marks (`lib/analyze/index.ts`, MIT, the project
//! owner's own), set on its corpus of logs.
//!
//! **Its bound.** A fit centered half a window before the opening sees the drogue's descent alone,
//! faster than the mark, so the run starts no sooner than half a window and a sample before the
//! opening. After the opening the main slows the descent as quadratic drag does a falling body,
//! `dv/dt = −g (v²/v_m² − 1)` (J. R. Taylor, *Classical Mechanics*, 2005, §2.4), from any speed
//! to the mark `v₁` within
//! `τ = (v_m / 2g) ln((v₁ + v_m)/(v₁ − v_m))`, where `v_m` is the main's rate at the opening's
//! height: the rate near the ground grown by `exp(h/2H)`, since a canopy's rate goes as the air's
//! density to the −½ and the troposphere's density scale height `H` is no less than
//! [`DENSITY_SCALE_HEIGHT_M`]. So the run starts no later than `τ`, half a window and a sample
//! after the opening; the bound adds a second interval as margin. A canopy's filling time isn't
//! in it: the synthetic flights open each canopy at once.
//!
//! **Each leg's rate** is the filtered height it loses over the time it takes: the drogue's from
//! apogee to the main's opening, the main's from there to landing, or from apogee when the descent
//! shows one canopy. A leg must lose more than [`LEG_LEAST_M`] and [`LEG_LEAST_SHARE`] of its
//! top's height, as Debrief asks. Its bound is the error two heights of the same track can hold
//! between them, over the leg's time.
//!
//! [m7-2c]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c

use super::{
    Apex, BAROMETER_GAIN, DENSITY_SCALE_HEIGHT_M, DESCENT_FIT_STEPS, DESCENT_GUARD_S,
    DESCENT_TAIL_S, DESCENT_WINDOW_MAX_S, DESCENT_WINDOW_S, DRIFT_M_PER_S, DROGUE_RATIO, Descent,
    LEG_LEAST_M, LEG_LEAST_SHARE, MAIN_TOLERANCE_M_S, MAIN_TOLERANCE_SHARE, Method, NOISE_M,
    Opening, Provenance, Reading, Reason, STANDARD_GRAVITY_MPS2, TEMPERATURE_SHARE, Track,
    local_interval, median, slope_over,
};
use crate::filter::running_median;

/// The leg from sample `from`, at `from_s`, to sample `to`, or why it is too short to read.
struct Leg {
    from: usize,
    from_s: f64,
    to: usize,
    opening: Option<Opening>,
}

/// The drogue's descent and the main's, from apogee to landing at sample `landing`.
pub(super) fn read(
    track: &Track,
    filtered: &[f64],
    pad: f64,
    apex: &Apex,
    landing: usize,
    peak_bound_m: f64,
) -> (Reading<Descent>, Reading<Descent>) {
    let time = &track.time_s;
    let landing_s = time[landing];
    let both = |reason: Reason, detail: String| {
        (
            Reading::withheld(reason, detail.clone()),
            Reading::withheld(reason, detail),
        )
    };
    // A fit across a step of the track reads a spike in the rate, so the fit must hold
    // [`DESCENT_FIT_STEPS`] of the main's descent: the window is widened to hold that many at the
    // rate of the chord back from landing over that many steps and twice the noise.
    let step = track.step_at(filtered[landing]);
    let climb = DESCENT_FIT_STEPS * step + 2.0 * NOISE_M;
    let Some(back) = (apex.index..landing)
        .rev()
        .find(|&k| filtered[k] - filtered[landing] >= climb)
    else {
        return both(
            Reason::ShortLeg,
            format!(
                "the descent loses less than {climb:.1} m from apogee to landing, too little to \
                 time a canopy's leg by"
            ),
        );
    };
    let near_ground = (filtered[back] - filtered[landing]) / (landing_s - time[back]);
    let window_s = DESCENT_WINDOW_S.max(DESCENT_FIT_STEPS * step / near_ground);
    if window_s > DESCENT_WINDOW_MAX_S {
        return both(
            Reason::StepTooCoarse,
            format!(
                "the track's step, {step:.2} m, is too coarse for the descent near the ground, \
                 {near_ground:.1} m/s: a fit of its rate would span {window_s:.1} s, more than the \
                 {DESCENT_WINDOW_MAX_S} s a canopy's leg is read over"
            ),
        );
    }
    // Each rate at the ground's density: a canopy's rate goes as the density to the −½, so one
    // canopy falls faster up high, and from a high apogee would pass the mark as a drogue does.
    // Scaled by the troposphere's least scale height, it is if anything slowed with height, to
    // 21.3 km above a sea-level pad on a day 15 K cold, 17.7 km above a pad 1,500 m up
    // (docs/physics/log-readings.md, what it leaves out).
    let rate = |k: usize| {
        slope_over(time, filtered, k, window_s)
            .map(|v| -v * (-(filtered[k] - pad).max(0.0) / (2.0 * DENSITY_SCALE_HEIGHT_M)).exp())
    };
    let tail_s = DESCENT_TAIL_S.max(window_s);
    let tail: Vec<usize> = (apex.index + 1..landing)
        .filter(|&k| time[k] >= landing_s - tail_s && time[k] <= landing_s - 0.5 * DESCENT_GUARD_S)
        .collect();
    let mut tail_rates: Vec<f64> = tail.iter().filter_map(|&k| rate(k)).collect();
    let Some(main_rate) = median(&mut tail_rates) else {
        return both(
            Reason::SampledTooSlowly,
            format!(
                "the {tail_s:.1} s before landing hold too few samples for a {window_s:.1} s fit \
                 of the descent's rate"
            ),
        );
    };
    if main_rate <= 1.0 {
        return both(
            Reason::ShortLeg,
            format!(
                "the descent's rate near the ground, {main_rate:.2} m/s, is too slow to tell a \
                 canopy's leg by"
            ),
        );
    }
    let mark = (MAIN_TOLERANCE_SHARE * main_rate).max(main_rate + MAIN_TOLERANCE_M_S);
    // Back from landing while the descent stays near the main's rate, on the rate's running
    // median over [`DESCENT_TAIL_S`], or twice the fit if that is longer: on a long leg the fit's
    // noise spikes past the mark now and then, and a median takes out a spike narrower than half
    // its window while it keeps a step, the opening, where it is.
    let first = apex.index + 1;
    let rates: Vec<f64> = (first..landing)
        .map(|k| rate(k).unwrap_or(f64::NAN))
        .collect();
    // The median's half window in samples at the descent's own interval, its median, so that a
    // few samples bunched together don't widen it; never wider than the descent.
    let mut intervals: Vec<f64> = (first..landing).map(|k| time[k + 1] - time[k]).collect();
    let interval = median(&mut intervals).unwrap_or(0.0);
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of samples converts exactly"
    )]
    let samples = rates.len() as f64;
    let half = if interval > 0.0 {
        (0.5 * DESCENT_TAIL_S.max(2.0 * window_s) / interval)
            .round()
            .min(samples)
    } else {
        samples
    };
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a positive count of samples, rounded, no more than the descent holds"
    )]
    let smoothed = running_median(&rates, (half as usize).max(1));
    let steady = |k: usize| {
        smoothed
            .get(k - first)
            .is_some_and(|v| v.is_finite() && *v <= mark)
    };
    let last = tail.last().copied().unwrap_or(landing - 1);
    let mut start = last + 1;
    while start > first && steady(start - 1) {
        start -= 1;
    }
    let before = |from: f64, to: f64| {
        let mut rates: Vec<f64> = (apex.index + 1..start)
            .filter(|&k| time[k] >= from && time[k] <= to)
            .filter_map(rate)
            .collect();
        median(&mut rates)
    };
    let opened_s = time[start.min(landing)];
    let drogue_median = before(f64::NEG_INFINITY, f64::INFINITY);
    let just_before = before(
        opened_s - 0.5 * window_s - tail_s,
        opened_s - 0.5 * window_s,
    );
    let two = start < landing.saturating_sub(1)
        && opened_s > apex.time_s + DESCENT_GUARD_S
        && drogue_median.is_some_and(|v| v > DROGUE_RATIO * main_rate)
        && just_before.is_some_and(|v| v > mark);
    let leg = |leg: Leg| descent(track, filtered, pad, apex, &leg, peak_bound_m);
    if !two {
        let detail = format!(
            "the descent shows no drogue's leg, which at the ground's density falls faster than \
             {mark:.1} m/s up to the main's opening, the mark {MAIN_TOLERANCE_SHARE} times the \
             rate near the ground, {main_rate:.1} m/s, or {MAIN_TOLERANCE_M_S} m/s more if that \
             is more, and on median faster than {DROGUE_RATIO} times that rate: the rocket came \
             down under one canopy, or under a drogue no faster than the mark, which reads as \
             one, and the main's descent is the whole descent's, from apogee"
        );
        return (
            Reading::withheld(Reason::OneCanopy, detail),
            leg(Leg {
                from: apex.index,
                from_s: apex.time_s,
                to: landing,
                opening: None,
            }),
        );
    }
    // The main's rate at the opening's height, and the time it takes to slow the descent to the
    // mark; past twice the scale height's log of the mark's ratio no time is bounded, and the
    // opening is placed no closer than after apogee.
    let height = (filtered[start] - pad).max(0.0);
    let main_there = main_rate * (height / (2.0 * DENSITY_SCALE_HEIGHT_M)).exp();
    let slowing_s = if mark > main_there {
        main_there / (2.0 * STANDARD_GRAVITY_MPS2)
            * ((mark + main_there) / (mark - main_there)).ln()
    } else {
        opened_s - apex.time_s
    };
    let opening = Opening {
        time_s: opened_s,
        bound_s: slowing_s + 0.5 * window_s + 2.0 * local_interval(time, start),
        height_m: height,
    };
    (
        leg(Leg {
            from: apex.index,
            from_s: apex.time_s,
            to: start,
            opening: None,
        }),
        leg(Leg {
            from: start,
            from_s: opened_s,
            to: landing,
            opening: Some(opening),
        }),
    )
}

/// One leg's rate, or why it is too short to read.
fn descent(
    track: &Track,
    filtered: &[f64],
    pad: f64,
    apex: &Apex,
    leg: &Leg,
    peak_bound_m: f64,
) -> Reading<Descent> {
    let time = &track.time_s;
    let to_s = time[leg.to];
    let (top, bottom) = (filtered[leg.from] - pad, filtered[leg.to] - pad);
    let drop = top - bottom;
    let span = to_s - leg.from_s;
    let least = LEG_LEAST_M.max(LEG_LEAST_SHARE * top.abs());
    if span <= 0.0 || drop <= least || drop.is_nan() {
        return Reading::withheld(
            Reason::ShortLeg,
            format!(
                "the leg from {:.2} s to {to_s:.2} s loses {drop:.1} m, no more than {least:.1} m, \
                 too little for its rate to be read",
                leg.from_s
            ),
        );
    }
    let from_apogee = if leg.from == apex.index {
        peak_bound_m
    } else {
        0.0
    };
    let bound_m = (TEMPERATURE_SHARE + BAROMETER_GAIN) * drop
        + 2.0 * NOISE_M
        + DRIFT_M_PER_S * span
        + track.step_at(filtered[leg.from])
        + track.step_at(filtered[leg.to])
        + from_apogee;
    Reading::Read(Descent {
        rate_m_s: drop / span,
        bound_m_s: bound_m / span,
        from_s: leg.from_s,
        to_s,
        from_height_m: top,
        to_height_m: bottom,
        opening: leg.opening,
        source: track.source,
        provenance: Provenance::Derived,
        method: Method::LegChord,
    })
}

#[cfg(test)]
mod tests {
    use super::super::read as read_all;
    use super::*;
    use crate::log::{FlightLog, LogFormat, Stated};

    const G: f64 = STANDARD_GRAVITY_MPS2;

    /// A climb at 4 g from 1 s to 3 s (78.5 m, 78.5 m/s), a vacuum coast to apogee at 11 s
    /// (392 m), then down at `drogue` m/s to `main_m`, and at `main` m/s to the pad.
    fn flight(drogue: f64, main_m: f64, main: f64) -> impl Fn(f64) -> f64 {
        move |t| {
            let top = 2.0 * G * 4.0 + 0.5 * 8.0 * G * 8.0;
            if t <= 1.0 {
                0.0
            } else if t < 3.0 {
                2.0 * G * (t - 1.0).powi(2)
            } else if t < 11.0 {
                let c = t - 3.0;
                8.0 * G + 8.0 * G * c - 0.5 * G * c * c
            } else {
                let opened = 11.0 + (top - main_m) / drogue;
                let h = if t < opened {
                    top - drogue * (t - 11.0)
                } else {
                    main_m - main * (t - opened)
                };
                h.max(0.0)
            }
        }
    }

    /// A log of `flight` every `dt` s to 200 s, its heights rounded to `step_m` where that is
    /// positive.
    fn log(dt: f64, step_m: f64, flight: impl Fn(f64) -> f64) -> FlightLog {
        let time_s: Vec<f64> = (0..)
            .map(|i| f64::from(i) * dt)
            .take_while(|t| *t <= 200.0 + 1e-9)
            .collect();
        let altitude_m = time_s
            .iter()
            .map(|t| {
                let h = flight(*t);
                if step_m > 0.0 {
                    (h / step_m).round() * step_m
                } else {
                    h
                }
            })
            .collect();
        FlightLog {
            format: LogFormat::PerfectFlitePf2,
            logger: "PerfectFlite Pnut".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
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

    /// A drogue to 150 m at 25 m/s, then a main at 6 m/s: each leg's rate within its bound, the
    /// main's opening found back from landing within its own, and the flight time.
    #[test]
    fn two_canopies_read_each_leg_and_the_opening() {
        let top = 2.0 * G * 4.0 + 0.5 * 8.0 * G * 8.0;
        let opened = 11.0 + (top - 150.0) / 25.0;
        let readings = read_all(&log(0.05, 0.0, flight(25.0, 150.0, 6.0)));
        let drogue = readings.drogue_descent.value().unwrap();
        assert!(
            (drogue.rate_m_s - 25.0).abs() <= drogue.bound_m_s,
            "{drogue:?}"
        );
        assert_eq!(drogue.method, Method::LegChord);
        assert_eq!(drogue.provenance, Provenance::Derived);
        assert!(drogue.opening.is_none());
        let main = readings.main_descent.value().unwrap();
        assert!((main.rate_m_s - 6.0).abs() <= main.bound_m_s, "{main:?}");
        let opening = main.opening.unwrap();
        assert!(
            (opening.time_s - opened).abs() <= opening.bound_s,
            "{opening:?} vs {opened}"
        );
        assert!((opening.height_m - 150.0).abs() < 3.0, "{opening:?}");
        assert!((drogue.to_s - opening.time_s).abs() < 1e-9);
        // The bound's slowing time: the main's 6 m/s grown by exp(150/14000), slowing the descent
        // to its mark, 9.6 m/s, 1.6 times its rate, from any speed above it; and half the fit
        // beside it, which the bound would hold without the slowing time.
        let v_m = 6.0 * (opening.height_m / (2.0 * DENSITY_SCALE_HEIGHT_M)).exp();
        let mark = (6.0 * MAIN_TOLERANCE_SHARE).max(6.0 + MAIN_TOLERANCE_M_S);
        assert!((mark - 9.6).abs() < 1e-12);
        let slowing = v_m / (2.0 * G) * ((mark + v_m) / (mark - v_m)).ln();
        assert!(
            opening.bound_s >= slowing + 0.5 * DESCENT_WINDOW_S,
            "{opening:?}, {slowing}"
        );
        let flight_time = readings.flight_time.value().unwrap();
        let landed = opened + 150.0 / 6.0;
        assert!(((flight_time.time_s) - (landed - 1.0)).abs() <= flight_time.bound_s);
        assert_eq!(flight_time.method, Method::LiftoffToLanding);
    }

    /// One canopy all the way down: no drogue, and the main's rate from apogee.
    #[test]
    fn one_canopy_withholds_the_drogue() {
        let readings = read_all(&log(0.05, 0.0, flight(10.0, 0.0, 10.0)));
        assert_eq!(readings.drogue_descent.reason(), Some(Reason::OneCanopy));
        let main = readings.main_descent.value().unwrap();
        assert!((main.rate_m_s - 10.0).abs() <= main.bound_m_s, "{main:?}");
        assert!(main.opening.is_none());
        let apogee = readings.apogee.value().unwrap();
        assert!((main.from_s - apogee.time_s).abs() < 1e-9);
    }

    /// A drogue at 12 m/s over a main at 10 m/s, no faster than the main's mark, 16 m/s, 1.6 times
    /// its rate, reads as one canopy, though Debrief's ratio of 1.4 would tell them apart.
    #[test]
    fn close_rates_are_one_canopy() {
        let readings = read_all(&log(0.05, 0.0, flight(12.0, 150.0, 10.0)));
        assert_eq!(readings.drogue_descent.reason(), Some(Reason::OneCanopy));
        assert!(readings.main_descent.value().unwrap().opening.is_none());
    }

    /// A main opened 20 m below apogee leaves the drogue's leg too short to read; the main's
    /// still reads.
    #[test]
    fn a_short_drogue_leg_is_withheld() {
        let top = 2.0 * G * 4.0 + 0.5 * 8.0 * G * 8.0;
        let readings = read_all(&log(0.05, 0.0, flight(25.0, top - 20.0, 6.0)));
        assert_eq!(readings.drogue_descent.reason(), Some(Reason::ShortLeg));
        let main = readings.main_descent.value().unwrap();
        assert!((main.rate_m_s - 6.0).abs() <= main.bound_m_s, "{main:?}");
    }

    /// A drogue at 9 m/s, no faster than the mark of a main at 6 m/s (9.6 m/s), whose rate passes
    /// the mark for 0.8 s at 12 m/s, a gust's: the walk back from landing stops there, but the 2 s
    /// just before it fell slower than the mark, so no main is named there, and the descent
    /// reads as one canopy's.
    #[test]
    fn a_drogue_passing_the_mark_for_a_moment_names_no_main() {
        let top = 2.0 * G * 4.0 + 0.5 * 8.0 * G * 8.0;
        let (gust_s, gust_end_s) = (16.0, 16.8);
        let at_gust = top - 9.0 * (gust_s - 11.0);
        let after_gust = at_gust - 12.0 * (gust_end_s - gust_s);
        let opened_s = gust_end_s + (after_gust - 150.0) / 9.0;
        let readings = read_all(&log(0.05, 0.0, |t| {
            let h = if t < 11.0 {
                flight(25.0, 150.0, 6.0)(t)
            } else if t < gust_s {
                top - 9.0 * (t - 11.0)
            } else if t < gust_end_s {
                at_gust - 12.0 * (t - gust_s)
            } else if t < opened_s {
                after_gust - 9.0 * (t - gust_end_s)
            } else {
                150.0 - 6.0 * (t - opened_s)
            };
            h.max(0.0)
        }));
        assert_eq!(
            readings.drogue_descent.reason(),
            Some(Reason::OneCanopy),
            "{:?}",
            readings.drogue_descent
        );
        assert!(readings.main_descent.value().unwrap().opening.is_none());
    }

    /// One canopy from 8.6 km, falling as the air's density to the −½ with a scale height of
    /// [`DENSITY_SCALE_HEIGHT_M`]: 1.85 times its rate near the ground at apogee, past the mark,
    /// 1.6 times. At the ground's density its rate is one all the way down, and no main is named.
    #[test]
    fn one_canopy_from_high_up_reads_as_one_at_the_grounds_density() {
        // A climb at 10 g from 1 s to 5 s (784.5 m, 392.3 m/s), a vacuum coast to apogee at 45 s.
        let boost_s = 4.0;
        let v = 10.0 * G * boost_s;
        let burnout_m = 0.5 * 10.0 * G * boost_s * boost_s;
        let coast_s = v / G;
        let apogee_s = 1.0 + boost_s + coast_s;
        let apogee_m = burnout_m + 0.5 * v * coast_s;
        let (ground_m_s, scale_m) = (10.0, 2.0 * DENSITY_SCALE_HEIGHT_M);
        let height = |t: f64| {
            if t <= 1.0 {
                0.0
            } else if t < 1.0 + boost_s {
                0.5 * 10.0 * G * (t - 1.0).powi(2)
            } else if t < apogee_s {
                let c = t - 1.0 - boost_s;
                burnout_m + v * c - 0.5 * G * c * c
            } else {
                // dh/dt = −v₀ exp(h/2H): exp(−h/2H) grows by v₀/2H a second.
                let grown = (-apogee_m / scale_m).exp() + ground_m_s * (t - apogee_s) / scale_m;
                (-scale_m * grown.ln()).max(0.0)
            }
        };
        let time_s: Vec<f64> = (0..7_500).map(|i| f64::from(i) * 0.1).collect();
        let mut descent = log(0.05, 0.0, |_| 0.0);
        descent.altitude_m = time_s.iter().map(|t| height(*t)).collect();
        descent.time_s = time_s;
        let readings = read_all(&descent);
        assert!(readings.landing.value().is_some(), "{:?}", readings.landing);
        assert!(apogee_m > 8_600.0, "{apogee_m}");
        assert_eq!(
            readings.drogue_descent.reason(),
            Some(Reason::OneCanopy),
            "{:?}",
            readings.drogue_descent
        );
        let main = readings.main_descent.value().unwrap();
        assert!(main.opening.is_none(), "{main:?}");
    }

    /// Samples 1e-9 s apart at landing don't stretch the walk's running median over the whole
    /// descent: its window is set by the descent's own interval, and the drogue and the main's
    /// opening read as on an even clock.
    #[test]
    fn samples_bunched_at_landing_leave_the_median_its_window() {
        let top = 2.0 * G * 4.0 + 0.5 * 8.0 * G * 8.0;
        let opened = 11.0 + (top - 150.0) / 25.0;
        let mut bunched = log(0.05, 0.0, flight(25.0, 150.0, 6.0));
        let landed_s = read_all(&bunched).landing.value().unwrap().time_s;
        let k = bunched.time_s.iter().position(|t| *t == landed_s).unwrap();
        bunched.time_s[k - 1] = landed_s - 1e-9;
        bunched.time_s[k + 1] = landed_s + 1e-9;
        let readings = read_all(&bunched);
        assert_eq!(readings.landing.value().unwrap().time_s, landed_s);
        let drogue = readings
            .drogue_descent
            .value()
            .unwrap_or_else(|| panic!("{:?}", readings.drogue_descent));
        assert!(
            (drogue.rate_m_s - 25.0).abs() <= drogue.bound_m_s,
            "{drogue:?}"
        );
        let opening = readings.main_descent.value().unwrap().opening.unwrap();
        assert!(
            (opening.time_s - opened).abs() <= opening.bound_s,
            "{opening:?} vs {opened}"
        );
    }

    /// A track in 10 m steps over a main at 3 m/s needs a fit over 13 s to hold four steps: more
    /// than a leg is read over, so neither canopy is.
    #[test]
    fn a_step_too_coarse_for_the_main_withholds_both() {
        let mut stepped = log(0.05, 10.0, flight(25.0, 150.0, 3.0));
        // A format that states no step of its own: the track's is measured, 10 m.
        stepped.format = LogFormat::GenericTable;
        let readings = read_all(&stepped);
        assert!(readings.landing.value().is_some(), "{:?}", readings.landing);
        assert_eq!(readings.main_descent.reason(), Some(Reason::StepTooCoarse));
        assert_eq!(
            readings.drogue_descent.reason(),
            Some(Reason::StepTooCoarse)
        );
    }
}
