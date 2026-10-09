//! `MonteCarlo`: a rocket flown many times, each flight's inputs scattered at random about the
//! nominal flight's, as `hpr mc` flies it; the run's table as NumPy arrays.

use hpr::hpr_analysis::montecarlo::{Dispersion, FailedAt, MonteCarlo as Runner, Outcome, Run};
use hpr::hpr_analysis::table::{RunTable, Values};
use hpr::hpr_sim::FlightSummary;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::Serialize;

use crate::flight::{DragTable, Environment, with_drag};
use crate::models::Raised;
use crate::rocket::Rocket;
use crate::{error, to_python};

/// The most flights a run takes, as `hpr mc --runs`: a run keeps every flight's summary, about a
/// kilobyte each, and 100,000 take minutes on a laptop.
const MAX_RUNS: u64 = 100_000;

/// A Monte Carlo run: `rocket` flown `runs` times from the same rail in `environment`, each
/// flight's inputs drawn at random about the nominal flight's, flown as soon as it is made.
///
/// `MonteCarlo(rocket, environment, rail_length_m, *, runs=100, seed=0, inclination_deg=90.0,
/// heading_deg=0.0, drag_table=None, drag=None, mass_sd_fraction=0.0, cg_sd_m=0.0,
/// drag_sd_fraction=0.0, impulse_sd_fraction=0.0, burn_time_sd_fraction=0.0, delay_sd_s=0.0,
/// wind_sd_fraction=0.0, wind_from_sd_deg=0.0, inclination_sd_deg=0.0, heading_sd_deg=0.0,
/// deployment_lag_sd_s=0.0)`: the rocket, the environment and the rail as `Flight` takes them,
/// and one standard deviation of each input the run scatters, each 0 (not scattered) unless
/// given, as `hpr mc`'s options are, with the same names as its JSON output: each stage's mass
/// without motors and each motor's impulse and burn time, the rocket's drag coefficient and the
/// wind's speed as fractions of their nominal values (0.02 is 2%); each stage's center of mass,
/// m; each motor's ejection delay and each parachute's lag after its trigger, s; the wind's
/// direction and the rail's angle and heading, degrees. Each input is drawn from a normal
/// distribution about its nominal value, independently. The guide's Monte Carlo page says what
/// each does and where to find numbers for them.
///
/// The run is the Rust library's `MonteCarlo::run`, its flights spread over the machine's
/// threads: a flight's draws depend only on the seed, its index and the inputs, so the same
/// seed, rocket and options fly the same flights, bit for bit, on one platform, on any number of
/// threads, and `hpr mc` built the same way (both release builds) flies them too. A flight that fails, its draw impossible (such as a
/// negative mass) or its flight stopped with an error, is counted (`failed`, `failures`) and
/// kept in the table, never dropped.
///
/// The run reads as a dictionary of NumPy arrays, one a column of `hpr mc --export`'s table, a
/// value a flight in index order: `run["apogee_m"]`. `columns` lists them: what each flight drew
/// and what it came to. A number is a `float64` array, NaN where a flight has none (a failed
/// flight, or a peak its flight didn't reach); `index` is a `uint64` array, and `outcome`,
/// `failed_at`, `reason`, `termination` and `flags` are arrays of strings, empty where a flight
/// has none.
///
/// A drag function or a wind function is called from the run's threads, one call at a time, and
/// an exception one raises stops the flights still to fly and is raised here. The calls come in
/// no set order, so a run repeats bit for bit only with a function whose answer depends on its
/// arguments alone.
#[pyclass(module = "fusionspace.hpr", frozen, skip_from_py_object)]
#[derive(Debug)]
pub struct MonteCarlo {
    runs: u64,
    seed: u64,
    /// Each option's name and its standard deviation, as given.
    given: Vec<(&'static str, f64)>,
    /// The nominal flight's metrics.
    nominal: FlightSummary,
    /// The failed flights, grouped by where they failed and why.
    failures: Vec<Failure>,
    failed: usize,
    table: RunTable,
}

/// The flights of a run that failed in one place for one reason, as `hpr mc`'s JSON gives them.
#[derive(Debug, Serialize)]
struct Failure {
    /// Making the flight's inputs from its draw, or flying them.
    at: FailedAt,
    /// The error.
    reason: String,
    /// How many flights failed so.
    count: usize,
    /// The first one's index: its row in the table.
    first_index: u64,
}

#[pymethods]
impl MonteCarlo {
    #[new]
    #[pyo3(signature = (
        rocket, environment, rail_length_m, *, runs = 100, seed = 0, inclination_deg = 90.0,
        heading_deg = 0.0, drag_table = None, drag = None, mass_sd_fraction = 0.0, cg_sd_m = 0.0,
        drag_sd_fraction = 0.0, impulse_sd_fraction = 0.0, burn_time_sd_fraction = 0.0,
        delay_sd_s = 0.0, wind_sd_fraction = 0.0, wind_from_sd_deg = 0.0,
        inclination_sd_deg = 0.0, heading_sd_deg = 0.0, deployment_lag_sd_s = 0.0
    ))]
    #[expect(
        clippy::too_many_arguments,
        reason = "Python's keyword arguments, one per option of a flight and per dispersion"
    )]
    fn new(
        py: Python<'_>,
        rocket: &Bound<'_, Rocket>,
        environment: &Environment,
        rail_length_m: f64,
        runs: u64,
        seed: u64,
        inclination_deg: f64,
        heading_deg: f64,
        drag_table: Option<&DragTable>,
        drag: Option<Py<PyAny>>,
        mass_sd_fraction: f64,
        cg_sd_m: f64,
        drag_sd_fraction: f64,
        impulse_sd_fraction: f64,
        burn_time_sd_fraction: f64,
        delay_sd_s: f64,
        wind_sd_fraction: f64,
        wind_from_sd_deg: f64,
        inclination_sd_deg: f64,
        heading_sd_deg: f64,
        deployment_lag_sd_s: f64,
    ) -> PyResult<Self> {
        if !(1..=MAX_RUNS).contains(&runs) {
            return Err(error(format!(
                "runs={runs}: a run flies 1 to {MAX_RUNS} flights"
            )));
        }
        let given = vec![
            ("mass_sd_fraction", mass_sd_fraction),
            ("cg_sd_m", cg_sd_m),
            ("drag_sd_fraction", drag_sd_fraction),
            ("impulse_sd_fraction", impulse_sd_fraction),
            ("burn_time_sd_fraction", burn_time_sd_fraction),
            ("delay_sd_s", delay_sd_s),
            ("wind_sd_fraction", wind_sd_fraction),
            ("wind_from_sd_deg", wind_from_sd_deg),
            ("inclination_sd_deg", inclination_sd_deg),
            ("heading_sd_deg", heading_sd_deg),
            ("deployment_lag_sd_s", deployment_lag_sd_s),
        ];
        for (name, value) in &given {
            if !(value.is_finite() && *value >= 0.0) {
                return Err(error(format!(
                    "{name}={value}: a standard deviation is a finite number, at least 0"
                )));
            }
        }
        // The library's dispersion, the angles in radians, as `hpr mc` makes it.
        let dispersion = Dispersion {
            dry_mass_sd_fraction: mass_sd_fraction,
            cg_sd_m,
            drag_sd_fraction,
            impulse_sd_fraction,
            burn_time_sd_fraction,
            ejection_delay_sd_s: delay_sd_s,
            wind_speed_sd_fraction: wind_sd_fraction,
            wind_heading_sd_rad: wind_from_sd_deg.to_radians(),
            rail_elevation_sd_rad: inclination_sd_deg.to_radians(),
            rail_azimuth_sd_rad: heading_sd_deg.to_radians(),
            deployment_lag_sd_s,
        };
        // A copy, so the rocket isn't held borrowed while the run flies without the GIL.
        let rocket = rocket.borrow().rocket.clone();
        // One slot for the run's functions, in every flight: the first exception stops the rest.
        let raised = Raised::default();
        let environment = environment.bound(py, &raised);
        let builder = hpr::Flight::builder(&rocket, &environment, rail_length_m)
            .inclination_deg(inclination_deg)
            .heading_deg(heading_deg);
        let builder = with_drag(builder, drag_table, drag, &raised)?;
        // The nominal flight first, as `hpr mc` flies it: a rocket that can't fly at all is
        // refused with the library's error, not counted as `runs` failed flights.
        let flown = py.detach(|| {
            let nominal = builder.fly().map_err(|refused| refused.to_string())?;
            let inputs = builder.inputs().map_err(|refused| refused.to_string())?;
            let runner = Runner::new(inputs, dispersion).map_err(|refused| refused.to_string())?;
            Ok::<_, String>((nominal, runner.run_parallel(seed, runs)))
        });
        if let Some(raised) = raised.take() {
            return Err(raised);
        }
        let (nominal, run) = flown.map_err(error)?;
        Ok(Self {
            runs,
            seed,
            given,
            nominal: nominal.summary().clone(),
            failures: failures(&run),
            failed: run.failed().count(),
            table: RunTable::new(&run),
        })
    }

    /// How many flights the run flew, failed ones included.
    #[getter]
    fn runs(&self) -> u64 {
        self.runs
    }

    /// The run's seed.
    #[getter]
    fn seed(&self) -> u64 {
        self.seed
    }

    /// How many flights failed.
    #[getter]
    fn failed(&self) -> usize {
        self.failed
    }

    /// How many flights flew: `runs` less `failed`.
    #[getter]
    fn flown(&self) -> usize {
        self.table.rows() - self.failed
    }

    /// The failed flights, grouped by where they failed and why, as `hpr mc`'s JSON lists them:
    /// one dictionary each, with `at` (`"inputs"`, making the flight's inputs from its draw, or
    /// `"flight"`, flying them), `reason`, `count` and `first_index`, the first such flight's
    /// row in the table.
    #[getter]
    fn failures<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.failures)
    }

    /// Each standard deviation the run was given, by its argument's name, 0 for one not
    /// scattered.
    #[getter]
    fn dispersion<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dispersion = PyDict::new(py);
        for (name, value) in &self.given {
            dispersion.set_item(name, value)?;
        }
        Ok(dispersion)
    }

    /// The nominal flight's summary, every input at its nominal value, as `Flight.summary` gives
    /// it.
    #[getter]
    fn nominal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.nominal)
    }

    /// The table's column names, in `hpr mc --export`'s order: `index`, `outcome`, `failed_at`
    /// and `reason`; what each flight drew, such as `drag_scale`, `stage_1_dry_mass_scale` and
    /// `motor_1_impulse_scale`; and what it came to, such as `apogee_m`, `landing_distance_m` and
    /// `flags`.
    #[getter]
    fn columns(&self) -> Vec<String> {
        self.names()
    }

    /// Whether the table has a column called `name`: `"apogee_m" in run`.
    fn __contains__(&self, name: &str) -> bool {
        self.table.column(name).is_some()
    }

    /// The column names, as `columns`: with `run[name]`, a run reads as a dictionary of arrays.
    fn keys(&self) -> Vec<String> {
        self.names()
    }

    /// The column names, one by one: `for name in run`.
    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        PyList::new(py, self.names())?
            .try_iter()
            .map(Bound::into_any)
    }

    /// The number of columns.
    fn __len__(&self) -> usize {
        self.table.columns().len()
    }

    /// One column as a new, read-only NumPy array, a value a flight: `run["apogee_m"]`.
    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        let column = self.table.column(name).ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!(
                "no column `{name}`; MonteCarlo.columns lists them"
            ))
        })?;
        let array = match &column.values {
            Values::Integers(values) => {
                let array = PyArray1::from_slice(py, values);
                array.readwrite().make_nonwriteable();
                array.into_any()
            }
            Values::Numbers(values) => {
                let values: Vec<f64> = values.iter().map(|v| v.unwrap_or(f64::NAN)).collect();
                let array = PyArray1::from_vec(py, values);
                array.readwrite().make_nonwriteable();
                array.into_any()
            }
            Values::Text(values) => {
                let array = py
                    .import("numpy")?
                    .call_method1("array", (values.clone(),))?;
                array.getattr("flags")?.setattr("writeable", false)?;
                array
            }
        };
        Ok(array)
    }

    /// The table as CSV text, the bytes `hpr mc --export` writes for the same run: a header of
    /// the columns in words with their units in brackets (`apogee [m]` for `run["apogee_m"]`),
    /// then a line a flight, each number in the shortest form that reads back
    /// to the same bits, a missing one an empty cell.
    fn to_csv(&self) -> PyResult<String> {
        self.table.csv().map_err(error)
    }

    fn __repr__(&self) -> String {
        format!(
            "MonteCarlo(runs={}, seed={}, failed={})",
            self.runs, self.seed, self.failed
        )
    }
}

impl MonteCarlo {
    fn names(&self) -> Vec<String> {
        self.table
            .columns()
            .iter()
            .map(|column| column.name.clone())
            .collect()
    }
}

/// `run`'s failed flights, grouped by where they failed and why, in the order each first failed.
fn failures(run: &Run) -> Vec<Failure> {
    let mut failures: Vec<Failure> = Vec::new();
    for sample in run.failed() {
        let Outcome::Failed { at, reason } = &sample.outcome else {
            continue;
        };
        match failures
            .iter_mut()
            .find(|failure| failure.at == *at && failure.reason == *reason)
        {
            Some(failure) => failure.count += 1,
            None => failures.push(Failure {
                at: *at,
                reason: reason.clone(),
                count: 1,
                first_index: sample.index,
            }),
        }
    }
    failures
}
