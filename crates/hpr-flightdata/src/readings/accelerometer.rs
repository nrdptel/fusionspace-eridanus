//! The readings of the axial accelerometer: the top acceleration and burnout ([M7.2c][m7-2c],
//! the rest of the readings), and burnout from the speed's peak where the accelerometer gives
//! none.
//!
//! **The channel.** Only an accelerometer along the rocket's axis is read, as its family names it
//! ([`family`]): a board's three axes don't say which lies along the axis, and their magnitude
//! never falls below zero at burnout. A sample that isn't a number is left out, and a clock faster
//! than [`MIN_INTERVAL_S`], the fastest the bounds were checked at, withholds both readings
//! ([`Reason::SampledTooFast`]).
//!
//! **The thrust, on a cleaned trace.** Where the thrust starts and ends, and burnout, are found on
//! the trace cleaned by a Hampel filter over [`SPIKE_WINDOW_S`] at [`SPIKE_THRESHOLD`]
//! ([`hampel`]), which takes out a spike narrower than half its window, a charge's shock or one
//! bad sample, and keeps a step or a steady run as it was. The readings are turned over where the
//! thrust reads negative: the first span of the cleaned trace, either way up, from liftoff's
//! earliest time, that holds past [`LIFTOFF_ACCELERATION_M_S2`] for [`BURN_LEAST_S`], found as
//! the burns are (below), or, where none holds that long, every such span, if they all go one
//! way ([`thrust_sign`]). A pad reads 1 g or 0 either way up, and a rocket leaves it at 3 g or
//! more, so that span is the thrust's, as a part mounted nose down writes it, while a knock on
//! the pad, either way, holds too briefly, and a canopy's shock later, however large, comes after
//! it. Spans both ways where none holds, or none, withhold both readings
//! ([`Reason::DirectionUnstated`]). Gravity's 1 g is put back where the logger took it out. Three witnesses say whether it
//! did, each within [`GRAVITY_TOLERANCE_G`]: the family's word; the pad, where about 1 g holds
//! gravity and about 0 doesn't; and the coast's last [`COAST_SPAN_S`] before apogee, where a
//! rocket falls freely but for a small drag, so about 0 holds gravity and about −1 g doesn't. The
//! pad is read off the accelerometer itself, the steady reading before the thrust lifts it by half
//! [`LIFTOFF_ACCELERATION_M_S2`], if that comes no later than liftoff's latest time by the
//! heights; a pad or a coast that reads neither, such as a log that starts in the thrust, says
//! nothing. Gravity is settled when one witness or more says and none disagrees; otherwise nothing
//! is read ([`Reason::GravityUnstated`]). A sample that repeats the time before it is left out, as
//! the primary track leaves it ([`first_at_each_time`]).
//!
//! **The top acceleration** is the highest reading over the burns: each from a cleaned reading past
//! [`LIFTOFF_ACCELERATION_M_S2`] to where the cleaned trace's mean over [`BURNOUT_MEAN_S`] next
//! falls through zero, or apogee if it doesn't, that holds past [`LIFTOFF_ACCELERATION_M_S2`] for
//! [`BURN_LEAST_S`], as a shock doesn't, or the first such span where none does; searched from
//! liftoff's earliest time by the heights, as a rocket at rest reads 1 g and anything more before
//! it moves is a knock on the pad, or from the log's start where liftoff is withheld; with no
//! burn, from there to apogee. In a burn the reading is the thrust less
//! the drag, over the mass. A charge or a canopy's shock in a coast lies outside the burns, and
//! isn't taken for the top. The top is read off the samples, not the cleaned trace. It is clipped,
//! two causes kept apart ([`Clip`]), where it reaches the part's range, as the family states it,
//! or else holds flat at its top for [`FLAT_TOP_SAMPLES`] samples or [`FLAT_TOP_S`], within
//! [`FLAT_TOP_SHARE`] of it or [`FLAT_TOP_M_S2`], as Debrief tells a part that clipped
//! (`lib/analyze/index.ts`, MIT, the project owner's own); HPR Sim narrows that to half the
//! part's noise where it is quieter, never below half a step ([`flat_top`]). A clipped top is a
//! floor, and so is one whose last burn never falls through zero before the accelerometer ends
//! or apogee: the thrust doesn't end on the channel, so the top may lie past its samples, and
//! burnout isn't read off it. Neither the top nor burnout comes before liftoff's earliest time.
//!
//! **Burnout.** In the climb an accelerometer reads `(T − D)/m`: positive while the thrust beats
//! the drag, below zero once it doesn't, and after burnout `−D/m`, which eases as the coast slows.
//! So the reading falls through zero as the thrust tails off, and is least at burnout, where the
//! thrust's fall ends and the drag's easing begins; the fall through zero, Debrief's mark, comes
//! before it, by as long as the thrust takes to tail off below the drag. The cleaned trace is
//! averaged over [`BURNOUT_MEAN_S`] in whole samples, an odd count, which cuts the noise by the
//! square root of the samples averaged and reads the least late by up to half the window. With `N`
//! five times the averaged trace's noise ([`noise`], from the top acceleration to a second past
//! the fall through zero) and a step, an averaged sample lies within `N` of its truth but about
//! once in 1.7 million, so the samples next to burnout read within `2N` of the least. The thrust
//! falls steeply and the coast's drag eases slowly, so burnout lies near the first such sample:
//! the reading is the first, less half the window, and its bound the span of those samples, an
//! interval, as a part's filter spreads a thrust cut off at once over about a sample, and the
//! window's `2h` intervals, or two intervals if that is more. The search runs from the fall
//! through zero until the averaged trace climbs `4N` above its least so far, as the coast's easing
//! drag lifts it. If it never does before apogee, or the least lies within `3N` of zero, too
//! little drag to see, the accelerometer gives no burnout; nor after more than one burn, an air
//! start's or a sustainer's, as the tuning set's flights burn once.
//!
//! **From the speed's peak.** A rocket gains speed only while the thrust beats the drag and its
//! weight, and after burnout its speed up falls at least as fast as gravity pulls it, so the top
//! speed comes at or before burnout, the last burn's end. Burnout from it is a floor: no sooner
//! than the peak's time, less the time a coast slowing at g takes to lose twice
//! [`SPEED_NOISE_M_S`], by which the fit's noise can place the peak late, half the fit and an
//! interval. How far before burnout it falls on the tuning flights is in the synthetic flights'
//! report, as the floor's errors.
//!
//! [m7-2c]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2c

use super::{
    ACCEL_BIAS_M_S2, ACCEL_NOISE_SIGMAS, ACCEL_SCALE_SHARE, Accelerometer, BURN_LEAST_S,
    BURNOUT_MEAN_S, Burnout, COAST_SPAN_S, Clip, FLAT_TOP_M_S2, FLAT_TOP_S, FLAT_TOP_SAMPLES,
    FLAT_TOP_SHARE, GRAVITY_TOLERANCE_G, Gravity, GravityBy, LIFTOFF_ACCELERATION_M_S2,
    MIN_INTERVAL_S, MaxAcceleration, Method, NOISE_SPAN_S, PAD_MIN_SAMPLES, Provenance,
    RAIL_TILT_DEG, Reading, Readings, Reason, SPEED_NOISE_M_S, SPEED_WINDOW_S, SPIKE_THRESHOLD,
    SPIKE_WINDOW_S, STANDARD_GRAVITY_MPS2, Source, Withheld, clocked, local_interval, median,
    smallest_step,
};
use crate::filter::hampel;
use crate::log::{Channel, FlightLog, LogFormat, Quantity};

/// The column the synthetic flights' loggers write their axial accelerometer under
/// (`hpr_forensics::synthetic::logger`), gravity in.
pub const SYNTHETIC_COLUMN: &str = "Axial acceleration (synthetic)";

/// The column the synthetic flights' IMU-like logger writes its axial accelerometer under, gravity
/// in, when its part is set to ±8 g (`hpr_forensics::synthetic::logger::LOWER_RANGE_G`).
pub const SYNTHETIC_LOWER_RANGE_COLUMN: &str = "Axial acceleration, ±8 g (synthetic)";

/// What a family states of its axial accelerometer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Axial {
    /// Whether its readings hold gravity; `None` where the family doesn't say.
    pub gravity: Option<Gravity>,
    /// The part's range, g; `None` where the family doesn't state one part.
    pub range_g: Option<f64>,
}

