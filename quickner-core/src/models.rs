// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//

use crate::{
    config::{Config, Filters, Format},
    utils::get_progress_bar,
};
use log::{error, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};
use std::{env, error::Error};

/// Quickner is the main struct of the application
/// It holds the configuration file and the path to the configuration file
#[derive(Clone)]
pub struct Quickner {
    /// Path to the configuration file
    /// Default: ./config.toml
    pub config: Config,
    pub config_file: String,
    pub documents: Vec<Document>,
    pub entities: Vec<Entity>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
pub struct Text {
    pub text: String,
}

/// An entity is a text with a label
///
/// This object is used to hold the label used to
/// annotate the text.
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SpacyEntity {
    entity: Vec<(usize, usize, String)>,
}

/// An annotation is a text with a set of entities
///
/// This object is used to hold the text and the
/// entities found in the text.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    pub id: u32,
    pub text: String,
    pub label: Vec<(usize, usize, String)>,
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
        Document {
            id: 0,
            text,
            label: Vec::new(),
        }
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

        let label = Quickner::find_index(self.text.clone(), entities);
        match label {
            Some(label) => self.label = label,
            None => self.label = Vec::new(),
        }
    }
}

impl Format {
    /// Save annotations to a file in the specified format
    /// # Examples
    /// ```
    /// use quickner::models::Format;
    /// use quickner::models::Document;
    ///
    /// let annotations = vec![Annotation::from_string("Hello World".to_string())];
    /// let format = Format::Spacy;
    /// let path = "./test";
    /// let result = format.save(annotations, path);
    /// ```
    /// # Errors
    /// Returns an error if the file cannot be written
    /// # Panics
    /// Panics if the format is not supported
    pub fn save(&self, annotations: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        match self {
            Format::Spacy => Format::spacy(annotations, path),
            Format::Jsonl => Format::jsonl(annotations, path),
            Format::Csv => Format::csv(annotations, path),
            Format::Brat => Format::brat(annotations, path),
            Format::Conll => Format::conll(annotations, path),
        }
    }

    fn remove_extension_from_path(path: &str) -> String {
        let mut path = path.to_string();
        if path.contains('.') {
            path.truncate(path.rfind('.').unwrap());
        }
        path
    }

    fn spacy(documents: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save as such [["text", {"entity": [[0, 4, "ORG"], [5, 10, "ORG"]]}]]

        // Transform Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)> into Structure

        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.json"))?;
        let annotations_tranformed: Vec<(String, SpacyEntity)> = documents
            .into_iter()
            .map(|annotation| {
                (
                    annotation.text,
                    SpacyEntity {
                        entity: annotation.label,
                    },
                )
            })
            .collect();
        let json = serde_json::to_string(&annotations_tranformed).unwrap();
        file.write_all(json.as_bytes())?;
        Ok(path)
    }

