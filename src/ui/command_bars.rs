use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Modifier, Color},
    Frame,
};

pub fn render_command_bar<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    prompt: &str,
    query: &str,
) {
    let title = format!("Command: {prompt}");
    let para = Paragraph::new(Line::from(Span::styled(
        query,
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    )))
    .block(Block::default().title(title).borders(Borders::ALL));

    f.render_widget(para, area);
}