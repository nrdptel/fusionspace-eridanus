//! Fitting a flight's simulation to its log: what the flight says of the design's drag, its
//! motor's impulse, the wind and its mass ([M7.3f][m7-3f], [ADR-381][adr-381]).
//!
//! **What it is for.** A flyer with a design and the log of its flight asks what the flight says
//! about the design. [`fit`] changes the drag, the motor's impulse and burn time, a constant wind
//! and the structure's mass until the simulated flight, read as the log's sensor reads it, passes
//! through the log's heights from a second before liftoff to apogee, and through its horizontal
//! track where it holds one. It is the least-squares fit of [`hpr_forensics::fit`] (Levenberg 1944, Marquardt
//! 1963), flown on the launch a [`FlightBuilder`] describes.
//!
//! **What it states.** Each parameter comes with its standard error and, where one can be stated,
//! a 90% interval: the standard error times the shares fitted on synthetic flights
//! ([`hpr_forensics::synthetic::fits::shares`]). The shares were fitted with each parameter held
//! to one stated range ([`hpr_forensics::synthetic::fits::stated`]), so the fit holds a real
//! flight's to the same: a drag scale and an impulse scale about 1, the mass about the design's,
//! and the wind about the wind the launch gives. On a log of height alone that wind is at least
//! the one the shares were fitted in ([`fit`]). Three things come with the numbers:
//!
//! - **Where each reading came from.** The fit reads the log from a second before its liftoff to
//!   its apogee, so [`LogFit::readings`] gives both, and the tracks read, each with its sensor
//!   and whether it was measured or derived.
//! - **The known issues the fitted flight meets** ([`LogFit::issue_warnings`]): the fit's model is
//!   HPR Sim's, so where its drag is known to read wrong ([`hpr_sim::issues`]) the fitted scale
//!   takes that error in.
//! - **A caveat on an interval that fails its test** ([`Caveat`]): the drag scale's does, on
//!   synthetic flights flown with a model unlike the fit's own ([ADR-368][adr-368]).
//!
//! **How far to trust it.** The intervals are tested on synthetic flights only; no real flight
//! has checked them. A scale is the factor that best fits one flight, not the rocket's drag at
//! every Mach number, and the model's own errors go into the scales with nothing to say so.
//!
//! [m7-3f]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3f
//! [adr-368]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
//! [adr-381]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0381-m7-3f-the-fit-on-the-command-line-and-in-python.md

use hpr_analysis::montecarlo::{FlightInputs, dispersed_motor};
use hpr_atmos::{ConstantWind, PowerLawWind};
use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings::interval::Interval;
use hpr_flightdata::readings::{self, Provenance, Reading, Source};
use hpr_forensics::fit::{self as least_squares, Forward, Noise, Parameters, Prior, Stated};
use hpr_forensics::residuals::{HeightKind, Simulated, Track};
use hpr_forensics::synthetic::BURN_TIME_SCALE_DRAWN;
use hpr_forensics::synthetic::fits::{self as calibrated, Asked};
use hpr_forensics::synthetic::truth::SHEAR_REFERENCE_HEIGHT_M;
use hpr_sim::{IssueWarning, Simulation};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::flight::{Flight, FlightBuilder};

/// The issue that holds the fix for the drag scale's interval ([`Caveat`]).
pub const INTERVAL_ISSUE_URL: &str = "https://github.com/nrdptel/fusionspace-eridanus/issues/1108";

/// The issue that records the wind a log's track can't settle ([`LogFit::wind_caveat`]).
pub const WIND_ISSUE_URL: &str = "https://github.com/nrdptel/fusionspace-eridanus/issues/1646";

/// The heights the launch's wind is averaged over for the fit's stated wind, evenly from the pad
/// to the stated flight's apogee.
const WIND_LEVELS: u32 = 100;

/// A parameter of the fit, in its order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Parameter {
    /// The factor on the rocket's zero-lift drag.
    DragScale,
    /// The factor on the motor's total impulse.
    ImpulseScale,
    /// The factor on the motor's burn time. The fit moves it so that a burn's length doesn't
    /// lean on the two scales; it isn't asked of the fit, and states no interval.
    BurnTimeScale,
    /// The constant wind's speed toward the east, m/s.
    #[serde(rename = "wind_east_m_s")]
    WindEast,
    /// The constant wind's speed toward the north, m/s.
    #[serde(rename = "wind_north_m_s")]
    WindNorth,
    /// The factor on the structure's mass: the rocket without its motors.
    MassScale,
}

impl Parameter {
    /// Every parameter, in the fit's order ([`Parameters::to_array`]).
    pub const ALL: [Self; least_squares::FLOWN] = [
        Self::DragScale,
        Self::ImpulseScale,
        Self::BurnTimeScale,
        Self::WindEast,
        Self::WindNorth,
        Self::MassScale,
    ];

