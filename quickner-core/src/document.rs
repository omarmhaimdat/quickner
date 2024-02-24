use std::sync::Arc;

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use utils::hash_string;

use crate::entity::Entity;
use crate::quickner::Quickner;
use crate::utils;
/// An annotation is a text with a set of entities
///
/// This object is used to hold the text and the
/// entities found in the text.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub label: Vec<(usize, usize, String)>,
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.text == other.text && self.label == other.label
    }
}

impl Document {
    /// Create an annotation from a string
    /// # Examples
    /// ```
    /// use quickner::models::Annotation;
    ///
    /// let annotation = Annotation::from_string("Rust is developed by Mozilla".to_string());
    /// assert_eq!(annotation.text, "Rust is developed by Mozilla");
    /// ```
    pub fn from_string(text: String) -> Self {
        let id = hash_string(text.as_str());
        Document {
            id,
            text,
            label: Vec::new(),
        }
    }

    pub fn new(text: String, label: Vec<(usize, usize, String)>) -> Self {
        let id = hash_string(text.as_str());
        Self { id, text, label }
    }

    /// Annotate text given a set of entities
    /// # Examples
    /// ```
    /// use quickner::models::Document;
    /// use quickner::models::Entity;
    /// use std::collections::HashSet;
    ///
    /// let mut annotation = Annotation::from_string("Rust is developed by Mozilla".to_string());
    /// let entities = vec![
    ///    Entity::new("Rust".to_string(), "Language".to_string()),
    ///    Entity::new("Mozilla".to_string(), "Organization".to_string()),
    /// ].into_iter().collect();
    /// annotation.annotate(entities);
    /// assert_eq!(annotation.label, vec![(0, 4, "Language".to_string()), (23, 30, "Organization".to_string())]);
    /// ```
    pub fn annotate(&mut self, mut entities: Vec<Entity>, case_sensitive: bool) {
        if !case_sensitive {
            self.text = self.text.to_lowercase();
            entities
                .iter_mut()
                .for_each(|e| e.name = e.name.to_lowercase());
        }
        let patterns = entities
            .iter()
            .map(|entity| entity.name.as_str())
            .collect::<Vec<&str>>();
        let aho_corasick = Arc::new(AhoCorasick::new(patterns));
        let label = Quickner::find_index_using_aho_corasick(&self.text, &aho_corasick, &entities);
        match label {
            Some(label) => self.label.extend(label),
            None => self.label.extend(Vec::new()),
        }
        // Remove duplicate labels based on start and end index and label
        self.label
            .sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        self.set_unique_labels();
    }

    fn set_unique_labels(&mut self) {
        let mut labels: Vec<(usize, usize, String)> = Vec::new();
        for (start, end, label) in &self.label {
            if !labels.contains(&(*start, *end, label.to_string())) {
                labels.push((*start, *end, label.to_string()));
            }
        }
        self.label = labels;
    }
}
