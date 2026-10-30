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
//! bad sample, and keeps a step or a steady run as it was.
//!
//! **Which way up, by three witnesses** ([`direction`]), each read on the trace as written, each
//! from physics that holds whether or not the part takes gravity out, within
//! [`GRAVITY_TOLERANCE_G`]. The pad: a part at rest reads 1 g nose up and −1 g nose down where it
//! holds gravity, and 0 either way where it doesn't; it is read as gravity's pad is (below), but
//! ends where the trace first moves from it either way. The coast's last [`COAST_SPAN_S`] before
//! apogee's earliest time: a part reads `−D/m` there, or `−D/m − g` without gravity, never above
//! 0 nose up and never below it nose down, so 1 g less the tolerance or more below 0 says nose up
//! and as far above it nose down, a drag only pushing it further; it is read only where a coast
//! was flown, from after liftoff's latest time and before an apogee the log doesn't end at. The
//! thrust ([`thrust_witness`]): the first span of the cleaned trace, either way up, from liftoff's
//! earliest time, that holds past [`LIFTOFF_ACCELERATION_M_S2`] for [`BURN_LEAST_S`], found as
//! the burns are (below), unless a span the other way comes before it, as a short burn before a
//! heavy drag or a knock's rebound does; where none holds that long, the way every span shares, a
//! short burn's. As the thrust is what lifts the rocket, the span that says must rise by
//! liftoff's latest time, or, where liftoff is withheld as a log that starts in the climb has
//! it, within [`BURN_LEAST_S`] of the log's start; one that rises later is a drag after a weak
//! thrust, or a second burn. A pad or a coast near 0, or spans both ways or none, say nothing. The way is
//! settled when the pad or the thrust says and no witness disagrees: a coast's second can hold a
//! weak thrust still burning or a canopy opened early, either of which reads the other way, so a
//! coast alone settles nothing, though it can disagree; otherwise, or where the trace so
//! turned, gravity put back, never passes [`LIFTOFF_ACCELERATION_M_S2`] and so shows no thrust,
//! both readings are withheld ([`Reason::DirectionUnstated`]).
//!
//! **Gravity.** Its 1 g is put back where the logger took it out. Three witnesses, on the trace
//! turned, say whether it did, each within [`GRAVITY_TOLERANCE_G`]: the family's word; the pad,
//! where about 1 g holds gravity and about 0 doesn't; and the coast's last [`COAST_SPAN_S`] before
//! apogee, where a rocket falls freely but for a small drag, so about 0 holds gravity and about
//! −1 g doesn't. The pad is read off the accelerometer itself, the steady reading before the
//! thrust lifts it by half [`LIFTOFF_ACCELERATION_M_S2`], if that comes no later than liftoff's
//! latest time by the heights; a pad or a coast that reads neither, such as a log that starts in
//! the thrust, says nothing. Gravity is settled when one witness or more says and none disagrees;
//! otherwise nothing is read ([`Reason::GravityUnstated`]). A sample that repeats the time before
//! it is left out, as the primary track leaves it ([`first_at_each_time`]).
//!
//! **The top acceleration** is the highest reading over the burns: each from a cleaned reading past
//! [`LIFTOFF_ACCELERATION_M_S2`] to where the cleaned trace's mean over [`BURNOUT_MEAN_S`] next
//! falls through zero, or apogee if it doesn't, that holds past [`LIFTOFF_ACCELERATION_M_S2`] for
//! [`BURN_LEAST_S`], as a shock doesn't, or the first such span where none does; searched from
//! liftoff's earliest time by the heights, as a rocket at rest reads 1 g and anything more before
//! it moves is a knock on the pad, or from the log's start where liftoff is withheld. In a burn
//! the reading is the thrust less the drag, over the mass. A charge or a canopy's shock in a coast lies outside the burns, and
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

