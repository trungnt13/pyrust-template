#[cfg(test)]
mod tests {
    use pyo3::prelude::*;
    use pyrust_py::sum_as_string;

    #[test]
    fn test_integration_pyrust_py() {
        Python::with_gil(|_py| {
            assert_eq!(sum_as_string(0, 0).unwrap(), "1");
        })
    }
}
