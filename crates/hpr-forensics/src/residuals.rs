//! A flight log against its simulation: how far the logged height and speed lie from the
//! simulated ones, sample by sample, and each of the log's readings beside the simulation's.
//!
//! **What it is for.** [M7.3][m7-3] starts from what [NASA Student Launch's handbook][adr-257] asks
//! a team after a flight: "compare the predicted flight model to the actual flight data". The
//! fits that come later work on these same residuals.
//!
//! **One rule: the simulation is read as the log is.** A logged height and a simulated one aren't
//! the same quantity. A barometric logger reads the standard atmosphere's altitude of the pressure
//! around it, so on a warm day it reads low ([`Barometer`]). And a reading such as apogee or top
//! speed depends on how it was taken from the track. So the simulated flight is written out as a
//! noise-free log at the real log's median sample interval, its height as the real log's sensor
//! would read it, and handed to the same reader ([`hpr_flightdata::readings::read`]). The two
//! clocks are lined up by what that reader calls liftoff on each, the [shared liftoff][adr-312]
//! that two logs of one flight use.
//!
//! **The sign.** A residual is the logged value less the simulated one, so a positive height
//! residual means the rocket read higher than the simulation did.
//!
//! **How far to trust it.** A residual describes one flight against one simulation. It is the sum
//! of what the simulation got wrong and what the logger got wrong, and it can't tell them apart;
//! it isn't a verdict on either. The heights are the log's own samples, unsmoothed, so an ejection
//! charge's pressure spike shows as a residual. The speeds are the smoother's, each with its
//! standard error ([`hpr_flightdata::smoother`]), its GNSS set aside on a barometric log so that
//! the speed stays on the height's scale. The clock is good to the bound it states, and on a log
//! with coarse steps the log's liftoff reads late, so an ascent's height residual leans positive.
//! The simulated values carry no error of their own here. The method is tested on constructed
//! flights below and run on the 7 public flights in `cargo xtask real-flights`.
//!
//! [m7-3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [adr-312]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0312-two-logs-on-one-timeline.md

use hpr_atmos::{AtmosError, Barometer};
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use hpr_flightdata::readings::{self, Reading, Readings, Source, Withheld};
use hpr_flightdata::smoother;
use hpr_flightdata::timeline::{self, Cell, Flew, Offset, What};
use hpr_sim::{Environment, FlightEvent, FlightStep, Observer, SimError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::synthetic::truth::{Grid, TRUTH_STEP_S, TruthSample, truth_sample};

/// How long the simulated log sits on the pad before the flight, s: long enough for any reader's
/// look at the pad.
const PAD_S: f64 = 10.0;
/// How long it sits on the ground after the flight, s: past the reader's wait for a landing
/// ([`readings::LANDED_FOR_S`]).
const LANDED_S: f64 = 3.0 * readings::LANDED_FOR_S;
/// The simulated log's coarsest sample interval, s: as coarse as the reader takes a log.
const COARSEST_INTERVAL_S: f64 = readings::MAX_INTERVAL_S;
/// The height step that gives a barometer's rate of reading, m.
const RATE_STEP_M: f64 = 1.0;

/// Why a log can't be compared with a simulation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ResidualsError {
    /// The log has no liftoff to line the clocks up by.
    #[error("the log's liftoff can't be read: {0}")]
    LogLiftoff(String),
    /// The simulated flight, read as a log, has no liftoff.
    #[error("the simulated flight's liftoff can't be read: {0}")]
    SimulationLiftoff(String),
    /// The simulation kept no samples.
    #[error("the simulation kept no samples")]
    NoSamples,
    /// The day's air doesn't reach the flight.
    #[error(transparent)]
    Atmosphere(#[from] AtmosError),
    /// The simulation refused.
    #[error(transparent)]
    Sim(#[from] SimError),
}

/// What a log's height stands for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeightKind {
    /// A barometer's: the standard atmosphere's altitude of the pressure, less the pad's.
    Barometric,
    /// A height climbed: a GNSS receiver's, or the accelerometer's integral.
    Geometric,
}

impl HeightKind {
    /// What a log's track from `source` stands for.
    #[must_use]
    pub fn of(source: Source) -> Self {
        match source {
            Source::Gnss | Source::Accelerometer => Self::Geometric,
            // Every other track is a barometer's, as a logger's main track is.
            _ => Self::Barometric,
        }
    }
}

/// Keeps a flight's samples as it flies: an observer for [`hpr_sim::Simulation::run`] and the
/// flight builder's `fly_with`.
#[derive(Debug, Default)]
pub struct Track {
    grid: Grid,
}

impl Track {
    /// A track with nothing kept yet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The flight kept, in `environment`, the air it flew in.
    ///
    /// # Errors
    ///
    /// [`ResidualsError`] if the flight kept no samples, or the day's air doesn't reach it.
    pub fn simulated(self, environment: &Environment) -> Result<Simulated, ResidualsError> {
        let mut samples = self.grid.samples;
        samples.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));
        samples.dedup_by(|b, a| a.time_s == b.time_s);
        let samples = samples
            .iter()
            .map(|sample| truth_sample(environment, sample))
            .collect::<Result<Vec<_>, _>>()?;
        Simulated::new(samples, environment)
    }
}

impl Observer for Track {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        self.grid.step(step)
    }

    fn event(&mut self, event: &FlightEvent) {
        self.grid.event(event);
    }
}

/// One moment of a simulated flight.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Moment {
    time_s: f64,
    /// The height climbed from the pad, m.
    height_m: f64,
    /// What a barometer on the pad reads, m.
    reading_m: f64,
    /// The vertical speed, m/s, up.
    speed_m_s: f64,
    /// The rate at which that barometer's reading changes, m/s.
    reading_speed_m_s: f64,
    /// The specific force along the rocket's axis, m/s².
    axial_specific_force_m_s2: f64,
    /// Meters east of where the flight starts.
    east_m: f64,
    /// Meters north of where the flight starts.
    north_m: f64,
    /// The Mach number.
    mach: f64,
    /// The dynamic pressure as the height a barometer reads for it, m, at a pressure coefficient of
    /// −1: `q / (ρ g)`, `ρ` the standard air's at its reading.
    port_m: f64,
}

