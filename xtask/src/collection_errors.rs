//! `cargo xtask fixture-flights` also predicts each cut collection flight's apogee from its boost,
//! and gives the errors by M2.7's halves (M7.3d6, ADR-322, ADR-326, ADR-329).
//!
//! **The flight.** Each flight with a cut log, among the flights the forward report compares, is
//! flown as `fixture-flights` flies it in hpr ([`ForwardFlight`]): its `.ork` and configuration,
//! with the separations the climb flies, its rail, and its day's ERA5 air. The fit's model changes
//! only what the fit asks: the motors' impulse and burn, the drag, the structure's mass, and a
//! wind constant with height. Flown with nothing changed in ERA5's own wind, it must land within
//! [`SAME_FLIGHT_WITHIN_M`] of the forward apogee, or the run fails: the check that it is the
//! forward simulation's flight.
//!
//! **The prediction.** The log is cut, fitted up to the cut and flown on to apogee by
//! `hpr_forensics::boost::predict`, told what `hpr_forensics::synthetic::fits::stated` states of
//! each parameter, with the wind's centered on ERA5's averaged evenly in height over the forward
//! climb, as the public flights' report does. Its apogee is the height climbed, set against the
//! logged apogee as a height climbed in the day's air, the forward errors' reference: both errors
//! are taken against the same number.
//!
//! **The halves.** Each half's errors, its paired bootstrap 95% interval
//! ([`boost::paired_bootstrap_interval`], [`RESAMPLES`] resamples, seed [`SEED`]) and the exact
//! sign-flip interval beside it are published. The calibration half's fit aggregates are, taken
//! through [`Calibration`], which refuses a held-out flight (M2.7a, ADR-332); the
//! held-out half's are written to the gitignored [`HELD_OUT_FITS`], and only that file's SHA-256 is
//! committed, in [`HELD_OUT_FITS_SHA`], until M7.5 publishes them (ADR-322 §2).
//!
//! **Before the shared bias's correction (M7.5a, ADR-346).** Each set's forward error is split at
//! the cut: the prediction's error is the share made after it, the forward error less the
//! prediction's the share made up to it. And each calibration flight predicted is fitted once
//! more with the impulse's stated range widened [`WIDENED`]-fold, for what its log alone says of
//! the boost ([`Implied`]); a flight whose log and files disagree by more than NAR's retest
//! rule allows one firing is flagged ([`Implied::disagrees`]). No correction is fitted
//! across flights here, and the held-out half's logs are not fitted this way until that
//! correction is judged (M7.5b).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr::hpr_analysis::montecarlo::dispersed_motor;
use hpr_atmos::{ConstantWind, PowerLawWind};
use hpr_flightdata::log::FlightLog;
use hpr_forensics::boost::{self, Unpredicted};
use hpr_forensics::fit::{Fit, Forward, Noise, Parameters, Stated};
use hpr_forensics::residuals::{HeightKind, Simulated, Track};
use hpr_forensics::synthetic::fits::{self as synthetic_fits, Asked};
use hpr_forensics::synthetic::truth::SHEAR_REFERENCE_HEIGHT_M;
use hpr_sim::{Environment, FlightSettings, Simulation};
use hpr_validate::correction::Correction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::collection_boost::Calibration;
use crate::collection_correction::{self, FitJudged, Judged, Pair, ScaleFit, SetJudged};
use crate::fixture_flights::{ForwardFlight, as_flown};

/// The interval's coverage, as ADR-257 §3 asks.
pub const LEVEL: f64 = 0.95;
/// The bootstrap's resamples.
pub const RESAMPLES: u32 = 10_000;
/// The bootstrap's seed: this decision's record's number, chosen before any half's number.
pub const SEED: u64 = 329;
/// How far the fit's model, flown in ERA5's own wind with nothing changed, may land from the
/// forward apogee, m: the same flight, its apogee read from a track's samples, not its apogee
/// event.
pub const SAME_FLIGHT_WITHIN_M: f64 = 0.5;
/// The held-out half's fit aggregates (private, gitignored).
pub const HELD_OUT_FITS: &str = "corpus-out/hpr-sim-fixtures/held-out-fits.json";
/// The SHA-256 of [`HELD_OUT_FITS`], committed.
pub const HELD_OUT_FITS_SHA: &str = "validation/reports/held-out-fits.sha256";
/// The halves' names, the calibration half first.
pub const CALIBRATION: &str = "calibration";
/// The held-out half's name.
pub const HELD_OUT: &str = "held out";
/// How many times wider the impulse scale's stated range is made for the fit of the log alone
/// (ADR-346 §2): wide enough that the range no longer pulls the fit toward the rated impulse. The
/// widened range's standard deviation is 3.28, a weight of 1/3.28² ≈ 0.09. A fit with the
/// calibration half's average standard error, 0.035, weighs 1/0.035² ≈ 800: the range's pull is
/// about a ten-thousandth of the log's.
pub const WIDENED: f64 = 100.0;
/// How far one recertification firing's total impulse may lie from the certified mean before NAR
/// retests the motor as a new type, a share of that mean: NAR Motor Testing Manual v1.5 (2011),
/// §6.2.3, p. 11. The manual says it of a test firing; taking it as the most a flown motor's
/// lot strays is ADR-346's inference. A boost farther than this from the rated impulse, at the
/// file's mass, is then unlikely to be the lot: the files, or the model of the boost, are.
pub const RECERTIFICATION_LIMIT: f64 = 0.131;
/// How many of its own standard errors a log-implied impulse scale must clear
/// [`RECERTIFICATION_LIMIT`] by for its flight to be flagged (ADR-346 §3).
pub const FLAG_STANDARD_ERRORS: f64 = 2.0;

/// The fit's flights of one collection flight.
pub(crate) struct Flown<'a> {
    pub forward: &'a ForwardFlight,
    /// The pad's height above sea level, m: where a sheared wind's ground is.
    pub pad_msl_m: f64,
    /// Flies ERA5's own wind, whatever the parameters say of it.
    pub era5_wind: bool,
}

impl Flown<'_> {
    pub(crate) fn flown(
        &self,
        p: &Parameters,
        until_s: f64,
        shear: Option<f64>,
    ) -> Result<Simulated, String> {
        if p.drag_scale <= 0.0 || p.impulse_scale <= 0.0 || p.burn_time_scale <= 0.0 {
            return Err("a scale that isn't positive".to_owned());
        }
        let speed_m_s = p.wind_east_m_s.hypot(p.wind_north_m_s);
        // It blows toward (east, north), so from the opposite way, clockwise from north.
        let from_rad = (-p.wind_east_m_s)
            .atan2(-p.wind_north_m_s)
            .rem_euclid(std::f64::consts::TAU);
        let day = &self.forward.day;
        let environment: Environment = match (self.era5_wind, shear) {
            (true, _) => day.clone(),
            (false, None) => day
                .clone()
                .with_wind(ConstantWind::new(speed_m_s, from_rad).map_err(|e| e.to_string())?),
            (false, Some(exponent)) => day.clone().with_wind(
                PowerLawWind::new(
                    speed_m_s,
                    SHEAR_REFERENCE_HEIGHT_M,
                    exponent,
                    from_rad,
                    self.pad_msl_m,
                )
                .map_err(|e| e.to_string())?,
            ),
        };
        let mut rocket = self.forward.design.rocket.clone();
        for configuration in &mut rocket.configurations {
            for mounted in &mut configuration.motors {
                mounted.motor = dispersed_motor(&mounted.motor, p.impulse_scale, p.burn_time_scale)
                    .map_err(|e| e.to_string())?;
            }
        }
        let settings = FlightSettings {
            max_time_s: until_s,
            accept_design_errors: true,
            ..FlightSettings::default()
        };
        let simulation = Simulation::new(
            &rocket,
            &self.forward.configuration.id,
            environment.clone(),
            self.forward.rail,
            settings,
        )
        .map_err(|e| e.to_string())?
        .with_drag_scale(p.drag_scale)
        .map_err(|e| e.to_string())?
        .with_structure_mass_scale(p.mass_scale)
        .map_err(|e| e.to_string())?;
        let simulation = as_flown(simulation, &self.forward.configuration)?;
        let mut track = Track::new();
        simulation.run(&mut track).map_err(|e| e.to_string())?;
        track.simulated(&environment).map_err(|e| e.to_string())
    }
}

impl Forward for Flown<'_> {
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

/// A cut flight, as `collection_boost` hands it over.
pub(crate) struct CutFlight {
    /// Its manifest id.
    pub flight_id: String,
    /// Its chosen log.
    pub log: FlightLog,
    /// Whether the cut came less than the motor's burn after the log's liftoff; `None` where
    /// either isn't known.
    pub cut_in_burn: Option<bool>,
    /// Whether another of the flight's logs on a barometer's track gave an apogee.
    pub other_logs: bool,
    /// Whether those logs' apogees lie more than `collection_boost::GAP_SHARE` of the logged
    /// apogee apart.
    pub other_logs_apart: bool,
}

/// Whether a flight's log is fitted alone: every predicted flight's. A calibration flight's feeds
/// the logs-alone table and the correction's fits, through [`Calibration`]; a held-out flight's
/// gives its flag alone, made in the run that judges the correction (M7.5b, ADR-346 §5) and
/// never fed to a fit across flights.
fn reads_log_alone(predicted: bool) -> bool {
    predicted
}

/// What the fit of the log alone is told: what the prediction's fit was, with only the impulse
/// scale's standard deviation made [`WIDENED`] times wider (ADR-346 §2).
pub(crate) fn stated_alone(mut stated: Stated) -> Stated {
    stated[Asked::ImpulseScale.index()].sd *= WIDENED;
    stated
}

