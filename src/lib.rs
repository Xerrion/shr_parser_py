use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use shr_parser::{SHRParser, SHRParsingType};
use std::fmt::{self, Display};
use std::path::PathBuf;

/// An enum mirroring SHRParsingType for Python.
#[pyclass(name = "SHRParsingType", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum PySHRParsingType {
    PEAK = 0,
    MEAN = 1,
    LOW = 2,
}

impl TryFrom<i32> for PySHRParsingType {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PySHRParsingType::PEAK),
            1 => Ok(PySHRParsingType::MEAN),
            2 => Ok(PySHRParsingType::LOW),
            _ => Err("Invalid value for SHRParsingType"),
        }
    }
}

impl TryFrom<PySHRParsingType> for SHRParsingType {
    type Error = &'static str;
    fn try_from(value: PySHRParsingType) -> Result<Self, Self::Error> {
        match value {
            PySHRParsingType::PEAK => Ok(SHRParsingType::Peak),
            PySHRParsingType::MEAN => Ok(SHRParsingType::Mean),
            PySHRParsingType::LOW => Ok(SHRParsingType::Low),
        }
    }
}

impl Display for PySHRParsingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PySHRParsingType::PEAK => write!(f, "SHRParsingType.PEAK"),
            PySHRParsingType::MEAN => write!(f, "SHRParsingType.MEAN"),
            PySHRParsingType::LOW => write!(f, "SHRParsingType.LOW"),
        }
    }
}

/// A Python wrapper for a sweep.
#[pyclass(name = "SHRSweep")]
struct PySHRSweep {
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

#[pymethods]
impl PySHRSweep {
    fn __repr__(&self) -> String {
        format!(
            "SHRSweep(sweep_number={}, timestamp={}, frequency={}, amplitude={}, Latitude={}, Longitude={})",
            self.sweep_number, self.timestamp, self.frequency, self.amplitude, self.latitude, self.longitude
        )
    }
}

/// A Python wrapper around SHRParser.
#[pyclass(name = "SHRParser", subclass)]
struct PySHRParser {
    parser: SHRParser,
    parsing_type: PySHRParsingType,
}

#[pymethods]
impl PySHRParser {
    #[new]
    fn new(file_path: &str, parsing_type: PySHRParsingType) -> PyResult<Self> {
        let shr_parsing = SHRParsingType::try_from(parsing_type)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let parser = SHRParser::new(PathBuf::from(file_path), shr_parsing).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to parse SHR file: {:?}", e))
        })?;
        Ok(PySHRParser {
            parser,
            parsing_type,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "SHRParser(file_path='{}', parsing_type={})",
            self.parser.get_file_path().to_string_lossy(),
            self.parsing_type
        )
    }

    fn to_csv(&self, path: String) -> PyResult<()> {
        self.parser.to_csv(PathBuf::from(path)).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to write to CSV: {:?}", e))
        })
    }

    fn get_sweeps(&self) -> PyResult<Vec<PySHRSweep>> {
        let sweeps = self.parser.get_sweeps();
        Ok(sweeps
            .into_iter()
            .map(|sweep| PySHRSweep {
                sweep_number: sweep.number,
                timestamp: sweep.header.timestamp,
                frequency: sweep.frequency,
                amplitude: sweep.amplitude,
                latitude: sweep.header.latitude,
                longitude: sweep.header.longitude,
            })
            .collect())
    }

    fn get_file_header(&self) -> PyResult<String> {
        let header = self.parser.get_file_header();
        Ok(format!("{:?}", header))
    }

    fn get_file_path(&self) -> PyResult<String> {
        Ok(self.parser.get_file_path().to_string_lossy().into_owned())
    }
}

/// Create a new SHRParser instance.
#[pyfunction]
fn create_parser(file_path: &str, parsing_type: PySHRParsingType) -> PyResult<PySHRParser> {
    PySHRParser::new(file_path, parsing_type)
}

/// A Python module implemented in Rust.
#[pymodule(name = "shr_parser")]
fn shr_parser_py(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PySHRParser>()?;
    m.add_class::<PySHRSweep>()?;
    m.add_class::<PySHRParsingType>()?;
    m.add_function(wrap_pyfunction!(create_parser, m)?)?;
    Ok(())
}
