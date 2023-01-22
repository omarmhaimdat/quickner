use crate::{
    config::{self, Config, Filters},
    utils::is_valid,
};
use clap::Parser;
use log::{error, info};
use std::{collections::HashSet, path::PathBuf};

use crate::models::{Annotations, Entity, Text};

#[derive(Parser)]
#[clap(version = "1.0", author = "Omar MHAIMDAT")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    /// Default: ./config.toml
    #[clap(short, long, default_value = "./config.toml")]
    pub config: PathBuf,
}

impl Cli {
    fn parse_config(&self) -> Config {
        let mut config = match self.config.to_str() {
            Some(path) => config::Config::from_file(path),
            None => {
                error!("Unable to parse the configuration file");
                std::process::exit(1);
            }
        };
        config.entities.filters.set_special_characters();
        config.texts.filters.set_special_characters();
        config
    }

    pub fn process(&self) {
        let config = self.parse_config();
        config.summary();
        info!("----------------------------------------");
        let entities: HashSet<Entity> = self.entities(
            config.entities.input.path.as_str(),
            config.entities.filters,
            config.entities.input.filter.unwrap_or(false),
        );
        let texts: HashSet<Text> = self.texts(
            &config.texts.input.path.as_str(),
            config.texts.filters,
            config.texts.input.filter.unwrap_or(false),
        );
        let texts: Vec<Text> = texts.into_iter().collect();
        let excludes: HashSet<String> = match config.entities.excludes.path {
            Some(path) => self.excludes(path.as_str()),
            None => HashSet::new(),
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
        print!("{} annotations found", annotations.annotations.len());
        // annotations.save(&config.annotations.output.path);
        let save = config
            .annotations
            .format
            .save(annotations.annotations, &config.annotations.output.path);
        match save {
            Ok(_) => info!(
                "Annotations saved with format {:?}",
                config.annotations.format
            ),
            Err(e) => error!("Unable to save the annotations: {}", e),
        }
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
                                if is_valid(&filters, &entity.name) {
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
                                if is_valid(&filters, &text.text) {
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