/// What a flight's log alone says of its boost: the impulse scale of the fit made with that
/// scale's stated range widened [`WIDENED`]-fold, everything else as the prediction's fit. It is
/// the impulse the climb to the cut shows over the motor's rated impulse, at the file's mass and
/// the model's drag: a barometer's boost can't tell less impulse from more mass, and in a boost
/// the drag and the impulse trade against each other
/// (`docs/research/drag-impulse-identifiability.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Implied {
    /// The impulse scale.
    pub scale: f64,
    /// Its standard error.
    pub standard_error: f64,
    /// The drag scale's standard error in that fit: near what the fit was told of the drag where
    /// the climb to the cut says nothing of it.
    pub drag_standard_error: f64,
    /// Whether that fit converged.
    pub converged: bool,
}

impl Implied {
    pub(crate) fn of(fit: &Fit) -> Self {
        Self {
            scale: fit.parameters.impulse_scale,
            standard_error: fit.standard_errors[Asked::ImpulseScale.index()],
            drag_standard_error: fit.standard_errors[Asked::DragScale.index()],
            converged: fit.converged,
        }
    }

    /// Whether the log and the files disagree by more than NAR's retest rule allows one firing
    /// (ADR-346 §3):
    /// the scale lies farther from 1 than [`RECERTIFICATION_LIMIT`], by more than
    /// [`FLAG_STANDARD_ERRORS`] of its standard errors, either way; a scale exactly that far
    /// isn't flagged. A fit that didn't converge, or came to no number, shows no disagreement.
    pub(crate) fn disagrees(&self) -> bool {
        // A standard error that is no number fails `>= 0.0`; an unbounded one clears nothing.
        self.converged
            && self.scale.is_finite()
            && self.standard_error >= 0.0
            && (self.scale - 1.0).abs() - FLAG_STANDARD_ERRORS * self.standard_error
                > RECERTIFICATION_LIMIT
    }
}

/// One flight's outcome, kept on this machine.
struct Outcome {
    flight_id: String,
    held_out: bool,
    /// The prediction's error and the forward simulation's against the same logged apogee, %,
    /// and the fit; or why there is none.
    result: Result<(f64, f64, Fit), String>,
    early: bool,
    cut_in_burn: Option<bool>,
    other_logs: bool,
    other_logs_apart: bool,
    same_flight_m: Option<f64>,
    /// What the log alone says of the boost, or why the widened fit came to nothing; `None`
    /// where the log wasn't read alone ([`reads_log_alone`]).
    implied: Option<Result<Implied, String>>,
    /// A calibration flight's drag at which its boost, refitted with the drag held there, flies
    /// on to the logged apogee, or why there is none (ADR-346 §4); `None` for a held-out flight,
    /// one with a deployment before apogee, or one not predicted.
    drag_solved: Option<Result<f64, String>>,
}

/// What the report names a prediction's absence by.
fn unpredicted_name(why: &Unpredicted) -> String {
    match why {
        Unpredicted::NoCut(_) => "no cut on the second reading".to_owned(),
        Unpredicted::NothingToFit(_) => "too little up to the cut to fit".to_owned(),
        Unpredicted::NotFitted { .. } => "the fit kept to what was stated".to_owned(),
        Unpredicted::NoApogee(_) => "the fitted flight reaches no apogee".to_owned(),
    }
}

/// Fits and predicts one flight, on its forward flight or the reason it has none (`None`: the
/// forward report never reached it).
fn predict(
    cut: &CutFlight,
    forward: Option<&Result<ForwardFlight, &'static str>>,
) -> Result<Outcome, String> {
    let held_out = crate::collection_boost::held_out(&cut.flight_id);
    let forward = match forward {
        Some(Ok(forward)) => forward,
        unflown => {
            let why = match unflown {
                Some(Err(why)) => why,
                _ => NOT_REACHED,
            };
            return Ok(Outcome {
                flight_id: cut.flight_id.clone(),
                held_out,
                result: Err((*why).to_owned()),
                early: false,
                cut_in_burn: cut.cut_in_burn,
                other_logs: cut.other_logs,
                other_logs_apart: cut.other_logs_apart,
                same_flight_m: None,
                implied: None,
                drag_solved: None,
            });
        }
    };
    let at = &cut.flight_id;
    let (stated, pad_msl_m) = stated_for(at, forward)?;
    let nominal = Parameters::from_array(stated.map(|p| p.mean));
    let mut flown = Flown {
        forward,
        pad_msl_m,
        era5_wind: true,
    };
    let same_flight_m = flown
        .fly(&nominal, boost::APOGEE_HORIZON_S)
        .map_err(|error| format!("{at}: {error}"))?
        .top_reading_m(HeightKind::Geometric)
        .ok_or_else(|| format!("{at}: the fit's model reaches no apogee"))?
        - forward.hpr_m;
    if !same_flight_m.is_finite() || same_flight_m.abs() > SAME_FLIGHT_WITHIN_M {
        return Err(format!(
            "{at}: the fit's model in ERA5's wind lands {same_flight_m:+.3} m from the forward \
             apogee"
        ));
    }
    flown.era5_wind = false;
    let percent = |apogee_m: f64| 100.0 * (apogee_m - forward.logged_m) / forward.logged_m;
    let result = match boost::predict(&cut.log, |_| stated, Noise::FromResiduals, &mut flown) {
        Ok(prediction) => Ok((
            percent(prediction.apogee_above_pad_m),
            percent(forward.hpr_m),
            prediction.fit,
        )),
        Err(why) => Err(unpredicted_name(&why)),
    };
    // What the log alone says of the boost: the same fit, the impulse's stated range widened
    // until it no longer binds (ADR-346 §2). A calibration flight's only: the held-out half's
    // logs are fitted this way when the correction is judged (M7.5b), not before.
    let implied = reads_log_alone(result.is_ok()).then(|| {
        boost::predict(
            &cut.log,
            |_| stated_alone(stated),
            Noise::FromResiduals,
            &mut flown,
        )
        .map(|prediction| Implied::of(&prediction.fit))
        .map_err(|why| unpredicted_name(&why))
    });
    // The drag the coast says, by bisection on refitted boosts (ADR-346 §4): a calibration
    // flight's, without a deployment before apogee, that was predicted.
    let drag_solved = (!held_out && !forward.early && result.is_ok()).then(|| {
        collection_correction::solve_drag(
            |d| {
                let prediction = boost::predict(
                    &cut.log,
                    |_| collection_correction::stated_at_drag(stated, d),
                    Noise::FromResiduals,
                    &mut flown,
                )
                .map_err(|why| unpredicted_name(&why))?;
                if prediction.fit.converged {
                    Ok(prediction.apogee_above_pad_m)
                } else {
                    Err("a fit along the way didn't converge".to_owned())
                }
            },
            forward.logged_m,
        )
    });
    Ok(Outcome {
        flight_id: cut.flight_id.clone(),
        held_out,
        result,
        early: forward.early,
        cut_in_burn: cut.cut_in_burn,
        other_logs: cut.other_logs,
        other_logs_apart: cut.other_logs_apart,
        same_flight_m: Some(same_flight_m),
        implied,
        drag_solved,
    })
}

/// What a collection flight's fit is told (`hpr_forensics::synthetic::fits::stated`), its wind
/// centered on ERA5's averaged over the forward climb, and the pad's height above sea level.
fn stated_for(at: &str, forward: &ForwardFlight) -> Result<(Stated, f64), String> {
    let day = &forward.day;
    let pad_msl_m = day.site().height_m - day.geoid_undulation_m;
    let wind = crate::from_boost::mean_wind(day, pad_msl_m, forward.hpr_m)
        .map_err(|error| format!("{at}: {error}"))?;
    let mut stated: Stated = synthetic_fits::stated();
    stated[Asked::WindEast.index()].mean = wind.0;
    stated[Asked::WindNorth.index()].mean = wind.1;
    Ok((stated, pad_msl_m))
}

/// A cut flight the forward report never reached.
const NOT_REACHED: &str = "not in the forward report";

/// One fitted parameter over a half's fits.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    /// Its name.
    pub name: String,
    /// Its mean over the fits.
    pub mean: Option<f64>,
    /// Its sample standard deviation over them; `None` for fewer than 2.
    pub sd: Option<f64>,
}

/// A half's fits of the logs up to their cuts, as aggregates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FitAggregates {
    /// The half.
    pub half: String,
    /// The fits: one a predicted flight.
    pub fits: usize,
    /// Those whose last pass ended where no step lowered χ² further.
    pub converged: usize,
    /// The drag, impulse and structure mass scales.
    pub parameters: Vec<Spread>,
    /// The heights' residuals' RMS, m, averaged over the fits.
    pub rms_mean_m: Option<f64>,
}

/// What a half's logs alone say of the boost, as aggregates (ADR-346 §2, §3).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LogImplied {
    /// The half.
    pub half: String,
    /// How many times wider the impulse's stated range was made: [`WIDENED`].
    pub widened: f64,
    /// The flag's limit: [`RECERTIFICATION_LIMIT`].
    pub limit: f64,
    /// The standard errors the limit must be cleared by: [`FLAG_STANDARD_ERRORS`].
    pub standard_errors: f64,
    /// The widened fits that came to a prediction.
    pub fits: usize,
    /// The predicted flights whose widened fit came to none, by why.
    pub unfitted: BTreeMap<String, usize>,
    /// The fits that converged. Every aggregate below is over these alone, as ADR-346 §4 pools
    /// them.
    pub converged: usize,
    /// The converged fits' impulse scale.
    pub impulse_scale: Spread,
    /// That mean's standard error: the standard deviation over the root of the count.
    pub impulse_scale_mean_standard_error: Option<f64>,
    /// The mean of the impulse scale's standard errors in the converged fits.
    pub standard_error_mean: Option<f64>,
    /// The mean of the drag scale's standard errors in them.
    pub drag_standard_error_mean: Option<f64>,
    /// The converged fits whose impulse scale is below 1, the rated impulse.
    pub below_rated: usize,
    /// The flights flagged: the log and the files disagree by more than NAR's retest rule allows.
    pub flagged: usize,
}

