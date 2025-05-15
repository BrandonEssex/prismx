use ratatui::{
    layout::{Rect, Layout, Constraint, Direction},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct MindmapState {
    pub current_input: String,
    pub cursor_visible: bool,
}

impl MindmapState {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            cursor_visible: true,
        }
    }
}

pub fn render_mindmap(f: &mut Frame, area: Rect, state: &MindmapState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3)])
        .split(area);

    let content = format!("> {}", state.current_input);
    let paragraph = Paragraph::new(Line::from(content))
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Mindmap").borders(Borders::ALL));

    f.render_widget(paragraph, layout[0]);
}
