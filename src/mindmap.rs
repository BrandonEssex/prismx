use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_mindmap(f: &mut Frame, area: Rect) {
    let mindmap_text = Paragraph::new(Line::from(vec![
        Span::styled("Root → ", Style::default().fg(Color::Cyan)),
        Span::raw("Idea 1 → Subidea A"),
    ]))
    .block(Block::default().title("Mindmap").borders(Borders::ALL));

    f.render_widget(mindmap_text, area);
}
