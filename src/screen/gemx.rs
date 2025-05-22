use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{layout_nodes, Coords};
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

    let roots = if let Some(drill_root) = state.drawing_root {
        vec![drill_root]
    } else {
        state.root_nodes.clone()
    };

    let mut drawn_at = std::collections::HashMap::new();
    let mut y_cursor = 1;

    for &root_id in &roots {
        let layout = layout_nodes(&state.nodes, root_id, 2, y_cursor);
        let max_y = layout.values().map(|c| c.y).max().unwrap_or(y_cursor);
        drawn_at.extend(layout);
        y_cursor = max_y.saturating_add(3);
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

        if let Some(target_id) = state.link_map.get(&node_id).and_then(|v| v.first()) {
            label.push_str(" 📎");
        }

        let scroll_x = state.scroll_x.max(0) as u16;
        let width = label.len().min((area.width - x.saturating_sub(scroll_x)) as usize);

        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(x.saturating_sub(scroll_x), y, width as u16, 1));
    }
}
