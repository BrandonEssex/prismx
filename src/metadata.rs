// src/metadata.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub fields: HashMap<String, String>,
}

impl Metadata {
    pub fn insert(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.fields.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.fields.remove(key);
    }
}