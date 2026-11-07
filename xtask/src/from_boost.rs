//! `cargo xtask real-flights` also predicts each public flight's apogee from its boost (M7.3d2,
//! ADR-257 §3, ADR-321): `validation/reports/from-boost.{json,md}`.
//!
//! Each flight's log is fitted whole by `hpr_forensics::fit`, and cut just after burnout by
//! `hpr_forensics::boost`, which fits the log up to the cut and flies the fit on to apogee. The
//! fit flies the flight as `real-flights` does: from the example's thrust file, rail and site, in
//! its ERA5 air, with its motor, drag, mass and wind changed as the fit asks. Its wind is constant
//! with height, as the fit's model is. What's stated of that wind is ERA5's, averaged evenly in
//! height over the forward simulation's climb. The prediction's apogee is read as the forward
//! simulation's is, from the ERA5 pressure at its center of mass, and set against the same log
//! apogee. The logs are under the gitignored `refs/`, so CI holds only the page to the committed
//! data, and the forward simulation's numbers to `real-flights`' committed report (below).

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr::hpr_analysis::montecarlo::dispersed_motor;
use hpr_atmos::{ConstantWind, PowerLawWind};
use hpr_design::Rocket;
use hpr_flightdata::readings::{self, Reading, Reason};
use hpr_forensics::boost::{self, Cut, NoCut, Unpredicted};
use hpr_forensics::fit::{self, Fit, Forward, Noise, Observations, Parameters, Stated};
use hpr_forensics::residuals::{HeightKind, Simulated, Track};
use hpr_forensics::synthetic::fits::{self as synthetic_fits, Asked, shares};
use hpr_forensics::synthetic::truth::SHEAR_REFERENCE_HEIGHT_M;
use hpr_sim::{Environment, FlightSettings, Simulation};
use hpr_validate::real_flight::{
    Altimeter, FLIGHTS, FileRead, FlightRow, ROCKETPY, RealFlight, RealFlightReport,
    example_thrust, launch_day, parse_thrust, rail_of, read_bytes, rocket_on_curve, sha256_hex,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The report's data.
pub const REPORT_JSON: &str = "validation/reports/from-boost.json";
/// The report's page.
pub const REPORT_MD: &str = "validation/reports/from-boost.md";
/// The sign-flip interval's coverage: at least this, as ADR-257 §3 gives the collection's
/// bootstrap.
pub const LEVEL: f64 = 0.95;
/// How far the fit's model, flown in ERA5's own wind with nothing changed, may land from the
/// forward simulation's apogee, m: the same flight, read the same way, but for the grid
/// `real-flights` reads its apogee on.
const SAME_FLIGHT_WITHIN_M: f64 = 0.5;
/// The heights ERA5's wind is averaged over, evenly from the pad to the forward simulation's
/// apogee.
const WIND_LEVELS: u32 = 100;

/// The fit's flights of one public flight: the example's rocket, rail and ERA5 air, with the
/// fit's scales and its wind.
#[derive(Debug)]
struct Flown<'a> {
    rocket: &'a Rocket,
    environment: &'a Environment,
    flight: &'a RealFlight,
    /// Flies ERA5's own wind, whatever the parameters say of it: the check that this is the
    /// forward simulation's flight.
    era5_wind: bool,
}

impl Flown<'_> {
    /// The flight on `p`, its wind constant, or growing with height by `shear` above
    /// [`SHEAR_REFERENCE_HEIGHT_M`] as the synthetic flights' does.
    fn flown(&self, p: &Parameters, until_s: f64, shear: Option<f64>) -> Result<Simulated, String> {
        if p.drag_scale <= 0.0 || p.impulse_scale <= 0.0 || p.burn_time_scale <= 0.0 {
            return Err("a scale that isn't positive".to_owned());
        }
        let speed_m_s = p.wind_east_m_s.hypot(p.wind_north_m_s);
        // It blows toward (east, north), so from the opposite way, clockwise from north.
        let from_rad = (-p.wind_east_m_s)
            .atan2(-p.wind_north_m_s)
            .rem_euclid(std::f64::consts::TAU);
        let environment = match (self.era5_wind, shear) {
            (true, _) => self.environment.clone(),
            (false, None) => self
                .environment
                .clone()
                .with_wind(ConstantWind::new(speed_m_s, from_rad).map_err(|e| e.to_string())?),
            (false, Some(exponent)) => self.environment.clone().with_wind(
                PowerLawWind::new(
                    speed_m_s,
                    SHEAR_REFERENCE_HEIGHT_M,
                    exponent,
                    from_rad,
                    self.flight.site.2,
                )
                .map_err(|e| e.to_string())?,
            ),
        };
        let mut rocket = self.rocket.clone();
        for configuration in &mut rocket.configurations {
            for mounted in &mut configuration.motors {
                mounted.motor = dispersed_motor(&mounted.motor, p.impulse_scale, p.burn_time_scale)
                    .map_err(|e| e.to_string())?;
            }
        }
        let settings = FlightSettings {
            max_time_s: until_s,
            ..FlightSettings::default()
        };
        let simulation = Simulation::new(
            &rocket,
            "example",
            environment.clone(),
            rail_of(self.flight),
            settings,
        )
        .map_err(|e| e.to_string())?
        .with_drag_scale(p.drag_scale)
        .map_err(|e| e.to_string())?
        .with_structure_mass_scale(p.mass_scale)
        .map_err(|e| e.to_string())?;
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

/// One parameter of a fit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fitted {
    /// Its name, as the fit's page gives it.
    pub name: String,
    /// The fit's value.
    pub value: f64,
    /// Its standard error.
    pub standard_error: f64,
    /// Its 90% interval, the standard error times the shares fitted on the calibration set's
    /// whole-log fits (`hpr_forensics::synthetic::fits::shares`). `None` for the burn time's
    /// scale, which the fit isn't asked for; for the wind on a log with no track, whose shares
    /// were fitted with the synthetic flights' own stated wind and don't carry over to a flight's
    /// own; and for every parameter of a fit up to the cut, which no shares were fitted on.
    pub interval_90: Option<(f64, f64)>,
}

