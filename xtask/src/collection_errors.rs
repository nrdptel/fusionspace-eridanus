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
//! sign-flip interval beside it are published. The calibration half's fit aggregates are; the
//! held-out half's are written to the gitignored [`HELD_OUT_FITS`], and only that file's SHA-256 is
//! committed, in [`HELD_OUT_FITS_SHA`], until M7.5 publishes them (ADR-322 §2).

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
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

/// The fit's flights of one collection flight.
struct Flown<'a> {
    forward: &'a ForwardFlight,
    /// The pad's height above sea level, m: where a sheared wind's ground is.
    pad_msl_m: f64,
    /// Flies ERA5's own wind, whatever the parameters say of it.
    era5_wind: bool,
}

impl Flown<'_> {
    fn flown(&self, p: &Parameters, until_s: f64, shear: Option<f64>) -> Result<Simulated, String> {
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

/// One flight's outcome, kept on this machine.
struct Outcome {
    held_out: bool,
    /// The prediction's error and the forward simulation's against the same logged apogee, %,
    /// and the fit; or why there is none.
    result: Result<(f64, f64, Fit), String>,
    early: bool,
    cut_in_burn: Option<bool>,
    other_logs: bool,
    other_logs_apart: bool,
    same_flight_m: Option<f64>,
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
                held_out,
                result: Err((*why).to_owned()),
                early: false,
                cut_in_burn: cut.cut_in_burn,
                other_logs: cut.other_logs,
                other_logs_apart: cut.other_logs_apart,
                same_flight_m: None,
            });
        }
    };
    let at = &cut.flight_id;
    let day = &forward.day;
    let pad_msl_m = day.site().height_m - day.geoid_undulation_m;
    let wind = crate::from_boost::mean_wind(day, pad_msl_m, forward.hpr_m)
        .map_err(|error| format!("{at}: {error}"))?;
    let mut stated: Stated = synthetic_fits::stated();
    stated[Asked::WindEast.index()].mean = wind.0;
    stated[Asked::WindNorth.index()].mean = wind.1;
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
    Ok(Outcome {
        held_out,
        result,
        early: forward.early,
        cut_in_burn: cut.cut_in_burn,
        other_logs: cut.other_logs,
        other_logs_apart: cut.other_logs_apart,
        same_flight_m: Some(same_flight_m),
    })
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
            difference_points: mean(differences.iter().copied()),
            bootstrap_interval: boost::paired_bootstrap_interval(
                &differences,
                LEVEL,
                RESAMPLES,
                SEED,
            ),
            sign_flip_interval: boost::sign_flip_interval(&differences, LEVEL)
                .map(|(low, high)| (finite(low), finite(high))),
        }
    }
}

