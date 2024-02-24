use crate::{
    pyentity::PyEntity,
    utils::{colorize, TermColor},
};
use pyo3::prelude::*;
use quickner::{hash_string, Document};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
#[pyclass(name = "Document")]
pub struct PyDocument {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub label: Vec<(usize, usize, String)>,
}

impl From<PyDocument> for Document {
    fn from(document: PyDocument) -> Self {
        Document {
            id: document.id,
            text: document.text,
            label: document.label,
        }
    }
}

impl From<Document> for PyDocument {
    fn from(document: Document) -> Self {
        PyDocument {
            id: document.id,
            text: document.text,
            label: document.label,
        }
    }
}

impl FromIterator<PyDocument> for Vec<Document> {
    fn from_iter<T: IntoIterator<Item = PyDocument>>(iter: T) -> Self {
        let mut documents = Vec::new();
        for document in iter {
            documents.push(Document::from(document));
        }
        documents
    }
}

#[pymethods]
impl PyDocument {
    #[new]
    #[pyo3(signature = (text, label=None))]
    pub fn new(text: &str, label: Option<Vec<(usize, usize, String)>>) -> Self {
        let id = hash_string(text);
        PyDocument {
            id,
            text: text.to_string(),
            label: label.unwrap_or(Vec::new()),
        }
    }

    #[staticmethod]
    pub fn from_string(text: &str) -> Self {
        let id = hash_string(text);
        PyDocument {
            id,
            text: text.to_string(),
            label: Vec::new(),
        }
    }

    // Annotate the document with the given entities
    #[pyo3(signature = (entities, case_sensitive = false))]
    pub fn annotate(&mut self, entities: Vec<PyEntity>, case_sensitive: bool) {
        let mut annotation = Document::from_string(self.text.clone());
        let entities = entities.into_iter().collect();
        annotation.annotate(entities, case_sensitive);
        self.label.extend(
            annotation
                .label
                .into_iter()
                .map(|label| (label.0, label.1, label.2))
                .collect::<Vec<(usize, usize, String)>>(),
        );
        self.set_unique_labels();
    }

    fn set_unique_labels(&mut self) {
        let mut labels: Vec<(usize, usize, String)> = Vec::new();
        for (start, end, label) in &self.label {
            if !labels.contains(&(*start, *end, label.clone())) {
                labels.push((*start, *end, label.clone()));
            }
        }
        self.label = labels;
    }

    // Pretty print the annotation
    // Example: Document(id=1, text="Hello World", label=[(0, 5, "Hello"), (6, 11, "World")])
    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = format!(
            "Document(id=\"{}\", text=\"{}\", label=[",
            self.id, self.text
        );
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
            let color = color_map.get(&label);
            if let Some(color) = color {
                // Handle case of this string: 'ne comprend absolument rien � twitter '
                // because of the � character
                if start_label > self.text.len() || end_label > self.text.len() {
                    return Err(pyo3::exceptions::PyValueError::new_err(
                        "start_label is greater than the length of the text",
                    ));
                }
                pretty.push_str(&self.text[start..start_label]);
                pretty.push_str(&colorize(&self.text[start_label..end_label], *color));
                pretty.push_str(&format!("[{label}]"));
                start = end_label;
            }
        }
        pretty.push_str(&self.text[start..]);
        Ok(pretty)
    }
}
