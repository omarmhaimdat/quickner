use std::collections::HashMap;

use crate::{
    pyconfig::{
        PyAnnotations, PyConfig, PyEntities, PyExcludes, PyFilters, PyFormat, PyInput, PyLogging,
        PyOutput, PyTexts,
    },
    pydocument::PyDocument,
    pyentity::PyEntity,
    utils::{colorize, TermColor},
};
use numpy::PyArray2;
use pyo3::create_exception;
use pyo3::{
    exceptions::{self, PyGeneratorExit},
    prelude::*,
    types::{PyDict, PyTuple},
};
use quickner::{Document, Entity, Quickner, SpacyEntity};

create_exception!(quickner, QuicknerError, exceptions::PyException);

#[pyclass(name = "Quickner")]
pub struct PyQuickner {
    #[pyo3(get)]
    pub config: PyConfig,
    #[pyo3(get)]
    pub config_path: String,
    #[pyo3(get)]
    pub documents: Vec<PyDocument>,
    #[pyo3(get)]
    pub entities: Vec<PyEntity>,
    quickner: Quickner,
}

#[pyclass(name = "SpacyEntity")]
pub struct PySpacyEntity {
    #[pyo3(get)]
    pub entity: Vec<(usize, usize, String)>,
}

pub type SpacyFormat = Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)>;

#[pyclass(name = "SpacyGenerator")]
pub struct PySpacyGenerator {
    #[pyo3(get)]
    pub entities: Vec<SpacyFormat>,
}

#[pymethods]
impl PySpacyGenerator {
    #[new]
    #[pyo3(signature = (entities))]
    fn new(entities: Vec<SpacyFormat>) -> Self {
        PySpacyGenerator { entities }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<SpacyFormat> {
        if slf.entities.is_empty() {
            PyGeneratorExit::new_err("No more entities");
            None
        } else {
            Some(slf.entities.remove(0))
        }
    }
}

impl From<SpacyEntity> for PySpacyEntity {
    fn from(entity: SpacyEntity) -> Self {
        PySpacyEntity {
            entity: entity.entity,
        }
    }
}

#[pymethods]
impl PyQuickner {
    // Quickner(config_path: Optional[str] = None)
    // Quickner(documents: List[Document])
    // Quickner(entities: List[Entity])

    #[new]
    #[pyo3(signature = (documents = None, entities = None, config = PyConfig::default()))]
    pub fn new(
        documents: Option<Vec<PyDocument>>,
        entities: Option<Vec<PyEntity>>,
        config: Option<PyConfig>,
    ) -> Self {
        let mut quickner = Quickner::new(None);
        match documents {
            Some(documents) => {
                quickner.documents = documents.into_iter().collect();
            }
            None => quickner.documents = Vec::new(),
        }
        match entities {
            Some(entities) => {
                quickner.entities = entities.into_iter().collect();
            }
            None => quickner.entities = Vec::new(),
        }
        let config = match config {
            Some(config) => config,
            None => PyConfig::default(),
        };
        quickner.config = PyConfig::to_config(config);
        PyQuickner::from(quickner)
    }

    #[setter(documents)]
    pub fn documents(&mut self, documents: Vec<PyDocument>) {
        self.documents = (*documents).to_vec();
        self.quickner.documents = documents.into_iter().collect();
        self.quickner.documents_hash = Quickner::document_hash(&self.quickner.documents);
        self.quickner.build_label_index();
        self.quickner.build_entity_index();
    }

    #[setter(entities)]
    pub fn entities(&mut self, entities: Vec<PyEntity>) {
        self.entities = (*entities).to_vec();
        self.quickner.entities = entities.into_iter().collect();
    }

    pub fn add_document(&mut self, document: PyDocument) {
        if self.documents.contains(&document) {
            return;
        }
        let documents = &mut self.documents;
        documents.push(document.clone());
        let document = Document::from(document);
        self.quickner.add_document(document);
    }

