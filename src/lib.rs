use pyo3::prelude::*;
use numpy::{PyReadonlyArray1, IntoPyArray};

/// PythonのNumPy配列の合計を文字列で返す
#[pyfunction]
fn sum_as_string(array: PyReadonlyArray1<f64>) -> PyResult<String> {
    let sum = array.as_array().sum();
    Ok(sum.to_string())
}

#[pymodule]
fn maturin_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
