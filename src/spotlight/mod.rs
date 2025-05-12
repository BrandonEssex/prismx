// src/spotlight/mod.rs

use crate::node::Node;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct SpotlightEngine {
    pub cache: HashMap<String, Vec<Uuid>>,
}

impl SpotlightEngine {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn index(&mut self, nodes: &[Node]) {
        self.cache.clear();
        for node in nodes {
            for word in node.title.split_whitespace() {
                let key = word.to_lowercase();
                self.cache.entry(key).or_default().push(node.id);
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<Uuid> {
        let key = query.to_lowercase();
        self.cache.get(&key).cloned().unwrap_or_default()
    }
}
