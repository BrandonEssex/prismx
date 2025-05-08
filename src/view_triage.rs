use crate::inbox::{InboxTask, TaskStatus};
use crate::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_triage_view(frame: &mut Frame, state: &AppState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(size);

    let header = Paragraph::new("📥 Triage Inbox — Use arrows to navigate, Ctrl+D to archive")
        .block(Block::default().borders(Borders::ALL).title("Inbox Triage"))
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(header, chunks[0]);

    let inbox_tasks: Vec<&InboxTask> = state.inbox.list_by_status(TaskStatus::Inbox);

    let items: Vec<ListItem> = inbox_tasks
        .iter()
        .map(|task| {
            let status_label = format!("{:?}", task.status);
            let line = format!("• [{}] {}", status_label, task.title);
            ListItem::new(Line::from(vec![Span::raw(line)]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox Tasks"))
        .highlight_symbol(">> ");

    frame.render_widget(list, chunks[1]);
}