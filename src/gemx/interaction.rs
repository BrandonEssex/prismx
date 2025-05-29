use crate::state::AppState;
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{
    layout_nodes, Coords, SIBLING_SPACING_X, CHILD_SPACING_Y,
    GEMX_HEADER_HEIGHT, PackRegion, subtree_span,
    subtree_depth, spacing_for_zoom,
};
use crossterm::terminal;
use std::collections::HashMap;
use std::time::Instant;

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
        use std::collections::HashSet;

        let (tw, th) = terminal::size().unwrap_or((80, 20));
        let used: HashSet<(i16, i16)> =
            state.nodes.values().map(|n| (n.x, n.y)).collect();

        let base_x = 6;
        let base_y = GEMX_HEADER_HEIGHT + 1;
        let step_x = SIBLING_SPACING_X * 2;
        let step_y = CHILD_SPACING_Y * 2;

        let mut x = base_x;
        let mut y = base_y;
        let max_y = th as i16 - 2;
        let max_x = tw as i16 - 4;

        while used.contains(&(x, y)) {
            y += step_y;
            if y > max_y {
                y = base_y;
                x += step_x;
                if x > max_x {
                    x = base_x;
                }
            }
        }

        node.x = x;
        node.y = y;
    }

    crate::log_debug!(
        state,
        "[Node {}] label=\"{}\", parent={:?}, x={}, y={}",
        new_id,
        node.label,
        node.parent,
        node.x,
        node.y
    );
    if state.debug_input_mode {
        eprintln!("📦 Free node {} at x={}, y={}", new_id, node.x, node.y);
    }

    state.nodes.insert(new_id, node);
    state.root_nodes.push(new_id);
    state.set_selected(Some(new_id));

    // Graph integrity + layout role refresh
    crate::layout::roles::recalculate_roles(state);
    state.ensure_valid_roots();
    state.audit_node_graph();
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
        let (tw, th) = terminal::size().unwrap_or((80, 20));
        let mut pack = PackRegion::new(tw as i16, GEMX_HEADER_HEIGHT);
        for &root_id in &roots {
            let w = subtree_span(&state.nodes, root_id);
            let h = subtree_depth(&state.nodes, root_id) * CHILD_SPACING_Y + 1;
            let (ox, oy) = pack.insert((w, h));
            let (mut l, _roles) = layout_nodes(
                &state.nodes,
                root_id,
                oy,
                tw as i16,
                th as i16,
                state.auto_arrange,
            );
            for pos in l.values_mut() {
                pos.x += ox;
            }
            layout.extend(l);
        }
    } else {
        for (id, node) in &state.nodes {
            layout.insert(*id, Coords { x: node.x, y: node.y });
        }
    }

    for (&id, &Coords { x: nx, y: ny }) in &layout {
        let zoom = state.zoom_scale as f32;
        let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
        let draw_x = ((nx as f32 - state.scroll_x as f32) * bsx as f32 * zoom)
            .round()
            .max(0.0) as u16;
        let draw_y = ((ny as f32 - state.scroll_y as f32) * bsy as f32 * zoom)
            .round()
            .max(0.0) as u16;

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
    if state.auto_arrange {
        state.status_message = "Drag disabled while auto-arrange is enabled".into();
        state.status_message_last_updated = Some(Instant::now());
        crate::log_debug!(state, "drag attempt blocked: auto-arrange active");
        return;
    }

    if state.nodes.get(&id).and_then(|n| n.parent).is_some() {
        tracing::debug!("ignored drag on child node {}", id);
        return;
    }
    state.dragging = Some(id);
    let zoom = state.zoom_scale as f32;
    let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
    let wx = (state.scroll_x as f32 + (x as f32 / (bsx as f32 * zoom))).round() as i16;
    let wy = (state.scroll_y as f32 + (y as f32 / (bsy as f32 * zoom))).round() as i16;
    state.last_mouse = Some((wx, wy));
    state.set_selected(Some(id));
}

/// Update dragging node position based on new mouse coords.
pub fn drag_update(state: &mut AppState, x: u16, y: u16) {
    if state.auto_arrange {
        return;
    }
    let zoom = state.zoom_scale as f32;
    let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
    let wx = (state.scroll_x as f32 + (x as f32 / (bsx as f32 * zoom))).round() as i16;
    let wy = (state.scroll_y as f32 + (y as f32 / (bsy as f32 * zoom))).round() as i16;
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
    if state.link_mode {
        crate::layout::roles::recalculate_roles(state);
        state.link_mode = false;
    } else {
        crate::log_debug!(state, "skipping reparent after drag (link_mode off)");
    }
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
