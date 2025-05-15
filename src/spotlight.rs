use ratatui::{
    layout::Rect,
    style::{Style, Color},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_spotlight(f: &mut Frame, area: Rect) {
    let box_area = Rect {
        x: area.width / 4,
        y: area.height / 3,
        width: area.width / 2,
        height: 3,
    };

    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled("🔍 Spotlight Search (TODO: bind input)", Style::default().fg(Color::White)),
    ]))
    .block(Block::default().title("Spotlight").borders(Borders::ALL))
    .style(Style::default().bg(Color::Black));

    f.render_widget(paragraph, box_area);
}
