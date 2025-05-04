use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: &[usize]) -> PyResult<String> {
    let sum = 0;
    for v in a {
        sum += v;
    }
    Ok(sum.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