/// A simulated flight, its heights both as climbed and as a barometer on the pad reads them.
#[derive(Debug, Clone, PartialEq)]
pub struct Simulated {
    moments: Vec<Moment>,
}

impl Simulated {
    /// A flight of `samples`, in time order, flown in `environment`.
    ///
    /// # Errors
    ///
    /// [`ResidualsError`] if there are no samples, or the day's air doesn't reach them.
    pub fn new(
        samples: Vec<TruthSample>,
        environment: &Environment,
    ) -> Result<Self, ResidualsError> {
        let first = samples.first().ok_or(ResidualsError::NoSamples)?;
        let pad_m = first.height_m;
        let (start_east_m, start_north_m) = (first.position_enu_m[0], first.position_enu_m[1]);
        let site_msl_m = environment.site().height_m - environment.geoid_undulation_m;
        let day = environment.atmosphere.as_ref();
        let barometer = Barometer::on_pad(day, site_msl_m, pad_m)?;
        let moments = samples
            .iter()
            .map(|sample| {
                let reading_m = barometer.reading_m(sample.height_m)?;
                // The barometer's reading doesn't rise at the rate the height does: its rate is
                // the vertical speed times the slope of reading against height there.
                let above = barometer.reading_m(sample.height_m + RATE_STEP_M)?;
                let below = barometer.reading_m(sample.height_m - RATE_STEP_M)?;
                let slope = (above - below) / (2.0 * RATE_STEP_M);
                let density = day.air(site_msl_m + sample.height_m)?.air.density_kg_m3;
                let port_m = sample.dynamic_pressure_pa / (density / slope * STANDARD_GRAVITY_MPS2);
                Ok(Moment {
                    time_s: sample.time_s,
                    height_m: sample.height_m - pad_m,
                    reading_m,
                    speed_m_s: sample.vertical_speed_m_s,
                    reading_speed_m_s: sample.vertical_speed_m_s * slope,
                    axial_specific_force_m_s2: sample.axial_specific_force_m_s2,
                    east_m: sample.position_enu_m[0] - start_east_m,
                    north_m: sample.position_enu_m[1] - start_north_m,
                    mach: sample.mach,
                    port_m,
                })
            })
            .collect::<Result<Vec<_>, AtmosError>>()?;
        Ok(Self { moments })
    }

    /// The moment at `t`, by straight lines between the samples; the first moment before the flight,
    /// where the rocket sits on the pad; `None` after it.
    fn at(&self, t: f64) -> Option<Moment> {
        let (first, last) = (self.moments.first()?, self.moments.last()?);
        if t > last.time_s {
            return None;
        }
        if t <= first.time_s {
            return Some(*first);
        }
        let after = self.moments.partition_point(|moment| moment.time_s < t);
        let b = self.moments[after];
        if b.time_s == t {
            return Some(b);
        }
        let a = self.moments[after - 1];
        let w = (t - a.time_s) / (b.time_s - a.time_s);
        let mix = |x: f64, y: f64| x + w * (y - x);
        Some(Moment {
            time_s: t,
            height_m: mix(a.height_m, b.height_m),
            reading_m: mix(a.reading_m, b.reading_m),
            speed_m_s: mix(a.speed_m_s, b.speed_m_s),
            reading_speed_m_s: mix(a.reading_speed_m_s, b.reading_speed_m_s),
            axial_specific_force_m_s2: mix(
                a.axial_specific_force_m_s2,
                b.axial_specific_force_m_s2,
            ),
            east_m: mix(a.east_m, b.east_m),
            north_m: mix(a.north_m, b.north_m),
            mach: mix(a.mach, b.mach),
            port_m: mix(a.port_m, b.port_m),
        })
    }

    /// The moment at `t`, holding the flight's first before it and its last after it.
    fn held_at(&self, t: f64) -> Option<Moment> {
        self.at(t).or_else(|| self.moments.last().copied())
    }

    /// The height as a sensor of `kind` reads it at `t`, above the pad, m.
    pub(crate) fn read_height_m(&self, t: f64, kind: HeightKind) -> f64 {
        self.held_at(t).map_or(f64::NAN, |moment| match kind {
            HeightKind::Barometric => moment.reading_m,
            HeightKind::Geometric => moment.height_m,
        })
    }

    /// The height a barometer reads for the flow at `t` at a pressure coefficient of −1, m.
    pub(crate) fn port_m(&self, t: f64) -> f64 {
        self.held_at(t).map_or(f64::NAN, |moment| moment.port_m)
    }

    /// The Mach number at `t`.
    pub(crate) fn mach(&self, t: f64) -> f64 {
        self.held_at(t).map_or(f64::NAN, |moment| moment.mach)
    }

    /// Meters east and north of where the flight starts at `t`.
    pub(crate) fn drift_m(&self, t: f64) -> (f64, f64) {
        self.held_at(t).map_or((f64::NAN, f64::NAN), |moment| {
            (moment.east_m, moment.north_m)
        })
    }

