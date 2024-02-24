use crate::{
    config::{Config, Filters},
    models::Text,
    utils::{char_to_byte, get_progress_bar, is_valid_utf8},
    SpacyEntity,
};
use aho_corasick::AhoCorasick;
use log::{error, info, warn};
use rayon::prelude::*;
use std::{collections::HashMap, path::Path, sync::Arc};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
use std::{env, error::Error};

use crate::document::Document;
use crate::entity::Entity;

/// Quickner is the main struct of the application
/// It holds the configuration file and the path to the configuration file
#[derive(Clone)]
pub struct Quickner {
    /// Path to the configuration file
    /// Default: ./config.toml
    pub config: Config,
    pub config_file: Option<String>,
    pub documents: Vec<Document>,
    pub entities: Vec<Entity>,
    pub documents_hash: HashMap<String, Document>,
    pub documents_label_index: HashMap<String, Vec<String>>,
    pub documents_entities_index: HashMap<String, Vec<String>>,
}

impl Default for Quickner {
    fn default() -> Self {
        Self {
            config: Config::default(),
            config_file: Some("./config.toml".to_string()),
            documents: Vec::new(),
            entities: Vec::new(),
            documents_hash: HashMap::new(),
            documents_label_index: HashMap::new(),
            documents_entities_index: HashMap::new(),
        }
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
    pub(crate) fn find_index(
        text: String,
        entities: Vec<Entity>,
    ) -> Option<Vec<(usize, usize, String)>> {
        // let mut annotations = Vec::new();
        let annotations = entities.iter().filter_map(|entity| {
            let target_len = entity.name.len();
            for (start, _) in text.match_indices(entity.name.as_str()) {
                if start == 0
                    || text
                        .chars()
                        .nth(start - 1)
                        .unwrap_or_else(|| 'N')
                        .is_whitespace()
                    || text
                        .chars()
                        .nth(start - 1)
                        .unwrap_or_else(|| 'N')
                        .is_ascii_punctuation()
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
                    return Some((start, start + target_len, entity.label.to_string()));
                }
            }
            None
        });
        // Unique annotations
        let mut annotations = annotations.collect::<Vec<(usize, usize, String)>>();
        annotations.sort_by(|a, b| a.0.cmp(&b.0));
        annotations.dedup();
        // Sort annotations by start index
        if !annotations.is_empty() {
            Some(annotations)
        } else {
            None
        }
    }

