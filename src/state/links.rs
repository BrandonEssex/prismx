use super::core::AppState;
use crate::node::NodeID;
use std::collections::HashMap;

impl AppState {
    /// Build a mapping of target nodes to all sources referencing them.
    pub fn inbound_links(&self) -> HashMap<NodeID, Vec<NodeID>> {
        let mut map: HashMap<NodeID, Vec<NodeID>> = HashMap::new();
        for (id, node) in &self.nodes {
            if let Some(pid) = node.parent {
                map.entry(*id).or_default().push(pid);
            }
        }
        for (src, targets) in &self.link_map {
            for tgt in targets {
                map.entry(*tgt).or_default().push(*src);
            }
        }
        map
    }
}
