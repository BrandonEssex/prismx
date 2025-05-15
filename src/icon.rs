use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Paragraph, Block, Borders},
    Frame,
};

pub fn render_prismx_icon(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("P", Style::default().fg(Color::Red)),
        Span::styled("R", Style::default().fg(Color::Yellow)),
        Span::styled("I", Style::default().fg(Color::Green)),
        Span::styled("S", Style::default().fg(Color::Cyan)),
        Span::styled("M", Style::default().fg(Color::Blue)),
        Span::styled("X", Style::default().fg(Color::Magenta)),
    ]);
    let block = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL).title("PrismX"));
    f.render_widget(block, area);
}