impl LogImplied {
    /// The calibration half's. Its aggregates feed the choice of M7.5's correction, so they take
    /// a [`Calibration`], which no held-out flight passes (ADR-209 §4, ADR-332 §2).
    fn calibration(read: &Calibration<&Result<Implied, String>>) -> Self {
        let mut unfitted = BTreeMap::new();
        let mut fits = 0;
        let mut converged = Vec::new();
        for flight in read.flights() {
            match flight {
                Ok(fit) => {
                    fits += 1;
                    if fit.converged {
                        converged.push(fit);
                    }
                }
                Err(why) => *unfitted.entry(why.clone()).or_default() += 1,
            }
        }
        let scales: Vec<f64> = converged.iter().map(|f| f.scale).collect();
        let spread = sd(&scales);
        Self {
            half: CALIBRATION.to_owned(),
            widened: WIDENED,
            limit: RECERTIFICATION_LIMIT,
            standard_errors: FLAG_STANDARD_ERRORS,
            fits,
            unfitted,
            converged: converged.len(),
            impulse_scale: Spread {
                name: "impulse scale, the log alone".to_owned(),
                mean: mean(scales.iter().copied()),
                sd: spread,
            },
            impulse_scale_mean_standard_error: spread
                .zip(u32::try_from(scales.len()).ok())
                .map(|(sd, count)| sd / f64::from(count).sqrt()),
            standard_error_mean: mean(converged.iter().map(|f| f.standard_error)),
            drag_standard_error_mean: mean(converged.iter().map(|f| f.drag_standard_error)),
            below_rated: scales.iter().filter(|s| **s < 1.0).count(),
            flagged: converged.iter().filter(|f| f.disagrees()).count(),
        }
    }

    /// Its counts add up, against the half's predicted flights, and its numbers are numbers: one
    /// that isn't would be written as `null` and fail `--check` with no reason given.
    fn adds_up(&self, predicted: usize) -> bool {
        let finite = |value: Option<f64>| value.is_none_or(f64::is_finite);
        self.fits + self.unfitted.values().sum::<usize>() == predicted
            && self.converged <= self.fits
            && self.below_rated <= self.converged
            && self.flagged <= self.converged
            && finite(self.impulse_scale.mean)
            && finite(self.impulse_scale.sd)
            && finite(self.impulse_scale_mean_standard_error)
            && finite(self.standard_error_mean)
            && finite(self.drag_standard_error_mean)
    }
}

/// A half's predictions and errors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Half {
    /// [`CALIBRATION`] or [`HELD_OUT`].
    pub name: String,
    /// Its flights with a cut log.
    pub cut: usize,
    /// Of those, the ones with no prediction, by why: not compared by the forward report, or the
    /// prediction's reason.
    pub unpredicted: BTreeMap<String, usize>,
    /// The ones predicted.
    pub predicted: usize,
    /// Of those, the ones the manifest says deployed something before apogee.
    pub early_deployments: usize,
    /// Of those, the ones cut less than the motor's burn after liftoff.
    pub cut_in_burn: usize,
    /// Of those, the ones whose burn or liftoff isn't known.
    pub cut_burn_unknown: usize,
    /// Of those, the ones with another log on a barometer's track that gave an apogee, which the
    /// rule for choosing a log passed over.
    pub with_other_logs: usize,
    /// Of those, the ones whose logs' apogees lie more than 2% of the logged apogee apart.
    pub other_logs_apart: usize,
    /// The errors on every flight predicted.
    pub every: Errors,
    /// The errors on those without a deployment before apogee.
    pub without_early: Errors,
}

/// The two methods' errors on a set of predicted flights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    /// The flights.
    pub flights: usize,
    /// The prediction's mean absolute apogee error, %.
    pub prediction_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign, %.
    pub prediction_mean_percent: Option<f64>,
    /// The forward simulation's mean absolute apogee error on the same flights, %.
    pub forward_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign on them, %.
    pub forward_mean_percent: Option<f64>,
    /// The share of the forward error made up to the cut: the mean of each flight's forward
    /// error less its prediction's, with its sign, percentage points of the logged apogee
    /// (ADR-346 §1). The prediction's error is the share made after the cut.
    pub up_to_cut_mean_points: Option<f64>,
    /// The flights whose share up to the cut is the larger of the two in size.
    pub mostly_up_to_cut: usize,
    /// The mean of each flight's absolute error from the prediction less the forward
    /// simulation's, percentage points: below zero, the prediction came closer.
    pub difference_points: Option<f64>,
    /// That difference's paired bootstrap interval at [`LEVEL`].
    pub bootstrap_interval: Option<(f64, f64)>,
    /// Its exact sign-flip interval at [`LEVEL`], each end `None` where it is unbounded; `None`
    /// for no flights or more than 20.
    pub sign_flip_interval: Option<(Option<f64>, Option<f64>)>,
}

/// The predictions' part of the report.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Predictions {
    /// The bootstrap's resamples.
    pub resamples: u32,
    /// Its seed.
    pub seed: u64,
    /// The largest distance, m, of the fit's model flown with nothing changed in ERA5's wind
    /// from the forward apogee, over the flights flown: at most [`SAME_FLIGHT_WITHIN_M`].
    pub largest_same_flight_m: Option<f64>,
    /// The calibration half, then the held-out half.
    pub halves: Vec<Half>,
    /// The calibration half's fit aggregates.
    pub calibration_fits: FitAggregates,
    /// What the calibration half's logs alone say of the boost (M7.5a, ADR-346).
    pub calibration_log_implied: LogImplied,
    /// The held-out half's fit aggregates, published once the correction was chosen on the
    /// calibration half alone (M7.5b, ADR-322 §2); their private file still matches its
    /// committed hash.
    pub held_out_fits: Option<FitAggregates>,
    /// The correction of the shared bias, fitted and judged, as the page gives it (M7.5b,
    /// ADR-346).
    pub correction: Option<Judged>,
}

/// Whether a percentile bootstrap on `n` flights at `level` gives more than the flights' own
/// range: a resample of one flight drawn `n` times comes `(1/n)ⁿ` of the time for each, so where
/// that is at least `(1 − level)/2` both ends are the smallest and largest difference, an interval
/// that holds a symmetric center only `1 − 2·2⁻ⁿ` of the time (75% on 3) and is two flights' own
/// values. At 95% that is 3 flights or fewer.
pub(crate) fn resolves(n: usize, level: f64) -> bool {
    let Ok(count) = i32::try_from(n) else {
        return true;
    };
    n > 0 && (1.0 / f64::from(count)).powi(count) < (1.0 - level) / 2.0
}

/// The mean of `values`; `None` for none.
fn mean(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, count) = values.fold((0.0, 0_u32), |(s, c), v| (s + v, c + 1));
    (count > 0).then(|| sum / f64::from(count))
}

/// The sample standard deviation of `values`; `None` for fewer than 2.
fn sd(values: &[f64]) -> Option<f64> {
    let center = mean(values.iter().copied())?;
    let count = u32::try_from(values.len()).ok()?;
    (count > 1).then(|| {
        (values.iter().map(|v| (v - center).powi(2)).sum::<f64>() / f64::from(count - 1)).sqrt()
    })
}

impl FitAggregates {
    /// The calibration half's: a fit across flights, so it takes a [`Calibration`], which no
    /// held-out flight passes (ADR-209 §4).
    fn calibration(fits: &Calibration<&Fit>) -> Self {
        Self::of(CALIBRATION, fits.flights())
    }

    /// The held-out half's, sealed in [`HELD_OUT_FITS`] and published by no report until M7.5
    /// has chosen its correction on the calibration half alone (ADR-322 §2).
    fn held_out(fits: &[&Fit]) -> Self {
        Self::of(HELD_OUT, fits)
    }

    fn of(half: &str, fits: &[&Fit]) -> Self {
        let parameter = |name: &str, value: fn(&Parameters) -> f64| {
            let values: Vec<f64> = fits.iter().map(|f| value(&f.parameters)).collect();
            Spread {
                name: name.to_owned(),
                mean: mean(values.iter().copied()),
                sd: sd(&values),
            }
        };
        Self {
            half: half.to_owned(),
            fits: fits.len(),
            converged: fits.iter().filter(|f| f.converged).count(),
            parameters: vec![
                parameter("drag scale", |p| p.drag_scale),
                parameter("impulse scale", |p| p.impulse_scale),
                parameter("structure mass scale", |p| p.mass_scale),
            ],
            rms_mean_m: mean(fits.iter().filter_map(|f| f.rms_m)),
        }
    }
}

impl Half {
    fn of(name: &str, outcomes: &[&Outcome]) -> Self {
        let mut unpredicted = BTreeMap::new();
        let mut rows = Vec::new();
        for outcome in outcomes {
            match &outcome.result {
                Ok((prediction, forward, _)) => rows.push((outcome, *prediction, *forward)),
                Err(why) => *unpredicted.entry(why.clone()).or_default() += 1,
            }
        }
        Self {
            name: name.to_owned(),
            cut: outcomes.len(),
            unpredicted,
            predicted: rows.len(),
            early_deployments: rows.iter().filter(|(o, _, _)| o.early).count(),
            cut_in_burn: rows
                .iter()
                .filter(|(o, _, _)| o.cut_in_burn == Some(true))
                .count(),
            cut_burn_unknown: rows
                .iter()
                .filter(|(o, _, _)| o.cut_in_burn.is_none())
                .count(),
            with_other_logs: rows.iter().filter(|(o, _, _)| o.other_logs).count(),
            other_logs_apart: rows.iter().filter(|(o, _, _)| o.other_logs_apart).count(),
            every: Errors::of(rows.iter().map(|(_, p, f)| (*p, *f))),
            without_early: Errors::of(
                rows.iter()
                    .filter(|(o, _, _)| !o.early)
                    .map(|(_, p, f)| (*p, *f)),
            ),
        }
    }

