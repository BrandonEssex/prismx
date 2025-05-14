// Author: Brandon Essex
// Renders mindmap view

use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, app: &AppState, area: Rect) {
    let content = format!("Mindmap Placeholder\nNode Count: {}", app.node_tree.len());

    let block = Block::default()
        .title("Mindmap")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(content).block(block);

    f.render_widget(paragraph, area);
}
