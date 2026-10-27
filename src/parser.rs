use std::path::PathBuf;

use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use shr_parser::SHRParser;

use crate::parsing_type::PySHRParsingType;
use crate::sweep::PySHRSweep;

/// A Python wrapper around SHRParser.
#[pyclass(name = "SHRParser", subclass)]
pub(crate) struct PySHRParser {
    pub(crate) parser: SHRParser,
    parsing_type: PySHRParsingType,
}

#[pymethods]
impl PySHRParser {
    #[new]
    fn new(file_path: &str, parsing_type: PySHRParsingType) -> PyResult<Self> {
        let parser =
            SHRParser::new(PathBuf::from(file_path), parsing_type.into()).map_err(|error| {
                PyIOError::new_err(format!("Failed to parse SHR file: {:?}", error))
            })?;
        Ok(Self {
            parser,
            parsing_type,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "SHRParser(file_path='{}', parsing_type={})",
            self.parser.file_path.to_string_lossy(),
            self.parsing_type
        )
    }

    fn to_csv(&self, path: String) -> PyResult<()> {
        self.parser
            .to_csv(PathBuf::from(path))
            .map_err(|error| PyIOError::new_err(format!("Failed to write to CSV: {:?}", error)))
    }

    fn get_sweeps(slf: &Bound<'_, Self>) -> Vec<PySHRSweep> {
        slf.borrow()
            .parser
            .shr_file
            .sweeps
            .iter()
            .enumerate()
            .map(|(index, sweep)| PySHRSweep::new(slf.clone().unbind(), index, sweep))
            .collect()
    }

    fn get_file_header(&self) -> String {
        format!("{:?}", self.parser.shr_file.file_header)
    }

    fn get_file_path(&self) -> String {
        self.parser.file_path.to_string_lossy().into_owned()
    }
}

/// Create a new SHRParser instance.
#[pyfunction]
pub(crate) fn create_parser(
    file_path: &str,
    parsing_type: PySHRParsingType,
) -> PyResult<PySHRParser> {
    PySHRParser::new(file_path, parsing_type)
}
