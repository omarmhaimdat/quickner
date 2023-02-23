use serde::{Deserialize, Serialize};

/// An entity is a text with a label
///
/// This object is used to hold the label used to
/// annotate the text.
#[derive(Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub label: String,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.label == other.label
    }
}
