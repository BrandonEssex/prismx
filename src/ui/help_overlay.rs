use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span, Text},
    style::{Color, Stylize},
    Frame,
};

pub fn render_help_overlay(f: &mut Frame<'_>, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("q", Style::default().bold()),
            Span::raw("   Quit application"),
        ]),
        Line::from(vec![
            Span::styled("h", Style::default().bold()),
            Span::raw("   Toggle help panel"),
        ]),
        Line::from(vec![
            Span::styled("i", Style::default().bold()),
            Span::raw("   Open inbox"),
        ]),
        Line::from(vec![
            Span::styled("m", Style::default().bold()),
            Span::raw("   Open mindmap"),
        ]),
        Line::from(vec![
            Span::styled("Tab", Style::default().bold()),
            Span::raw(" Cycle sidebar views"),
        ]),
        Line::from(vec![
            Span::styled("Esc", Style::default().bold()),
            Span::raw(" Hide sidebar"),
        ]),
    ];

    let block = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Keyboard Shortcuts").borders(Borders::ALL));

    f.render_widget(block, area);
}