    pub fn add_entity(&mut self, entity: PyEntity) {
        // Check if the entity is already in the list
        if self.entities.contains(&entity) {
            return;
        }
        let entities = &mut self.entities;
        entities.push(entity.clone());
        let entity = Entity {
            name: entity.name,
            label: entity.label,
        };
        self.quickner.add_entity(entity);
    }

    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = String::new();
        repr.push_str(&colorize("Entities: ", TermColor::Yellow));
        repr.push_str(&format!("{} | ", self.entities.len()));
        repr.push_str(&colorize("Documents: ", TermColor::Green));
        repr.push_str(&format!("{} | ", self.documents.len()));
        repr.push_str(&colorize("Annotations: ", TermColor::Blue));

        let annotations_hash = std::collections::HashMap::new();
        let annotations_count =
            self.documents
                .clone()
                .into_iter()
                .fold(annotations_hash, |mut acc, document| {
                    for (_, _, label) in document.label {
                        let count = acc.entry(label).or_insert(0);
                        *count += 1;
                    }
                    acc
                });
        let annotations_count = annotations_count
            .into_iter()
            .map(|(label, count)| format!("{label}: {count}"))
            .collect::<Vec<String>>()
            .join(", ");

        repr.push_str(&annotations_count);
        Ok(repr)
    }

    #[pyo3(signature = (save = false))]
    pub fn process(&mut self, save: bool) -> PyResult<()> {
        let annotations: Result<(), _> = self.quickner.process(save);
        match annotations {
            Ok(annotations) => annotations,
            Err(error) => return Err(PyErr::new::<exceptions::PyException, _>(error.to_string())),
        };
        self.documents = self
            .quickner
            .documents
            .clone()
            .into_iter()
            .map(PyDocument::from)
            .collect::<Vec<PyDocument>>();
        self.entities = self
            .quickner
            .entities
            .clone()
            .into_iter()
            .map(PyEntity::from)
            .collect::<Vec<PyEntity>>();
        Ok(())
    }

    #[pyo3(signature = (path = None, format = PyFormat::JSONL))]
    pub fn save_annotations(&self, path: Option<&str>, format: PyFormat) -> PyResult<String> {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let format = match format {
            PyFormat::CSV => quickner::Format::Csv,
            PyFormat::JSONL => quickner::Format::Jsonl,
            PyFormat::SPACY => quickner::Format::Spacy,
            PyFormat::BRAT => quickner::Format::Brat,
            PyFormat::CONLL => quickner::Format::Conll,
        };
        let save_annotations = format.save(&self.quickner.documents, &path);
        match save_annotations {
            Ok(_) => Ok(save_annotations.unwrap()),
            Err(error) => Err(PyErr::new::<exceptions::PyException, _>(error.to_string())),
        }
    }

    #[pyo3(signature = (path = None))]
    #[staticmethod]
    pub fn from_jsonl(path: Option<&str>) -> PyQuickner {
        let path = match path {
            Some(path) => path.to_string(),
            None => String::from(""),
        };
        let quickner = Quickner::from_jsonl(path.as_str());
        PyQuickner::from(quickner)
    }

    #[pyo3(signature = (path = None))]
    #[staticmethod]
    pub fn from_spacy(path: Option<&str>) -> PyQuickner {
        let path = match path {
            Some(path) => path.to_string(),
            None => String::from(""),
        };
        let quickner = Quickner::from_spacy(path.as_str());
        PyQuickner::from(quickner)
    }

    #[pyo3(signature = (path = None))]
    pub fn to_jsonl(&self, path: Option<&str>) {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let documents: Vec<Document> = self
            .documents
            .iter()
            .map(|annotation| Document::new((*annotation.text).to_string(), (*annotation.label).to_vec()))
            .collect();
        quickner::Format::Jsonl
            .save(&documents, path.as_str())
            .unwrap();
    }

    #[pyo3(signature = (path = None))]
    pub fn to_csv(&self, path: Option<&str>) {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let documents: Vec<Document> = self
            .documents
            .iter()
            .map(|annotation| Document::new((*annotation.text).to_string(), (*annotation.label).to_vec()))
            .collect();
        quickner::Format::Csv
            .save(&documents, path.as_str())
            .unwrap();
    }

