use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Color},
};

pub fn render_status_bar<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    status: &str,
    plugin_health: &str,
) {
    let line = Line::from(vec![
        Span::styled(status, Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(plugin_health, Style::default().fg(Color::Yellow)),
    ]);

    let para = Paragraph::new(line)
        .block(Block::default().title("Status").borders(Borders::ALL));

    f.render_widget(para, area);
}