    /// Its name, as a line of text gives it.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::DragScale => "drag scale",
            Self::ImpulseScale => "impulse scale",
            Self::BurnTimeScale => "burn time scale",
            Self::WindEast => "wind toward the east",
            Self::WindNorth => "wind toward the north",
            Self::MassScale => "structure mass scale",
        }
    }

    /// Whether it is a speed in m/s; the others are factors with no unit.
    #[must_use]
    pub fn is_speed(self) -> bool {
        matches!(self, Self::WindEast | Self::WindNorth)
    }

    /// The parameter the synthetic flights' shares were fitted for; `None` for the burn time's
    /// scale, which the fit isn't asked for.
    fn asked(self) -> Option<Asked> {
        match self {
            Self::DragScale => Some(Asked::DragScale),
            Self::ImpulseScale => Some(Asked::ImpulseScale),
            Self::BurnTimeScale => None,
            Self::WindEast => Some(Asked::WindEast),
            Self::WindNorth => Some(Asked::WindNorth),
            Self::MassScale => Some(Asked::MassScale),
        }
    }
}

/// Why a parameter states no 90% interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoInterval {
    /// The fit isn't asked for it: no shares were fitted for the burn time's scale.
    NotAsked,
    /// A log of height alone can't tell the wind: nothing in a height speaks to it, so its
    /// value is what was stated of it and an interval would only be the stated range's.
    HeightAlone,
    /// Its standard error sizes no interval: it isn't a number above zero, or an end of the
    /// interval it gives isn't finite.
    NoStandardError,
}

impl NoInterval {
    /// Why, in a few words.
    #[must_use]
    pub fn words(self) -> &'static str {
        match self {
            Self::NotAsked => "not asked of the fit",
            Self::HeightAlone => "a log of height alone can't tell the wind",
            Self::NoStandardError => "its standard error sizes none",
        }
    }
}

/// The caveat an interval carries when it fails its test ([ADR-257][adr-257] §1) and is recorded
/// as failing ([ADR-368][adr-368], [`hpr_forensics::synthetic::fits::KNOWN_GAPS`]): on synthetic
/// flights flown with a model unlike the fit's own, too many truths lay below the interval.
///
/// **Which way it errs.** The truths lie below the interval too often, so the interval's lower
/// end reads too high, and the fitted value with it. For the drag scale that is drag read high:
/// a flight predicted again with that scale comes out low, the wrong side for someone sizing a
/// flight to a waiver's ceiling.
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
/// [adr-368]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Caveat {
    /// The synthetic flights whose truth lay below the interval, as recorded.
    pub below: u64,
    /// The flights of the set.
    pub flights: u64,
    /// How many a 90% interval is meant to leave below it: 5% of the flights.
    pub meant: u64,
}

impl Caveat {
    /// The caveat `asked`'s least-squares interval carries; `None` for one that passes its test.
    fn of(asked: Asked) -> Option<Self> {
        calibrated::known_gap_of(asked).map(|(set, below)| Self {
            below,
            flights: set.count(),
            meant: set.count() / 20,
        })
    }

    /// The caveat on `parameter`'s interval, in sentences.
    #[must_use]
    pub fn message(&self, parameter: Parameter) -> String {
        format!(
            "The {name}'s 90% interval fails its test. On {flights} synthetic flights flown with \
             a model unlike the fit's own, {below} truths lay below the interval, where a 90% \
             interval leaves {meant}. So its lower end can read too high: take the fitted \
             {name} as reading high, and a flight predicted with it as reading low. Recorded, \
             not fixed: {INTERVAL_ISSUE_URL}",
            name = parameter.name(),
            flights = grouped(self.flights),
            below = grouped(self.below),
            meant = grouped(self.meant),
        )
    }
}

/// A count with a comma between its thousands, as text gives one.
fn grouped(count: u64) -> String {
    let digits = count.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// One parameter as the fit found it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fitted {
    /// Which parameter.
    pub parameter: Parameter,
    /// The fit's value: a factor, or for the wind a speed in m/s.
    pub value: f64,
    /// Its standard error, from the fit's own residuals.
    pub standard_error: f64,
    /// What the fit was told of it before the log: normal, `mean ± sd`.
    pub stated: Prior,
    /// Its least-squares 90% interval; `None` where none is stated ([`Fitted::no_interval`]).
    pub interval_90: Option<Interval>,
    /// Why it states no interval; `None` where it states one.
    pub no_interval: Option<NoInterval>,
    /// The caveat its interval carries; `None` for an interval that passes its test, and where
    /// no interval is stated.
    pub caveat: Option<Caveat>,
    /// The range the fit's intervals were tested over: what the synthetic flights drew its true
    /// value from ([`hpr_forensics::synthetic::fits::Asked::drawn`], and the burn time's
    /// [`hpr_forensics::synthetic::BURN_TIME_SCALE_DRAWN`]). For a wind component it is the
    /// fastest wind drawn, either way.
    pub tested_range: Interval,
    /// Whether the fitted value lies outside [`Fitted::tested_range`], or for the wind, whether
    /// the fitted wind is faster than the fastest drawn. No test covers the fit's intervals out
    /// there, this parameter's or another's.
    pub outside_tested_range: bool,
}

