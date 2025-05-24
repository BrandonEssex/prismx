use crate::state::AppState;
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{
    layout_nodes, Coords, SIBLING_SPACING_X, CHILD_SPACING_Y, FREE_GRID_COLUMNS,
    GEMX_HEADER_HEIGHT,
};
use std::collections::HashMap;

/// Toggle snap-to-grid mode
pub fn toggle_snap(state: &mut AppState) {
    state.toggle_snap_grid();
}

/// Create a new free node using the fallback grid spacing when manual mode is active.
pub fn spawn_free_node(state: &mut AppState) {
    let new_id = state
        .nodes
        .keys()
        .max()
        .copied()
        .unwrap_or(100)
        + 1;

    let mut node = Node::new(new_id, "Free Node", None);

    if !state.auto_arrange {
        let index = state.root_nodes.len();
        node.x = ((index % FREE_GRID_COLUMNS) as i16) * SIBLING_SPACING_X * 2 + 1;
        node.y = ((index / FREE_GRID_COLUMNS) as i16) * CHILD_SPACING_Y * 2
            + GEMX_HEADER_HEIGHT
            + 1;
    }

    state.nodes.insert(new_id, node);
    state.root_nodes.push(new_id);
    state.selected = Some(new_id);
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
        let (tw, _) = crossterm::terminal::size().unwrap_or((80, 20));
        let mut row = GEMX_HEADER_HEIGHT + 1;
        for &root_id in &roots {
            let l = layout_nodes(&state.nodes, root_id, row, tw as i16);
            let max_y = l.values().map(|c| c.y).max().unwrap_or(row);
            layout.extend(l);
            row = max_y.saturating_add(3);
        }
    } else {
        for (id, node) in &state.nodes {
            layout.insert(*id, Coords { x: node.x, y: node.y });
        }
    }

    for (&id, &Coords { x: nx, y: ny }) in &layout {
        let draw_x = (nx - state.scroll_x).max(0) as u16;
        let draw_y = (ny - state.scroll_y).max(0) as u16;

        if draw_y == y {
            let node = &state.nodes[&id];
            let start_x = draw_x;
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
    let wx = x as i16 + state.scroll_x;
    let wy = y as i16 + state.scroll_y;
    state.last_mouse = Some((wx, wy));
    state.selected = Some(id);
}

/// Update dragging node position based on new mouse coords.
pub fn drag_update(state: &mut AppState, x: u16, y: u16) {
    let wx = x as i16 + state.scroll_x;
    let wy = y as i16 + state.scroll_y;
    if let (Some(id), Some((lx, ly))) = (state.dragging, state.last_mouse) {
        let dx = wx - lx;
        let dy = wy - ly;
        drag_recursive(id, dx, dy, &mut state.nodes, state.snap_to_grid);
        state.last_mouse = Some((wx, wy));
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