/// A fit of one log, whole or cut.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FitRow {
    /// Whether the log was fitted: otherwise each parameter is what was stated of it.
    pub fitted: bool,
    /// Whether its last pass ended where no step lowered χ² further.
    pub converged: bool,
    /// Whether the log holds a horizontal track, which the intervals' shares tell apart.
    pub drift: bool,
    /// The heights' residual's RMS, m.
    pub rms_m: Option<f64>,
    /// That residual as a share of the spread of the heights read.
    pub misfit_share: Option<f64>,
    /// Simulations flown for it.
    pub flown: u64,
    /// The parameters, in the fit's order.
    pub parameters: Vec<Fitted>,
}

impl FitRow {
    /// `fit`'s row; with `calibrated`, the parameters' 90% intervals where shares apply.
    fn of(fit: &Fit, drift: bool, calibrated: bool) -> Self {
        let values = fit.parameters.to_array();
        let names = [
            ("drag scale", Some(Asked::DragScale)),
            ("impulse scale", Some(Asked::ImpulseScale)),
            ("burn time scale", None),
            ("wind toward the east (m/s)", Some(Asked::WindEast)),
            ("wind toward the north (m/s)", Some(Asked::WindNorth)),
            ("structure mass scale", Some(Asked::MassScale)),
        ];
        let parameters = names
            .iter()
            .zip(values.iter().zip(&fit.standard_errors))
            .map(|(&(name, asked), (&value, &standard_error))| Fitted {
                name: name.to_owned(),
                value,
                standard_error,
                interval_90: asked
                    .filter(|asked| {
                        calibrated
                            && fit.fitted
                            && (drift || !matches!(asked, Asked::WindEast | Asked::WindNorth))
                    })
                    .and_then(|asked| {
                        shares(asked, drift)
                            .interval(value, standard_error)
                            .map(|i| (i.low, i.high))
                    }),
            })
            .collect();
        Self {
            fitted: fit.fitted,
            converged: fit.converged,
            drift,
            rms_m: fit.rms_m,
            misfit_share: fit.misfit_share,
            flown: fit.flown,
            parameters,
        }
    }
}

/// What the prediction from the boost gave on one flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    /// An apogee.
    Predicted {
        /// The cut.
        cut: Cut,
        /// The fit of the log up to it.
        fit: FitRow,
        /// The fitted flight's apogee, m above the pad, as climbed.
        apogee_above_pad_m: f64,
        /// That apogee as the log's sensor reads it, m: what is set against the log's.
        apogee_reading_m: f64,
        /// Its error against the log's apogee, %.
        error_percent: f64,
    },
    /// None, and why.
    Unpredicted(Unpredicted),
}

/// One flight's fits and prediction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightBoost {
    /// The flight's id, as in `real-flights`.
    pub id: String,
    /// Its title.
    pub title: String,
    /// The altimeter's kind, as in `real-flights`.
    pub altimeter: String,
    /// The files read, with their hashes.
    pub files: Vec<FileRead>,
    /// The log's apogee, m, as `real-flights` reads it.
    pub log_apogee_m: f64,
    /// The forward simulation's, as `real-flights` reads it, m.
    pub forward_apogee_m: f64,
    /// Its error, %, as `real-flights` gives it.
    pub forward_error_percent: f64,
    /// The forward simulation's largest Mach number, as `real-flights` gives it.
    pub forward_max_mach: f64,
    /// The fit's model flown in ERA5's own wind with nothing changed, less the forward
    /// simulation's apogee, m: under [`SAME_FLIGHT_WITHIN_M`], or the run fails.
    pub same_flight_m: f64,
    /// The thrust file's burn, as flown and not scaled by any fit: its last time with thrust, s.
    pub burn_time_s: f64,
    /// The log's liftoff as the analyzer reads the whole log, s on the log's clock.
    pub liftoff_s: Option<f64>,
    /// Whether the analyzer, reading the whole log past the cut, withholds its top speed as at or
    /// past [`readings::TRANSONIC_SPEED_M_S`]: the climb leaves the range the cut is meant for.
    pub log_past_transonic: bool,
    /// ERA5's wind averaged over the forward simulation's climb, toward the east and north, m/s:
    /// what the fit is told of the wind.
    pub stated_wind_m_s: (f64, f64),
    /// The fit's model at what was stated, nothing fitted, in that constant wind: its apogee as
    /// the log reads it, m.
    pub stated_apogee_m: f64,
    /// The whole log's fit, from a second before liftoff to its apogee.
    pub whole: FitRow,
    /// Why the whole log has nothing to fit, where it hasn't: the reader's words for its apogee
    /// withheld, or that too few bins hold a sample.
    pub whole_unread: Option<String>,
    /// The prediction from the boost.
    pub boost: Outcome,
}

impl FlightBoost {
    /// The cut, where there was one.
    fn cut(&self) -> Option<Cut> {
        match &self.boost {
            Outcome::Predicted { cut, .. }
            | Outcome::Unpredicted(
                Unpredicted::NothingToFit(cut)
                | Unpredicted::NotFitted { cut, .. }
                | Unpredicted::NoApogee(cut),
            ) => Some(*cut),
            Outcome::Unpredicted(Unpredicted::NoCut(_)) => None,
        }
    }