/// A time the fit took from the log's readings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRead {
    /// The time, s on the log's clock.
    pub time_s: f64,
    /// The bound its error stays within, s.
    pub bound_s: f64,
    /// The track it was read from.
    pub source: Source,
    /// Whether it was measured or derived.
    pub provenance: Provenance,
}

/// The apogee the fit's window ends at, from the log's readings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ApogeeRead {
    /// Its time, s on the log's clock.
    pub time_s: f64,
    /// The bound the time's error stays within, s.
    pub time_bound_s: f64,
    /// Its height above the pad, m.
    pub height_m: f64,
    /// The bound the height's error stays within, m.
    pub bound_m: f64,
    /// The track it was read from.
    pub source: Source,
    /// Whether it was measured or derived.
    pub provenance: Provenance,
}

/// A track of the log the fit read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackRead {
    /// The sensor it came from.
    pub source: Source,
    /// Whether its values were measured or derived.
    pub provenance: Provenance,
    /// Its bins of [`hpr_forensics::fit::BIN_S`] that hold a sample, from a second before liftoff
    /// to apogee.
    pub bins: usize,
    /// The bins the fit read. A barometer's bins at or past Mach 0.8 on the fitted flight are
    /// left out, as its port reads the flow over the nose there.
    pub bins_read: usize,
}

/// What the fit read from the log, each with where it came from.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LogReadings {
    /// The liftoff the fit's window starts a second before; `None` for a log that starts in
    /// flight, whose first sample stands for it.
    pub liftoff: Option<TimeRead>,
    /// The apogee the window ends at.
    pub apogee: ApogeeRead,
    /// The heights read.
    pub height_track: TrackRead,
    /// The horizontal track read, meters east and north of the pad from the log's latitude and
    /// longitude; `None` for a log of height alone.
    pub horizontal_track: Option<TrackRead>,
}

/// A fit of a flight to its log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogFit {
    /// What the log's height was read as.
    pub height_read_as: HeightKind,
    /// Each parameter, in the fit's order ([`Parameter::ALL`]).
    pub parameters: Vec<Fitted>,
    /// What the fit read from the log, each with its source and provenance.
    pub readings: LogReadings,
    /// The heights' residual's RMS, m: the log's height less the fitted flight's, over the bins
    /// read.
    pub rms_m: f64,
    /// That residual as a share of the spread of the heights read.
    pub misfit_share: f64,
    /// Whether the fit ended where no step lowered its misfit further: not at its limit of 60
    /// steps, or where a flight for its slopes failed.
    pub converged: bool,
    /// The log's clock at the simulation's zero, s.
    pub clock_offset_s: f64,
    /// What the log's pad reads above the simulation's, m.
    pub height_offset_m: f64,
    /// Simulations flown for the fit.
    pub flights_flown: u64,
    /// The apogee of the climb as stated, before the fit, m above the pad: the launch flown as the
    /// fit flies it, with no recovery device opening.
    pub stated_apogee_m: f64,
    /// The apogee of the climb at the fitted parameters, m above the pad, flown the same way;
    /// `None` if it reaches none.
    pub fitted_apogee_m: Option<f64>,
    /// When that climb reaches its apogee, s after ignition; `None` if it reaches none.
    pub fitted_apogee_time_s: Option<f64>,
    /// The top Mach number of the flight at the fitted parameters, flown to the ground with the
    /// launch's recovery: the Mach number its warnings speak of.
    pub fitted_max_mach: Option<f64>,
    /// The known issues the flight at the fitted parameters meets ([`hpr_sim::issues`]), flown to
    /// the ground with the launch's recovery: where HPR Sim's drag or stability is known to read
    /// wrong on this flight, the fit's numbers take that error in.
    pub issue_warnings: Vec<IssueWarning>,
}

impl LogFit {
    /// The parameter's row.
    #[must_use]
    pub fn parameter(&self, parameter: Parameter) -> Option<&Fitted> {
        self.parameters.iter().find(|f| f.parameter == parameter)
    }

