use pyo3::prelude::*;
mod models;
mod utils;

/// Load data from JSONL and return a Quickner object
/// Parse the annotations and entities from the JSONL file
#[pyfunction]
fn from_jsonl(path: String) -> PyResult<models::PyQuickner> {
    let quick = models::PyQuickner::from_jsonl(Some(&path));
    Ok(quick)
}

/// Load data from Spacy JSON format and return a Quickner object
/// Parse the annotations and entities from the JSON file
#[pyfunction]
fn from_spacy(path: String) -> PyResult<models::PyQuickner> {
    let quick = models::PyQuickner::from_spacy(Some(&path));
    Ok(quick)
}

/// A Python module implemented in Rust.
#[pymodule]
fn quickner(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(from_jsonl))?;
    m.add_wrapped(wrap_pyfunction!(from_spacy))?;
    m.add_class::<models::PyQuickner>()?;
    m.add_class::<models::PyConfig>()?;
    m.add_class::<models::PyFormat>()?;
    m.add_class::<models::PyDocument>()?;
    m.add_class::<models::PyEntity>()?;
    Ok(())
}