    /// Whether the cut came less than the thrust file's burn after the log's liftoff. Liftoff
    /// follows ignition, so this also takes in a cut up to that lag after the burn ends.
    fn cut_in_burn(&self) -> Option<bool> {
        Some(self.cut()?.time_s - self.liftoff_s? < self.burn_time_s)
    }

    /// Whether the flight has a prediction from a cut after its burn, on a log whose climb stays
    /// under the transonic speed: the case the method is meant for.
    fn in_range(&self) -> bool {
        self.error_percent().is_some()
            && self.cut_in_burn() == Some(false)
            && !self.log_past_transonic
    }

    /// The stated model's apogee error, %.
    fn stated_error_percent(&self) -> f64 {
        100.0 * (self.stated_apogee_m - self.log_apogee_m) / self.log_apogee_m
    }

    /// The prediction's error, %.
    fn error_percent(&self) -> Option<f64> {
        match self.boost {
            Outcome::Predicted { error_percent, .. } => Some(error_percent),
            Outcome::Unpredicted(_) => None,
        }
    }
}

/// The errors on a set of predicted flights, side by side.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Means {
    /// The flights.
    pub flights: Vec<String>,
    /// The prediction's mean absolute apogee error on them, %.
    pub prediction_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign, %.
    pub prediction_mean_percent: Option<f64>,
    /// The forward simulation's mean absolute apogee error on the same flights, %.
    pub forward_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign on them, %.
    pub forward_mean_percent: Option<f64>,
    /// The fit's model at what was stated, nothing fitted, on the same flights: its mean absolute
    /// apogee error, %.
    pub stated_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign, %.
    pub stated_mean_percent: Option<f64>,
    /// The mean of each flight's absolute error from the prediction less the forward
    /// simulation's, percentage points: below zero, the prediction came closer.
    pub difference_points: Option<f64>,
    /// That difference's exact sign-flip interval at [`LEVEL`], each end `None` where it is
    /// unbounded; `None` for no flights.
    pub difference_interval: Option<(Option<f64>, Option<f64>)>,
}

impl Means {
    fn of<'a>(flights: impl Iterator<Item = &'a FlightBoost> + Clone) -> Self {
        let errors = || flights.clone().filter_map(FlightBoost::error_percent);
        let forward = || flights.clone().map(|f| f.forward_error_percent);
        let stated = || flights.clone().map(FlightBoost::stated_error_percent);
        let differences: Vec<f64> = flights
            .clone()
            .filter_map(|f| Some(f.error_percent()?.abs() - f.forward_error_percent.abs()))
            .collect();
        let finite = |x: f64| x.is_finite().then_some(x);
        Self {
            flights: flights.clone().map(|f| f.id.clone()).collect(),
            prediction_mean_absolute_percent: mean(errors().map(f64::abs)),
            prediction_mean_percent: mean(errors()),
            forward_mean_absolute_percent: mean(forward().map(f64::abs)),
            forward_mean_percent: mean(forward()),
            stated_mean_absolute_percent: mean(stated().map(f64::abs)),
            stated_mean_percent: mean(stated()),
            difference_points: mean(differences.iter().copied()),
            difference_interval: boost::sign_flip_interval(&differences, LEVEL)
                .map(|(low, high)| (finite(low), finite(high))),
        }
    }
}

/// The report's summary, all of it computed from its rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    /// The flights.
    pub flights: usize,
    /// Every flight with a prediction.
    pub predicted: Means,
    /// Those of them cut after the burn, on a log whose climb stays under the transonic speed.
    pub in_range: Means,
    /// The flights cut less than their thrust file's burn after liftoff.
    pub cut_in_burn: Vec<String>,
    /// The flights whose whole log climbs at or past the transonic speed.
    pub past_transonic: Vec<String>,
    /// The forward simulation's mean absolute apogee error on all the flights, %.
    pub forward_all_mean_absolute_percent: f64,
    /// Its mean error with its sign on all of them, %.
    pub forward_all_mean_percent: f64,
}

/// What is stated of each parameter before a log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatedRow {
    /// The parameter's name.
    pub name: String,
    /// Its middle; `None` for the wind, which is each flight's.
    pub mean: Option<f64>,
    /// Its standard deviation.
    pub sd: f64,
}

/// The report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// What is stated of each parameter.
    pub stated: Vec<StatedRow>,
    /// The flights.
    pub flights: Vec<FlightBoost>,
    /// The summary.
    pub summary: Summary,
}

/// The mean of `values`; `None` for none.
fn mean(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, count) = values.fold((0.0, 0_u32), |(s, c), v| (s + v, c + 1));
    (count > 0).then(|| sum / f64::from(count))
}

impl Summary {
    fn of(flights: &[FlightBoost]) -> Self {
        let ids = |keep: &dyn Fn(&FlightBoost) -> bool| -> Vec<String> {
            flights
                .iter()
                .filter(|f| keep(f))
                .map(|f| f.id.clone())
                .collect()
        };
        Self {
            flights: flights.len(),
            predicted: Means::of(flights.iter().filter(|f| f.error_percent().is_some())),
            in_range: Means::of(flights.iter().filter(|f| f.in_range())),
            cut_in_burn: ids(&|f| f.cut_in_burn() == Some(true)),
            past_transonic: ids(&|f| f.log_past_transonic),
            forward_all_mean_absolute_percent: mean(
                flights.iter().map(|f| f.forward_error_percent.abs()),
            )
            .unwrap_or(f64::NAN),
            forward_all_mean_percent: mean(flights.iter().map(|f| f.forward_error_percent))
                .unwrap_or(f64::NAN),
        }
    }
}

