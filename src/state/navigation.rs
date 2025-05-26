use super::core::AppState;
use crate::node::NodeID;

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
