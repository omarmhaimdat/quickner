use pyo3::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use crate::utils::{colorize, TermColor};
use quickner::{
    Annotations, Config, Entities, Excludes, Filters, Format, Input, Logging, Output, Texts,
};
use serde::{Deserialize, Serialize};

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

impl Default for PyConfig {
    fn default() -> Self {
        PyConfig {
            texts: PyTexts {
                input: PyInput {
                    path: "None".to_string(),
                    filter: None,
                },
                filters: PyFilters {
                    alphanumeric: false,
                    case_sensitive: false,
                    min_length: 0,
                    max_length: 0,
                    punctuation: false,
                    numbers: false,
                    special_characters: false,
                    accept_special_characters: None,
                    list_of_special_characters: None,
                },
            },
            annotations: PyAnnotations {
                output: PyOutput {
                    path: "None".to_string(),
                },
                format: PyFormat::SPACY,
            },
            entities: PyEntities {
                input: PyInput {
                    path: "None".to_string(),
                    filter: None,
                },
                filters: PyFilters {
                    alphanumeric: false,
                    case_sensitive: false,
                    min_length: 0,
                    max_length: 0,
                    punctuation: false,
                    numbers: false,
                    special_characters: false,
                    accept_special_characters: None,
                    list_of_special_characters: None,
                },
                excludes: PyExcludes { path: None },
            },
            logging: None,
        }
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
#[pyclass(name = "Output")]
pub struct PyOutput {
    #[pyo3(get)]
    pub path: String,
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

#[pymethods]
impl PyConfig {
    #[new]
    #[pyo3(signature = (path = None))]
    pub fn new(path: Option<&str>) -> PyResult<Self> {
        let path = match path {
            Some(path) => path.to_string(),
            None => "config.toml".to_string(),
        };
        let config: Config = Config::from_file(path.as_str());
        Ok(PyConfig::from_config(config))
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

impl PyConfig {
    fn from_config(config: Config) -> PyConfig {
        PyConfig {
            texts: PyTexts {
                input: PyInput {
                    path: config.texts.input.path,
                    filter: config.texts.input.filter,
                },
                filters: PyFilters {
                    alphanumeric: config.texts.filters.alphanumeric,
                    case_sensitive: config.texts.filters.case_sensitive,
                    min_length: config.texts.filters.min_length,
                    max_length: config.texts.filters.max_length,
                    punctuation: config.texts.filters.punctuation,
                    numbers: config.texts.filters.numbers,
                    special_characters: config.texts.filters.special_characters,
                    accept_special_characters: config.texts.filters.accept_special_characters,
                    list_of_special_characters: config
                        .texts
                        .filters
                        .list_of_special_characters
                        .map(|list| list.into_iter().collect::<Vec<char>>()),
                },
            },
            annotations: PyAnnotations {
                output: PyOutput {
                    path: config.annotations.output.path,
                },
                format: match config.annotations.format {
                    quickner::Format::Csv => PyFormat::CSV,
                    quickner::Format::Jsonl => PyFormat::JSONL,
                    quickner::Format::Spacy => PyFormat::SPACY,
                    quickner::Format::Brat => PyFormat::BRAT,
                    quickner::Format::Conll => PyFormat::CONLL,
                },
            },
            entities: PyEntities {
                input: PyInput {
                    path: config.entities.input.path,
                    filter: config.entities.input.filter,
                },
                filters: PyFilters {
                    alphanumeric: config.entities.filters.alphanumeric,
                    case_sensitive: config.entities.filters.case_sensitive,
                    min_length: config.entities.filters.min_length,
                    max_length: config.entities.filters.max_length,
                    punctuation: config.entities.filters.punctuation,
                    numbers: config.entities.filters.numbers,
                    special_characters: config.entities.filters.special_characters,
                    accept_special_characters: config.entities.filters.accept_special_characters,
                    list_of_special_characters: config
                        .entities
                        .filters
                        .list_of_special_characters
                        .map(|list| list.into_iter().collect::<Vec<char>>()),
                },
                excludes: PyExcludes {
                    path: config.entities.excludes.path,
                },
            },
            logging: match config.logging {
                Some(logging) => Some(PyLogging {
                    level: logging.level,
                }),
                None => None,
            },
        }
    }

    pub(crate) fn to_config(config: PyConfig) -> Config {
        Config {
            texts: Texts {
                input: Input {
                    path: config.texts.input.path,
                    filter: config.texts.input.filter,
                },
                filters: Filters {
                    alphanumeric: config.texts.filters.alphanumeric,
                    case_sensitive: config.texts.filters.case_sensitive,
                    min_length: config.texts.filters.min_length,
                    max_length: config.texts.filters.max_length,
                    punctuation: config.texts.filters.punctuation,
                    numbers: config.texts.filters.numbers,
                    special_characters: config.texts.filters.special_characters,
                    accept_special_characters: config.texts.filters.accept_special_characters,
                    list_of_special_characters: config
                        .texts
                        .filters
                        .list_of_special_characters
                        .map(|list| list.into_iter().collect::<HashSet<char>>()),
                },
            },
            annotations: Annotations {
                output: Output {
                    path: config.annotations.output.path,
                },
                format: match config.annotations.format {
                    PyFormat::CSV => Format::Csv,
                    PyFormat::JSONL => Format::Jsonl,
                    PyFormat::SPACY => Format::Spacy,
                    PyFormat::BRAT => Format::Brat,
                    PyFormat::CONLL => Format::Conll,
                },
            },
            entities: Entities {
                input: Input {
                    path: config.entities.input.path,
                    filter: config.entities.input.filter,
                },
                filters: Filters {
                    alphanumeric: config.entities.filters.alphanumeric,
                    case_sensitive: config.entities.filters.case_sensitive,
                    min_length: config.entities.filters.min_length,
                    max_length: config.entities.filters.max_length,
                    punctuation: config.entities.filters.punctuation,
                    numbers: config.entities.filters.numbers,
                    special_characters: config.entities.filters.special_characters,
                    accept_special_characters: config.entities.filters.accept_special_characters,
                    list_of_special_characters: config
                        .entities
                        .filters
                        .list_of_special_characters
                        .map(|list| list.into_iter().collect::<HashSet<char>>()),
                },
                excludes: Excludes {
                    path: config.entities.excludes.path,
                },
            },
            logging: match config.logging {
                Some(logging) => Some(Logging {
                    level: logging.level,
                }),
                None => None,
            },
        }
    }
}
