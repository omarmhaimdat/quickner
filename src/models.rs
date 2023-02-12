use pyo3::{exceptions, prelude::*};
use std::fmt::{Display, Formatter};

use crate::utils::{colorize, TermColor};
use quickner::{Document, Entity, Quickner};
use serde::{Deserialize, Serialize};
/// Transform Rust code into Python code
/// Create a Python Class version of the Rust struct
/// from the quickner_cli crate

#[derive(Clone)]
#[pyclass(name = "Quickner")]
pub struct PyQuickner {
    #[pyo3(get)]
    pub config: PyConfig,
    #[pyo3(get)]
    pub config_path: String,
    #[pyo3(get)]
    pub documents: Option<Vec<PyDocument>>,
    #[pyo3(get)]
    pub entities: Option<Vec<PyEntity>>,
    quickner: Option<Quickner>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Config")]
pub struct PyConfig {
    #[pyo3(get)]
    pub texts: PyTexts,
    #[pyo3(get)]
    pub annotations: PyAnnotations,
    #[pyo3(get)]
    pub entities: PyEntities,
    #[pyo3(get)]
    pub logging: Option<PyLogging>,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn new(config: PyConfig) -> Self {
        config
    }

    // Pretty print the config
    fn __repr__(&self) -> PyResult<String> {
        let mut output = String::new();
        output.push_str(&format!(
            "
        {}:
        ---------------------------
        {}
            Texts input path: {}
            Texts filters: 
                {}
        {}
            Annotations output path: {}
            Annotations format: {}
        {}
            Entities input path: {}
            Entities filters: 
                {}
            Entities excludes: {}
        {}
            Logging level: {}
        ",
            colorize("Configuration file summary", TermColor::Blue),
            colorize("TEXTS", TermColor::Green),
            self.texts.input.path,
            self.texts.filters,
            colorize("ANNOTATIONS", TermColor::Green),
            self.annotations.output.path,
            self.annotations.format,
            colorize("ENTITIES", TermColor::Green),
            self.entities.input.path,
            self.entities.filters,
            self.entities
                .excludes
                .path
                .clone()
                .unwrap_or("None".to_string()),
            colorize("LOGGING", TermColor::Green),
            self.logging
                .as_ref()
                .unwrap_or(&PyLogging {
                    level: "None".to_string()
                })
                .level
        ));
        Ok(output)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "AnnotationsConfig")]
pub struct PyAnnotations {
    #[pyo3(get)]
    pub output: PyOutput,
    #[pyo3(get)]
    pub format: PyFormat,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Logging")]
pub struct PyLogging {
    #[pyo3(get)]
    pub level: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Texts")]
pub struct PyTexts {
    #[pyo3(get)]
    pub input: PyInput,
    #[pyo3(get)]
    pub filters: PyFilters,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Input")]
pub struct PyInput {
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub filter: Option<bool>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Filters")]
pub struct PyFilters {
    #[pyo3(get)]
    pub alphanumeric: bool,
    #[pyo3(get)]
    pub case_sensitive: bool,
    #[pyo3(get)]
    pub min_length: i32,
    #[pyo3(get)]
    pub max_length: i32,
    #[pyo3(get)]
    pub punctuation: bool,
    #[pyo3(get)]
    pub numbers: bool,
    #[pyo3(get)]
    pub special_characters: bool,
    #[pyo3(get)]
    pub accept_special_characters: Option<String>,
    #[pyo3(get)]
    pub list_of_special_characters: Option<Vec<char>>,
}

impl Display for PyFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "alphanumeric: {},
            \tcase_sensitive: {},
            \tmin_length: {},
            \tmax_length: {},
            \tpunctuation: {},
            \tnumbers: {},
            \tspecial_characters: {},
            \taccept_special_characters: {},
            \tlist_of_special_characters: {}",
            self.alphanumeric,
            self.case_sensitive,
            self.min_length,
            self.max_length,
            self.punctuation,
            self.numbers,
            self.special_characters,
            self.accept_special_characters
                .as_ref()
                .unwrap_or(&"None".to_string()),
            self.list_of_special_characters
                .as_ref()
                .unwrap_or(&vec![' '])
                .iter()
                .collect::<String>()
        )
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Document")]
pub struct PyDocument {
    #[pyo3(get)]
    pub id: u32,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub label: Vec<(usize, usize, String)>,
}

#[pymethods]
impl PyDocument {
    #[new]
    #[pyo3(signature = (id, text, label))]
    pub fn new(id: u32, text: &str, label: Vec<(usize, usize, String)>) -> Self {
        PyDocument {
            id,
            text: text.to_string(),
            label,
        }
    }

