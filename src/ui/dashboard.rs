// Author: Brandon Essex
// Renders dashboard view

use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn render_dashboard<B: Backend>(f: &mut Frame<B>, _app: &AppState, area: Rect) {
    let block = Block::default()
        .title("Dashboard")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new("System Status\nTasks\nMetrics")
        .block(block);

    f.render_widget(paragraph, area);
}
