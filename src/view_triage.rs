use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Modifier},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    text::{Span},
    Frame,
};
use crate::inbox::{InboxTask, TaskStatus};
use crate::state::AppState;

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

    let inbox_tasks: Vec<&InboxTask> = state.inbox.all_tasks()
        .iter()
        .filter(|t| t.status == TaskStatus::Inbox)
        .collect();

    let items: Vec<ListItem> = inbox_tasks
        .iter()
        .map(|task| {
            let line = format!("• [{}] {}", task.status_string(), task.title);
            ListItem::new(Span::raw(line))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox Tasks"))
        .highlight_symbol(">> ");

    frame.render_widget(list, chunks[1]);
}