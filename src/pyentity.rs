use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Entity")]
pub struct PyEntity {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub label: String,
}

impl From<quickner::Entity> for PyEntity {
    fn from(entity: quickner::Entity) -> Self {
        PyEntity {
            name: entity.name,
            label: entity.label,
        }
    }
}

impl From<PyEntity> for quickner::Entity {
    fn from(entity: PyEntity) -> Self {
        quickner::Entity {
            name: entity.name,
            label: entity.label,
        }
    }
}

impl FromIterator<PyEntity> for Vec<quickner::Entity> {
    fn from_iter<T: IntoIterator<Item = PyEntity>>(iter: T) -> Self {
        let mut entities = Vec::new();
        for entity in iter {
            entities.push(quickner::Entity::from(entity));
        }
        entities
    }
}

#[pymethods]
impl PyEntity {
    #[new]
    #[pyo3(signature = (name, label))]
    pub fn new(name: &str, label: &str) -> Self {
        PyEntity {
            name: name.to_string(),
            label: label.to_string(),
        }
    }

    // Pretty print the entity
    // Example: Entity(name="Apple", label="ORG")
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Entity(name=\"{}\", label=\"{}\")",
            self.name, self.label
        ))
    }
}
