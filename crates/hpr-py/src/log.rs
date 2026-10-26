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
use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{error, to_python};

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
    /// pad (on a short pad, above the track's zero) and ``altitude_m`` above the track's own zero.
    /// It also holds ``bound_m``, ``time_s`` and ``time_bound_s``. ``max_speed`` holds ``speed_m_s`` and ``bound_m_s``, and
    /// ``max_acceleration`` holds ``acceleration_m_s2`` and ``bound_m_s2``. Each descent holds
    /// ``rate_m_s`` and ``bound_m_s``.
    ///
    /// An apogee, a burnout and a top acceleration each say whether they're a floor
    /// (``is_floor``). A top acceleration also says why it is clipped (``clip``): ``"range"``,
    /// ``"flat"`` or ``None``.
    ///
    /// Beside the readings, the dict says how they were taken: the track's ``source``,
    /// ``sample_interval_s`` and ``step_m``, and the running median's ``median_window_s`` and
    /// ``peak_bound_m``. It gives ``pad_altitude_m`` too, and the axial ``accelerometer`` the top
    /// acceleration and burnout came from, or ``None``.
    fn readings<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &readings::read(&self.log))
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