    /// Its counts add up.
    fn adds_up(&self) -> bool {
        self.unpredicted.values().sum::<usize>() + self.predicted == self.cut
            && self.early_deployments <= self.predicted
            && self.cut_in_burn + self.cut_burn_unknown <= self.predicted
            && self.every.flights == self.predicted
            && self.every.mostly_up_to_cut <= self.every.flights
            && self.without_early.mostly_up_to_cut <= self.without_early.flights
            && self.without_early.flights + self.early_deployments == self.predicted
            && self.other_logs_apart <= self.with_other_logs
            && self.with_other_logs <= self.predicted
    }
}

impl Errors {
    /// The errors of `(prediction, forward)` pairs, %.
    fn of(pairs: impl Iterator<Item = (f64, f64)>) -> Self {
        let pairs: Vec<(f64, f64)> = pairs.collect();
        let differences: Vec<f64> = pairs.iter().map(|(p, f)| p.abs() - f.abs()).collect();
        let finite = |x: f64| x.is_finite().then_some(x);
        Self {
            flights: pairs.len(),
            prediction_mean_absolute_percent: mean(pairs.iter().map(|(p, _)| p.abs())),
            prediction_mean_percent: mean(pairs.iter().map(|(p, _)| *p)),
            forward_mean_absolute_percent: mean(pairs.iter().map(|(_, f)| f.abs())),
            forward_mean_percent: mean(pairs.iter().map(|(_, f)| *f)),
            up_to_cut_mean_points: mean(pairs.iter().map(|(p, f)| f - p)),
            mostly_up_to_cut: pairs
                .iter()
                .filter(|(p, f)| (f - p).abs() > p.abs())
                .count(),
            difference_points: mean(differences.iter().copied()),
            bootstrap_interval: resolves(differences.len(), LEVEL)
                .then(|| boost::paired_bootstrap_interval(&differences, LEVEL, RESAMPLES, SEED))
                .flatten(),
            sign_flip_interval: boost::sign_flip_interval(&differences, LEVEL)
                .map(|(low, high)| (finite(low), finite(high))),
        }
    }
}

/// Predicts every cut flight, in parallel, fits the correction of the shared bias and judges it
/// (M7.5b), and gathers the published part, the held-out half's fit aggregates and the
/// correction's test as computed.
pub(crate) fn predict_all(
    root: &Path,
    cuts: &[CutFlight],
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
) -> Result<(Predictions, FitAggregates, Judged, String), String> {
    let threads = std::thread::available_parallelism().map_or(4, usize::from);
    let chunk = cuts.len().div_ceil(threads).max(1);
    let outcomes: Vec<Outcome> = std::thread::scope(|scope| {
        let handles: Vec<_> = cuts
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|cut| predict(cut, forward.get(&cut.flight_id)))
                        .collect::<Result<Vec<_>, String>>()
                })
            })
            .collect();
        // Every thread is joined before the first error is returned, in the flights' order.
        let joined: Vec<_> = handles.into_iter().map(|handle| handle.join()).collect();
        let mut all = Vec::new();
        for part in joined {
            all.extend(part.map_err(|_| "a prediction's thread panicked".to_owned())??);
        }
        Ok::<_, String>(all)
    })?;
    let half =
        |held: bool| -> Vec<&Outcome> { outcomes.iter().filter(|o| o.held_out == held).collect() };
    let fits = |held: bool| {
        outcomes
            .iter()
            .filter(move |o| o.held_out == held)
            .filter_map(|o| Some((o.flight_id.as_str(), &o.result.as_ref().ok()?.2)))
    };
    // The calibration half's fits feed a fit across flights, so they pass the guard, which reads
    // each flight's half from its id again.
    let calibration = Calibration::of(fits(false))?;
    let implied = Calibration::of(
        outcomes
            .iter()
            .filter(|o| !o.held_out)
            .filter_map(|o| Some((o.flight_id.as_str(), o.implied.as_ref()?))),
    )?;
    let predictions = Predictions {
        resamples: RESAMPLES,
        seed: SEED,
        largest_same_flight_m: outcomes
            .iter()
            .filter_map(|o| o.same_flight_m.map(f64::abs))
            .reduce(f64::max),
        halves: vec![
            Half::of(CALIBRATION, &half(false)),
            Half::of(HELD_OUT, &half(true)),
        ],
        calibration_fits: FitAggregates::calibration(&calibration),
        calibration_log_implied: LogImplied::calibration(&implied),
        held_out_fits: None,
        correction: None,
    };
    let alone = |o: &Outcome| o.implied.as_ref().and_then(|i| i.as_ref().ok()).copied();
    let audit: Vec<serde_json::Value> = outcomes
        .iter()
        .map(|o| {
            let (prediction, forward, why) = match &o.result {
                Ok((p, f, _)) => (Some(*p), Some(*f), None),
                Err(why) => (None, None, Some(why.as_str())),
            };
            serde_json::json!({
                "flight_id": o.flight_id,
                "half": if o.held_out { HELD_OUT } else { CALIBRATION },
                "prediction_error_percent": prediction,
                "forward_error_percent": forward,
                "unpredicted": why,
                "early_deployment": o.early,
                "cut_in_burn": o.cut_in_burn,
                "other_logs": o.other_logs,
                "other_logs_apart": o.other_logs_apart,
                "same_flight_m": o.same_flight_m,
                "log_implied_impulse_scale": alone(o).map(|i| i.scale),
                "log_implied_standard_error": alone(o).map(|i| i.standard_error),
                "log_implied_converged": alone(o).map(|i| i.converged),
                "log_implied_unfitted": o.implied.as_ref().and_then(|i| i.as_ref().err()),
                "flagged": alone(o).map(|i| i.disagrees()),
            })
        })
        .collect();
    let audit = serde_json::to_string_pretty(&audit).map_err(|error| error.to_string())? + "\n";
    let held_out: Vec<&Fit> = fits(true).map(|(_, fit)| fit).collect();
    let held_out_fits = FitAggregates::held_out(&held_out);
    let judged = judge(root, cuts, forward, &outcomes)?;
    // The correction is chosen; the held-out half's sealed fit aggregates are published
    // (ADR-322 §2), the file still matching its committed hash.
    let predictions = Predictions {
        held_out_fits: Some(held_out_fits.clone()),
        correction: Some(judged.published()),
        ..predictions
    };
    Ok((predictions, held_out_fits, judged, audit))
}

/// Whether a flight's log alone disagrees with its files (ADR-346 §3).
fn flagged(outcome: &Outcome) -> bool {
    outcome
        .implied
        .as_ref()
        .and_then(|implied| implied.as_ref().ok())
        .is_some_and(Implied::disagrees)
}

/// A forward flight flown with `correction`, its apogee in the day's air, m, as the forward
/// report reads it: the center of mass's height above where it started.
fn corrected_apogee_m(flight: &ForwardFlight, correction: Correction) -> Result<f64, String> {
    let day = &flight.day;
    let flown = Flown {
        forward: flight,
        pad_msl_m: day.site().height_m - day.geoid_undulation_m,
        era5_wind: true,
    };
    let mut p = Parameters::from_array(synthetic_fits::stated().map(|p| p.mean));
    p.impulse_scale = correction.impulse_scale;
    p.drag_scale = correction.drag_scale;
    flown
        .flown(&p, boost::APOGEE_HORIZON_S, None)?
        .top_reading_m(HeightKind::Geometric)
        .ok_or_else(|| "the corrected flight reaches no apogee".to_owned())
}

/// A calibration flight's boost fitted as its log alone is, its drag centered on `k_d`
/// (ADR-346 §4).
fn refit_at_drag(cut: &CutFlight, flight: &ForwardFlight, k_d: f64) -> Result<Implied, String> {
    let (stated, pad_msl_m) = stated_for(&cut.flight_id, flight)?;
    let mut flown = Flown {
        forward: flight,
        pad_msl_m,
        era5_wind: false,
    };
    boost::predict(
        &cut.log,
        |_| collection_correction::stated_told_drag(stated, k_d),
        Noise::FromResiduals,
        &mut flown,
    )
    .map(|prediction| Implied::of(&prediction.fit))
    .map_err(|why| unpredicted_name(&why))
}