    /// The flight as a noise-free log of `kind` at `log`'s median sample interval, sitting on the
    /// pad for [`PAD_S`] first and on the ground for [`LANDED_S`] after. Its clock runs from the
    /// start of that wait: a flight's time plus [`PAD_S`]. It has an accelerometer if `log` has.
    fn as_log(&self, log: &FlightLog, kind: HeightKind) -> Result<FlightLog, ResidualsError> {
        let first = self.moments.first().ok_or(ResidualsError::NoSamples)?;
        let last = self.moments.last().ok_or(ResidualsError::NoSamples)?;
        let (track_s, _) = readings::height_track(log).unwrap_or_default();
        let interval = median_interval_s(&track_s)
            .unwrap_or(COARSEST_INTERVAL_S)
            .clamp(TRUTH_STEP_S, COARSEST_INTERVAL_S);
        let end_s = PAD_S + last.time_s + LANDED_S;
        let rows = (end_s / interval).ceil() as usize;
        let (mut time_s, mut altitude_m, mut force) = (
            Vec::with_capacity(rows),
            Vec::with_capacity(rows),
            Vec::with_capacity(rows),
        );
        let height = |moment: &Moment| match kind {
            HeightKind::Barometric => moment.reading_m,
            HeightKind::Geometric => moment.height_m,
        };
        for row in 0..rows {
            let clock_s = row as f64 * interval;
            let t = clock_s - PAD_S;
            let moment = if t < first.time_s {
                first
            } else if t > last.time_s {
                last
            } else {
                &self.at(t).unwrap_or(*last)
            };
            time_s.push(clock_s);
            altitude_m.push(height(moment));
            force.push(moment.axial_specific_force_m_s2);
        }
        let channels = if log.channels_of(Quantity::Acceleration).next().is_some() {
            vec![Channel::new(
                "simulated axial specific force",
                Quantity::Acceleration,
                time_s.clone(),
                force,
            )]
        } else {
            Vec::new()
        };
        Ok(FlightLog {
            format: LogFormat::GenericTable,
            logger: "HPR Sim's simulated flight".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            time_s,
            altitude_m,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels,
            events: Vec::new(),
            notes: Vec::new(),
        })
    }
}

/// The median of `times`' steps; `None` for fewer than two samples or no forward step.
fn median_interval_s(times: &[f64]) -> Option<f64> {
    let mut steps: Vec<f64> = times
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|step| step.is_finite() && *step > 0.0)
        .collect();
    steps.sort_by(f64::total_cmp);
    steps.get(steps.len() / 2).copied()
}

/// A part of the flight, on the log's readings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// Liftoff to the log's burnout.
    Boost,
    /// The log's burnout to its apogee.
    Coast,
    /// Liftoff to the log's apogee, where its burnout isn't read.
    Ascent,
    /// The log's apogee to its landing, or its end.
    Descent,
    /// The whole log after liftoff, where its apogee isn't read whole.
    Flight,
}

/// How a phase's residuals fall.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    /// Which part of the flight.
    pub phase: Phase,
    /// When it starts, s after the log's liftoff.
    pub start_s: f64,
    /// When it ends, s after the log's liftoff.
    pub end_s: f64,
    /// How many samples it compares.
    pub samples: usize,
    /// The mean residual, with its sign.
    pub mean: f64,
    /// The root mean square residual.
    pub rms: f64,
    /// The residual of largest size, with its sign.
    pub largest: f64,
    /// When that residual falls, s after the log's liftoff.
    pub largest_at_s: f64,
}

/// One sample's residual.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// When it falls, s after the log's liftoff.
    pub time_after_liftoff_s: f64,
    /// The log's value.
    pub logged: f64,
    /// The log's standard error on it; `None` for a raw sample, which states none.
    pub logged_sd: Option<f64>,
    /// The simulation's value, read as the log's sensor would read it.
    pub simulated: f64,
    /// `logged − simulated`.
    pub residual: f64,
}

/// One quantity's residuals, from liftoff to the log's landing or its end.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Residuals {
    /// Each sample's.
    pub points: Vec<Point>,
    /// How they fall, by phase.
    pub phases: Vec<Summary>,
    /// The log's samples after the simulated flight had ended: compared with nothing.
    pub past_simulation: usize,
}

/// The speed's residuals, which need the smoother.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Speed {
    /// The smoother's speed less the simulated.
    Read(Residuals),
    /// Why the smoother gives no speed.
    Withheld {
        /// The reason in words, with the log's own numbers.
        detail: String,
    },
}

/// One reading beside the simulation's.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    /// Which reading.
    pub what: What,
    /// The log's: its value, bound, 90% interval, source and provenance, as `hpr analyze` gives.
    pub logged: Cell,
    /// The same reader's reading of the simulated flight. It states no source or provenance, as
    /// it wasn't measured; its bound is only the reader's on noise-free samples.
    pub simulated: Cell,
    /// `logged − simulated`; `None` if either is withheld or is a floor, not the value.
    pub logged_minus_simulated: Option<f64>,
}

/// A log against its simulation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comparison {
    /// How the clocks were lined up: [`timeline::METHOD`].
    pub method: String,
    /// What the log's height was taken to stand for, and so how the simulation's was read.
    pub height_read_as: HeightKind,
    /// The log's liftoff, s on its own clock.
    pub log_liftoff_s: f64,
    /// The simulated liftoff, as the same reader finds it, s after ignition.
    pub simulated_liftoff_s: f64,
    /// The simulation's clock against the log's: the offset's size is arbitrary, its bound says
    /// how well the two are lined up.
    pub clock: Offset,
    /// The sample interval the simulation was read at, s.
    pub simulated_interval_s: f64,
    /// Whether the residuals run to a landing read from the log, not just to the log's end.
    pub reaches_landing: bool,
    /// The logged height less the simulated.
    pub height_m: Residuals,
    /// The smoother's vertical speed less the simulated, or why the smoother is withheld.
    pub speed_m_s: Speed,
    /// Each reading beside the simulation's.
    pub rows: Vec<Row>,
}

