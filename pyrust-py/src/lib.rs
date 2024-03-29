use pyo3::prelude::*;
use pyrust_lib::add_one;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    let result = add_one(a, b);
    Ok(result.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrust_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_pyrust_py() {
        // pyo3::prepare_freethreaded_python(); // required without `pyo3/auto-initialize` feature
        Python::with_gil(|_py| {
            assert_eq!(sum_as_string(2, 3).unwrap(), "6");
            assert_eq!(sum_as_string(0, 0).unwrap(), "1");
            assert_eq!(sum_as_string(10, 5).unwrap(), "16");
        })
    }
}
