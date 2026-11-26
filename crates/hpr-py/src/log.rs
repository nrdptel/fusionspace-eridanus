//! `FlightLog`: a flight log read into one record, as `hpr analyze` reads it. It's read by its
//! format's own reader, or by a column mapping or record layout for a flyer's own logger
//! (`hpr_flightdata::custom`, M7.1d2). A workbook's sheet or a dump's recording is read as
//! chosen. A Raven FIP export that lost its header line is read under the header stated for it
//! (M7.2b).

use std::path::{Path, PathBuf};

use hpr::hpr_flightdata::custom;
use hpr::hpr_flightdata::error::LogError;
use hpr::hpr_flightdata::import;
use hpr::hpr_flightdata::log::FlightLog as Record;
use hpr::hpr_flightdata::readings;
use hpr::hpr_flightdata::smoother::{self, Smoothed, Withheld};
use hpr::hpr_forensics::residuals::HeightKind;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::Serialize;

use crate::flight::Environment;
use crate::models::Raised;
use crate::rocket::Rocket;
use crate::{error, to_python};

/// What ``FlightLog.smoothed`` gives: the track and its card as text, or why there is none.
#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum SmoothedOutcome {
    Read {
        #[serde(flatten)]
        smoothed: Box<Smoothed>,
        card: String,
    },
    Withheld(Withheld),
}

/// A flight log read into one record, as ``hpr analyze --record`` prints it. It holds a clock,
/// every channel the logger recorded on it, and what the file states about itself. The channels
/// are in SI units, degrees for angles. Read one with ``FlightLog.read``.
#[pyclass(module = "fusionspace.hpr", frozen, skip_from_py_object)]
#[derive(Debug)]
pub struct FlightLog {
    pub(crate) log: Record,
}

/// The file's bytes, or an error naming it.
fn read_bytes(path: &Path) -> PyResult<Vec<u8>> {
    std::fs::read(path).map_err(|e| error(format!("{}: {e}", path.display())))
}

/// A mapping's, a layout's or a FIP header's JSON: a dict's, or a file's at a path.
fn description_json(py: Python<'_>, given: &Bound<'_, PyAny>) -> PyResult<String> {
    if given.is_instance_of::<PyDict>() {
        // Indented, so that a refusal's line and column point into the dict's own lines; NaN
        // refused, as no field takes one.
        let options = PyDict::new(py);
        options.set_item("indent", 2)?;
        options.set_item("allow_nan", false)?;
        return py
            .import("json")?
            .call_method("dumps", (given,), Some(&options))
            .and_then(|text| text.extract::<String>())
            .map_err(|refused| {
                error(format!(
                    "a mapping, a layout or a FIP header given as a dict must be one JSON can \
                     hold: {refused}"
                ))
            });
    }
    let path: PathBuf = given.extract().map_err(|_| {
        error("a mapping, a layout or a FIP header is a dict, or the path of its JSON file")
    })?;
    custom::json_text(&read_bytes(&path)?).ok_or_else(|| {
        error(format!(
            "{}: not UTF-8 or UTF-16 text, as a JSON file is",
            path.display()
        ))
    })
}

/// A library error on the file at `path`, as the command line words it.
fn on(path: &Path, refused: LogError) -> PyErr {
    error(format!("{}: {refused}", path.display()))
}

/// A list of values as a read-only NumPy array, gaps as NaN.
fn array<'py>(py: Python<'py>, values: &[f64]) -> Bound<'py, PyArray1<f64>> {
    let array = PyArray1::from_slice(py, values);
    // The record is the log's: the arrays Python gets are read-only copies.
    array.readwrite().make_nonwriteable();
    array
}

