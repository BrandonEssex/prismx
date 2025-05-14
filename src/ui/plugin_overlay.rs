use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_plugin_overlay(f: &mut Frame, _app: &AppState, area: Rect) {
    let block = Block::default()
        .title("Plugins")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new("Plugin Data\nStatus / Logs").block(block);

    f.render_widget(paragraph, area);
}
