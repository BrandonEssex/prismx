use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{layout_nodes, Coords};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use crate::gemx::layout::{spacing_for, DEFAULT_SPACING_PROFILE};
use std::collections::HashMap;

pub fn render_gemx<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let scale = state.zoom_scale;
    let title = if state.auto_arrange {
        format!("Gemx [Auto-Arrange] {:.1}x", scale)
    } else {
        format!("Gemx {:.1}x", scale)
    };
    let block = Block::default().title(title).borders(Borders::ALL);
    f.render_widget(block, area);

    let roots = if let Some(drill_root) = state.drawing_root {
        vec![drill_root]
    } else {
        state.root_nodes.clone()
    };

    let mut drawn_at = HashMap::new();
    let mut y = 1u16;

    if state.auto_arrange {
        for &root_id in &roots {
            let layout = layout_nodes(&state.nodes, root_id, 2, y, DEFAULT_SPACING_PROFILE);
            let max_y = layout.values().map(|c| c.y).max().unwrap_or(y);
            drawn_at.extend(layout);
            y = max_y.saturating_add(3);
        }
    } else {
        for &root_id in &roots {
            let layout = layout_nodes(&state.nodes, root_id, 2, y, DEFAULT_SPACING_PROFILE);
            let max_y = layout.values().map(|c| c.y).max().unwrap_or(y);
            for (id, coord) in layout {
                let node = &state.nodes[&id];
                let coords = if node.is_positioned {
                    Coords { x: node.x as u16, y: node.y as u16 }
                } else {
                    coord
                };
                drawn_at.insert(id, coords);
            }
            y = max_y.saturating_add(3);
        }
    }

    let spacing = spacing_for(DEFAULT_SPACING_PROFILE);
    for (&node_id, &Coords { x, y }) in &drawn_at {
        let xs = ((x as f32) * spacing.x as f32 * scale) as u16;
        let ys = ((y as f32) * spacing.y as f32 * scale) as u16;
        if ys >= area.height {
            continue;
        }

        let node = &state.nodes[&node_id];
        let is_selected = Some(node_id) == state.selected;

        let parent_glyph = if let Some(parent_id) = node.parent {
            let parent_y = drawn_at.get(&parent_id).map(|c| c.y).unwrap_or(y.saturating_sub(1));
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
        if state.link_map.get(&node_id).and_then(|v| v.first()).is_some() {
            label.push_str(" 📎");
        }

        let scroll_x = ((state.scroll_x.max(0) as f32) * spacing.x as f32 * scale) as u16;
        let width = label.len().min((area.width - xs.saturating_sub(scroll_x)) as usize);

        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(xs.saturating_sub(scroll_x), ys, width as u16, 1));
    }

    for (source, targets) in &state.link_map {
        for target in targets {
            if let (Some(&Coords { x: sx, y: sy }), Some(&Coords { x: tx, y: ty })) = (drawn_at.get(source), drawn_at.get(target)) {
                let sys = ((sy as f32) * spacing.y as f32 * scale) as u16;
                let sxs = ((sx as f32) * spacing.x as f32 * scale) as u16;
                let txs = ((tx as f32) * spacing.x as f32 * scale) as u16;
                if sy == ty && sys < area.height {
                    let arrow = if sx < tx { "→" } else { "←" };
                    let mid = (sxs + txs) / 2;
                    let scroll_x = ((state.scroll_x.max(0) as f32) * spacing.x as f32 * scale) as u16;
                    if mid >= scroll_x && mid < scroll_x + area.width {
                        let para = Paragraph::new(arrow);
                        f.render_widget(para, Rect::new(mid.saturating_sub(scroll_x), sys, 1, 1));
                    }
                }
            }
        }
    }
}