/// `log`, read as `read`, against `simulated`.
///
/// `height_read_as` overrides what the log's height is taken to stand for, for a log whose sensor
/// is known from elsewhere; `None` goes by the source of the log's liftoff.
///
/// # Errors
///
/// [`ResidualsError`] if the log or the simulated flight has no liftoff to line the clocks up by.
pub fn compare(
    log: &FlightLog,
    read: &Readings,
    simulated: &Simulated,
    height_read_as: Option<HeightKind>,
) -> Result<Comparison, ResidualsError> {
    let liftoff = match &read.liftoff {
        Reading::Read(liftoff) => liftoff,
        Reading::Withheld(withheld) => {
            return Err(ResidualsError::LogLiftoff(withheld.detail.clone()));
        }
    };
    let kind = height_read_as.unwrap_or_else(|| HeightKind::of(liftoff.source));
    let simulated_log = simulated.as_log(log, kind)?;
    let simulated_read = readings::read(&simulated_log);
    let simulated_liftoff = match &simulated_read.liftoff {
        Reading::Read(liftoff) => liftoff,
        Reading::Withheld(withheld) => {
            return Err(ResidualsError::SimulationLiftoff(withheld.detail.clone()));
        }
    };
    let simulated_liftoff_s = simulated_liftoff.time_s - PAD_S;
    let side = timeline::side_by_side_read(
        &[(log, read), (&simulated_log, &simulated_read)],
        Flew::OneAirframe,
    );
    let clock = match side.offsets.first() {
        Some(Reading::Read(offset)) => *offset,
        // Both liftoffs are read above, so the offset is too.
        Some(Reading::Withheld(Withheld { detail, .. })) => {
            return Err(ResidualsError::LogLiftoff(detail.clone()));
        }
        None => return Err(ResidualsError::NoSamples),
    };
    let rows = side
        .rows
        .into_iter()
        // The smoother's apogee fuses a log's sensors; a noise-free track has nothing to fuse.
        .filter(|row| row.what != What::SmoothedApogee)
        .filter_map(|row| {
            let mut cells = row.cells.into_iter();
            let (logged, simulated) = (cells.next()?, cells.next()?);
            let simulated = unmeasured(row.what, simulated);
            let logged_minus_simulated = difference(&logged, &simulated);
            Some(Row {
                what: row.what,
                logged,
                simulated,
                logged_minus_simulated,
            })
        })
        .collect();

    let window = Window::of(read, liftoff.time_s, log);
    let pad_m = read.pad_altitude_m.unwrap_or(0.0);
    let (track_s, track_m) = readings::height_track(log).unwrap_or_default();
    let height_samples = track_s
        .iter()
        .zip(&track_m)
        .map(|(&t, &altitude)| (t, altitude - pad_m, None));
    let height_m =
        window.residuals(
            height_samples,
            simulated,
            simulated_liftoff_s,
            |moment| match kind {
                HeightKind::Barometric => moment.reading_m,
                HeightKind::Geometric => moment.height_m,
            },
        );
    // Fused with a GNSS receiver, the smoother's speed leaves the barometer's scale for the height
    // climbed, which is neither side's here: set aside, it stands on the log's height.
    let smoothed = match kind {
        HeightKind::Barometric => smoother::smooth(&without_gnss(log)),
        HeightKind::Geometric => smoother::smooth(log),
    };
    let speed_m_s = match smoothed {
        Ok(smoothed) => {
            let samples = smoothed
                .time_s
                .iter()
                .zip(&smoothed.speed_m_s)
                .zip(&smoothed.speed_sd_m_s)
                .map(|((&t, &speed), &sd)| (t, speed, Some(sd)));
            Speed::Read(
                window.residuals(
                    samples,
                    simulated,
                    simulated_liftoff_s,
                    |moment| match kind {
                        HeightKind::Barometric => moment.reading_speed_m_s,
                        HeightKind::Geometric => moment.speed_m_s,
                    },
                ),
            )
        }
        Err(withheld) => Speed::Withheld {
            detail: withheld.detail,
        },
    };
    Ok(Comparison {
        method: side.method,
        height_read_as: kind,
        log_liftoff_s: liftoff.time_s,
        simulated_liftoff_s,
        clock,
        simulated_interval_s: median_interval_s(&simulated_log.time_s).unwrap_or(0.0),
        reaches_landing: read.landing.value().is_some(),
        height_m,
        speed_m_s,
        rows,
    })
}

/// `log` with its GNSS altitude set aside.
fn without_gnss(log: &FlightLog) -> FlightLog {
    let mut log = log.clone();
    log.channels
        .retain(|channel| channel.quantity != Quantity::GnssAltitude);
    log
}

/// `cell` without a source or provenance: a simulated number wasn't measured. Its top acceleration
/// is the value, never a floor: the simulated accelerometer has no range to clip at, so a plateau
/// sampled twice alike isn't a sensor held at its limit.
fn unmeasured(what: What, cell: Cell) -> Cell {
    match cell {
        Cell::Read(given) => Cell::Read(timeline::Given {
            source: None,
            provenance: None,
            interval: None,
            is_floor: given.is_floor && what != What::TopAcceleration,
            ..given
        }),
        withheld @ Cell::Withheld { .. } => withheld,
    }
}

/// `logged − simulated` if both are whole values.
fn difference(logged: &Cell, simulated: &Cell) -> Option<f64> {
    match (logged, simulated) {
        (Cell::Read(logged), Cell::Read(simulated)) if !logged.is_floor && !simulated.is_floor => {
            Some(logged.value - simulated.value)
        }
        _ => None,
    }
}

/// The part of the log's clock compared, and its phases, on the log's own readings.
#[derive(Debug, Clone)]
struct Window {
    liftoff_s: f64,
    /// When the log's flight ends, s after liftoff.
    end_s: f64,
    /// Each phase and when it ends, s after liftoff, in order.
    phases: Vec<(Phase, f64)>,
}

impl Window {
    fn of(read: &Readings, liftoff_s: f64, log: &FlightLog) -> Self {
        let (track_s, _) = readings::height_track(log).unwrap_or_default();
        let last = track_s.last().copied().unwrap_or(liftoff_s);
        let end_s = read
            .landing
            .value()
            .map_or(last, |landing| landing.time_s)
            .max(liftoff_s)
            - liftoff_s;
        let apogee = read
            .apogee
            .value()
            .filter(|apogee| !apogee.is_floor)
            .map(|apogee| apogee.time_s - liftoff_s)
            .filter(|&time_s| time_s > 0.0 && time_s <= end_s);
        let burnout = read
            .burnout
            .value()
            .filter(|burnout| !burnout.is_floor)
            .map(|burnout| burnout.time_s - liftoff_s);
        let phases = match (apogee, burnout) {
            (Some(apogee), Some(burnout)) if burnout > 0.0 && burnout < apogee => vec![
                (Phase::Boost, burnout),
                (Phase::Coast, apogee),
                (Phase::Descent, end_s),
            ],
            (Some(apogee), _) => vec![(Phase::Ascent, apogee), (Phase::Descent, end_s)],
            (None, _) => vec![(Phase::Flight, end_s)],
        };
        Self {
            liftoff_s,
            end_s,
            phases,
        }
    }

