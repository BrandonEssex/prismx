use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_mindmap(f: &mut Frame, app: &AppState, area: Rect) {
    let content = format!("Mindmap Placeholder\nNode Count: {}", app.node_tree.len());

    let block = Block::default()
        .title("Mindmap")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(content).block(block);

    f.render_widget(paragraph, area);
}
