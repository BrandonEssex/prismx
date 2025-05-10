use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagEntry {
    pub name: String,
    pub trust: Option<String>,
}