/// Fits the correction twice on the calibration half, through the guard, and judges each on the
/// six sets (ADR-346 §4, §5).
fn judge(
    root: &Path,
    cuts: &[CutFlight],
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
    outcomes: &[Outcome],
) -> Result<Judged, String> {
    let tau = synthetic_fits::stated()[Asked::ImpulseScale.index()].sd;
    // The fits across flights take the calibration flights predicted through the guard, which
    // reads each one's half from its id again (ADR-332 §2).
    let predicted = Calibration::of(
        outcomes
            .iter()
            .filter(|o| !o.held_out && o.result.is_ok())
            .map(|o| (o.flight_id.as_str(), o)),
    )?;
    let flown_forward = |id: &str| match forward.get(id) {
        Some(Ok(flight)) => Ok(flight),
        _ => Err(format!("{id}: predicted, but not flown forward")),
    };
    let mut fits = Vec::new();
    for name in [
        collection_correction::UNFLAGGED,
        collection_correction::EVERY,
    ] {
        let members: Vec<&Outcome> = predicted
            .flights()
            .iter()
            .copied()
            .filter(|o| name == collection_correction::EVERY || !flagged(o))
            .collect();
        let solved: Vec<Option<f64>> = members
            .iter()
            .filter(|o| !o.early)
            .map(|o| {
                o.drag_solved
                    .as_ref()
                    .and_then(|d| d.as_ref().ok())
                    .copied()
            })
            .collect();
        let drag = ScaleFit::drag(&solved);
        let implied: Vec<Result<Implied, String>> = if drag.entered() {
            let k_d = drag.flown.unwrap_or(1.0);
            let jobs: Vec<(&CutFlight, &ForwardFlight)> = members
                .iter()
                .map(|o| {
                    let cut = cuts
                        .iter()
                        .find(|c| c.flight_id == o.flight_id)
                        .ok_or_else(|| format!("{}: no cut", o.flight_id))?;
                    Ok((cut, flown_forward(&o.flight_id)?))
                })
                .collect::<Result<_, String>>()?;
            collection_correction::parallel(&jobs, |(cut, flight)| refit_at_drag(cut, flight, k_d))
        } else {
            members
                .iter()
                .map(|o| match &o.implied {
                    Some(implied) => implied.clone(),
                    None => Err("not fitted alone".to_owned()),
                })
                .collect()
        };
        let impulse = ScaleFit::impulse(
            &implied
                .iter()
                .map(|i| {
                    i.as_ref()
                        .ok()
                        .filter(|i| i.converged)
                        .map(|i| (i.scale, i.standard_error))
                })
                .collect::<Vec<_>>(),
            tau,
        );
        println!(
            "fixture-flights: the correction's fit {name}: {} flights",
            members.len()
        );
        fits.push((name, members.len(), drag, impulse));
    }
    let corrections: Vec<Correction> = fits
        .iter()
        .map(|(_, _, drag, impulse)| Correction {
            impulse_scale: impulse.flown.unwrap_or(1.0),
            drag_scale: drag.flown.unwrap_or(1.0),
        })
        .collect();

    // Every flight the forward report compares, flown with no correction (the check that it is
    // the forward report's flight) and with each fit's.
    let compared: Vec<(&String, &ForwardFlight)> = forward
        .iter()
        .filter_map(|(id, flight)| Some((id, flight.as_ref().ok()?)))
        .collect();
    // Each flight's distance with no correction from its forward apogee, m, and each fit's error, %.
    type FlownErrors = Result<(f64, Vec<Option<f64>>), String>;
    let flown: Vec<FlownErrors> = collection_correction::parallel(&compared, |(id, flight)| {
        let none = corrected_apogee_m(flight, Correction::NONE)
            .map_err(|error| format!("{id}: {error}"))?
            - flight.hpr_m;
        if !none.is_finite() || none.abs() > SAME_FLIGHT_WITHIN_M {
            return Err(format!(
                "{id}: flown with both scales at 1, it lands {none:+.3} m from the forward \
                     apogee"
            ));
        }
        let errors = corrections
            .iter()
            .map(|c| {
                corrected_apogee_m(flight, *c)
                    .ok()
                    .map(|a| 100.0 * (a - flight.logged_m) / flight.logged_m)
            })
            .collect();
        Ok((none.abs(), errors))
    });
    let flown = flown.into_iter().collect::<Result<Vec<_>, String>>()?;
    let largest_collection = flown.iter().map(|(none, _)| *none).reduce(f64::max);
    let flagged_ids: std::collections::BTreeSet<&str> = outcomes
        .iter()
        .filter(|o| flagged(o))
        .map(|o| o.flight_id.as_str())
        .collect();
    let public = collection_correction::public_flights(root, &corrections)?;

    let mut judged_fits = Vec::new();
    for (k, (name, members, drag, impulse)) in fits.into_iter().enumerate() {
        let collection = |held_out: bool, ballistic: bool, less_flagged: bool| -> Vec<Pair> {
            compared
                .iter()
                .zip(&flown)
                .filter(|((id, flight), _)| {
                    crate::collection_boost::held_out(id) == held_out
                        && (!ballistic || !flight.early)
                        && (!less_flagged || !flagged_ids.contains(id.as_str()))
                })
                .map(|((_, flight), (_, errors))| Pair {
                    forward: 100.0 * (flight.hpr_m - flight.logged_m) / flight.logged_m,
                    corrected: errors[k],
                })
                .collect()
        };
        let public_set = |less_flagged: bool| -> Vec<Pair> {
            public
                .flights
                .iter()
                .filter(|p| !less_flagged || !p.flagged)
                .map(|p| Pair {
                    forward: p.forward_percent,
                    corrected: p.corrected_percent[k],
                })
                .collect()
        };
        use collection_correction::{CALIBRATION_SET, HELD_OUT_BALLISTIC, HELD_OUT_SET, PUBLIC};
        let sets = vec![
            SetJudged::of(HELD_OUT_SET, false, &collection(true, false, false), true),
            SetJudged::of(
                HELD_OUT_BALLISTIC,
                false,
                &collection(true, true, false),
                true,
            ),
            SetJudged::of(PUBLIC, false, &public_set(false), false),
            SetJudged::of(HELD_OUT_SET, true, &collection(true, false, true), true),
            SetJudged::of(
                HELD_OUT_BALLISTIC,
                true,
                &collection(true, true, true),
                true,
            ),
            SetJudged::of(PUBLIC, true, &public_set(true), false),
        ];
        judged_fits.push(FitJudged {
            name: name.to_owned(),
            calibration_flights: members,
            drag,
            impulse,
            passes: sets.iter().all(|s| s.passes),
            sets,
            calibration: SetJudged::of(
                CALIBRATION_SET,
                false,
                &collection(false, false, false),
                true,
            ),
        });
    }
    let (passes, failed) = Judged::verdict(&judged_fits);
    let count = |held_out: bool, ballistic: bool| {
        compared
            .iter()
            .filter(|(id, flight)| {
                crate::collection_boost::held_out(id) == held_out
                    && (!ballistic || !flight.early)
                    && flagged_ids.contains(id.as_str())
            })
            .count()
    };
    let judged = Judged {
        resamples: RESAMPLES,
        seed: collection_correction::SEED,
        tau,
        flagged_calibration: predicted.flights().iter().filter(|o| flagged(o)).count(),
        flagged_held_out: count(true, false),
        flagged_held_out_ballistic: count(true, true),
        flagged_public: public.flights.iter().filter(|p| p.flagged).count(),
        held_out_log_fits: outcomes
            .iter()
            .filter(|o| o.held_out && matches!(o.implied, Some(Ok(_))))
            .count(),
        public_log_fits: public.flights.iter().filter(|p| p.fitted).count(),
        largest_uncorrected_collection_m: largest_collection,
        largest_uncorrected_public_m: public.largest_uncorrected_m,
        fits: judged_fits,
        passes,
        failed,
        held_back: false,
    };
    if !judged.adds_up() {
        return Err("the correction's test doesn't add up".to_owned());
    }
    Ok(judged)
}

/// Each cut flight's outcome, by id, for checking the page's aggregates on this machine (private,
/// gitignored, beside [`HELD_OUT_FITS`]).
pub const AUDIT: &str = "corpus-out/hpr-sim-fixtures/collection-predictions.json";

/// Writes [`AUDIT`].
pub(crate) fn write_audit(root: &Path, audit: &str) -> Result<(), String> {
    let path = root.join(AUDIT);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{AUDIT}: {error}"))?;
    }
    fs::write(path, audit).map_err(|error| format!("writing {AUDIT}: {error}"))
}

/// [`HELD_OUT_FITS`]'s text.
pub(crate) fn held_out_fits_text(fits: &FitAggregates) -> Result<String, String> {
    Ok(serde_json::to_string_pretty(fits).map_err(|error| error.to_string())? + "\n")
}

/// [`HELD_OUT_FITS_SHA`]'s text: the digest of `text`, then what it is.
pub(crate) fn held_out_fits_sha_text(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{hex}  SHA-256 of the private collection's held-out half's fit aggregates (ADR-322), \
         kept private until M7.5 publishes them\n"
    )
}

/// Writes, or with `check` checks, the held-out half's fit aggregates and their digest.
pub(crate) fn seal_held_out_fits(
    root: &Path,
    fits: &FitAggregates,
    check: bool,
) -> Result<(), String> {
    let text = held_out_fits_text(fits)?;
    let sealed = held_out_fits_sha_text(&text);
    if check {
        let committed = fs::read_to_string(root.join(HELD_OUT_FITS_SHA))
            .map_err(|error| format!("reading {HELD_OUT_FITS_SHA}: {error}"))?;
        if committed != sealed {
            return Err(format!(
                "{HELD_OUT_FITS_SHA} doesn't hold the SHA-256 of the held-out half's fit \
                 aggregates; run `cargo xtask fixture-flights`"
            ));
        }
        // The digest seals the recomputed aggregates; the private file, where this machine has
        // one, must be them too.
        match fs::read_to_string(root.join(HELD_OUT_FITS)) {
            Ok(private) if private != text => {
                return Err(format!(
                    "{HELD_OUT_FITS} isn't the recomputed aggregates; run `cargo xtask \
                     fixture-flights`"
                ));
            }
            Ok(_) => {}
            Err(_) => println!(
                "fixture-flights: {HELD_OUT_FITS} isn't on this machine; its digest holds the \
                 recomputed aggregates"
            ),
        }
        return Ok(());
    }
    let private = root.join(HELD_OUT_FITS);
    if let Some(parent) = private.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{HELD_OUT_FITS}: {error}"))?;
    }
    fs::write(&private, text).map_err(|error| format!("writing {HELD_OUT_FITS}: {error}"))?;
    fs::write(root.join(HELD_OUT_FITS_SHA), sealed)
        .map_err(|error| format!("writing {HELD_OUT_FITS_SHA}: {error}"))
}

/// A count with a comma between each group of three digits.
pub(crate) fn grouped(count: u32) -> String {
    let digits = count.to_string();
    let mut out = String::new();
    for (k, digit) in digits.chars().enumerate() {
        if k > 0 && (digits.len() - k).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// A percentage with its sign, or "none".
fn signed(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_owned(), |v| format!("{v:+.2}%"))
}

/// A percentage, or "none".
fn unsigned(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_owned(), |v| format!("{v:.2}%"))
}

/// An interval's end in points to two decimals, rounded outward by `outward` (`f64::floor` for
/// the low end, `f64::ceil` for the high), so the printed interval holds the computed one;
/// "unbounded" where it is.
fn end(value: Option<f64>, outward: fn(f64) -> f64) -> String {
    value.map_or_else(
        || "unbounded".to_owned(),
        |v| format!("{:+.2}", outward(v * 100.0) / 100.0),
    )
}

