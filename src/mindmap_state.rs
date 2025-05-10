use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
}

#[derive(Debug)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Option<Uuid>,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
}

impl MindmapState {
    pub fn new() -> Self {
        let root_id = Uuid::new_v4();
        let root = Node {
            id: root_id,
            label: "Root".into(),
            parent: None,
            children: vec![],
        };

        let mut nodes = HashMap::new();
        nodes.insert(root_id, root);

        Self {
            nodes,
            root_id: Some(root_id),
            selected: Some(root_id),
            editing: None,
            edit_buffer: String::new(),
        }
    }

    pub fn start_edit(&mut self) {
        if let Some(id) = self.selected {
            self.editing = Some(id);
            self.edit_buffer = self.nodes.get(&id).map(|n| n.label.clone()).unwrap_or_default();
        }
    }

    pub fn push_edit_char(&mut self, c: char) {
        self.edit_buffer.push(c);
    }

    pub fn pop_edit_char(&mut self) {
        self.edit_buffer.pop();
    }

    pub fn commit_edit(&mut self) {
        if let Some(id) = self.editing.take() {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.label = self.edit_buffer.clone();
            }
        }
        self.edit_buffer.clear();
    }

    pub fn cancel_edit(&mut self) {
        self.editing = None;
        self.edit_buffer.clear();
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected {
            let mut all_ids: Vec<_> = self.nodes.keys().cloned().collect();
            all_ids.sort();
            if let Some(idx) = all_ids.iter().position(|id| id == &selected) {
                let next = all_ids.get((idx + 1) % all_ids.len()).cloned();
                self.selected = next;
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(selected) = self.selected {
            let mut all_ids: Vec<_> = self.nodes.keys().cloned().collect();
            all_ids.sort();
            if let Some(idx) = all_ids.iter().position(|id| id == &selected) {
                let prev = if idx == 0 { all_ids.len() - 1 } else { idx - 1 };
                self.selected = all_ids.get(prev).cloned();
            }
        }
    }

    pub fn create_sibling(&mut self) {
        if let Some(current_id) = self.selected {
            let new_id = Uuid::new_v4();
            let new_node = Node {
                id: new_id,
                label: "New Node".into(),
                parent: self.nodes.get(&current_id).and_then(|n| n.parent),
                children: vec![],
            };
            if let Some(parent_id) = new_node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.push(new_id);
                }
            }
            self.nodes.insert(new_id, new_node);
            self.selected = Some(new_id);
            self.start_edit();
        }
    }

    pub fn create_child(&mut self) {
        if let Some(current_id) = self.selected {
            let new_id = Uuid::new_v4();
            let new_node = Node {
                id: new_id,
                label: "Child Node".into(),
                parent: Some(current_id),
                children: vec![],
            };
            if let Some(parent) = self.nodes.get_mut(&current_id) {
                parent.children.push(new_id);
            }
            self.nodes.insert(new_id, new_node);
            self.selected = Some(new_id);
            self.start_edit();
        }
    }
}