    pub(crate) fn find_index_using_aho_corasick(
        text: &str,
        aho_corasick: &Arc<AhoCorasick>,
        entites: &Vec<Entity>,
    ) -> Option<Vec<(usize, usize, String)>> {
        if !is_valid_utf8(text) {
            warn!("Skipping invalid utf8 text: \"{}\"", text);
            return None;
        }
        let mut annotations = Vec::new();
        for mat in aho_corasick.find_iter(&text) {
            let start = mat.start();
            // convert byte index to char index (assuming utf8)
            let start = text[..start].chars().count();
            let end = mat.end();
            let end = text[..end].chars().count();
            let label = entites[mat.pattern()].label.to_string();
            let name = entites[mat.pattern()].name.to_string();
            let target_len = name.len();
            if start == 0
                && (text.chars().nth(end).unwrap_or('N').is_whitespace()
                    || (text.chars().nth(end).unwrap_or('N').is_ascii_punctuation()))
            {
                annotations.push((start, end, label));
                continue;
            }
            // if text == "python was created by guido van rossum" {
            //     println!("Start: {}, End: {}, text_len: {}, End + 1: {}", start, end, text.len(), text.chars().nth(end + 1).unwrap_or('N'));
            // }
            // println!("Start: {}, End: {}, text_len: {}", start, end, char_len);
            if start > 0
                && text
                    .chars()
                    .nth(start - 1)
                    .unwrap_or_else(|| 'N')
                    .is_whitespace()
                && (text.chars().nth(end).unwrap_or_else(|| 'N').is_whitespace()
                    || text
                        .chars()
                        .nth(end)
                        .unwrap_or_else(|| 'N')
                        .is_ascii_punctuation())
            {
                annotations.push((start, end, label));
                continue;
            }
            if start > 0
                && text
                    .chars()
                    .nth(start - 1)
                    .unwrap_or_else(|| 'N')
                    .is_ascii_punctuation()
                && (text.chars().nth(end).unwrap_or_else(|| 'N').is_whitespace()
                    || text
                        .chars()
                        .nth(end)
                        .unwrap_or_else(|| 'N')
                        .is_ascii_punctuation())
            {
                annotations.push((start, end, label));
                continue;
            }
            if (start + target_len) == text.len() {
                annotations.push((start, end, label));
                continue;
            }
            if (text
                .chars()
                .nth(start - 1)
                .unwrap_or_else(|| 'N')
                .is_ascii_punctuation()
                || text
                    .chars()
                    .nth(start - 1)
                    .unwrap_or_else(|| 'N')
                    .is_whitespace())
                && text
                    .chars()
                    .nth(start + target_len)
                    .unwrap_or('N')
                    .is_whitespace()
            {
                annotations.push((start, end, label));
                continue;
            }
            if (text
                .chars()
                .nth(start - 1)
                .unwrap_or_else(|| 'N')
                .is_ascii_punctuation()
                || text
                    .chars()
                    .nth(start - 1)
                    .unwrap_or_else(|| 'N')
                    .is_whitespace())
                && text
                    .chars()
                    .nth(start + target_len)
                    .unwrap_or('N')
                    .is_ascii_punctuation()
                && text.chars().nth(start + target_len).unwrap() != '.'
                && (start > 0 && text.chars().nth(start - 1).unwrap() != '.')
            {
                annotations.push((start, end, label));
            }
        }
        // Unique annotations
        annotations.sort_by(|a, b| a.0.cmp(&b.0));
        annotations.dedup();
        // Sort annotations by start index
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
        let patterns = self
            .entities
            .iter()
            .map(|entity| entity.name.as_str())
            .collect::<Vec<&str>>();
        // Check if apple is in the patterns
        // if patterns.contains(&"apple") {
        //     println!("Apple found in patterns");
        // }
        let aho_corasick = Arc::new(AhoCorasick::new(patterns));
        self.documents.par_iter_mut().for_each(|document| {
            let t: &mut String = &mut document.text;
            if !self.config.texts.filters.case_sensitive {
                *t = t.to_lowercase();
            };
            // ahocorasick implementation
            let index = Quickner::find_index_using_aho_corasick(&t, &aho_corasick, &self.entities);
            let mut index = match index {
                Some(index) => index,
                None => vec![],
            };
            index.sort_by(|a, b| a.0.cmp(&b.0));
            document.label.extend(index);
            pb.inc(1);
        });
        self.documents_hash = self
            .documents
            .iter()
            .map(|document| (document.id.clone(), document.clone()))
            .collect();
        self.build_label_index();
        self.build_entity_index();
        pb.finish();
    }

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
        let config_file = match config_file {
            Some(config_file) => config_file.to_string(),
            None => "./config.toml".to_string(),
        };
        // Check if the configuration file path exists
        if Path::new(config_file.as_str()).exists() {
            info!("Configuration file: {}", config_file.as_str());
        } else {
            warn!(
                "Configuration file {} does not exist, using default Config",
                config_file.as_str()
            );
            return Quickner::default();
        }
        let config = Config::from_file(config_file.as_str());
        Quickner {
            config,
            config_file: Some(config_file),
            ..Default::default()
        }
    }

    pub fn add_document(&mut self, document: Document) {
        {
            let document = self.documents_hash.get(&document.id);
            if document.is_some() {
                warn!("Document {} already exists", document.unwrap().id);
                return;
            }
        }
        self.documents.push(document.to_owned());
        self.documents_hash
            .insert(document.id.to_owned(), document.to_owned());
        self.add_to_entity_index(&document);
        self.add_to_label_index(&document);
    }

    pub fn add_document_from_string(&mut self, text: &str) {
        let document = Document::from_string(text.to_string());
        self.documents.push(document.to_owned());
        self.documents_hash
            .insert(document.id.to_owned(), document.to_owned());
        self.add_to_entity_index(&document);
        self.add_to_label_index(&document);
    }

    pub fn add_entity(&mut self, entity: Entity) {
        if self.entities.contains(&entity) {
            warn!("Entity {} already exists", entity.name);
            return;
        }
        self.entities.push(entity);
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
        if self.entities.is_empty() {
            let entities: HashSet<Entity> = self.entities(
                config.entities.input.path.as_str(),
                config.entities.filters,
                config.entities.input.filter.unwrap_or(false),
            );
            self.entities = entities.into_iter().collect();
        }
        if self.documents.is_empty() {
            let texts: HashSet<Text> = self.texts(
                config.texts.input.path.as_str(),
                config.texts.filters,
                config.texts.input.filter.unwrap_or(false),
            );
            self.documents = texts
                .par_iter()
                .map(|text| Document::new((*text.text).to_string(), vec![]))
                .collect();
        }
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
        let entities: HashSet<Entity> = self
            .entities
            .iter()
            .filter(|entity| !excludes.contains(&entity.name))
            .cloned()
            .collect();
        self.entities = Vec::from_iter(entities);
        if !self.config.entities.filters.case_sensitive {
            self.entities = self
                .entities
                .iter()
                .map(|entity| Entity {
                    name: entity.name.to_lowercase(),
                    label: entity.label.to_string(),
                })
                .collect();
        }
        info!("{} entities found", self.entities.len());
        self.annotate();
        info!("{} annotations found", self.documents.len());
        let len_entities = self.entities.len();
        let len_documents = self.documents.len();
        let number_of_checks = len_entities * len_documents;
        // Transform number of checks to a human readable string
        let number_of_checks = match number_of_checks {
            0..=1000 => format!("{}", number_of_checks),
            1001..=1000000 => format!("{:.2}K", number_of_checks as f64 / 1000.0),
            1000001..=1000000000 => format!("{:.2}M", number_of_checks as f64 / 1000000.0),
            _ => format!("{:.2}B", number_of_checks as f64 / 1000000000.0),
        };
        info!("Number of unique checks: {}", number_of_checks);
        // annotations.save(&config.annotations.output.path);
        if save {
            let save = config
                .annotations
                .format
                .save(&self.documents, &config.annotations.output.path);
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
                        Err(_) => {
                            warn!("Unable to parse the entities file, using empty list");
                            return HashSet::new();
                        }
                    }
                }
                entities
            }
            Err(_) => {
                warn!("Unable to parse the entities file, using empty list");
                HashSet::new()
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
        let mut entities = Vec::new();
        let mut texts: Vec<Text> = Vec::new();
        let documents: Vec<Document> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let annotation: Document = serde_json::from_str(line.as_str()).unwrap();
                let text = Text {
                    text: (*annotation.text).to_string(),
                };
                texts.push(text);
                // Extract the entity name from the label
                for label in &annotation.label {
                    let indices = char_to_byte((*annotation.text).to_string(), label.0, label.1);
                    let name = annotation.text[indices.0..indices.1].to_string();
                    let entity = Entity {
                        name: name.to_string().to_lowercase(),
                        label: label.2.to_string(),
                    };
                    entities.push(entity);
                }
                annotation
            })
            .collect();
        let entities = Quickner::unique_entities(entities);
        let documents_hash = Quickner::document_hash(&documents);
        let mut quick = Quickner {
            config: Config::default(),
            config_file: None,
            documents,
            entities,
            documents_hash,
            documents_label_index: HashMap::new(),
            documents_entities_index: HashMap::new(),
        };
        quick.build_entity_index();
        quick.build_label_index();
        quick
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
        let mut entities: Vec<Entity> = Vec::new();
        let mut texts: Vec<Text> = Vec::new();
        let spacy = serde_json::from_reader(reader);
        let spacy: Vec<(String, SpacyEntity)> = match spacy {
            Ok(spacy) => spacy,
            Err(e) => {
                error!("Unable to parse the file {}: {}", path, e);
                std::process::exit(1);
            }
        };
        let documents: Vec<Document> = spacy
            .into_iter()
            .map(|doc| {
                let text = Text {
                    text: (*doc.0).to_string(),
                };
                texts.push(text);
                // Extract the entity name from the label
                for ent in &doc.1.entity {
                    let name = doc.0[ent.0..ent.1].to_string();
                    let entity = Entity {
                        name: name.to_lowercase(),
                        label: ent.2.to_string(),
                    };
                    entities.push(entity);
                }
                Document::new(doc.0, doc.1.entity)
            })
            .collect();
        let entities = Quickner::unique_entities(entities);
        let documents_hash = Quickner::document_hash(&documents);
        let mut quick = Quickner {
            config: Config::default(),
            config_file: None,
            documents,
            entities,
            documents_hash,
            documents_label_index: HashMap::new(),
            documents_entities_index: HashMap::new(),
        };
        quick.build_entity_index();
        quick.build_label_index();
        quick
    }

    pub fn spacy(&self, chunks: Option<usize>) -> Vec<Vec<(String, SpacyEntity)>> {
        let mut spacy: Vec<(String, SpacyEntity)> = Vec::new();
        for document in &self.documents {
            let mut entity: Vec<(usize, usize, String)> = Vec::new();
            for label in &document.label {
                entity.push((label.0, label.1, (*label.2).to_string()));
            }
            spacy.push(((*document.text).to_string(), SpacyEntity { entity }));
        }
        let chunks = match chunks {
            Some(chunks) => chunks,
            None => spacy.len(),
        };
        // Split the spacy vector into chunks
        // i.e. if the vector has 1000 elements and the chunks is 100 then
        // the vector will be split into 10 chunks of 100 elements each
        let mut spacy_chunks: Vec<Vec<(String, SpacyEntity)>> = Vec::new();
        for chunk in spacy.chunks(chunks) {
            spacy_chunks.push(chunk.to_vec());
        }
        spacy_chunks
    }
}

