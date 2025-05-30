use super::core::AppState;

impl AppState {
    pub fn move_focus_up(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos > 0 {
                    self.set_selected(Some(order[pos - 1]));
                }
            }
        }
    }

    pub fn move_focus_down(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos + 1 < order.len() {
                    self.set_selected(Some(order[pos + 1]));
                }
            }
        }
    }

    pub fn move_focus_left(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if let Some(parent_id) = node.parent {
                    self.set_selected(Some(parent_id));
                }
            }
        }
    }

    pub fn move_focus_right(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if !node.children.is_empty() {
                    self.set_selected(Some(node.children[0]));
                }
            }
        }
    }

    /// Move focus to the previous sibling if available.
    pub fn focus_prev_sibling(&mut self) {
        if let Some(current) = self.selected {
            if let Some(parent_id) = self.nodes.get(&current).and_then(|n| n.parent) {
                if let Some(parent) = self.nodes.get(&parent_id) {
                    if let Some(pos) = parent.children.iter().position(|&c| c == current) {
                        if pos > 0 {
                            self.set_selected(Some(parent.children[pos - 1]));
                        }
                    }
                }
            } else if let Some(pos) = self.root_nodes.iter().position(|&r| r == current) {
                if pos > 0 {
                    self.set_selected(Some(self.root_nodes[pos - 1]));
                }
            }
        }
    }

    /// Move focus to the next sibling if available.
    pub fn focus_next_sibling(&mut self) {
        if let Some(current) = self.selected {
            if let Some(parent_id) = self.nodes.get(&current).and_then(|n| n.parent) {
                if let Some(parent) = self.nodes.get(&parent_id) {
                    if let Some(pos) = parent.children.iter().position(|&c| c == current) {
                        if pos + 1 < parent.children.len() {
                            self.set_selected(Some(parent.children[pos + 1]));
                        }
                    }
                }
            } else if let Some(pos) = self.root_nodes.iter().position(|&r| r == current) {
                if pos + 1 < self.root_nodes.len() {
                    self.set_selected(Some(self.root_nodes[pos + 1]));
                }
            }
        }
    }

    pub fn drill_down(&mut self) {
        if let Some(current) = self.get_selected_node() {
            if !current.children.is_empty() {
                self.set_selected(Some(current.children[0]));
            }
        }
    }

    pub fn drill_selected(&mut self) {
        if let Some(id) = self.selected {
            if self.nodes.contains_key(&id) {
                self.view_stack.push(self.drawing_root.take());
                self.drawing_root = Some(id);
                self.scroll_x = 0;
                self.scroll_y = 0;
                self.zoom_scale = 1.0;
            } else {
                tracing::warn!("❌ Drill failed: selected node not found.");
            }
        }
        self.fallback_this_frame = false;
        self.clear_fallback_promotions();
    }

    pub fn pop_stack(&mut self) {
        if let Some(prev) = self.view_stack.pop() {
            if let Some(id) = prev {
                if self.nodes.contains_key(&id) {
                    self.drawing_root = Some(id);
                    self.scroll_x = 0;
                    self.scroll_y = 0;
                    self.zoom_scale = 1.0;
                    self.fallback_this_frame = false;
                    self.clear_fallback_promotions();
                    return;
                }
            }
        }

        self.drawing_root = None;
        self.scroll_x = 0;
        self.scroll_y = 0;
        self.zoom_scale = 1.0;
        self.fallback_this_frame = false;
        self.clear_fallback_promotions();
    }
}
