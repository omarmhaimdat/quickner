use log::{error, info};
use serde::Deserialize;
use std::{collections::HashSet, fs};
use toml;

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
#[derive(Debug, Deserialize)]
pub struct Config {
    pub texts: Texts,
    pub annotations: Annotations,
    pub entities: Entities,
}

#[derive(Debug, Deserialize)]
pub struct Texts {
    pub input: Input,
    pub filters: Filters,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub path: String,
    pub filter: Option<bool>,
}

#[derive(Debug, Deserialize)]
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

// Post init function to set the special characters
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
}

#[derive(Debug, Deserialize)]
pub struct Annotations {
    pub output: Output,
    pub format: Format,
}

#[derive(Debug, Deserialize)]
pub enum Format {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "jsonl")]
    Jsonl,
    #[serde(rename = "spacy")]
    Spacy,
    #[serde(rename = "brat")]
    Brat,
    #[serde(rename = "conll")]
    Conll,
}
#[derive(Debug, Deserialize)]
pub struct Output {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    pub input: Input,
    pub filters: Filters,
    pub excludes: Excludes,
}

#[derive(Debug, Deserialize)]
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
        info!("----------------------------------------");
        info!("Configuration file summary");
        info!("----------------------------------------");
        info!("Texts input path: {}", self.texts.input.path);
        info!("Texts filters: {:?}", self.texts.filters);
        info!("Annotations output path: {}", self.annotations.output.path);
        info!("Entities input path: {}", self.entities.input.path);
        info!("Entities filters: {:?}", self.entities.filters);
        info!(
            "Entities excludes path: {}",
            self.entities
                .excludes
                .path
                .as_ref()
                .unwrap_or(&"None".to_string())
        );
        info!("----------------------------------------");
    }
}
