use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::{Span, Text, Line};
use ratatui::style::{Style, Color};
use ratatui::Frame;

pub fn render_shortcuts<B>(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![Span::styled("Ctrl+Q", Style::default().fg(Color::Yellow)), Span::raw("  - Quit")]),
        Line::from(vec![Span::styled("Ctrl+I", Style::default().fg(Color::Cyan)), Span::raw("  - PrismX Panel")]),
        Line::from(vec![Span::styled("Ctrl+N", Style::default().fg(Color::Green)), Span::raw("  - New Node")]),
    ];

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Shortcuts").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}