use super::interval;
use super::{
    ACCEL_BIAS_M_S2, ACCEL_NOISE_SIGMAS, ACCEL_SCALE_SHARE, Accelerometer, Apogee, BURN_LEAST_S,
    BURNOUT_MEAN_S, Burnout, COAST_SPAN_S, Clip, FLAT_TOP_M_S2, FLAT_TOP_S, FLAT_TOP_SAMPLES,
    FLAT_TOP_SHARE, GRAVITY_TOLERANCE_G, Gravity, GravityBy, LIFTOFF_ACCELERATION_M_S2, Liftoff,
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

/// Whether `channel` is AltOS's `acceleration` from a board with no accelerometer of its own.
/// AltosUI's CSV writes the column for every board. An EasyMini or a TeleMini has a barometer
/// and no accelerometer (*The Altus Metrum System*'s specifications; the `.eeprom` reader's
/// `Board::EasyMini`), so on one the column is the board's estimate from its barometer, not a
/// sensor's reading. Each AltOS board with an axial accelerometer carries an inertial unit
/// (TeleMega, EasyMega) or a GPS receiver (TeleMetrum), whose columns the export adds, so a file
/// with neither holds no accelerometer's reading. A TeleGPS, a GPS receiver with no
/// accelerometer, would pass if its export wrote the column.
fn from_the_barometer(log: &FlightLog, channel: &Channel) -> bool {
    log.format == LogFormat::AltosCsv
        && channel.column == "acceleration"
        && !log
            .channels
            .iter()
            .any(|other| matches!(other.column.as_str(), "accel_x" | "latitude"))
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
    let accelerometers: Vec<_> = log
        .channels_of(Quantity::Acceleration)
        .filter(|channel| !from_the_barometer(log, channel))
        .collect();
    if accelerometers.is_empty() {
        let detail = if log
            .channels_of(Quantity::Acceleration)
            .any(|channel| from_the_barometer(log, channel))
        {
            "the log has no accelerometer. The file holds neither an inertial unit's columns \
             (`accel_x` and on) nor a GPS receiver's. Every AltOS board with an axial \
             accelerometer adds one of the two. So the file's `acceleration` is taken for a board with \
             none of its own, an EasyMini's or a TeleMini's. That column is the board's \
             estimate from its barometer, and HPR Sim doesn't read a top from it"
        } else {
            "the log has no accelerometer; HPR Sim doesn't difference a height twice to make \
             one, as its steps would read as spikes of many g"
        };
        return (
            None,
            Reading::withheld(Reason::NoAccelerometer, detail),
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
    // Which way up the part reads, by three witnesses on the trace as written, before it is
    // turned: the pad, the coast to apogee and the thrust ([`direction`]). The pad ends where
    // the trace first moves from it either way. The coast is read only where one was flown:
    // after liftoff's latest time by the heights and after the trace first moves from its pad,
    // as a coast follows the thrust, and before an apogee the log doesn't end at, as a log that
    // ends in the climb shows no coast; otherwise the second before apogee's earliest time may
    // be the pad's or the thrust's.
    let either_way = |moved: f64| moved.abs() > 0.5 * LIFTOFF_ACCELERATION_M_S2;
    let moved = first_move(&raw, &clean, either_way);
    // Where liftoff is withheld, as a log that starts in the climb has it, the log's start
    // stands for liftoff's latest time.
    let flown = match &heights.apogee {
        Reading::Read(apogee) => {
            let start_s = apogee.time_s - apogee.time_bound_s - COAST_SPAN_S;
            let lifted_s = heights
                .liftoff
                .value()
                .map_or(time[0], |liftoff| liftoff.time_s + liftoff.bound_s);
            !apogee.is_floor
                && start_s >= lifted_s
                && moved.is_some_and(|moved| time[moved] <= start_s)
        }
        _ => false,
    };
    let (way, said_by) = match direction(
        pad_level(&time, &raw, &clean, &heights.liftoff, either_way),
        coast_level(&time, &raw, &boost, &heights.apogee).filter(|_| flown),
        &thrust_witness(
            &time,
            &clean,
            from,
            end,
            half_mean,
            interval,
            heights.liftoff.value().map_or(
                (
                    time[0] + BURN_LEAST_S,
                    "the log's start by 0.5 s, liftoff withheld",
                    false,
                ),
                |liftoff| {
                    (
                        liftoff.time_s + liftoff.bound_s,
                        "liftoff's latest time",
                        true,
                    )
                },
            ),
        ),
    ) {
        Ok(settled) => settled,
        Err(detail) => {
            return withheld(
                Reason::DirectionUnstated,
                format!("the accelerometer `{}`: {detail}", channel.column),
            );
        }
    };
    let sign = way.sign();
    let turned: Vec<f64> = raw.iter().map(|v| sign * v).collect();
    let turned_clean: Vec<f64> = clean.iter().map(|v| sign * v).collect();
    let g = STANDARD_GRAVITY_MPS2;
    let tolerance = GRAVITY_TOLERANCE_G * g;
    // The pad, on the accelerometer's own trace turned: a rocket at rest reads a steady level,
    // which the thrust lifts by [`LIFTOFF_ACCELERATION_M_S2`] or more once the rocket moves. So
    // the pad is the samples from the start until the first that reads half that above the
    // start's level, if that comes no later than liftoff's latest time by the heights.
    let pad = pad_level(&time, &turned, &turned_clean, &heights.liftoff, |moved| {
        moved > 0.5 * LIFTOFF_ACCELERATION_M_S2
    });
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
    let coast = coast_level(&time, &turned, &boost, &heights.apogee);
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
                    || format!("no {name} is read to tell by"),
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
    let f: Vec<f64> = turned.iter().map(|v| v + gravity_m_s2).collect();
    let f_clean: Vec<f64> = turned_clean.iter().map(|v| v + gravity_m_s2).collect();
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
    // A trace so turned, gravity put back, that never passes [`LIFTOFF_ACCELERATION_M_S2`]
    // shows no thrust, as a rocket leaving the pad does, and no top to read: which way up the
    // witnesses settled is then borne out by nothing.
    if spans.is_empty() {
        let by: Vec<&str> = said_by.iter().map(|by| by.words()).collect();
        return withheld(
            Reason::DirectionUnstated,
            format!(
                "the accelerometer `{}`, read {} as its {} say{} and with gravity put back. It \
                 never reads past {:.0} g from liftoff's earliest time to apogee, as a rocket \
                 leaving the pad does. The earliest time is the log's start where liftoff is \
                 withheld. The accelerometer shows no thrust to read the top or burnout from",
                channel.column,
                way.words(),
                match by.split_last() {
                    Some((last, rest)) if !rest.is_empty() => {
                        format!("{} and {last}", rest.join(", "))
                    }
                    _ => by.join(""),
                },
                if by.len() == 1 { "s" } else { "" },
                LIFTOFF_ACCELERATION_M_S2 / g
            ),
        );
    }
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
    // apogee, so the top may lie past the samples.
    let ended = burns.last().is_some_and(|burn| burn.falls);
    // Whether the thrust begins on the channel: the sample before the first burn's rise reads
    // under 2 g. Where it doesn't, the log starts in the thrust, or the search for it does, so
    // the top may have come before the samples searched, as an ignition spike before a log that
    // starts in the climb would.
    let began = burns
        .first()
        .is_none_or(|burn| burn.rise > 0 && f_clean[burn.rise - 1] < LIFTOFF_ACCELERATION_M_S2);
    let thrust: Vec<usize> = burns
        .iter()
        .flat_map(|burn| burn.rise..=burn.fall)
        .collect();
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
    let is_floor = clip.is_some() || !ended || !began;
    let max_acceleration = Reading::Read(MaxAcceleration {
        acceleration_m_s2: top,
        bound_m_s2,
        interval_m_s2: (!is_floor)
            .then(|| interval::TOP_ACCELERATION_SHARES.interval(top, bound_m_s2))
            .flatten(),
        lower_end_m_s2: is_floor
            .then(|| interval::TOP_ACCELERATION_FLOOR_SHARES.lower_end(top, bound_m_s2))
            .flatten(),
        time_s: time[top_index],
        range_m_s2,
        is_floor,
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
             before {}. The thrust doesn't end on the channel, so it shows no burnout",
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
            interval_s: interval::BURNOUT_SHARES.interval(time_s, bound_s),
            lower_end_s: None,
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

/// Which way up an accelerometer reads: nose up, as the rocket's axis points in the climb, or
/// nose down, as a part mounted the other way writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Way {
    Up,
    Down,
}

impl Way {
    /// The sign that turns a reading written this way up to nose up.
    fn sign(self) -> f64 {
        match self {
            Self::Up => 1.0,
            Self::Down => -1.0,
        }
    }

    /// The other way up.
    fn other(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    fn words(self) -> &'static str {
        match self {
            Self::Up => "nose up",
            Self::Down => "nose down",
        }
    }
}

/// A witness to which way up an accelerometer reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WayBy {
    Pad,
    Coast,
    Thrust,
}

impl WayBy {
    fn words(self) -> &'static str {
        match self {
            Self::Pad => "pad",
            Self::Coast => "coast to apogee",
            Self::Thrust => "thrust",
        }
    }
}

/// What the thrust says of which way up the part reads, `None` for nothing, and the words that
/// say why; `alone` where it can settle the way alone: where its span rises by liftoff's latest
/// time, not at the start of a log whose liftoff is withheld, which may start after burnout,
/// with a drag there.
#[derive(Debug, Clone, PartialEq)]
struct ThrustWitness {
    way: Option<Way>,
    words: String,
    alone: bool,
}

/// The first sample of `clean`, the cleaned twin of `values`, that `moved` says has left the
/// start's level, the median of the first [`PAD_MIN_SAMPLES`] of `values`, by how far it lies
/// from it; `None` if none does.
fn first_move(values: &[f64], clean: &[f64], moved: impl Fn(f64) -> bool) -> Option<usize> {
    let mut start: Vec<f64> = values.iter().take(PAD_MIN_SAMPLES).copied().collect();
    let level = median(&mut start).unwrap_or(f64::NAN);
    (0..clean.len()).find(|&k| moved(clean[k] - level))
}

/// The steady level of `values` on the pad, as the median of the samples from the start to the
/// [`first_move`] that `moved` says, if that sample is the [`PAD_MIN_SAMPLES`]th or later and
/// comes no later than liftoff's latest time by the heights; `None` otherwise, or where liftoff
/// is withheld. A log that starts in the thrust has no pad before its latest time.
fn pad_level(
    time: &[f64],
    values: &[f64],
    clean: &[f64],
    liftoff: &Reading<Liftoff>,
    moved: impl Fn(f64) -> bool,
) -> Option<f64> {
    let Reading::Read(liftoff) = liftoff else {
        return None;
    };
    let rise = first_move(values, clean, moved)?;
    if rise >= PAD_MIN_SAMPLES && time[rise] <= liftoff.time_s + liftoff.bound_s {
        median(&mut values[..rise].to_vec())
    } else {
        None
    }
}

/// The median of `values` over the climb's samples `boost` in the last [`COAST_SPAN_S`] before
/// apogee's earliest time, if [`PAD_MIN_SAMPLES`] or more lie there; `None` where apogee is
/// withheld.
fn coast_level(
    time: &[f64],
    values: &[f64],
    boost: &[usize],
    apogee: &Reading<Apogee>,
) -> Option<f64> {
    let Reading::Read(apogee) = apogee else {
        return None;
    };
    let end = apogee.time_s - apogee.time_bound_s;
    let mut readings: Vec<f64> = boost
        .iter()
        .filter(|&&k| time[k] >= end - COAST_SPAN_S && time[k] <= end)
        .map(|&k| values[k])
        .collect();
    if readings.len() >= PAD_MIN_SAMPLES {
        median(&mut readings)
    } else {
        None
    }
}

/// The thrust's witness to which way up the accelerometer reads, from its cleaned trace `clean`
/// as written, its samples `from` (liftoff's earliest time) to `end` (apogee) at `interval`,
/// averaged over `half` samples either side to end a span, its times `time`. The first span,
/// either way up, that holds past [`LIFTOFF_ACCELERATION_M_S2`] for [`BURN_LEAST_S`], as a
/// thrust does and a knock on the pad doesn't, says its way, unless a span the other way comes
/// before it, as a short burn before a heavy drag or a knock's rebound does: then it says
/// nothing. Where no span holds that long, the way every span shares says, a short burn's; spans
/// both ways, or none, say nothing. The thrust is what lifts the rocket, so the span that says
/// must rise by `lifted`'s time: liftoff's latest time by the heights, or, where liftoff is
/// withheld as a log that starts in the climb has it, [`BURN_LEAST_S`] after the log's start,
/// `lifted`'s words naming which. A span that rises later is a drag after a thrust too weak to
/// pass 2 g as written, or a second burn, and says nothing. A log whose liftoff is withheld may
/// start after burnout, its drag at its start, so a span there says, but not `alone`. Read as
/// written, a thrust under 3 g on a part that takes gravity out reads under 2 g, and says
/// nothing: the pad must say.
fn thrust_witness(
    time: &[f64],
    clean: &[f64],
    from: usize,
    end: usize,
    half: usize,
    interval: f64,
    lifted: (f64, &str, bool),
) -> ThrustWitness {
    let walk = |way: Way| {
        let values: Vec<f64> = clean.iter().map(|v| way.sign() * v).collect();
        let mean = running_mean(&values, half);
        spans(&values, &mean, from, end, interval)
    };
    let (up, down) = (walk(Way::Up), walk(Way::Down));
    let held = |spans: &[Span]| {
        spans
            .iter()
            .find(|span| span.held_s >= BURN_LEAST_S)
            .map(|span| span.rise)
    };
    let first_held = match (held(&up), held(&down)) {
        (Some(up), Some(down)) if down < up => Some((Way::Down, down)),
        (Some(up), _) => Some((Way::Up, up)),
        (None, Some(down)) => Some((Way::Down, down)),
        (None, None) => None,
    };
    let past = format!("{:.0} g", LIFTOFF_ACCELERATION_M_S2 / STANDARD_GRAVITY_MPS2);
    let (lifted_s, lifted_by, alone) = lifted;
    let says = |way: Way, words: String| ThrustWitness {
        way: Some(way),
        words: format!("its thrust says {}: {words}", way.words()),
        alone,
    };
    let neither = |words: String| ThrustWitness {
        way: None,
        words: format!("its thrust says neither: {words}"),
        alone,
    };
    // The span that would say rises by liftoff's latest time, as the thrust that lifts the
    // rocket does, or says nothing.
    let lifts = |way: Way, rise: usize, words: String| {
        if time[rise] <= lifted_s {
            says(way, words)
        } else {
            neither(format!(
                "its span that would say {}, from {:.2} s, rises after {lifted_by}, \
                 {lifted_s:.2} s. A drag after a thrust too weak to read past {past} or a \
                 second burn does so. That span is not the thrust that lifts the rocket",
                way.words(),
                time[rise]
            ))
        }
    };
    if let Some((way, rise)) = first_held {
        let other = match way {
            Way::Up => &down,
            Way::Down => &up,
        };
        return match other.first().filter(|span| span.rise < rise) {
            Some(before) => neither(format!(
                "the first span past {past} to hold there for {BURN_LEAST_S} s, {} from {:.2} s, \
                 comes after one {} from {:.2} s. A drag after a short burn or a knock's \
                 rebound may do so",
                way.words(),
                time[rise],
                way.other().words(),
                time[before.rise]
            )),
            None => lifts(
                way,
                rise,
                format!(
                    "the first span past {past} to hold there for {BURN_LEAST_S} s, from {:.2} s",
                    time[rise]
                ),
            ),
        };
    }
    let count = |spans: usize| match spans {
        1 => "1 span".to_owned(),
        spans => format!("{spans} spans"),
    };
    match (up.len(), down.len()) {
        (0, 0) => neither(format!(
            "as written, it never reads past {past} either way up from liftoff's earliest time \
             (the log's start where liftoff is withheld) to apogee"
        )),
        (spans, 0) | (0, spans) => {
            let (way, first) = if down.is_empty() {
                (Way::Up, &up[0])
            } else {
                (Way::Down, &down[0])
            };
            lifts(
                way,
                first.rise,
                format!(
                    "no span past {past} holds there for {BURN_LEAST_S} s, and its {} {}, as a \
                     short burn's does",
                    count(spans),
                    if spans == 1 {
                        "reads so"
                    } else {
                        "all read so"
                    }
                ),
            )
        }
        (up, down) => neither(format!(
            "it reads past {past} both ways up, in {} nose up and {} nose down, and never for \
             {BURN_LEAST_S} s, as a thrust holds",
            count(up),
            count(down)
        )),
    }
}

/// Which way up the accelerometer reads, by three witnesses, each on its trace as written and
/// each from physics that holds whether or not the part takes gravity out:
/// - the pad, its steady level `pad` before the thrust: at rest a part reads 1 g nose up and
///   −1 g nose down where it holds gravity, and 0 either way where it doesn't, so a pad within
///   [`GRAVITY_TOLERANCE_G`] of 1 g says nose up, of −1 g nose down, and of 0 nothing;
/// - the coast's last [`COAST_SPAN_S`] before apogee's earliest time, its median `coast`, where
///   one was flown: in a coast a part reads `−D/m` nose up where it holds gravity and
///   `−D/m − g` where it doesn't, never above 0 while it coasts with its thrust ended and no
///   canopy open, and the mirror nose down, so a coast at 1 g
///   less the tolerance or more below 0 says nose up, as far above 0 nose down, a drag only
///   pushing it further, and one nearer 0 nothing;
/// - the thrust ([`thrust_witness`]).
///
/// Settled when the pad or the thrust says and no witness disagrees: the way and the witnesses
/// that said it, or the words that say what each said. The coast can only disagree: its second
/// can hold a weak thrust still burning or a canopy opened early, either of which reads the
/// other way, so a coast alone settles nothing.
fn direction(
    pad: Option<f64>,
    coast: Option<f64>,
    thrust: &ThrustWitness,
) -> Result<(Way, Vec<WayBy>), String> {
    let g = STANDARD_GRAVITY_MPS2;
    let tolerance = GRAVITY_TOLERANCE_G * g;
    let by_pad = pad.and_then(|pad| {
        if (pad - g).abs() <= tolerance {
            Some(Way::Up)
        } else if (pad + g).abs() <= tolerance {
            Some(Way::Down)
        } else {
            None
        }
    });
    let by_coast = coast.and_then(|coast| {
        if coast <= -(g - tolerance) {
            Some(Way::Up)
        } else if coast >= g - tolerance {
            Some(Way::Down)
        } else {
            None
        }
    });
    let said: Vec<(Way, WayBy)> = [
        (by_pad, WayBy::Pad),
        (by_coast, WayBy::Coast),
        (thrust.way, WayBy::Thrust),
    ]
    .iter()
    .filter_map(|(way, by)| way.map(|way| (way, *by)))
    .collect();
    // A coast alone settles nothing, nor a thrust at the start of a log whose liftoff is
    // withheld: the pad can, the thrust where it rises by liftoff's latest time, or a thrust
    // at a log's start with the coast beside it.
    let settles =
        by_pad.is_some() || (thrust.way.is_some() && (thrust.alone || by_coast.is_some()));
    match said.first() {
        Some(&(way, _)) if settles && said.iter().all(|(each, _)| *each == way) => {
            Ok((way, said.iter().map(|(_, by)| *by).collect()))
        }
        _ => {
            let level = |name: &str, level: Option<f64>, said: Option<Way>, marks: &str| {
                level.map_or_else(
                    || format!("no {name} is read to tell by"),
                    |level| {
                        format!(
                            "its {name} reads {:.2} g as written, which says {} ({marks})",
                            level / g,
                            said.map_or("neither", Way::words)
                        )
                    },
                )
            };
            let near = 1.0 - GRAVITY_TOLERANCE_G;
            Err(format!(
                "{}; {}; {}: {}",
                level(
                    "pad",
                    pad,
                    by_pad,
                    "about 1 g says nose up, about −1 g nose down"
                ),
                level(
                    "coast to apogee",
                    coast,
                    by_coast,
                    &format!("−{near} g or less says nose up, {near} g or more nose down")
                ),
                thrust.words,
                if said.is_empty() {
                    "none of them says which way up it reads"
                } else if !settles && thrust.way.is_none() {
                    "the coast alone says. A coast's second can hold a weak thrust still \
                     burning or a canopy opened early. So the coast doesn't settle which way up it \
                     reads on its own"
                } else if !settles {
                    "the thrust alone says, at the start of a log whose liftoff is withheld, which \
                     may start after burnout with a drag there. So the thrust doesn't settle which way \
                     up it reads without the coast beside it"
                } else {
                    "they disagree on which way up it reads"
                }
            ))
        }
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
             its noise and step, before apogee. Burnout can't be told from the coast",
            4.0 * n
        ));
    };
    if least.0 > -3.0 * n {
        return Err(format!(
            "the accelerometer's least reading after the thrust, {:.2} m/s², lies within {:.2} \
             m/s², three times its noise and step, of zero. That is too little drag to tell \
             burnout by",
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
        Reading::Read(speed) => {
            let bound_s = 2.0 * SPEED_NOISE_M_S / STANDARD_GRAVITY_MPS2
                + 0.5 * SPEED_WINDOW_S
                + heights.sample_interval_s.unwrap_or(0.0);
            Reading::Read(Burnout {
                time_s: speed.time_s,
                bound_s,
                interval_s: None,
                lower_end_s: interval::BURNOUT_FLOOR_SHARES.lower_end(speed.time_s, bound_s),
                time_after_liftoff_s: heights.liftoff.value().map(|l| speed.time_s - l.time_s),
                is_floor: true,
                source: speed.source,
                provenance: Provenance::Derived,
                method: Method::SpeedPeak,
            })
        }
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
        let mut log = FlightLog {
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
        };
        // An AltOS log is a TeleMetrum's: its GPS receiver's column beside the accelerometer.
        if format == LogFormat::AltosCsv {
            let time_s = log.time_s.clone();
            let latitude = vec![40.0; time_s.len()];
            log.channels.push(Channel::new(
                "latitude",
                Quantity::Latitude,
                time_s,
                latitude,
            ));
        }
        log
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
        // Whole, each states its 90% interval by its own shares, and no lower end.
        assert_eq!(
            top.interval_m_s2,
            interval::TOP_ACCELERATION_SHARES.interval(top.acceleration_m_s2, top.bound_m_s2)
        );
        assert!(top.interval_m_s2.is_some() && top.lower_end_m_s2.is_none());
        let stated = readings.burnout.value().unwrap();
        assert_eq!(
            stated.interval_s,
            interval::BURNOUT_SHARES.interval(stated.time_s, stated.bound_s)
        );
        assert!(stated.interval_s.is_some() && stated.lower_end_s.is_none());
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
        // A floor states its lower end by the floor's shares, and no interval.
        assert_eq!(
            burnout.lower_end_s,
            interval::BURNOUT_FLOOR_SHARES.lower_end(burnout.time_s, burnout.bound_s)
        );
        assert_eq!(
            burnout.lower_end_s,
            Some(burnout.time_s + 0.0339 * burnout.bound_s)
        );
        assert!(burnout.interval_s.is_none());
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

    /// Three traces that show no thrust, each withheld as [`Reason::DirectionUnstated`], burnout
    /// falling back to the top speed's floor, the words saying why. One reads 1 g throughout: it
    /// never moves from its pad, so neither the pad nor the coast after a thrust says, and nor
    /// does the thrust. One passes 2 g both ways up and never holds there for [`BURN_LEAST_S`],
    /// so the thrust says nothing, and its pad's 1 g, nose up, and its coast's 1 g, nose down,
    /// disagree. One dips on the pad, so its pad's 1 g says nose up, then pushes at 1.8 g: so
    /// turned, the trace never passes 2 g, as a rocket leaving the pad does.
    #[test]
    fn a_trace_that_shows_no_thrust_either_way_up_is_withheld() {
        let pulse = |t: f64, at: f64| {
            if (at..at + 0.2).contains(&t) {
                4.0 * G
            } else {
                0.0
            }
        };
        let weak = |t: f64| {
            if (1.0..1.3).contains(&t) {
                -3.0 * G
            } else if t < 3.0 {
                G
            } else if t < 4.5 {
                1.8 * G
            } else if t < 8.0 {
                -0.2 * G * (8.0 - t) / 3.5
            } else {
                G
            }
        };
        for (case, words, trace) in [
            (
                "flat",
                "none of them says",
                Box::new(|_: f64| G) as Box<dyn Fn(f64) -> f64>,
            ),
            (
                "both ways",
                "which says nose down",
                // 5 g for 0.2 s in the thrust, then −5 g for 0.2 s in the coast.
                Box::new(move |t: f64| G + pulse(t, 3.4) - 1.5 * pulse(t, 5.0)),
            ),
            ("weak", "shows no thrust", Box::new(weak)),
        ] {
            let readings = read_all(&log(
                LogFormat::GenericTable,
                "Axial accel",
                0.01,
                |k, t| trace(t) + jitter(k, 0.3),
            ));
            let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
                panic!("{case}: {:?}", readings.max_acceleration);
            };
            assert_eq!(*reason, Reason::DirectionUnstated, "{case}");
            assert!(detail.contains(words), "{case}: {detail}");
            let burnout = readings.burnout.value().unwrap();
            assert_eq!(burnout.method, Method::SpeedPeak, "{case}");
            assert!(burnout.is_floor, "{case}");
        }
    }

    /// A dip on the pad, to −3 g over 0.3 s at 1 s, doesn't turn the trace over: it comes before
    /// liftoff's earliest time, so the thrust, which holds past 2 g for [`BURN_LEAST_S`], says
    /// nose up, as the pad's 1 g does on a column that holds gravity and the coast's −1 g on
    /// AltOS's, which takes it out. The top and burnout are the thrust's.
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
    /// 0.15 s, inside liftoff's bound: the rebound comes before the thrust's span, so the thrust
    /// says nothing, and the pad's 1 g says nose up. The trace isn't turned over, and the top and
    /// burnout are the thrust's, neither before liftoff's earliest time.
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

    /// A short burn, 40 g from 3.0 s to 3.3 s, then a heavy drag, −8 g easing to −2.9 g by 4.5 s
    /// and to nothing by 6 s, gravity in: the specific force along the axis, nose up.
    fn short_burn(t: f64) -> f64 {
        if t < 3.0 {
            G
        } else if t < 3.3 {
            40.0 * G
        } else if t < 4.5 {
            -8.0 * G + 5.1 * G * (t - 3.3) / 1.2
        } else if t < 6.0 {
            -2.9 * G * (6.0 - t) / 1.5
        } else if t < 8.0 {
            0.0
        } else {
            G
        }
    }

    /// A short burn before a heavy drag ([`short_burn`]): the drag's span holds past 2 g for
    /// [`BURN_LEAST_S`] and the burn's doesn't, but the burn's comes first, so the thrust says
    /// nothing; the pad, at 1 g, says nose up where gravity is in, and the top is the burn's
    /// 40 g, not the drag's. Where AltOS takes gravity out, its pad's 0 says nothing and the
    /// coast's −1 g alone settles nothing: withheld, never turned over.
    #[test]
    fn a_short_burn_before_a_heavy_drag_reads_nose_up() {
        let without = read_all(&log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
            short_burn(t) - G + jitter(k, 0.3)
        }));
        let Reading::Withheld(Withheld { reason, detail }) = &without.max_acceleration else {
            panic!("{:?}", without.max_acceleration);
        };
        assert_eq!(*reason, Reason::DirectionUnstated);
        assert!(detail.contains("the coast alone says"), "{detail}");
        let (format, column, out_m_s2) = (LogFormat::GenericTable, "Axial accel", 0.0);
        let readings = read_all(&log(format, column, 0.01, |k, t| {
            short_burn(t) - out_m_s2 + jitter(k, 0.3)
        }));
        let accelerometer = readings.accelerometer.as_ref();
        assert!(
            accelerometer.is_some_and(|a| !a.turned_over),
            "{column}: {accelerometer:?}, {:?}",
            readings.max_acceleration
        );
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 40.0 * G).abs() <= top.bound_m_s2,
            "{column}: {top:?}"
        );
        assert!((3.0..3.3).contains(&top.time_s), "{column}: {top:?}");
        assert_eq!(top.provenance, Provenance::Measured, "{column}");
        assert!(!top.is_floor, "{column}: {top:?}");
        let burnout = readings.burnout.value().unwrap();
        assert!(
            (burnout.time_s - 3.3).abs() <= burnout.bound_s,
            "{column}: {burnout:?}"
        );
    }

    /// The same short burn and drag on a part mounted nose down: the pad's −1 g says so where
    /// gravity is in, and the top is the burn's 40 g. Where gravity is out, the coast's +1 g
    /// alone settles nothing: withheld.
    #[test]
    fn a_short_burn_read_nose_down_reads_alike() {
        let without = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| -(short_burn(t) - G) + jitter(k, 0.3),
        ));
        assert_eq!(
            without.max_acceleration.reason(),
            Some(Reason::DirectionUnstated)
        );
        let out_m_s2 = 0.0;
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| -(short_burn(t) - out_m_s2) + jitter(k, 0.3),
        ));
        let accelerometer = readings.accelerometer.as_ref();
        assert!(
            accelerometer.is_some_and(|a| a.turned_over),
            "out {out_m_s2}: {accelerometer:?}, {:?}",
            readings.max_acceleration
        );
        let top = readings.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 40.0 * G).abs() <= top.bound_m_s2,
            "out {out_m_s2}: {top:?}"
        );
        assert!(!top.is_floor, "out {out_m_s2}: {top:?}");
    }

    /// A pad that reads 1 g, nose up with gravity in, before a thrust that reads −6 g, nose
    /// down: the witnesses disagree, so the top and the accelerometer's burnout are withheld,
    /// the words naming each witness, and burnout comes from the top speed.
    #[test]
    fn a_pad_that_disagrees_with_the_thrust_is_withheld() {
        let readings = read_all(&log(
            LogFormat::GenericTable,
            "Axial accel",
            0.01,
            |k, t| if t < 3.0 { G } else { -specific_force(t) } + jitter(k, 0.3),
        ));
        let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
            panic!("{:?}", readings.max_acceleration);
        };
        assert_eq!(*reason, Reason::DirectionUnstated);
        for words in [
            "its pad reads 1.00 g as written, which says nose up",
            "its coast to apogee reads",
            "its thrust says nose down",
            "they disagree",
        ] {
            assert!(detail.contains(words), "{words}: {detail}");
        }
        assert!(readings.accelerometer.is_none());
        let burnout = readings.burnout.value().unwrap();
        assert_eq!(burnout.method, Method::SpeedPeak);
        assert!(burnout.is_floor);
    }

    /// A thrust of 2.5 g, a thrust-to-weight of about 2.5, on AltOS, which takes gravity out:
    /// as written it reads 1.5 g, under 2 g, so the thrust says nothing, and the pad's 0 says
    /// nothing either; the coast's −1 g, less the drag, says nose up, but a coast alone settles
    /// nothing: withheld. Where the drag after burnout is heavy, −1.5 g, it reads past −2 g for
    /// over [`BURN_LEAST_S`] as written, but rises after liftoff's latest time, so it isn't the
    /// thrust and says nothing: withheld again. Never turned over.
    #[test]
    fn a_weak_thrust_without_gravity_is_withheld_not_turned() {
        let weak = |drag_g: f64| {
            move |t: f64| {
                if t < 3.0 {
                    G
                } else if t < 4.5 {
                    2.5 * G
                } else if t < 8.0 {
                    -drag_g * G * (8.0 - t) / 3.5
                } else {
                    G
                }
            }
        };
        let light = weak(0.4);
        let readings = read_all(&log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
            light(t) - G + jitter(k, 0.3)
        }));
        let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
            panic!("{:?}", readings.max_acceleration);
        };
        assert_eq!(*reason, Reason::DirectionUnstated);
        assert!(detail.contains("which says nose up"), "{detail}");
        assert!(detail.contains("the coast alone says"), "{detail}");
        let burnout = readings.burnout.value().unwrap();
        assert_eq!(burnout.method, Method::SpeedPeak, "{burnout:?}");
        assert!(burnout.is_floor);
        let heavy = weak(1.5);
        let readings = read_all(&log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
            heavy(t) - G + jitter(k, 0.3)
        }));
        let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
            panic!("{:?}", readings.max_acceleration);
        };
        assert_eq!(*reason, Reason::DirectionUnstated);
        assert!(detail.contains("which says nose up"), "{detail}");
        assert!(
            detail.contains("rises after liftoff's latest time"),
            "{detail}"
        );
        assert!(detail.contains("the coast alone says"), "{detail}");
    }

    /// A thrust of 2.5 g on AltOS, which takes gravity out, reads 1.5 g as written, under 2 g;
    /// the drag after it, 1.5 g easing to apogee, reads past −2 g as written for over
    /// [`BURN_LEAST_S`]. With the log cut short of apogee no coast is read, and the pad's 0 says
    /// nothing, so were the drag taken for the thrust the trace would be turned over and the
    /// drag read as a 3.5 g top: its span rises after liftoff's latest time, so it says
    /// nothing, and the top is withheld at every cut, never turned over.
    #[test]
    fn a_drag_after_a_weak_thrust_isnt_the_thrust() {
        let weak = |t: f64| {
            if t < 3.0 {
                G
            } else if t < 4.5 {
                2.5 * G
            } else if t < 8.0 {
                -1.5 * G * (8.0 - t) / 3.5
            } else {
                G
            }
        };
        for end_s in [5.6, 6.5, 7.0, 7.5] {
            let mut log = log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
                weak(t) - G + jitter(k, 0.3)
            });
            let cut = |time_s: &[f64]| time_s.partition_point(|t| *t <= end_s + 1e-9);
            let channel = &mut log.channels[0];
            let kept = cut(&channel.time_s);
            channel.time_s.truncate(kept);
            channel.values.truncate(kept);
            let kept = cut(&log.time_s);
            log.time_s.truncate(kept);
            log.altitude_m.truncate(kept);
            let readings = read_all(&log);
            assert_eq!(
                readings.max_acceleration.reason(),
                Some(Reason::DirectionUnstated),
                "to {end_s} s: {:?}",
                readings.max_acceleration
            );
        }
    }

    /// AltOS's `acceleration` from a board with no accelerometer, an EasyMini's, is its
    /// barometer's estimate: with neither a GPS receiver's column nor an inertial unit's beside
    /// it, no top or burnout is read from it, and burnout comes from the top speed. Either column
    /// makes it a sensor's.
    #[test]
    fn altos_acceleration_without_an_accelerometer_board_isnt_read() {
        let sensor = log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
            specific_force(t) - G + jitter(k, 0.3)
        });
        assert!(read_all(&sensor).max_acceleration.value().is_some());
        let mut barometer = sensor.clone();
        barometer
            .channels
            .retain(|channel| channel.column != "latitude");
        let readings = read_all(&barometer);
        let Reading::Withheld(withheld) = &readings.max_acceleration else {
            panic!("{:?}", readings.max_acceleration);
        };
        assert_eq!(withheld.reason, Reason::NoAccelerometer);
        assert!(
            withheld.detail.contains("estimate from its barometer"),
            "{withheld:?}"
        );
        assert!(readings.accelerometer.is_none());
        let burnout = readings.burnout.value().expect("from the top speed");
        assert_eq!(burnout.method, Method::SpeedPeak, "{burnout:?}");
        let mut inertial = barometer.clone();
        let (time_s, values) = (inertial.time_s.clone(), vec![9.8; inertial.time_s.len()]);
        inertial.channels.push(Channel::new(
            "accel_x",
            Quantity::Acceleration,
            time_s,
            values,
        ));
        assert!(read_all(&inertial).max_acceleration.value().is_some());
        // A receiver with no fix leaves its column all gaps: still a TeleMetrum's.
        let mut unfixed = sensor.clone();
        for channel in &mut unfixed.channels {
            if channel.column == "latitude" {
                channel.values.fill(f64::NAN);
            }
        }
        assert!(read_all(&unfixed).max_acceleration.value().is_some());
    }

    /// A log that starts in the thrust, past its 6 g top at 3.5 s: from 3.6 s the trace is
    /// already past 2 g at its first sample, so the thrust began before the log and its top may
    /// have come earlier, as it did: the top read is a floor, the truth no lower than it less
    /// its bound. So on AltOS with the whole log cut, liftoff withheld, and on a column that
    /// holds gravity with only the accelerometer cut, liftoff read by the heights.
    #[test]
    fn a_log_that_starts_in_the_thrust_gives_its_top_as_a_floor() {
        for (format, column, out_m_s2, whole) in [
            (LogFormat::AltosCsv, "acceleration", G, true),
            (LogFormat::GenericTable, "Axial accel", 0.0, false),
        ] {
            let mut log = log(format, column, 0.01, |k, t| {
                specific_force(t) - out_m_s2 + jitter(k, 0.3)
            });
            let from = |time_s: &[f64]| time_s.partition_point(|t| *t < 3.6);
            let channel = &mut log.channels[0];
            let cut = from(&channel.time_s);
            channel.time_s.drain(..cut);
            channel.values.drain(..cut);
            if whole {
                let cut = from(&log.time_s);
                log.time_s.drain(..cut);
                log.altitude_m.drain(..cut);
            }
            let readings = read_all(&log);
            let top = readings.max_acceleration.value();
            let top = top.unwrap_or_else(|| panic!("{column}: {:?}", readings.max_acceleration));
            assert!(top.is_floor, "{column}: {top:?}");
            assert_eq!(top.clip, None, "{column}");
            assert!(
                top.acceleration_m_s2 - top.bound_m_s2 <= 6.0 * G,
                "{column}: {top:?}"
            );
            // A floor states its lower end, inside the bound's, and no interval.
            assert_eq!(
                top.lower_end_m_s2,
                Some(top.acceleration_m_s2 - 0.223 * top.bound_m_s2),
                "{column}"
            );
            assert!(top.interval_m_s2.is_none(), "{column}");
        }
    }

    /// A log whose liftoff is withheld, as one that starts in the climb has it: its thrust's
    /// span, at its start, may be a drag where the log starts after burnout, so it settles the
    /// way only with the coast beside it. On AltOS, which takes gravity out, a log from 3.35 s,
    /// after liftoff, reads its 6 g top at 3.5 s nose up, its coast agreeing; logs that
    /// start at 4.6 s, after burnout, their drag at their start (a 2.5 g thrust's to apogee, or
    /// a 6 g thrust's cut at 7 s), are withheld, never turned over.
    #[test]
    fn a_thrust_at_a_log_start_settles_only_beside_the_coast() {
        let trimmed = |trace: &dyn Fn(f64) -> f64, dt: f64, from_s: f64, to_s: f64| {
            let mut log = log(LogFormat::AltosCsv, "acceleration", dt, |k, t| {
                trace(t) - G + jitter(k, 0.3)
            });
            let keep = |time_s: &[f64]| {
                (
                    time_s.partition_point(|t| *t < from_s - 1e-9),
                    time_s.partition_point(|t| *t <= to_s + 1e-9),
                )
            };
            let channel = &mut log.channels[0];
            let (a, b) = keep(&channel.time_s);
            channel.time_s = channel.time_s[a..b].to_vec();
            channel.values = channel.values[a..b].to_vec();
            let (a, b) = keep(&log.time_s);
            log.time_s = log.time_s[a..b].to_vec();
            log.altitude_m = log.altitude_m[a..b].to_vec();
            read_all(&log)
        };
        let started = trimmed(&specific_force, 0.01, 3.35, f64::INFINITY);
        assert!(started.liftoff.reason().is_some(), "{:?}", started.liftoff);
        let accelerometer = started.accelerometer.as_ref();
        assert!(
            accelerometer.is_some_and(|a| !a.turned_over),
            "{accelerometer:?}, {:?}",
            started.max_acceleration
        );
        let top = started.max_acceleration.value().unwrap();
        assert!(
            (top.acceleration_m_s2 - 6.0 * G).abs() <= top.bound_m_s2,
            "{top:?}"
        );
        let weak = |t: f64| {
            if t < 3.0 {
                G
            } else if t < 4.5 {
                2.5 * G
            } else if t < 8.0 {
                -1.5 * G * (8.0 - t) / 3.5
            } else {
                G
            }
        };
        let strong = |t: f64| {
            if t < 3.0 {
                G
            } else if t < 4.5 {
                6.0 * G
            } else if t < 8.0 {
                -3.0 * G * (8.0 - t) / 3.5
            } else {
                G
            }
        };
        for (case, readings) in [
            ("weak, 0.01 s", trimmed(&weak, 0.01, 4.6, f64::INFINITY)),
            ("weak, 0.02 s", trimmed(&weak, 0.02, 4.6, f64::INFINITY)),
            ("weak, 0.05 s", trimmed(&weak, 0.05, 4.6, f64::INFINITY)),
            ("weak, to 6.5 s", trimmed(&weak, 0.01, 4.6, 6.5)),
            ("strong, to 7 s", trimmed(&strong, 0.01, 4.6, 7.0)),
        ] {
            assert!(
                readings.max_acceleration.value().is_none()
                    && readings
                        .accelerometer
                        .as_ref()
                        .is_none_or(|a| !a.turned_over),
                "{case}: {:?}, {:?}",
                readings.accelerometer,
                readings.max_acceleration
            );
        }
    }

    /// A coast alone settles nothing, as its second can hold what reads the other way: on a
    /// part that holds gravity, nose up on a pad tilted to read 0.5 g, which says nothing, a
    /// 40 g burn of 0.3 s before a heavy drag leaves the thrust saying nothing, and a canopy
    /// opened early reads 1 g through the coast's second; on AltOS, which takes gravity out, a
    /// 2.5 g thrust, under 2 g as written, still burns into that second. Either coast says nose
    /// down; both are withheld, never turned over.
    #[test]
    fn a_lone_coast_settles_nothing() {
        let canopy = |t: f64| {
            if t < 3.0 {
                0.5 * G
            } else if t < 3.3 {
                40.0 * G
            } else if t < 4.5 {
                -8.0 * G + 5.1 * G * (t - 3.3) / 1.2
            } else if t < 6.0 {
                -2.9 * G * (6.0 - t) / 1.5
            } else {
                G
            }
        };
        let burning = |t: f64| {
            if t < 3.0 {
                G
            } else if t < 7.5 {
                2.5 * G
            } else if t < 8.0 {
                -0.3 * G
            } else {
                G
            }
        };
        for (case, readings) in [
            (
                "a canopy opened early",
                read_all(&log(
                    LogFormat::GenericTable,
                    "Axial accel",
                    0.01,
                    |k, t| canopy(t) + jitter(k, 0.3),
                )),
            ),
            (
                "a thrust still burning",
                read_all(&log(LogFormat::AltosCsv, "acceleration", 0.01, |k, t| {
                    burning(t) - G + jitter(k, 0.3)
                })),
            ),
        ] {
            let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
                panic!("{case}: {:?}", readings.max_acceleration);
            };
            assert_eq!(*reason, Reason::DirectionUnstated, "{case}");
            assert!(detail.contains("the coast alone says"), "{case}: {detail}");
        }
    }

    /// An accelerometer whose clock runs late against the heights', its thrust starting at 7.6 s
    /// where the heights put apogee at 8 s: the second before apogee's earliest time is still its
    /// pad, at 1 g, and comes before the trace first moves, so it isn't read as a coast, which
    /// would say nose down. Its thrust rises after liftoff's latest time by the heights, so it
    /// isn't the thrust that lifted the rocket on their clock and says nothing either: withheld,
    /// never turned over.
    #[test]
    fn a_coast_before_the_trace_moves_says_nothing() {
        let readings = read_all(&log(
            LogFormat::EntacoreAimCsv,
            "acceleration",
            0.01,
            |k, t| if t < 7.6 { G } else { 6.0 * G } + jitter(k, 0.3),
        ));
        let Reading::Withheld(Withheld { reason, detail }) = &readings.max_acceleration else {
            panic!("{:?}", readings.max_acceleration);
        };
        assert_eq!(*reason, Reason::DirectionUnstated);
        assert!(detail.contains("no coast to apogee is read"), "{detail}");
        assert!(
            detail.contains("rises after liftoff's latest time"),
            "{detail}"
        );
    }

    /// The thrust's witness, each way it says or doesn't, on traces sampled every 0.01 s: a held
    /// span with nothing the other way before it says its way; one after a span the other way
    /// says nothing; with none held, spans one way say it and spans both ways or none nothing;
    /// a span before liftoff's earliest time isn't searched.
    #[test]
    fn the_thrust_witness_says_its_way_or_nothing() {
        let dt = 0.01;
        let time: Vec<f64> = (0..=600).map(|i| f64::from(i) * dt).collect();
        let witness = |from_s: f64, trace: &dyn Fn(f64) -> f64| {
            let clean: Vec<f64> = time.iter().map(|t| trace(*t)).collect();
            let from = time.partition_point(|t| *t < from_s);
            thrust_witness(
                &time,
                &clean,
                from,
                time.len() - 1,
                half_window(BURNOUT_MEAN_S, dt),
                dt,
                (1.2, "liftoff's latest time", true),
            )
        };
        // A pulse of `g` g from `at` s for `long` s.
        let pulse = |t: f64, at: f64, long: f64, g: f64| {
            if (at..at + long).contains(&t) {
                g * G
            } else {
                0.0
            }
        };
        let thrust = move |t: f64| G + pulse(t, 1.0, 1.0, 4.0) - pulse(t, 2.0, 2.0, 1.5);
        let short_then_drag = move |t: f64| pulse(t, 1.0, 0.3, 40.0) - pulse(t, 1.3, 1.0, 8.0);
        let short = move |t: f64| pulse(t, 1.0, 0.3, 6.0) - pulse(t, 1.3, 2.0, 0.5);
        let both = move |t: f64| pulse(t, 1.0, 0.2, 5.0) - pulse(t, 3.0, 0.2, 5.0);
        let knocked = move |t: f64| -pulse(t, 0.5, 0.15, 3.5) + thrust(t);
        // A case's name, liftoff's earliest time, its trace, the way it says and its words.
        type Case<'a> = (&'a str, f64, &'a dyn Fn(f64) -> f64, Option<Way>, &'a str);
        let cases: [Case; 9] = [
            ("held", 0.0, &thrust, Some(Way::Up), "to hold there"),
            (
                "held, down",
                0.0,
                &|t| -thrust(t),
                Some(Way::Down),
                "to hold",
            ),
            (
                "drag after a short burn",
                0.0,
                &short_then_drag,
                None,
                "comes after one nose up",
            ),
            (
                "mirrored",
                0.0,
                &|t| -short_then_drag(t),
                None,
                "comes after one nose down",
            ),
            ("short only", 0.0, &short, Some(Way::Up), "1 span reads so"),
            (
                "short, down",
                0.0,
                &|t| -short(t),
                Some(Way::Down),
                "1 span reads so",
            ),
            ("both ways", 0.0, &both, None, "both ways up"),
            ("flat", 0.0, &|_| G, None, "never reads past"),
            (
                "knock before the search",
                0.8,
                &knocked,
                Some(Way::Up),
                "from 1.00 s",
            ),
        ];
        for (case, from_s, trace, way, words) in cases {
            let said = witness(from_s, trace);
            assert_eq!(said.way, way, "{case}: {said:?}");
            assert!(said.words.contains(words), "{case}: {said:?}");
        }
        // The knock searched: its rebound comes before the thrust, which then says nothing.
        assert_eq!(witness(0.0, &knocked).way, None);
        // A thrust under 2 g as written, then a drag that holds past 2 g the other way: the
        // drag's span rises after liftoff's latest time, 1.2 s here, so it says nothing.
        let weak_then_drag = move |t: f64| pulse(t, 1.0, 1.5, 1.5) - pulse(t, 2.5, 1.5, 2.5);
        let said = witness(0.0, &weak_then_drag);
        assert_eq!(said.way, None, "{said:?}");
        assert!(
            said.words
                .contains("from 2.50 s, rises after liftoff's latest time, 1.20 s"),
            "{said:?}"
        );
        // With liftoff withheld, as a log that starts in the climb has it, the thrust's span
        // rises within 0.5 s of the log's start or says nothing.
        let started = |at_s: f64| {
            let clean: Vec<f64> = time.iter().map(|t| thrust(*t + 1.0 - at_s)).collect();
            thrust_witness(
                &time,
                &clean,
                0,
                time.len() - 1,
                half_window(BURNOUT_MEAN_S, dt),
                dt,
                (
                    BURN_LEAST_S,
                    "the log's start by 0.5 s, liftoff withheld",
                    false,
                ),
            )
        };
        assert_eq!(started(0.2).way, Some(Way::Up), "{:?}", started(0.2));
        let late = started(1.0);
        assert_eq!(late.way, None, "{late:?}");
        assert!(
            late.words
                .contains("rises after the log's start by 0.5 s, liftoff withheld"),
            "{late:?}"
        );
    }

    /// The pad and the coast say by their levels as written, each within 0.25 g: the pad 1 g
    /// nose up, −1 g nose down, 0 nothing; the coast −0.75 g or less nose up, 0.75 g or more
    /// nose down, a drag only pushing it further. Settled when the pad or the thrust says and
    /// none disagrees; the coast alone settles nothing, but can disagree.
    #[test]
    fn the_pad_and_the_coast_say_by_their_levels() {
        let silent = ThrustWitness {
            way: None,
            words: "its thrust says neither".to_owned(),
            alone: true,
        };
        let up = ThrustWitness {
            way: Some(Way::Up),
            words: "its thrust says nose up".to_owned(),
            alone: true,
        };
        let down = ThrustWitness {
            way: Some(Way::Down),
            words: "its thrust says nose down".to_owned(),
            alone: true,
        };
        let way = |pad: Option<f64>, coast: Option<f64>, thrust: &ThrustWitness| {
            direction(pad.map(|g| g * G), coast.map(|g| g * G), thrust).map(|(way, _)| way)
        };
        assert_eq!(way(Some(1.0), None, &silent), Ok(Way::Up));
        assert_eq!(way(Some(1.24), None, &silent), Ok(Way::Up));
        assert_eq!(way(Some(-1.0), None, &silent), Ok(Way::Down));
        assert!(way(Some(1.26), None, &silent).is_err());
        assert!(way(Some(0.0), None, &silent).is_err());
        for coast in [-1.0, -0.76, -3.0, 1.3, -0.74, 0.0] {
            assert!(way(None, Some(coast), &silent).is_err(), "{coast}");
        }
        assert!(
            direction(None, Some(-G), &silent)
                .unwrap_err()
                .ends_with("So the coast doesn't settle which way up it reads on its own"),
        );
        assert_eq!(way(None, Some(-1.0), &up), Ok(Way::Up));
        assert_eq!(way(None, Some(-0.76), &up), Ok(Way::Up));
        assert_eq!(way(None, Some(-3.0), &up), Ok(Way::Up));
        assert!(way(None, Some(-0.76), &down).is_err());
        assert_eq!(way(None, Some(-0.74), &down), Ok(Way::Down));
        assert_eq!(way(None, Some(1.3), &down), Ok(Way::Down));
        assert!(way(None, Some(0.76), &up).is_err());
        assert_eq!(way(None, Some(0.74), &up), Ok(Way::Up));
        assert_eq!(way(Some(1.0), Some(-1.0), &silent), Ok(Way::Up));
        // A thrust at the start of a log whose liftoff is withheld settles only beside the
        // coast, or the pad.
        let at_start = ThrustWitness {
            way: Some(Way::Up),
            words: "its thrust says nose up".to_owned(),
            alone: false,
        };
        assert!(
            direction(None, None, &at_start)
                .unwrap_err()
                .ends_with("without the coast beside it")
        );
        assert_eq!(way(None, Some(-1.0), &at_start), Ok(Way::Up));
        assert!(way(None, Some(1.0), &at_start).is_err());
        assert_eq!(way(Some(-1.0), Some(1.0), &silent), Ok(Way::Down));
        assert!(way(None, None, &silent).is_err());
        assert_eq!(way(None, None, &down), Ok(Way::Down));
        assert_eq!(
            direction(Some(G), Some(-G), &up),
            Ok((Way::Up, vec![WayBy::Pad, WayBy::Coast, WayBy::Thrust]))
        );
        assert!(way(Some(1.0), Some(1.0), &silent).is_err());
        assert!(way(Some(1.0), None, &down).is_err());
        assert!(way(None, Some(-1.0), &down).is_err());
        // The words say what each witness said, and whether none said or two disagree.
        let none = direction(None, None, &silent).unwrap_err();
        assert!(none.starts_with("no pad is read to tell by"), "{none}");
        assert!(
            none.ends_with("none of them says which way up it reads"),
            "{none}"
        );
        let split = direction(Some(G), Some(G), &silent).unwrap_err();
        for words in [
            "its pad reads 1.00 g as written, which says nose up",
            "its coast to apogee reads 1.00 g as written, which says nose down",
            "they disagree on which way up it reads",
        ] {
            assert!(split.contains(words), "{words}: {split}");
        }
    }

    /// An accelerometer that ends in the thrust, still rising from 3 g to 6 g, gives its top as a
    /// floor: the thrust never falls through zero on the channel, so the top may have been
    /// higher, and burnout isn't read off it. So whether the whole log ends at 3.7 s or only the
    /// accelerometer does; and where the whole log ends at 6 s, at 12 g, its apogee a floor at
    /// its end: the second before that apogee's earliest time is the thrust's, not a coast, and
    /// would say nose down.
    #[test]
    fn an_accelerometer_that_ends_in_the_thrust_gives_its_top_as_a_floor() {
        let rising = |t: f64| {
            if t < 3.0 {
                G
            } else {
                3.0 * G + 3.0 * G * (t - 3.0)
            }
        };
        for (end_s, whole) in [(3.7, true), (3.7, false), (6.0, true)] {
            let mut log = log(LogFormat::GenericTable, "Axial accel", 0.01, |k, t| {
                rising(t) + jitter(k, 0.3)
            });
            let cut = |time_s: &[f64]| time_s.partition_point(|t| *t <= end_s + 1e-9);
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
            let case = format!("to {end_s} s, whole {whole}");
            let top = readings.max_acceleration.value();
            let top = top.unwrap_or_else(|| panic!("{case}: {:?}", readings.max_acceleration));
            assert!(top.is_floor, "{case}: {top:?}");
            assert_eq!(top.provenance, Provenance::Measured, "{case}");
            // The reading less its bound lies below the thrust where the channel ends.
            assert!(
                top.acceleration_m_s2 - top.bound_m_s2 <= rising(end_s),
                "{case}: {top:?}"
            );
            assert!(
                readings
                    .burnout
                    .value()
                    .is_none_or(|burnout| burnout.method != Method::LeastAfterThrust),
                "{case}: {:?}",
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