/// ERA5's wind toward the east and north, averaged evenly in height over the forward
/// simulation's climb, from the pad to its apogee as climbed.
fn mean_wind(
    environment: &Environment,
    pad_msl_m: f64,
    climb_m: f64,
) -> Result<(f64, f64), String> {
    let (mut east, mut north) = (0.0, 0.0);
    let levels = f64::from(WIND_LEVELS);
    for k in 0..WIND_LEVELS {
        let height = pad_msl_m + climb_m * (f64::from(k) + 0.5) / levels;
        let v = environment
            .wind
            .wind(height)
            .map_err(|e| e.to_string())?
            .velocity_enu_m_s;
        east += v.x / levels;
        north += v.y / levels;
    }
    Ok((east, north))
}

/// The flight on `p`'s apogee as a sensor of `kind` reads it, m.
fn top_reading(forward: &mut Flown<'_>, p: &Parameters, kind: HeightKind) -> Result<f64, String> {
    let flown = forward.fly(p, boost::APOGEE_HORIZON_S)?;
    flown
        .top_reading_m(kind)
        .ok_or_else(|| "the flight reaches no apogee".to_owned())
}

/// Fits and predicts one flight.
fn fly(root: &Path, flight: &RealFlight, row: &FlightRow) -> Result<FlightBoost, String> {
    let failed = |error: &dyn std::fmt::Display| format!("{}: {error}", flight.id);
    let mut files = Vec::new();
    let mut read = |relative: String, what: &'static str| -> Result<Vec<u8>, String> {
        let bytes = read_bytes(root, &relative, what).map_err(|error| failed(&error))?;
        files.push(FileRead {
            sha256: sha256_hex(&bytes),
            path: relative,
        });
        Ok(bytes)
    };
    let log_bytes = read(
        format!("{ROCKETPY}/data/rockets/{}", flight.log.file),
        "reading a flight log",
    )?;
    let log = crate::flight_residuals::flight_log(root, flight, &log_bytes)
        .map_err(|error| failed(&error))?;
    let (thrust_path, burn, reshape) =
        example_thrust(root, flight).map_err(|error| failed(&error))?;
    let thrust_bytes = read(thrust_path.clone(), "reading a thrust file")?;
    let (curve, _) = parse_thrust(
        flight.id,
        &thrust_path,
        &String::from_utf8_lossy(&thrust_bytes),
        burn,
        reshape,
    )
    .map_err(|error| failed(&error))?;
    let burn_time_s = curve
        .iter()
        .filter(|&&(_, thrust)| thrust > 0.0)
        .map(|&(t, _)| t)
        .fold(f64::NAN, f64::max);
    if !burn_time_s.is_finite() {
        return Err(failed(&format!("{thrust_path} holds no thrust")));
    }
    // The log's height as `real-flights` reads it.
    let kind = match flight.log.altimeter {
        Altimeter::Height(_) => HeightKind::Geometric,
        Altimeter::Barometric(_) | Altimeter::AssumedBarometric(_) => HeightKind::Barometric,
    };
    let design_path = format!("validation/designs/{}.json", flight.design);
    let design_bytes = read(design_path.clone(), "reading a design")?;
    let design: Value = serde_json::from_slice(&design_bytes).map_err(|error| failed(&error))?;
    let rocket = rocket_on_curve(flight, &design, &design_path, &thrust_path, &curve)
        .map_err(|error| failed(&error))?;
    let weather_path = format!("{ROCKETPY}/data/weather/{}", flight.weather);
    let weather_bytes = read(weather_path.clone(), "reading an ERA5 file")?;
    let (environment, _) =
        launch_day(flight, &weather_path, &weather_bytes).map_err(|error| failed(&error))?;

    let stated_wind_m_s = mean_wind(&environment, flight.site.2, row.hpr_height_apogee_m)
        .map_err(|error| failed(&error))?;
    let mut stated: Stated = synthetic_fits::stated();
    stated[Asked::WindEast.index()].mean = stated_wind_m_s.0;
    stated[Asked::WindNorth.index()].mean = stated_wind_m_s.1;
    let nominal = Parameters::from_array(stated.map(|p| p.mean));
    let mut forward = Flown {
        rocket: &rocket,
        environment: &environment,
        flight,
        era5_wind: true,
    };
    let same_flight_m = top_reading(&mut forward, &nominal, kind)
        .map_err(|error| failed(&error))?
        - row.hpr_apogee_m;
    if same_flight_m.abs() > SAME_FLIGHT_WITHIN_M || !same_flight_m.is_finite() {
        return Err(failed(&format!(
            "the fit's model in ERA5's wind lands {same_flight_m:+.3} m from the forward \
             simulation's apogee"
        )));
    }
    forward.era5_wind = false;
    let stated_apogee_m =
        top_reading(&mut forward, &nominal, kind).map_err(|error| failed(&error))?;

    let whole_read = readings::read(&log);
    let observed = fit::observations(&log, &whole_read);
    let whole_fit = fit::fit(
        observed.as_ref(),
        &stated,
        Noise::FromResiduals,
        &mut forward,
    );
    let drift = |o: Option<&Observations>| o.is_some_and(|o| !o.horizontal.is_empty());
    let whole = FitRow::of(
        &whole_fit,
        whole_fit.fitted && drift(observed.as_ref()),
        true,
    );
    let log_past_transonic = matches!(
        &whole_read.max_speed,
        Reading::Withheld(withheld) if withheld.reason == Reason::Transonic
    );
    let whole_unread = observed.is_none().then(|| match &whole_read.apogee {
        Reading::Withheld(withheld) => format!("its apogee is withheld: {}", withheld.detail),
        Reading::Read(_) => {
            "too few of its bins hold a sample, or its climb is too long".to_owned()
        }
    });
    let liftoff_s = match whole_read.liftoff {
        Reading::Read(liftoff) => Some(liftoff.time_s),
        Reading::Withheld(_) => None,
    };

    let mut boost_drift = false;
    let boost = match boost::predict(
        &log,
        |o| {
            boost_drift = drift(Some(o));
            stated
        },
        Noise::FromResiduals,
        &mut forward,
    ) {
        Ok(prediction) => Outcome::Predicted {
            cut: prediction.cut,
            fit: FitRow::of(&prediction.fit, boost_drift, false),
            apogee_above_pad_m: prediction.apogee_above_pad_m,
            apogee_reading_m: prediction.apogee_reading_m,
            error_percent: 100.0 * (prediction.apogee_reading_m - row.log_apogee_m)
                / row.log_apogee_m,
        },
        Err(why) => Outcome::Unpredicted(why),
    };
    Ok(FlightBoost {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        altimeter: flight.log.altimeter.kind().to_owned(),
        files,
        log_apogee_m: row.log_apogee_m,
        forward_apogee_m: row.hpr_apogee_m,
        forward_error_percent: row.apogee_error_percent,
        forward_max_mach: row.hpr_max_mach,
        same_flight_m,
        burn_time_s,
        liftoff_s,
        log_past_transonic,
        stated_wind_m_s,
        stated_apogee_m,
        whole,
        whole_unread,
        boost,
    })
}

