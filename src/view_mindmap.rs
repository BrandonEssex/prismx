use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Span;
use ratatui::layout::Rect;
use ratatui::style::{Style, Modifier};
use ratatui::Frame;

use crate::mindmap_state::{Node, NodeType};
use crate::state::AppState;

pub fn render_mindmap(f: &mut Frame, area: Rect, state: &AppState) {
    let root = &state.mindmap.root;
    let lines = render_node_recursive(root, 0);
    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Mindmap").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn render_node_recursive<'a>(node: &'a Node, indent: usize) -> Vec<Span<'a>> {
    let mut lines = vec![];
    let indent_str = "  ".repeat(indent);
    let icon = match node.node_type {
        NodeType::Note => "📄",
        NodeType::Task => "☑",
        NodeType::Idea => "💡",
        NodeType::Custom(_) => "🔸",
    };
    let label = format!("{}{} {}", indent_str, icon, node.label);
    lines.push(Span::styled(label, Style::default().add_modifier(Modifier::BOLD)));
    for child in &node.children {
        lines.extend(render_node_recursive(child, indent + 1));
    }
    lines
}