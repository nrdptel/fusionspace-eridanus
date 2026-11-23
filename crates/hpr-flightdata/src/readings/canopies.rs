//! The descent under each canopy ([M7.2c][m7-2c], the rest of the readings): the main's opening
//! found back from landing as Debrief finds it, then the rate of each leg.
//!
//! **The main's opening.** The rate of descent is the least-squares slope of the filtered track
//! over [`DESCENT_WINDOW_S`]. The main's rate is its median over the last [`DESCENT_TAIL_S`]
//! before landing, or the fit's span where that is longer, the last half of [`DESCENT_GUARD_S`]
//! left out. The mark is [`MAIN_TOLERANCE_SHARE`] of it, or [`MAIN_TOLERANCE_M_S`] more if that is
//! more: a rate within the mark is slow, the main's, and one past it fast. Where the descent's
//! rates go from fast to slow once, the samples back from landing to the first fast one are the
//! main's descent, and the first of them the opening: Debrief's walk. Where they change more often,
//! the main's descent starts at the likeliest place for one change in how often the rate is
//! slow ([`likeliest_split`], [ADR-366][adr-366]): a fast stretch under the main, a swing or a
//! coarse log's scatter, is weighed against the slow rates it would put in the drogue's leg.
//! Debrief's walk stopped at the first of them, however short. The rates are their running
//! median over
//! [`DESCENT_TAIL_S`], or twice the fit where that is longer, in samples at the descent's median
//! interval, held to no less than [`MIN_INTERVAL_S`] less a part in a thousand, and never more
//! than it holds, so a spike of
//! the fit's noise on a long leg is taken out, as a step, the opening, stays where it is under a
//! median. Every rate is
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
//! in it: the synthetic flights open each canopy at once. Where the rates go from fast to slow
//! once, the split is the walk's start and that is the bound. Where they change more often, the
//! split can be earlier than the walk's start, and either can be the opening: a swing under the
//! main makes the walk's start late
//! ([#903](https://github.com/nrdptel/fusionspace-eridanus/issues/903)), and a drogue slow for a
//! while near the mark makes the split early
//! ([#1593](https://github.com/nrdptel/fusionspace-eridanus/issues/1593)). The bound then adds the
//! time between them, so it holds an opening at either, with the coarser of the two places'
//! sample intervals as its margin ([ADR-374][adr-374]). The drogue's leg ends at the split, so
//! it leaves out the stretch between them: where that stretch falls slower than the leg, as in
//! #1593's shape, the drogue's rate reads fast, and where it falls faster, slow.
//!
//! **Each leg's rate** is the filtered height it loses over the time it takes: the drogue's from
//! apogee to the main's opening, the main's from there to landing, or from apogee when the descent
//! shows one canopy. A leg must lose more than [`LEG_LEAST_M`] and [`LEG_LEAST_SHARE`] of its
//! top's height, as Debrief asks. Its bound is the error two heights of the same track can hold
//! between them, over the leg's time.
//!
//! [m7-2c]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c
//! [adr-366]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0366-the-mains-opening-at-the-likeliest-split.md
//! [adr-374]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0374-the-mains-opening-bound-holds-the-split-and-the-walk.md

use super::interval::{self, Interval, Shares};
use super::{
    Apex, BAROMETER_GAIN, DENSITY_SCALE_HEIGHT_M, DESCENT_FIT_STEPS, DESCENT_GUARD_S,
    DESCENT_TAIL_S, DESCENT_WINDOW_MAX_S, DESCENT_WINDOW_S, DRIFT_M_PER_S, DROGUE_RATIO, Descent,
    LEG_LEAST_M, LEG_LEAST_SHARE, MAIN_TOLERANCE_M_S, MAIN_TOLERANCE_SHARE, MIN_INTERVAL_S, Method,
    NOISE_M, Opening, Provenance, Reading, Reason, STANDARD_GRAVITY_MPS2, TEMPERATURE_SHARE, Track,
    local_interval, median, slope_over,
};
use crate::filter::running_median;

