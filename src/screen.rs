use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::state::{AppState, visible_nodes};

pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let block = Block::default()
        .title(if state.edit_mode { "Mindmap (Edit)" } else { "Mindmap" })
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let offset_y = area.y + 1;
    let mut visible: Vec<(usize, Rc<RefCell<crate::state::Node>>)> = Vec::new();
    visible_nodes(&state.root, 0, &mut visible);

    for (i, (depth, node)) in visible.iter().enumerate() {
        let y = offset_y + i as u16;
        if y >= area.bottom() {
            break;
        }

        let n = node.borrow();
        let prefix = if !n.children.is_empty() {
            if n.collapsed { "[+]" } else { "[-]" }
        } else {
            "   "
        };

        let indent = "  ".repeat(*depth);
        let content = if i == state.active_node {
            format!("> {}{} {}", indent, prefix, n.label)
        } else {
            format!("  {}{} {}", indent, prefix, n.label)
        };

        let style = if i == state.active_node {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(content).style(style);
        f.render_widget(para, Rect::new(area.x + 2, y, area.width - 4, 1));
    }
}
