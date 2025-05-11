use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    text::Span,
    Frame,
};

pub fn render_status_icon(f: &mut Frame<'_>, area: Rect) {
    let content = Span::styled("X", Style::default().fg(Color::Green));
    let block = Paragraph::new(content)
        .block(Block::default().title("PrismX Icon").borders(Borders::ALL));

    f.render_widget(block, area);
}