use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
};

pub fn render_sidebar<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    title: &str,
    content: &[String],
) {
    let block = Block::default().title(title).borders(Borders::ALL);
    let lines: Vec<Line> = content
        .iter()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::LightCyan))))
        .collect();
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}