/// A number to `digits` decimals, or "none".
fn or_none(value: Option<f64>, digits: usize) -> String {
    value.map_or_else(|| "none".to_owned(), |v| format!("{v:.digits$}"))
}

/// A signed percentage, or "none".
fn percent(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_owned(), |v| format!("{v:+.2}%"))
}

/// Why there's no prediction, in words.
fn why_text(why: &Unpredicted) -> String {
    match why {
        Unpredicted::NoCut(why) => match why {
            NoCut::NoTrack => "no cut: no height track".to_owned(),
            NoCut::Gnss => "no cut: a GNSS receiver's track".to_owned(),
            NoCut::TooFast => "no cut: logged faster than the analyzer reads".to_owned(),
            NoCut::TooSparse => {
                "no cut: logged too slowly for the cut's half-second windows".to_owned()
            }
            NoCut::NeverLifted => "no cut: it never lifts off".to_owned(),
            NoCut::Transonic { time_s, speed_m_s } => {
                format!("no cut: climbing at {speed_m_s:.1} m/s at {time_s:.2} s, too fast")
            }
            NoCut::NeverSlowed => "no cut: it never slows enough".to_owned(),
        },
        Unpredicted::NothingToFit(_) => "cut, but too little before the cut to fit".to_owned(),
        Unpredicted::NotFitted { misfit_share, .. } => format!(
            "cut, but the fit kept what was stated (misfit share {})",
            or_none(*misfit_share, 3)
        ),
        Unpredicted::NoApogee(_) => "cut, but the fitted flight reached no apogee".to_owned(),
    }
}

/// An interval's end, or "unbounded".
fn end_text(end: Option<f64>) -> String {
    end.map_or_else(|| "unbounded".to_owned(), |v| format!("{v:+.2}"))
}

/// The summary's lines for one set of predicted flights.
fn means_line(page: &mut String, which: &str, m: &Means) {
    let pair = |a: Option<f64>, b: Option<f64>| match (a, b) {
        (Some(a), Some(b)) => format!("{a:.2}% ({b:+.2}% with its sign)"),
        _ => "none".to_owned(),
    };
    let names = if m.flights.is_empty() {
        "none".to_owned()
    } else {
        m.flights.join(", ")
    };
    let _ = writeln!(
        page,
        "- {which}, {} ({names}): the prediction's mean absolute apogee error is {}; the forward \
         simulation's on the same flights {}; the fit's model at what was stated, nothing fitted, {}. \
         The prediction's absolute error less the forward simulation's is {} percentage points on \
         average; its exact sign-flip interval: {}.",
        m.flights.len(),
        pair(
            m.prediction_mean_absolute_percent,
            m.prediction_mean_percent
        ),
        pair(m.forward_mean_absolute_percent, m.forward_mean_percent),
        pair(m.stated_mean_absolute_percent, m.stated_mean_percent),
        m.difference_points
            .map_or_else(|| "none".to_owned(), |v| format!("{v:+.2}")),
        match m.difference_interval {
            None => "none".to_owned(),
            Some((None, None)) => format!(
                "unbounded, as on {} no pattern of signs is rare enough at {:.0}% (it takes 6 flights)",
                match m.flights.len() {
                    1 => "1 flight".to_owned(),
                    n => format!("{n} flights"),
                },
                100.0 * LEVEL
            ),
            Some((low, high)) => format!("{} to {}", end_text(low), end_text(high)),
        }
    );
}

