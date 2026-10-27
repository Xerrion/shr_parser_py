mod parser;
mod parsing_type;
mod sweep;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use parser::{create_parser, PySHRParser};
use parsing_type::PySHRParsingType;
use sweep::PySHRSweep;

/// A Python module implemented in Rust.
#[pymodule(name = "shr_parser")]
fn shr_parser_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySHRParser>()?;
    m.add_class::<PySHRSweep>()?;
    m.add_class::<PySHRParsingType>()?;
    m.add_function(wrap_pyfunction!(create_parser, m)?)?;
    Ok(())
}
