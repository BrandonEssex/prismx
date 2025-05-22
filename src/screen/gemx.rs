use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::layout::{layout_nodes, Coords};
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;

pub fn render_gemx<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &AppState,
) {
    let block = Block::default()
        .title(if state.auto_arrange { "Gemx [Auto-Arrange]" } else { "Gemx" })
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let mut drawn_at = std::collections::HashMap::new();
    let mut y = 1;

    // Use only drawing_root if set (for drill down)
    let roots: Vec<NodeID> = if let Some(root_id) = state.drawing_root {
        vec![root_id]
    } else {
        state.root_nodes.clone()
    };

    // Layout
    if state.auto_arrange {
        for &root_id in &roots {
            let layout = layout_nodes(&state.nodes, root_id, 2, y);
            y = layout.values().map(|c| c.y).max().unwrap_or(y).saturating_add(2);
            drawn_at.extend(layout);
        }
    } else {
        // Fallback: use stored Coords (freeform layout - not yet implemented)
        // Could integrate in later patch
        return;
    }

    for (&node_id, &Coords { x, y }) in &drawn_at {
        if y >= area.height {
            continue;
        }

        let node = &state.nodes[&node_id];
        let is_selected = Some(node_id) == state.selected;

        let mut label = if is_selected {
            format!("> {}", node.label)
        } else {
            format!("  {}", node.label)
        };

        if state.link_map.get(&node_id).map_or(false, |v| !v.is_empty()) {
            label.push_str(" 📎");
        }

        let width = label.len().min((area.width - x.saturating_sub(state.scroll_x as u16)) as usize);
        let scroll_x = state.scroll_x.max(0) as u16;

        let para = Paragraph::new(label).style(
            if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::White)
            }
        );

        let draw_x = x.saturating_sub(scroll_x);
        f.render_widget(para, Rect::new(draw_x, y, width as u16, 1));
    }
}