impl Report {
    /// The report's page.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut page = String::new();
        let s = &self.summary;
        let names = |ids: &[String]| {
            if ids.is_empty() {
                "none".to_owned()
            } else {
                ids.join(", ")
            }
        };
        let _ = writeln!(
            page,
            "# Apogee predicted from the boost, on the logs of real flights\n"
        );
        let _ = writeln!(
            page,
            "This report asks whether apogee predicted from a log cut just after burnout comes closer to \
             the logged apogee than the forward simulation, the simulator's flight of the design with \
             nothing fitted. It covers the 7 public logged flights. {} get a prediction, {} of them in \
             the case the method is meant for, so it can't yet say at {:.0}% which comes closer. An \
             error is an apogee less the log's, as a share of the log's: + is too high.\n",
            s.predicted.flights.len(),
            s.in_range.flights.len(),
            100.0 * LEVEL
        );
        let _ = writeln!(
            page,
            "Written by `cargo xtask real-flights` ([M7.3d2][m7-3d], decisions [ADR-257][adr-257] §3 and \
             [ADR-325][adr-325]). How the prediction works is on the [fit's page][site].\n"
        );
        let _ = writeln!(
            page,
            "- **What is flown:** the 7 flights of the [real-flights report][real], each in its ERA5 air, \
             from its example's thrust file, rail and site.\n\
             - **The prediction:** the library finds burnout from the samples before the cut, cuts the \
             log there, fits it, and flies the fit on to apogee (`hpr_forensics::boost`). A barometer's \
             log whose fitted climb speed reaches 227.7 m/s (747 ft/s) before the cut gets none. Its \
             apogee is read as the forward simulation's is: the standard atmosphere's altitude of the \
             ERA5 pressure at the center of mass, less the pad's.\n\
             - **The fit:** least squares, told the design, rail, site and ERA5 air. Its wind is constant \
             with height; what's stated of it is ERA5's, averaged evenly in height from the pad to the \
             forward simulation's apogee. Each parameter is held to what's stated of it (below): the \
             synthetic flights' spreads, not a measure of a real flight's model error.\n\
             - **The case the method is meant for:** a cut after the thrust file's burn, on a log whose \
             climb, read whole, stays under 227.7 m/s. A cut less than the burn after liftoff counts as \
             inside it; as liftoff lags ignition, that also takes in a cut up to that lag after the \
             burn's end. The burn is the file's, not scaled by any fit.\n\
             - **The interval:** on the difference, flight by flight, between the prediction's absolute \
             error and the forward simulation's. It is the exact sign-flip interval (Hartigan, 1969), \
             whose coverage is at least {:.0}% for independent differences, each drawn symmetric about \
             their center. With 5 flights or fewer, no pattern of signs is rare enough at that level, \
             and it is unbounded.\n\
             - **Files:** the logs, thrust files and weather files are read from the pinned RocketPy \
             checkout and never committed; each one's SHA-256 is in the JSON.\n",
            100.0 * LEVEL
        );
        let _ = writeln!(
            page,
            "[m7-3d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3d\n\
             [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md\n\
             [adr-325]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0325-m7-3d2-the-public-flights-prediction-from-the-boost.md\n\
             [site]: https://hpr.fusionspace.co/physics/flight-fit.html#apogee-from-the-boost\n\
             [real]: real-flights.md\n"
        );
        let _ = writeln!(page, "## Summary\n");
        means_line(&mut page, "Every flight with a prediction", &s.predicted);
        means_line(
            &mut page,
            "In the case the method is meant for",
            &s.in_range,
        );
        let _ = writeln!(
            page,
            "- Set beside the forward simulation's, the stated model's error shows what the fit's \
             constant wind alone moves; beside the prediction's, what the fit to the boost moves."
        );
        let _ = writeln!(
            page,
            "- Cut less than the thrust file's burn after liftoff: {}. The finder can read a dip in a \
             barometer's fitted speed during the boost as burnout \
             ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)).",
            names(&s.cut_in_burn)
        );
        let _ = writeln!(
            page,
            "- Climbing at or past 227.7 m/s, the log read whole: {}. A cut before that speed on such a \
             log came on a dip, not at burnout.",
            names(&s.past_transonic)
        );
        let _ = writeln!(
            page,
            "- The forward simulation on all {} flights: mean absolute apogee error {:.2}% ({:+.2}% with \
             its sign).\n",
            s.flights, s.forward_all_mean_absolute_percent, s.forward_all_mean_percent
        );

        let _ = writeln!(page, "## The prediction\n");
        let _ = writeln!(
            page,
            "The stated apogee is the fit's model, nothing fitted, in its constant wind.\n"
        );
        let _ = writeln!(
            page,
            "| flight | outcome | log apogee (m) | predicted (m) | error | forward (m) | forward error | \
             stated (m) | stated error |"
        );
        let _ = writeln!(page, "|---|---|---:|---:|---:|---:|---:|---:|---:|");
        for f in &self.flights {
            let (outcome, predicted) = match &f.boost {
                Outcome::Predicted {
                    apogee_reading_m, ..
                } => ("predicted".to_owned(), format!("{apogee_reading_m:.1}")),
                Outcome::Unpredicted(why) => (why_text(why), "none".to_owned()),
            };
            let _ = writeln!(
                page,
                "| {} | {} | {:.1} | {} | {} | {:.1} | {:+.2}% | {:.1} | {:+.2}% |",
                f.title,
                outcome,
                f.log_apogee_m,
                predicted,
                percent(f.error_percent()),
                f.forward_apogee_m,
                f.forward_error_percent,
                f.stated_apogee_m,
                f.stated_error_percent()
            );
        }
        let _ = writeln!(page);

