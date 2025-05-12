// src/tag/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagEntry {
    pub name: String,
    pub usage_count: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TagGlossary {
    pub tags: HashMap<String, TagEntry>,
}

impl TagGlossary {
    pub fn add_tag(&mut self, tag: &str) {
        let entry = self.tags.entry(tag.to_string()).or_insert(TagEntry {
            name: tag.to_string(),
            usage_count: 0,
        });
        entry.usage_count += 1;
    }

    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(entry) = self.tags.get_mut(tag) {
            if entry.usage_count > 1 {
                entry.usage_count -= 1;
            } else {
                self.tags.remove(tag);
            }
        }
    }
}
