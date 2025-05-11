use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans, Text},
    Frame,
};

pub fn render_help_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let lines = vec![
        Spans::from(vec![
            Span::styled("q", ratatui::style::Style::default().bold()),
            Span::raw("   Quit application"),
        ]),
        Spans::from(vec![
            Span::styled("h", ratatui::style::Style::default().bold()),
            Span::raw("   Toggle help panel"),
        ]),
        Spans::from(vec![
            Span::styled("i", ratatui::style::Style::default().bold()),
            Span::raw("   Open inbox"),
        ]),
        Spans::from(vec![
            Span::styled("m", ratatui::style::Style::default().bold()),
            Span::raw("   Open mindmap"),
        ]),
        Spans::from(vec![
            Span::styled("Tab", ratatui::style::Style::default().bold()),
            Span::raw(" Cycle sidebar views"),
        ]),
        Spans::from(vec![
            Span::styled("Esc", ratatui::style::Style::default().bold()),
            Span::raw(" Hide sidebar"),
        ]),
    ];

    let block = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Keyboard Shortcuts").borders(Borders::ALL));

    f.render_widget(block, area);
}