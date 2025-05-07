use crate::inbox::{InboxTask, TaskStatus};
use crate::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_triage_view(frame: &mut Frame, state: &AppState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(size);

    let header = Paragraph::new("📥 Triage Inbox — Use arrows to navigate, Ctrl+D to archive")
        .block(Block::default().borders(Borders::ALL).title("Inbox Triage"))
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(header, chunks[0]);

    let inbox_tasks: Vec<&InboxTask> = state.inbox.list_by_status(TaskStatus::Inbox);

    let items: Vec<ListItem> = inbox_tasks
        .iter()
        .map(|task| {
            let mut line = format!("• [{}] {}", task.priority, task.title);

            if let Some(ref shard) = task.shard {
                line.push_str(&format!(" | Shard: {}", shard));
            }

            if !task.tags.is_empty() {
                line.push_str(&format!(" | Tags: {}", task.tags.join(", ")));
            }

            if let Some(ref owner) = task.assigned_to {
                line.push_str(&format!(" | Assigned: {}", owner));
            }

            ListItem::new(Line::from(Span::raw(line)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox Tasks"))
        .highlight_symbol(">> ");

    frame.render_widget(list, chunks[1]);
}