    /// The caveat on the wind of a log with a horizontal track; `None` for a log of height alone,
    /// whose wind states no interval.
    ///
    /// The search starts at the wind the fit is told and can stop far from the flight's. On one
    /// design a true wind of 9 m/s, fitted from calm air, came out as a wind the other way with a
    /// narrow interval about it, and the heights' residual didn't show it
    /// ([the issue][WIND_ISSUE_URL]; a test of the command reproduces it). Why isn't settled: on
    /// that design the drift at apogee isn't monotonic in the wind, which the issue records.
    #[must_use]
    pub fn wind_caveat(&self) -> Option<String> {
        self.readings.horizontal_track.as_ref().map(|_| {
            format!(
                "The fitted wind can settle far from the flight's, with a narrow interval about \
                 it: on one design, started from calm air, a true wind of 9 m/s (20 mph) fitted \
                 as a wind the other way, and the heights' residual didn't show it. So give the \
                 day's wind, and take a fitted wind that disagrees with the field's as unsettled. \
                 Recorded, not fixed: {WIND_ISSUE_URL}"
            )
        })
    }

    /// The fit as lines of text, in SI units: what was read with its source and provenance,
    /// each parameter with its interval or why it has none, the caveats and the warnings.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let read = &self.readings;
        lines.push("Read from the log".to_owned());
        if let Some(liftoff) = &read.liftoff {
            lines.push(format!(
                "  liftoff        {:.2} s ± {:.2} s  ({}, from {})",
                liftoff.time_s,
                up(liftoff.bound_s, 2),
                provenance_words(liftoff.provenance),
                source_words(liftoff.source),
            ));
        } else {
            lines.push("  liftoff        the log's first sample: it starts in flight".to_owned());
        }
        lines.push(format!(
            "  apogee         {:.1} m ± {:.1} m at {:.2} s ± {:.2} s  ({}, from {})",
            read.apogee.height_m,
            up(read.apogee.bound_m, 1),
            read.apogee.time_s,
            up(read.apogee.time_bound_s, 2),
            provenance_words(read.apogee.provenance),
            source_words(read.apogee.source),
        ));
        lines.push(format!(
            "  heights        {} of {} bins of {} s from 1 s before liftoff, read as {}  ({}, from {})",
            read.height_track.bins_read,
            read.height_track.bins,
            least_squares::BIN_S,
            match self.height_read_as {
                HeightKind::Barometric => "a barometer's",
                HeightKind::Geometric => "a height climbed",
            },
            provenance_words(read.height_track.provenance),
            source_words(read.height_track.source),
        ));
        match &read.horizontal_track {
            Some(track) => lines.push(format!(
                "  drift          {} bins, east and north of the pad  ({}, from the GNSS fixes)",
                track.bins,
                provenance_words(track.provenance),
            )),
            None => lines.push("  drift          none: a log of height alone".to_owned()),
        }
        lines.push(String::new());
        lines.push("Fitted by least squares".to_owned());
        // One row a parameter: its value, its 90% interval, and what the fit was told of it
        // before the log, so a scale the log pulled far from its stated range shows as one.
        let mut rows = vec![[
            String::new(),
            "fitted".to_owned(),
            "90% interval".to_owned(),
            "told before the log".to_owned(),
        ]];
        for fitted in &self.parameters {
            let unit = if fitted.parameter.is_speed() {
                " m/s"
            } else {
                ""
            };
            let digits = if fitted.parameter.is_speed() { 1 } else { 3 };
            let interval = match &fitted.interval_90 {
                // Rounded outward: a printed interval never claims less than it holds.
                Some(interval) => format!(
                    "{} to {}{unit}{}",
                    shown(down(interval.low, digits), digits),
                    shown(up(interval.high, digits), digits),
                    if fitted.caveat.is_some() { " *" } else { "" },
                ),
                None => "none".to_owned(),
            };
            rows.push([
                fitted.parameter.name().to_owned(),
                format!(
                    "{}{unit}{}",
                    shown(fitted.value, digits),
                    if fitted.outside_tested_range {
                        " †"
                    } else {
                        ""
                    }
                ),
                interval,
                format!(
                    "{} ± {:.digits$}{unit}",
                    shown(fitted.stated.mean, digits),
                    fitted.stated.sd
                ),
            ]);
        }
        let width = |column: usize| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        };
        let (name, value, interval) = (width(0), width(1), width(2));
        for row in &rows {
            lines.push(
                format!(
                    "  {:<name$}  {:<value$}  {:<interval$}  {}",
                    row[0], row[1], row[2], row[3]
                )
                .trim_end()
                .to_owned(),
            );
        }
        if self.parameters.iter().any(|f| f.caveat.is_some()) {
            lines.push("  * This interval fails its test: the caveat below.".to_owned());
        }
        lines.extend(self.outside_lines());
        for (why, whose) in [
            (NoInterval::NotAsked, "The burn time scale"),
            (NoInterval::HeightAlone, "The wind"),
        ] {
            if self.parameters.iter().any(|f| f.no_interval == Some(why)) {
                lines.push(format!("  {whose} states no interval: {}.", why.words()));
            }
        }
        if self
            .parameters
            .iter()
            .any(|f| f.no_interval == Some(NoInterval::HeightAlone))
        {
            // A floor is rounded down: the fit is never told less than the line says.
            lines.push(format!(
                "  So the fit is told a wind of at least {:.2} m/s, the wind its intervals were sized \
                 in.",
                down(height_alone_wind_m_s(), 2)
            ));
        }
        for fitted in &self.parameters {
            if fitted.no_interval == Some(NoInterval::NoStandardError) {
                lines.push(format!(
                    "  The {} states no interval: {}.",
                    fitted.parameter.name(),
                    NoInterval::NoStandardError.words()
                ));
            }
        }
        lines.push(format!(
            "  The heights' residual: {:.1} m RMS, {:.1}% of the heights' spread, over {} flights \
             flown.{}",
            self.rms_m,
            100.0 * self.misfit_share,
            self.flights_flown,
            if self.converged {
                ""
            } else {
                " The fit stopped before it settled: take its numbers as rough."
            },
        ));
        for fitted in &self.parameters {
            if let Some(caveat) = &fitted.caveat {
                lines.push(String::new());
                lines.push(format!("Caveat. {}", caveat.message(fitted.parameter)));
            }
        }
        if let Some(caveat) = self.wind_caveat() {
            lines.push(String::new());
            lines.push(format!("Caveat. {caveat}"));
        }
        for warning in &self.issue_warnings {
            lines.push(String::new());
            lines.push(format!("Warning. {}", warning.message()));
        }
        lines
    }
}

