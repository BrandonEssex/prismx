use super::core::AppState;
use crate::node::NodeID;

impl AppState {
    pub fn start_drag(&mut self) {
        self.selected_drag_source = self.selected;
        self.link_mode = true;
    }

    pub fn complete_drag(&mut self, target_id: NodeID) {
        if !self.link_mode {
            tracing::debug!("drag complete attempted without link_mode");
            self.selected_drag_source = None;
            return;
        }
        self.link_mode = false;
        if let Some(source_id) = self.selected_drag_source.take() {
            if source_id == target_id {
                return; // prevent self-parenting
            }

            if let Some(old_parent_id) = self.nodes.get(&source_id).and_then(|n| n.parent) {
                if let Some(parent) = self.nodes.get_mut(&old_parent_id) {
                    parent.children.retain(|&id| id != source_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != source_id);
            }

            if let Some(source_node) = self.nodes.get_mut(&source_id) {
                source_node.parent = Some(target_id);
            }

            self.nodes.get_mut(&target_id).map(|t| t.children.push(source_id));
        }
    }

    pub fn start_link(&mut self) {
        self.selected_drag_source = self.selected;
    }

    pub fn complete_link(&mut self, target_id: NodeID) {
        if let Some(source_id) = self.selected_drag_source.take() {
            if source_id != target_id {
                self.link_map.entry(source_id).or_default().push(target_id);
            }
        }
    }
}