    #[pyo3(signature = (path = None))]
    pub fn to_spacy(&self, path: Option<&str>) {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let documents: Vec<Document> = self
            .documents
            .iter()
            .map(|annotation| Document::new((*annotation.text).to_string(), (*annotation.label).to_vec()))
            .collect();
        quickner::Format::Spacy
            .save(&documents, path.as_str())
            .unwrap();
    }

    #[pyo3(signature = (label))]
    pub fn find_documents_by_label(&self, label: &str) -> Vec<PyDocument> {
        let quickner = &self.quickner;
        let documents_index = quickner.documents_label_index.to_owned();
        let documents_ids = match documents_index.get(label) {
            Some(documents_ids) => documents_ids,
            None => return vec![],
        };
        let quickner = &self.quickner;
        let documents = {
            let documents: Vec<_> = documents_ids
                .iter()
                .map(|id| {
                    let document = quickner.documents_hash.get(id).unwrap();
                    PyDocument::from(document.to_owned())
                })
                .collect();
            documents
        };
        // Remove duplicates
        let documents = documents
            .into_iter()
            .fold(Vec::new(), |mut acc, document| {
                if !acc.contains(&document) {
                    acc.push(document);
                }
                acc
            });
        println!("{:?}", documents);
        documents
    }

    #[pyo3(signature = (name))]
    pub fn find_documents_by_entity(&self, name: &str) -> Vec<PyDocument> {
        let quickner = &self.quickner;
        let documents_entities_index = quickner.documents_entities_index.to_owned();
        let binding = name.to_lowercase();
        let name = binding.as_str();
        let documents_ids = match documents_entities_index.get(name) {
            Some(documents_ids) => documents_ids,
            None => return vec![],
        };
        let quickner = &self.quickner;
        let documents = {
            let documents: Vec<_> = documents_ids
                .iter()
                .map(|id| {
                    let document = quickner.documents_hash.get(id).unwrap();
                    PyDocument::from(document.to_owned())
                })
                .collect();
            documents
        };
        // Remove duplicates
        let documents: Vec<_> = documents
            .into_iter()
            .fold(Vec::new(), |mut acc, document| {
                if !acc.contains(&document) {
                    acc.push(document);
                }
                acc
            });
        documents
    }

    #[pyo3(signature = (chunks = None))]
    pub fn spacy(&self, chunks: Option<usize>) -> PySpacyGenerator {
        let spacy = self.quickner.spacy(chunks);

        let spacy = spacy
            .into_iter()
            .map(|chunk| {
                chunk
                    .into_iter()
                    .map(|(text, entity)| {
                        let mut map = HashMap::new();
                        map.insert("entitiy".to_string(), entity.entity);
                        (text, map)
                    })
                    .collect::<Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)>>()
            })
            .collect();
        PySpacyGenerator { entities: spacy }
    }

    /// Convert Vec<Document> to numpy array of (string, array of (int, int, string))
    /// where the first int is the start index and the second int is the end index
    /// of the entity in the string.
    /// The string is the text of the document.
    /// The array of (int, int, string) is the list of entities in the document.
    /// Return a numpy array like so: array(['rust is made by Mozilla', list([(0, 4, 'PL'), (16, 23, 'ORG')])], dtype=object)
    /// And type is numpy.ndarray
    // pub fn numpy(&self) -> Py<PyArray1<PyObject>> {
    //     Python::with_gil(|py| {
    //         let numpy = PyModule::import(py, "numpy").unwrap();
    //         let array = numpy.getattr("array").unwrap();
    //         let array = array.call1((self.documents.clone(),)).unwrap();
    //         array.extract().unwrap()
    //     })
    // }

    pub fn numpy(&self) -> PyResult<Py<PyArray2<PyObject>>> {
        Python::with_gil(|py| {
            let numpy = PyModule::import(py, "numpy").unwrap();
            let array = numpy.getattr("array").unwrap();
            let object: Vec<&PyTuple> = self
                .documents
                .iter()
                .map(|document| {
                    let entities: Vec<&PyTuple> = document
                        .label
                        .iter()
                        .map(|entity| {
                            PyTuple::new(
                                py,
                                &[
                                    entity.0.to_object(py),
                                    entity.1.to_object(py),
                                    entity.2.clone().to_object(py),
                                ],
                            )
                        })
                        .collect();
                    PyTuple::new(
                        py,
                        &[
                            document.id.clone().to_object(py),
                            document.text.clone().to_object(py),
                            entities.to_object(py),
                        ],
                    )
                })
                .collect::<Vec<&PyTuple>>();
            let args = PyDict::new(py);
            args.set_item("dtype", "object").unwrap();
            let array = array.call((object,), Some(args));
            if let Ok(array) = array {
                let array = array.extract::<Py<PyArray2<PyObject>>>();
                match array {
                    Ok(array) => Ok(array),
                    Err(array) => {
                        // Raise an exception QuicknerError
                        Err(PyErr::new::<QuicknerError, _>(array.to_string()))
                    }
                }
            } else {
                let array = array.unwrap_err();
                // Raise an exception QuicknerError
                Err(PyErr::new::<QuicknerError, _>(array.to_string()))
            }
        })
    }
}

