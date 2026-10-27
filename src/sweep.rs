use pyo3::prelude::*;
use pyo3::types::PyList;
use shr_parser::SHRSweep;

use crate::parser::PySHRParser;

/// A Python wrapper for a sweep.
#[pyclass(name = "SHRSweep")]
pub(crate) struct PySHRSweep {
    parser: Py<PySHRParser>,
    sweep_index: usize,
    #[pyo3(get)]
    sweep_number: i32,
    #[pyo3(get)]
    timestamp: u64,
    #[pyo3(get)]
    frequency: f64,
    #[pyo3(get)]
    amplitude: f64,
    #[pyo3(get)]
    latitude: f64,
    #[pyo3(get)]
    longitude: f64,
}

impl PySHRSweep {
    pub(crate) fn new(parser: Py<PySHRParser>, sweep_index: usize, sweep: &SHRSweep) -> Self {
        Self {
            parser,
            sweep_index,
            sweep_number: sweep.number,
            timestamp: sweep.header.timestamp,
            frequency: sweep.frequency,
            amplitude: sweep.amplitude,
            latitude: sweep.header.latitude,
            longitude: sweep.header.longitude,
        }
    }
}

#[pymethods]
impl PySHRSweep {
    /// Return every bin as a (frequency in Hz, unmodified amplitude) tuple.
    fn get_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let parser = self.parser.borrow(py);
        PyList::new(
            py,
            parser.parser.shr_file.sweeps[self.sweep_index]
                .get_data()
                .iter()
                .map(|point| (point.frequency_hz, point.amplitude)),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "SHRSweep(sweep_number={}, timestamp={}, frequency={}, amplitude={}, Latitude={}, Longitude={})",
            self.sweep_number, self.timestamp, self.frequency, self.amplitude, self.latitude, self.longitude
        )
    }
}
