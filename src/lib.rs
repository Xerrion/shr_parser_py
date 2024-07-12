use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::PathBuf;
use shr_parser::{SHRParser, SHRParsingType};

/// A wrapper around the SHRParser for Python
#[pyclass]
struct PySHRParser {
    parser: SHRParser,
}

#[pymethods]
impl PySHRParser {
    #[new]
    fn new(file_path: String, parsing_type: i32) -> PyResult<Self> {
        let parsing_type = SHRParsingType::try_from(parsing_type).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid parsing type: {}", e))
        })?;
        let parser = SHRParser::new(PathBuf::from(file_path), parsing_type).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to parse SHR file: {:?}", e))
        })?;
        Ok(PySHRParser { parser })
    }

    fn to_csv(&self, path: String) -> PyResult<()> {
        self.parser.to_csv(PathBuf::from(path)).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to write to CSV: {:?}", e))
        })
    }

    fn get_sweeps(&self) -> PyResult<Vec<(i32, u64, f64, f64)>> {
        let sweeps = self.parser.get_sweeps();
        Ok(sweeps
            .into_iter()
            .map(|sweep| (sweep.sweep_number, sweep.timestamp, sweep.frequency, sweep.amplitude))
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
fn create_parser(file_path: String, parsing_type: i32) -> PyResult<PySHRParser> {
    PySHRParser::new(file_path, parsing_type)
}

/// A Python module implemented in Rust.
#[pymodule]
 fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PySHRParser>()?;
    module.add_function(wrap_pyfunction!(create_parser, module)?)?;
    Ok(())
}
