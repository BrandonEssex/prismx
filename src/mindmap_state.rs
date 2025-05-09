use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::actions::Action;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MindmapLayout {
    Radial,
    Tree,
    Timeline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Uuid,
    pub layout: MindmapLayout,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
    pub edit_started: bool,
    pub context_open: bool,
    pub context_selection: usize,
}

impl MindmapState {
    pub fn new() -> Self {
        let root_id = Uuid::new_v4();
        let mut nodes = HashMap::new();

        let root = Node {
            id: root_id,
            label: "Root".into(),
            parent: None,
            children: vec![],
            tags: vec![],
            meta: HashMap::new(),
        };
        nodes.insert(root_id, root);

        Self {
            nodes,
            root_id,
            layout: MindmapLayout::Radial,
            selected: Some(root_id),
            editing: None,
            edit_buffer: String::new(),
            edit_started: false,
            context_open: false,
            context_selection: 0,
        }
    }

    pub fn create_sibling(&mut self) {
        if let Some(current_id) = self.selected {
            if let Some(current) = self.nodes.get(&current_id) {
                let parent_id = current.parent.or(Some(current_id));
                if let Some(parent_id) = parent_id {
                    let id = Uuid::new_v4();
                    let node = Node {
                        id,
                        label: "New Node".into(),
                        parent: Some(parent_id),
                        children: vec![],
                        tags: vec![],
                        meta: HashMap::new(),
                    };
                    self.nodes.insert(id, node);
                    self.nodes.get_mut(&parent_id).unwrap().children.push(id);
                    self.selected = Some(id);
                    self.editing = Some(id);
                    self.edit_buffer.clear();
                    self.edit_started = false;
                }
            }
        }
    }

    pub fn create_child(&mut self) {
        if let Some(current_id) = self.selected {
            let id = Uuid::new_v4();
            let node = Node {
                id,
                label: "New Child".into(),
                parent: Some(current_id),
                children: vec![],
                tags: vec![],
                meta: HashMap::new(),
            };
            self.nodes.insert(id, node);
            self.nodes.get_mut(&current_id).unwrap().children.push(id);
            self.selected = Some(id);
            self.editing = Some(id);
            self.edit_buffer.clear();
            self.edit_started = false;
        }
    }

    pub fn delete_node(&mut self) {
        if let Some(current_id) = self.selected {
            if current_id == self.root_id {
                return;
            }

            if let Some(node) = self.nodes.remove(&current_id) {
                if let Some(parent_id) = node.parent {
                    if let Some(parent) = self.nodes.get_mut(&parent_id) {
                        parent.children.retain(|&c| c != current_id);
                    }
                    self.selected = Some(parent_id);
                }
            }
        }
    }

    pub fn duplicate_node(&mut self) {
        if let Some(current_id) = self.selected {
            let clone_opt = self.nodes.get(&current_id).cloned();

            if let Some(node) = clone_opt {
                let new_id = Uuid::new_v4();
                let mut clone = node.clone();
                clone.id = new_id;
                clone.label.push_str(" Copy");
                self.nodes.insert(new_id, clone);

                if let Some(parent_id) = node.parent {
                    if let Some(parent) = self.nodes.get_mut(&parent_id) {
                        parent.children.push(new_id);
                    }
                }

                self.selected = Some(new_id);
            }
        }
    }

    pub fn start_edit(&mut self) {
        if let Some(id) = self.selected {
            if let Some(node) = self.nodes.get(&id) {
                self.editing = Some(id);
                self.edit_buffer = node.label.clone();
                self.edit_started = false;
            }
        }
    }

    pub fn push_edit_char(&mut self, c: char) {
        if !self.edit_started {
            self.edit_buffer.clear();
            self.edit_started = true;
        }
        self.edit_buffer.push(c);
    }

    pub fn pop_edit_char(&mut self) {
        self.edit_buffer.pop();
    }

    pub fn cancel_edit(&mut self) {
        self.editing = None;
        self.edit_buffer.clear();
        self.edit_started = false;
    }

    pub fn commit_edit(&mut self) {
        if let Some(id) = self.editing.take() {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.label = self.edit_buffer.trim().to_string();
            }
        }
        self.edit_buffer.clear();
        self.edit_started = false;
    }

    pub fn select_next(&mut self) {
        if let Some(current) = self.selected {
            let mut all_ids: Vec<_> = self.nodes.keys().cloned().collect();
            all_ids.sort();
            if let Some(pos) = all_ids.iter().position(|id| *id == current) {
                let next = all_ids.get((pos + 1) % all_ids.len()).copied();
                self.selected = next;
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(current) = self.selected {
            let mut all_ids: Vec<_> = self.nodes.keys().cloned().collect();
            all_ids.sort();
            if let Some(pos) = all_ids.iter().position(|id| *id == current) {
                let prev = if pos == 0 { all_ids.len() - 1 } else { pos - 1 };
                self.selected = Some(all_ids[prev]);
            }
        }
    }

    pub fn toggle_context_menu(&mut self) {
        self.context_open = !self.context_open;
        self.context_selection = 0;
    }

    pub fn toggle_layout(&mut self) {
        self.layout = match self.layout {
            MindmapLayout::Radial => MindmapLayout::Tree,
            MindmapLayout::Tree => MindmapLayout::Timeline,
            MindmapLayout::Timeline => MindmapLayout::Radial,
        };
    }

    pub fn handle_action(&mut self, action: Action) {
        use Action::*;

        match action {
            EnterEditNode => self.start_edit(),
            PushEditChar(c) => self.push_edit_char(c),
            PopEditChar => self.pop_edit_char(),
            CancelEdit => self.cancel_edit(),
            CommitEdit => self.commit_edit(),
            NavigateNext => self.select_next(),
            NavigatePrev => self.select_prev(),
            ToggleMindmapLayout => self.toggle_layout(),
            OpenContextMenu => self.toggle_context_menu(),
            CreateSiblingNode => self.create_sibling(),
            CreateChildNode => self.create_child(),
            DuplicateNode => self.duplicate_node(),
            DeleteNode => self.delete_node(),
            _ => {}
        }
    }
}