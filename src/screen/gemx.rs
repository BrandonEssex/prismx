use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::layout::{layout_nodes, Coords};
use crate::node::{NodeID, NodeMap};

pub fn render_gemx<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    nodes: &NodeMap,
    root_nodes: &[NodeID],
    selected: Option<NodeID>,
) {
    let block = Block::default()
        .title("Gemx")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let mut drawn_at = std::collections::HashMap::new();
    let mut y = 1;

    for &root_id in root_nodes {
        let layout = layout_nodes(nodes, root_id, 2, y);
        y = layout.values().map(|c| c.y).max().unwrap_or(y).saturating_add(2);
        drawn_at.extend(layout);
    }

    for (&node_id, &Coords { x, y }) in &drawn_at {
        if y >= area.height {
            continue;
        }

        let node = &nodes[&node_id];
        let is_selected = Some(node_id) == selected;

        let mut label = if is_selected {
            format!("> {}", node.label)
        } else {
            format!("  {}", node.label)
        };

        if let Some(linked) = node.link.as_ref() {
            label.push_str(" 📎");
        }

        let width = label.len().min((area.width - x) as usize);

        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::White)
        };

        let para = Paragraph::new(label).style(style);
        f.render_widget(para, Rect::new(x, y, width as u16, 1));
    }
}
