use crate::mindtrace::MindTrace;
use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct MindmapState {
    pub input: String,
    pub selected_id: String,
    pub cursor_position: usize,
}

impl MindmapState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_id: "root".to_string(),
            cursor_position: 0,
        }
    }
}

pub fn render_mindmap(f: &mut Frame, area: Rect, trace: &MindTrace, state: &MindmapState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let (left, right) = state.input.split_at(state.cursor_position);
    let input_line = Line::from(vec![
        Span::raw(left),
        Span::styled("▍", Style::default().fg(Color::Yellow)),
        Span::raw(right),
    ]);

    let input = Paragraph::new(input_line)
        .block(Block::default().borders(Borders::ALL).title("New Node"));
    f.render_widget(input, layout[0]);

    let mut lines = vec![Line::from("Mindmap:")];
    build_lines_recursive(&mut lines, trace, &trace.root_id, 0);

    let map = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("MindTrace"))
        .style(Style::default().fg(Color::White));
    f.render_widget(map, layout[1]);
}

fn build_lines_recursive(lines: &mut Vec<Line>, trace: &MindTrace, id: &str, depth: usize) {
    if let Some(node) = trace.nodes.get(id) {
        let indent = "  ".repeat(depth);
        lines.push(Line::from(format!("{}- {}", indent, node.content)));
        for child_id in &node.children {
            build_lines_recursive(lines, trace, child_id, depth + 1);
        }
    }
}