/// An interval, or "none".
fn interval(value: Option<(Option<f64>, Option<f64>)>) -> String {
    value.map_or_else(
        || "none".to_owned(),
        |(low, high)| match (low, high) {
            (None, None) => "unbounded".to_owned(),
            _ => format!("{} to {}", end(low, f64::floor), end(high, f64::ceil)),
        },
    )
}

/// A difference in percentage points, with its sign.
fn points(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_owned(), |d| format!("{d:+.2}"))
}

/// The row of the forward error's share made up to the cut.
const UP_TO_CUT: &str = "forward error made up to the cut, forward less prediction (points)";

impl LogImplied {
    /// The page's part: what the half's logs alone say of the boost, and the flag.
    fn write_markdown(&self, page: &mut String) {
        let number = |v: Option<f64>| v.map_or_else(|| "none".to_owned(), |v| format!("{v:.3}"));
        let stated = synthetic_fits::stated();
        let read = self.fits + self.unfitted.values().sum::<usize>();
        let _ = writeln!(
            page,
            "\n**The calibration half's logs alone.** Each fit above was told that the motor \
             delivers its rated impulse to a standard deviation of {:.3} (a 90% range of 1 give \
             or take {:.3}), and that pulls its impulse scale toward 1. For what a log alone \
             says, each of the calibration half's {read} predicted flights is fitted once more \
             with that range made {:.0} times wider and nothing else changed: {} fits, {} of \
             them converged. Such a fit's impulse scale is the impulse the climb to the cut \
             shows over the motor's rated impulse, at the mass in the files and the model's \
             drag. A barometer's boost can't tell less impulse from more mass \
             ([why](../../docs/research/drag-impulse-identifiability.md)), so a scale below 1 \
             can be a motor that delivered less, a rocket built heavier than its files, or \
             something the model gets wrong in the boost. The fit can't say which. The table \
             is over the converged fits: a few flights of one half of one collection, so the \
             mean says what these logs show, not what motors deliver.\n",
            stated[Asked::ImpulseScale.index()].sd,
            Asked::ImpulseScale.stated_width() / 2.0,
            self.widened,
            self.fits,
            self.converged
        );
        let _ = writeln!(page, "| | calibration half |\n|---|---:|");
        for (why, count) in &self.unfitted {
            let _ = writeln!(page, "| no fit of the log alone: {why} | {count} |");
        }
        let _ = writeln!(
            page,
            "| {}: mean | {} |",
            self.impulse_scale.name,
            number(self.impulse_scale.mean)
        );
        let _ = writeln!(
            page,
            "| … the mean's standard error | {} |",
            number(self.impulse_scale_mean_standard_error)
        );
        let _ = writeln!(
            page,
            "| … standard deviation | {} |",
            number(self.impulse_scale.sd)
        );
        let _ = writeln!(
            page,
            "| … its standard error in a fit, averaged | {} |",
            number(self.standard_error_mean)
        );
        let _ = writeln!(
            page,
            "| the drag scale's standard error in those fits, averaged | {} |",
            number(self.drag_standard_error_mean)
        );
        let _ = writeln!(
            page,
            "| fits with the scale below 1 | {} |",
            self.below_rated
        );
        let _ = writeln!(page, "| {FLAGGED} | {} |", self.flagged);
        let _ = writeln!(
            page,
            "\nThe fits were told the drag scale to a standard deviation of {:.3}, and they \
             come back knowing it little better. In a boost the drag and the impulse trade \
             against each other, so the climb to the cut says almost nothing of the drag by \
             itself: a drag scale has to come from the coast.",
            stated[Asked::DragScale.index()].sd
        );
        let _ = writeln!(
            page,
            "\nA flight is flagged when its log and its files disagree by more than NAR's \
             retest rule allows one firing: its impulse scale's distance from 1, less {:.0} of \
             its own standard errors, is more than {:.1}%, in a fit that converged. NAR, one \
             of the bodies that certify motors, tests a motor again as a new type when one \
             recertification firing's total impulse is more than {:.1}% from the certified \
             mean (NAR Motor Testing Manual v1.5, 2011, §6.2.3). The manual says that of a \
             test firing; this page takes it as the most a flown motor's lot strays, an \
             inference. Past it the lot is an unlikely cause, and the files' mass or motor, or \
             the model of the boost, a likelier one. The flag removes no flight from this \
             page's errors. [ADR-346, the rule before the fit]({ADR_346}), fixes how the \
             correction of the shared bias is to be fitted on these scales and judged with \
             and without the flagged flights, before any correction was fitted or flown. The \
             held-out half's logs are fitted this way too, below, for their flags alone.",
            self.standard_errors,
            self.limit * 100.0,
            self.limit * 100.0
        );
    }
}

/// The row of flagged flights.
const FLAGGED: &str = "flights flagged: the log and the files disagree";

/// This increment's decision record, from the report's folder.
const ADR_346: &str = "../../docs/decisions/0346-m7-5-in-three-the-rule-before-the-fit.md";

/// The row of flights whose candidate logs disagree, by [`crate::collection_boost::GAP_SHARE`].
const OTHER_LOGS_APART: &str = "… … the logs' apogees more than 2% apart";