        let _ = writeln!(page, "## The cut\n");
        let _ = writeln!(
            page,
            "Times are on the log's clock; the burn is the thrust file's, as flown.\n"
        );
        let _ = writeln!(
            page,
            "| flight | liftoff (s) | cut (s) | top speed before the cut (m/s) | burn (s) | cut in the burn | \
             climbs past 227.7 m/s | forward's largest Mach |"
        );
        let _ = writeln!(page, "|---|---:|---:|---:|---:|---|---|---:|");
        let yes = |b: bool| if b { "yes" } else { "no" };
        for f in &self.flights {
            let cut = f.cut();
            let _ = writeln!(
                page,
                "| {} | {} | {} | {} | {:.2} | {} | {} | {:.3} |",
                f.title,
                or_none(f.liftoff_s, 2),
                or_none(cut.map(|c| c.time_s), 2),
                or_none(cut.map(|c| c.top_speed_m_s), 1),
                f.burn_time_s,
                f.cut_in_burn().map_or("none", yes),
                yes(f.log_past_transonic),
                f.forward_max_mach
            );
        }
        let _ = writeln!(page);

        let _ = writeln!(page, "## What is stated before a log\n");
        let _ = writeln!(
            page,
            "Each parameter is held to a normal distribution about its middle, as on the synthetic \
             flights. The wind's middle is each flight's own (below).\n"
        );
        let _ = writeln!(page, "| parameter | middle | standard deviation |");
        let _ = writeln!(page, "|---|---:|---:|");
        for row in &self.stated {
            let _ = writeln!(
                page,
                "| {} | {} | {:.4} |",
                row.name,
                row.mean
                    .map_or_else(|| "the flight's".to_owned(), |m| format!("{m:.3}")),
                row.sd
            );
        }
        let _ = writeln!(page);

        let _ = writeln!(page, "## The fits\n");
        let _ = writeln!(
            page,
            "Each log fitted whole, from a second before liftoff to its apogee, and up to its cut where \
             it has a prediction. A whole-log fit's 90% interval is the standard error times the shares \
             fitted on the synthetic calibration set's whole-log fits. The burn time's scale, which the \
             fit isn't asked for, has none. Nor has the wind on a log with no track, whose shares were \
             fitted with the synthetic flights' own stated wind, nor any fit up to the cut, which no \
             shares were fitted on. A scale is the factor that best fits one flight, not the rocket's \
             drag or motor at every speed; on a real flight the model's own errors go into it.\n"
        );
        for f in &self.flights {
            let _ = writeln!(page, "### {}\n", f.title);
            let _ = writeln!(
                page,
                "Stated wind: {:.2} m/s toward the east, {:.2} m/s toward the north. The fit's model in \
                 ERA5's own wind lands {:+.3} m from the forward simulation's apogee.\n",
                f.stated_wind_m_s.0, f.stated_wind_m_s.1, f.same_flight_m
            );
            if let Some(why) = &f.whole_unread {
                let _ = writeln!(page, "The whole log has nothing to fit: {why}.\n");
            }
            let mut fits = vec![("whole log", &f.whole)];
            if let Outcome::Predicted { fit, .. } = &f.boost {
                fits.push(("up to the cut", fit));
            }
            for (which, fit) in fits {
                if !fit.fitted {
                    let _ = writeln!(page, "Fitted {which}: not fitted.\n");
                    continue;
                }
                let _ = writeln!(
                    page,
                    "Fitted {which}: {}residual RMS {} m, misfit share {}, {} simulations flown.\n",
                    if fit.converged { "" } else { "not converged, " },
                    or_none(fit.rms_m, 2),
                    or_none(fit.misfit_share, 4),
                    fit.flown
                );
                let _ = writeln!(
                    page,
                    "| parameter | value | standard error | 90% interval |"
                );
                let _ = writeln!(page, "|---|---:|---:|---:|");
                for p in &fit.parameters {
                    let _ = writeln!(
                        page,
                        "| {} | {:.3} | {:.3} | {} |",
                        p.name,
                        p.value,
                        p.standard_error,
                        p.interval_90.map_or_else(
                            || "none".to_owned(),
                            |(low, high)| format!("{low:.3} to {high:.3}")
                        )
                    );
                }
                let _ = writeln!(page);
            }
        }
        page
    }
}

/// What is stated of each parameter, the wind's middle left to each flight.
fn stated_rows() -> Vec<StatedRow> {
    let stated = synthetic_fits::stated();
    [
        ("drag scale", true),
        ("impulse scale", true),
        ("burn time scale", true),
        ("wind toward the east (m/s)", false),
        ("wind toward the north (m/s)", false),
        ("structure mass scale", true),
    ]
    .iter()
    .zip(stated)
    .map(|(&(name, own), prior)| StatedRow {
        name: name.to_owned(),
        mean: own.then_some(prior.mean),
        sd: prior.sd,
    })
    .collect()
}

