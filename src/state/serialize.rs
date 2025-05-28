use serde::{Serialize, Deserialize};
use crate::state::core::AppState;
use crate::node::{Node, NodeID};
use alloc::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone)]
struct NodeData {
    id: NodeID,
    label: String,
    parent: Option<NodeID>,
    children: Vec<NodeID>,
    collapsed: bool,
    x: i16,
    y: i16,
}

impl From<&Node> for NodeData {
    fn from(n: &Node) -> Self {
        Self {
            id: n.id,
            label: n.label.clone(),
            parent: n.parent,
            children: n.children.clone(),
            collapsed: n.collapsed,
            x: n.x,
            y: n.y,
        }
    }
}

impl From<NodeData> for Node {
    fn from(d: NodeData) -> Self {
        Self {
            id: d.id,
            label: d.label,
            parent: d.parent,
            children: d.children,
            collapsed: d.collapsed,
            x: d.x,
            y: d.y,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PersistedLayout {
    pub nodes: Vec<NodeData>,
    pub root_nodes: Vec<NodeID>,
    pub selected: Option<NodeID>,
    pub zoom: f32,
    pub scroll_x: i16,
    pub scroll_y: i16,
    pub drawing_root: Option<NodeID>,
}

pub fn capture(state: &AppState) -> PersistedLayout {
    PersistedLayout {
        nodes: state.nodes.values().map(NodeData::from).collect(),
        root_nodes: state.root_nodes.clone(),
        selected: state.selected,
        zoom: state.zoom_scale,
        scroll_x: state.scroll_x,
        scroll_y: state.scroll_y,
        drawing_root: state.drawing_root,
    }
}

pub fn apply(state: &mut AppState, snap: PersistedLayout) {
    state.nodes = snap
        .nodes
        .into_iter()
        .map(|n| (n.id, Node::from(n)))
        .collect::<BTreeMap<_, _>>();
    state.root_nodes = snap.root_nodes;
    state.selected = snap.selected;
    state.zoom_scale = snap.zoom;
    state.scroll_x = snap.scroll_x;
    state.scroll_y = snap.scroll_y;
    state.drawing_root = snap.drawing_root;
}