/// A row of the halves' table: its label, and its value for a half.
type Row = (&'static str, fn(&Half) -> String);

/// A set of a half's flights: its name, and its errors in a half.
type SetRow = (&'static str, fn(&Half) -> &Errors);

/// A row of a set's errors: its label, and its value for the set.
type ErrorRow = (&'static str, fn(&Errors) -> String);

impl Predictions {
    /// Its counts add up, against the report's cut flights.
    pub(crate) fn adds_up(&self, cut: usize) -> bool {
        self.halves.len() == 2
            && self.halves[0].name == CALIBRATION
            && self.halves[1].name == HELD_OUT
            && self.halves.iter().all(Half::adds_up)
            && self.halves.iter().map(|h| h.cut).sum::<usize>() == cut
            && self.calibration_fits.fits == self.halves[0].predicted
            && self.calibration_fits.converged <= self.calibration_fits.fits
            && self
                .calibration_log_implied
                .adds_up(self.halves[0].predicted)
            && self
                .largest_same_flight_m
                .is_none_or(|m| m <= SAME_FLIGHT_WITHIN_M)
    }

    /// The page's part.
    pub(crate) fn write_markdown(&self, page: &mut String) {
        let [calibration, held] = self.halves.as_slice() else {
            let _ = writeln!(
                page,
                "\n**The predictions.** None: the halves weren't computed."
            );
            return;
        };
        let _ = writeln!(
            page,
            "\n**The predictions.** Each cut flight that the [forward report](fixture-flights.md) \
             compares is flown as it flies it in hpr: its `.ork` and motor configuration, its \
             rail and its day's ERA5 air, with nothing deployed. The fit changes only the motor's \
             impulse and burn time, the drag, the structure's mass and a wind constant with height, centered on \
             ERA5's wind averaged over the climb. Flown with nothing changed in ERA5's own wind, \
             the fit's model must land within {:.2} m of the forward simulation's apogee; the \
             largest miss was {}. The log \
             up to the cut is fitted and the fit flown on to apogee, as the \
             [public flights' report](from-boost.md) does. Both errors are against the same \
             number: the logged apogee as a height climbed in the day's air, as the forward \
             report takes it. Each set's last three rows give the difference and its intervals: each flight's absolute \
             error from the prediction less the forward simulation's, averaged, in percentage \
             points; below zero, the prediction came closer.\n",
            SAME_FLIGHT_WITHIN_M,
            self.largest_same_flight_m
                .map_or_else(|| "none".to_owned(), |m| format!("{m:.3} m"))
        );
        let _ = writeln!(page, "| | calibration half | held-out half |");
        let _ = writeln!(page, "|---|---:|---:|");
        let _ = writeln!(page, "| flights cut | {} | {} |", calibration.cut, held.cut);
        let mut reasons: Vec<&String> = calibration
            .unpredicted
            .keys()
            .chain(held.unpredicted.keys())
            .collect();
        reasons.sort();
        reasons.dedup();
        for why in reasons {
            let n = |h: &Half| h.unpredicted.get(why).copied().unwrap_or(0);
            let _ = writeln!(
                page,
                "| … no prediction: {why} | {} | {} |",
                n(calibration),
                n(held)
            );
        }
        let counts: [Row; 6] = [
            ("predicted", |h| h.predicted.to_string()),
            ("… with a deployment before apogee", |h| {
                h.early_deployments.to_string()
            }),
            ("… cut less than the motor's burn after liftoff", |h| {
                h.cut_in_burn.to_string()
            }),
            ("… cut, the burn or the liftoff not known", |h| {
                h.cut_burn_unknown.to_string()
            }),
            (
                "… with another log of the flight on a barometer's track",
                |h| h.with_other_logs.to_string(),
            ),
            (OTHER_LOGS_APART, |h| h.other_logs_apart.to_string()),
        ];
        for (label, value) in counts {
            let _ = writeln!(
                page,
                "| {label} | {} | {} |",
                value(calibration),
                value(held)
            );
        }
        let errors: [ErrorRow; 9] = [
            ("prediction from the boost: mean absolute error", |e| {
                unsigned(e.prediction_mean_absolute_percent)
            }),
            ("… mean error with its sign", |e| {
                signed(e.prediction_mean_percent)
            }),
            (
                "forward simulation, same flights: mean absolute error",
                |e| unsigned(e.forward_mean_absolute_percent),
            ),
            ("… mean error with its sign", |e| {
                signed(e.forward_mean_percent)
            }),
            (UP_TO_CUT, |e| points(e.up_to_cut_mean_points)),
            ("… flights where it is the larger share", |e| {
                e.mostly_up_to_cut.to_string()
            }),
            (
                "absolute errors' difference, prediction less forward (points)",
                |e| points(e.difference_points),
            ),
            ("… paired bootstrap 95% interval", |e| {
                match e.bootstrap_interval {
                    None if e.flights > 0 => "too few flights to resample".to_owned(),
                    bootstrap => interval(bootstrap.map(|(l, u)| (Some(l), Some(u)))),
                }
            }),
            ("… exact sign-flip 95% interval", |e| {
                interval(e.sign_flip_interval)
            }),
        ];
        let sets: [SetRow; 2] = [
            ("every flight predicted", |h| &h.every),
            ("without a deployment before apogee", |h| &h.without_early),
        ];
        for (set, of) in sets {
            let _ = writeln!(
                page,
                "| **{set}** | **{}** | **{}** |",
                of(calibration).flights,
                of(held).flights
            );
            for (label, value) in errors {
                let _ = writeln!(
                    page,
                    "| {label} | {} | {} |",
                    value(of(calibration)),
                    value(of(held))
                );
            }
        }
        let _ = writeln!(
            page,
            "\nBoth methods fly nothing \
             deployed, and the fit reads the log only up to its cut, so a deployment before apogee \
             lowers the logged apogee beneath both; the rows without those flights leave them \
             out, as the forward report's own set does. Where a flight has other logs on a \
             barometer's track, the rule above takes the one nearest the logged apogee; two rows \
             count how often there was such a choice, and how often the candidates' apogees lay \
             more than {:.0}% apart, where the choice could move an error. Interval ends are rounded \
             outward. The \
             bootstrap \
             resamples each half's flights {} times (seed {}), each flight's two errors kept \
             together, and takes the 2.5% and 97.5% points of the resampled means. On a few \
             flights it can cover less than 95%, so the exact sign-flip interval, which the public \
             flights' report uses, stands beside it: it assumes only that each flight's \
             difference is drawn independently and symmetrically about the center. On 3 flights \
             or fewer the bootstrap's 95% points are only the smallest and largest difference, \
             so the page gives none. On 5 flights or fewer no pattern of signs is rare enough at \
             95%, so the sign-flip interval is the whole line: \"unbounded\".\n",
            crate::collection_boost::GAP_SHARE * 100.0,
            grouped(self.resamples),
            self.seed
        );
        let _ = writeln!(
            page,
            "**Up to the cut and after it.** The cut is where the fitted part of a log ends, \
             after burnout. One row of each set gives the share of the forward error made up to \
             the cut, in percentage points of the logged apogee; above zero, the forward \
             simulation had climbed too high by then. The row under it counts the flights \
             whose share up to the cut is the larger of the two in size. The prediction \
             follows the log up to the cut and the model from there, so its error, in the \
             rows above, is the share made after the cut, in the coast. The forward error less \
             the prediction's is the share made up to the cut, where the motor's thrust, the \
             rocket's mass and the boost's own drag set the climb: taking the boost from the \
             log removes it. A flight's two shares add up to its forward error. The fit also \
             moves the drag, the mass and the wind within what it was told of them, and those \
             carry into the coast, so the split is of these two simulations' errors, not a \
             measurement of the motor or the drag alone.\n"
        );
        let fits = &self.calibration_fits;
        let _ = writeln!(
            page,
            "**The calibration half's fits.** {} fits of the logs up to the cut, {} of them \
             converged; their residuals' RMS averages {}. Over those fits:\n",
            fits.fits,
            fits.converged,
            fits.rms_mean_m
                .map_or_else(|| "none".to_owned(), |m| format!("{m:.2} m"))
        );
        let _ = writeln!(page, "| | mean | standard deviation |\n|---|---:|---:|");
        for spread in &fits.parameters {
            let number =
                |v: Option<f64>| v.map_or_else(|| "none".to_owned(), |v| format!("{v:.3}"));
            let _ = writeln!(
                page,
                "| {} | {} | {} |",
                spread.name,
                number(spread.mean),
                number(spread.sd)
            );
        }
        match &self.held_out_fits {
            Some(held_out) => {
                let _ = writeln!(
                    page,
                    "\n**The held-out half's fits,** kept private until the correction below was \
                     chosen on the calibration half alone, are published with it: {} fits, {} of \
                     them converged; their residuals' RMS averages {}. Their file still matches \
                     the SHA-256 committed in [`held-out-fits.sha256`](held-out-fits.sha256) \
                     before the correction was fitted.\n",
                    held_out.fits,
                    held_out.converged,
                    held_out
                        .rms_mean_m
                        .map_or_else(|| "none".to_owned(), |m| format!("{m:.2} m"))
                );
                let _ = writeln!(page, "| | mean | standard deviation |\n|---|---:|---:|");
                for spread in &held_out.parameters {
                    let number =
                        |v: Option<f64>| v.map_or_else(|| "none".to_owned(), |v| format!("{v:.3}"));
                    let _ = writeln!(
                        page,
                        "| {} | {} | {} |",
                        spread.name,
                        number(spread.mean),
                        number(spread.sd)
                    );
                }
            }
            None => {
                let _ = writeln!(
                    page,
                    "\nThe held-out half's fit aggregates stay private until the correction of \
                     the shared bias is chosen on the calibration half alone; the SHA-256 of \
                     their file is committed in [`held-out-fits.sha256`](held-out-fits.sha256), \
                     so a change to them shows."
                );
            }
        }
        self.calibration_log_implied.write_markdown(page);
        if let Some(correction) = &self.correction {
            correction.write_markdown(page, ADR_346);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fit() -> Fit {
        Fit {
            parameters: Parameters::from_array(synthetic_fits::stated().map(|p| p.mean)),
            standard_errors: [0.0; 6],
            clock_offset_s: 0.0,
            height_offset_m: 0.0,
            rms_m: Some(1.0),
            height_noise_m: 1.0,
            horizontal_noise_m: 1.0,
            fitted: true,
            converged: true,
            misfit_share: None,
            read: Vec::new(),
            flown: 1,
        }
    }

    fn outcome(held_out: bool, result: Result<(f64, f64), &str>, early: bool) -> Outcome {
        Outcome {
            flight_id: "example-flight".to_owned(),
            held_out,
            result: result.map(|(p, f)| (p, f, fit())).map_err(str::to_owned),
            early,
            cut_in_burn: Some(false),
            other_logs: early,
            other_logs_apart: false,
            same_flight_m: Some(0.1),
            implied: None,
            drag_solved: None,
        }
    }

    fn implied(scale: f64, standard_error: f64) -> Implied {
        Implied {
            scale,
            standard_error,
            drag_standard_error: 0.1,
            converged: true,
        }
    }

    /// A half's means are over its predicted flights only, the difference is of absolute
    /// errors, and an unpredicted flight is counted under its reason.
    #[test]
    fn a_half_takes_its_predicted_flights() {
        let outcomes = [
            outcome(false, Ok((-2.0, 6.0)), false),
            outcome(false, Ok((4.0, -2.0)), true),
            outcome(false, Err(NOT_REACHED), false),
        ];
        let refs: Vec<&Outcome> = outcomes.iter().collect();
        let half = Half::of(CALIBRATION, &refs);
        assert_eq!(half.cut, 3);
        assert_eq!(half.predicted, 2);
        assert_eq!(half.unpredicted.get(NOT_REACHED), Some(&1));
        assert_eq!(half.early_deployments, 1);
        assert_eq!((half.with_other_logs, half.other_logs_apart), (1, 0));
        let every = &half.every;
        assert_eq!(every.flights, 2);
        assert_eq!(every.prediction_mean_absolute_percent, Some(3.0));
        assert_eq!(every.prediction_mean_percent, Some(1.0));
        assert_eq!(every.forward_mean_absolute_percent, Some(4.0));
        assert_eq!(every.forward_mean_percent, Some(2.0));
        // |−2| − |6| = −4 and |4| − |−2| = 2.
        assert_eq!(every.difference_points, Some(-1.0));
        // Two flights are too few to resample: the interval would be their range.
        assert_eq!(every.bootstrap_interval, None);
        // Without the early deployment: the first flight alone.
        let without = &half.without_early;
        assert_eq!(without.flights, 1);
        assert_eq!(without.prediction_mean_percent, Some(-2.0));
        assert_eq!(without.forward_mean_percent, Some(6.0));
        assert_eq!(without.difference_points, Some(-4.0));
        // Up to the cut: 6 − (−2) = 8 and −2 − 4 = −6, each larger in size than what is left.
        assert_eq!(every.up_to_cut_mean_points, Some(1.0));
        assert_eq!(every.mostly_up_to_cut, 2);
        assert_eq!(without.up_to_cut_mean_points, Some(8.0));
        assert_eq!(without.mostly_up_to_cut, 1);
        assert!(half.adds_up());
    }

    /// A flight's forward error is its share up to the cut plus its share after it, the
    /// prediction's error; the larger share is the larger in size, and a tie is not the cut's.
    #[test]
    fn the_forward_error_splits_at_the_cut() {
        // (prediction, forward): up to the cut 9, 1, 0 and −4.
        let pairs = [(1.0, 10.0), (-5.0, -4.0), (3.0, 3.0), (2.0, -2.0)];
        let errors = Errors::of(pairs.into_iter());
        assert_eq!(errors.up_to_cut_mean_points, Some(1.5));
        // 9 against 1 and −4 against 2; not 1 against −5, nor 0 against 3.
        assert_eq!(errors.mostly_up_to_cut, 2);
        let sum = errors.up_to_cut_mean_points.unwrap() + errors.prediction_mean_percent.unwrap();
        assert!((sum - errors.forward_mean_percent.unwrap()).abs() < 1e-12);
        // Equal shares: 2 up to the cut, 2 after it.
        assert_eq!(Errors::of([(2.0, 4.0)].into_iter()).mostly_up_to_cut, 0);
        let none = Errors::of(std::iter::empty());
        assert_eq!(
            (none.up_to_cut_mean_points, none.mostly_up_to_cut),
            (None, 0)
        );
    }

    /// A flight is flagged only when its scale clears NAR's 13.1% by two of its standard
    /// errors, on either side of the rated impulse, in a fit that converged and came to numbers.
    #[test]
    fn a_flag_needs_the_limit_cleared_by_two_standard_errors() {
        // 0.185 from 1, less 0.04: past 0.131.
        assert!(implied(0.815, 0.02).disagrees());
        // Exactly known: just past the limit, and just inside it.
        assert!(implied(0.868, 0.0).disagrees());
        assert!(!implied(0.870, 0.0).disagrees());
        // 0.15 from 1: two standard errors of 0.009 leave 0.132, of 0.010 leave 0.130.
        assert!(implied(0.85, 0.009).disagrees());
        assert!(!implied(0.85, 0.010).disagrees());
        // One standard error would have flagged this: 0.15 − 0.015 is past the limit.
        assert!(!implied(0.85, 0.015).disagrees());
        // Above the rating too.
        assert!(implied(1.2, 0.02).disagrees());
        assert!(!implied(1.14, 0.01).disagrees());
        // The calibration half's mean, far inside.
        assert!(!implied(0.94, 0.03).disagrees());
        let unconverged = Implied {
            converged: false,
            ..implied(0.5, 0.0)
        };
        assert!(!unconverged.disagrees());
        for (scale, standard_error) in [
            (f64::NAN, 0.0),
            (0.5, f64::NAN),
            (0.5, f64::INFINITY),
            (f64::INFINITY, 0.0),
            (0.5, -0.01),
        ] {
            assert!(!implied(scale, standard_error).disagrees());
        }
    }

    /// The limit is NAR's and the widening is what the page says: the words carry the numbers.
    /// The aggregates are over the converged fits alone, a scale of exactly 1 isn't below the
    /// rating, and a widened fit that came to nothing is counted under its reason.
    #[test]
    fn the_page_names_the_flags_numbers() {
        assert!((RECERTIFICATION_LIMIT - 0.131).abs() < 1e-15);
        let unconverged = Implied {
            converged: false,
            ..implied(0.5, 0.0)
        };
        let flights = [
            Ok(implied(0.8, 0.01)),
            Ok(implied(0.95, 0.03)),
            Ok(implied(1.0, 0.05)),
            Ok(unconverged),
            Err("too little up to the cut to fit".to_owned()),
        ];
        // Invented ids of the calibration half (`collection_boost`'s tests: 6 to 10 are held out).
        let ids = (1..=5).map(|k| format!("example-flight-{k}"));
        let ids: Vec<String> = ids.collect();
        let guarded = Calibration::of(ids.iter().map(String::as_str).zip(flights.iter())).unwrap();
        let alone = LogImplied::calibration(&guarded);
        assert_eq!((alone.fits, alone.converged), (4, 3));
        assert_eq!(
            alone.unfitted.get("too little up to the cut to fit"),
            Some(&1)
        );
        // Over 0.8, 0.95 and 1.0: the unconverged 0.5 is in none of these.
        assert_eq!((alone.below_rated, alone.flagged), (2, 1));
        assert!((alone.impulse_scale.mean.unwrap() - 2.75 / 3.0).abs() < 1e-12);
        // The deviations are −7/60, 2/60 and 5/60: their squares sum to 78/3600, over 2.
        let sd = (78.0_f64 / 7200.0).sqrt();
        assert!((alone.impulse_scale.sd.unwrap() - sd).abs() < 1e-12);
        let of_mean = alone.impulse_scale_mean_standard_error.unwrap();
        assert!((of_mean - sd / 3.0_f64.sqrt()).abs() < 1e-12);
        assert!((alone.standard_error_mean.unwrap() - 0.03).abs() < 1e-12);
        assert!((alone.drag_standard_error_mean.unwrap() - 0.1).abs() < 1e-12);
        assert!(alone.adds_up(5) && !alone.adds_up(4));
        let not_a_number = LogImplied {
            standard_error_mean: Some(f64::NAN),
            ..alone.clone()
        };
        assert!(!not_a_number.adds_up(5));
        let mut page = String::new();
        alone.write_markdown(&mut page);
        for words in [
            "made 100 times wider",
            "distance from 1, less 2 of its own standard errors, is more than 13.1%",
            "something the model gets wrong in the boost",
            "fitted this way too, below, for their flags alone",
            "§6.2.3",
            "to a standard deviation of 0.033 (a 90% range of 1 give or take 0.054)",
            "each of the calibration half's 5 predicted flights",
            "4 fits, 3 of them converged",
            "| no fit of the log alone: too little up to the cut to fit | 1 |",
            "drag scale to a standard deviation of 0.109",
            "| the drag scale's standard error in those fits, averaged | 0.100 |",
            "| flights flagged: the log and the files disagree | 1 |",
            "| fits with the scale below 1 | 2 |",
        ] {
            assert!(page.contains(words), "{words}: {page}");
        }
    }

    /// The fit of the log alone is told what the prediction's fit was, but for the impulse
    /// scale's range, 100 times wider: nothing else is widened or moved.
    #[test]
    fn only_the_impulse_range_is_widened() {
        let stated = synthetic_fits::stated();
        let alone = stated_alone(stated);
        let impulse = Asked::ImpulseScale.index();
        for (k, (told, was)) in alone.iter().zip(stated.iter()).enumerate() {
            assert_eq!(told.mean.to_bits(), was.mean.to_bits(), "mean {k}");
            if k == impulse {
                assert!((told.sd - 100.0 * was.sd).abs() < 1e-12, "{}", told.sd);
            } else {
                assert_eq!(told.sd.to_bits(), was.sd.to_bits(), "sd {k}");
            }
        }
    }

    /// A held-out flight's log is fitted alone for its flag (M7.5b), but its fit never passes
    /// the guard into a fit across flights.
    #[test]
    fn a_held_out_log_never_reaches_the_logs_alone() {
        let flight = implied(0.9, 0.02);
        assert!(crate::collection_boost::held_out("example-flight-6"));
        let refused =
            Calibration::of([("example-flight-1", &flight), ("example-flight-6", &flight)]);
        assert!(refused.is_err());
        // Every predicted flight's log is fitted alone; an unpredicted one's isn't.
        assert!(reads_log_alone(true));
        assert!(!reads_log_alone(false));
    }

    /// A percentile bootstrap gives more than the flights' range from 4 flights on at 95%: on 3,
    /// a resample of one flight three times comes 1/27 of the time, above 2.5%.
    #[test]
    fn the_bootstrap_needs_four_flights_at_95_percent() {
        assert!(!resolves(0, 0.95));
        assert!(!resolves(1, 0.95));
        assert!(!resolves(3, 0.95));
        assert!(resolves(4, 0.95));
        assert!(resolves(12, 0.95));
        let three = Errors::of([(1.0, 5.0), (2.0, 6.0), (3.0, 4.0)].into_iter());
        assert_eq!(three.bootstrap_interval, None);
        assert!(three.sign_flip_interval.is_some());
        let four = Errors::of([(1.0, 5.0), (2.0, 6.0), (3.0, 4.0), (1.5, 2.0)].into_iter());
        assert!(four.bootstrap_interval.is_some());
    }

    /// The disagreeing logs' row names the share the count uses.
    #[test]
    fn the_logs_row_names_the_gap_it_counts() {
        let share = format!("{:.0}%", crate::collection_boost::GAP_SHARE * 100.0);
        assert!(OTHER_LOGS_APART.contains(&share), "{OTHER_LOGS_APART}");
    }

    /// Interval ends round outward, each to two decimals, so the printed interval holds the
    /// computed one; an end exact in binary (−4.0, 2.5) stays.
    #[test]
    fn interval_ends_round_outward() {
        assert_eq!(
            interval(Some((Some(-2.709_4), Some(-2.285_2)))),
            "-2.71 to -2.28"
        );
        assert_eq!(
            interval(Some((Some(-19.004_8), Some(-7.972_5)))),
            "-19.01 to -7.97"
        );
        assert_eq!(interval(Some((Some(-4.0), Some(2.5)))), "-4.00 to +2.50");
        assert_eq!(interval(Some((None, Some(1.0)))), "unbounded to +1.00");
        assert_eq!(interval(Some((None, None))), "unbounded");
        assert_eq!(interval(None), "none");
    }

    /// Groups of three digits, from the right.
    #[test]
    fn counts_are_grouped_by_thousands() {
        assert_eq!(grouped(10_000), "10,000");
        assert_eq!(grouped(1_234_567), "1,234,567");
        assert_eq!(grouped(999), "999");
        assert_eq!(grouped(0), "0");
    }

    /// The sample standard deviation divides by n − 1, and needs two values.
    #[test]
    fn the_spread_is_the_sample_standard_deviation() {
        assert_eq!(sd(&[1.0, 3.0]), Some(2.0_f64.sqrt()));
        assert_eq!(sd(&[1.0]), None);
        assert_eq!(sd(&[]), None);
    }

    /// The digest is 64 hex digits, and a change to the aggregates changes it.
    #[test]
    fn the_held_out_fits_are_sealed_by_their_text() {
        let fits = FitAggregates::held_out(&[]);
        let text = held_out_fits_text(&fits).unwrap();
        let sealed = held_out_fits_sha_text(&text);
        let digest = sealed.split("  ").next().unwrap();
        assert!(digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()));
        let mut other = fits.clone();
        other.converged = 1;
        assert_ne!(
            held_out_fits_sha_text(&held_out_fits_text(&other).unwrap()),
            sealed
        );
    }

    /// The committed digest is one line of the form written, here and in CI.
    #[test]
    fn the_committed_digest_has_its_form() {
        let root = crate::designs::root().unwrap();
        let sealed = fs::read_to_string(root.join(HELD_OUT_FITS_SHA)).unwrap();
        let (digest, rest) = sealed.split_once("  ").unwrap();
        assert!(digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()));
        assert!(rest.ends_with("publishes them\n") && !rest[..rest.len() - 1].contains('\n'));
    }
}
