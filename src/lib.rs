use fmt::Display;
use std::fmt;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::PathBuf;
use ::shr_parser::{SHRParser, SHRParsingType};

/// A wrapper around the SHRParser for Python
#[pyclass(name="SHRParser", subclass)]
struct PySHRParser {
    parser: SHRParser,
    parsing_type: PySHRParsingType,
}

#[pyclass(name="SHRSweep")]
struct PySHRSweep {
    sweep_number: i32,
    timestamp: u64,
    frequency: f64,
    amplitude: f64,
}

#[pyclass(name="SHRParsingType", eq, eq_int)]
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

#[pymethods]
impl PySHRSweep {
    fn __repr__(&self) -> String {
        format!(
            "SHRSweep(sweep_number={}, timestamp={}, frequency={}, amplitude={})",
            self.sweep_number, self.timestamp, self.frequency, self.amplitude
        )
    }

    #[getter]
    fn sweep_number(&self) -> i32 {
        self.sweep_number
    }

    #[getter]
    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.frequency
    }

    #[getter]
    fn amplitude(&self) -> f64 {
        self.amplitude
    }
}

#[pymethods]
impl PySHRParser {
    #[new]
    fn new(file_path: &str, parsing_type: PySHRParsingType) -> PyResult<Self> {
        let parser = SHRParser::new(PathBuf::from(file_path), SHRParsingType::try_from(parsing_type).unwrap()).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to parse SHR file: {:?}", e))
        })?;
        Ok(PySHRParser { parser, parsing_type })
    }

    fn __repr__(&self) -> String {
        format!("SHRParser(file_path='{}', parsing_type={})", self.parser.get_file_path().to_string_lossy(), self.parsing_type)
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
                sweep_number: sweep.sweep_number,
                timestamp: sweep.timestamp,
                frequency: sweep.frequency,
                amplitude: sweep.amplitude,
            })
            .collect())
    }

    fn get_file_header(&self) -> PyResult<String> {
        let header = self.parser.get_file_header();
        Ok(format!("{:?}", header))
    }

    fn get_file_path(&self) -> PyResult<String> {
        Ok(self.parser.get_file_path().to_string_lossy().to_string())
    }
}


/// Create a new SHRParser instance.
#[pyfunction]
fn create_parser(file_path: &str, parsing_type: i32) -> PyResult<PySHRParser> {
    PySHRParser::new(file_path, parsing_type.try_into().unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
 fn shr_parser(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PySHRParser>()?;
    module.add_class::<PySHRSweep>()?;
    module.add_class::<PySHRParsingType>()?;
    module.add_function(wrap_pyfunction!(create_parser, module)?)?;
    Ok(())
}