/// Predicts every cut flight, in parallel, and gathers the published part and the held-out
/// half's fit aggregates.
pub(crate) fn predict_all(
    cuts: &[CutFlight],
    forward: &BTreeMap<String, Result<ForwardFlight, &'static str>>,
) -> Result<(Predictions, FitAggregates), String> {
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
        let mut all = Vec::new();
        for handle in handles {
            all.extend(
                handle
                    .join()
                    .map_err(|_| "a prediction's thread panicked".to_owned())??,
            );
        }
        Ok::<_, String>(all)
    })?;
    let half =
        |held: bool| -> Vec<&Outcome> { outcomes.iter().filter(|o| o.held_out == held).collect() };
    let fits = |held: bool| -> Vec<&Fit> {
        outcomes
            .iter()
            .filter(|o| o.held_out == held)
            .filter_map(|o| o.result.as_ref().ok().map(|(_, _, fit)| fit))
            .collect()
    };
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
        calibration_fits: FitAggregates::of(CALIBRATION, &fits(false)),
    };
    Ok((predictions, FitAggregates::of(HELD_OUT, &fits(true))))
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
fn grouped(count: u32) -> String {
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
                .largest_same_flight_m
                .is_none_or(|m| m <= SAME_FLIGHT_WITHIN_M)
    }

    /// The page's part.
    pub(crate) fn write_markdown(&self, page: &mut String) {
        let [calibration, held] = [&self.halves[0], &self.halves[1]];
        let _ = writeln!(
            page,
            "\n**The predictions.** Each cut flight that the [forward report](fixture-flights.md) \
             compares is flown as it flies it in hpr: its `.ork` and motor configuration, its \
             rail and its day's ERA5 air, with nothing deployed. The fit changes only the motor's \
             impulse and burn time, the drag, the structure's mass and a wind constant with height, centered on \
             ERA5's wind averaged over the climb. Flown with nothing changed in ERA5's own wind, \
             each lands within {:.2} m of the forward simulation's apogee (largest {}). The log \
             up to the cut is fitted and the fit flown on to apogee, as the \
             [public flights' report](from-boost.md) does. Both errors are against the same \
             number: the logged apogee as a height climbed in the day's air, as the forward \
             report takes it.\n",
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
            ("… with another barometer's log of the flight", |h| {
                h.with_other_logs.to_string()
            }),
            ("… … the logs' apogees more than 2% apart", |h| {
                h.other_logs_apart.to_string()
            }),
        ];
        for (label, value) in counts {
            let _ = writeln!(
                page,
                "| {label} | {} | {} |",
                value(calibration),
                value(held)
            );
        }
        let errors: [ErrorRow; 7] = [
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
            (
                "absolute errors' difference, prediction less forward (points)",
                |e| {
                    e.difference_points
                        .map_or_else(|| "none".to_owned(), |d| format!("{d:+.2}"))
                },
            ),
            ("… paired bootstrap 95% interval", |e| {
                interval(e.bootstrap_interval.map(|(l, u)| (Some(l), Some(u))))
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
            "\nA difference below zero means the prediction came closer. Both methods fly nothing \
             deployed, and the fit reads the log only up to its cut, so a deployment before apogee \
             lowers the logged apogee beneath both; the rows without those flights leave them \
             out, as the forward report's own set does. Where a flight has other logs on a \
             barometer's track, the rule above takes the one nearest the logged apogee; two rows \
             count how often there was such a choice, and how often the candidates' apogees lay \
             more than 2% apart, where the choice could move an error. Interval ends are rounded \
             outward. The \
             bootstrap \
             resamples each half's flights {} times (seed {}), each flight's two errors kept \
             together, and takes the 2.5% and 97.5% points of the resampled means. On a few \
             flights it can cover less than 95%, so the exact sign-flip interval, which the public \
             flights' report uses, stands beside it: it assumes only that each flight's \
             difference is drawn independently and symmetrically about the center.\n",
            grouped(self.resamples),
            self.seed
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
        let _ = writeln!(
            page,
            "\nThe held-out half's fit aggregates, the drag and impulse scales a later correction \
             of the shared bias would aim at, stay private until that correction is chosen on \
             the calibration half alone; the SHA-256 of their file is committed in \
             [`held-out-fits.sha256`](held-out-fits.sha256), so a change to them shows."
        );
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
            held_out,
            result: result.map(|(p, f)| (p, f, fit())).map_err(str::to_owned),
            early,
            cut_in_burn: Some(false),
            other_logs: early,
            other_logs_apart: false,
            same_flight_m: Some(0.1),
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
        assert_eq!(every.bootstrap_interval, Some((-4.0, 2.0)));
        // Without the early deployment: the first flight alone.
        let without = &half.without_early;
        assert_eq!(without.flights, 1);
        assert_eq!(without.prediction_mean_percent, Some(-2.0));
        assert_eq!(without.forward_mean_percent, Some(6.0));
        assert_eq!(without.difference_points, Some(-4.0));
        assert!(half.adds_up());
    }

    /// Interval ends round outward, each to two decimals, so the printed interval holds the
    /// computed one; an end that is already on two decimals stays.
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

    /// The digest's file names what it seals, and a change to the aggregates changes it.
    #[test]
    fn the_held_out_fits_are_sealed_by_their_text() {
        let fits = FitAggregates::of(HELD_OUT, &[]);
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
