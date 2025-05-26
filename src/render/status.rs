use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};

pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect, status: &str) {
    let block = Block::default().borders(Borders::ALL).title("Status");
    let content = Paragraph::new(status).style(Style::default());
    f.render_widget(block, area);
    let inner_width = area.width.saturating_sub(2);
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, inner_width, 1));
}
