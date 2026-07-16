#[path = "pageindex_python.rs"]
mod pageindex_python;

#[path = "cache_python.rs"]
mod cache_python;

use pyo3::prelude::*;
use pyo3::types::PyAny;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

pub(crate) fn py_to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    depythonize(obj).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn value_to_py(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    Ok(pythonize(py, value)?.unbind())
}

#[pyfunction(name = "get_version")]
const fn get_version_py() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version_py, m)?)?;
    pageindex_python::register(m)?;
    cache_python::register(m)?;
    Ok(())
}
