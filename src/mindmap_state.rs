use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum MindmapLayout {
    Radial,
    Tree,
    Timeline,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Debug)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Uuid,
    pub layout: MindmapLayout,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
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

        for i in 1..=4 {
            let id = Uuid::new_v4();
            let label = format!("Child {}", i);
            let child = Node {
                id,
                label,
                parent: Some(root_id),
                children: vec![],
                tags: vec![],
                meta: HashMap::new(),
            };
            nodes.get_mut(&root_id).unwrap().children.push(id);
            nodes.insert(id, child);
        }

        Self {
            nodes,
            root_id,
            layout: MindmapLayout::Radial,
            selected: Some(root_id),
            editing: None,
            edit_buffer: String::new(),
        }
    }

    pub fn toggle_layout(&mut self) {
        self.layout = match self.layout {
            MindmapLayout::Radial => MindmapLayout::Tree,
            MindmapLayout::Tree => MindmapLayout::Timeline,
            MindmapLayout::Timeline => MindmapLayout::Radial,
        };
    }

    pub fn start_edit(&mut self) {
        if let Some(id) = self.selected {
            if let Some(node) = self.nodes.get(&id) {
                self.editing = Some(id);
                self.edit_buffer = node.label.clone();
            }
        }
    }

    pub fn cancel_edit(&mut self) {
        self.editing = None;
        self.edit_buffer.clear();
    }

    pub fn commit_edit(&mut self) {
        if let Some(id) = self.editing {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.label = self.edit_buffer.clone();
            }
        }
        self.cancel_edit();
    }

    pub fn push_edit_char(&mut self, c: char) {
        self.edit_buffer.push(c);
    }

    pub fn pop_edit_char(&mut self) {
        self.edit_buffer.pop();
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
}