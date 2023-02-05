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
use std::{
    collections::{HashMap, HashSet},
    io::Write,
};
use std::{env, error::Error};
use std::{path::Path, time::Instant};

pub struct Quickner {
    /// Path to the configuration file
    /// Default: ./config.toml
    pub config: Config,
    pub config_file: Config,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Annotation {
    pub id: u32,
    pub text: String,
    pub label: Vec<(usize, usize, String)>,
}

impl Annotation {
    pub fn new(id: u32, text: String, label: Vec<(usize, usize, String)>) -> Self {
        Annotation { id, text, label }
    }

    pub fn from_string(text: String) -> Self {
        Annotation {
            id: 0,
            text,
            label: Vec::new(),
        }
    }

    pub fn annotate(&mut self, entities: HashSet<Entity>) {
        let label = Annotations::find_index(self.text.clone(), entities);
        match label {
            Some(label) => self.label = label,
            None => self.label = Vec::new(),
        }
    }
}

impl Format {
    pub fn save(&self, annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
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

    fn spacy(annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
        // Save as such [["text", {"entity": [[0, 4, "ORG"], [5, 10, "ORG"]]}]]

        // Transform Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)> into Structure
        #[derive(Serialize)]
        struct SpacyEntity {
            entity: HashMap<String, Vec<(usize, usize, String)>>,
        }

        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.json"))?;
        let annotations_tranformed: Vec<(String, SpacyEntity)> = annotations
            .into_iter()
            .map(|annotation| {
                let mut map = HashMap::new();
                map.insert("entity".to_string(), annotation.label);
                (annotation.text, SpacyEntity { entity: map })
            })
            .collect();
        let json = serde_json::to_string(&annotations_tranformed).unwrap();
        file.write_all(json.as_bytes())?;
        Ok(path)
    }

    fn jsonl(annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
        // Save as such {"text": "text", "label": [[0, 4, "ORG"], [5, 10, "ORG"]]}
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.jsonl"))?;
        for annotation in annotations {
            let json = serde_json::to_string(&annotation).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(path)
    }

    fn csv(annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
        // Save as such "text", "label"
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.csv"))?;
        for annotation in annotations {
            let json = serde_json::to_string(&annotation).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(path)
    }

    fn brat(annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
        // Save .ann and .txt files
        let path = Format::remove_extension_from_path(path);
        let mut file_ann = std::fs::File::create(format!("{path}.ann"))?;
        let mut file_txt = std::fs::File::create(format!("{path}.txt"))?;
        for annotation in annotations {
            let text = annotation.text;
            file_txt.write_all(text.as_bytes())?;
            file_txt.write_all(b"\n")?;
            for (id, (start, end, label)) in annotation.label.into_iter().enumerate() {
                let entity = text[start..end].to_string();
                let line = format!("T{id}\t{label}\t{start}\t{end}\t{entity}");
                file_ann.write_all(line.as_bytes())?;
                file_ann.write_all(b"\n")?;
            }
        }
        Ok(path)
    }

    fn conll(annotations: Vec<Annotation>, path: &str) -> Result<String, std::io::Error> {
        // for reference: https://simpletransformers.ai/docs/ner-data-formats/
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.txt"))?;
        let annotations_tranformed: Vec<Vec<(String, String)>> = annotations
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

impl PartialEq for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Annotations {
    pub annotations: Vec<Annotation>,
    pub entities: HashSet<Entity>,
    pub texts: Vec<Text>,
}

impl Annotations {
    pub fn new(entities: HashSet<Entity>, texts: Vec<Text>) -> Self {
        Annotations {
            annotations: Vec::new(),
            entities,
            texts,
        }
    }

    fn find_index(text: String, entities: HashSet<Entity>) -> Option<Vec<(usize, usize, String)>> {
        // let mut annotations = Vec::new();
        let annotations = entities.iter().map(|entity| {
            let target_len = entity.name.len();
            for (start, _) in text.to_lowercase().match_indices(entity.name.as_str()) {
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

    pub fn annotate(&mut self) {
        let pb = get_progress_bar(self.texts.len() as u64);
        pb.set_message("Annotating texts");
        let start = Instant::now();
        self.texts
            .par_iter()
            .enumerate()
            .map(|(i, text)| {
                let t = text.text.clone();
                let index = Annotations::find_index(t, self.entities.clone());
                let mut index = match index {
                    Some(index) => index,
                    None => vec![],
                };
                index.sort_by(|a, b| a.0.cmp(&b.0));
                pb.inc(1);
                Annotation {
                    id: (i + 1) as u32,
                    text: text.text.clone(),
                    label: index,
                }
            })
            .collect_into_vec(&mut self.annotations);
        let end = start.elapsed();
        println!(
            "Time elapsed in find_index() is: {:?} for {} texts",
            end,
            self.texts.len() * self.entities.len()
        );
        pb.finish();
    }
}

impl Quickner {
    /// Creates a new instance of Quickner
    /// If no configuration file is provided, the default configuration file is used.
    /// Default: ./config.toml
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
            config_file: Config::from_file(config_file.as_str()),
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

    /// Returns a list of Annotations
    pub fn process(&self, save: bool) -> Result<Annotations, Box<dyn Error>> {
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
        let texts: Vec<Text> = texts.into_iter().collect();
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
        info!("{} entities found", entities.len());
        info!("{} texts found", texts.len());
        let mut annotations = Annotations::new(entities, texts);
        annotations.annotate();
        info!("{} annotations found", annotations.annotations.len());
        // annotations.save(&config.annotations.output.path);
        if save {
            let save = config.annotations.format.save(
                annotations.annotations.clone(),
                &config.annotations.output.path,
            );
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
        Ok(annotations)
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
                        Ok(entity) => {
                            if filter {
                                if filters.is_valid(&entity.name) {
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
}