/// The axial accelerometer a family writes under `column`, or `None` if the column isn't one.
/// Each entry is the reader's own account of its family (`docs/format/flight-logs.md`) and the
/// range its maker states (`docs/research/altimeter-sensor-noise.md`).
pub(super) fn family(format: LogFormat, column: &str) -> Option<Axial> {
    use Gravity::{In, Out};
    use LogFormat as F;
    let axial = |gravity, range_g| Some(Axial { gravity, range_g });
    match (format, column) {
        // The synthetic flights' loggers: the AltOS-like part an ADXL375, clipping at its least
        // stated range, ±180 g; the IMU like Blue Raven's ±32 g.
        (F::AltosCsv, SYNTHETIC_COLUMN) => axial(Some(In), Some(180.0)),
        (F::BlueRavenCsv, SYNTHETIC_COLUMN) => axial(Some(In), Some(32.0)),
        // The same IMU set to ±8 g, as the BMX160 and the MPU-6000 can be.
        (F::BlueRavenCsv, SYNTHETIC_LOWER_RANGE_COLUMN) => axial(Some(In), Some(8.0)),
        // AltOS's `acceleration` is along the flight path, its ground reading taken out; the
        // part varies by board (MMA2202, ADXL78, MMA6555, ADXL375).
        (
            F::AltosCsv | F::AltosEeprom | F::AltosTelemetry | F::AltosTelemetryCsv,
            "acceleration",
        ) => axial(Some(Out), None),
        // Raven and Blue Raven name the axis; neither says whether gravity is in it, and each
        // has carried more than one part (Raven3 ±70 or ±250 g, Raven4 ±105 g; Blue Raven ±32 g
        // and a high-range part).
        (F::RavenFipCsv | F::RavenFipa | F::BlueRavenCsv, "Axial Accel (Gs)") => axial(None, None),
        // The Eggtimer Proton's one accelerometer, its pad's reading taken out; 120 g or 200 g by
        // board. The filtered columns (`FAccel…`) are the logger's own smoothing.
        (F::EggtimerCsv | F::EggtimerText | F::EggtimerRtf, "Accel" | "AccelG" | "AccelFPSS") => {
            axial(Some(Out), None)
        }
        // The AIM XTRA's long axis, gravity in, 100 g (the AIM BASE writes the XTRA's telemetry).
        (F::EntacoreAimCsv | F::EntacoreXtra | F::EntacoreDump, "acceleration") => {
            axial(Some(In), Some(100.0))
        }
        // Fluctus's vertical acceleration, from a ±200 g part; the file doesn't say whether
        // gravity is in it.
        (F::FluctusCsv, "vert-accel (m/s2)") => axial(None, Some(200.0)),
        // MARSA's, its rest reading taken out; the part isn't named.
        (F::Marsa, "acceleration") => axial(Some(Out), None),
        // Altimax's points down the rocket, gravity in: about −1 g at rest, turned over below.
        (F::AltimaxXls, "ACCEL [m/s2]") | (F::AltimaxAls, "ACCEL") => axial(Some(In), None),
        // The ARTS2's ADXL78, ±50 g, its pad's reading taken out.
        (F::Arts2Csv, "Acc") | (F::Arts2Odf, "Acc(HI)") => axial(Some(Out), Some(50.0)),
        // A column that names the axis, as a table's or a mapping's may: nothing else is known.
        _ if column.to_ascii_lowercase().contains("axial") => axial(None, None),
        _ => None,
    }
}

/// The channel with each sample that has no time left out, and each that repeats the time of the
/// last kept, the first at each time kept: AltOS writes rows that share a time with the row
/// before, and its primary track keeps the first, as Debrief does (`lib/flight/build.ts`). A
/// clock that runs back is left for [`clocked`] to refuse.
fn first_at_each_time(channel: &Channel) -> Channel {
    let mut kept = channel.clone();
    kept.time_s.clear();
    kept.values.clear();
    for (&t, &value) in channel.time_s.iter().zip(&channel.values) {
        if t.is_finite() && kept.time_s.last() != Some(&t) {
            kept.time_s.push(t);
            kept.values.push(value);
        }
    }
    kept
}

/// The robust noise of `values`, 1σ: the median of each sample's distance from the mean of its
/// two neighbors, over the 0.6745 a normal distribution's median absolute deviation is of its σ
/// and the √1.5 the neighbors' mean adds. A curve's bend between three samples is left out with
/// the outliers; 0 with fewer than three samples.
pub(super) fn noise(values: &[f64]) -> f64 {
    let mut distances: Vec<f64> = values
        .windows(3)
        .map(|w| (w[1] - 0.5 * (w[0] + w[2])).abs())
        .filter(|d| d.is_finite())
        .collect();
    median(&mut distances).map_or(0.0, |m| m / (0.674_489_750_196_081_7 * 1.5_f64.sqrt()))
}

