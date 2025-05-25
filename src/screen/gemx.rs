use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{
    layout_nodes, Coords, LayoutRole, PackRegion, GEMX_HEADER_HEIGHT,
    CHILD_SPACING_Y, subtree_span, subtree_depth, spacing_for_zoom,
    BASE_SPACING_X, BASE_SPACING_Y, SIBLING_SPACING_X, SNAP_GRID_X, SNAP_GRID_Y,
};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

pub fn render_gemx<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let style = style_for_mode(&state.mode);
    let block = Block::default()
        .title(if state.auto_arrange { "Gemx [Auto-Arrange]" } else { "Gemx" })
        .borders(Borders::NONE);
    f.render_widget(block, area);

    if state.debug_input_mode && std::env::var("PRISMX_TEST").is_err() {
        let dot = Paragraph::new("·").style(Style::default().fg(Color::DarkGray));
        for gx in (0..area.width).step_by(SNAP_GRID_X as usize) {
            for gy in (0..area.height).step_by(SNAP_GRID_Y as usize) {
                let rect = Rect::new(area.x + gx, area.y + gy, 1, 1);
                f.render_widget(dot.clone(), rect);
            }
        }
    }

    // Reset unreachable fallback lock for this frame
    state.fallback_this_frame = false;

    if let Some(sel) = state.selected {
        if !state.nodes.contains_key(&sel) {
            state.selected = None;
        }
    }
    if state.selected.is_none() {
        return;
    }

    // Ensure we always have valid root nodes before any layout logic
    state.ensure_valid_roots();
    if state.auto_arrange {
        crate::layout::roles::recalculate_roles(state);
    }

    // Validate again in case role recalculation removed all roots
    state.ensure_valid_roots();
    if state.root_nodes.is_empty() {
        f.render_widget(
            Paragraph::new("⚠ No valid root nodes."),
            Rect::new(area.x + 2, area.y + 2, 40, 1),
        );
        return;
    }


    // // ✅ Always print the structure for diagnostics
    // println!("=== NODES AND CHILDREN ===");
    // for (id, node) in &state.nodes {
    //     println!("Node {} → parent: {:?}, children: {:?}", id, node.parent, node.children);
    // }

    let roots = if let Some(drill_root) = state.drawing_root {
        vec![drill_root]
    } else {
        state.root_nodes.clone()
    };

    let mut drawn_at = HashMap::new();
    let mut node_roles = HashMap::new();
    if state.auto_arrange {
        let mut pack = PackRegion::new(i16::MAX, GEMX_HEADER_HEIGHT);
        for &root_id in &roots {
            let w = subtree_span(&state.nodes, root_id);
            let h = subtree_depth(&state.nodes, root_id) * CHILD_SPACING_Y + 1;
            let (ox, oy) = pack.insert((w, h));

            let (mut layout, roles) = layout_nodes(
                &state.nodes,
                root_id,
                oy,
                area.width as i16,
                area.height as i16,
                state.auto_arrange,
            );
            if layout.is_empty() {
                crate::log_debug!(state, "skipping root {} - incomplete tree", root_id);
                state.layout_fail_count += 1;
                if state.layout_fail_count >= 3 {
                    state.auto_arrange = false;
                    state.layout_fail_count = 0;
                    crate::log_debug!(state, "\u{274c} auto-arrange disabled due to failures");
                }
                continue;
            } else {
                state.layout_fail_count = 0;
            }
            for pos in layout.values_mut() {
                pos.x += ox;
            }
            drawn_at.extend(layout);
            node_roles.extend(roles);
        }
    } else {
        fn collect(nodes: &NodeMap, id: NodeID, out: &mut HashMap<NodeID, Coords>) {
            if let Some(n) = nodes.get(&id) {
                out.insert(id, Coords { x: n.x, y: n.y });
                if !n.collapsed {
                    for child in &n.children {
                        collect(nodes, *child, out);
                    }
                }
            }
        }
        for &root_id in &roots {
            collect(&state.nodes, root_id, &mut drawn_at);
            let (_, roles) = layout_nodes(
                &state.nodes,
                root_id,
                0,
                area.width as i16,
                area.height as i16,
                state.auto_arrange,
            );
            node_roles.extend(roles);
        }

    }

    // Ensure that every declared root node is represented in the drawn layout.
    for &root_id in &state.root_nodes {
        if !drawn_at.contains_key(&root_id) {
            if let Some(n) = state.nodes.get_mut(&root_id) {
                if n.x == 0 && n.y == 0 {
                    n.x = 6;
                    n.y = GEMX_HEADER_HEIGHT + 3;
                }
                drawn_at.insert(root_id, Coords { x: n.x, y: n.y });
                node_roles.insert(root_id, LayoutRole::Root);
            }
        }
    }


    if drawn_at.is_empty() {
        let msg = if state.auto_arrange {
            "⚠ Node exists but layout failed."
        } else {
            "⚠ No valid root nodes."
        };
        f.render_widget(Paragraph::new(msg), area);
        return;
    }

    use std::collections::HashSet;
    let reachable_ids: HashSet<NodeID> = drawn_at.keys().copied().collect();

    if state.auto_arrange {
        let node_ids: Vec<NodeID> = state.nodes.keys().copied().collect();
        for id in node_ids {
            if state.fallback_this_frame {
                continue;
            }
            let node = match state.nodes.get(&id) {
                Some(n) => n,
                None => continue,
            };
            if state.root_nodes.contains(&id)
                || drawn_at.contains_key(&id)
                || reachable_ids.contains(&id)
                || state.fallback_promoted_this_session.contains(&id)
            {
                continue;
            }
            if node.children.is_empty() {
                continue;
            }

            state.root_nodes.push(id);
            state.root_nodes.sort_unstable();
            state.root_nodes.dedup();
            state.fallback_this_frame = true;
            state.fallback_promoted_this_session.insert(id);

            let Some(n) = state.nodes.get_mut(&id) else {
                crate::log_debug!(state, "❌ Fallback failed: Node {} not found.", id);
                return;
            };

            if n.x == 0 && n.y == 0 {
                n.x = state.fallback_next_x;
                n.y = state.fallback_next_y;
                state.fallback_next_y += 3;
                if state.fallback_next_y > area.height as i16 - 4 {
                    state.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
                    state.fallback_next_x += 20;
                }
                crate::log_debug!(state, "📦 Placed Node {} at x={}, y={}", id, n.x, n.y);
            }

            drawn_at.insert(id, Coords { x: n.x, y: n.y });
            node_roles.insert(id, LayoutRole::Root);

            crate::log_debug!(state, "🚨 Promoted Node {} to root (label-safe)", id);
            break;
        }
    }

    for (&id, _) in &state.nodes {
        if !drawn_at.contains_key(&id) {
            drawn_at.insert(id, Coords { x: 3, y: GEMX_HEADER_HEIGHT + 2 });
            node_roles.insert(id, LayoutRole::Free);
        }
    }

    // if state.debug_input_mode {
    //     eprintln!("Rendered {} nodes this frame.", drawn_at.len());
    // }

    // When auto-arrange is active, adjust zoom and scroll to fit all nodes
    if state.auto_arrange && !drawn_at.is_empty() {
        let min_x = drawn_at.values().map(|c| c.x).min().unwrap_or(0);
        let max_x = drawn_at.values().map(|c| c.x).max().unwrap_or(0);
        let min_y = drawn_at.values().map(|c| c.y).min().unwrap_or(0);
        let max_y = drawn_at.values().map(|c| c.y).max().unwrap_or(0);

        let layout_width = (max_x - min_x + 1) as f32 * BASE_SPACING_X as f32;
        let layout_height = (max_y - min_y + 1) as f32 * BASE_SPACING_Y as f32;

        let mut zoom = (area.width as f32 / layout_width)
            .min(area.height as f32 / layout_height)
            .min(2.0);
        if !state.zoom_locked_by_user {
            state.zoom_scale = zoom.max(0.5);
        } else {
            zoom = state.zoom_scale;
        }

        let (bsx, bsy) = spacing_for_zoom(zoom);
        let center_x = (min_x + max_x) as f32 / 2.0;
        let center_y = (min_y + max_y) as f32 / 2.0;

        state.scroll_x =
            (center_x - (area.width as f32 / (bsx as f32 * zoom)) / 2.0).round() as i16;
        state.scroll_y =
            (center_y - (area.height as f32 / (bsy as f32 * zoom)) / 2.0).round() as i16;

        if state.scroll_x < 0 {
            state.scroll_x = 0;
        }
        if state.scroll_y < 0 {
            state.scroll_y = 0;
        }
    }

    for (&node_id, &Coords { x, y }) in &drawn_at {
        let role = node_roles
            .get(&node_id)
            .copied()
            .unwrap_or(LayoutRole::Child);
        if role == LayoutRole::Ghost {
            continue;
        }
        let zoom = state.zoom_scale as f32;
        let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
        let draw_x = ((x as f32 - state.scroll_x as f32) * bsx as f32 * zoom)
            .round()
            .max(0.0) as u16;
        let draw_y = ((y as f32 - state.scroll_y as f32) * bsy as f32 * zoom)
            .round()
            .max(0.0) as u16;

        if draw_x >= area.width || draw_y >= area.height {
            #[cfg(debug_assertions)]
            tracing::debug!("clamp node ({},{})", draw_x, draw_y);
            continue;
        }

        let node = &state.nodes[&node_id];
        let is_selected = Some(node_id) == state.selected;

        let parent_glyph = if let Some(parent_id) = node.parent {
            let parent_y = drawn_at.get(&parent_id).map(|c| c.y).unwrap_or(y - 1);
            if parent_y < y { Some("│ ") } else { Some("  ") }
        } else { None };

        let elbow_glyph = if let Some(parent_id) = node.parent {
            let siblings = &state.nodes[&parent_id].children;
            if let Some(last) = siblings.last() {
                if *last == node_id { Some("└─") } else { Some("├─") }
            } else { None }
        } else { None };

        let mut label = String::new();
        if is_selected { label.push_str("> "); } else { label.push_str("  "); }
        if let Some(glyph) = parent_glyph { label.push_str(glyph); }
        if let Some(elbow) = elbow_glyph { label.push_str(elbow); }
        label.push_str(&node.label);
        if role == LayoutRole::Free {
            label.push_str(" 🧩");
        }
        if state.link_map.get(&node_id).and_then(|v| v.first()).is_some() {
            label.push_str(" 📎");
        }

        let unreachable = !reachable_ids.contains(&node_id);
        if state.debug_input_mode && unreachable {
            label = format!("❓ {}", label);
        }

        let width = label.len().min((area.width - draw_x) as usize);

        let mut style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };
        if !is_selected {
            match role {
                LayoutRole::Root => {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                }
                LayoutRole::Orphan => {
                    style = style.add_modifier(Modifier::DIM | Modifier::UNDERLINED);
                }
                _ => {}
            }
        }

        if unreachable {
            style = style.fg(Color::DarkGray);
        }

        if state.debug_input_mode {
            let has_parent = node.parent.is_some();
            if !has_parent && !matches!(role, LayoutRole::Root | LayoutRole::Free) {
                style = style.bg(Color::Red);
            }
        }

        let para = Paragraph::new(label).style(style);
        let safe_width = std::cmp::min(width as u16, area.width);
        let safe_rect = Rect::new(
            draw_x.min(area.width.saturating_sub(1)),
            draw_y.min(area.height.saturating_sub(1)),
            safe_width,
            1u16.min(area.height),
        );
        f.render_widget(para, safe_rect);
        if state.debug_input_mode && std::env::var("PRISMX_TEST").is_err() {
            let mark_rect = Rect::new(
                draw_x.min(area.width.saturating_sub(1)),
                draw_y.min(area.height.saturating_sub(1)),
                1u16.min(area.width),
                1u16.min(area.height),
            );
            f.render_widget(Paragraph::new("■").style(style), mark_rect);
            if x == 0 {
                let r = Rect::new(1, draw_y.min(area.height.saturating_sub(1)), 1, 1);
                f.render_widget(Paragraph::new("■"), r);
            }
        }
    }

    if let Some(id) = state.drawing_root {
        if let Some(coords) = drawn_at.get(&id) {
            let zoom = state.zoom_scale as f32;
            let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
            let hx = ((coords.x as f32 - state.scroll_x as f32) * bsx as f32 * zoom)
                .round()
                .max(0.0) as u16;
            let hy = ((coords.y as f32 - state.scroll_y as f32) * bsy as f32 * zoom)
                .round()
                .max(0.0) as u16;
            if hx < area.width && hy < area.height {
                let mark = Paragraph::new("◆").style(Style::default().fg(Color::Magenta));
                let rect = Rect::new(hx, hy, 1u16.min(area.width), 1u16.min(area.height));
                f.render_widget(mark, rect);
            }
        }
    }

    // Draw arrows
    for (source, targets) in &state.link_map {
        for target in targets {
            if let (Some(&Coords { x: sx, y: sy }), Some(&Coords { x: tx, y: ty })) =
                (drawn_at.get(source), drawn_at.get(target))
            {
                if sy == ty {
                    let zoom = state.zoom_scale as f32;
                    let (bsx, bsy) = spacing_for_zoom(state.zoom_scale);
                    let sxp = ((sx as f32 - state.scroll_x as f32) * bsx as f32 * zoom).round();
                    let txp = ((tx as f32 - state.scroll_x as f32) * bsx as f32 * zoom).round();
                    let syp = ((sy as f32 - state.scroll_y as f32) * bsy as f32 * zoom).round();
                    let arrow = if sx < tx { "→" } else { "←" };
                    let mid = ((sxp + txp) / 2.0).round().max(0.0) as u16;
                    let draw_sy = syp.max(0.0) as u16;
                    if mid >= area.width || draw_sy >= area.height {
                        #[cfg(debug_assertions)]
                        tracing::debug!("clamp arrow ({},{})", mid, draw_sy);
                        continue;
                    }
                    let para = Paragraph::new(arrow);
                    f.render_widget(para, Rect::new(mid, draw_sy, 1, 1));
                }
            }
        }
    }

    if state.auto_arrange {
        let indicator = Paragraph::new("[A] Auto-Arrange")
            .style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));
        f.render_widget(indicator, Rect::new(area.x + 1, area.y + 1, 20, 1));
    }

    render_full_border(f, area, &style, true, !state.debug_border);
    let tick = if std::env::var("PRISMX_TEST").is_ok() {
        0
    } else {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() / 300) as u64
    };
    let beamx = BeamX {
        tick,
        enabled: true,
        mode: BeamXMode::Default,
        style: BeamXStyle::from(BeamXMode::Default),
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);
    if !drawn_at.is_empty() && !state.root_nodes.is_empty() {
        state.last_promoted_root = None;
    }

    for &id in &state.root_nodes {
        if !drawn_at.contains_key(&id) {
            tracing::warn!("❌ Layout failed to render root node {}", id);
        }
    }

    if drawn_at.is_empty() {
        panic!("❌ Layout aborted: No root nodes rendered. Fallback failed.");
    }
}
