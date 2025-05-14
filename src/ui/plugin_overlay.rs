// Author: Brandon Essex
// Renders plugin overlay window

use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_plugin_overlay<B: Backend>(f: &mut Frame<B>, _app: &AppState, area: Rect) {
    let block = Block::default()
        .title("Plugins")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new("Plugin Data\nStatus / Logs").block(block);

    f.render_widget(paragraph, area);
}
