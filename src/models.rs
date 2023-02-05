use std::fmt::{Display, Formatter};

use pyo3::{exceptions, prelude::*};

use crate::utils::{colorize, TermColor};
use quickner_cli::models::{Annotation, Annotations, Quickner};
use serde::{Deserialize, Serialize};
/// Transform Rust code into Python code
/// Create a Python Class version of the Rust struct
/// from the quickner_cli crate

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Quickner")]
pub struct PyQuickner {
    #[pyo3(get)]
    pub config: PyConfig,
    #[pyo3(get)]
    pub config_path: String,
    #[pyo3(get)]
    pub texts: Option<Vec<PyText>>,
    #[pyo3(get)]
    pub annotations: Option<Vec<PyAnnotation>>,
    #[pyo3(get)]
    pub entities: Option<Vec<PyEntity>>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Config")]
pub struct PyConfig {
    #[pyo3(get)]
    pub texts: PyTexts,
    #[pyo3(get)]
    pub annotations: PyAnnotationsConfig,
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
        Configuration file summary:
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
pub struct PyAnnotationsConfig {
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
#[pyclass(name = "Annotation")]
pub struct PyAnnotation {
    #[pyo3(get)]
    pub id: u32,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub label: Vec<(usize, usize, String)>,
}

#[pymethods]
impl PyAnnotation {
    #[new]
    #[pyo3(signature = (id, text, label))]
    pub fn new(id: u32, text: &str, label: Vec<(usize, usize, String)>) -> Self {
        PyAnnotation {
            id,
            text: text.to_string(),
            label,
        }
    }

    // Pretty print the annotation
    // Example: Annotation(id=1, text="Hello World", label=[(0, 5, "Hello"), (6, 11, "World")])
    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = format!("Annotation(id={}, text={}, label=[", self.id, self.text);
        for (start, end, label) in &self.label {
            repr.push_str(&format!("({}, {}, {}), ", start, end, label));
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
            pretty.push_str(&format!("[{}]", label));
            start = end_label;
        }
        pretty.push_str(&self.text[start..]);
        Ok(pretty)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Annotations")]
pub struct PyAnnotations {
    #[pyo3(get)]
    pub annotations: Vec<PyAnnotation>,
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
        match quickner {
            quickner => PyQuickner {
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
                    annotations: PyAnnotationsConfig {
                        output: PyOutput {
                            path: quickner.config.annotations.output.path,
                        },
                        format: match quickner.config.annotations.format {
                            quickner_cli::config::Format::Csv => PyFormat::CSV,
                            quickner_cli::config::Format::Jsonl => PyFormat::JSONL,
                            quickner_cli::config::Format::Spacy => PyFormat::SPACY,
                            quickner_cli::config::Format::Brat => PyFormat::BRAT,
                            quickner_cli::config::Format::Conll => PyFormat::CONLL,
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
                config_path: match config_path {
                    Some(config_path) => config_path.to_string(),
                    None => String::from(""),
                },
                annotations: None,
                texts: None,
                entities: None,
            },
        }
    }

    #[pyo3(signature = (save = false))]
    pub fn process(&mut self, save: bool) -> PyResult<Vec<PyAnnotation>> {
        let quickner = Quickner::new(Some(&self.config_path));
        let annotations: Result<Annotations, _> = quickner.process(save);
        let annotations = match annotations {
            Ok(annotations) => annotations,
            Err(error) => return Err(PyErr::new::<exceptions::PyException, _>(error.to_string())),
        };
        let py_annotations = PyAnnotations {
            annotations: annotations
                .annotations
                .into_iter()
                .map(|annotation| PyAnnotation {
                    id: annotation.id,
                    text: annotation.text,
                    label: annotation.label,
                })
                .collect(),
            entities: annotations
                .entities
                .into_iter()
                .map(|entity| PyEntity {
                    name: entity.name,
                    label: entity.label,
                })
                .collect(),
            texts: annotations
                .texts
                .into_iter()
                .map(|text| PyText { text: text.text })
                .collect(),
        };
        self.annotations = Some(py_annotations.annotations.clone());
        self.texts = Some(py_annotations.texts.clone());
        self.entities = Some(py_annotations.entities.clone());
        Ok(py_annotations.annotations)
    }

    #[pyo3(signature = (path = None, format = PyFormat::JSONL))]
    pub fn save_annotations(&self, path: Option<&str>, format: PyFormat) -> PyResult<String> {
        let path = match path {
            Some(path) => path.to_string(),
            None => self.config.annotations.output.path.clone(),
        };
        let format = match format {
            PyFormat::CSV => quickner_cli::config::Format::Csv,
            PyFormat::JSONL => quickner_cli::config::Format::Jsonl,
            PyFormat::SPACY => quickner_cli::config::Format::Spacy,
            PyFormat::BRAT => quickner_cli::config::Format::Brat,
            PyFormat::CONLL => quickner_cli::config::Format::Conll,
        };
        let annotations: Vec<Annotation> = self
            .annotations
            .as_ref()
            .unwrap()
            .iter()
            .map(|annotation| Annotation {
                id: annotation.id,
                text: annotation.text.clone(),
                label: annotation.label.clone(),
            })
            .collect();
        let save_annotations = format.save(annotations, &path);
        match save_annotations {
            Ok(_) => Ok(save_annotations.unwrap()),
            Err(error) => Err(PyErr::new::<exceptions::PyException, _>(error.to_string())),
        }
    }
}
