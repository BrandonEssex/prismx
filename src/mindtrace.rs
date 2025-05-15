use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TraceNode {
    pub id: String,
    pub content: String,
    pub children: Vec<String>,
}

#[derive(Default)]
pub struct MindTrace {
    pub nodes: HashMap<String, TraceNode>,
}

impl MindTrace {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, id: &str, content: &str) {
        let node = TraceNode {
            id: id.to_string(),
            content: content.to_string(),
            children: Vec::new(),
        };
        self.nodes.insert(id.to_string(), node);
    }

    pub fn link_nodes(&mut self, parent_id: &str, child_id: &str) {
        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(child_id.to_string());
        }
    }
}