#[pymethods]
impl FlightLog {
    /// Reads the flight log at ``path``. It's read by its format's own reader, or by one of:
    ///
    /// - ``mapping``, a column mapping, for a table.
    /// - ``layout``, a record layout, for binary records or a serial console's capture of them.
    ///
    /// A mapping or a layout is a dict, or the path of its JSON file. The schemas
    /// ``schema/cli/analyze-mapping.schema.json`` and ``analyze-layout.schema.json`` describe
    /// them. ``FlightLog.header_mapping`` gives the mapping a table's header states.
    ///
    /// ``sheet`` names a sheet of a workbook (``.xlsx`` or ``.xls``), from 0. The sheet is read by
    /// its format's own reader, a mapping or a FIP header. With no ``sheet``, it's the
    /// workbook's one sheet that reads as a log. ``recording`` names an Entacore AIM raw dump's
    /// recording, from 0.
    ///
    /// ``fip_header`` reads a Featherweight Raven's FIP export that lost its header line. The
    /// export is read under the header ``fip_header`` states, a dict or a JSON file's path
    /// (``schema/cli/analyze-fip-header.schema.json``).
    ///
    /// Raises ``HprError`` naming the field a mapping, layout or header fails, the choice a file
    /// of several records needs, or why the log doesn't read.
    #[staticmethod]
    #[pyo3(signature = (
        path, *, mapping = None, layout = None, sheet = None, recording = None, fip_header = None
    ))]
    fn read(
        py: Python<'_>,
        path: PathBuf,
        mapping: Option<&Bound<'_, PyAny>>,
        layout: Option<&Bound<'_, PyAny>>,
        sheet: Option<usize>,
        recording: Option<usize>,
        fip_header: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        // The options `hpr analyze` refuses together, refused the same way.
        let described = usize::from(mapping.is_some())
            + usize::from(layout.is_some())
            + usize::from(fip_header.is_some());
        if described > 1 {
            return Err(error(
                "a log is read by one of a mapping, a layout and a FIP header, not several",
            ));
        }
        if recording.is_some() && (described > 0 || sheet.is_some()) {
            return Err(error(
                "recording names a dump's recording, read by its own reader: it takes no \
                 mapping, layout, FIP header or sheet",
            ));
        }
        if sheet.is_some() && layout.is_some() {
            return Err(error(
                "sheet names a workbook's sheet, and a layout reads binary records, not a sheet",
            ));
        }
        let log = match (mapping, layout, fip_header) {
            (Some(mapping), ..) => {
                let mapping =
                    custom::mapping_from_json(&description_json(py, mapping)?).map_err(error)?;
                custom::read_by_mapping(&read_bytes(&path)?, &mapping, sheet)
            }
            (None, Some(layout), _) => {
                let layout =
                    custom::layout_from_json(&description_json(py, layout)?).map_err(error)?;
                custom::read_by_layout(&read_bytes(&path)?, &layout)
            }
            (None, None, Some(header)) => {
                let stated =
                    custom::fip_header_from_json(&description_json(py, header)?).map_err(error)?;
                custom::read_by_fip_header(&read_bytes(&path)?, &stated, sheet)
            }
            (None, None, None) => import::read_one(&read_bytes(&path)?, sheet, recording),
        };
        Ok(Self {
            log: log.map_err(|refused| on(&path, refused))?,
        })
    }

    /// What the header of the table at ``path`` states, as a column mapping: a dict to edit and
    /// give back to ``FlightLog.read`` as ``mapping``. Set each column's ``quantity`` and
    /// ``unit``. Set ``primary`` to the column, from 0, that is the barometric altitude above the
    /// pad. ``sheet`` names a sheet of a workbook (``.xlsx`` or ``.xls``), from 0.
    #[staticmethod]
    #[pyo3(signature = (path, *, sheet = None))]
    fn header_mapping<'py>(
        py: Python<'py>,
        path: PathBuf,
        sheet: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let (mapping, _) = custom::header_mapping(&read_bytes(&path)?, sheet)
            .map_err(|refused| on(&path, refused))?;
        to_python(py, &mapping)
    }

    /// The file format it was read as, such as ``"generic_table"`` for a table read by a mapping.
    #[getter]
    fn format<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.log.format)
    }

    /// The logger, as the file names it; ``"unknown"`` for one read by a mapping or a layout.
    #[getter]
    fn logger(&self) -> &str {
        &self.log.logger
    }

    /// The logger's serial number, as the file states it, or ``None``.
    #[getter]
    fn serial_number(&self) -> Option<&str> {
        self.log.serial_number.as_deref()
    }

    /// The logger's firmware version, as the file states it, or ``None``.
    #[getter]
    fn firmware(&self) -> Option<&str> {
        self.log.firmware.as_deref()
    }

    /// The flight's number in the logger's memory, as the file states it, or ``None``.
    #[getter]
    fn flight_number(&self) -> Option<u32> {
        self.log.flight_number
    }

    /// The primary track's times, s, from the logger's zero, increasing. Empty for a log with no
    /// primary track, such as a table read by a mapping that marks none.
    #[getter]
    fn time_s<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        array(py, &self.log.time_s)
    }

    /// The primary track, the barometric altitude above the logger's zero (for a mapped table,
    /// the pad), m, on ``time_s``; NaN at a gap.
    #[getter]
    fn altitude_m<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        array(py, &self.log.altitude_m)
    }

    /// Every column the file holds, in its order, the primary track's among them. Each is a dict
    /// of:
    ///
    /// - ``column``, its name.
    /// - ``quantity``, what it measures.
    /// - ``unit``.
    /// - ``time_s`` and ``values``, NumPy arrays, NaN at a gap.
    #[getter]
    fn channels<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyDict>>> {
        self.log
            .channels
            .iter()
            .map(|channel| {
                let dict = PyDict::new(py);
                dict.set_item("column", &channel.column)?;
                dict.set_item("quantity", to_python(py, &channel.quantity)?)?;
                dict.set_item("unit", channel.quantity.unit())?;
                dict.set_item("time_s", array(py, &channel.time_s))?;
                dict.set_item("values", array(py, &channel.values))?;
                Ok(dict)
            })
            .collect()
    }

    /// What the logger says happened and when: each a dict of its ``time_s`` and ``name``.
    #[getter]
    fn events<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.log.events)
    }

    /// What the file states about the flight, such as the logger's own apogee, as a dict.
    #[getter]
    fn stated<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.log.stated)
    }

    /// What the reader noticed and worked around, such as a column kept as written.
    #[getter]
    fn notes(&self) -> Vec<String> {
        self.log.notes.clone()
    }

    /// The readings taken from the log on its own, as ``hpr analyze`` takes them, as a dict. It
    /// holds ``liftoff``, ``apogee``, ``max_speed``, ``max_acceleration``, ``burnout``,
    /// ``landing``, ``flight_time``, ``drogue_descent`` and ``main_descent``. Each has one of:
    ///
    /// - ``status`` ``"read"``, with its value and the bound its error stays within. It gives the
    ///   track it came from (``source``), whether it was measured, derived or clipped
    ///   (``provenance``), and how it was read (``method``).
    /// - ``status`` ``"withheld"``, with a ``reason`` and its ``detail``.
    ///
    /// The keys of a value and its bound name their unit. ``liftoff``, ``burnout``, ``landing`` and
    /// ``flight_time`` hold ``time_s`` and ``bound_s``. ``apogee`` holds ``height_m`` above the
    /// pad and ``altitude_m`` above the track's own zero, with ``bound_m``, ``time_s`` and
    /// ``time_bound_s``. On a short pad, a log that holds little or none of the pad, ``height_m``
    /// is above the track's zero too. ``max_speed`` holds ``speed_m_s`` and ``bound_m_s``, and
    /// ``max_acceleration`` holds ``acceleration_m_s2`` and ``bound_m_s2``. Each descent holds
    /// ``rate_m_s`` and ``bound_m_s``, and ``landing`` the mean rate from apogee,
    /// ``mean_descent_rate_m_s`` and ``mean_descent_rate_bound_m_s``.
    ///
    /// Twelve readings state a 90% interval, ``low`` and ``high``: ``liftoff``, ``burnout``,
    /// ``landing``, ``flight_time`` and the main's ``opening`` as ``interval_s``, ``max_speed`` and
    /// each descent as ``interval_m_s``, ``max_acceleration`` as ``interval_m_s2``, ``landing``'s
    /// mean rate as ``mean_descent_rate_interval_m_s``, and ``apogee``'s height and time as
    /// ``height_interval_m`` and ``time_interval_s``. On a floor the apogee's, the top speed's, the
    /// burnout's and the top acceleration's are ``None``; a burnout floor then states ``lower_end_s``, and a clipped
    /// top acceleration ``lower_end_m_s2``.
    ///
    /// An apogee, a top speed, a burnout and a top acceleration each say whether it's a floor
    /// (``is_floor``). A floor's true value is at least the value less its bound, and may be higher by any amount.
    /// A top acceleration also says why it is clipped (``clip``): ``"range"``, ``"flat"`` or
    /// ``None``.
    ///
    /// Beside the readings, the dict says how they were taken: the track's ``source``,
    /// ``sample_interval_s`` and ``step_m``, and the running median's ``median_window_s`` and
    /// ``peak_bound_m``. It gives ``pad_altitude_m`` too, and the axial ``accelerometer`` the top
    /// acceleration and burnout came from, or ``None``.
    ///
    /// ``day_air`` compares the log's temperature at the pad with the standard atmosphere's at the
    /// pad's height above sea level, or is ``None`` where the log has no temperature, no such
    /// height, a temperature outside −100 to 100 °C, a column of one value at a unit's zero (0 K,
    /// 0 °C or 0 °F: no sensor), or the pad isn't found. ``offset_k`` is the difference,
    /// ``covered`` whether the bounds' 15 K allowance covers it, and ``share`` how much the day's
    /// air may move a barometric height, speed or rate.
    fn readings<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &readings::read(&self.log))
    }

    /// The flight smoothed over every sensor, as ``hpr analyze`` smooths it, as a dict. Its
    /// ``status`` is ``"read"`` or ``"withheld"``; a log not smoothed holds a ``reason`` and its
    /// ``detail``.
    ///
    /// A log smoothed holds a track, one entry for every sample of every sensor: ``time_s`` on the
    /// log's clock, ``height_m``, ``height_sd_m``, ``speed_m_s`` and ``speed_sd_m_s``, a value and
    /// its standard error, its 90% interval ± 1.645 of them. It holds ``liftoff`` and
    /// ``apogee_time_s``. ``health`` is the health card's figures and ``card`` the same as text.
    /// ``health["assumed_pad"]`` says the log held too little of the pad, as one written from
    /// launch does, and the track then stands on the logger's zero.
    /// ``health["apogee_past_reading_bound"]`` says the smoothed apogee passes the reading's bound:
    /// go by the reading. ``apogee_is_floor`` says the log ends on the climb, so the rocket may have
    /// gone higher.
    fn smoothed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let outcome = match smoother::smooth(&self.log) {
            Ok(smoothed) => SmoothedOutcome::Read {
                card: smoothed.health.card(),
                smoothed: Box::new(smoothed),
            },
            Err(withheld) => SmoothedOutcome::Withheld(withheld),
        };
        to_python(py, &outcome)
    }

    /// Fits ``rocket``'s simulation to this log by least squares, as ``hpr fit`` does, and gives
    /// the fit as a dict. The simulation's drag, its motor's impulse and burn time, a constant
    /// wind and the structure's mass are moved until its flight, read as the log's sensor reads
    /// it, passes through the log's heights from a second before liftoff to apogee. A log with
    /// latitude and longitude is fitted through its drift too.
    ///
    /// ``fit(rocket, environment, rail_length_m, *, inclination_deg=90.0, heading_deg=0.0,
    /// tip_off=True, height_reads=None)``: the launch is given as ``Flight`` takes it. The
    /// environment's air is flown as given. Its wind, averaged over the climb, is what the fit is
    /// told of the wind before the log. A log with no latitude and longitude can't tell the wind,
    /// so on one the fit is told a wind of at least 5.16 m/s, the wind its intervals were sized
    /// in: a slower wind is raised to that along its own direction, and calm air to that toward
    /// the east. ``height_reads`` is ``"barometer"`` or ``"height_climbed"``; with ``None`` the
    /// log's main altitude is taken as a barometer's.
    ///
    /// ``parameters`` lists six dicts, in the fit's order: ``drag_scale``, ``impulse_scale``,
    /// ``burn_time_scale``, ``wind_east_m_s``, ``wind_north_m_s`` and ``mass_scale``, named by
    /// ``parameter``. Each holds its ``value``, a factor or for the wind a speed in m/s, its
    /// ``standard_error``, and what the fit was told of it, ``stated`` (``mean`` and ``sd``).
    /// ``interval_90`` is its least-squares 90% interval, ``low`` and ``high``, or ``None`` with
    /// ``no_interval`` saying why: ``"not_asked"`` for the burn time's scale,
    /// ``"height_alone"`` for the wind on a log with no latitude and longitude, and
    /// ``"no_standard_error"`` where the fit gave it no standard error.
    ///
    /// ``tested_range`` is the range the intervals were tested over, ``low`` and ``high``: what
    /// the synthetic flights drew that parameter's true value from, and for a wind component the
    /// fastest wind drawn, either way. ``outside_tested_range`` says the fitted value lies
    /// outside it, or for the wind that the fitted wind is faster. No test covers the fit's
    /// intervals out there, and ``text`` marks the value with a ``†``.
    ///
    /// ``caveat`` is ``None`` for an interval that passes its test on the synthetic flights. The
    /// drag scale's doesn't: on flights flown with a model unlike the fit's own, ``below`` of
    /// ``flights`` truths lay below it where ``meant`` are meant. Its ``message`` says which way
    /// that errs: the fitted drag may read high, and a flight predicted from it low.
    ///
    /// ``readings`` says what was read from the log and where each came from: ``liftoff`` (or
    /// ``None`` for a log that starts in flight), ``apogee``, ``height_track`` and
    /// ``horizontal_track`` (or ``None``). Each gives its ``source`` and its ``provenance``,
    /// measured or derived. The tracks give their quarter-second ``bins`` and ``bins_read``.
    ///
    /// ``wind_caveat`` is ``None`` for a log of height alone. On a log with latitude and
    /// longitude it is a sentence to read with the wind: the fitted wind can settle on another
    /// wind that leaves much the same drift, with a narrow interval about it, so give the day's
    /// wind.
    ///
    /// ``issue_warnings`` lists the known issues the flight at the fitted parameters meets, as
    /// ``Flight.issue_warnings`` gives them: where HPR Sim's drag or stability is known to read
    /// wrong on this flight, the fitted scales take that error in.
    ///
    /// The dict also holds ``height_read_as``, in ``height_reads``' two words, the residual's
    /// ``rms_m`` and ``misfit_share``, ``converged``, ``clock_offset_s``, ``height_offset_m``,
    /// ``flights_flown``, ``stated_apogee_m``, ``fitted_apogee_m``, ``fitted_apogee_time_s``
    /// (seconds after ignition) and ``fitted_max_mach``. The two apogees are the climb's as the
    /// fit flies it, with no recovery device opening. ``text`` is the whole fit as lines of text
    /// to print.
    ///
    /// The intervals were tested on synthetic flights only. A scale is the factor that best fits
    /// this one flight, not the rocket's drag at every speed.
    ///
    /// Raises ``HprError`` for a log with no apogee or too little of its climb, a flight that
    /// separates, a launch that can't be flown, and a log no flight of the design passes through.
    #[pyo3(signature = (
        rocket, environment, rail_length_m, *, inclination_deg = 90.0, heading_deg = 0.0,
        tip_off = true, height_reads = None
    ))]
    #[expect(
        clippy::too_many_arguments,
        reason = "Python's keyword arguments, one per option of a launch"
    )]
    fn fit<'py>(
        &self,
        py: Python<'py>,
        rocket: &Bound<'py, Rocket>,
        environment: &Environment,
        rail_length_m: f64,
        inclination_deg: f64,
        heading_deg: f64,
        tip_off: bool,
        height_reads: Option<&str>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let height_read_as = match height_reads {
            None => None,
            Some("barometer") => Some(HeightKind::Barometric),
            Some("height_climbed") => Some(HeightKind::Geometric),
            Some(other) => {
                return Err(error(format!(
                    "height_reads is {other:?}, and must be \"barometer\", \"height_climbed\" \
                     or None"
                )));
            }
        };
        // A copy, so the rocket isn't held borrowed while the fit flies without the GIL.
        let rocket = rocket.borrow().rocket.clone();
        let raised = Raised::default();
        let environment = environment.bound(py, &raised);
        let builder = hpr::Flight::builder(&rocket, &environment, rail_length_m)
            .inclination_deg(inclination_deg)
            .heading_deg(heading_deg)
            .tip_off(tip_off);
        let fitted = py.detach(|| hpr::fit::fit(&builder, &self.log, height_read_as));
        // A wind function's exception is raised in place of the fit's refusal.
        if let Some(raised) = raised.take() {
            return Err(raised);
        }
        let fitted = fitted.map_err(error)?;
        let given = to_python(py, &fitted)?;
        let dict = given.cast::<PyDict>()?;
        // Each caveat with its sentences, each warning as `Flight.issue_warnings` gives it.
        if let Some(parameters) = dict.get_item("parameters")? {
            for (row, fitted) in parameters.cast::<PyList>()?.iter().zip(&fitted.parameters) {
                if let (Some(caveat), Some(said)) =
                    (fitted.caveat, row.cast::<PyDict>()?.get_item("caveat")?)
                {
                    said.cast::<PyDict>()?
                        .set_item("message", caveat.message(fitted.parameter))?;
                }
            }
        }
        let warnings = PyList::empty(py);
        for warning in &fitted.issue_warnings {
            let said = PyDict::new(py);
            said.set_item("issue", warning.number())?;
            said.set_item("kind", warning.issue.kind().name())?;
            said.set_item("url", warning.issue.url())?;
            said.set_item("max_mach", warning.max_mach.value)?;
            said.set_item("time_s", warning.max_mach.time_s)?;
            said.set_item("parts", warning.parts.clone())?;
            said.set_item("message", warning.message())?;
            warnings.append(said)?;
        }
        dict.set_item("issue_warnings", warnings)?;
        dict.set_item(
            "height_read_as",
            match fitted.height_read_as {
                HeightKind::Barometric => "barometer",
                HeightKind::Geometric => "height_climbed",
            },
        )?;
        dict.set_item("wind_caveat", fitted.wind_caveat())?;
        dict.set_item("text", fitted.lines().join("\n"))?;
        Ok(given)
    }

    /// The whole record as a dict of lists, gaps as ``None``: the document ``hpr analyze
    /// --record`` prints, without its ``tool``.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.log)
    }

    fn __repr__(&self) -> String {
        // The format as ``format`` names it: a table read by a mapping is a `generic_table`, as
        // one read by its header is, so the format's words would name the wrong way it was read.
        let format = serde_json::to_value(self.log.format)
            .ok()
            .and_then(|name| name.as_str().map(str::to_owned))
            .unwrap_or_default();
        format!(
            "FlightLog({format}, {} channels, {} samples on the primary track)",
            self.log.channels.len(),
            self.log.time_s.len()
        )
    }
}
