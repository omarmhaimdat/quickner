// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//

use log::{debug, error};
use serde::Deserialize;
use std::{collections::HashSet, fs};
use std::{fmt::Display, fmt::Formatter, iter::FromIterator};

use crate::utils::{
    contains_numbers, contains_punctuation, contains_special_characters, is_alphanumeric,
};
/// A struct representing the configuration file.
/// # Examples
/// ```
/// use config::Config;
/// let config = Config::from_file("./config.toml");
/// ```
/// # Panics
/// Panics if the configuration file cannot be read or parsed.
/// # Errors
/// Returns an error if the configuration file cannot be read or parsed.
#[derive(Deserialize, Clone)]
pub struct Config {
    pub texts: Texts,
    pub annotations: Annotations,
    pub entities: Entities,
    pub logging: Option<Logging>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            texts: Texts::default(),
            annotations: Annotations::default(),
            entities: Entities::default(),
            logging: Some(Logging::default()),
        }
    }
}

/// A struct used to deserialize logging from the configuration file.
#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Logging {
    pub level: String,
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "info".to_string(),
        }
    }
}

/// A struct used to deserialize annotations from the configuration file.

#[derive(Deserialize, Clone, Default)]
pub struct Texts {
    pub input: Input,
    pub filters: Filters,
}

/// A struct used to deserialize input from the configuration file.
#[derive(Deserialize, Clone)]
pub struct Input {
    pub path: String,
    pub filter: Option<bool>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            path: "".to_string(),
            filter: Some(true),
        }
    }
}

/// A struct used to deserialize filters from the configuration file.
#[derive(Deserialize, Clone)]
pub struct Filters {
    pub alphanumeric: bool,
    pub case_sensitive: bool,
    pub min_length: i32,
    pub max_length: i32,
    pub punctuation: bool,
    pub numbers: bool,
    pub special_characters: bool,
    pub accept_special_characters: Option<String>,
    pub list_of_special_characters: Option<HashSet<char>>,
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            alphanumeric: false,
            case_sensitive: false,
            min_length: 0,
            max_length: 1024,
            punctuation: false,
            numbers: false,
            special_characters: false,
            accept_special_characters: None,
            list_of_special_characters: Some(HashSet::new()),
        }
    }
}

impl Display for Filters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "alphanumeric: {}, case_sensitive: {}, min_length: {}, max_length: {}, punctuation: {}, numbers: {}, special_characters: {}, accept_special_characters: {:?}",
            self.alphanumeric, self.case_sensitive, self.min_length, self.max_length, self.punctuation, self.numbers, self.special_characters, self.accept_special_characters
        )
    }
}

impl Filters {
    pub fn set_special_characters(&mut self) {
        let special_characters: HashSet<char> = HashSet::from_iter(vec![
            '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}',
            ';', ':', '"', '\'', '<', '>', ',', '.', '?', '/', '\\', '|', '~', '`',
        ]);
        let accept_special_characters: HashSet<char> = self
            .accept_special_characters
            .as_ref()
            .unwrap_or(&"".to_string())
            .chars()
            .collect();
        self.list_of_special_characters = Some(
            special_characters
                .difference(&accept_special_characters)
                .cloned()
                .collect(),
        );
    }

    pub fn get_special_characters(&self) -> HashSet<char> {
        self.list_of_special_characters.as_ref().unwrap().clone()
    }

    /// Checks if a string is a valid entity.
    /// Using the configuration file, it checks if the string is alphanumeric, contains punctuation, numbers, or special characters.
    /// # Examples
    /// ```
    /// use utils::is_valid;
    /// let text = "Hello, world!";
    /// assert_eq!(is_valid(config, text), true);
    /// ```
    pub fn is_valid(&self, text: &str) -> bool {
        if text.is_empty() {
            return false;
        }
        // False
        if self.alphanumeric && is_alphanumeric(text) {
            debug!("{} is not alphanumeric", text);
            return false;
        }
        if self.punctuation && contains_punctuation(text) {
            debug!("'{}' contains punctuation", text);
            return false;
        }
        if self.numbers && contains_numbers(text) {
            debug!("{} does not contain numbers", text);
            return false;
        }
        if self.special_characters
            && contains_special_characters(text, self.get_special_characters())
        {
            debug!("{} contains special characters", text);
            return false;
        }
        if self.min_length >= 0 && text.len() < self.min_length as usize {
            debug!("{} is too short", text);
            return false;
        }
        if self.max_length >= 0 && text.len() > self.max_length as usize {
            return false;
        }
        true
    }
}

/// A struct used to deserialize annotations from the configuration file.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Annotations {
    pub output: Output,
    pub format: Format,
}

/// A struct used to deserialize output format from the configuration file.
#[derive(Debug, Deserialize, Clone, Default)]
pub enum Format {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "jsonl")]
    #[default]
    Jsonl,
    #[serde(rename = "spacy")]
    Spacy,
    #[serde(rename = "brat")]
    Brat,
    #[serde(rename = "conll")]
    Conll,
}

/// A struct used to deserialize output from the configuration file.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Output {
    pub path: String,
}

/// A struct used to deserialize entities from the configuration file.
#[derive(Deserialize, Clone, Default)]
pub struct Entities {
    pub input: Input,
    pub filters: Filters,
    pub excludes: Excludes,
}

/// A struct used to deserialize excludes from the configuration file.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Excludes {
    pub path: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let config = fs::read_to_string(path).expect("Unable to read the configuration file");
        let config = toml::from_str(&config);
        match config {
            Ok(config) => config,
            Err(e) => {
                error!("Unable to parse the configuration file: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn summary(&self) {
        debug!("------------------------------");
        debug!("Configuration file summary    |");
        debug!("------------------------------");
        debug!("Texts input path: {}", self.texts.input.path);
        debug!("Texts filters: {}", self.texts.filters);
        debug!("Annotations output path: {}", self.annotations.output.path);
        debug!("Entities input path: {}", self.entities.input.path);
        debug!("Entities filters: {}", self.entities.filters);
        debug!(
            "Entities excludes path: {}",
            self.entities
                .excludes
                .path
                .as_ref()
                .unwrap_or(&"None".to_string())
        );
    }
}