/// Fits and predicts every flight, then writes or checks the report.
pub fn run(root: &Path, real: &RealFlightReport, check: bool) -> Result<(), String> {
    let mut flights = Vec::new();
    for flight in &FLIGHTS {
        let row = real
            .flights
            .iter()
            .find(|row| row.id == flight.id)
            .ok_or_else(|| format!("{}: not in the real-flights report", flight.id))?;
        let flown = fly(root, flight, row)?;
        println!(
            "from-boost: {:<16} cut {:>6} s  {:<40} forward {:>+6.2}%",
            flown.id,
            or_none(flown.cut().map(|c| c.time_s), 2),
            match &flown.boost {
                Outcome::Predicted { error_percent, .. } =>
                    format!("predicted {error_percent:+.2}%"),
                Outcome::Unpredicted(why) => why_text(why),
            },
            flown.forward_error_percent
        );
        flights.push(flown);
    }
    let summary = Summary::of(&flights);
    let report = Report {
        stated: stated_rows(),
        flights,
        summary,
    };
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: Value = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let flown: Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
        crate::flight_residuals::agrees("", &committed, &flown)?;
        println!("from-boost: the committed report reproduces");
        return Ok(());
    }
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, report.to_markdown())] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("from-boost: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;

    fn committed() -> (Report, String) {
        let root = crate::designs::root().unwrap();
        let json = fs::read_to_string(root.join(REPORT_JSON)).unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        (serde_json::from_str(&json).unwrap(), page)
    }

    /// CI has no `refs/`: it holds the committed page to the committed data, and the summary to
    /// the rows.
    #[test]
    fn the_committed_page_is_its_data() {
        let (report, page) = committed();
        assert_eq!(report.to_markdown(), page);
        assert_eq!(Summary::of(&report.flights), report.summary);
        assert_eq!(report.stated, stated_rows());
    }

    /// The forward simulation's numbers are `real-flights`' own, and the fit's model, flown in
    /// ERA5's wind with nothing changed, is that flight.
    #[test]
    fn the_forward_simulation_is_real_flights_own() {
        let (report, _) = committed();
        let root = crate::designs::root().unwrap();
        let text = fs::read_to_string(root.join(hpr_validate::real_flight::REPORT_JSON)).unwrap();
        let real: RealFlightReport = serde_json::from_str(&text).unwrap();
        assert_eq!(report.flights.len(), real.flights.len());
        for (flight, row) in report.flights.iter().zip(&real.flights) {
            assert_eq!(flight.id, row.id);
            assert_eq!(flight.log_apogee_m.to_bits(), row.log_apogee_m.to_bits());
            assert_eq!(
                flight.forward_apogee_m.to_bits(),
                row.hpr_apogee_m.to_bits()
            );
            assert_eq!(
                flight.forward_error_percent.to_bits(),
                row.apogee_error_percent.to_bits()
            );
            assert_eq!(
                flight.forward_max_mach.to_bits(),
                row.hpr_max_mach.to_bits()
            );
            assert!(
                flight.same_flight_m.abs() <= SAME_FLIGHT_WITHIN_M,
                "{}",
                flight.id
            );
        }
    }

    /// A prediction's error is its apogee reading against the log's.
    #[test]
    fn a_predictions_error_is_against_the_logs_apogee() {
        let (report, _) = committed();
        for flight in &report.flights {
            if let Outcome::Predicted {
                apogee_reading_m,
                error_percent,
                ..
            } = flight.boost
            {
                let error = 100.0 * (apogee_reading_m - flight.log_apogee_m) / flight.log_apogee_m;
                assert!((error - error_percent).abs() < 1e-9, "{}", flight.id);
            }
        }
    }

    /// The summary's difference and interval, on rows made up for the test: two flights, so the
    /// interval is unbounded; the difference is each absolute error less the other's.
    #[test]
    fn the_summary_sets_each_flights_errors_side_by_side() {
        let (report, _) = committed();
        let mut flights = report.flights.clone();
        let template = flights[0].clone();
        let cut = Cut {
            index: 10,
            time_s: 3.0,
            top_speed_s: 2.5,
            top_speed_m_s: 100.0,
        };
        let with = |error: f64, forward: f64| FlightBoost {
            forward_error_percent: forward,
            liftoff_s: Some(0.5),
            burn_time_s: 2.0,
            log_past_transonic: false,
            boost: Outcome::Predicted {
                cut,
                fit: template.whole.clone(),
                apogee_above_pad_m: 0.0,
                apogee_reading_m: 0.0,
                error_percent: error,
            },
            ..template.clone()
        };
        flights = vec![with(-2.0, 6.0), with(3.0, -1.0)];
        let summary = Summary::of(&flights);
        for m in [&summary.predicted, &summary.in_range] {
            assert_eq!(m.flights.len(), 2);
            assert_eq!(m.prediction_mean_absolute_percent, Some(2.5));
            assert_eq!(m.prediction_mean_percent, Some(0.5));
            assert_eq!(m.forward_mean_absolute_percent, Some(3.5));
            assert_eq!(m.forward_mean_percent, Some(2.5));
            assert_eq!(m.difference_points, Some(-1.0));
            assert_eq!(m.difference_interval, Some((None, None)));
        }
        assert!(
            summary.cut_in_burn.is_empty(),
            "cut 2.5 s after liftoff, a 2 s burn"
        );
        // A cut inside the burn keeps the flight among the predicted, out of the range.
        flights[1].burn_time_s = 2.6;
        let summary = Summary::of(&flights);
        assert_eq!(summary.cut_in_burn, [flights[1].id.clone()]);
        assert_eq!(summary.predicted.flights.len(), 2);
        assert_eq!(summary.in_range.flights.len(), 1);
        assert_eq!(summary.in_range.prediction_mean_absolute_percent, Some(2.0));
        assert_eq!(summary.in_range.forward_mean_percent, Some(6.0));
        // So does a log that later climbs past the transonic speed.
        flights[1].burn_time_s = 2.0;
        flights[0].log_past_transonic = true;
        let summary = Summary::of(&flights);
        assert_eq!(summary.past_transonic, [flights[0].id.clone()]);
        assert_eq!(summary.predicted.flights.len(), 2);
        assert_eq!(summary.in_range.flights.len(), 1);
        assert_eq!(summary.in_range.prediction_mean_absolute_percent, Some(3.0));
    }
}
