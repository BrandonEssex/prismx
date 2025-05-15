use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceNode {
    pub id: String,
    pub content: String,
    pub children: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MindTrace {
    pub nodes: HashMap<String, TraceNode>,
    pub root_id: String,
    pub counter: usize,
}

impl MindTrace {
    pub fn new() -> Self {
        let mut mt = Self {
            nodes: HashMap::new(),
            root_id: "root".to_string(),
            counter: 1,
        };
        mt.add_node("root", "Root");
        mt
    }

    pub fn add_node(&mut self, parent_id: &str, content: &str) -> String {
        let id = format!("n{}", self.counter);
        self.counter += 1;
        let node = TraceNode {
            id: id.clone(),
            content: content.to_string(),
            children: Vec::new(),
        };
        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(id.clone());
        }
        self.nodes.insert(id.clone(), node);
        id
    }

    pub fn get_node_text(&self, id: &str) -> String {
        self.nodes.get(id).map(|n| n.content.clone()).unwrap_or_default()
    }
}