impl LogFit {
    /// A line for each parameter fitted outside the range its interval was tested over, in SI
    /// units; the wind's two components share one.
    fn outside_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut wind = false;
        for fitted in self.parameters.iter().filter(|f| f.outside_tested_range) {
            let tested = fitted.tested_range;
            if fitted.parameter.is_speed() {
                if !wind {
                    lines.push(format!(
                        "  † The wind is faster than {:.1} m/s, the fastest the intervals were tested \
                         in.",
                        down(tested.high, 1)
                    ));
                }
                wind = true;
            } else {
                lines.push(format!(
                    "  † The {} lies outside {:.3} to {:.3}, the range the intervals were tested \
                     over.",
                    fitted.parameter.name(),
                    up(tested.low, 3),
                    down(tested.high, 3)
                ));
            }
        }
        lines
    }
}

/// `value` to `digits` decimal places, one that rounds to zero written without a sign.
fn shown(value: f64, digits: usize) -> String {
    let text = format!("{value:.digits$}");
    match text.strip_prefix('-') {
        Some(rest) if rest.chars().all(|c| c == '0' || c == '.') => rest.to_owned(),
        _ => text,
    }
}

/// `value` rounded up to `digits` decimal places, for a bound or an interval's upper end: a
/// printed bound never claims less than the number holds. A part in 10⁹ is let go, so a value
/// stored a hair over its exact decimal isn't printed a step higher.
fn up(value: f64, digits: usize) -> f64 {
    let scale = 10_f64.powi(i32::try_from(digits).unwrap_or(0));
    let scaled = value * scale;
    (scaled - scaled.abs() * 1e-9).ceil() / scale
}

/// `value` rounded down to `digits` decimal places, for an interval's lower end.
fn down(value: f64, digits: usize) -> f64 {
    -up(-value, digits)
}

/// The least wind speed the fit is told on a log of height alone, m/s: the wind the synthetic
/// flights' fits of such logs were told, the stated range's RMS speed
/// ([`hpr_forensics::synthetic::fits::stated_for`]).
#[must_use]
pub fn height_alone_wind_m_s() -> f64 {
    calibrated::height_alone_wind_m_s()
}

/// A reading's provenance, in a word.
#[must_use]
pub fn provenance_words(provenance: Provenance) -> &'static str {
    match provenance {
        Provenance::Measured => "measured",
        Provenance::Derived => "derived",
        Provenance::Clipped => "clipped",
        _ => "other",
    }
}

/// A reading's source, in a few words.
#[must_use]
pub fn source_words(source: Source) -> &'static str {
    match source {
        Source::Barometer => "the logger's barometric altitude",
        Source::Pressure => "the pressure, by the 1976 standard atmosphere",
        Source::AltitudeChannel => "the barometric altitude channel",
        Source::Gnss => "the GNSS altitude",
        Source::Accelerometer => "the accelerometer",
        _ => "another track",
    }
}

