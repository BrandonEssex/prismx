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
    let height = area.height.saturating_sub(2) as usize;
    state.max_visible_lines = height;

    // Collect visible nodes
    let mut flat: Vec<(usize, Rc<RefCell<Node>>)> = Vec::new();
    visible_nodes(&state.root, 0, &mut flat);

    // Auto-scroll to keep active in view
    if state.active_node < state.scroll_offset {
        state.scroll_offset = state.active_node;
    } else if state.active_node >= state.scroll_offset + height {
        state.scroll_offset = state.active_node.saturating_sub(height - 1);
    }

    let view = flat
        .iter()
        .skip(state.scroll_offset)
        .take(height)
        .enumerate()
        .collect::<Vec<_>>();

    // For each visible node, build context of "is_last" flags at each depth
    let mut depth_tracker = vec![false; 64]; // up to 64 nesting levels

    for (i, (depth, node)) in view {
        let y = offset_y + i as u16;
        if y >= area.bottom() {
            break;
        }

        let is_last = {
            if i + 1 >= flat.len() {
                true
            } else {
                let (next_depth, _) = flat[state.scroll_offset + i + 1];
                next_depth < *depth
            }
        };

        depth_tracker[*depth] = is_last;

        // Build prefix lines like void-rs
        let mut prefix = String::new();
        for d in 0..*depth {
            if depth_tracker[d] {
                prefix.push_str("   ");
            } else {
                prefix.push_str("│  ");
            }
        }

        prefix.push_str(if is_last { "└─" } else { "├─" });

        let n = node.borrow();
        let is_active = (state.scroll_offset + i) == state.active_node;

        let label = if is_active {
            format!("> {} {}", prefix, n.label)
        } else {
            format!("  {} {}", prefix, n.label)
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