/// The accelerometer's readings: the channel they came from, the top acceleration and burnout.
pub(super) fn read(
    log: &FlightLog,
    heights: &Readings,
) -> (
    Option<Accelerometer>,
    Reading<MaxAcceleration>,
    Reading<Burnout>,
) {
    let fallback = |detail: Option<String>| from_speed(heights, detail);
    let withheld = |reason, detail: String| {
        (
            None,
            Reading::withheld(reason, detail.clone()),
            fallback(Some(detail)),
        )
    };
    let accelerometers: Vec<_> = log.channels_of(Quantity::Acceleration).collect();
    if accelerometers.is_empty() {
        return (
            None,
            Reading::withheld(
                Reason::NoAccelerometer,
                "the log has no accelerometer; HPR Sim doesn't difference a height twice to make \
                 one, as its steps would read as spikes of many g",
            ),
            fallback(None),
        );
    }
    let Some((channel, stated)) = accelerometers
        .iter()
        .find_map(|channel| family(log.format, &channel.column).map(|axial| (*channel, axial)))
    else {
        let columns: Vec<String> = accelerometers
            .iter()
            .map(|channel| format!("`{}`", channel.column))
            .collect();
        return withheld(
            Reason::AxisUnstated,
            format!(
                "the log's accelerometers ({}) don't say which lies along the rocket's axis",
                columns.join(", ")
            ),
        );
    };
    let (time, raw) = match clocked(&first_at_each_time(channel)) {
        Ok(clocked) => clocked,
        Err((reason, detail)) => return withheld(reason, detail),
    };
    // A sample that isn't a number is no reading: it is left out, the samples beside it kept.
    let (time, raw): (Vec<f64>, Vec<f64>) = time
        .iter()
        .zip(&raw)
        .filter(|(_, value)| value.is_finite())
        .map(|(t, value)| (*t, *value))
        .unzip();
    let apogee_s = match &heights.apogee {
        Reading::Read(apogee) => apogee.time_s,
        Reading::Withheld(Withheld { reason, detail }) => {
            return withheld(
                *reason,
                format!(
                    "the top acceleration is searched up to apogee, which is withheld: {detail}"
                ),
            );
        }
    };
    let boost: Vec<usize> = (0..time.len()).filter(|&k| time[k] <= apogee_s).collect();
    if boost.len() < 3 {
        return withheld(
            Reason::TooShort,
            format!(
                "the accelerometer has {} samples before apogee, too few to take a reading from",
                boost.len()
            ),
        );
    }
    // The climb's interval: a logger may sample its pad or its descent at another rate.
    let mut steps: Vec<f64> = boost
        .windows(2)
        .map(|pair| time[pair[1]] - time[pair[0]])
        .collect();
    let interval = median(&mut steps).unwrap_or(0.0);
    if interval < MIN_INTERVAL_S * (1.0 - 1e-3) {
        return withheld(
            Reason::SampledTooFast,
            format!(
                "the accelerometer's samples come every {interval:.3e} s, faster than the \
                 {MIN_INTERVAL_S} s its readings were checked at"
            ),
        );
    }
    // The trace with its spikes taken out, to find the thrust's start and end and burnout on: a
    // charge's shock or one bad sample would otherwise turn the trace over or end the thrust.
    let clean = hampel(&raw, half_window(SPIKE_WINDOW_S, interval), SPIKE_THRESHOLD);
    // The thrust is searched from liftoff's earliest time by the heights, or the log's start
    // where liftoff is withheld: a rocket at rest reads 1 g, so anything more before it moves is
    // the pad's, a knock or a bump.
    let end = boost.last().copied().unwrap_or(0);
    let earliest_s = heights
        .liftoff
        .value()
        .map(|liftoff| liftoff.time_s - liftoff.bound_s);
    let from = earliest_s.map_or(0, |earliest_s| {
        time[..=end].partition_point(|t| *t < earliest_s).min(end)
    });
    let half_mean = half_window(BURNOUT_MEAN_S, interval);
    // Turned over where the thrust reads negative, the thrust found as the burns are (below), on
    // the cleaned trace as written, either way up.
    let sign = match thrust_sign(&clean, from, end, half_mean, interval) {
        Ok(sign) => sign,
        Err(detail) => {
            return withheld(
                Reason::DirectionUnstated,
                format!("the accelerometer `{}` {detail}", channel.column),
            );
        }
    };
    let g = STANDARD_GRAVITY_MPS2;
    let tolerance = GRAVITY_TOLERANCE_G * g;
    // The pad, on the accelerometer's own trace: a rocket at rest reads a steady level, which
    // the thrust lifts by [`LIFTOFF_ACCELERATION_M_S2`] or more once the rocket moves. So the pad
    // is the samples from the start until the first that reads half that above the start's
    // level, if that comes no later than liftoff's latest time by the heights.
    let pad = match &heights.liftoff {
        Reading::Read(liftoff) => {
            let mut start: Vec<f64> = raw.iter().take(PAD_MIN_SAMPLES).map(|v| sign * v).collect();
            let level = median(&mut start).unwrap_or(f64::NAN);
            let rise = (0..time.len())
                .find(|&k| sign * clean[k] - level > 0.5 * LIFTOFF_ACCELERATION_M_S2);
            match rise {
                Some(rise)
                    if rise >= PAD_MIN_SAMPLES
                        && time[rise] <= liftoff.time_s + liftoff.bound_s =>
                {
                    let mut on_pad: Vec<f64> = raw[..rise].iter().map(|v| sign * v).collect();
                    median(&mut on_pad)
                }
                _ => None,
            }
        }
        Reading::Withheld(_) => None,
    };
    // A pad within the tolerance of 1 g holds gravity, of 0 doesn't; one that reads neither,
    // such as a log that starts in the thrust, says nothing.
    let by_pad = pad.and_then(|pad| {
        if (pad - g).abs() <= tolerance {
            Some(Gravity::In)
        } else if pad.abs() <= tolerance {
            Some(Gravity::Out)
        } else {
            None
        }
    });
    // The coast's last [`COAST_SPAN_S`] before apogee's earliest time: about 0 holds gravity,
    // about −1 g doesn't; still under thrust, or anything else, it says nothing.
    let coast = match &heights.apogee {
        Reading::Read(apogee) => {
            let end = apogee.time_s - apogee.time_bound_s;
            let mut readings: Vec<f64> = boost
                .iter()
                .filter(|&&k| time[k] >= end - COAST_SPAN_S && time[k] <= end)
                .map(|&k| sign * raw[k])
                .collect();
            if readings.len() >= PAD_MIN_SAMPLES {
                median(&mut readings)
            } else {
                None
            }
        }
        Reading::Withheld(_) => None,
    };
    let by_coast = coast.and_then(|coast| {
        if coast.abs() <= tolerance {
            Some(Gravity::In)
        } else if (coast + g).abs() <= tolerance {
            Some(Gravity::Out)
        } else {
            None
        }
    });
    let witnesses = [
        (stated.gravity, GravityBy::Family),
        (by_pad, GravityBy::Pad),
        (by_coast, GravityBy::Coast),
    ];
    let said: Vec<(Gravity, GravityBy)> = witnesses
        .iter()
        .filter_map(|(word, by)| word.map(|word| (word, *by)))
        .collect();
    let gravity = match said.first() {
        Some(&first) if said.iter().all(|(word, _)| *word == first.0) => first,
        _ => {
            let family = match stated.gravity {
                Some(Gravity::In) => "says its readings hold gravity",
                Some(Gravity::Out) => "says gravity is taken out of its readings",
                None => "doesn't say whether its readings hold gravity",
            };
            let level = |name: &str, level: Option<f64>, held: &str, out: &str| {
                level.map_or_else(
                    || format!("the log holds no {name} to tell by"),
                    |level| {
                        format!(
                            "its {name} reads {:.2} g ({held} holds gravity, {out} doesn't)",
                            level / g
                        )
                    },
                )
            };
            return withheld(
                Reason::GravityUnstated,
                format!(
                    "the accelerometer `{}`'s family {family}; {}; {}: within \
                     {GRAVITY_TOLERANCE_G} g, they don't settle it, or disagree",
                    channel.column,
                    level("pad", pad, "1 g", "0"),
                    level("coast to apogee", coast, "0", "−1 g"),
                ),
            );
        }
    };
    let gravity_m_s2 = match gravity.0 {
        Gravity::In => 0.0,
        Gravity::Out => g,
    };
    let f: Vec<f64> = raw.iter().map(|v| sign * v + gravity_m_s2).collect();
    let f_clean: Vec<f64> = clean.iter().map(|v| sign * v + gravity_m_s2).collect();
    let step = smallest_step(&raw);
    let accelerometer = Accelerometer {
        column: channel.column.clone(),
        gravity: gravity.0,
        gravity_by: gravity.1,
        turned_over: sign < 0.0,
        sample_interval_s: interval,
        step_m_s2: step,
    };
    // The burns: each a span of the cleaned trace past [`LIFTOFF_ACCELERATION_M_S2`] ([`spans`])
    // that holds past there for [`BURN_LEAST_S`], as a canopy's or a charge's shock doesn't, or
    // the first such span where none does, searched from liftoff's earliest time. The top is
    // searched over the burns, so a shock in a coast isn't taken for it, nor a bump on the pad.
    let mean = running_mean(&f_clean, half_mean);
    let spans = spans(&f_clean, &mean, from, end, interval);
    let mut burns: Vec<Span> = spans
        .iter()
        .copied()
        .filter(|span| span.held_s >= BURN_LEAST_S)
        .collect();
    // A motor that holds past there for less, a short burn's, is the first such span.
    if burns.is_empty() {
        burns.extend(spans.first());
    }
    // Whether the thrust ends on the channel: the last burn's mean falls through zero before
    // apogee. Where it doesn't, the accelerometer ends in the thrust, or the thrust runs on to
    // apogee, so the top may lie past the samples. Never without a burn: the thrust's direction
    // was found on one.
    let ended = burns.last().is_some_and(|burn| burn.falls);
    let thrust: Vec<usize> = if burns.is_empty() {
        (from..=end).collect()
    } else {
        burns
            .iter()
            .flat_map(|burn| burn.rise..=burn.fall)
            .collect()
    };
    let top_index = thrust
        .iter()
        .copied()
        .max_by(|a, b| f[*a].total_cmp(&f[*b]).then(b.cmp(a)))
        .unwrap_or(from);
    let top = f[top_index];
    let around: Vec<f64> = boost
        .iter()
        .copied()
        .filter(|&k| (time[k] - time[top_index]).abs() <= NOISE_SPAN_S)
        .map(|k| f[k])
        .collect();
    let jump = [top_index.wrapping_sub(1), top_index + 1]
        .iter()
        .filter_map(|&k| f.get(k))
        .filter(|v| v.is_finite())
        .map(|v| (top - v).abs())
        .fold(0.0, f64::max);
    let zeroed_m_s2 = match gravity.0 {
        Gravity::In => 0.0,
        Gravity::Out => g * (1.0 - RAIL_TILT_DEG.to_radians().cos()),
    };
    let noise_m_s2 = noise(&around);
    // How far the pad strays from the level it witnessed: a tilt or the part's own offset.
    let strayed_m_s2 = match (by_pad, pad) {
        (Some(Gravity::In), Some(pad)) => (pad - g).abs(),
        (Some(Gravity::Out), Some(pad)) => pad.abs(),
        _ => 0.0,
    };
    let bound_m_s2 = ACCEL_SCALE_SHARE * top.abs()
        + ACCEL_BIAS_M_S2
        + strayed_m_s2
        + zeroed_m_s2
        + ACCEL_NOISE_SIGMAS * noise_m_s2
        + step
        + jump;
    let range_m_s2 = stated.range_g.map(|range| range * g);
    // Clipped at the stated range, or held flat at the top short of it or with no range stated:
    // two causes, kept apart, as the first says the part read no more and the second only that
    // it may have clipped.
    let clip = if range_m_s2.is_some_and(|range| top >= range - 2.0 * step - tolerance) {
        Some(Clip::Range)
    } else if flat_top(&f, &thrust, top, interval, noise_m_s2, step) {
        Some(Clip::Flat)
    } else {
        None
    };
    let max_acceleration = Reading::Read(MaxAcceleration {
        acceleration_m_s2: top,
        bound_m_s2,
        time_s: time[top_index],
        range_m_s2,
        is_floor: clip.is_some() || !ended,
        clip,
        source: Source::Accelerometer,
        provenance: if clip.is_some() {
            Provenance::Clipped
        } else {
            Provenance::Measured
        },
        method: Method::HighestReading,
    });
    // Burnout is the thrust's end. After more than one burn the tuning set, whose flights burn
    // once, holds no bound for it; burnout from the speed's peak, which comes no later than the
    // last burn's end, stands instead. Nor is it read where the thrust doesn't end on the
    // channel, or where it would come before liftoff's earliest time.
    let read = match burns.as_slice() {
        [_, .., last] => Err(format!(
            "the accelerometer shows {} burns, the last rising past {:.0} g at {:.2} s: burnout \
             is read from a flight that burns once",
            burns.len(),
            LIFTOFF_ACCELERATION_M_S2 / g,
            time[last.rise]
        )),
        _ if !ended => Err(format!(
            "the accelerometer's thrust, past {:.0} g from {:.2} s, never falls through zero \
             before {}: the thrust doesn't end on the channel, so it shows no burnout",
            LIFTOFF_ACCELERATION_M_S2 / g,
            burns.first().map_or(time[from], |burn| time[burn.rise]),
            if end + 1 == time.len() {
                "the accelerometer's last sample"
            } else {
                "apogee"
            }
        )),
        _ => burnout(&time, &f, &mean, end, top_index, step, interval).and_then(
            |(time_s, bound_s)| match earliest_s {
                Some(earliest_s) if time_s < earliest_s => Err(format!(
                    "the accelerometer's burnout, {time_s:.2} s, would come before liftoff's \
                     earliest time by the heights, {earliest_s:.2} s"
                )),
                _ => Ok((time_s, bound_s)),
            },
        ),
    };
    let burnout = match read {
        Ok((time_s, bound_s)) => Reading::Read(Burnout {
            time_s,
            bound_s,
            time_after_liftoff_s: heights.liftoff.value().map(|l| time_s - l.time_s),
            is_floor: false,
            source: Source::Accelerometer,
            provenance: Provenance::Measured,
            method: Method::LeastAfterThrust,
        }),
        Err(detail) => fallback(Some(detail)),
    };
    (Some(accelerometer), max_acceleration, burnout)
}