    fn jsonl(documents: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save as such {"text": "text", "label": [[0, 4, "ORG"], [5, 10, "ORG"]]}
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.jsonl"))?;
        for document in documents {
            let json = serde_json::to_string(&document).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(path)
    }

    fn csv(documents: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save as such "text", "label"
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.csv"))?;
        for document in documents {
            let json = serde_json::to_string(&document).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(path)
    }

    fn brat(documents: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save .ann and .txt files
        let path = Format::remove_extension_from_path(path);
        let mut file_ann = std::fs::File::create(format!("{path}.ann"))?;
        let mut file_txt = std::fs::File::create(format!("{path}.txt"))?;
        for document in documents {
            let text = document.text;
            file_txt.write_all(text.as_bytes())?;
            file_txt.write_all(b"\n")?;
            for (id, (start, end, label)) in document.label.into_iter().enumerate() {
                let entity = text[start..end].to_string();
                let line = format!("T{id}\t{label}\t{start}\t{end}\t{entity}");
                file_ann.write_all(line.as_bytes())?;
                file_ann.write_all(b"\n")?;
            }
        }
        Ok(path)
    }

    fn conll(documents: Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // for reference: https://simpletransformers.ai/docs/ner-data-formats/
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.txt"))?;
        let annotations_tranformed: Vec<Vec<(String, String)>> = documents
            .into_iter()
            .map(|annotation| {
                let text = annotation.text;
                // Split text into words
                let words: Vec<&str> = text.split_whitespace().collect();
                // If the word is not associated with an entity, then it is an "O"
                let mut labels: Vec<String> = vec!["O".to_string(); words.len()];
                // For each entity, find the word that contains it and assign the label to it
                for (start, end, label) in annotation.label {
                    let entity = text[start..end].to_string();
                    // Find the index of the word that contains the entity
                    let index = words.iter().position(|&word| word.contains(&entity));
                    if index.is_none() {
                        continue;
                    }
                    let index = index.unwrap();
                    // If the word is the same as the entity, then it is a "B" label
                    labels[index] = label;
                }
                // Combine the words and labels into a single vector
                words
                    .iter()
                    .zip(labels.iter())
                    .map(|(word, label)| (word.to_string(), label.to_string()))
                    .collect()
            })
            .collect();
        // Save the data, one line per word with the word and label separated by a space
        for annotation in annotations_tranformed {
            for (word, label) in annotation {
                let line = format!("{word}\t{label}");
                file.write_all(line.as_bytes())?;
                file.write_all(b"\n")?;
            }
            file.write_all(b"\n")?;
        }
        Ok(path)
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Quickner {
    /// Find the index of the entities in the text
    /// # Arguments
    /// * `text` - The text to search
    /// * `entities` - The entities to search for
    /// # Returns
    /// * `Option<Vec<(usize, usize, String)>>` - The start and end index of the entity and the label
    /// # Example
    /// ```
    /// use std::collections::HashSet;
    /// use quickner::models::Entity;
    ///
    /// let text = "Rust is made by Mozilla".to_string();
    /// let mut entities = HashSet::new();
    /// entities.insert(Entity::new("Mozilla".to_string(), "ORG".to_string()));
    /// let annotations = Annotations::find_index(text, entities);
    /// assert_eq!(annotations, Some(vec![(15, 22, "ORG".to_string())]));
    /// ```
    fn find_index(text: String, entities: Vec<Entity>) -> Option<Vec<(usize, usize, String)>> {
        // let mut annotations = Vec::new();
        let annotations = entities.iter().map(|entity| {
            let target_len = entity.name.len();
            for (start, _) in text.match_indices(entity.name.as_str()) {
                if start == 0
                    || text.chars().nth(start - 1).unwrap().is_whitespace()
                    || text.chars().nth(start - 1).unwrap().is_ascii_punctuation()
                    || ((start + target_len) == text.len()
                        || text
                            .chars()
                            .nth(start + target_len)
                            .unwrap_or('N')
                            .is_whitespace()
                        || (text
                            .chars()
                            .nth(start + target_len)
                            .unwrap_or('N')
                            .is_ascii_punctuation()
                            && text.chars().nth(start + target_len).unwrap() != '.'
                            && (start > 0 && text.chars().nth(start - 1).unwrap() != '.')))
                {
                    return (start, start + target_len, entity.label.to_string());
                }
            }
            (0, 0, String::new())
        });
        let annotations: Vec<(usize, usize, String)> = annotations
            .filter(|(_, _, label)| !label.is_empty())
            .collect();
        if !annotations.is_empty() {
            Some(annotations)
        } else {
            None
        }
    }

    /// Annotate the texts with the entities
    /// # Example
    /// ```
    /// let mut annotations = Annotations::new(entities, texts);
    /// annotations.annotate();
    /// ```
    /// # Panics
    /// This function will panic if the texts are not loaded
    /// # Performance
    /// This function is parallelized using rayon
    /// # Progress
    /// This function will show a progress bar
    /// # Arguments
    /// * `self` - The annotations
    /// # Returns
    /// * `self` - The annotations with the annotations added
    /// # Errors
    /// This function will return an error if the texts are not loaded
    pub fn annotate(&mut self) {
        let pb = get_progress_bar(self.documents.len() as u64);
        pb.set_message("Annotating texts");
        self.documents.par_iter_mut().for_each(|document| {
            let mut t = document.text.clone();
            if !self.config.texts.filters.case_sensitive {
                t = t.to_lowercase();
            }
            let index = Quickner::find_index(t, self.entities.clone());
            let mut index = match index {
                Some(index) => index,
                None => vec![],
            };
            index.sort_by(|a, b| a.0.cmp(&b.0));
            document.label = index;
            pb.inc(1);
        });
        pb.finish();
    }
}

impl Quickner {
    /// Creates a new instance of Quickner
    /// If no configuration file is provided, the default configuration file is used.
    /// Default: ./config.toml
    /// # Arguments
    /// * `config_file` - The path to the configuration file
    /// # Example
    /// ```
    /// use quickner::Quickner;
    /// let quickner = Quickner::new(Some("./config.toml"));
    /// ```
    /// # Panics
    /// This function will panic if the configuration file does not exist
    /// # Returns
    /// * `Self` - The instance of Quickner
    /// # Errors
    /// This function will return an error if the configuration file does not exist
    pub fn new(config_file: Option<&str>) -> Self {
        println!("New instance of Quickner");
        println!("Configuration file: {config_file:?}");
        let config_file = match config_file {
            Some(config_file) => config_file.to_string(),
            None => "./config.toml".to_string(),
        };
        // Check if the configuration file path exists
        if Path::new(config_file.as_str()).exists() {
            info!("Configuration file: {}", config_file.as_str());
        } else {
            println!("Configuration file {} does not exist", config_file.as_str());
            error!("Configuration file {} does not exist", config_file.as_str());
            std::process::exit(1);
        }
        let config = Config::from_file(config_file.as_str());
        Quickner {
            config,
            config_file,
            documents: vec![],
            entities: vec![],
        }
    }

    fn parse_config(&self) -> Config {
        let mut config = self.config.clone();
        config.entities.filters.set_special_characters();
        config.texts.filters.set_special_characters();
        let log_level_is_set = env::var("QUICKNER_LOG_LEVEL_SET").ok();
        if log_level_is_set.is_none() {
            match config.logging {
                Some(ref mut logging) => {
                    env_logger::Builder::from_env(
                        env_logger::Env::default().default_filter_or(logging.level.as_str()),
                    )
                    .init();
                    env::set_var("QUICKNER_LOG_LEVEL_SET", "true");
                }
                None => {
                    env_logger::Builder::from_env(
                        env_logger::Env::default().default_filter_or("info"),
                    )
                    .init();
                    env::set_var("QUICKNER_LOG_LEVEL_SET", "true");
                }
            };
        }

        config
    }

    /// Process the texts and entities, and annotate the texts with the entities.
    /// This method will return the annotations, and optionally save the annotations to a file.
    /// # Arguments
    /// * `self` - The instance of Quickner
    /// * `save` - Whether to save the annotations to a file
    /// # Example
    /// ```
    /// use quickner::Quickner;
    /// let quickner = Quickner::new(Some("./config.toml"));
    /// quickner.process(true);
    /// ```
    /// # Returns
    /// * `Result<Annotations, Box<dyn Error>>` - The annotations
    /// # Errors
    /// This function will return an error if the configuration file does not exist
    /// This function will return an error if the entities file does not exist
    /// This function will return an error if the texts file does not exist
    pub fn process(&mut self, save: bool) -> Result<(), Box<dyn Error>> {
        let config = self.parse_config();
        config.summary();

        info!("----------------------------------------");
        let entities: HashSet<Entity> = self.entities(
            config.entities.input.path.as_str(),
            config.entities.filters,
            config.entities.input.filter.unwrap_or(false),
        );
        let texts: HashSet<Text> = self.texts(
            config.texts.input.path.as_str(),
            config.texts.filters,
            config.texts.input.filter.unwrap_or(false),
        );
        self.documents = texts
            .par_iter()
            .map(|text| Document {
                id: 0,
                text: text.text.clone(),
                label: vec![],
            })
            .collect();
        let excludes: HashSet<String> = match config.entities.excludes.path {
            Some(path) => {
                info!("Reading excludes from {}", path.as_str());
                self.excludes(path.as_str())
            }
            None => {
                info!("No excludes file provided");
                HashSet::new()
            }
        };
        // Remove excludes from entities
        let entities: HashSet<Entity> = entities
            .iter()
            .filter(|entity| !excludes.contains(&entity.name))
            .cloned()
            .collect();
        self.entities = Vec::from_iter(entities);
        info!("{} entities found", self.entities.len());
        self.annotate();
        info!("{} annotations found", self.documents.len());
        // annotations.save(&config.annotations.output.path);
        if save {
            let save = config
                .annotations
                .format
                .save(self.documents.clone(), &config.annotations.output.path);
            match save {
                Ok(_) => info!(
                    "Annotations saved with format {:?}",
                    config.annotations.format
                ),
                Err(e) => error!("Unable to save the annotations: {}", e),
            }
        }
        // Transform annotations to Python objects
        // List of tuples (text, [[start, end, label], [start, end, label], ...
        // let annotations_py: Vec<(String, Vec<(usize, usize, String)>)> =
        //     annotations.transform_annotations();
        // Ok(annotations_py)
        Ok(())
    }

    fn entities(&self, path: &str, filters: Filters, filter: bool) -> HashSet<Entity> {
        // Read CSV file and parse it
        // Expect columns: name, label
        info!("Reading entities from {}", path);
        let rdr = csv::Reader::from_path(path);
        match rdr {
            Ok(mut rdr) => {
                let mut entities = HashSet::new();
                for result in rdr.deserialize() {
                    let record: Result<Entity, csv::Error> = result;
                    match record {
                        Ok(mut entity) => {
                            if filter {
                                if filters.is_valid(&entity.name) {
                                    if !filters.case_sensitive {
                                        entity.name = entity.name.to_lowercase();
                                    }
                                    entities.insert(entity);
                                }
                            } else {
                                entities.insert(entity);
                            }
                        }
                        Err(e) => {
                            error!("Unable to parse the entities file: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                entities
            }
            Err(e) => {
                error!("Unable to parse the entities file: {}", e);
                std::process::exit(1);
            }
        }
    }

    fn texts(&self, path: &str, filters: Filters, filter: bool) -> HashSet<Text> {
        // Read CSV file and parse it
        // Expect columns: texts
        info!("Reading texts from {}", path);
        let rdr = csv::Reader::from_path(path);
        match rdr {
            Ok(mut rdr) => {
                let mut texts = HashSet::new();
                for result in rdr.deserialize() {
                    let record: Result<Text, csv::Error> = result;
                    match record {
                        Ok(text) => {
                            if filter {
                                if filters.is_valid(&text.text) {
                                    texts.insert(text);
                                }
                            } else {
                                texts.insert(text);
                            }
                        }
                        Err(e) => {
                            error!("Unable to parse the texts file: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                texts
            }
            Err(e) => {
                error!("Unable to parse the texts file: {}", e);
                std::process::exit(1);
            }
        }
    }

    fn excludes(&self, path: &str) -> HashSet<String> {
        // Read CSV file and parse it
        let rdr = csv::Reader::from_path(path);
        match rdr {
            Ok(mut rdr) => {
                let mut excludes = HashSet::new();
                for result in rdr.records() {
                    let record = result.unwrap();
                    excludes.insert(record[0].to_string());
                }
                excludes
            }
            Err(e) => {
                error!("Unable to parse the excludes file: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn from_jsonl(path: &str) -> Quickner {
        let file = File::open(path);
        let file = match file {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open the file {}: {}", path, e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        // Read the JSON objects from the file
        // Parse each JSON object as Annotation and add it to the annotations
        let mut entities = HashSet::new();
        let mut texts: Vec<Text> = Vec::new();
        let documents = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let annotation: Document = serde_json::from_str(line.as_str()).unwrap();
                let text = Text {
                    text: annotation.clone().text,
                };
                texts.push(text);
                // Extract the entity name from the label
                for label in &annotation.label {
                    // Extarct the entity name using indexes
                    let name = annotation.text[label.0..label.1].to_string();
                    let entity = Entity {
                        name: name.to_string(),
                        label: label.2.to_string(),
                    };
                    entities.insert(entity);
                }
                annotation
            })
            .collect();
        let entities = entities
            .into_iter()
            .map(|entity| {
                let mut entity = entity;
                entity.name = entity.name.to_lowercase();
                entity
            })
            .collect();
        Quickner {
            config: Config::default(),
            config_file: String::from(""),
            documents,
            entities,
        }
    }

    pub fn from_spacy(path: &str) -> Quickner {
        let file = File::open(path);
        let file = match file {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open the file {}: {}", path, e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        // Read the JSON objects from the file
        // Parse each JSON object as Annotation and add it to the annotations
        let mut entities = HashSet::new();
        let mut texts: Vec<Text> = Vec::new();
        let spacy = serde_json::from_reader(reader);
        let spacy: Vec<(String, SpacyEntity)> = match spacy {
            Ok(spacy) => spacy,
            Err(e) => {
                error!("Unable to parse the file {}: {}", path, e);
                std::process::exit(1);
            }
        };
        let documents = spacy
            .into_iter()
            .map(|doc| {
                let text = Text {
                    text: doc.0.clone(),
                };
                texts.push(text);
                // Extract the entity name from the label
                for ent in &doc.1.entity {
                    let name = doc.0[ent.0..ent.1].to_string();
                    let entity = Entity {
                        name,
                        label: ent.2.to_string(),
                    };
                    entities.insert(entity);
                }
                Document {
                    id: 0,
                    text: doc.0,
                    label: doc.1.entity,
                }
            })
            .collect();
        let entities = entities
            .into_iter()
            .map(|entity| {
                let mut entity = entity;
                entity.name = entity.name.to_lowercase();
                entity
            })
            .collect();
        Quickner {
            config: Config::default(),
            config_file: String::from(""),
            documents,
            entities,
        }
    }
}
