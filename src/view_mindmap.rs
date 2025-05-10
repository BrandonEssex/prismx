use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::mindmap_state::{MindmapState, Node};

pub fn render_mindmap(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    let block = Block::default().title("Mindmap").borders(Borders::ALL);
    let mut lines = vec![];

    if let Some(root_id) = state.root_id {
        if let Some(root_node) = state.nodes.get(&root_id) {
            render_node(&mut lines, root_node, state, 0);
        }
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

fn render_node(
    lines: &mut Vec<Line<'_>>,
    node: &Node,
    state: &MindmapState,
    indent: usize,
) {
    let prefix = "  ".repeat(indent);
    let selected = state.selected == Some(node.id);
    let editing = state.editing == Some(node.id);

    let mut style = Style::default().fg(Color::White);
    if selected {
        style = style.fg(Color::LightGreen);
    }
    if editing {
        style = style.bg(Color::Blue);
    }

    let label = if editing {
        format!("✏ {}", state.edit_buffer)
    } else {
        node.label.clone()
    };

    lines.push(Line::from(Span::styled(
        format!("{}{}", prefix, label),
        style,
    )));

    for child_id in &node.children {
        if let Some(child) = state.nodes.get(child_id) {
            render_node(lines, child, state, indent + 1);
        }
    }
}