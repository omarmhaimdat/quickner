//! quickner is a library for NER annotation that prodives a CLI and a Python API.
//! It comes with a default configuration file that can be modified to fit your needs.
//!
//! # Batch Annotation
//!
//! You can use quickner to annotate a batch of texts.
//!
//! Provide a configuration file and a folder containing your texts:
//! - a csv file containing the **texts** you want to annotate.
//! - a csv file containing the **entities** you want to annotate.
//! - a csv file containing the **excludes** you want to exclude from the annotation.
//!
//! ## Configuration
//!
//! The configuration file is a toml file that contains the following fields:
//! ```toml
//! [logging]
//! level = "info" # level of logging (debug, info, warning, error, fatal)
//!
//! [texts]
//!
//! [texts.input]
//! filter = false     # if true, only texts in the filter list will be used
//! path = "texts.csv" # path to the texts file
//!
//! [texts.filters]
//! accept_special_characters = ".,-" # list of special characters to accept in the text (if special_characters is true)
//! alphanumeric = false              # if true, only strictly alphanumeric texts will be used
//! case_sensitive = false            # if true, case sensitive search will be used
//! max_length = 1024                 # maximum length of the text
//! min_length = 0                    # minimum length of the text
//! numbers = false                   # if true, texts with numbers will not be used
//! punctuation = false               # if true, texts with punctuation will not be used
//! special_characters = false        # if true, texts with special characters will not be used
//!
//! [annotations]
//! format = "spacy" # format of the output file (jsonl, spaCy, brat, conll)
//!
//! [annotations.output]
//! path = "annotations.jsonl" # path to the output file
//!
//! [entities]
//!
//! [entities.input]
//! filter = true         # if true, only entities in the filter list will be used
//! path = "entities.csv" # path to the entities file
//! save = true           # if true, the entities found will be saved in the output file
//!
//! [entities.filters]
//! accept_special_characters = ".-" # list of special characters to accept in the entity (if special_characters is true)
//! alphanumeric = false             # if true, only strictly alphanumeric entities will be used
//! case_sensitive = false           # if true, case sensitive search will be used
//! max_length = 20                  # maximum length of the entity
//! min_length = 0                   # minimum length of the entity
//! numbers = false                  # if true, entities with numbers will not be used
//! punctuation = false              # if true, entities with punctuation will not be used
//! special_characters = true        # if true, entities with special characters will not be used
//!
//! [entities.excludes]
//! # path = "excludes.csv" # path to entities to exclude from the search
//!
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use quickner::models::Quickner;
//!
//! let quick = Quickner::new("./config.toml");
//! let annotations = quick.process(true);
//! ```
//!
//! # Single Annotation
//!
//! You can also use quickner to annotate a single text.
//! This is useful when you want to annotate a single text and then use the annotation in your code.
//!  
//! ```no_run
//! use quickner::Document;
//!
//! let annotation = Document::from_string("Rust is maintained by Mozilla");
//! let entities = HashMap::new();
//! entities.insert("Rust", "Programming Language");
//! entities.insert("Mozilla", "Organization");
//! annotation.annotate(entities);
//! ```
mod config;
mod models;
mod utils;

pub use crate::config::{Config, Entities, Excludes, Filters, Format, Input, Logging, Texts};
pub use crate::models::{Document, Entity, Quickner, Text};