/// Why a log can't be fitted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Refused {
    /// The log's readings don't give the fit its window.
    #[error("the log can't be fitted: {0}")]
    Log(String),
    /// The flight comes apart on the way up, which the fit's model doesn't fly.
    #[error(
        "the flight has {0} separation(s) before it lands, and the fit flies one body from \
         liftoff to apogee"
    )]
    Staged(usize),
    /// The flight as stated can't be flown.
    #[error(transparent)]
    Flight(#[from] Error),
    /// The flight as stated reaches no apogee.
    #[error("the flight as stated reaches no apogee, so there is no climb to fit")]
    NoApogee,
    /// A flight the fit asked for couldn't be flown, before it had an answer.
    #[error("the fit couldn't fly the flight it needed ({flown} flown)")]
    NotFlown {
        /// Simulations flown before it stopped.
        flown: u64,
    },
    /// The best fit still misses the log by more than
    /// [`hpr_forensics::fit::MISFIT_SHARE`] of the heights' spread: the log is some other
    /// flight's than this design's, or the launch isn't as given.
    #[error(
        "no flight of this design passes through the log: the best misses its heights by \
         {percent:.0}% of their spread, past the 10% a fit may",
        percent = 100.0 * misfit_share
    )]
    SaysNothing {
        /// The heights' residual's RMS as a share of the heights' spread.
        misfit_share: f64,
    },
}

/// Flies the launch for the fit: its design, rail, site and air, with the fit's scales and its
/// constant wind in place of the launch's own.
///
/// It is meant to fly what [`hpr_forensics::synthetic::fits::Flown`], the model the 90%
/// intervals' shares were fitted with, flies for the same launch and parameters; a test holds
/// the two to it.
pub(crate) struct Flown {
    /// The launch, with no recovery: the fit reads the climb.
    inputs: FlightInputs,
    /// The first flight's simulation, whose supersonic table the others fly on.
    tables: Option<Simulation>,
    /// The pad's height above mean sea level, m, which a sheared wind is measured above.
    ground_msl_m: f64,
}

impl Flown {
    /// The fit's flights of the launch `inputs`, its recovery already taken off.
    pub(crate) fn new(inputs: FlightInputs) -> Self {
        let environment = &inputs.environment;
        Self {
            ground_msl_m: environment.site().height_m - environment.geoid_undulation_m,
            inputs,
            tables: None,
        }
    }

    /// The launch's inputs with `p`'s scales on its motors and drag, and `p`'s wind, constant
    /// or growing with height by `shear`.
    fn inputs(&self, p: &Parameters, shear: Option<f64>) -> Result<FlightInputs, String> {
        if p.drag_scale <= 0.0 || p.impulse_scale <= 0.0 || p.burn_time_scale <= 0.0 {
            return Err("a scale that isn't positive".to_owned());
        }
        let speed_m_s = p.wind_east_m_s.hypot(p.wind_north_m_s);
        // It blows toward (east, north), so from the opposite way, clockwise from north.
        let from_rad = (-p.wind_east_m_s)
            .atan2(-p.wind_north_m_s)
            .rem_euclid(std::f64::consts::TAU);
        let mut inputs = self.inputs.clone();
        inputs.environment = match shear {
            None => inputs
                .environment
                .with_wind(ConstantWind::new(speed_m_s, from_rad).map_err(|e| e.to_string())?),
            Some(exponent) => inputs.environment.with_wind(
                PowerLawWind::new(
                    speed_m_s,
                    SHEAR_REFERENCE_HEIGHT_M,
                    exponent,
                    from_rad,
                    self.ground_msl_m,
                )
                .map_err(|e| e.to_string())?,
            ),
        };
        for configuration in &mut inputs.rocket.configurations {
            for mounted in &mut configuration.motors {
                mounted.motor = dispersed_motor(&mounted.motor, p.impulse_scale, p.burn_time_scale)
                    .map_err(|e| e.to_string())?;
            }
        }
        inputs.drag_scale *= p.drag_scale;
        Ok(inputs)
    }

    /// The simulation of `inputs` with the structure's mass scaled, on the shared table.
    fn simulation(&mut self, inputs: &FlightInputs, mass_scale: f64) -> Result<Simulation, String> {
        let mut simulation = inputs.simulation().map_err(|e| e.to_string())?;
        match &self.tables {
            Some(first) => {
                simulation.share_supersonic_table(first.aero());
            }
            None => self.tables = Some(inputs.simulation().map_err(|e| e.to_string())?),
        }
        simulation
            .with_structure_mass_scale(mass_scale)
            .map_err(|e| e.to_string())
    }

    /// The flight on `p`, flown for `until_s`.
    fn flown(
        &mut self,
        p: &Parameters,
        until_s: f64,
        shear: Option<f64>,
    ) -> Result<Simulated, String> {
        let mut inputs = self.inputs(p, shear)?;
        inputs.settings.max_time_s = until_s;
        let simulation = self.simulation(&inputs, p.mass_scale)?;
        let mut track = Track::new();
        simulation.run(&mut track).map_err(|e| e.to_string())?;
        track
            .simulated(&inputs.environment)
            .map_err(|e| e.to_string())
    }
}

impl Forward for Flown {
    fn fly(&mut self, p: &Parameters, until_s: f64) -> Result<Simulated, String> {
        self.flown(p, until_s, None)
    }

    fn fly_sheared(
        &mut self,
        p: &Parameters,
        until_s: f64,
        exponent: f64,
    ) -> Result<Simulated, String> {
        self.flown(p, until_s, Some(exponent))
    }
}