impl Quickner {
    pub fn build_label_index(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::new();
        for document in &self.documents {
            for label in &document.label {
                let entry = index.entry((*label.2).to_string()).or_insert(Vec::new());
                entry.push((*document.id).to_string());
            }
        }
        self.documents_label_index = index;
    }

    pub fn build_entity_index(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::new();
        for document in &self.documents {
            for label in &document.label {
                // Translate the indices to byte indices
                let indices = char_to_byte((*document.text).to_string(), label.0, label.1);
                let name = document.text[indices.0..indices.1].to_string();
                let entry = index.entry(name.to_lowercase()).or_insert(Vec::new());
                entry.push((*document.id).to_string());
            }
        }
        self.documents_entities_index = index;
    }

    fn add_to_label_index(&mut self, document: &Document) {
        for label in &document.label {
            let entry = self
                .documents_label_index
                .entry((*label.2).to_string())
                .or_insert(Vec::new());
            entry.push((*document.id).to_string());
        }
    }

    fn add_to_entity_index(&mut self, document: &Document) {
        for label in &document.label {
            let indices = char_to_byte((*document.text).to_string(), label.0, label.1);
            let name = document.text[indices.0..indices.1].to_string();
            let entry = self
                .documents_entities_index
                .entry(name.to_lowercase())
                .or_insert(Vec::new());
            entry.push((*document.id).to_string());
        }
    }

    fn _remove_from_label_index(&mut self, document: &Document) {
        for label in &document.label {
            let entry = self
                .documents_label_index
                .entry((*label.2).to_string())
                .or_insert(Vec::new());
            entry.retain(|x| x != &document.id);
        }
    }

    fn _remove_from_entity_index(&mut self, document: &Document) {
        for label in &document.label {
            let indices = char_to_byte(document.text.clone(), label.0, label.1);
            let name = document.text[indices.0..indices.1].to_string();
            let entry = self
                .documents_entities_index
                .entry(name.to_lowercase())
                .or_insert(Vec::new());
            entry.retain(|x| x != &document.id);
        }
    }

    fn unique_entities(entities: Vec<Entity>) -> Vec<Entity> {
        entities
            .into_iter()
            .collect::<HashSet<Entity>>()
            .into_iter()
            .collect::<Vec<Entity>>()
    }

    pub fn document_hash(documents: &[Document]) -> HashMap<String, Document> {
        documents
            .iter()
            .map(|document| (document.id.clone(), document.clone()))
            .collect::<HashMap<String, Document>>()
    }
}
