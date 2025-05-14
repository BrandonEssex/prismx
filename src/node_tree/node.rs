#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub label: String,
    pub children: Vec<Node>,
    pub editing: bool,
}

impl Node {
    pub fn new(id: usize, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
            children: vec![],
            editing: false,
        }
    }
}

pub struct NodeTree {
    pub nodes: Vec<Node>,
    pub selected_index: usize,
    pub next_id: usize,
}

impl Default for NodeTree {
    fn default() -> Self {
        let root = Node::new(0, "Root");
        Self {
            nodes: vec![root],
            selected_index: 0,
            next_id: 1,
        }
    }
}

impl NodeTree {
    pub fn with_mock_data() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn begin_editing_selected(&mut self) {
        if let Some(node) = self.nodes.get_mut(self.selected_index) {
            node.editing = true;
        }
    }

    pub fn stop_editing(&mut self) {
        if let Some(node) = self.nodes.get_mut(self.selected_index) {
            node.editing = false;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(node) = self.nodes.get_mut(self.selected_index) {
            if node.editing {
                node.label.push(c);
            }
        }
    }

    pub fn backspace_char(&mut self) {
        if let Some(node) = self.nodes.get_mut(self.selected_index) {
            if node.editing {
                node.label.pop();
            }
        }
    }

    pub fn create_child_node(&mut self) {
        let new_node = Node::new(self.next_id, "New Node");
        self.next_id += 1;
        self.nodes.push(new_node);
        self.selected_index = self.nodes.len() - 1;
    }

    pub fn delete_selected(&mut self) {
        if self.nodes.len() > 1 {
            self.nodes.remove(self.selected_index);
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }
}
