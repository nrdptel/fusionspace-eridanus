//! `FlightLog`: a flight log read into one record, by its format's own reader or by the column
//! mapping or record layout a flyer writes for their own logger, as `hpr analyze` reads it
//! (`hpr_flightdata::custom`, M7.1d2).

use std::path::{Path, PathBuf};

use hpr::hpr_flightdata::custom;
use hpr::hpr_flightdata::error::LogError;
use hpr::hpr_flightdata::import;
use hpr::hpr_flightdata::log::FlightLog as Record;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{error, to_python};

/// A flight log, read: a clock, every channel the logger recorded on it in SI units (degrees for
/// angles), and what the file states about itself, as ``hpr analyze --record`` prints it. Read
/// one with ``FlightLog.read``.
#[pyclass(module = "fusionspace.hpr", frozen, skip_from_py_object)]
#[derive(Debug)]
pub struct FlightLog {
    pub(crate) log: Record,
}

/// The file's bytes, or an error naming it.
fn read_bytes(path: &Path) -> PyResult<Vec<u8>> {
    std::fs::read(path).map_err(|e| error(format!("{}: {e}", path.display())))
}

/// A mapping's or a layout's JSON: a dict's, or a file's at a path.
fn description_json(py: Python<'_>, given: &Bound<'_, PyAny>) -> PyResult<String> {
    if given.is_instance_of::<PyDict>() {
        return py
            .import("json")?
            .call_method1("dumps", (given,))?
            .extract::<String>();
    }
    let path: PathBuf = given
        .extract()
        .map_err(|_| error("a mapping or a layout is a dict, or the path of its JSON file"))?;
    std::fs::read_to_string(&path).map_err(|e| error(format!("{}: {e}", path.display())))
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
    /// Reads the flight log at ``path``: by its format's own reader, or by a column mapping, a
    /// table's, or a record layout, binary records' or a serial console's capture of them. A
    /// mapping or a layout is a dict, or the path of its JSON file, as the schemas
    /// ``schema/cli/analyze-mapping.schema.json`` and ``analyze-layout.schema.json`` describe;
    /// ``FlightLog.header_mapping`` gives the mapping a table's header states. ``sheet`` names the
    /// ``.xlsx`` workbook's sheet a mapped table is on, from 0. Raises ``HprError`` naming the field
    /// a mapping or layout fails, or why the log doesn't read.
    #[staticmethod]
    #[pyo3(signature = (path, *, mapping = None, layout = None, sheet = None))]
    fn read(
        py: Python<'_>,
        path: PathBuf,
        mapping: Option<&Bound<'_, PyAny>>,
        layout: Option<&Bound<'_, PyAny>>,
        sheet: Option<usize>,
    ) -> PyResult<Self> {
        if sheet.is_some() && mapping.is_none() {
            return Err(error(
                "sheet names the workbook's sheet a table is on, and no table is read by a mapping",
            ));
        }
        let log = match (mapping, layout) {
            (Some(_), Some(_)) => {
                return Err(error("a log is read by a mapping or by a layout, not both"));
            }
            (Some(mapping), None) => {
                let mapping =
                    custom::mapping_from_json(&description_json(py, mapping)?).map_err(error)?;
                custom::read_by_mapping(&read_bytes(&path)?, &mapping, sheet)
            }
            (None, Some(layout)) => {
                let layout =
                    custom::layout_from_json(&description_json(py, layout)?).map_err(error)?;
                custom::read_by_layout(&read_bytes(&path)?, &layout)
            }
            (None, None) => import::read(&read_bytes(&path)?),
        };
        Ok(Self {
            log: log.map_err(|refused| on(&path, refused))?,
        })
    }

    /// What the header of the table at ``path`` states, as a column mapping, a dict, to edit and
    /// give back to ``FlightLog.read`` as ``mapping``: set each column's ``quantity`` and
    /// ``unit``, and ``primary`` to the column, from 0, that is the barometric altitude above
    /// the pad. ``sheet`` names an ``.xlsx`` workbook's sheet, from 0.
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

    /// The primary track's times, s, from the logger's zero, increasing: empty for a log with no
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

    /// Every column the file holds, in its order, the primary track's among them: each a dict of
    /// its ``column`` name, the ``quantity`` it measures, its ``unit``, and NumPy arrays of its
    /// ``time_s`` and ``values``, NaN at a gap.
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

    /// The whole record as a dict of lists, gaps as ``None``: the document ``hpr analyze
    /// --record`` prints, without its ``tool``.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        to_python(py, &self.log)
    }

    fn __repr__(&self) -> String {
        format!(
            "FlightLog({}, {} channels, {} samples on the primary track)",
            self.log.format.name(),
            self.log.channels.len(),
            self.log.time_s.len()
        )
    }
}