    /// `samples`' residuals: each `(time on the log's clock, value, its standard error)` less
    /// `value_of` the simulated moment the same time after the simulated liftoff.
    fn residuals(
        &self,
        samples: impl Iterator<Item = (f64, f64, Option<f64>)>,
        simulated: &Simulated,
        simulated_liftoff_s: f64,
        value_of: impl Fn(&Moment) -> f64,
    ) -> Residuals {
        let mut points = Vec::new();
        let mut past_simulation = 0;
        for (t, logged, logged_sd) in samples {
            let after = t - self.liftoff_s;
            // Written so that a time that isn't a number falls outside too.
            if !(0.0..=self.end_s).contains(&after) || !logged.is_finite() {
                continue;
            }
            let Some(moment) = simulated.at(simulated_liftoff_s + after) else {
                past_simulation += 1;
                continue;
            };
            let simulated = value_of(&moment);
            points.push(Point {
                time_after_liftoff_s: after,
                logged,
                logged_sd,
                simulated,
                residual: logged - simulated,
            });
        }
        let mut phases = Vec::with_capacity(self.phases.len());
        let mut start_s = 0.0;
        for &(phase, end_s) in &self.phases {
            let last = phase == Phase::Descent || phase == Phase::Flight;
            let inside: Vec<&Point> = points
                .iter()
                .filter(|point| {
                    let t = point.time_after_liftoff_s;
                    t >= start_s && (t < end_s || (last && t <= end_s))
                })
                .collect();
            if let Some(summary) = summarize(phase, start_s, end_s, &inside) {
                phases.push(summary);
            }
            start_s = end_s;
        }
        Residuals {
            points,
            phases,
            past_simulation,
        }
    }
}

/// How `points` fall; `None` for none.
fn summarize(phase: Phase, start_s: f64, end_s: f64, points: &[&Point]) -> Option<Summary> {
    let extreme = points
        .iter()
        .max_by(|a, b| a.residual.abs().total_cmp(&b.residual.abs()))?;
    let count = points.len() as f64;
    let sum = points.iter().fold(0.0, |sum, point| sum + point.residual);
    let squares = points
        .iter()
        .fold(0.0, |sum, point| sum + point.residual * point.residual);
    Some(Summary {
        phase,
        start_s,
        end_s,
        samples: points.len(),
        mean: sum / count,
        rms: (squares / count).sqrt(),
        largest: extreme.residual,
        largest_at_s: extreme.time_after_liftoff_s,
    })
}

/// A flight drawn from a formula, for the fit's tests.
#[cfg(test)]
pub(crate) mod tests_support {
    use super::{Moment, Simulated};

