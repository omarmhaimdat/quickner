use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
/// Transform Rust code into Python code
/// Create a Python Class version of the Rust struct
/// from the quickner_cli crate

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Text")]
pub struct PyText {
    #[pyo3(get)]
    pub text: String,
}