impl From<Quickner> for PyQuickner {
    fn from(quickner: Quickner) -> Self {
        PyQuickner {
            quickner: quickner.clone(),
            config: PyConfig {
                texts: PyTexts {
                    input: PyInput {
                        path: quickner.config.texts.input.path,
                        filter: quickner.config.texts.input.filter,
                    },
                    filters: PyFilters {
                        alphanumeric: quickner.config.texts.filters.alphanumeric,
                        case_sensitive: quickner.config.texts.filters.case_sensitive,
                        min_length: quickner.config.texts.filters.min_length,
                        max_length: quickner.config.texts.filters.max_length,
                        punctuation: quickner.config.texts.filters.punctuation,
                        numbers: quickner.config.texts.filters.numbers,
                        special_characters: quickner.config.texts.filters.special_characters,
                        accept_special_characters: quickner
                            .config
                            .texts
                            .filters
                            .accept_special_characters,
                        list_of_special_characters: quickner
                            .config
                            .texts
                            .filters
                            .list_of_special_characters
                            .map(|list| list.into_iter().collect::<Vec<char>>()),
                    },
                },
                annotations: PyAnnotations {
                    output: PyOutput {
                        path: quickner.config.annotations.output.path,
                    },
                    format: match quickner.config.annotations.format {
                        quickner::Format::Csv => PyFormat::CSV,
                        quickner::Format::Jsonl => PyFormat::JSONL,
                        quickner::Format::Spacy => PyFormat::SPACY,
                        quickner::Format::Brat => PyFormat::BRAT,
                        quickner::Format::Conll => PyFormat::CONLL,
                    },
                },
                entities: PyEntities {
                    input: PyInput {
                        path: quickner.config.entities.input.path,
                        filter: quickner.config.entities.input.filter,
                    },
                    filters: PyFilters {
                        alphanumeric: quickner.config.entities.filters.alphanumeric,
                        case_sensitive: quickner.config.entities.filters.case_sensitive,
                        min_length: quickner.config.entities.filters.min_length,
                        max_length: quickner.config.entities.filters.max_length,
                        punctuation: quickner.config.entities.filters.punctuation,
                        numbers: quickner.config.entities.filters.numbers,
                        special_characters: quickner.config.entities.filters.special_characters,
                        accept_special_characters: quickner
                            .config
                            .entities
                            .filters
                            .accept_special_characters,
                        list_of_special_characters: quickner
                            .config
                            .entities
                            .filters
                            .list_of_special_characters
                            .map(|list| list.into_iter().collect::<Vec<char>>()),
                    },
                    excludes: PyExcludes {
                        path: quickner.config.entities.excludes.path,
                    },
                },
                logging: match quickner.config.logging {
                    Some(logging) => Some(PyLogging {
                        level: logging.level,
                    }),
                    None => None,
                },
            },
            config_path: quickner.config_file.unwrap_or("".to_string()),
            documents: quickner
                .documents
                .iter()
                .map(|annotation| {
                    PyDocument::new(annotation.text.as_str(), Some(annotation.label.clone()))
                })
                .collect(),
            entities: quickner
                .entities
                .iter()
                .map(|entity| PyEntity {
                    name: entity.name.clone(),
                    label: entity.label.clone(),
                })
                .collect(),
        }
    }
}