    #[staticmethod]
    pub fn from_string(text: &str) -> Self {
        PyDocument {
            id: 0,
            text: text.to_string(),
            label: Vec::new(),
        }
    }

    // Annotate the document with the given entities
    #[pyo3(signature = (entities, case_sensitive = false))]
    pub fn annotate(&mut self, entities: Vec<PyEntity>, case_sensitive: bool) {
        let mut annotation = Document::from_string(self.text.clone());
        let entities = entities
            .into_iter()
            .map(|entity| Entity {
                name: entity.name,
                label: entity.label,
            })
            .collect();
        annotation.annotate(entities, case_sensitive);
        self.label = annotation
            .label
            .into_iter()
            .map(|label| (label.0, label.1, label.2))
            .collect();
    }

    // Pretty print the annotation
    // Example: Document(id=1, text="Hello World", label=[(0, 5, "Hello"), (6, 11, "World")])
    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = format!("Document(id={}, text={}, label=[", self.id, self.text);
        for (start, end, label) in &self.label {
            repr.push_str(&format!("({start}, {end}, {label}), "));
        }
        // Remove the last ", " if there is one
        if repr.ends_with(", ") {
            repr.pop();
            repr.pop();
        }
        repr.push_str("])");
        Ok(repr)
    }

    // TODO: This method is not correct, it does not handle overlapping labels
    // Pretty print the annotation
    // With colors for the labels in the text
    // Example: "Hello World" -> "Hello" [Hello] "World"
    fn pretty(&self) -> PyResult<String> {
        // Keep track of the color per label
        let mut color_map: std::collections::HashMap<String, TermColor> =
            std::collections::HashMap::new();
        let possible_colors = vec![
            TermColor::Red,
            TermColor::Green,
            TermColor::Yellow,
            TermColor::Blue,
            TermColor::Magenta,
            TermColor::Cyan,
        ];
        // Assign a color to each label
        for (_, _, label) in &self.label {
            if !color_map.contains_key(label) {
                let color = possible_colors[color_map.len() % possible_colors.len()];
                color_map.insert(label.to_string(), color);
            }
        }
        // Build the pretty string, labels are not sorted
        // and they can overlap
        // colorize the substring associated with the label
        // Example: "Hello World" -> "colorized(Hello)[VERB] World"
        let mut pretty = String::new();
        let mut start = 0;
        let mut sorted_label: Vec<(usize, usize, String)> = self.label.clone();
        sorted_label.sort_by(|a, b| a.0.cmp(&b.0));
        for (start_label, end_label, label) in sorted_label {
            let color = color_map.get(&label).unwrap();
            pretty.push_str(&self.text[start..start_label]);
            pretty.push_str(&colorize(&self.text[start_label..end_label], *color));
            pretty.push_str(&format!("[{label}]"));
            start = end_label;
        }
        pretty.push_str(&self.text[start..]);
        Ok(pretty)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Annotations")]
pub struct PyDocuments {
    #[pyo3(get)]
    pub annotations: Vec<PyDocument>,
    #[pyo3(get)]
    pub entities: Vec<PyEntity>,
    #[pyo3(get)]
    pub texts: Vec<PyText>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Entity")]
pub struct PyEntity {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub label: String,
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

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Text")]
pub struct PyText {
    #[pyo3(get)]
    pub text: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Output")]
pub struct PyOutput {
    #[pyo3(get)]
    pub path: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Format")]
#[allow(clippy::upper_case_acronyms)]
pub enum PyFormat {
    CSV,
    JSONL,
    SPACY,
    BRAT,
    CONLL,
}

impl Display for PyFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PyFormat::CSV => write!(f, "csv"),
            PyFormat::JSONL => write!(f, "jsonl"),
            PyFormat::SPACY => write!(f, "spacy"),
            PyFormat::BRAT => write!(f, "brat"),
            PyFormat::CONLL => write!(f, "conll"),
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Entities")]
pub struct PyEntities {
    #[pyo3(get)]
    pub input: PyInput,
    #[pyo3(get)]
    pub filters: PyFilters,
    #[pyo3(get)]
    pub excludes: PyExcludes,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Excludes")]
pub struct PyExcludes {
    #[pyo3(get)]
    pub path: Option<String>,
}

#[pymethods]
impl PyQuickner {
    #[new]
    #[pyo3(signature = (config_path = None))]
    pub fn new(config_path: Option<&str>) -> Self {
        let quickner = Quickner::new(config_path);
        PyQuickner::from_quickner(quickner)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = String::new();
        repr.push_str(&colorize("Entities: ", TermColor::Yellow));
        repr.push_str(&format!(
            "{} | ",
            self.entities.clone().unwrap_or(vec![]).len()
        ));
        repr.push_str(&colorize("Documents: ", TermColor::Green));
        repr.push_str(&format!(
            "{} | ",
            self.documents.clone().unwrap_or(vec![]).len()
        ));
        repr.push_str(&colorize("Annotations: ", TermColor::Blue));

        let annotations_hash = std::collections::HashMap::new();
        let annotations_count =
            self.documents
                .clone()
                .into_iter()
                .fold(annotations_hash, |mut acc, documents| {
                    for document in documents {
                        for (_, _, label) in document.label {
                            let count = acc.entry(label).or_insert(0);
                            *count += 1;
                        }
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
        // let quickner = Quickner::new(Some(&self.config_path));
        let mut quickner = match self.quickner.clone() {
            Some(quickner) => quickner,
            None => {
                return Err(PyErr::new::<exceptions::PyException, _>(
                    "Quickner not initialized",
                ))
            }
        };
        let annotations: Result<(), _> = quickner.process(save);
        match annotations {
            Ok(annotations) => annotations,
            Err(error) => return Err(PyErr::new::<exceptions::PyException, _>(error.to_string())),
        };
        self.documents = Some(
            quickner
                .documents
                .into_iter()
                .map(|document| PyDocument {
                    id: document.id,
                    text: document.text,
                    label: document.label,
                })
                .collect(),
        );
        self.entities = Some(
            quickner
                .entities
                .into_iter()
                .map(|entity| PyEntity {
                    name: entity.name,
                    label: entity.label,
                })
                .collect(),
        );
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
        let documents: Vec<Document> = self
            .documents
            .as_ref()
            .unwrap()
            .iter()
            .map(|annotation| Document {
                id: annotation.id,
                text: annotation.text.clone(),
                label: annotation.label.clone(),
            })
            .collect();
        let save_annotations = format.save(documents, &path);
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
        PyQuickner::from_quickner(quickner)
    }

    #[pyo3(signature = (path = None))]
    #[staticmethod]
    pub fn from_spacy(path: Option<&str>) -> PyQuickner {
        let path = match path {
            Some(path) => path.to_string(),
            None => String::from(""),
        };
        let quickner = Quickner::from_spacy(path.as_str());
        PyQuickner::from_quickner(quickner)
    }

    #[pyo3(signature = (path = None))]
    pub fn to_jsonl(&self, path: Option<&str>) {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let documents: Vec<Document> = self
            .documents
            .as_ref()
            .unwrap()
            .iter()
            .map(|annotation| Document {
                id: annotation.id,
                text: annotation.text.clone(),
                label: annotation.label.clone(),
            })
            .collect();
        quickner::Format::Jsonl
            .save(documents, path.as_str())
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
            .as_ref()
            .unwrap()
            .iter()
            .map(|annotation| Document {
                id: annotation.id,
                text: annotation.text.clone(),
                label: annotation.label.clone(),
            })
            .collect();
        quickner::Format::Csv
            .save(documents, path.as_str())
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
            .as_ref()
            .unwrap()
            .iter()
            .map(|annotation| Document {
                id: annotation.id,
                text: annotation.text.clone(),
                label: annotation.label.clone(),
            })
            .collect();
        quickner::Format::Spacy
            .save(documents, path.as_str())
            .unwrap();
    }
}

impl PyQuickner {
    fn from_quickner(quickner: Quickner) -> PyQuickner {
        PyQuickner {
            quickner: Some(quickner.clone()),
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
            config_path: quickner.config_file,
            documents: Some(
                quickner
                    .documents
                    .iter()
                    .map(|annotation| PyDocument {
                        id: annotation.id,
                        text: annotation.text.clone(),
                        label: annotation.label.clone(),
                    })
                    .collect(),
            ),
            entities: Some(
                quickner
                    .entities
                    .iter()
                    .map(|entity| PyEntity {
                        name: entity.name.clone(),
                        label: entity.label.clone(),
                    })
                    .collect(),
            ),
        }
    }
}