    /// A flight of height, east and north at each 0.01 s to `until_s`, none of it in the air's
    /// reading: a barometer reads what was climbed.
    pub(crate) fn simulated_of(until_s: f64, flight: impl Fn(f64) -> (f64, f64, f64)) -> Simulated {
        let steps = (until_s / 0.01).ceil() as u32;
        let moments = (0..=steps)
            .map(|k| {
                let time_s = 0.01 * f64::from(k);
                let (height_m, east_m, north_m) = flight(time_s);
                Moment {
                    time_s,
                    height_m,
                    reading_m: height_m,
                    speed_m_s: 0.0,
                    reading_speed_m_s: 0.0,
                    axial_specific_force_m_s2: 0.0,
                    east_m,
                    north_m,
                    mach: 0.0,
                    port_m: 0.0,
                }
            })
            .collect();
        Simulated { moments }
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use std::sync::Arc;

    use hpr_atmos::{AtmosphereModel, Ussa76};
    use hpr_core::geodesy::Geodetic;

    use super::*;

    /// A flight in closed form: 3 s at 60 m/s², a coast under gravity, 8 m/s down on a canopy.
    /// The height and speed at `t` s after ignition.
    fn flown(t: f64) -> (f64, f64, f64) {
        const G: f64 = 9.8;
        const BOOST: f64 = 60.0;
        const BURN_S: f64 = 3.0;
        const DESCENT: f64 = 8.0;
        if t <= BURN_S {
            return (0.5 * BOOST * t * t, BOOST * t, BOOST + G);
        }
        let burnout_m = 0.5 * BOOST * BURN_S * BURN_S;
        let burnout_speed = BOOST * BURN_S;
        let coast_s = burnout_speed / G;
        let top_m = burnout_m + burnout_speed * burnout_speed / (2.0 * G);
        if t <= BURN_S + coast_s {
            let after = t - BURN_S;
            let height = burnout_m + burnout_speed * after - 0.5 * G * after * after;
            return (height, burnout_speed - G * after, 0.0);
        }
        let after = t - BURN_S - coast_s;
        ((top_m - DESCENT * after).max(0.0), -DESCENT, G)
    }

    /// When the closed-form flight lands, s.
    fn landed_s() -> f64 {
        let mut t = 0.0;
        while flown(t + 0.01).0 > 0.0 {
            t += 0.01;
        }
        t
    }

    fn environment(atmosphere: Option<AtmosphereModel>) -> Environment {
        let site = Geodetic::from_degrees(35.0, -106.0, 1000.0).unwrap();
        let mut environment = Environment::standard(site).unwrap();
        if let Some(atmosphere) = atmosphere {
            environment.atmosphere = Arc::new(atmosphere);
        }
        environment
    }

    fn simulated(environment: &Environment, height: impl Fn(f64) -> (f64, f64, f64)) -> Simulated {
        let end = landed_s() + 0.01;
        let samples = (0..=(end / 0.01) as usize)
            .map(|i| {
                let time_s = i as f64 * 0.01;
                let (height_m, speed, force) = height(time_s);
                TruthSample {
                    time_s,
                    height_m,
                    vertical_speed_m_s: speed,
                    axial_specific_force_m_s2: force,
                    mach: 0.0,
                    dynamic_pressure_pa: 0.0,
                    position_enu_m: [0.0, 0.0, height_m],
                    velocity_enu_m_s: [0.0, 0.0, speed],
                }
            })
            .collect();
        Simulated::new(samples, environment).unwrap()
    }

    /// A log of the closed-form flight at 20 Hz, its clock running `pad_s` before ignition.
    fn logged(pad_s: f64, height: impl Fn(f64) -> f64) -> FlightLog {
        logged_every(0.05, pad_s, height)
    }

    /// The same every `interval_s`.
    fn logged_every(interval_s: f64, pad_s: f64, height: impl Fn(f64) -> f64) -> FlightLog {
        let rows = ((landed_s() + pad_s + 20.0) / interval_s) as usize;
        let time_s: Vec<f64> = (0..rows).map(|i| i as f64 * interval_s).collect();
        let altitude_m = time_s
            .iter()
            .map(|&clock| height((clock - pad_s).max(0.0).min(landed_s())))
            .collect();
        FlightLog {
            format: LogFormat::GenericTable,
            logger: "a test's".to_owned(),
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

    fn rms_of(comparison: &Comparison, phase: Phase) -> Summary {
        *comparison
            .height_m
            .phases
            .iter()
            .find(|summary| summary.phase == phase)
            .unwrap_or_else(|| panic!("no {phase:?} in {:?}", comparison.height_m.phases))
    }

    fn row(comparison: &Comparison, what: What) -> &Row {
        comparison.rows.iter().find(|row| row.what == what).unwrap()
    }

    /// A log of the same flight the simulation flew compares as nothing, on any log clock: a
    /// mutation of the clock's sign or of `pad_m` fails it.
    #[test]
    fn the_same_flight_leaves_almost_nothing() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        for pad_s in [20.0, 37.3] {
            let log = logged(pad_s, |t| flown(t).0 + 12.0);
            let comparison = compare(
                &log,
                &readings::read(&log),
                &simulation,
                Some(HeightKind::Geometric),
            )
            .unwrap();
            for summary in &comparison.height_m.phases {
                // Noise-free and stepless: only the clocks' sampling is left.
                assert!(summary.rms < 1.0, "{pad_s}: {summary:?}");
            }
            assert_eq!(comparison.height_m.past_simulation, 0);
            assert!(comparison.reaches_landing);
            let apogee = row(&comparison, What::ApogeeHeight);
            assert!(
                apogee.logged_minus_simulated.unwrap().abs() < 1.0,
                "{apogee:?}"
            );
        }
    }

    /// With ignition between a slower log's samples, the clocks' offset is all that's left, and it
    /// isn't small: the report's page quotes this size.
    #[test]
    fn ignition_between_samples_leaves_the_clocks_offset() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let log = logged_every(0.2, 37.3, |t| flown(t).0 + 12.0);
        let comparison = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        let ascent = comparison
            .height_m
            .phases
            .iter()
            .find(|summary| matches!(summary.phase, Phase::Ascent | Phase::Boost))
            .unwrap();
        assert!((9.0..12.0).contains(&ascent.rms), "{ascent:?}");
    }

    /// The residual is logged less simulated: a log that flew 10% higher reads positive.
    #[test]
    fn a_residual_is_logged_less_simulated() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let log = logged(20.0, |t| 1.1 * flown(t).0);
        let comparison = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        let top_m = flown(3.0 + 180.0 / 9.8).0;
        let apogee = row(&comparison, What::ApogeeHeight);
        let difference = apogee.logged_minus_simulated.unwrap();
        assert!(
            (difference - 0.1 * top_m).abs() < 2.0,
            "{difference} of {top_m}"
        );
        assert!(rms_of(&comparison, Phase::Descent).mean > 0.0);
        // Climbing a tenth faster, its speed residual is positive too.
        let Speed::Read(speed) = &comparison.speed_m_s else {
            panic!("{:?}", comparison.speed_m_s);
        };
        let climb = speed
            .phases
            .iter()
            .find(|summary| summary.phase != Phase::Descent)
            .unwrap();
        assert!(climb.mean > 1.0, "{climb:?}");
        // And the document says the same of a sample as of its summary.
        let point = comparison.height_m.points.last().unwrap();
        assert_eq!(point.residual, point.logged - point.simulated);
        let largest = comparison
            .height_m
            .points
            .iter()
            .map(|point| point.residual)
            .fold(f64::MIN, f64::max);
        assert!((largest - 0.1 * top_m).abs() < 5.0, "{largest} of {top_m}");
    }

    /// On a warm day a barometer reads low. Read as a barometer, the simulation matches a log
    /// that does; read as a height climbed, it is off by the closed form's ratio.
    #[test]
    fn a_barometric_log_is_compared_with_a_barometers_reading() {
        let warm = AtmosphereModel::Standard(Ussa76::with_offset(20.0, 101_325.0).unwrap());
        let environment = environment(Some(warm));
        let simulation = simulated(&environment, flown);
        let barometer = Barometer::on_pad(environment.atmosphere.as_ref(), 1000.0, 0.0).unwrap();
        let log = logged(20.0, |t| barometer.reading_m(flown(t).0).unwrap());
        let read = readings::read(&log);
        let by_barometer = compare(&log, &read, &simulation, Some(HeightKind::Barometric)).unwrap();
        let by_height = compare(&log, &read, &simulation, Some(HeightKind::Geometric)).unwrap();
        let apogee = |comparison: &Comparison| {
            row(comparison, What::ApogeeHeight)
                .logged_minus_simulated
                .unwrap()
        };
        assert!(apogee(&by_barometer).abs() < 1.0, "{by_barometer:?}");
        // 20 K warm: the barometer reads about 7% low, the flattering side for a waiver.
        let top_m = flown(3.0 + 180.0 / 9.8).0;
        let low = -apogee(&by_height) / top_m;
        assert!((0.05..0.09).contains(&low), "{low}");
        // Sample by sample too: the height's residuals and the speed's follow the same reading.
        let largest = |points: &[Point]| {
            points
                .iter()
                .fold(0.0_f64, |most, point| most.max(point.residual.abs()))
        };
        assert!(
            largest(&by_barometer.height_m.points) < 0.01 * top_m,
            "{}",
            largest(&by_barometer.height_m.points)
        );
        assert!(largest(&by_height.height_m.points) > 0.05 * top_m);
        let coast_rms = |comparison: &Comparison| {
            let Speed::Read(speed) = &comparison.speed_m_s else {
                panic!("{:?}", comparison.speed_m_s);
            };
            speed
                .phases
                .iter()
                .find(|summary| summary.phase != Phase::Descent)
                .unwrap()
                .rms
        };
        assert!(
            coast_rms(&by_barometer) < coast_rms(&by_height),
            "{} against {}",
            coast_rms(&by_barometer),
            coast_rms(&by_height)
        );
    }

