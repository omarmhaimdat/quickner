use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use crate::{config::Format, utils::get_progress_bar};

#[derive(Eq, Serialize, Deserialize, Clone, Hash, Debug)]
pub struct Text {
    pub text: String,
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[derive(Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub label: String,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Annotation {
    id: u32,
    text: String,
    pub label: Vec<(usize, usize, String)>,
}

impl Format {
    pub fn save(&self, annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        match self {
            Format::Spacy => Format::spacy(annotations, path),
            Format::Jsonl => Format::jsonl(annotations, path),
            Format::Csv => Format::csv(annotations, path),
            Format::Brat => Format::brat(annotations, path),
            Format::Conll => Format::conll(annotations, path),
            _ => unimplemented!(),
        }
    }

    fn remove_extension_from_path(path: &str) -> String {
        let mut path = path.to_string();
        if path.contains(".") {
            path.truncate(path.rfind(".").unwrap());
        }
        path
    }

    fn spacy(annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        // Save as such [["text", {"entity": [[0, 4, "ORG"], [5, 10, "ORG"]]}]]
        let mut file = std::fs::File::create(path)?;
        // Transform annotation to fit this structure [["text", {"entity": [[0, 4, "ORG"], [5, 10, "ORG"]]}], ...]
        let annotations_tranformed: Vec<(String, HashMap<String, Vec<(usize, usize, String)>>)> =
            annotations
                .into_iter()
                .map(|annotation| {
                    let mut map = HashMap::new();
                    map.insert("entity".to_string(), annotation.label);
                    (annotation.text, map)
                })
                .collect();
        let json = serde_json::to_string(&annotations_tranformed).unwrap();
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn jsonl(annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        // Save as such {"text": "text", "label": [[0, 4, "ORG"], [5, 10, "ORG"]]}
        let mut file = std::fs::File::create(path)?;
        for annotation in annotations {
            let json = serde_json::to_string(&annotation).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    fn csv(annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        // Save as such "text", "label"
        // "text", [[0, 4, "ORG"], [5, 10, "ORG"]]
        let mut file = std::fs::File::create(path)?;
        for annotation in annotations {
            let json = serde_json::to_string(&annotation).unwrap();
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    fn brat(annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        // Save .ann and .txt files
        // .ann file
        // T1	ORG 0 4	Apple
        // T2	ORG 5 10	Inc
        // .txt file
        // Apple Inc
        // Save as brat format
        let mut file_ann = std::fs::File::create(format!("{}.ann", path))?;
        let mut file_txt = std::fs::File::create(format!("{}.txt", path))?;
        for annotation in annotations {
            let text = annotation.text;
            file_txt.write_all(text.as_bytes())?;
            file_txt.write_all(b"\n")?;
            let mut id = 0;
            for (start, end, label) in annotation.label {
                let entity = text[start..end].to_string();
                let line = format!("T{}\t{}\t{}\t{}\t{}", id, label, start, end, entity);
                file_ann.write_all(line.as_bytes())?;
                file_ann.write_all(b"\n")?;
                id += 1;
            }
        }
        Ok(())
    }

    fn conll(annotations: Vec<Annotation>, path: &str) -> Result<(), std::io::Error> {
        // for reference: https://simpletransformers.ai/docs/ner-data-formats/
        // Example:
        // Text = Harry Potter was a student at Hogwarts
        // Harry B-PER
        // Potter I-PER
        // was O
        // a O
        // student B-MISC
        // at B-PER
        // Hogwarts I-PER
        // Use the example above to save the data
        let mut file = std::fs::File::create(path)?;
        // Transform text to fit this structure
        // [["Harry", "B-PER"], ["Potter", "I-PER"], ["was", "O"], ["a", "O"], ["student", "B-MISC"], ["at", "B-PER"], ["Hogwarts", "I-PER"]]
        // Perform the same operation for each text in annotations
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
                    if let None = index {
                        continue;
                    }
                    let index = index.unwrap();
                    // If the word is the same as the entity, then it is a "B" label
                    labels[index] = format!("{}", label);
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
                let line = format!("{}\t{}", word, label);
                file.write_all(line.as_bytes())?;
                file.write_all(b"\n")?;
            }
            file.write_all(b"\n")?;
        }
        Ok(())
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
    entities: HashSet<Entity>,
    texts: Vec<Text>,
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
        let annotations: Vec<(usize, usize, String)> = entities
            .iter()
            .map(|entity| {
                let target_len = entity.name.len();
                for (start, _) in text.to_lowercase().match_indices(entity.name.as_str()) {
                    if start == 0
                        || text.chars().nth(start - 1).unwrap().is_whitespace()
                        || text.chars().nth(start - 1).unwrap().is_ascii_punctuation()
                    {
                        if (start + target_len) == text.len()
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
                                && (start > 0 && text.chars().nth(start - 1).unwrap() != '.'))
                        {
                            return (start, start + target_len, entity.label.to_string());
                        }
                    }
                }
                (0, 0, String::new())
            })
            .collect();
        let annotations: Vec<(usize, usize, String)> = annotations
            .into_iter()
            .filter(|(_, _, label)| !label.is_empty())
            .collect();
        if annotations.len() > 0 {
            Some(annotations)
        } else {
            None
        }
    }

    pub fn annotate(&mut self) {
        let pb = get_progress_bar(self.texts.len() as u64);
        pb.set_message("Annotating texts");
        self.texts
            .par_iter()
            .enumerate()
            .map(|(i, text)| {
                let t = text.text.clone();
                let index = Annotations::find_index(t, self.entities.clone());
                let index = match index {
                    Some(index) => index,
                    None => vec![],
                };
                pb.inc(1);
                Annotation {
                    id: (i + 1) as u32,
                    text: text.text.clone(),
                    label: index,
                }
            })
            .collect_into_vec(&mut self.annotations);
        pb.finish();
    }
}
