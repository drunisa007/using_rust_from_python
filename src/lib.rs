use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// #[pyfunction]
// fn sum_as_minor(a:usize,b:usize) -> PyResult<String> {
//     Ok((a+b).to_string())
// }

#[pymodule]
fn using_rust_from_python(_py:Python,m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(sum_as_string,m)?)?;
    Ok(())
}