/// Whether the top holds flat over [`FLAT_TOP_SAMPLES`] samples, or [`FLAT_TOP_S`] if that is
/// more: Debrief's sign of a part held at its range. Flat is within [`FLAT_TOP_SHARE`] of the top
/// or [`FLAT_TOP_M_S2`] if that is more, as Debrief has it, or within half the part's noise
/// `noise_m_s2` where that is less, and never less than half a step `step_m_s2`: a part at its
/// range writes one value, while a thrust's plateau or crest read by a quiet part still scatters
/// by its noise. On the tuning flights' quietest part, Debrief's tolerance alone is several times
/// the noise at the top, and called thrust plateaus clipped that weren't. A part quieter than half
/// its step writes a plateau as one value too, so on it a plateau as long as the run reads as
/// clipped: the reading is then a floor, which can't flatter the top.
fn flat_top(
    f: &[f64],
    boost: &[usize],
    top: f64,
    interval: f64,
    noise_m_s2: f64,
    step_m_s2: f64,
) -> bool {
    let near = (FLAT_TOP_SHARE * top.abs())
        .max(FLAT_TOP_M_S2)
        .min(0.5 * noise_m_s2)
        .max(0.5 * step_m_s2);
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a positive count of samples, rounded"
    )]
    let least = if interval > 0.0 {
        ((FLAT_TOP_S / interval).round() as usize).max(FLAT_TOP_SAMPLES)
    } else {
        FLAT_TOP_SAMPLES
    };
    let mut run = 0;
    let mut longest = 0;
    let mut last = None;
    for &k in boost {
        // A run is of neighboring samples.
        let next = last.is_some_and(|previous| k == previous + 1);
        run = if f[k] >= top - near {
            if next { run + 1 } else { 1 }
        } else {
            0
        };
        longest = longest.max(run);
        last = Some(k);
    }
    longest >= least
}

/// How far above a tie a count of intervals is lifted before it is rounded: a median interval
/// read off a clock's float times can fall a few parts in 10¹⁶ short, as 0.05 s does at 20 Hz,
/// and would round 0.5 to 0, not 1.
const TIE_SHARE: f64 = 1e-9;

/// The samples either side of a window `span_s` long holds, at `interval`: half the span in
/// intervals, rounded, a tie up, at an interval no shorter than [`MIN_INTERVAL_S`].
fn half_window(span_s: f64, interval: f64) -> usize {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a positive count of samples, rounded: at MIN_INTERVAL_S a span of a second holds \
                  250 either side"
    )]
    let half = (0.5 * span_s / interval.max(MIN_INTERVAL_S) * (1.0 + TIE_SHARE)).round() as usize;
    half
}

/// The mean of `values` over `half` samples either side of each, the window cut short at the ends.
fn running_mean(values: &[f64], half: usize) -> Vec<f64> {
    let last = values.len().saturating_sub(1);
    (0..values.len())
        .map(|k| {
            let window = &values[k.saturating_sub(half)..=k.saturating_add(half).min(last)];
            #[expect(
                clippy::cast_precision_loss,
                reason = "a window of a few samples converts exactly"
            )]
            let count = window.len() as f64;
            window.iter().sum::<f64>() / count
        })
        .collect()
}

/// A span of a trace past [`LIFTOFF_ACCELERATION_M_S2`]: from the sample `rise` past it to the
/// sample `fall` where the trace's running mean next falls through zero, or the search's end if
/// it doesn't.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Span {
    /// The first sample past [`LIFTOFF_ACCELERATION_M_S2`].
    rise: usize,
    /// Where the mean next falls through zero, or the search's end.
    fall: usize,
    /// Whether the mean falls through zero before the search's end.
    falls: bool,
    /// How long the trace holds past [`LIFTOFF_ACCELERATION_M_S2`] over the span, s: its samples
    /// there, an interval each.
    held_s: f64,
}

/// The spans of `values` past [`LIFTOFF_ACCELERATION_M_S2`] from sample `from` to `end`, each
/// ending where `mean`, the trace's running mean, next falls through zero, the next searched
/// from after it; at `interval`.
fn spans(values: &[f64], mean: &[f64], from: usize, end: usize, interval: f64) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut next = from;
    while let Some(rise) = (next..=end).find(|&k| values[k] >= LIFTOFF_ACCELERATION_M_S2) {
        let fall = (rise + 1..=end).find(|&k| mean[k] <= 0.0);
        let to = fall.unwrap_or(end);
        #[expect(
            clippy::cast_precision_loss,
            reason = "a count of samples converts exactly"
        )]
        let held_s = (rise..=to)
            .filter(|&k| values[k] >= LIFTOFF_ACCELERATION_M_S2)
            .count() as f64
            * interval;
        spans.push(Span {
            rise,
            fall: to,
            falls: fall.is_some(),
            held_s,
        });
        match fall {
            Some(fall) => next = fall + 1,
            None => break,
        }
    }
    spans
}

/// Which way up the accelerometer reads, +1 nose up or −1 nose down, from its cleaned trace
/// `clean` as written, its samples `from` to `end` at `interval`, averaged over `half` samples
/// either side to end a span: the sign of the first span, either way up, that holds past
/// [`LIFTOFF_ACCELERATION_M_S2`] for [`BURN_LEAST_S`], as a thrust does and a knock on the pad
/// doesn't, whichever way it reads; where none holds that long, the sign every span shares, a
/// short burn's. A pad reads 1 g or 0 either way up and a rocket leaves it at 3 g or more, so a
/// thrust passes 2 g either way. Where spans go both ways and none holds, or none passes 2 g, the
/// thrust can't be told, and the words say why.
fn thrust_sign(
    clean: &[f64],
    from: usize,
    end: usize,
    half: usize,
    interval: f64,
) -> Result<f64, String> {
    let walk = |sign: f64| {
        let values: Vec<f64> = clean.iter().map(|v| sign * v).collect();
        let mean = running_mean(&values, half);
        spans(&values, &mean, from, end, interval)
    };
    let (up, down) = (walk(1.0), walk(-1.0));
    let held = |spans: &[Span]| {
        spans
            .iter()
            .find(|span| span.held_s >= BURN_LEAST_S)
            .map(|span| span.rise)
    };
    match (held(&up), held(&down)) {
        (Some(up), Some(down)) => Ok(if down < up { -1.0 } else { 1.0 }),
        (Some(_), None) => Ok(1.0),
        (None, Some(_)) => Ok(-1.0),
        (None, None) => match (up.is_empty(), down.is_empty()) {
            (false, true) => Ok(1.0),
            (true, false) => Ok(-1.0),
            (false, false) => Err(format!(
                "reads past {:.0} g both ways up, {} times nose up and {} nose down, and never \
                 for {BURN_LEAST_S} s, as a thrust holds: which way up it reads, and so the \
                 thrust, can't be told from a knock",
                LIFTOFF_ACCELERATION_M_S2 / STANDARD_GRAVITY_MPS2,
                up.len(),
                down.len()
            )),
            (true, true) => Err(format!(
                "never reads past {:.0} g either way up from liftoff's earliest time to apogee, \
                 as a rocket leaving the pad does: which way up it reads, and so the thrust, \
                 can't be told",
                LIFTOFF_ACCELERATION_M_S2 / STANDARD_GRAVITY_MPS2
            )),
        },
    }
}

