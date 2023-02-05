use pyo3::prelude::*;
mod models;
mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn quickner(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<models::PyQuickner>()?;
    m.add_class::<models::PyConfig>()?;
    m.add_class::<models::PyFormat>()?;
    m.add_class::<models::PyAnnotation>()?;
    m.add_class::<models::PyEntity>()?;
    Ok(())
}
