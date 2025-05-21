use std::rc::Rc;
use std::cell::RefCell;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::state::{AppState, visible_nodes, Node};

pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let block = Block::default()
        .title(if state.edit_mode { "Mindmap (Edit)" } else { "Mindmap" })
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let offset_y = area.y + 1;
    let height = area.height.saturating_sub(2) as usize; // usable lines
    state.max_visible_lines = height;

    let mut all_nodes: Vec<(usize, Rc<RefCell<Node>>)> = Vec::new();
    visible_nodes(&state.root, 0, &mut all_nodes);

    if state.active_node < state.scroll_offset {
        state.scroll_offset = state.active_node;
    } else if state.active_node >= state.scroll_offset + height {
        state.scroll_offset = state.active_node.saturating_sub(height - 1);
    }

    let visible = all_nodes
        .iter()
        .skip(state.scroll_offset)
        .take(height)
        .enumerate();

    for (i, (depth, node)) in visible {
        let y = offset_y + i as u16;
        if y >= area.bottom() {
            break;
        }

        let n = node.borrow();
        let is_active = (state.scroll_offset + i) == state.active_node;

        let mut prefix = String::new();
        if !n.children.is_empty() {
            prefix.push_str(if n.collapsed { "┠─" } else { "├─" });
        } else {
            prefix.push_str("└─");
        }

        let indent = "│  ".repeat(*depth);
        let label = if is_active {
            format!("> {}{} {}", indent, prefix, n.label)
        } else {
            format!("  {}{} {}", indent, prefix, n.label)
        };

        let style = if is_active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(area.x + 2, y, area.width - 4, 1));
    }
}
