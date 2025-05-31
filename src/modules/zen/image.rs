use ratatui::prelude::*;
use crate::state::AppState;
use crate::theme::zen::zen_theme;

/// Draw placeholder drop zone for images.
pub fn render_drop_zone<B: Backend>(f: &mut Frame<B>, area: Rect) {
    use ratatui::{widgets::{Block, Borders, Paragraph}, text::Line, style::Style};

    let palette = zen_theme();
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(palette.background).fg(palette.text));
    let widget = Paragraph::new(Line::from("Drop image here"))
        .block(block)
        .style(Style::default().fg(palette.text).bg(palette.background));

    f.render_widget(widget, area);
}

/// Stub handler for future paste/image buffer support.
pub fn handle_drop(_state: &mut AppState, _path: &str) {
    // TODO: accept pasted images or dropped file paths
}