/// The launch's wind toward the east and the north, m/s, averaged evenly in height over a climb
/// of `climb_m` from the pad: what the fit is told of its constant wind.
fn mean_wind(environment: &hpr_sim::Environment, climb_m: f64) -> Result<(f64, f64), Error> {
    let ground_msl_m = environment.site().height_m - environment.geoid_undulation_m;
    let (mut east, mut north) = (0.0, 0.0);
    let levels = f64::from(WIND_LEVELS);
    for k in 0..WIND_LEVELS {
        let height = ground_msl_m + climb_m * (f64::from(k) + 0.5) / levels;
        let v = environment
            .wind
            .wind(height)
            .map_err(hpr_sim::SimError::from)?
            .velocity_enu_m_s;
        east += v.x / levels;
        north += v.y / levels;
    }
    Ok((east, north))
}

/// What the fit is told of the wind's mean, m/s toward the east and the north, from the
/// launch's `mean` wind.
///
/// On a log with a track it is the launch's. A log of height alone can't tell the wind, and a
/// wind of either sign lowers a flight's top, which a fit in no wind puts in the drag and the
/// impulse ([`hpr_forensics::synthetic::fits::stated_for`]). The synthetic flights' fits of such
/// logs were told the stated range's RMS speed, and the shares that size the intervals were
/// fitted that way: no test covers a fit told less. So on a log of height alone a launch's wind
/// slower than that is raised to it, along its own direction, or toward the east in calm air, as
/// those fits were told. A stronger wind stands: the variance the fit adds for the wind it can't
/// tell grows with the wind it is told (`hpr_forensics::fit`'s strong wind).
fn stated_wind(mean: (f64, f64), height_alone: bool) -> (f64, f64) {
    let (east, north) = mean;
    let least = height_alone_wind_m_s();
    let speed = east.hypot(north);
    if !height_alone || speed >= least {
        (east, north)
    } else if speed > f64::EPSILON {
        (east * least / speed, north * least / speed)
    } else {
        (least, 0.0)
    }
}