/// Burnout from the accelerometer: its time and bound, or why the accelerometer gives none. `f`
/// is the trace, `mean` its cleaned twin averaged over [`BURNOUT_MEAN_S`] at `interval`, `end` the
/// last sample before apogee and `top_index` the top acceleration's.
fn burnout(
    time: &[f64],
    f: &[f64],
    mean: &[f64],
    end: usize,
    top_index: usize,
    step: f64,
    interval: f64,
) -> Result<(f64, f64), String> {
    // The trace averaged over [`BURNOUT_MEAN_S`], `2h + 1` samples, which cuts its noise by
    // √(2h + 1) and spreads burnout over the window.
    let h = half_window(BURNOUT_MEAN_S, interval);
    let Some(crossing) = (top_index + 1..=end).find(|&k| mean[k] <= 0.0) else {
        return Err(
            "the accelerometer never reads below zero between its top and apogee, so it shows \
             no burnout"
                .to_owned(),
        );
    };
    let noise_end = time
        .partition_point(|t| *t <= time[crossing] + 1.0)
        .min(end + 1);
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a few samples converts exactly"
    )]
    let averaged = ((2 * h + 1) as f64).sqrt();
    let n =
        ACCEL_NOISE_SIGMAS * noise(&f[top_index..noise_end.max(top_index + 1)]) / averaged + step;
    let mut least = (f64::INFINITY, crossing);
    let mut stop = None;
    for (k, value) in mean.iter().enumerate().take(end + 1).skip(crossing) {
        if *value < least.0 {
            least = (*value, k);
        } else if *value > least.0 + 4.0 * n {
            stop = Some(k);
            break;
        }
    }
    let Some(stop) = stop else {
        return Err(format!(
            "the accelerometer's reading doesn't climb {:.2} m/s² back from its least, four times \
             its noise and step, before apogee: burnout can't be told from the coast",
            4.0 * n
        ));
    };
    if least.0 > -3.0 * n {
        return Err(format!(
            "the accelerometer's least reading after the thrust, {:.2} m/s², lies within {:.2} \
             m/s², three times its noise and step, of zero: too little drag to tell burnout by",
            least.0,
            3.0 * n
        ));
    }
    let near: Vec<usize> = (crossing..stop)
        .filter(|&k| mean[k] <= least.0 + 2.0 * n)
        .collect();
    // Never empty: the least's own sample lies in `crossing..stop` and within `2N` of itself.
    let (Some(&first), Some(&last)) = (near.first(), near.last()) else {
        return Err(
            "the accelerometer's least reading after the thrust can't be placed".to_owned(),
        );
    };
    // The averaged trace's least lies from two intervals before the first such sample, one for
    // the sample next to it and one for the part's filter, which spreads a thrust cut off at once
    // over about a sample, to an interval after the last; burnout lies within half the average's
    // window, `w/2`, of it. The thrust falls steeply and the coast's drag eases slowly, so it lies
    // near the first, where the average, reaching the coast's level only once its window has
    // passed a thrust cut off at once, reads it late by `w/2`: the reading is the first less
    // `w/2`, and the bound reaches back two intervals and on to the last, an interval and `w/2`.
    let interval = local_interval(time, first).max(local_interval(time, last));
    #[expect(
        clippy::cast_precision_loss,
        reason = "a window of a few samples converts exactly"
    )]
    let half_window_s = h as f64 * interval;
    let bound_s = (time[last] - time[first] + interval + 2.0 * half_window_s).max(2.0 * interval);
    Ok((time[first] - half_window_s, bound_s))
}

