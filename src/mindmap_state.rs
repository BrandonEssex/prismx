use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::actions::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Option<Uuid>,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
}

impl MindmapState {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let root_id = Uuid::new_v4();

        nodes.insert(root_id, Node {
            id: root_id,
            label: "Root".to_string(),
            parent: None,
            children: vec![],
        });

        Self {
            nodes,
            root_id: Some(root_id),
            selected: Some(root_id),
            editing: None,
            edit_buffer: String::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::PushEditChar(c) => {
                if self.editing.is_some() {
                    self.edit_buffer.push(c);
                }
            }
            Action::PopEditChar => {
                if self.editing.is_some() {
                    self.edit_buffer.pop();
                }
            }
            Action::CommitEdit => {
                if let Some(id) = self.editing.take() {
                    if let Some(node) = self.nodes.get_mut(&id) {
                        node.label = self.edit_buffer.clone();
                    }
                    self.edit_buffer.clear();
                }
            }
            Action::CancelEdit => {
                self.editing = None;
                self.edit_buffer.clear();
            }
            Action::EnterEditNode => {
                if let Some(id) = self.selected {
                    if let Some(node) = self.nodes.get(&id) {
                        self.edit_buffer = node.label.clone();
                        self.editing = Some(id);
                    }
                }
            }
            Action::NavigateNext => {
                if let Some(current) = self.selected {
                    if let Some(current_node) = self.nodes.get(&current) {
                        if let Some(parent_id) = current_node.parent {
                            if let Some(parent_node) = self.nodes.get(&parent_id) {
                                let siblings = &parent_node.children;
                                if let Some(index) = siblings.iter().position(|x| *x == current) {
                                    if index + 1 < siblings.len() {
                                        self.selected = Some(siblings[index + 1]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Action::NavigatePrev => {
                if let Some(current) = self.selected {
                    if let Some(current_node) = self.nodes.get(&current) {
                        if let Some(parent_id) = current_node.parent {
                            if let Some(parent_node) = self.nodes.get(&parent_id) {
                                let siblings = &parent_node.children;
                                if let Some(index) = siblings.iter().position(|x| *x == current) {
                                    if index > 0 {
                                        self.selected = Some(siblings[index - 1]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Action::CreateSiblingNode => {
                if let Some(current) = self.selected {
                    let new_id = Uuid::new_v4();
                    let label = "New Sibling".to_string();

                    let parent = self.nodes.get(&current).and_then(|n| n.parent);
                    let node = Node {
                        id: new_id,
                        label,
                        parent,
                        children: vec![],
                    };
                    self.nodes.insert(new_id, node);

                    if let Some(parent_id) = parent {
                        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                            parent_node.children.push(new_id);
                        }
                    }
                    self.selected = Some(new_id);
                    self.editing = Some(new_id);
                    self.edit_buffer.clear();
                }
            }
            Action::CreateChildNode => {
                if let Some(current) = self.selected {
                    let new_id = Uuid::new_v4();
                    let label = "New Child".to_string();

                    let node = Node {
                        id: new_id,
                        label,
                        parent: Some(current),
                        children: vec![],
                    };
                    self.nodes.insert(new_id, node);

                    if let Some(parent_node) = self.nodes.get_mut(&current) {
                        parent_node.children.push(new_id);
                    }
                    self.selected = Some(new_id);
                    self.editing = Some(new_id);
                    self.edit_buffer.clear();
                }
            }
            _ => {}
        }
    }
}