// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//

use crate::{config::Format, Document};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpacyEntity {
    pub entity: Vec<(usize, usize, String)>,
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
    pub fn save(&self, annotations: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
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

    fn spacy(documents: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save as such [["text", {"entity": [[0, 4, "ORG"], [5, 10, "ORG"]]}]]

        // Transform Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)> into Structure

        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.json"))?;
        let annotations_tranformed: Vec<(String, SpacyEntity)> = documents
            .into_iter()
            .map(|annotation| {
                (
                    (*annotation.text).to_string(),
                    SpacyEntity {
                        entity: (*annotation.label).to_vec(),
                    },
                )
            })
            .collect();
        let json = serde_json::to_string(&annotations_tranformed).unwrap();
        file.write_all(json.as_bytes())?;
        Ok(path)
    }

    fn jsonl(documents: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
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

    fn csv(documents: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
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

    fn brat(documents: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // Save .ann and .txt files
        let path = Format::remove_extension_from_path(path);
        let mut file_ann = std::fs::File::create(format!("{path}.ann"))?;
        let mut file_txt = std::fs::File::create(format!("{path}.txt"))?;
        for document in documents {
            let text = &document.text;
            file_txt.write_all(text.as_bytes())?;
            file_txt.write_all(b"\n")?;
            for (id, (start, end, label)) in (*document.label).to_vec().into_iter().enumerate() {
                let entity = text[start..end].to_string();
                let line = format!("T{id}\t{label}\t{start}\t{end}\t{entity}");
                file_ann.write_all(line.as_bytes())?;
                file_ann.write_all(b"\n")?;
            }
        }
        Ok(path)
    }

    fn conll(documents: &Vec<Document>, path: &str) -> Result<String, std::io::Error> {
        // for reference: https://simpletransformers.ai/docs/ner-data-formats/
        let path = Format::remove_extension_from_path(path);
        let mut file = std::fs::File::create(format!("{path}.txt"))?;
        let annotations_tranformed: Vec<Vec<(String, String)>> = documents
            .into_iter()
            .map(|annotation| {
                let text = &annotation.text;
                // Split text into words
                let words: Vec<&str> = text.split_whitespace().collect();
                // If the word is not associated with an entity, then it is an "O"
                let mut labels: Vec<String> = vec!["O".to_string(); words.len()];
                // For each entity, find the word that contains it and assign the label to it
                for (start, end, label) in (*annotation.label).to_vec() {
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