/// Burnout from the top speed's peak, a floor; or why it is withheld. `why` says why the
/// accelerometer gave none, where it has one.
fn from_speed(heights: &Readings, why: Option<String>) -> Reading<Burnout> {
    match &heights.max_speed {
        Reading::Read(speed) => Reading::Read(Burnout {
            time_s: speed.time_s,
            bound_s: 2.0 * SPEED_NOISE_M_S / STANDARD_GRAVITY_MPS2
                + 0.5 * SPEED_WINDOW_S
                + heights.sample_interval_s.unwrap_or(0.0),
            time_after_liftoff_s: heights.liftoff.value().map(|l| speed.time_s - l.time_s),
            is_floor: true,
            source: speed.source,
            provenance: Provenance::Derived,
            method: Method::SpeedPeak,
        }),
        Reading::Withheld(Withheld { reason, detail }) => Reading::withheld(
            *reason,
            match why {
                Some(why) => format!(
                    "{why}; and burnout from the speed's peak needs the top speed, which is \
                     withheld: {detail}"
                ),
                None => format!(
                    "burnout from the speed's peak needs the top speed, which is withheld: \
                     {detail}"
                ),
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::read as read_all;
    use super::*;
    use crate::log::{Channel, Stated};

    const G: f64 = STANDARD_GRAVITY_MPS2;

    /// Uniform noise in ±`amplitude`/2, the same on every run: σ is `amplitude`/√12.
    fn jitter(k: usize, amplitude: f64) -> f64 {
        #[expect(
            clippy::cast_precision_loss,
            reason = "a sample's index converts exactly"
        )]
        let x = (k as f64 * 12.9898).sin() * 43_758.545_3;
        amplitude * (x - x.floor() - 0.5)
    }

    /// The pad to 3 s, a climb at 4 g to 4 s, a vacuum coast to apogee at 8 s (98.1 m), then
    /// down at 10 m/s: the height track the accelerometer's readings are searched against.
    fn height(t: f64) -> f64 {
        let c = t - 3.0;
        if c <= 0.0 {
            0.0
        } else if c < 1.0 {
            2.0 * G * c * c
        } else if c < 5.0 {
            let d = c - 1.0;
            2.0 * G + 4.0 * G * d - 0.5 * G * d * d
        } else {
            (10.0 * G - 10.0 * (c - 5.0)).max(0.0)
        }
    }

    /// The specific force along the axis, gravity in: 1 g on the pad, a thrust rising to 6 g at
    /// 3.5 s, falling through zero to −0.6 g at burnout, 4.1 s, as the drag pulls back, the
    /// drag's pull easing to nothing at apogee, then 1 g again under a canopy.
    fn specific_force(t: f64) -> f64 {
        if t < 3.0 {
            G
        } else if t < 4.0 {
            5.0 * G + G * (std::f64::consts::PI * (t - 3.0)).sin()
        } else if t < 4.1 {
            5.0 * G - 5.6 * G * (t - 4.0) / 0.1
        } else if t < 8.0 {
            -0.6 * G * (8.0 - t) / 3.9
        } else {
            G
        }
    }

    /// A log of [`height`] every 0.05 s and an accelerometer under `column`, read as `format`,
    /// sampled every `dt` s, writing `value` of each sample's index and time.
    fn log(
        format: LogFormat,
        column: &str,
        dt: f64,
        value: impl Fn(usize, f64) -> f64,
    ) -> FlightLog {
        let clock = |dt: f64| -> Vec<f64> {
            (0..)
                .map(|i| f64::from(i) * dt)
                .take_while(|t| *t <= 30.0 + 1e-9)
                .collect()
        };
        let time_s = clock(0.05);
        let accel_s = clock(dt);
        let values = accel_s
            .iter()
            .enumerate()
            .map(|(k, t)| value(k, *t))
            .collect();
        FlightLog {
            format,
            logger: "test".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            altitude_m: time_s.iter().map(|t| height(*t)).collect(),
            time_s,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: vec![Channel::new(
                column,
                Quantity::Acceleration,
                accel_s,
                values,
            )],
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// A noisy axial trace reads its top and burnout within their bounds, measured; gravity is
    /// settled by the pad, as a column that only names the axis can't say.
    #[test]
    fn an_axial_trace_reads_its_top_and_burnout() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| specific_force(t) + jitter(k, 0.3),
        ));
        let accelerometer = readings.accelerometer.unwrap();
        assert_eq!(accelerometer.gravity, Gravity::In);
        assert_eq!(accelerometer.gravity_by, GravityBy::Pad);
        assert!(!accelerometer.turned_over);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!((top.time_s - 3.5).abs() < 0.1, "{top:?}");
        assert_eq!(top.provenance, Provenance::Measured);
        assert_eq!(top.method, Method::HighestReading);
        assert_eq!(top.range_m_s2, None);
        let burnout = readings.burnout.value().unwrap();
        assert!(!burnout.is_floor);
        assert_eq!(burnout.method, Method::LeastAfterThrust);
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
        // The least lies at the thrust's end, to a sample.
        assert!((burnout.time_s - 4.1).abs() <= 0.01, "{burnout:?}");
        // The bound is the samples within 2N of the least: N is 5σ over √7 samples averaged and
        // a step, 0.16 m/s², and the coast's drag eases 1.5 m/s² a second, so they span about
        // 0.22 s; with an interval and the 0.07 s window, 0.30 s.
        let sigma = 0.3 / 12.0_f64.sqrt();
        let n = ACCEL_NOISE_SIGMAS * sigma / 7.0_f64.sqrt();
        let span = 2.0 * n / (0.6 * G / 3.9);
        assert!(
            (burnout.bound_s - (span + 0.01 + 0.07)).abs() < 0.03,
            "{burnout:?}, span {span}"
        );
    }

    /// A part mounted nose down, gravity taken out, reads the same: turned over and put back.
    #[test]
    fn a_turned_over_part_without_gravity_reads_alike() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| -(specific_force(t) - G) + jitter(k, 0.3),
        ));
        let accelerometer = readings.accelerometer.unwrap();
        assert!(accelerometer.turned_over);
        assert_eq!(accelerometer.gravity, Gravity::Out);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        let burnout = readings.burnout.value().unwrap();
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
    }

    /// A part held at its range is clipped: by the range its family states, or, with none, by a
    /// top held flat. The true top lies above the reading less its bound, as a floor states.
    #[test]
    fn a_top_held_at_the_range_is_clipped() {
        // Fluctus states a ±200 g part; the trace scaled to peak at 240 g reads 200 g.
        let scale = 40.0;
        let readings = read_all(&log(
            LogFormat::FluctusCsv,
            "vert-accel (m/s2)",
            0.01,
            |_, t| {
                if t < 3.0 {
                    G
                } else {
                    (scale * (specific_force(t) - G) + G).min(200.0 * G)
                }
            },
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert_eq!(top.provenance, Provenance::Clipped, "{top:?}");
        assert_eq!(top.range_m_s2, Some(200.0 * G));
        assert!(top.acceleration_m_s2 - top.bound_m_s2 <= scale * 5.0 * G + G);
        // Below the range, the same family's part reads its top, measured.
        let readings = read_all(&log(
            LogFormat::FluctusCsv,
            "vert-accel (m/s2)",
            0.01,
            |k, t| {
                if t < 3.0 {
                    G + jitter(k, 0.3)
                } else {
                    20.0 * (specific_force(t) - G) + G + jitter(k, 0.3)
                }
            },
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert_eq!(top.provenance, Provenance::Measured, "{top:?}");
        // No range stated: flat at 5.5 g for 0.6 s, where the thrust peaks at 6 g.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| (specific_force(t) + jitter(k, 0.3)).min(5.5 * G),
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert_eq!(top.provenance, Provenance::Clipped, "{top:?}");
        assert_eq!(top.range_m_s2, None);
        assert!((top.acceleration_m_s2 - 5.5 * G).abs() < 1e-9);
    }

    /// A quiet part's plateau isn't a clip: within Debrief's tolerance, 0.25 m/s² at 5 g, it holds
    /// for the whole plateau, but it still scatters by its noise, 0.03 m/s² here.
    #[test]
    fn a_quiet_plateau_is_not_clipped() {
        let plateau = |t: f64| {
            if (3.0..4.0).contains(&t) {
                5.0 * G
            } else {
                specific_force(t)
            }
        };
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.002,
            |k, t| plateau(t) + jitter(k, 0.1),
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert!((top.acceleration_m_s2 - 5.0 * G).abs() <= top.bound_m_s2);
        assert_eq!(top.provenance, Provenance::Measured, "{top:?}");
    }

    /// A family's word that its readings hold no gravity, against a pad that reads 1 g, settles
    /// nothing; nor does a family that doesn't say, over a pad that reads neither.
    #[test]
    fn gravity_unsettled_withholds_the_top_and_burnout_falls_back() {
        let readings = read_all(&log(LogFormat::Marsa, "acceleration", 0.01, |_, t| {
            specific_force(t)
        }));
        assert_eq!(
            readings.max_acceleration.reason(),
            Some(Reason::GravityUnstated)
        );
        assert!(readings.accelerometer.is_none());
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |_, t| specific_force(t) - 0.5 * G,
        ));
        assert_eq!(
            readings.max_acceleration.reason(),
            Some(Reason::GravityUnstated)
        );
        // Burnout falls back to the speed's peak, a floor.
        let burnout = readings.burnout.value().unwrap();
        assert!(burnout.is_floor);
        assert_eq!(burnout.method, Method::SpeedPeak);
    }

    /// A log that starts in the thrust has no pad to tell gravity by; its coast to apogee still
    /// does, reading about 0 with gravity in and about −1 g with it out. The drag here eases as
    /// the square of a speed that falls off in the coast, under 0.04 g in the last second.
    #[test]
    fn the_coast_settles_gravity_where_there_is_no_pad() {
        let eased = |t: f64| {
            if (4.1..8.0).contains(&t) {
                -0.6 * G * ((8.0 - t) / 3.9).powi(2)
            } else {
                specific_force(t)
            }
        };
        let late = |gravity_out: bool| {
            let mut log = log(LogFormat::GenericTable, "Axial accel", 0.01, |k, t| {
                eased(t) - if gravity_out { G } else { 0.0 } + jitter(k, 0.3)
            });
            // The accelerometer starts at 3.2 s, its thrust already 5.6 g.
            let channel = &mut log.channels[0];
            let first = channel.time_s.iter().position(|t| *t >= 3.2).unwrap();
            channel.time_s.drain(..first);
            channel.values.drain(..first);
            read_all(&log)
        };
        for (out, gravity) in [(false, Gravity::In), (true, Gravity::Out)] {
            let readings = late(out);
            let accelerometer = readings.accelerometer.unwrap();
            assert_eq!(accelerometer.gravity, gravity);
            assert_eq!(accelerometer.gravity_by, GravityBy::Coast);
            let top = readings.max_acceleration.value().unwrap();
            assert!(
                (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
                "{top:?}"
            );
        }
        // A family that says gravity is out, over a coast that reads 0, disagrees.
        let mut log = log(LogFormat::Marsa, "acceleration", 0.01, |_, t| eased(t));
        let channel = &mut log.channels[0];
        let first = channel.time_s.iter().position(|t| *t >= 3.2).unwrap();
        channel.time_s.drain(..first);
        channel.values.drain(..first);
        let readings = read_all(&log);
        assert_eq!(
            readings.max_acceleration.reason(),
            Some(Reason::GravityUnstated)
        );
    }

    /// A sample that repeats the time before it is left out, the first kept, as AltOS's primary
    /// track keeps it; a clock that runs back is still refused.
    #[test]
    fn a_repeated_time_keeps_the_first_sample() {
        let mut log = log(LogFormat::GenericTable, "Axial accel", 0.01, |k, t| {
            specific_force(t) + jitter(k, 0.3)
        });
        let channel = &mut log.channels[0];
        let k = channel.time_s.iter().position(|t| *t >= 3.5).unwrap();
        channel.time_s.insert(k + 1, channel.time_s[k]);
        channel.values.insert(k + 1, 100.0 * G);
        let readings = read_all(&log);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        let channel = &mut log.channels[0];
        channel.time_s[k + 1] = channel.time_s[k] - 0.005;
        assert_eq!(
            read_all(&log).max_acceleration.reason(),
            Some(Reason::ClockOutOfOrder)
        );
    }

    /// With no drag to pull the reading below zero, the accelerometer shows no burnout, and the
    /// speed's peak gives a floor: burnout, 4.0 s here, comes no sooner than it less its bound.
    #[test]
    fn no_fall_below_zero_is_a_floor_from_the_speed() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |_, t| {
                if (4.0..8.0).contains(&t) {
                    0.0
                } else {
                    specific_force(t)
                }
            },
        ));
        assert!(readings.max_acceleration.value().is_some());
        let burnout = readings.burnout.value().unwrap();
        assert!(burnout.is_floor);
        assert_eq!(burnout.provenance, Provenance::Derived);
        assert!(burnout.time_s - 4.0 <= burnout.bound_s, "{burnout:?}");
    }

    /// A charge's shock in the coast, larger than the thrust, isn't taken for the top, and one
    /// sample of −60 g in the thrust neither turns the trace over nor moves burnout.
    #[test]
    fn a_spike_neither_tops_nor_turns_the_trace() {
        let base = |k: usize, t: f64| specific_force(t) + jitter(k, 0.3);
        // A 12 g shock for 30 ms at 7.5 s, half a second before apogee.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                if (7.5..7.53).contains(&t) {
                    12.0 * G
                } else {
                    base(k, t)
                }
            },
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!(top.time_s < 4.1, "{top:?}");
        assert!(!readings.accelerometer.as_ref().unwrap().turned_over);
        // A canopy's shock of −20 g for 0.2 s at 7 s, too long for the filter: the thrust's 6 g
        // still comes first, so the trace isn't turned over.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                if (7.0..7.2).contains(&t) {
                    -20.0 * G
                } else {
                    base(k, t)
                }
            },
        ));
        assert!(!readings.accelerometer.as_ref().unwrap().turned_over);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        // One sample of −60 g at 3.3 s, in the thrust.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                if (t - 3.3).abs() < 1e-9 {
                    -60.0 * G
                } else {
                    base(k, t)
                }
            },
        ));
        assert!(!readings.accelerometer.as_ref().unwrap().turned_over);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        let burnout = readings.burnout.value().unwrap();
        assert!(!burnout.is_floor, "{burnout:?}");
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
    }

    /// A sample that isn't a number is left out, and the readings stand without it.
    #[test]
    fn a_sample_that_is_no_number_is_left_out() {
        for bad in [f64::NAN, f64::NEG_INFINITY, f64::INFINITY] {
            let readings = read_all(&log(
                LogFormat::GenericTable,
                "Axial accel",
                0.01,
                |k, t| {
                    if (t - 3.8).abs() < 1e-9 {
                        bad
                    } else {
                        specific_force(t) + jitter(k, 0.3)
                    }
                },
            ));
            let top = readings.max_acceleration.value().unwrap();
            assert!(
                (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
                "{bad}: {top:?}"
            );
            let burnout = readings.burnout.value().unwrap();
            assert!(!burnout.is_floor, "{bad}: {burnout:?}");
            assert!(
                (burnout.time_s - 4.1).abs() <= burnout.bound_s,
                "{bad}: {burnout:?}"
            );
        }
    }

    /// An accelerometer sampled faster than its readings were checked at is withheld, as a height
    /// track is, and burnout falls back to the top speed.
    #[test]
    fn an_accelerometer_sampled_too_fast_is_withheld() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.001,
            |_, t| specific_force(t),
        ));
        assert_eq!(
            readings.max_acceleration.reason(),
            Some(Reason::SampledTooFast)
        );
        assert!(readings.accelerometer.is_none());
        assert!(readings.burnout.value().unwrap().is_floor);
        // 160,000 samples 1e-7 s apart, a clock that once overflowed the burnout window's count
        // and averaged for seconds: refused before either.
        let mut fast = log(LogFormat::GenericTable, "Axial accel", 0.01, |_, t| {
            specific_force(t)
        });
        let time_s: Vec<f64> = (0..160_000).map(|i| 3.0 + f64::from(i) * 1e-7).collect();
        let values = time_s.iter().map(|t| specific_force(*t)).collect();
        fast.channels = vec![Channel::new(
            "Axial accel",
            Quantity::Acceleration,
            time_s,
            values,
        )];
        assert_eq!(
            read_all(&fast).max_acceleration.reason(),
            Some(Reason::SampledTooFast)
        );
    }

    /// A second burn, an air start's or a sustainer's, is searched for the top, and as the tuning
    /// set's flights burn once, the accelerometer gives no burnout: the speed's peak, no later
    /// than the last burn's end, gives it as a floor. A pulse in the coast that holds past 2 g for
    /// less than [`BURN_LEAST_S`], as a canopy's shock does, is no burn.
    #[test]
    fn a_second_burn_is_searched_for_the_top_and_leaves_burnout_to_the_speed() {
        let with = |from: f64, to: f64, peak_g: f64| {
            read_all(&log(
                LogFormat::GenericTable,
                "Axial accel",
                0.01,
                |k, t| {
                    let value = if (from..to).contains(&t) {
                        peak_g * G * ((t - from) / 0.05).min((to - t) / 0.05).min(1.0)
                    } else {
                        specific_force(t)
                    };
                    value + jitter(k, 0.3)
                },
            ))
        };
        // A sustainer's burn at 8 g from 5 s to 6.5 s, in the coast.
        let readings = with(5.0, 6.5, 8.0);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 8.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!((5.0..6.5).contains(&top.time_s), "{top:?}");
        let burnout = readings.burnout.value().unwrap();
        assert!(burnout.is_floor, "{burnout:?}");
        assert_eq!(burnout.method, Method::SpeedPeak);
        // A canopy's shock at 10 g for 0.4 s, from 7 s: the top and burnout are the burn's.
        let readings = with(7.0, 7.4, 10.0);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        let burnout = readings.burnout.value().unwrap();
        assert!(!burnout.is_floor, "{burnout:?}");
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
    }

    /// A span past 2 g that holds there for less than [`BURN_LEAST_S`] is no burn, as a knock on
    /// the pad with its rebound isn't, unless no span holds that long: then the first is the
    /// burn, a short motor's.
    #[test]
    fn a_burn_holds_past_two_g_or_is_the_only_one() {
        // A knock to 4 g and its rebound to −2 g, 0.15 s each, at 2.5 s, before the thrust.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                let value = if (2.5..2.65).contains(&t) {
                    4.0 * G
                } else if (2.65..2.8).contains(&t) {
                    -2.0 * G
                } else {
                    specific_force(t)
                };
                value + jitter(k, 0.3)
            },
        ));
        let burnout = readings.burnout.value().unwrap();
        assert!(!burnout.is_floor, "{burnout:?}");
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
        // A motor that burns 0.3 s at 6 g, and a canopy's shock at 10 g for 0.4 s at 7 s.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                let value = if t < 3.0 {
                    G
                } else if t < 3.3 {
                    6.0 * G
                } else if t < 3.4 {
                    6.0 * G - 6.6 * G * (t - 3.3) / 0.1
                } else if (7.0..7.4).contains(&t) {
                    10.0 * G
                } else if t < 8.0 {
                    -0.6 * G * (8.0 - t) / 4.6
                } else {
                    G
                };
                value + jitter(k, 0.3)
            },
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!(top.time_s < 3.4, "{top:?}");
    }

    /// A bump on the pad, wider than the cleaning takes out and higher than the thrust, comes
    /// before liftoff's earliest time and isn't the top: a rocket at rest reads 1 g.
    #[test]
    fn a_bump_on_the_pad_isnt_the_top() {
        // A knock to 7 g over 0.3 s at 1 s, two seconds before liftoff.
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                let value = if (1.0..1.3).contains(&t) {
                    G + 6.0 * G * (std::f64::consts::PI * (t - 1.0) / 0.3).sin()
                } else {
                    specific_force(t)
                };
                value + jitter(k, 0.3)
            },
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!(top.time_s >= 3.0, "{top:?}");
        let burnout = readings.burnout.value().unwrap();
        assert!(!burnout.is_floor, "{burnout:?}");
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
    }

    /// A count of intervals a few parts in 10¹⁶ short of a tie, as a median interval of 20 Hz
    /// float times can be, rounds up as the tie itself does.
    #[test]
    fn a_window_on_a_tie_rounds_up() {
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.05), 1);
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.050_000_000_000_000_71), 1);
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.01), 3);
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.010_000_000_000_000_2), 3);
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.002), 13);
        assert_eq!(half_window(BURNOUT_MEAN_S, 0.002_000_000_000_000_002), 13);
    }

    /// A top that reaches the part's range in one sample, with no flat run, is clipped by the
    /// range alone: the same trace with no range stated is measured.
    #[test]
    fn a_top_reaching_the_range_in_one_sample_is_clipped() {
        // Peaking at 200 g at 3.5 s, Fluctus's stated range.
        let peak = |t: f64| {
            if t < 3.0 {
                G
            } else {
                39.8 * (specific_force(t) - G) + G
            }
        };
        let fluctus = read_all(&log(
            LogFormat::FluctusCsv,
            "vert-accel (m/s2)",
            0.01,
            |_, t| peak(t),
        ));
        let top = fluctus.max_acceleration.value().unwrap();
        assert_eq!(top.provenance, Provenance::Clipped, "{top:?}");
        let unstated = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |_, t| peak(t),
        ));
        let top = unstated.max_acceleration.value().unwrap();
        assert_eq!(top.provenance, Provenance::Measured, "{top:?}");
    }

    /// Burnout's refusals, each by its own words: a coast that never climbs back from its least
    /// before apogee, and a least too near zero to tell from the noise.
    #[test]
    fn burnout_refuses_a_coast_it_cant_tell() {
        let dt = 0.01;
        let time: Vec<f64> = (0..=790).map(|i| f64::from(i) * dt).collect();
        let try_burnout = |drag_g: f64, back: bool| {
            let f: Vec<f64> = time
                .iter()
                .enumerate()
                .map(|(k, &t)| {
                    let coast = if back && t >= 7.5 { G } else { -drag_g * G };
                    (if t < 4.0 { specific_force(t) } else { coast }) + jitter(k, 0.3)
                })
                .collect();
            let mean = running_mean(&f, half_window(BURNOUT_MEAN_S, dt));
            let top = (0..f.len()).max_by(|a, b| f[*a].total_cmp(&f[*b])).unwrap();
            burnout(&time, &f, &mean, time.len() - 1, top, 0.0, dt)
        };
        // A steady drag to the log's end: no climb back.
        let error = try_burnout(0.6, false).unwrap_err();
        assert!(error.contains("doesn't climb"), "{error}");
        // A drag of 0.02 g, then the canopy's 1 g: back, but the least is within 3N of zero.
        let error = try_burnout(0.02, true).unwrap_err();
        assert!(error.contains("too little drag"), "{error}");
        // The same at 0.6 g reads.
        assert!(try_burnout(0.6, true).is_ok());
    }

    /// Which way up a part reads can't be told where its trace never passes 2 g from liftoff's
    /// earliest time to apogee, or passes it both ways up and never holds there for
    /// [`BURN_LEAST_S`]: the top and the accelerometer's burnout are withheld as
    /// [`Reason::DirectionUnstated`], and burnout falls back to the top speed's floor.
    #[test]
    fn a_trace_that_shows_no_thrust_either_way_up_is_withheld() {
        let pulse = |t: f64, at: f64| {
            if (at..at + 0.2).contains(&t) {
                4.0 * G
            } else {
                0.0
            }
        };
        for (case, trace) in [
            ("flat", Box::new(|_: f64| G) as Box<dyn Fn(f64) -> f64>),
            (
                "both ways",
                // 5 g for 0.2 s in the thrust, then −5 g for 0.2 s in the coast.
                Box::new(move |t: f64| G + pulse(t, 3.4) - 1.5 * pulse(t, 5.0)),
            ),
        ] {
            let readings = read_all(&log(
                LogFormat::GenericTable,
                "Axial accel",
                0.01,
                |k, t| trace(t) + jitter(k, 0.3),
            ));
            assert_eq!(
                readings.max_acceleration.reason(),
                Some(Reason::DirectionUnstated),
                "{case}"
            );
            let burnout = readings.burnout.value().unwrap();
            assert_eq!(burnout.method, Method::SpeedPeak, "{case}");
            assert!(burnout.is_floor, "{case}");
        }
    }

    /// A dip on the pad, to −3 g over 0.3 s at 1 s, doesn't turn the trace over: it never holds
    /// past 2 g for [`BURN_LEAST_S`], and the thrust after it does. So on a column that holds
    /// gravity and on AltOS's, which takes it out, the top and burnout are the thrust's.
    #[test]
    fn a_dip_on_the_pad_doesnt_turn_the_trace_over() {
        let dipped = |t: f64| {
            if (1.0..1.3).contains(&t) {
                G - 4.0 * G * (std::f64::consts::PI * (t - 1.0) / 0.3).sin()
            } else {
                specific_force(t)
            }
        };
        for (format, column, out_m_s2) in [
            (LogFormat::GenericTable, "Axial accel", 0.0),
            (LogFormat::AltosCsv, "acceleration", G),
        ] {
            let readings = read_all(&log(format, column, 0.01, |k, t| {
                dipped(t) - out_m_s2 + jitter(k, 0.3)
            }));
            let accelerometer = readings.accelerometer.as_ref().unwrap();
            assert!(!accelerometer.turned_over, "{column}: {accelerometer:?}");
            let top = readings.max_acceleration.value().unwrap();
            assert!(
                (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
                "{column}: {top:?}"
            );
            assert!((top.time_s - 3.5).abs() < 0.1, "{column}: {top:?}");
            let burnout = readings.burnout.value().unwrap();
            assert_eq!(burnout.method, Method::LeastAfterThrust, "{column}");
            assert!(
                (burnout.time_s - 4.1).abs() <= burnout.bound_s,
                "{column}: {burnout:?}"
            );
        }
    }

    /// A knock on the pad with its rebound first, −2.5 g for 0.15 s at 2.5 s and then 4 g for
    /// 0.15 s, inside liftoff's bound: neither holds past 2 g for [`BURN_LEAST_S`], so the trace
    /// isn't turned over, and the top and burnout are the thrust's, neither before liftoff's
    /// earliest time.
    #[test]
    fn a_knock_with_its_rebound_first_doesnt_turn_the_trace_over() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| {
                let value = if (2.5..2.65).contains(&t) {
                    -2.5 * G
                } else if (2.65..2.8).contains(&t) {
                    4.0 * G
                } else {
                    specific_force(t)
                };
                value + jitter(k, 0.3)
            },
        ));
        assert!(!readings.accelerometer.as_ref().unwrap().turned_over);
        let liftoff = readings.liftoff.value().unwrap();
        let earliest = liftoff.time_s - liftoff.bound_s;
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        assert!(top.time_s >= earliest.max(3.0), "{top:?}, {liftoff:?}");
        let burnout = readings.burnout.value().unwrap();
        assert_eq!(burnout.method, Method::LeastAfterThrust, "{burnout:?}");
        assert!(
            (burnout.time_s - 4.1).abs() <= burnout.bound_s,
            "{burnout:?}"
        );
        assert!(burnout.time_s >= earliest, "{burnout:?}, {liftoff:?}");
        assert!(
            burnout
                .time_after_liftoff_s
                .is_some_and(|after| after > 0.0),
            "{burnout:?}"
        );
    }

    /// An accelerometer that ends in the thrust, still rising from 3 g to 6 g, gives its top as a
    /// floor: the thrust never falls through zero on the channel, so the top may have been
    /// higher, and burnout isn't read off it. So whether the whole log ends at 3.7 s or only the
    /// accelerometer does.
    #[test]
    fn an_accelerometer_that_ends_in_the_thrust_gives_its_top_as_a_floor() {
        let rising = |t: f64| {
            if t < 3.0 {
                G
            } else {
                3.0 * G + 3.0 * G * (t - 3.0)
            }
        };
        for whole in [true, false] {
            let mut log = log(LogFormat::GenericTable, "Axial accel", 0.01, |k, t| {
                rising(t) + jitter(k, 0.3)
            });
            let cut = |time_s: &[f64]| time_s.partition_point(|t| *t <= 3.7 + 1e-9);
            let channel = &mut log.channels[0];
            let kept = cut(&channel.time_s);
            channel.time_s.truncate(kept);
            channel.values.truncate(kept);
            if whole {
                let kept = cut(&log.time_s);
                log.time_s.truncate(kept);
                log.altitude_m.truncate(kept);
            }
            let readings = read_all(&log);
            let top = readings.max_acceleration.value().unwrap();
            assert!(top.is_floor, "whole {whole}: {top:?}");
            assert_eq!(top.provenance, Provenance::Measured, "whole {whole}");
            // The reading less its bound lies below the thrust where the channel ends.
            assert!(
                top.acceleration_m_s2 - top.bound_m_s2 <= rising(3.7),
                "whole {whole}: {top:?}"
            );
            assert!(
                readings
                    .burnout
                    .value()
                    .is_none_or(|burnout| burnout.method != Method::LeastAfterThrust),
                "whole {whole}: {:?}",
                readings.burnout
            );
        }
    }

    /// The top's bound holds the jump to the next sample either side: a peak of 10 g, 0.1 s wide,
    /// between two samples 0.05 s apart that read 7 g, is within the bound of the reading.
    #[test]
    fn a_peak_between_two_samples_is_within_the_jump() {
        let peaked = |t: f64| {
            if (3.0..4.0).contains(&t) {
                4.0 * G + 6.0 * G * (1.0 - (t - 3.525).abs() / 0.05).max(0.0)
            } else if (4.0..4.1).contains(&t) {
                4.0 * G - 4.6 * G * (t - 4.0) / 0.1
            } else {
                specific_force(t)
            }
        };
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.05,
            |k, t| peaked(t) + jitter(k, 0.3),
        ));
        let top = readings.max_acceleration.value().unwrap();
        assert!(top.acceleration_m_s2 < 7.1 * G, "{top:?}");
        assert!(
            (top.acceleration_m_s2 - 10.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
    }

    /// A sample with no time, then one that repeats the time before it: the sample with no time
    /// is left out, and the repeat with it, as the time it repeats is the last kept.
    #[test]
    fn a_repeat_after_a_sample_with_no_time_is_left_out() {
        let mut log = log(LogFormat::GenericTable, "Axial accel", 0.01, |k, t| {
            specific_force(t) + jitter(k, 0.3)
        });
        let channel = &mut log.channels[0];
        let k = channel.time_s.iter().position(|t| *t >= 3.5).unwrap();
        let repeated = channel.time_s[k];
        channel.time_s.splice(k + 1..k + 1, [f64::NAN, repeated]);
        channel.values.splice(k + 1..k + 1, [100.0 * G, 100.0 * G]);
        let kept = first_at_each_time(&log.channels[0]);
        assert_eq!(kept.time_s.len(), log.channels[0].time_s.len() - 2);
        assert!(kept.time_s.iter().all(|t| t.is_finite()));
        let readings = read_all(&log);
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
    }

    /// The noise of a straight line is 0; of uniform noise, its σ within a few percent.
    #[test]
    fn the_noise_is_the_scatter_about_the_curve() {
        let line: Vec<f64> = (0..100).map(|k| f64::from(k) * 0.5).collect();
        assert!(noise(&line).abs() < 1e-12);
        let scattered: Vec<f64> = (0..20_000).map(|k| jitter(k, 1.0)).collect();
        let sigma = 1.0 / 12.0_f64.sqrt();
        // The robust estimate assumes a normal spread; a uniform one's reads 10% under.
        assert!(
            (noise(&scattered) / sigma - 1.0).abs() < 0.15,
            "{}",
            noise(&scattered)
        );
    }
}
