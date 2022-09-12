
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn add_four_rust(x: f64) -> PyResult<f64>{
    return Ok(x + 4.0);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_functions_in_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_four_rust, m)?)?;
    Ok(())
}



