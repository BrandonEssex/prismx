use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};

pub fn render_sidebar(f: &mut Frame<'_>, area: Rect, title: &str, content: &[String]) {
    let block = Block::default().title(title).borders(Borders::ALL);
    let lines: Vec<Line> = content
        .iter()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::LightCyan))))
        .collect();
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}