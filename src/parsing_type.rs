use std::fmt::{self, Display};

use pyo3::prelude::*;
use shr_parser::SHRParsingType;

/// An enum mirroring SHRParsingType for Python.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(name = "SHRParsingType", eq, eq_int, from_py_object)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PySHRParsingType {
    PEAK = 0,
    MEAN = 1,
    LOW = 2,
}

impl From<PySHRParsingType> for SHRParsingType {
    fn from(value: PySHRParsingType) -> Self {
        match value {
            PySHRParsingType::PEAK => Self::Peak,
            PySHRParsingType::MEAN => Self::Mean,
            PySHRParsingType::LOW => Self::Low,
        }
    }
}

impl Display for PySHRParsingType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PEAK => "SHRParsingType.PEAK",
            Self::MEAN => "SHRParsingType.MEAN",
            Self::LOW => "SHRParsingType.LOW",
        })
    }
}