    /// The speed's residual is against the rate a barometer's reading changes when the log's
    /// height is a barometer's, as a speed taken from such a log is.
    #[test]
    fn a_barometers_rate_is_the_speed_times_its_slope() {
        let warm = AtmosphereModel::Standard(Ussa76::with_offset(20.0, 101_325.0).unwrap());
        let environment = environment(Some(warm));
        let simulation = simulated(&environment, flown);
        let moment = simulation.at(10.0).unwrap();
        let barometer = Barometer::on_pad(environment.atmosphere.as_ref(), 1000.0, 0.0).unwrap();
        let (height_m, speed, _) = flown(10.0);
        let slope = (barometer.reading_m(height_m + 0.5).unwrap()
            - barometer.reading_m(height_m - 0.5).unwrap())
            / 1.0;
        assert!(slope < 0.97, "{slope}");
        assert!((moment.reading_speed_m_s - speed * slope).abs() < 1e-3 * speed);
        assert!((moment.speed_m_s - speed).abs() < 1e-9);
    }

    /// A barometric log with a GNSS receiver: fused, the smoother's speed would climb off the
    /// barometer's scale, 7% high on a day 20 K warm, for no real difference in the flight.
    #[test]
    fn a_gnss_channel_leaves_the_speed_on_the_barometers_scale() {
        let warm = AtmosphereModel::Standard(Ussa76::with_offset(20.0, 101_325.0).unwrap());
        let environment = environment(Some(warm));
        let simulation = simulated(&environment, flown);
        let barometer = Barometer::on_pad(environment.atmosphere.as_ref(), 1000.0, 0.0).unwrap();
        let plain = logged(20.0, |t| barometer.reading_m(flown(t).0).unwrap());
        let mut with_gnss = plain.clone();
        with_gnss.channels.push(Channel::new(
            "GPS Altitude",
            Quantity::GnssAltitude,
            plain.time_s.clone(),
            plain
                .time_s
                .iter()
                .map(|&clock| flown((clock - 20.0).clamp(0.0, landed_s())).0)
                .collect(),
        ));
        let speed = |log: &FlightLog| {
            let comparison = compare(log, &readings::read(log), &simulation, None).unwrap();
            let Speed::Read(speed) = comparison.speed_m_s else {
                panic!("{:?}", comparison.speed_m_s);
            };
            speed.phases[0].mean
        };
        assert_eq!(speed(&plain), speed(&with_gnss));
    }

    /// Every source stands for what the page says it stands for.
    #[test]
    fn a_tracks_source_says_what_its_height_stands_for() {
        for source in [Source::Barometer, Source::Pressure, Source::AltitudeChannel] {
            assert_eq!(HeightKind::of(source), HeightKind::Barometric);
        }
        for source in [Source::Gnss, Source::Accelerometer] {
            assert_eq!(HeightKind::of(source), HeightKind::Geometric);
        }
    }