/// Where the main's descent starts among a descent's rates, `slow`, in the order of time: each
/// is whether the rate there is no faster than the main's mark. It is the likeliest place for
/// one change in how often the rate is slow, the change-point of a two-valued sequence (D. V.
/// Hinkley and E. A. Hinkley, "Inference about the change-point in a sequence of binomial
/// variables", *Biometrika* 57, 1970, pp. 477–488).
///
/// Were the rates independent, slow with one chance before the split `s` and another after it,
/// the likelihood of the `n` rates would be greatest, over the two chances, at each side's own
/// share of slow rates. Its logarithm is then
///
/// ```text
/// L(s) = l(a, s) + l(c, n − s),    l(k, m) = k ln(k/m) + (m − k) ln((m − k)/m)
/// ```
///
/// with `a` slow rates before the split and `c` after it, and `0 ln 0 = 0`. A side all slow or
/// all fast has `l = 0`, its greatest. The split is where `L` is greatest among those whose
/// later side holds the larger share of slow rates, since the main is the slower canopy. Of two
/// as likely, it is the one nearer landing. Where no split is likelier than no change at all,
/// the whole descent is the main's if at least half its rates are slow (the answer is 0), and
/// none of it otherwise (`slow.len()`).
///
/// Where the rates go from fast to slow once, after any slow ones of the start from rest at
/// apogee, the split is the first rate back from landing past the mark, the walk Debrief makes
/// (a test tries every such descent of its sizes). Where the rate passes the mark again under
/// the main, as a swing under the canopy or a coarse log's scatter makes it, the walk stopped
/// at the first of them, however short (#903); here a fast stretch among the main's slow rates
/// is weighed against the slow rates before it, which the later split would put among the
/// drogue's fast ones. Which costs more depends on their lengths and on the drogue's leg. The
/// split is never later than the walk's: every rate after the walk's start is slow, and adding
/// slow rates to an earlier side that holds a fast one only lowers `l` there.
///
/// The rates aren't independent, a running median of overlapping fits, so the likelihood is the
/// rule's reason and no stated confidence.
fn likeliest_split(slow: &[bool]) -> usize {
    let n = slow.len();
    let all_slow = slow.iter().filter(|slow| **slow).count();
    // No change at all: one share of slow rates for the whole descent.
    let (mut best, mut most) = (if 2 * all_slow >= n { 0 } else { n }, side(all_slow, n));
    let mut slow_after = 0_usize;
    // Back from landing, so that of two splits as likely the one nearer landing is kept.
    for split in (1..n).rev() {
        if slow[split] {
            slow_after += 1;
        }
        let (after, slow_before) = (n - split, all_slow - slow_after);
        // The later side is the main's, the slower: slow_after / after ≥ slow_before / split.
        // The products pass 32 bits from 131,072 rates on, so they are taken wider.
        if wide(slow_after) * wide(split) < wide(slow_before) * wide(after) {
            continue;
        }
        let likelihood = side(slow_before, split) + side(slow_after, after);
        // Likelier by more than the logarithm's rounding, which differs between platforms.
        if likelihood > most + SPLIT_TIE * (1.0 + most.abs()) {
            (best, most) = (split, likelihood);
        }
    }
    best
}

/// Where Debrief's walk back from landing starts the main's descent among a descent's rates,
/// `slow`: the first of the slow rates after the last fast one, or `slow.len()` where the last
/// rate is fast. Every rate from it on is slow, so [`likeliest_split`] is never later.
fn walk_start(slow: &[bool]) -> usize {
    slow.len() - slow.iter().rev().take_while(|slow| **slow).count()
}

/// A count as an integer wide enough for the product of two.
const fn wide(count: usize) -> u128 {
    count as u128
}

/// Two splits' log-likelihoods closer than this share of their size are as likely: the
/// logarithm's last bits differ between platforms, and a tie must fall the same way on each.
const SPLIT_TIE: f64 = 1e-9;

/// One side's log-likelihood at its own share of slow rates, `l(k, m)` of [`likeliest_split`]:
/// `k ln k + (m − k) ln(m − k) − m ln m`.
fn side(slow: usize, rates: usize) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of samples converts exactly"
    )]
    let x_ln_x = |count: usize| {
        if count == 0 {
            0.0
        } else {
            let x = count as f64;
            x * x.ln()
        }
    };
    x_ln_x(slow) + x_ln_x(rates - slow) - x_ln_x(rates)
}