/// Fits `launch`'s flight to `log` by least squares.
///
/// The launch gives the design, its motor, the rail, the site and the air, as a simulation of
/// the flight would take them. The log's height is read as a barometer's or as a height climbed
/// by the track its apogee came from, or as `height_read_as` says. The fit is told each
/// parameter's stated range ([`hpr_forensics::synthetic::fits::stated`]), its wind centered on
/// the launch's own, averaged over the stated climb. On a log of height alone that wind is at
/// least [`height_alone_wind_m_s`], the wind the intervals' shares were fitted in.
///
/// The fit flies the climb with no recovery device opening, and the stated and fitted apogees
/// are that climb's. The warnings are those of the flight at the fitted parameters, flown to the
/// ground with the launch's recovery.
///
/// # Errors
///
/// [`Refused`]: a log with no apogee or too little of the climb, a flight that separates, a
/// launch that can't be flown, or a best fit that still misses the log.
pub fn fit(
    launch: &FlightBuilder<'_>,
    log: &FlightLog,
    height_read_as: Option<HeightKind>,
) -> Result<LogFit, Refused> {
    let read = readings::read(log);
    let apogee = match &read.apogee {
        Reading::Read(apogee) => *apogee,
        Reading::Withheld(withheld) => {
            return Err(Refused::Log(format!(
                "its apogee is withheld: {}",
                withheld.detail
            )));
        }
    };
    let observed = match height_read_as {
        Some(kind) => least_squares::observations_as(log, &read, kind),
        None => least_squares::observations(log, &read),
    }
    .ok_or_else(|| {
        Refused::Log(
            "its climb can't be read: the pad's height is withheld, it holds no heights, or its \
             climb is too short or too long"
                .to_owned(),
        )
    })?;
    let mut inputs = launch.inputs()?;
    if !inputs.separations.is_empty() {
        return Err(Refused::Staged(inputs.separations.len()));
    }

    // The fit flies the climb with nothing open, so the climb as stated is flown the same way:
    // its apogee is the one the fitted climb's is set beside, and it sizes the climb the launch's
    // wind is averaged over.
    let recovery = std::mem::take(&mut inputs.recovery);
    let own_drag = inputs.drag.is_some();
    let stated_climb = Flight::flown(
        inputs.simulation().map_err(Error::from)?,
        &inputs.environment,
        own_drag,
        &mut (),
    )?;
    let stated_apogee_m = stated_climb.apogee_m().ok_or(Refused::NoApogee)?;
    let (wind_east_m_s, wind_north_m_s) = stated_wind(
        mean_wind(&inputs.environment, stated_apogee_m)?,
        observed.horizontal.is_empty(),
    );
    let mut stated: Stated = calibrated::stated();
    stated[Asked::WindEast.index()].mean = wind_east_m_s;
    stated[Asked::WindNorth.index()].mean = wind_north_m_s;

    let mut forward = Flown::new(inputs);
    let answer = least_squares::fit(Some(&observed), &stated, Noise::FromResiduals, &mut forward);
    if !answer.fitted {
        return Err(match answer.misfit_share {
            Some(misfit_share) => Refused::SaysNothing { misfit_share },
            None => Refused::NotFlown {
                flown: answer.flown,
            },
        });
    }
    let track = !observed.horizontal.is_empty();
    let values = answer.parameters.to_array();
    let wind_speed_m_s = answer
        .parameters
        .wind_east_m_s
        .hypot(answer.parameters.wind_north_m_s);
    let parameters = Parameter::ALL
        .iter()
        .zip(values.iter().zip(&answer.standard_errors).zip(&stated))
        .map(|(&parameter, ((&value, &standard_error), &stated))| {
            let asked = parameter.asked();
            let no_interval = match asked {
                None => Some(NoInterval::NotAsked),
                Some(asked) if !track && !asked.stated_on_height_alone() => {
                    Some(NoInterval::HeightAlone)
                }
                Some(_) => None,
            };
            let interval_90 = asked
                .filter(|_| no_interval.is_none())
                .and_then(|asked| calibrated::shares(asked, track).interval(value, standard_error));
            // Shares state no interval only on a standard error that isn't a number above zero.
            let no_interval = no_interval
                .or_else(|| interval_90.is_none().then_some(NoInterval::NoStandardError));
            let (low, high) = asked.map_or(BURN_TIME_SCALE_DRAWN, Asked::drawn);
            let tested_range = Interval { low, high };
            let outside_tested_range = if parameter.is_speed() {
                wind_speed_m_s > high
            } else {
                value < low || value > high
            };
            Fitted {
                parameter,
                value,
                standard_error,
                stated,
                interval_90,
                no_interval,
                caveat: asked.filter(|_| interval_90.is_some()).and_then(Caveat::of),
                tested_range,
                outside_tested_range,
            }
        })
        .collect();

    // The climb the fit found, flown as the fit flew it: its apogee is the fitted one. Then the
    // same flight flown whole with the launch's recovery, where it has any: its warnings are the
    // fit's.
    let mut fitted_inputs =
        forward
            .inputs(&answer.parameters, None)
            .map_err(|_| Refused::NotFlown {
                flown: answer.flown,
            })?;
    let flown = |inputs: &FlightInputs| -> Result<Flight, Error> {
        let simulation = inputs.simulation().and_then(|simulation| {
            simulation.with_structure_mass_scale(answer.parameters.mass_scale)
        })?;
        Flight::flown(simulation, &inputs.environment, own_drag, &mut ())
    };
    let fitted_climb = flown(&fitted_inputs)?;
    let fitted_whole = if recovery.is_empty() {
        None
    } else {
        fitted_inputs.recovery = recovery;
        Some(flown(&fitted_inputs)?)
    };
    let warned = fitted_whole.as_ref().unwrap_or(&fitted_climb);

    let rms_m = answer.rms_m.unwrap_or(f64::NAN);
    let bins_read = answer.read.iter().filter(|read| **read).count();
    Ok(LogFit {
        height_read_as: observed.kind,
        parameters,
        readings: LogReadings {
            liftoff: match &read.liftoff {
                Reading::Read(liftoff) => Some(TimeRead {
                    time_s: liftoff.time_s,
                    bound_s: liftoff.bound_s,
                    source: liftoff.source,
                    provenance: liftoff.provenance,
                }),
                Reading::Withheld(_) => None,
            },
            apogee: ApogeeRead {
                time_s: apogee.time_s,
                time_bound_s: apogee.time_bound_s,
                height_m: apogee.height_m,
                bound_m: apogee.bound_m,
                source: apogee.source,
                provenance: apogee.provenance,
            },
            height_track: TrackRead {
                source: readings::height_track_source(log).unwrap_or(apogee.source),
                provenance: apogee.provenance,
                bins: observed.heights.len(),
                bins_read,
            },
            horizontal_track: track.then_some(TrackRead {
                source: Source::Gnss,
                provenance: Provenance::Measured,
                bins: observed.horizontal.len(),
                bins_read: observed.horizontal.len(),
            }),
        },
        rms_m,
        misfit_share: answer.misfit_share.unwrap_or(f64::NAN),
        converged: answer.converged,
        clock_offset_s: answer.clock_offset_s,
        height_offset_m: answer.height_offset_m,
        flights_flown: answer.flown,
        stated_apogee_m,
        fitted_apogee_m: fitted_climb.apogee_m(),
        fitted_apogee_time_s: fitted_climb.apogee_time_s(),
        fitted_max_mach: warned.max_mach(),
        issue_warnings: warned.issue_warnings().to_vec(),
    })
}
