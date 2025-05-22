use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, input: &str) {
    let width = area.width.min(60);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let spotlight_area = Rect::new(x_offset, y_offset, width, 3);
    let block = Block::default()
        .title("Spotlight")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::Cyan));

    let paragraph = Paragraph::new(format!("> {}", input))
        .block(block)
        .style(Style::default().fg(ratatui::style::Color::White));

    f.render_widget(paragraph, spotlight_area);
}