    /// A slow log's samples are compared from its first after liftoff, none counted as past the
    /// flight, and the simulation read at the log's own interval, up to the coarsest the reader
    /// takes.
    #[test]
    fn a_slow_logs_simulation_is_read_at_its_interval() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        for interval_s in [0.33, 0.7, 0.9, 1.5] {
            let log = logged_every(interval_s, 20.0, |t| flown(t).0);
            let comparison = compare(
                &log,
                &readings::read(&log),
                &simulation,
                Some(HeightKind::Geometric),
            )
            .unwrap();
            let read_at = comparison.simulated_interval_s;
            assert!(
                (read_at - interval_s).abs() < 1e-9,
                "{interval_s}: {read_at}"
            );
            // The landing's reading can lie a sample past the simulated ground, but nothing at the
            // start is counted as past the flight.
            assert!(comparison.height_m.past_simulation <= 2, "{interval_s}");
            let first = comparison.height_m.points[0].time_after_liftoff_s;
            assert_eq!(first, 0.0, "{interval_s}");
            let apogee = row(&comparison, What::ApogeeHeight);
            let difference = apogee.logged_minus_simulated.unwrap_or(0.0);
            assert!(difference.abs() < 30.0, "{interval_s}: {apogee:?}");
        }
    }

    /// A simulation landing before the log does leaves samples compared with nothing, counted.
    #[test]
    fn samples_past_the_simulation_are_counted_not_compared() {
        let environment = environment(None);
        // The canopy twice the speed down: the simulation lands long before the log.
        let simulation = {
            let end = 3.0 + 180.0 / 9.8 + 20.0;
            let samples: Vec<TruthSample> = (0..=(end / 0.01) as usize)
                .map(|i| {
                    let t = i as f64 * 0.01;
                    let (height_m, speed, force) = flown(t);
                    TruthSample {
                        time_s: t,
                        height_m,
                        vertical_speed_m_s: speed,
                        axial_specific_force_m_s2: force,
                        mach: 0.0,
                        dynamic_pressure_pa: 0.0,
                        position_enu_m: [0.0; 3],
                        velocity_enu_m_s: [0.0; 3],
                    }
                })
                .collect();
            Simulated::new(samples, &environment).unwrap()
        };
        let log = logged(20.0, |t| flown(t).0);
        let comparison = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        assert!(comparison.height_m.past_simulation > 100);
        let last = comparison.height_m.points.last().unwrap();
        assert!(
            last.time_after_liftoff_s < 3.0 + 180.0 / 9.8 + 20.0 + 0.1,
            "{last:?}"
        );
    }

    /// A reading keeps the provenance M7.2 gave it; the simulation's states none.
    #[test]
    fn a_reading_keeps_its_provenance_beside_a_simulated_number() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let log = logged(20.0, |t| flown(t).0);
        let read = readings::read(&log);
        let comparison = compare(&log, &read, &simulation, Some(HeightKind::Geometric)).unwrap();
        let apogee = read.apogee.value().unwrap();
        let top = row(&comparison, What::ApogeeHeight);
        let Cell::Read(logged) = &top.logged else {
            panic!("{top:?}");
        };
        assert_eq!(logged.provenance, Some(apogee.provenance));
        assert_eq!(logged.source, Some(apogee.source));
        assert_eq!(logged.value, apogee.height_m);
        let Cell::Read(simulated) = &top.simulated else {
            panic!("{top:?}");
        };
        assert_eq!((simulated.provenance, simulated.source), (None, None));
        assert_eq!(simulated.interval, None);
        let speed = read.max_speed.value().unwrap();
        let Cell::Read(logged) = &row(&comparison, What::TopSpeed).logged else {
            panic!("no top speed");
        };
        assert_eq!(logged.provenance, Some(speed.provenance));
    }

    /// A log that holds its height only in a channel has the same residuals as one that holds it
    /// as its own altitude, and the simulation read at its sample interval.
    #[test]
    fn a_height_in_a_channel_is_the_logs_height() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let mut log = logged(20.0, |t| flown(t).0);
        let own = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        let time_s = std::mem::take(&mut log.time_s);
        let altitude_m = std::mem::take(&mut log.altitude_m);
        log.channels.push(Channel::new(
            "Altitude",
            Quantity::AltitudeAboveGround,
            time_s,
            altitude_m,
        ));
        let channel = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        assert_eq!(channel.height_m.points.len(), own.height_m.points.len());
        assert!(!channel.height_m.points.is_empty());
        assert_eq!(channel.simulated_interval_s, own.simulated_interval_s);
    }

    /// A log with no liftoff can't be lined up: refused with the reader's words, not compared at
    /// an arbitrary offset.
    #[test]
    fn a_log_with_no_liftoff_is_refused() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let mut log = logged(20.0, |_| 0.0);
        log.altitude_m.iter_mut().for_each(|h| *h = 0.0);
        let error = compare(&log, &readings::read(&log), &simulation, None).unwrap_err();
        assert!(matches!(error, ResidualsError::LogLiftoff(_)), "{error}");
    }

    /// A log with an accelerometer is read beside a simulation with one, so the top acceleration
    /// has both sides.
    #[test]
    fn a_logged_accelerometer_gets_a_simulated_one() {
        let environment = environment(None);
        // A force with no flat top, which the reader would take for a sensor at its range.
        let shaped = |t: f64| {
            let (height_m, speed, force) = flown(t);
            let burning = t <= 3.0;
            let swell = if burning {
                30.0 * (t / 3.0 * std::f64::consts::PI).sin()
            } else {
                0.0
            };
            (height_m, speed, force + swell)
        };
        let simulation = simulated(&environment, shaped);
        let mut log = logged(20.0, |t| flown(t).0);
        let forces: Vec<f64> = log
            .time_s
            .iter()
            .map(|&clock| match clock - 20.0 {
                t if (0.0..=landed_s()).contains(&t) => shaped(t).2,
                _ => 9.8,
            })
            .collect();
        log.channels.push(Channel::new(
            "axial force",
            Quantity::Acceleration,
            log.time_s.clone(),
            forces,
        ));
        let comparison = compare(
            &log,
            &readings::read(&log),
            &simulation,
            Some(HeightKind::Geometric),
        )
        .unwrap();
        let top = row(&comparison, What::TopAcceleration);
        let difference = top.logged_minus_simulated;
        assert!(difference.is_some_and(|d| d.abs() < 1.0), "{top:?}");
    }

    /// The phases are cut at the log's own burnout and apogee, each sample in one of them.
    #[test]
    fn phases_follow_the_logs_readings() {
        let environment = environment(None);
        let simulation = simulated(&environment, flown);
        let top_s = 3.0 + 180.0 / 9.8;
        // A log with no burnout whole reads one ascent, cut at its apogee.
        let log = logged(20.0, |t| flown(t).0);
        let read = readings::read(&log);
        let comparison = compare(&log, &read, &simulation, Some(HeightKind::Geometric)).unwrap();
        let phases = &comparison.height_m.phases;
        let names: Vec<Phase> = phases.iter().map(|s| s.phase).collect();
        assert_eq!(names, [Phase::Ascent, Phase::Descent]);
        assert!((phases[0].end_s - top_s).abs() < 1.0, "{phases:?}");
        assert_eq!(phases[1].start_s, phases[0].end_s);
        let count: usize = phases.iter().map(|s| s.samples).sum();
        assert_eq!(count, comparison.height_m.points.len(), "{names:?}");
        // A burnout read whole, as an accelerometer's is, splits the ascent; a floor doesn't.
        let mut read = readings::read(&log);
        let Reading::Read(burnout) = &mut read.burnout else {
            panic!("{:?}", read.burnout);
        };
        assert!(burnout.is_floor, "{burnout:?}");
        burnout.is_floor = false;
        let comparison = compare(&log, &read, &simulation, Some(HeightKind::Geometric)).unwrap();
        let phases = &comparison.height_m.phases;
        let names: Vec<Phase> = phases.iter().map(|s| s.phase).collect();
        assert_eq!(names, [Phase::Boost, Phase::Coast, Phase::Descent]);
        assert!((phases[0].end_s - 3.0).abs() < 0.5, "{phases:?}");
        assert!((phases[1].end_s - top_s).abs() < 1.0, "{phases:?}");
    }
}
