use crate::state::AppState;
use crate::node::{NodeID, NodeMap};
use crate::layout::{layout_nodes, Coords};
use std::collections::HashMap;

/// Toggle snap-to-grid mode
pub fn toggle_snap(state: &mut AppState) {
    state.toggle_snap_grid();
}

/// Determine which node is at the given coordinates considering current layout.
pub fn node_at_position(state: &AppState, x: u16, y: u16) -> Option<NodeID> {
    let mut layout = HashMap::new();
    if state.auto_arrange {
        let roots = if let Some(drill) = state.drawing_root {
            vec![drill]
        } else {
            state.root_nodes.clone()
        };
        let mut row = 1;
        for &root_id in &roots {
            let l = layout_nodes(&state.nodes, root_id, 2, row as i16);
            let max_y = l.values().map(|c| c.y).max().unwrap_or(row);
            layout.extend(l);
            row = max_y.saturating_add(3);
        }
    } else {
        for (id, node) in &state.nodes {
            layout.insert(*id, Coords { x: node.x as u16, y: node.y as u16 });
        }
    }

    for (&id, &Coords { x: nx, y: ny }) in &layout {
        if ny == y {
            let node = &state.nodes[&id];
            let start_x = nx.saturating_sub(state.scroll_x.max(0) as u16);
            let end_x = start_x + node.label.len() as u16 + 2;
            if x >= start_x && x < end_x {
                return Some(id);
            }
        }
    }
    None
}

/// Begin dragging the specified node from mouse coords.
pub fn start_drag(state: &mut AppState, id: NodeID, x: u16, y: u16) {
    state.dragging = Some(id);
    state.last_mouse = Some((x, y));
    state.selected = Some(id);
}

/// Update dragging node position based on new mouse coords.
pub fn drag_update(state: &mut AppState, x: u16, y: u16) {
    if let (Some(id), Some((lx, ly))) = (state.dragging, state.last_mouse) {
        let dx = x as i16 - lx as i16;
        let dy = y as i16 - ly as i16;
        drag_recursive(id, dx, dy, &mut state.nodes, state.snap_to_grid);
        state.last_mouse = Some((x, y));
    }
}

/// Stop current drag operation.
pub fn end_drag(state: &mut AppState) {
    state.dragging = None;
    state.last_mouse = None;
}

/// Drag a node and its children recursively.
fn drag_recursive(id: NodeID, dx: i16, dy: i16, nodes: &mut NodeMap, snap: bool) {
    if let Some(node) = nodes.get_mut(&id) {
        node.x += dx;
        node.y += dy;
        if snap {
            node.x = snap_value(node.x);
            node.y = snap_value(node.y);
        }
        let children = node.children.clone();
        for child in children {
            drag_recursive(child, dx, dy, nodes, snap);
        }
    }
}

/// Snap to nearest 20px grid unit.
fn snap_value(v: i16) -> i16 {
    ((v + 10) / 20) * 20
}
