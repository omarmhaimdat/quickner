use pyo3::prelude::*;
use pyquickner::QuicknerError;
mod pyconfig;
mod pydocument;
mod pyentity;
mod pymodels;
mod pyquickner;
mod utils;

/// Load data from JSONL and return a Quickner object
/// Parse the annotations and entities from the JSONL file
#[pyfunction]
fn from_jsonl(path: String) -> PyResult<pyquickner::PyQuickner> {
    let quick = pyquickner::PyQuickner::from_jsonl(Some(&path));
    Ok(quick)
}

/// Load data from Spacy JSON format and return a Quickner object
/// Parse the annotations and entities from the JSON file
#[pyfunction]
fn from_spacy(path: String) -> PyResult<pyquickner::PyQuickner> {
    let quick = pyquickner::PyQuickner::from_spacy(Some(&path));
    Ok(quick)
}

/// A Python module implemented in Rust.
#[pymodule]
fn quickner(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(from_jsonl))?;
    m.add_wrapped(wrap_pyfunction!(from_spacy))?;
    m.add_class::<pyquickner::PyQuickner>()?;
    m.add_class::<pyconfig::PyConfig>()?;
    m.add_class::<pyconfig::PyFormat>()?;
    m.add_class::<pydocument::PyDocument>()?;
    m.add_class::<pyentity::PyEntity>()?;
    m.add("QuicknerError", _py.get_type::<QuicknerError>())?;
    Ok(())
}