/// The half window, in samples, of the running median the descent's rates are walked back on: the
/// median spans [`DESCENT_TAIL_S`], or twice the fit's `window_s` where that is longer, so each
/// side is half of that, in samples at the descent's median `interval`, so that a few samples
/// bunched together don't widen it; never more than the descent's `samples`, nor less than one. The
/// interval is held to the shortest the readings accept, [`MIN_INTERVAL_S`] less a part in a
/// thousand, so that a steady log they read keeps its window: a burst of samples making most of the
/// descent's would otherwise widen the window to the whole descent, and each step of the median
/// would cost the descent's length (#1250).
fn median_half(interval: f64, window_s: f64, samples: usize) -> usize {
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of samples converts exactly"
    )]
    let samples = samples as f64;
    let half = if interval > 0.0 {
        (0.5 * DESCENT_TAIL_S.max(2.0 * window_s) / interval.max(MIN_INTERVAL_S * (1.0 - 1e-3)))
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
    let half = half as usize;
    half.max(1)
}

/// The leg from sample `from`, at `from_s`, to sample `to`, or why it is too short to read.
struct Leg {
    from: usize,
    from_s: f64,
    to: usize,
    opening: Option<Opening>,
    shares: Shares,
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
                 {near_ground:.1} m/s. A fit of its rate would span {window_s:.1} s, more than the \
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
    let mut intervals: Vec<f64> = (first..landing).map(|k| time[k + 1] - time[k]).collect();
    let interval = median(&mut intervals).unwrap_or(0.0);
    let smoothed = running_median(&rates, median_half(interval, window_s, rates.len()));
    let steady = |k: usize| {
        smoothed
            .get(k - first)
            .is_some_and(|v| v.is_finite() && *v <= mark)
    };
    let last = tail.last().copied().unwrap_or(landing - 1);
    let slow: Vec<bool> = (first..=last).map(steady).collect();
    let start = first + likeliest_split(&slow);
    // Debrief's walk starts the main no sooner than the split. Where it starts it later, either
    // may be the opening (#1593): a swing under the main makes the walk's start late, a drogue
    // slow for a while near the main's mark makes the split early.
    let walked = (first + walk_start(&slow)).min(landing);
    let walked_s = time[walked];
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
             {mark:.1} m/s up to the main's opening. That mark is {MAIN_TOLERANCE_SHARE} times the \
             rate near the ground, {main_rate:.1} m/s, or {MAIN_TOLERANCE_M_S} m/s more if that \
             is more. A drogue's leg would also fall on median faster than {DROGUE_RATIO} times \
             the rate near the ground. So the rocket came down under one canopy, or under a drogue no faster \
             than the mark, which reads as one. The main's descent is then the whole descent's, \
             from apogee"
        );
        return (
            Reading::withheld(Reason::OneCanopy, detail),
            leg(Leg {
                from: apex.index,
                from_s: apex.time_s,
                to: landing,
                opening: None,
                shares: interval::MAIN_RATE_SHARES,
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
    // The bound about one place: the slowing time is the split's, the higher and so the longer
    // (or the time since apogee, which holds the walk's start too); the sampling margin is the
    // coarser of the two places'. Where the walk starts later, the time between them is added, so
    // the bound holds an opening at either. The walk is never earlier than the split, as
    // `likeliest_split` proves; the floor at zero keeps a later rule from shrinking the bound.
    debug_assert!(walked >= start, "the walk starts before the split");
    let sampling_s = local_interval(time, start).max(local_interval(time, walked));
    let about_one_s = slowing_s + 0.5 * window_s + 2.0 * sampling_s;
    let between_s = (walked_s - opened_s).max(0.0);
    // The 90% interval's shares were fitted on the bound about one place, which puts it wholly
    // before that place's reading. So it is taken about the split and about the walk's start,
    // and runs from the first's lower end to the second's upper end.
    let shares = interval::MAIN_OPENING_SHARES;
    let interval_s = shares
        .interval(opened_s, about_one_s)
        .zip(shares.interval(walked_s, about_one_s))
        .map(|(at_split, at_walk)| Interval {
            low: at_split.low,
            high: at_walk.high,
        });
    let opening = Opening {
        time_s: opened_s,
        bound_s: about_one_s + between_s,
        interval_s,
        height_m: height,
    };
    (
        leg(Leg {
            from: apex.index,
            from_s: apex.time_s,
            to: start,
            opening: None,
            shares: interval::DROGUE_RATE_SHARES,
        }),
        leg(Leg {
            from: start,
            from_s: opened_s,
            to: landing,
            opening: Some(opening),
            shares: interval::MAIN_RATE_SHARES,
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
    let (rate_m_s, bound_m_s) = (drop / span, bound_m / span);
    Reading::Read(Descent {
        rate_m_s,
        bound_m_s,
        interval_m_s: leg.shares.interval(rate_m_s, bound_m_s),
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

    /// The descent rates' median spans the time it names at a steady log's own interval, a clock
    /// half a part in a thousand fast included, and at a burst's interval no more samples than a
    /// steady log at the fastest rate the readings accept would give it (#1250).
    #[test]
    fn the_median_window_is_held_at_a_bursts_interval() {
        // A 1 s fit: the median spans 2 s, so 1 s either side, 500 samples at 500 a second.
        assert_eq!(median_half(0.002, DESCENT_WINDOW_S, 1_000_000), 500);
        // A clock half a part in a thousand fast keeps its own window: a 2.5 s fit's 2.5 s either
        // side at 0.001999 s is 1,251 samples, where 0.002 s would give 1,250.
        assert_eq!(median_half(0.001_999, 2.5, 1_000_000), 1_251);
        // A burst at a microsecond: 1 s either side over the shortest interval read, 0.001998 s.
        assert_eq!(median_half(1e-6, DESCENT_WINDOW_S, 1_000_000), 501);
        // Never more than the descent, nor less than one.
        assert_eq!(median_half(0.05, DESCENT_WINDOW_S, 7), 7);
        assert_eq!(median_half(0.0, DESCENT_WINDOW_S, 0), 1);
    }

    fn rates(runs: &[(bool, usize)]) -> Vec<bool> {
        runs.iter()
            .flat_map(|(slow, count)| std::iter::repeat_n(*slow, *count))
            .collect()
    }

    /// The split by itself. One change is found where the walk back from landing found it. A
    /// fast moment under the main moves nothing, and a slow one in the drogue's leg nothing.
    #[test]
    fn the_split_is_the_likeliest_single_change() {
        let (fast, slow) = (false, true);
        assert_eq!(likeliest_split(&[]), 0);
        // One canopy: the whole descent. No main near the ground: none of it.
        assert_eq!(likeliest_split(&rates(&[(slow, 20)])), 0);
        assert_eq!(likeliest_split(&rates(&[(fast, 20)])), 20);
        // A drogue, then a main: where the rate changes.
        assert_eq!(likeliest_split(&rates(&[(fast, 5), (slow, 10)])), 5);
        // The rocket slow at apogee, the drogue, the main; and the drogue's leg the shorter.
        assert_eq!(
            likeliest_split(&rates(&[(slow, 2), (fast, 5), (slow, 10)])),
            7
        );
        assert_eq!(
            likeliest_split(&rates(&[(slow, 4), (fast, 3), (slow, 5)])),
            7
        );
        // A fast moment under the main: the walk back from landing stopped at it, at 16.
        assert_eq!(
            likeliest_split(&rates(&[(fast, 5), (slow, 10), (fast, 1), (slow, 10)])),
            5
        );
        // A slow moment in the drogue's leg.
        assert_eq!(
            likeliest_split(&rates(&[(fast, 10), (slow, 1), (fast, 10), (slow, 10)])),
            21
        );
        // A change the wrong way, slow then fast, is no main's opening: none of it is the main's.
        assert_eq!(likeliest_split(&rates(&[(slow, 3), (fast, 10)])), 13);
        // And with most of it slow, all of it.
        assert_eq!(likeliest_split(&rates(&[(slow, 10), (fast, 3)])), 0);
    }

    /// Cases by hand. The main opens, falls fast again for a moment and settles: F20 s4 F6 s20.
    /// After the fast moment (30), `L = l(4, 30) + l(20, 20)`
    /// `= 4 ln(4/30) + 26 ln(26/30) + 0 = −11.780`. At the first slowing (20),
    /// `L = 0 + l(24, 30) = 24 ln(24/30) + 6 ln(6/30) = −15.012`. The fast moment, 6 rates, is
    /// longer than the 4 slow ones before it, and the later split is the likelier. With the slow
    /// stretch at 12, F20 s12 F6 s20, the earlier is: `l(32, 38) = 32 ln(32/38) + 6 ln(6/38)`
    /// `= −16.574` against `l(12, 38) = 12 ln(12/38) + 26 ln(26/38) = −23.699`.
    ///
    /// After a long drogue's leg a fast moment longer than the slow rates before it doesn't
    /// move the split either, the shape of #903's flight: F100 s8 F10 s30 splits at 100, with
    /// `l(38, 48) = −24.564` against `l(8, 118) = −29.252` at 118. Counting the rates on the
    /// wrong side of each split would take the later: 8 against 10.
    ///
    /// Two splits as likely: F3 s3 F3 s3 at 3, `0 + l(6, 9)`, and at 9, `l(3, 9) + 0`, each
    /// `3 ln(3/9) + 6 ln(6/9) = −5.729`. The one nearer landing is kept.
    #[test]
    fn a_bounce_moves_the_split_only_if_the_slow_stretch_before_it_is_short() {
        let (fast, slow) = (false, true);
        let l = |k: f64, m: f64| k * (k / m).ln() + (m - k) * ((m - k) / m).ln();
        assert!((l(4.0, 30.0) - -11.780).abs() < 5e-4);
        assert!((l(24.0, 30.0) - -15.012).abs() < 5e-4);
        assert!((l(32.0, 38.0) - -16.574).abs() < 5e-4);
        assert!((l(12.0, 38.0) - -23.699).abs() < 5e-4);
        assert!((l(38.0, 48.0) - -24.564).abs() < 5e-4);
        assert!((l(8.0, 118.0) - -29.252).abs() < 5e-4);
        assert_eq!(
            likeliest_split(&rates(&[(fast, 100), (slow, 8), (fast, 10), (slow, 30)])),
            100
        );
        assert!((side(4, 30) - l(4.0, 30.0)).abs() < 1e-12);
        assert!((side(32, 38) - l(32.0, 38.0)).abs() < 1e-12);
        assert_eq!(
            likeliest_split(&rates(&[(fast, 20), (slow, 4), (fast, 6), (slow, 20)])),
            30
        );
        assert_eq!(
            likeliest_split(&rates(&[(fast, 20), (slow, 12), (fast, 6), (slow, 20)])),
            20
        );
        assert!((l(6.0, 9.0) - -5.729).abs() < 5e-4);
        assert!((l(3.0, 9.0) - -5.729).abs() < 5e-4);
        assert_eq!(
            likeliest_split(&rates(&[(fast, 3), (slow, 3), (fast, 3), (slow, 3)])),
            9
        );
    }

    /// Slow at apogee, fast under the drogue, slow under the main: for every such descent of up
    /// to 19, 40 and 40 rates the split is the walk's, the first rate back from landing past the
    /// mark.
    #[test]
    fn one_change_from_fast_to_slow_splits_where_the_walk_did() {
        for start in 0..20 {
            for drogue in 1..=40 {
                for main in 1..=40 {
                    let made = rates(&[(true, start), (false, drogue), (true, main)]);
                    assert_eq!(
                        likeliest_split(&made),
                        start + drogue,
                        "{start} {drogue} {main}"
                    );
                }
            }
        }
    }

    /// A descent long enough for the products of its counts to pass 32 bits, as they would wrap
    /// on a 32-bit target: 70,000 fast rates and 70,000 slow split at 70,000.
    #[test]
    fn a_long_descents_counts_do_not_wrap() {
        let made = rates(&[(false, 70_000), (true, 70_000)]);
        assert_eq!(likeliest_split(&made), 70_000);
        assert!(wide(70_000) * wide(70_000) > u128::from(u32::MAX));
    }

    /// The split against every split tried in turn with the likelihood written out, over 3,000
    /// invented descents of 60 to 71 rates in runs of 1 to 12. None is later than the walk back
    /// from landing.
    #[test]
    fn the_split_is_the_greatest_likelihood_of_every_split_tried() {
        let l = |k: usize, m: usize| {
            let term = |count: usize| {
                if count == 0 {
                    0.0
                } else {
                    let (c, m) = (
                        f64::from(u32::try_from(count).unwrap()),
                        f64::from(u32::try_from(m).unwrap()),
                    );
                    c * (c / m).ln()
                }
            };
            term(k) + term(m - k)
        };
        let mut state = 0x2545_f491_4f6c_dd1d_u64;
        let mut draw = |below: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            usize::try_from(state % below).unwrap()
        };
        let mut moved = 0;
        for _ in 0..3_000 {
            let mut made = Vec::new();
            let mut slow = draw(2) == 0;
            while made.len() < 60 {
                made.extend(std::iter::repeat_n(slow, 1 + draw(12)));
                slow = !slow;
            }
            let n = made.len();
            let count = |from: usize, to: usize| made[from..to].iter().filter(|s| **s).count();
            let all = count(0, n);
            let (mut best, mut most) = (if 2 * all >= n { 0 } else { n }, l(all, n));
            for split in (1..n).rev() {
                let (before, after) = (count(0, split), count(split, n));
                if after * split < before * (n - split) {
                    continue;
                }
                let likelihood = l(before, split) + l(after, n - split);
                if likelihood > most + 1e-9 * (1.0 + most.abs()) {
                    (best, most) = (split, likelihood);
                }
            }
            assert_eq!(likeliest_split(&made), best, "{made:?}");
            // Never later than the walk back from landing.
            let walk = n - made.iter().rev().take_while(|s| **s).count();
            assert!(best <= walk, "{made:?}");
            moved += usize::from(best != walk);
        }
        assert!(moved > 300, "{moved}");
    }

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
        // Each states the 90% interval its own shares make; no two share theirs.
        assert_eq!(
            drogue.interval_m_s,
            interval::DROGUE_RATE_SHARES.interval(drogue.rate_m_s, drogue.bound_m_s)
        );
        assert_eq!(
            main.interval_m_s,
            interval::MAIN_RATE_SHARES.interval(main.rate_m_s, main.bound_m_s)
        );
        assert_eq!(
            opening.interval_s,
            interval::MAIN_OPENING_SHARES.interval(opening.time_s, opening.bound_s)
        );
        assert_ne!(interval::DROGUE_RATE_SHARES, interval::MAIN_RATE_SHARES);
        assert!(drogue.interval_m_s.is_some() && main.interval_m_s.is_some());
        // The opening is read late, so its interval lies wholly before the reading.
        assert!(opening.interval_s.unwrap().high < opening.time_s);
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

    /// A drogue at 15 m/s over a main at 10 m/s, no faster than the main's mark, 16 m/s, 1.6 times
    /// its rate, reads as one canopy, though Debrief's ratio of 1.4 alone would tell them apart:
    /// 15 m/s is 1.5 times the main's.
    #[test]
    fn close_rates_are_one_canopy() {
        let readings = read_all(&log(0.05, 0.0, flight(15.0, 150.0, 10.0)));
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

    /// A climb at 4 g for 5 s (2,452 m at apogee, 26 s in), then down at `drogue` m/s to
    /// `main_m`, and at `main` m/s to the pad, with `under_main(seconds since the opening)` m
    /// added to the height under the main.
    fn high_flight(
        drogue: f64,
        main_m: f64,
        main: f64,
        under_main: impl Fn(f64) -> f64,
    ) -> impl Fn(f64) -> f64 {
        move |t| {
            let top = 10.0 * G * 25.0;
            if t <= 1.0 {
                0.0
            } else if t < 6.0 {
                2.0 * G * (t - 1.0).powi(2)
            } else if t < 26.0 {
                let c = t - 6.0;
                50.0 * G + 20.0 * G * c - 0.5 * G * c * c
            } else {
                let opened = 26.0 + (top - main_m) / drogue;
                let h = if t < opened {
                    top - drogue * (t - 26.0)
                } else {
                    main_m - main * (t - opened) + under_main(t - opened)
                };
                h.max(0.0)
            }
        }
    }

    /// When [`high_flight`]'s main opens, s.
    fn high_opening(drogue: f64, main_m: f64) -> f64 {
        26.0 + (10.0 * G * 25.0 - main_m) / drogue
    }

    /// [`log`], to `end_s`.
    fn long_log(dt: f64, step_m: f64, end_s: f64, flight: impl Fn(f64) -> f64) -> FlightLog {
        let mut made = log(dt, step_m, |_| 0.0);
        made.time_s = (0..)
            .map(|i| f64::from(i) * dt)
            .take_while(|t| *t <= end_s + 1e-9)
            .collect();
        made.altitude_m = made
            .time_s
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
        made
    }

    /// #903, on a coarse and noisy log: at 5 Hz in 0.93 m steps, the size of 10 Pa on the tuning
    /// flight's log that showed it, under
    /// a barometer's noise of about 1 m, a one-second fit holds five samples and its rate
    /// scatters by 1.6 m/s. A rate past the mark for a moment, far down the main's leg, is not
    /// the main's opening. Over a sweep of the main's rate and of the noise, 732 logs, each
    /// reads two canopies, with each leg's rate and the opening inside its bound. How the walk
    /// back from landing read them is in ADR-366.
    #[test]
    fn a_coarse_logs_scatter_under_the_main_is_not_its_opening() {
        let opened = high_opening(18.0, 750.0);
        let mut logs = 0;
        let mut missed = Vec::new();
        for hundredths in (400..=700).step_by(5) {
            for seed in 0..12 {
                let main = f64::from(hundredths) / 100.0;
                let end_s = opened + 750.0 / main + 20.0;
                // The noise, the same on every run: even within 1.7 m either way (a standard
                // deviation of 0.98 m), a sample's own by its place in the log and the seed.
                let noise = |since: f64| {
                    let sample = (since / 0.2).round() + 1000.0 * f64::from(seed);
                    3.4 * ((sample * 12.9898).sin() * 43_758.545_3).rem_euclid(1.0) - 1.7
                };
                let flown = high_flight(18.0, 750.0, main, noise);
                let readings = read_all(&long_log(0.2, 0.93, end_s, flown));
                let held = match (
                    readings.drogue_descent.value(),
                    readings.main_descent.value(),
                ) {
                    (Some(drogue), Some(leg)) => leg.opening.as_ref().is_some_and(|opening| {
                        (opening.time_s - opened).abs() <= opening.bound_s
                            && (leg.rate_m_s - main).abs() <= leg.bound_m_s
                            && (drogue.rate_m_s - 18.0).abs() <= drogue.bound_m_s
                    }),
                    _ => false,
                };
                logs += 1;
                if !held {
                    missed.push((main, seed, readings.drogue_descent.reason()));
                }
            }
        }
        assert_eq!(logs, 732);
        assert!(missed.is_empty(), "missed (rate, seed, reason): {missed:?}");
    }

    /// #1593's sequence: slow at apogee, the drogue, a slow stretch near the main's mark, a short
    /// fast one, then the main. The walk back from landing starts the main at 355, where it
    /// opened; the likeliest split is 230, before the slow stretch, 125 rates early.
    #[test]
    fn a_split_before_the_walks_start_is_the_slow_stretchs_start() {
        let (fast, slow) = (false, true);
        let made = rates(&[
            (slow, 30),
            (fast, 200),
            (slow, 100),
            (fast, 25),
            (slow, 1000),
        ]);
        assert_eq!(walk_start(&made), 355);
        assert_eq!(likeliest_split(&made), 230);
        // The walk's start at the edges: none, all slow, the last rate fast, one fast moment.
        assert_eq!(walk_start(&[]), 0);
        assert_eq!(walk_start(&rates(&[(slow, 5)])), 0);
        assert_eq!(walk_start(&rates(&[(slow, 5), (fast, 1)])), 6);
        assert_eq!(walk_start(&rates(&[(slow, 3), (fast, 1), (slow, 4)])), 4);
    }

    /// The split is never later than the walk's start, which `read` asserts: on every sequence of
    /// up to 12 rates.
    #[test]
    fn the_split_is_never_after_the_walks_start() {
        for n in 0..=12_usize {
            for bits in 0..(1_u32 << n) {
                let slow: Vec<bool> = (0..n).map(|k| bits >> k & 1 == 1).collect();
                assert!(likeliest_split(&slow) <= walk_start(&slow), "{slow:?}");
            }
        }
    }

    /// #1593's shape in a flight: the drogue at 18 m/s, then 8 s at 6 m/s, under the main's mark,
    /// then 3 s at 18 m/s again before the main opens at 750 m. The split falls at the slow
    /// stretch's start, about 11 s early, and the walk at the opening: the opening lies within
    /// its bound, which holds both, and the drogue's rate reads fast.
    #[test]
    fn a_slow_stretch_in_the_drogues_leg_keeps_the_opening_in_its_bound() {
        let top = 10.0 * G * 25.0;
        // The drogue's leg: 18 m/s to 1,000 m above the main, 8 s at 6 m/s, then 18 m/s again.
        let slow_from = 26.0 + (top - 750.0 - 102.0) / 18.0;
        let slow_to = slow_from + 8.0;
        let opened = slow_to + 3.0;
        let flight = move |t: f64| -> f64 {
            let h = if t <= 1.0 {
                0.0
            } else if t < 6.0 {
                2.0 * G * (t - 1.0).powi(2)
            } else if t < 26.0 {
                let c = t - 6.0;
                50.0 * G + 20.0 * G * c - 0.5 * G * c * c
            } else if t < slow_from {
                top - 18.0 * (t - 26.0)
            } else if t < slow_to {
                750.0 + 102.0 - 6.0 * (t - slow_from)
            } else if t < opened {
                750.0 + 54.0 - 18.0 * (t - slow_to)
            } else {
                750.0 - 5.0 * (t - opened)
            };
            h.max(0.0)
        };
        let end_s = opened + 750.0 / 5.0 + 20.0;
        for (dt, step_m) in [(0.05, 0.0), (0.05, 0.3048), (0.2, 0.0)] {
            let readings = read_all(&long_log(dt, step_m, end_s, flight));
            let main = readings.main_descent.value().unwrap();
            let opening = main.opening.as_ref().unwrap();
            assert!(
                (opening.time_s - opened).abs() <= opening.bound_s,
                "{dt} s, {step_m} m: {opening:?} against {opened}"
            );
            // The split reads the slow stretch's start, and the bound reaches the opening by no
            // more than a bound about one place, 2 s here, past it.
            assert!(
                (opening.time_s - slow_from).abs() < 1.0,
                "{dt} s, {step_m} m: {opening:?}"
            );
            assert!(
                opening.time_s + opening.bound_s < opened + 2.0,
                "{dt} s, {step_m} m: {opening:?} against {opened}"
            );
            // The 90% interval runs from before the split to the walk's start, the opening.
            let interval = opening.interval_s.unwrap();
            assert!(
                interval.low < opening.time_s,
                "{dt} s, {step_m} m: {opening:?}"
            );
            assert!(
                (interval.high - opened).abs() < 1.0,
                "{dt} s, {step_m} m: {opening:?} against {opened}"
            );
            // The drogue's leg ends at the split, before the slow stretch, so its rate leaves
            // that stretch out and reads faster than the leg's own, apogee to the opening: the
            // side a descent's energy errs high on, not the flattering one.
            let drogue = readings.drogue_descent.value().unwrap();
            let leg_m_s = (top - 750.0) / (opened - 26.0);
            assert!(
                drogue.rate_m_s > leg_m_s + 0.5,
                "{dt} s, {step_m} m: {drogue:?} against {leg_m_s}"
            );
        }
    }

    /// #903's shape: under the open main the rocket holds its height for 2.25 s and then falls
    /// 4.5 m/s faster than the main's rate for 2.5 s, a swing past the mark 40 s after the
    /// opening. The opening stays where the drogue's leg ends, and each leg's rate in its bound.
    #[test]
    fn a_swing_under_the_main_is_not_its_opening() {
        let opened = high_opening(18.0, 750.0);
        let swing = |since: f64| {
            let t = since - 40.0;
            if t <= 0.0 {
                0.0
            } else if t < 2.25 {
                5.0 * t
            } else if t < 4.75 {
                11.25 - 4.5 * (t - 2.25)
            } else {
                0.0
            }
        };
        let end_s = opened + 750.0 / 5.0 + 20.0;
        for (dt, step_m) in [(0.05, 0.0), (0.05, 0.3048), (0.2, 0.0)] {
            let readings = read_all(&long_log(
                dt,
                step_m,
                end_s,
                high_flight(18.0, 750.0, 5.0, swing),
            ));
            let drogue = readings.drogue_descent.value().unwrap();
            let main = readings.main_descent.value().unwrap();
            let opening = main.opening.as_ref().unwrap();
            assert!(
                (opening.time_s - opened).abs() <= opening.bound_s,
                "{dt} s, {step_m} m: {opening:?} against {opened}"
            );
            assert!(
                (drogue.rate_m_s - 18.0).abs() <= drogue.bound_m_s,
                "{dt} s, {step_m} m: {drogue:?}"
            );
            assert!(
                (main.rate_m_s - 5.0).abs() <= main.bound_m_s,
                "{dt} s, {step_m} m: {main:?}"
            );
        }
    }
}
